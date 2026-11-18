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


thread_local! { static G_ABOUT_PERSON_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACTION_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACTION_DURATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACTIVITY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGES_IN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_ADVERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_AS_A_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_FIX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_IN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_PERSON_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_RULE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_YOU_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWER_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANY_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANY_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_ABOUT_A_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_FOR_TASTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_ABOUT_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_TIME_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_BEFORE_A_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_SAID_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAVE_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_AFTER_A_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_WITH_ITS_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERCENT_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_STATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECALL_IN_ANOTHER_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHORT_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_UNIT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ECHO_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_NEW_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_SLOTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_WHAT_I_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREED_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_FITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_TIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALL_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANT_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAN_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAN_THEY_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAN_THEY_HAVE_A_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAR_CAPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_A_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_FOLLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_NOTHING_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSE_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSE_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONCEPTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_KEEPS_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_RULES_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_AGAIN_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_WHICH_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY_DROP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_SAY_PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTS_A_MAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTS_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTS_A_NAME_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_OF_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_STEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_RULES_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOG_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DROP_LISTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_NOT_SURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELIDED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEEDING_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_ANSWER_LEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEF_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_NAMES_TYPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_NAME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_ANY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELD_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MANY_OF_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_LAW_AFTER_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEPT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEY_SLOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LISTED_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MADE_OF_A_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEMORY_POINTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_CATCH_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_ALT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_ORDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_BY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_FACT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_FIX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_GIVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_GIVEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_NOT_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_STARTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FACT_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NAME_AFTER_CALLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NAME_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NEW_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_FROM_THE_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ON_A_MEDIUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_RETRACTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_PLANS_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_RECORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_SUBJECT_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_DIGITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_BARE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_FACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_KIND_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PLACE_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNED_BY_THE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIR_WITHOUT_A_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_ADVICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_AGE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_AGE_FIX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_NAME_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_RENAMED_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_WORD_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK_AT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANCY_SAID_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_OWN_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_WEEKS_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_FITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_HINTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_NAMES_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_SLOTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_SLOTS_BY_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_WRAPPER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION_ALIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATIVE_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMIND_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_ADVERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESTATED_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESTRICT_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESUME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETRACT_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_SAY_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AS_A_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_FACT_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHE_OR_HE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLOT_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLOT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORRY_OPENS_A_CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLING_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATES_A_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_WAS_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_THE_SAME_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOPWORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWAPPABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_WINDOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TASTE_ASKED_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THAT_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_A_YES_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_WHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_ARE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_KIDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PLACE_IS_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_AS_WELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TURNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_SECONDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKS_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_I_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_I_SAID_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_END_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_END_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_INVERTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_INVERTED_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_QUALITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_START_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_THING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_VAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_ON_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_NO_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONVERT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FACT_SLOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TENS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__concept: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__country: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__enough_reply: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__word: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__words: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_PERSON_ASK() -> Py { ensure_init(); G_ABOUT_PERSON_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_PERSON_ASK(v: Py) { ensure_init(); G_ABOUT_PERSON_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ACKS() -> Py { ensure_init(); G_ACKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACKS(v: Py) { ensure_init(); G_ACKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ACTION_AMOUNT() -> Py { ensure_init(); G_ACTION_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACTION_AMOUNT(v: Py) { ensure_init(); G_ACTION_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_ACTION_DURATION() -> Py { ensure_init(); G_ACTION_DURATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACTION_DURATION(v: Py) { ensure_init(); G_ACTION_DURATION.with(|c| *c.borrow_mut() = v); }
pub fn g_ACTIVITY_ASK() -> Py { ensure_init(); G_ACTIVITY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACTIVITY_ASK(v: Py) { ensure_init(); G_ACTIVITY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_MODULES() -> Py { ensure_init(); G_ADVICE_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_MODULES(v: Py) { ensure_init(); G_ADVICE_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_AGES_IN_WORDS() -> Py { ensure_init(); G_AGES_IN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGES_IN_WORDS(v: Py) { ensure_init(); G_AGES_IN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_ADVERB() -> Py { ensure_init(); G_AGE_ADVERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_ADVERB(v: Py) { ensure_init(); G_AGE_ADVERB.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_AGAIN() -> Py { ensure_init(); G_AGE_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_AGAIN(v: Py) { ensure_init(); G_AGE_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_ASK() -> Py { ensure_init(); G_AGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_ASK(v: Py) { ensure_init(); G_AGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_AS_A_CAUSE() -> Py { ensure_init(); G_AGE_AS_A_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_AS_A_CAUSE(v: Py) { ensure_init(); G_AGE_AS_A_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_COPULA() -> Py { ensure_init(); G_AGE_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_COPULA(v: Py) { ensure_init(); G_AGE_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_DETERMINER() -> Py { ensure_init(); G_AGE_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_DETERMINER(v: Py) { ensure_init(); G_AGE_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_FIX() -> Py { ensure_init(); G_AGE_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_FIX(v: Py) { ensure_init(); G_AGE_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_FIX_WORDS() -> Py { ensure_init(); G_AGE_FIX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_FIX_WORDS(v: Py) { ensure_init(); G_AGE_FIX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_IN_WORDS() -> Py { ensure_init(); G_AGE_IN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_IN_WORDS(v: Py) { ensure_init(); G_AGE_IN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_PERSON_WORD() -> Py { ensure_init(); G_AGE_PERSON_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_PERSON_WORD(v: Py) { ensure_init(); G_AGE_PERSON_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_RULE_WORD() -> Py { ensure_init(); G_AGE_RULE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_RULE_WORD(v: Py) { ensure_init(); G_AGE_RULE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_SAID() -> Py { ensure_init(); G_AGE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_SAID(v: Py) { ensure_init(); G_AGE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_WORD() -> Py { ensure_init(); G_AGE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_WORD(v: Py) { ensure_init(); G_AGE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_YOU_SAY() -> Py { ensure_init(); G_AGE_YOU_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_YOU_SAY(v: Py) { ensure_init(); G_AGE_YOU_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT() -> Py { ensure_init(); G_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT(v: Py) { ensure_init(); G_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_WORD() -> Py { ensure_init(); G_AMOUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_WORD(v: Py) { ensure_init(); G_AMOUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_ANIMAL() -> Py { ensure_init(); G_ANOTHER_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_ANIMAL(v: Py) { ensure_init(); G_ANOTHER_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_PERSON() -> Py { ensure_init(); G_ANOTHER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_PERSON(v: Py) { ensure_init(); G_ANOTHER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWER_MODULES() -> Py { ensure_init(); G_ANSWER_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWER_MODULES(v: Py) { ensure_init(); G_ANSWER_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_ANY_FIGURE() -> Py { ensure_init(); G_ANY_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANY_FIGURE(v: Py) { ensure_init(); G_ANY_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANY_NUMBER() -> Py { ensure_init(); G_ANY_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANY_NUMBER(v: Py) { ensure_init(); G_ANY_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_LEADS() -> Py { ensure_init(); G_ASKED_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_LEADS(v: Py) { ensure_init(); G_ASKED_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_ABOUT_A_TASTE() -> Py { ensure_init(); G_ASKS_ABOUT_A_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_A_TASTE(v: Py) { ensure_init(); G_ASKS_ABOUT_A_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_FOR_TASTES() -> Py { ensure_init(); G_ASKS_FOR_TASTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_TASTES(v: Py) { ensure_init(); G_ASKS_FOR_TASTES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_ABOUT_THEM() -> Py { ensure_init(); G_ASK_ABOUT_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_ABOUT_THEM(v: Py) { ensure_init(); G_ASK_ABOUT_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OPENERS() -> Py { ensure_init(); G_ASK_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENERS(v: Py) { ensure_init(); G_ASK_OPENERS.with(|c| *c.borrow_mut() = v); }
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
pub fn g_A_HAVE_QUESTION() -> Py { ensure_init(); G_A_HAVE_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAVE_QUESTION(v: Py) { ensure_init(); G_A_HAVE_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_FIGURE() -> Py { ensure_init(); G_A_MONEY_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_FIGURE(v: Py) { ensure_init(); G_A_MONEY_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_AFTER_A_PRONOUN() -> Py { ensure_init(); G_A_NAME_AFTER_A_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_AFTER_A_PRONOUN(v: Py) { ensure_init(); G_A_NAME_AFTER_A_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_WITH_ITS_VERB() -> Py { ensure_init(); G_A_NAME_WITH_ITS_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_WITH_ITS_VERB(v: Py) { ensure_init(); G_A_NAME_WITH_ITS_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERCENT_FIGURE() -> Py { ensure_init(); G_A_PERCENT_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERCENT_FIGURE(v: Py) { ensure_init(); G_A_PERCENT_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_ASKED() -> Py { ensure_init(); G_A_QUESTION_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_ASKED(v: Py) { ensure_init(); G_A_QUESTION_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_ASK() -> Py { ensure_init(); G_A_RATE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_ASK(v: Py) { ensure_init(); G_A_RATE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_STATED() -> Py { ensure_init(); G_A_RATE_STATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_STATED(v: Py) { ensure_init(); G_A_RATE_STATED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECALL_IN_ANOTHER_SHAPE() -> Py { ensure_init(); G_A_RECALL_IN_ANOTHER_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECALL_IN_ANOTHER_SHAPE(v: Py) { ensure_init(); G_A_RECALL_IN_ANOTHER_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHORT_ANSWER() -> Py { ensure_init(); G_A_SHORT_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHORT_ANSWER(v: Py) { ensure_init(); G_A_SHORT_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_UNIT_SAID() -> Py { ensure_init(); G_A_UNIT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_UNIT_SAID(v: Py) { ensure_init(); G_A_UNIT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_NEWS() -> Py { ensure_init(); G_BAD_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_NEWS(v: Py) { ensure_init(); G_BAD_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ECHO_ONLY() -> Py { ensure_init(); G_BARE_ECHO_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ECHO_ONLY(v: Py) { ensure_init(); G_BARE_ECHO_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_HOUR() -> Py { ensure_init(); G_BARE_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_HOUR(v: Py) { ensure_init(); G_BARE_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_NEW_COUNT() -> Py { ensure_init(); G_BARE_NEW_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_NEW_COUNT(v: Py) { ensure_init(); G_BARE_NEW_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_SLOTS() -> Py { ensure_init(); G_BARE_SLOTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_SLOTS(v: Py) { ensure_init(); G_BARE_SLOTS.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_WHAT_I_DO() -> Py { ensure_init(); G_BARE_WHAT_I_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_WHAT_I_DO(v: Py) { ensure_init(); G_BARE_WHAT_I_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_BREED_ASK() -> Py { ensure_init(); G_BREED_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREED_ASK(v: Py) { ensure_init(); G_BREED_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_FITS() -> Py { ensure_init(); G_BUDGET_FITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_FITS(v: Py) { ensure_init(); G_BUDGET_FITS.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_MORE() -> Py { ensure_init(); G_BUDGET_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_MORE(v: Py) { ensure_init(); G_BUDGET_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_TIGHT() -> Py { ensure_init(); G_BUDGET_TIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_TIGHT(v: Py) { ensure_init(); G_BUDGET_TIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_SAID() -> Py { ensure_init(); G_CALENDAR_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_SAID(v: Py) { ensure_init(); G_CALENDAR_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_CALL_LEADS() -> Py { ensure_init(); G_CALL_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALL_LEADS(v: Py) { ensure_init(); G_CALL_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_CANT_HAVE() -> Py { ensure_init(); G_CANT_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANT_HAVE(v: Py) { ensure_init(); G_CANT_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAN_HAVE() -> Py { ensure_init(); G_CAN_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAN_HAVE(v: Py) { ensure_init(); G_CAN_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAN_THEY_HAVE() -> Py { ensure_init(); G_CAN_THEY_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAN_THEY_HAVE(v: Py) { ensure_init(); G_CAN_THEY_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAN_THEY_HAVE_A_FOOD() -> Py { ensure_init(); G_CAN_THEY_HAVE_A_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAN_THEY_HAVE_A_FOOD(v: Py) { ensure_init(); G_CAN_THEY_HAVE_A_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_CAR_CAPS() -> Py { ensure_init(); G_CAR_CAPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAR_CAPS(v: Py) { ensure_init(); G_CAR_CAPS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_NAMES() -> Py { ensure_init(); G_CAT_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_NAMES(v: Py) { ensure_init(); G_CAT_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_HEAVY() -> Py { ensure_init(); G_CHANGE_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_HEAVY(v: Py) { ensure_init(); G_CHANGE_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_PLAIN() -> Py { ensure_init(); G_CHANGE_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_PLAIN(v: Py) { ensure_init(); G_CHANGE_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_TAIL() -> Py { ensure_init(); G_CHAT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_TAIL(v: Py) { ensure_init(); G_CHAT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_A_FEELING() -> Py { ensure_init(); G_CHECK_A_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_A_FEELING(v: Py) { ensure_init(); G_CHECK_A_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_A_QUESTION() -> Py { ensure_init(); G_CHECK_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_A_QUESTION(v: Py) { ensure_init(); G_CHECK_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_FOLLOWED() -> Py { ensure_init(); G_CHECK_FOLLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_FOLLOWED(v: Py) { ensure_init(); G_CHECK_FOLLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_NOTHING_SAID() -> Py { ensure_init(); G_CHECK_NOTHING_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_NOTHING_SAID(v: Py) { ensure_init(); G_CHECK_NOTHING_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_PLAIN() -> Py { ensure_init(); G_CHECK_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_PLAIN(v: Py) { ensure_init(); G_CHECK_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD() -> Py { ensure_init(); G_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD(v: Py) { ensure_init(); G_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_OF() -> Py { ensure_init(); G_CHILD_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_OF(v: Py) { ensure_init(); G_CHILD_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_RELATIONS() -> Py { ensure_init(); G_CHILD_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_RELATIONS(v: Py) { ensure_init(); G_CHILD_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORD() -> Py { ensure_init(); G_CHILD_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORD(v: Py) { ensure_init(); G_CHILD_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_SAID() -> Py { ensure_init(); G_CLOCK_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_SAID(v: Py) { ensure_init(); G_CLOCK_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSE_HEAVY() -> Py { ensure_init(); G_CLOSE_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSE_HEAVY(v: Py) { ensure_init(); G_CLOSE_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSE_PLAIN() -> Py { ensure_init(); G_CLOSE_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSE_PLAIN(v: Py) { ensure_init(); G_CLOSE_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CONCEPTS() -> Py { ensure_init(); G_CONCEPTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONCEPTS(v: Py) { ensure_init(); G_CONCEPTS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_KEEPS_DOWN() -> Py { ensure_init(); G_CONDITION_KEEPS_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_KEEPS_DOWN(v: Py) { ensure_init(); G_CONDITION_KEEPS_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_RULES_OUT() -> Py { ensure_init(); G_CONDITION_RULES_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_RULES_OUT(v: Py) { ensure_init(); G_CONDITION_RULES_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTABLE() -> Py { ensure_init(); G_CORRECTABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTABLE(v: Py) { ensure_init(); G_CORRECTABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_AGAIN() -> Py { ensure_init(); G_CORRECTED_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_AGAIN(v: Py) { ensure_init(); G_CORRECTED_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_AGAIN_WHO() -> Py { ensure_init(); G_CORRECTED_AGAIN_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_AGAIN_WHO(v: Py) { ensure_init(); G_CORRECTED_AGAIN_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_SAY() -> Py { ensure_init(); G_CORRECTED_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_SAY(v: Py) { ensure_init(); G_CORRECTED_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_WHICH_PERSON() -> Py { ensure_init(); G_CORRECTED_WHICH_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_WHICH_PERSON(v: Py) { ensure_init(); G_CORRECTED_WHICH_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTING() -> Py { ensure_init(); G_CORRECTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTING(v: Py) { ensure_init(); G_CORRECTING.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_BACK() -> Py { ensure_init(); G_CORRECTION_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_BACK(v: Py) { ensure_init(); G_CORRECTION_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_SAY() -> Py { ensure_init(); G_CORRECTION_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_SAY(v: Py) { ensure_init(); G_CORRECTION_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_SAY_DROP() -> Py { ensure_init(); G_CORRECTION_SAY_DROP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_SAY_DROP(v: Py) { ensure_init(); G_CORRECTION_SAY_DROP.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_SAY_PAIR() -> Py { ensure_init(); G_CORRECTION_SAY_PAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_SAY_PAIR(v: Py) { ensure_init(); G_CORRECTION_SAY_PAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTS_A_MAKE() -> Py { ensure_init(); G_CORRECTS_A_MAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTS_A_MAKE(v: Py) { ensure_init(); G_CORRECTS_A_MAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTS_A_NAME() -> Py { ensure_init(); G_CORRECTS_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTS_A_NAME(v: Py) { ensure_init(); G_CORRECTS_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTS_A_NAME_POSSESSIVE() -> Py { ensure_init(); G_CORRECTS_A_NAME_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTS_A_NAME_POSSESSIVE(v: Py) { ensure_init(); G_CORRECTS_A_NAME_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_ANIMAL() -> Py { ensure_init(); G_COUNT_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_ANIMAL(v: Py) { ensure_init(); G_COUNT_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_BARE() -> Py { ensure_init(); G_COUNT_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_BARE(v: Py) { ensure_init(); G_COUNT_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_SAID() -> Py { ensure_init(); G_COUNT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_SAID(v: Py) { ensure_init(); G_COUNT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORD() -> Py { ensure_init(); G_COUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORD(v: Py) { ensure_init(); G_COUNT_WORD.with(|c| *c.borrow_mut() = v); }
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
pub fn g_DAY_ASKS() -> Py { ensure_init(); G_DAY_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_ASKS(v: Py) { ensure_init(); G_DAY_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_OF_MONTH() -> Py { ensure_init(); G_DAY_OF_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_OF_MONTH(v: Py) { ensure_init(); G_DAY_OF_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_WORD() -> Py { ensure_init(); G_DAY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_WORD(v: Py) { ensure_init(); G_DAY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIAL_SAY() -> Py { ensure_init(); G_DENIAL_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIAL_SAY(v: Py) { ensure_init(); G_DENIAL_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIAL_STEMS() -> Py { ensure_init(); G_DENIAL_STEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIAL_STEMS(v: Py) { ensure_init(); G_DENIAL_STEMS.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIAL_WORDS() -> Py { ensure_init(); G_DENIAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIAL_WORDS(v: Py) { ensure_init(); G_DENIAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_FOOD() -> Py { ensure_init(); G_DIET_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_FOOD(v: Py) { ensure_init(); G_DIET_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_RULES_OUT() -> Py { ensure_init(); G_DIET_RULES_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_RULES_OUT(v: Py) { ensure_init(); G_DIET_RULES_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_DOG_NAMES() -> Py { ensure_init(); G_DOG_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOG_NAMES(v: Py) { ensure_init(); G_DOG_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_DROP_LISTS() -> Py { ensure_init(); G_DROP_LISTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DROP_LISTS(v: Py) { ensure_init(); G_DROP_LISTS.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_ASK() -> Py { ensure_init(); G_EAT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_ASK(v: Py) { ensure_init(); G_EAT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_AGAIN() -> Py { ensure_init(); G_ELIDED_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_AGAIN(v: Py) { ensure_init(); G_ELIDED_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_ASK() -> Py { ensure_init(); G_ELIDED_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_ASK(v: Py) { ensure_init(); G_ELIDED_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_NAMES() -> Py { ensure_init(); G_ELIDED_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_NAMES(v: Py) { ensure_init(); G_ELIDED_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_NOT_SURE() -> Py { ensure_init(); G_ELIDED_NOT_SURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_NOT_SURE(v: Py) { ensure_init(); G_ELIDED_NOT_SURE.with(|c| *c.borrow_mut() = v); }
pub fn g_ELIDED_WORDS() -> Py { ensure_init(); G_ELIDED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELIDED_WORDS(v: Py) { ensure_init(); G_ELIDED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_FAMILY() -> Py { ensure_init(); G_FACT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_FAMILY(v: Py) { ensure_init(); G_FACT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_WORDS() -> Py { ensure_init(); G_FACT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_WORDS(v: Py) { ensure_init(); G_FACT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_GROUP() -> Py { ensure_init(); G_FAMILY_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_GROUP(v: Py) { ensure_init(); G_FAMILY_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_FEEDING_SAY() -> Py { ensure_init(); G_FEEDING_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEEDING_SAY(v: Py) { ensure_init(); G_FEEDING_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_AFTER() -> Py { ensure_init(); G_FIGURE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_AFTER(v: Py) { ensure_init(); G_FIGURE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_ANSWER_LEN() -> Py { ensure_init(); G_FIGURE_ANSWER_LEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_ANSWER_LEN(v: Py) { ensure_init(); G_FIGURE_ANSWER_LEN.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_ASK() -> Py { ensure_init(); G_FIGURE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_ASK(v: Py) { ensure_init(); G_FIGURE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_FILLER() -> Py { ensure_init(); G_FIGURE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_FILLER(v: Py) { ensure_init(); G_FIGURE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_MONEY() -> Py { ensure_init(); G_FIGURE_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_MONEY(v: Py) { ensure_init(); G_FIGURE_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_NUMBER() -> Py { ensure_init(); G_FIGURE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_NUMBER(v: Py) { ensure_init(); G_FIGURE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_QUALIFIER() -> Py { ensure_init(); G_FIGURE_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_QUALIFIER(v: Py) { ensure_init(); G_FIGURE_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_TIME() -> Py { ensure_init(); G_FIGURE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_TIME(v: Py) { ensure_init(); G_FIGURE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_UNITS() -> Py { ensure_init(); G_FIGURE_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_UNITS(v: Py) { ensure_init(); G_FIGURE_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_ASK() -> Py { ensure_init(); G_FIRST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_ASK(v: Py) { ensure_init(); G_FIRST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_PHRASES() -> Py { ensure_init(); G_FOOD_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_PHRASES(v: Py) { ensure_init(); G_FOOD_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_GREET() -> Py { ensure_init(); G_GREET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREET(v: Py) { ensure_init(); G_GREET.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEF_WORDS() -> Py { ensure_init(); G_GRIEF_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEF_WORDS(v: Py) { ensure_init(); G_GRIEF_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_ASK() -> Py { ensure_init(); G_GROUP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_ASK(v: Py) { ensure_init(); G_GROUP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_NAMES_TYPED() -> Py { ensure_init(); G_GROUP_NAMES_TYPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_NAMES_TYPED(v: Py) { ensure_init(); G_GROUP_NAMES_TYPED.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_NAME_ASK() -> Py { ensure_init(); G_GROUP_NAME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_NAME_ASK(v: Py) { ensure_init(); G_GROUP_NAME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_OF() -> Py { ensure_init(); G_GROUP_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_OF(v: Py) { ensure_init(); G_GROUP_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_WORDS() -> Py { ensure_init(); G_GROUP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_WORDS(v: Py) { ensure_init(); G_GROUP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_ANY() -> Py { ensure_init(); G_HAVE_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_ANY(v: Py) { ensure_init(); G_HAVE_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_ANY_WORDS() -> Py { ensure_init(); G_HAVE_ANY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_ANY_WORDS(v: Py) { ensure_init(); G_HAVE_ANY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HELD_THREAD() -> Py { ensure_init(); G_HELD_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELD_THREAD(v: Py) { ensure_init(); G_HELD_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_HER_WORD() -> Py { ensure_init(); G_HER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HER_WORD(v: Py) { ensure_init(); G_HER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MANY_OF_MINE() -> Py { ensure_init(); G_HOW_MANY_OF_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_OF_MINE(v: Py) { ensure_init(); G_HOW_MANY_OF_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_LAW_AFTER_TEXT() -> Py { ensure_init(); G_IN_LAW_AFTER_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_LAW_AFTER_TEXT(v: Py) { ensure_init(); G_IN_LAW_AFTER_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_KEPT_THINGS() -> Py { ensure_init(); G_KEPT_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEPT_THINGS(v: Py) { ensure_init(); G_KEPT_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEY_SLOT() -> Py { ensure_init(); G_KEY_SLOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEY_SLOT(v: Py) { ensure_init(); G_KEY_SLOT.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_ASK() -> Py { ensure_init(); G_KIN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_ASK(v: Py) { ensure_init(); G_KIN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH() -> Py { ensure_init(); G_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH(v: Py) { ensure_init(); G_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMITS() -> Py { ensure_init(); G_LIMITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMITS(v: Py) { ensure_init(); G_LIMITS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMIT_SAY() -> Py { ensure_init(); G_LIMIT_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMIT_SAY(v: Py) { ensure_init(); G_LIMIT_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_LISTED_FIGURE() -> Py { ensure_init(); G_LISTED_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LISTED_FIGURE(v: Py) { ensure_init(); G_LISTED_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK_FIGURE() -> Py { ensure_init(); G_LOOK_BACK_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK_FIGURE(v: Py) { ensure_init(); G_LOOK_BACK_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_MADE_OF_A_VALUE() -> Py { ensure_init(); G_MADE_OF_A_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MADE_OF_A_VALUE(v: Py) { ensure_init(); G_MADE_OF_A_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_MEMORY_POINTER() -> Py { ensure_init(); G_MEMORY_POINTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEMORY_POINTER(v: Py) { ensure_init(); G_MEMORY_POINTER.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_CATCH_UP() -> Py { ensure_init(); G_MISSED_CATCH_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_CATCH_UP(v: Py) { ensure_init(); G_MISSED_CATCH_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_ALT() -> Py { ensure_init(); G_MONTH_ALT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_ALT(v: Py) { ensure_init(); G_MONTH_ALT.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_ORDER() -> Py { ensure_init(); G_MONTH_ORDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_ORDER(v: Py) { ensure_init(); G_MONTH_ORDER.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_WORD() -> Py { ensure_init(); G_MONTH_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_WORD(v: Py) { ensure_init(); G_MONTH_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_WORDS() -> Py { ensure_init(); G_MONTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_WORDS(v: Py) { ensure_init(); G_MONTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_BY() -> Py { ensure_init(); G_NAMED_BY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_BY(v: Py) { ensure_init(); G_NAMED_BY.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_FACT_ASK() -> Py { ensure_init(); G_NAMED_FACT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_FACT_ASK(v: Py) { ensure_init(); G_NAMED_FACT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_ADVICE() -> Py { ensure_init(); G_NAME_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_ADVICE(v: Py) { ensure_init(); G_NAME_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_AGAIN() -> Py { ensure_init(); G_NAME_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_AGAIN(v: Py) { ensure_init(); G_NAME_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_FIX() -> Py { ensure_init(); G_NAME_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_FIX(v: Py) { ensure_init(); G_NAME_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_FIX_WORDS() -> Py { ensure_init(); G_NAME_FIX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_FIX_WORDS(v: Py) { ensure_init(); G_NAME_FIX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_GIVEN() -> Py { ensure_init(); G_NAME_GIVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_GIVEN(v: Py) { ensure_init(); G_NAME_GIVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_GIVEN_WORDS() -> Py { ensure_init(); G_NAME_GIVEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_GIVEN_WORDS(v: Py) { ensure_init(); G_NAME_GIVEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_NOT_A_NAME() -> Py { ensure_init(); G_NAME_NOT_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_NOT_A_NAME(v: Py) { ensure_init(); G_NAME_NOT_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMING_VERB() -> Py { ensure_init(); G_NAMING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMING_VERB(v: Py) { ensure_init(); G_NAMING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_STARTED() -> Py { ensure_init(); G_NOTHING_STARTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_STARTED(v: Py) { ensure_init(); G_NOTHING_STARTED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FACT() -> Py { ensure_init(); G_NOT_A_FACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FACT(v: Py) { ensure_init(); G_NOT_A_FACT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FACT_OF_THEIRS() -> Py { ensure_init(); G_NOT_A_FACT_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FACT_OF_THEIRS(v: Py) { ensure_init(); G_NOT_A_FACT_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NAME_AFTER_CALLED() -> Py { ensure_init(); G_NOT_A_NAME_AFTER_CALLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME_AFTER_CALLED(v: Py) { ensure_init(); G_NOT_A_NAME_AFTER_CALLED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NAME_BEFORE() -> Py { ensure_init(); G_NOT_A_NAME_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME_BEFORE(v: Py) { ensure_init(); G_NOT_A_NAME_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NEW_VALUE() -> Py { ensure_init(); G_NOT_A_NEW_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NEW_VALUE(v: Py) { ensure_init(); G_NOT_A_NEW_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE_VALUE() -> Py { ensure_init(); G_NOT_A_PLACE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE_VALUE(v: Py) { ensure_init(); G_NOT_A_PLACE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_UNIT() -> Py { ensure_init(); G_NOT_A_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_UNIT(v: Py) { ensure_init(); G_NOT_A_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_FROM_THE_REPAIR() -> Py { ensure_init(); G_NOT_FROM_THE_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_FROM_THE_REPAIR(v: Py) { ensure_init(); G_NOT_FROM_THE_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ON_A_MEDIUM() -> Py { ensure_init(); G_NOT_ON_A_MEDIUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ON_A_MEDIUM(v: Py) { ensure_init(); G_NOT_ON_A_MEDIUM.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_RETRACTED() -> Py { ensure_init(); G_NOT_RETRACTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_RETRACTED(v: Py) { ensure_init(); G_NOT_RETRACTED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_PLAN() -> Py { ensure_init(); G_NOT_THE_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_PLAN(v: Py) { ensure_init(); G_NOT_THE_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_PLANS_REPLY() -> Py { ensure_init(); G_NO_PLANS_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_PLANS_REPLY(v: Py) { ensure_init(); G_NO_PLANS_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_RECORD() -> Py { ensure_init(); G_NO_RECORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_RECORD(v: Py) { ensure_init(); G_NO_RECORD.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_SUBJECT_OF_ITS_OWN() -> Py { ensure_init(); G_NO_SUBJECT_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_SUBJECT_OF_ITS_OWN(v: Py) { ensure_init(); G_NO_SUBJECT_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER() -> Py { ensure_init(); G_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER(v: Py) { ensure_init(); G_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_DIGITS() -> Py { ensure_init(); G_NUMBER_DIGITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_DIGITS(v: Py) { ensure_init(); G_NUMBER_DIGITS.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_VALUE() -> Py { ensure_init(); G_NUMBER_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_VALUE(v: Py) { ensure_init(); G_NUMBER_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORD() -> Py { ensure_init(); G_NUMBER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORD(v: Py) { ensure_init(); G_NUMBER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); G_NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); G_NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_BARE_NUMBER() -> Py { ensure_init(); G_OTHER_BARE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_BARE_NUMBER(v: Py) { ensure_init(); G_OTHER_BARE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_FACT() -> Py { ensure_init(); G_OTHER_FACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_FACT(v: Py) { ensure_init(); G_OTHER_FACT.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_KIND_SAY() -> Py { ensure_init(); G_OTHER_KIND_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_KIND_SAY(v: Py) { ensure_init(); G_OTHER_KIND_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_NOUNS() -> Py { ensure_init(); G_OTHER_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_NOUNS(v: Py) { ensure_init(); G_OTHER_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PLACE() -> Py { ensure_init(); G_OTHER_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PLACE(v: Py) { ensure_init(); G_OTHER_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PLACE_PRONOUN() -> Py { ensure_init(); G_OTHER_PLACE_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PLACE_PRONOUN(v: Py) { ensure_init(); G_OTHER_PLACE_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_SAID() -> Py { ensure_init(); G_OTHER_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_SAID(v: Py) { ensure_init(); G_OTHER_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNED_BY_THE_VERB() -> Py { ensure_init(); G_OWNED_BY_THE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNED_BY_THE_VERB(v: Py) { ensure_init(); G_OWNED_BY_THE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIR_WITHOUT_A_MARKER() -> Py { ensure_init(); G_PAIR_WITHOUT_A_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIR_WITHOUT_A_MARKER(v: Py) { ensure_init(); G_PAIR_WITHOUT_A_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_HELD() -> Py { ensure_init(); G_PART_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_HELD(v: Py) { ensure_init(); G_PART_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_KEYS() -> Py { ensure_init(); G_PART_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_KEYS(v: Py) { ensure_init(); G_PART_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_ADVICE_ASK() -> Py { ensure_init(); G_PERSON_ADVICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_ADVICE_ASK(v: Py) { ensure_init(); G_PERSON_ADVICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_AFTER() -> Py { ensure_init(); G_PERSON_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_AFTER(v: Py) { ensure_init(); G_PERSON_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_GROUP() -> Py { ensure_init(); G_PERSON_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_GROUP(v: Py) { ensure_init(); G_PERSON_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_AGE_ANSWER() -> Py { ensure_init(); G_PET_AGE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_AGE_ANSWER(v: Py) { ensure_init(); G_PET_AGE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_AGE_FIX_WORDS() -> Py { ensure_init(); G_PET_AGE_FIX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_AGE_FIX_WORDS(v: Py) { ensure_init(); G_PET_AGE_FIX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_ASK() -> Py { ensure_init(); G_PET_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_ASK(v: Py) { ensure_init(); G_PET_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_LEAD_IN() -> Py { ensure_init(); G_PET_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_LEAD_IN(v: Py) { ensure_init(); G_PET_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_NAME_AGE() -> Py { ensure_init(); G_PET_NAME_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_NAME_AGE(v: Py) { ensure_init(); G_PET_NAME_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_RENAMED_SAY() -> Py { ensure_init(); G_PET_RENAMED_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_RENAMED_SAY(v: Py) { ensure_init(); G_PET_RENAMED_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_WORDS() -> Py { ensure_init(); G_PET_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_WORDS(v: Py) { ensure_init(); G_PET_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACES() -> Py { ensure_init(); G_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACES(v: Py) { ensure_init(); G_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ENDS() -> Py { ensure_init(); G_PLACE_ENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ENDS(v: Py) { ensure_init(); G_PLACE_ENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_NOT() -> Py { ensure_init(); G_PLACE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_NOT(v: Py) { ensure_init(); G_PLACE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_ASKS() -> Py { ensure_init(); G_PLAN_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_ASKS(v: Py) { ensure_init(); G_PLAN_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_ANIMAL() -> Py { ensure_init(); G_PLURAL_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_ANIMAL(v: Py) { ensure_init(); G_PLURAL_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_PEOPLE() -> Py { ensure_init(); G_PLURAL_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PEOPLE(v: Py) { ensure_init(); G_PLURAL_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_PETS() -> Py { ensure_init(); G_PLURAL_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PETS(v: Py) { ensure_init(); G_PLURAL_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_WORD_NOTHING() -> Py { ensure_init(); G_POINTING_WORD_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_WORD_NOTHING(v: Py) { ensure_init(); G_POINTING_WORD_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK_AT_IT() -> Py { ensure_init(); G_POINTS_BACK_AT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK_AT_IT(v: Py) { ensure_init(); G_POINTS_BACK_AT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK_NOTHING() -> Py { ensure_init(); G_POINTS_BACK_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK_NOTHING(v: Py) { ensure_init(); G_POINTS_BACK_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE() -> Py { ensure_init(); G_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE(v: Py) { ensure_init(); G_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANCY_SAID_AGAIN() -> Py { ensure_init(); G_PREGNANCY_SAID_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANCY_SAID_AGAIN(v: Py) { ensure_init(); G_PREGNANCY_SAID_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_OWN_SAY() -> Py { ensure_init(); G_PREGNANT_OWN_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_OWN_SAY(v: Py) { ensure_init(); G_PREGNANT_OWN_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_SAY() -> Py { ensure_init(); G_PREGNANT_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_SAY(v: Py) { ensure_init(); G_PREGNANT_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_WEEKS_SAY() -> Py { ensure_init(); G_PREGNANT_WEEKS_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_WEEKS_SAY(v: Py) { ensure_init(); G_PREGNANT_WEEKS_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_FITS() -> Py { ensure_init(); G_PRONOUN_FITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_FITS(v: Py) { ensure_init(); G_PRONOUN_FITS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_PEOPLE() -> Py { ensure_init(); G_PRONOUN_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_PEOPLE(v: Py) { ensure_init(); G_PRONOUN_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTE_LEADS() -> Py { ensure_init(); G_QUOTE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTE_LEADS(v: Py) { ensure_init(); G_QUOTE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_HINTS() -> Py { ensure_init(); G_RECALL_HINTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_HINTS(v: Py) { ensure_init(); G_RECALL_HINTS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_NAMES_NOTHING() -> Py { ensure_init(); G_RECALL_NAMES_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_NAMES_NOTHING(v: Py) { ensure_init(); G_RECALL_NAMES_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_POSSESSIVE() -> Py { ensure_init(); G_RECALL_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_POSSESSIVE(v: Py) { ensure_init(); G_RECALL_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_SLOTS() -> Py { ensure_init(); G_RECALL_SLOTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_SLOTS(v: Py) { ensure_init(); G_RECALL_SLOTS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_SLOTS_BY_POSSESSIVE() -> Py { ensure_init(); G_RECALL_SLOTS_BY_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_SLOTS_BY_POSSESSIVE(v: Py) { ensure_init(); G_RECALL_SLOTS_BY_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_WRAPPER() -> Py { ensure_init(); G_RECALL_WRAPPER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_WRAPPER(v: Py) { ensure_init(); G_RECALL_WRAPPER.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION_ALIKE() -> Py { ensure_init(); G_RELATION_ALIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION_ALIKE(v: Py) { ensure_init(); G_RELATION_ALIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATIVE_MONTH() -> Py { ensure_init(); G_RELATIVE_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATIVE_MONTH(v: Py) { ensure_init(); G_RELATIVE_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_REMIND_QUESTION() -> Py { ensure_init(); G_REMIND_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMIND_QUESTION(v: Py) { ensure_init(); G_REMIND_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_ADVERB() -> Py { ensure_init(); G_REPEAT_ADVERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_ADVERB(v: Py) { ensure_init(); G_REPEAT_ADVERB.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_WORDS() -> Py { ensure_init(); G_REPEAT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_WORDS(v: Py) { ensure_init(); G_REPEAT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_RESTATED_KEYS() -> Py { ensure_init(); G_RESTATED_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESTATED_KEYS(v: Py) { ensure_init(); G_RESTATED_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_RESTRICT_OPENS() -> Py { ensure_init(); G_RESTRICT_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESTRICT_OPENS(v: Py) { ensure_init(); G_RESTRICT_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_RESUME() -> Py { ensure_init(); G_RESUME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESUME(v: Py) { ensure_init(); G_RESUME.with(|c| *c.borrow_mut() = v); }
pub fn g_RETRACT_SAY() -> Py { ensure_init(); G_RETRACT_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETRACT_SAY(v: Py) { ensure_init(); G_RETRACT_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_SAY_BACK() -> Py { ensure_init(); G_ROUTINE_SAY_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SAY_BACK(v: Py) { ensure_init(); G_ROUTINE_SAY_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_SUBJECT() -> Py { ensure_init(); G_ROUTINE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SUBJECT(v: Py) { ensure_init(); G_ROUTINE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AGAIN() -> Py { ensure_init(); G_SAID_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AGAIN(v: Py) { ensure_init(); G_SAID_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_ASK() -> Py { ensure_init(); G_SAID_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_ASK(v: Py) { ensure_init(); G_SAID_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AS_A_SENTENCE() -> Py { ensure_init(); G_SAID_AS_A_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_A_SENTENCE(v: Py) { ensure_init(); G_SAID_AS_A_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_FACT_THEIR_OWN() -> Py { ensure_init(); G_SAME_FACT_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_FACT_THEIR_OWN(v: Py) { ensure_init(); G_SAME_FACT_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF() -> Py { ensure_init(); G_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF(v: Py) { ensure_init(); G_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_SHE_OR_HE() -> Py { ensure_init(); G_SHE_OR_HE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHE_OR_HE(v: Py) { ensure_init(); G_SHE_OR_HE.with(|c| *c.borrow_mut() = v); }
pub fn g_SLOT_KEY() -> Py { ensure_init(); G_SLOT_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLOT_KEY(v: Py) { ensure_init(); G_SLOT_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_SLOT_WORDS() -> Py { ensure_init(); G_SLOT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLOT_WORDS(v: Py) { ensure_init(); G_SLOT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_NAMES() -> Py { ensure_init(); G_SMALL_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_NAMES(v: Py) { ensure_init(); G_SMALL_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_SORRY_OPENS_A_CORRECTION() -> Py { ensure_init(); G_SORRY_OPENS_A_CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORRY_OPENS_A_CORRECTION(v: Py) { ensure_init(); G_SORRY_OPENS_A_CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLING_APART() -> Py { ensure_init(); G_SPELLING_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLING_APART(v: Py) { ensure_init(); G_SPELLING_APART.with(|c| *c.borrow_mut() = v); }
pub fn g_STATES_A_LIMIT() -> Py { ensure_init(); G_STATES_A_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATES_A_LIMIT(v: Py) { ensure_init(); G_STATES_A_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_WAS_SAY() -> Py { ensure_init(); G_STATE_WAS_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_WAS_SAY(v: Py) { ensure_init(); G_STATE_WAS_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_THE_SAME_PERSON() -> Py { ensure_init(); G_STILL_THE_SAME_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_THE_SAME_PERSON(v: Py) { ensure_init(); G_STILL_THE_SAME_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_STOPWORDS() -> Py { ensure_init(); G_STOPWORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOPWORDS(v: Py) { ensure_init(); G_STOPWORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SWAPPABLE() -> Py { ensure_init(); G_SWAPPABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWAPPABLE(v: Py) { ensure_init(); G_SWAPPABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_WINDOW() -> Py { ensure_init(); G_TAIL_WINDOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_WINDOW(v: Py) { ensure_init(); G_TAIL_WINDOW.with(|c| *c.borrow_mut() = v); }
pub fn g_TASTE_ASKED_ABOUT() -> Py { ensure_init(); G_TASTE_ASKED_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TASTE_ASKED_ABOUT(v: Py) { ensure_init(); G_TASTE_ASKED_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_THAT_DATE() -> Py { ensure_init(); G_THAT_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THAT_DATE(v: Py) { ensure_init(); G_THAT_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_AGE() -> Py { ensure_init(); G_THEIR_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_AGE(v: Py) { ensure_init(); G_THEIR_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_JOB() -> Py { ensure_init(); G_THEIR_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_JOB(v: Py) { ensure_init(); G_THEIR_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME() -> Py { ensure_init(); G_THEIR_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME(v: Py) { ensure_init(); G_THEIR_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_A_YES_NO() -> Py { ensure_init(); G_THEIR_TASTE_A_YES_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_A_YES_NO(v: Py) { ensure_init(); G_THEIR_TASTE_A_YES_NO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_INTO() -> Py { ensure_init(); G_THEIR_TASTE_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_INTO(v: Py) { ensure_init(); G_THEIR_TASTE_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_WHAT() -> Py { ensure_init(); G_THEIR_TASTE_WHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_WHAT(v: Py) { ensure_init(); G_THEIR_TASTE_WHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEM() -> Py { ensure_init(); G_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEM(v: Py) { ensure_init(); G_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_ARE_NAMED() -> Py { ensure_init(); G_THEY_ARE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_ARE_NAMED(v: Py) { ensure_init(); G_THEY_ARE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_KIDS() -> Py { ensure_init(); G_THE_KIDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_KIDS(v: Py) { ensure_init(); G_THE_KIDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PLACE_IS_NAMED() -> Py { ensure_init(); G_THE_PLACE_IS_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PLACE_IS_NAMED(v: Py) { ensure_init(); G_THE_PLACE_IS_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_AS_WELL() -> Py { ensure_init(); G_TOO_AS_WELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_AS_WELL(v: Py) { ensure_init(); G_TOO_AS_WELL.with(|c| *c.borrow_mut() = v); }
pub fn g_TURNED() -> Py { ensure_init(); G_TURNED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TURNED(v: Py) { ensure_init(); G_TURNED.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_FAMILY() -> Py { ensure_init(); G_UNIT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_FAMILY(v: Py) { ensure_init(); G_UNIT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_SECONDS() -> Py { ensure_init(); G_UNIT_SECONDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_SECONDS(v: Py) { ensure_init(); G_UNIT_SECONDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK() -> Py { ensure_init(); G_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK(v: Py) { ensure_init(); G_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAY_WORD() -> Py { ensure_init(); G_WEEKDAY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_WORD(v: Py) { ensure_init(); G_WEEKDAY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKS_FIX() -> Py { ensure_init(); G_WEEKS_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKS_FIX(v: Py) { ensure_init(); G_WEEKS_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_I_SAID() -> Py { ensure_init(); G_WHAT_I_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_SAID(v: Py) { ensure_init(); G_WHAT_I_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_I_SAID_NOTHING() -> Py { ensure_init(); G_WHAT_I_SAID_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_SAID_NOTHING(v: Py) { ensure_init(); G_WHAT_I_SAID_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASKS() -> Py { ensure_init(); G_WHEN_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASKS(v: Py) { ensure_init(); G_WHEN_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_END_ASK() -> Py { ensure_init(); G_WHEN_END_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_END_ASK(v: Py) { ensure_init(); G_WHEN_END_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_END_WORD() -> Py { ensure_init(); G_WHEN_END_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_END_WORD(v: Py) { ensure_init(); G_WHEN_END_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_INVERTED() -> Py { ensure_init(); G_WHEN_INVERTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_INVERTED(v: Py) { ensure_init(); G_WHEN_INVERTED.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_INVERTED_SUBJECT() -> Py { ensure_init(); G_WHEN_INVERTED_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_INVERTED_SUBJECT(v: Py) { ensure_init(); G_WHEN_INVERTED_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_PLAN() -> Py { ensure_init(); G_WHEN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_PLAN(v: Py) { ensure_init(); G_WHEN_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_QUALITY() -> Py { ensure_init(); G_WHEN_QUALITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_QUALITY(v: Py) { ensure_init(); G_WHEN_QUALITY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_START_ASK() -> Py { ensure_init(); G_WHEN_START_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_START_ASK(v: Py) { ensure_init(); G_WHEN_START_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_THING() -> Py { ensure_init(); G_WHEN_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_THING(v: Py) { ensure_init(); G_WHEN_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_THING_VERB() -> Py { ensure_init(); G_WHEN_THING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_THING_VERB(v: Py) { ensure_init(); G_WHEN_THING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_VAGUE() -> Py { ensure_init(); G_WHEN_VAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_VAGUE(v: Py) { ensure_init(); G_WHEN_VAGUE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_VERB() -> Py { ensure_init(); G_WHEN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_VERB(v: Py) { ensure_init(); G_WHEN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_ASK() -> Py { ensure_init(); G_WHERE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_ASK(v: Py) { ensure_init(); G_WHERE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_ASK() -> Py { ensure_init(); G_WHOLE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_ASK(v: Py) { ensure_init(); G_WHOLE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_NUMBERS() -> Py { ensure_init(); G_WORD_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_NUMBERS(v: Py) { ensure_init(); G_WORD_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_ON_IT() -> Py { ensure_init(); G_WORK_ON_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_ON_IT(v: Py) { ensure_init(); G_WORK_ON_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_NO_ASK() -> Py { ensure_init(); G_YES_NO_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_NO_ASK(v: Py) { ensure_init(); G_YES_NO_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_WHO() -> Py { ensure_init(); G__ABOUT_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_WHO(v: Py) { ensure_init(); G__ABOUT_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNTS() -> Py { ensure_init(); G__AMOUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNTS(v: Py) { ensure_init(); G__AMOUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNT_WORDS() -> Py { ensure_init(); G__AMOUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNT_WORDS(v: Py) { ensure_init(); G__AMOUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONVERT() -> Py { ensure_init(); G__CONVERT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONVERT(v: Py) { ensure_init(); G__CONVERT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORD() -> Py { ensure_init(); G__COUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORD(v: Py) { ensure_init(); G__COUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__FACT_SLOT() -> Py { ensure_init(); G__FACT_SLOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FACT_SLOT(v: Py) { ensure_init(); G__FACT_SLOT.with(|c| *c.borrow_mut() = v); }
pub fn g__HEDGE() -> Py { ensure_init(); G__HEDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEDGE(v: Py) { ensure_init(); G__HEDGE.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_WORD() -> Py { ensure_init(); G__PET_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_WORD(v: Py) { ensure_init(); G__PET_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL() -> Py { ensure_init(); G__PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL(v: Py) { ensure_init(); G__PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__RELATION() -> Py { ensure_init(); G__RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RELATION(v: Py) { ensure_init(); G__RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_KEYS() -> Py { ensure_init(); G__SUBJECT_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_KEYS(v: Py) { ensure_init(); G__SUBJECT_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g__TENS_WORDS() -> Py { ensure_init(); G__TENS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TENS_WORDS(v: Py) { ensure_init(); G__TENS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT_SAID() -> Py { ensure_init(); G__UNIT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT_SAID(v: Py) { ensure_init(); G__UNIT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKLY() -> Py { ensure_init(); G__WEEKLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKLY(v: Py) { ensure_init(); G__WEEKLY.with(|c| *c.borrow_mut() = v); }
pub fn g__concept() -> Py { ensure_init(); G__concept.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__concept(v: Py) { ensure_init(); G__concept.with(|c| *c.borrow_mut() = v); }
pub fn g__country() -> Py { ensure_init(); G__country.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__country(v: Py) { ensure_init(); G__country.with(|c| *c.borrow_mut() = v); }
pub fn g__enough_reply() -> Py { ensure_init(); G__enough_reply.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__enough_reply(v: Py) { ensure_init(); G__enough_reply.with(|c| *c.borrow_mut() = v); }
pub fn g__word() -> Py { ensure_init(); G__word.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__word(v: Py) { ensure_init(); G__word.with(|c| *c.borrow_mut() = v); }
pub fn g__words() -> Py { ensure_init(); G__words.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__words(v: Py) { ensure_init(); G__words.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The memory store at work: remember what the user says, and answer questions about it.\n\nPRIORITY 7000 (README routing bands: memory recall that must lead). It observes every line and\nkeeps the shared keys in bot/memory/; it only proposes a reply when the user asks about the\nconversation itself (\"what did I say my job was?\", \"what were we talking about?\"), so it passes\non every ordinary question.\n\nOne exception, in the same spirit: when the person hands a choice back (\"you pick\", \"your\ncall\"), the answer is one of the options the bot itself offered a turn ago, so the store reads\nthe bot's own last reply for them (bot/memory/choice.py).\n");
    s_A_QUESTION_ASKED(gen_memory::memory::places::g_A_QUESTION_ASKED());
    s_NOT_A_PLACE_VALUE(gen_memory::memory::places::g_NOT_A_PLACE_VALUE());
    s_OTHER_PLACE(gen_memory::memory::places::g_OTHER_PLACE());
    s_OTHER_PLACE_PRONOUN(gen_memory::memory::places::g_OTHER_PLACE_PRONOUN());
    s_PLACE_ENDS(gen_memory::memory::places::g_PLACE_ENDS());
    s_ASKS_ABOUT_A_TASTE(gen_memory::memory::taste::g_ASKS_ABOUT_A_TASTE());
    s_ASKS_FOR_TASTES(gen_memory::memory::taste::g_ASKS_FOR_TASTES());
    s_TASTE_ASKED_ABOUT(gen_memory::memory::taste::g_TASTE_ASKED_ABOUT());
    s_ACKS(gen_memory::memory::topics::g_ACKS());
    s_GREET(gen_memory::memory::topics::g_GREET());
    s_STOPWORDS(gen_memory::memory::topics::g_STOPWORDS());
    s__HEDGE(gen_memory::memory::repeat::g_HEDGE());
    s_PLACES(gen_understanding::understanding::topics::g_PLACES());
    s_PRIORITY(gen_memory::memory::store_lines::g_PRIORITY());
    s_ANSWERS(gen_memory::memory::store_lines::g_ANSWERS());
    s_RECALL_HINTS(gen_memory::memory::store_lines::g_RECALL_HINTS());
    s_SELF(gen_memory::memory::store_lines::g_SELF());
    s_NOT_THE_PLAN(gen_memory::memory::store_lines::g_NOT_THE_PLAN());
    s_NOT_ON_A_MEDIUM(gen_memory::memory::store_lines::g_NOT_ON_A_MEDIUM());
    s_REPEAT_ADVERB(gen_memory::memory::store_lines::g_REPEAT_ADVERB());
    s_LIMITS(gen_memory::memory::store_lines::g_LIMITS());
    s_DENIAL_SAY(gen_memory::memory::store_lines::g_DENIAL_SAY());
    s_CORRECTED_SAY(gen_memory::memory::store_lines::g_CORRECTED_SAY());
    s_RETRACT_SAY(gen_memory::memory::store_lines::g_RETRACT_SAY());
    s_NOT_RETRACTED(gen_memory::memory::store_lines::g_NOT_RETRACTED());
    s__FACT_SLOT(gen_memory::memory::store_lines::g__FACT_SLOT());
    s_PET_AGE_FIX_WORDS(gen_memory::memory::store_lines::g_PET_AGE_FIX_WORDS());
    s_OTHER_SAID(gen_memory::memory::store_lines::g_OTHER_SAID());
    s_OTHER_KIND_SAY(gen_memory::memory::store_lines::g_OTHER_KIND_SAY());
    s_AGE_FIX_WORDS(gen_memory::memory::store_lines::g_AGE_FIX_WORDS());
    s_NAME_FIX_WORDS(gen_memory::memory::store_lines::g_NAME_FIX_WORDS());
    s_NAME_GIVEN_WORDS(gen_memory::memory::store_lines::g_NAME_GIVEN_WORDS());
    s_UNIT_FAMILY(gen_memory::memory::store_lines::g_UNIT_FAMILY());
    s_FACT_FAMILY(gen_memory::memory::store_lines::g_FACT_FAMILY());
    s_NOT_A_UNIT(gen_memory::memory::store_lines::g_NOT_A_UNIT());
    s_AMOUNT_WORD(gen_memory::memory::store_lines::g_AMOUNT_WORD());
    s_SPELLING_APART(gen_memory::memory::store_lines::g_SPELLING_APART());
    s_CORRECTION_SAY_DROP(gen_memory::memory::store_lines::g_CORRECTION_SAY_DROP());
    s_DROP_LISTS(gen_memory::memory::store_lines::g_DROP_LISTS());
    s_CORRECTION_SAY(gen_memory::memory::store_lines::g_CORRECTION_SAY());
    s_CORRECTION_SAY_PAIR(gen_memory::memory::store_lines::g_CORRECTION_SAY_PAIR());
    s_PET_RENAMED_SAY(gen_memory::memory::store_lines::g_PET_RENAMED_SAY());
    s_BARE_ECHO_ONLY(gen_memory::memory::store_lines::g_BARE_ECHO_ONLY());
    s_CORRECTION_BACK(gen_memory::memory::store_lines::g_CORRECTION_BACK());
    s_AT_TIME_VALUE(gen_memory::memory::store_lines::g_AT_TIME_VALUE());
    s_KIN_ASK(gen_memory::memory::store_lines::g_KIN_ASK());
    s_IN_LAW_AFTER_TEXT(gen_memory::memory::store_lines::g_IN_LAW_AFTER_TEXT());
    s_PRONOUN_FITS(gen_memory::memory::store_lines::g_PRONOUN_FITS());
    s_CHILD_RELATIONS(gen_memory::memory::store_lines::g_CHILD_RELATIONS());
    s__ABOUT_WHO(gen_memory::memory::store_lines::g__ABOUT_WHO());
    s_CHILD(gen_memory::memory::store_lines::g_CHILD());
    s_CHILD_OF(gen_memory::memory::store_lines::g_CHILD_OF());
    s_FAMILY_GROUP(gen_memory::memory::store_lines::g_FAMILY_GROUP());
    s_FOOD_PHRASES(gen_memory::memory::store_lines::g_FOOD_PHRASES());
    s_ADVICE_MODULES(gen_memory::memory::store_lines::g_ADVICE_MODULES());
    s_SLOT_KEY(gen_memory::memory::store_lines::g_SLOT_KEY());
    s_BARE_SLOTS(gen_memory::memory::store_lines::g_BARE_SLOTS());
    s_WHERE_ASK(gen_memory::memory::store_lines::g_WHERE_ASK());
    s_KIN(gen_memory::memory::store_lines::g_KIN());
    s_BAD_NEWS(gen_memory::memory::store_lines::g_BAD_NEWS());
    s__UNIT_SAID(gen_memory::memory::store_lines::g__UNIT_SAID());
    s__AMOUNTS(gen_memory::memory::store_lines::g__AMOUNTS());
    s__AMOUNT_WORDS(gen_memory::memory::store_lines::g__AMOUNT_WORDS());
    s__COUNT_WORD(gen_memory::memory::store_lines::g__COUNT_WORD());
    s__CONVERT(gen_memory::memory::store_lines::g__CONVERT());
    s__WEEKLY(gen_memory::memory::store_lines::g__WEEKLY());
    s_FIGURE_UNITS(gen_memory::memory::store_lines::g_FIGURE_UNITS());
    s_FIGURE_TIME(gen_memory::memory::store_lines::g_FIGURE_TIME());
    s_FIGURE_NUMBER(gen_memory::memory::store_lines::g_FIGURE_NUMBER());
    s_FIGURE_QUALIFIER(gen_memory::memory::store_lines::g_FIGURE_QUALIFIER());
    s_LOOK_BACK_FIGURE(gen_memory::memory::store_lines::g_LOOK_BACK_FIGURE());
    s_FIGURE_ANSWER_LEN(gen_memory::memory::store_lines::g_FIGURE_ANSWER_LEN());
    s_COUNT_WORD(gen_memory::memory::store_lines::g_COUNT_WORD());
    s_PLACE_NOT(gen_memory::memory::store_lines::g_PLACE_NOT());
    s_TURNED(gen_memory::memory::store_lines::g_TURNED());
    s_PART_HELD(gen_memory::memory::store_lines::g_PART_HELD());
    s_PART_KEYS(gen_memory::memory::store_lines::g_PART_KEYS());
    s_KEPT_THINGS(gen_memory::memory::store_lines::g_KEPT_THINGS());
    s_NAMED_BY(gen_memory::memory::store_lines::g_NAMED_BY());
    s_RESTATED_KEYS(gen_memory::memory::store_lines::g_RESTATED_KEYS());
    s_SWAPPABLE(gen_memory::memory::store_lines::g_SWAPPABLE());
    s__TENS_WORDS(gen_memory::memory::store_lines::g__TENS_WORDS());
    s_CORRECTED_AGAIN(gen_memory::memory::store_lines::g_CORRECTED_AGAIN());
    s_CORRECTED_AGAIN_WHO(gen_memory::memory::store_lines::g_CORRECTED_AGAIN_WHO());
    s_CORRECTED_WHICH_PERSON(gen_memory::memory::store_lines::g_CORRECTED_WHICH_PERSON());
    s_CAT_NAMES(gen_memory::memory::store_lines::g_CAT_NAMES());
    s_DOG_NAMES(gen_memory::memory::store_lines::g_DOG_NAMES());
    s_SMALL_NAMES(gen_memory::memory::store_lines::g_SMALL_NAMES());
    s_NAME_ADVICE(gen_memory::memory::store_lines::g_NAME_ADVICE());
    s_RESUME(gen_memory::memory::store_lines::g_RESUME());
    s_NO_PLANS_REPLY(gen_memory::memory::store_lines::g_NO_PLANS_REPLY());
    s_WEEK(gen_memory::memory::store_lines::g_WEEK());
    s_TAIL_WINDOW(gen_memory::memory::store_lines::g_TAIL_WINDOW());
    s_PLURAL_PEOPLE(gen_memory::memory::store_lines::g_PLURAL_PEOPLE());
    s_DIET_RULES_OUT(gen_memory::memory::store_lines::g_DIET_RULES_OUT());
    s_DENIAL_WORDS(gen_memory::memory::store_lines::g_DENIAL_WORDS());
    s_DENIAL_STEMS(gen_memory::memory::store_lines::g_DENIAL_STEMS());
    s_CHILD_RELATIONS(gen_memory::memory::store_lines::g_CHILD_RELATIONS());
    s_GROUP_OF(gen_memory::memory::store_lines::g_GROUP_OF());
    s_RELATION_ALIKE(gen_memory::memory::store_lines::g_RELATION_ALIKE());
    s_MONTH_ORDER(gen_memory::memory::store_lines::g_MONTH_ORDER());
    s_HAVE_ANY_WORDS(gen_memory::memory::store_lines::g_HAVE_ANY_WORDS());
    s_HER_WORD(gen_memory::memory::store_lines::g_HER_WORD());
    s_DIET_FOOD(gen_memory::memory::store_lines::g_DIET_FOOD());
    s_PERSON_GROUP(gen_memory::memory::store_lines::g_PERSON_GROUP());
    s_PLURAL_PEOPLE(gen_memory::memory::store_lines::g_PLURAL_PEOPLE());
    s_OWNED_BY_THE_VERB(gen_memory::memory::store_lines::g_OWNED_BY_THE_VERB());
    s_PREGNANT_OWN_SAY(gen_memory::memory::store_lines::g_PREGNANT_OWN_SAY());
    s_PREGNANT_SAY(gen_memory::memory::store_lines::g_PREGNANT_SAY());
    s_PREGNANT_WEEKS_SAY(gen_memory::memory::store_lines::g_PREGNANT_WEEKS_SAY());
    s_LIMIT_SAY(gen_memory::memory::store_lines::g_LIMIT_SAY());
    s_FEEDING_SAY(gen_memory::memory::store_lines::g_FEEDING_SAY());
    s_STATE_WAS_SAY(gen_memory::memory::store_lines::g_STATE_WAS_SAY());
    s_PET_WORDS(gen_memory::memory::store_lines::g_PET_WORDS());
    s_PRONOUN_PEOPLE(gen_memory::memory::store_lines::g_PRONOUN_PEOPLE());
    s_SAME_FACT_THEIR_OWN(gen_memory::memory::store_lines::g_SAME_FACT_THEIR_OWN());
    s_CONDITION_RULES_OUT(gen_memory::memory::store_lines::g_CONDITION_RULES_OUT());
    s_CONDITION_KEEPS_DOWN(gen_memory::memory::store_lines::g_CONDITION_KEEPS_DOWN());
    s_CAR_CAPS(gen_memory::memory::store_lines::g_CAR_CAPS());
    s_NUMBER_WORDS(gen_memory::memory::store_lines::g_NUMBER_WORDS());
    s__SUBJECT_KEYS(gen_memory::memory::store_lines::g__SUBJECT_KEYS());
    s_PLURAL_PETS(gen_memory::memory::store_lines::g_PLURAL_PETS());
    s_COUNT_WORDS(gen_memory::memory::store_lines::g_COUNT_WORDS());
    s_A_SHORT_ANSWER(gen_memory::memory::store_lines::g_A_SHORT_ANSWER());
    s_KEY_SLOT(gen_memory::memory::store_lines::g_KEY_SLOT());
    s_CONCEPTS(gen_memory::memory::store_lines::g_CONCEPTS());
    s_POSSESSIVE(gen_memory::memory::store_lines::g_POSSESSIVE());
    s_QUOTE_LEADS(gen_memory::memory::store_lines::g_QUOTE_LEADS());
    s_ASKED_LEADS(gen_memory::memory::store_lines::g_ASKED_LEADS());
    s_NOTHING_STARTED(gen_memory::memory::store_lines::g_NOTHING_STARTED());
    s_POINTS_BACK_NOTHING(gen_memory::memory::store_lines::g_POINTS_BACK_NOTHING());
    s_POINTING_WORD_NOTHING(gen_memory::memory::store_lines::g_POINTING_WORD_NOTHING());
    s_WHAT_I_SAID_NOTHING(gen_memory::memory::store_lines::g_WHAT_I_SAID_NOTHING());
    s_RESTRICT_OPENS(gen_memory::memory::store_lines::g_RESTRICT_OPENS());
    s_CALL_LEADS(gen_memory::memory::store_lines::g_CALL_LEADS());
    s_LENGTH(gen_memory::memory::store_lines::g_LENGTH());
    s_WORD_NUMBERS(gen_memory::memory::store_lines::g_WORD_NUMBERS());
    s_UNIT_SECONDS(gen_memory::memory::store_lines::g_UNIT_SECONDS());
    s_BUDGET_FITS(gen_memory::memory::store_lines::g_BUDGET_FITS());
    s_BUDGET_MORE(gen_memory::memory::store_lines::g_BUDGET_MORE());
    s_BUDGET_TIGHT(gen_memory::memory::store_lines::g_BUDGET_TIGHT());
    s_ANY_FIGURE(gen_memory::memory::store_lines::g_ANY_FIGURE());
    s_HELD_THREAD(gen_memory::memory::store_lines::g_HELD_THREAD());
    s_GRIEF_WORDS(gen_memory::memory::store_lines::g_GRIEF_WORDS());
    s_CLOSE_PLAIN(gen_memory::memory::store_lines::g_CLOSE_PLAIN());
    s_CLOSE_HEAVY(gen_memory::memory::store_lines::g_CLOSE_HEAVY());
    s_CHANGE_PLAIN(gen_memory::memory::store_lines::g_CHANGE_PLAIN());
    s_CHANGE_HEAVY(gen_memory::memory::store_lines::g_CHANGE_HEAVY());
    s_CHECK_NOTHING_SAID(gen_memory::memory::store_lines::g_CHECK_NOTHING_SAID());
    s_CHECK_A_FEELING(gen_memory::memory::store_lines::g_CHECK_A_FEELING());
    s_CHECK_A_QUESTION(gen_memory::memory::store_lines::g_CHECK_A_QUESTION());
    s_CHECK_PLAIN(gen_memory::memory::store_lines::g_CHECK_PLAIN());
    s_CLOCK_SAID(gen_memory::memory::store_lines::g_CLOCK_SAID());
    s_ROUTINE_SUBJECT(gen_memory::memory::store_lines::g_ROUTINE_SUBJECT());
    s_ROUTINE_SAY_BACK(gen_memory::memory::store_lines::g_ROUTINE_SAY_BACK());
    s__enough_reply(gen_memory::memory::store_replies::g_enough_reply());
    s_ABOUT_PERSON_ASK(gen_memory::memory::relations::g_ABOUT_PERSON_ASK());
    s_AGE_ADVERB(gen_memory::memory::relations::g_AGE_ADVERB());
    s_AGE_AGAIN(gen_memory::memory::relations::g_AGE_AGAIN());
    s_AGE_ASK(gen_memory::memory::relations::g_AGE_ASK());
    s_AGE_AS_A_CAUSE(gen_memory::memory::relations::g_AGE_AS_A_CAUSE());
    s_AGE_COPULA(gen_memory::memory::relations::g_AGE_COPULA());
    s_AGE_DETERMINER(gen_memory::memory::relations::g_AGE_DETERMINER());
    s_AGE_PERSON_WORD(gen_memory::memory::relations::g_AGE_PERSON_WORD());
    s_AGE_RULE_WORD(gen_memory::memory::relations::g_AGE_RULE_WORD());
    s_AGE_SAID(gen_memory::memory::relations::g_AGE_SAID());
    s_AGE_WORD(gen_memory::memory::relations::g_AGE_WORD());
    s_AGE_YOU_SAY(gen_memory::memory::relations::g_AGE_YOU_SAY());
    s_ANOTHER_PERSON(gen_memory::memory::relations::g_ANOTHER_PERSON());
    s_BREED_ASK(gen_memory::memory::relations::g_BREED_ASK());
    s_CHAT_TAIL(gen_memory::memory::relations::g_CHAT_TAIL());
    s_CHILD_WORD(gen_memory::memory::relations::g_CHILD_WORD());
    s_GROUP_ASK(gen_memory::memory::relations::g_GROUP_ASK());
    s_GROUP_NAMES_TYPED(gen_memory::memory::relations::g_GROUP_NAMES_TYPED());
    s_GROUP_NAME_ASK(gen_memory::memory::relations::g_GROUP_NAME_ASK());
    s_GROUP_WORDS(gen_memory::memory::relations::g_GROUP_WORDS());
    s_HAVE_ANY(gen_memory::memory::relations::g_HAVE_ANY());
    s_NAME_AGAIN(gen_memory::memory::relations::g_NAME_AGAIN());
    s_PERSON_AFTER(gen_memory::memory::relations::g_PERSON_AFTER());
    s_PET_ASK(gen_memory::memory::relations::g_PET_ASK());
    s_PLURAL_ANIMAL(gen_memory::memory::relations::g_PLURAL_ANIMAL());
    s_THEIR_AGE(gen_memory::memory::relations::g_THEIR_AGE());
    s_THEIR_JOB(gen_memory::memory::relations::g_THEIR_JOB());
    s_THEIR_NAME(gen_memory::memory::relations::g_THEIR_NAME());
    s_THEIR_TASTE_A_YES_NO(gen_memory::memory::relations::g_THEIR_TASTE_A_YES_NO());
    s_THEIR_TASTE_INTO(gen_memory::memory::relations::g_THEIR_TASTE_INTO());
    s_THEIR_TASTE_WHAT(gen_memory::memory::relations::g_THEIR_TASTE_WHAT());
    s_THEY_ARE_NAMED(gen_memory::memory::relations::g_THEY_ARE_NAMED());
    s_THE_KIDS(gen_memory::memory::relations::g_THE_KIDS());
    s__PET_WORD(gen_memory::memory::relations::g__PET_WORD());
    s__PLURAL(gen_memory::memory::relations::g__PLURAL());
    s_CALENDAR_SAID(crate::modules::memory_tables::g_CALENDAR_SAID());
    s_COUNT_ANIMAL(crate::modules::memory_tables::g_COUNT_ANIMAL());
    s_COUNT_BARE(crate::modules::memory_tables::g_COUNT_BARE());
    s_DAY_ASKS(crate::modules::memory_tables::g_DAY_ASKS());
    s_DAY_OF_MONTH(crate::modules::memory_tables::g_DAY_OF_MONTH());
    s_ELIDED_AGAIN(crate::modules::memory_tables::g_ELIDED_AGAIN());
    s_ELIDED_ASK(crate::modules::memory_tables::g_ELIDED_ASK());
    s_ELIDED_NAMES(crate::modules::memory_tables::g_ELIDED_NAMES());
    s_ELIDED_NOT_SURE(crate::modules::memory_tables::g_ELIDED_NOT_SURE());
    s_ELIDED_WORDS(crate::modules::memory_tables::g_ELIDED_WORDS());
    s_HOW_MANY_OF_MINE(crate::modules::memory_tables::g_HOW_MANY_OF_MINE());
    s_MEMORY_POINTER(crate::modules::memory_tables::g_MEMORY_POINTER());
    s_MISSED_CATCH_UP(crate::modules::memory_tables::g_MISSED_CATCH_UP());
    s_PLAN_ASKS(crate::modules::memory_tables::g_PLAN_ASKS());
    s_POINTS_BACK(crate::modules::memory_tables::g_POINTS_BACK());
    s_SAID_ASK(crate::modules::memory_tables::g_SAID_ASK());
    s_SAID_AS_A_SENTENCE(crate::modules::memory_tables::g_SAID_AS_A_SENTENCE());
    s_THAT_DATE(crate::modules::memory_tables::g_THAT_DATE());
    s_WEEKDAY_WORD(crate::modules::memory_tables::g_WEEKDAY_WORD());
    s_WHAT_I_SAID(crate::modules::memory_tables::g_WHAT_I_SAID());
    s_WHEN_ASKS(crate::modules::memory_tables::g_WHEN_ASKS());
    s_WHEN_END_ASK(crate::modules::memory_tables::g_WHEN_END_ASK());
    s_WHEN_END_WORD(crate::modules::memory_tables::g_WHEN_END_WORD());
    s_WHEN_INVERTED(crate::modules::memory_tables::g_WHEN_INVERTED());
    s_WHEN_INVERTED_SUBJECT(crate::modules::memory_tables::g_WHEN_INVERTED_SUBJECT());
    s_WHEN_PLAN(crate::modules::memory_tables::g_WHEN_PLAN());
    s_WHEN_QUALITY(crate::modules::memory_tables::g_WHEN_QUALITY());
    s_WHEN_START_ASK(crate::modules::memory_tables::g_WHEN_START_ASK());
    s_WHEN_THING(crate::modules::memory_tables::g_WHEN_THING());
    s_WHEN_THING_VERB(crate::modules::memory_tables::g_WHEN_THING_VERB());
    s_WHEN_VAGUE(crate::modules::memory_tables::g_WHEN_VAGUE());
    s_WHEN_VERB(crate::modules::memory_tables::g_WHEN_VERB());
    s_REMIND_QUESTION(stdlib_call("re", "compile", &[py_str("\\bremind me\\b[^.!?]{0,24}\\b(?:what|when|where|who|whom|whose|how|which|why|if)\\b|\\bremind me\\b(?:[\\s,]*(?:again|please))*[\\s,]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FACT(stdlib_call("re", "compile", &[py_str("\\bhaven't (?:told|said|mentioned|given|got)\\b|\\bhaven't your\\b|\\bdon't know\\b|\\bnot sure\\b|\\bnothing comes to mind\\b|\\bdidn't say\\b|\\bnever told\\b|\\bno idea\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_FROM_THE_REPAIR(call_builtin("frozenset", &[mcall(&py_str("plan plans drop retract correction"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NAME_BEFORE(stdlib_call("re", "compile", &[py_str("(?:\\b(?:a|an|the|some|another|his|her|their|our|your|my)\\s+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PLACE_IS_NAMED(stdlib_call("re", "compile", &[py_str("(?:^|[.!?;:]\\s+|\\n|\\b(?:to|in|into|from|at|near|of|around|through|toward|towards|via|going|coming|off|back|flying|travelling|traveling|moving|heading|headed|staying|visiting|visit|bound|left)\\s*(?:\\d+|two|three|four|five|six|seven|eight|nine|ten)?\\s*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_OPENERS(call_builtin("frozenset", &[mcall(&py_str("what whats which when where who whose why how hows\ndo does did is are was were am can could should would will shall have has had\nany anyone anything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGES_IN_WORDS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&gen_memory::memory::facts::g_FIGURE_WORDS(), "items", &[])) { let __unpk6060 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk6060[0].clone(); let mut number: Py = __unpk6060[1].clone(); __out.push((call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), word.clone())); } py_dict(__out) })() });
    s_NAMED_FACT_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\bwhat(?:'s|s| is| are| was| were)?\\s+my\\s+"), &gen_memory::memory::store_lines::g__FACT_SLOT()), &py_str("\\b|\\bwhat\\s+(?:is|are|was|were)\\s+my\\s+")), &gen_memory::memory::store_lines::g__FACT_SLOT()), &py_str("\\b|\\bwhere do i (?:work|live|come from)\\b(?=\\s*(?:[?.!,;:]|$)|\\s+(?:now|again|then|at|from|these|days|properly|though|honestly|anymore|either|oh|wait|sorry|hmm|actually|um|right|please|thanks)\\b)|\\bwhat do i work as\\b|\\bwho am i\\b|\\bhow old am i\\b|\\bwhat do i do for a living\\b")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_ABOUT_THEM(stdlib_call("re", "compile", &[py_str("^\\W*(?:is|are|does|do|did|was|were|can|could|has|have|will)\\b|\\bwhat (?:does|do|did|can|could|is|are|was|were) (?:he|she|they|your|my|his|her|their)\\b|\\bwhat(?:'s|s| is| are)\\s+(?:my|his|her|their|he|she|they)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_ADVICE_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:can|could|should|shall|would|will|may|might|is it (?:ok|okay|fine|safe)|what can|what could|what should|how (?:do|can|should)|any (?:tips|ideas|advice|suggestions|ways))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTS_A_NAME(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:no|nope|sorry|actually|oops|wait|hold on|correction)[,.]?\\s+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTS_A_NAME_POSSESSIVE(stdlib_call("re", "compile", &[py_str("^\\W*(?:his|her|its|their)\\s+names?\\s+(?:actually|really)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_IN_WORDS(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gen_memory::memory::facts::g_FIGURE_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_PET_NAME_AGE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:[a-z][a-z'-]{2,20})\\s+(?:is|was|are|were)\\s+(?P<age>\\d{1,2}|"), &g_AGE_IN_WORDS()), &py_str(")(?![\\d.,]\\d)(?!\\s+of\\b)(?!\\s*(?:kg|kilo\\w*|lbs?|pounds?|stone|st|cm|metres?|meters?|miles?|hours?|mins?|minutes?|percent|%))\\s*(?P<unit>months?|weeks?)?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_LEAD_IN(stdlib_call("re", "compile", &[py_str("^(?:and|also|oh|but|well|yeah|yes|no|sorry|actually|ok|okay|right|wait|god|hang on)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_AGE_ANSWER(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:(?:he|she|it|they)(?:'s|s|\\s+is|\\s+are|\\s+is\\s+now)?\\s+)?(?P<age>\\d{1,2}|"), &g_AGE_IN_WORDS()), &py_str(")\\s*(?P<unit>years?|months?|weeks?)?\\s*(?:old)?\\W*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AMOUNT(gen_memory::memory::corrected::g_AMOUNT());
    s_ANY_NUMBER(gen_memory::memory::corrected::g_ANY_NUMBER());
    s_ASK_START(gen_memory::memory::corrected::g_ASK_START());
    s_AT_TIME(gen_memory::memory::corrected::g_AT_TIME());
    s_A_DAY_BEFORE_A_MONTH(gen_memory::memory::corrected::g_A_DAY_BEFORE_A_MONTH());
    s_A_DAY_SAID_LONG(gen_memory::memory::corrected::g_A_DAY_SAID_LONG());
    s_A_UNIT_SAID(gen_memory::memory::corrected::g_A_UNIT_SAID());
    s_BARE_HOUR(gen_memory::memory::corrected::g_BARE_HOUR());
    s_BARE_NEW_COUNT(gen_memory::memory::corrected::g_BARE_NEW_COUNT());
    s_CORRECTABLE(gen_memory::memory::corrected::g_CORRECTABLE());
    s_COUNT_SAID(gen_memory::memory::corrected::g_COUNT_SAID());
    s_CURRENCY(gen_memory::memory::corrected::g_CURRENCY());
    s_DATE_DAY(gen_memory::memory::corrected::g_DATE_DAY());
    s_DATE_PHRASE(gen_memory::memory::corrected::g_DATE_PHRASE());
    s_DAY_WORD(gen_memory::memory::corrected::g_DAY_WORD());
    s_MADE_OF_A_VALUE(gen_memory::memory::corrected::g_MADE_OF_A_VALUE());
    s_MONTH_ALT(gen_memory::memory::corrected::g_MONTH_ALT());
    s_MONTH_WORD(gen_memory::memory::corrected::g_MONTH_WORD());
    s_MONTH_WORDS(gen_memory::memory::corrected::g_MONTH_WORDS());
    s_NAMING_VERB(gen_memory::memory::corrected::g_NAMING_VERB());
    s_NOT_A_FACT_OF_THEIRS(gen_memory::memory::corrected::g_NOT_A_FACT_OF_THEIRS());
    s_NO_SUBJECT_OF_ITS_OWN(gen_memory::memory::corrected::g_NO_SUBJECT_OF_ITS_OWN());
    s_NUMBER_DIGITS(gen_memory::memory::corrected::g_NUMBER_DIGITS());
    s_NUMBER_VALUE(gen_memory::memory::corrected::g_NUMBER_VALUE());
    s_NUMBER_WORD(gen_memory::memory::corrected::g_NUMBER_WORD());
    s_OTHER_BARE_NUMBER(gen_memory::memory::corrected::g_OTHER_BARE_NUMBER());
    s_RELATIVE_MONTH(gen_memory::memory::corrected::g_RELATIVE_MONTH());
    s_STILL_THE_SAME_PERSON(gen_memory::memory::corrected::g_STILL_THE_SAME_PERSON());
    s_A_HAVE_QUESTION(gen_memory::memory::store_replies::g_A_HAVE_QUESTION());
    s_ACTIVITY_ASK(gen_memory::memory::store_replies::g_ACTIVITY_ASK());
    s_ANSWER_MODULES(gen_memory::memory::store_replies::g_ANSWER_MODULES());
    s_BARE_WHAT_I_DO(gen_memory::memory::store_replies::g_BARE_WHAT_I_DO());
    s_CANT_HAVE(gen_memory::memory::store_replies::g_CANT_HAVE());
    s_CAN_HAVE(gen_memory::memory::store_replies::g_CAN_HAVE());
    s_CAN_THEY_HAVE(gen_memory::memory::store_replies::g_CAN_THEY_HAVE());
    s_CAN_THEY_HAVE_A_FOOD(gen_memory::memory::store_replies::g_CAN_THEY_HAVE_A_FOOD());
    s_CHECK_FOLLOWED(gen_memory::memory::store_replies::g_CHECK_FOLLOWED());
    s_EAT_ASK(gen_memory::memory::store_replies::g_EAT_ASK());
    s_FIRST_ASK(gen_memory::memory::store_replies::g_FIRST_ASK());
    s_NO_RECORD(gen_memory::memory::store_replies::g_NO_RECORD());
    s_OTHER_FACT(gen_memory::memory::store_replies::g_OTHER_FACT());
    s_POINTS_BACK_AT_IT(gen_memory::memory::store_replies::g_POINTS_BACK_AT_IT());
    s_THEM(gen_memory::memory::store_replies::g_THEM());
    s_WHOLE_ASK(gen_memory::memory::store_replies::g_WHOLE_ASK());
    s_YES_NO_ASK(gen_memory::memory::store_replies::g_YES_NO_ASK());
    s__RELATION(gen_memory::memory::store_replies::g__RELATION());
    s_STATES_A_LIMIT(gen_memory::memory::store_replies::g_STATES_A_LIMIT());
    s_REPEAT_WORDS(gen_memory::memory::store_replies::g_REPEAT_WORDS());
    s_AGE_FIX(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:he|she|they)(?:'s|s| is| are| was)\\s+(?:now\\s+)?(?P<age>\\d{1,3}|"), &g_AGE_IN_WORDS()), &py_str(")\\b(?!\\s+of\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTING(stdlib_call("re", "compile", &[py_str("\\b(?:actually|no|nope|sorry|wait|i mean|i meant|make it|makes it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEEKS_FIX(stdlib_call("re", "compile", &[py_str("\\b(?P<weeks>\\d{1,2})\\s*(?:weeks?|wks?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PREGNANCY_SAID_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:pregnant|expecting)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_FIX(stdlib_call("re", "compile", &[py_str("\\b(?:(?:he|she|they)(?:'s|s| is| was)?\\s+(?:actually\\s+|really\\s+)?(?:called|named)|(?:his|her|their)\\s+names?\\s+(?:is|was|actually|really))\\s+(?P<name>[a-z][a-z'\\u2019-]{1,19})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_NOT_A_NAME(call_builtin("frozenset", &[mcall(&py_str("not called named now really also still a an the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_AFTER_A_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they)(?:'s|s| is| was)\\s+(?P<name>[a-z][a-z'\\u2019-]{1,19})\\s*[,.!]?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_WITH_ITS_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:called|named)\\s+(?P<name>[A-Za-z][A-Za-z'-]{1,19})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANOTHER_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:other|second|another|also|third)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTS_A_MAKE(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|actually|no|i mean|make that|or)?[\\s,]*(?:its|it's|it is)\\s+(?:actually\\s+|really\\s+|just\\s+)?(?:a|an|the)\\s+(?P<value>[a-z0-9][a-z0-9' .+-]{0,31})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_GIVEN(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they)(?:'s|s| is| was)?\\s+(?:called|named)\\s+(?P<name>[a-z][a-z'\\-]{1,19})\\b|\\b(?:his|her|their)\\s+name(?:'s|\\s+is)\\s+(?P<name2>[a-z][a-z'\\-]{1,19})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NAME_AFTER_CALLED(call_builtin("frozenset", &[mcall(&py_str("\nin out off up down back away over round around about on at by for with without into onto\nsick ill late busy tired poorly bad good home here there now soon again today tonight\ntomorrow yesterday it them him her me us you one ones the a an\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER(stdlib_call("re", "compile", &[py_str("[£$€]?\\d+(?:[.,]\\d+)?(?:\\s?(?:pounds|quid|euros?|dollars?|k))?(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAIR_WITHOUT_A_MARKER(py_tuple(vec![stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|oh|well|so|and|hmm)[,. ]+)*(?:(?:it'?s|its|that'?s|thats)\\s+)?(?P<to>[a-z0-9][a-z0-9'-]{0,19})\\s*,?\\s+not\\s+(?P<wrong>[a-z0-9][a-z0-9'-]{0,19})[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|oh|well|so|and|hmm)[,. ]+)*not\\s+(?P<wrong>[a-z0-9][a-z0-9'-]{0,19})\\s*[,\\u2013\\u2014;-]\\s*(?:(?:it'?s|its|that'?s)\\s+)?(?P<to>[a-z0-9][a-z0-9'-]{0,19})[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|oh|well|so|and|hmm|sorry|actually|no|nope)[,. ]+)*(?:i'?m|i am|im|it'?s|its|it is|that'?s|thats|we'?re|we are)\\s+not\\s+(?P<wrong>[a-z0-9][a-z0-9'.\\-]{0,19})[\\s,;.!-]*(?:but\\s+)?(?:i'?m|i am|im|it'?s|its|it is|that'?s|thats|we'?re|we are)\\s+(?P<to>[a-z0-9][a-z0-9'.\\-]{0,19})[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_NOT_A_NEW_VALUE(call_builtin("frozenset", &[mcall(&py_str("a an the me you it that this these those my your his her our their\nthanks thank sure fair right wrong now then today tomorrow yesterday fine bad good great sorry\nplease yes yeah no not ok okay yet even both all much many one two three too so there here what\nwhy how who when where true false"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TOO_AS_WELL(stdlib_call("re", "compile", &[py_str("^(?:(?:and|also|but|plus)\\s+)?(?:my|our)\\s+(?P<who>[a-z]+(?:[ -]in[ -]law)?)\\s+(?P<agree>is\\s+as\\s+well|is\\s+too|as\\s+well|also|too)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHER_NOUNS(call_builtin("frozenset", &[mcall(&py_str("friend mate partner baby child kid children kids cousin parent parents"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_MONEY(py_tuple(vec![py_str("money")]));
    s_FIGURE_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|right|then|oh|well)\\W+)*how\\s+(?P<asked>many|much|long|far)\\b(?P<rest>.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the it its that this them those there here then now again\n    roughly exactly approximately about around all told overall altogether total more less much\n    many long do does did will would should can could you i we they he she reckon think suppose\n    guess say again please for of in at on to be have has had is are was were and but so if\n    when after before time times per each"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_AFTER(call_builtin("frozenset", &[mcall(&py_str("is are was were and or a an the in for of to on at by after before\n    now then so but if when that from with without each every per of till until into about\n    to around roughly or while than which where"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTION_DURATION(stdlib_call("re", "compile", &[py_str("\\bat a time\\b|\\bfor the first\\b|\\bfor the next\\b|\\bin a row\\b|\\beach time\\b|\\bthe (?:first|next|last|coming|following|remaining|final)\\b|\\b(?:once|twice|three times|a few times) a day\\b|\\ba day or two\\b|\\bper session\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTION_AMOUNT(stdlib_call("re", "compile", &[py_str("\\b(?:whenever|when|if|once|each time|any time|every time|in case|unless)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_FIGURE(stdlib_call("re", "compile", &[py_str("(?:£|\\$|€)\\s*\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:to|-|or)\\s*(?:£|\\$|€)?\\s*\\d[\\d,]*(?:\\.\\d+)?)?|(?<![\\d.,])\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:to|-|or)\\s*\\d[\\d,]*(?:\\.\\d+)?)?\\s*(?:pounds?|quid|dollars?|euros?|pence)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERCENT_FIGURE(stdlib_call("re", "compile", &[py_str("(?<![\\d.,])\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:to|-|or)\\s*\\d[\\d,]*(?:\\.\\d+)?)?\\s*(?:per\\s?cent|percent|%)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LISTED_FIGURE(stdlib_call("re", "compile", &[py_str("(?:,|;)\\s*\\d+(?:\\s*(?:to|-|or)\\s*\\d+)?\\s+for\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_ASK(stdlib_call("re", "compile", &[py_str("how\\s+much\\b[^.?!]*\\b(?:a|per|each|every)\\s+(?:day|week|month|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_STATED(stdlib_call("re", "compile", &[py_str("(?:£|\\bgbp\\s*)?\\d[\\d,.]*\\s*(?:pounds?\\s+)?(?:a|per|each|every)\\s+(?:day|week|month|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_AGAIN(stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:mean|meant)(?:\\s+to\\s+say)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SORRY_OPENS_A_CORRECTION(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|soz)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_WRAPPER(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|but\\s+|also\\s+|again\\s+)?(?P<word>what|when|how\\s+old)\\s+(?:did|do)\\s+(?:i|we)\\s+(?:say|call|tell\\s+(?:you|u)|told\\s+(?:you|u)|mention)\\s+(?P<rest>.+?)\\s*\\??\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_SLOTS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:names?|called)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("what"), py_str("name")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:birthday|bday|dob|date\\s+of\\s+birth)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("when"), py_str("birthday")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:ages?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("how old"), py_str("age")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:budget|salary|wage|rent|target|savings?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("what"), Py::None]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:address|postcode|number|time|date)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("what"), Py::None])]));
    s_RECALL_POSSESSIVE(stdlib_call("re", "compile", &[py_str("^\\s*(?P<poss>my|our|his|her|their)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_SLOTS_BY_POSSESSIVE(stdlib_call("re", "compile", &[py_str("^\\s*(?:my|our|his|her|their)\\s+(?P<attribute>names?|birthday|bday|dob|ages?|budget|salary|wage|societys?|address|postcode)\\??\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHE_OR_HE(py_dict(vec![(py_str("his"), py_tuple(vec![py_str("he"), py_str("is")])), (py_str("her"), py_tuple(vec![py_str("she"), py_str("is")])), (py_str("their"), py_tuple(vec![py_str("they"), py_str("are")])), (py_str("my"), py_tuple(vec![py_str("i"), py_str("am")])), (py_str("our"), py_tuple(vec![py_str("we"), py_str("are")]))]));
    s_SLOT_WORDS(py_dict(vec![(py_str("age"), stdlib_call("re", "compile", &[py_str("\\b\\d|\\b(?:age|aged|old)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("name"), stdlib_call("re", "compile", &[py_str("\\b(?:name|called|named|calls)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("birthday"), stdlib_call("re", "compile", &[py_str("\\b(?:birthday|bday|dob|january|february|march|april|may|june|july|august|september|october|november|december)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]));
    s_WORK_ON_IT(stdlib_call("re", "compile", &[py_str("\\b[a-z]{3,}\\s+it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("name"), py_str("called"), py_str("age"), py_str("old"), py_str("job"), py_str("work"), py_str("works"), py_str("working"), py_str("city"), py_str("town"), py_str("live"), py_str("lives"), py_str("living"), py_str("moved"), py_str("moving"), py_str("from"), py_str("diet"), py_str("dieting"), py_str("eating"), py_str("food"), py_str("budget"), py_str("money"), py_str("afford"), py_str("allergy"), py_str("allergies"), py_str("allergic"), py_str("pet"), py_str("pets"), py_str("dog"), py_str("cat"), py_str("height"), py_str("tall")])]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("mother"), py_str("mum mom mummy ma mam mother mumma")]), py_tuple(vec![py_str("father"), py_str("dad daddy pa papa father")]), py_tuple(vec![py_str("parent"), py_str("parent parents folks")]), py_tuple(vec![py_str("child"), py_str("kid kids child children baby toddler")]), py_tuple(vec![py_str("dog"), py_str("dog puppy pup dogs")]), py_tuple(vec![py_str("cat"), py_str("cat kitten kitty cats")]), py_tuple(vec![py_str("pet"), py_str("pet pets")]), py_tuple(vec![py_str("car"), py_str("car vehicle motor")]), py_tuple(vec![py_str("home"), py_str("home house flat apartment place")]), py_tuple(vec![py_str("party"), py_str("party birthday bday celebration bash")]), py_tuple(vec![py_str("job"), py_str("job work career")]), py_tuple(vec![py_str("food"), py_str("food dinner tea meal supper")]), py_tuple(vec![py_str("money"), py_str("money budget cash savings")]), py_tuple(vec![py_str("diet"), py_str("diet vegetarian vegan pescatarian")]), py_tuple(vec![py_str("doctor"), py_str("doctor gp surgery hospital")]), py_tuple(vec![py_str("wedding"), py_str("wedding weddings married marry marriage engaged engagement bride groom fiance fiancee honeymoon")]), py_tuple(vec![py_str("holiday"), py_str("holiday holidays vacation trip trips break abroad travelling travel")]), py_tuple(vec![py_str("exam"), py_str("exam exams revision revise revising coursework dissertation thesis mock")]), py_tuple(vec![py_str("gym"), py_str("gym gyms fitter fitness exercise exercising workout workouts jogging running")]), py_tuple(vec![py_str("smoking"), py_str("smoking smoke smoker cigarettes cigarette fags vaping vape nicotine")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    s__concept(__unpk2[0].clone());
    s__words(__unpk2[1].clone());
    let __iter3 = py_iter_or_raise(&mcall(&g__words(), "split", &[]));
    for __item in __iter3 {
    s__word(__item.clone());
    let _ = py_setitem(&gen_memory::memory::store_lines::g_CONCEPTS(), &g__word(), g__concept());
    }
    }
    let __iter4 = py_iter_or_raise(&gen_memory::memory::facts::g_COUNTRIES());
    for __item in __iter4 {
    s__country(__item.clone());
    let _ = mcall(&gen_memory::memory::store_lines::g_CONCEPTS(), "setdefault", &[mcall(&g__country(), "lower", &[]), py_str("holiday")]);
    }
    s_A_RECALL_IN_ANOTHER_SHAPE(stdlib_call("re", "compile", &[py_str("^(?:and|so|but|also|again|ok|okay|well)?\\s*(?:tell\\s+me\\s+what\\s+(?:i|we)\\s+(?:said|say|told\\s+(?:you|u)|tell\\s+(?:you|u)|mentioned?)\\b|(?:did|do)\\s+(?:i|we)\\s+(?:say|tell\\s+(?:you|u)|told\\s+(?:you|u)|mentioned?)\\s+(?:anything|something|much)\\b|what\\s+(?:was|were)\\s+(?:the\\s+thing|that\\s+thing|that)\\s+(?:that\\s+)?(?:i|we)\\s+(?:said|told\\s+(?:you|u)|mentioned?)\\b|remind\\s+me\\s+what\\s+(?:that|it|this|those|they)\\s+(?:was|were|is|are)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_NAMES_NOTHING(call_builtin("frozenset", &[mcall(&py_str("it that this them those these one ones there anything something\nmuch thing things about of on in at to for from with the a an was were is are be been being\ni we you u he she they me us my our his her their said say told tell mention mentioned mentioning\nand or so but also again then just really actually exactly then"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_kept_fact_read(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks back a fact they gave, in whatever words, and the store has it.\n\n    \"my budget is 300\" ... \"im going to rome in may\" ... \"whats my budget\" was answered with\n    the cost of a day in Rome. The ask carries no topic of its own, so the arbiter read the\n    conversation's topic on it and handed the turn to the module that declares that topic. A\n    recall of a fact they gave is not about the topic in the air, so the store takes it as it\n    comes - the same rule a plan asked for and the line they said already have.\n\n    The key is read by `recall.ask`, which knows the shapes and not the store, so the fact has\n    to be one the store really holds: with nothing kept the line is left to the entries, which\n    can answer it as advice.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } })) } }) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&gen_memory::memory::recall::ask(&[text.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    said = respond(&[text.clone(), stdlib_call("copy", "deepcopy", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e))]);
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_is(&mcall(&g_NOT_A_FACT(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))]), &Py::None));
    Py::None
}
pub fn _their_own_line_ask(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True on the one ask the store must win even against a topic entry: their own line back.\n\n    \"what was my plan?\" is read as the topic the plan is about (\"what paint should i use for a\n    hallway?\" then \"what was my plan?\" was answered by the money entry), and the store's own plan\n    line is the answer.\n\n    \"what was i cooking?\" is read as a cooking line, and the cooking entry answers a different\n    question - a chilli recipe for a chilli they had already told us they were making, and the\n    recipe's timings for \"how long does that take\" after it. The person asked what they said,\n    not what to do. The store answers only from a line they really said, so this is said for\n    exactly the asks whose answer is that line, and every other line the store proposes on is\n    left to the arbiter's own rules.\n    ");
    let __try1 = exc::catch(|| {
    low = { let __b = gen_memory::memory::store_replies::_bare(&[text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&mcall(&gen_memory::memory::store_replies::g_PLAN_MINE(), "search", &[low.clone()])) {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = mcall(&{ let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { mcall(&{ let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plan")]) } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&gen_memory::memory::store_replies::g_ACTIVITY_ASK(), "search", &[low.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[gen_memory::memory::store_replies::_activity_line(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _with_repaired(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kind: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut entry: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut more: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The facts the understanding engine's own repairs make plain.\n\n    The store read the line as it was typed, so a word the engine had already repaired hid the\n    fact from it: \"im alergic to peanuts\" filed nothing at all, and \"im alergic to peanits\" filed\n    a food called \"peanits\". The repaired line is read as well, and a fact is taken from it only\n    where the typed line gave nothing for that key - so this can add what a typo hid, and cannot\n    overwrite what they actually said.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    plain = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("plain")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[plain.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&plain.clone(), "strip", &[]))) } }) {
    return found.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&plain.clone(), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])))) {
    return found.clone();
    }
    more = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); __kws.extend(vec![("question".to_string(), Py::Bool(py_eq(&kind.clone(), &py_str("question"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::facts::extract(args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    let __iter1 = py_iter_or_raise(&mcall(&more.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NOT_FROM_THE_REPAIR(), &key.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[key.clone()]))); if !py_truthy(&__b) { __b } else { value.clone() } } } }) {
    let _ = py_setitem(&found.clone(), &key.clone(), value.clone());
    }
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("others")]); if !py_truthy(&__b) { __b } else { mcall(&more.clone(), "get", &[py_str("others")]) } }) {
    let __iter3 = py_iter_or_raise(&geti(&more.clone(), &py_str("others")));
    for __item in __iter3 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&entry.clone(), "get", &[py_str("who")]))) } }) {
    continue;
    }
    known = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&found.clone(), &py_str("others"))) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &geti(&entry.clone(), &py_str("who")))) } }) {  __out.push(one.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&known.clone(), &Py::None))) {
    let _ = mcall(&geti(&found.clone(), &py_str("others")), "append", &[call_builtin("dict", &[entry.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    continue;
    }
    let __iter4 = py_iter_or_raise(&mcall(&entry.clone(), "items", &[]));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    key = __unpk5[0].clone();
    value = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&key.clone(), &py_str("who"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[key.clone()]))) } }) {
    let _ = py_setitem(&known.clone(), &key.clone(), value.clone());
    }
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_places_are_written_as_names(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut city: Py = Py::None;
    let mut known: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut swap: Py = Py::None;
    let _ = py_str("A place the store holds is written the way the store holds it.\n\n    A plan is kept and quoted in the words they typed, which is right for the plan and wrong for\n    the name in it: \"we r going 2 spain in may\" came back as \"You r going 2 spain in may.\", and\n    \"im going to lisbon in may\" as \"You're going to lisbon in may.\" The line that named the place\n    told the store the name (\"Spain\", \"Lisbon\"), so the reply is written with it. Only a place the\n    store is holding is touched, and a determiner in front of the word says it is a common noun,\n    not a place: \"You're planning a bath and an early night\" is not the city of Bath.\n    ");
    known = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } } } }) {
    return reply.clone();
    }
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[name.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&name.clone(), "strip", &[]) } }) {  __out.push(name.clone()); } } py_list(__out) })() };
    city = mcall(&known.clone(), "get", &[py_str("city")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[city.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&city.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&names.clone(), &city.clone())) } } } }) {
    let _ = mcall(&names.clone(), "append", &[city.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&names.clone()))) {
    return reply.clone();
    }
    out = reply.clone();
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    pattern = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![\\w'])"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![\\w'])")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
pub fn __nested_swap_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut city: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut known: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut names: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut out: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut pattern: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut swap: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(9).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut name: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "name").map(|(_, v)| v.clone())).unwrap_or_else(|| name.clone());
    let mut before: Py = Py::None;
    before = slice_or_raise(&gattr(&match_.clone(), "string"), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(28))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&mcall(&g_NOT_A_NAME_BEFORE(), "search", &[before.clone()])) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THE_PLACE_IS_NAMED(), "search", &[before.clone()])))) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    return name.clone();
    Py::None
}
    swap = {{ let city = city.clone(); let known = known.clone(); let memory = memory.clone(); let name = name.clone(); let names = names.clone(); let out = out.clone(); let pattern = pattern.clone(); let reply = reply.clone(); let swap = swap.clone(); py_func("swap", move |args: &[Py]| { let mut full = vec![city.clone(), known.clone(), memory.clone(), name.clone(), names.clone(), out.clone(), pattern.clone(), reply.clone(), swap.clone()]; full.extend_from_slice(args); __nested_swap_1(&full) }) }};
    out = mcall(&pattern.clone(), "sub", &[swap.clone(), out.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn note_the_hours(args: &[Py]) -> Py {
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
    let mut _pool: Py = Py::None;
    let mut family: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Keep the kind of week the line names, for the follow-up on the next turn.\n\n    The reading is bot/memory/ack.py's (`_hours_family`), so the six shapes are written down in\n    one place: a night shift, an early start, a late finish, long shifts, a rota that moves, and\n    working from home. Asked here rather than in the reply, because the hours are often answered\n    by a knowledge entry that never reaches the ack module, and the following line - \"does that\n    get easier\", \"i never know when to stop\" - is the same follow-up however it was answered.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    topic = mcall(&understood.clone(), "get", &[py_str("topic")]);
    if py_truthy(&{ let __b = topic.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&topic.clone(), &py_str("work"))) } }) {
    return Py::None;
    }
    }
    let __try1 = exc::catch(|| {
    let mut ack: Py = gen_memory::memory::ack::module_obj();
    let __unpk6 = py_unpack(&mcall(&ack.clone(), "_hours_family", &[mcall(&text.clone(), "lower", &[])]), 2, None);
    family = __unpk6[0].clone();
    _pool = __unpk6[1].clone();
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&family.clone()) {
    let _ = gen_memory::memory::hours::note(&[memory.clone(), family.clone()]);
    }
    Py::None
}
pub fn _a_detail_does_not_replace_the_job(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut held: Py = Py::None;
    let mut held_frame: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_frame: Py = Py::None;
    let mut said_weight: Py = Py::None;
    let _ = py_str("A line that only says when or where they work keeps the job they already gave.\n\n    \"im a nurse\" then \"i work nights\" put \"nights\" in the store as their job and dropped the\n    nurse, so \"and my job?\" answered \"You work nights\" and the one fact they had given was gone.\n    A shift is a detail of the work, and a detail does not take the place of the fact.\n\n    The same for where they work: \"im a teacher\" then \"i work in a school\" said back \"You work in\n    a school\" and the teacher was gone, so \"what do you know about me\" lost the role altogether.\n    A place is a detail beside the role, and the summary now says both.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("job")))) {
    return found.clone();
    }
    held = mcall(&{ let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("job")]);
    held_frame = call_builtin("str", &[{ let __b = mcall(&{ let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("job_frame")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&found.clone(), "get", &[py_str("job")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&held.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return found.clone();
    }
    said_frame = call_builtin("str", &[{ let __b = mcall(&found.clone(), "get", &[py_str("job_frame")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    said_weight = gen_memory::memory::facts::work_weight(&[said.clone(), said_frame.clone()]);
    if py_truthy(&op_lt(&said_weight.clone(), &gen_memory::memory::facts::work_weight(&[held.clone(), held_frame.clone()]))) {
    kept = call_builtin("dict", &[found.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = mcall(&kept.clone(), "pop", &[py_str("job"), Py::None]);
    let _ = mcall(&kept.clone(), "pop", &[py_str("job_frame"), Py::None]);
    if py_truthy(&Py::Bool(py_eq(&said_weight.clone(), &Py::Int(2)))) {
    let _ = py_setitem(&kept.clone(), &py_str("workplace"), mcall(&op_add(&op_add(&said_frame.clone(), &py_str(" ")), &call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))), "strip", &[]));
    } else {
    let _ = py_setitem(&kept.clone(), &py_str("shift"), said.clone());
    }
    return kept.clone();
    }
    if py_truthy(&{ let __b = gen_memory::memory::facts::is_a_place_of_work(&[held_frame.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&found.clone(), "get", &[py_str("job_frame")]), &Py::None)) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&{ let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("workplace")])))) {
    let _ = py_setitem(&found.clone(), &py_str("workplace"), mcall(&op_add(&op_add(&held_frame.clone(), &py_str(" ")), &call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))), "strip", &[]));
    }
    }
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("job_frame")))) {
    let _ = mcall(&{ let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "pop", &[py_str("job_frame"), Py::None]);
    }
    return found.clone();
    Py::None
}
pub fn _without_the_shrug_at_the_end(args: &[Py]) -> Py {
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
    let _ = py_str("The line without the chat-speak hung on the end of it, when that leaves words behind.\n\n    `bot/memory/filler` holds the shrugs people put on the end of a line, and everything the store\n    says back reads a line through it. What the store *files* did not, so the shrug became part of\n    the thing told: \"how old is my son by the way\" reached no reader at all and was answered with\n    the search hedge, and \"when do we go to cornwall by the way\" filed a place called \"cornwall\n    bies the way\" and said it back: \"You go to cornwall bies the way in August.\"\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    words = mcall(&mcall(&mcall(&said.clone(), "lower", &[]), "strip", &[py_str(" .,!;:")]), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_OPENERS(), &geti(&words.clone(), &Py::Int(0)))) } })) } }) {
    return said.clone();
    }
    clean = gen_memory::memory::filler::without_the_end_shrug(&[said.clone()]);
    return if py_truthy(&{ let __b = clean.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&clean.clone(), &said.clone())) } }) { clean.clone() } else { said.clone() };
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut hurt_before: Py = Py::None;
    let _ = py_str("Remember what the line says, and note a second animal of a kind as the turn's news.\n\n    Everything below writes what the line says. This wrapper is here because the one thing the\n    store does with the result is not a fact but the turn: a line that leaves the store holding\n    two cats called Milo and Max is a line about the two of them, and the reply should say so.\n    The wording is not read - \"no my other cat is called max\", \"no we also have a cat called max\"\n    and \"i also have a cat called tom\" all file the second cat, and all three were left to the\n    greeting, which answered with the person's own line and \"Got it\" in front of it. A rule per\n    phrasing would have covered two of the three; the store's own record covers every phrasing.\n    ");
    before = _the_pets_on_record(&[memory.clone()]);
    let _ = py_setitem(&memory.clone(), &py_str("_pets_before"), before.clone());
    hurt_before = gen_memory::memory::whathurts::before(&[memory.clone()]);
    let __try1 = exc::catch(|| {
    let _ = _observe(&[text.clone(), memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    let _ = mcall(&memory.clone(), "pop", &[py_str("_pets_before"), Py::None]);
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let _ = gen_memory::memory::whathurts::after(&[memory.clone(), hurt_before.clone()]);
    let _ = _note_a_pet_that_is_new(&[text.clone(), memory.clone(), before.clone()]);
    let __try6 = exc::catch(|| {
    let _ = gen_memory::memory::store_replies::_the_limit_they_share(&[text.clone(), memory.clone()]);
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
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn after(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("Keep a correction the chat was given, whichever module gave the reply to it.\n\n    \"how long does a chicken take to roast\" / \"i meant a turkey\" reads right on the turn that says\n    it - the turkey row answers the correction by itself - and the chat was still on the chicken:\n    asked \"what about a duck\" the next line reached no row at all and got a search hedge. Whether\n    the bookkeeping happened used to depend on which module won the turn, because each module\n    replies to its own copy of the memory and only the winner's copy is kept. The correction is\n    the chat's, not the reply's, so it happens here, once, after the reply has been chosen.\n    ");
    let __try1 = exc::catch(|| {
    let _ = gen_memory::memory::repair::keep_the_correction(&[text.clone(), memory.clone()]);
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
    Py::None
}
pub fn _the_ages_of_the_children(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The reply for the ages of the children the store counts, given one at a time, or \"\".\n\n    \"i have two kids\" then \"ones 7 and ones 4\": the second line names no group word, so every\n    reader for a group passes it on, the store kept nothing, and the composer handed the line back\n    (\"Got it - ones 7 and ones 4.\"). The ages are the answer to \"how old are they?\", so they are\n    kept and said as ages. A bare list of figures is read only when the line before counted the\n    children, because \"7 and 4\" on its own could be anything.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    held = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("kids")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_lt(&held.clone(), &Py::Int(2)) } }) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(op_gt(&call_builtin("len", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("ages")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    ages = gen_memory::memory::facts::ages_said_one_by_one(&[text.clone(), held.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&ages.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = gen_memory::memory::facts::the_ones_are_given_one_at_a_time(&[text.clone()]); if py_truthy(&__b) { __b } else { _the_count_was_the_line_before(&[memory.clone()]) } }))) {
    return py_str("");
    }
    let _ = py_call(&gen_memory::memory::store_replies::g_remember_other(), &[memory.clone(), py_dict(vec![(py_str("who"), py_str("kids")), (py_str("ages"), ages.clone())]), text.clone()]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ages.clone()) { let mut age: Py = __item.clone(); __out.push(mcall(&g_AGES_IN_WORDS(), "get", &[age.clone(), call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    said = if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&words.clone(), &op_neg(&Py::Int(1)))) } else { mcall(&py_str(" and "), "join", &[words.clone()]) };
    return py_call(&gen_memory::memory::store_replies::g_pick(), &[py_list(vec![mcall(&py_str("{} — got it."), "format", &[op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None))]), mcall(&py_str("Got it — {}."), "format", &[said.clone()]), mcall(&py_str("{}."), "format", &[op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None))])]), memory.clone()]);
    Py::None
}
pub fn _the_count_was_the_line_before(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line before this one counted the children, so a bare list of ages is theirs.");
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:kids?|children|child)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_pets_on_record(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("The (animal, name) pairs the store holds, for comparing one turn with the next.");
    held = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return held.clone();
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = mcall(&held.clone(), "add", &[py_tuple(vec![mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])])]);
    }
    return held.clone();
    Py::None
}
pub fn _note_a_pet_that_is_new(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut added: Py = Py::None;
    let mut animal: Py = Py::None;
    let mut held: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut same: Py = Py::None;
    let _ = py_str("A line that leaves two of an animal on record is answered as a line about the two.\n\n    Nothing is read out of the words: the store is compared with itself, so a phrasing nobody\n    wrote a rule for is answered the same as the ones people type most. A first pet is left\n    alone - bot/memory/ack.py already answers the line that gives one (\"A cat called Milo.\n    What's it like to live with?\") - and so is a rename, which leaves one animal behind.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    held = mcall(&memory.clone(), "get", &[py_str("correction")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&held.clone(), "get", &[py_str("said")]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))) } }) {
    return Py::None;
    }
    added = op_sub(&_the_pets_on_record(&[memory.clone()]), &before.clone());
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("sorted", &[added.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    animal = __unpk2[0].clone();
    name = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&animal.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&name.clone())) } }) {
    continue;
    }
    same = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &animal.clone())); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("name")]) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[same.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    continue;
    }
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pets_more")), (py_str("value"), name.clone()), (py_str("animal"), animal.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), (py_str("text"), text.clone())]));
    return geti(&memory.clone(), &py_str("correction"));
    }
    return Py::None;
    Py::None
}
pub fn _observe(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut answered: Py = Py::None;
    let mut budget: Py = Py::None;
    let mut correction: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut folded: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut held_name: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut known: Py = Py::None;
    let mut more: Py = Py::None;
    let mut named_here: Py = Py::None;
    let mut one: Py = Py::None;
    let mut planned: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut retracted: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut value: Py = Py::None;
    let mut was: Py = Py::None;
    let mut who: Py = Py::None;
    let mut ruled_out_place: Py = gen_memory::memory::not_this::module_obj();
    if py_truthy(&mcall(&ruled_out_place.clone(), "the_place_ruled_out", &[text.clone(), memory.clone()])) {
    return Py::None;
    }
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    let _ = note_the_hours(&[text.clone(), memory.clone()]);
    let _ = gen_memory::memory::counts::observe(&[text.clone(), memory.clone()]);
    let _ = gen_memory::memory::surname::observe(&[text.clone(), memory.clone()]);
    planned = if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { gen_memory::memory::plans::answer(&[text.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&planned.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("plan_answer"), py_dict(vec![(py_str("text"), planned.clone()), (py_str("user"), text.clone())]));
    }
    kind = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("kind")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = gen_memory::memory::store_replies::_behind_a_remembering_frame(&[text.clone()]);
    if py_truthy(&said.clone()) {
    text = said.clone();
    }
    if py_truthy(&{ let __b = gen_memory::memory::recall::ask(&[text.clone()]); if py_truthy(&__b) { __b } else { _reads_the_store(&[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_REMIND_QUESTION(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&_asks_about_someone_else(&[text.clone()])) {
    rest = gen_memory::memory::store_replies::_after_the_yes_no_frame(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    text = rest.clone();
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { gen_memory::memory::store_replies::_statement(&[text.clone()]) } }) {
    return Py::None;
    }
    let _ = gen_memory::memory::decided::observe(&[text.clone(), memory.clone()]);
    let _ = gen_memory::memory::company::observe(&[text.clone(), memory.clone()]);
    let _ = mcall(&memory.clone(), "pop", &[py_str("pet_age"), Py::None]);
    let _ = gen_memory::memory::times::keep(&[text.clone(), memory.clone()]);
    let _ = gen_memory::memory::rates::keep(&[text.clone(), memory.clone()]);
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("question".to_string(), Py::Bool(py_eq(&kind.clone(), &py_str("question"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::facts::extract(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = gen_memory::memory::facts::a_correction_without_a_subject(&[text.clone(), py_call(&gen_memory::memory::store_replies::g_last_user(), &[memory.clone()])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("name")])))) {
    answered = _their_name_answered(&[text.clone(), memory.clone()]);
    if py_truthy(&answered.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("name"), answered.clone());
    }
    }
    found = _with_repaired(&[found.clone(), text.clone(), memory.clone(), kind.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("kids")])))) {
    counted = gen_memory::memory::facts::count_of_the_people(&[text.clone(), memory.clone()]);
    if py_truthy(&counted.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("kids"), counted.clone());
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("budget")])))) {
    folded = py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = folded.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&folded.clone(), &mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]))) } }) {
    budget = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(folded.clone()); __kws.extend(vec![("question".to_string(), Py::Bool(py_eq(&kind.clone(), &py_str("question"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::facts::extract(args))(&__args) }, "get", &[py_str("budget")]);
    if py_truthy(&budget.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("budget"), budget.clone());
    }
    }
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("budget")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[\\u00a3$\\u20ac]"), geti(&found.clone(), &py_str("budget"))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    budget = call_builtin("str", &[geti(&found.clone(), &py_str("budget"))]).unwrap_or_else(|e| exc::raise(e));
    sign = _budget_sign(&[text.clone(), budget.clone()]);
    if py_truthy(&sign.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("budget"), op_add(&sign.clone(), &mcall(&budget.clone(), "lstrip", &[])));
    }
    }
    correction = mcall(&found.clone(), "pop", &[py_str("correction"), Py::None]);
    if py_truthy(&Py::Bool(py_is(&correction.clone(), &Py::None))) {
    correction = _a_correction_without_a_marker(&[text.clone(), memory.clone()]);
    }
    dropped = mcall(&found.clone(), "pop", &[py_str("drop"), Py::None]);
    retracted = mcall(&found.clone(), "pop", &[py_str("retract"), Py::None]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("allergies")])))) {
    more = gen_memory::memory::facts::one_more(&[text.clone()]);
    held = { let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = more.clone(); if !py_truthy(&__b) { __b } else { held.clone() } }) {
    let _ = py_setitem(&found.clone(), &py_str("allergies"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&more.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&held.clone(), &item.clone()))) {  __out.push(item.clone()); } } py_list(__out) })() });
    }
    }
    named_here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut who: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[who.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(who.clone()); } } py_list(__out) })() };
    let _ = _remember_dislikes(&[text.clone(), memory.clone(), named_here.clone()]);
    second = _a_second_persons_limit(&[text.clone(), memory.clone()]);
    if py_truthy(&second.clone()) {
    let _ = mcall(&mcall(&found.clone(), "setdefault", &[py_str("others"), py_list(vec![])]), "append", &[second.clone()]);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&found.clone(), "pop", &[py_str("others"), py_list(vec![])]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    pronoun = mcall(&entry.clone(), "pop", &[py_str("pronoun"), Py::None]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&entry.clone(), "get", &[py_str("who")])))) {
    if py_truthy(&{ let __b = pronoun.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("she"), py_str("he")]), &pronoun.clone())) } }) {
    age = gen_memory::memory::store_replies::_a_number(&[mcall(&entry.clone(), "get", &[py_str("age")])]);
    if py_truthy(&age.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("their_age"), py_dict(vec![(py_str("age"), age.clone()), (py_str("turns"), { let __b = mcall(&entry.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } })]));
    }
    continue;
    }
    who = py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone(), named_here.clone()]);
    if py_truthy(&pronoun.clone()) {
    fits = if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("she")))) { gen_memory::memory::facts::g_SHE() } else { gen_memory::memory::facts::g_HE() };
    who = if py_truthy(&gen_memory::memory::relations::_pronoun_fits(&[who.clone(), fits.clone()])) { who.clone() } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("she"), py_str("he")]), &pronoun.clone())); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("months")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&entry.clone(), &py_str("pronoun"), pronoun.clone());
    let _ = py_call(&gen_memory::memory::store_replies::g_remember_other(), &[memory.clone(), entry.clone(), text.clone()]);
    }
    continue;
    }
    let _ = py_setitem(&entry.clone(), &py_str("who"), who.clone());
    }
    if py_truthy(&{ let __b = pronoun.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&entry.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { mcall(&g_CORRECTING(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    known = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &geti(&entry.clone(), &py_str("who"))))) {  __out.push(one.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = known.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("age"))) } }))) {
    continue;
    }
    }
    was = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &mcall(&entry.clone(), "get", &[py_str("who")])))) {  __out.push(call_builtin("dict", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_call(&gen_memory::memory::store_replies::g_remember_other(), &[memory.clone(), entry.clone(), text.clone()]);
    let _ = _note_a_corrected_fact(&[memory.clone(), text.clone(), entry.clone(), was.clone()]);
    let _ = _note_a_fact_moved(&[memory.clone(), text.clone(), entry.clone(), was.clone()]);
    }
    found = _a_detail_does_not_replace_the_job(&[found.clone(), memory.clone()]);
    held_name = call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&mcall(&found.clone(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    key = __unpk3[0].clone();
    value = __unpk3[1].clone();
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), key.clone(), value.clone(), text.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { { let __b = held_name.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&call_builtin("str", &[geti(&found.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&held_name.clone(), "lower", &[]))) } } } }) {
    let _ = py_setitem(&memory.clone(), &py_str("name_fixed"), py_dict(vec![(py_str("name"), call_builtin("str", &[geti(&found.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))), (py_str("from"), held_name.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("job")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("job_frame")]))) } }) {
    let _ = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "pop", &[py_str("job_frame"), Py::None]);
    }
    let _ = _fix_a_pets_name(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("pets")))) {
    let _ = _name_the_last_pet(&[text.clone(), memory.clone()]);
    let _ = _age_the_named_pet(&[text.clone(), memory.clone()]);
    let _ = _age_they_answered(&[text.clone(), memory.clone()]);
    let _ = _age_said_of_the_only_animal(&[text.clone(), memory.clone()]);
    let _ = _fix_a_pets_age(&[text.clone(), memory.clone()]);
    }
    let _ = _fix_a_persons_age(&[text.clone(), memory.clone()]);
    let _ = _fix_the_weeks_of_a_pregnancy(&[text.clone(), memory.clone()]);
    fixed = _fix_a_persons_name(&[text.clone(), memory.clone()]);
    if py_truthy(&fixed.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("name_fixed"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fixed.clone()); __kws.extend(vec![("said".to_string(), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("owns")))) {
    let _ = _fix_the_make_of_a_thing(&[text.clone(), memory.clone()]);
    }
    let _ = _the_name_they_gave_to_the_person_they_mean(&[text.clone(), memory.clone()]);
    let _ = _a_trip_booked_named(&[text.clone(), memory.clone()]);
    if py_truthy(&correction.clone()) {
    let _ = gen_memory::memory::corrected::_apply_correction(&[memory.clone(), correction.clone(), text.clone()]);
    let _ = _fix_a_pets_name(&[text.clone(), memory.clone()]);
    return Py::None;
    }
    if py_truthy(&{ let __b = dropped.clone(); if !py_truthy(&__b) { __b } else { _drop_value(&[memory.clone(), dropped.clone(), text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = retracted.clone(); if !py_truthy(&__b) { __b } else { _retract_last(&[memory.clone(), text.clone()]) } }) {
    return Py::None;
    }
    let _ = _forget_denied(&[text.clone(), memory.clone()]);
    let _ = _the_back_on_the_plate(&[text.clone(), memory.clone()]);
    if py_truthy(&gen_memory::memory::store_replies::_about_this_chat(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&gen_memory::memory::told::a_callback_the_chat_has(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_this_line_put_something_right(&[memory.clone(), text.clone()])))) {
    let _ = py_call(&gen_memory::memory::store_replies::g_note(), &[memory.clone(), text.clone(), kind.clone()]);
    }
    Py::None
}
pub fn _a_trip_booked_named(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("A trip that is already booked is a trip the chat is on, and the store keeps it.\n\n    \"we booked a cottage in cornwall for august\" was answered about Cornwall and then kept\n    nothing: \"where are we going\" said \"You haven't said - where are you going?\", and a correction\n    after it had no plan to land on. The place is filed, and the plan in the shape the other trips\n    are kept in (\"Cornwall in August\"), unless the store is already on that plan.\n    ");
    let mut trips: Py = gen_memory::memory::trips::module_obj();
    let __unpk1 = py_unpack(&mcall(&trips.clone(), "the_trip_they_booked", &[text.clone()]), 2, None);
    place = __unpk1[0].clone();
    when = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::None;
    }
    known = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    place = op_add(&mcall(&slice_or_raise(&place.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&place.clone(), &Py::Int(1), &Py::None, &Py::None));
    plan = if py_truthy(&when.clone()) { mcall(&py_str(" "), "join", &[mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&place.clone(), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&when.clone(), -1, "")); py_string(__s) }, "split", &[])]) } else { place.clone() };
    places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "lower", &[])); } py_set(__out) })() }, &mcall(&place.clone(), "lower", &[])))) {
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("places"), place.clone(), text.clone()]);
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &mcall(&plan.clone(), "lower", &[])))) {
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("plan"), plan.clone(), text.clone()]);
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("plans"), plan.clone(), text.clone()]);
    }
    Py::None
}
pub fn _this_line_put_something_right(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether this very line is the correction one of the readers has just applied.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("correction"), py_str("name_fixed")]));
    for __item in __iter1 {
    key = __item.clone();
    kept = mcall(&memory.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&kept.clone(), "get", &[py_str("said")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _on_record_before(args: &[Py]) -> Py {
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
    let mut records: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "records") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: records"))) };
    let mut kept: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The animals an earlier line put on record, out of the ones this line has just taken back.\n\n    A take-back only says itself when there was something to take back. \"i dont have a rabbit\"\n    from someone who has a cat is a thing they never had, and \"Right — no rabbit, then.\"\n    answers a line they did not say; the ordinary answer to it is the one they get.\n    ");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() }]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(py_call(&gen_memory::memory::store_replies::g_stem(), &[word.clone()])); } py_set(__out) })() };
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = records.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![mcall(&record.clone(), "get", &[py_str("animal")]), mcall(&record.clone(), "get", &[py_str("name")])]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_call(&gen_memory::memory::store_replies::g_stem(), &[mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]))) } }) {
    let _ = mcall(&kept.clone(), "append", &[record.clone()]);
    break;
    }
    }
    }
    return kept.clone();
    Py::None
}
pub fn _forget_denied(args: &[Py]) -> Py {
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
    let mut denied: Py = Py::None;
    let mut died: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut given: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A line that says they do not have a fact takes the old one out of the store.\n\n    \"im coeliac\" and then \"im not coeliac\", \"im allergic to peanuts and nuts\" and then \"im not\n    allergic to nuts\": the same fact being taken back. A store that keeps it goes on warning\n    about, or cooking around, the thing the person has just said they are fine with.\n    ");
    held = { let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    denied = gen_memory::memory::facts::denials(&[text.clone(), held.clone()]);
    theirs = if py_truthy(&call_builtin("isinstance", &[denied.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&denied.clone(), "pop", &[py_str("theirs"), Py::None]) } else { Py::None };
    died = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_losses_taking(&[text.clone(), held.clone(), memory.clone()])) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&denied.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(gen_memory::memory::store_replies::_kind(&[item.clone()])); } py_list(__out) })() }, &call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    given = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gen_memory::memory::facts::given_away_one(&[text.clone(), held.clone()])) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&denied.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[item.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(item.clone()); } } py_list(__out) })() }, &pet.clone()))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&denied.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&theirs.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&died.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&given.clone())) } } } } } }) {
    return Py::None;
    }
    gone = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&denied.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("diet")))) {
    value = _the_diet_they_are_back_on(&[memory.clone(), value.clone()]);
    }
    dropped = py_call(&gen_memory::memory::store_replies::g_forget(), &[memory.clone(), key.clone(), value.clone(), py_str("gone")]);
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pets")))) {
    dropped = _on_record_before(&[memory.clone(), dropped.clone()]);
    }
    if py_truthy(&dropped.clone()) {
    let _ = py_setitem(&gone.clone(), &key.clone(), dropped.clone());
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("pets"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }, &py_str("*"))) } }) {
    let _ = py_setitem(&gone.clone(), &py_str("pets_all"), Py::Bool(true));
    }
    }
    }
    if py_truthy(&{ let __b = died.clone(); if !py_truthy(&__b) { __b } else { py_call(&gen_memory::memory::store_replies::g_forget(), &[memory.clone(), py_str("pets"), died.clone(), py_str("died")]) } }) {
    let _ = py_setitem(&gone.clone(), &py_str("pets_died"), Py::Bool(true));
    }
    if py_truthy(&{ let __b = given.clone(); if !py_truthy(&__b) { __b } else { py_call(&gen_memory::memory::store_replies::g_forget(), &[memory.clone(), py_str("pets"), given.clone(), py_str("given away")]) } }) {
    let _ = py_setitem(&gone.clone(), &py_str("pets_given"), Py::Bool(true));
    }
    let __iter3 = py_iter_or_raise(&{ let __b = theirs.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter3 {
    entry = __item.clone();
    taken = _off_their_list(&[memory.clone(), entry.clone(), text.clone()]);
    if py_truthy(&taken.clone()) {
    let _ = mcall(&gone.clone(), "update", &[taken.clone()]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&gone.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&gone.clone(), &py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]));
    let _ = py_setitem(&memory.clone(), &py_str("denial"), gone.clone());
    return gone.clone();
    Py::None
}
pub fn _the_back_on_the_plate(args: &[Py]) -> Py {
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
    let mut food: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut likes: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("A line that puts back what somebody else does not eat takes it off their record.\n\n    \"my son doesnt eat fish\" then \"actually he eats fish\": the store kept \"no fish\" against him\n    and went on answering \"what does my son eat\" with \"Your son doesn't eat fish.\", which is the\n    opposite of the last thing the person said. A taste is the same shape - \"my wife hates\n    mushrooms\" then \"actually she loves them\" puts mushrooms on her likes and off her dislikes.\n    Only what is already on that person's record is lifted, so a line that describes somebody for\n    the first time (\"my son eats fish\") is left to the modules that read it on its own.\n    ");
    let __iter1 = py_iter_or_raise(&gen_memory::memory::facts::their_back(&[text.clone()]));
    for __item in __iter1 {
    said = __item.clone();
    who = { let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if py_truthy(&__b) { __b } else { py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter2 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&who.clone(), "lower", &[])))) {
    continue;
    }
    kind = mcall(&said.clone(), "get", &[py_str("kind")]);
    food = mcall(&call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("food")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("them"), py_str("it"), py_str("that")]), &mcall(&food.clone(), "lower", &[])))) {
    food = _the_one_they_are_back_on(&[record.clone(), kind.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&food.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("diet")))) {
    held = mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![op_add(&py_str("no "), &mcall(&food.clone(), "lower", &[])), op_add(&op_add(&py_str("no "), &mcall(&food.clone(), "lower", &[])), &py_str("s"))]), &held.clone()))) {
    continue;
    }
    let _ = py_setitem(&record.clone(), &py_str("diet"), op_add(&py_str("eats "), &food.clone()));
    } else {
    gone = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("dislikes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gone.clone()) { let mut item: Py = __item.clone(); __out.push(mcall(&item.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&food.clone(), "lower", &[])))) {
    continue;
    }
    let _ = py_setitem(&record.clone(), &py_str("dislikes"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gone.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&item.clone(), "lower", &[]), &mcall(&food.clone(), "lower", &[])))) {  __out.push(item.clone()); } } py_list(__out) })() });
    likes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("likes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&likes.clone()) { let mut item: Py = __item.clone(); __out.push(mcall(&item.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&food.clone(), "lower", &[])))) {
    let _ = mcall(&likes.clone(), "append", &[food.clone()]);
    }
    let _ = py_setitem(&record.clone(), &py_str("likes"), likes.clone());
    }
    let _ = py_setitem(&memory.clone(), &py_str("reversal"), py_dict(vec![(py_str("who"), who.clone()), (py_str("food"), food.clone()), (py_str("kind"), kind.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_one_they_are_back_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut record: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "record") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: record"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut gone: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let _ = py_str("The one thing on a person's record a \"them\" can point at, or \"\" when there are two.\n\n    \"my wife hates mushrooms\" then \"actually she loves them\" is mushrooms because they are the\n    only thing she does not like; with mushrooms and olives both on the list the pointing word\n    says nothing, and guessing one of them would put a taste in her mouth she never gave.\n    ");
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("diet")))) {
    held = mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return if py_truthy(&mcall(&mcall(&held.clone(), "lower", &[]), "startswith", &[py_str("no ")])) { mcall(&slice_or_raise(&held.clone(), &Py::Int(3), &Py::None, &Py::None), "strip", &[]) } else { py_str("") };
    }
    gone = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("dislikes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[gone.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&gone.clone(), &Py::Int(0)) } else { py_str("") };
    Py::None
}
pub fn _off_their_list(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut items: Py = Py::None;
    let mut left: Py = Py::None;
    let mut one: Py = Py::None;
    let mut record: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The take-back a line makes about somebody else, taken off the record kept for them.\n\n    \"my son is allergic to nuts\" then \"he isnt allergic to nuts\": the store files facts about\n    other people under their own entry, so the denial has to find that entry and name the person\n    it belongs to when it says the note back - \"Nuts back on the table, then.\" reads as the\n    speaker's own allergy, which they never mentioned.\n\n    The note is said even when they never had it on record: \"my daughter is 6\" then \"she isnt\n    allergic to eggs\" is a thing the person said about her, and the fallback's \"I'm listening\"\n    is no answer to it.\n    ");
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); if py_truthy(&call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&items.clone()))) {
    return Py::None;
    }
    who = { let __b = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    taken = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &who.clone()))) {
    continue;
    }
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "lower", &[])); } py_set(__out) })() }, &mcall(&item.clone(), "lower", &[])))) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = mcall(&taken.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&left.clone(), &item.clone()))) {  __out.push(item.clone()); } } py_list(__out) })() }]);
    let _ = py_setitem(&record.clone(), &py_str("allergies"), left.clone());
    }
    return py_dict(vec![(py_str("allergies"), items.clone()), (py_str("who"), who.clone())]);
    Py::None
}
pub fn _the_diet_they_are_back_on(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut held: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The diet as the store holds it, when the take-back names only the food.\n\n    \"im cutting out sugar\" is kept as that whole phrase, and the line that takes it back can\n    name nothing but the food - \"im back on sugar\", \"im eating bread again\". Matched against the\n    phrase word by word, or `forget` finds nothing and the store goes on telling them to avoid\n    what they eat now.\n    ");
    held = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("diet")]);
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return value.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[mcall(&mcall(&held.clone(), "lower", &[]), "split", &[])]), &said.clone()))) {
    return held.clone();
    }
    return value.clone();
    Py::None
}
pub fn _the_last_line(args: &[Py]) -> Py {
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
    let _ = py_str("The line just before this one, so a bare \"he died\" can point back at what it named.\n\n    \"i have a dog\" then \"he died\" is the dog; \"my grandad is in hospital\" then \"he died\" is the\n    grandad. Without the turn behind it a \"he\" says too little, and taking the only animal off the\n    store on that is the worst thing this reader could do.\n    ");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &op_neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _losses_taking(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut before: Py = Py::None;
    let mut everything: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut same: Py = Py::None;
    let mut taken: Py = Py::None;
    let _ = py_str("The animals a line about a death takes off the store, one record at a time.\n\n    \"my cat died\" from someone with two cats is one of them: the store keeps the other instead of\n    answering \"you haven't told me about a pet yet\" a turn after they grieved one. \"both cats\n    died\" is the whole lot.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    everything = call_builtin("bool", &[mcall(&gen_memory::memory::facts::g_LOSS_ALL(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    taken = py_list(vec![]);
    before = _the_last_line(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&gen_memory::memory::facts::losses(&[text.clone(), held.clone(), before.clone()]));
    for __item in __iter1 {
    kind = __item.clone();
    same = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&py_call(&gen_memory::memory::store_replies::g__animal_family(), &[mcall(&pet.clone(), "get", &[py_str("animal")])]), &py_call(&gen_memory::memory::store_replies::g__animal_family(), &[kind.clone()]))) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&same.clone()))) {
    continue;
    }
    if py_truthy(&everything.clone()) {
    let _ = mcall(&taken.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&same.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&taken.clone(), &pet.clone()))) {  __out.push(pet.clone()); } } py_list(__out) })() }]);
    continue;
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&same.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    let _ = mcall(&taken.clone(), "append", &[if py_truthy(&named.clone()) { geti(&named.clone(), &op_neg(&Py::Int(1))) } else { geti(&same.clone(), &op_neg(&Py::Int(1))) }]);
    }
    return taken.clone();
    Py::None
}
pub fn _last_stated(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The facts the line before this one put in the store.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = py_call(&gen_memory::memory::store_replies::g_last_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_list(vec![]);
    }
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&gen_memory::memory::log(&[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &said.clone()))) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[entry.clone()]);
    }
    return kept.clone();
    Py::None
}
pub fn _retract_last(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut animal: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A line that takes back the last thing they said without naming it: \"take that back\", \"scratch\n    that\".\n\n    \"im allergic to peanuts and shellfish\" then \"take that back\" left the allergy on record and the\n    next ask answered \"You told me you're allergic to peanuts, shellfish\" - a note they had just\n    asked to take off. A retraction names no value, so the only thing it can be about is the line\n    before it: every fact that line put in the store goes. A taste or a fact about someone else is\n    left alone - those are not what \"take that back\" means, and losing them silently would be worse\n    than keeping them.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    gone = __unpk1[0].clone();
    shown = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&_last_stated(&[memory.clone()]));
    for __item in __iter2 {
    entry = __item.clone();
    key = mcall(&entry.clone(), "get", &[py_str("key")]);
    value = mcall(&entry.clone(), "get", &[py_str("value")]);
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pet")))) {
    known = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]);
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_is(&pet.clone(), &value.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&pet.clone(), &value.clone())) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&pets.clone()) {
    let _ = py_setitem(&known.clone(), &py_str("pets"), pets.clone());
    } else {
    let _ = mcall(&known.clone(), "pop", &[py_str("pets"), Py::None]);
    }
    let _ = mcall(&gone.clone(), "append", &[py_str("pets")]);
    animal = { let __b = mcall(&{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("pet") } } } };
    let _ = mcall(&shown.clone(), "append", &[call_builtin("str", &[animal.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plans")))) {
    known = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]);
    if py_truthy(&py_call(&gen_memory::memory::store_replies::g_forget(), &[memory.clone(), py_str("plans"), py_list(vec![value.clone()])])) {
    let _ = mcall(&gone.clone(), "append", &[py_str("plans")]);
    let _ = mcall(&shown.clone(), "append", &[call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("plan")]), &value.clone()))) {
    let _ = mcall(&known.clone(), "pop", &[py_str("plan"), Py::None]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&gen_memory::memory::store_lines::g_NOT_RETRACTED(), &key.clone()))) {
    continue;
    }
    dropped = py_call(&gen_memory::memory::store_replies::g_forget(), &[memory.clone(), key.clone(), value.clone()]);
    if py_truthy(&dropped.clone()) {
    let _ = mcall(&gone.clone(), "append", &[key.clone()]);
    let _ = mcall(&shown.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dropped.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[item.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&gone.clone()))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gone.clone()) { let mut key: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("plans"), py_str("places")]), &key.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), &py_str("plan_dropped"), call_builtin("str", &[geti(&shown.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)));
    }
    let _ = py_setitem(&memory.clone(), &py_str("retraction"), py_dict(vec![(py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), (py_str("keys"), gone.clone()), (py_str("shown"), shown.clone())]));
    return geti(&memory.clone(), &py_str("retraction"));
    Py::None
}
pub fn _reads_the_store(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("True for a line that only asks for something already kept, so it states nothing new.\n\n    `facts` extracts what a line mentions, and an ask mentions the very thing it wants back:\n    \"i'm allergic to nuts\" then \"am i allergic to peanuts or shellfish?\" used to keep both and\n    answer *you're allergic to nuts, peanuts or shellfish* - a claim they never made.\n\n    An ask about a person's allergy or a food they cannot have is the same kind of line: it\n    names nothing of its own and asks for a fact the store keeps (\"my son is allergic to nuts\"\n    then \"what is he allergic to\" was answered with what an allergy is, by a topic entry that\n    matched the word). Said with nobody the store knows, it claims nothing.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = gen_memory::memory::store_replies::_bare(&[py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]);
    if py_truthy(&{ let __b = mcall(&g_NAMED_FACT_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_TOPIC_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_ASK_WITHOUT_MY(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::modules::memory_tables::g_HOW_MANY_OF_MINE(), "search", &[low.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&gen_memory::memory::store_replies::g_OTHER_FACT(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_CANT_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_CAN_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_CAN_THEY_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { gen_memory::memory::store_replies::_a_bare_limit_ask(&[low.clone(), memory.clone()]) } } } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&gen_memory::memory::store_replies::_asks_about_someone(&[low.clone(), text.clone(), memory.clone()])))) {
    return Py::Bool(false);
    }
    whole = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = mcall(&gen_memory::memory::store_replies::g_STATES_A_LIMIT(), "search", &[whole.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&gen_memory::memory::store_replies::g_A_HAVE_QUESTION(), "search", &[whole.clone()]))) } }) {
    return Py::Bool(false);
    }
    said = gen_memory::memory::store_replies::_other_person_reply(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, low.clone()]);
    return call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_plan_they_took_back(args: &[Py]) -> Py {
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
    let mut gone: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("A where ask about a trip they have just taken back is the store's line, not a repeat.\n\n    \"im going to lisbon in may\" / \"take it back\" / \"where am i going\" was answered \"Same as\n    before: In Lisbon: Alfama on foot and the 28 tram...\" - the trip's paragraph read back to\n    someone the bot had just told \"Right, that place is off the record, then.\" The ask names no\n    subject of its own, so the knowledge base leaned on the topic record for the line they had\n    just retracted and the answer on the screen came back. The store is the one part that knows\n    the plan went: its own answer to a where ask names what was taken back (\"Lisbon is off, then\n    - where are you going?\"). Only a where ask, and only while the thing they took back is the\n    newest thing in the store, so every other line after a take-back is read exactly as before.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    gone = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("plan_dropped")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[gone.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&gone.clone(), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[mcall(&gen_memory::memory::store_replies::g_WHERE_PLAN(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_clock_in_their_line(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the ask wants a clock or a day the person gave, and the store can place it.\n\n    \"what time is my appointment\" after \"i have a dentist appointment on friday at 3\" was\n    answered \"Ring practices near you and ask straight out whether they take new NHS patients\":\n    the store had the hour and proposed it, but the claim that hands the store its own asks was\n    gated on the word \"when\", so a clock ask fell to the knowledge base's entry about the same\n    noun. The hour or the day is in what the person said, so the ask is the store's - and every\n    such ask it cannot place (no plan naming it, nothing to read the hour off) is left alone,\n    which keeps \"what time do i start work\" with whoever can answer it.\n    ");
    let __try1 = exc::catch(|| {
    low = mcall(&text.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&gen_memory::memory::store_replies::g_TIME_ASKS(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_TIME_THING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_TIME_IT(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_DAY_THING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&gen_memory::memory::store_replies::g_DAY_IT(), "search", &[low.clone()]) } } } } } } } }))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[gen_memory::memory::store_replies::_plans_reply(&[memory.clone(), low.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _an_ask_about_someone_else(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut known: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("True when the line asks about a fact the store keeps for someone else.\n\n    \"i'm cooking for my mother in law on sunday\" then \"shes vegetarian\" then \"is she vegetarian\"\n    is an ask about the mother in law, and the diet is on record; the knowledge base reads the\n    word \"vegetarian\" and offers a curry instead, so the store has to say the line is its own\n    (`TAKES_AS_IT_COMES` below). Only a line that names or points at a person whose fact is on\n    record is claimed: with nothing stored for them the ask goes to whoever can answer it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASK_ABOUT_THEM(), "search", &[mcall(&text.clone(), "strip", &[])])))) {
    return Py::Bool(false);
    }
    keys = py_list(vec![]);
    if py_truthy(&mcall(&gen_memory::memory::store_replies::g_OTHER_FACT(), "search", &[text.clone()])) {
    let _ = mcall(&keys.clone(), "append", &[py_str("allergies")]);
    }
    if py_truthy(&mcall(&gen_memory::memory::store_replies::g_EAT_ASK(), "search", &[text.clone()])) {
    let _ = mcall(&keys.clone(), "append", &[py_str("diet")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&keys.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    who = gen_memory::memory::store_replies::_someone_else_named(&[gen_memory::memory::store_replies::_bare(&[py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]), memory.clone()]);
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
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::Bool(false);
    }
    let __iter6 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter6 {
    known = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone()))) {
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push(mcall(&known.clone(), "get", &[key.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_take_back_about_someone(args: &[Py]) -> Py {
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
    let mut gone: Py = Py::None;
    let _ = py_str("A line that takes a fact back off somebody else's list is the store's, not the topic's.\n\n    \"my son is allergic to nuts\" then \"he isnt allergic to nuts\" was answered with the nut-free\n    lecture from the knowledge base: the line names nuts, so `answers` read food on it and took\n    the turn from the store that had just taken them off his list. The store's note is about\n    what the chat keeps, not about nuts, so it says so in front of its reply.\n    ");
    gone = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("denial")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[gone.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&gone.clone(), "get", &[py_str("who")]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&mcall(&gone.clone(), "get", &[py_str("said")]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])));
    Py::None
}
pub fn _the_part_that_hurts_asked(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("The ask for the part that hurts, and the side it was said with.\n\n    \"my knee hurts\", then \"its on the outside of the knee\", then \"and which knee was it\" is\n    answered from what the person said, so nothing that only echoes their last line can take the\n    turn: `You said: \"its on the outside of the knee\"` says their own words again and answers\n    nothing. The claim is made for every line the hurting-part reader can answer, and for no\n    other.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[gen_memory::memory::whathurts::the_part_asked(&[memory.clone(), low.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_store_readings(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut line: Py = Py::None;
    let _ = py_str("The lines this store read for certain: a read of what it keeps, the ask whose answer is a\n    line they said themselves, a `when` ask it can place, and a question about what the chat\n    already holds. All are answered from what the person said, so a topic the knowledge base\n    declares must not take the turn.\n\n    This is a reading, not the hook: the exported name is `TAKES_AS_IT_COMES` at the end of the\n    file, and it asks this one first. A module's name is bound once, at import, so a second `def`\n    of the exported name is not an addition - it is a whole new hook, and everything under the\n    first one stops answering without a single test going red.\n\n    \"when is that again\" after \"i've got a dentist appointment on tuesday\" was answered\n    \"Same as before: Ring practices near you and ask straight out whether they take new NHS\n    patients...\" - the knowledge answer that happened to be the turn before, said again behind a\n    repeat lead-in. The date is in what the person said, so this is the memory store's line and\n    it says so in front of it. Every ask it cannot place is left alone.\n\n    \"have we talked about food\" is the same kind of line: the answers lane proposed from the word\n    `food` and won before the store was asked, so the person asking the bot to remember got a\n    recipe list instead.\n    ");
    if py_truthy(&{ let __b = _reads_the_store(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _their_own_line_ask(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_part_that_hurts_asked(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _their_fact_ask(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_kept_fact_read(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_memory::memory::decided::asks(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_memory::memory::told::ABOUT_THE_CHAT(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { gen_memory::memory::told::A_CALLBACK(&[text.clone(), memory.clone()]) } } } } } } } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&gen_memory::memory::store_replies::g_WHOLE_ASK(), "match", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]); if !py_truthy(&__b) { __b } else { gen_memory::memory::store_replies::_a_whole_answer_to_hand_back(&[memory.clone(), text.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::store_replies::_a_state_that_was(&[text.clone(), memory.clone(), py_str("")])) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_feeling_about_their_own_news(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_plan_they_took_back(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_take_back_about_someone(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::store_replies::_can_close(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::store_replies::_the_call_from_the_advice(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_asks_whether_we_followed(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::rates::is_a_rate_ask(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::times::is_a_time_ask(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_an_ask_about_someone_else(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_group_size_ask(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = gen_memory::memory::repeat::how_long_its_been(&[text.clone()]); if !py_truthy(&__b) { __b } else { gen_memory::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_figure_the_answer_names(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::store_replies::_the_budget_on_the_answer(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::store_replies::_the_back_this_turn(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::store_replies::_the_same_fact_about_someone(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    line = gen_memory::memory::store_replies::_bare(&[py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]);
    if py_truthy(&{ let __b = { let __b = mcall(&gen_memory::memory::store_replies::g_POINTS_BACK_AT_IT(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORK_ON_IT(), "search", &[line.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::store_replies::_the_prep_day(&[memory.clone(), line.clone()]))) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    if py_truthy(&_the_clock_in_their_line(&[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&Py::Bool(!py_contains(&mcall(&text.clone(), "lower", &[]), &py_str("when")))) {
    return py_tuple(vec![Py::Bool(true), _a_count_from_the_date(&[text.clone(), memory.clone()])]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&gen_memory::memory::store_replies::g_when_answer(), &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _their_fact_ask(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut undeclared: Py = Py::None;
    let _ = py_str("True for a line that only asks back a fact kept about someone else.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = gen_memory::memory::store_replies::_bare(&[py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]);
    if py_truthy(&{ let __b = mcall(&g_PERSON_ADVICE_ASK(), "match", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&gen_memory::memory::store_replies::g_CAN_THEY_HAVE(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { gen_memory::memory::store_replies::_a_limit_kept_for_them(&[low.clone(), memory.clone()]) } })) } }) {
    return Py::Bool(false);
    }
    undeclared = gen_memory::memory::store_replies::_the_person_still_in_hand(&[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = undeclared.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_OTHER_FACT(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_EAT_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_CANT_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_CAN_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&gen_memory::memory::store_replies::g_CAN_THEY_HAVE(), "search", &[low.clone()]) } } } } } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = undeclared.clone(); if py_truthy(&__b) { __b } else { { let __b = py_call(&gen_memory::memory::store_replies::g__asks(), &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { _asks_about_someone_else(&[text.clone()]) } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = undeclared.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&gen_memory::memory::store_replies::g_THEM(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = py_call(&gen_memory::memory::store_replies::g__named_person(), &[low.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bmy\\s+(?:"), &gen_memory::memory::store_replies::g__RELATION()), &py_str(")\\b(?!'s|s\\b)(?!s\\b)")), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&gen_memory::memory::store_replies::g_CAN_THEY_HAVE_A_FOOD(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::store_replies::_a_limit_kept_for_them(&[low.clone(), memory.clone()]))) } }) {
    return Py::Bool(false);
    }
    said = gen_memory::memory::store_replies::_other_person_reply(&[memory.clone(), low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&gen_memory::memory::store_replies::g_NO_RECORD(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _name_the_last_pet(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut last: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut old: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("\"we've also got a cat\" then \"and her name is Biscuit\" is one cat with a name.\n\n    A name given a second time is a correction when the line opens like one - \"sorry, his name\n    is milo\" after \"we have a dog called max\". The store kept the first name and said it back\n    (\"You've got a dog called Max\") while the person had already taken it back, and the same line\n    with the old name on the end (\"sorry, his name is milo not max\") was read as not having a dog\n    at all. Only the opener makes it a rename: \"we have a dog called max / his name is milo\" on\n    its own is the same dog said twice as easily as a correction.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    pets = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]);
    named = if py_truthy(&{ let __b = call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = pets.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&pets.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) { geti(&pets.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    name = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("lower".to_string(), call_builtin("bool", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::facts::pet_name_only(args))(&__args) }; if py_truthy(&__b) { __b } else { gen_memory::memory::store_replies::_name_they_answered(&[text.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pets.clone())) } }) {
    animal = gen_memory::memory::store_replies::_the_animal_of_the_pick(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&animal.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&animal.clone(), &py_str("pet"))) } }) {
    return Py::None;
    }
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("pets"), py_list(vec![py_dict(vec![(py_str("animal"), animal.clone()), (py_str("name"), name.clone())])]), text.clone()]);
    return Py::None;
    }
    last = geti(&pets.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&{ let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&last.clone(), "get", &[py_str("name")]))) } }) {
    let _ = py_setitem(&last.clone(), &py_str("name"), name.clone());
    kept = py_dict(vec![(py_str("animal"), mcall(&last.clone(), "get", &[py_str("animal"), py_str("pet")])), (py_str("name"), name.clone())]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&last.clone(), "get", &[py_str("count")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_gt(&geti(&last.clone(), &py_str("count")), &Py::Int(1)) } }) {
    let _ = py_setitem(&kept.clone(), &py_str("count"), geti(&last.clone(), &py_str("count")));
    }
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("pets"), py_list(vec![kept.clone()]), text.clone()]);
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&name.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_CORRECTS_A_NAME(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CORRECTS_A_NAME_POSSESSIVE(), "match", &[text.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_pet_here(&[text.clone(), memory.clone()]))) } } } } } }) {
    before = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&last.clone(), &py_str("name"), name.clone());
    let _ = mcall(&gen_memory::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), py_dict(vec![(py_str("animal"), mcall(&last.clone(), "get", &[py_str("animal")])), (py_str("name"), name.clone())])), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&before.clone(), "lower", &[]), &mcall(&name.clone(), "lower", &[]))) } }) {
    was = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&last.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut old: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&name.clone(), "lower", &[])))) {  __out.push(call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    let _ = py_setitem(&last.clone(), &py_str("was"), op_add(&was.clone(), &py_list(vec![before.clone()])));
    }
    if py_truthy(&before.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), name.clone()), (py_str("animal"), { let __b = mcall(&last.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } }), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    }
    }
    Py::None
}
pub fn _the_animals_named(args: &[Py]) -> Py {
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
    let mut here: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("The animals of the store's pets the line names: \"no the cat is called tom\" names the cat.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    here = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("animal")]))) } }) {
    continue;
    }
    animal = mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("animal"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[animal.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&here.clone(), "add", &[animal.clone()]);
    }
    }
    return here.clone();
    Py::None
}
pub fn _the_name_it_had(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut entry: Py = Py::None;
    let mut name: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The name the store had for this animal before the line that named it again.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = gen_memory::memory::log(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    value = if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&entry.clone(), "get", &[py_str("value")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &animal.clone()))) {
    continue;
    }
    name = call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = name.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&name.clone(), "lower", &[]), &mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return name.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _names_a_pet_here(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("Whether the line names an animal the store keeps a pet of (\"the cat\", \"the dogs\").");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("animal")]))) } }) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("animal"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_figure_said(args: &[Py]) -> Py {
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
    let mut value: Py = Py::None;
    let _ = py_str("The age a reader's own regex caught, as a number, whether it was digits or a word.");
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("age")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    value = gen_memory::memory::store_replies::_a_number(&[mcall(&gen_memory::memory::facts::g_FIGURE_WORDS(), "get", &[said.clone(), said.clone()])]);
    return if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { Py::None };
    Py::None
}
pub fn _without_a_lead_in(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("The line with the lead-ins people open a short sentence with taken off.");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(4)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    shorter = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET_LEAD_IN(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&shorter.clone(), &said.clone()))) {
    break;
    }
    said = mcall(&shorter.clone(), "strip", &[]);
    }
    return said.clone();
    Py::None
}
pub fn _put_a_pets_age(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pet: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pet"))) };
    let mut age: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "age") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: age"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("Store an age on one animal, or None when it already says exactly that.");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&pet.clone(), "get", &[py_str("age")]), &age.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &unit.clone())) } }) {
    return Py::None;
    }
    let _ = py_setitem(&pet.clone(), &py_str("age"), age.clone());
    let _ = mcall(&pet.clone(), "pop", &[py_str("unit"), Py::None]);
    if py_truthy(&unit.clone()) {
    let _ = py_setitem(&pet.clone(), &py_str("unit"), unit.clone());
    }
    return py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), call_builtin("dict", &[pet.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())]);
    Py::None
}
pub fn _age_the_named_pet(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("\"my dog is called fred\" then \"fred is 5\" is an age for the dog, not a fact about nobody.\n\n    The line names no animal at all, only the name the store has for one, so the store is what\n    says whose age it is. Without this the 5 went nowhere and \"how old is fred\" had no answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    low = mcall(&_without_a_lead_in(&[text.clone()]), "lower", &[]);
    match_ = mcall(&g_PET_NAME_AGE(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(30)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    unit = if py_truthy(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "startswith", &[py_str("month")])) { py_str("months") } else { py_str("") };
    said = if py_truthy(&mcall(&low.clone(), "strip", &[])) { mcall(&geti(&mcall(&mcall(&low.clone(), "strip", &[]), "split", &[]), &Py::Int(0)), "strip", &[py_str(".,!?")]) } else { py_str("") };
    pets = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone())) } }) {
    continue;
    }
    return _put_a_pets_age(&[pet.clone(), age.clone(), unit.clone(), text.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _fix_a_pets_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut aged: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("\"my dog is 5\" then \"sorry he is 6\": the correction goes on the animal it corrects.\n\n    The line names no animal - it names a pronoun and a figure - and the store already has one\n    age for the animal the chat is on, so that is the age the figure replaces. Read the same way\n    a person's corrected age is read (`_fix_a_persons_age`): only a short line, only with a\n    correction word in it, and only when there is an age stored to replace, because a\n    correction replaces a fact rather than being the first one said.\n\n    Without this the store kept the old figure and the bot said it as fact: \"my dog is 5\" /\n    \"sorry he is 6\" / \"how old is my dog\" answered \"Your dog is 5.\" after the person had said 6.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    low = _without_a_lead_in(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &gen_memory::memory::store_lines::g_PET_AGE_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CORRECTING(), "search", &[said.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&gen_memory::memory::relations::g__PET_WORD(), "search", &[low.clone()])) {
    return Py::None;
    }
    match_ = mcall(&g_AGE_FIX(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(30)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    who = py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut known: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("age"))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    unit = if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmonths?\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_str("months") } else { py_str("") };
    aged = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&pet.clone(), "get", &[py_str("age")]), &Py::None)) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[aged.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    return _put_a_pets_age(&[geti(&aged.clone(), &Py::Int(0)), age.clone(), unit.clone(), text.clone()]);
    Py::None
}
pub fn _age_said_of_the_only_animal(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut called: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"i have a greyhound called sid\" then \"hes 6\": the figure is that animal's age.\n\n    `_age_they_answered` reads a bare figure only when the bot asked for one, so a figure the\n    person volunteers (\"hes 6\", \"shes 12 now\") went nowhere and \"how old is sid\" answered that\n    they had never said. It is only read when the line before it named an animal or the one the\n    store knows by name, and when the chat holds one animal - with two, \"hes 6\" says nothing\n    about which one.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return Py::None;
    }
    previous = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = py_call(&gen_memory::memory::store_replies::g_last_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    pet = geti(&pets.clone(), &Py::Int(0));
    } else {
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&mcall(&call_builtin("str", &[geti(&one.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), previous.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    pet = geti(&named.clone(), &Py::Int(0));
    }
    if py_truthy(&op_gt(&call_builtin("int", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::None;
    }
    name = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    called = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("animal"), py_str("breed")])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&gen_memory::memory::relations::g__PET_WORD(), "search", &[previous.clone()]); if py_truthy(&__b) { __b } else { { let __b = { let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&previous.clone(), &name.clone())) } }; if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&called.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&previous.clone(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:he|she|it|they)(?:'s|s|\\s+is|\\s+are)?\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    match_ = mcall(&g_PET_AGE_ANSWER(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(30)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    if py_truthy(&py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[said.clone(), memory.clone()])) {
    return Py::None;
    }
    unit = mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    unit = if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) { py_str("months") } else { if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("week")])) { py_str("weeks") } else { py_str("") } };
    return _put_a_pets_age(&[pet.clone(), age.clone(), unit.clone(), text.clone()]);
    Py::None
}
pub fn _age_they_answered(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The bot asked how old their animal was, so the figure in this line is that animal's age:\n\n        i have a dog / You haven't told me how old your dog is. / hes 3\n\n    A bare age names nobody, so it is read only when the reply before it asked, the same way a\n    bare name is only read when the bot asked for one.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    asked = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_contains(&mcall(&asked.clone(), "lower", &[]), &py_str("how old")))) {
    return Py::None;
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&pets.clone()))) {
    return Py::None;
    }
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&gen_memory::memory::relations::g__PET_WORD(), "search", &[asked.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let __b = name.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), mcall(&asked.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::None;
    }
    match_ = mcall(&g_PET_AGE_ANSWER(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(30)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    unit = mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    unit = if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) { py_str("months") } else { if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("week")])) { py_str("weeks") } else { py_str("") } };
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&pet.clone(), "get", &[py_str("age")]), &age.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &unit.clone())) } }) {
    return Py::None;
    }
    let _ = py_setitem(&pet.clone(), &py_str("age"), age.clone());
    let _ = mcall(&pet.clone(), "pop", &[py_str("unit"), Py::None]);
    if py_truthy(&unit.clone()) {
    let _ = py_setitem(&pet.clone(), &py_str("unit"), unit.clone());
    }
    let _ = py_setitem(&memory.clone(), &py_str("pet_age"), call_builtin("dict", &[pet.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), call_builtin("dict", &[pet.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _their_name_answered(args: &[Py]) -> Py {
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
    let mut answered: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut last: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own name, given as the answer to the bot's own question:\n\n        can you remember my name / I don't know it yet - you haven't said / sam\n        can you remember my name / I don't know it yet - you haven't said / its sam\n\n    A word on its own says nothing, so it is read as a name only when the reply right before it\n    asked for one, and only when no name is on record yet (\"sam\" to a bot that already has the\n    name is something else). \"its\" in front of the word is only an introduction there.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("name")])) {
    return Py::None;
    }
    asked = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&memory.clone(), "get", &[py_str("asked_for")]), &py_str("name"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&gen_memory::memory::facts::g_USER_NAME_ASKED(), "search", &[asked.clone()]))) } }) {
    return Py::None;
    }
    answered = gen_memory::memory::facts::without_the_hedges(&[text.clone()]);
    match_ = mcall(&gen_memory::memory::facts::g_NAME_ANSWER_ANNOUNCED(), "match", &[answered.clone()]);
    if py_truthy(&match_.clone()) {
    return gen_memory::memory::facts::reads_as_a_name(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    }
    last = mcall(&gen_memory::memory::facts::g_NAME_ANSWER_ANNOUNCED_LAST(), "search", &[answered.clone()]);
    if py_truthy(&last.clone()) {
    return gen_memory::memory::facts::reads_as_a_name(&[mcall(&last.clone(), "group", &[Py::Int(1)])]);
    }
    return gen_memory::memory::facts::reads_as_a_name(&[answered.clone()]);
    Py::None
}
pub fn _fix_a_persons_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Put a corrected age on the person the pronoun points at.\n\n    \"my son is 7\" then \"actually hes 8\" leaves the store saying 7 while the bot has already said\n    \"Got it - he's 8.\", so the person thinks the correction landed. The line names no relation,\n    so who it is about comes from the pronoun, and the age it replaces has to be one the store\n    already has.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &gen_memory::memory::store_lines::g_AGE_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CORRECTING(), "search", &[low.clone()]))) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_AGE_FIX(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(120)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    who = py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter2 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("age"))) } }) {
    let _ = py_setitem(&known.clone(), &py_str("age"), age.clone());
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&gen_memory::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _fix_the_weeks_of_a_pregnancy(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut own: Py = Py::None;
    let mut said_mine: Py = Py::None;
    let mut said_theirs: Py = Py::None;
    let mut weeks: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Move the length the store holds to the one this line gives, for the person it points at.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::None;
    }
    match_ = mcall(&g_WEEKS_FIX(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_PREGNANCY_SAID_AGAIN(), "search", &[low.clone()])) {
    return Py::None;
    }
    weeks = call_builtin("int", &[mcall(&match_.clone(), "group", &[py_str("weeks")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = weeks.clone(); if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(45)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_CORRECTING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:now|today)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::None;
    }
    said_mine = stdlib_call("re", "search", &[py_str("\\b(?:im|i'm|i am|i)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    said_theirs = stdlib_call("re", "search", &[py_str("\\b(?:my|our|she|he|they|her|his|their|hers)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    own = { let __b = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = said_mine.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said_theirs.clone())); if !py_truthy(&__b) { __b } else { mcall(&own.clone(), "get", &[py_str("pregnant")]) } } } }) {
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("weeks"), weeks.clone(), text.clone()]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(own.clone()); __kws.extend(vec![("weeks".to_string(), weeks.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    who = py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter2 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&known.clone(), "get", &[py_str("pregnant")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("weeks"))) } } } }) {
    let _ = py_setitem(&known.clone(), &py_str("weeks"), weeks.clone());
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&gen_memory::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _fix_a_pets_name(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut filed: Py = Py::None;
    let mut found: Py = Py::None;
    let mut here: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kept_note: Py = Py::None;
    let mut key: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut moves: Py = Py::None;
    let mut named: Py = Py::None;
    let mut older: Py = Py::None;
    let mut other: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let mut target: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("A pet's name put right in the sentence moves the name on the pet.\n\n    \"my cat is called milo\" then \"no hes called max\": the person correction path needs a pointer\n    to a relation and a name that is a spelling of the old one, and a pet is neither, so the store\n    kept Milo and the next ask answered Milo. The pet is the animal the line names, or else the\n    pet the turn before was about.\n    ");
    let mut pet_rename: Py = gen_memory::memory::rename::module_obj();
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    moves = mcall(&pet_rename.clone(), "moves", &[low.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &gen_memory::memory::store_lines::g_NAME_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_CORRECTING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { moves.clone() } })) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_A_NAME_WITH_ITS_VERB(), "search", &[low.clone()]);
    raw = py_str("");
    if py_truthy(&match_.clone()) {
    raw = { let __b = mcall(&match_.clone(), "group", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } };
    } else {
    if py_truthy(&moves.clone()) {
    raw = mcall(&pet_rename.clone(), "the_name_a_line_moved_on", &[low.clone(), mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]), call_builtin("bool", &[mcall(&gen_memory::memory::facts::g_PETS(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&raw.clone()))) {
    return Py::None;
    }
    said = gen_memory::memory::facts::_finish(&[raw.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(raw.clone()); __kws.extend(vec![("strict_ly".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::facts::_not_a_name(args))(&__args) } } }) {
    return Py::None;
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&mcall(&pet.clone(), "get", &[py_str("name")])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    target = Py::None;
    if py_truthy(&_names_a_pet_here(&[text.clone(), memory.clone()])) {
    if py_truthy(&_another_animal_is_named(&[text.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("sorted", &[_the_animals_named(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    animal = __item.clone();
    kept = _pets_of(&[animal.clone(), pets.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    continue;
    }
    filed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&_a_pet_new_this_line(&[pet.clone(), memory.clone()])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    older = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_a_pet_new_this_line(&[pet.clone(), memory.clone()])))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&older.clone()) {
    target = geti(&older.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&said.clone(), "lower", &[])))) {
    let _ = py_setitem(&target.clone(), &py_str("name"), op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)));
    let _ = mcall(&target.clone(), "pop", &[py_str("turns"), Py::None]);
    was = _the_name_it_had(&[animal.clone(), said.clone(), memory.clone()]);
    if py_truthy(&{ let __b = was.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("correction")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&memory.clone(), &py_str("correction")), "get", &[py_str("said")]), &low.clone())) } })) } }) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), geti(&target.clone(), &py_str("name"))), (py_str("wrong"), op_add(&mcall(&slice_or_raise(&was.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&was.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("animal"), animal.clone()), (py_str("said"), low.clone()), (py_str("text"), text.clone())]));
    }
    }
    let __iter2 = py_iter_or_raise(&filed.clone());
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&pet.clone(), &target.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &pet.clone())) } }) {
    let _ = mcall(&geti(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), &py_str("pets")), "remove", &[pet.clone()]);
    }
    }
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&filed.clone());
    for __item in __iter3 {
    pet = __item.clone();
    let _ = py_setitem(&pet.clone(), &py_str("name"), op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)));
    was = _the_name_it_had(&[animal.clone(), said.clone(), memory.clone()]);
    if py_truthy(&{ let __b = was.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("correction")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&memory.clone(), &py_str("correction")), "get", &[py_str("said")]), &low.clone())) } })) } }) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), geti(&pet.clone(), &py_str("name"))), (py_str("wrong"), op_add(&mcall(&slice_or_raise(&was.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&was.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("animal"), animal.clone()), (py_str("said"), low.clone()), (py_str("text"), text.clone())]));
    }
    }
    return Py::None;
    }
    }
    if py_truthy(&_another_animal_is_named(&[text.clone()])) {
    return Py::None;
    }
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("animal"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&here.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&said.clone(), "lower", &[])))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[here.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    target = geti(&here.clone(), &Py::Int(0));
    } else {
    let __iter4 = py_iter_or_raise(&py_tuple(vec![py_str("name"), py_str("animal")]));
    for __item in __iter4 {
    key = __item.clone();
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&_the_turn_before_said(&[mcall(&pet.clone(), "get", &[key.clone()]), memory.clone()])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&found.clone()) {
    target = geti(&found.clone(), &op_neg(&Py::Int(1)));
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&target.clone(), &Py::None))) {
    return Py::None;
    }
    before = call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&mcall(&before.clone(), "lower", &[]), &mcall(&said.clone(), "lower", &[])))) {
    return Py::None;
    }
    let _ = py_setitem(&target.clone(), &py_str("name"), op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)));
    let _ = mcall(&target.clone(), "pop", &[py_str("turns"), Py::None]);
    kept_note = mcall(&memory.clone(), "get", &[py_str("correction")]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[kept_note.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&kept_note.clone(), "get", &[py_str("said")]), &low.clone())) } }))) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), geti(&target.clone(), &py_str("name"))), (py_str("wrong"), op_add(&mcall(&slice_or_raise(&before.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&before.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("animal"), call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("said"), low.clone()), (py_str("text"), text.clone())]));
    }
    animal = mcall(&call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter5 = py_iter_or_raise(&call_builtin("list", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    other = __item.clone();
    if py_truthy(&Py::Bool(py_is(&other.clone(), &target.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&other.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &animal.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&other.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&said.clone(), "lower", &[]))) } }) {
    let _ = mcall(&pets.clone(), "remove", &[other.clone()]);
    }
    }
    let _ = py_setitem(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), &py_str("pets"), pets.clone());
    let _ = mcall(&gen_memory::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), call_builtin("dict", &[target.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return target.clone();
    Py::None
}
pub fn _pets_of(args: &[Py]) -> Py {
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
    let mut pets: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut pet: Py = Py::None;
    let _ = py_str("The pets of one animal in the store's list, in the order the store keeps them.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &animal.clone())) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _a_pet_new_this_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pet: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pet"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut held: Py = Py::None;
    let _ = py_str("Whether this pet entry was filed by the line being read.\n\n    The store before the line was read is carried in the memory for the length of the turn\n    (`_pets_before`), so an entry that was not there before is the line's own: it is the name\n    the line said, filed beside the pets it was filed with, and not a pet of its own.\n    ");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("_pets_before")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&held.clone(), &py_tuple(vec![mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])])));
    Py::None
}
pub fn _another_animal_is_named(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line says the name belongs to another animal, not the one on record.");
    return call_builtin("bool", &[mcall(&g_ANOTHER_ANIMAL(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _two_of_the_animal_are_kept(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pets: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut animal: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut kept: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("Whether the store holds more than one pet of this animal.\n\n    When it does, a name filed against that animal was filed beside the pet already there. A\n    rename leaves one entry (the rename drops the echo the same line filed), so two entries of\n    the animal said with the new name is the store keeping both, not one name put right.\n    ");
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &animal.clone())) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    return op_gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _the_turn_before_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the line before this one said this word about the pet in question.");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } } } }) {
    return Py::Bool(false);
    }
    last = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fix_the_make_of_a_thing(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut held: Py = Py::None;
    let mut last_said: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("A correction that names only the make goes on the last thing the store was told about.\n\n    \"my car is a ford fiesta\" and then \"sorry, its a focus\" is one car with its make put right,\n    and a store that keeps the fiesta answers the next ask with a car they have just corrected.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_CORRECTS_A_MAKE(), "search", &[mcall(&line.clone(), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    held = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("owns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&held.clone())) } }) {
    return Py::None;
    }
    fits = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&held.clone());
    for __item in __iter1 {
    thing = __item.clone();
    name = gen_memory::memory::facts::_the_model(&[mcall(&match_.clone(), "group", &[py_str("value")]), thing.clone()]);
    if py_truthy(&name.clone()) {
    let _ = py_setitem(&fits.clone(), &thing.clone(), name.clone());
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&fits.clone()))) {
    return Py::None;
    }
    last_said = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&gen_memory::memory::log(&[memory.clone()]));
    for __item in __iter2 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("owns")))) {
    let _ = mcall(&last_said.clone(), "extend", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("value")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    }
    }
    thing = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[last_said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&fits.clone(), &one.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() }, call_builtin("next", &[call_builtin("iter", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), py_str("")]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    name = mcall(&fits.clone(), "get", &[thing.clone(), py_str("")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&name.clone(), &mcall(&held.clone(), "get", &[thing.clone()]))) } }) {
    return Py::None;
    }
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("owns"), py_dict(vec![(thing.clone(), name.clone())]), text.clone()]);
    Py::None
}
pub fn _fix_a_persons_name(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut before: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Put a corrected name on the person the pronoun points at.\n\n    \"my sister is called meg\" then \"actually she's called maggie\" left the store saying Meg, so\n    the ask after it answered with the name the person had just put right. The line names no\n    relation, so who it is about comes from the pronoun, and the name it replaces has to be one\n    the store already has: a correction replaces a fact, it is not the first one said.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &gen_memory::memory::store_lines::g_NAME_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CORRECTING(), "search", &[low.clone()]))) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_NAME_FIX(), "search", &[low.clone()]);
    bare = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    bare = mcall(&g_A_NAME_AFTER_A_PRONOUN(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return Py::None;
    }
    word = mcall(&mcall(&bare.clone(), "group", &[py_str("name")]), "strip", &[py_str("'’")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&gen_memory::memory::facts::g_COMMON_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&gen_memory::memory::facts::g_JOB_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&gen_memory::memory::facts::g_KIN(), &word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __kws.extend(vec![("strict_ly".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::facts::_not_a_name(args))(&__args) } } } } } } }) {
    return Py::None;
    }
    match_ = bare.clone();
    }
    said = mcall(&mcall(&match_.clone(), "group", &[py_str("name")]), "strip", &[py_str("'’")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NAME_NOT_A_NAME(), &said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("strict_ly".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::facts::_not_a_name(args))(&__args) } } } } }) {
    return Py::None;
    }
    name = gen_memory::memory::facts::_tidy(&[{ let __b = gen_memory::memory::facts::_finish(&[said.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    about = stdlib_call("re", "sub", &[py_str("\\bhis\\b(\\s+name)"), py_str("he\\1"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    about = stdlib_call("re", "sub", &[py_str("\\bher\\b(\\s+name)"), py_str("she\\1"), about.clone()]).unwrap_or_else(|e| exc::raise(e));
    who = py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[about.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { bare.clone() } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&mcall(&entry.clone(), "get", &[py_str("name")])) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    who = call_builtin("str", &[{ let __b = mcall(&geti(&named.clone(), &Py::Int(0)), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter1 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { mcall(&known.clone(), "get", &[py_str("name")]) } }) {
    before = call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&before.clone(), "lower", &[]), &mcall(&gen_memory::memory::facts::_tidy(&[name.clone()]), "lower", &[]))) } }) {
    return Py::None;
    }
    let _ = py_setitem(&known.clone(), &py_str("name"), gen_memory::memory::facts::_tidy(&[name.clone()]));
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&gen_memory::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return py_dict(vec![(py_str("who"), who.clone()), (py_str("name"), call_builtin("str", &[geti(&known.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))), (py_str("from"), before.clone())]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_name_they_gave_to_the_person_they_mean(args: &[Py]) -> Py {
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
    let mut fits: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut unnamed: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Put a name given with a pronoun on the person the store already holds.\n\n    \"my son is called jack\" then \"hes called jake\" left the store saying Jack while the reply said\n    \"He's called jake, good to know\": the person is told it landed and it did not, so the next\n    \"what is my son called\" answers with the name they have just given. Who it is about comes from\n    the pronoun.\n\n    A name the store already holds is replaced. A person it holds and never named gets the name as\n    their first one - \"im taking my daughter to london\" then \"her name is maggie\" - because the\n    reply already promises to remember it, and only when exactly one unnamed person fits the\n    pronoun.\n\n    A bare name after the word that opens a correction (\"no, jake\") is not this reader's: that is\n    a correction of a name said wrong, and `_correct_a_persons_name` owns it.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &gen_memory::memory::store_lines::g_NAME_GIVEN_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("?"))) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_NAME_GIVEN(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    raw = call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("name")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("name2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    name = mcall(&raw.clone(), "strip", &[py_str(" .,!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&gen_memory::memory::facts::_may_be_a_name(&[name.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_NAME_AFTER_CALLED(), &mcall(&name.clone(), "lower", &[]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&slice_or_raise(&low.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .,!;:")])) {
    return Py::None;
    }
    who = py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[py_str("name")]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[mcall(&known.clone(), "get", &[py_str("name")])]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &name.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&known.clone(), &py_str("name"), gen_memory::memory::facts::_tidy(&[name.clone()]));
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&gen_memory::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    if py_truthy(&mcall(&gen_memory::memory::facts::g_PETS(), "search", &[low.clone()])) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:she|her)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    fits = gen_memory::memory::facts::g_SHE();
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:he|his)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    fits = gen_memory::memory::facts::g_HE();
    } else {
    fits = Py::None;
    }
    }
    unnamed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&one.clone(), "get", &[py_str("who")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&one.clone(), "get", &[py_str("name")]))) } }) {  __out.push(call_builtin("str", &[mcall(&one.clone(), "get", &[py_str("who")])]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_set(__out) })() };
    let __aug2 = op_add(&unnamed.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&kept.clone(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() });
    unnamed = __aug2;
    unnamed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[unnamed.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&fits.clone())); if py_truthy(&__b) { __b } else { gen_memory::memory::relations::_pronoun_fits(&[word.clone(), fits.clone()]) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[unnamed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter3 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &geti(&unnamed.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[py_str("name")]))) } }) {
    let _ = py_setitem(&known.clone(), &py_str("name"), gen_memory::memory::facts::_tidy(&[name.clone()]));
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&gen_memory::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    }
    let _ = py_call(&gen_memory::memory::store_replies::g_remember_other(), &[memory.clone(), py_dict(vec![(py_str("who"), geti(&unnamed.clone(), &Py::Int(0))), (py_str("name"), gen_memory::memory::facts::_tidy(&[name.clone()]))]), text.clone()]);
    return py_dict(vec![(py_str("who"), geti(&unnamed.clone(), &Py::Int(0))), (py_str("name"), gen_memory::memory::facts::_tidy(&[name.clone()]))]);
    Py::None
}
pub fn _a_person(args: &[Py]) -> Py {
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
    let mut relation: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the corrected value names a person (\"sorry, i meant my wife\").\n\n    A person put right is answered better by the sentence that says whose the fact was, so the\n    short echo stays out of the way and the turn is left to it.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gen_memory::memory::facts::g_RELATIONS()) { let mut relation: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &mcall(&relation.clone(), "rstrip", &[py_str("s")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_correction_without_a_marker(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut right: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut wrong: Py = Py::None;
    let _ = py_str("The {to, from} of a pair correction said with no \"no\"/\"sorry\" in front, or None.\n\n    Only a value the store already holds can be taken back this way, so the line has to name an\n    older fact in the half it corrects: \"its Preeya not Priya\" after \"my name is Priya\" is a\n    rename, and \"its raining not sunny\" is a sentence about the weather that leaves the store\n    alone.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_PAIR_WITHOUT_A_MARKER());
    for __item in __iter1 {
    shape = __item.clone();
    found = mcall(&shape.clone(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![gen_memory::memory::facts::_tidy(&[mcall(&found.clone(), "group", &[py_str("to")])]), gen_memory::memory::facts::_tidy(&[mcall(&found.clone(), "group", &[py_str("wrong")])])]), 2, None);
    right = __unpk2[0].clone();
    wrong = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&right.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&wrong.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&right.clone(), "lower", &[]), &mcall(&wrong.clone(), "lower", &[]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_NEW_VALUE(), &mcall(&right.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_NEW_VALUE(), &mcall(&wrong.clone(), "lower", &[]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gen_memory::memory::corrected::_fact_naming(&[memory.clone(), wrong.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_person_called(&[memory.clone(), wrong.clone()]))) } }) {
    continue;
    }
    return py_dict(vec![(py_str("to"), right.clone()), (py_str("from"), wrong.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _a_person_called(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut entry: Py = Py::None;
    let mut low: Py = Py::None;
    let mut old: Py = Py::None;
    let _ = py_str("Whether the store holds a person under that name.\n\n    \"my sister is called anna\" then \"its hannah not anna\" names both values with no \"no\" in\n    front, and the name it takes back is one the store keeps for a person rather than in a fact\n    of the user's own - so the pair reader has to look in both places before it says the line\n    corrects nothing.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = name.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &low.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut old: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &low.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _stated_value(args: &[Py]) -> Py {
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
    let mut entries: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The value of `key` the line before this one stated, or None.\n\n    A correction replaces what was just said, so the old value is the one on that line: \"im\n    allergic to peanuts\" then \"make that tree nuts\" takes back peanuts, and not a food named\n    three turns earlier.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = py_call(&gen_memory::memory::store_replies::g_last_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
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
    if py_truthy(&Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &key.clone()))) {
    return mcall(&entry.clone(), "get", &[py_str("value")]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_second_persons_limit(args: &[Py]) -> Py {
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
    let mut allergies: Py = Py::None;
    let mut diet: Py = Py::None;
    let mut foods: Py = Py::None;
    let mut giving: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The limit the line before gave, carried onto the person this line names.\n\n    Only the limit is carried, and only when the line before stated one: \"my son is too\" after a\n    line about the weather carries nothing, and the person is filed with the words the store\n    already uses for an allergy and for a diet.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    match_ = mcall(&g_TOO_AS_WELL(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    who = py_call(&gen_memory::memory::store_replies::g__canon_relation(), &[gen_memory::memory::facts::a_person_word(&[mcall(&match_.clone(), "group", &[py_str("who")])])]);
    if py_truthy(&Py::Bool(!py_contains(&gen_memory::memory::facts::g__A_WORD_FOR_A_PERSON(), &who.clone()))) {
    return Py::None;
    }
    theirs = gen_memory::memory::store_replies::_the_limit_the_last_line_gave(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&theirs.clone()))) {
    return Py::None;
    }
    giving = py_dict(vec![(py_str("who"), who.clone())]);
    allergies = mcall(&theirs.clone(), "get", &[py_str("allergies")]);
    if py_truthy(&call_builtin("isinstance", &[allergies.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    foods = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&allergies.clone()) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&foods.clone()) {
    let _ = py_setitem(&giving.clone(), &py_str("allergies"), foods.clone());
    }
    }
    diet = mcall(&theirs.clone(), "get", &[py_str("diet")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[diet.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&diet.clone(), "strip", &[]) } }) {
    let _ = py_setitem(&giving.clone(), &py_str("diet"), diet.clone());
    }
    if py_truthy(&op_lt(&call_builtin("len", &[giving.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    return giving.clone();
    Py::None
}
pub fn _drop_value(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut held: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut left: Py = Py::None;
    let mut low: Py = Py::None;
    let mut v: Py = Py::None;
    let _ = py_str("'Sorry, not peanuts' takes that one back: it comes off the list of things they named.\n\n    The list is the fact the line before it stated, and only a value on it can go, so a \"not\"\n    about anything else (\"no, not today\") leaves memory alone. Nothing is put in its place, so\n    the note of the turn says the value went instead of a new one arriving.\n    ");
    known = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]);
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&gen_memory::memory::store_lines::g_DROP_LISTS());
    for __item in __iter1 {
    key = __item.clone();
    held = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut v: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[v.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_this(&[v.clone(), low.clone()]))) } }) {  __out.push(v.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = py_setitem(&known.clone(), &key.clone(), left.clone());
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("dropped"), Py::Bool(true)), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return geti(&memory.clone(), &py_str("correction"));
    }
    return _drop_from_plan(&[memory.clone(), value.clone(), text.clone()]);
    Py::None
}
pub fn _plan_names(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Does the plan say this value? A plan is kept as the sentence it arrived in, so the thing\n    taken back is a word inside it (\"Lisbon\" in \"Lisbon in may\") and not an item on a list.");
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&low.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &slice_or_raise(&low.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }; if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&low.clone(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _drop_from_plan(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut city: Py = Py::None;
    let mut current: Py = Py::None;
    let mut held: Py = Py::None;
    let mut home: Py = Py::None;
    let mut known: Py = Py::None;
    let mut left: Py = Py::None;
    let mut left_places: Py = Py::None;
    let mut low: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let _ = py_str("'Sorry, not Lisbon' after 'im going to Lisbon in May' takes the trip back with the place.\n\n    A value taken back can be a word inside a plan rather than an item on a list, and a plan has\n    no key of its own: it is the sentence the person said. Only a plan that names the value goes,\n    so \"sorry, not tuesday\" leaves the trip to Lisbon alone, and the place goes with it - a store\n    that still answers \"you're going to lisbon in may\" is telling them the thing they just took\n    back. A city they also stated on its own (\"i live in lisbon\") stays: it is a fact about where\n    they live, not the plan they dropped.\n    ");
    known = py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]);
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&gen_memory::memory::store_replies::g_MONTHS(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&gen_memory::memory::facts::g_DAYS(), &low.clone())) } }) {
    return Py::None;
    }
    plans = mcall(&known.clone(), "get", &[py_str("plans")]);
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[plans.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { plans.clone() } else { py_list(vec![]) }) { let mut plan: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(plan.clone()); } } py_list(__out) })() };
    current = mcall(&known.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[current.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &current.clone())) } }) {
    let _ = mcall(&held.clone(), "append", &[current.clone()]);
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut plan: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_plan_names(&[plan.clone(), low.clone()])))) {  __out.push(plan.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&left.clone()) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), slice_or_raise(&left.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    let _ = py_setitem(&known.clone(), &py_str("plan"), geti(&left.clone(), &op_neg(&Py::Int(1))));
    } else {
    let _ = mcall(&known.clone(), "pop", &[py_str("plans"), Py::None]);
    let _ = mcall(&known.clone(), "pop", &[py_str("plan"), Py::None]);
    }
    city = mcall(&known.clone(), "get", &[py_str("city")]);
    home = { let __b = call_builtin("isinstance", &[city.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _plan_names(&[city.clone(), low.clone()]) } };
    places = mcall(&known.clone(), "get", &[py_str("places")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[places.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&home.clone())) } }) {
    left_places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut place: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[place.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _plan_names(&[place.clone(), low.clone()]) } }))) {  __out.push(place.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[left_places.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[places.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&left_places.clone()) {
    let _ = py_setitem(&known.clone(), &py_str("places"), left_places.clone());
    } else {
    let _ = mcall(&known.clone(), "pop", &[py_str("places"), Py::None]);
    }
    }
    }
    let _ = py_setitem(&known.clone(), &py_str("plan_dropped"), low.clone());
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("dropped"), Py::Bool(true)), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return geti(&memory.clone(), &py_str("correction"));
    Py::None
}
pub fn _names_this(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut held: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let _ = py_str("Does the value the person named name the thing on the list? Only the word itself and its\n    singular or plural: \"peanut\" is \"peanuts\", and \"nuts\" is not something else that went.");
    held = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(py_eq(&held.clone(), &low.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![held.clone(), low.clone()]), py_tuple(vec![low.clone(), held.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    a = __unpk2[0].clone();
    b = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&b.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&a.clone(), &slice_or_raise(&b.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _budget_sign(args: &[Py]) -> Py {
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
    let mut budget: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "budget") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: budget"))) };
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let _ = py_str("The currency sign before the same amount in the line they typed, or \"\".");
    number = stdlib_call("re", "sub", &[py_str("[^\\d.,]"), py_str(""), call_builtin("str", &[{ let __b = budget.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&number.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("([\\u00a3$\\u20ac])\\s?([\\d][\\d,.]*)"), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&stdlib_call("re", "sub", &[py_str("[^\\d.,]"), py_str(""), mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)), &number.clone()))) {
    return mcall(&match_.clone(), "group", &[Py::Int(1)]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_feeling_about_their_own_news(args: &[Py]) -> Py {
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
    let _ = py_str("This line is a feeling about the thing they just said, and the store answers it.\n\n    A bare \"im scared\" is read by the small-talk topics as small talk, so a candidate from there\n    would take the turn; this says the store read the line whole and names what it is about. It is\n    one of the lines the arbiter's single hook asks about, not a hook of its own: a second\n    `TAKES_AS_IT_COMES` in this module would shadow the first one and take the store's recall and\n    `when` asks out of service.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&gen_memory::memory::store_replies::_a_feeling_about_what_they_said(&[memory.clone(), gen_memory::memory::store_replies::_bare(&[py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])])]), &Py::None))]);
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
    else { Py::None };
    let _ = py_str("True when the line is an ask for the bot's own last answer, so the store answers it.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return gen_memory::memory::store_replies::_repeat_ask(&[text.clone(), py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]), memory.clone()]);
    Py::None
}
pub fn _the_ask_about_how_it_went(args: &[Py]) -> Py {
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
    let _ = py_str("This line asks how the thing they told us about went, and only they can say.\n\n    \"i had an interview today\" / \"how did it go\" is a question about the person's own day: the\n    knowledge base read the word \"interview\" and answered how to prepare for one, one line after\n    they said it was over, and the specialist rule handed it the turn. The ask names no subject\n    of its own on a line that does not, so the store says it read this line whole\n    (bot/memory/thread.went).\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&gen_memory::memory::thread::went(&[text.clone(), memory.clone()]), &Py::None))]);
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
pub fn _a_count_from_the_date(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the ask is a count from a date already on file: \"how many weeks till that\".\n\n    The date is in a line they said, so the answer is arithmetic on it and this store's line; a\n    knowledge entry for the thing named in the ask (\"the driving test\") answers what that thing\n    is, which is a different question.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&gen_memory::memory::store_replies::g_until_answer(), &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _the_unit_asked(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut per_week: Py = Py::None;
    let mut units: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The unit families this line asks a figure in, or None when it names a thing of its own.");
    found = mcall(&g_FIGURE_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    units = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&found.clone(), "group", &[py_str("rest")])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FIGURE_FILLER(), &word.clone()))) {
    continue;
    }
    family = Py::None;
    let __iter2 = py_iter_or_raise(&mcall(&gen_memory::memory::store_lines::g_FIGURE_UNITS(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    name = __unpk3[0].clone();
    words = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &word.clone()))) {
    family = name.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&units.clone(), &family.clone()))) {
    let _ = mcall(&units.clone(), "append", &[family.clone()]);
    }
    }
    per_week = stdlib_call("re", "match", &[op_modulo(&py_str("^\\W*(?:a|an|per|each)\\s+(?:%s)\\b"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gen_memory::memory::store_lines::g_FIGURE_TIME()) { let mut family: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&gen_memory::memory::store_lines::g_FIGURE_UNITS(), &family.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), { let __b = mcall(&found.clone(), "group", &[py_str("rest")]); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = per_week.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[py_str("asked")]), "lower", &[]), &py_str("much"))); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("set", &[units.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[gen_memory::memory::store_lines::g_FIGURE_TIME()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return g_FIGURE_MONEY();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&units.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[py_str("asked")]), "lower", &[]), &py_str("much"))) } }) {
    return g_FIGURE_MONEY();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&units.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[py_str("asked")]), "lower", &[]), &py_str("many"))) } }) {
    return Py::None;
    }
    return { let __b = call_builtin("tuple", &[units.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { gen_memory::memory::store_lines::g_FIGURE_TIME() } };
    Py::None
}
pub fn _the_clause(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut at: Py = Py::None;
    let mut left: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut rights: Py = Py::None;
    let _ = py_str("The clause of `said` a figure sits in, between the punctuation either side of it.");
    left = call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_str(".;:")) { let mut mark: Py = __item.clone(); __out.push(mcall(&said.clone(), "rfind", &[mark.clone(), Py::Int(0), start.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    rights = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_str(".;:")) { let mut mark: Py = __item.clone(); __out.push(mcall(&said.clone(), "find", &[mark.clone(), end.clone()])); } py_list(__out) })() }) { let mut at: Py = __item.clone(); if py_truthy(&op_ge(&at.clone(), &Py::Int(0))) {  __out.push(at.clone()); } } py_list(__out) })() };
    return slice_or_raise(&said.clone(), &op_add(&left.clone(), &Py::Int(1)), &if py_truthy(&rights.clone()) { call_builtin("min", &[rights.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) }, &Py::None);
    Py::None
}
pub fn _the_money_figures(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("Every amount of money the answer states, in order, as (words, definite).");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_A_MONEY_FIGURE(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&mcall(&g_ACTION_AMOUNT(), "search", &[_the_clause(&[said.clone(), mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "end", &[])])])) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[]), "rstrip", &[py_str(",;:")]), Py::Bool(true)])]);
    }
    if py_truthy(&out.clone()) {
    return out.clone();
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_A_PERCENT_FIGURE(), "finditer", &[said.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&mcall(&g_ACTION_AMOUNT(), "search", &[_the_clause(&[said.clone(), mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "end", &[])])])) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[]), "rstrip", &[py_str(",;:")]), Py::Bool(true)])]);
    }
    return out.clone();
    Py::None
}
pub fn _the_figures_in(args: &[Py]) -> Py {
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
    let mut units: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "units") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: units"))) };
    let mut after: Py = Py::None;
    let mut definite: Py = Py::None;
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut next_word: Py = Py::None;
    let mut one: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut said_figure: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut unit_word: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every figure of these units the answer states, in order, as (words, definite).");
    if py_truthy(&Py::Bool(py_contains(&units.clone(), &py_str("money")))) {
    return _the_money_figures(&[said.clone()]);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&units.clone()) { let mut unit: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&gen_memory::memory::store_lines::g_FIGURE_UNITS(), &unit.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    unit = op_modulo(&py_str("(?-i:%s)"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    pattern = stdlib_call("re", "compile", &[op_modulo(&py_str("(?<![\\d,])\\b(?P<qual>%s\\s+)?(?P<num>%s)(?:\\s*(?:-|to|or)\\s*(?P<more>%s))?\\s*(?P<unit>%s)\\b(?P<extra>\\s+\\d{1,3})?(?P<after>[^.;:!?,]*)"), &py_tuple(vec![gen_memory::memory::store_lines::g_FIGURE_QUALIFIER(), gen_memory::memory::store_lines::g_FIGURE_NUMBER(), gen_memory::memory::store_lines::g_FIGURE_NUMBER(), unit.clone()])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    after = mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("after")]), "strip", &[]), "lower", &[]);
    next_word = geti(&{ let __b = stdlib_call("re", "findall", &[py_str("[a-z']+"), after.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0));
    unit_word = mcall(&mcall(&found.clone(), "group", &[py_str("unit")]), "lower", &[]);
    one = { let __b = Py::Bool(!py_truthy(&mcall(&unit_word.clone(), "endswith", &[py_str("s")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("km"), py_str("kg"), py_str("lb"), py_str("lbs")]), &unit_word.clone())) } };
    if py_truthy(&{ let __b = one.clone(); if !py_truthy(&__b) { __b } else { { let __b = next_word.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FIGURE_AFTER(), &next_word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&units.clone()) { let mut family: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&gen_memory::memory::store_lines::g_FIGURE_UNITS(), &family.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }, &next_word.clone())) } } } } } }) {
    continue;
    }
    pieces = py_list(vec![{ let __b = mcall(&found.clone(), "group", &[py_str("qual")]); if py_truthy(&__b) { __b } else { py_str("") } }, mcall(&found.clone(), "group", &[py_str("num")]), if py_truthy(&mcall(&found.clone(), "group", &[py_str("more")])) { op_add(&py_str("to "), &mcall(&found.clone(), "group", &[py_str("more")])) } else { py_str("") }, mcall(&found.clone(), "group", &[py_str("unit")]), { let __b = mcall(&found.clone(), "group", &[py_str("extra")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    said_figure = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&mcall(&piece.clone(), "strip", &[])) {  __out.push(mcall(&piece.clone(), "strip", &[])); } } py_list(__out) })() }]);
    if py_truthy(&{ let __b = stdlib_call("re", "match", &[py_str("(?:a|an|per)\\s+"), after.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&next_word.clone(), &py_str("per"))) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_ACTION_DURATION(), "search", &[_the_clause(&[said.clone(), mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "end", &[])])])) {
    continue;
    }
    definite = Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &mcall(&mcall(&found.clone(), "group", &[py_str("num")]), "lower", &[])));
    if py_truthy(&Py::Bool(!py_truthy(&definite.clone()))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![said_figure.clone(), definite.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn _the_figure_the_answer_names(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut rate_ask: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut units: Py = Py::None;
    let _ = py_str("The figure the answer on the screen named, on a line that asks only for a figure, else None.\n\n    \"how many weeks\" after \"Each week add a minute to the jog ... in about eight weeks you will run\n    20 minutes\" names no thing, so no entry is reached and the entry the topic does reach answers\n    the *unit* instead: the warm-up card, \"Five to ten minutes.\" Nobody asked that. The figure is\n    in the answer already on the screen, and `bot/memory/repeat.py` has read the line all along -\n    but its reply was only ever asked for under the composer's last-resort module, and an entry\n    that answers the wrong card wins the turn first.\n\n    The store is where the answer already said belongs, so the reading is claimed here. It is the\n    figure said back and not the sentence it is in, because the sentence is one the person has\n    just read.\n\n    Narrow on purpose: the line asks for one kind of figure and names nothing else, the answer is\n    a knowledge answer from a turn or two back, the figure is definite and of the kind asked for,\n    the unit is the noun of it, a rate (\"three runs a week\") is not a figure, and a length the\n    answer gives for doing something (\"for the first two days\", \"20 minutes at a time\") is a\n    duration of the action, not the length of the thing the person asked about.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    units = _the_unit_asked(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&units.clone()))) {
    return Py::None;
    }
    theirs = gen_memory::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    rate_ask = call_builtin("bool", &[mcall(&g_A_RATE_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&gen_memory::memory::store_lines::g_LOOK_BACK_FIGURE()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    back = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&gen_memory::memory::store_replies::g_ANSWER_MODULES(), &mcall(&turn.clone(), "get", &[py_str("module")])))) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &gen_memory::memory::store_lines::g_FIGURE_ANSWER_LEN()); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = rate_ask.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_RATE_STATED(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_LISTED_FIGURE(), "search", &[said.clone()])) {
    continue;
    }
    found = _the_figures_in(&[said.clone(), units.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    if py_truthy(&_a_figure_of_those_units(&[said.clone(), units.clone()])) {
    return Py::None;
    }
    continue;
    }
    stated = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let __unpk3705 = py_unpack(&__item.clone(), 2, None); let mut figure: Py = __unpk3705[0].clone(); let mut _u: Py = __unpk3705[1].clone(); __out.push(mcall(&figure.clone(), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[stated.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::None;
    }
    figure = geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(0));
    return op_add(&op_add(&mcall(&slice_or_raise(&figure.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&figure.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    return Py::None;
    Py::None
}
pub fn _a_figure_of_those_units(args: &[Py]) -> Py {
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
    let mut units: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "units") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: units"))) };
    let mut pattern: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the answer states a number with a unit of one of these families after it.\n\n    The loose reading, used only to tell \"this answer says a length and I cannot read which\"\n    from \"this answer says no length at all\": the second may hand the line to an older answer,\n    the first may not.\n    ");
    if py_truthy(&Py::Bool(py_contains(&units.clone(), &py_str("money")))) {
    return call_builtin("bool", &[{ let __b = mcall(&g_A_MONEY_FIGURE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_PERCENT_FIGURE(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&units.clone()) { let mut unit: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&gen_memory::memory::store_lines::g_FIGURE_UNITS(), &unit.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    unit = op_modulo(&py_str("(?-i:%s)"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    pattern = stdlib_call("re", "compile", &[op_modulo(&py_str("(?<![\\d,])(?:%s|%s)\\s*(?:-|to|or)?\\s*%s\\b"), &py_tuple(vec![gen_memory::memory::store_lines::g_FIGURE_NUMBER(), py_str("\\d[\\d,.]*"), unit.clone()])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[mcall(&pattern.clone(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_time_they_put_right(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut was: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("The reply to a line that does nothing but move a routine time, or None.\n\n    \"i get up at 6\" then \"i meant seven\" is one word put right, and the store has always kept it\n    (`times.keep`). But the reply to that line came from the composer's shrug (\"I'm\n    listening - say more if it helps.\") or, after \"make it 7\", from the search hedge: the bot had\n    just moved the time and said nothing about it. So the time is said back in the words the store\n    holds it in, and the one it replaced is named, because a correction is what the line is:\n    \"Got it - you're up at 7, not 6.\"\n    ");
    fixed = mcall(&memory.clone(), "get", &[py_str("fixed_time")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[fixed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("int", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { op_neg(&Py::Int(1)) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    at = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("at")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&at.clone()))) {
    return Py::None;
    }
    what = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("what")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    said_back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("at".to_string(), at.clone()), ("what".to_string(), what.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = mcall(&gen_memory::memory::store_lines::g_ROUTINE_SAY_BACK(), "get", &[what.clone()]); if py_truthy(&__b) { __b } else { py_str("You {what} at {at}.") } }, "format", args))(&__args) };
    said_back = mcall(&said_back.clone(), "rstrip", &[py_str(" .")]);
    was = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("from")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = was.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&was.clone(), &at.clone())) } }) {
    let __aug1 = op_add(&said_back.clone(), &op_add(&py_str(", not "), &was.clone()));
    said_back = __aug1;
    }
    return op_add(&op_add(&op_add(&py_str("Got it - "), &mcall(&geti(&said_back.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&said_back.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
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
    let mut ages_of_the_children: Py = Py::None;
    let mut along: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut ask_about_nothing: Py = Py::None;
    let mut attribute: Py = Py::None;
    let mut behind: Py = Py::None;
    let mut direct: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut framed: Py = Py::None;
    let mut going: Py = Py::None;
    let mut pointed_at_nothing: Py = Py::None;
    let mut possessive: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut recall_about_nothing: Py = Py::None;
    let mut recall_frame: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut ruled_out: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut told_about_the_trip: Py = Py::None;
    let _ = py_str("The store's reply, with the names in it written as the store holds them.\n\n    A plan is quoted back in the words they typed, so a name in it came back in their spelling:\n    \"we r going 2 spain in may\" -> \"You r going 2 spain in may.\". Every route out of the store\n    ends here, so the names are written once, at the end.\n\n    Then the floor under it: never the same words twice in a row. The store answers each line on\n    its own terms, so two different lines can be answered by the same words - \"is she vegan\" and\n    then \"what does my mum eat\" both reached the one line the store writes about a diet nobody\n    gave, and the person read the same sentence twice. Whatever route the reply came by,\n    `bot/memory/again.instead` frames it as the repeat it is - after the names are written, so\n    the words it compares are the words on the screen.\n    ");
    let mut same_again: Py = gen_memory::memory::again::module_obj();
    let mut not_this: Py = gen_memory::memory::not_this::module_obj();
    let mut trips: Py = gen_memory::memory::trips::module_obj();
    text = _without_the_shrug_at_the_end(&[text.clone()]);
    put_right = _the_time_they_put_right(&[memory.clone()]);
    if py_truthy(&put_right.clone()) {
    return put_right.clone();
    }
    pointed_at_nothing = _points_back_at_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&pointed_at_nothing.clone()) {
    return pointed_at_nothing.clone();
    }
    recall_frame = _a_recall_frame_about_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&recall_frame.clone()) {
    return recall_frame.clone();
    }
    ask_about_nothing = _an_ask_about_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&ask_about_nothing.clone()) {
    return ask_about_nothing.clone();
    }
    let mut _goals_progress: Py = gen_memory::memory::goals::module_obj();
    going = mcall(&_goals_progress.clone(), "a_progress_asked", &[text.clone(), memory.clone()]);
    if py_truthy(&going.clone()) {
    return going.clone();
    }
    recall_about_nothing = _a_recall_about_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&recall_about_nothing.clone()) {
    return recall_about_nothing.clone();
    }
    behind = _the_ask_behind_the_recall(&[text.clone()]);
    if py_truthy(&behind.clone()) {
    let __unpk1 = py_unpack(&behind.clone(), 3, None);
    direct = __unpk1[0].clone();
    attribute = __unpk1[1].clone();
    possessive = __unpk1[2].clone();
    answer = respond(&[direct.clone(), memory.clone()]);
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    if py_truthy(&_a_word_for_the_slot(&[memory.clone(), attribute.clone()])) {
    behind = Py::None;
    } else {
    theirs = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &possessive.clone()))) { py_str("your") } else { possessive.clone() };
    return py_call(&gen_memory::memory::store_replies::g_pick(), &[py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("You haven't told me "), &theirs.clone()), &py_str(" ")), &attribute.clone()), &py_str(".")), op_add(&op_add(&op_add(&op_add(&py_str("I don't have "), &theirs.clone()), &py_str(" ")), &attribute.clone()), &py_str(" from you yet.")), op_add(&op_add(&op_add(&op_add(&py_str("You haven't mentioned "), &theirs.clone()), &py_str(" ")), &attribute.clone()), &py_str(" to me."))]), memory.clone()]);
    }
    }
    ages_of_the_children = _the_ages_of_the_children(&[text.clone(), memory.clone()]);
    if py_truthy(&ages_of_the_children.clone()) {
    return ages_of_the_children.clone();
    }
    ruled_out = mcall(&not_this.clone(), "reply_for", &[text.clone(), memory.clone()]);
    if py_truthy(&ruled_out.clone()) {
    return ruled_out.clone();
    }
    told_about_the_trip = mcall(&trips.clone(), "reply_for", &[text.clone(), memory.clone()]);
    if py_truthy(&told_about_the_trip.clone()) {
    return told_about_the_trip.clone();
    }
    if py_truthy(&mcall(&gen_memory::memory::repeat::g_A_BARE_DURATION_ASK(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])) {
    theirs = gen_memory::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    if py_truthy(&mcall(&gen_memory::memory::repeat::g_A_PAY_WORD(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])) {
    theirs = gen_memory::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    if py_truthy(&gen_memory::memory::repeat::how_long_its_been(&[text.clone()])) {
    theirs = gen_memory::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    figure = _the_figure_the_answer_names(&[text.clone(), memory.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    along = gen_memory::memory::store_replies::_how_far_along(&[memory.clone(), gen_memory::memory::store_replies::_bare(&[py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])])]);
    if py_truthy(&along.clone()) {
    return along.clone();
    }
    theirs = gen_memory::memory::repeat::asked_back(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    reply = gen_memory::memory::store_replies::_the_store_reply(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return reply.clone();
    }
    reply = _the_places_are_written_as_names(&[reply.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&gen_memory::memory::store_replies::g_WHOLE_ASK(), "match", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_asks_to_hear_it_again(&[text.clone(), memory.clone()]))) } }) {
    framed = mcall(&same_again.clone(), "instead", &[reply.clone(), text.clone(), memory.clone()]);
    if py_truthy(&framed.clone()) {
    return framed.clone();
    }
    }
    return reply.clone();
    Py::None
}
pub fn _asks_to_hear_it_again(args: &[Py]) -> Py {
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
    let _ = py_str("True for the one turn where the same words are the answer: \"what did you say earlier\".\n\n    STYLE.md's rule is about answering twice, not about being asked twice: a person who asks to\n    hear the last answer again is handed it, and framing that as \"Same answer, and it covers\n    this:\" says the words were volunteered again rather than asked for.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), gen_memory::memory::store_replies::_repeat_ask(&[text.clone(), py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])])]);
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
pub fn _asks_about_someone_else(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True for a yes/no question about a person (\"is he allergic to peanuts\").\n\n    The question mark is not asked for. Most people type an ask about their son on a phone\n    without one, and \"is he allergic to peanuts\" filed peanuts on the son while the same line\n    with a \"?\" did not - the fault this guard exists for, in the one shape where a wrong stored\n    fact matters most. What makes the line an ask is the auxiliary at the front and a person\n    after it: \"is my son\", \"does he\", \"can they\" never open a statement. A word that is not a\n    person (\"is it expensive\", \"is the boiler new\") keeps the question mark it needs, because\n    those openings are also how a statement about the weather goes.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return call_builtin("bool", &[mcall(&gen_memory::memory::store_replies::g_YES_NO_ASK(), "match", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _note_a_corrected_fact(args: &[Py]) -> Py {
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
    let mut entry: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut was: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "was") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: was"))) };
    let mut before: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Remember that this turn put a fact about someone right, so the reply can say which one.\n\n    \"my son is 7\" then \"sorry, i meant my son is 8\" is a correction, and the store took it - it\n    had 7 and now has 8 - but nobody said so: the line was answered with \"Thanks for telling me -\n    I'm here if you want to dig into any of that\", which is the reply for news and tells the\n    person the correction did not land. Worse, \"sorry, i meant my son is 8\" carries the word\n    \"sorry\" and a relation, so it reached the row on how to word an apology and got advice about\n    apologising to him. A correction is answered with the fact, so this one is.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[was.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SAID_AGAIN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SORRY_OPENS_A_CORRECTION(), "search", &[said.clone()]) } }))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&entry.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("who"), py_str("pronoun"), py_str("turns")]), &kind.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str(""), Py::None]), &value.clone())) } }) {
    continue;
    }
    before = mcall(&was.clone(), "get", &[kind.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str(""), Py::None]), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&before.clone(), &value.clone())) } }) {
    continue;
    }
    let _ = py_setitem(&memory.clone(), &py_str("fact_fixed"), py_dict(vec![(py_str("who"), call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("kind"), kind.clone()), (py_str("value"), value.clone()), (py_str("was"), before.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return Py::None;
    }
    Py::None
}
pub fn _note_a_fact_moved(args: &[Py]) -> Py {
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
    let mut entry: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut was: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "was") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: was"))) };
    let mut hold: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Remember a correction that moves a fact from one person to another.\n\n    \"my son is 9\", then \"sorry i meant my daughter is 9\": the store filed the age on the\n    daughter, but the son kept his, so the chat ended up with two nine year olds and no word\n    said about it. The line also carries the word \"sorry\" and a relation, so it reached the row\n    on how to word an apology and the person who had just put a fact right was told to name what\n    they had done wrong. A correction that names another person takes the fact off the first one,\n    and the reply says which person it now belongs to.\n\n    Only the shape a correction is said in (\"i meant\", \"sorry\") counts, and only when a\n    different person already holds the same value: \"my daughter is 9 too\" is news about a second\n    child, not a correction, and it files both.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SAID_AGAIN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SORRY_OPENS_A_CORRECTION(), "search", &[said.clone()]) } }))) {
    return Py::None;
    }
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&entry.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("who"), py_str("pronoun"), py_str("turns")]), &kind.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str(""), Py::None]), &value.clone())) } }) {
    continue;
    }
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[was.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str(""), Py::None]), &mcall(&was.clone(), "get", &[kind.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&was.clone(), "get", &[kind.clone()]), &value.clone())) } } } }) {
    continue;
    }
    hold = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&one.clone(), "get", &[py_str("who")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&one.clone(), "get", &[kind.clone()]), &value.clone())) } } } } } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&hold.clone()))) {
    continue;
    }
    other = geti(&hold.clone(), &Py::Int(0));
    let _ = py_setitem(&memory.clone(), &py_str("fact_moved"), py_dict(vec![(py_str("who"), who.clone()), (py_str("was_who"), call_builtin("str", &[mcall(&other.clone(), "get", &[py_str("who")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("kind"), kind.clone()), (py_str("value"), value.clone()), (py_str("said"), said.clone())]));
    let _ = py_setitem(&memory.clone(), &py_str("moved_to_who"), said.clone());
    let _ = mcall(&other.clone(), "pop", &[kind.clone(), Py::None]);
    return Py::None;
    }
    Py::None
}
pub fn _a_word_for_the_slot(args: &[Py]) -> Py {
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
    let mut attribute: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "attribute") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: attribute"))) };
    let mut pattern: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether a line of theirs in this chat holds a word for this slot.");
    pattern = mcall(&g_SLOT_WORDS(), "get", &[mcall(&call_builtin("str", &[{ let __b = attribute.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&pattern.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&gen_memory::memory::store_replies::g_recent(), &[memory.clone(), Py::Int(8)])) { let mut turn: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ask_behind_the_recall(args: &[Py]) -> Py {
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
    let mut attribute: Py = Py::None;
    let mut name: Py = Py::None;
    let mut only_slot: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut person: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut possessive: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut slot: Py = Py::None;
    let mut slot_word: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let mut wrapped: Py = Py::None;
    let _ = py_str("The direct ask and the slot a recall ask is about, or None.\n\n    Only a slot the person could have given in words is read this way, and only with a possessive\n    in front of it (\"his name\", \"my budget\"): \"what did i say about the cake\" and \"what did i say\n    was wrong with it\" are not asks about a slot, and they keep their own readers.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[low.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    wrapped = mcall(&g_RECALL_WRAPPER(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wrapped.clone()))) {
    return Py::None;
    }
    rest = stdlib_call("re", "sub", &[py_str("\\s+(?:was|is|were|are)\\s*$"), py_str(""), mcall(&mcall(&wrapped.clone(), "group", &[py_str("rest")]), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&mcall(&mcall(&wrapped.clone(), "group", &[py_str("word")]), "lower", &[]), "replace", &[py_str("  "), py_str(" ")]);
    slot = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RECALL_SLOTS()) { let __unpk78 = py_unpack(&__item.clone(), 3, None); let mut pattern: Py = __unpk78[0].clone(); let mut word: Py = __unpk78[1].clone(); let mut name: Py = __unpk78[2].clone(); if py_truthy(&mcall(&pattern.clone(), "search", &[rest.clone()])) {  __out.push(py_tuple(vec![word.clone(), name.clone()])); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&slot.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&slot.clone(), 2, None);
    word = __unpk1[0].clone();
    attribute = __unpk1[1].clone();
    possessive = mcall(&g_RECALL_POSSESSIVE(), "match", &[rest.clone()]);
    if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("how old")])) {
    person = mcall(&mcall(&rest.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_SHE_OR_HE(), &person.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&geti(&g_SHE_OR_HE(), &person.clone()), 2, None);
    who = __unpk2[0].clone();
    verb = __unpk2[1].clone();
    return py_tuple(vec![op_add(&op_add(&py_str("how old "), &verb.clone()), &if py_truthy(&Py::Bool(py_eq(&who.clone(), &py_str("i")))) { py_str(" i") } else { op_add(&py_str(" "), &who.clone()) }), py_str("age"), person.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&possessive.clone()))) {
    return Py::None;
    }
    person = mcall(&mcall(&possessive.clone(), "group", &[py_str("poss")]), "lower", &[]);
    plain = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("\\b(?:was|is|were|are)\\b"), py_str(" "), rest.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    return Py::None;
    }
    word = if py_truthy(&Py::Bool(py_eq(&attribute.clone(), &py_str("birthday")))) { py_str("when is ") } else { py_str("what is ") };
    slot_word = mcall(&g_RECALL_SLOTS_BY_POSSESSIVE(), "match", &[plain.clone()]);
    only_slot = if py_truthy(&slot_word.clone()) { mcall(&slot_word.clone(), "group", &[py_str("attribute")]) } else { Py::None };
    return py_tuple(vec![op_add(&word.clone(), &plain.clone()), { let __b = only_slot.clone(); if py_truthy(&__b) { __b } else { { let __b = attribute.clone(); if py_truthy(&__b) { __b } else { py_str("answer") } } } }, person.clone()]);
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    else { Py::None };
    let _ = py_str("Claim every line the store can answer - the bare \"what do i do\" included.\n\n    The arbiter reads one `TAKES_AS_IT_COMES` per module and the last `def` in the file is the one\n    it reads, so a hook written above this one is not read at all: its line goes unclaimed, and the\n    specialist rule hands it to whatever entry the topic fits. That is what happened to the store's\n    own readings for a while (\"my son is 5\" then \"is my son 5\", \"he isnt allergic to nuts\", a plan\n    asked about by \"when\"), which is why every reading now has a private name and is asked from\n    here, at the end of the file, rather than from a hook of its own.\n\n    The job ask is the last reading: the bare \"what do i do\" said right after the person gave their\n    job is the fact, whatever else read. The line names nothing, so the understanding engine hands\n    it the topic of the turn before, and a knowledge entry for that trade answers it: an answer\n    about the entryways into nursing, given to someone who already is one and only asked what they\n    do. The store's reading is the one that answers what they asked, and it wins the turn on this\n    line alone. False everywhere else - a turn later the same words ask about whatever the chat has\n    moved on to.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&memory.clone(), "get", &[py_str("moved_to_who")]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])])))) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_store_readings(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::told::A_CALLBACK(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_ask_about_how_it_went(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_callback_that_names_its_thing(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gen_memory::memory::reminded::about_a_thing_they_said(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&gen_memory::memory::store_replies::g_BARE_WHAT_I_DO(), "match", &[gen_memory::memory::store_replies::_bare(&[{ let __b = py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { text.clone() } }])])))) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("job")])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { gen_memory::memory::store_replies::_the_job_was_just_said(&[memory.clone()]) } };
    Py::None
}
pub fn _about_the_verb(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut found: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut named: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The plan a when or what-time ask is about: the one that says its verb, and its name too.\n\n    Two plans can say the same verb - \"meeting sam at 6 tomorrow\" and \"meeting jo at 8 on friday\"\n    both say \"meeting\" - and the verb on its own answers \"what time am I meeting sam\" with jo's\n    hour, which is an answer about somebody the person did not ask about. The words the ask puts\n    after the verb are what tell the two plans apart, so a plan that names one of them wins.\n    When no plan names them the verb-only reading still stands: \"when do I start\" names nobody,\n    and \"what time do I start work\" keeps its answer even though the plan says \"job\".\n    ");
    named = py_call(&gen_memory::memory::store_replies::g_content(), &[slice_or_raise(&low.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]);
    if py_truthy(&named.clone()) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    plan = __item.clone();
    if py_truthy(&{ let __b = gen_memory::memory::store_replies::_says_the_verb(&[plan.clone(), mcall(&found.clone(), "group", &[Py::Int(1)])]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push(gen_memory::memory::store_replies::_names_the_thing(&[plan.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return plan.clone();
    }
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    plan = __item.clone();
    if py_truthy(&gen_memory::memory::store_replies::_says_the_verb(&[plan.clone(), mcall(&found.clone(), "group", &[Py::Int(1)])])) {
    return plan.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _remember_dislikes(args: &[Py]) -> Py {
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
    let mut named_here: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named_here") { v.clone() }
    else { py_tuple(vec![]) };
    let mut at: Py = Py::None;
    let mut food: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Keep a food someone else will not eat against that person: \"she hates mushrooms\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&gen_memory::memory::guest::found(&[text.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    food = __unpk2[0].clone();
    at = __unpk2[1].clone();
    let __unpk3 = py_unpack(&gen_memory::memory::facts::subject(&[text.clone(), at.clone()]), 2, None);
    kind = __unpk3[0].clone();
    who = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("user")))) {
    food = gen_memory::memory::guest::plain(&[food.clone()]);
    if py_truthy(&food.clone()) {
    let _ = py_call(&gen_memory::memory::store_replies::g_remember(), &[memory.clone(), py_str("dislikes"), py_list(vec![food.clone()]), text.clone()]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&kind.clone(), &py_str("other")))) {
    continue;
    }
    who = { let __b = who.clone(); if py_truthy(&__b) { __b } else { py_call(&gen_memory::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone(), named_here.clone()]) } };
    let _ = py_call(&gen_memory::memory::store_replies::g_remember_other(), &[memory.clone(), py_dict(vec![(py_str("who"), who.clone()), (py_str("dislikes"), py_list(vec![food.clone()]))]), text.clone()]);
    }
    Py::None
}
pub fn _diet_reply(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { py_str("") };
    let mut changed_from: Py = Py::None;
    let mut diet: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut left_out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_so: Py = Py::None;
    let mut take_back: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut was: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("How they eat, said back when they ask: \"You told me you're vegetarian.\"\n\n    The diet is kept on every line they say it in, so the answer is the fact and nothing else.\n    A diet stored as a negation comes back with its verb (\"you don't eat meat\"), because \"you're\n    no meat\" says nothing. What they cannot eat is not one key, though: \"im coeliac\" is a\n    condition and \"im allergic to nuts\" is an allergy, and a question about the fact is about\n    all of it - so the read says back whichever the store holds, and only when the line asks\n    about the fact (a line asking for a meal keeps its meal).\n    ");
    said = gen_memory::memory::change::reported(&[low.clone(), py_str("diet")]);
    if py_truthy(&said.clone()) {
    was = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("diet")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&said.clone(), &py_str("changed"))); if !py_truthy(&__b) { __b } else { was.clone() } }) {
    let _ = py_call(&gen_memory::memory::store_replies::g_forget(), &[memory.clone(), py_str("diet"), py_list(vec![was.clone()])]);
    let _ = py_setitem(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), &py_str("diet_changed"), was.clone());
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_str("diet")); __kws.extend(vec![("was".to_string(), was.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::change::reply(args))(&__args) };
    }
    diet = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("diet")]);
    if py_truthy(&Py::Bool(!py_truthy(&diet.clone()))) {
    if py_truthy(&{ let __b = call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))); if !py_truthy(&__b) { __b } else { py_call(&g__food_facts(), &[memory.clone()]) } } } }) {
    let __unpk1 = py_unpack(&py_call(&g__what_a_dinner_has_to_leave_out(), &[memory.clone()]), 2, None);
    left_out = __unpk1[0].clone();
    said_so = __unpk1[1].clone();
    if py_truthy(&left_out.clone()) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Nothing with "), &py_call(&g__say_a_list(), &[left_out.clone()])), &py_str(" in it — ")), &mcall(&py_str(" and "), "join", &[said_so.clone()])), &py_str(". Tell me what is in the fridge and I'll pick a dinner around it."));
    }
    }
    kept = py_call(&g__food_facts(), &[memory.clone()]);
    if py_truthy(&kept.clone()) {
    return op_add(&op_add(&py_str("You told me you're "), &py_call(&g__say_a_list(), &[kept.clone()])), &py_str("."));
    }
    if py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    kept = py_call(&g__food_facts(), &[memory.clone()]);
    if py_truthy(&kept.clone()) {
    return op_add(&op_add(&py_str("You told me you're "), &py_call(&g__say_a_list(), &[kept.clone()])), &py_str("."));
    }
    }
    let __unpk2 = py_unpack(&py_call(&g__other_fact(), &[memory.clone(), py_str("diet")]), 2, None);
    who = __unpk2[0].clone();
    theirs = __unpk2[1].clone();
    if py_truthy(&{ let __b = theirs.clone(); if !py_truthy(&__b) { __b } else { who.clone() } }) {
    return op_add(&op_add(&py_str("You haven't told me about yours — the diet you mentioned is your "), &who.clone()), &py_str("'s."));
    }
    changed_from = mcall(&py_call(&gen_memory::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("diet_changed")]);
    if py_truthy(&{ let __b = changed_from.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    was = mcall(&call_builtin("str", &[changed_from.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    was = if py_truthy(&mcall(&mcall(&was.clone(), "lower", &[]), "startswith", &[py_str("no ")])) { op_add(&py_str("not eating "), &slice_or_raise(&was.clone(), &Py::Int(3), &Py::None, &Py::None)) } else { op_add(&py_str("being "), &py_call(&g__diet_words(), &[was.clone()])) };
    return op_add(&op_add(&py_str("You told me your diet had changed from "), &was.clone()), &py_str(". What is it now?"));
    }
    let __unpk3 = py_unpack(&py_call(&g__the_take_back(), &[memory.clone(), py_str("diet")]), 2, None);
    take_back = __unpk3[0].clone();
    gone = __unpk3[1].clone();
    if py_truthy(&gone.clone()) {
    return py_call(&g__a_take_back_said_back(), &[take_back.clone(), gone.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_MEAL_WANTED(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_str("diet")); __args.push(py_str("You haven't told me about your diet.")); __kws.extend(vec![("own".to_string(), g__their_own_diet())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&g__deny(), args))(&__args) };
    }
    diet = call_builtin("str", &[diet.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_MEAL_WANTED(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    let __unpk4 = py_unpack(&py_call(&g__what_a_dinner_has_to_leave_out(), &[memory.clone()]), 2, None);
    left_out = __unpk4[0].clone();
    said_so = __unpk4[1].clone();
    if py_truthy(&left_out.clone()) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Nothing with "), &py_call(&g__say_a_list(), &[left_out.clone()])), &py_str(" in it — ")), &mcall(&py_str(" and "), "join", &[said_so.clone()])), &py_str(". Tell me what is in the fridge and I'll pick a dinner around it."));
    }
    return Py::None;
    }
    if py_truthy(&mcall(&diet.clone(), "startswith", &[py_str("no ")])) {
    return op_add(&op_add(&op_add(&op_add(&py_str("You told me you don't "), &py_call(&g__diet_how(), &[diet.clone()])), &py_str(" ")), &slice_or_raise(&diet.clone(), &Py::Int(3), &Py::None, &Py::None)), &py_str("."));
    }
    if py_truthy(&Py::Bool(py_contains(&g_WAY_OF_EATING(), &diet.clone()))) {
    return op_add(&op_add(&py_str("You told me you're "), &py_call(&g__diet_words(), &[diet.clone()])), &py_str("."));
    }
    if py_truthy(&Py::Bool(py_contains(&g_PLAN_DIETS(), &diet.clone()))) {
    return op_add(&op_add(&py_str("You told me you're doing "), &py_call(&g__diet_words(), &[diet.clone()])), &py_str("."));
    }
    return op_add(&op_add(&py_str("You told me you're "), &diet.clone()), &py_str("."));
    Py::None
}
pub fn _a_fact_word(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing a callback names is one of the facts the store keeps.");
    after = stdlib_call("re", "search", &[py_str("\\babout\\b\\s+(.+)$"), { let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&if py_truthy(&after.clone()) { mcall(&after.clone(), "group", &[Py::Int(1)]) } else { py_str("") }, "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[op_bitand(&words.clone(), &g_FACT_WORDS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_callback_that_names_its_thing(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the line asks about the conversation and names the thing it asks about.\n\n    \"what have i been saying about the boiler\" is a recap by name (`_about_this_chat`, the reading\n    the store answers with) and the name is what makes it about that thing rather than the chat as\n    a whole (`_names_its_thing`). Both, because either on its own is too wide for a claim: a line\n    that names nothing is the chat's own topic, and a line that names a boiler is not a recap\n    unless the store read it as one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = gen_memory::memory::store_replies::_names_its_thing(&[low.clone()]); if !py_truthy(&__b) { __b } else { gen_memory::memory::store_replies::_about_this_chat(&[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _points_back_at_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("The store's answer to a pointer with nothing behind it, or None when it is not that line.\n\n    \"same as before\" as the first thing in a chat is not a request the store can answer from what\n    it holds, and it is not small talk either: it is the person picking up a conversation that is\n    not there. Saying that plainly is the answer, and it is the same answer NOTHING_STARTED gives\n    to \"what have we talked about\" - so a chat that opens this way gets a line that names the\n    missing thing and asks for it, not a welcome and not a receipt.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gen_memory::memory::thread::points_back(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::thread::nothing_said(&[memory.clone()]))) } }) {
    return Py::None;
    }
    return gen_memory::memory::store_replies::_vary(&[gen_memory::memory::store_lines::g_POINTS_BACK_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _a_recall_about_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("The store's answer to \"what was it i was worried about\" with nothing said, or None.\n\n    The line names no slot and no fact: it asks the chat to give back something that was never\n    put in it. The readers below all want a thing to look for, so none of them reads it, and it\n    fell through to the chatter band as a line about a worry.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gen_memory::memory::thread::asks_what_i_said(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::thread::nothing_said(&[memory.clone()]))) } }) {
    return Py::None;
    }
    return gen_memory::memory::store_replies::_vary(&[gen_memory::memory::store_lines::g_WHAT_I_SAID_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _names_nothing_in_a_recall(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when what follows a recall ask names no thing of its own.");
    words = stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&mcall(&call_builtin("str", &[{ let __b = rest.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_RECALL_NAMES_NOTHING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _a_recall_frame_about_nothing(args: &[Py]) -> Py {
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
    let mut another: Py = Py::None;
    let mut low: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut wrapped: Py = Py::None;
    let mut wrapped_low: Py = Py::None;
    let _ = py_str("The store's answer to \"what did i say about it\" with nothing said, or None.\n\n    The recall frame - \"what did i say\", \"what did i say about that\", \"what did i say i lost\" -\n    is the ask for their own words, and it is that whatever the frame is filled with. The \"it\"\n    or the \"that\" inside it is not a thing the chat could name: the person is asking what they\n    themselves said, so there is nothing to give them, and saying that is the answer.\n\n    Read before the pointer asks, because the pointer reader looked at the same \"it\" and answered\n    a request for the chat's own words as if the person had asked a question about a subject\n    (\"Nothing said yet for that to be: tell me the thing itself...\"). A rule-out, not a new pool:\n    it hands back the same WHAT_I_SAID_NOTHING line the feelings recall gets.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&gen_memory::memory::thread::nothing_said(&[memory.clone()])))) {
    return Py::None;
    }
    low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    wrapped_low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    another = mcall(&g_A_RECALL_IN_ANOTHER_SHAPE(), "match", &[wrapped_low.clone()]);
    if py_truthy(&{ let __b = another.clone(); if !py_truthy(&__b) { __b } else { _names_nothing_in_a_recall(&[slice_or_raise(&wrapped_low.clone(), &mcall(&another.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    return gen_memory::memory::store_replies::_vary(&[gen_memory::memory::store_lines::g_WHAT_I_SAID_NOTHING(), memory.clone()]);
    }
    wrapped = mcall(&g_RECALL_WRAPPER(), "match", &[wrapped_low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wrapped.clone()))) {
    return Py::None;
    }
    rest = mcall(&wrapped.clone(), "group", &[py_str("rest")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&gen_memory::memory::thread::g_POINTING_WORD(), "search", &[rest.clone()]))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[a-z]+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    return gen_memory::memory::store_replies::_vary(&[gen_memory::memory::store_lines::g_WHAT_I_SAID_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _an_ask_about_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("The store's answer to an ask about nothing at all, or None when it is not that line.\n\n    \"how do i do it\" as the first line of a chat names one thing - the \"it\" - and the chat holds\n    nothing it can be. Nothing below reads the line: no fact is asked for, no plan is stated, so\n    the turn went to a search hedge over a question the person could be helped with in one line.\n    The same goes for \"do that again\" and \"is it worth it\". The answer says the thing is missing\n    and asks for it, which is what a friend does with a question whose subject they never heard.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gen_memory::memory::thread::points_at_a_word_alone(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::thread::nothing_said(&[memory.clone()]))) } }) {
    return Py::None;
    }
    let mut fallback: Py = gen_compose::compose::fallback::module_obj();
    if py_truthy(&mcall(&fallback.clone(), "_points_at_nothing", &[text.clone(), memory.clone()])) {
    return Py::None;
    }
    return gen_memory::memory::store_replies::_vary(&[gen_memory::memory::store_lines::g_POINTING_WORD_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _the_last_ask_about_the_bot(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The last thing they asked about the bot, as a sentence, when that is all they have said.\n\n    A chat that opens \"are you a real person\" and then asks what we have talked about has nothing\n    of theirs to go back over - but \"nothing yet\" is not true either, because they asked something.\n    The question is said back in their own words (\"whether I am a real person\") as the one thing\n    the chat has covered, which is what a friend would say.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[gen_memory::memory::thread::user_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    text = __unpk2[0].clone();
    reading = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&reading.clone(), "get", &[py_str("about_bot")]) } }))) {
    return Py::None;
    }
    asked = gen_memory::memory::thread::restate(&[text.clone()]);
    if py_truthy(&asked.clone()) {
    return asked.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _fits(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut room: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "room") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: room"))) };
    let mut best: Py = Py::None;
    let mut best_seconds: Py = Py::None;
    let mut names: Py = Py::None;
    let mut part: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut took: Py = Py::None;
    let _ = py_str("The move of an answer whose own length is the largest one that still fits `room`.\n\n    The largest that fits, not the shortest: somebody with 20 minutes wants the twenty-minute way\n    of doing it, not the two-minute half of a paragraph that is about something else.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, op_neg(&Py::Int(1))]), 2, None);
    best = __unpk1[0].clone();
    best_seconds = __unpk1[1].clone();
    names = gen_memory::memory::store_replies::_names_minutes(&[answer.clone()]);
    let __iter2 = py_iter_or_raise(&gen_memory::memory::store_replies::_sentences(&[answer.clone()]));
    for __item in __iter2 {
    sentence = __item.clone();
    let __iter3 = py_iter_or_raise(&gen_memory::memory::store_replies::_halves(&[sentence.clone()]));
    for __item in __iter3 {
    part = __item.clone();
    took = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __kws.extend(vec![("bare".to_string(), names.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::store_replies::_longest(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&took.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = op_gt(&took.clone(), &room.clone()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::store_replies::_says_something(&[part.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&op_gt(&took.clone(), &best_seconds.clone())) {
    let __unpk4 = py_unpack(&py_tuple(vec![part.clone(), took.clone()]), 2, None);
    best = __unpk4[0].clone();
    best_seconds = __unpk4[1].clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _asks_whether_we_followed(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the whole line asks whether the last thing they said was taken in.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = gen_memory::memory::store_replies::_plain_line(&[if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { py_call(&gen_memory::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]) } else { text.clone() }]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&low.clone(), "endswith", &[py_str("?")])) {
    low = mcall(&slice_or_raise(&low.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), "strip", &[]);
    }
    if py_truthy(&gen_memory::memory::store_replies::_reads_as_the_invitation(&[low.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&gen_memory::memory::store_replies::g_CHECK_FOLLOWED(), "match", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_group_size_ask(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("Whether this is a question about how many the person's own group is, and the number is kept.\n\n    The ask has to be about the group itself - \"how many of us\", \"how many people are there\",\n    \"how big is my family\" - and never about how many are going somewhere (\"how many of us are\n    going to the park\"), which `counts.GROUP_ASK` already tells apart. Nothing is claimed unless\n    the store actually holds a size, so a \"how many people\" with nothing said before it keeps\n    whatever answer the line has of its own.\n    ");
    low = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&gen_memory::memory::counts::g_GROUP_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&gen_memory::memory::counts::g_BARE_COUNT_ASK(), "search", &[low.clone()]) } }))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[gen_memory::memory::counts::answer(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.memory_store".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_PERSON_ASK" => Some(g_ABOUT_PERSON_ASK()),
            "ACKS" => Some(g_ACKS()),
            "ACTION_AMOUNT" => Some(g_ACTION_AMOUNT()),
            "ACTION_DURATION" => Some(g_ACTION_DURATION()),
            "ACTIVITY_ASK" => Some(g_ACTIVITY_ASK()),
            "ADVICE_MODULES" => Some(g_ADVICE_MODULES()),
            "AGES_IN_WORDS" => Some(g_AGES_IN_WORDS()),
            "AGE_ADVERB" => Some(g_AGE_ADVERB()),
            "AGE_AGAIN" => Some(g_AGE_AGAIN()),
            "AGE_ASK" => Some(g_AGE_ASK()),
            "AGE_AS_A_CAUSE" => Some(g_AGE_AS_A_CAUSE()),
            "AGE_COPULA" => Some(g_AGE_COPULA()),
            "AGE_DETERMINER" => Some(g_AGE_DETERMINER()),
            "AGE_FIX" => Some(g_AGE_FIX()),
            "AGE_FIX_WORDS" => Some(g_AGE_FIX_WORDS()),
            "AGE_IN_WORDS" => Some(g_AGE_IN_WORDS()),
            "AGE_PERSON_WORD" => Some(g_AGE_PERSON_WORD()),
            "AGE_RULE_WORD" => Some(g_AGE_RULE_WORD()),
            "AGE_SAID" => Some(g_AGE_SAID()),
            "AGE_WORD" => Some(g_AGE_WORD()),
            "AGE_YOU_SAY" => Some(g_AGE_YOU_SAY()),
            "AMOUNT" => Some(g_AMOUNT()),
            "AMOUNT_WORD" => Some(g_AMOUNT_WORD()),
            "ANOTHER_ANIMAL" => Some(g_ANOTHER_ANIMAL()),
            "ANOTHER_PERSON" => Some(g_ANOTHER_PERSON()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ANSWER_MODULES" => Some(g_ANSWER_MODULES()),
            "ANY_FIGURE" => Some(g_ANY_FIGURE()),
            "ANY_NUMBER" => Some(g_ANY_NUMBER()),
            "ASKED_LEADS" => Some(g_ASKED_LEADS()),
            "ASKS_ABOUT_A_TASTE" => Some(g_ASKS_ABOUT_A_TASTE()),
            "ASKS_FOR_TASTES" => Some(g_ASKS_FOR_TASTES()),
            "ASK_ABOUT_THEM" => Some(g_ASK_ABOUT_THEM()),
            "ASK_OPENERS" => Some(g_ASK_OPENERS()),
            "ASK_START" => Some(g_ASK_START()),
            "AT_TIME" => Some(g_AT_TIME()),
            "AT_TIME_VALUE" => Some(g_AT_TIME_VALUE()),
            "A_DAY_BEFORE_A_MONTH" => Some(g_A_DAY_BEFORE_A_MONTH()),
            "A_DAY_SAID_LONG" => Some(g_A_DAY_SAID_LONG()),
            "A_HAVE_QUESTION" => Some(g_A_HAVE_QUESTION()),
            "A_MONEY_FIGURE" => Some(g_A_MONEY_FIGURE()),
            "A_NAME_AFTER_A_PRONOUN" => Some(g_A_NAME_AFTER_A_PRONOUN()),
            "A_NAME_WITH_ITS_VERB" => Some(g_A_NAME_WITH_ITS_VERB()),
            "A_PERCENT_FIGURE" => Some(g_A_PERCENT_FIGURE()),
            "A_QUESTION_ASKED" => Some(g_A_QUESTION_ASKED()),
            "A_RATE_ASK" => Some(g_A_RATE_ASK()),
            "A_RATE_STATED" => Some(g_A_RATE_STATED()),
            "A_RECALL_IN_ANOTHER_SHAPE" => Some(g_A_RECALL_IN_ANOTHER_SHAPE()),
            "A_SHORT_ANSWER" => Some(g_A_SHORT_ANSWER()),
            "A_UNIT_SAID" => Some(g_A_UNIT_SAID()),
            "BAD_NEWS" => Some(g_BAD_NEWS()),
            "BARE_ECHO_ONLY" => Some(g_BARE_ECHO_ONLY()),
            "BARE_HOUR" => Some(g_BARE_HOUR()),
            "BARE_NEW_COUNT" => Some(g_BARE_NEW_COUNT()),
            "BARE_SLOTS" => Some(g_BARE_SLOTS()),
            "BARE_WHAT_I_DO" => Some(g_BARE_WHAT_I_DO()),
            "BREED_ASK" => Some(g_BREED_ASK()),
            "BUDGET_FITS" => Some(g_BUDGET_FITS()),
            "BUDGET_MORE" => Some(g_BUDGET_MORE()),
            "BUDGET_TIGHT" => Some(g_BUDGET_TIGHT()),
            "CALENDAR_SAID" => Some(g_CALENDAR_SAID()),
            "CALL_LEADS" => Some(g_CALL_LEADS()),
            "CANT_HAVE" => Some(g_CANT_HAVE()),
            "CAN_HAVE" => Some(g_CAN_HAVE()),
            "CAN_THEY_HAVE" => Some(g_CAN_THEY_HAVE()),
            "CAN_THEY_HAVE_A_FOOD" => Some(g_CAN_THEY_HAVE_A_FOOD()),
            "CAR_CAPS" => Some(g_CAR_CAPS()),
            "CAT_NAMES" => Some(g_CAT_NAMES()),
            "CHANGE_HEAVY" => Some(g_CHANGE_HEAVY()),
            "CHANGE_PLAIN" => Some(g_CHANGE_PLAIN()),
            "CHAT_TAIL" => Some(g_CHAT_TAIL()),
            "CHECK_A_FEELING" => Some(g_CHECK_A_FEELING()),
            "CHECK_A_QUESTION" => Some(g_CHECK_A_QUESTION()),
            "CHECK_FOLLOWED" => Some(g_CHECK_FOLLOWED()),
            "CHECK_NOTHING_SAID" => Some(g_CHECK_NOTHING_SAID()),
            "CHECK_PLAIN" => Some(g_CHECK_PLAIN()),
            "CHILD" => Some(g_CHILD()),
            "CHILD_OF" => Some(g_CHILD_OF()),
            "CHILD_RELATIONS" => Some(g_CHILD_RELATIONS()),
            "CHILD_WORD" => Some(g_CHILD_WORD()),
            "CLOCK_SAID" => Some(g_CLOCK_SAID()),
            "CLOSE_HEAVY" => Some(g_CLOSE_HEAVY()),
            "CLOSE_PLAIN" => Some(g_CLOSE_PLAIN()),
            "CONCEPTS" => Some(g_CONCEPTS()),
            "CONDITION_KEEPS_DOWN" => Some(g_CONDITION_KEEPS_DOWN()),
            "CONDITION_RULES_OUT" => Some(g_CONDITION_RULES_OUT()),
            "CORRECTABLE" => Some(g_CORRECTABLE()),
            "CORRECTED_AGAIN" => Some(g_CORRECTED_AGAIN()),
            "CORRECTED_AGAIN_WHO" => Some(g_CORRECTED_AGAIN_WHO()),
            "CORRECTED_SAY" => Some(g_CORRECTED_SAY()),
            "CORRECTED_WHICH_PERSON" => Some(g_CORRECTED_WHICH_PERSON()),
            "CORRECTING" => Some(g_CORRECTING()),
            "CORRECTION_BACK" => Some(g_CORRECTION_BACK()),
            "CORRECTION_SAY" => Some(g_CORRECTION_SAY()),
            "CORRECTION_SAY_DROP" => Some(g_CORRECTION_SAY_DROP()),
            "CORRECTION_SAY_PAIR" => Some(g_CORRECTION_SAY_PAIR()),
            "CORRECTS_A_MAKE" => Some(g_CORRECTS_A_MAKE()),
            "CORRECTS_A_NAME" => Some(g_CORRECTS_A_NAME()),
            "CORRECTS_A_NAME_POSSESSIVE" => Some(g_CORRECTS_A_NAME_POSSESSIVE()),
            "COUNT_ANIMAL" => Some(g_COUNT_ANIMAL()),
            "COUNT_BARE" => Some(g_COUNT_BARE()),
            "COUNT_SAID" => Some(g_COUNT_SAID()),
            "COUNT_WORD" => Some(g_COUNT_WORD()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "CURRENCY" => Some(g_CURRENCY()),
            "DATE_DAY" => Some(g_DATE_DAY()),
            "DATE_PHRASE" => Some(g_DATE_PHRASE()),
            "DAY_ASKS" => Some(g_DAY_ASKS()),
            "DAY_OF_MONTH" => Some(g_DAY_OF_MONTH()),
            "DAY_WORD" => Some(g_DAY_WORD()),
            "DENIAL_SAY" => Some(g_DENIAL_SAY()),
            "DENIAL_STEMS" => Some(g_DENIAL_STEMS()),
            "DENIAL_WORDS" => Some(g_DENIAL_WORDS()),
            "DIET_FOOD" => Some(g_DIET_FOOD()),
            "DIET_RULES_OUT" => Some(g_DIET_RULES_OUT()),
            "DOG_NAMES" => Some(g_DOG_NAMES()),
            "DROP_LISTS" => Some(g_DROP_LISTS()),
            "EAT_ASK" => Some(g_EAT_ASK()),
            "ELIDED_AGAIN" => Some(g_ELIDED_AGAIN()),
            "ELIDED_ASK" => Some(g_ELIDED_ASK()),
            "ELIDED_NAMES" => Some(g_ELIDED_NAMES()),
            "ELIDED_NOT_SURE" => Some(g_ELIDED_NOT_SURE()),
            "ELIDED_WORDS" => Some(g_ELIDED_WORDS()),
            "FACT_FAMILY" => Some(g_FACT_FAMILY()),
            "FACT_WORDS" => Some(g_FACT_WORDS()),
            "FAMILY_GROUP" => Some(g_FAMILY_GROUP()),
            "FEEDING_SAY" => Some(g_FEEDING_SAY()),
            "FIGURE_AFTER" => Some(g_FIGURE_AFTER()),
            "FIGURE_ANSWER_LEN" => Some(g_FIGURE_ANSWER_LEN()),
            "FIGURE_ASK" => Some(g_FIGURE_ASK()),
            "FIGURE_FILLER" => Some(g_FIGURE_FILLER()),
            "FIGURE_MONEY" => Some(g_FIGURE_MONEY()),
            "FIGURE_NUMBER" => Some(g_FIGURE_NUMBER()),
            "FIGURE_QUALIFIER" => Some(g_FIGURE_QUALIFIER()),
            "FIGURE_TIME" => Some(g_FIGURE_TIME()),
            "FIGURE_UNITS" => Some(g_FIGURE_UNITS()),
            "FIRST_ASK" => Some(g_FIRST_ASK()),
            "FOOD_PHRASES" => Some(g_FOOD_PHRASES()),
            "GREET" => Some(g_GREET()),
            "GRIEF_WORDS" => Some(g_GRIEF_WORDS()),
            "GROUP_ASK" => Some(g_GROUP_ASK()),
            "GROUP_NAMES_TYPED" => Some(g_GROUP_NAMES_TYPED()),
            "GROUP_NAME_ASK" => Some(g_GROUP_NAME_ASK()),
            "GROUP_OF" => Some(g_GROUP_OF()),
            "GROUP_WORDS" => Some(g_GROUP_WORDS()),
            "HAVE_ANY" => Some(g_HAVE_ANY()),
            "HAVE_ANY_WORDS" => Some(g_HAVE_ANY_WORDS()),
            "HELD_THREAD" => Some(g_HELD_THREAD()),
            "HER_WORD" => Some(g_HER_WORD()),
            "HOW_MANY_OF_MINE" => Some(g_HOW_MANY_OF_MINE()),
            "IN_LAW_AFTER_TEXT" => Some(g_IN_LAW_AFTER_TEXT()),
            "KEPT_THINGS" => Some(g_KEPT_THINGS()),
            "KEY_SLOT" => Some(g_KEY_SLOT()),
            "KIN" => Some(g_KIN()),
            "KIN_ASK" => Some(g_KIN_ASK()),
            "LENGTH" => Some(g_LENGTH()),
            "LIMITS" => Some(g_LIMITS()),
            "LIMIT_SAY" => Some(g_LIMIT_SAY()),
            "LISTED_FIGURE" => Some(g_LISTED_FIGURE()),
            "LOOK_BACK_FIGURE" => Some(g_LOOK_BACK_FIGURE()),
            "MADE_OF_A_VALUE" => Some(g_MADE_OF_A_VALUE()),
            "MEMORY_POINTER" => Some(g_MEMORY_POINTER()),
            "MISSED_CATCH_UP" => Some(g_MISSED_CATCH_UP()),
            "MONTH_ALT" => Some(g_MONTH_ALT()),
            "MONTH_ORDER" => Some(g_MONTH_ORDER()),
            "MONTH_WORD" => Some(g_MONTH_WORD()),
            "MONTH_WORDS" => Some(g_MONTH_WORDS()),
            "NAMED_BY" => Some(g_NAMED_BY()),
            "NAMED_FACT_ASK" => Some(g_NAMED_FACT_ASK()),
            "NAME_ADVICE" => Some(g_NAME_ADVICE()),
            "NAME_AGAIN" => Some(g_NAME_AGAIN()),
            "NAME_FIX" => Some(g_NAME_FIX()),
            "NAME_FIX_WORDS" => Some(g_NAME_FIX_WORDS()),
            "NAME_GIVEN" => Some(g_NAME_GIVEN()),
            "NAME_GIVEN_WORDS" => Some(g_NAME_GIVEN_WORDS()),
            "NAME_NOT_A_NAME" => Some(g_NAME_NOT_A_NAME()),
            "NAMING_VERB" => Some(g_NAMING_VERB()),
            "NOTHING_STARTED" => Some(g_NOTHING_STARTED()),
            "NOT_A_FACT" => Some(g_NOT_A_FACT()),
            "NOT_A_FACT_OF_THEIRS" => Some(g_NOT_A_FACT_OF_THEIRS()),
            "NOT_A_NAME_AFTER_CALLED" => Some(g_NOT_A_NAME_AFTER_CALLED()),
            "NOT_A_NAME_BEFORE" => Some(g_NOT_A_NAME_BEFORE()),
            "NOT_A_NEW_VALUE" => Some(g_NOT_A_NEW_VALUE()),
            "NOT_A_PLACE_VALUE" => Some(g_NOT_A_PLACE_VALUE()),
            "NOT_A_UNIT" => Some(g_NOT_A_UNIT()),
            "NOT_FROM_THE_REPAIR" => Some(g_NOT_FROM_THE_REPAIR()),
            "NOT_ON_A_MEDIUM" => Some(g_NOT_ON_A_MEDIUM()),
            "NOT_RETRACTED" => Some(g_NOT_RETRACTED()),
            "NOT_THE_PLAN" => Some(g_NOT_THE_PLAN()),
            "NO_PLANS_REPLY" => Some(g_NO_PLANS_REPLY()),
            "NO_RECORD" => Some(g_NO_RECORD()),
            "NO_SUBJECT_OF_ITS_OWN" => Some(g_NO_SUBJECT_OF_ITS_OWN()),
            "NUMBER" => Some(g_NUMBER()),
            "NUMBER_DIGITS" => Some(g_NUMBER_DIGITS()),
            "NUMBER_VALUE" => Some(g_NUMBER_VALUE()),
            "NUMBER_WORD" => Some(g_NUMBER_WORD()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "OTHER_BARE_NUMBER" => Some(g_OTHER_BARE_NUMBER()),
            "OTHER_FACT" => Some(g_OTHER_FACT()),
            "OTHER_KIND_SAY" => Some(g_OTHER_KIND_SAY()),
            "OTHER_NOUNS" => Some(g_OTHER_NOUNS()),
            "OTHER_PLACE" => Some(g_OTHER_PLACE()),
            "OTHER_PLACE_PRONOUN" => Some(g_OTHER_PLACE_PRONOUN()),
            "OTHER_SAID" => Some(g_OTHER_SAID()),
            "OWNED_BY_THE_VERB" => Some(g_OWNED_BY_THE_VERB()),
            "PAIR_WITHOUT_A_MARKER" => Some(g_PAIR_WITHOUT_A_MARKER()),
            "PART_HELD" => Some(g_PART_HELD()),
            "PART_KEYS" => Some(g_PART_KEYS()),
            "PERSON_ADVICE_ASK" => Some(g_PERSON_ADVICE_ASK()),
            "PERSON_AFTER" => Some(g_PERSON_AFTER()),
            "PERSON_GROUP" => Some(g_PERSON_GROUP()),
            "PET_AGE_ANSWER" => Some(g_PET_AGE_ANSWER()),
            "PET_AGE_FIX_WORDS" => Some(g_PET_AGE_FIX_WORDS()),
            "PET_ASK" => Some(g_PET_ASK()),
            "PET_LEAD_IN" => Some(g_PET_LEAD_IN()),
            "PET_NAME_AGE" => Some(g_PET_NAME_AGE()),
            "PET_RENAMED_SAY" => Some(g_PET_RENAMED_SAY()),
            "PET_WORDS" => Some(g_PET_WORDS()),
            "PLACES" => Some(g_PLACES()),
            "PLACE_ENDS" => Some(g_PLACE_ENDS()),
            "PLACE_NOT" => Some(g_PLACE_NOT()),
            "PLAN_ASKS" => Some(g_PLAN_ASKS()),
            "PLURAL_ANIMAL" => Some(g_PLURAL_ANIMAL()),
            "PLURAL_PEOPLE" => Some(g_PLURAL_PEOPLE()),
            "PLURAL_PETS" => Some(g_PLURAL_PETS()),
            "POINTING_WORD_NOTHING" => Some(g_POINTING_WORD_NOTHING()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "POINTS_BACK_AT_IT" => Some(g_POINTS_BACK_AT_IT()),
            "POINTS_BACK_NOTHING" => Some(g_POINTS_BACK_NOTHING()),
            "POSSESSIVE" => Some(g_POSSESSIVE()),
            "PREGNANCY_SAID_AGAIN" => Some(g_PREGNANCY_SAID_AGAIN()),
            "PREGNANT_OWN_SAY" => Some(g_PREGNANT_OWN_SAY()),
            "PREGNANT_SAY" => Some(g_PREGNANT_SAY()),
            "PREGNANT_WEEKS_SAY" => Some(g_PREGNANT_WEEKS_SAY()),
            "PRIORITY" => Some(g_PRIORITY()),
            "PRONOUN_FITS" => Some(g_PRONOUN_FITS()),
            "PRONOUN_PEOPLE" => Some(g_PRONOUN_PEOPLE()),
            "QUOTE_LEADS" => Some(g_QUOTE_LEADS()),
            "RECALL_HINTS" => Some(g_RECALL_HINTS()),
            "RECALL_NAMES_NOTHING" => Some(g_RECALL_NAMES_NOTHING()),
            "RECALL_POSSESSIVE" => Some(g_RECALL_POSSESSIVE()),
            "RECALL_SLOTS" => Some(g_RECALL_SLOTS()),
            "RECALL_SLOTS_BY_POSSESSIVE" => Some(g_RECALL_SLOTS_BY_POSSESSIVE()),
            "RECALL_WRAPPER" => Some(g_RECALL_WRAPPER()),
            "RELATION_ALIKE" => Some(g_RELATION_ALIKE()),
            "RELATIVE_MONTH" => Some(g_RELATIVE_MONTH()),
            "REMIND_QUESTION" => Some(g_REMIND_QUESTION()),
            "REPEAT_ADVERB" => Some(g_REPEAT_ADVERB()),
            "REPEAT_WORDS" => Some(g_REPEAT_WORDS()),
            "RESTATED_KEYS" => Some(g_RESTATED_KEYS()),
            "RESTRICT_OPENS" => Some(g_RESTRICT_OPENS()),
            "RESUME" => Some(g_RESUME()),
            "RETRACT_SAY" => Some(g_RETRACT_SAY()),
            "ROUTINE_SAY_BACK" => Some(g_ROUTINE_SAY_BACK()),
            "ROUTINE_SUBJECT" => Some(g_ROUTINE_SUBJECT()),
            "SAID_AGAIN" => Some(g_SAID_AGAIN()),
            "SAID_ASK" => Some(g_SAID_ASK()),
            "SAID_AS_A_SENTENCE" => Some(g_SAID_AS_A_SENTENCE()),
            "SAME_FACT_THEIR_OWN" => Some(g_SAME_FACT_THEIR_OWN()),
            "SELF" => Some(g_SELF()),
            "SHE_OR_HE" => Some(g_SHE_OR_HE()),
            "SLOT_KEY" => Some(g_SLOT_KEY()),
            "SLOT_WORDS" => Some(g_SLOT_WORDS()),
            "SMALL_NAMES" => Some(g_SMALL_NAMES()),
            "SORRY_OPENS_A_CORRECTION" => Some(g_SORRY_OPENS_A_CORRECTION()),
            "SPELLING_APART" => Some(g_SPELLING_APART()),
            "STATES_A_LIMIT" => Some(g_STATES_A_LIMIT()),
            "STATE_WAS_SAY" => Some(g_STATE_WAS_SAY()),
            "STILL_THE_SAME_PERSON" => Some(g_STILL_THE_SAME_PERSON()),
            "STOPWORDS" => Some(g_STOPWORDS()),
            "SWAPPABLE" => Some(g_SWAPPABLE()),
            "TAIL_WINDOW" => Some(g_TAIL_WINDOW()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "TASTE_ASKED_ABOUT" => Some(g_TASTE_ASKED_ABOUT()),
            "THAT_DATE" => Some(g_THAT_DATE()),
            "THEIR_AGE" => Some(g_THEIR_AGE()),
            "THEIR_JOB" => Some(g_THEIR_JOB()),
            "THEIR_NAME" => Some(g_THEIR_NAME()),
            "THEIR_TASTE_A_YES_NO" => Some(g_THEIR_TASTE_A_YES_NO()),
            "THEIR_TASTE_INTO" => Some(g_THEIR_TASTE_INTO()),
            "THEIR_TASTE_WHAT" => Some(g_THEIR_TASTE_WHAT()),
            "THEM" => Some(g_THEM()),
            "THEY_ARE_NAMED" => Some(g_THEY_ARE_NAMED()),
            "THE_KIDS" => Some(g_THE_KIDS()),
            "THE_PLACE_IS_NAMED" => Some(g_THE_PLACE_IS_NAMED()),
            "TOO_AS_WELL" => Some(g_TOO_AS_WELL()),
            "TURNED" => Some(g_TURNED()),
            "UNIT_FAMILY" => Some(g_UNIT_FAMILY()),
            "UNIT_SECONDS" => Some(g_UNIT_SECONDS()),
            "WEEK" => Some(g_WEEK()),
            "WEEKDAY_WORD" => Some(g_WEEKDAY_WORD()),
            "WEEKS_FIX" => Some(g_WEEKS_FIX()),
            "WHAT_I_SAID" => Some(g_WHAT_I_SAID()),
            "WHAT_I_SAID_NOTHING" => Some(g_WHAT_I_SAID_NOTHING()),
            "WHEN_ASKS" => Some(g_WHEN_ASKS()),
            "WHEN_END_ASK" => Some(g_WHEN_END_ASK()),
            "WHEN_END_WORD" => Some(g_WHEN_END_WORD()),
            "WHEN_INVERTED" => Some(g_WHEN_INVERTED()),
            "WHEN_INVERTED_SUBJECT" => Some(g_WHEN_INVERTED_SUBJECT()),
            "WHEN_PLAN" => Some(g_WHEN_PLAN()),
            "WHEN_QUALITY" => Some(g_WHEN_QUALITY()),
            "WHEN_START_ASK" => Some(g_WHEN_START_ASK()),
            "WHEN_THING" => Some(g_WHEN_THING()),
            "WHEN_THING_VERB" => Some(g_WHEN_THING_VERB()),
            "WHEN_VAGUE" => Some(g_WHEN_VAGUE()),
            "WHEN_VERB" => Some(g_WHEN_VERB()),
            "WHERE_ASK" => Some(g_WHERE_ASK()),
            "WHOLE_ASK" => Some(g_WHOLE_ASK()),
            "WORD_NUMBERS" => Some(g_WORD_NUMBERS()),
            "WORK_ON_IT" => Some(g_WORK_ON_IT()),
            "YES_NO_ASK" => Some(g_YES_NO_ASK()),
            "_ABOUT_WHO" => Some(g__ABOUT_WHO()),
            "_AMOUNTS" => Some(g__AMOUNTS()),
            "_AMOUNT_WORDS" => Some(g__AMOUNT_WORDS()),
            "_CONVERT" => Some(g__CONVERT()),
            "_COUNT_WORD" => Some(g__COUNT_WORD()),
            "_FACT_SLOT" => Some(g__FACT_SLOT()),
            "_HEDGE" => Some(g__HEDGE()),
            "_PET_WORD" => Some(g__PET_WORD()),
            "_PLURAL" => Some(g__PLURAL()),
            "_RELATION" => Some(g__RELATION()),
            "_SUBJECT_KEYS" => Some(g__SUBJECT_KEYS()),
            "_TENS_WORDS" => Some(g__TENS_WORDS()),
            "_UNIT_SAID" => Some(g__UNIT_SAID()),
            "_WEEKLY" => Some(g__WEEKLY()),
            "_a_callback_that_names_its_thing" => Some(py_func("_a_callback_that_names_its_thing", |args: &[Py]| _a_callback_that_names_its_thing(args))),
            "_a_correction_without_a_marker" => Some(py_func("_a_correction_without_a_marker", |args: &[Py]| _a_correction_without_a_marker(args))),
            "_a_count_from_the_date" => Some(py_func("_a_count_from_the_date", |args: &[Py]| _a_count_from_the_date(args))),
            "_a_detail_does_not_replace_the_job" => Some(py_func("_a_detail_does_not_replace_the_job", |args: &[Py]| _a_detail_does_not_replace_the_job(args))),
            "_a_fact_word" => Some(py_func("_a_fact_word", |args: &[Py]| _a_fact_word(args))),
            "_a_feeling_about_their_own_news" => Some(py_func("_a_feeling_about_their_own_news", |args: &[Py]| _a_feeling_about_their_own_news(args))),
            "_a_figure_of_those_units" => Some(py_func("_a_figure_of_those_units", |args: &[Py]| _a_figure_of_those_units(args))),
            "_a_kept_fact_read" => Some(py_func("_a_kept_fact_read", |args: &[Py]| _a_kept_fact_read(args))),
            "_a_person" => Some(py_func("_a_person", |args: &[Py]| _a_person(args))),
            "_a_person_called" => Some(py_func("_a_person_called", |args: &[Py]| _a_person_called(args))),
            "_a_pet_new_this_line" => Some(py_func("_a_pet_new_this_line", |args: &[Py]| _a_pet_new_this_line(args))),
            "_a_recall_about_nothing" => Some(py_func("_a_recall_about_nothing", |args: &[Py]| _a_recall_about_nothing(args))),
            "_a_recall_frame_about_nothing" => Some(py_func("_a_recall_frame_about_nothing", |args: &[Py]| _a_recall_frame_about_nothing(args))),
            "_a_second_persons_limit" => Some(py_func("_a_second_persons_limit", |args: &[Py]| _a_second_persons_limit(args))),
            "_a_trip_booked_named" => Some(py_func("_a_trip_booked_named", |args: &[Py]| _a_trip_booked_named(args))),
            "_a_word_for_the_slot" => Some(py_func("_a_word_for_the_slot", |args: &[Py]| _a_word_for_the_slot(args))),
            "_about_the_verb" => Some(py_func("_about_the_verb", |args: &[Py]| _about_the_verb(args))),
            "_age_said_of_the_only_animal" => Some(py_func("_age_said_of_the_only_animal", |args: &[Py]| _age_said_of_the_only_animal(args))),
            "_age_the_named_pet" => Some(py_func("_age_the_named_pet", |args: &[Py]| _age_the_named_pet(args))),
            "_age_they_answered" => Some(py_func("_age_they_answered", |args: &[Py]| _age_they_answered(args))),
            "_an_ask_about_nothing" => Some(py_func("_an_ask_about_nothing", |args: &[Py]| _an_ask_about_nothing(args))),
            "_an_ask_about_someone_else" => Some(py_func("_an_ask_about_someone_else", |args: &[Py]| _an_ask_about_someone_else(args))),
            "_another_animal_is_named" => Some(py_func("_another_animal_is_named", |args: &[Py]| _another_animal_is_named(args))),
            "_asks_about_someone_else" => Some(py_func("_asks_about_someone_else", |args: &[Py]| _asks_about_someone_else(args))),
            "_asks_to_hear_it_again" => Some(py_func("_asks_to_hear_it_again", |args: &[Py]| _asks_to_hear_it_again(args))),
            "_asks_whether_we_followed" => Some(py_func("_asks_whether_we_followed", |args: &[Py]| _asks_whether_we_followed(args))),
            "_budget_sign" => Some(py_func("_budget_sign", |args: &[Py]| _budget_sign(args))),
            "_concept" => Some(g__concept()),
            "_country" => Some(g__country()),
            "_diet_reply" => Some(py_func("_diet_reply", |args: &[Py]| _diet_reply(args))),
            "_drop_from_plan" => Some(py_func("_drop_from_plan", |args: &[Py]| _drop_from_plan(args))),
            "_drop_value" => Some(py_func("_drop_value", |args: &[Py]| _drop_value(args))),
            "_enough_reply" => Some(g__enough_reply()),
            "_fits" => Some(py_func("_fits", |args: &[Py]| _fits(args))),
            "_fix_a_persons_age" => Some(py_func("_fix_a_persons_age", |args: &[Py]| _fix_a_persons_age(args))),
            "_fix_a_persons_name" => Some(py_func("_fix_a_persons_name", |args: &[Py]| _fix_a_persons_name(args))),
            "_fix_a_pets_age" => Some(py_func("_fix_a_pets_age", |args: &[Py]| _fix_a_pets_age(args))),
            "_fix_a_pets_name" => Some(py_func("_fix_a_pets_name", |args: &[Py]| _fix_a_pets_name(args))),
            "_fix_the_make_of_a_thing" => Some(py_func("_fix_the_make_of_a_thing", |args: &[Py]| _fix_the_make_of_a_thing(args))),
            "_fix_the_weeks_of_a_pregnancy" => Some(py_func("_fix_the_weeks_of_a_pregnancy", |args: &[Py]| _fix_the_weeks_of_a_pregnancy(args))),
            "_forget_denied" => Some(py_func("_forget_denied", |args: &[Py]| _forget_denied(args))),
            "_last_stated" => Some(py_func("_last_stated", |args: &[Py]| _last_stated(args))),
            "_losses_taking" => Some(py_func("_losses_taking", |args: &[Py]| _losses_taking(args))),
            "_name_the_last_pet" => Some(py_func("_name_the_last_pet", |args: &[Py]| _name_the_last_pet(args))),
            "_names_a_pet_here" => Some(py_func("_names_a_pet_here", |args: &[Py]| _names_a_pet_here(args))),
            "_names_nothing_in_a_recall" => Some(py_func("_names_nothing_in_a_recall", |args: &[Py]| _names_nothing_in_a_recall(args))),
            "_names_this" => Some(py_func("_names_this", |args: &[Py]| _names_this(args))),
            "_note_a_corrected_fact" => Some(py_func("_note_a_corrected_fact", |args: &[Py]| _note_a_corrected_fact(args))),
            "_note_a_fact_moved" => Some(py_func("_note_a_fact_moved", |args: &[Py]| _note_a_fact_moved(args))),
            "_note_a_pet_that_is_new" => Some(py_func("_note_a_pet_that_is_new", |args: &[Py]| _note_a_pet_that_is_new(args))),
            "_observe" => Some(py_func("_observe", |args: &[Py]| _observe(args))),
            "_off_their_list" => Some(py_func("_off_their_list", |args: &[Py]| _off_their_list(args))),
            "_on_record_before" => Some(py_func("_on_record_before", |args: &[Py]| _on_record_before(args))),
            "_pets_of" => Some(py_func("_pets_of", |args: &[Py]| _pets_of(args))),
            "_plan_names" => Some(py_func("_plan_names", |args: &[Py]| _plan_names(args))),
            "_points_back_at_nothing" => Some(py_func("_points_back_at_nothing", |args: &[Py]| _points_back_at_nothing(args))),
            "_put_a_pets_age" => Some(py_func("_put_a_pets_age", |args: &[Py]| _put_a_pets_age(args))),
            "_reads_the_store" => Some(py_func("_reads_the_store", |args: &[Py]| _reads_the_store(args))),
            "_remember_dislikes" => Some(py_func("_remember_dislikes", |args: &[Py]| _remember_dislikes(args))),
            "_retract_last" => Some(py_func("_retract_last", |args: &[Py]| _retract_last(args))),
            "_stated_value" => Some(py_func("_stated_value", |args: &[Py]| _stated_value(args))),
            "_the_ages_of_the_children" => Some(py_func("_the_ages_of_the_children", |args: &[Py]| _the_ages_of_the_children(args))),
            "_the_animals_named" => Some(py_func("_the_animals_named", |args: &[Py]| _the_animals_named(args))),
            "_the_ask_about_how_it_went" => Some(py_func("_the_ask_about_how_it_went", |args: &[Py]| _the_ask_about_how_it_went(args))),
            "_the_ask_behind_the_recall" => Some(py_func("_the_ask_behind_the_recall", |args: &[Py]| _the_ask_behind_the_recall(args))),
            "_the_back_on_the_plate" => Some(py_func("_the_back_on_the_plate", |args: &[Py]| _the_back_on_the_plate(args))),
            "_the_clause" => Some(py_func("_the_clause", |args: &[Py]| _the_clause(args))),
            "_the_clock_in_their_line" => Some(py_func("_the_clock_in_their_line", |args: &[Py]| _the_clock_in_their_line(args))),
            "_the_count_was_the_line_before" => Some(py_func("_the_count_was_the_line_before", |args: &[Py]| _the_count_was_the_line_before(args))),
            "_the_diet_they_are_back_on" => Some(py_func("_the_diet_they_are_back_on", |args: &[Py]| _the_diet_they_are_back_on(args))),
            "_the_figure_said" => Some(py_func("_the_figure_said", |args: &[Py]| _the_figure_said(args))),
            "_the_figure_the_answer_names" => Some(py_func("_the_figure_the_answer_names", |args: &[Py]| _the_figure_the_answer_names(args))),
            "_the_figures_in" => Some(py_func("_the_figures_in", |args: &[Py]| _the_figures_in(args))),
            "_the_group_size_ask" => Some(py_func("_the_group_size_ask", |args: &[Py]| _the_group_size_ask(args))),
            "_the_last_ask_about_the_bot" => Some(py_func("_the_last_ask_about_the_bot", |args: &[Py]| _the_last_ask_about_the_bot(args))),
            "_the_last_line" => Some(py_func("_the_last_line", |args: &[Py]| _the_last_line(args))),
            "_the_money_figures" => Some(py_func("_the_money_figures", |args: &[Py]| _the_money_figures(args))),
            "_the_name_it_had" => Some(py_func("_the_name_it_had", |args: &[Py]| _the_name_it_had(args))),
            "_the_name_they_gave_to_the_person_they_mean" => Some(py_func("_the_name_they_gave_to_the_person_they_mean", |args: &[Py]| _the_name_they_gave_to_the_person_they_mean(args))),
            "_the_one_they_are_back_on" => Some(py_func("_the_one_they_are_back_on", |args: &[Py]| _the_one_they_are_back_on(args))),
            "_the_part_that_hurts_asked" => Some(py_func("_the_part_that_hurts_asked", |args: &[Py]| _the_part_that_hurts_asked(args))),
            "_the_pets_on_record" => Some(py_func("_the_pets_on_record", |args: &[Py]| _the_pets_on_record(args))),
            "_the_places_are_written_as_names" => Some(py_func("_the_places_are_written_as_names", |args: &[Py]| _the_places_are_written_as_names(args))),
            "_the_plan_they_took_back" => Some(py_func("_the_plan_they_took_back", |args: &[Py]| _the_plan_they_took_back(args))),
            "_the_store_readings" => Some(py_func("_the_store_readings", |args: &[Py]| _the_store_readings(args))),
            "_the_take_back_about_someone" => Some(py_func("_the_take_back_about_someone", |args: &[Py]| _the_take_back_about_someone(args))),
            "_the_time_they_put_right" => Some(py_func("_the_time_they_put_right", |args: &[Py]| _the_time_they_put_right(args))),
            "_the_turn_before_said" => Some(py_func("_the_turn_before_said", |args: &[Py]| _the_turn_before_said(args))),
            "_the_unit_asked" => Some(py_func("_the_unit_asked", |args: &[Py]| _the_unit_asked(args))),
            "_their_fact_ask" => Some(py_func("_their_fact_ask", |args: &[Py]| _their_fact_ask(args))),
            "_their_name_answered" => Some(py_func("_their_name_answered", |args: &[Py]| _their_name_answered(args))),
            "_their_own_line_ask" => Some(py_func("_their_own_line_ask", |args: &[Py]| _their_own_line_ask(args))),
            "_this_line_put_something_right" => Some(py_func("_this_line_put_something_right", |args: &[Py]| _this_line_put_something_right(args))),
            "_two_of_the_animal_are_kept" => Some(py_func("_two_of_the_animal_are_kept", |args: &[Py]| _two_of_the_animal_are_kept(args))),
            "_with_repaired" => Some(py_func("_with_repaired", |args: &[Py]| _with_repaired(args))),
            "_without_a_lead_in" => Some(py_func("_without_a_lead_in", |args: &[Py]| _without_a_lead_in(args))),
            "_without_the_shrug_at_the_end" => Some(py_func("_without_the_shrug_at_the_end", |args: &[Py]| _without_the_shrug_at_the_end(args))),
            "_word" => Some(g__word()),
            "_words" => Some(g__words()),
            "after" => Some(py_func("after", |args: &[Py]| after(args))),
            "note_the_hours" => Some(py_func("note_the_hours", |args: &[Py]| note_the_hours(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_PERSON_ASK" => s_ABOUT_PERSON_ASK(v),
            "ACKS" => s_ACKS(v),
            "ACTION_AMOUNT" => s_ACTION_AMOUNT(v),
            "ACTION_DURATION" => s_ACTION_DURATION(v),
            "ACTIVITY_ASK" => s_ACTIVITY_ASK(v),
            "ADVICE_MODULES" => s_ADVICE_MODULES(v),
            "AGES_IN_WORDS" => s_AGES_IN_WORDS(v),
            "AGE_ADVERB" => s_AGE_ADVERB(v),
            "AGE_AGAIN" => s_AGE_AGAIN(v),
            "AGE_ASK" => s_AGE_ASK(v),
            "AGE_AS_A_CAUSE" => s_AGE_AS_A_CAUSE(v),
            "AGE_COPULA" => s_AGE_COPULA(v),
            "AGE_DETERMINER" => s_AGE_DETERMINER(v),
            "AGE_FIX" => s_AGE_FIX(v),
            "AGE_FIX_WORDS" => s_AGE_FIX_WORDS(v),
            "AGE_IN_WORDS" => s_AGE_IN_WORDS(v),
            "AGE_PERSON_WORD" => s_AGE_PERSON_WORD(v),
            "AGE_RULE_WORD" => s_AGE_RULE_WORD(v),
            "AGE_SAID" => s_AGE_SAID(v),
            "AGE_WORD" => s_AGE_WORD(v),
            "AGE_YOU_SAY" => s_AGE_YOU_SAY(v),
            "AMOUNT" => s_AMOUNT(v),
            "AMOUNT_WORD" => s_AMOUNT_WORD(v),
            "ANOTHER_ANIMAL" => s_ANOTHER_ANIMAL(v),
            "ANOTHER_PERSON" => s_ANOTHER_PERSON(v),
            "ANSWERS" => s_ANSWERS(v),
            "ANSWER_MODULES" => s_ANSWER_MODULES(v),
            "ANY_FIGURE" => s_ANY_FIGURE(v),
            "ANY_NUMBER" => s_ANY_NUMBER(v),
            "ASKED_LEADS" => s_ASKED_LEADS(v),
            "ASKS_ABOUT_A_TASTE" => s_ASKS_ABOUT_A_TASTE(v),
            "ASKS_FOR_TASTES" => s_ASKS_FOR_TASTES(v),
            "ASK_ABOUT_THEM" => s_ASK_ABOUT_THEM(v),
            "ASK_OPENERS" => s_ASK_OPENERS(v),
            "ASK_START" => s_ASK_START(v),
            "AT_TIME" => s_AT_TIME(v),
            "AT_TIME_VALUE" => s_AT_TIME_VALUE(v),
            "A_DAY_BEFORE_A_MONTH" => s_A_DAY_BEFORE_A_MONTH(v),
            "A_DAY_SAID_LONG" => s_A_DAY_SAID_LONG(v),
            "A_HAVE_QUESTION" => s_A_HAVE_QUESTION(v),
            "A_MONEY_FIGURE" => s_A_MONEY_FIGURE(v),
            "A_NAME_AFTER_A_PRONOUN" => s_A_NAME_AFTER_A_PRONOUN(v),
            "A_NAME_WITH_ITS_VERB" => s_A_NAME_WITH_ITS_VERB(v),
            "A_PERCENT_FIGURE" => s_A_PERCENT_FIGURE(v),
            "A_QUESTION_ASKED" => s_A_QUESTION_ASKED(v),
            "A_RATE_ASK" => s_A_RATE_ASK(v),
            "A_RATE_STATED" => s_A_RATE_STATED(v),
            "A_RECALL_IN_ANOTHER_SHAPE" => s_A_RECALL_IN_ANOTHER_SHAPE(v),
            "A_SHORT_ANSWER" => s_A_SHORT_ANSWER(v),
            "A_UNIT_SAID" => s_A_UNIT_SAID(v),
            "BAD_NEWS" => s_BAD_NEWS(v),
            "BARE_ECHO_ONLY" => s_BARE_ECHO_ONLY(v),
            "BARE_HOUR" => s_BARE_HOUR(v),
            "BARE_NEW_COUNT" => s_BARE_NEW_COUNT(v),
            "BARE_SLOTS" => s_BARE_SLOTS(v),
            "BARE_WHAT_I_DO" => s_BARE_WHAT_I_DO(v),
            "BREED_ASK" => s_BREED_ASK(v),
            "BUDGET_FITS" => s_BUDGET_FITS(v),
            "BUDGET_MORE" => s_BUDGET_MORE(v),
            "BUDGET_TIGHT" => s_BUDGET_TIGHT(v),
            "CALENDAR_SAID" => s_CALENDAR_SAID(v),
            "CALL_LEADS" => s_CALL_LEADS(v),
            "CANT_HAVE" => s_CANT_HAVE(v),
            "CAN_HAVE" => s_CAN_HAVE(v),
            "CAN_THEY_HAVE" => s_CAN_THEY_HAVE(v),
            "CAN_THEY_HAVE_A_FOOD" => s_CAN_THEY_HAVE_A_FOOD(v),
            "CAR_CAPS" => s_CAR_CAPS(v),
            "CAT_NAMES" => s_CAT_NAMES(v),
            "CHANGE_HEAVY" => s_CHANGE_HEAVY(v),
            "CHANGE_PLAIN" => s_CHANGE_PLAIN(v),
            "CHAT_TAIL" => s_CHAT_TAIL(v),
            "CHECK_A_FEELING" => s_CHECK_A_FEELING(v),
            "CHECK_A_QUESTION" => s_CHECK_A_QUESTION(v),
            "CHECK_FOLLOWED" => s_CHECK_FOLLOWED(v),
            "CHECK_NOTHING_SAID" => s_CHECK_NOTHING_SAID(v),
            "CHECK_PLAIN" => s_CHECK_PLAIN(v),
            "CHILD" => s_CHILD(v),
            "CHILD_OF" => s_CHILD_OF(v),
            "CHILD_RELATIONS" => s_CHILD_RELATIONS(v),
            "CHILD_WORD" => s_CHILD_WORD(v),
            "CLOCK_SAID" => s_CLOCK_SAID(v),
            "CLOSE_HEAVY" => s_CLOSE_HEAVY(v),
            "CLOSE_PLAIN" => s_CLOSE_PLAIN(v),
            "CONCEPTS" => s_CONCEPTS(v),
            "CONDITION_KEEPS_DOWN" => s_CONDITION_KEEPS_DOWN(v),
            "CONDITION_RULES_OUT" => s_CONDITION_RULES_OUT(v),
            "CORRECTABLE" => s_CORRECTABLE(v),
            "CORRECTED_AGAIN" => s_CORRECTED_AGAIN(v),
            "CORRECTED_AGAIN_WHO" => s_CORRECTED_AGAIN_WHO(v),
            "CORRECTED_SAY" => s_CORRECTED_SAY(v),
            "CORRECTED_WHICH_PERSON" => s_CORRECTED_WHICH_PERSON(v),
            "CORRECTING" => s_CORRECTING(v),
            "CORRECTION_BACK" => s_CORRECTION_BACK(v),
            "CORRECTION_SAY" => s_CORRECTION_SAY(v),
            "CORRECTION_SAY_DROP" => s_CORRECTION_SAY_DROP(v),
            "CORRECTION_SAY_PAIR" => s_CORRECTION_SAY_PAIR(v),
            "CORRECTS_A_MAKE" => s_CORRECTS_A_MAKE(v),
            "CORRECTS_A_NAME" => s_CORRECTS_A_NAME(v),
            "CORRECTS_A_NAME_POSSESSIVE" => s_CORRECTS_A_NAME_POSSESSIVE(v),
            "COUNT_ANIMAL" => s_COUNT_ANIMAL(v),
            "COUNT_BARE" => s_COUNT_BARE(v),
            "COUNT_SAID" => s_COUNT_SAID(v),
            "COUNT_WORD" => s_COUNT_WORD(v),
            "COUNT_WORDS" => s_COUNT_WORDS(v),
            "CURRENCY" => s_CURRENCY(v),
            "DATE_DAY" => s_DATE_DAY(v),
            "DATE_PHRASE" => s_DATE_PHRASE(v),
            "DAY_ASKS" => s_DAY_ASKS(v),
            "DAY_OF_MONTH" => s_DAY_OF_MONTH(v),
            "DAY_WORD" => s_DAY_WORD(v),
            "DENIAL_SAY" => s_DENIAL_SAY(v),
            "DENIAL_STEMS" => s_DENIAL_STEMS(v),
            "DENIAL_WORDS" => s_DENIAL_WORDS(v),
            "DIET_FOOD" => s_DIET_FOOD(v),
            "DIET_RULES_OUT" => s_DIET_RULES_OUT(v),
            "DOG_NAMES" => s_DOG_NAMES(v),
            "DROP_LISTS" => s_DROP_LISTS(v),
            "EAT_ASK" => s_EAT_ASK(v),
            "ELIDED_AGAIN" => s_ELIDED_AGAIN(v),
            "ELIDED_ASK" => s_ELIDED_ASK(v),
            "ELIDED_NAMES" => s_ELIDED_NAMES(v),
            "ELIDED_NOT_SURE" => s_ELIDED_NOT_SURE(v),
            "ELIDED_WORDS" => s_ELIDED_WORDS(v),
            "FACT_FAMILY" => s_FACT_FAMILY(v),
            "FACT_WORDS" => s_FACT_WORDS(v),
            "FAMILY_GROUP" => s_FAMILY_GROUP(v),
            "FEEDING_SAY" => s_FEEDING_SAY(v),
            "FIGURE_AFTER" => s_FIGURE_AFTER(v),
            "FIGURE_ANSWER_LEN" => s_FIGURE_ANSWER_LEN(v),
            "FIGURE_ASK" => s_FIGURE_ASK(v),
            "FIGURE_FILLER" => s_FIGURE_FILLER(v),
            "FIGURE_MONEY" => s_FIGURE_MONEY(v),
            "FIGURE_NUMBER" => s_FIGURE_NUMBER(v),
            "FIGURE_QUALIFIER" => s_FIGURE_QUALIFIER(v),
            "FIGURE_TIME" => s_FIGURE_TIME(v),
            "FIGURE_UNITS" => s_FIGURE_UNITS(v),
            "FIRST_ASK" => s_FIRST_ASK(v),
            "FOOD_PHRASES" => s_FOOD_PHRASES(v),
            "GREET" => s_GREET(v),
            "GRIEF_WORDS" => s_GRIEF_WORDS(v),
            "GROUP_ASK" => s_GROUP_ASK(v),
            "GROUP_NAMES_TYPED" => s_GROUP_NAMES_TYPED(v),
            "GROUP_NAME_ASK" => s_GROUP_NAME_ASK(v),
            "GROUP_OF" => s_GROUP_OF(v),
            "GROUP_WORDS" => s_GROUP_WORDS(v),
            "HAVE_ANY" => s_HAVE_ANY(v),
            "HAVE_ANY_WORDS" => s_HAVE_ANY_WORDS(v),
            "HELD_THREAD" => s_HELD_THREAD(v),
            "HER_WORD" => s_HER_WORD(v),
            "HOW_MANY_OF_MINE" => s_HOW_MANY_OF_MINE(v),
            "IN_LAW_AFTER_TEXT" => s_IN_LAW_AFTER_TEXT(v),
            "KEPT_THINGS" => s_KEPT_THINGS(v),
            "KEY_SLOT" => s_KEY_SLOT(v),
            "KIN" => s_KIN(v),
            "KIN_ASK" => s_KIN_ASK(v),
            "LENGTH" => s_LENGTH(v),
            "LIMITS" => s_LIMITS(v),
            "LIMIT_SAY" => s_LIMIT_SAY(v),
            "LISTED_FIGURE" => s_LISTED_FIGURE(v),
            "LOOK_BACK_FIGURE" => s_LOOK_BACK_FIGURE(v),
            "MADE_OF_A_VALUE" => s_MADE_OF_A_VALUE(v),
            "MEMORY_POINTER" => s_MEMORY_POINTER(v),
            "MISSED_CATCH_UP" => s_MISSED_CATCH_UP(v),
            "MONTH_ALT" => s_MONTH_ALT(v),
            "MONTH_ORDER" => s_MONTH_ORDER(v),
            "MONTH_WORD" => s_MONTH_WORD(v),
            "MONTH_WORDS" => s_MONTH_WORDS(v),
            "NAMED_BY" => s_NAMED_BY(v),
            "NAMED_FACT_ASK" => s_NAMED_FACT_ASK(v),
            "NAME_ADVICE" => s_NAME_ADVICE(v),
            "NAME_AGAIN" => s_NAME_AGAIN(v),
            "NAME_FIX" => s_NAME_FIX(v),
            "NAME_FIX_WORDS" => s_NAME_FIX_WORDS(v),
            "NAME_GIVEN" => s_NAME_GIVEN(v),
            "NAME_GIVEN_WORDS" => s_NAME_GIVEN_WORDS(v),
            "NAME_NOT_A_NAME" => s_NAME_NOT_A_NAME(v),
            "NAMING_VERB" => s_NAMING_VERB(v),
            "NOTHING_STARTED" => s_NOTHING_STARTED(v),
            "NOT_A_FACT" => s_NOT_A_FACT(v),
            "NOT_A_FACT_OF_THEIRS" => s_NOT_A_FACT_OF_THEIRS(v),
            "NOT_A_NAME_AFTER_CALLED" => s_NOT_A_NAME_AFTER_CALLED(v),
            "NOT_A_NAME_BEFORE" => s_NOT_A_NAME_BEFORE(v),
            "NOT_A_NEW_VALUE" => s_NOT_A_NEW_VALUE(v),
            "NOT_A_PLACE_VALUE" => s_NOT_A_PLACE_VALUE(v),
            "NOT_A_UNIT" => s_NOT_A_UNIT(v),
            "NOT_FROM_THE_REPAIR" => s_NOT_FROM_THE_REPAIR(v),
            "NOT_ON_A_MEDIUM" => s_NOT_ON_A_MEDIUM(v),
            "NOT_RETRACTED" => s_NOT_RETRACTED(v),
            "NOT_THE_PLAN" => s_NOT_THE_PLAN(v),
            "NO_PLANS_REPLY" => s_NO_PLANS_REPLY(v),
            "NO_RECORD" => s_NO_RECORD(v),
            "NO_SUBJECT_OF_ITS_OWN" => s_NO_SUBJECT_OF_ITS_OWN(v),
            "NUMBER" => s_NUMBER(v),
            "NUMBER_DIGITS" => s_NUMBER_DIGITS(v),
            "NUMBER_VALUE" => s_NUMBER_VALUE(v),
            "NUMBER_WORD" => s_NUMBER_WORD(v),
            "NUMBER_WORDS" => s_NUMBER_WORDS(v),
            "OTHER_BARE_NUMBER" => s_OTHER_BARE_NUMBER(v),
            "OTHER_FACT" => s_OTHER_FACT(v),
            "OTHER_KIND_SAY" => s_OTHER_KIND_SAY(v),
            "OTHER_NOUNS" => s_OTHER_NOUNS(v),
            "OTHER_PLACE" => s_OTHER_PLACE(v),
            "OTHER_PLACE_PRONOUN" => s_OTHER_PLACE_PRONOUN(v),
            "OTHER_SAID" => s_OTHER_SAID(v),
            "OWNED_BY_THE_VERB" => s_OWNED_BY_THE_VERB(v),
            "PAIR_WITHOUT_A_MARKER" => s_PAIR_WITHOUT_A_MARKER(v),
            "PART_HELD" => s_PART_HELD(v),
            "PART_KEYS" => s_PART_KEYS(v),
            "PERSON_ADVICE_ASK" => s_PERSON_ADVICE_ASK(v),
            "PERSON_AFTER" => s_PERSON_AFTER(v),
            "PERSON_GROUP" => s_PERSON_GROUP(v),
            "PET_AGE_ANSWER" => s_PET_AGE_ANSWER(v),
            "PET_AGE_FIX_WORDS" => s_PET_AGE_FIX_WORDS(v),
            "PET_ASK" => s_PET_ASK(v),
            "PET_LEAD_IN" => s_PET_LEAD_IN(v),
            "PET_NAME_AGE" => s_PET_NAME_AGE(v),
            "PET_RENAMED_SAY" => s_PET_RENAMED_SAY(v),
            "PET_WORDS" => s_PET_WORDS(v),
            "PLACES" => s_PLACES(v),
            "PLACE_ENDS" => s_PLACE_ENDS(v),
            "PLACE_NOT" => s_PLACE_NOT(v),
            "PLAN_ASKS" => s_PLAN_ASKS(v),
            "PLURAL_ANIMAL" => s_PLURAL_ANIMAL(v),
            "PLURAL_PEOPLE" => s_PLURAL_PEOPLE(v),
            "PLURAL_PETS" => s_PLURAL_PETS(v),
            "POINTING_WORD_NOTHING" => s_POINTING_WORD_NOTHING(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "POINTS_BACK_AT_IT" => s_POINTS_BACK_AT_IT(v),
            "POINTS_BACK_NOTHING" => s_POINTS_BACK_NOTHING(v),
            "POSSESSIVE" => s_POSSESSIVE(v),
            "PREGNANCY_SAID_AGAIN" => s_PREGNANCY_SAID_AGAIN(v),
            "PREGNANT_OWN_SAY" => s_PREGNANT_OWN_SAY(v),
            "PREGNANT_SAY" => s_PREGNANT_SAY(v),
            "PREGNANT_WEEKS_SAY" => s_PREGNANT_WEEKS_SAY(v),
            "PRIORITY" => s_PRIORITY(v),
            "PRONOUN_FITS" => s_PRONOUN_FITS(v),
            "PRONOUN_PEOPLE" => s_PRONOUN_PEOPLE(v),
            "QUOTE_LEADS" => s_QUOTE_LEADS(v),
            "RECALL_HINTS" => s_RECALL_HINTS(v),
            "RECALL_NAMES_NOTHING" => s_RECALL_NAMES_NOTHING(v),
            "RECALL_POSSESSIVE" => s_RECALL_POSSESSIVE(v),
            "RECALL_SLOTS" => s_RECALL_SLOTS(v),
            "RECALL_SLOTS_BY_POSSESSIVE" => s_RECALL_SLOTS_BY_POSSESSIVE(v),
            "RECALL_WRAPPER" => s_RECALL_WRAPPER(v),
            "RELATION_ALIKE" => s_RELATION_ALIKE(v),
            "RELATIVE_MONTH" => s_RELATIVE_MONTH(v),
            "REMIND_QUESTION" => s_REMIND_QUESTION(v),
            "REPEAT_ADVERB" => s_REPEAT_ADVERB(v),
            "REPEAT_WORDS" => s_REPEAT_WORDS(v),
            "RESTATED_KEYS" => s_RESTATED_KEYS(v),
            "RESTRICT_OPENS" => s_RESTRICT_OPENS(v),
            "RESUME" => s_RESUME(v),
            "RETRACT_SAY" => s_RETRACT_SAY(v),
            "ROUTINE_SAY_BACK" => s_ROUTINE_SAY_BACK(v),
            "ROUTINE_SUBJECT" => s_ROUTINE_SUBJECT(v),
            "SAID_AGAIN" => s_SAID_AGAIN(v),
            "SAID_ASK" => s_SAID_ASK(v),
            "SAID_AS_A_SENTENCE" => s_SAID_AS_A_SENTENCE(v),
            "SAME_FACT_THEIR_OWN" => s_SAME_FACT_THEIR_OWN(v),
            "SELF" => s_SELF(v),
            "SHE_OR_HE" => s_SHE_OR_HE(v),
            "SLOT_KEY" => s_SLOT_KEY(v),
            "SLOT_WORDS" => s_SLOT_WORDS(v),
            "SMALL_NAMES" => s_SMALL_NAMES(v),
            "SORRY_OPENS_A_CORRECTION" => s_SORRY_OPENS_A_CORRECTION(v),
            "SPELLING_APART" => s_SPELLING_APART(v),
            "STATES_A_LIMIT" => s_STATES_A_LIMIT(v),
            "STATE_WAS_SAY" => s_STATE_WAS_SAY(v),
            "STILL_THE_SAME_PERSON" => s_STILL_THE_SAME_PERSON(v),
            "STOPWORDS" => s_STOPWORDS(v),
            "SWAPPABLE" => s_SWAPPABLE(v),
            "TAIL_WINDOW" => s_TAIL_WINDOW(v),
            "TASTE_ASKED_ABOUT" => s_TASTE_ASKED_ABOUT(v),
            "THAT_DATE" => s_THAT_DATE(v),
            "THEIR_AGE" => s_THEIR_AGE(v),
            "THEIR_JOB" => s_THEIR_JOB(v),
            "THEIR_NAME" => s_THEIR_NAME(v),
            "THEIR_TASTE_A_YES_NO" => s_THEIR_TASTE_A_YES_NO(v),
            "THEIR_TASTE_INTO" => s_THEIR_TASTE_INTO(v),
            "THEIR_TASTE_WHAT" => s_THEIR_TASTE_WHAT(v),
            "THEM" => s_THEM(v),
            "THEY_ARE_NAMED" => s_THEY_ARE_NAMED(v),
            "THE_KIDS" => s_THE_KIDS(v),
            "THE_PLACE_IS_NAMED" => s_THE_PLACE_IS_NAMED(v),
            "TOO_AS_WELL" => s_TOO_AS_WELL(v),
            "TURNED" => s_TURNED(v),
            "UNIT_FAMILY" => s_UNIT_FAMILY(v),
            "UNIT_SECONDS" => s_UNIT_SECONDS(v),
            "WEEK" => s_WEEK(v),
            "WEEKDAY_WORD" => s_WEEKDAY_WORD(v),
            "WEEKS_FIX" => s_WEEKS_FIX(v),
            "WHAT_I_SAID" => s_WHAT_I_SAID(v),
            "WHAT_I_SAID_NOTHING" => s_WHAT_I_SAID_NOTHING(v),
            "WHEN_ASKS" => s_WHEN_ASKS(v),
            "WHEN_END_ASK" => s_WHEN_END_ASK(v),
            "WHEN_END_WORD" => s_WHEN_END_WORD(v),
            "WHEN_INVERTED" => s_WHEN_INVERTED(v),
            "WHEN_INVERTED_SUBJECT" => s_WHEN_INVERTED_SUBJECT(v),
            "WHEN_PLAN" => s_WHEN_PLAN(v),
            "WHEN_QUALITY" => s_WHEN_QUALITY(v),
            "WHEN_START_ASK" => s_WHEN_START_ASK(v),
            "WHEN_THING" => s_WHEN_THING(v),
            "WHEN_THING_VERB" => s_WHEN_THING_VERB(v),
            "WHEN_VAGUE" => s_WHEN_VAGUE(v),
            "WHEN_VERB" => s_WHEN_VERB(v),
            "WHERE_ASK" => s_WHERE_ASK(v),
            "WHOLE_ASK" => s_WHOLE_ASK(v),
            "WORD_NUMBERS" => s_WORD_NUMBERS(v),
            "WORK_ON_IT" => s_WORK_ON_IT(v),
            "YES_NO_ASK" => s_YES_NO_ASK(v),
            "_ABOUT_WHO" => s__ABOUT_WHO(v),
            "_AMOUNTS" => s__AMOUNTS(v),
            "_AMOUNT_WORDS" => s__AMOUNT_WORDS(v),
            "_CONVERT" => s__CONVERT(v),
            "_COUNT_WORD" => s__COUNT_WORD(v),
            "_FACT_SLOT" => s__FACT_SLOT(v),
            "_HEDGE" => s__HEDGE(v),
            "_PET_WORD" => s__PET_WORD(v),
            "_PLURAL" => s__PLURAL(v),
            "_RELATION" => s__RELATION(v),
            "_SUBJECT_KEYS" => s__SUBJECT_KEYS(v),
            "_TENS_WORDS" => s__TENS_WORDS(v),
            "_UNIT_SAID" => s__UNIT_SAID(v),
            "_WEEKLY" => s__WEEKLY(v),
            "_concept" => s__concept(v),
            "_country" => s__country(v),
            "_enough_reply" => s__enough_reply(v),
            "_word" => s__word(v),
            "_words" => s__words(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.memory_store", module_obj);
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_kept_fact_read", py_func("_a_kept_fact_read", |args: &[Py]| _a_kept_fact_read(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_their_own_line_ask", py_func("_their_own_line_ask", |args: &[Py]| _their_own_line_ask(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_with_repaired", py_func("_with_repaired", |args: &[Py]| _with_repaired(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_places_are_written_as_names", py_func("_the_places_are_written_as_names", |args: &[Py]| _the_places_are_written_as_names(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "note_the_hours", py_func("note_the_hours", |args: &[Py]| note_the_hours(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_detail_does_not_replace_the_job", py_func("_a_detail_does_not_replace_the_job", |args: &[Py]| _a_detail_does_not_replace_the_job(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_without_the_shrug_at_the_end", py_func("_without_the_shrug_at_the_end", |args: &[Py]| _without_the_shrug_at_the_end(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "after", py_func("after", |args: &[Py]| after(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_ages_of_the_children", py_func("_the_ages_of_the_children", |args: &[Py]| _the_ages_of_the_children(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_count_was_the_line_before", py_func("_the_count_was_the_line_before", |args: &[Py]| _the_count_was_the_line_before(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_pets_on_record", py_func("_the_pets_on_record", |args: &[Py]| _the_pets_on_record(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_note_a_pet_that_is_new", py_func("_note_a_pet_that_is_new", |args: &[Py]| _note_a_pet_that_is_new(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_observe", py_func("_observe", |args: &[Py]| _observe(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_trip_booked_named", py_func("_a_trip_booked_named", |args: &[Py]| _a_trip_booked_named(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_this_line_put_something_right", py_func("_this_line_put_something_right", |args: &[Py]| _this_line_put_something_right(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_on_record_before", py_func("_on_record_before", |args: &[Py]| _on_record_before(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_forget_denied", py_func("_forget_denied", |args: &[Py]| _forget_denied(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_back_on_the_plate", py_func("_the_back_on_the_plate", |args: &[Py]| _the_back_on_the_plate(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_one_they_are_back_on", py_func("_the_one_they_are_back_on", |args: &[Py]| _the_one_they_are_back_on(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_off_their_list", py_func("_off_their_list", |args: &[Py]| _off_their_list(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_diet_they_are_back_on", py_func("_the_diet_they_are_back_on", |args: &[Py]| _the_diet_they_are_back_on(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_last_line", py_func("_the_last_line", |args: &[Py]| _the_last_line(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_losses_taking", py_func("_losses_taking", |args: &[Py]| _losses_taking(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_last_stated", py_func("_last_stated", |args: &[Py]| _last_stated(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_retract_last", py_func("_retract_last", |args: &[Py]| _retract_last(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_reads_the_store", py_func("_reads_the_store", |args: &[Py]| _reads_the_store(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_plan_they_took_back", py_func("_the_plan_they_took_back", |args: &[Py]| _the_plan_they_took_back(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_clock_in_their_line", py_func("_the_clock_in_their_line", |args: &[Py]| _the_clock_in_their_line(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_an_ask_about_someone_else", py_func("_an_ask_about_someone_else", |args: &[Py]| _an_ask_about_someone_else(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_take_back_about_someone", py_func("_the_take_back_about_someone", |args: &[Py]| _the_take_back_about_someone(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_part_that_hurts_asked", py_func("_the_part_that_hurts_asked", |args: &[Py]| _the_part_that_hurts_asked(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_store_readings", py_func("_the_store_readings", |args: &[Py]| _the_store_readings(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_their_fact_ask", py_func("_their_fact_ask", |args: &[Py]| _their_fact_ask(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_name_the_last_pet", py_func("_name_the_last_pet", |args: &[Py]| _name_the_last_pet(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_animals_named", py_func("_the_animals_named", |args: &[Py]| _the_animals_named(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_name_it_had", py_func("_the_name_it_had", |args: &[Py]| _the_name_it_had(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_names_a_pet_here", py_func("_names_a_pet_here", |args: &[Py]| _names_a_pet_here(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_figure_said", py_func("_the_figure_said", |args: &[Py]| _the_figure_said(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_without_a_lead_in", py_func("_without_a_lead_in", |args: &[Py]| _without_a_lead_in(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_put_a_pets_age", py_func("_put_a_pets_age", |args: &[Py]| _put_a_pets_age(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_age_the_named_pet", py_func("_age_the_named_pet", |args: &[Py]| _age_the_named_pet(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_fix_a_pets_age", py_func("_fix_a_pets_age", |args: &[Py]| _fix_a_pets_age(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_age_said_of_the_only_animal", py_func("_age_said_of_the_only_animal", |args: &[Py]| _age_said_of_the_only_animal(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_age_they_answered", py_func("_age_they_answered", |args: &[Py]| _age_they_answered(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_their_name_answered", py_func("_their_name_answered", |args: &[Py]| _their_name_answered(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_fix_a_persons_age", py_func("_fix_a_persons_age", |args: &[Py]| _fix_a_persons_age(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_fix_the_weeks_of_a_pregnancy", py_func("_fix_the_weeks_of_a_pregnancy", |args: &[Py]| _fix_the_weeks_of_a_pregnancy(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_fix_a_pets_name", py_func("_fix_a_pets_name", |args: &[Py]| _fix_a_pets_name(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_pets_of", py_func("_pets_of", |args: &[Py]| _pets_of(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_pet_new_this_line", py_func("_a_pet_new_this_line", |args: &[Py]| _a_pet_new_this_line(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_another_animal_is_named", py_func("_another_animal_is_named", |args: &[Py]| _another_animal_is_named(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_two_of_the_animal_are_kept", py_func("_two_of_the_animal_are_kept", |args: &[Py]| _two_of_the_animal_are_kept(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_turn_before_said", py_func("_the_turn_before_said", |args: &[Py]| _the_turn_before_said(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_fix_the_make_of_a_thing", py_func("_fix_the_make_of_a_thing", |args: &[Py]| _fix_the_make_of_a_thing(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_fix_a_persons_name", py_func("_fix_a_persons_name", |args: &[Py]| _fix_a_persons_name(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_name_they_gave_to_the_person_they_mean", py_func("_the_name_they_gave_to_the_person_they_mean", |args: &[Py]| _the_name_they_gave_to_the_person_they_mean(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_person", py_func("_a_person", |args: &[Py]| _a_person(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_correction_without_a_marker", py_func("_a_correction_without_a_marker", |args: &[Py]| _a_correction_without_a_marker(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_person_called", py_func("_a_person_called", |args: &[Py]| _a_person_called(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_stated_value", py_func("_stated_value", |args: &[Py]| _stated_value(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_second_persons_limit", py_func("_a_second_persons_limit", |args: &[Py]| _a_second_persons_limit(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_drop_value", py_func("_drop_value", |args: &[Py]| _drop_value(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_plan_names", py_func("_plan_names", |args: &[Py]| _plan_names(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_drop_from_plan", py_func("_drop_from_plan", |args: &[Py]| _drop_from_plan(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_names_this", py_func("_names_this", |args: &[Py]| _names_this(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_budget_sign", py_func("_budget_sign", |args: &[Py]| _budget_sign(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_feeling_about_their_own_news", py_func("_a_feeling_about_their_own_news", |args: &[Py]| _a_feeling_about_their_own_news(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_ask_about_how_it_went", py_func("_the_ask_about_how_it_went", |args: &[Py]| _the_ask_about_how_it_went(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_count_from_the_date", py_func("_a_count_from_the_date", |args: &[Py]| _a_count_from_the_date(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_unit_asked", py_func("_the_unit_asked", |args: &[Py]| _the_unit_asked(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_clause", py_func("_the_clause", |args: &[Py]| _the_clause(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_money_figures", py_func("_the_money_figures", |args: &[Py]| _the_money_figures(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_figures_in", py_func("_the_figures_in", |args: &[Py]| _the_figures_in(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_figure_the_answer_names", py_func("_the_figure_the_answer_names", |args: &[Py]| _the_figure_the_answer_names(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_figure_of_those_units", py_func("_a_figure_of_those_units", |args: &[Py]| _a_figure_of_those_units(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_time_they_put_right", py_func("_the_time_they_put_right", |args: &[Py]| _the_time_they_put_right(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_asks_to_hear_it_again", py_func("_asks_to_hear_it_again", |args: &[Py]| _asks_to_hear_it_again(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_asks_about_someone_else", py_func("_asks_about_someone_else", |args: &[Py]| _asks_about_someone_else(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_note_a_corrected_fact", py_func("_note_a_corrected_fact", |args: &[Py]| _note_a_corrected_fact(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_note_a_fact_moved", py_func("_note_a_fact_moved", |args: &[Py]| _note_a_fact_moved(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_word_for_the_slot", py_func("_a_word_for_the_slot", |args: &[Py]| _a_word_for_the_slot(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_ask_behind_the_recall", py_func("_the_ask_behind_the_recall", |args: &[Py]| _the_ask_behind_the_recall(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_about_the_verb", py_func("_about_the_verb", |args: &[Py]| _about_the_verb(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_remember_dislikes", py_func("_remember_dislikes", |args: &[Py]| _remember_dislikes(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_diet_reply", py_func("_diet_reply", |args: &[Py]| _diet_reply(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_fact_word", py_func("_a_fact_word", |args: &[Py]| _a_fact_word(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_callback_that_names_its_thing", py_func("_a_callback_that_names_its_thing", |args: &[Py]| _a_callback_that_names_its_thing(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_points_back_at_nothing", py_func("_points_back_at_nothing", |args: &[Py]| _points_back_at_nothing(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_recall_about_nothing", py_func("_a_recall_about_nothing", |args: &[Py]| _a_recall_about_nothing(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_names_nothing_in_a_recall", py_func("_names_nothing_in_a_recall", |args: &[Py]| _names_nothing_in_a_recall(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_a_recall_frame_about_nothing", py_func("_a_recall_frame_about_nothing", |args: &[Py]| _a_recall_frame_about_nothing(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_an_ask_about_nothing", py_func("_an_ask_about_nothing", |args: &[Py]| _an_ask_about_nothing(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_last_ask_about_the_bot", py_func("_the_last_ask_about_the_bot", |args: &[Py]| _the_last_ask_about_the_bot(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_fits", py_func("_fits", |args: &[Py]| _fits(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_asks_whether_we_followed", py_func("_asks_whether_we_followed", |args: &[Py]| _asks_whether_we_followed(args)));
    pyrt::xrefs::reg_fn("modules.memory_store", "_the_group_size_ask", py_func("_the_group_size_ask", |args: &[Py]| _the_group_size_ask(args)));
    pyrt::xrefs::reg_access("modules.memory_store", "ABOUT_PERSON_ASK", g_ABOUT_PERSON_ASK, s_ABOUT_PERSON_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "ACKS", g_ACKS, s_ACKS);
    pyrt::xrefs::reg_access("modules.memory_store", "ACTION_AMOUNT", g_ACTION_AMOUNT, s_ACTION_AMOUNT);
    pyrt::xrefs::reg_access("modules.memory_store", "ACTION_DURATION", g_ACTION_DURATION, s_ACTION_DURATION);
    pyrt::xrefs::reg_access("modules.memory_store", "ACTIVITY_ASK", g_ACTIVITY_ASK, s_ACTIVITY_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "ADVICE_MODULES", g_ADVICE_MODULES, s_ADVICE_MODULES);
    pyrt::xrefs::reg_access("modules.memory_store", "AGES_IN_WORDS", g_AGES_IN_WORDS, s_AGES_IN_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_ADVERB", g_AGE_ADVERB, s_AGE_ADVERB);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_AGAIN", g_AGE_AGAIN, s_AGE_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_ASK", g_AGE_ASK, s_AGE_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_AS_A_CAUSE", g_AGE_AS_A_CAUSE, s_AGE_AS_A_CAUSE);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_COPULA", g_AGE_COPULA, s_AGE_COPULA);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_DETERMINER", g_AGE_DETERMINER, s_AGE_DETERMINER);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_FIX", g_AGE_FIX, s_AGE_FIX);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_FIX_WORDS", g_AGE_FIX_WORDS, s_AGE_FIX_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_IN_WORDS", g_AGE_IN_WORDS, s_AGE_IN_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_PERSON_WORD", g_AGE_PERSON_WORD, s_AGE_PERSON_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_RULE_WORD", g_AGE_RULE_WORD, s_AGE_RULE_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_SAID", g_AGE_SAID, s_AGE_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_WORD", g_AGE_WORD, s_AGE_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "AGE_YOU_SAY", g_AGE_YOU_SAY, s_AGE_YOU_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "AMOUNT", g_AMOUNT, s_AMOUNT);
    pyrt::xrefs::reg_access("modules.memory_store", "AMOUNT_WORD", g_AMOUNT_WORD, s_AMOUNT_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "ANOTHER_ANIMAL", g_ANOTHER_ANIMAL, s_ANOTHER_ANIMAL);
    pyrt::xrefs::reg_access("modules.memory_store", "ANOTHER_PERSON", g_ANOTHER_PERSON, s_ANOTHER_PERSON);
    pyrt::xrefs::reg_access("modules.memory_store", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.memory_store", "ANSWER_MODULES", g_ANSWER_MODULES, s_ANSWER_MODULES);
    pyrt::xrefs::reg_access("modules.memory_store", "ANY_FIGURE", g_ANY_FIGURE, s_ANY_FIGURE);
    pyrt::xrefs::reg_access("modules.memory_store", "ANY_NUMBER", g_ANY_NUMBER, s_ANY_NUMBER);
    pyrt::xrefs::reg_access("modules.memory_store", "ASKED_LEADS", g_ASKED_LEADS, s_ASKED_LEADS);
    pyrt::xrefs::reg_access("modules.memory_store", "ASKS_ABOUT_A_TASTE", g_ASKS_ABOUT_A_TASTE, s_ASKS_ABOUT_A_TASTE);
    pyrt::xrefs::reg_access("modules.memory_store", "ASKS_FOR_TASTES", g_ASKS_FOR_TASTES, s_ASKS_FOR_TASTES);
    pyrt::xrefs::reg_access("modules.memory_store", "ASK_ABOUT_THEM", g_ASK_ABOUT_THEM, s_ASK_ABOUT_THEM);
    pyrt::xrefs::reg_access("modules.memory_store", "ASK_OPENERS", g_ASK_OPENERS, s_ASK_OPENERS);
    pyrt::xrefs::reg_access("modules.memory_store", "ASK_START", g_ASK_START, s_ASK_START);
    pyrt::xrefs::reg_access("modules.memory_store", "AT_TIME", g_AT_TIME, s_AT_TIME);
    pyrt::xrefs::reg_access("modules.memory_store", "AT_TIME_VALUE", g_AT_TIME_VALUE, s_AT_TIME_VALUE);
    pyrt::xrefs::reg_access("modules.memory_store", "A_DAY_BEFORE_A_MONTH", g_A_DAY_BEFORE_A_MONTH, s_A_DAY_BEFORE_A_MONTH);
    pyrt::xrefs::reg_access("modules.memory_store", "A_DAY_SAID_LONG", g_A_DAY_SAID_LONG, s_A_DAY_SAID_LONG);
    pyrt::xrefs::reg_access("modules.memory_store", "A_HAVE_QUESTION", g_A_HAVE_QUESTION, s_A_HAVE_QUESTION);
    pyrt::xrefs::reg_access("modules.memory_store", "A_MONEY_FIGURE", g_A_MONEY_FIGURE, s_A_MONEY_FIGURE);
    pyrt::xrefs::reg_access("modules.memory_store", "A_NAME_AFTER_A_PRONOUN", g_A_NAME_AFTER_A_PRONOUN, s_A_NAME_AFTER_A_PRONOUN);
    pyrt::xrefs::reg_access("modules.memory_store", "A_NAME_WITH_ITS_VERB", g_A_NAME_WITH_ITS_VERB, s_A_NAME_WITH_ITS_VERB);
    pyrt::xrefs::reg_access("modules.memory_store", "A_PERCENT_FIGURE", g_A_PERCENT_FIGURE, s_A_PERCENT_FIGURE);
    pyrt::xrefs::reg_access("modules.memory_store", "A_QUESTION_ASKED", g_A_QUESTION_ASKED, s_A_QUESTION_ASKED);
    pyrt::xrefs::reg_access("modules.memory_store", "A_RATE_ASK", g_A_RATE_ASK, s_A_RATE_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "A_RATE_STATED", g_A_RATE_STATED, s_A_RATE_STATED);
    pyrt::xrefs::reg_access("modules.memory_store", "A_RECALL_IN_ANOTHER_SHAPE", g_A_RECALL_IN_ANOTHER_SHAPE, s_A_RECALL_IN_ANOTHER_SHAPE);
    pyrt::xrefs::reg_access("modules.memory_store", "A_SHORT_ANSWER", g_A_SHORT_ANSWER, s_A_SHORT_ANSWER);
    pyrt::xrefs::reg_access("modules.memory_store", "A_UNIT_SAID", g_A_UNIT_SAID, s_A_UNIT_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "BAD_NEWS", g_BAD_NEWS, s_BAD_NEWS);
    pyrt::xrefs::reg_access("modules.memory_store", "BARE_ECHO_ONLY", g_BARE_ECHO_ONLY, s_BARE_ECHO_ONLY);
    pyrt::xrefs::reg_access("modules.memory_store", "BARE_HOUR", g_BARE_HOUR, s_BARE_HOUR);
    pyrt::xrefs::reg_access("modules.memory_store", "BARE_NEW_COUNT", g_BARE_NEW_COUNT, s_BARE_NEW_COUNT);
    pyrt::xrefs::reg_access("modules.memory_store", "BARE_SLOTS", g_BARE_SLOTS, s_BARE_SLOTS);
    pyrt::xrefs::reg_access("modules.memory_store", "BARE_WHAT_I_DO", g_BARE_WHAT_I_DO, s_BARE_WHAT_I_DO);
    pyrt::xrefs::reg_access("modules.memory_store", "BREED_ASK", g_BREED_ASK, s_BREED_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "BUDGET_FITS", g_BUDGET_FITS, s_BUDGET_FITS);
    pyrt::xrefs::reg_access("modules.memory_store", "BUDGET_MORE", g_BUDGET_MORE, s_BUDGET_MORE);
    pyrt::xrefs::reg_access("modules.memory_store", "BUDGET_TIGHT", g_BUDGET_TIGHT, s_BUDGET_TIGHT);
    pyrt::xrefs::reg_access("modules.memory_store", "CALENDAR_SAID", g_CALENDAR_SAID, s_CALENDAR_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "CALL_LEADS", g_CALL_LEADS, s_CALL_LEADS);
    pyrt::xrefs::reg_access("modules.memory_store", "CANT_HAVE", g_CANT_HAVE, s_CANT_HAVE);
    pyrt::xrefs::reg_access("modules.memory_store", "CAN_HAVE", g_CAN_HAVE, s_CAN_HAVE);
    pyrt::xrefs::reg_access("modules.memory_store", "CAN_THEY_HAVE", g_CAN_THEY_HAVE, s_CAN_THEY_HAVE);
    pyrt::xrefs::reg_access("modules.memory_store", "CAN_THEY_HAVE_A_FOOD", g_CAN_THEY_HAVE_A_FOOD, s_CAN_THEY_HAVE_A_FOOD);
    pyrt::xrefs::reg_access("modules.memory_store", "CAR_CAPS", g_CAR_CAPS, s_CAR_CAPS);
    pyrt::xrefs::reg_access("modules.memory_store", "CAT_NAMES", g_CAT_NAMES, s_CAT_NAMES);
    pyrt::xrefs::reg_access("modules.memory_store", "CHANGE_HEAVY", g_CHANGE_HEAVY, s_CHANGE_HEAVY);
    pyrt::xrefs::reg_access("modules.memory_store", "CHANGE_PLAIN", g_CHANGE_PLAIN, s_CHANGE_PLAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "CHAT_TAIL", g_CHAT_TAIL, s_CHAT_TAIL);
    pyrt::xrefs::reg_access("modules.memory_store", "CHECK_A_FEELING", g_CHECK_A_FEELING, s_CHECK_A_FEELING);
    pyrt::xrefs::reg_access("modules.memory_store", "CHECK_A_QUESTION", g_CHECK_A_QUESTION, s_CHECK_A_QUESTION);
    pyrt::xrefs::reg_access("modules.memory_store", "CHECK_FOLLOWED", g_CHECK_FOLLOWED, s_CHECK_FOLLOWED);
    pyrt::xrefs::reg_access("modules.memory_store", "CHECK_NOTHING_SAID", g_CHECK_NOTHING_SAID, s_CHECK_NOTHING_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "CHECK_PLAIN", g_CHECK_PLAIN, s_CHECK_PLAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "CHILD", g_CHILD, s_CHILD);
    pyrt::xrefs::reg_access("modules.memory_store", "CHILD_OF", g_CHILD_OF, s_CHILD_OF);
    pyrt::xrefs::reg_access("modules.memory_store", "CHILD_RELATIONS", g_CHILD_RELATIONS, s_CHILD_RELATIONS);
    pyrt::xrefs::reg_access("modules.memory_store", "CHILD_WORD", g_CHILD_WORD, s_CHILD_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "CLOCK_SAID", g_CLOCK_SAID, s_CLOCK_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "CLOSE_HEAVY", g_CLOSE_HEAVY, s_CLOSE_HEAVY);
    pyrt::xrefs::reg_access("modules.memory_store", "CLOSE_PLAIN", g_CLOSE_PLAIN, s_CLOSE_PLAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "CONCEPTS", g_CONCEPTS, s_CONCEPTS);
    pyrt::xrefs::reg_access("modules.memory_store", "CONDITION_KEEPS_DOWN", g_CONDITION_KEEPS_DOWN, s_CONDITION_KEEPS_DOWN);
    pyrt::xrefs::reg_access("modules.memory_store", "CONDITION_RULES_OUT", g_CONDITION_RULES_OUT, s_CONDITION_RULES_OUT);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTABLE", g_CORRECTABLE, s_CORRECTABLE);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTED_AGAIN", g_CORRECTED_AGAIN, s_CORRECTED_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTED_AGAIN_WHO", g_CORRECTED_AGAIN_WHO, s_CORRECTED_AGAIN_WHO);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTED_SAY", g_CORRECTED_SAY, s_CORRECTED_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTED_WHICH_PERSON", g_CORRECTED_WHICH_PERSON, s_CORRECTED_WHICH_PERSON);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTING", g_CORRECTING, s_CORRECTING);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTION_BACK", g_CORRECTION_BACK, s_CORRECTION_BACK);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTION_SAY", g_CORRECTION_SAY, s_CORRECTION_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTION_SAY_DROP", g_CORRECTION_SAY_DROP, s_CORRECTION_SAY_DROP);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTION_SAY_PAIR", g_CORRECTION_SAY_PAIR, s_CORRECTION_SAY_PAIR);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTS_A_MAKE", g_CORRECTS_A_MAKE, s_CORRECTS_A_MAKE);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTS_A_NAME", g_CORRECTS_A_NAME, s_CORRECTS_A_NAME);
    pyrt::xrefs::reg_access("modules.memory_store", "CORRECTS_A_NAME_POSSESSIVE", g_CORRECTS_A_NAME_POSSESSIVE, s_CORRECTS_A_NAME_POSSESSIVE);
    pyrt::xrefs::reg_access("modules.memory_store", "COUNT_ANIMAL", g_COUNT_ANIMAL, s_COUNT_ANIMAL);
    pyrt::xrefs::reg_access("modules.memory_store", "COUNT_BARE", g_COUNT_BARE, s_COUNT_BARE);
    pyrt::xrefs::reg_access("modules.memory_store", "COUNT_SAID", g_COUNT_SAID, s_COUNT_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "COUNT_WORD", g_COUNT_WORD, s_COUNT_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "COUNT_WORDS", g_COUNT_WORDS, s_COUNT_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "CURRENCY", g_CURRENCY, s_CURRENCY);
    pyrt::xrefs::reg_access("modules.memory_store", "DATE_DAY", g_DATE_DAY, s_DATE_DAY);
    pyrt::xrefs::reg_access("modules.memory_store", "DATE_PHRASE", g_DATE_PHRASE, s_DATE_PHRASE);
    pyrt::xrefs::reg_access("modules.memory_store", "DAY_ASKS", g_DAY_ASKS, s_DAY_ASKS);
    pyrt::xrefs::reg_access("modules.memory_store", "DAY_OF_MONTH", g_DAY_OF_MONTH, s_DAY_OF_MONTH);
    pyrt::xrefs::reg_access("modules.memory_store", "DAY_WORD", g_DAY_WORD, s_DAY_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "DENIAL_SAY", g_DENIAL_SAY, s_DENIAL_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "DENIAL_STEMS", g_DENIAL_STEMS, s_DENIAL_STEMS);
    pyrt::xrefs::reg_access("modules.memory_store", "DENIAL_WORDS", g_DENIAL_WORDS, s_DENIAL_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "DIET_FOOD", g_DIET_FOOD, s_DIET_FOOD);
    pyrt::xrefs::reg_access("modules.memory_store", "DIET_RULES_OUT", g_DIET_RULES_OUT, s_DIET_RULES_OUT);
    pyrt::xrefs::reg_access("modules.memory_store", "DOG_NAMES", g_DOG_NAMES, s_DOG_NAMES);
    pyrt::xrefs::reg_access("modules.memory_store", "DROP_LISTS", g_DROP_LISTS, s_DROP_LISTS);
    pyrt::xrefs::reg_access("modules.memory_store", "EAT_ASK", g_EAT_ASK, s_EAT_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "ELIDED_AGAIN", g_ELIDED_AGAIN, s_ELIDED_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "ELIDED_ASK", g_ELIDED_ASK, s_ELIDED_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "ELIDED_NAMES", g_ELIDED_NAMES, s_ELIDED_NAMES);
    pyrt::xrefs::reg_access("modules.memory_store", "ELIDED_NOT_SURE", g_ELIDED_NOT_SURE, s_ELIDED_NOT_SURE);
    pyrt::xrefs::reg_access("modules.memory_store", "ELIDED_WORDS", g_ELIDED_WORDS, s_ELIDED_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "FACT_FAMILY", g_FACT_FAMILY, s_FACT_FAMILY);
    pyrt::xrefs::reg_access("modules.memory_store", "FACT_WORDS", g_FACT_WORDS, s_FACT_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "FAMILY_GROUP", g_FAMILY_GROUP, s_FAMILY_GROUP);
    pyrt::xrefs::reg_access("modules.memory_store", "FEEDING_SAY", g_FEEDING_SAY, s_FEEDING_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_AFTER", g_FIGURE_AFTER, s_FIGURE_AFTER);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_ANSWER_LEN", g_FIGURE_ANSWER_LEN, s_FIGURE_ANSWER_LEN);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_ASK", g_FIGURE_ASK, s_FIGURE_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_FILLER", g_FIGURE_FILLER, s_FIGURE_FILLER);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_MONEY", g_FIGURE_MONEY, s_FIGURE_MONEY);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_NUMBER", g_FIGURE_NUMBER, s_FIGURE_NUMBER);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_QUALIFIER", g_FIGURE_QUALIFIER, s_FIGURE_QUALIFIER);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_TIME", g_FIGURE_TIME, s_FIGURE_TIME);
    pyrt::xrefs::reg_access("modules.memory_store", "FIGURE_UNITS", g_FIGURE_UNITS, s_FIGURE_UNITS);
    pyrt::xrefs::reg_access("modules.memory_store", "FIRST_ASK", g_FIRST_ASK, s_FIRST_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "FOOD_PHRASES", g_FOOD_PHRASES, s_FOOD_PHRASES);
    pyrt::xrefs::reg_access("modules.memory_store", "GREET", g_GREET, s_GREET);
    pyrt::xrefs::reg_access("modules.memory_store", "GRIEF_WORDS", g_GRIEF_WORDS, s_GRIEF_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "GROUP_ASK", g_GROUP_ASK, s_GROUP_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "GROUP_NAMES_TYPED", g_GROUP_NAMES_TYPED, s_GROUP_NAMES_TYPED);
    pyrt::xrefs::reg_access("modules.memory_store", "GROUP_NAME_ASK", g_GROUP_NAME_ASK, s_GROUP_NAME_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "GROUP_OF", g_GROUP_OF, s_GROUP_OF);
    pyrt::xrefs::reg_access("modules.memory_store", "GROUP_WORDS", g_GROUP_WORDS, s_GROUP_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "HAVE_ANY", g_HAVE_ANY, s_HAVE_ANY);
    pyrt::xrefs::reg_access("modules.memory_store", "HAVE_ANY_WORDS", g_HAVE_ANY_WORDS, s_HAVE_ANY_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "HELD_THREAD", g_HELD_THREAD, s_HELD_THREAD);
    pyrt::xrefs::reg_access("modules.memory_store", "HER_WORD", g_HER_WORD, s_HER_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "HOW_MANY_OF_MINE", g_HOW_MANY_OF_MINE, s_HOW_MANY_OF_MINE);
    pyrt::xrefs::reg_access("modules.memory_store", "IN_LAW_AFTER_TEXT", g_IN_LAW_AFTER_TEXT, s_IN_LAW_AFTER_TEXT);
    pyrt::xrefs::reg_access("modules.memory_store", "KEPT_THINGS", g_KEPT_THINGS, s_KEPT_THINGS);
    pyrt::xrefs::reg_access("modules.memory_store", "KEY_SLOT", g_KEY_SLOT, s_KEY_SLOT);
    pyrt::xrefs::reg_access("modules.memory_store", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("modules.memory_store", "KIN_ASK", g_KIN_ASK, s_KIN_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "LENGTH", g_LENGTH, s_LENGTH);
    pyrt::xrefs::reg_access("modules.memory_store", "LIMITS", g_LIMITS, s_LIMITS);
    pyrt::xrefs::reg_access("modules.memory_store", "LIMIT_SAY", g_LIMIT_SAY, s_LIMIT_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "LISTED_FIGURE", g_LISTED_FIGURE, s_LISTED_FIGURE);
    pyrt::xrefs::reg_access("modules.memory_store", "LOOK_BACK_FIGURE", g_LOOK_BACK_FIGURE, s_LOOK_BACK_FIGURE);
    pyrt::xrefs::reg_access("modules.memory_store", "MADE_OF_A_VALUE", g_MADE_OF_A_VALUE, s_MADE_OF_A_VALUE);
    pyrt::xrefs::reg_access("modules.memory_store", "MEMORY_POINTER", g_MEMORY_POINTER, s_MEMORY_POINTER);
    pyrt::xrefs::reg_access("modules.memory_store", "MISSED_CATCH_UP", g_MISSED_CATCH_UP, s_MISSED_CATCH_UP);
    pyrt::xrefs::reg_access("modules.memory_store", "MONTH_ALT", g_MONTH_ALT, s_MONTH_ALT);
    pyrt::xrefs::reg_access("modules.memory_store", "MONTH_ORDER", g_MONTH_ORDER, s_MONTH_ORDER);
    pyrt::xrefs::reg_access("modules.memory_store", "MONTH_WORD", g_MONTH_WORD, s_MONTH_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "MONTH_WORDS", g_MONTH_WORDS, s_MONTH_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "NAMED_BY", g_NAMED_BY, s_NAMED_BY);
    pyrt::xrefs::reg_access("modules.memory_store", "NAMED_FACT_ASK", g_NAMED_FACT_ASK, s_NAMED_FACT_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "NAME_ADVICE", g_NAME_ADVICE, s_NAME_ADVICE);
    pyrt::xrefs::reg_access("modules.memory_store", "NAME_AGAIN", g_NAME_AGAIN, s_NAME_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "NAME_FIX", g_NAME_FIX, s_NAME_FIX);
    pyrt::xrefs::reg_access("modules.memory_store", "NAME_FIX_WORDS", g_NAME_FIX_WORDS, s_NAME_FIX_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "NAME_GIVEN", g_NAME_GIVEN, s_NAME_GIVEN);
    pyrt::xrefs::reg_access("modules.memory_store", "NAME_GIVEN_WORDS", g_NAME_GIVEN_WORDS, s_NAME_GIVEN_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "NAME_NOT_A_NAME", g_NAME_NOT_A_NAME, s_NAME_NOT_A_NAME);
    pyrt::xrefs::reg_access("modules.memory_store", "NAMING_VERB", g_NAMING_VERB, s_NAMING_VERB);
    pyrt::xrefs::reg_access("modules.memory_store", "NOTHING_STARTED", g_NOTHING_STARTED, s_NOTHING_STARTED);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_A_FACT", g_NOT_A_FACT, s_NOT_A_FACT);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_A_FACT_OF_THEIRS", g_NOT_A_FACT_OF_THEIRS, s_NOT_A_FACT_OF_THEIRS);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_A_NAME_AFTER_CALLED", g_NOT_A_NAME_AFTER_CALLED, s_NOT_A_NAME_AFTER_CALLED);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_A_NAME_BEFORE", g_NOT_A_NAME_BEFORE, s_NOT_A_NAME_BEFORE);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_A_NEW_VALUE", g_NOT_A_NEW_VALUE, s_NOT_A_NEW_VALUE);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_A_PLACE_VALUE", g_NOT_A_PLACE_VALUE, s_NOT_A_PLACE_VALUE);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_A_UNIT", g_NOT_A_UNIT, s_NOT_A_UNIT);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_FROM_THE_REPAIR", g_NOT_FROM_THE_REPAIR, s_NOT_FROM_THE_REPAIR);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_ON_A_MEDIUM", g_NOT_ON_A_MEDIUM, s_NOT_ON_A_MEDIUM);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_RETRACTED", g_NOT_RETRACTED, s_NOT_RETRACTED);
    pyrt::xrefs::reg_access("modules.memory_store", "NOT_THE_PLAN", g_NOT_THE_PLAN, s_NOT_THE_PLAN);
    pyrt::xrefs::reg_access("modules.memory_store", "NO_PLANS_REPLY", g_NO_PLANS_REPLY, s_NO_PLANS_REPLY);
    pyrt::xrefs::reg_access("modules.memory_store", "NO_RECORD", g_NO_RECORD, s_NO_RECORD);
    pyrt::xrefs::reg_access("modules.memory_store", "NO_SUBJECT_OF_ITS_OWN", g_NO_SUBJECT_OF_ITS_OWN, s_NO_SUBJECT_OF_ITS_OWN);
    pyrt::xrefs::reg_access("modules.memory_store", "NUMBER", g_NUMBER, s_NUMBER);
    pyrt::xrefs::reg_access("modules.memory_store", "NUMBER_DIGITS", g_NUMBER_DIGITS, s_NUMBER_DIGITS);
    pyrt::xrefs::reg_access("modules.memory_store", "NUMBER_VALUE", g_NUMBER_VALUE, s_NUMBER_VALUE);
    pyrt::xrefs::reg_access("modules.memory_store", "NUMBER_WORD", g_NUMBER_WORD, s_NUMBER_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "OTHER_BARE_NUMBER", g_OTHER_BARE_NUMBER, s_OTHER_BARE_NUMBER);
    pyrt::xrefs::reg_access("modules.memory_store", "OTHER_FACT", g_OTHER_FACT, s_OTHER_FACT);
    pyrt::xrefs::reg_access("modules.memory_store", "OTHER_KIND_SAY", g_OTHER_KIND_SAY, s_OTHER_KIND_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "OTHER_NOUNS", g_OTHER_NOUNS, s_OTHER_NOUNS);
    pyrt::xrefs::reg_access("modules.memory_store", "OTHER_PLACE", g_OTHER_PLACE, s_OTHER_PLACE);
    pyrt::xrefs::reg_access("modules.memory_store", "OTHER_PLACE_PRONOUN", g_OTHER_PLACE_PRONOUN, s_OTHER_PLACE_PRONOUN);
    pyrt::xrefs::reg_access("modules.memory_store", "OTHER_SAID", g_OTHER_SAID, s_OTHER_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "OWNED_BY_THE_VERB", g_OWNED_BY_THE_VERB, s_OWNED_BY_THE_VERB);
    pyrt::xrefs::reg_access("modules.memory_store", "PAIR_WITHOUT_A_MARKER", g_PAIR_WITHOUT_A_MARKER, s_PAIR_WITHOUT_A_MARKER);
    pyrt::xrefs::reg_access("modules.memory_store", "PART_HELD", g_PART_HELD, s_PART_HELD);
    pyrt::xrefs::reg_access("modules.memory_store", "PART_KEYS", g_PART_KEYS, s_PART_KEYS);
    pyrt::xrefs::reg_access("modules.memory_store", "PERSON_ADVICE_ASK", g_PERSON_ADVICE_ASK, s_PERSON_ADVICE_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "PERSON_AFTER", g_PERSON_AFTER, s_PERSON_AFTER);
    pyrt::xrefs::reg_access("modules.memory_store", "PERSON_GROUP", g_PERSON_GROUP, s_PERSON_GROUP);
    pyrt::xrefs::reg_access("modules.memory_store", "PET_AGE_ANSWER", g_PET_AGE_ANSWER, s_PET_AGE_ANSWER);
    pyrt::xrefs::reg_access("modules.memory_store", "PET_AGE_FIX_WORDS", g_PET_AGE_FIX_WORDS, s_PET_AGE_FIX_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "PET_ASK", g_PET_ASK, s_PET_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "PET_LEAD_IN", g_PET_LEAD_IN, s_PET_LEAD_IN);
    pyrt::xrefs::reg_access("modules.memory_store", "PET_NAME_AGE", g_PET_NAME_AGE, s_PET_NAME_AGE);
    pyrt::xrefs::reg_access("modules.memory_store", "PET_RENAMED_SAY", g_PET_RENAMED_SAY, s_PET_RENAMED_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "PET_WORDS", g_PET_WORDS, s_PET_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "PLACES", g_PLACES, s_PLACES);
    pyrt::xrefs::reg_access("modules.memory_store", "PLACE_ENDS", g_PLACE_ENDS, s_PLACE_ENDS);
    pyrt::xrefs::reg_access("modules.memory_store", "PLACE_NOT", g_PLACE_NOT, s_PLACE_NOT);
    pyrt::xrefs::reg_access("modules.memory_store", "PLAN_ASKS", g_PLAN_ASKS, s_PLAN_ASKS);
    pyrt::xrefs::reg_access("modules.memory_store", "PLURAL_ANIMAL", g_PLURAL_ANIMAL, s_PLURAL_ANIMAL);
    pyrt::xrefs::reg_access("modules.memory_store", "PLURAL_PEOPLE", g_PLURAL_PEOPLE, s_PLURAL_PEOPLE);
    pyrt::xrefs::reg_access("modules.memory_store", "PLURAL_PETS", g_PLURAL_PETS, s_PLURAL_PETS);
    pyrt::xrefs::reg_access("modules.memory_store", "POINTING_WORD_NOTHING", g_POINTING_WORD_NOTHING, s_POINTING_WORD_NOTHING);
    pyrt::xrefs::reg_access("modules.memory_store", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("modules.memory_store", "POINTS_BACK_AT_IT", g_POINTS_BACK_AT_IT, s_POINTS_BACK_AT_IT);
    pyrt::xrefs::reg_access("modules.memory_store", "POINTS_BACK_NOTHING", g_POINTS_BACK_NOTHING, s_POINTS_BACK_NOTHING);
    pyrt::xrefs::reg_access("modules.memory_store", "POSSESSIVE", g_POSSESSIVE, s_POSSESSIVE);
    pyrt::xrefs::reg_access("modules.memory_store", "PREGNANCY_SAID_AGAIN", g_PREGNANCY_SAID_AGAIN, s_PREGNANCY_SAID_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "PREGNANT_OWN_SAY", g_PREGNANT_OWN_SAY, s_PREGNANT_OWN_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "PREGNANT_SAY", g_PREGNANT_SAY, s_PREGNANT_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "PREGNANT_WEEKS_SAY", g_PREGNANT_WEEKS_SAY, s_PREGNANT_WEEKS_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.memory_store", "PRONOUN_FITS", g_PRONOUN_FITS, s_PRONOUN_FITS);
    pyrt::xrefs::reg_access("modules.memory_store", "PRONOUN_PEOPLE", g_PRONOUN_PEOPLE, s_PRONOUN_PEOPLE);
    pyrt::xrefs::reg_access("modules.memory_store", "QUOTE_LEADS", g_QUOTE_LEADS, s_QUOTE_LEADS);
    pyrt::xrefs::reg_access("modules.memory_store", "RECALL_HINTS", g_RECALL_HINTS, s_RECALL_HINTS);
    pyrt::xrefs::reg_access("modules.memory_store", "RECALL_NAMES_NOTHING", g_RECALL_NAMES_NOTHING, s_RECALL_NAMES_NOTHING);
    pyrt::xrefs::reg_access("modules.memory_store", "RECALL_POSSESSIVE", g_RECALL_POSSESSIVE, s_RECALL_POSSESSIVE);
    pyrt::xrefs::reg_access("modules.memory_store", "RECALL_SLOTS", g_RECALL_SLOTS, s_RECALL_SLOTS);
    pyrt::xrefs::reg_access("modules.memory_store", "RECALL_SLOTS_BY_POSSESSIVE", g_RECALL_SLOTS_BY_POSSESSIVE, s_RECALL_SLOTS_BY_POSSESSIVE);
    pyrt::xrefs::reg_access("modules.memory_store", "RECALL_WRAPPER", g_RECALL_WRAPPER, s_RECALL_WRAPPER);
    pyrt::xrefs::reg_access("modules.memory_store", "RELATION_ALIKE", g_RELATION_ALIKE, s_RELATION_ALIKE);
    pyrt::xrefs::reg_access("modules.memory_store", "RELATIVE_MONTH", g_RELATIVE_MONTH, s_RELATIVE_MONTH);
    pyrt::xrefs::reg_access("modules.memory_store", "REMIND_QUESTION", g_REMIND_QUESTION, s_REMIND_QUESTION);
    pyrt::xrefs::reg_access("modules.memory_store", "REPEAT_ADVERB", g_REPEAT_ADVERB, s_REPEAT_ADVERB);
    pyrt::xrefs::reg_access("modules.memory_store", "REPEAT_WORDS", g_REPEAT_WORDS, s_REPEAT_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "RESTATED_KEYS", g_RESTATED_KEYS, s_RESTATED_KEYS);
    pyrt::xrefs::reg_access("modules.memory_store", "RESTRICT_OPENS", g_RESTRICT_OPENS, s_RESTRICT_OPENS);
    pyrt::xrefs::reg_access("modules.memory_store", "RESUME", g_RESUME, s_RESUME);
    pyrt::xrefs::reg_access("modules.memory_store", "RETRACT_SAY", g_RETRACT_SAY, s_RETRACT_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "ROUTINE_SAY_BACK", g_ROUTINE_SAY_BACK, s_ROUTINE_SAY_BACK);
    pyrt::xrefs::reg_access("modules.memory_store", "ROUTINE_SUBJECT", g_ROUTINE_SUBJECT, s_ROUTINE_SUBJECT);
    pyrt::xrefs::reg_access("modules.memory_store", "SAID_AGAIN", g_SAID_AGAIN, s_SAID_AGAIN);
    pyrt::xrefs::reg_access("modules.memory_store", "SAID_ASK", g_SAID_ASK, s_SAID_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "SAID_AS_A_SENTENCE", g_SAID_AS_A_SENTENCE, s_SAID_AS_A_SENTENCE);
    pyrt::xrefs::reg_access("modules.memory_store", "SAME_FACT_THEIR_OWN", g_SAME_FACT_THEIR_OWN, s_SAME_FACT_THEIR_OWN);
    pyrt::xrefs::reg_access("modules.memory_store", "SELF", g_SELF, s_SELF);
    pyrt::xrefs::reg_access("modules.memory_store", "SHE_OR_HE", g_SHE_OR_HE, s_SHE_OR_HE);
    pyrt::xrefs::reg_access("modules.memory_store", "SLOT_KEY", g_SLOT_KEY, s_SLOT_KEY);
    pyrt::xrefs::reg_access("modules.memory_store", "SLOT_WORDS", g_SLOT_WORDS, s_SLOT_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "SMALL_NAMES", g_SMALL_NAMES, s_SMALL_NAMES);
    pyrt::xrefs::reg_access("modules.memory_store", "SORRY_OPENS_A_CORRECTION", g_SORRY_OPENS_A_CORRECTION, s_SORRY_OPENS_A_CORRECTION);
    pyrt::xrefs::reg_access("modules.memory_store", "SPELLING_APART", g_SPELLING_APART, s_SPELLING_APART);
    pyrt::xrefs::reg_access("modules.memory_store", "STATES_A_LIMIT", g_STATES_A_LIMIT, s_STATES_A_LIMIT);
    pyrt::xrefs::reg_access("modules.memory_store", "STATE_WAS_SAY", g_STATE_WAS_SAY, s_STATE_WAS_SAY);
    pyrt::xrefs::reg_access("modules.memory_store", "STILL_THE_SAME_PERSON", g_STILL_THE_SAME_PERSON, s_STILL_THE_SAME_PERSON);
    pyrt::xrefs::reg_access("modules.memory_store", "STOPWORDS", g_STOPWORDS, s_STOPWORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "SWAPPABLE", g_SWAPPABLE, s_SWAPPABLE);
    pyrt::xrefs::reg_access("modules.memory_store", "TAIL_WINDOW", g_TAIL_WINDOW, s_TAIL_WINDOW);
    pyrt::xrefs::reg_access("modules.memory_store", "TASTE_ASKED_ABOUT", g_TASTE_ASKED_ABOUT, s_TASTE_ASKED_ABOUT);
    pyrt::xrefs::reg_access("modules.memory_store", "THAT_DATE", g_THAT_DATE, s_THAT_DATE);
    pyrt::xrefs::reg_access("modules.memory_store", "THEIR_AGE", g_THEIR_AGE, s_THEIR_AGE);
    pyrt::xrefs::reg_access("modules.memory_store", "THEIR_JOB", g_THEIR_JOB, s_THEIR_JOB);
    pyrt::xrefs::reg_access("modules.memory_store", "THEIR_NAME", g_THEIR_NAME, s_THEIR_NAME);
    pyrt::xrefs::reg_access("modules.memory_store", "THEIR_TASTE_A_YES_NO", g_THEIR_TASTE_A_YES_NO, s_THEIR_TASTE_A_YES_NO);
    pyrt::xrefs::reg_access("modules.memory_store", "THEIR_TASTE_INTO", g_THEIR_TASTE_INTO, s_THEIR_TASTE_INTO);
    pyrt::xrefs::reg_access("modules.memory_store", "THEIR_TASTE_WHAT", g_THEIR_TASTE_WHAT, s_THEIR_TASTE_WHAT);
    pyrt::xrefs::reg_access("modules.memory_store", "THEM", g_THEM, s_THEM);
    pyrt::xrefs::reg_access("modules.memory_store", "THEY_ARE_NAMED", g_THEY_ARE_NAMED, s_THEY_ARE_NAMED);
    pyrt::xrefs::reg_access("modules.memory_store", "THE_KIDS", g_THE_KIDS, s_THE_KIDS);
    pyrt::xrefs::reg_access("modules.memory_store", "THE_PLACE_IS_NAMED", g_THE_PLACE_IS_NAMED, s_THE_PLACE_IS_NAMED);
    pyrt::xrefs::reg_access("modules.memory_store", "TOO_AS_WELL", g_TOO_AS_WELL, s_TOO_AS_WELL);
    pyrt::xrefs::reg_access("modules.memory_store", "TURNED", g_TURNED, s_TURNED);
    pyrt::xrefs::reg_access("modules.memory_store", "UNIT_FAMILY", g_UNIT_FAMILY, s_UNIT_FAMILY);
    pyrt::xrefs::reg_access("modules.memory_store", "UNIT_SECONDS", g_UNIT_SECONDS, s_UNIT_SECONDS);
    pyrt::xrefs::reg_access("modules.memory_store", "WEEK", g_WEEK, s_WEEK);
    pyrt::xrefs::reg_access("modules.memory_store", "WEEKDAY_WORD", g_WEEKDAY_WORD, s_WEEKDAY_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "WEEKS_FIX", g_WEEKS_FIX, s_WEEKS_FIX);
    pyrt::xrefs::reg_access("modules.memory_store", "WHAT_I_SAID", g_WHAT_I_SAID, s_WHAT_I_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "WHAT_I_SAID_NOTHING", g_WHAT_I_SAID_NOTHING, s_WHAT_I_SAID_NOTHING);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_ASKS", g_WHEN_ASKS, s_WHEN_ASKS);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_END_ASK", g_WHEN_END_ASK, s_WHEN_END_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_END_WORD", g_WHEN_END_WORD, s_WHEN_END_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_INVERTED", g_WHEN_INVERTED, s_WHEN_INVERTED);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_INVERTED_SUBJECT", g_WHEN_INVERTED_SUBJECT, s_WHEN_INVERTED_SUBJECT);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_PLAN", g_WHEN_PLAN, s_WHEN_PLAN);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_QUALITY", g_WHEN_QUALITY, s_WHEN_QUALITY);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_START_ASK", g_WHEN_START_ASK, s_WHEN_START_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_THING", g_WHEN_THING, s_WHEN_THING);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_THING_VERB", g_WHEN_THING_VERB, s_WHEN_THING_VERB);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_VAGUE", g_WHEN_VAGUE, s_WHEN_VAGUE);
    pyrt::xrefs::reg_access("modules.memory_store", "WHEN_VERB", g_WHEN_VERB, s_WHEN_VERB);
    pyrt::xrefs::reg_access("modules.memory_store", "WHERE_ASK", g_WHERE_ASK, s_WHERE_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "WHOLE_ASK", g_WHOLE_ASK, s_WHOLE_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "WORD_NUMBERS", g_WORD_NUMBERS, s_WORD_NUMBERS);
    pyrt::xrefs::reg_access("modules.memory_store", "WORK_ON_IT", g_WORK_ON_IT, s_WORK_ON_IT);
    pyrt::xrefs::reg_access("modules.memory_store", "YES_NO_ASK", g_YES_NO_ASK, s_YES_NO_ASK);
    pyrt::xrefs::reg_access("modules.memory_store", "_ABOUT_WHO", g__ABOUT_WHO, s__ABOUT_WHO);
    pyrt::xrefs::reg_access("modules.memory_store", "_AMOUNTS", g__AMOUNTS, s__AMOUNTS);
    pyrt::xrefs::reg_access("modules.memory_store", "_AMOUNT_WORDS", g__AMOUNT_WORDS, s__AMOUNT_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "_CONVERT", g__CONVERT, s__CONVERT);
    pyrt::xrefs::reg_access("modules.memory_store", "_COUNT_WORD", g__COUNT_WORD, s__COUNT_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "_FACT_SLOT", g__FACT_SLOT, s__FACT_SLOT);
    pyrt::xrefs::reg_access("modules.memory_store", "_HEDGE", g__HEDGE, s__HEDGE);
    pyrt::xrefs::reg_access("modules.memory_store", "_PET_WORD", g__PET_WORD, s__PET_WORD);
    pyrt::xrefs::reg_access("modules.memory_store", "_PLURAL", g__PLURAL, s__PLURAL);
    pyrt::xrefs::reg_access("modules.memory_store", "_RELATION", g__RELATION, s__RELATION);
    pyrt::xrefs::reg_access("modules.memory_store", "_SUBJECT_KEYS", g__SUBJECT_KEYS, s__SUBJECT_KEYS);
    pyrt::xrefs::reg_access("modules.memory_store", "_TENS_WORDS", g__TENS_WORDS, s__TENS_WORDS);
    pyrt::xrefs::reg_access("modules.memory_store", "_UNIT_SAID", g__UNIT_SAID, s__UNIT_SAID);
    pyrt::xrefs::reg_access("modules.memory_store", "_WEEKLY", g__WEEKLY, s__WEEKLY);
    pyrt::xrefs::reg_access("modules.memory_store", "_concept", g__concept, s__concept);
    pyrt::xrefs::reg_access("modules.memory_store", "_country", g__country, s__country);
    pyrt::xrefs::reg_access("modules.memory_store", "_enough_reply", g__enough_reply, s__enough_reply);
    pyrt::xrefs::reg_access("modules.memory_store", "_word", g__word, s__word);
    pyrt::xrefs::reg_access("modules.memory_store", "_words", g__words, s__words);
}
