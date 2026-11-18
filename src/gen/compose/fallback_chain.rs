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


pub static ABOUT_THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOUT_THE_BOT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_AN_ERRAND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_AN_ERRAND_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_IN_A_DIARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_IN_A_DIARY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_ON_A_CLOCK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_ON_A_DOCUMENT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_ON_A_PHONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTS_ON_A_PHONE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_CHOICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_FOR_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_OWN_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_REPORT_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_SETTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADDRESSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ATTEMPT_AT_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_AFTER_THE_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AT_THE_CLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_ASKING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BAD_THING_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BARE_MODIFIER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BEING_DONE_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BETRAYAL_FOUND_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHILDHOOD_MEMORY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CROWD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CROWD_AHEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_SAID_BADLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_WENT_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DECISION_NOT_A_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DOING_GONE_BADLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FEELING_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HAND_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HARD_DAY_MOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LIST_ON_THE_TABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LOOKING_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MEAL_SKIPPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MESSAGE_OR_A_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MILESTONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MISTAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NAMES_TOLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NAME_A_HOUSEHOLD_TAKES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NAME_IS_NOT_A_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NEWS_IS_A_CHANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NEWS_IS_WEATHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_POSSESSIVE_OF_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REGRET_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPORTED_NOT_LIVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SUM_STATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SYMPTOM_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_GOT_AHEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_SKIPPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TIME_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VISIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WRONG_ABOUT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WRONG_AIMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WRONG_PLAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WRONG_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BAD_DAY_ANYWAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOOKING_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOT_WAS_SORRY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAN_DO_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_ABOUT_THE_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_NOT_THIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_SOMEONE_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_THE_DEPARTURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COLD_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTDOWN_AFTER_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTDOWN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTDOWN_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CROWD_ORDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEEL_COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOOD_TOPICS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOUND_IT_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOUND_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_PREFIX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOT_AHEAD_GONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOT_AHEAD_NOT_A_DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HABIT_MIND_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HABIT_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HABIT_STATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANDS_IT_OVER_HERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAVE_GOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOT_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HURT_BUT_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HURT_DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HURT_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HURT_ON_THE_SPEAKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HURT_SOMEBODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEP_ME_POSTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVES_A_GROUP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVES_PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVES_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVE_GROUP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVE_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVE_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVE_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVE_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOVE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOW_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOW_DAY_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOW_DAY_NOT_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_IN_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_IN_SHAPES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEAR_MISS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEAR_MISS_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_AND_A_PLEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_NEGATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_TO_WEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_TO_WEAR_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_ABOUT_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_CROWD_AHEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DAY_DONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DAY_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_JOB_BUT_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NAME_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_REGRET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING_TO_GET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_THING_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPTION_ON_THE_TABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_A_BUY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_A_CLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_A_DOCUMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_A_MESSAGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_A_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_A_REPORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_NOT_A_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORDER_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_THAT_FELL_THROUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLEASURE_ABOUT_ONE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLEASURE_GONE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLEASURE_GONE_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLEASURE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POLITE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PROBLEM_STATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REMIND_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RESULT_WENT_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_A_FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SNAPPED_A_PHOTO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_DONE_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSE_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMETHING_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_IN_THE_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_NOT_A_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_THEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKES_AS_IT_COMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKING_SOMETHING_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_JOB_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_THING_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_WORDS_BETWEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THINGS_THAT_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THINGS_THAT_STOP_AFTER_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEAR_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEAR_WHEN_NOTHING_ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEATHER_SUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_DID_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_ASKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_ATTEMPT_COUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BODY_UNWELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BODY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BREAK_THAT_IS_GOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_CORRECTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DETAIL_OPENING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HARM_HAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HARM_IDIOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HARM_ILL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HARM_ILL_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HARM_KNOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HURT_BY_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HURT_BY_SOMEONE_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HURT_DONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_ILLNESS_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_IN_AGONY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_NEGATIVE_AUXILIARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_OUT_OF_SOCKET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PART_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PART_WENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PROGNOSIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PROGNOSIS_THAT_IS_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SYMPATHY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_TELLING_OF_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BREAK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ROUTINE_NOT_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _UNDERTAKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_THE_BOT() -> Py { ensure_init(); ABOUT_THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THE_BOT.write() { *w = v; } }
pub fn g_ABOUT_THE_BOT_LINES() -> Py { ensure_init(); ABOUT_THE_BOT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_BOT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THE_BOT_LINES.write() { *w = v; } }
pub fn g_ACTS_AN_ERRAND() -> Py { ensure_init(); ACTS_AN_ERRAND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_AN_ERRAND(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_AN_ERRAND.write() { *w = v; } }
pub fn g_ACTS_AN_ERRAND_LINES() -> Py { ensure_init(); ACTS_AN_ERRAND_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_AN_ERRAND_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_AN_ERRAND_LINES.write() { *w = v; } }
pub fn g_ACTS_IN_A_DIARY() -> Py { ensure_init(); ACTS_IN_A_DIARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_IN_A_DIARY(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_IN_A_DIARY.write() { *w = v; } }
pub fn g_ACTS_IN_A_DIARY_LINES() -> Py { ensure_init(); ACTS_IN_A_DIARY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_IN_A_DIARY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_IN_A_DIARY_LINES.write() { *w = v; } }
pub fn g_ACTS_ON_A_CLOCK_LINES() -> Py { ensure_init(); ACTS_ON_A_CLOCK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_ON_A_CLOCK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_ON_A_CLOCK_LINES.write() { *w = v; } }
pub fn g_ACTS_ON_A_DOCUMENT_LINES() -> Py { ensure_init(); ACTS_ON_A_DOCUMENT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_ON_A_DOCUMENT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_ON_A_DOCUMENT_LINES.write() { *w = v; } }
pub fn g_ACTS_ON_A_PHONE() -> Py { ensure_init(); ACTS_ON_A_PHONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_ON_A_PHONE(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_ON_A_PHONE.write() { *w = v; } }
pub fn g_ACTS_ON_A_PHONE_LINES() -> Py { ensure_init(); ACTS_ON_A_PHONE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTS_ON_A_PHONE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ACTS_ON_A_PHONE_LINES.write() { *w = v; } }
pub fn g_ACT_CHOICE() -> Py { ensure_init(); ACT_CHOICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_CHOICE(v: Py) { ensure_init(); if let Ok(mut w) = ACT_CHOICE.write() { *w = v; } }
pub fn g_ACT_FOR_ME() -> Py { ensure_init(); ACT_FOR_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_FOR_ME(v: Py) { ensure_init(); if let Ok(mut w) = ACT_FOR_ME.write() { *w = v; } }
pub fn g_ACT_OPENER() -> Py { ensure_init(); ACT_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = ACT_OPENER.write() { *w = v; } }
pub fn g_ACT_OWN_STEP() -> Py { ensure_init(); ACT_OWN_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_OWN_STEP(v: Py) { ensure_init(); if let Ok(mut w) = ACT_OWN_STEP.write() { *w = v; } }
pub fn g_ACT_REPORT_TAIL() -> Py { ensure_init(); ACT_REPORT_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_REPORT_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = ACT_REPORT_TAIL.write() { *w = v; } }
pub fn g_ACT_SETTING() -> Py { ensure_init(); ACT_SETTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_SETTING(v: Py) { ensure_init(); if let Ok(mut w) = ACT_SETTING.write() { *w = v; } }
pub fn g_ACT_THING() -> Py { ensure_init(); ACT_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_THING(v: Py) { ensure_init(); if let Ok(mut w) = ACT_THING.write() { *w = v; } }
pub fn g_ADDRESSED() -> Py { ensure_init(); ADDRESSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADDRESSED(v: Py) { ensure_init(); if let Ok(mut w) = ADDRESSED.write() { *w = v; } }
pub fn g_AN_ATTEMPT_AT_SOMEONE() -> Py { ensure_init(); AN_ATTEMPT_AT_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ATTEMPT_AT_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = AN_ATTEMPT_AT_SOMEONE.write() { *w = v; } }
pub fn g_ASKS_AFTER_THE_OPENER() -> Py { ensure_init(); ASKS_AFTER_THE_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_AFTER_THE_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_AFTER_THE_OPENER.write() { *w = v; } }
pub fn g_ASK_SHAPE() -> Py { ensure_init(); ASK_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = ASK_SHAPE.write() { *w = v; } }
pub fn g_AT_THE_CLOCK() -> Py { ensure_init(); AT_THE_CLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AT_THE_CLOCK(v: Py) { ensure_init(); if let Ok(mut w) = AT_THE_CLOCK.write() { *w = v; } }
pub fn g_A_ASKING_WORD() -> Py { ensure_init(); A_ASKING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_ASKING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_ASKING_WORD.write() { *w = v; } }
pub fn g_A_BAD_THING_NAMED() -> Py { ensure_init(); A_BAD_THING_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BAD_THING_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = A_BAD_THING_NAMED.write() { *w = v; } }
pub fn g_A_BARE_MODIFIER() -> Py { ensure_init(); A_BARE_MODIFIER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BARE_MODIFIER(v: Py) { ensure_init(); if let Ok(mut w) = A_BARE_MODIFIER.write() { *w = v; } }
pub fn g_A_BEING_DONE_NOW() -> Py { ensure_init(); A_BEING_DONE_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BEING_DONE_NOW(v: Py) { ensure_init(); if let Ok(mut w) = A_BEING_DONE_NOW.write() { *w = v; } }
pub fn g_A_BETRAYAL_FOUND_OUT() -> Py { ensure_init(); A_BETRAYAL_FOUND_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BETRAYAL_FOUND_OUT(v: Py) { ensure_init(); if let Ok(mut w) = A_BETRAYAL_FOUND_OUT.write() { *w = v; } }
pub fn g_A_CHILDHOOD_MEMORY() -> Py { ensure_init(); A_CHILDHOOD_MEMORY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHILDHOOD_MEMORY(v: Py) { ensure_init(); if let Ok(mut w) = A_CHILDHOOD_MEMORY.write() { *w = v; } }
pub fn g_A_CLOCK() -> Py { ensure_init(); A_CLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CLOCK(v: Py) { ensure_init(); if let Ok(mut w) = A_CLOCK.write() { *w = v; } }
pub fn g_A_COMPARISON() -> Py { ensure_init(); A_COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = A_COMPARISON.write() { *w = v; } }
pub fn g_A_CROWD() -> Py { ensure_init(); A_CROWD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CROWD(v: Py) { ensure_init(); if let Ok(mut w) = A_CROWD.write() { *w = v; } }
pub fn g_A_CROWD_AHEAD() -> Py { ensure_init(); A_CROWD_AHEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CROWD_AHEAD(v: Py) { ensure_init(); if let Ok(mut w) = A_CROWD_AHEAD.write() { *w = v; } }
pub fn g_A_DAY_SAID_BADLY() -> Py { ensure_init(); A_DAY_SAID_BADLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_SAID_BADLY(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_SAID_BADLY.write() { *w = v; } }
pub fn g_A_DAY_SUBJECT() -> Py { ensure_init(); A_DAY_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_SUBJECT.write() { *w = v; } }
pub fn g_A_DAY_WENT_WRONG() -> Py { ensure_init(); A_DAY_WENT_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_WENT_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_WENT_WRONG.write() { *w = v; } }
pub fn g_A_DECISION_NOT_A_DAY() -> Py { ensure_init(); A_DECISION_NOT_A_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DECISION_NOT_A_DAY(v: Py) { ensure_init(); if let Ok(mut w) = A_DECISION_NOT_A_DAY.write() { *w = v; } }
pub fn g_A_DOING_GONE_BADLY() -> Py { ensure_init(); A_DOING_GONE_BADLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DOING_GONE_BADLY(v: Py) { ensure_init(); if let Ok(mut w) = A_DOING_GONE_BADLY.write() { *w = v; } }
pub fn g_A_FEELING_IN_THE_LINE() -> Py { ensure_init(); A_FEELING_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FEELING_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = A_FEELING_IN_THE_LINE.write() { *w = v; } }
pub fn g_A_HAND_OUT() -> Py { ensure_init(); A_HAND_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HAND_OUT(v: Py) { ensure_init(); if let Ok(mut w) = A_HAND_OUT.write() { *w = v; } }
pub fn g_A_HARD_DAY_MOOD() -> Py { ensure_init(); A_HARD_DAY_MOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HARD_DAY_MOOD(v: Py) { ensure_init(); if let Ok(mut w) = A_HARD_DAY_MOOD.write() { *w = v; } }
pub fn g_A_LENGTH() -> Py { ensure_init(); A_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = A_LENGTH.write() { *w = v; } }
pub fn g_A_LIST_ON_THE_TABLE() -> Py { ensure_init(); A_LIST_ON_THE_TABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LIST_ON_THE_TABLE(v: Py) { ensure_init(); if let Ok(mut w) = A_LIST_ON_THE_TABLE.write() { *w = v; } }
pub fn g_A_LOOKING_BACK() -> Py { ensure_init(); A_LOOKING_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LOOKING_BACK(v: Py) { ensure_init(); if let Ok(mut w) = A_LOOKING_BACK.write() { *w = v; } }
pub fn g_A_MEAL_SKIPPED() -> Py { ensure_init(); A_MEAL_SKIPPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MEAL_SKIPPED(v: Py) { ensure_init(); if let Ok(mut w) = A_MEAL_SKIPPED.write() { *w = v; } }
pub fn g_A_MESSAGE_OR_A_FORM() -> Py { ensure_init(); A_MESSAGE_OR_A_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MESSAGE_OR_A_FORM(v: Py) { ensure_init(); if let Ok(mut w) = A_MESSAGE_OR_A_FORM.write() { *w = v; } }
pub fn g_A_MILESTONE() -> Py { ensure_init(); A_MILESTONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MILESTONE(v: Py) { ensure_init(); if let Ok(mut w) = A_MILESTONE.write() { *w = v; } }
pub fn g_A_MISTAKE() -> Py { ensure_init(); A_MISTAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MISTAKE(v: Py) { ensure_init(); if let Ok(mut w) = A_MISTAKE.write() { *w = v; } }
pub fn g_A_MONTH() -> Py { ensure_init(); A_MONTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONTH(v: Py) { ensure_init(); if let Ok(mut w) = A_MONTH.write() { *w = v; } }
pub fn g_A_MOOD() -> Py { ensure_init(); A_MOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MOOD(v: Py) { ensure_init(); if let Ok(mut w) = A_MOOD.write() { *w = v; } }
pub fn g_A_NAMES_TOLD() -> Py { ensure_init(); A_NAMES_TOLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NAMES_TOLD(v: Py) { ensure_init(); if let Ok(mut w) = A_NAMES_TOLD.write() { *w = v; } }
pub fn g_A_NAME_A_HOUSEHOLD_TAKES() -> Py { ensure_init(); A_NAME_A_HOUSEHOLD_TAKES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NAME_A_HOUSEHOLD_TAKES(v: Py) { ensure_init(); if let Ok(mut w) = A_NAME_A_HOUSEHOLD_TAKES.write() { *w = v; } }
pub fn g_A_NAME_IS_NOT_A_DAY() -> Py { ensure_init(); A_NAME_IS_NOT_A_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NAME_IS_NOT_A_DAY(v: Py) { ensure_init(); if let Ok(mut w) = A_NAME_IS_NOT_A_DAY.write() { *w = v; } }
pub fn g_A_NEWS_IS_A_CHANGE() -> Py { ensure_init(); A_NEWS_IS_A_CHANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NEWS_IS_A_CHANGE(v: Py) { ensure_init(); if let Ok(mut w) = A_NEWS_IS_A_CHANGE.write() { *w = v; } }
pub fn g_A_NEWS_IS_WEATHER() -> Py { ensure_init(); A_NEWS_IS_WEATHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NEWS_IS_WEATHER(v: Py) { ensure_init(); if let Ok(mut w) = A_NEWS_IS_WEATHER.write() { *w = v; } }
pub fn g_A_POSSESSIVE_OF_MINE() -> Py { ensure_init(); A_POSSESSIVE_OF_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_POSSESSIVE_OF_MINE(v: Py) { ensure_init(); if let Ok(mut w) = A_POSSESSIVE_OF_MINE.write() { *w = v; } }
pub fn g_A_REGRET_DOING() -> Py { ensure_init(); A_REGRET_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REGRET_DOING(v: Py) { ensure_init(); if let Ok(mut w) = A_REGRET_DOING.write() { *w = v; } }
pub fn g_A_REPORTED_NOT_LIVED() -> Py { ensure_init(); A_REPORTED_NOT_LIVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPORTED_NOT_LIVED(v: Py) { ensure_init(); if let Ok(mut w) = A_REPORTED_NOT_LIVED.write() { *w = v; } }
pub fn g_A_SUM_STATED() -> Py { ensure_init(); A_SUM_STATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SUM_STATED(v: Py) { ensure_init(); if let Ok(mut w) = A_SUM_STATED.write() { *w = v; } }
pub fn g_A_SYMPTOM_PHRASE() -> Py { ensure_init(); A_SYMPTOM_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SYMPTOM_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = A_SYMPTOM_PHRASE.write() { *w = v; } }
pub fn g_A_THING_GOT_AHEAD() -> Py { ensure_init(); A_THING_GOT_AHEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_GOT_AHEAD(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_GOT_AHEAD.write() { *w = v; } }
pub fn g_A_THING_SKIPPED() -> Py { ensure_init(); A_THING_SKIPPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_SKIPPED(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_SKIPPED.write() { *w = v; } }
pub fn g_A_TIME_SAID() -> Py { ensure_init(); A_TIME_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TIME_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_TIME_SAID.write() { *w = v; } }
pub fn g_A_VISIT() -> Py { ensure_init(); A_VISIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VISIT(v: Py) { ensure_init(); if let Ok(mut w) = A_VISIT.write() { *w = v; } }
pub fn g_A_WISH() -> Py { ensure_init(); A_WISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WISH(v: Py) { ensure_init(); if let Ok(mut w) = A_WISH.write() { *w = v; } }
pub fn g_A_WRONG_ABOUT_A_THING() -> Py { ensure_init(); A_WRONG_ABOUT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WRONG_ABOUT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_WRONG_ABOUT_A_THING.write() { *w = v; } }
pub fn g_A_WRONG_AIMED() -> Py { ensure_init(); A_WRONG_AIMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WRONG_AIMED(v: Py) { ensure_init(); if let Ok(mut w) = A_WRONG_AIMED.write() { *w = v; } }
pub fn g_A_WRONG_PLAIN() -> Py { ensure_init(); A_WRONG_PLAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WRONG_PLAIN(v: Py) { ensure_init(); if let Ok(mut w) = A_WRONG_PLAIN.write() { *w = v; } }
pub fn g_A_WRONG_SAID() -> Py { ensure_init(); A_WRONG_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WRONG_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_WRONG_SAID.write() { *w = v; } }
pub fn g_BAD_DAY_ANYWAY() -> Py { ensure_init(); BAD_DAY_ANYWAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BAD_DAY_ANYWAY(v: Py) { ensure_init(); if let Ok(mut w) = BAD_DAY_ANYWAY.write() { *w = v; } }
pub fn g_BARE_STATE() -> Py { ensure_init(); BARE_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_STATE(v: Py) { ensure_init(); if let Ok(mut w) = BARE_STATE.write() { *w = v; } }
pub fn g_BOOKING_THING() -> Py { ensure_init(); BOOKING_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOOKING_THING(v: Py) { ensure_init(); if let Ok(mut w) = BOOKING_THING.write() { *w = v; } }
pub fn g_BOT_WAS_SORRY() -> Py { ensure_init(); BOT_WAS_SORRY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOT_WAS_SORRY(v: Py) { ensure_init(); if let Ok(mut w) = BOT_WAS_SORRY.write() { *w = v; } }
pub fn g_CAN_DO_ITSELF() -> Py { ensure_init(); CAN_DO_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAN_DO_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = CAN_DO_ITSELF.write() { *w = v; } }
pub fn g_CLOCK_ABOUT_THE_JOB() -> Py { ensure_init(); CLOCK_ABOUT_THE_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_ABOUT_THE_JOB(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_ABOUT_THE_JOB.write() { *w = v; } }
pub fn g_CLOCK_ASKED() -> Py { ensure_init(); CLOCK_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_ASKED.write() { *w = v; } }
pub fn g_CLOCK_NOT_THIS() -> Py { ensure_init(); CLOCK_NOT_THIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_NOT_THIS(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_NOT_THIS.write() { *w = v; } }
pub fn g_CLOCK_SHAPE() -> Py { ensure_init(); CLOCK_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_SHAPE.write() { *w = v; } }
pub fn g_CLOCK_SOMEONE_ELSE() -> Py { ensure_init(); CLOCK_SOMEONE_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_SOMEONE_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_SOMEONE_ELSE.write() { *w = v; } }
pub fn g_CLOCK_THE_DEPARTURE() -> Py { ensure_init(); CLOCK_THE_DEPARTURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_THE_DEPARTURE(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_THE_DEPARTURE.write() { *w = v; } }
pub fn g_COLD_FIGURE() -> Py { ensure_init(); COLD_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COLD_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = COLD_FIGURE.write() { *w = v; } }
pub fn g_COUNTDOWN_AFTER_CARE() -> Py { ensure_init(); COUNTDOWN_AFTER_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTDOWN_AFTER_CARE(v: Py) { ensure_init(); if let Ok(mut w) = COUNTDOWN_AFTER_CARE.write() { *w = v; } }
pub fn g_COUNTDOWN_ASK() -> Py { ensure_init(); COUNTDOWN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTDOWN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = COUNTDOWN_ASK.write() { *w = v; } }
pub fn g_COUNTDOWN_TO() -> Py { ensure_init(); COUNTDOWN_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTDOWN_TO(v: Py) { ensure_init(); if let Ok(mut w) = COUNTDOWN_TO.write() { *w = v; } }
pub fn g_COUNTED() -> Py { ensure_init(); COUNTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTED(v: Py) { ensure_init(); if let Ok(mut w) = COUNTED.write() { *w = v; } }
pub fn g_CROWD_ORDER() -> Py { ensure_init(); CROWD_ORDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CROWD_ORDER(v: Py) { ensure_init(); if let Ok(mut w) = CROWD_ORDER.write() { *w = v; } }
pub fn g_DAY_PAST() -> Py { ensure_init(); DAY_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_PAST(v: Py) { ensure_init(); if let Ok(mut w) = DAY_PAST.write() { *w = v; } }
pub fn g_FEELING_NOTHING() -> Py { ensure_init(); FEELING_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_NOTHING.write() { *w = v; } }
pub fn g_FEEL_COPULA() -> Py { ensure_init(); FEEL_COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEEL_COPULA(v: Py) { ensure_init(); if let Ok(mut w) = FEEL_COPULA.write() { *w = v; } }
pub fn g_FOOD_TOPICS() -> Py { ensure_init(); FOOD_TOPICS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOOD_TOPICS(v: Py) { ensure_init(); if let Ok(mut w) = FOOD_TOPICS.write() { *w = v; } }
pub fn g_FOUND_IT_OUT() -> Py { ensure_init(); FOUND_IT_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOUND_IT_OUT(v: Py) { ensure_init(); if let Ok(mut w) = FOUND_IT_OUT.write() { *w = v; } }
pub fn g_FOUND_OUT() -> Py { ensure_init(); FOUND_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOUND_OUT(v: Py) { ensure_init(); if let Ok(mut w) = FOUND_OUT.write() { *w = v; } }
pub fn g_FRAME_PREFIX() -> Py { ensure_init(); FRAME_PREFIX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_PREFIX(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_PREFIX.write() { *w = v; } }
pub fn g_GOT_AHEAD_GONE() -> Py { ensure_init(); GOT_AHEAD_GONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOT_AHEAD_GONE(v: Py) { ensure_init(); if let Ok(mut w) = GOT_AHEAD_GONE.write() { *w = v; } }
pub fn g_GOT_AHEAD_NOT_A_DATE() -> Py { ensure_init(); GOT_AHEAD_NOT_A_DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOT_AHEAD_NOT_A_DATE(v: Py) { ensure_init(); if let Ok(mut w) = GOT_AHEAD_NOT_A_DATE.write() { *w = v; } }
pub fn g_HABIT_MIND_IT() -> Py { ensure_init(); HABIT_MIND_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HABIT_MIND_IT(v: Py) { ensure_init(); if let Ok(mut w) = HABIT_MIND_IT.write() { *w = v; } }
pub fn g_HABIT_NOT() -> Py { ensure_init(); HABIT_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HABIT_NOT(v: Py) { ensure_init(); if let Ok(mut w) = HABIT_NOT.write() { *w = v; } }
pub fn g_HABIT_STATED() -> Py { ensure_init(); HABIT_STATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HABIT_STATED(v: Py) { ensure_init(); if let Ok(mut w) = HABIT_STATED.write() { *w = v; } }
pub fn g_HANDS_IT_OVER_HERE() -> Py { ensure_init(); HANDS_IT_OVER_HERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANDS_IT_OVER_HERE(v: Py) { ensure_init(); if let Ok(mut w) = HANDS_IT_OVER_HERE.write() { *w = v; } }
pub fn g_HAVE_GOT_A_THING() -> Py { ensure_init(); HAVE_GOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAVE_GOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = HAVE_GOT_A_THING.write() { *w = v; } }
pub fn g_HOT_FIGURE() -> Py { ensure_init(); HOT_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOT_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = HOT_FIGURE.write() { *w = v; } }
pub fn g_HURT_BUT_FINE() -> Py { ensure_init(); HURT_BUT_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HURT_BUT_FINE(v: Py) { ensure_init(); if let Ok(mut w) = HURT_BUT_FINE.write() { *w = v; } }
pub fn g_HURT_DEATH() -> Py { ensure_init(); HURT_DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HURT_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = HURT_DEATH.write() { *w = v; } }
pub fn g_HURT_FIGURE() -> Py { ensure_init(); HURT_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HURT_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = HURT_FIGURE.write() { *w = v; } }
pub fn g_HURT_ON_THE_SPEAKER() -> Py { ensure_init(); HURT_ON_THE_SPEAKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HURT_ON_THE_SPEAKER(v: Py) { ensure_init(); if let Ok(mut w) = HURT_ON_THE_SPEAKER.write() { *w = v; } }
pub fn g_HURT_SOMEBODY() -> Py { ensure_init(); HURT_SOMEBODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HURT_SOMEBODY(v: Py) { ensure_init(); if let Ok(mut w) = HURT_SOMEBODY.write() { *w = v; } }
pub fn g_KEEP_ME_POSTED() -> Py { ensure_init(); KEEP_ME_POSTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEP_ME_POSTED(v: Py) { ensure_init(); if let Ok(mut w) = KEEP_ME_POSTED.write() { *w = v; } }
pub fn g_LOVES_A_GROUP() -> Py { ensure_init(); LOVES_A_GROUP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVES_A_GROUP(v: Py) { ensure_init(); if let Ok(mut w) = LOVES_A_GROUP.write() { *w = v; } }
pub fn g_LOVES_PEOPLE() -> Py { ensure_init(); LOVES_PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVES_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = LOVES_PEOPLE.write() { *w = v; } }
pub fn g_LOVES_SOMEONE() -> Py { ensure_init(); LOVES_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVES_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = LOVES_SOMEONE.write() { *w = v; } }
pub fn g_LOVE_GROUP() -> Py { ensure_init(); LOVE_GROUP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVE_GROUP(v: Py) { ensure_init(); if let Ok(mut w) = LOVE_GROUP.write() { *w = v; } }
pub fn g_LOVE_ONE() -> Py { ensure_init(); LOVE_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVE_ONE(v: Py) { ensure_init(); if let Ok(mut w) = LOVE_ONE.write() { *w = v; } }
pub fn g_LOVE_OPENER() -> Py { ensure_init(); LOVE_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVE_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = LOVE_OPENER.write() { *w = v; } }
pub fn g_LOVE_PERSON() -> Py { ensure_init(); LOVE_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVE_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = LOVE_PERSON.write() { *w = v; } }
pub fn g_LOVE_START() -> Py { ensure_init(); LOVE_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVE_START(v: Py) { ensure_init(); if let Ok(mut w) = LOVE_START.write() { *w = v; } }
pub fn g_LOVE_TAIL() -> Py { ensure_init(); LOVE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOVE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = LOVE_TAIL.write() { *w = v; } }
pub fn g_LOW_DAY() -> Py { ensure_init(); LOW_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOW_DAY(v: Py) { ensure_init(); if let Ok(mut w) = LOW_DAY.write() { *w = v; } }
pub fn g_LOW_DAY_MINE() -> Py { ensure_init(); LOW_DAY_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOW_DAY_MINE(v: Py) { ensure_init(); if let Ok(mut w) = LOW_DAY_MINE.write() { *w = v; } }
pub fn g_LOW_DAY_NOT_MINE() -> Py { ensure_init(); LOW_DAY_NOT_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOW_DAY_NOT_MINE(v: Py) { ensure_init(); if let Ok(mut w) = LOW_DAY_NOT_MINE.write() { *w = v; } }
pub fn g_MONEY_IN_NOUNS() -> Py { ensure_init(); MONEY_IN_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_IN_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_IN_NOUNS.write() { *w = v; } }
pub fn g_MONEY_IN_SHAPES() -> Py { ensure_init(); MONEY_IN_SHAPES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_IN_SHAPES(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_IN_SHAPES.write() { *w = v; } }
pub fn g_NAMED_AFTER() -> Py { ensure_init(); NAMED_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_AFTER.write() { *w = v; } }
pub fn g_NEAR_MISS() -> Py { ensure_init(); NEAR_MISS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEAR_MISS(v: Py) { ensure_init(); if let Ok(mut w) = NEAR_MISS.write() { *w = v; } }
pub fn g_NEAR_MISS_REPLY() -> Py { ensure_init(); NEAR_MISS_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEAR_MISS_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = NEAR_MISS_REPLY.write() { *w = v; } }
pub fn g_NOTHING_AND_A_PLEASURE() -> Py { ensure_init(); NOTHING_AND_A_PLEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_AND_A_PLEASURE(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_AND_A_PLEASURE.write() { *w = v; } }
pub fn g_NOTHING_NEGATED() -> Py { ensure_init(); NOTHING_NEGATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NEGATED(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_NEGATED.write() { *w = v; } }
pub fn g_NOTHING_TO_WEAR() -> Py { ensure_init(); NOTHING_TO_WEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_TO_WEAR(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_TO_WEAR.write() { *w = v; } }
pub fn g_NOTHING_TO_WEAR_REPLIES() -> Py { ensure_init(); NOTHING_TO_WEAR_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_TO_WEAR_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_TO_WEAR_REPLIES.write() { *w = v; } }
pub fn g_NOT_ABOUT_THEM() -> Py { ensure_init(); NOT_ABOUT_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_THEM(v: Py) { ensure_init(); if let Ok(mut w) = NOT_ABOUT_THEM.write() { *w = v; } }
pub fn g_NOT_A_CROWD_AHEAD() -> Py { ensure_init(); NOT_A_CROWD_AHEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CROWD_AHEAD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_CROWD_AHEAD.write() { *w = v; } }
pub fn g_NOT_A_DAY_DONE() -> Py { ensure_init(); NOT_A_DAY_DONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DAY_DONE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DAY_DONE.write() { *w = v; } }
pub fn g_NOT_A_DAY_SAID() -> Py { ensure_init(); NOT_A_DAY_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DAY_SAID(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DAY_SAID.write() { *w = v; } }
pub fn g_NOT_A_JOB_BUT_A_PERSON() -> Py { ensure_init(); NOT_A_JOB_BUT_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOB_BUT_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_JOB_BUT_A_PERSON.write() { *w = v; } }
pub fn g_NOT_A_NAME_WORD() -> Py { ensure_init(); NOT_A_NAME_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME_WORD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NAME_WORD.write() { *w = v; } }
pub fn g_NOT_A_REGRET() -> Py { ensure_init(); NOT_A_REGRET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_REGRET(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_REGRET.write() { *w = v; } }
pub fn g_NOT_A_THING_TO_GET() -> Py { ensure_init(); NOT_A_THING_TO_GET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_TO_GET(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING_TO_GET.write() { *w = v; } }
pub fn g_NO_THING_SAID() -> Py { ensure_init(); NO_THING_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_THING_SAID(v: Py) { ensure_init(); if let Ok(mut w) = NO_THING_SAID.write() { *w = v; } }
pub fn g_OPTION_ON_THE_TABLE() -> Py { ensure_init(); OPTION_ON_THE_TABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPTION_ON_THE_TABLE(v: Py) { ensure_init(); if let Ok(mut w) = OPTION_ON_THE_TABLE.write() { *w = v; } }
pub fn g_ORDER_A_BUY() -> Py { ensure_init(); ORDER_A_BUY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_A_BUY(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_A_BUY.write() { *w = v; } }
pub fn g_ORDER_A_CLOCK() -> Py { ensure_init(); ORDER_A_CLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_A_CLOCK(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_A_CLOCK.write() { *w = v; } }
pub fn g_ORDER_A_DOCUMENT() -> Py { ensure_init(); ORDER_A_DOCUMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_A_DOCUMENT(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_A_DOCUMENT.write() { *w = v; } }
pub fn g_ORDER_A_MESSAGE() -> Py { ensure_init(); ORDER_A_MESSAGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_A_MESSAGE(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_A_MESSAGE.write() { *w = v; } }
pub fn g_ORDER_A_RECORD() -> Py { ensure_init(); ORDER_A_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_A_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_A_RECORD.write() { *w = v; } }
pub fn g_ORDER_A_REPORT() -> Py { ensure_init(); ORDER_A_REPORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_A_REPORT(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_A_REPORT.write() { *w = v; } }
pub fn g_ORDER_NOT_A_JOB() -> Py { ensure_init(); ORDER_NOT_A_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_NOT_A_JOB(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_NOT_A_JOB.write() { *w = v; } }
pub fn g_ORDER_OPENER() -> Py { ensure_init(); ORDER_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORDER_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = ORDER_OPENER.write() { *w = v; } }
pub fn g_OWN_THING() -> Py { ensure_init(); OWN_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_THING(v: Py) { ensure_init(); if let Ok(mut w) = OWN_THING.write() { *w = v; } }
pub fn g_PAST_DOING() -> Py { ensure_init(); PAST_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_DOING(v: Py) { ensure_init(); if let Ok(mut w) = PAST_DOING.write() { *w = v; } }
pub fn g_PLAN_THAT_FELL_THROUGH() -> Py { ensure_init(); PLAN_THAT_FELL_THROUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_THAT_FELL_THROUGH(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_THAT_FELL_THROUGH.write() { *w = v; } }
pub fn g_PLEASURE_ABOUT_ONE_THING() -> Py { ensure_init(); PLEASURE_ABOUT_ONE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLEASURE_ABOUT_ONE_THING(v: Py) { ensure_init(); if let Ok(mut w) = PLEASURE_ABOUT_ONE_THING.write() { *w = v; } }
pub fn g_PLEASURE_GONE_LINES() -> Py { ensure_init(); PLEASURE_GONE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLEASURE_GONE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = PLEASURE_GONE_LINES.write() { *w = v; } }
pub fn g_PLEASURE_GONE_OUT() -> Py { ensure_init(); PLEASURE_GONE_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLEASURE_GONE_OUT(v: Py) { ensure_init(); if let Ok(mut w) = PLEASURE_GONE_OUT.write() { *w = v; } }
pub fn g_PLEASURE_WORD() -> Py { ensure_init(); PLEASURE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLEASURE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = PLEASURE_WORD.write() { *w = v; } }
pub fn g_POLITE_ASK() -> Py { ensure_init(); POLITE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POLITE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = POLITE_ASK.write() { *w = v; } }
pub fn g_PRICE_NOUNS() -> Py { ensure_init(); PRICE_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_NOUNS.write() { *w = v; } }
pub fn g_PROBLEM_STATED() -> Py { ensure_init(); PROBLEM_STATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PROBLEM_STATED(v: Py) { ensure_init(); if let Ok(mut w) = PROBLEM_STATED.write() { *w = v; } }
pub fn g_REMIND_ME() -> Py { ensure_init(); REMIND_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REMIND_ME(v: Py) { ensure_init(); if let Ok(mut w) = REMIND_ME.write() { *w = v; } }
pub fn g_RESULT_WENT_WRONG() -> Py { ensure_init(); RESULT_WENT_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RESULT_WENT_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = RESULT_WENT_WRONG.write() { *w = v; } }
pub fn g_SAYS_A_FEELING() -> Py { ensure_init(); SAYS_A_FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_A_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_A_FEELING.write() { *w = v; } }
pub fn g_SAYS_A_THING() -> Py { ensure_init(); SAYS_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_A_THING.write() { *w = v; } }
pub fn g_SNAPPED_A_PHOTO() -> Py { ensure_init(); SNAPPED_A_PHOTO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SNAPPED_A_PHOTO(v: Py) { ensure_init(); if let Ok(mut w) = SNAPPED_A_PHOTO.write() { *w = v; } }
pub fn g_SOMEONE_DONE_WRONG() -> Py { ensure_init(); SOMEONE_DONE_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_DONE_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_DONE_WRONG.write() { *w = v; } }
pub fn g_SOMEONE_ELSE_NAMED() -> Py { ensure_init(); SOMEONE_ELSE_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSE_NAMED.write() { *w = v; } }
pub fn g_SOMETHING_UP() -> Py { ensure_init(); SOMETHING_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMETHING_UP(v: Py) { ensure_init(); if let Ok(mut w) = SOMETHING_UP.write() { *w = v; } }
pub fn g_STATE_IN_THE_PAST() -> Py { ensure_init(); STATE_IN_THE_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_IN_THE_PAST(v: Py) { ensure_init(); if let Ok(mut w) = STATE_IN_THE_PAST.write() { *w = v; } }
pub fn g_STATE_NOT_A_DOING() -> Py { ensure_init(); STATE_NOT_A_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_NOT_A_DOING(v: Py) { ensure_init(); if let Ok(mut w) = STATE_NOT_A_DOING.write() { *w = v; } }
pub fn g_STATE_THEN() -> Py { ensure_init(); STATE_THEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_THEN(v: Py) { ensure_init(); if let Ok(mut w) = STATE_THEN.write() { *w = v; } }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); TAKES_AS_IT_COMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); if let Ok(mut w) = TAKES_AS_IT_COMES.write() { *w = v; } }
pub fn g_TAKING_SOMETHING_BACK() -> Py { ensure_init(); TAKING_SOMETHING_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKING_SOMETHING_BACK(v: Py) { ensure_init(); if let Ok(mut w) = TAKING_SOMETHING_BACK.write() { *w = v; } }
pub fn g_THEIR_OWN_DOING() -> Py { ensure_init(); THEIR_OWN_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_DOING(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN_DOING.write() { *w = v; } }
pub fn g_THE_JOB_NAMED() -> Py { ensure_init(); THE_JOB_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_JOB_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = THE_JOB_NAMED.write() { *w = v; } }
pub fn g_THE_OPENER() -> Py { ensure_init(); THE_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = THE_OPENER.write() { *w = v; } }
pub fn g_THE_THING_ITSELF() -> Py { ensure_init(); THE_THING_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_THING_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = THE_THING_ITSELF.write() { *w = v; } }
pub fn g_THE_WORDS_BETWEEN() -> Py { ensure_init(); THE_WORDS_BETWEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_WORDS_BETWEEN(v: Py) { ensure_init(); if let Ok(mut w) = THE_WORDS_BETWEEN.write() { *w = v; } }
pub fn g_THINGS_THAT_STOP() -> Py { ensure_init(); THINGS_THAT_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THINGS_THAT_STOP(v: Py) { ensure_init(); if let Ok(mut w) = THINGS_THAT_STOP.write() { *w = v; } }
pub fn g_THINGS_THAT_STOP_AFTER_UP() -> Py { ensure_init(); THINGS_THAT_STOP_AFTER_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THINGS_THAT_STOP_AFTER_UP(v: Py) { ensure_init(); if let Ok(mut w) = THINGS_THAT_STOP_AFTER_UP.write() { *w = v; } }
pub fn g_WEAR_ASK() -> Py { ensure_init(); WEAR_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEAR_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WEAR_ASK.write() { *w = v; } }
pub fn g_WEAR_WHEN_NOTHING_ANSWERS() -> Py { ensure_init(); WEAR_WHEN_NOTHING_ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEAR_WHEN_NOTHING_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = WEAR_WHEN_NOTHING_ANSWERS.write() { *w = v; } }
pub fn g_WEATHER_SUN() -> Py { ensure_init(); WEATHER_SUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEATHER_SUN(v: Py) { ensure_init(); if let Ok(mut w) = WEATHER_SUN.write() { *w = v; } }
pub fn g_WHO_DID_IT() -> Py { ensure_init(); WHO_DID_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_DID_IT(v: Py) { ensure_init(); if let Ok(mut w) = WHO_DID_IT.write() { *w = v; } }
pub fn g__AN_ASKING() -> Py { ensure_init(); _AN_ASKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_ASKING(v: Py) { ensure_init(); if let Ok(mut w) = _AN_ASKING.write() { *w = v; } }
pub fn g__AN_ATTEMPT_COUNT() -> Py { ensure_init(); _AN_ATTEMPT_COUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_ATTEMPT_COUNT(v: Py) { ensure_init(); if let Ok(mut w) = _AN_ATTEMPT_COUNT.write() { *w = v; } }
pub fn g__A_BODY_UNWELL() -> Py { ensure_init(); _A_BODY_UNWELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BODY_UNWELL(v: Py) { ensure_init(); if let Ok(mut w) = _A_BODY_UNWELL.write() { *w = v; } }
pub fn g__A_BODY_WORD() -> Py { ensure_init(); _A_BODY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BODY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_BODY_WORD.write() { *w = v; } }
pub fn g__A_BREAK_THAT_IS_GOOD() -> Py { ensure_init(); _A_BREAK_THAT_IS_GOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BREAK_THAT_IS_GOOD(v: Py) { ensure_init(); if let Ok(mut w) = _A_BREAK_THAT_IS_GOOD.write() { *w = v; } }
pub fn g__A_CORRECTION() -> Py { ensure_init(); _A_CORRECTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_CORRECTION(v: Py) { ensure_init(); if let Ok(mut w) = _A_CORRECTION.write() { *w = v; } }
pub fn g__A_DETAIL_OPENING() -> Py { ensure_init(); _A_DETAIL_OPENING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DETAIL_OPENING(v: Py) { ensure_init(); if let Ok(mut w) = _A_DETAIL_OPENING.write() { *w = v; } }
pub fn g__A_HARM_HAD() -> Py { ensure_init(); _A_HARM_HAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HARM_HAD(v: Py) { ensure_init(); if let Ok(mut w) = _A_HARM_HAD.write() { *w = v; } }
pub fn g__A_HARM_IDIOM() -> Py { ensure_init(); _A_HARM_IDIOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HARM_IDIOM(v: Py) { ensure_init(); if let Ok(mut w) = _A_HARM_IDIOM.write() { *w = v; } }
pub fn g__A_HARM_ILL() -> Py { ensure_init(); _A_HARM_ILL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HARM_ILL(v: Py) { ensure_init(); if let Ok(mut w) = _A_HARM_ILL.write() { *w = v; } }
pub fn g__A_HARM_ILL_NOT() -> Py { ensure_init(); _A_HARM_ILL_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HARM_ILL_NOT(v: Py) { ensure_init(); if let Ok(mut w) = _A_HARM_ILL_NOT.write() { *w = v; } }
pub fn g__A_HARM_KNOCK() -> Py { ensure_init(); _A_HARM_KNOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HARM_KNOCK(v: Py) { ensure_init(); if let Ok(mut w) = _A_HARM_KNOCK.write() { *w = v; } }
pub fn g__A_HURT_BY_SOMEONE() -> Py { ensure_init(); _A_HURT_BY_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HURT_BY_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = _A_HURT_BY_SOMEONE.write() { *w = v; } }
pub fn g__A_HURT_BY_SOMEONE_NOT() -> Py { ensure_init(); _A_HURT_BY_SOMEONE_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HURT_BY_SOMEONE_NOT(v: Py) { ensure_init(); if let Ok(mut w) = _A_HURT_BY_SOMEONE_NOT.write() { *w = v; } }
pub fn g__A_HURT_DONE() -> Py { ensure_init(); _A_HURT_DONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HURT_DONE(v: Py) { ensure_init(); if let Ok(mut w) = _A_HURT_DONE.write() { *w = v; } }
pub fn g__A_ILLNESS_WORD() -> Py { ensure_init(); _A_ILLNESS_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_ILLNESS_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_ILLNESS_WORD.write() { *w = v; } }
pub fn g__A_IN_AGONY() -> Py { ensure_init(); _A_IN_AGONY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_IN_AGONY(v: Py) { ensure_init(); if let Ok(mut w) = _A_IN_AGONY.write() { *w = v; } }
pub fn g__A_NEGATIVE_AUXILIARY() -> Py { ensure_init(); _A_NEGATIVE_AUXILIARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_NEGATIVE_AUXILIARY(v: Py) { ensure_init(); if let Ok(mut w) = _A_NEGATIVE_AUXILIARY.write() { *w = v; } }
pub fn g__A_OUT_OF_SOCKET() -> Py { ensure_init(); _A_OUT_OF_SOCKET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_OUT_OF_SOCKET(v: Py) { ensure_init(); if let Ok(mut w) = _A_OUT_OF_SOCKET.write() { *w = v; } }
pub fn g__A_PART_OUT() -> Py { ensure_init(); _A_PART_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PART_OUT(v: Py) { ensure_init(); if let Ok(mut w) = _A_PART_OUT.write() { *w = v; } }
pub fn g__A_PART_WENT() -> Py { ensure_init(); _A_PART_WENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PART_WENT(v: Py) { ensure_init(); if let Ok(mut w) = _A_PART_WENT.write() { *w = v; } }
pub fn g__A_PROGNOSIS() -> Py { ensure_init(); _A_PROGNOSIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PROGNOSIS(v: Py) { ensure_init(); if let Ok(mut w) = _A_PROGNOSIS.write() { *w = v; } }
pub fn g__A_PROGNOSIS_THAT_IS_NOT() -> Py { ensure_init(); _A_PROGNOSIS_THAT_IS_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PROGNOSIS_THAT_IS_NOT(v: Py) { ensure_init(); if let Ok(mut w) = _A_PROGNOSIS_THAT_IS_NOT.write() { *w = v; } }
pub fn g__A_SOMEONE() -> Py { ensure_init(); _A_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = _A_SOMEONE.write() { *w = v; } }
pub fn g__A_SYMPATHY() -> Py { ensure_init(); _A_SYMPATHY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SYMPATHY(v: Py) { ensure_init(); if let Ok(mut w) = _A_SYMPATHY.write() { *w = v; } }
pub fn g__A_TELLING_OF_WRONG() -> Py { ensure_init(); _A_TELLING_OF_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_TELLING_OF_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = _A_TELLING_OF_WRONG.write() { *w = v; } }
pub fn g__BREAK_WORDS() -> Py { ensure_init(); _BREAK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BREAK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _BREAK_WORDS.write() { *w = v; } }
pub fn g__ROUTINE_NOT_SHAPE() -> Py { ensure_init(); _ROUTINE_NOT_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ROUTINE_NOT_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = _ROUTINE_NOT_SHAPE.write() { *w = v; } }
pub fn g__UNDERTAKING() -> Py { ensure_init(); _UNDERTAKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__UNDERTAKING(v: Py) { ensure_init(); if let Ok(mut w) = _UNDERTAKING.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Rows of fallback, kept in their own file so fallback.py stays under the publisher's 1 MiB limit.\n\nNothing here is new: every name below was written in fallback.py and moved out unchanged, verbatim\nand in the same order, when that file went over the limit. It is imported from the last line\nof fallback.py, so the two still read as one module.\n");
    s_FEEL_COPULA(call_builtin("frozenset", &[py_tuple(vec![py_str("feel"), py_str("feeling")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT_STATED(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\b(?:i|we)\\s+(?:(?:always|never)\\s+"), &crate::gen::compose::fallback::g_HABIT_ACT()), &py_str("|(?:keep|keeps|end\\s+up|ends\\s+up|can'?t\\s+stop|cant\\s+stop)\\s+")), &crate::gen::compose::fallback::g_HABIT_ACT()), &py_str("|(?:tend|tends)\\s+to\\s+")), &crate::gen::compose::fallback::g_HABIT_ACT()), &py_str("|never\\s+get\\s+round\\s+to\\b")), &py_str(")|\\b(?:im|i'?m)\\s+always\\s+")), &crate::gen::compose::fallback::g_HABIT_ACT()), &py_str("|\\bmy\\s+(?:brain|mind|memory|head|hands?|eyes?|stomach|skin|body|voice|mood|sleep|focus|concentration|attention|nerves|anxiety|appetite|breathing|gut|bladder|jaw)\\s+(?:keeps|always|never)\\s+")), &crate::gen::compose::fallback::g_HABIT_ACT()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:never\\s+get\\s+(?:invited|asked|picked|chosen|included)|left\\s+out|no\\s+one\\b|nobody\\b|\\b(?:always|never)\\s+do(?:es)?\\s+(?:this|that|it)\\b|\\b(?:always|never)\\s+late\\b|\\b(?:going|go|went|running|run)\\s+back\\s+to\\s+(?:him|her|them)\\b|\\bnever\\s+(?:hear|heard|see|saw|seen|win|won)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT_MIND_IT(stdlib_call("re", "compile", &[py_str("(?:i\\s+wish|can'?t\\s+stop|cant\\s+stop|want\\s+to\\s+stop|need\\s+to\\s+stop|trying\\s+to\\s+stop|drives\\s+me\\s+(?:mad|crazy|nuts|up\\s+the\\s+wall)|bothers\\s+me|gets\\s+to\\s+me|sick\\s+of|fed\\s+up|i\\s+hate\\s+that|shouldn'?t|shouldnt|i\\s+feel\\s+bad|i\\s+feel\\s+guilty|upsets\\s+me|i\\s+do\\s+it\\s+again)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_ABOUT_THEM(stdlib_call("re", "compile", &[py_str("\\bthe\\s+([\\w']+)\\s+(?:is|was|are|were|gets|looks)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_OPENER(stdlib_call("re", "compile", &[py_str("^(?:please\\s+|ok\\s+|okay\\s+|right\\s+|so\\s+|hi\\s+|hello\\s+|hey\\s+|and\\s+|(?:can|could|would|will)\\s+you\\s+(?:please\\s+)?)*(?:take|put|get|bring|grab|turn|set|fill|switch|open|lock|wash|hoover|tidy|clean|post|feed|walk|make|book|reserve|order|call|ring|phone|text|email|buy|pay|renew|send|search|sign|pick|save|keep|chase|look|find)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_THING(stdlib_call("re", "compile", &[py_str("\\b(?:rubbish|bins?|bin\\s+bags?|kettle|cuppa|tea|coffee|oven|heat|heating|thermostat|alarm|washing|laundry|dishes|door|window|windows|light|lights|lamp|water|bottle|table|taxi|cab|appointment|doctor|dentist|gp|surgery|prescription|milk|bread|paper|post|parcel)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ADDRESSED(stdlib_call("re", "compile", &[py_str("^(?:ok\\s+|okay\\s+|right\\s+|so\\s+|hi\\s+|hello\\s+|hey\\s+|and\\s+)*(?:(?:can|could|would|will)\\s+you\\b|please\\b|kindly\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_BOOKING_THING(stdlib_call("re", "compile", &[py_str("\\b(?:table|taxi|cab|room|tickets?|flight|hotel|appointment|restaurant|booking)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MESSAGE_OR_A_FORM(stdlib_call("re", "compile", &[py_str("\\b(?:text|message|email|e-mail|letter|note|reply|voicemail|form|application|document)\\b|\\b(?:sign|fill)\\s+(?:this|it|that|in|out)\\b|\\bfill\\s+in\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEP_ME_POSTED(stdlib_call("re", "compile", &[py_str("\\bkeep\\s+me\\s+(?:posted|informed|updated|in\\s+the\\s+(?:loop|picture)|company)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_FOR_ME(stdlib_call("re", "compile", &[py_str("\\b(?:for me|on my behalf)\\b|^\\s*(?:please\\s+)?(?:book|reserve|order|call|ring|phone|text|email|buy|pay|send|save|pick|sign|keep|search)\\s+(?:me|us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REMIND_ME(stdlib_call("re", "compile", &[py_str("^(?:please\\s+|ok\\s+|okay\\s+|hi\\s+|hello\\s+)?remind me\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_REPORT_TAIL(stdlib_call("re", "compile", &[py_str("(?:\\b(?:and|but|then|so|because|till|until|when|while|whereas)\\b|[,;])\\s*(?:it|they|he|she|i|we|the|my|its|this|that|there|nothing|nobody|everything|someone|something)\\b|\\b(?:still|keeps|kept|never|always|wont|cant|cannot|doesnt|didnt)\\b|\\b(?:it'?s|its|it|they'?re|theyre|they|we'?re|we|i'?m|im|i|he'?s|she'?s|he|she|there)\\s+(?:not\\s+)?(?:been|was|were|is|are|got|get|gets|went|goes|gone|split|splits|came|comes|trips|cuts|leaks|floods|flooded|keeps|kept|stopped|broke|breaks|smells|smelt|died|works|worked|fails|failed|\\w+ing|full|empty|dirty|wet|broken|dead|stuck|blocked|loud|noisy|cold|hot|freezing|boiling|on|off|open|shut|late|early)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_CHOICE(stdlib_call("re", "compile", &[py_str("\\bor\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_OWN_STEP(stdlib_call("re", "compile", &[py_str("^\\s*(?:it|them|that|this|those|these|one)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_SETTING(stdlib_call("re", "compile", &[py_str("\\bgas\\s+mark\\s+\\d+|\\b\\d{2,3}\\s*(?:c\\b|\\u00b0|degrees)|\\b(?:at|on|to)\\s+\\d{2,3}\\b|\\b(?:for|about)\\s+\\d+\\s*(?:mins?|minutes|hours?|hrs?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_POLITE_ASK(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|right|so|hi|hello|hey|and|thanks|ta|now|sorry|please)\\s+)*(?:(?:any\\s+chance\\s+)?(?:can|could|would|will)\\s+(?:you|u)|any\\s+chance\\s+(?:you|u)\\s+(?:could|can|would|will))\\s+(?:please\\s+|possibly\\s+)?(?P<rest>[a-z0-9 ',.\\-?!]+)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTS_IN_A_DIARY(stdlib_call("re", "compile", &[py_str("\\b(?:put|add|save|write|note|stick|move)\\b.*\\b(?:in|on|into|to)\\s+(?:my|the)\\s+(?:calendar|diary|phone|notes?|list|fridge)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTS_ON_A_PHONE(stdlib_call("re", "compile", &[py_str("^(?:just\\s+|also\\s+)?(?:(?:call|ring|phone|text|message|contact|chase)\\b|(?:speak|talk)\\s+to\\s+(?!me\\b|us\\b))")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTS_AN_ERRAND(stdlib_call("re", "compile", &[py_str("^(?:just\\s+|also\\s+)?(?:(?:book|reserve|buy|pay|purchase|renew|cancel|collect|pick\\s+up|apply|set\\s+up|drop\\s+off)\\b|(?:get|ring|call|book|order)\\s+(?:me|us)\\s+(?:a|an|some)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_HANDS_IT_OVER_HERE(stdlib_call("re", "compile", &[py_str("\\b(?:call|ring|phone|text|message|email|send|post|contact|chase)\\s+(?:me|us)\\b(?!\\s+(?:a|an|some)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING_TO_GET(stdlib_call("re", "compile", &[py_str("\\b(?:(?:some|a bit of|a little)\\s+(?:time|rest|peace|quiet|space|slack)|attention|my thoughts|the steps|a break|a rest|a day|a night|it a day|it a night)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CAN_DO_ITSELF(stdlib_call("re", "compile", &[py_str("^(?:write|draft|rewrite|help|suggest|recommend|explain|remember|remind|tell|say|show|pick\\s+(?:one|which|between|the\\b)|choose|plan|make|give|keep|freeze|cook|reheat|bake|read|listen|work|sort|order(?!\\s+(?:me|us)\\b)|deal|spell|check\\s+(?:my\\s+)?(?:spelling|grammar|writing|english|essay|answer|answers|maths|sums|homework|work)|check\\s+this|check\\s+that|check\\s+it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTS_ON_A_PHONE_LINES(py_tuple(vec![py_str("I can't ring or message anyone for you - there's no phone on this end. Tell me who it is and what has to be said, and I'll help you find the words before you call."), py_str("No phone here, so the call can't be mine. Give me the who and the one thing you need from them, and we'll work out what to say.")]));
    s_ACTS_AN_ERRAND_LINES(py_tuple(vec![py_str("I can't book, buy or pay for anything from here, sorry - the doing is yours. What I can do is the choosing: the options, the order to do it in and the questions to ask. Which part?"), py_str("Nothing in the world is mine to do - no booking, no paying. Tell me what you're after and I'll narrow it down, or work out what to check before you commit.")]));
    s_ACTS_IN_A_DIARY_LINES(py_tuple(vec![py_str("I can't put anything in your calendar - I only hold it for this chat, and it goes when the chat does. Put it in your phone with a reminder; tell me the date and I'll help you plan around it."), py_str("Your calendar isn't something I can reach, sorry. Write it in your phone with a reminder, and say the day here and I'll help you get ready for it.")]));
    s_ACTS_ON_A_CLOCK_LINES(py_tuple(vec![py_str("No clock on this end - nothing here can ring or buzz. The timer in your phone's clock app does it in two taps; tell me what it's for and I'll help with the rest of it."), py_str("I can't set a timer or an alarm, sorry - I'm only up while you are typing. Set it on your phone and tell me the job it's for; that part I can help with.")]));
    s_ACTS_ON_A_DOCUMENT_LINES(py_tuple(vec![py_str("Nothing on this end can print, scan or send a file - no printer, no outbox. The print or share button on whatever you're reading this on does it; tell me what it is and I'll help with the rest."), py_str("That one needs a printer or an inbox, and neither is mine. Do it on your own device, and bring me the wording or the order to send things in.")]));
    s_ORDER_OPENER(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|right|so|hi|hello|hey|and|just|now|quick|quickly|please|ta|thanks)\\s+)*(?P<job>(?:text|message|whatsapp|sms|email|mail|ring|call|phone|facetime|contact|chase|book|reserve|order|buy|purchase|renew|collect|pick\\s+up|put|add|save|write|note|jot|set|wake|alarm|print|scan|photocopy|copy|download|upload|send|forward|share)\\b)(?P<tail>.*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_A_MESSAGE(stdlib_call("re", "compile", &[py_str("^(?:text|message|whatsapp|sms|email|mail|ring|call|phone|facetime|contact|chase)\\s+(?:my|our|the|his|her|their|a|an|\\w+'s)\\s+\\w+")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_A_BUY(stdlib_call("re", "compile", &[py_str("^(?:book|reserve|order|buy|purchase|renew|collect|pick\\s+up)\\s+(?:me\\s+|us\\s+)?(?:a|an|the|some|two|three|four|more|my|our|another)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_A_RECORD(stdlib_call("re", "compile", &[py_str("^(?:put|add|save|write|note|jot|stick|move)\\b.*\\b(?:in|on|into|to)\\s+(?:my|the)\\s+(?:\\w+\\s+){0,2}(?:calendar|diary|phone|notes?|list|app|reminders?|fridge|whiteboard)\\b|^(?:save|keep)\\s+(?:this|that|it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_A_CLOCK(stdlib_call("re", "compile", &[py_str("^(?:set|put)\\s+(?:a|an|the|my)\\s+(?:timer|alarm|countdown|reminder|stopwatch)\\b|^wake\\s+me\\b|^(?:set|put)\\s+(?:a|an|the)\\s+alarm\\b|^alarm\\s+me\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_A_DOCUMENT(stdlib_call("re", "compile", &[py_str("^(?:print|scan|photocopy|copy|download|upload|send|forward|share)\\s+(?:this|that|it|these|those|them|a|an|the|my|two|three)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_A_REPORT(stdlib_call("re", "compile", &[py_str("\\b(?:and|but|then|so|because|till|until|while)\\s+(?:it|they|he|she|i|we|the|my|its|there|nothing|nobody|everyone)\\b|\\b(?:yesterday|last\\s+night|last\\s+week|this\\s+morning|earlier|already|never|always|every\\s+day|usually|used\\s+to|didnt|didn't|did|keeps|kept)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_NOT_A_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:ambulance|paramedic|paramedics|999|911|112|fire\\s+brigade|police|cops|101|it\\s+a\\s+(?:day|night)|a\\s+day|a\\s+night|it\\s+off|quits)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THINGS_THAT_STOP(call_builtin("frozenset", &[py_tuple(vec![py_str("died"), py_str("dead"), py_str("broke"), py_str("broken"), py_str("collapsed"), py_str("failed"), py_str("stopped"), py_str("conked")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THINGS_THAT_STOP_AFTER_UP(call_builtin("frozenset", &[py_tuple(vec![py_str("gave"), py_str("given"), py_str("packed"), py_str("went")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WORDS_BETWEEN(call_builtin("frozenset", &[py_tuple(vec![py_str("has"), py_str("have"), py_str("had"), py_str("is"), py_str("was"), py_str("were"), py_str("s"), py_str("just"), py_str("now")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLEASURE_WORD(py_str("(?:fun|funny|joy|joyful|pleasure|enjoy(?:s|ed|ing|able)?|happy|happiness|smile|laugh|mood|worth(?:while|\\s+doing)?|interest(?:s|ed|ing)?|look\\s+forward\\s+to|content|exciting|excites?)"));
    s_NOTHING_AND_A_PLEASURE(stdlib_call("re", "compile", &[add(&add(&py_str("\\bnothing\\b[^.?]{0,28}?\\b"), &g_PLEASURE_WORD()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COMPARISON(stdlib_call("re", "compile", &[py_str("\\b(?:than|beats?|more\\s+than|as\\s+much\\s+as)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_NEGATED(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:no|not|never|dont|don'?t|doesnt|doesn'?t|cant|can'?t|cannot|stopped|without)\\b[^.?]{0,24}?\\b"), &g_PLEASURE_WORD()), &py_str("\\b[^.?]{0,20}?\\b(?:anything|nothing|much|things|life|it|myself|any\\s?more|anymore|at\\s+all)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLEASURE_GONE_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:fun|joy|pleasure|interest|point|enjoyment)\\b\\s+(?:has|have|had)\\s+(?:gone|gone\\s+right|all\\s+gone)\\s+out\\s+of\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_NOTHING(stdlib_call("re", "compile", &[py_str("\\bfeel(?:s|ing)?\\s+nothing\\b|\\bnothing\\s+matters\\b|\\bnothing\\s+(?:makes\\s+me\\s+)?feels?\\s+(?:good|right|nice|better)\\b|\\b(?:enjoy|enjoying|enjoyed)\\s+nothing\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLEASURE_ABOUT_ONE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:happy|glad|pleased|excited|thrilled|proud)\\s+about\\s+(?:it|that|this|them|him|her|the\\b|my\\b|his\\b|her\\b|their\\b|our\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THE_BOT_LINES(py_tuple(vec![py_str("Nothing to do it with, I'm afraid: I'm a program, so no body, no senses and no car keys. Ask me the thing you would ask a friend and I will take that half of it on."), py_str("No, and I would rather say so than guess. I am a program in a chat window, so what I can do is read, remember this conversation and think the question through with you."), py_str("Not at all - there is no body here. If it is something you are wondering about, ask it anyway: plenty of that I can still help with.")]));
    s_ABOUT_THE_BOT(stdlib_call("re", "compile", &[py_str("^(?:can|could)\\s+(?:you|u)\\s+(?:actually\\s+|even\\s+|really\\s+|still\\s+)?(?:drive|smell|taste|feel|see|hear|touch|breathe|eat|drink|swim|run|walk|dance|sing|laugh|cry|shiver|sweat|blink|wave|hold|hug|kick|jump|climb|fly)\\b(?:\\s+(?:anything|something|things?|pain|cold|the\\s+sun|a\\s+car|at\\s+all|either|yet|though))?\\s*(?:please|then)?\\s*\\?*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLEASURE_GONE_LINES(py_tuple(vec![py_str("That flatness is a miserable thing to carry, and it is not laziness or a mood to be talked out of. More than a couple of weeks of it is worth saying to a GP, and until then the smallest thing that helps is one thing you used to like, put back in this week on a day you pick. What did you look forward to a year ago?"), py_str("I'm sorry - when nothing gives pleasure the days go flat and everything takes effort, and waiting for it to lift is the one thing that does not work. Tell one person, and put one small piece of what you used to enjoy back on the calendar this week; if it has been weeks, say it to a GP. What did you like doing before this started?"), py_str("That is worth taking seriously rather than shaking off: nothing giving pleasure for weeks is something a GP wants to hear about, and it is not a failing to say so. Until then the things to try are small and concrete - one walk, one person, one thing you used to look forward to, this week. Which of those feels least impossible?")]));
    s__A_HURT_DONE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:just\\s+|nearly\\s+|think\\s+i\\s+)?(?:have\\s+|had\\s+|ve\\s+)?(?:broke|broken|fractured|cracked|dislocated|sprained|tore|torn|snapped|slipped|hurt|injured|done)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BODY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:back|knees?|legs?|feet|foot|ankles?|hips?|arms?|elbows?|wrists?|hands?|fingers?|thumbs?|shoulders?|ribs?|neck|toes?|heels?|shins?|calves|calf|thighs?|collarbone|jaw|nose|eyes?|head|spine|disc|kneecap|ligaments?|muscles?|tendons?|bones?|teeth|tooth|nail|shoulder blade|lip|lips|cheek|cheeks|chin|ears?|eyebrow|forehead|scalp)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HARM_HAD(stdlib_call("re", "compile", &[py_str("\\b(?:(?:i|we)\\s+(?:(?:have|had|has)\\s+been\\s+)?(?:have|had|got|caught|was given|were given|been given|been told|was diagnosed with|diagnosed with)|(?:i've|we've|ive|weve)\\s+(?:had|got|been given|been diagnosed with))\\s+(?:to have\\s+|to get\\s+)?(?:a|an|the|my|some|a few|a couple of|two|three|four|five|six|seven|eight|nine|ten|\\d+)?\\s*(?:concussion|concussions|stitches?|staples?|whiplash|pneumonia|sepsis|septicaemia|meningitis|covid|coronavirus|shingles|glandular fever|miscarriage|stillbirth|seizure|seizures|stroke|heart attack|blood clot|pulmonary embolism|breakdown|relapse|chemo|chemotherapy|radiotherapy)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_ILLNESS_WORD(py_str("(?:[a-z]+\\s+){0,2}?(?:infections?|bugs?|virus(?:es)?|flu|influenza|fever|colds?|cough|temperature|poisoning|disease|illness|ailment|condition|attack|migraines?|seizures?|sore throat|sickness|nausea|diarrhoea|diarrhea|rash|abscess|cyst|ulcer|thrush|gout|shingles|covid|coronavirus|malaria|tuberculosis|hepatitis|uti|dvt|copd|ibs|mrsa|strep|\\w+(?:itis|osis|aemia|emia|iasis|pox|virus))"));
    s__A_HARM_ILL(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:(?:i|we)\\s+(?:(?:have|had|has)\\s+been\\s+)?(?:have|had|got|caught|was given|were given|been given|been told|was diagnosed with|diagnosed with)|(?:i've|we've|ive|weve)\\s+(?:had|got|been given|been diagnosed with|caught))\\s+(?:to have\\s+|to get\\s+)?(?:a|an|the|my|some|a few|a couple of|two|three|four|five|six|seven|eight|nine|ten|\\d+)?\\s*"), &g__A_ILLNESS_WORD()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HARM_ILL_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:cold (?:one|beer|drink|water|shower|bath|snap|call|weather|night|morning|day|evening|outside|front|shoulder|hands|feet)|bugs? (?:in|on) (?:my |the )?(?:code|computer|laptop|phone|app|program|software|system)|virus (?:on|in) (?:my |the )?(?:computer|laptop|phone)|temperature (?:setting|gauge|dial|outside|gauge)|sickness (?:policy|pay|record|note)|in (?:good|no|great|perfect|top) condition|condition of (?:the|my) (?:car|house|road)|attack of (?:the )?(?:nerves|conscience)|fever pitch|cabin fever|spring fever)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HARM_IDIOM(stdlib_call("re", "compile", &[py_str("\\b(?:stroke of (?:luck|genius|good luck|the brush)|brush ?stroke|stroke of a pen|broke the ice|broke the news|broke even|broke the bank|broke a record|broke the rules|broke a promise|broke the silence|cracked a joke|cracked a smile|cracked on|cracked a window|slipped my mind|slipped away|tore a strip off|torn between)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HARM_KNOCK(stdlib_call("re", "compile", &[py_str("\\b(?:knocked|thrown|shoved|swiped|swept)\\s+off\\s+(?:my|our|his|her|their|the|a)\\b|\\bhit\\s+by\\s+a\\s+(?:car|van|lorry|truck|bus|taxi|bike|motorbike|cyclist)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_OUT_OF_SOCKET(stdlib_call("re", "compile", &[py_str("\\bcame out of (?:its|the|my) socket\\b|\\bout of (?:its|the) socket\\b|\\b(?:my|our|his|her|their|the)\\s+(?:shoulders?|knees?|kneecap|hips?|elbows?|jaw|ankles?|fingers?|thumbs?|wrist|toe)\\s+(?:came|comes|went|goes|popped|pops) out\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_IN_AGONY(stdlib_call("re", "compile", &[py_str("\\bin (?:absolute |sheer |real )?agony\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PART_WENT(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:back|knee|knees|leg|arm|shoulder|hip|ankle|neck)\\s+(?:just\\s+|suddenly\\s+)?(?:went|gone|gave way|gives way)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SOMEONE(py_str("(?:he|she|they|someone|somebody|anyone|(?:a|the)\\s+(?:man|woman|lad|bloke|chap|guy|girl|boy|teen|carer|nurse|neighbour|neighbor)|(?:my|his|her|their|our)\\s+\\w+)"));
    s__A_HURT_BY_SOMEONE(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\b"), &g__A_SOMEONE()), &py_str("\\s+(?:\\w+\\s+){0,2}?(?:hit|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|slapp?(?:ed|ing)|smack(?:ed|ing)?|beat|beats|beating|beaten|headbutt(?:ed)?|bit|bites|burn(?:ed|t|s)|chok(?:ed|ing)|strangl(?:ed|ing)|stabbed|stabs|raped|molested|groped|dragg?(?:ed|ing)|whipp?(?:ed|ing)|belt(?:ed|ing))\\s+(?:me|us)\\b|\\b")), &g__A_SOMEONE()), &py_str("\\s+(?:\\w+\\s+){0,2}?(?:hit|punched|kicked|slapped|grabbed|pinched|twisted|yanked|wrenched|bit|bitten)\\s+my\\s+\\w+|\\b")), &g__A_SOMEONE()), &py_str("\\s+(?:\\w+\\s+){0,2}?(?:push(?:ed|es|ing)?|shov(?:ed|es|ing)?)\\s+(?:me|us)\\s+(?:down|into|against|over|off|up|to the)\\b|\\b")), &g__A_SOMEONE()), &py_str("\\s+(?:\\w+\\s+){0,2}?(?:threw|throws|tossed|spat|spits?)\\b[^.?!]{0,25}?\\bat\\s+(?:me|us)\\b|\\b(?:held|holds|holding|put|puts|pointed|pointing|pressed|had|pulled)\\s+(?:a|an|the|his|her|their)\\s+(?:knife|gun|blade|scissors|hammer|bat|belt|bottle|glass|razor|needle|pillow|cushion)\\s+(?:to|at|against)\\s+(?:my|her|his|our)\\s+\\w+")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HURT_BY_SOMEONE_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:hit me up\\b|hit on me\\b|kicked me out\\b|beat me (?:at|in|to)\\b|bit my head off\\b|grabbed my (?:attention|eye)\\b|pushed (?:me|us) into \\w+ing\\b|burn(?:ed|t) me (?:out|a copy|a cd)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PART_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:(?:had|have|having|needed|need|getting|got)\\s+)?(?:to have\\s+|to get\\s+)?(?:a|an|my|two|some|the)\\s+(?:tooth|teeth|wisdom tooth|wisdom teeth|molars?|appendix|tonsils?|adenoids|gallbladder|gall bladder|kidney stone|cataract|splinter|ingrown nail|nail)\\s+(?:out|removed|taken out|pulled|drained)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PROGNOSIS(stdlib_call("re", "compile", &[py_str("\\b(?:been given|given|been told (?:i (?:have|'ve|have got|got)|there (?:is|are))|told i (?:have|'ve|have got|got))\\s+(?:about\\s+|only\\s+|maybe\\s+|perhaps\\s+)?(?:a|two|three|four|five|six|nine|ten|twelve|eighteen|a few|a couple of)\\s+(?:more\\s+)?(?:days|weeks|months|years)\\b|\\b(?:two|three|four|five|six|nine|ten|twelve|eighteen|a few|a couple of)\\s+(?:more\\s+)?(?:weeks|months|years)\\s+(?:left|to live|at most|maybe)\\b|\\b(?:terminal(?:ly)? (?:ill|illness|cancer)|palliative care|in a hospice|end of life care)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PROGNOSIS_THAT_IS_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:to pay|to repay|to clear|to save|to appeal|to file|to finish|to complete|notice|sentence|probation|tenancy|contract|lease|to build|to train|to study|to sell|to deliver|warranty|guarantee|to get out of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BREAK_THAT_IS_GOOD(stdlib_call("re", "compile", &[py_str("\\bbroke(?:n)?\\s+(?:a|the|my|our|his|her|their|new|world|personal|course|club|national)\\s+(?:\\w+\\s+)?(?:records?|ice|sweat|ground|habit|cycle|deadlock|stalemate|personal\\s+best)\\b|\\bbroke\\s+even\\b|\\bbroke\\s+through\\b(?!\\s+(?:the|a|an|my|our|his|her|their)\\s)|\\bbreak(?:ing)?\\s+a\\s+record\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BREAK_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("broke"), py_str("broken"), py_str("breaking"), py_str("breaks")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_A_FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:hurts?|hurting|stings?|stung|miss(?:es|ed)?|dreading|dreads|sad|upset|gutted|lonely|ach(?:e|es|ing|y)|cry(?:ing)?|broken|struggling|hate|worried|devastated)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEELING_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:hurts?|hurting|stings?|stung|(?:will|do|would|could)\\s+miss|miss(?:es|ed)\\b(?!\\s+\\?)|dreading|sad|upset|gutted|lonely|ach(?:e|es|ing|y)|cry(?:ing)?|broken|struggling|hate|worried|devastated|hard|tough|quiet|empty)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ROUTINE_NOT_SHAPE(stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*-?\\s*year\\s*-?\\s*old\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__UNDERTAKING(stdlib_call("re", "compile", &[py_str("\\b(?:for|with|about)\\s+(?:my|our|the|a|an|this|that)?\\s*([a-z']+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIST_ON_THE_TABLE(stdlib_call("re", "compile", &[py_str("\\b(?:too much|lots (?:on|of|to)|a lot (?:on|to)|so much|everything|list|lists|jobs|things|plates|spinning|juggling|overwhelmed|swamped|drowning|stretched|spread thin|deadlines?|behind|catch(?:ing)? up|workload)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:should|could|can|do|does|did|is|are|was|were|will|would|shall|what|when|where|which|who|why|how|any)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_PREFIX(stdlib_call("re", "compile", &[py_str("^(?:(?:no|nah|nope|sorry|ok|okay|wait|oops|actually|oh|right|and|but),?\\s+)*(?:i\\s+(?:mean|meant|said)|scratch\\s+that|wait\\s+no|no\\s+wait|i\\s+mean\\s+to\\s+say)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_THING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+[a-z]|\\b(?:i|we)\\s*(?:'ve|ve|have)\\s+(?:got\\s+)?(?:a|an|some|two|three|four|a few|another)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MOOD(stdlib_call("re", "compile", &[py_str("\\b(?:sad|miserable|unhappy|depressed|lonely|dejected|glum|sorry for itself|in the wars|seen better days)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEAR_MISS(stdlib_call("re", "compile", &[py_str("\n    \\b(?:almost|nearly|just\\s+about)\\s+\n    (?:\n        won\\b\n      | beat\\s+(?:him|her|them|it|everyone|the\\s+\\w+)\n      | broke\\s+(?:the|my|a)\\s+(?:\\w+\\s+)?(?:record|pb|best|streak)\\b\n      | passed\\b(?!\\s*(?:out|away|on|wind|water|blood|a\\s+stool)\\b)\n      | got\\s+(?:the|that|my)\\s+(?:job|place|part|role|promotion|record|pb)\\b\n      | got\\s+(?:it|in|through)\\b\n      | made\\s+it\\b\n      | qualified\\b\n      | came\\s+(?:first|second|third|top)\\b\n      | scored\\b\n    )\n"), bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEAR_MISS_REPLY(py_tuple(vec![py_str("So close. Was there a last bit that got away, or was it lost earlier than that?"), py_str("Almost is its own kind of annoying, being right there. What happened at the end?"), py_str("Ugh, that is the hardest way to miss something. Will there be another go at it?"), py_str("That is a hard one to sit with, because nearly is not the same as not good enough. What would have made the difference?")]));
    s__A_CORRECTION(stdlib_call("re", "compile", &[crate::gen::memory::facts::g_CORRECTION_LEAD(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKING_SOMETHING_BACK(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:ok|okay|right|so)\\s*[,.]?\\s*)?(?!no\\s+(?:one|body)\\b)(?:no|nope|wait|hold\\s+on|hang\\s+on|scratch\\s+(?:that|it)|forget\\s+(?:it|that)|my\\s+mistake|i\\s+(?:got|had)\\s+(?:that|it|this)\\s+wrong|i\\s+made\\s+a\\s+mistake|i'?m\\s+wrong|(?:sorry\\s*,?\\s*)?i\\s+(?:mean|meant))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|am|be|been|being|do|does|did|done|have|has|had|can|could|will|would|shall|should|must|might|may|its|it's|thats|that's|hes|he's|shes|she's|theyre|they're|we're|youre|you're|im|i'm|ive|i've)\\b|\\w+ed\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_THING_ITSELF(stdlib_call("re", "compile", &[py_str("^(?:the|these|those|this|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_AFTER_THE_OPENER(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|how|why|when|where|who|which|can|could|should|would|will|do|does|did|is|are|was|were|have|has|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_OPENER(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:ok|okay|right|so|no|nope|wait|ah|oh|sorry)\\s*[,.]?\\s*)+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HURT_SOMEBODY(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|the)\\s+(?:son|daughter|boy|girl|child|kid|baby|mum|mom|mother|dad|father|brother|sister|husband|wife|partner|boyfriend|girlfriend|friend|mate|neighbour|neighbor|grandad|granddad|grandma|gran|nan|nana|uncle|aunt|auntie|cousin|colleague|flatmate|housemate|carer|teacher|boss|fella|lass|lad|family)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HURT_BUT_FINE(stdlib_call("re", "compile", &[py_str("\\bbut\\s+(?:he|she|they|hes|shes|theys|it)(?:'s|\\s+is|\\s+was|\\s+seems)?\\s*(?:ok|okay|alright|fine|good|bruised|home)\\b|\\b(?:he|she|they)(?:'s|\\s+is|\\s+seems)\\s+(?:ok|okay|alright|fine)\\b|\\bnothing\\s+(?:broken|serious|bad)\\b|\\bnot\\s+(?:too\\s+bad|serious)\\b|\\bonly\\s+(?:a\\s+)?(?:bruise|scrape|scratch|graze)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HURT_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:collapsed|fell|fall|falls)\\b[^.!?]{0,12}?\\b(?:laughing|about|over\\s+laughing|ill|sick\\s+with\\s+laughter)\\b|\\b(?:fell|fallen)\\s+(?:out|for|in\\s+love|asleep)\\b|\\b(?:knocked|blown)\\s+(?:out|away)\\b(?!\\s+by\\s+(?:a|the)\\s+\\w+\\s*(?:car|bus))")]).unwrap_or_else(|e| exc::raise(e)));
    s_HURT_ON_THE_SPEAKER(stdlib_call("re", "compile", &[py_str("\\b(?:attacked|hit|punched|stabbed|kicked|pushed|thrown|strangled|bitten|bit|beat|beaten|burned|burnt|hurt|injured|raped)\\s+me\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HURT_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:died|dead|death|dying|passed\\s+away|killed|kills?|funeral|buried|suicide|took\\s+(?:his|her|their)\\s+own\\s+life)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOW_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:low|rough|bad|rubbish|crap|hard|tough|awful|terrible|dreadful|long|heavy|rotten|flat)\\s+(?:old\\s+)?(?:day|week|month|morning|afternoon|evening|night|shift)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOW_DAY_MINE(stdlib_call("re", "compile", &[py_str("^(?:so |and |but |ok |okay |well |god |ugh |honestly |anyway |yeah |yeh |hmm )*(?:i\\s+(?:had|have|'ve|ve)\\b|ive\\b|i\\s*'?m\\s+having\\b|i\\s+am\\s+having\\b|we\\s+(?:had|have|'ve|ve)\\b|weve\\b|(?:kind\\s+of\\s+|sort\\s+of\\s+|bit\\s+of\\s+|such\\s+an?\\s+)?a\\b|(?:today|yesterday|this\\s+(?:morning|afternoon|evening|week))\\s+(?:was|has\\s+been|is)\\b|really\\b|such\\s+an?\\b|what\\s+an?\\b|(?:low|rough|bad|rubbish|crap|hard|tough|awful|terrible|dreadful|long|heavy|rotten|flat)\\s+(?:old\\s+)?(?:day|week|month|morning|afternoon|evening|night|shift)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOW_DAY_NOT_MINE(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|it)\\s+(?:had|has|was|is)\\b|\\b(?:my|his|her|their|your)\\s+(?!day\\b|week\\b|month\\b|morning\\b|night\\b|evening\\b)\\w+\\s+(?:had|has|was|is)\\b|\\bthe\\s+\\w+\\s+had\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOVE_ONE(py_str("(?:mum|mom|mam|mother|dad|father|nan|nana|gran|granny|grandma|grandad|grandpa|sister|brother|son|daughter|kid|child|wife|husband|partner|girlfriend|boyfriend|fiance|fiancee|fiancé|fiancée|stepmum|stepmom|stepdad|stepson|stepdaughter|mother[- ]in[- ]law|father[- ]in[- ]law|son[- ]in[- ]law|daughter[- ]in[- ]law)"));
    s_LOVE_GROUP(py_str("(?:kids|children|family|families|parents|grandparents|grandkids|grandchildren|siblings|brothers|sisters|boys|girls|aunts|aunties|uncles|cousins|nieces|nephews|stepchildren|stepkids|in[- ]laws|friends|mates)"));
    s_LOVE_PERSON(add(&add(&add(&add(&py_str("(?:"), &g_LOVE_ONE()), &py_str("|")), &g_LOVE_GROUP()), &py_str(")")));
    s_LOVE_OPENER(py_str("^(?:oh |aw+ |ah |ha+ |lol |tbh |honestly |ok |okay |well |and |but |so )*(?:i|we)(?: really| just| absolutely| so| still| do| totally| truly)? love (?:my|our) "));
    s_LOVE_TAIL(py_str("(?: (?:so much|lots|loads|to bits|to pieces|more than anything|really|tbh|honestly|though|tho|too|also))*[.!]*$"));
    s_LOVES_SOMEONE(stdlib_call("re", "compile", &[add(&add(&g_LOVE_OPENER(), &g_LOVE_ONE()), &g_LOVE_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    s_LOVES_A_GROUP(stdlib_call("re", "compile", &[add(&add(&g_LOVE_OPENER(), &g_LOVE_GROUP()), &g_LOVE_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    s_LOVES_PEOPLE(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&g_LOVE_OPENER(), &g_LOVE_PERSON()), &py_str("(?: (?:and|&|n) (?:my |our |the )?")), &g_LOVE_PERSON()), &py_str(")+")), &g_LOVE_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    s_LOVE_START(stdlib_call("re", "compile", &[add(&add(&g_LOVE_OPENER(), &g_LOVE_PERSON()), &py_str("(?:[ ,'-]+[a-z0-9][^?.!]{0,60})?[.!]*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLOCK(py_str("(?:\\d{1,2}(?:[:.]\\d{2})?\\s*(?:am|pm)|(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s*(?:am|pm)|(?:\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s*o'?clock|half\\s+(?:one|two|three|four|five|six|seven|eight|nine|ten))"));
    s_A_TIME_SAID(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:at|by|for|before|around|about|until|till|from)\\s+(\\d{1,2}[:.]\\d{2}\\s*(?:am|pm)?|"), &g_A_CLOCK()), &py_str("|\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_THE_DEPARTURE(stdlib_call("re", "compile", &[py_str("\\bthe\\s+(\\d{1,2}[:.]\\d{2}\\s*(?:am|pm)?|\\d{1,2}\\s*(?:am|pm)|half\\s+past\\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\\s+(?:train|flight|bus|coach|ferry|boat|tram|tube|plane)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:need|needs|needed|have|has|had|got|must|ought)\\s+(?:to\\s+)?(?:be|get|leave|head|go|catch|make)\\b|\\b(?:train|flight|bus|coach|ferry|boat|taxi|shift|meeting|work|check-?in)\\s+(?:is|are|starts?|leaves?|departs?|takes?\\s+off|begins?|kicks?\\s+off)?\\b|\\b(?:be|get|wake)\\s+up\\b|\\bset\\s+(?:the|my|an)\\s+alarm\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LENGTH(stdlib_call("re", "compile", &[py_str("^\\s*(?:hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?|miles?|km|kms|people|quid|pounds|kilos?|kg|st[o]?ne|times|of\\s+them|more|less)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_NOT_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|been|missed|lost|yesterday|last\\s+night|this\\s+morning|cancelled|canceled|used\\s+to|usually|always|every\\s+day)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_ASKED(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|when|how|which|why|who|should|shall|can|could|would|do|does|did|is|are|was|were|have|has|any|is\\s+it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_ABOUT_THE_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:im|i'm|i am)\\s+an?\\b|\\bi\\s+work\\s+as\\b|\\bmy\\s+job\\b|\\bmy\\s+shift\\s+pattern\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their)\\s+(?:mum|mom|mam|mother|dad|father|wife|husband|partner|boyfriend|girlfriend|son|daughter|child|kid|brother|sister|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|niece|nephew|friend|mate|neighbour|neighbor|colleague|boss)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AT_THE_CLOCK(py_tuple(vec![py_str("Work it back from {at}: the journey, then getting ready, then ten minutes spare. What is the trip like at that hour?"), py_str("{At} gives you a time to work back from. How long does the journey take from yours?"), py_str("Count backwards from {at} and set the alarm by the time you have to walk out. What has to happen before you get out of the door?"), py_str("The leaving time is the thing to fix for {at}, not the arriving one. How long does it take from yours?"), py_str("{At} is a timetable rather than a rush, as long as the leaving time is right. Where are you travelling from?")]));
    s_OPTION_ON_THE_TABLE(stdlib_call("re", "compile", &[py_str("\\b(?:or|either)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTDOWN_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how long|how many (?:days|weeks|months|sleeps|more days)|how much longer|how far away)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTDOWN_TO(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:till|til|until|before|away|to go)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTDOWN_AFTER_CARE(py_tuple(vec![py_str("{count} That is a while to be counting towards, and you don't have to get through it on your own - what is tonight like?"), py_str("{count} If there is something about that date you are counting towards, say what it is and we'll take it a day at a time."), py_str("{count} A long stretch of days is hard to carry on your own. Who knows how you are doing tonight?")]));
    s_COUNTED(stdlib_call("re", "compile", &[py_str("^(?:that's |that is |it's |its )?(?:about |around |just |nearly |only |almost |another )?(?:\\d|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|a hundred)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REGRET_DOING(stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:have\\s+|had\\s+)?(?:did|done|said|been|was|am|feel|felt)\\s+(?:something\\s+)?(?:really\\s+|so\\s+|very\\s+|quite\\s+|a bit\\s+|pretty\\s+|such\\s+)?(?:stupid|dumb|thick|silly|bad|awful|terrible|horrible|horrid|wrong|mean|nasty|rude|selfish|unfair|pathetic|useless|worthless|guilty|ashamed|judged|daft|idiotic|moronic|embarrassing|cringe|a cow|a bitch|a jerk)\\b|\\bsomething\\s+(?:a\\s+bit\\s+|really\\s+|so\\s+|very\\s+|quite\\s+|pretty\\s+|such\\s+)?(?:daft|idiotic|moronic|cringe|awkward)\\b|\\b(?:did|done|made|make|said)\\s+a\\s+(?:really\\s+|so\\s+|very\\s+|quite\\s+|right\\s+|real\\s+|proper\\s+|bit\\s+of\\s+a\\s+)?(?:stupid|daft|dumb|thick|silly|idiotic|moronic|embarrassing|cringe|awful|terrible|bad)\\s+(?:thing|things|stuff|job)\\b|\\bi\\s+(?:have\\s+|had\\s+)?(?:lied|cheated|stole|shouted|lost\\s+my\\s+temper|wasted|ruined|messed\\s+up|screwed\\s+up|cocked\\s+up|botched|betrayed|snapped)\\b|\\b(?:cocked|balls(?:ed)?|buggered|botched)\\s+(?:it|things?|that)?\\s*up\\b|\\bput\\s+my\\s+(?:foot|feet)\\s+in\\s+it\\b|\\bmade\\s+a\\s+(?:pig'?s?\\s+ear|hash)\\s+of\\b|\\bi\\s+(?:have\\s+|had\\s+)?(?:made|make)\\s+(?:a\\s+|one\\s+)?(?:huge\\s+|big\\s+|silly\\s+|stupid\\s+|terrible\\s+|awful\\s+|bad\\s+|daft\\s+|dumb\\s+|thick\\s+)?(?:mistake|error)\\b|^\\s*(?:i\\s+)?(?:made|did)\\s+(?:a\\s+|an\\s+|some\\s+|the\\s+)?(?:\\w+\\s+){0,2}?(?:mistake|error|howler|blunder)\\b|\\bi\\s+(?:have\\s+|had\\s+)?let\\s+(?:him|her|them|everyone|everybody|my\\s+\\w+|someone|somebody)\\s+down\\b|\\bi\\s+(?:really\\s+)?(?:regret)\\b|\\bi\\s+shouldnt\\s+have\\s+(?:said|done|told|lied|shouted|yelled|snapped|eaten|drunk|bought|spent|sent|texted|signed|answered|opened|posted|shared|taken|stayed|called|let|been|gone)\\b|\\bi\\s+should\\s+not\\s+have\\s+(?:said|done|told|lied|shouted|yelled|snapped|eaten|drunk|bought|spent|sent|texted|signed|answered|opened|posted|shared|taken|stayed|called|let|been|gone)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_REGRET(stdlib_call("re", "compile", &[py_str("\\?|^(?:can|could|should|would|will|do|does|did|is|are|was|were|what|how|why|when|where|who|which|any)\\b|\\b(?:how\\s+(?:do|can|should)|what\\s+should|should\\s+i|shall\\s+i|do\\s+i|can\\s+i|tomorrow|next\\s+(?:time|week)|from\\s+now\\s+on|in\\s+future|want\\s+to|wanna|gonna|going\\s+to|i\\s+will|i'?ll|plan(?:ning)?\\s+to|(?:if|unless|whether)\\s+i)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SNAPPED_A_PHOTO(stdlib_call("re", "compile", &[py_str("\\bsnapped\\s+(?:a|another|some|two|three)\\s+(?:photo|photos|pic|pics|picture|pictures|selfie|shot|shots)\\b|\\blet\\s+(?:my|his|her|their|our)\\s+(?:hair|guard)\\s+down\\b|\\blet\\s+off\\s+steam\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MISTAKE(stdlib_call("re", "compile", &[py_str("\\b(?:mistake|error|messed\\s+(?:it\\s+)?up|screwed\\s+(?:it\\s+)?up)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_JOB_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:mistake|error|messed\\s+(?:it\\s+)?up|screwed\\s+(?:it\\s+)?up)\\b[^.]{0,24}?\\b(?:on|in|with|at|to)?\\s*(?:my|the|our|a|an)\\s+([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_JOB_BUT_A_PERSON(call_builtin("frozenset", &[mcall(&py_str("\n    mum mom mummy dad daddy mate mates friend friends son daughter wife husband partner fiance\n    boss team kid kids child colleague workmate sister brother nana gran granny nan neighbour\n    neighbor someone somebody him her them everyone everybody people lass lad girl boy man woman\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_DID_IT(stdlib_call("re", "compile", &[py_str("\\b(?:s?he|they|my (?:partner|husband|wife|boyfriend|girlfriend|fianc\\w*|ex|mate|friend|brother|sister|son|daughter|mum|dad|mother|father|boss|landlord|flatmate|housemate))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WRONG_PLAIN(stdlib_call("re", "compile", &[py_str("\\b(?:gambl\\w*|affair|seeing someone|texting (?:his|her|their) ex|back with (?:his|her|their) ex|drinking again|on the (?:booze|drugs) again|using again|owed? (?:me|us) money|gone behind my back|kept it from me|been unfaithful|cheat(?:ing|ed) on (?:me|us)|slept with (?:someone|somebody|my)|sent pictures)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WRONG_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:ly(?:ing|ed)|lied|lies|cheat\\w*|hiding|hidden|hides|stole|stolen|stealing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WRONG_AIMED(stdlib_call("re", "compile", &[py_str("\\s+(?:to|at|with|on)\\s+(?:me|us)\\b|\\babout\\s+(?:\\w+\\s+){0,2}(?:me|my|us|our|mine|ours)\\b|\\s+(?:money|his wages|her wages|the savings|the rent|the bills)\\b|\\s+(?:it\\s+)?from\\s+(?:me|us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WRONG_ABOUT_A_THING(stdlib_call("re", "compile", &[py_str("\\s+(?:to|at|in|on|over|about|with|for)\\s+\\S|\\s+(?:the|a|an|his|her|their|its)\\s+\\S")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOUND_IT_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:found|find|turned|turn)\\s+(?:it\\s+)?out\\b|\\b(?:turns|turned)\\s+out\\b|\\b(?:just\\s+)?(?:heard|learned|learnt|discovered|found)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOUND_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:found|find|turned|turn) (?:it )?out\\b|\\b(?:turns|turned) out\\b|\\b(?:just )?(?:heard|learned|learnt|discovered|found)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|last night|yesterday|this morning|this afternoon|this evening|at the weekend|over the weekend|this weekend|last weekend|all day|all morning|the other day|earlier)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:i|i'm|im|ive|i've|we|we're|weve|we've|my|our|it|work|school|the weather|the day)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:had|got|saw|was|were|went|did|made|took|met|ate|drank|bought|brought|read|played|watched|walked|cooked|cleaned|painted|cut|sorted|finished|started|spent|sat|ran|came|left|gave|found|booked|visited|helped|learned|learnt|worked|slept|drove|caught|wore|won|felt|heard|told|said|sold|paid|sent|kept|put|let|set|fell|held|stood|thought|knew|began|broke|spoke|rang|sang|wrote|built|swam|saw|ran)\\b|\\b(?:(?!red\\b|bed\\b|shed\\b|need\\b|feed\\b|bleed\\b|speed\\b|weed\\b|seed\\b|breed\\b|proceed\\b|succeed\\b|exceed\\b|indeed\\b|hundred\\b|greed\\b|creed\\b)\\w+ed)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SYMPTOM_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:hot|warm)\\s+(?:and\\s+)?(?:red|redder|swollen|tender|puffy|angry)\\b|\\bred(?:ness)?\\s+(?:and\\s+)?(?:hot|warm|spreading|swollen|tender|angry|streaky)\\b|\\bhot to (?:the )?touch\\b|\\bweeping\\b|\\boozing\\b|\\boozed\\b|\\bpus\\b|\\bswollen (?:and|or) (?:red|hot|tender)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DAY_DONE(stdlib_call("re", "compile", &[py_str("\\b(?:tomorrow|next (?:week|weekend|month|year|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|gonna|going to|i will|i'll|ill|want to|wanna|plan to|planning to|should i|shall i|can i|could i|do i|did i|have i|am i|are we|named|renamed)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVE_GOT_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:['\\u2019]?ve|['\\u2019]?d|\\s+have)\\s+(?:only\\s+|just\\s+|still\\s+|already\\s+|now\\s+)?(?:got|gotten)\\b(?!\\s+to\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CROWD(stdlib_call("re", "compile", &[py_str("\\b(?:everyone|everybody|everyone'?s|everybody'?s|other people|others|the only one|only one|nobody else|no one else|no-one else|(?:everyone|everybody|people|friends|mates)\\s+(?:i know|around me|my age|else|at work|from school|from uni|from work|these days)|all (?:of )?my (?:mates|friends|family|cousins|brothers|sisters|colleagues|workmates)|people my age|people of my age|my age)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MILESTONE(stdlib_call("re", "compile", &[py_str("\\b(?:behind|ahead of|sorted|sorted out|settled|milestone|milestones|bought|buying|own(?:s|ed)? a|house|houses|flat|mortgage|babies|baby|kids|children|married|wedding|engaged|pregnan\\w*|career|promotion|their lives|getting on with it|moved on|all sorted)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_CROWD_AHEAD(stdlib_call("re", "compile", &[py_str("\\b(?:hate|hates|hated|bully|bullied|bullying|picks? on|picked on|leaves? me out|left me out|laughs? at|laughed at|ignores? me|ignored me|mean to me|rumours?|gossip|happy for (?:us|me)|proud of me|delighted for|congratulat\\w*|well done|coming round|coming over|round tonight|tonight|tomorrow|next week|next month|next year|this weekend|party|gonna|going to|we'?re all|we are all|shall we|should we|any tips|how do i|what should|am i|only one (?:who|doing|that|bothered))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CROWD_ORDER(stdlib_call("re", "compile", &[py_str("\\b(?:behind|ahead of|sorted|settled|caught up|only one|but me|all (?:of )?my (?:mates|friends|family|cousins|brothers|sisters|colleagues|workmates)|around me|my age|people my age|(?:and )?i(?:'?m| am)? (?:havent|haven't|have not|aint|ain't|am not|dont|do not)|i'?m not|im not|and i'?m still|still not|without)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CROWD_AHEAD(py_tuple(vec![py_str("That comparison lands on nearly everybody, and it is not a fair one: you are seeing their headline and your own whole week. What would you actually like for yourself?"), py_str("The people who look sorted are mostly just further into their own thing. What is the next bit you would choose for yourself, if nobody were watching?"), py_str("It is worth pulling those two apart: which of those things do you actually want, and which do you only want because they have it?"), py_str("Everyone's timetable is their own and yours is not late, it is just not the same one. What would a good year look like for you?"), py_str("Nobody posts the part where it is going badly. What is the bit of your own you would want to change first?"), py_str("That is the comparison that makes everybody feel behind, and it has never once had the full picture. What would you want to be true for yourself a year from now?")]));
    s_A_HARD_DAY_MOOD(py_tuple(vec![py_str("angry"), py_str("sad"), py_str("tired"), py_str("worried"), py_str("stressed"), py_str("upset")]));
    s__A_NEGATIVE_AUXILIARY(py_str("not|never|no|isn['’]?t|aren['’]?t|don['’]?t|didn['’]?t|wasn['’]?t|weren['’]?t|can['’]?t|couldn['’]?t|won['’]?t|hardly|cannot|haven['’]?t|hasn['’]?t|hadn['’]?t|wouldn['’]?t|shouldn['’]?t|mustn['’]?t|needn['’]?t|ain['’]?t|daren['’]?t|shan['’]?t|oughtn['’]?t"));
    s_NOT_A_DAY_SAID(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:"), &g__A_NEGATIVE_AUXILIARY()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TELLING_OF_WRONG(py_str("ly(?:ing|ed)|lies|lie\\b|cheat\\w*|hiding|hidden|stole|stealing|steals|gambl\\w*|affair|seeing someone|texting (?:his|her|their) ex|back with (?:his|her|their) ex|drinking again|on the (?:booze|drugs) again|using again|owed? (?:me|us) money|borrowed money|spent (?:it|the|our|my) .{0,20}money|gone behind my back"));
    s_A_BETRAYAL_FOUND_OUT(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:i |we )?(?:just |only |now )?(?:found|find|findin[g]|turned|turn) (?:it )?out\\b[^.!?]{0,45}\\b(?:"), &g__A_TELLING_OF_WRONG()), &py_str(")\\b|\\b(?:turns|turned) out\\b[^.!?]{0,45}\\b(?:")), &g__A_TELLING_OF_WRONG()), &py_str(")\\b|\\b(?:i )?(?:just )?(?:heard|learned|learnt|discovered)\\b[^.!?]{0,40}\\b(?:s?he|they|my \\w+) (?:has been|had been|was|were|is|cheated|lied|stole)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ASKING(py_str("asked|told|texted|messaged|phoned|rang|called|emailed|chased|reminded|warned|complained|spoke|talked|written|wrote"));
    s__AN_ATTEMPT_COUNT(py_str("twice|three times|four times|five times|repeatedly|again|once more|for the (?:second|third|fourth|fifth) time"));
    s_AN_ATTEMPT_AT_SOMEONE(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("\\b(?:"), &g__AN_ASKING()), &py_str(")(?: to)?\\s+(?:him|her|them|us)\\b|\\b(?:")), &g__AN_ASKING()), &py_str(")\\b(?:\\s+\\w+){0,4}?\\s+(?:")), &g__AN_ATTEMPT_COUNT()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSE_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their)\\s+(?:mum|mam|mom|mother|dad|father|son|daughter|wife|husband|partner|boyfriend|girlfriend|brother|sister|mate|friend|boss|marriage|relationship|nan|nana|gran|granny|grandad|grandma|uncle|aunt|cousin|neighbour|neighbor|child|kid|baby|mother-in-law|father-in-law|in-laws|ex|ex-wife|ex-husband|ex-partner|ex-boyfriend|ex-girlfriend|flatmate|housemate|roommate|landlord|stepson|stepdaughter|stepmum|stepdad)\\b|\\b(?:someone|somebody|a man|a woman|a bloke|a lad)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILDHOOD_MEMORY(stdlib_call("re", "compile", &[py_str("\\bwhen i was (?:\\d+|little|young|a kid|a child|at school|small)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|ive|i've|we|we're|weve|we've)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:came (?:round|over|by|in)|come (?:round|over)|coming (?:round|over|by)|popped (?:round|in|over)|stopped by|visiting|visited|rang|phoned|called|texted|facetimed|stayed over)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_MODIFIER(call_builtin("frozenset", &[mcall(&py_str("a an the again already too as well anyway now so just really instead back then still yet before either though even also all of it in on at to my me myself i we our for with"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_GOT_AHEAD(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:'?ve| have)?\\s+(?:just\\s+|finally\\s+)?(?:got|booked|secured|bagged|landed|sorted)\\s+(?:myself\\s+|us\\s+)?(?:a|an|the|my|our|two|three|four|some|another|front row|cheap)?\\s*(?:audition|tickets?|viewing|viewings|booking|slot|reservations?|table booked)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GOT_AHEAD_GONE(stdlib_call("re", "compile", &[py_str("\\b(?:yesterday|last (?:night|week|month|year|saturday|sunday|monday|tuesday|wednesday|thursday|friday)|this morning|earlier|the other day|on (?:mon|tues|wednes|thurs|fri|satur|sun)day)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONTH(stdlib_call("re", "compile", &[py_str("\\b(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sept|sep|oct|nov|dec|weekend after next|the \\d+(?:st|nd|rd|th))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOT_AHEAD_NOT_A_DATE(stdlib_call("re", "compile", &[py_str("\\b(?:parking|speeding|penalty|fine|fined|clamped|bus|train|plane|coach|tube|ferry|tram|lottery|raffle|on the way)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PROBLEM_STATED(stdlib_call("re", "compile", &[py_str("^(?:so |ok |okay |right |and |hi |hiya |hello |hey |um |er |honestly |well |but )*(?:(?:i|we)(?:'?ve|'?m| have| am| got|'?ve got)\\s+|there (?:is|'?s) |theres |its |it'?s )?(?:a |an |one )?(?:(?:bit of a|small|big|massive|real|proper|slight|little|serious|tiny|huge|nasty|right|bit of|wee|awkward) )?(?:problem|issue|trouble)(?: (?:here|really|actually|now|again|too|i (?:need|could do with) (?:some )?help(?: with)?|that i (?:need|could do with) (?:some )?help(?: with)?|i (?:need|could do with) (?:some )?advice))?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMETHING_UP(stdlib_call("re", "compile", &[py_str("^(?:so |ok |okay |right |and |hi |hello |hey |um |er |honestly |well |but )*(?:(?:there (?:is|'?s) |theres )?(?:(?:something|somethings|sommat|summat)(?: really| bad| weird| odd| serious| not right)?(?:'?s)? )?(?:is |was |keeps )?(?:wrong|up|not right|off|the matter)|(?:something|somethings|sommat|summat)\\s+(?:has |had |just )?(?:happened|gone wrong|come up)|(?:something|somethings|sommat|summat)\\s+(?:is |was |keeps )?(?:bothering|worrying|upsetting|bugging|nagging)\\s+me)(?: (?:here|today|yesterday|tonight|last night|this morning|this week|at work|at home|again|right now|at the moment)){0,2}$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WEAR_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|how)\\b[^.?!]{0,40}?\\b(?:wear|wearing|dress code|outfits?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEAR_WHEN_NOTHING_ANSWERS(py_tuple(vec![py_str("Clothes come down to the weather and how long you are standing about: a layer you can take off, and shoes you can spend the day in. Is it indoors or out?"), py_str("Go off the room rather than the label - one notch up from what most people there will have on, and a layer to shed if you are the only one dressed up. Cold where you are?"), py_str("Dress for the coldest hour of it and the longest you will be on your feet: shoes decide more of a day than people expect. Are you inside or out?")]));
    s_NOTHING_TO_WEAR(stdlib_call("re", "compile", &[py_str("\\b(?:nothin|nothing|nowt)\\b\\s+(?:at all\\s+)?to\\s+(?:wear|put on)\\b|\\b(?:nothin|nothing|nowt)\\s+fits\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_TO_WEAR_REPLIES(py_tuple(vec![py_str("Nothing to wear usually means nothing you fancy - pick the one thing you feel good in and build the rest round it. What is the day?"), py_str("Everyone has nothing to wear the night before. Is it work, a meal out, or a long walk?"), py_str("Then wear the one you always skip and keep everything else plain. What is it for?")]));
    s_HOT_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d{2})\\s*(?:degrees?|°|c\\b)|\\b(?:minus|below)\\s+(?:\\d{1,2}|zero)\\b|\\b(?:heat|heatwave|warm spell)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEATHER_SUN(stdlib_call("re", "compile", &[py_str("\\b(?:sunburn\\w*|sunbathe?\\w*|sunny|sun cream|sunscreen|factor\\s*(?:15|20|30|50)|beach|blazing|scorching sun)\\b|\\bburn(?:s|t)? in (?:\\w+ )?minutes?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COLD_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:freezing|frost|snow|sleet|chilly|bitter(?:ly)?)\\b|\\b(?:minus|below)\\s+(?:\\d{1,2}|zero)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BAD_THING_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:bad|sad|awful|terrible|dreadful|grim|rotten|lousy|horrible|horrid|rubbish|tough)\\s+(?:news|tidings)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAND_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:asked|ask|asking|looked|looking|reached|reaching)\\s+(?:for|out for)\\s+(?:help|support|a hand|some help)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_WENT_WRONG(stdlib_call("re", "compile", &[py_str("\\b(?:burgl\\w*|broken into|broke into|break-in|break in|robbed|robbery|mugged|mugging|stolen|stole|theft|shoplifted|nicked|pinched|keyed|vandalised|vandalized|scratched the car|written off|wrote it off|wrote the car off|caught fire|went on fire|on fire|burst pipe|water damage|smoke damage|gone missing|went missing|disappeared|vanished|crash|car crash|had a prang|a prang|in a smash|was rear-ended|rear-ended|hit by a car|fell off|come off my bike|came off my bike|fell down the stairs)\\b|\\bcrash(?:ed)?\\b(?!\\s+out)\\b|\\bflood(?:ed)?\\b(?!\\s+with)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SYMPATHY(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|condolences|sad news|real loss)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BODY_UNWELL(stdlib_call("re", "compile", &[py_str("\\b(?:ill|illness|unwell|poorly|sick|infection)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DETAIL_OPENING(stdlib_call("re", "compile", &[py_str("^(?:it|that|this|there)\\s+(?:was|were|is|had|happened)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LOOKING_BACK(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|but\\s+|so\\s+|well\\s+)?(?:like\\s+|as\\s+)?i\\s+(?:already\\s+|did\\s+)?(?:said|told\\s+you|mention\\w*)\\b[^.!?]{0,50}?\\b(?:it|that|this)\\s+(?:only\\s+|just\\s+|actually\\s+|never\\s+)?(?:was|were|is|had|started|began)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_DONE_WRONG(stdlib_call("re", "compile", &[py_str("\\b(?:called (?:me|him|her|them|us) names|call (?:me|him|her|them|us) names|called names|name[- ]calling|called (?:me|him|her|them|us) a \\w+ name|got called names|got called (?:a |an |the )?(?:\\w+ )?names?\\b|(?:was|were|is|are|being) called (?:a |an |the )?(?:\\w+ )?names?\\b|teased (?:me|him|her|them|us)|teasing (?:me|him|her|them|us)|bullied|bullying|bullies|picked on|picking on|picked last|picking (?:me|him|her|them) last|left (?:me|him|her|them|us) out|leaving (?:me|him|her|them|us) out|laughed at (?:me|him|her|them|us)|laughing at (?:me|him|her|them|us)|made fun of (?:me|him|her|them|us)|making fun of (?:me|him|her|them|us)|mean to (?:me|him|her|them|us)|nasty to (?:me|him|her|them|us)|horrible to (?:me|him|her|them|us)|(?:would|wouldnt|wouldn't|wont|won't|didnt|didn't) let (?:me|him|her|them) play|nobody (?:will|would|wants to) play with|snatched (?:it|the \\w+) (?:off|from) (?:me|him|her|them))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BAD_DAY_ANYWAY(stdlib_call("re", "compile", &[py_str("\\b(?:hospital|a&e|ambulance|fell over|a fall|funeral|died|death|test results|biopsy|crash|crashed|accident|collision|flood|flooded|burst pipe|fire|burgled|robbed|stolen|wrote off|killed|killing|(?:massive|big|huge|proper|right|awful|terrible) row|row (?:about|over|with)|chemo|miscarriage|evicted|lost my job|sacked|fight|row with|panic|anxiety attack|ticket|morphine|diamorphine|midazolam|syringe driver|palliative|end of life|hospice|terminal|clamped|fined|argued|argument|shouted|cried|upset|worried|worries|sick|threw up|migraine|rained|soaked|drenched|wet through|late for|missed my|missed the|clinic|dentist|surgery|wrong|mistake|messed up|screwed up|waited|waiting|queued|queue|on hold|hung around|hung about|two hours|three hours|four hours|an hour|all night|headache|aching|exhausted|shattered|knackered|gutted|a row|an argument|strange|grim|full on|fell flat|meh|came\\s+off|smashed|smash|flooded|flooding|stacked\\s+it\\b|stacked\\s+(?:my|the|his|her|their)\\s+(?:bike|motorbike|moped|scooter|motorcycle|car|van|horse)\\b|wrecked|dented|chipped|snapped\\s+(?:at|my|the|his|her|their)\\b|went up in flames|caught fire|blew up|blew a fuse|went bang|collapsed|fell apart|packed in|conked out|stopped working|wouldnt start|wouldn't start|broke down|leaking|came back (?:short|lower|worse)|under (?:our|the) (?:offer|asking)|over budget|went over budget|(?:is|are|was|were) (?:still )?closed|told\\s+[^,.;!?]{0,25}?\\boff\\b|had a go at me|had a right go|boot on (?:my|the|our) (?:car|van|motor|bike)\\b|fell in (?!love)|choked(?!\\s+up)|short of breath|blackened|burnt|burned|scorched|stained|rusted|scratched|slipping up|slipped up|messed up|mucking up|mucked up|knocked\\s+over|tipped\\s+over)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOOD_TOPICS(call_builtin("frozenset", &[py_tuple(vec![py_str("cooking"), py_str("food-and-drink")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKES_AS_IT_COMES(g_takes_as_it_comes());
    s_A_NAME_IS_NOT_A_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\s+(?:called|named)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAMES_TOLD(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:(?:new|old|little|big|eldest|youngest|middle|first|second|step|ex|late)\\s+)?(?P<person>[a-z][a-z'-]{1,20})\\s+(?:is|are|was|were)\\s+(?:called|named)\\s+(?P<names>[a-z][a-z'-]{0,20}(?:\\s*(?:,|and|\\s)\\s*[a-z][a-z'-]{0,20}){0,3})(?:\\s+(?:and|,)?\\s*(?:she|he|they|it)s?\\b[^.!?]{0,80})?\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_A_HOUSEHOLD_TAKES(stdlib_call("re", "compile", &[py_str("\\b(?:we|they)\\s+(?:are|were)\\s+(?:called|named)\\s+(?P<names>[a-z][a-z'-]{0,20}(?:\\s*(?:,|and|\\s)\\s*[a-z][a-z'-]{0,20}){0,3})\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NAME_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("it"), py_str("its"), py_str("that"), py_str("this"), py_str("him"), py_str("her"), py_str("them"), py_str("me"), py_str("after"), py_str("something"), py_str("nothing"), py_str("whatever"), py_str("what"), py_str("which"), py_str("who"), py_str("but"), py_str("and"), py_str("she"), py_str("hes"), py_str("he"), py_str("they"), py_str("we"), py_str("us"), py_str("his"), py_str("hers"), py_str("theirs"), py_str("is"), py_str("was"), py_str("were")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_AFTER(stdlib_call("re", "compile", &[py_str("\\b(?:called|named)\\s+(?:(?:him|her|them|it|me|us)\\s+)?after\\s+(?P<who>me|us|you|i|(?:my|his|her|their|our)\\s+[a-z-]+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POSSESSIVE_OF_MINE(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_NOT_A_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:self[- ]?employed|unemployed|retired|married|divorced|engaged|separated|pregnant|based|interested|tired|exhausted|worried|bothered|pleased|delighted|annoyed|scared|frightened|settled|sorted|done|on a diet|on benefits|in debt|in credit|on the dole|a vegetarian|a vegan|on medication|on tablets)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|been|is|are|am|im|i'm|its|it's)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_THEN(stdlib_call("re", "compile", &[py_str("\\b(?:last (?:year|summer|winter|spring|autumn|month)|years ago|used to|back then|when i was|in the past)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_IN_THE_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:was|were)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_THAT_FELL_THROUGH(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|is|are|had|have)\\s+(?:meant|supposed)\\s+to\\s+[a-z]+")]).unwrap_or_else(|e| exc::raise(e)));
    s_RESULT_WENT_WRONG(stdlib_call("re", "compile", &[py_str("\\b(?:came out|comes out|turned out|turns out|came back|comes back|ended up|went|goes)\\s+(?:a bit\\s+|a little\\s+|all\\s+|really\\s+|quite\\s+|so\\s+)?\\s*(?:grey|gray|dry|soggy|burnt|burned|raw|tough|dense|rubbery|pale|lumpy|stodgy|stale|bland|wrong|ruined|awful|horrible|terrible|overdone|undercooked|flat|sour|bitter|curdled|split|seized)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BOT_WAS_SORRY(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|that sounds (?:rough|awful|horrible|hard|heavy|tough)|that is (?:rough|awful|hard|heavy|tough)|that's (?:rough|awful|horrible|hard|heavy|tough)|rough one|hard one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_SAID_BADLY(stdlib_call("re", "compile", &[py_str("\\b(?:bad day|bad one|rough day|rough one|rubbish day|rubbish|awful day|horrible day|terrible day|hard day|hard one|long day|tough day|tough one|strange day|grim|draining|it kicked off|wiped out|hectic|full on|knackered|exhausted|shattered|gutted|a row|an argument)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOING_GONE_BADLY(stdlib_call("re", "compile", &[py_str("\\b(?:badly|too\\s+much|too\\s+many|too\\s+often|not\\s+enough|nothing\\s+but|binged|overdid\\s+it|junk|crap|rubbish)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_SKIPPED(stdlib_call("re", "compile", &[py_str("\\bskipped\\b(?:\\s+\\w+){0,4}\\s+(?:all\\s+(?:week|month|weekend)|this\\s+week|last\\s+week|lately|every\\s+(?:day|morning|week)|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEAL_SKIPPED(stdlib_call("re", "compile", &[py_str("\\bskipped\\b(?:\\s+\\w+){0,2}\\s+(?:a\\s+|any\\s+|my\\s+|the\\s+)?(?:breakfast|lunch|lunches|dinner|dinners|tea|supper|meals?|food)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BEING_DONE_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|he|she|they|it|you|the|my|our|his|her|their|\\w+s)\\s+(?:am|is|are|was|were|been)\\s+being\\b|\\b(?:i'm|im|we're|they're|he's|she's|it's)\\s+being\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DECISION_NOT_A_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:approved|granted|confirmed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WISH(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:really\\s+|just\\s+|so\\s+|do\\s+)?wish\\b|\\bif only\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUM_STATED(py_tuple(vec![py_str("{said} on its own tells me nothing about whether it is high or low. What were you paying before, or what has someone else been quoted for the same thing?"), py_str("Put {said} beside the other figure for the same job - what you were paying before, or what a friend was quoted - and then it means something. What have you got to compare it with?")]));
    s_PRICE_NOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("paid"), py_str("pay"), py_str("pays"), py_str("paying"), py_str("cost"), py_str("costs"), py_str("costing"), py_str("charged"), py_str("charge"), py_str("charging"), py_str("quote"), py_str("quoted"), py_str("quoting"), py_str("price"), py_str("priced"), py_str("fee"), py_str("fees"), py_str("bill"), py_str("bills"), py_str("invoice"), py_str("deposit"), py_str("fare"), py_str("fares"), py_str("spent"), py_str("spend"), py_str("spending"), py_str("wants"), py_str("wanting"), py_str("asking")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_THING_SAID(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("me"), py_str("my"), py_str("we"), py_str("us"), py_str("our"), py_str("it"), py_str("its"), py_str("it's"), py_str("that"), py_str("thats"), py_str("that's"), py_str("this"), py_str("they"), py_str("them"), py_str("their"), py_str("he"), py_str("she"), py_str("his"), py_str("her"), py_str("was"), py_str("were"), py_str("is"), py_str("are"), py_str("be"), py_str("been"), py_str("a"), py_str("an"), py_str("the"), py_str("and"), py_str("but"), py_str("so"), py_str("then"), py_str("now"), py_str("just"), py_str("only"), py_str("about"), py_str("around"), py_str("at"), py_str("for"), py_str("of"), py_str("to"), py_str("in"), py_str("on"), py_str("with"), py_str("from"), py_str("much"), py_str("many"), py_str("um"), py_str("err"), py_str("well"), py_str("really")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_IN_NOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("bonus"), py_str("bonuses"), py_str("refund"), py_str("refunds"), py_str("rebate"), py_str("rebates"), py_str("payout"), py_str("payouts"), py_str("pay"), py_str("wages"), py_str("salary"), py_str("earnings"), py_str("income"), py_str("pension"), py_str("pensions"), py_str("winnings"), py_str("commission"), py_str("dividend"), py_str("dividends"), py_str("raise"), py_str("rise"), py_str("backpay"), py_str("arrears")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_IN_SHAPES(stdlib_call("re", "compile", &[py_str("\\b(?:paid|pay|pays|paying|gave|give|gives|giving|refunded|reimbursed|repaid)\\s+(?:me|us)\\b|\\b(?:i|we)\\s+(?:was|were|am|are|got|get|have|had)\\s+(?:paid|given|refunded|reimbursed|awarded)\\b|\\b(?:i|we)\\s+(?:earned|earn|made|make|won|win|sold)\\b[^.]{0,24}?\\bfor\\b|\\b(?:sold|selling)\\b[^.]{0,24}?\\bfor\\b\\s*[\\u00a3$\\u20ac]?\\d|\\b(?:got|get|gets|getting)\\s+[\\u00a3$\\u20ac]?\\d+\\s+(?:for|from|back)\\b|\\b(?:pay|wages|salary|income|pension)\\s+(?:went|goes|has gone|rose|increased|is up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_ASKING_WORD(stdlib_call("re", "compile", &[py_str("^(?:did|do|does|have|has|had|are|is|was|were|can|could|would|will|should|shall|what|whats|how|why|when|where|who|which|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORTED_NOT_LIVED(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:saw|seen|heard|read|watched)\\s+(?:that\\b|there(?:'s|s| is| are)\\b|on\\s+the\\s+\\w+|in\\s+the\\s+\\w+|the\\s+\\w+|somebody\\b|someone\\b|they\\b)|\\b(?:on|in)\\s+the\\s+(?:news|paper|radio|telly|local news)\\b|\\bthey\\s+say\\b|\\bive\\s+been\\s+told\\b|\\b(?:says|said|told\\s+me)\\s+(?:that\\b|there(?:'s|s| is| are)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEWS_IS_A_CHANGE(stdlib_call("re", "compile", &[py_str("\\b(?:clos(?:e|es|ed|ing|ure)|cancel\\w*|chang(?:e|es|ed|ing)|charg(?:e|es|ed|ing)|rules?|ban(?:s|ned|ning)?|cuts?|ris(?:e|es|ing)|increas(?:e|es|ed|ing)|delay\\w*|roadworks?|strikes?|walkout|fees?|prices?|tax(?:es)?|fines?|shut|new\\s+(?:rules|charges|prices|fares)|from\\s+(?:monday|next\\s+week|january|april))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEWS_IS_WEATHER(stdlib_call("re", "compile", &[py_str("\\b(?:storm|storms|gale|gales|wind|winds|windy|rain|snow|ice|sleet|flood|floods|freeze|freezing|frost|heatwave|thunder|lightning|hurricane|tornado|weather)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _body_state(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut index: Py = Py::None;
    let mut part_state: Py = Py::None;
    let mut start: Py = Py::None;
    let mut state: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Which body state the whole line is (\"cold\", \"hot\", \"hungry\", \"thirsty\"), or None.\n\n    Only a line that is the state and nothing else comes here: \"im cold\" is this reading, and\n    \"im cold water\" and \"im boiling eggs\" are things rather than states. A question is not a\n    report of one (\"im cold?\" is a person checking they said it), and a line with \"but\" in it\n    is a mixed one the trouble and feeling pools answer better.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str(" but "))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str(" not "))) } } } }) {
    return Py::None;
    }
    flat = mcall(&mcall(&clean.clone(), "replace", &[py_str("light-headed"), py_str("lightheaded")]), "replace", &[py_str("light headed"), py_str("lightheaded")]);
    flat = mcall(&mcall(&flat.clone(), "replace", &[py_str("bunged-up"), py_str("bungedup")]), "replace", &[py_str("bunged up"), py_str("bungedup")]);
    flat = mcall(&mcall(&flat.clone(), "replace", &[py_str("blocked-up"), py_str("blockedup")]), "replace", &[py_str("blocked up"), py_str("blockedup")]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&flat.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_opens_a_question(&[words.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("im"), py_str("i'm"), py_str("am"), py_str("iam")]), &geti(&words.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&words.clone(), &index.clone()), &py_str("i"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&words.clone(), &add(&index.clone(), &Py::Int(1))), &py_str("am"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FEEL_COPULA(), &geti(&words.clone(), &add(&index.clone(), &Py::Int(1))))) } } } })) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None)) { let mut w: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&crate::gen::compose::fallback::g_BODY_LEAD(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    start = if py_truthy(&Py::Bool(py_eq(&geti(&words.clone(), &index.clone()), &py_str("i")))) { add(&index.clone(), &Py::Int(2)) } else { add(&index.clone(), &Py::Int(1)) };
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_GOING_BOTH_WAYS(), "search", &[clean.clone()])) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&_state_from(&[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &start.clone(), &Py::None, &Py::None)])]), 2, None);
    state = __unpk2[0].clone();
    after = __unpk2[1].clone();
    if py_truthy(&state.clone()) {
    return state.clone();
    }
    }
    part_state = _a_part_in_a_state(&[clean.clone()]);
    if py_truthy(&part_state.clone()) {
    return part_state.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_part_in_a_state(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut state: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Which body state the line says of a part of them, or None: \"my hands are clammy\".\n\n    A nose or an ear has no answer of its own, so a cold one is the person being cold, and the\n    same goes for a clammy hand or a pale face: the pools answer a state of the body, not only a\n    state said with \"im\", so \"my hands are clammy\" gets the line \"im clammy\" gets. The state word\n    can come after a qualifier (\"my nose is really cold\", \"my hands are a bit clammy\") and can be\n    said twice (\"my nose is freezing cold\"), which is how people say it; anything else in the line\n    before the state word (\"my ears are red and cold\") says something else is going on, and anything\n    after it (\"my face is pale and i feel sick\") makes it more than a state, so both keep\n    whatever reading they had.\n    ");
    part = mcall(&crate::gen::compose::fallback::g_BODY_PART(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&part.clone(), "group", &[py_str("rest")]), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    state = _the_state(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&state.clone()))) {
    if py_truthy(&Py::Bool(!py_contains(&crate::gen::compose::fallback::g_BODY_PART_QUALIFIER(), &word.clone()))) {
    return Py::None;
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&state.clone(), &py_str("hot")))) {
    return Py::None;
    }
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&w.clone(), &py_str("and")))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_BODY_PART_TAIL(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_the_state(&[w.clone()]), &state.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return state.clone();
    }
    return Py::None;
    }
    return Py::None;
    Py::None
}
pub fn _a_part_is_cold(args: &[Py]) -> Py {
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
    let _ = py_str("The old name for the same reading, kept because the export list in fallback.py names it.\n\n    It answered only a part being cold (\"my nose is freezing\"); it reads any state a pool answers\n    now, so the name says less than the function does.\n    ");
    return Py::Bool(py_eq(&_a_part_in_a_state(&[clean.clone()]), &py_str("cold")));
    Py::None
}
pub fn _state_from(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut state: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(state, words after it) for the state the rest of the line names, or (None, []) .");
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&rest.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    state = _the_state(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&state.clone()))) {
    continue;
    }
    tail = slice_or_raise(&words.clone(), &add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&tail.clone(), &Py::Int(0)), &py_str("and"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::compose::fallback::g_BODY_ALSO(), &geti(&tail.clone(), &Py::Int(1)))) } } } } } }) {
    return py_tuple(vec![state.clone(), py_list(vec![])]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::compose::fallback::g_BODY_TAIL(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_tuple(vec![state.clone(), tail.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_BODY_SITUATION(), &mcall(&py_str(" "), "join", &[tail.clone()])))) {
    return py_tuple(vec![state.clone(), tail.clone()]);
    }
    }
    return py_tuple(vec![Py::None, py_list(vec![])]);
    Py::None
}
pub fn _the_state(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The body-state name for this one word, or None.");
    let __iter1 = py_iter_or_raise(&mcall(&crate::gen::compose::fallback::g_BODY_STATES(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    pool = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&pool.clone(), &word.clone()))) {
    return name.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _habit_is_a_problem(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the repeated doing is one a person would want a hand with.");
    return call_builtin("bool", &[{ let __b = mcall(&crate::gen::compose::fallback::g_HABIT_PROBLEM_ACT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HABIT_MIND_IT(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_flat_habit(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the composer's habit pool claims this line.\n\n    Read by `chat` before it turns the same line into its own stock question (\"Annoying. Is it\n    the same every time, or does it vary?\"): a habit said flatly is answered here, and the\n    module above stands aside so the two readings cannot disagree about who answers. So this is\n    the branch's own gate and nothing else - a recurrence and a doing, no question, and no mood\n    word. A claim the reply does not keep is worse than no claim at all: `chat` would stand down\n    and the echo (\"Noted: you keep interrupting your sister\") would answer instead.\n    ");
    said = py_list(vec![call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    repaired = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plain")]) } else { Py::None };
    if py_truthy(&{ let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &geti(&said.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&said.clone(), "append", &[repaired.clone()]);
    }
    let __iter1 = py_iter_or_raise(&said.clone());
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&_flat_habit_stated(&[line.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _flat_habit_stated(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut raw: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "raw") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: raw"))) };
    let mut clean: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The gate itself, for one spelling of the line.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&raw.clone(), "lower", &[]), "split", &[])]);
    words = mcall(&clean.clone(), "split", &[]);
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = mcall(&g_HABIT_STATED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_HABIT_NOT(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&_mood(&[words.clone(), clean.clone()]), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_reads_as_question(&[raw.clone(), words.clone()]))); if !py_truthy(&__b) { __b } else { _habit_is_a_problem(&[clean.clone()]) } } } } } } } }]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _a_claim_about_a_thing(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("True when the bad word belongs to a thing named with \"the\" (\"the weather is ...\").\n\n    A stretch of time is the exception: \"the week was rubbish\" and \"the day was awful\" are how\n    people say it went for them, so the noun after \"the\" has to be something other than time.\n    ");
    found = mcall(&g_NOT_ABOUT_THEM(), "search", &[add(&before.clone(), &py_str(" "))]);
    return { let __b = call_builtin("bool", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::compose::fallback::g_STRETCH_OF_TIME(), &mcall(&found.clone(), "group", &[Py::Int(1)]))) } };
    Py::None
}
pub fn _an_order_it_cannot_do(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let _ = py_str("The honest line for a job in the world typed as an order, or None.\n\n    Only the shapes where the doing can only be the person's are read: a message to somebody, a\n    purchase or a booking, something put in their calendar or notes, a timer, and a document. The\n    offer after the limit is what makes it an answer - the words, the choosing, the order to do\n    things in - so a line that is a thing to work out keeps the advice it gets elsewhere.\n    ");
    bare = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9 ']+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ORDER_OPENER(), "match", &[bare.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_ORDER_A_REPORT(), "search", &[bare.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ORDER_NOT_A_JOB(), "search", &[bare.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ORDER_A_CLOCK(), "search", &[bare.clone()])) {
    return g_ACTS_ON_A_CLOCK_LINES();
    }
    if py_truthy(&mcall(&g_ORDER_A_RECORD(), "search", &[bare.clone()])) {
    return g_ACTS_IN_A_DIARY_LINES();
    }
    if py_truthy(&mcall(&g_ORDER_A_MESSAGE(), "search", &[bare.clone()])) {
    return g_ACTS_ON_A_PHONE_LINES();
    }
    if py_truthy(&mcall(&g_ORDER_A_BUY(), "search", &[bare.clone()])) {
    return g_ACTS_AN_ERRAND_LINES();
    }
    if py_truthy(&mcall(&g_ORDER_A_DOCUMENT(), "search", &[bare.clone()])) {
    return g_ACTS_ON_A_DOCUMENT_LINES();
    }
    return Py::None;
    Py::None
}
pub fn _act_asked_of_me(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The honest line for a job asked of the bot in the polite form, or None.\n\n    Only the polite form is read: the imperative has `_acts_for_me` and `ACTS_FOR_ME` already,\n    and its house tasks (\"put the kettle on\") are a different offer from these.\n    ");
    match_ = mcall(&g_POLITE_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    rest = mcall(&mcall(&match_.clone(), "group", &[py_str("rest")]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { mcall(&g_CAN_DO_ITSELF(), "match", &[rest.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ACTS_IN_A_DIARY(), "search", &[rest.clone()])) {
    return g_ACTS_IN_A_DIARY_LINES();
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_EMERGENCY_CALL(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_POLICE_CALL(), "search", &[rest.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_THING_TO_GET(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HANDS_IT_OVER_HERE(), "search", &[rest.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ACTS_AN_ERRAND(), "search", &[rest.clone()])) {
    return g_ACTS_AN_ERRAND_LINES();
    }
    if py_truthy(&mcall(&g_ACTS_ON_A_PHONE(), "search", &[rest.clone()])) {
    return g_ACTS_ON_A_PHONE_LINES();
    }
    return Py::None;
    Py::None
}
pub fn _the_store_has_the_recall(args: &[Py]) -> Py {
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
    let _ = py_str("True when the memory store has the line a \"remind me ...\" asks for.\n\n    The store is asked rather than a list of words here, because only the store knows what it can\n    hand back: it answers \"remind me what we said about the boiler\" and \"remind me where we\n    started\", and returns nothing for \"remind me who my sister is\". A word list in this file would\n    be a second, drifting copy of that knowledge - the first cut of this had one, and every shape\n    it waved through but the store could not answer fell to a nod (\"Got it. Is there anything\n    you'd like a hand with?\") where the limit had been the honest answer.\n    ");
    let __try1 = exc::catch(|| {
    let mut memory_store: Py = crate::gen::modules::memory_store::module_obj();
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
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&memory_store.clone(), "respond", &[clean.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _acts_for_me(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("True when the line tells the bot to do a job in the world, which it cannot do.");
    bare = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9 ']+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&mcall(&g_KEEP_ME_POSTED(), "search", &[bare.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_MESSAGE_OR_A_FORM(), "search", &[bare.clone()])) {
    if py_truthy(&{ let __b = crate::gen::compose::no_eyes::writes_the_message(&[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::no_eyes::cannot_see_your_thing(&[clean.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::no_eyes::look_at_a_body_lines(&[clean.clone(), bare.clone()]) } } } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&mcall(&g_REMIND_ME(), "match", &[bare.clone()])) {
    return Py::Bool(!py_truthy(&_the_store_has_the_recall(&[clean.clone(), memory.clone()])));
    }
    opener = mcall(&g_ACT_OPENER(), "match", &[bare.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&opener.clone()))) {
    return Py::Bool(false);
    }
    tail = slice_or_raise(&bare.clone(), &mcall(&opener.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&mcall(&g_ACT_REPORT_TAIL(), "search", &[tail.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ACT_FOR_ME(), "search", &[bare.clone()])))) {
    if py_truthy(&{ let __b = mcall(&g_ACT_CHOICE(), "search", &[bare.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ACT_OWN_STEP(), "match", &[tail.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ACT_SETTING(), "search", &[bare.clone()]) } } } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&mcall(&g_ACT_FOR_ME(), "search", &[bare.clone()])) {
    return Py::Bool(true);
    }
    verb = geti(&mcall(&bare.clone(), "split", &[]), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::gen::compose::fallback::g_HOUSE_VERB(), &verb.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ADDRESSED(), "match", &[bare.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("call"), py_str("ring"), py_str("phone"), py_str("text"), py_str("email"), py_str("buy"), py_str("pay"), py_str("renew")]), &verb.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("book"), py_str("reserve"), py_str("order")]), &verb.clone()))) {
    return call_builtin("bool", &[mcall(&g_BOOKING_THING(), "search", &[bare.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_ADDRESSED(), "match", &[bare.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ACT_THING(), "search", &[bare.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _reads_as_a_bad_stretch(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is a bad stretch of time and nothing else.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&clean.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&crate::gen::compose::fallback::g_BAD_STRETCH(), &word.clone()))) {
    continue;
    }
    before = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&index.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)]);
    if py_truthy(&_a_claim_about_a_thing(&[before.clone()])) {
    continue;
    }
    after = slice_or_raise(&words.clone(), &add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_list(vec![py_str("few")]), py_list(vec![py_str("couple")])]), &slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None))); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    after = slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::compose::fallback::g_STRETCH_OF_TIME(), &geti(&after.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&before.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::compose::fallback::g_STRETCH_OF_TIME(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _cue_negated(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a negator stands just in front of the cue: \"not finished\", \"not yet finished\".\n\n    Two words is the widest that stays honest. One misses \"not yet sorted\"; three starts\n    swallowing real wins - \"i cant believe i finished it\" and \"not bad, i finished it\" are both\n    good news with a negator three words away. Three is only enough when a doubt word sits\n    between them: \"i don't think i'm happy\" is not happy, and `turns_round` tells that apart\n    from the \"can't believe\" that is delight.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &cue.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&index.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::compose::fallback::g_NEGATORS(), &other.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    before = slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&index.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None);
    if py_truthy(&bitand(&call_builtin("set", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_DOUBTED())) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = bitand(&call_builtin("set", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_DOUBT()); if !py_truthy(&__b) { __b } else { py_call(&crate::gen::compose::fallback::g_turns_round(), &[before.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_state_in_the_past(args: &[Py]) -> Py {
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
    let _ = py_str("True when the state word at `index` is said in the past: \"my wife was pregnant\".");
    return call_builtin("bool", &[bitand(&call_builtin("set", &[slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&index.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g__WAS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_win_is_someone_elses(args: &[Py]) -> Py {
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
    let mut j: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the good thing at `index` is a crowd's, not the person's own.\n\n    Reads back to the nearest subject and stops at the first one: \"my friend told me i should be\n    proud\" is still the person's own pride, because the \"i\" is nearer the cue than the friend.\n    What counts as someone else's is a crowd - the thing a person can be measured against, \"everyone\n    else\", \"all my mates\". A relation on their own is not: \"my mum finally finished chemo\" is news\n    a person is glad of, and withholding that cheer is not what this is for.\n\n    A crowd feeling something about the person is still the person's: \"everyone is happy for us\",\n    \"everyone's proud of me\" - the words after the cue say who it is about, so they are read too.\n    ");
    after = slice_or_raise(&words.clone(), &add(&index.clone(), &Py::Int(1)), &add(&index.clone(), &Py::Int(3)), &Py::None);
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_ABOUT_WORDS(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_PERSON_WORDS()) } } } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&index.clone(), &Py::Int(1)), call_builtin("max", &[neg(&Py::Int(1)), sub(&index.clone(), &Py::Int(7))]).unwrap_or_else(|e| exc::raise(e)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    j = __item.clone();
    word = geti(&words.clone(), &j.clone());
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_YOU_SUBJECTS(), &word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_CROWD(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::loss::g_PEOPLE(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&j.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &add(&j.clone(), &Py::Int(1)), &Py::None), &py_str("all"))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_no_pleasure(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line is a person saying the pleasure has gone out of everything.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_COMPARISON(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::limit::g_NOTHING_THAT_LIMITS(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLEASURE_ABOUT_ONE_THING(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_FEELING_NOTHING(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLEASURE_GONE_OUT(), "search", &[text.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_NOTHING_AND_A_PLEASURE(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_NOTHING_NEGATED(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_question_about_the_bot(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True for \"can you drive\", \"can you feel pain\": the whole line, about this program.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_ABOUT_THE_BOT(), "match", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _a_thing_stopping_after(args: &[Py]) -> Py {
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
    let mut i: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "i") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: i"))) };
    let mut after: Py = Py::None;
    let _ = py_str("True when the words after a cue name that thing stopping (\"has finally died\").");
    after = call_builtin("list", &[slice_or_raise(&words.clone(), &add(&i.clone(), &Py::Int(1)), &add(&i.clone(), &Py::Int(5)), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THE_WORDS_BETWEEN(), &geti(&after.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&after.clone(), "pop", &[Py::Int(0)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_THINGS_THAT_STOP(), &geti(&after.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THINGS_THAT_STOP_AFTER_UP(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("up"), py_str("in"), py_str("out")]), &geti(&after.clone(), &Py::Int(1)))) } } } }) {
    return { let __b = Py::Bool(py_eq(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("on"), py_str("it"), py_str("them"), py_str("me"), py_str("him"), py_str("her"), py_str("us")]), &geti(&after.clone(), &Py::Int(2)))) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_harm_said(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line reports a harm done to the person, or an illness they have.\n\n    Read by `_mood`, so the line is a bad line and the warm pool answers it rather than the day\n    pool asking how it went. A line that already says it is over (\"i broke my wrist years ago\")\n    is still a harm they told the bot about, and the warm line fits it better than a cheer.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_HARM_IDIOM(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_PART_OUT(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__A_PROGNOSIS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_PROGNOSIS_THAT_IS_NOT(), "search", &[said.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__A_HURT_DONE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_BODY_WORD(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__A_HURT_BY_SOMEONE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_HURT_BY_SOMEONE_NOT(), "search", &[said.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__A_HARM_ILL(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_HARM_ILL_NOT(), "search", &[said.clone()]))) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__A_HARM_HAD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_PART_WENT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_HARM_KNOCK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_OUT_OF_SOCKET(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_IN_AGONY(), "search", &[said.clone()]) } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _mood(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut after: Py = Py::None;
    let mut back: Py = Py::None;
    let mut bad: Py = Py::None;
    let mut before: Py = Py::None;
    let mut good: Py = Py::None;
    let mut i: Py = Py::None;
    let mut window: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("'positive', 'negative' or 'neutral', from the words of a statement.\n\n    A cue only says which way the line goes when nothing in front of it turns it round: \"not\n    happy\" is a bad line, not a good one, and a cue that is turned round the other way (\"not\n    terrible\") is left out rather than counted either way. Without this the cheerful pool\n    congratulates someone who has just said they feel awful, which is the reply people\n    remember. A cue inside an aim is not a mood at all (\"i want to lose weight\"), and neither is\n    one behind \"nearly\"/\"almost\" (\"i nearly finished it\"), which is not done and not condoled.\n    ");
    words = call_builtin("tuple", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&crate::gen::memory::ack::only_a_dislike(&[clean.clone()])) {
    return py_str("neutral");
    }
    if py_truthy(&crate::gen::compose::limit::a_limit_on_the_answer(&[clean.clone()])) {
    return py_str("neutral");
    }
    if py_truthy(&reads_as_no_pleasure(&[clean.clone()])) {
    return py_str("negative");
    }
    good = Py::Bool(false);
    bad = Py::Bool(false);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("ill"))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_reads_as_illness(&[clean.clone()]) } }) {
    bad = Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&good.clone())); if !py_truthy(&__b) { __b } else { { let __b = _a_harm_said(&[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&crate::gen::compose::fallback::g_reads_like_a_symptom(), &[clean.clone()]))) } } } }) {
    bad = Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&crate::gen::compose::fallback::_in_an_aim(&[clean.clone(), word.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_JUNK_CUES(), &word.clone())); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_mood_word(&[words.clone(), word.clone()]) } }) {
    bad = Py::Bool(true);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__BREAK_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { mcall(&g__A_BREAK_THAT_IS_GOOD(), "search", &[clean.clone()]) } }) {
    good = Py::Bool(true);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::gen::compose::fallback::g_POSITIVE_CUES(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::compose::fallback::g_NEGATIVE_CUES(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_A_MOOD_WITH_A_CATCH(), &word.clone()))) {
    before = call_builtin("set", &[slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&i.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &i.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    after = if py_truthy(&lt(&add(&i.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &add(&i.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_eq(&after.clone(), &py_str("up")))) {
    good = Py::Bool(true);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&before.clone(), &geti(&crate::gen::compose::fallback::g_A_MOOD_WITH_A_CATCH(), &word.clone())))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("me"), py_str("him"), py_str("her"), py_str("them"), py_str("us")]), &after.clone())) } }) {
    continue;
    }
    }
    window = slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&i.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &i.clone(), &Py::None);
    if py_truthy(&bitand(&call_builtin("set", &[window.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_NEARLY())) {
    continue;
    }
    back = _cue_negated(&[words.clone(), word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_PAST_STATE_CUES(), &word.clone())); if !py_truthy(&__b) { __b } else { _a_state_in_the_past(&[words.clone(), i.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_POSITIVE_CUES(), &word.clone())); if !py_truthy(&__b) { __b } else { _a_thing_stopping_after(&[words.clone(), i.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_POSITIVE_CUES(), &word.clone()))) {
    if py_truthy(&_no_such_thing_before(&[words.clone(), i.clone()])) {
    back = Py::Bool(true);
    }
    if py_truthy(&back.clone()) {
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_STATE_CUES(), &word.clone()))) {
    bad = Py::Bool(true);
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback_data::g_A_WAIT_ENDING(), &word.clone())); if !py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback_data::g_SOMETHING_CAME_APART(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    continue;
    } else {
    if py_truthy(&_the_win_is_someone_elses(&[words.clone(), i.clone()])) {
    continue;
    } else {
    good = Py::Bool(true);
    }
    }
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&back.clone())); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[window.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_AIM_HEADS()) } }) {
    bad = Py::Bool(true);
    }
    }
    }
    if py_truthy(&bad.clone()) {
    return py_str("negative");
    }
    if py_truthy(&good.clone()) {
    return py_str("positive");
    }
    return py_str("neutral");
    Py::None
}
pub fn _no_such_thing_before(args: &[Py]) -> Py {
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
    let mut i: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "i") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: i"))) };
    let mut step: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the cue is the thing a word meaning none of it is said about.\n\n    \"nobody makes me feel good\" puts three words between the quantity and the cue, so the\n    three-word window the negators are read in is not wide enough; a contrast word starts a new\n    claim and ends the search (\"nobody believed i could do it but i did\" is good news).\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), Py::Int(9)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    if py_truthy(&lt(&sub(&i.clone(), &step.clone()), &Py::Int(0))) {
    return Py::Bool(false);
    }
    word = geti(&words.clone(), &sub(&i.clone(), &step.clone()));
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback_data::g_CONTRAST(), &word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback_data::g_NOTHING_AT_ALL(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&word.clone(), &py_str("one"))); if !py_truthy(&__b) { __b } else { { let __b = ge(&sub(&sub(&i.clone(), &step.clone()), &Py::Int(1)), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &sub(&sub(&i.clone(), &step.clone()), &Py::Int(1))), &py_str("no"))) } } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _mixed_seed(args: &[Py]) -> Py {
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
    let mut value: Py = Py::None;
    let _ = py_str("A number that changes when the line changes.\n\n    `_choose` adds up letter values, which two lines of the same length can share: the four\n    outcome pools are all about the same length, so a sum gave several threads the same sentence.\n    ");
    value = Py::Int(7);
    let __iter1 = py_iter_or_raise(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    char = __item.clone();
    value = modulo(&add(&mul(&value.clone(), &Py::Int(31)), &call_builtin("ord", &[char.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1000003));
    }
    return value.clone();
    Py::None
}
pub fn _only_what_the_line_gives(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut asserts: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The pool minus the lines that tell the person how they feel, unless they said it.");
    if py_truthy(&mcall(&g_A_FEELING_IN_THE_LINE(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return pool.clone();
    }
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    line = __item.clone();
    asserts = Py::Bool(false);
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), line.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    sentence = __item.clone();
    if py_truthy(&mcall(&mcall(&sentence.clone(), "rstrip", &[]), "endswith", &[py_str("?")])) {
    continue;
    }
    if py_truthy(&mcall(&g_SAYS_A_FEELING(), "search", &[sentence.clone()])) {
    asserts = Py::Bool(true);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&asserts.clone()))) {
    let _ = mcall(&kept.clone(), "append", &[line.clone()]);
    }
    }
    return { let __b = call_builtin("tuple", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { pool.clone() } };
    Py::None
}
pub fn _choose(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut seed_text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seed_text") { v.clone() }
    else { Py::None };
    let mut c: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut last: Py = Py::None;
    let mut o: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One option, varied by the line itself and never the reply just given.\n\n    A pool as short as NEUTRAL's three lines cannot keep the five-reply promise: once all three\n    are in the window nothing is fresh, and restoring the whole pool brings back the line just\n    used - \"stacked up with emails\" straight after \"back to school\", two of the same kind of\n    reply in a row, which reads like the bot stopped listening. So the wide window holds while\n    it can, and what is never repeated is the reply just given.\n    ");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    recent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(5)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&mcall(&t.clone(), "get", &[py_str("bot")])) {  __out.push(mcall(&t.clone(), "get", &[py_str("bot")])); } } py_list(__out) })() };
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&recent.clone(), &o.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&fresh.clone()))) {
    last = if py_truthy(&recent.clone()) { geti(&recent.clone(), &neg(&Py::Int(1))) } else { Py::None };
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&o.clone(), &last.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&fresh.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&seed_text.clone(), &Py::None))) {
    seed = add(&mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clean.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    } else {
    seed = add(&mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &_mixed_seed(&[seed_text.clone()]));
    }
    return mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[fresh.clone()]);
    Py::None
}
pub fn _a_birthday_still_to_come(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut met: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The one birthday this line gave a figure for, from the people the store now holds.\n\n    The store keeps a birthday still to come beside the age the person is (\"my mum is 72 and she\n    is 73 next month\" is 72 now, 73 next month), so the figure and the day are both there to say\n    back. Only the entry whose number and day are in this line counts, so an age filed turns ago\n    cannot take over a line about somebody else, and only one entry: two birthdays in one line is\n    not one thing to name.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    met = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = mcall(&one.clone(), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&one.clone(), "get", &[py_str("who")]))) } } } }) {
    continue;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&one.clone(), "get", &[py_str("to_come")]), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]), 2, None);
    age = __unpk2[0].clone();
    when = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&when.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &when.clone())) } }) {
    continue;
    }
    let _ = mcall(&met.clone(), "append", &[one.clone()]);
    }
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[met.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&met.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _the_birthday_said_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut birthday: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "birthday") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: birthday"))) };
    let mut age: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut when: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The pool above with the person, the figure and the day filled in.");
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&birthday.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&who.clone(), "startswith", &[py_tuple(vec![py_str("your "), py_str("my ")])]))) } }) {
    who = add(&py_str("your "), &who.clone());
    }
    age = call_builtin("str", &[mcall(&birthday.clone(), "get", &[py_str("to_come")])]).unwrap_or_else(|e| exc::raise(e));
    when = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&birthday.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&crate::gen::compose::fallback::g_A_BIRTHDAY_STILL_TO_COME());
    for __item in __iter1 {
    line = __item.clone();
    line = mcall(&mcall(&mcall(&line.clone(), "replace", &[py_str("{who}"), who.clone()]), "replace", &[py_str("{age}"), age.clone()]), "replace", &[py_str("{when}"), when.clone()]);
    let _ = mcall(&said.clone(), "append", &[add(&mcall(&slice_or_raise(&line.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&line.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    return call_builtin("tuple", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _somebody_elses_day_in_thread(args: &[Py]) -> Py {
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
    let mut said_words: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the thread, not the line, belongs to somebody else's day.\n\n    `the baby is 8 months and still not sleeping through` / `weve tried a bedtime routine`: the\n    second line names nobody, so the words on the line cannot see the baby, and the method for\n    an adult's day (\"give it a fixed slot - what are you not getting to?\") answers nothing in\n    that thread. The line before it is the whole of the test, and it is read for the same words\n    the line itself is read for - no module has to have answered it, because a chat line naming\n    a puppy is as much a puppy thread as a knowledge line naming one.\n\n    A death is the exception, and it is the reason this is not just a word check: `my dog died\n    last night` then `help me get organised` is a person arranging their own day while grieving,\n    and a dog that has died has no routine to protect.\n    ");
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &neg(&Py::Int(1)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g__NO_DAY_OF_THEIR_OWN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { crate::gen::compose::loss::reads_as_loss(&[said.clone()]) } }) {
    return Py::Bool(false);
    }
    said_words = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:'\"()")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bitand(&said_words.clone(), &crate::gen::compose::fallback::g_ROUTINE_NOT()); if py_truthy(&__b) { __b } else { mcall(&g__ROUTINE_NOT_SHAPE(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_person_named_a_list(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the person's own last line or two put a list of things on the table.\n\n    Only their lines: the bot's last reply is not evidence of a list, because the cards that are\n    not about lists at all (\"ring a crisis line now\", \"here is one about a parrot\") would pass\n    for one and the size-of-the-list pool would answer them.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &neg(&Py::Int(2)), &Py::None, &Py::None);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(mcall(&g_A_LIST_ON_THE_TABLE(), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _routine_pool(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The pool for a line about the shape of the speaker's day, or None if it is not one.");
    words = call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_is(&memory.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::memory::stuck::the_advice_was_to_get_seen(&[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_TOO_MUCH_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { bitand(&words.clone(), &py_set(vec![py_str("overwhelmed"), py_str("drowning"), py_str("swamped")])) } } } } } }) {
    return crate::gen::memory::stuck::g_URGENT_ALREADY_GIVEN();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_TOO_MUCH_PHRASES()) { let mut phrase: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&crate::gen::compose::fallback::g__START_ASK(), &phrase.clone()))) {  __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return crate::gen::compose::fallback::g_TOO_MUCH();
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_money_decision(&[clean.clone()]); if !py_truthy(&__b) { __b } else { bitand(&words.clone(), &crate::gen::compose::fallback::g_BEHIND_ON_MONEY()) } }) {
    return crate::gen::compose::fallback::g_UNKNOWN_MONEY();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__UNDERTAKING(), "findall", &[clean.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&crate::gen::compose::fallback::g__DAY_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &crate::gen::compose::fallback::g_ROUTINE_NOT()); if py_truthy(&__b) { __b } else { mcall(&g__ROUTINE_NOT_SHAPE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&memory.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _somebody_elses_day_in_thread(&[memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_ROUTINE_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return crate::gen::compose::fallback::g_ROUTINE();
    }
    if py_truthy(&bitand(&words.clone(), &crate::gen::compose::fallback::g_ROUTINE_WORDS())) {
    return if py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("overwhelmed"), py_str("drowning"), py_str("swamped")]))) { crate::gen::compose::fallback::g_TOO_MUCH() } else { crate::gen::compose::fallback::g_ROUTINE() };
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g__START_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return if py_truthy(&_the_person_named_a_list(&[memory.clone()])) { crate::gen::compose::fallback::g_TOO_MUCH() } else { Py::None };
    }
    return Py::None;
    Py::None
}
pub fn _asks_the_bot(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the bot for something: an imperative, or an ask for options.");
    if py_truthy(&bitand(&call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("recommend"), py_str("steps"), py_str("options")]))) {
    return Py::Bool(true);
    }
    norm = { let __b = mcall(&understood.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    return { let __b = call_builtin("bool", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::compose::fallback::g_IMPERATIVE(), &geti(&norm.clone(), &Py::Int(0)))) } };
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
    let _ = py_str("The words of a line the way `_mood` wants them: lowercase, no punctuation.");
    return call_builtin("tuple", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _reads(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pred: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pred") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pred"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut lower: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lower") { v.clone() }
    else { Py::Bool(false) };
    let mut memory: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut ask: Py = Py::None;
    let _ = py_str("True when a cue reads the typed line or the same line with its words put right.\n\n    \"me n my gf r arguin a lot\" is the same line as \"my girlfriend and i keep arguing a lot\",\n    and the reading has already repaired it (bot/understanding's `plain`): the cue \"arguing\" is\n    in what the person said even though they typed \"arguin\". Every pool in this file tests the\n    typed line first, so a line that reads with no repair is untouched; the repaired line is\n    only ever a second try, never a replacement.\n    ");
pub fn __nested_ask_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut ask: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut clean: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut lower: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut pred: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut repaired: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "text").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return if py_truthy(&Py::Bool(!py_is(&memory.clone(), &Py::None))) { py_call(&pred, &[if py_truthy(&lower.clone()) { mcall(&text.clone(), "lower", &[]) } else { text.clone() }, memory.clone()]) } else { py_call(&pred, &[if py_truthy(&lower.clone()) { mcall(&text.clone(), "lower", &[]) } else { text.clone() }]) };
    Py::None
}
    ask = py_func("ask", move |args: &[Py]| { let mut full = vec![ask.clone(), clean.clone(), lower.clone(), memory.clone(), pred.clone(), repaired.clone()]; full.extend_from_slice(args); __nested_ask_0(&full) });
    if py_truthy(&py_call(&ask, &[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    return call_builtin("bool", &[py_call(&ask, &[repaired.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _restated(args: &[Py]) -> Py {
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
    let mut focus: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The ask a correction points at (\"no i meant <this>\" -> \"<this>\"), or the empty string.");
    if py_truthy(&Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("intent")]), &py_str("correction")))) {
    return py_str("");
    }
    focus = mcall(&mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("focus")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]), "strip", &[]);
    focus = mcall(&mcall(&g_FRAME_PREFIX(), "sub", &[py_str(""), focus.clone()]), "strip", &[py_str(" ,.!?")]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&focus.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&focus.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::compose::fallback::g_FRAME_WORDS(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASK_SHAPE(), "search", &[mcall(&focus.clone(), "lower", &[])])))) {
    return py_str("");
    }
    return focus.clone();
    Py::None
}
pub fn _answers_the_correction(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut restated: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "restated") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: restated"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut pool: Py = Py::None;
    let mut punt: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut sub: Py = Py::None;
    let _ = py_str("The pointed-at ask answered from the thread, or None when it has only a punt in it.");
    let mut verdict: Py = crate::gen::compose::verdict::module_obj();
    sub = call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&sub.clone(), &py_str("compose"), call_builtin("dict", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("compose")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setitem(&sub.clone(), &py_str("understood"), py_call(&crate::gen::compose::fallback::g_understand(), &[restated.clone(), memory.clone()]));
    let _ = py_setitem(&sub.clone(), &py_str("correction_answered"), Py::Bool(true));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![gattr(&verdict.clone(), "next_step"), gattr(&verdict.clone(), "reply_for"), py_func("reply_for", |args: &[Py]| reply_for(args))]));
    for __item in __iter1 {
    pool = __item.clone();
    reply = py_call(&pool, &[restated.clone(), sub.clone()]);
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_PUNT()) { let mut punt: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&reply.clone(), "lower", &[]), &punt.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return reply.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_near_miss(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is a result they almost had (\"i almost got the job\", \"i nearly won\").");
    return call_builtin("bool", &[mcall(&g_NEAR_MISS(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_things_state(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut text: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Whether the state the line names belongs to a thing of the person's, not to the person.\n\n    \"my lawn looks sad\" is a report about a lawn; \"im sad\" is a person's own state, \"my mum is sad\"\n    is somebody else's, and \"the weather is grim\" is the day rather than a thing that can be seen\n    to. The shared reading in memory already says whose the state is (state_owner), so the composer\n    asks it rather than re-reading the line.\n    ");
    understood = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    text = mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("state_owner")]), &py_str("thing"))); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_OWN_THING(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_MOOD(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _a_chat_to_correct(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the bot has said anything this chat for a line to be putting right.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _takes_something_back(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line opens by correcting something and is not then asking a question.\n\n    \"no it was my idea not the kids\" is the person putting the bot right about who had the idea.\n    Three things keep it to that. The chat has to be under way, because a correction is a reply to\n    something said here and \"no onions in it\" as the opening line is somebody telling the bot what\n    they are cooking. The line must not go on to ask anything (\"no what about the other one\"). And\n    the rest of it has to say a thing rather than name one: a verb, or the thing behind its own\n    article. A correction with nothing on the end of it (\"no\", \"no that's not it\") is the\n    withdrawal pool's, and \"no heating since friday\" is the sentence.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TAKING_SOMETHING_BACK(), "match", &[text.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_chat_to_correct(&[memory.clone()])))) {
    return Py::Bool(false);
    }
    body = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_OPENER(), "sub", args))(&__args) }, "strip", &[py_str(" ,.;:-")]);
    if py_truthy(&{ let __b = body.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_TAKING_SOMETHING_BACK(), "match", &[body.clone()]) } }) {
    body = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(body.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TAKING_SOMETHING_BACK(), "sub", args))(&__args) }); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_OPENER(), "sub", args))(&__args) };
    }
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mcall(&body.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { mcall(&g_ASKS_AFTER_THE_OPENER(), "match", &[body.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_SAYS_A_THING(), "search", &[body.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_THING_ITSELF(), "match", &[body.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_lateness_report(args: &[Py]) -> Py {
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
    let mut normalised: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "normalised") { v.clone() }
    else { py_str("") };
    let mut after: Py = Py::None;
    let mut between: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut read: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The speaker's own lateness, stated and not asked about - the one line the pool is for.\n\n    Read off the typed line first, then off the reading's `norm_text` when that found nothing:\n    chat-speak is written the way people type it, and the reader has already put it back into\n    words (`gona` -> \"going to\", `bot/understanding/normalize.py`). \"im gona be late for work\"\n    is the same report as \"im gonna be late for work\", and the pool that read only the typed\n    words answered a person rushing out of the door with the neutral nod.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_LATE_ASKS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_LATE_NOT_A_TIME(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_eq(&_mood(&[py_call(&crate::gen::compose::fallback::g__words(), &[clean.clone()]), clean.clone()]), &py_str("neutral")))) {
    return Py::Bool(false);
    }
    read = clean.clone();
    hit = mcall(&crate::gen::compose::fallback::g_LATE_NOW(), "search", &[read.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&hit.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { normalised.clone() } }) {
    read = mcall(&py_str(" "), "join", &[mcall(&mcall(&normalised.clone(), "replace", &[py_str(" | "), py_str(" ")]), "split", &[])]);
    hit = mcall(&crate::gen::compose::fallback::g_LATE_NOW(), "search", &[read.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&hit.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    between = { let __b = mcall(&hit.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_LATE_NOT_IT(), "search", &[between.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&{ let __b = mcall(&hit.clone(), "group", &[Py::Int(0)]); if py_truthy(&__b) { __b } else { py_str("") } }, "endswith", &[py_str("behind")])) {
    after = slice_or_raise(&read.clone(), &mcall(&hit.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), after.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::compose::loss::g_SELF_NOUNS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _somebody_hurt(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The reply to a line that says somebody the person loves has been hurt, or None.");
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("asks")])) {
    return Py::None;
    }
    if py_truthy(&crate::gen::compose::fallback::_in_a_care_thread(&[memory.clone()])) {
    return Py::None;
    }
    found = mcall(&g_HURT_SOMEBODY(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HURT_DEATH(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_HURT_EVENT(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HURT_ON_THE_SPEAKER(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HURT_BUT_FINE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HURT_FIGURE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    who = stdlib_call("re", "sub", &[py_str("^(?:my|our)\\b"), py_str("your"), mcall(&found.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e));
    return _choose(&[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_SOMEBODY_HURT()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _a_low_day(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("True when the line says one of the person's own days went badly, and nothing else.");
    if py_truthy(&Py::Bool(py_eq(&mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("kind")]), &py_str("question")))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_LOW_DAY_NOT_MINE(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str(","))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str(" because "))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str(" cos "))) } } } }) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_LOW_DAY(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_LOW_DAY_MINE(), "match", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _loves_people(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("How many of their own people the whole line says they love: 0, 1 or 2 (two or more).");
    if py_truthy(&Py::Bool(py_contains(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &py_str("?")))) {
    return Py::Int(0);
    }
    text = mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&add(&add(&py_str(" "), &text.clone()), &py_str(" ")), &py_str(" but "))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&add(&add(&py_str(" "), &text.clone()), &py_str(" ")), &py_str(" though "))) } }) {
    return Py::Int(0);
    }
    if py_truthy(&{ let __b = mcall(&g_LOVES_A_GROUP(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_LOVES_PEOPLE(), "match", &[text.clone()]) } }) {
    return Py::Int(2);
    }
    if py_truthy(&mcall(&g_LOVES_SOMEONE(), "match", &[text.clone()])) {
    return Py::Int(1);
    }
    if py_truthy(&{ let __b = mcall(&g_LOVE_START(), "match", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:mum|mom|dad|nan|gran|sister|brother|kid|child|parent|family)'?s\\b|\\band (?:my|our|his|her|their)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Int(1);
    }
    return Py::Int(0);
    Py::None
}
pub fn _loves_someone(args: &[Py]) -> Py {
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
    let _ = py_str("True when the whole line is the person saying they love someone of their own.");
    return call_builtin("bool", &[_loves_people(&[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_clock_pool(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The pool with the time worked into the lines, so the reply names it back.");
    at = mcall(&{ let __b = at.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&at.clone()))) {
    return g_AT_THE_CLOCK();
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AT_THE_CLOCK()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("at".to_string(), at.clone()), ("At".to_string(), add(&mcall(&slice_or_raise(&at.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&at.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_time_said(args: &[Py]) -> Py {
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
    let _ = py_str("The clock time in the line, as the person said it, with its match.\n\n    A time said with the little word in front of it (\"by 7.45\", \"for 6am\") and a time said in\n    front of the thing that leaves (\"catch the 7.15 train\") are the same morning, and both have\n    to be quoted back the way they were given, minutes and all.\n    ");
    return { let __b = mcall(&g_A_TIME_SAID(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CLOCK_THE_DEPARTURE(), "search", &[clean.clone()]) } };
    Py::None
}
pub fn _the_morning_has_a_clock(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line puts a clock time on somebody having to be somewhere.");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_CLOCK_ASKED(), "match", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_CLOCK_NOT_THIS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CLOCK_SOMEONE_ELSE(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_CLOCK_ABOUT_THE_JOB(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    said = _the_time_said(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_LENGTH(), "match", &[slice_or_raise(&clean.clone(), &mcall(&said.clone(), "end", &[]), &Py::None, &Py::None)])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_CLOCK_SHAPE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&gattr(&said.clone(), "re"), &g_CLOCK_THE_DEPARTURE())) } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut verdict: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verdict") { v.clone() }
    else { Py::Bool(true) };
    let mut alone: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("One short reply that fits the kind of line, never repeating the last few.\n\n    `verdict=False` is for a caller asking what the composer would say about a line *as a thing*\n    rather than as this turn: the memory store asks it that way (`bot/memory/cause.py`,\n    `_the_bot_would_only_shrug`) to find out whether the words on their own already have an\n    answer, and the verdict pool is the answer to the turn, not to the words. Left in, \"im so\n    tired\" then \"thats annoying\" made the store think the composer had answered and stand down,\n    and the pool's line then lost the turn to chat's - the store's own line about that tiredness\n    is the better answer and the one it gave on main (gray-cypress on #5479).\n    ");
    if py_truthy(&{ let __b = verdict.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    alone = { let __b = crate::gen::memory::ack::a_person_named_alone(&[crate::gen::compose::fallback::_matchable(&[text.clone()]), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::ack::a_person_pointed_at(&[crate::gen::compose::fallback::_matchable(&[text.clone()]), memory.clone()]) } };
    if py_truthy(&alone.clone()) {
    return alone.clone();
    }
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); __kws.extend(vec![("verdict".to_string(), verdict.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reply_for(args))(&__args) };
    if py_truthy(&{ let __b = verdict.clone(); if !py_truthy(&__b) { __b } else { { let __b = said.clone(); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::is_a_hedge(&[said.clone()]) } } } }) {
    clean = crate::gen::compose::fallback::_matchable(&[text.clone()]);
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); __kws.extend(vec![("past_a_question".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::fallback::_a_verdict_on_their_plan(args))(&__args) };
    if py_truthy(&pool.clone()) {
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = verdict.clone(); if !py_truthy(&__b) { __b } else { said.clone() } }) {
    let mut composer: Py = crate::gen::compose::fallback::module_obj();
    if py_truthy(&{ let __b = mcall(&composer.clone(), "nothing_said_yet", &[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&composer.clone(), "brings_nothing_of_its_own", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&composer.clone(), "assumes_an_earlier_turn", &[said.clone()]) } } } }) {
    return _choose(&[gattr(&composer.clone(), "NOTHING_SAID_YET"), memory.clone(), crate::gen::compose::fallback::_matchable(&[text.clone()])]);
    }
    }
    return said.clone();
    Py::None
}
pub fn _options_already_up(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the things to choose between were named in the last turn or two.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &neg(&Py::Int(2)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    asked = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = call_builtin("len", &[mcall(&g_OPTION_ON_THE_TABLE(), "findall", &[mcall(&said.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&asked.clone(), "lower", &[]), &py_str(" or "))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _reads_value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pred: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pred") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pred"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let _ = py_str("The pool a cue picks out of the typed line, or out of the same line with its words put\n    right, or None.\n\n    `_reads` is the same idea for a cue that only says yes or no. The pools that pick a reply out\n    of a line need the pool itself back, and they were handed the typed line alone: \"im ment to be\n    goin to a weddin nxt week\" was answered \"I'm listening - say more if it helps\" while the same\n    line spelled properly got \"Something to look forward to, then\", because \"next week\" is a\n    spelling the reading puts right and the pool never saw the mended line. As in `_reads`, the\n    typed line is asked first, so a line that needs no repair is untouched.\n    ");
    found = if py_truthy(&Py::Bool(py_is(&memory.clone(), &Py::None))) { py_call(&pred, &[clean.clone()]) } else { py_call(&pred, &[clean.clone(), memory.clone()]) };
    if py_truthy(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&repaired.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&repaired.clone(), &clean.clone())) } } } }) {
    return found.clone();
    }
    return if py_truthy(&Py::Bool(py_is(&memory.clone(), &Py::None))) { py_call(&pred, &[repaired.clone()]) } else { py_call(&pred, &[repaired.clone(), memory.clone()]) };
    Py::None
}
pub fn _a_countdown_in_a_care_thread(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut understood: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut count: Py = Py::None;
    let mut line: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The date, said plainly, in a thread where the last real answer was an emergency.");
    let mut in_a_safety_thread: Py = py_func("in_a_safety_thread", |args: &[Py]| crate::gen::memory::topics::in_a_safety_thread(args));
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()])))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = repaired.clone(); if py_truthy(&__b) { __b } else { { let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_COUNTDOWN_ASK(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_COUNTDOWN_TO(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    let mut solve: Py = py_func("solve", |args: &[Py]| crate::gen::knowledge::maths::solve(args));
    reading = py_dict(vec![(py_str("clean"), said.clone()), (py_str("words"), mcall(&said.clone(), "split", &[])), (py_str("kind"), py_str("question"))]);
    count = py_call(&solve, &[said.clone(), reading.clone(), py_dict(vec![])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[count.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&count.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_COUNTED(), "match", &[mcall(&mcall(&count.clone(), "strip", &[]), "lower", &[])])))) {
    return Py::None;
    }
    count = add(&mcall(&mcall(&count.clone(), "strip", &[]), "rstrip", &[py_str(".")]), &py_str("."));
    return _choose(&[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COUNTDOWN_AFTER_CARE()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("count".to_string(), count.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _reply_for(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut verdict: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verdict") { v.clone() }
    else { Py::Bool(true) };
    let mut about: Py = Py::None;
    let mut about_a_person: Py = Py::None;
    let mut action: Py = Py::None;
    let mut after: Py = Py::None;
    let mut again: Py = Py::None;
    let mut ahead_pool: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut answered: Py = Py::None;
    let mut arranged: Py = Py::None;
    let mut arrived: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asking: Py = Py::None;
    let mut asks_for_help: Py = Py::None;
    let mut asks_for_words: Py = Py::None;
    let mut at_a_time: Py = Py::None;
    let mut awkward: Py = Py::None;
    let mut axis: Py = Py::None;
    let mut birthday: Py = Py::None;
    let mut body_lines: Py = Py::None;
    let mut bound: Py = Py::None;
    let mut broken_into: Py = Py::None;
    let mut bye: Py = Py::None;
    let mut cannot: Py = Py::None;
    let mut cannot_see: Py = Py::None;
    let mut care: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut condition: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut disliked: Py = Py::None;
    let mut doable: Py = Py::None;
    let mut drafted: Py = Py::None;
    let mut free_time: Py = Py::None;
    let mut going_on: Py = Py::None;
    let mut got_ahead: Py = Py::None;
    let mut handed: Py = Py::None;
    let mut has_happened: Py = Py::None;
    let mut help_topic: Py = Py::None;
    let mut hurt: Py = Py::None;
    let mut idiom: Py = Py::None;
    let mut invited: Py = Py::None;
    let mut is_a_child: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut landed: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut leaning: Py = Py::None;
    let mut limits: Py = Py::None;
    let mut line: Py = Py::None;
    let mut live: Py = Py::None;
    let mut lost: Py = Py::None;
    let mut loved: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut mood: Py = Py::None;
    let mut mooded: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_after: Py = Py::None;
    let mut narrowed: Py = Py::None;
    let mut no_words: Py = Py::None;
    let mut number: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut of_theirs: Py = Py::None;
    let mut one_thing: Py = Py::None;
    let mut opens_with_hello: Py = Py::None;
    let mut ordered: Py = Py::None;
    let mut own: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut place_ask: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut planned: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut positive: Py = Py::None;
    let mut promise: Py = Py::None;
    let mut reaction: Py = Py::None;
    let mut refused: Py = Py::None;
    let mut regretted: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut reported: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut restated: Py = Py::None;
    let mut returned: Py = Py::None;
    let mut routed: Py = Py::None;
    let mut routine: Py = Py::None;
    let mut safety_ask: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut saying_ask: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut situation: Py = Py::None;
    let mut state: Py = Py::None;
    let mut step: Py = Py::None;
    let mut suggestion: Py = Py::None;
    let mut sum_said: Py = Py::None;
    let mut summons: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut the_day: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut tips: Py = Py::None;
    let mut together: Py = Py::None;
    let mut told: Py = Py::None;
    let mut tried_reply: Py = Py::None;
    let mut unknown: Py = Py::None;
    let mut view: Py = Py::None;
    let mut w: Py = Py::None;
    let mut was_taste: Py = Py::None;
    let mut wear: Py = Py::None;
    let mut who: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut why_follow_up: Py = Py::None;
    let mut wondered: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let mut wrecked: Py = Py::None;
    let _ = py_str("The composer's reply to this turn (`reply_for`), with the reading already made.");
    if py_truthy(&Py::Bool(py_is(&understood.clone(), &Py::None))) {
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    understood = py_call(&crate::gen::compose::fallback::g_understand(), &[text.clone(), memory.clone()]);
    }
    state = mcall(&memory.clone(), "setdefault", &[py_str("compose"), py_dict(vec![])]);
    was_taste = call_builtin("bool", &[mcall(&state.clone(), "get", &[py_str("taste")])]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&state.clone(), &py_str("taste"), Py::Bool(false));
    kind = { let __b = mcall(&understood.clone(), "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("statement") } };
    words = { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]) } };
    counted = _a_countdown_in_a_care_thread(&[memory.clone(), clean.clone(), text.clone(), understood.clone()]);
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    after = crate::gen::compose::fallback::_after_safety(&[memory.clone(), clean.clone(), understood.clone()]);
    if py_truthy(&after.clone()) {
    return after.clone();
    }
    let mut childwords: Py = crate::gen::compose::childwords::module_obj();
    if py_truthy(&mcall(&childwords.clone(), "asks_the_words_for_a_child", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { clean.clone() } }, memory.clone()])) {
    return _choose(&[gattr(&childwords.clone(), "THE_WORDS_FOR_A_CHILD"), memory.clone(), clean.clone()]);
    }
    let mut talktime: Py = crate::gen::compose::talktime::module_obj();
    at_a_time = mcall(&talktime.clone(), "answer_for", &[clean.clone(), memory.clone()]);
    if py_truthy(&at_a_time.clone()) {
    return at_a_time.clone();
    }
    if py_truthy(&crate::gen::compose::morning_after::reads_as_a_morning_after(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::morning_after::g_A_MORNING_AFTER(), memory.clone(), clean.clone()]);
    }
    hurt = _somebody_hurt(&[clean.clone(), memory.clone(), understood.clone()]);
    if py_truthy(&hurt.clone()) {
    return hurt.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::_a_care_thread_to_leave_alone(&[memory.clone()])))) {
    safety_ask = crate::gen::compose::safetyask::reply_for(&[clean.clone(), memory.clone()]);
    if py_truthy(&safety_ask.clone()) {
    return _choose(&[safety_ask.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_HANDS_IT_BACK(), "match", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_options_already_up(&[memory.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_HANDS_BACK(), memory.clone(), clean.clone()]);
    }
    returned = crate::gen::compose::fallback::reads_as_a_return(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&returned.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_came_back(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_grief_thread(&[memory.clone()]))) } } } }) {
    if py_truthy(&returned.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), returned.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[crate::gen::compose::fallback_data::g_RETURNED_TO_A_THING(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::_a_chat_behind_it(&[memory.clone()])))) {
    return _choose(&[crate::gen::compose::fallback_data::g_RETURNED_TO_IT_FIRST(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback_data::g_RETURNED_TO_IT(), memory.clone(), clean.clone()]);
    }
    drafted = crate::gen::compose::fallback::_named_draft(&[memory.clone(), text.clone(), words.clone(), clean.clone()]);
    if py_truthy(&drafted.clone()) {
    return drafted.clone();
    }
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("agreement")])) {
    return _choose(&[crate::gen::compose::fallback::g_AGREEMENT(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("tight_on_money")])) {
    pool = crate::gen::compose::fallback::g_TIGHT_MONEY();
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("amount_left")])) {
    pool = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&mcall(&line.clone(), "lower", &[]), &py_str("what's left")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::a_continuation_of_the_tight_week(&[memory.clone(), clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_TIGHT_MONEY_MORE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_low_day(&[clean.clone(), understood.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::compose::fallback::g_LOW_DAY_REPLIES()); __args.push(memory.clone()); __args.push(clean.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_HAND_BACK(), "match", &[crate::gen::compose::fallback::_self_line(&[text.clone()])]); if !py_truthy(&__b) { __b } else { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::compose::fallback::g_HOW_ARE_YOU()); __args.push(memory.clone()); __args.push(clean.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    verdict = if py_truthy(&verdict.clone()) { crate::gen::compose::fallback::_verdict_pool(&[clean.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = verdict.clone(); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]) } }) {
    verdict = Py::None;
    } else {
    if py_truthy(&{ let __b = verdict.clone(); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::a_happening_now(&[memory.clone()]) } }) {
    verdict = Py::None;
    } else {
    if py_truthy(&{ let __b = verdict.clone(); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::a_loss_in_the_thread(&[memory.clone()]) } }) {
    verdict = crate::gen::compose::fallback::_the_heavy_pool(&[memory.clone()]);
    }
    }
    }
    tried_reply = if py_truthy(&verdict.clone()) { Py::None } else { crate::gen::compose::fallback::_after_advice(&[memory.clone(), clean.clone(), words.clone()]) };
    if py_truthy(&tried_reply.clone()) {
    return tried_reply.clone();
    }
    refused = crate::gen::compose::fallback::_a_refusal_to_answer(&[clean.clone(), memory.clone(), words.clone()]);
    if py_truthy(&refused.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(refused.clone()); __args.push(memory.clone()); __args.push(clean.clone()); __kws.extend(vec![("seed_text".to_string(), { let __b = crate::gen::compose::fallback::the_advice(&[memory.clone()]); if py_truthy(&__b) { __b } else { clean.clone() } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    doable = crate::gen::compose::fallback::_a_doable(&[clean.clone(), memory.clone()]);
    if py_truthy(&doable.clone()) {
    return doable.clone();
    }
    if py_truthy(&verdict.clone()) {
    return _choose(&[verdict.clone(), memory.clone(), clean.clone()]);
    }
    answered = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { crate::gen::compose::reaction::reply_for(&[clean.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&answered.clone()) {
    return answered.clone();
    }
    reaction = crate::gen::compose::fallback::_reaction_pool(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&reaction.clone(), &crate::gen::compose::fallback::g_REACTION_REPEAT()))) {
    again = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { crate::gen::compose::why::said_again(&[clean.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&again.clone()) {
    return again.clone();
    }
    }
    if py_truthy(&{ let __b = reaction.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_a_care_thread_to_leave_alone(&[memory.clone()]))) } }) {
    if py_truthy(&crate::gen::compose::fallback::_the_person_already_said_it(&[memory.clone()])) {
    reaction = mcall(&crate::gen::compose::fallback::g_REACTIONS_SAID(), "get", &[reaction.clone(), reaction.clone()]);
    }
    return _choose(&[reaction.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = reaction.clone(); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_care_thread_to_leave_alone(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&reaction.clone(), &crate::gen::compose::fallback::g_REACTION_REPEAT())) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_REACTIONS_CARE(), memory.clone(), clean.clone()]);
    }
    the_day = crate::gen::compose::fallback::_the_day_described(&[clean.clone()]);
    if py_truthy(&{ let __b = the_day.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_a_care_thread_to_leave_alone(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_the_turn_before_was_a_heavy_one(&[memory.clone(), clean.clone()]))) } } } } } }) {
    return _choose(&[the_day.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_FLAT_LINE(), "match", &[mcall(&py_str(" "), "join", &[words.clone()])]) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_FLAT(), memory.clone(), clean.clone()]);
    }
    view = crate::gen::compose::fallback::_the_view_asked_for(&[clean.clone(), memory.clone()]);
    if py_truthy(&view.clone()) {
    return view.clone();
    }
    no_words = crate::gen::compose::fallback::_no_words(&[clean.clone()]);
    if py_truthy(&no_words.clone()) {
    return _choose(&[no_words.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("thanks"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    kind = py_str("question");
    }
    asks_for_help = crate::gen::compose::fallback::_asks_for_help(&[crate::gen::compose::fallback::_plain(&[clean.clone()]), understood.clone()]);
    opens_with_hello = { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("statement"))); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_reads_as_greeting(&[words.clone(), text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_reads_as_question(&[text.clone(), words.clone()]))) } } } };
    if py_truthy(&{ let __b = opens_with_hello.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asks_for_help.clone())); if !py_truthy(&__b) { __b } else { { let __b = py_call(&crate::gen::compose::fallback::g_greeting_only(), &[words.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("intent")]), &py_str("request"))) } } } } } }) {
    kind = py_str("greeting");
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("empty")))) {
    return _choose(&[crate::gen::compose::fallback::g_EMPTY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("greeting"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_for_help.clone())) } }) {
    return _choose(&[crate::gen::compose::fallback::g_GREETING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("thanks")))) {
    if py_truthy(&crate::gen::compose::fallback::_thanks_news_went_badly(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_THANKS_NO_BETTER(), memory.clone(), clean.clone()]);
    }
    bye = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); __kws.extend(vec![("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::leaving::pool(args))(&__args) };
    if py_truthy(&bye.clone()) {
    return _choose(&[bye.clone(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_THANKS(), memory.clone(), clean.clone()]);
    }
    loved = _loves_people(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kind.clone(), &py_str("question"))); if !py_truthy(&__b) { __b } else { loved.clone() } }) {
    return _choose(&[if py_truthy(&gt(&loved.clone(), &Py::Int(1))) { crate::gen::compose::fallback_data::g_A_LOVE_FOR_PEOPLE() } else { crate::gen::compose::fallback_data::g_A_LOVE_FOR_SOMEONE() }, memory.clone(), clean.clone()]);
    }
    going_on = crate::gen::compose::handoff::reply_for(&[clean.clone(), memory.clone()]);
    if py_truthy(&going_on.clone()) {
    return going_on.clone();
    }
    restated = if py_truthy(&mcall(&memory.clone(), "get", &[py_str("correction_answered")])) { py_str("") } else { _restated(&[understood.clone()]) };
    if py_truthy(&restated.clone()) {
    reply = _answers_the_correction(&[restated.clone(), memory.clone()]);
    if py_truthy(&reply.clone()) {
    return reply.clone();
    }
    }
    corrected = crate::gen::memory::ack::value_put_right(&[clean.clone(), memory.clone()]);
    if py_truthy(&corrected.clone()) {
    return corrected.clone();
    }
    wondered = crate::gen::memory::ack::_a_condition_they_wonder_about(&[clean.clone(), memory.clone()]);
    if py_truthy(&wondered.clone()) {
    return wondered.clone();
    }
    plain = crate::gen::compose::fallback::_plain(&[clean.clone()]);
    repaired = { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { clean.clone() } };
    if py_truthy(&crate::gen::compose::fallback::_is_presence_check(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_PRESENT(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_a_declined_plan(&[plain.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::draft::drafter(&[memory.clone(), clean.clone()]))) } }) {
    if py_truthy(&crate::gen::compose::fallback::_a_hard_week(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_DECLINED_PLAN_GENTLE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_the_turn_before_was_a_heavy_one(&[memory.clone(), plain.clone()])) {
    } else {
    if py_truthy(&crate::gen::compose::fallback::_someone_else_dropped_it(&[plain.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_PLAN_KNOCKED(), memory.clone(), clean.clone()]);
    } else {
    return _choose(&[crate::gen::compose::fallback::g_DECLINED_PLAN(), memory.clone(), clean.clone()]);
    }
    }
    }
    help_topic = crate::gen::compose::fallback::_help_topic(&[clean.clone(), understood.clone()]);
    if py_truthy(&help_topic.clone()) {
    return _choose(&[help_topic.clone(), memory.clone(), clean.clone()]);
    }
    leaning = crate::gen::compose::fallback::_a_leaning_side(&[plain.clone(), memory.clone()]);
    if py_truthy(&leaning.clone()) {
    return leaning.clone();
    }
    mine = crate::gen::compose::fallback::_my_pick(&[plain.clone(), memory.clone()]);
    if py_truthy(&mine.clone()) {
    return mine.clone();
    }
    verdict = crate::gen::compose::fallback::_a_verdict_on_their_plan(&[plain.clone(), memory.clone()]);
    if py_truthy(&verdict.clone()) {
    return _choose(&[verdict.clone(), memory.clone(), clean.clone()]);
    }
    place_ask = crate::gen::compose::fallback::_a_place_asked_about(&[plain.clone(), memory.clone()]);
    if py_truthy(&place_ask.clone()) {
    return _choose(&[place_ask.clone(), memory.clone(), clean.clone()]);
    }
    saying_ask = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); __args.push(understood.clone()); __kws.extend(vec![("body".to_string(), crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::why::respond_to_a_saying_ask(args))(&__args) };
    if py_truthy(&saying_ask.clone()) {
    return saying_ask.clone();
    }
    about = crate::gen::compose::fallback::_about(&[plain.clone(), understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("self_doing"), py_str("clothes")]), &about.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fallback::g_ANSWERING_A_QUOTED_ASK(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_the_answers_take_it(&[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } } } }) {
    about = Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&about.clone(), &py_str("memory"))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_tally_of_its_own_day(&[understood.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_ABOUT_TALLY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&about.clone(), &py_str("hand")))) {
    return _choose(&[crate::gen::compose::fallback::g_HAND(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_ABOUT(), &about.clone()))) {
    return _choose(&[geti(&crate::gen::compose::fallback::g_ABOUT(), &about.clone()), memory.clone(), clean.clone()]);
    }
    if py_truthy(&about.clone()) {
    return _choose(&[geti(&crate::gen::compose::fallback::g_ABOUT_SELF(), &about.clone()), memory.clone(), clean.clone()]);
    }
    awkward = crate::gen::compose::fallback::_their_own_slip(&[clean.clone(), memory.clone()]);
    if py_truthy(&awkward.clone()) {
    said = _choose(&[awkward.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    said = _choose(&[if py_truthy(&Py::Bool(py_is(&awkward.clone(), &crate::gen::compose::fallback::g_LAUGHED_AT()))) { crate::gen::compose::fallback::g_EMBARRASSED() } else { crate::gen::compose::fallback::g_LAUGHED_AT() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_SELF_SUBJECT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_SELF_BLAME_ASKS(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_WORDING_ASKS(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_RATES_A_SKILL(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_own_feeling(&[clean.clone()]))) } } } } } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_SELF_BLAME(), memory.clone(), clean.clone()]);
    }
    asking = { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("question"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("statement"))); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_reads_as_question(&[text.clone(), words.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_reads_as_a_question_when_repaired(&[repaired.clone()]) } } } } } } } };
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_thanks_then_said(&[clean.clone()]) } }) {
    asking = Py::Bool(false);
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_acknowledges_alone(&[clean.clone()]) } }) {
    asking = Py::Bool(false);
    }
    action = crate::gen::compose::fallback::_action(&[clean.clone(), memory.clone()]);
    if py_truthy(&action.clone()) {
    return action.clone();
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_WOULD_HAVE_BEEN(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_MILESTONE(), memory.clone(), clean.clone()]);
    }
    bye = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); __kws.extend(vec![("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::leaving::pool(args))(&__args) };
    if py_truthy(&bye.clone()) {
    return _choose(&[bye.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::leaving::reads_as_a_leave_with_a_body_in_it(&[clean.clone(), words.clone()])) {
    bye = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); __kws.extend(vec![("unwell_blocks".to_string(), Py::Bool(false)), ("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::leaving::pool(args))(&__args) };
    if py_truthy(&bye.clone()) {
    return _choose(&[bye.clone(), memory.clone(), clean.clone()]);
    }
    }
    asks_for_words = { let __b = crate::gen::compose::fallback::_asks_for_words(&[clean.clone(), words.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_asks_for_words(&[repaired.clone(), words.clone()]) } } } };
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_WORDING_ASKS_WHO(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::compose::fallback::g_EXPERT_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    asks_for_words = Py::Bool(true);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_QUOTE_ASKS(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_QUOTE_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&asks_for_words.clone()) {
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_TELL_HARD(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_TELL_HARD_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("card")))) {
    return _choose(&[crate::gen::compose::fallback::g_CARD_LINES(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_WORDING_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_asks_how_are_you(&[plain.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("asks_how_i_am")]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_HOW_ARE_YOU(), memory.clone(), clean.clone()]);
    }
    taste = crate::gen::compose::fallback::_taste(&[plain.clone(), memory.clone(), was_taste.clone(), asking.clone(), repaired.clone()]);
    if py_truthy(&taste.clone()) {
    let _ = py_setitem(&state.clone(), &py_str("taste"), Py::Bool(true));
    return taste.clone();
    }
    number = crate::gen::compose::fallback::_their_contact_number(&[plain.clone(), asking.clone()]);
    if py_truthy(&number.clone()) {
    return _choose(&[number.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_no_from_the_bot(&[plain.clone()])) {
    cannot = { let __b = crate::gen::compose::fallback::_live(&[plain.clone(), Py::Bool(true), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_the_live_thing_named(&[plain.clone()]) } };
    if py_truthy(&cannot.clone()) {
    return _choose(&[cannot.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_CANNOT_SEE_ANYTHING(), "search", &[plain.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_CANNOT_SEE_ANYTHING_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_CANNOT_READ_YOURS(), "search", &[plain.clone()])) {
    return _choose(&[{ let __b = crate::gen::compose::fallback::_what_no_eyes_already_says(&[plain.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::g_CANNOT_YOUR_OWN() } }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_SO_NOTHING_LIVE(), "search", &[plain.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_the_bot_says_it_cannot(&[plain.clone()]); if !py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_CANNOT_ANYTHING_ASK(), "search", &[plain.clone()]) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_CANNOT_ANYTHING(), memory.clone(), clean.clone()]);
    }
    }
    live = crate::gen::compose::fallback::_live(&[plain.clone(), asking.clone(), memory.clone()]);
    if py_truthy(&live.clone()) {
    return _choose(&[live.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::loss::found_what_was_lost(&[clean.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::loss::lost_thing_to_find(&[{ let __b = py_call(&crate::gen::compose::fallback::g_last_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_FOUND_IT(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::loss::locked_out(&[clean.clone()])) {
    if py_truthy(&crate::gen::compose::loss::locked_out_of_the_car(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_LOCKED_OUT_OF_THE_CAR(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_LOCKED_OUT_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::loss::keeps_losing_thing(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_LOST_A_THING_AGAIN(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::loss::lost_thing_to_find(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_LOST_A_THING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::unseen::reads(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::unseen::g_LINES(), memory.clone(), clean.clone()]);
    }
    own = { let __b = crate::gen::compose::fallback::_own_feeling(&[clean.clone()]); if py_truthy(&__b) { __b } else { if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_LOST_QUESTION(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_NOT_SELF(), "search", &[clean.clone()]))) } }) { crate::gen::compose::fallback::g_NO_DIRECTION_REPLY() } else { Py::None } } };
    if py_truthy(&own.clone()) {
    return _choose(&[own.clone(), memory.clone(), clean.clone()]);
    }
    routine = _routine_pool(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&routine.clone()) {
    return _choose(&[routine.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_I_RATE_A_SKILL(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_WORDING_ASKS(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_A_WAY_OF_BEING(), "search", &[clean.clone()]))) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_ONE_SKILL_RATED(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_SELF_BLAME_ASKS(), "search", &[clean.clone()])))) {
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_A_SKILL_ASKED_OF_ME(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_A_WAY_OF_BEING(), "search", &[clean.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_ASKED_A_RATING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_A_GOODNESS_ASKED_OF_ME(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_ASKED_IF_GOOD(), memory.clone(), clean.clone()]);
    }
    }
    why_follow_up = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); __args.push(understood.clone()); __kws.extend(vec![("body".to_string(), crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::why::respond(args))(&__args) };
    if py_truthy(&why_follow_up.clone()) {
    return why_follow_up.clone();
    }
    asked = _act_asked_of_me(&[clean.clone()]);
    if py_truthy(&asked.clone()) {
    return _choose(&[asked.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_acts_for_me(&[clean.clone(), memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_ACTS_FOR_ME(), memory.clone(), clean.clone()]);
    }
    ordered = _an_order_it_cannot_do(&[clean.clone()]);
    if py_truthy(&ordered.clone()) {
    return _choose(&[ordered.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g_reads_like_a_symptom(), &[mcall(&repaired.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g__LUMP(), "search", &[repaired.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_LUMP(), memory.clone(), clean.clone()]);
    }
    settled = add(&add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9']"), py_str(" "), clean.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g_reads_like_a_symptom(), &[mcall(&repaired.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_who_for(&[clean.clone(), settled.clone(), mcall(&mcall(&settled.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&crate::gen::compose::trouble::reads_as_trouble(&[clean.clone(), words.clone(), repaired.clone()]), &py_str("sore"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g__CAUGHT(), "search", &[clean.clone()]))) } })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); __kws.extend(vec![("illness".to_string(), call_builtin("bool", &[mcall(&crate::gen::compose::fallback::g__ILL(), "search", &[repaired.clone()])]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::gen::compose::fallback::g_says_it_has_lingered(), args))(&__args) } } } } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_SYMPTOM_LINGERS(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = py_call(&crate::gen::compose::fallback::g_reads_like_a_symptom(), &[mcall(&repaired.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_who_for(&[clean.clone(), settled.clone(), mcall(&mcall(&settled.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&crate::gen::compose::trouble::reads_as_trouble(&[clean.clone(), words.clone(), repaired.clone()]), &py_str("sore"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g__CAUGHT(), "search", &[clean.clone()]))) } })) } } } } } }) {
    return _choose(&[if py_truthy(&mcall(&crate::gen::compose::fallback::g__ILL(), "search", &[repaired.clone()])) { crate::gen::compose::fallback::g_SYMPTOM_COLD() } else { crate::gen::compose::fallback::g_SYMPTOM() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_lateness_report(&[clean.clone(), { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    pool = if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("behind"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } }, &py_str("behind"))) } }) { crate::gen::compose::fallback::g_LATE_BEHIND_LINES() } else { crate::gen::compose::fallback::g_LATE_NOW_LINES() };
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    invited = crate::gen::compose::fallback::_an_invitation(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_eq(&invited.clone(), &py_str("talk")))) {
    return _choose(&[crate::gen::compose::fallback::g_INVITED_TO_TALK(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&invited.clone()) {
    return _choose(&[crate::gen::compose::fallback::g_INVITED(), memory.clone(), clean.clone()]);
    }
    limits = { let __b = crate::gen::compose::fallback::the_limits_said(&[clean.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![py_tuple(vec![crate::gen::compose::fallback::reads_as_a_narrowing(&[clean.clone()]), py_str("")])]) } };
    let __iter1 = py_iter_or_raise(&limits.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    axis = __unpk2[0].clone();
    part = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&axis.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::the_answer_to_cut(&[memory.clone(), axis.clone()]))) } }) {
    continue;
    }
    bound = crate::gen::compose::fallback::the_bound_said(&[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { clean.clone() } }]);
    if py_truthy(&bound.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("bound".to_string(), bound.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[geti(&crate::gen::compose::fallback::g_BOUND_FOR(), &axis.clone()), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return _choose(&[geti(&crate::gen::compose::fallback::g_CUT_FOR(), &axis.clone()), memory.clone(), clean.clone()]);
    }
    refused = { let __b = crate::gen::compose::fallback::_a_carer_refused(&[clean.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_carer_refused(&[repaired.clone()]) } };
    if py_truthy(&{ let __b = refused.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_grief_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]) } } } })) } } } }) {
    pool = geti(&py_dict(vec![(py_str("aid"), crate::gen::compose::fallback::g_ELDER_AID_REPLIES()), (py_str("clinic"), crate::gen::compose::fallback::g_ELDER_CLINIC_REPLIES()), (py_str("doctor"), crate::gen::compose::fallback::g_ELDER_DOCTOR_REPLIES()), (py_str("help"), crate::gen::compose::fallback::g_ELDER_HELP_REPLIES())]), &refused.clone());
    said_before = call_builtin("str", &[{ let __b = mcall(&geti(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![py_dict(vec![])]) } }, &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_pool(&[said_before.clone(), py_tuple(vec![crate::gen::compose::fallback::g_ELDER_AID_REPLIES(), crate::gen::compose::fallback::g_ELDER_CLINIC_REPLIES(), crate::gen::compose::fallback::g_ELDER_DOCTOR_REPLIES(), crate::gen::compose::fallback::g_ELDER_HELP_REPLIES()])])))) {
    return _choose(&[pool.clone(), memory.clone(), clean.clone(), add(&add(&clean.clone(), &py_str(" ")), &crate::gen::compose::fallback::_the_line_before(&[memory.clone()]))]);
    }
    }
    handed = crate::gen::compose::no_eyes::handed_you_something(&[clean.clone()]);
    if py_truthy(&handed.clone()) {
    return _choose(&[handed.clone(), memory.clone(), clean.clone()]);
    }
    body_lines = crate::gen::compose::no_eyes::look_at_a_body_lines(&[clean.clone(), Py::None, memory.clone()]);
    if py_truthy(&body_lines.clone()) {
    return _choose(&[body_lines.clone(), memory.clone(), clean.clone()]);
    }
    cannot_see = crate::gen::compose::no_eyes::cannot_see_your_thing(&[clean.clone()]);
    if py_truthy(&cannot_see.clone()) {
    return _choose(&[cannot_see.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_am_i_a_nuisance(&[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_grief_thread(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]))) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_SORRY_FOR_ASKING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_takes_something_back(&[clean.clone(), memory.clone()]))) } }) {
    if py_truthy(&{ let __b = reads_as_no_pleasure(&[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { reads_as_no_pleasure(&[repaired.clone()]) } } } }) {
    return _choose(&[g_PLEASURE_GONE_LINES(), memory.clone(), clean.clone()]);
    }
    step = py_call(&crate::gen::compose::fallback::g_step_when_low(), &[clean.clone(), memory.clone(), crate::gen::compose::fallback::g_COMFORT_SAID()]);
    if py_truthy(&step.clone()) {
    return step.clone();
    }
    if py_truthy(&{ let __b = reads_as_a_question_about_the_bot(&[clean.clone()]); if py_truthy(&__b) { __b } else { reads_as_a_question_about_the_bot(&[repaired.clone()]) } }) {
    return _choose(&[g_ABOUT_THE_BOT_LINES(), memory.clone(), clean.clone()]);
    }
    unknown = crate::gen::compose::fallback::_unknown(&[understood.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&{ let __b = _a_wear_ask_about_a_day(&[clean.clone(), repaired.clone(), text.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::is_a_hedge(&[unknown.clone()]) } }) {
    return _choose(&[_the_wear_pool_for_the_line(&[clean.clone(), repaired.clone(), text.clone()]), memory.clone(), clean.clone()]);
    }
    return unknown.clone();
    }
    drafted = crate::gen::compose::draft::drafter(&[memory.clone(), text.clone(), words.clone()]);
    if py_truthy(&drafted.clone()) {
    return _choose(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::draft::g_LEAD_INS()) { let mut lead: Py = __item.clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&lead.clone(), -1, "")); __s.push_str(" \""); __s.push_str(&fstr(&drafted.clone(), -1, "")); __s.push_str("\""); py_string(__s) }); } py_list(__out) })() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::draft::not_the_message(&[memory.clone(), text.clone()])) {
    return _choose(&[crate::gen::compose::draft::g_NOT_YET(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_pregnancy_bleeding(), clean.clone(), repaired.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_PREGNANCY_BLEEDING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::a_fresh_death_and_the_words(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_THE_FIRST_HOUR_AND_A_CHILD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_sudden_loss(), clean.clone(), repaired.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_SUDDEN_LOSS(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::compose::fallback::g_reads_as_recovery()); __args.push(clean.clone()); __args.push(repaired.clone()); __kws.extend(vec![("lower".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_RELIEF(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::compose::fallback::g_reads_as_a_clear_result()); __args.push(clean.clone()); __args.push(repaired.clone()); __kws.extend(vec![("lower".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_RELIEF(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g_reads_as_settled(), &[mcall(&clean.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { { let __b = py_call(&crate::gen::compose::fallback::g_worry_on_the_table(), &[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]))) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_RELIEF(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_a_closed_door(), clean.clone(), repaired.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_CLOSED_DOOR(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_came_back(&[clean.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback_data::g_PAUSE_IN_A_SAFETY_THREAD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_came_back(&[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_grief_thread(&[memory.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_CAME_BACK(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = _reads(&[crate::gen::compose::fallback::g_reads_as_the_leaving(), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_the_speakers_own_plan(), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_things_state(&[memory.clone(), clean.clone()]))) } } } }) {
    return _choose(&[if py_truthy(&py_call(&crate::gen::compose::fallback::g_reads_as_a_leaving_already_gone(), &[clean.clone()])) { crate::gen::compose::fallback::g_LEFT_POOL_GONE() } else { crate::gen::compose::fallback::g_LEFT_POOL() }, memory.clone(), clean.clone()]);
    }
    situation = crate::gen::compose::circumstance::pool(&[clean.clone()]);
    if py_truthy(&situation.clone()) {
    return _choose(&[situation.clone(), memory.clone(), clean.clone()]);
    }
    broken_into = crate::gen::compose::remark::break_in_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&broken_into.clone()) {
    return broken_into.clone();
    }
    if py_truthy(&{ let __b = _reads(&[crate::gen::compose::fallback::g_reads_as_bad_news(), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_the_speakers_own_plan(), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_PLAN_IDIOM(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _reads(&[crate::gen::compose::fallback::g_reads_as_an_award_lost(), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::who_settles(&[clean.clone(), memory.clone()]) } })) } } } } } }) {
    if py_truthy(&_a_things_state(&[memory.clone(), clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_A_THING_LOOKS_SAD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_the_leaving(), clean.clone(), repaired.clone()])) {
    return _choose(&[if py_truthy(&py_call(&crate::gen::compose::fallback::g_reads_as_a_leaving_already_gone(), &[clean.clone()])) { crate::gen::compose::fallback::g_LEFT_POOL_GONE() } else { crate::gen::compose::fallback::g_LEFT_POOL() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&crate::gen::compose::remark::reads_as_report(&[clean.clone(), words.clone()]), &py_str("again")))) {
    again = crate::gen::compose::remark::report_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&again.clone()) {
    return again.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { _reads(&[py_func("reads_as_someone_close_is_ill", |args: &[Py]| crate::gen::compose::fallback::reads_as_someone_close_is_ill(args)), clean.clone(), repaired.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_SOMEONE_SERIOUSLY_ILL(), memory.clone(), clean.clone()]);
    }
    pool = if py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_news_with_a_name(), clean.clone(), repaired.clone()])) { crate::gen::compose::fallback::g_NAMED_NEGATIVE() } else { crate::gen::compose::fallback::g_NEGATIVE() };
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _reads(&[gattr(&crate::gen::compose::fallback::g_LEAVING_A_PERSON(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _reads(&[gattr(&crate::gen::compose::fallback::g_LEFT_BEHIND(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_reads(&[gattr(&crate::gen::compose::fallback::g_NOT_LEAVING_ANYONE(), "search"), clean.clone(), repaired.clone()]))) } } } } } } } }) {
    is_a_child = _reads(&[gattr(&crate::gen::compose::fallback::g_A_CHILD(), "search"), clean.clone(), repaired.clone()]);
    has_happened = _reads(&[gattr(&crate::gen::compose::fallback::g_THE_LEAVE_HAS_HAPPENED(), "search"), clean.clone(), repaired.clone()]);
    if py_truthy(&has_happened.clone()) {
    pool = if py_truthy(&is_a_child.clone()) { crate::gen::compose::fallback::g_A_CHILD_LEFT() } else { crate::gen::compose::fallback::g_SOMEONE_GONE() };
    } else {
    pool = _only_what_the_line_gives(&[if py_truthy(&is_a_child.clone()) { crate::gen::compose::fallback::g_A_CHILD_LEAVING() } else { crate::gen::compose::fallback::g_SOMEONE_LEAVING() }, clean.clone()]);
    }
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_reads(&[gattr(&crate::gen::memory::ack::g_COMPLAINT(), "search"), clean.clone(), repaired.clone()])) {
    if py_truthy(&_a_things_state(&[memory.clone(), clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_A_THING_LOOKS_SAD(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_NEGATIVE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = _asks_the_bot(&[understood.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_takes_something_back(&[clean.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_reads_value(&[py_func("_a_good_time", |args: &[Py]| crate::gen::compose::fallback::_a_good_time(args)), clean.clone(), repaired.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_reads(&[crate::gen::compose::fallback::g_reads_as_a_win_nobody_noticed(), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_a_good_time(&[clean.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = crate::gen::compose::fallback::_reads_as_dither(&[clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_weighs_a_choice(&[clean.clone()]) } })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_a_dead_end(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_the_person_they_are_stuck_on(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_acknowledges_alone(&[clean.clone()]))) } } } } } } } } } } } } } } } }) {
    tips = crate::gen::compose::fallback::_tips(&[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&tips.clone()) {
    return tips.clone();
    }
    free_time = crate::gen::compose::fallback::_the_stretch_of_time(&[clean.clone(), memory.clone()]);
    if py_truthy(&free_time.clone()) {
    return free_time.clone();
    }
    lost = crate::gen::compose::fallback::the_lost_ask_reply(&[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&lost.clone()) {
    return lost.clone();
    }
    pool = crate::gen::compose::fallback::unknown_pool(&[clean.clone(), words.clone()]);
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_WORD()))) {
    let mut _drafting_pool: Py = py_func("_drafting_pool", |args: &[Py]| crate::gen::compose::fallback_pools::_drafting_pool(args));
    return _choose(&[py_call(&_drafting_pool, &[clean.clone()]), memory.clone(), clean.clone()]);
    }
    routed = crate::gen::compose::finding_out::for_the_ask(&[clean.clone(), repaired.clone(), understood.clone()]);
    if py_truthy(&routed.clone()) {
    said = _choose(&[routed.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    if py_truthy(&_a_wear_ask_about_a_day(&[clean.clone(), repaired.clone(), text.clone()])) {
    return _choose(&[_the_wear_pool_for_the_line(&[clean.clone(), repaired.clone(), text.clone()]), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_UNKNOWN(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_reads(&[py_func("_is_a_tips_ask", |args: &[Py]| crate::gen::compose::fallback::_is_a_tips_ask(args)), clean.clone(), repaired.clone()])) {
    tips = crate::gen::compose::fallback::_tips(&[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&tips.clone()) {
    return tips.clone();
    }
    return _choose(&[crate::gen::compose::fallback::g_TIPS_LINES(), memory.clone(), clean.clone()]);
    }
    told = crate::gen::compose::fallback::_trouble(&[memory.clone(), repaired.clone(), words.clone()]);
    if py_truthy(&told.clone()) {
    return told.clone();
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::asks_for_more(&[clean.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_answered_recently(&[memory.clone()]) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_HELP_ALREADY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_in_sudden_loss_thread(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::loss::g_AFTER_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_ACKS()) } }) {
    if py_truthy(&crate::gen::compose::fallback::_a_loss_recently(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::loss::g_AFTER_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_the_last_line_was_bad_news(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::loss::g_AFTER_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_the_last_line_was_a_fault(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_ACK_AFTER_TROUBLE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_the_last_line_was_heavy(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_ACK_WARM(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_ACK(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()])))) {
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_FORGETS_A_CLASH(), "search", &[plain.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_DOUBLE_BOOKED(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_FORGET_A_DATE(), "search", &[plain.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_DATES_KEEP_SLIPPING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_FORGET_A_SECRET(), "search", &[plain.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_SECRETS_KEEP_SLIPPING(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_FORGETS_AGAIN(), "search", &[plain.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_KEEPS_FORGETTING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_is_rumination(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_RUMINATION_REPLIES(), memory.clone(), clean.clone()]);
    }
    mood = _mood(&[words.clone(), clean.clone()]);
    carried = crate::gen::compose::trouble::carried_on(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&carried.clone()) {
    return _choose(&[carried.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_near_miss(&[clean.clone()])) {
    return _choose(&[g_NEAR_MISS_REPLY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } } } }) {
    mood = _mood(&[py_call(&crate::gen::compose::fallback::g__words(), &[repaired.clone()]), repaired.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("positive"))); if !py_truthy(&__b) { __b } else { _reads(&[crate::gen::compose::fallback::g_reads_for_a_pet(), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]) } }) {
    if py_truthy(&_reads(&[py_func("reads_as_a_pet_low", |args: &[Py]| crate::gen::compose::badnews::reads_as_a_pet_low(args)), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])])) {
    return _choose(&[crate::gen::compose::fallback_data::g_PET_LOW_REPLY(), memory.clone(), clean.clone()]);
    }
    condition = _reads(&[crate::gen::compose::fallback::g_reads_as_a_pet_condition(), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]);
    return _choose(&[if py_truthy(&condition.clone()) { crate::gen::compose::fallback::g_PET_CONDITION_TROUBLE() } else { crate::gen::compose::fallback::g_PET_TROUBLE() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("positive"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::compose::fallback::g_reads_as_a_carer_body()); __args.push(mcall(&clean.clone(), "lower", &[])); __args.push(mcall(&repaired.clone(), "lower", &[])); __kws.extend(vec![("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads(args))(&__args) } } }) {
    return _choose(&[crate::gen::compose::fallback::g_CARING_FOR_A_BODY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("positive"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::compose::fallback::g_reads_for_a_carer()); __args.push(mcall(&clean.clone(), "lower", &[])); __args.push(mcall(&repaired.clone(), "lower", &[])); __kws.extend(vec![("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads(args))(&__args) } } }) {
    return _choose(&[crate::gen::compose::fallback::g_CARING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = _reads(&[crate::gen::compose::fallback::g_reads_like_a_symptom(), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&crate::gen::compose::trouble::reads_as_trouble(&[clean.clone(), words.clone(), repaired.clone()]), &py_str("sore"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g__CAUGHT(), "search", &[clean.clone()]))) } })) } } } }) {
    return _choose(&[if py_truthy(&mcall(&crate::gen::compose::fallback::g__ILL(), "search", &[repaired.clone()])) { crate::gen::compose::fallback::g_SYMPTOM_COLD() } else { crate::gen::compose::fallback::g_SYMPTOM() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_relationship_ending(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_RELATIONSHIP_DECISION(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::a_comparison_of_standing(&[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_A_COMPARISON_OF_STANDING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::limit::a_limit_on_the_answer(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_something_funny(&[clean.clone(), memory.clone()]) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_A_LAUGH(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_care_thread(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { _the_crowd_is_ahead(&[clean.clone()]) } }) {
    return _choose(&[g_A_CROWD_AHEAD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { _reads_value(&[py_func("_a_good_time", |args: &[Py]| crate::gen::compose::fallback::_a_good_time(args)), clean.clone(), repaired.clone(), memory.clone()]) } }) {
    if py_truthy(&crate::gen::compose::fallback::_in_a_care_thread(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_A_GOOD_TIME_IN_A_CARE_THREAD(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_A_GOOD_TIME(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { _reads(&[crate::gen::compose::fallback::g_reads_as_a_win_nobody_noticed(), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_WIN_IN_A_SILENCE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { _reads(&[py_func("reads_as_a_win_a_cost_came_with", |args: &[Py]| crate::gen::compose::goodnews::reads_as_a_win_a_cost_came_with(args)), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]) } }) {
    return _choose(&[crate::gen::compose::fallback_data::g_WIN_AND_A_COST(), memory.clone(), clean.clone()]);
    }
    occasion = { let __b = py_call(&crate::gen::compose::fallback::g_reads_as_an_occasion(), &[mcall(&clean.clone(), "lower", &[])]); if py_truthy(&__b) { __b } else { if py_truthy(&Py::Bool(!py_eq(&repaired.clone(), &clean.clone()))) { py_call(&crate::gen::compose::fallback::g_reads_as_an_occasion(), &[mcall(&repaired.clone(), "lower", &[])]) } else { Py::None } } };
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { occasion.clone() } }) {
    return _choose(&[crate::gen::compose::fallback::_occasion_pool(&[occasion.clone(), clean.clone()]), memory.clone(), clean.clone()]);
    }
    ahead_pool = _reads_value(&[py_func("_a_day_ahead", |args: &[Py]| crate::gen::compose::fallback::_a_day_ahead(args)), clean.clone(), repaired.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&ahead_pool.clone(), &Py::None)) } }) {
    return _choose(&[ahead_pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { _reads(&[crate::gen::compose::fallback::g_reads_as_news_to_come(), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]) } }) {
    birthday = _a_birthday_still_to_come(&[clean.clone(), memory.clone()]);
    if py_truthy(&birthday.clone()) {
    return _choose(&[_the_birthday_said_back(&[birthday.clone()]), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_NEWS_TO_COME(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&py_call(&crate::gen::memory::ack::g_a_limit_on_the_plan(), &[clean.clone()])) {
    said_back = crate::gen::memory::ack::a_limit_reply(&[text.clone(), memory.clone()]);
    if py_truthy(&said_back.clone()) {
    return said_back.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_dreading(&[clean.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_A_DREAD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_wants_it_over(&[clean.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_A_WANT_IT_OVER(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { _reads_value(&[py_func("_looking_forward", |args: &[Py]| crate::gen::compose::fallback::_looking_forward(args)), clean.clone(), repaired.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_NEWS_TO_COME(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { _reads(&[crate::gen::compose::fallback::g_reads_as_good_news(), mcall(&clean.clone(), "lower", &[]), mcall(&repaired.clone(), "lower", &[])]) } }) {
    return _choose(&[crate::gen::compose::fallback::_says_only_what_they_said(&[crate::gen::compose::fallback::g_CONGRATS(), clean.clone()]), memory.clone(), clean.clone()]);
    }
    arrived = crate::gen::compose::goodnews::new_thing(&[mcall(&clean.clone(), "lower", &[])]);
    if py_truthy(&{ let __b = arrived.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))) } }) {
    return _choose(&[crate::gen::compose::goodnews::new_thing_replies(&[arrived.clone()]), memory.clone(), clean.clone()]);
    }
    regretted = crate::gen::compose::remark::regret_reply_for(&[clean.clone(), words.clone(), memory.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&regretted.clone()) {
    return regretted.clone();
    }
    idiom = mcall(&crate::gen::compose::fallback::g_PLAN_IDIOM(), "search", &[clean.clone()]);
    if py_truthy(&idiom.clone()) {
    return _choose(&[if py_truthy(&Py::Bool(!py_contains(&mcall(&idiom.clone(), "group", &[Py::Int(0)]), &py_str("excit")))) { crate::gen::compose::fallback::g_IDIOM_SHY() } else { crate::gen::compose::fallback::g_IDIOM_GLAD() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_plan_reported(&[clean.clone(), mood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::ack::reads_as_a_trip(&[clean.clone()]))) } }) {
    if py_truthy(&crate::gen::compose::fallback::reads_as_a_change_of_medicine(&[clean.clone()])) {
    whole = if py_truthy(&mcall(&crate::gen::compose::fallback::g_PLAN_MEDICINE_SLOW(), "search", &[clean.clone()])) { crate::gen::compose::fallback::g_PLAN_MEDICINE_SLOW_REPLY() } else { crate::gen::compose::fallback::g_PLAN_MEDICINE_REPLY() };
    return _choose(&[whole.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_PLAN_DONE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_PLAN_NOT_YET(), "search", &[clean.clone()]))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_PLAN_DONE_ACK(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_PLAN(), memory.clone(), clean.clone()]);
    }
    promise = crate::gen::compose::fallback::_a_promise_that_keeps_moving(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&promise.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mood.clone(), &py_str("positive"))) } }) {
    return _choose(&[promise.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("positive"))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_someone_else_minding(&[clean.clone(), words.clone(), crate::gen::compose::fallback::_grievance_thread(&[memory.clone()])]) } }) {
    if py_truthy(&crate::gen::compose::fallback::_grief_thread(&[memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_GRIEF_AFTER(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_GRIEVANCE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_sorry_for_asking(&[clean.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_SORRY_FOR_ASKING(), memory.clone(), clean.clone()]);
    }
    said = crate::gen::compose::fallback::_thanks_then_said(&[clean.clone()]);
    if py_truthy(&crate::gen::compose::fallback::_thanks_news_went_badly(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_THANKS_NO_BETTER(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&said.clone()) {
    said_words = stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_THANKS_GOOD_AGAIN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_THANKS_WENT_BADLY(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&_mood(&[said_words.clone(), said.clone()]), &py_str("negative"))) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_THANKS_THEN_SAID(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_thanks_with_more(&[clean.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_acknowledges_alone(&[clean.clone()]) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_THANKS(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&crate::gen::compose::fallback::g_DITHER(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_DITHER(), "search", &[{ let __b = repaired.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }; if !py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_HELP_SERVICE(), "search", &[clean.clone()]) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("vet"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("vets"))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_ANIMALS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::compose::fallback::g_ANIMALS(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return crate::gen::compose::fallback::g_VET_WHO();
    }
    return _choose(&[crate::gen::compose::fallback::g_UNKNOWN_HEALTH(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&mood.clone(), &py_str("neutral")))) {
    about_a_person = crate::gen::compose::fallback::_a_question_about_the_person(&[clean.clone(), memory.clone()]);
    if py_truthy(&about_a_person.clone()) {
    return about_a_person.clone();
    }
    }
    free_time = crate::gen::compose::fallback::_the_stretch_of_time(&[clean.clone(), memory.clone()]);
    if py_truthy(&free_time.clone()) {
    return free_time.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_reads_as_dither(&[clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_weighs_a_choice(&[clean.clone()]) } } } }) {
    if py_truthy(&{ let __b = { let __b = bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_HEALTH_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_HEALTH_MODIFIER(), "search", &[clean.clone()]))) } }; if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_CHECK_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return _choose(&[crate::gen::compose::fallback::g_UNKNOWN_HEALTH(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_DITHER_YES_NO(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_DITHER_DECIDE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_DITHER_OPEN(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_DITHER_OPEN_LINES(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_DITHER_TWO_WAY_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_run_out(&[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_care_thread(&[memory.clone()]))) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_RUN_OUT_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_only_the_stuck_phrase(&[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_care_thread(&[memory.clone()]))) } } } }) {
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_A_HELP_WORD(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_HAND(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_news_above(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_subject_above(&[memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_bad_line_above(&[memory.clone()]) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_STUCK_AFTER_TROUBLE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::memory::stuck::g_STUCK(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_start_ask(&[clean.clone()]) } }))) {
    return _choose(&[crate::gen::compose::fallback::g_DITHER_OPEN_LINES(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&_a_problem_stated(&[clean.clone(), repaired.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_HAND(), memory.clone(), clean.clone()]);
    }
    amount = crate::gen::compose::quantity::reply_for(&[clean.clone(), memory.clone()]);
    if py_truthy(&amount.clone()) {
    return amount.clone();
    }
    named_after = _named_after_reply(&[clean.clone(), memory.clone()]);
    if py_truthy(&named_after.clone()) {
    return named_after.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&mood.clone(), &py_str("neutral")))) {
    own = crate::gen::compose::fallback::_own_feeling(&[clean.clone()]);
    if py_truthy(&own.clone()) {
    return _choose(&[own.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_knock_on_oneself(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_KNOCKED(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_knock_into_a_thing(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_WALKED_INTO_POOL(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_mishap(&[clean.clone(), words.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_MISHAP_BACK(), memory.clone(), clean.clone()]);
    }
    summons = mcall(&understood.clone(), "get", &[py_str("summons")]);
    if py_truthy(&summons.clone()) {
    return _choose(&[crate::gen::compose::fallback::_summons_pool(&[summons.clone(), clean.clone()]), memory.clone(), clean.clone()]);
    }
    taken = crate::gen::compose::remark::interest_reply_for(&[mcall(&understood.clone(), "get", &[py_str("taking_up")]), memory.clone()]);
    if py_truthy(&taken.clone()) {
    return taken.clone();
    }
    planned = crate::gen::compose::remark::reply_for(&[clean.clone(), words.clone(), memory.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    if py_truthy(&planned.clone()) {
    return planned.clone();
    }
    arranged = crate::gen::compose::remark::arranged_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&arranged.clone()) {
    return arranged.clone();
    }
    pool = crate::gen::compose::fallback::_thing_done_pool(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { pool.clone() } }) {
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    reported = crate::gen::compose::remark::report_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&reported.clone()) {
    return reported.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { reads_as_a_flat_habit(&[text.clone(), understood.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_HABIT_REPLY(), memory.clone(), clean.clone()]);
    }
    together = crate::gen::compose::remark::the_two_of_them(&[clean.clone(), memory.clone()]);
    if py_truthy(&together.clone()) {
    return together.clone();
    }
    of_theirs = crate::gen::compose::remark::their_people_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&of_theirs.clone()) {
    return of_theirs.clone();
    }
    got_ahead = _a_thing_got_ahead(&[clean.clone()]);
    if py_truthy(&got_ahead.clone()) {
    return _choose(&[got_ahead.clone(), memory.clone(), clean.clone()]);
    }
    going_on = crate::gen::compose::remark::their_mood_going_on_reply_for(&[clean.clone(), memory.clone()]);
    if py_truthy(&going_on.clone()) {
    return going_on.clone();
    }
    mooded = crate::gen::compose::remark::their_mood_with_you_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&mooded.clone()) {
    return mooded.clone();
    }
    wear = crate::gen::compose::worn::reads_as_worn(&[clean.clone(), words.clone(), repaired.clone()]);
    if py_truthy(&wear.clone()) {
    return _choose(&[crate::gen::compose::worn::pool(&[wear.clone()]), memory.clone(), clean.clone()]);
    }
    disliked = crate::gen::compose::fallback::the_dislike_ask(&[clean.clone(), memory.clone()]);
    if py_truthy(&disliked.clone()) {
    return disliked.clone();
    }
    said_back = crate::gen::memory::ack::reply(&[text.clone(), memory.clone()]);
    if py_truthy(&crate::gen::compose::remark::a_want_said_back(&[said_back.clone(), clean.clone()])) {
    theirs = crate::gen::compose::remark::dependant_want_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    told = _a_day_told_instead_of_the_receipt(&[said_back.clone(), clean.clone(), memory.clone()]);
    if py_truthy(&told.clone()) {
    return told.clone();
    }
    if py_truthy(&said_back.clone()) {
    return said_back.clone();
    }
    }
    if py_truthy(&Py::Bool(py_eq(&mood.clone(), &py_str("neutral")))) {
    of_theirs = crate::gen::compose::remark::their_people_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&of_theirs.clone()) {
    return of_theirs.clone();
    }
    if py_truthy(&_the_morning_has_a_clock(&[clean.clone()])) {
    said = _the_time_said(&[clean.clone()]);
    return _choose(&[_the_clock_pool(&[if py_truthy(&said.clone()) { mcall(&said.clone(), "group", &[Py::Int(1)]) } else { py_str("") }]), memory.clone(), clean.clone()]);
    }
    said_back = crate::gen::memory::ack::reply(&[text.clone(), memory.clone()]);
    if py_truthy(&crate::gen::compose::remark::a_want_said_back(&[said_back.clone(), clean.clone()])) {
    theirs = crate::gen::compose::remark::dependant_want_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    told = _a_day_told_instead_of_the_receipt(&[said_back.clone(), clean.clone(), memory.clone()]);
    if py_truthy(&told.clone()) {
    return told.clone();
    }
    if py_truthy(&said_back.clone()) {
    return said_back.clone();
    }
    shape = crate::gen::compose::trouble::reads_as_trouble(&[clean.clone(), words.clone(), repaired.clone()]);
    if py_truthy(&shape.clone()) {
    said = _choose(&[crate::gen::compose::trouble::pool(&[shape.clone(), clean.clone(), memory.clone()]), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&mood.clone(), &py_str("negative")))) {
    shape = crate::gen::compose::trouble::reads_as_trouble(&[clean.clone(), words.clone(), repaired.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shape.clone(), &py_str("sore"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g__CAUGHT(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_FRIGHTENED(), "search", &[clean.clone()]))) } } } }) {
    said = _choose(&[crate::gen::compose::trouble::pool(&[shape.clone(), clean.clone(), memory.clone()]), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    landed = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_grief_thread(&[memory.clone()]) } }))) {
    landed = crate::gen::compose::fallback::_the_news_landed(&[clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&landed.clone(), &py_str("bad")))) {
    return _choose(&[crate::gen::compose::fallback::g_LANDED_BAD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&landed.clone(), &py_str("good")))) {
    return _choose(&[crate::gen::compose::fallback::_says_only_what_they_said(&[crate::gen::compose::fallback::g_CONGRATS(), clean.clone()]), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&landed.clone(), &py_str("landed")))) {
    return _choose(&[crate::gen::compose::fallback::g_PENDING_LANDED(), memory.clone(), clean.clone()]);
    }
    let mut _ask_about: Py = py_func("ask_about_what_to_look_out_for", |args: &[Py]| crate::gen::compose::toknow::ask_about_what_to_look_out_for(args));
    answered = py_call(&_ask_about, &[clean.clone(), memory.clone()]);
    if py_truthy(&answered.clone()) {
    return answered.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&answered.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_an_open_result(&[clean.clone()]) } }) {
    if py_truthy(&crate::gen::compose::fallback::_a_time_in_it(&[clean.clone()])) {
    pool = crate::gen::compose::fallback::g_PENDING_WAITING();
    } else {
    if py_truthy(&crate::gen::compose::fallback::_a_guessed_result(&[clean.clone()])) {
    pool = crate::gen::compose::fallback::g_PENDING_DOUBT();
    } else {
    pool = crate::gen::compose::fallback::g_PENDING_WHEN();
    }
    }
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = opens_with_hello.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))) } }) {
    return _choose(&[crate::gen::compose::fallback::g_GREETING(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_reads_as_a_bad_stretch(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_BAD_STRETCH_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_says_there_is_nothing_to_wear(&[clean.clone(), repaired.clone(), text.clone()])) {
    return _choose(&[g_NOTHING_TO_WEAR_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_wear_ask_about_a_day(&[clean.clone(), repaired.clone(), text.clone()])) {
    return _choose(&[_the_wear_pool_for_the_line(&[clean.clone(), repaired.clone(), text.clone()]), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__A_CORRECTION(), "match", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { crate::gen::memory::swap::rewritten(&[clean.clone(), memory.clone()]) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_UNKNOWN(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&py_call(&crate::gen::compose::fallback::g_verdicts_a_thing(), &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_NEGATIVE(), memory.clone(), clean.clone()]);
    }
    state = _body_state(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&state.clone()))) {
    repaired = { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    state = _body_state(&[repaired.clone()]);
    }
    }
    if py_truthy(&state.clone()) {
    return _choose(&[geti(&crate::gen::compose::fallback::g_BODY_POOLS(), &state.clone()), memory.clone(), clean.clone()]);
    }
    pool = crate::gen::compose::fallback::_thing_done_pool(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { { let __b = pool.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_things_state(&[memory.clone(), clean.clone()]))) } } } }) {
    return _choose(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("negative"))); if !py_truthy(&__b) { __b } else { _a_things_state(&[memory.clone(), clean.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback::g_A_THING_LOOKS_SAD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_A_BIT_PAST_IT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_thing_they_started(&[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]))) } } } } } }) {
    return _choose(&[if py_truthy(&crate::gen::compose::fallback::_names_their_age(&[clean.clone()])) { crate::gen::compose::fallback::g_NOT_TOO_OLD() } else { crate::gen::compose::fallback::g_NOT_FIT_ENOUGH() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::leaving::reads_as_a_leave_with_a_trouble_in_it(&[clean.clone(), words.clone()])) {
    bye = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); __kws.extend(vec![("trouble_blocks".to_string(), Py::Bool(false)), ("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::leaving::pool(args))(&__args) };
    if py_truthy(&bye.clone()) {
    return _choose(&[bye.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&crate::gen::compose::fallback::_a_cost_somebody_asks(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_A_COST_SOMEBODY_ASKS(), memory.clone(), clean.clone()]);
    }
    lost = crate::gen::compose::fallback::the_lost_ask_reply(&[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&lost.clone()) {
    return lost.clone();
    }
    if py_truthy(&mcall(&g_RESULT_WENT_WRONG(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_THING_CAME_OUT_WRONG(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&g_PLAN_THAT_FELL_THROUGH(), "search", &[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_PLAN_THAT_FELL_THROUGH(), memory.clone(), clean.clone()]);
    }
    state = if py_truthy(&{ let __b = mcall(&g_RESULT_WENT_WRONG(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mood.clone(), &py_str("positive"))) } }) { Py::None } else { _a_state_of_theirs(&[clean.clone()]) };
    if py_truthy(&state.clone()) {
    return _choose(&[if py_truthy(&Py::Bool(py_eq(&state.clone(), &py_str("then")))) { crate::gen::compose::fallback_data::g_A_STATE_PAST() } else { crate::gen::compose::fallback_data::g_A_STATE_NOW() }, memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_bad_time_told(&[clean.clone(), memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_BAD_TIME_TOLD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_day_asked_without_a_cheer(&[clean.clone(), memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_DAY_ASKED_PLAIN(), memory.clone(), clean.clone()]);
    }
    named = _a_name_told_reply(&[clean.clone(), memory.clone()]);
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    suggestion = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("suggestion")]) } else { Py::None };
    if py_truthy(&{ let __b = suggestion.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asking.clone())) } }) {
    who = call_builtin("str", &[{ let __b = mcall(&suggestion.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&who.clone()) {
    return _choose(&[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_A_SUGGESTED_BY_SOMEONE()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_A_SUGGESTED(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_thing_they_regret(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_A_THING_THEY_REGRET(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_betrayal(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_BETRAYAL(), memory.clone(), clean.clone()]);
    }
    care = _a_care_home_line(&[understood.clone(), memory.clone()]);
    if py_truthy(&care.clone()) {
    return care.clone();
    }
    wrecked = crate::gen::compose::finding_out::the_wrecked_form_reply(&[clean.clone()]);
    if py_truthy(&wrecked.clone()) {
    said = _choose(&[wrecked.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    sum_said = _a_sum_with_nothing_beside_it(&[clean.clone(), memory.clone()]);
    if py_truthy(&sum_said.clone()) {
    return _choose(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_SUM_STATED()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), sum_said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), clean.clone()]);
    }
    rest = _the_rest_of_the_day(&[clean.clone(), memory.clone(), mood.clone()]);
    if py_truthy(&Py::Bool(!py_is(&rest.clone(), &Py::None))) {
    return _choose(&[rest.clone(), memory.clone(), clean.clone()]);
    }
    reported = _a_report_from_elsewhere(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_is(&reported.clone(), &Py::None))) {
    return _choose(&[reported.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mood.clone(), &py_str("neutral"))); if !py_truthy(&__b) { __b } else { { let __b = _a_small_day(&[clean.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_just_asked_about_the_day(&[memory.clone()]))) } } } }) {
    return _choose(&[crate::gen::compose::fallback::g_A_SMALL_DAY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_A_SORRY_ON_ITS_OWN(), "match", &[mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])])) {
    return _choose(&[crate::gen::compose::fallback::g_SORRY_ON_ITS_OWN(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_A_DISMISSAL(), "match", &[mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])])) {
    return _choose(&[crate::gen::compose::fallback::g_NEVER_MIND(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_takes_something_back(&[clean.clone(), memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_A_CORRECTION_TAKEN(), memory.clone(), clean.clone()]);
    }
    positive = crate::gen::compose::fallback::_says_only_what_they_said(&[crate::gen::compose::fallback::g_POSITIVE(), clean.clone()]);
    if py_truthy(&Py::Bool(py_eq(&mood.clone(), &py_str("neutral")))) {
    one_thing = crate::gen::compose::onething::pool(&[clean.clone(), understood.clone(), memory.clone()]);
    if py_truthy(&one_thing.clone()) {
    return _choose(&[one_thing.clone(), memory.clone(), clean.clone()]);
    }
    let mut limit_line: Py = crate::gen::compose::limit::module_obj();
    narrowed = mcall(&limit_line.clone(), "a_limit_the_reading_missed", &[clean.clone(), memory.clone()]);
    if py_truthy(&narrowed.clone()) {
    return narrowed.clone();
    }
    }
    if py_truthy(&reads_as_no_pleasure(&[clean.clone()])) {
    return _choose(&[g_PLEASURE_GONE_LINES(), memory.clone(), clean.clone()]);
    }
    return _choose(&[geti(&py_dict(vec![(py_str("positive"), positive.clone()), (py_str("negative"), crate::gen::compose::fallback::g_NEGATIVE()), (py_str("neutral"), crate::gen::compose::fallback::g_NEUTRAL())]), &mood.clone()), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _a_betrayal(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut wrong: Py = Py::None;
    let _ = py_str("True when the line reports that somebody close lied, cheated or hid something from them.\n\n    \"i found out hes been lying\" and \"turns out hes been texting his ex\" are the same news, and so\n    is \"my husband has been lying to me\", which says nothing about finding out: the reading is the\n    wrong done by a person of theirs, and everything else here - the day pool, the cheer, the\n    receipt that says it back - is a reply to another kind of line.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(22)) } }) {
    return Py::Bool(false);
    }
    wrong = mcall(&g_A_WRONG_PLAIN(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wrong.clone()))) {
    wrong = mcall(&g_A_WRONG_SAID(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wrong.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_WRONG_AIMED(), "search", &[text.clone()])))) {
    if py_truthy(&mcall(&g_A_WRONG_ABOUT_A_THING(), "match", &[slice_or_raise(&text.clone(), &mcall(&wrong.clone(), "end", &[]), &Py::None, &Py::None)])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_FOUND_IT_OUT(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_WHO_DID_IT(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&wrong.clone(), "start", &[]), &Py::None)]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_FOUND_OUT(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&wrong.clone(), "start", &[]), &Py::None)]))) } }) {
    return Py::Bool(false);
    }
    after = slice_or_raise(&text.clone(), &mcall(&wrong.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:to|with|from|on|at)\\s+(?!me\\b|us\\b)(?:his|her|their|my|the|a|an|some)\\b"), after.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_HARM_ON_ME(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WHO_DID_IT(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_thing_they_regret(args: &[Py]) -> Py {
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
    let mut job: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line tells the bot about something the person did and is sorry for.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_REGRET_DOING(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_REGRET(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SNAPPED_A_PHOTO(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    job = if py_truthy(&mcall(&g_A_MISTAKE(), "search", &[text.clone()])) { mcall(&g_THE_JOB_NAMED(), "search", &[text.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = job.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_JOB_BUT_A_PERSON(), &mcall(&job.clone(), "group", &[Py::Int(1)]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_HARM_NAMED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_HARM_ON_ME(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_care_home_line(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut care: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut several: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The reply to somebody living in a care home, or None.");
    care = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("care_home")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[care.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    pronoun = call_builtin("str", &[{ let __b = mcall(&care.clone(), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { py_str("they") } }]).unwrap_or_else(|e| exc::raise(e));
    several = Py::Bool(py_eq(&pronoun.clone(), &py_str("they")));
    words = py_dict(vec![(py_str("they"), pronoun.clone()), (py_str("them"), py_str("them")), (py_str("their"), py_str("their")), (py_str("is"), if py_truthy(&several.clone()) { py_str("are") } else { py_str("is") })]);
    shape = call_builtin("str", &[{ let __b = mcall(&care.clone(), "get", &[py_str("shape")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("bring")))) {
    pool = crate::gen::compose::fallback_data::g_A_CARE_HOME_BRING();
    } else {
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("guilt")))) {
    pool = crate::gen::compose::fallback_data::g_A_CARE_HOME_GUILT();
    } else {
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("visiting")))) {
    pool = crate::gen::compose::fallback_data::g_A_CARE_HOME_VISITING();
    } else {
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("moved")))) {
    pool = crate::gen::compose::fallback_data::g_A_CARE_HOME_MOVED();
    } else {
    pool = crate::gen::compose::fallback_data::g_A_CARE_HOME();
    }
    }
    }
    }
    return _choose(&[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); for (k, v) in kw_pairs(&words.clone()) { __kws.push((k, v)); } if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), call_builtin("str", &[{ let __b = mcall(&care.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _a_day_told_instead_of_the_receipt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said_back: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_back") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_back"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut care: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut wrecked: Py = Py::None;
    let _ = py_str("The question about the day, where the echo is only a receipt of it.\n\n    \"i saw my sister at the weekend\" is the store filing the visit, and the filing said back -\n    \"Noted: you saw your sister at the weekend.\" - is the coldest reply in this file and says\n    nothing about the visit. A pool's own line (\"Your sister is called Meg - noted.\") is not a\n    receipt and keeps its turn, and neither does a hire, a win or a thing bought reach here.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said_back.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::ack::reads_as_a_receipt(&[said_back.clone()]))) } }) {
    return Py::None;
    }
    named = _a_name_told_reply(&[clean.clone(), memory.clone()]);
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    if py_truthy(&{ let __b = _a_small_day_shape(&[clean.clone()]); if !py_truthy(&__b) { __b } else { _a_detail_of_a_bad_day(&[memory.clone(), clean.clone()]) } }) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_BAD_TIME_TOLD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_thing_they_regret(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback::g_A_THING_THEY_REGRET(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_betrayal(&[clean.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_BETRAYAL(), memory.clone(), clean.clone()]);
    }
    care = _a_care_home_line(&[mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]), memory.clone()]);
    if py_truthy(&care.clone()) {
    return care.clone();
    }
    if py_truthy(&mcall(&g_HAVE_GOT_A_THING(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::None;
    }
    if py_truthy(&_just_asked_about_the_day(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&_a_bad_time_told(&[clean.clone(), memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_BAD_TIME_TOLD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_a_day_asked_without_a_cheer(&[clean.clone(), memory.clone()])) {
    return _choose(&[crate::gen::compose::fallback_data::g_A_DAY_ASKED_PLAIN(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_small_day(&[clean.clone(), memory.clone()])))) {
    return Py::None;
    }
    wrecked = crate::gen::compose::finding_out::the_wrecked_form_reply(&[clean.clone()]);
    if py_truthy(&wrecked.clone()) {
    said = _choose(&[wrecked.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    rest = _the_rest_of_the_day(&[clean.clone(), memory.clone()]);
    if py_truthy(&rest.clone()) {
    return _choose(&[rest.clone(), memory.clone(), clean.clone()]);
    }
    return _choose(&[crate::gen::compose::fallback::g_A_SMALL_DAY(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _the_crowd_is_ahead(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True for a line that measures the person against other people.\n\n    A short line, no question in it, a crowd in it, a milestone in it, and the frame that makes\n    the milestone a comparison (see CROWD_ORDER). The exclusions are the crowds that are not a\n    measuring stick: one doing something to the person, one cheering their news, and one that is\n    a plan for the evening.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(16)) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_CROWD(), "search", &[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_MILESTONE(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_CROWD_ORDER(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_NOT_A_CROWD_AHEAD(), "search", &[text.clone()])));
    Py::None
}
pub fn _the_rest_of_the_day(args: &[Py]) -> Py {
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
    let mut mood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mood") { v.clone() }
    else { py_str("neutral") };
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("The pool for the rest of a day the turn before was about, or None.\n\n    The line has to be an account of the day on its own, which is the same reading the flat pool\n    takes: a subject with something asked about it, a plan or a question has had its own reply\n    higher up and never reaches here. Then the turn before decides which day it is part of - a\n    good one, so the reply adds to it instead of asking for its best bit, or a hard one, so the\n    reply carries it instead of congratulating the person on it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mood.clone(), &py_str("negative"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_small_day(&[clean.clone(), memory.clone()])))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    was = crate::gen::compose::fallback::_matchable(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&was.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_a_good_time(&[was.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_in_a_pool(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![crate::gen::compose::fallback::g_A_GOOD_TIME(), crate::gen::compose::fallback::g_A_GOOD_TIME_IN_A_CARE_THREAD()])]) } }) {
    return crate::gen::compose::fallback_data::g_A_REST_OF_A_GOOD_DAY();
    }
    if py_truthy(&crate::gen::compose::fallback::_in_a_pool(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![crate::gen::compose::fallback::g_LOW_DAY_REPLIES()])])) {
    return crate::gen::compose::fallback_data::g_A_REST_OF_A_HARD_DAY();
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_HARD_DAY_MOOD(), &_an_emotion(&[was.clone()])))) {
    return crate::gen::compose::fallback_data::g_A_REST_OF_A_HARD_DAY();
    }
    return Py::None;
    Py::None
}
pub fn _an_emotion(args: &[Py]) -> Py {
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
    let _ = py_str("The emotion the understanding engine reads in a line, or None.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = crate::gen::understanding::kinds::emotion_of(&[clean.clone()]); if py_truthy(&__b) { __b } else { Py::None } }]);
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
    Py::None
}
pub fn _just_asked_about_the_day(args: &[Py]) -> Py {
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
    let _ = py_str("True when the bot's own last line was already the question about the day.\n\n    \"we had a takeaway last night\" / \"it was nice not cooking\" was asked how it went twice, which\n    reads as not listening to the answer. The bot's last line is the test, so the second turn is\n    left to the rest of the chain.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return crate::gen::compose::fallback::_in_a_pool(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![crate::gen::compose::fallback::g_A_SMALL_DAY()])]);
    Py::None
}
pub fn _a_day_of_someone_else(args: &[Py]) -> Py {
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
    let mut mine: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("True when the day belongs to somebody else, or to a childhood that is not this day.\n\n    One predication is enough: whoever comes first in the line is the one the day is about, so\n    \"my husband cornered me\" is his doing and \"i told my mum\" is hers, and a line that names no\n    speaker at all (\"my marriage ended\", \"someone drove into the back of my car\") is not an\n    account of the person's own day.\n    ");
    if py_truthy(&mcall(&g_A_CHILDHOOD_MEMORY(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    who = mcall(&g_SOMEONE_ELSE_NAMED(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_VISIT(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    mine = mcall(&g_THEIR_OWN_DOING(), "search", &[text.clone()]);
    return { let __b = Py::Bool(py_is(&mine.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&mcall(&who.clone(), "start", &[]), &mcall(&mine.clone(), "start", &[])) } };
    Py::None
}
pub fn _a_thing_happened(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line holds a thing or a time, not only a verb and a modifier.");
    doing = mcall(&g_PAST_DOING(), "search", &[text.clone()]);
    rest = if py_truthy(&Py::Bool(py_is(&doing.clone(), &Py::None))) { text.clone() } else { add(&add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&doing.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&text.clone(), &mcall(&doing.clone(), "end", &[]), &Py::None, &Py::None)) };
    return call_builtin("bool", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&rest.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_BARE_MODIFIER(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_thing_got_ahead(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The pool for a place secured in something still to come, or None.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_THING_GOT_AHEAD(), "search", &[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_GOT_AHEAD_GONE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_GOT_AHEAD_NOT_A_DATE(), "search", &[text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_MONTH(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_DATE_AHEAD(), "search", &[text.clone()]) } }) {
    return crate::gen::compose::fallback_data::g_A_BOOKING_AHEAD_DATED();
    }
    return crate::gen::compose::fallback_data::g_A_BOOKING_AHEAD();
    Py::None
}
pub fn _a_problem_stated(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { Py::None };
    let mut bare: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("True when the line is a statement of a problem and nothing else.\n\n    Both the line as typed and the reading's repaired text are tried, for the same reason\n    `_asks_for_help` tries both: the reader puts the words back (\"i hav a problem\" -> \"i have a\n    problem\", \"ive got a problm\" -> \"i've got a problem\"), and the patterns are written for\n    ordinary words.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    bare = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PROBLEM_STATED(), "match", &[bare.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SOMETHING_UP(), "match", &[bare.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_there_is_nothing_to_wear(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { Py::None };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { Py::None };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line says there is nothing to put on, with no occasion named in the tables.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone(), text.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_NOTHING_TO_WEAR(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_wear_ask_about_a_day(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { Py::None };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { Py::None };
    let mut line: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks what to put on and has a day to put it on for.\n\n    The ask has to be part of a longer line, because the ask on its own is `_bare_pick`'s and\n    already has its pool; this is the line that says the occasion first and asks after.\n\n    All three readings are tried: `clean` is the line with the filler taken out and the ask read\n    off it, so \"im going to a do at the golf club and i have no idea what to wear\" arrives here\n    as the occasion alone and the ask in `understood[\"ask\"]`, and only `text` still has both.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone(), text.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WEAR_ASK(), "search", &[said.clone()]))) } }) {
    continue;
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WEAR_ASK(), "sub", args))(&__args) };
    if py_truthy(&stdlib_call("re", "findall", &[py_str("[a-z']{2,}"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_wear_pool_for_the_line(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { Py::None };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { Py::None };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The wear pool that fits the weather the line itself named, or the general one.");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone(), text.clone()])) { let mut line: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])); } py_list(__out) })() }]);
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_WEATHER_WET(), "search", &[said.clone()])) {
    return crate::gen::compose::fallback_data::g_BARE_PICK_WEAR_WET();
    }
    if py_truthy(&{ let __b = mcall(&g_COLD_FIGURE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_WEATHER_COLD(), "search", &[said.clone()]) } }) {
    return crate::gen::compose::fallback_data::g_BARE_PICK_WEAR_COLD();
    }
    if py_truthy(&{ let __b = mcall(&g_HOT_FIGURE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fallback::g_WEATHER_HOT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WEATHER_SUN(), "search", &[said.clone()]) } } } }) {
    return crate::gen::compose::fallback_data::g_BARE_PICK_WEAR_HOT();
    }
    return g_WEAR_WHEN_NOTHING_ANSWERS();
    Py::None
}
pub fn _a_small_day(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True for a short, flat account of a day of the person's own that went neither way.\n\n    The shape first, then what the line feels like: a day that went well has its own warm pool, and\n    a day that went badly is met rather than congratulated. This can only replace the intake form,\n    because everything with a reading of its own has already had its turn above.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_a_small_day_shape(&[clean.clone()])))) {
    return Py::Bool(false);
    }
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    read = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&read.clone(), "get", &[py_str("suggestion")]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_good_time(&[clean.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_detail_about_what_was_said(&[text.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_LOOKING_BACK(), "match", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("polarity")]), &neg(&Py::Int(1))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_thing_got_ahead(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::remark::g_THEIR_NAME_AGE(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::remark::g_THEIR_NAME(), "match", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_day_of_someone_else(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_problem_stated(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DAY_PAST(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_thing_happened(&[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_DAY_SUBJECT(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_PAST_DOING(), "search", &[text.clone()]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DAY_PAST(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DAY_DONE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_DAY_SAID(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_thing_they_regret(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_SYMPTOM_PHRASE(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_NEGATIVE_LINE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fallback::g_HARM_NAMED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_A_HURT_MYSELF(), "search", &[text.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_DAY_WENT_WRONG(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::remark::done_to_me(&[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_mishap_to_ask_about(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_SOMEONE_DONE_WRONG(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_BAD_THING_NAMED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_HAND_OUT(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _the_day_was_already_bad(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DAY_PAST(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::onething::g_A_MEDS(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::onething::g_A_MEDS_CHANGE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::onething::g_A_MEDS_MINE(), "match", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::onething::g_A_MEDS_SYMPTOM(), "search", &[text.clone()]))) } } } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_BAD_DAY_ANYWAY(), "search", &[text.clone()])));
    Py::None
}
pub fn _a_detail_about_what_was_said(args: &[Py]) -> Py {
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
    let mut last_bot: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True for a clause that only adds to the line before it: \"it was yesterday\".\n\n    The pleasantry this stands down is the answer to a question about a day, so it is kept when\n    the bot has just asked one: \"how was it, overall?\" then \"it was ok\" is the answer, not a\n    detail. What makes the line a detail is that the answer before it was advice or sympathy -\n    the bot was talking about the ankle, not asking about the day.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_DETAIL_OPENING(), "match", &[text.clone()])))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    last_bot = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&crate::gen::compose::fallback::_in_a_pool(&[last_bot.clone(), py_tuple(vec![crate::gen::compose::fallback::g_A_SMALL_DAY()])])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_SYMPATHY(), "search", &[mcall(&last_bot.clone(), "lower", &[])])) {
    return Py::Bool(true);
    }
    before = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![crate::gen::compose::fallback::g_HARM_ANYWHERE(), crate::gen::compose::fallback::g_NEGATIVE_LINE(), g_BAD_DAY_ANYWAY(), g_A_DAY_WENT_WRONG(), g__A_BODY_UNWELL()])) { let mut reading: Py = __item.clone(); __out.push(mcall(&reading.clone(), "search", &[before.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_dish_name(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut topics: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Is this line the name of a dish rather than a report of something done?\n\n    Asked of the understanding engine's own reading, not of a word list: a line it read as\n    cooking or food-and-drink is a subject the knowledge base answers, and \"packed lunch for a\n    vegan\" is a question about lunch however it opens.\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::compose::fallback::g_DISH_PARTICIPLE(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_DISH_DETERMINER(), &geti(&words.clone(), &Py::Int(1))))) {
    return Py::Bool(false);
    }
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    topics = call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("isinstance", &[mcall(&understood.clone(), "get", &[py_str("topic")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&topics.clone(), "add", &[geti(&understood.clone(), &py_str("topic"))]);
    }
    return call_builtin("bool", &[bitand(&topics.clone(), &g_FOOD_TOPICS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_report_with_no_pronoun(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reply: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A thing done said without the \"i\": \"booked the hotel last night\", \"paid the deposit\".\n\n    The dropped pronoun is what makes this the composer's line rather than the knowledge base's.\n    A card cannot tell \"booked the hotel last night\" from \"how do i book a hotel\" - both hold the\n    word \"hotel\" and the word \"book\" - but a past tense in the first word is a report, and the\n    whole line is then a thing done with nothing to advise. A line with its subject (\"i booked the\n    hotel last night\") is not this: the answers module can still tell it from the how-to, and the\n    composer only takes what nothing else can read.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::compose::fallback::g_DONE_VERB(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_HARM_ANYWHERE(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::Int(3), &Py::None)) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::compose::fallback::g_DONE_NOT_THIS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_dish_name(&[clean.clone(), words.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_eq(&geti(&crate::gen::compose::fallback::_thing_done_scan(&[clean.clone()]), &Py::Int(0)), &py_str("done")))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    reply = reply_for(&[text.clone(), memory.clone()]);
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
    return Py::Bool(py_contains(&crate::gen::compose::fallback::g__THING_DONE_LINES(), &reply.clone()));
    Py::None
}
pub fn _repaired_of(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let _ = py_str("The line with its chat-speak put right, as the reading made it, or \"\" for none.\n\n    The reading is the arbiter's own (`memory[\"understood\"]`) when it is this turn's, so the\n    router asks for no second reading of the line; a reading that fails claims no repair.\n    ");
    let __try1 = exc::catch(|| {
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&read.clone(), "get", &[py_str("plain")]) } }) {
    return py_tuple(vec![Py::Bool(true), call_builtin("str", &[geti(&read.clone(), &py_str("plain"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    read = py_call(&crate::gen::compose::fallback::g_understand(), &[text.clone(), memory.clone()]);
    return py_tuple(vec![Py::Bool(true), call_builtin("str", &[{ let __b = mcall(&{ let __b = read.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
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
    return py_str("");
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = Py::None;
    let _ = py_str("Does the composer read this whole line, whatever any topic entry matching it says?");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __try1 = exc::catch(|| {
    if py_truthy(&crate::gen::compose::fallback::reads_as_a_report(&[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&crate::gen::compose::remark::regret_reply_for(&[clean.clone(), mcall(&clean.clone(), "split", &[]), memory.clone(), _repaired_of(&[text.clone(), memory.clone()])])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&_a_report_with_no_pronoun(&[clean.clone(), text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&crate::gen::compose::why::takes_as_it_comes(&[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_verdict_line(&[clean.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&crate::gen::compose::safetyask::reply_for(&[clean.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_truthy(&crate::gen::compose::fallback::_after_safety(&[memory.clone(), clean.clone(), mcall(&memory.clone(), "get", &[py_str("understood")])])))]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_verdict_on_their_plan(&[clean.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_place_asked_about(&[clean.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&crate::gen::compose::fallback::reads_as_if_i_saw(&[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?"))); if !py_truthy(&__b) { __b } else { crate::gen::compose::trouble::answers_a_strain_question(&[clean.clone(), memory.clone()]) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::compose::fallback::_plain(&[clean.clone()])); __kws.extend(vec![("without_a_thing".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::fallback::_a_declined_plan(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let __b = crate::gen::compose::fallback::_a_hard_week(&[memory.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_the_turn_before_was_a_heavy_one(&[memory.clone(), crate::gen::compose::fallback::_plain(&[clean.clone()])]))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = crate::gen::compose::fallback::_a_plan_on_the_table(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_the_line_cancels(&[crate::gen::compose::fallback::_plain(&[clean.clone()])]))) } })) } } } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_truthy(&crate::gen::compose::fallback::_after_safety(&[memory.clone(), clean.clone(), mcall(&memory.clone(), "get", &[py_str("understood")])])))]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::handoff::takes_as_it_comes(&[text.clone(), memory.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_truthy(&crate::gen::compose::fallback::_after_safety(&[memory.clone(), clean.clone(), mcall(&memory.clone(), "get", &[py_str("understood")])])))]);
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
pub fn _a_name_told(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut names: Py = Py::None;
    let mut person: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The names and the person, when the line is only a name: (\"Jack and Emma\", \"kids\", True).\n\n    The third part is whether the names are more than one, because \"Jack and Emma\" is they and\n    \"June\" is she or he: the pronoun comes from the person word the same way the name reader\n    does it, so nothing here guesses.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    match_ = mcall(&g_A_NAMES_TOLD(), "search", &[said.clone()]);
    person = if py_truthy(&match_.clone()) { mcall(&match_.clone(), "group", &[py_str("person")]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = mcall(&g_A_NAME_A_HOUSEHOLD_TAKES(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    }
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s*(?:,|and)\\s*"), mcall(&match_.clone(), "group", &[py_str("names")])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[])); } py_list(__out) })() };
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { le(&call_builtin("set", &[mcall(&word.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_NAME_WORD()) } }) {
    break;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    names = kept.clone();
    if py_truthy(&Py::Bool(!py_truthy(&names.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    spoken = mcall(&geti(&names.clone(), &Py::Int(0)), "title", &[]);
    } else {
    spoken = add(&add(&mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&names.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "title", &[])); } py_list(__out) })() }]), &py_str(" and ")), &mcall(&geti(&names.clone(), &neg(&Py::Int(1))), "title", &[]));
    }
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&crate::gen::understanding::topics::g_THE_PEOPLE(), &person.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&person.clone(), &py_str("-"))) } } } }) {
    person = py_str("");
    }
    return py_tuple(vec![spoken.clone(), person.clone(), gt(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]);
    Py::None
}
pub fn _named_after_reply(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The reply to a name that came from somebody, or None.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = mcall(&g_NAMED_AFTER(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    who = mcall(&found.clone(), "group", &[py_str("who")]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("me"), py_str("us"), py_str("you"), py_str("i")]), &who.clone()))) {
    return { let __b = _choose(&[crate::gen::compose::fallback_data::g_A_NAMED_AFTER_THEM(), memory.clone(), clean.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    }
    words = mcall(&who.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_A_POSSESSIVE_OF_MINE(), &geti(&words.clone(), &Py::Int(0))))) {
    who = add(&py_str("your "), &geti(&words.clone(), &Py::Int(1)));
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback_data::g_A_NAMED_AFTER()) { let mut line: Py = __item.clone(); __out.push(crate::gen::compose::remark::fill_person(&[mcall(&line.clone(), "replace", &[py_str("{who}"), who.clone()]), geti(&words.clone(), &neg(&Py::Int(1)))])); } py_list(__out) })() };
    return { let __b = _choose(&[lines.clone(), memory.clone(), clean.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _a_name_told_reply(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut person: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut told: Py = Py::None;
    let _ = py_str("The reply to a line that is only a name, said back and asked about, or None.");
    told = _a_name_told(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&told.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&told.clone(), 3, None);
    spoken = __unpk1[0].clone();
    person = __unpk1[1].clone();
    plural = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    return { let __b = _choose(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback_data::g_A_NAME_STORY()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "replace", &[py_str("{names}"), spoken.clone()])); } py_list(__out) })() }, memory.clone(), clean.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback_data::g_A_NAME_TOLD()) { let mut line: Py = __item.clone(); __out.push(crate::gen::compose::remark::fill_person(&[mcall(&line.clone(), "replace", &[py_str("{names}"), spoken.clone()]), if py_truthy(&plural.clone()) { py_str("kids") } else { person.clone() }])); } py_list(__out) })() };
    return { let __b = _choose(&[lines.clone(), memory.clone(), clean.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _a_state_of_theirs(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let _ = py_str("\"now\", \"then\" or None: what is true of the person rather than what they did.\n\n    A line here has the person as its subject and a word of state in it (\"self employed\",\n    \"on a diet\", \"married\") and no past doing of its own, so there is no day to ask about.\n    The day marker is what makes a line an account again: \"work was ok today\" is a day had.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DAY_SUBJECT(), "search", &[text.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_STATE_NOT_A_DOING(), "search", &[text.clone()])))) {
    return Py::None;
    }
    rest = mcall(&g_BARE_STATE(), "sub", &[py_str(" "), mcall(&g_STATE_NOT_A_DOING(), "sub", &[py_str(" "), text.clone()])]);
    if py_truthy(&{ let __b = mcall(&g_DAY_PAST(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PAST_DOING(), "search", &[rest.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_STATE_THEN(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_STATE_IN_THE_PAST(), "search", &[text.clone()]) } }) {
    return py_str("then");
    }
    return py_str("now");
    Py::None
}
pub fn _a_day_said_badly(args: &[Py]) -> Py {
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
    let _ = py_str("True when the person's own line reads as a day that went badly.");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_DAY_SAID_BADLY(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_last_exchange(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last turn that has a reply on it, or None.\n\n    The turn being composed is already on the record with an empty bot line by the time this file\n    reads it, so `turns[-1]` is the line being answered and not the exchange before it. Every\n    reading of \"the line before this one\" in this file has to look for the last turn that has a\n    bot line, or it reads the wrong turn - which is how the bad-day guard came out false on the\n    very row it was written for (\"sorry, bad day\" then \"sorry, it was the meeting\").\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {
    return turn.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_day_was_already_bad(args: &[Py]) -> Py {
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
    let mut seen: Py = Py::None;
    let _ = py_str("True when the bot's own last line already met a bad day.\n\n    \"sorry, bad day\" / \"sorry, it was the meeting\": the second line names what went wrong in the\n    day the bot has just answered with sympathy, and the cheer that follows it (\"Good to hear.\n    What was the best bit?\") reads as not having heard the first line at all. The bot's own last\n    reply is the test, so a day that has already gone badly is only read as one while it is still\n    on the table; a chat the bot has moved on from is left to the rest of the chain.\n    ");
    seen = _the_last_exchange(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&seen.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    last = call_builtin("str", &[{ let __b = mcall(&seen.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&crate::gen::compose::fallback::_in_a_pool(&[last.clone(), py_tuple(vec![crate::gen::compose::fallback::g_LOW_DAY_REPLIES(), crate::gen::compose::fallback::g_NEGATIVE(), crate::gen::compose::fallback_data::g_A_BAD_TIME_TOLD(), crate::gen::compose::fallback_data::g_A_THING_CAME_OUT_WRONG(), crate::gen::compose::fallback_data::g_A_PLAN_THAT_FELL_THROUGH(), crate::gen::compose::fallback_data::g_A_DAY_ASKED_PLAIN()])])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_BOT_WAS_SORRY(), "search", &[slice_or_raise(&last.clone(), &Py::None, &Py::Int(80), &Py::None)])) {
    return Py::Bool(true);
    }
    return _a_day_said_badly(&[mcall(&seen.clone(), "get", &[py_str("user")])]);
    Py::None
}
pub fn _a_detail_of_a_bad_day(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut seen: Py = Py::None;
    let _ = py_str("True when this line is the detail of a day the person has already said went badly.\n\n    Narrower than `_the_day_was_already_bad` on purpose: the bot says something sympathetic after\n    a great many lines, and the cheer for a haircut after a boiler is a new day, not a detail of\n    the old one. What makes it a detail is the person's own line before it - a day read as one\n    gone badly - and the short shape of this one.\n    ");
    seen = _the_last_exchange(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&seen.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_DAY_PAST(), "search", &[mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]) } }) {
    return call_builtin("bool", &[_a_detail_about_what_was_said(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    return _a_day_said_badly(&[mcall(&seen.clone(), "get", &[py_str("user")])]);
    Py::None
}
pub fn _money_came_to_them(args: &[Py]) -> Py {
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
    let _ = py_str("True for a sum the person was given, paid, earned or refunded rather than asked to judge.");
    if py_truthy(&bitand(&words.clone(), &g_MONEY_IN_NOUNS())) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_MONEY_IN_SHAPES(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_sum_with_nothing_beside_it(args: &[Py]) -> Py {
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
    let mut _unit: Py = Py::None;
    let mut _value: Py = Py::None;
    let mut found: Py = Py::None;
    let mut marked: Py = Py::None;
    let mut money: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The figure, said back the way they wrote it, for a sum of money stated with no question.\n\n    Three ways for a line to say the figure is money: a currency mark, a price word (\"paid\", \"the\n    quote came to\", \"the fee\"), or a price verb with a thing in front of it (\"my new sofa was\n    800\"). The third is the one that needs care, because the same verb says a measurement: \"it was\n    60\" names nothing, so it is not a price and the reading stands down. An amount with none of the\n    three is somebody else's money - \"my nan left me 500\" is a bequest, and asking what they were\n    paying before would be the wrong question about it - so that stands down too.\n\n    The direction is fenced first: money that came to the person (\"my bonus was 500\", \"they paid me\n    200\", \"my pay went up by 200\") is not a price, and a friend answers a bonus or a refund as the\n    good news it is. Only money going out is a sum with nothing to judge it by.\n\n    The figure itself is read by the same code a doubt is read by (`reading._their_figure`), so a\n    year, an age and a distance are passed over here for the same reasons they are passed over\n    there. No range is looked for in the answer just given: everything with a card behind it has\n    already answered the turn above, so a sum that reaches this line is one the bot knows nothing\n    about.\n    ");
    let mut reading: Py = crate::gen::compose::reading::module_obj();
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_money_came_to_them(&[text.clone(), words.clone()])) {
    return Py::None;
    }
    marked = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("[\\u00a3$\\u20ac]"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    named = call_builtin("bool", &[sub(&words.clone(), &g_NO_THING_SAID())]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = marked.clone(); if py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_PRICE_NOUNS()); if py_truthy(&__b) { __b } else { { let __b = named.clone(); if !py_truthy(&__b) { __b } else { mcall(&gattr(&reading.clone(), "A_PRICE_VERB"), "search", &[text.clone()]) } } } } } }))) {
    return Py::None;
    }
    found = mcall(&reading.clone(), "_their_figure", &[py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("user"), text.clone())])]))])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 4, None);
    _value = __unpk1[0].clone();
    _unit = __unpk1[1].clone();
    said = __unpk1[2].clone();
    money = __unpk1[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&money.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::None;
    }
    return said.clone();
    Py::None
}
pub fn _a_report_from_elsewhere(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The pool for a thing the person has only been told about, or None.\n\n    Read where the day pool would answer, so a line with a reading of its own never reaches it.\n    The weather half and the everything-else half are different jobs - the local forecast and the\n    loose things outside, against the body that said it and the dates one line down - so they are\n    two pools and the weather words choose.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_ASKING_WORD(), "match", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_REPORTED_NOT_LIVED(), "search", &[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_NEWS_IS_WEATHER(), "search", &[text.clone()])) {
    return crate::gen::compose::fallback_data::g_A_NEWS_OF_WEATHER();
    }
    if py_truthy(&mcall(&g_A_NEWS_IS_A_CHANGE(), "search", &[text.clone()])) {
    return crate::gen::compose::fallback_data::g_A_NEWS_OF_SOMETHING_ELSE();
    }
    return Py::None;
    Py::None
}
pub fn _a_small_day_shape(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True for the shape of a short, flat account of a day of the person's own.\n\n    The shape only: a subject, a past doing, short enough to be one thing, and none of the things\n    that already have a reading above (a name, a state, a result that came out wrong, a plan that\n    fell through, a confession, a symptom, a mishap, a thing that went wrong). What the line\n    feels like is read by `_a_small_day`, so the same shape can be answered warmly when it went\n    well and met when it went badly.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_BEING_DONE_NOW(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_DECISION_NOT_A_DAY(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_WISH(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_AN_ATTEMPT_AT_SOMEONE(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_BETRAYAL_FOUND_OUT(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_day_of_someone_else(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DAY_PAST(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_thing_happened(&[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_DAY_SUBJECT(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_PAST_DOING(), "search", &[text.clone()]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DAY_PAST(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DAY_DONE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_DAY_SAID(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_thing_they_regret(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_SYMPTOM_PHRASE(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_NEGATIVE_LINE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_HARM_NAMED(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_DAY_WENT_WRONG(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&crate::gen::compose::fallback::_a_mishap_to_ask_about(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_BAD_THING_NAMED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_HAND_OUT(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_NAME_IS_NOT_A_DAY(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_is(&_a_report_from_elsewhere(&[text.clone()]), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_state_of_theirs(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_RESULT_WENT_WRONG(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLAN_THAT_FELL_THROUGH(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_bad_time_told(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The day pool's own shape, read as a day that went badly: \"we had a row last night\",\n    \"we had a strange day at work\", \"ngl ive bin knackered all week\", \"were gutted when it\n    happened\".\n\n    With the cheer taken away from a bad feeling these were left to the floor (\"I'm listening -\n    say more if it helps\"), which is the line a person gets when nothing in what they said was\n    heard - and the floor is right under this one, so a small pool here is the whole of the help\n    available: one question about the day they have just told you about. The test is the reading\n    the rest of the bot already makes, in either of the two ways it makes it: the mood of the\n    whole line, or the words this file already keeps for a day gone wrong.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_a_small_day_shape(&[clean.clone()])))) {
    return Py::Bool(false);
    }
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_NEGATIVE_LINE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fallback::g_HARM_NAMED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DAY_WENT_WRONG(), "search", &[text.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("polarity")]), &neg(&Py::Int(1))))) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_detail_of_a_bad_day(&[memory.clone(), text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_A_DOING_GONE_BADLY(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_THING_SKIPPED(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_MEAL_SKIPPED(), "search", &[text.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_A_DAY_SAID_BADLY(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_day_asked_without_a_cheer(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True for the day pool's shape where a word in the line says the day went wrong somewhere.\n\n    Two lists, two jobs. A_DAY_SAID_BADLY is narrow because its words choose the sympathetic pool\n    (`_a_bad_time_told`). BAD_DAY_ANYWAY is deliberately wide, because it only ever took the cheer\n    away and left the line to whatever answers next - and on today's main what answers next is the\n    floor (\"I'm listening - say more if it helps\"), which is worse than the cheer it replaced:\n    \"we had a strange day at work\" went from \"Good to hear. What was the best bit?\" to the floor.\n    So the wide list gets a question with no presumption in it either way, which is true of a\n    strange day at work and of a fish that got gutted (\"gutted\" is in the list for the row it\n    usually appears in).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_a_small_day_shape(&[clean.clone()])))) {
    return Py::Bool(false);
    }
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_NEGATIVE_LINE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fallback::g_HARM_NAMED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DAY_WENT_WRONG(), "search", &[text.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_bad_time_told(&[clean.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_just_asked_about_the_day(&[memory.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_BAD_DAY_ANYWAY(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.fallback_chain".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_BOT" => Some(g_ABOUT_THE_BOT()),
            "ABOUT_THE_BOT_LINES" => Some(g_ABOUT_THE_BOT_LINES()),
            "ACTS_AN_ERRAND" => Some(g_ACTS_AN_ERRAND()),
            "ACTS_AN_ERRAND_LINES" => Some(g_ACTS_AN_ERRAND_LINES()),
            "ACTS_IN_A_DIARY" => Some(g_ACTS_IN_A_DIARY()),
            "ACTS_IN_A_DIARY_LINES" => Some(g_ACTS_IN_A_DIARY_LINES()),
            "ACTS_ON_A_CLOCK_LINES" => Some(g_ACTS_ON_A_CLOCK_LINES()),
            "ACTS_ON_A_DOCUMENT_LINES" => Some(g_ACTS_ON_A_DOCUMENT_LINES()),
            "ACTS_ON_A_PHONE" => Some(g_ACTS_ON_A_PHONE()),
            "ACTS_ON_A_PHONE_LINES" => Some(g_ACTS_ON_A_PHONE_LINES()),
            "ACT_CHOICE" => Some(g_ACT_CHOICE()),
            "ACT_FOR_ME" => Some(g_ACT_FOR_ME()),
            "ACT_OPENER" => Some(g_ACT_OPENER()),
            "ACT_OWN_STEP" => Some(g_ACT_OWN_STEP()),
            "ACT_REPORT_TAIL" => Some(g_ACT_REPORT_TAIL()),
            "ACT_SETTING" => Some(g_ACT_SETTING()),
            "ACT_THING" => Some(g_ACT_THING()),
            "ADDRESSED" => Some(g_ADDRESSED()),
            "AN_ATTEMPT_AT_SOMEONE" => Some(g_AN_ATTEMPT_AT_SOMEONE()),
            "ASKS_AFTER_THE_OPENER" => Some(g_ASKS_AFTER_THE_OPENER()),
            "ASK_SHAPE" => Some(g_ASK_SHAPE()),
            "AT_THE_CLOCK" => Some(g_AT_THE_CLOCK()),
            "A_ASKING_WORD" => Some(g_A_ASKING_WORD()),
            "A_BAD_THING_NAMED" => Some(g_A_BAD_THING_NAMED()),
            "A_BARE_MODIFIER" => Some(g_A_BARE_MODIFIER()),
            "A_BEING_DONE_NOW" => Some(g_A_BEING_DONE_NOW()),
            "A_BETRAYAL_FOUND_OUT" => Some(g_A_BETRAYAL_FOUND_OUT()),
            "A_CHILDHOOD_MEMORY" => Some(g_A_CHILDHOOD_MEMORY()),
            "A_CLOCK" => Some(g_A_CLOCK()),
            "A_COMPARISON" => Some(g_A_COMPARISON()),
            "A_CROWD" => Some(g_A_CROWD()),
            "A_CROWD_AHEAD" => Some(g_A_CROWD_AHEAD()),
            "A_DAY_SAID_BADLY" => Some(g_A_DAY_SAID_BADLY()),
            "A_DAY_SUBJECT" => Some(g_A_DAY_SUBJECT()),
            "A_DAY_WENT_WRONG" => Some(g_A_DAY_WENT_WRONG()),
            "A_DECISION_NOT_A_DAY" => Some(g_A_DECISION_NOT_A_DAY()),
            "A_DOING_GONE_BADLY" => Some(g_A_DOING_GONE_BADLY()),
            "A_FEELING_IN_THE_LINE" => Some(g_A_FEELING_IN_THE_LINE()),
            "A_HAND_OUT" => Some(g_A_HAND_OUT()),
            "A_HARD_DAY_MOOD" => Some(g_A_HARD_DAY_MOOD()),
            "A_LENGTH" => Some(g_A_LENGTH()),
            "A_LIST_ON_THE_TABLE" => Some(g_A_LIST_ON_THE_TABLE()),
            "A_LOOKING_BACK" => Some(g_A_LOOKING_BACK()),
            "A_MEAL_SKIPPED" => Some(g_A_MEAL_SKIPPED()),
            "A_MESSAGE_OR_A_FORM" => Some(g_A_MESSAGE_OR_A_FORM()),
            "A_MILESTONE" => Some(g_A_MILESTONE()),
            "A_MISTAKE" => Some(g_A_MISTAKE()),
            "A_MONTH" => Some(g_A_MONTH()),
            "A_MOOD" => Some(g_A_MOOD()),
            "A_NAMES_TOLD" => Some(g_A_NAMES_TOLD()),
            "A_NAME_A_HOUSEHOLD_TAKES" => Some(g_A_NAME_A_HOUSEHOLD_TAKES()),
            "A_NAME_IS_NOT_A_DAY" => Some(g_A_NAME_IS_NOT_A_DAY()),
            "A_NEWS_IS_A_CHANGE" => Some(g_A_NEWS_IS_A_CHANGE()),
            "A_NEWS_IS_WEATHER" => Some(g_A_NEWS_IS_WEATHER()),
            "A_POSSESSIVE_OF_MINE" => Some(g_A_POSSESSIVE_OF_MINE()),
            "A_REGRET_DOING" => Some(g_A_REGRET_DOING()),
            "A_REPORTED_NOT_LIVED" => Some(g_A_REPORTED_NOT_LIVED()),
            "A_SUM_STATED" => Some(g_A_SUM_STATED()),
            "A_SYMPTOM_PHRASE" => Some(g_A_SYMPTOM_PHRASE()),
            "A_THING_GOT_AHEAD" => Some(g_A_THING_GOT_AHEAD()),
            "A_THING_SKIPPED" => Some(g_A_THING_SKIPPED()),
            "A_TIME_SAID" => Some(g_A_TIME_SAID()),
            "A_VISIT" => Some(g_A_VISIT()),
            "A_WISH" => Some(g_A_WISH()),
            "A_WRONG_ABOUT_A_THING" => Some(g_A_WRONG_ABOUT_A_THING()),
            "A_WRONG_AIMED" => Some(g_A_WRONG_AIMED()),
            "A_WRONG_PLAIN" => Some(g_A_WRONG_PLAIN()),
            "A_WRONG_SAID" => Some(g_A_WRONG_SAID()),
            "BAD_DAY_ANYWAY" => Some(g_BAD_DAY_ANYWAY()),
            "BARE_STATE" => Some(g_BARE_STATE()),
            "BOOKING_THING" => Some(g_BOOKING_THING()),
            "BOT_WAS_SORRY" => Some(g_BOT_WAS_SORRY()),
            "CAN_DO_ITSELF" => Some(g_CAN_DO_ITSELF()),
            "CLOCK_ABOUT_THE_JOB" => Some(g_CLOCK_ABOUT_THE_JOB()),
            "CLOCK_ASKED" => Some(g_CLOCK_ASKED()),
            "CLOCK_NOT_THIS" => Some(g_CLOCK_NOT_THIS()),
            "CLOCK_SHAPE" => Some(g_CLOCK_SHAPE()),
            "CLOCK_SOMEONE_ELSE" => Some(g_CLOCK_SOMEONE_ELSE()),
            "CLOCK_THE_DEPARTURE" => Some(g_CLOCK_THE_DEPARTURE()),
            "COLD_FIGURE" => Some(g_COLD_FIGURE()),
            "COUNTDOWN_AFTER_CARE" => Some(g_COUNTDOWN_AFTER_CARE()),
            "COUNTDOWN_ASK" => Some(g_COUNTDOWN_ASK()),
            "COUNTDOWN_TO" => Some(g_COUNTDOWN_TO()),
            "COUNTED" => Some(g_COUNTED()),
            "CROWD_ORDER" => Some(g_CROWD_ORDER()),
            "DAY_PAST" => Some(g_DAY_PAST()),
            "FEELING_NOTHING" => Some(g_FEELING_NOTHING()),
            "FEEL_COPULA" => Some(g_FEEL_COPULA()),
            "FOOD_TOPICS" => Some(g_FOOD_TOPICS()),
            "FOUND_IT_OUT" => Some(g_FOUND_IT_OUT()),
            "FOUND_OUT" => Some(g_FOUND_OUT()),
            "FRAME_PREFIX" => Some(g_FRAME_PREFIX()),
            "GOT_AHEAD_GONE" => Some(g_GOT_AHEAD_GONE()),
            "GOT_AHEAD_NOT_A_DATE" => Some(g_GOT_AHEAD_NOT_A_DATE()),
            "HABIT_MIND_IT" => Some(g_HABIT_MIND_IT()),
            "HABIT_NOT" => Some(g_HABIT_NOT()),
            "HABIT_STATED" => Some(g_HABIT_STATED()),
            "HANDS_IT_OVER_HERE" => Some(g_HANDS_IT_OVER_HERE()),
            "HAVE_GOT_A_THING" => Some(g_HAVE_GOT_A_THING()),
            "HOT_FIGURE" => Some(g_HOT_FIGURE()),
            "HURT_BUT_FINE" => Some(g_HURT_BUT_FINE()),
            "HURT_DEATH" => Some(g_HURT_DEATH()),
            "HURT_FIGURE" => Some(g_HURT_FIGURE()),
            "HURT_ON_THE_SPEAKER" => Some(g_HURT_ON_THE_SPEAKER()),
            "HURT_SOMEBODY" => Some(g_HURT_SOMEBODY()),
            "KEEP_ME_POSTED" => Some(g_KEEP_ME_POSTED()),
            "LOVES_A_GROUP" => Some(g_LOVES_A_GROUP()),
            "LOVES_PEOPLE" => Some(g_LOVES_PEOPLE()),
            "LOVES_SOMEONE" => Some(g_LOVES_SOMEONE()),
            "LOVE_GROUP" => Some(g_LOVE_GROUP()),
            "LOVE_ONE" => Some(g_LOVE_ONE()),
            "LOVE_OPENER" => Some(g_LOVE_OPENER()),
            "LOVE_PERSON" => Some(g_LOVE_PERSON()),
            "LOVE_START" => Some(g_LOVE_START()),
            "LOVE_TAIL" => Some(g_LOVE_TAIL()),
            "LOW_DAY" => Some(g_LOW_DAY()),
            "LOW_DAY_MINE" => Some(g_LOW_DAY_MINE()),
            "LOW_DAY_NOT_MINE" => Some(g_LOW_DAY_NOT_MINE()),
            "MONEY_IN_NOUNS" => Some(g_MONEY_IN_NOUNS()),
            "MONEY_IN_SHAPES" => Some(g_MONEY_IN_SHAPES()),
            "NAMED_AFTER" => Some(g_NAMED_AFTER()),
            "NEAR_MISS" => Some(g_NEAR_MISS()),
            "NEAR_MISS_REPLY" => Some(g_NEAR_MISS_REPLY()),
            "NOTHING_AND_A_PLEASURE" => Some(g_NOTHING_AND_A_PLEASURE()),
            "NOTHING_NEGATED" => Some(g_NOTHING_NEGATED()),
            "NOTHING_TO_WEAR" => Some(g_NOTHING_TO_WEAR()),
            "NOTHING_TO_WEAR_REPLIES" => Some(g_NOTHING_TO_WEAR_REPLIES()),
            "NOT_ABOUT_THEM" => Some(g_NOT_ABOUT_THEM()),
            "NOT_A_CROWD_AHEAD" => Some(g_NOT_A_CROWD_AHEAD()),
            "NOT_A_DAY_DONE" => Some(g_NOT_A_DAY_DONE()),
            "NOT_A_DAY_SAID" => Some(g_NOT_A_DAY_SAID()),
            "NOT_A_JOB_BUT_A_PERSON" => Some(g_NOT_A_JOB_BUT_A_PERSON()),
            "NOT_A_NAME_WORD" => Some(g_NOT_A_NAME_WORD()),
            "NOT_A_REGRET" => Some(g_NOT_A_REGRET()),
            "NOT_A_THING_TO_GET" => Some(g_NOT_A_THING_TO_GET()),
            "NO_THING_SAID" => Some(g_NO_THING_SAID()),
            "OPTION_ON_THE_TABLE" => Some(g_OPTION_ON_THE_TABLE()),
            "ORDER_A_BUY" => Some(g_ORDER_A_BUY()),
            "ORDER_A_CLOCK" => Some(g_ORDER_A_CLOCK()),
            "ORDER_A_DOCUMENT" => Some(g_ORDER_A_DOCUMENT()),
            "ORDER_A_MESSAGE" => Some(g_ORDER_A_MESSAGE()),
            "ORDER_A_RECORD" => Some(g_ORDER_A_RECORD()),
            "ORDER_A_REPORT" => Some(g_ORDER_A_REPORT()),
            "ORDER_NOT_A_JOB" => Some(g_ORDER_NOT_A_JOB()),
            "ORDER_OPENER" => Some(g_ORDER_OPENER()),
            "OWN_THING" => Some(g_OWN_THING()),
            "PAST_DOING" => Some(g_PAST_DOING()),
            "PLAN_THAT_FELL_THROUGH" => Some(g_PLAN_THAT_FELL_THROUGH()),
            "PLEASURE_ABOUT_ONE_THING" => Some(g_PLEASURE_ABOUT_ONE_THING()),
            "PLEASURE_GONE_LINES" => Some(g_PLEASURE_GONE_LINES()),
            "PLEASURE_GONE_OUT" => Some(g_PLEASURE_GONE_OUT()),
            "PLEASURE_WORD" => Some(g_PLEASURE_WORD()),
            "POLITE_ASK" => Some(g_POLITE_ASK()),
            "PRICE_NOUNS" => Some(g_PRICE_NOUNS()),
            "PROBLEM_STATED" => Some(g_PROBLEM_STATED()),
            "REMIND_ME" => Some(g_REMIND_ME()),
            "RESULT_WENT_WRONG" => Some(g_RESULT_WENT_WRONG()),
            "SAYS_A_FEELING" => Some(g_SAYS_A_FEELING()),
            "SAYS_A_THING" => Some(g_SAYS_A_THING()),
            "SNAPPED_A_PHOTO" => Some(g_SNAPPED_A_PHOTO()),
            "SOMEONE_DONE_WRONG" => Some(g_SOMEONE_DONE_WRONG()),
            "SOMEONE_ELSE_NAMED" => Some(g_SOMEONE_ELSE_NAMED()),
            "SOMETHING_UP" => Some(g_SOMETHING_UP()),
            "STATE_IN_THE_PAST" => Some(g_STATE_IN_THE_PAST()),
            "STATE_NOT_A_DOING" => Some(g_STATE_NOT_A_DOING()),
            "STATE_THEN" => Some(g_STATE_THEN()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "TAKING_SOMETHING_BACK" => Some(g_TAKING_SOMETHING_BACK()),
            "THEIR_OWN_DOING" => Some(g_THEIR_OWN_DOING()),
            "THE_JOB_NAMED" => Some(g_THE_JOB_NAMED()),
            "THE_OPENER" => Some(g_THE_OPENER()),
            "THE_THING_ITSELF" => Some(g_THE_THING_ITSELF()),
            "THE_WORDS_BETWEEN" => Some(g_THE_WORDS_BETWEEN()),
            "THINGS_THAT_STOP" => Some(g_THINGS_THAT_STOP()),
            "THINGS_THAT_STOP_AFTER_UP" => Some(g_THINGS_THAT_STOP_AFTER_UP()),
            "WEAR_ASK" => Some(g_WEAR_ASK()),
            "WEAR_WHEN_NOTHING_ANSWERS" => Some(g_WEAR_WHEN_NOTHING_ANSWERS()),
            "WEATHER_SUN" => Some(g_WEATHER_SUN()),
            "WHO_DID_IT" => Some(g_WHO_DID_IT()),
            "_AN_ASKING" => Some(g__AN_ASKING()),
            "_AN_ATTEMPT_COUNT" => Some(g__AN_ATTEMPT_COUNT()),
            "_A_BODY_UNWELL" => Some(g__A_BODY_UNWELL()),
            "_A_BODY_WORD" => Some(g__A_BODY_WORD()),
            "_A_BREAK_THAT_IS_GOOD" => Some(g__A_BREAK_THAT_IS_GOOD()),
            "_A_CORRECTION" => Some(g__A_CORRECTION()),
            "_A_DETAIL_OPENING" => Some(g__A_DETAIL_OPENING()),
            "_A_HARM_HAD" => Some(g__A_HARM_HAD()),
            "_A_HARM_IDIOM" => Some(g__A_HARM_IDIOM()),
            "_A_HARM_ILL" => Some(g__A_HARM_ILL()),
            "_A_HARM_ILL_NOT" => Some(g__A_HARM_ILL_NOT()),
            "_A_HARM_KNOCK" => Some(g__A_HARM_KNOCK()),
            "_A_HURT_BY_SOMEONE" => Some(g__A_HURT_BY_SOMEONE()),
            "_A_HURT_BY_SOMEONE_NOT" => Some(g__A_HURT_BY_SOMEONE_NOT()),
            "_A_HURT_DONE" => Some(g__A_HURT_DONE()),
            "_A_ILLNESS_WORD" => Some(g__A_ILLNESS_WORD()),
            "_A_IN_AGONY" => Some(g__A_IN_AGONY()),
            "_A_NEGATIVE_AUXILIARY" => Some(g__A_NEGATIVE_AUXILIARY()),
            "_A_OUT_OF_SOCKET" => Some(g__A_OUT_OF_SOCKET()),
            "_A_PART_OUT" => Some(g__A_PART_OUT()),
            "_A_PART_WENT" => Some(g__A_PART_WENT()),
            "_A_PROGNOSIS" => Some(g__A_PROGNOSIS()),
            "_A_PROGNOSIS_THAT_IS_NOT" => Some(g__A_PROGNOSIS_THAT_IS_NOT()),
            "_A_SOMEONE" => Some(g__A_SOMEONE()),
            "_A_SYMPATHY" => Some(g__A_SYMPATHY()),
            "_A_TELLING_OF_WRONG" => Some(g__A_TELLING_OF_WRONG()),
            "_BREAK_WORDS" => Some(g__BREAK_WORDS()),
            "_ROUTINE_NOT_SHAPE" => Some(g__ROUTINE_NOT_SHAPE()),
            "_UNDERTAKING" => Some(g__UNDERTAKING()),
            "_a_bad_time_told" => Some(py_func("_a_bad_time_told", |args: &[Py]| _a_bad_time_told(args))),
            "_a_betrayal" => Some(py_func("_a_betrayal", |args: &[Py]| _a_betrayal(args))),
            "_a_birthday_still_to_come" => Some(py_func("_a_birthday_still_to_come", |args: &[Py]| _a_birthday_still_to_come(args))),
            "_a_care_home_line" => Some(py_func("_a_care_home_line", |args: &[Py]| _a_care_home_line(args))),
            "_a_chat_to_correct" => Some(py_func("_a_chat_to_correct", |args: &[Py]| _a_chat_to_correct(args))),
            "_a_claim_about_a_thing" => Some(py_func("_a_claim_about_a_thing", |args: &[Py]| _a_claim_about_a_thing(args))),
            "_a_countdown_in_a_care_thread" => Some(py_func("_a_countdown_in_a_care_thread", |args: &[Py]| _a_countdown_in_a_care_thread(args))),
            "_a_day_asked_without_a_cheer" => Some(py_func("_a_day_asked_without_a_cheer", |args: &[Py]| _a_day_asked_without_a_cheer(args))),
            "_a_day_of_someone_else" => Some(py_func("_a_day_of_someone_else", |args: &[Py]| _a_day_of_someone_else(args))),
            "_a_day_said_badly" => Some(py_func("_a_day_said_badly", |args: &[Py]| _a_day_said_badly(args))),
            "_a_day_told_instead_of_the_receipt" => Some(py_func("_a_day_told_instead_of_the_receipt", |args: &[Py]| _a_day_told_instead_of_the_receipt(args))),
            "_a_detail_about_what_was_said" => Some(py_func("_a_detail_about_what_was_said", |args: &[Py]| _a_detail_about_what_was_said(args))),
            "_a_detail_of_a_bad_day" => Some(py_func("_a_detail_of_a_bad_day", |args: &[Py]| _a_detail_of_a_bad_day(args))),
            "_a_dish_name" => Some(py_func("_a_dish_name", |args: &[Py]| _a_dish_name(args))),
            "_a_harm_said" => Some(py_func("_a_harm_said", |args: &[Py]| _a_harm_said(args))),
            "_a_lateness_report" => Some(py_func("_a_lateness_report", |args: &[Py]| _a_lateness_report(args))),
            "_a_low_day" => Some(py_func("_a_low_day", |args: &[Py]| _a_low_day(args))),
            "_a_name_told" => Some(py_func("_a_name_told", |args: &[Py]| _a_name_told(args))),
            "_a_name_told_reply" => Some(py_func("_a_name_told_reply", |args: &[Py]| _a_name_told_reply(args))),
            "_a_near_miss" => Some(py_func("_a_near_miss", |args: &[Py]| _a_near_miss(args))),
            "_a_part_in_a_state" => Some(py_func("_a_part_in_a_state", |args: &[Py]| _a_part_in_a_state(args))),
            "_a_part_is_cold" => Some(py_func("_a_part_is_cold", |args: &[Py]| _a_part_is_cold(args))),
            "_a_problem_stated" => Some(py_func("_a_problem_stated", |args: &[Py]| _a_problem_stated(args))),
            "_a_report_from_elsewhere" => Some(py_func("_a_report_from_elsewhere", |args: &[Py]| _a_report_from_elsewhere(args))),
            "_a_report_with_no_pronoun" => Some(py_func("_a_report_with_no_pronoun", |args: &[Py]| _a_report_with_no_pronoun(args))),
            "_a_small_day" => Some(py_func("_a_small_day", |args: &[Py]| _a_small_day(args))),
            "_a_small_day_shape" => Some(py_func("_a_small_day_shape", |args: &[Py]| _a_small_day_shape(args))),
            "_a_state_in_the_past" => Some(py_func("_a_state_in_the_past", |args: &[Py]| _a_state_in_the_past(args))),
            "_a_state_of_theirs" => Some(py_func("_a_state_of_theirs", |args: &[Py]| _a_state_of_theirs(args))),
            "_a_sum_with_nothing_beside_it" => Some(py_func("_a_sum_with_nothing_beside_it", |args: &[Py]| _a_sum_with_nothing_beside_it(args))),
            "_a_thing_got_ahead" => Some(py_func("_a_thing_got_ahead", |args: &[Py]| _a_thing_got_ahead(args))),
            "_a_thing_happened" => Some(py_func("_a_thing_happened", |args: &[Py]| _a_thing_happened(args))),
            "_a_thing_stopping_after" => Some(py_func("_a_thing_stopping_after", |args: &[Py]| _a_thing_stopping_after(args))),
            "_a_thing_they_regret" => Some(py_func("_a_thing_they_regret", |args: &[Py]| _a_thing_they_regret(args))),
            "_a_things_state" => Some(py_func("_a_things_state", |args: &[Py]| _a_things_state(args))),
            "_a_wear_ask_about_a_day" => Some(py_func("_a_wear_ask_about_a_day", |args: &[Py]| _a_wear_ask_about_a_day(args))),
            "_act_asked_of_me" => Some(py_func("_act_asked_of_me", |args: &[Py]| _act_asked_of_me(args))),
            "_acts_for_me" => Some(py_func("_acts_for_me", |args: &[Py]| _acts_for_me(args))),
            "_an_emotion" => Some(py_func("_an_emotion", |args: &[Py]| _an_emotion(args))),
            "_an_order_it_cannot_do" => Some(py_func("_an_order_it_cannot_do", |args: &[Py]| _an_order_it_cannot_do(args))),
            "_answers_the_correction" => Some(py_func("_answers_the_correction", |args: &[Py]| _answers_the_correction(args))),
            "_asks_the_bot" => Some(py_func("_asks_the_bot", |args: &[Py]| _asks_the_bot(args))),
            "_body_state" => Some(py_func("_body_state", |args: &[Py]| _body_state(args))),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_cue_negated" => Some(py_func("_cue_negated", |args: &[Py]| _cue_negated(args))),
            "_flat_habit_stated" => Some(py_func("_flat_habit_stated", |args: &[Py]| _flat_habit_stated(args))),
            "_habit_is_a_problem" => Some(py_func("_habit_is_a_problem", |args: &[Py]| _habit_is_a_problem(args))),
            "_just_asked_about_the_day" => Some(py_func("_just_asked_about_the_day", |args: &[Py]| _just_asked_about_the_day(args))),
            "_loves_people" => Some(py_func("_loves_people", |args: &[Py]| _loves_people(args))),
            "_loves_someone" => Some(py_func("_loves_someone", |args: &[Py]| _loves_someone(args))),
            "_mixed_seed" => Some(py_func("_mixed_seed", |args: &[Py]| _mixed_seed(args))),
            "_money_came_to_them" => Some(py_func("_money_came_to_them", |args: &[Py]| _money_came_to_them(args))),
            "_mood" => Some(py_func("_mood", |args: &[Py]| _mood(args))),
            "_named_after_reply" => Some(py_func("_named_after_reply", |args: &[Py]| _named_after_reply(args))),
            "_no_such_thing_before" => Some(py_func("_no_such_thing_before", |args: &[Py]| _no_such_thing_before(args))),
            "_only_what_the_line_gives" => Some(py_func("_only_what_the_line_gives", |args: &[Py]| _only_what_the_line_gives(args))),
            "_options_already_up" => Some(py_func("_options_already_up", |args: &[Py]| _options_already_up(args))),
            "_reads" => Some(py_func("_reads", |args: &[Py]| _reads(args))),
            "_reads_as_a_bad_stretch" => Some(py_func("_reads_as_a_bad_stretch", |args: &[Py]| _reads_as_a_bad_stretch(args))),
            "_reads_value" => Some(py_func("_reads_value", |args: &[Py]| _reads_value(args))),
            "_repaired_of" => Some(py_func("_repaired_of", |args: &[Py]| _repaired_of(args))),
            "_reply_for" => Some(py_func("_reply_for", |args: &[Py]| _reply_for(args))),
            "_restated" => Some(py_func("_restated", |args: &[Py]| _restated(args))),
            "_routine_pool" => Some(py_func("_routine_pool", |args: &[Py]| _routine_pool(args))),
            "_says_there_is_nothing_to_wear" => Some(py_func("_says_there_is_nothing_to_wear", |args: &[Py]| _says_there_is_nothing_to_wear(args))),
            "_somebody_elses_day_in_thread" => Some(py_func("_somebody_elses_day_in_thread", |args: &[Py]| _somebody_elses_day_in_thread(args))),
            "_somebody_hurt" => Some(py_func("_somebody_hurt", |args: &[Py]| _somebody_hurt(args))),
            "_state_from" => Some(py_func("_state_from", |args: &[Py]| _state_from(args))),
            "_takes_something_back" => Some(py_func("_takes_something_back", |args: &[Py]| _takes_something_back(args))),
            "_the_birthday_said_back" => Some(py_func("_the_birthday_said_back", |args: &[Py]| _the_birthday_said_back(args))),
            "_the_clock_pool" => Some(py_func("_the_clock_pool", |args: &[Py]| _the_clock_pool(args))),
            "_the_crowd_is_ahead" => Some(py_func("_the_crowd_is_ahead", |args: &[Py]| _the_crowd_is_ahead(args))),
            "_the_day_was_already_bad" => Some(py_func("_the_day_was_already_bad", |args: &[Py]| _the_day_was_already_bad(args))),
            "_the_last_exchange" => Some(py_func("_the_last_exchange", |args: &[Py]| _the_last_exchange(args))),
            "_the_morning_has_a_clock" => Some(py_func("_the_morning_has_a_clock", |args: &[Py]| _the_morning_has_a_clock(args))),
            "_the_person_named_a_list" => Some(py_func("_the_person_named_a_list", |args: &[Py]| _the_person_named_a_list(args))),
            "_the_rest_of_the_day" => Some(py_func("_the_rest_of_the_day", |args: &[Py]| _the_rest_of_the_day(args))),
            "_the_state" => Some(py_func("_the_state", |args: &[Py]| _the_state(args))),
            "_the_store_has_the_recall" => Some(py_func("_the_store_has_the_recall", |args: &[Py]| _the_store_has_the_recall(args))),
            "_the_time_said" => Some(py_func("_the_time_said", |args: &[Py]| _the_time_said(args))),
            "_the_wear_pool_for_the_line" => Some(py_func("_the_wear_pool_for_the_line", |args: &[Py]| _the_wear_pool_for_the_line(args))),
            "_the_win_is_someone_elses" => Some(py_func("_the_win_is_someone_elses", |args: &[Py]| _the_win_is_someone_elses(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "reads_as_a_flat_habit" => Some(py_func("reads_as_a_flat_habit", |args: &[Py]| reads_as_a_flat_habit(args))),
            "reads_as_a_question_about_the_bot" => Some(py_func("reads_as_a_question_about_the_bot", |args: &[Py]| reads_as_a_question_about_the_bot(args))),
            "reads_as_no_pleasure" => Some(py_func("reads_as_no_pleasure", |args: &[Py]| reads_as_no_pleasure(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: None,
    }))
}
