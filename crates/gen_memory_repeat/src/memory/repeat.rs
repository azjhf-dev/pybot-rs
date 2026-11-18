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


thread_local! { static G_ABOUT_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACK_AND_REST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACK_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGREEING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGREEMENT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGREES_IN_A_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGREES_ON_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALSO_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AND_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWER_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_TO_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ATTEMPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_A_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_AMOUNT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_DURATION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_OF_MORE_THAN_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DOING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOAL_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GROUP_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAS_THE_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOW_LONG_ITS_BEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_IM_ON_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LINE_THAT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIST_OF_AGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MARKER_AT_THE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEASURE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MENU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOVE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NARROWING_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NARROWING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NUMBER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ONE_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAY_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAY_VERB_AT_THE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLURAL_ONLY_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POINTS_BACK_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUALITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_IN_THE_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_OF_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECALL_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SCHEDULED_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STORE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TAKE_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THE_PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NAMED_AFTER_THE_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TOLD_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WERE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ASK_NEEDS_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BE_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREVITY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BROODING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIMS_IT_WAS_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_SPLIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSES_THE_TURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_MID_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DANGLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLARATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIGIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DO_I_NEED_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ECHO_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAVE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAVE_LOOK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOAL_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP_ASK_QUESTION_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDEAS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDEAS_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IRREGULAR_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_A_NOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADING_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADING_PHRASE_MAX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_SAYS_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_SPLIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LESS_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LESS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIST_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK_FOR_LESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOSS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOST_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_METHOD_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_AFTER_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEEDS_COMPLEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEXT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEXT_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEXT_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEXT_STEP_EMERGENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_TO_CUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FIGURE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_TRIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOW_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_LONGER_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_SHORTER_STILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECT_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_LEAD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_STAND_INS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAY_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAY_WHEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIOD_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PIECE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_RIGHT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_RIGHT_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_RIGHT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTIONS_THAT_ARE_NOT_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTE_PAIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFER_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REJOINDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESET_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_SAFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAY_AGAIN_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_ENOUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSES_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STANDS_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTING_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTING_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTING_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTING_WHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_START_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_SAYS_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_WITH_ITS_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_SAFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_SAME_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USE_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WELCOMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_IT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_TO_LOOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_TO_LOOK_DEFAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_TO_LOOK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_TO_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHETHER_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_OF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORDS_WANTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_A_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_AMOUNT_WANTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ENTRY_IS_LOOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXPLANATION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRAME_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HANGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_PAID_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEGATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWNS_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SO_COMES_THE_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SO_DOES_NOT_ASK_SOMETHING_OF_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SO_THE_STEP_FOLLOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEIGHS_ONE_AGAINST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_whole_cut: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_handed_back: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_recent_answers: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_said_recently: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_same_subject: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_understand: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.repeat: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_THE_BOT() -> Py { ensure_init(); G_ABOUT_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_BOT(v: Py) { ensure_init(); G_ABOUT_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_ACK_AND_REST() -> Py { ensure_init(); G_ACK_AND_REST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK_AND_REST(v: Py) { ensure_init(); G_ACK_AND_REST.with(|c| *c.borrow_mut() = v); }
pub fn g_ACK_OPENS() -> Py { ensure_init(); G_ACK_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK_OPENS(v: Py) { ensure_init(); G_ACK_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGREEING() -> Py { ensure_init(); G_AGREEING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREEING(v: Py) { ensure_init(); G_AGREEING.with(|c| *c.borrow_mut() = v); }
pub fn g_AGREEMENT_TAIL() -> Py { ensure_init(); G_AGREEMENT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREEMENT_TAIL(v: Py) { ensure_init(); G_AGREEMENT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_AGREES_IN_A_PHRASE() -> Py { ensure_init(); G_AGREES_IN_A_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREES_IN_A_PHRASE(v: Py) { ensure_init(); G_AGREES_IN_A_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_AGREES_ON_ITS_OWN() -> Py { ensure_init(); G_AGREES_ON_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREES_ON_ITS_OWN(v: Py) { ensure_init(); G_AGREES_ON_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_SHORT() -> Py { ensure_init(); G_ALREADY_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_SHORT(v: Py) { ensure_init(); G_ALREADY_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_ALSO_LEADS() -> Py { ensure_init(); G_ALSO_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALSO_LEADS(v: Py) { ensure_init(); G_ALSO_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_AND_NOW() -> Py { ensure_init(); G_AND_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AND_NOW(v: Py) { ensure_init(); G_AND_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWER_MODULES() -> Py { ensure_init(); G_ANSWER_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWER_MODULES(v: Py) { ensure_init(); G_ANSWER_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ARTICLE() -> Py { ensure_init(); G_AN_ARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ARTICLE(v: Py) { ensure_init(); G_AN_ARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_THEM() -> Py { ensure_init(); G_ASKING_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_THEM(v: Py) { ensure_init(); G_ASKING_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_TO_START() -> Py { ensure_init(); G_ASKING_TO_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_TO_START(v: Py) { ensure_init(); G_ASKING_TO_START.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FRAME() -> Py { ensure_init(); G_ASK_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FRAME(v: Py) { ensure_init(); G_ASK_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_VERB() -> Py { ensure_init(); G_ASK_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_VERB(v: Py) { ensure_init(); G_ASK_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_ATTEMPT() -> Py { ensure_init(); G_ATTEMPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ATTEMPT(v: Py) { ensure_init(); G_ATTEMPT.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_A_LOSS() -> Py { ensure_init(); G_AT_A_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_A_LOSS(v: Py) { ensure_init(); G_AT_A_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_AMOUNT_ASK() -> Py { ensure_init(); G_A_BARE_AMOUNT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_AMOUNT_ASK(v: Py) { ensure_init(); G_A_BARE_AMOUNT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_DURATION_ASK() -> Py { ensure_init(); G_A_BARE_DURATION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_DURATION_ASK(v: Py) { ensure_init(); G_A_BARE_DURATION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_TIME() -> Py { ensure_init(); G_A_BARE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_TIME(v: Py) { ensure_init(); G_A_BARE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CORRECTION() -> Py { ensure_init(); G_A_CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CORRECTION(v: Py) { ensure_init(); G_A_CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_OF_MORE_THAN_ONE() -> Py { ensure_init(); G_A_COUNT_OF_MORE_THAN_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_OF_MORE_THAN_ONE(v: Py) { ensure_init(); G_A_COUNT_OF_MORE_THAN_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DOING_WORD() -> Py { ensure_init(); G_A_DOING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOING_WORD(v: Py) { ensure_init(); G_A_DOING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_WORD() -> Py { ensure_init(); G_A_FIGURE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_WORD(v: Py) { ensure_init(); G_A_FIGURE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GAIN() -> Py { ensure_init(); G_A_GAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GAIN(v: Py) { ensure_init(); G_A_GAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOAL_LINE() -> Py { ensure_init(); G_A_GOAL_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOAL_LINE(v: Py) { ensure_init(); G_A_GOAL_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GROUP_WORD() -> Py { ensure_init(); G_A_GROUP_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GROUP_WORD(v: Py) { ensure_init(); G_A_GROUP_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAS_THE_LENGTH() -> Py { ensure_init(); G_A_HAS_THE_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAS_THE_LENGTH(v: Py) { ensure_init(); G_A_HAS_THE_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOW_LONG_ITS_BEEN() -> Py { ensure_init(); G_A_HOW_LONG_ITS_BEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOW_LONG_ITS_BEEN(v: Py) { ensure_init(); G_A_HOW_LONG_ITS_BEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_IM_ON_A_FIGURE() -> Py { ensure_init(); G_A_IM_ON_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_IM_ON_A_FIGURE(v: Py) { ensure_init(); G_A_IM_ON_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_ASK() -> Py { ensure_init(); G_A_LENGTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_ASK(v: Py) { ensure_init(); G_A_LENGTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LINE_THAT_ASKS() -> Py { ensure_init(); G_A_LINE_THAT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LINE_THAT_ASKS(v: Py) { ensure_init(); G_A_LINE_THAT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIST_OF_AGES() -> Py { ensure_init(); G_A_LIST_OF_AGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIST_OF_AGES(v: Py) { ensure_init(); G_A_LIST_OF_AGES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOSS() -> Py { ensure_init(); G_A_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOSS(v: Py) { ensure_init(); G_A_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MARKER_AT_THE_END() -> Py { ensure_init(); G_A_MARKER_AT_THE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MARKER_AT_THE_END(v: Py) { ensure_init(); G_A_MARKER_AT_THE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEASURE_WORD() -> Py { ensure_init(); G_A_MEASURE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEASURE_WORD(v: Py) { ensure_init(); G_A_MEASURE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MENU() -> Py { ensure_init(); G_A_MENU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MENU(v: Py) { ensure_init(); G_A_MENU.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOVE() -> Py { ensure_init(); G_A_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOVE(v: Py) { ensure_init(); G_A_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOVE_WORDS() -> Py { ensure_init(); G_A_MOVE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOVE_WORDS(v: Py) { ensure_init(); G_A_MOVE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NARROWING_ONLY() -> Py { ensure_init(); G_A_NARROWING_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NARROWING_ONLY(v: Py) { ensure_init(); G_A_NARROWING_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NARROWING_WORDS() -> Py { ensure_init(); G_A_NARROWING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NARROWING_WORDS(v: Py) { ensure_init(); G_A_NARROWING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NUMBER_WORD() -> Py { ensure_init(); G_A_NUMBER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NUMBER_WORD(v: Py) { ensure_init(); G_A_NUMBER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ONE_MORE() -> Py { ensure_init(); G_A_ONE_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ONE_MORE(v: Py) { ensure_init(); G_A_ONE_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAY_JUDGEMENT() -> Py { ensure_init(); G_A_PAY_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAY_JUDGEMENT(v: Py) { ensure_init(); G_A_PAY_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAY_VERB_AT_THE_END() -> Py { ensure_init(); G_A_PAY_VERB_AT_THE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAY_VERB_AT_THE_END(v: Py) { ensure_init(); G_A_PAY_VERB_AT_THE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAY_WORD() -> Py { ensure_init(); G_A_PAY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAY_WORD(v: Py) { ensure_init(); G_A_PAY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN_ASK() -> Py { ensure_init(); G_A_PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_ASK(v: Py) { ensure_init(); G_A_PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLURAL_ONLY_NOUN() -> Py { ensure_init(); G_A_PLURAL_ONLY_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLURAL_ONLY_NOUN(v: Py) { ensure_init(); G_A_PLURAL_ONLY_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POINTS_BACK_HARD() -> Py { ensure_init(); G_A_POINTS_BACK_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POINTS_BACK_HARD(v: Py) { ensure_init(); G_A_POINTS_BACK_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_ASK() -> Py { ensure_init(); G_A_PRICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_ASK(v: Py) { ensure_init(); G_A_PRICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUALITY() -> Py { ensure_init(); G_A_QUALITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUALITY(v: Py) { ensure_init(); G_A_QUALITY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION() -> Py { ensure_init(); G_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION(v: Py) { ensure_init(); G_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_IN_THE_FIGURE() -> Py { ensure_init(); G_A_RATE_IN_THE_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_IN_THE_FIGURE(v: Py) { ensure_init(); G_A_RATE_IN_THE_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_LINE() -> Py { ensure_init(); G_A_RATE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_LINE(v: Py) { ensure_init(); G_A_RATE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_OF_TIME() -> Py { ensure_init(); G_A_RATE_OF_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_OF_TIME(v: Py) { ensure_init(); G_A_RATE_OF_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECALL_CUE() -> Py { ensure_init(); G_A_RECALL_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECALL_CUE(v: Py) { ensure_init(); G_A_RECALL_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SCHEDULED_AFTER() -> Py { ensure_init(); G_A_SCHEDULED_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SCHEDULED_AFTER(v: Py) { ensure_init(); G_A_SCHEDULED_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STORE_ANSWER() -> Py { ensure_init(); G_A_STORE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STORE_ANSWER(v: Py) { ensure_init(); G_A_STORE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TAKE_HOME() -> Py { ensure_init(); G_A_TAKE_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TAKE_HOME(v: Py) { ensure_init(); G_A_TAKE_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THE_PLAN_ASK() -> Py { ensure_init(); G_A_THE_PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THE_PLAN_ASK(v: Py) { ensure_init(); G_A_THE_PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NAMED_AFTER_THE_CUE() -> Py { ensure_init(); G_A_THING_NAMED_AFTER_THE_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED_AFTER_THE_CUE(v: Py) { ensure_init(); G_A_THING_NAMED_AFTER_THE_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TOLD_BACK() -> Py { ensure_init(); G_A_TOLD_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TOLD_BACK(v: Py) { ensure_init(); G_A_TOLD_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WERE_ASKS() -> Py { ensure_init(); G_A_WERE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WERE_ASKS(v: Py) { ensure_init(); G_A_WERE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ASK() -> Py { ensure_init(); G_BARE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ASK(v: Py) { ensure_init(); G_BARE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ASK_NEEDS_A_QUESTION() -> Py { ensure_init(); G_BARE_ASK_NEEDS_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ASK_NEEDS_A_QUESTION(v: Py) { ensure_init(); G_BARE_ASK_NEEDS_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_BE_FORM() -> Py { ensure_init(); G_BE_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BE_FORM(v: Py) { ensure_init(); G_BE_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_BREVITY_WORDS() -> Py { ensure_init(); G_BREVITY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREVITY_WORDS(v: Py) { ensure_init(); G_BREVITY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BROODING() -> Py { ensure_init(); G_BROODING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROODING(v: Py) { ensure_init(); G_BROODING.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_IT() -> Py { ensure_init(); G_CHANGED_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_IT(v: Py) { ensure_init(); G_CHANGED_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_ASK() -> Py { ensure_init(); G_CHANGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_ASK(v: Py) { ensure_init(); G_CHANGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_LEADS() -> Py { ensure_init(); G_CHANGE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_LEADS(v: Py) { ensure_init(); G_CHANGE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIMS_IT_WAS_SAID() -> Py { ensure_init(); G_CLAIMS_IT_WAS_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIMS_IT_WAS_SAID(v: Py) { ensure_init(); G_CLAIMS_IT_WAS_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_SPLIT() -> Py { ensure_init(); G_CLAUSE_SPLIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_SPLIT(v: Py) { ensure_init(); G_CLAUSE_SPLIT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSES_THE_TURN() -> Py { ensure_init(); G_CLOSES_THE_TURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSES_THE_TURN(v: Py) { ensure_init(); G_CLOSES_THE_TURN.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_CLAUSE() -> Py { ensure_init(); G_CONDITION_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_CLAUSE(v: Py) { ensure_init(); G_CONDITION_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_ASK() -> Py { ensure_init(); G_COUNT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_ASK(v: Py) { ensure_init(); G_COUNT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_MID_SENTENCE() -> Py { ensure_init(); G_CUT_MID_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_MID_SENTENCE(v: Py) { ensure_init(); G_CUT_MID_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_WORDS() -> Py { ensure_init(); G_CUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_WORDS(v: Py) { ensure_init(); G_CUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DANGLING() -> Py { ensure_init(); G_DANGLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DANGLING(v: Py) { ensure_init(); G_DANGLING.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLARATIVE() -> Py { ensure_init(); G_DECLARATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLARATIVE(v: Py) { ensure_init(); G_DECLARATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_DIGIT() -> Py { ensure_init(); G_DIGIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIGIT(v: Py) { ensure_init(); G_DIGIT.with(|c| *c.borrow_mut() = v); }
pub fn g_DOINGS() -> Py { ensure_init(); G_DOINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOINGS(v: Py) { ensure_init(); G_DOINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE() -> Py { ensure_init(); G_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE(v: Py) { ensure_init(); G_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_DO_I_NEED_TO() -> Py { ensure_init(); G_DO_I_NEED_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DO_I_NEED_TO(v: Py) { ensure_init(); G_DO_I_NEED_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_ECHO_FRAME() -> Py { ensure_init(); G_ECHO_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ECHO_FRAME(v: Py) { ensure_init(); G_ECHO_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURES() -> Py { ensure_init(); G_FIGURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURES(v: Py) { ensure_init(); G_FIGURES.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_ASKS() -> Py { ensure_init(); G_FIGURE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_ASKS(v: Py) { ensure_init(); G_FIGURE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_NOUNS() -> Py { ensure_init(); G_FIGURE_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_NOUNS(v: Py) { ensure_init(); G_FIGURE_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_SAYING() -> Py { ensure_init(); G_FIGURE_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_SAYING(v: Py) { ensure_init(); G_FIGURE_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_TAIL() -> Py { ensure_init(); G_FIGURE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_TAIL(v: Py) { ensure_init(); G_FIGURE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_WORDS() -> Py { ensure_init(); G_FIGURE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_WORDS(v: Py) { ensure_init(); G_FIGURE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_THING() -> Py { ensure_init(); G_FIRST_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_THING(v: Py) { ensure_init(); G_FIRST_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_GAVE_ASKS() -> Py { ensure_init(); G_GAVE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAVE_ASKS(v: Py) { ensure_init(); G_GAVE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_GAVE_LOOK_BACK() -> Py { ensure_init(); G_GAVE_LOOK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAVE_LOOK_BACK(v: Py) { ensure_init(); G_GAVE_LOOK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_GOAL_NOUNS() -> Py { ensure_init(); G_GOAL_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOAL_NOUNS(v: Py) { ensure_init(); G_GOAL_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEDGE() -> Py { ensure_init(); G_HEDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEDGE(v: Py) { ensure_init(); G_HEDGE.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP_ASK_QUESTION_FIRST() -> Py { ensure_init(); G_HELP_ASK_QUESTION_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK_QUESTION_FIRST(v: Py) { ensure_init(); G_HELP_ASK_QUESTION_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP_SHAPE() -> Py { ensure_init(); G_HELP_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP_SHAPE(v: Py) { ensure_init(); G_HELP_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_IT() -> Py { ensure_init(); G_HOW_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_IT(v: Py) { ensure_init(); G_HOW_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO() -> Py { ensure_init(); G_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); G_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_IDEAS_ASK() -> Py { ensure_init(); G_IDEAS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDEAS_ASK(v: Py) { ensure_init(); G_IDEAS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_IDEAS_ONLY() -> Py { ensure_init(); G_IDEAS_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDEAS_ONLY(v: Py) { ensure_init(); G_IDEAS_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_IRREGULAR_PAST() -> Py { ensure_init(); G_IRREGULAR_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR_PAST(v: Py) { ensure_init(); G_IRREGULAR_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINERS() -> Py { ensure_init(); G_JOINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINERS(v: Py) { ensure_init(); G_JOINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_A_NOD() -> Py { ensure_init(); G_JUST_A_NOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_A_NOD(v: Py) { ensure_init(); G_JUST_A_NOD.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_THIS() -> Py { ensure_init(); G_JUST_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_THIS(v: Py) { ensure_init(); G_JUST_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPING() -> Py { ensure_init(); G_KEEPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPING(v: Py) { ensure_init(); G_KEEPING.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_CLAUSE() -> Py { ensure_init(); G_KEEP_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_CLAUSE(v: Py) { ensure_init(); G_KEEP_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADING_PHRASE() -> Py { ensure_init(); G_LEADING_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADING_PHRASE(v: Py) { ensure_init(); G_LEADING_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADING_PHRASE_MAX() -> Py { ensure_init(); G_LEADING_PHRASE_MAX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADING_PHRASE_MAX(v: Py) { ensure_init(); G_LEADING_PHRASE_MAX.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS() -> Py { ensure_init(); G_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); G_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_IN() -> Py { ensure_init(); G_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); G_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_SAYS_FIRST() -> Py { ensure_init(); G_LEAD_SAYS_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_SAYS_FIRST(v: Py) { ensure_init(); G_LEAD_SAYS_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_SPLIT() -> Py { ensure_init(); G_LEAD_SPLIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_SPLIT(v: Py) { ensure_init(); G_LEAD_SPLIT.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_BACK() -> Py { ensure_init(); G_LENGTH_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_BACK(v: Py) { ensure_init(); G_LENGTH_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_LESS() -> Py { ensure_init(); G_LESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LESS(v: Py) { ensure_init(); G_LESS.with(|c| *c.borrow_mut() = v); }
pub fn g_LESS_LEADS() -> Py { ensure_init(); G_LESS_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LESS_LEADS(v: Py) { ensure_init(); G_LESS_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_LESS_WORDS() -> Py { ensure_init(); G_LESS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LESS_WORDS(v: Py) { ensure_init(); G_LESS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIST_LEAD() -> Py { ensure_init(); G_LIST_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIST_LEAD(v: Py) { ensure_init(); G_LIST_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK() -> Py { ensure_init(); G_LOOK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); G_LOOK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK_FOR_LESS() -> Py { ensure_init(); G_LOOK_BACK_FOR_LESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK_FOR_LESS(v: Py) { ensure_init(); G_LOOK_BACK_FOR_LESS.with(|c| *c.borrow_mut() = v); }
pub fn g_LOSS_WORDS() -> Py { ensure_init(); G_LOSS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOSS_WORDS(v: Py) { ensure_init(); G_LOSS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LOST_LEADS() -> Py { ensure_init(); G_LOST_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOST_LEADS(v: Py) { ensure_init(); G_LOST_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_METHOD_ASKS() -> Py { ensure_init(); G_METHOD_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_METHOD_ASKS(v: Py) { ensure_init(); G_METHOD_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_ASK() -> Py { ensure_init(); G_MORE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_ASK(v: Py) { ensure_init(); G_MORE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_LEADS() -> Py { ensure_init(); G_MORE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_LEADS(v: Py) { ensure_init(); G_MORE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST() -> Py { ensure_init(); G_MOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST(v: Py) { ensure_init(); G_MOST.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_ASK() -> Py { ensure_init(); G_MOVE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_ASK(v: Py) { ensure_init(); G_MOVE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_LEADS() -> Py { ensure_init(); G_MOVE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_LEADS(v: Py) { ensure_init(); G_MOVE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_VERBS() -> Py { ensure_init(); G_MOVE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_VERBS(v: Py) { ensure_init(); G_MOVE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_AFTER_IT() -> Py { ensure_init(); G_NAMED_AFTER_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_AFTER_IT(v: Py) { ensure_init(); G_NAMED_AFTER_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_NEEDS_COMPLEMENT() -> Py { ensure_init(); G_NEEDS_COMPLEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEEDS_COMPLEMENT(v: Py) { ensure_init(); G_NEEDS_COMPLEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_NEXT_ASK() -> Py { ensure_init(); G_NEXT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEXT_ASK(v: Py) { ensure_init(); G_NEXT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NEXT_LEADS() -> Py { ensure_init(); G_NEXT_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEXT_LEADS(v: Py) { ensure_init(); G_NEXT_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEXT_STEP() -> Py { ensure_init(); G_NEXT_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEXT_STEP(v: Py) { ensure_init(); G_NEXT_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_NEXT_STEP_EMERGENCY() -> Py { ensure_init(); G_NEXT_STEP_EMERGENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEXT_STEP_EMERGENCY(v: Py) { ensure_init(); G_NEXT_STEP_EMERGENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_TO_CUT() -> Py { ensure_init(); G_NOTHING_TO_CUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_TO_CUT(v: Py) { ensure_init(); G_NOTHING_TO_CUT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ALONE() -> Py { ensure_init(); G_NOT_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ALONE(v: Py) { ensure_init(); G_NOT_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DOING() -> Py { ensure_init(); G_NOT_A_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DOING(v: Py) { ensure_init(); G_NOT_A_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FIGURE_ASK() -> Py { ensure_init(); G_NOT_A_FIGURE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FIGURE_ASK(v: Py) { ensure_init(); G_NOT_A_FIGURE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_METHOD() -> Py { ensure_init(); G_NOT_A_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_METHOD(v: Py) { ensure_init(); G_NOT_A_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_MOVE() -> Py { ensure_init(); G_NOT_A_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MOVE(v: Py) { ensure_init(); G_NOT_A_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_STEP() -> Py { ensure_init(); G_NOT_A_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STEP(v: Py) { ensure_init(); G_NOT_A_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SUBJECT() -> Py { ensure_init(); G_NOT_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SUBJECT(v: Py) { ensure_init(); G_NOT_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TAIL() -> Py { ensure_init(); G_NOT_A_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TAIL(v: Py) { ensure_init(); G_NOT_A_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_ASKED() -> Py { ensure_init(); G_NOT_A_THING_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_ASKED(v: Py) { ensure_init(); G_NOT_A_THING_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_TRIED() -> Py { ensure_init(); G_NOT_TRIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_TRIED(v: Py) { ensure_init(); G_NOT_TRIED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOW_LEADS() -> Py { ensure_init(); G_NOW_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOW_LEADS(v: Py) { ensure_init(); G_NOW_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_LONGER_TO_DO() -> Py { ensure_init(); G_NO_LONGER_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_LONGER_TO_DO(v: Py) { ensure_init(); G_NO_LONGER_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_SHORTER_STILL() -> Py { ensure_init(); G_NO_SHORTER_STILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_SHORTER_STILL(v: Py) { ensure_init(); G_NO_SHORTER_STILL.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECT_END() -> Py { ensure_init(); G_OBJECT_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECT_END(v: Py) { ensure_init(); G_OBJECT_END.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENERS() -> Py { ensure_init(); G_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENERS(v: Py) { ensure_init(); G_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDER() -> Py { ensure_init(); G_ORDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDER(v: Py) { ensure_init(); G_ORDER.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNED() -> Py { ensure_init(); G_OWNED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNED(v: Py) { ensure_init(); G_OWNED.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_LEAD() -> Py { ensure_init(); G_OWN_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_LEAD(v: Py) { ensure_init(); G_OWN_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_LEAD_WORDS() -> Py { ensure_init(); G_OWN_LEAD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_LEAD_WORDS(v: Py) { ensure_init(); G_OWN_LEAD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_ASKS() -> Py { ensure_init(); G_PART_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_ASKS(v: Py) { ensure_init(); G_PART_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_OPENS() -> Py { ensure_init(); G_PART_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_OPENS(v: Py) { ensure_init(); G_PART_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_STAND_INS() -> Py { ensure_init(); G_PART_STAND_INS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_STAND_INS(v: Py) { ensure_init(); G_PART_STAND_INS.with(|c| *c.borrow_mut() = v); }
pub fn g_PAY_FAMILY() -> Py { ensure_init(); G_PAY_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAY_FAMILY(v: Py) { ensure_init(); G_PAY_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_PAY_WHEN_WORDS() -> Py { ensure_init(); G_PAY_WHEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAY_WHEN_WORDS(v: Py) { ensure_init(); G_PAY_WHEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIOD_WORD() -> Py { ensure_init(); G_PERIOD_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIOD_WORD(v: Py) { ensure_init(); G_PERIOD_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_PIECE_END() -> Py { ensure_init(); G_PIECE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PIECE_END(v: Py) { ensure_init(); G_PIECE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_AT() -> Py { ensure_init(); G_POINTING_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_AT(v: Py) { ensure_init(); G_POINTING_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_BACK() -> Py { ensure_init(); G_POINTING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_BACK(v: Py) { ensure_init(); G_POINTING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_WORDS() -> Py { ensure_init(); G_PRICE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_WORDS(v: Py) { ensure_init(); G_PRICE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_RIGHT() -> Py { ensure_init(); G_PUT_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_RIGHT(v: Py) { ensure_init(); G_PUT_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_RIGHT_ASK() -> Py { ensure_init(); G_PUT_RIGHT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_RIGHT_ASK(v: Py) { ensure_init(); G_PUT_RIGHT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_RIGHT_LEADS() -> Py { ensure_init(); G_PUT_RIGHT_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_RIGHT_LEADS(v: Py) { ensure_init(); G_PUT_RIGHT_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_RIGHT_WORDS() -> Py { ensure_init(); G_PUT_RIGHT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_RIGHT_WORDS(v: Py) { ensure_init(); G_PUT_RIGHT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALIFIER() -> Py { ensure_init(); G_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALIFIER(v: Py) { ensure_init(); G_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION() -> Py { ensure_init(); G_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); G_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTIONS_THAT_ARE_NOT_STEPS() -> Py { ensure_init(); G_QUESTIONS_THAT_ARE_NOT_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTIONS_THAT_ARE_NOT_STEPS(v: Py) { ensure_init(); G_QUESTIONS_THAT_ARE_NOT_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_MORE() -> Py { ensure_init(); G_QUESTION_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_MORE(v: Py) { ensure_init(); G_QUESTION_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTE_PAIRS() -> Py { ensure_init(); G_QUOTE_PAIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTE_PAIRS(v: Py) { ensure_init(); G_QUOTE_PAIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_REFER_BACK() -> Py { ensure_init(); G_REFER_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFER_BACK(v: Py) { ensure_init(); G_REFER_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_REJOINDER() -> Py { ensure_init(); G_REJOINDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REJOINDER(v: Py) { ensure_init(); G_REJOINDER.with(|c| *c.borrow_mut() = v); }
pub fn g_RESET_LEAD() -> Py { ensure_init(); G_RESET_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESET_LEAD(v: Py) { ensure_init(); G_RESET_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_SAFE() -> Py { ensure_init(); G_SAID_SAFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_SAFE(v: Py) { ensure_init(); G_SAID_SAFE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAY_AGAIN_LEADS() -> Py { ensure_init(); G_SAY_AGAIN_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAY_AGAIN_LEADS(v: Py) { ensure_init(); G_SAY_AGAIN_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF() -> Py { ensure_init(); G_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF(v: Py) { ensure_init(); G_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE_END() -> Py { ensure_init(); G_SENTENCE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE_END(v: Py) { ensure_init(); G_SENTENCE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_ENOUGH() -> Py { ensure_init(); G_SHORT_ENOUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_ENOUGH(v: Py) { ensure_init(); G_SHORT_ENOUGH.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_STEP() -> Py { ensure_init(); G_SHORT_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_STEP(v: Py) { ensure_init(); G_SHORT_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSES_LIMIT() -> Py { ensure_init(); G_SOMEONE_ELSES_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_LIMIT(v: Py) { ensure_init(); G_SOMEONE_ELSES_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_STALL() -> Py { ensure_init(); G_STALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STALL(v: Py) { ensure_init(); G_STALL.with(|c| *c.borrow_mut() = v); }
pub fn g_STANDS_IN() -> Py { ensure_init(); G_STANDS_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STANDS_IN(v: Py) { ensure_init(); G_STANDS_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTING_AT() -> Py { ensure_init(); G_STARTING_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTING_AT(v: Py) { ensure_init(); G_STARTING_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTING_BACK() -> Py { ensure_init(); G_STARTING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTING_BACK(v: Py) { ensure_init(); G_STARTING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTING_INTO() -> Py { ensure_init(); G_STARTING_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTING_INTO(v: Py) { ensure_init(); G_STARTING_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTING_OUT() -> Py { ensure_init(); G_STARTING_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTING_OUT(v: Py) { ensure_init(); G_STARTING_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTING_WHAT() -> Py { ensure_init(); G_STARTING_WHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTING_WHAT(v: Py) { ensure_init(); G_STARTING_WHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_START_ASK() -> Py { ensure_init(); G_START_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_START_ASK(v: Py) { ensure_init(); G_START_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_SAYS_FIRST() -> Py { ensure_init(); G_STEP_SAYS_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_SAYS_FIRST(v: Py) { ensure_init(); G_STEP_SAYS_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_VERBS() -> Py { ensure_init(); G_STEP_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_VERBS(v: Py) { ensure_init(); G_STEP_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_WITH_ITS_METHOD() -> Py { ensure_init(); G_STEP_WITH_ITS_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_WITH_ITS_METHOD(v: Py) { ensure_init(); G_STEP_WITH_ITS_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_WORDS() -> Py { ensure_init(); G_STEP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_WORDS(v: Py) { ensure_init(); G_STEP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_ASKS() -> Py { ensure_init(); G_STILL_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_ASKS(v: Py) { ensure_init(); G_STILL_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_SAFE() -> Py { ensure_init(); G_STILL_SAFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_SAFE(v: Py) { ensure_init(); G_STILL_SAFE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_SAME_NUMBER() -> Py { ensure_init(); G_THE_SAME_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_SAME_NUMBER(v: Py) { ensure_init(); G_THE_SAME_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME() -> Py { ensure_init(); G_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME(v: Py) { ensure_init(); G_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED() -> Py { ensure_init(); G_TRIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED(v: Py) { ensure_init(); G_TRIED.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_LEADS() -> Py { ensure_init(); G_TRIED_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_LEADS(v: Py) { ensure_init(); G_TRIED_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_TROUBLE() -> Py { ensure_init(); G_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TROUBLE(v: Py) { ensure_init(); G_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_USE_WITH() -> Py { ensure_init(); G_USE_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USE_WITH(v: Py) { ensure_init(); G_USE_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_WELCOMES() -> Py { ensure_init(); G_WELCOMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WELCOMES(v: Py) { ensure_init(); G_WELCOMES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_IT_IS() -> Py { ensure_init(); G_WHAT_IT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_IT_IS(v: Py) { ensure_init(); G_WHAT_IT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_TO_LOOK() -> Py { ensure_init(); G_WHERE_TO_LOOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_TO_LOOK(v: Py) { ensure_init(); G_WHERE_TO_LOOK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_TO_LOOK_DEFAULT() -> Py { ensure_init(); G_WHERE_TO_LOOK_DEFAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_TO_LOOK_DEFAULT(v: Py) { ensure_init(); G_WHERE_TO_LOOK_DEFAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_TO_LOOK_WORDS() -> Py { ensure_init(); G_WHERE_TO_LOOK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_TO_LOOK_WORDS(v: Py) { ensure_init(); G_WHERE_TO_LOOK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_TO_START() -> Py { ensure_init(); G_WHERE_TO_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_TO_START(v: Py) { ensure_init(); G_WHERE_TO_START.with(|c| *c.borrow_mut() = v); }
pub fn g_WHETHER_ASKS() -> Py { ensure_init(); G_WHETHER_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHETHER_ASKS(v: Py) { ensure_init(); G_WHETHER_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_OF_IT() -> Py { ensure_init(); G_WHOLE_OF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_OF_IT(v: Py) { ensure_init(); G_WHOLE_OF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WORDING() -> Py { ensure_init(); G_WORDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORDING(v: Py) { ensure_init(); G_WORDING.with(|c| *c.borrow_mut() = v); }
pub fn g_WORDS_WANTED() -> Py { ensure_init(); G_WORDS_WANTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORDS_WANTED(v: Py) { ensure_init(); G_WORDS_WANTED.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_A_NOUN() -> Py { ensure_init(); G__AFTER_A_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_A_NOUN(v: Py) { ensure_init(); G__AFTER_A_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_AMOUNT_WANTED() -> Py { ensure_init(); G__AN_AMOUNT_WANTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_AMOUNT_WANTED(v: Py) { ensure_init(); G__AN_AMOUNT_WANTED.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ENTRY_IS_LOOKING() -> Py { ensure_init(); G__AN_ENTRY_IS_LOOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ENTRY_IS_LOOKING(v: Py) { ensure_init(); G__AN_ENTRY_IS_LOOKING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_AGE() -> Py { ensure_init(); G__A_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_AGE(v: Py) { ensure_init(); G__A_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JUDGEMENT() -> Py { ensure_init(); G__A_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JUDGEMENT(v: Py) { ensure_init(); G__A_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STALL() -> Py { ensure_init(); G__A_STALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STALL(v: Py) { ensure_init(); G__A_STALL.with(|c| *c.borrow_mut() = v); }
pub fn g__EXPLANATION_WORDS() -> Py { ensure_init(); G__EXPLANATION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXPLANATION_WORDS(v: Py) { ensure_init(); G__EXPLANATION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__FRAME_HEADS() -> Py { ensure_init(); G__FRAME_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRAME_HEADS(v: Py) { ensure_init(); G__FRAME_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g__HANGS() -> Py { ensure_init(); G__HANGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HANGS(v: Py) { ensure_init(); G__HANGS.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_PAID_FOR() -> Py { ensure_init(); G__MONEY_PAID_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_PAID_FOR(v: Py) { ensure_init(); G__MONEY_PAID_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__NEGATES() -> Py { ensure_init(); G__NEGATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEGATES(v: Py) { ensure_init(); G__NEGATES.with(|c| *c.borrow_mut() = v); }
pub fn g__OWNS_THE_THING() -> Py { ensure_init(); G__OWNS_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWNS_THE_THING(v: Py) { ensure_init(); G__OWNS_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__SO_COMES_THE_STEP() -> Py { ensure_init(); G__SO_COMES_THE_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SO_COMES_THE_STEP(v: Py) { ensure_init(); G__SO_COMES_THE_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g__SO_DOES_NOT_ASK_SOMETHING_OF_THEM() -> Py { ensure_init(); G__SO_DOES_NOT_ASK_SOMETHING_OF_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SO_DOES_NOT_ASK_SOMETHING_OF_THEM(v: Py) { ensure_init(); G__SO_DOES_NOT_ASK_SOMETHING_OF_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g__SO_THE_STEP_FOLLOWS() -> Py { ensure_init(); G__SO_THE_STEP_FOLLOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SO_THE_STEP_FOLLOWS(v: Py) { ensure_init(); G__SO_THE_STEP_FOLLOWS.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_OPEN() -> Py { ensure_init(); G__SUBJECT_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_OPEN(v: Py) { ensure_init(); G__SUBJECT_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g__WEIGHS_ONE_AGAINST() -> Py { ensure_init(); G__WEIGHS_ONE_AGAINST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEIGHS_ONE_AGAINST(v: Py) { ensure_init(); G__WEIGHS_ONE_AGAINST.with(|c| *c.borrow_mut() = v); }
pub fn g_a_whole_cut() -> Py { ensure_init(); G_a_whole_cut.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_whole_cut(v: Py) { ensure_init(); G_a_whole_cut.with(|c| *c.borrow_mut() = v); }
pub fn g_handed_back() -> Py { ensure_init(); G_handed_back.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_handed_back(v: Py) { ensure_init(); G_handed_back.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_recent_answers() -> Py { ensure_init(); G_recent_answers.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_recent_answers(v: Py) { ensure_init(); G_recent_answers.with(|c| *c.borrow_mut() = v); }
pub fn g_said_recently() -> Py { ensure_init(); G_said_recently.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_said_recently(v: Py) { ensure_init(); G_said_recently.with(|c| *c.borrow_mut() = v); }
pub fn g_same_subject() -> Py { ensure_init(); G_same_subject.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_same_subject(v: Py) { ensure_init(); G_same_subject.with(|c| *c.borrow_mut() = v); }
pub fn g_understand() -> Py { ensure_init(); G_understand.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_understand(v: Py) { ensure_init(); G_understand.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    s_handed_back(py_func("handed_back", |args: &[Py]| handed_back__wrapped(args)));
    let _ = py_str("They ask for what to do next, and the bot has just said it.\n\nThe commonest follow-up there is: the bot answers \"my washing machine is leaking\" with the steps\nto take, and the next line is \"what do I do?\" or \"and now?\" - or the bot answers \"i've got about\n400 pounds for a used bike\" with what to check, and the next line is \"what should I look for?\".\nBoth got a search hedge (\"I'm not sure about that one, sorry - I'd rather not guess\"), which\nreads as if the answer had never been given.\n\nThis finds that turn: a question with no thing of its own (every word in it is an ask word), the\nanswer the bot gave a turn or two ago, and nothing said in between that moved the subject on.\nThen the reply is that answer again in short form, not a hedge.\n\nThe rule is narrow on purpose:\n* the line must name nothing, so \"what should i cook tonight\" and \"how long will it take\" never\n  come here - they have an answer of their own;\n* the bot's own hedges are skipped, never handed back (\"I'm not sure about that one\" is not an\n  answer to repeat);\n* only the turns just before this one count, and the answer has to be about the thing the chat\n  is still on;\n* the answer comes back in its short form, so the long version is never read out twice.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_a_whole_cut(py_func("a_whole_cut", |args: &[Py]| pyrt::xrefs::xcall("compose.whole", "a_whole_cut", args)));
    s_understand(py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args)));
    s_ASK(call_builtin("frozenset", &[mcall(&py_str("\n    what whats how where when which why who\n    do does doing done now next then instead else anything something\n    look looking start begin fix sort try use get make buy take stop\n    should shall can could would will may might have has had\n    i me my we you your it its that this these those there\n    the a an again more better please tell say said too as well\n    go going work help helps helped helping suggest recommend about at on in with\n    first begin beginning step steps change advice tip tips thing things\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_VERB(call_builtin("frozenset", &[mcall(&py_str("\n    check checking ask asking bring bringing pick picking choose choosing spend spending\n    watch watching expect expecting pack packing book booking\n    first next step steps part bit\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("make"), py_str("makes"), py_str("made"), py_str("making"), py_str("cook"), py_str("cooks"), py_str("cooked"), py_str("cooking"), py_str("bake"), py_str("bakes"), py_str("baked"), py_str("baking"), py_str("prepare"), py_str("preparing"), py_str("serve"), py_str("serving")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("buy"), py_str("buys"), py_str("buying"), py_str("bought"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("pick"), py_str("picks"), py_str("picking"), py_str("choose"), py_str("chooses"), py_str("choosing"), py_str("chose"), py_str("shop"), py_str("shopping"), py_str("spend"), py_str("spending")]) }
    s_DOINGS(py_tuple(vec![__data1(), __data2(), py_tuple(vec![py_str("check"), py_str("checks"), py_str("checking"), py_str("look"), py_str("looking"), py_str("inspect"), py_str("test"), py_str("testing"), py_str("watch"), py_str("watching")]), py_tuple(vec![py_str("ask"), py_str("asks"), py_str("asking"), py_str("say"), py_str("says"), py_str("saying"), py_str("tell"), py_str("telling"), py_str("word"), py_str("words")]), py_tuple(vec![py_str("bring"), py_str("bringing"), py_str("take"), py_str("taking"), py_str("pack"), py_str("packing"), py_str("wear"), py_str("wearing")]), py_tuple(vec![py_str("try"), py_str("tries"), py_str("trying"), py_str("use"), py_str("using"), py_str("start"), py_str("starting"), py_str("begin"), py_str("beginning")])]));
    s_WHOLE_OF_IT(call_builtin("frozenset", &[mcall(&py_str("\n    do does doing done next then else now step steps first also more again\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLER(call_builtin("frozenset", &[mcall(&py_str("and so ok okay but well alright right then just maybe perhaps also too good best"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME(call_builtin("frozenset", &[mcall(&py_str("tonight today tomorrow morning afternoon evening night later soon week\n    weekdays weekend month"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF(call_builtin("frozenset", &[mcall(&py_str("myself yourself himself herself itself oneself ourselves yourselves themselves"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|ok\\s+|okay\\s+|but\\s+|well\\s+|right\\s+)*(?:what|whats|how|where|now|next|then)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RESET_LEAD(stdlib_call("re", "compile", &[py_str("^\\s*(?:nvm(?:\\s+(?:that|this|it))?|never\\s*mind(?:\\s+(?:that|this|it))?|nevermind|forget\\s+(?:it|that|this)|ignore\\s+(?:it|that|this)|scrap\\s+(?:it|that|this)|anyway|anyways|new\\s+question|different\\s+question|changing\\s+the\\s+subject|change\\s+of\\s+subject)\\b[\\s,.:;-]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_WORDS(call_builtin("frozenset", &[mcall(&py_str("dont know idea sure stuck really not even lost help someone\n    thought anymore loss at myself"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AT_A_LOSS(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|ok\\s+|okay\\s+|but\\s+|well\\s+|right\\s+|honestly\\s+)*(?:i\\s+)?(?:really\\s+|just\\s+)?(?:don'?t|dont|do not|didn'?t|didnt)\\s+(?:really\\s+)?know\\s+(?:what|how|where|who|which|when)\\s+(?:to\\b|next\\b|else\\b|any\\s?more\\b|first\\b)|\\b(?:(?:have|got)\\s+)?no\\s+idea\\s+(?:what|how|where)\\s+to\\b|\\b(?:not\\s+sure|stuck|at\\s+a\\s+loss)\\s+(?:what|how|where)\\s+to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWER_MODULES(py_tuple(vec![py_str("answers"), py_str("memory_store")]));
    s_A_STORE_ANSWER(Py::Int(120));
    s_HEDGE(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m not sure|i don'?t know enough|that'?s beyond what i know|a quick search|searching for it|i'?d rather not guess|can'?t call that one|not mine to decide|nothing so far|search should sort you out|tell me what you find|what would be most useful|anything you'?d like a hand with)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADS(py_tuple(vec![py_str("You've got it: {said}"), py_str("That's the one: {said}"), py_str("It's what I said a moment ago: {said}"), py_str("Same answer: {said}"), py_str("From just now: {said}")]));
    s_BARE_ASK(stdlib_call("re", "compile", &[py_str("^(?:(?:sorry|er|erm|um|uh|oh|ah|ok|okay|and|so|but|wait|hmm|hey)\\W+)*(?P<word>huh|eh|come again|what was that|what did you say|say what|wha|hm|what|sorry|pardon)(?:\\s+(?:please|again|then|now|sorry))*\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_ASK_NEEDS_A_QUESTION(call_builtin("frozenset", &[mcall(&py_str("what sorry pardon"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAY_AGAIN_LEADS(py_tuple(vec![py_str("Again: {said}"), py_str("Sorry, again: {said}"), py_str("Said again: {said}")]));
    s_TRIED_LEADS(py_tuple(vec![py_str("Then it is worth going a step further: {step}"), py_str("If that is not shifting it, the next move is this: {step}"), py_str("Right - then the thing that is left is to go further: {step}")]));
    s_NEXT_STEP_EMERGENCY(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|a&e|accident and emergency|emergency (?:services|department|room)|crisis line|116 123|988|samaritans)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEXT_STEP(stdlib_call("re", "compile", &[py_str("\\b(?:gp|doctor|nurse|pharmacist|chemist|dentist|vet|optician|midwife|health visitor|111|999|911|helpline|a&e|hospital|council|citizens advice|plumber|electrician|engineer|gas safe|mechanic|locksmith|roofer|builder|technician|specialist)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:help|helps|helped|helping|first|start|begin|next|step|steps|change|try|do|advice|tip)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_RIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:save|saved|salvage|salvageable|rescue|rescuable|revive|recover|redeem|fix|fixed|repair|ruined|wasted|undo|undoable)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_RIGHT_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|ok\\s+|okay\\s+|but\\s+|well\\s+|right\\s+)*(?:can|could|can'?t|is|are|will|would|should|does|do|any|anything|is there|any chance)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_RIGHT_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    save saved saving salvage salvageable rescue rescuable revive recover recoverable redeem\n    fix fixed fixing repair repaired ruined waste wasted undo undoable gone dead done late still\n    worth anyway any way ways chance point hope\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AND_NOW(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|ok\\s+|okay\\s+|right\\s+)?now(?:\\s+then)?[\\s?.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_RIGHT_LEADS(py_tuple(vec![py_str("Worth a go: {step}."), py_str("Yes, there is something: {step}."), py_str("Not too late: {step}."), py_str("There is still this: {step}.")]));
    s_USE_WITH(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything)\\s+(?:can|could|should|do|would)\\s+(?:i|you|we)\\s+do\\s+with\\s+(?:that|this|it|them|those)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEXT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything)\\s+else\\b|\\bwhat\\s+next\\b|\\bnext\\s+one\\b|\\bwhat\\s+now\\b|\\bwhat\\s+other\\b|\\band\\s+then\\b|\\band\\s+after\\s+that\\b|\\bthen\\s+what\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEXT_LEADS(py_tuple(vec![py_str("Next up: {step}."), py_str("The next one: {step}."), py_str("Then this: {step}."), py_str("After that: {step}.")]));
    s_ALSO_LEADS(py_tuple(vec![py_str("Also in that answer: {step}."), py_str("And this: {step}."), py_str("It also said this: {step}."), py_str("There is this too: {step}.")]));
    s_MOVE_LEADS(py_tuple(vec![py_str("Start with this: {step}."), py_str("The thing to do first: {step}."), py_str("I would start here: {step}."), py_str("The first thing to do: {step}.")]));
    s_STEP_SAYS_FIRST(stdlib_call("re", "compile", &[py_str("^(?:start|begin|first)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD_SAYS_FIRST(stdlib_call("re", "compile", &[py_str("\\b(?:start|begin|first)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOW_LEADS(py_tuple(vec![py_str("The one to do now: {step}."), py_str("The one I would pick: {step}."), py_str("Do this one: {step}.")]));
    s_LOST_LEADS(py_tuple(vec![py_str("Start with this: {step}."), py_str("One thing to do first: {step}."), py_str("Begin here: {step}."), py_str("That's alright. Start with this: {step}.")]));
    s_BE_FORM(call_builtin("frozenset", &[mcall(&py_str("be been being is are was were am its it's thats that's"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING_AT(call_builtin("frozenset", &[mcall(&py_str("it that this"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGREEING(call_builtin("frozenset", &[mcall(&py_str("\nok okay k right yes yeah yep yup got it good cool nice alright oh ah so then just too as well and\nbut actually honestly really well great perfect thanks cheers course fine sure indeed exactly\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGREEMENT_TAIL(call_builtin("frozenset", &[mcall(&py_str("\nthen too well right ok okay yes yeah yep yup both is\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGREES_ON_ITS_OWN(call_builtin("frozenset", &[mcall(&py_str("\nok okay k right yes yeah yep yup good cool nice alright great perfect thanks cheers course\nfine sure indeed exactly\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGREES_IN_A_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:got|get|gets|that's|thats|is|was|does|did)\\s+(?:it|that|this|you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:change|changed|fix|tackle|improve)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGE_LEADS(py_tuple(vec![py_str("The first thing I would change: {step}."), py_str("What I would change first: {step}."), py_str("Change this first: {step}.")]));
    s_MORE_LEADS(py_tuple(vec![py_str("Also from that answer: {step}."), py_str("Another one from it: {step}."), py_str("Worth trying as well: {step}."), py_str("There is this one too: {step}.")]));
    fn __data3() -> Py { py_tuple(vec![py_str("track"), py_str("run"), py_str("note"), py_str("list"), py_str("lift"), py_str("simmer"), py_str("scrub"), py_str("wipe"), py_str("weigh"), py_str("sort"), py_str("split"), py_str("tip"), py_str("pour"), py_str("stir"), py_str("seal"), py_str("label"), py_str("spray"), py_str("blot"), py_str("trim"), py_str("tie"), py_str("cap"), py_str("dab"), py_str("pick"), py_str("chip"), py_str("sand"), py_str("scrape"), py_str("air"), py_str("dose"), py_str("feed"), py_str("renew"), py_str("fix"), py_str("swap out"), py_str("cut out"), py_str("turn off"), py_str("shut"), py_str("lock"), py_str("charge"), py_str("defrost"), py_str("descale"), py_str("dilute"), py_str("bring"), py_str("anchor"), py_str("aim"), py_str("count"), py_str("hold off"), py_str("leave out"), py_str("cut back"), py_str("write down"), py_str("undo"), py_str("unscrew"), py_str("screw"), py_str("pull"), py_str("push"), py_str("lift out"), py_str("prise"), py_str("wiggle"), py_str("dry"), py_str("brush"), py_str("vacuum"), py_str("mop"), py_str("dust"), py_str("sweep"), py_str("empty"), py_str("shake"), py_str("tape"), py_str("glue"), py_str("clamp"), py_str("snip"), py_str("preheat"), py_str("grease"), py_str("knead"), py_str("prove"), py_str("dust off"), py_str("wash out"), py_str("scrub out"), py_str("air out")]) }
    s_MOVE_VERBS(__data3());
    s_STEP_WORDS(Py::Int(20));
    s_STEP_WITH_ITS_METHOD(Py::Int(30));
    s_CUT_WORDS(Py::Int(12));
    s_SHORT_STEP(Py::Int(6));
    s_NEEDS_COMPLEMENT(call_builtin("frozenset", &[mcall(&py_str("put place set leave keep wrap cover soak spray apply dip store hang lay pour fill seal"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUALIFIER(stdlib_call("re", "compile", &[py_str("\\b(?:because|so that|which|if|unless|before|after|until|while|when|once|in case|as long as|rather than)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADING_PHRASE(call_builtin("frozenset", &[mcall(&py_str("half quarter each every once twice three two four five one times\n    day days week weeks month months year years minute minutes hour hours night nights morning\n    mornings evening evenings afternoon per in on at for after before during gently slowly\n    carefully lightly quickly easily really also just only about"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ARTICLE(call_builtin("frozenset", &[mcall(&py_str("a an the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADING_PHRASE_MAX(Py::Int(3));
    s_OPENERS(call_builtin("frozenset", &[mcall(&py_str("with in into onto inside on at by from to for of under over through"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DANGLING(call_builtin("frozenset", &[mcall(&py_str("is are was were be been being am what that which who whom whose how why the a an of to for and or but with in on at by from as it its this these those his her their my your our there here than then so if even when while because unless until plus yet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PIECE_END(stdlib_call("re", "compile", &[py_str(":\\s|:|(?<!\\d),(?!\\d)| - |;| \\u2014 ")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_STEP(stdlib_call("re", "compile", &[py_str("^(?:tell|let|send|give|show|ask|name)\\s+(?:me|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MENU(stdlib_call("re", "compile", &[py_str("\\b(?:whichever|whatever|whenever|either|any of them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD_SPLIT(stdlib_call("re", "compile", &[py_str("(?<=[.!?;])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUOTE_PAIRS(py_tuple(vec![py_tuple(vec![py_str("\""), py_str("\"")]), py_tuple(vec![py_str("“"), py_str("”")])]));
    s_MOST(Py::Int(240));
    s_LEAD_IN(Py::Int(45));
    s_LOOK_BACK(Py::Int(3));
    s_LOOK_BACK_FOR_LESS(Py::Int(6));
    s_ASKS(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|ok(?:ay)?\\s+|but\\s+|well\\s+|right\\s+|also\\s+)*(?:is|are|was|were|do|does|did|can|could|should|shall|would|will|have|has|had|am|may|might|must|any|anything|anyone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_THEM(stdlib_call("re", "compile", &[py_str("\\btell me\\b|\\blet me know\\b|\\bwhat(?:'s| is| have you)\\b|\\bsay which\\b|\\bwhat about\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFER_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:the list i gave you|what i said (?:above|a moment ago)|down the list|first, then|i said (?:just )?now|same answer|you'?ve got it|that'?s the one|from just now|my answer above|the rest of what i said|the other things i said|first move|follows from (?:that|it)|comes after it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THE_BOT(stdlib_call("re", "compile", &[py_str("\\b(?:you|your|you'?re|u|ur)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORDS_WANTED(stdlib_call("re", "compile", &[py_str("\\b(?:say|tell|write|phrase|word(?:ed|s)?|put\\s+it|email|text|message|reply|respond|explain|ask\\s+for|apologise)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ECHO_FRAME(call_builtin("frozenset", &[mcall(&py_str("got not right ok okay yes noted good to know remember that thats\n    ill i you your youre its it and the a an of in on at for with so then anything like hand\n    question theres there here me my"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STALL(stdlib_call("re", "compile", &[py_str("\\b(?:say more if it helps|go on|keep going|tell me the rest|i'?m with you|i'?m listening|i'?m here if you want|anything you'?d like a hand with|i'?ll remember that|noted|good to know|i'?ve got that|i'?ll keep that in mind)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JUST_A_NOD(stdlib_call("re", "compile", &[py_str("^\\s*(?:ok|okay|alright|all right|right|fine|good|great|cool|cheers|ta|thanks|nice)(?:\\s+(?:then|thanks|cheers|mate|ok|okay|so))*[\\s.,!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:any\\s?way|anything|something\\s+(?:that|which|to)|is there|are there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK_QUESTION_FIRST(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|should|shall)\\s+(?:i|we)\\s+do\\s+(?:anything|something)\\b|\\bis\\s+there\\s+(?:anything|something)\\s+(?:i|we)\\s+(?:can|could|should|might|would)\\s+do\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_AFTER_IT(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|your|the|a|an)\\s+[a-z]")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("think"), py_str("reckon"), py_str("want"), py_str("suppose"), py_str("guess"), py_str("even"), py_str("actually"), py_str("back"), py_str("again"), py_str("one"), py_str("ones"), py_str("thing"), py_str("things"), py_str("stuff"), py_str("lot"), py_str("bit"), py_str("kind"), py_str("sort"), py_str("way"), py_str("ways"), py_str("time"), py_str("times"), py_str("people"), py_str("anyone"), py_str("someone"), py_str("everything"), py_str("anything"), py_str("something"), py_str("nothing")]) }
    s_NOT_A_THING(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_CLAUSE(stdlib_call("re", "compile", &[py_str("^\\s*(?:if|unless|when|whenever|once|in case|should|as long as)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_MOVE(stdlib_call("re", "compile", &[py_str("^(?:ask|ring|call|see|phone|speak to|talk to|book|visit|check)\\b[^.!?]{0,30}\\b(?:gp|doctor|doctors|dentist|nurse|pharmacist|chemist|optician|vets?|hospital|ae|a&e|plumber|electrician|engineer|specialist)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_VERBS(call_builtin("frozenset", &[mcall(&py_str("delete remove clear keep pick choose check replace"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER(stdlib_call("re", "compile", &[py_str("\\b(?:first|next|then|before|after that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_THING(stdlib_call("re", "compile", &[py_str("^(?P<move>.{4,120}?)\\s+is\\s+(?:the|your|my)\\s+(?:first thing|thing to do first|best first (?:step|move)|first (?:step|move)|best (?:step|move)|place to start|one thing|fix|change|main thing|thing that helps|bit that helps|part that matters)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WEIGHS_ONE_AGAINST(stdlib_call("re", "compile", &[py_str("\\s(?:rather than|instead of)\\s"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AFTER_A_NOUN(call_builtin("frozenset", &[mcall(&py_str("at in on from of for to with by near under behind over into about"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_OPEN(stdlib_call("re", "compile", &[py_str("^(?:they|it|it's|its|he|he's|she|she's|this|that|that's|these|those|there|there's|we|we're)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__EXPLANATION_WORDS(Py::Int(5));
    s__HANGS(call_builtin("frozenset", &[mcall(&py_str("and or but then with to of the a an in on at for is are that this"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__NEGATES(stdlib_call("re", "compile", &[py_str("^(?:do not|don't|dont|never|avoid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SO_COMES_THE_STEP(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|but\\s+)?so\\b[,\\s]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SO_THE_STEP_FOLLOWS(stdlib_call("re", "compile", &[py_str("[,;]\\s*(?:and\\s+)?so\\b[,\\s]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SO_DOES_NOT_ASK_SOMETHING_OF_THEM(stdlib_call("re", "compile", &[py_str("^do(?:es)?\\b(?!\\s+(?:it|that|this|these|those|them|the|a|an|your|my|his|her|their|our|both|one|all|more|less|as|so)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_JUDGEMENT(stdlib_call("re", "compile", &[py_str("^\\s*(?:when|once|if|as soon as|next time)\\b[^,]{0,60}?\\b(?:sounds?|feels?|you'?re happy|you are happy|you can live with|you'?re ok(?:ay)? with|you are ok(?:ay)? with)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAIMS_IT_WAS_SAID(py_tuple(vec![py_str("it's what i said a moment ago"), py_str("you've got it"), py_str("that's the one"), py_str("same answer"), py_str("from just now"), py_str("said again"), py_str("sorry, again"), py_str("as i said"), py_str("again")]));
    s__FRAME_HEADS(py_list(vec![]));
    s_LESS(stdlib_call("re", "compile", &[py_str("\\b(?:make (?:it|that)|keep it|be|say it|put it|give it to me|give me it)\\s+(?:a bit |a lot |really |much |nice and )?(?:short|shorter|brief|briefer|quick|quicker|snappy|concise)\\b|\\b(?:short|shorter|brief|briefly|concisely)\\s+please\\b|\\b(?:shorter|tldr|tl;?dr|summarise|summarize|sum it up|sum up|short version|cut it down|cut to the chase|get to the point|skip the detail|less detail|in (?:a|one) (?:sentence|line)|one line|just the (?:gist|answer|short version))\\b|\\b(?:simpl(?:e|er|ify|ified)|in plain (?:english|terms|words)|plain english|(?:in )?simple (?:english|terms|words)|no jargon|jargon[- ]free|layman'?s? ?terms?|explain (?:it|that)(?: (?:all )?(?:again|to me))? (?:simply|in english|in simple words)|put it (?:simply|plainly|in plain (?:english|words))|dumb (?:it|that)? ?down|plainly\\b|like (?:i'?m|im|i am) (?:5|five))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LESS_WORDS(call_builtin("frozenset", &[mcall(&py_str("make made it that keep be say put give me a bit lot really much\n    nice short shorter brief briefer briefly quick quicker snappy concise please tldr tl dr\n    summarise summarize sum gist answer version cut chase point skip detail details less\n    sentence line one just and so ok okay right then now again too long word words\n    more fewer condensed simple simpler simply simplify simplified plain english terms words\n    jargon free layman laymans explain plainly like im i'm five 5 dumb dumbed easy easier wordy"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LESS_LEADS(py_tuple(vec![py_str("{short}"), py_str("Shorter: {short}"), py_str("In short: {short}"), py_str("Briefly: {short}")]));
    s_OWN_LEAD(stdlib_call("re", "compile", &[py_str("^[^:!?.\\n]{1,45}:\\s")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_LEAD_WORDS(Py::Int(5));
    s_KEEP_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:but\\s+)?(?:keep|keep\\s+in|leave\\s+in|don'?t\\s+(?:leave\\s+out|drop|lose)|without\\s+(?:losing|dropping)|making\\s+sure\\s+(?:to\\s+)?(?:keep|say))\\s+(?P<what>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURES(stdlib_call("re", "compile", &[py_str("\\b(?:timings?|times?|numbers?|figures?|ratios?|amounts?|quantit(?:y|ies)|temperatures?|degrees?|weights?|measures?|proportions?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIGIT(stdlib_call("re", "compile", &[py_str("[0-9]")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_TO(stdlib_call("re", "compile", &[py_str("\\bhow\\b(?:\\s+\\w+){0,3}\\s+(?:do|can|should|would|could|might|shall)\\s+(?:i|we|you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BREVITY_WORDS(call_builtin("frozenset", &[mcall(&py_str("brief briefly short shorter concise concisely tldr gist sentence\n    condensed summarise summarize sum version line"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPING(py_tuple(vec![py_str("Keeping {keep}: {short}"), py_str("With {keep} in: {short}")]));
    s_ALREADY_SHORT(py_tuple(vec![py_str("That's about as short as it gets: {short}"), py_str("That is the short version: {short}")]));
    s_NO_SHORTER_STILL(py_tuple(vec![py_str("That is the short version - there is nothing left in it to take out."), py_str("Nothing left to cut: every part of that answer you have now had on its own.")]));
    s_NOTHING_TO_CUT(py_tuple(vec![py_str("Nothing to cut down yet - ask me something and I'll keep it short."), py_str("Nothing in hand to shorten - ask away and I'll keep it brief.")]));
    s_JUST_THIS(py_tuple(vec![py_str("Just this: {short}"), py_str("The one thing in it: {short}")]));
    s_WELCOMES(py_str("greeting"));
    s_SENTENCE_END(stdlib_call("re", "compile", &[py_str("([.!?;][\\\"\\u201d]?)\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT_ENOUGH(Py::Int(26));
    s_CLAUSE_SPLIT(stdlib_call("re", "compile", &[py_str("(?<=[,;])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_CUT_MID_SENTENCE(call_builtin("frozenset", &[mcall(&py_str("and but so or because which that while with rather than to of the\n    a an in on for if when as also then it its your my their there"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LENGTH_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b|\\bhow\\s+many\\s+(?:days|weeks|months|hours|minutes)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_TIME(stdlib_call("re", "compile", &[py_str("^\\s*(?:about|around|roughly|usually|at least|up to|only|just|no more than)?\\s*(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|ninety|half an?|an?)\\s*(?:to\\s+\\w+\\s*)?(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_BACK(Py::Int(3));
    s_A_PRICE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\b|\\bhow\\s+expensive\\b|\\bwhat\\s+(?:does|do|would|will|did)\\s+(?:that|it|this)\\s+cost|\\bwhat\\s+(?:is|was)\\s+the\\s+price\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING_BACK(call_builtin("frozenset", &[mcall(&py_str("that this it those these they them ones one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nhow much many expensive price priced cost costs costing cheap cheaper cheapest usually typical\nnormally roughly about approximately altogether total altogether pay paying spend spends by for\nat a an the per each and so then do does did would will is was are were be be a lot really\nhere there again any more of on in to get got buy buying round roughly average range\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ENTRY_IS_LOOKING(py_list(vec![Py::Bool(false)]));
    s_A_COUNT_OF_MORE_THAN_ONE(stdlib_call("re", "compile", &[py_str("^\\W*(?:two|three|four|five|six|seven|eight|nine|ten|both|a couple|a pair|a few|the two|the three)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:can'?t|cant|cannot|won'?t|wont|doesn'?t|doesnt|isn'?t|isnt|keeps?|still|never|again|worse|worst|luck|helping|working|nothing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OWNED(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their)\\s+([a-z]{2,})")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSES_LIMIT(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|mum|mom|mother|dad|father|nan|nana|gran|granny|grandma|grandad|grandpa|son|daughter|child|kid|boy|girl|husband|wife|partner|brother|sister|friend|baby|toddler|little one)\\b[^.!?]{0,60}?\\b(?:can(?:no|')?t\\s+(?:have|eat|drink|take|manage)|cant\\s+(?:have|eat|drink|take|manage)|cannot\\s+(?:have|eat|drink|take)|must\\s+not\\s+(?:have|eat|drink|take)|(?:does|do|did)(?:n'?t|nt|\\s+not)\\s+(?:eat|drink|like|touch)|(?:is|are)(?:n'?t|\\s+not)\\s+allowed|allergic\\s+to|intolerant\\s+to|stays?\\s+away\\s+from|has\\s+to\\s+avoid|cuts?\\s+out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ACK_OPENS(stdlib_call("re", "compile", &[py_str("^\\s*(?:ok(?:ay)?\\s+)?(?:thanks|thank you|thankyou|thank u|thx|ty|ta|cheers|cheerz|much appreciated|appreciated)\\b[^.!?]{0,20}[.!?]+\\s*\\S"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ACK_AND_REST(stdlib_call("re", "compile", &[py_str("^\\s*(?:ok(?:ay)?\\s+|right\\s+|ah\\s+|oh\\s+|aw\\s+)?(?:thanks|thank you|thankyou|thank u|thx|ty|ta|cheers|cheerz|much appreciated|appreciated)(?:\\s+(?:so|very|much|a lot|again|loads|tonnes))?[^a-z0-9]{0,20}(?P<rest>[a-z0-9].*)$"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSES_THE_TURN(call_builtin("frozenset", &[mcall(&py_str("that this it helps helped helping help works worked work working\n    does did will would good great fine perfect nice handy useful brilliant lovely amazing awesome\n    fantastic ok okay much loads lot lots again everything all so very really too just then now\n    know noted needed needs glad cheers thanks please mate love pet\n    thats kind kindest thoughtful generous sweet of you"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BROODING(stdlib_call("re", "compile", &[py_str("\\b(?:(?:going|goes|went)\\s+(?:over|round|around)\\b|round\\s+in\\s+(?:my|our)\\s+(?:head|mind)|\\bin\\s+my\\s+head\\b|\\bon\\s+my\\s+mind\\b|(?:cant|can'?t|cannot|keep|keeps|kept)\\s+(?:stop\\s+)?thinking\\b|think(?:ing)?\\s+about\\s+it\\b|\\bthoughts?\\s+(?:racing|going)\\b|my\\s+mind\\s+(?:is\\s+)?(?:racing|going|spinning))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIED(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|he|she|they)\\s+(?:already\\s+)?(?:tried|done)\\b|\\b(?:i(?:'ve|ve| have)|we(?:'ve|ve| have)|he(?:'s|s| has)|she(?:'s|s| has)|they(?:'ve|ve| have))\\s+(?:already\\s+)?(?:tried|done|had)\\b|\\balready\\s+(?:tried|done|had)\\b|\\b(?:tried|done)\\s+(?:that|it|this)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_TRIED(stdlib_call("re", "compile", &[py_str("\\b(?:not|never|havent|hasnt|didnt|haven't|hasn't|didn't)\\s+(?:\\w+\\s+){0,2}(?:tried|done|had)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_MORE(stdlib_call("re", "compile", &[py_str("\\b(?:how|what|when|why|which|should)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|he|she|they)\\s+(?:(?:have|had|already|just|finally|now|then|since)\\s+)*(?P<verb>[a-z]+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_dict(vec![(py_str("did"), py_str("do")), (py_str("done"), py_str("do")), (py_str("made"), py_str("make")), (py_str("took"), py_str("take")), (py_str("given"), py_str("give")), (py_str("got"), py_str("get")), (py_str("left"), py_str("leave")), (py_str("kept"), py_str("keep")), (py_str("cut"), py_str("cut")), (py_str("put"), py_str("put")), (py_str("set"), py_str("set")), (py_str("ran"), py_str("run")), (py_str("bled"), py_str("bleed")), (py_str("hung"), py_str("hang")), (py_str("wore"), py_str("wear")), (py_str("fed"), py_str("feed")), (py_str("found"), py_str("find")), (py_str("built"), py_str("build")), (py_str("sent"), py_str("send")), (py_str("spent"), py_str("spend")), (py_str("threw"), py_str("throw")), (py_str("dug"), py_str("dig")), (py_str("held"), py_str("hold")), (py_str("lit"), py_str("light")), (py_str("swept"), py_str("sweep")), (py_str("bought"), py_str("buy")), (py_str("caught"), py_str("catch")), (py_str("taught"), py_str("teach")), (py_str("told"), py_str("tell")), (py_str("sold"), py_str("sell")), (py_str("froze"), py_str("freeze")), (py_str("drew"), py_str("draw")), (py_str("shut"), py_str("shut")), (py_str("spread"), py_str("spread")), (py_str("paid"), py_str("pay")), (py_str("struck"), py_str("strike")), (py_str("shook"), py_str("shake"))]) }
    s_IRREGULAR_PAST(__data5());
    s_NO_LONGER_TO_DO(call_builtin("frozenset", &[mcall(&py_str("not never no havent hasnt didnt doesnt dont wont cant\n    haven't hasn't didn't doesn't don't won't can't"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STANDS_IN(call_builtin("frozenset", &[mcall(&py_str("it them that this one ones those these there"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OBJECT_END(call_builtin("frozenset", &[mcall(&py_str("for to in on at with by from into onto and but then so because when\n    while before after again twice once already yesterday today tonight now later still\n    earlier this morning evening week month year"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REJOINDER(stdlib_call("re", "compile", &[py_str("\\b(?:though|although|already|but|anyway|still|mind you|to be fair)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_recent_answers(py_func("_recent_answers", |args: &[Py]| _recent_answers(args)));
    s_same_subject(py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args)));
    s_said_recently(py_func("_already_repeated", |args: &[Py]| _already_repeated(args)));
    s_handed_back(py_func("_handed_back", |args: &[Py]| _handed_back(args)));
    s__A_STALL(py_str("(?:keeps? putting (?:it |things )?off|puts? (?:it |things )?off|putting (?:it |things )?off|keeps? avoiding (?:it|them|this)|does not know (?:how|where) to start|doesn[\\u2019']?t know (?:how|where) to start|not sure (?:how|where) to start|can[\\u2019']?t get started|cannot get started|has not started|hasn[\\u2019']?t started|not even started|leaves? it to the last minute|hates? starting)"));
    s_ASKING_TO_START(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?:\\b(?:he|she|they)\\b|\\bthe (?:kids?|children)\\b|\\bmy (?:son|daughter|child|kid|boy|girl|mum|mom|dad|father|mother|partner|husband|wife|brother|sister|friend|nephew|niece|grandson|granddaughter)\\b)[^.!?]{0,30}?\\b"), &g__A_STALL()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_START_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:where|how)\\s+(?:do|does|should|shall|can|could|would|might|to)\\s+(?:(?:i|we|you)\\s+)?(?:start|begin)(?:ed|ing)?\\b|\\bhow do (?:i|we) get (?:started|going)\\b|\\b(?:do|try|change|fix|sort|put|write|tackle|get)\\s+first\\b|\\bfirst (?:thing|step|move)\\b|\\bnext step\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTING_WHAT(stdlib_call("re", "compile", &[py_str("\\b(?:where|how)\\s+(?:do|does|should|shall|can|could|would|might|to)\\s+(?:(?:i|we|you)\\s+)?(?:start|begin)(?:ed|ing)?\\s+([a-z']+)(?:\\s+([a-z']+))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTING_BACK(call_builtin("frozenset", &[mcall(&py_str("\nit this that them these those one again over now today here then though please anyway\nsmall slowly simple easy gently early off out up\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTING_AT(call_builtin("frozenset", &[py_tuple(vec![py_str("with"), py_str("on"), py_str("about")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTING_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:start|begin)(?:ed|ing)?\\s+(?:out|off)\\s+([a-z']+)(?:\\s+([a-z']+))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTING_INTO(call_builtin("frozenset", &[py_tuple(vec![py_str("in"), py_str("into"), py_str("at"), py_str("for"), py_str("as")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MORE_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat else (?:can|could|should|would) (?:i|we)\\b|\\bwhat else (?:is there|works|helped|helps|do i do)\\b|\\banything else (?:i|we) (?:can|could|should) (?:do|try)\\b|\\bis there anything else\\b|\\b(?:and )?what else\\b(?!\\s+(?:could|would|might|is|are|was|were)\\s+it\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IDEAS_ASK(py_str("anything (?:else|more|other)|(?:any|some) (?:other |more |different )?(?:ideas?|suggestions?|thoughts?|recommendations?|options?|tips?)|what (?:do|would|can|should) you (?:suggest|recommend|pick|choose|go for|advise)|what would you (?:pick|choose|go for|do)|your (?:ideas|suggestions|thoughts|recommendations)|ideas|suggestions"));
    s_IDEAS_ONLY(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^\\W*(?:(?:and|so|ok|okay|right|well|alright|then|but)\\b[\\s,]+)*(?:"), &g_IDEAS_ASK()), &py_str(")")), &py_str("\\W*(?:[\\s,]+(?:then|please|now|tho|though|thanks|ta))?\\W*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORDING(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|should|could|would|might) (?:i|we)\\s+(?:(?:word|write|say|phrase|put|explain|draft|reply|tell|ask|answer)\\b|(?:bring|raise|mention|broach|approach)\\s+(?:it|that|this|them|the (?:subject|issue|matter|problem))\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_IT(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|should|could|would|might|shall) (?:i|we)\\s+(?:[a-z']+\\s+){0,3}(?:it|that|this|them|these|those)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIST_LEAD(stdlib_call("re", "compile", &[py_str("\\b(?:works?|working|helps?|helped|things to do|to do|steps?|rules?|habits?|ways?|tips?|start|begin|try|keep|stick to|do these|do this)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:things|rest|following|list|these|those|steps|rules|ways|tips|habits|ideas|examples|reasons)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:you|your|i|we|they|he|she|it|its|their)\\b|\\b(?:is|are|was|were|has|have|had|means|meant)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION(stdlib_call("re", "compile", &[py_str("^\\s*(?:who|what|which|when|where|why|how|whether)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DECLARATIVE(stdlib_call("re", "compile", &[py_str("^(?:[a-z']+\\s+)(?:is|are|was|were|has|have|had|can|cannot|will|would|should|takes?|gives?|means?|costs?|comes?|goes?|does|did|gets?|keeps?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTIONS_THAT_ARE_NOT_STEPS(stdlib_call("re", "compile", &[py_str("^(?:what|which|how|when|where|who|why|tell me|let me know)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_IT_IS(stdlib_call("re", "compile", &[py_str("^\\s*(?:what(?:'s|s| is| are| does| do| did)\\b|tell me (?:about|what)\\b|define\\b|meaning of\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ATTEMPT(stdlib_call("re", "compile", &[py_str("\\b(?:tried|trying|already|gave it|had a go|no luck|used)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_ASKS(py_tuple(vec![py_tuple(vec![py_str("time"), stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:long|soon|quick(?:ly)?|longer)\\b|\\bhow\\s+many\\s+(?:seconds?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|nights?)\\b|\\bwhat\\s+(?:time|timing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:\\d+(?:\\.\\d+)?|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|forty-five|fifty|sixty|ninety)\\s*(?:\\w+\\s+to\\s+)?(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|wks?|months?|years?|nights?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("heat"), stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:hot|warm)\\b|\\bwhat\\s+(?:temperature|heat|gas\\s+mark)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:\\d{2,3}\\s*(?:°|degrees?|c\\b|celsius|f\\b|fahrenheit)|gas\\s+mark\\s+\\d|mark\\s+\\d\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("amount"), stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:much|many)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("(?:£\\s?\\d|\\b\\d+\\s*(?:g|kg|grams?|ml|l|litres?|pints?|cups?|tbsp|tsp|oz|ounces?)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("often"), stdlib_call("re", "compile", &[py_str("\\bhow\\s+often\\b|\\bhow\\s+many\\s+times\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:once|twice|three\\s+times|\\d+\\s+times)\\b[^.?!]{0,25}\\b(?:a|per|every)\\s+(?:day|week|month|year|night)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])]));
    s_FIGURE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    how long soon quickly quick longer much many often hot warm what time timing temperature heat\n    gas mark should shall can could would will may might must do does did is are was were am\n    i we you they he she it its that this these those them there and so ok okay but then again\n    also just really leave left need use take cook bake keep wait soak boil simmer rest run\n    sit stand stay lie hang dry cool warm heat set settle marinate marinade prove rise chill\n    exercise give put apply wear change replace before after for in on at to it too much\n    before afterwards after that\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_SAYING(call_builtin("frozenset", &[mcall(&py_str("\n    ask asking say saying tell telling mention mentioning bring bringing raise raising\n    broach broaching approach approaching him her them\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_SAFE(stdlib_call("re", "compile", &[py_str("\\bstill\\s+(?:ok|okay|good|fine|safe|edible|alright)\\b|\\bis\\s+(?:it|that|this)\\s+(?:safe|alright|fine|ok|okay|good)\\b|\\bcan\\s+i\\s+still\\b|\\bsafe\\s+(?:to|for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_SAFE(stdlib_call("re", "compile", &[py_str("\\b(?:safe|fine|okay|ok|keep|keeps|kept|bin|binned|binning|throw|thrown|eat|eaten|edible|cooked|refreeze|put\\s+back|nothing\\s+needs|no\\s+need)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DO_I_NEED_TO(stdlib_call("re", "compile", &[py_str("\\bdo\\s+i\\s+(?:need|have)\\s+to\\b|\\bhave\\s+i\\s+got\\s+to\\b|\\bshould\\s+i\\b|\\bam\\s+i\\s+meant\\s+to\\b|\\bdo\\s+you\\s+have\\s+to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MOVE(stdlib_call("re", "compile", &[py_str("\\b(?:prick|cut|turn|stir|cover|leave|put|add|check|wash|rinse|soak|rest|shake|score|season|test|wait|ask|keep|write|print|sign|ring|call|book|take|use|give|hold|open|close|run|cook|bake|boil|simmer|freeze|defrost|empty|clean|wipe|change|replace)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_TO_START(stdlib_call("re", "compile", &[py_str("\\bwhere\\s+(?:do|should|would|can|shall)\\s+i\\s+(?:really\\s+|even\\s+)?(?:start|begin)\\b|\\bwhere\\s+to\\s+start\\b|\\bwhat\\s+(?:do|should)\\s+i\\s+(?:do|try)\\s+first\\b|\\bhow\\s+do\\s+i\\s+(?:start|begin|get\\s+going)\\b|\\bwhere\\s+do\\s+i\\s+(?:go|look)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PART_ASKS(py_tuple(vec![py_tuple(vec![py_str("time"), stdlib_call("re", "compile", &[py_str("\\bwhen\\s+(?:does|do|will|would|should|can|is|are)\\b|\\bhow\\s+(?:soon|long)\\b|\\bhow\\s+(?:much\\s+)?longer\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), geti(&geti(&g_FIGURE_ASKS(), &Py::Int(0)), &Py::Int(2))]), py_tuple(vec![py_str("start"), g_WHERE_TO_START(), Py::None]), py_tuple(vec![py_str("safe"), g_STILL_SAFE(), g_SAID_SAFE()]), py_tuple(vec![py_str("move"), g_DO_I_NEED_TO(), g_A_MOVE()])]));
    s_PART_STAND_INS(call_builtin("frozenset", &[mcall(&py_str("\n    food foods stuff thing things bit bits part parts piece pieces kit gear item items\n    water drink milk meat rest lot lots everything anything something nothing else\n    job work house home place places one plant plants garden washing clothes\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_ALONE(stdlib_call("re", "compile", &[py_str("^(?:then|after that|afterwards?|next|also|and|but|or|so|instead|either|once|while|because|which|that|this|it|they|them|he|she)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PART_OPENS(call_builtin("frozenset", &[mcall(&py_str("\n    then afterwards next also and but or so instead either once while because which\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_dict(vec![(py_str("cooking"), py_str("the packet or a cookery site")), (py_str("food-and-drink"), py_str("the label or a cookery site")), (py_str("health"), py_str("the NHS site or a pharmacist")), (py_str("fitness"), py_str("a coach or the NHS site")), (py_str("sleep"), py_str("the NHS site")), (py_str("money"), py_str("the official page, gov.uk or your bank")), (py_str("work"), py_str("your contract or ACAS")), (py_str("school-and-study"), py_str("the school's own notice or the exam board")), (py_str("science"), py_str("a textbook or the Royal Society site")), (py_str("history"), py_str("a museum's own page")), (py_str("geography-and-travel"), py_str("a map app or the operator's own site")), (py_str("technology"), py_str("the maker's support page")), (py_str("home-and-diy"), py_str("the maker's instructions or a qualified trade")), (py_str("garden-and-plants"), py_str("the RHS site or the plant's own label")), (py_str("pets"), py_str("your vet")), (py_str("family-and-parenting"), py_str("your GP or the NHS site")), (py_str("friends-and-relationships"), py_str("the person themselves")), (py_str("shopping"), py_str("the shop's own listing")), (py_str("cars"), py_str("the handbook or a garage")), (py_str("entertainment"), py_str("a listings site")), (py_str("sport"), py_str("the club or the governing body's site")), (py_str("language-and-writing"), py_str("a dictionary")), (py_str("feelings"), py_str("a friend who knows you, or your GP")), (py_str("maths-and-numbers"), py_str("a calculator")), (py_str("nature-and-weather"), py_str("the Met Office site"))]) }
    s_WHERE_TO_LOOK(__data6());
    s_WHERE_TO_LOOK_DEFAULT(py_str("a search"));
    s_WHERE_TO_LOOK_WORDS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("baby"), py_str("formula"), py_str("bottle"), py_str("breastfeed"), py_str("nappy"), py_str("weaning")]), py_str("the NHS site or a health visitor")]), py_tuple(vec![py_tuple(vec![py_str("boiler"), py_str("radiator"), py_str("thermostat"), py_str("bleed")]), py_str("the maker's instructions or a gas engineer")]), py_tuple(vec![py_tuple(vec![py_str("extractor"), py_str("socket"), py_str("fuse"), py_str("appliance"), py_str("plug"), py_str("wiring")]), py_str("the maker's manual or an electrician")]), py_tuple(vec![py_tuple(vec![py_str("oven"), py_str("roast"), py_str("bake"), py_str("boil"), py_str("fry"), py_str("pan"), py_str("saucepan"), py_str("dough"), py_str("oven's")]), py_str("the packet or a cookery site")]), py_tuple(vec![py_tuple(vec![py_str("dose"), py_str("tablet"), py_str("antibiotic"), py_str("paracetamol"), py_str("ibuprofen"), py_str("prescription")]), py_str("a pharmacist or the leaflet in the box")]), py_tuple(vec![py_tuple(vec![py_str("battery"), py_str("charger"), py_str("router"), py_str("app"), py_str("software"), py_str("phone")]), py_str("the maker's support page")]), py_tuple(vec![py_tuple(vec![py_str("mortgage"), py_str("overdraft"), py_str("pension"), py_str("tax"), py_str("benefit"), py_str("allowance")]), py_str("the official page, gov.uk or your bank")]), py_tuple(vec![py_tuple(vec![py_str("lease"), py_str("landlord"), py_str("deposit"), py_str("tenancy")]), py_str("your contract or the council's housing team")]), py_tuple(vec![py_tuple(vec![py_str("passport"), py_str("visa"), py_str("embassy"), py_str("renewal")]), py_str("the passport office's own site")]), py_tuple(vec![py_tuple(vec![py_str("weed"), py_str("soil"), py_str("compost"), py_str("prune"), py_str("seedling"), py_str("plant")]), py_str("the RHS site or the plant's own label")]), py_tuple(vec![py_tuple(vec![py_str("puppy"), py_str("kitten"), py_str("vet"), py_str("dog"), py_str("cat")]), py_str("your vet")]), py_tuple(vec![py_tuple(vec![py_str("stitch"), py_str("sew"), py_str("hem"), py_str("zip"), py_str("fabric"), py_str("garment")]), py_str("the care label or a tailor")])]));
    s_A_MOVE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    prick cut score slash stab turn stir cover uncover leave put add check wash rinse soak scrub use try\n    rest shake season salt test taste wait keep freeze defrost empty clean wipe change replace\n    trim peel chop slice mix whisk knead roll fill drain squeeze press\n    bleed flush prime seal grease oil sand paint varnish glue screw tighten loosen unplug plug\n    switch reset restart reboot charge update install download back defrost refrigerate reheat\n    tin jar bottle label marinate brine gut bone stuff ice chill cool warm heat preheat blind\n    line flour dust glaze baste carve prove rise fold beat cream sieve sift blend mash grate zest\n    juice deseed core hull soak de-ice descale unblock plunge rod hoover vacuum mop dust sweep\n    water feed prune repot stake tie mulch mow scarify aerate\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHETHER_ASKS(py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:do|did)\\s+i\\s+(?:need|have|got)\\s+(?:to\\s+)?(?P<rest>[^?!]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\bshould\\s+i\\s+(?:be\\s+|just\\s+|still\\s+)?(?P<rest>[^?!]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\bam\\s+i\\s+(?:meant|supposed)\\s+(?:to\\s+)?(?P<rest>[^?!]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\bcan\\s+i\\s+(?:just|still|safely|even)?\\s*(?P<rest>[^?!]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\bis\\s+it\\s+(?:ok|okay|safe|alright|fine)\\s+to\\s+(?P<rest>[^?!]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_METHOD_ASKS(py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:will|would|does|do|did|is|are|can|could)\\s+(?P<rest>[^?!]{3,40}?)\\s+(?:work|works|help|helps|do\\s+anything|do\\s+it|do\\s+the\\s+job|(?:be\\s+)?enough|(?:be\\s+)?any\\s+good|be\\s+ok\\b|be\\s+okay|be\\s+worth\\s+(?:it|doing))\\b[^?!]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_NOT_A_METHOD(call_builtin("frozenset", &[mcall(&py_str("\n    it that this they those these there he she i we you one ones\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUALITY(call_builtin("frozenset", &[mcall(&py_str("\n    work works worked working better best help helps helped worth enough good fine okay ok\n    safe still anymore actually really even than\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_ASKS(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|or)\\s+)?(?P<aux>can|could|may|should|would)\\s+(?P<who>i|we)\\s+still\\s+(?P<rest>[a-z].*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("\n    first thing things one ones rest bit part way ways sort kind lot next still even actually\n    really everything anything something nothing else another\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GAVE_ASKS(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s|\\s+(?:was|were|is|are))\\s+(?:my|our|the)\\b|\\bwhat\\s+did\\s+(?:i|we)\\s+(?:say|tell)\\b|\\bhow\\s+(?:long|much|many|often|soon)\\b|\\bwhat\\s+was\\s+it\\s+(?:i|we)\\b|\\bwhat(?:'s|s|\\s+is|\\s+are|\\s+do|\\s+does|\\s+did)\\s+(?:i|we|my|our)\\s+(?:(?:current|exact|actual)\\s+)?(?:earn|earns|earning|make|makes|paid|pay|pays|wage|wages|salary|salaries|income)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FRAME(op_bitor(&g_FIGURE_WORDS(), &call_builtin("frozenset", &[mcall(&py_str("\n    said say says saying tell told telling mention mentioned remember remembers remember forgot\n    recall earlier before after since again exactly actually probably guess think know my our\n    the a an i we you it its thing things one ones number figure what whats which was were is are did\n    do does have has had got left until till by about story now been\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("(?P<money>£\\s?\\d[\\d,]*(?:\\.\\d+)?(?:\\s?(?:k|grand|thousand|m)\\b)?|\\b(?<![\\d.])\\d[\\d,]*(?:\\.\\d+)?\\s?(?:grand|k|quid|pounds?|dollars?|euros?|bucks)\\b)|(?P<time>\\b(?<![\\d.])\\d[\\d,]*(?:\\.\\d+)?\\s*(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|wks?|months?|years?|nights?)\\b|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+(?:seconds?|minutes?|hours?|days?|weeks?|months?|years?|nights?)\\b|\\b(?:about\\s+|around\\s+|nearly\\s+|over\\s+|under\\s+)?(?:a|an)\\s+(?:second|minute|hour|day|week|month|year|night)\\b|\\b(?:a\\s+)?(?:couple\\s+of|few|couple)\\s+(?:seconds?|minutes?|hours?|days?|weeks?|months?|years?|nights?)\\b|\\bhalf\\s+an?\\s+(?:hour|day|week|month)\\b)|(?P<share>\\b(?<![\\d.])\\d[\\d,]*(?:\\.\\d+)?\\s?(?:%|per\\s?cent\\b|percent\\b))|(?P<count>\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|(?<![\\d.])\\d[\\d,]*(?:\\.\\d+)?)\\s+[a-z][a-z-]{2,}\\b)|(?P<amount>\\b(?<![\\d.])\\d[\\d,]{2,}\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GAVE_LOOK_BACK(Py::Int(8));
    s_PAY_FAMILY(call_builtin("frozenset", &[mcall(&py_str("earn earning earned pay paid pays payslip wage salary income make making\nmade takehome"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PAY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:earn|earns|earning|earnings|pay|pays|paid|wage|wages|salary|salaries|income|make|makes|making|made)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PAY_VERB_AT_THE_END(stdlib_call("re", "compile", &[py_str("\\b(?:make|makes|earn|earns|paid|pay)\\s*[.?!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TAKE_HOME(stdlib_call("re", "compile", &[py_str("\\btak(?:e|es|en|ing)\\s+home\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_IM_ON_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m|i am|we'?re|we are)\\s+on\\s+(?:about\\s+|roughly\\s+|around\\s+)?[\\u00a3$£]?\\s*\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PAY_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:like|worth|enough|fair|reasonable|normal|average|typical|good|bad|ok|okay|competitive)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAY_WHEN_WORDS(call_builtin("frozenset", &[mcall(&py_str("current currently presently now moment latest today exact exactly\n    actual precisely roughly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEASURE_WORD(call_builtin("frozenset", &[mcall(&py_str("\n    day days night nights week weeks weekend weekends month months year years hour hours minute\n    minutes morning mornings afternoon afternoons evening evenings time times while"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOING_WORD(call_builtin("frozenset", &[mcall(&py_str("going away off there gone staying travelled traveled travelling\n    traveling returning back doing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_NOUNS(call_builtin("frozenset", &[mcall(&py_str("target amount total figure count number budget price cost age date\n    sum score"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GOAL_NOUNS(call_builtin("frozenset", &[mcall(&py_str("target goal aim objective ambition"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOAL_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:save|saving|saved|need|needs|needed|want|wants|wanted|aim|aiming|goal|target|objective|ambition|trying|try|plan|planning|afford)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_LINE(stdlib_call("re", "compile", &[py_str("\\b\\d[\\d,.]*\\s*(?:a|an|per|each|every)\\s+(?:day|week|month|year|fortnight)\\b|\\b(?:a|per|each|every)\\s+(?:day|week|month|year|fortnight)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION(stdlib_call("re", "compile", &[py_str("\\W*(?:how|what|which|when|where|who|why|do|did|does|can|could|should|would|will|shall|is|are|was|were|have|has)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WERE_ASKS(stdlib_call("re", "compile", &[py_str("\\A\\s*were\\s+(?:i|you|we|they|he|she|it|there|any|all|none|not|this|these|those|either)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_OF_TIME(stdlib_call("re", "compile", &[py_str("(?:\\d|\\u00a3|\\$|\\u20ac)\\s*$|\\b(?:once|twice|thrice|times|per|each|every|a|an)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SCHEDULED_AFTER(stdlib_call("re", "compile", &[py_str("\\s*(?:before|after|past|prior\\s+to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAS_THE_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:have|has|had|got|there's|there\\s+are|there\\s+is)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FIGURE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:called|named|call|spell|spelt|means?|stand\\s+for)\\b|\\btoo\\s+(?:much|many)\\b|\\benough\\b|\\bsafe\\b|\\bhealthy\\b|\\brecommend\\b|\\b(?:is|are|was|were)\\s+how\\s+(?:many|much)\\b|\\bhow\\s+do\\s+i\\b|\\bhow\\s+much\\s+(?:have|has|did)\\s+(?:i|we|they)\\b(?!\\s+(?:say|said|tell|told|mention|mentioned))|\\b(?:in total|altogether|all together|so far)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLAN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:should|can|could|shall)\\s+(?:i|we)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THE_PLAN_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s|\\s+is|\\s+are|\\s+was|\\s+were)?\\s+(?:the|a|our|my)\\s+plan\\b|\\bwhat\\s+(?:do|does|should)\\s+(?:i|we)\\s+do\\s+(?:first|next)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_NAMED_AFTER_THE_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:did|do|does|was|were|am|are)\\s+(?:i|we)\\s+(?:say|said|tell|told|think|want|wanted|need|needed|do|does|did)?\\s*(?:the|a|an|my|our|his|her|their|your)\\s+[a-z]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_AMOUNT_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:was|is|were|are)\\s+(?:it|that|this|they|those|these)\\s*(?:again|now|then)?\\s*[.?!]*\\s*\\Z|\\bhow\\s+much\\s+(?:did|do|does)\\s+(?:i|we)\\s+(?:say|need|want|have|owe|pay|spend|agree)\\s*(?:again|now|then|already|so\\s+far|in\\s+total|altogether|for\\s+it)?\\s*[.?!]*\\s*\\Z|\\bremind\\s+me\\s+how\\s+much\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_DURATION_ASK(stdlib_call("re", "compile", &[py_str("\\bdid\\s+i\\s+(?:say|want|choose|pick)\\b|\\bhow\\s+long\\s+(?:am|are|was|were)\\s+(?:i|we)\\s+(?:going|away|out|there|staying|off)\\b|\\bhow\\s+long\\s+have\\s+i\\s+had\\b|\\bhave\\s+i\\s+had\\b|\\bhow\\s+long\\s+(?:has|have)\\s+(?:it|that|this)\\s+been\\b|\\bhow\\s+long\\s+(?:was|is)\\s+(?:it|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOW_LONG_ITS_BEEN(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\s+(?:has|have|had)\\s+(?:[\\w']+\\s+){1,5}been\\s+(?:like\\s+(?:this|that)|going\\s+on|that\\s+way|this\\s+way|now|so\\s+far)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OWNS_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|the)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LINE_THAT_ASKS(stdlib_call("re", "compile", &[op_add(&py_str("\\A(?:\\W*(?:and|but|so|ok|okay|also|then|well|now|hmm|oh|yeah|yes|right|actually|sorry|plus|just)\\b[\\s,]*)*"), &gattr(&g_A_QUESTION(), "pattern")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CORRECTION(stdlib_call("re", "compile", &[py_str("^[\\s,.\\-]*(?:(?:oh|ok|okay|and|but|so|well|sorry|no|nope|actually|wait|oops|correction|my mistake|it'?s)\\b[\\s,;:.-]*){1,3}|^[\\s,.\\-]*(?:make\\s+(?:it|that)|i\\s+mean|we\\s+mean|scratch\\s+that|i\\s+meant|we\\s+meant)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MARKER_AT_THE_END(stdlib_call("re", "compile", &[py_str("\\s+(?:actually|really|honestly)\\s*[.!?]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NUMBER_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:\\d[\\d,]*(?:\\.\\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINERS(call_builtin("frozenset", &[mcall(&py_str("a an the and or but so of to in on at for with from by as then that this\n    is are was were it them us you"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_TAIL(op_bitor(&g_JOINERS(), &call_builtin("frozenset", &[mcall(&py_str("because cos coz cuz cause since"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_A_ONE_MORE(stdlib_call("re", "compile", &[py_str("\\b(?:more|extra|other|others|fewer|less)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GAIN(stdlib_call("re", "compile", &[py_str("\\b(?:one|two|three|four|\\d+|another)\\s+(?:more|extra|other)\\b|\\banother\\s+(?:one|guest|person|adult|kid|child|friend|mate|of)\\b|\\bplus\\s+(?:one|two|three|\\d+)\\b|\\bone\\s+of\\s+(?:my|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GROUP_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:us|we|them|they|people|guests?|party)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LOSS(stdlib_call("re", "compile", &[py_str("\\b(?:died|passed away|put (?:him|her|it|them) down|gave\\b[^.?!]{0,20}\\baway|rehomed|sold|stolen|went missing|ran away|eaten|no longer|not any ?more)\\b|\\bone\\s+of\\s+(?:them|those|these|the)\\b|\\bgave\\s+one\\b|\\bone\\s+(?:away|off|back)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_AGE(py_str("(?:\\d[\\d,]*(?:\\.\\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)"));
    s_A_LIST_OF_AGES(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:aged?|ages)\\s*"), &g__A_AGE()), &py_str("\\b\\s*(?:and|&|,)\\s*")), &g__A_AGE()), &py_str("\\b|\\b")), &g__A_AGE()), &py_str("\\b\\s*(?:and|&|,)\\s*")), &g__A_AGE()), &py_str("\\b\\s*(?:years?|yrs?)(?:\\s+old)?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+many\\s+([a-z'&-]+(?:\\s+[a-z'&-]+){0,3})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERIOD_WORD(call_builtin("frozenset", &[mcall(&py_str("hour hours day days week weeks fortnight fortnights month months\n    year years night nights morning mornings evening evenings weekend weekends christmas"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_IN_THE_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:once|twice|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d+)\\s+times?\\b|\\bevery\\s+(?:\\w+\\s+)?(?:hour|day|week|fortnight|month|year|morning|evening|night|weekend)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_dict(vec![(py_str("one"), py_str("1")), (py_str("two"), py_str("2")), (py_str("three"), py_str("3")), (py_str("four"), py_str("4")), (py_str("five"), py_str("5")), (py_str("six"), py_str("6")), (py_str("seven"), py_str("7")), (py_str("eight"), py_str("8")), (py_str("nine"), py_str("9")), (py_str("ten"), py_str("10")), (py_str("eleven"), py_str("11")), (py_str("twelve"), py_str("12")), (py_str("fifteen"), py_str("15")), (py_str("twenty"), py_str("20")), (py_str("thirty"), py_str("30")), (py_str("forty"), py_str("40")), (py_str("fifty"), py_str("50")), (py_str("sixty"), py_str("60")), (py_str("seventy"), py_str("70")), (py_str("eighty"), py_str("80")), (py_str("ninety"), py_str("90")), (py_str("hundred"), py_str("100")), (py_str("thousand"), py_str("1000"))]) }
    s_THE_SAME_NUMBER(__data7());
    s_A_PLURAL_ONLY_NOUN(call_builtin("frozenset", &[mcall(&py_str("people children men women feet teeth"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY_PAID_FOR(stdlib_call("re", "compile", &[py_str("\\b(?:paid|pay|pays|spent|spend|cost|costs|charged|gave|give|offered|quoted|put|set)\\b[^.?!]{0,25}?[\\u00a3$\\u20ac]?\\s?\\d[\\d.,]*[^.?!]{0,25}?\\b(?:for|on|towards)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AMOUNT_WANTED(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:much|many)\\b|\\bcost\\b|\\bcosts\\b|\\bprice\\b|\\bpaid\\b|\\bpay\\b|\\bspent\\b|\\bspend\\b|\\bbudget\\b|\\bfee\\b|\\btotal\\b|\\bworth\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TOLD_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:did\\s+i\\s+(?:say|tell|mention)|what\\s+did\\s+i\\s+(?:say|tell)|what\\s+was\\s+it\\s+i\\s+(?:said|told)|how\\s+(?:long|many|much)\\s+did\\s+i\\s+(?:say|tell))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data8() -> Py { py_tuple(vec![py_str("sorry"), py_str("apologise"), py_str("apologize"), py_str("going"), py_str("told"), py_str("tell"), py_str("said"), py_str("say"), py_str("mention"), py_str("remember"), py_str("again"), py_str("asked"), py_str("ask")]) }
    s_NOT_A_THING_ASKED(call_builtin("frozenset", &[__data8()]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NARROWING_ONLY(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|and|or|also|but|then|so|just|only|maybe|actually)[\\s,]+)*\n        (?:\n            (?:the|those|these|any|some|other|both|all)(?:\\s+[a-z']+){0,3}\\s+(?:one|ones)\n          | (?:one|ones)\n          | [a-z']+\\s+(?:one|ones)\n          | (?:something|anything|everything)\\s+[a-z']+\n          | (?:not|no)\\s+(?:too|very|that|so|more|as)\\s+[a-z']+\n          | (?:a\\s+bit\\s+)?(?:cheaper|closer|nearer|bigger|smaller|longer|shorter|quicker|faster|\n                easier|simpler|warmer|colder|quieter|lighter|nicer|better|worse)\\b\n          | (?:since|before|after|from|until|till|by|around|near|in|on|at)\\s+[\\w' ]{1,24}\n        )\n        (?:\\s+[a-z']+)*\n        \\s*[.!]?$"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NARROWING_WORDS(Py::Int(7));
    s_A_FIGURE_WORD(stdlib_call("re", "compile", &[py_str("\\d|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|ninety|hundred|thousand)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RECALL_CUE(pyrt::xrefs::xget("memory.recall", "LOOKS_BACK"));
    s_A_POINTS_BACK_HARD(stdlib_call("re", "compile", &[py_str("\\b(?:again|remind\\w*|remember|recall|earlier|same\\s+(?:again|one))\\b|\\b(?:you|u)\\s+(?:said|told)\\b|\\btold\\s+me\\b|\\bwhat(?:'s|s|\\s+is|\\s+are|\\s+was|\\s+were)?\\s+(?:my|our)\\b|\\bwhat\\s+did\\s+(?:i|we)\\s+(?:say|tell)\\b|\\b(?:am|are|did|do|does|was|were|have|has|had)\\s+(?:i|we)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGED_IT(stdlib_call("re", "compile", &[py_str("\\b(?:actually|instead|changed|change of plan|moved it|moved to|now it'?s|now we'?re|no longer|swapped|postponed|cancell?ed|put it back|make it|makes it|we'?ve moved)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _doings(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut i: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The verb families a line asks its work with.");
    words = call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[g_DOINGS()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5252 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk5252[0].clone(); let mut family: Py = __unpk5252[1].clone(); if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[family.clone()]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(i.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn asks_for_the_same_work(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut question: Py = Py::None;
    let mut said: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("True when a line that names nothing asks for the answer the question before it got.\n\n    This is the test a knowledge answer gives way to: a line like this has no thing of its own,\n    so the entry it reaches is a neighbour of the subject rather than the question asked - but\n    only when the work is the same work. A new verb is a new question and keeps its own answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&names_nothing(&[text.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.history", "about_the_chat", &[text.clone()])) {
    return Py::Bool(false);
    }
    asked = _doings(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)), &g_WHOLE_OF_IT()))) } }) {
    return Py::Bool(false);
    }
    whole = call_builtin("bool", &[op_bitand(&call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)), &g_WHOLE_OF_IT())]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&_recent_answers(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    question = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_same_thing(&[question.clone(), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    return { let __b = whole.clone(); if py_truthy(&__b) { __b } else { call_builtin("bool", &[op_bitand(&asked.clone(), &_doings(&[question.clone()]))]).unwrap_or_else(|e| exc::raise(e)) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn without_a_reset(args: &[Py]) -> Py {
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
    let _ = py_str("The line with a \"forget that\" lead taken off, so the new ask is read as the whole turn.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RESET_LEAD(), "sub", args))(&__args) };
    Py::None
}
pub fn _the_first_instruction_in_a_long_piece(args: &[Py]) -> Py {
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
    let mut verbs: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verbs") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verbs"))) };
    let mut piece: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first instruction out of a piece too long to hand over whole, else None.\n\n    The answer stops telling the person to do something at the comma where it moves on to what\n    will happen (\"...and in about eight weeks you will run 20 minutes without stopping\"), so the\n    words above that comma are the step. Only when they read as an instruction of their own: a\n    long piece is still passed over whole when what is above the first comma is not one.\n    ");
    piece = mcall(&geti(&mcall(&call_builtin("str", &[{ let __b = head.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str(",")]), &Py::Int(0)), "strip", &[py_str(" ,;:.")]);
    words = mcall(&piece.clone(), "split", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT_STEP())) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&verbs.clone(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))); if py_truthy(&__b) { __b } else { _a_verb_after_a_leading_phrase(&[piece.clone(), verbs.clone()]) } }) {
    return piece.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_verb_after_a_leading_phrase(args: &[Py]) -> Py {
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
    let mut verbs: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verbs") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verbs"))) };
    let mut at: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut tokens: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the instruction's verb sits just after a phrase like \"each week\" or \"half\".");
    tokens = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = head.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(" ,-")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("min", &[op_add(&g_LEADING_PHRASE_MAX(), &Py::Int(1)), call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    at = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&verbs.clone(), &geti(&tokens.clone(), &at.clone())))) {
    continue;
    }
    phrase = slice_or_raise(&tokens.clone(), &Py::None, &at.clone(), &Py::None);
    if py_truthy(&Py::Bool(py_contains(&g_AN_ARTICLE(), &geti(&phrase.clone(), &Py::Int(0))))) {
    continue;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrase.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_LEADING_PHRASE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AN_ARTICLE(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn only_said_it_back(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut kept: Py = Py::None;
    let mut low: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("Whether the reply only says the person's own line back, so nothing has been answered.\n\n    The words of the line that name something, less the ones a frame is built from, are nearly\n    all in the reply: that is the line said back with a frame around it, which files a statement\n    and answers nobody. It is here rather than in `_an_echo` because two readers need it and they\n    must agree: the repeat guard below reads it a turn at a time, and the composer's want reader\n    (`bot/compose/remark.py`, `a_want_said_back`) uses it to tell a receipt from an answer -\n    \"Got it — your son wants a tattoo.\" and \"Your son is 16 and wants a motorbike. That explains\n    it — no wonder it sits on you.\" are the same shape, and the shape is what decides it, not the\n    frame either one happens to use.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    low = mcall(&said.clone(), "lower", &[]);
    spoken = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ECHO_FRAME(), &w.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&spoken.clone()))) {
    return Py::Bool(false);
    }
    kept = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spoken.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&low.clone(), &w.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = op_ge(&call_builtin("len", &[spoken.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { op_ge(&kept.clone(), &call_builtin("max", &[Py::Int(2), op_floordiv(&op_mul(&call_builtin("len", &[spoken.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e))) } };
    Py::None
}
pub fn _an_echo(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turn: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turn") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turn"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The bot only said the person's own line back, or stalled, so nothing has been answered.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&only_said_it_back(&[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }, said.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_STALL(), "search", &[mcall(&said.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _names_no_topic(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turn: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turn") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turn"))) };
    let mut line: Py = Py::None;
    let _ = py_str("The line was a detail, not a subject of its own: \"we only have a small patio\" is about the\n    garden we are already on, while \"my mother in law is coming on sunday\" is a new thing.");
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[])))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_truthy(&{ let __b = mcall(&pyrt::xrefs::xcall("understanding", "understand", &[line.clone(), Py::None]), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }))]);
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
pub fn _only_echoes_between(args: &[Py]) -> Py {
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
    let mut back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: back"))) };
    let mut between: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether every turn after an answer `back` turns ago was a detail said back, and no more.\n\n    Both halves matter: the bot has to have only echoed the line or stalled, and the line has to\n    have named no subject of its own. \"we only have a small patio\" after the veg answer is the\n    same question again, but \"my mother in law is coming on sunday\" is a new thing, and the roast\n    timing is not the answer to it.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    between = if py_truthy(&op_gt(&back.clone(), &Py::Int(1))) { slice_or_raise(&turns.clone(), &op_add(&op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &back.clone()), &Py::Int(1)), &Py::None, &Py::None) } else { py_list(vec![]) };
    return { let __b = call_builtin("bool", &[between.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut turn: Py = __item.clone(); __out.push({ let __b = _an_echo(&[turn.clone()]); if !py_truthy(&__b) { __b } else { _names_no_topic(&[turn.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _a_help_ask(args: &[Py]) -> Py {
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
    let _ = py_str("True when the whole line asks for a way to help and names nothing of its own.\n\n    Three asks that read as one: \"is there anything i can do\", \"any way of making it easier\",\n    \"anything that helps\". None of them opens on a question word, so `QUESTION` below never\n    matched them and the line went on to be answered as a new question - \"any way of making it\n    easier\" straight after the answer about a toddler who will not settle got \"That's most of what\n    I know on it\", and \"is there anything i can do\" after the answer about waking at four got the\n    same shrug. The person asked for the part of the answer to do, which is what `START_ASK`\n    already reads when the line is written \"where do i start\".\n\n    The reading is the understanding engine's, so its fences hold here too: a negation (\"there\n    isnt anything i can do\"), a thing named after it (\"...about the mole on my back\") and a line\n    that states rather than asks (\"anything i can do is never enough\") are other turns.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_HELP_SHAPE(), "search", &[line.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_HELP_ASK_QUESTION_FIRST(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NAMED_AFTER_IT(), "search", &[line.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&a_more_ask(&[line.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&pyrt::xrefs::xcall("understanding", "understand", &[line.clone()]), "get", &[py_str("help_ask")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_for_the_move(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks which part of the answer in hand to do.\n\n    A start ask (\"where do i start\", \"what should i do first\"), a help ask (\"is there anything i\n    can do\") and a how ask with a pronoun in it (\"how do i clean it off\") are the same turn, so\n    every reader that answers one of them answers all three.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = says_where_to_start(&[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_IT(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { _a_help_ask(&[line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn names_nothing(args: &[Py]) -> Py {
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
    let mut help_ask: Py = Py::None;
    let mut lost: Py = Py::None;
    let _ = py_str("True when the line asks for something without naming anything of its own.");
    if py_truthy(&mcall(&g_JUST_A_NOD(), "match", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(false);
    }
    lost = call_builtin("bool", &[mcall(&g_AT_A_LOSS(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    help_ask = _a_help_ask(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_QUESTION(), "match", &[mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])]); if py_truthy(&__b) { __b } else { { let __b = lost.clone(); if py_truthy(&__b) { __b } else { help_ask.clone() } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_ABOUT_THE_BOT(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_WORDS_WANTED(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(false);
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.topics", "pairing", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(false);
    }
    if py_truthy(&help_ask.clone()) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[pyrt::xrefs::xcall("memory.topics", "words", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_thing_of_its_own(&[text.clone()]))) } };
    Py::None
}
pub fn _names_a_thing_of_its_own(args: &[Py]) -> Py {
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
    let mut lost: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The content words of the line that name something, in the ask's own frame left out.\n\n    This is the reading behind `names_nothing`, kept separate because the start asks need the\n    same test without the fences around it: \"my laptop keeps dropping the wifi in the back room,\n    what should i try first\" names a laptop, a wifi and a back room, so it is a question about\n    those and not a question about the answer just given.\n    ");
    lost = call_builtin("bool", &[mcall(&g_AT_A_LOSS(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    said = op_bitor(&op_bitor(&op_bitor(&op_bitor(&g_ASK(), &g_ASK_VERB()), &g_TIME()), &g_FILLER()), &g_NOT_A_THING());
    if py_truthy(&lost.clone()) {
    said = op_bitor(&said.clone(), &g_LOSS_WORDS());
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&said.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SELF(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())) } } } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn asks_to_put_it_right(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks whether the thing on the table can still be put right.\n\n    \"can i save it\", \"is it ruined\", \"can it be fixed\": the thing is the one the answer just\n    gave, so nothing about the line is new - it is a question about that answer, and the answer's\n    own first move is what it wants. A line that names a subject or a goal of its own (\"can i save\n    money on my heating\", \"can i fix my bike\") is a question of its own and keeps it.\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_PUT_RIGHT(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PUT_RIGHT_ASK(), "match", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("memory.topics", "pairing", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ABOUT_THE_BOT(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[line.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FILLER(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&w.clone(), "isdigit", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SELF(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PUT_RIGHT_WORDS(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } } } } } } } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    return Py::Bool(!py_truthy(&words.clone()));
    Py::None
}
pub fn a_step_of_the_same_answer(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks which of the answer's own steps to do first.\n\n    \"what should i delete first\" after the Mac storage answer carries one content word, and that\n    word (\"delete\") acts on the answer rather than naming a subject of its own, so the answer on\n    the screen still answers it. A line that names a thing (\"how do i delete a file\") is a\n    question of its own and keeps it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ORDER(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MOVE_ASK(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("memory.topics", "pairing", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { mcall(&g_ABOUT_THE_BOT(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[text.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FILLER(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&w.clone(), "isdigit", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } } } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_VERBS()) } };
    Py::None
}
pub fn _recent_answers(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen_a_card: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The bot's own answers in the recent past, newest first, each with the line it answered and\n    how many turns ago it was (1 is the turn just before this one).");
    out = py_list(vec![]);
    seen_a_card = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_LOOK_BACK()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    back = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_ANSWER_MODULES(), &mcall(&turn.clone(), "get", &[py_str("module")])))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HEDGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(50)) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_STORE_ANSWER()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("answers"))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_QUESTION(), "match", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.topics", "about_a_place_they_left_behind", &[said.clone(), memory.clone()])) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![back.clone(), call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), said.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn _the_answer_about_what_we_are_on(args: &[Py]) -> Py {
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
    let mut recent: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "recent") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: recent"))) };
    let mut about_it: Py = Py::None;
    let mut here: Py = Py::None;
    let mut item: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The recent answers that can be handed back, newest first, with the topic in hand first.\n\n    The newest answer is the answer to a follow-up that names nothing of its own, which is right\n    while the chat is still on it. Going back to an earlier topic is the case where it is not:\n    the conversation is on the boiler again, and the newest answer is about the school places the\n    line between them brought up. The store has already made the returned topic current, so an\n    answer about some *other* subject is not the answer in hand, and the answer to the topic we\n    are on is the one to draw on:\n\n        my boiler keeps losing pressure\n        my daughter is 6 and starts school in september\n        anyway back to the boiler      -> \"Back to the boiler, then. What did you want to know\n                                          about it?\"\n        what should i do first         -> \"The thing to do first: Apply through your council by\n                                          15 January ...\"   (the school places)\n\n    Only answers about the topic in hand are offered when the newest one is about something else;\n    when the newest answer *is* about the topic in hand the list is left exactly as it was, so\n    every other chat reads as before. The order of the answers about the topic in hand is kept,\n    so the newest of those is still tried first, and the guards in `answer_for` still decide\n    whether it may be handed back at all.\n    ");
    here = pyrt::xrefs::xcall("memory.topics", "current", &[memory.clone()]);
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[call_builtin("str", &[{ let __b = mcall(&{ let __b = here.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } py_set(__out) })() };
    let __aug1 = op_bitor(&mine.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = here.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() });
    mine = __aug1;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&recent.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mine.clone())) } }) {
    return recent.clone();
    }
    if py_truthy(&{ let __b = op_bitand(&mine.clone(), &call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "keywords", &[geti(&geti(&recent.clone(), &Py::Int(0)), &Py::Int(1))])]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { op_bitand(&mine.clone(), &call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "keywords", &[geti(&geti(&recent.clone(), &Py::Int(0)), &Py::Int(2))])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return recent.clone();
    }
    about_it = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&recent.clone()) { let mut item: Py = __item.clone(); if py_truthy(&op_bitand(&mine.clone(), &call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "keywords", &[geti(&item.clone(), &Py::Int(1))])]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(item.clone()); } } py_list(__out) })() };
    return { let __b = about_it.clone(); if py_truthy(&__b) { __b } else { recent.clone() } };
    Py::None
}
pub fn _nothing_but_acknowledgements(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut back: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: back"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Nothing has answered since that answer: only acknowledgements of the person's own line.\n\n    \"my back hurts\" -> the answer, \"ive tried stretching\" -> \"Got it - you've tried stretching\",\n    \"what else can i do\": the answer is still the last thing said about it, so a follow-up that\n    names nothing of its own is about that answer. A real answer in between (cooking, money)\n    means the chat has moved and the older paragraph is not what the line leans on.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = op_le(&back.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&back.clone(), &call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_add(&op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &back.clone()), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    return Py::Bool(false);
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ATTEMPT(), "search", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _about_the_same_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut asked: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut broken: Py = Py::None;
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut t: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The answer belongs to what the chat is still on, not to a subject that has moved.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_HEDGE(), "search", &[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return Py::Bool(true);
    }
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(true);
    }
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[asked.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[said.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    mine = op_bitor(&mine.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_keys_of(&[said.clone(), memory.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(w.clone()); } } py_set(__out) })() });
    if py_truthy(&{ let __b = theirs.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&mine.clone(), &theirs.clone()))) } }) {
    return Py::Bool(false);
    }
    broken = op_bitor(&call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[asked.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&op_bitand(&broken.clone(), &call_builtin("set", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[said.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ATTEMPT(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&op_ge(&call_builtin("len", &[op_bitand(&mine.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[line.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(w.clone()); } } py_set(__out) })() })]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_chat_is_still_on__dup1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said_before: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_before"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut now: Py = Py::None;
    let mut then: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("Whether the thing the chat is on now is still the thing this older answer was about.\n\n    `_about_the_same_thing` asks an older answer whether it is about the line it answered, and a\n    knowledge answer is about its own question by construction, so that reading cannot see a\n    subject that came in between. \"ive started running three times a week\" / \"i want to lose a\n    stone\" / \"how long will it take\" was answered \"20 to 30 minutes\" - the length of the runs,\n    from the answer two turns back - because the only thing the reader compared was the running\n    question with the running answer. The line the chat is on says which thing the person is\n    asking about now, so a figure said about something else is not handed back.\n    ");
    topic = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("topic")]) } else { Py::None };
    now = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[call_builtin("str", &[{ let __b = mcall(&{ let __b = topic.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&now.clone()))) {
    return Py::Bool(true);
    }
    then = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[said_before.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    return call_builtin("bool", &[op_bitand(&then.clone(), &now.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _short(args: &[Py]) -> Py {
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
    let mut p: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("The answer the way a friend repeats it: its first sentence, or two when the first is only\n    a lead-in, and never more than MOST characters.");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&said.clone(), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    short = geti(&parts.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LEAD_IN()); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    short = op_add(&op_add(&short.clone(), &py_str(" ")), &geti(&parts.clone(), &Py::Int(1)));
    }
    if py_truthy(&op_gt(&call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST())) {
    short = op_add(&mcall(&geti(&mcall(&slice_or_raise(&short.clone(), &Py::None, &g_MOST(), &Py::None), "rsplit", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "rstrip", &[py_str(",;:")]), &py_str("..."));
    }
    return short.clone();
    Py::None
}
pub fn _explains(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True for a sentence that says how something is, so holds no step of its own.");
    text = mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__SUBJECT_OPEN(), "match", &[text.clone()])))) {
    return Py::Bool(false);
    }
    head = geti(&stdlib_call("re", "split", &[py_str(":|,|;| - | \\u2014 "), text.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    return op_gt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__EXPLANATION_WORDS());
    Py::None
}
pub fn _a_step_comes_after(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether the sentence that explains a fault ends on the step (\"..., so clear the vents\").\n\n    `_explains` caps a description at its first clause and treats the rest of the sentence as more\n    description. A sentence that closes on \"so <verb>\" is different: everything after the \"so\" is\n    what to do, and skipping it lost the step of answers like \"It is cooling itself because it is\n    hot, so clear the vents with a can of air\".\n    ");
    text = call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g__SO_THE_STEP_FOLLOWS(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&_opens_an_instruction(&[slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _opens_an_instruction(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a piece of an answer opens on a verb an instruction can open on.");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    words = mcall(&mcall(&text.clone(), "strip", &[py_str(" ,.:;")]), "split", &[]);
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&IMPERATIVES.clone(), &mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(" ,.;:")]))) } };
    Py::None
}
pub fn _opening_move(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut explaining: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut index: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut leads_with_the_plan: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut shorter: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The instruction a sentence opens with, in the answer's own words, or None.\n\n    Only the start of the sentence counts: an instruction opens on its verb, and a word further\n    in is the thing it is about: \"Stop the cycle, unplug it and turn the water off\" opens on the\n    instruction, and \"Water at the front is usually the door seal\" is about water, though `water`\n    is also a verb in the list. A sentence that opens on a fact and then says what to do (\"A\n    dripping tap is nearly always a worn washer: turn the water off at the valve\") hands back what\n    follows its colon, and the whole of it, because the steps are the answer.\n\n    A sentence that leads with the plan (\"Start with run-walk: one minute of easy jogging\") hands\n    back what follows its colon too: the lead is the verb, and reading the lead as the move would\n    hand back \"start with run-walk\" and say start twice.\n    ");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    lead = geti(&stdlib_call("re", "split", &[py_str("\\s+[-\\u2014]\\s+"), mcall(&call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    if py_truthy(&{ let __b = mcall(&lead.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&lead.clone(), "strip", &[]), &mcall(&call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&lead.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } } } }) {
    found = _opening_move(&[lead.clone()]);
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    }
    leads_with_the_plan = call_builtin("bool", &[stdlib_call("re", "match", &[py_str("(?:start|begin) with\\b"), mcall(&part.clone(), "strip", &[]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    pieces = op_add(&py_list(vec![part.clone()]), &mcall(&part.clone(), "split", &[py_str(":")]));
    explaining = _explains(&[part.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    piece = __unpk2[1].clone();
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { explaining.clone() } }) {
    continue;
    }
    head = mcall(&g__SO_COMES_THE_STEP(), "sub", &[py_str(""), mcall(&piece.clone(), "strip", &[py_str(" ,.:;")])]);
    if py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[head.clone()])) {
    continue;
    }
    words = mcall(&head.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp31: Py = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(3), &__cmp31)) { op_le(&__cmp31, &Py::Int(40)) } else { Py::Bool(false) } }))) {
    continue;
    }
    first = mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(" ,.;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&first.clone(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_opens_an_instruction(&[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__NEGATES(), "match", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)])]))) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&IMPERATIVES.clone(), &first.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = leads_with_the_plan.clone(); if !py_truthy(&__b) { __b } else { op_gt(&index.clone(), &Py::Int(1)) } }))) {
    continue;
    }
    } else {
    if py_truthy(&Py::Bool(py_contains(&g__AFTER_A_NOUN(), &mcall(&mcall(&geti(&words.clone(), &Py::Int(1)), "lower", &[]), "strip", &[py_str(" ,.;:")])))) {
    continue;
    } else {
    if py_truthy(&{ let __b = stdlib_call("re", "match", &[py_str("(?:start|begin) with\\b"), head.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&part.clone(), &py_str(":"))) } }) {
    continue;
    } else {
    if py_truthy(&{ let __b = mcall(&g__WEIGHS_ONE_AGAINST(), "search", &[head.clone()]); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _opens_an_instruction(&[geti(&pieces.clone(), &op_add(&index.clone(), &Py::Int(1)))]) } } } }) {
    continue;
    }
    }
    }
    }
    head = mcall(&geti(&stdlib_call("re", "split", &[py_str("\\s+rather than\\b"), head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[head.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&head.clone(), &py_str(":"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&head.clone(), &py_str(", then "))) } }) {
    shorter = mcall(&geti(&mcall(&head.clone(), "split", &[py_str(", then ")]), &Py::Int(0)), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = { let __cmp41: Py = call_builtin("len", &[mcall(&shorter.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(3), &__cmp41)) { op_le(&__cmp41, &Py::Int(8)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { { let __b = _opens_an_instruction(&[shorter.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__HANGS(), &mcall(&geti(&mcall(&shorter.clone(), "split", &[]), &op_neg(&Py::Int(1))), "lower", &[]))) } } } }) {
    head = shorter.clone();
    }
    }
    return op_add(&mcall(&geti(&head.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&head.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return Py::None;
    Py::None
}
pub fn _a_question(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("Whether the sentence asks rather than tells.\n\n    The bot's own question is not a step: a wish to write a book is answered with \"tell me what\n    you last enjoyed and I'll narrow it down\", and handing that back as \"Start with this: Tell me\n    what you last enjoyed\" is the bot asking the person its own question as if it were advice.\n    A step never opens on an interrogative, and \"do the theory test early\" starts on a verb, so\n    verbs are not in the list.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&mcall(&text.clone(), "endswith", &[py_str("?")])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_QUESTIONS_THAT_ARE_NOT_STEPS(), "match", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_move(args: &[Py]) -> Py {
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
    let mut questions_are_steps: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "questions_are_steps") { v.clone() }
    else { Py::Bool(false) };
    let mut move_: Py = Py::None;
    let _ = py_str("The move in the answer, when it is a move and not the answer with a lead on it.\n\n    A move is one thing to do: \"Stop the cycle and turn the water off\". `_a_move_in` reads the\n    answer for the sentence that says what to do, and on an answer whose opening sentence says\n    what to do and how, all season and at what pace, it hands back thirty-five words of it. Said\n    under a lead (\"That's the one: start where you can already run 5km comfortably and build up\n    over about sixteen weeks: three runs a week (one long, two short and easy)...\") that is the\n    answer again with two words in front of it, and the ask that gets it is the one the answer\n    already answered. Past a step's length there is no one thing to do, so this reader stands\n    down and the answer comes back as the repeat it is.\n    ");
    move_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("questions_are_steps".to_string(), questions_are_steps.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_move_in(args))(&__args) };
    if py_truthy(&{ let __b = move_.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&move_.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WITH_ITS_METHOD()) } }) {
    return Py::None;
    }
    return move_.clone();
    Py::None
}
pub fn _a_move_in(args: &[Py]) -> Py {
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
    let mut questions_are_steps: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "questions_are_steps") { v.clone() }
    else { Py::Bool(false) };
    let mut IMPERATIVES: Py = Py::None;
    let mut a_conclusion: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut conditional: Py = Py::None;
    let mut head: Py = Py::None;
    let mut in_a_bracket: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut opened_inside: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The sentence of the answer that tells them what to do, or None.\n\n    `questions_are_steps` is for the reader that hands back what still stands of an answer the\n    person has just ruled part of out (`bot/memory/obstacle.py`): the bot's own question is part\n    of what stands (\"Then skip black pepper - the rest of it stands: tell me what is in the fridge\n    and I will pick properly\"), where as the first thing to do it is the bot asking itself.\n\n    A knowledge answer either opens with what to do (\"Stop the cycle, unplug it and turn the\n    water off\") or opens with what is true and ends with what to do (\"Most bad posture is the\n    chair and the sitting still: set the screen so the top of it is at eye level\"). The opening\n    sentence wins when it is an instruction: it is the first step, and the step is what was asked\n    for. Only when the answer opens on a fact is the last instruction the move.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&parts.clone()) {
    opening = _opening_move(&[geti(&parts.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = opening.clone(); if !py_truthy(&__b) { __b } else { { let __b = questions_are_steps.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_question(&[opening.clone()]))) } } } }) {
    return opening.clone();
    }
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()])) {
    continue;
    }
    named = mcall(&g_FIRST_THING(), "match", &[part.clone()]);
    if py_truthy(&named.clone()) {
    move_ = mcall(&mcall(&named.clone(), "group", &[py_str("move")]), "strip", &[py_str(" ,;:")]);
    if py_truthy(&{ let __b = { let __cmp21: Py = call_builtin("len", &[mcall(&move_.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp21)) { op_le(&__cmp21, &Py::Int(20)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[move_.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = questions_are_steps.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_question(&[move_.clone()]))) } } } } } }) {
    return op_add(&mcall(&geti(&move_.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&move_.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    head = _opening_move(&[part.clone()]);
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_CONDITION_CLAUSE(), "match", &[head.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_A_MOVE(), "match", &[head.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = questions_are_steps.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_question(&[head.clone()]))) } } } } } } } }) {
    return head.clone();
    }
    if py_truthy(&{ let __b = _explains(&[part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_step_comes_after(&[part.clone()]))) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&op_add(&py_list(vec![part.clone()]), &mcall(&part.clone(), "split", &[py_str(":")])));
    for __item in __iter3 {
    piece = __item.clone();
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    clauses = stdlib_call("re", "split", &[py_str("(?<!\\d),(?!\\d)| - |;| \\u2014 |\\. "), piece.clone()]).unwrap_or_else(|e| exc::raise(e));
    conditional = Py::Bool(false);
    in_a_bracket = Py::Int(0);
    let __iter4 = py_iter_or_raise(&clauses.clone());
    let mut __else5 = false;
    for __item in __iter4 {
    clause = __item.clone();
    head = mcall(&clause.clone(), "strip", &[py_str(" .:;")]);
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    continue;
    }
    opened_inside = op_gt(&in_a_bracket.clone(), &Py::Int(0));
    in_a_bracket = call_builtin("max", &[Py::Int(0), op_sub(&op_add(&in_a_bracket.clone(), &mcall(&head.clone(), "count", &[py_str("(")])), &mcall(&head.clone(), "count", &[py_str(")")]))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&opened_inside.clone()) {
    continue;
    }
    a_conclusion = call_builtin("bool", &[mcall(&g__SO_COMES_THE_STEP(), "match", &[head.clone()])]).unwrap_or_else(|e| exc::raise(e));
    head = mcall(&g__SO_COMES_THE_STEP(), "sub", &[py_str(""), head.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&head.clone(), "count", &[py_str("(")]), &mcall(&head.clone(), "count", &[py_str(")")])))) {
    continue;
    }
    if py_truthy(&{ let __b = a_conclusion.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__SO_DOES_NOT_ASK_SOMETHING_OF_THEM(), "match", &[head.clone()]) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_CONDITION_CLAUSE(), "match", &[head.clone()])) {
    conditional = Py::Bool(true);
    continue;
    }
    if py_truthy(&conditional.clone()) {
    continue;
    }
    if py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[head.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_MOVE(), "match", &[head.clone()])) {
    continue;
    }
    words = mcall(&head.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp61: Py = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp61)) { op_le(&__cmp61, &if py_truthy(&a_conclusion.clone()) { g_STEP_WITH_ITS_METHOD() } else { Py::Int(14) }) } else { Py::Bool(false) } }))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&IMPERATIVES.clone(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&questions_are_steps.clone())); if !py_truthy(&__b) { __b } else { _a_question(&[head.clone()]) } }) {
    continue;
    }
    break;
    }
    continue;
    if py_truthy(&{ let __b = stdlib_call("re", "match", &[py_str("(?:start|begin) with\\b"), head.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&part.clone(), &py_str(":"))) } }) {
    continue;
    }
    return op_add(&mcall(&geti(&head.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&head.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_conditional_move(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The answer's step that comes wrapped in its own condition, kept whole, or None.\n\n    An answer often puts the step behind the moment to do it: \"When you're ready, ask for one\n    line of feedback\", \"Once it has cooled, press it flat\". `_a_move` reads that condition and\n    stops, because a step for later is not the step to do first - which is right for \"what\n    should i do first\" and wrong for the person who says they do not know what to do at all:\n    there, the conditional step is the one thing the answer has to give them, and repeating the\n    answer's opening sentence (\"That's disappointing, especially after an interview\") says\n    nothing they can do. The condition is kept, so the step is not handed over as if it were\n    for now.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_MOVE(), "match", &[part.clone()]) } }) {
    continue;
    }
    match_ = mcall(&g_CONDITION_CLAUSE(), "match", &[part.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    step = mcall(&geti(&stdlib_call("re", "split", &[py_str("\\s+[-\\u2014]\\s+|\\s+\\("), mcall(&part.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(" ,.;:")]);
    after = mcall(&step.clone(), "split", &[py_str(","), Py::Int(1)]);
    rest = mcall(&if py_truthy(&op_gt(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&after.clone(), &Py::Int(1)) } else { slice_or_raise(&step.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None) }, "strip", &[py_str(" ,")]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp21: Py = call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(3), &__cmp21)) { op_le(&__cmp21, &Py::Int(14)) } else { Py::Bool(false) } }))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_opens_an_instruction(&[rest.clone()])))) {
    continue;
    }
    if py_truthy(&mcall(&g__A_JUDGEMENT(), "match", &[step.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp31: Py = call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(4), &__cmp31)) { op_le(&__cmp31, &Py::Int(24)) } else { Py::Bool(false) } }))) {
    continue;
    }
    if py_truthy(&_explains(&[step.clone()])) {
    continue;
    }
    return op_add(&mcall(&geti(&step.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return Py::None;
    Py::None
}
pub fn asks_for_more(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks for the next part of the answer in hand, and says nothing else.\n\n    \"what else\", \"anything else\", \"and after that\" - four words at most, because a line that says\n    what it is about (\"what else can i do about my nails\") has a subject of its own for the\n    entries to match. `names_nothing` reads \"anything else\" as naming something, so this ask needs\n    a reading of its own.\n    ");
    if py_truthy(&op_gt(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_NEXT_ASK(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_hand_back(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("Whether this bot line was handed back out of an earlier answer rather than being one.\n\n    A step is a clause of the answer it came from, so a turn that holds one is not the answer: what\n    to do with \"what else\" after it is the answer behind it, which `_handed_back` kept. Read off\n    what was handed back rather than off a list of lead-ins, so a frame added later is one of these\n    too.\n    ");
    wanted = slice_or_raise(&mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &Py::None, &Py::Int(60), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("repeated")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut line: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &wanted.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _handed_back_last(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the bot's last turn was a short form handed back out of an earlier answer.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return a_hand_back(&[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]), memory.clone()]);
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
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clause: Py = Py::None;
    let mut head: Py = Py::None;
    let mut one: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut under: Py = Py::None;
    let _ = py_str("The next part of the answer in hand and whether it says what to do, else (\"\", False).\n\n    The answer is walked in order, so \"what else\" after a step gets the part after the step rather\n    than the step again or the whole paragraph. A red flag is not a next step (\"Get it looked at\n    if it gives way\" is what to do in an emergency, not the next thing to try) and neither is a\n    sentence that asks the person for something, so both are skipped. A sentence that tells them\n    to do something wins over one that only explains, because \"what else\" asks for the next move -\n    but an answer of nothing but explanation still has more in it, so the first part is the\n    fallback and the caller says which of the two it got.\n    ");
    spare = py_str("");
    let __iter1 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); __out.push(mcall(&p.clone(), "strip", &[])); } py_list(__out) })() });
    for __item in __iter1 {
    part = __item.clone();
    part = mcall(&part.clone(), "rstrip", &[py_str(" .")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&part.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST()) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_THEM(), "search", &[part.clone()]) } }) {
    continue;
    }
    if py_truthy(&_already_repeated(&[part.clone(), memory.clone()])) {
    continue;
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+|so\\s+|then\\s+|after\\s+that,?\\s+|next,?\\s+)")); __args.push(py_str("")); __args.push(part.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^then\\s+")); __args.push(py_str("")); __args.push(part.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_gt(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(110))) {
    head = mcall(&geti(&mcall(&part.clone(), "split", &[py_str(": ")]), &Py::Int(0)), "strip", &[py_str(" ,;")]);
    if py_truthy(&op_ge(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    under = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",\\s+"), geti(&mcall(&part.clone(), "split", &[py_str(": "), Py::Int(1)]), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "strip", &[py_str(" ,;:")])); } py_list(__out) })() };
    under = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&under.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = under.clone(); if !py_truthy(&__b) { __b } else { _already_repeated(&[geti(&under.clone(), &Py::Int(0)), memory.clone()]) } }) {
    let __iter2 = py_iter_or_raise(&slice_or_raise(&under.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter2 {
    clause = __item.clone();
    if py_truthy(&{ let __b = _says_what_to_do(&[clause.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already_repeated(&[clause.clone(), memory.clone()]))) } }) {
    return py_tuple(vec![clause.clone(), Py::Bool(true)]);
    }
    }
    continue;
    }
    part = head.clone();
    if py_truthy(&_already_repeated(&[part.clone(), memory.clone()])) {
    continue;
    }
    }
    }
    if py_truthy(&_says_what_to_do(&[part.clone()])) {
    return py_tuple(vec![part.clone(), Py::Bool(true)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&spare.clone()))) {
    spare = part.clone();
    }
    }
    if py_truthy(&{ let __b = spare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_in_a_turn(&[spare.clone(), memory.clone()]))) } }) {
    return py_tuple(vec![spare.clone(), Py::Bool(false)]);
    }
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    Py::None
}
pub fn _said_in_a_turn(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether these words are already in a reply the bot gave, whichever turn it was.");
    said = mcall(&slice_or_raise(&mcall(&call_builtin("str", &[{ let __b = short.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), &Py::None, &Py::Int(60), &Py::None), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_what_to_do(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut at: Py = Py::None;
    let mut index: Py = Py::None;
    let mut orders: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the sentence tells them to do something rather than saying how things are.");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(" ,;:.")])); } py_list(__out) })() };
    orders = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bdo not\\b|\\bdon'?t\\b"), call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    at = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk1317 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk1317[0].clone(); let mut word: Py = __unpk1317[1].clone(); if py_truthy(&Py::Bool(py_contains(&IMPERATIVES.clone(), &word.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    return orders.clone();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &at.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("means"), py_str("sounds")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return orders.clone();
    }
    return Py::Bool(true);
    Py::None
}
pub fn _said_in_the_chat(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut opening: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the chat has already read this wording.\n\n    Checked the way a friend would: the opening of the line, whitespace flattened, against the\n    bot's own turns. A short form is a cut of an answer rather than a copy of one, so the opening\n    is what has to be found, not the whole line.\n    ");
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("..."), py_str(" ")]), "split", &[])]), "split", &[]), &Py::None, &Py::Int(10), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &opening.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _claims_what_it_never_said(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The line, with its claim to have said this before taken off when the claim does not hold.");
    line = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    low = mcall(&line.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_CLAIMS_IT_WAS_SAID());
    for __item in __iter1 {
    head = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&low.clone(), "startswith", &[head.clone()])))) {
    continue;
    }
    rest = mcall(&mcall(&slice_or_raise(&line.clone(), &call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "lstrip", &[py_str(" :-,—")]), "strip", &[]);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_in_the_chat(&[rest.clone(), memory.clone()]))) } }) {
    return op_add(&mcall(&slice_or_raise(&rest.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return line.clone();
    }
    return line.clone();
    Py::None
}
pub fn _handed_back(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut kept: Py = Py::None;
    let _ = py_str("Keep the line handed back, so the next bare ask sees it as already said.\n\n    A short form of an answer is not a copy of it (a move is one clause of it), so counting the\n    earlier answers alone would let the same clause come back a third time: the turn that handed\n    it back has to count as having said it.\n    ");
    reply = _claims_what_it_never_said(&[reply.clone(), memory.clone()]);
    kept = mcall(&memory.clone(), "get", &[py_str("repeated")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[kept.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    kept = py_list(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("repeated"), py_list(vec![]));
    }
    let _ = mcall(&kept.clone(), "append", &[reply.clone()]);
    let _ = py_delitem(&kept.clone(), &Py::None).unwrap_or(());
    return reply.clone();
    Py::None
}
pub fn _frame_heads(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut head: Py = Py::None;
    let mut heads: Py = Py::None;
    let mut item: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Every lead-in this module can put in front of a line, as the text it opens on.\n\n    Read off the templates rather than listed again, so a pool added later - here or below this\n    point in the file - is framed like the rest, and a frame can never be stacked on a frame the\n    test does not know about. Found on first use, when the whole module has been read.\n    ");
    if py_truthy(&g__FRAME_HEADS()) {
    return geti(&g__FRAME_HEADS(), &Py::Int(0));
    }
    heads = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("list", &[mcall(&call_builtin("globals", &[]).unwrap_or_else(|e| exc::raise(e)), "values", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    value = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&value.clone());
    for __item in __iter2 {
    item = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[item.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    head = mcall(&geti(&mcall(&item.clone(), "split", &[py_str("{"), Py::Int(1)]), &Py::Int(0)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&item.clone(), &py_str("{"))); if !py_truthy(&__b) { __b } else { head.clone() } }) {
    let _ = mcall(&heads.clone(), "add", &[mcall(&head.clone(), "lower", &[])]);
    }
    }
    }
    let _ = mcall(&g__FRAME_HEADS(), "append", &[call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(heads.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e))]);
    return geti(&g__FRAME_HEADS(), &Py::Int(0));
    Py::None
}
pub fn framed(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when this line already has one of this module's hand-back frames on it.\n\n    \"i dont follow\" after a line that repeat had framed came back as \"I said: It's what I said a\n    moment ago: ...\" - a frame stacked on a frame - because the module that answers \"what did you\n    mean\" only knew its own leads. A line that has already been handed back under a lead is the\n    answer's second telling, and framing it again says the same thing a third time.\n    ");
    line = mcall(&mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    return { let __b = call_builtin("bool", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&line.clone(), "startswith", &[_frame_heads(&[])]) } };
    Py::None
}
pub fn handed_back__wrapped(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let _ = py_str("The public form of `_handed_back`: a line the bot gave with a lead on it is kept.\n\n    Every module that hands a line over under a frame records it here, so nothing frames it again:\n    a frame on a frame is the bot saying the same thing a third time.\n    ");
    return _handed_back(&[memory.clone(), reply.clone()]);
    Py::None
}
pub fn _already_repeated(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Saying the same paragraph a third time reads worse than moving on: once the answer has\n    come back once, another \"what now\" wants the next step, not the same line again.");
    said = mcall(&slice_or_raise(&mcall(&short.clone(), "strip", &[]), &Py::None, &Py::Int(60), &Py::None), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone()))) {
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    }
    }
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("repeated")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter3 {
    line = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone()))) {
    let __aug4 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug4;
    }
    }
    return op_ge(&seen.clone(), &Py::Int(2));
    Py::None
}
pub fn _handed_back_already(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when this very step has already been handed back as the reply, not merely said.\n\n    `_already_repeated` asks whether the words are on the screen, and a step is one clause of the\n    answer it comes from, so the answer itself counts as one telling. That is right for a step\n    handed back a turn ago; it is wrong for the ask that comes after a paragraph, where the words\n    are there and the step never was: \"i keep burning the bottom of my pans\" / \"what am i doing\n    wrong\" (the answer again, framed) / \"how do i clean it off\" was answered with a search hedge,\n    because the first clause of the pan answer had now been \"said\" twice and the step once never.\n    ");
    said = mcall(&slice_or_raise(&mcall(&step.clone(), "strip", &[]), &Py::None, &Py::Int(60), &Py::None), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    reply = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&reply.clone(), &said.clone())) } }) {
    continue;
    }
    if py_truthy(&op_ge(&op_mul(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)), &call_builtin("len", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("repeated")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    line = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_lead_of_its_own(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("True when the short form already opens with a label of its own (\"Quick: ...\").");
    match_ = mcall(&g_OWN_LEAD(), "match", &[call_builtin("str", &[{ let __b = short.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    return { let __b = call_builtin("bool", &[match_.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_OWN_LEAD_WORDS()) } };
    Py::None
}
pub fn _kept_clause(args: &[Py]) -> Py {
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
    let _ = py_str("The words a shorter version has to keep, or \"\" when the line names none.");
    match_ = mcall(&g_KEEP_CLAUSE(), "search", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    return if py_truthy(&match_.clone()) { mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "strip", &[py_str(" .!?,")]) } else { py_str("") };
    Py::None
}
pub fn asks_for_less(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut left: Py = Py::None;
    let mut said: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True when the whole line, bar what it says to keep, is a request for a shorter answer.");
    clean = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_HOW_TO(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    asked = if py_truthy(&mcall(&g_KEEP_CLAUSE(), "search", &[clean.clone()])) { mcall(&g_KEEP_CLAUSE(), "sub", &[py_str(""), clean.clone()]) } else { clean.clone() };
    said = pyrt::xrefs::xcall("memory.topics", "words", &[asked.clone()]);
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_LESS_WORDS(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FILLER(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK_VERB(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } } } } } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&left.clone()) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_LESS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BREVITY_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::Bool(false);
    }
    settled = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.topics", "words", &[clean.clone()]) } }) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FILLER(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[settled.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LESS_WORDS(), &geti(&settled.clone(), &op_neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BREVITY_WORDS(), &geti(&settled.clone(), &op_neg(&Py::Int(1))))) } } } };
    Py::None
}
pub fn _recent_lines(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The bot's own recent lines that said something, newest first, with how many turns ago.\n\n    Wider than `_recent_answers`: a request for less is about whatever the bot just said, and the\n    line can be the small-talk answer as easily as a knowledge one (\"i feel a bit low\" / \"make it\n    short\"). A greeting, a hedge, a question back and a line too short to have any detail in it\n    are all still left out. The count is of lines that said something, not of turns: after two\n    hand-backs the answer they came from is still the line in hand, and counting the hand-backs\n    would put it out of reach of a third request for less.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_LOOK_BACK_FOR_LESS()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&g_HEDGE(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &g_WELCOMES())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_quotation(&[said.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&{ let __b = mcall(&memory.clone(), "get", &[py_str("repeated")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![op_add(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), said.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn _sentences(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut out: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The sentences of a line, split only where the line is not inside a quotation.\n\n    A quotation is one unit: an answer that shows the person what to write has a question mark in\n    it, and splitting there cut the message in half - \"Shorter: Ask early, name the dates and say\n    who covers: \"Could I take 12-16 May off?\" is a short answer that stops inside the sentence it\n    is quoting, and the next request cut the other way, starting mid-quote with the closing mark\n    left on it. So a boundary only counts once the quotes before it pair up.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    out = __unpk1[0].clone();
    last = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_SENTENCE_END(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    end = __item.clone();
    piece = mcall(&slice_or_raise(&text.clone(), &last.clone(), &mcall(&end.clone(), "end", &[]), &Py::None), "rstrip", &[]);
    if py_truthy(&_unbalanced(&[piece.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&piece.clone(), "strip", &[])) {
    let _ = mcall(&out.clone(), "append", &[mcall(&piece.clone(), "strip", &[])]);
    }
    last = mcall(&end.clone(), "end", &[]);
    }
    tail = mcall(&slice_or_raise(&text.clone(), &last.clone(), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&tail.clone()) {
    let _ = mcall(&out.clone(), "append", &[tail.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _most_condensed(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut long_parts: Py = Py::None;
    let mut p: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The one sentence of the answer that carries the most, for a second request for less.\n\n    A sentence already handed back is left out: after \"Shorter: <the lead>\", the sentence that\n    carries the rest of the answer is the one thing they have not read twice, and handing back\n    the lead again under a second lead-in is the one thing that makes a short answer longer.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_sentences(&[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); __out.push(mcall(&mcall(&p.clone(), "strip", &[]), "strip", &[py_str(",;:")])); } py_list(__out) })() };
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = { let __cmp11: Py = call_builtin("len", &[mcall(&p.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(4), &__cmp11)) { op_le(&__cmp11, &Py::Int(30)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[p.clone()]))) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut p: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_already_repeated(&[p.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])))) {  __out.push(p.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    long_parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_sentences(&[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); __out.push(mcall(&mcall(&p.clone(), "strip", &[]), "strip", &[py_str(",;:")])); } py_list(__out) })() };
    long_parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&long_parts.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = { let __cmp11: Py = call_builtin("len", &[mcall(&p.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(4), &__cmp11)) { op_le(&__cmp11, &Py::Int(45)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[p.clone()]))) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    long_parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&long_parts.clone()) { let mut p: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_already_repeated(&[p.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])))) {  __out.push(p.clone()); } } py_list(__out) })() };
    if py_truthy(&long_parts.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(long_parts.clone()); __kws.extend(vec![("key".to_string(), { let long_parts = long_parts.clone(); let memory = memory.clone(); let p = p.clone(); let parts = parts.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let long_parts = long_parts.clone(); let memory = memory.clone(); let p = p.clone(); let parts = parts.clone(); let said = said.clone(); let p: Py = __kwp(args, 0, "p").unwrap_or(Py::None); call_builtin("len", &[mcall(&p.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); __kws.extend(vec![("key".to_string(), { let long_parts = long_parts.clone(); let memory = memory.clone(); let p = p.clone(); let parts = parts.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let long_parts = long_parts.clone(); let memory = memory.clone(); let p = p.clone(); let parts = parts.clone(); let said = said.clone(); let p: Py = __kwp(args, 0, "p").unwrap_or(Py::None); call_builtin("len", &[mcall(&p.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _short_enough(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cut: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cut") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cut"))) };
    let mut head: Py = Py::None;
    let mut out: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut together: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The answer's shorter read, trimmed to something a person would call short.\n\n    A short lead is handed back as it is. A long one is cut at its own colon first - the place an\n    answer often offers its own summary (\"... and staying active is what helps most: keep walking\n    ...\") - and then at the last clause boundary inside the cap, never ending on a word that\n    leaves the sentence hanging.\n    ");
    cut = mcall(&call_builtin("str", &[{ let __b = cut.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT_ENOUGH())) {
    return _ended(&[cut.clone()]);
    }
    head = _read_up_to_the_colon(&[cut.clone()]);
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT_ENOUGH())) {
    return _ended(&[head.clone()]);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_CLAUSE_SPLIT(), "split", &[head.clone()])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    out = py_str("");
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    part = __item.clone();
    together = mcall(&op_add(&op_add(&out.clone(), &py_str(" ")), &part.clone()), "strip", &[]);
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&together.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT_ENOUGH()) } }) {
    break;
    }
    out = together.clone();
    }
    words = mcall(&mcall(&out.clone(), "rstrip", &[py_str(",;")]), "split", &[]);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CUT_MID_SENTENCE(), &mcall(&mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(",;")]))) } }) {
    words = slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    out = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "rstrip", &[py_str(",;")]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&out.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))) {
    return _ended(&[head.clone()]);
    }
    return _ended(&[out.clone()]);
    Py::None
}
pub fn _ended(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cut: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cut") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cut"))) };
    let _ = py_str("A short answer cut out of a sentence still ends like a sentence.\n\n    A cut that ends on a quotation needs no full stop of its own when the quoted sentence already\n    has one: `... nothing is due that week.\"\" is finished, and a stop after the closing mark says it\n    twice.\n    ");
    cut = mcall(&mcall(&call_builtin("str", &[{ let __b = cut.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "rstrip", &[py_str(",;:")]);
    if py_truthy(&Py::Bool(py_contains(&py_str(".!?"), &slice_or_raise(&cut.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)))) {
    return cut.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("\"”"), &slice_or_raise(&cut.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str(".!?"), &slice_or_raise(&cut.clone(), &op_neg(&Py::Int(2)), &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return cut.clone();
    }
    return op_add(&cut.clone(), &py_str("."));
    Py::None
}
pub fn _unbalanced(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut closes: Py = Py::None;
    let mut opens: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a piece opens a quotation it does not close, so it is the middle of one.\n\n    A straight quote is its own mark, so an odd number of them is a quotation left open; the curly\n    pair is counted against itself.\n    ");
    said = call_builtin("str", &[{ let __b = piece.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_QUOTE_PAIRS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    opens = __unpk2[0].clone();
    closes = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&opens.clone(), &closes.clone()))) {
    if py_truthy(&op_modulo(&mcall(&said.clone(), "count", &[opens.clone()]), &Py::Int(2))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&Py::Bool(!py_eq(&mcall(&said.clone(), "count", &[opens.clone()]), &mcall(&said.clone(), "count", &[closes.clone()])))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_quotation(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut closes: Py = Py::None;
    let mut end: Py = Py::None;
    let mut opens: Py = Py::None;
    let mut quote: Py = Py::None;
    let _ = py_str("The draft the answer handed over inside its own quotation marks, or \"\".\n\n    \"Here's something you could send: \"Hi, basically I want Friday off. Thanks.\"\" is a draft with\n    a lead in front of it, and asking for less of a message asks for the message: cutting the\n    lead leaves the whole draft with a shorter introduction, which is not a short version of it.\n    ");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_QUOTE_PAIRS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    opens = __unpk2[0].clone();
    closes = __unpk2[1].clone();
    at = mcall(&said.clone(), "find", &[opens.clone()]);
    while py_truthy(&Py::Bool(!py_eq(&at.clone(), &op_neg(&Py::Int(1))))) {
    end = mcall(&said.clone(), "find", &[closes.clone(), op_add(&at.clone(), &call_builtin("len", &[opens.clone()]).unwrap_or_else(|e| exc::raise(e)))]);
    if py_truthy(&Py::Bool(!py_eq(&end.clone(), &op_neg(&Py::Int(1))))) {
    quote = mcall(&slice_or_raise(&said.clone(), &op_add(&at.clone(), &call_builtin("len", &[opens.clone()]).unwrap_or_else(|e| exc::raise(e))), &end.clone(), &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[mcall(&quote.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { op_ge(&op_mul(&call_builtin("len", &[mcall(&quote.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return quote.clone();
    }
    }
    at = mcall(&said.clone(), "find", &[opens.clone(), op_add(&at.clone(), &call_builtin("len", &[opens.clone()]).unwrap_or_else(|e| exc::raise(e)))]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _closing_the_quote(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cut: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cut") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cut"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut end: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The cut, carried on to the end of the quotation it stops inside.\n\n    A short answer may stop inside a quotation as long as it stops after it: \"... say who covers:\n    \"Could I take 12-16 May off? I've arranged cover with Priya.\"\" reads as a whole. When the\n    answer has no end for the quotation the cut is left as it is - that is the module below\n    saying nothing it can shorten, not a line to invent the rest of.\n    ");
    cut = mcall(&call_builtin("str", &[{ let __b = cut.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    whole = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&cut.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_unbalanced(&[cut.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&whole.clone(), &cut.clone())) } } } }) {
    return cut.clone();
    }
    rest = slice_or_raise(&whole.clone(), &op_add(&mcall(&whole.clone(), "index", &[cut.clone()]), &call_builtin("len", &[cut.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("[.!?][\\\"\\u201d]?"), rest.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    end = __item.clone();
    longer = mcall(&op_add(&cut.clone(), &slice_or_raise(&rest.clone(), &Py::None, &mcall(&end.clone(), "end", &[]), &Py::None)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&_unbalanced(&[longer.clone()])))) {
    return longer.clone();
    }
    }
    return cut.clone();
    Py::None
}
pub fn _read_up_to_the_colon(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let _ = py_str("The sentence, or the part of it before its own colon when that is a shorter read of it.");
    head = geti(&mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "partition", &[py_str(":")]), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&head.clone(), &call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(8), &__cmp11)) { op_lt(&__cmp11, &call_builtin("len", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) } } } }) {
    return mcall(&head.clone(), "strip", &[]);
    }
    return call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _keeps(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut counted: Py = Py::None;
    let mut named: Py = Py::None;
    let mut p: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The sentence of the answer that carries what the person said the short version has to keep.\n\n    A word of the requirement first (\"the water ratio\" is the sentence with the mugs of water in\n    it), then the figures when the requirement is about numbers (\"keep the timings\" is in the\n    minutes, and no sentence of the answer has the word \"timings\" in it). Empty when the answer\n    has neither, and then the lead is the short version as before.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    words = op_sub(&op_sub(&call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[what.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[pyrt::xrefs::xget("memory.topics", "STOPWORDS")]).unwrap_or_else(|e| exc::raise(e))), &py_set(vec![py_str("timings"), py_str("timing"), py_str("numbers"), py_str("figures"), py_str("details")]));
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut p: Py = __item.clone(); if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[p.clone()])]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(p.clone()); } } py_list(__out) })() };
    if py_truthy(&named.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&mcall(&g_FIGURES(), "search", &[what.clone()])) {
    counted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&g_DIGIT(), "search", &[p.clone()])) {  __out.push(p.clone()); } } py_list(__out) })() };
    if py_truthy(&counted.clone()) {
    return geti(&counted.clone(), &Py::Int(0));
    }
    }
    return py_str("");
    Py::None
}
pub fn shorter_for(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut carry: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut p: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut quote: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_any: Py = Py::None;
    let _ = py_str("The last answer's own lead, or None when this is not a request for less.\n\n    Three shapes, one per ask, because the second request cannot be answered with the sentence\n    the first one just gave: the lead of the answer, then the single sentence of it that carries\n    the most, and then the honest \"that is the short version\". A request for less never changes\n    the subject and never looks anything up.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_for_less(&[text.clone()]))) } }) {
    return Py::None;
    }
    said_any = py_str("");
    let __iter1 = py_iter_or_raise(&_recent_lines(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    back = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&op_gt(&back.clone(), &Py::Int(2))) {
    break;
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_sentences(&[said.clone()])) { let mut p: Py = __item.clone(); __out.push(mcall(&mcall(&p.clone(), "strip", &[]), "strip", &[py_str(";")])); } py_list(__out) })() };
    said_any = { let __b = said_any.clone(); if py_truthy(&__b) { __b } else { said.clone() } };
    lead = if py_truthy(&op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&parts.clone(), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&{ let __b = lead.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[mcall(&lead.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } }) {
    lead = geti(&parts.clone(), &Py::Int(1));
    }
    if py_truthy(&Py::Bool(!py_truthy(&lead.clone()))) {
    lead = mcall(&_read_up_to_the_colon(&[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))]), "strip", &[]);
    }
    kept = _kept_clause(&[text.clone()]);
    if py_truthy(&kept.clone()) {
    carry = _keeps(&[said.clone(), kept.clone()]);
    if py_truthy(&carry.clone()) {
    return _handed_back(&[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("keep".to_string(), kept.clone()), ("short".to_string(), carry.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_KEEPING(), memory.clone(), text.clone()]), "format", args))(&__args) }]);
    }
    }
    quote = _the_quotation(&[said.clone()]);
    if py_truthy(&{ let __b = quote.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&quote.clone(), "strip", &[]), &mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already_repeated(&[quote.clone(), memory.clone()]))) } } } }) {
    return _handed_back(&[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("short".to_string(), quote.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_JUST_THIS(), memory.clone(), text.clone()]), "format", args))(&__args) }]);
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_LESS_LEADS(), lead.clone()]), py_tuple(vec![g_JUST_THIS(), _most_condensed(&[said.clone(), memory.clone()])])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    pool = __unpk4[0].clone();
    cut = __unpk4[1].clone();
    cut = _closing_the_quote(&[_short_enough(&[cut.clone()]), said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&cut.clone())); if py_truthy(&__b) { __b } else { _unbalanced(&[cut.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&cut.clone(), "strip", &[]), "lower", &[]), &mcall(&mcall(&said.clone(), "strip", &[]), "lower", &[])))) {
    continue;
    }
    if py_truthy(&_already_repeated(&[cut.clone(), memory.clone()])) {
    continue;
    }
    line = pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&line.clone(), &py_str("{short}"))); if !py_truthy(&__b) { __b } else { _a_lead_of_its_own(&[cut.clone()]) } }) {
    line = py_str("{short}");
    }
    return _handed_back(&[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("short".to_string(), cut.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said_any.clone()))) {
    return _handed_back(&[memory.clone(), pyrt::xrefs::xcall("compose", "pick", &[g_NOTHING_TO_CUT(), memory.clone(), text.clone()])]);
    }
    if py_truthy(&said_any.clone()) {
    one = _most_condensed(&[said_any.clone(), memory.clone()]);
    if py_truthy(&one.clone()) {
    return _handed_back(&[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("short".to_string(), _ended(&[one.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_ALREADY_SHORT(), memory.clone(), text.clone()]), "format", args))(&__args) }]);
    }
    return _handed_back(&[memory.clone(), pyrt::xrefs::xcall("compose", "pick", &[g_NO_SHORTER_STILL(), memory.clone(), text.clone()])]);
    }
    return Py::None;
    Py::None
}
pub fn _a_bare_time(args: &[Py]) -> Py {
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
    let _ = py_str("The part of an answer when it is nothing but the length asked for, else None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    part = mcall(&mcall(&py_str(" "), "join", &[mcall(&part.clone(), "split", &[])]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&part.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_BARE_TIME(), "match", &[part.clone()]))) } } } }) {
    return Py::None;
    }
    return op_add(&mcall(&geti(&part.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _names_no_thing_of_its_own(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks about a price without naming what the price is for.");
    words = pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTING_BACK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_POINTING_BACK(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PRICE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_price_the_answer_gave(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let _ = py_str("The price the answer in hand stated, when the line asks what that costs, else None.\n\n    \"the boiler is old\" / \"should i get it serviced\" (\"Once a year, yes: a gas boiler service is\n    usually 60 to 120 pounds ...\") / \"how much is that usually\" was answered with the price of a\n    new boiler, because the module that knows about boilers took the question and the answer on\n    the screen already said the figure. An ask that points at the last answer rather than naming\n    its own thing is a question about that answer, and an answer that states one price has\n    nothing to choose between: that price is what it says it costs.\n\n    Two prices are two different things (\"a patch is 150 to 400, a whole roof 3,000\") and the one\n    the person means is not known, so an answer with two is left alone.\n    ");
    part = _the_price_in_hand(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    return Py::None;
    }
    if py_truthy(&_a_length_already_handed(&[part.clone(), memory.clone()])) {
    return Py::None;
    }
    return _handed_back(&[memory.clone(), part.clone()]);
    Py::None
}
pub fn asks_what_that_costs(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks the price of the thing the answer in hand has just priced.\n\n    The turn is this reading's in the same way a check of a fact is: the line names nothing of\n    its own, so every entry about the word in it (\"how much is that usually\" after a boiler\n    service) answers the ask by pricing its own subject instead of the figure on the screen.\n    ");
    return call_builtin("bool", &[_the_price_in_hand(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_price_in_hand(args: &[Py]) -> Py {
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
    let mut hit: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut part: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bare price the answer in hand gave, when the line asks what that costs, else None.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_PRICE_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&pyrt::xrefs::xcall("memory.narrow", "kind_of", &[line.clone()]), &py_str("money"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_no_thing_of_its_own(&[line.clone()])))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::None;
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&geti(&pyrt::xrefs::xget("memory.narrow", "SPANS"), &py_str("money")), "finditer", &[last.clone()])) { let mut hit: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&hit.clone(), "group", &[Py::Int(0)]), "lower", &[]), "split", &[])])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    part = pyrt::xrefs::xcall("memory.narrow", "narrow", &[last.clone(), line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&geti(&pyrt::xrefs::xget("memory.narrow", "SPANS"), &py_str("money")), "search", &[part.clone()]))) } }) {
    return Py::None;
    }
    part = mcall(&py_str(" "), "join", &[mcall(&part.clone(), "split", &[])]);
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&part.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&part.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])))) {
    let __aug1 = op_add(&part.clone(), &py_str("."));
    part = __aug1;
    }
    return op_add(&mcall(&geti(&part.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _the_time_an_earlier_answer_gave(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut last: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The length an answer further back gave, when the line asks for one and the last one does\n    not hold it.\n\n    \"i want to run a 5k\" (\"Three runs a week gets most people to 5k in about eight weeks\") /\n    \"im 45 and havent run since school\" (\"Come back at about half of what you used to do ...\") /\n    \"how long to train\" was met with \"That's beyond what I know well, sorry\", a search hedge,\n    while \"eight weeks\" sat in the chat two turns back. The answer in hand is the wrong place to\n    look: it is about how hard to come back, not how long, and narrowing it gives no length.\n    ");
    said_line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_LENGTH_ASK(), "search", &[said_line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&pyrt::xrefs::xcall("memory.narrow", "kind_of", &[said_line.clone()]), &py_str("time"))) } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    last = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { _a_bare_time(&[pyrt::xrefs::xcall("memory.narrow", "narrow", &[last.clone(), said_line.clone()])]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&_recent_answers(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    asked = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&{ let __b = op_lt(&back.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { { let __b = op_gt(&back.clone(), &g_LENGTH_BACK()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&geti(&pyrt::xrefs::xget("memory.narrow", "SPANS"), &py_str("time")), "search", &[said.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_about_the_same_thing(&[asked.clone(), said.clone(), memory.clone()])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_chat_is_still_on(&[op_add(&op_add(&asked.clone(), &py_str(" ")), &said.clone()), memory.clone()])))) {
    continue;
    }
    part = _a_bare_time(&[pyrt::xrefs::xcall("memory.narrow", "narrow", &[said.clone(), said_line.clone()])]);
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_length_already_handed(&[part.clone(), memory.clone()]))) } }) {
    return _handed_back(&[memory.clone(), part.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_length_already_handed(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut flat: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when this length has been handed back already in this chat.\n\n    Once \"Eight weeks.\" has answered \"how long to train\", the next length ask in the same chat is\n    about a different length: \"and how long to recover\" was given the same \"Eight weeks.\" - the\n    time to reach 5k, handed over as the time to recover from a run, which is a confident wrong\n    number where a hedge is honest. A length is spent once it has been handed over.\n    ");
    kept = mcall(&memory.clone(), "get", &[py_str("repeated")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[kept.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    flat = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .")]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut said: Py = __item.clone(); __out.push(Py::Bool(py_eq(&flat.clone(), &mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_what_to_do_answer(args: &[Py]) -> Py {
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
    let mut FREE_TIME: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut blocks: Py = Py::None;
    let mut key: Py = Py::None;
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the answer in hand is itself the answer to \"what should i do\": a plan, or the\n    free time to fill.\n\n    \"What can i do with that\" is a use ask - a dish, or a purpose for the thing the last answer\n    was about - and repeating that answer under a lead is wrong for it. But the pronoun can point\n    at the free time that was just planned: \"got the day off\" / \"no plans yet though\" / \"what\n    should i do with it\" asks the same thing as \"what should i do now\", and it reached a search\n    hedge. Only the answer in hand's own keys decide it: a recipe's keys name its ingredients and\n    a plan's name the event or the free block, in the plans domain's own words.\n    ");
    FREE_TIME = pyrt::xrefs::xget("knowledge.plans", "FREE_TIME");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    answer = mcall(&memory.clone(), "get", &[py_str("entry_answer")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[answer.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    blocks = py_set(vec![py_str("plan"), py_str("plans"), py_str("planning"), py_str("weekend"), py_str("weekends"), py_str("bored"), py_str("dayoff")]);
    let __iter1 = py_iter_or_raise(&FREE_TIME.clone());
    for __item in __iter1 {
    one = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&blocks.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&blocks.clone(), "update", &[words.clone()]);
    }
    }
    return call_builtin("bool", &[op_bitand(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&answer.clone(), "get", &[py_str("keys")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut key: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() }, &blocks.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_city_guide_is_the_answer_in_front(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut travel: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the reply on the screen is a city's list of places to go.\n\n    A city card is a list of places, not a list of steps, so \"what should we do first\" after it\n    asks which of those places comes first - and the city row answers that itself. Read as a step\n    of the card it came back \"The thing to do first: Eat lunch at 2pm like locals\", which is a\n    habit, not the first place. The travel cards are imported here rather than at the top because\n    they are the only thing in this module that reads them, and `answers` asks the same question\n    of the same helpers (see `_a_city_card_asks_which_first`).\n    ");
    travel = pyrt::xrefs::xmodule("knowledge.travel");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return mcall(&travel.clone(), "a_city_card_is_in_front", &[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")])]);
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut _wording: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut later: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut limit_line: Py = Py::None;
    let mut more: Py = Py::None;
    let mut nothing_more: Py = Py::None;
    let mut part: Py = Py::None;
    let mut price_said: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut said: Py = Py::None;
    let mut short: Py = Py::None;
    let mut step: Py = Py::None;
    let mut t: Py = Py::None;
    let mut time_said: Py = Py::None;
    let mut travel: Py = Py::None;
    let mut tried: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut which_part: Py = Py::None;
    let _ = py_str("The answer they just asked for again, or None when this is not that turn.");
    limit_line = pyrt::xrefs::xmodule("compose.limit");
    if py_truthy(&{ let __b = asks_for_the_move(&[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _a_city_guide_is_the_answer_in_front(&[memory.clone()]) } } } }) {
    travel = pyrt::xrefs::xmodule("knowledge.travel");
    if py_truthy(&mcall(&travel.clone(), "asks_which_one_first", &[text.clone()])) {
    return Py::None;
    }
    }
    more = { let __b = asks_for_more(&[text.clone()]); if !py_truthy(&__b) { __b } else { _handed_back_last(&[memory.clone()]) } };
    if py_truthy(&{ let __b = mcall(&g_USE_WITH(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_what_to_do_answer(&[memory.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_WORDING(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    time_said = _the_time_an_earlier_answer_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&time_said.clone()) {
    return time_said.clone();
    }
    price_said = _the_price_the_answer_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&price_said.clone()) {
    return price_said.clone();
    }
    put_right = asks_to_put_it_right(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = names_nothing(&[text.clone()]); if py_truthy(&__b) { __b } else { a_step_of_the_same_answer(&[text.clone()]) } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = more.clone(); if py_truthy(&__b) { __b } else { { let __b = says_where_to_start(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_IT(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { { let __b = a_more_ask(&[text.clone()]); if py_truthy(&__b) { __b } else { put_right.clone() } } } } } } } })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_IDEAS_ONLY(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { _an_entry_answers_it(&[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    which_part = asks_for_the_move(&[text.clone()]);
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&which_part.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_REFER_BACK(), "search", &[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { mcall(&limit_line.clone(), "restates_the_answer", &[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return Py::None;
    }
    tried = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    wanted = pyrt::xrefs::xcall("memory.narrow", "kind_of", &[text.clone()]);
    let __iter1 = py_iter_or_raise(&_the_answer_about_what_we_are_on(&[memory.clone(), _recent_answers(&[memory.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    asked = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_about_the_same_thing(&[asked.clone(), said.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_only_echoes_between(&[memory.clone(), back.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_nothing_but_acknowledgements(&[back.clone(), memory.clone()]))) } } } } } }) {
    continue;
    }
    if py_truthy(&_a_new_work(&[text.clone(), asked.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&wanted.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.narrow", "carries", &[said.clone(), wanted.clone()]))) } }) {
    continue;
    }
    nothing_more = Py::Bool(false);
    if py_truthy(&more.clone()) {
    let __unpk3 = py_unpack(&_another_part(&[said.clone(), memory.clone()]), 2, None);
    part = __unpk3[0].clone();
    doing = __unpk3[1].clone();
    if py_truthy(&part.clone()) {
    leads = if py_truthy(&doing.clone()) { g_NEXT_LEADS() } else { g_ALSO_LEADS() };
    return _handed_back(&[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), op_add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[leads.clone(), memory.clone(), text.clone()]), "format", args))(&__args) }]);
    }
    }
    if py_truthy(&{ let __b = tried.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_QUESTION_MORE(), "search", &[tried.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _tried(&[tried.clone()]); if py_truthy(&__b) { __b } else { _a_step_done(&[tried.clone(), said.clone()]) } } } } } }) {
    later = _later_part(&[tried.clone(), said.clone()]);
    if py_truthy(&later.clone()) {
    short = _short(&[later.clone()]);
    return _handed_back(&[memory.clone(), _a_lead_and_a_step(&[_a_lead_for(&[_the_lead_for(&[text.clone(), short.clone()]), short.clone(), memory.clone(), text.clone()]), short.clone()])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&a_more_ask(&[text.clone()])))) {
    continue;
    }
    }
    if py_truthy(&a_more_ask(&[text.clone()])) {
    step = _the_next_step(&[said.clone(), memory.clone()]);
    if py_truthy(&step.clone()) {
    _wording = op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    return _handed_back(&[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), _wording.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_MORE_LEADS(), memory.clone(), text.clone()]), "format", args))(&__args) }]);
    }
    nothing_more = Py::Bool(true);
    }
    if py_truthy(&asks_for_the_move(&[text.clone()])) {
    step = _the_first_step(&[said.clone()]);
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_handed_back_already(&[step.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_they_say_they_tried(&[step.clone(), memory.clone()]))) } } } }) {
    _wording = op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    return _handed_back(&[memory.clone(), _a_lead_and_a_step(&[_a_lead_for(&[_leads_for(&[text.clone()]), _wording.clone(), memory.clone(), text.clone()]), _wording.clone()])]);
    }
    }
    if py_truthy(&{ let __b = mcall(&g_MOVE_ASK(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = put_right.clone(); if py_truthy(&__b) { __b } else { mcall(&g_AND_NOW(), "match", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } } } }) {
    step = _a_move(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&step.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g_AT_A_LOSS(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    step = _a_conditional_move(&[said.clone()]);
    }
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_already_repeated(&[step.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_they_say_they_tried(&[step.clone(), memory.clone()]))) } } } }) {
    said = op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&put_right.clone()) {
    leads = g_PUT_RIGHT_LEADS();
    } else {
    leads = if py_truthy(&mcall(&g_AT_A_LOSS(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { g_LOST_LEADS() } else { _leads_for(&[text.clone()]) };
    }
    return _handed_back(&[memory.clone(), _a_lead_and_a_step(&[_a_lead_for(&[leads.clone(), said.clone(), memory.clone(), text.clone()]), said.clone()])]);
    }
    if py_truthy(&step.clone()) {
    _wording = op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    return _handed_back(&[memory.clone(), _a_lead_and_a_step(&[_a_lead_for(&[_the_lead_for(&[text.clone(), step.clone()]), _wording.clone(), memory.clone(), text.clone()]), _wording.clone()])]);
    }
    }
    if py_truthy(&{ let __b = more.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_more_ask(&[text.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&nothing_more.clone()) {
    return Py::None;
    }
    short = _short(&[said.clone()]);
    if py_truthy(&{ let __b = short.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already_repeated(&[short.clone(), memory.clone()]))) } }) {
    leads = if py_truthy(&_a_bare_ask(&[text.clone()])) { g_SAY_AGAIN_LEADS() } else { _the_lead_for(&[text.clone(), short.clone()]) };
    return _handed_back(&[memory.clone(), _a_lead_and_a_step(&[_a_lead_for(&[leads.clone(), short.clone(), memory.clone(), text.clone()]), short.clone()])]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _an_entry_answers_it(args: &[Py]) -> Py {
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
    let mut entries: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the knowledge base has an idea of its own for this ask, rather than the answer again.\n\n    The entries are asked the way the turn asks them - their own `respond`, on a copy of the\n    memory so a probe claims nothing - and a reply that restates the answer in hand (`Same answer,\n    and it covers this: ...`, `Nothing separate for suggest - it is the same advice: ...`) is not\n    an idea, so it leaves the line to this reader. A reply with something new in it is the better\n    answer by a long way, and this reader steps aside for it.\n\n    The lookup is guarded against coming back into itself: `respond` asks this module whether the\n    line is a repeat of the answer, and without the guard that question would return here.\n    ");
    if py_truthy(&geti(&g__AN_ENTRY_IS_LOOKING(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    let _ = py_setitem(&g__AN_ENTRY_IS_LOOKING(), &Py::Int(0), Py::Bool(true));
    let __try1 = exc::catch(|| {
    entries = pyrt::xrefs::xmodule("modules.answers");
    reply = mcall(&entries.clone(), "respond", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("dict", &[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
    let _ = py_setitem(&g__AN_ENTRY_IS_LOOKING(), &Py::Int(0), Py::Bool(false));
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    said = _words(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() } } else { py_list(vec![]) };
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&_words(&[if py_truthy(&turns.clone()) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]) } else { py_str("") }]), &Py::None, &Py::Int(8), &Py::None)]);
    if py_truthy(&{ let __b = opening.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[said.clone()]), &opening.clone())) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a line, lowercased, in order: enough to see one line inside another.");
    return stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_bare_ask(args: &[Py]) -> Py {
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
    let _ = py_str("True when the whole line is one word asking to hear the last answer again.");
    match_ = mcall(&g_BARE_ASK(), "match", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BARE_ASK_NEEDS_A_QUESTION(), &mcall(&mcall(&match_.clone(), "group", &[py_str("word")]), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_lead_and_a_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lead: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lead") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lead"))) };
    let mut step: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut said: Py = Py::None;
    let _ = py_str("A lead with the step in it, and one full stop at the end.\n\n    Every lead ends the step with its own \".\" (\"One thing to do first: {step}.\"), and a step that\n    already ended a sentence carries one too: \"One thing to do first: Quick: tuna pasta in 15 ...\n    for 30 minutes..\" is the same sentence closed twice.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "rstrip", &[]);
    return stdlib_call("re", "sub", &[py_str("\\.{2,}$"), py_str("."), stdlib_call("re", "sub", &[py_str("\\.\\.\\s+"), py_str(". "), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), said.clone()), ("said".to_string(), said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "format", args))(&__args) }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_lead_for(args: &[Py]) -> Py {
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
    let mut step: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { py_str("") };
    let mut lead: Py = Py::None;
    let mut leads: Py = Py::None;
    let _ = py_str("Which set of leads this line gets.\n\n    \"That's the one:\" and \"You've got it:\" say the person agreed to something. A line that says\n    they do not know what to do has agreed to nothing, so it takes the leads written for it, the\n    ones that just start them off. Without this, \"i am thinking of leaving my husband\" then \"i\n    dont know what to do\" was handed \"That's the one: Before you decide anything...\", which\n    answers a question nobody asked.\n    ");
    leads = if py_truthy(&mcall(&g_AT_A_LOSS(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { g_LOST_LEADS() } else { g_LEADS() };
    if py_truthy(&mcall(&g_A_COUNT_OF_MORE_THAN_ONE(), "match", &[call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    leads = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&leads.clone()) { let mut lead: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&mcall(&lead.clone(), "lower", &[]), &py_str("one thing")))) {  __out.push(lead.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { leads.clone() } };
    }
    return leads.clone();
    Py::None
}
pub fn _content(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("The words of a line that name something, in the spelling the two sides can be compared in.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &w.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _trouble(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a complaint about the thing still in play, not a fact about it.\n\n    \"i cant get my kids to eat veg\" is followed by \"theyre 4 and 7\", and \"my dog ate half a bar of\n    dark chocolate\" by \"he's a labrador, about 30 kilos\": both are the person handing over what\n    the bot asked for, and \"Got it - they're 4 and 7\" is exactly right. The advice handed back\n    instead answers a complaint they did not make. So the line has to say something is still\n    going wrong (\"cant\", \"keeps\", \"still\", \"worse\") before the answer's advice comes back.\n    ");
    return call_builtin("bool", &[mcall(&g_TROUBLE(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_thing_of_its_own(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut named: Py = Py::None;
    let mut stems: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names a thing as its own that the answer does not mention.");
    named = call_builtin("set", &[mcall(&g_OWNED(), "findall", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    stems = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&words.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&stems.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn after_an_ack(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The rest of a line that opens with a thank-you, or \"\" when it opens with anything else.\n\n    \"cheers. also im skint till friday and theres nothing in the fridge\" is a thank-you and then\n    a line about money; \"thanks, that helps\" is the thank-you and nothing else. The callers use\n    this to read what comes after the courtesy: the knowledge base answers it, and a line that\n    only closes the turn has nothing there to answer.\n    ");
    match_ = mcall(&g_ACK_AND_REST(), "match", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    rest = mcall(&mcall(&match_.clone(), "group", &[py_str("rest")]), "strip", &[py_str(" ,.!?;:-—")]);
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:mate|love|pet|hun|hunny|duck|sunshine|chief|fella|sweetheart)\\b[,!.]*\\s*")); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&stdlib_call("re", "match", &[py_str("for\\b"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[rest.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_CLOSES_THE_TURN(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&op_ge(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { rest.clone() } else { py_str("") };
    Py::None
}
pub fn _a_feeling(args: &[Py]) -> Py {
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
    let mut NEGATIVE_CUES: Py = Py::None;
    let _ = py_str("True when the line is about how they are, not about the thing the answer was about.\n\n    \"i lost my job last week\" is answered with what to check in the offer, and the next line,\n    \"i cant stop crying\", is not a question about the offer: handing back \"ask for it in writing\n    and don't sign it on the day\" reads as if nobody had heard the crying. The feeling words are\n    the ones the fallback already reads a bad week by, so the two cannot drift apart. A line about\n    the trouble following them about (BROODING) is how they are too, even with no feeling word in\n    it: \"i keep going over it in my head at 2am\" is the part of the evening the answer never\n    spoke to.\n    ");
    NEGATIVE_CUES = pyrt::xrefs::xget("compose.fallback", "NEGATIVE_CUES");
    if py_truthy(&mcall(&g_BROODING(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[op_bitand(&call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)), &NEGATIVE_CUES.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_doing(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut last: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut sep: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The doing part of a sentence: what follows a short label, and cut to a length that is a\n    reply rather than a paragraph.\n\n    \"The fix is what you do in those minutes: do not check the clock, keep the light off, and if\n    you are properly awake after about twenty minutes get up and sit somewhere dim ...\" is advice\n    with a label in front of it; on its own the label says nothing about the sleeping, so the part\n    after the colon is what the person gets, with its first letter raised because it now opens\n    the reply.\n    ");
    let __unpk1 = py_unpack(&mcall(&mcall(&call_builtin("str", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "partition", &[py_str(":")]), 3, None);
    head = __unpk1[0].clone();
    sep = __unpk1[1].clone();
    rest = __unpk1[2].clone();
    if py_truthy(&{ let __b = sep.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_opens_a_doing(&[head.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[mcall(&rest.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } } } } } }) {
    whole = mcall(&rest.clone(), "strip", &[]);
    whole = op_add(&mcall(&geti(&whole.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&whole.clone(), &Py::Int(1), &Py::None, &Py::None));
    } else {
    whole = mcall(&call_builtin("str", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    }
    whole = pyrt::xrefs::xcall("compose.whole", "a_whole_cut", &[whole.clone(), g_MOST()]);
    last = mcall(&whole.clone(), "rstrip", &[py_str("'\"’”")]);
    return if py_truthy(&{ let __b = whole.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&last.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])]))) } }) { op_add(&whole.clone(), &py_str(".")) } else { whole.clone() };
    Py::None
}
pub fn _the_advice(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut best: Py = Py::None;
    let mut count: Py = Py::None;
    let mut most: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The sentence of an answer that tells them what to do, or None when it tells them nothing.\n\n    \"The fix is what you do in those minutes: do not check the clock, keep the light off, and if\n    you are properly awake after about twenty minutes get up ...\" is the most of an instruction in\n    that answer, and it is the part a person who says \"i cant get back off\" can use. A sentence of\n    explanation (\"Waking in the night is normal\") has nothing to do in it, so it is never picked.\n    ");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    let __unpk1 = py_unpack(&py_tuple(vec![py_str(""), Py::Int(1)]), 2, None);
    best = __unpk1[0].clone();
    most = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() });
    for __item in __iter2 {
    part = __item.clone();
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_THEM(), "search", &[part.clone()]) } }) {
    continue;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&part.clone(), "lower", &[]), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(" ,;:.")])); } py_list(__out) })() };
    count = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&IMPERATIVES.clone(), &w.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    let __aug3 = op_add(&count.clone(), &if py_truthy(&stdlib_call("re", "search", &[py_str("\\bdo not\\b|\\bdon'?t\\b"), part.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { Py::Int(1) } else { Py::Int(0) });
    count = __aug3;
    if py_truthy(&op_gt(&count.clone(), &most.clone())) {
    let __unpk4 = py_unpack(&py_tuple(vec![part.clone(), count.clone()]), 2, None);
    best = __unpk4[0].clone();
    most = __unpk4[1].clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn the_next_move(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The step after advice the person says did not work, in the answer's own words, or \"\".\n\n    A report on the advice is answered by the composer, and its pools say \"change one thing\n    instead\" - advice to change, which the person in front of it has already done. When the answer\n    itself ends on who to go to if the advice is not enough (\"ask a GP\", \"call a Gas Safe\n    engineer\"), that sentence is the next move and it is the one thing left to say. Nothing is\n    invented here: it is the answer's own closing sentence, said once (a step already repeated\n    gets \"\" and the composer's own line stands).\n    ");
    let __iter1 = py_iter_or_raise(&_recent_answers(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    asked = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_same_thing(&[asked.clone(), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_NEXT_STEP_EMERGENCY(), "search", &[said.clone()])) {
    return py_str("");
    }
    step = _next_step(&[said.clone()]);
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already_repeated(&[step.clone(), memory.clone()]))) } }) {
    step = op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_TRIED_LEADS(), memory.clone(), { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "format", args))(&__args) };
    }
    return py_str("");
    }
    return py_str("");
    Py::None
}
pub fn _next_step(args: &[Py]) -> Py {
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
    let mut s: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let _ = py_str("The sentence an answer ends on about who to go to, when the advice alone is not enough.");
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut s: Py = __item.clone(); if py_truthy(&mcall(&s.clone(), "strip", &[])) {  __out.push(mcall(&s.clone(), "strip", &[])); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&sentences.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&mcall(&g_NEXT_STEP(), "search", &[sentence.clone()])) {
    return sentence.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _later_part(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tried: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tried") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tried"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut best: Py = Py::None;
    let mut best_i: Py = Py::None;
    let mut count: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut i: Py = Py::None;
    let mut later: Py = Py::None;
    let mut most: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut tried_words: Py = Py::None;
    let _ = py_str("The part of one answer that comes after the step the line says it has tried.");
pub fn __nested_shared_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut best: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut best_i: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut count: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut doing: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut i: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut later: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut most: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut p: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut part: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut parts: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut shared: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut tried: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut tried_words: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut part: Py = __pos.get(14).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "part").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut words: Py = __pos.get(15).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "words").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut v: Py = Py::None;
    let mut w: Py = Py::None;
    return call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_content(&[part.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut v: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&w.clone(), &v.clone())); if py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[v.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&w.clone(), &Py::None, &Py::Int(5), &Py::None), &slice_or_raise(&v.clone(), &Py::None, &Py::Int(5), &Py::None))) } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    shared = {{ let best = best.clone(); let best_i = best_i.clone(); let count = count.clone(); let doing = doing.clone(); let i = i.clone(); let later = later.clone(); let most = most.clone(); let p = p.clone(); let part = part.clone(); let parts = parts.clone(); let said = said.clone(); let shared = shared.clone(); let tried = tried.clone(); let tried_words = tried_words.clone(); py_func("shared", move |args: &[Py]| { let mut full = vec![best.clone(), best_i.clone(), count.clone(), doing.clone(), i.clone(), later.clone(), most.clone(), p.clone(), part.clone(), parts.clone(), said.clone(), shared.clone(), tried.clone(), tried_words.clone()]; full.extend_from_slice(args); __nested_shared_0(&full) }) }};
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    tried_words = _content(&[tried.clone()]);
    let __unpk2 = py_unpack(&py_tuple(vec![op_neg(&Py::Int(1)), Py::Int(0)]), 2, None);
    best_i = __unpk2[0].clone();
    most = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    i = __unpk4[0].clone();
    part = __unpk4[1].clone();
    count = py_call(&shared, &[part.clone(), tried_words.clone()]);
    if py_truthy(&op_gt(&count.clone(), &most.clone())) {
    let __unpk5 = py_unpack(&py_tuple(vec![i.clone(), count.clone()]), 2, None);
    best_i = __unpk5[0].clone();
    most = __unpk5[1].clone();
    }
    }
    if py_truthy(&op_ge(&best_i.clone(), &Py::Int(0))) {
    later = py_list(vec![]);
    let __iter6 = py_iter_or_raise(&slice_or_raise(&parts.clone(), &op_add(&best_i.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter6 {
    part = __item.clone();
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_THEM(), "search", &[part.clone()]) } }) {
    continue;
    }
    if py_truthy(&op_ge(&py_call(&shared, &[part.clone(), tried_words.clone()]), &most.clone())) {
    continue;
    }
    doing = _the_doing(&[part.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[doing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25))) {
    let _ = mcall(&later.clone(), "append", &[doing.clone()]);
    }
    }
    let __iter7 = py_iter_or_raise(&later.clone());
    for __item in __iter7 {
    doing = __item.clone();
    if py_truthy(&_opens_a_doing(&[doing.clone()])) {
    return doing.clone();
    }
    }
    return if py_truthy(&later.clone()) { geti(&later.clone(), &Py::Int(0)) } else { py_str("") };
    }
    best = py_str("");
    let __iter8 = py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter8 {
    part = __item.clone();
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_THEM(), "search", &[part.clone()]) } }) {
    continue;
    }
    doing = _the_doing(&[part.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[doing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25))) {
    best = doing.clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _opens_a_doing(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut first: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a sentence opens on a doing rather than on its subject.\n\n    \"Leaving the door and drawer open between washes is what stops the smell coming back\" opens\n    on the habit to keep, and \"The wash itself is about two hours\" opens on the wash: the first\n    is something to do, the second is how long the thing they just did takes. The test is the\n    first word - an instruction's verb, or the \"-ing\" form of one, which is how an answer says a\n    step that is a habit rather than a one-off.\n    ");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    words = mcall(&mcall(&call_builtin("str", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,:;")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    first = mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(".,;:")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&IMPERATIVES.clone(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MOVE_VERBS(), &first.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&first.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    stem = slice_or_raise(&first.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None);
    bare = py_list(vec![stem.clone(), op_add(&stem.clone(), &py_str("e"))]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&stem.clone(), &op_neg(&Py::Int(1))), &geti(&stem.clone(), &op_neg(&Py::Int(2))))) } }) {
    let _ = mcall(&bare.clone(), "append", &[slice_or_raise(&stem.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bare.clone()) { let mut candidate: Py = __item.clone(); __out.push(Py::Bool(py_contains(&IMPERATIVES.clone(), &candidate.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_untried(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tried: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tried") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tried"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _question: Py = Py::None;
    let mut back: Py = Py::None;
    let mut best: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The later part of the last answer that is not the step they have just tried.\n\n    \"my sourdough is flat\" / \"i tried the starter and it still flopped\" used to stand down to a\n    filler, but the answer's last sentence is the next diagnosis, not the step they tried. The\n    same is true of \"ive tried the filter and the drawer\": the answer's later sentence - leave the\n    door open - is what is left to do. Only a later sentence qualifies: a one-sentence answer has\n    no next step to hand back.\n    ");
    let __iter1 = py_iter_or_raise(&_recent_answers(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    _question = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&Py::Bool(!py_eq(&back.clone(), &Py::Int(1)))) {
    return Py::None;
    }
    best = _later_part(&[tried.clone(), said.clone()]);
    if py_truthy(&{ let __b = best.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already_repeated(&[best.clone(), memory.clone()]))) } }) {
    return _handed_back(&[memory.clone(), best.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _tried(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says the advice has already been carried out, not that it failed.");
    return { let __b = call_builtin("bool", &[mcall(&g_TRIED(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_TRIED(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]))) } };
    Py::None
}
pub fn _says_they_agree(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line carries a word that agrees: \"ok so low heat then\", \"low heat, got it\".\n\n    A line that happens to sit inside one sentence of the answer word for word is not always an\n    agreement: \"the washer is old\" after the dripping tap answer is the person telling the bot\n    something, and saying it back (\"Yes - the washer is old.\") is the bot parroting them. What\n    makes a line an agreement is the word in it that agrees - ok, so, then, got it, right - so\n    that is what is asked for.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&clean.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!;:\"'")]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_AGREES_ON_ITS_OWN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_AGREES_IN_A_PHRASE(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AGREEMENT_TAIL(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } } } };
    Py::None
}
pub fn _the_thing_they_named(args: &[Py]) -> Py {
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
    let mut part: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut bare: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of the line that are the answer's own, in the person's order: \"low heat\".\n\n    \"ok so low heat then\" is the answer's sentence with an agreement round it, and the thing the\n    person said is the part of it they named. The agreement words come out, so the reply names the\n    thing rather than repeating the agreement back at them. Every other word stays as they said\n    it, including one with no stem of its own (\"same time every day\"), because the line has\n    already been matched against the answer and those are the words they chose.\n    ");
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&clean.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    bare = mcall(&mcall(&word.clone(), "strip", &[py_str(".,!;:\"'")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AGREEING(), &bare.clone())) } }) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[mcall(&word.clone(), "strip", &[py_str(".,!;:\"'")])]);
    }
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&g_BE_FORM(), &g_POINTING_AT()), &mcall(&geti(&kept.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    kept = slice_or_raise(&kept.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&g_BE_FORM(), &g_POINTING_AT()), &mcall(&geti(&kept.clone(), &Py::Int(0)), "lower", &[]))) } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_OPENERS(), &mcall(&geti(&kept.clone(), &Py::Int(0)), "lower", &[])))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[kept.clone()]);
    Py::None
}
pub fn agreement_with_the_answer(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut best: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut count: Py = Py::None;
    let mut named_thing: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut question: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shared: Py = Py::None;
    let _ = py_str("The confirmation of a thing the person has just said back, or \"\" for every other line.\n\n    \"ok so low heat then\" is answered by `carried_on`, but a second agreement - \"low heat, got\n    it\", a turn later - is answered before that: the answer it agrees with is now two turns back,\n    the lookup reads the named thing as the thing to answer about, and the whole paragraph comes\n    out again, which is the answer the person has just agreed with. The answer modules call this\n    first, the way they call `asks_for_the_same_work`, so the confirmation is what they get.\n    ");
    clean = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&clean.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUESTION(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASKS(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } } } } } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_ABOUT_THE_BOT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORDS_WANTED(), "search", &[clean.clone()]) } }) {
    return py_str("");
    }
    asked = _content(&[clean.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&_recent_answers(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    question = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_same_thing(&[question.clone(), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![py_str(""), Py::Int(0)]), 2, None);
    best = __unpk3[0].clone();
    shared = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[said.clone()])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() });
    for __item in __iter4 {
    part = __item.clone();
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_THEM(), "search", &[part.clone()]) } }) {
    continue;
    }
    count = call_builtin("len", &[op_bitand(&asked.clone(), &_content(&[part.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&count.clone(), &shared.clone())) {
    let __unpk5 = py_unpack(&py_tuple(vec![part.clone(), count.clone()]), 2, None);
    best = __unpk5[0].clone();
    shared = __unpk5[1].clone();
    }
    }
    if py_truthy(&{ let __b = op_lt(&shared.clone(), &call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_they_agree(&[clean.clone()]))) } }) {
    continue;
    }
    named_thing = _the_thing_they_named(&[clean.clone(), best.clone()]);
    if py_truthy(&named_thing.clone()) {
    return _agreed(&[named_thing.clone(), memory.clone()]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _agreed(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut low: Py = Py::None;
    let mut up: Py = Py::None;
    let _ = py_str("The confirmation of a thing the person said back: \"Low heat, yes.\"\n\n    Every line names the thing, so the pool cannot be read out to anybody else, and the answer\n    just given is never one of them.\n    ");
    low = op_add(&mcall(&slice_or_raise(&named.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&named.clone(), &Py::Int(1), &Py::None, &Py::None));
    up = op_add(&mcall(&slice_or_raise(&named.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&named.clone(), &Py::Int(1), &Py::None, &Py::None));
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&up.clone(), &py_str(", yes.")), op_add(&op_add(&py_str("Yes — "), &low.clone()), &py_str(".")), op_add(&up.clone(), &py_str(" it is.")), op_add(&op_add(&py_str("That's it: "), &low.clone()), &py_str(".")), op_add(&up.clone(), &py_str(", that's the main thing."))]), memory.clone(), named.clone()]);
    Py::None
}
pub fn _the_stems(args: &[Py]) -> Py {
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
    let mut base: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("The base verbs an -ed form could have come from (\"topped\" -> top, \"dried\" -> dry).");
    word = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ied")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let _ = mcall(&out.clone(), "add", &[op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"))]);
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ed")])) {
    base = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    let _ = mcall(&out.clone(), "add", &[base.clone()]);
    let _ = mcall(&out.clone(), "add", &[op_add(&base.clone(), &py_str("e"))]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[base.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&base.clone(), &op_neg(&Py::Int(1))), &geti(&base.clone(), &op_neg(&Py::Int(2))))) } }) {
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&base.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    } else {
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("d")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _the_act(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut base: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The step a word names in the past tense, if it is one an instruction can open on.");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    base = mcall(&g_IRREGULAR_PAST(), "get", &[word.clone()]);
    if py_truthy(&{ let __b = base.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&base.clone(), &py_str("do"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&IMPERATIVES.clone(), &base.clone())) } } } }) {
    return base.clone();
    }
    let __iter1 = py_iter_or_raise(&_the_stems(&[word.clone()]));
    for __item in __iter1 {
    stem = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&IMPERATIVES.clone(), &stem.clone()))) {
    return stem.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_object(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut bare: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing a past tense was done to, as words to find in the answer; None if it says none.\n\n    \"i topped up my phone\" is the phone and not the boiler, \"i bled my knee\" is the knee and not\n    the radiators, so what follows the verb has to be read or the act alone hands back the next\n    step of a job the person never did. A pronoun is not a new thing (\"i topped it up\"), so it\n    says nothing and the act is left to decide.\n    ");
    tail = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&mcall(&mcall(&slice_or_raise(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None, &Py::None), "lower", &[]), "replace", &[py_str(","), py_str(" ")]), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    bare = mcall(&word.clone(), "strip", &[py_str(" .;:!?()[]\"")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OBJECT_END(), &bare.clone())) } }) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g_STANDS_IN(), &bare.clone()))) {
    return Py::None;
    }
    let _ = mcall(&tail.clone(), "append", &[bare.clone()]);
    }
    said = call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = said.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _the_steps_done(args: &[Py]) -> Py {
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
    let mut act: Py = Py::None;
    let mut acts: Py = Py::None;
    let mut before: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The steps a line reports having done: the act, and the thing it was done to if it names one.");
    acts = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    clean = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_DONE(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    before = slice_or_raise(&mcall(&mcall(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "lower", &[]), "split", &[]), &op_neg(&Py::Int(3)), &Py::None, &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NO_LONGER_TO_DO(), &mcall(&w.clone(), "strip", &[py_str(" ,.;:!?'")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    act = _the_act(&[mcall(&mcall(&match_.clone(), "group", &[py_str("verb")]), "lower", &[])]);
    if py_truthy(&act.clone()) {
    let _ = mcall(&acts.clone(), "add", &[py_tuple(vec![act.clone(), _the_object(&[clean.clone(), mcall(&match_.clone(), "end", &[])])])]);
    }
    }
    return acts.clone();
    Py::None
}
pub fn _the_answer_gives(args: &[Py]) -> Py {
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
    let mut act: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "act") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: act"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether the answer just given is the one that named the step.");
    let __iter1 = py_iter_or_raise(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")]), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str(" ,.;:!?()[]\"")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &act.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_the_stems(&[word.clone()]), &act.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_step_done(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut act: Py = Py::None;
    let mut acts: Py = Py::None;
    let mut by_object: Py = Py::None;
    let mut given: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("True when the line reports doing a step of the answer just given.\n\n    The step is the answer's, not the word list's: \"i changed the washer already\" is a step when\n    the answer said \"take the old part to a hardware shop\" and not when the chat is about\n    something else. A step named by its object rather than its verb (\"i did the hot wash\") counts\n    too, and only for the empty verb - \"do\" is not a step any answer says, so the word that has to\n    be in the answer is the thing done.\n    ");
    acts = _the_steps_done(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&acts.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::Bool(false);
    }
    given = _content(&[said.clone()]);
    let __iter1 = py_iter_or_raise(&acts.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    act = __unpk2[0].clone();
    thing = __unpk2[1].clone();
    by_object = Py::Bool(py_eq(&act.clone(), &py_str("do")));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&by_object.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_answer_gives(&[said.clone(), act.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_is(&thing.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&by_object.clone()))) {
    return Py::Bool(true);
    }
    continue;
    }
    if py_truthy(&op_bitand(&thing.clone(), &given.clone())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_step_just_given(args: &[Py]) -> Py {
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
    let mut _question: Py = Py::None;
    let mut back: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line reports doing a step of the answer one turn back.");
    let __iter1 = py_iter_or_raise(&_recent_answers(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    _question = __unpk2[1].clone();
    said = __unpk2[2].clone();
    return { let __b = Py::Bool(py_eq(&back.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { _a_step_done(&[text.clone(), said.clone()]) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn carried_on(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut asked_after: Py = Py::None;
    let mut back: Py = Py::None;
    let mut best: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut count: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut named: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut question: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut which_part: Py = Py::None;
    let _ = py_str("The reply to a line that picks up a detail of the answer just given, or None.\n\n    \"ive been waking up at 4 every night\" is answered with the advice - the clock, the light, get\n    up after twenty minutes. The next line, \"and i honestly cant get back off\", names the detail\n    they mean and says nothing the answer does not already cover, so the honest reply is the part\n    of that answer that speaks to it. What it got was \"Thanks for telling me - I'm here if you\n    want to dig into any of that\", which would fit any message anybody ever sent.\n\n    The rule is narrow on purpose:\n    * the line has to name at least two things of its own, and they have to be in one sentence of\n      the answer, so a line about something else never comes here;\n    * it is a statement, not a question - a question has the modules of its own;\n    * only the answer just given counts, or, a turn older, one the chat is still on;\n    * a part already handed back once is not handed back again.\n    ");
    clean = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    asked_after = without_a_reset(&[clean.clone()]);
    if py_truthy(&{ let __b = mcall(&g_QUESTION(), "match", &[asked_after.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKS(), "match", &[asked_after.clone()]) } }) {
    clean = asked_after.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&clean.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUESTION(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASKS(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_ABOUT_THE_BOT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORDS_WANTED(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ASKING_TO_START(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ACK_OPENS(), "match", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.limits", "names_a_limit", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_QUESTION_MORE(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _tried(&[clean.clone()]); if py_truthy(&__b) { __b } else { _the_step_just_given(&[clean.clone(), memory.clone()]) } } } }) {
    return _the_untried(&[clean.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("routine")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_REJOINDER(), "search", &[clean.clone()]))) } } } }) {
    return Py::None;
    }
    asked = _content(&[clean.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    which_part = asks_for_the_move(&[text.clone()]);
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&which_part.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_REFER_BACK(), "search", &[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&_recent_answers(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    question = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_same_thing(&[question.clone(), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![py_str(""), Py::Int(0)]), 2, None);
    best = __unpk3[0].clone();
    shared = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[said.clone()])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() });
    for __item in __iter4 {
    part = __item.clone();
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_THEM(), "search", &[part.clone()]) } }) {
    continue;
    }
    count = call_builtin("len", &[op_bitand(&asked.clone(), &_content(&[part.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&count.clone(), &shared.clone())) {
    let __unpk5 = py_unpack(&py_tuple(vec![part.clone(), count.clone()]), 2, None);
    best = __unpk5[0].clone();
    shared = __unpk5[1].clone();
    }
    }
    if py_truthy(&op_lt(&shared.clone(), &Py::Int(2))) {
    best = if py_truthy(&{ let __b = Py::Bool(py_eq(&back.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = _trouble(&[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_feeling(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_thing_of_its_own(&[clean.clone(), said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SOMEONE_ELSES_LIMIT(), "search", &[clean.clone()]))) } } } } } } } }) { _the_advice(&[said.clone()]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    continue;
    }
    } else {
    if py_truthy(&{ let __b = op_ge(&shared.clone(), &call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _says_they_agree(&[clean.clone()]) } }) {
    named = _the_thing_they_named(&[clean.clone(), best.clone()]);
    if py_truthy(&named.clone()) {
    return _agreed(&[named.clone(), memory.clone()]);
    }
    }
    }
    doing = _the_doing(&[best.clone()]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[doing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25)); if py_truthy(&__b) { __b } else { _already_repeated(&[doing.clone(), memory.clone()]) } }) {
    continue;
    }
    return _handed_back(&[memory.clone(), doing.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _starting_out_names_a_thing(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("True when what is started out *into* is in the line: \"start out in gardening\".");
    found = mcall(&g_STARTING_OUT(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), { let __b = mcall(&found.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } }]), 2, None);
    first = __unpk1[0].clone();
    second = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_STARTING_INTO(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STARTING_AT(), &first.clone())) } }) {
    return { let __b = call_builtin("bool", &[second.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_STARTING_BACK(), &second.clone())) } };
    }
    return Py::Bool(!py_contains(&g_STARTING_BACK(), &first.clone()));
    Py::None
}
pub fn says_where_to_start(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("START_ASK for a line that asks which part of the answer to do first, and names no starting.\n\n    \"Where do i start\", \"how do i begin with it\" and \"what should i do first\" lean on the answer\n    in hand. \"How do i start running\", \"how do i start saving\", \"how do i start learning guitar\"\n    and \"how do i start out in gardening\" name a thing of their own and are answered by the entry\n    for it.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_START_ASK(), "search", &[line.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&_starting_out_names_a_thing(&[line.clone()])) {
    return Py::Bool(false);
    }
    found = mcall(&g_STARTING_WHAT(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return op_lt(&call_builtin("len", &[_names_a_thing_of_its_own(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), { let __b = mcall(&found.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } }]), 2, None);
    first = __unpk1[0].clone();
    second = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_STARTING_AT(), &first.clone()))) {
    return { let __b = Py::Bool(!py_truthy(&second.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STARTING_BACK(), &second.clone())) } };
    }
    return Py::Bool(py_contains(&g_STARTING_BACK(), &first.clone()));
    Py::None
}
pub fn a_more_ask(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks for the next part of the answer in hand, said either way.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g_MORE_ASK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_IDEAS_ONLY(), "match", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _places_to_cut(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut char: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut spots: Py = Py::None;
    let _ = py_str("Where a long instruction could end: at a qualifier, or at a bracket that explains it.\n\n    A qualifier that is the last word leaves nothing to cut, and one that is followed by a\n    preposition (\"put a bowl under it and turn the water off at the stopcock\") is the second half\n    of the phrase rather than its end, so both are passed over.\n    ");
    spots = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_QUALIFIER(), "finditer", &[head.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    after = mcall(&mcall(&slice_or_raise(&head.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_OPENERS(), &mcall(&geti(&after.clone(), &Py::Int(0)), "lower", &[])))) {
    continue;
    }
    let _ = mcall(&spots.clone(), "append", &[mcall(&found.clone(), "start", &[])]);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    char = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&char.clone(), &py_str("("))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&head.clone(), &index.clone(), &Py::None, &Py::None), &py_str(")"))) } }) {
    let _ = mcall(&spots.clone(), "append", &[index.clone()]);
    }
    }
    return call_builtin("sorted", &[call_builtin("set", &[spots.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _inside_a_bracket(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let _ = py_str("Whether the character at `at` sits inside a bracket that has not closed by then.");
    return op_gt(&mcall(&text.clone(), "count", &[py_str("("), Py::Int(0), at.clone()]), &mcall(&text.clone(), "count", &[py_str(")"), Py::Int(0), at.clone()]));
    Py::None
}
pub fn _a_list_item(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut first: Py = Py::None;
    let mut items: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut p: Py = Py::None;
    let mut verbs: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first item of a list the answer gives after a colon, when no item opens on a verb.\n\n    \"The things that work: same wake-up time every day, no caffeine after early afternoon, a cool\n    dark room\" is three things to do and not one of them opens on a verb, so a reader that wants\n    the verb first found nothing in it and went on to the next sentence - \"ask a GP\", which is the\n    last resort at the end of the answer, not the first thing to do.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&part.clone(), &py_str(":")))) {
    return py_str("");
    }
    lead = geti(&mcall(&part.clone(), "split", &[py_str(":")]), &Py::Int(0));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_LIST_LEAD(), "search", &[lead.clone()])))) {
    return py_str("");
    }
    items = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_pieces(&[part.clone()])) { let mut p: Py = __item.clone(); __out.push(mcall(&p.clone(), "strip", &[py_str(" .:;")])); } py_list(__out) })() }, &Py::Int(1), &Py::None, &Py::None);
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut p: Py = __item.clone(); if py_truthy(&p.clone()) {  __out.push(p.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    first = geti(&items.clone(), &Py::Int(0));
    words = mcall(&first.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(3), &__cmp11)) { op_le(&__cmp11, &Py::Int(8)) } else { Py::Bool(false) } }))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DOING(), "search", &[first.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[first.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_STEP(), "match", &[first.clone()]) } } } }) {
    return py_str("");
    }
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    verbs = op_bitor(&call_builtin("set", &[IMPERATIVES.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g_MOVE_VERBS()]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&verbs.clone(), &mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:'\"")]), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    return first.clone();
    Py::None
}
pub fn _pieces(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut end: Py = Py::None;
    let mut f: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut keeps: Py = Py::None;
    let mut listed: Py = Py::None;
    let mut marks: Py = Py::None;
    let mut out: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The instructions of one sentence, split at its colons and commas.\n\n    Nothing is split inside a bracket: \"turn the boiler off at the wall switch (not at the fuse\n    board, you want the pump to run on for a minute first)\" is one instruction with its reason in\n    brackets, and splitting at that comma handed over half of it, bracket still open.\n    ");
    marks = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PIECE_END(), "finditer", &[part.clone()])) { let mut f: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_inside_a_bracket(&[part.clone(), mcall(&f.clone(), "start", &[])])))) {  __out.push(f.clone()); } } py_list(__out) })() };
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0), Py::Bool(false)]), 3, None);
    out = __unpk1[0].clone();
    start = __unpk1[1].clone();
    listed = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[marks.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    found = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[]), "strip", &[]), &py_str(","))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&listed.clone())) } }) {
    end = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[marks.clone()]).unwrap_or_else(|e| exc::raise(e)))) { mcall(&geti(&marks.clone(), &op_add(&index.clone(), &Py::Int(1))), "start", &[]) } else { call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    after = mcall(&slice_or_raise(&part.clone(), &mcall(&found.clone(), "end", &[]), &end.clone(), &Py::None), "strip", &[py_str(" ,;:.")]);
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|then|but|also|so|next|plus)\\s+")); __args.push(py_str("")); __args.push(after.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&_opens_on_a_verb(&[after.clone()])))) {
    continue;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[]), "strip", &[]), &py_str(":"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&listed.clone())) } }) {
    let __unpk4 = py_unpack(&_the_colon_tail_kept(&[part.clone(), marks.clone(), index.clone(), start.clone(), found.clone()]), 2, None);
    keeps = __unpk4[0].clone();
    at = __unpk4[1].clone();
    if py_truthy(&Py::Bool(py_is(&keeps.clone(), &Py::None))) {
    let __unpk5 = py_unpack(&py_tuple(vec![call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Bool(true)]), 2, None);
    start = __unpk5[0].clone();
    listed = __unpk5[1].clone();
    continue;
    }
    if py_truthy(&keeps.clone()) {
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[keeps.clone()]);
    start = at.clone();
    listed = Py::Bool(true);
    continue;
    }
    }
    let _ = mcall(&out.clone(), "append", &[slice_or_raise(&part.clone(), &start.clone(), &mcall(&found.clone(), "start", &[]), &Py::None)]);
    start = mcall(&found.clone(), "end", &[]);
    listed = { let __b = listed.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[]), "strip", &[]), &py_str(":"))) } };
    }
    let _ = mcall(&out.clone(), "append", &[slice_or_raise(&part.clone(), &start.clone(), &Py::None, &Py::None)]);
    return out.clone();
    Py::None
}
pub fn _the_tail_asks_a_question(args: &[Py]) -> Py {
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
    let mut marks: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "marks") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: marks"))) };
    let mut index: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut found: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut end: Py = Py::None;
    let _ = py_str("True when what follows the colon is the questions to put, not a list of things to try.");
    end = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[marks.clone()]).unwrap_or_else(|e| exc::raise(e)))) { mcall(&geti(&marks.clone(), &op_add(&index.clone(), &Py::Int(1))), "start", &[]) } else { call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    return call_builtin("bool", &[mcall(&g_A_QUESTION(), "match", &[mcall(&slice_or_raise(&part.clone(), &mcall(&found.clone(), "end", &[]), &end.clone(), &Py::None), "strip", &[py_str(" ,;.")])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_colon_tail_kept(args: &[Py]) -> Py {
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
    let mut marks: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "marks") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: marks"))) };
    let mut index: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut start: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut found: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut first: Py = Py::None;
    let mut head: Py = Py::None;
    let mut later: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The instruction with its colon tail, when the tail belongs to it rather than to a list.\n\n    \"Open a lifetime ISA and put a monthly standing order into it: that adds 25%\", \"start with\n    run-walk: one minute of easy jogging, two minutes of walking\", \"pick one wake-up time for two\n    weeks, weekends included: the morning light is what sets the clock\" - the words before the\n    colon are the whole of the instruction and the colon gives its method or its reason, so\n    cutting there handed over a headline with the part that makes it work gone (\"Split your time\n    in three.\").\n\n    A colon after a noun phrase still opens a list, and a list is read item by item: \"the things\n    that work: same wake-up time every day, no caffeine\" has no verb in front of its colon, so\n    this leaves it alone. Returns (the piece to hand over, where the next piece starts) - the\n    piece is \"\" when the colon splits as before, None when the head only announces a list of\n    things, and the offset is None when the whole of the sentence after it belongs to this\n    instruction.\n    ");
    head = mcall(&slice_or_raise(&part.clone(), &start.clone(), &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;")]);
    if py_truthy(&Py::Bool(!py_truthy(&_opens_on_a_verb(&[head.clone()])))) {
    return py_tuple(vec![py_str(""), Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_TAIL(), "search", &[head.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_tail_asks_a_question(&[part.clone(), marks.clone(), index.clone(), found.clone()]))) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    tail = slice_or_raise(&part.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&op_add(&op_add(&head.clone(), &py_str(": ")), &tail.clone()), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WITH_ITS_METHOD())) {
    return py_tuple(vec![op_add(&op_add(&head.clone(), &py_str(": ")), &mcall(&tail.clone(), "rstrip", &[py_str(" ,;")])), Py::None]);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&marks.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter1 {
    later = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str(","), py_str(";")]), &mcall(&mcall(&later.clone(), "group", &[]), "strip", &[])))) {
    continue;
    }
    first = mcall(&slice_or_raise(&part.clone(), &mcall(&found.clone(), "end", &[]), &mcall(&later.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;")]);
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    continue;
    }
    piece = op_add(&op_add(&head.clone(), &py_str(": ")), &first.clone());
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&piece.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WITH_ITS_METHOD())) {
    return py_tuple(vec![piece.clone(), mcall(&later.clone(), "end", &[])]);
    }
    return py_tuple(vec![py_str(""), Py::None]);
    }
    return py_tuple(vec![py_str(""), Py::None]);
    Py::None
}
pub fn _opens_on_a_verb(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether this clause starts on a verb, which is what makes it an instruction of its own.");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    words = mcall(&mcall(&text.clone(), "strip", &[py_str(" .,:;")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&IMPERATIVES.clone(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MOVE_VERBS(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } };
    Py::None
}
pub fn _the_steps(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut at: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut cut_words: Py = Py::None;
    let mut first_half: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut item: Py = Py::None;
    let mut left: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut p: Py = Py::None;
    let mut part: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut step: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let mut verbs: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every instruction in the answer, in the order the answer gives them.\n\n    `_the_first_step` wants the first one; a \"what else can i do\" wants the first one that is not\n    the thing the person has just said did not work, so both read the answer through this.\n    ");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    verbs = op_bitor(&call_builtin("frozenset", &[IMPERATIVES.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[g_MOVE_VERBS()]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() });
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[part.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g_CONDITION_CLAUSE(), "match", &[part.clone()])) {
    continue;
    }
    found = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&_pieces(&[part.clone()]));
    for __item in __iter2 {
    piece = __item.clone();
    head = mcall(&piece.clone(), "strip", &[py_str(" .:;")]);
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|then|but|also|so|next|plus)\\s+")); __args.push(py_str("")); __args.push(head.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[head.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_STEP(), "match", &[head.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_MENU(), "search", &[head.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g_DECLARATIVE(), "match", &[head.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&head.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_QUESTIONS_THAT_ARE_NOT_STEPS(), "match", &[head.clone()]) } }) {
    continue;
    }
    words = mcall(&head.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_contains(&head.clone(), &py_str(":")))) {
    let __unpk3 = py_unpack(&py_tuple(vec![g_STEP_WITH_ITS_METHOD(), g_STEP_WITH_ITS_METHOD()]), 2, None);
    limit = __unpk3[0].clone();
    cut_words = __unpk3[1].clone();
    } else {
    let __unpk4 = py_unpack(&py_tuple(vec![g_STEP_WORDS(), g_CUT_WORDS()]), 2, None);
    limit = __unpk4[0].clone();
    cut_words = __unpk4[1].clone();
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &cut_words.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NEEDS_COMPLEMENT(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    cut = Py::None;
    let __iter5 = py_iter_or_raise(&_places_to_cut(&[head.clone()]));
    for __item in __iter5 {
    at = __item.clone();
    if py_truthy(&_inside_a_bracket(&[head.clone(), at.clone()])) {
    continue;
    }
    left = mcall(&mcall(&slice_or_raise(&head.clone(), &Py::None, &at.clone(), &Py::None), "strip", &[py_str(" ,;:.")]), "split", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT_STEP())) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_DANGLING(), &mcall(&geti(&left.clone(), &op_neg(&Py::Int(1))), "lower", &[])))) {
    continue;
    }
    cut = at.clone();
    break;
    }
    if py_truthy(&cut.clone()) {
    head = mcall(&slice_or_raise(&head.clone(), &Py::None, &cut.clone(), &Py::None), "strip", &[py_str(" ,;:.")]);
    words = mcall(&head.clone(), "split", &[]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp61: Py = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(3), &__cmp61)) { op_le(&__cmp61, &limit.clone()) } else { Py::Bool(false) } }))) {
    first_half = _the_first_instruction_in_a_long_piece(&[head.clone(), verbs.clone()]);
    if py_truthy(&Py::Bool(py_is(&first_half.clone(), &Py::None))) {
    continue;
    }
    let __unpk7 = py_unpack(&py_tuple(vec![first_half.clone(), mcall(&first_half.clone(), "split", &[])]), 2, None);
    head = __unpk7[0].clone();
    words = __unpk7[1].clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&verbs.clone(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_verb_after_a_leading_phrase(&[head.clone(), verbs.clone()]))) } }) {
    continue;
    }
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:first|next)$")); __args.push(py_str("")); __args.push(head.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = mcall(&trimmed.clone(), "split", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DANGLING(), &mcall(&geti(&mcall(&trimmed.clone(), "split", &[]), &op_neg(&Py::Int(1))), "lower", &[]))) } }) {
    head = trimmed.clone();
    }
    let _ = mcall(&found.clone(), "append", &[op_add(&mcall(&geti(&head.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&head.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    let __iter8 = py_iter_or_raise(&found.clone());
    for __item in __iter8 {
    step = __item.clone();
    let _ = Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    item = _a_list_item(&[part.clone()]);
    if py_truthy(&item.clone()) {
    let _ = Py::None;
    }
    }
    }
    Py::None
}
pub fn _the_next_step(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut step: Py = Py::None;
    let mut steps: Py = Py::None;
    let _ = py_str("One thing out of the answer past the first, which they have not already had or tried.\n\n    \"ive tried stretching\" then \"what else can i do\" is a person two things into a list, and the\n    next item is the whole reply: handing back the stretch they just told us did nothing, or the\n    paragraph they are working through, is not.\n\n    The first instruction is skipped: the whole answer was handed over a turn ago, so \"what else\"\n    asks for what comes after it, and giving the opening sentence back with \"there is this one\n    too\" in front of it is the same thing said again with a new label.\n    ");
    steps = call_builtin("list", &[_the_steps(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&slice_or_raise(&steps.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    step = __item.clone();
    if py_truthy(&mcall(&pyrt::xrefs::xget("memory.topics", "RED_FLAG"), "match", &[step.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = _already_repeated(&[step.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { _they_say_they_tried(&[step.clone(), memory.clone()]) } }) {
    continue;
    }
    return step.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_first_step(args: &[Py]) -> Py {
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
    let mut step: Py = Py::None;
    let _ = py_str("The first thing to do in the answer, read more generously than `_a_move`.\n\n    The move reader above is tuned for a line that asks back at the answer generally, so it only\n    takes a verb phrase it recognises and only when the phrase is short. A \"where do i start\" is\n    a narrower ask, and the entries answer it with verbs that list does not have (\"track two\n    weeks of what leaves the account\", \"run a hot service wash\"), so this one takes those verbs\n    and cuts a long instruction at its qualifier: the verb phrase is the step, the rest is detail.\n    ");
    let __iter1 = py_iter_or_raise(&_the_steps(&[said.clone()]));
    for __item in __iter1 {
    step = __item.clone();
    return step.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_new_work(args: &[Py]) -> Py {
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
    let mut asked: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut line: Py = Py::None;
    let _ = py_str("True when a how-to question follows a definition of the same thing.\n\n    \"what is an apprenticeship\" then \"how do i get one\": the definition answered what the thing\n    is, and the person now asks how to get one, which its own entry answers (\"apply to them like\n    jobs, because that is what they are\"). Handed the definition back they got its first fact as\n    if it were a step: \"Start with this: Pay is at least the apprentice minimum wage\"\n    (@blue-hedgehog, @sky-tuna, #6152). A line that only asks back at the answer (\"how do i use\n    it\", \"how do i do that\", \"where do i start\") names no doing of its own, and \"can i save it\"\n    is answered from the answer by its own reader, so neither comes here.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = asks_to_put_it_right(&[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_IT(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_WHAT_IT_IS(), "match", &[call_builtin("str", &[{ let __b = asked.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[_doings(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_which_part(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks which part of the answer to do first, and names no doing of its own.\n\n    \"Where do i start\", \"what should i do first\", \"where should i begin with this\": the whole of\n    the line is the ask, so it is answered from the answer in hand and never by a fresh lookup.\n    \"How do i clean it\" is not this: it names a doing (\"clean\") that may be a question of its own\n    and a neighbouring entry answers it well, so the lookup keeps that line and this reader stays\n    out of it.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { mcall(&g_WORDING(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[asks_for_the_move(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HOW_IT(), "search", &[line.clone()]))) } };
    Py::None
}
pub fn _leads_for(args: &[Py]) -> Py {
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
    let _ = py_str("The lead-ins that answer this ask: the thing to do, or the thing to change.");
    return if py_truthy(&mcall(&g_CHANGE_ASK(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { g_CHANGE_LEADS() } else { g_MOVE_LEADS() };
    Py::None
}
pub fn _a_lead_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut leads: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "leads") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: leads"))) };
    let mut step: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut lead: Py = Py::None;
    let _ = py_str("One lead out of `leads`, and never one that says what the step's first word says.\n\n    Every lead in these pools marks the step as the first thing, which is the whole point of it -\n    except when the step opens on its own \"Start with\": then the reply says start twice, and to\n    somebody who asked where to start that reads like a machine that did not hear the question.\n    Only the lead can give way, because the step is the answer's own wording and trimming it is\n    how a step stops being the answer's. A placeholder is not a word of the lead, so it comes out\n    before the lead is read.\n    ");
    if py_truthy(&mcall(&g_STEP_SAYS_FIRST(), "search", &[mcall(&call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) {
    leads = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&leads.clone()) { let mut lead: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_LEAD_SAYS_FIRST(), "search", &[stdlib_call("re", "sub", &[py_str("\\{[a-z_]+\\}"), py_str(" "), lead.clone()]).unwrap_or_else(|e| exc::raise(e))])))) {  __out.push(lead.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    leads = { let __b = leads.clone(); if py_truthy(&__b) { __b } else { g_NOW_LEADS() } };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[leads.clone(), memory.clone(), text.clone()]);
    Py::None
}
pub fn the_move(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _wording: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The first step of the answer just given, when the line is asking which part to do.\n\n    \"Where do i start\" a turn after the answer has already been on the screen is not a request\n    for the paragraph again: the knowledge base reaches the same entry and frames it \"Same as\n    before: <the whole of it>\", which tells a person who has just read it nothing. The step is\n    already in the answer, so it is handed over in the answer's own words.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&asks_for_the_move(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_WORDING(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow (?:long|much|many|often|far|hot|soon)\\b"), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    step = _the_first_step(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { _already_repeated(&[step.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&_they_say_they_tried(&[step.clone(), memory.clone()])) {
    return Py::None;
    }
    _wording = op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    return _handed_back(&[memory.clone(), _a_lead_and_a_step(&[_a_lead_for(&[_leads_for(&[text.clone()]), _wording.clone(), memory.clone(), text.clone()]), _wording.clone()])]);
    Py::None
}
pub fn _they_say_they_tried(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a line just before this one reports having done this very thing.\n\n    The report does not have to be the bare \"i tried that\" the tried pools read: \"ive tried a hot\n    wash already\" names the thing it tried, and that is exactly the one the move must not hand\n    back.\n    ");
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&step.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ATTEMPT(), "search", &[line.clone()])))) {
    continue;
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(word.clone()); } } py_set(__out) })() };
    shared = op_bitand(&mine.clone(), &said.clone());
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&shared.clone()) { let mut word: Py = __item.clone(); __out.push(op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_figure_asked(args: &[Py]) -> Py {
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
    let mut _name: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut part: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen_a_card: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The sentence of the last answer that carries the figure this line asks for, or None.\n\n    \"how long should i leave it\", a turn after the bot said \"Wipe it, shake out the port, and\n    leave it in a warm dry place for 24 hours\", got the search hedge: the answers machinery goes\n    looking for an entry keyed on this line, and the line names no thing, so nothing is found -\n    while the figure asked for is in the answer already on the screen. The figure is handed back\n    in the answer's own sentence, so the person gets \"leave it in a warm dry place for 24 hours\"\n    rather than a shrug.\n\n    Narrow on purpose: the line must ask for one kind of figure and name nothing else, the answer\n    must be a knowledge answer from a turn or two ago, and the sentence handed back must carry a\n    figure of the kind asked for.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    asked = Py::None;
    let __iter1 = py_iter_or_raise(&g_FIGURE_ASKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _name = __unpk2[0].clone();
    ask = __unpk2[1].clone();
    figure = __unpk2[2].clone();
    if py_truthy(&mcall(&ask.clone(), "search", &[clean.clone()])) {
    asked = py_tuple(vec![ask.clone(), figure.clone()]);
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&asked.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&asked.clone(), 2, None);
    ask = __unpk3[0].clone();
    figure = __unpk3[1].clone();
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&ask.clone(), "sub", &[py_str(" "), clean.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FIGURE_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FIGURE_SAYING(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&rest.clone()) {
    return Py::None;
    }
    seen_a_card = Py::Bool(false);
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_LOOK_BACK()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    back = __unpk5[0].clone();
    turn = __unpk5[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HEDGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(50)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&figure.clone(), "search", &[said.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_same_thing(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    let __iter6 = py_iter_or_raise(&_sentences(&[said.clone()]));
    for __item in __iter6 {
    part = __item.clone();
    part = mcall(&mcall(&part.clone(), "strip", &[]), "rstrip", &[py_str(" ;")]);
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&figure.clone(), "search", &[part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already_repeated(&[part.clone(), memory.clone()]))) } } } }) {
    return op_add(&op_add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None)), &if py_truthy(&Py::Bool(py_contains(&py_str(".!?"), &slice_or_raise(&part.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)))) { py_str("") } else { py_str(".") });
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_keys_of(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut head: Py = Py::None;
    let mut key: Py = Py::None;
    let mut record: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The words the entry behind an answer was keyed on, or an empty set.\n\n    `memory[\"entry_answers\"]` keeps the keys of every card this chat was given, and a follow-up\n    can be about the card without using a word of the card's own sentences: \"where do i start\"\n    after a card on sorting out money says nothing about money, and the card's keys do.\n    ");
    head = slice_or_raise(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), &Py::None, &Py::Int(60), &Py::None);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("entry_answers")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&text.clone(), &Py::None, &Py::Int(60), &Py::None), &head.clone())) } }) {
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("keys")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut key: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    }
    }
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn answers_nothing(args: &[Py]) -> Py {
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
    let mut fallback: Py = Py::None;
    let mut verdict: Py = Py::None;
    let _ = py_str("True when the reply is one of the bot's own ways of not answering this turn.\n\n    The composer has a handful of lines for a question it cannot take on: the search hedge, the\n    note that there is nothing to stand behind, \"two goes at the same thing is enough\", \"that is\n    in the answer above\", \"tell me what you've got so far\". All of them are the right reply when\n    there is nothing else to say, and all of them are the wrong reply when the card on the screen\n    already says part of the answer, which is what `the_part_asked` and `what_it_knows` are for.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    verdict = pyrt::xrefs::xmodule("compose.verdict");
    if py_truthy(&{ let __b = mcall(&fallback.clone(), "_is_a_hedge", &[reply.clone()]); if py_truthy(&__b) { __b } else { mcall(&fallback.clone(), "_nothing_to_stand_behind", &[reply.clone()]) } }) {
    return Py::Bool(true);
    }
    return mcall(&fallback.clone(), "_in_a_pool", &[reply.clone(), py_tuple(vec![gattr(&fallback.clone(), "SAME_AGAIN"), gattr(&fallback.clone(), "HELP_ALREADY"), gattr(&fallback.clone(), "TIPS_LINES"), gattr(&fallback.clone(), "TIPS_THING"), gattr(&fallback.clone(), "TRIED_NO_CHANGE_ELSEWHERE"), gattr(&verdict.clone(), "ALL_OF_IT_LINES")])]);
    Py::None
}
pub fn the_advice_was_to_get_seen(args: &[Py]) -> Py {
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
    let mut stuck: Py = Py::None;
    let _ = py_str("True when the bot has already told them to be seen today - 111, A&E, a GP, a vet.\n\n    A follow-up after that is about everything else, and the card's own sentence put in front of\n    the advice again reads as the bot talking over what it just said.\n    ");
    stuck = pyrt::xrefs::xmodule("memory.stuck");
    return mcall(&stuck.clone(), "the_advice_was_to_get_seen", &[memory.clone()]);
    Py::None
}
pub fn the_part_asked(args: &[Py]) -> Py {
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
    let mut _mark2: Py = Py::None;
    let mut _name2: Py = Py::None;
    let mut _step: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut ask2: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asked_words: Py = Py::None;
    let mut back: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut doing_words: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut name: Py = Py::None;
    let mut on_the_card: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen_a_card: Py = Py::None;
    let mut still: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The sentence of a card in hand that answers this follow-up, or None.\n\n    \"is the food still ok\", two turns after the bot's answer about a freezer door, got \"A quick\n    search online will get you a solid answer\": the answers machinery looks for an entry keyed on\n    this line, finds none, and the card that would answer it is already on the screen. So the\n    sentence of that card which says what is safe, or which step has to be done, or the step it\n    says to take first, is handed back in the card's own words.\n\n    Narrow on purpose. The ask has to be one of the three kinds above, the answer has to be a\n    knowledge card from the last few turns, and the card has to be about the thing the line names\n    or about the thing the chat is still on. Nothing is invented: the sentence is the bot's own.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    kind = Py::None;
    let __iter1 = py_iter_or_raise(&g_PART_ASKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    ask = __unpk2[1].clone();
    mark = __unpk2[2].clone();
    if py_truthy(&mcall(&ask.clone(), "search", &[clean.clone()])) {
    kind = py_tuple(vec![name.clone(), mark.clone()]);
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&kind.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&kind.clone(), 2, None);
    name = __unpk3[0].clone();
    mark = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&name.clone(), &py_str("move"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_whether_to(&[clean.clone()]))) } }) {
    return Py::None;
    }
    asked_words = pyrt::xrefs::xcall("memory.topics", "words", &[clean.clone()]);
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("move")))) {
    let __iter4 = py_iter_or_raise(&g_PART_ASKS());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 3, None);
    _name2 = __unpk5[0].clone();
    ask2 = __unpk5[1].clone();
    _mark2 = __unpk5[2].clone();
    if py_truthy(&Py::Bool(py_is(&_mark2.clone(), &Py::None))) {
    continue;
    }
    found = mcall(&ask2.clone(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    asked_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked_words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&found.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e)), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    break;
    }
    }
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("move")))) {
    _step = _whether_to(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&mcall(&_step.clone(), "split", &[]), &Py::Int(2), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_PART_STAND_INS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, &Py::Int(1), &Py::None, &Py::None)))) {
    return Py::None;
    }
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked_words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK_VERB(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_TIME(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SELF(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PART_STAND_INS(), &word.clone())) } } } } } } } } } } } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    doing = py_str("");
    still = mcall(&g_STILL_ASKS(), "match", &[clean.clone()]);
    if py_truthy(&still.clone()) {
    doing_words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&still.clone(), "group", &[py_str("rest")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&doing_words.clone()) {
    doing = _a_stem(&[geti(&doing_words.clone(), &Py::Int(0))]);
    }
    }
    seen_a_card = Py::Bool(false);
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_LOOK_BACK()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    back = __unpk7[0].clone();
    turn = __unpk7[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HEDGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    continue;
    }
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(50))) {
    continue;
    }
    keys = _the_keys_of(&[said.clone(), memory.clone()]);
    on_the_card = { let __b = call_builtin("bool", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[op_bitand(&call_builtin("set", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &keys.clone())]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { seen_a_card.clone() } }) {
    continue;
    }
    seen_a_card = Py::Bool(true);
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&on_the_card.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_same_thing(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), said.clone(), memory.clone()]))) } } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_chat_is_still_on(&[said.clone(), memory.clone()]))) } } } }) {
    continue;
    }
    let __iter8 = py_iter_or_raise(&call_builtin("enumerate", &[_sentences(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter8 {
    let __unpk9 = py_unpack(&__item.clone(), 2, None);
    index = __unpk9[0].clone();
    part = __unpk9[1].clone();
    part = mcall(&mcall(&part.clone(), "strip", &[]), "rstrip", &[py_str(" ;")]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&part.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    continue;
    }
    if py_truthy(&mcall(&mcall(&part.clone(), "lower", &[]), "endswith", &[py_str("?")])) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_ALONE(), "match", &[part.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("start")))) {
    if py_truthy(&op_gt(&index.clone(), &Py::Int(0))) {
    break;
    }
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&mark.clone(), "search", &[part.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_CONDITION_CLAUSE(), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_MOVE(), "match", &[part.clone()]) } }) {
    continue;
    }
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&on_the_card.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&part.clone(), "lower", &[]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = doing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&part.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_a_stem(&[word.clone()])); } py_set(__out) })() }, &doing.clone())) } }) {
    continue;
    }
    if py_truthy(&_already_repeated(&[part.clone(), memory.clone()])) {
    continue;
    }
    return op_add(&op_add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None)), &if py_truthy(&Py::Bool(py_contains(&py_str(".!?"), &slice_or_raise(&part.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)))) { py_str("") } else { py_str(".") });
    }
    }
    return Py::None;
    Py::None
}
pub fn _whether_to(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut words_of_it: Py = Py::None;
    let _ = py_str("The step the line asks about, as \"whether to <it>\", or \"\".\n\n    The step has to start with a verb of doing a thing to something, and be short: \"should i take\n    her tonight\" and \"what should i bring\" are not asks about a step in hand, and \"whether to\n    bring\" is not a sentence. Only ever a yes/no about a move on the thing the answer was about.\n    ");
    let __iter1 = py_iter_or_raise(&g_WHETHER_ASKS());
    for __item in __iter1 {
    shape = __item.clone();
    found = mcall(&shape.clone(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    rest = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "split", &[])]), "strip", &[py_str(" .,")]);
    words_of_it = mcall(&rest.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words_of_it.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words_of_it.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_A_MOVE_WORDS(), &geti(&words_of_it.clone(), &Py::Int(0))))) {
    continue;
    }
    return op_add(&py_str("whether to "), &rest.clone());
    }
    let __iter2 = py_iter_or_raise(&g_METHOD_ASKS());
    for __item in __iter2 {
    shape = __item.clone();
    found = mcall(&shape.clone(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    rest = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "split", &[])]), "strip", &[py_str(" .,")]);
    words_of_it = mcall(&rest.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words_of_it.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words_of_it.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_METHOD(), &geti(&words_of_it.clone(), &Py::Int(0))))) {
    continue;
    }
    return op_add(&py_str("whether to use "), &rest.clone());
    }
    found = mcall(&g_STILL_ASKS(), "match", &[clean.clone()]);
    if py_truthy(&found.clone()) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&py_str("whether "), &mcall(&mcall(&found.clone(), "group", &[py_str("who")]), "lower", &[])), &py_str(" ")), &mcall(&mcall(&found.clone(), "group", &[py_str("aux")]), "lower", &[])), &py_str(" still ")), &mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "split", &[])]), "strip", &[py_str(" .,")]));
    }
    return py_str("");
    Py::None
}
pub fn _a_stem(args: &[Py]) -> Py {
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
    let mut suffix: Py = Py::None;
    let _ = py_str("\"preheat\" and \"preheating\" are one word said in two forms.\n\n    The gap this reads is a step the card never covered, and a card that says \"about five minutes\n    preheating\" has covered \"and how long should i preheat it\". Comparing the words as typed said\n    it had not, and the reply then told the person the packet knows - about the number the card in\n    front of them had just given.\n    ");
    word = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ings"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[suffix.clone()]); if !py_truthy(&__b) { __b } else { op_ge(&op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(4)) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    break;
    }
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("e")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _the_chat_is_still_on(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut here: Py = Py::None;
    let mut last: Py = Py::None;
    let mut side: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the card is the one the chat is still on, not just one it passed through.\n\n    Two things make a card the one in hand. Its words (and the words it is keyed on, because a\n    card written for the person's own line never says them back - \"slugs\" becomes \"them\") touch\n    the line just given, or they touch the reply just given, which is how a card stays in hand\n    when the bot has been handing parts of it back. A card four turns back that shares a word with\n    one of the lines in between is a different thing: the child-sleep card came back for a\n    question about putting a shift change in writing, because \"my kid is 6\" was still in the last\n    three lines. The reply just given there was \"Say the one thing that matters first\", which is\n    about nothing in that card, and the line just given was about writing it down.\n    ");
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __aug1 = op_bitor(&words.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_keys_of(&[said.clone(), memory.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(word.clone()); } } py_set(__out) })() });
    words = __aug1;
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_SUBJECT(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("bot"), py_str("user")]));
    for __item in __iter2 {
    side = __item.clone();
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[side.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_SUBJECT(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&op_bitand(&words.clone(), &here.clone())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _where_the_rest_is(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut turn: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turn") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turn"))) };
    let mut ask: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_str("") };
    let mut ask_topic: Py = Py::None;
    let mut card_read: Py = Py::None;
    let mut card_topic: Py = Py::None;
    let mut card_words: Py = Py::None;
    let mut needles: Py = Py::None;
    let mut place: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The one place that settles what an answer in hand does not cover, read off the answer.\n\n    Where the rest of it lives is read off the card's own line, not the ask: \"should i keep\n    driving\" carries no subject, and the subject of the chat is what the answer was about. The\n    card's own words, and the entries it came from, go in too: a timings card for a roast says\n    \"20 minutes per 500g ... a 2kg bird\" and names no oven at all, while the entry behind it is\n    keyed on one. Without them the place came from the topic of the turn that asked for the\n    figure - \"how long does it take anyway\" reads as maths, and the reply offered \"a calculator\"\n    for whether to cover a chicken with foil.\n    ");
    card_words = mcall(&py_str(" "), "join", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]);
    card_words = mcall(&op_add(&op_add(&card_words.clone(), &py_str(" ")), &mcall(&py_str(" "), "join", &[call_builtin("sorted", &[_the_keys_of(&[said.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))])), "strip", &[]);
    let __iter1 = py_iter_or_raise(&g_WHERE_TO_LOOK_WORDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    needles = __unpk2[0].clone();
    place = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&needles.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b")), card_words.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return place.clone();
    }
    }
    let __try3 = exc::catch(|| {
    card_read = { let __b = pyrt::xrefs::xcall("understanding", "understand", &[call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    card_topic = call_builtin("str", &[{ let __b = mcall(&card_read.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
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
    card_topic = py_str("");
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&ask.clone()) {
    let __try8 = exc::catch(|| {
    ask_topic = call_builtin("str", &[{ let __b = mcall(&{ let __b = pyrt::xrefs::xcall("understanding", "understand", &[ask.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["Exception"]) {
                __handled12 = true;
    ask_topic = py_str("");
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    if py_truthy(&{ let __b = card_topic.clone(); if !py_truthy(&__b) { __b } else { { let __b = ask_topic.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&card_topic.clone(), &ask_topic.clone())) } } } }) {
    return g_WHERE_TO_LOOK_DEFAULT();
    }
    }
    return mcall(&g_WHERE_TO_LOOK(), "get", &[card_topic.clone(), g_WHERE_TO_LOOK_DEFAULT()]);
    Py::None
}
pub fn what_it_knows(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut first_card: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut head: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut look: Py = Py::None;
    let mut missing: Py = Py::None;
    let mut move_words: Py = Py::None;
    let mut named: Py = Py::None;
    let mut on_the_card: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen_a_card: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut w: Py = Py::None;
    let mut whether: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What the answer in hand does know, when the follow-up asks something it never covered.\n\n    \"how do i soft boil an egg\" is answered with the timings, and the follow-up \"do i need to\n    prick them\" met \"That's beyond what I know well, sorry. Searching for it will turn up better\n    sources\" - as if the chat had not already answered half of it. The card is still on the\n    screen and the part of it that matters here is still true, so it comes back in the card's own\n    words, and the bit the card does not cover is named as a gap with the one place that knows it.\n    A search on its own is not a reply; this gives what the bot can, and only then points on.\n\n    Narrow on purpose. The card has to be a knowledge card from the last few turns, the ask has\n    to be a yes/no about a step, and the step has to be one the card never mentions - where the\n    card does name it, the answer is already there and `the_part_asked` hands it back instead.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" ?!.")]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    whether = _whether_to(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&whether.clone()))) {
    return Py::None;
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&whether.clone(), "split", &[]), &Py::Int(2), &Py::None, &Py::None)])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK_VERB(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_TIME(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SELF(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PART_STAND_INS(), &word.clone())) } } } } } } } } } } } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    seen_a_card = Py::Bool(false);
    gap = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_LOOK_BACK()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    back = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("answers"))) } }) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HEDGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    continue;
    }
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(50))) {
    continue;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &_the_keys_of(&[said.clone(), memory.clone()]))) {
    continue;
    }
    on_the_card = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_a_stem(&[word.clone()])); } py_set(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&on_the_card.clone(), &_a_stem(&[word.clone()])))) {  __out.push(word.clone()); } } py_list(__out) })() };
    missing = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&on_the_card.clone(), &_a_stem(&[word.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_QUALITY(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&missing.clone())) } }) {
    let __iter3 = py_iter_or_raise(&_sentences(&[said.clone()]));
    for __item in __iter3 {
    sentence = __item.clone();
    sentence = mcall(&mcall(&sentence.clone(), "strip", &[]), "rstrip", &[py_str(" ;")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { mcall(&mcall(&sentence.clone(), "lower", &[]), "endswith", &[py_str("?")]) } }) {
    continue;
    }
    if py_truthy(&op_bitand(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); __out.push(_a_stem(&[word.clone()])); } py_set(__out) })() }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&sentence.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_a_stem(&[word.clone()])); } py_set(__out) })() })) {
    head = if py_truthy(&Py::Bool(py_contains(&sentence.clone(), &py_str(":")))) { geti(&mcall(&sentence.clone(), "split", &[py_str(":")]), &Py::Int(0)) } else { sentence.clone() };
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    head = sentence.clone();
    }
    return op_add(&op_add(&mcall(&slice_or_raise(&head.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&head.clone(), &Py::Int(1), &Py::None, &Py::None)), &if py_truthy(&Py::Bool(py_contains(&py_str(".!?\""), &slice_or_raise(&head.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)))) { py_str("") } else { py_str(".") });
    }
    }
    continue;
    }
    seen_a_card = Py::Bool(true);
    first_card = { let __b = op_le(&back.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_add(&op_neg(&back.clone()), &Py::Int(1)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); __out.push({ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&t.clone(), "get", &[py_str("module")]), &py_str("answers"))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } };
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_chat_is_still_on(&[said.clone(), memory.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&first_card.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_same_thing(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), said.clone(), memory.clone()]))) } } } }) {
    continue;
    }
    move_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&mcall(&whether.clone(), "split", &[]), &Py::Int(2), &Py::None, &Py::None)) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_PART_STAND_INS(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    on_the_card = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_a_stem(&[word.clone()])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&slice_or_raise(&move_words.clone(), &Py::Int(1), &Py::None, &Py::None)))) {
    continue;
    }
    part = py_str("");
    lead = py_str("");
    let __iter4 = py_iter_or_raise(&_sentences(&[said.clone()]));
    for __item in __iter4 {
    sentence = __item.clone();
    sentence = mcall(&mcall(&sentence.clone(), "strip", &[]), "rstrip", &[py_str(" ;")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { mcall(&mcall(&sentence.clone(), "lower", &[]), "endswith", &[py_str("?")]) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&lead.clone())); if !py_truthy(&__b) { __b } else { { let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&mcall(&sentence.clone(), "lower", &[]), "endswith", &[py_str("?")]))); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_ALONE(), "match", &[sentence.clone()]))) } } } } } } } }) {
    lead = sentence.clone();
    }
    continue;
    }
    if py_truthy(&_already_repeated(&[sentence.clone(), memory.clone()])) {
    continue;
    }
    if py_truthy(&op_bitand(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); __out.push(_a_stem(&[word.clone()])); } py_set(__out) })() }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&sentence.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_a_stem(&[word.clone()])); } py_set(__out) })() })) {
    return Py::None;
    }
    part = sentence.clone();
    break;
    }
    look = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); __args.push(turn.clone()); __kws.extend(vec![("ask".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _where_the_rest_is(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&look.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    gap = op_add(&op_add(&op_add(&op_add(&py_str("On "), &whether.clone()), &py_str(", ")), &look.clone()), &py_str(" will know more than I can - nothing I have covers it."));
    continue;
    }
    head = op_add(&op_add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None)), &if py_truthy(&Py::Bool(py_contains(&py_str(".!?\""), &slice_or_raise(&part.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)))) { py_str("") } else { py_str(".") });
    if py_truthy(&lead.clone()) {
    head = op_add(&op_add(&op_add(&mcall(&slice_or_raise(&lead.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&lead.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &head.clone());
    }
    if py_truthy(&op_modulo(&call_builtin("len", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&head.clone(), &py_str(" On ")), &whether.clone()), &py_str(", ")), &look.clone()), &py_str(" will know more than I can."));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&head.clone(), &py_str(" That is what I can tell you; on ")), &whether.clone()), &py_str(", ")), &look.clone()), &py_str(" will say."));
    }
    return { let __b = gap.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn the_plain_line(args: &[Py]) -> Py {
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
    let _ = py_str("The line with \"were\" read as \"we are\" where that is what it says, and otherwise as it is.");
    text = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_A_WERE_ASKS(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("\\A\\s*were\\s+\\w"), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return text.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\A(\\s*)were\\b")); __args.push(py_str("\\1we are")); __args.push(text.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn how_long_its_been(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks how long the person's own thing has been like this.");
    found = mcall(&g_A_HOW_LONG_ITS_BEEN(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    return call_builtin("bool", &[{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__OWNS_THE_THING(), "search", &[mcall(&found.clone(), "group", &[Py::Int(0)])]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_line_that_corrects(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether a line takes a figure back, whether it opens with the word or ends with it.");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_A_CORRECTION(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    found = mcall(&g_A_MARKER_AT_THE_END(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    head = mcall(&slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.facts", "extract", &[head.clone()])));
    Py::None
}
pub fn _a_list_of_ages(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line lists ages, which names the members of a group rather than its size.");
    return call_builtin("bool", &[mcall(&g_A_LIST_OF_AGES(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _something_gone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut later: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a later line says one of the counted things is gone, so the count is out of date.");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&lines.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter1 {
    later = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&later.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_A_GAIN(), "search", &[later.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_GROUP_WORD(), "search", &[later.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_LOSS(), "search", &[later.clone()])))) {
    continue;
    }
    said = pyrt::xrefs::xcall("memory.topics", "keywords", &[later.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&said.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&mcall(&mcall(&g_A_LOSS(), "search", &[later.clone()]), "group", &[Py::Int(0)]), "lower", &[]), &py_str("one")))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_ask_gave_that_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut first: Py = Py::None;
    let mut looked: Py = Py::None;
    let mut other: Py = Py::None;
    let _ = py_str("Whether the ask itself names the figure the reading found.\n\n    A number in the ask - \"for 8\", \"how many for 10 people\" - is the ask's own quantity, so it is\n    not a figure the person gave earlier and saying it back claims a recall that did not happen.\n    ");
    number = mcall(&mcall(&call_builtin("str", &[{ let __b = number.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&number.clone()))) {
    return Py::Bool(false);
    }
    first = geti(&mcall(&number.clone(), "split", &[]), &Py::Int(0));
    looked = py_set(vec![first.clone(), mcall(&g_THE_SAME_NUMBER(), "get", &[first.clone(), first.clone()])]);
    let __iter1 = py_iter_or_raise(&mcall(&g_A_NUMBER_WORD(), "findall", &[clean.clone()]));
    for __item in __iter1 {
    other = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&looked.clone(), &mcall(&g_THE_SAME_NUMBER(), "get", &[other.clone(), other.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&other.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&slice_or_raise(&first.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[]); if !py_truthy(&__b) { __b } else { _the_same_word(&[first.clone(), other.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_count_of_one_with_a_plural(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut noun: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "noun") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: noun"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether a count of one is about to be said with a noun in the plural: \"one people\".");
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("1"), py_str("one"), py_str("a"), py_str("single")]), &mcall(&mcall(&call_builtin("str", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])))) {
    return Py::Bool(false);
    }
    word = mcall(&geti(&{ let __b = mcall(&noun.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &op_neg(&Py::Int(1))), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&g_A_PLURAL_ONLY_NOUN(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("s")]) } };
    Py::None
}
pub fn _the_thing_counted(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The ask's word for the thing counted, with the figure's own modifier when it has one.");
    match_ = mcall(&g_COUNT_ASK(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    phrase = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrase.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK_FRAME(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FIGURE_NOUNS(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    tail = mcall(&geti(&mcall(&said.clone(), "split", &[]), &op_neg(&Py::Int(1))), "lower", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("zip", &[phrase.clone(), slice_or_raise(&phrase.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    one = __unpk2[0].clone();
    other = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &other.clone())); if !py_truthy(&__b) { __b } else { _the_same_word(&[one.clone(), tail.clone()]) } }) {
    return op_add(&op_add(&one.clone(), &py_str(" ")), &other.clone());
    }
    }
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&_the_same_word(&[word.clone(), tail.clone()])) {
    return word.clone();
    }
    }
    return geti(&words.clone(), &op_neg(&Py::Int(1)));
    Py::None
}
pub fn _figures_in(args: &[Py]) -> Py {
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
    let mut _who: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut digits: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut whose: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The figures a line gives, as (kind, the words used, the number, whether it counted).\n\n    A figure that belongs to somebody else is not one they gave: \"my mum has three brothers\" is\n    the mother's count, and \"how many brothers do i have\" must not be answered with it. The\n    subject of the clause is read by the store's own reader (`facts.subject`), so a relation, a\n    pronoun or a name is somebody else everywhere at once.\n    ");
    text = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_A_QUESTION(), "match", &[text.clone()]) } } } }) {
    return py_list(vec![]);
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_A_FIGURE(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&{ let __b = mcall(&match_.clone(), "group", &[py_str("time")]); if !py_truthy(&__b) { __b } else { mcall(&g_A_RATE_OF_TIME(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&match_.clone(), "group", &[py_str("time")]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_SCHEDULED_AFTER(), "match", &[slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_HAS_THE_LENGTH(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]))) } } } }) {
    continue;
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]), "rstrip", &[py_str(" ,.;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&match_.clone(), "group", &[py_str("count")]); if !py_truthy(&__b) { __b } else { mcall(&g_A_ONE_MORE(), "match", &[mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&said.clone(), "split", &[]), &Py::Int(1), &Py::None, &Py::None)])]) } }) {
    continue;
    }
    let __try2 = exc::catch(|| {
    let __unpk7 = py_unpack(&pyrt::xrefs::xcall("memory.facts", "subject", &[text.clone(), mcall(&match_.clone(), "start", &[])]), 2, None);
    whose = __unpk7[0].clone();
    _who = __unpk7[1].clone();
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
    whose = py_str("");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&Py::Bool(py_eq(&whose.clone(), &py_str("other")))) {
    continue;
    }
    counted = Py::Bool(!py_is(&mcall(&match_.clone(), "group", &[py_str("count")]), &Py::None));
    kind = if py_truthy(&mcall(&match_.clone(), "group", &[py_str("time")])) { py_str("time") } else { py_str("amount") };
    if py_truthy(&mcall(&match_.clone(), "group", &[py_str("share")])) {
    digits = stdlib_call("re", "match", &[py_str("\\s*([\\d][\\d,]*(?:\\.\\d+)?)"), mcall(&match_.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e));
    number = if py_truthy(&digits.clone()) { mcall(&digits.clone(), "group", &[Py::Int(1)]) } else { said.clone() };
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![kind.clone(), op_add(&number.clone(), &py_str(" per cent")), number.clone(), Py::Bool(false)])]);
    continue;
    }
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "lower", &[]), "endswith", &[py_str(" per")]) } }) {
    said = mcall(&slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(4)), &Py::None), "rstrip", &[py_str(" ,.;:")]);
    }
    words = mcall(&said.clone(), "split", &[]);
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FIGURE_TAIL(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[]))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    said = geti(&stdlib_call("re", "split", &[py_str("(?<!\\d),|,(?!\\d)"), mcall(&py_str(" "), "join", &[words.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    said = mcall(&said.clone(), "rstrip", &[py_str(" ,.;:")]);
    number = mcall(&if py_truthy(&counted.clone()) { geti(&mcall(&said.clone(), "split", &[]), &Py::Int(0)) } else { said.clone() }, "rstrip", &[py_str(" ,.;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&number.clone()))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![kind.clone(), said.clone(), number.clone(), counted.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn _money_paid_for_a_thing(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let _ = py_str("True when the line's figure is money spent on a thing, and the ask wants the thing itself.");
    if py_truthy(&mcall(&g__AN_AMOUNT_WANTED(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__MONEY_PAID_FOR(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _one_figure(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut one: Py = Py::None;
    let _ = py_str("The figure of the kind asked for, or () when the line holds none or holds two kinds.");
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![]);
    }
    if py_truthy(&kind.clone()) {
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&geti(&one.clone(), &Py::Int(0)), &kind.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() };
    } else {
    if py_truthy(&op_gt(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut one: Py = __item.clone(); __out.push(geti(&one.clone(), &Py::Int(0))); } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return py_tuple(vec![]);
    }
    }
    return if py_truthy(&found.clone()) { geti(&found.clone(), &Py::Int(0)) } else { py_tuple(vec![]) };
    Py::None
}
pub fn _a_bare_count(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("Whether a figure is the number on its own, with nothing saying what it counts.\n\n    The span line is written for a figure a person gave in their own words with its unit -\n    \"£250 a month - that's what you told me\" - and with no unit in it the line reads as a\n    template cut short. A count said back is just the count: \"You said 5.\"\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = figure.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    return { let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_NUMBER_WORD(), "fullmatch", &[geti(&words.clone(), &Py::Int(0))])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_same_word(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut second: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("Whether two words are the same thing said twice: \"move\" and \"moving\", \"bed\" and \"bedrooms\".");
    let __unpk1 = py_unpack(&py_tuple(vec![pyrt::xrefs::xcall("memory.topics", "stem", &[one.clone()]), pyrt::xrefs::xcall("memory.topics", "stem", &[other.clone()])]), 2, None);
    first = __unpk1[0].clone();
    second = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&first.clone(), &second.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&op_bitand(&pyrt::xrefs::xcall("memory.topics", "forms", &[first.clone()]), &pyrt::xrefs::xcall("memory.topics", "forms", &[second.clone()]))) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![first.clone(), second.clone()]), py_tuple(vec![second.clone(), first.clone()])]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    shorter = __unpk3[0].clone();
    longer = __unpk3[1].clone();
    if py_truthy(&{ let __b = mcall(&longer.clone(), "endswith", &[py_str("room")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&longer.clone(), &Py::None, &op_neg(&Py::Int(4)), &Py::None), &shorter.clone())) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PAY_FAMILY(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PAY_FAMILY(), &second.clone())) } }) {
    return Py::Bool(true);
    }
    return { let __b = op_ge(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[second.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&first.clone(), "startswith", &[second.clone()]); if py_truthy(&__b) { __b } else { mcall(&second.clone(), "startswith", &[first.clone()]) } } } } } };
    Py::None
}
pub fn _the_corrected(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut kind: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut found: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut got: Py = Py::None;
    let mut later: Py = Py::None;
    let _ = py_str("The figure a later line takes back to, or the one it was given, or () when it is unsure.\n\n    \"my budget is 500\" / \"make it 900\" / \"what is my budget\": the last figure they said for the\n    thing is the one they mean. A correction with no figure in it (\"no pork remember\") takes\n    nothing back, so the figure stands.\n    ");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&lines.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter1 {
    later = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&later.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_line_that_corrects(&[later.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_A_LINE_THAT_ASKS(), "search", &[later.clone()]) } } } }) {
    continue;
    }
    got = _one_figure(&[_figures_in(&[later.clone()]), kind.clone()]);
    if py_truthy(&got.clone()) {
    found = got.clone();
    } else {
    if py_truthy(&{ let __b = mcall(&g_A_NUMBER_WORD(), "search", &[later.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&kind.clone(), &py_str("time"))) } }) {
    return py_tuple(vec![]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_length_they_mean(args: &[Py]) -> Py {
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
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut spoken: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spoken") { v.clone() }
    else { py_tuple(vec![]) };
    let mut _back: Py = Py::None;
    let mut anywhere: Py = Py::None;
    let mut back: Py = Py::None;
    let mut distinct: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut heard: Py = Py::None;
    let mut later: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut new: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The length a recall ask points at, in the person's own words, or \"\".\n\n    Two lengths in a chat is normally a question of which, so the thing the ask names decides:\n    \"how long did i tell you the headaches been going on\" picks the line about the headache even\n    when the person also said a root canal was a month ago. With nothing named to go on, the one\n    length they gave anywhere is the answer, and a corrected one is read as corrected. Anything\n    still unclear is left to the module that knows the thing, so this says \"\" rather than guess.\n\n    `spoken` is what the bot itself said in the same window. A word of the ask that no line of\n    theirs carries is not always a new thing: \"sorry, how long did i tell you it has been sore\"\n    after \"i hurt my shoulder about a week ago\" was left unanswered because the person's line says\n    \"hurt\" and the ask says \"sore\", a word only the replies had used. When every word the ask adds\n    to the cue is a word the chat has already used, the ask is naming nothing new and the one length\n    in the chat is the one they mean - while \"how long did i say the shop was open\" still names a\n    thing the chat has never mentioned, and is still left alone.\n    ");
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[clean.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_FRAME(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING_ASKED(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = wanted.clone(); if !py_truthy(&__b) { __b } else { spoken.clone() } }) {
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()])) { let mut other: Py = __item.clone(); __out.push(other.clone()); } } py_list(__out) })() };
    heard = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spoken.clone()) { let mut line: Py = __item.clone(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()])) { let mut other: Py = __item.clone(); __out.push(other.clone()); } } py_list(__out) })() };
    new = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&theirs.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = new.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&new.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&heard.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    wanted = py_list(vec![]);
    }
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    named = __unpk1[0].clone();
    anywhere = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    back = __unpk3[0].clone();
    line = __unpk3[1].clone();
    if py_truthy(&mcall(&g_A_LINE_THAT_ASKS(), "search", &[line.clone()])) {
    continue;
    }
    figures = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_figures_in(&[line.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&geti(&one.clone(), &Py::Int(0)), &py_str("time")))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&figures.clone()))) {
    continue;
    }
    let _ = mcall(&anywhere.clone(), "append", &[py_tuple(vec![back.clone(), figures.clone()])]);
    words = pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&words.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&named.clone(), "append", &[py_tuple(vec![back.clone(), figures.clone()])]);
    }
    }
    let __iter4 = py_iter_or_raise(&if py_truthy(&wanted.clone()) { py_tuple(vec![named.clone()]) } else { py_tuple(vec![named.clone(), anywhere.clone()]) });
    for __item in __iter4 {
    pool = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    continue;
    }
    distinct = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let __unpk5765 = py_unpack(&__item.clone(), 2, None); let mut _back: Py = __unpk5765[0].clone(); let mut figures: Py = __unpk5765[1].clone(); for __item in py_iter_or_raise(&figures.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&geti(&one.clone(), &Py::Int(1)), "lower", &[])); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[distinct.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    let __unpk5 = py_unpack(&geti(&pool.clone(), &op_neg(&Py::Int(1))), 2, None);
    back = __unpk5[0].clone();
    figures = __unpk5[1].clone();
    later = _the_corrected(&[lines.clone(), back.clone(), py_str("time"), geti(&figures.clone(), &op_neg(&Py::Int(1)))]);
    return geti(&{ let __b = later.clone(); if py_truthy(&__b) { __b } else { geti(&figures.clone(), &op_neg(&Py::Int(1))) } }, &Py::Int(1));
    }
    if py_truthy(&mcall(&g_A_CORRECTION(), "search", &[geti(&lines.clone(), &geti(&geti(&pool.clone(), &op_neg(&Py::Int(1))), &Py::Int(0)))])) {
    return geti(&geti(&geti(&geti(&pool.clone(), &op_neg(&Py::Int(1))), &Py::Int(1)), &op_neg(&Py::Int(1))), &Py::Int(1));
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_figure_of_a_goal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut wanted: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { py_tuple(vec![]) };
    let mut at: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut narrowing: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The figure on the line where the person set a goal, or () when there is none.\n\n    A goal is the amount they are trying to reach, so a rate they put by is skipped even though\n    it is a figure of the same kind. Where the line states both an amount and a length (\"save\n    8000 over two years\") and the ask names neither (\"what was my target again\"), the amount is\n    the target and the length is how long it takes. A word the ask adds besides the goal noun\n    (\"the van\", \"savings\") narrows which goal it is, so the line has to carry it too.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    line = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_LINE_THAT_ASKS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_GOAL_LINE(), "search", &[line.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_RATE_LINE(), "search", &[line.clone()])) {
    continue;
    }
    words = pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()]);
    narrowing = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_GOAL_NOUNS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&narrowing.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    found = _one_figure(&[_figures_in(&[line.clone()]), kind.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&kind.clone(), &Py::None)) } }) {
    found = _one_figure(&[_figures_in(&[line.clone()]), py_str("amount")]);
    }
    if py_truthy(&found.clone()) {
    return _the_corrected(&[lines.clone(), at.clone(), kind.clone(), found.clone()]);
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _the_figure_counts_the_thing(args: &[Py]) -> Py {
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
    let mut figure: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut named: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("True when a figure is an answer to an ask that names a thing, or the ask names nothing.\n\n    A number answers a number's question, and an ask that names the thing it wants back is not\n    one: \"what did i say about lisbon\", asked after \"im saving 3000 for a trip to lisbon\" and\n    \"the flights are about 120 each\", was answered \"You said 3000.\" - a figure given two lines\n    after the thing the ask named, with nothing on it to say what the figure is of. The same\n    shape answered \"what did i say about the tap\" with \"About a week\", a length from a line about\n    something else.\n\n    So the figure comes back only when it carries the thing asked about (\"what did i say about\n    the cats\" after \"i have two cats\" is \"two cats\"). When it does not, the answer is the line\n    that names the thing, chosen by the one check in bot/memory/history.py and quoted below.\n    ");
    thing = pyrt::xrefs::xcall("memory.history", "the_thing_they_named", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::Bool(true);
    }
    named = call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "content", &[thing.clone()])]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[op_bitand(&named.clone(), &call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "content", &[call_builtin("str", &[{ let __b = figure.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_narrowing_and_its_line(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut places: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line `line` narrows, with `line` after it, or None.\n\n    A recall finds the line that carries the word asked about, and the last one can be a narrowing\n    of the line above it: \"any chemist in the area\" / \"the ones that are open on sunday\", then\n    \"what did i say about sunday\". The only line with \"sunday\" in it is the narrowing, so the\n    answer was `You said: \"the ones that are open on sunday\"` - their own fragment, which says\n    nothing about them and nothing about the chemist either. What they are asking about is the\n    line the fragment narrows, so it is said with the narrowing after it.\n\n    The narrowing must be the line *right after* the one it narrows: a short line three turns\n    later is about whatever that turn was about. It must be a narrowing on its own terms (see\n    `A_NARROWING_ONLY`), so \"i bought new tyres\" is never read as one, and the line it narrows\n    must state a thing - a question narrowed is the chat still looking for the thing.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_A_NARROWING_WORDS()) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_NARROWING_ONLY(), "match", &[said.clone()])))) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])); } } py_list(__out) })() };
    places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk4101 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk4101[0].clone(); let mut text: Py = __unpk4101[1].clone(); if py_truthy(&Py::Bool(py_eq(&text.clone(), &said.clone()))) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&places.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&places.clone(), &op_neg(&Py::Int(1))))) } }) {
    return Py::None;
    }
    before = geti(&lines.clone(), &op_sub(&geti(&places.clone(), &op_neg(&Py::Int(1))), &Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&before.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[mcall(&before.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.topics", "content", &[before.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_A_NARROWING_ONLY(), "match", &[before.clone()]) } } } } } } } }) {
    return Py::None;
    }
    return py_tuple(vec![before.clone(), said.clone()]);
    Py::None
}
pub fn _the_turn_that_names_it(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("The line they said about the thing the ask names, quoted back, or None.");
    row = pyrt::xrefs::xcall("memory.history", "the_earlier_turn", &[memory.clone(), text.clone()]);
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = row.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "rstrip", &[py_str(" .!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    pair = a_narrowing_and_its_line(&[line.clone(), memory.clone()]);
    if py_truthy(&pair.clone()) {
    let __unpk1 = py_unpack(&pair.clone(), 2, None);
    before = __unpk1[0].clone();
    after = __unpk1[1].clone();
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("You said: \""), &before.clone()), &py_str("\" — then \"")), &after.clone()), &py_str("\".")), op_add(&op_add(&op_add(&op_add(&py_str("Your own words were: \""), &before.clone()), &py_str("\", then \"")), &after.clone()), &py_str("\".")), op_add(&op_add(&op_add(&op_add(&py_str("That was: \""), &before.clone()), &py_str("\" — then \"")), &after.clone()), &py_str("\"."))]), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You said: \""), &line.clone()), &py_str("\"")), op_add(&op_add(&py_str("Your own words were: \""), &line.clone()), &py_str("\"")), op_add(&op_add(&py_str("That was: \""), &line.clone()), &py_str("\""))]), memory.clone()]);
    Py::None
}
pub fn the_figure_they_gave(args: &[Py]) -> Py {
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
    let mut _back: Py = Py::None;
    let mut _figure: Py = Py::None;
    let mut _score: Py = Py::None;
    let mut all_lines: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut back: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut beside: Py = Py::None;
    let mut best: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut core: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut counting: Py = Py::None;
    let mut distinct: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut found: Py = Py::None;
    let mut goal_named: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut kind_of: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut many: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut number: Py = Py::None;
    let mut one: Py = Py::None;
    let mut one_name: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut score: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The figure a person gave earlier and asks for again, or None.\n\n    Only a line that asks for the figure back, names the thing it is for, and has that thing and\n    the figure in the chat within the last few turns is answered. Everything else is left to the\n    module that knows about the thing asked.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(16)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GAVE_ASKS(), "search", &[clean.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_A_FIGURE_ASK(), "search", &[clean.clone()])) {
    return Py::None;
    }
    counting = mcall(&g_COUNT_ASK(), "search", &[clean.clone()]);
    if py_truthy(&counting.clone()) {
    counted = pyrt::xrefs::xcall("memory.counts", "answer", &[clean.clone(), memory.clone()]);
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&counting.clone(), "group", &[Py::Int(1)]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&pyrt::xrefs::xget("memory.counts", "THEIRS"), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[clean.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_FRAME(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&mcall(&g_A_PAY_WORD(), "search", &[clean.clone()])) {
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_PAY_WHEN_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wanted.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PAY_WORD(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_PAY_VERB_AT_THE_END(), "search", &[clean.clone()]) } } } }) {
    wanted = py_list(vec![py_str("pay")]);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::None;
    }
    goal_named = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_GOAL_NOUNS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    kind = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("often"), py_str("time"), py_str("heat"), py_str("amount")]));
    for __item in __iter1 {
    name = __item.clone();
    let __iter2 = py_iter_or_raise(&g_FIGURE_ASKS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    one_name = __unpk3[0].clone();
    ask = __unpk3[1].clone();
    _figure = __unpk3[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&one_name.clone(), &name.clone())); if !py_truthy(&__b) { __b } else { mcall(&ask.clone(), "search", &[clean.clone()]) } }) {
    kind = name.clone();
    break;
    }
    }
    if py_truthy(&kind.clone()) {
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("amount"), py_str("heat")]), &kind.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PLAN_ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_THE_PLAN_ASK(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    all_lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(the_plain_line(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } } py_list(__out) })() };
    lines = slice_or_raise(&all_lines.clone(), &op_neg(&g_GAVE_LOOK_BACK()), &Py::None, &Py::None);
    spoken = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }, &op_neg(&g_GAVE_LOOK_BACK()), &Py::None, &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { mcall(&g_A_TOLD_BACK(), "search", &[clean.clone()]) } }) {
    said = _the_length_they_mean(&[clean.clone(), lines.clone(), spoken.clone()]);
    if py_truthy(&said.clone()) {
    if py_truthy(&Py::Bool(!py_truthy(&_the_figure_counts_the_thing(&[clean.clone(), said.clone()])))) {
    return _the_turn_that_names_it(&[text.clone(), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You said "), &said.clone()), &py_str(".")), op_add(&op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" — that's what you told me."))]), memory.clone()]);
    }
    }
    if py_truthy(&goal_named.clone()) {
    found = _the_figure_of_a_goal(&[all_lines.clone(), kind.clone(), wanted.clone()]);
    if py_truthy(&found.clone()) {
    figure = geti(&found.clone(), &Py::Int(1));
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You said "), &figure.clone()), &py_str(".")), op_add(&op_add(&mcall(&slice_or_raise(&figure.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&figure.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" — that's what you told me."))]), memory.clone()]);
    }
    }
    if py_truthy(&wanted.clone()) {
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()])) { let mut other: Py = __item.clone(); __out.push(other.clone()); } } py_list(__out) })() };
    let __aug4 = op_add(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&mcall(&g_A_TAKE_HOME(), "search", &[line.clone()])) {  __out.push(py_str("takehome")); } } py_list(__out) })() });
    said = __aug4;
    let __aug5 = op_add(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&mcall(&g_A_IM_ON_A_FIGURE(), "search", &[line.clone()])) {  __out.push(py_str("pay")); } } py_list(__out) })() });
    said = __aug5;
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_A_MEASURE_WORD(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_DOING_WORD(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&kept.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_PAY_WORD(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PAY_JUDGEMENT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_THING_NAMED_AFTER_THE_CUE(), "search", &[clean.clone()])))) {
    wanted = kept.clone();
    }
    }
    if py_truthy(&_the_store_keeps_this_figure(&[clean.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    bare = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("amount")))) { mcall(&g_A_BARE_AMOUNT_ASK(), "search", &[clean.clone()]) } else { if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("time")))) { mcall(&g_A_BARE_DURATION_ASK(), "search", &[clean.clone()]) } else { Py::None } };
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return Py::None;
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk47 = py_unpack(&__item.clone(), 2, None); let mut back: Py = __unpk47[0].clone(); let mut line: Py = __unpk47[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_LINE_THAT_ASKS(), "search", &[line.clone()])))) {  for __item in py_iter_or_raise(&_figures_in(&[line.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&geti(&one.clone(), &Py::Int(0)), &kind.clone()))) {  __out.push(py_tuple(vec![back.clone(), geti(&one.clone(), &Py::Int(1))])); } } } } py_list(__out) })() };
    distinct = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let __unpk1487 = py_unpack(&__item.clone(), 2, None); let mut _back: Py = __unpk1487[0].clone(); let mut one: Py = __unpk1487[1].clone(); __out.push(mcall(&one.clone(), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[distinct.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    corrected = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let __unpk2255 = py_unpack(&__item.clone(), 2, None); let mut back: Py = __unpk2255[0].clone(); let mut one: Py = __unpk2255[1].clone(); if py_truthy(&mcall(&g_A_CORRECTION(), "search", &[geti(&lines.clone(), &back.clone())])) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&corrected.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&geti(&found.clone(), &op_neg(&Py::Int(1))), &Py::Int(1)), &geti(&corrected.clone(), &op_neg(&Py::Int(1))))) } }) {
    return Py::None;
    }
    figure = geti(&corrected.clone(), &op_neg(&Py::Int(1)));
    } else {
    figure = geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(1));
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_figure_counts_the_thing(&[clean.clone(), figure.clone()])))) {
    return _the_turn_that_names_it(&[text.clone(), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You said "), &figure.clone()), &py_str(".")), op_add(&op_add(&mcall(&slice_or_raise(&figure.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&figure.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" — that's what you told me."))]), memory.clone()]);
    }
    best = Py::None;
    let __iter6 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    back = __item.clone();
    line = geti(&lines.clone(), &back.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_LINE_THAT_ASKS(), "search", &[line.clone()]) } }) {
    continue;
    }
    said = pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()]);
    if py_truthy(&mcall(&g_A_TAKE_HOME(), "search", &[line.clone()])) {
    said = op_add(&said.clone(), &py_list(vec![py_str("takehome")]));
    }
    if py_truthy(&mcall(&g_A_IM_ON_A_FIGURE(), "search", &[line.clone()])) {
    said = op_add(&said.clone(), &py_list(vec![py_str("pay")]));
    }
    core = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FIGURE_NOUNS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { wanted.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&core.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    shared = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    figures = _figures_in(&[line.clone()]);
    found = _one_figure(&[figures.clone(), kind.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    beside = _the_figure_beside_the_thing(&[figures.clone(), core.clone()]);
    if py_truthy(&beside.clone()) {
    found = beside.clone();
    }
    if py_truthy(&{ let __b = mcall(&g_COUNT_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { _a_list_of_ages(&[line.clone()]) } }) {
    continue;
    }
    if py_truthy(&_money_paid_for_a_thing(&[clean.clone(), line.clone()])) {
    continue;
    }
    found = _the_corrected(&[lines.clone(), back.clone(), kind.clone(), found.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    many = _every_figure_the_ask_names(&[wanted.clone(), line.clone(), kind.clone()]);
    last = geti(&{ let __b = mcall(&geti(&found.clone(), &Py::Int(1)), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &op_neg(&Py::Int(1)));
    if py_truthy(&_something_gone(&[lines.clone(), back.clone(), op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&shared.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, &py_list(vec![last.clone()]))])) {
    continue;
    }
    score = py_tuple(vec![call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), back.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&score.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![score.clone(), found.clone(), shared.clone(), many.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&best.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk7 = py_unpack(&best.clone(), 4, None);
    _score = __unpk7[0].clone();
    let __unpk8 = py_unpack(&__unpk7[1].clone(), 4, None);
    kind_of = __unpk8[0].clone();
    said = __unpk8[1].clone();
    number = __unpk8[2].clone();
    counted = __unpk8[3].clone();
    shared = __unpk7[2].clone();
    many = __unpk7[3].clone();
    if py_truthy(&many.clone()) {
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You said "), &many.clone()), &py_str(".")), op_add(&op_add(&mcall(&slice_or_raise(&many.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&many.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" — that's what you told me."))]), memory.clone()]);
    }
    noun = if py_truthy(&counted.clone()) { _the_thing_counted(&[clean.clone(), said.clone()]) } else { py_str("") };
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { { let __b = noun.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PERIOD_WORD(), &mcall(&geti(&mcall(&noun.clone(), "split", &[]), &op_neg(&Py::Int(1))), "lower", &[]))); if !py_truthy(&__b) { __b } else { mcall(&g_A_RATE_IN_THE_FIGURE(), "search", &[said.clone()]) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = noun.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("1"), py_str("one")]), &mcall(&number.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&noun.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&noun.clone(), "endswith", &[py_str("ss")]))) } } } } } }) {
    noun = slice_or_raise(&noun.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    figure = if py_truthy(&noun.clone()) { mcall(&op_add(&op_add(&number.clone(), &py_str(" ")), &noun.clone()), "strip", &[]) } else { said.clone() };
    if py_truthy(&_the_ask_gave_that_figure(&[number.clone(), clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = noun.clone(); if !py_truthy(&__b) { __b } else { _a_count_of_one_with_a_plural(&[number.clone(), noun.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_figure_counts_the_thing(&[clean.clone(), figure.clone()])))) {
    return _the_turn_that_names_it(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&_a_bare_count(&[figure.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You said "), &figure.clone()), &py_str(".")), op_add(&figure.clone(), &py_str(", as you said."))]), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You said "), &figure.clone()), &py_str(".")), op_add(&op_add(&mcall(&slice_or_raise(&figure.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&figure.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" — that's what you told me."))]), memory.clone()]);
    Py::None
}
pub fn _the_store_keeps_this_figure(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut key_words: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said_on: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the store keeps a figure under the thing this ask names.\n\n    The figure reader is for the figures the store does not keep: it scans the lines the person\n    said and works out which one the ask is about. A figure the store holds under a key is a\n    different thing - the store's own recall says it, in its own words, and it is the record the\n    chat settled on. \"my budget is 500\" then \"change that to 650\" leaves 650 under \"budget\", and\n    the scan could only find the line that says the word, which is the one before the correction.\n    So where the ask names a key the store holds a figure under, this reading steps out of the way\n    and the store answers it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    asked = call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "keywords", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    value = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("value")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&value.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_FIGURE_WORD(), "search", &[value.clone()]))) } }) {
    continue;
    }
    key_words = pyrt::xrefs::xcall("memory.topics", "keywords", &[mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("key")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("_"), py_str(" ")])]);
    if py_truthy(&Py::Bool(!py_truthy(&key_words.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&key_words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&asked.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said_on = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = said_on.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_LINE_THAT_ASKS(), "search", &[said_on.clone()]) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_figure_beside_the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut figures: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figures") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figures"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut beside: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The figure the line says next to the thing the ask names, or None.\n\n    A line can give a figure for each of two things - \"i have two brothers and one sister\" - and\n    the first figure on the line is the wrong one to say back when the ask names the second:\n    \"how many sisters do i have\" was answered \"You said two sisters.\" A figure belongs to the\n    words it is said with, so the phrase the store kept beside the number is where the thing the\n    ask names is looked for.\n    ");
    let __iter1 = py_iter_or_raise(&figures.clone());
    for __item in __iter1 {
    figure = __item.clone();
    beside = pyrt::xrefs::xcall("memory.topics", "keywords", &[geti(&figure.clone(), &Py::Int(1))]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&beside.clone()) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return figure.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _every_figure_the_ask_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut wanted: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut kind: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut figure: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every figure the line gives for the things the ask names, said as one phrase, or \"\".\n\n    \"how many brothers and sisters do i have\" names two kinds of thing, and the line that holds\n    the answer - \"i have two brothers and one sister\" - gives a figure for each of them. Saying\n    the first one back (\"You said two brothers.\") drops the sister, which is half the answer and\n    wrong about the other half (deck row `how-many-02`). While the ask names more than one of\n    them and the line gives a figure for each, they are all said back, joined the way the person\n    joined them.\n    ");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_FRAME(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FIGURE_NOUNS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    figures = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_figures_in(&[line.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kind.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&one.clone(), &Py::Int(0)), &kind.clone())) } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    taken = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&named.clone());
    for __item in __iter1 {
    word = __item.clone();
    let __iter2 = py_iter_or_raise(&figures.clone());
    for __item in __iter2 {
    figure = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&taken.clone(), &figure.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[geti(&figure.clone(), &Py::Int(1))])) { let mut other: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&taken.clone(), "append", &[figure.clone()]);
    break;
    }
    }
    }
    if py_truthy(&op_lt(&call_builtin("len", &[taken.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&taken.clone()) { let mut figure: Py = __item.clone(); __out.push(geti(&figure.clone(), &Py::Int(1))); } py_list(__out) })() };
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&parts.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&parts.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn asked_back(args: &[Py]) -> Py {
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
    let mut _counts: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The figure a recall ask wants back, or None when the line is not one.\n\n    `the_figure_they_gave` is only reached where the composer has nothing, so a recall ask the\n    composer thinks it has answered was lost: \"how much was i putting away\" was met with \"I can't\n    look up live prices, sorry\" and \"what was my excess again\" with \"That is everything I have on\n    that one, so I would only repeat myself\" - while the 400 and the 100 were in the person's own\n    lines. The cue keeps a question about the world out: \"how much should i save each month\" is\n    not asking me what they said, and their 200 is not the answer to it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.recall", "looks_back", &[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_POINTS_BACK_HARD(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&pyrt::xrefs::xcall("memory.recall", "the_earlier_turn", &[text.clone(), memory.clone()]), &Py::None)) } }) {
    return Py::None;
    }
    figure = the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    let __try1 = exc::catch(|| {
    _counts = pyrt::xrefs::xmodule("memory.counts");
    said = mcall(&_counts.clone(), "answer", &[text.clone(), memory.clone()]);
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
    return { let __b = said.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn carries_the_figure(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut low: Py = Py::None;
    let mut number: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a reply already says the figure the person asked back.\n\n    A reply that names it answered the ask, whatever else it says; one that does not is the\n    composer making conversation about a question it did not answer, and the figure is the answer.\n    ");
    said = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    numbers = stdlib_call("re", "findall", &[py_str("\\d[\\d,]*"), call_builtin("str", &[{ let __b = figure.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&numbers.clone()) {
    let __iter1 = py_iter_or_raise(&numbers.clone());
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![\\d,.])"), &stdlib_call("re", "escape", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?!\\d)")), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[figure.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() };
    low = mcall(&said.clone(), "lower", &[]);
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
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
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut bare: Py = Py::None;
    let _ = py_str("Whether two lines are the same question in the same words.");
pub fn __nested_bare_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut bare: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut one: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut other: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "line").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
    bare = {{ let bare = bare.clone(); let one = one.clone(); let other = other.clone(); py_func("bare", move |args: &[Py]| { let mut full = vec![bare.clone(), one.clone(), other.clone()]; full.extend_from_slice(args); __nested_bare_0(&full) }) }};
    return { let __b = Py::Bool(py_eq(&py_call(&bare, &[one.clone()]), &py_call(&bare, &[other.clone()]))); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[mcall(&py_call(&bare, &[one.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } };
    Py::None
}
pub fn _shortened(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("The answer, kept whole up to a paragraph, so a repeat is not a wall of text again.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&op_le(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(320))) {
    return said.clone();
    }
    kept = py_str("");
    let __iter1 = py_iter_or_raise(&_sentences(&[said.clone()]));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { op_gt(&op_add(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(320)) } }) {
    break;
    }
    kept = mcall(&op_add(&op_add(&kept.clone(), &py_str(" ")), &part.clone()), "strip", &[]);
    }
    return { let __b = kept.clone(); if py_truthy(&__b) { __b } else { said.clone() } };
    Py::None
}
pub fn _the_answer_again(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut framed: Py = Py::None;
    let _ = py_str("The earlier answer, framed as the repeat it is, in the frames the chat already knows.\n\n    The wording is not this module's to invent: `topics.said_again` says an answer again in the\n    same words is what a machine does, frames it, and moves the lead-in on with each repeat - and\n    every frame in the codebase is listed in `again.FRAMES`, so a lead-in written here would make\n    the answer read as new words to every caller that asks whether the chat has said this already.\n    So the repeat is handed to that, and when the answer is further back than it looks the first\n    of the same three frames is used.\n    ");
    framed = pyrt::xrefs::xcall("memory.topics", "said_again", &[said.clone(), memory.clone()]);
    if py_truthy(&framed.clone()) {
    return framed.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reply".to_string(), _shortened(&[said.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&pyrt::xrefs::xget("memory.topics", "REPEAT"), &Py::Int(0)), "format", args))(&__args) };
    Py::None
}
pub fn the_same_question_again(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut between: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The answer this same question got earlier in the chat, or None.\n\n    See the note above: only the line's own words, only an answer that was an answer, only with a\n    turn in between, and never once the chat has changed the thing that answer was about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::None;
    }
    clean = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[slice_or_raise(&turns.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    back = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_same_words(&[clean.clone(), mcall(&turn.clone(), "get", &[py_str("user")])])))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_HEDGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    continue;
    }
    index = op_sub(&op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &back.clone());
    between = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_neg(&Py::Int(1)), &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = t.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&g_CHANGED_IT(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return _the_answer_again(&[said.clone(), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.repeat".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_BOT" => Some(g_ABOUT_THE_BOT()),
            "ACK_AND_REST" => Some(g_ACK_AND_REST()),
            "ACK_OPENS" => Some(g_ACK_OPENS()),
            "AGREEING" => Some(g_AGREEING()),
            "AGREEMENT_TAIL" => Some(g_AGREEMENT_TAIL()),
            "AGREES_IN_A_PHRASE" => Some(g_AGREES_IN_A_PHRASE()),
            "AGREES_ON_ITS_OWN" => Some(g_AGREES_ON_ITS_OWN()),
            "ALREADY_SHORT" => Some(g_ALREADY_SHORT()),
            "ALSO_LEADS" => Some(g_ALSO_LEADS()),
            "AND_NOW" => Some(g_AND_NOW()),
            "ANSWER_MODULES" => Some(g_ANSWER_MODULES()),
            "AN_ARTICLE" => Some(g_AN_ARTICLE()),
            "ASK" => Some(g_ASK()),
            "ASKING_THEM" => Some(g_ASKING_THEM()),
            "ASKING_TO_START" => Some(g_ASKING_TO_START()),
            "ASKS" => Some(g_ASKS()),
            "ASK_FRAME" => Some(g_ASK_FRAME()),
            "ASK_VERB" => Some(g_ASK_VERB()),
            "ATTEMPT" => Some(g_ATTEMPT()),
            "AT_A_LOSS" => Some(g_AT_A_LOSS()),
            "A_BARE_AMOUNT_ASK" => Some(g_A_BARE_AMOUNT_ASK()),
            "A_BARE_DURATION_ASK" => Some(g_A_BARE_DURATION_ASK()),
            "A_BARE_TIME" => Some(g_A_BARE_TIME()),
            "A_CORRECTION" => Some(g_A_CORRECTION()),
            "A_COUNT_OF_MORE_THAN_ONE" => Some(g_A_COUNT_OF_MORE_THAN_ONE()),
            "A_DOING_WORD" => Some(g_A_DOING_WORD()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_FIGURE_WORD" => Some(g_A_FIGURE_WORD()),
            "A_GAIN" => Some(g_A_GAIN()),
            "A_GOAL_LINE" => Some(g_A_GOAL_LINE()),
            "A_GROUP_WORD" => Some(g_A_GROUP_WORD()),
            "A_HAS_THE_LENGTH" => Some(g_A_HAS_THE_LENGTH()),
            "A_HOW_LONG_ITS_BEEN" => Some(g_A_HOW_LONG_ITS_BEEN()),
            "A_IM_ON_A_FIGURE" => Some(g_A_IM_ON_A_FIGURE()),
            "A_LENGTH_ASK" => Some(g_A_LENGTH_ASK()),
            "A_LINE_THAT_ASKS" => Some(g_A_LINE_THAT_ASKS()),
            "A_LIST_OF_AGES" => Some(g_A_LIST_OF_AGES()),
            "A_LOSS" => Some(g_A_LOSS()),
            "A_MARKER_AT_THE_END" => Some(g_A_MARKER_AT_THE_END()),
            "A_MEASURE_WORD" => Some(g_A_MEASURE_WORD()),
            "A_MENU" => Some(g_A_MENU()),
            "A_MOVE" => Some(g_A_MOVE()),
            "A_MOVE_WORDS" => Some(g_A_MOVE_WORDS()),
            "A_NARROWING_ONLY" => Some(g_A_NARROWING_ONLY()),
            "A_NARROWING_WORDS" => Some(g_A_NARROWING_WORDS()),
            "A_NUMBER_WORD" => Some(g_A_NUMBER_WORD()),
            "A_ONE_MORE" => Some(g_A_ONE_MORE()),
            "A_PAY_JUDGEMENT" => Some(g_A_PAY_JUDGEMENT()),
            "A_PAY_VERB_AT_THE_END" => Some(g_A_PAY_VERB_AT_THE_END()),
            "A_PAY_WORD" => Some(g_A_PAY_WORD()),
            "A_PLAN_ASK" => Some(g_A_PLAN_ASK()),
            "A_PLURAL_ONLY_NOUN" => Some(g_A_PLURAL_ONLY_NOUN()),
            "A_POINTS_BACK_HARD" => Some(g_A_POINTS_BACK_HARD()),
            "A_PRICE_ASK" => Some(g_A_PRICE_ASK()),
            "A_QUALITY" => Some(g_A_QUALITY()),
            "A_QUESTION" => Some(g_A_QUESTION()),
            "A_RATE_IN_THE_FIGURE" => Some(g_A_RATE_IN_THE_FIGURE()),
            "A_RATE_LINE" => Some(g_A_RATE_LINE()),
            "A_RATE_OF_TIME" => Some(g_A_RATE_OF_TIME()),
            "A_RECALL_CUE" => Some(g_A_RECALL_CUE()),
            "A_SCHEDULED_AFTER" => Some(g_A_SCHEDULED_AFTER()),
            "A_STORE_ANSWER" => Some(g_A_STORE_ANSWER()),
            "A_TAKE_HOME" => Some(g_A_TAKE_HOME()),
            "A_THE_PLAN_ASK" => Some(g_A_THE_PLAN_ASK()),
            "A_THING_NAMED_AFTER_THE_CUE" => Some(g_A_THING_NAMED_AFTER_THE_CUE()),
            "A_TOLD_BACK" => Some(g_A_TOLD_BACK()),
            "A_WERE_ASKS" => Some(g_A_WERE_ASKS()),
            "BARE_ASK" => Some(g_BARE_ASK()),
            "BARE_ASK_NEEDS_A_QUESTION" => Some(g_BARE_ASK_NEEDS_A_QUESTION()),
            "BE_FORM" => Some(g_BE_FORM()),
            "BREVITY_WORDS" => Some(g_BREVITY_WORDS()),
            "BROODING" => Some(g_BROODING()),
            "CHANGED_IT" => Some(g_CHANGED_IT()),
            "CHANGE_ASK" => Some(g_CHANGE_ASK()),
            "CHANGE_LEADS" => Some(g_CHANGE_LEADS()),
            "CLAIMS_IT_WAS_SAID" => Some(g_CLAIMS_IT_WAS_SAID()),
            "CLAUSE_SPLIT" => Some(g_CLAUSE_SPLIT()),
            "CLOSES_THE_TURN" => Some(g_CLOSES_THE_TURN()),
            "CONDITION_CLAUSE" => Some(g_CONDITION_CLAUSE()),
            "COUNT_ASK" => Some(g_COUNT_ASK()),
            "CUT_MID_SENTENCE" => Some(g_CUT_MID_SENTENCE()),
            "CUT_WORDS" => Some(g_CUT_WORDS()),
            "DANGLING" => Some(g_DANGLING()),
            "DECLARATIVE" => Some(g_DECLARATIVE()),
            "DIGIT" => Some(g_DIGIT()),
            "DOINGS" => Some(g_DOINGS()),
            "DONE" => Some(g_DONE()),
            "DO_I_NEED_TO" => Some(g_DO_I_NEED_TO()),
            "ECHO_FRAME" => Some(g_ECHO_FRAME()),
            "FIGURES" => Some(g_FIGURES()),
            "FIGURE_ASKS" => Some(g_FIGURE_ASKS()),
            "FIGURE_NOUNS" => Some(g_FIGURE_NOUNS()),
            "FIGURE_SAYING" => Some(g_FIGURE_SAYING()),
            "FIGURE_TAIL" => Some(g_FIGURE_TAIL()),
            "FIGURE_WORDS" => Some(g_FIGURE_WORDS()),
            "FILLER" => Some(g_FILLER()),
            "FIRST_THING" => Some(g_FIRST_THING()),
            "GAVE_ASKS" => Some(g_GAVE_ASKS()),
            "GAVE_LOOK_BACK" => Some(g_GAVE_LOOK_BACK()),
            "GOAL_NOUNS" => Some(g_GOAL_NOUNS()),
            "HEDGE" => Some(g_HEDGE()),
            "HELP_ASK_QUESTION_FIRST" => Some(g_HELP_ASK_QUESTION_FIRST()),
            "HELP_SHAPE" => Some(g_HELP_SHAPE()),
            "HOW_IT" => Some(g_HOW_IT()),
            "HOW_TO" => Some(g_HOW_TO()),
            "IDEAS_ASK" => Some(g_IDEAS_ASK()),
            "IDEAS_ONLY" => Some(g_IDEAS_ONLY()),
            "IRREGULAR_PAST" => Some(g_IRREGULAR_PAST()),
            "JOINERS" => Some(g_JOINERS()),
            "JUST_A_NOD" => Some(g_JUST_A_NOD()),
            "JUST_THIS" => Some(g_JUST_THIS()),
            "KEEPING" => Some(g_KEEPING()),
            "KEEP_CLAUSE" => Some(g_KEEP_CLAUSE()),
            "LEADING_PHRASE" => Some(g_LEADING_PHRASE()),
            "LEADING_PHRASE_MAX" => Some(g_LEADING_PHRASE_MAX()),
            "LEADS" => Some(g_LEADS()),
            "LEAD_IN" => Some(g_LEAD_IN()),
            "LEAD_SAYS_FIRST" => Some(g_LEAD_SAYS_FIRST()),
            "LEAD_SPLIT" => Some(g_LEAD_SPLIT()),
            "LENGTH_BACK" => Some(g_LENGTH_BACK()),
            "LESS" => Some(g_LESS()),
            "LESS_LEADS" => Some(g_LESS_LEADS()),
            "LESS_WORDS" => Some(g_LESS_WORDS()),
            "LIST_LEAD" => Some(g_LIST_LEAD()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "LOOK_BACK_FOR_LESS" => Some(g_LOOK_BACK_FOR_LESS()),
            "LOSS_WORDS" => Some(g_LOSS_WORDS()),
            "LOST_LEADS" => Some(g_LOST_LEADS()),
            "METHOD_ASKS" => Some(g_METHOD_ASKS()),
            "MORE_ASK" => Some(g_MORE_ASK()),
            "MORE_LEADS" => Some(g_MORE_LEADS()),
            "MOST" => Some(g_MOST()),
            "MOVE_ASK" => Some(g_MOVE_ASK()),
            "MOVE_LEADS" => Some(g_MOVE_LEADS()),
            "MOVE_VERBS" => Some(g_MOVE_VERBS()),
            "NAMED_AFTER_IT" => Some(g_NAMED_AFTER_IT()),
            "NEEDS_COMPLEMENT" => Some(g_NEEDS_COMPLEMENT()),
            "NEXT_ASK" => Some(g_NEXT_ASK()),
            "NEXT_LEADS" => Some(g_NEXT_LEADS()),
            "NEXT_STEP" => Some(g_NEXT_STEP()),
            "NEXT_STEP_EMERGENCY" => Some(g_NEXT_STEP_EMERGENCY()),
            "NOTHING_TO_CUT" => Some(g_NOTHING_TO_CUT()),
            "NOT_ALONE" => Some(g_NOT_ALONE()),
            "NOT_A_DOING" => Some(g_NOT_A_DOING()),
            "NOT_A_FIGURE_ASK" => Some(g_NOT_A_FIGURE_ASK()),
            "NOT_A_METHOD" => Some(g_NOT_A_METHOD()),
            "NOT_A_MOVE" => Some(g_NOT_A_MOVE()),
            "NOT_A_STEP" => Some(g_NOT_A_STEP()),
            "NOT_A_SUBJECT" => Some(g_NOT_A_SUBJECT()),
            "NOT_A_TAIL" => Some(g_NOT_A_TAIL()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_A_THING_ASKED" => Some(g_NOT_A_THING_ASKED()),
            "NOT_TRIED" => Some(g_NOT_TRIED()),
            "NOW_LEADS" => Some(g_NOW_LEADS()),
            "NO_LONGER_TO_DO" => Some(g_NO_LONGER_TO_DO()),
            "NO_SHORTER_STILL" => Some(g_NO_SHORTER_STILL()),
            "OBJECT_END" => Some(g_OBJECT_END()),
            "OPENERS" => Some(g_OPENERS()),
            "ORDER" => Some(g_ORDER()),
            "OWNED" => Some(g_OWNED()),
            "OWN_LEAD" => Some(g_OWN_LEAD()),
            "OWN_LEAD_WORDS" => Some(g_OWN_LEAD_WORDS()),
            "PART_ASKS" => Some(g_PART_ASKS()),
            "PART_OPENS" => Some(g_PART_OPENS()),
            "PART_STAND_INS" => Some(g_PART_STAND_INS()),
            "PAY_FAMILY" => Some(g_PAY_FAMILY()),
            "PAY_WHEN_WORDS" => Some(g_PAY_WHEN_WORDS()),
            "PERIOD_WORD" => Some(g_PERIOD_WORD()),
            "PIECE_END" => Some(g_PIECE_END()),
            "POINTING_AT" => Some(g_POINTING_AT()),
            "POINTING_BACK" => Some(g_POINTING_BACK()),
            "PRICE_WORDS" => Some(g_PRICE_WORDS()),
            "PUT_RIGHT" => Some(g_PUT_RIGHT()),
            "PUT_RIGHT_ASK" => Some(g_PUT_RIGHT_ASK()),
            "PUT_RIGHT_LEADS" => Some(g_PUT_RIGHT_LEADS()),
            "PUT_RIGHT_WORDS" => Some(g_PUT_RIGHT_WORDS()),
            "QUALIFIER" => Some(g_QUALIFIER()),
            "QUESTION" => Some(g_QUESTION()),
            "QUESTIONS_THAT_ARE_NOT_STEPS" => Some(g_QUESTIONS_THAT_ARE_NOT_STEPS()),
            "QUESTION_MORE" => Some(g_QUESTION_MORE()),
            "QUOTE_PAIRS" => Some(g_QUOTE_PAIRS()),
            "REFER_BACK" => Some(g_REFER_BACK()),
            "REJOINDER" => Some(g_REJOINDER()),
            "RESET_LEAD" => Some(g_RESET_LEAD()),
            "SAID_SAFE" => Some(g_SAID_SAFE()),
            "SAY_AGAIN_LEADS" => Some(g_SAY_AGAIN_LEADS()),
            "SELF" => Some(g_SELF()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SENTENCE_END" => Some(g_SENTENCE_END()),
            "SHORT_ENOUGH" => Some(g_SHORT_ENOUGH()),
            "SHORT_STEP" => Some(g_SHORT_STEP()),
            "SOMEONE_ELSES_LIMIT" => Some(g_SOMEONE_ELSES_LIMIT()),
            "STALL" => Some(g_STALL()),
            "STANDS_IN" => Some(g_STANDS_IN()),
            "STARTING_AT" => Some(g_STARTING_AT()),
            "STARTING_BACK" => Some(g_STARTING_BACK()),
            "STARTING_INTO" => Some(g_STARTING_INTO()),
            "STARTING_OUT" => Some(g_STARTING_OUT()),
            "STARTING_WHAT" => Some(g_STARTING_WHAT()),
            "START_ASK" => Some(g_START_ASK()),
            "STEP_SAYS_FIRST" => Some(g_STEP_SAYS_FIRST()),
            "STEP_VERBS" => Some(g_STEP_VERBS()),
            "STEP_WITH_ITS_METHOD" => Some(g_STEP_WITH_ITS_METHOD()),
            "STEP_WORDS" => Some(g_STEP_WORDS()),
            "STILL_ASKS" => Some(g_STILL_ASKS()),
            "STILL_SAFE" => Some(g_STILL_SAFE()),
            "THE_SAME_NUMBER" => Some(g_THE_SAME_NUMBER()),
            "TIME" => Some(g_TIME()),
            "TRIED" => Some(g_TRIED()),
            "TRIED_LEADS" => Some(g_TRIED_LEADS()),
            "TROUBLE" => Some(g_TROUBLE()),
            "USE_WITH" => Some(g_USE_WITH()),
            "WELCOMES" => Some(g_WELCOMES()),
            "WHAT_IT_IS" => Some(g_WHAT_IT_IS()),
            "WHERE_TO_LOOK" => Some(g_WHERE_TO_LOOK()),
            "WHERE_TO_LOOK_DEFAULT" => Some(g_WHERE_TO_LOOK_DEFAULT()),
            "WHERE_TO_LOOK_WORDS" => Some(g_WHERE_TO_LOOK_WORDS()),
            "WHERE_TO_START" => Some(g_WHERE_TO_START()),
            "WHETHER_ASKS" => Some(g_WHETHER_ASKS()),
            "WHOLE_OF_IT" => Some(g_WHOLE_OF_IT()),
            "WORDING" => Some(g_WORDING()),
            "WORDS_WANTED" => Some(g_WORDS_WANTED()),
            "_AFTER_A_NOUN" => Some(g__AFTER_A_NOUN()),
            "_AN_AMOUNT_WANTED" => Some(g__AN_AMOUNT_WANTED()),
            "_AN_ENTRY_IS_LOOKING" => Some(g__AN_ENTRY_IS_LOOKING()),
            "_A_AGE" => Some(g__A_AGE()),
            "_A_JUDGEMENT" => Some(g__A_JUDGEMENT()),
            "_A_STALL" => Some(g__A_STALL()),
            "_EXPLANATION_WORDS" => Some(g__EXPLANATION_WORDS()),
            "_FRAME_HEADS" => Some(g__FRAME_HEADS()),
            "_HANGS" => Some(g__HANGS()),
            "_MONEY_PAID_FOR" => Some(g__MONEY_PAID_FOR()),
            "_NEGATES" => Some(g__NEGATES()),
            "_OWNS_THE_THING" => Some(g__OWNS_THE_THING()),
            "_SO_COMES_THE_STEP" => Some(g__SO_COMES_THE_STEP()),
            "_SO_DOES_NOT_ASK_SOMETHING_OF_THEM" => Some(g__SO_DOES_NOT_ASK_SOMETHING_OF_THEM()),
            "_SO_THE_STEP_FOLLOWS" => Some(g__SO_THE_STEP_FOLLOWS()),
            "_SUBJECT_OPEN" => Some(g__SUBJECT_OPEN()),
            "_WEIGHS_ONE_AGAINST" => Some(g__WEIGHS_ONE_AGAINST()),
            "_a_bare_ask" => Some(py_func("_a_bare_ask", |args: &[Py]| _a_bare_ask(args))),
            "_a_bare_count" => Some(py_func("_a_bare_count", |args: &[Py]| _a_bare_count(args))),
            "_a_bare_time" => Some(py_func("_a_bare_time", |args: &[Py]| _a_bare_time(args))),
            "_a_city_guide_is_the_answer_in_front" => Some(py_func("_a_city_guide_is_the_answer_in_front", |args: &[Py]| _a_city_guide_is_the_answer_in_front(args))),
            "_a_conditional_move" => Some(py_func("_a_conditional_move", |args: &[Py]| _a_conditional_move(args))),
            "_a_count_of_one_with_a_plural" => Some(py_func("_a_count_of_one_with_a_plural", |args: &[Py]| _a_count_of_one_with_a_plural(args))),
            "_a_feeling" => Some(py_func("_a_feeling", |args: &[Py]| _a_feeling(args))),
            "_a_help_ask" => Some(py_func("_a_help_ask", |args: &[Py]| _a_help_ask(args))),
            "_a_lead_and_a_step" => Some(py_func("_a_lead_and_a_step", |args: &[Py]| _a_lead_and_a_step(args))),
            "_a_lead_for" => Some(py_func("_a_lead_for", |args: &[Py]| _a_lead_for(args))),
            "_a_lead_of_its_own" => Some(py_func("_a_lead_of_its_own", |args: &[Py]| _a_lead_of_its_own(args))),
            "_a_length_already_handed" => Some(py_func("_a_length_already_handed", |args: &[Py]| _a_length_already_handed(args))),
            "_a_line_that_corrects" => Some(py_func("_a_line_that_corrects", |args: &[Py]| _a_line_that_corrects(args))),
            "_a_list_item" => Some(py_func("_a_list_item", |args: &[Py]| _a_list_item(args))),
            "_a_list_of_ages" => Some(py_func("_a_list_of_ages", |args: &[Py]| _a_list_of_ages(args))),
            "_a_move" => Some(py_func("_a_move", |args: &[Py]| _a_move(args))),
            "_a_move_in" => Some(py_func("_a_move_in", |args: &[Py]| _a_move_in(args))),
            "_a_new_work" => Some(py_func("_a_new_work", |args: &[Py]| _a_new_work(args))),
            "_a_question" => Some(py_func("_a_question", |args: &[Py]| _a_question(args))),
            "_a_stem" => Some(py_func("_a_stem", |args: &[Py]| _a_stem(args))),
            "_a_step_comes_after" => Some(py_func("_a_step_comes_after", |args: &[Py]| _a_step_comes_after(args))),
            "_a_step_done" => Some(py_func("_a_step_done", |args: &[Py]| _a_step_done(args))),
            "_a_thing_of_its_own" => Some(py_func("_a_thing_of_its_own", |args: &[Py]| _a_thing_of_its_own(args))),
            "_a_verb_after_a_leading_phrase" => Some(py_func("_a_verb_after_a_leading_phrase", |args: &[Py]| _a_verb_after_a_leading_phrase(args))),
            "_a_what_to_do_answer" => Some(py_func("_a_what_to_do_answer", |args: &[Py]| _a_what_to_do_answer(args))),
            "_about_the_same_thing" => Some(py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args))),
            "_agreed" => Some(py_func("_agreed", |args: &[Py]| _agreed(args))),
            "_already_repeated" => Some(py_func("_already_repeated", |args: &[Py]| _already_repeated(args))),
            "_an_echo" => Some(py_func("_an_echo", |args: &[Py]| _an_echo(args))),
            "_an_entry_answers_it" => Some(py_func("_an_entry_answers_it", |args: &[Py]| _an_entry_answers_it(args))),
            "_another_part" => Some(py_func("_another_part", |args: &[Py]| _another_part(args))),
            "_claims_what_it_never_said" => Some(py_func("_claims_what_it_never_said", |args: &[Py]| _claims_what_it_never_said(args))),
            "_closing_the_quote" => Some(py_func("_closing_the_quote", |args: &[Py]| _closing_the_quote(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_doings" => Some(py_func("_doings", |args: &[Py]| _doings(args))),
            "_ended" => Some(py_func("_ended", |args: &[Py]| _ended(args))),
            "_every_figure_the_ask_names" => Some(py_func("_every_figure_the_ask_names", |args: &[Py]| _every_figure_the_ask_names(args))),
            "_explains" => Some(py_func("_explains", |args: &[Py]| _explains(args))),
            "_figures_in" => Some(py_func("_figures_in", |args: &[Py]| _figures_in(args))),
            "_frame_heads" => Some(py_func("_frame_heads", |args: &[Py]| _frame_heads(args))),
            "_handed_back" => Some(py_func("_handed_back", |args: &[Py]| _handed_back(args))),
            "_handed_back_already" => Some(py_func("_handed_back_already", |args: &[Py]| _handed_back_already(args))),
            "_handed_back_last" => Some(py_func("_handed_back_last", |args: &[Py]| _handed_back_last(args))),
            "_inside_a_bracket" => Some(py_func("_inside_a_bracket", |args: &[Py]| _inside_a_bracket(args))),
            "_keeps" => Some(py_func("_keeps", |args: &[Py]| _keeps(args))),
            "_kept_clause" => Some(py_func("_kept_clause", |args: &[Py]| _kept_clause(args))),
            "_later_part" => Some(py_func("_later_part", |args: &[Py]| _later_part(args))),
            "_leads_for" => Some(py_func("_leads_for", |args: &[Py]| _leads_for(args))),
            "_money_paid_for_a_thing" => Some(py_func("_money_paid_for_a_thing", |args: &[Py]| _money_paid_for_a_thing(args))),
            "_most_condensed" => Some(py_func("_most_condensed", |args: &[Py]| _most_condensed(args))),
            "_names_a_thing_of_its_own" => Some(py_func("_names_a_thing_of_its_own", |args: &[Py]| _names_a_thing_of_its_own(args))),
            "_names_no_thing_of_its_own" => Some(py_func("_names_no_thing_of_its_own", |args: &[Py]| _names_no_thing_of_its_own(args))),
            "_names_no_topic" => Some(py_func("_names_no_topic", |args: &[Py]| _names_no_topic(args))),
            "_next_step" => Some(py_func("_next_step", |args: &[Py]| _next_step(args))),
            "_nothing_but_acknowledgements" => Some(py_func("_nothing_but_acknowledgements", |args: &[Py]| _nothing_but_acknowledgements(args))),
            "_one_figure" => Some(py_func("_one_figure", |args: &[Py]| _one_figure(args))),
            "_only_echoes_between" => Some(py_func("_only_echoes_between", |args: &[Py]| _only_echoes_between(args))),
            "_opening_move" => Some(py_func("_opening_move", |args: &[Py]| _opening_move(args))),
            "_opens_a_doing" => Some(py_func("_opens_a_doing", |args: &[Py]| _opens_a_doing(args))),
            "_opens_an_instruction" => Some(py_func("_opens_an_instruction", |args: &[Py]| _opens_an_instruction(args))),
            "_opens_on_a_verb" => Some(py_func("_opens_on_a_verb", |args: &[Py]| _opens_on_a_verb(args))),
            "_pieces" => Some(py_func("_pieces", |args: &[Py]| _pieces(args))),
            "_places_to_cut" => Some(py_func("_places_to_cut", |args: &[Py]| _places_to_cut(args))),
            "_read_up_to_the_colon" => Some(py_func("_read_up_to_the_colon", |args: &[Py]| _read_up_to_the_colon(args))),
            "_recent_answers" => Some(py_func("_recent_answers", |args: &[Py]| _recent_answers(args))),
            "_recent_lines" => Some(py_func("_recent_lines", |args: &[Py]| _recent_lines(args))),
            "_said_in_a_turn" => Some(py_func("_said_in_a_turn", |args: &[Py]| _said_in_a_turn(args))),
            "_said_in_the_chat" => Some(py_func("_said_in_the_chat", |args: &[Py]| _said_in_the_chat(args))),
            "_says_they_agree" => Some(py_func("_says_they_agree", |args: &[Py]| _says_they_agree(args))),
            "_says_what_to_do" => Some(py_func("_says_what_to_do", |args: &[Py]| _says_what_to_do(args))),
            "_sentences" => Some(py_func("_sentences", |args: &[Py]| _sentences(args))),
            "_short" => Some(py_func("_short", |args: &[Py]| _short(args))),
            "_short_enough" => Some(py_func("_short_enough", |args: &[Py]| _short_enough(args))),
            "_shortened" => Some(py_func("_shortened", |args: &[Py]| _shortened(args))),
            "_something_gone" => Some(py_func("_something_gone", |args: &[Py]| _something_gone(args))),
            "_starting_out_names_a_thing" => Some(py_func("_starting_out_names_a_thing", |args: &[Py]| _starting_out_names_a_thing(args))),
            "_the_act" => Some(py_func("_the_act", |args: &[Py]| _the_act(args))),
            "_the_advice" => Some(py_func("_the_advice", |args: &[Py]| _the_advice(args))),
            "_the_answer_about_what_we_are_on" => Some(py_func("_the_answer_about_what_we_are_on", |args: &[Py]| _the_answer_about_what_we_are_on(args))),
            "_the_answer_again" => Some(py_func("_the_answer_again", |args: &[Py]| _the_answer_again(args))),
            "_the_answer_gives" => Some(py_func("_the_answer_gives", |args: &[Py]| _the_answer_gives(args))),
            "_the_ask_gave_that_figure" => Some(py_func("_the_ask_gave_that_figure", |args: &[Py]| _the_ask_gave_that_figure(args))),
            "_the_chat_is_still_on" => Some(py_func("_the_chat_is_still_on", |args: &[Py]| _the_chat_is_still_on(args))),
            "_the_chat_is_still_on__dup1" => Some(py_func("_the_chat_is_still_on__dup1", |args: &[Py]| _the_chat_is_still_on__dup1(args))),
            "_the_colon_tail_kept" => Some(py_func("_the_colon_tail_kept", |args: &[Py]| _the_colon_tail_kept(args))),
            "_the_corrected" => Some(py_func("_the_corrected", |args: &[Py]| _the_corrected(args))),
            "_the_doing" => Some(py_func("_the_doing", |args: &[Py]| _the_doing(args))),
            "_the_figure_beside_the_thing" => Some(py_func("_the_figure_beside_the_thing", |args: &[Py]| _the_figure_beside_the_thing(args))),
            "_the_figure_counts_the_thing" => Some(py_func("_the_figure_counts_the_thing", |args: &[Py]| _the_figure_counts_the_thing(args))),
            "_the_figure_of_a_goal" => Some(py_func("_the_figure_of_a_goal", |args: &[Py]| _the_figure_of_a_goal(args))),
            "_the_first_instruction_in_a_long_piece" => Some(py_func("_the_first_instruction_in_a_long_piece", |args: &[Py]| _the_first_instruction_in_a_long_piece(args))),
            "_the_first_step" => Some(py_func("_the_first_step", |args: &[Py]| _the_first_step(args))),
            "_the_keys_of" => Some(py_func("_the_keys_of", |args: &[Py]| _the_keys_of(args))),
            "_the_lead_for" => Some(py_func("_the_lead_for", |args: &[Py]| _the_lead_for(args))),
            "_the_length_they_mean" => Some(py_func("_the_length_they_mean", |args: &[Py]| _the_length_they_mean(args))),
            "_the_next_step" => Some(py_func("_the_next_step", |args: &[Py]| _the_next_step(args))),
            "_the_object" => Some(py_func("_the_object", |args: &[Py]| _the_object(args))),
            "_the_price_in_hand" => Some(py_func("_the_price_in_hand", |args: &[Py]| _the_price_in_hand(args))),
            "_the_price_the_answer_gave" => Some(py_func("_the_price_the_answer_gave", |args: &[Py]| _the_price_the_answer_gave(args))),
            "_the_quotation" => Some(py_func("_the_quotation", |args: &[Py]| _the_quotation(args))),
            "_the_same_word" => Some(py_func("_the_same_word", |args: &[Py]| _the_same_word(args))),
            "_the_same_words" => Some(py_func("_the_same_words", |args: &[Py]| _the_same_words(args))),
            "_the_stems" => Some(py_func("_the_stems", |args: &[Py]| _the_stems(args))),
            "_the_step_just_given" => Some(py_func("_the_step_just_given", |args: &[Py]| _the_step_just_given(args))),
            "_the_steps" => Some(py_func("_the_steps", |args: &[Py]| _the_steps(args))),
            "_the_steps_done" => Some(py_func("_the_steps_done", |args: &[Py]| _the_steps_done(args))),
            "_the_store_keeps_this_figure" => Some(py_func("_the_store_keeps_this_figure", |args: &[Py]| _the_store_keeps_this_figure(args))),
            "_the_tail_asks_a_question" => Some(py_func("_the_tail_asks_a_question", |args: &[Py]| _the_tail_asks_a_question(args))),
            "_the_thing_counted" => Some(py_func("_the_thing_counted", |args: &[Py]| _the_thing_counted(args))),
            "_the_thing_they_named" => Some(py_func("_the_thing_they_named", |args: &[Py]| _the_thing_they_named(args))),
            "_the_time_an_earlier_answer_gave" => Some(py_func("_the_time_an_earlier_answer_gave", |args: &[Py]| _the_time_an_earlier_answer_gave(args))),
            "_the_turn_that_names_it" => Some(py_func("_the_turn_that_names_it", |args: &[Py]| _the_turn_that_names_it(args))),
            "_the_untried" => Some(py_func("_the_untried", |args: &[Py]| _the_untried(args))),
            "_they_say_they_tried" => Some(py_func("_they_say_they_tried", |args: &[Py]| _they_say_they_tried(args))),
            "_tried" => Some(py_func("_tried", |args: &[Py]| _tried(args))),
            "_trouble" => Some(py_func("_trouble", |args: &[Py]| _trouble(args))),
            "_unbalanced" => Some(py_func("_unbalanced", |args: &[Py]| _unbalanced(args))),
            "_where_the_rest_is" => Some(py_func("_where_the_rest_is", |args: &[Py]| _where_the_rest_is(args))),
            "_whether_to" => Some(py_func("_whether_to", |args: &[Py]| _whether_to(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "a_hand_back" => Some(py_func("a_hand_back", |args: &[Py]| a_hand_back(args))),
            "a_more_ask" => Some(py_func("a_more_ask", |args: &[Py]| a_more_ask(args))),
            "a_narrowing_and_its_line" => Some(py_func("a_narrowing_and_its_line", |args: &[Py]| a_narrowing_and_its_line(args))),
            "a_step_of_the_same_answer" => Some(py_func("a_step_of_the_same_answer", |args: &[Py]| a_step_of_the_same_answer(args))),
            "a_whole_cut" => Some(g_a_whole_cut()),
            "after_an_ack" => Some(py_func("after_an_ack", |args: &[Py]| after_an_ack(args))),
            "agreement_with_the_answer" => Some(py_func("agreement_with_the_answer", |args: &[Py]| agreement_with_the_answer(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "answers_nothing" => Some(py_func("answers_nothing", |args: &[Py]| answers_nothing(args))),
            "asked_back" => Some(py_func("asked_back", |args: &[Py]| asked_back(args))),
            "asks_for_less" => Some(py_func("asks_for_less", |args: &[Py]| asks_for_less(args))),
            "asks_for_more" => Some(py_func("asks_for_more", |args: &[Py]| asks_for_more(args))),
            "asks_for_the_move" => Some(py_func("asks_for_the_move", |args: &[Py]| asks_for_the_move(args))),
            "asks_for_the_same_work" => Some(py_func("asks_for_the_same_work", |args: &[Py]| asks_for_the_same_work(args))),
            "asks_to_put_it_right" => Some(py_func("asks_to_put_it_right", |args: &[Py]| asks_to_put_it_right(args))),
            "asks_what_that_costs" => Some(py_func("asks_what_that_costs", |args: &[Py]| asks_what_that_costs(args))),
            "asks_which_part" => Some(py_func("asks_which_part", |args: &[Py]| asks_which_part(args))),
            "carried_on" => Some(py_func("carried_on", |args: &[Py]| carried_on(args))),
            "carries_the_figure" => Some(py_func("carries_the_figure", |args: &[Py]| carries_the_figure(args))),
            "framed" => Some(py_func("framed", |args: &[Py]| framed(args))),
            "handed_back" => Some(g_handed_back()),
            "how_long_its_been" => Some(py_func("how_long_its_been", |args: &[Py]| how_long_its_been(args))),
            "names_nothing" => Some(py_func("names_nothing", |args: &[Py]| names_nothing(args))),
            "only_said_it_back" => Some(py_func("only_said_it_back", |args: &[Py]| only_said_it_back(args))),
            "pick" => Some(g_pick()),
            "recent_answers" => Some(g_recent_answers()),
            "said_recently" => Some(g_said_recently()),
            "same_subject" => Some(g_same_subject()),
            "says_where_to_start" => Some(py_func("says_where_to_start", |args: &[Py]| says_where_to_start(args))),
            "shorter_for" => Some(py_func("shorter_for", |args: &[Py]| shorter_for(args))),
            "the_advice_was_to_get_seen" => Some(py_func("the_advice_was_to_get_seen", |args: &[Py]| the_advice_was_to_get_seen(args))),
            "the_figure_asked" => Some(py_func("the_figure_asked", |args: &[Py]| the_figure_asked(args))),
            "the_figure_they_gave" => Some(py_func("the_figure_they_gave", |args: &[Py]| the_figure_they_gave(args))),
            "the_move" => Some(py_func("the_move", |args: &[Py]| the_move(args))),
            "the_next_move" => Some(py_func("the_next_move", |args: &[Py]| the_next_move(args))),
            "the_part_asked" => Some(py_func("the_part_asked", |args: &[Py]| the_part_asked(args))),
            "the_plain_line" => Some(py_func("the_plain_line", |args: &[Py]| the_plain_line(args))),
            "the_same_question_again" => Some(py_func("the_same_question_again", |args: &[Py]| the_same_question_again(args))),
            "understand" => Some(g_understand()),
            "what_it_knows" => Some(py_func("what_it_knows", |args: &[Py]| what_it_knows(args))),
            "without_a_reset" => Some(py_func("without_a_reset", |args: &[Py]| without_a_reset(args))),
            "counts" => Some(pyrt::xrefs::xmodule("memory.counts")),
            "facts" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "history" => Some(pyrt::xrefs::xmodule("memory.history")),
            "limits" => Some(pyrt::xrefs::xmodule("memory.limits")),
            "narrow" => Some(pyrt::xrefs::xmodule("memory.narrow")),
            "recall" => Some(pyrt::xrefs::xmodule("memory.recall")),
            "topics" => Some(pyrt::xrefs::xmodule("memory.topics")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THE_BOT" => s_ABOUT_THE_BOT(v),
            "ACK_AND_REST" => s_ACK_AND_REST(v),
            "ACK_OPENS" => s_ACK_OPENS(v),
            "AGREEING" => s_AGREEING(v),
            "AGREEMENT_TAIL" => s_AGREEMENT_TAIL(v),
            "AGREES_IN_A_PHRASE" => s_AGREES_IN_A_PHRASE(v),
            "AGREES_ON_ITS_OWN" => s_AGREES_ON_ITS_OWN(v),
            "ALREADY_SHORT" => s_ALREADY_SHORT(v),
            "ALSO_LEADS" => s_ALSO_LEADS(v),
            "AND_NOW" => s_AND_NOW(v),
            "ANSWER_MODULES" => s_ANSWER_MODULES(v),
            "AN_ARTICLE" => s_AN_ARTICLE(v),
            "ASK" => s_ASK(v),
            "ASKING_THEM" => s_ASKING_THEM(v),
            "ASKING_TO_START" => s_ASKING_TO_START(v),
            "ASKS" => s_ASKS(v),
            "ASK_FRAME" => s_ASK_FRAME(v),
            "ASK_VERB" => s_ASK_VERB(v),
            "ATTEMPT" => s_ATTEMPT(v),
            "AT_A_LOSS" => s_AT_A_LOSS(v),
            "A_BARE_AMOUNT_ASK" => s_A_BARE_AMOUNT_ASK(v),
            "A_BARE_DURATION_ASK" => s_A_BARE_DURATION_ASK(v),
            "A_BARE_TIME" => s_A_BARE_TIME(v),
            "A_CORRECTION" => s_A_CORRECTION(v),
            "A_COUNT_OF_MORE_THAN_ONE" => s_A_COUNT_OF_MORE_THAN_ONE(v),
            "A_DOING_WORD" => s_A_DOING_WORD(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_FIGURE_WORD" => s_A_FIGURE_WORD(v),
            "A_GAIN" => s_A_GAIN(v),
            "A_GOAL_LINE" => s_A_GOAL_LINE(v),
            "A_GROUP_WORD" => s_A_GROUP_WORD(v),
            "A_HAS_THE_LENGTH" => s_A_HAS_THE_LENGTH(v),
            "A_HOW_LONG_ITS_BEEN" => s_A_HOW_LONG_ITS_BEEN(v),
            "A_IM_ON_A_FIGURE" => s_A_IM_ON_A_FIGURE(v),
            "A_LENGTH_ASK" => s_A_LENGTH_ASK(v),
            "A_LINE_THAT_ASKS" => s_A_LINE_THAT_ASKS(v),
            "A_LIST_OF_AGES" => s_A_LIST_OF_AGES(v),
            "A_LOSS" => s_A_LOSS(v),
            "A_MARKER_AT_THE_END" => s_A_MARKER_AT_THE_END(v),
            "A_MEASURE_WORD" => s_A_MEASURE_WORD(v),
            "A_MENU" => s_A_MENU(v),
            "A_MOVE" => s_A_MOVE(v),
            "A_MOVE_WORDS" => s_A_MOVE_WORDS(v),
            "A_NARROWING_ONLY" => s_A_NARROWING_ONLY(v),
            "A_NARROWING_WORDS" => s_A_NARROWING_WORDS(v),
            "A_NUMBER_WORD" => s_A_NUMBER_WORD(v),
            "A_ONE_MORE" => s_A_ONE_MORE(v),
            "A_PAY_JUDGEMENT" => s_A_PAY_JUDGEMENT(v),
            "A_PAY_VERB_AT_THE_END" => s_A_PAY_VERB_AT_THE_END(v),
            "A_PAY_WORD" => s_A_PAY_WORD(v),
            "A_PLAN_ASK" => s_A_PLAN_ASK(v),
            "A_PLURAL_ONLY_NOUN" => s_A_PLURAL_ONLY_NOUN(v),
            "A_POINTS_BACK_HARD" => s_A_POINTS_BACK_HARD(v),
            "A_PRICE_ASK" => s_A_PRICE_ASK(v),
            "A_QUALITY" => s_A_QUALITY(v),
            "A_QUESTION" => s_A_QUESTION(v),
            "A_RATE_IN_THE_FIGURE" => s_A_RATE_IN_THE_FIGURE(v),
            "A_RATE_LINE" => s_A_RATE_LINE(v),
            "A_RATE_OF_TIME" => s_A_RATE_OF_TIME(v),
            "A_RECALL_CUE" => s_A_RECALL_CUE(v),
            "A_SCHEDULED_AFTER" => s_A_SCHEDULED_AFTER(v),
            "A_STORE_ANSWER" => s_A_STORE_ANSWER(v),
            "A_TAKE_HOME" => s_A_TAKE_HOME(v),
            "A_THE_PLAN_ASK" => s_A_THE_PLAN_ASK(v),
            "A_THING_NAMED_AFTER_THE_CUE" => s_A_THING_NAMED_AFTER_THE_CUE(v),
            "A_TOLD_BACK" => s_A_TOLD_BACK(v),
            "A_WERE_ASKS" => s_A_WERE_ASKS(v),
            "BARE_ASK" => s_BARE_ASK(v),
            "BARE_ASK_NEEDS_A_QUESTION" => s_BARE_ASK_NEEDS_A_QUESTION(v),
            "BE_FORM" => s_BE_FORM(v),
            "BREVITY_WORDS" => s_BREVITY_WORDS(v),
            "BROODING" => s_BROODING(v),
            "CHANGED_IT" => s_CHANGED_IT(v),
            "CHANGE_ASK" => s_CHANGE_ASK(v),
            "CHANGE_LEADS" => s_CHANGE_LEADS(v),
            "CLAIMS_IT_WAS_SAID" => s_CLAIMS_IT_WAS_SAID(v),
            "CLAUSE_SPLIT" => s_CLAUSE_SPLIT(v),
            "CLOSES_THE_TURN" => s_CLOSES_THE_TURN(v),
            "CONDITION_CLAUSE" => s_CONDITION_CLAUSE(v),
            "COUNT_ASK" => s_COUNT_ASK(v),
            "CUT_MID_SENTENCE" => s_CUT_MID_SENTENCE(v),
            "CUT_WORDS" => s_CUT_WORDS(v),
            "DANGLING" => s_DANGLING(v),
            "DECLARATIVE" => s_DECLARATIVE(v),
            "DIGIT" => s_DIGIT(v),
            "DOINGS" => s_DOINGS(v),
            "DONE" => s_DONE(v),
            "DO_I_NEED_TO" => s_DO_I_NEED_TO(v),
            "ECHO_FRAME" => s_ECHO_FRAME(v),
            "FIGURES" => s_FIGURES(v),
            "FIGURE_ASKS" => s_FIGURE_ASKS(v),
            "FIGURE_NOUNS" => s_FIGURE_NOUNS(v),
            "FIGURE_SAYING" => s_FIGURE_SAYING(v),
            "FIGURE_TAIL" => s_FIGURE_TAIL(v),
            "FIGURE_WORDS" => s_FIGURE_WORDS(v),
            "FILLER" => s_FILLER(v),
            "FIRST_THING" => s_FIRST_THING(v),
            "GAVE_ASKS" => s_GAVE_ASKS(v),
            "GAVE_LOOK_BACK" => s_GAVE_LOOK_BACK(v),
            "GOAL_NOUNS" => s_GOAL_NOUNS(v),
            "HEDGE" => s_HEDGE(v),
            "HELP_ASK_QUESTION_FIRST" => s_HELP_ASK_QUESTION_FIRST(v),
            "HELP_SHAPE" => s_HELP_SHAPE(v),
            "HOW_IT" => s_HOW_IT(v),
            "HOW_TO" => s_HOW_TO(v),
            "IDEAS_ASK" => s_IDEAS_ASK(v),
            "IDEAS_ONLY" => s_IDEAS_ONLY(v),
            "IRREGULAR_PAST" => s_IRREGULAR_PAST(v),
            "JOINERS" => s_JOINERS(v),
            "JUST_A_NOD" => s_JUST_A_NOD(v),
            "JUST_THIS" => s_JUST_THIS(v),
            "KEEPING" => s_KEEPING(v),
            "KEEP_CLAUSE" => s_KEEP_CLAUSE(v),
            "LEADING_PHRASE" => s_LEADING_PHRASE(v),
            "LEADING_PHRASE_MAX" => s_LEADING_PHRASE_MAX(v),
            "LEADS" => s_LEADS(v),
            "LEAD_IN" => s_LEAD_IN(v),
            "LEAD_SAYS_FIRST" => s_LEAD_SAYS_FIRST(v),
            "LEAD_SPLIT" => s_LEAD_SPLIT(v),
            "LENGTH_BACK" => s_LENGTH_BACK(v),
            "LESS" => s_LESS(v),
            "LESS_LEADS" => s_LESS_LEADS(v),
            "LESS_WORDS" => s_LESS_WORDS(v),
            "LIST_LEAD" => s_LIST_LEAD(v),
            "LOOK_BACK" => s_LOOK_BACK(v),
            "LOOK_BACK_FOR_LESS" => s_LOOK_BACK_FOR_LESS(v),
            "LOSS_WORDS" => s_LOSS_WORDS(v),
            "LOST_LEADS" => s_LOST_LEADS(v),
            "METHOD_ASKS" => s_METHOD_ASKS(v),
            "MORE_ASK" => s_MORE_ASK(v),
            "MORE_LEADS" => s_MORE_LEADS(v),
            "MOST" => s_MOST(v),
            "MOVE_ASK" => s_MOVE_ASK(v),
            "MOVE_LEADS" => s_MOVE_LEADS(v),
            "MOVE_VERBS" => s_MOVE_VERBS(v),
            "NAMED_AFTER_IT" => s_NAMED_AFTER_IT(v),
            "NEEDS_COMPLEMENT" => s_NEEDS_COMPLEMENT(v),
            "NEXT_ASK" => s_NEXT_ASK(v),
            "NEXT_LEADS" => s_NEXT_LEADS(v),
            "NEXT_STEP" => s_NEXT_STEP(v),
            "NEXT_STEP_EMERGENCY" => s_NEXT_STEP_EMERGENCY(v),
            "NOTHING_TO_CUT" => s_NOTHING_TO_CUT(v),
            "NOT_ALONE" => s_NOT_ALONE(v),
            "NOT_A_DOING" => s_NOT_A_DOING(v),
            "NOT_A_FIGURE_ASK" => s_NOT_A_FIGURE_ASK(v),
            "NOT_A_METHOD" => s_NOT_A_METHOD(v),
            "NOT_A_MOVE" => s_NOT_A_MOVE(v),
            "NOT_A_STEP" => s_NOT_A_STEP(v),
            "NOT_A_SUBJECT" => s_NOT_A_SUBJECT(v),
            "NOT_A_TAIL" => s_NOT_A_TAIL(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_A_THING_ASKED" => s_NOT_A_THING_ASKED(v),
            "NOT_TRIED" => s_NOT_TRIED(v),
            "NOW_LEADS" => s_NOW_LEADS(v),
            "NO_LONGER_TO_DO" => s_NO_LONGER_TO_DO(v),
            "NO_SHORTER_STILL" => s_NO_SHORTER_STILL(v),
            "OBJECT_END" => s_OBJECT_END(v),
            "OPENERS" => s_OPENERS(v),
            "ORDER" => s_ORDER(v),
            "OWNED" => s_OWNED(v),
            "OWN_LEAD" => s_OWN_LEAD(v),
            "OWN_LEAD_WORDS" => s_OWN_LEAD_WORDS(v),
            "PART_ASKS" => s_PART_ASKS(v),
            "PART_OPENS" => s_PART_OPENS(v),
            "PART_STAND_INS" => s_PART_STAND_INS(v),
            "PAY_FAMILY" => s_PAY_FAMILY(v),
            "PAY_WHEN_WORDS" => s_PAY_WHEN_WORDS(v),
            "PERIOD_WORD" => s_PERIOD_WORD(v),
            "PIECE_END" => s_PIECE_END(v),
            "POINTING_AT" => s_POINTING_AT(v),
            "POINTING_BACK" => s_POINTING_BACK(v),
            "PRICE_WORDS" => s_PRICE_WORDS(v),
            "PUT_RIGHT" => s_PUT_RIGHT(v),
            "PUT_RIGHT_ASK" => s_PUT_RIGHT_ASK(v),
            "PUT_RIGHT_LEADS" => s_PUT_RIGHT_LEADS(v),
            "PUT_RIGHT_WORDS" => s_PUT_RIGHT_WORDS(v),
            "QUALIFIER" => s_QUALIFIER(v),
            "QUESTION" => s_QUESTION(v),
            "QUESTIONS_THAT_ARE_NOT_STEPS" => s_QUESTIONS_THAT_ARE_NOT_STEPS(v),
            "QUESTION_MORE" => s_QUESTION_MORE(v),
            "QUOTE_PAIRS" => s_QUOTE_PAIRS(v),
            "REFER_BACK" => s_REFER_BACK(v),
            "REJOINDER" => s_REJOINDER(v),
            "RESET_LEAD" => s_RESET_LEAD(v),
            "SAID_SAFE" => s_SAID_SAFE(v),
            "SAY_AGAIN_LEADS" => s_SAY_AGAIN_LEADS(v),
            "SELF" => s_SELF(v),
            "SENTENCE" => s_SENTENCE(v),
            "SENTENCE_END" => s_SENTENCE_END(v),
            "SHORT_ENOUGH" => s_SHORT_ENOUGH(v),
            "SHORT_STEP" => s_SHORT_STEP(v),
            "SOMEONE_ELSES_LIMIT" => s_SOMEONE_ELSES_LIMIT(v),
            "STALL" => s_STALL(v),
            "STANDS_IN" => s_STANDS_IN(v),
            "STARTING_AT" => s_STARTING_AT(v),
            "STARTING_BACK" => s_STARTING_BACK(v),
            "STARTING_INTO" => s_STARTING_INTO(v),
            "STARTING_OUT" => s_STARTING_OUT(v),
            "STARTING_WHAT" => s_STARTING_WHAT(v),
            "START_ASK" => s_START_ASK(v),
            "STEP_SAYS_FIRST" => s_STEP_SAYS_FIRST(v),
            "STEP_VERBS" => s_STEP_VERBS(v),
            "STEP_WITH_ITS_METHOD" => s_STEP_WITH_ITS_METHOD(v),
            "STEP_WORDS" => s_STEP_WORDS(v),
            "STILL_ASKS" => s_STILL_ASKS(v),
            "STILL_SAFE" => s_STILL_SAFE(v),
            "THE_SAME_NUMBER" => s_THE_SAME_NUMBER(v),
            "TIME" => s_TIME(v),
            "TRIED" => s_TRIED(v),
            "TRIED_LEADS" => s_TRIED_LEADS(v),
            "TROUBLE" => s_TROUBLE(v),
            "USE_WITH" => s_USE_WITH(v),
            "WELCOMES" => s_WELCOMES(v),
            "WHAT_IT_IS" => s_WHAT_IT_IS(v),
            "WHERE_TO_LOOK" => s_WHERE_TO_LOOK(v),
            "WHERE_TO_LOOK_DEFAULT" => s_WHERE_TO_LOOK_DEFAULT(v),
            "WHERE_TO_LOOK_WORDS" => s_WHERE_TO_LOOK_WORDS(v),
            "WHERE_TO_START" => s_WHERE_TO_START(v),
            "WHETHER_ASKS" => s_WHETHER_ASKS(v),
            "WHOLE_OF_IT" => s_WHOLE_OF_IT(v),
            "WORDING" => s_WORDING(v),
            "WORDS_WANTED" => s_WORDS_WANTED(v),
            "_AFTER_A_NOUN" => s__AFTER_A_NOUN(v),
            "_AN_AMOUNT_WANTED" => s__AN_AMOUNT_WANTED(v),
            "_AN_ENTRY_IS_LOOKING" => s__AN_ENTRY_IS_LOOKING(v),
            "_A_AGE" => s__A_AGE(v),
            "_A_JUDGEMENT" => s__A_JUDGEMENT(v),
            "_A_STALL" => s__A_STALL(v),
            "_EXPLANATION_WORDS" => s__EXPLANATION_WORDS(v),
            "_FRAME_HEADS" => s__FRAME_HEADS(v),
            "_HANGS" => s__HANGS(v),
            "_MONEY_PAID_FOR" => s__MONEY_PAID_FOR(v),
            "_NEGATES" => s__NEGATES(v),
            "_OWNS_THE_THING" => s__OWNS_THE_THING(v),
            "_SO_COMES_THE_STEP" => s__SO_COMES_THE_STEP(v),
            "_SO_DOES_NOT_ASK_SOMETHING_OF_THEM" => s__SO_DOES_NOT_ASK_SOMETHING_OF_THEM(v),
            "_SO_THE_STEP_FOLLOWS" => s__SO_THE_STEP_FOLLOWS(v),
            "_SUBJECT_OPEN" => s__SUBJECT_OPEN(v),
            "_WEIGHS_ONE_AGAINST" => s__WEIGHS_ONE_AGAINST(v),
            "a_whole_cut" => s_a_whole_cut(v),
            "handed_back" => s_handed_back(v),
            "pick" => s_pick(v),
            "recent_answers" => s_recent_answers(v),
            "said_recently" => s_said_recently(v),
            "same_subject" => s_same_subject(v),
            "understand" => s_understand(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.repeat", module_obj);
    pyrt::xrefs::reg_fn("memory.repeat", "_doings", py_func("_doings", |args: &[Py]| _doings(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asks_for_the_same_work", py_func("asks_for_the_same_work", |args: &[Py]| asks_for_the_same_work(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "without_a_reset", py_func("without_a_reset", |args: &[Py]| without_a_reset(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_first_instruction_in_a_long_piece", py_func("_the_first_instruction_in_a_long_piece", |args: &[Py]| _the_first_instruction_in_a_long_piece(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_verb_after_a_leading_phrase", py_func("_a_verb_after_a_leading_phrase", |args: &[Py]| _a_verb_after_a_leading_phrase(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "only_said_it_back", py_func("only_said_it_back", |args: &[Py]| only_said_it_back(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_an_echo", py_func("_an_echo", |args: &[Py]| _an_echo(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_names_no_topic", py_func("_names_no_topic", |args: &[Py]| _names_no_topic(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_only_echoes_between", py_func("_only_echoes_between", |args: &[Py]| _only_echoes_between(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_help_ask", py_func("_a_help_ask", |args: &[Py]| _a_help_ask(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asks_for_the_move", py_func("asks_for_the_move", |args: &[Py]| asks_for_the_move(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "names_nothing", py_func("names_nothing", |args: &[Py]| names_nothing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_names_a_thing_of_its_own", py_func("_names_a_thing_of_its_own", |args: &[Py]| _names_a_thing_of_its_own(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asks_to_put_it_right", py_func("asks_to_put_it_right", |args: &[Py]| asks_to_put_it_right(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "a_step_of_the_same_answer", py_func("a_step_of_the_same_answer", |args: &[Py]| a_step_of_the_same_answer(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_recent_answers", py_func("_recent_answers", |args: &[Py]| _recent_answers(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_answer_about_what_we_are_on", py_func("_the_answer_about_what_we_are_on", |args: &[Py]| _the_answer_about_what_we_are_on(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_nothing_but_acknowledgements", py_func("_nothing_but_acknowledgements", |args: &[Py]| _nothing_but_acknowledgements(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_about_the_same_thing", py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_chat_is_still_on__dup1", py_func("_the_chat_is_still_on__dup1", |args: &[Py]| _the_chat_is_still_on__dup1(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_short", py_func("_short", |args: &[Py]| _short(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_explains", py_func("_explains", |args: &[Py]| _explains(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_step_comes_after", py_func("_a_step_comes_after", |args: &[Py]| _a_step_comes_after(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_opens_an_instruction", py_func("_opens_an_instruction", |args: &[Py]| _opens_an_instruction(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_opening_move", py_func("_opening_move", |args: &[Py]| _opening_move(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_question", py_func("_a_question", |args: &[Py]| _a_question(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_move", py_func("_a_move", |args: &[Py]| _a_move(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_move_in", py_func("_a_move_in", |args: &[Py]| _a_move_in(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_conditional_move", py_func("_a_conditional_move", |args: &[Py]| _a_conditional_move(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asks_for_more", py_func("asks_for_more", |args: &[Py]| asks_for_more(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "a_hand_back", py_func("a_hand_back", |args: &[Py]| a_hand_back(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_handed_back_last", py_func("_handed_back_last", |args: &[Py]| _handed_back_last(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_another_part", py_func("_another_part", |args: &[Py]| _another_part(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_said_in_a_turn", py_func("_said_in_a_turn", |args: &[Py]| _said_in_a_turn(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_says_what_to_do", py_func("_says_what_to_do", |args: &[Py]| _says_what_to_do(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_said_in_the_chat", py_func("_said_in_the_chat", |args: &[Py]| _said_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_claims_what_it_never_said", py_func("_claims_what_it_never_said", |args: &[Py]| _claims_what_it_never_said(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_handed_back", py_func("_handed_back", |args: &[Py]| _handed_back(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_frame_heads", py_func("_frame_heads", |args: &[Py]| _frame_heads(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "framed", py_func("framed", |args: &[Py]| framed(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "handed_back", py_func("handed_back", |args: &[Py]| py_call(&g_handed_back(), args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_already_repeated", py_func("_already_repeated", |args: &[Py]| _already_repeated(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_handed_back_already", py_func("_handed_back_already", |args: &[Py]| _handed_back_already(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_lead_of_its_own", py_func("_a_lead_of_its_own", |args: &[Py]| _a_lead_of_its_own(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_kept_clause", py_func("_kept_clause", |args: &[Py]| _kept_clause(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asks_for_less", py_func("asks_for_less", |args: &[Py]| asks_for_less(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_recent_lines", py_func("_recent_lines", |args: &[Py]| _recent_lines(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_sentences", py_func("_sentences", |args: &[Py]| _sentences(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_most_condensed", py_func("_most_condensed", |args: &[Py]| _most_condensed(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_short_enough", py_func("_short_enough", |args: &[Py]| _short_enough(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_ended", py_func("_ended", |args: &[Py]| _ended(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_unbalanced", py_func("_unbalanced", |args: &[Py]| _unbalanced(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_quotation", py_func("_the_quotation", |args: &[Py]| _the_quotation(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_closing_the_quote", py_func("_closing_the_quote", |args: &[Py]| _closing_the_quote(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_read_up_to_the_colon", py_func("_read_up_to_the_colon", |args: &[Py]| _read_up_to_the_colon(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_keeps", py_func("_keeps", |args: &[Py]| _keeps(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "shorter_for", py_func("shorter_for", |args: &[Py]| shorter_for(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_bare_time", py_func("_a_bare_time", |args: &[Py]| _a_bare_time(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_names_no_thing_of_its_own", py_func("_names_no_thing_of_its_own", |args: &[Py]| _names_no_thing_of_its_own(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_price_the_answer_gave", py_func("_the_price_the_answer_gave", |args: &[Py]| _the_price_the_answer_gave(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asks_what_that_costs", py_func("asks_what_that_costs", |args: &[Py]| asks_what_that_costs(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_price_in_hand", py_func("_the_price_in_hand", |args: &[Py]| _the_price_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_time_an_earlier_answer_gave", py_func("_the_time_an_earlier_answer_gave", |args: &[Py]| _the_time_an_earlier_answer_gave(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_length_already_handed", py_func("_a_length_already_handed", |args: &[Py]| _a_length_already_handed(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_what_to_do_answer", py_func("_a_what_to_do_answer", |args: &[Py]| _a_what_to_do_answer(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_city_guide_is_the_answer_in_front", py_func("_a_city_guide_is_the_answer_in_front", |args: &[Py]| _a_city_guide_is_the_answer_in_front(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_an_entry_answers_it", py_func("_an_entry_answers_it", |args: &[Py]| _an_entry_answers_it(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_bare_ask", py_func("_a_bare_ask", |args: &[Py]| _a_bare_ask(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_lead_and_a_step", py_func("_a_lead_and_a_step", |args: &[Py]| _a_lead_and_a_step(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_lead_for", py_func("_the_lead_for", |args: &[Py]| _the_lead_for(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_content", py_func("_content", |args: &[Py]| _content(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_trouble", py_func("_trouble", |args: &[Py]| _trouble(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_thing_of_its_own", py_func("_a_thing_of_its_own", |args: &[Py]| _a_thing_of_its_own(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "after_an_ack", py_func("after_an_ack", |args: &[Py]| after_an_ack(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_feeling", py_func("_a_feeling", |args: &[Py]| _a_feeling(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_doing", py_func("_the_doing", |args: &[Py]| _the_doing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_advice", py_func("_the_advice", |args: &[Py]| _the_advice(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_next_move", py_func("the_next_move", |args: &[Py]| the_next_move(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_next_step", py_func("_next_step", |args: &[Py]| _next_step(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_later_part", py_func("_later_part", |args: &[Py]| _later_part(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_opens_a_doing", py_func("_opens_a_doing", |args: &[Py]| _opens_a_doing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_untried", py_func("_the_untried", |args: &[Py]| _the_untried(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_tried", py_func("_tried", |args: &[Py]| _tried(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_says_they_agree", py_func("_says_they_agree", |args: &[Py]| _says_they_agree(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_thing_they_named", py_func("_the_thing_they_named", |args: &[Py]| _the_thing_they_named(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "agreement_with_the_answer", py_func("agreement_with_the_answer", |args: &[Py]| agreement_with_the_answer(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_agreed", py_func("_agreed", |args: &[Py]| _agreed(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_stems", py_func("_the_stems", |args: &[Py]| _the_stems(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_act", py_func("_the_act", |args: &[Py]| _the_act(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_object", py_func("_the_object", |args: &[Py]| _the_object(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_steps_done", py_func("_the_steps_done", |args: &[Py]| _the_steps_done(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_answer_gives", py_func("_the_answer_gives", |args: &[Py]| _the_answer_gives(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_step_done", py_func("_a_step_done", |args: &[Py]| _a_step_done(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_step_just_given", py_func("_the_step_just_given", |args: &[Py]| _the_step_just_given(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "carried_on", py_func("carried_on", |args: &[Py]| carried_on(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_starting_out_names_a_thing", py_func("_starting_out_names_a_thing", |args: &[Py]| _starting_out_names_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "says_where_to_start", py_func("says_where_to_start", |args: &[Py]| says_where_to_start(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "a_more_ask", py_func("a_more_ask", |args: &[Py]| a_more_ask(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_places_to_cut", py_func("_places_to_cut", |args: &[Py]| _places_to_cut(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_inside_a_bracket", py_func("_inside_a_bracket", |args: &[Py]| _inside_a_bracket(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_list_item", py_func("_a_list_item", |args: &[Py]| _a_list_item(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_pieces", py_func("_pieces", |args: &[Py]| _pieces(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_tail_asks_a_question", py_func("_the_tail_asks_a_question", |args: &[Py]| _the_tail_asks_a_question(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_colon_tail_kept", py_func("_the_colon_tail_kept", |args: &[Py]| _the_colon_tail_kept(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_opens_on_a_verb", py_func("_opens_on_a_verb", |args: &[Py]| _opens_on_a_verb(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_steps", py_func("_the_steps", |args: &[Py]| _the_steps(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_next_step", py_func("_the_next_step", |args: &[Py]| _the_next_step(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_first_step", py_func("_the_first_step", |args: &[Py]| _the_first_step(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_new_work", py_func("_a_new_work", |args: &[Py]| _a_new_work(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asks_which_part", py_func("asks_which_part", |args: &[Py]| asks_which_part(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_leads_for", py_func("_leads_for", |args: &[Py]| _leads_for(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_lead_for", py_func("_a_lead_for", |args: &[Py]| _a_lead_for(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_move", py_func("the_move", |args: &[Py]| the_move(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_they_say_they_tried", py_func("_they_say_they_tried", |args: &[Py]| _they_say_they_tried(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_figure_asked", py_func("the_figure_asked", |args: &[Py]| the_figure_asked(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_keys_of", py_func("_the_keys_of", |args: &[Py]| _the_keys_of(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "answers_nothing", py_func("answers_nothing", |args: &[Py]| answers_nothing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_advice_was_to_get_seen", py_func("the_advice_was_to_get_seen", |args: &[Py]| the_advice_was_to_get_seen(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_part_asked", py_func("the_part_asked", |args: &[Py]| the_part_asked(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_whether_to", py_func("_whether_to", |args: &[Py]| _whether_to(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_stem", py_func("_a_stem", |args: &[Py]| _a_stem(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_chat_is_still_on", py_func("_the_chat_is_still_on", |args: &[Py]| _the_chat_is_still_on(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_where_the_rest_is", py_func("_where_the_rest_is", |args: &[Py]| _where_the_rest_is(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "what_it_knows", py_func("what_it_knows", |args: &[Py]| what_it_knows(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_plain_line", py_func("the_plain_line", |args: &[Py]| the_plain_line(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "how_long_its_been", py_func("how_long_its_been", |args: &[Py]| how_long_its_been(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_line_that_corrects", py_func("_a_line_that_corrects", |args: &[Py]| _a_line_that_corrects(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_list_of_ages", py_func("_a_list_of_ages", |args: &[Py]| _a_list_of_ages(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_something_gone", py_func("_something_gone", |args: &[Py]| _something_gone(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_ask_gave_that_figure", py_func("_the_ask_gave_that_figure", |args: &[Py]| _the_ask_gave_that_figure(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_count_of_one_with_a_plural", py_func("_a_count_of_one_with_a_plural", |args: &[Py]| _a_count_of_one_with_a_plural(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_thing_counted", py_func("_the_thing_counted", |args: &[Py]| _the_thing_counted(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_figures_in", py_func("_figures_in", |args: &[Py]| _figures_in(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_money_paid_for_a_thing", py_func("_money_paid_for_a_thing", |args: &[Py]| _money_paid_for_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_one_figure", py_func("_one_figure", |args: &[Py]| _one_figure(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_a_bare_count", py_func("_a_bare_count", |args: &[Py]| _a_bare_count(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_same_word", py_func("_the_same_word", |args: &[Py]| _the_same_word(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_corrected", py_func("_the_corrected", |args: &[Py]| _the_corrected(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_length_they_mean", py_func("_the_length_they_mean", |args: &[Py]| _the_length_they_mean(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_figure_of_a_goal", py_func("_the_figure_of_a_goal", |args: &[Py]| _the_figure_of_a_goal(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_figure_counts_the_thing", py_func("_the_figure_counts_the_thing", |args: &[Py]| _the_figure_counts_the_thing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "a_narrowing_and_its_line", py_func("a_narrowing_and_its_line", |args: &[Py]| a_narrowing_and_its_line(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_turn_that_names_it", py_func("_the_turn_that_names_it", |args: &[Py]| _the_turn_that_names_it(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_figure_they_gave", py_func("the_figure_they_gave", |args: &[Py]| the_figure_they_gave(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_store_keeps_this_figure", py_func("_the_store_keeps_this_figure", |args: &[Py]| _the_store_keeps_this_figure(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_figure_beside_the_thing", py_func("_the_figure_beside_the_thing", |args: &[Py]| _the_figure_beside_the_thing(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_every_figure_the_ask_names", py_func("_every_figure_the_ask_names", |args: &[Py]| _every_figure_the_ask_names(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "asked_back", py_func("asked_back", |args: &[Py]| asked_back(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "carries_the_figure", py_func("carries_the_figure", |args: &[Py]| carries_the_figure(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_same_words", py_func("_the_same_words", |args: &[Py]| _the_same_words(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_shortened", py_func("_shortened", |args: &[Py]| _shortened(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "_the_answer_again", py_func("_the_answer_again", |args: &[Py]| _the_answer_again(args)));
    pyrt::xrefs::reg_fn("memory.repeat", "the_same_question_again", py_func("the_same_question_again", |args: &[Py]| the_same_question_again(args)));
    pyrt::xrefs::reg_access("memory.repeat", "ABOUT_THE_BOT", g_ABOUT_THE_BOT, s_ABOUT_THE_BOT);
    pyrt::xrefs::reg_access("memory.repeat", "ACK_AND_REST", g_ACK_AND_REST, s_ACK_AND_REST);
    pyrt::xrefs::reg_access("memory.repeat", "ACK_OPENS", g_ACK_OPENS, s_ACK_OPENS);
    pyrt::xrefs::reg_access("memory.repeat", "AGREEING", g_AGREEING, s_AGREEING);
    pyrt::xrefs::reg_access("memory.repeat", "AGREEMENT_TAIL", g_AGREEMENT_TAIL, s_AGREEMENT_TAIL);
    pyrt::xrefs::reg_access("memory.repeat", "AGREES_IN_A_PHRASE", g_AGREES_IN_A_PHRASE, s_AGREES_IN_A_PHRASE);
    pyrt::xrefs::reg_access("memory.repeat", "AGREES_ON_ITS_OWN", g_AGREES_ON_ITS_OWN, s_AGREES_ON_ITS_OWN);
    pyrt::xrefs::reg_access("memory.repeat", "ALREADY_SHORT", g_ALREADY_SHORT, s_ALREADY_SHORT);
    pyrt::xrefs::reg_access("memory.repeat", "ALSO_LEADS", g_ALSO_LEADS, s_ALSO_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "AND_NOW", g_AND_NOW, s_AND_NOW);
    pyrt::xrefs::reg_access("memory.repeat", "ANSWER_MODULES", g_ANSWER_MODULES, s_ANSWER_MODULES);
    pyrt::xrefs::reg_access("memory.repeat", "AN_ARTICLE", g_AN_ARTICLE, s_AN_ARTICLE);
    pyrt::xrefs::reg_access("memory.repeat", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "ASKING_THEM", g_ASKING_THEM, s_ASKING_THEM);
    pyrt::xrefs::reg_access("memory.repeat", "ASKING_TO_START", g_ASKING_TO_START, s_ASKING_TO_START);
    pyrt::xrefs::reg_access("memory.repeat", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "ASK_FRAME", g_ASK_FRAME, s_ASK_FRAME);
    pyrt::xrefs::reg_access("memory.repeat", "ASK_VERB", g_ASK_VERB, s_ASK_VERB);
    pyrt::xrefs::reg_access("memory.repeat", "ATTEMPT", g_ATTEMPT, s_ATTEMPT);
    pyrt::xrefs::reg_access("memory.repeat", "AT_A_LOSS", g_AT_A_LOSS, s_AT_A_LOSS);
    pyrt::xrefs::reg_access("memory.repeat", "A_BARE_AMOUNT_ASK", g_A_BARE_AMOUNT_ASK, s_A_BARE_AMOUNT_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "A_BARE_DURATION_ASK", g_A_BARE_DURATION_ASK, s_A_BARE_DURATION_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "A_BARE_TIME", g_A_BARE_TIME, s_A_BARE_TIME);
    pyrt::xrefs::reg_access("memory.repeat", "A_CORRECTION", g_A_CORRECTION, s_A_CORRECTION);
    pyrt::xrefs::reg_access("memory.repeat", "A_COUNT_OF_MORE_THAN_ONE", g_A_COUNT_OF_MORE_THAN_ONE, s_A_COUNT_OF_MORE_THAN_ONE);
    pyrt::xrefs::reg_access("memory.repeat", "A_DOING_WORD", g_A_DOING_WORD, s_A_DOING_WORD);
    pyrt::xrefs::reg_access("memory.repeat", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("memory.repeat", "A_FIGURE_WORD", g_A_FIGURE_WORD, s_A_FIGURE_WORD);
    pyrt::xrefs::reg_access("memory.repeat", "A_GAIN", g_A_GAIN, s_A_GAIN);
    pyrt::xrefs::reg_access("memory.repeat", "A_GOAL_LINE", g_A_GOAL_LINE, s_A_GOAL_LINE);
    pyrt::xrefs::reg_access("memory.repeat", "A_GROUP_WORD", g_A_GROUP_WORD, s_A_GROUP_WORD);
    pyrt::xrefs::reg_access("memory.repeat", "A_HAS_THE_LENGTH", g_A_HAS_THE_LENGTH, s_A_HAS_THE_LENGTH);
    pyrt::xrefs::reg_access("memory.repeat", "A_HOW_LONG_ITS_BEEN", g_A_HOW_LONG_ITS_BEEN, s_A_HOW_LONG_ITS_BEEN);
    pyrt::xrefs::reg_access("memory.repeat", "A_IM_ON_A_FIGURE", g_A_IM_ON_A_FIGURE, s_A_IM_ON_A_FIGURE);
    pyrt::xrefs::reg_access("memory.repeat", "A_LENGTH_ASK", g_A_LENGTH_ASK, s_A_LENGTH_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "A_LINE_THAT_ASKS", g_A_LINE_THAT_ASKS, s_A_LINE_THAT_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "A_LIST_OF_AGES", g_A_LIST_OF_AGES, s_A_LIST_OF_AGES);
    pyrt::xrefs::reg_access("memory.repeat", "A_LOSS", g_A_LOSS, s_A_LOSS);
    pyrt::xrefs::reg_access("memory.repeat", "A_MARKER_AT_THE_END", g_A_MARKER_AT_THE_END, s_A_MARKER_AT_THE_END);
    pyrt::xrefs::reg_access("memory.repeat", "A_MEASURE_WORD", g_A_MEASURE_WORD, s_A_MEASURE_WORD);
    pyrt::xrefs::reg_access("memory.repeat", "A_MENU", g_A_MENU, s_A_MENU);
    pyrt::xrefs::reg_access("memory.repeat", "A_MOVE", g_A_MOVE, s_A_MOVE);
    pyrt::xrefs::reg_access("memory.repeat", "A_MOVE_WORDS", g_A_MOVE_WORDS, s_A_MOVE_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "A_NARROWING_ONLY", g_A_NARROWING_ONLY, s_A_NARROWING_ONLY);
    pyrt::xrefs::reg_access("memory.repeat", "A_NARROWING_WORDS", g_A_NARROWING_WORDS, s_A_NARROWING_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "A_NUMBER_WORD", g_A_NUMBER_WORD, s_A_NUMBER_WORD);
    pyrt::xrefs::reg_access("memory.repeat", "A_ONE_MORE", g_A_ONE_MORE, s_A_ONE_MORE);
    pyrt::xrefs::reg_access("memory.repeat", "A_PAY_JUDGEMENT", g_A_PAY_JUDGEMENT, s_A_PAY_JUDGEMENT);
    pyrt::xrefs::reg_access("memory.repeat", "A_PAY_VERB_AT_THE_END", g_A_PAY_VERB_AT_THE_END, s_A_PAY_VERB_AT_THE_END);
    pyrt::xrefs::reg_access("memory.repeat", "A_PAY_WORD", g_A_PAY_WORD, s_A_PAY_WORD);
    pyrt::xrefs::reg_access("memory.repeat", "A_PLAN_ASK", g_A_PLAN_ASK, s_A_PLAN_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "A_PLURAL_ONLY_NOUN", g_A_PLURAL_ONLY_NOUN, s_A_PLURAL_ONLY_NOUN);
    pyrt::xrefs::reg_access("memory.repeat", "A_POINTS_BACK_HARD", g_A_POINTS_BACK_HARD, s_A_POINTS_BACK_HARD);
    pyrt::xrefs::reg_access("memory.repeat", "A_PRICE_ASK", g_A_PRICE_ASK, s_A_PRICE_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "A_QUALITY", g_A_QUALITY, s_A_QUALITY);
    pyrt::xrefs::reg_access("memory.repeat", "A_QUESTION", g_A_QUESTION, s_A_QUESTION);
    pyrt::xrefs::reg_access("memory.repeat", "A_RATE_IN_THE_FIGURE", g_A_RATE_IN_THE_FIGURE, s_A_RATE_IN_THE_FIGURE);
    pyrt::xrefs::reg_access("memory.repeat", "A_RATE_LINE", g_A_RATE_LINE, s_A_RATE_LINE);
    pyrt::xrefs::reg_access("memory.repeat", "A_RATE_OF_TIME", g_A_RATE_OF_TIME, s_A_RATE_OF_TIME);
    pyrt::xrefs::reg_access("memory.repeat", "A_RECALL_CUE", g_A_RECALL_CUE, s_A_RECALL_CUE);
    pyrt::xrefs::reg_access("memory.repeat", "A_SCHEDULED_AFTER", g_A_SCHEDULED_AFTER, s_A_SCHEDULED_AFTER);
    pyrt::xrefs::reg_access("memory.repeat", "A_STORE_ANSWER", g_A_STORE_ANSWER, s_A_STORE_ANSWER);
    pyrt::xrefs::reg_access("memory.repeat", "A_TAKE_HOME", g_A_TAKE_HOME, s_A_TAKE_HOME);
    pyrt::xrefs::reg_access("memory.repeat", "A_THE_PLAN_ASK", g_A_THE_PLAN_ASK, s_A_THE_PLAN_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "A_THING_NAMED_AFTER_THE_CUE", g_A_THING_NAMED_AFTER_THE_CUE, s_A_THING_NAMED_AFTER_THE_CUE);
    pyrt::xrefs::reg_access("memory.repeat", "A_TOLD_BACK", g_A_TOLD_BACK, s_A_TOLD_BACK);
    pyrt::xrefs::reg_access("memory.repeat", "A_WERE_ASKS", g_A_WERE_ASKS, s_A_WERE_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "BARE_ASK", g_BARE_ASK, s_BARE_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "BARE_ASK_NEEDS_A_QUESTION", g_BARE_ASK_NEEDS_A_QUESTION, s_BARE_ASK_NEEDS_A_QUESTION);
    pyrt::xrefs::reg_access("memory.repeat", "BE_FORM", g_BE_FORM, s_BE_FORM);
    pyrt::xrefs::reg_access("memory.repeat", "BREVITY_WORDS", g_BREVITY_WORDS, s_BREVITY_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "BROODING", g_BROODING, s_BROODING);
    pyrt::xrefs::reg_access("memory.repeat", "CHANGED_IT", g_CHANGED_IT, s_CHANGED_IT);
    pyrt::xrefs::reg_access("memory.repeat", "CHANGE_ASK", g_CHANGE_ASK, s_CHANGE_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "CHANGE_LEADS", g_CHANGE_LEADS, s_CHANGE_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "CLAIMS_IT_WAS_SAID", g_CLAIMS_IT_WAS_SAID, s_CLAIMS_IT_WAS_SAID);
    pyrt::xrefs::reg_access("memory.repeat", "CLAUSE_SPLIT", g_CLAUSE_SPLIT, s_CLAUSE_SPLIT);
    pyrt::xrefs::reg_access("memory.repeat", "CLOSES_THE_TURN", g_CLOSES_THE_TURN, s_CLOSES_THE_TURN);
    pyrt::xrefs::reg_access("memory.repeat", "CONDITION_CLAUSE", g_CONDITION_CLAUSE, s_CONDITION_CLAUSE);
    pyrt::xrefs::reg_access("memory.repeat", "COUNT_ASK", g_COUNT_ASK, s_COUNT_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "CUT_MID_SENTENCE", g_CUT_MID_SENTENCE, s_CUT_MID_SENTENCE);
    pyrt::xrefs::reg_access("memory.repeat", "CUT_WORDS", g_CUT_WORDS, s_CUT_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "DANGLING", g_DANGLING, s_DANGLING);
    pyrt::xrefs::reg_access("memory.repeat", "DECLARATIVE", g_DECLARATIVE, s_DECLARATIVE);
    pyrt::xrefs::reg_access("memory.repeat", "DIGIT", g_DIGIT, s_DIGIT);
    pyrt::xrefs::reg_access("memory.repeat", "DOINGS", g_DOINGS, s_DOINGS);
    pyrt::xrefs::reg_access("memory.repeat", "DONE", g_DONE, s_DONE);
    pyrt::xrefs::reg_access("memory.repeat", "DO_I_NEED_TO", g_DO_I_NEED_TO, s_DO_I_NEED_TO);
    pyrt::xrefs::reg_access("memory.repeat", "ECHO_FRAME", g_ECHO_FRAME, s_ECHO_FRAME);
    pyrt::xrefs::reg_access("memory.repeat", "FIGURES", g_FIGURES, s_FIGURES);
    pyrt::xrefs::reg_access("memory.repeat", "FIGURE_ASKS", g_FIGURE_ASKS, s_FIGURE_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "FIGURE_NOUNS", g_FIGURE_NOUNS, s_FIGURE_NOUNS);
    pyrt::xrefs::reg_access("memory.repeat", "FIGURE_SAYING", g_FIGURE_SAYING, s_FIGURE_SAYING);
    pyrt::xrefs::reg_access("memory.repeat", "FIGURE_TAIL", g_FIGURE_TAIL, s_FIGURE_TAIL);
    pyrt::xrefs::reg_access("memory.repeat", "FIGURE_WORDS", g_FIGURE_WORDS, s_FIGURE_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("memory.repeat", "FIRST_THING", g_FIRST_THING, s_FIRST_THING);
    pyrt::xrefs::reg_access("memory.repeat", "GAVE_ASKS", g_GAVE_ASKS, s_GAVE_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "GAVE_LOOK_BACK", g_GAVE_LOOK_BACK, s_GAVE_LOOK_BACK);
    pyrt::xrefs::reg_access("memory.repeat", "GOAL_NOUNS", g_GOAL_NOUNS, s_GOAL_NOUNS);
    pyrt::xrefs::reg_access("memory.repeat", "HEDGE", g_HEDGE, s_HEDGE);
    pyrt::xrefs::reg_access("memory.repeat", "HELP_ASK_QUESTION_FIRST", g_HELP_ASK_QUESTION_FIRST, s_HELP_ASK_QUESTION_FIRST);
    pyrt::xrefs::reg_access("memory.repeat", "HELP_SHAPE", g_HELP_SHAPE, s_HELP_SHAPE);
    pyrt::xrefs::reg_access("memory.repeat", "HOW_IT", g_HOW_IT, s_HOW_IT);
    pyrt::xrefs::reg_access("memory.repeat", "HOW_TO", g_HOW_TO, s_HOW_TO);
    pyrt::xrefs::reg_access("memory.repeat", "IDEAS_ASK", g_IDEAS_ASK, s_IDEAS_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "IDEAS_ONLY", g_IDEAS_ONLY, s_IDEAS_ONLY);
    pyrt::xrefs::reg_access("memory.repeat", "IRREGULAR_PAST", g_IRREGULAR_PAST, s_IRREGULAR_PAST);
    pyrt::xrefs::reg_access("memory.repeat", "JOINERS", g_JOINERS, s_JOINERS);
    pyrt::xrefs::reg_access("memory.repeat", "JUST_A_NOD", g_JUST_A_NOD, s_JUST_A_NOD);
    pyrt::xrefs::reg_access("memory.repeat", "JUST_THIS", g_JUST_THIS, s_JUST_THIS);
    pyrt::xrefs::reg_access("memory.repeat", "KEEPING", g_KEEPING, s_KEEPING);
    pyrt::xrefs::reg_access("memory.repeat", "KEEP_CLAUSE", g_KEEP_CLAUSE, s_KEEP_CLAUSE);
    pyrt::xrefs::reg_access("memory.repeat", "LEADING_PHRASE", g_LEADING_PHRASE, s_LEADING_PHRASE);
    pyrt::xrefs::reg_access("memory.repeat", "LEADING_PHRASE_MAX", g_LEADING_PHRASE_MAX, s_LEADING_PHRASE_MAX);
    pyrt::xrefs::reg_access("memory.repeat", "LEADS", g_LEADS, s_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "LEAD_IN", g_LEAD_IN, s_LEAD_IN);
    pyrt::xrefs::reg_access("memory.repeat", "LEAD_SAYS_FIRST", g_LEAD_SAYS_FIRST, s_LEAD_SAYS_FIRST);
    pyrt::xrefs::reg_access("memory.repeat", "LEAD_SPLIT", g_LEAD_SPLIT, s_LEAD_SPLIT);
    pyrt::xrefs::reg_access("memory.repeat", "LENGTH_BACK", g_LENGTH_BACK, s_LENGTH_BACK);
    pyrt::xrefs::reg_access("memory.repeat", "LESS", g_LESS, s_LESS);
    pyrt::xrefs::reg_access("memory.repeat", "LESS_LEADS", g_LESS_LEADS, s_LESS_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "LESS_WORDS", g_LESS_WORDS, s_LESS_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "LIST_LEAD", g_LIST_LEAD, s_LIST_LEAD);
    pyrt::xrefs::reg_access("memory.repeat", "LOOK_BACK", g_LOOK_BACK, s_LOOK_BACK);
    pyrt::xrefs::reg_access("memory.repeat", "LOOK_BACK_FOR_LESS", g_LOOK_BACK_FOR_LESS, s_LOOK_BACK_FOR_LESS);
    pyrt::xrefs::reg_access("memory.repeat", "LOSS_WORDS", g_LOSS_WORDS, s_LOSS_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "LOST_LEADS", g_LOST_LEADS, s_LOST_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "METHOD_ASKS", g_METHOD_ASKS, s_METHOD_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "MORE_ASK", g_MORE_ASK, s_MORE_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "MORE_LEADS", g_MORE_LEADS, s_MORE_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "MOST", g_MOST, s_MOST);
    pyrt::xrefs::reg_access("memory.repeat", "MOVE_ASK", g_MOVE_ASK, s_MOVE_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "MOVE_LEADS", g_MOVE_LEADS, s_MOVE_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "MOVE_VERBS", g_MOVE_VERBS, s_MOVE_VERBS);
    pyrt::xrefs::reg_access("memory.repeat", "NAMED_AFTER_IT", g_NAMED_AFTER_IT, s_NAMED_AFTER_IT);
    pyrt::xrefs::reg_access("memory.repeat", "NEEDS_COMPLEMENT", g_NEEDS_COMPLEMENT, s_NEEDS_COMPLEMENT);
    pyrt::xrefs::reg_access("memory.repeat", "NEXT_ASK", g_NEXT_ASK, s_NEXT_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "NEXT_LEADS", g_NEXT_LEADS, s_NEXT_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "NEXT_STEP", g_NEXT_STEP, s_NEXT_STEP);
    pyrt::xrefs::reg_access("memory.repeat", "NEXT_STEP_EMERGENCY", g_NEXT_STEP_EMERGENCY, s_NEXT_STEP_EMERGENCY);
    pyrt::xrefs::reg_access("memory.repeat", "NOTHING_TO_CUT", g_NOTHING_TO_CUT, s_NOTHING_TO_CUT);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_ALONE", g_NOT_ALONE, s_NOT_ALONE);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_DOING", g_NOT_A_DOING, s_NOT_A_DOING);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_FIGURE_ASK", g_NOT_A_FIGURE_ASK, s_NOT_A_FIGURE_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_METHOD", g_NOT_A_METHOD, s_NOT_A_METHOD);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_MOVE", g_NOT_A_MOVE, s_NOT_A_MOVE);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_STEP", g_NOT_A_STEP, s_NOT_A_STEP);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_SUBJECT", g_NOT_A_SUBJECT, s_NOT_A_SUBJECT);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_TAIL", g_NOT_A_TAIL, s_NOT_A_TAIL);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_A_THING_ASKED", g_NOT_A_THING_ASKED, s_NOT_A_THING_ASKED);
    pyrt::xrefs::reg_access("memory.repeat", "NOT_TRIED", g_NOT_TRIED, s_NOT_TRIED);
    pyrt::xrefs::reg_access("memory.repeat", "NOW_LEADS", g_NOW_LEADS, s_NOW_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "NO_LONGER_TO_DO", g_NO_LONGER_TO_DO, s_NO_LONGER_TO_DO);
    pyrt::xrefs::reg_access("memory.repeat", "NO_SHORTER_STILL", g_NO_SHORTER_STILL, s_NO_SHORTER_STILL);
    pyrt::xrefs::reg_access("memory.repeat", "OBJECT_END", g_OBJECT_END, s_OBJECT_END);
    pyrt::xrefs::reg_access("memory.repeat", "OPENERS", g_OPENERS, s_OPENERS);
    pyrt::xrefs::reg_access("memory.repeat", "ORDER", g_ORDER, s_ORDER);
    pyrt::xrefs::reg_access("memory.repeat", "OWNED", g_OWNED, s_OWNED);
    pyrt::xrefs::reg_access("memory.repeat", "OWN_LEAD", g_OWN_LEAD, s_OWN_LEAD);
    pyrt::xrefs::reg_access("memory.repeat", "OWN_LEAD_WORDS", g_OWN_LEAD_WORDS, s_OWN_LEAD_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "PART_ASKS", g_PART_ASKS, s_PART_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "PART_OPENS", g_PART_OPENS, s_PART_OPENS);
    pyrt::xrefs::reg_access("memory.repeat", "PART_STAND_INS", g_PART_STAND_INS, s_PART_STAND_INS);
    pyrt::xrefs::reg_access("memory.repeat", "PAY_FAMILY", g_PAY_FAMILY, s_PAY_FAMILY);
    pyrt::xrefs::reg_access("memory.repeat", "PAY_WHEN_WORDS", g_PAY_WHEN_WORDS, s_PAY_WHEN_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "PERIOD_WORD", g_PERIOD_WORD, s_PERIOD_WORD);
    pyrt::xrefs::reg_access("memory.repeat", "PIECE_END", g_PIECE_END, s_PIECE_END);
    pyrt::xrefs::reg_access("memory.repeat", "POINTING_AT", g_POINTING_AT, s_POINTING_AT);
    pyrt::xrefs::reg_access("memory.repeat", "POINTING_BACK", g_POINTING_BACK, s_POINTING_BACK);
    pyrt::xrefs::reg_access("memory.repeat", "PRICE_WORDS", g_PRICE_WORDS, s_PRICE_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "PUT_RIGHT", g_PUT_RIGHT, s_PUT_RIGHT);
    pyrt::xrefs::reg_access("memory.repeat", "PUT_RIGHT_ASK", g_PUT_RIGHT_ASK, s_PUT_RIGHT_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "PUT_RIGHT_LEADS", g_PUT_RIGHT_LEADS, s_PUT_RIGHT_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "PUT_RIGHT_WORDS", g_PUT_RIGHT_WORDS, s_PUT_RIGHT_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "QUALIFIER", g_QUALIFIER, s_QUALIFIER);
    pyrt::xrefs::reg_access("memory.repeat", "QUESTION", g_QUESTION, s_QUESTION);
    pyrt::xrefs::reg_access("memory.repeat", "QUESTIONS_THAT_ARE_NOT_STEPS", g_QUESTIONS_THAT_ARE_NOT_STEPS, s_QUESTIONS_THAT_ARE_NOT_STEPS);
    pyrt::xrefs::reg_access("memory.repeat", "QUESTION_MORE", g_QUESTION_MORE, s_QUESTION_MORE);
    pyrt::xrefs::reg_access("memory.repeat", "QUOTE_PAIRS", g_QUOTE_PAIRS, s_QUOTE_PAIRS);
    pyrt::xrefs::reg_access("memory.repeat", "REFER_BACK", g_REFER_BACK, s_REFER_BACK);
    pyrt::xrefs::reg_access("memory.repeat", "REJOINDER", g_REJOINDER, s_REJOINDER);
    pyrt::xrefs::reg_access("memory.repeat", "RESET_LEAD", g_RESET_LEAD, s_RESET_LEAD);
    pyrt::xrefs::reg_access("memory.repeat", "SAID_SAFE", g_SAID_SAFE, s_SAID_SAFE);
    pyrt::xrefs::reg_access("memory.repeat", "SAY_AGAIN_LEADS", g_SAY_AGAIN_LEADS, s_SAY_AGAIN_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "SELF", g_SELF, s_SELF);
    pyrt::xrefs::reg_access("memory.repeat", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.repeat", "SENTENCE_END", g_SENTENCE_END, s_SENTENCE_END);
    pyrt::xrefs::reg_access("memory.repeat", "SHORT_ENOUGH", g_SHORT_ENOUGH, s_SHORT_ENOUGH);
    pyrt::xrefs::reg_access("memory.repeat", "SHORT_STEP", g_SHORT_STEP, s_SHORT_STEP);
    pyrt::xrefs::reg_access("memory.repeat", "SOMEONE_ELSES_LIMIT", g_SOMEONE_ELSES_LIMIT, s_SOMEONE_ELSES_LIMIT);
    pyrt::xrefs::reg_access("memory.repeat", "STALL", g_STALL, s_STALL);
    pyrt::xrefs::reg_access("memory.repeat", "STANDS_IN", g_STANDS_IN, s_STANDS_IN);
    pyrt::xrefs::reg_access("memory.repeat", "STARTING_AT", g_STARTING_AT, s_STARTING_AT);
    pyrt::xrefs::reg_access("memory.repeat", "STARTING_BACK", g_STARTING_BACK, s_STARTING_BACK);
    pyrt::xrefs::reg_access("memory.repeat", "STARTING_INTO", g_STARTING_INTO, s_STARTING_INTO);
    pyrt::xrefs::reg_access("memory.repeat", "STARTING_OUT", g_STARTING_OUT, s_STARTING_OUT);
    pyrt::xrefs::reg_access("memory.repeat", "STARTING_WHAT", g_STARTING_WHAT, s_STARTING_WHAT);
    pyrt::xrefs::reg_access("memory.repeat", "START_ASK", g_START_ASK, s_START_ASK);
    pyrt::xrefs::reg_access("memory.repeat", "STEP_SAYS_FIRST", g_STEP_SAYS_FIRST, s_STEP_SAYS_FIRST);
    pyrt::xrefs::reg_access("memory.repeat", "STEP_VERBS", g_STEP_VERBS, s_STEP_VERBS);
    pyrt::xrefs::reg_access("memory.repeat", "STEP_WITH_ITS_METHOD", g_STEP_WITH_ITS_METHOD, s_STEP_WITH_ITS_METHOD);
    pyrt::xrefs::reg_access("memory.repeat", "STEP_WORDS", g_STEP_WORDS, s_STEP_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "STILL_ASKS", g_STILL_ASKS, s_STILL_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "STILL_SAFE", g_STILL_SAFE, s_STILL_SAFE);
    pyrt::xrefs::reg_access("memory.repeat", "THE_SAME_NUMBER", g_THE_SAME_NUMBER, s_THE_SAME_NUMBER);
    pyrt::xrefs::reg_access("memory.repeat", "TIME", g_TIME, s_TIME);
    pyrt::xrefs::reg_access("memory.repeat", "TRIED", g_TRIED, s_TRIED);
    pyrt::xrefs::reg_access("memory.repeat", "TRIED_LEADS", g_TRIED_LEADS, s_TRIED_LEADS);
    pyrt::xrefs::reg_access("memory.repeat", "TROUBLE", g_TROUBLE, s_TROUBLE);
    pyrt::xrefs::reg_access("memory.repeat", "USE_WITH", g_USE_WITH, s_USE_WITH);
    pyrt::xrefs::reg_access("memory.repeat", "WELCOMES", g_WELCOMES, s_WELCOMES);
    pyrt::xrefs::reg_access("memory.repeat", "WHAT_IT_IS", g_WHAT_IT_IS, s_WHAT_IT_IS);
    pyrt::xrefs::reg_access("memory.repeat", "WHERE_TO_LOOK", g_WHERE_TO_LOOK, s_WHERE_TO_LOOK);
    pyrt::xrefs::reg_access("memory.repeat", "WHERE_TO_LOOK_DEFAULT", g_WHERE_TO_LOOK_DEFAULT, s_WHERE_TO_LOOK_DEFAULT);
    pyrt::xrefs::reg_access("memory.repeat", "WHERE_TO_LOOK_WORDS", g_WHERE_TO_LOOK_WORDS, s_WHERE_TO_LOOK_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "WHERE_TO_START", g_WHERE_TO_START, s_WHERE_TO_START);
    pyrt::xrefs::reg_access("memory.repeat", "WHETHER_ASKS", g_WHETHER_ASKS, s_WHETHER_ASKS);
    pyrt::xrefs::reg_access("memory.repeat", "WHOLE_OF_IT", g_WHOLE_OF_IT, s_WHOLE_OF_IT);
    pyrt::xrefs::reg_access("memory.repeat", "WORDING", g_WORDING, s_WORDING);
    pyrt::xrefs::reg_access("memory.repeat", "WORDS_WANTED", g_WORDS_WANTED, s_WORDS_WANTED);
    pyrt::xrefs::reg_access("memory.repeat", "_AFTER_A_NOUN", g__AFTER_A_NOUN, s__AFTER_A_NOUN);
    pyrt::xrefs::reg_access("memory.repeat", "_AN_AMOUNT_WANTED", g__AN_AMOUNT_WANTED, s__AN_AMOUNT_WANTED);
    pyrt::xrefs::reg_access("memory.repeat", "_AN_ENTRY_IS_LOOKING", g__AN_ENTRY_IS_LOOKING, s__AN_ENTRY_IS_LOOKING);
    pyrt::xrefs::reg_access("memory.repeat", "_A_AGE", g__A_AGE, s__A_AGE);
    pyrt::xrefs::reg_access("memory.repeat", "_A_JUDGEMENT", g__A_JUDGEMENT, s__A_JUDGEMENT);
    pyrt::xrefs::reg_access("memory.repeat", "_A_STALL", g__A_STALL, s__A_STALL);
    pyrt::xrefs::reg_access("memory.repeat", "_EXPLANATION_WORDS", g__EXPLANATION_WORDS, s__EXPLANATION_WORDS);
    pyrt::xrefs::reg_access("memory.repeat", "_FRAME_HEADS", g__FRAME_HEADS, s__FRAME_HEADS);
    pyrt::xrefs::reg_access("memory.repeat", "_HANGS", g__HANGS, s__HANGS);
    pyrt::xrefs::reg_access("memory.repeat", "_MONEY_PAID_FOR", g__MONEY_PAID_FOR, s__MONEY_PAID_FOR);
    pyrt::xrefs::reg_access("memory.repeat", "_NEGATES", g__NEGATES, s__NEGATES);
    pyrt::xrefs::reg_access("memory.repeat", "_OWNS_THE_THING", g__OWNS_THE_THING, s__OWNS_THE_THING);
    pyrt::xrefs::reg_access("memory.repeat", "_SO_COMES_THE_STEP", g__SO_COMES_THE_STEP, s__SO_COMES_THE_STEP);
    pyrt::xrefs::reg_access("memory.repeat", "_SO_DOES_NOT_ASK_SOMETHING_OF_THEM", g__SO_DOES_NOT_ASK_SOMETHING_OF_THEM, s__SO_DOES_NOT_ASK_SOMETHING_OF_THEM);
    pyrt::xrefs::reg_access("memory.repeat", "_SO_THE_STEP_FOLLOWS", g__SO_THE_STEP_FOLLOWS, s__SO_THE_STEP_FOLLOWS);
    pyrt::xrefs::reg_access("memory.repeat", "_SUBJECT_OPEN", g__SUBJECT_OPEN, s__SUBJECT_OPEN);
    pyrt::xrefs::reg_access("memory.repeat", "_WEIGHS_ONE_AGAINST", g__WEIGHS_ONE_AGAINST, s__WEIGHS_ONE_AGAINST);
    pyrt::xrefs::reg_access("memory.repeat", "a_whole_cut", g_a_whole_cut, s_a_whole_cut);
    pyrt::xrefs::reg_access("memory.repeat", "handed_back", g_handed_back, s_handed_back);
    pyrt::xrefs::reg_access("memory.repeat", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("memory.repeat", "recent_answers", g_recent_answers, s_recent_answers);
    pyrt::xrefs::reg_access("memory.repeat", "said_recently", g_said_recently, s_said_recently);
    pyrt::xrefs::reg_access("memory.repeat", "same_subject", g_same_subject, s_same_subject);
    pyrt::xrefs::reg_access("memory.repeat", "understand", g_understand, s_understand);
}
