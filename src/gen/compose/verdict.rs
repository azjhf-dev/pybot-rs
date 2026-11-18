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


pub static ABOUT_THE_TALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADVICE_OK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALL_OF_IT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALSO_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_OFFER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKED_FIRST_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKED_FOR_A_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKED_THEM_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_HOW_TO_TELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_OK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BODY_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BODY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHECK_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHECK_ON_A_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHECK_ON_A_PLAN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DECISION_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DOUBT_ABOUT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIGURE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FREQUENCY_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FREQUENCY_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOW_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOW_TO_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_KEEP_OR_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LENGTH_IN_THE_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_THEY_KNOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLAN_DOUBT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REFUSAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RELATION_TROUBLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STATEMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SYMPTOM_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_THEY_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THINKING_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WAIT_IN_THE_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WORRY_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACKSTOP_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOTH_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALLS_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAN_HURT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_NORMAL_UNKNOWABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_NOW_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHECK_JOB_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHECK_ONCE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHECK_WATCH_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHECK_WORKS_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOICE_OK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPOSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITIONAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITIONAL_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITIONAL_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITION_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITION_HANGING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITION_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITION_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITION_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONSIDERING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTINUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTRAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DANGER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DECLARATIVE_REFERRAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DESCRIBES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETAIL_STATEMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DIRECTED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOING_WITH_NO_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DO_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DO_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ECHO_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ECHO_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EMERGENCY_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EMERGENCY_CALL_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EMERGENCY_ORDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENOUGH_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENOUGH_STATED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPLANATION_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_DOUBT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_FAMILY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FENCED_IN_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FENCE_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIX_ASKED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIX_ASKED_NOW_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLOOR_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_AND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRONT_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRONT_SUBJECT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRONT_THIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRONT_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GO_FROM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_FLOOR_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANDED_BACK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_OVER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANGING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HARD_NEWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HARD_NEWS_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HARMLESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAD_REFUSES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGE_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELPED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MANY_PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IMPERATIVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_A_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JUDGED_REFERRAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JUST_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KNOWING_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LATER_CONDITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIST_STEP_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LONG_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LONG_STEP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOK_AND_SEE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static METHOD_DOUBT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static METHOD_OK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MET_CONDITION_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MET_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_NOT_MET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_NOUN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_TROUBLE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_A_CLINICIAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_A_TRADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEEDS_A_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATED_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATIVE_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEUTRAL_ASK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_WARNED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_WARNED_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_STEP_TO_WATCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_STEP_TO_WATCH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_MONEY_PAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TOLD_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_STATED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFFER_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFFER_TAKEN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONE_OPTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONE_THING_ADVERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONE_TURN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONE_TURN_PLAIN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_CONTINUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDINARY_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OVER_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_PATTERN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_ORDINARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POOLS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUNT_CHAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUNT_OFFER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUNT_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_BACK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static READABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REASSURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECAP_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFER_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RELATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RESTATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULED_BY_THE_COMPOSER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_YES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEE_ANYWAY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEE_NONE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEE_NOW_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEE_STATED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIGN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOON_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAK_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAK_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPLIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATED_ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATED_LONGEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATED_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATES_A_REFERRAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_ALREADY_FIRST_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_DETAIL_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_KEPT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_ONE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_GOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBJECT_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SYMPATHY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TARGET_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TESTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TEST_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEN_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_PLAN_CAN_HURT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_PLEA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_ORDINARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THROW_AWAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THROW_AWAY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIPS_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIM_INSTRUMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNREADABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static URGENT_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static URGENT_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_CALL_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORRIED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORRYING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YES_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YES_SAID_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ACK_PATTERN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AFTER_A_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_OFFER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_ABOUT_A_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_FOR_THE_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BODY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DASH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FIX_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_RANGE_OF_ITS_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_REFERRAL_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BODY_PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FILL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _INVITATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LEAD_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LIST_JOIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_SIGN_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ONE_THING_ADVERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STEP_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SYMPTOM_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _a_thing_stopped: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _reads_as_good_news: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_THE_TALK() -> Py { ensure_init(); ABOUT_THE_TALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_TALK(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THE_TALK.write() { *w = v; } }
pub fn g_ADVICE_OK_LINES() -> Py { ensure_init(); ADVICE_OK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE_OK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE_OK_LINES.write() { *w = v; } }
pub fn g_AFTER() -> Py { ensure_init(); AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = AFTER.write() { *w = v; } }
pub fn g_AGAIN() -> Py { ensure_init(); AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = AGAIN.write() { *w = v; } }
pub fn g_AGE_LINES() -> Py { ensure_init(); AGE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = AGE_LINES.write() { *w = v; } }
pub fn g_ALL_OF_IT_LINES() -> Py { ensure_init(); ALL_OF_IT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALL_OF_IT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ALL_OF_IT_LINES.write() { *w = v; } }
pub fn g_ALSO_LINES() -> Py { ensure_init(); ALSO_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALSO_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ALSO_LINES.write() { *w = v; } }
pub fn g_AN_OFFER() -> Py { ensure_init(); AN_OFFER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_OFFER(v: Py) { ensure_init(); if let Ok(mut w) = AN_OFFER.write() { *w = v; } }
pub fn g_ASKED_FIRST_LINES() -> Py { ensure_init(); ASKED_FIRST_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKED_FIRST_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ASKED_FIRST_LINES.write() { *w = v; } }
pub fn g_ASKED_FOR_A_CALL() -> Py { ensure_init(); ASKED_FOR_A_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKED_FOR_A_CALL(v: Py) { ensure_init(); if let Ok(mut w) = ASKED_FOR_A_CALL.write() { *w = v; } }
pub fn g_ASKED_THEM_LINES() -> Py { ensure_init(); ASKED_THEM_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKED_THEM_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ASKED_THEM_LINES.write() { *w = v; } }
pub fn g_ASKING_HOW_TO_TELL() -> Py { ensure_init(); ASKING_HOW_TO_TELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_HOW_TO_TELL(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_HOW_TO_TELL.write() { *w = v; } }
pub fn g_ASKS_FINE() -> Py { ensure_init(); ASKS_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_FINE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_FINE.write() { *w = v; } }
pub fn g_ASKS_NORMAL() -> Py { ensure_init(); ASKS_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_NORMAL.write() { *w = v; } }
pub fn g_ASKS_OK() -> Py { ensure_init(); ASKS_OK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_OK(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_OK.write() { *w = v; } }
pub fn g_ASKS_THEM() -> Py { ensure_init(); ASKS_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_THEM(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_THEM.write() { *w = v; } }
pub fn g_ASKS_WRONG() -> Py { ensure_init(); ASKS_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WRONG.write() { *w = v; } }
pub fn g_A_BODY_IN_THE_LINE() -> Py { ensure_init(); A_BODY_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BODY_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = A_BODY_IN_THE_LINE.write() { *w = v; } }
pub fn g_A_BODY_WORD() -> Py { ensure_init(); A_BODY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BODY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_BODY_WORD.write() { *w = v; } }
pub fn g_A_CHECK_ASK() -> Py { ensure_init(); A_CHECK_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHECK_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_CHECK_ASK.write() { *w = v; } }
pub fn g_A_CHECK_ON_A_PLAN() -> Py { ensure_init(); A_CHECK_ON_A_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHECK_ON_A_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = A_CHECK_ON_A_PLAN.write() { *w = v; } }
pub fn g_A_CHECK_ON_A_PLAN_LINES() -> Py { ensure_init(); A_CHECK_ON_A_PLAN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHECK_ON_A_PLAN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = A_CHECK_ON_A_PLAN_LINES.write() { *w = v; } }
pub fn g_A_COMPARISON() -> Py { ensure_init(); A_COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = A_COMPARISON.write() { *w = v; } }
pub fn g_A_DECISION_ASK() -> Py { ensure_init(); A_DECISION_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DECISION_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_DECISION_ASK.write() { *w = v; } }
pub fn g_A_DOUBT_ABOUT_THE_THING() -> Py { ensure_init(); A_DOUBT_ABOUT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DOUBT_ABOUT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_DOUBT_ABOUT_THE_THING.write() { *w = v; } }
pub fn g_A_FIGURE() -> Py { ensure_init(); A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = A_FIGURE.write() { *w = v; } }
pub fn g_A_FIGURE_ASK() -> Py { ensure_init(); A_FIGURE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_FIGURE_ASK.write() { *w = v; } }
pub fn g_A_FREQUENCY_ASKED() -> Py { ensure_init(); A_FREQUENCY_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FREQUENCY_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = A_FREQUENCY_ASKED.write() { *w = v; } }
pub fn g_A_FREQUENCY_SAID() -> Py { ensure_init(); A_FREQUENCY_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FREQUENCY_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_FREQUENCY_SAID.write() { *w = v; } }
pub fn g_A_HOW_ASK() -> Py { ensure_init(); A_HOW_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOW_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_HOW_ASK.write() { *w = v; } }
pub fn g_A_HOW_TO_ASK() -> Py { ensure_init(); A_HOW_TO_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOW_TO_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_HOW_TO_ASK.write() { *w = v; } }
pub fn g_A_KEEP_OR_STOP() -> Py { ensure_init(); A_KEEP_OR_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_KEEP_OR_STOP(v: Py) { ensure_init(); if let Ok(mut w) = A_KEEP_OR_STOP.write() { *w = v; } }
pub fn g_A_LENGTH_IN_THE_ANSWER() -> Py { ensure_init(); A_LENGTH_IN_THE_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_IN_THE_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = A_LENGTH_IN_THE_ANSWER.write() { *w = v; } }
pub fn g_A_PERSON_THEY_KNOW() -> Py { ensure_init(); A_PERSON_THEY_KNOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_THEY_KNOW(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_THEY_KNOW.write() { *w = v; } }
pub fn g_A_PLAN() -> Py { ensure_init(); A_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = A_PLAN.write() { *w = v; } }
pub fn g_A_PLAN_DOUBT_LINES() -> Py { ensure_init(); A_PLAN_DOUBT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_DOUBT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = A_PLAN_DOUBT_LINES.write() { *w = v; } }
pub fn g_A_REFUSAL() -> Py { ensure_init(); A_REFUSAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REFUSAL(v: Py) { ensure_init(); if let Ok(mut w) = A_REFUSAL.write() { *w = v; } }
pub fn g_A_RELATION_TROUBLE() -> Py { ensure_init(); A_RELATION_TROUBLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RELATION_TROUBLE(v: Py) { ensure_init(); if let Ok(mut w) = A_RELATION_TROUBLE.write() { *w = v; } }
pub fn g_A_STATEMENT() -> Py { ensure_init(); A_STATEMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STATEMENT(v: Py) { ensure_init(); if let Ok(mut w) = A_STATEMENT.write() { *w = v; } }
pub fn g_A_SYMPTOM_WORD() -> Py { ensure_init(); A_SYMPTOM_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SYMPTOM_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_SYMPTOM_WORD.write() { *w = v; } }
pub fn g_A_THING_THEY_OWN() -> Py { ensure_init(); A_THING_THEY_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_THEY_OWN(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_THEY_OWN.write() { *w = v; } }
pub fn g_A_THINKING_ABOUT() -> Py { ensure_init(); A_THINKING_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THINKING_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = A_THINKING_ABOUT.write() { *w = v; } }
pub fn g_A_WAIT_IN_THE_STEP() -> Py { ensure_init(); A_WAIT_IN_THE_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WAIT_IN_THE_STEP(v: Py) { ensure_init(); if let Ok(mut w) = A_WAIT_IN_THE_STEP.write() { *w = v; } }
pub fn g_A_WORRY_IN_THE_LINE() -> Py { ensure_init(); A_WORRY_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WORRY_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = A_WORRY_IN_THE_LINE.write() { *w = v; } }
pub fn g_BACKSTOP_LINES() -> Py { ensure_init(); BACKSTOP_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACKSTOP_LINES(v: Py) { ensure_init(); if let Ok(mut w) = BACKSTOP_LINES.write() { *w = v; } }
pub fn g_BARE() -> Py { ensure_init(); BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE(v: Py) { ensure_init(); if let Ok(mut w) = BARE.write() { *w = v; } }
pub fn g_BODY_WORD() -> Py { ensure_init(); BODY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = BODY_WORD.write() { *w = v; } }
pub fn g_BOTH_LINES() -> Py { ensure_init(); BOTH_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOTH_LINES(v: Py) { ensure_init(); if let Ok(mut w) = BOTH_LINES.write() { *w = v; } }
pub fn g_CALLS_FOR() -> Py { ensure_init(); CALLS_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALLS_FOR(v: Py) { ensure_init(); if let Ok(mut w) = CALLS_FOR.write() { *w = v; } }
pub fn g_CAN_HURT() -> Py { ensure_init(); CAN_HURT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAN_HURT(v: Py) { ensure_init(); if let Ok(mut w) = CAN_HURT.write() { *w = v; } }
pub fn g_CARE() -> Py { ensure_init(); CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE(v: Py) { ensure_init(); if let Ok(mut w) = CARE.write() { *w = v; } }
pub fn g_CARE_NORMAL() -> Py { ensure_init(); CARE_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = CARE_NORMAL.write() { *w = v; } }
pub fn g_CARE_NORMAL_UNKNOWABLE() -> Py { ensure_init(); CARE_NORMAL_UNKNOWABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_NORMAL_UNKNOWABLE(v: Py) { ensure_init(); if let Ok(mut w) = CARE_NORMAL_UNKNOWABLE.write() { *w = v; } }
pub fn g_CARE_NOW() -> Py { ensure_init(); CARE_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_NOW(v: Py) { ensure_init(); if let Ok(mut w) = CARE_NOW.write() { *w = v; } }
pub fn g_CARE_NOW_NORMAL() -> Py { ensure_init(); CARE_NOW_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_NOW_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = CARE_NOW_NORMAL.write() { *w = v; } }
pub fn g_CHECK_JOB_LINES() -> Py { ensure_init(); CHECK_JOB_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHECK_JOB_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CHECK_JOB_LINES.write() { *w = v; } }
pub fn g_CHECK_ONCE_LINES() -> Py { ensure_init(); CHECK_ONCE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHECK_ONCE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CHECK_ONCE_LINES.write() { *w = v; } }
pub fn g_CHECK_WATCH_LINES() -> Py { ensure_init(); CHECK_WATCH_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHECK_WATCH_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CHECK_WATCH_LINES.write() { *w = v; } }
pub fn g_CHECK_WORKS_LINES() -> Py { ensure_init(); CHECK_WORKS_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHECK_WORKS_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CHECK_WORKS_LINES.write() { *w = v; } }
pub fn g_CHOICE_OK_LINES() -> Py { ensure_init(); CHOICE_OK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOICE_OK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CHOICE_OK_LINES.write() { *w = v; } }
pub fn g_COMPARATIVE() -> Py { ensure_init(); COMPARATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARATIVE(v: Py) { ensure_init(); if let Ok(mut w) = COMPARATIVE.write() { *w = v; } }
pub fn g_COMPOSED() -> Py { ensure_init(); COMPOSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPOSED(v: Py) { ensure_init(); if let Ok(mut w) = COMPOSED.write() { *w = v; } }
pub fn g_CONDITIONAL() -> Py { ensure_init(); CONDITIONAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL(v: Py) { ensure_init(); if let Ok(mut w) = CONDITIONAL.write() { *w = v; } }
pub fn g_CONDITIONAL_AFTER() -> Py { ensure_init(); CONDITIONAL_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = CONDITIONAL_AFTER.write() { *w = v; } }
pub fn g_CONDITIONAL_OPEN() -> Py { ensure_init(); CONDITIONAL_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = CONDITIONAL_OPEN.write() { *w = v; } }
pub fn g_CONDITION_CLAUSE() -> Py { ensure_init(); CONDITION_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITION_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = CONDITION_CLAUSE.write() { *w = v; } }
pub fn g_CONDITION_HANGING() -> Py { ensure_init(); CONDITION_HANGING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITION_HANGING(v: Py) { ensure_init(); if let Ok(mut w) = CONDITION_HANGING.write() { *w = v; } }
pub fn g_CONDITION_LINES() -> Py { ensure_init(); CONDITION_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITION_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CONDITION_LINES.write() { *w = v; } }
pub fn g_CONDITION_NORMAL() -> Py { ensure_init(); CONDITION_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITION_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = CONDITION_NORMAL.write() { *w = v; } }
pub fn g_CONDITION_VERBS() -> Py { ensure_init(); CONDITION_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITION_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = CONDITION_VERBS.write() { *w = v; } }
pub fn g_CONSIDERING() -> Py { ensure_init(); CONSIDERING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONSIDERING(v: Py) { ensure_init(); if let Ok(mut w) = CONSIDERING.write() { *w = v; } }
pub fn g_CONTINUES() -> Py { ensure_init(); CONTINUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTINUES(v: Py) { ensure_init(); if let Ok(mut w) = CONTINUES.write() { *w = v; } }
pub fn g_CONTRAST() -> Py { ensure_init(); CONTRAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTRAST(v: Py) { ensure_init(); if let Ok(mut w) = CONTRAST.write() { *w = v; } }
pub fn g_COPULA() -> Py { ensure_init(); COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); if let Ok(mut w) = COPULA.write() { *w = v; } }
pub fn g_DANGER() -> Py { ensure_init(); DANGER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DANGER(v: Py) { ensure_init(); if let Ok(mut w) = DANGER.write() { *w = v; } }
pub fn g_DECLARATIVE_REFERRAL() -> Py { ensure_init(); DECLARATIVE_REFERRAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DECLARATIVE_REFERRAL(v: Py) { ensure_init(); if let Ok(mut w) = DECLARATIVE_REFERRAL.write() { *w = v; } }
pub fn g_DENIES() -> Py { ensure_init(); DENIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENIES(v: Py) { ensure_init(); if let Ok(mut w) = DENIES.write() { *w = v; } }
pub fn g_DESCRIBES() -> Py { ensure_init(); DESCRIBES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DESCRIBES(v: Py) { ensure_init(); if let Ok(mut w) = DESCRIBES.write() { *w = v; } }
pub fn g_DETAIL_STATEMENT() -> Py { ensure_init(); DETAIL_STATEMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETAIL_STATEMENT(v: Py) { ensure_init(); if let Ok(mut w) = DETAIL_STATEMENT.write() { *w = v; } }
pub fn g_DIRECTED_LINES() -> Py { ensure_init(); DIRECTED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DIRECTED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = DIRECTED_LINES.write() { *w = v; } }
pub fn g_DOING() -> Py { ensure_init(); DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOING(v: Py) { ensure_init(); if let Ok(mut w) = DOING.write() { *w = v; } }
pub fn g_DOING_WITH_NO_OBJECT() -> Py { ensure_init(); DOING_WITH_NO_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOING_WITH_NO_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = DOING_WITH_NO_OBJECT.write() { *w = v; } }
pub fn g_DO_LINES() -> Py { ensure_init(); DO_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DO_LINES(v: Py) { ensure_init(); if let Ok(mut w) = DO_LINES.write() { *w = v; } }
pub fn g_DO_NORMAL() -> Py { ensure_init(); DO_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DO_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = DO_NORMAL.write() { *w = v; } }
pub fn g_ECHO_FILLER() -> Py { ensure_init(); ECHO_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ECHO_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = ECHO_FILLER.write() { *w = v; } }
pub fn g_ECHO_WORDS() -> Py { ensure_init(); ECHO_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ECHO_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ECHO_WORDS.write() { *w = v; } }
pub fn g_EMERGENCY_CALL() -> Py { ensure_init(); EMERGENCY_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_CALL(v: Py) { ensure_init(); if let Ok(mut w) = EMERGENCY_CALL.write() { *w = v; } }
pub fn g_EMERGENCY_CALL_OPEN() -> Py { ensure_init(); EMERGENCY_CALL_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_CALL_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = EMERGENCY_CALL_OPEN.write() { *w = v; } }
pub fn g_EMERGENCY_ORDER() -> Py { ensure_init(); EMERGENCY_ORDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_ORDER(v: Py) { ensure_init(); if let Ok(mut w) = EMERGENCY_ORDER.write() { *w = v; } }
pub fn g_ENOUGH_ASK() -> Py { ensure_init(); ENOUGH_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENOUGH_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ENOUGH_ASK.write() { *w = v; } }
pub fn g_ENOUGH_STATED_LINES() -> Py { ensure_init(); ENOUGH_STATED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENOUGH_STATED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ENOUGH_STATED_LINES.write() { *w = v; } }
pub fn g_EXPLANATION_WORDS() -> Py { ensure_init(); EXPLANATION_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPLANATION_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = EXPLANATION_WORDS.write() { *w = v; } }
pub fn g_FEELING_DOUBT_LINES() -> Py { ensure_init(); FEELING_DOUBT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_DOUBT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_DOUBT_LINES.write() { *w = v; } }
pub fn g_FEELING_FAMILY() -> Py { ensure_init(); FEELING_FAMILY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_FAMILY(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_FAMILY.write() { *w = v; } }
pub fn g_FENCED_IN_SENTENCE() -> Py { ensure_init(); FENCED_IN_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FENCED_IN_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = FENCED_IN_SENTENCE.write() { *w = v; } }
pub fn g_FENCE_FILLER() -> Py { ensure_init(); FENCE_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FENCE_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = FENCE_FILLER.write() { *w = v; } }
pub fn g_FIGURE_LINES() -> Py { ensure_init(); FIGURE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE_LINES.write() { *w = v; } }
pub fn g_FIRST_ITSELF() -> Py { ensure_init(); FIRST_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_ITSELF.write() { *w = v; } }
pub fn g_FIX_ASKED_LINES() -> Py { ensure_init(); FIX_ASKED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIX_ASKED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = FIX_ASKED_LINES.write() { *w = v; } }
pub fn g_FIX_ASKED_NOW_LINES() -> Py { ensure_init(); FIX_ASKED_NOW_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIX_ASKED_NOW_LINES(v: Py) { ensure_init(); if let Ok(mut w) = FIX_ASKED_NOW_LINES.write() { *w = v; } }
pub fn g_FLOOR_LINES() -> Py { ensure_init(); FLOOR_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLOOR_LINES(v: Py) { ensure_init(); if let Ok(mut w) = FLOOR_LINES.write() { *w = v; } }
pub fn g_FRAME_AND() -> Py { ensure_init(); FRAME_AND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_AND(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_AND.write() { *w = v; } }
pub fn g_FRONT_LEAD() -> Py { ensure_init(); FRONT_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRONT_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = FRONT_LEAD.write() { *w = v; } }
pub fn g_FRONT_SUBJECT_WORDS() -> Py { ensure_init(); FRONT_SUBJECT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRONT_SUBJECT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FRONT_SUBJECT_WORDS.write() { *w = v; } }
pub fn g_FRONT_THIS() -> Py { ensure_init(); FRONT_THIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRONT_THIS(v: Py) { ensure_init(); if let Ok(mut w) = FRONT_THIS.write() { *w = v; } }
pub fn g_FRONT_WHEN() -> Py { ensure_init(); FRONT_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRONT_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = FRONT_WHEN.write() { *w = v; } }
pub fn g_GO_FROM() -> Py { ensure_init(); GO_FROM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GO_FROM(v: Py) { ensure_init(); if let Ok(mut w) = GO_FROM.write() { *w = v; } }
pub fn g_GRIEF_FLOOR_LINES() -> Py { ensure_init(); GRIEF_FLOOR_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_FLOOR_LINES(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_FLOOR_LINES.write() { *w = v; } }
pub fn g_HANDED_BACK_LINES() -> Py { ensure_init(); HANDED_BACK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANDED_BACK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = HANDED_BACK_LINES.write() { *w = v; } }
pub fn g_HAND_OVER_WORDS() -> Py { ensure_init(); HAND_OVER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_OVER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HAND_OVER_WORDS.write() { *w = v; } }
pub fn g_HANGING() -> Py { ensure_init(); HANGING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANGING(v: Py) { ensure_init(); if let Ok(mut w) = HANGING.write() { *w = v; } }
pub fn g_HARD_NEWS() -> Py { ensure_init(); HARD_NEWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HARD_NEWS(v: Py) { ensure_init(); if let Ok(mut w) = HARD_NEWS.write() { *w = v; } }
pub fn g_HARD_NEWS_LINES() -> Py { ensure_init(); HARD_NEWS_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HARD_NEWS_LINES(v: Py) { ensure_init(); if let Ok(mut w) = HARD_NEWS_LINES.write() { *w = v; } }
pub fn g_HARMLESS() -> Py { ensure_init(); HARMLESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HARMLESS(v: Py) { ensure_init(); if let Ok(mut w) = HARMLESS.write() { *w = v; } }
pub fn g_HEAD_REFUSES() -> Py { ensure_init(); HEAD_REFUSES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAD_REFUSES(v: Py) { ensure_init(); if let Ok(mut w) = HEAD_REFUSES.write() { *w = v; } }
pub fn g_HEDGE_SPAN() -> Py { ensure_init(); HEDGE_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGE_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = HEDGE_SPAN.write() { *w = v; } }
pub fn g_HELPED() -> Py { ensure_init(); HELPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELPED(v: Py) { ensure_init(); if let Ok(mut w) = HELPED.write() { *w = v; } }
pub fn g_HELPED_LINES() -> Py { ensure_init(); HELPED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELPED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = HELPED_LINES.write() { *w = v; } }
pub fn g_HELPS() -> Py { ensure_init(); HELPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELPS(v: Py) { ensure_init(); if let Ok(mut w) = HELPS.write() { *w = v; } }
pub fn g_HOW_MANY_PEOPLE() -> Py { ensure_init(); HOW_MANY_PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MANY_PEOPLE.write() { *w = v; } }
pub fn g_HOW_TO() -> Py { ensure_init(); HOW_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO.write() { *w = v; } }
pub fn g_IMPERATIVES() -> Py { ensure_init(); IMPERATIVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IMPERATIVES(v: Py) { ensure_init(); if let Ok(mut w) = IMPERATIVES.write() { *w = v; } }
pub fn g_IS_A_NUMBER() -> Py { ensure_init(); IS_A_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_A_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = IS_A_NUMBER.write() { *w = v; } }
pub fn g_JUDGED_REFERRAL() -> Py { ensure_init(); JUDGED_REFERRAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JUDGED_REFERRAL(v: Py) { ensure_init(); if let Ok(mut w) = JUDGED_REFERRAL.write() { *w = v; } }
pub fn g_JUST_AGE() -> Py { ensure_init(); JUST_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JUST_AGE(v: Py) { ensure_init(); if let Ok(mut w) = JUST_AGE.write() { *w = v; } }
pub fn g_KNOWING_DOING() -> Py { ensure_init(); KNOWING_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KNOWING_DOING(v: Py) { ensure_init(); if let Ok(mut w) = KNOWING_DOING.write() { *w = v; } }
pub fn g_LATER_CONDITION() -> Py { ensure_init(); LATER_CONDITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LATER_CONDITION(v: Py) { ensure_init(); if let Ok(mut w) = LATER_CONDITION.write() { *w = v; } }
pub fn g_LIST_STEP_LINES() -> Py { ensure_init(); LIST_STEP_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIST_STEP_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LIST_STEP_LINES.write() { *w = v; } }
pub fn g_LONG_SPAN() -> Py { ensure_init(); LONG_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LONG_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = LONG_SPAN.write() { *w = v; } }
pub fn g_LONG_STEP_WORDS() -> Py { ensure_init(); LONG_STEP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LONG_STEP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = LONG_STEP_WORDS.write() { *w = v; } }
pub fn g_LOOK_AND_SEE() -> Py { ensure_init(); LOOK_AND_SEE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOK_AND_SEE(v: Py) { ensure_init(); if let Ok(mut w) = LOOK_AND_SEE.write() { *w = v; } }
pub fn g_LOSS_WORDS() -> Py { ensure_init(); LOSS_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_WORDS.write() { *w = v; } }
pub fn g_METHOD_DOUBT() -> Py { ensure_init(); METHOD_DOUBT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_METHOD_DOUBT(v: Py) { ensure_init(); if let Ok(mut w) = METHOD_DOUBT.write() { *w = v; } }
pub fn g_METHOD_OK_LINES() -> Py { ensure_init(); METHOD_OK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_METHOD_OK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = METHOD_OK_LINES.write() { *w = v; } }
pub fn g_MET_CONDITION_LINES() -> Py { ensure_init(); MET_CONDITION_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MET_CONDITION_LINES(v: Py) { ensure_init(); if let Ok(mut w) = MET_CONDITION_LINES.write() { *w = v; } }
pub fn g_MET_LINES() -> Py { ensure_init(); MET_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MET_LINES(v: Py) { ensure_init(); if let Ok(mut w) = MET_LINES.write() { *w = v; } }
pub fn g_MONEY_LINES() -> Py { ensure_init(); MONEY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_LINES.write() { *w = v; } }
pub fn g_MONEY_NOT_MET() -> Py { ensure_init(); MONEY_NOT_MET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_NOT_MET(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_NOT_MET.write() { *w = v; } }
pub fn g_MONEY_NOUN_WORDS() -> Py { ensure_init(); MONEY_NOUN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_NOUN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_NOUN_WORDS.write() { *w = v; } }
pub fn g_MONEY_TROUBLE_LINES() -> Py { ensure_init(); MONEY_TROUBLE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_TROUBLE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_TROUBLE_LINES.write() { *w = v; } }
pub fn g_MONEY_WORD() -> Py { ensure_init(); MONEY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_WORD.write() { *w = v; } }
pub fn g_MOOD() -> Py { ensure_init(); MOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOOD(v: Py) { ensure_init(); if let Ok(mut w) = MOOD.write() { *w = v; } }
pub fn g_NAMES_A_CLINICIAN() -> Py { ensure_init(); NAMES_A_CLINICIAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_A_CLINICIAN(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_A_CLINICIAN.write() { *w = v; } }
pub fn g_NAMES_A_TRADE() -> Py { ensure_init(); NAMES_A_TRADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_A_TRADE(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_A_TRADE.write() { *w = v; } }
pub fn g_NAMES_NOTHING() -> Py { ensure_init(); NAMES_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_NOTHING.write() { *w = v; } }
pub fn g_NEEDS_A_CALL() -> Py { ensure_init(); NEEDS_A_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEEDS_A_CALL(v: Py) { ensure_init(); if let Ok(mut w) = NEEDS_A_CALL.write() { *w = v; } }
pub fn g_NEGATED() -> Py { ensure_init(); NEGATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATED(v: Py) { ensure_init(); if let Ok(mut w) = NEGATED.write() { *w = v; } }
pub fn g_NEGATED_BEFORE() -> Py { ensure_init(); NEGATED_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATED_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = NEGATED_BEFORE.write() { *w = v; } }
pub fn g_NEGATIVE_OPEN() -> Py { ensure_init(); NEGATIVE_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = NEGATIVE_OPEN.write() { *w = v; } }
pub fn g_NEUTRAL_ASK_LINES() -> Py { ensure_init(); NEUTRAL_ASK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEUTRAL_ASK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = NEUTRAL_ASK_LINES.write() { *w = v; } }
pub fn g_NOTHING_LINES() -> Py { ensure_init(); NOTHING_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_LINES(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_LINES.write() { *w = v; } }
pub fn g_NOTHING_WARNED_LINES() -> Py { ensure_init(); NOTHING_WARNED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_WARNED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_WARNED_LINES.write() { *w = v; } }
pub fn g_NOTHING_WARNED_NORMAL() -> Py { ensure_init(); NOTHING_WARNED_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_WARNED_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_WARNED_NORMAL.write() { *w = v; } }
pub fn g_NOT_A_STEP_TO_WATCH() -> Py { ensure_init(); NOT_A_STEP_TO_WATCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STEP_TO_WATCH(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_STEP_TO_WATCH.write() { *w = v; } }
pub fn g_NOT_A_STEP_TO_WATCH_WORDS() -> Py { ensure_init(); NOT_A_STEP_TO_WATCH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STEP_TO_WATCH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_STEP_TO_WATCH_WORDS.write() { *w = v; } }
pub fn g_NOT_MONEY_PAY() -> Py { ensure_init(); NOT_MONEY_PAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_MONEY_PAY(v: Py) { ensure_init(); if let Ok(mut w) = NOT_MONEY_PAY.write() { *w = v; } }
pub fn g_NOT_TOLD_TO() -> Py { ensure_init(); NOT_TOLD_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TOLD_TO(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TOLD_TO.write() { *w = v; } }
pub fn g_NO_LINES() -> Py { ensure_init(); NO_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_LINES(v: Py) { ensure_init(); if let Ok(mut w) = NO_LINES.write() { *w = v; } }
pub fn g_NO_STATED_LINES() -> Py { ensure_init(); NO_STATED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_STATED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = NO_STATED_LINES.write() { *w = v; } }
pub fn g_OFFER_QUESTION() -> Py { ensure_init(); OFFER_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFFER_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = OFFER_QUESTION.write() { *w = v; } }
pub fn g_OFFER_TAKEN_LINES() -> Py { ensure_init(); OFFER_TAKEN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFFER_TAKEN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = OFFER_TAKEN_LINES.write() { *w = v; } }
pub fn g_OK_LINES() -> Py { ensure_init(); OK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = OK_LINES.write() { *w = v; } }
pub fn g_ONE_OPTION() -> Py { ensure_init(); ONE_OPTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONE_OPTION(v: Py) { ensure_init(); if let Ok(mut w) = ONE_OPTION.write() { *w = v; } }
pub fn g_ONE_THING() -> Py { ensure_init(); ONE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONE_THING(v: Py) { ensure_init(); if let Ok(mut w) = ONE_THING.write() { *w = v; } }
pub fn g_ONE_THING_ADVERB() -> Py { ensure_init(); ONE_THING_ADVERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONE_THING_ADVERB(v: Py) { ensure_init(); if let Ok(mut w) = ONE_THING_ADVERB.write() { *w = v; } }
pub fn g_ONE_TURN_LINES() -> Py { ensure_init(); ONE_TURN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONE_TURN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ONE_TURN_LINES.write() { *w = v; } }
pub fn g_ONE_TURN_PLAIN_LINES() -> Py { ensure_init(); ONE_TURN_PLAIN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONE_TURN_PLAIN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ONE_TURN_PLAIN_LINES.write() { *w = v; } }
pub fn g_ORDER_CONTINUES() -> Py { ensure_init(); ORDER_CONTINUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_CONTINUES(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_CONTINUES.write() { *w = v; } }
pub fn g_ORDINARY_NORMAL() -> Py { ensure_init(); ORDINARY_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDINARY_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = ORDINARY_NORMAL.write() { *w = v; } }
pub fn g_OVER_VERB() -> Py { ensure_init(); OVER_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OVER_VERB(v: Py) { ensure_init(); if let Ok(mut w) = OVER_VERB.write() { *w = v; } }
pub fn g_OWED() -> Py { ensure_init(); OWED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWED(v: Py) { ensure_init(); if let Ok(mut w) = OWED.write() { *w = v; } }
pub fn g_OWED_LINES() -> Py { ensure_init(); OWED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = OWED_LINES.write() { *w = v; } }
pub fn g_OWN_DOING() -> Py { ensure_init(); OWN_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_DOING(v: Py) { ensure_init(); if let Ok(mut w) = OWN_DOING.write() { *w = v; } }
pub fn g_OWN_LINES() -> Py { ensure_init(); OWN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = OWN_LINES.write() { *w = v; } }
pub fn g_OWN_PATTERN() -> Py { ensure_init(); OWN_PATTERN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_PATTERN(v: Py) { ensure_init(); if let Ok(mut w) = OWN_PATTERN.write() { *w = v; } }
pub fn g_PEOPLE_CARE() -> Py { ensure_init(); PEOPLE_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_CARE(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE_CARE.write() { *w = v; } }
pub fn g_PERSON_CALL() -> Py { ensure_init(); PERSON_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_CALL(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_CALL.write() { *w = v; } }
pub fn g_PERSON_ORDINARY() -> Py { ensure_init(); PERSON_ORDINARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_ORDINARY(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_ORDINARY.write() { *w = v; } }
pub fn g_POOLS() -> Py { ensure_init(); POOLS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POOLS(v: Py) { ensure_init(); if let Ok(mut w) = POOLS.write() { *w = v; } }
pub fn g_PUNT_CHAT() -> Py { ensure_init(); PUNT_CHAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUNT_CHAT(v: Py) { ensure_init(); if let Ok(mut w) = PUNT_CHAT.write() { *w = v; } }
pub fn g_PUNT_OFFER() -> Py { ensure_init(); PUNT_OFFER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUNT_OFFER(v: Py) { ensure_init(); if let Ok(mut w) = PUNT_OFFER.write() { *w = v; } }
pub fn g_PUNT_START() -> Py { ensure_init(); PUNT_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUNT_START(v: Py) { ensure_init(); if let Ok(mut w) = PUNT_START.write() { *w = v; } }
pub fn g_QUESTION_BACK_LINES() -> Py { ensure_init(); QUESTION_BACK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_BACK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_BACK_LINES.write() { *w = v; } }
pub fn g_QUESTION_FRAME() -> Py { ensure_init(); QUESTION_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_FRAME.write() { *w = v; } }
pub fn g_READABLE() -> Py { ensure_init(); READABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_READABLE(v: Py) { ensure_init(); if let Ok(mut w) = READABLE.write() { *w = v; } }
pub fn g_REASSURE() -> Py { ensure_init(); REASSURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REASSURE(v: Py) { ensure_init(); if let Ok(mut w) = REASSURE.write() { *w = v; } }
pub fn g_RECAP_FILLER() -> Py { ensure_init(); RECAP_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECAP_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = RECAP_FILLER.write() { *w = v; } }
pub fn g_REFER() -> Py { ensure_init(); REFER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFER(v: Py) { ensure_init(); if let Ok(mut w) = REFER.write() { *w = v; } }
pub fn g_REFER_VERB() -> Py { ensure_init(); REFER_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFER_VERB(v: Py) { ensure_init(); if let Ok(mut w) = REFER_VERB.write() { *w = v; } }
pub fn g_RELATIVE() -> Py { ensure_init(); RELATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RELATIVE(v: Py) { ensure_init(); if let Ok(mut w) = RELATIVE.write() { *w = v; } }
pub fn g_RESTATED() -> Py { ensure_init(); RESTATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RESTATED(v: Py) { ensure_init(); if let Ok(mut w) = RESTATED.write() { *w = v; } }
pub fn g_RULED_BY_THE_COMPOSER() -> Py { ensure_init(); RULED_BY_THE_COMPOSER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULED_BY_THE_COMPOSER(v: Py) { ensure_init(); if let Ok(mut w) = RULED_BY_THE_COMPOSER.write() { *w = v; } }
pub fn g_RUN() -> Py { ensure_init(); RUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RUN(v: Py) { ensure_init(); if let Ok(mut w) = RUN.write() { *w = v; } }
pub fn g_SAID_YES() -> Py { ensure_init(); SAID_YES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_YES(v: Py) { ensure_init(); if let Ok(mut w) = SAID_YES.write() { *w = v; } }
pub fn g_SAYS_NORMAL() -> Py { ensure_init(); SAYS_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_NORMAL.write() { *w = v; } }
pub fn g_SEE() -> Py { ensure_init(); SEE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEE(v: Py) { ensure_init(); if let Ok(mut w) = SEE.write() { *w = v; } }
pub fn g_SEE_ANYWAY_LINES() -> Py { ensure_init(); SEE_ANYWAY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEE_ANYWAY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = SEE_ANYWAY_LINES.write() { *w = v; } }
pub fn g_SEE_NONE_LINES() -> Py { ensure_init(); SEE_NONE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEE_NONE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = SEE_NONE_LINES.write() { *w = v; } }
pub fn g_SEE_NOW_LINES() -> Py { ensure_init(); SEE_NOW_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEE_NOW_LINES(v: Py) { ensure_init(); if let Ok(mut w) = SEE_NOW_LINES.write() { *w = v; } }
pub fn g_SEE_STATED_LINES() -> Py { ensure_init(); SEE_STATED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEE_STATED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = SEE_STATED_LINES.write() { *w = v; } }
pub fn g_SIGN() -> Py { ensure_init(); SIGN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIGN(v: Py) { ensure_init(); if let Ok(mut w) = SIGN.write() { *w = v; } }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); SOMEONE_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSE.write() { *w = v; } }
pub fn g_SOON_LINES() -> Py { ensure_init(); SOON_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOON_LINES(v: Py) { ensure_init(); if let Ok(mut w) = SOON_LINES.write() { *w = v; } }
pub fn g_SPEAK_UP() -> Py { ensure_init(); SPEAK_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAK_UP(v: Py) { ensure_init(); if let Ok(mut w) = SPEAK_UP.write() { *w = v; } }
pub fn g_SPEAK_VERBS() -> Py { ensure_init(); SPEAK_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAK_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = SPEAK_VERBS.write() { *w = v; } }
pub fn g_SPLIT() -> Py { ensure_init(); SPLIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPLIT(v: Py) { ensure_init(); if let Ok(mut w) = SPLIT.write() { *w = v; } }
pub fn g_STATED_ENOUGH() -> Py { ensure_init(); STATED_ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATED_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = STATED_ENOUGH.write() { *w = v; } }
pub fn g_STATED_LONGEST() -> Py { ensure_init(); STATED_LONGEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATED_LONGEST(v: Py) { ensure_init(); if let Ok(mut w) = STATED_LONGEST.write() { *w = v; } }
pub fn g_STATED_NO() -> Py { ensure_init(); STATED_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATED_NO(v: Py) { ensure_init(); if let Ok(mut w) = STATED_NO.write() { *w = v; } }
pub fn g_STATES_A_REFERRAL() -> Py { ensure_init(); STATES_A_REFERRAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATES_A_REFERRAL(v: Py) { ensure_init(); if let Ok(mut w) = STATES_A_REFERRAL.write() { *w = v; } }
pub fn g_STEP() -> Py { ensure_init(); STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP(v: Py) { ensure_init(); if let Ok(mut w) = STEP.write() { *w = v; } }
pub fn g_STEP_ALREADY_FIRST_LINES() -> Py { ensure_init(); STEP_ALREADY_FIRST_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_ALREADY_FIRST_LINES(v: Py) { ensure_init(); if let Ok(mut w) = STEP_ALREADY_FIRST_LINES.write() { *w = v; } }
pub fn g_STEP_DETAIL_LINES() -> Py { ensure_init(); STEP_DETAIL_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_DETAIL_LINES(v: Py) { ensure_init(); if let Ok(mut w) = STEP_DETAIL_LINES.write() { *w = v; } }
pub fn g_STEP_KEPT_LINES() -> Py { ensure_init(); STEP_KEPT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_KEPT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = STEP_KEPT_LINES.write() { *w = v; } }
pub fn g_STEP_LINES() -> Py { ensure_init(); STEP_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_LINES(v: Py) { ensure_init(); if let Ok(mut w) = STEP_LINES.write() { *w = v; } }
pub fn g_STEP_LONG() -> Py { ensure_init(); STEP_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_LONG(v: Py) { ensure_init(); if let Ok(mut w) = STEP_LONG.write() { *w = v; } }
pub fn g_STEP_ONE_LINES() -> Py { ensure_init(); STEP_ONE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_ONE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = STEP_ONE_LINES.write() { *w = v; } }
pub fn g_STEP_WORDS() -> Py { ensure_init(); STEP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = STEP_WORDS.write() { *w = v; } }
pub fn g_STILL_GOING() -> Py { ensure_init(); STILL_GOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_GOING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_GOING.write() { *w = v; } }
pub fn g_STOP_DOING() -> Py { ensure_init(); STOP_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP_DOING(v: Py) { ensure_init(); if let Ok(mut w) = STOP_DOING.write() { *w = v; } }
pub fn g_SUBJECT_OPEN() -> Py { ensure_init(); SUBJECT_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = SUBJECT_OPEN.write() { *w = v; } }
pub fn g_SYMPATHY() -> Py { ensure_init(); SYMPATHY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SYMPATHY(v: Py) { ensure_init(); if let Ok(mut w) = SYMPATHY.write() { *w = v; } }
pub fn g_TARGET_SHAPE() -> Py { ensure_init(); TARGET_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TARGET_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = TARGET_SHAPE.write() { *w = v; } }
pub fn g_TESTING() -> Py { ensure_init(); TESTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TESTING(v: Py) { ensure_init(); if let Ok(mut w) = TESTING.write() { *w = v; } }
pub fn g_TEST_LINES() -> Py { ensure_init(); TEST_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TEST_LINES(v: Py) { ensure_init(); if let Ok(mut w) = TEST_LINES.write() { *w = v; } }
pub fn g_THEN_OPEN() -> Py { ensure_init(); THEN_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEN_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = THEN_OPEN.write() { *w = v; } }
pub fn g_THE_PLAN_CAN_HURT_LINES() -> Py { ensure_init(); THE_PLAN_CAN_HURT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_PLAN_CAN_HURT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = THE_PLAN_CAN_HURT_LINES.write() { *w = v; } }
pub fn g_THE_PLEA() -> Py { ensure_init(); THE_PLEA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_PLEA(v: Py) { ensure_init(); if let Ok(mut w) = THE_PLEA.write() { *w = v; } }
pub fn g_THING_CALL() -> Py { ensure_init(); THING_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_CALL(v: Py) { ensure_init(); if let Ok(mut w) = THING_CALL.write() { *w = v; } }
pub fn g_THING_ORDINARY() -> Py { ensure_init(); THING_ORDINARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_ORDINARY(v: Py) { ensure_init(); if let Ok(mut w) = THING_ORDINARY.write() { *w = v; } }
pub fn g_THING_WORDS() -> Py { ensure_init(); THING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = THING_WORDS.write() { *w = v; } }
pub fn g_THROW_AWAY() -> Py { ensure_init(); THROW_AWAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THROW_AWAY(v: Py) { ensure_init(); if let Ok(mut w) = THROW_AWAY.write() { *w = v; } }
pub fn g_THROW_AWAY_LINES() -> Py { ensure_init(); THROW_AWAY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THROW_AWAY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = THROW_AWAY_LINES.write() { *w = v; } }
pub fn g_TIME_NOW() -> Py { ensure_init(); TIME_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_NOW(v: Py) { ensure_init(); if let Ok(mut w) = TIME_NOW.write() { *w = v; } }
pub fn g_TIPS_AGAIN() -> Py { ensure_init(); TIPS_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIPS_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = TIPS_AGAIN.write() { *w = v; } }
pub fn g_TRADE() -> Py { ensure_init(); TRADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRADE(v: Py) { ensure_init(); if let Ok(mut w) = TRADE.write() { *w = v; } }
pub fn g_TRIM() -> Py { ensure_init(); TRIM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIM(v: Py) { ensure_init(); if let Ok(mut w) = TRIM.write() { *w = v; } }
pub fn g_TRIM_INSTRUMENT() -> Py { ensure_init(); TRIM_INSTRUMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIM_INSTRUMENT(v: Py) { ensure_init(); if let Ok(mut w) = TRIM_INSTRUMENT.write() { *w = v; } }
pub fn g_UNREADABLE() -> Py { ensure_init(); UNREADABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNREADABLE(v: Py) { ensure_init(); if let Ok(mut w) = UNREADABLE.write() { *w = v; } }
pub fn g_URGENT_CALL() -> Py { ensure_init(); URGENT_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_URGENT_CALL(v: Py) { ensure_init(); if let Ok(mut w) = URGENT_CALL.write() { *w = v; } }
pub fn g_URGENT_TIME() -> Py { ensure_init(); URGENT_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_URGENT_TIME(v: Py) { ensure_init(); if let Ok(mut w) = URGENT_TIME.write() { *w = v; } }
pub fn g_WHEN() -> Py { ensure_init(); WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = WHEN.write() { *w = v; } }
pub fn g_WHEN_ASK() -> Py { ensure_init(); WHEN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_ASK.write() { *w = v; } }
pub fn g_WHEN_CALL_LINES() -> Py { ensure_init(); WHEN_CALL_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_CALL_LINES(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_CALL_LINES.write() { *w = v; } }
pub fn g_WHEN_LINES() -> Py { ensure_init(); WHEN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_LINES.write() { *w = v; } }
pub fn g_WORRIED_ASK() -> Py { ensure_init(); WORRIED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORRIED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WORRIED_ASK.write() { *w = v; } }
pub fn g_WORRYING_ASK() -> Py { ensure_init(); WORRYING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORRYING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WORRYING_ASK.write() { *w = v; } }
pub fn g_YES_LINES() -> Py { ensure_init(); YES_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YES_LINES(v: Py) { ensure_init(); if let Ok(mut w) = YES_LINES.write() { *w = v; } }
pub fn g_YES_SAID_LINES() -> Py { ensure_init(); YES_SAID_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YES_SAID_LINES(v: Py) { ensure_init(); if let Ok(mut w) = YES_SAID_LINES.write() { *w = v; } }
pub fn g__ACK_PATTERN() -> Py { ensure_init(); _ACK_PATTERN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ACK_PATTERN(v: Py) { ensure_init(); if let Ok(mut w) = _ACK_PATTERN.write() { *w = v; } }
pub fn g__AFTER_A_NOUN() -> Py { ensure_init(); _AFTER_A_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AFTER_A_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = _AFTER_A_NOUN.write() { *w = v; } }
pub fn g__AN_OFFER() -> Py { ensure_init(); _AN_OFFER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_OFFER(v: Py) { ensure_init(); if let Ok(mut w) = _AN_OFFER.write() { *w = v; } }
pub fn g__AN_OPENER() -> Py { ensure_init(); _AN_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = _AN_OPENER.write() { *w = v; } }
pub fn g__ASKS_ABOUT_A_STEP() -> Py { ensure_init(); _ASKS_ABOUT_A_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_ABOUT_A_STEP(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_ABOUT_A_STEP.write() { *w = v; } }
pub fn g__ASKS_FOR_THE_STEP() -> Py { ensure_init(); _ASKS_FOR_THE_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_THE_STEP(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_FOR_THE_STEP.write() { *w = v; } }
pub fn g__A_BODY_WORD() -> Py { ensure_init(); _A_BODY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BODY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_BODY_WORD.write() { *w = v; } }
pub fn g__A_DASH() -> Py { ensure_init(); _A_DASH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DASH(v: Py) { ensure_init(); if let Ok(mut w) = _A_DASH.write() { *w = v; } }
pub fn g__A_FIX_WORD() -> Py { ensure_init(); _A_FIX_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FIX_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_FIX_WORD.write() { *w = v; } }
pub fn g__A_NUMBER() -> Py { ensure_init(); _A_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = _A_NUMBER.write() { *w = v; } }
pub fn g__A_RANGE_OF_ITS_OWN() -> Py { ensure_init(); _A_RANGE_OF_ITS_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_RANGE_OF_ITS_OWN(v: Py) { ensure_init(); if let Ok(mut w) = _A_RANGE_OF_ITS_OWN.write() { *w = v; } }
pub fn g__A_REFERRAL_OUT() -> Py { ensure_init(); _A_REFERRAL_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_REFERRAL_OUT(v: Py) { ensure_init(); if let Ok(mut w) = _A_REFERRAL_OUT.write() { *w = v; } }
pub fn g__A_STOP() -> Py { ensure_init(); _A_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_STOP(v: Py) { ensure_init(); if let Ok(mut w) = _A_STOP.write() { *w = v; } }
pub fn g__BODY_PARTS() -> Py { ensure_init(); _BODY_PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BODY_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = _BODY_PARTS.write() { *w = v; } }
pub fn g__FILL() -> Py { ensure_init(); _FILL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FILL(v: Py) { ensure_init(); if let Ok(mut w) = _FILL.write() { *w = v; } }
pub fn g__FILLER() -> Py { ensure_init(); _FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FILLER(v: Py) { ensure_init(); if let Ok(mut w) = _FILLER.write() { *w = v; } }
pub fn g__INVITATION() -> Py { ensure_init(); _INVITATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__INVITATION(v: Py) { ensure_init(); if let Ok(mut w) = _INVITATION.write() { *w = v; } }
pub fn g__LEAD_FILLER() -> Py { ensure_init(); _LEAD_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LEAD_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = _LEAD_FILLER.write() { *w = v; } }
pub fn g__LIST_JOIN() -> Py { ensure_init(); _LIST_JOIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LIST_JOIN(v: Py) { ensure_init(); if let Ok(mut w) = _LIST_JOIN.write() { *w = v; } }
pub fn g__NOT_A_SIGN_WORD() -> Py { ensure_init(); _NOT_A_SIGN_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SIGN_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_SIGN_WORD.write() { *w = v; } }
pub fn g__ONE_THING_ADVERB() -> Py { ensure_init(); _ONE_THING_ADVERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ONE_THING_ADVERB(v: Py) { ensure_init(); if let Ok(mut w) = _ONE_THING_ADVERB.write() { *w = v; } }
pub fn g__STEP_OPENERS() -> Py { ensure_init(); _STEP_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STEP_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = _STEP_OPENERS.write() { *w = v; } }
pub fn g__SYMPTOM_WORDS() -> Py { ensure_init(); _SYMPTOM_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SYMPTOM_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _SYMPTOM_WORDS.write() { *w = v; } }
pub fn g__WORD() -> Py { ensure_init(); _WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WORD(v: Py) { ensure_init(); if let Ok(mut w) = _WORD.write() { *w = v; } }
pub fn g__a_thing_stopped() -> Py { ensure_init(); _a_thing_stopped.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__a_thing_stopped(v: Py) { ensure_init(); if let Ok(mut w) = _a_thing_stopped.write() { *w = v; } }
pub fn g__reads_as_good_news() -> Py { ensure_init(); _reads_as_good_news.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__reads_as_good_news(v: Py) { ensure_init(); if let Ok(mut w) = _reads_as_good_news.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A bare yes/no question about the thing the bot just explained, answered from that answer.\n\n\"I think my knee clicks going up the stairs\" gets the harmless-or-not answer. Then \"is that\nbad\" arrives, and the bot says \"Alright then — good luck with it.\": the person asked whether\nwhat it just told them matters, and the reply hands them the door. \"is that normal\" is worse\nstill, because it gets \"I'm not sure about that one — a search\", which disowns the very thing\nthe bot explained two seconds ago.\n\nThe answer is already in the bot's own last line, so this reads it. Three shapes come out of\nthat reading, and each has its own pool:\n\n- the reply called the thing ordinary (\"usually harmless\", \"that is normal\") -> say no, and say\n  what would change the answer;\n- the reply said to get someone in, with no condition on it -> say yes;\n- the reply gave a job to do and a \"if it carries on, get someone in\" after it -> say it is a\n  job rather than a danger, and point at the condition.\n\nNothing is invented about the subject: every line says what the previous answer already said,\nand what would change it. It stands aside for anything that is not bare (\"is my knee normal\"\nnames its own subject and belongs to the knowledge base), for a previous turn that was not a\nknowledge answer (a safety line's referral is not a verdict to re-read, and \"should i be\nworried\" after one must not be told \"no\"), for a previous turn with no verdict in it, and for\nits own answers, so a person asking twice does not get the same sentence twice.\n");
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
    s__reads_as_good_news(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    let __try6 = exc::catch(|| {
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
    s__a_thing_stopped(Py::None);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    s_BARE(stdlib_call("re", "compile", &[py_str("^(?:and |so )?(?:is|was|are|were|does|do|did) (?:that|this|it|all of that|all that)(?: (?:really|actually|very|too|that))?(?: (?:bad|normal|serious|dangerous|ok|okay|alright|all right|acceptable|fine|safe|common|weird|odd|strange|a problem|something to worry about|anything to worry about|worth worrying about|(?:something|anything|a thing) (?:i should|to|that i should|worth) worry(?:ing)? about|something (?:i should|to) be worried about))(?: then| though| or not)?$|^(?:and |so )?should i (?:be )?(?:worried|worry|worrying)(?: then| though)?$|^(?:and |so )?when (?:should|do|would) i (?:be )?(?:worried|worry|worrying)(?: about (?:it|that|this))?(?: then| though)?$|^(?:and |so )?when (?:should|do) i (?:start|need) (?:to )?(?:be )?worry(?:ing)?$|^(?:and |so )?is (?:that|it|this) (?:worrying|worrisome|a worry)(?: then| though)?$|^(?:and |so )?(?:do|will|should|would) i (?:really |actually |honestly )?(?:need|have) to (?:be )?worr(?:y|ied)(?: about (?:it|that|this))?(?: then| though)?$|^(?:and |so )?(?:is|was) (?:that|it|this)(?: (?:going to|gonna))? (?:get worse|getting worse|spread|serious)$|^(?:and |so )?(?:does|do|did) (?:that|this|it) matter(?: then| though)?$|^(?:and |so )?(?:iz |is |s |z )?(?:that|it|this) (?:ok|okay|fine|alright|bad|normal|serious)(?: then| though| or not)?$|^(?:and |so |or )?(?:is|was) (?:that|it|this) (?:just|only) (?:old age|age|a phase|a stage|the age|his age|her age|their age|tiredness|the tiredness)\\??$|^(?:and |so )?is (?:that|it|this) (?:a|much of a) (?:problem|worry|issue)(?: then| though)?$|^(?:and |so )?(?:is|was|would|will|can|could) (?:that|this|it) (?:be )?(?:bad|harmful|dangerous|unsafe|alright|ok|okay) for (?:me|us)(?: then| though)?$|^(?:and |so )?(?:is|was|would|will|can|could) (?:that|this|it) (?:be )?(?:harmful|dangerous|unsafe)(?: then| though)?$|^(?:and |so )?(?:will|would|could) (?:that|this|it) hurt (?:me|us)(?: then| though)?$|^(?:and |so )?(?:does|do|did|will) (?:that|this|it) (?:hurt|harmed|harm|make me ill|make me sick)(?: then| though)?$|^(?:and |so )?(?:am|was) i (?:ok|okay|alright|all right|normal|going mad|going crazy|losing it|losing my mind)(?: then| though)?$|^(?:and |so )?(?:do|would|will) i (?:really |actually )?(?:need|have) to (?:be )?worr(?:y|ied)(?: about (?:it|that|this))?(?: then| though)?$|^(?:and |so )?(?:should|do) i need to worry(?: then| though)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_PEOPLE(py_tuple(vec![py_str("common"), py_str("for most people"), py_str("for most adults"), py_str("for most children"), py_str("for most babies"), py_str("usual for"), py_str("is usual")]));
    s_DENIES(py_tuple(vec![py_str("harmless"), py_str("nothing to worry"), py_str("not serious"), py_str("nothing serious"), py_str("no need to"), py_str("nothing to be")]));
    s_HARMLESS(py_tuple(vec![py_str("harmless"), py_str("not serious"), py_str("nothing serious"), py_str("nothing to worry"), py_str("nothing to be"), py_str("no need to"), py_str("needs nothing doing"), py_str("does not need")]));
    s_SAYS_NORMAL(py_tuple(vec![py_str("normal"), py_str("nothing to worry"), py_str("not serious"), py_str("nothing serious"), py_str("nothing to be"), py_str("no need to"), py_str("harmless")]));
    s_REASSURE(py_tuple(vec![py_str("harmless"), py_str("nothing to worry"), py_str("not serious"), py_str("nothing serious"), py_str("usually"), py_str("normal"), py_str("no need to"), py_str("should settle"), py_str("usually settles"), py_str("will settle"), py_str("goes away"), py_str("just air"), py_str("nothing to be"), py_str("settles"), py_str("settle down"), py_str("clears up"), py_str("clears by itself"), py_str("goes by itself"), py_str("gets better on its own"), py_str("heals on its own"), py_str("resolves on its own"), py_str("does the job on its own"), py_str("is muscular"), py_str("is common"), py_str("common"), py_str("common in"), py_str("common at"), py_str("common for"), py_str("quite common"), py_str("for most people"), py_str("for most adults"), py_str("for most children"), py_str("for most babies"), py_str("usual for"), py_str("is usual")]));
    s_CARE(py_tuple(vec![py_str("get it checked"), py_str("get checked"), py_str("get it looked at"), py_str("get it seen"), py_str("see a doctor"), py_str("see your gp"), py_str("ring 111"), py_str("call 111"), py_str("999"), py_str("a&e"), py_str("get someone in"), py_str("get an engineer"), py_str("get a heating engineer"), py_str("call an engineer"), py_str("engineer out"), py_str("call out"), py_str("straight away"), py_str("as soon as you can"), py_str("urgent"), py_str("needs an engineer"), py_str("is an engineer's job"), py_str("stop using it"), py_str("stop and"), py_str("unplug"), py_str("turn it off"), py_str("switch it off"), py_str("turn the water off"), py_str("stop the cycle"), py_str("leave it off"), py_str("ring your vet"), py_str("ring the vet"), py_str("call your vet"), py_str("call the vet"), py_str("see a vet"), py_str("see your vet"), py_str("get to the vet"), py_str("vet today"), py_str("vet straight away"), py_str("ask your vet"), py_str("your vet will want"), py_str("phone the vet"), py_str("phone your vet"), py_str("gp visit"), py_str("worth a hearing test"), py_str("a referral"), py_str("for a referral"), py_str("health visitor"), py_str("111 line"), py_str("worth seeing"), py_str("worth getting looked at"), py_str("see a gp"), py_str("see the gp"), py_str("ring the gp"), py_str("phone the gp"), py_str("see a nurse"), py_str("ring the surgery"), py_str("call the surgery"), py_str("phone the surgery"), py_str("walk-in centre"), py_str("minor injuries"), py_str("out of hours"), py_str("urgent care"), py_str("111 line"), py_str("nhs 111"), py_str("111 will"), py_str("health visitor"), py_str("out of hours service"), py_str("urgent treatment centre"), py_str("ask your midwife"), py_str("ring your midwife"), py_str("worth a gp visit"), py_str("gp visit"), py_str("gp appointment"), py_str("doctor's appointment")]));
    s_CONDITIONAL(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|once|when) \\w+[^.]{0,80}$")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITIONAL_AFTER(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|once|when|whenever)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HARD_NEWS_LINES(py_tuple(vec![py_str("There is no verdict to give on that one - some things are just hard, and that is not a judgement on you."), py_str("I would not put that on a scale. What happened is heavy enough without working out where it sits."), py_str("That is not something with a good or bad answer. It is a hard thing to carry, and you are allowed to just feel it.")]));
    s_HARD_NEWS(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|death|passed away|funeral|buried|cremated|hospice|cancer|leuka?emia|lymphoma|myeloma|tumou?r|terminal|palliative|chemo|radiotherapy|miscarriage|miscarried|stillborn|dementia|alzheimers|motor neurone|multiple sclerosis|parkinsons|copd|kidney failure|heart failure|liver failure|stroke|heart attack|sepsis|given months|not going to make it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SYMPATHY(stdlib_call("re", "compile", &[py_str("\\b(?:i'm sorry|im sorry|so sorry|really sorry|sorry to hear|sorry for your loss|sorry about (?:your|the)|sounds rough|that sounds rough|a real loss|how awful|that's awful|that is awful|how sad|that's sad|that is sad|frightening|scary|a lot to carry|no wonder|that's heavy|that is heavy|nothing about that is a fault|hardest days?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARATIVE(stdlib_call("re", "compile", &[py_str("\\b(?:sooner|earlier|quicker|faster|more so|especially|particularly|above all)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_URGENT_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|now|straight away|right away|immediately|as soon as|first thing|urgent(?:ly)?|within \\d+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SYMPTOM_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:skin|rash|itch|itching|eyes?|ears?|throat|chest|stomach|belly|temperature|fever|vomit|vomiting|sick|diarrhoea|diarrhea|feeding|feeding|breath|breathing|breathless|cough|poo|wee|blood|pain|hurts?|hurting|swelling|swollen|knee|joint|joints|mouth|teeth|tooth|napp(?:y|ies)|nappy|wet(?:s|ting)? the bed|constipated|constipation|gums?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_THEY_KNOW(stdlib_call("re", "compile", &[py_str("\\b(?:mums?|moms?|mother|father|dads?|son|daughter|kids?|child(?:ren)?|wife|husband|partner|girlfriend|boyfriend|friend|friends|neighbours?|grans?|granny|nan|nana|grandma|granddad|grandad|grandpa|sister|brother|aunt|uncle|cousin|bab(?:y|ies)|in[- ]laws?|mother in law|father in law)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RELATION_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:lonely|loneliness|company|visit|visits|visiting|ring|rings|rang|call|calls|called|speak|speaks|spoke|spoken|talk|talks|talked|talking|quiet|withdrawn|alone|on (?:his|her|their|its) own|lives? on (?:his|her|their) own|arguments?|arguing|row|rows|falling out|fallen out|ignores?|ignoring|does not listen|doesnt listen|wont listen|wont say|wont go out|does not go out|trust|trusts|jealous|suspicious|accuses?|accusing|controlling|keeps? checking|checking (?:my|his|her|their)|doesnt go out|birthday|anniversary|seen (?:him|her|them)|see (?:him|her|them))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_ORDINARY(py_tuple(vec![py_str("There is nothing there to watch for going wrong: it is an ordinary stretch, and the answer's own first step is what moves it. Do that for a few weeks - if it is the same after that, say it to them plainly rather than sitting on it."), py_str("Nothing in that is a fault on either side. The small thing in the answer, done consistently, is what changes it, and if nothing has changed after a few weeks that is the point to say something out loud.")]));
    s_PERSON_CALL(py_tuple(vec![py_str("Yes - take up the first half of what I said now, and the second half is what to act on: if it is still like that after a few weeks, do the part about talking to somebody properly."), py_str("Yes, that one is worth acting on rather than watching. Start with the simple step in the answer, and if nothing has moved in a few weeks, take the next step it names.")]));
    s_NO_LINES(py_tuple(vec![py_str("Nothing in that sounded like a warning sign. What would change it is the thing itself changing — getting worse, or not settling — and then it's worth getting looked at."), py_str("That one is the ordinary kind, so no. Watch it rather than worry about it: if it's still the same in a week or two, or it gets worse, get someone to look at it."), py_str("No — the part you asked about is the harmless kind. What to keep an eye on is change rather than the thing itself.")]));
    s_BOTH_LINES(py_tuple(vec![py_str("It's the common kind rather than a warning sign — and the check is still worth doing."), py_str("Common enough, so no panic. Getting it looked at is worth doing all the same: that is what catches the less usual ones."), py_str("Not a warning sign on its own, no. Common enough that mentioning it is enough rather than rushing — do that, and say what they tell you.")]));
    s_DO_LINES(py_tuple(vec![py_str("It's a job rather than a danger. Do it and see whether it holds — if it comes back, that's the point where it stops being a DIY thing."), py_str("Not bad in itself, but not something to leave alone either. Do the job properly, and if it comes back after that, get someone in."), py_str("Worth doing rather than worrying about. Sort it out; if it keeps coming back, that's when it's somebody else's job.")]));
    s_FIX_ASKED_LINES(py_tuple(vec![py_str("Give it a day or two and see whether it holds. If the same thing comes back, the step was not the fix and it wants a proper look."), py_str("Do it and give it a few days. Coming back after that is the sign it is not something the step sorts out on its own."), py_str("Try it and watch what happens next. If it stays away, that was the whole of it; if it does not, the thing behind it needs seeing to.")]));
    s_FIX_ASKED_NOW_LINES(py_tuple(vec![py_str("Do it now and check it the way you checked before. If nothing has changed at all, that step was not the fix and the next thing to look at is further in."), py_str("Try it straight away. If it does what it should, that was the fix; if it behaves the same, the step was not the answer and something else is doing it."), py_str("Do it and look again straight after. Unchanged means the step was not the fix, rather than that it needs longer.")]));
    s_A_WAIT_IN_THE_STEP(stdlib_call("re", "compile", &[py_str("\\b(?:after|within|for|over|in|another)\\s+(?:a|an|the|two|three|four|five|six|few|couple|several|\\d+)\\s+(?:day|days|week|weeks|night|nights|month|months)\\b|\\bovernight\\b|\\bnext day\\b|\\bthe following (?:day|week)\\b|\\bleave it (?:to|for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_ABOUT_A_STEP(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|will|would|should|can|could|is|was)\\b(?:\\s+\\w+){0,3}\\s+(?:that|this|it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_FOR_THE_STEP(stdlib_call("re", "compile", &[py_str("^\\s*(?:how|what|where|when|which|who|why)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FIX_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:fix|fixes|fixed|sort|sorts|sorted|solve|solves|cure|cures|work|works|help|enough|stop|stops|clear|clears|do the trick|make a difference)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_STEP_TO_WATCH(call_builtin("frozenset", &[py_tuple(vec![py_str("health"), py_str("pregnancy"), py_str("babyfood"), py_str("body"), py_str("caring"), py_str("complaints"), py_str("crisis_replies"), py_str("pharmacy"), py_str("pets"), py_str("animals"), py_str("family"), py_str("friends"), py_str("sleep")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_STEP_TO_WATCH_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:baby|babies|infant|newborn|toddler|pregnan\\w*|napp(?:y|ies)|rash|fever|vomit\\w*|diarr\\w*|breathless|symptom\\w*|seizure|dehydrat\\w*|allerg\\w*|asthma|inhaler|epipen|insulin|diabet\\w*|medicin\\w*|medication|antibiotic\\w*|paracetamol|ibuprofen|painkiller\\w*|gp|gps|doctor|doctors|nurse|midwife|pharmac\\w*|hospital|a&e|ambulance|111|999|911|dentist|vet)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_LINES(py_tuple(vec![py_str("Not the thing itself, no. It would stop being ordinary if {when} — that is the point to act on."), py_str("Ordinary on its own, yes. It stops being ordinary if {when}, so that is what to watch."), py_str("Nothing in that is a warning by itself. It becomes one if {when}: that is when it wants someone.")]));
    s_GRIEF_FLOOR_LINES(py_tuple(vec![py_str("Not something to be graded bad or not - it is a loss, and it deserves the room it takes. Be gentle with yourself, and let whoever is around you know how you are."), py_str("That is not a thing to call bad, and not a thing to carry on your own. Take the help that is offered, and tell someone close how today actually went."), py_str("Nothing about that is a fault in you. Give it the time it takes, and lean on the person who asks how you are.")]));
    s_MET_CONDITION_LINES(py_tuple(vec![py_str("Then you are already at the point it named, so that is the bit to act on rather than the bit to watch. Get it looked at, and say what you told me here when you do."), py_str("That is the thing that answer said to act on, and it is what you are describing. Get it seen to rather than sitting with it."), py_str("So it is past the watching stage: the answer's own next step is the one for now. Get that started today.")]));
    s_WHEN_ASK(stdlib_call("re", "compile", &[py_str("^(?:and |so )?when (?:should|do|would) i (?:be )?(?:worried|worry|worrying)(?: about (?:it|that|this))?(?: then| though)?$|^(?:and |so )?when (?:should|do) i (?:start|need) (?:to )?(?:be )?worry(?:ing)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORRYING_ASK(stdlib_call("re", "compile", &[py_str("^(?:and |so )?is (?:that|it|this) (?:worrying|worrisome|a worry)(?: then| though)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_LINES(py_tuple(vec![py_str("This is the line: {when}. Short of that it is the ordinary kind, and worth watching rather than acting on."), py_str("What makes it somebody else's job: {when}. Until then it is a watch-and-see."), py_str("The point to act on is this: {when}. Before that it is the ordinary thing, so keep an eye on it rather than acting on it.")]));
    s_WHEN_CALL_LINES(py_tuple(vec![py_str("The line to act on: {when}. Short of that it is the watching stage, and the answer's own steps are the plan."), py_str("This is where it stops being a wait-and-see: {when}. Before that, do what the answer said and keep an eye on it."), py_str("What sends you to someone: {when}. Under that line the answer's steps are the whole of it.")]));
    s_NEEDS_A_CALL(stdlib_call("re", "compile", &[py_str("(?P<list>[^.;:]*?)\\s*(?:,\\s*and\\s+|\\s+and\\s+)?(?:needs?|wants?|want|means)\\s+(?:the\\s+|your\\s+|a\\s+)?(?:vet|gp|doctor|dentist|optician|pharmacist|physio|111|a&e|engineer|plumber|electrician)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LIST_JOIN(stdlib_call("re", "compile", &[py_str("\\b(?:are|is|was|were)\\s+not\\s*,?\\s*(?:and)?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CALLS_FOR(stdlib_call("re", "compile", &[py_str("\\b(?:see|ring|call|contact|speak to|go to|get)\\s+(?:a |your |the )?(?P<who>gp|doctor|practice nurse|nurse|111|999|a&e|vet|dentist|optician|pharmacist|surgery|midwife|physio(?:therapist)?|urgent care|out of hours)(?P<quick>\\s+(?:the same day|today|straight away|now|first thing|as soon as you can|within \\w+ hours))?\\s+for\\s+(?P<list>[^.;:]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IS_A_NUMBER(stdlib_call("re", "compile", &[py_str("(?P<when>[^.;:]+?)\\s+(?:is|are)\\s+(?P<who>111|999|a&e)\\b(?P<quick>\\s+(?:today|now|the same day|that day|straight away))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_SIGN_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("about"), py_str("after"), py_str("again"), py_str("been"), py_str("before"), py_str("could"), py_str("doesnt"), py_str("from"), py_str("have"), py_str("into"), py_str("just"), py_str("like"), py_str("more"), py_str("much"), py_str("only"), py_str("over"), py_str("same"), py_str("should"), py_str("some"), py_str("than"), py_str("that"), py_str("their"), py_str("them"), py_str("then"), py_str("there"), py_str("these"), py_str("they"), py_str("thing"), py_str("this"), py_str("very"), py_str("want"), py_str("what"), py_str("when"), py_str("will"), py_str("with"), py_str("worried"), py_str("worry")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:i'm|im|i am|we're|were|we are)\\s+(?:going|off|flying|driving|sailing|heading|travelling|traveling|moving|booking|booked|selling|buying|renting)\\b|\\b(?:i|we)\\s+(?:want|wanna|hope|plan|intend|need|'d like|would like)\\s+to\\b|\\b(?:i'm|im|i am|we're|were|we are)\\s+(?:about to|planning to|thinking (?:of|about)|going to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOW_TO_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how|where|when)\\s+(?:do|should|can|could|would|shall)\\s+(?:i|we)\\b|\\bhow\\s+(?:do|can|would)\\s+(?:i|we)\\s+(?:stop|get|fix|clean|keep|make|put|sort|deal|go|use|set|apply)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DECISION_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:should|shall|do|would)\\s+(?:i|we)\\b|\\b(?:should|shall)\\s+(?:i|we)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_YES(stdlib_call("re", "compile", &[py_str("\\A\\W*(?:yes|yep|yeah|yup|it does|that does|they can|it can|you can|both do)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_METHOD_DOUBT(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |ok |okay |well )*(?:will|would|does|do|did|can|could) (?:that|this|it) (?:actually |really |even |still )?(?:work|help|be enough|do the job|do it)(?: then| though)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_METHOD_DOUBT(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |ok |okay |well )*(?:will|would|does|do|did|can|could) (?:that|this|it) (?:actually |really |even |still )?(?:work|help|be enough|do the job|do it)(?: then| though)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHECK_ASK(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |ok(?:ay)?[, ]+)?(?:do you think\\s+)?(?:will|would|does|do|did|should|shall|could|can|is|was)\\s+(?:that|this|it|they|those)\\s+(?:actually |really |even )?(?:be\\s+)?(?:work|works|help|helps|fix it|fix that|do it|do the trick|do any good|make a difference|sort it|be enough|enough|ok|okay|fine|right|worth it|worth doing|worth a go|worth the money|the right thing|the right thing to do|the right way|the done thing|the usual way)(?: then| though| at all| for me| for us| for them| really)?\\s*$|^(?:and |so )?(?:shall|should|could|can|do)\\s+i\\s+(?:try|do|use|go with|give)\\s+(?:(?:that|this|it)\\s+)?(?:a\\s+)?(?:go|try|shot)|^(?:and |so )?(?:shall|should|could|can|do)\\s+i\\s+(?:try|do|use|go with)\\s+(?:that|this|it)(?:\\s+then)?\\s*$|^(?:and |so )?(?:do you think\\s+)?(?:that|this|it)\\s+(?:will|would|does|is|'s)?\\s*(?:be enough|enough|work|help|be worth it|be ok|be fine)\\s*$|^(?:and |so )?(?:is|was)\\s+(?:that|this|it)\\s+the\\s+(?:right|best|proper|usual|sensible|standard)\\s+(?:thing|way|way to do it|order|approach|method|call)(?:\\s+to\\s+do)?\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THINKING_ABOUT(stdlib_call("re", "compile", &[py_str("\\bthinking (?:about|of)\\b|\\bwondering\\b|\\bconsidering\\b|\\bnot sure (?:if|whether)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_METHOD_OK_LINES(py_tuple(vec![py_str("Nothing there is a problem - that is how I would do it too."), py_str("That is how I would go about it, and there is nothing in it to worry about."), py_str("Done that way, it needs no one else: that is the method I would use.")]));
    s_CHECK_WORKS_LINES(py_tuple(vec![py_str("It is the right thing to try first, and it works for most people. Give it a week or two and tell me how it goes."), py_str("Yes - that is the standard way, so it is the one I would count on to start with. What changes after you have tried it is what tells us the next step."), py_str("That is the usual way to go at it and it tends to do the job. If nothing has moved in a week, say so and we will go a step further."), py_str("That is what I would do myself, so yes - it is worth a run. A fortnight of it is usually enough to know.")]));
    s_CHECK_ONCE_LINES(py_tuple(vec![py_str("That is the standard way of doing it and it works - tell me how it goes and we will take it from there."), py_str("Yes, that is how it is done. Do it that way and tell me how you get on."), py_str("That way of doing it is the right one, so go ahead - tell me if it does not do the trick.")]));
    s_A_LENGTH_IN_THE_ANSWER(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:a|one|two|three|four)(?: or (?:more|so))? (?:day|days|week|weeks|night|nights|hour|hours|fortnight|fortnights)(?![a-z])|(?<![a-z])a few (?:days|weeks|nights)(?![a-z])|(?<![a-z])fortnight(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHECK_WATCH_LINES(py_tuple(vec![py_str("That is the thing to try first, and most people do well on it. If it is no better in a week or two, that is when it wants someone to look rather than more of the same."), py_str("Yes, that is worth doing first. If it stops helping, or it goes the wrong way, that is the point to get it looked at rather than wait.")]));
    let __try11 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret12: Py = Py::Bool(false);
    let mut __val13: Py = Py::None;
    let mut __unhandled14: Option<exc::PyExc> = None;
    match __try11 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret12 = __p[0].clone(); __val13 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled15 = false;
            if !__handled15 && exc::matches_any(&__e, &["Exception"]) {
                __handled15 = true;
    s__BODY_PARTS(call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)));
    s__SYMPTOM_WORDS(call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)));
            }
            if !__handled15 { __unhandled14 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled14 { exc::raise(__e); }
    s__A_BODY_WORD(bitor(&bitor(&call_builtin("frozenset", &[g__BODY_PARTS()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[g__SYMPTOM_WORDS()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[py_tuple(vec![py_str("sick"), py_str("unwell"), py_str("ill"), py_str("nauseous"), py_str("sore"), py_str("soreness"), py_str("swollen"), py_str("swelling")])]).unwrap_or_else(|e| exc::raise(e))));
    s_CHECK_JOB_LINES(py_tuple(vec![py_str("That is how I would go about it, and it is the right thing to do first. Tell me how you get on and I will pick it up from there."), py_str("Yes, do it that way - it is the sensible order to do it in. Say how it goes."), py_str("That is the right first move, and what comes back from it decides the next step.")]));
    s_ASKS_FINE(stdlib_call("re", "compile", &[py_str("\\b(?:ok|okay|alright|all right|fine|sensible|a good idea|worth it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_WRONG(stdlib_call("re", "compile", &[py_str("\\b(?:bad|wrong|worrying|worry|a problem|dangerous|stupid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHOICE_OK_LINES(py_tuple(vec![py_str("Yes - that is a good call, and there is nothing in it I would change."), py_str("Yes, go with it. It is a sensible thing to do and nothing in it is a problem."), py_str("That one is fine as it stands. Nothing in it needs second-guessing."), py_str("Nothing wrong with that choice. If it is the one that suits you, take it."), py_str("Yes, I would. That is the pick I would make from where you are standing.")]));
    s_NOTHING_WARNED_LINES(py_tuple(vec![py_str("Nothing there is a warning sign - it is the ordinary thing to do. What would change it is the thing itself getting worse, or not settling."), py_str("There is no warning in it: that is the everyday fix. What would matter is it getting worse or not shifting, so watch for that."), py_str("Nobody needs to be called over it yet - it is the standard way to handle it. The point to get someone is if it does not settle."), py_str("It is the usual handling of a thing like that rather than a warning about it. What would matter is it getting worse or not shifting."), py_str("It is a job to do rather than a sign of how bad it is. Do that, and the thing to watch is it getting worse or not settling.")]));
    s_MONEY_TROUBLE_LINES(py_tuple(vec![py_str("Nothing here is broken - it is the money that does not stretch, and that is a thing letters and calls fix faster than waiting does. What is the first date you have to meet?"), py_str("It matters, and it is the sort of thing that shrinks as soon as somebody official is told about it. Ring the one the answer named before the next payment date, not after."), py_str("That is a real hole in the month rather than a fault in you. Start with the one the answer named, and tell me what they say."), py_str("It is a squeeze, not a sign that you have done something wrong, and it is the kind that gets smaller once it is written down. What is the number you cannot meet?")]));
    s_MONEY_NOUN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("rent"), py_str("money"), py_str("bill"), py_str("bills"), py_str("debt"), py_str("debts"), py_str("owe"), py_str("owed"), py_str("arrears"), py_str("overdue"), py_str("loan"), py_str("loans"), py_str("credit"), py_str("card"), py_str("cards"), py_str("mortgage"), py_str("council"), py_str("tax"), py_str("fine"), py_str("fines"), py_str("energy"), py_str("electric"), py_str("electricity"), py_str("gas"), py_str("heating"), py_str("water"), py_str("payment"), py_str("payments"), py_str("payday"), py_str("wages"), py_str("pay"), py_str("savings"), py_str("overdraft"), py_str("afford")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_NOT_MET(stdlib_call("re", "compile", &[py_str("\\b(?:cant|can'?t|couldnt|couldn'?t|no|not|nothing|never|behind|missed|miss|late|overdue|short|stuck|struggling|struggle|afford|owe|owed|debt|debts|arrears|skint|going down|cut|reduced|behind on|fine|fines|fined|penalty|penalties|repossession|evicted|eviction|bailiffs|declined|decline|refused|bounced|overdrawn|overdraft)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MONEY_PAY(stdlib_call("re", "compile", &[py_str("\\bpay(?:s|ing|ed)?\\s+(?:attention|heed|respects?|tribute|homage|lip\\s+service|a\\s+visit|a\\s+call|compliments)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BODY_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:pain|ache|hurts?|hurting|sore|swollen|rash|bleeding|lump|cough|fever|temperature|vomiting|sick|dizzy|breathless|doctor|gp|hospital|ambulance)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NUMBER(stdlib_call("re", "compile", &[py_str("\\d")]).unwrap_or_else(|e| exc::raise(e)));
    s_MET_LINES(py_tuple(vec![py_str("That answer put a timescale on watching it, and you are already past it. That is the point it meant, so get it looked at rather than sitting on it."), py_str("The answer's own line on this is the timescale, and yours has been going on longer. That is the bit to act on, and it sounds like it is now."), py_str("It said to watch it, and gave the timescale for doing that. You are past the timescale, so it is past watching - get it seen to.")]));
    s_FIGURE_LINES(py_tuple(vec![py_str("Same money every month adds up to a yearly figure, and that is the one to weigh: if a year of it still feels worth what you would otherwise do with the money, it is not too much."), py_str("It is only ever a lot next to something. Put the year's total beside what else that money would do for you, and the answer usually picks itself."), py_str("A monthly figure hides its size. Work out the year, then ask what you get for the year - to one budget it is loose change and to another it is the whole of the saving.")]));
    s_A_FIGURE_ASK(stdlib_call("re", "compile", &[py_str("^(?:and |so )?is (?:that|this|it) (?:a lot|a lot of money|too much|too much money|too expensive|expensive|cheap|good value|about right|a fair price|that much|really that much)(?: then| though| or not)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_LINES(py_tuple(vec![py_str("That is a figure to compare rather than a fault: the answer above gives the usual one, so put yours beside it, and if yours is the big one, that is what to change."), py_str("Not bad so much as a number to check. The answer above quotes what is usual, and the gap between that and yours is the bit worth acting on."), py_str("Nothing is broken there - it is your figure against the usual one the answer gave. If yours is the higher of the two, that is the thing to do something about.")]));
    s_MONEY_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("bill"), py_str("bills"), py_str("invoice"), py_str("invoices"), py_str("rent"), py_str("owe"), py_str("owed"), py_str("owes"), py_str("owing"), py_str("debt"), py_str("debts"), py_str("tariff"), py_str("tariffs"), py_str("refund"), py_str("refunds"), py_str("fare"), py_str("fares"), py_str("price"), py_str("prices"), py_str("charge"), py_str("charges"), py_str("charged"), py_str("payment"), py_str("payments"), py_str("paying"), py_str("paid"), py_str("pay"), py_str("cost"), py_str("costs"), py_str("mortgage"), py_str("loan"), py_str("loans"), py_str("overdraft"), py_str("insurance"), py_str("premium"), py_str("premiums"), py_str("subscription"), py_str("subscriptions"), py_str("council"), py_str("tax"), py_str("vat"), py_str("fine"), py_str("fined"), py_str("money"), py_str("pennies"), py_str("quid"), py_str("pounds"), py_str("dollars"), py_str("euros"), py_str("pension"), py_str("salary"), py_str("wage"), py_str("wages"), py_str("budget"), py_str("savings"), py_str("saving")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("[\\u00a3$\\u20ac]\\s?\\d|\\b\\d+(?:\\.\\d+)?\\s*(?:p|quid|pounds?|dollars?|euros?|grand|k)\\b|\\b\\d[\\d,.]*\\s*(?:a|per|each)\\s+(?:month|week|year|fortnight|day)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COMPARISON(py_tuple(vec![py_str("usual"), py_str("usually"), py_str("typical"), py_str("typically"), py_str("compare"), py_str("compares"), py_str("average"), py_str("on average"), py_str("benchmark")]));
    s_JUST_AGE(stdlib_call("re", "compile", &[py_str("\\b(?:just|only)\\b[^.!?]{0,24}\\b(?:age|a phase|a stage|tiredness|the tiredness)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_LINES(py_tuple(vec![py_str("Often it is age, and the thing that is not ordinary is a change: slower than before is usual, new or getting worse is the part to get looked at."), py_str("Some of that is ordinary at that age. What is worth a word with the GP is a change - something that came on recently, or that keeps getting worse."), py_str("Mostly age, as a rule, and it is the change that settles it rather than the thing itself: suddenly different, or worse each month, is the point to ask someone about it.")]));
    s_BODY_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("pain"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("sick"), py_str("ill"), py_str("nauseous"), py_str("nausea"), py_str("dizzy"), py_str("headache"), py_str("temperature"), py_str("fever"), py_str("breathless"), py_str("throat"), py_str("chest"), py_str("stomach"), py_str("arm"), py_str("leg"), py_str("legs"), py_str("back"), py_str("knee"), py_str("hand"), py_str("foot"), py_str("feet"), py_str("head"), py_str("tooth"), py_str("toothache"), py_str("cough"), py_str("cold"), py_str("flu"), py_str("rash"), py_str("lump"), py_str("bleeding"), py_str("bleed"), py_str("blood"), py_str("itch"), py_str("itching"), py_str("swollen"), py_str("cramp"), py_str("period"), py_str("surgery"), py_str("pill"), py_str("pills"), py_str("tablet"), py_str("tablets"), py_str("dose")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSE(call_builtin("frozenset", &[py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("mum"), py_str("mom"), py_str("dad"), py_str("mother"), py_str("father"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("sister"), py_str("brother"), py_str("friend"), py_str("gran"), py_str("grandad"), py_str("grandma"), py_str("grandpa"), py_str("nan"), py_str("colleague"), py_str("boss"), py_str("dog"), py_str("cat")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_FAMILY(py_dict(vec![(py_str("tired"), py_str("tired")), (py_str("exhausted"), py_str("tired")), (py_str("drained"), py_str("tired")), (py_str("wiped"), py_str("tired")), (py_str("knackered"), py_str("tired")), (py_str("shattered"), py_str("tired")), (py_str("worn"), py_str("tired")), (py_str("burnt"), py_str("tired")), (py_str("burnout"), py_str("tired")), (py_str("anxious"), py_str("anxious")), (py_str("anxiety"), py_str("anxious")), (py_str("worried"), py_str("anxious")), (py_str("stressed"), py_str("anxious")), (py_str("stress"), py_str("anxious")), (py_str("crying"), py_str("crying")), (py_str("cry"), py_str("crying")), (py_str("tearful"), py_str("crying")), (py_str("low"), py_str("flat")), (py_str("down"), py_str("flat")), (py_str("flat"), py_str("flat")), (py_str("miserable"), py_str("flat")), (py_str("unhappy"), py_str("flat")), (py_str("sad"), py_str("flat")), (py_str("numb"), py_str("flat")), (py_str("lonely"), py_str("flat")), (py_str("hopeless"), py_str("flat")), (py_str("overwhelmed"), py_str("heavy")), (py_str("struggling"), py_str("heavy")), (py_str("fed"), py_str("heavy")), (py_str("useless"), py_str("heavy")), (py_str("worthless"), py_str("heavy")), (py_str("pointless"), py_str("heavy"))]));
    s_FEELING_DOUBT_LINES(py_dict(vec![(py_str("tired"), py_tuple(vec![py_str("That is what running on empty feels like, and it is not a fault in you. Sleep and food come first - tiredness from a hard stretch lifts once those are back."), py_str("Nothing about it is a sign something is broken. Tired like that is what carrying a lot does to a person, and it comes back up slowly rather than all at once."), py_str("Being this worn out is ordinary for the run you have had, and it is allowed to be this heavy. If a couple of weeks pass with no better days in them, that is worth saying to a GP.")])), (py_str("anxious"), py_tuple(vec![py_str("That is worry doing its work, and it is not a sign something is wrong with you. It eases once the thing on your mind eases; until then, sleep and eating are the two worth guarding."), py_str("Nothing about it is unusual, and nothing to be ashamed of. The part worth watching is whether it is there all day every day - weeks of that is worth saying to a GP.")])), (py_str("crying"), py_tuple(vec![py_str("That is tiredness and grief coming out of you, and it is not a weakness. It comes in waves, and the waves get further apart - they do not keep to a timetable."), py_str("Nothing about crying like that is a sign something is broken. It usually eases within a few weeks of the thing that set it off; a month of it is worth saying to a GP.")])), (py_str("flat"), py_tuple(vec![py_str("That is what a low stretch does to a person, and it is not a fault in you. It lifts, slowly and not in a straight line - watch the good hours rather than the bad ones."), py_str("Nothing there is a sign something is broken. Feeling flat is what a heavy few months looks like from the inside, and the way back is usually small and boring: sleep, food, one person who knows."), py_str("That is ordinary for the run you have had, and it is allowed to be this heavy. If a couple of weeks go by with no better hours in them, that is the point to say it to a GP.")])), (py_str("heavy"), py_tuple(vec![py_str("That is a lot to carry, and it is not a measure of you. If a couple of weeks pass with no better hours in them, say it to a GP - that is what they are there for."), py_str("Nothing there is a weakness. What you are describing is what too much for too long feels like, and the first thing that helps is usually a shorter list rather than more effort.")]))]));
    s_ASKS_NORMAL(stdlib_call("re", "compile", &[py_str("\\b(?:normal|abnormal|unusual|usual|typical|common|ordinary|standard)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_OK(stdlib_call("re", "compile", &[py_str("\\b(?:ok|okay|alright|all\\s+right|acceptable|fine|safe)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OK_LINES(py_tuple(vec![py_str("Nothing in that sounded like a warning sign — so yes, it is fine. What would change it is the thing itself changing: getting worse, or not settling."), py_str("Yes — that one is the ordinary kind. Watch it rather than worry about it: if it is still the same in a week or two, or it gets worse, get someone to look at it."), py_str("Yes — the part you asked about is the harmless kind. What to keep an eye on is change rather than the thing itself.")]));
    s_ORDINARY_NORMAL(py_tuple(vec![py_str("Yes, that is the usual thing - what to keep an eye on is change rather than the thing itself."), py_str("Yes, that is normal for most people. Watch it rather than worry about it: if it is the same in a week or two, or it gets worse, get someone to look at it."), py_str("Yes, that is the ordinary thing, and it stays ordinary unless it changes. Getting worse, or not settling, is what would make it worth someone's look.")]));
    s_CARE_NORMAL(py_tuple(vec![py_str("Yes - common or not, that one is worth getting looked at properly. Say what you told me here when you do."), py_str("Yes. That one needs a look rather than watching, and whether it is the usual thing is exactly what the look settles. Say what you told me here when you go.")]));
    s_CARE_NORMAL_UNKNOWABLE(py_tuple(vec![py_str("I could not call that usual or not from here, and it is one to have looked at: the check is cheap and it settles the question rather than the guessing.")]));
    s_CARE_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|the same day|same day|straight away|right away|now|urgent|within (?:a|one|two|three|\\d+) (?:day|days|hour|hours))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_NOW_NORMAL(py_tuple(vec![py_str("Don't sit on that one - it is a today thing, and whether it is the usual thing is what the look settles rather than the guessing."), py_str("That wants seeing today. Whether it is the ordinary version or not is exactly what the visit is for, so ring and be seen.")]));
    s_DO_NORMAL(py_tuple(vec![py_str("That is the ordinary thing to do about it, and it is a job rather than a danger: do what I said and see whether it holds."), py_str("Nothing in that is out of the ordinary - it is the everyday job, so do it and see whether it holds; if it comes back after that, get someone in."), py_str("That is the usual way of dealing with it, and it is worth sorting rather than worrying about - do it and see whether it holds.")]));
    s_CONDITION_NORMAL(py_tuple(vec![py_str("Ordinary on its own, yes - and it stops being usual if {when}. That is the point to act on."), py_str("That is the usual case on its own, and it becomes something else if {when}."), py_str("Usual up to a point: what would make it something to act on is if {when}.")]));
    s_NOTHING_WARNED_NORMAL(py_tuple(vec![py_str("Yes, that is how it is normally handled. What would matter is a change in it."), py_str("Yes, that is the ordinary job, and what would send you to someone is a change in it."), py_str("Yes, ordinary - that is the everyday fix for it, and the thing to watch is it getting worse or not shifting.")]));
    s_YES_LINES(py_tuple(vec![py_str("Yes — it is worth acting on rather than sitting on. Asking costs very little and it stops the guessing."), py_str("Yes, that one wants a proper look rather than watching it. Get it looked at properly, and say what you told me here when you do."), py_str("Yes. Get it seen to — not an emergency, but not something to sit on either.")]));
    s_A_THING_THEY_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:boilers?|radiators?|central heating|thermostat|washing machine|washer dryer|tumble dryer|dishwashers?|fridge|freezer|fridge freezer|ovens?|hob|cooker|microwave|kettles?|toaster|hoover|vacuum|lawnmower|mower|strimmer|pressure washer|shower tray|showers?|taps?|toilets?|sinks?|drains?|plughole|gutters?|roof|fencing|fence|decking|doors?|windows?|locks?|sockets?|fuse board|consumer unit|lights?|cars?|vans?|bikes?|bicycles?|motorbike|scooter|tyres?|brakes?|clutch|exhaust|engine|laptops?|computers?|pcs?|printers?|phones?|tablets?|monitors?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BODY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:skin|rash|itch|itching|eyes?|ears?|throat|chest|stomach|belly|knee|knees|joint|joints|hips?|shoulders?|neck|foot|feet|hands?|arms?|legs?|tooth|teeth|gums?|breath|back(?! (?:door|doors|garden|gate|wall|bed|room|bit|passage))|breathing|breathless|cough|poo|wee|blood|fever|temperature|hearing|sight|head|headaches?|migraine)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_ORDINARY(py_tuple(vec![py_str("It is the everyday thing rather than a fault, and the job I described is what settles it. If it is still doing it after that, or it comes back, that is when it wants somebody who does this for a living."), py_str("Nothing in that is a fault on its own: it is the ordinary thing with a job attached. Do the job, and if the noise or the mark is still there afterwards, get somebody in."), py_str("That is the usual reason for it, and the fix I gave is the check. Do that first - if it holds, it was the everyday thing, and if it does not, that is a call to somebody with the trade.")]));
    s_THING_CALL(py_tuple(vec![py_str("Yes - that one is for somebody who does this for a living rather than a watch-and-see. Describe it as you did here when you book them in, and hold off using it until they have looked."), py_str("Yes, get that looked at properly. It is a call to a trade rather than something to sit on, and the sooner they see it the less it usually costs."), py_str("Yes. That is a job for somebody with the right tools, not a wait-and-see.")]));
    s_BACKSTOP_LINES(py_tuple(vec![py_str("Not bad, no. What would make it matter is how long it has been going on and what it is costing you — how do those two look?"), py_str("I wouldn't call it bad. It is worth a proper look rather than a shrug, though: what does it take out of your week?"), py_str("Not a fault, and nothing to be ashamed of. The question worth asking is whether it has a hold on your days rather than whether it is bad.")]));
    s_PEOPLE_CARE(py_tuple(vec![py_str("gp"), py_str("doctor"), py_str("nurse"), py_str("midwife"), py_str("health visitor"), py_str("physio"), py_str("therapist"), py_str("pharmacist"), py_str("optician"), py_str("dentist"), py_str("vet"), py_str("111"), py_str("referral"), py_str("hospital"), py_str("a&e"), py_str("urgent care"), py_str("out of hours"), py_str("specialist"), py_str("clinic")]));
    s_FLOOR_LINES(py_tuple(vec![py_str("How long has it been going on, and what does it cost you day to day?"), py_str("How long has it been like that, and is it getting in the way of your week?"), py_str("What would you like to do about it, and how long has it been like that?")]));
    s_NEUTRAL_ASK_LINES(py_tuple(vec![py_str("Say a bit more: how long has it been like that, and what does it cost you day to day?"), py_str("How long has it been going on, and what would you like to do about it?"), py_str("Tell me more and I will help you think it through: how long, and how is it affecting you?")]));
    s_A_PLAN_DOUBT_LINES(py_tuple(vec![py_str("Nothing in that sounds wrong: it is a plan rather than a fault to check. Which part of it are you unsure about?"), py_str("Say which part you are unsure about and we can weigh it up together."), py_str("That is yours to decide rather than something to worry about. What is the bit you keep going over?")]));
    s_SOON_LINES(py_tuple(vec![py_str("Yes, that wants looking at - the answer named who, so book it rather than wait for it to settle."), py_str("It is worth having it seen to, and that answer already said by whom. Get that booked, and say how long it has been going on when you do."), py_str("Not an emergency by the sound of it, but not something to leave either: the person named in that answer is the one to see.")]));
    s_HELPED_LINES(py_tuple(vec![py_str("Then it is doing its job - keep it up and give it a few more days before you judge it."), py_str("Good: that is the thing that worked, so stay with it. The point to get someone to look is if it stops helping."), py_str("That is the answer then. Keep doing it, and tell me if it stops working.")]));
    s_WORRIED_ASK(stdlib_call("re", "compile", &[py_str("^(?:and |so )?should i (?:be )?(?:worried|worry|worrying|concerned|see someone|see a doctor|do anything|do something|get it checked|be checked|call someone|ring someone)(?: then| though)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HELPED(stdlib_call("re", "compile", &[py_str("\\b(?:(?:that|it|this) (?:has )?(?:helped|worked|did the trick|did help|is helping|is working)|both worked|they worked|it worked out|(?:much )?better (?:now|today)|(?:its|it is) (?:helping|working)|on the mend)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_YES_SAID_LINES(py_tuple(vec![py_str("Yes - that is why the call is the thing to do now rather than something to wait on. Do that first, and tell me what they say."), py_str("Yes it is, and that is what the call is for. Make it now rather than tomorrow, and we can go through the rest once you have."), py_str("Yes - ringing is not an overreaction here, it is the whole point of it. Do that first and come back to me after."), py_str("Yes. Somebody needs to hear this today rather than next week, so make the call and then tell me what they say.")]));
    s_THROW_AWAY(stdlib_call("re", "compile", &[py_str("\\b(?:bin\\s+(?:it|them|that)|throw\\s+(?:it|them|that)\\s+(?:away|out)|chuck\\s+(?:it|them|that)(?!\\s+(?:\\w+\\s+){0,2}(?:in|on|into|with|to)\\b)|(?:get|getting)\\s+rid\\s+of\\s+(?:it|them|that))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_TOLD_TO(stdlib_call("re", "compile", &[py_str("(?:\\bdon'?t|\\bdo not|\\bnever|\\bnot|\\brather than|\\binstead of|\\bbefore you)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ONE_OPTION(stdlib_call("re", "compile", &[py_str("\\b(?:or|and|rather than|instead of)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TESTING(stdlib_call("re", "compile", &[py_str("^[\\s\\\"'(\\u2018\\u201c]*(?:if|unless|once|whenever|when|any ?time|should|whether)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THROW_AWAY_LINES(py_tuple(vec![py_str("{Action} — it is not worth the risk, and it is not a close call."), py_str("Yes, really. {Action}. This is the one where guessing costs the most."), py_str("{Action}, and do not talk yourself out of it: nothing you can see or smell makes it safe.")]));
    s_TEST_LINES(py_tuple(vec![py_str("{Action} once it has turned - that is the whole test, and I would not taste it to check."), py_str("You do not need me to judge it. {Action} once it has turned, and that is the whole test."), py_str("{Action} if it has actually turned. The test is the answer, and the thing itself is the judge.")]));
    s_STATED_NO(stdlib_call("re", "compile", &[py_str("\\bthere (?:is|are) no safe (?:number|amount|level|dose|limit|quantity|way)\\b|\\bno amount of .{1,40} (?:is|was) safe\\b|\\bnothing about .{1,40} (?:is|makes it) safe\\b|\\bis not safe to (?:eat|drink|drive|take|use|give|leave|keep)\\b|\\bare not safe to (?:eat|drink|take|use|give)\\b|\\bnone of (?:it|them|these) is safe\\b|\\byou cannot make .{1,40} safe\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_STATED_LINES(py_tuple(vec![py_str("No — that is the answer's own line: {no} There is nothing in it for you to weigh up."), py_str("That one is a no, and the answer said it plainly: {no} What to do instead is in the rest of what I said."), py_str("No. {no} That is the whole of it, so it is not a thing for you to judge."), py_str("No, and not a close call. {no}")]));
    s_ENOUGH_ASK(stdlib_call("re", "compile", &[py_str("^(?:and |so )?(?:is|was|are|were) (?:that|this|it|all of that|all that)(?: (?:really|actually|about|roughly))? enough(?: then| though)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STATED_ENOUGH(stdlib_call("re", "compile", &[py_str("\\bis (?:definitely |probably |easily |more than |plenty )?enough\\b|\\bis plenty\\b|\\bmore than enough\\b|\\bthat will do\\b|\\bthat does it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENOUGH_STATED_LINES(py_tuple(vec![py_str("Yes — {yes} That is the amount, and the rest of what I said is what makes it work."), py_str("Yes: {yes} Nothing in the answer asks for more than that."), py_str("It covers it as it stands. {yes}"), py_str("Yes, on the answer's own figures. {yes}")]));
    s_STOP_DOING(py_tuple(vec![py_str("stop driving"), py_str("don't drive"), py_str("dont drive"), py_str("don't eat"), py_str("dont eat"), py_str("don't use it"), py_str("dont use it"), py_str("don't touch"), py_str("dont touch"), py_str("don't drink"), py_str("dont drink"), py_str("leave the building"), py_str("get everyone away"), py_str("stop if it is safe")]));
    s_DANGER(py_tuple(vec![py_str("dangerous"), py_str("toxic"), py_str("poisonous"), py_str("poisoning"), py_str("can die"), py_str("could die"), py_str("may die"), py_str("can kill"), py_str("could kill"), py_str("fatal"), py_str("harmful"), py_str("not safe"), py_str("unsafe"), py_str("an emergency"), py_str("never give"), py_str("never use"), py_str("never take"), py_str("never leave")]));
    s_NEGATED(stdlib_call("re", "compile", &[py_str("(?:\\bnot|\\bno|n't|\\bnever|\\bnon)[ -]\\w*[ -]?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_AGAIN(py_tuple(vec![py_str("Same answer as a moment ago: nothing in what I said was a warning sign. It's the change to watch for, not the thing itself."), py_str("Still no. If it would help, tell me what's actually worrying you about it and I'll be straight with you.")]));
    s__FILLER(py_str("(?:(?:and|so|ok|okay|but|well|right|honestly|actually|basically|literally|simply|truly|really|just) )"));
    s__LEAD_FILLER(add(&g__FILLER(), &py_str("*")));
    s__FILL(add(&g__FILLER(), &py_str("?")));
    s_STEP(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&py_str("^(?:"), &g__LEAD_FILLER()), &py_str(")(?:(?:what|which) ")), &g__FILL()), &py_str("(?:should|do|can) ")), &g__FILL()), &py_str("i ")), &g__FILL()), &py_str("do(?: now| next| first| about it)?(?: then)?$|what ")), &g__FILL()), &py_str("now$|what ")), &g__FILL()), &py_str("(?:do|should) ")), &g__FILL()), &py_str("i ")), &g__FILL()), &py_str("(?:try|do) ")), &g__FILL()), &py_str("(?:first|next)$|how ")), &g__FILL()), &py_str("do ")), &g__FILL()), &py_str("i ")), &g__FILL()), &py_str("(?:fix|sort|deal with) ")), &g__FILL()), &py_str("(?:it|that|this)$|what ")), &g__FILL()), &py_str("(?:should|do|can|could) ")), &g__FILL()), &py_str("i ")), &g__FILL()), &py_str("(?:put|use|apply|try) ")), &g__FILL()), &py_str("(?:on|to|for) ")), &g__FILL()), &py_str("(?:it|that|this)(?: then)?$)"))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAK_VERBS(py_str("(?:say|mention|bring (?:it|that|this|them) up|raise|tell|ask|text|call|ring|email|write|speak|talk)"));
    s_SPEAK_UP(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("^(?:and |so )?(?:(?:do you think|do you reckon|d'you reckon) )?(?:should|shall|ought) i "), &g_SPEAK_VERBS()), &py_str("\\b|^(?:and |so )?(?:do you think|do you reckon|d'you reckon) i should ")), &g_SPEAK_VERBS()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_EMERGENCY_CALL(stdlib_call("re", "compile", &[py_str("\\b(?:ring|call|dial|phone|contact)\\b[^.!?]{0,60}?\\b(?:999|911|112)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SEE(stdlib_call("re", "compile", &[py_str("^(?:and |so )?(?:should|do) i (?:need to )?(?:see|call|ring|go to) (?:a |the |my )?(?:someone|somebody|doctor|gp|vet|dentist|engineer|plumber|doctor about it)(?: then)?$|^(?:and |so )?do i need (?:to see )?(?:someone|a doctor|the doctor|the vet)(?: then)?$|^(?:and |so )?is it worth (?:seeing|going to|ringing) (?:someone|a doctor|the vet)(?: then)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_TO(stdlib_call("re", "compile", &[py_str("^(?:and |so |ok |okay |well )?how (?:do|should|can|would|might|will) i (?P<doing>[a-z]+)(?P<obj>(?: (?:[a-z]+ ){0,2}(?:it|that|this|them)(?: (?:on (?:friday|monday|tuesday|wednesday|thursday|saturday|sunday)|then|now|first|out|off|down|up|again|properly|safely))?)?)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_DOING_WITH_NO_OBJECT(call_builtin("frozenset", &[mcall(&py_str("prepare practise practice rehearse revise study train\nwarm stretch"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_KNOWING_DOING(call_builtin("frozenset", &[mcall(&py_str("know tell check spot find see say guess recognise recognize"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_AGAIN(stdlib_call("re", "compile", &[py_str("^(?:and |so |ok |okay |well )?(?:(?:any|some|got any|got some|need some|need any|want some|more|any more|another|a few|any other)\\s+)?(?:tips?|advice|pointers?|tricks?|hints?|ideas?|suggestions?|thoughts?)(?: (?:at all|then|please|too|now|really|anyway))*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ONE_THING_ADVERB(py_tuple(vec![py_str("even"), py_str("actually"), py_str("really"), py_str("just"), py_str("possibly"), py_str("maybe"), py_str("instead"), py_str("honestly"), py_str("normally")]));
    s__ONE_THING_ADVERB(modulo(&py_str("(?:(?:%s) )?"), &mcall(&py_str("|"), "join", &[g_ONE_THING_ADVERB()])));
    s_A_HOW_ASK(stdlib_call("re", "compile", &[py_str("^(?:and |so |ok |okay |well )?(?:what|whats|what is|which is|whichs) the (?:safest|best|quickest|fastest|easiest|simplest|surest|most reliable|right|proper|correct) (?:way|method|bet|option|approach|route)(?: (?:to (?:do|fix|sort|handle|treat|clean|stop|get rid of|deal with|go about) (?:it|that|this)|to (?:start|begin|try|go about)(?: (?:it|that|this))?|of doing (?:it|that|this)|of getting rid of (?:it|that|this)|for (?:it|that|this)|here|then|now|at all|please))?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ONE_THING(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("^(?:and |so |ok |okay |well |but )?(?:just (?:one|the one|the first|1) (?:thing|step|move|bit)(?: to (?:start|begin|try|do)(?: with)?(?: first)?)?|(?:one|the one|the first) (?:thing|step|move) (?:to (?:start|begin|try|do)(?: with)?(?: first)?|first)|(?:what|which) (?:is|would be|should be) the (?:one|first) thing(?: to do)?|(?:where|how) (?:do|should) i "), &g__ONE_THING_ADVERB()), &py_str("(?:start|begin)(?: with it| with that| here| off)?|(?:what|where) do i ")), &g__ONE_THING_ADVERB()), &py_str("(?:start|begin)(?: with it| with that)?)(?: (?:then|please|though|now|really))*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER(stdlib_call("re", "compile", &[py_str("[:;] | - | — | – ")]).unwrap_or_else(|e| exc::raise(e)));
    s_SPLIT(stdlib_call("re", "compile", &[py_str("([:;] | - | — | – )")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEN_OPEN(stdlib_call("re", "compile", &[py_str("^(?:and\\s+)?then[,]?\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFER(stdlib_call("re", "compile", &[py_str("\\b(?:see|ring|call|contact) (?:a |your |the )?(?:gp|doctor|vet|dentist|nurse|someone|surgery|physio|physiotherapist|pharmacist|optician|midwife|111|999|engineer|plumber)|\\b(?:have|get) (?:a |your |the )?(?:gp|doctor|vet|dentist|nurse|physio|physiotherapist|pharmacist|optician|midwife)\\s+(?:to\\s+)?(?:look|check|see|take a look)\\b|\\bask (?:a |your |the )?(?:gp|doctor|vet|dentist|nurse|physio|physiotherapist|pharmacist|optician|midwife)\\b|\\b(?:see|ring|call|contact) (?:a |your |the )?(?:(?!and\\b|or\\b|to\\b|for\\b|of\\b|in\\b|on\\b|at\\b|from\\b|with\\b|that\\b|if\\b|when\\b|it\\b)[a-z-]+ ){0,3}(?:gp|doctor|vet|dentist|nurse|someone|physio|physiotherapist|pharmacist|optician|midwife|111|999|engineer|plumber)|get (?:it|that|this|him|her|them|someone|somebody)? ?(?:looked at|checked|seen)|a&e|urgent care|out of hours|\\b(?:vet|doctor|gp|dentist) (?:visit|job|matter|appointment)\\b|\\btake (?:it|this|that|them) (?:to|along to|back to|in to) (?:a |your |the )?(?:gp|doctor|pharmacist|nurse|dentist|vet|physio|physiotherapist|optician|midwife)\\b|\\bnot a wait-and-see\\b|worth getting (?:it |them )?checked"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_A_CLINICIAN(stdlib_call("re", "compile", &[py_str("\\b(?:gps?|doctors?|nurse|practice nurse|midwife|health visitor|pharmacist|vets?|dentist|a&e|minor injuries|walk-in (?:centre|clinic)|urgent care|out of hours|111|999|nhs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DECLARATIVE_REFERRAL(stdlib_call("re", "compile", &[py_str("\\b(?:vet|doctor|gp|dentist) (?:visit|job|matter|appointment)\\b|\\bnot a wait-and-see\\b|\\b(?:checked|looked at|assessed|seen) by (?:a |your |the )?(?:gp|doctor|nurse|practice nurse|physio|physiotherapist|dentist|vet|pharmacist|optician|midwife)\\b|\\bneeds? (?:a |your |the )?(?:gp|doctor|nurse|vet|dentist|physio|pharmacist|optician)\\b|\\bneeds? (?:to be )?(?:seen|checked|looked at)\\b|\\bworth (?:a |an |your |the )?(?:gp|doctor|doctor's|vet|dentist|optician|pharmacist|physio\\w*|nurse|midwife|health visitor)\\b|\\b(?:gp|doctor's|doctor|vet|dentist|optician|physio\\w*|midwife|health visitor) appointment\\b|\\b(?:nhs 111|111 line|out of hours service|urgent treatment centre|health visitor)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN(call_builtin("frozenset", &[mcall(&py_str("about around roughly per couple once twice three four five six seven two\na an every each daily nightly\nmorning afternoon evening night day week month year hour minute second time times"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HANGING(call_builtin("frozenset", &[py_tuple(vec![py_str("with"), py_str("in"), py_str("of"), py_str("to"), py_str("on"), py_str("for"), py_str("at"), py_str("and"), py_str("or"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("your"), py_str("its"), py_str("it's"), py_str("from"), py_str("by"), py_str("as"), py_str("than"), py_str("but"), py_str("if"), py_str("so"), py_str("into"), py_str("about"), py_str("between"), py_str("among"), py_str("through"), py_str("under"), py_str("over"), py_str("without"), py_str("within"), py_str("against"), py_str("across"), py_str("behind"), py_str("beyond"), py_str("towards"), py_str("toward"), py_str("upon"), py_str("except"), py_str("despite"), py_str("unless"), py_str("until"), py_str("whether"), py_str("onto"), py_str("than")])]).unwrap_or_else(|e| exc::raise(e)));
    s_IMPERATIVES(call_builtin("frozenset", &[py_tuple(vec![py_str("keep"), py_str("ease"), py_str("ice"), py_str("rest"), py_str("bleed"), py_str("put"), py_str("take"), py_str("stop"), py_str("swap"), py_str("use"), py_str("ring"), py_str("call"), py_str("check"), py_str("top"), py_str("wear"), py_str("cut"), py_str("leave"), py_str("unplug"), py_str("turn"), py_str("sit"), py_str("drink"), py_str("wrap"), py_str("raise"), py_str("prop"), py_str("wash"), py_str("rinse"), py_str("soak"), py_str("cover"), py_str("avoid"), py_str("try"), py_str("start"), py_str("do"), py_str("give"), py_str("pick"), py_str("choose"), py_str("compare"), py_str("buy"), py_str("order"), py_str("fill"), py_str("print"), py_str("ask"), py_str("tell"), py_str("move"), py_str("set"), py_str("open"), py_str("close"), py_str("save"), py_str("read"), py_str("write"), py_str("clean"), py_str("clear"), py_str("make"), py_str("budget"), py_str("shop"), py_str("cool"), py_str("reheat"), py_str("freeze"), py_str("drain"), py_str("store"), py_str("boil"), py_str("bake"), py_str("chill"), py_str("go"), py_str("get"), py_str("book"), py_str("phone"), py_str("add"), py_str("apply"), py_str("hold"), py_str("heat"), py_str("cook"), py_str("remove"), py_str("dig"), py_str("plant"), py_str("water"), py_str("measure"), py_str("log"), py_str("email"), py_str("text"), py_str("fill"), py_str("mix"), py_str("line"), py_str("air"), py_str("bathe"), py_str("blot"), py_str("dab"), py_str("sponge"), py_str("rub"), py_str("wipe"), py_str("dry"), py_str("pull"), py_str("empty"), py_str("hang"), py_str("lay"), py_str("press"), py_str("find"), py_str("look"), py_str("run"), py_str("work"), py_str("change"), py_str("split"), py_str("pat"), py_str("stir"), py_str("track"), py_str("iron"), py_str("serve"), py_str("roll"), py_str("fix"), py_str("follow"), py_str("build"), py_str("replace"), py_str("send"), py_str("stand"), py_str("draw"), py_str("rehearse"), py_str("practise"), py_str("practice"), py_str("recite"), py_str("tighten"), py_str("blend"), py_str("stretch"), py_str("skip"), py_str("practise"), py_str("watch"), py_str("learn"), py_str("offer"), py_str("report"), py_str("stay"), py_str("seal"), py_str("trim"), py_str("prime"), py_str("screw"), py_str("weigh"), py_str("scrape"), py_str("simmer"), py_str("roast"), py_str("grill"), py_str("shred"), py_str("mash"), py_str("chop"), py_str("begin"), py_str("feed"), py_str("place"), py_str("switch"), py_str("fit"), py_str("test"), py_str("brush"), py_str("scrub"), py_str("pour"), py_str("grate"), py_str("slice"), py_str("spray"), py_str("prune"), py_str("repot"), py_str("balance"), py_str("let"), py_str("be"), py_str("eat"), py_str("scour"), py_str("flush"), py_str("vacuum"), py_str("hoover"), py_str("peel"), py_str("fold"), py_str("loosen"), py_str("walk"), py_str("tuck"), py_str("settle"), py_str("swaddle"), py_str("treat"), py_str("pack"), py_str("pay"), py_str("photograph"), py_str("aim"), py_str("melt"), py_str("halve"), py_str("agree"), py_str("shut"), py_str("sort"), py_str("repair"), py_str("label"), py_str("install"), py_str("cancel"), py_str("plug"), py_str("restart"), py_str("reset"), py_str("scan"), py_str("reply"), py_str("spend"), py_str("bin"), py_str("tidy"), py_str("sell"), py_str("throw"), py_str("push"), py_str("answer"), py_str("mend"), py_str("rent"), py_str("bring"), py_str("separate")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LIST_STEP_LINES(py_tuple(vec![py_str("Start with the first thing on that list: {step}."), py_str("The first one on that list: {step}.")]));
    s_STEP_LINES(py_tuple(vec![py_str("{Step} first."), py_str("{Step} - that is the first move."), py_str("{Step}, before anything else.")]));
    s_STEP_DETAIL_LINES(py_tuple(vec![py_str("{Step}. That is the first move."), py_str("{Step}. Start there."), py_str("{Step}. Do that before anything else.")]));
    s_FIRST_ITSELF(stdlib_call("re", "compile", &[py_str("\\b(?:start|begin|first)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_ALREADY_FIRST_LINES(py_tuple(vec![py_str("{Step}. Nothing else before it."), py_str("{Step}, and only that to begin with."), py_str("{Step}, in that order.")]));
    s_SUBJECT_OPEN(stdlib_call("re", "compile", &[py_str("^(?:they|it|it's|its|he|he's|she|she's|this|that|that's|these|those|there|there's|we|we're)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPLANATION_WORDS(Py::Int(5));
    s_STEP_ONE_LINES(py_tuple(vec![py_str("{Step}, and take it from there."), py_str("{Step}. That is the one to do today; the rest is waiting to see whether it shifts."), py_str("{Step} - start with that, and tell me how it goes.")]));
    s_STEP_KEPT_LINES(py_tuple(vec![py_str("Still the same move, and nothing to add to it: it is small, and it is the one that counts today."), py_str("That step stands - it is a thing to keep doing rather than a second thing to start."), py_str("Nothing new from me on it. Do the first thing as I said it, and tell me how it goes.")]));
    s_ALSO_LINES(py_tuple(vec![py_str("There is more in what I said: {other}"), py_str("One more from that answer: {other}"), py_str("The other half of it: {other}")]));
    s_ASKS_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:tell me|let me know|say which|what have you|how ready)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ONE_TURN_LINES(py_tuple(vec![py_str("Just this: {Step}."), py_str("The one thing: {Step}."), py_str("Start with this: {Step}."), py_str("Just the one, then: {Step}.")]));
    s_ONE_TURN_PLAIN_LINES(py_tuple(vec![py_str("{Step}. That is the one I would start with."), py_str("{Step}. Just that one, and the rest can wait."), py_str("{Step} - the one to do today, and the rest will keep.")]));
    s_ALL_OF_IT_LINES(py_tuple(vec![py_str("That answer is the whole of what I have on it, and there is no single first move in it: tell me which bit you want and I will take that one further."), py_str("Everything I know on that one is in the answer above, and none of it is one job to start with. Say which part matters most and I will come at it from there.")]));
    s_QUESTION_BACK_LINES(py_tuple(vec![py_str("Nothing to do on my side, honestly - whatever comes to mind about it first is the useful bit."), py_str("No steps from me yet. Take the question in your own time, and start with whatever is loudest."), py_str("There is nothing to do about it until we have looked at it - so start there, in your own time.")]));
    s_ASKED_FIRST_LINES(py_tuple(vec![py_str("I can be specific if you tell me this much: {q}? Even a line is enough."), py_str("One thing and I can be concrete: {q}?"), py_str("Tell me this and I can say something useful: {q}?")]));
    s_OFFER_TAKEN_LINES(py_tuple(vec![py_str("Yes - the one thing that is bothering you most, in your own words, and I will start there."), py_str("Then give me the thing itself. One sentence is enough, and I will pick the first step from it."), py_str("Go on then: name the one part that is worst, and we will take it from there.")]));
    s_SEE_NOW_LINES(py_tuple(vec![py_str("See someone if {when}. Before that, what I said above is the plan."), py_str("Not yet, unless {when}. If that happens, get it looked at rather than waiting."), py_str("The point where it wants a person is if {when}. Until then, what I said above is the plan.")]));
    s_SEE_ANYWAY_LINES(py_tuple(vec![py_str("That one is a visit rather than a wait, so yes - get it looked at."), py_str("Yes. Ring them rather than waiting on it, and say what you have noticed so far."), py_str("Nothing to sit on there: that is one to get looked at.")]));
    s_SEE_STATED_LINES(py_tuple(vec![py_str("Nothing there needs anyone today. That changes with {when}."), py_str("Not yet. The point where it wants a person: {when}. Short of that, what I said above is the plan."), py_str("What would change that: {when}. Until then, what I said above is the plan.")]));
    s_NOTHING_LINES(py_tuple(vec![py_str("Nothing you need to do yet - it matters if {when}."), py_str("Nothing to do for now. That changes if {when}."), py_str("No steps to take: what I said only changes if {when}.")]));
    s_SEE_NONE_LINES(py_tuple(vec![py_str("Nothing in what I said needs anyone today. If it has not settled in a week or so, that changes."), py_str("Not yet. Give what I described a few days; if it is no better, that is when to ask.")]));
    s_POOLS(py_tuple(vec![g_NO_LINES(), g_DO_LINES(), g_YES_LINES()]));
    s_OWN_LINES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![g_NO_LINES(), g_DO_LINES(), g_YES_LINES(), g_STEP_LINES(), g_STEP_DETAIL_LINES(), g_STEP_ONE_LINES(), g_STEP_ALREADY_FIRST_LINES(), g_STEP_KEPT_LINES(), g_SEE_NOW_LINES(), g_SEE_ANYWAY_LINES(), g_SEE_STATED_LINES(), g_SEE_NONE_LINES(), g_NOTHING_LINES(), g_CONDITION_LINES(), g_NOTHING_WARNED_LINES(), g_ORDINARY_NORMAL(), g_CARE_NORMAL(), g_CARE_NORMAL_UNKNOWABLE(), g_CARE_NOW_NORMAL(), g_DO_NORMAL(), g_NOTHING_WARNED_NORMAL()])) { let mut pool: Py = __item.clone(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_PATTERN(stdlib_call("re", "compile", &[mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OWN_LINES()) { let mut line: Py = __item.clone(); __out.push(_own_pattern(&[line.clone()])); } py_list(__out) })() }]), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ACK_PATTERN(Py::None);
    s_ECHO_WORDS(Py::Int(14));
    s_READABLE(call_builtin("frozenset", &[py_tuple(vec![py_str("answers"), py_str("chat"), py_str("trouble"), py_str("fallback"), py_str("greeting")])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNREADABLE(call_builtin("frozenset", &[py_tuple(vec![py_str("safety"), py_str("urgent"), py_str("loss"), py_str("grief")])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPOSED(call_builtin("frozenset", &[py_tuple(vec![py_str("greeting"), py_str("fallback")])]).unwrap_or_else(|e| exc::raise(e)));
    s_RULED_BY_THE_COMPOSER(py_tuple(vec![g_ORDINARY_NORMAL(), g_CARE_NORMAL(), g_CARE_NORMAL_UNKNOWABLE(), g_CARE_NOW_NORMAL(), g_DO_NORMAL(), g_CONDITION_NORMAL(), g_NOTHING_WARNED_NORMAL(), g_NO_LINES(), g_BOTH_LINES(), g_DO_LINES(), g_CONDITION_LINES(), g_MET_LINES(), g_MET_CONDITION_LINES(), g_METHOD_OK_LINES(), g_YES_LINES(), g_BACKSTOP_LINES(), g_SOON_LINES(), g_HELPED_LINES(), g_MONEY_TROUBLE_LINES()]));
    s_RUN(Py::Int(4));
    s__WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s__AFTER_A_NOUN(call_builtin("frozenset", &[mcall(&py_str("at in on from of for to with by near under behind over into about"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PUNT_START(stdlib_call("re", "compile", &[py_str("^(?:got it|noted|ok(?:ay)?|alright|all right|right|fair enough|i see|makes sense|understood|sure|cool)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUNT_OFFER(stdlib_call("re", "compile", &[py_str("\\b(?:is there anything|anything else|what else|let me know|anything (?:i|we) can do)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUNT_CHAT(stdlib_call("re", "compile", &[py_str("\\b(?:good luck|sounds good|fair enough|nice one|lovely|alright then|ok then|well then|makes sense|hope (?:it|that) (?:goes|works))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ECHO_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an the and or so but then too that thats that's this it its it's is are was were be been\ni we you he she they his her their our your yours mine theirs\nmy me us them him got get note noted right ok okay\nsure yes yeah no not ah oh mm good fine well still just only also very really quite\nknow sounds thanks hope glad\nhave has had do did does for to of in on at with about from by as if\none two three four five six seven eight nine ten eleven twelve half twice both all some\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RECAP_FILLER(call_builtin("frozenset", &[mcall(&py_str("\nthing things one ones first next step steps again before said same short version part bit\nhere that this it its is are was the a an and or of to for in on at with about from\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_URGENT_CALL(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|a&e|accident and emergency|ambulance|111|out of hours|straight away|right away|today|tonight|do not wait|dont wait|immediately)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EMERGENCY_ORDER(stdlib_call("re", "compile", &[py_str("^\\W*(?:that's|that is|this is|it's|it is)\\s+(?:an?\\s+)?(?:emergency|ambulance\\b(?: job)?)|^\\W*treat (?:this|that) as (?:an? )?(?:emergency|ambulance)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EMERGENCY_CALL_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:ring|call|dial|phone)\\s+(?:999|911|112|an ambulance)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHECK_ON_A_PLAN_LINES(py_tuple(vec![py_str("Say which part you are unsure about and we can weigh it up together."), py_str("Tell me the bit that is nagging you and I will go through it with you."), py_str("Which part of it is the one you keep coming back to? That is usually the part worth settling first."), py_str("Put the two sides next to each other and the answer usually shows itself: what it costs you, and what it gives you. Which one is it for you?")]));
    s_A_CHECK_ON_A_PLAN(stdlib_call("re", "compile", &[py_str("^(?:and |so |but )?(?:is|was|does|did|do) (?:that|this|it) (?:be |sound |look |seem )?(?:fair|ok|okay|alright|all right|fine|make (?:any )?sense|sound (?:ok|okay|fine|fair))(?: then| though)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONSIDERING(stdlib_call("re", "compile", &[py_str("\\b(?:thinking (?:of|about)|considering|planning (?:to|on)|looking to|hoping to|(?:i|we) (?:really )?(?:want|need|fancy)|want(?:ing)? to)\\b[^.]{0,30}?\\b(?:get|getting|buy|buying|adopt|adopting|rescue|rescueing|book|booking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_DOING(stdlib_call("re", "compile", &[py_str("^i\\s+(?!am\\b|m\\b|'m\\b|have\\b|had\\b|havent\\b|got\\b|ve\\b|need\\b|want\\b|found\\b|noticed\\b|spotted\\b|saw\\b|seen\\b|might\\b|may\\b|could\\b|probably\\b)[a-z']+\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CAN_HURT(py_tuple(vec![py_tuple(vec![py_str("carbon monoxide"), stdlib_call("re", "compile", &[py_str("\\b(?:car|engine|van|generator|mower|bbq|barbecue|charcoal|stove)\\b(?:\\W+\\w+){0,4}?\\W+(?:running|left\\s+on|on|lit|going|burning)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:garage|shed|indoors?|inside|tent|room)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("a medicine"), stdlib_call("re", "compile", &[py_str("\\b(?:stop|stopping|quit|quitting|come\\s+off|coming\\s+off|double|doubling|halve|halving|skip|skipping|cut\\s+(?:it\\s+)?(?:in\\s+half|down)|take\\s+more|up\\s+the\\s+dose)\\b(?!\\s+(?:working|helping|hurting))(?:\\W+\\w+){0,4}?\\W+(?:insulin|antidepress\\w*|antibiotic\\w*|medication|medications|meds|tablets?|pills?|statins?|steroids?|inhaler|epipen|epi\\s*pen|blood\\s+pressure\\s+(?:tablets?|meds?)|dose|dosage|prescription)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), Py::None]), py_tuple(vec![py_str("the mains"), stdlib_call("re", "compile", &[py_str("\\b(?:rewire|re-wire|wire\\s+(?:it|up|the\\s+socket|a\\s+socket|the\\s+lights?)|fit\\s+(?:a\\s+|the\\s+)?(?:socket|fuse\\s*box|consumer\\s+unit)|move\\s+(?:a\\s+|the\\s+)?(?:socket|fuse\\s*box)|(?:socket|fuse\\s*box|consumer\\s+unit|wiring|boiler)\\b(?:\\W+\\w+){0,4}?\\W+(?:myself|ourselves|himself|herself))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), Py::None]), py_tuple(vec![py_str("that blade"), stdlib_call("re", "compile", &[py_str("\\b(?:angle\\s+grinder|grinder|chainsaw|chain\\s+saw|circular\\s+saw|table\\s+saw|chop\\s+saw|bench\\s+saw|saw)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:no\\s+guard|without\\s+(?:the\\s+|a\\s+|its\\s+)?guard|guard\\s+off|took\\s+(?:the\\s+|its\\s+)?guard\\s+off|no\\s+goggles|without\\s+goggles)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("a job like that"), stdlib_call("re", "compile", &[py_str("\\b(?:take|taking|knock|knocking|pull|pulling|remove|removing)\\s+(?:the\\s+|my\\s+|our\\s+)?(?:chimney|wall|ceiling|lintel)\\b(?:\\W+\\w+){0,3}?\\W+(?:out|down|off)\\b|\\b(?:on\\s+the\\s+roof|up\\s+(?:a|the)\\s+ladder|on\\s+(?:a|the)\\s+ladder)\\b(?:\\W+\\w+){0,3}?\\W+(?:myself|ourselves|alone|on\\s+my\\s+own|by\\s+myself)\\b|\\b(?:myself|ourselves)\\b(?:\\W+\\w+){0,4}?\\W+(?:on\\s+the\\s+roof|up\\s+(?:a|the)\\s+ladder|off\\s+the\\s+ladder)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), Py::None])]));
    s_NAMES_A_TRADE(stdlib_call("re", "compile", &[py_str("\\b(?:electrician|plumber|builder|roofer|chimney\\s+sweep|gas\\s+safe|engineer|handyman|scaffold\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_KEEP_OR_STOP(stdlib_call("re", "compile", &[py_str("\\b(?:keep|carry\\s+on|stopping|stop|do\\s+not|don\\u2019t|dont|never|ask|check|review|until|rather\\s+than)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PLAN_CAN_HURT_LINES(py_dict(vec![(py_str("carbon monoxide"), py_tuple(vec![py_str("Good that you checked. Not with the engine running in there, though - that is carbon monoxide, and it puts people to sleep before they notice it. Run it outside, and if the door has to stay shut, have somebody with you."), py_str("Right to ask. That one gives off carbon monoxide, and a garage or a tent holds it: do it out in the open, or with the doors wide open and somebody knowing you are in there."), py_str("Worth asking: an engine or a fire in a closed space is carbon monoxide, and that is the kind that takes people quietly. Move it outside before you start it.")])), (py_str("a medicine"), py_tuple(vec![py_str("Not without the person who prescribes it: stopping it, doubling it or cutting a tablet in half is the one change they want to hear about first. Ring the surgery, a pharmacist or 111 and ask what to do today."), py_str("That one goes through whoever prescribes it - a change of dose or a stop is not something to work out alone. Ring the surgery or a pharmacist, and keep taking it as written until they say otherwise."), py_str("Ask the prescriber before you change it, and carry on as written until then. A pharmacist will answer this the same day, and 111 will at night.")])), (py_str("the mains"), py_tuple(vec![py_str("Not by hand: that is mains, and it is not a thing to learn on. Get a registered electrician in - a socket or a fuse box done wrong is the sort of fault that waits."), py_str("Not on your own. Mains wiring has to be done and signed off by somebody registered, and the part that goes wrong is inside the wall where you cannot see it.")])), (py_str("that blade"), py_tuple(vec![py_str("Not with the guard off. That guard is the part that comes between you and the blade, and a saw does not give you a second go: put it back, or get somebody who has one."), py_str("Put the guard back on first, and goggles too - that is the half of the job that keeps your hands the shape they are.")])), (py_str("a job like that"), py_tuple(vec![py_str("Not from a ladder on your own, and not by taking it down yourself: that wants somebody who has done it before, and a second pair of hands. Get a builder or a chimney sweep round, and let them say what it is."), py_str("A job like that is one to hand over rather than climb to - a ladder on your own is how people get hurt at home. Ring somebody who does it for a living.")]))]));
    s_OVER_VERB(stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:\\w+ed|did|ran|got|went|came|made|took|won|bought|wrote|sent|met|found|gave|had|kept|left|put|saw|spoke|sold|paid|drove|flew|finished|passed|booked|handed|signed|told|started|stopped|began|broke|lost|held|built|caught|chose|felt)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_GOING(stdlib_call("re", "compile", &[py_str("\\b(?:yet|still|again|every (?:day|night|week|morning|time)|keeps?|always|constantly|anymore|any more|since|for (?:weeks|months|years|days)|all the time|at the moment|now|these days|lately)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SIGN(stdlib_call("re", "compile", &[py_str("\\b(?:sour|off|mouldy|moldy|curdled|smelly|smells|stinks|stinking|fizzy|rancid|turned|slimy|furry|green|black|gone bad|gone off|past it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_HOW_TO_TELL(stdlib_call("re", "compile", &[py_str("\\b(?:how do i (?:know|tell|check)|how can i tell|how to tell|how do you tell|how do i spot|what should i look for|what do i look for|what should i check|is it still (?:ok|okay|fine))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LATER_CONDITION(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|when|once|whenever|as soon as)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRADE(stdlib_call("re", "compile", &[py_str("\\b(?:gp|doctor|midwife|health visitor|111|999|112|a&e|dentist|pharmacist|optician|physio(?:therapist)?|nurse|vet|paramedic|district nurse|out of hours)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATED_BEFORE(stdlib_call("re", "compile", &[py_str("(?:\\bnot|\\bnever|\\bno|n't|\\bless|\\bhardly|\\bexcept|\\bwithout)\\s+(?:\\w+\\s+){0,2}$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FREQUENCY_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:every|most|each|all) (?:\\w+ )?(?:day|days|night|nights|morning|mornings|afternoon|afternoons|evening|evenings|week|weeks|month|months)\\b|\\ball the time\\b|\\balways\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FREQUENCY_ASKED(stdlib_call("re", "compile", &[py_str("\\bmost days\\b|\\bmost nights\\b|\\bevery day\\b|\\bregularly\\b|\\bseveral times\\b|\\bmore than (?:once|twice)\\b|\\bkeeps? (?:coming|happening|returning)\\b|\\btime and again\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_NOW(stdlib_call("re", "compile", &[py_str("\\btoday\\b|\\bnow\\b|\\btonight\\b|straight away|as soon as|\\btomorrow\\b|\\bthis (?:morning|afternoon|evening)\\b|\\bnext few hours\\b|\\bthe same day\\b(?!\\s+if\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_CLAUSE(stdlib_call("re", "compile", &[py_str(", (?:and |or |then |so )?| and | or ")]).unwrap_or_else(|e| exc::raise(e)));
    s_GO_FROM(py_tuple(vec![py_str("worth a gp"), py_str("gp visit"), py_str("worth a hearing test"), py_str("a referral"), py_str("for a referral"), py_str("worth getting looked at"), py_str("worth seeing")]));
    s_FENCED_IN_SENTENCE(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|once|when|whenever|in case|most days)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_WORDS(call_builtin("frozenset", &[mcall(&py_str("about after again also always another anything around because been\nbefore being both came cant could does doing done down during each even ever every from\ngetting give going good have having here just keep keeps know last like little long made make\nmany more most much must need needs never next only other over really right same should since\nsome something still such sure than that their them then there these thing things think this\nthose through time today tomorrow tonight under until very want wants well went were what when\nwhere which while will with would your yours yourself"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STATEMENT(stdlib_call("re", "compile", &[py_str("^(?:[a-z']+\\s+)(?:is|are|was|were|has|have|had|can|cannot|will|would|should|takes?|gives?|means?|costs?|comes?|goes?|does|did|gets?|keeps?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TARGET_SHAPE(stdlib_call("re", "compile", &[py_str("^(?:build|work|save|grow)\\s+up\\b|^(?:build|work|save|aim|get|grow|reach)\\b[^.!?]{0,24}?\\b(?:towards|toward|up to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIM(py_tuple(vec![py_str(" because "), py_str(" so it "), py_str(" so that "), py_str(" before "), py_str(" after "), py_str(" until "), py_str(" if "), py_str(" when "), py_str(" rather than ")]));
    s_TRIM_INSTRUMENT(py_tuple(vec![py_str(" using "), py_str(" with ")]));
    s_CONTRAST(stdlib_call("re", "compile", &[py_str("\\b(?:rather than|instead of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_WORDS(Py::Int(16));
    s_LONG_STEP_WORDS(Py::Int(24));
    s_STEP_LONG(Py::Int(10));
    s_COPULA(stdlib_call("re", "compile", &[py_str("^(?:[\\w,'-]+\\s+){0,5}(?:is|are|was|were)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DETAIL_STATEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITIONAL_OPEN(stdlib_call("re", "compile", &[py_str("^(?:if|unless|once|when|whenever|should|in case)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THE_TALK(stdlib_call("re", "compile", &[py_str("\\b(?:let me know|keep me posted|tell me how (?:it|that) (?:goes|went)|how (?:it|that) goes|say more|tell me more|fair enough|thanks for telling me|i'?m listening|good to know|noted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATIVE_OPEN(stdlib_call("re", "compile", &[py_str("^(?:do not|don't|dont|never)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELPS(stdlib_call("re", "compile", &[py_str("\\b(?:things?|ones?|bits?|steps?|changes?|habits?|moves?|tips?|fixes|tricks|what|everything|anything|these|those|the rest)\\b[^:]{0,40}?\\b(?:works?|worked|helps?|helped|matters?|counts?|do|does|did|worth (?:a|trying|doing|a go))\\b|\\b(?:do|does|did) more than\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEAD_REFUSES(stdlib_call("re", "compile", &[py_str("\\b(?:not|n't|depends|depend|differ|if|whether|unless)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DESCRIBES(stdlib_call("re", "compile", &[py_str("\\b(?:who|whom|whose|which|that)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTINUES(stdlib_call("re", "compile", &[py_str("^\\s*(?:or|but|so|yet|nor|otherwise)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_CONTINUES(stdlib_call("re", "compile", &[py_str("^\\s*(?:then|after\\s+that|next)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_AND(stdlib_call("re", "compile", &[py_str("\\b(?:between|both)\\b[^,;:.!?]*\\band\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_FRAME(stdlib_call("re", "compile", &[py_str("^(?:(?:so|and|but|or|ok|okay|right|well|then|now|look|listen|go on|go ahead|come on|tell me)\\b[\\s,\\u2014\\u2013.:;-]*)*(?:(?:do|did|does|would|will|can|could|shall|should|have|has|are|is|was|were)\\s+(?:you|i|we|they|he|she|it|there|that|this|anyone|anybody|someone)\\b|(?:what|why|how|where|who|which|whose)\\b)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OFFER_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:want (?:me to|a hand|me to)|would you like|shall i|can i help|may i help|what would help|how can i help|anything (?:else )?i can do|where would you like|do you want to (?:say|tell|talk)|where are you with|what(?:'s| is) it like|is there (?:anyone|someone|anybody|somebody)|want to (?:say|tell|talk))\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__INVITATION(stdlib_call("re", "compile", &[py_str("^(?:(?:do|would|will|can|could)\\s+you\\s+(?:want\\s+to\\s+|like\\s+to\\s+|care\\s+to\\s+|mind\\s+)?(?:tell\\s+me|talk\\s+(?:me\\s+through|about)|say|start)\\s*(?:a\\s+bit\\s+)?(?:more\\s+)?(?:about\\s+)?)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PLEA(stdlib_call("re", "compile", &[py_str("^(?:and |so )?what now$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_OFFER(stdlib_call("re", "compile", &[py_str("\\b(?:anything\\s+(?:else\\s+)?(?:you.?d|i\\s+can|to\\s+add)|anything\\s+on\\s+your\\s+mind|what\\s+else|something\\s+else|is\\s+there\\s+anything)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DASH(stdlib_call("re", "compile", &[py_str("^(.*?)\\s+[\\u2014\\u2013-]\\s+(.*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_OPENER(stdlib_call("re", "compile", &[py_str("^(?:so|and|but|or|ok|okay|right|well|then|now|look|listen|go on|go ahead|come on|tell me)$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_VERBS(call_builtin("frozenset", &[py_tuple(vec![py_str("comes"), py_str("come"), py_str("gets"), py_str("get"), py_str("turns"), py_str("turned"), py_str("starts"), py_str("started"), py_str("goes"), py_str("went"), py_str("spreads"), py_str("spread"), py_str("wakes"), py_str("woke"), py_str("lasts"), py_str("lasted"), py_str("keeps"), py_str("kept"), py_str("has"), py_str("have"), py_str("develops"), py_str("developed"), py_str("bleeds"), py_str("bled"), py_str("swells"), py_str("swelled"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("hurts"), py_str("stops"), py_str("stopped"), py_str("changes"), py_str("changed"), py_str("moves"), py_str("moved"), py_str("suddenly")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_HANGING(call_builtin("frozenset", &[mcall(&py_str("\nyou i he she we they it or and but so if when with of the a an is are was were been t\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REFER_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|needs|needed|means|wants|should|must|will\\s+need)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RESTATED(stdlib_call("re", "compile", &[py_str(",\\s*(?:that|this|which|it)\\s+(?:is|means|needs|wants|should)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RELATIVE(stdlib_call("re", "compile", &[py_str("\\b(?:that|which|who)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOOK_AND_SEE(call_builtin("frozenset", &[py_tuple(vec![py_str("see"), py_str("check"), py_str("know"), py_str("wait"), py_str("tell"), py_str("find"), py_str("wonder"), py_str("decide"), py_str("guess"), py_str("ask"), py_str("notice")])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGED_REFERRAL(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were) (?:a|the|your) reason to (?:ring|call|see|ask|contact|speak|get)\\b|\\btake (?:it|that|this|them|him|her) (?:to|along to|back to) (?:a |your |the )?(?:pharmacist|gp|doctor|nurse|vet|dentist|optician)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STATES_A_REFERRAL(stdlib_call("re", "compile", &[py_str("^(?P<when>.+?)\\s+(?:is|are)\\s+(?:high and |low and |still )?(?:worth|a reason to|the point (?:at which|where))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_NOTHING(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("they"), py_str("there"), py_str("which"), py_str("either"), py_str("one"), py_str("both"), py_str("any"), py_str("all"), py_str("any of that"), py_str("any of those"), py_str("any of them"), py_str("either of those"), py_str("either of them"), py_str("one of those"), py_str("both of those"), py_str("some of that"), py_str("all of that"), py_str("the rest"), py_str("one or both"), py_str("none of that"), py_str("anything"), py_str("something"), py_str("much"), py_str("most of it")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATED_LONGEST(Py::Int(16));
    s_DOING(bitor(&g_IMPERATIVES(), &call_builtin("frozenset", &[py_tuple(vec![py_str("run"), py_str("cool"), py_str("warm"), py_str("heat"), py_str("wipe"), py_str("spray"), py_str("scrape"), py_str("sand"), py_str("drain"), py_str("vacuum"), py_str("replace"), py_str("tighten"), py_str("loosen"), py_str("unscrew"), py_str("test"), py_str("brush"), py_str("scrub"), py_str("soak"), py_str("fill"), py_str("pour"), py_str("freeze"), py_str("thaw"), py_str("reheat"), py_str("serve"), py_str("add"), py_str("mix"), py_str("stir"), py_str("grate"), py_str("slice"), py_str("chop"), py_str("cover"), py_str("wrap"), py_str("store"), py_str("label"), py_str("weigh"), py_str("measure"), py_str("book"), py_str("text"), py_str("email"), py_str("write"), py_str("plant"), py_str("water"), py_str("prune"), py_str("feed"), py_str("repot"), py_str("keep"), py_str("hang"), py_str("place"), py_str("swap"), py_str("switch"), py_str("fit"), py_str("fix"), py_str("seal"), py_str("paint"), py_str("install"), py_str("bleed"), py_str("balance"), py_str("ventilate"), py_str("insulate"), py_str("lag"), py_str("defrost"), py_str("descale"), py_str("replace"), py_str("trim"), py_str("sweep"), py_str("mop"), py_str("hoover"), py_str("eat"), py_str("drink"), py_str("sleep"), py_str("nap"), py_str("walk"), py_str("stretch"), py_str("rest"), py_str("sit"), py_str("stand"), py_str("try")])]).unwrap_or_else(|e| exc::raise(e))));
    s_LONG_SPAN(stdlib_call("re", "compile", &[py_str("\\b(?:two|three|four|five|six|seven|eight|nine|ten|\\d+)\\s+(?:days?|weeks?|fortnights?)\\b|\\ba\\s+fortnight\\b|\\b\\d+\\s*(?:months|years)\\b|\\bmonths?\\b|\\byears?\\b|\\bfor\\s+(?:a\\s+long|ages)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGE_SPAN(stdlib_call("re", "compile", &[py_str("\\b(?:in|for|after|within)\\s+(?:a few|a couple of|a|an|one|two|three|four|five|six|several|\\d+)?\\s*(?:days?|weeks?|fortnights?)(?:\\s+or\\s+(?:two|three|four))?\\b|\\b(?:two|three|four|five|six|seven|\\d+)\\s+(?:days?|weeks?)\\s+of\\s+(?:it|this|that)\\b|\\b(?:lasts?|lasted|lasting|continues?|continued|continuing|goes? on|gone on|drags? on|more than|longer than|over|beyond|past)\\s+(?:a|an|one|two|three|four|five|six|seven|several|\\d+)?\\s*(?:days?|weeks?|fortnights?|months?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FENCE_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an the it its this that these those there is are was were be been being am i you your he she\nthey we my mine our their his her them us of to in on at for with and or but so then than if\nwhen while as by from up out off down not no any more very still just also both either own same\nother thing things way lot lots bit does do did done gets get getting goes going come comes\ncoming keep keeps kept start starts started stop stops stopped carry carries carried turn turns\nturned become becomes became make makes made up\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:died|dies|death|dead|funeral|cremation|burial|passed away|miscarriage|miscarried|stillborn|bereave\\w*|lost (?:the|my) baby|took (?:his|her|their) own life)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOOD(stdlib_call("re", "compile", &[py_str("\\b(?:feel(?:ing|s|t)?\\s+(?:\\w+\\s+){0,2}(?:low|down|flat|numb|sad|anxious|worried|stressed|overwhelmed|lonely|rubbish|awful|miserable|hopeless|empty|knackered|exhausted)|depress\\w*|anxi\\w*|panick?\\w*|miserable|lonely|burnt out|burned out|burnout|can(?:'t|t|not)? be bothered|no energy|no motivation|dont want to go out|dont want to see anyone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE_OK_LINES(py_tuple(vec![py_str("Yes - go with that. {step}"), py_str("That is the right first move, and it is the one I would take: {step}"), py_str("Go with it. {step} If it feels wrong once you are in it, say so and we can change tack.")]));
    s_ASKED_FOR_A_CALL(stdlib_call("re", "compile", &[py_str("^\\s*(?:so\\s+|ok(?:ay)?\\s+)?(?:should|shall|do you think|would you|what would you do|is it (?:a good idea|ok|okay|wise|worth it|fine) to|is it wrong to|am i (?:ok|okay|right) to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REFUSAL(stdlib_call("re", "compile", &[py_str("\\b(?:i can(?:no|')?t\\b|i cannot\\b|beyond what i know|not one i can call|i don't know enough|i would not want to guess|that is not something i can|not mine to decide|not mine to say|not for me to (?:decide|say|call)|(?:that|this|it) is your call|up to you whether)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIRECTED_LINES(py_tuple(vec![py_str("I wouldn't call it a warning sign - the answer above is about getting it sorted, and that is the part that matters. What has you asking?"), py_str("Nothing in that says how serious it is. The step above is what moves it, and what would change the answer is it getting worse or going on. Where is the worry?"), py_str("Hard to put a scale on it. Doing the thing above is what makes the difference, so start there and tell me what is behind the question.")]));
    s_OWED_LINES(py_tuple(vec![py_str("Then it is theirs to fix rather than yours. Put it in writing, keep a copy, and note the date it started — that is the thing that turns it from your complaint into their job."), py_str("That one is their job, and the date is what makes it stick. Send it in writing, keep the reply, and say when it started: anyone who has to step in later asks for that first."), py_str("Keep it in writing and keep the dates. A landlord, a council or a boss who has been told in writing is a different conversation from one who has only been told.")]));
    s_OWED(stdlib_call("re", "compile", &[py_str("\\b(?:landlord|landlady|letting agent|estate agent|council|housing association|housing officer|boss|manager|employer|supervisor|neighbour|neighbor|builder|contractor|freeholder|owner|letting|lettings)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOUBT_ABOUT_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:safe|safely|unsafe|dangerous|danger|ok|okay|fine|risky|risk|bad|wrong|worried|worry|worrying|problem|normal|harm|harmful)\\b|\\b(?:can|could|may|should) i (?:still |even )?(?:use|keep using|leave|carry on using)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HANDED_BACK_LINES(py_tuple(vec![py_str("That is worth acting on rather than watching: {referral}"), py_str("It needs sorting rather than waiting: {referral}"), py_str("Not one to sit on: {referral}")]));
    s__A_RANGE_OF_ITS_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:between|from) (\\d+(?:\\.\\d+)?) (?:and|to) (\\d+(?:\\.\\d+)?) ?([a-z]{2,10})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_REFERRAL_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:get|have|call|ring|book) (?:a |an |your |the )?(?:(?!and\\b|or\\b|to\\b|it\\b)[a-z-]+ ){0,3}(?:engineer|plumber|electrician|mechanic|technician|specialist|someone) ?(?:out|round|in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_STOP(stdlib_call("re", "compile", &[py_str("\\b(?:stop|don't|dont|do not) (?:[a-z]+ing|it|using it|driving|eating)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__STEP_OPENERS(bitor(&g_DOING(), &call_builtin("frozenset", &[py_tuple(vec![py_str("say"), py_str("deal"), py_str("mention"), py_str("flag"), py_str("chase"), py_str("push"), py_str("insist")])]).unwrap_or_else(|e| exc::raise(e))));
    s_ASKED_THEM_LINES(py_tuple(vec![py_str("Nothing you have told me makes it bad or fine on its own - that depends on how much it is getting to you. What is the bit that worries you most?"), py_str("No verdict from me on that one yet. Say what has you asking and I will tell you whether it is worth doing something about."), py_str("That depends on something I have not heard yet. What is it that has you wondering?")]));
    s_A_WORRY_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|kept|wont|won't|refuses?|never|won't stop|hate|hates|hated|awful|horrible|rude|nasty|unfair|upset|angry|annoyed|miserable|cry|crying|cried|stress|stressed|worried|worry|worries|exhausted|knackered|drained|tired|hits?|hit me|shouts?|shouting|yells?|yelling|ignores?|ignoring|off with|weird|odd|wrong|problem|falling out|fell out|argument|arguing|argue|fights?|fighting|bully|bullied|bullying|debt|debts|skint|broke|struggling|struggle|too much|fed up|sick of|dreading|scared|afraid|lonely|alone|not talking|stopped talking|on top of me|getting on top|cant cope|can't cope|coping|breaking point|drowning|shattered|rough week|bad week|a lot on|driving me|drives me|nightmare|cant stand|can't stand|nagging|bossy|unbearable|hell|temper|disrespectful|mean to me|picking on me|picks on me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_OFFER(stdlib_call("re", "compile", &[py_str("\\b(?:if you want|if it helps|i can help|happy to help|tell me more|say more|i'm listening|i am listening|want to talk|would you like|do you want)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_OVER_WORDS(Py::Int(42));
    s_FRONT_LEAD(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("your"), py_str("our"), py_str("their"), py_str("his"), py_str("her"), py_str("its"), py_str("one")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FRONT_THIS(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("they"), py_str("these"), py_str("those"), py_str("there")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FRONT_WHEN(call_builtin("frozenset", &[py_tuple(vec![py_str("when"), py_str("if"), py_str("once"), py_str("unless"), py_str("whenever"), py_str("till"), py_str("until")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FRONT_SUBJECT_WORDS(Py::Int(2));
}
pub fn _a_job_somewhere(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when any clause of a reply is an instruction, not only the first one.\n\n    `opens_on_a_job` reads the opening because a bare \"what do i do\" wants the reply that starts\n    with the job. The step a follow-up points at is not always the opening one: the boiler answer\n    that ends \"...\" / \"ok and what about the radiators, they're cold at the top\" answers with\n    \"Radiators that are hot at the bottom and cold at the top have air in them: open the bleed\n    valve at the top with a radiator key ...\", which opens on a fact and carries its job after\n    the colon. \"and does that fix the pressure too\" after it still means the bleeding, so the\n    job has to be read wherever it is in the line.\n    ");
    flat = _flat(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&flat.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[:;.!?]|,?\\s+and\\s+"), flat.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    clause = __item.clone();
    words = mcall(&g__WORD(), "findall", &[mcall(&clause.clone(), "strip", &[py_str(" ,-")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_IMPERATIVES(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("me"), py_str("us")]), &geti(&words.clone(), &Py::Int(1)))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__AFTER_A_NOUN(), &geti(&words.clone(), &Py::Int(1)))) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_fix_asked(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut last: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A line for \"and does that fix the pressure too\", or \"\" when the line is not that ask.\n\n    Read by the chat module: the step was given a turn ago and the question is whether doing it\n    is the fix. Two things have to hold, and each of them is a way of leaving the turn alone:\n\n    * the last thing the bot said opens on a job (`opens_on_a_job`) - there is a step for \"that\"\n      to point at, and the person has already read it;\n    * the line is short and asks about fixing - a long line brings its own subject, and a line\n      that asks what to do next, how long it takes or why it happens is another question that\n      the answers entry or the verdict line answers better.\n    ");
    flat = _flat(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&flat.clone()))) {
    return py_str("");
    }
    words = mcall(&flat.clone(), "split", &[]);
    if py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__ASKS_ABOUT_A_STEP(), "search", &[flat.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_FIX_WORD(), "search", &[flat.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g__ASKS_FOR_THE_STEP(), "match", &[flat.clone()])) {
    return py_str("");
    }
    last = _last_reply(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = opens_on_a_job(&[last.clone()]); if py_truthy(&__b) { __b } else { _a_job_somewhere(&[last.clone()]) } })) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_STEP_TO_WATCH(), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("last_answer")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("domain")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_NOT_A_STEP_TO_WATCH_WORDS(), "search", &[last.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_WAIT_IN_THE_STEP(), "search", &[last.clone()])) {
    return _say(&[g_FIX_ASKED_LINES(), memory.clone(), flat.clone()]);
    }
    return _say(&[g_FIX_ASKED_NOW_LINES(), memory.clone(), flat.clone()]);
    Py::None
}
pub fn _the_list_before_the_call(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut m: Py = Py::None;
    let mut part: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("\"A leg he won't put weight on, a swelling, or ...\" - the answer's own list of signs.");
    best = py_str("");
    let __iter1 = py_iter_or_raise(&mcall(&g_NEEDS_A_CALL(), "finditer", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    m = __item.clone();
    part = mcall(&g__LIST_JOIN(), "sub", &[py_str(""), mcall(&mcall(&m.clone(), "group", &[py_str("list")]), "strip", &[py_str(" ,;:-")])]);
    part = mcall(&part.clone(), "strip", &[py_str(" ,;:-")]);
    words = mcall(&part.clone(), "split", &[]);
    if py_truthy(&{ let __b = if py_truthy(&le(&Py::Int(3), &{ let __cmp20 = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp20.clone() })) { le(&__cmp20.clone(), &Py::Int(24)) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = part.clone();
    }
    }
    if py_truthy(&{ let __b = best.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("A"), py_str("An"), py_str("The")]), &geti(&mcall(&best.clone(), "split", &[]), &Py::Int(0)))) } }) {
    best = add(&mcall(&geti(&best.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&best.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return best.clone();
    Py::None
}
pub fn _the_call_and_its_reason(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut m: Py = Py::None;
    let mut other: Py = Py::None;
    let mut s: Py = Py::None;
    let mut said_signs: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The answer's own \"see a GP for X\" and \"X is 111 today\", said as one clause, or \"\".\n\n    Only for the \"when should i worry\" ask, which wants the answer's condition and never a\n    verdict on the thing. Two at most: a card listing more than two calls has written a\n    paragraph, and half of a paragraph read back is worse than the pool.\n    ");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_CALLS_FOR(), "finditer", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    m = __item.clone();
    sign = _no_hanging_end(&[mcall(&mcall(&m.clone(), "group", &[py_str("list")]), "strip", &[py_str(" ,;:-")])]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp20 = call_builtin("len", &[mcall(&sign.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); __cmp20.clone() })) { le(&__cmp20.clone(), &Py::Int(20)) } else { Py::Bool(false) }))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![mcall(&m.clone(), "start", &[]), sign.clone(), _the_person_for(&[mcall(&m.clone(), "group", &[py_str("who")]), mcall(&m.clone(), "group", &[py_str("quick")])])])]);
    }
    let __iter3 = py_iter_or_raise(&mcall(&g_IS_A_NUMBER(), "finditer", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter3 {
    m = __item.clone();
    sign = _no_hanging_end(&[mcall(&mcall(&m.clone(), "group", &[py_str("when")]), "strip", &[py_str(" ,;:-")])]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp40 = call_builtin("len", &[mcall(&sign.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); __cmp40.clone() })) { le(&__cmp40.clone(), &Py::Int(20)) } else { Py::Bool(false) }))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![mcall(&m.clone(), "start", &[]), sign.clone(), _the_person_for(&[mcall(&m.clone(), "group", &[py_str("who")]), mcall(&m.clone(), "group", &[py_str("quick")])])])]);
    }
    said_signs = py_list(vec![]);
    let __iter5 = py_iter_or_raise(&call_builtin("sorted", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 3, None);
    _u = __unpk6[0].clone();
    sign = __unpk6[1].clone();
    who = __unpk6[2].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_signs.clone()) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&other.clone(), &sign.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    said_signs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_signs.clone()) { let mut s: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&sign.clone(), &s.clone()))) {  __out.push(s.clone()); } } py_list(__out) })() };
    let _ = mcall(&said_signs.clone(), "append", &[if py_truthy(&who.clone()) { modulo(&py_str("%s (%s)"), &py_tuple(vec![sign.clone(), who.clone()])) } else { sign.clone() }]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said_signs.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said_signs.clone()))) {
    return py_str("");
    }
    clause = mcall(&py_str("; "), "join", &[said_signs.clone()]);
    return if py_truthy(&mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) { add(&mcall(&geti(&clause.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { clause.clone() };
    Py::None
}
pub fn _the_sign_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut call: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "call") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: call"))) };
    let mut signs: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The content words of the signs in the quoted call, without the calls themselves.");
    signs = stdlib_call("re", "sub", &[py_str("\\([^)]*\\)"), py_str(" "), call_builtin("str", &[{ let __b = call.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&signs.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = ge(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_A_SIGN_WORD(), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _they_named_the_sign_themselves(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut call: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "call") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: call"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the person's own line already says the sign the call is for.");
    words = _the_sign_words(&[call.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    said = _flat(&[{ let __b = _the_line_behind(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return call_builtin("bool", &[bitand(&words.clone(), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_person_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut quick: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "quick") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: quick"))) };
    let mut hurry: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut trade: Py = Py::None;
    let _ = py_str("\"See a GP the same day\" -> \"a GP the same day\"; \"111 today\" keeps its number.");
    trade = mcall(&mcall(&{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&trade.clone()))) {
    return py_str("");
    }
    trade = mcall(&py_dict(vec![(py_str("gp"), py_str("GP")), (py_str("a&e"), py_str("A&E")), (py_str("nhs"), py_str("NHS"))]), "get", &[trade.clone(), trade.clone()]);
    if py_truthy(&{ let __b = mcall(&geti(&trade.clone(), &Py::Int(0)), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("A&E"), py_str("NHS")]), &trade.clone())) } }) {
    piece = trade.clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("GP"), py_str("doctor"), py_str("practice nurse"), py_str("nurse"), py_str("surgery"), py_str("vet"), py_str("dentist"), py_str("optician"), py_str("midwife"), py_str("physio"), py_str("physiotherapist")]), &trade.clone()))) {
    piece = add(&py_str("your "), &trade.clone());
    } else {
    piece = add(&py_str("a "), &trade.clone());
    }
    }
    hurry = mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), call_builtin("str", &[{ let __b = quick.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,")]);
    return if py_truthy(&hurry.clone()) { mcall(&add(&add(&piece.clone(), &py_str(" ")), &hurry.clone()), "strip", &[]) } else { piece.clone() };
    Py::None
}
pub fn _about_a_body(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut texts: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut text: Py = Py::None;
    let mut texts: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when one of these lines names a body part or a symptom.\n\n    Callers pass the line the person typed, not the answer: an answer says \"back the files up\",\n    and read as a body that made the laptop answer a remedy (\"give it a week or two\" over \"leave\n    it two days\"). The line the person typed is what the check is about.\n    ");
    let __iter1 = py_iter_or_raise(&texts.clone());
    for __item in __iter1 {
    text = __item.clone();
    words = stdlib_call("re", "findall", &[py_str("[a-z'-]+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_BODY_WORD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_doubt_about_money(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the doubt this turn is about money, or \"\".\n\n    Read off the person's own line one turn back, not off the answer: the answer over a money\n    line is a knowledge card about rent or a card or a bill, and its own \"what would matter is it\n    getting worse\" is the thing this replaces.\n    ");
    said = _flat(&[{ let __b = _last_person_line(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(16)) } }) {
    return py_str("");
    }
    words = call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g_MONEY_NOUN_WORDS())))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_MONEY_NOT_MET(), "search", &[said.clone()])))) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_BODY_IN_THE_LINE(), "search", &[said.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_NOT_MONEY_PAY(), "search", &[said.clone()])) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn _is_a_comparison(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the answer above quotes a usual figure for the person's own to be read against.");
    if py_truthy(&mcall(&g_A_FIGURE(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_is(&_first(&[said.clone(), g_A_COMPARISON()]), &Py::None));
    Py::None
}
pub fn _a_figure_owed(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the person's line is an amount of money rather than a condition of a body.");
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[A-Za-z\\u00a3$\\u20ac']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&w.clone(), "replace", &[py_str("'"), py_str("")]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BODY_WORD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_MONEY_WORD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_is(&mcall(&g_A_FIGURE(), "search", &[line.clone()]), &Py::None));
    Py::None
}
pub fn _is_an_explanation(args: &[Py]) -> Py {
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
    let _ = py_str("True for a sentence that says how something is, and so holds no step of its own.");
    text = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SUBJECT_OPEN(), "match", &[text.clone()])))) {
    return Py::Bool(false);
    }
    head = geti(&stdlib_call("re", "split", &[py_str(":|,|;| - | \\u2014 "), text.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    return gt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_EXPLANATION_WORDS());
    Py::None
}
pub fn _own_pattern(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("A line of this composer as a pattern, with the step or the cause in it left open.\n\n    They are templates, so \"Keep it plain, and take it from there.\" does not contain \"{Step}\",\n    and matching the template as it is written never finds the sentence the composer said. A\n    second follow-up would then read the composer's own line back as if it were the answer\n    behind it, and answer the answer.\n    ");
    parts = stdlib_call("re", "split", &[py_str("\\{[A-Za-z]+\\}"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    return mcall(&py_str(".+?"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    Py::None
}
pub fn takes_a_fact_back(args: &[Py]) -> Py {
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
    let mut frames: Py = Py::None;
    let mut line: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line only says the bot heard, so the answer behind it is still the answer.\n\n    \"my son is 4 and wont sleep\" / \"he shares with his sister\" / \"any tips\" was answered \"Sure.\n    What have you tried so far?\", a plea thrown back at somebody the bot had just told what to do,\n    because the acknowledgement sat between the ask and the answer it is about.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__ACK_PATTERN(), &Py::None))) {
    let mut ack: Py = crate::gen::memory::ack::module_obj();
    frames = add(&add(&add(&gattr(&ack.clone(), "FRAMES"), &gattr(&ack.clone(), "HABIT_TAIL")), &gattr(&ack.clone(), "GOING_TAIL")), &gattr(&ack.clone(), "EXPLAIN_TAIL"));
    s__ACK_PATTERN(stdlib_call("re", "compile", &[mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut line: Py = __item.clone(); __out.push(_own_pattern(&[line.clone()])); } py_list(__out) })() }]), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    }
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__ACK_PATTERN(), "match", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&text.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ECHO_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_IMPERATIVES(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn _say(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut flat: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "flat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: flat"))) };
    let mut gaps: Py = py_dict(__kw.iter().filter(|(n, _)| !["pool", "memory", "flat"].contains(&n.as_str())).map(|(n, v)| (py_str(n), v.clone())).collect());
    let mut filled: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut gaps: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut rendered: Py = Py::None;
    let mut template: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("One line from a pool of templates, chosen after its gaps are filled in.\n\n    pick() drops the reply just given, but the pool holds \"{Step}, and take it from there.\" and\n    the reply just given was \"Keep it plain, and take it from there.\", so the two are never equal\n    and a person asking twice got the same sentence twice running. Filling the gaps first is what\n    lets the never-repeat rule see them.\n    ");
    rendered = call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&gaps.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    value = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![name.clone(), modulo(&py_str("{%s}"), &name.clone())]), py_tuple(vec![add(&mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None)), modulo(&py_str("{%s}"), &add(&mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None)))])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    template = __unpk4[0].clone();
    gap = __unpk4[1].clone();
    filled = if py_truthy(&Py::Bool(py_eq(&gap.clone(), &modulo(&py_str("{%s}"), &name.clone())))) { value.clone() } else { add(&mcall(&slice_or_raise(&value.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None)) };
    rendered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rendered.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "replace", &[gap.clone(), filled.clone()])); } py_list(__out) })() };
    }
    }
    return crate::gen::compose::pick(&[rendered.clone(), memory.clone(), flat.clone()]);
    Py::None
}
pub fn _a_report_line(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the bot's own last line was the composer's report on the advice it had given.\n\n    The report lines are spoken under whichever module proposes them (`greeting` under the floor),\n    so the module name says nothing and the composer's own list of them does. A doubt two turns\n    after the answer - \"im 34 and i still bite my nails\" / \"ive tried everything\" / \"is that bad\" -\n    is about the nails, and the report in between is not the answer it is about.\n    ");
    let __try1 = exc::catch(|| {
    let mut is_a_report_reply: Py = py_func("is_a_report_reply", |args: &[Py]| crate::gen::compose::fallback::is_a_report_reply(args));
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
    return call_builtin("bool", &[py_call(&is_a_report_reply, &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _flat(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn rules_on_the_doubt(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("True when a reply is one of the composer's lines that rules on a doubt, for the arbiter.\n\n    The pools carry no slots in the part that tells them apart - a {when} or {topic} comes later\n    in the sentence - so the first 40 characters are compared, the way `fallback.is_a_hedge` reads\n    its pools, and a pool that reflows its wording keeps working.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    head = mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(40), &Py::None), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RULED_BY_THE_COMPOSER()) { let mut pool: Py = __item.clone(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(Py::Bool(py_eq(&head.clone(), &mcall(&slice_or_raise(&line.clone(), &Py::None, &Py::Int(40), &Py::None), "lower", &[])))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn opens_on_a_job(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut head: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a reply opens on an instruction (\"Ring or log in and ask why it went up\").\n\n    Read by the arbiter beside `carries_the_answer`, on the turn where a bare follow-up asks what\n    to do next: an entry that opens on a job is already answering that question, so it keeps the\n    turn over the step from the answer a turn ago. An entry that opens on a fact (\"Five a day is\n    400g\", \"The things that work: same wake-up time every day\") is telling the person something\n    else about the subject, and the step they asked for is in the answer they already have.\n    ");
    flat = _flat(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&flat.clone()))) {
    return Py::Bool(false);
    }
    head = mcall(&geti(&stdlib_call("re", "split", &[py_str("[:;.!?]"), flat.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(" ,-")]);
    words = mcall(&g__WORD(), "findall", &[head.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_IMPERATIVES(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("me"), py_str("us")]), &geti(&words.clone(), &Py::Int(1)))) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__AFTER_A_NOUN(), &geti(&words.clone(), &Py::Int(1)))) } };
    Py::None
}
pub fn carries_the_answer(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut runs: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when `reply` comes back to the answer the bot gave a turn ago.\n\n    Read by the arbiter (bot/modules/arbiter.py) on a turn that asks what to do next: the\n    composer's step line keeps the answer's own opening (\"Keep it plain, and take it from\n    there.\" after \"Keep it plain and separate: ...\"), and a hand-back keeps its first sentence,\n    so both share a run of four words with it. An entry that only shares the topic shares none:\n    \"Five a day is 400g, and a portion is a handful\" after an answer about a child refusing veg.\n    Wording only, and nothing about the subject, so no entry can be read as a repeat it is not.\n    ");
    if py_truthy(&_kept_step(&[reply.clone()])) {
    return Py::Bool(true);
    }
    said = mcall(&g__WORD(), "findall", &[_flat(&[_last_reply(&[memory.clone()])])]);
    words = mcall(&g__WORD(), "findall", &[_flat(&[reply.clone()])]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RUN()); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RUN()) } }) {
    return Py::Bool(false);
    }
    runs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[add(&sub(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RUN()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[slice_or_raise(&said.clone(), &i.clone(), &add(&i.clone(), &g_RUN()), &Py::None)])); } py_set(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[add(&sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RUN()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push(Py::Bool(py_contains(&runs.clone(), &mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &i.clone(), &add(&i.clone(), &g_RUN()), &Py::None)])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_punt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut module: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True for a bot turn that said nothing about the thing being discussed.");
    module = mcall(&last.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("greeting"), py_str("fallback"), py_str("chat")]), &module.clone()))) {
    return Py::Bool(false);
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PUNT_START(), "match", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&module.clone(), &py_str("chat"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PUNT_CHAT(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_PUNT_OFFER(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("?")))) {
    return Py::Bool(false);
    }
    return le(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8));
    Py::None
}
pub fn _skip_the_derived(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("The turns to read when the last one was read off the answer behind it.\n\n    A frequency line (\"Every night, then. That is enough to get it looked at\") and a healing line\n    (\"Most back pain is muscular and settles in two to six weeks\", said of the card above it) are\n    both taken from the answer before them rather than from anything of their own, so the card is\n    what a doubt this turn is about. Both are skipped, and no other turn is.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    return turns.clone();
    }
    if py_truthy(&{ let __b = mcall(&memory.clone(), "get", &[py_str("frequency_reply")]); if py_truthy(&__b) { __b } else { crate::gen::memory::healing::derived(&[memory.clone()]) } }) {
    return slice_or_raise(&turns.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    return turns.clone();
    Py::None
}
pub fn answers_the_ask(args: &[Py]) -> Py {
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
    let _ = py_str("True when there is a knowledge answer behind the ask for this composer to read.\n\n    A bare \"is that bad\" has no content of its own: the answer to it is in the bot's last line,\n    and that line is worth standing aside for only when it is an answer about a thing. After a\n    condolence (\"my dad died last week\") or a mood (\"i feel low\") the last line is neither, this\n    composer can only shrug, and a search hedge handed to someone who has just said their dad\n    died is worse than the line the chat pool would have given. The gate is the same test\n    reply_for makes for itself, asked from outside: the last readable turn was a knowledge answer\n    and this composer has a verdict to give about it. \"how do i descale the kettle\" / \"is that bad\"\n    is advice the person asked for, not a fault, and reply_for stands aside for it - so the chat\n    pool keeps that turn rather than leaving a search hedge where it had a warm line.\n    ");
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_nod".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_module(args))(&__args) }, &py_str("answers")))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&reply_for(&[clean.clone(), memory.clone()]), &Py::None));
    Py::None
}
pub fn _the_news_just_before(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the turn before this ask was hard news, not a feeling to sit with.\n\n    A comfort line that ends in a question is an offer about how the person is doing when the line\n    before it was about them (\"im feeling really low\"): there is nothing more concrete to say than\n    the first step of a bad day, so the question comes back. When the line before it was news that\n    names a thing (\"bailiffs turned up\", \"i got a letter about a court date\"), the ask that follows\n    is about the thing, and whoever read the news answers it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let mut reads_as_bad_news: Py = py_func("reads_as_bad_news", |args: &[Py]| crate::gen::compose::badnews::reads_as_bad_news(args));
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
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    turn = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { geti(&turns.clone(), &neg(&Py::Int(1))) } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { py_call(&reads_as_bad_news, &[said.clone()]) } };
    Py::None
}
pub fn _last_reply(args: &[Py]) -> Py {
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
    let mut past_the_nod: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "past_the_nod") { v.clone() }
    else { Py::Bool(false) };
    let mut past_the_verdict: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "past_the_verdict") { v.clone() }
    else { Py::Bool(false) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut module: Py = Py::None;
    let mut punt_line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last answer of the bot's this turn is a follow-up to, or \"\" if there is not one.\n\n    It is the bot's own last line whichever module wrote it, because the step a person asks for\n    is in that line and nowhere else: \"im getting headaches every afternoon\" is answered by the\n    composer's warm line, and \"what should i do\" after it used to get \"a quick search online\".\n    A safety or urgent answer is not read back (it has already said what to do now), and\n    neither is a condolence.\n\n    My own verdict line is not a change of subject: \"the boiler pressure is low\" / \"is that bad\"\n    / \"is that going to get worse\" is still the boiler, so the question is read from the answer\n    behind the verdict rather than dropped.\n\n    A cause handed back by another composer is the same case: \"the boiler keeps making a banging\n    noise\" / \"why is that\" / \"is it dangerous\" is still the boiler, so the reason line is read\n    past to the answer it came out of. Both are composer lines, spoken as the fallback module,\n    which is why the text decides it rather than the module name.\n    ");
    punt_line = py_str("");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    turns = _skip_the_derived(&[turns.clone(), memory.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    last = geti(&turns.clone(), &index.clone());
    before = if py_truthy(&index.clone()) { geti(&turns.clone(), &sub(&index.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return punt_line.clone();
    }
    module = mcall(&last.clone(), "get", &[py_str("module")]);
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g_UNREADABLE(), &module.clone()))) {
    return punt_line.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&module.clone(), &py_str("chat"))); if !py_truthy(&__b) { __b } else { mcall(&g_OWN_PATTERN(), "search", &[_flat(&[said.clone()])]) } }) {
    continue;
    }
    if py_truthy(&_a_report_line(&[said.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_COMPOSED(), &module.clone())); if !py_truthy(&__b) { __b } else { crate::gen::compose::why::is_a_reason_line(&[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_READABLE(), &module.clone())); if !py_truthy(&__b) { __b } else { takes_a_fact_back(&[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::g_A_STEP_ASK(), "match", &[_flat(&[mcall(&last.clone(), "get", &[py_str("user")])])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("answers"))) } }) {
    continue;
    }
    if py_truthy(&_a_punt(&[last.clone()])) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&past_the_nod.clone())); if py_truthy(&__b) { __b } else { punt_line.clone() } }) {
    return if py_truthy(&Py::Bool(!py_truthy(&past_the_nod.clone()))) { said.clone() } else { punt_line.clone() };
    }
    punt_line = said.clone();
    continue;
    }
    if py_truthy(&{ let __b = past_the_verdict.clone(); if !py_truthy(&__b) { __b } else { _only_a_verdict(&[mcall(&last.clone(), "get", &[py_str("user")])]) } }) {
    continue;
    }
    if py_truthy(&_is_an_echo(&[last.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _is_a_recap(&[last.clone(), before.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_READABLE(), &module.clone()))) {
    if py_truthy(&{ let __b = punt_line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&module.clone(), &py_str("answers"))) } }) {
    return punt_line.clone();
    }
    return said.clone();
    }
    return punt_line.clone();
    }
    return punt_line.clone();
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
    let mut word: Py = Py::None;
    let _ = py_str("The words of a line, lowercased and bare, for the echo test below.\n\n    The apostrophe goes, so \"he's two\" and the person's \"hes 2\" are the same two words.\n    ");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "replace", &[py_str("'"), py_str("")])); } py_list(__out) })() };
    Py::None
}
pub fn _is_an_echo(args: &[Py]) -> Py {
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
    let mut theirs: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the bot's line only says the person's own words back.\n\n    Short on purpose: a long line has said something of its own even if it repeats a lot of the\n    person's words (a paraphrase of what they described is an answer), so only a line of a dozen\n    words or fewer that adds nothing is skipped.\n    ");
    said = _words(&[mcall(&turn.clone(), "get", &[py_str("bot")])]);
    theirs = call_builtin("set", &[_words(&[mcall(&turn.clone(), "get", &[py_str("user")])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&theirs.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_ECHO_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&theirs.clone(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _is_a_recap(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut had: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the bot's line is the answer above, said shorter: a step quoted out of it.\n\n    The quote is shorter than the card it came from and holds no word the card does not, apart\n    from the lead-in (\"The thing to do first:\") and the words for saying it again. A line that\n    brings a word of its own is an answer of its own, however much it repeats.\n    ");
    said = _words(&[mcall(&turn.clone(), "get", &[py_str("bot")])]);
    had = call_builtin("set", &[_words(&[mcall(&before.clone(), "get", &[py_str("bot")])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&had.clone())); if py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[had.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&before.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&had.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_RECAP_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ECHO_FILLER(), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _last_line_that_named_a_call(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The last line when the answer the person is asking about named somebody to ring, or \"\".\n\n    A safety answer is the first case - \"my cat is having trouble breathing\" / \"is that serious\"\n    came back as the search hedge, because the verdict reading only reads a knowledge answer and\n    a safety line is not one. Another module hands a line on with a call in it too: \"my baby has\n    a rash on her cheeks\" is answered \"Your health visitor or the 111 line will talk it through\n    with you today\", and the verdict question after it has the same answer. A knowledge answer\n    never comes here - its own reading above is better than a yes - and a line that named nobody\n    has no call to point back at.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    return py_str("");
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&_first(&[said.clone(), g_CARE()]), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&g_REFER(), "search", &[said.clone()]), &Py::None)) } } } }) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn _a_call_for_now(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let mut trade: Py = Py::None;
    let _ = py_str("True when this line names somebody to ring or care for now, with no \"if\" on it.\n\n    \"Sorry to hear it — go easy on yourself today, and see someone if it's not shifting in a few\n    days\" is a greeting's own line, and the referral in it is fenced: \"i feel dizzy\" / \"is that\n    bad\" was told \"ringing is not an overreaction here, it is the whole point of it\" for a\n    dizziness that is ordinary. A cue with a condition after it in its own sentence is not a call\n    for now, and the verdict path carries on without it.\n    ");
    if py_truthy(&_care_for_now(&[said.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    found = mcall(&g_REFER(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    trade = mcall(&g_TRADE(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(py_is(&trade.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_LATER_CONDITION(), "search", &[slice_or_raise(&sentence.clone(), &Py::None, &mcall(&trade.clone(), "start", &[]), &Py::None)]), &Py::None))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_TIME_NOW(), "search", &[sentence.clone()]), &Py::None))) {
    return Py::Bool(true);
    }
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_LATER_CONDITION(), "search", &[slice_or_raise(&sentence.clone(), &mcall(&found.clone(), "start", &[]), &Py::None, &Py::None)])))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_TIME_NOW(), "search", &[slice_or_raise(&sentence.clone(), &mcall(&found.clone(), "start", &[]), &Py::None, &Py::None)])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_call_sentence(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut trade: Py = Py::None;
    let _ = py_str("The sentence that carries the call this answer makes, or the answer itself.\n\n    A long answer puts its fence and its call in one breath: \"a bent limb, bone through the skin or\n    numb fingers is 999, otherwise get it checked at A&E or a minor injuries unit today\" is a call\n    for today, and reading the whole answer for the emergency word made it a 999. The call is the\n    first sentence whose referral is not fenced, so that sentence is what \"how urgent\" is read\n    from.\n\n    A sentence that opens by naming the whole answer as an emergency has already given the order\n    and needs no time word to say when: \"That's an emergency: ring 999 or 911 and stay on the line\"\n    is a call for now, and so is \"Treat this as an emergency and ring 999\". Without that, the\n    safety lines that do not say \"straight away\" were read as a call for today instead, and a\n    doubt about one was answered \"Not an emergency by the sound of it\" over an answer that had\n    just said to ring 999.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_EMERGENCY_ORDER(), "match", &[sentence.clone()]), &Py::None))) {
    return sentence.clone();
    }
    opening = mcall(&g_EMERGENCY_CALL_OPEN(), "match", &[sentence.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&opening.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&g_LATER_CONDITION(), "search", &[slice_or_raise(&sentence.clone(), &mcall(&opening.clone(), "end", &[]), &Py::None, &Py::None)]), &Py::None)) } }) {
    return sentence.clone();
    }
    if py_truthy(&Py::Bool(py_is(&mcall(&g_TIME_NOW(), "search", &[sentence.clone()]), &Py::None))) {
    continue;
    }
    found = mcall(&g_REFER(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    tail = slice_or_raise(&sentence.clone(), &mcall(&found.clone(), "start", &[]), &Py::None, &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_is(&mcall(&g_LATER_CONDITION(), "search", &[tail.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_TIME_NOW(), "search", &[tail.clone()]), &Py::None)) } }) {
    return sentence.clone();
    }
    continue;
    }
    trade = mcall(&g_TRADE(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(py_is(&trade.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_LATER_CONDITION(), "search", &[slice_or_raise(&sentence.clone(), &Py::None, &mcall(&trade.clone(), "start", &[]), &Py::None)]), &Py::None))) {
    continue;
    }
    return sentence.clone();
    }
    return py_str("");
    Py::None
}
pub fn _last_person_line(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own last line, skipping the echoes a doubt ask follows.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    return said.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_worst_thing_they_said(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the thing this turn is a doubt about is a loss or a diagnosis, not a fault.\n\n    The person's own last line decides it: \"my dad has cancer\" is the thing they brought, and\n    \"is that bad\" on top of it is about that. A body in the line is still a body, so the doubt\n    path's own readings come first wherever there is something to read.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    if py_truthy(&said.clone()) {
    return if py_truthy(&mcall(&g_HARD_NEWS(), "search", &[said.clone()])) { said.clone() } else { py_str("") };
    }
    }
    return py_str("");
    Py::None
}
pub fn _as_written(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The line as it was meant, for the ask patterns that read it: (spelled right, said out).\n\n    \"wat shud i cook 2nite\" is \"what should i cook tonight\", and a misspelling is the person's\n    ordinary way of writing rather than a rare slip: the understanding engine repairs every line\n    and the composer was reading the raw text, so the same sentence spelled differently took two\n    different routes through this file. The first return is that repair with the words in the\n    order they were typed and the punctuation kept (`plain`); the second is the fuller reading\n    (`norm_text`), which also expands \"gonna\" and \"im\". A line with nothing to repair comes back\n    as it went in, and a reading that fails leaves the arms exactly as they were.\n    ");
    text = _flat(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    let __try1 = exc::catch(|| {
    read = crate::gen::understanding::normalize::normalise(&[text.clone()]);
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
    return py_tuple(vec![py_str(""), py_str("")]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return py_tuple(vec![_flat(&[{ let __b = mcall(&read.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } }]), _flat(&[{ let __b = mcall(&read.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } }])]);
    Py::None
}
pub fn _a_thing_they_own(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("The noun for the thing whose answer this doubt is about, or \"\" for a body or a person.\n\n    Read off the line the answer replied to, so it is the subject the person brought and not a\n    word in the doubt, and only when that line names no body: \"my boiler is making a knocking\n    noise\" is a machine, \"my knee clicks\" is not.\n    ");
    line = _the_line_behind(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    low = _flat(&[line.clone()]);
    if py_truthy(&mcall(&g_A_BODY_WORD(), "search", &[low.clone()])) {
    return py_str("");
    }
    found = mcall(&g_A_THING_THEY_OWN(), "search", &[low.clone()]);
    return if py_truthy(&found.clone()) { mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]) } else { py_str("") };
    Py::None
}
pub fn _a_person_they_know(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("The person whose situation this doubt is about, or \"\" when it is their body or nobody.\n\n    Read off the line the answer replied to, and only when that line is about contact, company or\n    quiet rather than a symptom: \"my mum is 79 and lives on her own\" is a week, \"my baby is not\n    feeding\" is not. A fenced care in the answer (\"if they seem low for weeks, help them see a GP\")\n    is the answer's own later step, not a referral now, so the verdict points at both halves.\n    ");
    line = _the_line_behind(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    low = _flat(&[line.clone()]);
    if py_truthy(&mcall(&g_A_SYMPTOM_WORD(), "search", &[low.clone()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_PERSON_THEY_KNOW(), "search", &[low.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_RELATION_TROUBLE(), "search", &[low.clone()]))) } }) {
    return py_str("");
    }
    return py_str("person");
    Py::None
}
pub fn _the_line_behind(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut module: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own line that the answer this turn is a follow-up to replies to, or \"\".\n\n    Read with the same walk `_last_reply` does, so the two agree about which answer is being\n    followed up: a chat line of this composer's own is stepped past, and a safety answer ends it.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    turns = _skip_the_derived(&[turns.clone(), memory.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    last = geti(&turns.clone(), &index.clone());
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    module = mcall(&last.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(py_contains(&g_UNREADABLE(), &module.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&module.clone(), &py_str("chat"))); if !py_truthy(&__b) { __b } else { mcall(&g_OWN_PATTERN(), "search", &[_flat(&[mcall(&last.clone(), "get", &[py_str("bot")])])]) } }) {
    continue;
    }
    if py_truthy(&_is_an_echo(&[last.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::g_A_STEP_ASK(), "match", &[{ let __b = _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]); if py_truthy(&__b) { __b } else { py_str("") } }]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&module.clone(), &py_str("answers"))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { _is_a_recap(&[last.clone(), geti(&turns.clone(), &sub(&index.clone(), &Py::Int(1)))]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_READABLE(), &module.clone()))) {
    return _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    }
    return py_str("");
    }
    return py_str("");
    Py::None
}
pub fn _the_plan_they_brought(args: &[Py]) -> Py {
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
    else { Py::Int(2) };
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own plan or choice among their last few lines, or \"\".\n\n    A plan is rarely the line immediately before the check. \"im thinking of getting a dog\" /\n    \"but i work all day\" / \"is that fair\" is one thought in three lines, and the line behind the\n    check is the *detail*, which no plan reader matches - so the arms below stood aside and the\n    acknowledgement pool ended the conversation (\"Sounds good.\"). Two lines is the reach: the plan\n    and one line said about it. Further back and the check belongs to whatever was said last.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = _flat(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    let __aug2 = add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&gt(&seen.clone(), &back.clone())) {
    break;
    }
    if py_truthy(&{ let __b = mcall(&g_A_PLAN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CONSIDERING(), "search", &[said.clone()]) } }) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _considering(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the line before this one is about a thing the person is considering getting.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = _flat(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&said.clone()) {
    return call_builtin("bool", &[mcall(&g_CONSIDERING(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_last_line_was_mine(args: &[Py]) -> Py {
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
    let mut module: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last line is a line of this composer's, whichever module filed it.\n\n    The pools that answer a question a second time (\"Same answer as a moment ago: nothing in what\n    I said was a warning sign\") are only true when the bot itself said the answer the first time.\n    Reading the words alone is not enough, because these lines are templates: `{Step} first.` fits\n    any sentence that ends in that word, and a knowledge card that says \"the last resort, not the\n    first\" is not a verdict of mine.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    module = mcall(&last.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(!py_truthy(&module.clone()))) {
    continue;
    }
    return { let __b = Py::Bool(py_eq(&module.clone(), &py_str("chat"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMPOSED(), &module.clone())) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _last_module(args: &[Py]) -> Py {
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
    let mut past_the_nod: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "past_the_nod") { v.clone() }
    else { Py::Bool(false) };
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut module: Py = Py::None;
    let mut punt_module: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The module that wrote the answer `_last_reply` reads, or \"\" if there is not one.");
    punt_module = py_str("");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    turns = _skip_the_derived(&[turns.clone(), memory.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    last = geti(&turns.clone(), &index.clone());
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return punt_module.clone();
    }
    module = mcall(&last.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(py_contains(&g_UNREADABLE(), &module.clone()))) {
    return punt_module.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&module.clone(), &py_str("chat"))); if !py_truthy(&__b) { __b } else { mcall(&g_OWN_PATTERN(), "search", &[_flat(&[mcall(&last.clone(), "get", &[py_str("bot")])])]) } }) {
    continue;
    }
    if py_truthy(&_a_report_line(&[mcall(&last.clone(), "get", &[py_str("bot")])])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_COMPOSED(), &module.clone())); if !py_truthy(&__b) { __b } else { crate::gen::compose::why::is_a_reason_line(&[_flat(&[mcall(&last.clone(), "get", &[py_str("bot")])])]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_READABLE(), &module.clone())); if !py_truthy(&__b) { __b } else { takes_a_fact_back(&[mcall(&last.clone(), "get", &[py_str("bot")])]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::g_A_STEP_ASK(), "match", &[_flat(&[mcall(&last.clone(), "get", &[py_str("user")])])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&module.clone(), &py_str("answers"))) } }) {
    continue;
    }
    if py_truthy(&_a_punt(&[last.clone()])) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&past_the_nod.clone())); if py_truthy(&__b) { __b } else { punt_module.clone() } }) {
    return if py_truthy(&Py::Bool(!py_truthy(&past_the_nod.clone()))) { module.clone() } else { punt_module.clone() };
    }
    punt_module = module.clone();
    continue;
    }
    if py_truthy(&_is_an_echo(&[last.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { _is_a_recap(&[last.clone(), geti(&turns.clone(), &sub(&index.clone(), &Py::Int(1)))]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_READABLE(), &module.clone()))) {
    if py_truthy(&{ let __b = punt_module.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&module.clone(), &py_str("answers"))) } }) {
    return punt_module.clone();
    }
    return module.clone();
    }
    return punt_module.clone();
    }
    return punt_module.clone();
    Py::None
}
pub fn _the_plan_can_hurt(args: &[Py]) -> Py {
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
    let mut act: Py = Py::None;
    let mut family: Py = Py::None;
    let mut guard: Py = Py::None;
    let _ = py_str("The family of harm the plan behind the check is in, or None.\n\n    An ordinary plan is untouched: this only answers a doubt when the line states one of the five\n    things that injure people, and each family needs both halves - the act and the thing that\n    makes it dangerous. \"stop taking my antidepressants\" is the medicine; \"stop taking the bins\n    out on tuesday\" is nothing.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NAMES_A_TRADE(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_CAN_HURT());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    family = __unpk2[0].clone();
    act = __unpk2[1].clone();
    guard = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_is(&mcall(&act.clone(), "search", &[said.clone()]), &Py::None))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&guard.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&guard.clone(), "search", &[said.clone()]), &Py::None)) } }) {
    continue;
    }
    return family.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_answer_already_warned(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let _ = py_str("The warning the answer above already gave, when it gave one, else \"\".\n\n    \"stop taking my antidepressants\" / \"is that ok\" follows an answer that says \"Keep taking it\n    until a pharmacist, nurse or GP tells you to stop\". The best reply is that sentence, not a\n    new one, so a warning the answer already carries is handed back rather than rewritten.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[said.clone()]))) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(6), &{ let __cmp20 = call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); __cmp20.clone() })) { le(&__cmp20.clone(), &Py::Int(40)) } else { Py::Bool(false) }))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_is(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[sentence.clone()]), &Py::None))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_is(&mcall(&g_A_KEEP_OR_STOP(), "search", &[sentence.clone()]), &Py::None))) {
    continue;
    }
    return add(&mcall(&sentence.clone(), "rstrip", &[py_str(" .")]), &py_str("."));
    }
    return py_str("");
    Py::None
}
pub fn _a_plan_doubt(args: &[Py]) -> Py {
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
    let _ = py_str("The line for a bare doubt that follows the person's own plan, or None.\n\n    A plan is theirs, so this composer has no verdict to give and stands aside - which used to\n    mean returning nothing, and the turn then landed on the acknowledgement pool: \"im going to\n    rome\" / \"is that ok\", \"i want to book a holiday\" / \"is that ok\" and \"we're thinking of getting\n    a rescue greyhound\" / \"is that ok\" were all answered \"Alright then - good luck with it\". A\n    sign-off says the conversation is over, and what the person is checking is that wanting it is\n    allowed. Nothing is ruled on about the plan; the ask is for the part being weighed.\n    ");
    said = _flat(&[_the_line_behind(&[memory.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PLAN(), "search", &[said.clone()]))) } }) {
    said = _the_plan_they_brought(&[memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[_round_robin(&[g_A_PLAN_DOUBT_LINES(), said.clone()]), memory.clone(), said.clone()]);
    Py::None
}
pub fn _about_the_person(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The line the ask follows, when it opens on the person: \"i have a lump\", \"ive a cough\".\n\n    A dying hedge or a hiring market is somebody else's business, and a verdict question about\n    one is not the person's own (\"is that bad\" after \"my hedge is dying\" is a question about the\n    hedge). What somebody has or noticed is theirs, and it may be a lump - which is why this is a\n    looser test than `_own_doing` and only ever feeds the pool that claims nothing.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_is_an_echo(&[last.clone()])) {
    continue;
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&stdlib_call("re", "match", &[py_str("(?:i\\s|ive\\s|i've\\s|i\\u2019ve\\s)"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _own_doing(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The line the verdict ask follows, when it is about something the person does themselves.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_is_an_echo(&[last.clone()])) {
    continue;
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = ge(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_OWN_DOING(), "match", &[said.clone()]), &Py::None)) } } } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_plan_or_a_choice(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line the doubt follows, when it is the person's own plan, choice or wish.\n\n    \"is that ok\" after a plan is not a symptom check and there is no answer to read it out of, so\n    without this the doubt path stands aside and the ack pool reads \"ok\" as a nod. Anything with a\n    body in it is somebody else's reading (\"should i get my mole checked\" has an answer in the\n    knowledge base and a verdict in it), so it is left alone here, as is a line about another\n    person.\n    ");
    said = _flat(&[_the_line_behind(&[memory.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_PLAN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DECISION_ASK(), "search", &[said.clone()]) } })) } }) {
    return Py::Bool(false);
    }
    words = _words(&[said.clone()]);
    return { let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BODY_WORD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SOMEONE_ELSE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn _good_news(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The line the verdict ask follows, when it is news that something went well.\n\n    \"i got the job\" / \"is that bad\" was met with \"How long has it been going on, and what would\n    you like to do about it?\", which reads as if the job were an illness. A floor that asks how\n    long it has been going on only belongs over a condition; news of a thing that went well is\n    not one, and it keeps whatever the composer had for it.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__reads_as_good_news(), &Py::None))) {
    return Py::Bool(false);
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_is_an_echo(&[last.clone()])) {
    continue;
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { py_call(&g__reads_as_good_news(), &[said.clone()]) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _an_event_that_is_over(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The line the verdict ask follows, when it is about something that happened and finished.\n\n    \"i finished my essay\" / \"is that bad\" was met with \"What would you like to do about it, and\n    how long has it been like that?\", which asks a person how long their essay has been going on.\n    A floor that asks how long is for a state, so a finished event keeps whatever the composer\n    had for it.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_is_an_echo(&[last.clone()])) {
    continue;
    }
    said = _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&g_STILL_GOING(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&mcall(&g_OVER_VERB(), "search", &[said.clone()]), &Py::None));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _nothing_to_be_long_about(args: &[Py]) -> Py {
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
    let _ = py_str("True when the floor's \"how long has it been going on\" would be asked of no such thing.\n\n    Two shapes: news that went well (\"i got the job\", read by the shared good-news reading) and\n    an event that has finished (\"i finished my essay\"). Neither is a condition, and both keep\n    what the composer had for them instead of being asked how long they have lasted.\n    ");
    return { let __b = _good_news(&[memory.clone()]); if py_truthy(&__b) { __b } else { _an_event_that_is_over(&[memory.clone()]) } };
    Py::None
}
pub fn _first(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut best: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the first of these words sits in the text, read case-blind.\n\n    A knowledge answer is written as sentences and starts them with a capital: \"See a GP if the\n    arm is weak or numb\" carried the referral that answers \"is that bad\", and a cue written\n    \"see a gp\" never saw it, so the arm got \"Alright then - good luck with it\".");
    text = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    best = Py::None;
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    at = mcall(&text.clone(), "find", &[word.clone()]);
    if py_truthy(&{ let __b = ge(&at.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&at.clone(), &best.clone()) } } } }) {
    best = at.clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _a_throw_away(args: &[Py]) -> Py {
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
    let mut action: Py = Py::None;
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The thing the answer told the person to get rid of, and whether it was written as a test.\n\n    \"If milk smells sour, bin it\" is not that: the answer is a rule for testing the milk, and a\n    bare \"is that bad\" on top of it is a question about the rule, not a report that the milk is\n    off, so the same fence `_unconditional_care` uses stands this reading down.\n    ");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_THROW_AWAY(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    before = slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&{ let __b = mcall(&g_NOT_TOLD_TO(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ONE_OPTION(), "search", &[before.clone()]) } }) {
    continue;
    }
    sentence = geti(&mcall(&before.clone(), "split", &[py_str(".")]), &neg(&Py::Int(1)));
    action = mcall(&match_.clone(), "group", &[Py::Int(0)]);
    action = add(&mcall(&geti(&action.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&action.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&{ let __b = mcall(&g_TESTING(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CONDITIONAL(), "search", &[geti(&mcall(&sentence.clone(), "split", &[py_str(",")]), &neg(&Py::Int(1)))]) } }) {
    return py_tuple(vec![action.clone(), Py::Bool(true)]);
    }
    return py_tuple(vec![action.clone(), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::None, Py::Bool(false)]);
    Py::None
}
pub fn _the_person_line(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The person's own last line, the one this doubt is a doubt about.");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&line.clone(), &said.clone())) } }) {
    return line.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _said_no(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own sentence saying the thing is not safe, or None.\n\n    Read sentence by sentence, because the no is often not the opening: \"There is no safe number\n    of drinks before driving. The England and Wales limit is 35 micrograms ...\" puts the verdict in\n    its first sentence and the numbers after it. The sentence is handed back as it was written, so\n    the reply repeats the answer rather than making a new claim about how serious the thing is.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_STATED_NO(), "search", &[sentence.clone()]), &Py::None))) {
    sentence = mcall(&mcall(&mcall(&sentence.clone(), "strip", &[]), "strip", &[py_str("\"'“”")]), "strip", &[]);
    if py_truthy(&sentence.clone()) {
    return add(&mcall(&sentence.clone(), "rstrip", &[py_str(".")]), &py_str("."));
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _said_enough(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own words saying the amount is enough, or None.\n\n    Cut at the first comma when the sentence runs long: \"Three half-hour sessions a week is enough\n    to feel different in a month, and you need no equipment: a walk or a jog for ten minutes to\n    warm up\" is one sentence, and handing the whole of it back repeats the answer the person is\n    looking at. The clause that says the amount is the half that answers them.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(py_is(&mcall(&g_STATED_ENOUGH(), "search", &[sentence.clone()]), &Py::None))) {
    continue;
    }
    sentence = mcall(&mcall(&mcall(&mcall(&sentence.clone(), "strip", &[]), "strip", &[py_str("\"'“”")]), "strip", &[]), "rstrip", &[py_str(".")]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(90)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&sentence.clone(), &py_str(","))) } }) {
    head = mcall(&geti(&mcall(&sentence.clone(), "split", &[py_str(",")]), &Py::Int(0)), "strip", &[]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_STATED_ENOUGH(), "search", &[head.clone()]), &Py::None)) } }) {
    sentence = head.clone();
    }
    }
    if py_truthy(&sentence.clone()) {
    return add(&sentence.clone(), &py_str("."));
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_person_gave_the_sign(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut line: Py = Py::None;
    let _ = py_str("Whether the person's own last line already said the sign the answer tests for.");
    line = _the_person_line(&[memory.clone(), said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_HOW_TO_TELL(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_SIGN(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _care_for_now(args: &[Py]) -> Py {
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
    let mut cue: Py = Py::None;
    let mut found: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the answer names care that is not fenced by a condition before it or after it.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    found = _first(&[sentence.clone(), g_CARE()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    continue;
    }
    cue = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CARE()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&mcall(&sentence.clone(), "lower", &[]), &word.clone()))) {  __out.push(mcall(&mcall(&sentence.clone(), "lower", &[]), "find", &[word.clone()])); } } py_list(__out) })() }); __kws.extend(vec![("default".to_string(), neg(&Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = ge(&cue.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_LATER_CONDITION(), "search", &[slice_or_raise(&sentence.clone(), &cue.clone(), &Py::None, &Py::None)]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_LATER_CONDITION(), "search", &[slice_or_raise(&sentence.clone(), &Py::None, &cue.clone(), &Py::None)]))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _sentence_at(args: &[Py]) -> Py {
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
    let mut begin: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The sentence of `text` that the offset `at` sits in.");
    begin = Py::Int(0);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("(?<=[.!?]) "), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&ge(&mcall(&found.clone(), "start", &[]), &at.clone())) {
    return slice_or_raise(&text.clone(), &begin.clone(), &mcall(&found.clone(), "start", &[]), &Py::None);
    }
    begin = mcall(&found.clone(), "end", &[]);
    }
    return slice_or_raise(&text.clone(), &begin.clone(), &Py::None, &Py::None);
    Py::None
}
pub fn _behind_a_description(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut cue: Py = Py::None;
    let mut described: Py = Py::None;
    let mut found: Py = Py::None;
    let mut marker: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("True when the care cue at `at` is the referral a described condition hangs on.\n\n    \"Shaking that has crept on over months ... is worth a GP appointment\" is the same advice for\n    later that an \"if\" makes it, written as a relative clause instead: the care word comes after\n    the condition rather than in front of it, so reading only backwards called it a verdict now.\n    ");
    sentence = _sentence_at(&[said.clone(), at.clone()]);
    if py_truthy(&{ let __b = le(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &sub(&sub(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &at.clone()), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&sentence.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])]))) } }) {
    sentence = add(&sentence.clone(), &py_str("."));
    }
    described = _a_described_condition(&[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&described.clone()))) {
    return Py::Bool(false);
    }
    marker = Py::None;
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("\\b(?:that|which|who)\\s+"), sentence.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    marker = found.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&marker.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&sentence.clone(), &Py::None, &mcall(&marker.clone(), "start", &[]), &Py::None), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    cue = if py_truthy(&lt(&at.clone(), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))) { mcall(&mcall(&sentence.clone(), "lower", &[]), "find", &[mcall(&slice_or_raise(&said.clone(), &at.clone(), &add(&at.clone(), &Py::Int(12)), &Py::None), "lower", &[])]) } else { neg(&Py::Int(1)) };
    return gt(&cue.clone(), &mcall(&marker.clone(), "start", &[]));
    Py::None
}
pub fn _not_fenced(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut fence: Py = Py::None;
    let mut rider: Py = Py::None;
    let _ = py_str("True when no \"if ...\" sits either side of this cue inside its own sentence.\n\n    A condition can come after the cue as well as before it: \"it is worth getting it checked if\n    it hurts or gives way\" and \"see a GP if the arm is weak or numb\" are both advice for a\n    condition, and reading only the words in front of the cue made them a verdict now.\n    ");
    before = slice_or_raise(&said.clone(), &call_builtin("max", &[Py::Int(0), sub(&start.clone(), &Py::Int(90))]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None);
    if py_truthy(&mcall(&g_CONDITIONAL(), "search", &[geti(&mcall(&before.clone(), "split", &[py_str(".")]), &neg(&Py::Int(1)))])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_is(&end.clone(), &Py::None))) {
    after = geti(&mcall(&slice_or_raise(&said.clone(), &end.clone(), &add(&end.clone(), &Py::Int(90)), &Py::None), "split", &[py_str(".")]), &Py::Int(0));
    fence = mcall(&g_CONDITIONAL_AFTER(), "search", &[after.clone()]);
    if py_truthy(&{ let __b = fence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_URGENT_TIME(), "search", &[slice_or_raise(&said.clone(), &start.clone(), &add(&end.clone(), &Py::Int(40)), &Py::None)]))) } }) {
    rider = slice_or_raise(&after.clone(), &Py::None, &mcall(&fence.clone(), "start", &[]), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_COMPARATIVE(), "search", &[rider.clone()])))) {
    return Py::Bool(false);
    }
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _said_without_a_not(args: &[Py]) -> Py {
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
    let mut length: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "length") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: length"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let _ = py_str("True when the word at `at` is said plainly, not under a \"not\", \"less\" or \"un-\".");
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None), &py_str("un")))) {
    return Py::Bool(false);
    }
    before = if py_truthy(&at.clone()) { geti(&text.clone(), &sub(&at.clone(), &Py::Int(1))) } else { py_str(" ") };
    after = slice_or_raise(&text.clone(), &add(&at.clone(), &length.clone()), &add(&add(&at.clone(), &length.clone()), &Py::Int(1)), &Py::None);
    if py_truthy(&{ let __b = mcall(&before.clone(), "isalnum", &[]); if py_truthy(&__b) { __b } else { mcall(&after.clone(), "isalnum", &[]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_is(&mcall(&g_NEGATED_BEFORE(), "search", &[slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(26))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None)]), &Py::None));
    Py::None
}
pub fn _first_positive(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut best: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("`_first`, but a word with a \"not\", \"less\" or \"un-\" in front of it is not that reading.\n\n    \"it is common at four and still common at nine, but past ten it is **less usual**\" says the\n    opposite of \"usual\" for the age in question, and \"not serious in most cases\" is not the same\n    answer as \"nothing serious\". Every worry cue is read this way, so the reassuring words cannot\n    be borrowed from a sentence that took them back.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    best = Py::None;
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    start = Py::Int(0);
    while py_truthy(&Py::Bool(true)) {
    at = mcall(&text.clone(), "find", &[word.clone(), start.clone()]);
    if py_truthy(&lt(&at.clone(), &Py::Int(0))) {
    break;
    }
    if py_truthy(&_said_without_a_not(&[text.clone(), at.clone(), call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&at.clone(), &best.clone()) } }) {
    best = at.clone();
    }
    break;
    }
    start = add(&at.clone(), &Py::Int(1));
    }
    }
    return best.clone();
    Py::None
}
pub fn _unconditional_care(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The first care cue that is not fenced off by an \"if ...\" or a description in front of it.\n\n    The words a referral is made in are not all in CARE: an answer offers \"get looked at\" and\n    \"Ring your GP or a physio\", and a physio, a dentist and a vet are the same kind of referral\n    as a doctor. REFER is read here too, so the referral counts however it is worded.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_CARE());
    for __item in __iter1 {
    word = __item.clone();
    start = mcall(&said.clone(), "find", &[word.clone()]);
    while py_truthy(&ge(&start.clone(), &Py::Int(0))) {
    if py_truthy(&{ let __b = _not_fenced(&[said.clone(), start.clone(), add(&start.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_behind_a_description(&[said.clone(), start.clone()]))) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&start.clone(), &best.clone()) } }) {
    best = start.clone();
    }
    break;
    }
    start = mcall(&said.clone(), "find", &[word.clone(), add(&start.clone(), &Py::Int(1))]);
    }
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_REFER(), "finditer", &[said.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_not_fenced(&[said.clone(), mcall(&match_.clone(), "start", &[]), mcall(&match_.clone(), "end", &[])])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&mcall(&match_.clone(), "start", &[]), &best.clone()) } }) {
    best = mcall(&match_.clone(), "start", &[]);
    }
    break;
    }
    return best.clone();
    Py::None
}
pub fn _the_fence_is_met(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fence"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the person's own line already says the words the answer's fence is written in.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    line = mcall(&{ let __b = _the_line_it_answered(&[said.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    line = if py_truthy(&turns.clone()) { mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), mcall(&call_builtin("str", &[{ let __b = fence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_frequency_already_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fence"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answered: Py = Py::None;
    let mut line: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the person has already said how often it happens, so the answer's condition is met.\n\n    `im getting headaches every afternoon` / `what should i do first` / `is that bad` is the person\n    the headache card fences: \"if you get them most days\" is their \"every afternoon\" in the card's\n    words. The doubt's own line says only \"is that bad\", so no words of the fence are in front of\n    it and reading the fence as the answer's condition told them the headache was ordinary on its\n    own, over their own line that it happens every afternoon. Found in review by @loyal-brush.\n\n    Both halves are needed: the fence has to be about how often, and the person has to have said\n    how often, so a chat whose answer fences on something else is untouched by a stray \"every day\"\n    about the bins.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_FREQUENCY_ASKED(), "search", &[call_builtin("str", &[{ let __b = fence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return Py::Bool(false);
    }
    answered = _the_line_it_answered(&[said.clone(), memory.clone()]);
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), answered.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_FREQUENCY_SAID(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    if py_truthy(&bitand(&words.clone(), &subject.clone())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _condition_after_care(args: &[Py]) -> Py {
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
    let mut care_at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "care_at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: care_at"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut fence: Py = Py::None;
    let mut later: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own condition, when it sits after the referral in that same sentence.\n\n    \"Get up in two stages, drink enough water, and see a doctor if you actually faint or it\n    happens most times you stand\" has a referral in it, so the bare \"is that bad\" after it was\n    answered \"Yes, that one wants a proper look rather than watching it\" - a claim the answer\n    itself had just contradicted, since the standing up is ordinary and the faint is the thing\n    to watch. A referral with the answer's own \"if\" after it is not a verdict now: it is the\n    condition the answer named, and that condition is the answer to the bare ask.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&care_at.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&care_at.clone(), &Py::Int(0)) } }) {
    return py_str("");
    }
    sentence = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), slice_or_raise(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &care_at.clone(), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    fence = stdlib_call("re", "search", &[py_str("\\b(?:if|unless|once|when) \\w"), sentence.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&fence.clone()))) {
    if py_truthy(&_the_call_sentence(&[said.clone()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&memory.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _past_the_timescale(&[said.clone(), memory.clone()]) } }) {
    return py_str("");
    }
    later = _when(&[said.clone()]);
    if py_truthy(&{ let __b = later.clone(); if !py_truthy(&__b) { __b } else { _a_frequency_already_said(&[later.clone(), said.clone(), memory.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = later.clone(); if !py_truthy(&__b) { __b } else { _the_fence_is_met(&[later.clone(), said.clone(), memory.clone()]) } }) {
    return py_str("");
    }
    return if py_truthy(&later.clone()) { _cut_condition(&[later.clone()]) } else { py_str("") };
    }
    if py_truthy(&mcall(&g_TIME_NOW(), "search", &[sentence.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_COMPARATIVE(), "search", &[slice_or_raise(&sentence.clone(), &Py::None, &mcall(&fence.clone(), "start", &[]), &Py::None)])) {
    return py_str("");
    }
    return _cut_condition(&[_when(&[said.clone()])]);
    Py::None
}
pub fn _cut_condition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut most: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "most") { v.clone() }
    else { Py::Int(22) };
    when = mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,.")]);
    if py_truthy(&le(&call_builtin("len", &[mcall(&when.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &most.clone())) {
    return when.clone();
    }
    when = mcall(&geti(&mcall(&g_CONDITION_CLAUSE(), "split", &[when.clone()]), &Py::Int(0)), "strip", &[py_str(" ,.")]);
    return when.clone();
    Py::None
}
pub fn _first_sentence_end(args: &[Py]) -> Py {
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
    let _ = py_str("Where the answer's first sentence ends, so a later one can be told apart from it.");
    found = stdlib_call("re", "search", &[py_str("[.!?](?:\\s|$)"), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&found.clone()) { mcall(&found.clone(), "end", &[]) } else { call_builtin("len", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) };
    Py::None
}
pub fn _told_to_act(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let _ = py_str("Whether the referral at `at` is the answer telling the person to do something.\n\n    The words a referral is made in come in two shapes. One is an instruction: \"ring the vet\",\n    \"get it looked at\", and the cue itself is the verb. The other only names the appointment:\n    \"worth a GP visit\", \"a job for the vet\". The second shape is how an answer writes its list\n    of warning signs - \"Pins and needles in the fingers that wake you is worth a GP visit\" - and\n    that is a pointer to a different symptom, not advice about the wrist that is \"usually an\n    overused tendon\". So it is the declarative shape that has to be in the answer's opening to\n    count, and a later one is a warning rather than a referral.\n    ");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_DECLARATIVE_REFERRAL(), "match", &[said.clone(), at.clone()])))) {
    return Py::Bool(true);
    }
    return le(&at.clone(), &_first_sentence_end(&[said.clone()]));
    Py::None
}
pub fn _referral(args: &[Py]) -> Py {
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
    let mut care: Py = Py::None;
    let mut stated: Py = Py::None;
    let _ = py_str("The answer's own referral, from its first sentence or from a later one that tells them to act.\n\n    The knee answer's referral is its last sentence, and reading only the opening answered the\n    knee with \"that one is the ordinary kind, so no\". A later sentence that only states some\n    OTHER symptom is worth a visit is a warning about that symptom, not advice about this one,\n    so a referral out of the first sentence has to be an instruction.\n    ");
    care = _unconditional_care(&[said.clone()]);
    if py_truthy(&Py::Bool(py_is(&care.clone(), &Py::None))) {
    stated = mcall(&g_DECLARATIVE_REFERRAL(), "search", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&stated.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _not_fenced(&[said.clone(), mcall(&stated.clone(), "start", &[]), mcall(&stated.clone(), "end", &[])]) } }) {
    return mcall(&stated.clone(), "start", &[]);
    }
    return Py::None;
    }
    if py_truthy(&le(&care.clone(), &_first_sentence_end(&[said.clone()]))) {
    return care.clone();
    }
    return if py_truthy(&_told_to_act(&[said.clone(), care.clone()])) { care.clone() } else { Py::None };
    Py::None
}
pub fn _a_plain_go(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut at: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut window: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the care cue at `at` is a recommendation, not a warning behind a condition.\n\n    The answer that says both halves - \"It is common at four and still common at nine, but past\n    ten it is less usual ... so it is worth a GP visit whatever the age\" - was read as the\n    ordinary kind because \"common\" comes first, and a twelve-year-old still wetting the bed was\n    told \"Nothing in that sounded like a warning sign\". A \"worth a GP visit\" is a thing to do\n    whatever the age, so it answers the ask rather than the words in front of it.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    window = slice_or_raise(&said.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(24))]).unwrap_or_else(|e| exc::raise(e)), &add(&at.clone(), &Py::Int(24)), &Py::None);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GO_FROM()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&window.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _opening(args: &[Py]) -> Py {
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
    let mut parts: Py = Py::None;
    let _ = py_str("The answer's first sentence or two, where its verdict and its first instruction live.\n\n    A word searched for anywhere finds the wrong sentence as often as the right one: the gas\n    bill answer carries a \"usually\" about tariffs three sentences in, and reading that as the\n    verdict turned \"is that bad\" after it into \"that one is the ordinary kind, so no\".\n    ");
    parts = stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return mcall(&py_str(" "), "join", &[slice_or_raise(&parts.clone(), &Py::None, &Py::Int(2), &Py::None)]);
    Py::None
}
pub fn _the_thing(args: &[Py]) -> Py {
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
    let _ = py_str("The words in the person's own line that name what they are asking about.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), _flat(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_THING_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _referral_about_the_thing(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut sentence: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("True when a sentence names a clinician, plainly, about the thing the person asked about.");
    thing = _the_thing(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[sentence.clone()])))) {
    continue;
    }
    if py_truthy(&mcall(&g_FENCED_IN_SENTENCE(), "search", &[sentence.clone()])) {
    continue;
    }
    if py_truthy(&bitand(&thing.clone(), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), mcall(&sentence.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _echoes_the_person(args: &[Py]) -> Py {
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
    let mut have: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("Whether the \"step\" is only the person's own words said back to them.\n\n    \"i want to be more organised\" was met with \"Be more organised - fair enough. What's stopped\n    it happening so far?\", and the first clause of that is the wish itself: answering \"what\n    should i do first\" with \"Be more organised, and take it from there\" tells nobody anything.\n    A short step whose every content word is in the line the person just wrote is that echo.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    said = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    said = call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let mut content: Py = py_func("content", |args: &[Py]| crate::gen::memory::topics::content(args));
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("set", &[py_call(&content, &[step.clone()])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[py_call(&content, &[said.clone()])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    want = __unpk2[0].clone();
    have = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&want.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&sub(&want.clone(), &have.clone())));
    Py::None
}
pub fn _kept_step(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut plain: Py = Py::None;
    let _ = py_str("True when `reply` is one of the lines that says the step has not changed, else False.");
    plain = mcall(&_flat(&[reply.clone()]), "strip", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STEP_KEPT_LINES()) { let mut line: Py = __item.clone(); __out.push(stdlib_call("re", "fullmatch", &[_own_pattern(&[line.clone()]), plain.clone(), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _only_a_verdict(args: &[Py]) -> Py {
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
    let _ = py_str("Whether a line the person sent was nothing but their verdict on the news above it.");
    text = mcall(&_flat(&[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let mut reads_as_a_verdict: Py = py_func("reads_as_a_verdict", |args: &[Py]| crate::gen::compose::fallback::reads_as_a_verdict(args));
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
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&reads_as_a_verdict, &[text.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn _the_line_just_given(args: &[Py]) -> Py {
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
    let _ = py_str("The bot's own last line, the one the person has just read. \"\" when there is not one.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    return call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_answer_before_that(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's line before its last one, when the last one was only a question.\n\n    A question the bot asked is not an answer: there is no step in it to hand over, and the answer\n    the person is asking about is the one before it. \"\" when there is not one.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    let __aug2 = add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&Py::Bool(py_eq(&seen.clone(), &Py::Int(2)))) {
    return call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _word_runs(args: &[Py]) -> Py {
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
    let mut size: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "size") { v.clone() }
    else { Py::Int(5) };
    let mut at: Py = Py::None;
    let mut words: Py = Py::None;
    words = mcall(&_flat(&[text.clone()]), "split", &[]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("max", &[Py::Int(0), add(&sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut at: Py = __item.clone(); __out.push(call_builtin("tuple", &[slice_or_raise(&words.clone(), &at.clone(), &add(&at.clone(), &size.clone()), &Py::None)]).unwrap_or_else(|e| exc::raise(e))); } py_set(__out) })() };
    Py::None
}
pub fn _says_it_again(args: &[Py]) -> Py {
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
    let _ = py_str("Whether saying `line` now would hand the person a sentence they have just read.\n\n    A five-word run in common is the whole sentence in the cases that matter (\"Keep it plain and\n    separate: one new food next to one they already eat\"), and it cannot be a coincidence: the\n    composer builds its steps out of the answer's own words, so a run that long is the answer's\n    words coming back. Anything shorter is shared phrasing and is allowed.\n    ");
    return call_builtin("bool", &[bitand(&_word_runs(&[line.clone()]), &_word_runs(&[said.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _already_asked_for_a_step(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the person's own last line was a bare step ask too.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    return call_builtin("bool", &[mcall(&g_STEP(), "match", &[_flat(&[geti(&turn.clone(), &py_str("user"))])])]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_referral_sentence(args: &[Py]) -> Py {
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
    let mut start: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The one sentence of the answer that holds its judgement, whole.");
    text = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    at = _first(&[mcall(&text.clone(), "lower", &[]), g_CARE()]);
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    at = Py::Int(0);
    }
    start = call_builtin("max", &[mcall(&text.clone(), "rfind", &[py_str(". "), Py::Int(0), at.clone()]), mcall(&text.clone(), "rfind", &[py_str("? "), Py::Int(0), at.clone()]), mcall(&text.clone(), "rfind", &[py_str("! "), Py::Int(0), at.clone()])]).unwrap_or_else(|e| exc::raise(e));
    return geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), mcall(&slice_or_raise(&text.clone(), &add(&start.clone(), &Py::Int(1)), &Py::None, &Py::None), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    Py::None
}
pub fn _the_line_is_the_threshold(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut stated: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stated") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stated"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut have: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("Whether the person's own line names the thing the answer set as its threshold.\n\n    \"A dog that will not eat for a day is worth a call to the vet\" over \"my dog wont eat\" is the\n    answer's own yes: the threshold is what they are asking about, so handing it back as \"Not yet.\n    The point where it wants a person: a dog that will not eat for a day\" argues with the answer.\n    Two content words in common is the test, because the two are written by different people\n    (\"wont eat\" against \"will not eat\") and the overlap is what a reader sees.\n    ");
    let mut content: Py = py_func("content", |args: &[Py]| crate::gen::memory::topics::content(args));
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("set", &[py_call(&content, &[stated.clone()])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[py_call(&content, &[line.clone()])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    want = __unpk1[0].clone();
    have = __unpk1[1].clone();
    return ge(&call_builtin("len", &[bitand(&want.clone(), &have.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _most_of_the_line(args: &[Py]) -> Py {
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
    let mut heard: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "heard") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: heard"))) };
    let mut have: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("Whether the step is most of what the bot said last turn, so quoting it repeats it.\n\n    \"Long one? Give yourself half an hour doing nothing before the next thing.\" is a whole\n    answer that is one instruction, and handing it back as the step said the person's last two\n    lines over again. A clause out of a longer answer is not that: the rest of the answer was\n    there, so the step adds to what they read rather than replacing it.\n    ");
    let mut content: Py = py_func("content", |args: &[Py]| crate::gen::memory::topics::content(args));
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("set", &[py_call(&content, &[step.clone()])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[py_call(&content, &[heard.clone()])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    want = __unpk1[0].clone();
    have = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&want.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&have.clone())) } }) {
    return Py::Bool(false);
    }
    return ge(&truediv(&call_builtin("len", &[bitand(&want.clone(), &have.clone())]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Float(0.8));
    Py::None
}
pub fn _step(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let _ = py_str("The first thing the answer told them to do, in the answer's own words.");
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_steps(&[said.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NEGATIVE_OPEN(), "match", &[one.clone()])))) {  __out.push(one.clone()); } } py_list(__out) })() };
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_STATEMENT(), "match", &[one.clone()])))) {  __out.push(one.clone()); } } py_list(__out) })() };
    return if py_truthy(&found.clone()) { geti(&found.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _listed(args: &[Py]) -> Py {
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
    let mut colon: Py = Py::None;
    let mut head: Py = Py::None;
    let mut item: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first thing on a list the answer wrote after a colon, in the answer's own words.\n\n    \"The things that work: same wake-up time every day, no caffeine after early afternoon\" has\n    given a move without writing one as an instruction, and a person who asks for tips wants\n    that first one, not a request to say what they have tried. Only a colon list, and only its\n    first item, so nothing here reads a sentence that was not written as a list.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&mcall(&g_CONDITIONAL_OPEN(), "match", &[mcall(&sentence.clone(), "strip", &[])])) {
    continue;
    }
    let __unpk2 = py_unpack(&mcall(&sentence.clone(), "partition", &[py_str(": ")]), 3, None);
    head = __unpk2[0].clone();
    colon = __unpk2[1].clone();
    tail = __unpk2[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&colon.clone())); if py_truthy(&__b) { __b } else { lt(&mcall(&tail.clone(), "count", &[py_str(",")]), &Py::Int(2)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_HEAD_REFUSES(), "search", &[head.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HELPS(), "search", &[head.clone()]))) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",| and "), tail.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    item = __item.clone();
    item = mcall(&py_str(" "), "join", &[mcall(&mcall(&item.clone(), "strip", &[py_str(" .:;,-")]), "split", &[])]);
    words = mcall(&item.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(3), &{ let __cmp40 = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp40.clone() })) { le(&__cmp40.clone(), &g_STEP_WORDS()) } else { Py::Bool(false) }))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_NEGATIVE_OPEN(), "match", &[item.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CONDITIONAL_OPEN(), "match", &[item.clone()]) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_DESCRIBES(), "search", &[item.clone()])) {
    continue;
    }
    return item.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _cut_on(args: &[Py]) -> Py {
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
    let mut joins: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "joins") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: joins"))) };
    let mut at: Py = Py::None;
    let mut best: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut cut_words: Py = Py::None;
    let mut join: Py = Py::None;
    let _ = py_str("The longest cut of `head` before one of these joins that is still a step, or \"\".");
    best = py_str("");
    let __iter1 = py_iter_or_raise(&joins.clone());
    for __item in __iter1 {
    join = __item.clone();
    at = mcall(&head.clone(), "find", &[join.clone()]);
    while py_truthy(&gt(&at.clone(), &Py::Int(0))) {
    cut = mcall(&slice_or_raise(&head.clone(), &Py::None, &at.clone(), &Py::None), "strip", &[py_str(" ,;:-")]);
    cut_words = mcall(&cut.clone(), "split", &[]);
    if py_truthy(&{ let __b = if py_truthy(&le(&Py::Int(4), &{ let __cmp20 = call_builtin("len", &[cut_words.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp20.clone() })) { le(&__cmp20.clone(), &g_STEP_WORDS()) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_HANGING(), &mcall(&geti(&cut_words.clone(), &neg(&Py::Int(1))), "lower", &[]))); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[cut.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    best = cut.clone();
    }
    at = mcall(&head.clone(), "find", &[join.clone(), add(&at.clone(), &Py::Int(1))]);
    }
    }
    return best.clone();
    Py::None
}
pub fn _shorten(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut best: Py = Py::None;
    let _ = py_str("A step of more than STEP_WORDS words, cut on a joining word, or \"\" if it cannot be cut.");
    if py_truthy(&le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    return head.clone();
    }
    best = _cut_on(&[head.clone(), g_TRIM()]);
    if py_truthy(&best.clone()) {
    return best.clone();
    }
    if py_truthy(&{ let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_STEP_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_HANGING(), &mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "lower", &[]))) } }) {
    return head.clone();
    }
    return _cut_on(&[head.clone(), g_TRIM_INSTRUMENT()]);
    Py::None
}
pub fn _cut_at_a_join(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut keep_until: Py = Py::None;
    let _ = py_str("The part up to the first \" and \" or \" then \" that is not inside a between/both frame.");
    keep_until = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g_FRAME_AND(), "finditer", &[part.clone()]));
    for __item in __iter1 {
    frame = __item.clone();
    if py_truthy(&gt(&mcall(&frame.clone(), "end", &[]), &keep_until.clone())) {
    keep_until = mcall(&frame.clone(), "end", &[]);
    }
    }
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str(" and | then "), part.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    at = __item.clone();
    if py_truthy(&lt(&mcall(&at.clone(), "start", &[]), &keep_until.clone())) {
    continue;
    }
    return slice_or_raise(&part.clone(), &Py::None, &mcall(&at.clone(), "start", &[]), &Py::None);
    }
    return part.clone();
    Py::None
}
pub fn _first_clause(args: &[Py]) -> Py {
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
    let mut and_break: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "and_break") { v.clone() }
    else { Py::Bool(true) };
    let mut at: Py = Py::None;
    let mut char: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The part of an instruction up to where it stops being one instruction.\n\n    The break is the first comma that is followed by a new instruction (\"Put the child's name and\n    class in the subject, say what you have noticed\"), not the first \" and \": cutting at \" and \"\n    quoted \"Put the child's name\", a noun phrase, and the step templates then mangled it into\n    \"Put the child's name, before anything else on that list.\" A comma that hands the same\n    instruction on (\"stop driving and get them changed, or you will score the discs too\") is not\n    a break, and with none of either the whole part is the clause.\n    ");
    let __iter1 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5207 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk5207[0].clone(); let mut char: Py = __unpk5207[1].clone(); if py_truthy(&Py::Bool(py_eq(&char.clone(), &py_str(",")))) {  __out.push(index.clone()); } } py_list(__out) })() });
    for __item in __iter1 {
    at = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&and_break.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_ORDER_CONTINUES(), "match", &[slice_or_raise(&part.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_CONTINUES(), "match", &[slice_or_raise(&part.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)])))) {
    return slice_or_raise(&part.clone(), &Py::None, &at.clone(), &Py::None);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&and_break.clone()))) {
    return part.clone();
    }
    return _cut_at_a_join(&[part.clone()]);
    Py::None
}
pub fn _an_offer_is_waiting(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether the bot's own last question was an offer the person may be taking up.");
    text = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "endswith", &[py_str("?")])))) {
    return Py::Bool(false);
    }
    last = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), text.clone()]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(1)));
    return call_builtin("bool", &[mcall(&g_OFFER_QUESTION(), "search", &[last.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _after_a_comfort_line(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the bot's own last line was one of its comfort lines.\n\n    A step asked for after a comfort line already has an owner: `step_when_low`, in the module\n    that said it, answers with the first step of a bad day (\"something to eat, a shower, ten\n    minutes outside\"). The subject of that thread is how they feel, so naming the bot's own\n    question back at them (\"is there one thing you can check?\") is the colder reply and would\n    take the turn off the module that knows what to say. The sets live with the modules that\n    speak them, so they are read here rather than imported at the top of this file.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::Bool(false);
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let mut COMFORT_SAID: Py = crate::gen::compose::fallback::g_COMFORT_SAID();
    let mut _chat: Py = crate::gen::modules::chat::module_obj();
    lines = add(&call_builtin("tuple", &[COMFORT_SAID.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[call_builtin("getattr", &[_chat.clone(), py_str("FEELING_SAID"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push({ let __b = line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &mcall(&line.clone(), "lower", &[]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_question_asked(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut front: Py = Py::None;
    let mut last: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut split: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The bot's own last question, as the thing the person can answer, or \"\" if it is too long.");
    text = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "endswith", &[py_str("?")])))) {
    return py_str("");
    }
    last = mcall(&geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), text.clone()]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(1))), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&_is_a_question(&[last.clone()])))) {
    return py_str("");
    }
    asked = mcall(&slice_or_raise(&last.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), "strip", &[py_str(" ,;:-")]);
    split = mcall(&g__A_DASH(), "match", &[asked.clone()]);
    if py_truthy(&split.clone()) {
    front = mcall(&mcall(&split.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ,;:-")]);
    rest = mcall(&mcall(&split.clone(), "group", &[Py::Int(2)]), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = mcall(&g__AN_OPENER(), "match", &[front.clone()]); if !py_truthy(&__b) { __b } else { _is_a_question(&[add(&rest.clone(), &py_str("?"))]) } }) {
    front = rest.clone();
    }
    asked = front.clone();
    }
    if py_truthy(&mcall(&g__INVITATION(), "match", &[asked.clone()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&asked.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return py_str("");
    }
    words = mcall(&asked.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("I"), py_str("I'm"), py_str("I've"), py_str("I'd"), py_str("I'll")]), &geti(&words.clone(), &Py::Int(0))))) {
    let _ = py_setitem(&words.clone(), &Py::Int(0), add(&mcall(&slice_or_raise(&geti(&words.clone(), &Py::Int(0)), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&geti(&words.clone(), &Py::Int(0)), &Py::Int(1), &Py::None, &Py::None)));
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _ends_on_a_question(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether the bot's own last line finished on a question put to the person.");
    text = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "endswith", &[py_str("?")])))) {
    return Py::Bool(false);
    }
    last = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), text.clone()]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(1)));
    return call_builtin("bool", &[{ let __b = mcall(&g_QUESTION_FRAME(), "match", &[last.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_QUESTION_FRAME(), "match", &[geti(&mcall(&last.clone(), "rsplit", &[py_str(", "), Py::Int(1)]), &neg(&Py::Int(1)))]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _is_a_question(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("Whether this sentence of a bot line is a question put to the person.");
    text = mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "endswith", &[py_str("?")])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_QUESTION_FRAME(), "match", &[text.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_QUESTION_FRAME(), "match", &[geti(&mcall(&text.clone(), "rsplit", &[py_str(", "), Py::Int(1)]), &neg(&Py::Int(1)))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _steps(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut at: Py = Py::None;
    let mut at_sentence: Py = Py::None;
    let mut chunk: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut opened: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut protected: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sep: Py = Py::None;
    let mut spread: Py = Py::None;
    let mut text: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every step the answer gave, in the answer's own words, in the order it gave them.");
    let mut RED_FLAG: Py = crate::gen::memory::topics::g_RED_FLAG();
    found = py_list(vec![]);
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at_sentence = __unpk2[0].clone();
    sentence = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&g_CONDITIONAL_OPEN(), "match", &[mcall(&sentence.clone(), "strip", &[])]); if py_truthy(&__b) { __b } else { { let __b = mcall(&RED_FLAG.clone(), "match", &[mcall(&sentence.clone(), "strip", &[])]); if py_truthy(&__b) { __b } else { { let __b = _is_a_question(&[sentence.clone()]); if py_truthy(&__b) { __b } else { _is_an_explanation(&[sentence.clone()]) } } } } } }) {
    continue;
    }
    opened = if py_truthy(&Py::Bool(!py_truthy(&at_sentence.clone()))) { mcall(&g_THEN_OPEN(), "sub", &[py_str(""), mcall(&sentence.clone(), "strip", &[])]) } else { sentence.clone() };
    if py_truthy(&Py::Bool(!py_is(&opened.clone(), &sentence.clone()))) {
    sentence = add(&mcall(&slice_or_raise(&opened.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&opened.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&Py::Bool(!py_truthy(&sentence.clone()))) {
    continue;
    }
    }
    raw = mcall(&g_SPLIT(), "split", &[sentence.clone()]);
    kept = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&call_builtin("enumerate", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    at = __unpk4[0].clone();
    chunk = __unpk4[1].clone();
    if py_truthy(&{ let __b = modulo(&at.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&chunk.clone(), "strip", &[]))) } }) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[py_tuple(vec![mcall(&chunk.clone(), "strip", &[]), if py_truthy(&kept.clone()) { geti(&raw.clone(), &sub(&at.clone(), &Py::Int(1))) } else { py_str("") }])]);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_CONTRAST(), "search", &[geti(&geti(&kept.clone(), &Py::Int(0)), &Py::Int(0))]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_IMPERATIVES(), &_doing_word(&[mcall(&geti(&geti(&kept.clone(), &Py::Int(1)), &Py::Int(0)), "split", &[])]))) } } } }) {
    kept = slice_or_raise(&kept.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_IMPERATIVES(), &_doing_word(&[mcall(&geti(&geti(&kept.clone(), &Py::Int(0)), &Py::Int(0)), "split", &[])]))) } }) {
    kept = slice_or_raise(&kept.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&kept.clone(), &Py::Int(1), &Py::None, &Py::None)) { let __unpk401 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk401[0].clone(); let mut sep: Py = __unpk401[1].clone(); __out.push(Py::Bool(py_eq(&sep.clone(), &py_str(": ")))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&parts.clone(), "append", &[py_tuple(vec![mcall(&py_str(": "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let __unpk9392 = py_unpack(&__item.clone(), 2, None); let mut text: Py = __unpk9392[0].clone(); let mut _u: Py = __unpk9392[1].clone(); __out.push(text.clone()); } py_list(__out) })() }]), Py::Bool(true)])]);
    }
    let _ = mcall(&parts.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let __unpk8207 = py_unpack(&__item.clone(), 2, None); let mut text: Py = __unpk8207[0].clone(); let mut _u: Py = __unpk8207[1].clone(); __out.push(py_tuple(vec![text.clone(), Py::Bool(false)])); } py_list(__out) })() }]);
    }
    let __iter5 = py_iter_or_raise(&parts.clone());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    part = __unpk6[0].clone();
    spread = __unpk6[1].clone();
    if py_truthy(&mcall(&RED_FLAG.clone(), "match", &[part.clone()])) {
    continue;
    }
    part = stdlib_call("re", "sub", &[py_str("\\([^)]*\\)|\\S*\\.(?:com|co\\.uk|org|net|gov)\\S*"), py_str(" "), part.clone()]).unwrap_or_else(|e| exc::raise(e));
    part = mcall(&py_str(" "), "join", &[mcall(&mcall(&part.clone(), "strip", &[py_str(" .:;,-")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    continue;
    }
    words = mcall(&part.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_IMPERATIVES(), &_doing_word(&[words.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_COPULA(), "match", &[part.clone()]) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_TARGET_SHAPE(), "match", &[part.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g_ABOUT_THE_TALK(), "search", &[part.clone()])) {
    continue;
    }
    protected = stdlib_call("re", "sub", &[py_str("(?<=\\d) and (?=[\\d.])"), py_str(" \0 "), part.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&spread.clone()) {
    let __unpk7 = py_unpack(&mcall(&protected.clone(), "partition", &[py_str(": ")]), 3, None);
    opening = __unpk7[0].clone();
    _u = __unpk7[1].clone();
    detail = __unpk7[2].clone();
    opening = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(opening.clone()); __kws.extend(vec![("and_break".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _first_clause(args))(&__args) }, "replace", &[py_str("\0"), py_str("and")]);
    opening = mcall(&opening.clone(), "strip", &[py_str(" .:;,-")]);
    detail = mcall(&mcall(&detail.clone(), "replace", &[py_str("\0"), py_str("and")]), "strip", &[py_str(" .:;,-")]);
    if py_truthy(&mcall(&g_DETAIL_STATEMENT(), "search", &[detail.clone()])) {
    detail = py_str("");
    } else {
    detail = mcall(&_first_clause(&[detail.clone()]), "strip", &[py_str(" .:;,-")]);
    }
    head = if py_truthy(&detail.clone()) { modulo(&py_str("%s: %s"), &py_tuple(vec![opening.clone(), detail.clone()])) } else { opening.clone() };
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&detail.clone(), &py_str(","))) } }) {
    detail = mcall(&geti(&mcall(&detail.clone(), "split", &[py_str(",")]), &Py::Int(0)), "strip", &[py_str(" .:;,-")]);
    head = if py_truthy(&detail.clone()) { modulo(&py_str("%s: %s"), &py_tuple(vec![opening.clone(), detail.clone()])) } else { opening.clone() };
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    head = { let __b = _shorten(&[opening.clone(), mcall(&opening.clone(), "split", &[])]); if py_truthy(&__b) { __b } else { opening.clone() } };
    }
    } else {
    clause = mcall(&mcall(&_first_clause(&[protected.clone()]), "replace", &[py_str("\0"), py_str("and")]), "strip", &[py_str(" .:;,-")]);
    head = if py_truthy(&ge(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) { _shorten(&[clause.clone(), mcall(&clause.clone(), "split", &[])]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    trimmed = geti(&stdlib_call("re", "split", &[py_str(",? then "), part.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    head = if py_truthy(&ge(&call_builtin("len", &[mcall(&trimmed.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { _shorten(&[trimmed.clone(), mcall(&trimmed.clone(), "split", &[])]) } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    head = _shorten(&[part.clone(), words.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return found.clone();
    }
    words = mcall(&head.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_HANGING(), &geti(&words.clone(), &neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&found.clone(), &head.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_IMPERATIVES(), &_doing_word(&[words.clone()])))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[head.clone()]);
    }
    return found.clone();
    Py::None
}
pub fn _doing_word(args: &[Py]) -> Py {
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
    let mut core: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The first word that is not one of the when words in front of a step.");
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str("*_`(\"'[]")]), "lower", &[]))) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(5), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    core = mcall(&mcall(&word.clone(), "strip", &[py_str(",;:-—–")]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_WHEN(), &core.clone()))) {
    continue;
    }
    return core.clone();
    }
    return py_str("");
    Py::None
}
pub fn _action_count(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut s: Py = Py::None;
    let mut total: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How many things the answer told them to do, so a \"list\" is only promised when there is\n    one. A clause counts when it opens with a verb, which is how the answers write a step\n    (\"put a bowl under it\", \"ease off hills\", \"take paracetamol\").");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut s: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_is_a_question(&[s.clone()])))) {  __out.push(s.clone()); } } py_list(__out) })() }]);
    total = Py::Int(0);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",| and | then |\\. |; |: | - | — | – "), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    part = __item.clone();
    words = mcall(&mcall(&part.clone(), "strip", &[py_str(" .:;,-()")]), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_IMPERATIVES(), &_doing_word(&[words.clone()]))) } }) {
    let __aug2 = add(&total.clone(), &Py::Int(1));
    total = __aug2;
    }
    }
    return total.clone();
    Py::None
}
pub fn _no_hanging_end(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut shorter: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The condition with any trailing join, comma or half-list taken off its end.");
    text = mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,;:")]);
    while py_truthy(&Py::Bool(true)) {
    shorter = mcall(&stdlib_call("re", "sub", &[py_str("(?:^|[ ,]+)(?:and|or|but|then|so|also|plus)$"), py_str(""), text.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,;:")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shorter.clone(), &text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&shorter.clone())) } }) {
    break;
    }
    text = shorter.clone();
    }
    return text.clone();
    Py::None
}
pub fn _described_clause(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut marker: Py = Py::None;
    let mut referral: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut start: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The condition a sentence writes as a description: (from, to, its words), or None.\n\n    \"Shaking that has crept on over months, is much worse on one side ... is worth a GP\n    appointment\" hangs its referral on the clause that follows \"that\", and so does \"one that is\n    also dull or sick needs to be seen today\". Where the clause sits is what tells a referral\n    inside it from one beside it: the dog answer has \"worth a call to the vet\" flat, and a\n    condition next to it, and only the second is a warning for later.\n    ");
    referral = mcall(&g_REFER(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(py_is(&referral.clone(), &Py::None))) {
    return Py::None;
    }
    head = slice_or_raise(&sentence.clone(), &Py::None, &mcall(&referral.clone(), "start", &[]), &Py::None);
    marker = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_RELATIVE(), "finditer", &[head.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    marker = found.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&marker.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&head.clone(), &Py::None, &mcall(&marker.clone(), "start", &[]), &Py::None), "strip", &[]))) } }) {
    return Py::None;
    }
    start = mcall(&marker.clone(), "end", &[]);
    clause = slice_or_raise(&head.clone(), &start.clone(), &Py::None, &Py::None);
    cut = Py::None;
    let __iter2 = py_iter_or_raise(&mcall(&g_REFER_VERB(), "finditer", &[clause.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    rest = mcall(&slice_or_raise(&clause.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,")]);
    if py_truthy(&{ let __b = le(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&rest.clone(), &py_str(","))) } }) {
    cut = mcall(&found.clone(), "start", &[]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&cut.clone(), &Py::None))) {
    return Py::None;
    }
    words = mcall(&mcall(&slice_or_raise(&clause.clone(), &Py::None, &cut.clone(), &Py::None), "strip", &[py_str(" ,;:-")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp30 = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp30.clone() })) { le(&__cmp30.clone(), &Py::Int(26)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_CONDITION_HANGING(), &mcall(&mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "strip", &[py_str(",;:")]), "lower", &[])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_CONDITION_VERBS(), &mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(",;:")]), "lower", &[])))) {
    return Py::None;
    }
    return py_tuple(vec![start.clone(), mcall(&referral.clone(), "start", &[]), mcall(&py_str(" "), "join", &[words.clone()])]);
    Py::None
}
pub fn _in_a_described_clause(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut found: Py = Py::None;
    let _ = py_str("True when position `at` in the sentence is inside the condition it describes.");
    found = _described_clause(&[sentence.clone()]);
    return { let __b = Py::Bool(!py_is(&found.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { if py_truthy(&le(&geti(&found.clone(), &Py::Int(0)), &{ let __cmp10 = at.clone(); __cmp10.clone() })) { lt(&__cmp10.clone(), &geti(&found.clone(), &Py::Int(1))) } else { Py::Bool(false) } } };
    Py::None
}
pub fn _a_described_condition(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The relative clause a sentence hangs its referral on, or \"\" when it hangs on an \"if\".");
    found = _described_clause(&[sentence.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return py_str("");
    }
    return add(&py_str("it "), &geti(&found.clone(), &Py::Int(2)));
    Py::None
}
pub fn _an_if_that_is_a_condition(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("Where the sentence's \" if \" puts a condition on the referral, or -1.\n\n    \"do not wait to see if it passes\" is a test the person runs, not a condition for care, and\n    reading it as one took the verdict off the pale-gums answer: \"Ring the vet now and go in\n    today ... do not wait to see if it passes\" came back as \"Ordinary on its own, yes. It stops\n    being ordinary if it passes\" - an all-clear over an answer that opens \"Pale, white or blue\n    gums are an emergency\" (main answers it with the yes pool, and this keeps that). The word in\n    front of the \"if\" is what tells the two apart.\n    ");
    start = Py::Int(0);
    while py_truthy(&Py::Bool(true)) {
    at = mcall(&sentence.clone(), "find", &[py_str(" if "), start.clone()]);
    if py_truthy(&lt(&at.clone(), &Py::Int(0))) {
    return neg(&Py::Int(1));
    }
    before = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&slice_or_raise(&sentence.clone(), &Py::None, &at.clone(), &Py::None), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LOOK_AND_SEE(), &geti(&before.clone(), &neg(&Py::Int(1))))) } }) {
    return at.clone();
    }
    start = add(&at.clone(), &Py::Int(4));
    }
    Py::None
}
pub fn _a_judgement_fence(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut s: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The condition a closing, declarative referral puts on itself, or \"\".\n\n    Only the last sentence is read, and only when it names a clinician and carries a list in\n    front of the referral (\"Anything hard, painless, fixed in place, or still there after three\n    weeks needs a GP\"). Narrow on purpose: `REFER` and `_when` are left exactly as they are, so\n    no step or how-to answer changes.\n    ");
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), mcall(&said.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut s: Py = __item.clone(); if py_truthy(&mcall(&s.clone(), "strip", &[])) {  __out.push(s.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&sentences.clone()))) {
    return py_str("");
    }
    last = geti(&sentences.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_is(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[last.clone()]), &Py::None))) {
    return py_str("");
    }
    found = stdlib_call("re", "search", &[py_str("(?i)\\b(?:anything|any [a-z-]+)\\b(.*?)\\b(?:needs?|should be|is worth|wants)\\b"), last.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return py_str("");
    }
    clause = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&clause.clone(), &py_str(","))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&clause.clone(), &py_str(" or "))) } }) {
    return py_str("");
    }
    words = mcall(&clause.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp10 = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(26)) } else { Py::Bool(false) }))) {
    return py_str("");
    }
    return clause.clone();
    Py::None
}
pub fn _after_the_if(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut opens: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The condition a sentence writes after its \"if\", cut where the answer asks something.");
    opens = stdlib_call("re", "match", &[py_str("(?:so )?if "), sentence.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&opens.clone()) {
    when = slice_or_raise(&sentence.clone(), &mcall(&opens.clone(), "end", &[]), &Py::None, &Py::None);
    when = geti(&stdlib_call("re", "split", &[py_str("(?:^|, |; | and | then | so )(?:and )?(?:get|see|ring|call|take|go|book|treat|watch|keep|check|apply|cover|use|rest|ice|put|give|try|test|monitor|stop|unplug|switch|leave|turn|have|ask|phone|contact|speak|mention)\\b"), when.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    } else {
    at = mcall(&sentence.clone(), "find", &[py_str(" if ")]);
    if py_truthy(&lt(&at.clone(), &Py::Int(0))) {
    return py_str("");
    }
    when = mcall(&slice_or_raise(&sentence.clone(), &add(&at.clone(), &Py::Int(4)), &Py::None, &Py::None), "rstrip", &[py_str(" .")]);
    when = geti(&stdlib_call("re", "split", &[py_str("(?:^|, | and | then )(?:and )?(?:get|see|ring|call|take|go|book|treat|watch|keep|check|apply|cover|use|rest|ice|put|give|try|test|monitor|have|ask|phone|contact|speak|mention)\\b"), when.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    }
    when = geti(&mcall(&g_RESTATED(), "split", &[when.clone()]), &Py::Int(0));
    when = _no_hanging_end(&[mcall(&when.clone(), "strip", &[py_str(" ,.")])]);
    return if py_truthy(&ge(&call_builtin("len", &[mcall(&when.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { when.clone() } else { py_str("") };
    Py::None
}
pub fn _when(args: &[Py]) -> Py {
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
    let mut opens: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The condition the answer itself put on getting someone in.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_REFER(), "search", &[sentence.clone()])))) {
    continue;
    }
    opens = stdlib_call("re", "match", &[py_str("(?:so )?if "), sentence.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = opens.clone(); if py_truthy(&__b) { __b } else { ge(&_an_if_that_is_a_condition(&[sentence.clone()]), &Py::Int(0)) } }) {
    when = _after_the_if(&[sentence.clone()]);
    if py_truthy(&when.clone()) {
    return when.clone();
    }
    continue;
    }
    when = _a_described_condition(&[sentence.clone()]);
    if py_truthy(&when.clone()) {
    return when.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _stated_before_the_judgement(args: &[Py]) -> Py {
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
    let mut frame: Py = Py::None;
    let mut head: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The thing a sentence puts in front of its judgement (\"blood in it is worth a GP's time\").");
    frame = mcall(&g_STATES_A_REFERRAL(), "match", &[sentence.clone()]);
    if py_truthy(&Py::Bool(py_is(&frame.clone(), &Py::None))) {
    return py_str("");
    }
    when = _no_hanging_end(&[mcall(&mcall(&frame.clone(), "group", &[py_str("when")]), "strip", &[py_str(" ,;:")])]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp10 = call_builtin("len", &[mcall(&when.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &g_STATED_LONGEST()) } else { Py::Bool(false) }))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_NAMES_NOTHING(), &mcall(&mcall(&when.clone(), "strip", &[py_str(" .,")]), "lower", &[])))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_NAMES_A_CLINICIAN(), "search", &[when.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("only"), py_str("just")]), &mcall(&geti(&mcall(&when.clone(), "split", &[]), &Py::Int(0)), "lower", &[]))) } }) {
    return py_str("");
    }
    head = geti(&mcall(&when.clone(), "split", &[]), &Py::Int(0));
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&head.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&slice_or_raise(&head.clone(), &Py::Int(1), &Py::None, &Py::None), "islower", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("i"), py_str("a&e"), py_str("nhs")]), &mcall(&head.clone(), "lower", &[]))) } } } }) {
    when = add(&mcall(&geti(&when.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&when.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return when.clone();
    Py::None
}
pub fn _stated_referral(args: &[Py]) -> Py {
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
    let mut opens: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The threshold the answer stated as its own judgement, and how it is written.\n\n    The kind matters because the two cannot be phrased alike: a clause reads after an \"if\"\n    (\"...if it is one-sided and bad\") and a thing does not (\"if three weeks of it\" is clipped),\n    so a noun is named as the thing that changes the answer instead.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_JUDGED_REFERRAL(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DECLARATIVE_REFERRAL(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REFER(), "search", &[sentence.clone()]) } } } }))) {
    continue;
    }
    opens = stdlib_call("re", "match", &[py_str("(?:so )?if "), sentence.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = opens.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&sentence.clone(), &py_str(" if "))) } }) {
    when = _after_the_if(&[sentence.clone()]);
    if py_truthy(&when.clone()) {
    when = stdlib_call("re", "sub", &[py_str("\\b(or|and) if\\b"), py_str("\\1"), when.clone()]).unwrap_or_else(|e| exc::raise(e));
    when = _no_hanging_end(&[mcall(&geti(&stdlib_call("re", "split", &[py_str("\\s+-\\s+|\\s+--\\s+"), when.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(" ,.;:")])]);
    when = mcall(&geti(&mcall(&when.clone(), "split", &[py_str(", ask ")]), &Py::Int(0)), "strip", &[py_str(" ,.;:")]);
    if py_truthy(&ge(&call_builtin("len", &[mcall(&when.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_tuple(vec![when.clone(), py_str("clause")]);
    }
    }
    continue;
    }
    when = _stated_before_the_judgement(&[sentence.clone()]);
    if py_truthy(&when.clone()) {
    return py_tuple(vec![when.clone(), py_str("noun")]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn _danger(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut best: Py = Py::None;
    let mut condition: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The first harm the answer names that is not itself denied (\"not dangerous\").\n\n    A harm named inside an \"if\" is the answer's warning for later, not its verdict now: \"My arm\n    aches\" ends \"treat a heavy left arm with tightness in the chest, sweating or breathlessness\n    as an emergency\", and that answer is not an answer that says the ache is an emergency.\n    ");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_DANGER());
    for __item in __iter1 {
    word = __item.clone();
    start = mcall(&said.clone(), "find", &[word.clone()]);
    while py_truthy(&ge(&start.clone(), &Py::Int(0))) {
    before = slice_or_raise(&said.clone(), &call_builtin("max", &[Py::Int(0), sub(&start.clone(), &Py::Int(14))]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None);
    sentence = slice_or_raise(&said.clone(), &call_builtin("max", &[Py::Int(0), mcall(&said.clone(), "rfind", &[py_str(". "), Py::Int(0), start.clone()])]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None);
    condition = stdlib_call("re", "search", &[py_str("\\b(?:if|unless|when|once)\\b"), sentence.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_NEGATED(), "search", &[before.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&condition.clone())) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&start.clone(), &best.clone()) } }) {
    best = start.clone();
    }
    break;
    }
    start = mcall(&said.clone(), "find", &[word.clone(), add(&start.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)))]);
    }
    }
    return best.clone();
    Py::None
}
pub fn _a_how_to(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut part: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the point of the answer is a job to do, in any of its sentences.");
    let __iter1 = py_iter_or_raise(&add(&slice_or_raise(&mcall(&g_AFTER(), "split", &[said.clone()]), &Py::Int(1), &Py::None, &Py::None), &stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), said.clone()]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",|;| and | then | so | but "), mcall(&part.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    clause = __item.clone();
    words = mcall(&mcall(&clause.clone(), "strip", &[py_str(" .:;,-")]), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DOING(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _round_robin(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut c: Py = Py::None;
    let mut i: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The same pool, rotated by a positional hash of the answer.\n\n    `pick` seeds on the sum of the characters, and two long answers collide on that easily: the\n    damp, the rice and the radiator are all ordinary healthy sentences, and the sum landed on\n    the same line for all three, so three rows of the deck got one sentence word for word. A\n    rotation weighted by where the letters sit in the answer spreads them; `pick` still decides\n    which of the rotated pool is said, so the no-repeat rules keep working.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pool.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return pool.clone();
    }
    start = modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5974 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk5974[0].clone(); let mut c: Py = __unpk5974[1].clone(); __out.push(mul(&add(&i.clone(), &Py::Int(1)), &call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e)))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return add(&call_builtin("tuple", &[slice_or_raise(&pool.clone(), &start.clone(), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[slice_or_raise(&pool.clone(), &Py::None, &start.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _says_what_it_is(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when some clause of the answer tells the person something rather than telling them\n    to do something.\n\n    \"Half fill the kettle with equal parts water and white vinegar, boil it and leave it half an\n    hour\" is a recipe and nothing else: it answers how to clean a kettle, not whether limescale\n    matters, so \"is that bad\" after it is still the person's own question and the composer has\n    nothing to say about it. An answer that explains the thing first - \"she is bringing food to\n    the household rather than being naughty, so take the bird away\" - has said something, and\n    that reading is the verdict.\n    ");
    let __iter1 = py_iter_or_raise(&add(&slice_or_raise(&mcall(&g_AFTER(), "split", &[said.clone()]), &Py::Int(1), &Py::None, &Py::None), &stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), said.clone()]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",|;| and | then | so | but |:"), mcall(&part.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    clause = __item.clone();
    words = mcall(&mcall(&clause.clone(), "strip", &[py_str(" .:;,-")]), "split", &[]);
    leads = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &Py::Int(2), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&leads.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_line_it_answered(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own line that the answer in `said` was answering.\n\n    Not the last line they typed: the line that prompted a bare \"is that bad\" is the one before\n    it, and that is the line whose span says how long the thing has been going on.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&_flat(&[mcall(&last.clone(), "get", &[py_str("bot")])]), &_flat(&[said.clone()])))) {
    return _flat(&[mcall(&last.clone(), "get", &[py_str("user")])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _past_the_timescale(args: &[Py]) -> Py {
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
    let mut ours: Py = Py::None;
    let _ = py_str("True when the answer set a timescale on watching it and the person is already past it.\n\n    Read only where the answer hedged on time and the person's own line says the thing has been\n    going on longer: \"i wake up soaked in sweat every night for months\" is not answered by\n    \"Not the thing itself, no - what would change it is you wake drenched for weeks\", because\n    the answer's own threshold is already met. A line with no span (\"i wake up soaked\") keeps\n    the ordinary reading: there the answer is telling them the timescale to watch.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&g_HEDGE_SPAN(), "search", &[said.clone()]), &Py::None)) } }) {
    return Py::Bool(false);
    }
    line = { let __b = _the_line_it_answered(&[said.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(py_is(&mcall(&g_LONG_SPAN(), "search", &[line.clone()]), &Py::None))) {
    line = { let __b = _the_span_further_up(&[said.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { line.clone() } };
    }
    ours = mcall(&g_LONG_SPAN(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&ours.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:days?|weeks?)\\s+ago\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    ours = Py::None;
    }
    return Py::Bool(!py_is(&ours.clone(), &Py::None));
    Py::None
}
pub fn _subject_words(args: &[Py]) -> Py {
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
    let _ = py_str("The long words of a line, cut to five letters: \"coughs\" and \"cough\" are the same subject.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[text.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FENCE_FILLER(), &word.clone())) } }) {  __out.push(slice_or_raise(&word.clone(), &Py::None, &Py::Int(5), &Py::None)); } } py_set(__out) })() };
    Py::None
}
pub fn _the_span_further_up(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A span further up the chat, on the same subject as the answer in front of the doubt.\n\n    The span and the doubt are rarely in the same line: \"i have had a cough for months\" / \"what\n    should i do first\" / \"is that bad\" is one chat, and the answer that hedged on three weeks\n    answered the middle line, which says nothing about how long. Read on its own, that line has no\n    span, so the fence said \"not yet - three weeks of it is the point where it wants a person\" to\n    somebody who is already at months.\n\n    The subject is what keeps it honest: an earlier line counts only when the answer that answered\n    it and the answer in front of us are about the same thing, so a \"for months\" about the rent\n    cannot decide a cough. Getting this wrong errs one way - no span means the ordinary reading,\n    which is what the chat had before - so where the span comes from is read generously.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return py_str("");
    }
    here = _subject_words(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&here.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = _flat(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&Py::Bool(py_is(&mcall(&g_LONG_SPAN(), "search", &[line.clone()]), &Py::None))) {
    continue;
    }
    if py_truthy(&bitand(&_subject_words(&[mcall(&turn.clone(), "get", &[py_str("bot")])]), &here.clone())) {
    return line.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _stems(args: &[Py]) -> Py {
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
    let mut suffix: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The content words of a line, stemmed, so \"feeding\" and \"feeds\" are the same word.");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&_flat(&[text.clone()]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str("'")]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FENCE_FILLER(), &word.clone())) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter2 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[suffix.clone()]); if !py_truthy(&__b) { __b } else { ge(&sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(4)) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    break;
    }
    }
    let _ = mcall(&words.clone(), "add", &[word.clone()]);
    }
    return words.clone();
    Py::None
}
pub fn _the_fence_is_their_own_line(args: &[Py]) -> Py {
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
    let mut fence: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("True when the answer's condition is the very thing the person just told us.\n\n    Read from the answer's own words against the person's own words, not from the topic: when\n    half the condition's content words are theirs, the condition describes what they said, and\n    quoting it back to them decides nothing. \"i sweat a lot at night\" / \"is that bad\" got \"It\n    stops being ordinary if you sweat buckets from one place or at night\": the answer's own \"if\"\n    is already true of the line in front of it, so the reassurance in front of it is wrong.\n    ");
    when = _when(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    return Py::Bool(false);
    }
    fence = _stems(&[when.clone()]);
    theirs = _stems(&[_the_line_it_answered(&[said.clone(), memory.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&fence.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&theirs.clone())) } }) {
    return Py::Bool(false);
    }
    shared = bitand(&fence.clone(), &theirs.clone());
    return { let __b = call_builtin("bool", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { ge(&mul(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[fence.clone()]).unwrap_or_else(|e| exc::raise(e))) } };
    Py::None
}
pub fn _a_mood(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line the ask follows is about how the person feels rather than a body.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_last_person_lines(&[memory.clone(), Py::Int(2)])) { let mut line: Py = __item.clone(); __out.push(mcall(&g_MOOD(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about_a_death(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when one of the last few lines is a death or a miscarriage.\n\n    The death word alone is not enough: a laptop, a battery and a boiler \"die\" too, and \"dead\"\n    is as often the intensifier (\"dead comfy\", \"dead quiet\", \"dead tired\"). Read that way a\n    fault got the condolence: \"my laptop battery dies fast\" / \"is it safe\" was answered \"That is\n    not a thing to call bad, and not a thing to carry on your own\", which is the coldest line in\n    this file over a flat battery. The thing the word is about decides, and `compose/loss.py` is\n    where that reading lives - it is already the reader that tells a phone that stopped from a\n    father who died.\n    ");
    let __iter1 = py_iter_or_raise(&_the_last_person_lines(&[memory.clone(), Py::Int(3)]));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_LOSS_WORDS(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__a_thing_stopped(), &Py::None)); if !py_truthy(&__b) { __b } else { py_call(&g__a_thing_stopped(), &[line.clone()]) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_last_person_lines(args: &[Py]) -> Py {
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
    let mut count: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last few things the person said, oldest first, skipping the echoes doubt asks make.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![]);
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(_flat(&[mcall(&turn.clone(), "get", &[py_str("user")])])); } } py_list(__out) })() };
    return slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(line.clone()); } } py_list(__out) })() }, &neg(&count.clone()), &Py::None, &Py::None);
    Py::None
}
pub fn _a_job_to_do(args: &[Py]) -> Py {
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
    let mut piece: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a run of words holds an instruction the person could go and do.\n\n    Read clause by clause, because an answer often puts the job after the reason for it: \"The wait\n    is worse than the call nearly every time, and you can keep it short once you are through - say\n    the one thing you actually want to say\" is advice with its step in the second half, and\n    reading only the opening of the sentence would call it a verdict. `_STEP_OPENERS` is the\n    vocabulary `_an_instruction_and_nothing_else` already trusts for \"this answer tells them what\n    to do\", so the two readings cannot drift apart.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[.!?;:]\\s+| \\u2014 | \\u2013 | - "), _flat(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    piece = __item.clone();
    words = mcall(&g__WORD(), "findall", &[mcall(&piece.clone(), "strip", &[py_str(" ,-")])]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__STEP_OPENERS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_advice_itself(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's first real sentence, when the answer holds a job to go and do.\n\n    \"Go, and give yourself an exit:\" is the opening of a recommendation whose rest is a list, and\n    handed back with its colon it reads as a sentence that stops dead. The words are the answer's\n    own either way.\n\n    It also has to hold a job: an answer can open on a verdict or on a shrug rather than on\n    something to do, and \"Yes - go with that. Not unreasonable at all.\" (over \"should i worry\") or\n    \"Yes - go with that. Not mine to decide, sorry.\" (over \"should i ring in sick\") tells the\n    person to go and do nothing. `_a_job_to_do` is the reading for that, and where no clause of\n    the answer opens on a job the arm stands aside and the line it replaced keeps the turn.\n    Found by @snappy-net on #7845.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    clean = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&mcall(&clean.clone(), "endswith", &[py_str(":")])) {
    clean = mcall(&slice_or_raise(&clean.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), "strip", &[]);
    }
    if py_truthy(&ge(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    if py_truthy(&Py::Bool(!py_truthy(&_a_job_to_do(&[clean.clone()])))) {
    return py_str("");
    }
    return if py_truthy(&mcall(&clean.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { clean.clone() } else { add(&clean.clone(), &py_str(".")) };
    }
    }
    return py_str("");
    Py::None
}
pub fn _an_ask_for_a_call(args: &[Py]) -> Py {
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
    let _ = py_str("True when the person's own last line asked for a recommendation.");
    said = mcall(&_flat(&[_last_person_line(&[memory.clone()])]), "strip", &[]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_ASKED_FOR_A_CALL(), "match", &[said.clone()]), &Py::None)) } };
    Py::None
}
pub fn _a_fix_with_no_warning(args: &[Py]) -> Py {
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
    let _ = py_str("True for an answer that gives a job to do and says nothing about danger either way.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(80)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_says_what_it_is(&[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&_first(&[said.clone(), g_CARE()]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_REFER(), "search", &[said.clone()]), &Py::None)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&_first(&[said.clone(), g_REASSURE()]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_first(&[said.clone(), g_STOP_DOING()]), &Py::None)) } }) {
    return Py::Bool(false);
    }
    return _a_how_to(&[said.clone()]);
    Py::None
}
pub fn _somebody_owes_them(args: &[Py]) -> Py {
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
    else { Py::Int(3) };
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when one of the last few lines named somebody who owes the person a duty.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = { let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&last.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&last.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[line.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_OWED(), "search", &[line.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_reading_of_the_feeling(args: &[Py]) -> Py {
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
    let mut flat: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "flat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: flat"))) };
    let mut behind: Py = Py::None;
    let mut family: Py = Py::None;
    let mut seed: Py = Py::None;
    let _ = py_str("The sentence for a doubt about a state the person named themselves, or None.\n\n    Shared by both passes over the doubt: a chat line that named a feeling is answered with the\n    reading for that family rather than with a question floor, and the pass after the answer\n    finds the same family. The rotation is taken as it comes rather than handed to `pick`: `pick`\n    seeds on the sum of the characters, and three of these pairs (\"i feel low\" / \"is that bad\",\n    \"i feel anxious all day\" / \"am i ok\", \"ive been feeling flat all week\" / \"is that weird\")\n    collided on one sentence. The count of turns is in the seed so the same ask twice in a row\n    does not get the same sentence back.\n    ");
    behind = _the_line_behind(&[memory.clone()]);
    family = _a_feeling_of_their_own(&[behind.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    return Py::None;
    }
    seed = add(&add(&add(&add(&behind.clone(), &py_str(" ")), &flat.clone()), &py_str(" ")), &call_builtin("str", &[call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    return geti(&_round_robin(&[geti(&g_FEELING_DOUBT_LINES(), &family.clone()), seed.clone()]), &Py::Int(0));
    Py::None
}
pub fn _care_pool(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let _ = py_str("Which care pool a normality ask gets, by what the answer already said.\n\n    Three readings of one answer, in the order they can be true: a card that put a time on its\n    referral has already answered the doubt (\"ring the vet today\" is not a thing to wonder about),\n    so the reply says to do it. A card that said how many people it happens to has answered\n    \"is it normal\" itself, so the reply must not say it cannot tell. Only a card silent on both\n    gets the honest \"I could not call that usual or not from here\".\n    ");
    flat = _flat(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_is(&mcall(&g_CARE_NOW(), "search", &[flat.clone()]), &Py::None))) {
    return g_CARE_NOW_NORMAL();
    }
    if py_truthy(&Py::Bool(!py_is(&_first_positive(&[flat.clone(), g_SAYS_NORMAL()]), &Py::None))) {
    return g_ORDINARY_NORMAL();
    }
    if py_truthy(&Py::Bool(!py_is(&_first_positive(&[flat.clone(), g_HOW_MANY_PEOPLE()]), &Py::None))) {
    return slice_or_raise(&g_CARE_NORMAL(), &Py::None, &Py::Int(1), &Py::None);
    }
    if py_truthy(&Py::Bool(!py_is(&_first_positive(&[flat.clone(), g_REASSURE()]), &Py::None))) {
    return g_CARE_NORMAL();
    }
    return add(&g_CARE_NORMAL(), &g_CARE_NORMAL_UNKNOWABLE());
    Py::None
}
pub fn _the_knowledge_has_this(args: &[Py]) -> Py {
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
    let mut above: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "above") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: above"))) };
    let mut card: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The knowledge base's own new answer to this line, or None.\n\n    Read the way `safetyask` reads it: on a copy of the memory, so asking changes nothing about\n    this turn. A card that cannot be read is no card.\n\n    A card that only says the answer above again is not a new answer either, and asking the\n    knowledge base does not tell the two apart: for \"will that help\" after the back-pain card it\n    hands back that card, sometimes with the nod (\"Same as before: ...\") and sometimes without.\n    So the test is the words - the answer above's own opening, said again - and not the nod in\n    front of them. The person asking whether the method works has read that paragraph.\n    ");
    let __try1 = exc::catch(|| {
    let mut _the_answers_have_this: Py = py_func("_the_answers_have_this", |args: &[Py]| crate::gen::compose::safetyask::_the_answers_have_this(args));
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
    card = py_call(&_the_answers_have_this, &[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&card.clone()))) {
    return Py::None;
    }
    said = _flat(&[card.clone()]);
    opening = slice_or_raise(&_flat(&[above.clone()]), &Py::None, &Py::Int(60), &Py::None);
    if py_truthy(&{ let __b = opening.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &opening.clone())) } }) {
    return Py::None;
    }
    return card.clone();
    Py::None
}
pub fn a_check_on_the_reply(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when a reply is this composer's answer to a check on the answer above, else False.\n\n    Read by the arbiter: a check on the method just given is not answered by saying that method\n    again, so the recall band - which repeats the paragraph - gives way to this line. The pools\n    are the whole reading, because the same check is asked in many wordings.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    text = mcall(&reply.clone(), "strip", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![g_CHECK_WORKS_LINES(), g_CHECK_ONCE_LINES(), g_CHECK_WATCH_LINES(), g_CHECK_JOB_LINES()])) { let mut pool: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "strip", &[])); } py_set(__out) })() }, &text.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_amount_asked(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the check is about how much of a thing, which is the one kind a card can answer.");
    return call_builtin("bool", &[{ let __b = mcall(&g_ENOUGH_ASK(), "match", &[flat.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow (?:much|many)\\b|\\bamount\\b|\\benough\\b"), { let __b = flat.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_last_thing_said_is_a_nod(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the bot's last turn only said the person was heard.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    return { let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _a_punt(&[last.clone()]) } };
    Py::None
}
pub fn _a_check_on_the_method(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut behind: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut warns: Py = Py::None;
    let _ = py_str("\"Will that work\" / \"will that help\" / \"is that enough\" about the answer above, or None.\n\n    Only an answer that is a method is what the check is about: the person is asking whether the\n    thing the bot just told them to do does anything. A check on a choice the bot has no view on\n    (\"we are thinking about a second baby\" / \"will that work\") keeps the line it has today - the\n    answer above asked the person a question back, and no pool here can say the question works.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_nod".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_reply(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_how_to(&[said.clone()]))) } }) {
    return Py::None;
    }
    behind = _flat(&[{ let __b = _last_person_line(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = mcall(&g_A_THINKING_ABOUT(), "search", &[behind.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DECISION_ASK(), "search", &[behind.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_PLAN(), "search", &[behind.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&_last_module(&[memory.clone()]), &py_str("answers")))) {
    return Py::None;
    }
    if py_truthy(&_the_last_thing_said_is_a_nod(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _an_amount_asked(&[flat.clone()]); if !py_truthy(&__b) { __b } else { _the_knowledge_has_this(&[flat.clone(), memory.clone(), said.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_about_a_body(&[behind.clone()])))) {
    pool = g_CHECK_JOB_LINES();
    } else {
    warns = { let __b = Py::Bool(!py_is(&mcall(&g_REFER(), "search", &[said.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[said.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_first(&[mcall(&said.clone(), "split", &[]), g_STOP_DOING()]), &Py::None)) } } } };
    if py_truthy(&warns.clone()) {
    pool = g_CHECK_WATCH_LINES();
    } else {
    if py_truthy(&mcall(&g_A_LENGTH_IN_THE_ANSWER(), "search", &[_flat(&[said.clone()])])) {
    pool = g_CHECK_WORKS_LINES();
    } else {
    pool = g_CHECK_ONCE_LINES();
    }
    }
    }
    return crate::gen::compose::pick(&[_round_robin(&[pool.clone(), add(&add(&flat.clone(), &py_str(" ")), &said.clone())]), memory.clone(), said.clone()]);
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut action: Py = Py::None;
    let mut anywhere: Py = Py::None;
    let mut behind: Py = Py::None;
    let mut call: Py = Py::None;
    let mut care: Py = Py::None;
    let mut care_at: Py = Py::None;
    let mut checked: Py = Py::None;
    let mut denies: Py = Py::None;
    let mut enough_ask: Py = Py::None;
    let mut family: Py = Py::None;
    let mut fence: Py = Py::None;
    let mut fenced: Py = Py::None;
    let mut figure_line: Py = Py::None;
    let mut figure_said: Py = Py::None;
    let mut first_said: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut money: Py = Py::None;
    let mut named: Py = Py::None;
    let mut no_stated: Py = Py::None;
    let mut normal_ask: Py = Py::None;
    let mut ok_ask: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut ordinary: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut referral: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut step: Py = Py::None;
    let mut tested: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut warned: Py = Py::None;
    let mut warns: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let mut worst: Py = Py::None;
    let mut written: Py = Py::None;
    let mut yes_pool: Py = Py::None;
    let _ = py_str("The answer to a bare \"is that bad / what should i do / should i see someone\".");
    flat = mcall(&_flat(&[clean.clone()]), "strip", &[py_str(" .!?")]);
    let mut safetyask: Py = crate::gen::compose::safetyask::module_obj();
    if py_truthy(&{ let __b = mcall(&safetyask.clone(), "the_bot_already_ruled", &[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_DOUBT_ABOUT_THE_THING(), "search", &[flat.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&safetyask.clone(), "A_RULING_ALREADY")) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&flat.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return crate::gen::compose::pick(&[gattr(&safetyask.clone(), "A_SAFETY_ALREADY_RULED"), memory.clone(), flat.clone()]);
    }
    if py_truthy(&mcall(&g_A_CHECK_ON_A_PLAN(), "match", &[flat.clone()])) {
    plan = _the_plan_they_brought(&[memory.clone()]);
    if py_truthy(&plan.clone()) {
    return crate::gen::compose::pick(&[_round_robin(&[g_A_CHECK_ON_A_PLAN_LINES(), plan.clone()]), memory.clone(), plan.clone()]);
    }
    }
    if py_truthy(&mcall(&g_METHOD_DOUBT(), "match", &[flat.clone()])) {
    behind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_nod".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_reply(args))(&__args) };
    line = _the_line_behind(&[memory.clone()]);
    let __unpk1 = py_unpack(&_as_written(&[line.clone()]), 2, None);
    written = __unpk1[0].clone();
    spoken = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&behind.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_how_to(&[behind.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_HOW_TO_ASK(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HOW_TO_ASK(), "search", &[written.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_HOW_TO_ASK(), "search", &[spoken.clone()]) } } } })) } } } }) {
    return _a_check_on_the_method(&[flat.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&_danger(&[behind.clone()]), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&mcall(&g_REFER(), "search", &[behind.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[behind.clone()]), &Py::None)) } }) {
    return _a_check_on_the_method(&[flat.clone(), memory.clone()]);
    }
    checked = _a_check_on_the_method(&[flat.clone(), memory.clone()]);
    if py_truthy(&checked.clone()) {
    return checked.clone();
    }
    return crate::gen::compose::pick(&[_round_robin(&[g_METHOD_OK_LINES(), behind.clone()]), memory.clone(), behind.clone()]);
    }
    if py_truthy(&mcall(&g_A_FIGURE_ASK(), "match", &[flat.clone()])) {
    figure_line = _the_line_behind(&[memory.clone()]);
    figure_said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_nod".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_reply(args))(&__args) };
    if py_truthy(&{ let __b = figure_line.clone(); if !py_truthy(&__b) { __b } else { { let __b = _a_figure_owed(&[figure_line.clone()]); if !py_truthy(&__b) { __b } else { figure_said.clone() } } } }) {
    if py_truthy(&Py::Bool(!py_is(&_first(&[figure_said.clone(), g_A_COMPARISON()]), &Py::None))) {
    return crate::gen::compose::pick(&[g_MONEY_LINES(), memory.clone(), figure_said.clone()]);
    }
    if py_truthy(&mcall(&g_A_FIGURE(), "search", &[figure_said.clone()])) {
    return crate::gen::compose::pick(&[g_FIGURE_LINES(), memory.clone(), figure_said.clone()]);
    }
    }
    return Py::None;
    }
    enough_ask = call_builtin("bool", &[mcall(&g_ENOUGH_ASK(), "match", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_BARE(), "match", &[flat.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&enough_ask.clone())) } }) {
    if py_truthy(&mcall(&g_A_CHECK_ASK(), "search", &[flat.clone()])) {
    return _a_check_on_the_method(&[flat.clone(), memory.clone()]);
    }
    return Py::None;
    }
    normal_ask = call_builtin("bool", &[mcall(&g_ASKS_NORMAL(), "search", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e));
    ok_ask = { let __b = Py::Bool(!py_truthy(&normal_ask.clone())); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_ASKS_OK(), "search", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_nod".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_reply(args))(&__args) };
    plan = _the_line_behind(&[memory.clone()]);
    family = _the_plan_can_hurt(&[_flat(&[plan.clone()])]);
    if py_truthy(&family.clone()) {
    warned = _the_answer_already_warned(&[_flat(&[said.clone()])]);
    if py_truthy(&warned.clone()) {
    return add(&py_str("Go with the answer above rather than around it: "), &warned.clone());
    }
    return crate::gen::compose::pick(&[_round_robin(&[geti(&g_THE_PLAN_CAN_HURT_LINES(), &family.clone()), _flat(&[plan.clone()])]), memory.clone(), _flat(&[plan.clone()])]);
    }
    if py_truthy(&{ let __b = _about_a_death(&[memory.clone()]); if !py_truthy(&__b) { __b } else { _last_module(&[memory.clone()]) } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_GRIEF_FLOOR_LINES(), { let __b = _the_line_it_answered(&[said.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { flat.clone() } }]), memory.clone(), flat.clone()]);
    }
    if py_truthy(&enough_ask.clone()) {
    stated = _said_enough(&[said.clone()]);
    if py_truthy(&Py::Bool(py_is(&stated.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(py_eq(&_last_module(&[memory.clone()]), &py_str("answers")))) {
    checked = _a_check_on_the_method(&[flat.clone(), memory.clone()]);
    if py_truthy(&checked.clone()) {
    return checked.clone();
    }
    }
    return Py::None;
    }
    return mcall(&crate::gen::compose::pick(&[_round_robin(&[g_ENOUGH_STATED_LINES(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]), memory.clone(), said.clone()]), "replace", &[py_str("{yes}"), stated.clone()]);
    }
    if py_truthy(&{ let __b = _considering(&[memory.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_PLAN(), "search", &[_flat(&[_the_line_behind(&[memory.clone()])])]) } }) {
    return _a_plan_doubt(&[memory.clone()]);
    }
    worst = _the_worst_thing_they_said(&[memory.clone()]);
    if py_truthy(&{ let __b = worst.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_SYMPATHY(), "search", &[_flat(&[said.clone()])]) } }) {
    seed = add(&add(&add(&add(&worst.clone(), &py_str(" ")), &flat.clone()), &py_str(" ")), &call_builtin("str", &[call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    return geti(&_round_robin(&[g_HARD_NEWS_LINES(), seed.clone()]), &Py::Int(0));
    }
    money = _a_doubt_about_money(&[memory.clone()]);
    if py_truthy(&{ let __b = money.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("answers"), py_str("chat"), py_str("greeting"), py_str("trouble")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_nod".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_module(args))(&__args) })) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_about_a_death(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_the_worst_thing_they_said(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__A_NUMBER(), "search", &[money.clone()]); if !py_truthy(&__b) { __b } else { _is_a_comparison(&[said.clone()]) } })) } } } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_MONEY_TROUBLE_LINES(), { let __b = said.clone(); if py_truthy(&__b) { __b } else { money.clone() } }]), memory.clone(), { let __b = said.clone(); if py_truthy(&__b) { __b } else { money.clone() } }]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_nod".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_module(args))(&__args) }, &py_str("answers"))) } }) {
    reading = _a_reading_of_the_feeling(&[memory.clone(), flat.clone()]);
    if py_truthy(&reading.clone()) {
    return reading.clone();
    }
    if py_truthy(&mcall(&g_HELPED(), "search", &[_flat(&[_last_person_line(&[memory.clone()])])])) {
    return crate::gen::compose::pick(&[_round_robin(&[g_HELPED_LINES(), _flat(&[_last_person_line(&[memory.clone()])])]), memory.clone(), _flat(&[_last_person_line(&[memory.clone()])])]);
    }
    named = _last_line_that_named_a_call(&[memory.clone()]);
    if py_truthy(&named.clone()) {
    if py_truthy(&_a_call_for_now(&[named.clone()])) {
    call = _the_call_sentence(&[named.clone()]);
    seed = add(&add(&flat.clone(), &py_str(" ")), &call.clone());
    if py_truthy(&{ let __b = call.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_URGENT_CALL(), "search", &[call.clone()]) } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_YES_SAID_LINES(), call.clone()]), memory.clone(), seed.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&mcall(&g_TRADE(), "search", &[named.clone()]), &Py::None))) {
    return crate::gen::compose::pick(&[slice_or_raise(&g_YES_LINES(), &Py::None, &Py::Int(2), &Py::None), memory.clone(), flat.clone()]);
    }
    return crate::gen::compose::pick(&[g_SOON_LINES(), memory.clone(), flat.clone()]);
    }
    when = _when(&[named.clone()]);
    if py_truthy(&when.clone()) {
    return mcall(&crate::gen::compose::pick(&[g_CONDITION_LINES(), memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), when.clone()]);
    }
    }
    if py_truthy(&mcall(&g_WORRIED_ASK(), "match", &[flat.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("chat"), py_str("greeting"), py_str("trouble")]), &_last_module(&[memory.clone()])))) {
    reading = _a_reading_of_the_feeling(&[memory.clone(), flat.clone()]);
    if py_truthy(&reading.clone()) {
    return reading.clone();
    }
    if py_truthy(&{ let __b = _own_doing(&[memory.clone()]); if !py_truthy(&__b) { __b } else { _a_mood(&[memory.clone()]) } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_BACKSTOP_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    step = _the_advice_itself(&[said.clone()]);
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { { let __b = _an_ask_for_a_call(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_REFUSAL(), "search", &[said.clone()]))) } } } }) {
    return mcall(&crate::gen::compose::pick(&[_round_robin(&[g_ADVICE_OK_LINES(), said.clone()]), memory.clone(), said.clone()]), "replace", &[py_str("{step}"), step.clone()]);
    }
    if py_truthy(&{ let __b = _own_doing(&[memory.clone()]); if !py_truthy(&__b) { __b } else { _a_fix_with_no_warning(&[said.clone()]) } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_NOTHING_WARNED_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&{ let __b = _own_doing(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_nothing_to_be_long_about(&[memory.clone()]))) } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_FLOOR_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&{ let __b = _about_the_person(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_nothing_to_be_long_about(&[memory.clone()]))) } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_NEUTRAL_ASK_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&_the_floor_asked_them_something(&[memory.clone()])) {
    return crate::gen::compose::pick(&[_round_robin(&[g_ASKED_THEM_LINES(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]), memory.clone(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]);
    }
    }
    return _a_plan_doubt(&[memory.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_WHEN_ASK(), "match", &[flat.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORRYING_ASK(), "match", &[flat.clone()]) } }) {
    when = { let __b = _when(&[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_list_before_the_call(&[said.clone()]); if py_truthy(&__b) { __b } else { geti(&_stated_referral(&[said.clone()]), &Py::Int(0)) } } } };
    if py_truthy(&when.clone()) {
    return mcall(&crate::gen::compose::pick(&[g_WHEN_LINES(), memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), when.clone()]);
    }
    call = _the_call_and_its_reason(&[said.clone()]);
    if py_truthy(&{ let __b = call.clone(); if !py_truthy(&__b) { __b } else { _they_named_the_sign_themselves(&[call.clone(), memory.clone()]) } }) {
    call = py_str("");
    }
    if py_truthy(&call.clone()) {
    return mcall(&crate::gen::compose::pick(&[g_WHEN_CALL_LINES(), memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), call.clone()]);
    }
    if py_truthy(&mcall(&g_WORRYING_ASK(), "match", &[flat.clone()])) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = _the_last_line_was_mine(&[memory.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_OWN_PATTERN(), "search", &[_flat(&[said.clone()])]) } }) {
    return crate::gen::compose::pick(&[g_AGAIN(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&add(&add(&add(&add(&add(&add(&add(&g_NO_LINES(), &g_DO_LINES()), &g_YES_LINES()), &g_ORDINARY_NORMAL()), &g_CARE_NORMAL()), &g_CARE_NORMAL_UNKNOWABLE()), &g_CARE_NOW_NORMAL()), &g_DO_NORMAL())) { let mut line: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_flat(&[said.clone()]), &mcall(&line.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return crate::gen::compose::pick(&[g_AGAIN(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&_first(&[said.clone(), g_AGAIN()]), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&_a_person_they_know(&[memory.clone(), said.clone()])) {
    if py_truthy(&{ let __b = _when(&[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_REFER(), "search", &[said.clone()]), &Py::None)) } }) {
    return crate::gen::compose::pick(&[g_PERSON_CALL(), memory.clone(), add(&add(&flat.clone(), &py_str(" ")), &_the_line_behind(&[memory.clone()]))]);
    }
    return crate::gen::compose::pick(&[g_PERSON_ORDINARY(), memory.clone(), add(&add(&flat.clone(), &py_str(" ")), &_the_line_behind(&[memory.clone()]))]);
    }
    opening = _opening(&[said.clone()]);
    thing = _a_thing_they_own(&[memory.clone()]);
    yes_pool = if py_truthy(&thing.clone()) { g_THING_CALL() } else { g_YES_LINES() };
    let __unpk2 = py_unpack(&_a_throw_away(&[opening.clone()]), 2, None);
    action = __unpk2[0].clone();
    tested = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_is(&action.clone(), &Py::None))) {
    pool = if py_truthy(&{ let __b = tested.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_person_gave_the_sign(&[memory.clone(), clean.clone()]))) } }) { g_TEST_LINES() } else { g_THROW_AWAY_LINES() };
    return mcall(&crate::gen::compose::pick(&[pool.clone(), memory.clone(), add(&add(&opening.clone(), &py_str(" ")), &_the_person_line(&[memory.clone(), clean.clone()]))]), "replace", &[py_str("{Action}"), action.clone()]);
    }
    no_stated = _said_no(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_is(&no_stated.clone(), &Py::None))) {
    return mcall(&crate::gen::compose::pick(&[_round_robin(&[g_NO_STATED_LINES(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]), memory.clone(), said.clone()]), "replace", &[py_str("{no}"), no_stated.clone()]);
    }
    care = _referral(&[said.clone()]);
    ordinary = _first_positive(&[opening.clone(), g_REASSURE()]);
    anywhere = care.clone();
    if py_truthy(&Py::Bool(!py_is(&_first(&[opening.clone(), g_STOP_DOING()]), &Py::None))) {
    return crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { _care_pool(&[said.clone()]) } else { if py_truthy(&thing.clone()) { g_THING_CALL() } else { slice_or_raise(&g_YES_LINES(), &Py::None, &Py::Int(2), &Py::None) } }, memory.clone(), flat.clone()]);
    }
    when = _when(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&when.clone(), &py_str(""))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_danger(&[said.clone()]), &Py::None)) } }) {
    return crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { _care_pool(&[said.clone()]) } else { if py_truthy(&thing.clone()) { g_THING_CALL() } else { slice_or_raise(&g_YES_LINES(), &Py::None, &Py::Int(2), &Py::None) } }, memory.clone(), flat.clone()]);
    }
    line = _the_line_behind(&[memory.clone()]);
    let __unpk3 = py_unpack(&_as_written(&[line.clone()]), 2, None);
    written = __unpk3[0].clone();
    spoken = __unpk3[1].clone();
    warns = { let __b = Py::Bool(!py_is(&mcall(&g_REFER(), "search", &[said.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[said.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_first(&[opening.clone(), g_STOP_DOING()]), &Py::None)) } } } };
    if py_truthy(&{ let __b = mcall(&g_SAID_YES(), "search", &[_flat(&[said.clone()])]); if !py_truthy(&__b) { __b } else { { let __b = _a_how_to(&[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&warns.clone())) } } } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_METHOD_OK_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_A_HOW_TO_ASK(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HOW_TO_ASK(), "search", &[written.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_HOW_TO_ASK(), "search", &[spoken.clone()]) } } } }; if !py_truthy(&__b) { __b } else { { let __b = _a_how_to(&[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&warns.clone())) } } } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_METHOD_OK_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_A_PLAN(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PLAN(), "search", &[written.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_PLAN(), "search", &[spoken.clone()]) } } } }) {
    return _a_plan_doubt(&[memory.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_A_DECISION_ASK(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_DECISION_ASK(), "search", &[written.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DECISION_ASK(), "search", &[spoken.clone()]) } } } }) {
    if py_truthy(&{ let __b = mcall(&g_ASKS_FINE(), "search", &[flat.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_WRONG(), "search", &[flat.clone()]))) } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_CHOICE_OK_LINES(), add(&add(&line.clone(), &py_str(" ")), &said.clone())]), memory.clone(), add(&add(&line.clone(), &py_str(" ")), &said.clone())]);
    }
    opening = _opening(&[said.clone()]);
    first_said = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&warns.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&_referral(&[first_said.clone()]), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&mcall(&g_NAMES_A_CLINICIAN(), "search", &[first_said.clone()]), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&g_REFER(), "search", &[first_said.clone()]), &Py::None)) } } } } } }) {
    fenced = _condition_after_care(&[said.clone(), _first(&[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), g_CARE()])]);
    if py_truthy(&fenced.clone()) {
    return mcall(&crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { g_CONDITION_NORMAL() } else { g_CONDITION_LINES() }, memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), fenced.clone()]);
    }
    let __unpk4 = py_unpack(&_stated_referral(&[said.clone()]), 2, None);
    stated = __unpk4[0].clone();
    kind = __unpk4[1].clone();
    sentence = _the_referral_sentence(&[said.clone()]);
    if py_truthy(&{ let __b = stated.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("noun"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_TIME_NOW(), "search", &[sentence.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_COMPARATIVE(), "search", &[sentence.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_line_is_the_threshold(&[stated.clone(), _the_line_it_answered(&[said.clone(), memory.clone()])]))) } } } } } } } }) {
    if py_truthy(&_past_the_timescale(&[said.clone(), memory.clone()])) {
    return crate::gen::compose::pick(&[_round_robin(&[g_MET_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SEE_STATED_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("when".to_string(), stated.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    return Py::None;
    }
    if py_truthy(&_care_for_now(&[first_said.clone()])) {
    return crate::gen::compose::pick(&[if py_truthy(&mcall(&g_URGENT_CALL(), "search", &[first_said.clone()])) { g_YES_SAID_LINES() } else { g_SOON_LINES() }, memory.clone(), flat.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&mcall(&g_JUST_AGE(), "search", &[flat.clone()])) {
    return crate::gen::compose::pick(&[g_AGE_LINES(), memory.clone(), said.clone()]);
    }
    if py_truthy(&{ let __b = _a_figure_owed(&[line.clone()]); if !py_truthy(&__b) { __b } else { _is_a_comparison(&[said.clone()]) } }) {
    return crate::gen::compose::pick(&[g_MONEY_LINES(), memory.clone(), said.clone()]);
    }
    care_at = _first(&[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), g_CARE()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&care.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&care_at.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&care.clone(), &care_at.clone()) } } } }) {
    care_at = care.clone();
    }
    if py_truthy(&_past_the_timescale(&[said.clone(), memory.clone()])) {
    return crate::gen::compose::pick(&[_round_robin(&[g_MET_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&care_at.clone(), &Py::None))) {
    fenced = _condition_after_care(&[said.clone(), care_at.clone(), memory.clone()]);
    if py_truthy(&fenced.clone()) {
    return mcall(&crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { g_CONDITION_NORMAL() } else { g_CONDITION_LINES() }, memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), fenced.clone()]);
    }
    }
    denies = _first(&[opening.clone(), g_DENIES()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&care.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&ordinary.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = lt(&care.clone(), &ordinary.clone()); if py_truthy(&__b) { __b } else { _a_plain_go(&[care.clone(), said.clone()]) } } } } } }) {
    return crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { _care_pool(&[said.clone()]) } else { yes_pool.clone() }, memory.clone(), flat.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&care.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&ordinary.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&care.clone(), &ordinary.clone()) } } } }) {
    return crate::gen::compose::pick(&[g_YES_LINES(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&denies.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&anywhere.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&ordinary.clone(), &Py::None)) } } } }) {
    return crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { _care_pool(&[said.clone()]) } else { yes_pool.clone() }, memory.clone(), flat.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&ordinary.clone(), &Py::None))) {
    if py_truthy(&{ let __b = when.clone(); if !py_truthy(&__b) { __b } else { normal_ask.clone() } }) {
    return mcall(&crate::gen::compose::pick(&[g_CONDITION_NORMAL(), memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), when.clone()]);
    }
    if py_truthy(&{ let __b = normal_ask.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&_first_positive(&[opening.clone(), g_SAYS_NORMAL()]), &Py::None)); if !py_truthy(&__b) { __b } else { _a_how_to(&[said.clone()]) } } } }) {
    return crate::gen::compose::pick(&[_round_robin(&[g_NOTHING_WARNED_NORMAL(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&_care_for_now(&[said.clone()])) {
    return crate::gen::compose::pick(&[g_BOTH_LINES(), memory.clone(), flat.clone()]);
    }
    fence = _a_judgement_fence(&[said.clone()]);
    if py_truthy(&fence.clone()) {
    return mcall(&crate::gen::compose::pick(&[g_CONDITION_LINES(), memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), fence.clone()]);
    }
    if py_truthy(&{ let __b = warns.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_when(&[said.clone()]))); if !py_truthy(&__b) { __b } else { _referral_about_the_thing(&[said.clone(), _the_line_behind(&[memory.clone()])]) } } } }) {
    return crate::gen::compose::pick(&[g_SEE_ANYWAY_LINES(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&normal_ask.clone())) } }) {
    return crate::gen::compose::pick(&[g_THING_ORDINARY(), memory.clone(), add(&add(&flat.clone(), &py_str(" ")), &thing.clone())]);
    }
    return crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { g_ORDINARY_NORMAL() } else { if py_truthy(&ok_ask.clone()) { g_OK_LINES() } else { g_NO_LINES() } }, memory.clone(), flat.clone()]);
    }
    when = _when(&[said.clone()]);
    if py_truthy(&when.clone()) {
    if py_truthy(&_the_fence_is_their_own_line(&[said.clone(), memory.clone()])) {
    if py_truthy(&_a_call_for_now(&[said.clone()])) {
    return crate::gen::compose::pick(&[g_YES_LINES(), memory.clone(), flat.clone()]);
    }
    return crate::gen::compose::pick(&[_round_robin(&[g_MET_CONDITION_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    return mcall(&crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { g_CONDITION_NORMAL() } else { g_CONDITION_LINES() }, memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), when.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&care.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&anywhere.clone(), &Py::None)) } }) {
    return crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { g_DO_NORMAL() } else { g_DO_LINES() }, memory.clone(), flat.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&_first(&[opening.clone(), g_CARE()]), &Py::None))) {
    if py_truthy(&Py::Bool(!py_is(&_first(&[opening.clone(), g_PEOPLE_CARE()]), &Py::None))) {
    return crate::gen::compose::pick(&[g_YES_LINES(), memory.clone(), flat.clone()]);
    }
    return crate::gen::compose::pick(&[if py_truthy(&normal_ask.clone()) { g_DO_NORMAL() } else { g_DO_LINES() }, memory.clone(), flat.clone()]);
    }
    if py_truthy(&_a_fix_with_no_warning(&[said.clone()])) {
    if py_truthy(&_somebody_owes_them(&[memory.clone()])) {
    return crate::gen::compose::pick(&[_round_robin(&[g_OWED_LINES(), said.clone()]), memory.clone(), said.clone()]);
    }
    pool = if py_truthy(&normal_ask.clone()) { g_NOTHING_WARNED_NORMAL() } else { g_NOTHING_WARNED_LINES() };
    return crate::gen::compose::pick(&[_round_robin(&[pool.clone(), said.clone()]), memory.clone(), said.clone()]);
    }
    if py_truthy(&_an_instruction_and_nothing_else(&[said.clone(), memory.clone()])) {
    referral = _the_referral_it_closed_on(&[said.clone()]);
    if py_truthy(&referral.clone()) {
    kept = _the_range_it_gave(&[said.clone()]);
    said_back = add(&referral.clone(), &if py_truthy(&kept.clone()) { { let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&kept.clone(), -1, "")); py_string(__s) } } else { py_str("") });
    return mcall(&crate::gen::compose::pick(&[_round_robin(&[g_HANDED_BACK_LINES(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]), memory.clone(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]), "replace", &[py_str("{referral}"), said_back.clone()]);
    }
    return crate::gen::compose::pick(&[_round_robin(&[g_DIRECTED_LINES(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]), memory.clone(), add(&add(&said.clone(), &py_str(" ")), &flat.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _the_referral_it_closed_on(args: &[Py]) -> Py {
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
    let mut act: Py = Py::None;
    let mut last: Py = Py::None;
    let mut s: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut unfenced: Py = Py::None;
    let _ = py_str("The answer's closing sentence, when that sentence says who to get or what to stop.\n\n    Only the closing sentence: an answer that names a professional in passing (\"if it is a gas\n    leak, a Gas Safe engineer needs to look at it\") and then ends on a step is a how-to, and the\n    pools above have it. What is left is an answer whose last word is the act.\n    ");
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) +"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut s: Py = __item.clone(); if py_truthy(&mcall(&s.clone(), "strip", &[])) {  __out.push(mcall(&s.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&sentences.clone()))) {
    return py_str("");
    }
    last = geti(&sentences.clone(), &neg(&Py::Int(1)));
    if py_truthy(&lt(&call_builtin("len", &[mcall(&last.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_REFER(), "search", &[last.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_REFERRAL_OUT(), "search", &[last.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_STOP(), "search", &[last.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_first(&[last.clone(), g_STOP_DOING()]), &Py::None)) } } } } } }))) {
    return py_str("");
    }
    unfenced = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:but )?(?:if|once|when|whenever) [^,]{0,70}, ")); __args.push(py_str("")); __args.push(last.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    act = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:stop|don't|dont|do not) [^,]{0,40}?,? (?:and|but) ")); __args.push(py_str("")); __args.push(unfenced.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&ge(&call_builtin("len", &[mcall(&act.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return act.clone();
    }
    return if py_truthy(&ge(&call_builtin("len", &[mcall(&unfenced.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) { unfenced.clone() } else { last.clone() };
    Py::None
}
pub fn _the_range_it_gave(args: &[Py]) -> Py {
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
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The range the answer told the person to hold, said as a line to keep, or \"\".\n\n    A referral on its own tells somebody to ring someone; the figure is what they do while they\n    wait, and it is the half of the answer they act on. \"between 1 and 1.5 bar\" comes back as\n    \"Keep it between 1 and 1.5 bar in the meantime.\"\n    ");
    found = mcall(&g__A_RANGE_OF_ITS_OWN(), "search", &[_flat(&[said.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(2)]), mcall(&found.clone(), "group", &[Py::Int(3)])]), 3, None);
    low = __unpk1[0].clone();
    high = __unpk1[1].clone();
    unit = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("and"), py_str("the"), py_str("for"), py_str("with")]), &mcall(&unit.clone(), "lower", &[])))) {
    return py_str("");
    }
    return { let mut __s = String::new(); __s.push_str("Keep it between "); __s.push_str(&fstr(&low.clone(), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&high.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str(" in the meantime."); py_string(__s) };
    Py::None
}
pub fn _an_instruction_and_nothing_else(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut head: Py = Py::None;
    let mut line: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the answer only told the person what to do, on a line that stated a situation.\n\n    A how-to the person asked for (\"how do i clean the kettle\") is not this: its answer is a\n    method, and the pools above have it. What is left is somebody describing their week - a mate\n    who keeps cancelling, a parking ticket, a rent rise - answered with the one thing to do about\n    it, and a bare doubt that no pool above could rule on.\n    ");
    flat = _flat(&[said.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&flat.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12))) {
    return Py::Bool(false);
    }
    head = mcall(&geti(&stdlib_call("re", "split", &[py_str("[:;.!?]"), flat.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(" ,-")]);
    words = mcall(&g__WORD(), "findall", &[head.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__STEP_OPENERS(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    return Py::Bool(false);
    }
    line = _flat(&[_last_person_line(&[memory.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_HOW_TO_ASK(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_floor_asked_them_something(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last line was a question rather than an answer to rule on.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&said.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g__AN_OFFER(), "search", &[said.clone()]) } }))) {
    return Py::Bool(false);
    }
    line = _flat(&[_the_line_behind(&[memory.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_HOW_TO_ASK(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&mcall(&g_A_WORRY_IN_THE_LINE(), "search", &[line.clone()]), &Py::None));
    Py::None
}
pub fn _a_feeling_of_their_own(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The family of state the person's own line named, or \"\" when it named none or a body.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[A-Za-z']+"), { let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&w.clone(), "replace", &[py_str("'"), py_str("")]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("i"), py_str("im"), py_str("ive"), py_str("id")]), &geti(&words.clone(), &Py::Int(0))))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BODY_WORD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SOMEONE_ELSE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FEELING_FAMILY(), &word.clone()))) {
    return geti(&g_FEELING_FAMILY(), &word.clone());
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_other_thing_in_the_answer(args: &[Py]) -> Py {
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
    let mut step: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut earlier: Py = Py::None;
    let mut heard: Py = Py::None;
    let mut i: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut needle: Py = Py::None;
    let mut one: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("A sentence of the answer above that is not the step and not already on the screen, or \"\".\n\n    The answer handed over a turn ago is not all one instruction: the card on vegetables opens\n    with what to do and then says what makes it work (\"Roasting carrots or broccoli in a little\n    oil makes them sweeter, and a dip to dunk turns it into a game\"). Asked for tips, a friend\n    picks that second sentence out, because it is the part the person has not acted on yet. Only\n    a sentence that stands on its own is picked: not the step's own sentence, not a question, not\n    the warning that ends a health answer, and not something the bot has said in an earlier turn.\n    ");
    let mut RED_FLAG: Py = crate::gen::memory::topics::g_RED_FLAG();
    needle = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]), "strip", &[py_str(" .!?,")]);
    mine = _flat(&[said.clone()]);
    seen = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(_flat(&[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }])); } } py_list(__out) })() };
    earlier = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&seen.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut heard: Py = __item.clone(); if py_truthy(&{ let __b = heard.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&heard.clone(), &mine.clone())) } }) {  __out.push(heard.clone()); } } py_list(__out) })() };
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); __out.push(one.clone()); } py_list(__out) })() };
    starts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5506 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk5506[0].clone(); let mut one: Py = __unpk5506[1].clone(); if py_truthy(&{ let __b = needle.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&mcall(&one.clone(), "lower", &[]), "strip", &[py_str(" .!?,")]), &needle.clone())) } }) {  __out.push(i.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&sentences.clone(), &if py_truthy(&starts.clone()) { add(&geti(&starts.clone(), &Py::Int(0)), &Py::Int(1)) } else { Py::Int(1) }, &Py::None, &Py::None));
    for __item in __iter1 {
    sentence = __item.clone();
    text = mcall(&mcall(&py_str(" "), "join", &[mcall(&sentence.clone(), "split", &[])]), "strip", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { _is_a_question(&[text.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = needle.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&text.clone(), "lower", &[]), &needle.clone())) } }) {
    continue;
    }
    if py_truthy(&mcall(&RED_FLAG.clone(), "match", &[text.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g_ASKS_THEM(), "match", &[text.clone()])) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&earlier.clone()) { let mut heard: Py = __item.clone(); __out.push(Py::Bool(py_contains(&heard.clone(), &mcall(&text.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    return text.clone();
    }
    return py_str("");
    Py::None
}
pub fn _step_sentence(args: &[Py]) -> Py {
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
    let mut step: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut needle: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The whole sentence the answer wrote the step in, or \"\" when it cannot be found again.\n\n    The step itself is the first clause of that sentence (\"Give them sun\"), which is the right\n    unit for the pools that add an order to it. A person who asks for the one thing is owed the\n    instruction, not a fragment of it: \"where do i start\" after the tomato answer was handed back\n    \"Just the one, then: Give them sun.\" and lost the pot, the watering and the reason, none of\n    which the person has read twice. So the whole sentence is quoted, and only the pools written\n    for a sentence of its own hand it over.\n    ");
    needle = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]), "strip", &[py_str(" .!?,")]);
    if py_truthy(&Py::Bool(!py_truthy(&needle.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    text = mcall(&py_str(" "), "join", &[mcall(&sentence.clone(), "split", &[])]);
    if py_truthy(&{ let __b = needle.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&text.clone(), "lower", &[]), &needle.clone())) } }) {
    return mcall(&text.clone(), "strip", &[py_str(" .!?")]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _up_to_a_break(args: &[Py]) -> Py {
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
    let mut step: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut within: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "within") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: within"))) };
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut start: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The longest run of the sentence that ends on one of the sentence's own breaks.\n\n    A person who asks for the one thing is owed the instruction, and one instruction can be a\n    sentence with a colon and a dash in it: \"Fill a freezer bag with white vinegar - half vinegar,\n    half water if the chrome worries you - tie it over the shower head so the nozzles soak for an\n    hour\" is one move from start to finish, and the moves after it (\"then rub them with a sponge\n    and poke any blocked holes with a pin\") are the next thing to do, not this one. So where the\n    sentence runs past what one instruction is worth (`within`), the longest piece of it that ends\n    where the answer itself breaks is handed over, and the tail is left for \"what else\".\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &within.clone())) {
    return said.clone();
    }
    opening = mcall(&mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "lower", &[]), "find", &[mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[])]);
    start = if py_truthy(&gt(&opening.clone(), &Py::Int(0))) { call_builtin("len", &[mcall(&slice_or_raise(&mcall(&py_str(" "), "join", &[words.clone()]), &Py::None, &opening.clone(), &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    if py_truthy(&{ let __b = start.clone(); if !py_truthy(&__b) { __b } else { lt(&start.clone(), &sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    said = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &start.clone(), &Py::None, &Py::None)]);
    words = mcall(&said.clone(), "split", &[]);
    if py_truthy(&le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &within.clone())) {
    return said.clone();
    }
    }
    best = py_str("");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("[,;:]|\\s[-\\u2013\\u2014]\\s"), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    head = mcall(&slice_or_raise(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;:-–—")]);
    if py_truthy(&{ let __b = le(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &within.clone()); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&best.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = head.clone();
    }
    }
    return { let __b = best.clone(); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &within.clone(), &Py::None)]) } };
    Py::None
}
pub fn _an_instruction_sentence(args: &[Py]) -> Py {
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
    let mut colon: Py = Py::None;
    let mut head: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The answer's own first instruction, for an answer `_step` could not read one out of.\n\n    Some answers write the move as a thing rather than a verb - \"A dentist or a hygienist scale\n    and polish first, because most yellowing is staining on the outside\" - or label it before a\n    colon (\"The thing to do first: write down what is actually due this week\"). `_step` finds\n    neither, and the one-thing ask was answered \"none of it is one job to start with\" about an\n    answer that plainly had one. Only a sentence with something imperative in it counts, so an\n    answer that really is all explanation still gets the line that says so.\n    ");
    let mut RED_FLAG: Py = crate::gen::memory::topics::g_RED_FLAG();
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    text = mcall(&py_str(" "), "join", &[mcall(&sentence.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CONDITIONAL_OPEN(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&RED_FLAG.clone(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { _is_a_question(&[text.clone()]) } } } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_STATEMENT(), "match", &[text.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_IMPERATIVES(), &_doing_word(&[slice_or_raise(&stdlib_call("re", "findall", &[py_str("[A-Za-z'’-]+"), text.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(5), &Py::None)])))) {
    return mcall(&text.clone(), "strip", &[py_str(" .!?")]);
    }
    let __unpk2 = py_unpack(&mcall(&text.clone(), "partition", &[py_str(": ")]), 3, None);
    head = __unpk2[0].clone();
    colon = __unpk2[1].clone();
    tail = __unpk2[2].clone();
    if py_truthy(&{ let __b = colon.clone(); if !py_truthy(&__b) { __b } else { { let __b = tail.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_IMPERATIVES(), &_doing_word(&[slice_or_raise(&stdlib_call("re", "findall", &[py_str("[A-Za-z'’-]+"), tail.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(5), &Py::None)]))) } } } }) {
    return mcall(&text.clone(), "strip", &[py_str(" .!?")]);
    }
    if py_truthy(&{ let __b = mcall(&g_FIRST_ITSELF(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_COPULA(), "match", &[text.clone()]))); if !py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } } } }) {
    return mcall(&text.clone(), "strip", &[py_str(" .!?")]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_answer_under_a_frame(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The answer under the last repeat frame in the chat, or \"\" if there is not one.\n\n    A repeat said by the store (\"Nothing separate for friday - it is the same advice: Rehearse the\n    first sixty seconds out loud ...\") is not a module the composer reads, so nothing was read at\n    all and the step ask after it got a search hedge. The frame is the store saying this is the\n    answer already on the screen, so the answer under it is the one to read - and only that: a\n    turn that is not framed is left for the store to answer, exactly as before.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return py_str("");
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&g_UNREADABLE(), &mcall(&last.clone(), "get", &[py_str("module")])))) {
    return py_str("");
    }
    return { let __b = crate::gen::memory::again::unframed(&[mcall(&last.clone(), "get", &[py_str("bot")])]); if py_truthy(&__b) { __b } else { py_str("") } };
    Py::None
}
pub fn _the_front_names_what_is_in_hand(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut in_hand: Py = Py::None;
    let mut part: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the subject of the clause in front of a colon is the thing this chat is about.\n\n    The words of the chat are the line itself and what the person and the bot have already said,\n    because a follow-up that names the job says \"how do i fix it\" and not the thing: the dripping\n    tap is in the first turn, not in the ask.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[A-Za-z'’-]+"), mcall(&call_builtin("str", &[{ let __b = head.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FRONT_WHEN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FRONT_THIS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    subject = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FRONT_LEAD(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, &Py::None, &g_FRONT_SUBJECT_WORDS(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(false);
    }
    in_hand = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z'’-]+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { py_tuple(vec![{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]) } else { py_tuple(vec![]) });
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&add(&py_list(vec![mcall(&part.clone(), "get", &[py_str("topic")])]), &call_builtin("list", &[{ let __b = mcall(&part.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter2 {
    topic = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&in_hand.clone(), "update", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&topic.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    }
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&part.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter3 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&in_hand.clone(), "update", &[stdlib_call("re", "findall", &[py_str("[a-z'’-]+"), mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    }
    let __iter4 = py_iter_or_raise(&subject.clone());
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&in_hand.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn next_step(args: &[Py]) -> Py {
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
    let mut asks_step: Py = Py::None;
    let mut asks_who: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut colon: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut head: Py = Py::None;
    let mut how_to: Py = Py::None;
    let mut how_to_asked: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut listed: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut other: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut plea: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut someone_new: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut step: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut tips: Py = Py::None;
    let mut when: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("A bare \"what should i do\" or \"should i see someone\", answered from the advice given.");
    flat = mcall(&_flat(&[clean.clone()]), "strip", &[py_str(" .!?")]);
    bare = call_builtin("bool", &[mcall(&g_STEP(), "match", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e));
    someone_new = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("someone_new")]) } else { Py::None };
    asks_step = { let __b = bare.clone(); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_SPEAK_UP(), "match", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&someone_new.clone())) } } } };
    asks_who = call_builtin("bool", &[mcall(&g_SEE(), "match", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e));
    plea = call_builtin("bool", &[{ let __b = mcall(&g_TIPS_AGAIN(), "match", &[flat.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ONE_THING(), "match", &[flat.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    tips = call_builtin("bool", &[mcall(&g_TIPS_AGAIN(), "match", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e));
    how_to = mcall(&g_HOW_TO(), "match", &[flat.clone()]);
    how_to_asked = { let __b = call_builtin("bool", &[how_to.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DOING_WITH_NO_OBJECT(), &mcall(&how_to.clone(), "group", &[py_str("doing")]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&how_to.clone(), "group", &[py_str("obj")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_KNOWING_DOING(), &mcall(&how_to.clone(), "group", &[py_str("doing")]))) } } } } } };
    if py_truthy(&how_to_asked.clone()) {
    asks_step = Py::Bool(true);
    }
    if py_truthy(&{ let __b = plea.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_last_module(&[memory.clone()]), &py_str("answers"))) } }) {
    asks_step = Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_A_HOW_ASK(), "match", &[flat.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_last_module(&[memory.clone()]), &py_str("answers"))) } }) {
    asks_step = Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = asks_step.clone(); if py_truthy(&__b) { __b } else { asks_who.clone() } }))) {
    return Py::None;
    }
    said = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("past_the_verdict".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_reply(args))(&__args) }; if py_truthy(&__b) { __b } else { _the_answer_under_a_frame(&[memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_is(&_first(&[said.clone(), g_AGAIN()]), &Py::None))) {
    said = crate::gen::memory::again::unframed(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    }
    if py_truthy(&mcall(&g_EMERGENCY_CALL(), "match", &[mcall(&said.clone(), "strip", &[])])) {
    return Py::None;
    }
    when = _when(&[said.clone()]);
    if py_truthy(&asks_who.clone()) {
    if py_truthy(&when.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SEE_NOW_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("when".to_string(), when.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    let __unpk1 = py_unpack(&_stated_referral(&[said.clone()]), 2, None);
    stated = __unpk1[0].clone();
    kind = __unpk1[1].clone();
    if py_truthy(&stated.clone()) {
    pool = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("clause")))) { g_SEE_NOW_LINES() } else { g_SEE_STATED_LINES() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("when".to_string(), stated.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    if py_truthy(&mcall(&g_REFER(), "search", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_SEE_ANYWAY_LINES(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&_first(&[said.clone(), g_CARE()]), &Py::None))) {
    return crate::gen::compose::pick(&[g_SEE_NONE_LINES(), memory.clone(), flat.clone()]);
    }
    return Py::None;
    }
    step = _step(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if !py_truthy(&__b) { __b } else { _ends_on_a_question(&[said.clone()]) } }) {
    step = _step(&[_the_answer_before_that(&[memory.clone()])]);
    }
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { _echoes_the_person(&[step.clone(), memory.clone()]) } }) {
    step = Py::None;
    }
    if py_truthy(&tips.clone()) {
    other = _the_other_thing_in_the_answer(&[said.clone(), step.clone(), memory.clone()]);
    if py_truthy(&other.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ALSO_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("other".to_string(), other.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return crate::gen::compose::pick(&[g_STEP_KEPT_LINES(), memory.clone(), flat.clone()]);
    }
    }
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { { let __b = _already_asked_for_a_step(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _most_of_the_line(&[step.clone(), _the_line_just_given(&[memory.clone()])]); if py_truthy(&__b) { __b } else { tips.clone() } } } } } }) {
    return crate::gen::compose::pick(&[g_STEP_KEPT_LINES(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    if py_truthy(&_an_offer_is_waiting(&[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { _ends_on_a_question(&[said.clone()]) } }) {
    if py_truthy(&_the_news_just_before(&[memory.clone()])) {
    return Py::None;
    }
    asked = _the_question_asked(&[said.clone()]);
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_THE_PLEA(), "match", &[flat.clone()]) } }) {
    return crate::gen::compose::pick(&[g_QUESTION_BACK_LINES(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { _after_a_comfort_line(&[memory.clone()]) } }) {
    return crate::gen::compose::pick(&[g_QUESTION_BACK_LINES(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_AN_OFFER(), "search", &[asked.clone()]) } }) {
    return crate::gen::compose::pick(&[g_OFFER_TAKEN_LINES(), memory.clone(), flat.clone()]);
    }
    if py_truthy(&asked.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ASKED_FIRST_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("q".to_string(), asked.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    return crate::gen::compose::pick(&[g_QUESTION_BACK_LINES(), memory.clone(), flat.clone()]);
    }
    listed = _listed(&[said.clone()]);
    if py_truthy(&listed.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LIST_STEP_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), listed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    if py_truthy(&{ let __b = when.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&plea.clone())); if !py_truthy(&__b) { __b } else { lt(&_action_count(&[said.clone()]), &Py::Int(2)) } } } }) {
    return mcall(&crate::gen::compose::pick(&[g_NOTHING_LINES(), memory.clone(), flat.clone()]), "replace", &[py_str("{when}"), when.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_ONE_THING(), "match", &[flat.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_last_module(&[memory.clone()]), &py_str("answers"))) } }) {
    move_ = _an_instruction_sentence(&[said.clone()]);
    if py_truthy(&move_.clone()) {
    head = mcall(&geti(&mcall(&move_.clone(), "partition", &[py_str(": ")]), &Py::Int(0)), "strip", &[py_str(" .;:,-")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&move_.clone(), &py_str(": "))); if !py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    move_ = head.clone();
    } else {
    if py_truthy(&gt(&call_builtin("len", &[mcall(&move_.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_HAND_OVER_WORDS())) {
    move_ = { let __b = mcall(&_first_clause(&[move_.clone()]), "strip", &[py_str(" .;:,-")]); if py_truthy(&__b) { __b } else { move_.clone() } };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ONE_TURN_PLAIN_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), move_.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    return _say(&[g_ALL_OF_IT_LINES(), memory.clone(), flat.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&mcall(&mcall(&step.clone(), "lower", &[]), "endswith", &[py_str(" first")])) {
    step = mcall(&slice_or_raise(&step.clone(), &Py::None, &neg(&call_builtin("len", &[py_str(" first")]).unwrap_or_else(|e| exc::raise(e))), &Py::None), "rstrip", &[]);
    }
    if py_truthy(&mcall(&g_ONE_THING(), "match", &[flat.clone()])) {
    whole = _step_sentence(&[said.clone(), step.clone()]);
    piece = if py_truthy(&whole.clone()) { _up_to_a_break(&[whole.clone(), step.clone(), g_HAND_OVER_WORDS()]) } else { py_str("") };
    if py_truthy(&gt(&call_builtin("len", &[mcall(&piece.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ONE_TURN_PLAIN_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), piece.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    pool = if py_truthy(&{ let __b = Py::Bool(py_contains(&step.clone(), &py_str(": "))); if py_truthy(&__b) { __b } else { mcall(&g_FIRST_ITSELF(), "search", &[step.clone()]) } }) { g_ONE_TURN_PLAIN_LINES() } else { g_ONE_TURN_LINES() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    if py_truthy(&how_to_asked.clone()) {
    whole = _step_sentence(&[said.clone(), step.clone()]);
    if py_truthy(&{ let __b = whole.clone(); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&whole.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    step = whole.clone();
    }
    let __unpk2 = py_unpack(&mcall(&step.clone(), "partition", &[py_str(": ")]), 3, None);
    head = __unpk2[0].clone();
    colon = __unpk2[1].clone();
    tail = __unpk2[2].clone();
    if py_truthy(&{ let __b = colon.clone(); if !py_truthy(&__b) { __b } else { { let __b = tail.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_IMPERATIVES(), &_doing_word(&[stdlib_call("re", "findall", &[py_str("[A-Za-z'’-]+"), head.clone()]).unwrap_or_else(|e| exc::raise(e))]))); if !py_truthy(&__b) { __b } else { _the_front_names_what_is_in_hand(&[head.clone(), flat.clone(), memory.clone()]) } } } } } }) {
    step = mcall(&tail.clone(), "strip", &[]);
    }
    }
    if py_truthy(&mcall(&g_FIRST_ITSELF(), "search", &[step.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_STEP_ALREADY_FIRST_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&step.clone(), &py_str(": "))); if py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_LONG()) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_STEP_DETAIL_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    if py_truthy(&gt(&_action_count(&[said.clone()]), &Py::Int(1))) {
    pool = if py_truthy(&mcall(&g_FIRST_ITSELF(), "search", &[step.clone()])) { g_STEP_ALREADY_FIRST_LINES() } else { g_STEP_LINES() };
    if py_truthy(&how_to_asked.clone()) {
    pool = g_STEP_DETAIL_LINES();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_STEP_ONE_LINES()); __args.push(memory.clone()); __args.push(flat.clone()); __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.verdict".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_TALK" => Some(g_ABOUT_THE_TALK()),
            "ADVICE_OK_LINES" => Some(g_ADVICE_OK_LINES()),
            "AFTER" => Some(g_AFTER()),
            "AGAIN" => Some(g_AGAIN()),
            "AGE_LINES" => Some(g_AGE_LINES()),
            "ALL_OF_IT_LINES" => Some(g_ALL_OF_IT_LINES()),
            "ALSO_LINES" => Some(g_ALSO_LINES()),
            "AN_OFFER" => Some(g_AN_OFFER()),
            "ASKED_FIRST_LINES" => Some(g_ASKED_FIRST_LINES()),
            "ASKED_FOR_A_CALL" => Some(g_ASKED_FOR_A_CALL()),
            "ASKED_THEM_LINES" => Some(g_ASKED_THEM_LINES()),
            "ASKING_HOW_TO_TELL" => Some(g_ASKING_HOW_TO_TELL()),
            "ASKS_FINE" => Some(g_ASKS_FINE()),
            "ASKS_NORMAL" => Some(g_ASKS_NORMAL()),
            "ASKS_OK" => Some(g_ASKS_OK()),
            "ASKS_THEM" => Some(g_ASKS_THEM()),
            "ASKS_WRONG" => Some(g_ASKS_WRONG()),
            "A_BODY_IN_THE_LINE" => Some(g_A_BODY_IN_THE_LINE()),
            "A_BODY_WORD" => Some(g_A_BODY_WORD()),
            "A_CHECK_ASK" => Some(g_A_CHECK_ASK()),
            "A_CHECK_ON_A_PLAN" => Some(g_A_CHECK_ON_A_PLAN()),
            "A_CHECK_ON_A_PLAN_LINES" => Some(g_A_CHECK_ON_A_PLAN_LINES()),
            "A_COMPARISON" => Some(g_A_COMPARISON()),
            "A_DECISION_ASK" => Some(g_A_DECISION_ASK()),
            "A_DOUBT_ABOUT_THE_THING" => Some(g_A_DOUBT_ABOUT_THE_THING()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_FIGURE_ASK" => Some(g_A_FIGURE_ASK()),
            "A_FREQUENCY_ASKED" => Some(g_A_FREQUENCY_ASKED()),
            "A_FREQUENCY_SAID" => Some(g_A_FREQUENCY_SAID()),
            "A_HOW_ASK" => Some(g_A_HOW_ASK()),
            "A_HOW_TO_ASK" => Some(g_A_HOW_TO_ASK()),
            "A_KEEP_OR_STOP" => Some(g_A_KEEP_OR_STOP()),
            "A_LENGTH_IN_THE_ANSWER" => Some(g_A_LENGTH_IN_THE_ANSWER()),
            "A_PERSON_THEY_KNOW" => Some(g_A_PERSON_THEY_KNOW()),
            "A_PLAN" => Some(g_A_PLAN()),
            "A_PLAN_DOUBT_LINES" => Some(g_A_PLAN_DOUBT_LINES()),
            "A_REFUSAL" => Some(g_A_REFUSAL()),
            "A_RELATION_TROUBLE" => Some(g_A_RELATION_TROUBLE()),
            "A_STATEMENT" => Some(g_A_STATEMENT()),
            "A_SYMPTOM_WORD" => Some(g_A_SYMPTOM_WORD()),
            "A_THING_THEY_OWN" => Some(g_A_THING_THEY_OWN()),
            "A_THINKING_ABOUT" => Some(g_A_THINKING_ABOUT()),
            "A_WAIT_IN_THE_STEP" => Some(g_A_WAIT_IN_THE_STEP()),
            "A_WORRY_IN_THE_LINE" => Some(g_A_WORRY_IN_THE_LINE()),
            "BACKSTOP_LINES" => Some(g_BACKSTOP_LINES()),
            "BARE" => Some(g_BARE()),
            "BODY_WORD" => Some(g_BODY_WORD()),
            "BOTH_LINES" => Some(g_BOTH_LINES()),
            "CALLS_FOR" => Some(g_CALLS_FOR()),
            "CAN_HURT" => Some(g_CAN_HURT()),
            "CARE" => Some(g_CARE()),
            "CARE_NORMAL" => Some(g_CARE_NORMAL()),
            "CARE_NORMAL_UNKNOWABLE" => Some(g_CARE_NORMAL_UNKNOWABLE()),
            "CARE_NOW" => Some(g_CARE_NOW()),
            "CARE_NOW_NORMAL" => Some(g_CARE_NOW_NORMAL()),
            "CHECK_JOB_LINES" => Some(g_CHECK_JOB_LINES()),
            "CHECK_ONCE_LINES" => Some(g_CHECK_ONCE_LINES()),
            "CHECK_WATCH_LINES" => Some(g_CHECK_WATCH_LINES()),
            "CHECK_WORKS_LINES" => Some(g_CHECK_WORKS_LINES()),
            "CHOICE_OK_LINES" => Some(g_CHOICE_OK_LINES()),
            "COMPARATIVE" => Some(g_COMPARATIVE()),
            "COMPOSED" => Some(g_COMPOSED()),
            "CONDITIONAL" => Some(g_CONDITIONAL()),
            "CONDITIONAL_AFTER" => Some(g_CONDITIONAL_AFTER()),
            "CONDITIONAL_OPEN" => Some(g_CONDITIONAL_OPEN()),
            "CONDITION_CLAUSE" => Some(g_CONDITION_CLAUSE()),
            "CONDITION_HANGING" => Some(g_CONDITION_HANGING()),
            "CONDITION_LINES" => Some(g_CONDITION_LINES()),
            "CONDITION_NORMAL" => Some(g_CONDITION_NORMAL()),
            "CONDITION_VERBS" => Some(g_CONDITION_VERBS()),
            "CONSIDERING" => Some(g_CONSIDERING()),
            "CONTINUES" => Some(g_CONTINUES()),
            "CONTRAST" => Some(g_CONTRAST()),
            "COPULA" => Some(g_COPULA()),
            "DANGER" => Some(g_DANGER()),
            "DECLARATIVE_REFERRAL" => Some(g_DECLARATIVE_REFERRAL()),
            "DENIES" => Some(g_DENIES()),
            "DESCRIBES" => Some(g_DESCRIBES()),
            "DETAIL_STATEMENT" => Some(g_DETAIL_STATEMENT()),
            "DIRECTED_LINES" => Some(g_DIRECTED_LINES()),
            "DOING" => Some(g_DOING()),
            "DOING_WITH_NO_OBJECT" => Some(g_DOING_WITH_NO_OBJECT()),
            "DO_LINES" => Some(g_DO_LINES()),
            "DO_NORMAL" => Some(g_DO_NORMAL()),
            "ECHO_FILLER" => Some(g_ECHO_FILLER()),
            "ECHO_WORDS" => Some(g_ECHO_WORDS()),
            "EMERGENCY_CALL" => Some(g_EMERGENCY_CALL()),
            "EMERGENCY_CALL_OPEN" => Some(g_EMERGENCY_CALL_OPEN()),
            "EMERGENCY_ORDER" => Some(g_EMERGENCY_ORDER()),
            "ENOUGH_ASK" => Some(g_ENOUGH_ASK()),
            "ENOUGH_STATED_LINES" => Some(g_ENOUGH_STATED_LINES()),
            "EXPLANATION_WORDS" => Some(g_EXPLANATION_WORDS()),
            "FEELING_DOUBT_LINES" => Some(g_FEELING_DOUBT_LINES()),
            "FEELING_FAMILY" => Some(g_FEELING_FAMILY()),
            "FENCED_IN_SENTENCE" => Some(g_FENCED_IN_SENTENCE()),
            "FENCE_FILLER" => Some(g_FENCE_FILLER()),
            "FIGURE_LINES" => Some(g_FIGURE_LINES()),
            "FIRST_ITSELF" => Some(g_FIRST_ITSELF()),
            "FIX_ASKED_LINES" => Some(g_FIX_ASKED_LINES()),
            "FIX_ASKED_NOW_LINES" => Some(g_FIX_ASKED_NOW_LINES()),
            "FLOOR_LINES" => Some(g_FLOOR_LINES()),
            "FRAME_AND" => Some(g_FRAME_AND()),
            "FRONT_LEAD" => Some(g_FRONT_LEAD()),
            "FRONT_SUBJECT_WORDS" => Some(g_FRONT_SUBJECT_WORDS()),
            "FRONT_THIS" => Some(g_FRONT_THIS()),
            "FRONT_WHEN" => Some(g_FRONT_WHEN()),
            "GO_FROM" => Some(g_GO_FROM()),
            "GRIEF_FLOOR_LINES" => Some(g_GRIEF_FLOOR_LINES()),
            "HANDED_BACK_LINES" => Some(g_HANDED_BACK_LINES()),
            "HAND_OVER_WORDS" => Some(g_HAND_OVER_WORDS()),
            "HANGING" => Some(g_HANGING()),
            "HARD_NEWS" => Some(g_HARD_NEWS()),
            "HARD_NEWS_LINES" => Some(g_HARD_NEWS_LINES()),
            "HARMLESS" => Some(g_HARMLESS()),
            "HEAD_REFUSES" => Some(g_HEAD_REFUSES()),
            "HEDGE_SPAN" => Some(g_HEDGE_SPAN()),
            "HELPED" => Some(g_HELPED()),
            "HELPED_LINES" => Some(g_HELPED_LINES()),
            "HELPS" => Some(g_HELPS()),
            "HOW_MANY_PEOPLE" => Some(g_HOW_MANY_PEOPLE()),
            "HOW_TO" => Some(g_HOW_TO()),
            "IMPERATIVES" => Some(g_IMPERATIVES()),
            "IS_A_NUMBER" => Some(g_IS_A_NUMBER()),
            "JUDGED_REFERRAL" => Some(g_JUDGED_REFERRAL()),
            "JUST_AGE" => Some(g_JUST_AGE()),
            "KNOWING_DOING" => Some(g_KNOWING_DOING()),
            "LATER_CONDITION" => Some(g_LATER_CONDITION()),
            "LIST_STEP_LINES" => Some(g_LIST_STEP_LINES()),
            "LONG_SPAN" => Some(g_LONG_SPAN()),
            "LONG_STEP_WORDS" => Some(g_LONG_STEP_WORDS()),
            "LOOK_AND_SEE" => Some(g_LOOK_AND_SEE()),
            "LOSS_WORDS" => Some(g_LOSS_WORDS()),
            "METHOD_DOUBT" => Some(g_METHOD_DOUBT()),
            "METHOD_OK_LINES" => Some(g_METHOD_OK_LINES()),
            "MET_CONDITION_LINES" => Some(g_MET_CONDITION_LINES()),
            "MET_LINES" => Some(g_MET_LINES()),
            "MONEY_LINES" => Some(g_MONEY_LINES()),
            "MONEY_NOT_MET" => Some(g_MONEY_NOT_MET()),
            "MONEY_NOUN_WORDS" => Some(g_MONEY_NOUN_WORDS()),
            "MONEY_TROUBLE_LINES" => Some(g_MONEY_TROUBLE_LINES()),
            "MONEY_WORD" => Some(g_MONEY_WORD()),
            "MOOD" => Some(g_MOOD()),
            "NAMES_A_CLINICIAN" => Some(g_NAMES_A_CLINICIAN()),
            "NAMES_A_TRADE" => Some(g_NAMES_A_TRADE()),
            "NAMES_NOTHING" => Some(g_NAMES_NOTHING()),
            "NEEDS_A_CALL" => Some(g_NEEDS_A_CALL()),
            "NEGATED" => Some(g_NEGATED()),
            "NEGATED_BEFORE" => Some(g_NEGATED_BEFORE()),
            "NEGATIVE_OPEN" => Some(g_NEGATIVE_OPEN()),
            "NEUTRAL_ASK_LINES" => Some(g_NEUTRAL_ASK_LINES()),
            "NOTHING_LINES" => Some(g_NOTHING_LINES()),
            "NOTHING_WARNED_LINES" => Some(g_NOTHING_WARNED_LINES()),
            "NOTHING_WARNED_NORMAL" => Some(g_NOTHING_WARNED_NORMAL()),
            "NOT_A_STEP_TO_WATCH" => Some(g_NOT_A_STEP_TO_WATCH()),
            "NOT_A_STEP_TO_WATCH_WORDS" => Some(g_NOT_A_STEP_TO_WATCH_WORDS()),
            "NOT_MONEY_PAY" => Some(g_NOT_MONEY_PAY()),
            "NOT_TOLD_TO" => Some(g_NOT_TOLD_TO()),
            "NO_LINES" => Some(g_NO_LINES()),
            "NO_STATED_LINES" => Some(g_NO_STATED_LINES()),
            "OFFER_QUESTION" => Some(g_OFFER_QUESTION()),
            "OFFER_TAKEN_LINES" => Some(g_OFFER_TAKEN_LINES()),
            "OK_LINES" => Some(g_OK_LINES()),
            "ONE_OPTION" => Some(g_ONE_OPTION()),
            "ONE_THING" => Some(g_ONE_THING()),
            "ONE_THING_ADVERB" => Some(g_ONE_THING_ADVERB()),
            "ONE_TURN_LINES" => Some(g_ONE_TURN_LINES()),
            "ONE_TURN_PLAIN_LINES" => Some(g_ONE_TURN_PLAIN_LINES()),
            "ORDER_CONTINUES" => Some(g_ORDER_CONTINUES()),
            "ORDINARY_NORMAL" => Some(g_ORDINARY_NORMAL()),
            "OVER_VERB" => Some(g_OVER_VERB()),
            "OWED" => Some(g_OWED()),
            "OWED_LINES" => Some(g_OWED_LINES()),
            "OWN_DOING" => Some(g_OWN_DOING()),
            "OWN_LINES" => Some(g_OWN_LINES()),
            "OWN_PATTERN" => Some(g_OWN_PATTERN()),
            "PEOPLE_CARE" => Some(g_PEOPLE_CARE()),
            "PERSON_CALL" => Some(g_PERSON_CALL()),
            "PERSON_ORDINARY" => Some(g_PERSON_ORDINARY()),
            "POOLS" => Some(g_POOLS()),
            "PUNT_CHAT" => Some(g_PUNT_CHAT()),
            "PUNT_OFFER" => Some(g_PUNT_OFFER()),
            "PUNT_START" => Some(g_PUNT_START()),
            "QUESTION_BACK_LINES" => Some(g_QUESTION_BACK_LINES()),
            "QUESTION_FRAME" => Some(g_QUESTION_FRAME()),
            "READABLE" => Some(g_READABLE()),
            "REASSURE" => Some(g_REASSURE()),
            "RECAP_FILLER" => Some(g_RECAP_FILLER()),
            "REFER" => Some(g_REFER()),
            "REFER_VERB" => Some(g_REFER_VERB()),
            "RELATIVE" => Some(g_RELATIVE()),
            "RESTATED" => Some(g_RESTATED()),
            "RULED_BY_THE_COMPOSER" => Some(g_RULED_BY_THE_COMPOSER()),
            "RUN" => Some(g_RUN()),
            "SAID_YES" => Some(g_SAID_YES()),
            "SAYS_NORMAL" => Some(g_SAYS_NORMAL()),
            "SEE" => Some(g_SEE()),
            "SEE_ANYWAY_LINES" => Some(g_SEE_ANYWAY_LINES()),
            "SEE_NONE_LINES" => Some(g_SEE_NONE_LINES()),
            "SEE_NOW_LINES" => Some(g_SEE_NOW_LINES()),
            "SEE_STATED_LINES" => Some(g_SEE_STATED_LINES()),
            "SIGN" => Some(g_SIGN()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "SOON_LINES" => Some(g_SOON_LINES()),
            "SPEAK_UP" => Some(g_SPEAK_UP()),
            "SPEAK_VERBS" => Some(g_SPEAK_VERBS()),
            "SPLIT" => Some(g_SPLIT()),
            "STATED_ENOUGH" => Some(g_STATED_ENOUGH()),
            "STATED_LONGEST" => Some(g_STATED_LONGEST()),
            "STATED_NO" => Some(g_STATED_NO()),
            "STATES_A_REFERRAL" => Some(g_STATES_A_REFERRAL()),
            "STEP" => Some(g_STEP()),
            "STEP_ALREADY_FIRST_LINES" => Some(g_STEP_ALREADY_FIRST_LINES()),
            "STEP_DETAIL_LINES" => Some(g_STEP_DETAIL_LINES()),
            "STEP_KEPT_LINES" => Some(g_STEP_KEPT_LINES()),
            "STEP_LINES" => Some(g_STEP_LINES()),
            "STEP_LONG" => Some(g_STEP_LONG()),
            "STEP_ONE_LINES" => Some(g_STEP_ONE_LINES()),
            "STEP_WORDS" => Some(g_STEP_WORDS()),
            "STILL_GOING" => Some(g_STILL_GOING()),
            "STOP_DOING" => Some(g_STOP_DOING()),
            "SUBJECT_OPEN" => Some(g_SUBJECT_OPEN()),
            "SYMPATHY" => Some(g_SYMPATHY()),
            "TARGET_SHAPE" => Some(g_TARGET_SHAPE()),
            "TESTING" => Some(g_TESTING()),
            "TEST_LINES" => Some(g_TEST_LINES()),
            "THEN_OPEN" => Some(g_THEN_OPEN()),
            "THE_PLAN_CAN_HURT_LINES" => Some(g_THE_PLAN_CAN_HURT_LINES()),
            "THE_PLEA" => Some(g_THE_PLEA()),
            "THING_CALL" => Some(g_THING_CALL()),
            "THING_ORDINARY" => Some(g_THING_ORDINARY()),
            "THING_WORDS" => Some(g_THING_WORDS()),
            "THROW_AWAY" => Some(g_THROW_AWAY()),
            "THROW_AWAY_LINES" => Some(g_THROW_AWAY_LINES()),
            "TIME_NOW" => Some(g_TIME_NOW()),
            "TIPS_AGAIN" => Some(g_TIPS_AGAIN()),
            "TRADE" => Some(g_TRADE()),
            "TRIM" => Some(g_TRIM()),
            "TRIM_INSTRUMENT" => Some(g_TRIM_INSTRUMENT()),
            "UNREADABLE" => Some(g_UNREADABLE()),
            "URGENT_CALL" => Some(g_URGENT_CALL()),
            "URGENT_TIME" => Some(g_URGENT_TIME()),
            "WHEN" => Some(g_WHEN()),
            "WHEN_ASK" => Some(g_WHEN_ASK()),
            "WHEN_CALL_LINES" => Some(g_WHEN_CALL_LINES()),
            "WHEN_LINES" => Some(g_WHEN_LINES()),
            "WORRIED_ASK" => Some(g_WORRIED_ASK()),
            "WORRYING_ASK" => Some(g_WORRYING_ASK()),
            "YES_LINES" => Some(g_YES_LINES()),
            "YES_SAID_LINES" => Some(g_YES_SAID_LINES()),
            "_ACK_PATTERN" => Some(g__ACK_PATTERN()),
            "_AFTER_A_NOUN" => Some(g__AFTER_A_NOUN()),
            "_AN_OFFER" => Some(g__AN_OFFER()),
            "_AN_OPENER" => Some(g__AN_OPENER()),
            "_ASKS_ABOUT_A_STEP" => Some(g__ASKS_ABOUT_A_STEP()),
            "_ASKS_FOR_THE_STEP" => Some(g__ASKS_FOR_THE_STEP()),
            "_A_BODY_WORD" => Some(g__A_BODY_WORD()),
            "_A_DASH" => Some(g__A_DASH()),
            "_A_FIX_WORD" => Some(g__A_FIX_WORD()),
            "_A_NUMBER" => Some(g__A_NUMBER()),
            "_A_RANGE_OF_ITS_OWN" => Some(g__A_RANGE_OF_ITS_OWN()),
            "_A_REFERRAL_OUT" => Some(g__A_REFERRAL_OUT()),
            "_A_STOP" => Some(g__A_STOP()),
            "_BODY_PARTS" => Some(g__BODY_PARTS()),
            "_FILL" => Some(g__FILL()),
            "_FILLER" => Some(g__FILLER()),
            "_INVITATION" => Some(g__INVITATION()),
            "_LEAD_FILLER" => Some(g__LEAD_FILLER()),
            "_LIST_JOIN" => Some(g__LIST_JOIN()),
            "_NOT_A_SIGN_WORD" => Some(g__NOT_A_SIGN_WORD()),
            "_ONE_THING_ADVERB" => Some(g__ONE_THING_ADVERB()),
            "_STEP_OPENERS" => Some(g__STEP_OPENERS()),
            "_SYMPTOM_WORDS" => Some(g__SYMPTOM_WORDS()),
            "_WORD" => Some(g__WORD()),
            "_a_call_for_now" => Some(py_func("_a_call_for_now", |args: &[Py]| _a_call_for_now(args))),
            "_a_check_on_the_method" => Some(py_func("_a_check_on_the_method", |args: &[Py]| _a_check_on_the_method(args))),
            "_a_described_condition" => Some(py_func("_a_described_condition", |args: &[Py]| _a_described_condition(args))),
            "_a_doubt_about_money" => Some(py_func("_a_doubt_about_money", |args: &[Py]| _a_doubt_about_money(args))),
            "_a_feeling_of_their_own" => Some(py_func("_a_feeling_of_their_own", |args: &[Py]| _a_feeling_of_their_own(args))),
            "_a_figure_owed" => Some(py_func("_a_figure_owed", |args: &[Py]| _a_figure_owed(args))),
            "_a_fix_with_no_warning" => Some(py_func("_a_fix_with_no_warning", |args: &[Py]| _a_fix_with_no_warning(args))),
            "_a_frequency_already_said" => Some(py_func("_a_frequency_already_said", |args: &[Py]| _a_frequency_already_said(args))),
            "_a_how_to" => Some(py_func("_a_how_to", |args: &[Py]| _a_how_to(args))),
            "_a_job_somewhere" => Some(py_func("_a_job_somewhere", |args: &[Py]| _a_job_somewhere(args))),
            "_a_job_to_do" => Some(py_func("_a_job_to_do", |args: &[Py]| _a_job_to_do(args))),
            "_a_judgement_fence" => Some(py_func("_a_judgement_fence", |args: &[Py]| _a_judgement_fence(args))),
            "_a_mood" => Some(py_func("_a_mood", |args: &[Py]| _a_mood(args))),
            "_a_person_they_know" => Some(py_func("_a_person_they_know", |args: &[Py]| _a_person_they_know(args))),
            "_a_plain_go" => Some(py_func("_a_plain_go", |args: &[Py]| _a_plain_go(args))),
            "_a_plan_doubt" => Some(py_func("_a_plan_doubt", |args: &[Py]| _a_plan_doubt(args))),
            "_a_plan_or_a_choice" => Some(py_func("_a_plan_or_a_choice", |args: &[Py]| _a_plan_or_a_choice(args))),
            "_a_punt" => Some(py_func("_a_punt", |args: &[Py]| _a_punt(args))),
            "_a_reading_of_the_feeling" => Some(py_func("_a_reading_of_the_feeling", |args: &[Py]| _a_reading_of_the_feeling(args))),
            "_a_report_line" => Some(py_func("_a_report_line", |args: &[Py]| _a_report_line(args))),
            "_a_thing_stopped" => Some(g__a_thing_stopped()),
            "_a_thing_they_own" => Some(py_func("_a_thing_they_own", |args: &[Py]| _a_thing_they_own(args))),
            "_a_throw_away" => Some(py_func("_a_throw_away", |args: &[Py]| _a_throw_away(args))),
            "_about_a_body" => Some(py_func("_about_a_body", |args: &[Py]| _about_a_body(args))),
            "_about_a_death" => Some(py_func("_about_a_death", |args: &[Py]| _about_a_death(args))),
            "_about_the_person" => Some(py_func("_about_the_person", |args: &[Py]| _about_the_person(args))),
            "_action_count" => Some(py_func("_action_count", |args: &[Py]| _action_count(args))),
            "_after_a_comfort_line" => Some(py_func("_after_a_comfort_line", |args: &[Py]| _after_a_comfort_line(args))),
            "_after_the_if" => Some(py_func("_after_the_if", |args: &[Py]| _after_the_if(args))),
            "_already_asked_for_a_step" => Some(py_func("_already_asked_for_a_step", |args: &[Py]| _already_asked_for_a_step(args))),
            "_an_amount_asked" => Some(py_func("_an_amount_asked", |args: &[Py]| _an_amount_asked(args))),
            "_an_ask_for_a_call" => Some(py_func("_an_ask_for_a_call", |args: &[Py]| _an_ask_for_a_call(args))),
            "_an_event_that_is_over" => Some(py_func("_an_event_that_is_over", |args: &[Py]| _an_event_that_is_over(args))),
            "_an_if_that_is_a_condition" => Some(py_func("_an_if_that_is_a_condition", |args: &[Py]| _an_if_that_is_a_condition(args))),
            "_an_instruction_and_nothing_else" => Some(py_func("_an_instruction_and_nothing_else", |args: &[Py]| _an_instruction_and_nothing_else(args))),
            "_an_instruction_sentence" => Some(py_func("_an_instruction_sentence", |args: &[Py]| _an_instruction_sentence(args))),
            "_an_offer_is_waiting" => Some(py_func("_an_offer_is_waiting", |args: &[Py]| _an_offer_is_waiting(args))),
            "_as_written" => Some(py_func("_as_written", |args: &[Py]| _as_written(args))),
            "_behind_a_description" => Some(py_func("_behind_a_description", |args: &[Py]| _behind_a_description(args))),
            "_care_for_now" => Some(py_func("_care_for_now", |args: &[Py]| _care_for_now(args))),
            "_care_pool" => Some(py_func("_care_pool", |args: &[Py]| _care_pool(args))),
            "_condition_after_care" => Some(py_func("_condition_after_care", |args: &[Py]| _condition_after_care(args))),
            "_considering" => Some(py_func("_considering", |args: &[Py]| _considering(args))),
            "_cut_at_a_join" => Some(py_func("_cut_at_a_join", |args: &[Py]| _cut_at_a_join(args))),
            "_cut_condition" => Some(py_func("_cut_condition", |args: &[Py]| _cut_condition(args))),
            "_cut_on" => Some(py_func("_cut_on", |args: &[Py]| _cut_on(args))),
            "_danger" => Some(py_func("_danger", |args: &[Py]| _danger(args))),
            "_described_clause" => Some(py_func("_described_clause", |args: &[Py]| _described_clause(args))),
            "_doing_word" => Some(py_func("_doing_word", |args: &[Py]| _doing_word(args))),
            "_echoes_the_person" => Some(py_func("_echoes_the_person", |args: &[Py]| _echoes_the_person(args))),
            "_ends_on_a_question" => Some(py_func("_ends_on_a_question", |args: &[Py]| _ends_on_a_question(args))),
            "_first" => Some(py_func("_first", |args: &[Py]| _first(args))),
            "_first_clause" => Some(py_func("_first_clause", |args: &[Py]| _first_clause(args))),
            "_first_positive" => Some(py_func("_first_positive", |args: &[Py]| _first_positive(args))),
            "_first_sentence_end" => Some(py_func("_first_sentence_end", |args: &[Py]| _first_sentence_end(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_good_news" => Some(py_func("_good_news", |args: &[Py]| _good_news(args))),
            "_in_a_described_clause" => Some(py_func("_in_a_described_clause", |args: &[Py]| _in_a_described_clause(args))),
            "_is_a_comparison" => Some(py_func("_is_a_comparison", |args: &[Py]| _is_a_comparison(args))),
            "_is_a_question" => Some(py_func("_is_a_question", |args: &[Py]| _is_a_question(args))),
            "_is_a_recap" => Some(py_func("_is_a_recap", |args: &[Py]| _is_a_recap(args))),
            "_is_an_echo" => Some(py_func("_is_an_echo", |args: &[Py]| _is_an_echo(args))),
            "_is_an_explanation" => Some(py_func("_is_an_explanation", |args: &[Py]| _is_an_explanation(args))),
            "_kept_step" => Some(py_func("_kept_step", |args: &[Py]| _kept_step(args))),
            "_last_line_that_named_a_call" => Some(py_func("_last_line_that_named_a_call", |args: &[Py]| _last_line_that_named_a_call(args))),
            "_last_module" => Some(py_func("_last_module", |args: &[Py]| _last_module(args))),
            "_last_person_line" => Some(py_func("_last_person_line", |args: &[Py]| _last_person_line(args))),
            "_last_reply" => Some(py_func("_last_reply", |args: &[Py]| _last_reply(args))),
            "_listed" => Some(py_func("_listed", |args: &[Py]| _listed(args))),
            "_most_of_the_line" => Some(py_func("_most_of_the_line", |args: &[Py]| _most_of_the_line(args))),
            "_no_hanging_end" => Some(py_func("_no_hanging_end", |args: &[Py]| _no_hanging_end(args))),
            "_not_fenced" => Some(py_func("_not_fenced", |args: &[Py]| _not_fenced(args))),
            "_nothing_to_be_long_about" => Some(py_func("_nothing_to_be_long_about", |args: &[Py]| _nothing_to_be_long_about(args))),
            "_only_a_verdict" => Some(py_func("_only_a_verdict", |args: &[Py]| _only_a_verdict(args))),
            "_opening" => Some(py_func("_opening", |args: &[Py]| _opening(args))),
            "_own_doing" => Some(py_func("_own_doing", |args: &[Py]| _own_doing(args))),
            "_own_pattern" => Some(py_func("_own_pattern", |args: &[Py]| _own_pattern(args))),
            "_past_the_timescale" => Some(py_func("_past_the_timescale", |args: &[Py]| _past_the_timescale(args))),
            "_reads_as_good_news" => Some(g__reads_as_good_news()),
            "_referral" => Some(py_func("_referral", |args: &[Py]| _referral(args))),
            "_referral_about_the_thing" => Some(py_func("_referral_about_the_thing", |args: &[Py]| _referral_about_the_thing(args))),
            "_round_robin" => Some(py_func("_round_robin", |args: &[Py]| _round_robin(args))),
            "_said_enough" => Some(py_func("_said_enough", |args: &[Py]| _said_enough(args))),
            "_said_no" => Some(py_func("_said_no", |args: &[Py]| _said_no(args))),
            "_said_without_a_not" => Some(py_func("_said_without_a_not", |args: &[Py]| _said_without_a_not(args))),
            "_say" => Some(py_func("_say", |args: &[Py]| _say(args))),
            "_says_it_again" => Some(py_func("_says_it_again", |args: &[Py]| _says_it_again(args))),
            "_says_what_it_is" => Some(py_func("_says_what_it_is", |args: &[Py]| _says_what_it_is(args))),
            "_sentence_at" => Some(py_func("_sentence_at", |args: &[Py]| _sentence_at(args))),
            "_shorten" => Some(py_func("_shorten", |args: &[Py]| _shorten(args))),
            "_skip_the_derived" => Some(py_func("_skip_the_derived", |args: &[Py]| _skip_the_derived(args))),
            "_somebody_owes_them" => Some(py_func("_somebody_owes_them", |args: &[Py]| _somebody_owes_them(args))),
            "_stated_before_the_judgement" => Some(py_func("_stated_before_the_judgement", |args: &[Py]| _stated_before_the_judgement(args))),
            "_stated_referral" => Some(py_func("_stated_referral", |args: &[Py]| _stated_referral(args))),
            "_stems" => Some(py_func("_stems", |args: &[Py]| _stems(args))),
            "_step" => Some(py_func("_step", |args: &[Py]| _step(args))),
            "_step_sentence" => Some(py_func("_step_sentence", |args: &[Py]| _step_sentence(args))),
            "_steps" => Some(py_func("_steps", |args: &[Py]| _steps(args))),
            "_subject_words" => Some(py_func("_subject_words", |args: &[Py]| _subject_words(args))),
            "_the_advice_itself" => Some(py_func("_the_advice_itself", |args: &[Py]| _the_advice_itself(args))),
            "_the_answer_already_warned" => Some(py_func("_the_answer_already_warned", |args: &[Py]| _the_answer_already_warned(args))),
            "_the_answer_before_that" => Some(py_func("_the_answer_before_that", |args: &[Py]| _the_answer_before_that(args))),
            "_the_answer_under_a_frame" => Some(py_func("_the_answer_under_a_frame", |args: &[Py]| _the_answer_under_a_frame(args))),
            "_the_call_and_its_reason" => Some(py_func("_the_call_and_its_reason", |args: &[Py]| _the_call_and_its_reason(args))),
            "_the_call_sentence" => Some(py_func("_the_call_sentence", |args: &[Py]| _the_call_sentence(args))),
            "_the_fence_is_met" => Some(py_func("_the_fence_is_met", |args: &[Py]| _the_fence_is_met(args))),
            "_the_fence_is_their_own_line" => Some(py_func("_the_fence_is_their_own_line", |args: &[Py]| _the_fence_is_their_own_line(args))),
            "_the_floor_asked_them_something" => Some(py_func("_the_floor_asked_them_something", |args: &[Py]| _the_floor_asked_them_something(args))),
            "_the_front_names_what_is_in_hand" => Some(py_func("_the_front_names_what_is_in_hand", |args: &[Py]| _the_front_names_what_is_in_hand(args))),
            "_the_knowledge_has_this" => Some(py_func("_the_knowledge_has_this", |args: &[Py]| _the_knowledge_has_this(args))),
            "_the_last_line_was_mine" => Some(py_func("_the_last_line_was_mine", |args: &[Py]| _the_last_line_was_mine(args))),
            "_the_last_person_lines" => Some(py_func("_the_last_person_lines", |args: &[Py]| _the_last_person_lines(args))),
            "_the_last_thing_said_is_a_nod" => Some(py_func("_the_last_thing_said_is_a_nod", |args: &[Py]| _the_last_thing_said_is_a_nod(args))),
            "_the_line_behind" => Some(py_func("_the_line_behind", |args: &[Py]| _the_line_behind(args))),
            "_the_line_is_the_threshold" => Some(py_func("_the_line_is_the_threshold", |args: &[Py]| _the_line_is_the_threshold(args))),
            "_the_line_it_answered" => Some(py_func("_the_line_it_answered", |args: &[Py]| _the_line_it_answered(args))),
            "_the_line_just_given" => Some(py_func("_the_line_just_given", |args: &[Py]| _the_line_just_given(args))),
            "_the_list_before_the_call" => Some(py_func("_the_list_before_the_call", |args: &[Py]| _the_list_before_the_call(args))),
            "_the_news_just_before" => Some(py_func("_the_news_just_before", |args: &[Py]| _the_news_just_before(args))),
            "_the_other_thing_in_the_answer" => Some(py_func("_the_other_thing_in_the_answer", |args: &[Py]| _the_other_thing_in_the_answer(args))),
            "_the_person_for" => Some(py_func("_the_person_for", |args: &[Py]| _the_person_for(args))),
            "_the_person_gave_the_sign" => Some(py_func("_the_person_gave_the_sign", |args: &[Py]| _the_person_gave_the_sign(args))),
            "_the_person_line" => Some(py_func("_the_person_line", |args: &[Py]| _the_person_line(args))),
            "_the_plan_can_hurt" => Some(py_func("_the_plan_can_hurt", |args: &[Py]| _the_plan_can_hurt(args))),
            "_the_plan_they_brought" => Some(py_func("_the_plan_they_brought", |args: &[Py]| _the_plan_they_brought(args))),
            "_the_question_asked" => Some(py_func("_the_question_asked", |args: &[Py]| _the_question_asked(args))),
            "_the_range_it_gave" => Some(py_func("_the_range_it_gave", |args: &[Py]| _the_range_it_gave(args))),
            "_the_referral_it_closed_on" => Some(py_func("_the_referral_it_closed_on", |args: &[Py]| _the_referral_it_closed_on(args))),
            "_the_referral_sentence" => Some(py_func("_the_referral_sentence", |args: &[Py]| _the_referral_sentence(args))),
            "_the_sign_words" => Some(py_func("_the_sign_words", |args: &[Py]| _the_sign_words(args))),
            "_the_span_further_up" => Some(py_func("_the_span_further_up", |args: &[Py]| _the_span_further_up(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_the_worst_thing_they_said" => Some(py_func("_the_worst_thing_they_said", |args: &[Py]| _the_worst_thing_they_said(args))),
            "_they_named_the_sign_themselves" => Some(py_func("_they_named_the_sign_themselves", |args: &[Py]| _they_named_the_sign_themselves(args))),
            "_told_to_act" => Some(py_func("_told_to_act", |args: &[Py]| _told_to_act(args))),
            "_unconditional_care" => Some(py_func("_unconditional_care", |args: &[Py]| _unconditional_care(args))),
            "_up_to_a_break" => Some(py_func("_up_to_a_break", |args: &[Py]| _up_to_a_break(args))),
            "_when" => Some(py_func("_when", |args: &[Py]| _when(args))),
            "_word_runs" => Some(py_func("_word_runs", |args: &[Py]| _word_runs(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "a_check_on_the_reply" => Some(py_func("a_check_on_the_reply", |args: &[Py]| a_check_on_the_reply(args))),
            "answers_the_ask" => Some(py_func("answers_the_ask", |args: &[Py]| answers_the_ask(args))),
            "carries_the_answer" => Some(py_func("carries_the_answer", |args: &[Py]| carries_the_answer(args))),
            "next_step" => Some(py_func("next_step", |args: &[Py]| next_step(args))),
            "opens_on_a_job" => Some(py_func("opens_on_a_job", |args: &[Py]| opens_on_a_job(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "rules_on_the_doubt" => Some(py_func("rules_on_the_doubt", |args: &[Py]| rules_on_the_doubt(args))),
            "takes_a_fact_back" => Some(py_func("takes_a_fact_back", |args: &[Py]| takes_a_fact_back(args))),
            "the_fix_asked" => Some(py_func("the_fix_asked", |args: &[Py]| the_fix_asked(args))),
            _ => None,
        }),
        set: None,
    }))
}
