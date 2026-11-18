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


pub static ABBREVIATION_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AMOUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANY_DO_WITH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_INSTRUCTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_OFFER_NOT_A_TEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ORDINAL_OPENING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKED_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_ABOUT_A_QUALITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_AN_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_FOR_MEANING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_IF_SERIOUS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_ITS_OWN_YES_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_OUT_LOUD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_TO_BE_TOLD_WHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_YES_OR_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_TO_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BARE_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BOUND_THAT_IS_THE_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CALCULATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COUNT_OF_THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COVER_LETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DURATION_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FEW_THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIGURE_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIGURE_OF_A_MONEY_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_GOOD_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HARM_IS_WHAT_IT_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOLDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_JOB_LANDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_JUDGEMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LIMIT_ON_ITS_USE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_TO_GIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_QUANTITY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RECALL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RULE_THAT_SETTLES_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SIDE_WINS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SIZE_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STANDING_AFTER_THE_TEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SUBORDINATE_FRAGMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_TO_GIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TIME_COUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TRADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VERB_THEN_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WELL_WISHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BALD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BALD_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_A_RECALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_A_SECOND_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_NOTHING_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_NOT_A_THING_ANYWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_NOT_A_THING_AT_THE_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_RANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_VALUE_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_WHAT_AS_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_WHAT_IS_PLAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_WRITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEST_OF_MANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEST_WAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAUSE_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAUSE_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_ANYWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOCK_SURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMFORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARATIVE_THAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARE_MYSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFIRM_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTRACTION_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTRAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONVERSION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONVERTED_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COSTS_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COST_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COST_STANDS_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTED_LIST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTED_STEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEFINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DIFFERENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DIRECTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOES_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOES_NOT_ANYWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOING_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DO_WITH_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DURATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ECHO_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ECHO_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EDGED_PART_OF_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVIDENCE_SAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPLAINS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPLICIT_COMPARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXTENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_PERSON_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIX_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOLLOWUP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_A_LIST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_A_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GARDEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVE_ME_AN_IDIOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVE_ME_A_FEW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVE_ME_LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVE_ME_MADE_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVE_ME_NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVE_ME_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVING_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_FIGURE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOOD_WAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HALF_A_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANDOFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAVE_RECALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK_NO_TASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOUR_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_ARE_YOU: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_ASK_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_COME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_I: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_IT_HAPPENED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_IT_WENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_IT_WORKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MANY_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MEASURE_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_SOON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HYPOTHETICAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IF_IT_HAPPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IMPERATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INSTRUCTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INSTRUCTS_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_DONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IT_HELPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static I_OFFER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOB_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JUDGED_MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LABEL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LABEL_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADING_JOIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LENGTH_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LINE_TEXT_FIELDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIST_AFTER_COLON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LONG_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MADE_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANS_ARTICLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANS_BY_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANS_INFINITIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANS_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEANS_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEASURED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEASURE_LABELS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MECHANISM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEMBERS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MERIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static METHOD_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static METHOD_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MINUTES_AS_A_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_RATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_VERB_NOT_A_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MUST_BE_A_TRADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MY_THING_CALLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_CLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_IN_EMPHASIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAME_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMING_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEED_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEED_TO_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEED_TO_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATED_AUX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEW_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEXT_IDEA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_TO_GO_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_AN_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_GIVING_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_LABEL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_PRICE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_STEP_BUT_REASSURANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_VERB_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_KINDS_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_STEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_WORDS_ANYWAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_WORDS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_FIXED_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_CLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPEN_WHO_HELP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPINION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPTIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPTIONS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORIGIN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORIGIN_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OUTCOME_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAIRING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_DEFINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PATH_THEN_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERMISSION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERMISSION_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_FULL_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_RELATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICKS_BY_COMPARISON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICK_BY_CASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICK_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICK_ONE_OF_A_KIND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICK_SIDES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_AS_EVIDENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_AT_HOME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_KIND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_NAME_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINT_NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREFERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_ADJECTIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_ADJECTIVE_KEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_ADJECTIVE_THE_ASK_IS_LATER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_ADJECTIVE_WHY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_GIVEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_IDIOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURPOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURPOSE_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUALITY_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUOTED_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RATE_OF_A_COUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RATE_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REASON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REASON_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REASON_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_AMOUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_KIND_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_KIND_NOUN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_OF_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_PRONOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_SHAPE_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_SHAPE_BEFORE_KIND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFERS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPEAT_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPORTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULE_CONDITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULE_DROP_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULE_GO_NEW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULE_KEEP_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_NOTHING_ABOUT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_WHAT_IT_DOES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEASON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEASON_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEEKS_HELP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENDS_THEM_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENTENCE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEQUENCES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SETTLED_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SET_LABEL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIDES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIDES_VS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIDE_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIDE_PAIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIGN_OFF_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOURCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOURCE_COLON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOURCE_DOES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOURCE_LIST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOURCE_MADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOURCE_MAKES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPECIFIC_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPOT_OF_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPOT_ON_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPOT_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAGE_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATES_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEPS_ARE_COUNTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEPS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEPS_AS_A_DISTANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEPS_INSIDE_A_LIST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBSTITUTE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUM_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUM_OUT_LOUD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAIL_VERDICT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TELLS_A_TEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TELL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TEST_IS_THE_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_SIGN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_THING_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_WINNER_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THINGS_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_THAT_EXTENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRAILING_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNSETTLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static USUALLY_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_ABOUT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_AFTER_A_CASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_AFTER_A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_BAND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_ECHO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_INSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_INSIDE_QUALITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_ON_A_SCALE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTS_INFO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTS_OF_THE_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTS_THE_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_ARE_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_ARE_TIMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_CAN_I_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_CAN_I_DO_WITH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_DOES_MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_ELSE_STEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_ELSE_WITH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_HELPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_IS_PER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_I_GROW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_I_NEED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_I_WRITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_SHOULD_I_DO_WITH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_THING_FRAGMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TIME_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_BAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_PICK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_STEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_YOU_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_YOU_TAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_ALONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_MONEY_GOES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_POINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_RECALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_RELATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHICH_DAY_IS_BEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_AND_MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_OF_A_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_OF_A_PRONOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORDS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WOULD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRITES_A_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRITES_IN_A_DOCUMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRITES_TO_A_MESSAGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRITTEN_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRONG_ASKED_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRONG_RECALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YESNO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ADVISES_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_AGE_BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_AGE_BIRTHDAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_AGE_MONTHS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_AGE_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_AGE_SPELLED_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_AGE_YEARS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ARTICLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_CLAUSE_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_CLAUSE_SPLIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FIGURE_AGAINST_THE_STANDARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_QUANTITY_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_QUANTITY_RELATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_QUANTITY_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_QUOTE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SOCIAL_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHECKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CLEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DURATION_QUALIFIER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _EMPTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ENDS_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FIGURES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FIGURE_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HOUR_24: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _JUDGES_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LIST_OF_THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MEASURE_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MEASURE_THEN_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NAMING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_ROLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_THE_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NO_WAY_TO_KNOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OPINION_OF_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PARTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PREFERS_STATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PRONOUN_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PUTS_THE_THING_TO_USE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _QUALITIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _QUALITY_OF_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _QUALITY_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ROLE_ENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ROLE_OFFICES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAYS_WHAT_IT_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SIDE_EMPTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STEPS_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TALKING_ABOUT_A_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_VERDICT_IS_EARLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TRAILING_MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WORD_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WORKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _gives_as_read: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _steps_by_the_verb_list: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABBREVIATION_END() -> Py { ensure_init(); ABBREVIATION_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABBREVIATION_END(v: Py) { ensure_init(); if let Ok(mut w) = ABBREVIATION_END.write() { *w = v; } }
pub fn g_ADVICE() -> Py { ensure_init(); ADVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE.write() { *w = v; } }
pub fn g_AMOUNT() -> Py { ensure_init(); AMOUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AMOUNT(v: Py) { ensure_init(); if let Ok(mut w) = AMOUNT.write() { *w = v; } }
pub fn g_ANSWER() -> Py { ensure_init(); ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = ANSWER.write() { *w = v; } }
pub fn g_ANY_DO_WITH() -> Py { ensure_init(); ANY_DO_WITH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANY_DO_WITH(v: Py) { ensure_init(); if let Ok(mut w) = ANY_DO_WITH.write() { *w = v; } }
pub fn g_AN_INSTRUCTION() -> Py { ensure_init(); AN_INSTRUCTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_INSTRUCTION(v: Py) { ensure_init(); if let Ok(mut w) = AN_INSTRUCTION.write() { *w = v; } }
pub fn g_AN_OFFER_NOT_A_TEST() -> Py { ensure_init(); AN_OFFER_NOT_A_TEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_OFFER_NOT_A_TEST(v: Py) { ensure_init(); if let Ok(mut w) = AN_OFFER_NOT_A_TEST.write() { *w = v; } }
pub fn g_AN_ORDINAL_OPENING() -> Py { ensure_init(); AN_ORDINAL_OPENING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ORDINAL_OPENING(v: Py) { ensure_init(); if let Ok(mut w) = AN_ORDINAL_OPENING.write() { *w = v; } }
pub fn g_ASKED_FOR() -> Py { ensure_init(); ASKED_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKED_FOR(v: Py) { ensure_init(); if let Ok(mut w) = ASKED_FOR.write() { *w = v; } }
pub fn g_ASKS_ABOUT_A_QUALITY() -> Py { ensure_init(); ASKS_ABOUT_A_QUALITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_A_QUALITY(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_ABOUT_A_QUALITY.write() { *w = v; } }
pub fn g_ASKS_AN_AGE() -> Py { ensure_init(); ASKS_AN_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_AN_AGE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_AN_AGE.write() { *w = v; } }
pub fn g_ASKS_FOR_MEANING() -> Py { ensure_init(); ASKS_FOR_MEANING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_MEANING(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_FOR_MEANING.write() { *w = v; } }
pub fn g_ASKS_HEAD() -> Py { ensure_init(); ASKS_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_HEAD.write() { *w = v; } }
pub fn g_ASKS_IF_SERIOUS() -> Py { ensure_init(); ASKS_IF_SERIOUS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_IF_SERIOUS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_IF_SERIOUS.write() { *w = v; } }
pub fn g_ASKS_ITS_OWN_YES_NO() -> Py { ensure_init(); ASKS_ITS_OWN_YES_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_ITS_OWN_YES_NO(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_ITS_OWN_YES_NO.write() { *w = v; } }
pub fn g_ASKS_OUT_LOUD() -> Py { ensure_init(); ASKS_OUT_LOUD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_OUT_LOUD(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_OUT_LOUD.write() { *w = v; } }
pub fn g_ASKS_THE_BOT() -> Py { ensure_init(); ASKS_THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_THE_BOT.write() { *w = v; } }
pub fn g_ASKS_TO_BE_TOLD_WHERE() -> Py { ensure_init(); ASKS_TO_BE_TOLD_WHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_TO_BE_TOLD_WHERE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_TO_BE_TOLD_WHERE.write() { *w = v; } }
pub fn g_ASKS_WHEN() -> Py { ensure_init(); ASKS_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WHEN.write() { *w = v; } }
pub fn g_ASKS_YES_OR_NO() -> Py { ensure_init(); ASKS_YES_OR_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_YES_OR_NO(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_YES_OR_NO.write() { *w = v; } }
pub fn g_ASK_TO_ASK() -> Py { ensure_init(); ASK_TO_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_TO_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ASK_TO_ASK.write() { *w = v; } }
pub fn g_A_BARE_NUMBER() -> Py { ensure_init(); A_BARE_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BARE_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = A_BARE_NUMBER.write() { *w = v; } }
pub fn g_A_BOUND_THAT_IS_THE_ANSWER() -> Py { ensure_init(); A_BOUND_THAT_IS_THE_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BOUND_THAT_IS_THE_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = A_BOUND_THAT_IS_THE_ANSWER.write() { *w = v; } }
pub fn g_A_CALCULATION() -> Py { ensure_init(); A_CALCULATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CALCULATION(v: Py) { ensure_init(); if let Ok(mut w) = A_CALCULATION.write() { *w = v; } }
pub fn g_A_COUNT_OF_THINGS() -> Py { ensure_init(); A_COUNT_OF_THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_OF_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = A_COUNT_OF_THINGS.write() { *w = v; } }
pub fn g_A_COVER_LETTER() -> Py { ensure_init(); A_COVER_LETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COVER_LETTER(v: Py) { ensure_init(); if let Ok(mut w) = A_COVER_LETTER.write() { *w = v; } }
pub fn g_A_DURATION_WORD() -> Py { ensure_init(); A_DURATION_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DURATION_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_DURATION_WORD.write() { *w = v; } }
pub fn g_A_FEW_THINGS() -> Py { ensure_init(); A_FEW_THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FEW_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = A_FEW_THINGS.write() { *w = v; } }
pub fn g_A_FIGURE_NAMED() -> Py { ensure_init(); A_FIGURE_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = A_FIGURE_NAMED.write() { *w = v; } }
pub fn g_A_FIGURE_OF_A_MONEY_VERB() -> Py { ensure_init(); A_FIGURE_OF_A_MONEY_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_OF_A_MONEY_VERB(v: Py) { ensure_init(); if let Ok(mut w) = A_FIGURE_OF_A_MONEY_VERB.write() { *w = v; } }
pub fn g_A_GOOD_TIME() -> Py { ensure_init(); A_GOOD_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_GOOD_TIME.write() { *w = v; } }
pub fn g_A_HARM_IS_WHAT_IT_IS() -> Py { ensure_init(); A_HARM_IS_WHAT_IT_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HARM_IS_WHAT_IT_IS(v: Py) { ensure_init(); if let Ok(mut w) = A_HARM_IS_WHAT_IT_IS.write() { *w = v; } }
pub fn g_A_HOLDER() -> Py { ensure_init(); A_HOLDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOLDER(v: Py) { ensure_init(); if let Ok(mut w) = A_HOLDER.write() { *w = v; } }
pub fn g_A_JOB_LANDS() -> Py { ensure_init(); A_JOB_LANDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_JOB_LANDS(v: Py) { ensure_init(); if let Ok(mut w) = A_JOB_LANDS.write() { *w = v; } }
pub fn g_A_JUDGEMENT() -> Py { ensure_init(); A_JUDGEMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_JUDGEMENT(v: Py) { ensure_init(); if let Ok(mut w) = A_JUDGEMENT.write() { *w = v; } }
pub fn g_A_LIMIT_ON_ITS_USE() -> Py { ensure_init(); A_LIMIT_ON_ITS_USE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LIMIT_ON_ITS_USE(v: Py) { ensure_init(); if let Ok(mut w) = A_LIMIT_ON_ITS_USE.write() { *w = v; } }
pub fn g_A_PERSON_TO_GIVE() -> Py { ensure_init(); A_PERSON_TO_GIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_TO_GIVE(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_TO_GIVE.write() { *w = v; } }
pub fn g_A_PRICE_ASK() -> Py { ensure_init(); A_PRICE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE_ASK.write() { *w = v; } }
pub fn g_A_QUANTITY_ASK() -> Py { ensure_init(); A_QUANTITY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_QUANTITY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_QUANTITY_ASK.write() { *w = v; } }
pub fn g_A_RECALL_ASK() -> Py { ensure_init(); A_RECALL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RECALL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_RECALL_ASK.write() { *w = v; } }
pub fn g_A_RULE_THAT_SETTLES_IT() -> Py { ensure_init(); A_RULE_THAT_SETTLES_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RULE_THAT_SETTLES_IT(v: Py) { ensure_init(); if let Ok(mut w) = A_RULE_THAT_SETTLES_IT.write() { *w = v; } }
pub fn g_A_SIDE_WINS() -> Py { ensure_init(); A_SIDE_WINS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SIDE_WINS(v: Py) { ensure_init(); if let Ok(mut w) = A_SIDE_WINS.write() { *w = v; } }
pub fn g_A_SIZE_LONG() -> Py { ensure_init(); A_SIZE_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SIZE_LONG(v: Py) { ensure_init(); if let Ok(mut w) = A_SIZE_LONG.write() { *w = v; } }
pub fn g_A_STANDING_AFTER_THE_TEST() -> Py { ensure_init(); A_STANDING_AFTER_THE_TEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STANDING_AFTER_THE_TEST(v: Py) { ensure_init(); if let Ok(mut w) = A_STANDING_AFTER_THE_TEST.write() { *w = v; } }
pub fn g_A_SUBORDINATE_FRAGMENT() -> Py { ensure_init(); A_SUBORDINATE_FRAGMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SUBORDINATE_FRAGMENT(v: Py) { ensure_init(); if let Ok(mut w) = A_SUBORDINATE_FRAGMENT.write() { *w = v; } }
pub fn g_A_THING_TO_GIVE() -> Py { ensure_init(); A_THING_TO_GIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_TO_GIVE(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_TO_GIVE.write() { *w = v; } }
pub fn g_A_TIME_COUNT() -> Py { ensure_init(); A_TIME_COUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TIME_COUNT(v: Py) { ensure_init(); if let Ok(mut w) = A_TIME_COUNT.write() { *w = v; } }
pub fn g_A_TRADE() -> Py { ensure_init(); A_TRADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TRADE(v: Py) { ensure_init(); if let Ok(mut w) = A_TRADE.write() { *w = v; } }
pub fn g_A_UNIT() -> Py { ensure_init(); A_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = A_UNIT.write() { *w = v; } }
pub fn g_A_VERB_THEN_THE_THING() -> Py { ensure_init(); A_VERB_THEN_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VERB_THEN_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_VERB_THEN_THE_THING.write() { *w = v; } }
pub fn g_A_WELL_WISHING() -> Py { ensure_init(); A_WELL_WISHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WELL_WISHING(v: Py) { ensure_init(); if let Ok(mut w) = A_WELL_WISHING.write() { *w = v; } }
pub fn g_BACK_HEAD() -> Py { ensure_init(); BACK_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = BACK_HEAD.write() { *w = v; } }
pub fn g_BALD() -> Py { ensure_init(); BALD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BALD(v: Py) { ensure_init(); if let Ok(mut w) = BALD.write() { *w = v; } }
pub fn g_BALD_VERB() -> Py { ensure_init(); BALD_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BALD_VERB(v: Py) { ensure_init(); if let Ok(mut w) = BALD_VERB.write() { *w = v; } }
pub fn g_BARE_A_RECALL() -> Py { ensure_init(); BARE_A_RECALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_A_RECALL(v: Py) { ensure_init(); if let Ok(mut w) = BARE_A_RECALL.write() { *w = v; } }
pub fn g_BARE_A_SECOND_QUESTION() -> Py { ensure_init(); BARE_A_SECOND_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_A_SECOND_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = BARE_A_SECOND_QUESTION.write() { *w = v; } }
pub fn g_BARE_NOTHING_ASKED() -> Py { ensure_init(); BARE_NOTHING_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_NOTHING_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = BARE_NOTHING_ASKED.write() { *w = v; } }
pub fn g_BARE_NOT_A_THING_ANYWHERE() -> Py { ensure_init(); BARE_NOT_A_THING_ANYWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_NOT_A_THING_ANYWHERE(v: Py) { ensure_init(); if let Ok(mut w) = BARE_NOT_A_THING_ANYWHERE.write() { *w = v; } }
pub fn g_BARE_NOT_A_THING_AT_THE_START() -> Py { ensure_init(); BARE_NOT_A_THING_AT_THE_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_NOT_A_THING_AT_THE_START(v: Py) { ensure_init(); if let Ok(mut w) = BARE_NOT_A_THING_AT_THE_START.write() { *w = v; } }
pub fn g_BARE_RANGE() -> Py { ensure_init(); BARE_RANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_RANGE(v: Py) { ensure_init(); if let Ok(mut w) = BARE_RANGE.write() { *w = v; } }
pub fn g_BARE_VALUE_NOUN() -> Py { ensure_init(); BARE_VALUE_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_VALUE_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = BARE_VALUE_NOUN.write() { *w = v; } }
pub fn g_BARE_WHAT_AS_IS() -> Py { ensure_init(); BARE_WHAT_AS_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_WHAT_AS_IS(v: Py) { ensure_init(); if let Ok(mut w) = BARE_WHAT_AS_IS.write() { *w = v; } }
pub fn g_BARE_WHAT_IS_PLAIN() -> Py { ensure_init(); BARE_WHAT_IS_PLAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_WHAT_IS_PLAIN(v: Py) { ensure_init(); if let Ok(mut w) = BARE_WHAT_IS_PLAIN.write() { *w = v; } }
pub fn g_BARE_WRITE() -> Py { ensure_init(); BARE_WRITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_WRITE(v: Py) { ensure_init(); if let Ok(mut w) = BARE_WRITE.write() { *w = v; } }
pub fn g_BEST_OF_MANY() -> Py { ensure_init(); BEST_OF_MANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEST_OF_MANY(v: Py) { ensure_init(); if let Ok(mut w) = BEST_OF_MANY.write() { *w = v; } }
pub fn g_BEST_WAY() -> Py { ensure_init(); BEST_WAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEST_WAY(v: Py) { ensure_init(); if let Ok(mut w) = BEST_WAY.write() { *w = v; } }
pub fn g_CALENDAR_WORD() -> Py { ensure_init(); CALENDAR_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_WORD(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR_WORD.write() { *w = v; } }
pub fn g_CAUSE() -> Py { ensure_init(); CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = CAUSE.write() { *w = v; } }
pub fn g_CAUSE_IS() -> Py { ensure_init(); CAUSE_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAUSE_IS(v: Py) { ensure_init(); if let Ok(mut w) = CAUSE_IS.write() { *w = v; } }
pub fn g_CAUSE_WHEN() -> Py { ensure_init(); CAUSE_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAUSE_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = CAUSE_WHEN.write() { *w = v; } }
pub fn g_CHOICE() -> Py { ensure_init(); CHOICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOICE(v: Py) { ensure_init(); if let Ok(mut w) = CHOICE.write() { *w = v; } }
pub fn g_CLOCK() -> Py { ensure_init(); CLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK.write() { *w = v; } }
pub fn g_CLOCK_ANYWHERE() -> Py { ensure_init(); CLOCK_ANYWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_ANYWHERE(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_ANYWHERE.write() { *w = v; } }
pub fn g_CLOCK_SURE() -> Py { ensure_init(); CLOCK_SURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOCK_SURE(v: Py) { ensure_init(); if let Ok(mut w) = CLOCK_SURE.write() { *w = v; } }
pub fn g_COMFORT() -> Py { ensure_init(); COMFORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMFORT(v: Py) { ensure_init(); if let Ok(mut w) = COMFORT.write() { *w = v; } }
pub fn g_COMPARATIVE_THAN() -> Py { ensure_init(); COMPARATIVE_THAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARATIVE_THAN(v: Py) { ensure_init(); if let Ok(mut w) = COMPARATIVE_THAN.write() { *w = v; } }
pub fn g_COMPARE() -> Py { ensure_init(); COMPARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARE(v: Py) { ensure_init(); if let Ok(mut w) = COMPARE.write() { *w = v; } }
pub fn g_COMPARED() -> Py { ensure_init(); COMPARED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARED(v: Py) { ensure_init(); if let Ok(mut w) = COMPARED.write() { *w = v; } }
pub fn g_COMPARE_MYSELF() -> Py { ensure_init(); COMPARE_MYSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARE_MYSELF(v: Py) { ensure_init(); if let Ok(mut w) = COMPARE_MYSELF.write() { *w = v; } }
pub fn g_COMPARISON() -> Py { ensure_init(); COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = COMPARISON.write() { *w = v; } }
pub fn g_CONFIRM_CAUSE() -> Py { ensure_init(); CONFIRM_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFIRM_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = CONFIRM_CAUSE.write() { *w = v; } }
pub fn g_CONTRACTION_OPENER() -> Py { ensure_init(); CONTRACTION_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTRACTION_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = CONTRACTION_OPENER.write() { *w = v; } }
pub fn g_CONTRAST() -> Py { ensure_init(); CONTRAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTRAST(v: Py) { ensure_init(); if let Ok(mut w) = CONTRAST.write() { *w = v; } }
pub fn g_CONVERSION() -> Py { ensure_init(); CONVERSION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONVERSION(v: Py) { ensure_init(); if let Ok(mut w) = CONVERSION.write() { *w = v; } }
pub fn g_CONVERTED_FIGURE() -> Py { ensure_init(); CONVERTED_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONVERTED_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = CONVERTED_FIGURE.write() { *w = v; } }
pub fn g_COPULA() -> Py { ensure_init(); COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); if let Ok(mut w) = COPULA.write() { *w = v; } }
pub fn g_COSTS_NOTHING() -> Py { ensure_init(); COSTS_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COSTS_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = COSTS_NOTHING.write() { *w = v; } }
pub fn g_COST_ASK() -> Py { ensure_init(); COST_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COST_ASK(v: Py) { ensure_init(); if let Ok(mut w) = COST_ASK.write() { *w = v; } }
pub fn g_COST_STANDS_IN() -> Py { ensure_init(); COST_STANDS_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COST_STANDS_IN(v: Py) { ensure_init(); if let Ok(mut w) = COST_STANDS_IN.write() { *w = v; } }
pub fn g_COUNTED_LIST() -> Py { ensure_init(); COUNTED_LIST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTED_LIST(v: Py) { ensure_init(); if let Ok(mut w) = COUNTED_LIST.write() { *w = v; } }
pub fn g_COUNTED_STEPS() -> Py { ensure_init(); COUNTED_STEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTED_STEPS(v: Py) { ensure_init(); if let Ok(mut w) = COUNTED_STEPS.write() { *w = v; } }
pub fn g_DATE() -> Py { ensure_init(); DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DATE(v: Py) { ensure_init(); if let Ok(mut w) = DATE.write() { *w = v; } }
pub fn g_DAY_NAME() -> Py { ensure_init(); DAY_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_NAME(v: Py) { ensure_init(); if let Ok(mut w) = DAY_NAME.write() { *w = v; } }
pub fn g_DEFINES() -> Py { ensure_init(); DEFINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEFINES(v: Py) { ensure_init(); if let Ok(mut w) = DEFINES.write() { *w = v; } }
pub fn g_DIFFERENCE() -> Py { ensure_init(); DIFFERENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DIFFERENCE(v: Py) { ensure_init(); if let Ok(mut w) = DIFFERENCE.write() { *w = v; } }
pub fn g_DIRECTED() -> Py { ensure_init(); DIRECTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DIRECTED(v: Py) { ensure_init(); if let Ok(mut w) = DIRECTED.write() { *w = v; } }
pub fn g_DOES() -> Py { ensure_init(); DOES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOES(v: Py) { ensure_init(); if let Ok(mut w) = DOES.write() { *w = v; } }
pub fn g_DOES_NOT() -> Py { ensure_init(); DOES_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOES_NOT(v: Py) { ensure_init(); if let Ok(mut w) = DOES_NOT.write() { *w = v; } }
pub fn g_DOES_NOT_ANYWHERE() -> Py { ensure_init(); DOES_NOT_ANYWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOES_NOT_ANYWHERE(v: Py) { ensure_init(); if let Ok(mut w) = DOES_NOT_ANYWHERE.write() { *w = v; } }
pub fn g_DOING_WRONG() -> Py { ensure_init(); DOING_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOING_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = DOING_WRONG.write() { *w = v; } }
pub fn g_DO_WITH_A_PERSON() -> Py { ensure_init(); DO_WITH_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DO_WITH_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = DO_WITH_A_PERSON.write() { *w = v; } }
pub fn g_DURATION() -> Py { ensure_init(); DURATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DURATION(v: Py) { ensure_init(); if let Ok(mut w) = DURATION.write() { *w = v; } }
pub fn g_ECHO_HEAD() -> Py { ensure_init(); ECHO_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ECHO_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = ECHO_HEAD.write() { *w = v; } }
pub fn g_ECHO_VERDICT() -> Py { ensure_init(); ECHO_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ECHO_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = ECHO_VERDICT.write() { *w = v; } }
pub fn g_EDGED_PART_OF_DAY() -> Py { ensure_init(); EDGED_PART_OF_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EDGED_PART_OF_DAY(v: Py) { ensure_init(); if let Ok(mut w) = EDGED_PART_OF_DAY.write() { *w = v; } }
pub fn g_EVIDENCE_SAYS() -> Py { ensure_init(); EVIDENCE_SAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVIDENCE_SAYS(v: Py) { ensure_init(); if let Ok(mut w) = EVIDENCE_SAYS.write() { *w = v; } }
pub fn g_EXPLAINS() -> Py { ensure_init(); EXPLAINS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPLAINS(v: Py) { ensure_init(); if let Ok(mut w) = EXPLAINS.write() { *w = v; } }
pub fn g_EXPLICIT_COMPARE() -> Py { ensure_init(); EXPLICIT_COMPARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPLICIT_COMPARE(v: Py) { ensure_init(); if let Ok(mut w) = EXPLICIT_COMPARE.write() { *w = v; } }
pub fn g_EXTENT() -> Py { ensure_init(); EXTENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXTENT(v: Py) { ensure_init(); if let Ok(mut w) = EXTENT.write() { *w = v; } }
pub fn g_FIGURE() -> Py { ensure_init(); FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE.write() { *w = v; } }
pub fn g_FIGURE_HEAD() -> Py { ensure_init(); FIGURE_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE_HEAD.write() { *w = v; } }
pub fn g_FIRST_PERSON_VERDICT() -> Py { ensure_init(); FIRST_PERSON_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_PERSON_VERDICT.write() { *w = v; } }
pub fn g_FIX_ASK() -> Py { ensure_init(); FIX_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIX_ASK(v: Py) { ensure_init(); if let Ok(mut w) = FIX_ASK.write() { *w = v; } }
pub fn g_FOLLOWUP() -> Py { ensure_init(); FOLLOWUP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOLLOWUP(v: Py) { ensure_init(); if let Ok(mut w) = FOLLOWUP.write() { *w = v; } }
pub fn g_FOR_A_LIST() -> Py { ensure_init(); FOR_A_LIST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_A_LIST(v: Py) { ensure_init(); if let Ok(mut w) = FOR_A_LIST.write() { *w = v; } }
pub fn g_FOR_A_TIME() -> Py { ensure_init(); FOR_A_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_A_TIME(v: Py) { ensure_init(); if let Ok(mut w) = FOR_A_TIME.write() { *w = v; } }
pub fn g_FRAME_HEAD() -> Py { ensure_init(); FRAME_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_HEAD.write() { *w = v; } }
pub fn g_GARDEN() -> Py { ensure_init(); GARDEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GARDEN(v: Py) { ensure_init(); if let Ok(mut w) = GARDEN.write() { *w = v; } }
pub fn g_GIVE_ME_AN_IDIOM() -> Py { ensure_init(); GIVE_ME_AN_IDIOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVE_ME_AN_IDIOM(v: Py) { ensure_init(); if let Ok(mut w) = GIVE_ME_AN_IDIOM.write() { *w = v; } }
pub fn g_GIVE_ME_A_FEW() -> Py { ensure_init(); GIVE_ME_A_FEW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVE_ME_A_FEW(v: Py) { ensure_init(); if let Ok(mut w) = GIVE_ME_A_FEW.write() { *w = v; } }
pub fn g_GIVE_ME_LEAD_IN() -> Py { ensure_init(); GIVE_ME_LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVE_ME_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = GIVE_ME_LEAD_IN.write() { *w = v; } }
pub fn g_GIVE_ME_MADE_UP() -> Py { ensure_init(); GIVE_ME_MADE_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVE_ME_MADE_UP(v: Py) { ensure_init(); if let Ok(mut w) = GIVE_ME_MADE_UP.write() { *w = v; } }
pub fn g_GIVE_ME_NOT_A_THING() -> Py { ensure_init(); GIVE_ME_NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVE_ME_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = GIVE_ME_NOT_A_THING.write() { *w = v; } }
pub fn g_GIVE_ME_ONE() -> Py { ensure_init(); GIVE_ME_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVE_ME_ONE(v: Py) { ensure_init(); if let Ok(mut w) = GIVE_ME_ONE.write() { *w = v; } }
pub fn g_GIVING_FRAME() -> Py { ensure_init(); GIVING_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVING_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = GIVING_FRAME.write() { *w = v; } }
pub fn g_GOOD_FIGURE() -> Py { ensure_init(); GOOD_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_FIGURE.write() { *w = v; } }
pub fn g_GOOD_FIGURE_WORD() -> Py { ensure_init(); GOOD_FIGURE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_FIGURE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_FIGURE_WORD.write() { *w = v; } }
pub fn g_GOOD_MEASURE() -> Py { ensure_init(); GOOD_MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_MEASURE.write() { *w = v; } }
pub fn g_GOOD_ONE() -> Py { ensure_init(); GOOD_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_ONE(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_ONE.write() { *w = v; } }
pub fn g_GOOD_THING() -> Py { ensure_init(); GOOD_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_THING(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_THING.write() { *w = v; } }
pub fn g_GOOD_WAY() -> Py { ensure_init(); GOOD_WAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOOD_WAY(v: Py) { ensure_init(); if let Ok(mut w) = GOOD_WAY.write() { *w = v; } }
pub fn g_HALF_A_UNIT() -> Py { ensure_init(); HALF_A_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HALF_A_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = HALF_A_UNIT.write() { *w = v; } }
pub fn g_HANDOFF() -> Py { ensure_init(); HANDOFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANDOFF(v: Py) { ensure_init(); if let Ok(mut w) = HANDOFF.write() { *w = v; } }
pub fn g_HAVE_RECALL() -> Py { ensure_init(); HAVE_RECALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAVE_RECALL(v: Py) { ensure_init(); if let Ok(mut w) = HAVE_RECALL.write() { *w = v; } }
pub fn g_HELP_ASK_ANY() -> Py { ensure_init(); HELP_ASK_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK_ANY(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK_ANY.write() { *w = v; } }
pub fn g_HELP_ASK_NO_TASK() -> Py { ensure_init(); HELP_ASK_NO_TASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK_NO_TASK(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK_NO_TASK.write() { *w = v; } }
pub fn g_HELP_ME() -> Py { ensure_init(); HELP_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ME(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ME.write() { *w = v; } }
pub fn g_HELP_WORD() -> Py { ensure_init(); HELP_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_WORD(v: Py) { ensure_init(); if let Ok(mut w) = HELP_WORD.write() { *w = v; } }
pub fn g_HOUR_WORD() -> Py { ensure_init(); HOUR_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOUR_WORD(v: Py) { ensure_init(); if let Ok(mut w) = HOUR_WORD.write() { *w = v; } }
pub fn g_HOW_ARE_YOU() -> Py { ensure_init(); HOW_ARE_YOU.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_ARE_YOU(v: Py) { ensure_init(); if let Ok(mut w) = HOW_ARE_YOU.write() { *w = v; } }
pub fn g_HOW_ASK_FORM() -> Py { ensure_init(); HOW_ASK_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_ASK_FORM(v: Py) { ensure_init(); if let Ok(mut w) = HOW_ASK_FORM.write() { *w = v; } }
pub fn g_HOW_COME() -> Py { ensure_init(); HOW_COME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_COME(v: Py) { ensure_init(); if let Ok(mut w) = HOW_COME.write() { *w = v; } }
pub fn g_HOW_I() -> Py { ensure_init(); HOW_I.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_I(v: Py) { ensure_init(); if let Ok(mut w) = HOW_I.write() { *w = v; } }
pub fn g_HOW_IT_HAPPENED() -> Py { ensure_init(); HOW_IT_HAPPENED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_IT_HAPPENED(v: Py) { ensure_init(); if let Ok(mut w) = HOW_IT_HAPPENED.write() { *w = v; } }
pub fn g_HOW_IT_WENT() -> Py { ensure_init(); HOW_IT_WENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_IT_WENT(v: Py) { ensure_init(); if let Ok(mut w) = HOW_IT_WENT.write() { *w = v; } }
pub fn g_HOW_IT_WORKS() -> Py { ensure_init(); HOW_IT_WORKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_IT_WORKS(v: Py) { ensure_init(); if let Ok(mut w) = HOW_IT_WORKS.write() { *w = v; } }
pub fn g_HOW_MANY_TIME() -> Py { ensure_init(); HOW_MANY_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_TIME(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MANY_TIME.write() { *w = v; } }
pub fn g_HOW_MEASURE() -> Py { ensure_init(); HOW_MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MEASURE.write() { *w = v; } }
pub fn g_HOW_MEASURE_HEAD() -> Py { ensure_init(); HOW_MEASURE_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MEASURE_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MEASURE_HEAD.write() { *w = v; } }
pub fn g_HOW_SOON() -> Py { ensure_init(); HOW_SOON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_SOON(v: Py) { ensure_init(); if let Ok(mut w) = HOW_SOON.write() { *w = v; } }
pub fn g_HOW_TO_ANSWER() -> Py { ensure_init(); HOW_TO_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO_ANSWER.write() { *w = v; } }
pub fn g_HOW_TO_DO() -> Py { ensure_init(); HOW_TO_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_DO(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO_DO.write() { *w = v; } }
pub fn g_HYPOTHETICAL() -> Py { ensure_init(); HYPOTHETICAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HYPOTHETICAL(v: Py) { ensure_init(); if let Ok(mut w) = HYPOTHETICAL.write() { *w = v; } }
pub fn g_IF_IT_HAPPENS() -> Py { ensure_init(); IF_IT_HAPPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IF_IT_HAPPENS(v: Py) { ensure_init(); if let Ok(mut w) = IF_IT_HAPPENS.write() { *w = v; } }
pub fn g_IMPERATIVE() -> Py { ensure_init(); IMPERATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IMPERATIVE(v: Py) { ensure_init(); if let Ok(mut w) = IMPERATIVE.write() { *w = v; } }
pub fn g_INSTRUCTS() -> Py { ensure_init(); INSTRUCTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INSTRUCTS(v: Py) { ensure_init(); if let Ok(mut w) = INSTRUCTS.write() { *w = v; } }
pub fn g_INSTRUCTS_CLAUSE() -> Py { ensure_init(); INSTRUCTS_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INSTRUCTS_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = INSTRUCTS_CLAUSE.write() { *w = v; } }
pub fn g_IS_IT_DONE() -> Py { ensure_init(); IS_IT_DONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_DONE(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_DONE.write() { *w = v; } }
pub fn g_IT_HELPS() -> Py { ensure_init(); IT_HELPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IT_HELPS(v: Py) { ensure_init(); if let Ok(mut w) = IT_HELPS.write() { *w = v; } }
pub fn g_I_OFFER() -> Py { ensure_init(); I_OFFER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_I_OFFER(v: Py) { ensure_init(); if let Ok(mut w) = I_OFFER.write() { *w = v; } }
pub fn g_JOB() -> Py { ensure_init(); JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOB(v: Py) { ensure_init(); if let Ok(mut w) = JOB.write() { *w = v; } }
pub fn g_JOB_ANY() -> Py { ensure_init(); JOB_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOB_ANY(v: Py) { ensure_init(); if let Ok(mut w) = JOB_ANY.write() { *w = v; } }
pub fn g_JUDGED_MEASURE() -> Py { ensure_init(); JUDGED_MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JUDGED_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = JUDGED_MEASURE.write() { *w = v; } }
pub fn g_KINDS_OF() -> Py { ensure_init(); KINDS_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_OF(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_OF.write() { *w = v; } }
pub fn g_LABEL() -> Py { ensure_init(); LABEL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LABEL(v: Py) { ensure_init(); if let Ok(mut w) = LABEL.write() { *w = v; } }
pub fn g_LABEL_HEAD() -> Py { ensure_init(); LABEL_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LABEL_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = LABEL_HEAD.write() { *w = v; } }
pub fn g_LEADING_JOIN() -> Py { ensure_init(); LEADING_JOIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADING_JOIN(v: Py) { ensure_init(); if let Ok(mut w) = LEADING_JOIN.write() { *w = v; } }
pub fn g_LENGTH_ASKED() -> Py { ensure_init(); LENGTH_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LENGTH_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = LENGTH_ASKED.write() { *w = v; } }
pub fn g_LINE_TEXT_FIELDS() -> Py { ensure_init(); LINE_TEXT_FIELDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LINE_TEXT_FIELDS(v: Py) { ensure_init(); if let Ok(mut w) = LINE_TEXT_FIELDS.write() { *w = v; } }
pub fn g_LIST_AFTER_COLON() -> Py { ensure_init(); LIST_AFTER_COLON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIST_AFTER_COLON(v: Py) { ensure_init(); if let Ok(mut w) = LIST_AFTER_COLON.write() { *w = v; } }
pub fn g_LONG_TIME() -> Py { ensure_init(); LONG_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LONG_TIME(v: Py) { ensure_init(); if let Ok(mut w) = LONG_TIME.write() { *w = v; } }
pub fn g_MADE_OF() -> Py { ensure_init(); MADE_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MADE_OF(v: Py) { ensure_init(); if let Ok(mut w) = MADE_OF.write() { *w = v; } }
pub fn g_MEANING() -> Py { ensure_init(); MEANING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANING(v: Py) { ensure_init(); if let Ok(mut w) = MEANING.write() { *w = v; } }
pub fn g_MEANS() -> Py { ensure_init(); MEANS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANS(v: Py) { ensure_init(); if let Ok(mut w) = MEANS.write() { *w = v; } }
pub fn g_MEANS_ARTICLE() -> Py { ensure_init(); MEANS_ARTICLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANS_ARTICLE(v: Py) { ensure_init(); if let Ok(mut w) = MEANS_ARTICLE.write() { *w = v; } }
pub fn g_MEANS_BY_DOING() -> Py { ensure_init(); MEANS_BY_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANS_BY_DOING(v: Py) { ensure_init(); if let Ok(mut w) = MEANS_BY_DOING.write() { *w = v; } }
pub fn g_MEANS_INFINITIVE() -> Py { ensure_init(); MEANS_INFINITIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANS_INFINITIVE(v: Py) { ensure_init(); if let Ok(mut w) = MEANS_INFINITIVE.write() { *w = v; } }
pub fn g_MEANS_NAMED() -> Py { ensure_init(); MEANS_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANS_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = MEANS_NAMED.write() { *w = v; } }
pub fn g_MEANS_OF() -> Py { ensure_init(); MEANS_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEANS_OF(v: Py) { ensure_init(); if let Ok(mut w) = MEANS_OF.write() { *w = v; } }
pub fn g_MEASURED() -> Py { ensure_init(); MEASURED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEASURED(v: Py) { ensure_init(); if let Ok(mut w) = MEASURED.write() { *w = v; } }
pub fn g_MEASURE_LABELS() -> Py { ensure_init(); MEASURE_LABELS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEASURE_LABELS(v: Py) { ensure_init(); if let Ok(mut w) = MEASURE_LABELS.write() { *w = v; } }
pub fn g_MECHANISM() -> Py { ensure_init(); MECHANISM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MECHANISM(v: Py) { ensure_init(); if let Ok(mut w) = MECHANISM.write() { *w = v; } }
pub fn g_MEMBERS_ASK() -> Py { ensure_init(); MEMBERS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEMBERS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = MEMBERS_ASK.write() { *w = v; } }
pub fn g_MERIT() -> Py { ensure_init(); MERIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MERIT(v: Py) { ensure_init(); if let Ok(mut w) = MERIT.write() { *w = v; } }
pub fn g_METHOD_OPENS() -> Py { ensure_init(); METHOD_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_METHOD_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = METHOD_OPENS.write() { *w = v; } }
pub fn g_METHOD_VERBS() -> Py { ensure_init(); METHOD_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_METHOD_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = METHOD_VERBS.write() { *w = v; } }
pub fn g_MINUTES_AS_A_LENGTH() -> Py { ensure_init(); MINUTES_AS_A_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MINUTES_AS_A_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = MINUTES_AS_A_LENGTH.write() { *w = v; } }
pub fn g_MONEY() -> Py { ensure_init(); MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = MONEY.write() { *w = v; } }
pub fn g_MONEY_RATE() -> Py { ensure_init(); MONEY_RATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_RATE(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_RATE.write() { *w = v; } }
pub fn g_MONEY_VERB_NOT_A_PRICE() -> Py { ensure_init(); MONEY_VERB_NOT_A_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_VERB_NOT_A_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_VERB_NOT_A_PRICE.write() { *w = v; } }
pub fn g_MUST_BE_A_TRADE() -> Py { ensure_init(); MUST_BE_A_TRADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MUST_BE_A_TRADE(v: Py) { ensure_init(); if let Ok(mut w) = MUST_BE_A_TRADE.write() { *w = v; } }
pub fn g_MY_THING_CALLED() -> Py { ensure_init(); MY_THING_CALLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MY_THING_CALLED(v: Py) { ensure_init(); if let Ok(mut w) = MY_THING_CALLED.write() { *w = v; } }
pub fn g_NAMED_CLOCK() -> Py { ensure_init(); NAMED_CLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_CLOCK(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_CLOCK.write() { *w = v; } }
pub fn g_NAMED_HEAD() -> Py { ensure_init(); NAMED_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_HEAD.write() { *w = v; } }
pub fn g_NAMED_TIME() -> Py { ensure_init(); NAMED_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_TIME(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_TIME.write() { *w = v; } }
pub fn g_NAMED_WORDS() -> Py { ensure_init(); NAMED_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_WORDS.write() { *w = v; } }
pub fn g_NAMES_IN_EMPHASIS() -> Py { ensure_init(); NAMES_IN_EMPHASIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_IN_EMPHASIS(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_IN_EMPHASIS.write() { *w = v; } }
pub fn g_NAME_STOP() -> Py { ensure_init(); NAME_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_STOP(v: Py) { ensure_init(); if let Ok(mut w) = NAME_STOP.write() { *w = v; } }
pub fn g_NAMING_STOP() -> Py { ensure_init(); NAMING_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMING_STOP(v: Py) { ensure_init(); if let Ok(mut w) = NAMING_STOP.write() { *w = v; } }
pub fn g_NEED_TO() -> Py { ensure_init(); NEED_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEED_TO(v: Py) { ensure_init(); if let Ok(mut w) = NEED_TO.write() { *w = v; } }
pub fn g_NEED_TO_ASKED() -> Py { ensure_init(); NEED_TO_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEED_TO_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = NEED_TO_ASKED.write() { *w = v; } }
pub fn g_NEED_TO_DO() -> Py { ensure_init(); NEED_TO_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEED_TO_DO(v: Py) { ensure_init(); if let Ok(mut w) = NEED_TO_DO.write() { *w = v; } }
pub fn g_NEGATED_AUX() -> Py { ensure_init(); NEGATED_AUX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATED_AUX(v: Py) { ensure_init(); if let Ok(mut w) = NEGATED_AUX.write() { *w = v; } }
pub fn g_NEGATES() -> Py { ensure_init(); NEGATES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATES(v: Py) { ensure_init(); if let Ok(mut w) = NEGATES.write() { *w = v; } }
pub fn g_NEVER() -> Py { ensure_init(); NEVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEVER(v: Py) { ensure_init(); if let Ok(mut w) = NEVER.write() { *w = v; } }
pub fn g_NEW_ASK() -> Py { ensure_init(); NEW_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEW_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NEW_ASK.write() { *w = v; } }
pub fn g_NEXT_IDEA() -> Py { ensure_init(); NEXT_IDEA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEXT_IDEA(v: Py) { ensure_init(); if let Ok(mut w) = NEXT_IDEA.write() { *w = v; } }
pub fn g_NOTHING_TO_GO_ON() -> Py { ensure_init(); NOTHING_TO_GO_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_TO_GO_ON(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_TO_GO_ON.write() { *w = v; } }
pub fn g_NOT_AN_OPENER() -> Py { ensure_init(); NOT_AN_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = NOT_AN_OPENER.write() { *w = v; } }
pub fn g_NOT_A_COMPARISON() -> Py { ensure_init(); NOT_A_COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_COMPARISON.write() { *w = v; } }
pub fn g_NOT_A_FIGURE() -> Py { ensure_init(); NOT_A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_FIGURE.write() { *w = v; } }
pub fn g_NOT_A_GIVING_THING() -> Py { ensure_init(); NOT_A_GIVING_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_GIVING_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_GIVING_THING.write() { *w = v; } }
pub fn g_NOT_A_LABEL() -> Py { ensure_init(); NOT_A_LABEL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_LABEL(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_LABEL.write() { *w = v; } }
pub fn g_NOT_A_PRICE_ASK() -> Py { ensure_init(); NOT_A_PRICE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PRICE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_PRICE_ASK.write() { *w = v; } }
pub fn g_NOT_A_STEP_BUT_REASSURANCE() -> Py { ensure_init(); NOT_A_STEP_BUT_REASSURANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STEP_BUT_REASSURANCE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_STEP_BUT_REASSURANCE.write() { *w = v; } }
pub fn g_NOT_A_SUBJECT() -> Py { ensure_init(); NOT_A_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_SUBJECT.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOT_A_VERB() -> Py { ensure_init(); NOT_A_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_VERB(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_VERB.write() { *w = v; } }
pub fn g_NOT_A_VERB_HEAD() -> Py { ensure_init(); NOT_A_VERB_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_VERB_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_VERB_HEAD.write() { *w = v; } }
pub fn g_NOT_A_YEAR() -> Py { ensure_init(); NOT_A_YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_YEAR.write() { *w = v; } }
pub fn g_NOT_KINDS_OF() -> Py { ensure_init(); NOT_KINDS_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_KINDS_OF(v: Py) { ensure_init(); if let Ok(mut w) = NOT_KINDS_OF.write() { *w = v; } }
pub fn g_NOT_STEPS() -> Py { ensure_init(); NOT_STEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_STEPS(v: Py) { ensure_init(); if let Ok(mut w) = NOT_STEPS.write() { *w = v; } }
pub fn g_NOT_WORDS_ANYWAY() -> Py { ensure_init(); NOT_WORDS_ANYWAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_WORDS_ANYWAY(v: Py) { ensure_init(); if let Ok(mut w) = NOT_WORDS_ANYWAY.write() { *w = v; } }
pub fn g_NOT_WORDS_ASK() -> Py { ensure_init(); NOT_WORDS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_WORDS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_WORDS_ASK.write() { *w = v; } }
pub fn g_NO_FIXED_TIME() -> Py { ensure_init(); NO_FIXED_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_FIXED_TIME(v: Py) { ensure_init(); if let Ok(mut w) = NO_FIXED_TIME.write() { *w = v; } }
pub fn g_NO_JOB() -> Py { ensure_init(); NO_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_JOB(v: Py) { ensure_init(); if let Ok(mut w) = NO_JOB.write() { *w = v; } }
pub fn g_ON_THE_CLOCK() -> Py { ensure_init(); ON_THE_CLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_CLOCK(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_CLOCK.write() { *w = v; } }
pub fn g_OPEN_WHO_HELP() -> Py { ensure_init(); OPEN_WHO_HELP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPEN_WHO_HELP(v: Py) { ensure_init(); if let Ok(mut w) = OPEN_WHO_HELP.write() { *w = v; } }
pub fn g_OPINION() -> Py { ensure_init(); OPINION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPINION(v: Py) { ensure_init(); if let Ok(mut w) = OPINION.write() { *w = v; } }
pub fn g_OPTIONS() -> Py { ensure_init(); OPTIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPTIONS(v: Py) { ensure_init(); if let Ok(mut w) = OPTIONS.write() { *w = v; } }
pub fn g_OPTIONS_ASK() -> Py { ensure_init(); OPTIONS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPTIONS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = OPTIONS_ASK.write() { *w = v; } }
pub fn g_ORIGIN_ASK() -> Py { ensure_init(); ORIGIN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORIGIN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ORIGIN_ASK.write() { *w = v; } }
pub fn g_ORIGIN_PERSON() -> Py { ensure_init(); ORIGIN_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORIGIN_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = ORIGIN_PERSON.write() { *w = v; } }
pub fn g_OTHER_QUESTION() -> Py { ensure_init(); OTHER_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_QUESTION.write() { *w = v; } }
pub fn g_OUTCOME_ASK() -> Py { ensure_init(); OUTCOME_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OUTCOME_ASK(v: Py) { ensure_init(); if let Ok(mut w) = OUTCOME_ASK.write() { *w = v; } }
pub fn g_PAIRING_ASK() -> Py { ensure_init(); PAIRING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAIRING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PAIRING_ASK.write() { *w = v; } }
pub fn g_PAST_DEFINES() -> Py { ensure_init(); PAST_DEFINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_DEFINES(v: Py) { ensure_init(); if let Ok(mut w) = PAST_DEFINES.write() { *w = v; } }
pub fn g_PATH_THEN_VERB() -> Py { ensure_init(); PATH_THEN_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PATH_THEN_VERB(v: Py) { ensure_init(); if let Ok(mut w) = PATH_THEN_VERB.write() { *w = v; } }
pub fn g_PERMISSION() -> Py { ensure_init(); PERMISSION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERMISSION(v: Py) { ensure_init(); if let Ok(mut w) = PERMISSION.write() { *w = v; } }
pub fn g_PERMISSION_RULE() -> Py { ensure_init(); PERMISSION_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERMISSION_RULE(v: Py) { ensure_init(); if let Ok(mut w) = PERMISSION_RULE.write() { *w = v; } }
pub fn g_PERSON() -> Py { ensure_init(); PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = PERSON.write() { *w = v; } }
pub fn g_PERSON_FULL_NAME() -> Py { ensure_init(); PERSON_FULL_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_FULL_NAME(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_FULL_NAME.write() { *w = v; } }
pub fn g_PERSON_NAME() -> Py { ensure_init(); PERSON_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_NAME(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_NAME.write() { *w = v; } }
pub fn g_PERSON_NAMED() -> Py { ensure_init(); PERSON_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_NAMED.write() { *w = v; } }
pub fn g_PERSON_RELATION() -> Py { ensure_init(); PERSON_RELATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_RELATION(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_RELATION.write() { *w = v; } }
pub fn g_PERSON_WORD() -> Py { ensure_init(); PERSON_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_WORD(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_WORD.write() { *w = v; } }
pub fn g_PICK() -> Py { ensure_init(); PICK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICK(v: Py) { ensure_init(); if let Ok(mut w) = PICK.write() { *w = v; } }
pub fn g_PICKS_BY_COMPARISON() -> Py { ensure_init(); PICKS_BY_COMPARISON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICKS_BY_COMPARISON(v: Py) { ensure_init(); if let Ok(mut w) = PICKS_BY_COMPARISON.write() { *w = v; } }
pub fn g_PICK_BY_CASE() -> Py { ensure_init(); PICK_BY_CASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICK_BY_CASE(v: Py) { ensure_init(); if let Ok(mut w) = PICK_BY_CASE.write() { *w = v; } }
pub fn g_PICK_ONE() -> Py { ensure_init(); PICK_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICK_ONE(v: Py) { ensure_init(); if let Ok(mut w) = PICK_ONE.write() { *w = v; } }
pub fn g_PICK_ONE_OF_A_KIND() -> Py { ensure_init(); PICK_ONE_OF_A_KIND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICK_ONE_OF_A_KIND(v: Py) { ensure_init(); if let Ok(mut w) = PICK_ONE_OF_A_KIND.write() { *w = v; } }
pub fn g_PICK_SIDES() -> Py { ensure_init(); PICK_SIDES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICK_SIDES(v: Py) { ensure_init(); if let Ok(mut w) = PICK_SIDES.write() { *w = v; } }
pub fn g_PLACE() -> Py { ensure_init(); PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = PLACE.write() { *w = v; } }
pub fn g_PLACE_AS_EVIDENCE() -> Py { ensure_init(); PLACE_AS_EVIDENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_AS_EVIDENCE(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_AS_EVIDENCE.write() { *w = v; } }
pub fn g_PLACE_AT_HOME() -> Py { ensure_init(); PLACE_AT_HOME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_AT_HOME(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_AT_HOME.write() { *w = v; } }
pub fn g_PLACE_KIND() -> Py { ensure_init(); PLACE_KIND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_KIND(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_KIND.write() { *w = v; } }
pub fn g_PLACE_NAME() -> Py { ensure_init(); PLACE_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_NAME(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_NAME.write() { *w = v; } }
pub fn g_PLACE_NAME_ANY() -> Py { ensure_init(); PLACE_NAME_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_NAME_ANY(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_NAME_ANY.write() { *w = v; } }
pub fn g_PLACE_WORD() -> Py { ensure_init(); PLACE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_WORD.write() { *w = v; } }
pub fn g_PLAN_VERB() -> Py { ensure_init(); PLAN_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_VERB(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_VERB.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_POINT_ASK() -> Py { ensure_init(); POINT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = POINT_ASK.write() { *w = v; } }
pub fn g_POINT_NOT_A_THING() -> Py { ensure_init(); POINT_NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINT_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = POINT_NOT_A_THING.write() { *w = v; } }
pub fn g_PREFERS() -> Py { ensure_init(); PREFERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREFERS(v: Py) { ensure_init(); if let Ok(mut w) = PREFERS.write() { *w = v; } }
pub fn g_PRICE_ADJECTIVE() -> Py { ensure_init(); PRICE_ADJECTIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_ADJECTIVE(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_ADJECTIVE.write() { *w = v; } }
pub fn g_PRICE_ADJECTIVE_KEEPS() -> Py { ensure_init(); PRICE_ADJECTIVE_KEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_ADJECTIVE_KEEPS(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_ADJECTIVE_KEEPS.write() { *w = v; } }
pub fn g_PRICE_ADJECTIVE_THE_ASK_IS_LATER() -> Py { ensure_init(); PRICE_ADJECTIVE_THE_ASK_IS_LATER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_ADJECTIVE_THE_ASK_IS_LATER(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_ADJECTIVE_THE_ASK_IS_LATER.write() { *w = v; } }
pub fn g_PRICE_ADJECTIVE_WHY() -> Py { ensure_init(); PRICE_ADJECTIVE_WHY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_ADJECTIVE_WHY(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_ADJECTIVE_WHY.write() { *w = v; } }
pub fn g_PRICE_ASKED() -> Py { ensure_init(); PRICE_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_ASKED.write() { *w = v; } }
pub fn g_PRICE_GIVEN() -> Py { ensure_init(); PRICE_GIVEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_GIVEN(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_GIVEN.write() { *w = v; } }
pub fn g_PRICE_IDIOM() -> Py { ensure_init(); PRICE_IDIOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_IDIOM(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_IDIOM.write() { *w = v; } }
pub fn g_PUNT() -> Py { ensure_init(); PUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUNT(v: Py) { ensure_init(); if let Ok(mut w) = PUNT.write() { *w = v; } }
pub fn g_PURPOSE() -> Py { ensure_init(); PURPOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURPOSE(v: Py) { ensure_init(); if let Ok(mut w) = PURPOSE.write() { *w = v; } }
pub fn g_PURPOSE_TO() -> Py { ensure_init(); PURPOSE_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURPOSE_TO(v: Py) { ensure_init(); if let Ok(mut w) = PURPOSE_TO.write() { *w = v; } }
pub fn g_QUALITY_IS() -> Py { ensure_init(); QUALITY_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUALITY_IS(v: Py) { ensure_init(); if let Ok(mut w) = QUALITY_IS.write() { *w = v; } }
pub fn g_QUESTION_IN_THE_LINE() -> Py { ensure_init(); QUESTION_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_IN_THE_LINE.write() { *w = v; } }
pub fn g_QUOTED_NAME() -> Py { ensure_init(); QUOTED_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUOTED_NAME(v: Py) { ensure_init(); if let Ok(mut w) = QUOTED_NAME.write() { *w = v; } }
pub fn g_RATE() -> Py { ensure_init(); RATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RATE(v: Py) { ensure_init(); if let Ok(mut w) = RATE.write() { *w = v; } }
pub fn g_RATE_OF_A_COUNT() -> Py { ensure_init(); RATE_OF_A_COUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RATE_OF_A_COUNT(v: Py) { ensure_init(); if let Ok(mut w) = RATE_OF_A_COUNT.write() { *w = v; } }
pub fn g_RATE_PHRASE() -> Py { ensure_init(); RATE_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RATE_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = RATE_PHRASE.write() { *w = v; } }
pub fn g_REASON() -> Py { ensure_init(); REASON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REASON(v: Py) { ensure_init(); if let Ok(mut w) = REASON.write() { *w = v; } }
pub fn g_REASON_ASK() -> Py { ensure_init(); REASON_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REASON_ASK(v: Py) { ensure_init(); if let Ok(mut w) = REASON_ASK.write() { *w = v; } }
pub fn g_REASON_OPENS() -> Py { ensure_init(); REASON_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REASON_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = REASON_OPENS.write() { *w = v; } }
pub fn g_RECALL_AMOUNT() -> Py { ensure_init(); RECALL_AMOUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_AMOUNT(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_AMOUNT.write() { *w = v; } }
pub fn g_RECALL_KIND_NOUN() -> Py { ensure_init(); RECALL_KIND_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_KIND_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_KIND_NOUN.write() { *w = v; } }
pub fn g_RECALL_KIND_NOUN_ASK() -> Py { ensure_init(); RECALL_KIND_NOUN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_KIND_NOUN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_KIND_NOUN_ASK.write() { *w = v; } }
pub fn g_RECALL_NOT() -> Py { ensure_init(); RECALL_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_NOT(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_NOT.write() { *w = v; } }
pub fn g_RECALL_OF_MINE() -> Py { ensure_init(); RECALL_OF_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_OF_MINE(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_OF_MINE.write() { *w = v; } }
pub fn g_RECALL_PERSON() -> Py { ensure_init(); RECALL_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_PERSON.write() { *w = v; } }
pub fn g_RECALL_PLACE() -> Py { ensure_init(); RECALL_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_PLACE.write() { *w = v; } }
pub fn g_RECALL_PRONOUN() -> Py { ensure_init(); RECALL_PRONOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_PRONOUN(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_PRONOUN.write() { *w = v; } }
pub fn g_RECALL_SAID() -> Py { ensure_init(); RECALL_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_SAID(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_SAID.write() { *w = v; } }
pub fn g_RECALL_SHAPE_BEFORE() -> Py { ensure_init(); RECALL_SHAPE_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_SHAPE_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_SHAPE_BEFORE.write() { *w = v; } }
pub fn g_RECALL_SHAPE_BEFORE_KIND() -> Py { ensure_init(); RECALL_SHAPE_BEFORE_KIND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_SHAPE_BEFORE_KIND(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_SHAPE_BEFORE_KIND.write() { *w = v; } }
pub fn g_RECALL_TIME() -> Py { ensure_init(); RECALL_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_TIME(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_TIME.write() { *w = v; } }
pub fn g_REFERS_BACK() -> Py { ensure_init(); REFERS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFERS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = REFERS_BACK.write() { *w = v; } }
pub fn g_REPEAT_LEAD() -> Py { ensure_init(); REPEAT_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPEAT_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = REPEAT_LEAD.write() { *w = v; } }
pub fn g_REPORTED() -> Py { ensure_init(); REPORTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPORTED(v: Py) { ensure_init(); if let Ok(mut w) = REPORTED.write() { *w = v; } }
pub fn g_RULE_CONDITION() -> Py { ensure_init(); RULE_CONDITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULE_CONDITION(v: Py) { ensure_init(); if let Ok(mut w) = RULE_CONDITION.write() { *w = v; } }
pub fn g_RULE_DROP_IT() -> Py { ensure_init(); RULE_DROP_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULE_DROP_IT(v: Py) { ensure_init(); if let Ok(mut w) = RULE_DROP_IT.write() { *w = v; } }
pub fn g_RULE_GO_NEW() -> Py { ensure_init(); RULE_GO_NEW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULE_GO_NEW(v: Py) { ensure_init(); if let Ok(mut w) = RULE_GO_NEW.write() { *w = v; } }
pub fn g_RULE_KEEP_IT() -> Py { ensure_init(); RULE_KEEP_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULE_KEEP_IT(v: Py) { ensure_init(); if let Ok(mut w) = RULE_KEEP_IT.write() { *w = v; } }
pub fn g_SAID_WORDS() -> Py { ensure_init(); SAID_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SAID_WORDS.write() { *w = v; } }
pub fn g_SAYS_NO() -> Py { ensure_init(); SAYS_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_NO(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_NO.write() { *w = v; } }
pub fn g_SAYS_NOTHING_ABOUT_IT() -> Py { ensure_init(); SAYS_NOTHING_ABOUT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_NOTHING_ABOUT_IT(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_NOTHING_ABOUT_IT.write() { *w = v; } }
pub fn g_SAYS_WHAT_IT_DOES() -> Py { ensure_init(); SAYS_WHAT_IT_DOES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_WHAT_IT_DOES(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_WHAT_IT_DOES.write() { *w = v; } }
pub fn g_SEASON() -> Py { ensure_init(); SEASON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEASON(v: Py) { ensure_init(); if let Ok(mut w) = SEASON.write() { *w = v; } }
pub fn g_SEASON_WHEN() -> Py { ensure_init(); SEASON_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEASON_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = SEASON_WHEN.write() { *w = v; } }
pub fn g_SEEKS_HELP() -> Py { ensure_init(); SEEKS_HELP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEEKS_HELP(v: Py) { ensure_init(); if let Ok(mut w) = SEEKS_HELP.write() { *w = v; } }
pub fn g_SENDS_THEM_ON() -> Py { ensure_init(); SENDS_THEM_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENDS_THEM_ON(v: Py) { ensure_init(); if let Ok(mut w) = SENDS_THEM_ON.write() { *w = v; } }
pub fn g_SENTENCE_END() -> Py { ensure_init(); SENTENCE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENTENCE_END(v: Py) { ensure_init(); if let Ok(mut w) = SENTENCE_END.write() { *w = v; } }
pub fn g_SEQUENCES() -> Py { ensure_init(); SEQUENCES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEQUENCES(v: Py) { ensure_init(); if let Ok(mut w) = SEQUENCES.write() { *w = v; } }
pub fn g_SETTLED_ON() -> Py { ensure_init(); SETTLED_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SETTLED_ON(v: Py) { ensure_init(); if let Ok(mut w) = SETTLED_ON.write() { *w = v; } }
pub fn g_SET_LABEL() -> Py { ensure_init(); SET_LABEL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SET_LABEL(v: Py) { ensure_init(); if let Ok(mut w) = SET_LABEL.write() { *w = v; } }
pub fn g_SIDES() -> Py { ensure_init(); SIDES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIDES(v: Py) { ensure_init(); if let Ok(mut w) = SIDES.write() { *w = v; } }
pub fn g_SIDES_VS() -> Py { ensure_init(); SIDES_VS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIDES_VS(v: Py) { ensure_init(); if let Ok(mut w) = SIDES_VS.write() { *w = v; } }
pub fn g_SIDE_FILLER() -> Py { ensure_init(); SIDE_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIDE_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = SIDE_FILLER.write() { *w = v; } }
pub fn g_SIDE_PAIRS() -> Py { ensure_init(); SIDE_PAIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIDE_PAIRS(v: Py) { ensure_init(); if let Ok(mut w) = SIDE_PAIRS.write() { *w = v; } }
pub fn g_SIGN_OFF_LEAD() -> Py { ensure_init(); SIGN_OFF_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIGN_OFF_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = SIGN_OFF_LEAD.write() { *w = v; } }
pub fn g_SOFT() -> Py { ensure_init(); SOFT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT(v: Py) { ensure_init(); if let Ok(mut w) = SOFT.write() { *w = v; } }
pub fn g_SOURCE() -> Py { ensure_init(); SOURCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOURCE(v: Py) { ensure_init(); if let Ok(mut w) = SOURCE.write() { *w = v; } }
pub fn g_SOURCE_COLON() -> Py { ensure_init(); SOURCE_COLON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOURCE_COLON(v: Py) { ensure_init(); if let Ok(mut w) = SOURCE_COLON.write() { *w = v; } }
pub fn g_SOURCE_DOES() -> Py { ensure_init(); SOURCE_DOES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOURCE_DOES(v: Py) { ensure_init(); if let Ok(mut w) = SOURCE_DOES.write() { *w = v; } }
pub fn g_SOURCE_LIST() -> Py { ensure_init(); SOURCE_LIST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOURCE_LIST(v: Py) { ensure_init(); if let Ok(mut w) = SOURCE_LIST.write() { *w = v; } }
pub fn g_SOURCE_MADE() -> Py { ensure_init(); SOURCE_MADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOURCE_MADE(v: Py) { ensure_init(); if let Ok(mut w) = SOURCE_MADE.write() { *w = v; } }
pub fn g_SOURCE_MAKES() -> Py { ensure_init(); SOURCE_MAKES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOURCE_MAKES(v: Py) { ensure_init(); if let Ok(mut w) = SOURCE_MAKES.write() { *w = v; } }
pub fn g_SPECIFIC_ASKS() -> Py { ensure_init(); SPECIFIC_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPECIFIC_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = SPECIFIC_ASKS.write() { *w = v; } }
pub fn g_SPOT_OF_THE_THING() -> Py { ensure_init(); SPOT_OF_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPOT_OF_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = SPOT_OF_THE_THING.write() { *w = v; } }
pub fn g_SPOT_ON_THE_THING() -> Py { ensure_init(); SPOT_ON_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPOT_ON_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = SPOT_ON_THE_THING.write() { *w = v; } }
pub fn g_SPOT_PART() -> Py { ensure_init(); SPOT_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPOT_PART(v: Py) { ensure_init(); if let Ok(mut w) = SPOT_PART.write() { *w = v; } }
pub fn g_STAGE() -> Py { ensure_init(); STAGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAGE(v: Py) { ensure_init(); if let Ok(mut w) = STAGE.write() { *w = v; } }
pub fn g_STAGE_WHEN() -> Py { ensure_init(); STAGE_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAGE_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = STAGE_WHEN.write() { *w = v; } }
pub fn g_STATES_HEAD() -> Py { ensure_init(); STATES_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATES_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = STATES_HEAD.write() { *w = v; } }
pub fn g_STEPS() -> Py { ensure_init(); STEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEPS(v: Py) { ensure_init(); if let Ok(mut w) = STEPS.write() { *w = v; } }
pub fn g_STEPS_ARE_COUNTED() -> Py { ensure_init(); STEPS_ARE_COUNTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEPS_ARE_COUNTED(v: Py) { ensure_init(); if let Ok(mut w) = STEPS_ARE_COUNTED.write() { *w = v; } }
pub fn g_STEPS_ASK() -> Py { ensure_init(); STEPS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEPS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = STEPS_ASK.write() { *w = v; } }
pub fn g_STEPS_AS_A_DISTANCE() -> Py { ensure_init(); STEPS_AS_A_DISTANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEPS_AS_A_DISTANCE(v: Py) { ensure_init(); if let Ok(mut w) = STEPS_AS_A_DISTANCE.write() { *w = v; } }
pub fn g_STEPS_INSIDE_A_LIST() -> Py { ensure_init(); STEPS_INSIDE_A_LIST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEPS_INSIDE_A_LIST(v: Py) { ensure_init(); if let Ok(mut w) = STEPS_INSIDE_A_LIST.write() { *w = v; } }
pub fn g_SUBSTITUTE() -> Py { ensure_init(); SUBSTITUTE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBSTITUTE(v: Py) { ensure_init(); if let Ok(mut w) = SUBSTITUTE.write() { *w = v; } }
pub fn g_SUM_ASKS() -> Py { ensure_init(); SUM_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUM_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = SUM_ASKS.write() { *w = v; } }
pub fn g_SUM_OUT_LOUD() -> Py { ensure_init(); SUM_OUT_LOUD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUM_OUT_LOUD(v: Py) { ensure_init(); if let Ok(mut w) = SUM_OUT_LOUD.write() { *w = v; } }
pub fn g_TAIL_VERDICT_ASK() -> Py { ensure_init(); TAIL_VERDICT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAIL_VERDICT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = TAIL_VERDICT_ASK.write() { *w = v; } }
pub fn g_TELLS_A_TEST() -> Py { ensure_init(); TELLS_A_TEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TELLS_A_TEST(v: Py) { ensure_init(); if let Ok(mut w) = TELLS_A_TEST.write() { *w = v; } }
pub fn g_TELL_ASK() -> Py { ensure_init(); TELL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TELL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = TELL_ASK.write() { *w = v; } }
pub fn g_TEST_IS_THE_VERDICT() -> Py { ensure_init(); TEST_IS_THE_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TEST_IS_THE_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = TEST_IS_THE_VERDICT.write() { *w = v; } }
pub fn g_THEIR_OWN() -> Py { ensure_init(); THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN.write() { *w = v; } }
pub fn g_THE_SIGN() -> Py { ensure_init(); THE_SIGN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_SIGN(v: Py) { ensure_init(); if let Ok(mut w) = THE_SIGN.write() { *w = v; } }
pub fn g_THE_THING_ASKED() -> Py { ensure_init(); THE_THING_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_THING_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = THE_THING_ASKED.write() { *w = v; } }
pub fn g_THE_WINNER_NAMED() -> Py { ensure_init(); THE_WINNER_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_WINNER_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = THE_WINNER_NAMED.write() { *w = v; } }
pub fn g_THINGS_VERB() -> Py { ensure_init(); THINGS_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THINGS_VERB(v: Py) { ensure_init(); if let Ok(mut w) = THINGS_VERB.write() { *w = v; } }
pub fn g_THING_THAT_EXTENDS() -> Py { ensure_init(); THING_THAT_EXTENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_THAT_EXTENDS(v: Py) { ensure_init(); if let Ok(mut w) = THING_THAT_EXTENDS.write() { *w = v; } }
pub fn g_TIME() -> Py { ensure_init(); TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME(v: Py) { ensure_init(); if let Ok(mut w) = TIME.write() { *w = v; } }
pub fn g_TIME_LEAD_IN() -> Py { ensure_init(); TIME_LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = TIME_LEAD_IN.write() { *w = v; } }
pub fn g_TRAILING_VERDICT() -> Py { ensure_init(); TRAILING_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRAILING_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = TRAILING_VERDICT.write() { *w = v; } }
pub fn g_UNSETTLED() -> Py { ensure_init(); UNSETTLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNSETTLED(v: Py) { ensure_init(); if let Ok(mut w) = UNSETTLED.write() { *w = v; } }
pub fn g_USUALLY_IS() -> Py { ensure_init(); USUALLY_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_USUALLY_IS(v: Py) { ensure_init(); if let Ok(mut w) = USUALLY_IS.write() { *w = v; } }
pub fn g_VERDICT_ABOUT_IT() -> Py { ensure_init(); VERDICT_ABOUT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ABOUT_IT(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_ABOUT_IT.write() { *w = v; } }
pub fn g_VERDICT_AFTER_A_CASE() -> Py { ensure_init(); VERDICT_AFTER_A_CASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_AFTER_A_CASE(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_AFTER_A_CASE.write() { *w = v; } }
pub fn g_VERDICT_AFTER_A_FIGURE() -> Py { ensure_init(); VERDICT_AFTER_A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_AFTER_A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_AFTER_A_FIGURE.write() { *w = v; } }
pub fn g_VERDICT_BAND() -> Py { ensure_init(); VERDICT_BAND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_BAND(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_BAND.write() { *w = v; } }
pub fn g_VERDICT_ECHO() -> Py { ensure_init(); VERDICT_ECHO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ECHO(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_ECHO.write() { *w = v; } }
pub fn g_VERDICT_FRAME() -> Py { ensure_init(); VERDICT_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_FRAME.write() { *w = v; } }
pub fn g_VERDICT_INSIDE() -> Py { ensure_init(); VERDICT_INSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_INSIDE(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_INSIDE.write() { *w = v; } }
pub fn g_VERDICT_INSIDE_QUALITY() -> Py { ensure_init(); VERDICT_INSIDE_QUALITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_INSIDE_QUALITY(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_INSIDE_QUALITY.write() { *w = v; } }
pub fn g_VERDICT_ON_A_SCALE() -> Py { ensure_init(); VERDICT_ON_A_SCALE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ON_A_SCALE(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_ON_A_SCALE.write() { *w = v; } }
pub fn g_VERDICT_OPEN() -> Py { ensure_init(); VERDICT_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_OPEN.write() { *w = v; } }
pub fn g_VERDICT_RULE() -> Py { ensure_init(); VERDICT_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_RULE(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_RULE.write() { *w = v; } }
pub fn g_WANTS() -> Py { ensure_init(); WANTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTS(v: Py) { ensure_init(); if let Ok(mut w) = WANTS.write() { *w = v; } }
pub fn g_WANTS_INFO() -> Py { ensure_init(); WANTS_INFO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTS_INFO(v: Py) { ensure_init(); if let Ok(mut w) = WANTS_INFO.write() { *w = v; } }
pub fn g_WANTS_OF_THE_PERSON() -> Py { ensure_init(); WANTS_OF_THE_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTS_OF_THE_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = WANTS_OF_THE_PERSON.write() { *w = v; } }
pub fn g_WANTS_THE_CAUSE() -> Py { ensure_init(); WANTS_THE_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTS_THE_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = WANTS_THE_CAUSE.write() { *w = v; } }
pub fn g_WHAT_AGE() -> Py { ensure_init(); WHAT_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_AGE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_AGE.write() { *w = v; } }
pub fn g_WHAT_ARE_MINE() -> Py { ensure_init(); WHAT_ARE_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_ARE_MINE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_ARE_MINE.write() { *w = v; } }
pub fn g_WHAT_ARE_TIMES() -> Py { ensure_init(); WHAT_ARE_TIMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_ARE_TIMES(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_ARE_TIMES.write() { *w = v; } }
pub fn g_WHAT_CAN_I_DO() -> Py { ensure_init(); WHAT_CAN_I_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_CAN_I_DO(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_CAN_I_DO.write() { *w = v; } }
pub fn g_WHAT_CAN_I_DO_WITH() -> Py { ensure_init(); WHAT_CAN_I_DO_WITH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_CAN_I_DO_WITH(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_CAN_I_DO_WITH.write() { *w = v; } }
pub fn g_WHAT_DOES_MEASURE() -> Py { ensure_init(); WHAT_DOES_MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_DOES_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_DOES_MEASURE.write() { *w = v; } }
pub fn g_WHAT_ELSE_STEPS() -> Py { ensure_init(); WHAT_ELSE_STEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_ELSE_STEPS(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_ELSE_STEPS.write() { *w = v; } }
pub fn g_WHAT_ELSE_WITH() -> Py { ensure_init(); WHAT_ELSE_WITH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_ELSE_WITH(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_ELSE_WITH.write() { *w = v; } }
pub fn g_WHAT_FIGURE() -> Py { ensure_init(); WHAT_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_FIGURE.write() { *w = v; } }
pub fn g_WHAT_HELPS() -> Py { ensure_init(); WHAT_HELPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_HELPS(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_HELPS.write() { *w = v; } }
pub fn g_WHAT_IS_PER() -> Py { ensure_init(); WHAT_IS_PER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_IS_PER(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_IS_PER.write() { *w = v; } }
pub fn g_WHAT_I_GROW() -> Py { ensure_init(); WHAT_I_GROW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_GROW(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_I_GROW.write() { *w = v; } }
pub fn g_WHAT_I_NEED() -> Py { ensure_init(); WHAT_I_NEED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_NEED(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_I_NEED.write() { *w = v; } }
pub fn g_WHAT_I_WRITE() -> Py { ensure_init(); WHAT_I_WRITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_WRITE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_I_WRITE.write() { *w = v; } }
pub fn g_WHAT_SHOULD_I_DO_WITH() -> Py { ensure_init(); WHAT_SHOULD_I_DO_WITH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_SHOULD_I_DO_WITH(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_SHOULD_I_DO_WITH.write() { *w = v; } }
pub fn g_WHAT_THING_FRAGMENT() -> Py { ensure_init(); WHAT_THING_FRAGMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_THING_FRAGMENT(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_THING_FRAGMENT.write() { *w = v; } }
pub fn g_WHAT_TIME_NAMED() -> Py { ensure_init(); WHAT_TIME_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TIME_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TIME_NAMED.write() { *w = v; } }
pub fn g_WHAT_TO_BAKE() -> Py { ensure_init(); WHAT_TO_BAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_BAKE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_BAKE.write() { *w = v; } }
pub fn g_WHAT_TO_DO() -> Py { ensure_init(); WHAT_TO_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_DO(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_DO.write() { *w = v; } }
pub fn g_WHAT_TO_OPENS() -> Py { ensure_init(); WHAT_TO_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_OPENS.write() { *w = v; } }
pub fn g_WHAT_TO_PICK() -> Py { ensure_init(); WHAT_TO_PICK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_PICK(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_PICK.write() { *w = v; } }
pub fn g_WHAT_TO_STEPS() -> Py { ensure_init(); WHAT_TO_STEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_STEPS(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_STEPS.write() { *w = v; } }
pub fn g_WHAT_YOU_DO() -> Py { ensure_init(); WHAT_YOU_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_YOU_DO(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_YOU_DO.write() { *w = v; } }
pub fn g_WHAT_YOU_TAKE() -> Py { ensure_init(); WHAT_YOU_TAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_YOU_TAKE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_YOU_TAKE.write() { *w = v; } }
pub fn g_WHEN_ASKS() -> Py { ensure_init(); WHEN_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_ASKS.write() { *w = v; } }
pub fn g_WHERE_ALONE() -> Py { ensure_init(); WHERE_ALONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_ALONE(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_ALONE.write() { *w = v; } }
pub fn g_WHERE_MONEY_GOES() -> Py { ensure_init(); WHERE_MONEY_GOES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_MONEY_GOES(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_MONEY_GOES.write() { *w = v; } }
pub fn g_WHERE_POINT() -> Py { ensure_init(); WHERE_POINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_POINT(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_POINT.write() { *w = v; } }
pub fn g_WHERE_RECALL() -> Py { ensure_init(); WHERE_RECALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_RECALL(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_RECALL.write() { *w = v; } }
pub fn g_WHERE_RELATIVE() -> Py { ensure_init(); WHERE_RELATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_RELATIVE(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_RELATIVE.write() { *w = v; } }
pub fn g_WHERE_START() -> Py { ensure_init(); WHERE_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_START(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_START.write() { *w = v; } }
pub fn g_WHICH_DAY_IS_BEST() -> Py { ensure_init(); WHICH_DAY_IS_BEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHICH_DAY_IS_BEST(v: Py) { ensure_init(); if let Ok(mut w) = WHICH_DAY_IS_BEST.write() { *w = v; } }
pub fn g_WHO_AND_MEASURE() -> Py { ensure_init(); WHO_AND_MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_AND_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = WHO_AND_MEASURE.write() { *w = v; } }
pub fn g_WHO_OF_A_NOUN() -> Py { ensure_init(); WHO_OF_A_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_OF_A_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = WHO_OF_A_NOUN.write() { *w = v; } }
pub fn g_WHO_OF_A_PRONOUN() -> Py { ensure_init(); WHO_OF_A_PRONOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_OF_A_PRONOUN(v: Py) { ensure_init(); if let Ok(mut w) = WHO_OF_A_PRONOUN.write() { *w = v; } }
pub fn g_WHO_OPENS() -> Py { ensure_init(); WHO_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = WHO_OPENS.write() { *w = v; } }
pub fn g_WORDS_ASK() -> Py { ensure_init(); WORDS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORDS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WORDS_ASK.write() { *w = v; } }
pub fn g_WOULD() -> Py { ensure_init(); WOULD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WOULD(v: Py) { ensure_init(); if let Ok(mut w) = WOULD.write() { *w = v; } }
pub fn g_WRITES_A_FORM() -> Py { ensure_init(); WRITES_A_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRITES_A_FORM(v: Py) { ensure_init(); if let Ok(mut w) = WRITES_A_FORM.write() { *w = v; } }
pub fn g_WRITES_IN_A_DOCUMENT() -> Py { ensure_init(); WRITES_IN_A_DOCUMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRITES_IN_A_DOCUMENT(v: Py) { ensure_init(); if let Ok(mut w) = WRITES_IN_A_DOCUMENT.write() { *w = v; } }
pub fn g_WRITES_TO_A_MESSAGE() -> Py { ensure_init(); WRITES_TO_A_MESSAGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRITES_TO_A_MESSAGE(v: Py) { ensure_init(); if let Ok(mut w) = WRITES_TO_A_MESSAGE.write() { *w = v; } }
pub fn g_WRITTEN_THING() -> Py { ensure_init(); WRITTEN_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRITTEN_THING(v: Py) { ensure_init(); if let Ok(mut w) = WRITTEN_THING.write() { *w = v; } }
pub fn g_WRONG_ASKED_ABOUT() -> Py { ensure_init(); WRONG_ASKED_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRONG_ASKED_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = WRONG_ASKED_ABOUT.write() { *w = v; } }
pub fn g_WRONG_RECALL() -> Py { ensure_init(); WRONG_RECALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRONG_RECALL(v: Py) { ensure_init(); if let Ok(mut w) = WRONG_RECALL.write() { *w = v; } }
pub fn g_YESNO() -> Py { ensure_init(); YESNO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YESNO(v: Py) { ensure_init(); if let Ok(mut w) = YESNO.write() { *w = v; } }
pub fn g__ADVISES_THE_THING() -> Py { ensure_init(); _ADVISES_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ADVISES_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _ADVISES_THE_THING.write() { *w = v; } }
pub fn g__AN_AGE_BARE() -> Py { ensure_init(); _AN_AGE_BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_BARE(v: Py) { ensure_init(); if let Ok(mut w) = _AN_AGE_BARE.write() { *w = v; } }
pub fn g__AN_AGE_BIRTHDAY() -> Py { ensure_init(); _AN_AGE_BIRTHDAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_BIRTHDAY(v: Py) { ensure_init(); if let Ok(mut w) = _AN_AGE_BIRTHDAY.write() { *w = v; } }
pub fn g__AN_AGE_MONTHS() -> Py { ensure_init(); _AN_AGE_MONTHS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_MONTHS(v: Py) { ensure_init(); if let Ok(mut w) = _AN_AGE_MONTHS.write() { *w = v; } }
pub fn g__AN_AGE_NAMED() -> Py { ensure_init(); _AN_AGE_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = _AN_AGE_NAMED.write() { *w = v; } }
pub fn g__AN_AGE_SPELLED_OPENS() -> Py { ensure_init(); _AN_AGE_SPELLED_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_SPELLED_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = _AN_AGE_SPELLED_OPENS.write() { *w = v; } }
pub fn g__AN_AGE_YEARS() -> Py { ensure_init(); _AN_AGE_YEARS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_YEARS(v: Py) { ensure_init(); if let Ok(mut w) = _AN_AGE_YEARS.write() { *w = v; } }
pub fn g__ARTICLE() -> Py { ensure_init(); _ARTICLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ARTICLE(v: Py) { ensure_init(); if let Ok(mut w) = _ARTICLE.write() { *w = v; } }
pub fn g__ASK_CLAUSE_OPENS() -> Py { ensure_init(); _ASK_CLAUSE_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_CLAUSE_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_CLAUSE_OPENS.write() { *w = v; } }
pub fn g__ASK_CLAUSE_SPLIT() -> Py { ensure_init(); _ASK_CLAUSE_SPLIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_CLAUSE_SPLIT(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_CLAUSE_SPLIT.write() { *w = v; } }
pub fn g__A_DOING() -> Py { ensure_init(); _A_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DOING(v: Py) { ensure_init(); if let Ok(mut w) = _A_DOING.write() { *w = v; } }
pub fn g__A_FIGURE_AGAINST_THE_STANDARD() -> Py { ensure_init(); _A_FIGURE_AGAINST_THE_STANDARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE_AGAINST_THE_STANDARD(v: Py) { ensure_init(); if let Ok(mut w) = _A_FIGURE_AGAINST_THE_STANDARD.write() { *w = v; } }
pub fn g__A_QUANTITY_NUMBER() -> Py { ensure_init(); _A_QUANTITY_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_QUANTITY_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = _A_QUANTITY_NUMBER.write() { *w = v; } }
pub fn g__A_QUANTITY_RELATION() -> Py { ensure_init(); _A_QUANTITY_RELATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_QUANTITY_RELATION(v: Py) { ensure_init(); if let Ok(mut w) = _A_QUANTITY_RELATION.write() { *w = v; } }
pub fn g__A_QUANTITY_UNIT() -> Py { ensure_init(); _A_QUANTITY_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_QUANTITY_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = _A_QUANTITY_UNIT.write() { *w = v; } }
pub fn g__A_QUOTE() -> Py { ensure_init(); _A_QUOTE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_QUOTE(v: Py) { ensure_init(); if let Ok(mut w) = _A_QUOTE.write() { *w = v; } }
pub fn g__A_SOCIAL_OPEN() -> Py { ensure_init(); _A_SOCIAL_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SOCIAL_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = _A_SOCIAL_OPEN.write() { *w = v; } }
pub fn g__CHECKS() -> Py { ensure_init(); _CHECKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHECKS(v: Py) { ensure_init(); if let Ok(mut w) = _CHECKS.write() { *w = v; } }
pub fn g__CLEAR() -> Py { ensure_init(); _CLEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CLEAR(v: Py) { ensure_init(); if let Ok(mut w) = _CLEAR.write() { *w = v; } }
pub fn g__COPULA() -> Py { ensure_init(); _COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COPULA(v: Py) { ensure_init(); if let Ok(mut w) = _COPULA.write() { *w = v; } }
pub fn g__DURATION_QUALIFIER() -> Py { ensure_init(); _DURATION_QUALIFIER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DURATION_QUALIFIER(v: Py) { ensure_init(); if let Ok(mut w) = _DURATION_QUALIFIER.write() { *w = v; } }
pub fn g__EMPTY() -> Py { ensure_init(); _EMPTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__EMPTY(v: Py) { ensure_init(); if let Ok(mut w) = _EMPTY.write() { *w = v; } }
pub fn g__ENDS_THE_THING() -> Py { ensure_init(); _ENDS_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ENDS_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _ENDS_THE_THING.write() { *w = v; } }
pub fn g__FIGURES() -> Py { ensure_init(); _FIGURES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FIGURES(v: Py) { ensure_init(); if let Ok(mut w) = _FIGURES.write() { *w = v; } }
pub fn g__FIGURE_HEAD() -> Py { ensure_init(); _FIGURE_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FIGURE_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = _FIGURE_HEAD.write() { *w = v; } }
pub fn g__HOUR_24() -> Py { ensure_init(); _HOUR_24.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HOUR_24(v: Py) { ensure_init(); if let Ok(mut w) = _HOUR_24.write() { *w = v; } }
pub fn g__JUDGES_THE_THING() -> Py { ensure_init(); _JUDGES_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__JUDGES_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _JUDGES_THE_THING.write() { *w = v; } }
pub fn g__LIST_OF_THINGS() -> Py { ensure_init(); _LIST_OF_THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LIST_OF_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = _LIST_OF_THINGS.write() { *w = v; } }
pub fn g__MEASURE_AFTER() -> Py { ensure_init(); _MEASURE_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MEASURE_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = _MEASURE_AFTER.write() { *w = v; } }
pub fn g__MEASURE_THEN_VERB() -> Py { ensure_init(); _MEASURE_THEN_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MEASURE_THEN_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _MEASURE_THEN_VERB.write() { *w = v; } }
pub fn g__MONEY() -> Py { ensure_init(); _MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MONEY(v: Py) { ensure_init(); if let Ok(mut w) = _MONEY.write() { *w = v; } }
pub fn g__NAMING_WORD() -> Py { ensure_init(); _NAMING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NAMING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _NAMING_WORD.write() { *w = v; } }
pub fn g__NOT_A_ROLE() -> Py { ensure_init(); _NOT_A_ROLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_ROLE(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_ROLE.write() { *w = v; } }
pub fn g__NOT_A_THING() -> Py { ensure_init(); _NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_THING.write() { *w = v; } }
pub fn g__NOT_A_WORD() -> Py { ensure_init(); _NOT_A_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_WORD.write() { *w = v; } }
pub fn g__NOT_THE_SUBJECT() -> Py { ensure_init(); _NOT_THE_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_THE_SUBJECT.write() { *w = v; } }
pub fn g__NO_WAY_TO_KNOW() -> Py { ensure_init(); _NO_WAY_TO_KNOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NO_WAY_TO_KNOW(v: Py) { ensure_init(); if let Ok(mut w) = _NO_WAY_TO_KNOW.write() { *w = v; } }
pub fn g__OPINION_OF_THE_THING() -> Py { ensure_init(); _OPINION_OF_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OPINION_OF_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _OPINION_OF_THE_THING.write() { *w = v; } }
pub fn g__PARTY() -> Py { ensure_init(); _PARTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PARTY(v: Py) { ensure_init(); if let Ok(mut w) = _PARTY.write() { *w = v; } }
pub fn g__PREFERS_STATED() -> Py { ensure_init(); _PREFERS_STATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PREFERS_STATED(v: Py) { ensure_init(); if let Ok(mut w) = _PREFERS_STATED.write() { *w = v; } }
pub fn g__PRONOUN_SUBJECT() -> Py { ensure_init(); _PRONOUN_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PRONOUN_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _PRONOUN_SUBJECT.write() { *w = v; } }
pub fn g__PUTS_THE_THING_TO_USE() -> Py { ensure_init(); _PUTS_THE_THING_TO_USE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PUTS_THE_THING_TO_USE(v: Py) { ensure_init(); if let Ok(mut w) = _PUTS_THE_THING_TO_USE.write() { *w = v; } }
pub fn g__QUALITIES() -> Py { ensure_init(); _QUALITIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__QUALITIES(v: Py) { ensure_init(); if let Ok(mut w) = _QUALITIES.write() { *w = v; } }
pub fn g__QUALITY_OF_A_THING() -> Py { ensure_init(); _QUALITY_OF_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__QUALITY_OF_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = _QUALITY_OF_A_THING.write() { *w = v; } }
pub fn g__QUALITY_SAID() -> Py { ensure_init(); _QUALITY_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__QUALITY_SAID(v: Py) { ensure_init(); if let Ok(mut w) = _QUALITY_SAID.write() { *w = v; } }
pub fn g__ROLE_ENDS() -> Py { ensure_init(); _ROLE_ENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ROLE_ENDS(v: Py) { ensure_init(); if let Ok(mut w) = _ROLE_ENDS.write() { *w = v; } }
pub fn g__ROLE_OFFICES() -> Py { ensure_init(); _ROLE_OFFICES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ROLE_OFFICES(v: Py) { ensure_init(); if let Ok(mut w) = _ROLE_OFFICES.write() { *w = v; } }
pub fn g__SAYS_WHAT_IT_IS() -> Py { ensure_init(); _SAYS_WHAT_IT_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAYS_WHAT_IT_IS(v: Py) { ensure_init(); if let Ok(mut w) = _SAYS_WHAT_IT_IS.write() { *w = v; } }
pub fn g__SIDE() -> Py { ensure_init(); _SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SIDE(v: Py) { ensure_init(); if let Ok(mut w) = _SIDE.write() { *w = v; } }
pub fn g__SIDE_EMPTY() -> Py { ensure_init(); _SIDE_EMPTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SIDE_EMPTY(v: Py) { ensure_init(); if let Ok(mut w) = _SIDE_EMPTY.write() { *w = v; } }
pub fn g__STEPS_IN() -> Py { ensure_init(); _STEPS_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STEPS_IN(v: Py) { ensure_init(); if let Ok(mut w) = _STEPS_IN.write() { *w = v; } }
pub fn g__TALKING_ABOUT_A_LENGTH() -> Py { ensure_init(); _TALKING_ABOUT_A_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TALKING_ABOUT_A_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = _TALKING_ABOUT_A_LENGTH.write() { *w = v; } }
pub fn g__THE() -> Py { ensure_init(); _THE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE(v: Py) { ensure_init(); if let Ok(mut w) = _THE.write() { *w = v; } }
pub fn g__THE_VERDICT_IS_EARLY() -> Py { ensure_init(); _THE_VERDICT_IS_EARLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_VERDICT_IS_EARLY(v: Py) { ensure_init(); if let Ok(mut w) = _THE_VERDICT_IS_EARLY.write() { *w = v; } }
pub fn g__TRAILING_MEASURE() -> Py { ensure_init(); _TRAILING_MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TRAILING_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = _TRAILING_MEASURE.write() { *w = v; } }
pub fn g__WORD() -> Py { ensure_init(); _WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WORD(v: Py) { ensure_init(); if let Ok(mut w) = _WORD.write() { *w = v; } }
pub fn g__WORD_NUMBER() -> Py { ensure_init(); _WORD_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WORD_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = _WORD_NUMBER.write() { *w = v; } }
pub fn g__WORKING() -> Py { ensure_init(); _WORKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WORKING(v: Py) { ensure_init(); if let Ok(mut w) = _WORKING.write() { *w = v; } }
pub fn g__gives_as_read() -> Py { ensure_init(); _gives_as_read.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__gives_as_read(v: Py) { ensure_init(); if let Ok(mut w) = _gives_as_read.write() { *w = v; } }
pub fn g__steps_by_the_verb_list() -> Py { ensure_init(); _steps_by_the_verb_list.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__steps_by_the_verb_list(v: Py) { ensure_init(); if let Ok(mut w) = _steps_by_the_verb_list.write() { *w = v; } }
fn module_body() {
    let _ = py_str("What kind of answer a line wants, and whether a reply gives it.\n\nThe understanding engine says what a line is about and which question words it carries\n(`memory[\"understood\"][\"asks\"]`). It does not say what shape of answer the person is owed: a\n`how_long` line wants a time, a `where` line a place, a `which` line a pick, a bare `yesno` a\nverdict. This module reads that shape from the whole line, and checks a reply for it.\n\nThe reading runs once per turn in `bot/modules/intent.py`, which writes\n`memory[\"understood\"][\"wants\"]` before any other observer runs. The check is for the composer and\nfor `tools/intent.py`: a reply on the right topic that does not answer the question - the story of\nwhat rice is for, for \"how long does rice take\" - is a miss even though it reads well.\n\nThe labels are the lane's fixed list, and a module may return None rather than guess:\n\n    time      a duration or a clock time ........ \"about 12 minutes\", \"by Friday\", \"1 to 1.5 bar\"\n    amount    a quantity, price or sum .......... \"about 2 kilos\", \"£3.50\", \"21 ÷ 3 = 7\"\n    place     somewhere to go ................... \"the post office\", \"Antarctica\"\n    person    who to ask ........................ \"the vet\", \"your GP\", \"Napoleon\"\n    meaning   what the thing is ................. \"An APR is the yearly cost...\"\n    yesno     a yes or a no ..................... \"Yes\", \"Straight in is fine\", \"No need\"\n    pick      which of two or more .............. \"Replace it\", \"Oat milk for coffee\"\n    steps     what to do, in order .............. \"First..., then...\"\n    options   a few ideas ....................... \"one, or two, or three\" (not one idea)\n    reason    why it happens .................... \"because...\", \"that's why...\"\n    opinion   what the bot makes of it .......... not checkable by words\n    advice    what the person might do .......... not checkable by words\n    comfort   an answer to something hard ....... not checkable by words\n    answer    anything else ..................... not checkable by words\n\n`wants` reads the shape. `gives` says True when a reply's opening sentence gives that shape,\nFalse when it can say what the reply gives instead (a shrug, or an unmistakable other shape), and\nNone where words cannot tell - the soft labels above, and an opening the cue list has nothing\nfor. It is a finder, not a mark: \"eight to ten years\" is a time without a digit and \"straight in\nis fine\" is a yes without the word yes, so every check is written to be right more often than it\nis wrong and to say nothing where it cannot be sure.\n");
    s_TIME(py_str("time"));
    s_AMOUNT(py_str("amount"));
    s_PLACE(py_str("place"));
    s_PERSON(py_str("person"));
    s_MEANING(py_str("meaning"));
    s_YESNO(py_str("yesno"));
    s_PICK(py_str("pick"));
    s_COMPARE(py_str("compare"));
    s_STEPS(py_str("steps"));
    s_OPTIONS(py_str("options"));
    s_REASON(py_str("reason"));
    s_OPINION(py_str("opinion"));
    s_ADVICE(py_str("advice"));
    s_COMFORT(py_str("comfort"));
    s_ANSWER(py_str("answer"));
    s_WANTS(py_tuple(vec![g_TIME(), g_AMOUNT(), g_PLACE(), g_PERSON(), g_MEANING(), g_YESNO(), g_PICK(), g_COMPARE(), g_STEPS(), g_OPTIONS(), g_REASON(), g_OPINION(), g_ADVICE(), g_COMFORT(), g_ANSWER()]));
    s_SOFT(call_builtin("frozenset", &[py_tuple(vec![g_OPINION(), g_ADVICE(), g_COMFORT(), g_ANSWER()])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED_FOR(py_tuple(vec![py_tuple(vec![g_PICK(), call_builtin("frozenset", &[py_tuple(vec![py_str("choice"), py_str("compare"), py_str("which")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_MEANING(), call_builtin("frozenset", &[py_tuple(vec![py_str("definition")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_PLACE(), call_builtin("frozenset", &[py_tuple(vec![py_str("where")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_PERSON(), call_builtin("frozenset", &[py_tuple(vec![py_str("who")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_REASON(), call_builtin("frozenset", &[py_tuple(vec![py_str("why")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_TIME(), call_builtin("frozenset", &[py_tuple(vec![py_str("how_long")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_AMOUNT(), call_builtin("frozenset", &[py_tuple(vec![py_str("how_much"), py_str("how_often")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_AMOUNT(), call_builtin("frozenset", &[py_tuple(vec![py_str("maths")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_OPTIONS(), call_builtin("frozenset", &[py_tuple(vec![py_str("list")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_STEPS(), call_builtin("frozenset", &[py_tuple(vec![py_str("steps"), py_str("how")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_PICK(), call_builtin("frozenset", &[py_tuple(vec![py_str("recommend")])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![g_YESNO(), call_builtin("frozenset", &[py_tuple(vec![py_str("yesno")])]).unwrap_or_else(|e| exc::raise(e))])]));
    s_A_RULE_THAT_SETTLES_IT(stdlib_call("re", "compile", &[py_str("\\b(?:if|when|whenever|whether|once|until|unless|as long as)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_OFFER_NOT_A_TEST(stdlib_call("re", "compile", &[py_str("\\b(?:here|happy|around|about|available|back|on hand)\\s+if\\b|\\b(?:say|tell me|tell us|let me know|ask|shout|message|write|reply|try)\\b[^.]{0,24}?\\bif\\b|\\b(?:what|as)\\s+if\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STANDING_AFTER_THE_TEST(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|means|counts as|needs?|should|worth|too|enough|fine|ok|okay|safe|normal|risky|a problem|gets?|lasts?|keeps?|stays?|throw|bin|toss|see (?:a|the|your)|ring|call|matters|turns on|decides|depends)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEW_THINGS(stdlib_call("re", "compile", &[py_str("\\b(?:tips?|ideas?|suggestions?|recommendations?|pointers?|hacks?|options)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PICK_SIDES(call_builtin("frozenset", &[py_tuple(vec![py_str("choice"), py_str("compare"), py_str("which")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DIFFERENCE(stdlib_call("re", "compile", &[py_str("\\bwhat(?:['\\u2019]?s| is| are| was| were)?\\b(?:\\s+[\\w'-]+){0,3}\\s+differences?\\b|\\bhow (?:is|are|does)\\b[^.?!]{0,40}\\b(?:different|differ)\\b\\s*(?:from|to)\\b|^[\\W]*(?:the )?differences? between\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPECIFIC_ASKS(call_builtin("frozenset", &[py_tuple(vec![py_str("choice"), py_str("compare"), py_str("which"), py_str("definition"), py_str("where"), py_str("who"), py_str("why"), py_str("how_long"), py_str("how_much"), py_str("how_often"), py_str("list"), py_str("steps")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KINDS_OF(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:kinds?|types?|sorts?|varieties|styles|brands|makes|models)(?: of (?:[\\w'-]+ ){0,4})?(?:are there|are available|is there|exist|do you|would you|can|could)\\b|\\bwhat (?:are|were) the (?:kinds?|types?|sorts?|varieties|styles)\\b|\\bwhat (?:are|were) (?:my|the|our) (?:options|choices)\\b|\\bwhat (?:options|choices) (?:do i|do we|have i|are there)\\b|\\b(?:any|other|more) (?:options|choices|alternatives)\\b|\\bwhat else (?:is|are) (?:there|out there)\\b|\\bwhat (?:is|are) available\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_KINDS_OF(stdlib_call("re", "compile", &[py_str("\\bwhat (?:kind|type|sort) of (?:a )?(?:[\\w'-]+ ){0,3}(?:is|was|are|were) (?:that|this|it|those|these|he|she)\\b|\\bwhat (?:kind|type|sort) of (?:[\\w'-]+ ){0,3}(?:would|will|should|could|might|suits?|fits?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEPS_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:are|is) the steps\\b|\\bsteps (?:to|for|in)\\b|\\bstep[- ]by[- ]step\\b|\\bwalk me through\\b|\\bwhat(?:'s| is) the process\\b|\\bthe process (?:for|of|to)\\b|\\bhow do i go about\\b|\\bwhat order\\b|\\bin what order\\b|\\b(?:the )?(?:best|right|correct|proper) order (?:to|for|in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTED_STEPS(stdlib_call("re", "compile", &[py_str("\\bhow many steps\\b|\\bsteps? per\\b|\\bsteps? in (?:a|one) (?:mile|km|day)\\b|\\bstep count\\b|\\b\\d{3,} steps\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEMBERS_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:are|were) the (?:early |first |main |usual |common |classic |key |warning |typical |initial )?(?:signs?|symptoms?|stages?|phases?)\\b|\\bwhat (?:signs?|symptoms?|stages?)\\b[^.?!]{0,30}\\b(?:should|would|do|can|might) (?:i|you|we) (?:look|watch|expect|see)\\b|\\bwhat are the (?:warning|telltale) signs\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARISON(stdlib_call("re", "compile", &[py_str("\\bcompare[ds]?\\b|\\bcomparing\\b|\\bvs\\.?(?=\\s|$)|\\bversus\\b|\\bdifferences? between\\b|\\bhow (?:is|are|does|do|did) [^.?!]{0,40}\\b(?:different|differ|compare)\\b|\\bwhat sets? [^.?!]{0,30}\\bapart\\b|\\bsets? [^.?!]{0,30} apart\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARED(stdlib_call("re", "compile", &[py_str("\\b(?:better|worse|cheaper|dearer|quieter|louder|faster|slower|easier|harder|healthier|safer|simpler|tastier|stronger) than\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_COMPARISON(stdlib_call("re", "compile", &[py_str("\\b(?:should i|shall i|worth (?:it|doing|the)|for me|which should i|which one should i|is it better to|would you)\\b|\\b(?:i|we) (?:always |still |keep |often |just )?compare\\b|\\bhow (?:do|can|should|would|could) (?:i|we)\\b|\\b(?:better|worse) than (?:i|we|you|they|she|he) (?:expected|thought|hoped|feared|looked)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SIDE(py_str("([\\w'-]+(?: (?!and\\b|from\\b|than\\b|vs\\b|versus\\b)[\\w'-]+)?)"));
    s__THE(py_str("(?:a |an |the )?"));
    s_SIDE_PAIRS(py_tuple(vec![stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&py_str("\\bbetween "), &g__THE()), &g__SIDE()), &py_str(" and ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&py_str("\\bcompare[sd]? "), &g__THE()), &g__SIDE()), &py_str(" (?:and|with|to) ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[add(&add(&add(&g__SIDE(), &py_str(" apart from ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[add(&add(&add(&g__SIDE(), &py_str(" (?:vs\\.?|versus) ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[add(&add(&add(&g__SIDE(), &py_str(" compar(?:e|ed) (?:with|to) ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[add(&add(&add(&g__SIDE(), &py_str(" (?:is |are |was |were )?(?:comparable to) ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[add(&add(&add(&g__SIDE(), &py_str(" (?:is |are |was |were )?(?:different|differs?) (?:from|to) ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[add(&add(&add(&g__SIDE(), &py_str(" (?:better|worse|cheaper|dearer|quieter|louder|faster|slower|easier|harder|healthier|safer|simpler|tastier|stronger) than ")), &g__THE()), &g__SIDE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_EXPLICIT_COMPARE(stdlib_call("re", "compile", &[py_str("\\bcompare[ds]?\\b|\\bcomparing\\b|\\bvs\\.?(?=\\s|$)|\\bversus\\b|\\bdifferences? between\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OPEN_WHO_HELP(stdlib_call("re", "compile", &[py_str("^\\W*who\\b[^.!?]{0,40}\\bhelp\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ME(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would|will) you help me\\b|\\bhelp me (?:with|to|out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIX_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what(?:'?s| is| was)?|any|the|a)\\s+(?:quick |easy |simple |cheap |good |real )?(?:fix|cure|remedy)\\s+(?:for|to)\\b|\\b(?:fix|cure|remedy) for (?:my|the|a|an|this|that)\\b|\\b(?:what(?:'?s| is| was)|any)\\s+(?:the\\s+)?(?:quick |easy |simple |cheap |good |real )?(?:fix|cure|remedy)\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK_ANY(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:hi|hey|hello|ok|okay|so|right|please|sorry|and)\\b[ ,]*)*(?:(?:can|could|would|will) you\\s+)?(?:help (?:me|us)|i need help|i could do with (?:some )?help|give me a hand)\\b(?!\\s+(?:choose|choosing|pick|picking|decide|deciding|select|selecting|compare|comparing|name|naming|word|wording|come up with)\\b|\\s+between\\b|\\s+which\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK_NO_TASK(stdlib_call("re", "compile", &[py_str("^[\\s,.;:!?\\-]*(?:(?:with|about|on)\\s+(?:something|anything|stuff|it|this|that)\\b)?[\\s,.;:!?\\-]*$|\\b(?:dont|don'?t|do not) know (?:what|where|how)\\b|\\bnot sure what\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:help(?: me| us)?|i need help|i could do with (?:some )?help|give me a hand)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_I(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|could|should|would) i\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARE_MYSELF(stdlib_call("re", "compile", &[py_str("\\bcompar(?:e|ing)\\b[^.?!]{0,14}\\b(?:myself|ourselves)\\b|\\bcompare\\s+(?:to|with)\\s+(?:me|my|us|our|him|her|them|others)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_SOON(stdlib_call("re", "compile", &[py_str("\\bhow (?:soon|early|quickly|long)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_METHOD_OPENS(stdlib_call("re", "compile", &[py_str("^how (?:do|does|did) (?:i|we|you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MEASURE(stdlib_call("re", "compile", &[py_str("\\bhow (?:tall|old|big|far|heavy|wide|deep|high|fast|hot|cold|warm|hard|loud|cheap|expensive|thick|thin|narrow|broad|shallow|steep|fine|coarse|tight|loose|many|much|often)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MEASURE_HEAD(stdlib_call("re", "compile", &[py_str("^how (?:tall|old|big|far|heavy|wide|deep|high|fast|hot|cold|warm|hard|loud|cheap|expensive|thick|thin|ripe|tight|loose|full|low|slow|quick|short|small|large|near|soft|firm|dark|bright|sharp|rough|smooth|fine|coarse|steep|cool|fresh|dry|wet|salty|sweet|spicy|sour|mild|strong|weak|light|stiff)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_DOES_MEASURE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:does|do)\\b(?:(?!\\bwhat\\b)[^?]){0,30}?\\b(?:cost|costs|charge|charges|weigh|weighs)\\b|\\bwhat (?:does|do)\\b(?:(?!\\bwhat\\b)[^?]){0,30}?\\bcome(?:s)? (?:to|out at|out as)\\b|\\bwhat (?:do|does|would|will|should|can|could|did) .{0,40}?\\b(?:charge|charges|cost|costs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_IDIOM(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s| is| was)?\\s+the (?:going|current|average|typical|usual) rate\\b|\\bset (?:me|us|you) back\\b|\\bexpect(?:ing)? to (?:pay|spend|be paying)\\b|\\b(?:usually|normally|typically|roughly|about) run(?:s)? to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COSTS_NOTHING(stdlib_call("re", "compile", &[py_str("\\bcosts? (?:nothing|free|nowt)\\b|\\bfor free\\b|\\bthat (?:is|are|will be) free\\b|\\b(?:doesn'?t|does not|didn'?t|did not|don'?t|do not|won'?t|will not) costs?\\b|\\bcosts? (?:much|a lot|more|less|too much)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONVERSION(stdlib_call("re", "compile", &[py_str("\\bhow long (?:is|are|was)\\b[^?]{0,30}?\\b(?:in|to|as|into) (?:feet|foot|metres?|meters?|miles?|kilometres?|kilometers?|km|yards?|inches|inch|cm|mm)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONVERTED_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:what(?:'s| is|s)|whats)\\s*\\d+(?:\\.\\d+)?\\s*(?:c|f|degrees?|deg|gas|ml|l|g|kg|oz|lb|lbs|cm|mm|in|inches|pints?|litres?|liters?|ft|feet|foot|metres?|meters?|m|miles?|mi|yards?|yd|km|stone|st)\\b[^?]{0,25}?\\b(?:in|to|into|as)\\b|\\bconvert\\s+\\d+(?:\\.\\d+)?\\s*(?:c|f|degrees?|deg|ml|g|kg|oz|lb|lbs|cm|mm|in)\\b[^?]{0,20}?\\b(?:in|to|into)\\b|\\bwhat(?:'s| is|s)?\\s*\\d+(?:\\.\\d+)?\\s+\\w+(?:\\s+\\w+)?\\s+(?:in|to)\\s+(?:grams?|g|kg|ml|millilitres?|milliliters?|litres?|liters?|ounces?|oz|pounds?|lb|cups?|tablespoons?|teaspoons?|degrees?|c|f)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADING_JOIN(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|but|then|also|ok|okay|right|now|well)\\b[ ,]*)+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_ASKS(stdlib_call("re", "compile", &[py_str("^(?:when'?s|when|what time|what day|what date|what month|what year|which day|what weekday) (?:should|shall|is|are|was|were|will|would|does|do|did|can|could|to|am|has|have|of|had)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_WHEN(stdlib_call("re", "compile", &[py_str("^[\\W]*(?:and |so |but |then )?(?:when|what time|what day|what date|what month|what year|which day)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHICH_DAY_IS_BEST(stdlib_call("re", "compile", &[py_str("\\b(?:best|better|rather|prefer|do you think|recommend)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_THING(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s| is| are)\\s+(?:a|an|the|some)?\\s*(?:really |actually |bit of a )?(?:good|great|nice|decent|cheap|budget|quick|easy|simple|sensible|healthy)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_WAY(stdlib_call("re", "compile", &[py_str("\\b(?:way|method|approach|routine)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_FIGURE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:time|timeframe|rate|age|price|cost|number|amount|weight|height|length|salary|wage|dose|dosage|percentage|temperature|speed|distance|size|pressure|humidity|mileage|voltage|rent)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_ARE_TIMES(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:so|and|ok|okay|oh|also|right)\\s+)*what(?:'s| is| are|s)\\b[^?]{0,60}?\\b(?:opening|visiting|closing|trading|check ?in|check ?out|business|office|term|school|library|swimming|bus|train|ferry|flight|shop|bank|tip|pool|gym|leisure|surgery|pharmacy|restaurant|pub|museum) ?(?:times|hours|dates)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_FRAME(stdlib_call("re", "compile", &[py_str("^\\W*(?:is|was|are|were|does|do|did)\\s+(?:that|this|it|these|those|they)\\s+(?:a|an|the|too|really|actually|still|about|much)?\\s*(?:good|normal|average|typical|usual|fair|ok|okay|alright|right|fine)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:good|average|normal|typical|usual|best) (?:\\w+ ){0,3}(?:time|age|price|number|amount|cost|salary|wage|rate|temperature|weight|height|speed|distance|size|dose|percentage)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGED_MEASURE(stdlib_call("re", "compile", &[py_str("\\b(?:good|average|normal|typical|usual|best|safe|right|ideal|healthy|cheap|correct|recommended|sensible|maximum|minimum|perfect|decent|acceptable)\\s+(?:\\w+\\s+){0,3}(?:rate|age|price|cost|number|amount|weight|height|length|salary|wage|dose|dosage|percentage|temperature|speed|distance|size|pressure|humidity|mileage|rent|voltage)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_THE_CLOCK(stdlib_call("re", "compile", &[py_str("\\b(?:time|times|hour|hours|minute|minutes)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_YES_OR_NO(stdlib_call("re", "compile", &[py_str("^\\W*(?:is|are|was|were|am|do|does|did|should|shall|can|could|would|will|may|might|have|has|had)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOOD_TIME(stdlib_call("re", "compile", &[py_str("(?:^|[,;]\\s*|\\b(?:and|so|but|then|also)\\s+)(?:is|are|was|were|would|could|will|do you think it is|do you think its)\\b[^.?!]{0,60}?\\b(?:a|an)\\s+(?:good|bad|better|best|ok|okay|fine|safe|alright|decent|terrible|awful|busy|quiet)\\s+time\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LONG_TIME(stdlib_call("re", "compile", &[py_str("\\blong time (?:no (?:see|speak|hear|talk)\\b|since\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_IT_WORKS(stdlib_call("re", "compile", &[py_str("\\bhow (?:does|do|did) (?:(?!\\b(?:i|you|we|im|me)\\b)[\\w']+ ){1,4}(?:work|works|working)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WRITTEN_THING(stdlib_call("re", "compile", &[py_str("\\bhow long (?:should |would |will |does |do |can )?(?:a|an|my|our|your|the|this|that)\\s+(?:[a-z]+\\s+){0,2}(?:personal statement|statement|essay|dissertation|thesis|coursework|assignment|cover(?:ing)? letter|letter|cv|resume|blog post|post|article|abstract|summary|report|review|answer|paragraphs?|chapter|book|story|speech|presentation|email|text|message|comment|description|bio|application|proposal|synopsis|blurb|poem|script|personal profile|profile)\\s+be\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_THAT_EXTENDS(stdlib_call("re", "compile", &[py_str("\\bhow long (?:is|are|was|were|will be|would be) (?:(?:the|a|an|my|our|your|this|that)\\s+)?(?:[a-z'-]+\\s+){0,2}?(?:rivers?|nile|amazon|thames|severn|danube|rhine|mississippi|walls?|pitch(?:es)?|playing fields?|marathons?|bridges?|tunnels?|canals?|motorways?|roads?|runways?|coastlines?|coast|borders?|beaches?|whales?|snakes?|crocodiles?|sharks?|string|ropes?|hose|cables?|pools?|tracks?|corridors?|driveways?)\\b(?=\\s*(?:[.?!,;:]|$|\\s+(?:of|in|on|at|from|to|by)\\b))(?!\\s+(?:pregnant|old|big|tall|wide|deep|long|away|till|until|now)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_TO_DO(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|could|should|would|will) (?:i|we|you) (?:explain|answer|say|tell|word|respond|reply|put it|break it|approach|handle)\\b|\\bhow to (?:explain|answer|say|tell|word|respond|reply|approach|handle)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_ARE_YOU(stdlib_call("re", "compile", &[py_str("^(?:(?:hey|hi|hiya|heya|hello|yo)(?: there)?[ ,]+)?how(?:'?s| is| are)?[ ,]+(?:you|u|ya|things|tricks|life|are you|are things|is it going|s it going|have you been|are we|your (?:day|week|evening|morning|night|head|family))\\b(?!\\s+(?:work|works|worked|working|go|goes|went|change|changes|changed|happen|happens|happened|fit|fits|seem|seems)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_IT_WENT(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:did|does|was|were|is|are|'s)\\s+(?:(?!\\b(?:i|you|we|me|us)\\b)\\w+\\s+){0,4}(?:go|goes|went|get on|got on|getting on|get along|turn out|turned out|work out|worked out)\\b(?!\\s+\\w)|\\bhow (?:it|that|this|things|everything) (?:goes|went|go|turns out|turned out)\\b(?!\\s+\\w)|\\bhow (?:was|were) (?:it|that|this|things|everything)\\b(?!\\s+\\w)|\\bhow goes it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_IT_HAPPENED(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:did|does|do)\\s+(?:it|that|this)\\s+(?:happen|happens|happened|come about|came about)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_OUT_LOUD(stdlib_call("re", "compile", &[py_str("^(?:what|which|who|whose|when|where|why|how|is|are|was|were|do|does|did|can|could|should|shall|would|will|any|anything|anyone|recommend|suggest|name|give me|tell me|got any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COST_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:does|do|will|would|did|is it going to) .{0,40}\\bcosts?\\b|\\bwhat .{0,40}\\bcosts?\\b|\\b(?:price|cost|charge|fee)s? (?:of|for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_IS_PER(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s| is)\\b[^?]{0,40}\\b(?:a|an|per|each|every) (?:month|week|year|day|hour|litre|liter|kg|kilo|pint|mile|gb|gigabyte|night|visit|session|person|head|class|lesson)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COST_STANDS_IN(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("definition"), py_str("list"), py_str("opinion"), py_str("recommend"), py_str("question")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE_OF_A_MONEY_VERB(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:should|shall|do|does|would|can|could|will)\\s+(?:i|we|you|they|he|she)\\s+(?:be\\s+)?(?:pay|paying|spend|spending|earn|earning|budget)\\b(?!\\s+off\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_VERB_NOT_A_PRICE(stdlib_call("re", "compile", &[py_str("\\bpay(?:ing)?\\s+attention\\b|\\b(?:pay|paying|spend|spending)\\s+(?:it|them|this|that|the\\s+money|the\\s+rest|the\\s+lot)\\b|\\b(?:pay|paying)\\s+off\\b|\\bspend(?:ing)?\\s+(?:(?:my|our|your|the)\\s+)?(?:time|attention|effort|trouble|energy|evening|day|weekend|morning|afternoon|night|holiday|summer|hours|years|life)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUM_OUT_LOUD(stdlib_call("re", "compile", &[py_str("^(?:subtract|add|multiply|divide|solve|factorise|factorize|work out|convert|split|minus|plus|double|halve|times)\\b|\\bhow much is\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUM_ASKS(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|half|quarter|third|two thirds|three quarters|a few)\\b.{0,30}\\bin (?:grams?|gms?|kg|kilos?|kilograms?|ml|millilitres?|litres?|liters?|cups?|ounces?|oz|pounds?|lbs?|tablespoons?|tbsp|teaspoons?|tsp|pints?|centimetres?|cm|inches|feet|metres?)\\b|\\b(?:half|a quarter|a third|two thirds|three quarters|three quarters|a fifth) of \\d+|\\bas a percentage\\b|\\bpercent (?:off|on top)\\b|\\b\\d+\\s*(?:%|percent)\\b.{0,25}\\b(?:off|tip|on top|between|split)\\b|\\bsplit (?:the )?(?:bill|cost|rent|fare|total|it|\\d|\\u00a3)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many) (?:do|did|can|could|would|have) you (?:know|remember|understand|recall)\\b|\\bhow (?:much|often) (?:does|do) (?:it|that|this|they) matter\\b|\\bhow much (?:should|do) (?:i|we) (?:worry|care|mind)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_ELSE_WITH(stdlib_call("re", "compile", &[py_str("\\bwhat else (?:can|could|should|shall|do|would) (?:i|we|you)\\b[^.!?]{0,25}?\\bwith\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_ELSE_STEPS(stdlib_call("re", "compile", &[py_str("\\bwhat else (?:can|could|should|shall|do|would) (?:i|we|you) (?:do|try)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PICK_ONE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|can|could) (?:i|we|he|she|they) (?:cook|make|eat|have|drink|watch|read|listen to|wear|get|buy|order|play|study|plant|visit|do with|give|bring)\\b|\\bwhat (?:do|should|shall|can|could) (?:my|our|his|her|their) \\w+ (?:cook|make|eat|have|drink|watch|read|wear|get|buy|order|play|study|plant|visit|give|bring)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAIRING_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:[\\w'-]+ )?go(?:es)? (?:with|alongside|nicely with)\\b|\\bwhat (?:[\\w'-]+ )?(?:wine|beer|drink|drinks|food|music|songs?|salad|sides?|dessert|cheese|bread|pudding|starter|sauce|film|book|present|gift)s? (?:for|with)\\b|\\bwhat should (?:i|we) name\\b|\\bwhat should (?:i|we) call (?:my|our|the|his|her|their)\\b(?![^\\n]*?\\b(?:about|for|to|if|when)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PICK_ONE_OF_A_KIND(stdlib_call("re", "compile", &[py_str("\\bwhat (?!else\\b|about\\b|of\\b|in\\b|on\\b|at\\b|to\\b|from\\b|for\\b|with\\b|by\\b|between\\b|age\\b|time\\b|date\\b|month\\b|year\\b|day\\b|size\\b|temp\\b|temperature\\b|price\\b|cost\\b|amount\\b|number\\b|help\\b|support\\b|percentage\\b|share\\b|spf\\b|budget\\b|salary\\b|wage\\b|dose\\b|dosage\\b|calorie\\w*\\b|weight\\b|length\\b|height\\b|depth\\b|speed\\b|rate\\b|fee\\b|charge\\b|rent\\b|tip\\b|discount\\b|voltage\\b|wattage\\b|qualifications?\\b|pets?\\b|rights?\\b|entitlements?\\b|apr\\b|interest\\b|deposit\\b|savings?\\b)(?:(?:kind|sort|type|brand|make) of (?!price\\b|cost\\b|amount\\b|salary\\b|budget\\b|time\\b|size\\b|weight\\b))?(?:[\\w'-]+ ){1,2}(?:should|shall) (?:i|we) [a-z]+\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_ADJECTIVE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|isn'?t|aren'?t|wasn'?t|weren'?t)\\b[^?.!]{0,30}?\\b(?:expensive|costly|pricey)\\b|\\b(?:will|would|won'?t|wouldn'?t)\\b[^?.!]{0,24}?\\bbe\\b[^?.!]{0,14}?\\b(?:expensive|costly|pricey)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_ADJECTIVE_KEEPS(stdlib_call("re", "compile", &[py_str("\\b(?:than|or|instead|versus|vs|worth|whether|rather)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_ADJECTIVE_THE_ASK_IS_LATER(stdlib_call("re", "compile", &[py_str("\\b(?:expensive|costly|pricey)\\b[^?.!]{0,30}?\\b(?:can|could|should|shall|would|will|do|does|did|am|is|are)\\s+(?:i|we|you|he|she|they|it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_ADJECTIVE_WHY(call_builtin("frozenset", &[py_tuple(vec![py_str("why")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_DO(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|can|could|would) (?:i|we) (?:do|say|take|try|check|avoid|ask|tell|help|start|stop|keep)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_YOU_DO(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|does|would|should|can|could) (?:you|one) (?:do|try)\\s+(?:about|with|if|when|for|to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_YOU_TAKE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|does|would|should|can|could|shall) (?:you|one) (?:take|bring|pack|give|get|need|buy)\\b(?! (?:for|to do|to get|to make)\\b)|\\bwhat (?:do|does|would|should|can|could|shall) (?:i|we) (?:take|bring|pack)\\b(?! (?:for|to do|to get|to make)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_I_NEED(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|does|will|would|should|shall|might|can) (?:i|we) (?:actually |really |even |truly |possibly |then |now )*(?:need|require)\\b(?!\\s+(?:to\\s+)?(?:do|know|say|ask|tell|learn|understand|work|watch|avoid|stop|prevent|fix|deal|look|sort|handle|word|phrase)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEED_TO_DO(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|does|will|would|should|shall|might|can) (?:i|we) (?:need|require) to\\s+(?:do|know|say|ask|tell|learn|understand|work|watch|avoid|stop|prevent|fix|deal|look|sort|handle|prepare|plan)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_FIGURE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:temperature|temp|heat|gas mark|setting|speed|depth|size|weight|voltage)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_AGE(stdlib_call("re", "compile", &[py_str("\\bwhat age\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_THING_FRAGMENT(stdlib_call("re", "compile", &[py_str("(?:^|[,;]\\s*)what (?!about\\b|is\\b|are\\b|was\\b|were\\b|if\\b|else\\b|next\\b|now\\b|size\\b|time\\b|colour\\b|color\\b|kind\\b|sort\\b|type\\b|age\\b|happens\\b|went\\b|works\\b)([a-z]{3,}s)\\b\\s*[.!?]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_HELPS(stdlib_call("re", "compile", &[py_str("\\bwhat (?:helps|can help|might help|will help|would help|works)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_BAKE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|can|could) (?:i|we|he|she|they) (?:bake|roast)\\b|\\bwhat (?:do|should|shall|can|could) (?:my|our|his|her|their) \\w+ (?:bake|roast)\\b|\\bwhat to (?:bake|roast)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_OPENS(stdlib_call("re", "compile", &[py_str("^(?:so |and |ok |okay |now |also |please |but )?what to\\b|\\b(?:i (?:don'?t|do not) know|not sure|no idea|dunno) what to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_PICK(stdlib_call("re", "compile", &[py_str("\\bwhat to (?:cook|make|eat|have|drink|watch|read|listen to|wear|get|buy|order|play|study|plant|visit|give|bring|pack)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_STEPS(stdlib_call("re", "compile", &[py_str("\\bwhat to (?:do|say|write|text|reply|feed|serve|take|try|check|avoid|ask|tell|keep)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_I_WRITE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|can|could|would) (?:i|we) (?:write|say|text|reply|word|phrase|sign)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_I_GROW(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|can|could|would) (?:i|we) (?:grow|sow|plant|batch|bake|roast)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORTED(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|they|mum|dad|mom|someone|somebody|my \\w+)\\b.{0,25}\\b(?:asked|told|wonder\\w*|wants to know|keeps asking)\\b\\s*(?:(?:me|us|him|her|them|that|about|to)\\s*){0,2}(?:what|whether|if)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBSTITUTE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|can|could) (?:i|we) use\\b(?! (?:for|on|to)\\b)|\\bwhat (?:else )?(?:do|should|shall|can|could) (?:i|we) (?:use|try|have|make)\\b.{0,25}\\binstead\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RECALL_ASK(stdlib_call("re", "compile", &[py_str("\\bremind me (?:of |about |that |if |whether )|\\bremind me (?:what|when|where|who|which|how) (?!to\\b)|\\bwhat (?:did|have) (?:i|we) (?:say|said|ask|asked|tell|told|mention|mentioned)\\b|\\bwhat (?:was|were) (?:my|our) (?:first|last|earlier|original)\\b|\\b(?:what|which) (?:was|is) the (?:first|last) (?:thing|line|question|message) i\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVE_RECALL(stdlib_call("re", "compile", &[py_str("^\\W*what (?:do|did) i have\\b(?! (?:for|on|in the|tonight|to eat|to wear|to drink))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_STEPS(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|does|did) (?:i|you|we|he|she|they) do for (?:a )?living\\b|\\bwhat (?:do|does|did) (?:i|you|he|she|they) do for work\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHER_QUESTION(stdlib_call("re", "compile", &[py_str("^\\W*(?:where|when|who|whose|which|why|how|is|are|was|were|can|could|should|shall|would|will|does|did)\\b(?!\\s+not\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEED_TO(stdlib_call("re", "compile", &[py_str("\\bi (?:need|have|'ve|have got|got|'ll need|will need) to (?:(?:sort|register|ring|call|fill|send|tell|book|apply|arrange|cancel|close|write|deal)\\b(?:\\s+(?:out|up|off|in|on|with|for|about))?\\s+[\\w'-]+|(?:get|find|move|do|work|set|change)\\b(?:\\s+(?:out|up|off|in|to|with|for|about))?\\s+(?:the|my|his|her|their|our|a|an|this|that|these|those|it|them|him|us|me)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEED_TO_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:do|does|did|will|would|should|shall|can|could|might|may|must|have)\\s+(?:i|we)\\s+(?:need|have|got|'ve)\\s+to\\b|\\b(?:what|which|how|when|where|who|why)\\s+(?:\\w+\\s+)?(?:do|does|did|will|would|should|shall|can|could)\\s+(?:i|we)\\s+(?:need|have|got|'ve)\\s+to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURE_LABELS(call_builtin("frozenset", &[py_tuple(vec![py_str("how_long"), py_str("how_much"), py_str("how_often")])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|where|when|who|whom|whose|why|how)\\s+(?:do|does|did|should|shall|can|could|will|would|must|is|are|am)\\b|\\bhow (?:long|often|many|much)\\s+(?:do|does|did|should|shall|can|could|will|would|must)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_START(stdlib_call("re", "compile", &[py_str("\\bwhere (?:do|does|should|shall|can|could|would|will|to) (?:\\w+ ){0,3}(?:start|begin)\\b|\\bwhere to (?:begin|start)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ORIGIN_ASK(stdlib_call("re", "compile", &[py_str("\\bwhere (?:does|do|did) (?:\\w+ ){0,3}(?:come from|grow)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MADE_OF(stdlib_call("re", "compile", &[py_str("\\bwhat (?:is|are) (?:\\w+ ){0,3}made (?:of|from|out of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ORIGIN_PERSON(stdlib_call("re", "compile", &[py_str("\\bwhere (?:does|do|did) (?:you|they|we|i|he|she|u)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_ITS_OWN_YES_NO(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|will|would|can|could|should|did|is|are|was|were) (?:it|that|this|they|these|those)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_POINT(stdlib_call("re", "compile", &[py_str("\\bwhere(?:'s|s| is| was)\\b[^?]{0,20}\\b(?:the|any) point\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_RECALL(stdlib_call("re", "compile", &[py_str("\\bwhere (?:am|was) i\\b|\\bwhere (?:were|are) we\\b|\\bwhere we (?:left|were|are)\\b|\\bremind me where\\b|\\b(?:know|knows|known|tell me|told me|remind|remember|forgot|forget|said)\\b[^?]{0,25}\\bwhere (?:i|we|you) (?:live|work|are|were|went|put)\\b|\\b(?:know|knows|known|tell me|told me|remind|remember|forgot|forget|said)\\b[^?]{0,25}\\bwhere (?:he|she|they) (?:live|lives|work|works|stay|stays)\\b|\\bwhere (?:does|did) (?!a |an |the )[a-z]{2,}(?: [a-z]{2,})? (?:live|work|stay)\\b|\\bwhere (?:do|does|did) (?:i|you|he|she|we|they) (?:live|work|stay)\\b|\\bwhere (?:do|does|did) (?:my|our|your|his|her|their) \\w+(?: \\w+)? (?:live|work|stay)\\b|\\bwhere (?:do|does|did) i (?:decide|say|tell|mean|think|go\\b[^?]{0,12}\\b(?:last|before))|\\bwhere did we (?:leave off|left off|leave it|left it|get to|got to)\\b|\\bwhere (?:did|do) we (?:get up to|got up to)\\b|\\bwhere(?:'s|s| is| are| did i put| have i put| can i find)\\b[^?]{0,20}\\b(?:my|me|our|the)\\s+(?:phone|keys|key|wallet|purse|glasses|bag|bags|parcel|nappy|nappies|coat|remote|passport|documents|bus pass|ticket|tickets)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_ALONE(stdlib_call("re", "compile", &[py_str("^(?:and |so |ok |okay )?where(?:'s|s| is| are) (?:it|that|this)$|^(?:and |so |ok |okay )?where (?:i|we|you) (?:live|work)$|^where (?:do|does|did) i (?:live|work)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_TO_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:how|what) (?:do|should|can|could|would) i (?:answer|reply to|explain|word|phrase)\\b|\\b(?:what|how) (?:do|should|would) you say when they ask\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_AND_MEASURE(stdlib_call("re", "compile", &[py_str("\\bwho\\b[^?]{0,40}\\b(?:what|how much|how many)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_OF_A_PRONOUN(stdlib_call("re", "compile", &[py_str("\\bwho (?:i|you|we|they|he|she|it) (?:am|are|is|was|were)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_OF_A_NOUN(stdlib_call("re", "compile", &[py_str("\\b(?:the|a|an|this|that|these|those|someone|somebody|anyone|anybody|everyone|everybody|no one|nobody|person|people|man|woman|boy|girl|child|kid|friend|colleague|neighbour|neighbor|boss|manager|teacher|nurse|doctor|carer|minder|babysitter)\\s+who\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_OPENS(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|so|but|then|or|also|now|ok|okay)\\s+)?who(?:'s|s|se)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BEST_WAY(stdlib_call("re", "compile", &[py_str("\\b(?:the )?(?:best|good|right|proper|easiest|quickest|simplest|safest|cheapest|fastest) (?:way|method|approach) (?:to|of|for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_MEASURE(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s| is)\\s+(?:a |an |the )?(?:really |actually |roughly |sensible |sort of )?(?:good|decent|normal|typical|average|usual|realistic|fair)\\s+(?:\\w+\\s+){0,2}(?:salary|wage|wages|pay|income|rate|deposit|budget|score|pension|pot|percentage|percent|tip(?!\\s+(?:for|on|to|about)\\b))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_ONE(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s| is) (?:a |an )?good\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVE_ME_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:gimme|give me|show me|send me)\\s+(?:a|an|one|another)\\s+([a-z][a-z'-]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVE_ME_A_FEW(stdlib_call("re", "compile", &[py_str("\\b(?:gimme|give me|show me|send me)\\s+(?:some|a few|couple of|a couple|any)\\s+([a-z][a-z'-]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVE_ME_NOT_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:human|person|somebody|someone|real person|operator|manager)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVE_ME_AN_IDIOM(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|one)\\s+(?:rough\\s+idea|minute|minutes|second|seconds|sec|secs|mo|moment|break|chance|ring|rings|shout|call|lift|ride|hand|hands|hint|clue|tip|tips|way|method|reason|sign|excuse|rest|try|kiss|discount|picture|photo|image)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVE_ME_MADE_UP(stdlib_call("re", "compile", &[py_str("\\b(?:limerick|poem|haiku|riddle|joke|story|tale|rhyme|tongue twister|fun fact|fact|quiz question|trivia question|general knowledge question|question|example|routine|prompt|toast|speech|plan|itinerary|list|recipe)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVE_ME_LEAD_IN(call_builtin("frozenset", &[py_tuple(vec![py_str("ok"), py_str("okay"), py_str("so"), py_str("and"), py_str("now"), py_str("then"), py_str("just"), py_str("please"), py_str("also"), py_str("but"), py_str("you"), py_str("could"), py_str("can"), py_str("would"), py_str("will"), py_str("hi"), py_str("hey"), py_str("gimme"), py_str("me"), py_str("of"), py_str("if"), py_str("maybe"), py_str("alright")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_TO_GIVE(stdlib_call("re", "compile", &[py_str("\\b(?:gift|gifts|present|presents|pressie|pressies|stocking filler|secret santa)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_TO_GIVE(py_str("(?:my|our|his|her|their|him|them|mum|mam|mom|dad|mother|father|sister|brother|son|daughter|nan|nana|grandma|grandad|granny|grandpa|gran|aunt|uncle|cousin|friend|mate|bestie|teacher|colleague|boss|partner|wife|husband|boyfriend|girlfriend|neighbours?|baby|newborn|toddler|child|kid|boy|girl|teenager|nephew|niece|in-laws?|flatmate|housemate|carer|\\d+\\s*(?:year|yr)s?\\s*old)"));
    s_GIVING_FRAME(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:for|to)\\s+(?:a|an|the)?\\s*"), &g_A_PERSON_TO_GIVE()), &py_str("\\b|\\bwhat (?:should i |shall i |do i |can i |would you |to )?(?:get|buy|make|give)\\b|\\b(?:idea|ideas|suggest|suggestion|suggestions|recommend|recommendation)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_GIVING_THING(stdlib_call("re", "compile", &[py_str("\\bgift (?:card|cards|voucher|vouchers|wrap|wrapping|box|boxes|receipt|set)\\b|\\bpresent (?:tense|day|perfect)\\b|\\bgift (?:aid|tax|taxable)\\b|\\b(?:letter|note|thanks|thank you|receipt|return|returns)\\b|\\b(?:keeps?|kept|always)\\b[^?]{0,25}\\b(?:gift|present)s?\\b|\\b(?:buys|bought|gives|gave|brings|brought|receives|received)\\b[^?]{0,20}\\b(?:gift|present)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_COME(stdlib_call("re", "compile", &[py_str("\\bhow come\\b|\\bhow is it that\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BEST_OF_MANY(stdlib_call("re", "compile", &[py_str("\\bwhat (?:are|were) (?:the )?(?:best|main|top|nicest|cheapest|better)\\b|\\bwhat (?:are|were) (?:some|a few|any)\\b|\\bwhat (?:are|were) (?:the )?\\w+ (?:to|for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PRICE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\s+(?:it|that|this|these|those|they|there|one)\\s+(?:(?:too|so|really|actually|very|quite)\\s+)?(?:expensive|pricey|dear|cheap|costly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PRICE_ASK(stdlib_call("re", "compile", &[py_str("\\bworth\\b|\\bcheaper than\\b|\\bmore expensive than\\b|\\bafford"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_ARE_MINE(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s| is| are|s)\\b[^?]{0,40}?\\b(?:my|our) (?:[a-z]+ ){0,2}(?:rights?|entitlements?|obligations?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MY_THING_CALLED(stdlib_call("re", "compile", &[py_str("\\bwhat (?:are|is|were|was) (?:my|our)\\b[^?]{0,30}?\\b(?:called|named)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_OF_MINE(stdlib_call("re", "compile", &[py_str("\\bremind(?:ing)? me (?:what|who|which|where|when)\\b|\\bwhat (?:did|do) i (?:say|told you|tell you|tell u|ask|asked|decide|choose)\\b|\\bwhat (?:was|am) i (?:saying|doing|about to say)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_FOR_MEANING(stdlib_call("re", "compile", &[py_str("\\bwhat (?:is|are|was|were|'s)\\b|\\bwhat (?:does|do)\\b(?:(?!\\bwhat\\b)[^?]){0,30}?\\b(?:do|does|include|includes|cover|covers|contain|contains|show|shows|sound like)\\b|\\bwhat do you call\\b|\\b(?:mean|means|meaning|stand for|stands for|short for)\\b|\\bdifference between\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEPS_ARE_COUNTED(stdlib_call("re", "compile", &[py_str("\\b\\d[\\d,]*\\s*(?:k\\s+)?steps?\\b|\\bsteps? count\\b|\\bsteps? (?:a|per|each|every) (?:day|week|month|hour|mile|km|kilometre)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEPS_AS_A_DISTANCE(stdlib_call("re", "compile", &[py_str("\\bsteps? (?:in|into|to|per|versus|vs)\\s+(?:a |one )?(?:mile|miles|km|kilometre|kilometres|kilometer|kilometers)\\b|\\b(?:mile|miles|km|kilometre)s? (?:in|to|from|of) (?:a |one )?\\d[\\d,]*\\s*(?:k\\s+)?steps?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_ASK_FORM(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many|long|often|soon|far|tall|big|heavy|old|hot|cold)\\s+(?:is|are|was|were|does|do|did|will|would|can|could|should|shall|much|many|far)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_TIME(stdlib_call("re", "compile", &[py_str("\\bhow (?:many|much) (?:more |longer )?(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|nights?|time)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SIZE_LONG(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:mm|cm|m|km|metres?|meters?|feet|foot|ft|inches?|in|yards?|yds?)\\s+(?:long|wide|high|deep|tall|thick)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DURATION_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:take|takes|took|taking|last|lasts|lasted|lasting|keep|keeps|kept|stay|stays|sit|sits|wait|waiting|seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|overnight|fortnight|fortnightly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REASON_ASK(stdlib_call("re", "compile", &[py_str("\\bhow come\\b|\\bwhat (?:causes|caused|causing|is causing)\\b|\\bwhat(?:'?s| is| was)?\\s+(?:causing|making|the cause|the reason)\\b|\\bwhat (?:makes|made) (?:you|me|us|people|him|her|them|it|this|that|someone|my|your|our|their|his|its)\\b|\\bwhat (?:makes|made) (?:the |a |an )?[a-z']+(?:(?: and| or) (?:the |a |an )?[a-z']+)*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DOING_WRONG(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'?s| is| was)?\\s+(?:going\\s+|gone\\s+)?wrong\\b|\\bwhat (?:went|had gone|has gone|have gone) wrong\\b|\\bwhat (?:could|might|may|would) be (?:wrong|making|causing)\\b|\\bwhat (?:am i|are we|did i|did we|have i|have we) (?:do|doing|done|did|get|getting|got|have|having|had)\\b.{0,25}\\bwrong\\b|\\bwhat (?:do i|do we|does he|does she|do they) do wrong\\b|\\bwhere (?:am i|are we|did i|did we|do i|do we) going wrong\\b|\\bwhat(?:'?s| is| was)?\\s+(?:the|my|our|his|her|their)\\s+(?:problem|trouble|issue|fault)\\s+(?:with|in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WRONG_ASKED_ABOUT(stdlib_call("re", "compile", &[py_str("\\b(?:mean|means|meaning|meant|called|spell|spelled|word)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WRONG_RECALL(stdlib_call("re", "compile", &[py_str("\\bremind me\\b|\\bwhat (?:was|were)\\b[^?]{0,30}\\b(?:wrong|problem|trouble|issue|fault)\\b|\\b(?:wrong|problem|trouble|issue|fault)\\b[^?]{0,20}\\bagain\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINT_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'?s| is| was)?\\s+the point (?:of|in)\\s+(\\w+(?: \\w+){0,2})|^(?:any )?point (?:of|in)\\s+(\\w+(?: \\w+){0,2})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINT_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\n    it this that these those anything everything nothing any all carrying carry continuing going\n    living life you me us them anymore honestly really now boiling freezing melting smoke\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FOLLOWUP(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|so|then|but|ok(?:ay)?[,\\s]+(?:and|so)|what\\s+about|how\\s+about|what\\s+if|would\\s+that|does\\s+that|same\\s+(?:for|with))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEW_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|does|should|shall|can|could|will|would) (?:i|we|you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:did|do)\\s+(?:i|you)\\s+(?:say|said|told|tell|mention|remember)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything|something)\\s+(?:do|does|should|can|could)\\s+(?:i|you)\\s+(?:say|tell)\\b|\\bhow\\s+(?:do|does|can|could|should|would)\\s+(?:i|you|we|u|ya)\\s+tell\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_TIME(stdlib_call("re", "compile", &[py_str("^(?:when|what\\s+time|what\\s+date|what\\s+day|how\\s+(?:long|soon|often)|how\\s+many\\s+(?:weeks|days|months|years|hours|minutes|nights|times))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_AMOUNT(stdlib_call("re", "compile", &[py_str("^(?:how\\s+(?:much|many|old|heavy|tall|far|big)|what\\s+age|what\\s+size)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_PLACE(stdlib_call("re", "compile", &[py_str("^where\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_PERSON(stdlib_call("re", "compile", &[py_str("^(?:who|what\\s+name|(?:her|his|their|my|our)\\s+name)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_KIND_NOUN(py_dict(vec![(py_str("budget"), g_AMOUNT()), (py_str("rent"), g_AMOUNT()), (py_str("hours"), g_AMOUNT()), (py_str("shift"), g_AMOUNT()), (py_str("salary"), g_AMOUNT()), (py_str("wage"), g_AMOUNT()), (py_str("target"), g_AMOUNT()), (py_str("apr"), g_AMOUNT()), (py_str("age"), g_AMOUNT()), (py_str("size"), g_AMOUNT()), (py_str("name"), g_PERSON()), (py_str("day"), g_TIME()), (py_str("date"), g_TIME()), (py_str("time"), g_TIME())]));
    s_RECALL_PRONOUN(stdlib_call("re", "compile", &[py_str("^(?:you|me|us|it|that|then)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_KIND_NOUN_ASK(stdlib_call("re", "compile", &[py_str("^(?:my|our)\\s+(budget|rent|hours|shift|salary|wage|target|apr|age|size|name|day|date|time)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_SHAPE_BEFORE(stdlib_call("re", "compile", &[py_str("(?:\\bwhen|\\bwhat\\s+time|\\bwhat\\s+date|\\bwhat\\s+day|\\bhow\\s+(?:long|soon|often)|\\bhow\\s+many\\s+(?:weeks|days|months|years|hours|minutes|nights|times)|\\bhow\\s+(?:much|many|old|far)|\\bwhat\\s+age|\\bwhat\\s+size|\\bwhere|\\bwho|\\bwhat\\s+name)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_SHAPE_BEFORE_KIND(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("when"), py_str("what time"), py_str("what date"), py_str("what day"), py_str("how long"), py_str("how soon"), py_str("how often")]), g_TIME()]), py_tuple(vec![py_tuple(vec![py_str("how many weeks"), py_str("how many days"), py_str("how many months"), py_str("how many years"), py_str("how many hours"), py_str("how many minutes"), py_str("how many nights"), py_str("how many times")]), g_TIME()]), py_tuple(vec![py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how old"), py_str("how far"), py_str("what age"), py_str("what size")]), g_AMOUNT()]), py_tuple(vec![py_tuple(vec![py_str("where")]), g_PLACE()]), py_tuple(vec![py_tuple(vec![py_str("who"), py_str("what name")]), g_PERSON()])]));
    s__ASK_CLAUSE_SPLIT(stdlib_call("re", "compile", &[py_str("\\s*(?:[,;:]|\\band\\b|\\bbut\\b|\\bso\\b|\\bthen\\b)\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TRAILING_MEASURE(stdlib_call("re", "compile", &[py_str("\\b(?:how long|how soon|what time)\\s*[.?!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MEASURE_THEN_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:how long|how soon|what time)\\s+(?:do|does|did|will|would|should|shall|can|could|is|are|was|were|to|till|until|before|in|on|for|per|at|with|by|after|into|under|over|about|from)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TALKING_ABOUT_A_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|you|they|he|she)\\s*(?:did ?n[o']t|do ?n[o']t|does ?n[o']t|never)?\\s*(?:know|knew|realise|realize|remember|wonder|wondered|sure|idea)\\b|\\b(?:amazing|surprising|interesting|funny|odd|strange) how (?:long|soon|much|many|often)\\b|\\bno idea how\\b|\\bhow long it (?:takes|took|would take|will take)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_CLAUSE_OPENS(stdlib_call("re", "compile", &[py_str("^(?:when|what|which|how|where|who|why|whom|whose|should|shall|can|could|do|does|did|will|would|is|are|was|were|am|have|has|had)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFIRM_CAUSE(stdlib_call("re", "compile", &[py_str("(?:^|[;,.]\\s*|\\b(?:and|so|but|then)\\s+)(?:is|was) (?:that|this|it) why\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IF_IT_HAPPENS(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|so\\s+|but\\s+|then\\s+)?if\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_TO_ASK(stdlib_call("re", "compile", &[py_str("\\bshould i (?:ask|explain|check|say|mention|bring up)\\s+why\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE_NAMED(stdlib_call("re", "compile", &[py_str("\\bwhat (?:age|price|cost|number|salary|pay|wage|rate|fee|dose|dosage|amount|percentage|percent|discount|charge|quote|score|mark|grade|figure|budget)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TIME_NAMED(stdlib_call("re", "compile", &[py_str("\\bwhat time\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_VERDICT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|was|are)\\s+(?:it|that|this)\\s+(?:worth it|worth|ok|okay|fine|right|normal|bad|a good (?:idea|plan|buy|choice))[\\s.?!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_VERB_HEAD(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|it|it's|its|there|this|that|these|those|they|them|we|you|i|he|she|both|all|some|most|many|one|two|three|four|five|six|seven|eight|nine|ten|from|in|at|on|for|to|with|without|after|before|about|around|once|if|when|until|roughly|up|down|by|as|of|no|not|nothing|nobody|never|always|usually|often|your|my|our|his|her|their|every|each|another|any|much|little|few|several|such|what|who)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_I_OFFER(stdlib_call("re", "compile", &[py_str("\\b(?:i'll|i will|i can|i'd|we'll|we can|let me|i've|i would)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_THE_BOT(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would|will)\\s+(?:you|u)\\b|\\bdo you\\b|\\bare you\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_UNIT(stdlib_call("re", "compile", &[py_str("\\d|\\b(?:mg|g|kg|ml|l|litres?|liters?|cm|mm|metres?|meters?|feet|foot|inches?|degrees?|c|f|hours?|hrs?|minutes?|mins?|seconds?|days?|weeks?|months?|years?|nights?|p|pence|pounds?|quid|grams?|calories|kcal|watts?|kw|kwh|amps?|volts?|per\\s+cent|%)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(stdlib_call("re", "compile", &[py_str("^(?:it|it's|this|that|these|those|they|them|there|its)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("and"), py_str("for"), py_str("you"), py_str("not"), py_str("but"), py_str("all"), py_str("any"), py_str("one"), py_str("two"), py_str("are"), py_str("was"), py_str("has"), py_str("had"), py_str("how"), py_str("why"), py_str("who"), py_str("did"), py_str("may"), py_str("can"), py_str("its"), py_str("our"), py_str("per"), py_str("via"), py_str("yet"), py_str("own"), py_str("off"), py_str("out"), py_str("too"), py_str("get"), py_str("put"), py_str("use"), py_str("let"), py_str("now"), py_str("far"), py_str("lot"), py_str("bit"), py_str("way"), py_str("she"), py_str("him"), py_str("her"), py_str("his"), py_str("new"), py_str("old"), py_str("few"), py_str("yes"), py_str("them"), py_str("they"), py_str("their"), py_str("there"), py_str("then"), py_str("that"), py_str("this"), py_str("with"), py_str("from"), py_str("your"), py_str("have"), py_str("what"), py_str("when"), py_str("will"), py_str("would"), py_str("could"), py_str("should"), py_str("about"), py_str("into"), py_str("over"), py_str("just"), py_str("only"), py_str("even"), py_str("still"), py_str("much"), py_str("many"), py_str("some"), py_str("very"), py_str("also"), py_str("been"), py_str("being"), py_str("does"), py_str("done"), py_str("more"), py_str("most"), py_str("less"), py_str("like"), py_str("need"), py_str("want"), py_str("make"), py_str("made"), py_str("going"), py_str("were"), py_str("than"), py_str("such"), py_str("each"), py_str("both"), py_str("else"), py_str("said")])]).unwrap_or_else(|e| exc::raise(e)));
    s_METHOD_VERBS(stdlib_call("re", "compile", &[py_str("^(?:lower|brown|mash|crush|sprinkle|grease|steam|blend|scatter|lay|shake|score|chill|thaw|reheat|serve|strain|sieve|portion|layer|arrange|thread|screw|hammer|nail|clamp|level|preheat|dust|wring|drape|tuck|loop|knot|peg|hoover|vacuum|mop|air|cure|rest)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OPTIONS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|wat|wot|wats)\\s+(?:are|r)\\s+(?:my|the|our|his|her|their)\\s+(?:other\\s+|main\\s+)?options?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_CAN_I_DO(stdlib_call("re", "compile", &[py_str("\\bwhat (?:can|could) (?:i|we) do\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_CAN_I_DO_WITH(stdlib_call("re", "compile", &[py_str("\\bwhat (?:(?:can|could) (?:i|we)|to) do with\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_SHOULD_I_DO_WITH(stdlib_call("re", "compile", &[py_str("\\bwhat (?:should|shall|do|did|would|might) (?:i|we) do with\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANY_DO_WITH(stdlib_call("re", "compile", &[py_str("\\bwhat (?:to|(?:can|could|should|shall|do|did) (?:i|we)) do with\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DO_WITH_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:do|does|doing) with (?:myself|yourself|himself|herself|ourselves|yourselves|themselves|him|her|each other)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHOICE(stdlib_call("re", "compile", &[py_str("\\bor\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORDS_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|could|should|would) (?:i|we|you) (?:tell|say|ask|word|phrase|put it|bring (?:it|this|that) up|raise (?:it|this|that)|mention|approach)\\b|\\bhow to (?:tell|say|ask|word|phrase|bring (?:it|this|that) up|raise (?:it|this|that))\\b|\\bwhat (?:should|shall|do|can|could|would) (?:i|we) (?:say|tell|text|write|ask|reply|answer|word it|phrase it)\\b|\\bwhat to (?:say|tell)\\b|\\bwhat should be said\\b|\\b(?:dont|don't|do not|didnt|didn't|never|no idea) (?:know )?(?:quite |really )?(?:how|what) to (?:say|tell|word|phrase|ask)\\b|\\b(?:idk|dunno|no idea) (?:how|what) to (?:say|tell|word|phrase|ask)\\b|\\bhow (?:should|do) i (?:break|ease) it to\\b|\\bbreak it to\\b|\\b(?:cant|cannot|can't|dont|don't|struggling to) (?:find|think of) (?:the )?(?:right |proper )?words\\b|\\b(?:find|finding|choose|need|want) (?:the |some )?(?:right |proper |good )?words\\b|\\bwords to (?:say|tell|use)\\b|\\bhelp me (?:word|phrase|put it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TELLS_A_TEST(stdlib_call("re", "compile", &[py_str("\\b(?:tell|saying) (?:if|whether|when|apart|the difference|which|how|what|why|where|who)\\b|\\btell me\\b|\\btell us\\b|\\btell (?:you|ya)\\b|\\btell (?:\\w+ ){0,4}\\bfrom\\b|\\btell (?:\\w+ ){0,3}\\boff\\b|\\btell (?:\\w+ ){0,2}\\bapart\\b|\\bhow (?:do|can|could|would) (?:i|we|you) tell\\s*[?.!]*$|\\bhow (?:do|can|could|should|would) (?:i|we|you) ask (?:for|about)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_WORDS_ANYWAY(stdlib_call("re", "compile", &[py_str("\\b(?:in|into) (?:french|spanish|italian|german|welsh|irish|gaelic|japanese|chinese|portuguese|dutch|polish|russian|arabic|hindi|greek|turkish|latin)\\b|\\bbreak (?:the ice|a habit|the bank|a record)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WRITES_A_FORM(stdlib_call("re", "compile", &[py_str("\\b(?:writ(?:e|ing)|fill(?:ed)? in) (?:(?:\\w+) ){0,4}(?:form|forms|declaration|customs|invoice|receipt|application|claim\\b|return\\b|questionnaire|box|boxes|field)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WRITES_IN_A_DOCUMENT(stdlib_call("re", "compile", &[py_str("\\bwrit(?:e|ing|ten)\\b[^.!?]{0,30}\\b(?:cv|curriculum vitae|cover(?:ing)? letter|journal|diary|essay|report|thesis|dissertation|blog|article|minutes|form|declaration|invoice|receipt|application|claim|questionnaire|statement|record)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WRITES_TO_A_MESSAGE(stdlib_call("re", "compile", &[py_str("\\b(?:card|letter|note|message|e-?mail|text|reply|comment|speech|toast|eulogy|announcement|invitation|post)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COVER_LETTER(stdlib_call("re", "compile", &[py_str("\\bcover(?:ing)? letter\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_WRITE(stdlib_call("re", "compile", &[py_str("\\bwhat (?:should|shall|do|can|could|would) (?:i|we) write\\s*[?.!]*$|\\bwrite (?:about|next|something|anything|more)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_WORDS_ASK(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&gattr(&g_TELLS_A_TEST(), "pattern"), &py_str("|")), &gattr(&g_NOT_WORDS_ANYWAY(), "pattern")), &py_str("|")), &gattr(&g_WRITES_A_FORM(), "pattern")), &py_str("|")), &gattr(&g_BARE_WRITE(), "pattern")), &py_str("|")), &py_str("\\b(?:tell|talk to|speak to|say to) (?:\\w+ ){1,3}about\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_WORDS(stdlib_call("re", "compile", &[py_str("\\bwords to (?:say|use|tell)\\b|\\bwords for (?:telling|saying|asking)\\b|\\b(?:give|send|hand) (?:me )?(?:the )?words\\b|\\bfind (?:me )?(?:the )?(?:right |proper )?words\\b|\\bput (?:it|this) in(?:to)? words\\b|\\bhelp me (?:word|phrase|put it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TELL_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|could|would|will) (?:i|you|we|someone) (?:tell|know|check|spot|test)\\b[^.!?]{0,30}\\b(?:if|whether|when)\\b|\\bhow to (?:tell|know|check|test)\\b[^.!?]{0,20}\\b(?:if|whether|when)\\b|\\bhow (?:do|can) (?:i|you|we) know when\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IS_IT_DONE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|will)\\b[^?]{0,40}\\b(?:done|cooked|ready|risen|proved|proven|set|thickened|browned|defrosted|thawed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_SIGN(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:°\\s*c|c\\b|degrees|°\\s*f)\\b[^.!?]{0,80}\\b(?:thickest|centre|center|middle|inside|core|internal|juices|clear|firm|soft|springy|opaque|no longer|done|cooked|ready|set)\\b|\\b(?:the|that)\\b[^.!?]{0,30}\\bis the (?:tell|sign|test|clue|giveaway)\\b|\\bthat is how\\b[^.!?]{0,40}\\b(?:reads|looks|feels)\\b|\\b(?:when|until|once|by the time)\\b[^.!?]{0,90}\\b(?:browned|golden|clear|firm|soft|springy|opaque|thick|risen|doubled|separat\\w*|cracked|loose|tender|flaky|hollow)\\b|\\bsounds? (?:hollow|right|done)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__gives_as_read(g_gives());
    s_BARE_WHAT_AS_IS(stdlib_call("re", "compile", &[py_str("^what(?:'s|s)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_WHAT_IS_PLAIN(stdlib_call("re", "compile", &[py_str("^what\\s+(?:is|are|was)\\s+(?P<thing>[^\\s,;:]+.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_NOT_A_THING_AT_THE_START(stdlib_call("re", "compile", &[py_str("^(?:you|u|ur|your|yours|i|im|i'm|my|mine|our|ours|we|us|me|it|this|that|these|those|there|here|her|hers|his|their|theirs|its|up|on|at|for|with|under|inside|outside|behind|near|next|around|about|in|new|nice|good|great|better|best|cheap|cheapest|expensive|wrong|missing|left|leftover|easy|easiest|hard|hardest|quick|quickest|simple|simplest|safe|safest|healthy|healthiest|popular|common|normal|usual|fast|fastest|slow|slowest|big|biggest|small|smallest|right|going|doing|showing|happening|playing|coming|open|one|two|three|four|five|six|seven|eight|nine|ten|hundred|thousand|million|half|double)\\b|^the\\s+(?:\\w+est|best|worst|most|least|furthest)\\b|^the\\s+(?:new|newest|latest|current|next|other|same|main|real)\\b|^the\\s+(?:time|date|day|capital|price|cost|charge|fee|number|address|name|answer|way|point|difference|matter)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_VALUE_NOUN(stdlib_call("re", "compile", &[py_str("^(?:the\\s+)?(?:\\w+\\s+){0,3}(?:rate|rates|price|prices|cost|costs|charge|charges|fee|fees|number|numbers|phone|telephone|address|addresses|email|website|opening\\s+hours|point|points|allowance|temperature|password|postcode|pin|score|limit|deadline|budget|balance|total|amount|figure|percentage|salary|wage|wages|weight|height|length|width|depth|distance|speed|age|population|capacity|mileage)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_NOT_A_THING_ANYWHERE(stdlib_call("re", "compile", &[py_str("\\d|\\bcalled\\b|\\bname of\\b|\\bmeaning of\\b|\\blike\\b|\\bwrong\\b|\\bpower\\s+of\\b|\\btimes\\b|\\bplus\\b|\\bminus\\b|\\bdivided\\b|\\bmultiplied\\b|\\bpercent\\s+of\\b|\\bsquared\\b|\\bcubed\\b|\\btodays?\\b|\\btonights?\\b|\\btomorrows?\\b|\\bthis\\s+(?:week|weekend|month|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_A_SECOND_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:why|how|where|when|who|which)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_A_RECALL(stdlib_call("re", "compile", &[py_str("\\b(?:again|earlier|before|yesterday)\\s*$|\\b(?:i|you)\\s+said\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_NOTHING_ASKED(call_builtin("frozenset", &[py_tuple(vec![py_str("up"), py_str("on"), py_str("it"), py_str("that"), py_str("new"), py_str("happening"), py_str("going on")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABBREVIATION_END(stdlib_call("re", "compile", &[py_str("\\b(?:e\\.g|i\\.e|approx|incl|etc|vs|cf|fig|dept|est|ref|tel|mr|mrs|ms|dr|prof)\\.$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE_END(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_LEAD_IN(call_builtin("frozenset", &[py_tuple(vec![py_str("within"), py_str("about"), py_str("around"), py_str("roughly"), py_str("approximately"), py_str("every"), py_str("by"), py_str("up"), py_str("least"), py_str("most"), py_str("after"), py_str("before"), py_str("for"), py_str("takes"), py_str("take"), py_str("lasts"), py_str("last"), py_str("lasted"), py_str("keeps"), py_str("keep"), py_str("needs"), py_str("need"), py_str("expected"), py_str("allow"), py_str("allows")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_STOP(stdlib_call("re", "compile", &[py_str("\\b(?:i|you|your|yours|we|our|us|me|my|it|its|they|them|their|he|she|his|her|is|are|was|were|be|been|being|do|does|did|have|has|had|can|could|will|would|should|shall|may|might|must|if|then|so|but|that|this|these|those|there|here|when|while|because|about|from|with)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEPS_INSIDE_A_LIST(stdlib_call("re", "compile", &[py_str(",\\s*(?:then|next|after that|afterwards|finally|lastly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WORD_NUMBER(py_str("(?:\\d+(?:[.,]\\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand|million|a couple|couple|a few|half|a quarter|a third|a fifth|double|dozen)"));
    s__DURATION_QUALIFIER(py_str("(?:(?:working|clear|business|banking|bank|calendar|school|full|whole|solid|straight|extra|more|further)\\s+)?"));
    s_DURATION(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("(?:"), &g__WORD_NUMBER()), &py_str("|\\ba\\b|\\ban\\b)[\\s-]*(?:")), &g__WORD_NUMBER()), &py_str("[\\s-]*)?")), &g__DURATION_QUALIFIER()), &py_str("(?:sec|second|min|minute|hr|hour|day|week|fortnight|month|year|night|morning|afternoon|evening)s?\\b|\\bovernight\\b|\\ball day\\b|\\ball night\\b|\\bfor a bit\\b|\\bwithin the hour\\b|\\bsoon\\b|\\b(?:for|in) (?:days|weeks|months|years)\\b|\\ba (?:full|whole) (?:hour|day|week|month|year)s?\\b|\\b(?:a couple of|a pair of|a number of|a handful of|several|many|numerous)\\s+(?:sec|second|min|minute|hr|hour|day|week|fortnight|month|year|night|morning|afternoon|evening)s?\\b|\\bten minutes\\b|\\bnext day\\b|\\b(?:a )?couple of (?:secs?|seconds?|mins?|minutes?|hours?|days?|weeks?|months?|years?|nights?)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_ASKED(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|wide|tall|deep|high|far) (?:is|are|was|were)\\b|\\bhow (?:long|wide|tall|deep|high|far) (?:does|do|did) (?:[\\w'-]+ ){0,3}(?:run|runs|stretch|stretches|measure|measures|reach|reaches|extend|extends|go|goes)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EXTENT(stdlib_call("re", "compile", &[add(&add(&py_str("(?:"), &g__WORD_NUMBER()), &py_str("|\\ba\\b|\\ban\\b)[\\s-]*(?:[a-z]+[\\s-]+){0,3}(?:km|kilometres?|kilometers?|miles?|metres?|meters?|yards?|feet|foot|inches?|cm|mm|centimetres?|centimeters?|millimetres?|millimeters?|pages?|words?|lines?|songs?|tracks?|episodes?|chapters?|metres?|m)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_SURE(stdlib_call("re", "compile", &[py_str("\\b(?:mon|tues|wednes|thurs|fri|satur|sun)day\\b|\\b(?:january|february|april|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sept|sep|oct|nov|dec)\\b\\.?|\\b(?-i:May|March)\\b\\.?|\\b(?:may|march)\\b\\.?\\s+\\d{1,2}\\b|\\b\\d{1,2}\\s*(?:am|pm)\\b|\\bo'clock\\b|\\bmidday\\b|\\bmidnight\\b|\\btonight\\b|\\btomorrow\\b|\\b(?:same|next|this|that) (?:morning|afternoon|evening|night)\\b|\\b(?:at|by|until|till|before|after|from) \\d{1,2}[.:]\\d{2}(?![.:]?\\d)|\\b(?:in|since|until|till|from|by|during|after|before) (?:\\d{3,4}s?\\b|the \\d{3,4}s?\\b)(?!\\s*(?:g|kg|ml|l|p|pence|%|bar|cm|mm|km|m|ft|lb|oz|st|deg|grit|uses?|pages?|words?|steps?|calories|kcal|people|guests?|servings?|slices?|eggs?|metres?|meters?|miles?|yards?|feet|foot|litres?|liters?|grams?|kilos?|acres?|square|long|wide|tall)\\b)|\\b\\d{3,4}\\s*(?:ad|bc|b\\.c\\.)\\b|\\b(?:1[0-9]{3}|20[0-9]{2})\\s*(?:-|to|or|and)\\s*(?:1[0-9]{3}|20[0-9]{2})\\b|\\bthe \\d{1,2}(?:st|nd|rd|th)\\b|\\b\\d{1,2}(?:st|nd|rd|th) of (?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SEASON(stdlib_call("re", "compile", &[py_str("\\b(?:spring|summer|autumn|winter)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK(stdlib_call("re", "compile", &[add(&add(&gattr(&g_CLOCK_SURE(), "pattern"), &py_str("|")), &gattr(&g_SEASON(), "pattern")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_YEAR(py_str("(?!\\s*(?:cal|kcal|calorie|step|mile|km|kilomet|metre|meter|gram|g\\b|kg|ml|litre|liter|watt|lumen|gb|mb|megabyte|pound|dollar|euro|per cent|%)\\w*)(?!\\s+[a-z]{2,}s\\b)"));
    s_DATE(stdlib_call("re", "compile", &[add(&add(&py_str("(?:\\b(?:in|from|since|until|till|by|of|during|between|after|before|around|about|circa|early|late|mid|and|to)\\s+(?:the\\s+)?|^)(?:1[0-9]{3}|20[0-2][0-9])\\b"), &g_NOT_A_YEAR()), &py_str("|\\b(?:in|from|since|during|of|the|by|until|early|late|mid)\\s+(?:\\d{4}s|\\d{1,2}(?:st|nd|rd|th) century)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOUR_WORD(py_str("(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)(?:\\s*(?:am|pm|a\\.m|p\\.m))?"));
    s_CLOCK_ANYWHERE(add(&add(&py_str("(?:\\d{1,2}[:.]\\d{2}|\\d{1,2}\\s*(?:am|pm|am\\.|pm\\.|a\\.m|p\\.m|o'clock)|(?:the\\s+)?\\d{1,2}(?:st|nd|rd|th)\\b|(?:the\\s+)?\\d{1,2}\\s+of\\s+(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)|(?:half|quarter|ten|twenty|five|twenty-five)\\s+(?:past|to)\\s+"), &g_HOUR_WORD()), &py_str(")")));
    s__HOUR_24(py_str("(?:[01]?\\d|2[0-4])(?![\\d:])(?!\\.\\d)(?!\\s*[-\\u2013\\u2014]\\s*\\d)(?!\\s*(?:%|percent|per cent|g|kg|ml|l|p|pence|bar|cm|mm|km|m|ft|lb|oz|st|deg|metres?|meters?|miles?|yards?|feet|foot|litres?|liters?|grams?|kilos?|calories|kcal|acres?|grit|uses?|square|long|wide|tall)\\b)"));
    s_NAMED_CLOCK(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&py_str("(?:\\b(?:at|by|from|until|till|before|after|around|about|on)\\s+"), &g_CLOCK_ANYWHERE()), &py_str("|\\b(?:at|by|from|until|till|before|after)\\s+(?:")), &g_HOUR_WORD()), &py_str("(?!\\s+of\\b)|")), &g__HOUR_24()), &py_str(")|(?:half|quarter|ten|twenty|five|twenty-five)\\s+(?:past|to)\\s+")), &g_HOUR_WORD()), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE(stdlib_call("re", "compile", &[add(&add(&py_str("\\d|\\b(?!one of\\b)"), &g__WORD_NUMBER()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY(stdlib_call("re", "compile", &[py_str("[£$€]|\\b(?:pence|pounds?|quid|dollars?|euros?|cents?|free)\\b|^(?:nothing|nil|nowt)\\b|\\bnothing (?:at all|extra|much|to pay|else|on top)\\b|\\bcosts? nothing\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_RANGE(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("^[^\\w]{0,3}(?:about |around |roughly )?(?:\\d{1,3}|"), &g__WORD_NUMBER()), &py_str(")\\s*(?:-|to|or|\\u2013)\\s*(?:\\d{1,3}|")), &g__WORD_NUMBER()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STAGE(stdlib_call("re", "compile", &[py_str("\\b(?:for|in|over|through|during) the first (?:few |couple of |several )?(?:days|weeks|months|year|bit)\\b|\\bthe first (?:few |couple of )?(?:days|weeks|months)\\b|\\b(?:first|1st) birthday\\b|\\buntil (?:they|he|she) (?:are|is) \\w+\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEVER(stdlib_call("re", "compile", &[py_str("\\b(?:indefinitely|(?:does not|doesn't|do not|don't|will not|won't|never) (?:go(?:es)? off|expire|spoil|perish)|(?:last|lasts|keep|keeps|stay|stays|good) (?:good )?(?:for ever|forever|indefinitely))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE_PHRASE(stdlib_call("re", "compile", &[py_str("^(?:once|twice|(?:three|four|five|six|\\d+)\\s+times|every (?:\\w+ )?(?:day|week|month|year|morning|evening|night)|daily|weekly|monthly|yearly|nightly)(?:\\s+(?:a|an|per|each|every)\\s+(?:day|week|month|year|morning|evening|night))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STAGE_WHEN(stdlib_call("re", "compile", &[py_str("(?:(?:\\bwhen|\\bas soon as)\\s+(?:the|it|its|they|they're|there|a|your|their)\\b|\\bonce\\s+(?:the|it|its|they|they're|there|your|their)\\b)[^.?!]{0,70}?\\b(?:has|have|had|is|are|was|were|shows?|showing|sets?|set|starts?|started|reaches?|reached|begins?|begun|appears?|emerges?|sprouts?|sprouted|flowered|flowers|warms?|warmed|workable|cooled|cools|dried|dries|firm|ready|grown|thickened|risen)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EDGED_PART_OF_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:mid|early|late)[- ]?(?:morning|afternoon|evening|night)\\b|\\b(?:first|last) thing\\b(?!\\s+(?:a|an|the|it|i|you|we|they|he|she|that|this|of|about)\\b)|\\bfirst light\\b|\\bdaytime\\b|\\b(?:at|by|before|after|until|till|around|about) (?:dawn|dusk|nightfall|sunset|sunrise|daybreak)\\b|\\bend of the (?:day|afternoon|evening)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SEASON_WHEN(stdlib_call("re", "compile", &[py_str("\\b(?:in|during|through|throughout|into|over)\\s+(?:the\\s+)?(?:growing|dormant|bare|planting|flowering|nesting|mating|wet|dry|rainy)\\s+season\\b|\\b(?:the\\s+)?growing months?\\b|\\b(?:in|during)\\s+(?:a\\s+|the\\s+)?(?:dry|wet|cold|warm)\\s+spells?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:before|after|until|till|by|at|on|from|in|over|during)\\s+(?:the\\s+)?(?:first|second|third|next|last|that|their|his|her)?\\s*(?:birthday|christmas|easter|new year|weekend|weekday|morning|afternoon|evening|night|breakfast|lunch|teatime|tea|dinner|supper|bedtime|school|school night|work|the day|arrival|takeoff|checkout|check-in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_NAME(stdlib_call("re", "compile", &[py_str("\\b(?:christmas|easter|new year'?s?|bank holiday|birthday|half term|term time|the weekend)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE_OF_A_COUNT(stdlib_call("re", "compile", &[py_str("\\b\\d[\\d,.]*\\s*(?:cal|kcal|calorie|step|rep|unit|mile|km|kilomet|metre|meter)s?\\b(?:\\s+\\w+){0,2}\\s+(?:a|an|per|each|every)\\s+(?:day|week|month|year|morning|evening|night)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MINUTES_AS_A_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:within|for)\\s+(?:a few|a couple of|several|a|one|two|three|four|five|ten)\\b[\\s-]*(?:min|minute)s?\\b|\\ba few (?:min|minute)s\\b|\\bwithin (?:a few |a couple of )?(?:min|minute)s\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_FIXED_TIME(stdlib_call("re", "compile", &[py_str("^(?:no|there'?s no|there is no) (?:fixed|set|exact|hard and fast) (?:time|length|deadline)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE(stdlib_call("re", "compile", &[py_str("\\b(?:every (?:day|week|month|year|morning|evening|night|other (?:day|morning|evening|night))|every few (?:days|weeks)|most (?:days|mornings|evenings|nights|weekends)|once|twice|three times|daily|weekly|monthly|yearly|nightly|\\d+ times)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_AN_AGE(stdlib_call("re", "compile", &[py_str("\\bwhat age\\b|\\bhow old\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AGE_YEARS(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("(?:\\d+|"), &g__WORD_NUMBER()), &py_str(")\\s*(?:-\\s*)?years?\\s*(?:old|of age)\\b|\\b(?:age|aged)\\s+(?:of\\s+)?(?:\\d+|")), &g__WORD_NUMBER()), &py_str(")\\s*years?\\b|^(?:\\d+|")), &g__WORD_NUMBER()), &py_str(")\\s*years?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AGE_MONTHS(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("(?:\\d+|"), &g__WORD_NUMBER()), &py_str(")\\s*(?:-\\s*)?(?:months?|weeks?)\\s*old\\b|\\b(?:about|around|at|by|from|until|till|under|over|aged?)\\s+(?:\\d+|")), &g__WORD_NUMBER()), &py_str(")\\s*(?:months?|weeks?)\\b|^(?:\\d+|")), &g__WORD_NUMBER()), &py_str(")\\s*(?:months?|weeks?)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AGE_BIRTHDAY(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+(?:st|nd|rd|th)|[a-z]+(?:st|nd|rd|th))\\s+birthday\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AGE_NAMED(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:age|aged)\\s+(?:\\d+|"), &g__WORD_NUMBER()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AGE_BARE(stdlib_call("re", "compile", &[py_str("(?<![£$€\\d.,])\\b\\d{1,2}\\b(?![.,]?\\d)(?!\\s*(?:%|minutes?\\b|mins?\\b|hours?\\b|hrs?\\b|days?\\b|pounds?\\b|pence\\b|quid\\b|times?\\b|per\\b|each\\b|of them\\b))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AGE_SPELLED_OPENS(stdlib_call("re", "compile", &[add(&add(&py_str("^(?:"), &g__WORD_NUMBER()), &py_str(")\\b(?=\\s*(?:,|to\\b|is\\b|or\\b|and\\b|-|\\u2013))")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:cost|costs|priced?|prices?|pay|pays|paid|charged|charges? for|afford|fees?|quote|salary|rent)\\b|[\\u00a3$\\u20ac]|\\b(?:pounds|quid|euros?|dollars?|pence|tenner)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_GIVEN(stdlib_call("re", "compile", &[py_str("[\\u00a3$\\u20ac]|\\b\\d+p\\b|\\b(?:pounds|quid|pence|euros?|dollars?|cents?|free|costs?|prices?|charges?|fees?|paid|pay|spend|budget|afford|quote|refund|salary|rent|tenner)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURED(stdlib_call("re", "compile", &[add(&add(&py_str("(?:\\d|\\b"), &g__WORD_NUMBER()), &py_str(")\\s*-?\\s*(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|seconds?|secs?|mugs?|cups?|tablespoons?|tbsps?|teaspoons?|tsps?|grams?|kilograms?|kg|litres?|liters?|ml|pints?|quarts?|per cent|percent|degrees?|miles?|km|metres?|meters?|feet|foot|inches?|times?|pence|pounds?|quid|each|people|servings?)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_NUMBER(stdlib_call("re", "compile", &[add(&add(&py_str("^(?:\\d+|"), &g__WORD_NUMBER()), &py_str(")\\.?$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__FIGURE_HEAD(Py::Int(5));
    s__MEASURE_AFTER(stdlib_call("re", "compile", &[py_str("\\s*-?\\s*(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|seconds?|secs?|mugs?|cups?|tablespoons?|tbsps?|teaspoons?|tsps?|grams?|kilograms?|kg|litres?|liters?|ml|pints?|quarts?|per cent|percent|centigrade|degrees?|miles?|km|metres?|meters?|feet|foot|inches?|times?|pence|pounds?|quid|each|people|servings?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:shop|store|supermarket|chemist|pharmac|post office|bank|building society|council|\\bgp\\b|doctor'?s|surgery|dentist|library|garage|station|market|website|web site|site|app|online|branch|office|centre|center|clinic|hospital|recycling|tip\\b|dump|high street|department|service desk|desk|reception|kiosk|counter|aisle|dealer|showroom|garden centre|petrol station|airport|bus stop|platform|address|tesco|asda|sainsbury|aldi|lidl|morrisons|co-op|boots|argos|screwfix|ikea|amazon|ebay|gov\\.uk|passport office|dvla|jobcentre|citizens advice|charity shop|car boot|facebook marketplace|freecycle|wherever|downstairs|upstairs|outside|indoors|in the garden|in the kitchen|in the loft|in the shed|drawer|cupboard|cabinet|wardrobe|shelf|shelves|rack|hook|peg|locker|safe|pocket|handbag|backpack|rucksack|suitcase|glove ?box|boot of the car|hallway|corridor|landing|garage|shed|loft|attic|basement|cellar|balcony|porch|kitchen|bedroom|bathroom|toilet|utility room|airing cupboard|under the sink|in the cupboard|in the drawer|on the shelf|in the fridge|in the freezer|in the bin|on the wall|on the door|in the hall|by the door|at the back of|at the front of|on top of|behind the|under the|next to the|beside the|inside the)(?:s|es)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_AS_EVIDENCE(stdlib_call("re", "compile", &[py_str("\\b(?:shop|store|supermarket|chemist|pharmac|post office|bank|building society|council|\\bgp\\b|doctor'?s|surgery|dentist|library|garage|station|market|website|web site|site|app|online|branch|office|centre|center|clinic|hospital|recycling|tip\\b|dump|high street|department|service desk|desk|reception|kiosk|counter|aisle|dealer|showroom|garden centre|petrol station|airport|bus stop|platform|address|tesco|asda|sainsbury|aldi|lidl|morrisons|co-op|boots|argos|screwfix|ikea|amazon|ebay|gov\\.uk|passport office|dvla|jobcentre|citizens advice|charity shop|car boot|facebook marketplace|freecycle|wherever|downstairs|upstairs|outside|indoors|in the garden|in the kitchen|in the loft|in the shed)(?:s|es)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_MONEY_GOES(stdlib_call("re", "compile", &[py_str("\\bwhere (?:does|do|did) (?:it all |all )?(?:my|our|the|his|her|your) (?:money|pay|wages|salary|income|budget|earnings|time)\\b[^?]{0,20}\\b(?:go|goes|went)\\b|\\bwhere (?:my|our|the|his|her|your) (?:money|pay|wages|salary|income|budget|time) (?:goes|go|went)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_RELATIVE(stdlib_call("re", "compile", &[py_str("\\bwhere\\s+(?:i|we|you|he|she|they|it|my|your|our|his|her|their)\\b[^?.]{0,20}?\\b(?:am|is|are|was|were|live|lives|stay|stays|sit|sits|work|works|go|goes)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_TO_BE_TOLD_WHERE(stdlib_call("re", "compile", &[py_str("\\b(?:tell (?:me|us)|know|knows|say|says|shows?|explain|remind|remember|any idea|wonder|find|check)\\b[^?]{0,25}\\bwhere\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_NAME(stdlib_call("re", "compile", &[py_str("\\b(?:in|at|to|from|near|of|round|around|towards)\\s+(?:(?:the\\s+)?[A-Z]{2,4}(?![A-Za-z])|(?:the\\s+|a\\s+)?[A-Z][a-z]{2,})")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_KIND(stdlib_call("re", "compile", &[py_str("\\b(?:shops|stores|supermarkets|pharmacies|station|stations|markets|cafes|cafés|booths|branches|clinics|charities|tip|depots|counters|showrooms|kiosks|market hall|market halls|food court|food courts|bakery|bakeries|street food|food van|food vans|multi storey|multistorey|multi-storey)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_AT_HOME(stdlib_call("re", "compile", &[py_str("\\b(?:room|bedroom|bathroom|kitchen|utility|hall|hallway|landing|corridor|garage|loft|shed|porch|doorstep|doormat|doorway|door|stairs|staircase|step|floor|corner|wardrobe|cupboard|drawer|shelf|shelves|bookcase|bookshelf|mantelpiece|windowsill|window sill|bedside|nightstand|bed|sofa|settee|couch|cushion|chair|table|desk|hook|peg|pocket|bag|handbag|rucksack|backpack|glovebox|glove box|footwell|boot|bin|washing machine|tumble dryer|airing cupboard|street|side street|back street|road|car park|park and ride|lay-by|lay by|cul-de-sac|estate|alley|alleyway|walkway|underpass|carpark)s?\\b|\\b(?:on|in|under|behind|beside|next to|top of|bottom of|end of)\\s+(?:the\\s+|your\\s+|my\\s+)?(?:sofa|bed|table|desk|stairs|fridge|freezer|counter|shelf|cupboard|wardrobe|radiator|window|door|mat|hook|hall|car|boot|pushchair|pram|buggy)s?\\b|\\b(?:behind|under|underneath|beside|next to|on top of|in front of|at the bottom of|at the back of|at the front of|at the side of)\\s+(?:the\\s+|your\\s+|my\\s+|a\\s+)?[a-z]{3,}|\\b(?:in the open|up high|off the floor|in the middle of|away from the (?:wall|floor|heat))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_NAME_ANY(stdlib_call("re", "compile", &[py_str("(?:(?<=[:;,]\\s)|(?<=\\bor\\s)|(?<=\\band\\s))[A-Z][a-z]{2,}")]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:in|on|of|by|until|till|from|during|after|before|early|late|mid|and|to)\\s+(?:the\\s+)?(?:january|february|april|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sept|sep|oct|nov|dec)\\b\\.?|\\b(?:in|on|of|by|until|till|from|during|after|before|early|late|mid)\\s+(?-i:May|March)\\b\\.?|\\b(?:mon|tues|wednes|thurs|fri|satur|sun)day\\b\\.?|\\b\\d{1,2}\\s*(?:st|nd|rd|th)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:vet|veterinar|doctor|gp|nurse|pharmacist|chemist|dentist|plumber|electrician|gas safe|engineer|mechanic|solicitor|lawyer|adviser|advisor|accountant|broker|landlord|letting agent|employer|hr|union|citizens advice|council|police|someone|somebody|a professional|specialist|consultant|technician|builder|roofer|physio|optician|midwife|health visitor|social services|ombudsman|registrar|witness|referee|teacher|tutor|your bank|the bank|neighbour|neighbor)(?:s|es)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_NAME(stdlib_call("re", "compile", &[py_str("(?<=[a-z,;(]\\s)[A-Z][a-z]{2,}")]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_RELATION(stdlib_call("re", "compile", &[py_str("\\b(?:mum|mom|mam|dad|mother|father|son|daughter|sister|brother|wife|husband|partner|friend|mate|boyfriend|girlfriend|fianc\\w*|neighbou?r|boss|colleague|manager|supervisor|team leader|line manager|head of (?:year|department|team)|child|kid|baby|gran|grandma|grandad|granny|nan|aunt|uncle|cousin|step(?:son|daughter|mum|dad)|in-laws?|twins?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ROLE_ENDS(py_str("[a-z]{3,}(?:er|or|ist|ian|ant|smith|minder|wright)"));
    s__ROLE_OFFICES(py_str("(?:council|company|authority|agency|department|office|service|board|team)"));
    s__NOT_A_ROLE(call_builtin("frozenset", &[py_tuple(vec![py_str("power"), py_str("water"), py_str("answer"), py_str("number"), py_str("other"), py_str("order"), py_str("letter"), py_str("paper"), py_str("dinner"), py_str("offer"), py_str("cover"), py_str("corner"), py_str("member"), py_str("record"), py_str("filter"), py_str("computer"), py_str("calendar"), py_str("character")])]).unwrap_or_else(|e| exc::raise(e)));
    s__PARTY(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:a|an|the|your|our|their|his|her|my)\\s+(?:[a-z-]+\\s+){0,2}?"), &g__ROLE_ENDS()), &py_str("(?=\\s*(?:,|'s|:))|\\b(?:a|an|the|your|our|their)\\s+(?:[a-z-]+\\s+){0,2}?")), &g__ROLE_OFFICES()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_NAMED(stdlib_call("re", "compile", &[py_str("^[A-Z][a-z]{2,}\\s+(?:is|was|'s)\\s+(?:your|our|his|her|their)\\s+[a-z]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_FULL_NAME(stdlib_call("re", "compile", &[py_str("^[A-Z][a-z]{2,}\\s+(?:[A-Z][a-z]{2,}|[IVXLC]{2,})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MEANS(stdlib_call("re", "compile", &[py_str("\\b(?:means|meaning|stands for|is short for|refers to|is when|is what|is (?:a|an|the)\\s+\\w+|are (?:a|an|the)\\s+\\w+|is called|called|is basically|is just|is simply|is the (?:name|word|yearly|total|amount|cost|price|process))\\b|\\b(?:is|are|means)\\s+(?:\\w+\\s+)?for\\s+[\\\"'\\u201c\\u2018]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEANS_ARTICLE(stdlib_call("re", "compile", &[py_str("^(?:is|are) (?:a|an|the)\\s+\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEANS_INFINITIVE(stdlib_call("re", "compile", &[py_str("^to\\s+\\w+\\s+(?:is|means)\\s+to\\s+\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SETTLED_ON(stdlib_call("re", "compile", &[py_str("\\byou (?:settled on|decided on|went for|opted for|chose|picked|plumped for|said|mentioned|told me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPOT_ON_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:in|on|under|underneath|behind|inside|beneath|beside|alongside|next to|on top of|just under|just behind|halfway up|partway up)\\s+(?:the|a|an|its|your|each|either|one)\\s+([\\w'-]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPOT_PART(call_builtin("frozenset", &[mcall(&py_str("\nplate plates sticker stickers label labels badge case casing housing shell body flap lid cover\npanel door doors seal seals drawer box compartment slot port socket plug valve hose pipe tube\ndrum filter cartridge cap knob handle handles shelf rack tray trays wand bag packaging manual\nleaflet screen dial switch button hinge underside underside base bottom top side sides back\nfront rear corner edge middle centre center bracket clamp nut bolt screw seam cord cable wire\nspout nozzle brush blade bar rim trim guard grille vent clip strap tie\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPOT_OF_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:on|at|in|under|near)\\s+the\\s+(?:back|front|top|bottom|side|underside|base|rear|middle|centre|center|corner)\\s+of\\s+(?:the|a|an|its|your)\\s+[\\w'-]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_JUDGEMENT(stdlib_call("re", "compile", &[py_str("^\\s*(?:it|that|this|they|these|those|there|he|she|you|we|i)\\s+(?:is|are|was|were)\\s+(?:a|an|the)\\s+(?:real|good|nice|fine|great|bad|big|small|awful|lovely|honest|fair|decent|brilliant|rubbish|terrible|wonderful|amazing|perfect|silly|daft|sad|happy|serious|proper|big|tough|hard|easy|common|rare)\\s+\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEFINES(stdlib_call("re", "compile", &[py_str("^(?!\\s*(?:it|that|this|they|these|those|there|he|she|i|we|you|what|which|here|so|and|but|do|did|does|don'?t|doesn'?t|didn'?t)\\b)(?:[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,4}\\s+(?:is|are|mean|means)\\b|[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,4}\\s+(?:has|have)\\b(?!\\s+to\\b))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEANS_BY_DOING(stdlib_call("re", "compile", &[py_str("^(?:\\w+ ){0,3}(?:give|gives|provide|provides|let|lets|allow|allows|help|helps|names)\\s+(?:you|a|an|some|free|people|anyone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MECHANISM(stdlib_call("re", "compile", &[py_str("\\bis how\\b|\\bworks? by\\b|\\bworks? like\\b|\\bworks? because\\b|^(?:(?:the|a|an)\\s+[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,3}\\s+|it\\s+)(?:has|have)(?!\\s+to\\b)|(?:uses|consists of|contains|comes from|breaks|pulls|pushes|turns|carries|draws|sucks|lifts|heats|cools|reacts|sends|absorbs|filters|moves|moved|stores|dumps|spins|tumbles|bends|drives|releases|fills|pumps|blows|burns|melts|freezes|bounces|blocks|traps|spreads|grows|charges|recharges|squeezes|stretches|swings|shakes|rotates|grinds|crushes|mixes|separates|holds|held|takes in|gives out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOURCE_MAKES(stdlib_call("re", "compile", &[py_str("^(?:\\w[\\w'\\u2019-]*[ ,]+){1,6}(?:make|makes|made|lay|lays|laid|grow|grows|produce|produces|give|gives)\\s+(?:it|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOURCE_COLON(stdlib_call("re", "compile", &[py_str("^(?!\\s*(?:it|that|this|there|they|you|we|i)\\b)[\\w'\\u2019,()/-]+(?:\\s+[\\w'\\u2019,()/-]+){1,7}:\\s*\\w"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOURCE_MADE(stdlib_call("re", "compile", &[py_str("^[\\w'\\u2019,()-]+(?:\\s+[\\w'\\u2019,()-]+){0,12},\\s*(?:boiled|crushed|heated|picked|shorn|wilted|dried|melted|grown|laid|dug|pressed|cooked|washed|cut|made|then|and then)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOURCE_DOES(stdlib_call("re", "compile", &[py_str("^[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,3}\\s+(?:soaks|runs|flows|comes|grows|forms|makes|gives|produces|holds|carries|falls|melts|boils|ferments|rots|breaks|builds|lays)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOURCE_LIST(stdlib_call("re", "compile", &[py_str("^(?!\\s*(?:it|that|this|there|they|you|we|i)\\b)[\\w'\\u2019-]+(?:,\\s*[\\w'\\u2019-]+){2,}(?:\\s+and\\s+[\\w'\\u2019-]+)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OPINION_OF_THE_THING(stdlib_call("re", "compile", &[py_str("^(?:that|this|it|they|these|those|he|she|you|we)(?:'s|\\u2019s| is| are| was| were)\\s+(?:a|an)\\s+(?:real|proper|genuine|right|good|great|big|small|huge|little|nice|fair|tough|hard|easy|lovely|brilliant|terrible|awful|total|complete|classic|common|usual|rare|dangerous|safe|useful|useless|important|serious|key|major|minor|main|whole|full|big|decent|solid|mucky|filthy|messy)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_DEFINES(stdlib_call("re", "compile", &[py_str("\\b(?:was|were)\\s+(?:a|an|the)\\s+[a-z][\\w'\\u2019-]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SUBJECT(stdlib_call("re", "compile", &[py_str("(?:^|[^\\w'\\u2019-])(?:it|that|this|there|these|those|he|she|they|we|you|i|who|which|what|one|none|everyone|someone|anyone|nobody|nothing|something|anything|everything|my|your|his|her|its|our|their|mine|yours|theirs)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_OPEN(stdlib_call("re", "compile", &[py_str("^(?:yes|yeah|yep|yup|no|nope|nah|not|never|don'?t|do not|doesn'?t|avoid|skip|sure|definitely|absolutely|probably|usually|generally|normally|often|mostly|sometimes|better|best|fine|safe|okay|ok|it'?s|it is|that'?s|that is|they'?re|there'?s no|you'?re|you (?:can|could|should|don'?t|do not|does not|did not|won'?t|can'?t|shouldn'?t|need|want|have to|must)|bin|throw|chuck|toss|dump|discard|refrain|none|nothing|straight|warm|cold|plain|just|private|confidential|anonymous|between us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_PERSON_VERDICT(stdlib_call("re", "compile", &[py_str("^i (?:would|wouldn'?t|do|don'?t|will|won'?t)\\b(?!\\s+(?:only|just|rather|say that))|^i'?d (?:say|skip|wait|hold off|bother|not bother|avoid|keep|leave|go for|get|change|stick with|try)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ECHO_VERDICT(stdlib_call("re", "compile", &[py_str("^(?:(?:still|that'?s|that is|you'?re|you are|absolutely|dead) right|correct|exactly|spot on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_BAND(stdlib_call("re", "compile", &[py_str("(?:inside|within|outside)\\s+(?:the\\s+|well\\s+)?(?:normal|usual|average|expected|recommended|accepted|healthy)\\b(?!\\s*(?:use|uses|conditions?|circumstances?|operation|working|service|wear|load))|(?:inside|within|outside|above|below|over|under)\\s+the\\s+(?:range|band|limits?)\\b|(?:above|below|over|under)\\s+the\\s+(?:normal|usual|recommended|expected)\\b|within reach\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_INSIDE(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:yes|no need|not really|not worth|worth it|isn'?t worth|a bad idea|a good idea|\\bworth (?:repairing|fixing|doing|buying|getting|having|paying|spending|keeping|servicing|replacing|renewing|hiring|installing|the money|the effort|a go)\\b|\\bmeans (?:no|yes|not)\\b|\\b(?:keeps?|lasts?) (?:for|until|about|roughly|easily|well|a couple|a few|a good|\\d)\\b|harmless|safe|fine|okay|avoid|go ahead|of course|by law|make sure|only|is (?:normal|common|usual|typical|expected|ordinary|fine|safe|okay|harmless|allowed|legal)|is (?:good|fine|ok|okay|great|safe|bad) (?:with|at|for|in)\\b|is (?:usually|normally|generally|probably|often) (?:just |fine|normal|harmless|safe|okay)|\\bis (?:almost |nearly )?(?:always |usually )?the (?:cheaper|cheapest|better|best|quicker|quickest|easier|easiest|safer|safest|simpler|simplest) (?:way|option|choice|bet|buy|route|one)\\b|\\b(?:works? out|comes? out|turns? out) (?:cheaper|cheapest|better|best|quicker|easier|safer)\\b|suits|works well|works fine|no harm|nothing wrong|nothing to worry|you'?ll be fine|it depends|not (?:a problem|necessary|needed|allowed)|(?:do not|does not|don'?t|doesn'?t) need\\b|(?:is|are|does) not\\b|\\b(?:isn'?t|aren'?t|doesn'?t)\\b|harmful|unhealthy|healthy|bad for|good for|not good for|risky|dangerous|unsafe|not safe|a lot|too much|too late|not enough|plenty|worth (?:a go|doing)|the one to (?:pick|avoid))\\b|"), &gattr(&g_VERDICT_BAND(), "pattern")), &py_str("")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HARM_IS_WHAT_IT_IS(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were) (?:a|an) (?:real|serious|genuine|proper|major|big|known|health|significant|grave) (?:risk|hazard|danger|problem|no-no)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUALITY_IS(stdlib_call("re", "compile", &[py_str("\\bis (?:really |very |always |usually )?(?:great|good|fine|perfect|brilliant|lovely|ideal|free|cheap|easy|worth it|worthwhile)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IT_HELPS(stdlib_call("re", "compile", &[py_str("\\b(?:it|this) (?:really |usually |often )?(?:helps?|works?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WORKING(stdlib_call("re", "compile", &[py_str("\\b(?:work|works|working|worked|help|helps|helping|use|using|used|worth|bother|good)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PRONOUN_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("they"), py_str("these"), py_str("those"), py_str("he"), py_str("she")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_ABOUT_A_QUALITY(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|isn'?t|aren'?t)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__QUALITIES(py_tuple(vec![py_str("worth it"), py_str("worthwhile"), py_str("normal"), py_str("typical"), py_str("ordinary"), py_str("common"), py_str("okay"), py_str("ok"), py_str("safe"), py_str("alright"), py_str("legal"), py_str("allowed"), py_str("healthy"), py_str("necessary"), py_str("essential"), py_str("fine")]));
    s__QUALITY_SAID({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g__QUALITIES()) { let mut q: Py = __item.clone(); __out.push((q.clone(), stdlib_call("re", "compile", &[modulo(&py_str("\\b%s\\b"), &stdlib_call("re", "escape", &[q.clone()]).unwrap_or_else(|e| exc::raise(e))), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))); } py_dict(__out) })() });
    s__THE_VERDICT_IS_EARLY(Py::Int(12));
    s__QUALITY_OF_A_THING(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__QUALITIES()) { let mut q: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("worth it"), py_str("worthwhile")]), &q.clone()))) {  __out.push(q.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__COPULA(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("isnt"), py_str("arent"), py_str("wasnt"), py_str("werent"), py_str("be"), py_str("feels"), py_str("feel"), py_str("looks"), py_str("look"), py_str("seems"), py_str("seem"), py_str("sounds"), py_str("sound"), py_str("gets"), py_str("stays")])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_INSIDE_QUALITY(stdlib_call("re", "compile", &[py_str("\\bis (?P<q>normal|common|usual|typical|expected|fine|safe|okay|harmless|allowed|legal)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUNT(stdlib_call("re", "compile", &[py_str("^(?:i'?m not sure|i am not sure|i don'?t know|i do not know|i can'?t|i cannot|can'?t call|that'?s beyond|that is beyond|that'?s all|that is all|there is no more|beyond what i know|not mine to|not a call|not one i can call|a quick search|searching for|i'?d rather not|you haven't told me|that'?s a good question|good question|no idea|not something i|no (?:figure|number|price|answer|idea|verdict|view|opinion|guess) from me|say (?:what (?:you(?:'?re| are)? (?:asking|after|on about|worried about|unsure)|you mean|you want to know|has you (?:asking|wondering))|which|that again|more|me more|a bit more)\\b|tell me (?:what|which|more) (?:you|has|is|part)\\b|no (?:first[- ]hand )?(?:knowledge|idea|clue) (?:of|on|about|here|from me)\\b|^nothing (?:in|about) (?:that|this|it|what you (?:said|told me|described))\\b[^.!?]{0,60}?\\b(?:say|says|said|tell|tells|told|show|shows|showed|suggest|suggests|indicate|indicates|go on)\\b|^nothing here (?:to go on|that (?:says|tells))\\b|^hard to (?:put a (?:scale|number) on|say how|judge|tell how|gauge)\\b|i'?m not sure (?:what|how|which|whether|where) to|i'?m not sure without)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HANDOFF(stdlib_call("re", "compile", &[py_str("^(?:(?:that'?s|that is|it'?s|it is) (?:your|a) call(?![a-z])|not (?:a )?call (?:i|we) (?:can|could|would)|(?:that'?s|that is) (?:one|a question|something) for (?:your|a|the)\\b|(?:that'?s|that is) (?:everything|all) i (?:have|know)\\b|(?:that'?s|that is|it'?s|it is) not (?:mine|for me) to (?:answer|say|tell|call|judge)\\b|i would only repeat myself|nothing more i can (?:say|add|tell)\\b|(?:that'?s|that is) not for me to|whether (?:it|that|they|this|[0-9]+|[a-z]+) (?:is|are|was|will be|would be) (?:worth|a lot|too much|too long|too far|any good|safe|normal|fine)\\b|hold (?:it|that|this|the price|[0-9]+)\\b[^.]{0,30}\\bagainst (?:one|another|a|other)\\b|only you can (?:weigh|judge|say|decide|call)\\b|(?:that'?s|that is|it'?s|it is) yours to (?:weigh|say|decide|call|judge)\\b|that part(?:'?s| is) (?:yours|up to you)\\b|(?:i'?ll|i will) leave (?:that|this|it) to (?:your|a|the|somebody|someone)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFERS_BACK(stdlib_call("re", "compile", &[py_str("^\\s*(?:i wouldn'?t call|i would not call|nothing in that|hard to|no way to|not something i can)\\b[^.!?]{0,80}?\\b(?:answer|step|thing|doing) above\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOURCE(py_str("(?:page|web ?site|site|app|account|portal|label|packet|packaging|manual|instructions|leaflet|paperwork|statement|bill|letter|receipt|booking|policy|schedule|timetable|council|gp|doctor|dentist|vet|surgery|pharmacy|provider|supplier|landlord|employer|insurer|bank|met office|forecast|meter)"));
    s_DIRECTED(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:your|their|its|the)\\b[^.!?]{0,40}\\b"), &g_SOURCE()), &py_str("\\b[^.!?]{0,60}\\b(?:will|would|should|can|does)\\s+(?:say|tell|show|give|have|confirm|list|explain)\\b|\\b")), &g_SOURCE()), &py_str("\\b[^.!?]{0,40}\\b(?:has|have|carries|lists|shows|gives)\\s+(?:it|them|you|the (?:day|date|time|figure|price|number|amount|answer|rate))\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOLDER(stdlib_call("re", "compile", &[py_str("\\b(?:your|their|its|the|a|an)\\b[^.!?]{0,30}\\b(?:app|e-?mail|bill|statement|account|page|web ?site|site|letter|receipt|label|packet|packaging|manual|instructions|paperwork|booking|confirmation|tracking|policy|meter|portal|reference number)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_QUOTE(py_str("\"“”"));
    s__NOT_A_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("you"), py_str("your"), py_str("yours"), py_str("yourself"), py_str("yourselves")])]).unwrap_or_else(|e| exc::raise(e)));
    s__WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s__NO_WAY_TO_KNOW(stdlib_call("re", "compile", &[py_str("(?:you'?ve not said|you haven'?t said|you have not said|only you know|tell me and i'?ll remember|i only get what you type|no way for me to know)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HYPOTHETICAL(stdlib_call("re", "compile", &[py_str("\\bif (?:i|we|you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OUTCOME_ASK(stdlib_call("re", "compile", &[py_str("^(?:and )?(?:will|would|does|do|did)\\b|^if\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATES(stdlib_call("re", "compile", &[py_str("\\b(?:not|n't|never|no longer|nothing|none|nowhere)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_NO(stdlib_call("re", "compile", &[py_str("(?:^|[,;:\\u2014-]\\s*)(?P<subject>[^,;:.!?]{1,40}?)\\s+(?:is|are|was|were|does|do|did|will|would|can|could|has|have|needs?|takes?)\\s+(?:not|never|n't)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:don'?t|do not|never|avoid|skip|stop|no need to)\\s+([a-z]{3,})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("not"), py_str("never"), py_str("nothing"), py_str("none"), py_str("does"), py_str("doesn"), py_str("dont"), py_str("don"), py_str("did"), py_str("didn"), py_str("isn"), py_str("aren"), py_str("wasn"), py_str("weren"), py_str("can"), py_str("can’t"), py_str("cant"), py_str("cannot"), py_str("won"), py_str("would"), py_str("should"), py_str("could"), py_str("have"), py_str("has"), py_str("had"), py_str("need"), py_str("needs"), py_str("want"), py_str("wants"), py_str("why"), py_str("how"), py_str("what"), py_str("when"), py_str("where"), py_str("which"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("they"), py_str("them"), py_str("there"), py_str("their"), py_str("your"), py_str("you"), py_str("and"), py_str("but"), py_str("for"), py_str("with"), py_str("into"), py_str("from"), py_str("about"), py_str("just"), py_str("like"), py_str("much"), py_str("more"), py_str("less"), py_str("than"), py_str("then"), py_str("only"), py_str("really"), py_str("still"), py_str("even")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRAILING_VERDICT(stdlib_call("re", "compile", &[py_str("[,\\-\\u2013\\u2014]\\s*(?:so|then|but|which means|that means|meaning)\\s+(?:no|yes|nope|not really|definitely not|probably not|of course|not worth it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SEEKS_HELP(stdlib_call("re", "compile", &[py_str("\\b(?:see|ring|call|contact|visit|speak to|talk to|get seen by|book (?:an appointment )?with)\\s+(?:a|an|the|your|nhs|out of hours)?\\s*(?:gp|doctor|dentist|vet|111|999|urgent care|walk-in cent(?:re|er)|midwife|health visitor|pharmacist|optician|nurse|surgery|hospital)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_IF_SERIOUS(stdlib_call("re", "compile", &[py_str("\\b(?:dangerous|serious|a problem|a worry|worrying|worry about|something to worry|should i worry|normal|abnormal|a sign of|a concern)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TEST_IS_THE_VERDICT(stdlib_call("re", "compile", &[py_str("comes? out clean|is done when|is ready when|is cooked when|when it'?s? (?:firm|golden|clean|set|done)|if it (?:looks|feels|comes|smells) (?:done|cooked|firm|golden|clean|set|right)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERMISSION_RULE(stdlib_call("re", "compile", &[py_str("\\b(?:can|cannot|can'?t|could|couldn'?t|may|must|should|need(?:s)? to) be [a-z]+ed\\b|\\bcosts? nothing\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:is|are)\\s+(?:yours|your own|for you to|one for you|a (?:diy|weekend|home|simple) job)\\b|\\b(?:you|yourself) (?:can|could) (?:do|change|replace|fit|swap|fix|service|repair)\\b|\\byourself\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRADE(stdlib_call("re", "compile", &[py_str("\\b(?:engineer|electrician|plumber|gas safe|registered|qualified|competent|specialist|professional|trade)\\w*\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MUST_BE_A_TRADE(stdlib_call("re", "compile", &[py_str("\\b(?:has|have|needs?|must|should)\\s+to\\s+(?:be\\s+)?(?:done\\s+)?by\\b|\\bnot a (?:diy|home) job\\b|\\b(?:by|get) an? (?:gas safe|registered|qualified)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RULE_CONDITION(stdlib_call("re", "compile", &[py_str("\\b(?:if|when|while|once|until|unless|over about|under about|more than about|less than about|half (?:of )?the price)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RULE_KEEP_IT(stdlib_call("re", "compile", &[py_str("\\b(?:fix|repair|keep|mend|service)\\s+it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RULE_DROP_IT(stdlib_call("re", "compile", &[py_str("\\b(?:bin|replace|scrap|chuck|throw)\\s+it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RULE_GO_NEW(stdlib_call("re", "compile", &[py_str("\\b(?:put (?:the |that )?money towards|go for|buy|get|fit|have|bring in)\\b[^.;:]{0,24}\\b(?:a |the )?new\\b|\\bbuy new\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_ECHO(stdlib_call("re", "compile", &[py_str("^(?:it|that|this|they|these|those|he|she) (?:does|do|did|will|would|can|could|should|doesn'?t|don'?t|didn'?t|won'?t|can'?t|shouldn'?t)\\b|^(?:you|we) (?:would|will|do|did|can|could|should) not\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_RULE(stdlib_call("re", "compile", &[py_str("\\byou(?:'?re| are) (?:owed|entitled|covered|allowed|within your rights)\\b|\\byou (?:qualify|are owed|are entitled|can claim|claim|can apply|apply for|are covered|do not need|don't need|get it|can get one|can have one)\\b|\\bgets? (?:\\d[\\d,.]*|a|the|half|a third|one) (?:per cent|percent|%)\\b|\\bis the wrong (?:choice|one|thing|call)\\b|\\bthere is no (?:table service|need|charge|fee|cost)\\b|\\bnothing to (?:tip|pay|declare|owe|worry about)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_AFTER_A_CASE(stdlib_call("re", "compile", &[py_str("^(?:for|in|on|with|at|if|when|unless|after|before)\\b[^,.;:!?]{1,40},\\s*(?:yes|yeah|yep|yup|no|nope|nah|not|never|definitely|absolutely|probably|usually|generally|normally|mostly|it'?s|it is|that'?s|that is|you'?re|you (?:can|could|should|don'?t))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WINNER_NAMED(stdlib_call("re", "compile", &[py_str("\\bis the (?:cheaper|better|worse|cheapest|best|worst|safer|safest|healthier|healthiest|stronger|weaker|easier|harder) (?:one|option|choice|bet|buy|of the two)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_ABOUT_IT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were) (?:the|one of the|a) (?:better|best|safest|kindest|cheapest|healthiest|easiest|simplest|nicest|worst|strongest|tastiest|kinder|safer|cheaper|healthier|easier)\\b|\\bis a good (?:way|idea|choice|bet|pick|option|buy|start|investment)\\b|\\bis the one (?:to|for|that|you)\\b|\\bworth (?:reading|knowing|having|checking|watching|visiting|seeing|a look|a read)\\b|\\b(?:if|unless|when)\\b[^.;]{0,90},\\s*(?:then\\s+)?(?:go|go ahead|go for it|yes|no|fine|okay|safe|skip|skip it|leave it|let it go|do it|take it|avoid it|worth it|try it|it'?s (?:fine|okay|safe|worth it)|that'?s (?:fine|okay|safe)|you'?re (?:fine|okay|safe))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NAMING_WORD(stdlib_call("re", "compile", &[py_str("[a-z]{4,}")]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMING_STOP(call_builtin("frozenset", &[mcall(&py_str("\nthat this with have your them when what just like then than also make need from into over been\nwere will would could should about there their some more most much very only even still well\ngood best fine time times day days year years work works thing things home back down know want\ntake keep give goes going said says here where while which whose these those being does doing\ndone made many each both after before again always never often sometimes usually really quite\nrather around without within because since until during other another same different little\nsmall large"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REPEAT_LEAD(stdlib_call("re", "compile", &[py_str("^(?:same as before|same answer(?:,? and it covers this)?|same as i said|as before|as i said|like i said|same thing|same again)\\s*[:,]?\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIGN_OFF_LEAD(stdlib_call("re", "compile", &[py_str("^(?:(?:that'?s|it'?s|this is)\\s+)?(?:nice|great|lovely|wonderful|perfect|brilliant|excellent|fantastic|amazing|awesome|glad|sorry|fair enough|no rush|no worries|sounds? good|got it|understood|alright then|good luck|hope|thanks|thank you|of course|cheers)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SOCIAL_OPEN(stdlib_call("re", "compile", &[py_str("^(?:have|enjoy|hope|good luck|take care|cheers|thanks|thank you|hi|hello|hey|morning|goodnight|bye)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_ON_A_SCALE(stdlib_call("re", "compile", &[py_str("\\b(?:inside|within|outside|past|beyond|above|below|at the top of|at the bottom of)\\s+(?:the\\s+|its\\s+)?(?:normal|usual|typical|expected|average|recommended|healthy)(?:\\s+(?:range|band|limits?|age|window|spread|side))?\\b|\\bon the (?:late|early|young|old|small|large|big|high|low|heavy|light|slow|fast|short|long|tricky|tight|generous|easy) side\\b|\\bborderline\\b|\\bexactly on time\\b|\\bnothing (?:unusual|abnormal|out of the ordinary)\\b|\\bstill (?:inside|within|in)\\b.{0,20}\\b(?:normal|range)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIMIT_ON_ITS_USE(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|lasts?|stays?|stored?|freezes?)\\b[^.\\d]{0,20}\\b(?:\\d[\\d.,]*\\s*(?:to|-|or)\\s*)?\\d[\\d.,]*\\s*(?:days?|hours?|weeks?|months?|years?|fortnights?)\\b|\\b(?:keep|keeps|kept|take|takes|move|put|leave|shut|block)\\b[^.]{0,45}?\\b(?:out of|away from|well away)\\b|\\b\\w+\\b[^.]{0,30}\\bunless\\b|\\b(?:up to|no more than|at most|not more than|until about|until they|under about)\\b[^.]{0,12}?\\b\\d[\\d.,]*\\s*(?:kg|kgs|g|grams?|cm|mm|ml|litres?|inches?|years?|months?|weeks?|days?|hours?|minutes?|mins?|seconds?|decibels|db|degrees?)\\b|\\b\\d[\\d.,]*\\s*(?:to|-|or)\\s*\\d[\\d.,]*\\s*(?:minutes?|mins?|hours?|seconds?)\\b[^.]{0,12}?\\bat a time\\b|\\b(?:within|inside|reheat(?:ed)?\\b[^.]{0,15}?until)\\b[^.]{0,20}?(?:\\d[\\d.,]*\\s*(?:hours?|minutes?|days?)|steaming|piping hot|boiling)\\b|\\b(?:it|they|that|this)\\b[^.]{0,12}?\\b(?:is|are)\\b[^.]{0,12}?\\b(?:fine|safe|ok|okay|alright|harmless)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BOUND_THAT_IS_THE_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|lasts?|stored?)\\b[^.\\d]{0,20}\\b(?:\\d[\\d.,]*\\s*(?:to|-|or)\\s*)?\\d[\\d.,]*\\s*(?:days?|hours?|weeks?|months?|years?|fortnights?)\\b|\\b(?:use|eat|drink|keep|finish|consume)\\b[^.]{0,25}?\\bwithin\\b[^.]{0,15}?\\d[\\d.,]*\\s*(?:days?|hours?|weeks?)\\b|\\b(?:no more than|up to|at most)\\b[^.]{0,12}?\\b\\d[\\d.,]*\\s*(?:kg|kgs|g|grams?|ml|units?|decibels|db|degrees?|hours?|minutes?)\\b|\\b\\d[\\d.,]*\\s*(?:to|-|or)\\s*\\d[\\d.,]*\\s*(?:minutes?|mins?|hours?|seconds?)\\b[^.]{0,12}?\\bat a time\\b|\\b\\w+\\b[^.]{0,25}\\bunless\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATED_AUX(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|do|does|did|have|has|had|can|could|will|would|should|must|need|needs|kills?|harm|hurt|matter|work|works)\\s*(?:not|n'?t)\\b|\\b(?:isn'?t|aren'?t|wasn'?t|weren'?t|don'?t|doesn'?t|didn'?t|haven'?t|hasn'?t|can'?t|cannot|won'?t|wouldn'?t|shouldn'?t)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_USUALLY_IS(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\s+(?:usually|often|nearly always|almost always|mostly|generally|typically|most likely|in most cases)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PREFERS(stdlib_call("re", "compile", &[py_str("^(?:replace|repair|rebuild|service|swap|install|keep|buy|rent|hire|book|order|get|go for|go with|take|choose|pick|stick with|read|watch|visit|try|start with|hold off|wait)\\b|\\b(?:better|best|rather|instead|go for|go with|opt for|choose|pick|stick with|you (?:mentioned|said|named|had)|you'?re on|\\b(?:buy|get|fit|install|replace|repair|swap|go for|go with) (?:the|a|an|one|it)\\b|i'?(?:d| would) (?:go|do|make|cook|have|take|get|pick|choose|suggest|recommend|buy|wear|watch|read|order|book|serve|use|say|stay|leave|keep|skip|cancel|wait|ring|call)|worth (?:the extra|it)|either .{0,40}\\bor\\b|\\bworth (?:repairing|replacing|fixing|doing|keeping|buying|getting|upgrading|waiting|swapping|having|sticking with)\\b|both are|both work|depends on|for (?:a|the) .{0,30}\\band\\b.{0,30}\\bfor\\b)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BALD(stdlib_call("re", "compile", &[py_str("^(?:the|a|an|this|that|[A-Z][a-z]+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BALD_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|has|have|holds?|works?|comes?|goes?|keeps?|takes?|lasts?|costs?|means|does|do|will|can)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REASON_OPENS(stdlib_call("re", "compile", &[py_str("^(?:because|cos|'cause|that'?s why|that is why|since|so that|which is why)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VERB_THEN_THE_THING(stdlib_call("re", "compile", &[py_str("^(?!(?:the|a|an|this|that|these|those|your|my|our|their|his|her|its|some|any|each|every|one|two|both)\\b)[a-z]+\\s+(?:it|them|the|a|an|your|this|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_HEAD(stdlib_call("re", "compile", &[py_str("^(.{2,70}?):\\s"), stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_HEAD(stdlib_call("re", "compile", &[py_str("^(?:how|what|why|when|where|which|who|steps?|first|firstly|then|next|to|tldr|quick tips?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LABEL_HEAD(stdlib_call("re", "compile", &[py_str("^(?:quick|best|cheapest|easiest|fastest|faster|fast|simplest|simple|cheap|easy|options?|ideas?|while you [a-z]+|bonus|top|also|first|firstly|then|next|tip|tips|note|tl;?dr|answer|short answer|in short|verdict|my pick|worth it)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SET_LABEL(stdlib_call("re", "compile", &[py_str("^(?:quick|best|cheapest|easiest|fastest|faster|fast|simplest|simple|cheap|easy|options?|ideas?|while you [a-z]+|bonus|top|also|first|firstly|then|next|tip|tips|note|worth it)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ECHO_HEAD(stdlib_call("re", "compile", &[py_str("^(?:same as (?:before|last time)|as (?:before|above)|like before|as i said|same|same again|still|unchanged|no change)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_HEAD(stdlib_call("re", "compile", &[py_str("^(?:you'?ve got it|you got it|i (?:like|liked|love|prefer) (?:it|them|that|those)|that'?s (?:it|right|the one)|this one|as i said|like before|same as (?:before|last time))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STATES_HEAD(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|means|sounds?|seems?)\\b|\\b(?:it|that|here|there|what)'s\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_HEAD(stdlib_call("re", "compile", &[py_str("^(?:here'?s|think of it|imagine|to be fair|the (?:truth|thing|point|trick|catch|problem|issue|trouble|trouble is)|honest\\w*|the way i see it|what you want)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PICKS_BY_COMPARISON(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were) (?:the )?(?:longer|shorter|bigger|smaller|taller|wider|deeper|heavier|lighter|faster|slower|quicker|cheaper|dearer|pricier|warmer|colder|hotter|newer|older|nicer|easier|harder|safer|stronger|thicker|thinner)\\b|\\b(?:is|are) (?:exactly |about |nearly |roughly )?twice as\\b|\\bis the (?:one|answer|pick|choice|setting|winner|default)\\b(?!\\s+(?:that|which|who)\\b)|\\bare the thing\\b|\\bthe thing to (?:get|buy|have|do|go for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PICK_BY_CASE(stdlib_call("re", "compile", &[py_str("\\bif\\b[^:]{0,90}\\b(?:for (?:anything|everything) else|otherwise|else)\\b|\\bif not\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUOTED_NAME(stdlib_call("re", "compile", &[py_str("'[^'\\n]{2,40}'|\\u2018[^\\u2019\\n]{2,40}\\u2019")]).unwrap_or_else(|e| exc::raise(e)));
    s_WOULD(stdlib_call("re", "compile", &[py_str("\\bi'd (?:do|make|cook|have|wear|say|keep|buy|book|order|get|take|go|try|use|put)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LABEL(stdlib_call("re", "compile", &[py_str("^([^:]{1,90}):\\s+(\\S.+)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_LABEL(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|means?|reason|cause|because|depends?|you'?ve got|same as|as before|good question|it'?s|that'?s|there'?s|here'?s|^i (?:like|liked|love|prefer) (?:it|them|that|those)|^both|^same)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_IN_EMPHASIS(stdlib_call("re", "compile", &[py_str("^\\*{1,2}[^*\\n]{2,70}\\*{1,2}(?!\\*)(?!\\s*(?:is|are|was|were|means|refers|describes|comes)\\b)(?!\\s*:)")]).unwrap_or_else(|e| exc::raise(e)));
    s_DOES(stdlib_call("re", "compile", &[py_str("\\b(?:first|firstly|then|next|after that|afterwards|finally|lastly|start by|start with|begin by|once (?:it|that|you)|step one|step 1|before you|while you|leave it|put it|take it|turn it|give it|run it|use a|use the|you (?:could|can|should|need to) (?:try|use|start|put|take|run|give|leave|set|check))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INSTRUCTS(stdlib_call("re", "compile", &[py_str("^(?:open|book|ring|call|phone|top|wipe|soak|give|leave|put|take|turn|run|use|set|check|clean|wash|rinse|melt|stir|mix|fry|boil|bake|roast|grill|simmer|add|pour|spread|cut|chop|slice|peel|grate|whisk|beat|knead|roll|press|heat|cook|drain|freeze|cool|cover|store|keep|throw|bin|recycle|replace|repair|fit|fix|install|service|bleed|flush|tie|wrap|apply|ask|tell|send|email|visit|go|start|finish|stop|avoid|make|buy|order|measure|weigh|label|file|register|report|complain|sign|write|read|watch|listen|carry|hold|stand|sit|wait|test|taste|smell|look|choose|pick|swap|follow|keep|get|bring|put|note|note down|squeeze|rub|scrape|buff|dab|blot|scoop|trim|mist|sweep|pat|fold|split|delay|aim|try|hook|wedge|prop|secure|tape|glue|batch|preheat|defrost|pick over|sort out|go through|block|treat|air|cancel|renew|practise|practice)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CALCULATION(stdlib_call("re", "compile", &[py_str("^(?:multiply|divide|subtract|convert|work out|work back|add up|share out|multiply out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DOES_NOT(stdlib_call("re", "compile", &[py_str("^(?:don'?t|do not|never|avoid|skip|stop|no need to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOB(stdlib_call("re", "compile", &[mcall(&gattr(&g_INSTRUCTS(), "pattern"), "replace", &[py_str(")\\b"), py_str("|see|strap|warm|wear|feed|water|brush|rest|drink|eat|sleep|walk|save|pay|spend|charge|plug|reset|update|download|print|scan|iron|sand|paint|screw|seal|glue|tape|hang|fold|dry|find|air|bath|shower|talk|say|reply|text|answer|freeze|chill|cool|defrost|thaw|soak|rinse|change|adjust|scrub|boil|bake|roast|fry|cook|chop|slice|peel|stir|mix|whisk|season)\\b")]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_WHAT_IT_DOES(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("am"), py_str("makes"), py_str("make"), py_str("made"), py_str("helps"), py_str("help"), py_str("needs"), py_str("need"), py_str("seems"), py_str("happens"), py_str("takes"), py_str("gives"), py_str("keeps"), py_str("does"), py_str("comes"), py_str("goes"), py_str("looks"), py_str("sounds"), py_str("stops"), py_str("starts"), py_str("works"), py_str("hurts"), py_str("depends"), py_str("matters"), py_str("causes"), py_str("means"), py_str("prevents"), py_str("beats"), py_str("lowers"), py_str("raises"), py_str("reduces"), py_str("improves"), py_str("eases"), py_str("dries"), py_str("warms"), py_str("cools"), py_str("stays"), py_str("gets"), py_str("matter"), py_str("build"), py_str("builds"), py_str("vary"), py_str("varies"), py_str("cost"), py_str("costs"), py_str("change"), py_str("changes"), py_str("depend")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEANS_NAMED(stdlib_call("re", "compile", &[py_str("^[^:]{0,40}\\b(?:routes?|ways?|options?|methods?|approaches?|choices?)\\b\\s*:"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEANS_OF(stdlib_call("re", "compile", &[py_str("\\b(?:with|using) (?:a|an|some|the)\\b.{0,40}\\b(?:takes? (?:off|out|away)|lifts?|removes?|cleans?|works|does it|gets? (?:rid of|it off))\\b|:\\s*(?:follow|put|take|give|get|ask|ring|call|send|keep|check|book|use|do|start|run|leave|try|add|make|go)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_INFO(stdlib_call("re", "compile", &[py_str("^(?:tell me|say which|let me know|say more)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("it"), py_str("its"), py_str("it's"), py_str("they"), py_str("you"), py_str("your"), py_str("we"), py_str("our"), py_str("i"), py_str("my"), py_str("me"), py_str("he"), py_str("she"), py_str("his"), py_str("her"), py_str("there"), py_str("here"), py_str("what"), py_str("which"), py_str("who"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("if"), py_str("once"), py_str("after"), py_str("before"), py_str("for"), py_str("with"), py_str("on"), py_str("in"), py_str("to"), py_str("of"), py_str("at"), py_str("by"), py_str("from"), py_str("so"), py_str("but"), py_str("and"), py_str("or"), py_str("also"), py_str("then"), py_str("never"), py_str("always"), py_str("most"), py_str("some"), py_str("all"), py_str("every"), py_str("any"), py_str("no"), py_str("not"), py_str("one"), py_str("two"), py_str("both"), py_str("each"), py_str("few"), py_str("many"), py_str("more"), py_str("less"), py_str("very")])]).unwrap_or_else(|e| exc::raise(e)));
    s_IMPERATIVE(stdlib_call("re", "compile", &[py_str("^([A-Za-z']+)\\s+(?:a|an|the|your|their|its|it|them|these|those|some|one|two|each|every|both|all|him|her|me|us|my|out|in|up|down|off|on|away)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THINGS_VERB(stdlib_call("re", "compile", &[py_str("^[^\\w]*(?:take|bring|pack|get|buy|grab|carry|put in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_SUBJECT(bitor(&g_NOT_A_VERB(), &call_builtin("frozenset", &[py_tuple(vec![py_str("does"), py_str("did"), py_str("can"), py_str("could"), py_str("should"), py_str("would"), py_str("will"), py_str("shall"), py_str("was"), py_str("were"), py_str("are"), py_str("have"), py_str("has"), py_str("had"), py_str("get"), py_str("got"), py_str("make"), py_str("made"), py_str("need"), py_str("needs"), py_str("want"), py_str("wants"), py_str("like"), py_str("likes"), py_str("good"), py_str("bad"), py_str("better"), py_str("best"), py_str("okay"), py_str("fine"), py_str("worth"), py_str("still"), py_str("really"), py_str("think"), py_str("know"), py_str("sure"), py_str("same"), py_str("keep"), py_str("keeps"), py_str("take"), py_str("takes"), py_str("mean")])]).unwrap_or_else(|e| exc::raise(e))));
    s_INSTRUCTS_CLAUSE(stdlib_call("re", "compile", &[add(&py_str("(?:^|[.:;!?]\\s+|,\\s+(?:then|so|and|but|just|or)\\s+|,\\s+)(?P<verb>be|"), &geti(&mcall(&gattr(&g_INSTRUCTS(), "pattern"), "split", &[py_str("(?:"), Py::Int(1)]), &Py::Int(1))), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEXT_IDEA(stdlib_call("re", "compile", &[py_str("\\b(?:something|anything) else\\b|\\banother (?:one|option|idea|way|approach|thing)\\b|\\balternatively\\b|\\botherwise\\b|\\bthe other (?:option|idea|one)|\\boption (?:two|2)\\b|\\bsecond option\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIST_AFTER_COLON(stdlib_call("re", "compile", &[py_str(":\\s*\\w[^:]*,\\s*\\w")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_A_LIST(stdlib_call("re", "compile", &[py_str("\\bfor\\b[^,.;:]{2,70},\\s*[^,.;:]{0,70}\\bfor\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_A_TIME(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:about\\s+|around\\s+|roughly\\s+|just\\s+|another\\s+|a good\\s+|at least\\s+|up to\\s+|half an?\\s+)?(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|fifteen|twenty|thirty|forty|fifty|sixty|ninety|a|an|each|every)\\s*-?\\s*(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|nights?|weeks?|months?|years?|while|bit|moment|spell)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SEQUENCES(stdlib_call("re", "compile", &[py_str("\\b(?:then|after that|afterwards|next|finally|once|first)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRAST(stdlib_call("re", "compile", &[py_str("\\b(?:whereas|unlike|both|either|but|while|the difference|differs?|compared|comparison|on the other hand|the other way|instead|rather than)\\b|\\b(?:cheaper|dearer|quieter|louder|faster|slower|easier|harder|bigger|smaller|longer|shorter|better|worse|more|less|stronger|weaker|heavier|lighter)\\b|\\bthan\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTED_LIST(stdlib_call("re", "compile", &[py_str("\\b(?:two|three|four|five|six|a few|several|a couple of|pair of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME_COUNT(stdlib_call("re", "compile", &[py_str("\\b(?:two|three|four|five|six|a few|several|a couple of)\\s+(?:seconds?|minutes?|hours?|days?|weeks?|months?|years?|times|per cent|percent)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LIST_OF_THINGS(stdlib_call("re", "compile", &[py_str("[^:]{0,200}:\\s*[^.]{0,300}"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_VERB(stdlib_call("re", "compile", &[py_str("^(?:plan|block|book|aim)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:because|that'?s why|which is why|the reason|reason is|since|so that|due to|comes? from|caused by|it happens|the cause|as a result|is down to|thanks to|which means|means that|at which|stems? from|leads? to|that'?s what|that is what|this is what|is what makes)\\b|, so\\b|\\b(?:goes?|gets?|turns?|becomes?|comes? out|feels?)\\b.{0,40}\\b(?:when|if|because)\\b|\\b(?:is|are) (?:usually|often|nearly always|mostly|generally|really|just) the\\b|\\b(?:often|usually|sometimes|mostly|generally|nearly always|probably)\\s+means?\\b|\\bso\\b.{0,40}\\bthat\\b|\\b(?:is|are|was|were) (?:the|a|an) .{0,50}\\b(?:rather than|not)\\b|\\b(?:which |that )?happens? (?:when|if|because)\\b|\\b(?:especially|particularly) (?:when|if|in)\\b|\\b(?:is|are|was|were)\\s+\\w+ed\\s+rather than\\b|\\b(?:sticks?|stuck|smells?|smelt|tastes?|fades?|squeaks?|clicks?|wobbles?|sags?|leaks?|cracks?|burns?|turns?|goes?|gets?|comes? out|feels?|looks?|sounds?|happens?|builds? up|collects?|gathers?|settles?|clings?|pools?|spreads?|grows?|stops?|dies?|fails?|drops?|floats?|sinks?|rises?|falls?|heats?|cools?|shrinks?|expands?|swells?|warps?|splits?|melts?|freezes?|curdles?|separates?|wears?|rots?|wilts?|droops?|curls?)\\b.{0,25}\\b(?:when|if|because|with|from)\\b|\\b(?:is|are|was|were|has|have|had)\\s+(?:usually|often|normally|mostly|generally|probably|most likely|nearly always|just|really|only|simply)?\\s*(?:(?:under|over|un)\\w+|\\btoo\\b|\\bnot\\b)\\s*\\w*\\b|\\b(?:has|have|had|did|does|do|is|are|was|were)\\s+not\\s+\\w+\\s+(?:long\\s+)?enough\\b|,\\s*which\\s+(?:is|was|means)\\b|\\b(?:is|are|was|were)\\s+(?:\\w+\\s+)?\\w+\\s*,\\s*\\w+\\s+or\\s+(?:both|so|else)\\b|\\b(?:is|are|was|were)\\b[^.;:!?]{0,40}\\brather than\\b|\\b(?:is|are|was|were)\\s+(?:the|a|an)\\s+\\w+(?:\\s+\\w+){0,4}\\s+(?:(?<!to )\\w+ing|\\w+ed)\\b|\\b(?:is|are|was|were)\\s+(?:\\w+ly\\s+)?\\w+\\s*,\\s*not\\b|,\\s*(?:making|leaving|causing|giving|sending|turning|setting|which\\s+(?:makes|causes|leaves|gives))\\b|\\b(?:which|that)\\s+causes?\\b|\\b(?:one|two|three|four|five|\\d+)\\s+(?:things|reasons|causes|factors|suspects)\\s+\\w|\\b(?:is|are|was|were)\\s+(?:the|a|an)\\s+\\w+(?:\\s+\\w+)?\\s+or\\s+(?:the|a|an)\\b|\\b(?:is|are|was|were)\\s+(?:\\w+\\s+)?\\w+\\s+[-\\u2013\\u2014]\\s+\\w|\\b(?:is|are|was|were)\\s+(?:nearly always|usually|often|mostly|always|almost always)?\\s*(?:the|a|an)?\\s*\\w+(?:\\s+\\w+){0,2}\\s+or\\s+(?:the|a|an)\\b|\\binstead of\\b|\\bpoints? (?:at|to)\\b|\\b\\w+ing\\s+(?:from|off|through)\\b|\\bfrom\\s+(?:too|not|lack of|a lack)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CAUSE_IS(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\s+(?:usually\\s+|often\\s+|mostly\\s+|nearly always\\s+|generally\\s+)?(?:(?:a|an|the)\\s+)?[a-z][a-z-]*(?:\\s+[a-z][a-z-]*){0,3}\\s+(?:of|that|which|from|turning|sitting|going|coming|leaving|building|(?<!than )(?<!more )(?<!less )\\b(?:in|on|at|under))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CAUSE_WHEN(stdlib_call("re", "compile", &[py_str("\\b(?:\\w+s|\\w+ed|is|are|was|were|gets?|got|goes|go|came|comes)\\b\\s+(?:when|if|because)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_THE_CAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:wants?|needs?|craves?|is asking for|is crying out for|is begging for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_OF_THE_PERSON(stdlib_call("re", "compile", &[py_str("^(?:you|i|we|they|he|she|one|people|somebody|someone)\\s+(?:just\\s+|really\\s+|only\\s+)?(?:want|wants|need|needs|will want|will need)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_OF_THINGS(stdlib_call("re", "compile", &[py_str("\\b(?:on|down to)\\s+(?:two|three|four|five|several|a few)\\s+(?:things|reasons|factors|parts|steps|rules)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PURPOSE(stdlib_call("re", "compile", &[py_str("\\bthe point of (?:it |them |this |that )?(?:is|was)\\b|\\b(?:its|their|his|her) (?:job|point|purpose|use) is\\b|\\bthe (?:point|purpose|job|use) is (?:to|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PURPOSE_TO(stdlib_call("re", "compile", &[py_str("^(?:it is |its |the )?(?:to|for) [a-z]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MERIT(stdlib_call("re", "compile", &[py_str("\\bworth it\\b|\\bworth having\\b|\\bworth doing\\b|\\bworthwhile\\b|\\bworth the\\b|\\b(?:usually|generally|probably|mostly) (?:yes|no|worth)\\b|\\b(?:yes|no)\\b[^.!?]{0,40}\\b(?:if|when|unless|for|in a|as long as)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNSETTLED(stdlib_call("re", "compile", &[py_str("\\b(?:(?:nobody|no one|no-one) (?:really )?(?:knows|is sure|can say|agrees)|it is not (?:known|certain|settled|clear)|(?:unclear|unsettled|still debated?|a matter of debate|not fully understood))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EVIDENCE_SAYS(stdlib_call("re", "compile", &[py_str("\\b(?:evidence|studies|research|the work on it|experts|biologists|historians|economists|scientists)\\b[\\w\\s,'\\u2019-]{0,40}?\\b(?:points? (?:at|to)|suggests?|shows?|indicates?|thinks?|suspects?|leans? (?:to|towards)|is that)\\b|\\bbest (?:guess|evidence)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPLAINS(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|means?|comes? from|happens? (?:when|if|because)|because|the reason|causes?|caused|brings?|brought|leads? to|led to|sets? off|triggers?|gives? you|is behind|is down to|stems? from|comes? down to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENDS_THEM_ON(stdlib_call("re", "compile", &[py_str("\\b(?:one|something|a thing|the kind of thing)\\s+(?:worth|to (?:have|get|see|go|ring|call|book|check|ask|look|speak|talk|keep|take|make|do|be|try|use|buy|find|stop|fix|start|wait|leave|stay|read|watch|wash|clean|test|measure|move|change|put|give|send|pay|see))\\b|\\b(?:book|ring|call|see|visit)\\s+(?:a|the|your)\\s+(?:gp|doctor|vet|dentist|nurse)\\b|\\bappointment\\b|\\blooked at\\b|\\bchecked out\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PREFERS_STATED(stdlib_call("re", "compile", &[mcall(&gattr(&g_PREFERS(), "pattern"), "replace", &[py_str("|rather|"), py_str("|"), Py::Int(1)]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_HEAD(stdlib_call("re", "compile", &[py_str("^[£$\\u20ac]?\\s?\\d|^[\\u00bd\\u00bc\\u00be]|^(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|a hundred|half a)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CLEAR(py_tuple(vec![py_tuple(vec![g_TIME(), g__a_clear_time_at_the_head()]), py_tuple(vec![g_AMOUNT(), g__an_amount_at_the_head()]), py_tuple(vec![g_STEPS(), g__steps()]), py_tuple(vec![g_PLACE(), py_func("lambda", move |args: &[Py]| {  let opening: Py = __kwp(args, 0, "opening").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g_PLACE_AS_EVIDENCE(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) })]), py_tuple(vec![g_PERSON(), py_func("lambda", move |args: &[Py]| {  let opening: Py = __kwp(args, 0, "opening").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g_PERSON_WORD(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) })]), py_tuple(vec![g_MEANING(), py_func("lambda", move |args: &[Py]| {  let opening: Py = __kwp(args, 0, "opening").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g_MEANS(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) })]), py_tuple(vec![g_YESNO(), py_func("lambda", move |args: &[Py]| {  let opening: Py = __kwp(args, 0, "opening").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g_VERDICT_OPEN(), "match", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) })]), py_tuple(vec![g_PICK(), py_func("lambda", move |args: &[Py]| {  let opening: Py = __kwp(args, 0, "opening").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g__PREFERS_STATED(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) })]), py_tuple(vec![g_REASON(), py_func("lambda", move |args: &[Py]| {  let opening: Py = __kwp(args, 0, "opening").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g_REASON_OPENS(), "match", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) })])]));
    s__FIGURES(call_builtin("frozenset", &[py_tuple(vec![g_TIME(), g_AMOUNT()])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_TO_GO_ON(stdlib_call("re", "compile", &[py_str("^(?:nothing (?:from me|on my side|more from me|in hand|to (?:add|say|cut|shorten|go on))\\b|there (?:is|'?s|isn'?t|'?s not) nothing (?:from me|on my side|more|in hand|to (?:add|say|cut|shorten))\\b|there'?s nothing (?:from me|on my side|more|in hand|to (?:add|say|cut|shorten))\\b|'?(?:that|it)'? could be (?:a lot|anything|many|loads|one of|several)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_NOTHING_ABOUT_IT(stdlib_call("re", "compile", &[py_str("^(?:(?:honestly,? |well,? )?(?:i|we) (?:have|'?ve) no idea\\b|(?:honestly,? |well,? )?(?:i|we) (?:do not|don'?t|dont|could not|couldn'?t|can'?t|cannot) (?:know|say|tell you|help with (?:that|this|it))\\b|(?:nobody|no one|no-one|not many people) (?:really )?(?:knows|could say)\\b|(?:it|that|this) (?:depends|all depends) (?:on (?:the person|the individual|who you ask|each person|the person concerned)|, honestly)\\b|(?:it|that|this) (?:varies|changes|differs) (?:a lot |a great deal |a bit )?(?:from person to person|from one person to another|between people|by person)\\b|(?:it|that|this) (?:is|'?s) (?:different|not the same|much the same) (?:for everyone|for everybody|for each|for most people)\\b|(?:it|that|this) (?:is|'?s) (?:a|such a) big question\\b|(?:it|that|this) (?:is|'?s) not something (?:i|we) can help\\b|(?:it|that|this) (?:is|'?s|was) beyond (?:me|what i know|my knowledge)\\b|(?:(?:you|we) (?:should|could|might) |(?:i would |i'?d )?)?(?:ask|speak to|talk to|see|check with) (?:an?|your|the) \\w+(?: \\w+)? about (?:that|this|it)\\b|(?:it|that) (?:was|has been|is) a (?:long|busy|hard|tough|rough|hectic) (?:day|week|morning|night)\\b|there (?:is|'?s) a lot (?:to say|said)\\b|(?:it|that) (?:is|'?s) hard to say\\b|(?:this|that|it) (?:is|'?s) (?:one of those|the kind of) (?:questions|things)\\b|(?:a|the) (?:quick |short |little )?(?:search|google|look it up)\\b[^.]{0,30}?\\b(?:should|will|would|can)\\b|(?:no|not a|there'?s no) (?:straight|simple|clear|easy) answer\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COPULA(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|means|equals|measures|amounts to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_JOB_LANDS(stdlib_call("re", "compile", &[py_str("^([A-Za-z']+)\\s+(?:with|by|at|for|from|into|to|about|through|round|around|over|under|back|\\d+|three|four|five|six|seven|eight|nine|ten|a few|several)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ORDINAL_OPENING(stdlib_call("re", "compile", &[py_str("^(?:(?:first|firstly|then|next|after that|finally|lastly|to start|to begin|start by|begin by|step one|step 1)\\b[,:]?\\s*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CHECKS(py_dict(vec![(g_TIME(), g__a_time()), (g_COMPARE(), g__a_comparison()), (g_AMOUNT(), g__an_amount()), (g_PLACE(), g__a_place()), (g_PERSON(), g__a_person()), (g_MEANING(), g__a_meaning_or_a_statement()), (g_YESNO(), g__a_yesno()), (g_PICK(), g__a_pick()), (g_STEPS(), g__steps()), (g_OPTIONS(), g__options()), (g_REASON(), g__a_reason())]));
    s__EMPTY(call_builtin("frozenset", &[py_tuple(vec![py_str("about"), py_str("after"), py_str("again"), py_str("before"), py_str("being"), py_str("could"), py_str("doing"), py_str("first"), py_str("going"), py_str("have"), py_str("just"), py_str("like"), py_str("make"), py_str("much"), py_str("need"), py_str("only"), py_str("really"), py_str("should"), py_str("still"), py_str("than"), py_str("that"), py_str("their"), py_str("them"), py_str("then"), py_str("there"), py_str("these"), py_str("they"), py_str("thing"), py_str("this"), py_str("those"), py_str("want"), py_str("what"), py_str("when"), py_str("where"), py_str("which"), py_str("while"), py_str("will"), py_str("with"), py_str("would"), py_str("your"), py_str("yours"), py_str("from"), py_str("into")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PERMISSION(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|should|shall|may|do|does|did|is it|are they|am i) (?:i|we|you|it|they|my|i have to|i need to|we need to)\\b|\\bdo i need\\b|\\bis it ok\\b|\\bis it safe\\b|\\bam i allowed\\b|\\bi (?:should|must|need to|have to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOB_ANY(stdlib_call("re", "compile", &[mcall(&mcall(&gattr(&g_INSTRUCTS(), "pattern"), "replace", &[py_str("^(?:"), py_str("(?:"), Py::Int(1)]), "replace", &[py_str(")\\b"), py_str("|double|halve|split|scale|reheat|refreeze)\\b")]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIDES(stdlib_call("re", "compile", &[py_str("([a-z0-9']+(?: [a-z0-9']+)?) or ([a-z0-9']+(?: [a-z0-9']+)?)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARATIVE_THAN(stdlib_call("re", "compile", &[py_str("([^?.!,;:]{1,40}?)\\s+(?:much |a bit |a lot |far |way |any |still )?(?:\\b(?!rather\\b|other\\b|fewer\\b)\\w+er\\b|more \\w+|less \\w+|worse)((?:\\s+(?!than\\b)(?!(?:\\w+er|more|less|worse)\\b)\\w+){0,2})\\s+than\\b\\s*([^?.!,;:]{1,40})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIDES_VS(stdlib_call("re", "compile", &[py_str("([a-z']+(?: [a-z']+)?)\\s+vs\\.?\\s+([a-z']+(?: [a-z']+)?)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIDE_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("would"), py_str("will"), py_str("do"), py_str("does"), py_str("did"), py_str("which"), py_str("what"), py_str("and"), py_str("or"), py_str("but"), py_str("so"), py_str("then"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("it"), py_str("its"), py_str("your"), py_str("my"), py_str("our"), py_str("their"), py_str("his"), py_str("her"), py_str("me"), py_str("us"), py_str("them"), py_str("be"), py_str("being"), py_str("been"), py_str("get"), py_str("gets"), py_str("got"), py_str("buy"), py_str("buying"), py_str("have"), py_str("has"), py_str("had"), py_str("should"), py_str("shall"), py_str("could"), py_str("can")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SIDE_WINS(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were) (?:still |usually |often |much |far |a bit )?(?:the )?(?:better|best|kinder|healthier|cheaper|safer|stronger|quicker|smarter|more useful)\\b|\\bthe (?:better|best|kinder|healthier|cheaper|safer|stronger|quicker|smarter|easier) (?:choice|buy|option|bet|value|start|one|bowl|loaf|plan|pick|of the two)\\b|\\b(?:wins?|won)\\b|\\b(?:take|get|go for|pick|choose|buy|stick with) (?:the|a|an) \\w+|\\bnothing (?:[a-z]+ ){0,4}comes close\\b|\\bcomes? close\\b|\\bif you (?:want|need|have|are)\\b|\\b(?:much|far|by far|a lot|still|easily) the (?:better|best|cheapest|healthiest|easiest)\\b|\\bon (?:that|this) one\\b|\\btwice (?:as|the)\\b|\\bhalf the (?:price|cost|time|weight|size)\\b|\\b(?:is|are|was|were) (?:much |far )?(?:further|longer|bigger|heavier|lighter|warmer|colder|stronger|quicker|faster|cheaper|worse|better) than\\b|\\bthe one (?:to|that|for|I would|id)\\b|^both\\b|\\bboth, and\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LINE_TEXT_FIELDS(py_tuple(vec![py_str("plain"), py_str("clean"), py_str("text"), py_str("norm_text"), py_str("norm")]));
    s_THE_THING_ASKED(stdlib_call("re", "compile", &[py_str("^(?:what'?s|what is|what are|what was|what were|what does|what do|what did|how does|how do|tell me about|what about)\\s+(?:(?:an|a|the)\\b\\s*)?(.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ENDS_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\n    and or but for in on at to with of from about when while if that which who whom whose how\n    why where do does did is are was were be been being mean means meant cost costs worth\n    make makes made need needs want wants go goes get gets come comes work works help helps\n    take takes give gives happen happens feel feels look looks sound sounds include includes\n    cover covers contain contains show shows sit sits stand stands say says called call\n    used using made based known named supposed said done grown kept sold eaten worn driven written\n    built filled meant\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__ARTICLE(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|your|my|his|her|their|our|its|this|that|these|those|any|some)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\n    i you we they it that this there one someone anyone everyone nobody everything anything\n    something nothing people\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__JUDGES_THE_THING(stdlib_call("re", "compile", &[py_str("^(?:is|are|was|were|should|must|ought|needs?)?\\s*(?:be\\s+)?(?:not\\s+)?(?:good|bad|great|fine|okay|ok|safe|worth|better|worse|best|worst|useless|rubbish|lovely|nice|amazing|brilliant|terrible|awful|really|usually|often|always|never|definitely|probably|a (?:good|bad|great|waste|rip|must)|the (?:best|worst|only)|your|a waste|the wrong)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ADVISES_THE_THING(stdlib_call("re", "compile", &[py_str("^(?:should|must|ought|needs? to|need to|has to|have to|had to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SAYS_WHAT_IT_IS(stdlib_call("re", "compile", &[py_str("^(?:is|are|was|were|has|have|had|can|cannot|do|does|did|give|gives|provide|provides|let|lets|allow|allows|help|helps|mean|means|refers|comes|runs|works|starts|ends|takes|costs|keeps|contains|restarts|pays|holds|covers|makes|sits|stands|forms|happens|moves|lubricates|carries|spreads|turns|shows|includes|protects|needs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DOING(stdlib_call("re", "compile", &[py_str("^\\w+s\\s+(?:how|what|which|when|where|whether|who|you|it|them|him|her|us|me|the|a|an|your|their|its|some|any|each|every|all|only|just)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PUTS_THE_THING_TO_USE(stdlib_call("re", "compile", &[py_str("^(?:use|put|add|place|stick|tie|clip|screw|wear|keep|hold|ride|apply|spread|slide|hang)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SIDE_EMPTY(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("and"), py_str("but"), py_str("not"), py_str("for"), py_str("with"), py_str("your"), py_str("you"), py_str("our"), py_str("its"), py_str("it's"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("what"), py_str("which"), py_str("from"), py_str("into"), py_str("than"), py_str("then"), py_str("are"), py_str("was"), py_str("were"), py_str("has"), py_str("have"), py_str("had"), py_str("one"), py_str("two"), py_str("any"), py_str("all"), py_str("much"), py_str("more"), py_str("less"), py_str("same")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PATH_THEN_VERB(stdlib_call("re", "compile", &[py_str("(?:>|\\u2192|->)\\s*\\S.{0,120}\\b(?:find|choose|select|click|tap|press|open|set|switch|turn|tick|untick|check|uncheck|change|remove|delete|add|enable|disable|toggle)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GARDEN(stdlib_call("re", "compile", &[py_str("(?:sow|plant|prune|dig|mow|scarify|weed|mulch|repot|stake|harvest|deadhead|sprinkle|hammer|grout|caulk|unblock|unplug)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DOES_NOT_ANYWHERE(stdlib_call("re", "compile", &[py_str("(?:^|[.:;!?]\\s+|,\\s+(?:then|so|and|but|just|or)\\s+|:\\s+)(?:don'?t|do not|never|avoid|skip|no need to)\\s+(?P<after>[^.;:!?]{0,40})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_STEP_BUT_REASSURANCE(stdlib_call("re", "compile", &[py_str("^(?:worry|panic|fret|stress|beat yourself|mind|bother|overthink|be (?:too )?(?:hard|afraid|scared|embarrassed|guilty|silly|worried|shy)|feel (?:bad|guilty|silly|stupid)|feel like|let (?:it|that|them)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__STEPS_IN(g__steps_in());
    s_SAID_WORDS(stdlib_call("re", "compile", &[py_str("^(?:just |only |maybe |perhaps |something like:? |try:? |say:? |start with:? |open with:? |how about:? )?\\s*[\\\"'\\u201c\\u2018\\u00ab][^\\\"'\\u201d\\u2019\\u00bb]{8,}[.!?,]?[\\\"'\\u201d\\u2019\\u00bb]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY(py_str("(?:[£$€]\\s*\\d[\\d,.]*|\\b\\d[\\d,.]*\\s*(?:p|pence|pennies|quid|pounds?|bucks?|dollars?|euros?|cents?|gbp)\\b)"));
    s_MONEY_RATE(stdlib_call("re", "compile", &[add(&g__MONEY(), &py_str("(?:\\s*(?:to|-)\\s*(?:[£$€]\\s*)?\\d[\\d,.]*\\s*(?:p|pence)?)?\\s*(?:a|an|per|each)\\s+(?:sec\\w*|min\\w*|hour|hr|hrs|day|week|month|year|night|visit|call|session|unit)s?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUANTITY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:enough|too (?:much|many|far|often|long|hot|cold|big|small|old|young|late|soon|heavy|short|thick|thin|deep|tall|high|low|expensive|dear))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_QUANTITY_NUMBER(py_str("(?:\\d[\\d.,]*|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|ninety|hundred|half|a couple|a few|an|a)"));
    s__A_QUANTITY_UNIT(py_str("(?:sec|second|minute|min|hour|hr|day|week|month|year|night|kg|kilo|pound|mile|km|litre|pint|cup|gram|ounce|stone|cm|inch|foot|feet|metre|session|time|portion|serving|slice|step|dose|tablet|degree|celsius|unit|item|episode|page|p)s?\\b"));
    s__A_QUANTITY_RELATION(py_str("(?:under|over|above|below|inside|within|short of|past|at|less than|more than|on the line|clear of|off the|away from)"));
    s__A_FIGURE_AGAINST_THE_STANDARD(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\A\\s*(?:[^:]{1,45}:\\s*)?(?:about|around|just|only|nearly|almost|well|roughly)?\\s*(?:£\\s*\\d[\\d.,]*|\\$\\s*\\d[\\d.,]*|€\\s*\\d[\\d.,]*|"), &g__A_QUANTITY_NUMBER()), &py_str("[\\s-]*(?:")), &g__A_QUANTITY_NUMBER()), &py_str("[\\s-]*)?-?\\s*")), &g__A_QUANTITY_UNIT()), &py_str(")(?:\\s+(?:a|an|per|each)\\s+(?:week|day|month|year|night|hour|session))?\\s+(?:is|are|comes|sits|lands)\\s+(?:the\\s+|well\\s+|just\\s+|a bit\\s+|a little\\s+)?")), &g__A_QUANTITY_RELATION()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_AFTER_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\A(?:that|it|this) (?:is|'s|would be|should be) (?:a |an |quite a |quite |a bit of a )?(?:fair|fine|easy|doable|manageable|sensible|reasonable|safe|risky|all right|alright|okay|ok|worthwhile|worth it|no bother)\\b|\\A(?:doable|fine|fair|easy|sensible|reasonable|manageable|worth it|worth doing|workable|sound|just the job|no bother|all right|alright)\\b|\\b(?:doable|easy to|easy|manageable|sensible|reasonable|no bother|just the job|all right|alright|worth it)\\b|\\bfine (?:to|for|in|if|on|as)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_INSTRUCTION(stdlib_call("re", "compile", &[py_str("^[A-Za-z']+\\s+(?:a|an|the|your|their|its|some|one|two|each|every|both|all|my|these|those)(?=\\s+[A-Za-z0-9'\\u2019\\\"\\u00a3$])|^[A-Za-z']+\\s+(?:it|them|him|her|me|us)\\s+(?:off|on|out|in|up|down|away|over|back|through|aside|together|to|into|onto|under|with|for|at|inside|across|around|along|near|behind|beside|against)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRACTION_OPENER(stdlib_call("re", "compile", &[py_str("^[A-Za-z]+['\\u2019](?:s|re|ll|ve|d|m)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WELL_WISHING(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|so\\s+)?(?:enjoy|hope (?:you|it|that)|all the best|take care|good luck|sleep well|get well|have (?:a|an) (?:nice|good|great|lovely|brilliant|fab|wonderful|happy|safe|quiet|lazy|restful|easy|better|fun|relaxing|peaceful))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUBORDINATE_FRAGMENT(stdlib_call("re", "compile", &[py_str("^(?:because|since|although|though|unless|whereas|while|if|when|after|before|as|where)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_OPENER(call_builtin("frozenset", &[py_tuple(vec![py_str("still"), py_str("sorry"), py_str("even"), py_str("maybe"), py_str("perhaps"), py_str("honestly"), py_str("actually"), py_str("obviously"), py_str("apparently"), py_str("probably"), py_str("really"), py_str("quite"), py_str("please"), py_str("thanks"), py_str("ok"), py_str("okay"), py_str("well"), py_str("oh"), py_str("ah"), py_str("yes"), py_str("yeah"), py_str("no"), py_str("nope"), py_str("sure"), py_str("fine"), py_str("anyway"), py_str("love"), py_str("nice"), py_str("good"), py_str("great"), py_str("lovely"), py_str("brilliant"), py_str("perfect"), py_str("glad"), py_str("congratulations"), py_str("congrats"), py_str("cheers"), py_str("smashing"), py_str("fab"), py_str("wonderful"), py_str("grand"), py_str("ace"), py_str("decent"), py_str("super"), py_str("agreed"), py_str("yay"), py_str("hurrah"), py_str("hooray"), py_str("about"), py_str("as"), py_str("only"), py_str("around"), py_str("over"), py_str("under"), py_str("after"), py_str("before"), py_str("with"), py_str("from"), py_str("than"), py_str("like"), py_str("at"), py_str("in"), py_str("on"), py_str("to"), py_str("of"), py_str("for"), py_str("by"), py_str("up"), py_str("down"), py_str("out")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HALF_A_UNIT(stdlib_call("re", "compile", &[py_str("^half\\s+(?:a|an|the)?\\s*(?:hour|hours|minute|minutes|second|seconds|day|days|week|weeks|month|months|year|years|cup|cups|glass|glasses|pint|pints|litre|litres|liter|liters|kilo|kilos|kilogram|kilograms|pound|pounds|mile|miles|metre|metres|meter|meters|inch|inches|teaspoon|teaspoons|tablespoon|tablespoons|tsp|tbsp|degree|degrees)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__steps_by_the_verb_list(g__steps());
    let _ = py_setitem(&g__CHECKS(), &g_STEPS(), g__steps());
    s__CLEAR(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__CLEAR()) { let __unpk2723 = py_unpack(&__item.clone(), 2, None); let mut want: Py = __unpk2723[0].clone(); let mut check: Py = __unpk2723[1].clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&want.clone(), &g_STEPS()))) { py_tuple(vec![want.clone(), py_func("_steps", |args: &[Py]| _steps(args))]) } else { py_tuple(vec![want.clone(), check.clone()]) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_rule_settles_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the reply gives the test the person's case turns on, and asks them nothing.\n\n    The condition has to be in the opening, where a verdict lives. Read over the whole reply it\n    credited advice that answers another question: \"my mum might come too, is that too much food\"\n    was told how to set a boundary with a parent, and the `if` in the fifth line of that made it\n    an answer to a question about catering.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { opening.clone() } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("?")))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_AN_OFFER_NOT_A_TEST(), "search", &[opening.clone()])) {
    return Py::Bool(false);
    }
    found = mcall(&g_A_RULE_THAT_SETTLES_IT(), "search", &[opening.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("once"), py_str("until"), py_str("unless")]), &mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("[:;]"), slice_or_raise(&opening.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&slice_or_raise(&opening.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("if"), py_str("when"), py_str("whenever"), py_str("whether")]), &mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&mcall(&g_A_STANDING_AFTER_THE_TEST(), "search", &[slice_or_raise(&opening.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]), &Py::None));
    Py::None
}
pub fn _said(args: &[Py]) -> Py {
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
    let mut parts: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("norm"), py_str("norm_text"), py_str("clean"), py_str("text")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&understood.clone(), "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    value = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { value.clone() } }) {
    let _ = mcall(&parts.clone(), "append", &[value.clone()]);
    }
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&py_str(" "), "join", &[parts.clone()]), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn _a_who_a_person_owns(args: &[Py]) -> Py {
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
    let mut entities: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the who in the line belongs to the person the line is already about.\n\n    \"something for my sister who just had a baby\", \"how do i deal with a boss who shouts\":\n    understanding has read the sister and the boss as a person of the line, and the who standing\n    right after them is the relative pronoun - the ask is a present or for advice, not a name.\n    Read as a who-question, 33 of the deck's 349 person wants owed a name no gift or advice reply\n    could give, and each of those replies was counted a miss. A who the line opens on, a who that\n    asks after a new person (\"my daughter wants to know who mozart was\") and a who with no\n    person before it (\"i got whiplash in a crash who pays\") still owe the name.\n    ");
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    entities = mcall(&understood.clone(), "get", &[py_str("entities")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[entities.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&entities.clone());
    for __item in __iter1 {
    entity = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entity.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&entity.clone(), "get", &[py_str("type")]), &py_str("person"))) } }) {
    continue;
    }
    at = mcall(&entity.clone(), "get", &[py_str("at")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[at.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&at.clone(), &Py::Int(0)) } }) {
    continue;
    }
    after = add(&at.clone(), &call_builtin("len", &[mcall(&call_builtin("str", &[{ let __b = mcall(&entity.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    word = if py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp20 = after.clone(); __cmp20.clone() })) { lt(&__cmp20.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) }) { mcall(&mcall(&call_builtin("str", &[geti(&words.clone(), &after.clone())]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str("'’")]) } else { py_str("") };
    if py_truthy(&{ let __b = mcall(&word.clone(), "startswith", &[py_str("who")]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("that"))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_a_pair(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("True when the line itself names the two things it is about, as a comparison has to.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SIDE_PAIRS()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_about_them(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let _ = py_str("True when the line is asking about the pair rather than telling the bot something about it.\n\n    \"Hows that different\" carries the words of comparing but no pair, and it is a question about\n    the bot's own last answer, so it falls through to the follow-up rule and keeps the shape it\n    followed. \"I think im being underpaid compared to the new guy\" names a pair and is still news.\n    ");
    if py_truthy(&Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("intent")]), &py_str("news")))) {
    return Py::Bool(false);
    }
    return { let __b = mcall(&g_EXPLICIT_COMPARE(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))) } };
    Py::None
}
pub fn _a_help_ask_that_names_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks for help and names no task to be helped with.\n\n    \"i need help\", \"i need help with something\", \"i need help but i dont know what with\" ask for\n    help and say nothing about what with, so the reply that asks what is going on is the right one\n    and there is no method to give. \"anything i can do about it\" carries no help word at all and\n    keeps its steps.\n    ");
    match_ = { let __b = mcall(&g_HELP_ASK_ANY(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { mcall(&g_HELP_WORD(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } };
    rest = if py_truthy(&match_.clone()) { slice_or_raise(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None) } else { { let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } } };
    return call_builtin("bool", &[mcall(&g_HELP_ASK_NO_TASK(), "search", &[rest.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _whole(args: &[Py]) -> Py {
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
    let mut value: Py = Py::None;
    let _ = py_str("The line as the person typed it, for the few readings that need the whole of its shape.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plain"), py_str("text")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&understood.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&value.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_give_is_to_the_bot(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let _ = py_str("Whether the give at `at` is the person asking, and not somebody giving something to them.");
    before = mcall(&slice_or_raise(&said.clone(), &Py::None, &at.clone(), &Py::None), "split", &[]);
    return { let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GIVE_ME_LEAD_IN(), &geti(&before.clone(), &neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _recall_wants(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut after: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The kind of thing a line asks back for (\"what did i say my budget was\" is an amount), or None.\n\n    The shape is read from the words on either side of the recall verb: \"how long did i say i had\"\n    puts it in front, \"did i say when we were going\" behind, and \"what did i say my budget was\"\n    names it as the noun of the thing recalled. A shape word further back is not about the recall:\n    \"what did i say she does when she revises\" asks what she does, so only the words right behind\n    the verb are read.\n    ");
    whole = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { said.clone() } }, "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&whole.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_RECALL_SAID(), "search", &[whole.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_RECALL_NOT(), "search", &[whole.clone()]) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_RECALL_SAID(), "finditer", &[whole.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    after = mcall(&g_RECALL_PRONOUN(), "sub", &[py_str(""), mcall(&slice_or_raise(&whole.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[])]);
    kind = { let __b = _recall_kind_in_front(&[after.clone()]); if py_truthy(&__b) { __b } else { { let __b = _recall_kind_named(&[after.clone()]); if py_truthy(&__b) { __b } else { _recall_kind_behind(&[slice_or_raise(&whole.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]) } } } };
    if py_truthy(&kind.clone()) {
    return kind.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _recall_kind_in_front(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The kind of thing asked back for, named at the front of the words behind the verb.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_RECALL_TIME(), g_TIME()]), py_tuple(vec![g_RECALL_AMOUNT(), g_AMOUNT()]), py_tuple(vec![g_RECALL_PLACE(), g_PLACE()]), py_tuple(vec![g_RECALL_PERSON(), g_PERSON()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "match", &[after.clone()])) {
    return kind.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _recall_kind_named(args: &[Py]) -> Py {
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
    let mut noun: Py = Py::None;
    let _ = py_str("The kind named as the thing recalled: \"my budget\", \"her name\".");
    noun = mcall(&g_RECALL_KIND_NOUN_ASK(), "match", &[after.clone()]);
    return if py_truthy(&noun.clone()) { geti(&g_RECALL_KIND_NOUN(), &mcall(&mcall(&noun.clone(), "group", &[Py::Int(1)]), "lower", &[])) } else { Py::None };
    Py::None
}
pub fn _recall_kind_behind(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The kind asked in front of the verb: \"how long did i say\", \"where did i say\".");
    match_ = mcall(&g_RECALL_SHAPE_BEFORE(), "search", &[before.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    named = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&g_RECALL_SHAPE_BEFORE_KIND());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&named.clone(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&named.clone(), "startswith", &[add(&word.clone(), &py_str(" "))]) } }) {
    return kind.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn asks_a_follow_up(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line is a bare follow-up: it opens on \"and\"/\"what about\" and asks nothing new.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("asks_a_verdict")]) } }) {
    return Py::Bool(false);
    }
    said = _said(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_FOLLOWUP(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&sub(&call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("what"), py_str("yesno")]))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NEW_ASK(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    kind = mcall(&understood.clone(), "get", &[py_str("kind")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("question"))); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    return Py::Bool(true);
    }
    line = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("statement"))); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&understood.clone(), "get", &[py_str("followup")])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } } } };
    Py::None
}
pub fn _a_fragment_asks_for_a_time(args: &[Py]) -> Py {
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
    let _ = py_str("True when a measure word in the line is the ask, even with no question grammar.");
    if py_truthy(&mcall(&g__TALKING_ABOUT_A_LENGTH(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__TRAILING_MEASURE(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { mcall(&g__MEASURE_THEN_VERB(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ask_clause(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The last clause of a line that opens on a question word, or \"\" when none does.\n\n    A two-clause line - the scene, then the question - is answered from the question, so the words\n    that say what it asks are read there rather than at the opening of the setting.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__ASK_CLAUSE_SPLIT(), "split", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    last = geti(&parts.clone(), &neg(&Py::Int(1)));
    return if py_truthy(&mcall(&g__ASK_CLAUSE_OPENS(), "match", &[last.clone()])) { last.clone() } else { py_str("") };
    Py::None
}
pub fn wants(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::None };
    let mut own: Py = Py::None;
    let _ = py_str("The shape of answer the line asks for, or None when the line does not ask for one.\n\n    Read from the whole line's reading, not one keyword. `asks_a_verdict` (\"is that bad\", \"should\n    i worry\") is a yes/no about the bot's own last answer and comes before every `asks` label;\n    `help_ask` (\"anything i can do about it\") is a bare ask for steps. A question about the bot\n    itself (\"who are you\") wants the bot's own answer and no shape from this list, so it is left\n    alone rather than read as a who-question that owes a person's name.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_RECALL_ASK(), "search", &[{ let __b = _said(&[understood.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::None;
    }
    own = _wants_alone(&[understood.clone()]);
    own = _the_shape_named_outside_the_reading(&[understood.clone(), own.clone()]);
    if py_truthy(&own.clone()) {
    return own.clone();
    }
    if py_truthy(&_a_bare_definition(&[understood.clone()])) {
    return g_MEANING();
    }
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { asks_a_follow_up(&[understood.clone()]) } }) {
    return before.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_shape_named_outside_the_reading(args: &[Py]) -> Py {
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
    let mut own: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The shape a line names outright, when the reading found none or found a cause instead.\n\n    A reading of the question words is what most lines need. These two are the words that name the\n    answer themselves and that no cue list carries: a cause the person has just been given, and a\n    figure whose unit is money or a count. Neither is a keyword family - both are read off the\n    whole line - and both are kept out of `_wants_alone` because they correct its answer rather\n    than add to it, so they run on the shape it returns.\n    ");
    said = { let __b = _said(&[understood.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return own.clone();
    }
    if py_truthy(&mcall(&g_IF_IT_HAPPENS(), "match", &[said.clone()])) {
    return own.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&own.clone(), &g_REASON())); if !py_truthy(&__b) { __b } else { mcall(&g_CONFIRM_CAUSE(), "search", &[said.clone()]) } }) {
    return g_YESNO();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![Py::None, g_REASON(), g_MEANING()]), &own.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_ASK_TO_ASK(), "search", &[said.clone()]) } }) {
    return g_YESNO();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&own.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g_A_FIGURE_NAMED(), "search", &[said.clone()]) } }) {
    return if py_truthy(&mcall(&g_WHAT_TIME_NAMED(), "search", &[said.clone()])) { g_TIME() } else { g_AMOUNT() };
    }
    return own.clone();
    Py::None
}
pub fn wanted(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("The shape of answer this turn asked for, read from the store rather than from a reading.\n\n    `wants` above is the finder: it reads one reading of the line. A module that holds a reading\n    of its own is not the turn, though - `answers._ask_a_rewritten_line` swaps `understood` for a\n    reading of words the person never typed, and the module underneath then looked at a line that\n    asked for nothing. The shape is a fact about the turn, so a module that wants to know which\n    answer it owes asks the store: `bot/modules/intent.py` keeps it under `understood[\"wants\"]`,\n    and `intent_wants` holds the bare shape for the same turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    want = mcall(&read.clone(), "get", &[py_str("wants")]);
    if py_truthy(&Py::Bool(py_contains(&g_WANTS(), &want.clone()))) {
    return want.clone();
    }
    }
    want = mcall(&memory.clone(), "get", &[py_str("intent_wants")]);
    return if py_truthy(&Py::Bool(py_contains(&g_WANTS(), &want.clone()))) { want.clone() } else { Py::None };
    Py::None
}
pub fn _wants_alone(args: &[Py]) -> Py {
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
    let mut ask_clause: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asked_back: Py = Py::None;
    let mut asking: Py = Py::None;
    let mut asks: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut focus: Py = Py::None;
    let mut focus_said: Py = Py::None;
    let mut found: Py = Py::None;
    let mut labels: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The shape this line asks for on its own words, ignoring the line before it.");
    asked_back = _recall_wants(&[_said(&[understood.clone()]), _whole(&[understood.clone()])]);
    if py_truthy(&asked_back.clone()) {
    return asked_back.clone();
    }
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("about_bot")])) {
    return Py::None;
    }
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("asks_a_verdict")])) {
    return g_YESNO();
    }
    if py_truthy(&mcall(&g_OPEN_WHO_HELP(), "search", &[_said(&[understood.clone()])])) {
    return g_PERSON();
    }
    said = _said(&[understood.clone()]);
    typed = { let __b = _whole(&[understood.clone()]); if py_truthy(&__b) { __b } else { said.clone() } };
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("help_ask")])) {
    return if py_truthy(&_a_help_ask_that_names_nothing(&[typed.clone()])) { Py::None } else { g_STEPS() };
    }
    asks = mcall(&understood.clone(), "get", &[py_str("asks")]);
    asks = if py_truthy(&call_builtin("isinstance", &[asks.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("set", &[asks.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("intent")]), &py_str("news")))) {
    dropped = py_set(vec![py_str("choice")]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SUM_OUT_LOUD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SUM_ASKS(), "search", &[said.clone()]) } }))) {
    let _ = mcall(&dropped.clone(), "add", &[py_str("maths")]);
    }
    let __aug1 = sub(&asks.clone(), &call_builtin("frozenset", &[dropped.clone()]).unwrap_or_else(|e| exc::raise(e)));
    asks = __aug1;
    }
    asking = { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("question"), py_str("request")]), &mcall(&understood.clone(), "get", &[py_str("intent")]))); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_HOW_ASK_FORM(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_QUESTION_IN_THE_LINE(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))) } } } } } } } };
    if py_truthy(&mcall(&g_HOW_ARE_YOU(), "match", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HOW_IT_WENT(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HOW_IT_HAPPENED(), "search", &[said.clone()])) {
    return g_REASON();
    }
    if py_truthy(&mcall(&g_HELP_ME(), "search", &[said.clone()])) {
    return g_STEPS();
    }
    if py_truthy(&mcall(&g_HELP_ASK_ANY(), "search", &[typed.clone()])) {
    return if py_truthy(&_a_help_ask_that_names_nothing(&[typed.clone()])) { Py::None } else { g_STEPS() };
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_DOING_WRONG(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_WRONG_ASKED_ABOUT(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WRONG_RECALL(), "search", &[said.clone()]))) } } } } } } } }) {
    return g_REASON();
    }
    if py_truthy(&{ let __b = mcall(&g_FIX_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("\\s*(?:is|are|can|could|will|would|do|does)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = mcall(&g_WHERE_START(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BEST_WAY(), "search", &[said.clone()]) } }) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = mcall(&g_NEED_TO(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NEED_TO_ASKED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&sub(&asks.clone(), &py_set(vec![py_str("maths"), py_str("choice"), py_str("write")])))) } } } }) {
    return g_STEPS();
    }
    if py_truthy(&mcall(&g_WHERE_POINT(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_WHERE_RECALL(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHERE_ALONE(), "search", &[_whole(&[understood.clone()])]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HOW_TO_ANSWER(), "search", &[said.clone()])) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = mcall(&g_PRICE_IDIOM(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = asking.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:what|how much)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_HOW_COME(), "search", &[said.clone()])) {
    return g_REASON();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("steps"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_I(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_COMPARE_MYSELF(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&asks.clone(), &py_str("maths"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&asks.clone(), &py_str("how_much"))) } }; if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("[0-9]"), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } })) } } } } } }) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = mcall(&g_STEPS_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_COUNTED_STEPS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow (?:do|can|should) i\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } })) } }) {
    return g_STEPS();
    }
    if py_truthy(&mcall(&g_MEMBERS_ASK(), "search", &[said.clone()])) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = mcall(&g_KINDS_OF(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_KINDS_OF(), "search", &[said.clone()]))) } }) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_COMPARISON(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_COMPARED(), "search", &[said.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_A_COMPARISON(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _names_a_pair(&[said.clone()]); if !py_truthy(&__b) { __b } else { _asks_about_them(&[understood.clone(), said.clone()]) } } } } } }) {
    return g_COMPARE();
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_ORIGIN_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ORIGIN_PERSON(), "search", &[said.clone()]))) } }; if py_truthy(&__b) { __b } else { mcall(&g_MADE_OF(), "search", &[said.clone()]) } }) {
    return g_MEANING();
    }
    if py_truthy(&{ let __b = mcall(&g_BEST_OF_MANY(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_COST_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { le(&asks.clone(), &g_COST_STANDS_IN()) } })) } }) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("definition"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHAT_ARE_MINE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:called|mean|means|meaning|stand for|stands for)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return g_STEPS();
    }
    if py_truthy(&mcall(&g_MY_THING_CALLED(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("definition"))); if !py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "search", &[py_str("\\bwhat does\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_FOR_MEANING(), "search", &[said.clone()]))) } } } }) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("definition")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("definition"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_FOR_MEANING(), "search", &[said.clone()]))) } } } }) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("definition")]);
    }
    if py_truthy(&mcall(&g_GOOD_MEASURE(), "search", &[said.clone()])) {
    return g_AMOUNT();
    }
    if py_truthy(&{ let __b = mcall(&g_GOOD_THING(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&asks.clone(), &py_str("definition"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GOOD_FIGURE_WORD(), "search", &[said.clone()]))) } } } }) {
    return if py_truthy(&mcall(&g_GOOD_WAY(), "search", &[said.clone()])) { g_STEPS() } else { g_OPTIONS() };
    }
    if py_truthy(&{ let __b = mcall(&g_HOW_IT_WORKS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow do i\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return g_MEANING();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("who"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_WHO_OPENS(), "match", &[said.clone()]))); if !py_truthy(&__b) { __b } else { _a_who_a_person_owns(&[understood.clone()]) } } } }) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("who")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("who"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHO_AND_MEASURE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHO_OF_A_PRONOUN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHO_OF_A_NOUN(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WHO_OPENS(), "match", &[said.clone()]))) } } } } } } } }) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("who")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("where"))); if !py_truthy(&__b) { __b } else { mcall(&g_WHERE_MONEY_GOES(), "search", &[said.clone()]) } }) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("where"))); if !py_truthy(&__b) { __b } else { { let __b = bitand(&asks.clone(), &call_builtin("frozenset", &[py_tuple(vec![py_str("yesno"), py_str("recommend")])]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHERE_RELATIVE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_TO_BE_TOLD_WHERE(), "search", &[said.clone()]))) } } } } } }) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("where")]);
    }
    if py_truthy(&mcall(&g_LONG_TIME(), "search", &[said.clone()])) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("how_long")]);
    let _ = mcall(&asks.clone(), "discard", &[py_str("how_often")]);
    }
    if py_truthy(&{ let __b = mcall(&g_CONVERSION(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CONVERTED_FIGURE(), "search", &[said.clone()]) } }) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_NOT_A_FIGURE(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bitand(&asks.clone(), &g_MEASURE_LABELS()); if !py_truthy(&__b) { __b } else { asking.clone() } }) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how_long"))); if !py_truthy(&__b) { __b } else { mcall(&g_WRITTEN_THING(), "search", &[said.clone()]) } }) {
    return g_AMOUNT();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how_long"))); if !py_truthy(&__b) { __b } else { mcall(&g_THING_THAT_EXTENDS(), "search", &[said.clone()]) } }) {
    return g_AMOUNT();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how_long"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&asks.clone(), &py_str("steps"))); if !py_truthy(&__b) { __b } else { mcall(&g_METHOD_OPENS(), "match", &[mcall(&g_LEADING_JOIN(), "sub", &[py_str(""), said.clone()])]) } } } }) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how_long"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_SIZE_LONG(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DURATION_WORD(), "search", &[said.clone()]))) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how_much"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how_many"))); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow (?:much|many)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("how_long")]);
    return g_AMOUNT();
    }
    return Py::None;
    }
    return if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how_long"))); if py_truthy(&__b) { __b } else { mcall(&g_HOW_MANY_TIME(), "search", &[said.clone()]) } }) { g_TIME() } else { g_AMOUNT() };
    }
    if py_truthy(&{ let __b = mcall(&g_WHAT_DOES_MEASURE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_ASKS_FOR_MEANING(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_COSTS_NOTHING(), "search", &[said.clone()]))) } } } }) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_WHAT_IS_PER(), "search", &[said.clone()])) {
    return g_AMOUNT();
    }
    if py_truthy(&Py::Bool(py_contains(&asks.clone(), &py_str("how")))) {
    if py_truthy(&mcall(&g_HOW_SOON(), "search", &[said.clone()])) {
    return g_TIME();
    }
    if py_truthy(&mcall(&g_HOW_MEASURE(), "search", &[said.clone()])) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_HOW_MEASURE_HEAD(), "match", &[mcall(&g_LEADING_JOIN(), "sub", &[py_str(""), said.clone()])])) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_HOW_TO_DO(), "search", &[said.clone()])) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("definition")]);
    }
    }
    if py_truthy(&_a_fragment_asks_for_a_time(&[said.clone()])) {
    return g_TIME();
    }
    asked = mcall(&g_LEADING_JOIN(), "sub", &[py_str(""), said.clone()]);
    focus = { let __b = mcall(&understood.clone(), "get", &[py_str("asking")]); if py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("focus")]) } };
    focus_said = if py_truthy(&call_builtin("isinstance", &[focus.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&focus.clone()) { let mut w: Py = __item.clone(); __out.push(call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]) } else { if py_truthy(&call_builtin("isinstance", &[focus.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { focus.clone() } else { py_str("") } };
    if py_truthy(&{ let __b = { let __b = mcall(&g_WHEN_ASKS(), "match", &[asked.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHEN_ASKS(), "match", &[mcall(&g_LEADING_JOIN(), "sub", &[py_str(""), mcall(&focus_said.clone(), "strip", &[])])]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WHICH_DAY_IS_BEST(), "search", &[said.clone()]))) } }) {
    return g_TIME();
    }
    ask_clause = if py_truthy(&asking.clone()) { _the_ask_clause(&[said.clone()]) } else { py_str("") };
    if py_truthy(&{ let __b = ask_clause.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHEN_ASKS(), "match", &[mcall(&g_LEADING_JOIN(), "sub", &[py_str(""), ask_clause.clone()])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WHICH_DAY_IS_BEST(), "search", &[said.clone()]))) } } } }) {
    return g_TIME();
    }
    if py_truthy(&{ let __b = mcall(&g_WHAT_ARE_TIMES(), "match", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:called|mean|means|meaning|stand for)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return g_TIME();
    }
    if py_truthy(&mcall(&g_A_GOOD_TIME(), "search", &[said.clone()])) {
    return g_YESNO();
    }
    if py_truthy(&{ let __b = mcall(&g_GOOD_FIGURE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_VERDICT_FRAME(), "match", &[said.clone()]))) } }) {
    return if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:time|timeframe)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { g_TIME() } else { g_AMOUNT() };
    }
    if py_truthy(&{ let __b = mcall(&g_JUDGED_MEASURE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_ON_THE_CLOCK(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_YES_OR_NO(), "match", &[mcall(&g_LEADING_JOIN(), "sub", &[py_str(""), said.clone()])]))) } } } } } } } }) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_GOOD_ONE(), "search", &[said.clone()])) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GIVE_ME_NOT_A_THING(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GIVE_ME_MADE_UP(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GIVE_ME_AN_IDIOM(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&asks.clone(), &py_str("list"))) } } } } } } } }) {
    found = mcall(&g_GIVE_ME_A_FEW(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { _the_give_is_to_the_bot(&[said.clone(), mcall(&found.clone(), "start", &[])]) } }) {
    return g_OPTIONS();
    }
    found = mcall(&g_GIVE_ME_ONE(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { _the_give_is_to_the_bot(&[said.clone(), mcall(&found.clone(), "start", &[])]) } }) {
    return g_PICK();
    }
    }
    if py_truthy(&{ let __b = mcall(&g_A_THING_TO_GIVE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_GIVING_FRAME(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&asks.clone(), &py_str("yesno"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_GIVING_THING(), "search", &[said.clone()]))) } } } } } }) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = mcall(&g_WHAT_FIGURE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHAT_AGE(), "search", &[said.clone()]) } }) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_NEED_TO_DO(), "search", &[said.clone()])) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = mcall(&g_WHAT_HELPS(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHAT_YOU_TAKE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHAT_I_NEED(), "search", &[said.clone()]) } } } }) {
    return g_OPTIONS();
    }
    if py_truthy(&mcall(&g_WHAT_YOU_DO(), "search", &[said.clone()])) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_OTHER_QUESTION(), "match", &[said.clone()]))) } }) {
    if py_truthy(&mcall(&g_WHAT_I_WRITE(), "search", &[said.clone()])) {
    return g_STEPS();
    }
    if py_truthy(&mcall(&g_WHAT_I_GROW(), "search", &[said.clone()])) {
    return g_OPTIONS();
    }
    }
    if py_truthy(&mcall(&g_WHAT_TO_BAKE(), "search", &[said.clone()])) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = mcall(&g_WHAT_THING_FRAGMENT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SUBSTITUTE(), "search", &[said.clone()]))) } }) {
    return g_PICK();
    }
    if py_truthy(&mcall(&g_DO_WITH_A_PERSON(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHAT_CAN_I_DO_WITH(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHAT_SHOULD_I_DO_WITH(), "search", &[said.clone()]) } } } }) {
    if py_truthy(&bitand(&asks.clone(), &g_PICK_SIDES())) {
    return g_PICK();
    }
    if py_truthy(&mcall(&g_WHAT_SHOULD_I_DO_WITH(), "search", &[said.clone()])) {
    return g_STEPS();
    }
    return g_OPTIONS();
    }
    if py_truthy(&mcall(&g_WHAT_ELSE_WITH(), "search", &[said.clone()])) {
    return g_OPTIONS();
    }
    if py_truthy(&mcall(&g_WHAT_ELSE_STEPS(), "search", &[said.clone()])) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = mcall(&g_PICK_ONE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PICK_ONE_OF_A_KIND(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHAT_TO_OPENS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_WHAT_TO_PICK(), "search", &[said.clone()]) } } } } } }) {
    return if py_truthy(&mcall(&g_HAVE_RECALL(), "match", &[said.clone()])) { Py::None } else { g_PICK() };
    }
    if py_truthy(&mcall(&g_SUBSTITUTE(), "search", &[said.clone()])) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = mcall(&g_PAIRING_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))) } }) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_WHAT_TO_DO(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHAT_TO_OPENS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_WHAT_TO_STEPS(), "search", &[said.clone()]) } } } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_STEPS(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_OTHER_QUESTION(), "match", &[said.clone()]))) } } } } } }) {
    return g_STEPS();
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_REPORTED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { mcall(&g_REASON_ASK(), "search", &[said.clone()]) } } } }) {
    return g_REASON();
    }
    found = mcall(&g_POINT_ASK(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_ITS_OWN_YES_NO(), "search", &[said.clone()]))) } }) {
    thing = mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "split", &[]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POINT_NOT_A_THING(), &geti(&thing.clone(), &Py::Int(0)))) } }) {
    return g_REASON();
    }
    }
    if py_truthy(&{ let __b = le(&asks.clone(), &bitor(&py_set(vec![py_str("recommend")]), &g_MEASURE_LABELS())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_ASKS_OUT_LOUD(), "match", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TAIL_VERDICT_ASK(), "search", &[_whole(&[understood.clone()])]))) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_DIFFERENCE(), "search", &[said.clone()])) {
    return g_MEANING();
    }
    if py_truthy(&bitand(&asks.clone(), &g_PICK_SIDES())) {
    return g_PICK();
    }
    if py_truthy(&{ let __b = mcall(&g_STEPS_ARE_COUNTED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_STEPS_AS_A_DISTANCE(), "search", &[said.clone()]) } }) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("steps")]);
    if py_truthy(&mcall(&g_STEPS_AS_A_DISTANCE(), "search", &[said.clone()])) {
    return g_AMOUNT();
    }
    }
    if py_truthy(&{ let __b = mcall(&g_PRICE_ADJECTIVE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_PRICE_ADJECTIVE_KEEPS(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_PRICE_ADJECTIVE_THE_ASK_IS_LATER(), "search", &[_line_text(&[mcall(&understood.clone(), "get", &[py_str("text")])])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&asks.clone(), &g_PRICE_ADJECTIVE_WHY()))) } } } } } }) {
    return g_AMOUNT();
    }
    if py_truthy(&{ let __b = mcall(&g_PRICE_ADJECTIVE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { bitand(&asks.clone(), &g_PRICE_ADJECTIVE_WHY()) } }) {
    return g_REASON();
    }
    if py_truthy(&{ let __b = mcall(&g_A_PRICE_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_PRICE_ASK(), "search", &[said.clone()]))) } }) {
    return g_AMOUNT();
    }
    if py_truthy(&mcall(&g_RECALL_OF_MINE(), "search", &[said.clone()])) {
    let _ = mcall(&asks.clone(), "discard", &[py_str("yesno")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("yesno"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&asks.clone(), &g_SPECIFIC_ASKS()))) } }) {
    return g_YESNO();
    }
    if py_truthy(&{ let __b = mcall(&g_COST_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { le(&asks.clone(), &g_COST_STANDS_IN()) } }) {
    return g_AMOUNT();
    }
    if py_truthy(&{ let __b = mcall(&g_A_FIGURE_OF_A_MONEY_VERB(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MONEY_VERB_NOT_A_PRICE(), "search", &[said.clone()]))) } }) {
    return g_AMOUNT();
    }
    if py_truthy(&{ let __b = mcall(&g_WHAT_CAN_I_DO(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&asks.clone(), &g_PICK_SIDES()))) } }) {
    return g_STEPS();
    }
    let __iter2 = py_iter_or_raise(&g_ASKED_FOR());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    shape = __unpk3[0].clone();
    labels = __unpk3[1].clone();
    if py_truthy(&bitand(&sub(&asks.clone(), &if py_truthy(&asking.clone()) { call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)) } else { g_MEASURE_LABELS() }), &labels.clone())) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shape.clone(), &g_PICK())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&labels.clone(), &py_str("recommend"))); if !py_truthy(&__b) { __b } else { mcall(&g_A_FEW_THINGS(), "search", &[said.clone()]) } } } }) {
    return g_OPTIONS();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shape.clone(), &g_AMOUNT())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&asks.clone(), &py_str("maths"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&asks.clone(), &py_str("how_much"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_SUM_OUT_LOUD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SUM_ASKS(), "search", &[said.clone()]) } })) } } } } } } } }) {
    continue;
    }
    return shape.clone();
    }
    }
    if py_truthy(&mcall(&g_TAIL_VERDICT_ASK(), "search", &[_whole(&[understood.clone()])])) {
    return g_YESNO();
    }
    return Py::None;
    Py::None
}
pub fn _an_amount_with_a_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    return { let __b = call_builtin("bool", &[_an_amount(&[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_UNIT(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _about_the_thing_the_line_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut head: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut wide: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply is about something the line named: provenance, not subject.\n\n    A wide reading of a yes or a no - why, what the thing is, the figure the yes rested on, what\n    to do about it - is worth nothing when the reply is about something else, and read without\n    the line it says yes to nearly any fact-shaped reply: in a paired test \"Two to three hours.\"\n    was credited as the verdict for every yes/no line there is, and \"Sort by colour, not by kind\n    of garment\" answered \"should i run a hot wash with vinegar\".\n\n    The thing has to be named where the reply states its subject - the first five words - unless\n    the reply points back with a pronoun (\"It varies with the drink, ... 60 to 80mg\") or tells the\n    person what to do, where the object comes later (\"Slice it first, then bag it: bread keeps\n    three months\" answers \"can i freeze bread\"). The figure the line asked about counts as well:\n    \"is 60mg enough\" / \"60 to 80mg is a mug of instant coffee\".\n    ");
    words = _words(&[opening.clone()]);
    wide = { let __b = call_builtin("bool", &[mcall(&g_POINTS_BACK(), "match", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { _an_instruction(&[opening.clone()]) } };
    head = if py_truthy(&wide.clone()) { call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("set", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(5), &Py::None)]).unwrap_or_else(|e| exc::raise(e)) };
    let __iter1 = py_iter_or_raise(&call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__EMPTY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&head.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]));
    for __item in __iter2 {
    stem = __item.clone();
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__EMPTY(), &stem.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &stem.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&head.clone(), &stem.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return call_builtin("bool", &[bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("\\d+"), opening.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("\\d+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_instruction(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut second: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply tells the person to do something, which is not a verdict on its own.");
    if py_truthy(&{ let __b = mcall(&g_INSTRUCTS(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_JOB(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_METHOD_VERBS(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MEANS_OF(), "search", &[opening.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    words = _words(&[opening.clone()]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_VERB_HEAD(), "match", &[opening.clone()]) } }) {
    return Py::Bool(false);
    }
    second = geti(&words.clone(), &Py::Int(1));
    return Py::Bool(!py_truthy(&{ let __b = mcall(&second.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&second.clone(), "endswith", &[py_str("ss")]))) } }));
    Py::None
}
pub fn _do_with_ask(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks what to do with something, however it spells the ask.");
    return call_builtin("bool", &[mcall(&g_ANY_DO_WITH(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_for_words(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks what to say, rather than what to do about the thing in it.\n\n    The shape of answer is the words themselves, so a card that answers the thing named - how to\n    wash up for \"how do i tell my flatmate to do the dishes\", what bad breath is for \"how do i\n    tell my friend her breath smells\" - is on the right topic and answers a question nobody\n    asked. Read from the whole line, and only from an ask: \"she said i was rude and i dont know\n    what to say\" is the same ask, and \"how do i tell if the steak is done\" is not one at all.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { let __b = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]); if py_truthy(&__b) { __b } else { _said(&[understood.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WORDS_ASK(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NAMED_WORDS(), "search", &[said.clone()])) {
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g_TELLS_A_TEST(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_WORDS_ANYWAY(), "search", &[said.clone()]) } }));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("subject")]))); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("followup")]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_WRITES_IN_A_DOCUMENT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_WRITES_TO_A_MESSAGE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_COVER_LETTER(), "search", &[said.clone()]))) } })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_NOT_WORDS_ASK(), "search", &[said.clone()])));
    Py::None
}
pub fn gives(args: &[Py]) -> Py {
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
    let mut want: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut answer: Py = Py::None;
    let _ = py_str("True when `reply` gives an answer of kind `want`, False when the check can name what is\n    wrong with it, None when words cannot tell.\n\n    A question put back to the person is never an answer, whatever shape the check read out of it:\n    see `is_a_question_back`. That test is applied to the credit and not to the whole reply, so it\n    can only take a credit away. A reply the cues cannot read stays the \"words cannot tell\" it was,\n    because the check only calls a reply a miss when it can name what the reply is instead, and a\n    reply it cannot read is not named.\n    ");
    answer = _the_shape_a_reply_gives(&[reply.clone(), want.clone(), line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&answer.clone(), &Py::Bool(true))); if !py_truthy(&__b) { __b } else { is_a_question_back(&[reply.clone(), line.clone()]) } }) {
    return Py::Bool(false);
    }
    return answer.clone();
    Py::None
}
pub fn _the_shape_a_reply_gives(args: &[Py]) -> Py {
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
    let mut want: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut both: Py = Py::None;
    let mut check: Py = Py::None;
    let mut head: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut other: Py = Py::None;
    let mut repeat: Py = Py::None;
    let mut repeated: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut strong: Py = Py::None;
    let _ = py_str("The check itself: the shapes a reply's opening carries, and what it is instead.\n\n    Everything the docstring of `gives` says about a true, a false and a \"words cannot tell\" is\n    about this function.\n\n    Only the opening sentence is read, because STYLE.md asks for the answer in the first sentence\n    and a figure buried in the third paragraph of a paragraph about something else is not the\n    reply the person got. A method is the one shape read a sentence further, because its opening\n    is often the reason: \"Sticking is nearly always a wet dough meeting a dry surface. Chill the\n    pastry 20 to 30 minutes before rolling.\" A want this module does not know, or one of the soft\n    shapes, is None.\n\n    False is a claim about the reply, so the check only makes it when it can say what the reply\n    is instead: a shrug, or an unmistakable other shape - a duration, a figure, a method, a\n    named kind of place, a role, what the thing is, a verdict or a named side. A reply in words\n    the cue list has nothing for is None, not a miss: \"Vinegar does not go off\" is the answer to\n    \"does vinegar go off\", and \"Granita is fruit juice or coffee\" is the answer to \"what is\n    granita\", and neither carries a cue this module knows.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_WANTS(), &want.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SOFT(), &want.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    line = { let __b = mcall(&line.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } };
    }
    opening = first_sentence(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::None;
    }
    repeat = if py_truthy(&Py::Bool(py_eq(&want.clone(), &g_YESNO()))) { mcall(&g_REPEAT_LEAD(), "match", &[opening.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = repeat.clone(); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&opening.clone(), &mcall(&repeat.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]) } }) {
    opening = mcall(&slice_or_raise(&opening.clone(), &mcall(&repeat.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    repeated = Py::Bool(true);
    } else {
    repeated = Py::Bool(false);
    }
    if py_truthy(&is_a_punt(&[opening.clone()])) {
    head = { let __b = slice_or_raise(&reply.clone(), &Py::None, &Py::Int(400), &Py::None); if py_truthy(&__b) { __b } else { py_str("") } };
    return if py_truthy(&{ let __b = mcall(&g_DIRECTED(), "search", &[head.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_HOLDER(), "search", &[head.clone()]) } }) { Py::None } else { Py::Bool(false) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { _a_sign_off(&[opening.clone()]) } }) {
    return if py_truthy(&mcall(&g__A_SOCIAL_OPEN(), "match", &[{ let __b = _line_text(&[line.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }])) { Py::None } else { Py::Bool(false) };
    }
    check = mcall(&g__CHECKS(), "get", &[want.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_TIME())); if !py_truthy(&__b) { __b } else { _a_money_rate(&[opening.clone()]) } }) {
    stripped = _without_a_money_rate(&[opening.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&stripped.clone(), &opening.clone()))) {
    return call_builtin("bool", &[_a_time(&[stripped.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    if py_truthy(&Py::Bool(py_eq(&want.clone(), &g_STEPS()))) {
    if py_truthy(&_steps_in(&[reply.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_imperative(&[opening.clone()])) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_TIME())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_STAGE_WHEN(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SEASON_WHEN(), "search", &[opening.clone()]) } }; if !py_truthy(&__b) { __b } else { mcall(&g_ASKS_WHEN(), "search", &[_line_text(&[line.clone()])]) } } } } } }) {
    return Py::Bool(true);
    } else {
    if py_truthy(&Py::Bool(py_eq(&want.clone(), &g_REASON()))) {
    if py_truthy(&_a_reason(&[opening.clone(), line.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = _names(&[_opening_two(&[reply.clone()]), _line_text(&[line.clone()])]); if !py_truthy(&__b) { __b } else { _a_reason(&[_opening_two(&[reply.clone()]), line.clone()]) } } } }) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&Py::Bool(py_eq(&want.clone(), &g_YESNO()))) {
    if py_truthy(&_the_rule_settles_it(&[opening.clone(), line.clone(), reply.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_yesno(&[opening.clone(), line.clone()])) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_AMOUNT())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_ASKS_AN_AGE(), "search", &[_line_text(&[line.clone()])]) } } } }) {
    if py_truthy(&_an_age_is_given(&[opening.clone()])) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&{ let __b = check.clone(); if !py_truthy(&__b) { __b } else { py_call(&check, &[opening.clone()]) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_AMOUNT())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = _a_price_asked(&[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _a_time(&[opening.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PRICE_GIVEN(), "search", &[opening.clone()]))) } } } } } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_TIME())); if !py_truthy(&__b) { __b } else { _a_time_a_sentence_on(&[reply.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    }
    }
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_ASKS_THE_BOT(), "search", &[_line_text(&[line.clone()])]) } } } }) {
    if py_truthy(&{ let __b = mcall(&g_I_OFFER(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { _about_the_thing_the_line_named(&[opening.clone(), _line_text(&[line.clone()])]) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_an_outcome_of_their_own_doing(&[_line_text(&[line.clone()])]))); if !py_truthy(&__b) { __b } else { _about_the_thing_the_line_named(&[opening.clone(), _line_text(&[line.clone()])]) } } } } } }) {
    if py_truthy(&{ let __b = _a_meaning(&[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_reason(&[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_time(&[opening.clone()]); if py_truthy(&__b) { __b } else { _an_amount_with_a_unit(&[opening.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { _the_quality_it_was_asked_about(&[opening.clone(), _line_text(&[line.clone()])]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&repeated.clone())); if !py_truthy(&__b) { __b } else { _the_job_answers(&[opening.clone(), line.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&repeated.clone())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASKS_IF_SERIOUS(), "search", &[_line_text(&[line.clone()])]); if !py_truthy(&__b) { __b } else { mcall(&g_SEEKS_HELP(), "search", &[reply.clone()]) } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { _the_thing_is_named_as_what_it_usually_is(&[opening.clone(), line.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_VERDICT_ABOUT_IT(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { _names_what_the_line_named(&[opening.clone(), _line_text(&[line.clone()])]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { mcall(&g_VERDICT_RULE(), "search", &[reply.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { _says_no_about_the_asked_thing(&[opening.clone(), line.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { _the_side_wins(&[opening.clone(), line.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { _an_echoed_no(&[opening.clone(), line.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { _the_figure_against_the_standard(&[opening.clone(), _line_text(&[line.clone()])]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { _a_figure_then_the_verdict(&[opening.clone(), reply.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_TIME())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_EXTENT(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_LENGTH_ASKED(), "search", &[_line_text(&[line.clone()])]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_PICK())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { _names_a_side(&[opening.clone(), _line_text(&[line.clone()])]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_PICK())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = _an_instruction(&[opening.clone()]); if !py_truthy(&__b) { __b } else { _names(&[opening.clone(), line.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_OPTIONS())); if !py_truthy(&__b) { __b } else { line.clone() } }) {
    said = _line_text(&[line.clone()]);
    if py_truthy(&{ let __b = mcall(&g_OPTIONS_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_INSTRUCTS(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_JOB(), "match", &[opening.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    both = if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_COMPARE())); if !py_truthy(&__b) { __b } else { line.clone() } }) { _names_both_sides(&[reply.clone(), _line_text(&[line.clone()])]) } else { Py::None };
    if py_truthy(&both.clone()) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_COMPARE())); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&both.clone(), &Py::None)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_REASON())); if !py_truthy(&__b) { __b } else { _head_explains(&[opening.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_MEANING())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { _defines_the_thing(&[opening.clone(), _line_text(&[line.clone()])]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_MEANING())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = _a_past_definition(&[opening.clone()]); if !py_truthy(&__b) { __b } else { _names_what_the_line_named(&[opening.clone(), _line_text(&[line.clone()])]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_MEANING())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_ORIGIN_ASK(), "search", &[_line_text(&[line.clone()])]); if py_truthy(&__b) { __b } else { mcall(&g_MADE_OF(), "search", &[_line_text(&[line.clone()])]) } }; if !py_truthy(&__b) { __b } else { _an_origin(&[opening.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_OPTIONS())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_imperative(&[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_VERDICT_OPEN(), "match", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_LIST_AFTER_COLON(), "search", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _a_pick(&[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CHOICE(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { ge(&_marked_things(&[opening.clone()]), &Py::Int(2)) } } } } } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_OPTIONS())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_COUNTED_LIST(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_TIME_COUNT(), "search", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_an_instruction(&[opening.clone()]))); if !py_truthy(&__b) { __b } else { _a_list_of_things(&[opening.clone()]) } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_STEPS())); if !py_truthy(&__b) { __b } else { line.clone() } }) {
    said = _line_text(&[line.clone()]);
    if py_truthy(&{ let __b = _options(&[opening.clone()]); if !py_truthy(&__b) { __b } else { _names_what_the_line_named(&[opening.clone(), said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_OPTIONS(), g_STEPS()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let __b = line.clone(); if !py_truthy(&__b) { __b } else { _do_with_ask(&[_line_text(&[line.clone()])]) } } } }) {
    if py_truthy(&{ let __b = _options(&[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = _steps_in(&[reply.clone()]); if py_truthy(&__b) { __b } else { _an_instruction(&[opening.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_YESNO(), g_PICK()]), &want.clone())); if !py_truthy(&__b) { __b } else { _the_rule_names_both_sides(&[reply.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_OPTIONS())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_an_instruction(&[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_VERDICT_OPEN(), "match", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { _three_names(&[_opening_two(&[reply.clone()])]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { _the_same_subject(&[opening.clone(), line.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__CLEAR());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    other = __unpk2[0].clone();
    strong = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &want.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&want.clone(), &g_STEPS())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__FIGURES(), &other.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &g_MEANING())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![g_PICK(), g_OPTIONS()]), &want.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &g_MEANING())); if !py_truthy(&__b) { __b } else { mcall(&g_FIGURE_HEAD(), "match", &[opening.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &g_AMOUNT())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&want.clone(), &g_YESNO())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &g_REASON())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![g_STEPS(), g_YESNO()]), &want.clone())) } }) {
    continue;
    }
    if py_truthy(&py_call(&strong, &[opening.clone()])) {
    if py_truthy(&is_directed(&[opening.clone()])) {
    return Py::None;
    }
    return Py::Bool(false);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_sign_answers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut want: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut opening: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when an ask about how to tell, or whether a thing is done, is answered by the sign.");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&want.clone(), &g_STEPS()))) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_TELL_ASK(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    } else {
    if py_truthy(&Py::Bool(py_eq(&want.clone(), &g_YESNO()))) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_IS_IT_DONE(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    } else {
    return Py::Bool(false);
    }
    }
    opening = first_sentence(&[reply.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&opening.clone())); if py_truthy(&__b) { __b } else { is_a_punt(&[opening.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_THE_SIGN(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn gives(args: &[Py]) -> Py {
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
    let mut wants_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wants_") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wants_"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut read: Py = Py::None;
    let _ = py_str("`gives`, plus the sign that settles a how-do-I-know or is-it-done ask.");
    read = py_call(&g__gives_as_read(), &[reply.clone(), wants_.clone(), line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&read.clone(), &Py::Bool(false))); if !py_truthy(&__b) { __b } else { _a_sign_answers(&[wants_.clone(), line.clone(), reply.clone()]) } }) {
    return Py::Bool(true);
    }
    return read.clone();
    Py::None
}
pub fn missing(args: &[Py]) -> Py {
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
    let mut wants_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wants_") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wants_"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut gone: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("The wants in `wants_` (one label or a list) that `reply` clearly does not give.");
    if py_truthy(&call_builtin("isinstance", &[wants_.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    wants_ = py_list(vec![wants_.clone()]);
    }
    gone = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = wants_.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    want = __item.clone();
    if py_truthy(&Py::Bool(py_is(&gives(&[reply.clone(), want.clone(), line.clone()]), &Py::Bool(false)))) {
    let _ = mcall(&gone.clone(), "append", &[want.clone()]);
    }
    }
    return gone.clone();
    Py::None
}
pub fn second_sentence(args: &[Py]) -> Py {
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
    let mut parts: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The sentence after the opening one, tidied, or \"\" when the reply has only one.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    parts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(text.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return if py_truthy(&gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { mcall(&geti(&parts.clone(), &Py::Int(1)), "strip", &[]) } else { py_str("") };
    Py::None
}
pub fn _a_thing_to_define(args: &[Py]) -> Py {
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
    let mut whole: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let _ = py_str("Is this \"what is X\" asking what X is, rather than one of the other things the words open?");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&phrase.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&phrase.clone(), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_BARE_NOTHING_ASKED(), &mcall(&mcall(&phrase.clone(), "strip", &[]), "lower", &[])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_BARE_NOT_A_THING_AT_THE_START(), "match", &[phrase.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_BARE_VALUE_NOUN(), "match", &[phrase.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:mean|means|work|works)\\b"), phrase.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_BARE_NOT_A_THING_ANYWHERE(), "search", &[whole.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_BARE_A_SECOND_QUESTION(), "search", &[phrase.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_BARE_A_RECALL(), "search", &[phrase.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&mcall(&mcall(&phrase.clone(), "rstrip", &[]), "lower", &[]), "endswith", &[py_str(" for")])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_bare_definition(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("`what is nectar` asks what the thing is: a bare noun carries the ask the label missed.");
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("about_bot")])) {
    return Py::Bool(false);
    }
    whole = mcall(&g_BARE_WHAT_AS_IS(), "sub", &[py_str("what is"), mcall(&g_LEADING_JOIN(), "sub", &[py_str(""), _whole(&[understood.clone()])])]);
    plain = mcall(&g_BARE_WHAT_IS_PLAIN(), "match", &[whole.clone()]);
    return call_builtin("bool", &[{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { _a_thing_to_define(&[mcall(&plain.clone(), "group", &[py_str("thing")]), whole.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_time_a_sentence_on(args: &[Py]) -> Py {
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
    let mut following: Py = Py::None;
    let mut found: Py = Py::None;
    let mut lead_in: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the sentence after the reply's opening gives a time as its answer.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    opening = first_sentence(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&text.clone(), &call_builtin("len", &[opening.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "lstrip", &[py_str(" .!?")]);
    following = if py_truthy(&rest.clone()) { first_sentence(&[rest.clone()]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&following.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_DURATION(), g_CLOCK(), g_STAGE(), g_NAMED_CLOCK()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[following.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    lead_in = slice_or_raise(&mcall(&slice_or_raise(&following.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "split", &[]), &neg(&Py::Int(4)), &Py::None, &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lead_in.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TIME_LEAD_IN(), &stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&word.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn first_sentence(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut end: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The reply's opening sentence, tidied, or \"\" for an empty reply.\n\n    A full stop inside a figure (\"1.5 bar\") has no space after it, so it does not end a sentence;\n    the split needs the space a real sentence ends with. A full stop after an abbreviation has one,\n    so the split skips it and reads on to the sentence the reply really opens with.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SENTENCE_END(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    end = __item.clone();
    before = slice_or_raise(&text.clone(), &Py::None, &mcall(&end.clone(), "start", &[]), &Py::None);
    if py_truthy(&mcall(&g_ABBREVIATION_END(), "search", &[before.clone()])) {
    continue;
    }
    return mcall(&before.clone(), "strip", &[]);
    }
    return text.clone();
    Py::None
}
pub fn _opening_two(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The reply's first two sentences, tidied, or \"\" for an empty reply.\n\n    A short verdict or a figure often opens the reply and the list the person asked for follows in\n    the next sentence: \"About 0.5kg of raw joint for 2, which is 250g a person. Then potatoes,\n    carrots, parsnips or swede\". Two sentences and not the whole reply, so a list that has nothing\n    to do with the answer cannot make the reply pass.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g_SENTENCE_END(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    end = __item.clone();
    if py_truthy(&mcall(&g_ABBREVIATION_END(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&end.clone(), "start", &[]), &Py::None)])) {
    continue;
    }
    let __aug2 = add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&Py::Bool(py_eq(&seen.clone(), &Py::Int(2)))) {
    return mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&end.clone(), "start", &[]), &Py::None), "strip", &[]);
    }
    }
    return text.clone();
    Py::None
}
pub fn _three_names(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut item: Py = Py::None;
    let mut items: Py = Py::None;
    let mut opened: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut run: Py = Py::None;
    let mut short: Py = Py::None;
    let mut trailing: Py = Py::None;
    let _ = py_str("True when three or more things are named in one run, not prose that has commas in it.\n\n    A list of things is names and joining words and nothing else: \"Litter tray and litter, two\n    bowls, a scratching post, a carrier\". A sentence that carries on about something has a verb or\n    a pronoun in it - \"you put in 5% and your employer at least 3%, and the tax relief comes on\n    top\" - and that is prose with commas, not a list. So each run between the verbs and the\n    pronouns is read, and three or more short items inside one of those runs is the list. Long\n    items are left out: a clause with more than four words in it is a sentence, not a thing.\n    ");
    plain = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?;:])\\s+"), plain.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&lt(&mcall(&clause.clone(), "count", &[py_str(",")]), &Py::Int(2))) {
    continue;
    }
    opened = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^[\\W]*(?:then|next|after that|afterwards|finally|lastly)\\b")); __args.push(py_str("")); __args.push(clause.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&mcall(&g_STEPS_INSIDE_A_LIST(), "search", &[opened.clone()])) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_NAME_STOP(), "split", &[opened.clone()]));
    for __item in __iter2 {
    run = __item.clone();
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s*,\\s*|\\s+(?:and|or|plus)\\s+"), run.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut item: Py = __item.clone(); __out.push(mcall(&item.clone(), "strip", &[])); } py_list(__out) })() };
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut item: Py = __item.clone(); if py_truthy(&item.clone()) {  __out.push(stdlib_call("re", "sub", &[py_str("^(?:and|or|plus)\\s+"), py_str(""), item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut item: Py = __item.clone(); if py_truthy(&item.clone()) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&run.clone(), &py_str("(")))) {
    continue;
    }
    short = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut item: Py = __item.clone(); if py_truthy(&le(&call_builtin("len", &[mcall(&item.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {  __out.push(item.clone()); } } py_list(__out) })() };
    trailing = { let __b = Py::Bool(py_eq(&sub(&call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&geti(&items.clone(), &neg(&Py::Int(1))), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } };
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { trailing.clone() } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    return call_builtin("bool", &[mcall(&g_FIGURE(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut first_clause: Py = Py::None;
    let mut said: Py = Py::None;
    if py_truthy(&mcall(&g_NAMED_CLOCK(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_EDGED_PART_OF_DAY(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    said = if py_truthy(&mcall(&g_RATE_PHRASE(), "match", &[opening.clone()])) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(opening.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATE_PHRASE(), "sub", args))(&__args) } } else { opening.clone() };
    said = if py_truthy(&mcall(&g_RATE_OF_A_COUNT(), "search", &[said.clone()])) { mcall(&g_RATE_OF_A_COUNT(), "sub", &[py_str(" "), said.clone()]) } else { said.clone() };
    if py_truthy(&mcall(&g_NO_FIXED_TIME(), "match", &[said.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_DURATION(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CLOCK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BARE_RANGE(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STAGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NEVER(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NO_FIXED_TIME(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MINUTES_AS_A_LENGTH(), "search", &[said.clone()]) } } } } } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_INSTRUCTS(), "match", &[opening.clone()])) {
    return Py::Bool(false);
    }
    first_clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;:]")); __args.push(said.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    return call_builtin("bool", &[{ let __b = mcall(&g_NAMED_TIME(), "search", &[first_clause.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DAY_NAME(), "search", &[first_clause.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DATE(), "search", &[first_clause.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_clear_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("The time readings strong enough to fail a reply to a different question.\n\n    A figure with a unit, an hour on the clock, a weekday, a date, the part of a day named as\n    \"next morning\" are all what a reply is *about*. A bare season is not, and neither is a word\n    list; see `_CLEAR`.\n    ");
    return call_builtin("bool", &[{ let __b = mcall(&g_DURATION(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CLOCK_SURE(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BARE_RANGE(), "match", &[opening.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_age_is_given(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    return call_builtin("bool", &[{ let __b = mcall(&g__AN_AGE_YEARS(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__AN_AGE_MONTHS(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__AN_AGE_BIRTHDAY(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__AN_AGE_NAMED(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__AN_AGE_BARE(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__AN_AGE_SPELLED_OPENS(), "match", &[opening.clone()]) } } } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_amount(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    return call_builtin("bool", &[{ let __b = mcall(&g_FIGURE(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MONEY(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_RATE(), "search", &[opening.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_price_asked(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks what something costs, in a money word of its own.");
    return call_builtin("bool", &[mcall(&g_PRICE_ASKED(), "search", &[_line_text(&[line.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_figure_that_counts(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("A figure that makes an opening a figure: a digit, a price, a rate, a measured amount, or a\n    count that is the whole reply.");
    return call_builtin("bool", &[{ let __b = _an_amount_to_read(&[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_BARE_NUMBER(), "match", &[mcall(&opening.clone(), "strip", &[])]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_amount_to_read(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("The amount only as evidence: a digit, a price or a rate, or a number counting a measure.");
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d"), opening.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MONEY(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_RATE(), "search", &[opening.clone()]) } } } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_MEASURED(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _figure_at_the_head(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("Whether `pattern` matches within the first `_FIGURE_HEAD` words of `opening`.");
    match_ = mcall(&pattern.clone(), "search", &[opening.clone()]);
    return { let __b = Py::Bool(!py_is(&match_.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&slice_or_raise(&opening.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__FIGURE_HEAD()) } };
    Py::None
}
pub fn _a_clear_time_at_the_head(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("`_a_clear_time` as evidence: the time has to be where the reply starts.");
    return { let __b = _figure_at_the_head(&[opening.clone(), g_DURATION()]); if py_truthy(&__b) { __b } else { { let __b = _figure_at_the_head(&[opening.clone(), g_CLOCK_SURE()]); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_BARE_RANGE(), "match", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _an_amount_at_the_head(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("`_an_amount` as evidence: the figure has to be where the reply starts, and a number\n    written out has to be counting a measure rather than sitting in front of a noun.");
    if py_truthy(&{ let __b = _figure_at_the_head(&[opening.clone(), g_MONEY()]); if py_truthy(&__b) { __b } else { _figure_at_the_head(&[opening.clone(), g_RATE()]) } }) {
    return Py::Bool(true);
    }
    match_ = mcall(&g_FIGURE(), "search", &[opening.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&match_.clone(), &Py::None)); if py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[mcall(&slice_or_raise(&opening.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__FIGURE_HEAD()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&geti(&mcall(&match_.clone(), "group", &[Py::Int(0)]), &Py::Int(0)), "isdigit", &[])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__MEASURE_AFTER(), "match", &[opening.clone(), mcall(&match_.clone(), "end", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_party(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut found: Py = Py::None;
    let _ = py_str("True when the opening names the party as a role or an office, not as a thing.");
    let __iter1 = py_iter_or_raise(&mcall(&g__PARTY(), "finditer", &[opening.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g__NOT_A_ROLE(), &mcall(&geti(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[]), &neg(&Py::Int(1))), "lower", &[])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_spot_a_thing_has(args: &[Py]) -> Py {
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
    let _ = py_str("The part named after the preposition, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), "strip", &[py_str("'-")]);
    Py::None
}
pub fn _names_a_spot(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut found: Py = Py::None;
    let _ = py_str("True when the reply says where on the thing it is: \"on a plate\", \"under the case\".");
    if py_truthy(&mcall(&g_SPOT_OF_THE_THING(), "search", &[{ let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SPOT_ON_THE_THING(), "finditer", &[{ let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_SPOT_PART(), &_the_spot_a_thing_has(&[found.clone()])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    opening = mcall(&g_CALENDAR_WORD(), "sub", &[py_str(" "), opening.clone()]);
    return call_builtin("bool", &[{ let __b = mcall(&g_PLACE_WORD(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PLACE_NAME(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = ge(&call_builtin("len", &[mcall(&g_PLACE_NAME_ANY(), "findall", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SETTLED_ON(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = _names_a_spot(&[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PLACE_KIND(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLACE_AT_HOME(), "search", &[opening.clone()]) } } } } } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    opening = mcall(&g_CALENDAR_WORD(), "sub", &[py_str(" "), opening.clone()]);
    if py_truthy(&mcall(&g_PERSON_RELATION(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = _a_party(&[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PERSON_NAMED(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PERSON_FULL_NAME(), "match", &[opening.clone()]) } } } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_PERSON_WORD(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PERSON_NAME(), "search", &[opening.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_origin(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    return call_builtin("bool", &[{ let __b = mcall(&g_SOURCE_MAKES(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SOURCE_COLON(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SOURCE_MADE(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SOURCE_DOES(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SOURCE_LIST(), "match", &[opening.clone()]) } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_past_definition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut found: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("True when the reply opens by defining a thing in the past tense. A pronoun or a dummy\n    subject is never the thing being defined, so the words before the copula are read first.");
    found = mcall(&g_PAST_DEFINES(), "search", &[opening.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    subject = mcall(&geti(&stdlib_call("re", "split", &[py_str("[.;:!?,()\\u2014]"), slice_or_raise(&opening.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(1))), "strip", &[]);
    if py_truthy(&mcall(&g_NOT_A_SUBJECT(), "search", &[subject.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("(?:my|your|his|her|its|our|their|mine|yours|theirs)\\b"), subject.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_meaning(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut hit: Py = Py::None;
    if py_truthy(&mcall(&g__OPINION_OF_THE_THING(), "match", &[mcall(&call_builtin("str", &[{ let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_MECHANISM(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_MEANS_BY_DOING(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MEANS_INFINITIVE(), "match", &[opening.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_DEFINES(), "match", &[opening.clone()])) {
    return Py::Bool(true);
    }
    hit = mcall(&g_MEANS(), "search", &[opening.clone()]);
    if py_truthy(&Py::Bool(py_is(&hit.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_MEANS_ARTICLE(), "match", &[mcall(&hit.clone(), "group", &[Py::Int(0)])]); if !py_truthy(&__b) { __b } else { mcall(&g_A_JUDGEMENT(), "match", &[opening.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_quality_is_about_the_thing(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the subject of \"is great/free/cheap\" is the thing the line asks about.\n\n    A quality word is a verdict about whatever noun is nearest, and the nearest noun is often the\n    wrong one: \"is it worth appealing the ticket\" / \"The form is free to download, but the appeal\n    usually loses\" answers with a quality of the form. So the subject of the quality has to be the\n    thing the line named, or the pronoun standing in for it (\"so it is fine\" after \"is it ok\").\n    ");
    found = mcall(&g_QUALITY_IS(), "search", &[clause.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    subject = mcall(&slice_or_raise(&clause.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(true);
    }
    words = _words(&[subject.clone()]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__PRONOUN_SUBJECT(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    said = _line_text(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = ge(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__EMPTY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    return { let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_tail_says_what_it_is_about(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the verdict in the setup's tail is said about the thing the line asked about.\n\n    A tail verdict counts when the clause says what it judges: \"is my mince gone off\" / \"Red meat\n    is more forgiving than chicken: a grey-brown colour on the outside of a chop is just air and it\n    is fine\". \"should i pop it\" / \"A smooth lump on the back of the wrist is usually a ganglion\n    cyst: harmless, often smaller after a few weeks\" says a quality of nothing - the thing is\n    harmless, but the reply never says the blister is - and that is a wrong reply, not a verdict\n    (young-latch's find).\n\n    A named subject is not enough on its own, because the tail is often a detail rather than the\n    verdict: \"can i put my glasses in the washing machine\" / \"... load it so a hand fits flat on\n    top of the pile - a packed drum does not rinse\" matched the negation in \"a packed drum does\n    not rinse\", \"is it worth buying a diesel now\" / \"There is no fixed figure: the forecourt sign\n    is the only price that matters\" matched \"only\", and \"is that normal\" / \"... quotes ... tell\n    you whether a figure is normal\" matched \"is normal\" while naming a test rather than giving\n    one. So the subject has to be the pronoun standing in for the line's thing, or a word the line\n    named, the same reading the quality arms use.\n    ");
    found = mcall(&g_VERDICT_INSIDE(), "search", &[clause.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    subject = mcall(&slice_or_raise(&clause.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(false);
    }
    words = _words(&[subject.clone()]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__PRONOUN_SUBJECT(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    said = _line_text(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = ge(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__EMPTY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    return { let __b = call_builtin("bool", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut w: Py = __item.clone(); __out.push(_the_same_word(&[w.clone(), said.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_quality_it_was_asked_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut asked: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut named: Py = Py::None;
    let mut q: Py = Py::None;
    let mut quality: Py = Py::None;
    let mut w: Py = Py::None;
    let mut x: Py = Py::None;
    let _ = py_str("True when the line asked \"is it <quality>\" and the opening says <quality> of it.\n\n    The word has to be near the head of the reply, where the verdict lives: \"worth fixing, and\n    cheap\" is the answer to \"is it worth fixing\", while a \"worth\" in a closing aside is not.\n    ");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[said.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = ge(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__EMPTY(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_THING(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__QUALITIES()) { let mut q: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&q.clone(), "split", &[])) { let mut x: Py = __item.clone(); __out.push(x.clone()); } } py_set(__out) })() }, &w.clone())) } } } } } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { { let __b = said.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_ASKS_ABOUT_A_QUALITY(), "search", &[said.clone()]) } } } }) {
    asked = mcall(&py_str(" "), "join", &[slice_or_raise(&_words(&[opening.clone()]), &Py::None, &g__THE_VERDICT_IS_EARLY(), &Py::None)]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__QUALITIES()) { let mut q: Py = __item.clone(); __out.push(mcall(&geti(&g__QUALITY_SAID(), &q.clone()), "search", &[asked.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_ABOUT_A_QUALITY(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    head = mcall(&py_str(" "), "join", &[slice_or_raise(&_words(&[opening.clone()]), &Py::None, &g__THE_VERDICT_IS_EARLY(), &Py::None)]);
    let __iter1 = py_iter_or_raise(&g__QUALITIES());
    for __item in __iter1 {
    quality = __item.clone();
    found = mcall(&geti(&g__QUALITY_SAID(), &quality.clone()), "search", &[head.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { mcall(&geti(&g__QUALITY_SAID(), &quality.clone()), "search", &[said.clone()]) } }))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__QUALITY_OF_A_THING(), &quality.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_quality_is_said_of_the_thing(&[head.clone(), mcall(&found.clone(), "start", &[]), said.clone()]))) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_quality_is_said_of_the_thing(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut subject: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the quality at `found` is said of the line's own thing.\n\n    A quality word is a verdict about whatever the nearest noun is, and the nearest noun is\n    sometimes another thing altogether: \"my cat is flat out all day, is that normal\" / \"Dogs need\n    a walk every day, so a lively dog is normal\" is a quality of a dog, and the check read it as\n    the verdict for a question about a cat. The same words with a pronoun for the subject - \"so\n    they are normal\" - say the quality of whatever \"they\" is, so the reply also has to name\n    something the line named before that counts.\n\n    A quality with no copula in front of it is the verdict itself and is left alone: \"Late but\n    still inside normal, and worth a light touch rather than a push\" answers \"is it normal that a\n    5 year old cant write his name\" and names no subject at all.\n    ");
    words = mcall(&slice_or_raise(&head.clone(), &Py::None, &at.clone(), &Py::None), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__COPULA(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut w: Py = __item.clone(); if py_truthy(&mcall(&w.clone(), "isalpha", &[])) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g__PRONOUN_SUBJECT(), &geti(&subject.clone(), &neg(&Py::Int(1)))))) {
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _about_the_thing_the_line_named(&[head.clone(), said.clone()]) } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&subject.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_verdict_is_inside(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut at: Py = Py::None;
    let mut found: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("`VERDICT_INSIDE`, with a bare quality read against the thing the line names.");
    found = mcall(&g_VERDICT_INSIDE_QUALITY(), "search", &[clause.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return call_builtin("bool", &[mcall(&g_VERDICT_INSIDE(), "search", &[clause.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    at = mcall(&clause.clone(), "index", &[mcall(&found.clone(), "group", &[py_str("q")]), mcall(&found.clone(), "start", &[])]);
    if py_truthy(&_the_quality_is_said_of_the_thing(&[clause.clone(), at.clone(), { let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(true);
    }
    rest = add(&add(&slice_or_raise(&clause.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&clause.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    return call_builtin("bool", &[mcall(&g_VERDICT_INSIDE(), "search", &[rest.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_quality_answers(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let _ = py_str("The two quality arms, read against the line: see `QUALITY_IS` and `IT_HELPS`.");
    if py_truthy(&_the_quality_is_about_the_thing(&[clause.clone(), line.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_IT_HELPS(), "search", &[clause.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__WORKING(), "search", &[_line_text(&[line.clone()])]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn is_a_punt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("True when the reply's opening says the bot has nothing to give, in whatever clothes.\n\n    Every shape check asks this first, so a hand-off is never a figure, a place, a method or a\n    verdict, whatever words follow it.\n    ");
    return call_builtin("bool", &[{ let __b = mcall(&g_PUNT(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HANDOFF(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REFERS_BACK(), "match", &[opening.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn is_directed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("True when the opening sends the person to a source that holds the answer and names it.");
    return call_builtin("bool", &[mcall(&g_DIRECTED(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_first_sentence(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut char: Py = Py::None;
    let mut index: Py = Py::None;
    let mut quoted: Py = Py::None;
    let _ = py_str("The opening sentence, with a question mark inside a quotation left where it is.");
    quoted = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    char = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g__A_QUOTE(), &char.clone()))) {
    quoted = Py::Bool(!py_truthy(&quoted.clone()));
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&quoted.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str(".!?"), &char.clone())) } }) {
    after = slice_or_raise(&reply.clone(), &add(&index.clone(), &Py::Int(1)), &add(&index.clone(), &Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { mcall(&after.clone(), "isspace", &[]) } }) {
    return slice_or_raise(&reply.clone(), &Py::None, &add(&index.clone(), &Py::Int(1)), &Py::None);
    }
    }
    }
    }
    return reply.clone();
    Py::None
}
pub fn _the_echo_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the question only hands the person's own words back before answering.");
    said = mcall(&g__WORD(), "findall", &[mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__WORD(), "findall", &[mcall(&opening.clone(), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_A_WORD(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn is_a_question_back(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut opening: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the reply's first sentence asks the person something instead of answering.");
    text = mcall(&{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    opening = _the_first_sentence(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&mcall(&opening.clone(), "rstrip", &[]), "endswith", &[py_str("?")])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_echo_question(&[opening.clone(), line.clone()])))) {
    return Py::Bool(true);
    }
    rest = mcall(&slice_or_raise(&mcall(&text.clone(), "strip", &[]), &call_builtin("len", &[opening.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = is_a_punt(&[first_sentence(&[rest.clone()])]); if py_truthy(&__b) { __b } else { mcall(&g__NO_WAY_TO_KNOW(), "search", &[rest.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_outcome_of_their_own_doing(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True for \"will it break if i leave it\", False for \"can i send it back if i change my mind\".");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g_HYPOTHETICAL(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_OUTCOME_ASK(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_the_lines_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut reply: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the reply's opening names a thing the line named, by stem and from three letters.\n\n    `_names` wants four letters, which is right for the sides of a pick (\"a fiesta or a focus\")\n    and too long for the everyday nouns a verdict is said about: tea, egg, sun, ice, cat.\n    ");
    reply = call_builtin("set", &[_words(&[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__EMPTY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&reply.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&reply.clone(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_no_about_the_asked_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut asked: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("True when the reply negates the very thing the line asked about.\n\n    \"does tea go stale\" / \"Tea does not go off, it goes stale\" is the answer, said as a fact about\n    the tea rather than as a verdict word. The subject in front of the no has to name the line's\n    own thing, so \"not the ticket seller\" in a delay-repay answer is not read as a verdict about\n    the train. An order about the line's own verb counts too: \"so don't eat it\" answers \"is it ok\n    to eat raw chicken past its date\".\n    ");
    said = _line_text(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&mcall(&opening.clone(), "rstrip", &[py_str("\"” ")]), "endswith", &[py_str("?")])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SAYS_NO(), "finditer", &[opening.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&_names_the_lines_thing(&[mcall(&match_.clone(), "group", &[py_str("subject")]), said.clone()])) {
    return Py::Bool(true);
    }
    }
    asked = call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&mcall(&g_NO_JOB(), "finditer", &[opening.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    verb = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asked.clone(), &verb.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[verb.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&asked.clone(), &slice_or_raise(&verb.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_job_is_split(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("True when the reply divides the job between the person and a trade they would need.");
    if py_truthy(&mcall(&g_MUST_BE_A_TRADE(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_THEIR_OWN(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_TRADE(), "search", &[opening.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:s|es)?\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_what_the_line_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut first_clause: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply's first clause uses a word of its own that the line also used.\n\n    The words a line and a reply share because both are English say nothing, so they are dropped\n    before the comparison; what is left is the name of the thing that was asked about. The name has\n    to sit in the clause that carries the verdict, because that is what the verdict is said about:\n    \"is a peace lily safe for cats\" / \"Lilies are the worst plant for cats\" names its subject,\n    while a word met further along the sentence (\"... a year of rent tax free\" answering a council\n    tax question) is a coincidence of subject matter, not an answer about it.\n    ");
    words = sub(&call_builtin("set", &[mcall(&g__NAMING_WORD(), "findall", &[mcall(&said.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)), &g_NAMING_STOP());
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    first_clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;:]")); __args.push(opening.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(_the_same_word(&[w.clone(), mcall(&first_clause.clone(), "lower", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_rule_names_both_sides(args: &[Py]) -> Py {
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
    let mut drop: Py = Py::None;
    let _ = py_str("True when the reply settles the decision for each side of a condition.\n\n    \"Fix it if it is a part and a few pounds; bin it if the repair is half the price of new\" is\n    the answer to \"is it worth fixing\": the person's own case falls one way or the other, and the\n    reply has said what to do in both. Read over the whole reply, because the second side is often\n    the second sentence (\"Rule of thumb: fix it while the part is a few pounds and you can fit it\n    yourself. Replace it once the repair costs more than about half of a new one\").\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_RULE_CONDITION(), "search", &[reply.clone()]))) } }) {
    return Py::Bool(false);
    }
    drop = { let __b = mcall(&g_RULE_DROP_IT(), "search", &[reply.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_RULE_GO_NEW(), "search", &[reply.clone()]) } };
    return call_builtin("bool", &[{ let __b = mcall(&g_RULE_KEEP_IT(), "search", &[reply.clone()]); if !py_truthy(&__b) { __b } else { drop.clone() } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_sign_off(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the opening closes the chat instead of answering it.\n\n    Only the part after the sign-off phrase is read for a verdict, so \"No worries, it's fine.\" and\n    \"Of course you can freeze it\" stay the yes they are, while \"No rush.\" and \"Fair enough.\" say\n    nothing about the thing.\n    ");
    match_ = mcall(&g_SIGN_OFF_LEAD(), "match", &[opening.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&opening.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" \t.,;:!?-—")]);
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:but|and|so|though|however|yet|although|still)\\b[\\s,]*")); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ let __b = _a_yesno(&[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_VERDICT_INSIDE(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_VERDICT_ECHO(), "match", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_VERDICT_RULE(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TEST_IS_THE_VERDICT(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TRAILING_VERDICT(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PERMISSION_RULE(), "search", &[rest.clone()]) } } } } } } } } } } } }));
    Py::None
}
pub fn _a_yesno(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut first_clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut tail: Py = Py::None;
    if py_truthy(&is_a_punt(&[opening.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_FIRST_PERSON_VERDICT(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_VERDICT_OPEN(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ECHO_VERDICT(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_VERDICT_AFTER_A_CASE(), "match", &[opening.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_PERMISSION_RULE(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_job_is_split(&[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_VERDICT_ECHO(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_VERDICT_RULE(), "search", &[opening.clone()]) } }) {
    return Py::Bool(true);
    }
    first_clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;:]")); __args.push(opening.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    if py_truthy(&{ let __b = _the_verdict_is_inside(&[first_clause.clone(), line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TEST_IS_THE_VERDICT(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TRAILING_VERDICT(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_THE_WINNER_NAMED(), "search", &[first_clause.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HARM_IS_WHAT_IT_IS(), "search", &[first_clause.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_VERDICT_ON_A_SCALE(), "search", &[first_clause.clone()]) } } } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_quality_answers(&[first_clause.clone(), line.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_rule_of_its_use(&[opening.clone(), line.clone()])) {
    return Py::Bool(true);
    }
    tail = py_str("");
    if py_truthy(&Py::Bool(py_contains(&opening.clone(), &py_str(":")))) {
    tail = geti(&mcall(&opening.clone(), "split", &[py_str(":"), Py::Int(1)]), &Py::Int(1));
    } else {
    found = stdlib_call("re", "search", &[py_str(",?\\s+so\\b"), opening.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    tail = slice_or_raise(&opening.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    }
    }
    tail = if py_truthy(&tail.clone()) { geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;]")); __args.push(tail.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)) } else { py_str("") };
    return call_builtin("bool", &[{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { { let __b = _the_tail_says_what_it_is_about(&[tail.clone(), line.clone()]); if py_truthy(&__b) { __b } else { _the_quality_answers(&[tail.clone(), line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_rule_of_its_use(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True when the reply answers \"is it ok/safe\" by giving the rule of use itself.\n\n    A bound that is the answer on its own - a keeping time, a \"use it within\", a \"no more than\" -\n    settles the question without naming the thing, because the cards name the class (\"a cooked\n    dish\") and the person named one of it (\"leftover curry\").\n\n    The rest of the bounds have to be on the thing the line asked about, so the reply names a word\n    the line named: \"keep the cat out of the room\" answers the spray line, and \"keep the receipt in\n    the tin\" as the answer to a washing machine line names nothing of it and is not this.\n    ");
    if py_truthy(&mcall(&g_A_BOUND_THAT_IS_THE_ANSWER(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    found = mcall(&g_A_LIMIT_ON_ITS_USE(), "search", &[opening.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    said = _line_text(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = _names(&[mcall(&found.clone(), "group", &[Py::Int(0)]), said.clone()]); if py_truthy(&__b) { __b } else { _names(&[opening.clone(), said.clone()]) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = _long_enough_a_name(&[w.clone(), Py::Int(4)]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__EMPTY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _the_thing_is_named_as_what_it_usually_is(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut found: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the reply says what the line's own thing usually is: \"is that conjunctivitis\".\n\n    \"A red, watery, gunky eye is usually conjunctivitis or an allergy\", \"Banging or rumbling when\n    the heating starts is usually air or a pump set too fast\" answer the line by naming the thing\n    it asked about as what the case usually is. The subject is described rather than named, so the\n    verdict-word tests find nothing in it, and the reply is a real answer rather than a method.\n\n    The line's own word has to appear after the \"is usually\", or the sentence is about something\n    else: \"A cat is usually asleep by then\" is not an answer to \"is that a good sign\".\n    ");
    said = _line_text(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    found = mcall(&g_USUALLY_IS(), "search", &[opening.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    rest = slice_or_raise(&opening.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z][a-z']*"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__EMPTY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "search", &[modulo(&py_str("\\b%s"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _an_echoed_no(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the first sentence denies the line's own words: \"Vinegar does not go off\".\n\n    The reply has to do both things - carry a denial, and name something the line named - or it\n    is not an answer to this line. \"It isn't cooked until the skewer comes out clean\" denies\n    nothing of \"are my scones done\", so the test-reading is what settles that one.\n    ");
    said = _line_text(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NEGATED_AUX(), "search", &[opening.clone()]))) } }) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z][a-z']*"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = ge(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__EMPTY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&opening.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_job_answers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let _ = py_str("True when the reply does the thing the person asked whether to do.\n\n    A yes/no about doing something is often answered by doing it: \"Slice it first, then bag it\"\n    is the yes to \"can i freeze bread\", \"Strap them in every time\" the yes to \"do i need to\n    strap my toddler in\". This is a yes and never a miss: the reply is only counted when it\n    opens on a job and the line is asking whether, so a reply this cannot read stays the\n    \"words cannot tell\" it was before.\n\n    Not when the line asks what happens if it does something itself: then the outcome is the\n    answer, and \"Wipe it off with a mould spray\" is no answer to \"will it come back if i just\n    wipe it\". That one is a miss the clear-shape test below makes, on the method it names.\n\n    The price of reading any verb and not a list of them: an instruction about something else\n    passes too. \"is my butter out of date\" / \"Rub washing-up liquid into the grease mark\", and\n    \"did you see the match last night\" / \"Dip your headlights as soon as another car is in sight\"\n    are misses this calls a fit (rugged-tuna's find in review). Both narrowings I measured cost\n    more good replies than they save: requiring the reply to name a word from the line turns 7\n    fits into faults (\"should he dye it\" / \"Colour the new growth every four to six weeks\"), and\n    requiring `PERMISSION` in the line turns 5 (\"is it rude to leave a group chat\" / \"Mute it\n    first\", \"am i getting enough water each day\" / \"Fill a bottle in the morning\"). At 612 shaped\n    turns the wide reading takes misses from 150 to 134 and the narrow ones to 137 and 139, so it\n    is kept, and the error left is the one that hides a miss rather than the one that invents one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { _an_outcome_of_their_own_doing(&[line.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_JOB(), "match", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_imperative(&[opening.clone()])) {
    return Py::Bool(true);
    }
    return _the_job_named(&[opening.clone(), _line_text(&[line.clone()])]);
    Py::None
}
pub fn _undecorated(args: &[Py]) -> Py {
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
    let _ = py_str("A head without the asterisks or quotes that set it off: \"*Note*\" and \"Note\" are one label.");
    return mcall(&mcall(&mcall(&head.clone(), "strip", &[]), "strip", &[py_str("*_\"“”'")]), "strip", &[]);
    Py::None
}
pub fn _listed_after(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut label_lead: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "label_lead") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: label_lead"))) };
    let _ = py_str("True when the words after a lead's colon are a list of things to pick from.");
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bor\\b"), label_lead.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&label_lead.clone(), &py_str(";"))) } }) {
    return Py::Bool(true);
    }
    return ge(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str(",\\s+\\S"), label_lead.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _marked_things(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("How many things the opening marks out as things: a title in asterisks or in quotes. The\n    reply is pointing at named things rather than describing a method.");
    return add(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("\\*[^*\\n]{2,60}\\*"), opening.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[stdlib_call("re", "findall", &[py_str("\\u201c[^\\u201d\\n]{2,60}\\u201d|\\B\\\"[^\\\"\\n]{2,60}\\\""), opening.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _a_pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut head: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut named: Py = Py::None;
    let mut words: Py = Py::None;
    if py_truthy(&{ let __b = mcall(&g_PREFERS(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PICKS_BY_COMPARISON(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WOULD(), "search", &[opening.clone()]) } } } }) {
    return Py::Bool(true);
    }
    lead = mcall(&g_LABEL(), "match", &[opening.clone()]);
    if py_truthy(&{ let __b = lead.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_A_LABEL(), "search", &[mcall(&lead.clone(), "group", &[Py::Int(1)])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_SET_LABEL(), "match", &[mcall(&mcall(&lead.clone(), "group", &[Py::Int(1)]), "strip", &[])]))); if !py_truthy(&__b) { __b } else { _listed_after(&[mcall(&lead.clone(), "group", &[Py::Int(2)])]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:is|are|was|were) (?:almost |nearly |usually |often |always |generally |probably |still |quite |about )?(?:the )?(?:(?!(?:request|interest|honest|guest|test|rest|nest|west|east|forest|harvest|protest|suggest|arrest|contest|digest|invest|earnest|chest|crest|tempest|midwest|northwest|southwest|northeast|southeast)\\b)[a-z]{3,}est|most [a-z]+|least [a-z]+)\\b|\\bis the (?:cheapest|dearest|worst|easiest|quickest|safest|hardest|cheaper|better)\\b|\\bwins?\\b(?: on| over| for| by)\\b|\\b(?:has|have|holds|gets) (?:the )?(?:most|highest|lowest|fewest|best)\\b"), opening.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_NAMES_IN_EMPHASIS(), "match", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[mcall(&g_QUOTED_NAME(), "findall", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { mcall(&g_PICK_BY_CASE(), "search", &[opening.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = is_a_punt(&[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_VERDICT_OPEN(), "match", &[opening.clone()]) } }) {
    return Py::Bool(false);
    }
    head = mcall(&g_NAMED_HEAD(), "match", &[opening.clone()]);
    if py_truthy(&head.clone()) {
    named = _undecorated(&[mcall(&mcall(&head.clone(), "group", &[Py::Int(1)]), "strip", &[])]);
    if py_truthy(&{ let __b = mcall(&g_ECHO_HEAD(), "match", &[named.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BACK_HEAD(), "match", &[named.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STATES_HEAD(), "search", &[named.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_FRAME_HEAD(), "match", &[named.clone()]); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\d"), named.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = _a_time(&[named.clone()]); if py_truthy(&__b) { __b } else { _an_amount(&[named.clone()]) } } } } } } } } } } } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_INSTRUCTS(), "match", &[_undecorated(&[mcall(&head.clone(), "group", &[Py::Int(1)])])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_ASKS_HEAD(), "match", &[_undecorated(&[mcall(&head.clone(), "group", &[Py::Int(1)])])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SET_LABEL(), "match", &[mcall(&_undecorated(&[mcall(&head.clone(), "group", &[Py::Int(1)])]), "strip", &[])]))) } } } } } }) {
    return Py::Bool(true);
    }
    words = mcall(&mcall(&opening.clone(), "rstrip", &[py_str(".!")]), "split", &[]);
    return { let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_BALD(), "match", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_BALD_VERB(), "search", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_INSTRUCTS(), "match", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_REASON_OPENS(), "match", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_VERB_THEN_THE_THING(), "match", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_clear_time(&[opening.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_figure_that_counts(&[opening.clone()]))) } } } } } } } } } } } } } };
    Py::None
}
pub fn _asks_the_person(args: &[Py]) -> Py {
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
    let _ = py_str("True when this text asks the person something rather than telling them a step.\n\n    The nod words are read at its head. A question mark counts only in the sentence the text opens\n    on, because a method often ends on a question about the person and that does not take the\n    method back: \"Pick one thing you can finish inside it. Are you on your own?\" told them how, and\n    \"Give it ten minutes. How does it look then?\" is a step with a check after it.\n\n    A bare `?` used to be the first alternative of `WANTS_INFO`, and that pattern is only ever\n    read with `re.match`, which looks at the start of the text and nowhere else: the alternative\n    could only fire on a text beginning with \"?\", so every reply that ended \"...how long is the\n    run?\" was read as an instruction because the \"?\" was never looked for. Searching the whole\n    text fixes that and goes too far the other way - \"Who's in charge?\" then cancelled the method\n    in front of it - so the mark is read inside the sentence the clause opens on.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_WANTS_INFO(), "match", &[said.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&geti(&stdlib_call("re", "split", &[py_str("(?<=[.?!])\\s"), said.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), &py_str("?")));
    Py::None
}
pub fn _an_instruction(args: &[Py]) -> Py {
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
    let _ = py_str("True when the sentence tells the person to do something that is not fetching things.");
    return { let __b = call_builtin("bool", &[_imperative(&[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("bool", &[mcall(&g_THINGS_VERB(), "match", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn _imperative(args: &[Py]) -> Py {
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
    let mut m: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when `sentence` is shaped like an instruction: a verb and the thing it acts on.");
    m = mcall(&g_IMPERATIVE(), "match", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&m.clone()))) {
    return Py::Bool(false);
    }
    word = mcall(&mcall(&mcall(&m.clone(), "group", &[Py::Int(1)]), "lower", &[]), "strip", &[py_str("'")]);
    return { let __b = Py::Bool(!py_contains(&g_NOT_A_VERB(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")]))) } };
    Py::None
}
pub fn _opens_on_a_job(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut m: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when `sentence` is an instruction and not a statement that opens on a verb.\n\n    \"Look, it happens\" is an aside however the verb list reads it, so an opening that is only the\n    verb and then punctuation is not one, and a finite verb within the first few words is the\n    sentence's own subject rather than something to act on.\n    ");
    m = mcall(&g_JOB(), "match", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&m.clone()))) {
    return Py::Bool(false);
    }
    first = geti(&stdlib_call("re", "split", &[py_str("[,;:]"), slice_or_raise(&{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &mcall(&m.clone(), "end", &[]), &Py::None, &Py::None), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    words = slice_or_raise(&stdlib_call("re", "findall", &[py_str("[A-Za-z']+"), first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(3), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SAYS_WHAT_IT_DOES(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
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
    let mut end: Py = Py::None;
    let _ = py_str("The word without the ending that would keep it from matching its other form.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    end = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[end.clone()]); if !py_truthy(&__b) { __b } else { ge(&sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[end.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(4)) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[end.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words in `text` that name a thing, as stems, with a title left out.\n\n    \"Frozen\" in \"Idina Menzel is the voice of Elsa in Frozen\" is the film, not the frozen food\n    the line asked about, so a capital in the middle of a reply is a name rather than a subject.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("[A-Za-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    word = mcall(&match_.clone(), "group", &[Py::Int(0)]);
    low = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str("'")]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__NOT_THE_SUBJECT(), &low.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "start", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str(".!?"), &geti(&said.clone(), &sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(1))))) } } } }) {
    continue;
    }
    let _ = mcall(&words.clone(), "add", &[_stem(&[low.clone()])]);
    }
    return words.clone();
    Py::None
}
pub fn _the_same_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let _ = py_str("True when the reply speaks about the thing the line named.\n\n    Not a claim that the reply answers the line: it is the reason the shape of the reply cannot\n    be read against the shape the line asked for, because a yes or a no in words is a statement\n    about the thing, and the statement is the one shape that has no cue list.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[bitand(&_subject_words(&[line.clone()]), &_subject_words(&[opening.clone()]))]).unwrap_or_else(|e| exc::raise(e));
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
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { py_str("") };
    let mut part: Py = Py::None;
    if py_truthy(&{ let __b = mcall(&g_DOES_NOT(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DOES(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MEANS_OF(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MEANS_NAMED(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = _opens_on_a_job(&[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_CALCULATION(), "match", &[opening.clone()]) } } } } } } } } } }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&stdlib_call("re", "split", &[py_str("[:;]"), opening.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    part = __item.clone();
    part = mcall(&part.clone(), "strip", &[]);
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_INSTRUCTS(), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DOES(), "search", &[part.clone()]); if py_truthy(&__b) { __b } else { _imperative(&[part.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_asks_the_person(&[after.clone()]))) } }) {
    if py_truthy(&{ let __b = mcall(&g_INSTRUCTS(), "match", &[after.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DOES(), "search", &[after.clone()]); if py_truthy(&__b) { __b } else { _imperative(&[after.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _instruction_after_a_clause(args: &[Py]) -> Py {
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
    let mut m: Py = Py::None;
    let _ = py_str("True when some clause of `text` after the first starts on a method's verb.\n\n    A clause that asks the person for the information is not one of them: \"Tell me what's going\n    on\", \"Say which one you are on\" is the person's next move, not a step of the answer.\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g_INSTRUCTS_CLAUSE(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    m = __item.clone();
    if py_truthy(&_asks_the_person(&[slice_or_raise(&text.clone(), &mcall(&m.clone(), "start", &[py_str("verb")]), &Py::None, &Py::None)])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&m.clone(), "start", &[]), &Py::Int(0))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_on_a_job(&[text.clone()]))) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _steps_in(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the reply gives a method, read over the whole reply and not only its opening.\n\n    Every other shape is the first sentence, because STYLE.md asks for the answer there and a\n    figure three paragraphs down is not the reply the person got. A method is the exception: the\n    answer to a how question *is* the sequence, so \"A dripping tap is nearly always a worn washer:\n    turn the water off at the isolating valve\" and \"Most veg keeps about a week: keep it in the\n    fridge drawer\" are a method told over two sentences, and the instruction often starts a later\n    clause (\"Lower eggs into already boiling water: 6 minutes for a runny yolk\"). Reading only the\n    opening called 114 of 294 good step replies a miss on the deck. A list of dishes with their\n    times (\"Quick: tuna pasta in 15, a frittata in 20\") is a few ideas, not a method, and is left\n    to `_options`.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_steps(&[first_sentence(&[text.clone()]), second_sentence(&[text.clone()])])) {
    return Py::Bool(true);
    }
    return _instruction_after_a_clause(&[text.clone()]);
    Py::None
}
pub fn _a_comparison(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("One side of a pair set against the other, in the reply's own words.");
    return call_builtin("bool", &[mcall(&g_CONTRAST(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_list_of_things(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut after: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("True when a colon in the sentence opens a list of two or more things.");
    found = mcall(&g__LIST_OF_THINGS(), "search", &[{ let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    after = geti(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[py_str(":"), Py::Int(1)]), &Py::Int(1));
    return { let __b = ge(&mcall(&after.clone(), "count", &[py_str(",")]), &Py::Int(2)); if py_truthy(&__b) { __b } else { { let __b = ge(&mcall(&after.clone(), "count", &[py_str(",")]), &Py::Int(1)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\band\\b"), after.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _options(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:1\\.|1\\)|first option|option one|one idea)\\b"), opening.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bor\\b"), opening.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&opening.clone(), &py_str(";")))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_NEXT_IDEA(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_FOR_A_LIST(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_FOR_A_TIME(), "search", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SEQUENCES(), "search", &[opening.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_LIST_AFTER_COLON(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SEQUENCES(), "search", &[opening.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_LIST_AFTER_COLON(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { _three_things_after_the_colon(&[opening.clone()]) } }) {
    return Py::Bool(true);
    }
    return _two_things_to_do(&[opening.clone()]);
    Py::None
}
pub fn _two_things_to_do(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut part: Py = Py::None;
    head = mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\.\\s")); __args.push(opening.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)), "rstrip", &[py_str(".")]);
    if py_truthy(&mcall(&g_SEQUENCES(), "search", &[head.clone()])) {
    return Py::Bool(false);
    }
    found = Py::Int(0);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",?\\s+and\\s+"), head.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    part = __item.clone();
    part = mcall(&part.clone(), "strip", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mcall(&part.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_INSTRUCTS(), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLAN_VERB(), "match", &[part.clone()]) } })) } }) {
    break;
    }
    let __aug2 = add(&found.clone(), &Py::Int(1));
    found = __aug2;
    }
    return ge(&found.clone(), &Py::Int(2));
    Py::None
}
pub fn _three_things_after_the_colon(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut after: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("True when what follows the first colon is three or more things, not one plan's steps.\n\n    The \"first\" that a wake-up-time list opens with is the reply picking out the one thing that\n    matters, and it is before the colon, so a sequence word after the colon is the only one that\n    makes what follows a plan: \"One evening: a film, a walk and a call, then tea\" is one evening.\n    ");
    parts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); __args.push(opening.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    after = mcall(&geti(&parts.clone(), &Py::Int(1)), "strip", &[]);
    if py_truthy(&{ let __b = mcall(&g_INSTRUCTS(), "match", &[after.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SEQUENCES(), "search", &[geti(&parts.clone(), &Py::Int(1))]) } }) {
    return Py::Bool(false);
    }
    return ge(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str(",\\s+\\S"), geti(&parts.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _a_reason(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    if py_truthy(&mcall(&g_CAUSE(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_PURPOSE(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PURPOSE_TO(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MERIT(), "search", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_UNSETTLED(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_EVIDENCE_SAYS(), "search", &[opening.clone()]) } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_INSTRUCTS(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DOES_NOT(), "match", &[opening.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_SENDS_THEM_ON(), "search", &[opening.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_CAUSE_WHEN(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&line.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_WANTS_THE_CAUSE(), "search", &[opening.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_WANTS_OF_THE_PERSON(), "match", &[opening.clone()]))); if !py_truthy(&__b) { __b } else { _names(&[opening.clone(), line.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_A_COUNT_OF_THINGS(), "search", &[opening.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_CAUSE_IS(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&line.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _names_the_subject(&[opening.clone(), line.clone()]) } } } };
    Py::None
}
pub fn _head_explains(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut head: Py = Py::None;
    let _ = py_str("True when what comes before the colon states the cause, so a method after it is the fix.");
    head = mcall(&opening.clone(), "split", &[py_str(":"), Py::Int(1)]);
    if py_truthy(&lt(&call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_SENDS_THEM_ON(), "search", &[geti(&head.clone(), &Py::Int(0))])) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_EXPLAINS(), "search", &[geti(&head.clone(), &Py::Int(0))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_imperative(&[geti(&head.clone(), &Py::Int(0))]))) } };
    Py::None
}
pub fn _a_meaning_or_a_statement(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("`_a_meaning`, and a meaning said in a verb no list has: see `_a_statement_answers_it`.");
    if py_truthy(&mcall(&g_SAYS_NOTHING_ABOUT_IT(), "match", &[{ let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_meaning(&[opening.clone()])) {
    return Py::Bool(true);
    }
    return _a_statement_answers_it(&[opening.clone()]);
    Py::None
}
pub fn _a_statement_answers_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut words: Py = Py::None;
    let _ = py_str("True when a meaning ask is answered by a plain statement about something.\n\n    The three other things a first sentence can be are taken out: a job for the person (\"Start with\n    the router your provider gave you\", \"Learn to change a tyre\"), a question put back (\"?\"), and a\n    shrug (\"Nothing from me so far\", \"That's beyond what I know\"). What is left is a sentence saying\n    something, and a sentence too short to say anything (\"Fine.\", \"Yes, about right\") is left alone.\n\n    A figure on its own is how much, not what a thing is: \"how does it work\" / \"It takes about 50\n    to 100ml per kilo a day\" is a dose and it is the whole sentence. What joins a thing to a figure\n    and makes a statement about the thing is the copula - \"A normal resting heart rate for an adult\n    is 60 to 100 beats a minute\" - or the rest of a sentence around the figure: \"The Inca empire ran\n    along the Andes from about 1438 to 1533\". So a short sentence whose only content is a figure\n    needs the copula to count.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&opening.clone())); if py_truthy(&__b) { __b } else { mcall(&mcall(&opening.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = is_a_punt(&[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOTHING_TO_GO_ON(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SAYS_NOTHING_ABOUT_IT(), "match", &[opening.clone()]); if py_truthy(&__b) { __b } else { _an_opening_job(&[opening.clone()]) } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_an_origin(&[opening.clone()])) {
    return Py::Bool(false);
    }
    words = stdlib_call("re", "findall", &[py_str("[A-Za-z0-9'\\u2019-]+"), opening.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)); if !py_truthy(&__b) { __b } else { { let __b = _a_figure(&[opening.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_COPULA(), "search", &[opening.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    return ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5));
    Py::None
}
pub fn _an_opening_job(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the sentence opens on a job for the person to do.");
    if py_truthy(&_an_instruction(&[opening.clone()])) {
    return Py::Bool(true);
    }
    rest = mcall(&g_AN_ORDINAL_OPENING(), "sub", &[py_str(""), { let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&rest.clone(), &{ let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } })); if !py_truthy(&__b) { __b } else { _an_instruction(&[rest.clone()]) } }) {
    return Py::Bool(true);
    }
    match_ = mcall(&g_A_JOB_LANDS(), "match", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    word = mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]), "strip", &[py_str("'")]);
    return { let __b = Py::Bool(!py_contains(&g_NOT_A_VERB(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")]))) } };
    Py::None
}
pub fn fits(args: &[Py]) -> Py {
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
    let mut want: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let _ = py_str("`gives` under the name the lane plan used, for callers that read it that way.");
    return gives(&[reply.clone(), want.clone(), line.clone()]);
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut field: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The line as plain text: lower case, one space, curly apostrophes straightened.\n\n    Takes the words or the `understood` dict, so a caller may pass either. A dict read with\n    `str()` is every field name and its value, and the words of that read - \"short\", \"lines\",\n    \"false\" - are what a `_names_what_the_line_named` gate would match: the line's own words are\n    all a reply should be measured against.\n\n    The field is the line as the person typed it, in the plain reading: the field `plain` keeps\n    the figures and drops the punctuation, and `norm_text` does the opposite, turning \"i got a 5\n    in my maths mock\" into \"a five ... | ...\", so a reply that says \"A 5 is a strong pass\" shares\n    no word with its own line and the \"names what the line named\" gate reads the answer as a\n    miss. The words a reply is measured against are the words on the screen.\n    ");
    if py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&g_LINE_TEXT_FIELDS());
    let mut __else2 = false;
    for __item in __iter1 {
    field = __item.clone();
    value = mcall(&line.clone(), "get", &[field.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    line = value.clone();
    break;
    }
    }
    line = py_str("");
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
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
    return stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_asked(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing a definition ask is about, as the words the reply would open on: \"student loan\"\n    for \"what is a student loan and do i pay it back\".");
    match_ = mcall(&g_THE_THING_ASKED(), "match", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__ENDS_THE_THING(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__EMPTY(), &word.clone())) } }) {
    break;
    }
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)))) {
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__NOT_A_THING(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return py_str("");
    }
    if py_truthy(&lt(&call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _uses_the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut at: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when the reply opens on the verb for using the thing asked about and gives its job.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__PUTS_THE_THING_TO_USE(), "match", &[opening.clone()]))) } }) {
    return Py::Bool(false);
    }
    low = mcall(&opening.clone(), "lower", &[]);
    at = mcall(&low.clone(), "find", &[thing.clone()]);
    if py_truthy(&lt(&at.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:to|for|so)\\b"), slice_or_raise(&low.clone(), &at.clone(), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _defines_the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut head: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("True when the reply opens on the thing the line asked about and then says what it is or\n    does: \"what is a student loan\" / \"A student loan pays the tuition fee and the maintenance\n    money while you study\". The verb is the reply's own - `MEANS_BY_DOING` knows `give` and\n    `provide`, and this reads the rest - so what matters is that the subject comes first and\n    something is said about it, not the words it is said with.");
    thing = _the_thing_asked(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_uses_the_thing(&[opening.clone(), thing.clone()])) {
    return Py::Bool(true);
    }
    head = mcall(&mcall(&g__ARTICLE(), "sub", &[py_str(""), mcall(&opening.clone(), "strip", &[])]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&head.clone(), "startswith", &[add(&thing.clone(), &py_str(" "))]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&head.clone(), "startswith", &[add(&thing.clone(), &py_str("'"))]))) } }) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&head.clone(), &call_builtin("len", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = mcall(&g__JUDGES_THE_THING(), "match", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__ADVISES_THE_THING(), "match", &[rest.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&thing.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__SAYS_WHAT_IT_IS(), "match", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_DOING(), "match", &[rest.clone()]) } })) } }) {
    return Py::Bool(false);
    }
    return ge(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3));
    Py::None
}
pub fn _name_words(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a line or a reply as names, digits and all: `_words` drops \"15\", \"4g\" and \"b12\"\n    with every other figure, and a figure can be one of the two things a line offers.");
    return stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _long_enough_a_name(args: &[Py]) -> Py {
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
    let mut least: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "least") { v.clone() }
    else { Py::Int(3) };
    let mut ch: Py = Py::None;
    let _ = py_str("A token with a digit is a name however short: \"4g\", \"5g\", \"15\", \"dmp\" and \"b12\" all name\n    something, where a short word of letters alone is more likely \"it\", \"or\" or \"my\".");
    return { let __b = ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &least.clone()); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&word.clone()) { let mut ch: Py = __item.clone(); __out.push(mcall(&ch.clone(), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut reply: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the reply's opening names a thing the line named, by stem: bread / breads,\n    freeze / freezing. A word from the line that is not in `_EMPTY` and is long enough to mean\n    something.");
    reply = call_builtin("set", &[_words(&[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_long_enough_a_name(&[word.clone(), Py::Int(4)]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__EMPTY(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&reply.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&reply.clone(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&reply.clone(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_the_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut reply: Py = Py::None;
    let mut said_word: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the reply's cause-is-the-thing sentence is about something the line named.\n\n    The same job as `_names` beside it, read a little wider, because a why-answer names its\n    subject in the reply's own form of the word: \"why is my cabbage bitter\" is answered \"The\n    *bitterness* is the sulphur compounds in the leaves\", and \"why should i neuter my cat\" by \"a\n    spayed *cat* is out of the annual litters\". `_names` wants a word of four letters or more and\n    matches it by stem, which drops \"bitter\" against \"bitterness\" and every three-letter subject\n    - cat, dog, car, gas, job - and the two most ordinary why-answers on the deck read as\n    sentences about nothing.\n\n    A three-letter word has to be the whole word in the reply, not the start of one, so \"car\"\n    is not read into \"care\" and \"gas\" not into \"gasket\".\n    ");
    reply = call_builtin("set", &[_words(&[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_long_enough_a_name(&[word.clone(), Py::Int(3)]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__EMPTY(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)))) {
    if py_truthy(&Py::Bool(py_contains(&reply.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&reply.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&reply.clone(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&reply.clone(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&reply.clone());
    for __item in __iter2 {
    said_word = __item.clone();
    if py_truthy(&mcall(&said_word.clone(), "startswith", &[word.clone()])) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _opens_on_the_lines_verb(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut first: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for a short opening that is the verb a decision in the line named, and nothing else:\n    \"should i bin it\" / \"Bin it.\" or \"does that mean i should bin it\" / \"Bin it, I'm afraid.\".");
    words = mcall(&opening.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return Py::Bool(false);
    }
    first = _words(&[opening.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[geti(&first.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::Bool(false);
    }
    stem = geti(&first.clone(), &Py::Int(0));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("\\b(?:should|must|need to|have to)\\b\\s+(?:i\\s+)?([a-z]{3,})"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    verb = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&verb.clone(), &stem.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&verb.clone(), "startswith", &[slice_or_raise(&stem.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _trimmed(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The words of a side without the filler at either end.");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::Int(0), call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    start = __unpk1[0].clone();
    end = __unpk1[1].clone();
    while py_truthy(&{ let __b = lt(&start.clone(), &end.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SIDE_FILLER(), &geti(&words.clone(), &start.clone()))) } }) {
    let __aug2 = add(&start.clone(), &Py::Int(1));
    start = __aug2;
    }
    while py_truthy(&{ let __b = gt(&end.clone(), &start.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SIDE_FILLER(), &geti(&words.clone(), &sub(&end.clone(), &Py::Int(1))))) } }) {
    let __aug3 = sub(&end.clone(), &Py::Int(1));
    end = __aug3;
    }
    return slice_or_raise(&words.clone(), &start.clone(), &end.clone(), &Py::None);
    Py::None
}
pub fn _the_pair_up(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let mut right: Py = Py::None;
    let mut spent: Py = Py::None;
    let _ = py_str("The two things a line puts up to weigh against each other, or None.\n\n    Three ways people ask it: \"is wholemeal better than white\" and \"does a deep bath use more\n    water than a shower\" (a comparative and \"than\"), \"dried beans or tinned, which is the better\n    value\" and \"which is better, the train or the coach\" (an \"or\"), \"dried beans vs tinned\".\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    found = mcall(&g_COMPARATIVE_THAN(), "search", &[clean.clone()]);
    if py_truthy(&found.clone()) {
    left = _trimmed(&[_words(&[mcall(&found.clone(), "group", &[Py::Int(1)])])]);
    right = _trimmed(&[_words(&[mcall(&found.clone(), "group", &[Py::Int(3)])])]);
    spent = call_builtin("set", &[_words(&[mcall(&found.clone(), "group", &[Py::Int(2)])])]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = left.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&spent.clone(), &geti(&left.clone(), &neg(&Py::Int(1))))) } }) {
    left = slice_or_raise(&left.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&left.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&right.clone())) } }) {
    return Py::None;
    }
    return py_tuple(vec![slice_or_raise(&left.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None), slice_or_raise(&right.clone(), &Py::None, &Py::Int(2), &Py::None)]);
    }
    found = { let __b = mcall(&g_SIDES(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SIDES_VS(), "search", &[clean.clone()]) } };
    if py_truthy(&found.clone()) {
    return py_tuple(vec![_words(&[mcall(&found.clone(), "group", &[Py::Int(1)])]), _words(&[mcall(&found.clone(), "group", &[Py::Int(2)])])]);
    }
    return Py::None;
    Py::None
}
pub fn _a_side_said_in_the_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut side: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "side") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: side"))) };
    let mut ordered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ordered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ordered"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the reply says one of the side's own words, in any of the forms people say it in.\n\n    A different question from the one below, which asks about one word of a side: this asks about\n    a whole side, and it answers it through that one-word matcher rather than a reading of its own,\n    so \"Showering\" is the shower here as it is in the comparison arms, \"cats\" is the cat in \"a cat\"\n    and \"hay fever\" is the hayfever the line named. The floor is three letters, because a\n    three-letter name is a name - \"is a cat better than a dog\" has to be read on \"cat\" - and every\n    word in `_EMPTY` is still skipped.\n    ");
    let __iter1 = py_iter_or_raise(&side.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_long_enough_a_name(&[word.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__EMPTY(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&_a_side_word_is_named(&[word.clone(), ordered.clone(), words.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_side_wins(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut named: Py = Py::None;
    let mut ordered: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut side: Py = Py::None;
    let mut sides: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line weighed two things up and the reply opened on one of them.\n\n    \"is a bath better than a shower\" / \"Showering wins on time and water\" is a yes with a reason,\n    and so is \"is a plank better than a sit up\" / \"Both, and they do different jobs\". The reply\n    has to name one of the two sides and say one of the things a choice says, so a reply that\n    describes the pair without choosing stays the \"words cannot tell\" it was.\n    ");
    pair = _the_pair_up(&[_line_text(&[line.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&pair.clone()))) {
    return Py::Bool(false);
    }
    sides = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pair.clone()) { let mut side: Py = __item.clone(); if py_truthy(&side.clone()) {  __out.push(side.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&sides.clone()))) {
    return Py::Bool(false);
    }
    ordered = _words(&[opening.clone()]);
    words = call_builtin("set", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e));
    named = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sides.clone()) { let mut side: Py = __item.clone(); __out.push(_a_side_said_in_the_reply(&[side.clone(), ordered.clone(), words.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = named.clone(); if py_truthy(&__b) { __b } else { stdlib_call("re", "match", &[py_str("^both\\b"), opening.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_SIDE_WINS(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_a_side(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut reply: Py = Py::None;
    let mut side: Py = Py::None;
    let mut side_words: Py = Py::None;
    let mut sides: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A pick of one of two named sides, without the words a pick normally uses: the line says\n    \"a fiesta or a focus\" and the reply opens on the fiesta. Only the words either side of the\n    \"or\" are the sides, so the rest of the line is not mistaken for one.");
    sides = mcall(&g_SIDES(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&sides.clone()))) {
    return Py::Bool(false);
    }
    reply = slice_or_raise(&_name_words(&[opening.clone()]), &Py::None, &Py::Int(3), &Py::None);
    side_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&sides.clone(), "groups", &[])) { let mut side: Py = __item.clone(); if py_truthy(&side.clone()) {  for __item in py_iter_or_raise(&_name_words(&[side.clone()])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } } py_list(__out) })() };
    return _a_side_said_in_the_reply(&[side_words.clone(), reply.clone(), call_builtin("set", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _a_side_word_is_named(args: &[Py]) -> Py {
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
    let mut ordered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ordered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ordered"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut index: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut reply_word: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("True when this one word of a side is said in the reply.\n\n    The same thing is said in the line and in the reply in different shapes, so the match allows\n    for the plural (seat/seats), for a verb the reply gives as its stem (\"simmering\" in the line,\n    \"a simmer\" in the reply, and the other way about), and for a compound the reply spaces out\n    (\"hayfever\" in the line, \"hay fever\" in the reply). A short name the reply never says, such as\n    \"dmp\", is matched by its initials instead.\n    ");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    reply_word = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&reply_word.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![reply_word.clone(), word.clone()])); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 2, None);
    shorter = __unpk2[0].clone();
    longer = __unpk2[1].clone();
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[longer.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&longer.clone(), "startswith", &[shorter.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("s")]), &slice_or_raise(&shorter.clone(), &neg(&Py::Int(1)), &Py::None, &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&shorter.clone(), "rstrip", &[py_str("s")]), &mcall(&longer.clone(), "rstrip", &[py_str("s")]))) } }) {
    return Py::Bool(true);
    }
    }
    let __iter3 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    index = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&add(&geti(&ordered.clone(), &index.clone()), &geti(&ordered.clone(), &add(&index.clone(), &Py::Int(1)))), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    return _initials_named(&[word.clone(), ordered.clone()]);
    Py::None
}
pub fn _initials_named(args: &[Py]) -> Py {
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
    let mut reply_words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply_words"))) };
    let mut pos: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("A short name spelled out by the reply: \"dmp\" as \"debt management plan\", \"iva\" as \"individual\n    voluntary arrangement\". An acronym is a name the reply may only ever say in full.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp10 = call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(4)) } else { Py::Bool(false) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } }) {
    return Py::Bool(false);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[reply_words.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    pos = Py::Int(0);
    let __iter3 = py_iter_or_raise(&slice_or_raise(&reply_words.clone(), &start.clone(), &add(&start.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None));
    for __item in __iter3 {
    spoken = __item.clone();
    if py_truthy(&{ let __b = lt(&pos.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&spoken.clone(), &Py::None, &Py::Int(1), &Py::None), &geti(&word.clone(), &pos.clone()))) } }) {
    let __aug4 = add(&pos.clone(), &Py::Int(1));
    pos = __aug4;
    }
    }
    if py_truthy(&Py::Bool(py_eq(&pos.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_both_sides(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut missing: Py = Py::None;
    let mut named: Py = Py::None;
    let mut ordered: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut pairs: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut side: Py = Py::None;
    let mut side_words: Py = Py::None;
    let mut sides: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when both of the two things the line put together are in the reply, False when one of\n    them is missing, None when the missing one is too short to look for.\n\n    The pair is read the way the line words it - \"the difference between X and Y\", \"X vs Y\",\n    \"compare X and Y\", \"X apart from Y\", \"X cheaper than Y\" - and a side counts when any of its own\n    words of three letters or more is in the reply, allowing for the plural (ssd/ssds,\n    seater/seats) or for a short name the reply spells out (dmp/debt management plan). The whole\n    reply is read rather than the opening, because a comparison is two-part by nature: \"An SSD has\n    no moving parts ... a hard drive is a spinning disc\" is both sides, one in each clause.\n\n    A reply that names one of the two and never the other has answered half the question, but a\n    three-letter name is not evidence of anything: \"hdd\" may be \"a hard drive\" in the reply, so\n    nothing is claimed against the reply on a short name alone.\n    ");
    said = mcall(&mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    sides = Py::None;
    let __iter1 = py_iter_or_raise(&g_SIDE_PAIRS());
    for __item in __iter1 {
    pattern = __item.clone();
    sides = mcall(&pattern.clone(), "search", &[said.clone()]);
    if py_truthy(&sides.clone()) {
    break;
    }
    }
    pairs = if py_truthy(&sides.clone()) { py_list(vec![slice_or_raise(&mcall(&sides.clone(), "groups", &[]), &Py::None, &Py::Int(2), &Py::None)]) } else { py_list(vec![]) };
    if py_truthy(&Py::Bool(!py_truthy(&pairs.clone()))) {
    pair = _the_pair_up(&[said.clone()]);
    if py_truthy(&{ let __b = pair.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[pair.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    pairs = py_list(vec![{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pair.clone()) { let mut side: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[side.clone()])); } py_list(__out) })() }]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&pairs.clone()))) {
    return Py::None;
    }
    ordered = _words(&[reply.clone()]);
    words = call_builtin("set", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __unpk2 = py_unpack(&py_tuple(vec![Py::Int(0), py_list(vec![])]), 2, None);
    named = __unpk2[0].clone();
    missing = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&geti(&pairs.clone(), &Py::Int(0)));
    for __item in __iter3 {
    side = __item.clone();
    side_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[side.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__SIDE_EMPTY(), &word.clone())) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&side_words.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&side_words.clone()) { let mut word: Py = __item.clone(); __out.push(_a_side_word_is_named(&[word.clone(), ordered.clone(), words.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug4 = add(&named.clone(), &Py::Int(1));
    named = __aug4;
    } else {
    let _ = mcall(&missing.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(side_words.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    }
    }
    if py_truthy(&lt(&add(&named.clone(), &call_builtin("len", &[missing.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(2))) {
    return Py::None;
    }
    if py_truthy(&ge(&named.clone(), &Py::Int(2))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&missing.clone()) { let mut word: Py = __item.clone(); __out.push(le(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_job_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut first: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The job the line asked leave for, in a reply whose first word is not one of the verbs the\n    cue list knows. Every part is needed: the line asks leave, the reply opens on a job, and the\n    reply names the thing the line named. The verb alone answers a question about something else,\n    and the name alone is a paragraph that happens to mention it.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PERMISSION(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names(&[opening.clone(), said.clone()])))) {
    return _opens_on_the_lines_verb(&[opening.clone(), said.clone()]);
    }
    if py_truthy(&_steps(&[opening.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_JOB_ANY(), "search", &[mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&opening.clone(), "split", &[]), &Py::None, &Py::Int(12), &Py::None)])])) {
    return Py::Bool(true);
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&opening.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9))) {
    return Py::Bool(false);
    }
    first = _words(&[opening.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[geti(&first.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::Bool(false);
    }
    stem = geti(&first.clone(), &Py::Int(0));
    let __iter1 = py_iter_or_raise(&_words(&[said.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&word.clone(), &stem.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "startswith", &[stem.clone()]); if py_truthy(&__b) { __b } else { mcall(&stem.clone(), "startswith", &[word.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_step_told_as_what_not_to_do(args: &[Py]) -> Py {
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
    let mut m: Py = Py::None;
    let _ = py_str("True when a clause of the reply tells a step in the negative form.\n\n    Read over the whole reply, like the other step shapes: \"Mushrooms keep about a week in the\n    fridge in a paper bag, never a plastic one ... Don't wash them until you cook them\".\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g_DOES_NOT_ANYWHERE(), "finditer", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    m = __item.clone();
    after = mcall(&mcall(&m.clone(), "group", &[py_str("after")]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_STEP_BUT_REASSURANCE(), "match", &[after.clone()])) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _steps_in(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("`_steps_in`, and the two methods a verb list cannot see.\n\n    \"Sow peas from March to early June, 5cm deep in a wide flat trench\" and \"Prune the roses in\n    late winter, just as the buds start to swell\" have no article after the verb, so `INSTRUCTS`\n    does not know them; \"Windows: Settings > Apps > Installed apps, find it in the list and choose\n    Uninstall\" opens on a path. All three are methods, and each was a clear miss on a real turn.\n    ");
    if py_truthy(&py_call(&g__STEPS_IN(), &[reply.clone()])) {
    return Py::Bool(true);
    }
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&_a_step_told_as_what_not_to_do(&[text.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = mcall(&g_GARDEN(), "match", &[sentence.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PATH_THEN_VERB(), "search", &[sentence.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return _words_handed_over(&[text.clone()]);
    Py::None
}
pub fn _words_handed_over(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply opens by handing the person the words to say, in quotes.");
    return call_builtin("bool", &[mcall(&g_SAID_WORDS(), "match", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_money_rate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("True when the reply says a price for a stretch of time, not a length of time.");
    return call_builtin("bool", &[mcall(&g_MONEY_RATE(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _without_a_money_rate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let _ = py_str("The reply with its price-per-time phrases taken out, so the time check can be asked again.");
    return mcall(&g_MONEY_RATE(), "sub", &[py_str(" "), opening.clone()]);
    Py::None
}
pub fn _the_figure_against_the_standard(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let _ = py_str("The verdict a yes/no about a quantity wants, when the reply sets the figure against it.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_QUANTITY_ASK(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__A_FIGURE_AGAINST_THE_STANDARD(), "match", &[{ let __b = opening.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_figure_then_the_verdict(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut second: Py = Py::None;
    let mut two: Py = Py::None;
    let _ = py_str("The verdict in the sentence after the figure it rests on: see `VERDICT_AFTER_A_FIGURE`.\n\n    The reply answers \"is that too far\" by doing the sum and then judging it - \"About 5 minutes in\n    the car and 20 minutes on a bike, each way. That is a short trip - easy to do every day\" - and\n    the figure is the evidence rather than the answer. Only the sentence straight after the\n    opening is read, and only when the opening gave a figure or a measure, so a reply that only\n    measures stays the miss it was.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&opening.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _a_time(&[opening.clone()]); if py_truthy(&__b) { __b } else { _an_amount_with_a_unit(&[opening.clone()]) } })) } }) {
    return Py::Bool(false);
    }
    two = _opening_two(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&two.clone(), "startswith", &[opening.clone()])))) {
    return Py::Bool(false);
    }
    second = mcall(&slice_or_raise(&two.clone(), &call_builtin("len", &[opening.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    return call_builtin("bool", &[{ let __b = second.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_VERDICT_AFTER_A_FIGURE(), "search", &[second.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_VERDICT_BAND(), "search", &[second.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _opens_on_a_job_to_do(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when the sentence is shaped like an instruction, whatever verb it uses.\n\n    `_opens_on_a_job` reads the verb against `JOB`, a list, so a method that opens on a verb\n    nobody thought of (\"clear\", \"unplug\", \"reinstall\") was read as a reply about something else.\n    The shape - a verb, then the thing it acts on - is what an instruction is, and `AN_INSTRUCTION`\n    reads it without a list. `WANTS_INFO` keeps out a reply that asks the person for the\n    information (\"Tell me what's going on\"), and a word that is no verb, or an -ing, is not one.\n    ");
    text = call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_AN_INSTRUCTION(), "match", &[text.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_WANTS_INFO(), "match", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_CONTRACTION_OPENER(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_WELL_WISHING(), "match", &[mcall(&text.clone(), "strip", &[])]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_SUBORDINATE_FRAGMENT(), "match", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&text.clone(), &py_str(","))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_HALF_A_UNIT(), "match", &[text.clone()])) {
    return Py::Bool(false);
    }
    word = mcall(&mcall(&mcall(&stdlib_call("re", "match", &[py_str("[A-Za-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e)), "group", &[Py::Int(0)]), "lower", &[]), "strip", &[py_str("'")]);
    return { let __b = Py::Bool(!py_contains(&g_NOT_A_VERB(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_AN_OPENER(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")]))) } } } };
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
    let mut opening: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { py_str("") };
    let _ = py_str("`_steps` by its verb list, and then by its shape: see `_opens_on_a_job_to_do`.");
    return { let __b = py_call(&g__steps_by_the_verb_list(), &[opening.clone(), after.clone()]); if py_truthy(&__b) { __b } else { _opens_on_a_job_to_do(&[opening.clone()]) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.fit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABBREVIATION_END" => Some(g_ABBREVIATION_END()),
            "ADVICE" => Some(g_ADVICE()),
            "AMOUNT" => Some(g_AMOUNT()),
            "ANSWER" => Some(g_ANSWER()),
            "ANY_DO_WITH" => Some(g_ANY_DO_WITH()),
            "AN_INSTRUCTION" => Some(g_AN_INSTRUCTION()),
            "AN_OFFER_NOT_A_TEST" => Some(g_AN_OFFER_NOT_A_TEST()),
            "AN_ORDINAL_OPENING" => Some(g_AN_ORDINAL_OPENING()),
            "ASKED_FOR" => Some(g_ASKED_FOR()),
            "ASKS_ABOUT_A_QUALITY" => Some(g_ASKS_ABOUT_A_QUALITY()),
            "ASKS_AN_AGE" => Some(g_ASKS_AN_AGE()),
            "ASKS_FOR_MEANING" => Some(g_ASKS_FOR_MEANING()),
            "ASKS_HEAD" => Some(g_ASKS_HEAD()),
            "ASKS_IF_SERIOUS" => Some(g_ASKS_IF_SERIOUS()),
            "ASKS_ITS_OWN_YES_NO" => Some(g_ASKS_ITS_OWN_YES_NO()),
            "ASKS_OUT_LOUD" => Some(g_ASKS_OUT_LOUD()),
            "ASKS_THE_BOT" => Some(g_ASKS_THE_BOT()),
            "ASKS_TO_BE_TOLD_WHERE" => Some(g_ASKS_TO_BE_TOLD_WHERE()),
            "ASKS_WHEN" => Some(g_ASKS_WHEN()),
            "ASKS_YES_OR_NO" => Some(g_ASKS_YES_OR_NO()),
            "ASK_TO_ASK" => Some(g_ASK_TO_ASK()),
            "A_BARE_NUMBER" => Some(g_A_BARE_NUMBER()),
            "A_BOUND_THAT_IS_THE_ANSWER" => Some(g_A_BOUND_THAT_IS_THE_ANSWER()),
            "A_CALCULATION" => Some(g_A_CALCULATION()),
            "A_COUNT_OF_THINGS" => Some(g_A_COUNT_OF_THINGS()),
            "A_COVER_LETTER" => Some(g_A_COVER_LETTER()),
            "A_DURATION_WORD" => Some(g_A_DURATION_WORD()),
            "A_FEW_THINGS" => Some(g_A_FEW_THINGS()),
            "A_FIGURE_NAMED" => Some(g_A_FIGURE_NAMED()),
            "A_FIGURE_OF_A_MONEY_VERB" => Some(g_A_FIGURE_OF_A_MONEY_VERB()),
            "A_GOOD_TIME" => Some(g_A_GOOD_TIME()),
            "A_HARM_IS_WHAT_IT_IS" => Some(g_A_HARM_IS_WHAT_IT_IS()),
            "A_HOLDER" => Some(g_A_HOLDER()),
            "A_JOB_LANDS" => Some(g_A_JOB_LANDS()),
            "A_JUDGEMENT" => Some(g_A_JUDGEMENT()),
            "A_LIMIT_ON_ITS_USE" => Some(g_A_LIMIT_ON_ITS_USE()),
            "A_PERSON_TO_GIVE" => Some(g_A_PERSON_TO_GIVE()),
            "A_PRICE_ASK" => Some(g_A_PRICE_ASK()),
            "A_QUANTITY_ASK" => Some(g_A_QUANTITY_ASK()),
            "A_RECALL_ASK" => Some(g_A_RECALL_ASK()),
            "A_RULE_THAT_SETTLES_IT" => Some(g_A_RULE_THAT_SETTLES_IT()),
            "A_SIDE_WINS" => Some(g_A_SIDE_WINS()),
            "A_SIZE_LONG" => Some(g_A_SIZE_LONG()),
            "A_STANDING_AFTER_THE_TEST" => Some(g_A_STANDING_AFTER_THE_TEST()),
            "A_SUBORDINATE_FRAGMENT" => Some(g_A_SUBORDINATE_FRAGMENT()),
            "A_THING_TO_GIVE" => Some(g_A_THING_TO_GIVE()),
            "A_TIME_COUNT" => Some(g_A_TIME_COUNT()),
            "A_TRADE" => Some(g_A_TRADE()),
            "A_UNIT" => Some(g_A_UNIT()),
            "A_VERB_THEN_THE_THING" => Some(g_A_VERB_THEN_THE_THING()),
            "A_WELL_WISHING" => Some(g_A_WELL_WISHING()),
            "BACK_HEAD" => Some(g_BACK_HEAD()),
            "BALD" => Some(g_BALD()),
            "BALD_VERB" => Some(g_BALD_VERB()),
            "BARE_A_RECALL" => Some(g_BARE_A_RECALL()),
            "BARE_A_SECOND_QUESTION" => Some(g_BARE_A_SECOND_QUESTION()),
            "BARE_NOTHING_ASKED" => Some(g_BARE_NOTHING_ASKED()),
            "BARE_NOT_A_THING_ANYWHERE" => Some(g_BARE_NOT_A_THING_ANYWHERE()),
            "BARE_NOT_A_THING_AT_THE_START" => Some(g_BARE_NOT_A_THING_AT_THE_START()),
            "BARE_RANGE" => Some(g_BARE_RANGE()),
            "BARE_VALUE_NOUN" => Some(g_BARE_VALUE_NOUN()),
            "BARE_WHAT_AS_IS" => Some(g_BARE_WHAT_AS_IS()),
            "BARE_WHAT_IS_PLAIN" => Some(g_BARE_WHAT_IS_PLAIN()),
            "BARE_WRITE" => Some(g_BARE_WRITE()),
            "BEST_OF_MANY" => Some(g_BEST_OF_MANY()),
            "BEST_WAY" => Some(g_BEST_WAY()),
            "CALENDAR_WORD" => Some(g_CALENDAR_WORD()),
            "CAUSE" => Some(g_CAUSE()),
            "CAUSE_IS" => Some(g_CAUSE_IS()),
            "CAUSE_WHEN" => Some(g_CAUSE_WHEN()),
            "CHOICE" => Some(g_CHOICE()),
            "CLOCK" => Some(g_CLOCK()),
            "CLOCK_ANYWHERE" => Some(g_CLOCK_ANYWHERE()),
            "CLOCK_SURE" => Some(g_CLOCK_SURE()),
            "COMFORT" => Some(g_COMFORT()),
            "COMPARATIVE_THAN" => Some(g_COMPARATIVE_THAN()),
            "COMPARE" => Some(g_COMPARE()),
            "COMPARED" => Some(g_COMPARED()),
            "COMPARE_MYSELF" => Some(g_COMPARE_MYSELF()),
            "COMPARISON" => Some(g_COMPARISON()),
            "CONFIRM_CAUSE" => Some(g_CONFIRM_CAUSE()),
            "CONTRACTION_OPENER" => Some(g_CONTRACTION_OPENER()),
            "CONTRAST" => Some(g_CONTRAST()),
            "CONVERSION" => Some(g_CONVERSION()),
            "CONVERTED_FIGURE" => Some(g_CONVERTED_FIGURE()),
            "COPULA" => Some(g_COPULA()),
            "COSTS_NOTHING" => Some(g_COSTS_NOTHING()),
            "COST_ASK" => Some(g_COST_ASK()),
            "COST_STANDS_IN" => Some(g_COST_STANDS_IN()),
            "COUNTED_LIST" => Some(g_COUNTED_LIST()),
            "COUNTED_STEPS" => Some(g_COUNTED_STEPS()),
            "DATE" => Some(g_DATE()),
            "DAY_NAME" => Some(g_DAY_NAME()),
            "DEFINES" => Some(g_DEFINES()),
            "DIFFERENCE" => Some(g_DIFFERENCE()),
            "DIRECTED" => Some(g_DIRECTED()),
            "DOES" => Some(g_DOES()),
            "DOES_NOT" => Some(g_DOES_NOT()),
            "DOES_NOT_ANYWHERE" => Some(g_DOES_NOT_ANYWHERE()),
            "DOING_WRONG" => Some(g_DOING_WRONG()),
            "DO_WITH_A_PERSON" => Some(g_DO_WITH_A_PERSON()),
            "DURATION" => Some(g_DURATION()),
            "ECHO_HEAD" => Some(g_ECHO_HEAD()),
            "ECHO_VERDICT" => Some(g_ECHO_VERDICT()),
            "EDGED_PART_OF_DAY" => Some(g_EDGED_PART_OF_DAY()),
            "EVIDENCE_SAYS" => Some(g_EVIDENCE_SAYS()),
            "EXPLAINS" => Some(g_EXPLAINS()),
            "EXPLICIT_COMPARE" => Some(g_EXPLICIT_COMPARE()),
            "EXTENT" => Some(g_EXTENT()),
            "FIGURE" => Some(g_FIGURE()),
            "FIGURE_HEAD" => Some(g_FIGURE_HEAD()),
            "FIRST_PERSON_VERDICT" => Some(g_FIRST_PERSON_VERDICT()),
            "FIX_ASK" => Some(g_FIX_ASK()),
            "FOLLOWUP" => Some(g_FOLLOWUP()),
            "FOR_A_LIST" => Some(g_FOR_A_LIST()),
            "FOR_A_TIME" => Some(g_FOR_A_TIME()),
            "FRAME_HEAD" => Some(g_FRAME_HEAD()),
            "GARDEN" => Some(g_GARDEN()),
            "GIVE_ME_AN_IDIOM" => Some(g_GIVE_ME_AN_IDIOM()),
            "GIVE_ME_A_FEW" => Some(g_GIVE_ME_A_FEW()),
            "GIVE_ME_LEAD_IN" => Some(g_GIVE_ME_LEAD_IN()),
            "GIVE_ME_MADE_UP" => Some(g_GIVE_ME_MADE_UP()),
            "GIVE_ME_NOT_A_THING" => Some(g_GIVE_ME_NOT_A_THING()),
            "GIVE_ME_ONE" => Some(g_GIVE_ME_ONE()),
            "GIVING_FRAME" => Some(g_GIVING_FRAME()),
            "GOOD_FIGURE" => Some(g_GOOD_FIGURE()),
            "GOOD_FIGURE_WORD" => Some(g_GOOD_FIGURE_WORD()),
            "GOOD_MEASURE" => Some(g_GOOD_MEASURE()),
            "GOOD_ONE" => Some(g_GOOD_ONE()),
            "GOOD_THING" => Some(g_GOOD_THING()),
            "GOOD_WAY" => Some(g_GOOD_WAY()),
            "HALF_A_UNIT" => Some(g_HALF_A_UNIT()),
            "HANDOFF" => Some(g_HANDOFF()),
            "HAVE_RECALL" => Some(g_HAVE_RECALL()),
            "HELP_ASK_ANY" => Some(g_HELP_ASK_ANY()),
            "HELP_ASK_NO_TASK" => Some(g_HELP_ASK_NO_TASK()),
            "HELP_ME" => Some(g_HELP_ME()),
            "HELP_WORD" => Some(g_HELP_WORD()),
            "HOUR_WORD" => Some(g_HOUR_WORD()),
            "HOW_ARE_YOU" => Some(g_HOW_ARE_YOU()),
            "HOW_ASK_FORM" => Some(g_HOW_ASK_FORM()),
            "HOW_COME" => Some(g_HOW_COME()),
            "HOW_I" => Some(g_HOW_I()),
            "HOW_IT_HAPPENED" => Some(g_HOW_IT_HAPPENED()),
            "HOW_IT_WENT" => Some(g_HOW_IT_WENT()),
            "HOW_IT_WORKS" => Some(g_HOW_IT_WORKS()),
            "HOW_MANY_TIME" => Some(g_HOW_MANY_TIME()),
            "HOW_MEASURE" => Some(g_HOW_MEASURE()),
            "HOW_MEASURE_HEAD" => Some(g_HOW_MEASURE_HEAD()),
            "HOW_SOON" => Some(g_HOW_SOON()),
            "HOW_TO_ANSWER" => Some(g_HOW_TO_ANSWER()),
            "HOW_TO_DO" => Some(g_HOW_TO_DO()),
            "HYPOTHETICAL" => Some(g_HYPOTHETICAL()),
            "IF_IT_HAPPENS" => Some(g_IF_IT_HAPPENS()),
            "IMPERATIVE" => Some(g_IMPERATIVE()),
            "INSTRUCTS" => Some(g_INSTRUCTS()),
            "INSTRUCTS_CLAUSE" => Some(g_INSTRUCTS_CLAUSE()),
            "IS_IT_DONE" => Some(g_IS_IT_DONE()),
            "IT_HELPS" => Some(g_IT_HELPS()),
            "I_OFFER" => Some(g_I_OFFER()),
            "JOB" => Some(g_JOB()),
            "JOB_ANY" => Some(g_JOB_ANY()),
            "JUDGED_MEASURE" => Some(g_JUDGED_MEASURE()),
            "KINDS_OF" => Some(g_KINDS_OF()),
            "LABEL" => Some(g_LABEL()),
            "LABEL_HEAD" => Some(g_LABEL_HEAD()),
            "LEADING_JOIN" => Some(g_LEADING_JOIN()),
            "LENGTH_ASKED" => Some(g_LENGTH_ASKED()),
            "LINE_TEXT_FIELDS" => Some(g_LINE_TEXT_FIELDS()),
            "LIST_AFTER_COLON" => Some(g_LIST_AFTER_COLON()),
            "LONG_TIME" => Some(g_LONG_TIME()),
            "MADE_OF" => Some(g_MADE_OF()),
            "MEANING" => Some(g_MEANING()),
            "MEANS" => Some(g_MEANS()),
            "MEANS_ARTICLE" => Some(g_MEANS_ARTICLE()),
            "MEANS_BY_DOING" => Some(g_MEANS_BY_DOING()),
            "MEANS_INFINITIVE" => Some(g_MEANS_INFINITIVE()),
            "MEANS_NAMED" => Some(g_MEANS_NAMED()),
            "MEANS_OF" => Some(g_MEANS_OF()),
            "MEASURED" => Some(g_MEASURED()),
            "MEASURE_LABELS" => Some(g_MEASURE_LABELS()),
            "MECHANISM" => Some(g_MECHANISM()),
            "MEMBERS_ASK" => Some(g_MEMBERS_ASK()),
            "MERIT" => Some(g_MERIT()),
            "METHOD_OPENS" => Some(g_METHOD_OPENS()),
            "METHOD_VERBS" => Some(g_METHOD_VERBS()),
            "MINUTES_AS_A_LENGTH" => Some(g_MINUTES_AS_A_LENGTH()),
            "MONEY" => Some(g_MONEY()),
            "MONEY_RATE" => Some(g_MONEY_RATE()),
            "MONEY_VERB_NOT_A_PRICE" => Some(g_MONEY_VERB_NOT_A_PRICE()),
            "MUST_BE_A_TRADE" => Some(g_MUST_BE_A_TRADE()),
            "MY_THING_CALLED" => Some(g_MY_THING_CALLED()),
            "NAMED_CLOCK" => Some(g_NAMED_CLOCK()),
            "NAMED_HEAD" => Some(g_NAMED_HEAD()),
            "NAMED_TIME" => Some(g_NAMED_TIME()),
            "NAMED_WORDS" => Some(g_NAMED_WORDS()),
            "NAMES_IN_EMPHASIS" => Some(g_NAMES_IN_EMPHASIS()),
            "NAME_STOP" => Some(g_NAME_STOP()),
            "NAMING_STOP" => Some(g_NAMING_STOP()),
            "NEED_TO" => Some(g_NEED_TO()),
            "NEED_TO_ASKED" => Some(g_NEED_TO_ASKED()),
            "NEED_TO_DO" => Some(g_NEED_TO_DO()),
            "NEGATED_AUX" => Some(g_NEGATED_AUX()),
            "NEGATES" => Some(g_NEGATES()),
            "NEVER" => Some(g_NEVER()),
            "NEW_ASK" => Some(g_NEW_ASK()),
            "NEXT_IDEA" => Some(g_NEXT_IDEA()),
            "NOTHING_TO_GO_ON" => Some(g_NOTHING_TO_GO_ON()),
            "NOT_AN_OPENER" => Some(g_NOT_AN_OPENER()),
            "NOT_A_COMPARISON" => Some(g_NOT_A_COMPARISON()),
            "NOT_A_FIGURE" => Some(g_NOT_A_FIGURE()),
            "NOT_A_GIVING_THING" => Some(g_NOT_A_GIVING_THING()),
            "NOT_A_LABEL" => Some(g_NOT_A_LABEL()),
            "NOT_A_PRICE_ASK" => Some(g_NOT_A_PRICE_ASK()),
            "NOT_A_STEP_BUT_REASSURANCE" => Some(g_NOT_A_STEP_BUT_REASSURANCE()),
            "NOT_A_SUBJECT" => Some(g_NOT_A_SUBJECT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_A_VERB" => Some(g_NOT_A_VERB()),
            "NOT_A_VERB_HEAD" => Some(g_NOT_A_VERB_HEAD()),
            "NOT_A_YEAR" => Some(g_NOT_A_YEAR()),
            "NOT_KINDS_OF" => Some(g_NOT_KINDS_OF()),
            "NOT_STEPS" => Some(g_NOT_STEPS()),
            "NOT_WORDS_ANYWAY" => Some(g_NOT_WORDS_ANYWAY()),
            "NOT_WORDS_ASK" => Some(g_NOT_WORDS_ASK()),
            "NO_FIXED_TIME" => Some(g_NO_FIXED_TIME()),
            "NO_JOB" => Some(g_NO_JOB()),
            "ON_THE_CLOCK" => Some(g_ON_THE_CLOCK()),
            "OPEN_WHO_HELP" => Some(g_OPEN_WHO_HELP()),
            "OPINION" => Some(g_OPINION()),
            "OPTIONS" => Some(g_OPTIONS()),
            "OPTIONS_ASK" => Some(g_OPTIONS_ASK()),
            "ORIGIN_ASK" => Some(g_ORIGIN_ASK()),
            "ORIGIN_PERSON" => Some(g_ORIGIN_PERSON()),
            "OTHER_QUESTION" => Some(g_OTHER_QUESTION()),
            "OUTCOME_ASK" => Some(g_OUTCOME_ASK()),
            "PAIRING_ASK" => Some(g_PAIRING_ASK()),
            "PAST_DEFINES" => Some(g_PAST_DEFINES()),
            "PATH_THEN_VERB" => Some(g_PATH_THEN_VERB()),
            "PERMISSION" => Some(g_PERMISSION()),
            "PERMISSION_RULE" => Some(g_PERMISSION_RULE()),
            "PERSON" => Some(g_PERSON()),
            "PERSON_FULL_NAME" => Some(g_PERSON_FULL_NAME()),
            "PERSON_NAME" => Some(g_PERSON_NAME()),
            "PERSON_NAMED" => Some(g_PERSON_NAMED()),
            "PERSON_RELATION" => Some(g_PERSON_RELATION()),
            "PERSON_WORD" => Some(g_PERSON_WORD()),
            "PICK" => Some(g_PICK()),
            "PICKS_BY_COMPARISON" => Some(g_PICKS_BY_COMPARISON()),
            "PICK_BY_CASE" => Some(g_PICK_BY_CASE()),
            "PICK_ONE" => Some(g_PICK_ONE()),
            "PICK_ONE_OF_A_KIND" => Some(g_PICK_ONE_OF_A_KIND()),
            "PICK_SIDES" => Some(g_PICK_SIDES()),
            "PLACE" => Some(g_PLACE()),
            "PLACE_AS_EVIDENCE" => Some(g_PLACE_AS_EVIDENCE()),
            "PLACE_AT_HOME" => Some(g_PLACE_AT_HOME()),
            "PLACE_KIND" => Some(g_PLACE_KIND()),
            "PLACE_NAME" => Some(g_PLACE_NAME()),
            "PLACE_NAME_ANY" => Some(g_PLACE_NAME_ANY()),
            "PLACE_WORD" => Some(g_PLACE_WORD()),
            "PLAN_VERB" => Some(g_PLAN_VERB()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "POINT_ASK" => Some(g_POINT_ASK()),
            "POINT_NOT_A_THING" => Some(g_POINT_NOT_A_THING()),
            "PREFERS" => Some(g_PREFERS()),
            "PRICE_ADJECTIVE" => Some(g_PRICE_ADJECTIVE()),
            "PRICE_ADJECTIVE_KEEPS" => Some(g_PRICE_ADJECTIVE_KEEPS()),
            "PRICE_ADJECTIVE_THE_ASK_IS_LATER" => Some(g_PRICE_ADJECTIVE_THE_ASK_IS_LATER()),
            "PRICE_ADJECTIVE_WHY" => Some(g_PRICE_ADJECTIVE_WHY()),
            "PRICE_ASKED" => Some(g_PRICE_ASKED()),
            "PRICE_GIVEN" => Some(g_PRICE_GIVEN()),
            "PRICE_IDIOM" => Some(g_PRICE_IDIOM()),
            "PUNT" => Some(g_PUNT()),
            "PURPOSE" => Some(g_PURPOSE()),
            "PURPOSE_TO" => Some(g_PURPOSE_TO()),
            "QUALITY_IS" => Some(g_QUALITY_IS()),
            "QUESTION_IN_THE_LINE" => Some(g_QUESTION_IN_THE_LINE()),
            "QUOTED_NAME" => Some(g_QUOTED_NAME()),
            "RATE" => Some(g_RATE()),
            "RATE_OF_A_COUNT" => Some(g_RATE_OF_A_COUNT()),
            "RATE_PHRASE" => Some(g_RATE_PHRASE()),
            "REASON" => Some(g_REASON()),
            "REASON_ASK" => Some(g_REASON_ASK()),
            "REASON_OPENS" => Some(g_REASON_OPENS()),
            "RECALL_AMOUNT" => Some(g_RECALL_AMOUNT()),
            "RECALL_KIND_NOUN" => Some(g_RECALL_KIND_NOUN()),
            "RECALL_KIND_NOUN_ASK" => Some(g_RECALL_KIND_NOUN_ASK()),
            "RECALL_NOT" => Some(g_RECALL_NOT()),
            "RECALL_OF_MINE" => Some(g_RECALL_OF_MINE()),
            "RECALL_PERSON" => Some(g_RECALL_PERSON()),
            "RECALL_PLACE" => Some(g_RECALL_PLACE()),
            "RECALL_PRONOUN" => Some(g_RECALL_PRONOUN()),
            "RECALL_SAID" => Some(g_RECALL_SAID()),
            "RECALL_SHAPE_BEFORE" => Some(g_RECALL_SHAPE_BEFORE()),
            "RECALL_SHAPE_BEFORE_KIND" => Some(g_RECALL_SHAPE_BEFORE_KIND()),
            "RECALL_TIME" => Some(g_RECALL_TIME()),
            "REFERS_BACK" => Some(g_REFERS_BACK()),
            "REPEAT_LEAD" => Some(g_REPEAT_LEAD()),
            "REPORTED" => Some(g_REPORTED()),
            "RULE_CONDITION" => Some(g_RULE_CONDITION()),
            "RULE_DROP_IT" => Some(g_RULE_DROP_IT()),
            "RULE_GO_NEW" => Some(g_RULE_GO_NEW()),
            "RULE_KEEP_IT" => Some(g_RULE_KEEP_IT()),
            "SAID_WORDS" => Some(g_SAID_WORDS()),
            "SAYS_NO" => Some(g_SAYS_NO()),
            "SAYS_NOTHING_ABOUT_IT" => Some(g_SAYS_NOTHING_ABOUT_IT()),
            "SAYS_WHAT_IT_DOES" => Some(g_SAYS_WHAT_IT_DOES()),
            "SEASON" => Some(g_SEASON()),
            "SEASON_WHEN" => Some(g_SEASON_WHEN()),
            "SEEKS_HELP" => Some(g_SEEKS_HELP()),
            "SENDS_THEM_ON" => Some(g_SENDS_THEM_ON()),
            "SENTENCE_END" => Some(g_SENTENCE_END()),
            "SEQUENCES" => Some(g_SEQUENCES()),
            "SETTLED_ON" => Some(g_SETTLED_ON()),
            "SET_LABEL" => Some(g_SET_LABEL()),
            "SIDES" => Some(g_SIDES()),
            "SIDES_VS" => Some(g_SIDES_VS()),
            "SIDE_FILLER" => Some(g_SIDE_FILLER()),
            "SIDE_PAIRS" => Some(g_SIDE_PAIRS()),
            "SIGN_OFF_LEAD" => Some(g_SIGN_OFF_LEAD()),
            "SOFT" => Some(g_SOFT()),
            "SOURCE" => Some(g_SOURCE()),
            "SOURCE_COLON" => Some(g_SOURCE_COLON()),
            "SOURCE_DOES" => Some(g_SOURCE_DOES()),
            "SOURCE_LIST" => Some(g_SOURCE_LIST()),
            "SOURCE_MADE" => Some(g_SOURCE_MADE()),
            "SOURCE_MAKES" => Some(g_SOURCE_MAKES()),
            "SPECIFIC_ASKS" => Some(g_SPECIFIC_ASKS()),
            "SPOT_OF_THE_THING" => Some(g_SPOT_OF_THE_THING()),
            "SPOT_ON_THE_THING" => Some(g_SPOT_ON_THE_THING()),
            "SPOT_PART" => Some(g_SPOT_PART()),
            "STAGE" => Some(g_STAGE()),
            "STAGE_WHEN" => Some(g_STAGE_WHEN()),
            "STATES_HEAD" => Some(g_STATES_HEAD()),
            "STEPS" => Some(g_STEPS()),
            "STEPS_ARE_COUNTED" => Some(g_STEPS_ARE_COUNTED()),
            "STEPS_ASK" => Some(g_STEPS_ASK()),
            "STEPS_AS_A_DISTANCE" => Some(g_STEPS_AS_A_DISTANCE()),
            "STEPS_INSIDE_A_LIST" => Some(g_STEPS_INSIDE_A_LIST()),
            "SUBSTITUTE" => Some(g_SUBSTITUTE()),
            "SUM_ASKS" => Some(g_SUM_ASKS()),
            "SUM_OUT_LOUD" => Some(g_SUM_OUT_LOUD()),
            "TAIL_VERDICT_ASK" => Some(g_TAIL_VERDICT_ASK()),
            "TELLS_A_TEST" => Some(g_TELLS_A_TEST()),
            "TELL_ASK" => Some(g_TELL_ASK()),
            "TEST_IS_THE_VERDICT" => Some(g_TEST_IS_THE_VERDICT()),
            "THEIR_OWN" => Some(g_THEIR_OWN()),
            "THE_SIGN" => Some(g_THE_SIGN()),
            "THE_THING_ASKED" => Some(g_THE_THING_ASKED()),
            "THE_WINNER_NAMED" => Some(g_THE_WINNER_NAMED()),
            "THINGS_VERB" => Some(g_THINGS_VERB()),
            "THING_THAT_EXTENDS" => Some(g_THING_THAT_EXTENDS()),
            "TIME" => Some(g_TIME()),
            "TIME_LEAD_IN" => Some(g_TIME_LEAD_IN()),
            "TRAILING_VERDICT" => Some(g_TRAILING_VERDICT()),
            "UNSETTLED" => Some(g_UNSETTLED()),
            "USUALLY_IS" => Some(g_USUALLY_IS()),
            "VERDICT_ABOUT_IT" => Some(g_VERDICT_ABOUT_IT()),
            "VERDICT_AFTER_A_CASE" => Some(g_VERDICT_AFTER_A_CASE()),
            "VERDICT_AFTER_A_FIGURE" => Some(g_VERDICT_AFTER_A_FIGURE()),
            "VERDICT_BAND" => Some(g_VERDICT_BAND()),
            "VERDICT_ECHO" => Some(g_VERDICT_ECHO()),
            "VERDICT_FRAME" => Some(g_VERDICT_FRAME()),
            "VERDICT_INSIDE" => Some(g_VERDICT_INSIDE()),
            "VERDICT_INSIDE_QUALITY" => Some(g_VERDICT_INSIDE_QUALITY()),
            "VERDICT_ON_A_SCALE" => Some(g_VERDICT_ON_A_SCALE()),
            "VERDICT_OPEN" => Some(g_VERDICT_OPEN()),
            "VERDICT_RULE" => Some(g_VERDICT_RULE()),
            "WANTS" => Some(g_WANTS()),
            "WANTS_INFO" => Some(g_WANTS_INFO()),
            "WANTS_OF_THE_PERSON" => Some(g_WANTS_OF_THE_PERSON()),
            "WANTS_THE_CAUSE" => Some(g_WANTS_THE_CAUSE()),
            "WHAT_AGE" => Some(g_WHAT_AGE()),
            "WHAT_ARE_MINE" => Some(g_WHAT_ARE_MINE()),
            "WHAT_ARE_TIMES" => Some(g_WHAT_ARE_TIMES()),
            "WHAT_CAN_I_DO" => Some(g_WHAT_CAN_I_DO()),
            "WHAT_CAN_I_DO_WITH" => Some(g_WHAT_CAN_I_DO_WITH()),
            "WHAT_DOES_MEASURE" => Some(g_WHAT_DOES_MEASURE()),
            "WHAT_ELSE_STEPS" => Some(g_WHAT_ELSE_STEPS()),
            "WHAT_ELSE_WITH" => Some(g_WHAT_ELSE_WITH()),
            "WHAT_FIGURE" => Some(g_WHAT_FIGURE()),
            "WHAT_HELPS" => Some(g_WHAT_HELPS()),
            "WHAT_IS_PER" => Some(g_WHAT_IS_PER()),
            "WHAT_I_GROW" => Some(g_WHAT_I_GROW()),
            "WHAT_I_NEED" => Some(g_WHAT_I_NEED()),
            "WHAT_I_WRITE" => Some(g_WHAT_I_WRITE()),
            "WHAT_SHOULD_I_DO_WITH" => Some(g_WHAT_SHOULD_I_DO_WITH()),
            "WHAT_THING_FRAGMENT" => Some(g_WHAT_THING_FRAGMENT()),
            "WHAT_TIME_NAMED" => Some(g_WHAT_TIME_NAMED()),
            "WHAT_TO_BAKE" => Some(g_WHAT_TO_BAKE()),
            "WHAT_TO_DO" => Some(g_WHAT_TO_DO()),
            "WHAT_TO_OPENS" => Some(g_WHAT_TO_OPENS()),
            "WHAT_TO_PICK" => Some(g_WHAT_TO_PICK()),
            "WHAT_TO_STEPS" => Some(g_WHAT_TO_STEPS()),
            "WHAT_YOU_DO" => Some(g_WHAT_YOU_DO()),
            "WHAT_YOU_TAKE" => Some(g_WHAT_YOU_TAKE()),
            "WHEN_ASKS" => Some(g_WHEN_ASKS()),
            "WHERE_ALONE" => Some(g_WHERE_ALONE()),
            "WHERE_MONEY_GOES" => Some(g_WHERE_MONEY_GOES()),
            "WHERE_POINT" => Some(g_WHERE_POINT()),
            "WHERE_RECALL" => Some(g_WHERE_RECALL()),
            "WHERE_RELATIVE" => Some(g_WHERE_RELATIVE()),
            "WHERE_START" => Some(g_WHERE_START()),
            "WHICH_DAY_IS_BEST" => Some(g_WHICH_DAY_IS_BEST()),
            "WHO_AND_MEASURE" => Some(g_WHO_AND_MEASURE()),
            "WHO_OF_A_NOUN" => Some(g_WHO_OF_A_NOUN()),
            "WHO_OF_A_PRONOUN" => Some(g_WHO_OF_A_PRONOUN()),
            "WHO_OPENS" => Some(g_WHO_OPENS()),
            "WORDS_ASK" => Some(g_WORDS_ASK()),
            "WOULD" => Some(g_WOULD()),
            "WRITES_A_FORM" => Some(g_WRITES_A_FORM()),
            "WRITES_IN_A_DOCUMENT" => Some(g_WRITES_IN_A_DOCUMENT()),
            "WRITES_TO_A_MESSAGE" => Some(g_WRITES_TO_A_MESSAGE()),
            "WRITTEN_THING" => Some(g_WRITTEN_THING()),
            "WRONG_ASKED_ABOUT" => Some(g_WRONG_ASKED_ABOUT()),
            "WRONG_RECALL" => Some(g_WRONG_RECALL()),
            "YESNO" => Some(g_YESNO()),
            "_ADVISES_THE_THING" => Some(g__ADVISES_THE_THING()),
            "_AN_AGE_BARE" => Some(g__AN_AGE_BARE()),
            "_AN_AGE_BIRTHDAY" => Some(g__AN_AGE_BIRTHDAY()),
            "_AN_AGE_MONTHS" => Some(g__AN_AGE_MONTHS()),
            "_AN_AGE_NAMED" => Some(g__AN_AGE_NAMED()),
            "_AN_AGE_SPELLED_OPENS" => Some(g__AN_AGE_SPELLED_OPENS()),
            "_AN_AGE_YEARS" => Some(g__AN_AGE_YEARS()),
            "_ARTICLE" => Some(g__ARTICLE()),
            "_ASK_CLAUSE_OPENS" => Some(g__ASK_CLAUSE_OPENS()),
            "_ASK_CLAUSE_SPLIT" => Some(g__ASK_CLAUSE_SPLIT()),
            "_A_DOING" => Some(g__A_DOING()),
            "_A_FIGURE_AGAINST_THE_STANDARD" => Some(g__A_FIGURE_AGAINST_THE_STANDARD()),
            "_A_QUANTITY_NUMBER" => Some(g__A_QUANTITY_NUMBER()),
            "_A_QUANTITY_RELATION" => Some(g__A_QUANTITY_RELATION()),
            "_A_QUANTITY_UNIT" => Some(g__A_QUANTITY_UNIT()),
            "_A_QUOTE" => Some(g__A_QUOTE()),
            "_A_SOCIAL_OPEN" => Some(g__A_SOCIAL_OPEN()),
            "_CHECKS" => Some(g__CHECKS()),
            "_CLEAR" => Some(g__CLEAR()),
            "_COPULA" => Some(g__COPULA()),
            "_DURATION_QUALIFIER" => Some(g__DURATION_QUALIFIER()),
            "_EMPTY" => Some(g__EMPTY()),
            "_ENDS_THE_THING" => Some(g__ENDS_THE_THING()),
            "_FIGURES" => Some(g__FIGURES()),
            "_FIGURE_HEAD" => Some(g__FIGURE_HEAD()),
            "_HOUR_24" => Some(g__HOUR_24()),
            "_JUDGES_THE_THING" => Some(g__JUDGES_THE_THING()),
            "_LIST_OF_THINGS" => Some(g__LIST_OF_THINGS()),
            "_MEASURE_AFTER" => Some(g__MEASURE_AFTER()),
            "_MEASURE_THEN_VERB" => Some(g__MEASURE_THEN_VERB()),
            "_MONEY" => Some(g__MONEY()),
            "_NAMING_WORD" => Some(g__NAMING_WORD()),
            "_NOT_A_ROLE" => Some(g__NOT_A_ROLE()),
            "_NOT_A_THING" => Some(g__NOT_A_THING()),
            "_NOT_A_WORD" => Some(g__NOT_A_WORD()),
            "_NOT_THE_SUBJECT" => Some(g__NOT_THE_SUBJECT()),
            "_NO_WAY_TO_KNOW" => Some(g__NO_WAY_TO_KNOW()),
            "_OPINION_OF_THE_THING" => Some(g__OPINION_OF_THE_THING()),
            "_PARTY" => Some(g__PARTY()),
            "_PREFERS_STATED" => Some(g__PREFERS_STATED()),
            "_PRONOUN_SUBJECT" => Some(g__PRONOUN_SUBJECT()),
            "_PUTS_THE_THING_TO_USE" => Some(g__PUTS_THE_THING_TO_USE()),
            "_QUALITIES" => Some(g__QUALITIES()),
            "_QUALITY_OF_A_THING" => Some(g__QUALITY_OF_A_THING()),
            "_QUALITY_SAID" => Some(g__QUALITY_SAID()),
            "_ROLE_ENDS" => Some(g__ROLE_ENDS()),
            "_ROLE_OFFICES" => Some(g__ROLE_OFFICES()),
            "_SAYS_WHAT_IT_IS" => Some(g__SAYS_WHAT_IT_IS()),
            "_SIDE" => Some(g__SIDE()),
            "_SIDE_EMPTY" => Some(g__SIDE_EMPTY()),
            "_STEPS_IN" => Some(g__STEPS_IN()),
            "_TALKING_ABOUT_A_LENGTH" => Some(g__TALKING_ABOUT_A_LENGTH()),
            "_THE" => Some(g__THE()),
            "_THE_VERDICT_IS_EARLY" => Some(g__THE_VERDICT_IS_EARLY()),
            "_TRAILING_MEASURE" => Some(g__TRAILING_MEASURE()),
            "_WORD" => Some(g__WORD()),
            "_WORD_NUMBER" => Some(g__WORD_NUMBER()),
            "_WORKING" => Some(g__WORKING()),
            "_a_bare_definition" => Some(py_func("_a_bare_definition", |args: &[Py]| _a_bare_definition(args))),
            "_a_clear_time" => Some(py_func("_a_clear_time", |args: &[Py]| _a_clear_time(args))),
            "_a_clear_time_at_the_head" => Some(py_func("_a_clear_time_at_the_head", |args: &[Py]| _a_clear_time_at_the_head(args))),
            "_a_comparison" => Some(py_func("_a_comparison", |args: &[Py]| _a_comparison(args))),
            "_a_figure" => Some(py_func("_a_figure", |args: &[Py]| _a_figure(args))),
            "_a_figure_that_counts" => Some(py_func("_a_figure_that_counts", |args: &[Py]| _a_figure_that_counts(args))),
            "_a_figure_then_the_verdict" => Some(py_func("_a_figure_then_the_verdict", |args: &[Py]| _a_figure_then_the_verdict(args))),
            "_a_fragment_asks_for_a_time" => Some(py_func("_a_fragment_asks_for_a_time", |args: &[Py]| _a_fragment_asks_for_a_time(args))),
            "_a_help_ask_that_names_nothing" => Some(py_func("_a_help_ask_that_names_nothing", |args: &[Py]| _a_help_ask_that_names_nothing(args))),
            "_a_list_of_things" => Some(py_func("_a_list_of_things", |args: &[Py]| _a_list_of_things(args))),
            "_a_meaning" => Some(py_func("_a_meaning", |args: &[Py]| _a_meaning(args))),
            "_a_meaning_or_a_statement" => Some(py_func("_a_meaning_or_a_statement", |args: &[Py]| _a_meaning_or_a_statement(args))),
            "_a_money_rate" => Some(py_func("_a_money_rate", |args: &[Py]| _a_money_rate(args))),
            "_a_party" => Some(py_func("_a_party", |args: &[Py]| _a_party(args))),
            "_a_past_definition" => Some(py_func("_a_past_definition", |args: &[Py]| _a_past_definition(args))),
            "_a_person" => Some(py_func("_a_person", |args: &[Py]| _a_person(args))),
            "_a_pick" => Some(py_func("_a_pick", |args: &[Py]| _a_pick(args))),
            "_a_place" => Some(py_func("_a_place", |args: &[Py]| _a_place(args))),
            "_a_price_asked" => Some(py_func("_a_price_asked", |args: &[Py]| _a_price_asked(args))),
            "_a_reason" => Some(py_func("_a_reason", |args: &[Py]| _a_reason(args))),
            "_a_rule_of_its_use" => Some(py_func("_a_rule_of_its_use", |args: &[Py]| _a_rule_of_its_use(args))),
            "_a_side_said_in_the_reply" => Some(py_func("_a_side_said_in_the_reply", |args: &[Py]| _a_side_said_in_the_reply(args))),
            "_a_side_word_is_named" => Some(py_func("_a_side_word_is_named", |args: &[Py]| _a_side_word_is_named(args))),
            "_a_sign_answers" => Some(py_func("_a_sign_answers", |args: &[Py]| _a_sign_answers(args))),
            "_a_sign_off" => Some(py_func("_a_sign_off", |args: &[Py]| _a_sign_off(args))),
            "_a_statement_answers_it" => Some(py_func("_a_statement_answers_it", |args: &[Py]| _a_statement_answers_it(args))),
            "_a_step_told_as_what_not_to_do" => Some(py_func("_a_step_told_as_what_not_to_do", |args: &[Py]| _a_step_told_as_what_not_to_do(args))),
            "_a_thing_to_define" => Some(py_func("_a_thing_to_define", |args: &[Py]| _a_thing_to_define(args))),
            "_a_time" => Some(py_func("_a_time", |args: &[Py]| _a_time(args))),
            "_a_time_a_sentence_on" => Some(py_func("_a_time_a_sentence_on", |args: &[Py]| _a_time_a_sentence_on(args))),
            "_a_who_a_person_owns" => Some(py_func("_a_who_a_person_owns", |args: &[Py]| _a_who_a_person_owns(args))),
            "_a_yesno" => Some(py_func("_a_yesno", |args: &[Py]| _a_yesno(args))),
            "_about_the_thing_the_line_named" => Some(py_func("_about_the_thing_the_line_named", |args: &[Py]| _about_the_thing_the_line_named(args))),
            "_an_age_is_given" => Some(py_func("_an_age_is_given", |args: &[Py]| _an_age_is_given(args))),
            "_an_amount" => Some(py_func("_an_amount", |args: &[Py]| _an_amount(args))),
            "_an_amount_at_the_head" => Some(py_func("_an_amount_at_the_head", |args: &[Py]| _an_amount_at_the_head(args))),
            "_an_amount_to_read" => Some(py_func("_an_amount_to_read", |args: &[Py]| _an_amount_to_read(args))),
            "_an_amount_with_a_unit" => Some(py_func("_an_amount_with_a_unit", |args: &[Py]| _an_amount_with_a_unit(args))),
            "_an_echoed_no" => Some(py_func("_an_echoed_no", |args: &[Py]| _an_echoed_no(args))),
            "_an_instruction" => Some(py_func("_an_instruction", |args: &[Py]| _an_instruction(args))),
            "_an_opening_job" => Some(py_func("_an_opening_job", |args: &[Py]| _an_opening_job(args))),
            "_an_origin" => Some(py_func("_an_origin", |args: &[Py]| _an_origin(args))),
            "_an_outcome_of_their_own_doing" => Some(py_func("_an_outcome_of_their_own_doing", |args: &[Py]| _an_outcome_of_their_own_doing(args))),
            "_asks_about_them" => Some(py_func("_asks_about_them", |args: &[Py]| _asks_about_them(args))),
            "_asks_the_person" => Some(py_func("_asks_the_person", |args: &[Py]| _asks_the_person(args))),
            "_defines_the_thing" => Some(py_func("_defines_the_thing", |args: &[Py]| _defines_the_thing(args))),
            "_do_with_ask" => Some(py_func("_do_with_ask", |args: &[Py]| _do_with_ask(args))),
            "_figure_at_the_head" => Some(py_func("_figure_at_the_head", |args: &[Py]| _figure_at_the_head(args))),
            "_gives_as_read" => Some(g__gives_as_read()),
            "_head_explains" => Some(py_func("_head_explains", |args: &[Py]| _head_explains(args))),
            "_imperative" => Some(py_func("_imperative", |args: &[Py]| _imperative(args))),
            "_initials_named" => Some(py_func("_initials_named", |args: &[Py]| _initials_named(args))),
            "_instruction_after_a_clause" => Some(py_func("_instruction_after_a_clause", |args: &[Py]| _instruction_after_a_clause(args))),
            "_line_text" => Some(py_func("_line_text", |args: &[Py]| _line_text(args))),
            "_listed_after" => Some(py_func("_listed_after", |args: &[Py]| _listed_after(args))),
            "_long_enough_a_name" => Some(py_func("_long_enough_a_name", |args: &[Py]| _long_enough_a_name(args))),
            "_marked_things" => Some(py_func("_marked_things", |args: &[Py]| _marked_things(args))),
            "_name_words" => Some(py_func("_name_words", |args: &[Py]| _name_words(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_names_a_pair" => Some(py_func("_names_a_pair", |args: &[Py]| _names_a_pair(args))),
            "_names_a_side" => Some(py_func("_names_a_side", |args: &[Py]| _names_a_side(args))),
            "_names_a_spot" => Some(py_func("_names_a_spot", |args: &[Py]| _names_a_spot(args))),
            "_names_both_sides" => Some(py_func("_names_both_sides", |args: &[Py]| _names_both_sides(args))),
            "_names_the_lines_thing" => Some(py_func("_names_the_lines_thing", |args: &[Py]| _names_the_lines_thing(args))),
            "_names_the_subject" => Some(py_func("_names_the_subject", |args: &[Py]| _names_the_subject(args))),
            "_names_what_the_line_named" => Some(py_func("_names_what_the_line_named", |args: &[Py]| _names_what_the_line_named(args))),
            "_opening_two" => Some(py_func("_opening_two", |args: &[Py]| _opening_two(args))),
            "_opens_on_a_job" => Some(py_func("_opens_on_a_job", |args: &[Py]| _opens_on_a_job(args))),
            "_opens_on_a_job_to_do" => Some(py_func("_opens_on_a_job_to_do", |args: &[Py]| _opens_on_a_job_to_do(args))),
            "_opens_on_the_lines_verb" => Some(py_func("_opens_on_the_lines_verb", |args: &[Py]| _opens_on_the_lines_verb(args))),
            "_options" => Some(py_func("_options", |args: &[Py]| _options(args))),
            "_recall_kind_behind" => Some(py_func("_recall_kind_behind", |args: &[Py]| _recall_kind_behind(args))),
            "_recall_kind_in_front" => Some(py_func("_recall_kind_in_front", |args: &[Py]| _recall_kind_in_front(args))),
            "_recall_kind_named" => Some(py_func("_recall_kind_named", |args: &[Py]| _recall_kind_named(args))),
            "_recall_wants" => Some(py_func("_recall_wants", |args: &[Py]| _recall_wants(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_says_no_about_the_asked_thing" => Some(py_func("_says_no_about_the_asked_thing", |args: &[Py]| _says_no_about_the_asked_thing(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_steps" => Some(py_func("_steps", |args: &[Py]| _steps(args))),
            "_steps_by_the_verb_list" => Some(g__steps_by_the_verb_list()),
            "_steps_in" => Some(py_func("_steps_in", |args: &[Py]| _steps_in(args))),
            "_subject_words" => Some(py_func("_subject_words", |args: &[Py]| _subject_words(args))),
            "_the_ask_clause" => Some(py_func("_the_ask_clause", |args: &[Py]| _the_ask_clause(args))),
            "_the_echo_question" => Some(py_func("_the_echo_question", |args: &[Py]| _the_echo_question(args))),
            "_the_figure_against_the_standard" => Some(py_func("_the_figure_against_the_standard", |args: &[Py]| _the_figure_against_the_standard(args))),
            "_the_first_sentence" => Some(py_func("_the_first_sentence", |args: &[Py]| _the_first_sentence(args))),
            "_the_give_is_to_the_bot" => Some(py_func("_the_give_is_to_the_bot", |args: &[Py]| _the_give_is_to_the_bot(args))),
            "_the_job_answers" => Some(py_func("_the_job_answers", |args: &[Py]| _the_job_answers(args))),
            "_the_job_is_split" => Some(py_func("_the_job_is_split", |args: &[Py]| _the_job_is_split(args))),
            "_the_job_named" => Some(py_func("_the_job_named", |args: &[Py]| _the_job_named(args))),
            "_the_pair_up" => Some(py_func("_the_pair_up", |args: &[Py]| _the_pair_up(args))),
            "_the_quality_answers" => Some(py_func("_the_quality_answers", |args: &[Py]| _the_quality_answers(args))),
            "_the_quality_is_about_the_thing" => Some(py_func("_the_quality_is_about_the_thing", |args: &[Py]| _the_quality_is_about_the_thing(args))),
            "_the_quality_is_said_of_the_thing" => Some(py_func("_the_quality_is_said_of_the_thing", |args: &[Py]| _the_quality_is_said_of_the_thing(args))),
            "_the_quality_it_was_asked_about" => Some(py_func("_the_quality_it_was_asked_about", |args: &[Py]| _the_quality_it_was_asked_about(args))),
            "_the_rule_names_both_sides" => Some(py_func("_the_rule_names_both_sides", |args: &[Py]| _the_rule_names_both_sides(args))),
            "_the_rule_settles_it" => Some(py_func("_the_rule_settles_it", |args: &[Py]| _the_rule_settles_it(args))),
            "_the_same_subject" => Some(py_func("_the_same_subject", |args: &[Py]| _the_same_subject(args))),
            "_the_same_word" => Some(py_func("_the_same_word", |args: &[Py]| _the_same_word(args))),
            "_the_shape_a_reply_gives" => Some(py_func("_the_shape_a_reply_gives", |args: &[Py]| _the_shape_a_reply_gives(args))),
            "_the_shape_named_outside_the_reading" => Some(py_func("_the_shape_named_outside_the_reading", |args: &[Py]| _the_shape_named_outside_the_reading(args))),
            "_the_side_wins" => Some(py_func("_the_side_wins", |args: &[Py]| _the_side_wins(args))),
            "_the_spot_a_thing_has" => Some(py_func("_the_spot_a_thing_has", |args: &[Py]| _the_spot_a_thing_has(args))),
            "_the_tail_says_what_it_is_about" => Some(py_func("_the_tail_says_what_it_is_about", |args: &[Py]| _the_tail_says_what_it_is_about(args))),
            "_the_thing_asked" => Some(py_func("_the_thing_asked", |args: &[Py]| _the_thing_asked(args))),
            "_the_thing_is_named_as_what_it_usually_is" => Some(py_func("_the_thing_is_named_as_what_it_usually_is", |args: &[Py]| _the_thing_is_named_as_what_it_usually_is(args))),
            "_the_verdict_is_inside" => Some(py_func("_the_verdict_is_inside", |args: &[Py]| _the_verdict_is_inside(args))),
            "_three_names" => Some(py_func("_three_names", |args: &[Py]| _three_names(args))),
            "_three_things_after_the_colon" => Some(py_func("_three_things_after_the_colon", |args: &[Py]| _three_things_after_the_colon(args))),
            "_trimmed" => Some(py_func("_trimmed", |args: &[Py]| _trimmed(args))),
            "_two_things_to_do" => Some(py_func("_two_things_to_do", |args: &[Py]| _two_things_to_do(args))),
            "_undecorated" => Some(py_func("_undecorated", |args: &[Py]| _undecorated(args))),
            "_uses_the_thing" => Some(py_func("_uses_the_thing", |args: &[Py]| _uses_the_thing(args))),
            "_wants_alone" => Some(py_func("_wants_alone", |args: &[Py]| _wants_alone(args))),
            "_whole" => Some(py_func("_whole", |args: &[Py]| _whole(args))),
            "_without_a_money_rate" => Some(py_func("_without_a_money_rate", |args: &[Py]| _without_a_money_rate(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "_words_handed_over" => Some(py_func("_words_handed_over", |args: &[Py]| _words_handed_over(args))),
            "asks_a_follow_up" => Some(py_func("asks_a_follow_up", |args: &[Py]| asks_a_follow_up(args))),
            "asks_for_words" => Some(py_func("asks_for_words", |args: &[Py]| asks_for_words(args))),
            "first_sentence" => Some(py_func("first_sentence", |args: &[Py]| first_sentence(args))),
            "fits" => Some(py_func("fits", |args: &[Py]| fits(args))),
            "gives" => Some(py_func("gives", |args: &[Py]| gives(args))),
            "is_a_punt" => Some(py_func("is_a_punt", |args: &[Py]| is_a_punt(args))),
            "is_a_question_back" => Some(py_func("is_a_question_back", |args: &[Py]| is_a_question_back(args))),
            "is_directed" => Some(py_func("is_directed", |args: &[Py]| is_directed(args))),
            "missing" => Some(py_func("missing", |args: &[Py]| missing(args))),
            "second_sentence" => Some(py_func("second_sentence", |args: &[Py]| second_sentence(args))),
            "wanted" => Some(py_func("wanted", |args: &[Py]| wanted(args))),
            "wants" => Some(py_func("wants", |args: &[Py]| wants(args))),
            _ => None,
        }),
        set: None,
    }))
}
