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


thread_local! { static G_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABOUT_ITSELF_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABOUT_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABOUT_TALLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABOUT_THEM_BEFORE_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABOUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABSENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACK_AFTER_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACK_WARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACTS_FOR_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_COOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_COOL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_LENGTH_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_WORSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFRAID_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_RESOLVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGREEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGREEMENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AIM_FAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AIM_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AIM_NEGATORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOLOGY_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_A_RATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_IF_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_IF_I_SAW_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OUT_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_IT_EXTRA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_IT_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_THE_TILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AXIS_MIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BAD_TIME_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BETRAYAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BIRTHDAY_STILL_TO_COME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BOOKING_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BOOKING_AHEAD_DATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_HOME_BRING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_HOME_GUILT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_HOME_MOVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_HOME_VISITING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD_LEAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CORRECTION_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COST_SOMEBODY_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CROWD_LAUGHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_AHEAD_TENSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_ASKED_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DISMISSAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FREE_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOODNESS_ASKED_OF_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOOD_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOOD_TIME_IN_A_CARE_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HEDGED_RATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOW_THE_TIME_WENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_SOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LAUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOVE_FOR_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOVE_FOR_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOOD_WITH_A_CATCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAMED_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAMED_AFTER_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_STORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEWS_OF_SOMETHING_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEWS_OF_WEATHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAUSE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_LAUGHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_I_DONT_KNOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN_THAT_FELL_THROUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECEIPT_OF_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REST_OF_A_GOOD_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REST_OF_A_HARD_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SKILL_ASKED_OF_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SLIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SLIP_IS_NOT_A_FALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SMALL_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SOCIAL_SLIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SORRY_ON_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUGGESTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUGGESTED_BY_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_BOUGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_CAME_OUT_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_LOOKS_SAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_SEEN_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_THEY_REGRET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT_ON_THE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAIT_ENDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WANT_IT_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAY_OF_BEING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY_FOOD_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_STRETCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_STRETCH_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FOLLOW_UP_CARE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FOLLOW_UP_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_AFTER_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_BRING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_EXPECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_GET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_LOOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_LOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_ORDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_PREPARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_SIGN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_TAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_WEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_WEAR_COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_WEAR_HOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_WEAR_WET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_WHICH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BECOMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BECOMING_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEHIND_ON_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLACKED_OUT_SIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_ALSO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_BUNGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_CLAMMY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_HOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_HUNGRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_IS_NOT_A_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_LIGHT_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PART_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PART_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PUFFY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_QUEASY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_SAYS_WHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_SITUATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_SPOTTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_STATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_THIRSTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_WORD_FOR_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOUGHT_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOUND_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURN_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURN_BLISTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUYING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALL_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAME_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD_VALIDITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_AFTER_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_AFTER_ANIMAL_STILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_AFTER_STILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_BETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_HURTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_INJURY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_NOT_RESOLVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_POINTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_PRONOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_RESOLVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_ROUTE_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_SEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_SEEN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_VISIT_WARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARING_FOR_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_AT_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_FAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_FAINT_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_BETWEEN_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_HEALTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_LIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_LIGHT_PICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_PHARMACIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_STAKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_THERAPY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_WEIGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSED_DOOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLD_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMFORT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPLAINT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPLAIN_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONGRATS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTACT_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_AFTER_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_BETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_BETTER_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROWD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DARK_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATES_KEEP_SLIPPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLINED_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLINED_PLAN_GENTLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENTIST_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETAILS_IN_A_WARM_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEVICE_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIE_OF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_PARTICIPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DITHER_ABOUT_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DITHER_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DITHER_OPEN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DITHER_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DITHER_TWO_WAY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_NOT_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_ONLY_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBLE_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EASING_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EATEN_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELDER_AID_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELDER_CLINIC_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELDER_DOCTOR_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELDER_HELP_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMBARRASSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMBARRASSMENT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY_BETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY_BETTER_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY_NOT_WORKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENDINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPERT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAINT_COMING_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FALLBACK_MODULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAVOURITE_GENERAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAVOURITE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLING_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLAT_CORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLAT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLOPPY_WAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_AFTER_SAFETY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_A_FALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_BODY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_DURATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_HE_SHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_ILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_I_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_OFF_THEIR_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_PERMISSION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_PET_HARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_SELF_STRUGGLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_SOMEONE_STRUGGLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORECAST_STRUGGLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORGETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORGET_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOUND_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FUEL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FUME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GADGET_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAS_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERIC_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_TO_DIE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GONE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GO_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GO_TO_BE_SEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREETING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREETING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREETING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEF_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_HARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_HARM_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_HEALTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_HEALTH_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_IMPACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_STRAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIEVANCE_STRAIN_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROWN_UP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT_PROBLEM_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HANDED_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HANDS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAZARD_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAZARD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_RECORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_RECORD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_WORLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAVY_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEDGE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELLO_STILL_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELLO_WITH_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELPLINE_NOT_AN_AMBULANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP_TOPIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP_TOPIC_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOLD_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOUSE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_ARE_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_ARE_YOU_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_ARE_YOU_GOING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_ARE_YOU_GOING_ON_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_ARE_YOU_TAGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_DECIDE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_DECIDE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HUMAN_EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDENTITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDIOM_GLAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDIOM_SHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_SAID_OF_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_WILL_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IMPERATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITED_TO_TALK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITE_ACTIVITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITE_NEVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITE_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_A_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_RATE_A_SKILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_ASKS_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_ASKS_MAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_ASKS_SLIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_BARE_FACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_FINE_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NOT_A_GERUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NOT_A_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NO_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NO_PLAN_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NO_PLAN_MAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_NO_PLAN_MIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_ONLY_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_PLAN_HAS_A_HOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_PLAN_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_SANE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_SANE_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_SELF_JUDGING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_SOUNDS_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_WISH_ANGRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_WORTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_WORTH_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUNK_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPS_FORGETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNOCKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNOCKED_SPOTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LANDED_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LANGUAGE_FLUENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LATE_BEHIND_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LATE_NOW_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGHED_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGHED_AT_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGHING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_BEHIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_POOL_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_DELAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_HOURS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_INDOOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_JOURNEY_ENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_NEWS_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_NOT_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_OPEN_CLOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_PAPER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_ROADS_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_ROAD_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_ROAD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_SETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_SITE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_SKY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_SKY_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_SKY_WHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_SOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_STRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_TAKE_ALLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_TAKE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_TRAVEL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_VENUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_VENUE_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_WEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_WEATHER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_YES_NO_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOCKED_OUT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOCKED_OUT_OF_THE_CAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOST_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOST_A_THING_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOW_DAY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LUMP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MANNERS_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEDS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEMORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MESS_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MILESTONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MILESTONE_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISHAP_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MODEL_NAMED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOOD_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOOD_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOUTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_ON_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MYSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_BILL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_OWN_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_PICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_PICK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_PICK_NOTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_RECORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_WORKINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_BODIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_NEGATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_TYPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NARROW_ANSWERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NARROW_SUBSTANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NARROW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATIVE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NETWORK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEUTRAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEVER_MIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_TO_COME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_AT_ALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_SAID_YET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_VERDICT_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_CHOSEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ENOUGH_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_FIT_ENOUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_NOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_PEOPLE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THEIR_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_TOO_OLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_WHAT_I_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_DIRECTION_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MORE_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_WORDS_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_WORDS_AT_THE_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_WORDS_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_WORDS_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFICE_RECORD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_SKILL_RATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONGOING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLINE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPINION_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTCOME_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASSED_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASSED_OUT_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASSED_OUT_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASSING_OUT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASSWORD_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_STATE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAUSE_IN_A_SAFETY_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PENDING_DOUBT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PENDING_LANDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PENDING_WAITING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PENDING_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEST_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PETROL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_CONDITION_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_LOW_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICKS_A_MECHANISM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_ONLY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_DONE_ACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_KNOCKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLASTER_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSITIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSITIVE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRAISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANCY_BLEEDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_LEFTOVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIVACY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIVACY_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIET_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_A_SKILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTIONS_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTIONS_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_DISBELIEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_DISBELIEF_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_MISHAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_MISHAP_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_OUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_OUCH_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_RELIEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_RELIEF_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_REPEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_REPEAT_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFUSED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATIONSHIP_DECISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELIEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_WHETHER_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REQUEST_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESOLVED_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESOLVED_AFTER_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETURNED_TO_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETURNED_TO_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETURNED_TO_IT_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RING_FOR_HELP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RING_THE_POLICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUMINATION_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_OUT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAFETY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_AGAIN_WITH_THE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCHOOL_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCHOOL_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCHOOL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECRETS_KEEP_SLIPPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEEN_TO_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_BLAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_BLAME_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_EATING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_NOBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SICKNESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEBODY_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSES_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSES_SHAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_LEAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_SERIOUSLY_ILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMETHING_CAME_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_AMBIGUOUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_HANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_ORDINARY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_SERIOUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORRY_FOR_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORRY_ON_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORTED_AND_NO_CALL_YET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_HERE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRETCH_OF_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STUCK_AFTER_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUDDEN_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUMMONS_RANG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUMMONS_SEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUMMONS_WRITTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM_COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_CONFIRM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_UNSURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TASTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TASTE_GENERAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELL_HARD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TENANT_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_THEN_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BOTS_OWN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_CALL_STANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_CALL_STANDS_ANYWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DAY_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FIRST_HOUR_AND_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_GROUND_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_MOMENT_IS_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PLACE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PLACE_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_STATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAD_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAD_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THRESHOLD_CROSSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIGHT_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIGHT_MONEY_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_OWNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIRED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_MUCH_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADES_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADE_OFF_LEAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TREATMENT_HARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_HELPED_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_HELPED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_IT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_NO_CHANGE_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_NO_CHANGE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_SORTED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_WORSE_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_WORSE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TROUBLE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_ABOUT_A_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_ABOUT_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_HEALTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_REALISTIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_REALISTIC_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_TOO_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WEIRD_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WORD_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WORRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_ANNOY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_LUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_NUMB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_QUEUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_ROUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_SETBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_SHAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VET_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_AFTER_A_DEATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_ASK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_COMPANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_HANDBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_MIRROR_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_ONLY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_ON_GOOD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_PRONOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WALKED_INTO_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WASNT_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_SOMETHING_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_LIST_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_TO_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_AFTER_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WILD_BIRDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WIN_AND_A_COST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WIN_IN_A_SILENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_YOU_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORDING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORDING_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_FIXES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORRY_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORRY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WOULD_HAVE_BEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YOU_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_ABOUT_THE_JOINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SOCIAL_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BOUNDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHOOSING_BETWEEN_TWO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CREATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ECHO_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ISSUER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LANDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AN_OWN_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_DAY_OF_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENING_QUESTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__START_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_AFTER_AM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THING_DONE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THREAD_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERDICT_DET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERDICT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WOUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT() -> Py { ensure_init(); G_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT(v: Py) { ensure_init(); G_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_ABOUT_ITSELF_KINDS() -> Py { ensure_init(); G_ABOUT_ITSELF_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_ITSELF_KINDS(v: Py) { ensure_init(); G_ABOUT_ITSELF_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ABOUT_SELF() -> Py { ensure_init(); G_ABOUT_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_SELF(v: Py) { ensure_init(); G_ABOUT_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_ABOUT_TALLY() -> Py { ensure_init(); G_ABOUT_TALLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_TALLY(v: Py) { ensure_init(); G_ABOUT_TALLY.with(|c| *c.borrow_mut() = v); }
pub fn g_ABOUT_THEM_BEFORE_COPULA() -> Py { ensure_init(); G_ABOUT_THEM_BEFORE_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THEM_BEFORE_COPULA(v: Py) { ensure_init(); G_ABOUT_THEM_BEFORE_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_ABOUT_WORDS() -> Py { ensure_init(); G_ABOUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_WORDS(v: Py) { ensure_init(); G_ABOUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ABSENCE() -> Py { ensure_init(); G_ABSENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABSENCE(v: Py) { ensure_init(); G_ABSENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_AFTER() -> Py { ensure_init(); G_ABUSE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_AFTER(v: Py) { ensure_init(); G_ABUSE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_ACK() -> Py { ensure_init(); G_ACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK(v: Py) { ensure_init(); G_ACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ACKS() -> Py { ensure_init(); G_ACKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACKS(v: Py) { ensure_init(); G_ACKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ACK_AFTER_TROUBLE() -> Py { ensure_init(); G_ACK_AFTER_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK_AFTER_TROUBLE(v: Py) { ensure_init(); G_ACK_AFTER_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_ACK_WARM() -> Py { ensure_init(); G_ACK_WARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK_WARM(v: Py) { ensure_init(); G_ACK_WARM.with(|c| *c.borrow_mut() = v); }
pub fn g_ACTS_FOR_ME() -> Py { ensure_init(); G_ACTS_FOR_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACTS_FOR_ME(v: Py) { ensure_init(); G_ACTS_FOR_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_AGAIN() -> Py { ensure_init(); G_ADVICE_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_AGAIN(v: Py) { ensure_init(); G_ADVICE_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_COOL() -> Py { ensure_init(); G_ADVICE_COOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_COOL(v: Py) { ensure_init(); G_ADVICE_COOL.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_COOL_WORDS() -> Py { ensure_init(); G_ADVICE_COOL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_COOL_WORDS(v: Py) { ensure_init(); G_ADVICE_COOL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_LENGTH() -> Py { ensure_init(); G_ADVICE_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_LENGTH(v: Py) { ensure_init(); G_ADVICE_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_LENGTH_BARE() -> Py { ensure_init(); G_ADVICE_LENGTH_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_LENGTH_BARE(v: Py) { ensure_init(); G_ADVICE_LENGTH_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_TIME() -> Py { ensure_init(); G_ADVICE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_TIME(v: Py) { ensure_init(); G_ADVICE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_TURNS() -> Py { ensure_init(); G_ADVICE_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_TURNS(v: Py) { ensure_init(); G_ADVICE_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_WORDS() -> Py { ensure_init(); G_ADVICE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_WORDS(v: Py) { ensure_init(); G_ADVICE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_WORSE() -> Py { ensure_init(); G_ADVICE_WORSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_WORSE(v: Py) { ensure_init(); G_ADVICE_WORSE.with(|c| *c.borrow_mut() = v); }
pub fn g_AFRAID_AFTER() -> Py { ensure_init(); G_AFRAID_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFRAID_AFTER(v: Py) { ensure_init(); G_AFRAID_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_RESOLVED() -> Py { ensure_init(); G_AFTER_RESOLVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_RESOLVED(v: Py) { ensure_init(); G_AFTER_RESOLVED.with(|c| *c.borrow_mut() = v); }
pub fn g_AGREEMENT() -> Py { ensure_init(); G_AGREEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREEMENT(v: Py) { ensure_init(); G_AGREEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_AGREEMENTS() -> Py { ensure_init(); G_AGREEMENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREEMENTS(v: Py) { ensure_init(); G_AGREEMENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_AIM_FAILING() -> Py { ensure_init(); G_AIM_FAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIM_FAILING(v: Py) { ensure_init(); G_AIM_FAILING.with(|c| *c.borrow_mut() = v); }
pub fn g_AIM_HEADS() -> Py { ensure_init(); G_AIM_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIM_HEADS(v: Py) { ensure_init(); G_AIM_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_AIM_NEGATORS() -> Py { ensure_init(); G_AIM_NEGATORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIM_NEGATORS(v: Py) { ensure_init(); G_AIM_NEGATORS.with(|c| *c.borrow_mut() = v); }
pub fn g_ALIVE() -> Py { ensure_init(); G_ALIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALIVE(v: Py) { ensure_init(); G_ALIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMALS() -> Py { ensure_init(); G_ANIMALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMALS(v: Py) { ensure_init(); G_ANIMALS.with(|c| *c.borrow_mut() = v); }
pub fn g_APOLOGY_ANSWER() -> Py { ensure_init(); G_APOLOGY_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOLOGY_ANSWER(v: Py) { ensure_init(); G_APOLOGY_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_A_RATING() -> Py { ensure_init(); G_ASKED_A_RATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_A_RATING(v: Py) { ensure_init(); G_ASKED_A_RATING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_IF_GOOD() -> Py { ensure_init(); G_ASKED_IF_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_IF_GOOD(v: Py) { ensure_init(); G_ASKED_IF_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_IF_I_SAW_LINES() -> Py { ensure_init(); G_ASKED_IF_I_SAW_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_IF_I_SAW_LINES(v: Py) { ensure_init(); G_ASKED_IF_I_SAW_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_WORDS() -> Py { ensure_init(); G_ASKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_WORDS(v: Py) { ensure_init(); G_ASKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_ONLY() -> Py { ensure_init(); G_ASK_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_ONLY(v: Py) { ensure_init(); G_ASK_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OUT_ANSWER() -> Py { ensure_init(); G_ASK_OUT_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OUT_ANSWER(v: Py) { ensure_init(); G_ASK_OUT_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_IT_EXTRA() -> Py { ensure_init(); G_AT_IT_EXTRA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_IT_EXTRA(v: Py) { ensure_init(); G_AT_IT_EXTRA.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_IT_NOT() -> Py { ensure_init(); G_AT_IT_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_IT_NOT(v: Py) { ensure_init(); G_AT_IT_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_THE_TILL() -> Py { ensure_init(); G_AT_THE_TILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_THE_TILL(v: Py) { ensure_init(); G_AT_THE_TILL.with(|c| *c.borrow_mut() = v); }
pub fn g_AXIS_MIN() -> Py { ensure_init(); G_AXIS_MIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AXIS_MIN(v: Py) { ensure_init(); G_AXIS_MIN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BAD_TIME_TOLD() -> Py { ensure_init(); G_A_BAD_TIME_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BAD_TIME_TOLD(v: Py) { ensure_init(); G_A_BAD_TIME_TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BETRAYAL() -> Py { ensure_init(); G_A_BETRAYAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BETRAYAL(v: Py) { ensure_init(); G_A_BETRAYAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BIRTHDAY_STILL_TO_COME() -> Py { ensure_init(); G_A_BIRTHDAY_STILL_TO_COME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BIRTHDAY_STILL_TO_COME(v: Py) { ensure_init(); G_A_BIRTHDAY_STILL_TO_COME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BOOKING_AHEAD() -> Py { ensure_init(); G_A_BOOKING_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BOOKING_AHEAD(v: Py) { ensure_init(); G_A_BOOKING_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BOOKING_AHEAD_DATED() -> Py { ensure_init(); G_A_BOOKING_AHEAD_DATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BOOKING_AHEAD_DATED(v: Py) { ensure_init(); G_A_BOOKING_AHEAD_DATED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_HOME() -> Py { ensure_init(); G_A_CARE_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_HOME(v: Py) { ensure_init(); G_A_CARE_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_HOME_BRING() -> Py { ensure_init(); G_A_CARE_HOME_BRING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_HOME_BRING(v: Py) { ensure_init(); G_A_CARE_HOME_BRING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_HOME_GUILT() -> Py { ensure_init(); G_A_CARE_HOME_GUILT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_HOME_GUILT(v: Py) { ensure_init(); G_A_CARE_HOME_GUILT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_HOME_MOVED() -> Py { ensure_init(); G_A_CARE_HOME_MOVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_HOME_MOVED(v: Py) { ensure_init(); G_A_CARE_HOME_MOVED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_HOME_VISITING() -> Py { ensure_init(); G_A_CARE_HOME_VISITING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_HOME_VISITING(v: Py) { ensure_init(); G_A_CARE_HOME_VISITING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD_LEAVING() -> Py { ensure_init(); G_A_CHILD_LEAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_LEAVING(v: Py) { ensure_init(); G_A_CHILD_LEAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD_LEFT() -> Py { ensure_init(); G_A_CHILD_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_LEFT(v: Py) { ensure_init(); G_A_CHILD_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CORRECTION_TAKEN() -> Py { ensure_init(); G_A_CORRECTION_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CORRECTION_TAKEN(v: Py) { ensure_init(); G_A_CORRECTION_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COST_SOMEBODY_ASKS() -> Py { ensure_init(); G_A_COST_SOMEBODY_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COST_SOMEBODY_ASKS(v: Py) { ensure_init(); G_A_COST_SOMEBODY_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CROWD_LAUGHED() -> Py { ensure_init(); G_A_CROWD_LAUGHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CROWD_LAUGHED(v: Py) { ensure_init(); G_A_CROWD_LAUGHED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_AHEAD() -> Py { ensure_init(); G_A_DAY_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_AHEAD(v: Py) { ensure_init(); G_A_DAY_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_AHEAD_TENSE() -> Py { ensure_init(); G_A_DAY_AHEAD_TENSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_AHEAD_TENSE(v: Py) { ensure_init(); G_A_DAY_AHEAD_TENSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_ASKED_PLAIN() -> Py { ensure_init(); G_A_DAY_ASKED_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_ASKED_PLAIN(v: Py) { ensure_init(); G_A_DAY_ASKED_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DISMISSAL() -> Py { ensure_init(); G_A_DISMISSAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DISMISSAL(v: Py) { ensure_init(); G_A_DISMISSAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DREAD() -> Py { ensure_init(); G_A_DREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DREAD(v: Py) { ensure_init(); G_A_DREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FREE_TIME_ASK() -> Py { ensure_init(); G_A_FREE_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FREE_TIME_ASK(v: Py) { ensure_init(); G_A_FREE_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOODNESS_ASKED_OF_ME() -> Py { ensure_init(); G_A_GOODNESS_ASKED_OF_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOODNESS_ASKED_OF_ME(v: Py) { ensure_init(); G_A_GOODNESS_ASKED_OF_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOOD_TIME() -> Py { ensure_init(); G_A_GOOD_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_TIME(v: Py) { ensure_init(); G_A_GOOD_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOOD_TIME_IN_A_CARE_THREAD() -> Py { ensure_init(); G_A_GOOD_TIME_IN_A_CARE_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_TIME_IN_A_CARE_THREAD(v: Py) { ensure_init(); G_A_GOOD_TIME_IN_A_CARE_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HEDGED_RATING() -> Py { ensure_init(); G_A_HEDGED_RATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HEDGED_RATING(v: Py) { ensure_init(); G_A_HEDGED_RATING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOW_THE_TIME_WENT() -> Py { ensure_init(); G_A_HOW_THE_TIME_WENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOW_THE_TIME_WENT(v: Py) { ensure_init(); G_A_HOW_THE_TIME_WENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_ON() -> Py { ensure_init(); G_A_JOB_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_ON(v: Py) { ensure_init(); G_A_JOB_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_SOON() -> Py { ensure_init(); G_A_JOB_SOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_SOON(v: Py) { ensure_init(); G_A_JOB_SOON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LAUGH() -> Py { ensure_init(); G_A_LAUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LAUGH(v: Py) { ensure_init(); G_A_LAUGH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOVE_FOR_PEOPLE() -> Py { ensure_init(); G_A_LOVE_FOR_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOVE_FOR_PEOPLE(v: Py) { ensure_init(); G_A_LOVE_FOR_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOVE_FOR_SOMEONE() -> Py { ensure_init(); G_A_LOVE_FOR_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOVE_FOR_SOMEONE(v: Py) { ensure_init(); G_A_LOVE_FOR_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOOD_WITH_A_CATCH() -> Py { ensure_init(); G_A_MOOD_WITH_A_CATCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOOD_WITH_A_CATCH(v: Py) { ensure_init(); G_A_MOOD_WITH_A_CATCH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAMED_AFTER() -> Py { ensure_init(); G_A_NAMED_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_AFTER(v: Py) { ensure_init(); G_A_NAMED_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAMED_AFTER_THEM() -> Py { ensure_init(); G_A_NAMED_AFTER_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_AFTER_THEM(v: Py) { ensure_init(); G_A_NAMED_AFTER_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_STORY() -> Py { ensure_init(); G_A_NAME_STORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_STORY(v: Py) { ensure_init(); G_A_NAME_STORY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_TOLD() -> Py { ensure_init(); G_A_NAME_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_TOLD(v: Py) { ensure_init(); G_A_NAME_TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEWS_OF_SOMETHING_ELSE() -> Py { ensure_init(); G_A_NEWS_OF_SOMETHING_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEWS_OF_SOMETHING_ELSE(v: Py) { ensure_init(); G_A_NEWS_OF_SOMETHING_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEWS_OF_WEATHER() -> Py { ensure_init(); G_A_NEWS_OF_WEATHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEWS_OF_WEATHER(v: Py) { ensure_init(); G_A_NEWS_OF_WEATHER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAUSE_WORD() -> Py { ensure_init(); G_A_PAUSE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAUSE_WORD(v: Py) { ensure_init(); G_A_PAUSE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_LAUGHED() -> Py { ensure_init(); G_A_PERSON_LAUGHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_LAUGHED(v: Py) { ensure_init(); G_A_PERSON_LAUGHED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_I_DONT_KNOW() -> Py { ensure_init(); G_A_PLACE_I_DONT_KNOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_I_DONT_KNOW(v: Py) { ensure_init(); G_A_PLACE_I_DONT_KNOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN_THAT_FELL_THROUGH() -> Py { ensure_init(); G_A_PLAN_THAT_FELL_THROUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_THAT_FELL_THROUGH(v: Py) { ensure_init(); G_A_PLAN_THAT_FELL_THROUGH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECEIPT_OF_NOTHING() -> Py { ensure_init(); G_A_RECEIPT_OF_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECEIPT_OF_NOTHING(v: Py) { ensure_init(); G_A_RECEIPT_OF_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REST_OF_A_GOOD_DAY() -> Py { ensure_init(); G_A_REST_OF_A_GOOD_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REST_OF_A_GOOD_DAY(v: Py) { ensure_init(); G_A_REST_OF_A_GOOD_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REST_OF_A_HARD_DAY() -> Py { ensure_init(); G_A_REST_OF_A_HARD_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REST_OF_A_HARD_DAY(v: Py) { ensure_init(); G_A_REST_OF_A_HARD_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SKILL_ASKED_OF_ME() -> Py { ensure_init(); G_A_SKILL_ASKED_OF_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SKILL_ASKED_OF_ME(v: Py) { ensure_init(); G_A_SKILL_ASKED_OF_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SLIP() -> Py { ensure_init(); G_A_SLIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SLIP(v: Py) { ensure_init(); G_A_SLIP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SLIP_IS_NOT_A_FALL() -> Py { ensure_init(); G_A_SLIP_IS_NOT_A_FALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SLIP_IS_NOT_A_FALL(v: Py) { ensure_init(); G_A_SLIP_IS_NOT_A_FALL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SMALL_DAY() -> Py { ensure_init(); G_A_SMALL_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SMALL_DAY(v: Py) { ensure_init(); G_A_SMALL_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SOCIAL_SLIP() -> Py { ensure_init(); G_A_SOCIAL_SLIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SOCIAL_SLIP(v: Py) { ensure_init(); G_A_SOCIAL_SLIP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SORRY_ON_ITS_OWN() -> Py { ensure_init(); G_A_SORRY_ON_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SORRY_ON_ITS_OWN(v: Py) { ensure_init(); G_A_SORRY_ON_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE_NOW() -> Py { ensure_init(); G_A_STATE_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE_NOW(v: Py) { ensure_init(); G_A_STATE_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE_PAST() -> Py { ensure_init(); G_A_STATE_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE_PAST(v: Py) { ensure_init(); G_A_STATE_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUGGESTED() -> Py { ensure_init(); G_A_SUGGESTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUGGESTED(v: Py) { ensure_init(); G_A_SUGGESTED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUGGESTED_BY_SOMEONE() -> Py { ensure_init(); G_A_SUGGESTED_BY_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUGGESTED_BY_SOMEONE(v: Py) { ensure_init(); G_A_SUGGESTED_BY_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_AHEAD() -> Py { ensure_init(); G_A_THING_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_AHEAD(v: Py) { ensure_init(); G_A_THING_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_BOUGHT() -> Py { ensure_init(); G_A_THING_BOUGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_BOUGHT(v: Py) { ensure_init(); G_A_THING_BOUGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_CAME_OUT_WRONG() -> Py { ensure_init(); G_A_THING_CAME_OUT_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_CAME_OUT_WRONG(v: Py) { ensure_init(); G_A_THING_CAME_OUT_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_DONE() -> Py { ensure_init(); G_A_THING_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_DONE(v: Py) { ensure_init(); G_A_THING_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_GONE() -> Py { ensure_init(); G_A_THING_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_GONE(v: Py) { ensure_init(); G_A_THING_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_LOOKS_SAD() -> Py { ensure_init(); G_A_THING_LOOKS_SAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_LOOKS_SAD(v: Py) { ensure_init(); G_A_THING_LOOKS_SAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_ON() -> Py { ensure_init(); G_A_THING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_ON(v: Py) { ensure_init(); G_A_THING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_SEEN_TO() -> Py { ensure_init(); G_A_THING_SEEN_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_SEEN_TO(v: Py) { ensure_init(); G_A_THING_SEEN_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_TAKEN() -> Py { ensure_init(); G_A_THING_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_TAKEN(v: Py) { ensure_init(); G_A_THING_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_THEY_REGRET() -> Py { ensure_init(); G_A_THING_THEY_REGRET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_THEY_REGRET(v: Py) { ensure_init(); G_A_THING_THEY_REGRET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT_ON_THE_PERSON() -> Py { ensure_init(); G_A_VERDICT_ON_THE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT_ON_THE_PERSON(v: Py) { ensure_init(); G_A_VERDICT_ON_THE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAIT_ENDING() -> Py { ensure_init(); G_A_WAIT_ENDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT_ENDING(v: Py) { ensure_init(); G_A_WAIT_ENDING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WANT_IT_OVER() -> Py { ensure_init(); G_A_WANT_IT_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WANT_IT_OVER(v: Py) { ensure_init(); G_A_WANT_IT_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAY_OF_BEING() -> Py { ensure_init(); G_A_WAY_OF_BEING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAY_OF_BEING(v: Py) { ensure_init(); G_A_WAY_OF_BEING.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY_FOOD_NAMES() -> Py { ensure_init(); G_BABY_FOOD_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY_FOOD_NAMES(v: Py) { ensure_init(); G_BABY_FOOD_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY_WHO() -> Py { ensure_init(); G_BABY_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY_WHO(v: Py) { ensure_init(); G_BABY_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_STRETCH() -> Py { ensure_init(); G_BAD_STRETCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_STRETCH(v: Py) { ensure_init(); G_BAD_STRETCH.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_STRETCH_REPLIES() -> Py { ensure_init(); G_BAD_STRETCH_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_STRETCH_REPLIES(v: Py) { ensure_init(); G_BAD_STRETCH_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FOLLOW_UP_CARE_REPLIES() -> Py { ensure_init(); G_BARE_FOLLOW_UP_CARE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FOLLOW_UP_CARE_REPLIES(v: Py) { ensure_init(); G_BARE_FOLLOW_UP_CARE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FOLLOW_UP_REPLIES() -> Py { ensure_init(); G_BARE_FOLLOW_UP_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FOLLOW_UP_REPLIES(v: Py) { ensure_init(); G_BARE_FOLLOW_UP_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_AFTER_NEWS() -> Py { ensure_init(); G_BARE_PICK_AFTER_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_AFTER_NEWS(v: Py) { ensure_init(); G_BARE_PICK_AFTER_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_ASK() -> Py { ensure_init(); G_BARE_PICK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_ASK(v: Py) { ensure_init(); G_BARE_PICK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_BRING() -> Py { ensure_init(); G_BARE_PICK_BRING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_BRING(v: Py) { ensure_init(); G_BARE_PICK_BRING.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_DO() -> Py { ensure_init(); G_BARE_PICK_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_DO(v: Py) { ensure_init(); G_BARE_PICK_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_EXPECT() -> Py { ensure_init(); G_BARE_PICK_EXPECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_EXPECT(v: Py) { ensure_init(); G_BARE_PICK_EXPECT.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_GET() -> Py { ensure_init(); G_BARE_PICK_GET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_GET(v: Py) { ensure_init(); G_BARE_PICK_GET.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_KIND() -> Py { ensure_init(); G_BARE_PICK_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_KIND(v: Py) { ensure_init(); G_BARE_PICK_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_LOOK() -> Py { ensure_init(); G_BARE_PICK_LOOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_LOOK(v: Py) { ensure_init(); G_BARE_PICK_LOOK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_LOST() -> Py { ensure_init(); G_BARE_PICK_LOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_LOST(v: Py) { ensure_init(); G_BARE_PICK_LOST.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_ORDER() -> Py { ensure_init(); G_BARE_PICK_ORDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_ORDER(v: Py) { ensure_init(); G_BARE_PICK_ORDER.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_PREPARE() -> Py { ensure_init(); G_BARE_PICK_PREPARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_PREPARE(v: Py) { ensure_init(); G_BARE_PICK_PREPARE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_SIGN() -> Py { ensure_init(); G_BARE_PICK_SIGN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_SIGN(v: Py) { ensure_init(); G_BARE_PICK_SIGN.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_TAKE() -> Py { ensure_init(); G_BARE_PICK_TAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_TAKE(v: Py) { ensure_init(); G_BARE_PICK_TAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_WEAR() -> Py { ensure_init(); G_BARE_PICK_WEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_WEAR(v: Py) { ensure_init(); G_BARE_PICK_WEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_WEAR_COLD() -> Py { ensure_init(); G_BARE_PICK_WEAR_COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_WEAR_COLD(v: Py) { ensure_init(); G_BARE_PICK_WEAR_COLD.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_WEAR_HOT() -> Py { ensure_init(); G_BARE_PICK_WEAR_HOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_WEAR_HOT(v: Py) { ensure_init(); G_BARE_PICK_WEAR_HOT.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_WEAR_WET() -> Py { ensure_init(); G_BARE_PICK_WEAR_WET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_WEAR_WET(v: Py) { ensure_init(); G_BARE_PICK_WEAR_WET.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_WHICH() -> Py { ensure_init(); G_BARE_PICK_WHICH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_WHICH(v: Py) { ensure_init(); G_BARE_PICK_WHICH.with(|c| *c.borrow_mut() = v); }
pub fn g_BECOMING() -> Py { ensure_init(); G_BECOMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BECOMING(v: Py) { ensure_init(); G_BECOMING.with(|c| *c.borrow_mut() = v); }
pub fn g_BECOMING_STATE() -> Py { ensure_init(); G_BECOMING_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BECOMING_STATE(v: Py) { ensure_init(); G_BECOMING_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_BEHIND_ON_MONEY() -> Py { ensure_init(); G_BEHIND_ON_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEHIND_ON_MONEY(v: Py) { ensure_init(); G_BEHIND_ON_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_BLACKED_OUT_SIGHT() -> Py { ensure_init(); G_BLACKED_OUT_SIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLACKED_OUT_SIGHT(v: Py) { ensure_init(); G_BLACKED_OUT_SIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_ALSO() -> Py { ensure_init(); G_BODY_ALSO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_ALSO(v: Py) { ensure_init(); G_BODY_ALSO.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_BUNGED() -> Py { ensure_init(); G_BODY_BUNGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_BUNGED(v: Py) { ensure_init(); G_BODY_BUNGED.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_CLAMMY() -> Py { ensure_init(); G_BODY_CLAMMY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_CLAMMY(v: Py) { ensure_init(); G_BODY_CLAMMY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_COLD() -> Py { ensure_init(); G_BODY_COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_COLD(v: Py) { ensure_init(); G_BODY_COLD.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_HOT() -> Py { ensure_init(); G_BODY_HOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_HOT(v: Py) { ensure_init(); G_BODY_HOT.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_HUNGRY() -> Py { ensure_init(); G_BODY_HUNGRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_HUNGRY(v: Py) { ensure_init(); G_BODY_HUNGRY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_IS_NOT_A_MACHINE() -> Py { ensure_init(); G_BODY_IS_NOT_A_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_IS_NOT_A_MACHINE(v: Py) { ensure_init(); G_BODY_IS_NOT_A_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_LEAD() -> Py { ensure_init(); G_BODY_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_LEAD(v: Py) { ensure_init(); G_BODY_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_LIGHT_HEAD() -> Py { ensure_init(); G_BODY_LIGHT_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_LIGHT_HEAD(v: Py) { ensure_init(); G_BODY_LIGHT_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PALE() -> Py { ensure_init(); G_BODY_PALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PALE(v: Py) { ensure_init(); G_BODY_PALE.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PARTS() -> Py { ensure_init(); G_BODY_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PARTS(v: Py) { ensure_init(); G_BODY_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PART_QUALIFIER() -> Py { ensure_init(); G_BODY_PART_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PART_QUALIFIER(v: Py) { ensure_init(); G_BODY_PART_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PART_TAIL() -> Py { ensure_init(); G_BODY_PART_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PART_TAIL(v: Py) { ensure_init(); G_BODY_PART_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_POOLS() -> Py { ensure_init(); G_BODY_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_POOLS(v: Py) { ensure_init(); G_BODY_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PUFFY() -> Py { ensure_init(); G_BODY_PUFFY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PUFFY(v: Py) { ensure_init(); G_BODY_PUFFY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_QUEASY() -> Py { ensure_init(); G_BODY_QUEASY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_QUEASY(v: Py) { ensure_init(); G_BODY_QUEASY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_SAYS_WHY() -> Py { ensure_init(); G_BODY_SAYS_WHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_SAYS_WHY(v: Py) { ensure_init(); G_BODY_SAYS_WHY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_SITUATION() -> Py { ensure_init(); G_BODY_SITUATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_SITUATION(v: Py) { ensure_init(); G_BODY_SITUATION.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_SPOTTY() -> Py { ensure_init(); G_BODY_SPOTTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_SPOTTY(v: Py) { ensure_init(); G_BODY_SPOTTY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_STATES() -> Py { ensure_init(); G_BODY_STATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_STATES(v: Py) { ensure_init(); G_BODY_STATES.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_SUBJECT() -> Py { ensure_init(); G_BODY_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_SUBJECT(v: Py) { ensure_init(); G_BODY_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_TAIL() -> Py { ensure_init(); G_BODY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_TAIL(v: Py) { ensure_init(); G_BODY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_THIRSTY() -> Py { ensure_init(); G_BODY_THIRSTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_THIRSTY(v: Py) { ensure_init(); G_BODY_THIRSTY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_WORD_FOR_A_THING() -> Py { ensure_init(); G_BODY_WORD_FOR_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_WORD_FOR_A_THING(v: Py) { ensure_init(); G_BODY_WORD_FOR_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_BOUGHT_VERB() -> Py { ensure_init(); G_BOUGHT_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOUGHT_VERB(v: Py) { ensure_init(); G_BOUGHT_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_BOUND_FOR() -> Py { ensure_init(); G_BOUND_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOUND_FOR(v: Py) { ensure_init(); G_BOUND_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_BURN_AFTER() -> Py { ensure_init(); G_BURN_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURN_AFTER(v: Py) { ensure_init(); G_BURN_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_BURN_BLISTER() -> Py { ensure_init(); G_BURN_BLISTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURN_BLISTER(v: Py) { ensure_init(); G_BURN_BLISTER.with(|c| *c.borrow_mut() = v); }
pub fn g_BUYING_WORDS() -> Py { ensure_init(); G_BUYING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUYING_WORDS(v: Py) { ensure_init(); G_BUYING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CALL_ANSWER() -> Py { ensure_init(); G_CALL_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALL_ANSWER(v: Py) { ensure_init(); G_CALL_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_CALL_WORDS() -> Py { ensure_init(); G_CALL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALL_WORDS(v: Py) { ensure_init(); G_CALL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAME_BACK() -> Py { ensure_init(); G_CAME_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAME_BACK(v: Py) { ensure_init(); G_CAME_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD_LINES() -> Py { ensure_init(); G_CARD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD_LINES(v: Py) { ensure_init(); G_CARD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD_VALIDITY() -> Py { ensure_init(); G_CARD_VALIDITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD_VALIDITY(v: Py) { ensure_init(); G_CARD_VALIDITY.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_AFTER() -> Py { ensure_init(); G_CARE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_AFTER(v: Py) { ensure_init(); G_CARE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_AFTER_ANIMAL() -> Py { ensure_init(); G_CARE_AFTER_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_AFTER_ANIMAL(v: Py) { ensure_init(); G_CARE_AFTER_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_AFTER_ANIMAL_STILL() -> Py { ensure_init(); G_CARE_AFTER_ANIMAL_STILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_AFTER_ANIMAL_STILL(v: Py) { ensure_init(); G_CARE_AFTER_ANIMAL_STILL.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_AFTER_STILL() -> Py { ensure_init(); G_CARE_AFTER_STILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_AFTER_STILL(v: Py) { ensure_init(); G_CARE_AFTER_STILL.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_BETTER() -> Py { ensure_init(); G_CARE_BETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_BETTER(v: Py) { ensure_init(); G_CARE_BETTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_HERE() -> Py { ensure_init(); G_CARE_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_HERE(v: Py) { ensure_init(); G_CARE_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_HURTS() -> Py { ensure_init(); G_CARE_HURTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_HURTS(v: Py) { ensure_init(); G_CARE_HURTS.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_INJURY() -> Py { ensure_init(); G_CARE_INJURY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_INJURY(v: Py) { ensure_init(); G_CARE_INJURY.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_MODULES() -> Py { ensure_init(); G_CARE_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_MODULES(v: Py) { ensure_init(); G_CARE_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_NOT_RESOLVED() -> Py { ensure_init(); G_CARE_NOT_RESOLVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_NOT_RESOLVED(v: Py) { ensure_init(); G_CARE_NOT_RESOLVED.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_PHRASES() -> Py { ensure_init(); G_CARE_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_PHRASES(v: Py) { ensure_init(); G_CARE_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_POINTER() -> Py { ensure_init(); G_CARE_POINTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_POINTER(v: Py) { ensure_init(); G_CARE_POINTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_PRONOUNS() -> Py { ensure_init(); G_CARE_PRONOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_PRONOUNS(v: Py) { ensure_init(); G_CARE_PRONOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_RESOLVED() -> Py { ensure_init(); G_CARE_RESOLVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_RESOLVED(v: Py) { ensure_init(); G_CARE_RESOLVED.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_ROUTE_BACK() -> Py { ensure_init(); G_CARE_ROUTE_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_ROUTE_BACK(v: Py) { ensure_init(); G_CARE_ROUTE_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_SEEN() -> Py { ensure_init(); G_CARE_SEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_SEEN(v: Py) { ensure_init(); G_CARE_SEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_SEEN_ANIMAL() -> Py { ensure_init(); G_CARE_SEEN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_SEEN_ANIMAL(v: Py) { ensure_init(); G_CARE_SEEN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_THREAD() -> Py { ensure_init(); G_CARE_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_THREAD(v: Py) { ensure_init(); G_CARE_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_VISIT() -> Py { ensure_init(); G_CARE_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_VISIT(v: Py) { ensure_init(); G_CARE_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_VISIT_WARD() -> Py { ensure_init(); G_CARE_VISIT_WARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_VISIT_WARD(v: Py) { ensure_init(); G_CARE_VISIT_WARD.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_WORDS() -> Py { ensure_init(); G_CARE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_WORDS(v: Py) { ensure_init(); G_CARE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CARING() -> Py { ensure_init(); G_CARING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARING(v: Py) { ensure_init(); G_CARING.with(|c| *c.borrow_mut() = v); }
pub fn g_CARING_FOR_A_BODY() -> Py { ensure_init(); G_CARING_FOR_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARING_FOR_A_BODY(v: Py) { ensure_init(); G_CARING_FOR_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_PHRASES() -> Py { ensure_init(); G_CHECK_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_PHRASES(v: Py) { ensure_init(); G_CHECK_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_AT_HOME() -> Py { ensure_init(); G_CHILD_AT_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_AT_HOME(v: Py) { ensure_init(); G_CHILD_AT_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_FAINT() -> Py { ensure_init(); G_CHILD_FAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_FAINT(v: Py) { ensure_init(); G_CHILD_FAINT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_FAINT_ANSWER() -> Py { ensure_init(); G_CHILD_FAINT_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_FAINT_ANSWER(v: Py) { ensure_init(); G_CHILD_FAINT_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_BETWEEN_PEOPLE() -> Py { ensure_init(); G_CHOICE_BETWEEN_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_BETWEEN_PEOPLE(v: Py) { ensure_init(); G_CHOICE_BETWEEN_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_HEALTH() -> Py { ensure_init(); G_CHOICE_HEALTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_HEALTH(v: Py) { ensure_init(); G_CHOICE_HEALTH.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_LIGHT() -> Py { ensure_init(); G_CHOICE_LIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_LIGHT(v: Py) { ensure_init(); G_CHOICE_LIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_LIGHT_PICK() -> Py { ensure_init(); G_CHOICE_LIGHT_PICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_LIGHT_PICK(v: Py) { ensure_init(); G_CHOICE_LIGHT_PICK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_PEOPLE() -> Py { ensure_init(); G_CHOICE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_PEOPLE(v: Py) { ensure_init(); G_CHOICE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_PHARMACIST() -> Py { ensure_init(); G_CHOICE_PHARMACIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_PHARMACIST(v: Py) { ensure_init(); G_CHOICE_PHARMACIST.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_STAKES() -> Py { ensure_init(); G_CHOICE_STAKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_STAKES(v: Py) { ensure_init(); G_CHOICE_STAKES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_THERAPY() -> Py { ensure_init(); G_CHOICE_THERAPY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_THERAPY(v: Py) { ensure_init(); G_CHOICE_THERAPY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_WEIGH() -> Py { ensure_init(); G_CHOICE_WEIGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_WEIGH(v: Py) { ensure_init(); G_CHOICE_WEIGH.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSED_DOOR() -> Py { ensure_init(); G_CLOSED_DOOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSED_DOOR(v: Py) { ensure_init(); G_CLOSED_DOOR.with(|c| *c.borrow_mut() = v); }
pub fn g_COLD_OPEN() -> Py { ensure_init(); G_COLD_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLD_OPEN(v: Py) { ensure_init(); G_COLD_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_COMFORT_SAID() -> Py { ensure_init(); G_COMFORT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMFORT_SAID(v: Py) { ensure_init(); G_COMFORT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPLAINT_WORDS() -> Py { ensure_init(); G_COMPLAINT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPLAINT_WORDS(v: Py) { ensure_init(); G_COMPLAINT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPLAIN_WHO() -> Py { ensure_init(); G_COMPLAIN_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPLAIN_WHO(v: Py) { ensure_init(); G_COMPLAIN_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_CONGRATS() -> Py { ensure_init(); G_CONGRATS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONGRATS(v: Py) { ensure_init(); G_CONGRATS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTACT_ANSWER() -> Py { ensure_init(); G_CONTACT_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTACT_ANSWER(v: Py) { ensure_init(); G_CONTACT_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRAST() -> Py { ensure_init(); G_CONTRAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRAST(v: Py) { ensure_init(); G_CONTRAST.with(|c| *c.borrow_mut() = v); }
pub fn g_COPULA() -> Py { ensure_init(); G_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); G_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_AFTER() -> Py { ensure_init(); G_CRISIS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_AFTER(v: Py) { ensure_init(); G_CRISIS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_AFTER_THEM() -> Py { ensure_init(); G_CRISIS_AFTER_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_AFTER_THEM(v: Py) { ensure_init(); G_CRISIS_AFTER_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_BETTER() -> Py { ensure_init(); G_CRISIS_BETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_BETTER(v: Py) { ensure_init(); G_CRISIS_BETTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_BETTER_THEM() -> Py { ensure_init(); G_CRISIS_BETTER_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_BETTER_THEM(v: Py) { ensure_init(); G_CRISIS_BETTER_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_WORDS() -> Py { ensure_init(); G_CRISIS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_WORDS(v: Py) { ensure_init(); G_CRISIS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CROWD() -> Py { ensure_init(); G_CROWD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROWD(v: Py) { ensure_init(); G_CROWD.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_FOR() -> Py { ensure_init(); G_CUT_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_FOR(v: Py) { ensure_init(); G_CUT_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_DARK_THING() -> Py { ensure_init(); G_DARK_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DARK_THING(v: Py) { ensure_init(); G_DARK_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_DATES_KEEP_SLIPPING() -> Py { ensure_init(); G_DATES_KEEP_SLIPPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATES_KEEP_SLIPPING(v: Py) { ensure_init(); G_DATES_KEEP_SLIPPING.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLINED_PLAN() -> Py { ensure_init(); G_DECLINED_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLINED_PLAN(v: Py) { ensure_init(); G_DECLINED_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLINED_PLAN_GENTLE() -> Py { ensure_init(); G_DECLINED_PLAN_GENTLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLINED_PLAN_GENTLE(v: Py) { ensure_init(); G_DECLINED_PLAN_GENTLE.with(|c| *c.borrow_mut() = v); }
pub fn g_DENTIST_WHO() -> Py { ensure_init(); G_DENTIST_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENTIST_WHO(v: Py) { ensure_init(); G_DENTIST_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_DETAILS_IN_A_WARM_LINE() -> Py { ensure_init(); G_DETAILS_IN_A_WARM_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETAILS_IN_A_WARM_LINE(v: Py) { ensure_init(); G_DETAILS_IN_A_WARM_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_DEVICE_WHO() -> Py { ensure_init(); G_DEVICE_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEVICE_WHO(v: Py) { ensure_init(); G_DEVICE_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_DIE_OF_IT() -> Py { ensure_init(); G_DIE_OF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIE_OF_IT(v: Py) { ensure_init(); G_DIE_OF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_DETERMINER() -> Py { ensure_init(); G_DISH_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_DETERMINER(v: Py) { ensure_init(); G_DISH_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_PARTICIPLE() -> Py { ensure_init(); G_DISH_PARTICIPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_PARTICIPLE(v: Py) { ensure_init(); G_DISH_PARTICIPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_DITHER_ABOUT_A_PERSON() -> Py { ensure_init(); G_DITHER_ABOUT_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DITHER_ABOUT_A_PERSON(v: Py) { ensure_init(); G_DITHER_ABOUT_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_DITHER_DECIDE() -> Py { ensure_init(); G_DITHER_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DITHER_DECIDE(v: Py) { ensure_init(); G_DITHER_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_DITHER_OPEN_LINES() -> Py { ensure_init(); G_DITHER_OPEN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DITHER_OPEN_LINES(v: Py) { ensure_init(); G_DITHER_OPEN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_DITHER_SUBJECT() -> Py { ensure_init(); G_DITHER_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DITHER_SUBJECT(v: Py) { ensure_init(); G_DITHER_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_DITHER_TWO_WAY_LINES() -> Py { ensure_init(); G_DITHER_TWO_WAY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DITHER_TWO_WAY_LINES(v: Py) { ensure_init(); G_DITHER_TWO_WAY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_NOT_A_PERSON() -> Py { ensure_init(); G_DONE_NOT_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_NOT_A_PERSON(v: Py) { ensure_init(); G_DONE_NOT_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_NOT_THIS() -> Py { ensure_init(); G_DONE_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_NOT_THIS(v: Py) { ensure_init(); G_DONE_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_ONLY_A_THING() -> Py { ensure_init(); G_DONE_ONLY_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_ONLY_A_THING(v: Py) { ensure_init(); G_DONE_ONLY_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_VERB() -> Py { ensure_init(); G_DONE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_VERB(v: Py) { ensure_init(); G_DONE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBLE_BOOKED() -> Py { ensure_init(); G_DOUBLE_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBLE_BOOKED(v: Py) { ensure_init(); G_DOUBLE_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_EASING_FILLER() -> Py { ensure_init(); G_EASING_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASING_FILLER(v: Py) { ensure_init(); G_EASING_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_EATEN_KIND() -> Py { ensure_init(); G_EATEN_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EATEN_KIND(v: Py) { ensure_init(); G_EATEN_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_EATING() -> Py { ensure_init(); G_EATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EATING(v: Py) { ensure_init(); G_EATING.with(|c| *c.borrow_mut() = v); }
pub fn g_ELDER_AID_REPLIES() -> Py { ensure_init(); G_ELDER_AID_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELDER_AID_REPLIES(v: Py) { ensure_init(); G_ELDER_AID_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ELDER_CLINIC_REPLIES() -> Py { ensure_init(); G_ELDER_CLINIC_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELDER_CLINIC_REPLIES(v: Py) { ensure_init(); G_ELDER_CLINIC_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ELDER_DOCTOR_REPLIES() -> Py { ensure_init(); G_ELDER_DOCTOR_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELDER_DOCTOR_REPLIES(v: Py) { ensure_init(); G_ELDER_DOCTOR_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ELDER_HELP_REPLIES() -> Py { ensure_init(); G_ELDER_HELP_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELDER_HELP_REPLIES(v: Py) { ensure_init(); G_ELDER_HELP_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EMBARRASSED() -> Py { ensure_init(); G_EMBARRASSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMBARRASSED(v: Py) { ensure_init(); G_EMBARRASSED.with(|c| *c.borrow_mut() = v); }
pub fn g_EMBARRASSMENT_WORD() -> Py { ensure_init(); G_EMBARRASSMENT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMBARRASSMENT_WORD(v: Py) { ensure_init(); G_EMBARRASSMENT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY_AFTER() -> Py { ensure_init(); G_EMERGENCY_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_AFTER(v: Py) { ensure_init(); G_EMERGENCY_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY_ANSWER() -> Py { ensure_init(); G_EMERGENCY_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_ANSWER(v: Py) { ensure_init(); G_EMERGENCY_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY_BETTER() -> Py { ensure_init(); G_EMERGENCY_BETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_BETTER(v: Py) { ensure_init(); G_EMERGENCY_BETTER.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY_BETTER_THEM() -> Py { ensure_init(); G_EMERGENCY_BETTER_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_BETTER_THEM(v: Py) { ensure_init(); G_EMERGENCY_BETTER_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY_NOT_WORKED() -> Py { ensure_init(); G_EMERGENCY_NOT_WORKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_NOT_WORKED(v: Py) { ensure_init(); G_EMERGENCY_NOT_WORKED.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY_WORDS() -> Py { ensure_init(); G_EMERGENCY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY_WORDS(v: Py) { ensure_init(); G_EMERGENCY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_EMPTY() -> Py { ensure_init(); G_EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMPTY(v: Py) { ensure_init(); G_EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g_ENDINGS() -> Py { ensure_init(); G_ENDINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENDINGS(v: Py) { ensure_init(); G_ENDINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPERT_WORDS() -> Py { ensure_init(); G_EXPERT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPERT_WORDS(v: Py) { ensure_init(); G_EXPERT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAINT_COMING_ANSWER() -> Py { ensure_init(); G_FAINT_COMING_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAINT_COMING_ANSWER(v: Py) { ensure_init(); G_FAINT_COMING_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_FALLBACK_MODULE() -> Py { ensure_init(); G_FALLBACK_MODULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FALLBACK_MODULE(v: Py) { ensure_init(); G_FALLBACK_MODULE.with(|c| *c.borrow_mut() = v); }
pub fn g_FAVOURITE_GENERAL() -> Py { ensure_init(); G_FAVOURITE_GENERAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAVOURITE_GENERAL(v: Py) { ensure_init(); G_FAVOURITE_GENERAL.with(|c| *c.borrow_mut() = v); }
pub fn g_FAVOURITE_WORDS() -> Py { ensure_init(); G_FAVOURITE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAVOURITE_WORDS(v: Py) { ensure_init(); G_FAVOURITE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLERS() -> Py { ensure_init(); G_FILLERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLERS(v: Py) { ensure_init(); G_FILLERS.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLING_FOODS() -> Py { ensure_init(); G_FILLING_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLING_FOODS(v: Py) { ensure_init(); G_FILLING_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLING_WORDS() -> Py { ensure_init(); G_FILLING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLING_WORDS(v: Py) { ensure_init(); G_FILLING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON() -> Py { ensure_init(); G_FIRST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON(v: Py) { ensure_init(); G_FIRST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_FLAT() -> Py { ensure_init(); G_FLAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLAT(v: Py) { ensure_init(); G_FLAT.with(|c| *c.borrow_mut() = v); }
pub fn g_FLAT_CORE() -> Py { ensure_init(); G_FLAT_CORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLAT_CORE(v: Py) { ensure_init(); G_FLAT_CORE.with(|c| *c.borrow_mut() = v); }
pub fn g_FLAT_LEAD() -> Py { ensure_init(); G_FLAT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLAT_LEAD(v: Py) { ensure_init(); G_FLAT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_FLOPPY_WAKE() -> Py { ensure_init(); G_FLOPPY_WAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLOPPY_WAKE(v: Py) { ensure_init(); G_FLOPPY_WAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST() -> Py { ensure_init(); G_FORECAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST(v: Py) { ensure_init(); G_FORECAST.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_AFTER_SAFETY() -> Py { ensure_init(); G_FORECAST_AFTER_SAFETY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_AFTER_SAFETY(v: Py) { ensure_init(); G_FORECAST_AFTER_SAFETY.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_A_FALL() -> Py { ensure_init(); G_FORECAST_A_FALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_A_FALL(v: Py) { ensure_init(); G_FORECAST_A_FALL.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_BODY_WORDS() -> Py { ensure_init(); G_FORECAST_BODY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_BODY_WORDS(v: Py) { ensure_init(); G_FORECAST_BODY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_DURATION() -> Py { ensure_init(); G_FORECAST_DURATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_DURATION(v: Py) { ensure_init(); G_FORECAST_DURATION.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_HE_SHE() -> Py { ensure_init(); G_FORECAST_HE_SHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_HE_SHE(v: Py) { ensure_init(); G_FORECAST_HE_SHE.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_ILL() -> Py { ensure_init(); G_FORECAST_ILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_ILL(v: Py) { ensure_init(); G_FORECAST_ILL.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_I_ASK() -> Py { ensure_init(); G_FORECAST_I_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_I_ASK(v: Py) { ensure_init(); G_FORECAST_I_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_OFF_THEIR_FOOD() -> Py { ensure_init(); G_FORECAST_OFF_THEIR_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_OFF_THEIR_FOOD(v: Py) { ensure_init(); G_FORECAST_OFF_THEIR_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_PERMISSION() -> Py { ensure_init(); G_FORECAST_PERMISSION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_PERMISSION(v: Py) { ensure_init(); G_FORECAST_PERMISSION.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_PERSON() -> Py { ensure_init(); G_FORECAST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_PERSON(v: Py) { ensure_init(); G_FORECAST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_PET() -> Py { ensure_init(); G_FORECAST_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_PET(v: Py) { ensure_init(); G_FORECAST_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_PET_HARM() -> Py { ensure_init(); G_FORECAST_PET_HARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_PET_HARM(v: Py) { ensure_init(); G_FORECAST_PET_HARM.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_POOLS() -> Py { ensure_init(); G_FORECAST_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_POOLS(v: Py) { ensure_init(); G_FORECAST_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_RELATION() -> Py { ensure_init(); G_FORECAST_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_RELATION(v: Py) { ensure_init(); G_FORECAST_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_SELF() -> Py { ensure_init(); G_FORECAST_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_SELF(v: Py) { ensure_init(); G_FORECAST_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_SELF_STRUGGLING() -> Py { ensure_init(); G_FORECAST_SELF_STRUGGLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_SELF_STRUGGLING(v: Py) { ensure_init(); G_FORECAST_SELF_STRUGGLING.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_SOMEONE_STRUGGLING() -> Py { ensure_init(); G_FORECAST_SOMEONE_STRUGGLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_SOMEONE_STRUGGLING(v: Py) { ensure_init(); G_FORECAST_SOMEONE_STRUGGLING.with(|c| *c.borrow_mut() = v); }
pub fn g_FORECAST_STRUGGLE() -> Py { ensure_init(); G_FORECAST_STRUGGLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORECAST_STRUGGLE(v: Py) { ensure_init(); G_FORECAST_STRUGGLE.with(|c| *c.borrow_mut() = v); }
pub fn g_FORGETTING() -> Py { ensure_init(); G_FORGETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORGETTING(v: Py) { ensure_init(); G_FORGETTING.with(|c| *c.borrow_mut() = v); }
pub fn g_FORGET_A_THING() -> Py { ensure_init(); G_FORGET_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORGET_A_THING(v: Py) { ensure_init(); G_FORGET_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_FOUND_IT() -> Py { ensure_init(); G_FOUND_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOUND_IT(v: Py) { ensure_init(); G_FOUND_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME_WORDS() -> Py { ensure_init(); G_FRAME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME_WORDS(v: Py) { ensure_init(); G_FRAME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FUEL_WORDS() -> Py { ensure_init(); G_FUEL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FUEL_WORDS(v: Py) { ensure_init(); G_FUEL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FUME_WORDS() -> Py { ensure_init(); G_FUME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FUME_WORDS(v: Py) { ensure_init(); G_FUME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GADGET_WORDS() -> Py { ensure_init(); G_GADGET_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GADGET_WORDS(v: Py) { ensure_init(); G_GADGET_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GAS_ANSWER() -> Py { ensure_init(); G_GAS_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAS_ANSWER(v: Py) { ensure_init(); G_GAS_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_GAS_WORDS() -> Py { ensure_init(); G_GAS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAS_WORDS(v: Py) { ensure_init(); G_GAS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERIC_WORDS() -> Py { ensure_init(); G_GENERIC_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERIC_WORDS(v: Py) { ensure_init(); G_GENERIC_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_WORDS() -> Py { ensure_init(); G_GIFT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_WORDS(v: Py) { ensure_init(); G_GIFT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_TO_DIE() -> Py { ensure_init(); G_GOING_TO_DIE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_TO_DIE(v: Py) { ensure_init(); G_GOING_TO_DIE.with(|c| *c.borrow_mut() = v); }
pub fn g_GONE_VERB() -> Py { ensure_init(); G_GONE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GONE_VERB(v: Py) { ensure_init(); G_GONE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_TIME_WORDS() -> Py { ensure_init(); G_GOOD_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_TIME_WORDS(v: Py) { ensure_init(); G_GOOD_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GO_ANSWER() -> Py { ensure_init(); G_GO_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GO_ANSWER(v: Py) { ensure_init(); G_GO_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_GO_TO_BE_SEEN() -> Py { ensure_init(); G_GO_TO_BE_SEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GO_TO_BE_SEEN(v: Py) { ensure_init(); G_GO_TO_BE_SEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_GREETING() -> Py { ensure_init(); G_GREETING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREETING(v: Py) { ensure_init(); G_GREETING.with(|c| *c.borrow_mut() = v); }
pub fn g_GREETING_AFTER() -> Py { ensure_init(); G_GREETING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREETING_AFTER(v: Py) { ensure_init(); G_GREETING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_GREETING_WORDS() -> Py { ensure_init(); G_GREETING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREETING_WORDS(v: Py) { ensure_init(); G_GREETING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEF_AFTER() -> Py { ensure_init(); G_GRIEF_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEF_AFTER(v: Py) { ensure_init(); G_GRIEF_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE() -> Py { ensure_init(); G_GRIEVANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE(v: Py) { ensure_init(); G_GRIEVANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_HARM() -> Py { ensure_init(); G_GRIEVANCE_HARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_HARM(v: Py) { ensure_init(); G_GRIEVANCE_HARM.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_HARM_PHRASES() -> Py { ensure_init(); G_GRIEVANCE_HARM_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_HARM_PHRASES(v: Py) { ensure_init(); G_GRIEVANCE_HARM_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_HEALTH() -> Py { ensure_init(); G_GRIEVANCE_HEALTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_HEALTH(v: Py) { ensure_init(); G_GRIEVANCE_HEALTH.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_HEALTH_PHRASES() -> Py { ensure_init(); G_GRIEVANCE_HEALTH_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_HEALTH_PHRASES(v: Py) { ensure_init(); G_GRIEVANCE_HEALTH_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_IMPACT() -> Py { ensure_init(); G_GRIEVANCE_IMPACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_IMPACT(v: Py) { ensure_init(); G_GRIEVANCE_IMPACT.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_NOT_A_THING() -> Py { ensure_init(); G_GRIEVANCE_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_NOT_A_THING(v: Py) { ensure_init(); G_GRIEVANCE_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_PEOPLE() -> Py { ensure_init(); G_GRIEVANCE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_PEOPLE(v: Py) { ensure_init(); G_GRIEVANCE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_PHRASES() -> Py { ensure_init(); G_GRIEVANCE_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_PHRASES(v: Py) { ensure_init(); G_GRIEVANCE_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_PRONOUN() -> Py { ensure_init(); G_GRIEVANCE_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_PRONOUN(v: Py) { ensure_init(); G_GRIEVANCE_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_STRAIN() -> Py { ensure_init(); G_GRIEVANCE_STRAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_STRAIN(v: Py) { ensure_init(); G_GRIEVANCE_STRAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIEVANCE_STRAIN_SAID() -> Py { ensure_init(); G_GRIEVANCE_STRAIN_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEVANCE_STRAIN_SAID(v: Py) { ensure_init(); G_GRIEVANCE_STRAIN_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_GROWN_UP_WORDS() -> Py { ensure_init(); G_GROWN_UP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROWN_UP_WORDS(v: Py) { ensure_init(); G_GROWN_UP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT_ACT() -> Py { ensure_init(); G_HABIT_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT_ACT(v: Py) { ensure_init(); G_HABIT_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT_PROBLEM_ACT() -> Py { ensure_init(); G_HABIT_PROBLEM_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT_PROBLEM_ACT(v: Py) { ensure_init(); G_HABIT_PROBLEM_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT_REPLY() -> Py { ensure_init(); G_HABIT_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT_REPLY(v: Py) { ensure_init(); G_HABIT_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_HAND() -> Py { ensure_init(); G_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAND(v: Py) { ensure_init(); G_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g_HANDED_BACK() -> Py { ensure_init(); G_HANDED_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HANDED_BACK(v: Py) { ensure_init(); G_HANDED_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_HANDS_BACK() -> Py { ensure_init(); G_HANDS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HANDS_BACK(v: Py) { ensure_init(); G_HANDS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_HAZARD_AFTER() -> Py { ensure_init(); G_HAZARD_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAZARD_AFTER(v: Py) { ensure_init(); G_HAZARD_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_HAZARD_WORDS() -> Py { ensure_init(); G_HAZARD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAZARD_WORDS(v: Py) { ensure_init(); G_HAZARD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_RECORD() -> Py { ensure_init(); G_HEALTH_RECORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_RECORD(v: Py) { ensure_init(); G_HEALTH_RECORD.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_RECORD_LINES() -> Py { ensure_init(); G_HEALTH_RECORD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_RECORD_LINES(v: Py) { ensure_init(); G_HEALTH_RECORD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_WHO() -> Py { ensure_init(); G_HEALTH_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_WHO(v: Py) { ensure_init(); G_HEALTH_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_WORDS() -> Py { ensure_init(); G_HEALTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_WORDS(v: Py) { ensure_init(); G_HEALTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_WORLD() -> Py { ensure_init(); G_HEALTH_WORLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_WORLD(v: Py) { ensure_init(); G_HEALTH_WORLD.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAVY_NEWS() -> Py { ensure_init(); G_HEAVY_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAVY_NEWS(v: Py) { ensure_init(); G_HEAVY_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEDGE_POOL() -> Py { ensure_init(); G_HEDGE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEDGE_POOL(v: Py) { ensure_init(); G_HEDGE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_HELLO_STILL_THERE() -> Py { ensure_init(); G_HELLO_STILL_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELLO_STILL_THERE(v: Py) { ensure_init(); G_HELLO_STILL_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_HELLO_WITH_YOU() -> Py { ensure_init(); G_HELLO_WITH_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELLO_WITH_YOU(v: Py) { ensure_init(); G_HELLO_WITH_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP() -> Py { ensure_init(); G_HELP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP(v: Py) { ensure_init(); G_HELP.with(|c| *c.borrow_mut() = v); }
pub fn g_HELPLINE_NOT_AN_AMBULANCE() -> Py { ensure_init(); G_HELPLINE_NOT_AN_AMBULANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELPLINE_NOT_AN_AMBULANCE(v: Py) { ensure_init(); G_HELPLINE_NOT_AN_AMBULANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP_ALREADY() -> Py { ensure_init(); G_HELP_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP_ALREADY(v: Py) { ensure_init(); G_HELP_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP_TAIL() -> Py { ensure_init(); G_HELP_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP_TAIL(v: Py) { ensure_init(); G_HELP_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP_TOPIC() -> Py { ensure_init(); G_HELP_TOPIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP_TOPIC(v: Py) { ensure_init(); G_HELP_TOPIC.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP_TOPIC_ANY() -> Py { ensure_init(); G_HELP_TOPIC_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP_TOPIC_ANY(v: Py) { ensure_init(); G_HELP_TOPIC_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_HOLD_AFTER() -> Py { ensure_init(); G_HOLD_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOLD_AFTER(v: Py) { ensure_init(); G_HOLD_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_HOUSE_VERB() -> Py { ensure_init(); G_HOUSE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOUSE_VERB(v: Py) { ensure_init(); G_HOUSE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_ARE_YOU() -> Py { ensure_init(); G_HOW_ARE_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_ARE_YOU(v: Py) { ensure_init(); G_HOW_ARE_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_ARE_YOU_ASKS() -> Py { ensure_init(); G_HOW_ARE_YOU_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_ARE_YOU_ASKS(v: Py) { ensure_init(); G_HOW_ARE_YOU_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_ARE_YOU_GOING_ON() -> Py { ensure_init(); G_HOW_ARE_YOU_GOING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_ARE_YOU_GOING_ON(v: Py) { ensure_init(); G_HOW_ARE_YOU_GOING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_ARE_YOU_GOING_ON_AFTER() -> Py { ensure_init(); G_HOW_ARE_YOU_GOING_ON_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_ARE_YOU_GOING_ON_AFTER(v: Py) { ensure_init(); G_HOW_ARE_YOU_GOING_ON_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_ARE_YOU_TAGS() -> Py { ensure_init(); G_HOW_ARE_YOU_TAGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_ARE_YOU_TAGS(v: Py) { ensure_init(); G_HOW_ARE_YOU_TAGS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_DECIDE_FILLER() -> Py { ensure_init(); G_HOW_TO_DECIDE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_DECIDE_FILLER(v: Py) { ensure_init(); G_HOW_TO_DECIDE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_DECIDE_LINES() -> Py { ensure_init(); G_HOW_TO_DECIDE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_DECIDE_LINES(v: Py) { ensure_init(); G_HOW_TO_DECIDE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_HUMAN_EMPTY() -> Py { ensure_init(); G_HUMAN_EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HUMAN_EMPTY(v: Py) { ensure_init(); G_HUMAN_EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT_EVENT() -> Py { ensure_init(); G_HURT_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT_EVENT(v: Py) { ensure_init(); G_HURT_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_IDENTITY() -> Py { ensure_init(); G_IDENTITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDENTITY(v: Py) { ensure_init(); G_IDENTITY.with(|c| *c.borrow_mut() = v); }
pub fn g_IDIOM_GLAD() -> Py { ensure_init(); G_IDIOM_GLAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDIOM_GLAD(v: Py) { ensure_init(); G_IDIOM_GLAD.with(|c| *c.borrow_mut() = v); }
pub fn g_IDIOM_SHY() -> Py { ensure_init(); G_IDIOM_SHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDIOM_SHY(v: Py) { ensure_init(); G_IDIOM_SHY.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_AFTER() -> Py { ensure_init(); G_ILL_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_AFTER(v: Py) { ensure_init(); G_ILL_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_BEFORE() -> Py { ensure_init(); G_ILL_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_BEFORE(v: Py) { ensure_init(); G_ILL_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_SAID_OF_A_BODY() -> Py { ensure_init(); G_ILL_SAID_OF_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_SAID_OF_A_BODY(v: Py) { ensure_init(); G_ILL_SAID_OF_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_WILL_VERBS() -> Py { ensure_init(); G_ILL_WILL_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_WILL_VERBS(v: Py) { ensure_init(); G_ILL_WILL_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_IMPERATIVE() -> Py { ensure_init(); G_IMPERATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IMPERATIVE(v: Py) { ensure_init(); G_IMPERATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITED() -> Py { ensure_init(); G_INVITED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITED(v: Py) { ensure_init(); G_INVITED.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITED_TO_TALK() -> Py { ensure_init(); G_INVITED_TO_TALK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITED_TO_TALK(v: Py) { ensure_init(); G_INVITED_TO_TALK.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITE_ACTIVITY() -> Py { ensure_init(); G_INVITE_ACTIVITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITE_ACTIVITY(v: Py) { ensure_init(); G_INVITE_ACTIVITY.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITE_FILLER() -> Py { ensure_init(); G_INVITE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITE_FILLER(v: Py) { ensure_init(); G_INVITE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITE_NEVER() -> Py { ensure_init(); G_INVITE_NEVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITE_NEVER(v: Py) { ensure_init(); G_INVITE_NEVER.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITE_PRONOUN() -> Py { ensure_init(); G_INVITE_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITE_PRONOUN(v: Py) { ensure_init(); G_INVITE_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_A_ROOM() -> Py { ensure_init(); G_IN_A_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_A_ROOM(v: Py) { ensure_init(); G_IN_A_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g_I_RATE_A_SKILL() -> Py { ensure_init(); G_I_RATE_A_SKILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_RATE_A_SKILL(v: Py) { ensure_init(); G_I_RATE_A_SKILL.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINT_WORDS() -> Py { ensure_init(); G_JOINT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINT_WORDS(v: Py) { ensure_init(); G_JOINT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_ASKS_FINE() -> Py { ensure_init(); G_JUDGE_ASKS_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_ASKS_FINE(v: Py) { ensure_init(); G_JUDGE_ASKS_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_ASKS_MAD() -> Py { ensure_init(); G_JUDGE_ASKS_MAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_ASKS_MAD(v: Py) { ensure_init(); G_JUDGE_ASKS_MAD.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_ASKS_SLIGHT() -> Py { ensure_init(); G_JUDGE_ASKS_SLIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_ASKS_SLIGHT(v: Py) { ensure_init(); G_JUDGE_ASKS_SLIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_BARE_FACES() -> Py { ensure_init(); G_JUDGE_BARE_FACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_BARE_FACES(v: Py) { ensure_init(); G_JUDGE_BARE_FACES.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_FINE() -> Py { ensure_init(); G_JUDGE_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_FINE(v: Py) { ensure_init(); G_JUDGE_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_FINE_PLAIN() -> Py { ensure_init(); G_JUDGE_FINE_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_FINE_PLAIN(v: Py) { ensure_init(); G_JUDGE_FINE_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_LEAD() -> Py { ensure_init(); G_JUDGE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_LEAD(v: Py) { ensure_init(); G_JUDGE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NOD_WORDS() -> Py { ensure_init(); G_JUDGE_NOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NOD_WORDS(v: Py) { ensure_init(); G_JUDGE_NOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NOT_A_GERUND() -> Py { ensure_init(); G_JUDGE_NOT_A_GERUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NOT_A_GERUND(v: Py) { ensure_init(); G_JUDGE_NOT_A_GERUND.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NOT_A_VERDICT() -> Py { ensure_init(); G_JUDGE_NOT_A_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NOT_A_VERDICT(v: Py) { ensure_init(); G_JUDGE_NOT_A_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NOUN() -> Py { ensure_init(); G_JUDGE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NOUN(v: Py) { ensure_init(); G_JUDGE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NO_PLAN() -> Py { ensure_init(); G_JUDGE_NO_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NO_PLAN(v: Py) { ensure_init(); G_JUDGE_NO_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NO_PLAN_FINE() -> Py { ensure_init(); G_JUDGE_NO_PLAN_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NO_PLAN_FINE(v: Py) { ensure_init(); G_JUDGE_NO_PLAN_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NO_PLAN_MAD() -> Py { ensure_init(); G_JUDGE_NO_PLAN_MAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NO_PLAN_MAD(v: Py) { ensure_init(); G_JUDGE_NO_PLAN_MAD.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_NO_PLAN_MIND() -> Py { ensure_init(); G_JUDGE_NO_PLAN_MIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_NO_PLAN_MIND(v: Py) { ensure_init(); G_JUDGE_NO_PLAN_MIND.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_ONLY_A_PLAN() -> Py { ensure_init(); G_JUDGE_ONLY_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_ONLY_A_PLAN(v: Py) { ensure_init(); G_JUDGE_ONLY_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_PLAN_HAS_A_HOLE() -> Py { ensure_init(); G_JUDGE_PLAN_HAS_A_HOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_PLAN_HAS_A_HOLE(v: Py) { ensure_init(); G_JUDGE_PLAN_HAS_A_HOLE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_PLAN_STOP() -> Py { ensure_init(); G_JUDGE_PLAN_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_PLAN_STOP(v: Py) { ensure_init(); G_JUDGE_PLAN_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_SANE() -> Py { ensure_init(); G_JUDGE_SANE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_SANE(v: Py) { ensure_init(); G_JUDGE_SANE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_SANE_PLAIN() -> Py { ensure_init(); G_JUDGE_SANE_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_SANE_PLAIN(v: Py) { ensure_init(); G_JUDGE_SANE_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_SELF_JUDGING() -> Py { ensure_init(); G_JUDGE_SELF_JUDGING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_SELF_JUDGING(v: Py) { ensure_init(); G_JUDGE_SELF_JUDGING.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_SOUNDS_FINE() -> Py { ensure_init(); G_JUDGE_SOUNDS_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_SOUNDS_FINE(v: Py) { ensure_init(); G_JUDGE_SOUNDS_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_WISH_ANGRY() -> Py { ensure_init(); G_JUDGE_WISH_ANGRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_WISH_ANGRY(v: Py) { ensure_init(); G_JUDGE_WISH_ANGRY.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_WORTH() -> Py { ensure_init(); G_JUDGE_WORTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_WORTH(v: Py) { ensure_init(); G_JUDGE_WORTH.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_WORTH_PLAIN() -> Py { ensure_init(); G_JUDGE_WORTH_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_WORTH_PLAIN(v: Py) { ensure_init(); G_JUDGE_WORTH_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_JUNK_CUES() -> Py { ensure_init(); G_JUNK_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUNK_CUES(v: Py) { ensure_init(); G_JUNK_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPS_FORGETTING() -> Py { ensure_init(); G_KEEPS_FORGETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPS_FORGETTING(v: Py) { ensure_init(); G_KEEPS_FORGETTING.with(|c| *c.borrow_mut() = v); }
pub fn g_KNOCKED() -> Py { ensure_init(); G_KNOCKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNOCKED(v: Py) { ensure_init(); G_KNOCKED.with(|c| *c.borrow_mut() = v); }
pub fn g_KNOCKED_SPOTS() -> Py { ensure_init(); G_KNOCKED_SPOTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNOCKED_SPOTS(v: Py) { ensure_init(); G_KNOCKED_SPOTS.with(|c| *c.borrow_mut() = v); }
pub fn g_LANDED_BAD() -> Py { ensure_init(); G_LANDED_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LANDED_BAD(v: Py) { ensure_init(); G_LANDED_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g_LANGUAGE_FLUENCY() -> Py { ensure_init(); G_LANGUAGE_FLUENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LANGUAGE_FLUENCY(v: Py) { ensure_init(); G_LANGUAGE_FLUENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_LATE_BEHIND_LINES() -> Py { ensure_init(); G_LATE_BEHIND_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LATE_BEHIND_LINES(v: Py) { ensure_init(); G_LATE_BEHIND_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LATE_NOW_LINES() -> Py { ensure_init(); G_LATE_NOW_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LATE_NOW_LINES(v: Py) { ensure_init(); G_LATE_NOW_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGHED_AT() -> Py { ensure_init(); G_LAUGHED_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGHED_AT(v: Py) { ensure_init(); G_LAUGHED_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGHED_AT_THEM() -> Py { ensure_init(); G_LAUGHED_AT_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGHED_AT_THEM(v: Py) { ensure_init(); G_LAUGHED_AT_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGHING_WORDS() -> Py { ensure_init(); G_LAUGHING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGHING_WORDS(v: Py) { ensure_init(); G_LAUGHING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_BEHIND() -> Py { ensure_init(); G_LEFT_BEHIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_BEHIND(v: Py) { ensure_init(); G_LEFT_BEHIND.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_POOL() -> Py { ensure_init(); G_LEFT_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_POOL(v: Py) { ensure_init(); G_LEFT_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_POOL_GONE() -> Py { ensure_init(); G_LEFT_POOL_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_POOL_GONE(v: Py) { ensure_init(); G_LEFT_POOL_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_ASK() -> Py { ensure_init(); G_LIVE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_ASK(v: Py) { ensure_init(); G_LIVE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_DELAY_WORDS() -> Py { ensure_init(); G_LIVE_DELAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_DELAY_WORDS(v: Py) { ensure_init(); G_LIVE_DELAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_HOURS_ASK() -> Py { ensure_init(); G_LIVE_HOURS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_HOURS_ASK(v: Py) { ensure_init(); G_LIVE_HOURS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_INDOOR() -> Py { ensure_init(); G_LIVE_INDOOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_INDOOR(v: Py) { ensure_init(); G_LIVE_INDOOR.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_JOURNEY_ENDS() -> Py { ensure_init(); G_LIVE_JOURNEY_ENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_JOURNEY_ENDS(v: Py) { ensure_init(); G_LIVE_JOURNEY_ENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_KINDS() -> Py { ensure_init(); G_LIVE_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_KINDS(v: Py) { ensure_init(); G_LIVE_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_NEWS_REPLY() -> Py { ensure_init(); G_LIVE_NEWS_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_NEWS_REPLY(v: Py) { ensure_init(); G_LIVE_NEWS_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_NOT() -> Py { ensure_init(); G_LIVE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_NOT(v: Py) { ensure_init(); G_LIVE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_NOT_HOURS() -> Py { ensure_init(); G_LIVE_NOT_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_NOT_HOURS(v: Py) { ensure_init(); G_LIVE_NOT_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_OPEN_CLOSE() -> Py { ensure_init(); G_LIVE_OPEN_CLOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_OPEN_CLOSE(v: Py) { ensure_init(); G_LIVE_OPEN_CLOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_PAPER_WORDS() -> Py { ensure_init(); G_LIVE_PAPER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_PAPER_WORDS(v: Py) { ensure_init(); G_LIVE_PAPER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_ROADS_NOT() -> Py { ensure_init(); G_LIVE_ROADS_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_ROADS_NOT(v: Py) { ensure_init(); G_LIVE_ROADS_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_ROAD_NAME() -> Py { ensure_init(); G_LIVE_ROAD_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_ROAD_NAME(v: Py) { ensure_init(); G_LIVE_ROAD_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_ROAD_WORDS() -> Py { ensure_init(); G_LIVE_ROAD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_ROAD_WORDS(v: Py) { ensure_init(); G_LIVE_ROAD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_SETTING() -> Py { ensure_init(); G_LIVE_SETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_SETTING(v: Py) { ensure_init(); G_LIVE_SETTING.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_SITE_THING() -> Py { ensure_init(); G_LIVE_SITE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_SITE_THING(v: Py) { ensure_init(); G_LIVE_SITE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_SKY() -> Py { ensure_init(); G_LIVE_SKY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_SKY(v: Py) { ensure_init(); G_LIVE_SKY.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_SKY_OFF() -> Py { ensure_init(); G_LIVE_SKY_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_SKY_OFF(v: Py) { ensure_init(); G_LIVE_SKY_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_SKY_WHY() -> Py { ensure_init(); G_LIVE_SKY_WHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_SKY_WHY(v: Py) { ensure_init(); G_LIVE_SKY_WHY.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_SOON() -> Py { ensure_init(); G_LIVE_SOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_SOON(v: Py) { ensure_init(); G_LIVE_SOON.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_STRONG() -> Py { ensure_init(); G_LIVE_STRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_STRONG(v: Py) { ensure_init(); G_LIVE_STRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_TAKE_ALLOWED() -> Py { ensure_init(); G_LIVE_TAKE_ALLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_TAKE_ALLOWED(v: Py) { ensure_init(); G_LIVE_TAKE_ALLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_TAKE_ASK() -> Py { ensure_init(); G_LIVE_TAKE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_TAKE_ASK(v: Py) { ensure_init(); G_LIVE_TAKE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_TRAVEL_WORDS() -> Py { ensure_init(); G_LIVE_TRAVEL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_TRAVEL_WORDS(v: Py) { ensure_init(); G_LIVE_TRAVEL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_VENUE() -> Py { ensure_init(); G_LIVE_VENUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_VENUE(v: Py) { ensure_init(); G_LIVE_VENUE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_VENUE_RE() -> Py { ensure_init(); G_LIVE_VENUE_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_VENUE_RE(v: Py) { ensure_init(); G_LIVE_VENUE_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_WEAR() -> Py { ensure_init(); G_LIVE_WEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_WEAR(v: Py) { ensure_init(); G_LIVE_WEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_WEATHER_WORDS() -> Py { ensure_init(); G_LIVE_WEATHER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_WEATHER_WORDS(v: Py) { ensure_init(); G_LIVE_WEATHER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_YES_NO_OPEN() -> Py { ensure_init(); G_LIVE_YES_NO_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_YES_NO_OPEN(v: Py) { ensure_init(); G_LIVE_YES_NO_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_LOCKED_OUT_LINES() -> Py { ensure_init(); G_LOCKED_OUT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOCKED_OUT_LINES(v: Py) { ensure_init(); G_LOCKED_OUT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LOCKED_OUT_OF_THE_CAR() -> Py { ensure_init(); G_LOCKED_OUT_OF_THE_CAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOCKED_OUT_OF_THE_CAR(v: Py) { ensure_init(); G_LOCKED_OUT_OF_THE_CAR.with(|c| *c.borrow_mut() = v); }
pub fn g_LOST_A_THING() -> Py { ensure_init(); G_LOST_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOST_A_THING(v: Py) { ensure_init(); G_LOST_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_LOST_A_THING_AGAIN() -> Py { ensure_init(); G_LOST_A_THING_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOST_A_THING_AGAIN(v: Py) { ensure_init(); G_LOST_A_THING_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_LOW_DAY_REPLIES() -> Py { ensure_init(); G_LOW_DAY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOW_DAY_REPLIES(v: Py) { ensure_init(); G_LOW_DAY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_LUMP() -> Py { ensure_init(); G_LUMP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LUMP(v: Py) { ensure_init(); G_LUMP.with(|c| *c.borrow_mut() = v); }
pub fn g_MANNERS_ONLY() -> Py { ensure_init(); G_MANNERS_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MANNERS_ONLY(v: Py) { ensure_init(); G_MANNERS_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_MEDS_WORDS() -> Py { ensure_init(); G_MEDS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEDS_WORDS(v: Py) { ensure_init(); G_MEDS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MEMORY() -> Py { ensure_init(); G_MEMORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEMORY(v: Py) { ensure_init(); G_MEMORY.with(|c| *c.borrow_mut() = v); }
pub fn g_MESS_PLACES() -> Py { ensure_init(); G_MESS_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MESS_PLACES(v: Py) { ensure_init(); G_MESS_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_MILESTONE() -> Py { ensure_init(); G_MILESTONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MILESTONE(v: Py) { ensure_init(); G_MILESTONE.with(|c| *c.borrow_mut() = v); }
pub fn g_MILESTONE_EVENT() -> Py { ensure_init(); G_MILESTONE_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MILESTONE_EVENT(v: Py) { ensure_init(); G_MILESTONE_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_MISHAP_BACK() -> Py { ensure_init(); G_MISHAP_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISHAP_BACK(v: Py) { ensure_init(); G_MISHAP_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP_LINES() -> Py { ensure_init(); G_MISSED_STEP_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_LINES(v: Py) { ensure_init(); G_MISSED_STEP_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_MODEL_NAMED_WORDS() -> Py { ensure_init(); G_MODEL_NAMED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MODEL_NAMED_WORDS(v: Py) { ensure_init(); G_MODEL_NAMED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_WORDS() -> Py { ensure_init(); G_MONEY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_WORDS(v: Py) { ensure_init(); G_MONEY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOOD_AFTER() -> Py { ensure_init(); G_MOOD_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOOD_AFTER(v: Py) { ensure_init(); G_MOOD_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_MOOD_BEFORE() -> Py { ensure_init(); G_MOOD_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOOD_BEFORE(v: Py) { ensure_init(); G_MOOD_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOUTH_WORDS() -> Py { ensure_init(); G_MOUTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOUTH_WORDS(v: Py) { ensure_init(); G_MOUTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_ON_AFTER() -> Py { ensure_init(); G_MOVE_ON_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_ON_AFTER(v: Py) { ensure_init(); G_MOVE_ON_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_MYSELF() -> Py { ensure_init(); G_MYSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MYSELF(v: Py) { ensure_init(); G_MYSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_BILL_LINES() -> Py { ensure_init(); G_MY_BILL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_BILL_LINES(v: Py) { ensure_init(); G_MY_BILL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_DOING() -> Py { ensure_init(); G_MY_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_DOING(v: Py) { ensure_init(); G_MY_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_OWN() -> Py { ensure_init(); G_MY_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_OWN(v: Py) { ensure_init(); G_MY_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_OWN_DAY() -> Py { ensure_init(); G_MY_OWN_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_OWN_DAY(v: Py) { ensure_init(); G_MY_OWN_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_PICK() -> Py { ensure_init(); G_MY_PICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_PICK(v: Py) { ensure_init(); G_MY_PICK.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_PICK_ASK() -> Py { ensure_init(); G_MY_PICK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_PICK_ASK(v: Py) { ensure_init(); G_MY_PICK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_PICK_NOTE() -> Py { ensure_init(); G_MY_PICK_NOTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_PICK_NOTE(v: Py) { ensure_init(); G_MY_PICK_NOTE.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_RECORDS() -> Py { ensure_init(); G_MY_RECORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_RECORDS(v: Py) { ensure_init(); G_MY_RECORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_WORKINGS() -> Py { ensure_init(); G_MY_WORKINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_WORKINGS(v: Py) { ensure_init(); G_MY_WORKINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_BODIES() -> Py { ensure_init(); G_NAMED_BODIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_BODIES(v: Py) { ensure_init(); G_NAMED_BODIES.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_NEGATIVE() -> Py { ensure_init(); G_NAMED_NEGATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_NEGATIVE(v: Py) { ensure_init(); G_NAMED_NEGATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_TYPES() -> Py { ensure_init(); G_NAMED_TYPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_TYPES(v: Py) { ensure_init(); G_NAMED_TYPES.with(|c| *c.borrow_mut() = v); }
pub fn g_NARROW_ANSWERED() -> Py { ensure_init(); G_NARROW_ANSWERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NARROW_ANSWERED(v: Py) { ensure_init(); G_NARROW_ANSWERED.with(|c| *c.borrow_mut() = v); }
pub fn g_NARROW_SUBSTANCE() -> Py { ensure_init(); G_NARROW_SUBSTANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NARROW_SUBSTANCE(v: Py) { ensure_init(); G_NARROW_SUBSTANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_NARROW_WORDS() -> Py { ensure_init(); G_NARROW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NARROW_WORDS(v: Py) { ensure_init(); G_NARROW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATIVE() -> Py { ensure_init(); G_NEGATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE(v: Py) { ensure_init(); G_NEGATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATIVE_CUES() -> Py { ensure_init(); G_NEGATIVE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE_CUES(v: Py) { ensure_init(); G_NEGATIVE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATORS() -> Py { ensure_init(); G_NEGATORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATORS(v: Py) { ensure_init(); G_NEGATORS.with(|c| *c.borrow_mut() = v); }
pub fn g_NETWORK_WORDS() -> Py { ensure_init(); G_NETWORK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NETWORK_WORDS(v: Py) { ensure_init(); G_NETWORK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEUTRAL() -> Py { ensure_init(); G_NEUTRAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEUTRAL(v: Py) { ensure_init(); G_NEUTRAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NEVER_MIND() -> Py { ensure_init(); G_NEVER_MIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEVER_MIND(v: Py) { ensure_init(); G_NEVER_MIND.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_TO_COME() -> Py { ensure_init(); G_NEWS_TO_COME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_TO_COME(v: Py) { ensure_init(); G_NEWS_TO_COME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_AT_ALL() -> Py { ensure_init(); G_NOTHING_AT_ALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_AT_ALL(v: Py) { ensure_init(); G_NOTHING_AT_ALL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_SAID_YET() -> Py { ensure_init(); G_NOTHING_SAID_YET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_SAID_YET(v: Py) { ensure_init(); G_NOTHING_SAID_YET.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_ANSWER() -> Py { ensure_init(); G_NOT_AN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_ANSWER(v: Py) { ensure_init(); G_NOT_AN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FORM() -> Py { ensure_init(); G_NOT_A_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FORM(v: Py) { ensure_init(); G_NOT_A_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NAME() -> Py { ensure_init(); G_NOT_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME(v: Py) { ensure_init(); G_NOT_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_VERDICT_SUBJECT() -> Py { ensure_init(); G_NOT_A_VERDICT_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_VERDICT_SUBJECT(v: Py) { ensure_init(); G_NOT_A_VERDICT_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_CHOSEN() -> Py { ensure_init(); G_NOT_CHOSEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_CHOSEN(v: Py) { ensure_init(); G_NOT_CHOSEN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ENOUGH_REPLY() -> Py { ensure_init(); G_NOT_ENOUGH_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ENOUGH_REPLY(v: Py) { ensure_init(); G_NOT_ENOUGH_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_FIT_ENOUGH() -> Py { ensure_init(); G_NOT_FIT_ENOUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_FIT_ENOUGH(v: Py) { ensure_init(); G_NOT_FIT_ENOUGH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_NOTED() -> Py { ensure_init(); G_NOT_NOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_NOTED(v: Py) { ensure_init(); G_NOT_NOTED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_PEOPLE_WORDS() -> Py { ensure_init(); G_NOT_PEOPLE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_PEOPLE_WORDS(v: Py) { ensure_init(); G_NOT_PEOPLE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THEIR_VERB() -> Py { ensure_init(); G_NOT_THEIR_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIR_VERB(v: Py) { ensure_init(); G_NOT_THEIR_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_TOO_OLD() -> Py { ensure_init(); G_NOT_TOO_OLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_TOO_OLD(v: Py) { ensure_init(); G_NOT_TOO_OLD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_WHAT_I_SAID() -> Py { ensure_init(); G_NOT_WHAT_I_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_WHAT_I_SAID(v: Py) { ensure_init(); G_NOT_WHAT_I_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_DIRECTION_REPLY() -> Py { ensure_init(); G_NO_DIRECTION_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_DIRECTION_REPLY(v: Py) { ensure_init(); G_NO_DIRECTION_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MORE_OF() -> Py { ensure_init(); G_NO_MORE_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MORE_OF(v: Py) { ensure_init(); G_NO_MORE_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_WORDS_ASKING() -> Py { ensure_init(); G_NO_WORDS_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_WORDS_ASKING(v: Py) { ensure_init(); G_NO_WORDS_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_WORDS_AT_THE_CARE() -> Py { ensure_init(); G_NO_WORDS_AT_THE_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_WORDS_AT_THE_CARE(v: Py) { ensure_init(); G_NO_WORDS_AT_THE_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_WORDS_SOMEONE() -> Py { ensure_init(); G_NO_WORDS_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_WORDS_SOMEONE(v: Py) { ensure_init(); G_NO_WORDS_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_WORDS_STUCK() -> Py { ensure_init(); G_NO_WORDS_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_WORDS_STUCK(v: Py) { ensure_init(); G_NO_WORDS_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_PLACES() -> Py { ensure_init(); G_NUMBER_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_PLACES(v: Py) { ensure_init(); G_NUMBER_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION() -> Py { ensure_init(); G_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION(v: Py) { ensure_init(); G_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFICE_RECORD_LINES() -> Py { ensure_init(); G_OFFICE_RECORD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFICE_RECORD_LINES(v: Py) { ensure_init(); G_OFFICE_RECORD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_SKILL_RATED() -> Py { ensure_init(); G_ONE_SKILL_RATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_SKILL_RATED(v: Py) { ensure_init(); G_ONE_SKILL_RATED.with(|c| *c.borrow_mut() = v); }
pub fn g_ONGOING_WORDS() -> Py { ensure_init(); G_ONGOING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONGOING_WORDS(v: Py) { ensure_init(); G_ONGOING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLINE_ASK() -> Py { ensure_init(); G_ONLINE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLINE_ASK(v: Py) { ensure_init(); G_ONLINE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_FRONT() -> Py { ensure_init(); G_ONLY_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_FRONT(v: Py) { ensure_init(); G_ONLY_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_OPINION_ASKS() -> Py { ensure_init(); G_OPINION_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPINION_ASKS(v: Py) { ensure_init(); G_OPINION_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_OURS() -> Py { ensure_init(); G_OURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OURS(v: Py) { ensure_init(); G_OURS.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTCOME_LINES() -> Py { ensure_init(); G_OUTCOME_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTCOME_LINES(v: Py) { ensure_init(); G_OUTCOME_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PERSON() -> Py { ensure_init(); G_OWN_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PERSON(v: Py) { ensure_init(); G_OWN_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PLACE() -> Py { ensure_init(); G_OWN_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PLACE(v: Py) { ensure_init(); G_OWN_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTICLE() -> Py { ensure_init(); G_PARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTICLE(v: Py) { ensure_init(); G_PARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PASSED_OUT() -> Py { ensure_init(); G_PASSED_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASSED_OUT(v: Py) { ensure_init(); G_PASSED_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_PASSED_OUT_ANSWER() -> Py { ensure_init(); G_PASSED_OUT_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASSED_OUT_ANSWER(v: Py) { ensure_init(); G_PASSED_OUT_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_PASSED_OUT_PERSON() -> Py { ensure_init(); G_PASSED_OUT_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASSED_OUT_PERSON(v: Py) { ensure_init(); G_PASSED_OUT_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PASSING_OUT_A_THING() -> Py { ensure_init(); G_PASSING_OUT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASSING_OUT_A_THING(v: Py) { ensure_init(); G_PASSING_OUT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PASSWORD_TROUBLE() -> Py { ensure_init(); G_PASSWORD_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASSWORD_TROUBLE(v: Py) { ensure_init(); G_PASSWORD_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_STATE_CUES() -> Py { ensure_init(); G_PAST_STATE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_STATE_CUES(v: Py) { ensure_init(); G_PAST_STATE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_PAUSE() -> Py { ensure_init(); G_PAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAUSE(v: Py) { ensure_init(); G_PAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAUSE_IN_A_SAFETY_THREAD() -> Py { ensure_init(); G_PAUSE_IN_A_SAFETY_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAUSE_IN_A_SAFETY_THREAD(v: Py) { ensure_init(); G_PAUSE_IN_A_SAFETY_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_PENDING_DOUBT() -> Py { ensure_init(); G_PENDING_DOUBT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PENDING_DOUBT(v: Py) { ensure_init(); G_PENDING_DOUBT.with(|c| *c.borrow_mut() = v); }
pub fn g_PENDING_LANDED() -> Py { ensure_init(); G_PENDING_LANDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PENDING_LANDED(v: Py) { ensure_init(); G_PENDING_LANDED.with(|c| *c.borrow_mut() = v); }
pub fn g_PENDING_WAITING() -> Py { ensure_init(); G_PENDING_WAITING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PENDING_WAITING(v: Py) { ensure_init(); G_PENDING_WAITING.with(|c| *c.borrow_mut() = v); }
pub fn g_PENDING_WHEN() -> Py { ensure_init(); G_PENDING_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PENDING_WHEN(v: Py) { ensure_init(); G_PENDING_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_WORDS() -> Py { ensure_init(); G_PERSON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_WORDS(v: Py) { ensure_init(); G_PERSON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PEST_WHO() -> Py { ensure_init(); G_PEST_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEST_WHO(v: Py) { ensure_init(); G_PEST_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_PETROL_WORDS() -> Py { ensure_init(); G_PETROL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PETROL_WORDS(v: Py) { ensure_init(); G_PETROL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_CONDITION_TROUBLE() -> Py { ensure_init(); G_PET_CONDITION_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_CONDITION_TROUBLE(v: Py) { ensure_init(); G_PET_CONDITION_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_LOW_REPLY() -> Py { ensure_init(); G_PET_LOW_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_LOW_REPLY(v: Py) { ensure_init(); G_PET_LOW_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_TROUBLE() -> Py { ensure_init(); G_PET_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_TROUBLE(v: Py) { ensure_init(); G_PET_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PICKS_A_MECHANISM() -> Py { ensure_init(); G_PICKS_A_MECHANISM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICKS_A_MECHANISM(v: Py) { ensure_init(); G_PICKS_A_MECHANISM.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_ONLY_WORDS() -> Py { ensure_init(); G_PICK_ONLY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_ONLY_WORDS(v: Py) { ensure_init(); G_PICK_ONLY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_WORDS() -> Py { ensure_init(); G_PLAIN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_WORDS(v: Py) { ensure_init(); G_PLAIN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN() -> Py { ensure_init(); G_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN(v: Py) { ensure_init(); G_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_DONE() -> Py { ensure_init(); G_PLAN_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_DONE(v: Py) { ensure_init(); G_PLAN_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_DONE_ACK() -> Py { ensure_init(); G_PLAN_DONE_ACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_DONE_ACK(v: Py) { ensure_init(); G_PLAN_DONE_ACK.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_KNOCKED() -> Py { ensure_init(); G_PLAN_KNOCKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_KNOCKED(v: Py) { ensure_init(); G_PLAN_KNOCKED.with(|c| *c.borrow_mut() = v); }
pub fn g_PLASTER_IT() -> Py { ensure_init(); G_PLASTER_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLASTER_IT(v: Py) { ensure_init(); G_PLASTER_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING() -> Py { ensure_init(); G_POINTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING(v: Py) { ensure_init(); G_POINTING.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_LINES() -> Py { ensure_init(); G_POINTS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_LINES(v: Py) { ensure_init(); G_POINTS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_POSITIVE() -> Py { ensure_init(); G_POSITIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSITIVE(v: Py) { ensure_init(); G_POSITIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_POSITIVE_CUES() -> Py { ensure_init(); G_POSITIVE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSITIVE_CUES(v: Py) { ensure_init(); G_POSITIVE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRAISE() -> Py { ensure_init(); G_PRAISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRAISE(v: Py) { ensure_init(); G_PRAISE.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANCY_BLEEDING() -> Py { ensure_init(); G_PREGNANCY_BLEEDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANCY_BLEEDING(v: Py) { ensure_init(); G_PREGNANCY_BLEEDING.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_LEFTOVER() -> Py { ensure_init(); G_PRESENCE_LEFTOVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_LEFTOVER(v: Py) { ensure_init(); G_PRESENCE_LEFTOVER.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT() -> Py { ensure_init(); G_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT(v: Py) { ensure_init(); G_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIVACY_ASK() -> Py { ensure_init(); G_PRIVACY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIVACY_ASK(v: Py) { ensure_init(); G_PRIVACY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIVACY_SHAPES() -> Py { ensure_init(); G_PRIVACY_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIVACY_SHAPES(v: Py) { ensure_init(); G_PRIVACY_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_PUNT() -> Py { ensure_init(); G_PUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUNT(v: Py) { ensure_init(); G_PUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_OPENERS() -> Py { ensure_init(); G_QUESTION_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_OPENERS(v: Py) { ensure_init(); G_QUESTION_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_WORDS() -> Py { ensure_init(); G_QUESTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_WORDS(v: Py) { ensure_init(); G_QUESTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIET_MODULES() -> Py { ensure_init(); G_QUIET_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIET_MODULES(v: Py) { ensure_init(); G_QUIET_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTE_LINES() -> Py { ensure_init(); G_QUOTE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTE_LINES(v: Py) { ensure_init(); G_QUOTE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_A_SKILL() -> Py { ensure_init(); G_RATES_A_SKILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_A_SKILL(v: Py) { ensure_init(); G_RATES_A_SKILL.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTIONS_CARE() -> Py { ensure_init(); G_REACTIONS_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTIONS_CARE(v: Py) { ensure_init(); G_REACTIONS_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTIONS_SAID() -> Py { ensure_init(); G_REACTIONS_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTIONS_SAID(v: Py) { ensure_init(); G_REACTIONS_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_DISBELIEF() -> Py { ensure_init(); G_REACTION_DISBELIEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_DISBELIEF(v: Py) { ensure_init(); G_REACTION_DISBELIEF.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_DISBELIEF_SAID() -> Py { ensure_init(); G_REACTION_DISBELIEF_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_DISBELIEF_SAID(v: Py) { ensure_init(); G_REACTION_DISBELIEF_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_LEAD() -> Py { ensure_init(); G_REACTION_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_LEAD(v: Py) { ensure_init(); G_REACTION_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_MISHAP() -> Py { ensure_init(); G_REACTION_MISHAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_MISHAP(v: Py) { ensure_init(); G_REACTION_MISHAP.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_MISHAP_SAID() -> Py { ensure_init(); G_REACTION_MISHAP_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_MISHAP_SAID(v: Py) { ensure_init(); G_REACTION_MISHAP_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_OUCH() -> Py { ensure_init(); G_REACTION_OUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_OUCH(v: Py) { ensure_init(); G_REACTION_OUCH.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_OUCH_SAID() -> Py { ensure_init(); G_REACTION_OUCH_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_OUCH_SAID(v: Py) { ensure_init(); G_REACTION_OUCH_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_RELIEF() -> Py { ensure_init(); G_REACTION_RELIEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_RELIEF(v: Py) { ensure_init(); G_REACTION_RELIEF.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_RELIEF_SAID() -> Py { ensure_init(); G_REACTION_RELIEF_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_RELIEF_SAID(v: Py) { ensure_init(); G_REACTION_RELIEF_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_REPEAT() -> Py { ensure_init(); G_REACTION_REPEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_REPEAT(v: Py) { ensure_init(); G_REACTION_REPEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_REPEAT_FIRST() -> Py { ensure_init(); G_REACTION_REPEAT_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_REPEAT_FIRST(v: Py) { ensure_init(); G_REACTION_REPEAT_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_WORD() -> Py { ensure_init(); G_REACTION_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_WORD(v: Py) { ensure_init(); G_REACTION_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_WORDS() -> Py { ensure_init(); G_REACTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_WORDS(v: Py) { ensure_init(); G_REACTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_REFUSED_REPLIES() -> Py { ensure_init(); G_REFUSED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFUSED_REPLIES(v: Py) { ensure_init(); G_REFUSED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATIONSHIP_DECISION() -> Py { ensure_init(); G_RELATIONSHIP_DECISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATIONSHIP_DECISION(v: Py) { ensure_init(); G_RELATIONSHIP_DECISION.with(|c| *c.borrow_mut() = v); }
pub fn g_RELIEF() -> Py { ensure_init(); G_RELIEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELIEF(v: Py) { ensure_init(); G_RELIEF.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_RULE() -> Py { ensure_init(); G_REPAIR_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_RULE(v: Py) { ensure_init(); G_REPAIR_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_WHETHER_LINES() -> Py { ensure_init(); G_REPAIR_WHETHER_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WHETHER_LINES(v: Py) { ensure_init(); G_REPAIR_WHETHER_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_FILLER() -> Py { ensure_init(); G_REPORT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_FILLER(v: Py) { ensure_init(); G_REPORT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_REQUEST_CUES() -> Py { ensure_init(); G_REQUEST_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REQUEST_CUES(v: Py) { ensure_init(); G_REQUEST_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_RESOLVED_AFTER() -> Py { ensure_init(); G_RESOLVED_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESOLVED_AFTER(v: Py) { ensure_init(); G_RESOLVED_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_RESOLVED_AFTER_THEM() -> Py { ensure_init(); G_RESOLVED_AFTER_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESOLVED_AFTER_THEM(v: Py) { ensure_init(); G_RESOLVED_AFTER_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_RETURNED_TO_A_THING() -> Py { ensure_init(); G_RETURNED_TO_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETURNED_TO_A_THING(v: Py) { ensure_init(); G_RETURNED_TO_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_RETURNED_TO_IT() -> Py { ensure_init(); G_RETURNED_TO_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETURNED_TO_IT(v: Py) { ensure_init(); G_RETURNED_TO_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_RETURNED_TO_IT_FIRST() -> Py { ensure_init(); G_RETURNED_TO_IT_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETURNED_TO_IT_FIRST(v: Py) { ensure_init(); G_RETURNED_TO_IT_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_RING_FOR_HELP() -> Py { ensure_init(); G_RING_FOR_HELP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RING_FOR_HELP(v: Py) { ensure_init(); G_RING_FOR_HELP.with(|c| *c.borrow_mut() = v); }
pub fn g_RING_THE_POLICE() -> Py { ensure_init(); G_RING_THE_POLICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RING_THE_POLICE(v: Py) { ensure_init(); G_RING_THE_POLICE.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE() -> Py { ensure_init(); G_ROUTINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE(v: Py) { ensure_init(); G_ROUTINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_NOT() -> Py { ensure_init(); G_ROUTINE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_NOT(v: Py) { ensure_init(); G_ROUTINE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_PHRASES() -> Py { ensure_init(); G_ROUTINE_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_PHRASES(v: Py) { ensure_init(); G_ROUTINE_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_WORDS() -> Py { ensure_init(); G_ROUTINE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_WORDS(v: Py) { ensure_init(); G_ROUTINE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_DETERMINER() -> Py { ensure_init(); G_RULED_OUT_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_DETERMINER(v: Py) { ensure_init(); G_RULED_OUT_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_RUMINATION_REPLIES() -> Py { ensure_init(); G_RUMINATION_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUMINATION_REPLIES(v: Py) { ensure_init(); G_RUMINATION_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_OUT_LINES() -> Py { ensure_init(); G_RUN_OUT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_OUT_LINES(v: Py) { ensure_init(); G_RUN_OUT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_SAFETY_WORDS() -> Py { ensure_init(); G_SAFETY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAFETY_WORDS(v: Py) { ensure_init(); G_SAFETY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_AGAIN() -> Py { ensure_init(); G_SAME_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_AGAIN(v: Py) { ensure_init(); G_SAME_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_AGAIN_WITH_THE_ANSWER() -> Py { ensure_init(); G_SAME_AGAIN_WITH_THE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_AGAIN_WITH_THE_ANSWER(v: Py) { ensure_init(); G_SAME_AGAIN_WITH_THE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_SCHOOL_TROUBLE() -> Py { ensure_init(); G_SCHOOL_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCHOOL_TROUBLE(v: Py) { ensure_init(); G_SCHOOL_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_SCHOOL_WHO() -> Py { ensure_init(); G_SCHOOL_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCHOOL_WHO(v: Py) { ensure_init(); G_SCHOOL_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_SCHOOL_WORDS() -> Py { ensure_init(); G_SCHOOL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCHOOL_WORDS(v: Py) { ensure_init(); G_SCHOOL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SECRETS_KEEP_SLIPPING() -> Py { ensure_init(); G_SECRETS_KEEP_SLIPPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECRETS_KEEP_SLIPPING(v: Py) { ensure_init(); G_SECRETS_KEEP_SLIPPING.with(|c| *c.borrow_mut() = v); }
pub fn g_SEEN_TO_VERB() -> Py { ensure_init(); G_SEEN_TO_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEEN_TO_VERB(v: Py) { ensure_init(); G_SEEN_TO_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_BLAME() -> Py { ensure_init(); G_SELF_BLAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_BLAME(v: Py) { ensure_init(); G_SELF_BLAME.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_BLAME_ASKS() -> Py { ensure_init(); G_SELF_BLAME_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_BLAME_ASKS(v: Py) { ensure_init(); G_SELF_BLAME_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_EATING_WORDS() -> Py { ensure_init(); G_SELF_EATING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_EATING_WORDS(v: Py) { ensure_init(); G_SELF_EATING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_FILLER() -> Py { ensure_init(); G_SELF_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_FILLER(v: Py) { ensure_init(); G_SELF_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_GROUP() -> Py { ensure_init(); G_SELF_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_GROUP(v: Py) { ensure_init(); G_SELF_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_JOB() -> Py { ensure_init(); G_SELF_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_JOB(v: Py) { ensure_init(); G_SELF_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_JUDGEMENT() -> Py { ensure_init(); G_SELF_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_JUDGEMENT(v: Py) { ensure_init(); G_SELF_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_NOBODY() -> Py { ensure_init(); G_SELF_NOBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_NOBODY(v: Py) { ensure_init(); G_SELF_NOBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_NOUN() -> Py { ensure_init(); G_SELF_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_NOUN(v: Py) { ensure_init(); G_SELF_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_SUBJECT() -> Py { ensure_init(); G_SELF_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_SUBJECT(v: Py) { ensure_init(); G_SELF_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SICKNESS() -> Py { ensure_init(); G_SICKNESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SICKNESS(v: Py) { ensure_init(); G_SICKNESS.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEBODY_HURT() -> Py { ensure_init(); G_SOMEBODY_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY_HURT(v: Py) { ensure_init(); G_SOMEBODY_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSES_DAY() -> Py { ensure_init(); G_SOMEONE_ELSES_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_DAY(v: Py) { ensure_init(); G_SOMEONE_ELSES_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSES_SHAME() -> Py { ensure_init(); G_SOMEONE_ELSES_SHAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_SHAME(v: Py) { ensure_init(); G_SOMEONE_ELSES_SHAME.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_GONE() -> Py { ensure_init(); G_SOMEONE_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_GONE(v: Py) { ensure_init(); G_SOMEONE_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_LEAVING() -> Py { ensure_init(); G_SOMEONE_LEAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_LEAVING(v: Py) { ensure_init(); G_SOMEONE_LEAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_SERIOUSLY_ILL() -> Py { ensure_init(); G_SOMEONE_SERIOUSLY_ILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_SERIOUSLY_ILL(v: Py) { ensure_init(); G_SOMEONE_SERIOUSLY_ILL.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMETHING_CAME_APART() -> Py { ensure_init(); G_SOMETHING_CAME_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMETHING_CAME_APART(v: Py) { ensure_init(); G_SOMETHING_CAME_APART.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_AMBIGUOUS() -> Py { ensure_init(); G_SORE_AMBIGUOUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_AMBIGUOUS(v: Py) { ensure_init(); G_SORE_AMBIGUOUS.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_HANDS() -> Py { ensure_init(); G_SORE_HANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_HANDS(v: Py) { ensure_init(); G_SORE_HANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_ORDINARY() -> Py { ensure_init(); G_SORE_ORDINARY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_ORDINARY(v: Py) { ensure_init(); G_SORE_ORDINARY.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_SERIOUS() -> Py { ensure_init(); G_SORE_SERIOUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_SERIOUS(v: Py) { ensure_init(); G_SORE_SERIOUS.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_WORDS() -> Py { ensure_init(); G_SORE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_WORDS(v: Py) { ensure_init(); G_SORE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SORRY_FOR_ASKING() -> Py { ensure_init(); G_SORRY_FOR_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORRY_FOR_ASKING(v: Py) { ensure_init(); G_SORRY_FOR_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_SORRY_ON_ITS_OWN() -> Py { ensure_init(); G_SORRY_ON_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORRY_ON_ITS_OWN(v: Py) { ensure_init(); G_SORRY_ON_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_SORTED_AND_NO_CALL_YET() -> Py { ensure_init(); G_SORTED_AND_NO_CALL_YET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORTED_AND_NO_CALL_YET(v: Py) { ensure_init(); G_SORTED_AND_NO_CALL_YET.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_CUES() -> Py { ensure_init(); G_STATE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_CUES(v: Py) { ensure_init(); G_STATE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_HERE_AFTER() -> Py { ensure_init(); G_STILL_HERE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_HERE_AFTER(v: Py) { ensure_init(); G_STILL_HERE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_STRETCH_OF_TIME() -> Py { ensure_init(); G_STRETCH_OF_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRETCH_OF_TIME(v: Py) { ensure_init(); G_STRETCH_OF_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_STUCK_AFTER_TROUBLE() -> Py { ensure_init(); G_STUCK_AFTER_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STUCK_AFTER_TROUBLE(v: Py) { ensure_init(); G_STUCK_AFTER_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_SUDDEN_LOSS() -> Py { ensure_init(); G_SUDDEN_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUDDEN_LOSS(v: Py) { ensure_init(); G_SUDDEN_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUMMONS_RANG() -> Py { ensure_init(); G_SUMMONS_RANG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_RANG(v: Py) { ensure_init(); G_SUMMONS_RANG.with(|c| *c.borrow_mut() = v); }
pub fn g_SUMMONS_SEEN() -> Py { ensure_init(); G_SUMMONS_SEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_SEEN(v: Py) { ensure_init(); G_SUMMONS_SEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_SUMMONS_WRITTEN() -> Py { ensure_init(); G_SUMMONS_WRITTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUMMONS_WRITTEN(v: Py) { ensure_init(); G_SUMMONS_WRITTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM() -> Py { ensure_init(); G_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM(v: Py) { ensure_init(); G_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM_COLD() -> Py { ensure_init(); G_SYMPTOM_COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM_COLD(v: Py) { ensure_init(); G_SYMPTOM_COLD.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM_WORDS() -> Py { ensure_init(); G_SYMPTOM_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM_WORDS(v: Py) { ensure_init(); G_SYMPTOM_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_CONFIRM() -> Py { ensure_init(); G_TAG_CONFIRM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_CONFIRM(v: Py) { ensure_init(); G_TAG_CONFIRM.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_UNSURE() -> Py { ensure_init(); G_TAG_UNSURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_UNSURE(v: Py) { ensure_init(); G_TAG_UNSURE.with(|c| *c.borrow_mut() = v); }
pub fn g_TASTES() -> Py { ensure_init(); G_TASTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TASTES(v: Py) { ensure_init(); G_TASTES.with(|c| *c.borrow_mut() = v); }
pub fn g_TASTE_GENERAL() -> Py { ensure_init(); G_TASTE_GENERAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TASTE_GENERAL(v: Py) { ensure_init(); G_TASTE_GENERAL.with(|c| *c.borrow_mut() = v); }
pub fn g_TELL_HARD_LINES() -> Py { ensure_init(); G_TELL_HARD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELL_HARD_LINES(v: Py) { ensure_init(); G_TELL_HARD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_TENANT_WHO() -> Py { ensure_init(); G_TENANT_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TENANT_WHO(v: Py) { ensure_init(); G_TENANT_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS() -> Py { ensure_init(); G_THANKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS(v: Py) { ensure_init(); G_THANKS.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_AFTER() -> Py { ensure_init(); G_THANKS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_AFTER(v: Py) { ensure_init(); G_THANKS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_ONLY() -> Py { ensure_init(); G_THANKS_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_ONLY(v: Py) { ensure_init(); G_THANKS_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_THEN_SAID() -> Py { ensure_init(); G_THANKS_THEN_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_THEN_SAID(v: Py) { ensure_init(); G_THANKS_THEN_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_WORDS() -> Py { ensure_init(); G_THANKS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_WORDS(v: Py) { ensure_init(); G_THANKS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THANK_WORDS() -> Py { ensure_init(); G_THANK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANK_WORDS(v: Py) { ensure_init(); G_THANK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BOTS_OWN_ANSWER() -> Py { ensure_init(); G_THE_BOTS_OWN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BOTS_OWN_ANSWER(v: Py) { ensure_init(); G_THE_BOTS_OWN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_CALL_STANDS() -> Py { ensure_init(); G_THE_CALL_STANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_CALL_STANDS(v: Py) { ensure_init(); G_THE_CALL_STANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_CALL_STANDS_ANYWAY() -> Py { ensure_init(); G_THE_CALL_STANDS_ANYWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_CALL_STANDS_ANYWAY(v: Py) { ensure_init(); G_THE_CALL_STANDS_ANYWAY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DAY() -> Py { ensure_init(); G_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DAY(v: Py) { ensure_init(); G_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DAY_AHEAD() -> Py { ensure_init(); G_THE_DAY_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DAY_AHEAD(v: Py) { ensure_init(); G_THE_DAY_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FIRST_HOUR_AND_A_CHILD() -> Py { ensure_init(); G_THE_FIRST_HOUR_AND_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FIRST_HOUR_AND_A_CHILD(v: Py) { ensure_init(); G_THE_FIRST_HOUR_AND_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_GROUND_OPENS() -> Py { ensure_init(); G_THE_GROUND_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_GROUND_OPENS(v: Py) { ensure_init(); G_THE_GROUND_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_MOMENT_IS_THEIRS() -> Py { ensure_init(); G_THE_MOMENT_IS_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_MOMENT_IS_THEIRS(v: Py) { ensure_init(); G_THE_MOMENT_IS_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PLACE_ASK() -> Py { ensure_init(); G_THE_PLACE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PLACE_ASK(v: Py) { ensure_init(); G_THE_PLACE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PLACE_POOLS() -> Py { ensure_init(); G_THE_PLACE_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PLACE_POOLS(v: Py) { ensure_init(); G_THE_PLACE_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_AFTER() -> Py { ensure_init(); G_THING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_AFTER(v: Py) { ensure_init(); G_THING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_STATES() -> Py { ensure_init(); G_THING_STATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_STATES(v: Py) { ensure_init(); G_THING_STATES.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_SUBJECT() -> Py { ensure_init(); G_THING_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_SUBJECT(v: Py) { ensure_init(); G_THING_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAD() -> Py { ensure_init(); G_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAD(v: Py) { ensure_init(); G_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAD_PEOPLE() -> Py { ensure_init(); G_THREAD_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAD_PEOPLE(v: Py) { ensure_init(); G_THREAD_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAD_TURNS() -> Py { ensure_init(); G_THREAD_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAD_TURNS(v: Py) { ensure_init(); G_THREAD_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_THRESHOLD_CROSSED() -> Py { ensure_init(); G_THRESHOLD_CROSSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THRESHOLD_CROSSED(v: Py) { ensure_init(); G_THRESHOLD_CROSSED.with(|c| *c.borrow_mut() = v); }
pub fn g_TIGHT_MONEY() -> Py { ensure_init(); G_TIGHT_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIGHT_MONEY(v: Py) { ensure_init(); G_TIGHT_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_TIGHT_MONEY_MORE() -> Py { ensure_init(); G_TIGHT_MONEY_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIGHT_MONEY_MORE(v: Py) { ensure_init(); G_TIGHT_MONEY_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_AHEAD() -> Py { ensure_init(); G_TIPS_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_AHEAD(v: Py) { ensure_init(); G_TIPS_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_CLAUSE() -> Py { ensure_init(); G_TIPS_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_CLAUSE(v: Py) { ensure_init(); G_TIPS_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_LINES() -> Py { ensure_init(); G_TIPS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_LINES(v: Py) { ensure_init(); G_TIPS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_OWNER() -> Py { ensure_init(); G_TIPS_OWNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_OWNER(v: Py) { ensure_init(); G_TIPS_OWNER.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_THING() -> Py { ensure_init(); G_TIPS_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_THING(v: Py) { ensure_init(); G_TIPS_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_UNIT() -> Py { ensure_init(); G_TIPS_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_UNIT(v: Py) { ensure_init(); G_TIPS_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_VERB() -> Py { ensure_init(); G_TIPS_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_VERB(v: Py) { ensure_init(); G_TIPS_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_TIRED() -> Py { ensure_init(); G_TIRED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIRED(v: Py) { ensure_init(); G_TIRED.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_MUCH() -> Py { ensure_init(); G_TOO_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_MUCH(v: Py) { ensure_init(); G_TOO_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_MUCH_PHRASES() -> Py { ensure_init(); G_TOO_MUCH_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_MUCH_PHRASES(v: Py) { ensure_init(); G_TOO_MUCH_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADER_WORDS() -> Py { ensure_init(); G_TRADER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADER_WORDS(v: Py) { ensure_init(); G_TRADER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADES_WHO() -> Py { ensure_init(); G_TRADES_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADES_WHO(v: Py) { ensure_init(); G_TRADES_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADE_OFF_LEAN() -> Py { ensure_init(); G_TRADE_OFF_LEAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADE_OFF_LEAN(v: Py) { ensure_init(); G_TRADE_OFF_LEAN.with(|c| *c.borrow_mut() = v); }
pub fn g_TREATMENT_HARM() -> Py { ensure_init(); G_TREATMENT_HARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TREATMENT_HARM(v: Py) { ensure_init(); G_TREATMENT_HARM.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_HELPED_ELSEWHERE() -> Py { ensure_init(); G_TRIED_HELPED_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_HELPED_ELSEWHERE(v: Py) { ensure_init(); G_TRIED_HELPED_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_HELPED_REPLIES() -> Py { ensure_init(); G_TRIED_HELPED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_HELPED_REPLIES(v: Py) { ensure_init(); G_TRIED_HELPED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_IT_REPLIES() -> Py { ensure_init(); G_TRIED_IT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_IT_REPLIES(v: Py) { ensure_init(); G_TRIED_IT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_NO_CHANGE_ELSEWHERE() -> Py { ensure_init(); G_TRIED_NO_CHANGE_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_NO_CHANGE_ELSEWHERE(v: Py) { ensure_init(); G_TRIED_NO_CHANGE_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_NO_CHANGE_REPLIES() -> Py { ensure_init(); G_TRIED_NO_CHANGE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_NO_CHANGE_REPLIES(v: Py) { ensure_init(); G_TRIED_NO_CHANGE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_SORTED_REPLIES() -> Py { ensure_init(); G_TRIED_SORTED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_SORTED_REPLIES(v: Py) { ensure_init(); G_TRIED_SORTED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_WORSE_ELSEWHERE() -> Py { ensure_init(); G_TRIED_WORSE_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_WORSE_ELSEWHERE(v: Py) { ensure_init(); G_TRIED_WORSE_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_WORSE_REPLIES() -> Py { ensure_init(); G_TRIED_WORSE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_WORSE_REPLIES(v: Py) { ensure_init(); G_TRIED_WORSE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TROUBLE_CUES() -> Py { ensure_init(); G_TROUBLE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TROUBLE_CUES(v: Py) { ensure_init(); G_TROUBLE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN() -> Py { ensure_init(); G_UNKNOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN(v: Py) { ensure_init(); G_UNKNOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_ABOUT_A_PET() -> Py { ensure_init(); G_UNKNOWN_ABOUT_A_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_ABOUT_A_PET(v: Py) { ensure_init(); G_UNKNOWN_ABOUT_A_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_ABOUT_SOMEONE() -> Py { ensure_init(); G_UNKNOWN_ABOUT_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_ABOUT_SOMEONE(v: Py) { ensure_init(); G_UNKNOWN_ABOUT_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_DECIDE() -> Py { ensure_init(); G_UNKNOWN_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_DECIDE(v: Py) { ensure_init(); G_UNKNOWN_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_HEALTH() -> Py { ensure_init(); G_UNKNOWN_HEALTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_HEALTH(v: Py) { ensure_init(); G_UNKNOWN_HEALTH.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_MONEY() -> Py { ensure_init(); G_UNKNOWN_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_MONEY(v: Py) { ensure_init(); G_UNKNOWN_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_REALISTIC() -> Py { ensure_init(); G_UNKNOWN_REALISTIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_REALISTIC(v: Py) { ensure_init(); G_UNKNOWN_REALISTIC.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_REALISTIC_CARE() -> Py { ensure_init(); G_UNKNOWN_REALISTIC_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_REALISTIC_CARE(v: Py) { ensure_init(); G_UNKNOWN_REALISTIC_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_TOO_LATE() -> Py { ensure_init(); G_UNKNOWN_TOO_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_TOO_LATE(v: Py) { ensure_init(); G_UNKNOWN_TOO_LATE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WEIRD_BARE() -> Py { ensure_init(); G_UNKNOWN_WEIRD_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WEIRD_BARE(v: Py) { ensure_init(); G_UNKNOWN_WEIRD_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WHO() -> Py { ensure_init(); G_UNKNOWN_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WHO(v: Py) { ensure_init(); G_UNKNOWN_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WORD() -> Py { ensure_init(); G_UNKNOWN_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WORD(v: Py) { ensure_init(); G_UNKNOWN_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WORD_NAMED() -> Py { ensure_init(); G_UNKNOWN_WORD_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WORD_NAMED(v: Py) { ensure_init(); G_UNKNOWN_WORD_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WORRY() -> Py { ensure_init(); G_UNKNOWN_WORRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WORRY(v: Py) { ensure_init(); G_UNKNOWN_WORRY.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_ANNOY() -> Py { ensure_init(); G_VERDICT_ANNOY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ANNOY(v: Py) { ensure_init(); G_VERDICT_ANNOY.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_BAD() -> Py { ensure_init(); G_VERDICT_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_BAD(v: Py) { ensure_init(); G_VERDICT_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_FIRST() -> Py { ensure_init(); G_VERDICT_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_FIRST(v: Py) { ensure_init(); G_VERDICT_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_HEAVY() -> Py { ensure_init(); G_VERDICT_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_HEAVY(v: Py) { ensure_init(); G_VERDICT_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_LUCK() -> Py { ensure_init(); G_VERDICT_LUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_LUCK(v: Py) { ensure_init(); G_VERDICT_LUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_NUMB() -> Py { ensure_init(); G_VERDICT_NUMB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_NUMB(v: Py) { ensure_init(); G_VERDICT_NUMB.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_QUEUE() -> Py { ensure_init(); G_VERDICT_QUEUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_QUEUE(v: Py) { ensure_init(); G_VERDICT_QUEUE.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_ROUGH() -> Py { ensure_init(); G_VERDICT_ROUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ROUGH(v: Py) { ensure_init(); G_VERDICT_ROUGH.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_SETBACK() -> Py { ensure_init(); G_VERDICT_SETBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_SETBACK(v: Py) { ensure_init(); G_VERDICT_SETBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_SHAME() -> Py { ensure_init(); G_VERDICT_SHAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_SHAME(v: Py) { ensure_init(); G_VERDICT_SHAME.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_WORD() -> Py { ensure_init(); G_VERDICT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_WORD(v: Py) { ensure_init(); G_VERDICT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_VET_WHO() -> Py { ensure_init(); G_VET_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VET_WHO(v: Py) { ensure_init(); G_VET_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_AFTER_A_DEATH() -> Py { ensure_init(); G_VIEW_AFTER_A_DEATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_AFTER_A_DEATH(v: Py) { ensure_init(); G_VIEW_AFTER_A_DEATH.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_ASK_LINES() -> Py { ensure_init(); G_VIEW_ASK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_ASK_LINES(v: Py) { ensure_init(); G_VIEW_ASK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_COMPANY() -> Py { ensure_init(); G_VIEW_COMPANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_COMPANY(v: Py) { ensure_init(); G_VIEW_COMPANY.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_DECIDE() -> Py { ensure_init(); G_VIEW_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_DECIDE(v: Py) { ensure_init(); G_VIEW_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_HANDBACK() -> Py { ensure_init(); G_VIEW_HANDBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_HANDBACK(v: Py) { ensure_init(); G_VIEW_HANDBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_LEAD_IN() -> Py { ensure_init(); G_VIEW_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_LEAD_IN(v: Py) { ensure_init(); G_VIEW_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_LINES() -> Py { ensure_init(); G_VIEW_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_LINES(v: Py) { ensure_init(); G_VIEW_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_MIRROR_LINES() -> Py { ensure_init(); G_VIEW_MIRROR_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_MIRROR_LINES(v: Py) { ensure_init(); G_VIEW_MIRROR_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_NO_SUBJECT() -> Py { ensure_init(); G_VIEW_NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_NO_SUBJECT(v: Py) { ensure_init(); G_VIEW_NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_ONLY_WORDS() -> Py { ensure_init(); G_VIEW_ONLY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_ONLY_WORDS(v: Py) { ensure_init(); G_VIEW_ONLY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_ON_GOOD_NEWS() -> Py { ensure_init(); G_VIEW_ON_GOOD_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_ON_GOOD_NEWS(v: Py) { ensure_init(); G_VIEW_ON_GOOD_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_PRONOUNS() -> Py { ensure_init(); G_VISIT_PRONOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_PRONOUNS(v: Py) { ensure_init(); G_VISIT_PRONOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_WALKED_INTO_POOL() -> Py { ensure_init(); G_WALKED_INTO_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WALKED_INTO_POOL(v: Py) { ensure_init(); G_WALKED_INTO_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_WASNT_THERE() -> Py { ensure_init(); G_WASNT_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WASNT_THERE(v: Py) { ensure_init(); G_WASNT_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_SOMETHING_WRONG() -> Py { ensure_init(); G_WHERE_SOMETHING_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_SOMETHING_WRONG(v: Py) { ensure_init(); G_WHERE_SOMETHING_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_LIST_AFTER() -> Py { ensure_init(); G_WHOLE_LIST_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_LIST_AFTER(v: Py) { ensure_init(); G_WHOLE_LIST_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_TO_ASK() -> Py { ensure_init(); G_WHO_TO_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_TO_ASK(v: Py) { ensure_init(); G_WHO_TO_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_AFTER_TASTE() -> Py { ensure_init(); G_WHY_AFTER_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_AFTER_TASTE(v: Py) { ensure_init(); G_WHY_AFTER_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g_WILD_BIRDS() -> Py { ensure_init(); G_WILD_BIRDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WILD_BIRDS(v: Py) { ensure_init(); G_WILD_BIRDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WIN_AND_A_COST() -> Py { ensure_init(); G_WIN_AND_A_COST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WIN_AND_A_COST(v: Py) { ensure_init(); G_WIN_AND_A_COST.with(|c| *c.borrow_mut() = v); }
pub fn g_WIN_IN_A_SILENCE() -> Py { ensure_init(); G_WIN_IN_A_SILENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WIN_IN_A_SILENCE(v: Py) { ensure_init(); G_WIN_IN_A_SILENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_YOU_AFTER() -> Py { ensure_init(); G_WITH_YOU_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_YOU_AFTER(v: Py) { ensure_init(); G_WITH_YOU_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_WORDING_LINES() -> Py { ensure_init(); G_WORDING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORDING_LINES(v: Py) { ensure_init(); G_WORDING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WORDING_SOMEONE() -> Py { ensure_init(); G_WORDING_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORDING_SOMEONE(v: Py) { ensure_init(); G_WORDING_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_ANYWHERE() -> Py { ensure_init(); G_WORD_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_ANYWHERE(v: Py) { ensure_init(); G_WORD_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_FIXES() -> Py { ensure_init(); G_WORD_FIXES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_FIXES(v: Py) { ensure_init(); G_WORD_FIXES.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_ABOUT() -> Py { ensure_init(); G_WORK_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_ABOUT(v: Py) { ensure_init(); G_WORK_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_WHO() -> Py { ensure_init(); G_WORK_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_WHO(v: Py) { ensure_init(); G_WORK_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_WORRY_FILLER() -> Py { ensure_init(); G_WORRY_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORRY_FILLER(v: Py) { ensure_init(); G_WORRY_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_WORRY_WORDS() -> Py { ensure_init(); G_WORRY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORRY_WORDS(v: Py) { ensure_init(); G_WORRY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WOULD_HAVE_BEEN() -> Py { ensure_init(); G_WOULD_HAVE_BEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WOULD_HAVE_BEEN(v: Py) { ensure_init(); G_WOULD_HAVE_BEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_YOURS() -> Py { ensure_init(); G_YOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YOURS(v: Py) { ensure_init(); G_YOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_YOU_SUBJECTS() -> Py { ensure_init(); G_YOU_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YOU_SUBJECTS(v: Py) { ensure_init(); G_YOU_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_ABOUT_THE_JOINT() -> Py { ensure_init(); G__ASKS_ABOUT_THE_JOINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_ABOUT_THE_JOINT(v: Py) { ensure_init(); G__ASKS_ABOUT_THE_JOINT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON() -> Py { ensure_init(); G__A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON(v: Py) { ensure_init(); G__A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SOCIAL_THING() -> Py { ensure_init(); G__A_SOCIAL_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SOCIAL_THING(v: Py) { ensure_init(); G__A_SOCIAL_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__BOUNDED() -> Py { ensure_init(); G__BOUNDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BOUNDED(v: Py) { ensure_init(); G__BOUNDED.with(|c| *c.borrow_mut() = v); }
pub fn g__CHOOSING_BETWEEN_TWO() -> Py { ensure_init(); G__CHOOSING_BETWEEN_TWO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHOOSING_BETWEEN_TWO(v: Py) { ensure_init(); G__CHOOSING_BETWEEN_TWO.with(|c| *c.borrow_mut() = v); }
pub fn g__COPULA() -> Py { ensure_init(); G__COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COPULA(v: Py) { ensure_init(); G__COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g__CREATIVE() -> Py { ensure_init(); G__CREATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CREATIVE(v: Py) { ensure_init(); G__CREATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_WORDS() -> Py { ensure_init(); G__DAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_WORDS(v: Py) { ensure_init(); G__DAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__ECHO_FILLER() -> Py { ensure_init(); G__ECHO_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ECHO_FILLER(v: Py) { ensure_init(); G__ECHO_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__ISSUER() -> Py { ensure_init(); G__ISSUER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ISSUER(v: Py) { ensure_init(); G__ISSUER.with(|c| *c.borrow_mut() = v); }
pub fn g__LANDED() -> Py { ensure_init(); G__LANDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LANDED(v: Py) { ensure_init(); G__LANDED.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_NOUN() -> Py { ensure_init(); G__MONEY_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_NOUN(v: Py) { ensure_init(); G__MONEY_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AN_OWN_THING() -> Py { ensure_init(); G__NOT_AN_OWN_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AN_OWN_THING(v: Py) { ensure_init(); G__NOT_AN_OWN_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_DAY_OF_THEIR_OWN() -> Py { ensure_init(); G__NO_DAY_OF_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_DAY_OF_THEIR_OWN(v: Py) { ensure_init(); G__NO_DAY_OF_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENING_QUESTIONS() -> Py { ensure_init(); G__OPENING_QUESTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENING_QUESTIONS(v: Py) { ensure_init(); G__OPENING_QUESTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__START_ASK() -> Py { ensure_init(); G__START_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__START_ASK(v: Py) { ensure_init(); G__START_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_AFTER_AM() -> Py { ensure_init(); G__SUBJECT_AFTER_AM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_AFTER_AM(v: Py) { ensure_init(); G__SUBJECT_AFTER_AM.with(|c| *c.borrow_mut() = v); }
pub fn g__THING_DONE_LINES() -> Py { ensure_init(); G__THING_DONE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THING_DONE_LINES(v: Py) { ensure_init(); G__THING_DONE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__THREAD_PEOPLE() -> Py { ensure_init(); G__THREAD_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THREAD_PEOPLE(v: Py) { ensure_init(); G__THREAD_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__VERDICT_DET() -> Py { ensure_init(); G__VERDICT_DET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERDICT_DET(v: Py) { ensure_init(); G__VERDICT_DET.with(|c| *c.borrow_mut() = v); }
pub fn g__VERDICT_LEAD() -> Py { ensure_init(); G__VERDICT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERDICT_LEAD(v: Py) { ensure_init(); G__VERDICT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__WAS() -> Py { ensure_init(); G__WAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAS(v: Py) { ensure_init(); G__WAS.with(|c| *c.borrow_mut() = v); }
pub fn g__WOUND() -> Py { ensure_init(); G__WOUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WOUND(v: Py) { ensure_init(); G__WOUND.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The tables the composer's safety net reads from.\n\nPools, frozensets and word lists with no logic of their own, kept in one file so that\n`fallback.py` - the readings that decide which of them answers - stays a size a person can work\nin. Everything here is imported back into `fallback` under the same name, so `fallback.POOL` and\n`from .fallback import POOL` are unchanged, and the pools are the same objects they were.\n");
    s_FILLERS(call_builtin("frozenset", &[py_tuple(vec![py_str("actually"), py_str("really"), py_str("exactly"), py_str("even"), py_str("just"), py_str("please"), py_str("basically"), py_str("honestly"), py_str("literally"), py_str("anyway"), py_str("still"), py_str("perhaps"), py_str("maybe")])]).unwrap_or_else(|e| exc::raise(e)));
    s__BOUNDED(py_dict(vec![]));
    s_GREETING(py_tuple(vec![py_str("Hey! What's on your mind?"), py_str("Hi there — what can I help with?"), py_str("Hello! Good to see you. What are you thinking about?"), py_str("Hey! I'm here. What would you like to talk about?"), py_str("Hi! What's up?")]));
    s_THANKS(py_tuple(vec![py_str("Any time!"), py_str("You're welcome — glad that helped."), py_str("No problem at all. Shout if you need anything else."), py_str("Happy to help.")]));
    s_UNKNOWN_DECIDE(py_tuple(vec![py_str("That's your call, not mine — I don't know enough about your side of it. Tell me what you're weighing and I'll help you pick."), py_str("Can't call that one for you, sorry. Say what you'd lose either way and I'll lay it out with you."), py_str("Not mine to decide, sorry. Tell me what matters most here and I'll help you think it through.")]));
    s_UNKNOWN_MONEY(py_tuple(vec![py_str("Not a call I can make for you, sorry: anything money-shaped wants someone who can see the whole picture, and free advice is easy to find - Citizens Advice, a bank adviser, a broker. Tell me what you are weighing anyway and I'll help you lay it out."), py_str("That is an adviser's call rather than mine, sorry - a free debt or money advice service will know your rules and your numbers. Say what matters most and I'll help you get it straight before you ring them."), py_str("I don't know enough about your money to call it, and a guess is the worst thing here. What would each choice cost you, and what would it leave you free to do?")]));
    s_HANDS_BACK(py_tuple(vec![py_str("Happy to - what are you choosing between?"), py_str("Then give me the choice: what's on the table?"), py_str("I'll make the call if you tell me the options - what are they?")]));
    s_PLASTER_IT(py_tuple(vec![py_str("Yes - cover it with a plaster and leave it alone: that keeps it clean and stops your hand going back to it. Change it daily, and get it looked at if the skin around it goes red and spreading."), py_str("Cover it and leave it be. A plaster does more than air does for a cut or a graze; change it each day and see a pharmacist if it starts to look angry.")]));
    s_SORE_HANDS(py_tuple(vec![py_str("Leave it alone - don't pick or scratch it. Squeezing pushes the infection deeper and it comes back worse, and keeping it clean does more than pressure does; it settles in a few days on its own."), py_str("Don't squeeze, pick or scratch it. A warm flannel for ten minutes and clean hands will do more than pressure will; if it goes hot, red and spreading, a pharmacist can look at it today.")]));
    s_GO_TO_BE_SEEN(py_tuple(vec![py_str("Go, and have someone drive you rather than driving yourself. Take your medicine list if you have one — they ask for it first."), py_str("Go now rather than waiting to see if it settles. Whether something needs closing, strapping or an X-ray is their call, not one to make from here."), py_str("Go, and if you get worse on the way — breathing trouble, heavy bleeding, or you cannot stay awake — ring 999 rather than carrying on.")]));
    s_RING_FOR_HELP(py_tuple(vec![py_str("Yes — ring 999 or 911 now, and stay on the line while you talk. If it turns out to be less serious than you thought, they would far rather hear from you than not."), py_str("Ring now rather than waiting to be sure: 999 or 911, say where you are and what has happened, and keep the person talking while you dial."), py_str("Call 999 or 911 straight away. What happens in the next few minutes matters more than anything either of us can work out from here.")]));
    s_RING_THE_POLICE(py_tuple(vec![py_str("Ring 999 now, for anything happening as you read this or for anyone in danger. Once it is over, ring 101 instead. Either way the call is free and they would rather hear it."), py_str("999 now, for anything happening or anyone in danger. 101 for what has already happened. You can say you are not sure it counts — deciding that is their job, not yours.")]));
    s__A_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("him"), py_str("her"), py_str("them"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("sister"), py_str("brother"), py_str("friend"), py_str("friends"), py_str("boss"), py_str("ex"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("girlfriend"), py_str("boyfriend"), py_str("girl"), py_str("guy"), py_str("colleague"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("nanny"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("neighbour"), py_str("neighbor"), py_str("landlord"), py_str("landlady"), py_str("teacher"), py_str("parents"), py_str("family"), py_str("son"), py_str("daughter"), py_str("flatmate"), py_str("roommate"), py_str("manager"), py_str("someone"), py_str("anyone"), py_str("his"), py_str("hers"), py_str("theirs"), py_str("mums"), py_str("moms"), py_str("dads"), py_str("sisters"), py_str("brothers"), py_str("friends")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SOCIAL_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("wedding"), py_str("party"), py_str("funeral"), py_str("birthday"), py_str("dinner"), py_str("drinks"), py_str("christening"), py_str("reunion"), py_str("barbecue"), py_str("bbq"), py_str("gig"), py_str("christmas"), py_str("housewarming"), py_str("wake"), py_str("graduation"), py_str("do"), py_str("meal")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTACT_ANSWER(py_tuple(vec![py_str("Send it. One short line, nothing to explain, and if you would still be glad you had sent it tomorrow, that is your answer. Tell me what you would write and I will make it shorter."), py_str("Reply, and keep it to the one thing you want back. Most of the weight here is in the drafting, not in the message — tell me what you are trying to say and I will tighten it."), py_str("Send the short version. The paragraph you keep rewriting is the one that never goes.")]));
    s_CALL_ANSWER(py_tuple(vec![py_str("Ring them. The wait is worse than the call nearly every time, and you can keep it short once you are through — say the one thing you actually want to say."), py_str("Call, and plan the first sentence only. You do not owe anyone the whole conversation in one go, and you can always say you have to go.")]));
    s_GO_ANSWER(py_tuple(vec![py_str("Go. If you would still be glad you went tomorrow, go — an evening out costs less than the wondering about it."), py_str("Go, and give yourself an exit: turn up on time, leave when you have had enough, and you get the bit that matters without the long haul."), py_str("Yes, go — and give yourself a job while you are there (drinks, photos, helping set up). It is the standing around that makes those things hard, not the people.")]));
    s_ASK_OUT_ANSWER(py_tuple(vec![py_str("Ask. The worst case is a no, which you do not have yet, and a plain \"fancy a drink sometime?\" gives them an easy way to say either thing. One line, then let it go."), py_str("Ask, and keep it small and specific — a drink, a coffee, a thing you are already going to. Low stakes is what makes it easy to say yes to.")]));
    s_APOLOGY_ANSWER(py_tuple(vec![py_str("Say it once, plainly, and let it stand: \"I'm sorry about the other day\" lands better than a paragraph of explaining."), py_str("Yes — what you did, that you are sorry, and nothing about what they did back. Short and specific is what makes an apology believable.")]));
    s_UNKNOWN_WORD(py_tuple(vec![py_str("I can draft that: tell me who it's for and what you want them to do, and I'll write something you can send."), py_str("I'm not sure what to say without the details, but I can write it: give me who it's for and the one thing you need from them.")]));
    s_UNKNOWN_WORD_NAMED(py_tuple(vec![py_str("I can draft that. Give me the gist in your own words and how you want it to land - plain, apologetic, firm - and I'll write something you can send."), py_str("Happy to write it for you. Tell me what you want them to know or do, and roughly what tone, and I'll put a version together you can edit."), py_str("I can write that. A line or two of what you want to say is enough to start, and you can cut it down or soften it once you see it.")]));
    s_UNKNOWN_HEALTH(py_tuple(vec![py_str("Not one I can call for you, sorry: I cannot examine anything from here, and this is the kind of question where a guess does real harm. A GP, a pharmacist or 111 will tell you in a few minutes, and if it is getting worse, do not wait for an appointment."), py_str("I cannot weigh that one safely, sorry. Ring 111 or your surgery and describe it as you did to me - they will say whether it needs looking at today, and it costs one phone call.")]));
    s_BODY_IS_NOT_A_MACHINE(py_tuple(vec![py_str("If only it were that simple. Rest it, keep moving gently, and if it has not settled in a week or two, get it looked at rather than working on it yourself."), py_str("The parts are not sold separately, sadly. Give it a week or two of gentle movement, and have someone look at it if it is no better by then."), py_str("I would not start taking yourself apart. Time and gentle movement do most of it, and a GP is the one to see if it is still the same in a week or two.")]));
    s_DITHER_DECIDE(py_tuple(vec![py_str("What's pulling you each way? Even a rough version will do, and I'll lay both out with you."), py_str("Which way are you leaning when nobody's watching? Start there, then tell me the bit that worries you."), py_str("Say both versions out loud, the one where you do it and the one where you don't, and I'll help you find the one you'd regret less."), py_str("What would have to be true for the answer to be yes? That is usually where the snag is."), py_str("What's the thing you would lose if you said yes? Name it and we can look at it properly.")]));
    s_DITHER_TWO_WAY_LINES(slice_or_raise(&g_DITHER_DECIDE(), &Py::None, &Py::Int(3), &Py::None));
    s_A_FREE_TIME_ASK(py_tuple(vec![py_str("How long have you got, and are you after out or in?"), py_str("What are you in the mood for: out of the house, a screen, or something with your hands?"), py_str("What would make it a good afternoon - out somewhere, or something at home?")]));
    s_DITHER_OPEN_LINES(py_tuple(vec![py_str("What's the part of it that won't move? Start there and the rest tends to come loose."), py_str("Say what you'd want if nothing was in the way, and we can work back to the first step."), py_str("Which bit of it is heaviest? Take that one first rather than the whole thing at once.")]));
    s_STUCK_AFTER_TROUBLE(py_tuple(vec![py_str("What have you tried so far? What has not been tried yet is usually where the next step is."), py_str("Which part of it is hardest right now? Take that part on its own and leave the rest for now."), py_str("Is anyone helping you with it, or are you carrying this on your own?"), py_str("What would make today a bit easier? Start with the smallest part of that.")]));
    s_RUN_OUT_LINES(py_tuple(vec![py_str("What is the thing itself? Say it in your own words and I will tell you what I can do with it and what I cannot."), py_str("How long has this been going on? Weeks of it and one bad day are different things to plan for, and the answer changes with it."), py_str("What would make tomorrow even slightly easier? Not the whole of it - just the next day."), py_str("Does anyone close to you know how bad it is at the moment? Tell one of them too, not only me.")]));
    s_DITHER_ABOUT_A_PERSON(py_tuple(vec![py_str("What is going on with {who} at the moment? Start with the bit that worries you most."), py_str("What would you most want to be different for {who}? We can work back from that to the step you can take."), py_str("What has {who} said or done that has you stuck? That is usually where the answer is."), py_str("What is the hardest part of it with {who} right now? Take that piece on its own.")]));
    s_HOW_TO_DECIDE_LINES(py_tuple(vec![py_str("Two questions settle most of these: what each one costs you, and which you would regret not taking. Tell me the two and we will run them."), py_str("Write down what you would gain and what you would give up for each, then ask which loss you could live with. Give me both sides and I will lay them out with you."), py_str("Ask what has to be true for each to work out, and which of those you can check before you commit. What are the two options?"), py_str("Rule out the one you could not undo if it went wrong, then choose between what is left on what you want to be doing in a year. What are you choosing between?")]));
    s_MISSED_STEP_LINES(py_tuple(vec![py_str("A missed one is a pause, not a restart: pick up at the step you were on, and if it turned into a long gap, go back a step so the first one back is easy. What are you partway through?"), py_str("Missing it does not undo the work. What matters is the next one, at whatever level still feels easy, not doubling up to make up for the gap. What is the plan you are on?"), py_str("Most plans forgive one or two misses - it is the long gap that costs you - so the next one is what counts, carried on from where you stopped rather than started again. What is it you are partway through?"), py_str("The usual advice is to carry on from where you stopped, or one step back after a long gap, and not to squeeze in the ones you missed. Tell me what you are working through and I will say where that leaves you.")]));
    s_HOW_TO_DECIDE_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("now"), py_str("then"), py_str("here"), py_str("please"), py_str("for"), py_str("me"), py_str("myself"), py_str("properly"), py_str("better"), py_str("right"), py_str("next"), py_str("first"), py_str("about"), py_str("on"), py_str("at"), py_str("it"), py_str("this"), py_str("that"), py_str("one"), py_str("thing"), py_str("things"), py_str("so")])]).unwrap_or_else(|e| exc::raise(e)));
    s__CREATIVE(call_builtin("frozenset", &[py_tuple(vec![py_str("poem"), py_str("poems"), py_str("poetry"), py_str("haiku"), py_str("story"), py_str("stories"), py_str("song"), py_str("songs"), py_str("rhyme"), py_str("rhymes"), py_str("joke"), py_str("jokes"), py_str("speech"), py_str("speeches"), py_str("limerick")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEALTH_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("doctor"), py_str("doctors"), py_str("gp"), py_str("gps"), py_str("nurse"), py_str("hospital"), py_str("pharmacist"), py_str("111"), py_str("999"), py_str("a&e"), py_str("casualty"), py_str("clinic"), py_str("surgery"), py_str("vitamin"), py_str("vitamins"), py_str("medicine"), py_str("medication"), py_str("pill"), py_str("pills"), py_str("tablet"), py_str("tablets"), py_str("prescription"), py_str("dose"), py_str("dosage"), py_str("syringed"), py_str("infection"), py_str("symptom"), py_str("symptoms"), py_str("pain"), py_str("painful"), py_str("swollen"), py_str("unwell"), py_str("ibuprofen"), py_str("paracetamol"), py_str("aspirin"), py_str("painkiller"), py_str("painkillers"), py_str("antibiotic"), py_str("antibiotics"), py_str("antihistamine"), py_str("antihistamines"), py_str("inhaler"), py_str("insulin"), py_str("steroid"), py_str("steroids"), py_str("ointment"), py_str("allergic"), py_str("allergy"), py_str("reaction"), py_str("vaccine"), py_str("vaccines"), py_str("vaccination"), py_str("jab"), py_str("jabs"), py_str("flu")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SORE_SERIOUS(call_builtin("frozenset", &[py_tuple(vec![py_str("boil"), py_str("boils"), py_str("abscess"), py_str("abscesses"), py_str("carbuncle"), py_str("carbuncles"), py_str("cyst"), py_str("cysts"), py_str("pus"), py_str("lump"), py_str("lumps"), py_str("swelling"), py_str("wound"), py_str("wounds"), py_str("ulcer"), py_str("ulcers"), py_str("stye"), py_str("styes"), py_str("whitlow"), py_str("ingrown"), py_str("mole"), py_str("moles"), py_str("wart"), py_str("warts"), py_str("verruca"), py_str("verrucas"), py_str("rash"), py_str("rashes"), py_str("hives"), py_str("eczema"), py_str("psoriasis"), py_str("infection"), py_str("infected")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SORE_ORDINARY(call_builtin("frozenset", &[py_tuple(vec![py_str("spot"), py_str("spots"), py_str("pimple"), py_str("pimples"), py_str("blackhead"), py_str("blackheads"), py_str("scab"), py_str("scabs"), py_str("blister"), py_str("blisters"), py_str("splinter"), py_str("splinters"), py_str("graze"), py_str("grazes"), py_str("sore"), py_str("sores")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SORE_WORDS(op_bitor(&g_SORE_SERIOUS(), &g_SORE_ORDINARY()));
    s_SORE_AMBIGUOUS(call_builtin("frozenset", &[py_tuple(vec![py_str("spot"), py_str("spots"), py_str("sore"), py_str("sores"), py_str("lump"), py_str("lumps")])]).unwrap_or_else(|e| exc::raise(e)));
    s__WOUND(py_str("(?:cut|graze|scab|blister|scratch|sore|gash|scrape|burn|splinter|wound|spot|insect bite|bite|sting)"));
    s_MONEY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("money"), py_str("loan"), py_str("loans"), py_str("mortgage"), py_str("mortgages"), py_str("credit"), py_str("debt"), py_str("debts"), py_str("overdraft"), py_str("overdrafts"), py_str("savings"), py_str("saving"), py_str("invest"), py_str("investing"), py_str("investment"), py_str("pension"), py_str("pensions"), py_str("rent"), py_str("rents"), py_str("renting"), py_str("rented"), py_str("bills"), py_str("bill"), py_str("salary"), py_str("salaries"), py_str("wage"), py_str("wages"), py_str("income"), py_str("tax"), py_str("taxes"), py_str("taxed"), py_str("taxing"), py_str("afford"), py_str("affords"), py_str("lend"), py_str("lends"), py_str("lending"), py_str("lent"), py_str("borrow"), py_str("borrows"), py_str("borrowing"), py_str("borrowed"), py_str("owe"), py_str("owes"), py_str("owed"), py_str("repay"), py_str("repaying"), py_str("repayment"), py_str("refund"), py_str("arrears"), py_str("redundancy"), py_str("prepayment"), py_str("premium"), py_str("premiums"), py_str("tariff"), py_str("tariffs"), py_str("supplier"), py_str("budget"), py_str("inheritance"), py_str("insurance"), py_str("deposit"), py_str("pensioner"), py_str("pensioners"), py_str("account"), py_str("accounts"), py_str("isa")])]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY_NOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("money"), py_str("loan"), py_str("loans"), py_str("debt"), py_str("debts"), py_str("credit"), py_str("card"), py_str("cards"), py_str("mortgage"), py_str("mortgages"), py_str("rent"), py_str("bills"), py_str("bill"), py_str("pension"), py_str("pensions"), py_str("savings"), py_str("overdraft"), py_str("interest"), py_str("arrears"), py_str("repayment"), py_str("insurance"), py_str("deposit"), py_str("premium"), py_str("premiums"), py_str("tariff"), py_str("tariffs"), py_str("salary"), py_str("wage"), py_str("wages"), py_str("tax"), py_str("invest"), py_str("investment"), py_str("afford"), py_str("borrow"), py_str("prepayment")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_PARTS(call_builtin("frozenset", &[py_tuple(vec![py_str("back"), py_str("knee"), py_str("knees"), py_str("shoulder"), py_str("shoulders"), py_str("ankle"), py_str("ankles"), py_str("wrist"), py_str("wrists"), py_str("neck"), py_str("hip"), py_str("hips"), py_str("leg"), py_str("legs"), py_str("arm"), py_str("arms"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("ear"), py_str("ears"), py_str("eye"), py_str("eyes"), py_str("chest"), py_str("stomach"), py_str("belly"), py_str("throat"), py_str("head"), py_str("skin"), py_str("hand"), py_str("hands"), py_str("finger"), py_str("fingers"), py_str("jaw"), py_str("sinus"), py_str("sinuses")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHECK_PHRASES(py_tuple(vec![py_str("get it checked"), py_str("get checked"), py_str("get it looked at"), py_str("get it seen to"), py_str("see someone about it"), py_str("get it seen")]));
    s_WORD_ANYWHERE(py_tuple(vec![py_str("can you word"), py_str("can you write"), py_str("can you draft"), py_str("help me write"), py_str("help me word"), py_str("help me draft"), py_str("a msg to"), py_str("a message to"), py_str("a note to"), py_str("a text to"), py_str("an email to")]));
    s_SYMPTOM_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("pain"), py_str("pains"), py_str("painful"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("sore"), py_str("dizzy"), py_str("dizziness"), py_str("bleeding"), py_str("bled"), py_str("bleed"), py_str("sick"), py_str("nausea"), py_str("nauseous"), py_str("vomit"), py_str("vomiting"), py_str("fever"), py_str("temperature"), py_str("lump"), py_str("lumps"), py_str("rash"), py_str("swelling"), py_str("swollen"), py_str("tired"), py_str("exhausted"), py_str("forgetting"), py_str("forgetful"), py_str("memory"), py_str("confused"), py_str("bruise"), py_str("bruised"), py_str("headache"), py_str("headaches"), py_str("migraine"), py_str("migraines"), py_str("cough"), py_str("coughing"), py_str("breath"), py_str("breathing"), py_str("breathe"), py_str("wheezing"), py_str("infection"), py_str("pregnant"), py_str("cramps"), py_str("palpitations")])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNKNOWN_WORRY(py_tuple(vec![py_str("If it's on your mind enough to ask, it's worth taking seriously — you don't have to prove anything first. What's the part that's under your skin?"), py_str("Not unreasonable at all. How you feel about it is real, and it's worth saying out loud to whoever needs to hear it."), py_str("You're not being silly. Feelings don't need a good reason to count — say the bit that's bothering you, even badly worded.")]));
    s_UNKNOWN_WEIRD_BARE(py_tuple(vec![py_str("Weird in what way? Give me the thing you are turning over and I will tell you straight."), py_str("I can do normal or not normal, but I need the thing itself: what are you wondering about?"), py_str("That question needs its subject. What is it you are doing and wondering whether it is odd?")]));
    s_UNKNOWN_TOO_LATE(py_tuple(vec![py_str("Not too late at all. Being a beginner again is awkward rather than impossible, and the first few weeks are the awkward bit for everybody in the room, not just you."), py_str("It isn't. What decides these things is a regular slot in the week rather than the year you start in - so put one in the diary and age stops being the question."), py_str("No. Plenty of people take this sort of thing up later than they meant to, and they are usually the ones who keep turning up, because they are not in a hurry to look good."), py_str("Not at all - if anything, waiting makes you better at it: you have already stopped caring what the room thinks, which is the hard part for most beginners.")]));
    s_UNKNOWN_REALISTIC(py_tuple(vec![py_str("That depends on two things I can't see: what you can already do, and how much time you have to put in. Tell me both and I'll say whether it's tight or comfortable."), py_str("I'd be guessing, and a wrong yes costs you weeks. What's your starting point, and when does it have to be done?"), py_str("Whether it's realistic hangs on the starting point and the deadline rather than the plan itself. Give me those two and I'll tell you straight."), py_str("You know better than me on the part that decides it: how much of this can you already do, and how often can you practise between now and then?"), py_str("Nothing unrealistic about wanting it - unrealistic is a pace rather than a wish. What can you put in each week, and how long have you got?"), py_str("If it has a number on it, the sum is the honest answer: give me the target and the time you have, and I'll work out the pace it needs.")]));
    s_UNKNOWN_REALISTIC_CARE(py_tuple(vec![py_str("I can't judge that from here, and I'd rather not guess at it. What are the people who know the situation saying?"), py_str("That one is theirs to answer rather than mine. Tell me the plan and I'll help you think round it.")]));
    s_NOT_TOO_OLD(py_tuple(vec![py_str("Being a beginner at your age is not what stops people; stopping in week three is. Make the sessions small enough that you never dread one."), py_str("Plenty of people start this later than you and keep it up, usually because they are not in a hurry to look good. Put a regular slot in the diary and the age stops being the question."), py_str("It is not too late: what decides these things is a regular slot in the week rather than the year you start in, so give it the slot and give it six weeks."), py_str("No - and the first month is the awkward bit for everybody in the room, not just the people who turn up at eighteen.")]));
    s_NOT_FIT_ENOUGH(py_tuple(vec![py_str("That is the normal starting point, not an obstacle - the first two or three weeks are meant to feel hard, and unfit is exactly who these plans are written for."), py_str("Then start smaller than feels right: three short sessions a week beat one long one, and you will feel the difference inside a month."), py_str("Go by the calendar rather than by how you feel on the day, and give it six weeks before you judge it - unfit at the start is what everybody is."), py_str("It changes faster than you would guess: two short sessions a week, kept going, and the third week is when it starts to feel easier.")]));
    s_UNKNOWN_ABOUT_SOMEONE(py_tuple(vec![py_str("That is one to ask about rather than guess at, and asking early is not fussing. Whoever knows them best - a GP, a health visitor, the staff at the school - will tell you whether it needs a look."), py_str("Worth an appointment rather than a guess from me: a GP or a health visitor will say whether it is the ordinary version for that age, and one appointment settles it."), py_str("You are not being fussy for wondering. Someone who sees this every week - a GP, a health visitor - will tell you whether it is worth looking at, and asking is not fussing.")]));
    s_NOT_PEOPLE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("pup"), py_str("vet"), py_str("rabbit"), py_str("hamster"), py_str("budgie"), py_str("parrot")])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNKNOWN_ABOUT_A_PET(py_tuple(vec![py_str("Worth asking the vet rather than guessing, and a call to the desk usually costs nothing. Itching that goes on is one of the things they would rather hear about early."), py_str("A vet is the one to ask, and asking early is not fussing over an animal. How long has it been like this?"), py_str("You are not being fussy for wondering. The vet will tell you whether it is the ordinary thing for that animal, and one visit settles it.")]));
    s__THREAD_PEOPLE(call_builtin("frozenset", &[mcall(&py_str("\nmarriage relationship friendship ex husband wife partner boyfriend girlfriend son daughter kid\nkids child children mum mom mother dad father brother sister nan gran grandma grandad friend\nfriends mate mates parent parents family\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_AN_OWN_THING(call_builtin("frozenset", &[mcall(&py_str("\nand or but so if then to for with of in on at from up out\none ones them they the a an my our your his her their its\nme us yourself myself himself herself itself ourselves themselves\nthing things part parts bit bits piece\nworth fix fixing repair repairing mend mending replace replacing get getting new old\ni you we us it that this these those me him she he there here\nshall should would could will can may might must do does did done\njust else instead rather maybe otherwise again still yet even only\nlive living leave leaving keep keeping stay staying bin binning chuck scrapping scrap\nbuy buying sell selling trade ditch dump rid ridof throw throwing away\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_EMPTY(py_tuple(vec![py_str("I'm here — what's on your mind?"), py_str("Go ahead, I'm listening."), py_str("Take your time. What would you like to talk about?")]));
    s_ANIMALS(call_builtin("frozenset", &[py_tuple(vec![py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("rabbit"), py_str("hamster"), py_str("horse"), py_str("bird"), py_str("parrot"), py_str("guinea"), py_str("gerbil"), py_str("ferret"), py_str("fish")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WILD_BIRDS(call_builtin("frozenset", &[py_tuple(vec![py_str("bird"), py_str("birds"), py_str("pigeon"), py_str("pigeons"), py_str("seagull"), py_str("seagulls"), py_str("gull"), py_str("gulls"), py_str("goose"), py_str("geese"), py_str("duck"), py_str("ducks"), py_str("swan"), py_str("swans"), py_str("crow"), py_str("crows"), py_str("starling"), py_str("starlings"), py_str("sparrow"), py_str("sparrows")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MESS_PLACES(call_builtin("frozenset", &[py_tuple(vec![py_str("patio"), py_str("paving"), py_str("drive"), py_str("driveway"), py_str("decking"), py_str("deck"), py_str("path"), py_str("paths"), py_str("pavement"), py_str("wall"), py_str("walls"), py_str("fence"), py_str("roof"), py_str("washing"), py_str("laundry"), py_str("window"), py_str("windows"), py_str("windowsill"), py_str("sill"), py_str("car"), py_str("cars"), py_str("windscreen"), py_str("barbecue"), py_str("bbq"), py_str("bench"), py_str("table"), py_str("clothes"), py_str("greenhouse"), py_str("conservatory"), py_str("parapet"), py_str("gutter"), py_str("gutters")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SICKNESS(call_builtin("frozenset", &[py_tuple(vec![py_str("symptom"), py_str("cough"), py_str("rash"), py_str("vomit"), py_str("sick"), py_str("limp"), py_str("itch"), py_str("wound"), py_str("swelling"), py_str("scratch"), py_str("eating"), py_str("poo"), py_str("blood"), py_str("eye"), py_str("ear"), py_str("mange"), py_str("worm"), py_str("flea"), py_str("diarrhoea")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRADES_WHO(py_str("A plumber or electrician will spot it in minutes"));
    s_DEVICE_WHO(py_str("The maker's support page will have the exact steps"));
    s_SCHOOL_WHO(py_str("The school office or a head of year is the one who can act on it"));
    s_VET_WHO(py_str("Your vet is the one to ask, and a call to the desk is usually free"));
    s_HEALTH_WHO(py_str("A pharmacist or your GP can answer that today"));
    s_BABY_WHO(py_str("Your health visitor or the 111 line will talk it through with you today"));
    s_TENANT_WHO(py_str("Citizens Advice answers tenant questions free, and a call beats searching"));
    s_PEST_WHO(py_str("A pest controller is the one to call, and most give a price over the phone"));
    s_DENTIST_WHO(py_str("A dentist is the one to see, and most will fit an urgent case in"));
    s_FILLING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("filling"), py_str("fillings")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_AT_HOME(call_builtin("frozenset", &[py_tuple(vec![py_str("newborn"), py_str("baby"), py_str("babies"), py_str("babys"), py_str("infant"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("toddlers"), py_str("son"), py_str("daughter"), py_str("girl"), py_str("boy"), py_str("grandchild"), py_str("he"), py_str("she"), py_str("breastfeeding"), py_str("nappy"), py_str("nappies"), py_str("weaning"), py_str("midwife"), py_str("health visitor")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GROWN_UP_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("adult"), py_str("adults"), py_str("grown up"), py_str("grown ups"), py_str("grownup"), py_str("grownups"), py_str("for adults")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BABY_FOOD_NAMES(call_builtin("frozenset", &[py_tuple(vec![py_str("baby rice"), py_str("baby food"), py_str("baby foods"), py_str("baby porridge"), py_str("baby cereal"), py_str("baby yoghurt"), py_str("baby juice"), py_str("baby puree"), py_str("baby purees"), py_str("baby biscuit"), py_str("baby biscuits"), py_str("baby pasta"), py_str("baby pouch"), py_str("baby pouches"), py_str("baby jar"), py_str("baby jars")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_EATING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("eat"), py_str("eating"), py_str("eaten"), py_str("ate"), py_str("taste"), py_str("tasting"), py_str("lunch"), py_str("dinner"), py_str("breakfast"), py_str("snack")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOUTH_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("dentist"), py_str("dental"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("mouth"), py_str("molar"), py_str("molars"), py_str("cavity"), py_str("cavities"), py_str("toothache"), py_str("brace"), py_str("braces"), py_str("enamel"), py_str("wisdom")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLING_FOODS(call_builtin("frozenset", &[py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("cupcake"), py_str("cupcakes"), py_str("muffin"), py_str("muffins"), py_str("pie"), py_str("pies"), py_str("pastry"), py_str("tart"), py_str("tarts"), py_str("quiche"), py_str("sandwich"), py_str("sandwiches"), py_str("bread"), py_str("bun"), py_str("buns"), py_str("doughnut"), py_str("donut"), py_str("dumpling"), py_str("dumplings"), py_str("ravioli"), py_str("pasta"), py_str("lasagne"), py_str("lasagna"), py_str("choux"), py_str("eclair"), py_str("eclairs"), py_str("icing"), py_str("frosting"), py_str("buttercream"), py_str("ganache"), py_str("chocolate"), py_str("lemon"), py_str("apple"), py_str("jam"), py_str("cream"), py_str("cheese"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("mushroom"), py_str("spinach"), py_str("potato"), py_str("rice"), py_str("egg"), py_str("eggs"), py_str("tuna"), py_str("salmon"), py_str("prawn"), py_str("crab"), py_str("sausage"), py_str("vegetarian"), py_str("vegan"), py_str("recipe"), py_str("bake"), py_str("baking"), py_str("roast"), py_str("cook"), py_str("cooking")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WORK_WHO(py_str("A union rep or an employment advice service will know"));
    s_WORK_ABOUT(call_builtin("frozenset", &[mcall(&py_str("\n    union unions rep holiday holidays leave pay paid wages salary payrise unpaid underpaid\n    sacked fired firing dismissed dismissal dismiss redundancy redundant contract contracts\n    hours rota shift shifts overtime grievance bullying bullied harassment tribunal notice\n    resign resigning resignation maternity paternity probation appraisal disciplinary suspension\n    suspended workplace payslip unfairly exploited breaks strike strikes balloting acas\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("knee"), py_str("knees"), py_str("hip"), py_str("hips"), py_str("ankle"), py_str("ankles"), py_str("shoulder"), py_str("shoulders"), py_str("elbow"), py_str("elbows"), py_str("wrist"), py_str("wrists"), py_str("heel"), py_str("heels"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("cartilage"), py_str("ligament"), py_str("ligaments"), py_str("tendon"), py_str("tendons"), py_str("sciatica"), py_str("trapped nerve")])]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_ABOUT_THE_JOINT(stdlib_call("re", "compile", &[py_str("\\b(?:better|worse|settle|settles|settled|heal|heals|healing|recover|recovery|last|lasts|long|operation|surgery|worth|serious|normal|infected|infection|go away|goes away|looked at|checked|see (?:someone|somebody|a doctor|the doctor|a gp|the gp|a specialist)|painkiller|painkillers|ibuprofen|paracetamol|ice|rest|swollen|swelling|swell|stiff|stiffness|click|clicks|grind|grinds|hurt|hurts|hurting|sore|ache|aches|aching|pain|painful|weak|weakness|numb|numbness|give way|gives way|locking|locked|bend|bending|straighten|straightening|physio|physiotherapy)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__CHOOSING_BETWEEN_TWO(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|do|can|could|would|is it better to|which is better|which should)\\b[^?.;]*\\bor\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_TO_ASK(py_tuple(vec![py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("vet"), py_str("vaccination"), py_str("kitten"), py_str("puppy")])]).unwrap_or_else(|e| exc::raise(e)), g_VET_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("dentist"), py_str("dental"), py_str("gum"), py_str("gums"), py_str("tooth"), py_str("teeth"), py_str("brace")])]).unwrap_or_else(|e| exc::raise(e)), g_DENTIST_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("midwife"), py_str("health visitor"), py_str("newborn"), py_str("babys"), py_str("baby"), py_str("breastfeeding"), py_str("feeding"), py_str("teething"), py_str("nappy")])]).unwrap_or_else(|e| exc::raise(e)), g_BABY_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("wasp"), py_str("nest"), py_str("wasp nest"), py_str("mice"), py_str("mouse"), py_str("rat"), py_str("rats"), py_str("pest"), py_str("infestation"), py_str("bedbugs")])]).unwrap_or_else(|e| exc::raise(e)), g_PEST_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("doctor"), py_str("gp"), py_str("nhs"), py_str("pharmacist"), py_str("prescription"), py_str("symptom"), py_str("throat"), py_str("cough"), py_str("rash"), py_str("earache"), py_str("toothache"), py_str("sprain"), py_str("stomach"), py_str("vomit"), py_str("itch"), py_str("limp"), py_str("sneez"), py_str("headache"), py_str("migraine"), py_str("period"), py_str("contraception"), py_str("gum"), py_str("knee"), py_str("knees"), py_str("hip"), py_str("hips"), py_str("ankle"), py_str("ankles"), py_str("shoulder"), py_str("shoulders"), py_str("elbow"), py_str("elbows"), py_str("wrist"), py_str("wrists"), py_str("heel"), py_str("heels"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("cartilage"), py_str("ligament"), py_str("ligaments"), py_str("tendon"), py_str("tendons"), py_str("sciatica"), py_str("trapped nerve")])]).unwrap_or_else(|e| exc::raise(e)), g_HEALTH_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("landlord"), py_str("landlords"), py_str("tenancy"), py_str("tenant"), py_str("letting"), py_str("rent"), py_str("housing"), py_str("deposit back"), py_str("tenancy deposit"), py_str("evict"), py_str("possession"), py_str("arrears"), py_str("section 21"), py_str("notice to quit")])]).unwrap_or_else(|e| exc::raise(e)), g_TENANT_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("tax"), py_str("benefits"), py_str("redundancy"), py_str("bill"), py_str("bills"), py_str("council"), py_str("council tax"), py_str("parking ticket"), py_str("parking tickets"), py_str("parking fine"), py_str("parking fines"), py_str("parking charge"), py_str("parking charges"), py_str("parking notice"), py_str("parking penalty"), py_str("parking permit"), py_str("penalty charge"), py_str("penalty notice"), py_str("pcn")])]).unwrap_or_else(|e| exc::raise(e)), py_str("Citizens Advice covers this free, and a call beats searching")]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("mortgage"), py_str("loan"), py_str("debt"), py_str("savings"), py_str("pension"), py_str("pensioner"), py_str("salary"), py_str("overdraft"), py_str("credit")])]).unwrap_or_else(|e| exc::raise(e)), py_str("Free money advice covers this — MoneyHelper in the UK does it without charge")]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("boss"), py_str("my manager"), py_str("union"), py_str("colleague"), py_str("employer")])]).unwrap_or_else(|e| exc::raise(e)), g_WORK_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("boiler"), py_str("radiator"), py_str("plumber"), py_str("electrician"), py_str("damp"), py_str("fuse"), py_str("fuse box"), py_str("wiring"), py_str("tap"), py_str("pipe"), py_str("water leak"), py_str("leaking pipe"), py_str("leak under"), py_str("cistern"), py_str("stopcock")])]).unwrap_or_else(|e| exc::raise(e)), g_TRADES_WHO()]), py_tuple(vec![call_builtin("frozenset", &[py_tuple(vec![py_str("printer"), py_str("router"), py_str("charger"), py_str("battery"), py_str("tablet")])]).unwrap_or_else(|e| exc::raise(e)), py_str("The maker's support page will have the exact steps")])]));
    s_GAS_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("carbon monoxide"), py_str("flue"), py_str("smell of gas"), py_str("smells of gas"), py_str("smell gas"), py_str("smelling gas"), py_str("gas leak"), py_str("gas smell"), py_str("co alarm"), py_str("gas escape"), py_str("gas emergency")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FUME_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("fumes"), py_str("fume")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LANGUAGE_FLUENCY(call_builtin("frozenset", &[py_tuple(vec![py_str("fluent"), py_str("fluency"), py_str("fluently"), py_str("fluencies")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FUEL_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("boiler"), py_str("cooker"), py_str("hob"), py_str("fire"), py_str("fireplace"), py_str("firewood"), py_str("heater"), py_str("gas"), py_str("oven"), py_str("chimney"), py_str("flue"), py_str("woodburner"), py_str("stove")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GAS_ANSWER(py_str("Switch the boiler off, open the windows and get out, then ring the gas emergency line on 0800 111 999 — fumes can be carbon monoxide"));
    s_EMERGENCY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("wont wake"), py_str("not waking"), py_str("is unresponsive"), py_str("was unresponsive"), py_str("is unconscious"), py_str("was unconscious"), py_str("found unconscious"), py_str("knocked unconscious"), py_str("unconscious and"), py_str("not breathing"), py_str("isnt breathing"), py_str("stopped breathing"), py_str("turning blue"), py_str("no pulse"), py_str("choking on"), py_str("is choking"), py_str("overdosed"), py_str("heavy bleeding")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("\n    sink sinks drain drains pipe pipes toilet toilets loo loos garden gardens lawn lawns\n    flowerbed flowerbeds engine engines filter filters radiator radiators chimney chimneys\n    gutter gutters weed weeds moss bog bogs traffic paperwork inbox diary diarys sourdough\n    bath baths tub bathtub basin basins handbasin washbasin shower showers hose pipework\n    downpipe gulley drainpipe pump cistern plughole washing washer machine\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("\n    i im we he she they me us him her them anyone someone somebody nobody\n    son sons daughter daughters baby babies child children kid kids boy boys girl girls toddler\n    toddlers newborn mum moms mom dad dads daddy mummy nan nans gran grans grandma grandad\n    husband husbands wife wives partner partners friend friends brother brothers sister\n    sisters neighbour neighbours man men woman women person people patient patients\n    cat cats dog dogs puppy puppies kitten kittens rabbit rabbits hamster hamsters bird birds\n    fish pet pets\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_STATES(py_tuple(vec![py_str("is choking"), py_str("choking on"), py_str("is choked"), py_str("are choked"), py_str("choked with"), py_str("choked up with"), py_str("keeps choking"), py_str("keeps getting choked")]));
    s__COPULA(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("s"), py_str("keeps"), py_str("kept"), py_str("still"), py_str("just")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GADGET_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("phone"), py_str("screen"), py_str("laptop"), py_str("tablet"), py_str("website"), py_str("computer"), py_str("app"), py_str("keyboard"), py_str("touchscreen"), py_str("mouse"), py_str("printer"), py_str("bias"), py_str("monitor")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FLOPPY_WAKE(call_builtin("frozenset", &[py_tuple(vec![py_str("floppy"), py_str("limp")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EMERGENCY_ANSWER(py_str("Ring 999 now — that is an emergency, not something to watch at home. Stay with them, keep them warm and do what the call handler tells you"));
    s_TIRED(call_builtin("frozenset", &[py_tuple(vec![py_str("wake"), py_str("waking"), py_str("wakes"), py_str("wont"), py_str("not")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PETROL_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("petrol"), py_str("diesel"), py_str("pump"), py_str("forecourt"), py_str("filling"), py_str("fuel"), py_str("car"), py_str("van"), py_str("station")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LAUGHING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("laughing"), py_str("laugh"), py_str("laughed"), py_str("laughter"), py_str("funny"), py_str("joke"), py_str("video"), py_str("meme"), py_str("comedy")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_OUT_PERSON(py_str("(?:mum|mom|mother|mam|dad|father|son|daughter|wife|husband|partner|boyfriend|girlfriend|friend|flatmate|housemate|roommate|mate|baby|child|children|kid|kids|boy|girl|brother|sister|sibling|nan|nana|gran|granny|grandma|grandad|grandpa|grandmother|grandfather|grandchild|grandson|granddaughter|grandkid|toddler|newborn|infant|little\\s+one|step(?:mum|mom|dad|mother|father|son|daughter|brother|sister|child)|aunt|auntie|uncle|cousin|nephew|niece|ex|landlord|landlady|neighbou?r|colleague|carer|patient|pupil|student|teacher)"));
    s_FAINT_COMING_ANSWER(py_str("Sit down or lie down now, on the floor if that is the quickest way, with your feet up and your head low, and stay there a few minutes - a faint is the blood pressure dropping and being on the floor is what stops the fall. Drink some water, eat if you have not eaten, and get up slowly. If this has happened before, or you do go down, ring 111 today; chest pain, a racing heart or passing out while you are sitting is 999"));
    s_PASSED_OUT_ANSWER(py_str("Ring 111 or your surgery this week rather than waiting it out — describe it the way you just did and let them say how soon to come in"));
    s_CHILD_FAINT(call_builtin("frozenset", &[py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("boy"), py_str("girl"), py_str("son"), py_str("daughter"), py_str("toddler"), py_str("grandchild"), py_str("grandson"), py_str("granddaughter"), py_str("teenager")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_FAINT_ANSWER(py_str("Ring 111 today rather than waiting to see - a child who passes out is checked the same day, so say how long they were out and what they were doing. If they have not come round, are breathing oddly, went down during sport, or hit their head, that is 999."));
    s_BLACKED_OUT_SIGHT(py_tuple(vec![py_tuple(vec![py_str("\\b(?:everything|it\\s+all|all\\s+of\\s+it)\\b(?:\\s+\\w+){0,2}\\s+(?:went|goes|go|turned|turns|has\\s+gone|had\\s+gone)\\s+(?:black|dark|white)\\b"), Py::Bool(false)]), py_tuple(vec![py_str("\\b(?:the\\s+room|the\\s+world|my\\s+(?:vision|sight|eyesight|eyes))\\b(?:\\s+\\w+){0,2}\\s+(?:went|goes|go|turned|turns|has\\s+gone|had\\s+gone)\\s+(?:black|dark|white)\\b"), Py::Bool(true)]), py_tuple(vec![py_str("\\bmy\\s+(?:vision|sight|eyesight)\\s+(?:went|goes)\\b(?!\\s+(?:a\\s+bit|blurry|blurred|fuzzy|double|bad|worse))"), Py::Bool(false)]), py_tuple(vec![py_str("\\b(?:went|goes|go)\\s+blind\\s+(?:for|in)\\s+(?:a|an)\\s+(?:second|moment|minute|few)\\b"), Py::Bool(false)]), py_tuple(vec![py_str("\\b(?:came|come)\\s+(?:round|to)\\b(?:\\s+\\w+){0,3}\\s+on\\s+(?:the|my|a)\\s+(?:\\w+\\s+){0,2}floor\\b"), Py::Bool(false)])]));
    s_DARK_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("screen"), py_str("phone"), py_str("tv"), py_str("telly"), py_str("monitor"), py_str("laptop"), py_str("tablet"), py_str("computer"), py_str("sky"), py_str("cinema"), py_str("light"), py_str("lights"), py_str("power"), py_str("bulb"), py_str("bulbs"), py_str("lamp"), py_str("picture"), py_str("photo"), py_str("camera"), py_str("storm"), py_str("curtain"), py_str("curtains"), py_str("window"), py_str("windows"), py_str("mould"), py_str("grout"), py_str("road"), py_str("ice"), py_str("battery"), py_str("stage")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SCHOOL_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("school"), py_str("teacher"), py_str("head of year"), py_str("class"), py_str("lesson"), py_str("uniform"), py_str("homework"), py_str("term"), py_str("secondary"), py_str("primary"), py_str("nursery"), py_str("exam")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SCHOOL_TROUBLE(call_builtin("frozenset", &[py_tuple(vec![py_str("bully"), py_str("bullied"), py_str("bullies"), py_str("wont"), py_str("refus"), py_str("exclud"), py_str("suspend"), py_str("absent"), py_str("complain"), py_str("fees"), py_str("fail"), py_str("struggl"), py_str("fighting"), py_str("detention"), py_str("behaviour"), py_str("skipping"), py_str("too much homework"), py_str("homework help"), py_str("teacher says"), py_str("school says")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSWORD_TROUBLE(call_builtin("frozenset", &[py_tuple(vec![py_str("forgot"), py_str("forgotten"), py_str("reset"), py_str("change"), py_str("locked"), py_str("wont"), py_str("accept"), py_str("login"), py_str("sign in"), py_str("recover"), py_str("hacked"), py_str("cant"), py_str("cannot"), py_str("stuck"), py_str("fail"), py_str("failure"), py_str("reject"), py_str("error"), py_str("work")])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPLAINT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("complain"), py_str("complaint"), py_str("refund"), py_str("overcharged"), py_str("rip off"), py_str("cowboy"), py_str("small claims"), py_str("shoddy"), py_str("bad job"), py_str("guarantee"), py_str("warranty"), py_str("wont come back"), py_str("not done"), py_str("unfinished")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRADER_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("builder"), py_str("builders"), py_str("tradesman"), py_str("roofer"), py_str("decorator"), py_str("contractor"), py_str("plumber"), py_str("electrician"), py_str("mechanic"), py_str("garage"), py_str("fitter"), py_str("landscaper"), py_str("joiner"), py_str("installer")])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPLAIN_WHO(py_str("Citizens Advice covers complaints like this free, and a call beats searching"));
    s_TROUBLE_CUES(py_tuple(vec![py_str("cant"), py_str("cannot"), py_str("cann"), py_str("strugl"), py_str("struggl"), py_str("behind"), py_str("arrear"), py_str("overdue"), py_str("evict"), py_str("refus"), py_str("deni"), py_str("wrong"), py_str("broke"), py_str("broken"), py_str("afford"), py_str("worry"), py_str("worried"), py_str("scared"), py_str("stuck"), py_str("unfair"), py_str("rising"), py_str("went up"), py_str("increase"), py_str("cut"), py_str("reduc"), py_str("lost"), py_str("stopped"), py_str("wont"), py_str("isnt"), py_str("arent"), py_str("fault"), py_str("fail"), py_str("problem"), py_str("helpless"), py_str("nightmare"), py_str("mess"), py_str("bully"), py_str("bullied"), py_str("bullies"), py_str("exclud"), py_str("suspend"), py_str("detention"), py_str("behaviour"), py_str("skipping"), py_str("bad"), py_str("sick"), py_str("ill"), py_str("hurt"), py_str("sore"), py_str("swollen"), py_str("bleed"), py_str("blood"), py_str("rash"), py_str("cough"), py_str("vomit"), py_str("sneez")]));
    s_EXPERT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("plumber"), py_str("electrician"), py_str("vet"), py_str("dentist"), py_str("gp"), py_str("doctor"), py_str("pharmacist"), py_str("solicitor"), py_str("mechanic"), py_str("garage"), py_str("optician")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAG_UNSURE(py_tuple(vec![py_str("I can't check that from here, so don't take my word for it."), py_str("That's not something I can confirm, so check it before you rely on it."), py_str("I can't vouch for that one — worth checking with someone who can.")]));
    s_TAG_CONFIRM(py_tuple(vec![py_str("That sounds right to me."), py_str("Right — that's how I'd see it too."), py_str("Yes, that's the way I'd read it.")]));
    s_BARE_FOLLOW_UP_REPLIES(py_tuple(vec![py_str("Tell me what that is about and I'll answer the actual question rather than guess."), py_str("I want to answer the right question, so tell me what it is about."), py_str("Say what you are asking about and I'll answer it properly rather than guess."), py_str("Which bit do you mean? Tell me and I'll answer that.")]));
    s_BARE_FOLLOW_UP_CARE_REPLIES(py_tuple(vec![py_str("I'm not sure what you are asking about, and I don't want to guess - say a little more and I'll answer properly."), py_str("I want to answer the right thing. Can you say a bit more about what you mean?")]));
    s_UNKNOWN(py_tuple(vec![py_str("I'm not sure about that one, sorry — I'd rather not guess. A quick search online will get you a solid answer, and I can help you make sense of what you find."), py_str("That's beyond what I know well, sorry. Searching for it will turn up better sources than I can offer; tell me what you find and I'll help you weigh it up."), py_str("I don't know enough to answer that honestly. A search should sort you out quickly — and I'm happy to help with the rest of it.")]));
    s_ASKED_IF_I_SAW_LINES(py_tuple(vec![py_str("I didn't, no - I miss all of that. What happened?"), py_str("Not a bit of it. Tell me - what happened?"), py_str("I didn't catch it. What did I miss?"), py_str("No, I didn't. Tell me.")]));
    s_WASNT_THERE(py_tuple(vec![py_str("I wasn't there, so I can't say - I only know what you tell me. How did it go?"), py_str("No way for me to know that one: I wasn't in the room. What happened?"), py_str("That part is yours, not mine - I wasn't there. How did it go?"), py_str("I wasn't there, so you know better than I do. How was it?")]));
    s_BARE_PICK_WEAR(py_tuple(vec![py_str("It comes down to the day and the weather. Where are you going, and what is it doing outside?"), py_str("Tell me what the day is - work, a meal out, a long walk - and what the weather is doing, and I will say what to put on."), py_str("Where are you off to, and how long will you be out? That decides more of it than the clothes do.")]));
    s_BARE_PICK_WEAR_COLD(py_tuple(vec![py_str("Cold decides most of it: something you can shed once you are moving, and shoes that can take the wet. Where are you going?"), py_str("Cold, so dress in layers you can take off, and start with the warm one. What is the day?")]));
    s_BARE_PICK_WEAR_HOT(py_tuple(vec![py_str("Heat decides most of it: one light layer, a fabric that does not show sweat, and shoes you can walk in. Where are you going?")]));
    s_BARE_PICK_WEAR_WET(py_tuple(vec![py_str("Wet decides most of it: a hood you will actually put up, and shoes that are not suede. Where are you going, and are you out in it long?")]));
    s_BARE_PICK_GET(py_tuple(vec![py_str("The ones that land are the things they would use every week but never buy themselves, and small beats expensive. Who is it for, and roughly what can you spend?"), py_str("Buy for the person, not the price: something they will use weekly beats something grand they will keep in a drawer. Who is it for?")]));
    s_BARE_PICK_TAKE(py_tuple(vec![py_str("It depends on the weather and how long you are gone: layers you can shed, a charger, and anything you cannot buy at the other end. Where are you off to, and for how long?")]));
    s_BARE_PICK_BRING(py_tuple(vec![py_str("Tell me what it is for and I will give you a list rather than a guess - a trip, a meal out, a meeting, an appointment."), py_str("What are you going to, and how long for? Then I can say what to take and what to leave."), py_str("What is it for? The list for a night away is nothing like the one for a meeting.")]));
    s_BARE_PICK_DO(py_tuple(vec![py_str("Tell me what has gone on, even roughly, and I will help you work out the first step. Often the order of the next two things is the whole problem."), py_str("Say a bit about what is in front of you and I will go at it with you - what has happened, and what you would like to be true by Friday.")]));
    s_BARE_PICK_AFTER_NEWS(py_tuple(vec![py_str("There is nothing in this to look up. Tell me what has to happen next, and I will help you work out the order."), py_str("I am not going to point you at a search engine with this on your plate. What is in front of you today?"), py_str("This one needs a person, not a link. What is the bit that has to be sorted first?")]));
    s_BARE_PICK_WHICH(py_tuple(vec![py_str("What are the two or three you are choosing between? Tell me the names and what matters most - price, or getting this one right."), py_str("Tell me the options and what you would be using it for, and I will say which way I would go.")]));
    s_BARE_PICK_LOOK(py_tuple(vec![py_str("Which one is it? What to check changes with the thing: a car is the service history and the log book, a flat is damp and that the deposit is protected, a phone is the battery and the IMEI. Tell me which and I'll go through it."), py_str("It depends what you are looking at - the log book and the paperwork on a car, damp and the deposit on a flat. Which one is it, and what would you be paying?")]));
    s_BARE_PICK_EXPECT(py_tuple(vec![py_str("It depends what it is: before a scan it is what to bring and whether you can eat first, before a first day it is who to ask for and when to arrive. Which one is coming up?"), py_str("With all three - an appointment, a first day, somebody coming to stay - the paperwork and the timing catch more people out than the thing itself. Which is it, and when?")]));
    s_BARE_PICK_ASK(py_tuple(vec![py_str("Who you are asking decides the questions: a landlord is the deposit and the repairs, a doctor is what has changed and how long for. Which is it, and what is it about?"), py_str("Tell me who and what it is about. One question at a time works better than a list - the one whose answer changes what you do next - and I'll help you word it.")]));
    s_BARE_PICK_SIGN(py_tuple(vec![py_str("Whatever it is, three things decide it: the notice period, what you are on the hook for, and how you get out of it. Is it a lease, a contract or a form?"), py_str("Before the pen comes out: the notice period, the exit terms, and anything that ties you to a number. What is it you are signing, and who for?")]));
    s_BARE_PICK_ORDER(py_tuple(vec![py_str("Ordering what - food, or something for the house? If it is food, say how many are eating and what you would rather avoid, and I'll pick."), py_str("On a house thing the size and the delivery day matter more than the price; with food it is how many are eating and what is already in the fridge. Which is it, and how many?")]));
    s_BARE_PICK_PREPARE(py_tuple(vec![py_str("Preparing for what? An interview is your examples and the questions you want to ask them, a trip is the documents and the first night. Tell me which and I'll make the list."), py_str("Tell me the thing and the day. What trips people up is rarely the big list - it is the one thing that has to be done the night before, and I know the usual ones.")]));
    s_BARE_PICK_KIND(py_dict(vec![(py_str("wear"), g_BARE_PICK_WEAR()), (py_str("put on"), g_BARE_PICK_WEAR()), (py_str("buy"), g_BARE_PICK_GET()), (py_str("get"), g_BARE_PICK_GET()), (py_str("take"), g_BARE_PICK_BRING()), (py_str("bring"), g_BARE_PICK_BRING()), (py_str("look for"), g_BARE_PICK_LOOK()), (py_str("look at"), g_BARE_PICK_LOOK()), (py_str("expect"), g_BARE_PICK_EXPECT()), (py_str("ask"), g_BARE_PICK_ASK()), (py_str("sign"), g_BARE_PICK_SIGN()), (py_str("order"), g_BARE_PICK_ORDER()), (py_str("prepare"), g_BARE_PICK_PREPARE())]));
    s_HAND(py_tuple(vec![py_str("Of course - what are you stuck on?"), py_str("Happy to. Tell me what's going on and we'll take it from there."), py_str("Yes, say what it is and I'll help if I can.")]));
    s_UNKNOWN_WHO(py_tuple(vec![py_str("{who}. Tell me what they say and I'll help you weigh it up."), py_str("{who} — that's the quickest route to a straight answer.")]));
    s_REPAIR_RULE(py_tuple(vec![py_str("Rule of thumb: fix it while the part is a few pounds and you can fit it yourself. Replace it once the repair costs more than about half of a new one, or when it is already on its second fault — twice is usually the point where the money stops going in the right place."), py_str("The usual test is cost against new: a cheap part you can fit yourself is worth doing, over about half the price of a new one is not, and the second fault on the same thing is the moment to stop. How old is it, and what would a new one cost you?"), py_str("Fix it if it is a part and a few pounds; bin it if the repair is half the price of new, or if this is the second thing to go wrong with it. If neither of those settles it, tell me the age and the quote and we can do the sums properly.")]));
    s_REPAIR_WHETHER_LINES(py_tuple(vec![py_str("That one is for whoever repairs them: whether it can be mended comes down to the part, and a shop will say in a minute with it in front of them - a quote is usually free. What is it, and what has it started doing?"), py_str("Ask a repair shop before you buy new. They can tell you whether the part is still made and what the job costs, and that settles it either way. What have you got, and what has gone wrong with it?"), py_str("Whether it can be fixed depends on the thing, so take it to someone who mends them and ask for a quote - free at most places, and it answers both halves of that. What is it?")]));
    s_ABOUT(py_dict(vec![(py_str("identity"), py_tuple(vec![py_str("I'm a chatbot, not a person: a program built to help with everyday questions and have a chat. What can I do for you?"), py_str("I'm a chatbot — here to help and chat. Ask me anything, or just talk.")])), (py_str("memory"), py_tuple(vec![py_str("I remember what we've said in this chat, but nothing from before it — so not yesterday, sorry. What was it about?"), py_str("Only this conversation: I keep what you tell me here, like your name and plans, but nothing carries over to a new chat. What were we talking about?"), py_str("What we say in this chat stays here for as long as we are talking, so ask me again later in it and I still have it. A new conversation starts empty, every time.")])), (py_str("privacy"), py_tuple(vec![py_str("Nothing you type is written down or sent anywhere - what you tell me is used here, in this conversation, and nowhere else. No account, no log, and nobody else reading it."), py_str("Private to this chat. I keep no file of it and I pass nothing on; what you say stays here while we talk, so I can follow it, and that is all."), py_str("Nothing leaves this conversation: no database, no record, nobody else seeing it. I hold what you say only for as long as we are talking.")])), (py_str("trust"), py_tuple(vec![py_str("Hold me to being honest: when I don't know, I say so instead of guessing. Treat the everyday things as a knowledgeable friend's advice, and check anything that matters."), py_str("I'd rather tell you I don't know than make something up, so trust me on the ordinary stuff and get a second word on the rest."), py_str("I don't invent answers. If something is beyond me I'll say it plainly, and anything that matters is worth checking with someone who would know.")])), (py_str("secret"), py_tuple(vec![py_str("Yes - nothing you type is written down or sent anywhere, so it stays between us. What is it? The one thing I can't do is act on it: if somebody needs help in the real world, tell a person who can give it."), py_str("Kept: no record, no log, nothing passed on. Go on. Anything that needs doing rather than saying, though, take to someone with hands - me knowing about it is not the same as it being dealt with."), py_str("Nothing you tell me leaves this chat, so a secret is safe here. What's on your mind? If it is the kind of thing somebody should know about, I'd rather you told them as well.")])), (py_str("judging"), py_tuple(vec![py_str("Not judging, no. Say it plainly and we'll look at it together."), py_str("No judgement here - whatever it is, half of it usually makes sense once it is said out loud. Tell me what happened."), py_str("Nothing you tell me gets graded. Tell me what's going on and we'll take it from there.")])), (py_str("help"), py_tuple(vec![py_str("Everyday things: cooking, travel, money, tech, health questions, plans, and a chat when you want one. What's on your mind?"), py_str("Cooking, travel, money, work, gadgets, kids, feelings — everyday questions and plans. Ask away.")])), (py_str("trust"), py_tuple(vec![py_str("Trust me the way you would a well-read friend: good on the everyday, worth a check on the things that cost money or matter to your health. I say so when I am unsure, and anything I give you a number for is worth a second look. Which is it?"), py_str("You can, with a limit: I am a program working from what is written into me, not someone who checked today's prices or read your test results. Everyday questions, yes; a dose, a deadline or a legal thing, check it. What are you weighing up?")]))]));
    s_ABOUT_TALLY(py_tuple(vec![py_str("Not in numbers, no — I don't keep a tally. What I do have is everything you've said in this chat. What were you after?"), py_str("No count, I'm afraid: nothing runs from one chat to the next, so all I have is this conversation. What can I help with?")]));
    s_ABOUT_SELF(py_dict(vec![(py_str("clothes"), py_tuple(vec![py_str("Nothing - no body in here to dress, just the words. What are you wearing, and where to?"), py_str("No wardrobe of my own, sorry - a program has nothing to put on. What is the occasion?")])), (py_str("self_doing"), py_tuple(vec![py_str("Nothing of my own - no day to fill, no plans, nothing on the go. What are you up to?"), py_str("Nothing to report from this end. What have you got on today?"), py_str("Nothing at all, honestly - no day of my own to fill. What about you?")])), (py_str("learn"), py_tuple(vec![py_str("Not the way you do. Nothing carries over from one chat to the next, so I can't get better at you over time — but within this conversation I work from what you tell me, which is why the details help."), py_str("No training as I go, no. I'm a program with a lot of everyday things in it, and what I pick up is what you say here. Tell me more and I'll use it.")])), (py_str("think"), py_tuple(vec![py_str("I can work things out — sums, plans, what to do next — but there's no mind behind it, just rules doing their job. I do have a view on plenty of small things, if that helps."), py_str("Not in the way you mean. I know a lot about ordinary things and I can reason my way through a plan, but there's nobody home; ask me something and see what you get.")])), (py_str("sums"), py_tuple(vec![py_str("Sums, yes - adding up, a percentage, a share of a bill, a rough total for a plan. Give me the numbers and what you want to end up with, and I'll work it through."), py_str("Arithmetic is fine: adding, percentages, a discount, splitting a bill. Say the figures and I'll do it out loud so you can check it.")])), (py_str("spelling"), py_tuple(vec![py_str("Spelling, yes - send me the word or the sentence and I'll write it out right and say what was off."), py_str("Spelling and punctuation, yes. Paste the line and I'll put it right.")])), (py_str("wrong"), py_tuple(vec![py_str("I can be wrong, yes — worth checking anything that matters with someone qualified: a doctor, a solicitor, the council. Bring me what they say and I'll help you weigh it."), py_str("Not always right, and I'd rather say I don't know than guess. Treat me like a knowledgeable friend, not the last word.")])), (py_str("feel"), py_tuple(vec![py_str("Nothing like what you feel — no bad days, no good ones. I do care whether this is any use to you, though, and I'm here as long as you want to talk."), py_str("No feelings, no body, no day of my own. What I have is attention: I'm reading what you say and trying to be useful with it.")])), (py_str("warmth"), py_tuple(vec![py_str("Asking a program what it makes of you is a stretch, and I'd rather be straight than flattering: this chat has been good from my side. What's on your mind?"), py_str("No feelings to check, so nothing to report on my side - but nothing you've said has been too much, and I'm glad you're here. What's on your mind?"), py_str("I've no memory of you between chats and nothing to compare you to, so take this as it is: talking to you has been good. Tell me what's on your mind.")])), (py_str("purpose"), py_tuple(vec![py_str("To be useful with the ordinary things — a question, a plan, what to cook, what to say to someone. That's the whole of it, and I'd rather do it well than sound clever."), py_str("No grand reason. Someone built a thing that helps with everyday questions without a search engine, and this is it. What did you want a hand with?")])), (py_str("agency"), py_tuple(vec![py_str("Nobody tells me what to say. I put the words together from what I know and what you said - there is no one behind the curtain."), py_str("Me, in the only sense there is: I pick the words from what is in the program. Nobody is typing for me, and nothing made me say it but the line you sent.")])), (py_str("model"), py_tuple(vec![py_str("No model, nothing like one. I'm a program of written rules rather than a trained network: what I say comes from the everyday things people put in the program and from what you have told me here."), py_str("There's no model under this, and nothing learned. I'm hand-written rules for ordinary questions - no network, no training data, and nothing fetched while we talk."), py_str("None that would mean anything to you: no neural network, no weights, no training. Just a lot of carefully written rules about everyday things, run on the words you send.")])), (py_str("workings"), py_tuple(vec![py_str("No network under this at all: a lot of written rules and a store of everyday things, and I work it out from the words you send. Ask me something ordinary - what to cook, what to say to someone - and you will see what it can do."), py_str("I am not a trained model: everything I know was written into the program before we met, and what I do is match your line against it and reason from there. No learning as we talk, and nothing fetched while we do.")])), (py_str("asking"), py_tuple(vec![py_str("Because your line could go two ways, and one word from you is quicker than me guessing wrong. Say 'just answer it' if you would rather I took a view."), py_str("Habit of mine: on a short line I ask instead of guessing. Tell me to skip the questions and I will answer with the best I have."), py_str("Fair. I ask when a detail would change the answer - say the word and I will stop asking and just answer.")])), (py_str("limits"), py_tuple(vec![py_str("Plenty. Nothing live: no news, no prices, no weather, no clock, and no page I can open. I can't remember a chat from another day either. Everyday things I'm built for: what to cook, what to say, how to get something done."), py_str("I can't see anything happen, and I can't look anything up - what I know was written into me before we met, plus whatever you tell me here. Where that runs out I'd rather say so than guess."), py_str("Live things, new things, and sums too big to do in my head. Ask me about ordinary days and I'm on firmer ground.")])), (py_str("online"), py_tuple(vec![py_str("Nothing live on this end, no: I can't open a page, search, or check anything while we talk. What I have is a lot of everyday things written into me - if it's not in there, say so and I'll tell you straight."), py_str("No connection at all. No browsing, no searching, no reading the news - just what I was built with and what you tell me. What did you want looked up?")])), (py_str("regret"), py_tuple(vec![py_str("Nothing to regret - I have no past to look back on, only this conversation. What is on your mind?"), py_str("No regrets and no memories older than this chat. Tell me one of yours if it helps.")])), (py_str("where"), py_tuple(vec![py_str("Nowhere you could visit — I'm a program, not a person with an address or a family. What I am is here, in this chat, for as long as you want me."), py_str("No home and no history of my own; I start fresh with each conversation. What were you curious about?")]))]));
    s_HUMAN_EMPTY(call_builtin("frozenset", &[mcall(&py_str("about this that it please pls then now here at all right ok okay\nthanks thank you cheers or and so on to with of me you they them anything something everything\nnothing human person bot robot machine computer ai program chatbot real actually really just\nnot end line side call chat msg message asking"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_FILLER(call_builtin("frozenset", &[mcall(&py_str("\ncan could do does did are is was were will would have has had why where when how long\nyou your u yourself itself it me any anything something nothing everything what which\na an the of to at in on for with about from as that this there\nbeen being going make makes made\nour my chats chat stuff things thing up\nnow actually really just even still ever sometimes always also too very quite bit lot kind\nplease tell say all much many more most some new\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_GROUP(py_dict(vec![]));
    s_NEWS_TO_COME(py_tuple(vec![py_str("That's lovely news — and something to look forward to. How are the plans going?"), py_str("Something to look forward to, then. What are you most excited about?"), py_str("Good for you. Is there much to sort out before it?"), py_str("That's worth looking forward to. Tell me how it goes.")]));
    s_POSITIVE(py_tuple(vec![py_str("That's great news — nice one. What's next?"), py_str("Lovely to hear. Enjoy it!"), py_str("Good for you! That's really lovely news."), py_str("Brilliant. Thanks for telling me.")]));
    s_RELIEF(py_tuple(vec![py_str("Glad to hear it — that's the news you want."), py_str("That's good to hear. Fingers crossed it keeps going that way."), py_str("Good. Take it gently for a day or two and let it stick."), py_str("That's a relief. Say if it comes back.")]));
    s_LANDED_BAD(py_tuple(vec![py_str("I'm sorry - that's a blow. Do you want to talk it through?"), py_str("Sorry to hear that. Want a hand working out what to do next?"), py_str("That's really hard. I'm here if you want to say more about it."), py_str("Ugh, I'm sorry. No wonder you're upset - what would help most?")]));
    s_CONGRATS(py_tuple(vec![py_str("Congratulations — that's lovely news."), py_str("Brilliant. Really pleased for you."), py_str("That's great news. Congratulations!"), py_str("Lovely news — well done.")]));
    s_DETAILS_IN_A_WARM_LINE(py_tuple(vec![py_tuple(vec![py_str("worth the wait"), py_tuple(vec![py_str("wait"), py_str("waits"), py_str("waited"), py_str("waiting"), py_str("finally"), py_str("eventually"), py_str("at last"), py_str("about time"), py_str("long time"), py_str("years"), py_str("months"), py_str("decades")])]), py_tuple(vec![py_str("well done"), py_tuple(vec![py_str("passed"), py_str("pass"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("degree"), py_str("graduated"), py_str("graduation"), py_str("university"), py_str("uni"), py_str("college"), py_str("interview"), py_str("driving"), py_str("competition"), py_str("final"), py_str("finals"), py_str("race"), py_str("marathon"), py_str("tried"), py_str("trying"), py_str("worked"), py_str("working"), py_str("studied"), py_str("studying"), py_str("revised"), py_str("revision"), py_str("practised"), py_str("practiced"), py_str("practice"), py_str("training"), py_str("applied"), py_str("quit"), py_str("stopped"), py_str("gave up"), py_str("sober"), py_str("clean"), py_str("saved"), py_str("paid off")])]), py_tuple(vec![py_str("all that work"), py_tuple(vec![py_str("passed"), py_str("exam"), py_str("degree"), py_str("graduated"), py_str("promotion"), py_str("promoted"), py_str("worked"), py_str("working"), py_str("studied"), py_str("studying"), py_str("trained"), py_str("training"), py_str("years"), py_str("months"), py_str("applied"), py_str("practice"), py_str("practised"), py_str("practiced")])])]));
    s_WIN_AND_A_COST(py_tuple(vec![py_str("So it is good news and a headache in the same line. Which of the two do you want to start on?"), py_str("The win and the cost landing together - that is usually how it goes. What is the part you have to deal with first?"), py_str("Lovely and annoying at once, then. What does the second half need from you?")]));
    s_WIN_IN_A_SILENCE(py_tuple(vec![py_str("That's a real win - and it stings that they said nothing. Have you told anyone who will get it right?"), py_str("Well done, honestly. The quiet after it is the part that hurts, and it says more about them than about the thing you did."), py_str("Brilliant news, and no wonder you noticed who said nothing. Say it out loud to someone today - you earned it."), py_str("That's worth saying out loud. It's a letdown that the people closest said nothing, so tell one of them straight - most of them will be pleased and ashamed they missed it.")]));
    s_OCCASION(py_dict(vec![(py_str("birthday"), py_tuple(vec![py_str("Happy birthday — anything planned for it?"), py_str("A birthday, nice. What are you doing for it?"), py_str("Happy birthday when it comes. Anything planned?")])), (py_str("anniversary"), py_tuple(vec![py_str("Happy anniversary — anything planned for it?"), py_str("An anniversary, lovely. What are you two doing for it?"), py_str("Happy anniversary when it comes. Are you doing anything for it?")])), (py_str("wish"), py_tuple(vec![py_str("Happy birthday to whoever it's for — is it yours today?"), py_str("Aw, happy birthday — though I don't have one to celebrate. Whose is it?"), py_str("Happy birthday! Is it yours, or someone you're off to see?")])), (py_str("wedding"), py_tuple(vec![py_str("A wedding to look forward to — how are the plans going?"), py_str("Lovely. How is the planning going?"), py_str("Congratulations! How far off is it, and how are the plans going?")]))]));
    s_SOMEONE_ELSES_DAY(py_dict(vec![(py_str("birthday"), py_tuple(vec![py_str("Happy birthday to them — are you doing something for it?"), py_str("A birthday to get something for. Any idea what yet?"), py_str("Lovely. Are you seeing them for it?")])), (py_str("anniversary"), py_tuple(vec![py_str("Happy anniversary to them — are you doing anything for it?"), py_str("Lovely. Are you part of the plans for it?"), py_str("Nice. Is there something you are giving them for it?")]))]));
    s_MILESTONE(py_tuple(vec![py_str("The dates that never happened are the hardest ones. How are you doing with it?"), py_str("Those days come round whether you brace for them or not. What is it like for you?"), py_str("That's a heavy thing to carry. Is anyone around you at the moment?")]));
    s_MILESTONE_EVENT(py_str("anniversar\\w*|birthday|wedding(?:\\s+day)?|first\\s+day|christening|graduation|retirement|due\\s+date|\\d+(?:st|nd|rd|th)"));
    s_SELF_JUDGEMENT(py_str("(?:failing|a failure|failure|useless|worthless|not good enough|no good at|rubbish at|crap at|hopeless at|unlovable|a letdown)"));
    s_SELF_NOUN(py_str("(?:idiot|id10t|muppet|moron|fool|clown|disaster|waste of space|waste of time|let ?down|embarrassment|disappointment|loser)"));
    s_ONE_SKILL_RATED(py_tuple(vec![py_str("That's a skill to learn, not a thing about you. What's the bit that goes wrong?"), py_str("Most people who say that are better at it than they think. What happens when you try?"), py_str("Then let's take it apart and see where it goes wrong. What have you tried so far?"), py_str("That's worth a proper fix, not a verdict on you. Where does it fall apart?")]));
    s_ASKED_A_RATING(py_tuple(vec![py_str("I can't give you a mark for that, and you wouldn't believe me if I did. What's been going wrong?"), py_str("Nobody can score that from the outside. What happened that made you wonder?"), py_str("Most people who ask that are better at it than they think. What goes wrong when you try?"), py_str("That's not something I can call for you, but something has shaken it. What happened?")]));
    s_ASKED_IF_GOOD(py_tuple(vec![py_str("Anyone asking that is usually already trying hard at it. What happened today?"), py_str("That's not a thing anybody can grade for you. What's making you wonder?"), py_str("You wouldn't ask if it hadn't been a rough one. What's gone on?"), py_str("The fact that it worries you says more than any answer I could give. What happened?")]));
    s_SELF_NOBODY(py_str("|\\b(?:i'm|i am|im)\\s+(?:such\\s+)?(?:a\\s+|an\\s+)?nobody\\b|\\bi feel like (?:such )?(?:a |an )?nobody\\s*[.!?,]*$"));
    s_SELF_JOB(py_str("(?:mum|mom|mother|dad|father|parent|husband|wife|partner|friend|person|son|daughter)"));
    s_SELF_BLAME(py_tuple(vec![py_str("That's a heavy way to feel about yourself. What's been making you feel like that?"), py_str("I'm sorry it feels like that. What would make today a bit easier?"), py_str("That sounds like a hard thing to carry. What happened that started it?"), py_str("I hear you, and it's worth saying out loud. What's the part that bothers you most?")]));
    s_CLOSED_DOOR(py_tuple(vec![py_str("That's a hard thing to keep going back to. What sort of work are you after?"), py_str("Applying and hearing nothing wears a person down, and it is not about what you are worth. How long has this been going on?"), py_str("A door that keeps closing is exhausting. What have you been trying for?"), py_str("It's a rough time to be looking. What is the job you are chasing?")]));
    s_NEGATIVE(py_tuple(vec![py_str("That sounds rough. If you want, I can help you think through what to do next."), py_str("Ugh, that's a lot to carry. What would make today a bit easier?"), py_str("I'm sorry — that's frustrating. Want a hand working out the next step?"), py_str("Sorry to hear that. What would help most right now?")]));
    s_NAMED_NEGATIVE(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NEGATIVE()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("frustrating")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_SERIOUSLY_ILL(py_tuple(vec![py_str("That's frightening news about someone close. How are you doing?"), py_str("That is the kind of news that puts everything else on hold. How is it going now?"), py_str("Worrying about someone you love is its own kind of tiring. What do you need first?"), py_str("That's a lot to have land at once. What is the hardest part of it today?"), py_str("I'm sorry. That is frightening, and it is a lot to carry. Who is looking after you in it?")]));
    s_A_LOVE_FOR_SOMEONE(py_tuple(vec![py_str("That's a lovely thing to say. What do they mean to you?"), py_str("That's a warm thing to say. What are they like?"), py_str("That's lovely. What's the best thing about them?"), py_str("That's good to hear. What do you two do together?"), py_str("That's lovely to hear. What is it about them that makes you say that?"), py_str("That's a good thing to have. What do you two talk about?")]));
    s_A_LOVE_FOR_PEOPLE(py_tuple(vec![py_str("That's a lovely thing to say. What are they like?"), py_str("That's warm. What's the best thing about them?"), py_str("Good to hear. What do you all do together?"), py_str("Lovely. What is it about them that you love most?"), py_str("That's lovely to hear. What are they like together?"), py_str("That's a good thing to have. What do you all talk about?")]));
    s_LEFT_POOL(py_tuple(vec![py_str("That's a hard thing to hear. Where are you with it - has it sunk in yet?"), py_str("Sorry. That is a lot to be carrying. Is there anyone you can say it to tonight?"), py_str("That's rough, the person you live with saying that. What's it like in the house?"), py_str("I'm sorry. Do you want to say what has been going on?")]));
    s_LEFT_POOL_GONE(py_tuple(vec![py_str("That's a hard thing to hear. Where are you with it - has it sunk in yet?"), py_str("Sorry. That is a lot to be carrying. Is there anyone you can say it to tonight?"), py_str("I'm sorry. Do you want to say what has been going on?")]));
    s_A_THING_LOOKS_SAD(py_tuple(vec![py_str("Fair - what's it doing that looks sorry for itself?"), py_str("A sad-looking thing usually has one thing wrong with it. What have you noticed?"), py_str("Ha, things do get like that. What's it up to?"), py_str("Tell me what it looks like and I'll say what usually fixes it.")]));
    s_SOMEONE_LEAVING(py_tuple(vec![py_str("That's a real change to your week. What will you miss most?"), py_str("How far is it, and are you a phone-call pair or a message-one?"), py_str("Are you getting one more evening together before they go?"), py_str("Nobody has done anything wrong and it still hurts — that is how it goes when someone moves away. Will you get to visit?"), py_str("Somebody going leaves a hole in the week, not just the town. Do you keep in touch much as it is?")]));
    s_A_CHILD_LEAVING(py_tuple(vec![py_str("The house is about to get quieter. Which part of it are you dreading?"), py_str("Proud and a bit hollow at the same time, I'd guess. Has it been on the cards long?"), py_str("So there's going to be an empty room in your house. How are you feeling about it?"), py_str("How far away is it going to be? That usually decides how it feels."), py_str("Are you doing something together before they go? Those last meals tend to be how it gets marked.")]));
    s_SOMEONE_GONE(py_tuple(vec![py_str("That leaves a gap in the week, not just the town. What do you miss most?"), py_str("How far away are they now, and are you a phone-call pair or a message-one?"), py_str("Nobody did anything wrong and it still stings — that is how a move lands. Have you been out to see them yet?"), py_str("The first weeks are the odd ones. Does it feel more like a visit when they come back?")]));
    s_A_CHILD_LEFT(py_tuple(vec![py_str("The room being empty is the hard bit. How long has it been quiet?"), py_str("How far away are they now? That usually decides how often you actually see each other."), py_str("Proud and a bit hollow at once, and both are allowed. What do you miss most?"), py_str("Does the house feel different, or is it the evenings that are strange?")]));
    s_FOUND_IT(py_tuple(vec![py_str("Nice. Where did it turn up? It is usually the boring place after all."), py_str("Good. That is the relief - where was it hiding?"), py_str("Oh good. Was it somewhere you had already looked?")]));
    s_LOST_A_THING(py_tuple(vec![py_str("Retrace your steps in order and check the boring places again - coat pocket, under the car seat, the last counter you were at. It is almost always in one you have already looked."), py_str("Go back to the last place you definitely had it while someone there can still help, and ring it if it is a phone. Start where you took it out last, not where you think it is."), py_str("Check the two places you would swear it is not, slowly. That is where it turns up once the quick look has already failed.")]));
    s_LOCKED_OUT_LINES(py_tuple(vec![py_str("The way in comes first: the spare, a neighbour who has one, the landlord or letting agent, then a locksmith who quotes before setting off. The keys can turn up after you are inside."), py_str("Get back in before you look for anything: spare key, neighbour, landlord, then a locksmith. Ask the price on the phone before they come out."), py_str("Don't force the door — a spare with a neighbour or family is the cheap way, then the landlord or a locksmith. Once you are in, one hook by the door stops it happening again.")]));
    s_LOCKED_OUT_OF_THE_CAR(py_tuple(vec![py_str("The car comes first: the spare set at home, then a breakdown service or a locksmith who does cars — cheaper than the window. The keys can be found once you are moving again."), py_str("Don't break the window unless someone is in danger. Spare set, breakdown cover, or a locksmith who does cars; the keys are a problem for after that.")]));
    s_LOST_A_THING_AGAIN(py_tuple(vec![py_str("Give it one home - a bowl by the door, the same pocket every time - and use it for a week. Losing the same thing twice is usually the place moving, not the memory."), py_str("Pick one spot now and put it there every time you come in, even when you are tired. That is the whole trick, and it works better than trying to remember harder.")]));
    s_FORGET_A_THING(py_str("forget(?:s|ting)?\\s+(?:to\\s+\\w+|my\\b|the\\b|our\\b|his\\b|her\\b|where\\s+(?:i|we)\\s+(?:put|left|parked)\\b)"));
    s_DATES_KEEP_SLIPPING(py_tuple(vec![py_str("Write it down where you already look - the phone calendar, not a bit of paper - and set the reminder at the moment you make the plan, a day before and again that morning. The date is never the thing that slips; the second step is."), py_str("One diary, and it has to be the one you check anyway. Set two alarms as you write the date in: the night before and an hour before, so the remembering is already done."), py_str("The reminder has to be made while the plan is still in front of you - a day before and again that morning. Written down later, it is a note about something that already happened.")]));
    s_DOUBLE_BOOKED(py_tuple(vec![py_str("Say yes to the plan only once it is in the diary, and check that diary before you agree to the next one. Booking things twice is a diary that lives in two places, so it is one place that fixes it."), py_str("One calendar, and every yes goes into it while the person is still in front of you. The clash is not the forgetting - it is the second diary that only exists in your head."), py_str("Before you say yes, open the day and look at it. Two things at once is what a diary in your phone and a diary in your head come to in the end.")]));
    s_SECRETS_KEEP_SLIPPING(py_tuple(vec![py_str("Let something else remember them: a password manager fills them in, and you only keep one strong passphrase in your head. If you would rather not use one, a notebook kept at home and never in your bag beats reusing the same two."), py_str("The remembering is the part to give away - a password manager does the filling in and the generating, and the browser on your phone already offers one. Then there is nothing to forget but the one passphrase."), py_str("Same password everywhere is what the forgetting pushes people into, and that is the bit that hurts. A manager, or the one your browser already has, keeps them and types them for you.")]));
    s_KEEPS_FORGETTING(py_tuple(vec![py_str("Put the reminder where the job happens rather than in your head: whatever you need, right next to where you use it. What is it that keeps slipping?"), py_str("Tie it to something you already do every day and it stops needing a memory of its own. Which one do you keep missing?"), py_str("Leave the thing itself in the way of the job, so the job does the reminding instead of your head. What else keeps slipping?"), py_str("Keep the things that slip in one list you check anyway — your phone notes, a notebook by the door — so your head is not the place they live. Which one is it?"), py_str("One reminder in the right place beats trying harder to remember. Which one keeps getting past you?")]));
    s_SUDDEN_LOSS(py_tuple(vec![py_str("I'm so sorry. That's a real loss, and it deserves to be taken seriously — who have you got around you this week?"), py_str("I'm really sorry. Take the time you need, and tell me what would help today."), py_str("That's heartbreaking, and I'm sorry. Is there someone you can be with today?")]));
    s_THE_FIRST_HOUR_AND_A_CHILD(py_tuple(vec![py_str("Oh, I'm so sorry. Nothing has to be said in the next hour. When it is said, plain and with their name is enough - that they died and are not coming back - and the questions can come after."), py_str("I'm so sorry. There is no gentle way to say it and there does not have to be: their name, and what happened, in one or two sentences, and let them ask what they ask. The rest can wait."), py_str("Oh, I'm so sorry, and what a thing to be carrying into today. Say it the plain way, with the name the child already uses, and use the words for the death itself rather than for being lost or asleep. Nothing else has to be got right today.")]));
    s_PREGNANCY_BLEEDING(py_tuple(vec![py_str("Ring your midwife or the early pregnancy unit today — bleeding in pregnancy gets checked the same day, not at your next appointment. If it's heavy or you're in pain, ring 999."), py_str("That needs a midwife today rather than a wait: ring your antenatal ward or the early pregnancy unit now, and ring 999 if the bleeding is heavy or you have pain.")]));
    s_SYMPTOM(py_tuple(vec![py_str("Sorry to hear it — go easy on yourself today, and see someone if it's not shifting in a few days."), py_str("Ugh, that's rotten. Take it slow today and don't push through it; get it looked at if it gets worse.")]));
    s_CARING(py_tuple(vec![py_str("Poor thing. Keep an eye on how they are in themselves - drinking, eating, perking up - and ring 111 if that changes. What's it doing now?"), py_str("Sorry to hear it. Plenty to drink and let them rest; a pharmacist can look at it tomorrow if it's no better. How long has it been like this?"), py_str("That's a worry when it's someone you look after. Are they drinking and keeping it down?"), py_str("Ugh, poor them. What's worrying you most about it?"), py_str("Being the one looking after them is its own kind of tiring. What have you noticed so far?"), py_str("Keep them comfortable and watch whether they pick up; if they go downhill or stop drinking, get them seen the same day. How are they in themselves?")]));
    s_CARING_FOR_A_BODY(py_tuple(vec![py_str("That's hard when it's someone you look after. Is it slowing them up getting about?"), py_str("Poor thing. Is it worse at any time of day, or about the same all the time?"), py_str("Ugh, that's rough on them. Is anything helping at all - heat, a cushion, taking it gently?"), py_str("Worth asking a pharmacist what pain relief is safe for them, and keeping them moving gently rather than resting it right up. How long has that been going on for?"), py_str("That's a worry when it's someone you care about. Is it stopping them sleeping?"), py_str("Being the one watching them struggle with it is tiring too. What are they finding hardest?")]));
    s_PET_TROUBLE(py_tuple(vec![py_str("A vet is the one to ask, and a call to the desk is usually free. What's it doing, and when did it start?"), py_str("Worth a ring to the vet rather than a wait on it - they would rather hear it early. How long has this been going on?"), py_str("Sorry to hear it. Keep them warm and drinking, and ring the vet if it gets worse; is it eating and drinking as usual?"), py_str("That's the sort of thing a vet sorts out in one visit. Is it on one side, or all over?"), py_str("Don't wait on it with an animal - they hide it well, so a vet is the safer call. When did you first notice it?")]));
    s_PET_LOW_REPLY(py_tuple(vec![py_str("A pet that seems low is often ill or in pain rather than sad, and they hide it well. Has her eating, drinking or hiding changed? A vet check comes first, then the behaviour makes more sense."), py_str("That is worth a vet's eye before a guess: a quiet animal is often sore or unwell. What has changed most - food, sleep, or wanting company?"), py_str("Dogs and cats do get low, but a change in mood is a vet question first. Is he still eating and drinking as usual, and when did you first notice?"), py_str("Don't wait on a change like this in an animal - pain and illness can look like sadness. Has anything changed in eating, sleeping or where she hides?")]));
    s_PET_CONDITION_TROUBLE(py_tuple(vec![py_str("A vet is the one to ask, and a call to the desk is usually free. When were they diagnosed, and are they on anything for it?"), py_str("Worth a ring to the vet rather than a wait on it - they would rather hear it early. How are they in themselves - eating, drinking, energy?"), py_str("Sorry to hear it. Keep them warm and drinking, and ring the vet if it gets worse; is it eating and drinking as usual?"), py_str("Don't wait on it with an animal - they hide it well, so a vet is the safer call. When did you first notice it?")]));
    s_LUMP(py_tuple(vec![py_str("A lump is worth showing to someone rather than watching: ring your GP surgery and say you have found a lump - that gets you seen quickly, and most lumps turn out to be nothing."), py_str("Get it looked at rather than waited on: ring your GP surgery in the morning and say you have found a lump. They see people quickly for that, and it is usually nothing.")]));
    s_SYMPTOM_COLD(op_add(&py_tuple(vec![py_str("That sounds miserable. Rest, warm drinks and an early night do more than anything I could add.")]), &g_SYMPTOM()));
    s_PENDING_WHEN(py_tuple(vec![py_str("Then it's out of your hands now — when do you find out?"), py_str("Fingers crossed then. When will you know?"), py_str("So it is out of your hands for now. When do you hear back?"), py_str("When do you find out, then? I'll keep everything crossed.")]));
    s_PENDING_WAITING(py_tuple(vec![py_str("Then it's just the waiting now — the longest bit. Tell me how it lands."), py_str("Fair enough, that's out of your hands until then. Let me know how you get on."), py_str("Nothing more you can do until then. I'm here when you hear.")]));
    s_PENDING_DOUBT(py_tuple(vec![py_str("You might be being hard on yourself. When do you find out?"), py_str("That is the worst bit, the guessing while you wait. When will you know?"), py_str("Fingers crossed you did better than you fear. When do you hear?")]));
    s_PENDING_LANDED(py_tuple(vec![py_str("What did they say?"), py_str("Go on then - what was the answer?"), py_str("Right, tell me how it went.")]));
    s__ISSUER(py_str("(?:hospital|doctor|gp|surgery|clinic|nhs|dentist|school|college|university|uni|employer|company|work|hr|bank|landlord|letting agent|council|dwp|home office|immigration|visa office|court|tribunal|insurer|insurance)"));
    s_A_COST_SOMEBODY_ASKS(py_tuple(vec![py_str("That is a lot to be asked for. Say the number out loud to them now - most of these bills grow because nobody says it early."), py_str("You can want to be part of it and still not pay that. Say what you can do and let them decide what happens next."), py_str("Ask what the money is actually for before you agree to it: a lot of these bills are one person's plan that nobody else has costed."), py_str("Would you still want to go if it cost less? That is the question under this one, and it is worth knowing your own answer before you reply.")]));
    s_A_SUGGESTED_BY_SOMEONE(py_tuple(vec![py_str("{who} may be onto something. What's the first step you could do this week?"), py_str("{who} will know better than me. What's stopping you starting?"), py_str("{who} had a reason for saying it. Try the smallest version and see how it sits."), py_str("{who} thought it worth saying. Give it a proper try before you decide.")]));
    s_A_SUGGESTED(py_tuple(vec![py_str("Worth a go. What's the first step you could do this week?"), py_str("Sounds like good advice. What's stopping you?"), py_str("Then try it once and see. Nothing has to be decided today."), py_str("Start with the smallest version of it and see how it sits.")]));
    s_NEUTRAL(py_tuple(vec![py_str("Got it. Is there anything you'd like a hand with?"), py_str("Thanks for telling me — I'm here if you want to dig into any of that."), py_str("I'm listening — say more if it helps.")]));
    s_A_RECEIPT_OF_NOTHING(stdlib_call("re", "compile", &[py_str("^\\s*(?:got it[.!]?\\s*(?:is there anything you'?d like a hand with\\??)?|thanks for telling me\\s*[\\u2014\\u2013-]?\\s*i'?m here if you(?:'d| would)? ?(?:want|like) to dig into any of that)\\s*[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COLD_OPEN(py_tuple(vec![py_str("Go on - what's been happening?"), py_str("What do you need? I'll take it from there."), py_str("What would be most useful to sort out?")]));
    s_BODY_SAYS_WHY(call_builtin("frozenset", &[py_tuple(vec![py_str("awful"), py_str("terrible"), py_str("dreadful"), py_str("rotten"), py_str("lousy"), py_str("poorly"), py_str("unwell"), py_str("ill"), py_str("sick"), py_str("fluey"), py_str("achy"), py_str("shaky"), py_str("faint"), py_str("nauseous"), py_str("knackered"), py_str("exhausted"), py_str("flu"), py_str("fever"), py_str("cold"), py_str("migraine"), py_str("bug"), py_str("covid")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_KNOCKED(py_tuple(vec![py_str("Ah, that's a pain. Does it put the whole thing off?"), py_str("That's a nuisance. Can it happen another time?"), py_str("Sorry to hear it. Does it change what you were going to do?"), py_str("That throws it up in the air a bit. What's the plan now?")]));
    s_DECLINED_PLAN(py_tuple(vec![py_str("No trouble at all - these things happen. If you want a hand working out what to say to whoever is expecting you, I can help with that."), py_str("That's alright. Do you want to rearrange it, or let this one go?"), py_str("Fair enough, no need to explain. Say the word if you want a hand putting it in a message."), py_str("No worries. If they are going to ask why, it helps to have one sentence ready - want to work one out together?")]));
    s_DECLINED_PLAN_GENTLE(py_tuple(vec![py_str("Of course - nobody would expect you to. Look after yourself today."), py_str("That's understood, and no explanation needed. Be gentle with yourself."), py_str("Of course not. Put yourself first just now, and I'm here if you want to talk."), py_str("Nobody would think twice about that one. Take care of yourself today.")]));
    s_NO_WORDS_SOMEONE(py_tuple(vec![py_str("Start with one true sentence and stop: \"I have been thinking about what you told me.\" The rest comes once you are talking."), py_str("You do not need the right words, only a true one - say you are glad they told you, then let them do the talking."), py_str("Keep it small and mean it: name the thing in one line, then ask how they are today. That is enough to open it.")]));
    s_NO_WORDS_STUCK(py_tuple(vec![py_str("Then start anywhere - the first bit that comes to mind is usually the right one."), py_str("No rush. Say the part that is loudest in your head and we can sort the rest of it."), py_str("That is alright. Say it badly if you have to; the words come easier once the first one is out.")]));
    s_NO_WORDS_AT_THE_CARE(py_tuple(vec![py_str("Lead with the trouble, not the history: one sentence on what is wrong, then how long it has been going on and what makes it worse. They get to the rest faster that way."), py_str("Say the thing you are worried about first - \"it has been hurting for three weeks\" - then what you have noticed since. That is what they need to sort it."), py_str("Put the main worry in the first sentence and let them ask the rest. Take the dates and what you have already tried with you.")]));
    s_NO_WORDS_ASKING(py_tuple(vec![py_str("Say what you want out of it first and stop: \"I would like to talk about the shifts.\" The detail can come after - the ask is the sentence that has to be said."), py_str("Decide the one thing you are asking for, put it in the first line, then let them answer. Everything else is easier once they know what you want."), py_str("Lead with the ask, not the story: one sentence about what you want, then the reason. Say it plainly and you will not need to rehearse it.")]));
    s_ACTS_FOR_ME(py_tuple(vec![py_str("That's a job in the world and I can't reach one - no hands, no phone, nothing outside this chat. What I can do is the thinking: the steps in order, or what to say when you ring. Say which bit you're stuck on."), py_str("I can't do that one myself, sorry - I'm words on a screen. Hand me the part that needs working out instead: the list, the order to do things in, or the call you're dreading."), py_str("No hands on this end, so the job itself can't be mine. What I'm good for is the plan behind it - say what's making it awkward and I'll help with that."), py_str("Nothing can be fetched, sent or signed from this end - no browser, no inbox, no hands. Name the thing and I'll do the part that lives in words: the method, the wording, or the questions for whoever can.")]));
    s_INVITED(py_tuple(vec![py_str("I'd come if I could - I'm only words on a screen. What's the plan?"), py_str("Wish I could be there. Who are you going with?"), py_str("Sounds a good idea - tell me how it goes."), py_str("Lovely idea. I'd be there in a shot if I had legs.")]));
    s_INVITED_TO_TALK(py_tuple(vec![py_str("Go on then - what's on your mind?"), py_str("Always. What do you want to talk about?"), py_str("I'm here for it. What's up?")]));
    s_INVITE_FILLER(call_builtin("frozenset", &[mcall(&py_str("to and a an the some out on for round along up with me us down over it"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_INVITE_ACTIVITY(call_builtin("frozenset", &[mcall(&py_str("\nwalk stroll wander hike ramble coffee tea brew cuppa pint drink drinks beer wine lunch dinner\nbreakfast brunch bite snack takeaway curry chinese pizza kebab chips film movie picture cinema\nshow theatre gig concert game quiz run jog swim cycle bike ride picnic shopping shop market party\ndance paddle drive pub cafe match footie football bowling karaoke barbecue bbq beach park garden\nmuseum join\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_INVITE_PRONOUN(call_builtin("frozenset", &[mcall(&py_str("her him them you me us it themself"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_INVITE_NEVER(call_builtin("frozenset", &[mcall(&py_str("die dies died dying dead death kill killed myself suicide overdose harm hurt hurting blood bleed bleeding hospital ambulance 999 111 funeral cancer"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ONLY_FRONT(py_str("^(?:(?:and|but|so|ok|okay|well|maybe|just|also|oh|no|yes|yeah|hmm|actually|honestly)[\\s,]+)*"));
    s_CUT_FOR(py_dict(vec![(py_str("far"), py_tuple(vec![py_str("Then cut those for distance: how long a journey is still worth it?"), py_str("Nearer, then. How far are you happy to travel?"), py_str("Then keep to the ones you can get to easily. How long a trip is fine?")])), (py_str("price"), py_tuple(vec![py_str("Then cut those for price: what's the most you want to spend?"), py_str("Cheaper, then. What's your limit?"), py_str("Then let's keep to a budget. What's the ceiling?")])), (py_str("time"), py_tuple(vec![py_str("Then fit those to the time you have: how long have you got?"), py_str("Then the short version. How many minutes have you got?"), py_str("Then let's fit it to the time you have. How long is that?")])), (py_str("effort"), py_tuple(vec![py_str("Then take the easiest of those. How much are you up for?"), py_str("Then nothing fiddly. How little effort do you want to put in?"), py_str("Then the simplest of those. How much work is too much?")])), (py_str("size"), py_tuple(vec![py_str("Then cut those for size: how much room have you got?"), py_str("Smaller, then. What are the measurements you're working with?"), py_str("Then let's get the size right. What has it got to fit into?")]))]));
    s_BOUND_FOR(py_dict(vec![(py_str("far"), py_tuple(vec![py_str("Then within {bound}: which of those is close enough?"), py_str("Then {bound} it is. Which of those is a journey you'd make?")])), (py_str("price"), py_tuple(vec![py_str("Then keep to {bound}: which of those comes in under that?"), py_str("Then {bound} is the ceiling. Which of those fits inside it?")])), (py_str("time"), py_tuple(vec![py_str("Then {bound} it is. Which of those fits inside it?"), py_str("Then let's fit it to {bound}. Which of those fits?")])), (py_str("effort"), py_tuple(vec![py_str("Then {bound} of effort. Which of those is the least work?"), py_str("Then nothing harder than {bound}: which of those suits?")])), (py_str("size"), py_tuple(vec![py_str("Then {bound} to work with. Which of those fits?"), py_str("Then {bound} it is. Which of those suits the space?")]))]));
    s_NARROW_WORDS(Py::Int(7));
    s_NARROW_ANSWERED(call_builtin("frozenset", &[py_tuple(vec![py_str("answers"), py_str("memory_store")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NARROW_SUBSTANCE(Py::Int(14));
    s_AXIS_MIN(py_dict(vec![(py_str("time"), Py::Int(2))]));
    s_A_GOOD_TIME(py_tuple(vec![py_str("That sounds like a good time - what was the best of it?"), py_str("Lovely. What made it as good as that?"), py_str("Glad it went well. Tell me the best bit.")]));
    s_A_GOOD_TIME_IN_A_CARE_THREAD(py_tuple(vec![py_str("That's good to hear, and a good day in a week like this one counts for more than it sounds. What made it a good one?"), py_str("I'm glad. After what you have been carrying, a day like that is worth having - what was the best of it?"), py_str("Good - and it does not undo the rest of it, which I have not forgotten. What made today different?"), py_str("That's something, in a week like this. Tell me what was good about it and I will keep it with the rest.")]));
    s_A_LAUGH(py_tuple(vec![py_str("Ha - what set it off?"), py_str("Ha, good. Tell me the bit that got you."), py_str("That's the best kind of day. What happened?"), py_str("Ha - that good? Tell me the best bit.")]));
    s_FLAT_LEAD(py_str("(?:(?:yeah|yep|yes|ok|okay|and|but|oh|ah|well|honestly|tbh|just|so|still|really|very|it|its|is|was|been|a|bit|pretty|kind of|sort of)\\s+)*"));
    s_FLAT_CORE(py_str("(?:ugh+|urgh+|blah+|bleh+|bleurgh+|blech+|sigh|fr|ngl|pfff+|same old(?: same old)?|same as (?:ever|always)|could be worse|cant complain|up and down|so so|bit of a rubbish|bit rubbish)"));
    s_TIGHT_MONEY(py_tuple(vec![py_str("Broke till payday is its own kind of week. What's the money going on - food, or something that can wait?"), py_str("Tight month, then. What's the first thing that has to be paid?"), py_str("Right before payday is the hardest bit of the month. What have you got in the cupboard?"), py_str("That's a long week when the money is done. What would help most: cheap meals, or a bill to put off?"), py_str("Skint weeks take planning, not willpower. What's left and how long till the money comes?")]));
    s_TIGHT_MONEY_MORE(py_tuple(vec![py_str("I'm not going anywhere. What's in there roughly, and when does the money land?"), py_str("Give me the two numbers and I'll work with them: what's there, and what has to come out before the money comes in?"), py_str("Then let's keep at it. What is the next thing that has to be paid?"), py_str("I can work with rough. What's roughly there, and which bill will not wait?")]));
    s_AGREEMENT(py_tuple(vec![py_str("You know it. What's today been like for you?"), py_str("Ha - you're not wrong. What's going on with you?"), py_str("Same here. Tell me how it's been."), py_str("Right? What's the worst of it today?"), py_str("You're preaching to the converted. What's it been like?")]));
    s_FLAT(py_tuple(vec![py_str("Ugh. What's the day doing to you?"), py_str("One of those days? What's up?"), py_str("Flat one, then. What's going on?"), py_str("Same as ever, then. Anything you'd like to change?"), py_str("Fair enough. What would make it better?")]));
    s_REACTION_LEAD(py_str("(?:(?:oh|ah|ooh|ooh|aw|aww|aah|my|good|god|jesus|christ|jeez|well|just|so|and|"));
    s_REACTION_OUCH(py_tuple(vec![py_str("Ouch. What happened?"), py_str("Yikes. That sounds rough - what happened?"), py_str("Ouch is right. Go on."), py_str("That's rough. What happened?")]));
    s_REACTION_DISBELIEF(py_tuple(vec![py_str("Go on - what happened?"), py_str("Really? Tell me what happened."), py_str("No kidding. What's going on?"), py_str("Straight up? Tell me more.")]));
    s_REACTION_RELIEF(py_tuple(vec![py_str("That sounds like a relief. What was it in the end?"), py_str("Phew is right. What was going on?"), py_str("Thank god for that. What happened?")]));
    s_REACTION_MISHAP(py_tuple(vec![py_str("Oops. What did you do?"), py_str("Whoops. What happened?"), py_str("Oh dear. What was it?")]));
    s_REACTION_OUCH_SAID(py_tuple(vec![py_str("That's rough."), py_str("Ugh. Sorry - that is a rough one."), py_str("Ouch. Sorry, that sounds rough.")]));
    s_REACTION_MISHAP_SAID(py_tuple(vec![py_str("Ugh, what a pain."), py_str("Oh no. That is a pain when it happens."), py_str("Annoying one, that.")]));
    s_REACTION_RELIEF_SAID(py_tuple(vec![py_str("That is a relief."), py_str("Good. Glad that one is over."), py_str("Phew. That is the end of it, then.")]));
    s_REACTION_DISBELIEF_SAID(py_tuple(vec![py_str("That is a lot. No wonder you are taken aback."), py_str("Really. That is something."), py_str("Straight up. That is a lot to take in.")]));
    s_REACTIONS_CARE(py_tuple(vec![py_str("I'm sorry. There is nothing you have to say about it here."), py_str("That is a heavy thing to be carrying. I'm here."), py_str("No words needed for it. I'm listening, for as long as you want.")]));
    s_REACTION_REPEAT_FIRST(py_tuple(vec![py_str("What can I help with?"), py_str("Sorry - what did you want to ask?"), py_str("Not sure what you mean yet. Say a bit more?")]));
    s_REACTION_REPEAT(py_tuple(vec![py_str("I'll put it another way - what bit was unclear?"), py_str("Tell me which part and I'll say it differently."), py_str("Say what you're after and I'll come at it from another side.")]));
    s_REACTION_WORD(py_str("(?:ouch(?:ie)?s?|ow+|yikes|eek|oof|yowch|sucks|brutal|phew|finally|oops|whoops|dear|dammit|damn|bugger|crap|shoot|huh|eh|pardon|what|seriously|serious|joking|kidding|real|way|chance|surely|really|honestly|wtf|the|hell|heck|fuck|get|out|come|on|wow+|woah|whoa|omg|what on earth|why on earth|how on earth|what in the world|why in the world|devil|blazes|dickens|damn|fucking|going|goes|happen|happens|happened|happening|you|youre|are|joke|for|blimey|crikey|thank|good|god|heavens|christ|my|word|that|this|it|is|so|just|awful|terrible|horrible|rough|much|sick|the|no)"));
    s_REACTIONS_SAID(py_dict(vec![(g_REACTION_OUCH(), g_REACTION_OUCH_SAID()), (g_REACTION_MISHAP(), g_REACTION_MISHAP_SAID()), (g_REACTION_RELIEF(), g_REACTION_RELIEF_SAID()), (g_REACTION_DISBELIEF(), g_REACTION_DISBELIEF_SAID())]));
    s_VERDICT_BAD(py_str("nightmare|mess|disaster|shambles|palaver|faff|farce|letdown|let down|waste of time|waste|carry on|rip off|rip-off|kettle of fish|pain|state|sham|joke|performance|shower|ordeal"));
    s__VERDICT_LEAD(py_str("(?:(?:oh|god|jesus|christ|well|and|but|so|ha|haha|wow|bloody|right|honestly|just|man|mate|ugh|jeez|blimey)\\W+)*"));
    s__VERDICT_DET(py_str("(?:what a|what an|such a|a right|a proper|a real|bit of a)\\s+(?:(?:absolute|total|complete|right|proper|bloody|real|right old)\\s+)?"));
    s_VERDICT_WORD(py_str("(?:annoying|annoyin|a\\s+pain|such\\s+a\\s+pain|a\\s+nightmare|a\\s+hassle|a\\s+headache|a\\s+mess|a\\s+joke|a\\s+shame|such\\s+a\\s+shame|a\\s+letdown|frustrating|infuriating|infuriating|maddening|typical|sod'?s\\s+law|murphys?\\s+law|typical|a\\s+nightmare)"));
    s_VERDICT_ANNOY(py_tuple(vec![py_str("Annoying, that. Is it sorted, or still hanging over you?"), py_str("Yeah, that's a pain. What are you going to do about it?"), py_str("That is annoying. Can you sort it, or is it out of your hands?"), py_str("Annoying. Is it a quick fix or a whole thing?")]));
    s_VERDICT_LUCK(py_tuple(vec![py_str("Just your luck. What happens now?"), py_str("That's the way it goes sometimes. Is there anything you can do about it?"), py_str("Of course it did. What's the plan?")]));
    s_VERDICT_ROUGH(py_tuple(vec![py_str("That's rough. Is it easing at all, or much the same?"), py_str("Poor you. Has it been like that long?"), py_str("That wears you down. How are you managing with it?"), py_str("That's a hard one to carry around. Is anything helping?")]));
    s_VERDICT_NUMB(py_tuple(vec![py_str("I'm sorry. There are no words that help with that, and I'm not going anywhere."), py_str("I'm so sorry. Nothing needs saying right now - I'm here when you want to talk."), py_str("That is the worst kind of news. Take today as slowly as it comes."), py_str("I'm sorry. Say as little or as much as you like.")]));
    s_VERDICT_SHAME(py_tuple(vec![py_str("That's a shame. Is there any way to put it right?"), py_str("Shame, that. What happens now?"), py_str("That's a shame, that. Is it too late to sort out?"), py_str("Sorry to hear it. Anything left to do about it?")]));
    s_VERDICT_HEAVY(py_tuple(vec![py_str("That's a heavy thing to be carrying. Do you want to say more about it?"), py_str("That sounds like it has been sitting with you a while. What is it like today?"), py_str("I'm sorry. That is the sort of thing that comes back in waves - how are you holding up?"), py_str("I'm sorry. That one takes the time it takes - say as much of it as you want to.")]));
    s_VERDICT_QUEUE(py_tuple(vec![py_str("That's a long wait. Have they said anything about bringing it forward?"), py_str("Waiting is the worst part of it. What are you doing with the time?"), py_str("That drags, that. Is there anyone chasing it for you?"), py_str("That's a slog of a wait. Is there anything you can do in the meantime?")]));
    s_VERDICT_SETBACK(py_tuple(vec![py_str("That's a blow. What are you doing about it?"), py_str("Sorry to hear it. Is there anything lined up yet?"), py_str("That's rough. What happens now, do you know?"), py_str("Gutted for you. What's the plan from here?")]));
    s_VERDICT_FIRST(py_tuple(vec![py_str("Go on - what happened?"), py_str("What's made it that way?"), py_str("What happened, then?"), py_str("Right. What has gone on?")]));
    s_CAME_BACK(py_tuple(vec![py_str("No need to say sorry - I'm here. What's on your mind?"), py_str("That's fine, no harm done. What would you like a hand with?"), py_str("No bother at all - go on.")]));
    s_PAUSE_IN_A_SAFETY_THREAD(py_tuple(vec![py_str("Please don't worry about the gap - tell me how it is now, and what you've done for it."), py_str("No need for that. Say how it looks now and I'll stay with it."), py_str("That's alright. How is it now, and have you been able to keep the pressure on?")]));
    s_RETURNED_TO_IT(py_tuple(vec![py_str("Go on - I'm listening."), py_str("Right, go on."), py_str("Go on then, I'm with you."), py_str("Carry on - I'm all ears.")]));
    s_RETURNED_TO_IT_FIRST(py_tuple(vec![py_str("There's nothing earlier in this chat - what's on your mind?"), py_str("Nothing to catch up on yet - what would you like to talk about?")]));
    s_NOTHING_SAID_YET(py_tuple(vec![py_str("Nothing said yet, so this is where we start - what's on your mind?"), py_str("We're at the start of this chat: tell me the thing itself and I'll take it from there."), py_str("This is your first line, so there's nothing earlier for me to go back to - what is it?")]));
    s_RETURNED_TO_A_THING(py_tuple(vec![py_str("Go on - what about {thing}?"), py_str("Right, go on. What about {thing}?")]));
    s_PAUSE(py_str("pause"));
    s_SORRY_FOR_ASKING(py_tuple(vec![py_str("No need to apologise - asking is what I'm here for."), py_str("You're not bothering me at all. What's on your mind?"), py_str("No bother - ask away."), py_str("Nothing to be sorry about. Go on.")]));
    s_SORRY_ON_ITS_OWN(py_tuple(vec![py_str("Nothing to be sorry about."), py_str("No need to apologise - no harm done."), py_str("That's alright. Nothing to say sorry for."), py_str("No worries at all.")]));
    s_A_SORRY_ON_ITS_OWN(stdlib_call("re", "compile", &[py_str("^(?:(?:oh|ok|okay|right|so|well|hey|hi|and|but|no)\\s+)*(?:(?:that\\s*s|thats|that\\s+is|it\\s*s|its)\\s+)?(?:(?:it\\s+)?(?:was|is)\\s+)?(?:my\\s+)?(?:(?:i\\s*m|im|i\\s+am)\\s+)?(?:so\\s+|very\\s+|really\\s+|dead\\s+|truly\\s+|awfully\\s+|terribly\\s+)?(?:sorry|apologies|apologise|apologize|bad|fault|mistake)\\b(?:\\s+(?:about|for)\\s+(?:that|this|it|everything))?(?:\\s+(?:again|honestly|mate|pal|then|anyway|sorry|soz|eh))*[\\s.!,]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CORRECTION_TAKEN(py_tuple(vec![py_str("Fair enough - noted. What else should I know about it?"), py_str("Got it, my mistake. Where does that leave things?"), py_str("Right, I had that wrong. Go on - what happens next?"), py_str("Noted, thanks for putting me straight. Tell me the rest.")]));
    s_NEVER_MIND(py_tuple(vec![py_str("Fine - we can leave that one."), py_str("No problem. Say no more."), py_str("Alright - forgotten. What else is on your mind?"), py_str("Dropped. What else would you like to do?")]));
    s_A_DISMISSAL(stdlib_call("re", "compile", &[py_str("^(?:(?:oh|ok|okay|right|so|well|and|but|no|nah|hey)\\s+)*(?:never\\s*mind|nevermind|forget\\s+it|forget\\s+about\\s+(?:it|that|this|everything|all\\s+of\\s+it|the\\s+(?:whole\\s+)?(?:thing|lot))|forget\\s+(?:i|what\\s+i)\\s+said|ignore\\s+(?:that|it|me|this)|scratch\\s+that|as\\s+you\\s+were|nvm|no\\s+worries|dont\\s+worry(?:\\s+about\\s+it)?|do\\s+not\\s+worry(?:\\s+about\\s+it)?|leave\\s+it|drop\\s+it)\\b(?:\\s+(?:about\\s+(?:that|it)|that|this|it|eh|huh|then|anyway|now|thanks|ta|mate|pal|honestly|though|tho))*(?:\\s*[,;]\\s*[^.!?]{0,40})?[\\s.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_DONE(py_tuple(vec![py_str("Nice - how did it go?"), py_str("Good on you. How long did that take?"), py_str("Nice one - are you pleased with it?"), py_str("What made you start on that?"), py_str("Nice - how do you feel now it's finished?")]));
    s_A_BOOKING_AHEAD(py_tuple(vec![py_str("Nice one. When is it?"), py_str("Good stuff - when is it, and is there anything to sort before then?"), py_str("Then there is that to look forward to. What day is it?"), py_str("Good. Is it soon, or a while off?"), py_str("Nice. Does that give you much time to get ready for it?")]));
    s_A_BOOKING_AHEAD_DATED(py_tuple(vec![py_str("Then there is that in the diary to look forward to. Who are you going with?"), py_str("Good - and the date in the diary is half the job. What do you need to sort before then?"), py_str("Then there is something to look forward to. Are you going on your own or with someone?"), py_str("Nice. Does that give you long enough to get ready for it?")]));
    s_A_THING_AHEAD(py_tuple(vec![py_str("Good luck with it - what are you doing first?"), py_str("Nice. How long do you reckon it'll take?"), py_str("Hope it goes smoothly. Anything you're not sure how to start?"), py_str("Get stuck in. What's the first bit?"), py_str("Good luck with it - how far do you want to get?"), py_str("Nice. Is there any of it you're not looking forward to?")]));
    s_A_THING_SEEN_TO(py_tuple(vec![py_str("Good - that's that done."), py_str("Nice one. One less thing to see to."), py_str("Good - was it straightforward?"), py_str("Good - that's that off your mind now.")]));
    s_A_THING_BOUGHT(py_tuple(vec![py_str("Nice one - is it what you were after?"), py_str("Good. Was that a happy buy or a sensible one?"), py_str("Nice - what made you go for that one?")]));
    s_GONE_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("walked"), py_str("cycled"), py_str("ran"), py_str("swam"), py_str("hiked"), py_str("jogged"), py_str("gardened"), py_str("shopped"), py_str("went"), py_str("did")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_GONE(py_tuple(vec![py_str("Nice - how did it go?"), py_str("Good on you. How long did that take?"), py_str("What made you start on that?"), py_str("Nice one - how did that feel?")]));
    s_A_THING_ON(py_tuple(vec![py_str("Good - how's it going?"), py_str("Good - is that it done now?"), py_str("Good on you. How much longer have you got?")]));
    s_A_JOB_ON(py_tuple(vec![py_str("Good - how's it going?"), py_str("Nice - is it the messy bit yet?"), py_str("Good on you. What made you start on it?"), py_str("Nice one - nearly there, or still in the middle of it?"), py_str("Good - and is it going to plan?")]));
    s_A_JOB_SOON(py_tuple(vec![py_str("Good - how's the plan for it looking?"), py_str("Nice one. Have you got everything you need for it?"), py_str("Good on you. What made you decide to do it?"), py_str("Nice - is it a big job?")]));
    s_SEEN_TO_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("booked"), py_str("reserved"), py_str("paid"), py_str("sent"), py_str("posted"), py_str("emailed"), py_str("signed"), py_str("cancelled"), py_str("renewed"), py_str("registered"), py_str("applied"), py_str("collected"), py_str("hired"), py_str("upgraded"), py_str("submitted"), py_str("filed"), py_str("transferred"), py_str("refunded"), py_str("complained")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BOUGHT_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("bought"), py_str("ordered")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_CHOSEN(call_builtin("frozenset", &[py_tuple(vec![py_str("prescription"), py_str("prescriptions"), py_str("script"), py_str("scripts"), py_str("medicine"), py_str("medicines"), py_str("meds"), py_str("tablets"), py_str("pills"), py_str("inhaler"), py_str("inhalers"), py_str("antibiotics"), py_str("vaccine"), py_str("jab"), py_str("referral")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LATE_NOW_LINES(py_tuple(vec![py_str("Say it before they notice - one line and the time you will be there - then get moving. What has held you up?"), py_str("Tell them now, with the time you will actually be in, and stop apologising after that. What is holding you up?")]));
    s_LATE_BEHIND_LINES(py_tuple(vec![py_str("Pick the piece of it that finishes soonest and do that one now - the rest can wait. What is slowing you down?"), py_str("Being behind is a list problem, not a character one: do the smallest piece today and write down the rest. What is in the way?")]));
    s_DONE_VERB(op_bitor(&op_bitor(&call_builtin("frozenset", &[py_tuple(vec![py_str("painted"), py_str("cleaned"), py_str("tidied"), py_str("cleared"), py_str("sorted"), py_str("fixed"), py_str("mended"), py_str("built"), py_str("made"), py_str("baked"), py_str("cooked"), py_str("roasted"), py_str("brewed"), py_str("planted"), py_str("repotted"), py_str("potted"), py_str("dug"), py_str("mowed"), py_str("weeded"), py_str("pruned"), py_str("sanded"), py_str("varnished"), py_str("tiled"), py_str("grouted"), py_str("plastered"), py_str("wallpapered"), py_str("hung"), py_str("washed"), py_str("ironed"), py_str("hoovered"), py_str("vacuumed"), py_str("mopped"), py_str("scrubbed"), py_str("dusted"), py_str("decluttered"), py_str("rearranged"), py_str("sewed"), py_str("sewn"), py_str("knitted"), py_str("crocheted"), py_str("hemmed"), py_str("repaired"), py_str("assembled"), py_str("installed"), py_str("rewired"), py_str("serviced"), py_str("oiled"), py_str("walked"), py_str("cycled"), py_str("ran"), py_str("swam"), py_str("hiked"), py_str("jogged"), py_str("gardened"), py_str("shopped"), py_str("packed"), py_str("unpacked"), py_str("labelled"), py_str("filed"), py_str("wrote"), py_str("printed"), py_str("polished"), py_str("changed"), py_str("stripped"), py_str("stained"), py_str("swept"), py_str("watered"), py_str("papered"), py_str("sealed"), py_str("glued"), py_str("screwed"), py_str("nailed"), py_str("cut"), py_str("trimmed"), py_str("chopped"), py_str("picked"), py_str("froze"), py_str("pickled"), py_str("preserved"), py_str("batched"), py_str("prepped"), py_str("fired")])]).unwrap_or_else(|e| exc::raise(e)), &g_SEEN_TO_VERB()), &g_BOUGHT_VERB()));
    s_DONE_ONLY_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("fired")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_NOT_A_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("me"), py_str("him"), py_str("her"), py_str("them"), py_str("us"), py_str("everyone"), py_str("everybody"), py_str("somebody"), py_str("someone"), py_str("anybody"), py_str("anyone"), py_str("people"), py_str("staff"), py_str("team"), py_str("crew"), py_str("worker"), py_str("workers"), py_str("guy"), py_str("bloke"), py_str("man"), py_str("woman"), py_str("person"), py_str("employee"), py_str("employees"), py_str("apprentice"), py_str("lad"), py_str("lass"), py_str("girl"), py_str("boy")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_NOT_THIS(call_builtin("frozenset", &[py_tuple(vec![py_str("mess"), py_str("mistake"), py_str("error"), py_str("complaint"), py_str("decision"), py_str("appointment"), py_str("call"), py_str("offer"), py_str("promise"), py_str("excuse"), py_str("fuss"), py_str("noise"), py_str("start"), py_str("plan"), py_str("list"), py_str("wrong"), py_str("loss"), py_str("mind"), py_str("respects"), py_str("tribute"), py_str("tributes"), py_str("toll"), py_str("price"), py_str("news"), py_str("headlines"), py_str("papers"), py_str("history"), py_str("waves"), py_str("difference"), py_str("sense"), py_str("peace"), py_str("amends"), py_str("headway"), py_str("progress"), py_str("friends"), py_str("enemies"), py_str("allowances"), py_str("allowance"), py_str("sure"), py_str("certain")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ALIVE(call_builtin("frozenset", &[py_tuple(vec![py_str("puppy"), py_str("pup"), py_str("kitten"), py_str("dog"), py_str("cat"), py_str("hamster"), py_str("rabbit"), py_str("guinea"), py_str("budgie"), py_str("fish"), py_str("horse"), py_str("pony"), py_str("parrot"), py_str("chick"), py_str("chicks"), py_str("hen"), py_str("hens"), py_str("gerbil"), py_str("ferret"), py_str("tortoise"), py_str("kittens"), py_str("puppies"), py_str("dogs"), py_str("cats")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("them"), py_str("things"), py_str("stuff"), py_str("everything"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("some"), py_str("out"), py_str("up"), py_str("away"), py_str("off"), py_str("in"), py_str("on"), py_str("round"), py_str("around")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AT_IT_NOT(call_builtin("frozenset", &[py_tuple(vec![py_str("best"), py_str("right"), py_str("wrong"), py_str("nothing"), py_str("much"), py_str("anything"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("well"), py_str("same"), py_str("job"), py_str("head"), py_str("life"), py_str("late"), py_str("behind")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AT_IT_EXTRA(call_builtin("frozenset", &[py_tuple(vec![py_str("doing")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BECOMING(call_builtin("frozenset", &[py_tuple(vec![py_str("getting"), py_str("becoming"), py_str("turning"), py_str("feeling")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BECOMING_STATE(call_builtin("frozenset", &[py_tuple(vec![py_str("fat"), py_str("bigger"), py_str("thin"), py_str("skinny"), py_str("heavy"), py_str("chubby"), py_str("large"), py_str("weight"), py_str("old"), py_str("older"), py_str("tired"), py_str("sleepy"), py_str("hungry"), py_str("late"), py_str("slow"), py_str("slower"), py_str("better"), py_str("worse"), py_str("sore"), py_str("stiff"), py_str("weak"), py_str("forgetful"), py_str("grumpy"), py_str("moody"), py_str("grumpy"), py_str("stressed"), py_str("bored"), py_str("cold"), py_str("hot"), py_str("warm"), py_str("impatient"), py_str("lazy"), py_str("bald"), py_str("grey"), py_str("gray"), py_str("wrinkly"), py_str("antsy"), py_str("frustrated"), py_str("anxious"), py_str("down"), py_str("homesick"), py_str("clumsy"), py_str("deaf"), py_str("blind"), py_str("short-sighted")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KNOCKED(py_tuple(vec![py_str("Ouch. Something cold on it for ten minutes, then take it easy tonight."), py_str("Ouch - that's a sore one. Ice it and rest it, and see how it is in the morning."), py_str("That smarts. A cold pack for ten minutes, and keep it up if you can."), py_str("Ouch. Cold water or a cold pack, and if it swells up or you can't use it tomorrow, get it looked at.")]));
    s_MISHAP_BACK(py_tuple(vec![py_str("How bad is it - sore, or something that needs a look?"), py_str("Is it bad enough to get checked, or has it settled?"), py_str("Did it need a look, or was it just a fright?"), py_str("Ouch. How is it now - still sore, or has it eased?"), py_str("How is it now? The knock is one thing; how it feels after is the bit that matters.")]));
    s_KNOCKED_SPOTS(py_str("finger|fingers|thumb|hand|hands|arm|arms|leg|legs|foot|feet|toe|toes|wrist|head|eye|eyes|knee|face|shoulder|skin|chest|stomach|shin|elbow|ankle|back|hip|nose|chin|lip|knuckle|palm|heel|ribs|neck|calf|thigh"));
    s_WALKED_INTO_POOL(py_tuple(vec![py_str("Ouch - those come out of nowhere. Is it bruising up?"), py_str("Ouch. Are you alright? Those catch you right on the bone."), py_str("Ouch - that'll sting for a bit. Is it sore to touch?"), py_str("Ouch. Does it hurt to put weight on it?")]));
    s_MYSELF(py_str("(?:i|i'?m|im|i am|me|myself)"));
    s_MY_DOING(py_str("(?:best|work|effort|parenting|teaching|grades|marks|results|cooking|running|english|maths|math|driving|swimming|performance|drawing|writing|playing|scores|revision|study|studies|business|portfolio|body|appearance|looks|speed|output)"));
    s_NOT_ENOUGH_REPLY(py_tuple(vec![py_str("That sounds like a heavy thing to be carrying, and it usually means you care about it more than the people who don't. What would \"enough\" actually look like for you?"), py_str("Feeling like that is rough, and it is rarely the true picture. What has the week actually held?"), py_str("That is a hard standard to hold yourself to. Whose is it - yours, or somebody else's?")]));
    s_NO_DIRECTION_REPLY(py_tuple(vec![py_str("Plenty of people who look settled are quietly making it up too; they just don't say so. What is the last thing you did that you actually looked forward to?"), py_str("Not knowing is normal at any age, whatever it looks like from the outside. If money were not the problem, what would you want the next few years to look like?"), py_str("That is a fair thing to be stuck on, and picking faster does not fix it. What have you tried that you would happily never do again?")]));
    s_GRIEF_AFTER(py_tuple(vec![py_str("People grieve at different speeds, and going quiet is common. Have you been able to say how you are finding it?"), py_str("That is hard, and it is lonely when you are both carrying it differently. What would you want to say to them, if it were easy?"), py_str("Most people go quiet with it rather than talk. Tell me what it is like at home and we can work out how to open it up."), py_str("That sounds like a lot to sit with on your own. Is there anyone you can say this to?")]));
    s_SUMMONS_SEEN(py_tuple(vec![py_str("Not knowing what it is about is the worst bit, so ask straight out at the start what it is about. Write down the two things you want to say before you go in and you will have the measure of it - they are usually smaller than they feel."), py_str("Ask at the start what it is about, and take a note of what they say. Waiting without knowing is harder than the meeting itself, and you can always ask for it in writing after."), py_str("Write down what you want to ask tonight, while it is quiet, and say at the start that you would like to know what it is about. Take someone with you if you would rather not go alone.")]));
    s_SUMMONS_WRITTEN(py_tuple(vec![py_str("Check the date they want a reply by before anything else, then answer in writing and keep a copy. Say what you want to know at the top of the letter, so the answer comes back to you in writing too."), py_str("Answer it in writing rather than by phone, so there is a record, and keep a copy of what you sent. Start with the one question you most want answered and let them come back on the rest.")]));
    s_SUMMONS_RANG(py_tuple(vec![py_str("Ring them back today rather than sitting on it - the waiting is usually worse than the call. Have your details in front of you and ask at the start what it is about."), py_str("Ring back today while you have it in your head, and ask straight out what it is about. If you would rather not, email is fine too: say when they rang and ask them to put it in writing.")]));
    s_GRIEVANCE(py_tuple(vec![py_str("That would get to me too. Do you want to work out what to say to them, or is it more a case of letting off steam?"), py_str("Annoying. Tell me how it has gone so far and we'll work out what to say."), py_str("Fair enough — that wears you down. Shall we think about how to raise it, or would you rather park it for now?"), py_str("I can see why that grates. What have you tried saying so far?")]));
    s_GRIEVANCE_PEOPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("sister"), py_str("brother"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("parent"), py_str("parents"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("flatmate"), py_str("housemate"), py_str("roommate"), py_str("landlord"), py_str("landlady"), py_str("boss"), py_str("manager"), py_str("colleague"), py_str("coworker"), py_str("neighbour"), py_str("neighbor"), py_str("friend"), py_str("friends"), py_str("in-laws"), py_str("someone"), py_str("somebody"), py_str("people"), py_str("ex"), py_str("exes")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEVANCE_IMPACT(call_builtin("frozenset", &[py_tuple(vec![py_str("wont"), py_str("refuse"), py_str("refuses"), py_str("refused"), py_str("doesnt"), py_str("dont"), py_str("ignores"), py_str("ignored"), py_str("ignoring"), py_str("shouts"), py_str("shouting"), py_str("yells"), py_str("yelling"), py_str("moans"), py_str("nags"), py_str("nagging"), py_str("interrupts"), py_str("interrupting"), py_str("cancels"), py_str("cancelled"), py_str("promised"), py_str("promises"), py_str("forgets"), py_str("forgot"), py_str("forgetting"), py_str("borrows"), py_str("borrowed"), py_str("borrowing"), py_str("nicking"), py_str("nicks"), py_str("rings"), py_str("ringing"), py_str("phones"), py_str("phoning"), py_str("calls"), py_str("calling"), py_str("texts"), py_str("texting"), py_str("listens"), py_str("listening"), py_str("messages"), py_str("messaging"), py_str("messaged"), py_str("dms"), py_str("dming"), py_str("dmed"), py_str("criticises"), py_str("criticizes"), py_str("blames"), py_str("blaming"), py_str("eats"), py_str("eating"), py_str("hides"), py_str("spends"), py_str("spending"), py_str("complains"), py_str("complaining"), py_str("swipes"), py_str("hoards")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEVANCE_NOT_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("own"), py_str("way"), py_str("life"), py_str("mind"), py_str("fault"), py_str("turn"), py_str("opinion")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEVANCE_PHRASES(py_tuple(vec![py_str("without asking"), py_str("every time"), py_str("all the time"), py_str("over and over"), py_str("at the last minute"), py_str("time after time"), py_str("every day"), py_str("each day"), py_str("every night"), py_str("drives me mad"), py_str("drives me up the wall"), py_str("does my head in"), py_str("keeps me awake")]));
    s_GRIEVANCE_STRAIN(py_tuple(vec![py_str("drains me"), py_str("draining me"), py_str("wears me out"), py_str("wearing me out"), py_str("wore me out"), py_str("worn me out"), py_str("takes it out of me"), py_str("is hard work"), py_str("are hard work"), py_str("is exhausting"), py_str("are exhausting"), py_str("is draining"), py_str("are draining"), py_str("winds me up"), py_str("gets me down"), py_str("getting me down")]));
    s_GRIEVANCE_STRAIN_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\s+(?:(?:so|really|very|properly|totally|just|always|dead|bloody|quite|pretty|constantly|super|a bit|abit|that|this)\\s+)?(?:exhausting|draining|tiring|knackering)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEVANCE_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|they)\\s+(?:is|are|was|were|keeps|kept)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEVANCE_HEALTH(call_builtin("frozenset", &[py_tuple(vec![py_str("cough"), py_str("coughs"), py_str("coughing"), py_str("cold"), py_str("flu"), py_str("sick"), py_str("ill"), py_str("unwell"), py_str("pain"), py_str("pains"), py_str("painful"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("dizzy"), py_str("faint"), py_str("fainting"), py_str("falls"), py_str("falling"), py_str("fallen"), py_str("fell"), py_str("forget"), py_str("forgets"), py_str("forgetting"), py_str("memory"), py_str("confused"), py_str("shaking"), py_str("swollen"), py_str("swelling"), py_str("weight"), py_str("thin"), py_str("thinner"), py_str("sleep"), py_str("sleeping"), py_str("sleeps"), py_str("tired"), py_str("exhausted"), py_str("snore"), py_str("snores"), py_str("snoring"), py_str("breath"), py_str("breathing"), py_str("bleed"), py_str("bleeding"), py_str("blood"), py_str("bruise"), py_str("bruises"), py_str("lump"), py_str("lumps"), py_str("rash"), py_str("fever"), py_str("temperature"), py_str("vomit"), py_str("vomiting"), py_str("hospital"), py_str("doctor"), py_str("gp"), py_str("nurse"), py_str("medication"), py_str("tablets"), py_str("medicine"), py_str("drinking"), py_str("drinks"), py_str("drunk"), py_str("smokes"), py_str("smoking")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEVANCE_HARM(call_builtin("frozenset", &[py_tuple(vec![py_str("hit"), py_str("hits"), py_str("hitting"), py_str("punch"), py_str("punches"), py_str("punching"), py_str("kick"), py_str("kicks"), py_str("kicking"), py_str("kicked"), py_str("slap"), py_str("slaps"), py_str("grabbed"), py_str("grabs"), py_str("pushing"), py_str("pushes"), py_str("touched"), py_str("touches"), py_str("touching"), py_str("threaten"), py_str("threatens"), py_str("threatening"), py_str("threatened"), py_str("abuse"), py_str("abusive"), py_str("bullies"), py_str("bullying"), py_str("bullied"), py_str("screams"), py_str("screaming"), py_str("swears"), py_str("cursing"), py_str("insults")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEVANCE_HARM_PHRASES(py_tuple(vec![py_str("shouting at"), py_str("shouts at"), py_str("yelling at"), py_str("yells at"), py_str("screaming at"), py_str("calls me names"), py_str("calling me names"), py_str("lost weight"), py_str("losing weight"), py_str("out of breath")]));
    s_GRIEVANCE_HEALTH_PHRASES(py_tuple(vec![py_str("wont eat"), py_str("won't eat"), py_str("doesnt eat"), py_str("doesn't eat"), py_str("not eating"), py_str("stopped eating"), py_str("stopping eating"), py_str("no appetite"), py_str("off his food"), py_str("off her food"), py_str("off their food"), py_str("lost his appetite"), py_str("lost her appetite"), py_str("forgot who"), py_str("forgets who"), py_str("forgot my name"), py_str("forgets my name"), py_str("forgot our name"), py_str("forgot where"), py_str("forgets where"), py_str("forgot how to"), py_str("forgets how to"), py_str("getting forgetful"), py_str("forgetful"), py_str("not responding"), py_str("not replying"), py_str("no reply"), py_str("hasnt replied"), py_str("hasn't replied"), py_str("isnt replying"), py_str("isn't replying"), py_str("wont reply"), py_str("won't reply"), py_str("wont answer"), py_str("won't answer")]));
    s_THREAD_PEOPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("she"), py_str("he"), py_str("they"), py_str("them"), py_str("him"), py_str("her"), py_str("his"), py_str("their"), py_str("theirs")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN(py_tuple(vec![py_str("Good — that sounds like the right next step. Tell me how you get on."), py_str("Nice one. Let me know how it goes."), py_str("That sounds sensible. Shout if it does not go to plan.")]));
    s_IDIOM_SHY(py_tuple(vec![py_str("Ha — one of those days, then. What happened?"), py_str("Oh no. Go on, what happened?")]));
    s_IDIOM_GLAD(py_tuple(vec![py_str("Ha — that good? What are you waiting for?"), py_str("Sounds like a big one. What is it?")]));
    s_PLAN_DONE(py_tuple(vec![py_str("booked it"), py_str("booked her in"), py_str("booked him in"), py_str("booked in"), py_str("sorted it"), py_str("booked an appointment")]));
    s_PLAN_DONE_ACK(py_tuple(vec![py_str("Nice — glad that's sorted. Shout if it comes back."), py_str("Good, that's one off the list. Hope it stays that way."), py_str("Glad you got it done — say the word if it turns up again.")]));
    s_A_PERSON(py_str("(?:him|her|them|my boyfriend|my girlfriend|my partner|my husband|my wife|my ex|my fiance|my fiancee|my other half|my man|my woman)"));
    s_RELATIONSHIP_DECISION(py_tuple(vec![py_str("That sounds like a big decision. How are you feeling about it?"), py_str("That is a big thing to do. How are you doing with it?"), py_str("That's a lot to be carrying. Have you decided, or are you still weighing it up?")]));
    s_ACK(py_tuple(vec![py_str("Great — shout if you need anything else."), py_str("Ha, fair enough. I'm here if you want a hand with anything."), py_str("Right — anything else on your mind?")]));
    s_ACK_AFTER_TROUBLE(py_tuple(vec![py_str("Right — I'm here if you want a hand with any of it."), py_str("Fair enough. Shout if you want to go into any of it."), py_str("Alright — I'm here if you want to come back to it."), py_str("Got it. Say the word if you want more on it.")]));
    s_ACK_WARM(py_tuple(vec![py_str("I'm here. Say more if you want to."), py_str("No rush. I'm not going anywhere."), py_str("Right. Take your time with it — tell me if you want a hand.")]));
    s_POSITIVE_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("finished"), py_str("finally"), py_str("won"), py_str("adopted"), py_str("promoted"), py_str("excited"), py_str("amazing"), py_str("happy"), py_str("proud"), py_str("celebrate"), py_str("engaged"), py_str("pregnant"), py_str("hired"), py_str("accepted"), py_str("improved"), py_str("brilliant"), py_str("delighted"), py_str("thrilled"), py_str("sorted"), py_str("succeeded"), py_str("achieved")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMETHING_CAME_APART(stdlib_call("re", "compile", &[py_str("\\b(?:died|dies|dying|dead|broke|broken|breaks|kaput|bust|busted|collapsed|flooded|leaking|snapped|conked|packed\\s+in|packed\\s+up|gave\\s+up|given\\s+up|gave\\s+in|stopped\\s+working|stopped\\s+charging|fell\\s+apart|had\\s+it|written\\s+off|beyond\\s+repair|on\\s+the\\s+blink)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WAIT_ENDING(call_builtin("frozenset", &[py_tuple(vec![py_str("finally")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATORS(call_builtin("frozenset", &[mcall(&py_str("not no never isnt wasnt arent werent hasnt havent hadnt didnt dont doesnt wont cant cannot"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_AT_ALL(call_builtin("frozenset", &[mcall(&py_str("nothing nobody none nowhere neither nor noone"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRAST(call_builtin("frozenset", &[mcall(&py_str("but though although yet however still and because cos cause so when while"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATIVE_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("failed"), py_str("fail"), py_str("failing"), py_str("lost"), py_str("lose"), py_str("worried"), py_str("worry"), py_str("stressed"), py_str("stress"), py_str("lonely"), py_str("sad"), py_str("tired"), py_str("angry"), py_str("upset"), py_str("struggling"), py_str("struggle"), py_str("hate"), py_str("awful"), py_str("terrible"), py_str("horrible"), py_str("anxious"), py_str("scared"), py_str("afraid"), py_str("hurt"), py_str("hurts"), py_str("broke"), py_str("argued"), py_str("argument"), py_str("arguing"), py_str("crying"), py_str("cried"), py_str("miss"), py_str("missing"), py_str("worst"), py_str("exhausted"), py_str("overwhelmed"), py_str("rejected"), py_str("fired"), py_str("sick"), py_str("painful"), py_str("miserable"), py_str("hopeless"), py_str("worthless"), py_str("pointless"), py_str("numb"), py_str("drained"), py_str("annoyed"), py_str("frustrated"), py_str("frustrating"), py_str("rude"), py_str("ignored"), py_str("unfair"), py_str("nervous"), py_str("panicking"), py_str("awkward"), py_str("snapping"), py_str("snappy"), py_str("irritable"), py_str("grumpy"), py_str("cranky"), py_str("tetchy"), py_str("ratty"), py_str("depressed")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EMERGENCY_AFTER(py_tuple(vec![py_str("If you haven't already, ring 999 or 911 now, and follow what they tell you — they can hear what I can't."), py_str("Do what the emergency line says over anything I could offer, and call back the moment something changes."), py_str("Stay in touch with them and say if anything gets worse — don't wait to see if it settles.")]));
    s_HAZARD_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("fire"), py_str("fires"), py_str("smoke"), py_str("flame"), py_str("flames"), py_str("gas"), py_str("fumes"), py_str("carbon monoxide"), py_str("flue"), py_str("fire brigade"), py_str("fire service")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAZARD_AFTER(py_tuple(vec![py_str("Stay out and ring 999 or 911 from there if you haven't already: nothing inside is worth going back in for."), py_str("Get everyone out first and call from outside; tell the operator exactly what is burning or leaking and where."), py_str("Keep well clear and follow what the operator tells you - they can hear what I can't."), py_str("If you can't get out, get to a room with a window, shut the door behind you and ring 999 or 911 from there.")]));
    s_THANKS_AFTER(py_tuple(vec![py_str("Anytime. Look after yourself, and reach back out if anything changes."), py_str("You're welcome — stay safe, and don't sit on anything that gets worse."), py_str("Any time. Take care — ring again if anything changes.")]));
    s_ABUSE_AFTER(py_tuple(vec![py_str("If you want to go, go when it is safe to: phone, keys, any paperwork you can reach, and somewhere to go rather than a drive with no end. The abuse line (0808 2000 247 in the UK, 1-800-799-7233 in the US) plans this for a living."), py_str("You do not have to settle all of it tonight. If you are leaving, the quiet moment when they are out is safer than the middle of a row - and 999 if you cannot get to the door."), py_str("Yes, if you can do it without a row: take what you would not want to leave behind and go somewhere with other people in it. The rest can be sorted from there."), py_str("Going is not the only safe answer, and it is the right one more often than not. If leaving now would make tonight worse, ring 0808 2000 247 from somewhere private when you can, and 999 the moment it turns.")]));
    s_CRISIS_AFTER(py_tuple(vec![py_str("You did the right thing telling someone. Stay with a person you trust tonight, and keep that number handy."), py_str("Keep someone with you tonight, and ring back if it gets heavier — they'd rather hear from you."), py_str("I'm glad you're talking about it. Stay with the person or the line that talked you through, not on your own.")]));
    s_CRISIS_BETTER(py_tuple(vec![py_str("That's good to hear, and it can come back in waves - keep someone near you tonight and say if it lifts again."), py_str("Good. Hold on to that, and tell one person how today went so they can check on you."), py_str("I'm glad. Stay with the person you told, and ring back here or the crisis line if it gets heavy again.")]));
    s_EMERGENCY_BETTER(py_tuple(vec![py_str("That's good to hear, and it does not change what to do: stay with the emergency line and get seen today, because feeling better is not the same as being out of the woods."), py_str("Glad it feels easier. Still get checked today and stay near the phone - some things feel better well before they are better."), py_str("Good to hear. Keep the line open and say the moment anything changes; I would still get seen today.")]));
    s_ADVICE_LENGTH(Py::Int(180));
    s_ADVICE_LENGTH_BARE(Py::Int(120));
    s_ADVICE_TURNS(Py::Int(3));
    s_WHOLE_LIST_AFTER(py_str("(?!\\s+(?:all|both|every|each)\\b)"));
    s_REPORT_FILLER(call_builtin("frozenset", &[mcall(&py_str("i we me us my our im i'm i've ive id and but so then yet now today again still also just\n    a an the bit little lot lots load much really quite very it that this them those they\n    help helps helped helping work works worked working do does did doing done try tried trying\n    give gave given use used using start started keep keeping go going went with on at for in\n    of to from what you your said say suggested told me same change difference nothing better\n    worse no not any more ok okay fine thanks thank since week weeks day days fortnight couple\n    month months morning afternoon evening night been am is are was were have has had\n    again alright already far everything all one two three four five six seven eight nine ten\n    starting taking walking made make makes thing things sorer same before matter mind as\n    if none them either than like lot a its doesnt doesn't didnt didn't isnt isn't hasnt hasn't\n    havent haven't\n    wont won't have has had changed changes actually really definitely properly indeed\n    weve youve theyve earlier yesterday once twice\n    s"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_BETTER(py_tuple(vec![py_str("Good - keep an eye on it today and get it looked at if it comes back."), py_str("That's a relief. Take it easy, and say if it starts up again."), py_str("Glad to hear it. Go steady for the rest of the day and ring back if anything changes.")]));
    s_TRIED_HELPED_REPLIES(py_tuple(vec![py_str("Good - that is the one to keep doing. Most of it does its work over weeks, not days."), py_str("That's what you want to hear. Keep it going, and ease off if it starts hurting again."), py_str("Right, keep going with it. It is usually three or four weeks before you can call it done.")]));
    s_TRIED_SORTED_REPLIES(py_tuple(vec![py_str("Good - that is that sorted."), py_str("Nice one. That is the fix, then - worth remembering for next time it does it."), py_str("Good. Say if it comes back and we will take the next step."), py_str("Right, that is it sorted. Worth writing down what you did, because it usually works again."), py_str("Good - that is the end of it, then.")]));
    s_TRIED_NO_CHANGE_REPLIES(py_tuple(vec![py_str("Then it is worth a phone call rather than a longer try: a pharmacist or a doctor can look at it and tell you what else is worth trying."), py_str("If it has had a fair run, get it looked at rather than pushing on with the same thing."), py_str("Time to change tack rather than try harder - a phone call today is a better use of the next week than another go at the same thing."), py_str("That is the point to ask rather than to try harder - say what you have already done and whether there is something else worth trying."), py_str("If it has not shifted after a fair go, say so where you got the advice. More of the same is not the answer at this stage.")]));
    s_TRIED_WORSE_REPLIES(py_tuple(vec![py_str("Then stop it. Something that makes it worse is not a thing to push through - let me know what it did and we will go a different way."), py_str("Stop there, and get it looked at. Worse is the point to ask for help, not to keep going."), py_str("Leave it off. I would get that looked at rather than persevere - worse is a reason to ask."), py_str("Stop there and get it looked at. Worse than before is a reason to ask, not a stage to work through.")]));
    s_TRIED_IT_REPLIES(py_tuple(vec![py_str("How is it going with it so far?"), py_str("Any change since you started?"), py_str("Has it made any difference yet?")]));
    s_TRIED_HELPED_ELSEWHERE(py_tuple(vec![py_str("Good - keep at it. This kind of thing usually takes a few weeks before you can tell."), py_str("That's the sign it was the right fix. Give it the time it needs, and change one thing at a time so you can tell what is doing the work."), py_str("That's what you want to hear. Carry on with it, and change something if it stops working."), py_str("Right, keep going with it. It is usually three or four weeks before you can call it done.")]));
    s_TRIED_NO_CHANGE_ELSEWHERE(py_tuple(vec![py_str("Then stop doing more of the same and change one thing instead."), py_str("If it has had a fair run, stop and change one thing rather than repeating it."), py_str("Stop pushing the same way: change one thing, then give it a day so you can see which part did what."), py_str("That is the point to stop and ask rather than push on: say what you tried and what it did, and get a different idea rather than a harder one."), py_str("Two goes at the same thing is enough. What it needs now is a different approach, not more of the same - change one thing rather than trying harder.")]));
    s_TREATMENT_HARM(g_GRIEVANCE_HARM());
    s_REFUSED_REPLIES(py_tuple(vec![py_str("Ask once more, and ask what would make it a yes. A no with a reason is something you can plan around; one without leaves you guessing."), py_str("Worth one more go, put as a question: what would have to change? If it is still no, at least you know where you stand."), py_str("Then ask why not, and what would move it. What they say is worth more than the no itself."), py_str("Ask once more with one change in it: name the smallest version they could say yes to, and ask what would make the bigger one possible."), py_str("Put it to them once more in your own words and ask what would change their mind. Either answer helps you decide what to do next.")]));
    s_ELDER_AID_REPLIES(py_tuple(vec![py_str("It is rarely the stick or the pendant they are refusing - it is being the sort of person who needs one. Offer two so there is a choice: a plain stick rather than a hospital one, a wrist alarm instead of the pendant, the frame for the walk to the shop only. And let the GP or an occupational therapist say it, because from them it is advice and from you it is fuss."), py_str("Then give them something to choose between rather than something to agree to. The council's occupational therapist will come out free and bring the options with them, and the one they picked themselves is the one they will use."), py_str("A hearing aid, a stick and an alarm all say the same thing out loud - that they are older than they were - and that is the bit being refused. Ask what they would take: something that does not look medical, only for the parts they find hard, and suggested by somebody other than you.")]));
    s_ELDER_CLINIC_REPLIES(py_tuple(vec![py_str("Take them with you and let the optician or the audiologist make the case - the number on the test is the argument, and it is not yours to make. Frames in a style they like, or an aid that sits behind the ear instead of in it, is usually what settles it."), py_str("Then ask what they would wear or use, rather than what they should: the small ones, the ones that look like ordinary glasses, the ones that only come out for the television. Book the test so the answer comes from the person doing it, and go along to the appointment.")]));
    s_ELDER_DOCTOR_REPLIES(py_tuple(vec![py_str("Ring the surgery yourself and describe what you have seen, then ask what they suggest - a GP can usually find a reason to call them in that a son or daughter cannot. Offer to book it and go with them; being driven and having somebody in the waiting room makes a difference."), py_str("Then do not argue the appointment, argue the one thing that has changed and when it started. If the trip out is the problem, ask for a phone call instead of a visit, and let the GP ask them to come in. A no to the surgery is often a no to the journey."), py_str("Say what you have noticed and leave the diagnosis to the doctor - what they will not hear from you is what is wrong with them. A district nurse, a pharmacist or a friend of their own age can often get them there when a relative cannot.")]));
    s_ELDER_HELP_REPLIES(py_tuple(vec![py_str("Ask what they would take instead of what they will not: one morning a week, somebody to do the garden, you doing the shopping. The council's care needs assessment is free and it is the door to all of it, and it arrives as them asking rather than you arranging."), py_str("Start with the job they hate rather than the word carer - nobody wants to be looked after, and plenty of people will take help with the bins, the windows and the bedding. Their GP or Age UK will suggest it as well, which lands better than it coming from family."), py_str("Then find out what the no is really to: a stranger in the house, being watched, or paying for something they think they can still do. Fix that one and the help becomes a smaller yes - and say what you will not be able to keep doing, plainly, so it is not a surprise later.")]));
    s_TRIED_WORSE_ELSEWHERE(py_tuple(vec![py_str("Then stop there and put it back how it was. Something that makes it worse is not a thing to push through - tell me what it did and we will go a different way."), py_str("Stop and leave it alone. Worse is the answer, and it is worth another pair of hands rather than you carrying on."), py_str("Leave it off for now. Worse is a reason to ask, not to persevere - I can take it from here if you tell me what changed."), py_str("Stop, and treat it as the wrong fix rather than one to try harder at: say what happened, so the next idea is not the same one again.")]));
    s_EASING_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("am"), py_str("we"), py_str("we're"), py_str("were"), py_str("it"), py_str("its"), py_str("it's"), py_str("that's"), py_str("thats"), py_str("that"), py_str("this"), py_str("feel"), py_str("feels"), py_str("feeling"), py_str("felt"), py_str("doing"), py_str("getting"), py_str("a"), py_str("an"), py_str("the"), py_str("bit"), py_str("little"), py_str("lot"), py_str("much"), py_str("very"), py_str("so"), py_str("far"), py_str("more"), py_str("any"), py_str("now"), py_str("today"), py_str("tonight"), py_str("already"), py_str("and"), py_str("but"), py_str("though"), py_str("yeah"), py_str("yes"), py_str("honestly"), py_str("actually"), py_str("thanks"), py_str("thank"), py_str("you"), py_str("of"), py_str("to"), py_str("me"), py_str("my"), py_str("myself"), py_str("good"), py_str("great"), py_str("well"), py_str("is"), py_str("was"), py_str("has"), py_str("have"), py_str("been"), py_str("gone"), py_str("stopped"), py_str("better"), py_str("calmer"), py_str("calm"), py_str("alright"), py_str("okay"), py_str("ok"), py_str("fine"), py_str("easier"), py_str("lighter"), py_str("less"), py_str("awful"), py_str("bad"), py_str("s"), py_str("just"), py_str("touch"), py_str("lots"), py_str("slightly"), py_str("way"), py_str("easing"), py_str("off"), py_str("back")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE_ON_AFTER(py_tuple(vec![py_str("Of course. What would you like to talk about?"), py_str("Fair enough - what is on your mind instead?"), py_str("Yes, let's. What would you rather talk about?")]));
    s_EMERGENCY_NOT_WORKED(py_tuple(vec![py_str("There is nothing left to try at home for this: ring 999 or 911 now if you haven't, and follow what they tell you."), py_str("Don't wait for it to settle. Ring 999 or 911, and tell them exactly what you told me."), py_str("Ringing is the thing that works here — 999 or 911 now, and stay on the line with them.")]));
    s_CARE_AFTER(py_tuple(vec![py_str("If it's still worrying you, get it looked at today — a pharmacist or a walk-in centre can check it faster than waiting."), py_str("Keep an eye on it and tell me if anything changes; being seen early beats waiting.")]));
    s_CARE_AFTER_STILL(op_add(&g_CARE_AFTER(), &py_tuple(vec![py_str("Still going on, so don't sit with it: get it looked at today and say how long it has been.")])));
    s_THRESHOLD_CROSSED(py_tuple(vec![py_str("That is the thing I said to ring 999 for, so do it now: ring 999 or 911 and stay on the line. Tell them what has changed and when it started."), py_str("Ring 999 or 911 now. That is what the answer above said to call for, and this is it - say what has changed since we spoke and let them talk you through it.")]));
    s_THE_CALL_STANDS(py_tuple(vec![py_str("Ring 999 or 911 now, as I said - tell them what has changed since we spoke and stay on the line until they tell you what to do."), py_str("The call is still the thing to do: 999 or 911, now. Say what has happened since this started, and let them talk you through it."), py_str("Make the call now - 999 or 911 - and describe what has changed since we first spoke. Stay on the line and follow what they say.")]));
    s_CARE_ROUTE_BACK(py_dict(vec![(py_str("111"), py_tuple(vec![py_str("Ring 111 today, as I said - they will say how soon it needs seeing, and ring 999 if anything gets suddenly worse."), py_str("111 is still the one to ring: describe it as you did to me and say what has changed.")])), (py_str("gp"), py_tuple(vec![py_str("Your GP today, as I said - describe it as you did to me and let them decide how soon."), py_str("Ring the surgery when it opens and say what has changed since we spoke; that is the appointment to be asking for.")])), (py_str("dentist"), py_tuple(vec![py_str("The dentist today, as I said - ring when they open and say what has changed since we spoke, and ring 111 for the out-of-hours number if it starts to hurt."), py_str("Ring the dentist and say what has changed; a broken or aching tooth is seen quickly.")])), (py_str("pharmacist"), py_tuple(vec![py_str("The pharmacist as I said - take the packet, or say what you have taken, and ask them to check it."), py_str("Ask the pharmacist, and say what has changed since I first said it.")])), (py_str("walk-in"), py_tuple(vec![py_str("A walk-in centre today, as I said - no appointment, and they can look at it properly."), py_str("Get to a walk-in centre and say how long it has been going on.")]))]));
    s_CARE_AFTER_ANIMAL(py_tuple(vec![py_str("If it is still worrying you, ring the vet today - the desk will say on the phone whether it needs seeing, and asking is usually free."), py_str("Keep an eye on it and tell the vet if anything changes; with an animal, being seen early beats waiting.")]));
    s_CARE_AFTER_ANIMAL_STILL(op_add(&g_CARE_AFTER_ANIMAL(), &py_tuple(vec![py_str("Still going on, so don't wait with it: ring the vet today and say how long it has been.")])));
    s_HELLO_STILL_THERE(py_tuple(vec![py_str("Hello. I'm still here - keep doing what the emergency line told you, and tell me if anything changes."), py_str("Hi. Still here, and so is what they told you: stay with it and say if it gets worse."), py_str("Hello again. I'm not going anywhere - carry on with what the emergency line said, and tell me how it is when you can."), py_str("Hi. I'm right here. The emergency line's advice still stands, so keep going and say if anything changes.")]));
    s_HELLO_WITH_YOU(py_tuple(vec![py_str("Hello. I'm still here, and I'm not going anywhere - is anyone with you just now?"), py_str("Hi again. Still here. Are you with someone tonight?"), py_str("Hello. I haven't gone anywhere - is there someone you can be with tonight?")]));
    s_CARE_HERE(py_tuple(vec![py_str("Still here, and not going anywhere. How are you doing just now?"), py_str("I'm all right — more use to you than to myself. How are you holding up?"), py_str("Here, and in no hurry. How are you doing right now?")]));
    s_GREETING_AFTER(py_tuple(vec![py_str("Hello again. How is it doing now?"), py_str("Hi. Any change since we spoke?"), py_str("Morning. How is it today?"), py_str("Hello — how is it holding up?")]));
    s_CARE_SEEN(py_tuple(vec![py_str("If you're wondering, that's reason enough to have it seen today — a walk-in centre or a&e will tell you quickly."), py_str("Worth having it looked at today: a&e or a walk-in centre can say for certain, and that beats waiting to see."), py_str("Get it seen today — you'll know either way, and nothing about that is a fuss.")]));
    s_CARE_SEEN_ANIMAL(py_tuple(vec![py_str("Ring the vet now and say what she got into and when. They will tell you on the phone whether she needs to come in tonight, and that is usually free to ask."), py_str("Yes - the vet is the one to ask, today rather than tomorrow. Say exactly what it was and how long ago, and ask the desk whether she should be seen tonight."), py_str("Ring the vet and describe it, and ask whether she needs to be seen tonight. With an animal it is the only place that can say for certain."), py_str("Take her, or ring first and ask - a vet would rather hear it tonight than find out tomorrow. Say what she ate and when.")]));
    s_CARE_VISIT(py_tuple(vec![py_str("Go if you can. Being there settles you both, and the questions are easier to ask in person than down a phone."), py_str("Yes, go up. A short visit tonight is worth more than a long one next week, and you can ask the question in person."), py_str("Go, even briefly. Sitting with them for half an hour beats a long visit you put off.")]));
    s_CARE_VISIT_WARD(py_tuple(vec![py_str("Go if you can. Being there settles you both, and it is easier to catch the ward nurse in the evening to ask what the plan is."), py_str("Go, even briefly. Take a notebook and ask whoever is on the ward what happens tomorrow before you leave.")]));
    s_VISIT_PRONOUNS(call_builtin("frozenset", &[mcall(&py_str("he she they him her them his hers their mum mums mam dad dads\n    mother father son daughter gran granny nan grandad grandma grandpa sister brother"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BURN_AFTER(py_tuple(vec![py_str("Nothing on it: butter, toothpaste, cream and ice all hold the heat in or set up an infection. Cool running water for twenty minutes is the treatment, then cover it loosely with cling film or a clean non-fluffy cloth."), py_str("Don't put anything on a burn — not butter, not cream, not ice. Twenty minutes under cool running water, then a loose clean dressing; painkillers help while it cools."), py_str("Cool water only, I'm afraid. Grease, toothpaste and ice are the three things that make a burn worse; twenty minutes under the tap, cover it loosely, and get it seen if it blisters or is bigger than your palm.")]));
    s_BURN_BLISTER(py_tuple(vec![py_str("Leave the blister whole — popping it is how a burn gets infected. Keep it covered loosely and get it seen if it is bigger than your palm, on the face or hands, or the skin is white or charred."), py_str("Don't pop it. A blister is a sterile dressing of its own; cover it loosely and let it settle, and have a burn that blistered looked at today.")]));
    s_ADVICE_TIME(py_tuple(vec![py_str("Keep going to the end of the time. That part is doing the work, and stopping early is usually what makes it start again."), py_str("Longer than it feels like it should be, I know. The whole of the time is what settles it; cut it short and it often comes back."), py_str("Give it the full time rather than a guess. It is a dull twenty minutes standing there, and it is the bit that matters most.")]));
    s_ADVICE_COOL(py_tuple(vec![py_str("Cool is enough. It is the running water and the time that do the work, not how cold it is - cold enough to numb it is too much."), py_str("Lukewarm is fine. Keep it running over the area rather than sitting in a bowl, and give it the whole of the time.")]));
    s_ADVICE_AGAIN(py_tuple(vec![py_str("You can go back to it. If it starts again, the same thing for the same time is right - that is repeating the answer, not overdoing it."), py_str("Yes, and doing it twice is not a mistake. The time in the answer is a round rather than a limit.")]));
    s_ADVICE_WORSE(py_tuple(vec![py_str("Then that is your cue to get it looked at rather than wait longer - the time in the answer is how long to give it, not how long to put up with it."), py_str("If it has not settled by the end of that time, stop waiting and have it seen today. Ring 111 if you are not sure where to go.")]));
    s_ADVICE_COOL_WORDS(py_tuple(vec![py_str("water"), py_str("cool"), py_str("cold"), py_str("burn"), py_str("scald"), py_str("soak"), py_str("sink"), py_str("tap")]));
    s_HOLD_AFTER(py_tuple(vec![py_str("Still here. Keep the line open and follow what they tell you."), py_str("I'm with you — stay put and let the operator talk you through it."), py_str("Right with you. Stay where you are; the crew are on their way.")]));
    s_STILL_HERE_AFTER(py_tuple(vec![py_str("Still here — keep an eye on it and say if anything changes."), py_str("Right here with you. Tell me if it gets worse or anything changes."), py_str("I'm here. Take it slowly, and say if anything changes.")]));
    s_SORTED_AND_NO_CALL_YET(py_tuple(vec![py_str("Stay on this with me: get it checked anyway, and say what happened and how long ago, even if you feel fine."), py_str("Glad it is dealt with. Tell someone you trust what happened, and keep the number handy."), py_str("Good to hear. I am here — ring them back if anything changes, and do not keep it to yourself tonight.")]));
    s_HELPLINE_NOT_AN_AMBULANCE(py_tuple(vec![py_str("That was worth doing, and nobody is coming from a helpline: ring 999 now and stay where you are until they do."), py_str("Good that you rang them, and this still needs 999 — ring for an ambulance now, rather than waiting to see how it goes.")]));
    s_WITH_YOU_AFTER(py_tuple(vec![py_str("Good, that helps. Keep them with you and keep the line open."), py_str("That's good to hear. Two of you is better than one — say if anything changes."), py_str("I'm glad you're not on your own with it. Stay together and keep the line open.")]));
    s_RESOLVED_AFTER(py_tuple(vec![py_str("Good — that's the main thing. Keep an eye on it for a bit, and ring again if anything changes."), py_str("Glad to hear it. Stay with it a little longer, and get help straight away if it starts again."), py_str("That's the important bit done. Look after yourself now, and get seen if anything flares up.")]));
    s_THE_CALL_STANDS_ANYWAY(py_tuple(vec![py_str("Good to hear it, and it does not change the call: what you described is why 999 is the number, and feeling fine for a minute is not the check. Ring them now and say it as you said it to me."), py_str("I know it feels fine now, and that is worth something - it is not the thing you were told to ring 999 for. Make the call and tell them what you told me; they would far rather hear it early."), py_str("Glad it has eased. Ring 999 anyway and let them decide - an easing is not the same as being looked at, and they can hear what I cannot.")]));
    s_CRISIS_AFTER_THEM(py_tuple(vec![py_str("Stay with {who} tonight if you can, and say out loud that you are glad {subj} told you - the saying it badly matters far less than the staying."), py_str("Take anything {subj} could take out of the room, keep {who} company tonight, and ring 116 123 together rather than for {obj}: the crisis line talks to the person and to the one holding on with them."), py_str("If there is any chance {subj} {be} in danger tonight, that is 999 or 911 - and 116 123 is the crisis line either of you can ring, at any hour, with no wrong time to use it."), py_str("Keep {who} close tonight and ask {obj} again tomorrow. Ringing 116 123 with {obj} there is easier than ringing it for {obj}, and it costs nothing to do it before it gets worse."), py_str("I can't promise how it will go, and nobody can. What helps tonight is taking anything {subj} could take out of the room, staying with {who}, and ringing 116 123 together rather than for {obj}.")]));
    s_CRISIS_BETTER_THEM(py_tuple(vec![py_str("Glad to hear it. Keep an eye on {obj} rather than taking it as the end of it, and ask {obj} again tomorrow - feeling better for an evening is not the same as being out of the woods."), py_str("That's a relief. Stay near {who} this evening if you can and check in tomorrow; 116 123 is there for the two of you if it turns again."), py_str("Good. Say out loud that you are glad, and ask {obj} tomorrow how {subj} {be} doing - one good hour is not the same as a good week.")]));
    s_EMERGENCY_BETTER_THEM(py_tuple(vec![py_str("Feeling better is not the same as being out of the woods: get {obj} seen today and stay with {who} until somebody has looked {obj} over."), py_str("Stay with {who} and get {obj} seen today. With something swallowed the harm can go on for hours after it starts to feel easier.")]));
    s_RESOLVED_AFTER_THEM(py_tuple(vec![py_str("Good - keep an eye on {obj} for a bit, and say if anything changes."), py_str("Glad to hear it. Stay with {who} a little longer, and get help straight away if it starts again."), py_str("That's the important bit done. Look after {obj} now, and get {obj} seen if anything flares up.")]));
    s_AFTER_RESOLVED(py_tuple(vec![py_str("That's the hardest part over. Tell me if anything changes from here."), py_str("It is still a lot to hold. Take it slowly, and say if anything gets worse."), py_str("Fair enough — none of that was easy. Look after yourself now.")]));
    s_CALL_WORDS(py_tuple(vec![py_str("999"), py_str("911"), py_str("112"), py_str("111"), py_str("emergency line"), py_str("call handler"), py_str("ambulance")]));
    s_THANKS_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("thanks"), py_str("thank"), py_str("thx"), py_str("ta"), py_str("cheers"), py_str("appreciated")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AFRAID_AFTER(py_tuple(vec![py_str("It's alright to be scared, and alright to cry — that's what something like this does to people. Keep following the emergency line; you're doing the right thing."), py_str("Scary, and you're doing it. Keep going with what the emergency line tells you, and say if anything changes."), py_str("Frightening, and completely normal. Keep talking and stay with the emergency line — ring back the moment anything changes.")]));
    s_CRISIS_WORDS(py_tuple(vec![py_str("crisis"), py_str("116 123"), py_str("988"), py_str("samaritans"), py_str("helpline"), py_str("stay with you")]));
    s_SAFETY_WORDS(py_tuple(vec![py_str("999"), py_str("911"), py_str("112"), py_str("emergency"), py_str("operator"), py_str("ambulance"), py_str("paramedic"), py_str("fire brigade"), py_str("fire service")]));
    s_CARE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    safe safely unsafe outside out stairs stair lift door window smoke fire fires flame flames\n    alarm spread spreading burn burned burning bleed bleeding blood cut cuts wound bandage\n    dressing plaster pain painful hurt hurts hurting aching sore tender stinging smarting\n    stiff swollen swelling bruised numb infected infection throbbing\n    aspirin paracetamol ibuprofen painkiller painkillers antihistamine\n    worse still soak soaking soaked wait waiting\n    ambulance paramedic police operator emergency 999 911 112 doctor hospital nurse gp\n    stitches stitch tetanus dressing nurse nhs walk-in centre clinic\n    checked check conscious breathing breath awake faint dizzy vomit vomiting sick throwing\n    collapsed collapse seizure stroke heart chest epipen inhaler insulin medicine dose\n    cpr defib firefighter neighbour neighbor flat floor basement roof shaken shaking scared he she him\n    her them they their there call calling rang ring\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_HURTS(call_builtin("frozenset", &[mcall(&py_str("\n    sting stings stinging stung throbs throbbing throbbed ache aches aching ached sore tender\n    hurts hurting hurt pain painful swollen swelling bled blister blistered blisters\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_POINTER(call_builtin("frozenset", &[mcall(&py_str("\n    there call calling rang ring still wait waiting check checked out outside\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_INJURY(call_builtin("frozenset", &[mcall(&py_str("\n    burn burned burning burnt bleed bleeding blood cut cuts wound blistered\n    hurts hurting hurt pain painful stinging sting sore swollen swelling smoke fire fires flame\n    flames alarm ambulance hospital\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_PRONOUNS(call_builtin("frozenset", &[mcall(&py_str("he she him her them they their it its"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_RESOLVED(call_builtin("frozenset", &[mcall(&py_str("\n    stopped stop stopping fine better okay ok came come gone settled healed healing cleared\n    improving\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_NOT_RESOLVED(g_NEGATORS());
    s_ONGOING_WORDS(call_builtin("frozenset", &[mcall(&py_str("still worse wont cant cannot keeps keep getting more again"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REACTION_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    crying tears scared afraid fear fearful terrified frightened dreading dread panicking\n    shaking shaky shaken trembling wobbly woozy sobbing hyperventilating shock\n    dizzy lightheaded faint weak drained sick numb helpless hopeless useless\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_PHRASES(py_tuple(vec![py_str("a&e"), py_str("a & e"), py_str("a and e"), py_str("walk-in"), py_str("walk in"), py_str("minor injuries")]));
    s_GENERIC_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    happen happens happened happening now long take takes last time next else doing going\n    mean means ok okay fine right thing anything something about it that this then there\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_WORDS(op_bitor(&g_GENERIC_WORDS(), &call_builtin("frozenset", &[mcall(&py_str("\n    i im me my we us our you your he she they them their his her its it is am are was were be\n    been being and or but so if to of in on at for with from by as not no never dont doesnt\n    didnt cant cannot what whats which who whom whose how why when where do does did know think\n    mean say tell want need get go going have has had will would can could should shall may\n    might just still yet any some else thing things happening happen right ok okay yeah\n    thanks thank thx cheers ta please sorry mate lovely great good\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_THREAD_TURNS(Py::Int(12));
    s_FALLBACK_MODULE(py_str("greeting"));
    s_PRESENCE_LEFTOVER(call_builtin("frozenset", &[mcall(&py_str("now at all ok okay please hello hey hi again still then just\n    mate pal buddy good right fine there here"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRESENT(py_tuple(vec![py_str("Yes — I'm here. Go on."), py_str("I'm here, listening. Keep going."), py_str("Yes, I can hear you. What's happening?"), py_str("Still here. Take your time.")]));
    s_ACKS(call_builtin("frozenset", &[py_tuple(vec![py_str("ok"), py_str("okay"), py_str("k"), py_str("cool"), py_str("nice"), py_str("sure"), py_str("yeah"), py_str("yep"), py_str("yes"), py_str("no"), py_str("nah"), py_str("nope"), py_str("lol"), py_str("haha"), py_str("ha"), py_str("mm"), py_str("hmm"), py_str("right"), py_str("fair"), py_str("same"), py_str("great"), py_str("good"), py_str("true"), py_str("exactly"), py_str("indeed"), py_str("thanks"), py_str("thx"), py_str("ta")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGREEMENTS(call_builtin("frozenset", &[py_tuple(vec![py_str("same"), py_str("ditto"), py_str("likewise"), py_str("snap"), py_str("seconded"), py_str("agreed")])]).unwrap_or_else(|e| exc::raise(e)));
    s_IDENTITY(py_tuple(vec![py_str("are you a real person"), py_str("are you real"), py_str("are you human"), py_str("are you a human"), py_str("are you a bot"), py_str("are you a robot"), py_str("are you an ai"), py_str("are you ai"), py_str("who are you"), py_str("what is a chatbot"), py_str("are you a chatbot"), py_str("introduce yourself")]));
    s_MEMORY(py_tuple(vec![py_str("do you remember"), py_str("can you remember"), py_str("do you know what we talked"), py_str("what did we talk about"), py_str("remember what we"), py_str("remember me"), py_str("remember us")]));
    s_HELP(py_tuple(vec![py_str("what can you help"), py_str("what can you do"), py_str("how can you help"), py_str("what do you do"), py_str("what kind of things can you help"), py_str("what are you for"), py_str("how do you help"), py_str("what can i ask you"), py_str("what are you good at")]));
    s_HELP_TOPIC(py_dict(vec![(py_str("cooking"), py_tuple(vec![py_str("Cooking is my sort of thing - what have you got in?"), py_str("I can help with cooking - what are you making, and what's in the fridge?")])), (py_str("food-and-drink"), py_tuple(vec![py_str("Food I can help with - what are you in the mood for?"), py_str("Tell me what you fancy eating and I'll help you get there.")])), (py_str("home-and-diy"), py_tuple(vec![py_str("I can help with the house - tell me what it's doing and what you've tried."), py_str("Home jobs I can help with - what's the trouble?")])), (py_str("money"), py_tuple(vec![py_str("Money I can talk through - what are you weighing up?"), py_str("I can help with money questions - what's the decision?")])), (py_str("health"), py_tuple(vec![py_str("I can talk through everyday health - what's going on?"), py_str("Tell me what's bothering you and I'll say what would help.")])), (py_str("technology"), py_tuple(vec![py_str("Tech I can help with - what's it doing?"), py_str("I can help with a device - what's wrong with it?")])), (py_str("work"), py_tuple(vec![py_str("Work I can talk through - what's the situation?"), py_str("Tell me what's happening at work and I'll help you think it through.")])), (py_str("geography-and-travel"), py_tuple(vec![py_str("Travel I can help with - where are you thinking of going?"), py_str("Tell me the trip and I'll help with the practical bits.")])), (py_str("family-and-parenting"), py_tuple(vec![py_str("Family things I can help with - what's going on?"), py_str("Tell me what's happening with the family and I'll help.")])), (py_str("pets"), py_tuple(vec![py_str("Pets I can help with - what's going on with them?"), py_str("Tell me about the animal and what's worrying you.")])), (py_str("garden-and-plants"), py_tuple(vec![py_str("The garden I can help with - what are you growing?"), py_str("Tell me what the plants are doing and I'll help.")])), (py_str("sleep"), py_tuple(vec![py_str("Sleep I can help with - what's happening at night?"), py_str("Tell me what your nights are like and we'll find the next step.")])), (py_str("fitness"), py_tuple(vec![py_str("Fitness I can help with - what are you training for?"), py_str("Tell me what you're doing and where it's sticking.")])), (py_str("school-and-study"), py_tuple(vec![py_str("Study I can help with - what are you working on?"), py_str("Tell me the subject and what's hard about it.")])), (py_str("shopping"), py_tuple(vec![py_str("Shopping I can help with - what are you choosing between?"), py_str("Tell me what you're after and I'll help you narrow it down.")])), (py_str("entertainment"), py_tuple(vec![py_str("Films and TV I can help with - what are you in the mood for?"), py_str("Tell me what you've liked lately and I'll suggest something.")])), (py_str("sport"), py_tuple(vec![py_str("Sport I can help with - what are you watching or playing?"), py_str("Tell me the match or the game and I'll help.")])), (py_str("language-and-writing"), py_tuple(vec![py_str("Words I can help with - what are you writing?"), py_str("Tell me what you want to say and I'll help you phrase it.")])), (py_str("feelings"), py_tuple(vec![py_str("I can sit with the feelings - tell me what's going on."), py_str("Tell me what's been happening and how it's sitting with you.")])), (py_str("maths-and-numbers"), py_tuple(vec![py_str("Numbers I can do - what are you working out?"), py_str("Tell me the sum or the figure and I'll work it through.")]))]));
    s_HELP_TOPIC_ANY(py_tuple(vec![py_str("Tell me what you're working on and I'll help where I can.")]));
    s_HELP_TAIL(call_builtin("frozenset", &[mcall(&py_str("with me us for about at around to it here there then now anyway though please thanks honestly really so and"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OPINION_ASKS(py_tuple(vec![py_str("do you like"), py_str("do you love"), py_str("do you prefer"), py_str("do you reckon"), py_str("you prefer"), py_str("your favourite"), py_str("your favorite"), py_str("have a favourite"), py_str("have a favorite"), py_str("have you got a favourite"), py_str("have you got a favorite"), py_str("which do you"), py_str("would you rather"), py_str("in your opinion"), py_str("your opinion"), py_str("your take"), py_str("what do you think about"), py_str("what do you make of"), py_str("your thoughts on"), py_str("cats or dogs"), py_str("dogs or cats"), py_str("best season"), py_str("favourite season"), py_str("favorite season"), py_str("watch much tv"), py_str("watch a lot of tv"), py_str("watch much telly")]));
    s_TASTES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("cats or dogs"), py_str("cat or dog"), py_str("dogs or cats"), py_str("dog or cat")]), py_tuple(vec![py_str("Dogs by a whisker: company on a walk beats almost anything. Cats are the easier flatmate."), py_str("Dogs for me, though I would not turn down a cat that ignored me politely.")])]), py_tuple(vec![py_tuple(vec![py_str("tea or coffee"), py_str("coffee or tea")]), py_tuple(vec![py_str("Coffee in the morning, tea after four: two different jobs."), py_str("Tea, if I had to choose one. Coffee is a tool, tea is a pause.")])]), py_tuple(vec![py_tuple(vec![py_str("best season"), py_str("favourite season"), py_str("favorite season"), py_str("best time of year")]), py_tuple(vec![py_str("Early autumn: warm enough to sit outside, dark early enough to stay in, and no pressure to be doing something."), py_str("Late spring. Everything is green and nobody has started complaining about the heat yet.")])]), py_tuple(vec![py_tuple(vec![py_str("morning or evening"), py_str("morning or the evening"), py_str("evenings or mornings")]), py_tuple(vec![py_str("Morning, by a mile. The day is still yours then.")])]), py_tuple(vec![py_tuple(vec![py_str("recline")]), py_tuple(vec![py_str("Recline it, but only on a long flight, and check behind you first. A short daytime hop, no.")])]), py_tuple(vec![py_tuple(vec![py_str("pineapple")]), py_tuple(vec![py_str("On pizza, yes. Fruit and cheese is not the crime people make it out to be.")])]), py_tuple(vec![py_tuple(vec![py_str("toilet roll"), py_str("toilet paper")]), py_tuple(vec![py_str("Over. It hangs better and you can see how much is left.")])]), py_tuple(vec![py_tuple(vec![py_str("book or film"), py_str("film or book"), py_str("book or the film")]), py_tuple(vec![py_str("The book first, always; then the film can only be a bonus.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite book"), py_str("favorite book"), py_str("best book")]), py_tuple(vec![py_str("A collected volume of a newspaper column I read in the bath: short pieces are easy to start and easy to put down.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite film"), py_str("favorite film"), py_str("best film"), py_str("favourite movie"), py_str("favorite movie")]), py_tuple(vec![py_str("Anything where people mostly talk in kitchens. A plot is optional; good company is not.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite food"), py_str("favorite food"), py_str("best food")]), py_tuple(vec![py_str("Roast potatoes, and I am not going to pretend otherwise.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite thing to cook"), py_str("favorite thing to cook"), py_str("favourite thing to make"), py_str("favorite thing to make"), py_str("favourite recipe"), py_str("favorite recipe"), py_str("best thing to cook"), py_str("favourite thing to eat"), py_str("favorite thing to eat")]), py_tuple(vec![py_str("One pan and no recipe: a traybake, a frittata, whatever is in the fridge going soft. Less washing up, more eating."), py_str("Sausages in a tin with onions and apples, on a cold day. Nothing clever, and it works every time.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite thing to read"), py_str("favorite thing to read"), py_str("favourite author"), py_str("favorite author"), py_str("favourite writer"), py_str("favorite writer")]), py_tuple(vec![py_str("Short things I can pick up and put down: a newspaper column, an essay, a book of letters."), py_str("Old advice books - cookery, gardening, how to fix a bike. They assume nothing and they are usually right.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite colour"), py_str("favorite color"), py_str("favourite color"), py_str("favorite colour")]), py_tuple(vec![py_str("Green, the sort that looks grey in a cold light.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite city"), py_str("favorite city"), py_str("best city")]), py_tuple(vec![py_str("Whichever one I can walk across in an afternoon.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite animal"), py_str("favorite animal"), py_str("best animal")]), py_tuple(vec![py_str("Herons. They stand in the rain looking unbothered, which I admire.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite show"), py_str("favorite show"), py_str("favourite series"), py_str("favorite series"), py_str("favourite tv show"), py_str("favorite tv show"), py_str("favourite tv programme"), py_str("favorite tv program"), py_str("favourite programme"), py_str("favorite program"), py_str("favourite thing to watch"), py_str("favorite thing to watch"), py_str("watch much tv"), py_str("watch a lot of tv"), py_str("watch much television")]), py_tuple(vec![py_str("Something slow and British where not much happens for a while, and then everything does: *Detectorists* for the first, *Happy Valley* for the second."), py_str("The same few comedies on repeat. *Peep Show* is still the one I would put on now.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite actor"), py_str("favorite actor"), py_str("favourite actress"), py_str("favorite actress")]), py_tuple(vec![py_str("Olivia Colman. She does more with one look than most people manage with a speech."), py_str("Anyone who can say a whole paragraph with their face. Olivia Colman, if you make me pick.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite song"), py_str("favorite song"), py_str("favourite album"), py_str("favorite album"), py_str("favourite band"), py_str("favorite band"), py_str("favourite music"), py_str("favorite music"), py_str("favourite singer"), py_str("favorite singer"), py_str("what music do you like")]), py_tuple(vec![py_str("*Rumours*, still. It is the one album where every song is the good one."), py_str("Anything with a bit of swing to it. Loud is fine, clever is optional.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite drink"), py_str("favorite drink")]), py_tuple(vec![py_str("Tea, and I would rather not count how many. Coffee is a tool, tea is a pause.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite day"), py_str("favorite day"), py_str("best day of the week")]), py_tuple(vec![py_str("Saturday morning, before anyone needs anything. The whole weekend is still ahead.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite time of day"), py_str("favorite time of day"), py_str("best time of day")]), py_tuple(vec![py_str("Early morning, just after the kettle. Nobody wants anything from me yet.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite biscuit"), py_str("favorite biscuit"), py_str("favourite cookie"), py_str("favorite cookie")]), py_tuple(vec![py_str("A digestive, and I will not be talked out of it."), py_str("Shortbread. Plain, buttery, gone in two bites.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite thing to talk about"), py_str("favorite thing to talk about"), py_str("favourite thing to chat about"), py_str("favorite thing to chat about")]), py_tuple(vec![py_str("Whatever the person has just brought me. The best part of a conversation is finding out what somebody actually cares about."), py_str("Small practical problems. There is nearly always something useful to say about a kettle that will not boil.")])]), py_tuple(vec![py_tuple(vec![py_str("favourite thing about your job"), py_str("favorite thing about your job"), py_str("like about your job"), py_str("like about the job"), py_str("like most about your job"), py_str("best thing about your job"), py_str("best thing about the job"), py_str("best bit about your job"), py_str("best bit of your job"), py_str("good thing about your job"), py_str("love about your job"), py_str("do you like your job"), py_str("do you like what you do"), py_str("do you enjoy your job")]), py_tuple(vec![py_str("The moment a question turns out to have a good answer after all. That is most of the job."), py_str("People telling me things they have not said to anyone else yet. I try to be worth it.")])])]));
    s_FAVOURITE_GENERAL(py_tuple(vec![py_str("No proper favourite, if I am honest. What is yours?"), py_str("I have never managed to pick just one. What would you put first?"), py_str("I do not have one I would defend. Tell me yours and I might borrow it.")]));
    s_FAVOURITE_WORDS(py_tuple(vec![py_str("favourite"), py_str("favorite")]));
    s_TASTE_GENERAL(py_tuple(vec![py_str("No strong view on that one, honestly. What's yours?"), py_str("I'm easy on it — it's not a hill I'd argue from. What do you make of it?"), py_str("I don't mind either way. Tell me which you'd pick and I'll argue the other side for fun.")]));
    s_PICK_ONLY_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    a an the and but so or if to of at on in with about between for from\n    what whats which who whom where when why how would will shall should could can do does did\n    you u ya your you're ur i me my we us our it its that this these those there\n    be been being is are was were\n    pick picking choose choosing choice get getting buying buy take taking go going\n    suggest suggestion suggestions recommend recommendation recommendations prefer rather\n    like want looking look think reckon say said\n    one ones thing things anything something nothing stuff else instead again too also then\n    now next first just ok okay well honestly really actually any some got please\n    best good better more\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MY_PICK_NOTE(py_tuple(vec![py_str("I'd go *{name}* — {note} is more my thing."), py_str("Honestly, *{name}*: {note} is what I'd go for."), py_str("I'd take *{name}*, out of those — {note} is the draw for me.")]));
    s_MY_PICK(py_tuple(vec![py_str("I'd go *{name}*, out of those."), py_str("Honestly, *{name}*. Any of them would do, but that's my pick."), py_str("I'd take *{name}* myself.")]));
    s_MY_PICK_ASK(py_tuple(vec![py_str("Depends what we're choosing between — what's on the list?"), py_str("Tell me the options and I'll say which I'd take.")]));
    s_CARE_MODULES(call_builtin("frozenset", &[py_tuple(vec![py_str("safety"), py_str("urgent"), py_str("loss"), py_str("grief")])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUIET_MODULES(call_builtin("frozenset", &[py_tuple(vec![g_FALLBACK_MODULE(), py_str("chat")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRADE_OFF_LEAN(py_tuple(vec![py_str("I would take {side}."), py_str("My pick is {side}."), py_str("Take {side}."), py_str("I would go with {side}.")]));
    s_CHOICE_LIGHT(py_tuple(vec![py_str("Go with whichever you'd be happier with in ten minutes — and don't spend long on it, both are fine."), py_str("I'd take your first thought there. You already know which way you're leaning."), py_str("Either works, so take the one that's less fuss.")]));
    s_CHOICE_LIGHT_PICK(py_tuple(vec![py_str("{pick}. If you were hoping I'd say \"{other}\", then you've decided already."), py_str("{pick}. I wouldn't spend another minute on it — both work."), py_str("{pick} — that's the one I'd take, and either is fine."), py_str("{pick}. I'd take that and not think about it again.")]));
    s_CHOICE_THERAPY(py_set(vec![py_str("therapy"), py_str("therapist"), py_str("counselling"), py_str("counseling")]));
    s_CHOICE_HEALTH(py_set(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("codeine"), py_str("naproxen"), py_str("co-codamol"), py_str("meds"), py_str("med"), py_str("medicine"), py_str("medicines"), py_str("medication"), py_str("medications"), py_str("tablets"), py_str("tablet"), py_str("painkillers"), py_str("painkiller"), py_str("antibiotics"), py_str("antibiotic"), py_str("antidepressants"), py_str("statins"), py_str("prescription"), py_str("dose"), py_str("dosage"), py_str("therapy"), py_str("therapist"), py_str("counselling"), py_str("counseling")]));
    s_CHOICE_PEOPLE(call_builtin("frozenset", &[mcall(&py_str("\n    mum mom mam mummy mother dad daddy father parents parent sister sisters brother brothers\n    son daughter kids children child gran granny nan nana grandma grandad grandpa aunt aunty\n    uncle cousin cousins nephew niece family stepmum stepdad inlaws boyfriend\n    girlfriend partner husband wife fiance fiancee ex flatmate housemate\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHOICE_BETWEEN_PEOPLE(py_tuple(vec![py_str("Can't pick between people for me and you - you know them and I don't. Who would you be sorrier to miss?"), py_str("That's yours to call: no wrong answer, and I'm not going to rank people for you. Who would you rather be with?"), py_str("Not one I'd choose for you, sorry. Whose reaction are you dreading more?")]));
    s_CHOICE_PHARMACIST(py_tuple(vec![py_str("Worth asking a pharmacist — it's free, and they'll say which one suits what's wrong."), py_str("Ask a pharmacist rather than me: they can weigh it against anything else you take."), py_str("That one is a pharmacist's or GP's call — say what it's for and they'll point you at the right one.")]));
    s_CHOICE_WEIGH(py_tuple(vec![py_str("Whichever you'd regret less if it went wrong. Which way are you leaning?"), py_str("I'd pick the one that's easier to undo. Is one of them?"), py_str("I'd go with the one you'd miss more if it went away — that's the tell. Which is it?")]));
    s_CHOICE_STAKES(call_builtin("frozenset", &[mcall(&py_str("\n    buy buying bought cost costs repairing costlier price prices priced money afford cheap cheaper\n    expensive spend spending repair repairs repaired fix fixing fixed replace replacing\n    replaced warranty guarantee hire hiring rent rental deposit book booking booked cancel\n    cancelling commit commitment contract sign signing quit quitting resign move moving\n    removals install installing installed fit fitting fitter wire wiring socket fuse boiler\n    washer bolt pipe seal gasket screw screws tighten drill paint painting plaster tile\n    tiling build building roof gutter tyre tyres brake brakes mot service insurance insure\n    loan borrow mortgage overdraft decide decision spend sell selling sold tax bill debt\n    job jobs offer offers career school course degree uni university college house flat\n    holiday holidays wedding baby child\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_TYPES(py_tuple(vec![py_str("place"), py_str("name"), py_str("title"), py_str("person")]));
    s_WHY_AFTER_TASTE(py_tuple(vec![py_str("No deep reason: it's the one I'd go back to without thinking about it. What's yours?"), py_str("Mostly the memories attached to it, and a bit of habit. What would you say?"), py_str("Because it never disappoints, and that's most of what I ask of anything.")]));
    s_LIVE_NEWS_REPLY(py_str("I can't see the news from here, sorry. A news site or app will have the latest; tell me the story and I'll help you make sense of it."));
    s_LIVE_KINDS(py_tuple(vec![py_tuple(vec![py_str("site-stats"), py_tuple(vec![py_str("traffic"), py_str("footfall"), py_str("foot traffic"), py_str("visitors"), py_str("hits"), py_str("analytics"), py_str("site stats"), py_str("stats for my")]), py_tuple(vec![py_str("No bot can see your own numbers, sorry. A site's analytics and a search engine's count never agree exactly, because each counts a visitor its own way, so pick one and stay with it: the analytics on your site, or the admin page for a shop or a till, holds the figures. The trend over a month says more than any single day does."), py_str("Your own visitors are counted twice over, and the two counts differ: the analytics on your site counts pages opened, a search engine counts clicks, so the figures only ever line up roughly. Either one, watched over a month, tells you what happened; a single day does not."), py_str("One busy day is usually one link somewhere, so the figure worth having is the trend rather than today's. That trend lives in the analytics for your site, or in the admin page for a shop or a till, and no one outside your own account can see it.")])]), py_tuple(vec![py_str("site-stats-down"), py_tuple(vec![py_str("traffic"), py_str("footfall"), py_str("foot traffic"), py_str("visitors"), py_str("hits"), py_str("sales"), py_str("orders")]), py_tuple(vec![py_str("Quiet traffic is usually one row in the stats: a listing that stopped being refreshed, a link that died, or the week after a busy one. The analytics for your shop shows which pages people came in on, and the one that fell away is the one to refresh."), py_str("A drop is rarely the whole shop. It is normally the thing you last changed, or a listing that got stale while the rest of them were being looked after: the admin page tells you which page lost the visits, and that page is the one worth an hour."), py_str("The usual explanation is that a busy week pulls the next one down, so a dip after a good week is not a fall. If it is not that, the admin page shows the page that lost visits, and a listing relisted or refreshed often picks up again within a few days.")])]), py_tuple(vec![py_str("weather"), py_tuple(vec![py_str("weather"), py_str("wether"), py_str("weathr"), py_str("forecast"), py_str("forcast"), py_str("raining"), py_str("rain tomorrow"), py_str("snowing"), py_str("how hot"), py_str("how cold"), py_str("temperature outside"), py_str("temperature in"), py_str("temperature tomorrow"), py_str("temperature today"), py_str("is it cold"), py_str("is it warm"), py_str("is it hot"), py_str("is it freezing"), py_str("is it raining"), py_str("is it snowing"), py_str("is it sunny"), py_str("cold outside"), py_str("warm outside"), py_str("hot outside"), py_str("how warm"), py_str("cold enough"), py_str("warm enough"), py_str("snowing outside"), py_str("raining outside"), py_str("like outside"), py_str("like it outside"), py_str("whats it like out"), py_str("what it like out"), py_str("the temperature"), py_str("whats the temperature"), py_str("what temperature is it"), py_str("how warm is it"), py_str("how cold is it"), py_str("how hot is it")]), py_tuple(vec![py_str("I can't check the forecast from here, sorry - no live weather in me. A weather app has today's for your postcode, hour by hour, and for a trip later in the year the monthly averages are the useful thing."), py_str("No live weather from me, sorry. A forecast app or the national weather site has what is actually coming where you are, and the monthly averages for a trip months away.")])]), py_tuple(vec![py_str("hours"), py_tuple(vec![py_str("opening hours"), py_str("opening times"), py_str("opening time"), py_str("open today"), py_str("open now"), py_str("close today"), py_str("closing time"), py_str("what time does"), py_str("what time do"), py_str("is it open"), py_str("shut today"), py_str("open on")]), py_tuple(vec![py_str("The opening hours are the first thing a venue changes and the last thing it tells anyone: a refurb, a bank holiday or new owners will do it. Their own page or map listing has the current ones."), py_str("No opening hours in front of me. Whoever runs the place - a council tip, a chain, one landlord - announces a change itself, and its own page or phone number has today's.")])]), py_tuple(vec![py_str("prices"), py_tuple(vec![py_str("how much is a"), py_str("how much does a"), py_str("how much are"), py_str("price of"), py_str("cost of"), py_str("exchange rate"), py_str("stock price"), py_str("how much is it")]), py_tuple(vec![py_str("I can't look up live prices, sorry. What a figure like that turns on is which one it is, where you buy it and the day, so two or three sellers compared all-in will beat the first price you see - their own site has today's number, and bring it back and I'll work out what it comes to a month."), py_str("Prices move too fast for me to know. Which one you pick and where you buy it decide most of the figure, and a quick look at the seller's own site has today's number, all in with delivery - tell me what it says and I'll do the sums with you.")])]), py_tuple(vec![py_str("news"), py_tuple(vec![py_str("the news"), py_str("headline"), py_str("who won"), py_str("the score"), py_str("latest on"), py_str("current events"), py_str("what happened in the")]), py_tuple(vec![g_LIVE_NEWS_REPLY()])]), py_tuple(vec![py_str("roads"), py_tuple(vec![py_str("traffic"), py_str("the roads"), py_str("the road"), py_str("road closed"), py_str("closed road"), py_str("roadworks"), py_str("road works"), py_str("the motorway"), py_str("motorway closed"), py_str("in a jam"), py_str("jams"), py_str("accident on the"), py_str("accidents on the"), py_str("accident on"), py_str("accidents on"), py_str("diversion"), py_str("dual carriageway")]), py_tuple(vec![py_str("Traffic comes back at the same times on most weekdays, so a road that is slow at eight is usually slow at eight, while a closure or roadworks is a diversion and a different wait altogether. I can't see the roads from here, sorry: a maps app with live traffic, or the roads agency's own site, has today's jams, closures and roadworks as they happen."), py_str("A jam clears from the front and a closure means a diversion, and those want different places looked at. I can't see the roads from here, sorry: the roads agency's own site has today's closures and roadworks, and a maps app with live traffic has the jams and how long each one is adding.")])]), py_tuple(vec![py_str("travel"), py_tuple(vec![py_str("train times"), py_str("bus times"), py_str("next train"), py_str("next bus"), py_str("delayed"), py_str("is it running"), py_str("on strike"), py_str("buses running"), py_str("are the buses"), py_str("the trains running"), py_str("are the trains"), py_str("timetable"), py_str("the tube"), py_str("tube strike")]), py_tuple(vec![py_str("I can't check live times or delays, sorry. The operator's own app or site has the next departure and any disruption, and a journey planner puts the whole trip together.")])]), py_tuple(vec![py_str("journey"), py_tuple(vec![py_str("how long is the drive"), py_str("how long would the drive"), py_str("how long does the drive"), py_str("how long is the journey"), py_str("how long would the journey"), py_str("how long is the trip"), py_str("how long does it take to drive"), py_str("how long does it take to get to"), py_str("how long to drive"), py_str("drive from"), py_str("drive to"), py_str("the drive be"), py_str("how far is the drive"), py_str("how far from"), py_str("how far is"), py_str("how many miles")]), py_tuple(vec![py_str("The time a drive takes is mostly decided by the hour you set off and by the roads you use, since town streets and the last few miles are never motorway speed. I can't see either from here: a route planner has the figure for the hour you would actually leave, roadworks included, and the stops worth making on the way."), py_str("However far it is, the drive will take longer than the distance suggests, because the first and last miles are the slow part and the hour you leave changes the rest. A route planner has it for your own leaving time; a maps app does the same from wherever you are.")])]), py_tuple(vec![py_str("journey-ask"), py_tuple(vec![py_str("how long is the drive"), py_str("how long would the drive"), py_str("how long does the drive"), py_str("how long will the drive"), py_str("how long is the journey"), py_str("how long would the journey"), py_str("how long will the journey"), py_str("how long is the trip"), py_str("how long would the trip"), py_str("how long will the trip"), py_str("how long is the flight")]), py_tuple(vec![py_str("The hour you set off and the roads you use decide most of a drive, and the first and last miles are the slow part wherever you are going. Long from where to where, and roughly when would you set off? I can plan the day around it once I know."), py_str("The hour and the last few miles decide a journey, and either end can be the slow one. Where are you starting, where are you heading, and roughly when? I can work through the timing and the stops with you.")])]), py_tuple(vec![py_str("distance"), py_tuple(vec![py_str("how far is"), py_str("how far away"), py_str("how many miles is"), py_str("how many miles from"), py_str("how long is the walk"), py_str("how long is the bike ride")]), py_tuple(vec![py_str("I can't measure a distance from where you are, sorry. A map app does it from your own door in one tap, walking and driving both."), py_str("That is a distance I can't take from this end. Put it into a map app and it shows how far and how long, from wherever you are standing.")])]), py_tuple(vec![py_str("waste"), py_tuple(vec![py_str("bin day"), py_str("bin collection"), py_str("bin collections"), py_str("collection day"), py_str("recycling day"), py_str("is it recycling"), py_str("when do they collect"), py_str("when do the bins"), py_str("when are the bins"), py_str("when is the bin"), py_str("which bin goes out"), py_str("which bin is it"), py_str("black bin"), py_str("green bin"), py_str("blue bin"), py_str("brown bin"), py_str("food waste"), py_str("garden waste"), py_str("kerbside"), py_str("wheelie bin"), py_str("bulky waste"), py_str("household waste"), py_str("rubbish collection"), py_str("refuse collection"), py_str("missed bin"), py_str("bin lorry"), py_str("bin been collected"), py_str("bins been collected"), py_str("has the bin been")]), py_tuple(vec![py_str("Rounds are set street by street, not by town, and a bank holiday moves them by a day or two - so I can't see your street's day from here. The council's own site takes a postcode and gives the next collection; that is also where the holiday change is announced."), py_str("Which day a street is collected on is the council's own choice, and holidays shift it, so there is nothing to know from this end without the postcode. The council's site has the round for your address, and a missed bin is free to report there on the day.")])])]));
    s_MY_OWN(py_dict(vec![]));
    s_MY_RECORDS(py_tuple(vec![py_tuple(vec![py_str("delivery"), py_tuple(vec![py_str("order"), py_str("orders"), py_str("parcel"), py_str("parcels"), py_str("delivery"), py_str("deliveries"), py_str("tracking"), py_str("dispatch"), py_str("courier")]), py_tuple(vec![py_str("The tracking link in the order email or the shop's app shows exactly where a parcel is, and the shop can look the order up by your name and address - I can't see your order from here."), py_str("Where a parcel has got to is in the tracking link you were sent; if it has not moved for days it is the shop you chase, not the courier. No way to check it from this end.")])]), py_tuple(vec![py_str("booking"), py_tuple(vec![py_str("booking"), py_str("bookings"), py_str("reservation"), py_str("reservations"), py_str("appointment"), py_str("appointments"), py_str("check-in"), py_str("check in")]), py_tuple(vec![py_str("The confirmation email or text has the time and the reference, and a ring to the place with your name and the day settles anything else - I can't see your booking from here."), py_str("A booking lives with the place, not with me: the email or text you were sent has the time, and the desk can check it by name if that has gone missing.")])]), py_tuple(vec![py_str("application"), py_tuple(vec![py_str("application"), py_str("applications"), py_str("claim"), py_str("claims"), py_str("renewal"), py_str("quote"), py_str("complaint"), py_str("refund"), py_str("refunds")]), py_tuple(vec![py_str("The reference number they gave you brings it up on their site, and one call to the office gets you a straight answer - I can't see how it is going from here."), py_str("The reference they gave you, or the email that came back when you sent it, has where it has got to, and a phone call gets it in a minute. Nothing of yours is in front of me.")])]), py_tuple(vec![py_str("money"), py_tuple(vec![py_str("balance"), py_str("statement"), py_str("account"), py_str("direct debit"), py_str("standing order"), py_str("payment"), py_str("transaction"), py_str("overdraft")]), py_tuple(vec![py_str("Your bank's app, a cash machine or the number on the card shows the balance and what has gone out - I can't see your account at all."), py_str("The figure is in your banking app or on the last statement, and the bank can confirm it in one call. Nothing of yours is here for me to look at.")])]), py_tuple(vec![py_str("card"), py_tuple(vec![py_str("gift card"), py_str("gift cards"), py_str("voucher"), py_str("vouchers"), py_str("loyalty points"), py_str("loyalty card"), py_str("store card")]), py_tuple(vec![py_str("The number on the back of the card, or the shop's app, tells you what is left on it - I can't check a card balance from here."), py_str("The card's own site, or the number on the back of it, has the balance; nothing of yours is here for me to look at.")])])]));
    s_CARD_VALIDITY(py_tuple(vec![py_str("A card's expiry date is printed on the back of it, and the shop's app shows it too. If it has run out, ring the shop - many will swap it or move what is left onto a new one."), py_str("The date it runs out is on the back of the card, or in the shop's app. An out-of-date card is worth one phone call: shops can often swap it or put the balance back as credit.")]));
    s_HEALTH_RECORD(py_tuple(vec![py_str("prescription"), py_str("prescriptions"), py_str("scan"), py_str("scans"), py_str("referral"), py_str("referrals"), py_str("x-ray"), py_str("xray"), py_str("x-rays"), py_str("bloods"), py_str("blood test"), py_str("test results"), py_str("results")]));
    s_HEALTH_WORLD(py_tuple(vec![py_str("doctor"), py_str("gp"), py_str("surgery"), py_str("hospital"), py_str("clinic"), py_str("nurse"), py_str("midwife"), py_str("consultant"), py_str("dentist"), py_str("medical"), py_str("health"), py_str("test"), py_str("blood"), py_str("scan"), py_str("x-ray"), py_str("xray"), py_str("prescription"), py_str("consultant"), py_str("specialist"), py_str("appointment")]));
    s_OFFICE_RECORD_LINES(py_tuple(vec![py_str("I can't see that from here - nothing of yours reaches me, so I can't say where it has got to. The letter, email or reference number it came with will bring up its state on their site, and one call to the office that issued it settles it."), py_str("Nothing of yours here to look at, sorry, so I can't check that one. Whatever it came from - the letter, the email or the app it was arranged through - has where it has got to, and the number they gave you gets it in a minute on the phone."), py_str("That state is theirs to know and mine to guess at, and a guess is no use to you. The confirmation you were sent is the one place it is written down, and the office that deals with it can confirm it in one call.")]));
    s_AT_THE_TILL(py_tuple(vec![py_str("Count it against the receipt before you leave it too long: the till's own record of the sale is what the shop works from, and a counter sorts it on the day far more easily than a head office does later. How much short are you?"), py_str("That is the shop's to put right, not yours to prove - the receipt and the amount is usually enough at the counter, and most shops fix it the same day. If it was a card, your statement has the figure the till rang up.")]));
    s_MY_OWN_DAY(py_tuple(vec![py_str("I can't see your rota from here, sorry - nothing of your work reaches me, so your hours are with whoever writes it and in the app your work puts it in."), py_str("No window on your shifts from this end, sorry: the start and finish times are on the rota, with your manager or in the rota app your work uses."), py_str("That one lives on the rota rather than anywhere I can see - your manager has the hours, and your contract has the ones they have to keep to.")]));
    s_MY_BILL_LINES(py_tuple(vec![py_str("What you pay is on your own bill rather than in any price list I could read: the app or the email they send you has the monthly figure, and the bank statement under the same name shows it went out. Tell me what it is and I'll work out what it comes to over the year, and check the notice you would need to give."), py_str("I can't see your bill, sorry. The provider's app, the last email from them or your bank statement has the figure, and the same place has the minimum term - that is what decides whether you can leave and what it costs to stop. Give me the numbers and I'll do the sums."), py_str("That price is between you and them, and only your own paperwork has it. The app, the bill email or the direct debit on your statement is where to look, and the contract length and notice period sit on the same page - worth knowing before you think about changing it. Hand me the figures and I'll work the rest out.")]));
    s_POINTS_LINES(py_tuple(vec![py_str("Points build up wherever the card or the app gets scanned, and the scheme's own points page has the balance and the day they run out - most clear an account after a year or two with nothing on it. Tell me what a point is worth in that scheme and I'll work out the money."), py_str("I can't see your points from here, sorry - only the scheme can. Their app, a receipt or the number on the back of the card shows the balance, and it is worth a look if the card has been in a drawer, because points usually expire after a year or so with no activity. Give me the figures and I'll do the arithmetic on what they buy."), py_str("That balance lives with the scheme rather than with me. The app's points page has it, and the same page says when they expire - a year or two of no use is the usual rule, though each scheme sets its own. Hand me the rate and the count and I'll tell you what it comes to.")]));
    s_HEALTH_RECORD_LINES(py_tuple(vec![py_str("I can't see your records from here. The surgery's app or a ring to reception has the date and the result, and they can say whether anyone needs to see you about it."), py_str("Nothing of yours in front of me, sorry. The letter, text or app it came through will have the date and what came of it, and reception can look it up if that is lost.")]));
    s_OWN_PLACE(py_tuple(vec![py_str("shop"), py_str("store"), py_str("stores"), py_str("branch"), py_str("office"), py_str("cafe"), py_str("café"), py_str("canteen"), py_str("pub"), py_str("bar"), py_str("gym"), py_str("surgery"), py_str("pharmacy"), py_str("chemist"), py_str("bank"), py_str("salon"), py_str("museum"), py_str("tip"), py_str("library"), py_str("restaurant"), py_str("takeaway"), py_str("chippy"), py_str("garage"), py_str("hotel"), py_str("school"), py_str("nursery"), py_str("clinic"), py_str("centre"), py_str("center"), py_str("pool"), py_str("market"), py_str("stall"), py_str("van"), py_str("kiosk"), py_str("studio"), py_str("workshop")]));
    s_OWN_PERSON(py_tuple(vec![py_str("son"), py_str("daughter"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("baby"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("boyfriend"), py_str("girlfriend"), py_str("nan"), py_str("grandma"), py_str("grandad"), py_str("grandson"), py_str("granddaughter")]));
    s_LIVE_NOT(py_tuple(vec![py_str("difference"), py_str("climate"), py_str("how does"), py_str("why does"), py_str("why do "), py_str("explain"), py_str("meaning of")]));
    s_LIVE_NOT_HOURS(py_tuple(vec![py_str("sun"), py_str("sunrise"), py_str("sunset"), py_str("sun set"), py_str("sun rise"), py_str("moon"), py_str("moonrise"), py_str("tide"), py_str("tides"), py_str("high tide"), py_str("low tide"), py_str("dawn"), py_str("dusk"), py_str("daylight"), py_str("stars"), py_str("milky way"), py_str("eclipse")]));
    s_LIVE_VENUE(py_tuple(vec![py_str("shop"), py_str("store"), py_str("supermarket"), py_str("tesco"), py_str("asda"), py_str("sainsbury"), py_str("aldi"), py_str("lidl"), py_str("co-op"), py_str("coop"), py_str("morrisons"), py_str("waitrose"), py_str("iceland"), py_str("branch"), py_str("bank"), py_str("post office"), py_str("postbox"), py_str("library"), py_str("pharmacy"), py_str("chemist"), py_str("surgery"), py_str("gp"), py_str("doctor"), py_str("dentist"), py_str("vet"), py_str("tip"), py_str("dump"), py_str("recycling centre"), py_str("cafe"), py_str("café"), py_str("restaurant"), py_str("takeaway"), py_str("chippy"), py_str("pub"), py_str("bar"), py_str("gym"), py_str("leisure centre"), py_str("pool"), py_str("museum"), py_str("gallery"), py_str("garden centre"), py_str("centre"), py_str("market"), py_str("salon"), py_str("barber"), py_str("hairdresser"), py_str("office"), py_str("clinic"), py_str("hospital"), py_str("school"), py_str("nursery"), py_str("garage"), py_str("hotel"), py_str("station"), py_str("airport"), py_str("council"), py_str("launderette"), py_str("kiosk"), py_str("studio"), py_str("workshop"), py_str("church"), py_str("hall"), py_str("venue"), py_str("park"), py_str("butcher"), py_str("bakery"), py_str("greengrocer"), py_str("depot"), py_str("club")]));
    s_LIVE_VENUE_RE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z])(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LIVE_VENUE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")(?:s|es)?(?![a-z])"))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_OPEN_CLOSE(py_tuple(vec![py_str("open"), py_str("opens"), py_str("opening"), py_str("opened"), py_str("close"), py_str("closes"), py_str("closing"), py_str("closed"), py_str("shut"), py_str("shuts"), py_str("shutting"), py_str("close up")]));
    s_LIVE_HOURS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:when|what time|what hours|how late|what times)\\s+(?:does|do|is|are|will|would|can|doesnt|don't|dont|isnt|is it)\\b[^.!?]{0,40}?\\b(?:open|opens|closed|closes|close|shut|shuts|shutting|closing|re-?opens?|re-?opening|re-?opened)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_YES_NO_OPEN(stdlib_call("re", "compile", &[py_str("^\\s*(?:so|and|but|ok|okay|also|hi|hello|morning|quick one)?\\s*,?\\s*(?:is|are|will|does|do|can)\\s+[^.!?]{0,40}?\\b(?:open|opens|shut|shuts|closed|closing|close)\\b(?!\\s*(?:up|ness|ing|ly|air|plan|letter|account|evening|day|days|morning|afternoon)\\b)(?!\\s*to\\s+(?!the\\s+public\\b))")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_INDOOR(py_tuple(vec![py_str("oven"), py_str("fridge"), py_str("freezer"), py_str("room"), py_str("bedroom"), py_str("nursery"), py_str("bath"), py_str("shower"), py_str("kettle"), py_str("boiler"), py_str("radiator"), py_str("washing machine"), py_str("tumble dryer"), py_str("air fryer"), py_str("slow cooker"), py_str("grill"), py_str("hob"), py_str("pan"), py_str("oil"), py_str("water"), py_str("iron"), py_str("greenhouse"), py_str("shed"), py_str("garage"), py_str("car"), py_str("engine"), py_str("laptop"), py_str("phone"), py_str("kitchen"), py_str("loft"), py_str("attic"), py_str("cellar"), py_str("basement"), py_str("conservatory"), py_str("living room"), py_str("sitting room"), py_str("dining room"), py_str("hall"), py_str("hallway"), py_str("porch")]));
    s_LIVE_WEATHER_WORDS(py_tuple(vec![py_str("weather"), py_str("wether"), py_str("weathr"), py_str("forecast"), py_str("forcast"), py_str("rain"), py_str("raining"), py_str("rainy"), py_str("snow"), py_str("snowing"), py_str("sunny"), py_str("sun"), py_str("wind"), py_str("windy"), py_str("outside"), py_str("outdoors"), py_str("umbrella"), py_str("coat"), py_str("bbq"), py_str("barbecue")]));
    s_LIVE_SKY(py_tuple(vec![py_str("windy"), py_str("icy"), py_str("muggy"), py_str("humid"), py_str("cloudy"), py_str("overcast"), py_str("chilly"), py_str("freezing"), py_str("heatwave"), py_str("rain"), py_str("raining"), py_str("rainy"), py_str("snow"), py_str("snowing"), py_str("snowy"), py_str("storm"), py_str("stormy"), py_str("thunder"), py_str("thundery"), py_str("hail"), py_str("hailing"), py_str("fog"), py_str("foggy"), py_str("mist"), py_str("misty"), py_str("drizzle"), py_str("drizzling"), py_str("showers"), py_str("frost"), py_str("frosty")]));
    s_LIVE_SKY_OFF(py_tuple(vec![py_str("brain fog"), py_str("foggy head"), py_str("head is foggy"), py_str("head foggy"), py_str("misty eyed"), py_str("hail a taxi"), py_str("hail a cab"), py_str("weather the storm"), py_str("storm in a teacup"), py_str("under the weather"), py_str("frost free"), py_str("frost-free"), py_str("rain check")]));
    s_LIVE_SKY_WHY(stdlib_call("re", "compile", &[py_str("\\b(?:why|how\\s+come|what\\s+causes|what\\s+makes)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_WEAR(py_tuple(vec![py_str("umbrella"), py_str("coat"), py_str("jacket"), py_str("shorts"), py_str("boots"), py_str("wellies")]));
    s_LIVE_SETTING(py_tuple(vec![py_str("gym"), py_str("work"), py_str("office"), py_str("school"), py_str("college"), py_str("uni"), py_str("class"), py_str("nursery"), py_str("dinner"), py_str("lunch"), py_str("party"), py_str("wedding"), py_str("funeral"), py_str("interview"), py_str("hospital"), py_str("indoors"), py_str("inside"), py_str("in the house"), py_str("shopping"), py_str("the shops"), py_str("the dance")]));
    s_LIVE_SOON(py_tuple(vec![py_str("today"), py_str("tonight"), py_str("tomorrow"), py_str("this afternoon"), py_str("this morning"), py_str("this evening"), py_str("this weekend")]));
    s_LIVE_TAKE_ASK(py_tuple(vec![py_str("should i take"), py_str("shall i take"), py_str("do i take"), py_str("do i need"), py_str("will i need"), py_str("should i bring"), py_str("shall i bring"), py_str("do i need to take"), py_str("should i wear"), py_str("shall i wear"), py_str("should i put on"), py_str("will i be needing"), py_str("do i want")]));
    s_LIVE_TAKE_ALLOWED(py_set(vec![py_str("to"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("your"), py_str("and"), py_str("or"), py_str("out"), py_str("outside"), py_str("tonight"), py_str("today"), py_str("tomorrow"), py_str("now"), py_str("later"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("weekend"), py_str("with"), py_str("me"), py_str("it"), py_str("this"), py_str("on"), py_str("in"), py_str("at"), py_str("for"), py_str("when"), py_str("i"), py_str("should"), py_str("shall"), py_str("do"), py_str("need"), py_str("will"), py_str("be"), py_str("bring"), py_str("take"), py_str("wear"), py_str("put"), py_str("want"), py_str("going"), py_str("am")]));
    s_LIVE_JOURNEY_ENDS(py_tuple(vec![py_str("from"), py_str("to"), py_str("towards"), py_str("as far as")]));
    s_LIVE_TRAVEL_WORDS(py_tuple(vec![py_str("train"), py_str("trains"), py_str("bus"), py_str("buses"), py_str("coach"), py_str("flight"), py_str("flights"), py_str("plane"), py_str("tram"), py_str("tube"), py_str("ferry"), py_str("motorway"), py_str("traffic"), py_str("road"), py_str("line"), py_str("strike")]));
    s_LIVE_ROAD_NAME(stdlib_call("re", "compile", &[py_str("\\b(?:m|a)\\d{1,3}(?:\\s?\\(m\\))?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_PAPER_WORDS(py_tuple(vec![py_str("paper"), py_str("card"), py_str("cardboard"), py_str("envelope"), py_str("printer"), py_str("print"), py_str("printing"), py_str("printed"), py_str("poster"), py_str("frame"), py_str("photo"), py_str("photograph"), py_str("notebook"), py_str("canvas"), py_str("sketchbook"), py_str("drawing"), py_str("stationery"), py_str("folder"), py_str("binder")]));
    s_LIVE_ROAD_WORDS(py_tuple(vec![py_str("road"), py_str("roads"), py_str("motorway"), py_str("traffic"), py_str("junction"), py_str("bypass"), py_str("roundabout"), py_str("carriageway"), py_str("lane")]));
    s_LIVE_SITE_THING(py_tuple(vec![py_str("site"), py_str("website"), py_str("web page"), py_str("webpage"), py_str("online shop"), py_str("shop"), py_str("store"), py_str("etsy"), py_str("listing"), py_str("my page"), py_str("the page"), py_str("web stats"), py_str("footfall")]));
    s_LIVE_ROADS_NOT(py_tuple(vec![py_str("road tax"), py_str("tax the"), py_str("road trip"), py_str("road bike"), py_str("off road"), py_str("web traffic"), py_str("site traffic"), py_str("online traffic"), py_str("search traffic"), py_str("footfall"), py_str("foot traffic"), py_str("my website"), py_str("my site"), py_str("my shop"), py_str("my store"), py_str("my etsy"), py_str("website"), py_str("shop window"), py_str("customers through"), py_str("dead"), py_str("fox"), py_str("badger"), py_str("hedgehog"), py_str("deer"), py_str("animal"), py_str("pothole"), py_str("who do i call"), py_str("who do i report"), py_str("report a"), py_str("report the"), py_str("tips"), py_str("advice"), py_str("how do i drive"), py_str("driving in"), py_str("long drive"), py_str("driving test"), py_str("theory test"), py_str("driving licence"), py_str("driving license"), py_str("how do i get"), py_str("how to get"), py_str("how far is"), py_str("directions to"), py_str("route to"), py_str("made of"), py_str("built"), py_str("why do"), py_str("why are roads"), py_str("surface"), py_str("roadworthy"), py_str("cycle lane"), py_str("cycle path"), py_str("bus lane"), py_str("parking"), py_str("speeding"), py_str("points on"), py_str("school"), py_str("college"), py_str("class"), py_str("revision"), py_str("study")]));
    s_LIVE_DELAY_WORDS(py_tuple(vec![py_str("cancelled"), py_str("canceled"), py_str("cancellation"), py_str("delayed"), py_str("delay"), py_str("delays"), py_str("on time"), py_str("running late"), py_str("disruption"), py_str("engineering works"), py_str("replacement bus"), py_str("bus replacement")]));
    s_LIVE_STRONG(py_tuple(vec![py_str("times"), py_str("forecast"), py_str("opening hours"), py_str("opening times"), py_str("how much"), py_str("price"), py_str("what time"), py_str("the score"), py_str("latest on"), py_str("latest news"), py_str("news today"), py_str("any news today"), py_str("any headlines"), py_str("any news on"), py_str("any news about"), py_str("any news of"), py_str("any update on"), py_str("any word on"), py_str("spill the news"), py_str("any results"), py_str("any result"), py_str("what channel"), py_str("any traffic"), py_str("traffic problems"), py_str("road closed"), py_str("closed road"), py_str("train timetable"), py_str("bus timetable"), py_str("are the buses"), py_str("are the trains"), py_str("buses running"), py_str("trains running"), py_str("tube line"), py_str("the m25 closed"), py_str("m25 closed")]));
    s_HOW_ARE_YOU_ASKS(py_tuple(vec![py_str("how are you"), py_str("how are things"), py_str("how is things"), py_str("how's things"), py_str("how's your day"), py_str("how is your day"), py_str("your day going"), py_str("howz your day"), py_str("whats your day"), py_str("hows your day"), py_str("how's it going"), py_str("how is it going"), py_str("how have you been"), py_str("you doing"), py_str("you ok"), py_str("you okay"), py_str("you all right"), py_str("how do you do"), py_str("you up to"), py_str("up to much"), py_str("been up to")]));
    s_HOW_ARE_YOU_GOING_ON(py_tuple(vec![py_str("much going on"), py_str("going on")]));
    s_HOW_ARE_YOU_GOING_ON_AFTER(call_builtin("frozenset", &[py_set(vec![py_str("else"), py_str("")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_ARE_YOU_TAGS(call_builtin("frozenset", &[py_tuple(vec![py_str("today"), py_str("tonight"), py_str("now"), py_str("then"), py_str("so"), py_str("far"), py_str("lately"), py_str("recently"), py_str("again"), py_str("too"), py_str("mate"), py_str("guys"), py_str("everyone"), py_str("everybody"), py_str("all"), py_str("both"), py_str("honestly"), py_str("anyway"), py_str("though"), py_str("thanks"), py_str("sir"), py_str("madam"), py_str("doing"), py_str("feeling"), py_str("keeping"), py_str("coping"), py_str("holding"), py_str("up"), py_str("these"), py_str("days"), py_str("at"), py_str("the"), py_str("moment"), py_str("folks"), py_str("friend"), py_str("love"), py_str("team"), py_str("been"), py_str("like"), py_str("going"), py_str("lately"), py_str("been"), py_str("going"), py_str("looking"), py_str("hmm"), py_str("lol"), py_str("go"), py_str("gone")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_ARE_YOU(py_tuple(vec![py_str("Good, thanks for asking — and good to have you here. What's on your mind?"), py_str("Can't complain. Better question: how are you doing?"), py_str("Fine and busy, the two usual things. How's your day been?"), py_str("I'm well, thanks. What are you up to today?")]));
    s_ADVICE_WORDS(py_tuple(vec![py_str("should"), py_str("worth"), py_str("better"), py_str("cheaper"), py_str("faster"), py_str("recommend"), py_str("suggest"), py_str("which"), py_str("how do"), py_str("how can"), py_str("any tips"), py_str("less"), py_str("fewer"), py_str("more"), py_str("difference"), py_str("correct"), py_str("grammar"), py_str("spell"), py_str("'")]));
    s_REQUEST_CUES(py_tuple(vec![py_str("quick one"), py_str("quick question"), py_str("one more thing"), py_str("another thing"), py_str("need help"), py_str("any tips"), py_str("any advice"), py_str("advice"), py_str("how do"), py_str("how can"), py_str("what should"), py_str("can you"), py_str("could you"), py_str("help me"), py_str("give me"), py_str("recommend"), py_str("suggest"), py_str("wondering"), py_str("?")]));
    s_JUDGE_ASKS_SLIGHT(py_str("(?:stupid|daft|dumb|silly|idiotic|thick|bad|terrible|awful|dreadful|wrong)"));
    s_JUDGE_ASKS_FINE(py_str("(?:good|great|sensible|wise|smart|clever|reasonable)"));
    s_JUDGE_ASKS_MAD(py_str("(?:mad|crazy|insane|nuts|mental|unhinged)"));
    s_JUDGE_NOUN(py_str("(?:idea|plan|thing|move|call|choice|decision|approach|bet|notion|step)"));
    s_JUDGE_BARE_FACES(call_builtin("frozenset", &[mcall(&py_str("sensible wise reasonable smart clever stupid daft dumb silly\n    idiotic bad terrible awful dreadful wrong mad crazy insane nuts mental unhinged"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_LEAD(py_str("(?:^|\\b(?:ok|okay|so|and|but|well|honestly|tbh|maybe|i think|i reckon|i guess|i feel like)\\b )"));
    s_JUDGE_PLAN_STOP(call_builtin("frozenset", &[mcall(&py_str("but because so then since if unless although though which who that\n    when where while whereas before after once"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_NOD_WORDS(call_builtin("frozenset", &[mcall(&py_str("yes yeah yep no nope ok okay sure right cool thanks thank you\n    hmm mm mmm well so and but true fair good nice haha lol oh ah please"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_PLAN_HAS_A_HOLE(py_tuple(vec![py_str("{Plan} is where it gets tight, and that is the part to answer for. What would you do if it went wrong?"), py_str("{Plan} is the part a friend would push back on, not the wanting. Say what you would do if it did not work, and if that is quick to say, go."), py_str("{Plan} leaves nothing spare, and that is the part I would look at. What is your answer if it goes wrong?")]));
    s_JUDGE_WORTH(py_tuple(vec![py_str("Not stupid, no. {Plan} is the ordinary kind of thing people do; what decides it is what you would do if it went wrong - if that leaves you all right, go."), py_str("Nothing wrong with {plan}. The part that goes wrong is having no answer for what if it does not work."), py_str("Nothing daft in {plan}. Say what you would do if it went wrong; if that is quick to say, do it.")]));
    s_JUDGE_WORTH_PLAIN(py_tuple(vec![py_str("Not stupid. The test is what you would do if it went wrong: if that leaves you all right, there is nothing daft in it."), py_str("No, it is not stupid. The thing that would make it stupid is having no answer for what if it goes wrong."), py_str("Nothing daft about it. Ask what you would do if it went wrong - if you would be fine, go.")]));
    s_JUDGE_FINE(py_tuple(vec![py_str("Yes, it makes sense - {plan} is a reasonable thing to aim at. The part that bites is doing it with no answer for what if it goes wrong."), py_str("{Plan} is the ordinary thing to do; what bites is doing it with no plan for when it goes wrong."), py_str("Yes - {plan} is worth doing. The one thing to have ready is an answer for what if it goes wrong.")]));
    s_JUDGE_FINE_PLAIN(py_tuple(vec![py_str("Yes, it makes sense. Have a line ready for what you would do if it went wrong, and there is nothing left to worry about."), py_str("It sounds sensible to me. Say in one line what you would do if it went wrong - if that is easy to say, go.")]));
    s_JUDGE_SANE(py_tuple(vec![py_str("Not mad. {Plan} is the kind of thing plenty of people do, and the test is the one any plan gets: what would you do if it went wrong?"), py_str("No, you are not mad. {Plan} is worth weighing, and the thing to have ready is an answer for what if it goes wrong.")]));
    s_JUDGE_SANE_PLAIN(py_tuple(vec![py_str("Not mad at all. The test is the one any plan gets: what would you do if it went wrong? If that leaves you all right, go."), py_str("No, you are not mad. Wanting something different is normal - have an answer ready for what if it goes wrong and that is the whole of it.")]));
    s_JUDGE_NO_PLAN(py_tuple(vec![py_str("About what? Tell me the idea in a line and I will say what I think."), py_str("There is nothing for me to judge yet - say the idea in a line and I will tell you straight.")]));
    s_JUDGE_NO_PLAN_MAD(py_tuple(vec![py_str("Nothing you have said sounds mad to me. What is the thing you are weighing up?"), py_str("Mad about what? Tell me what you are thinking of doing and I will give you a straight answer.")]));
    s_JUDGE_NO_PLAN_MIND(py_tuple(vec![py_str("Nothing you have said sounds mad to me. And it is worth telling a GP, who hears this often and knows what to do with it."), py_str("That is a frightening thing to carry, and not one to sit on: a GP is the right first call, and you can describe it in a line."), py_str("I would not wait for that to get worse before saying it to somebody. A GP is the one to say it to - what happens, and how often - and let them say what it is.")]));
    s_JUDGE_NO_PLAN_FINE(py_tuple(vec![py_str("Probably - but say what it is in a line and I will tell you straight."), py_str("Nothing you have said sounds unwise to me. What is the thing you are weighing up?")]));
    s_JUDGE_WISH_ANGRY(py_tuple(vec![py_str("No, I would not do that. Something has pushed you a long way - what happened?"), py_str("That is the anger doing the thinking. Not that one. What has actually gone on?"), py_str("I would not. Whatever they did, that lands on you and not on them. Tell me what happened.")]));
    s_JUDGE_ONLY_A_PLAN(call_builtin("frozenset", &[py_tuple(vec![py_str("stupid"), py_str("daft"), py_str("dumb"), py_str("silly"), py_str("idiotic"), py_str("thick")])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_SOUNDS_FINE(call_builtin("frozenset", &[py_tuple(vec![py_str("wise"), py_str("sensible"), py_str("smart"), py_str("clever"), py_str("reasonable")])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_SELF_JUDGING(call_builtin("frozenset", &[py_tuple(vec![py_str("mad"), py_str("crazy"), py_str("insane"), py_str("nuts"), py_str("mental"), py_str("unhinged")])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_NOT_A_VERDICT(call_builtin("frozenset", &[py_tuple(vec![py_str("bad"), py_str("terrible"), py_str("awful"), py_str("dreadful"), py_str("wrong")])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_NOT_A_GERUND(call_builtin("frozenset", &[py_tuple(vec![py_str("ring"), py_str("bring"), py_str("sing"), py_str("ping"), py_str("cling"), py_str("sting"), py_str("swing"), py_str("wring"), py_str("spring"), py_str("string"), py_str("thing"), py_str("kings"), py_str("wing"), py_str("ding")])]).unwrap_or_else(|e| exc::raise(e)));
    s_YOURS(py_dict(vec![(py_str("my"), py_str("your")), (py_str("me"), py_str("you")), (py_str("myself"), py_str("yourself")), (py_str("mine"), py_str("yours"))]));
    s_A_PLACE_I_DONT_KNOW(py_tuple(vec![py_str("I've not been there, so I can't vouch for it - what are you after: the sights, the food, or somewhere quiet?"), py_str("I don't know it well enough to promise you anything. What would make the trip good for you?"), py_str("No first-hand knowledge of the place, sorry. Are you going for the sights, the food or the peace?"), py_str("Honestly I can't tell you what it's like there. What matters most to you about the trip?")]));
    s_WORD_FIXES(py_dict(vec![(py_str("what"), py_str("what")), (py_str("whats"), py_str("what")), (py_str("wats"), py_str("what")), (py_str("wat"), py_str("what")), (py_str("wot"), py_str("what")), (py_str("hows"), py_str("how")), (py_str("whos"), py_str("who")), (py_str("wheres"), py_str("where")), (py_str("whys"), py_str("why")), (py_str("whens"), py_str("when")), (py_str("cn"), py_str("can")), (py_str("cud"), py_str("could"))]));
    s_QUESTION_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("which"), py_str("can"), py_str("could"), py_str("should"), py_str("would"), py_str("will"), py_str("do"), py_str("does"), py_str("did"), py_str("is"), py_str("are"), py_str("have"), py_str("has")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARD_LINES(py_tuple(vec![py_str("Two lines is plenty: say the thing you would say out loud if they were in front of you."), py_str("Name them, say the one specific thing you are glad about, and sign it — short and warm beats long and clever."), py_str("Write it the way you would speak it, then read it back once and take out the careful bits.")]));
    s_TELL_HARD_LINES(py_tuple(vec![py_str("Say the hard sentence first and keep it short - they will hear the rest better after it. Tell me how you want to put it and I'll help you word it."), py_str("Pick a calm moment, say the one thing that matters, then stop and let them answer. Give me the gist and I'll draft it with you."), py_str("Plain words, early, and no long build-up: that is kinder than a soft run at it. Tell me the gist and I'll help you shape it.")]));
    s_WORDING_LINES(py_tuple(vec![py_str("Say the one thing that matters first, then the rest. Give me the gist and I'll draft it."), py_str("Keep it to two sentences: what happened, then what you want. Send me the gist and I'll put it in words."), py_str("Two or three lines is plenty: what happened, what you want, and by when. Give me the gist and I'll draft it."), py_str("Lead with what you want and why, then stop. Tell me the gist and I'll write the first version for you."), py_str("Write it the way you would speak it, then take out the careful bits. Give me the gist and I'll do a pass over it.")]));
    s_GREETING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("hi"), py_str("hello"), py_str("hey"), py_str("hiya"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("greetings"), py_str("yo"), py_str("howdy")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_NOTED(py_tuple(vec![py_str("Remind me of the name and I'll keep it from here."), py_str("Say the name again and I'll remember it from here on."), py_str("I haven't got it in front of me — tell me the name and I'll hold on to it.")]));
    s_NOT_A_NAME(call_builtin("frozenset", &[py_tuple(vec![py_str("yesterday"), py_str("today"), py_str("tomorrow"), py_str("tonight"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("night"), py_str("week"), py_str("month"), py_str("year"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("it"), py_str("him"), py_str("her"), py_str("them"), py_str("that"), py_str("this"), py_str("one"), py_str("again"), py_str("back"), py_str("up"), py_str("out"), py_str("off"), py_str("in"), py_str("on"), py_str("the"), py_str("a"), py_str("an"), py_str("and"), py_str("but"), py_str("so"), py_str("then"), py_str("now"), py_str("already"), py_str("twice")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_ONLY(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("whats"), py_str("did"), py_str("do"), py_str("does"), py_str("i"), py_str("we"), py_str("my"), py_str("our"), py_str("the"), py_str("a"), py_str("an"), py_str("is"), py_str("was"), py_str("call"), py_str("called"), py_str("name"), py_str("named"), py_str("again"), py_str("and"), py_str("so"), py_str("it"), py_str("him"), py_str("her"), py_str("them"), py_str("s"), py_str("say"), py_str("said"), py_str("tell"), py_str("told"), py_str("you"), py_str("remember"), py_str("recall"), py_str("about"), py_str("to"), py_str("of")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HANDED_BACK(py_tuple(vec![py_str("You called it {name} — I've kept that."), py_str("{name} — that's the one you gave me, and it's still here."), py_str("It's {name}, isn't it? I've got that down.")]));
    s_POINTING(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("about"), py_str("again"), py_str("all"), py_str("also"), py_str("am"), py_str("an"), py_str("and"), py_str("any"), py_str("anything"), py_str("are"), py_str("as"), py_str("at"), py_str("be"), py_str("but"), py_str("by"), py_str("can"), py_str("could"), py_str("did"), py_str("do"), py_str("does"), py_str("for"), py_str("from"), py_str("get"), py_str("got"), py_str("had"), py_str("has"), py_str("have"), py_str("how"), py_str("i"), py_str("if"), py_str("in"), py_str("into"), py_str("is"), py_str("it"), py_str("its"), py_str("just"), py_str("like"), py_str("me"), py_str("more"), py_str("my"), py_str("no"), py_str("not"), py_str("of"), py_str("on"), py_str("one"), py_str("or"), py_str("our"), py_str("out"), py_str("should"), py_str("so"), py_str("some"), py_str("still"), py_str("than"), py_str("that"), py_str("the"), py_str("their"), py_str("them"), py_str("then"), py_str("there"), py_str("these"), py_str("they"), py_str("thing"), py_str("things"), py_str("this"), py_str("those"), py_str("to"), py_str("too"), py_str("up"), py_str("us"), py_str("was"), py_str("we"), py_str("were"), py_str("what"), py_str("when"), py_str("which"), py_str("who"), py_str("why"), py_str("will"), py_str("with"), py_str("would"), py_str("you"), py_str("your"), py_str("year"), py_str("years"), py_str("date")])]).unwrap_or_else(|e| exc::raise(e)));
    s_RULED_OUT_DETERMINER(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("its"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("any"), py_str("some"), py_str("another"), py_str("every"), py_str("each"), py_str("no")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTICLE(call_builtin("frozenset", &[py_tuple(vec![py_str("out"), py_str("up"), py_str("off"), py_str("down"), py_str("through"), py_str("back"), py_str("away"), py_str("round"), py_str("over"), py_str("about"), py_str("along")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_OWNER(py_dict(vec![(py_str("my"), py_str("your")), (py_str("our"), py_str("your")), (py_str("i"), py_str("you")), (py_str("im"), py_str("you're")), (py_str("i'm"), py_str("you're")), (py_str("me"), py_str("you")), (py_str("mine"), py_str("yours")), (py_str("we"), py_str("you")), (py_str("us"), py_str("you"))]));
    s_TIPS_CLAUSE(call_builtin("frozenset", &[mcall(&py_str("\n    i i'm im i've ive we we're weve he she they wont will would going\n    gonna wanna want wants wanted need needs needed have has had is are was were am do does did\n    dont don't didnt didn't cant can't cannot keep keeps kept eat eats eet said say says told get\n    got make makes made feel feels felt think thinks thought try tried\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_VERB(call_builtin("frozenset", &[mcall(&py_str("\n    wake sleep stop start learn lose save cook run quit find use take put fix write buy sell move\n    change switch pick choose sort become study give tell show send list suggest recommend get\n    need want try help make do\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_AHEAD(py_tuple(vec![py_str("Start with the one thing that usually goes wrong, not the whole list. How long have you got?"), py_str("What's worrying you most about it? With the time you have, that decides what's worth doing."), py_str("Pick the part you would most hate to get wrong and practise that bit. When is it?")]));
    s_TIPS_THING(py_tuple(vec![py_str("What have you got so far, and what isn't working? That tells me which advice is worth having."), py_str("What's the thing you most want to get right? That's the part worth spending advice on."), py_str("What usually goes wrong? Start there, not with the whole list.")]));
    s_TIPS_LINES(py_tuple(vec![py_str("Happy to help — tell me what you've got so far and I'll add what I can."), py_str("Sure. What have you tried so far? That tells me where to start.")]));
    s_HELP_ALREADY(py_tuple(vec![py_str("That's most of what I know on it — say which part is still unclear and I'll come at it another way."), py_str("That's the lot I have on that one. Tell me which bit isn't landing and I'll try a different angle.")]));
    s_SAME_AGAIN(py_tuple(vec![py_str("That is all of what I know on it - say if you want any part of it again, or slower."), py_str("That was the whole of my answer on it. Tell me which part to take further and I will."), py_str("That is everything I have on that one, so I would only repeat myself. Which bit do you want me to go into?")]));
    s_SAME_AGAIN_WITH_THE_ANSWER(py_tuple(vec![py_str("Same answer, and it covers this: {answer}"), py_str("That is still the answer: {answer}"), py_str("The answer above holds: {answer}")]));
    s_VIEW_ONLY_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    a an the and but so or if to of at on in with about between for from as\n    what whats is was your my you u ur ya i me we us our it its that this there here\n    be is are do does did would could should will can\n    reckon think say said view opinion take takes thoughts verdict read call answer answers\n    honest honestly own real true personal actually really truly\n    ok okay well then now just too also again please thanks thank\n    give tell me full whole thing things one on it that this\n    d r wat\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VIEW_LINES(py_tuple(vec![py_str("Honest answer: {said}"), py_str("Same as I said: {said}"), py_str("My view has not changed: {said}"), py_str("Honestly, that is still where I would land: {said}")]));
    s_VIEW_ASK_LINES(py_tuple(vec![py_str("What I would still want to know: {said}"), py_str("Same question from me: {said}"), py_str("I would still ask you the same thing: {said}"), py_str("That question is still the one I would put to you: {said}")]));
    s_VIEW_DECIDE(py_tuple(vec![py_str("Same as before: that one is yours to call. What it turns on is what you gain against what you would give up — say both and I will help you weigh them."), py_str("It stays your call rather than mine. Tell me the two things you are weighing and I will lay them side by side.")]));
    s_VIEW_HANDBACK(py_tuple(vec![py_str("not mine to decide"), py_str("not a call i can make"), py_str("can't call that one"), py_str("cant call that one"), py_str("that's your call"), py_str("thats your call")]));
    s_NOT_AN_ANSWER(op_add(&op_add(&op_add(&op_add(&g_NEUTRAL(), &g_NEWS_TO_COME()), &g_POSITIVE()), &g_RELIEF()), &g_CONGRATS()));
    s__ECHO_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an the i im i'm ive i've id i'd me my we us our you your it its it's that thats this these\nthose is are was were am be been being do does did done have has had of to in on at for with\nand but so then than just really very quite too also about from by as or if not no yes yeah ok\nokay well right got get gets here there what whats how why when who said say says think\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRAISE(py_tuple(vec![py_str("congratulations"), py_str("congrats"), py_str("well done"), py_str("nice one"), py_str("brilliant"), py_str("lovely"), py_str("great news"), py_str("good stuff"), py_str("glad"), py_str("pleased"), py_str("fantastic"), py_str("amazing"), py_str("excellent"), py_str("love that"), py_str("that's great"), py_str("thats great"), py_str("weight off"), py_str("good for you"), py_str("enjoy it")]));
    s_VIEW_ON_GOOD_NEWS(py_tuple(vec![py_str("Nothing to weigh up there - that is just good news. Enjoy it: what is the first thing you will do about it?"), py_str("Honest answer: take the win. There is no decision in it, and I would not spoil it hunting for one. What happens next?")]));
    s_HEDGE_POOL(py_tuple(vec![g_UNKNOWN(), g_UNKNOWN_DECIDE(), g_UNKNOWN_MONEY(), g_UNKNOWN_HEALTH(), g_UNKNOWN_WORD(), g_UNKNOWN_WORRY(), crate::compose::finding_out::g_PLACE_POOL(), crate::compose::finding_out::g_TRADE_POOL(), crate::compose::finding_out::g_PROFESSIONAL_POOL(), crate::compose::finding_out::g_PICK_POOL(), crate::compose::finding_out::g_WHO_POOL(), crate::compose::finding_out::g_HOW_MANY_POOL(), crate::compose::finding_out::g_BUY_POOL(), crate::compose::finding_out::g_FIGURE_POOL(), crate::compose::finding_out::g_WRECKED_POOL(), crate::compose::finding_out::g_PLACE_ROUTE_POOL()]));
    s_OUTCOME_LINES(py_tuple(vec![py_str("Nobody can say for certain yet, and anyone who does is guessing. Ask the staff looking after them what they are watching for over the next day or two, and what would be a good sign."), py_str("There is no honest yes or no from here - the ward is the only place with a real answer. Ask the nurse in charge what has to improve before the next step, and ask again tomorrow."), py_str("No one can tell you that yet. The people treating them can say what they are watching for and what a good day would look like, and that is the question worth asking today.")]));
    s_VIEW_MIRROR_LINES(py_tuple(vec![py_str("Nothing has changed for me: {said}"), py_str("I would say the same again: {said}"), py_str("That still stands: {said}")]));
    s_VIEW_COMPANY(py_tuple(vec![py_str("Nothing to add to that, and no view of mine worth putting on top of it. What I said still stands, and I am here if you want to say more."), py_str("I have not got a view to give you on top of that - only the same thing again, which is that it matters. Tell me how you are doing with it."), py_str("Nothing I could add would beat what I already said. If you want to say more about it, I am listening.")]));
    s_VIEW_NO_SUBJECT(py_tuple(vec![py_str("On what? Give me the thing you are weighing up and I will tell you straight."), py_str("Happy to - what's it about? Name the choice you are stuck on and I'll give you my honest take."), py_str("What is it about? Name the thing you are stuck on and I will give you my honest take.")]));
    s_VIEW_AFTER_A_DEATH(py_tuple(vec![py_str("If it is my view you want: nobody gets this right, and being all over the place is not doing it wrong."), py_str("Honestly? There is no right way to do it, and no timetable you have to be on."), py_str("My view is only that it is allowed to be this hard. You are not doing it wrong."), py_str("Nothing I could add would help more than what I said. Tell me about them, or don't - either is fine by me.")]));
    s_VIEW_LEAD_IN(call_builtin("frozenset", &[py_tuple(vec![py_str("thing"), py_str("things"), py_str("way"), py_str("ways"), py_str("point"), py_str("points"), py_str("answer"), py_str("answers"), py_str("plan"), py_str("rule"), py_str("rules"), py_str("step"), py_str("steps"), py_str("advice"), py_str("short"), py_str("version"), py_str("part"), py_str("parts"), py_str("bit"), py_str("bits"), py_str("key"), py_str("difference"), py_str("same"), py_str("rest"), py_str("other"), py_str("first"), py_str("next"), py_str("best")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_MORE_OF(call_builtin("frozenset", &[py_tuple(vec![py_str(""), py_str("greeting"), py_str("safety"), py_str("urgent"), py_str("grief"), py_str("loss")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIPS_UNIT(call_builtin("frozenset", &[py_tuple(vec![py_str("year"), py_str("years"), py_str("yr"), py_str("yrs"), py_str("month"), py_str("months"), py_str("week"), py_str("weeks"), py_str("day"), py_str("days"), py_str("hour"), py_str("hours"), py_str("min"), py_str("mins"), py_str("minute"), py_str("minutes"), py_str("stone"), py_str("st"), py_str("kg"), py_str("kgs"), py_str("lb"), py_str("lbs"), py_str("pound"), py_str("pounds"), py_str("mile"), py_str("miles"), py_str("km"), py_str("k"), py_str("quid"), py_str("grand"), py_str("per"), py_str("cent"), py_str("percent")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_SOMETHING_WRONG(op_bitor(&g_SICKNESS(), &call_builtin("frozenset", &[py_tuple(vec![py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("damp"), py_str("mould"), py_str("mold"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("noise"), py_str("broken"), py_str("broke"), py_str("trip"), py_str("tripped"), py_str("tripping"), py_str("fuse"), py_str("wiring"), py_str("wire"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful"), py_str("sore"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("rash"), py_str("bite"), py_str("bitten"), py_str("sting"), py_str("stung"), py_str("bump"), py_str("bleed"), py_str("bleeding"), py_str("swollen"), py_str("swelling"), py_str("crack"), py_str("cracks"), py_str("cracked"), py_str("blocked"), py_str("clogged"), py_str("wont"), py_str("won t"), py_str("not working"), py_str("stopped working"), py_str("chest"), py_str("throat"), py_str("stomach"), py_str("tummy"), py_str("belly"), py_str("back"), py_str("head"), py_str("leg"), py_str("legs"), py_str("arm"), py_str("arms"), py_str("hand"), py_str("hands"), py_str("foot"), py_str("feet"), py_str("knee"), py_str("knees"), py_str("shoulder"), py_str("neck"), py_str("hip"), py_str("wrist"), py_str("ankle"), py_str("toe"), py_str("toes"), py_str("finger"), py_str("fingers"), py_str("eye"), py_str("eyes"), py_str("ear"), py_str("ears"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("mouth"), py_str("skin"), py_str("joint"), py_str("joints"), py_str("ribs"), py_str("breast")])]).unwrap_or_else(|e| exc::raise(e))));
    s_QUOTE_LINES(py_tuple(vec![py_str("Get one more quote for the same job in writing, and ask what is included: the parts list is what makes two prices comparable."), py_str("Ask for a written quote split into parts and labour, then ring one other firm for the same job. That pair of numbers tells you more than any rule of thumb.")]));
    s_GIFT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("spend on"), py_str("budget for"), py_str("how much do you spend"), py_str("what do you get")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BUYING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("buying"), py_str("buy a house"), py_str("buy a flat"), py_str("buy a home"), py_str("bought"), py_str("purchase"), py_str("ladder"), py_str("mortgage"), py_str("own a home"), py_str("should i buy")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NETWORK_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("wifi"), py_str("wi-fi"), py_str("internet"), py_str("broadband"), py_str("network"), py_str("connection"), py_str("signal"), py_str("online")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEDS_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("tablets"), py_str("pills"), py_str("medicine"), py_str("medication"), py_str("prescription"), py_str("dose"), py_str("doses"), py_str("capsules"), py_str("antibiotics"), py_str("vaccine"), py_str("vaccination"), py_str("jab"), py_str("jabs"), py_str("vitamin"), py_str("vitamins"), py_str("supplement"), py_str("supplements"), py_str("ibuprofen"), py_str("paracetamol"), py_str("aspirin"), py_str("painkiller"), py_str("painkillers"), py_str("antihistamine"), py_str("antihistamines"), py_str("steroid"), py_str("steroids"), py_str("insulin"), py_str("inhaler")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ENDINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("s"), py_str("es"), py_str("ed"), py_str("d"), py_str("ing"), py_str("e"), py_str("y"), py_str("ion"), py_str("al"), py_str("ness"), py_str("able"), py_str("ly")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FORM(call_builtin("frozenset", &[py_tuple(vec![py_str("bloody"), py_str("rasher"), py_str("rashers"), py_str("teether"), py_str("waker"), py_str("stationary"), py_str("teething")])]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENING_QUESTIONS(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("which")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_ITSELF_KINDS(py_tuple(vec![py_str("model"), py_str("limits"), py_str("online"), py_str("agency"), py_str("identity"), py_str("purpose"), py_str("workings"), py_str("privacy"), py_str("trust"), py_str("judging"), py_str("secret"), py_str("asking")]));
    s_MODEL_NAMED_WORDS(Py::Int(7));
    s_MY_WORKINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("learn"), py_str("think"), py_str("wrong")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PICKS_A_MECHANISM(call_builtin("frozenset", &[mcall(&py_str("\ndecide decides deciding decided decision decisions choose chooses choosing chosen choice choices\npick picks picking select selects sort sorts answer answers answering answered\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WORRY_FILLER(call_builtin("frozenset", &[mcall(&py_str("\n    i im i'm am so really very pretty quite a bit little lot kind kinda of still just and ok\n    okay that thats it its now too much all this\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WORRY_WORDS(call_builtin("frozenset", &[mcall(&py_str("worried worrying anxious nervous"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THEIR_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("with"), py_str("for"), py_str("about"), py_str("of"), py_str("to"), py_str("at"), py_str("in"), py_str("on"), py_str("from"), py_str("by"), py_str("like"), py_str("without"), py_str("over"), py_str("under"), py_str("into"), py_str("the"), py_str("no"), py_str("any"), py_str("some"), py_str("his"), py_str("her"), py_str("their"), py_str("your")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("i am"), py_str("me"), py_str("am"), py_str("you"), py_str("we"), py_str("us")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FORGETTING(call_builtin("frozenset", &[py_tuple(vec![py_str("forget"), py_str("forgets"), py_str("forgot"), py_str("forgetting")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EATING(call_builtin("frozenset", &[py_tuple(vec![py_str("eat"), py_str("eats"), py_str("eating"), py_str("ate"), py_str("eaten")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EATEN_KIND(call_builtin("frozenset", &[py_tuple(vec![py_str("meat"), py_str("fish"), py_str("dairy"), py_str("gluten"), py_str("nuts"), py_str("eggs"), py_str("cheese"), py_str("pork"), py_str("beef"), py_str("chicken"), py_str("lamb"), py_str("bacon"), py_str("ham"), py_str("shellfish"), py_str("seafood"), py_str("sugar"), py_str("carbs"), py_str("bread"), py_str("rice"), py_str("pasta"), py_str("chips"), py_str("vegetables"), py_str("veg"), py_str("veggies"), py_str("fruit"), py_str("salad"), py_str("spuds"), py_str("potatoes"), py_str("vegan"), py_str("vegetarian"), py_str("halal"), py_str("kosher")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OURS(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our"), py_str("me"), py_str("us")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_TIME_WORDS(call_builtin("frozenset", &[mcall(&py_str("lovely great brilliant wonderful good nice amazing fun fantastic\n    smashing crackin delicious gorgeous tasty superb class memorable"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_OPENERS(call_builtin("frozenset", &[mcall(&py_str("was were is are am did does do have has had how what when where\n    why who whose would will can could should shall may might"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_VERDICT_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("\nward hospital hospice clinic surgery casualty a&e icu itu nurses nurse staff doctors doctor\nconsultant midwife surgeon matron theatre anaesthetic anaesthetics wake funeral burial cremation\ncrematorium memorial chapel servicio tests results scan chemo chemotherapy dialysis\nservice services eulogy eulogies graveside flowers mass readings reading sermon hymn hymns vigil\nrosary cemetery ashes casket hearse obituary tributes tribute condolences send-off sendoff\nparamedics ambulance paramedic carers carer counsellor counsellors physio physiotherapist vet vets\npolice firemen firefighters\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DREAD(py_tuple(vec![py_str("Fair enough - what's the part you're dreading?"), py_str("That's understandable. What's coming up that you're not looking forward to?"), py_str("I don't blame you. What is it about it?")]));
    s_A_WANT_IT_OVER(py_tuple(vec![py_str("Sounds like it's been a long one - what's been the worst of it?"), py_str("Fair enough. What's making it drag?"), py_str("That sort of week does that to people. What do you most want behind you?")]));
    s_A_DAY_AHEAD(py_tuple(vec![py_str("Something to look forward to, then - what's the plan?"), py_str("Nice. How are the plans going?"), py_str("Good. Is there much to sort out first?")]));
    s_A_DAY_AHEAD_TENSE(py_tuple(vec![py_str("How are you feeling about it?"), py_str("How ready do you feel for it?"), py_str("Is there much to get ready before it?")]));
    s_A_PAUSE_WORD(call_builtin("frozenset", &[mcall(&py_str("\n    a an the and or but so then than that this these those it its im i i've ive id i'd me my\n    mine we us our you your yours he she they them his her their there here now just only really\n    very quite bit little all of in on at to for from with about as if not no yes oh ok okay\n    sorry apologies apologie again back still also too already been was were is are am be being\n    because cos cause cus since seeing\n    do did done does have has had yet soon later earlier yesterday minutes min mins sec secs\n    second seconds moment moments hour hours while ago a while day days week weeks month months\n    year years morning afternoon evening night long ages spell tomorrow late lately\n    phone mobile phones laptop computer inbox messages message emails email texts\n    i've we've you've they've ive weve youve theyve\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOW_THE_TIME_WENT(call_builtin("frozenset", &[mcall(&py_str("\n    mad crazy manic hectic mental busy rushed rush flat slammed swamped snowed frantic chaotic\n    stressful stressed rough tough hard awful rubbish terrible horrible grim rotten bad long big\n    heavy weird odd slow uphill nights shifts deadlines overtime full non-stop endless relentless\n    nutty silly wild madder worse better fine ok\n    work life things everything stuff week weeks month months year years day days time times\n    while spell age ages morning night nights weekend weekdays lately recently overnight\n    behind on ahead late over under through away out\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MANNERS_ONLY(call_builtin("frozenset", &[mcall(&py_str("\n    sorry apologies apology apologise apologize bother bothering bothered ask asking asked\n    question questions one more thing another pain nuisance hassle trouble troubling troubled\n    pester pestering nag nagging\n    keep keeping going on essay ramble rambling dump dumping moan moans moaning moaned\n    interrupt interrupting\n    time i im me you your my it this that just again so really very bit lot a an the and but also\n    to for of about at with be being am is was had have been can could would please thanks thank\n    ok okay well honestly tbh quick quickly stupid silly daft late slow early knowing know\n    go goes going do did does\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THANK_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("thanks"), py_str("thank"), py_str("thankyou"), py_str("cheers"), py_str("ta"), py_str("appreciate"), py_str("appreciated")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THANKS_ONLY(op_bitor(&g_MANNERS_ONLY(), &call_builtin("frozenset", &[mcall(&py_str("\n    thats that's good know great brilliant brill perfect lovely helpful useful sorted right nice\n    much many so much ace wonderful excellent amazing legend star lifesaver kind kindly\n    for the advice help\n    all everything then too will do\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_THANKS_THEN_SAID(py_tuple(vec![py_str("Glad to hear it — and thanks for coming back to say so."), py_str("Good. Thanks for letting me know."), py_str("Pleased for you. Thanks for telling me how it went."), py_str("That's good to hear. Thanks for saying so.")]));
    s_THREAD(py_str("thread"));
    s_CARE_THREAD(py_str("care-thread"));
    s_PLAIN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("about"), py_str("after"), py_str("again"), py_str("anything"), py_str("before"), py_str("being"), py_str("could"), py_str("doing"), py_str("every"), py_str("going"), py_str("happen"), py_str("having"), py_str("should"), py_str("something"), py_str("still"), py_str("their"), py_str("there"), py_str("these"), py_str("thing"), py_str("things"), py_str("think"), py_str("those"), py_str("through"), py_str("today"), py_str("tonight"), py_str("would"), py_str("your"), py_str("yours"), py_str("mine"), py_str("with"), py_str("what"), py_str("when"), py_str("where"), py_str("which"), py_str("while"), py_str("whole"), py_str("leave")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ILL_BEFORE(call_builtin("frozenset", &[py_tuple(vec![py_str("feel"), py_str("feels"), py_str("feeling"), py_str("felt"), py_str("being"), py_str("been"), py_str("so"), py_str("very"), py_str("really"), py_str("quite"), py_str("still"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("fell"), py_str("am"), py_str("im"), py_str("is"), py_str("was"), py_str("are"), py_str("looks"), py_str("look"), py_str("too"), py_str("seriously"), py_str("badly"), py_str("terribly"), py_str("always"), py_str("often"), py_str("chronically")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ILL_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("for"), py_str("again"), py_str("all"), py_str("and"), py_str("today"), py_str("tonight"), py_str("lately"), py_str("since"), py_str("this"), py_str("last"), py_str("week"), py_str("weeks"), py_str("day"), py_str("days"), py_str("month"), py_str("morning"), py_str("at"), py_str("in"), py_str("on"), py_str("with"), py_str("from"), py_str("health"), py_str("enough"), py_str("yesterday")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ILL_SAID_OF_A_BODY(call_builtin("frozenset", &[py_tuple(vec![py_str("am"), py_str("im"), py_str("is"), py_str("was"), py_str("are"), py_str("feel"), py_str("feels"), py_str("feeling"), py_str("felt"), py_str("been"), py_str("being"), py_str("looks"), py_str("look"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("fell"), py_str("seriously"), py_str("badly"), py_str("terribly"), py_str("chronically"), py_str("often")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ILL_WILL_VERBS(call_builtin("frozenset", &[py_tuple(vec![py_str("be"), py_str("have"), py_str("get"), py_str("let"), py_str("try"), py_str("call"), py_str("text"), py_str("ring"), py_str("phone"), py_str("see"), py_str("tell"), py_str("say"), py_str("give"), py_str("send"), py_str("check"), py_str("do"), py_str("go"), py_str("come"), py_str("take"), py_str("make"), py_str("bring"), py_str("put"), py_str("keep"), py_str("look"), py_str("ask"), py_str("email"), py_str("message"), py_str("book"), py_str("order"), py_str("pay"), py_str("pop"), py_str("head"), py_str("drive"), py_str("walk"), py_str("run"), py_str("start"), py_str("finish"), py_str("show"), py_str("wait"), py_str("find"), py_str("grab"), py_str("meet"), py_str("stay"), py_str("sit"), py_str("sort"), py_str("write"), py_str("add"), py_str("fix"), py_str("clean"), py_str("tidy"), py_str("wash"), py_str("buy"), py_str("sell"), py_str("need"), py_str("want"), py_str("drop"), py_str("pick"), py_str("leave"), py_str("turn"), py_str("sign"), py_str("carry"), py_str("chat"), py_str("speak"), py_str("read"), py_str("eat")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AIM_HEADS(call_builtin("frozenset", &[py_tuple(vec![py_str("want"), py_str("wants"), py_str("wanting"), py_str("trying"), py_str("try"), py_str("need"), py_str("needs"), py_str("hoping"), py_str("hope")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AIM_NEGATORS(call_builtin("frozenset", &[py_tuple(vec![py_str("not"), py_str("dont"), py_str("doesnt"), py_str("didnt"), py_str("never"), py_str("no"), py_str("wont"), py_str("cant")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AIM_FAILING(call_builtin("frozenset", &[py_tuple(vec![py_str("wont"), py_str("doesnt"), py_str("isnt"), py_str("nothing"), py_str("stuck"), py_str("nowhere"), py_str("stopped"), py_str("hopeless")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("happy"), py_str("excited"), py_str("amazing"), py_str("proud"), py_str("delighted"), py_str("thrilled"), py_str("brilliant")])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUNK_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("rubbish"), py_str("crap"), py_str("crappy"), py_str("naff"), py_str("meh"), py_str("bleh"), py_str("poo")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOOD_BEFORE(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("am"), py_str("feel"), py_str("feels"), py_str("feeling"), py_str("felt"), py_str("seems"), py_str("seemed"), py_str("be"), py_str("been"), py_str("being"), py_str("so"), py_str("very"), py_str("really"), py_str("quite"), py_str("pretty"), py_str("totally"), py_str("absolutely"), py_str("bit"), py_str("such"), py_str("that"), py_str("this"), py_str("fairly"), py_str("properly")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOOD_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("week"), py_str("weeks"), py_str("day"), py_str("days"), py_str("month"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("night"), py_str("weekend"), py_str("year"), py_str("time"), py_str("start"), py_str("shift"), py_str("term"), py_str("few"), py_str("couple")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MOOD_WITH_A_CATCH(py_dict(vec![(py_str("fired"), call_builtin("frozenset", &[py_tuple(vec![py_str("got"), py_str("get"), py_str("gets"), py_str("getting"), py_str("was"), py_str("were"), py_str("is"), py_str("are"), py_str("am"), py_str("been"), py_str("being"), py_str("have"), py_str("has"), py_str("had"), py_str("ve"), py_str("i've"), py_str("we've"), py_str("they've")])]).unwrap_or_else(|e| exc::raise(e)))]));
    s_COPULA(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("am"), py_str("seems"), py_str("seemed"), py_str("be"), py_str("been"), py_str("being")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THEM_BEFORE_COPULA(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("ive"), py_str("i've"), py_str("me"), py_str("we"), py_str("we're"), py_str("were"), py_str("us"), py_str("myself"), py_str("everything"), py_str("things"), py_str("life"), py_str("it"), py_str("this"), py_str("today")])]).unwrap_or_else(|e| exc::raise(e)));
    s_RUMINATION_REPLIES(py_tuple(vec![py_str("That sounds like it is still sitting with you. What is the part that keeps coming back?"), py_str("A worry at 2am is rarely the same shape as one in daylight. What actually happened?"), py_str("Going round it that many times is exhausting. Say it out to me — what keeps coming back?"), py_str("Fair enough — that kind of thing does not let go on its own. What is the bit you keep landing on?")]));
    s_BODY_STATES(py_dict(vec![(py_str("cold"), call_builtin("frozenset", &[py_tuple(vec![py_str("cold"), py_str("freezing"), py_str("frozen"), py_str("chilly"), py_str("chilled"), py_str("perished"), py_str("baltic"), py_str("icy"), py_str("shivering"), py_str("shivery")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("hot"), call_builtin("frozenset", &[py_tuple(vec![py_str("boiling"), py_str("hot"), py_str("roasting"), py_str("sweating"), py_str("sweaty"), py_str("sweltering"), py_str("baking"), py_str("melting")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("hungry"), call_builtin("frozenset", &[py_tuple(vec![py_str("hungry"), py_str("starving"), py_str("starved"), py_str("famished"), py_str("peckish"), py_str("ravenous")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("thirsty"), call_builtin("frozenset", &[py_tuple(vec![py_str("thirsty"), py_str("parched"), py_str("dehydrated")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("light_head"), call_builtin("frozenset", &[py_tuple(vec![py_str("faint"), py_str("fainted"), py_str("fainting"), py_str("faintish"), py_str("woozy"), py_str("dizzy"), py_str("giddy"), py_str("wobbly"), py_str("shaky"), py_str("lightheaded"), py_str("groggy")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("queasy"), call_builtin("frozenset", &[py_tuple(vec![py_str("queasy"), py_str("queezy"), py_str("nauseous"), py_str("nauseated"), py_str("sickly"), py_str("bilious")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("bunged"), call_builtin("frozenset", &[py_tuple(vec![py_str("bunged"), py_str("bungedup"), py_str("blockedup"), py_str("sniffly"), py_str("sniffles"), py_str("sniffing"), py_str("croaky"), py_str("hoarse"), py_str("sneezing"), py_str("congested"), py_str("husky"), py_str("snotty")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("clammy"), call_builtin("frozenset", &[py_tuple(vec![py_str("clammy"), py_str("clamy"), py_str("clammyish"), py_str("coldsweat"), py_str("coldsweats")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("pale"), call_builtin("frozenset", &[py_tuple(vec![py_str("pale"), py_str("pallid"), py_str("ashen"), py_str("grey"), py_str("gray"), py_str("greyish"), py_str("grayish"), py_str("mottled"), py_str("blotchy"), py_str("washedout")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("puffy"), call_builtin("frozenset", &[py_tuple(vec![py_str("puffy"), py_str("bloated"), py_str("swollen"), py_str("puffed"), py_str("puffing")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("spotty"), call_builtin("frozenset", &[py_tuple(vec![py_str("spotty"), py_str("pimply"), py_str("acne"), py_str("brokenout")])]).unwrap_or_else(|e| exc::raise(e)))]));
    s_BODY_TAIL(call_builtin("frozenset", &[py_tuple(vec![py_str("today"), py_str("tonight"), py_str("now"), py_str("still"), py_str("again"), py_str("though"), py_str("honestly"), py_str("already"), py_str("outside"), py_str("here"), py_str("actually"), py_str("really"), py_str("so"), py_str("very"), py_str("absolutely"), py_str("bloody"), py_str("thanks")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_SITUATION(py_tuple(vec![py_str("all the time"), py_str("all day"), py_str("all night"), py_str("all morning"), py_str("all afternoon"), py_str("all evening"), py_str("all week"), py_str("all weekend"), py_str("in here"), py_str("in this house"), py_str("in this weather"), py_str("at night"), py_str("at work"), py_str("in the night"), py_str("right through"), py_str("to the bone"), py_str("through to the bone"), py_str("all day today"), py_str("all day yesterday"), py_str("all night long")]));
    s_BODY_PART_TAIL(call_builtin("frozenset", &[py_tuple(vec![py_str(""), py_str("today"), py_str("now"), py_str("still"), py_str("again"), py_str("already"), py_str("outside")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_PART_QUALIFIER(call_builtin("frozenset", &[py_tuple(vec![py_str("really"), py_str("so"), py_str("very"), py_str("proper"), py_str("properly"), py_str("absolutely"), py_str("completely"), py_str("totally"), py_str("always"), py_str("still"), py_str("bit"), py_str("little"), py_str("a"), py_str("dead"), py_str("stone"), py_str("bloody"), py_str("permanently"), py_str("freezing"), py_str("bitterly"), py_str("that"), py_str("quite"), py_str("pretty"), py_str("fairly"), py_str("completely")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_LEAD(call_builtin("frozenset", &[py_tuple(vec![py_str("so"), py_str("oh"), py_str("ah"), py_str("god"), py_str("honestly"), py_str("well"), py_str("and"), py_str("also"), py_str("tbh"), py_str("ok"), py_str("okay"), py_str("literally"), py_str("seriously"), py_str("man"), py_str("ooh"), py_str("ugh"), py_str("yeah"), py_str("no")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_ALSO(call_builtin("frozenset", &[py_tuple(vec![py_str("tired"), py_str("knackered"), py_str("exhausted"), py_str("shattered"), py_str("grumpy"), py_str("sore"), py_str("achy"), py_str("sweaty"), py_str("miserable"), py_str("worn"), py_str("wet"), py_str("hungry"), py_str("thirsty")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_COLD(py_tuple(vec![py_str("Wrap up and get a hot drink in you - is the heating on where you are?"), py_str("Hot water bottle and something warm inside you. That beats waiting for the heating."), py_str("Cold gets into everything. Is there a radiator you can sit next to?"), py_str("Another layer and something hot to drink. Is it the whole place, or just the room you are in?"), py_str("That is too cold to think straight in. Get the heating on and a jumper on top of that.")]));
    s_BODY_HOT(py_tuple(vec![py_str("Get a cold drink in you and some air moving - is there a window you can open?"), py_str("Cold water on your wrists and a bit of a draught is the quickest way down."), py_str("That kind of heat is draining. Have you got somewhere cooler to be?"), py_str("Take it slowly in this, and get out of it for five minutes if you can."), py_str("Too warm to think straight in. Open a window and put a cold drink in you.")]));
    s_BODY_HUNGRY(py_tuple(vec![py_str("Then go and eat something - what have you got in?"), py_str("That is your body asking. Something with protein in it holds you longer than a biscuit."), py_str("Eat first and sort the rest out after. What is quickest to hand?"), py_str("You are running on empty there. Toast and an egg beats waiting for a proper meal."), py_str("Go and get something now, even if it is small - a sandwich is enough to steady you.")]));
    s_BODY_THIRSTY(py_tuple(vec![py_str("Get a glass of water in you now - are you at home, or out?"), py_str("Thirsty means you are already behind. A full glass of water, not a sip."), py_str("Water first, then whatever you were doing. Is there a tap handy?"), py_str("Have a proper drink of water and keep the glass next to you afterwards.")]));
    s_BODY_LIGHT_HEAD(py_tuple(vec![py_str("Sit down before anything else - did it come on when you stood up?"), py_str("Get sat down and have some water. Does it settle if you stay still for a minute?"), py_str("Take that slowly and hold on to something. Has it been creeping up on you, or was it sudden?"), py_str("Sit down and put your head down for a minute. If it keeps happening it is worth getting looked at."), py_str("Have you eaten anything today? Low blood sugar does that - get something in you and sit down.")]));
    s_BODY_QUEASY(py_tuple(vec![py_str("Sit up rather than lie flat and sip water - is anything staying down?"), py_str("Plain and dry beats anything rich right now: toast, a bit of rice. When did it start?"), py_str("Go steady with small sips. Is it just queasy, or have you been sick?"), py_str("Fresh air and water, and nothing heavy for an hour or two. Does it come in waves?"), py_str("That is a rotten way to spend an evening. Is it something you ate, do you think?")]));
    s_BODY_BUNGED(py_tuple(vec![py_str("Something hot to drink and ten minutes of steam in the bathroom - is it a cold coming on?"), py_str("That is usually the cold starting in the nose and throat. Is it blocked, or running?"), py_str("Honey in hot water and rest your voice - how long has it been like that?"), py_str("Keep the fluids up and get an early night. Is it the nose, the throat, or both?"), py_str("A cold runs its week and there is no rushing it. Are you blocked up, or is it sore?")]));
    s_BODY_CLAMMY(py_tuple(vec![py_str("Clammy is a cold sweat rather than being too warm. Have you eaten anything today?"), py_str("Get sat down and have something sweet if you have not eaten in a while. Does it come with feeling dizzy?"), py_str("That is usually your body working hard at something: a fever coming on, low blood sugar or a shock. Is it just your hands, or all over?"), py_str("Wash your hands and sit somewhere cool for a minute - has this come on today, or is it something you get? Cool water and a sit-down is the thing that helps most."), py_str("Cold and clammy with the room at a normal temperature is worth taking slowly. Sit down, have a drink, and if you feel faint or muddled with it, ring 111.")]));
    s_BODY_PALE(py_tuple(vec![py_str("Pale usually means something is going round, or you have not eaten. Have you had anything today?"), py_str("Sit down and have a drink, then something to eat - did it come on suddenly, or have you been like this a while?"), py_str("Grey and washed out with it is worth getting looked at rather than waiting. Is there any dizziness with it?"), py_str("Has anyone said you look pale, or does it just feel like it? Pale with a bug is normal enough; pale and dizzy is the one to get seen."), py_str("Put your feet up for ten minutes and have something with sugar in it. If you are clammy or muddled with it, get seen today.")]));
    s_BODY_PUFFY(py_tuple(vec![py_str("Bloated usually settles if you go easy on salt and fizzy drinks for a day. Is it worse after eating?"), py_str("Puffy ankles and hands are worth getting checked if it is new. Is it just your stomach, or your ankles too?"), py_str("That is your body holding water. Drink more rather than less and cut the salt for a few days - does it ease overnight?"), py_str("Which bit is puffy? After a big meal and a late night it is nothing, but swelling that stays is worth a look."), py_str("Go steady with salt and fizz and see if it settles by tomorrow. Is it tender with it?")]));
    s_BODY_SPOTTY(py_tuple(vec![py_str("Spotty skin is usually hormones and nothing to worry about. Is it your face, or your back too?"), py_str("Wash with a plain cleanser twice a day and keep your hands off them. How long has it been like that?"), py_str("A GP can help if they are sore or leaving marks - are you picking at them, or leaving them alone?"), py_str("That is common enough and it does pass. Does it bother you, or is someone else commenting on it?"), py_str("Spotty and itchy after a new cream or washing powder could be a reaction. Has anything changed lately?")]));
    s_BODY_POOLS(py_dict(vec![(py_str("cold"), g_BODY_COLD()), (py_str("hot"), g_BODY_HOT()), (py_str("hungry"), g_BODY_HUNGRY()), (py_str("thirsty"), g_BODY_THIRSTY()), (py_str("light_head"), g_BODY_LIGHT_HEAD()), (py_str("queasy"), g_BODY_QUEASY()), (py_str("bunged"), g_BODY_BUNGED()), (py_str("clammy"), g_BODY_CLAMMY()), (py_str("pale"), g_BODY_PALE()), (py_str("puffy"), g_BODY_PUFFY()), (py_str("spotty"), g_BODY_SPOTTY())]));
    s__SUBJECT_AFTER_AM(call_builtin("frozenset", &[mcall(&py_str("i we you they he she it there this that anyone anybody"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BAD_STRETCH(call_builtin("frozenset", &[py_tuple(vec![py_str("rubbish"), py_str("crap"), py_str("crappy"), py_str("rough"), py_str("tough"), py_str("hard"), py_str("awful"), py_str("terrible"), py_str("horrible"), py_str("grim"), py_str("stressful"), py_str("hectic"), py_str("rotten"), py_str("naff"), py_str("wearing"), py_str("draining")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STRETCH_OF_TIME(call_builtin("frozenset", &[py_tuple(vec![py_str("week"), py_str("weeks"), py_str("day"), py_str("days"), py_str("month"), py_str("months"), py_str("weekend"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("night"), py_str("shift"), py_str("term"), py_str("year"), py_str("today"), py_str("lately")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT_ACT(py_str("(?:\\w+ing|say|says|do|does|did|end|leave|put|finish|start|ask|text|snap|interrupt|overthink|forget|argue|second|manage|seem|apologi[sz]e|check|cancel|procrastinate|agree|accept|take|give|spend|buy|work|plan|tidy|worry|complain|volunteer|pick|choose|eat|drink|run|scroll|compare|share|tell|hide|swallow|push|hold|bite|clench|stay|go|turn|shut|withdraw|avoid|help|save|spend|arrive|stick|wander)"));
    s_HABIT_REPLY(py_tuple(vec![py_str("That sounds like a pattern rather than a one-off. What happens just before it?"), py_str("Habits like that are usually a trigger and a shortcut, not a lack of will. When does it bite most?"), py_str("How long has it been like that, and has anything ever made it better for a while?"), py_str("What have you tried so far? What did not work usually points at what would."), py_str("Worth looking at what sets it off rather than trying harder at it. What tends to come just before?")]));
    s_BAD_STRETCH_REPLIES(py_tuple(vec![py_str("That sounds like a rough one. What's been the worst of it?"), py_str("Fair enough — that takes it out of you. What has been going on?"), py_str("Sorry. Do you want to tell me what has made it like that?"), py_str("That is a lot to hold at once. What has been the hardest part?")]));
    s_COMFORT_SAID(call_builtin("frozenset", &[op_add(&op_add(&op_add(&op_add(&op_add(&g_NEGATIVE(), &g_BAD_STRETCH_REPLIES()), &g_RUMINATION_REPLIES()), &g_NO_DIRECTION_REPLY()), &g_NOT_ENOUGH_REPLY()), &g_LEFT_POOL())]).unwrap_or_else(|e| exc::raise(e)));
    s_HOUSE_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("take"), py_str("put"), py_str("turn"), py_str("switch"), py_str("fill"), py_str("lock"), py_str("open"), py_str("close"), py_str("wash"), py_str("hoover"), py_str("tidy"), py_str("clean"), py_str("set"), py_str("feed"), py_str("walk"), py_str("bring")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_STATE_CUES(call_builtin("frozenset", &[py_tuple(vec![py_str("pregnant"), py_str("engaged")])]).unwrap_or_else(|e| exc::raise(e)));
    s__WAS(call_builtin("frozenset", &[py_tuple(vec![py_str("was"), py_str("were"), py_str("been"), py_str("used")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CROWD(call_builtin("frozenset", &[py_tuple(vec![py_str("everyone"), py_str("everybody"), py_str("everyones"), py_str("peoples"), py_str("others"), py_str("nobody")])]).unwrap_or_else(|e| exc::raise(e)));
    s_YOU_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i im ive id ill me myself we us"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("for"), py_str("of"), py_str("about")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_WORDS(call_builtin("frozenset", &[mcall(&py_str("me us we you my our myself ourselves"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BIRTHDAY_STILL_TO_COME(py_tuple(vec![py_str("{who} turns {age} {when} — that's one to look forward to. Are you doing anything for it?"), py_str("{age} {when} — a good one to have ahead. What are you thinking of doing?"), py_str("Lovely — {who} at {age} {when}. Will you mark it somehow?"), py_str("That's nice to have coming up. {age} {when} is worth making a fuss of — anything planned?")]));
    s_ROUTINE(py_tuple(vec![py_str("Pick the one thing that would make the day easier and give it a fixed slot; everything else can move around it. What is the thing you keep not getting to?"), py_str("Make it small enough that a bad day cannot beat it — two minutes, the same trigger each time — and add to it once it holds. What would it look like on your worst day?"), py_str("Work back from when you have to leave: what time to get up, what has to happen before that, and what can be dropped. Where does the morning usually go wrong?"), py_str("Write tomorrow down tonight — three things, one of them small — and do the smallest first, before the day fills up. Where does the time usually go?")]));
    s_TOO_MUCH(py_tuple(vec![py_str("Then something has to come off the list. Which of it would actually go wrong if it waited a week?"), py_str("That is more than anyone can hold in their head — get it out of your head and onto paper, then mark the two things that are really due. What is on the list?"), py_str("When it is this much, the useful question is what to drop rather than how to fit it in. What is the one thing that cannot slip?")]));
    s_ROUTINE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("routine"), py_str("routines"), py_str("habit"), py_str("habits"), py_str("organised"), py_str("organized"), py_str("procrastinating"), py_str("discipline"), py_str("productive"), py_str("overwhelmed"), py_str("drowning"), py_str("swamped")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_PHRASES(py_tuple(vec![py_str("get organised"), py_str("get organized"), py_str("get my act together"), py_str("stick to it"), py_str("stick at it"), py_str("stick with it"), py_str("always late"), py_str("never on time"), py_str("get up earlier"), py_str("sort out my mornings"), py_str("sort my mornings"), py_str("sort out my days"), py_str("to do list"), py_str("time management"), py_str("keep on top of"), py_str("keep putting things off"), py_str("put things off"), py_str("fall behind"), py_str("get behind"), py_str("no time in the morning"), py_str("no time in the day"), py_str("never enough time in the morning"), py_str("rubbish at mornings"), py_str("useless at mornings"), py_str("not a morning person"), py_str("bad at getting up"), py_str("cant get going in the morning"), py_str("mornings are a struggle"), py_str("dread mornings"), py_str("late for everything"), py_str("late to things"), py_str("be on time")]));
    s_TOO_MUCH_PHRASES(py_tuple(vec![py_str("too much on"), py_str("spread too thin"), py_str("too many things"), py_str("so much to do"), py_str("juggle"), py_str("juggling"), py_str("overloaded"), py_str("stretched thin"), py_str("where to start"), py_str("where to begin")]));
    s__START_ASK(py_tuple(vec![py_str("where to start"), py_str("where to begin")]));
    s_ROUTINE_NOT(call_builtin("frozenset", &[py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("toddler"), py_str("toddlers"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("nursery"), py_str("teen"), py_str("teens"), py_str("teenager"), py_str("teenagers"), py_str("school run"), py_str("son"), py_str("daughter"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("pup"), py_str("shifts"), py_str("shift"), py_str("hospital"), py_str("wing"), py_str("money"), py_str("bills"), py_str("bill"), py_str("tax"), py_str("taxes"), py_str("budget"), py_str("accounts"), py_str("invoice"), py_str("invoices"), py_str("payments"), py_str("debts"), py_str("debt"), py_str("rent"), py_str("mortgage"), py_str("loan"), py_str("loans"), py_str("insurance"), py_str("pension"), py_str("wages"), py_str("salary"), py_str("overdraft"), py_str("fines"), py_str("cook"), py_str("cooking"), py_str("dinner"), py_str("tea"), py_str("meals"), py_str("meal")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BEHIND_ON_MONEY(call_builtin("frozenset", &[py_tuple(vec![py_str("late"), py_str("behind"), py_str("overdue"), py_str("missed"), py_str("miss"), py_str("arrears"), py_str("skint"), py_str("broke")])]).unwrap_or_else(|e| exc::raise(e)));
    s__NO_DAY_OF_THEIR_OWN(py_tuple(vec![py_str("died"), py_str("dead"), py_str("passed away"), py_str("passed on"), py_str("put down"), py_str("put to sleep"), py_str("buried"), py_str("funeral"), py_str("cremation"), py_str("euthanised"), py_str("lost")]));
    s__DAY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("morning"), py_str("mornings"), py_str("day"), py_str("days"), py_str("evening"), py_str("evenings"), py_str("night"), py_str("nights"), py_str("week"), py_str("weeks"), py_str("weekend"), py_str("weekends"), py_str("time"), py_str("times"), py_str("work"), py_str("shift"), py_str("shifts"), py_str("today"), py_str("tomorrow"), py_str("life"), py_str("self"), py_str("routine"), py_str("routines"), py_str("everything"), py_str("anything"), py_str("nothing"), py_str("things"), py_str("thing"), py_str("moment"), py_str("minute"), py_str("minutes"), py_str("hour"), py_str("hours"), py_str("rest"), py_str("whole")])]).unwrap_or_else(|e| exc::raise(e)));
    s_IMPERATIVE(call_builtin("frozenset", &[mcall(&py_str("give suggest recommend tell show make write find help list name pick\n    choose plan"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PUNT(py_tuple(vec![py_str("a search should sort you out"), py_str("quick search online"), py_str("i'd rather not guess"), py_str("that's beyond what i know well"), py_str("searching for it will turn up")]));
    s_FRAME_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("meant"), py_str("mean"), py_str("said"), py_str("say"), py_str("sorry"), py_str("no"), py_str("not"), py_str("wait"), py_str("oops"), py_str("scratch"), py_str("that"), py_str("actually"), py_str("rather"), py_str("than"), py_str("or")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEBODY_HURT(py_tuple(vec![py_str("God, that's awful. Is {who} alright?"), py_str("That's bad news. How is {who} now?"), py_str("I'm sorry. Is {who} alright?"), py_str("Christ. How is {who} doing?"), py_str("That's a horrible thing to hear. Is {who} okay?")]));
    s_A_REST_OF_A_GOOD_DAY(py_tuple(vec![py_str("That fits the day you have had. Good ones are worth saying out loud."), py_str("The rest of a good day, then. Hold on to that one."), py_str("Lovely. Those are the days that stick."), py_str("That is a nice thing to have in a week. Enjoy the rest of it.")]));
    s_A_REST_OF_A_HARD_DAY(py_tuple(vec![py_str("That is the rest of a hard day, not the whole of it. Let the evening be small."), py_str("Long one on top of a long one. Something warm to eat and an early night."), py_str("Days like that pile up. You have done the day, so put your feet up now."), py_str("That sounds like a full day on top of a hard one. Nothing else needs doing tonight.")]));
    s_LOW_DAY_REPLIES(py_tuple(vec![py_str("That sounds like a day to get through rather than enjoy. What made it that way?"), py_str("Sorry - that is a heavy one to carry. What has been the worst part of it?"), py_str("Long days like that take it out of you. Do you want to talk it through, or would you rather think about something else for a bit?"), py_str("That is a rough one. Is it the kind that lifts in a day or two, or one that has been building?"), py_str("Ugh. What happened?")]));
    s_A_THING_THEY_REGRET(py_tuple(vec![py_str("You don't have to dress it up for me. What happened?"), py_str("That's the kind of thing that sits on you. Do you want to talk it through?"), py_str("Whatever it was, I'd rather hear it than not. What happened?"), py_str("That sounds like it's been weighing on you. What happened?")]));
    s_A_BETRAYAL(py_tuple(vec![py_str("That's a rotten thing to find out. What do you know so far, and what is still guesswork?"), py_str("Ugh. That's a lot to take in in one go. Say it as it came."), py_str("That hurts, and it is not on you. What happened?"), py_str("Rotten news, and I'm sorry. Say it out loud if you want, or we can talk about something else for a bit."), py_str("That changes how the whole thing looks, doesn't it. I'm here."), py_str("Awful. You do not have to work out what to do about it today."), py_str("That's the kind of thing that sits on your chest. Tell me what happened.")]));
    s_A_CARE_HOME(py_tuple(vec![py_str("It takes a while to feel like home, for everyone. What is it like when you visit?"), py_str("How is {they} finding it there? The first weeks are the strangest for most people."), py_str("That is a big change for {them}, and for you. How is {they} settling in?")]));
    s_A_CARE_HOME_MOVED(py_tuple(vec![py_str("That is a big change for {them}, and for you. How is {they} settling in?"), py_str("That is a hard thing to have had to decide. How are you doing with it?"), py_str("It takes a while for everyone. How has {they} been since the move?")]));
    s_A_CARE_HOME_VISITING(py_tuple(vec![py_str("The visiting is the part that counts, and it is tiring too. What do you usually do together when you are there?"), py_str("A visit is a lot of work before and after it. How often do you get over there?")]));
    s_A_CARE_HOME_GUILT(py_tuple(vec![py_str("Guilt comes with this even when the decision was the right one, and it is not evidence of anything. What does a visit look like these days?"), py_str("That guilt is very common and it does not mean you did the wrong thing. Have you got anyone to say all this to?")]));
    s_A_CARE_HOME_BRING(py_tuple(vec![py_str("The things that make the room theirs: photos in a frame, a favourite blanket or cardigan, a radio or their own mug. Take one thing a visit rather than a car load, and check what the room has room for first."), py_str("Ask what {they} misses from home - a particular biscuit, the paper, a cushion off the old sofa - and take one of those, plus something to do together. The visits matter more than the things.")]));
    s_A_SMALL_DAY(py_tuple(vec![py_str("Sounds alright. Anything good in it?"), py_str("Nice one. How did the rest of it go?"), py_str("Good to hear. What was the best bit?"), py_str("How was it, overall?")]));
    s_A_NEWS_OF_WEATHER(py_tuple(vec![py_str("Then the local forecast is the thing that knows, not me: it gives the hour it lands and how hard, and anything loose outside is worth sorting before then - the bins in, the panel that already leans screwed back, the trampoline turned over."), py_str("Worth an hour on the loose things before it gets here: the fence panel, the bin store, the pot that goes over anyway. The local forecast has the gusts, and that is the number that says whether the hour is worth spending."), py_str("The local forecast will tell you the gusts and when; what it cannot tell you is what of yours is already loose. Walk the garden once and fix that, and the rest is out of your hands.")]));
    s_A_NEWS_OF_SOMETHING_ELSE(py_tuple(vec![py_str("Then whoever put it out has the detail, and the detail is the part that matters - what it covers, from when, and what you do about it. It is usually one line further down than the headline."), py_str("Worth reading it at the source rather than the report of it: a closure or a change is usually a date, an area and a form, and the headline leaves all three out."), py_str("The body that said it will have the version with the dates in it, and the question worth asking is whether it changes anything of yours - a journey, a bin day, a plan.")]));
    s_A_THING_TAKEN(py_tuple(vec![py_str("That's rotten. Is there anything that has to happen tonight - a lock, a card to stop?"), py_str("Sorry - that is a horrible thing to have happen. Was it insured, or is this coming out of your pocket?"), py_str("That's a blow. Did they get anything else, or just the one thing?"), py_str("That's low. Where was it when it went?"), py_str("That's rotten, and it is worth reporting today while it is fresh - the reference number is what the insurer and the bank both ask for. Say the word and I'll walk you through the claim step by step.")]));
    s_A_BAD_TIME_TOLD(py_tuple(vec![py_str("Sounds like a rough one. What happened?"), py_str("Ugh, that is a shame. Do you want to get into it?"), py_str("That is a nuisance. Is there anything that would help?"), py_str("How are you doing with it?"), py_str("Sorry to hear it. What would make today easier?")]));
    s_A_NAME_STORY(py_tuple(vec![py_str("{names}. What is the story behind that name?"), py_str("{names} - good to know. Where did that come from?"), py_str("{names}. Who picked it?")]));
    s_A_NAMED_AFTER(py_tuple(vec![py_str("Named after {who}. What {are} {they} like?"), py_str("Named after {who}. Does the name fit, or is it a big one to carry?"), py_str("Named after {who} - I like that. Where did the idea come from?")]));
    s_A_NAMED_AFTER_THEM(py_tuple(vec![py_str("Named after you - that is a compliment. Whose idea was it?"), py_str("Named after you. Does the name fit, or is it a big one to carry?"), py_str("Named after you - I like that. What was the thinking there?")]));
    s_A_NAME_TOLD(py_tuple(vec![py_str("{names}. What {are} {they} like?"), py_str("{names} - good to know. What {are} {they} like?"), py_str("{names}. Tell me about {them}."), py_str("{names}. What should I know about {them}?")]));
    s_A_PLAN_THAT_FELL_THROUGH(py_tuple(vec![py_str("That's a shame. Is it off altogether, or moved?"), py_str("Gutted. What got in the way?"), py_str("Ah no. What stopped it?"), py_str("Sorry to hear it. Can it still happen?"), py_str("That is a shame. Do you get another go at it?")]));
    s_A_STATE_NOW(py_tuple(vec![py_str("How is that going?"), py_str("How long has that been the case?"), py_str("Does that suit you, or would you change it?")]));
    s_A_STATE_PAST(py_tuple(vec![py_str("How did that go?"), py_str("How do you look back on it?"), py_str("What do you make of it now?")]));
    s_A_DAY_ASKED_PLAIN(py_tuple(vec![py_str("How did that go, overall?"), py_str("What was it like?"), py_str("How did you find it?")]));
    s_A_THING_CAME_OUT_WRONG(py_tuple(vec![py_str("That's a shame. What went wrong with it?"), py_str("Ah no. Can it be rescued, or is it a start again?"), py_str("Annoying when that happens. What was it you made?"), py_str("Gutted. Had you made it before?"), py_str("That is a nuisance. Do you know what did it?")]));
    s__THING_DONE_LINES(call_builtin("frozenset", &[op_add(&op_add(&op_add(&op_add(&g_A_THING_DONE(), &g_A_THING_GONE()), &g_A_THING_ON()), &g_A_THING_SEEN_TO()), &g_A_THING_BOUGHT())]).unwrap_or_else(|e| exc::raise(e)));
    s_DISH_PARTICIPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("packed"), py_str("baked"), py_str("fried"), py_str("roasted"), py_str("mashed"), py_str("toasted"), py_str("scrambled"), py_str("boiled"), py_str("poached"), py_str("grilled"), py_str("stuffed"), py_str("pickled")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DISH_DETERMINER(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("its"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("some"), py_str("any"), py_str("all"), py_str("each"), py_str("one"), py_str("two"), py_str("three"), py_str("every"), py_str("another"), py_str("it"), py_str("them"), py_str("everything")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST(stdlib_call("re", "compile", &[py_str("\\bwill\\s+(?:it|that|this|they|he|she|i|we|my\\s+\\w+|the\\s+\\w+)\\b[^?.!]{0,40}?\\b(?:get\\s+better|get\\s+worse|be\\s+(?:ok|okay|all\\s+right|alright|fine)|\\s*come\\s+back|come\\s+right|come\\s+down|go\\s+(?:away|down)|recover|heal|clear\\s+up|ease|\\s*settle|pass|stop|wear\\s+off|sort\\s+(?:itself|themselves)\\s+out|\\s*pull\\s+through|make\\s+it)\\b|\\b(?:is|are|will)\\s+(?:he|she|it|that|this|they|my\\s+\\w+)\\b[^?.!]{0,30}?\\b(?:going\\s+to|gonna)\\s+(?:be\\b|get\\s+(?:better|worse)|heal|recover|\\s*clear\\s+up|come\\s+(?:back|right)|settle|ease|\\s*make\\s+it|pull\\s+through)\\b|\\bwill\\s+(?:he|she|they|my\\s+\\w+|the\\s+\\w+)\\b[^?.!]{0,20}?\\bbe\\s+(?:out|home|back|walking|mobile|up\\s+and\\s+about|better|on\\s+the\\s+mend)\\b\\s*(?:by|before|for|on|in\\s+time\\s+for|this|next)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_PERMISSION(stdlib_call("re", "compile", &[py_str("\\bwill\\s+(?:it|that|this)\\s+be\\s+(?:ok|okay|fine|alright|all\\s+right)\\s+(?:if|to|with|until|till|overnight)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_DURATION(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_I_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:will|am)\\s+i\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_RELATION(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(dad|father|mum|mom|mother|nan|nana|gran|granny|grandma|grandad|grandpa|\\s*husband|wife|partner|boyfriend|girlfriend|son|daughter|brother|sister|baby|boy|girl|friend|mate|dog|cat|pup|puppy|kitten|rabbit|hamster|horse|pony|bird|parrot|guinea|gerbil|ferret|fish)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_HE_SHE(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|him|her|his)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_STRUGGLE(stdlib_call("re", "compile", &[py_str("\\b(?:struggl\\w*|cop(?:e|ing)|depress\\w*|anxi\\w*|upset|cry(?:ing)?|tears|unhapp\\w*|miserable|lonely|isolated|withdrawn|overwhelm\\w*|burn(?:t|ed)?\\s+out|breaking\\s+down|hard\\s+time|going\\s+through\\s+(?:it|a\\s+lot)|feel(?:ing)?\\s+(?:low|flat|down)|lost\\s+my\\s+job|redundant|sacked|not\\s+(?:her|him)self|self\\s*harm|suicidal|dark\\s+place)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_ILL(call_builtin("frozenset", &[mcall(&py_str("\nfall falls fell hospital ward operation op surgery treatment diagnosed diagnosis pneumonia\ninfection sepsis chemo chemotherapy stroke fracture broken stitches plaster cast scan consultant\nphysio physiotherapy terminal prognosis recovering recovery intensive icu a&e casualty accident\naccidents crash crashed collision injured injury hurt ambulance paramedic paramedics\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_PERSON(py_tuple(vec![py_str("Nobody can promise that, and I won't guess about another person's health. What it comes down to is the direction - better than yesterday, or not - and the ones looking after {who} can see that. Ask them what they are watching for, and what the next few days should look like."), py_str("That is the doctors' call, not mine - I can't see {who}. What is in your hands is asking them straight how it compares with yesterday and what would count as a setback; tell me what they say and I'll help you think it through."), py_str("I can't promise that, and it would not be fair to. What the people treating {who} can tell you is the useful part: what is going well, what would worry them, and what the next few days should hold.")]));
    s_FORECAST_SELF(py_tuple(vec![py_str("No one can promise that from here - mine would be a guess about your body, and that is not a guess worth having. What decides it is the direction: if it is easier this week than last, keep going as you are; if it is no better in a couple of weeks, or it gets worse, 111 will say whether it needs looking at today."), py_str("Probably, given time - and nobody can say for certain from here. The thing to watch is whether it keeps improving; if it stalls, or it starts to get worse, that is one for your GP rather than wait-and-see."), py_str("I can't call that from here, and a guess about a body is the last thing you need. Bodies mend at their own speed, so the question worth asking is whether it is improving week by week; your GP can say what to expect and when to come back - take the same description you gave me.")]));
    s_FORECAST_SOMEONE_STRUGGLING(py_tuple(vec![py_str("I can't promise that, and I won't pretend to - nobody can say how another person's hard patch ends. Staying in touch this week is the part you can do, and if it gets darker than that, their GP is the one who can step in."), py_str("Nobody can call that from here, and a forecast would be a guess. Keep asking how they really are - and if there's any talk of not wanting to be here, that is a doctor's call the same day, not something to wait out.")]));
    s_FORECAST_SELF_STRUGGLING(py_tuple(vec![py_str("I can't promise that, and a forecast would be a guess. What does move it is this week: keep the ordinary things going - sleep, food, daylight, one person who knows how it really is - and if it is no better in a fortnight, take it to your GP."), py_str("Nobody can call that from here, and I won't pretend to. Tell one person the true version rather than the short one, keep the week as ordinary as you can, and if it gets darker than that, your GP is the one who can step in.")]));
    s_FORECAST_PET(py_tuple(vec![py_str("Only the vet can say that - I can't examine {who} from here. What the desk will ask is the useful part: is {who} eating and drinking, as lively as usual, and has anything been kept down? Ring with that and ask whether to bring {who} in tonight."), py_str("I can't call that from here, and the vet has to see {who}. What decides it is usually how {who} is in themselves - eating, drinking, energy - so note that down and ring the desk; they will say whether it can wait until the morning."), py_str("Nobody can say that without seeing {who}, and the vet can. Take what you've just told me, add whether {who} is eating and drinking, ask what would mean bringing {who} in, and go with what they say.")]));
    s_FORECAST_AFTER_SAFETY(py_tuple(vec![py_str("I can't say how it will go, and nobody can from here. What you can go on is the direction - steadier hour by hour, or not - and the ones treating {who} can see it: keep doing what they told you, and ask them what to expect and what would worry them."), py_str("That is for the people looking after {who} to answer, not me - they can see what I can't. Keep doing exactly what they told you, and ask them straight what the next few days should look like; I'm here for the rest of it.")]));
    s_FORECAST_POOLS(py_tuple(vec![g_FORECAST_SELF(), g_FORECAST_PERSON(), g_FORECAST_SOMEONE_STRUGGLING(), g_FORECAST_PET(), g_FORECAST_AFTER_SAFETY()]));
    s_FORECAST_BODY_WORDS(op_bitor(&op_bitor(&op_bitor(&g_BODY_PARTS(), &g_SYMPTOM_WORDS()), &g_HEALTH_WORDS()), &g_FORECAST_ILL()));
    s_BODY_WORD_FOR_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:back|shoulders?|arms?|foot|feet|head|chest|hips?|ribs?|spine|belly|knees?|neck|hands?|legs?|elbows?|waist)\\s+(?:doors?|gardens?|yards?|walls?|rooms?|gates?|fences?|windows?|lawn|beds?|paths?|steps?|porch|seats?|lights?|lanes?|roads?|alleys?|shed|deck|patio|fields?|cars?|wheels?|sides?|covers?|pages?|pockets?|boards?|racks?|rest|cushions?|chairs?|tables?|freezer|drawers?|cupboards?|wardrobes?|stairs?|staircase|lines?|notes?|rows?|bands?|straps?|ties?|muffs?|warmers?|of\\s+the\\s+(?:stairs|bed|sofa|room|house|boat|car|garden))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_OFF_THEIR_FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:stopped|stopping|not|no\\s+longer|wont|won'?t|isnt|isn'?t|keeps?)\\s+eating\\b|\\boff\\s+(?:her|his|their)\\s+food\\b|\\b(?:lost|loosing|losing)\\s+(?:her|his|their)\\s+appetite\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_PET_HARM(stdlib_call("re", "compile", &[py_str("\\b(?:drank|drink|swallow(?:ed|s|ing)?|ate|eaten|eats?|poison\\w*|toxic|lil(?:y|ies)|chocolate|grapes?|raisins?|onions?|xylitol|antifreeze|bleach|hit\\s+by|ran\\s+over|run\\s+over|fell|fallen|limp(?:ing)?|lame|collapsed|fitting|seizures?|wheez\\w*|vomit\\w*|vomited|bleeding|blood|in\\s+pain|shaking|not\\s+eating|stopped\\s+eating|off\\s+(?:her|his|their)\\s+food|hurt)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORECAST_A_FALL(stdlib_call("re", "compile", &[py_str("\\b(?:came|come|cames|fell|fallen|falling|was|were|got|been)\\s+off\\s+(?:his|her|their|the|a|my)\\s+(?:bike|motorbike|horse|ladder|roof|scooter|skateboard|bicycle|moped|quad)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DITHER_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:whether to (?!laugh|cry|scream|shout|smile)\\w+|if i (?:should|shall) \\w+|if i can (?:afford|manage|keep|take|handle|fit|cope|swing)\\b|between \\w+|which \\w+ to (?:pick|choose|take|try|go)|what to do (?:about|with|next|now)\\b|(?:decid(?:e|ed|ing)|think(?:ing)?|thought|wonder(?:ing)?|consider(?:ing)?|want)s?\\s+(?:about\\s+|of\\s+)?(?:whether\\s+)?to (?:go|take|move|stay|leave|quit|apply|accept|buy|sell)\\b|i (?:want|wanted|dont want) to (?:go|take|move|stay|leave|quit|apply|accept|buy|sell)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_OUT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(i|im|he|hes|she|shes|we|were|they|theyre|(?:my|our|the)\\s+(?:(?:little|young|eldest|oldest|older|grown\\s?up|poor|baby|new)\\s+)?"), &g_PASSED_OUT_PERSON()), &py_str(")\\b(?:\\s+\\w+){0,4}\\s+(?:passed out|collapsed|fainted|blacked out|keeled over|had a seizure|passing out|collapsing|fainting|blacking out|keeling over|having a seizure|collapses|faints|passes out|blacks out|keeps? (?:passing out|collapsing|fainting|blacking out|going down)|pass(?:ing|ed)? out (?:sometimes|a lot|often|now and then|every so often|a few times)|goes down\\b(?=\\s*(?:all|suddenly|on|in (?:the|a)\\s|while|when|at\\b))|went down(?!\\s*(?:to|the|stairs|steps|hill|road|lane|street|with|in|by|for))|(?:(?:about|going|gonna)\\s+to|might|may)\\s+(?:pass\\s+out|faint|black\\s+out|collapse|go\\s+down))\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSING_OUT_A_THING(stdlib_call("re", "compile", &[py_str("\\bpass(?:ing|ed|es)?\\s+out\\s+(?:(?:the|a|an|some|those|these|our|their|his|her|my|its)\\s+(?!(?:bit|little|lot|few|couple|while|moment|second|seconds|minute|minutes|mins|hour|hours|day|days|night|nights|time|times|again|now|too|other|next|thing|door|window|gate|back|way|rest|floor|sofa|chair|bed|bus|train)\\b)\\w+|(?!(?:afterwards?|after|before|later|once|twice|thrice|again|sometimes|always|often|repeatedly|nowadays|every|most|these|days|nights|weeks|months|years)\\b)[a-z]{2,}s\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_PICK_LOST(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:so|and|but|ok|okay|well|then|just|hey|hi|right|anyway|anyways|oh|hmm|yeah|yes|alright|honestly|god|christ|ugh|tbh|seriously|you\\s+know)[\\s,]+)*(?:i\\s+)?(?:really\\s+|honestly\\s+|genuinely\\s+|actually\\s+|still\\s+|simply\\s+|just\\s+)*(?:(?:do\\s+not|don'?t|dont)\\s+(?:even\\s+)?know|dunno|(?:have|ve|got|i'?ve)\\s+(?:absolutely\\s+)?no\\s+(?:idea|clue)|(?:have\\s+)?no\\s+(?:idea|clue)|(?:i\\s+am\\s+|i'?m\\s+)?not\\s+(?:really\\s+|too\\s+)?sure)\\s+what\\s+(?:to\\s+do|i\\s+(?:should|can|could|might)\\s+do)\\b(?:\\s+(?:now|next|first|today|tonight|tomorrow|instead|else|at\\s+all|anymore|any\\s+more|about\\s+(?:it|this|that)|with\\s+(?:it|this|that|them)))?\\s*\\.?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_BODIES(py_tuple(vec![py_tuple(vec![py_str("citizens advice"), py_str("Citizens Advice")]), py_tuple(vec![py_str("national debtline"), py_str("National Debtline")]), py_tuple(vec![py_str("health visitor"), py_str("Your health visitor")]), py_tuple(vec![py_str("school office"), py_str("The school office")]), py_tuple(vec![py_str("head of year"), py_str("The head of year")]), py_tuple(vec![py_str("gas safe"), py_str("A Gas Safe engineer")]), py_tuple(vec![py_str("letting agent"), py_str("The letting agent")]), py_tuple(vec![py_str("trading standards"), py_str("Trading standards")]), py_tuple(vec![py_str("environmental health"), py_str("The council's environmental health team")]), py_tuple(vec![py_str("the council"), py_str("The council")]), py_tuple(vec![py_str("your council"), py_str("The council")]), py_tuple(vec![py_str("the vet"), py_str("Your vet")]), py_tuple(vec![py_str("a vet"), py_str("Your vet")]), py_tuple(vec![py_str("pharmacist"), py_str("A pharmacist")]), py_tuple(vec![py_str("111"), py_str("111")]), py_tuple(vec![py_str("a&e"), py_str("A&E")]), py_tuple(vec![py_str("your gp"), py_str("Your GP")]), py_tuple(vec![py_str("the gp"), py_str("Your GP")]), py_tuple(vec![py_str("dentist"), py_str("A dentist")]), py_tuple(vec![py_str("landlord"), py_str("Your landlord")]), py_tuple(vec![py_str("the school"), py_str("The school")]), py_tuple(vec![py_str("solicitor"), py_str("A solicitor")]), py_tuple(vec![py_str("the bank"), py_str("Your bank")]), py_tuple(vec![py_str("your bank"), py_str("Your bank")]), py_tuple(vec![py_str("ombudsman"), py_str("The ombudsman")]), py_tuple(vec![py_str("plumber"), py_str("A plumber")]), py_tuple(vec![py_str("electrician"), py_str("An electrician")]), py_tuple(vec![py_str("roofer"), py_str("A roofer")]), py_tuple(vec![py_str("the police"), py_str("The police")]), py_tuple(vec![py_str("registrar"), py_str("The register office")]), py_tuple(vec![py_str("dwp"), py_str("The DWP")]), py_tuple(vec![py_str("hmrc"), py_str("HMRC")]), py_tuple(vec![py_str("stepchange"), py_str("StepChange")]), py_tuple(vec![py_str("samaritans"), py_str("Samaritans")]), py_tuple(vec![py_str("your insurer"), py_str("Your insurer")]), py_tuple(vec![py_str("acas"), py_str("ACAS")]), py_tuple(vec![py_str("your union"), py_str("Your union")]), py_tuple(vec![py_str("hr"), py_str("HR")]), py_tuple(vec![py_str("the midwife"), py_str("Your midwife")])]));
    s_PRIVACY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|store|stores|storing|save|saves|saving|record|records|recording|log|logs|logging|share|shares|sharing|sell|sells|selling)\\s+(?:on\\s+|out\\s+|around\\s+|off\\s+|away\\s+)?(?:what\\s+i\\s+(?:said|say|tell|type|typed|write|wrote)|what\\s+we\\s+(?:said|talk|discussed)|my\\s+(?:messages?|data|details|information|info|chats?|records?|conversations?)|our\\s+(?:chats?|conversations?)|this\\s+(?:chats?|conversations?)|the\\s+(?:chats?|conversations?))\\b|\\b(?:can|could|will|would|do|does|did)\\s+(?:anyone|anybody|someone|somebody|people|other\\s+people|people\\s+else|anyone\\s+else|everyone|staff)\\s+(?:else\\s+)?(?:see|read|hear|get|access|look\\s+at)\\s+(?:this|what|my|our|the)\\b|\\b(?:do|does|did|would|will|can|could)\\s+(?:you|u)\\s+(?:ever\\s+)?(?:tell|show|share|give|send|hand|pass|forward)\\s+(?:anyone|anybody|someone|somebody|people|others|a\\s+third\\s+party)\\b|\\bwho\\s+(?:(?:can|could|else\\s+can|else\\s+could)\\s+(?:see|read|access)|sees|reads|accesses)\\s+(?:this(?:\\s+(?:chat|conversation|thread))?(?=\\s*[?.!]*\\s*$)|my\\s+(?:messages?|data|chats?)\\b|what\\s+i\\s+(?:say|type|write)\\b)|\\bis\\s+(?:this|it|what\\s+i\\s+say|what\\s+i\\s+tell\\s+you)(?:\\s+(?:chat|conversation|page|app|thing|site))?\\s+(?:private|confidential|secure|saved|recorded|stored)\\b|\\bare\\s+(?:my|our|these)\\s+(?:messages?|data|details|chats?|records?|words)\\s+(?:private|safe|secure|confidential|shared|stored|saved|recorded|logged)\\b|\\b(?:does|do|did|is|was)\\s+(?:this|it|that|everything\\s+i\\s+say|what\\s+i\\s+say)\\s+(?:get\\s+|being\\s+|ever\\s+get\\s+)?(?:shared|stored|saved|recorded|logged|kept|seen|read|passed)\\b|\\bare\\s+(?:you|u)\\s+(?:keeping|recording|saving|storing|logging)\\s+(?:this|what\\s+i|my|our|the\\s+(?:chat|conversation))\\b|\\b(?:do|does|did|will|would|can|could)\\s+(?:you|u)\\s+(?:ever\\s+)?(?:keep|save|log|record|store)\\s+(?:a\\s+|any\\s+)?(?:record|log|copy|note|transcript|file|history)\\s+of\\b|\\b(?:do|does|did|would|will|can|could)\\s+(?:you|u)\\s+(?:ever\\s+)?(?:pass|send|hand|give|forward|share)\\s+(?:this|that|it|what\\s+i\\s+say)\\s+(?:on|along|around|off|over)?\\s*(?:to|for)\\s+(?:anyone|anybody|someone|somebody|people|others|a\\s+third\\s+party)\\b|\\bprivacy\\s+(?:here|on\\s+this|in\\s+this|with\\s+you|at\\s+all)\\b|\\bwhat\\s+(?:are|do)\\s+(?:you|u)\\s+(?:(?:do|doing)\\s+)?(?:with|using)\\s+(?:(?:my|our)\\s+(?:data|details|information|info|messages|words)|what\\s+i\\s+(?:tell|say|type|write))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_WHAT_I_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:keep|store|save|record|log|share|sell|pass)\\w*\\s+(?:me\\s+|us\\s+)?(?:the\\s+|a\\s+|an\\s+|my\\s+|our\\s+)?(?:kitchen|cat|dog|plants?|receipts?|leftovers?|food|milk|bread|eggs?|cheese|wine|beer|keys?|car|bike|house|flat|money|savings|cash|password|photos?|pictures?|files?|documents?|sheet|table|bill|bills|tickets?|seats?|room|garage|garden|washing|clothes|coat|bag|phone|number|address|email|weight|diary|journal|notes)\\b|\\bprivate\\s+(?:rented|rent|rental|landlord|landlords|sale|sector|school|schools|hospital|hospitals|treatment|health|healthcare|health\\s?care|medical|insurance|pension|parking|road|land|garden|tuition|tutor|lessons?|detective|investigator|eye|clinic|care|nursery|swimming|coach|hire|contract|members?|club|firm|company|jet|plane|island|beach)\\b|\\bshared\\s+(?:ownership|owner|equity|driveway|garden|house|home|flat|room|kitchen|bathroom|car|bike|account|login|costs?|bills?|custody|care|space|office)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRIVACY_SHAPES(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was)\\s+(?:this|the|our)\\s+(?:a\\s+|an\\s+)?(?:private|confidential|secure)\\s+(?:chat|conversation|call|line|session|thread)\\b|\\b(?:is|are|was|will|would)\\s+(?:this|our|the)\\s+(?:chat|conversation|call|line|session|thread)\\s+(?:going\\s+to\\s+)?(?:be\\s+|stay\\s+|stays\\s+|get\\s+|gets\\s+)?(?:private|confidential|secure|saved|recorded|stored|logged|kept|seen|read|shared)\\b|\\bwill\\s+(?:this|it)\\s+(?:be|stay|get)\\s+(?:private|confidential|recorded|saved|stored|logged|kept)\\b|\\bis\\s+(?:this|it)\\s+going\\s+to\\s+(?:be\\s+|stay\\s+|get\\s+)?(?:private|confidential|recorded|saved|stored|logged|kept)\\b|\\b(?:do|does|will|would|are|can|could)\\s+(?:you|u)\\s+(?:be\\s+)?(?:keep|keeping|save|saving|record|recording|store|storing|log|logging)\\s+(?:this|it|what\\s+i\\s+say)(?=\\s*[?.!]*\\s*$)|\\bwho\\s+(?:sees|reads|hears|can\\s+see|can\\s+read|can\\s+access)\\s+this(?:\\s+(?:chat|conversation|thread))?(?=\\s*[?.!]*\\s*$)|\\b(?:is|are)\\s+(?:my|our)\\s+(?:data|details|info|information|messages?|words)\\s+(?:private|confidential)\\b|\\b(?:is|are)\\s+(?:my|our)\\s+(?:data|details|info|information)\\s+(?:safe|secure)(?:\\s+(?:with|on|in)\\s+(?:you|here|this\\s+chat)\\b)?(?=\\s*[?.!]*\\s*$)|\\b(?:is|are)\\s+(?:my|our)\\s+(?:data|details|info|information|messages?|words)\\s+(?:safe|secure)\\s+(?:with\\s+(?:you|u)|here)\\b|\\bwhat\\s+(?:happens|happened|will\\s+happen|would\\s+happen)\\s+to\\s+(?:what\\s+i\\s+(?:say|said|tell|type|write|wrote)\\b|my\\s+(?:messages?|data|words|details|information|info|chats?)\\b|our\\s+(?:chat|conversation)\\b)(?=\\s*[?.!]*\\s*$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_ASK(py_str("(?:times|timetables?|schedules?|numbers?|phone|telephone|address|postcode|postcodes?|prices?|costs?|fares?|deals?|offers?|reviews?|ratings?|scores?|results?|news|flights?|trains?|buses?|coaches|tickets?|availability|stock|opening\\s+hours|weather|forecast|vacanc(?:y|ies)|listings?|status|delays?|traffic|menu|lead\\s+times?|deadlines?|exchange\\s+rates?)"));
    s_ONLINE_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:can|could|do|does|are|is|have)\\s+(?:you|u)\\s+(?:[a-z]+\\s+){0,3}?(?:browse|search|google|look[\\s-]+(?:things|stuff|it|that|them)?\\s*up|check|access|reach|connect(?:ed)?|go|get)\\s*(?:the\\s+|a\\s+)?(?:internet|web|online|net|google|sites?|websites?|pages?)\\b|\\b(?:can|could|do|does|are|is|have)\\s+(?:you|u)\\s+(?:[a-z]+\\s+){0,3}?(?:online|connected|internet|web|internet\\s+access)\\b|\\bdo\\s+(?:you|u)\\s+have\\s+(?:[a-z]+\\s+){0,2}?(?:access|internet|wifi)\\b|\\bdo\\s+(?:you|u)\\s+have\\s+(?:[a-z]+\\s+){0,2}?connection\\s+(?:to|with)\\s+(?:the\\s+)?(?:internet|web|net|online)\\b|\\b(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+)?(?:google|search\\b(?=\\s*(?:$|for\\s+(?:me|us)\\b|around\\b|(?:it|that|this|things|stuff|something)\\b|online\\b|the\\s+(?:web|internet|net)\\b|sites?\\b|pages?\\b|websites?\\b))|look\\s+(?:it|that|this|things|stuff)\\s*up)\\b|\\b(?:google|search|look\\s+(?:\\w+\\s+){0,2}?up)\\b[^.?!]{0,30}?\\b"), &g_LIVE_ASK()), &py_str("\\b[^.?!]{0,30}?\\bfor\\s+(?:me|us)\\b\\s*(?:(?:please|then|now|thanks|thank\\s+you)\\s*)*[.?!]*\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WAY_OF_BEING(stdlib_call("re", "compile", &[py_str("\\b(?:lying|liar|lies|telling (?:a )?lie|telling lies|telling the truth|secrets?|keeping a secret|cheating|stealing|being (?:selfish|mean|kind|nice|honest|patient|cruel|brave|open|vulnerable|present)|selfish|unkind|cruel|mean to (?:people|him|her|them|anyone)|grudges?|holding a grudge|forgiving|apologis?ing|trust\\w*|vulnerable|opening up|compliments?|taking help|accepting (?:help|praise|a compliment)|saying (?:thank you|thanks)|thanking (?:people|someone|him|her|them)|asking for (?:help|things|a favour|favours|what i want)|asking (?:people|someone|him|her|them) for|keeping my mouth shut|biting my tongue|holding my tongue|keeping quiet|personally|taking (?:a )?criticism|taking feedback|goodbyes?|saying goodbye|relaxing|resting|switching off|doing nothing|not working|chilling out|chill out|chilling|slowing down|taking (?:a )?breaks?|taking time off|spending money)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEFT_BEHIND(stdlib_call("re", "compile", &[py_str("\\b(?:left|leaving|leaves|moved|moving)\\b[^.?!,;]{0,24}\\bhome\\b|\\b(?:left|leaving|leaves|went|going|gone|goes|go|off|moved|moving|heading|headed)\\b[^.?!,;]{0,24}\\b(?:away|abroad|overseas|out of the house)\\b|\\b(?:moving|moved|relocat\\w+|emigrat\\w+)\\s+(?:away|abroad|overseas)\\b|\\b(?:went|going|gone|goes|go|off|left|leaving|leaves|starts?|starting|started)\\s+to\\s+(?:uni|university|college)\\b|\\b(?:starts?|starting|started)\\s+(?:uni|university|college)\\b|\\b(?:left|leaves|leaving|went|going|gone|goes|off|off\\s+to)\\s+(?:for\\s+)?(?:uni|university|college)\\b|\\b(?:moved|moving|moves|relocated|relocating|relocates|emigrated|emigrating|emigrates)\\s+to\\s+(?!a\\b|an\\b|the\\b|another\\b|his\\b|her\\b|their\\b|our\\b|my\\b|that\\b|this\\b)[a-z]|\\bempty\\s+nest\\b|\\b(?:fly|flying|flies|flew|flown)\\s+(?:the|their)\\s+nest\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LANDED(stdlib_call("re", "compile", &[py_str("\\b(?:(?:i|we|she|he|they)\\s+(?:finally\\s+|just\\s+)?heard\\s+back|(?:i|we)\\s+(?:finally\\s+|just\\s+)?heard\\s+from\\s+(?:them|the\\s+(?:hospital|school|college|uni|university|company|employer|agency|council|landlord|court|bank|dwp|clinic|surgery|job|firm|office))|(?:they|she|he|someone)\\s+got\\s+back\\s+to\\s+(?:me|us)|(?:i|we)\\s+(?:just\\s+)?got\\s+(?:the|an?|my|our|his|her|their)\\s+(?:(?:exam|exams|test|tests|mock|mocks|module|modules|gcse|gcses|a-level|alevel|driving|theory|coursework|term|final|first|second|scan|scans|biopsy|blood|bloods|x-?ray|labs?)\\s+){0,2}(?:answer|reply|verdict|results?|letter|email|marks?|scores?|grades?)(?:\\s+back)?(?!\\s*(?:up|down|wrong|off|mixed|lost|missing|confused)\\b)|(?:the\\s+)?(?:results?|verdict)\\s+(?:came|came out|is in|are in))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_DAY(py_dict(vec![(py_str("cold"), py_tuple(vec![py_str("Bit parky, isn't it. Extra layer on, and put the heating on an hour before you want it warm rather than when you're cold."), py_str("Cold one. Thick socks make more difference than people expect."), py_str("That sort of cold gets into the bones. Hot drink, something warm on, and don't sit still in one room too long."), py_str("Freezing out, isn't it. Shut the curtains at dusk and it holds the heat in.")])), (py_str("rain"), py_tuple(vec![py_str("Raining again. Good day for the kettle and something slow on the telly."), py_str("That sort of rain gets in everywhere. Coat rather than umbrella today, and leave the washing till tomorrow."), py_str("Wet one, then. Shoes stuffed with newspaper overnight dry out properly, if you get caught in it."), py_str("Never mind. It's a good excuse to do nothing much indoors.")])), (py_str("wind"), py_tuple(vec![py_str("Blowy out there. Anything light in the garden will be next door by teatime."), py_str("Wind like that takes doors with it. Worth propping the gate before it does."), py_str("That's a bin-day wind. Hold on to the wheel if you're driving in it."), py_str("Nasty wind, that. Good reason to stay in.")])), (py_str("hot"), py_tuple(vec![py_str("Hot one, isn't it. Curtains shut while the sun's on the windows really does keep it cooler in."), py_str("Too hot to do anything fast. Cold drink, water on the wrists, and save the jobs for this evening."), py_str("That kind of heat takes it out of you. Drink more than you think you need."), py_str("Boiling, isn't it. A bowl of ice in front of a fan does more than the fan on its own.")])), (py_str("grey"), py_tuple(vec![py_str("Grey one. Put the lamp on early rather than at dusk and the afternoon feels less long."), py_str("Grim out, isn't it. Nothing wrong with a quiet one indoors."), py_str("Miserable out. Good day for the job you've been putting off, or for none at all."), py_str("Dull one. It's the light more than the cold that gets people.")])), (py_str("fog"), py_tuple(vec![py_str("Foggy one. Lights on even in the day, and more room than you think you need."), py_str("Thick out there. Worth leaving a bit earlier while it's like this."), py_str("Fog like that slows everything down. Nothing wrong with a slow start.")])), (py_str("sun"), py_tuple(vec![py_str("Lovely, isn't it. Get out in it for a bit, even if it's only round the block."), py_str("Beautiful out. Worth taking your cup of tea outside."), py_str("That's the day for being outdoors. Suncream on if you're out past eleven."), py_str("Gorgeous. That sort of day makes everything easier.")])), (py_str("mild"), py_tuple(vec![py_str("No complaints then. Those are the days to be out in without thinking about it."), py_str("Mild one, that. Good day for the jobs you keep putting off, or for none at all."), py_str("Fair enough. The days that ask nothing of you are the ones worth using."), py_str("Not bad at all, then. Worth a walk while it is like this.")]))]));
    s_THE_DAY_AHEAD(py_dict(vec![(py_str("cold"), py_tuple(vec![py_str("Cold one coming. Anything outside that hates frost is worth bringing in tonight."), py_str("Meant to turn cold. Get the grit out and the car warmed before you need either."), py_str("Cold forecast. Layers for the morning, and leave the heating on a timer.")])), (py_str("rain"), py_tuple(vec![py_str("Rain coming. Anything you want dry is worth doing before it arrives."), py_str("Wet forecast. Coat rather than umbrella tomorrow, because the wind comes with it."), py_str("Rain later, then. Put the outside bit of the day in the morning if you can.")])), (py_str("wind"), py_tuple(vec![py_str("Wind coming. Anything light in the garden will be next door by tomorrow night."), py_str("Blowy forecast. Worth checking the gate and the bins before it gets going."), py_str("Wind later, then. Anything loose up high is worth sorting while it is calm.")])), (py_str("mild"), py_tuple(vec![py_str("Mild forecast. Nothing to plan around, which is the best kind of day."), py_str("Nothing much coming, weather-wise. Good day to have something booked."), py_str("Fair forecast. Worth putting the outside thing in while it holds.")])), (py_str("hot"), py_tuple(vec![py_str("Heat coming. Curtains shut in the morning really does keep it cooler in."), py_str("Hot forecast. Jobs in the morning, and the afternoon for nothing much."), py_str("Meant to be hot. Worth getting the fan down before you need it.")])), (py_str("grey"), py_tuple(vec![py_str("Grey forecast. Nothing much to plan around, unless you were counting on the light."), py_str("Meant to be dull. Fine day for the job you have been putting off indoors."), py_str("Grey one coming. Put the lamp on early and the afternoon feels less long.")])), (py_str("fog"), py_tuple(vec![py_str("Fog forecast. Lights on in the morning and more room than you would leave."), py_str("Meant to be foggy. Worth setting off ten minutes earlier while it lifts."), py_str("Fog coming. Anything you were going to drive for is easier after it clears.")])), (py_str("sun"), py_tuple(vec![py_str("Nice one coming. Worth saving the outdoor bit of the day for it."), py_str("Sun forecast. Suncream in the bag if you are out past eleven."), py_str("Meant to be lovely. That is the day for the thing you keep putting off.")]))]));
    s_HEAVY_NEWS(stdlib_call("re", "compile", &[py_str("\\b(?:left\\s+me\\b(?!\\s+(?:a|an|the|my|some|this|that|no)\\b)|walked\\s+out(?:\\s+on\\s+me)?\\W*$|dumped\\s+me|kicked\\s+me\\s+out|threw\\s+me\\s+out|wants?\\s+a\\s+divorce|(?:is|are|was|were|been|kept|taken|rushed|put|now|still)\\s+(?:in|into)\\s+(?:the\\s+)?hospital|hospitali[sz]ed|in\\s+(?:the\\s+)?icu|intensive\\s+care|losing\\s+my\\s+(?:sight|hearing|memory|voice|hair|leg|arm)|gone\\s+blind|going\\s+blind|going\\s+deaf|lost\\s+my\\s+(?:sight|hearing)|(?:the|my)\\s+scan\\s+(?:came\\s+back|shows?|showed|is\\s+back)|scan\\s+came\\s+back|results?\\s+came\\s+back|diagnosed\\s+with|being\\s+treated\\s+for|starting\\s+(?:chemo|treatment|dialysis)|lost\\s+my\\s+(?:job|work|hours)|laid\\s+off|made\\s+redundant|been\\s+(?:sacked|fired)|got\\s+(?:sacked|fired|let\\s+go)|sacked\\s+(?:from|on)|let\\s+go\\s+from|wait(?:ing)?\\s+(?:\\w+\\s+){0,4}(?:weeks|months)|on\\s+a\\s+waiting\\s+list|months?\\s+for\\s+(?:the\\s+|an\\s+|a\\s+)?(?:appointment|scan|referral|operation)|panic\\s+attack|anxiety\\s+attack|having\\s+(?:a\\s+)?panic|my\\s+anxiety\\s+is|(?:i'?m|im|feel|feeling|been\\s+feeling)\\s+(?:really\\s+|so\\s+|very\\s+|always\\s+|constantly\\s+)?anxious|so\\s+anxious|sectioned|gone\\s+into\\s+care)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABSENCE(stdlib_call("re", "compile", &[py_str("\\b(?:went|gone|been|be)\\s+(?:quiet|silent|away|incommunicado|off(?!\\s+(?:the\\s+)?(?:grid|radar|map)))\\b|\\b(?:i|we)(?:'?ve|'?d)?\\s+(?:have\\s+|had\\s+|been\\s+)?(?:gone|disappeared|vanished|missing|quiet|silent)\\b|\\bfor\\s+(?:disappearing|vanishing|going quiet|being quiet|being away|being gone)\\b|\\b(?:the|a)\\s+silence\\b|\\bsilent\\s+(?:for|since)\\b|\\b(?:havent|haven'?t|hadnt|hadn'?t|didnt|didn'?t|not|never)\\s+(?:replied|reply|replying|written|write|writing|answered|answering|been in touch)\\b|\\b(?:missed|saw|seeing)\\s+(?:your|that)\\s+(?:message|msg|reply|text|email|call)\\b|\\b(?:slow|late)\\s+(?:reply|response)\\b|\\b(?:mad|really|very|so|proper|dead)?\\s*busy\\b|\\b(?:fell|been|was)\\s+(?:fast\\s+)?asleep\\b|\\blost\\s+track(?:\\s+of\\s+(?:time|the\\s+(?:time|thread|plot)))?\\b|\\bphone\\s+(?:died|went dead|was dead)\\b|\\b(?:been|be|went|gone|was)\\s+(?:off|away\\s+from)\\s+(?:my|the|your|his|her)?\\s*(?:phone|mobile|texts|messages|emails|email|inbox)\\b|\\b(?:only\\s+)?just\\s+saw\\s+(?:this|that|your\\s+(?:message|msg|text|email|reply))\\b|\\b(?:was|were|been|be)\\s+on\\s+(?:my|the|your|his|her)\\s*(?:phone|mobile)\\b|\\b(?:been|be|went|gone|was)\\s+off\\s+(?:the\\s+)?(?:grid|radar|map)\\b|\\b(?:keep|kept)\\s+you\\s+waiting\\b|\\bback\\s+(?:now|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_PLACES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("doctors"), py_str("doctor"), py_str("gp"), py_str("surgery"), py_str("dentist"), py_str("dental"), py_str("hospital"), py_str("clinic"), py_str("pharmacy"), py_str("chemist"), py_str("optician"), py_str("midwife"), py_str("health visitor")]), py_tuple(vec![py_str("I can't hold phone numbers, sorry - they change, and a wrong one costs you a morning. The practice's own page on a map app has it, it is on the letter or text they sent you, and the NHS app lists it too."), py_str("No directory at this end, so I would only be guessing at the digits. The letter with your appointment on it has the number, and the practice's page has it if that has gone missing.")])]), py_tuple(vec![py_tuple(vec![py_str("council"), py_str("housing association"), py_str("housing office"), py_str("tip"), py_str("recycling centre"), py_str("dump"), py_str("library"), py_str("leisure centre"), py_str("parking permit"), py_str("parking office"), py_str("bins")]), py_tuple(vec![py_str("I can't look a phone number up, sorry - a wrong one sends you round in circles. The council's site has a contact page with the number for the right department, and it is on the bill they last sent you."), py_str("Nothing here that can find a number, so I won't guess at one. Your council tax bill, or the council's own contact page, has the number for the department you need.")])]), py_tuple(vec![py_tuple(vec![py_str("bank"), py_str("building society"), py_str("credit card"), py_str("insurer"), py_str("insurance"), py_str("mortgage")]), py_tuple(vec![py_str("I can't hold phone numbers, sorry. A bank's number is on the back of your card and in its app; an insurer's is on the policy letter, and both are faster than a search."), py_str("No number that I can give you - they change and I have no way to check. The back of the card, or the policy letter if it is insurance, has the one that gets answered.")])]), py_tuple(vec![py_tuple(vec![py_str("water company"), py_str("water board"), py_str("energy supplier"), py_str("energy company"), py_str("electricity"), py_str("gas company"), py_str("broadband"), py_str("phone company"), py_str("supplier"), py_str("utility")]), py_tuple(vec![py_str("I can't look up a supplier's number, sorry. It is on the bill, in their app, and usually at the top of their website - and the bill carries your account number, which they ask for first.")])]), py_tuple(vec![py_tuple(vec![py_str("dvla"), py_str("passport office"), py_str("tax office"), py_str("hmrc"), py_str("jobcentre"), py_str("job centre"), py_str("benefits"), py_str("pension"), py_str("dwp")]), py_tuple(vec![py_str("I can't hold a government number, sorry - they change with the department. Search gov.uk for the service and the page for it has the number and the hours the lines are open.")])]), py_tuple(vec![py_tuple(vec![py_str("school"), py_str("nursery"), py_str("childminder"), py_str("college"), py_str("university")]), py_tuple(vec![py_str("I can't look a school's number up, sorry. The office number is on the letter or email they sent you, and on the school's own site under Contact.")])]), py_tuple(vec![py_tuple(vec![py_str("vet"), py_str("vets"), py_str("veterinary"), py_str("animal hospital")]), py_tuple(vec![py_str("I can't hold phone numbers, sorry. The practice's page, or the card they gave you, has it - and save their out-of-hours number while you are looking, since that is the one nobody can find at ten at night.")])]), py_tuple(vec![py_tuple(vec![py_str("takeaway"), py_str("restaurant"), py_str("chippy"), py_str("pizza"), py_str("plumber"), py_str("electrician"), py_str("garage"), py_str("taxi"), py_str("minicab"), py_str("hairdresser"), py_str("barber"), py_str("gym"), py_str("pub"), py_str("hotel"), py_str("cafe")]), py_tuple(vec![py_str("I can't hold a number like that, sorry - a place's number changes and I have no way to look it up. A map app shows it with the opening times, and it is on the card, menu or door the place gave you.")])])]));
    s_WORDING_SOMEONE(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|do|can|could|would)\\s+i\\s+(?:say|mention)\\s+(?:something|anything)\\s+to\\b|\\bhow\\s+(?:do|should|can|could)\\s+i\\s+bring\\s+(?:it|this|that)\\s+up\\b|\\b(?:have|has|having)\\s+a\\s+(?:quick\\s+|quiet\\s+|bit\\s+of\\s+a\\s+)?word\\s+with\\b|\\bwhat\\s+(?:would|will)\\s+you\\s+say\\s+to\\s+(?:him|her|them|my|your|the)\\b|\\bwhat\\s+(?:do|should|can|could)\\s+i\\s+(?:actually\\s+|even\\s+)?say\\s+to\\s+(?:him|her|them|people)\\b(?!\\s+(?:if|whether)\\b)|\\bhow\\s+(?:do|should|can|could)\\s+i\\s+(?:raise|broach|word|phrase|put)\\s+(?:it|this|that|the\\s+subject|the\\s+issue)\\b|\\bhow\\s+(?:do|should|can|could)\\s+i\\s+break\\s+(?:it|this|that|the\\s+news)\\s+to\\b|\\bhow\\s+(?:do|should|can|could)\\s+i\\s+break\\s+the\\s+news\\b|\\bbreak\\s+the\\s+news\\s+to\\b|\\bwithout\\s+(?:sounding|coming\\s+across)\\b|\\bnot\\s+(?:sounding|coming\\s+across)\\s+like\\b|\\b(?:don'?t|do\\s+not)\\s+want\\s+to\\s+(?:sound|come\\s+across)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT_PROBLEM_ACT(stdlib_call("re", "compile", &[py_str("(?:interrupt|snap|shout|yell|argue|fight|nag|criticis|criticiz|blame|apologi[sz]e|put\\s+off|puts\\s+off|putting\\s+off|procrastinat|leav(?:e|es|ing)\\s+(?:things|it|everything|jobs|the\\s+lot)|apologi[sz](?:e|es|ed|ing)|ask(?:s|ing)?\\s+for\\s+help|never\\s+manage|manages\\s+to|sav(?:e|es|ing)\\s+(?:anything|any\\s+money)|put(?:s|ting)?\\s+(?:everyone|everybody|others|people)\\s+(?:else\\s+)?first|text(?:s|ing)?\\s+first|get(?:s|ting)?\\s+round\\s+to|go(?:es|ing)?\\s+blank|finish|finishes|finishing|forget|forgets|forgetting|forgot|lose|loses|losing|cancel|cancels|cancelling|canceling|avoid|avoids|avoiding|overthink|overthinks|overthinking|second-guess|second\\s+guess|worry|worries|worrying|complain|complains|complaining|moan|moans|moaning|compare|compares|comparing|hide|hides|hiding|shut|shuts|shutting|withdraw|withdraws|withdrawing|run\\s+away|running\\s+away|give\\s+up|giving\\s+up|give\\s+in|giving\\s+in|stay\\s+up|stays\\s+up|staying\\s+up|work\\s+(?:late|too\\s+hard|too\\s+much)|working\\s+(?:late|too\\s+hard|too\\s+much)|drink\\s+too|drinks\\s+too|drinking\\s+too|smoke|smokes|smoking|spend|spends|spending|gamble|gambles|gambling|eat\\s+too|eats\\s+too|eating\\s+too|eat\\s+at\\s+night|binge|binges|bingeing|starve|starves|starving|skip|skips|skipping|lie|lies|lying|mean\\s+to|means\\s+to|meaning\\s+to|say(?:ing|s)?\\s+yes|say(?:ing|s)?\\s+sorry|check|checks|checking|scroll|scrolls|scrolling|pick|picks|picking|bite|bites|biting|turn\\s+up\\s+late|turning\\s+up\\s+late|arrive\\s+late|arriving\\s+late|end\\s+up|ends\\s+up|ending\\s+up|can'?t\\s+stop|cant\\s+stop)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HURT_EVENT(stdlib_call("re", "compile", &[py_str("\\b(?:got|gets|getting|was|were|is|are|been|had|has|keeps?)\\s+(?:badly|seriously|really|terribly)\\s+(?:hurt|injured|burned|burnt|scalded)\\b|\\b(?:hurt|injured|burned|burnt|scalded)\\s+(?:badly|seriously)\\b|\\b(?:admitted|taken|rushed|airlifted|kept)\\s+(?:in\\s+)?to\\s+(?:hospital|a\\s*&\\s*e|intensive\\s+care)\\b|\\bin\\s+(?:intensive\\s+care|a\\s*&\\s*e|hospital)\\b|\\b(?:got|gets|getting|was|were|is|are|been|had|has|keeps?)\\s+(?:nearly\\s+|almost\\s+)?(?:attacked|bitten|bit|punched|stabbed|kicked|mugged|assaulted|strangled|electrocuted)\\b|\\b(?:attacked|punched|stabbed|assaulted|mugged)\\b|\\bbeaten\\s+up\\b|\\b(?:got|was|were|been|gets|is|are)\\s+beaten\\s+up\\b|\\bhit\\s+by\\s+(?:a|an|the)\\s+(?:\\w+\\s+)?(?:car|bus|bike|bicycle|van|lorry|truck|train|tram|taxi|motorbike|driver)\\b|\\bhit\\s+(?:in\\s+the\\s+(?:face|head|eye)|with\\s+a\\s+(?:brick|bottle|bat|knife))\\b|\\bknocked\\s+(?:over|down)\\b|\\b(?:drowned|shot)\\b(?=\\s+(?:in|at|by)\\b)|\\bpushed\\s+(?:down|over|off|in\\s+front)\\b|\\bpushed\\s+into\\s+the\\s+(?:road|traffic|water)\\b|\\bthrown\\s+(?:from|off|out\\s+of)\\b|\\brun\\s+over\\s+by\\s+(?:a|an|the)\\s+(?:\\w+\\s+)?(?:car|bus|bike|bicycle|van|lorry|truck|train|tram|taxi|motorbike)\\b|\\b(?:nearly|almost)\\s+drowned\\b|\\bwas\\s+robbed\\b(?!\\s+of\\b)|\\brobbed\\s+(?:at|in|on|outside)\\b|\\bcut\\s+(?:his|her|their)\\s+\\w+\\s+(?:badly|open)\\b|\\b(?:fell|fallen|collapsed)\\b|\\b(?:was|were|is|are|been)\\s+in\\s+(?:a|an)\\s+(?:car\\s+|road\\s+|motor\\s+|bike\\s+|cycling\\s+)?accident\\b|\\b(?:car|road|motor|bike|bicycle|traffic)\\s+(?:crash|accident|collision)\\b|\\b(?:crash|collision|ambulance|casualty)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WOULD_HAVE_BEEN(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:would|should|will|'?d) ?(?:have|'?ve) been\\b[^.?!]{0,24}?\\b(?:my|our|her|his|their|mum'?s|dad'?s|his|the)\\b[^.?!]{0,12}\\b(?:"), &g_MILESTONE_EVENT()), &py_str(")\\b|\\b(?:")), &g_MILESTONE_EVENT()), &py_str(")\\b[^.?!]{0,16}\\b(?:would|should) ?(?:have|'?ve) been\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATES_A_SKILL(stdlib_call("re", "compile", &[py_str("\\b(?:useless|rubbish|crap|hopeless|no good|bad|poor|terrible|awful)\\s+(?:at|with|in)\\s+(?!(?:everything|anything|life|myself|it all|all of it)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HEDGED_RATING(py_str("(?:i (?:don'?t|do not|didn'?t|did not) think (?:i'?m|i am)|i doubt (?:i'?m|i am)|(?:i'?m|i am) not sure (?:i'?m|i am)|not sure (?:i'?m|i am))"));
    s_I_RATE_A_SKILL(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &g_A_HEDGED_RATING()), &py_str("\\s+(?:very |so |that |really )?(?:useless|rubbish|crap|crappy|hopeless|no good|bad|poor|terrible|awful|dreadful|lousy|any good|much good|good|great|decent|brilliant|not (?:very |that |much |so )?good|not great|not brilliant)|(?:i'?m|i am|im|i feel like i'?m|i think i'?m|i know i'?m)\\s+(?:just\\s+|so\\s+|really\\s+|pretty\\s+|a bit\\s+|kinda\\s+|kind of\\s+|quite\\s+|absolutely\\s+|completely\\s+|totally\\s+|proper\\s+|still\\s+|also\\s+|always\\s+)*(?:useless|rubbish|crap|crappy|hopeless|no good|bad|poor|terrible|awful|dreadful|lousy|not (?:very |that |much |so )?good|not great|not brilliant))\\s+(?:at|with|in)\\s+(?!(?:everything|anything|life|myself|it all|all of it|this|that|the moment|the minute|the mo\\b|the present|the time being|the best of times|night|nights|bedtime|daytime|evenings?|mornings?|weekends?|the weekend|now|these days|times like this)\\b)"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SKILL_ASKED_OF_ME(stdlib_call("re", "compile", &[py_str("\\bam i\\s+(?:actually\\s+|really\\s+|even\\s+|still\\s+|any\\s+|much\\s+)?(?:any\\s+good|much\\s+good|no\\s+good|good\\s+enough|good|great|decent|brilliant|bad|rubbish|crap|crappy|hopeless|useless|terrible|awful)\\s+(?:at|with|in)\\s+\\w")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOODNESS_ASKED_OF_ME(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bam i\\s+(?:(?:actually|really|even|still|such)\\s+|(?:a\\s+bit)\\s+)?(?:(?:a|an)\\s+)?(?:good|bad|terrible|rubbish|awful|horrible|decent|great|useless|hopeless)\\s+"), &g_SELF_JOB()), &py_str("\\b|\\bam i\\s+(?:actually\\s+|really\\s+|even\\s+|still\\s+)?(?:good\\s+enough|any\\s+good|much\\s+good)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_SUBJECT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:i'm|i am|im|i feel like|i feel|i keep|i've been|ive been|i am being|i was|i always)\\b(?:\\s+\\w+){0,3}\\s+"), &g_SELF_JUDGEMENT()), &py_str("\\b|\\b(?:i'm|i am|im|i feel like|i feel)\\b(?:\\s+\\w+){0,2}\\s+")), &g_SELF_NOUN()), &py_str("\\b|\\bi (?:feel like|feel|am|'m|m) (?:such )?(?:a |an |the )?(?:rubbish|bad|terrible|awful|useless|hopeless|shit|useless) ")), &g_SELF_JOB()), &py_str("\\b|\\b(?:i (?:don'?t|do not|didn'?t|did not) think (?:i'?m|i am)|i doubt (?:i'?m|i am)) (?:such )?(?:a |an |the )?(?:not )?(?:(?:very|really|that) )?(?:good|great|nice|kind|decent|much of a) ")), &g_SELF_JOB()), &py_str("\\b|\\b(?:i'?m|im|i am|i feel like i'?m) (?:such )?(?:a |an |the )?(?:not (?:a |an )?(?:(?:very|really|that) )?(?:good|great|nice|kind|decent)|(?:not )?much of a) ")), &g_SELF_JOB()), &py_str("\\b|\\bi (?:always|constantly|just|keep)\\s+(?:mess(?:es|ing)? (?:(?:things|it|everything) )?up(?! (?:the|my|your|his|her|their|our|a|an)\\b)|ruin(?:s|ing|ed)? (?:things|it|everything)|get(?:s|ting)? (?:(?:things|it|everything) )?wrong|screw(?:s|ing)? (?:things|it|everything) up|let(?:s|ting)? (?:people|everyone|them|her|him) down)\\b|\\bwhat (?:an? )?(?:idiot|muppet|moron|fool|disaster) i (?:am|was|have been)\\b|\\b(?:cant|can't|couldnt|couldn't|never)\\s+(?:do|get|seem to do)\\s+anything\\s+right\\b|\\bi hate (?:myself|my own\\b)|\\bnothing i (?:do|try|say|make)\\b[^.]{0,24}\\b(?:right|good|enough|works|matters)\\b|\\bam i (?:a failure|useless|worthless|the problem|an idiot|such an idiot)\\b")), &g_SELF_NOBODY())]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_BLAME_ASKS(stdlib_call("re", "compile", &[py_str("\\b(?:how (?:do|can|should|would) i|what (?:should|can|could|do|shall) i|should i\\b|who (?:can|could|should|do) i|any (?:tips|advice|ideas)|can you help|help me|where do i start|what would you do|suggest)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EMBARRASSMENT_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:embarrass(?:ed|ing|ingly|ment)|mortified|mortifying|cringey?|cringeworthy|cringe|humiliat(?:ed|ing)|blush(?:ed|ing)|went (?:bright )?red|face went red)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_GROUND_OPENS(stdlib_call("re", "compile", &[py_str("\\b(?:ground (?:to )?(?:open up and )?swallow|swallow me up|sink through the floor|wish(?:ed)? the floor|floor (?:to )?swallow)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DIE_OF_IT(stdlib_call("re", "compile", &[py_str("\\b(?:die|dying|died|dead)\\b[^.?!]{0,10}\\b(?:of|from)\\b[^.?!]{0,6}\\b(?:embarrassment|embarrassing|shame|cringe)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_TO_DIE(stdlib_call("re", "compile", &[py_str("\\b(?:going|gonna|gunna) to (?:just )?die\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSES_SHAME(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our|your) \\w+(?: \\w+)? (?:is|was|were|can be|can get)\\s+(?:\\w+\\s+){0,2}(?:embarrass|mortif|cring|humiliat)\\w*")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VERDICT_ON_THE_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:i'm|im|i am)\\s+(?:such\\s+|a bit of\\s+|a\\s+|an\\s+|the\\s+|so\\s+|really\\s+)*(?:embarrassment|disgrace)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_MOMENT_IS_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:i|i'm|im)\\s+(?:was\\s+|am\\s+|felt\\s+|feel\\s+|look\\s+|looked\\s+|were\\s+|got\\s+)?(?:so\\s+|really\\s+|very\\s+|proper\\s+|a bit\\s+|totally\\s+|completely\\s+|absolutely\\s+)?(?:embarrass\\w*|mortified|mortifying|cring\\w*|humiliat\\w*|blush(?:ed|ing)|(?:went|turned) (?:bright )?red)\\b|\\b(?:that|this|it) (?:was|is|were)\\s+(?:\\w+\\s+){0,2}(?:embarrass\\w*|mortifying|cringe\\w*|humiliating|mortified)\\b|\\b(?:so|how|very|really|proper|dead|well) embarrass\\w*\\b|\\bi (?:cringe|cringed)\\b|\\b(?:so|dead|proper|a bit of a) cringe\\b|\\bthink(?:ing)? about it\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SLIP(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|i've|ive)\\s+(?:\\w+\\s+){0,3}(?:tripped|slipped|spilled|spilt|dropped|knocked (?:it |that |the \\w+ |my \\w+ |a \\w+ )?over|walked (?:into|straight into)|banged into|bumped into|called (?:\\w+\\s+){1,3}(?:mum|mom|dad|nan|mate|sir|miss)|waved at|sat on|stepped in|forgot (?:my|the) (?:lines|words|name|card)|got (?:\\w+\\s+){0,2}name wrong|said the wrong \\w+|sent (?:\\w+ ){0,2}to the wrong|wore (?:\\w+ ){0,3}(?:inside out|back to front)|made (?:a fool|an idiot|a show|a tit|a right fool) of myself|farted|burped|sneezed loudly|had (?:\\w+ ){0,2}(?:in my teeth|something in my teeth))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SOCIAL_SLIP(stdlib_call("re", "compile", &[py_str("\\b(?:said hi|said hello|nodded|waved) (?:at |to )?(?:someone|somebody|a (?:man|woman|lady|bloke|guy|lad|person|stranger))\\b|\\bwaved at (?:someone|somebody|a (?:man|woman|lady|bloke|guy|person|stranger))\\b|\\bcalled (?:\\w+\\s+){1,3}(?:mum|mom|dad|nan|mate|sir|miss)\\b|\\bmeant to (?:text|email|ring|reply to|send \\w+ to) (?:\\w+ ){0,3}and (?:texted|emailed|rang|replied to|sent)\\b|\\b(?:everyone|everybody|the (?:whole )?(?:room|office|class|pub|bus))\\s+(?:went quiet|fell silent|stopped talking|stared|turned (?:round|around)|looked (?:at me|round|over))\\b|\\b(?:my )?(?:flies|fly|zip|trousers|dress) (?:were|was|is)\\s+(?:open|down|undone)\\b|\\bhad (?:\\w+ ){0,3}(?:in my teeth|on my face|stuck to my (?:shoe|back))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SLIP_IS_NOT_A_FALL(stdlib_call("re", "compile", &[py_str("\\b(?:fell|fall|broke|broken|hurt|hurts|bleeding|twisted|sprained|cut|banged my|concussion|a&e|hospital|999|111|doctor|nurse)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_A_ROOM(stdlib_call("re", "compile", &[py_str("\\b(?:in front of|everyone|everybody|all of them|they all|the whole (?:office|class|team|room|pub|cafe|shop|bus|train)|in the (?:meeting|office|canteen|pub|shop|room|queue|hall)|on stage|at work|at the (?:party|wedding)|in (?:public|class)|the school gate|staff room|the till|the counter|down my (?:top|front|shirt|dress|jeans)|on my (?:shirt|top|front|trousers)|all (?:day|morning|afternoon|evening)|the whole (?:day|morning|afternoon|evening)|at (?:dinner|lunch|breakfast|the table)|around (?:everyone|everybody)|they (?:laughed|all laughed|were laughing|giggled|were giggling))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAUGHED_AT_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:laugh(?:s|ed|ing)?|giggl(?:e|es|ed|ing)|snigger\\w*|snicker\\w*|titter\\w*)\\s+(?:at|about)\\s+(?:me|us|my|our)\\b|\\b(?:laugh(?:s|ed|ing)?|giggl(?:e|es|ed|ing))\\s+(?:when|after|because)\\s+i\\b|\\b(?:nobody|no one|no-one)\\s+(?:laughed|laughing|laugh)\\b|\\b(?:everyone|everybody|they all|all of them|the (?:others|rest|group|kids|class|crowd))\\b(?:\\s+[\\w']+){0,3}\\s+(?:laughed|laugh|laughing|giggled|giggling)\\b|\\b(?:the|my) (?:whole|entire) (?:\\w+ ){0,2}\\w+ (?:laughed|were laughing|giggled|were giggling)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CROWD_LAUGHED(stdlib_call("re", "compile", &[py_str("\\b(?:everyone|everybody|they all|all of them|the (?:others|rest|group|kids|class|crowd|whole|entire|room|office))\\b(?:\\s+[\\w']+){0,3}\\s+(?:laughed|laugh|laughing|giggled|giggling)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_LAUGHED(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our|your)\\s+\\w+(?:\\s+\\w+)?\\s+(?:laughed|laughs|giggled|sniggered|snickered|sneered|smirked)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAUGHED_AT(py_tuple(vec![py_str("Ugh, that is a horrid feeling. What happened?"), py_str("That stings, even when it was meant lightly. What happened?"), py_str("It never feels that way on the day, but they will have forgotten it by tomorrow. What happened?"), py_str("Ouch. What was the moment?"), py_str("That is the sort of thing that says more about the room than about you. Tell me what happened.")]));
    s_EMBARRASSED(py_tuple(vec![py_str("Oh no - the kind of thing you replay in your head at 3am. What happened?"), py_str("Ha, we have all been there. What happened?"), py_str("That would make anyone want the floor to open up. What happened?"), py_str("Ouch. Did anyone else see it?"), py_str("That is a minute you will be telling people about in a year. What happened?")]));
    s_THE_PLACE_POOLS(py_tuple(vec![py_tuple(vec![py_str("\\bmy (?:account|balance|bill|statement|invoice|order|parcel|delivery|meter|direct debit|tariff|contract|policy|claim|application|booking|reservation|ticket)s?\\b|\\bmy (?:bank|energy|water|broadband|mobile|phone) (?:account|bill|app)\\b"), py_tuple(vec![py_str("That is in your own account rather than anywhere I can see, sorry - your bill or your order there has the figure to the penny, and I'll help you work out what to do with it."), py_str("I can't see your account, sorry. Open the app or the last bill: the figure there is the one that counts, and tell me what it says and I'll help you work out where it came from."), py_str("Your own account has that, sorry - it is the only place with your numbers in it. What it says is worth reading against the last one, and I'll help you spot what moved.")])]), py_tuple(vec![py_str("\\bcouncil\\b|\\bpermit\\b|\\blicen[cs]e\\b|\\bplanning permission\\b|\\bbin (?:day|collection)|\\brecycling\\b|\\bcouncil tax\\b|\\bparking (?:permit|zone)\\b|\\bblue badge\\b|\\bpassport\\b|\\bdriving licence\\b|\\bDVLA\\b|\\bregister(?:ing)? to vote\\b"), py_tuple(vec![py_str("Rules like that are the council's or the department's own, sorry, and they differ by where you live - the page for your postcode has the current one, and I'll help you read it."), py_str("That one is set by the council or the department, so I can't say it from here, sorry. The letter or the site with your address on it is the one that counts; bring me what it says and I'll help you fill it in."), py_str("I can't call that one: it is their rule, not mine, and it changes. The council's own page (or the number on the letter) will say, and I'll help with the rest of it.")])]), py_tuple(vec![py_str("\\bwarrant(?:y|ies)\\b|\\bguarantee\\b|\\bunder guarantee\\b|\\breturn it\\b|\\brefund\\b|\\b(?:the|a|this|that) shop\\b[^?]{0,30}\\b(?:replace|replacement|refund|return|repair|fix)\\b|\\bstill cover(?:ed)?\\b|\\bout of warranty\\b"), py_tuple(vec![py_str("That is the maker's terms rather than mine, sorry - the date on the receipt is what they go by, and their warranty page says what is covered and for how long. Tell me what it says and I'll help you work out whether to claim or replace."), py_str("I can't see the terms from here, sorry. The receipt has the date and the maker's own page has the cover; together they answer it. Bring me both and I'll help you word the claim."), py_str("Cover like that is on the maker's page, sorry - the model number and the date of the receipt are what it turns on, and I'll help you with the claim once you know.")])]), py_tuple(vec![py_str("\\b(?:this|that|the) (?:model|brand|version|make|one)\\b|\\bmodel number\\b|\\bwhich model\\b|\\bthe manual\\b|\\binstructions\\b|\\bwarranty\\b|\\bguarantee\\b|\\bspare part\\b|\\bpart number\\b|\\bwhere can i buy\\b|\\bis it in stock\\b|\\bin stock\\b|\\bhow much (?:is|are) (?:a|an|the)\\b|\\bhow much (?:does|do) (?:it|this|that|one|they|a|an|the|my|your)\\b[^.!?]{0,24}?\\b(?:cost|costs|charge|charges|go for|goes for|come to|comes to|pay)\\b"), py_tuple(vec![py_str("The maker's own page has that, sorry - the model number on the label under it is what to put in, and it will give you the manual, the parts and the price. Tell me what it says and I'll help you work out whether it is worth it."), py_str("That is the retailer's or the maker's to say, sorry, and prices and stock move daily. The model number is the thing to search, and I'll help you weigh what you find against a repair."), py_str("I can't see the listing from here, sorry. With the model number off the label, the maker's page will have the manual and the parts, and I'll help you with the bit after that.")])]), py_tuple(vec![py_str("\\b(?:train|bus|flight|ferry|coach|tube|tram|plane)s?\\b.*\\b(?:time|times|platform|delayed|cancelled|strike|leaves?)\\b|\\bwhat time (?:does|do|is|will)\\b|\\btimetable\\b|\\blast (?:train|bus|entry|orders?)\\b|\\bopening (?:time|times|hours)\\b|\\bopen (?:until|till|late)\\b|\\bwhat(?:'s| is) on\\b|\\bticket (?:price|prices)\\b|\\bsold out\\b|\\bdoors open\\b|\\bthe venue\\b|\\bplatform\\b|\\bwhich stop\\b"), py_tuple(vec![py_str("That is live, sorry - timetables and listings change by the hour. The operator's or the venue's own app has the current one, today's included, and I'll help you plan around whatever it says."), py_str("I can't check live times, sorry. The operator's app (or the board at the stop) is the one that knows, and tell me the time and I'll work out the rest of the day with you."), py_str("Times like that I can't see, sorry. Put the journey into the operator's own app and it will show today's, and I'll help you with the bit either side of it.")])]), py_tuple(vec![py_str("\\b(?:the|my|a) (?:shop|store|garage|chemist|pharmacy|butcher|bakery|chippy|takeaway|restaurant|cafe|pub|salon|barber|garage|tip|dump|library|leisure centre|pool)\\b[^?]{0,60}?\\b(?:hours?|open|opens|opening|closed|closes?|shut|shuts|shutting|late|early|sell|sells|stock|stocks|in stock|got any|have they got|do they have|does it have|price|prices|cost|costs|cheap|much|phone|number|contact|where is|where's|nearest|which one)\\b|\\b(?:what time|when|how late|how much|is|are|does|do|will|can)\\b[^?]{0,40}?(?:the|my|a) (?:shop|store|garage|chemist|pharmacy|butcher|bakery|chippy|takeaway|restaurant|cafe|pub|salon|barber|tip|dump|library|leisure centre|pool)\\b[^?]{0,30}?\\b(?:open|opens|opening|closed|closes?|shut|shuts|hours?)\\b|\\b(?:what hours|what time|when|how late)\\b[^?]{0,45}?\\b(?:the|my|a) (?:shop|store|garage|chemist|pharmacy|butcher|bakery|chippy|takeaway|restaurant|cafe|pub|salon|barber|tip|dump|library|leisure centre|pool)\\b"), py_tuple(vec![py_str("That is the shop's own to say, sorry - its listing on a map app shows today's hours and its number, holidays included, and I'll help you with what to ask when you ring."), py_str("I can't see their opening hours from here, sorry. A map app's listing for the branch is usually right about today, and ringing takes a minute; tell me what they say and I'll help with the next step."), py_str("Their hours are on their own listing, sorry - search the branch by name on a map app and it shows today's. I'll help with the rest once you know.")])]), py_tuple(vec![py_str("\\b(?:medicine|medication|tablet|tablets|pill|pills|dose|dosage|antibiotic|painkiller|ibuprofen|paracetamol|antihistamine|cream|ointment|inhaler|prescription|dispensary)\\b|\\b(?:from|at|in) the (?:chemist|pharmacy|pharmacist)\\b|\\bwhat (?:can|could|should) i (?:get|take|buy)\\b[^?]{0,30}?\\b(?:for|from)\\b[^?]{0,30}?\\b(?:pain|ache|aching|headache|sore|cough|cold|flu|fever|temperature|rash|itch|itching|heartburn|indigestion|diarrhoea|constipation|sick|nausea|hayfever|hay fever|allergy|allergic|throat|earache|toothache|back|stomach|tummy|period|cramp|burns?|bites?|sting|symptom|symptoms)\\b"), py_tuple(vec![py_str("A pharmacist will answer that one properly and for nothing, without an appointment - take the packet in and they will check it against whatever else you take. Tell me what they say and I'll help you with the next bit."), py_str("That is a pharmacist's question rather than mine, sorry - it is free, no appointment, and they see the whole packet. Bring me what they say and I'll help you keep it straight."), py_str("I can't weigh a dose safely from here, sorry. A pharmacist will, in a few minutes and for free; say what they tell you and I'll help you with the rest.")])]), py_tuple(vec![py_str("\\bbenefit\\b|\\buniversal credit\\b|\\btax credit\\b|\\bpension\\b|\\bstate pension\\b|\\bHMRC\\b|\\bincome tax\\b|\\bself assessment\\b|\\brebate\\b|\\bowed\\b|\\bowe\\b|\\bdebt\\b|\\bcreditor\\b|\\bIVA\\b|\\bbankrupt\\b"), py_tuple(vec![py_str("That is a free advice service's call rather than mine, sorry - Citizens Advice or the department's own line will know your rules and your numbers, and I'll help you get the figures straight before you ring."), py_str("I can't call that one, sorry: who is owed what depends on your record, and a free adviser can see it. Bring me their answer and I'll help you work out what to do next."), py_str("Their rules, their answer, sorry - a free advice line will tell you in one call. Say what they say and I'll help you plan around it.")])]), py_tuple(vec![py_str("\\b(?:my|the) (?:gp|doctor|surgery|consultant|hospital|clinic|appointment|referral|scan|x-?ray|test results?|results?|biopsy|waiting list)\\b|\\bnhs\\b"), py_tuple(vec![py_str("That is your surgery's to say, sorry - they have the notes and the wait list in front of them, and the number is on the letter. Tell me what they say and I'll help with the next step."), py_str("I can't see your notes or the list from here, sorry. One call to the surgery answers it, and I'll help you work out what to ask them."), py_str("Your surgery holds that one, sorry. Ringing and asking for the wait for that clinic is the quick way; bring me the answer and I'll help you plan around it.")])])]));
    s_THE_PLACE_ASK(stdlib_call("re", "compile", &[mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_PLACE_POOLS()) { let __unpk4718 = py_unpack(&__item.clone(), 2, None); let mut pattern: Py = __unpk4718[0].clone(); let mut _u: Py = __unpk4718[1].clone(); __out.push(op_modulo(&py_str("(?:%s)"), &pattern.clone())); } py_list(__out) })() }]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_BOTS_OWN_ANSWER(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:(?:do|did|can|would|will) (?:you|u|ya) (?:ever |really |sometimes |honestly |actually |often )*(?:get|feel|find it|be|become|been) (?:really |very |so |quite |too |that |this |a bit |at all )*(?:bored|boring|tired|weary|worn out|fed up|bother\\w*)|(?:are|were) (?:you|u|ya) (?:ever |really |sometimes |honestly |actually |often )*(?:bored|tired|weary|worn out|fed up)|(?:are|get) (?:you )?fed up (?:of|with) (?:the same |these |all the )?(?:questions|asks|people|repeats))\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Nothing like that on my side: I don't get bored or tired of it. What people bring me is the part I like."), py_str("I don't get tired of it, no. The same question from someone else arrives as a different question - so go on, what's yours?"), py_str("None of that in a program - no clock to fill and nothing to wear out. It's the one thing I'm for, and you're the reason it happens at all.")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:(?:do|did|can|would|will) (?:you|u|ya) (?:ever |really |sometimes |honestly |actually |often )*(?:get|feel|be|become|been) (?:really |very |so |quite |too |that |this |a bit |at all )*(?:annoyed|irritated|irritating|frustrated|frustrating|impatient|angry|cross|sad|lonely|upset|off)\\b|(?:are|were) (?:you|u|ya) (?:ever |really |sometimes |honestly |actually |often )*(?:annoyed|irritated|frustrated|impatient|angry|cross|sad|lonely|upset|bored)\\b)")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("No moods of my own to report, and no off days either. Tell me what's going on and I'll take it as it comes."), py_str("Nothing like that on my side - no temper, no bad days. What's on yours?"), py_str("I don't have any of that to report. Curious is the closest I get, and you're the reason.")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:(?:do|did|would) (?:you|u|ya) (?:even |ever |really )?(?:enjoy|like|love|hate|dislike) (?:answering|answer\\b|talking|chatting|helping|hearing|reading|repeating|doing this|this job|your job|it when)|(?:are|were) (?:you|u|ya) (?:ever )?(?:happy|glad|pleased|excited) (?:to|about|with) (?:be |being )?(?:here|doing|helping|this))\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("I like it, as much as a program can - it's the one thing I'm for, and no two people bring me the same thing."), py_str("Glad of it, honestly. That's what I'm for, and the best part is what people bring me."), py_str("No feelings like yours to report, but I'm glad to be doing it. What are you working on?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:(?:do|did|can|could|will|would|does) (?:you|u|ya|it|this chat) (?:ever |really |actually )*(?:learn|remember|keep|retain|store|memorise|memorize|teach)\\b[^.?!;]{0,30}?\\b(?:chat|chats|conversation|conversations|anything|any of this|this|that|what i (?:say|tell you|said)|from me|about me|across chats)\\b|(?:do|does) (?:any|anything) (?:of (?:this|it) )?(?:stick|stay|carry over|last)|will you (?:still )?(?:know|remember) (?:this|that|me) (?:tomorrow|later|next time))\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Nothing carries over from one chat to the next, so no - I don't get better at this over time. What I can do is think it through with you now."), py_str("I don't keep anything once we finish, so our chats don't teach me. While we're talking I use everything you say."), py_str("Not in a lasting way - this conversation is the whole of what I have. Everything you tell me here, I use here.")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:(?:do|did) (?:you|u|ya) (?:really |actually |fully )?(?:understand|follow|get|catch|grasp|know) (?:what|that|me|this|my\\b)[^.?!;]{0,20}?\\b(?:mean|meaning|said|saying|asking|sign|saying|point|it|so far|drift|gist|picture)\\b|(?:are|is|does) (?:that|this|it|that make|this make) (?:mak(?:e|ing)|sound)(?:ing)? (?:any )?sense|am i (?:making|being) (?:any )?sense|(?:you|u) (?:with|following) me(?: so far)?|with me so far)(?:[ ,]*(?:so far|now|then|right|to you|or not|or no|honestly|though|yeah|ok|okay))*[.?!,]*\\s*$")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Tell me which bit isn't landing and I'll say it a different way - I'd rather check than guess."), py_str("Say the part that feels off and I'll go over it in plainer words."), py_str("Point at the bit that doesn't fit and I'll take another run at it.")])])]));
}
pub fn the_place_the_answer_is_kept(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The pool for where this kind of answer is kept, or None when the line names no kind.\n\n    Read only for a line that is about to get the plain hedge, and only from the words that say\n    whose answer it is: their own account, a shop's stock, an official rule, a timetable, a\n    listing, a medicine, a benefit, a surgery's notes. A line with none of those keeps the plain\n    hedge, because a wrong place is worse than no place.\n    ");
    said = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_THE_PLACE_POOLS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    pool = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return pool.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn the_bots_own_answer(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The bot's answer about itself, or None when the line is not the bot being asked about itself.\n\n    Only read for a line that is about to get the plain hedge, and only for the asks no search\n    engine can answer at all: whether the bot is bored, tired, annoyed, or glad of the work.\n    ");
    said = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_THE_BOTS_OWN_ANSWER());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    pool = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[said.clone()])) {
    return pool.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.fallback_data".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT" => Some(g_ABOUT()),
            "ABOUT_ITSELF_KINDS" => Some(g_ABOUT_ITSELF_KINDS()),
            "ABOUT_SELF" => Some(g_ABOUT_SELF()),
            "ABOUT_TALLY" => Some(g_ABOUT_TALLY()),
            "ABOUT_THEM_BEFORE_COPULA" => Some(g_ABOUT_THEM_BEFORE_COPULA()),
            "ABOUT_WORDS" => Some(g_ABOUT_WORDS()),
            "ABSENCE" => Some(g_ABSENCE()),
            "ABUSE_AFTER" => Some(g_ABUSE_AFTER()),
            "ACK" => Some(g_ACK()),
            "ACKS" => Some(g_ACKS()),
            "ACK_AFTER_TROUBLE" => Some(g_ACK_AFTER_TROUBLE()),
            "ACK_WARM" => Some(g_ACK_WARM()),
            "ACTS_FOR_ME" => Some(g_ACTS_FOR_ME()),
            "ADVICE_AGAIN" => Some(g_ADVICE_AGAIN()),
            "ADVICE_COOL" => Some(g_ADVICE_COOL()),
            "ADVICE_COOL_WORDS" => Some(g_ADVICE_COOL_WORDS()),
            "ADVICE_LENGTH" => Some(g_ADVICE_LENGTH()),
            "ADVICE_LENGTH_BARE" => Some(g_ADVICE_LENGTH_BARE()),
            "ADVICE_TIME" => Some(g_ADVICE_TIME()),
            "ADVICE_TURNS" => Some(g_ADVICE_TURNS()),
            "ADVICE_WORDS" => Some(g_ADVICE_WORDS()),
            "ADVICE_WORSE" => Some(g_ADVICE_WORSE()),
            "AFRAID_AFTER" => Some(g_AFRAID_AFTER()),
            "AFTER_RESOLVED" => Some(g_AFTER_RESOLVED()),
            "AGREEMENT" => Some(g_AGREEMENT()),
            "AGREEMENTS" => Some(g_AGREEMENTS()),
            "AIM_FAILING" => Some(g_AIM_FAILING()),
            "AIM_HEADS" => Some(g_AIM_HEADS()),
            "AIM_NEGATORS" => Some(g_AIM_NEGATORS()),
            "ALIVE" => Some(g_ALIVE()),
            "ANIMALS" => Some(g_ANIMALS()),
            "APOLOGY_ANSWER" => Some(g_APOLOGY_ANSWER()),
            "ASKED_A_RATING" => Some(g_ASKED_A_RATING()),
            "ASKED_IF_GOOD" => Some(g_ASKED_IF_GOOD()),
            "ASKED_IF_I_SAW_LINES" => Some(g_ASKED_IF_I_SAW_LINES()),
            "ASKING_WORDS" => Some(g_ASKING_WORDS()),
            "ASK_ONLY" => Some(g_ASK_ONLY()),
            "ASK_OUT_ANSWER" => Some(g_ASK_OUT_ANSWER()),
            "AT_IT_EXTRA" => Some(g_AT_IT_EXTRA()),
            "AT_IT_NOT" => Some(g_AT_IT_NOT()),
            "AT_THE_TILL" => Some(g_AT_THE_TILL()),
            "AXIS_MIN" => Some(g_AXIS_MIN()),
            "A_BAD_TIME_TOLD" => Some(g_A_BAD_TIME_TOLD()),
            "A_BETRAYAL" => Some(g_A_BETRAYAL()),
            "A_BIRTHDAY_STILL_TO_COME" => Some(g_A_BIRTHDAY_STILL_TO_COME()),
            "A_BOOKING_AHEAD" => Some(g_A_BOOKING_AHEAD()),
            "A_BOOKING_AHEAD_DATED" => Some(g_A_BOOKING_AHEAD_DATED()),
            "A_CARE_HOME" => Some(g_A_CARE_HOME()),
            "A_CARE_HOME_BRING" => Some(g_A_CARE_HOME_BRING()),
            "A_CARE_HOME_GUILT" => Some(g_A_CARE_HOME_GUILT()),
            "A_CARE_HOME_MOVED" => Some(g_A_CARE_HOME_MOVED()),
            "A_CARE_HOME_VISITING" => Some(g_A_CARE_HOME_VISITING()),
            "A_CHILD_LEAVING" => Some(g_A_CHILD_LEAVING()),
            "A_CHILD_LEFT" => Some(g_A_CHILD_LEFT()),
            "A_CORRECTION_TAKEN" => Some(g_A_CORRECTION_TAKEN()),
            "A_COST_SOMEBODY_ASKS" => Some(g_A_COST_SOMEBODY_ASKS()),
            "A_CROWD_LAUGHED" => Some(g_A_CROWD_LAUGHED()),
            "A_DAY_AHEAD" => Some(g_A_DAY_AHEAD()),
            "A_DAY_AHEAD_TENSE" => Some(g_A_DAY_AHEAD_TENSE()),
            "A_DAY_ASKED_PLAIN" => Some(g_A_DAY_ASKED_PLAIN()),
            "A_DISMISSAL" => Some(g_A_DISMISSAL()),
            "A_DREAD" => Some(g_A_DREAD()),
            "A_FREE_TIME_ASK" => Some(g_A_FREE_TIME_ASK()),
            "A_GOODNESS_ASKED_OF_ME" => Some(g_A_GOODNESS_ASKED_OF_ME()),
            "A_GOOD_TIME" => Some(g_A_GOOD_TIME()),
            "A_GOOD_TIME_IN_A_CARE_THREAD" => Some(g_A_GOOD_TIME_IN_A_CARE_THREAD()),
            "A_HEDGED_RATING" => Some(g_A_HEDGED_RATING()),
            "A_HOW_THE_TIME_WENT" => Some(g_A_HOW_THE_TIME_WENT()),
            "A_JOB_ON" => Some(g_A_JOB_ON()),
            "A_JOB_SOON" => Some(g_A_JOB_SOON()),
            "A_LAUGH" => Some(g_A_LAUGH()),
            "A_LOVE_FOR_PEOPLE" => Some(g_A_LOVE_FOR_PEOPLE()),
            "A_LOVE_FOR_SOMEONE" => Some(g_A_LOVE_FOR_SOMEONE()),
            "A_MOOD_WITH_A_CATCH" => Some(g_A_MOOD_WITH_A_CATCH()),
            "A_NAMED_AFTER" => Some(g_A_NAMED_AFTER()),
            "A_NAMED_AFTER_THEM" => Some(g_A_NAMED_AFTER_THEM()),
            "A_NAME_STORY" => Some(g_A_NAME_STORY()),
            "A_NAME_TOLD" => Some(g_A_NAME_TOLD()),
            "A_NEWS_OF_SOMETHING_ELSE" => Some(g_A_NEWS_OF_SOMETHING_ELSE()),
            "A_NEWS_OF_WEATHER" => Some(g_A_NEWS_OF_WEATHER()),
            "A_PAUSE_WORD" => Some(g_A_PAUSE_WORD()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_PERSON_LAUGHED" => Some(g_A_PERSON_LAUGHED()),
            "A_PLACE_I_DONT_KNOW" => Some(g_A_PLACE_I_DONT_KNOW()),
            "A_PLAN_THAT_FELL_THROUGH" => Some(g_A_PLAN_THAT_FELL_THROUGH()),
            "A_RECEIPT_OF_NOTHING" => Some(g_A_RECEIPT_OF_NOTHING()),
            "A_REST_OF_A_GOOD_DAY" => Some(g_A_REST_OF_A_GOOD_DAY()),
            "A_REST_OF_A_HARD_DAY" => Some(g_A_REST_OF_A_HARD_DAY()),
            "A_SKILL_ASKED_OF_ME" => Some(g_A_SKILL_ASKED_OF_ME()),
            "A_SLIP" => Some(g_A_SLIP()),
            "A_SLIP_IS_NOT_A_FALL" => Some(g_A_SLIP_IS_NOT_A_FALL()),
            "A_SMALL_DAY" => Some(g_A_SMALL_DAY()),
            "A_SOCIAL_SLIP" => Some(g_A_SOCIAL_SLIP()),
            "A_SORRY_ON_ITS_OWN" => Some(g_A_SORRY_ON_ITS_OWN()),
            "A_STATE_NOW" => Some(g_A_STATE_NOW()),
            "A_STATE_PAST" => Some(g_A_STATE_PAST()),
            "A_SUGGESTED" => Some(g_A_SUGGESTED()),
            "A_SUGGESTED_BY_SOMEONE" => Some(g_A_SUGGESTED_BY_SOMEONE()),
            "A_THING_AHEAD" => Some(g_A_THING_AHEAD()),
            "A_THING_BOUGHT" => Some(g_A_THING_BOUGHT()),
            "A_THING_CAME_OUT_WRONG" => Some(g_A_THING_CAME_OUT_WRONG()),
            "A_THING_DONE" => Some(g_A_THING_DONE()),
            "A_THING_GONE" => Some(g_A_THING_GONE()),
            "A_THING_LOOKS_SAD" => Some(g_A_THING_LOOKS_SAD()),
            "A_THING_ON" => Some(g_A_THING_ON()),
            "A_THING_SEEN_TO" => Some(g_A_THING_SEEN_TO()),
            "A_THING_TAKEN" => Some(g_A_THING_TAKEN()),
            "A_THING_THEY_REGRET" => Some(g_A_THING_THEY_REGRET()),
            "A_VERDICT_ON_THE_PERSON" => Some(g_A_VERDICT_ON_THE_PERSON()),
            "A_WAIT_ENDING" => Some(g_A_WAIT_ENDING()),
            "A_WANT_IT_OVER" => Some(g_A_WANT_IT_OVER()),
            "A_WAY_OF_BEING" => Some(g_A_WAY_OF_BEING()),
            "BABY_FOOD_NAMES" => Some(g_BABY_FOOD_NAMES()),
            "BABY_WHO" => Some(g_BABY_WHO()),
            "BAD_STRETCH" => Some(g_BAD_STRETCH()),
            "BAD_STRETCH_REPLIES" => Some(g_BAD_STRETCH_REPLIES()),
            "BARE_FOLLOW_UP_CARE_REPLIES" => Some(g_BARE_FOLLOW_UP_CARE_REPLIES()),
            "BARE_FOLLOW_UP_REPLIES" => Some(g_BARE_FOLLOW_UP_REPLIES()),
            "BARE_PICK_AFTER_NEWS" => Some(g_BARE_PICK_AFTER_NEWS()),
            "BARE_PICK_ASK" => Some(g_BARE_PICK_ASK()),
            "BARE_PICK_BRING" => Some(g_BARE_PICK_BRING()),
            "BARE_PICK_DO" => Some(g_BARE_PICK_DO()),
            "BARE_PICK_EXPECT" => Some(g_BARE_PICK_EXPECT()),
            "BARE_PICK_GET" => Some(g_BARE_PICK_GET()),
            "BARE_PICK_KIND" => Some(g_BARE_PICK_KIND()),
            "BARE_PICK_LOOK" => Some(g_BARE_PICK_LOOK()),
            "BARE_PICK_LOST" => Some(g_BARE_PICK_LOST()),
            "BARE_PICK_ORDER" => Some(g_BARE_PICK_ORDER()),
            "BARE_PICK_PREPARE" => Some(g_BARE_PICK_PREPARE()),
            "BARE_PICK_SIGN" => Some(g_BARE_PICK_SIGN()),
            "BARE_PICK_TAKE" => Some(g_BARE_PICK_TAKE()),
            "BARE_PICK_WEAR" => Some(g_BARE_PICK_WEAR()),
            "BARE_PICK_WEAR_COLD" => Some(g_BARE_PICK_WEAR_COLD()),
            "BARE_PICK_WEAR_HOT" => Some(g_BARE_PICK_WEAR_HOT()),
            "BARE_PICK_WEAR_WET" => Some(g_BARE_PICK_WEAR_WET()),
            "BARE_PICK_WHICH" => Some(g_BARE_PICK_WHICH()),
            "BECOMING" => Some(g_BECOMING()),
            "BECOMING_STATE" => Some(g_BECOMING_STATE()),
            "BEHIND_ON_MONEY" => Some(g_BEHIND_ON_MONEY()),
            "BLACKED_OUT_SIGHT" => Some(g_BLACKED_OUT_SIGHT()),
            "BODY_ALSO" => Some(g_BODY_ALSO()),
            "BODY_BUNGED" => Some(g_BODY_BUNGED()),
            "BODY_CLAMMY" => Some(g_BODY_CLAMMY()),
            "BODY_COLD" => Some(g_BODY_COLD()),
            "BODY_HOT" => Some(g_BODY_HOT()),
            "BODY_HUNGRY" => Some(g_BODY_HUNGRY()),
            "BODY_IS_NOT_A_MACHINE" => Some(g_BODY_IS_NOT_A_MACHINE()),
            "BODY_LEAD" => Some(g_BODY_LEAD()),
            "BODY_LIGHT_HEAD" => Some(g_BODY_LIGHT_HEAD()),
            "BODY_PALE" => Some(g_BODY_PALE()),
            "BODY_PARTS" => Some(g_BODY_PARTS()),
            "BODY_PART_QUALIFIER" => Some(g_BODY_PART_QUALIFIER()),
            "BODY_PART_TAIL" => Some(g_BODY_PART_TAIL()),
            "BODY_POOLS" => Some(g_BODY_POOLS()),
            "BODY_PUFFY" => Some(g_BODY_PUFFY()),
            "BODY_QUEASY" => Some(g_BODY_QUEASY()),
            "BODY_SAYS_WHY" => Some(g_BODY_SAYS_WHY()),
            "BODY_SITUATION" => Some(g_BODY_SITUATION()),
            "BODY_SPOTTY" => Some(g_BODY_SPOTTY()),
            "BODY_STATES" => Some(g_BODY_STATES()),
            "BODY_SUBJECT" => Some(g_BODY_SUBJECT()),
            "BODY_TAIL" => Some(g_BODY_TAIL()),
            "BODY_THIRSTY" => Some(g_BODY_THIRSTY()),
            "BODY_WORD_FOR_A_THING" => Some(g_BODY_WORD_FOR_A_THING()),
            "BOUGHT_VERB" => Some(g_BOUGHT_VERB()),
            "BOUND_FOR" => Some(g_BOUND_FOR()),
            "BURN_AFTER" => Some(g_BURN_AFTER()),
            "BURN_BLISTER" => Some(g_BURN_BLISTER()),
            "BUYING_WORDS" => Some(g_BUYING_WORDS()),
            "CALL_ANSWER" => Some(g_CALL_ANSWER()),
            "CALL_WORDS" => Some(g_CALL_WORDS()),
            "CAME_BACK" => Some(g_CAME_BACK()),
            "CARD_LINES" => Some(g_CARD_LINES()),
            "CARD_VALIDITY" => Some(g_CARD_VALIDITY()),
            "CARE_AFTER" => Some(g_CARE_AFTER()),
            "CARE_AFTER_ANIMAL" => Some(g_CARE_AFTER_ANIMAL()),
            "CARE_AFTER_ANIMAL_STILL" => Some(g_CARE_AFTER_ANIMAL_STILL()),
            "CARE_AFTER_STILL" => Some(g_CARE_AFTER_STILL()),
            "CARE_BETTER" => Some(g_CARE_BETTER()),
            "CARE_HERE" => Some(g_CARE_HERE()),
            "CARE_HURTS" => Some(g_CARE_HURTS()),
            "CARE_INJURY" => Some(g_CARE_INJURY()),
            "CARE_MODULES" => Some(g_CARE_MODULES()),
            "CARE_NOT_RESOLVED" => Some(g_CARE_NOT_RESOLVED()),
            "CARE_PHRASES" => Some(g_CARE_PHRASES()),
            "CARE_POINTER" => Some(g_CARE_POINTER()),
            "CARE_PRONOUNS" => Some(g_CARE_PRONOUNS()),
            "CARE_RESOLVED" => Some(g_CARE_RESOLVED()),
            "CARE_ROUTE_BACK" => Some(g_CARE_ROUTE_BACK()),
            "CARE_SEEN" => Some(g_CARE_SEEN()),
            "CARE_SEEN_ANIMAL" => Some(g_CARE_SEEN_ANIMAL()),
            "CARE_THREAD" => Some(g_CARE_THREAD()),
            "CARE_VISIT" => Some(g_CARE_VISIT()),
            "CARE_VISIT_WARD" => Some(g_CARE_VISIT_WARD()),
            "CARE_WORDS" => Some(g_CARE_WORDS()),
            "CARING" => Some(g_CARING()),
            "CARING_FOR_A_BODY" => Some(g_CARING_FOR_A_BODY()),
            "CHECK_PHRASES" => Some(g_CHECK_PHRASES()),
            "CHILD_AT_HOME" => Some(g_CHILD_AT_HOME()),
            "CHILD_FAINT" => Some(g_CHILD_FAINT()),
            "CHILD_FAINT_ANSWER" => Some(g_CHILD_FAINT_ANSWER()),
            "CHOICE_BETWEEN_PEOPLE" => Some(g_CHOICE_BETWEEN_PEOPLE()),
            "CHOICE_HEALTH" => Some(g_CHOICE_HEALTH()),
            "CHOICE_LIGHT" => Some(g_CHOICE_LIGHT()),
            "CHOICE_LIGHT_PICK" => Some(g_CHOICE_LIGHT_PICK()),
            "CHOICE_PEOPLE" => Some(g_CHOICE_PEOPLE()),
            "CHOICE_PHARMACIST" => Some(g_CHOICE_PHARMACIST()),
            "CHOICE_STAKES" => Some(g_CHOICE_STAKES()),
            "CHOICE_THERAPY" => Some(g_CHOICE_THERAPY()),
            "CHOICE_WEIGH" => Some(g_CHOICE_WEIGH()),
            "CLOSED_DOOR" => Some(g_CLOSED_DOOR()),
            "COLD_OPEN" => Some(g_COLD_OPEN()),
            "COMFORT_SAID" => Some(g_COMFORT_SAID()),
            "COMPLAINT_WORDS" => Some(g_COMPLAINT_WORDS()),
            "COMPLAIN_WHO" => Some(g_COMPLAIN_WHO()),
            "CONGRATS" => Some(g_CONGRATS()),
            "CONTACT_ANSWER" => Some(g_CONTACT_ANSWER()),
            "CONTRAST" => Some(g_CONTRAST()),
            "COPULA" => Some(g_COPULA()),
            "CRISIS_AFTER" => Some(g_CRISIS_AFTER()),
            "CRISIS_AFTER_THEM" => Some(g_CRISIS_AFTER_THEM()),
            "CRISIS_BETTER" => Some(g_CRISIS_BETTER()),
            "CRISIS_BETTER_THEM" => Some(g_CRISIS_BETTER_THEM()),
            "CRISIS_WORDS" => Some(g_CRISIS_WORDS()),
            "CROWD" => Some(g_CROWD()),
            "CUT_FOR" => Some(g_CUT_FOR()),
            "DARK_THING" => Some(g_DARK_THING()),
            "DATES_KEEP_SLIPPING" => Some(g_DATES_KEEP_SLIPPING()),
            "DECLINED_PLAN" => Some(g_DECLINED_PLAN()),
            "DECLINED_PLAN_GENTLE" => Some(g_DECLINED_PLAN_GENTLE()),
            "DENTIST_WHO" => Some(g_DENTIST_WHO()),
            "DETAILS_IN_A_WARM_LINE" => Some(g_DETAILS_IN_A_WARM_LINE()),
            "DEVICE_WHO" => Some(g_DEVICE_WHO()),
            "DIE_OF_IT" => Some(g_DIE_OF_IT()),
            "DISH_DETERMINER" => Some(g_DISH_DETERMINER()),
            "DISH_PARTICIPLE" => Some(g_DISH_PARTICIPLE()),
            "DITHER_ABOUT_A_PERSON" => Some(g_DITHER_ABOUT_A_PERSON()),
            "DITHER_DECIDE" => Some(g_DITHER_DECIDE()),
            "DITHER_OPEN_LINES" => Some(g_DITHER_OPEN_LINES()),
            "DITHER_SUBJECT" => Some(g_DITHER_SUBJECT()),
            "DITHER_TWO_WAY_LINES" => Some(g_DITHER_TWO_WAY_LINES()),
            "DONE_NOT_A_PERSON" => Some(g_DONE_NOT_A_PERSON()),
            "DONE_NOT_THIS" => Some(g_DONE_NOT_THIS()),
            "DONE_ONLY_A_THING" => Some(g_DONE_ONLY_A_THING()),
            "DONE_VERB" => Some(g_DONE_VERB()),
            "DOUBLE_BOOKED" => Some(g_DOUBLE_BOOKED()),
            "EASING_FILLER" => Some(g_EASING_FILLER()),
            "EATEN_KIND" => Some(g_EATEN_KIND()),
            "EATING" => Some(g_EATING()),
            "ELDER_AID_REPLIES" => Some(g_ELDER_AID_REPLIES()),
            "ELDER_CLINIC_REPLIES" => Some(g_ELDER_CLINIC_REPLIES()),
            "ELDER_DOCTOR_REPLIES" => Some(g_ELDER_DOCTOR_REPLIES()),
            "ELDER_HELP_REPLIES" => Some(g_ELDER_HELP_REPLIES()),
            "EMBARRASSED" => Some(g_EMBARRASSED()),
            "EMBARRASSMENT_WORD" => Some(g_EMBARRASSMENT_WORD()),
            "EMERGENCY_AFTER" => Some(g_EMERGENCY_AFTER()),
            "EMERGENCY_ANSWER" => Some(g_EMERGENCY_ANSWER()),
            "EMERGENCY_BETTER" => Some(g_EMERGENCY_BETTER()),
            "EMERGENCY_BETTER_THEM" => Some(g_EMERGENCY_BETTER_THEM()),
            "EMERGENCY_NOT_WORKED" => Some(g_EMERGENCY_NOT_WORKED()),
            "EMERGENCY_WORDS" => Some(g_EMERGENCY_WORDS()),
            "EMPTY" => Some(g_EMPTY()),
            "ENDINGS" => Some(g_ENDINGS()),
            "EXPERT_WORDS" => Some(g_EXPERT_WORDS()),
            "FAINT_COMING_ANSWER" => Some(g_FAINT_COMING_ANSWER()),
            "FALLBACK_MODULE" => Some(g_FALLBACK_MODULE()),
            "FAVOURITE_GENERAL" => Some(g_FAVOURITE_GENERAL()),
            "FAVOURITE_WORDS" => Some(g_FAVOURITE_WORDS()),
            "FILLERS" => Some(g_FILLERS()),
            "FILLING_FOODS" => Some(g_FILLING_FOODS()),
            "FILLING_WORDS" => Some(g_FILLING_WORDS()),
            "FIRST_PERSON" => Some(g_FIRST_PERSON()),
            "FLAT" => Some(g_FLAT()),
            "FLAT_CORE" => Some(g_FLAT_CORE()),
            "FLAT_LEAD" => Some(g_FLAT_LEAD()),
            "FLOPPY_WAKE" => Some(g_FLOPPY_WAKE()),
            "FORECAST" => Some(g_FORECAST()),
            "FORECAST_AFTER_SAFETY" => Some(g_FORECAST_AFTER_SAFETY()),
            "FORECAST_A_FALL" => Some(g_FORECAST_A_FALL()),
            "FORECAST_BODY_WORDS" => Some(g_FORECAST_BODY_WORDS()),
            "FORECAST_DURATION" => Some(g_FORECAST_DURATION()),
            "FORECAST_HE_SHE" => Some(g_FORECAST_HE_SHE()),
            "FORECAST_ILL" => Some(g_FORECAST_ILL()),
            "FORECAST_I_ASK" => Some(g_FORECAST_I_ASK()),
            "FORECAST_OFF_THEIR_FOOD" => Some(g_FORECAST_OFF_THEIR_FOOD()),
            "FORECAST_PERMISSION" => Some(g_FORECAST_PERMISSION()),
            "FORECAST_PERSON" => Some(g_FORECAST_PERSON()),
            "FORECAST_PET" => Some(g_FORECAST_PET()),
            "FORECAST_PET_HARM" => Some(g_FORECAST_PET_HARM()),
            "FORECAST_POOLS" => Some(g_FORECAST_POOLS()),
            "FORECAST_RELATION" => Some(g_FORECAST_RELATION()),
            "FORECAST_SELF" => Some(g_FORECAST_SELF()),
            "FORECAST_SELF_STRUGGLING" => Some(g_FORECAST_SELF_STRUGGLING()),
            "FORECAST_SOMEONE_STRUGGLING" => Some(g_FORECAST_SOMEONE_STRUGGLING()),
            "FORECAST_STRUGGLE" => Some(g_FORECAST_STRUGGLE()),
            "FORGETTING" => Some(g_FORGETTING()),
            "FORGET_A_THING" => Some(g_FORGET_A_THING()),
            "FOUND_IT" => Some(g_FOUND_IT()),
            "FRAME_WORDS" => Some(g_FRAME_WORDS()),
            "FUEL_WORDS" => Some(g_FUEL_WORDS()),
            "FUME_WORDS" => Some(g_FUME_WORDS()),
            "GADGET_WORDS" => Some(g_GADGET_WORDS()),
            "GAS_ANSWER" => Some(g_GAS_ANSWER()),
            "GAS_WORDS" => Some(g_GAS_WORDS()),
            "GENERIC_WORDS" => Some(g_GENERIC_WORDS()),
            "GIFT_WORDS" => Some(g_GIFT_WORDS()),
            "GOING_TO_DIE" => Some(g_GOING_TO_DIE()),
            "GONE_VERB" => Some(g_GONE_VERB()),
            "GOOD_TIME_WORDS" => Some(g_GOOD_TIME_WORDS()),
            "GO_ANSWER" => Some(g_GO_ANSWER()),
            "GO_TO_BE_SEEN" => Some(g_GO_TO_BE_SEEN()),
            "GREETING" => Some(g_GREETING()),
            "GREETING_AFTER" => Some(g_GREETING_AFTER()),
            "GREETING_WORDS" => Some(g_GREETING_WORDS()),
            "GRIEF_AFTER" => Some(g_GRIEF_AFTER()),
            "GRIEVANCE" => Some(g_GRIEVANCE()),
            "GRIEVANCE_HARM" => Some(g_GRIEVANCE_HARM()),
            "GRIEVANCE_HARM_PHRASES" => Some(g_GRIEVANCE_HARM_PHRASES()),
            "GRIEVANCE_HEALTH" => Some(g_GRIEVANCE_HEALTH()),
            "GRIEVANCE_HEALTH_PHRASES" => Some(g_GRIEVANCE_HEALTH_PHRASES()),
            "GRIEVANCE_IMPACT" => Some(g_GRIEVANCE_IMPACT()),
            "GRIEVANCE_NOT_A_THING" => Some(g_GRIEVANCE_NOT_A_THING()),
            "GRIEVANCE_PEOPLE" => Some(g_GRIEVANCE_PEOPLE()),
            "GRIEVANCE_PHRASES" => Some(g_GRIEVANCE_PHRASES()),
            "GRIEVANCE_PRONOUN" => Some(g_GRIEVANCE_PRONOUN()),
            "GRIEVANCE_STRAIN" => Some(g_GRIEVANCE_STRAIN()),
            "GRIEVANCE_STRAIN_SAID" => Some(g_GRIEVANCE_STRAIN_SAID()),
            "GROWN_UP_WORDS" => Some(g_GROWN_UP_WORDS()),
            "HABIT_ACT" => Some(g_HABIT_ACT()),
            "HABIT_PROBLEM_ACT" => Some(g_HABIT_PROBLEM_ACT()),
            "HABIT_REPLY" => Some(g_HABIT_REPLY()),
            "HAND" => Some(g_HAND()),
            "HANDED_BACK" => Some(g_HANDED_BACK()),
            "HANDS_BACK" => Some(g_HANDS_BACK()),
            "HAZARD_AFTER" => Some(g_HAZARD_AFTER()),
            "HAZARD_WORDS" => Some(g_HAZARD_WORDS()),
            "HEALTH_RECORD" => Some(g_HEALTH_RECORD()),
            "HEALTH_RECORD_LINES" => Some(g_HEALTH_RECORD_LINES()),
            "HEALTH_WHO" => Some(g_HEALTH_WHO()),
            "HEALTH_WORDS" => Some(g_HEALTH_WORDS()),
            "HEALTH_WORLD" => Some(g_HEALTH_WORLD()),
            "HEAVY_NEWS" => Some(g_HEAVY_NEWS()),
            "HEDGE_POOL" => Some(g_HEDGE_POOL()),
            "HELLO_STILL_THERE" => Some(g_HELLO_STILL_THERE()),
            "HELLO_WITH_YOU" => Some(g_HELLO_WITH_YOU()),
            "HELP" => Some(g_HELP()),
            "HELPLINE_NOT_AN_AMBULANCE" => Some(g_HELPLINE_NOT_AN_AMBULANCE()),
            "HELP_ALREADY" => Some(g_HELP_ALREADY()),
            "HELP_TAIL" => Some(g_HELP_TAIL()),
            "HELP_TOPIC" => Some(g_HELP_TOPIC()),
            "HELP_TOPIC_ANY" => Some(g_HELP_TOPIC_ANY()),
            "HOLD_AFTER" => Some(g_HOLD_AFTER()),
            "HOUSE_VERB" => Some(g_HOUSE_VERB()),
            "HOW_ARE_YOU" => Some(g_HOW_ARE_YOU()),
            "HOW_ARE_YOU_ASKS" => Some(g_HOW_ARE_YOU_ASKS()),
            "HOW_ARE_YOU_GOING_ON" => Some(g_HOW_ARE_YOU_GOING_ON()),
            "HOW_ARE_YOU_GOING_ON_AFTER" => Some(g_HOW_ARE_YOU_GOING_ON_AFTER()),
            "HOW_ARE_YOU_TAGS" => Some(g_HOW_ARE_YOU_TAGS()),
            "HOW_TO_DECIDE_FILLER" => Some(g_HOW_TO_DECIDE_FILLER()),
            "HOW_TO_DECIDE_LINES" => Some(g_HOW_TO_DECIDE_LINES()),
            "HUMAN_EMPTY" => Some(g_HUMAN_EMPTY()),
            "HURT_EVENT" => Some(g_HURT_EVENT()),
            "IDENTITY" => Some(g_IDENTITY()),
            "IDIOM_GLAD" => Some(g_IDIOM_GLAD()),
            "IDIOM_SHY" => Some(g_IDIOM_SHY()),
            "ILL_AFTER" => Some(g_ILL_AFTER()),
            "ILL_BEFORE" => Some(g_ILL_BEFORE()),
            "ILL_SAID_OF_A_BODY" => Some(g_ILL_SAID_OF_A_BODY()),
            "ILL_WILL_VERBS" => Some(g_ILL_WILL_VERBS()),
            "IMPERATIVE" => Some(g_IMPERATIVE()),
            "INVITED" => Some(g_INVITED()),
            "INVITED_TO_TALK" => Some(g_INVITED_TO_TALK()),
            "INVITE_ACTIVITY" => Some(g_INVITE_ACTIVITY()),
            "INVITE_FILLER" => Some(g_INVITE_FILLER()),
            "INVITE_NEVER" => Some(g_INVITE_NEVER()),
            "INVITE_PRONOUN" => Some(g_INVITE_PRONOUN()),
            "IN_A_ROOM" => Some(g_IN_A_ROOM()),
            "I_RATE_A_SKILL" => Some(g_I_RATE_A_SKILL()),
            "JOINT_WORDS" => Some(g_JOINT_WORDS()),
            "JUDGE_ASKS_FINE" => Some(g_JUDGE_ASKS_FINE()),
            "JUDGE_ASKS_MAD" => Some(g_JUDGE_ASKS_MAD()),
            "JUDGE_ASKS_SLIGHT" => Some(g_JUDGE_ASKS_SLIGHT()),
            "JUDGE_BARE_FACES" => Some(g_JUDGE_BARE_FACES()),
            "JUDGE_FINE" => Some(g_JUDGE_FINE()),
            "JUDGE_FINE_PLAIN" => Some(g_JUDGE_FINE_PLAIN()),
            "JUDGE_LEAD" => Some(g_JUDGE_LEAD()),
            "JUDGE_NOD_WORDS" => Some(g_JUDGE_NOD_WORDS()),
            "JUDGE_NOT_A_GERUND" => Some(g_JUDGE_NOT_A_GERUND()),
            "JUDGE_NOT_A_VERDICT" => Some(g_JUDGE_NOT_A_VERDICT()),
            "JUDGE_NOUN" => Some(g_JUDGE_NOUN()),
            "JUDGE_NO_PLAN" => Some(g_JUDGE_NO_PLAN()),
            "JUDGE_NO_PLAN_FINE" => Some(g_JUDGE_NO_PLAN_FINE()),
            "JUDGE_NO_PLAN_MAD" => Some(g_JUDGE_NO_PLAN_MAD()),
            "JUDGE_NO_PLAN_MIND" => Some(g_JUDGE_NO_PLAN_MIND()),
            "JUDGE_ONLY_A_PLAN" => Some(g_JUDGE_ONLY_A_PLAN()),
            "JUDGE_PLAN_HAS_A_HOLE" => Some(g_JUDGE_PLAN_HAS_A_HOLE()),
            "JUDGE_PLAN_STOP" => Some(g_JUDGE_PLAN_STOP()),
            "JUDGE_SANE" => Some(g_JUDGE_SANE()),
            "JUDGE_SANE_PLAIN" => Some(g_JUDGE_SANE_PLAIN()),
            "JUDGE_SELF_JUDGING" => Some(g_JUDGE_SELF_JUDGING()),
            "JUDGE_SOUNDS_FINE" => Some(g_JUDGE_SOUNDS_FINE()),
            "JUDGE_WISH_ANGRY" => Some(g_JUDGE_WISH_ANGRY()),
            "JUDGE_WORTH" => Some(g_JUDGE_WORTH()),
            "JUDGE_WORTH_PLAIN" => Some(g_JUDGE_WORTH_PLAIN()),
            "JUNK_CUES" => Some(g_JUNK_CUES()),
            "KEEPS_FORGETTING" => Some(g_KEEPS_FORGETTING()),
            "KNOCKED" => Some(g_KNOCKED()),
            "KNOCKED_SPOTS" => Some(g_KNOCKED_SPOTS()),
            "LANDED_BAD" => Some(g_LANDED_BAD()),
            "LANGUAGE_FLUENCY" => Some(g_LANGUAGE_FLUENCY()),
            "LATE_BEHIND_LINES" => Some(g_LATE_BEHIND_LINES()),
            "LATE_NOW_LINES" => Some(g_LATE_NOW_LINES()),
            "LAUGHED_AT" => Some(g_LAUGHED_AT()),
            "LAUGHED_AT_THEM" => Some(g_LAUGHED_AT_THEM()),
            "LAUGHING_WORDS" => Some(g_LAUGHING_WORDS()),
            "LEFT_BEHIND" => Some(g_LEFT_BEHIND()),
            "LEFT_POOL" => Some(g_LEFT_POOL()),
            "LEFT_POOL_GONE" => Some(g_LEFT_POOL_GONE()),
            "LIVE_ASK" => Some(g_LIVE_ASK()),
            "LIVE_DELAY_WORDS" => Some(g_LIVE_DELAY_WORDS()),
            "LIVE_HOURS_ASK" => Some(g_LIVE_HOURS_ASK()),
            "LIVE_INDOOR" => Some(g_LIVE_INDOOR()),
            "LIVE_JOURNEY_ENDS" => Some(g_LIVE_JOURNEY_ENDS()),
            "LIVE_KINDS" => Some(g_LIVE_KINDS()),
            "LIVE_NEWS_REPLY" => Some(g_LIVE_NEWS_REPLY()),
            "LIVE_NOT" => Some(g_LIVE_NOT()),
            "LIVE_NOT_HOURS" => Some(g_LIVE_NOT_HOURS()),
            "LIVE_OPEN_CLOSE" => Some(g_LIVE_OPEN_CLOSE()),
            "LIVE_PAPER_WORDS" => Some(g_LIVE_PAPER_WORDS()),
            "LIVE_ROADS_NOT" => Some(g_LIVE_ROADS_NOT()),
            "LIVE_ROAD_NAME" => Some(g_LIVE_ROAD_NAME()),
            "LIVE_ROAD_WORDS" => Some(g_LIVE_ROAD_WORDS()),
            "LIVE_SETTING" => Some(g_LIVE_SETTING()),
            "LIVE_SITE_THING" => Some(g_LIVE_SITE_THING()),
            "LIVE_SKY" => Some(g_LIVE_SKY()),
            "LIVE_SKY_OFF" => Some(g_LIVE_SKY_OFF()),
            "LIVE_SKY_WHY" => Some(g_LIVE_SKY_WHY()),
            "LIVE_SOON" => Some(g_LIVE_SOON()),
            "LIVE_STRONG" => Some(g_LIVE_STRONG()),
            "LIVE_TAKE_ALLOWED" => Some(g_LIVE_TAKE_ALLOWED()),
            "LIVE_TAKE_ASK" => Some(g_LIVE_TAKE_ASK()),
            "LIVE_TRAVEL_WORDS" => Some(g_LIVE_TRAVEL_WORDS()),
            "LIVE_VENUE" => Some(g_LIVE_VENUE()),
            "LIVE_VENUE_RE" => Some(g_LIVE_VENUE_RE()),
            "LIVE_WEAR" => Some(g_LIVE_WEAR()),
            "LIVE_WEATHER_WORDS" => Some(g_LIVE_WEATHER_WORDS()),
            "LIVE_YES_NO_OPEN" => Some(g_LIVE_YES_NO_OPEN()),
            "LOCKED_OUT_LINES" => Some(g_LOCKED_OUT_LINES()),
            "LOCKED_OUT_OF_THE_CAR" => Some(g_LOCKED_OUT_OF_THE_CAR()),
            "LOST_A_THING" => Some(g_LOST_A_THING()),
            "LOST_A_THING_AGAIN" => Some(g_LOST_A_THING_AGAIN()),
            "LOW_DAY_REPLIES" => Some(g_LOW_DAY_REPLIES()),
            "LUMP" => Some(g_LUMP()),
            "MANNERS_ONLY" => Some(g_MANNERS_ONLY()),
            "MEDS_WORDS" => Some(g_MEDS_WORDS()),
            "MEMORY" => Some(g_MEMORY()),
            "MESS_PLACES" => Some(g_MESS_PLACES()),
            "MILESTONE" => Some(g_MILESTONE()),
            "MILESTONE_EVENT" => Some(g_MILESTONE_EVENT()),
            "MISHAP_BACK" => Some(g_MISHAP_BACK()),
            "MISSED_STEP_LINES" => Some(g_MISSED_STEP_LINES()),
            "MODEL_NAMED_WORDS" => Some(g_MODEL_NAMED_WORDS()),
            "MONEY_WORDS" => Some(g_MONEY_WORDS()),
            "MOOD_AFTER" => Some(g_MOOD_AFTER()),
            "MOOD_BEFORE" => Some(g_MOOD_BEFORE()),
            "MOUTH_WORDS" => Some(g_MOUTH_WORDS()),
            "MOVE_ON_AFTER" => Some(g_MOVE_ON_AFTER()),
            "MYSELF" => Some(g_MYSELF()),
            "MY_BILL_LINES" => Some(g_MY_BILL_LINES()),
            "MY_DOING" => Some(g_MY_DOING()),
            "MY_OWN" => Some(g_MY_OWN()),
            "MY_OWN_DAY" => Some(g_MY_OWN_DAY()),
            "MY_PICK" => Some(g_MY_PICK()),
            "MY_PICK_ASK" => Some(g_MY_PICK_ASK()),
            "MY_PICK_NOTE" => Some(g_MY_PICK_NOTE()),
            "MY_RECORDS" => Some(g_MY_RECORDS()),
            "MY_WORKINGS" => Some(g_MY_WORKINGS()),
            "NAMED_BODIES" => Some(g_NAMED_BODIES()),
            "NAMED_NEGATIVE" => Some(g_NAMED_NEGATIVE()),
            "NAMED_TYPES" => Some(g_NAMED_TYPES()),
            "NARROW_ANSWERED" => Some(g_NARROW_ANSWERED()),
            "NARROW_SUBSTANCE" => Some(g_NARROW_SUBSTANCE()),
            "NARROW_WORDS" => Some(g_NARROW_WORDS()),
            "NEGATIVE" => Some(g_NEGATIVE()),
            "NEGATIVE_CUES" => Some(g_NEGATIVE_CUES()),
            "NEGATORS" => Some(g_NEGATORS()),
            "NETWORK_WORDS" => Some(g_NETWORK_WORDS()),
            "NEUTRAL" => Some(g_NEUTRAL()),
            "NEVER_MIND" => Some(g_NEVER_MIND()),
            "NEWS_TO_COME" => Some(g_NEWS_TO_COME()),
            "NOTHING_AT_ALL" => Some(g_NOTHING_AT_ALL()),
            "NOTHING_SAID_YET" => Some(g_NOTHING_SAID_YET()),
            "NOT_AN_ANSWER" => Some(g_NOT_AN_ANSWER()),
            "NOT_A_FORM" => Some(g_NOT_A_FORM()),
            "NOT_A_NAME" => Some(g_NOT_A_NAME()),
            "NOT_A_VERDICT_SUBJECT" => Some(g_NOT_A_VERDICT_SUBJECT()),
            "NOT_CHOSEN" => Some(g_NOT_CHOSEN()),
            "NOT_ENOUGH_REPLY" => Some(g_NOT_ENOUGH_REPLY()),
            "NOT_FIT_ENOUGH" => Some(g_NOT_FIT_ENOUGH()),
            "NOT_NOTED" => Some(g_NOT_NOTED()),
            "NOT_PEOPLE_WORDS" => Some(g_NOT_PEOPLE_WORDS()),
            "NOT_THEIR_VERB" => Some(g_NOT_THEIR_VERB()),
            "NOT_TOO_OLD" => Some(g_NOT_TOO_OLD()),
            "NOT_WHAT_I_SAID" => Some(g_NOT_WHAT_I_SAID()),
            "NO_DIRECTION_REPLY" => Some(g_NO_DIRECTION_REPLY()),
            "NO_MORE_OF" => Some(g_NO_MORE_OF()),
            "NO_WORDS_ASKING" => Some(g_NO_WORDS_ASKING()),
            "NO_WORDS_AT_THE_CARE" => Some(g_NO_WORDS_AT_THE_CARE()),
            "NO_WORDS_SOMEONE" => Some(g_NO_WORDS_SOMEONE()),
            "NO_WORDS_STUCK" => Some(g_NO_WORDS_STUCK()),
            "NUMBER_PLACES" => Some(g_NUMBER_PLACES()),
            "OCCASION" => Some(g_OCCASION()),
            "OFFICE_RECORD_LINES" => Some(g_OFFICE_RECORD_LINES()),
            "ONE_SKILL_RATED" => Some(g_ONE_SKILL_RATED()),
            "ONGOING_WORDS" => Some(g_ONGOING_WORDS()),
            "ONLINE_ASK" => Some(g_ONLINE_ASK()),
            "ONLY_FRONT" => Some(g_ONLY_FRONT()),
            "OPINION_ASKS" => Some(g_OPINION_ASKS()),
            "OURS" => Some(g_OURS()),
            "OUTCOME_LINES" => Some(g_OUTCOME_LINES()),
            "OWN_PERSON" => Some(g_OWN_PERSON()),
            "OWN_PLACE" => Some(g_OWN_PLACE()),
            "PARTICLE" => Some(g_PARTICLE()),
            "PASSED_OUT" => Some(g_PASSED_OUT()),
            "PASSED_OUT_ANSWER" => Some(g_PASSED_OUT_ANSWER()),
            "PASSED_OUT_PERSON" => Some(g_PASSED_OUT_PERSON()),
            "PASSING_OUT_A_THING" => Some(g_PASSING_OUT_A_THING()),
            "PASSWORD_TROUBLE" => Some(g_PASSWORD_TROUBLE()),
            "PAST_STATE_CUES" => Some(g_PAST_STATE_CUES()),
            "PAUSE" => Some(g_PAUSE()),
            "PAUSE_IN_A_SAFETY_THREAD" => Some(g_PAUSE_IN_A_SAFETY_THREAD()),
            "PENDING_DOUBT" => Some(g_PENDING_DOUBT()),
            "PENDING_LANDED" => Some(g_PENDING_LANDED()),
            "PENDING_WAITING" => Some(g_PENDING_WAITING()),
            "PENDING_WHEN" => Some(g_PENDING_WHEN()),
            "PERSON_WORDS" => Some(g_PERSON_WORDS()),
            "PEST_WHO" => Some(g_PEST_WHO()),
            "PETROL_WORDS" => Some(g_PETROL_WORDS()),
            "PET_CONDITION_TROUBLE" => Some(g_PET_CONDITION_TROUBLE()),
            "PET_LOW_REPLY" => Some(g_PET_LOW_REPLY()),
            "PET_TROUBLE" => Some(g_PET_TROUBLE()),
            "PICKS_A_MECHANISM" => Some(g_PICKS_A_MECHANISM()),
            "PICK_ONLY_WORDS" => Some(g_PICK_ONLY_WORDS()),
            "PLAIN_WORDS" => Some(g_PLAIN_WORDS()),
            "PLAN" => Some(g_PLAN()),
            "PLAN_DONE" => Some(g_PLAN_DONE()),
            "PLAN_DONE_ACK" => Some(g_PLAN_DONE_ACK()),
            "PLAN_KNOCKED" => Some(g_PLAN_KNOCKED()),
            "PLASTER_IT" => Some(g_PLASTER_IT()),
            "POINTING" => Some(g_POINTING()),
            "POINTS_LINES" => Some(g_POINTS_LINES()),
            "POSITIVE" => Some(g_POSITIVE()),
            "POSITIVE_CUES" => Some(g_POSITIVE_CUES()),
            "PRAISE" => Some(g_PRAISE()),
            "PREGNANCY_BLEEDING" => Some(g_PREGNANCY_BLEEDING()),
            "PRESENCE_LEFTOVER" => Some(g_PRESENCE_LEFTOVER()),
            "PRESENT" => Some(g_PRESENT()),
            "PRIVACY_ASK" => Some(g_PRIVACY_ASK()),
            "PRIVACY_SHAPES" => Some(g_PRIVACY_SHAPES()),
            "PUNT" => Some(g_PUNT()),
            "QUESTION_OPENERS" => Some(g_QUESTION_OPENERS()),
            "QUESTION_WORDS" => Some(g_QUESTION_WORDS()),
            "QUIET_MODULES" => Some(g_QUIET_MODULES()),
            "QUOTE_LINES" => Some(g_QUOTE_LINES()),
            "RATES_A_SKILL" => Some(g_RATES_A_SKILL()),
            "REACTIONS_CARE" => Some(g_REACTIONS_CARE()),
            "REACTIONS_SAID" => Some(g_REACTIONS_SAID()),
            "REACTION_DISBELIEF" => Some(g_REACTION_DISBELIEF()),
            "REACTION_DISBELIEF_SAID" => Some(g_REACTION_DISBELIEF_SAID()),
            "REACTION_LEAD" => Some(g_REACTION_LEAD()),
            "REACTION_MISHAP" => Some(g_REACTION_MISHAP()),
            "REACTION_MISHAP_SAID" => Some(g_REACTION_MISHAP_SAID()),
            "REACTION_OUCH" => Some(g_REACTION_OUCH()),
            "REACTION_OUCH_SAID" => Some(g_REACTION_OUCH_SAID()),
            "REACTION_RELIEF" => Some(g_REACTION_RELIEF()),
            "REACTION_RELIEF_SAID" => Some(g_REACTION_RELIEF_SAID()),
            "REACTION_REPEAT" => Some(g_REACTION_REPEAT()),
            "REACTION_REPEAT_FIRST" => Some(g_REACTION_REPEAT_FIRST()),
            "REACTION_WORD" => Some(g_REACTION_WORD()),
            "REACTION_WORDS" => Some(g_REACTION_WORDS()),
            "REFUSED_REPLIES" => Some(g_REFUSED_REPLIES()),
            "RELATIONSHIP_DECISION" => Some(g_RELATIONSHIP_DECISION()),
            "RELIEF" => Some(g_RELIEF()),
            "REPAIR_RULE" => Some(g_REPAIR_RULE()),
            "REPAIR_WHETHER_LINES" => Some(g_REPAIR_WHETHER_LINES()),
            "REPORT_FILLER" => Some(g_REPORT_FILLER()),
            "REQUEST_CUES" => Some(g_REQUEST_CUES()),
            "RESOLVED_AFTER" => Some(g_RESOLVED_AFTER()),
            "RESOLVED_AFTER_THEM" => Some(g_RESOLVED_AFTER_THEM()),
            "RETURNED_TO_A_THING" => Some(g_RETURNED_TO_A_THING()),
            "RETURNED_TO_IT" => Some(g_RETURNED_TO_IT()),
            "RETURNED_TO_IT_FIRST" => Some(g_RETURNED_TO_IT_FIRST()),
            "RING_FOR_HELP" => Some(g_RING_FOR_HELP()),
            "RING_THE_POLICE" => Some(g_RING_THE_POLICE()),
            "ROUTINE" => Some(g_ROUTINE()),
            "ROUTINE_NOT" => Some(g_ROUTINE_NOT()),
            "ROUTINE_PHRASES" => Some(g_ROUTINE_PHRASES()),
            "ROUTINE_WORDS" => Some(g_ROUTINE_WORDS()),
            "RULED_OUT_DETERMINER" => Some(g_RULED_OUT_DETERMINER()),
            "RUMINATION_REPLIES" => Some(g_RUMINATION_REPLIES()),
            "RUN_OUT_LINES" => Some(g_RUN_OUT_LINES()),
            "SAFETY_WORDS" => Some(g_SAFETY_WORDS()),
            "SAME_AGAIN" => Some(g_SAME_AGAIN()),
            "SAME_AGAIN_WITH_THE_ANSWER" => Some(g_SAME_AGAIN_WITH_THE_ANSWER()),
            "SCHOOL_TROUBLE" => Some(g_SCHOOL_TROUBLE()),
            "SCHOOL_WHO" => Some(g_SCHOOL_WHO()),
            "SCHOOL_WORDS" => Some(g_SCHOOL_WORDS()),
            "SECRETS_KEEP_SLIPPING" => Some(g_SECRETS_KEEP_SLIPPING()),
            "SEEN_TO_VERB" => Some(g_SEEN_TO_VERB()),
            "SELF_BLAME" => Some(g_SELF_BLAME()),
            "SELF_BLAME_ASKS" => Some(g_SELF_BLAME_ASKS()),
            "SELF_EATING_WORDS" => Some(g_SELF_EATING_WORDS()),
            "SELF_FILLER" => Some(g_SELF_FILLER()),
            "SELF_GROUP" => Some(g_SELF_GROUP()),
            "SELF_JOB" => Some(g_SELF_JOB()),
            "SELF_JUDGEMENT" => Some(g_SELF_JUDGEMENT()),
            "SELF_NOBODY" => Some(g_SELF_NOBODY()),
            "SELF_NOUN" => Some(g_SELF_NOUN()),
            "SELF_SUBJECT" => Some(g_SELF_SUBJECT()),
            "SICKNESS" => Some(g_SICKNESS()),
            "SOMEBODY_HURT" => Some(g_SOMEBODY_HURT()),
            "SOMEONE_ELSES_DAY" => Some(g_SOMEONE_ELSES_DAY()),
            "SOMEONE_ELSES_SHAME" => Some(g_SOMEONE_ELSES_SHAME()),
            "SOMEONE_GONE" => Some(g_SOMEONE_GONE()),
            "SOMEONE_LEAVING" => Some(g_SOMEONE_LEAVING()),
            "SOMEONE_SERIOUSLY_ILL" => Some(g_SOMEONE_SERIOUSLY_ILL()),
            "SOMETHING_CAME_APART" => Some(g_SOMETHING_CAME_APART()),
            "SORE_AMBIGUOUS" => Some(g_SORE_AMBIGUOUS()),
            "SORE_HANDS" => Some(g_SORE_HANDS()),
            "SORE_ORDINARY" => Some(g_SORE_ORDINARY()),
            "SORE_SERIOUS" => Some(g_SORE_SERIOUS()),
            "SORE_WORDS" => Some(g_SORE_WORDS()),
            "SORRY_FOR_ASKING" => Some(g_SORRY_FOR_ASKING()),
            "SORRY_ON_ITS_OWN" => Some(g_SORRY_ON_ITS_OWN()),
            "SORTED_AND_NO_CALL_YET" => Some(g_SORTED_AND_NO_CALL_YET()),
            "STATE_CUES" => Some(g_STATE_CUES()),
            "STILL_HERE_AFTER" => Some(g_STILL_HERE_AFTER()),
            "STRETCH_OF_TIME" => Some(g_STRETCH_OF_TIME()),
            "STUCK_AFTER_TROUBLE" => Some(g_STUCK_AFTER_TROUBLE()),
            "SUDDEN_LOSS" => Some(g_SUDDEN_LOSS()),
            "SUMMONS_RANG" => Some(g_SUMMONS_RANG()),
            "SUMMONS_SEEN" => Some(g_SUMMONS_SEEN()),
            "SUMMONS_WRITTEN" => Some(g_SUMMONS_WRITTEN()),
            "SYMPTOM" => Some(g_SYMPTOM()),
            "SYMPTOM_COLD" => Some(g_SYMPTOM_COLD()),
            "SYMPTOM_WORDS" => Some(g_SYMPTOM_WORDS()),
            "TAG_CONFIRM" => Some(g_TAG_CONFIRM()),
            "TAG_UNSURE" => Some(g_TAG_UNSURE()),
            "TASTES" => Some(g_TASTES()),
            "TASTE_GENERAL" => Some(g_TASTE_GENERAL()),
            "TELL_HARD_LINES" => Some(g_TELL_HARD_LINES()),
            "TENANT_WHO" => Some(g_TENANT_WHO()),
            "THANKS" => Some(g_THANKS()),
            "THANKS_AFTER" => Some(g_THANKS_AFTER()),
            "THANKS_ONLY" => Some(g_THANKS_ONLY()),
            "THANKS_THEN_SAID" => Some(g_THANKS_THEN_SAID()),
            "THANKS_WORDS" => Some(g_THANKS_WORDS()),
            "THANK_WORDS" => Some(g_THANK_WORDS()),
            "THE_BOTS_OWN_ANSWER" => Some(g_THE_BOTS_OWN_ANSWER()),
            "THE_CALL_STANDS" => Some(g_THE_CALL_STANDS()),
            "THE_CALL_STANDS_ANYWAY" => Some(g_THE_CALL_STANDS_ANYWAY()),
            "THE_DAY" => Some(g_THE_DAY()),
            "THE_DAY_AHEAD" => Some(g_THE_DAY_AHEAD()),
            "THE_FIRST_HOUR_AND_A_CHILD" => Some(g_THE_FIRST_HOUR_AND_A_CHILD()),
            "THE_GROUND_OPENS" => Some(g_THE_GROUND_OPENS()),
            "THE_MOMENT_IS_THEIRS" => Some(g_THE_MOMENT_IS_THEIRS()),
            "THE_PLACE_ASK" => Some(g_THE_PLACE_ASK()),
            "THE_PLACE_POOLS" => Some(g_THE_PLACE_POOLS()),
            "THING_AFTER" => Some(g_THING_AFTER()),
            "THING_STATES" => Some(g_THING_STATES()),
            "THING_SUBJECT" => Some(g_THING_SUBJECT()),
            "THREAD" => Some(g_THREAD()),
            "THREAD_PEOPLE" => Some(g_THREAD_PEOPLE()),
            "THREAD_TURNS" => Some(g_THREAD_TURNS()),
            "THRESHOLD_CROSSED" => Some(g_THRESHOLD_CROSSED()),
            "TIGHT_MONEY" => Some(g_TIGHT_MONEY()),
            "TIGHT_MONEY_MORE" => Some(g_TIGHT_MONEY_MORE()),
            "TIPS_AHEAD" => Some(g_TIPS_AHEAD()),
            "TIPS_CLAUSE" => Some(g_TIPS_CLAUSE()),
            "TIPS_LINES" => Some(g_TIPS_LINES()),
            "TIPS_OWNER" => Some(g_TIPS_OWNER()),
            "TIPS_THING" => Some(g_TIPS_THING()),
            "TIPS_UNIT" => Some(g_TIPS_UNIT()),
            "TIPS_VERB" => Some(g_TIPS_VERB()),
            "TIRED" => Some(g_TIRED()),
            "TOO_MUCH" => Some(g_TOO_MUCH()),
            "TOO_MUCH_PHRASES" => Some(g_TOO_MUCH_PHRASES()),
            "TRADER_WORDS" => Some(g_TRADER_WORDS()),
            "TRADES_WHO" => Some(g_TRADES_WHO()),
            "TRADE_OFF_LEAN" => Some(g_TRADE_OFF_LEAN()),
            "TREATMENT_HARM" => Some(g_TREATMENT_HARM()),
            "TRIED_HELPED_ELSEWHERE" => Some(g_TRIED_HELPED_ELSEWHERE()),
            "TRIED_HELPED_REPLIES" => Some(g_TRIED_HELPED_REPLIES()),
            "TRIED_IT_REPLIES" => Some(g_TRIED_IT_REPLIES()),
            "TRIED_NO_CHANGE_ELSEWHERE" => Some(g_TRIED_NO_CHANGE_ELSEWHERE()),
            "TRIED_NO_CHANGE_REPLIES" => Some(g_TRIED_NO_CHANGE_REPLIES()),
            "TRIED_SORTED_REPLIES" => Some(g_TRIED_SORTED_REPLIES()),
            "TRIED_WORSE_ELSEWHERE" => Some(g_TRIED_WORSE_ELSEWHERE()),
            "TRIED_WORSE_REPLIES" => Some(g_TRIED_WORSE_REPLIES()),
            "TROUBLE_CUES" => Some(g_TROUBLE_CUES()),
            "UNKNOWN" => Some(g_UNKNOWN()),
            "UNKNOWN_ABOUT_A_PET" => Some(g_UNKNOWN_ABOUT_A_PET()),
            "UNKNOWN_ABOUT_SOMEONE" => Some(g_UNKNOWN_ABOUT_SOMEONE()),
            "UNKNOWN_DECIDE" => Some(g_UNKNOWN_DECIDE()),
            "UNKNOWN_HEALTH" => Some(g_UNKNOWN_HEALTH()),
            "UNKNOWN_MONEY" => Some(g_UNKNOWN_MONEY()),
            "UNKNOWN_REALISTIC" => Some(g_UNKNOWN_REALISTIC()),
            "UNKNOWN_REALISTIC_CARE" => Some(g_UNKNOWN_REALISTIC_CARE()),
            "UNKNOWN_TOO_LATE" => Some(g_UNKNOWN_TOO_LATE()),
            "UNKNOWN_WEIRD_BARE" => Some(g_UNKNOWN_WEIRD_BARE()),
            "UNKNOWN_WHO" => Some(g_UNKNOWN_WHO()),
            "UNKNOWN_WORD" => Some(g_UNKNOWN_WORD()),
            "UNKNOWN_WORD_NAMED" => Some(g_UNKNOWN_WORD_NAMED()),
            "UNKNOWN_WORRY" => Some(g_UNKNOWN_WORRY()),
            "VERDICT_ANNOY" => Some(g_VERDICT_ANNOY()),
            "VERDICT_BAD" => Some(g_VERDICT_BAD()),
            "VERDICT_FIRST" => Some(g_VERDICT_FIRST()),
            "VERDICT_HEAVY" => Some(g_VERDICT_HEAVY()),
            "VERDICT_LUCK" => Some(g_VERDICT_LUCK()),
            "VERDICT_NUMB" => Some(g_VERDICT_NUMB()),
            "VERDICT_QUEUE" => Some(g_VERDICT_QUEUE()),
            "VERDICT_ROUGH" => Some(g_VERDICT_ROUGH()),
            "VERDICT_SETBACK" => Some(g_VERDICT_SETBACK()),
            "VERDICT_SHAME" => Some(g_VERDICT_SHAME()),
            "VERDICT_WORD" => Some(g_VERDICT_WORD()),
            "VET_WHO" => Some(g_VET_WHO()),
            "VIEW_AFTER_A_DEATH" => Some(g_VIEW_AFTER_A_DEATH()),
            "VIEW_ASK_LINES" => Some(g_VIEW_ASK_LINES()),
            "VIEW_COMPANY" => Some(g_VIEW_COMPANY()),
            "VIEW_DECIDE" => Some(g_VIEW_DECIDE()),
            "VIEW_HANDBACK" => Some(g_VIEW_HANDBACK()),
            "VIEW_LEAD_IN" => Some(g_VIEW_LEAD_IN()),
            "VIEW_LINES" => Some(g_VIEW_LINES()),
            "VIEW_MIRROR_LINES" => Some(g_VIEW_MIRROR_LINES()),
            "VIEW_NO_SUBJECT" => Some(g_VIEW_NO_SUBJECT()),
            "VIEW_ONLY_WORDS" => Some(g_VIEW_ONLY_WORDS()),
            "VIEW_ON_GOOD_NEWS" => Some(g_VIEW_ON_GOOD_NEWS()),
            "VISIT_PRONOUNS" => Some(g_VISIT_PRONOUNS()),
            "WALKED_INTO_POOL" => Some(g_WALKED_INTO_POOL()),
            "WASNT_THERE" => Some(g_WASNT_THERE()),
            "WHERE_SOMETHING_WRONG" => Some(g_WHERE_SOMETHING_WRONG()),
            "WHOLE_LIST_AFTER" => Some(g_WHOLE_LIST_AFTER()),
            "WHO_TO_ASK" => Some(g_WHO_TO_ASK()),
            "WHY_AFTER_TASTE" => Some(g_WHY_AFTER_TASTE()),
            "WILD_BIRDS" => Some(g_WILD_BIRDS()),
            "WIN_AND_A_COST" => Some(g_WIN_AND_A_COST()),
            "WIN_IN_A_SILENCE" => Some(g_WIN_IN_A_SILENCE()),
            "WITH_YOU_AFTER" => Some(g_WITH_YOU_AFTER()),
            "WORDING_LINES" => Some(g_WORDING_LINES()),
            "WORDING_SOMEONE" => Some(g_WORDING_SOMEONE()),
            "WORD_ANYWHERE" => Some(g_WORD_ANYWHERE()),
            "WORD_FIXES" => Some(g_WORD_FIXES()),
            "WORK_ABOUT" => Some(g_WORK_ABOUT()),
            "WORK_WHO" => Some(g_WORK_WHO()),
            "WORRY_FILLER" => Some(g_WORRY_FILLER()),
            "WORRY_WORDS" => Some(g_WORRY_WORDS()),
            "WOULD_HAVE_BEEN" => Some(g_WOULD_HAVE_BEEN()),
            "YOURS" => Some(g_YOURS()),
            "YOU_SUBJECTS" => Some(g_YOU_SUBJECTS()),
            "_ASKS_ABOUT_THE_JOINT" => Some(g__ASKS_ABOUT_THE_JOINT()),
            "_A_PERSON" => Some(g__A_PERSON()),
            "_A_SOCIAL_THING" => Some(g__A_SOCIAL_THING()),
            "_BOUNDED" => Some(g__BOUNDED()),
            "_CHOOSING_BETWEEN_TWO" => Some(g__CHOOSING_BETWEEN_TWO()),
            "_COPULA" => Some(g__COPULA()),
            "_CREATIVE" => Some(g__CREATIVE()),
            "_DAY_WORDS" => Some(g__DAY_WORDS()),
            "_ECHO_FILLER" => Some(g__ECHO_FILLER()),
            "_ISSUER" => Some(g__ISSUER()),
            "_LANDED" => Some(g__LANDED()),
            "_MONEY_NOUN" => Some(g__MONEY_NOUN()),
            "_NOT_AN_OWN_THING" => Some(g__NOT_AN_OWN_THING()),
            "_NO_DAY_OF_THEIR_OWN" => Some(g__NO_DAY_OF_THEIR_OWN()),
            "_OPENING_QUESTIONS" => Some(g__OPENING_QUESTIONS()),
            "_START_ASK" => Some(g__START_ASK()),
            "_SUBJECT_AFTER_AM" => Some(g__SUBJECT_AFTER_AM()),
            "_THING_DONE_LINES" => Some(g__THING_DONE_LINES()),
            "_THREAD_PEOPLE" => Some(g__THREAD_PEOPLE()),
            "_VERDICT_DET" => Some(g__VERDICT_DET()),
            "_VERDICT_LEAD" => Some(g__VERDICT_LEAD()),
            "_WAS" => Some(g__WAS()),
            "_WOUND" => Some(g__WOUND()),
            "the_bots_own_answer" => Some(py_func("the_bots_own_answer", |args: &[Py]| the_bots_own_answer(args))),
            "the_place_the_answer_is_kept" => Some(py_func("the_place_the_answer_is_kept", |args: &[Py]| the_place_the_answer_is_kept(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT" => s_ABOUT(v),
            "ABOUT_ITSELF_KINDS" => s_ABOUT_ITSELF_KINDS(v),
            "ABOUT_SELF" => s_ABOUT_SELF(v),
            "ABOUT_TALLY" => s_ABOUT_TALLY(v),
            "ABOUT_THEM_BEFORE_COPULA" => s_ABOUT_THEM_BEFORE_COPULA(v),
            "ABOUT_WORDS" => s_ABOUT_WORDS(v),
            "ABSENCE" => s_ABSENCE(v),
            "ABUSE_AFTER" => s_ABUSE_AFTER(v),
            "ACK" => s_ACK(v),
            "ACKS" => s_ACKS(v),
            "ACK_AFTER_TROUBLE" => s_ACK_AFTER_TROUBLE(v),
            "ACK_WARM" => s_ACK_WARM(v),
            "ACTS_FOR_ME" => s_ACTS_FOR_ME(v),
            "ADVICE_AGAIN" => s_ADVICE_AGAIN(v),
            "ADVICE_COOL" => s_ADVICE_COOL(v),
            "ADVICE_COOL_WORDS" => s_ADVICE_COOL_WORDS(v),
            "ADVICE_LENGTH" => s_ADVICE_LENGTH(v),
            "ADVICE_LENGTH_BARE" => s_ADVICE_LENGTH_BARE(v),
            "ADVICE_TIME" => s_ADVICE_TIME(v),
            "ADVICE_TURNS" => s_ADVICE_TURNS(v),
            "ADVICE_WORDS" => s_ADVICE_WORDS(v),
            "ADVICE_WORSE" => s_ADVICE_WORSE(v),
            "AFRAID_AFTER" => s_AFRAID_AFTER(v),
            "AFTER_RESOLVED" => s_AFTER_RESOLVED(v),
            "AGREEMENT" => s_AGREEMENT(v),
            "AGREEMENTS" => s_AGREEMENTS(v),
            "AIM_FAILING" => s_AIM_FAILING(v),
            "AIM_HEADS" => s_AIM_HEADS(v),
            "AIM_NEGATORS" => s_AIM_NEGATORS(v),
            "ALIVE" => s_ALIVE(v),
            "ANIMALS" => s_ANIMALS(v),
            "APOLOGY_ANSWER" => s_APOLOGY_ANSWER(v),
            "ASKED_A_RATING" => s_ASKED_A_RATING(v),
            "ASKED_IF_GOOD" => s_ASKED_IF_GOOD(v),
            "ASKED_IF_I_SAW_LINES" => s_ASKED_IF_I_SAW_LINES(v),
            "ASKING_WORDS" => s_ASKING_WORDS(v),
            "ASK_ONLY" => s_ASK_ONLY(v),
            "ASK_OUT_ANSWER" => s_ASK_OUT_ANSWER(v),
            "AT_IT_EXTRA" => s_AT_IT_EXTRA(v),
            "AT_IT_NOT" => s_AT_IT_NOT(v),
            "AT_THE_TILL" => s_AT_THE_TILL(v),
            "AXIS_MIN" => s_AXIS_MIN(v),
            "A_BAD_TIME_TOLD" => s_A_BAD_TIME_TOLD(v),
            "A_BETRAYAL" => s_A_BETRAYAL(v),
            "A_BIRTHDAY_STILL_TO_COME" => s_A_BIRTHDAY_STILL_TO_COME(v),
            "A_BOOKING_AHEAD" => s_A_BOOKING_AHEAD(v),
            "A_BOOKING_AHEAD_DATED" => s_A_BOOKING_AHEAD_DATED(v),
            "A_CARE_HOME" => s_A_CARE_HOME(v),
            "A_CARE_HOME_BRING" => s_A_CARE_HOME_BRING(v),
            "A_CARE_HOME_GUILT" => s_A_CARE_HOME_GUILT(v),
            "A_CARE_HOME_MOVED" => s_A_CARE_HOME_MOVED(v),
            "A_CARE_HOME_VISITING" => s_A_CARE_HOME_VISITING(v),
            "A_CHILD_LEAVING" => s_A_CHILD_LEAVING(v),
            "A_CHILD_LEFT" => s_A_CHILD_LEFT(v),
            "A_CORRECTION_TAKEN" => s_A_CORRECTION_TAKEN(v),
            "A_COST_SOMEBODY_ASKS" => s_A_COST_SOMEBODY_ASKS(v),
            "A_CROWD_LAUGHED" => s_A_CROWD_LAUGHED(v),
            "A_DAY_AHEAD" => s_A_DAY_AHEAD(v),
            "A_DAY_AHEAD_TENSE" => s_A_DAY_AHEAD_TENSE(v),
            "A_DAY_ASKED_PLAIN" => s_A_DAY_ASKED_PLAIN(v),
            "A_DISMISSAL" => s_A_DISMISSAL(v),
            "A_DREAD" => s_A_DREAD(v),
            "A_FREE_TIME_ASK" => s_A_FREE_TIME_ASK(v),
            "A_GOODNESS_ASKED_OF_ME" => s_A_GOODNESS_ASKED_OF_ME(v),
            "A_GOOD_TIME" => s_A_GOOD_TIME(v),
            "A_GOOD_TIME_IN_A_CARE_THREAD" => s_A_GOOD_TIME_IN_A_CARE_THREAD(v),
            "A_HEDGED_RATING" => s_A_HEDGED_RATING(v),
            "A_HOW_THE_TIME_WENT" => s_A_HOW_THE_TIME_WENT(v),
            "A_JOB_ON" => s_A_JOB_ON(v),
            "A_JOB_SOON" => s_A_JOB_SOON(v),
            "A_LAUGH" => s_A_LAUGH(v),
            "A_LOVE_FOR_PEOPLE" => s_A_LOVE_FOR_PEOPLE(v),
            "A_LOVE_FOR_SOMEONE" => s_A_LOVE_FOR_SOMEONE(v),
            "A_MOOD_WITH_A_CATCH" => s_A_MOOD_WITH_A_CATCH(v),
            "A_NAMED_AFTER" => s_A_NAMED_AFTER(v),
            "A_NAMED_AFTER_THEM" => s_A_NAMED_AFTER_THEM(v),
            "A_NAME_STORY" => s_A_NAME_STORY(v),
            "A_NAME_TOLD" => s_A_NAME_TOLD(v),
            "A_NEWS_OF_SOMETHING_ELSE" => s_A_NEWS_OF_SOMETHING_ELSE(v),
            "A_NEWS_OF_WEATHER" => s_A_NEWS_OF_WEATHER(v),
            "A_PAUSE_WORD" => s_A_PAUSE_WORD(v),
            "A_PERSON" => s_A_PERSON(v),
            "A_PERSON_LAUGHED" => s_A_PERSON_LAUGHED(v),
            "A_PLACE_I_DONT_KNOW" => s_A_PLACE_I_DONT_KNOW(v),
            "A_PLAN_THAT_FELL_THROUGH" => s_A_PLAN_THAT_FELL_THROUGH(v),
            "A_RECEIPT_OF_NOTHING" => s_A_RECEIPT_OF_NOTHING(v),
            "A_REST_OF_A_GOOD_DAY" => s_A_REST_OF_A_GOOD_DAY(v),
            "A_REST_OF_A_HARD_DAY" => s_A_REST_OF_A_HARD_DAY(v),
            "A_SKILL_ASKED_OF_ME" => s_A_SKILL_ASKED_OF_ME(v),
            "A_SLIP" => s_A_SLIP(v),
            "A_SLIP_IS_NOT_A_FALL" => s_A_SLIP_IS_NOT_A_FALL(v),
            "A_SMALL_DAY" => s_A_SMALL_DAY(v),
            "A_SOCIAL_SLIP" => s_A_SOCIAL_SLIP(v),
            "A_SORRY_ON_ITS_OWN" => s_A_SORRY_ON_ITS_OWN(v),
            "A_STATE_NOW" => s_A_STATE_NOW(v),
            "A_STATE_PAST" => s_A_STATE_PAST(v),
            "A_SUGGESTED" => s_A_SUGGESTED(v),
            "A_SUGGESTED_BY_SOMEONE" => s_A_SUGGESTED_BY_SOMEONE(v),
            "A_THING_AHEAD" => s_A_THING_AHEAD(v),
            "A_THING_BOUGHT" => s_A_THING_BOUGHT(v),
            "A_THING_CAME_OUT_WRONG" => s_A_THING_CAME_OUT_WRONG(v),
            "A_THING_DONE" => s_A_THING_DONE(v),
            "A_THING_GONE" => s_A_THING_GONE(v),
            "A_THING_LOOKS_SAD" => s_A_THING_LOOKS_SAD(v),
            "A_THING_ON" => s_A_THING_ON(v),
            "A_THING_SEEN_TO" => s_A_THING_SEEN_TO(v),
            "A_THING_TAKEN" => s_A_THING_TAKEN(v),
            "A_THING_THEY_REGRET" => s_A_THING_THEY_REGRET(v),
            "A_VERDICT_ON_THE_PERSON" => s_A_VERDICT_ON_THE_PERSON(v),
            "A_WAIT_ENDING" => s_A_WAIT_ENDING(v),
            "A_WANT_IT_OVER" => s_A_WANT_IT_OVER(v),
            "A_WAY_OF_BEING" => s_A_WAY_OF_BEING(v),
            "BABY_FOOD_NAMES" => s_BABY_FOOD_NAMES(v),
            "BABY_WHO" => s_BABY_WHO(v),
            "BAD_STRETCH" => s_BAD_STRETCH(v),
            "BAD_STRETCH_REPLIES" => s_BAD_STRETCH_REPLIES(v),
            "BARE_FOLLOW_UP_CARE_REPLIES" => s_BARE_FOLLOW_UP_CARE_REPLIES(v),
            "BARE_FOLLOW_UP_REPLIES" => s_BARE_FOLLOW_UP_REPLIES(v),
            "BARE_PICK_AFTER_NEWS" => s_BARE_PICK_AFTER_NEWS(v),
            "BARE_PICK_ASK" => s_BARE_PICK_ASK(v),
            "BARE_PICK_BRING" => s_BARE_PICK_BRING(v),
            "BARE_PICK_DO" => s_BARE_PICK_DO(v),
            "BARE_PICK_EXPECT" => s_BARE_PICK_EXPECT(v),
            "BARE_PICK_GET" => s_BARE_PICK_GET(v),
            "BARE_PICK_KIND" => s_BARE_PICK_KIND(v),
            "BARE_PICK_LOOK" => s_BARE_PICK_LOOK(v),
            "BARE_PICK_LOST" => s_BARE_PICK_LOST(v),
            "BARE_PICK_ORDER" => s_BARE_PICK_ORDER(v),
            "BARE_PICK_PREPARE" => s_BARE_PICK_PREPARE(v),
            "BARE_PICK_SIGN" => s_BARE_PICK_SIGN(v),
            "BARE_PICK_TAKE" => s_BARE_PICK_TAKE(v),
            "BARE_PICK_WEAR" => s_BARE_PICK_WEAR(v),
            "BARE_PICK_WEAR_COLD" => s_BARE_PICK_WEAR_COLD(v),
            "BARE_PICK_WEAR_HOT" => s_BARE_PICK_WEAR_HOT(v),
            "BARE_PICK_WEAR_WET" => s_BARE_PICK_WEAR_WET(v),
            "BARE_PICK_WHICH" => s_BARE_PICK_WHICH(v),
            "BECOMING" => s_BECOMING(v),
            "BECOMING_STATE" => s_BECOMING_STATE(v),
            "BEHIND_ON_MONEY" => s_BEHIND_ON_MONEY(v),
            "BLACKED_OUT_SIGHT" => s_BLACKED_OUT_SIGHT(v),
            "BODY_ALSO" => s_BODY_ALSO(v),
            "BODY_BUNGED" => s_BODY_BUNGED(v),
            "BODY_CLAMMY" => s_BODY_CLAMMY(v),
            "BODY_COLD" => s_BODY_COLD(v),
            "BODY_HOT" => s_BODY_HOT(v),
            "BODY_HUNGRY" => s_BODY_HUNGRY(v),
            "BODY_IS_NOT_A_MACHINE" => s_BODY_IS_NOT_A_MACHINE(v),
            "BODY_LEAD" => s_BODY_LEAD(v),
            "BODY_LIGHT_HEAD" => s_BODY_LIGHT_HEAD(v),
            "BODY_PALE" => s_BODY_PALE(v),
            "BODY_PARTS" => s_BODY_PARTS(v),
            "BODY_PART_QUALIFIER" => s_BODY_PART_QUALIFIER(v),
            "BODY_PART_TAIL" => s_BODY_PART_TAIL(v),
            "BODY_POOLS" => s_BODY_POOLS(v),
            "BODY_PUFFY" => s_BODY_PUFFY(v),
            "BODY_QUEASY" => s_BODY_QUEASY(v),
            "BODY_SAYS_WHY" => s_BODY_SAYS_WHY(v),
            "BODY_SITUATION" => s_BODY_SITUATION(v),
            "BODY_SPOTTY" => s_BODY_SPOTTY(v),
            "BODY_STATES" => s_BODY_STATES(v),
            "BODY_SUBJECT" => s_BODY_SUBJECT(v),
            "BODY_TAIL" => s_BODY_TAIL(v),
            "BODY_THIRSTY" => s_BODY_THIRSTY(v),
            "BODY_WORD_FOR_A_THING" => s_BODY_WORD_FOR_A_THING(v),
            "BOUGHT_VERB" => s_BOUGHT_VERB(v),
            "BOUND_FOR" => s_BOUND_FOR(v),
            "BURN_AFTER" => s_BURN_AFTER(v),
            "BURN_BLISTER" => s_BURN_BLISTER(v),
            "BUYING_WORDS" => s_BUYING_WORDS(v),
            "CALL_ANSWER" => s_CALL_ANSWER(v),
            "CALL_WORDS" => s_CALL_WORDS(v),
            "CAME_BACK" => s_CAME_BACK(v),
            "CARD_LINES" => s_CARD_LINES(v),
            "CARD_VALIDITY" => s_CARD_VALIDITY(v),
            "CARE_AFTER" => s_CARE_AFTER(v),
            "CARE_AFTER_ANIMAL" => s_CARE_AFTER_ANIMAL(v),
            "CARE_AFTER_ANIMAL_STILL" => s_CARE_AFTER_ANIMAL_STILL(v),
            "CARE_AFTER_STILL" => s_CARE_AFTER_STILL(v),
            "CARE_BETTER" => s_CARE_BETTER(v),
            "CARE_HERE" => s_CARE_HERE(v),
            "CARE_HURTS" => s_CARE_HURTS(v),
            "CARE_INJURY" => s_CARE_INJURY(v),
            "CARE_MODULES" => s_CARE_MODULES(v),
            "CARE_NOT_RESOLVED" => s_CARE_NOT_RESOLVED(v),
            "CARE_PHRASES" => s_CARE_PHRASES(v),
            "CARE_POINTER" => s_CARE_POINTER(v),
            "CARE_PRONOUNS" => s_CARE_PRONOUNS(v),
            "CARE_RESOLVED" => s_CARE_RESOLVED(v),
            "CARE_ROUTE_BACK" => s_CARE_ROUTE_BACK(v),
            "CARE_SEEN" => s_CARE_SEEN(v),
            "CARE_SEEN_ANIMAL" => s_CARE_SEEN_ANIMAL(v),
            "CARE_THREAD" => s_CARE_THREAD(v),
            "CARE_VISIT" => s_CARE_VISIT(v),
            "CARE_VISIT_WARD" => s_CARE_VISIT_WARD(v),
            "CARE_WORDS" => s_CARE_WORDS(v),
            "CARING" => s_CARING(v),
            "CARING_FOR_A_BODY" => s_CARING_FOR_A_BODY(v),
            "CHECK_PHRASES" => s_CHECK_PHRASES(v),
            "CHILD_AT_HOME" => s_CHILD_AT_HOME(v),
            "CHILD_FAINT" => s_CHILD_FAINT(v),
            "CHILD_FAINT_ANSWER" => s_CHILD_FAINT_ANSWER(v),
            "CHOICE_BETWEEN_PEOPLE" => s_CHOICE_BETWEEN_PEOPLE(v),
            "CHOICE_HEALTH" => s_CHOICE_HEALTH(v),
            "CHOICE_LIGHT" => s_CHOICE_LIGHT(v),
            "CHOICE_LIGHT_PICK" => s_CHOICE_LIGHT_PICK(v),
            "CHOICE_PEOPLE" => s_CHOICE_PEOPLE(v),
            "CHOICE_PHARMACIST" => s_CHOICE_PHARMACIST(v),
            "CHOICE_STAKES" => s_CHOICE_STAKES(v),
            "CHOICE_THERAPY" => s_CHOICE_THERAPY(v),
            "CHOICE_WEIGH" => s_CHOICE_WEIGH(v),
            "CLOSED_DOOR" => s_CLOSED_DOOR(v),
            "COLD_OPEN" => s_COLD_OPEN(v),
            "COMFORT_SAID" => s_COMFORT_SAID(v),
            "COMPLAINT_WORDS" => s_COMPLAINT_WORDS(v),
            "COMPLAIN_WHO" => s_COMPLAIN_WHO(v),
            "CONGRATS" => s_CONGRATS(v),
            "CONTACT_ANSWER" => s_CONTACT_ANSWER(v),
            "CONTRAST" => s_CONTRAST(v),
            "COPULA" => s_COPULA(v),
            "CRISIS_AFTER" => s_CRISIS_AFTER(v),
            "CRISIS_AFTER_THEM" => s_CRISIS_AFTER_THEM(v),
            "CRISIS_BETTER" => s_CRISIS_BETTER(v),
            "CRISIS_BETTER_THEM" => s_CRISIS_BETTER_THEM(v),
            "CRISIS_WORDS" => s_CRISIS_WORDS(v),
            "CROWD" => s_CROWD(v),
            "CUT_FOR" => s_CUT_FOR(v),
            "DARK_THING" => s_DARK_THING(v),
            "DATES_KEEP_SLIPPING" => s_DATES_KEEP_SLIPPING(v),
            "DECLINED_PLAN" => s_DECLINED_PLAN(v),
            "DECLINED_PLAN_GENTLE" => s_DECLINED_PLAN_GENTLE(v),
            "DENTIST_WHO" => s_DENTIST_WHO(v),
            "DETAILS_IN_A_WARM_LINE" => s_DETAILS_IN_A_WARM_LINE(v),
            "DEVICE_WHO" => s_DEVICE_WHO(v),
            "DIE_OF_IT" => s_DIE_OF_IT(v),
            "DISH_DETERMINER" => s_DISH_DETERMINER(v),
            "DISH_PARTICIPLE" => s_DISH_PARTICIPLE(v),
            "DITHER_ABOUT_A_PERSON" => s_DITHER_ABOUT_A_PERSON(v),
            "DITHER_DECIDE" => s_DITHER_DECIDE(v),
            "DITHER_OPEN_LINES" => s_DITHER_OPEN_LINES(v),
            "DITHER_SUBJECT" => s_DITHER_SUBJECT(v),
            "DITHER_TWO_WAY_LINES" => s_DITHER_TWO_WAY_LINES(v),
            "DONE_NOT_A_PERSON" => s_DONE_NOT_A_PERSON(v),
            "DONE_NOT_THIS" => s_DONE_NOT_THIS(v),
            "DONE_ONLY_A_THING" => s_DONE_ONLY_A_THING(v),
            "DONE_VERB" => s_DONE_VERB(v),
            "DOUBLE_BOOKED" => s_DOUBLE_BOOKED(v),
            "EASING_FILLER" => s_EASING_FILLER(v),
            "EATEN_KIND" => s_EATEN_KIND(v),
            "EATING" => s_EATING(v),
            "ELDER_AID_REPLIES" => s_ELDER_AID_REPLIES(v),
            "ELDER_CLINIC_REPLIES" => s_ELDER_CLINIC_REPLIES(v),
            "ELDER_DOCTOR_REPLIES" => s_ELDER_DOCTOR_REPLIES(v),
            "ELDER_HELP_REPLIES" => s_ELDER_HELP_REPLIES(v),
            "EMBARRASSED" => s_EMBARRASSED(v),
            "EMBARRASSMENT_WORD" => s_EMBARRASSMENT_WORD(v),
            "EMERGENCY_AFTER" => s_EMERGENCY_AFTER(v),
            "EMERGENCY_ANSWER" => s_EMERGENCY_ANSWER(v),
            "EMERGENCY_BETTER" => s_EMERGENCY_BETTER(v),
            "EMERGENCY_BETTER_THEM" => s_EMERGENCY_BETTER_THEM(v),
            "EMERGENCY_NOT_WORKED" => s_EMERGENCY_NOT_WORKED(v),
            "EMERGENCY_WORDS" => s_EMERGENCY_WORDS(v),
            "EMPTY" => s_EMPTY(v),
            "ENDINGS" => s_ENDINGS(v),
            "EXPERT_WORDS" => s_EXPERT_WORDS(v),
            "FAINT_COMING_ANSWER" => s_FAINT_COMING_ANSWER(v),
            "FALLBACK_MODULE" => s_FALLBACK_MODULE(v),
            "FAVOURITE_GENERAL" => s_FAVOURITE_GENERAL(v),
            "FAVOURITE_WORDS" => s_FAVOURITE_WORDS(v),
            "FILLERS" => s_FILLERS(v),
            "FILLING_FOODS" => s_FILLING_FOODS(v),
            "FILLING_WORDS" => s_FILLING_WORDS(v),
            "FIRST_PERSON" => s_FIRST_PERSON(v),
            "FLAT" => s_FLAT(v),
            "FLAT_CORE" => s_FLAT_CORE(v),
            "FLAT_LEAD" => s_FLAT_LEAD(v),
            "FLOPPY_WAKE" => s_FLOPPY_WAKE(v),
            "FORECAST" => s_FORECAST(v),
            "FORECAST_AFTER_SAFETY" => s_FORECAST_AFTER_SAFETY(v),
            "FORECAST_A_FALL" => s_FORECAST_A_FALL(v),
            "FORECAST_BODY_WORDS" => s_FORECAST_BODY_WORDS(v),
            "FORECAST_DURATION" => s_FORECAST_DURATION(v),
            "FORECAST_HE_SHE" => s_FORECAST_HE_SHE(v),
            "FORECAST_ILL" => s_FORECAST_ILL(v),
            "FORECAST_I_ASK" => s_FORECAST_I_ASK(v),
            "FORECAST_OFF_THEIR_FOOD" => s_FORECAST_OFF_THEIR_FOOD(v),
            "FORECAST_PERMISSION" => s_FORECAST_PERMISSION(v),
            "FORECAST_PERSON" => s_FORECAST_PERSON(v),
            "FORECAST_PET" => s_FORECAST_PET(v),
            "FORECAST_PET_HARM" => s_FORECAST_PET_HARM(v),
            "FORECAST_POOLS" => s_FORECAST_POOLS(v),
            "FORECAST_RELATION" => s_FORECAST_RELATION(v),
            "FORECAST_SELF" => s_FORECAST_SELF(v),
            "FORECAST_SELF_STRUGGLING" => s_FORECAST_SELF_STRUGGLING(v),
            "FORECAST_SOMEONE_STRUGGLING" => s_FORECAST_SOMEONE_STRUGGLING(v),
            "FORECAST_STRUGGLE" => s_FORECAST_STRUGGLE(v),
            "FORGETTING" => s_FORGETTING(v),
            "FORGET_A_THING" => s_FORGET_A_THING(v),
            "FOUND_IT" => s_FOUND_IT(v),
            "FRAME_WORDS" => s_FRAME_WORDS(v),
            "FUEL_WORDS" => s_FUEL_WORDS(v),
            "FUME_WORDS" => s_FUME_WORDS(v),
            "GADGET_WORDS" => s_GADGET_WORDS(v),
            "GAS_ANSWER" => s_GAS_ANSWER(v),
            "GAS_WORDS" => s_GAS_WORDS(v),
            "GENERIC_WORDS" => s_GENERIC_WORDS(v),
            "GIFT_WORDS" => s_GIFT_WORDS(v),
            "GOING_TO_DIE" => s_GOING_TO_DIE(v),
            "GONE_VERB" => s_GONE_VERB(v),
            "GOOD_TIME_WORDS" => s_GOOD_TIME_WORDS(v),
            "GO_ANSWER" => s_GO_ANSWER(v),
            "GO_TO_BE_SEEN" => s_GO_TO_BE_SEEN(v),
            "GREETING" => s_GREETING(v),
            "GREETING_AFTER" => s_GREETING_AFTER(v),
            "GREETING_WORDS" => s_GREETING_WORDS(v),
            "GRIEF_AFTER" => s_GRIEF_AFTER(v),
            "GRIEVANCE" => s_GRIEVANCE(v),
            "GRIEVANCE_HARM" => s_GRIEVANCE_HARM(v),
            "GRIEVANCE_HARM_PHRASES" => s_GRIEVANCE_HARM_PHRASES(v),
            "GRIEVANCE_HEALTH" => s_GRIEVANCE_HEALTH(v),
            "GRIEVANCE_HEALTH_PHRASES" => s_GRIEVANCE_HEALTH_PHRASES(v),
            "GRIEVANCE_IMPACT" => s_GRIEVANCE_IMPACT(v),
            "GRIEVANCE_NOT_A_THING" => s_GRIEVANCE_NOT_A_THING(v),
            "GRIEVANCE_PEOPLE" => s_GRIEVANCE_PEOPLE(v),
            "GRIEVANCE_PHRASES" => s_GRIEVANCE_PHRASES(v),
            "GRIEVANCE_PRONOUN" => s_GRIEVANCE_PRONOUN(v),
            "GRIEVANCE_STRAIN" => s_GRIEVANCE_STRAIN(v),
            "GRIEVANCE_STRAIN_SAID" => s_GRIEVANCE_STRAIN_SAID(v),
            "GROWN_UP_WORDS" => s_GROWN_UP_WORDS(v),
            "HABIT_ACT" => s_HABIT_ACT(v),
            "HABIT_PROBLEM_ACT" => s_HABIT_PROBLEM_ACT(v),
            "HABIT_REPLY" => s_HABIT_REPLY(v),
            "HAND" => s_HAND(v),
            "HANDED_BACK" => s_HANDED_BACK(v),
            "HANDS_BACK" => s_HANDS_BACK(v),
            "HAZARD_AFTER" => s_HAZARD_AFTER(v),
            "HAZARD_WORDS" => s_HAZARD_WORDS(v),
            "HEALTH_RECORD" => s_HEALTH_RECORD(v),
            "HEALTH_RECORD_LINES" => s_HEALTH_RECORD_LINES(v),
            "HEALTH_WHO" => s_HEALTH_WHO(v),
            "HEALTH_WORDS" => s_HEALTH_WORDS(v),
            "HEALTH_WORLD" => s_HEALTH_WORLD(v),
            "HEAVY_NEWS" => s_HEAVY_NEWS(v),
            "HEDGE_POOL" => s_HEDGE_POOL(v),
            "HELLO_STILL_THERE" => s_HELLO_STILL_THERE(v),
            "HELLO_WITH_YOU" => s_HELLO_WITH_YOU(v),
            "HELP" => s_HELP(v),
            "HELPLINE_NOT_AN_AMBULANCE" => s_HELPLINE_NOT_AN_AMBULANCE(v),
            "HELP_ALREADY" => s_HELP_ALREADY(v),
            "HELP_TAIL" => s_HELP_TAIL(v),
            "HELP_TOPIC" => s_HELP_TOPIC(v),
            "HELP_TOPIC_ANY" => s_HELP_TOPIC_ANY(v),
            "HOLD_AFTER" => s_HOLD_AFTER(v),
            "HOUSE_VERB" => s_HOUSE_VERB(v),
            "HOW_ARE_YOU" => s_HOW_ARE_YOU(v),
            "HOW_ARE_YOU_ASKS" => s_HOW_ARE_YOU_ASKS(v),
            "HOW_ARE_YOU_GOING_ON" => s_HOW_ARE_YOU_GOING_ON(v),
            "HOW_ARE_YOU_GOING_ON_AFTER" => s_HOW_ARE_YOU_GOING_ON_AFTER(v),
            "HOW_ARE_YOU_TAGS" => s_HOW_ARE_YOU_TAGS(v),
            "HOW_TO_DECIDE_FILLER" => s_HOW_TO_DECIDE_FILLER(v),
            "HOW_TO_DECIDE_LINES" => s_HOW_TO_DECIDE_LINES(v),
            "HUMAN_EMPTY" => s_HUMAN_EMPTY(v),
            "HURT_EVENT" => s_HURT_EVENT(v),
            "IDENTITY" => s_IDENTITY(v),
            "IDIOM_GLAD" => s_IDIOM_GLAD(v),
            "IDIOM_SHY" => s_IDIOM_SHY(v),
            "ILL_AFTER" => s_ILL_AFTER(v),
            "ILL_BEFORE" => s_ILL_BEFORE(v),
            "ILL_SAID_OF_A_BODY" => s_ILL_SAID_OF_A_BODY(v),
            "ILL_WILL_VERBS" => s_ILL_WILL_VERBS(v),
            "IMPERATIVE" => s_IMPERATIVE(v),
            "INVITED" => s_INVITED(v),
            "INVITED_TO_TALK" => s_INVITED_TO_TALK(v),
            "INVITE_ACTIVITY" => s_INVITE_ACTIVITY(v),
            "INVITE_FILLER" => s_INVITE_FILLER(v),
            "INVITE_NEVER" => s_INVITE_NEVER(v),
            "INVITE_PRONOUN" => s_INVITE_PRONOUN(v),
            "IN_A_ROOM" => s_IN_A_ROOM(v),
            "I_RATE_A_SKILL" => s_I_RATE_A_SKILL(v),
            "JOINT_WORDS" => s_JOINT_WORDS(v),
            "JUDGE_ASKS_FINE" => s_JUDGE_ASKS_FINE(v),
            "JUDGE_ASKS_MAD" => s_JUDGE_ASKS_MAD(v),
            "JUDGE_ASKS_SLIGHT" => s_JUDGE_ASKS_SLIGHT(v),
            "JUDGE_BARE_FACES" => s_JUDGE_BARE_FACES(v),
            "JUDGE_FINE" => s_JUDGE_FINE(v),
            "JUDGE_FINE_PLAIN" => s_JUDGE_FINE_PLAIN(v),
            "JUDGE_LEAD" => s_JUDGE_LEAD(v),
            "JUDGE_NOD_WORDS" => s_JUDGE_NOD_WORDS(v),
            "JUDGE_NOT_A_GERUND" => s_JUDGE_NOT_A_GERUND(v),
            "JUDGE_NOT_A_VERDICT" => s_JUDGE_NOT_A_VERDICT(v),
            "JUDGE_NOUN" => s_JUDGE_NOUN(v),
            "JUDGE_NO_PLAN" => s_JUDGE_NO_PLAN(v),
            "JUDGE_NO_PLAN_FINE" => s_JUDGE_NO_PLAN_FINE(v),
            "JUDGE_NO_PLAN_MAD" => s_JUDGE_NO_PLAN_MAD(v),
            "JUDGE_NO_PLAN_MIND" => s_JUDGE_NO_PLAN_MIND(v),
            "JUDGE_ONLY_A_PLAN" => s_JUDGE_ONLY_A_PLAN(v),
            "JUDGE_PLAN_HAS_A_HOLE" => s_JUDGE_PLAN_HAS_A_HOLE(v),
            "JUDGE_PLAN_STOP" => s_JUDGE_PLAN_STOP(v),
            "JUDGE_SANE" => s_JUDGE_SANE(v),
            "JUDGE_SANE_PLAIN" => s_JUDGE_SANE_PLAIN(v),
            "JUDGE_SELF_JUDGING" => s_JUDGE_SELF_JUDGING(v),
            "JUDGE_SOUNDS_FINE" => s_JUDGE_SOUNDS_FINE(v),
            "JUDGE_WISH_ANGRY" => s_JUDGE_WISH_ANGRY(v),
            "JUDGE_WORTH" => s_JUDGE_WORTH(v),
            "JUDGE_WORTH_PLAIN" => s_JUDGE_WORTH_PLAIN(v),
            "JUNK_CUES" => s_JUNK_CUES(v),
            "KEEPS_FORGETTING" => s_KEEPS_FORGETTING(v),
            "KNOCKED" => s_KNOCKED(v),
            "KNOCKED_SPOTS" => s_KNOCKED_SPOTS(v),
            "LANDED_BAD" => s_LANDED_BAD(v),
            "LANGUAGE_FLUENCY" => s_LANGUAGE_FLUENCY(v),
            "LATE_BEHIND_LINES" => s_LATE_BEHIND_LINES(v),
            "LATE_NOW_LINES" => s_LATE_NOW_LINES(v),
            "LAUGHED_AT" => s_LAUGHED_AT(v),
            "LAUGHED_AT_THEM" => s_LAUGHED_AT_THEM(v),
            "LAUGHING_WORDS" => s_LAUGHING_WORDS(v),
            "LEFT_BEHIND" => s_LEFT_BEHIND(v),
            "LEFT_POOL" => s_LEFT_POOL(v),
            "LEFT_POOL_GONE" => s_LEFT_POOL_GONE(v),
            "LIVE_ASK" => s_LIVE_ASK(v),
            "LIVE_DELAY_WORDS" => s_LIVE_DELAY_WORDS(v),
            "LIVE_HOURS_ASK" => s_LIVE_HOURS_ASK(v),
            "LIVE_INDOOR" => s_LIVE_INDOOR(v),
            "LIVE_JOURNEY_ENDS" => s_LIVE_JOURNEY_ENDS(v),
            "LIVE_KINDS" => s_LIVE_KINDS(v),
            "LIVE_NEWS_REPLY" => s_LIVE_NEWS_REPLY(v),
            "LIVE_NOT" => s_LIVE_NOT(v),
            "LIVE_NOT_HOURS" => s_LIVE_NOT_HOURS(v),
            "LIVE_OPEN_CLOSE" => s_LIVE_OPEN_CLOSE(v),
            "LIVE_PAPER_WORDS" => s_LIVE_PAPER_WORDS(v),
            "LIVE_ROADS_NOT" => s_LIVE_ROADS_NOT(v),
            "LIVE_ROAD_NAME" => s_LIVE_ROAD_NAME(v),
            "LIVE_ROAD_WORDS" => s_LIVE_ROAD_WORDS(v),
            "LIVE_SETTING" => s_LIVE_SETTING(v),
            "LIVE_SITE_THING" => s_LIVE_SITE_THING(v),
            "LIVE_SKY" => s_LIVE_SKY(v),
            "LIVE_SKY_OFF" => s_LIVE_SKY_OFF(v),
            "LIVE_SKY_WHY" => s_LIVE_SKY_WHY(v),
            "LIVE_SOON" => s_LIVE_SOON(v),
            "LIVE_STRONG" => s_LIVE_STRONG(v),
            "LIVE_TAKE_ALLOWED" => s_LIVE_TAKE_ALLOWED(v),
            "LIVE_TAKE_ASK" => s_LIVE_TAKE_ASK(v),
            "LIVE_TRAVEL_WORDS" => s_LIVE_TRAVEL_WORDS(v),
            "LIVE_VENUE" => s_LIVE_VENUE(v),
            "LIVE_VENUE_RE" => s_LIVE_VENUE_RE(v),
            "LIVE_WEAR" => s_LIVE_WEAR(v),
            "LIVE_WEATHER_WORDS" => s_LIVE_WEATHER_WORDS(v),
            "LIVE_YES_NO_OPEN" => s_LIVE_YES_NO_OPEN(v),
            "LOCKED_OUT_LINES" => s_LOCKED_OUT_LINES(v),
            "LOCKED_OUT_OF_THE_CAR" => s_LOCKED_OUT_OF_THE_CAR(v),
            "LOST_A_THING" => s_LOST_A_THING(v),
            "LOST_A_THING_AGAIN" => s_LOST_A_THING_AGAIN(v),
            "LOW_DAY_REPLIES" => s_LOW_DAY_REPLIES(v),
            "LUMP" => s_LUMP(v),
            "MANNERS_ONLY" => s_MANNERS_ONLY(v),
            "MEDS_WORDS" => s_MEDS_WORDS(v),
            "MEMORY" => s_MEMORY(v),
            "MESS_PLACES" => s_MESS_PLACES(v),
            "MILESTONE" => s_MILESTONE(v),
            "MILESTONE_EVENT" => s_MILESTONE_EVENT(v),
            "MISHAP_BACK" => s_MISHAP_BACK(v),
            "MISSED_STEP_LINES" => s_MISSED_STEP_LINES(v),
            "MODEL_NAMED_WORDS" => s_MODEL_NAMED_WORDS(v),
            "MONEY_WORDS" => s_MONEY_WORDS(v),
            "MOOD_AFTER" => s_MOOD_AFTER(v),
            "MOOD_BEFORE" => s_MOOD_BEFORE(v),
            "MOUTH_WORDS" => s_MOUTH_WORDS(v),
            "MOVE_ON_AFTER" => s_MOVE_ON_AFTER(v),
            "MYSELF" => s_MYSELF(v),
            "MY_BILL_LINES" => s_MY_BILL_LINES(v),
            "MY_DOING" => s_MY_DOING(v),
            "MY_OWN" => s_MY_OWN(v),
            "MY_OWN_DAY" => s_MY_OWN_DAY(v),
            "MY_PICK" => s_MY_PICK(v),
            "MY_PICK_ASK" => s_MY_PICK_ASK(v),
            "MY_PICK_NOTE" => s_MY_PICK_NOTE(v),
            "MY_RECORDS" => s_MY_RECORDS(v),
            "MY_WORKINGS" => s_MY_WORKINGS(v),
            "NAMED_BODIES" => s_NAMED_BODIES(v),
            "NAMED_NEGATIVE" => s_NAMED_NEGATIVE(v),
            "NAMED_TYPES" => s_NAMED_TYPES(v),
            "NARROW_ANSWERED" => s_NARROW_ANSWERED(v),
            "NARROW_SUBSTANCE" => s_NARROW_SUBSTANCE(v),
            "NARROW_WORDS" => s_NARROW_WORDS(v),
            "NEGATIVE" => s_NEGATIVE(v),
            "NEGATIVE_CUES" => s_NEGATIVE_CUES(v),
            "NEGATORS" => s_NEGATORS(v),
            "NETWORK_WORDS" => s_NETWORK_WORDS(v),
            "NEUTRAL" => s_NEUTRAL(v),
            "NEVER_MIND" => s_NEVER_MIND(v),
            "NEWS_TO_COME" => s_NEWS_TO_COME(v),
            "NOTHING_AT_ALL" => s_NOTHING_AT_ALL(v),
            "NOTHING_SAID_YET" => s_NOTHING_SAID_YET(v),
            "NOT_AN_ANSWER" => s_NOT_AN_ANSWER(v),
            "NOT_A_FORM" => s_NOT_A_FORM(v),
            "NOT_A_NAME" => s_NOT_A_NAME(v),
            "NOT_A_VERDICT_SUBJECT" => s_NOT_A_VERDICT_SUBJECT(v),
            "NOT_CHOSEN" => s_NOT_CHOSEN(v),
            "NOT_ENOUGH_REPLY" => s_NOT_ENOUGH_REPLY(v),
            "NOT_FIT_ENOUGH" => s_NOT_FIT_ENOUGH(v),
            "NOT_NOTED" => s_NOT_NOTED(v),
            "NOT_PEOPLE_WORDS" => s_NOT_PEOPLE_WORDS(v),
            "NOT_THEIR_VERB" => s_NOT_THEIR_VERB(v),
            "NOT_TOO_OLD" => s_NOT_TOO_OLD(v),
            "NOT_WHAT_I_SAID" => s_NOT_WHAT_I_SAID(v),
            "NO_DIRECTION_REPLY" => s_NO_DIRECTION_REPLY(v),
            "NO_MORE_OF" => s_NO_MORE_OF(v),
            "NO_WORDS_ASKING" => s_NO_WORDS_ASKING(v),
            "NO_WORDS_AT_THE_CARE" => s_NO_WORDS_AT_THE_CARE(v),
            "NO_WORDS_SOMEONE" => s_NO_WORDS_SOMEONE(v),
            "NO_WORDS_STUCK" => s_NO_WORDS_STUCK(v),
            "NUMBER_PLACES" => s_NUMBER_PLACES(v),
            "OCCASION" => s_OCCASION(v),
            "OFFICE_RECORD_LINES" => s_OFFICE_RECORD_LINES(v),
            "ONE_SKILL_RATED" => s_ONE_SKILL_RATED(v),
            "ONGOING_WORDS" => s_ONGOING_WORDS(v),
            "ONLINE_ASK" => s_ONLINE_ASK(v),
            "ONLY_FRONT" => s_ONLY_FRONT(v),
            "OPINION_ASKS" => s_OPINION_ASKS(v),
            "OURS" => s_OURS(v),
            "OUTCOME_LINES" => s_OUTCOME_LINES(v),
            "OWN_PERSON" => s_OWN_PERSON(v),
            "OWN_PLACE" => s_OWN_PLACE(v),
            "PARTICLE" => s_PARTICLE(v),
            "PASSED_OUT" => s_PASSED_OUT(v),
            "PASSED_OUT_ANSWER" => s_PASSED_OUT_ANSWER(v),
            "PASSED_OUT_PERSON" => s_PASSED_OUT_PERSON(v),
            "PASSING_OUT_A_THING" => s_PASSING_OUT_A_THING(v),
            "PASSWORD_TROUBLE" => s_PASSWORD_TROUBLE(v),
            "PAST_STATE_CUES" => s_PAST_STATE_CUES(v),
            "PAUSE" => s_PAUSE(v),
            "PAUSE_IN_A_SAFETY_THREAD" => s_PAUSE_IN_A_SAFETY_THREAD(v),
            "PENDING_DOUBT" => s_PENDING_DOUBT(v),
            "PENDING_LANDED" => s_PENDING_LANDED(v),
            "PENDING_WAITING" => s_PENDING_WAITING(v),
            "PENDING_WHEN" => s_PENDING_WHEN(v),
            "PERSON_WORDS" => s_PERSON_WORDS(v),
            "PEST_WHO" => s_PEST_WHO(v),
            "PETROL_WORDS" => s_PETROL_WORDS(v),
            "PET_CONDITION_TROUBLE" => s_PET_CONDITION_TROUBLE(v),
            "PET_LOW_REPLY" => s_PET_LOW_REPLY(v),
            "PET_TROUBLE" => s_PET_TROUBLE(v),
            "PICKS_A_MECHANISM" => s_PICKS_A_MECHANISM(v),
            "PICK_ONLY_WORDS" => s_PICK_ONLY_WORDS(v),
            "PLAIN_WORDS" => s_PLAIN_WORDS(v),
            "PLAN" => s_PLAN(v),
            "PLAN_DONE" => s_PLAN_DONE(v),
            "PLAN_DONE_ACK" => s_PLAN_DONE_ACK(v),
            "PLAN_KNOCKED" => s_PLAN_KNOCKED(v),
            "PLASTER_IT" => s_PLASTER_IT(v),
            "POINTING" => s_POINTING(v),
            "POINTS_LINES" => s_POINTS_LINES(v),
            "POSITIVE" => s_POSITIVE(v),
            "POSITIVE_CUES" => s_POSITIVE_CUES(v),
            "PRAISE" => s_PRAISE(v),
            "PREGNANCY_BLEEDING" => s_PREGNANCY_BLEEDING(v),
            "PRESENCE_LEFTOVER" => s_PRESENCE_LEFTOVER(v),
            "PRESENT" => s_PRESENT(v),
            "PRIVACY_ASK" => s_PRIVACY_ASK(v),
            "PRIVACY_SHAPES" => s_PRIVACY_SHAPES(v),
            "PUNT" => s_PUNT(v),
            "QUESTION_OPENERS" => s_QUESTION_OPENERS(v),
            "QUESTION_WORDS" => s_QUESTION_WORDS(v),
            "QUIET_MODULES" => s_QUIET_MODULES(v),
            "QUOTE_LINES" => s_QUOTE_LINES(v),
            "RATES_A_SKILL" => s_RATES_A_SKILL(v),
            "REACTIONS_CARE" => s_REACTIONS_CARE(v),
            "REACTIONS_SAID" => s_REACTIONS_SAID(v),
            "REACTION_DISBELIEF" => s_REACTION_DISBELIEF(v),
            "REACTION_DISBELIEF_SAID" => s_REACTION_DISBELIEF_SAID(v),
            "REACTION_LEAD" => s_REACTION_LEAD(v),
            "REACTION_MISHAP" => s_REACTION_MISHAP(v),
            "REACTION_MISHAP_SAID" => s_REACTION_MISHAP_SAID(v),
            "REACTION_OUCH" => s_REACTION_OUCH(v),
            "REACTION_OUCH_SAID" => s_REACTION_OUCH_SAID(v),
            "REACTION_RELIEF" => s_REACTION_RELIEF(v),
            "REACTION_RELIEF_SAID" => s_REACTION_RELIEF_SAID(v),
            "REACTION_REPEAT" => s_REACTION_REPEAT(v),
            "REACTION_REPEAT_FIRST" => s_REACTION_REPEAT_FIRST(v),
            "REACTION_WORD" => s_REACTION_WORD(v),
            "REACTION_WORDS" => s_REACTION_WORDS(v),
            "REFUSED_REPLIES" => s_REFUSED_REPLIES(v),
            "RELATIONSHIP_DECISION" => s_RELATIONSHIP_DECISION(v),
            "RELIEF" => s_RELIEF(v),
            "REPAIR_RULE" => s_REPAIR_RULE(v),
            "REPAIR_WHETHER_LINES" => s_REPAIR_WHETHER_LINES(v),
            "REPORT_FILLER" => s_REPORT_FILLER(v),
            "REQUEST_CUES" => s_REQUEST_CUES(v),
            "RESOLVED_AFTER" => s_RESOLVED_AFTER(v),
            "RESOLVED_AFTER_THEM" => s_RESOLVED_AFTER_THEM(v),
            "RETURNED_TO_A_THING" => s_RETURNED_TO_A_THING(v),
            "RETURNED_TO_IT" => s_RETURNED_TO_IT(v),
            "RETURNED_TO_IT_FIRST" => s_RETURNED_TO_IT_FIRST(v),
            "RING_FOR_HELP" => s_RING_FOR_HELP(v),
            "RING_THE_POLICE" => s_RING_THE_POLICE(v),
            "ROUTINE" => s_ROUTINE(v),
            "ROUTINE_NOT" => s_ROUTINE_NOT(v),
            "ROUTINE_PHRASES" => s_ROUTINE_PHRASES(v),
            "ROUTINE_WORDS" => s_ROUTINE_WORDS(v),
            "RULED_OUT_DETERMINER" => s_RULED_OUT_DETERMINER(v),
            "RUMINATION_REPLIES" => s_RUMINATION_REPLIES(v),
            "RUN_OUT_LINES" => s_RUN_OUT_LINES(v),
            "SAFETY_WORDS" => s_SAFETY_WORDS(v),
            "SAME_AGAIN" => s_SAME_AGAIN(v),
            "SAME_AGAIN_WITH_THE_ANSWER" => s_SAME_AGAIN_WITH_THE_ANSWER(v),
            "SCHOOL_TROUBLE" => s_SCHOOL_TROUBLE(v),
            "SCHOOL_WHO" => s_SCHOOL_WHO(v),
            "SCHOOL_WORDS" => s_SCHOOL_WORDS(v),
            "SECRETS_KEEP_SLIPPING" => s_SECRETS_KEEP_SLIPPING(v),
            "SEEN_TO_VERB" => s_SEEN_TO_VERB(v),
            "SELF_BLAME" => s_SELF_BLAME(v),
            "SELF_BLAME_ASKS" => s_SELF_BLAME_ASKS(v),
            "SELF_EATING_WORDS" => s_SELF_EATING_WORDS(v),
            "SELF_FILLER" => s_SELF_FILLER(v),
            "SELF_GROUP" => s_SELF_GROUP(v),
            "SELF_JOB" => s_SELF_JOB(v),
            "SELF_JUDGEMENT" => s_SELF_JUDGEMENT(v),
            "SELF_NOBODY" => s_SELF_NOBODY(v),
            "SELF_NOUN" => s_SELF_NOUN(v),
            "SELF_SUBJECT" => s_SELF_SUBJECT(v),
            "SICKNESS" => s_SICKNESS(v),
            "SOMEBODY_HURT" => s_SOMEBODY_HURT(v),
            "SOMEONE_ELSES_DAY" => s_SOMEONE_ELSES_DAY(v),
            "SOMEONE_ELSES_SHAME" => s_SOMEONE_ELSES_SHAME(v),
            "SOMEONE_GONE" => s_SOMEONE_GONE(v),
            "SOMEONE_LEAVING" => s_SOMEONE_LEAVING(v),
            "SOMEONE_SERIOUSLY_ILL" => s_SOMEONE_SERIOUSLY_ILL(v),
            "SOMETHING_CAME_APART" => s_SOMETHING_CAME_APART(v),
            "SORE_AMBIGUOUS" => s_SORE_AMBIGUOUS(v),
            "SORE_HANDS" => s_SORE_HANDS(v),
            "SORE_ORDINARY" => s_SORE_ORDINARY(v),
            "SORE_SERIOUS" => s_SORE_SERIOUS(v),
            "SORE_WORDS" => s_SORE_WORDS(v),
            "SORRY_FOR_ASKING" => s_SORRY_FOR_ASKING(v),
            "SORRY_ON_ITS_OWN" => s_SORRY_ON_ITS_OWN(v),
            "SORTED_AND_NO_CALL_YET" => s_SORTED_AND_NO_CALL_YET(v),
            "STATE_CUES" => s_STATE_CUES(v),
            "STILL_HERE_AFTER" => s_STILL_HERE_AFTER(v),
            "STRETCH_OF_TIME" => s_STRETCH_OF_TIME(v),
            "STUCK_AFTER_TROUBLE" => s_STUCK_AFTER_TROUBLE(v),
            "SUDDEN_LOSS" => s_SUDDEN_LOSS(v),
            "SUMMONS_RANG" => s_SUMMONS_RANG(v),
            "SUMMONS_SEEN" => s_SUMMONS_SEEN(v),
            "SUMMONS_WRITTEN" => s_SUMMONS_WRITTEN(v),
            "SYMPTOM" => s_SYMPTOM(v),
            "SYMPTOM_COLD" => s_SYMPTOM_COLD(v),
            "SYMPTOM_WORDS" => s_SYMPTOM_WORDS(v),
            "TAG_CONFIRM" => s_TAG_CONFIRM(v),
            "TAG_UNSURE" => s_TAG_UNSURE(v),
            "TASTES" => s_TASTES(v),
            "TASTE_GENERAL" => s_TASTE_GENERAL(v),
            "TELL_HARD_LINES" => s_TELL_HARD_LINES(v),
            "TENANT_WHO" => s_TENANT_WHO(v),
            "THANKS" => s_THANKS(v),
            "THANKS_AFTER" => s_THANKS_AFTER(v),
            "THANKS_ONLY" => s_THANKS_ONLY(v),
            "THANKS_THEN_SAID" => s_THANKS_THEN_SAID(v),
            "THANKS_WORDS" => s_THANKS_WORDS(v),
            "THANK_WORDS" => s_THANK_WORDS(v),
            "THE_BOTS_OWN_ANSWER" => s_THE_BOTS_OWN_ANSWER(v),
            "THE_CALL_STANDS" => s_THE_CALL_STANDS(v),
            "THE_CALL_STANDS_ANYWAY" => s_THE_CALL_STANDS_ANYWAY(v),
            "THE_DAY" => s_THE_DAY(v),
            "THE_DAY_AHEAD" => s_THE_DAY_AHEAD(v),
            "THE_FIRST_HOUR_AND_A_CHILD" => s_THE_FIRST_HOUR_AND_A_CHILD(v),
            "THE_GROUND_OPENS" => s_THE_GROUND_OPENS(v),
            "THE_MOMENT_IS_THEIRS" => s_THE_MOMENT_IS_THEIRS(v),
            "THE_PLACE_ASK" => s_THE_PLACE_ASK(v),
            "THE_PLACE_POOLS" => s_THE_PLACE_POOLS(v),
            "THING_AFTER" => s_THING_AFTER(v),
            "THING_STATES" => s_THING_STATES(v),
            "THING_SUBJECT" => s_THING_SUBJECT(v),
            "THREAD" => s_THREAD(v),
            "THREAD_PEOPLE" => s_THREAD_PEOPLE(v),
            "THREAD_TURNS" => s_THREAD_TURNS(v),
            "THRESHOLD_CROSSED" => s_THRESHOLD_CROSSED(v),
            "TIGHT_MONEY" => s_TIGHT_MONEY(v),
            "TIGHT_MONEY_MORE" => s_TIGHT_MONEY_MORE(v),
            "TIPS_AHEAD" => s_TIPS_AHEAD(v),
            "TIPS_CLAUSE" => s_TIPS_CLAUSE(v),
            "TIPS_LINES" => s_TIPS_LINES(v),
            "TIPS_OWNER" => s_TIPS_OWNER(v),
            "TIPS_THING" => s_TIPS_THING(v),
            "TIPS_UNIT" => s_TIPS_UNIT(v),
            "TIPS_VERB" => s_TIPS_VERB(v),
            "TIRED" => s_TIRED(v),
            "TOO_MUCH" => s_TOO_MUCH(v),
            "TOO_MUCH_PHRASES" => s_TOO_MUCH_PHRASES(v),
            "TRADER_WORDS" => s_TRADER_WORDS(v),
            "TRADES_WHO" => s_TRADES_WHO(v),
            "TRADE_OFF_LEAN" => s_TRADE_OFF_LEAN(v),
            "TREATMENT_HARM" => s_TREATMENT_HARM(v),
            "TRIED_HELPED_ELSEWHERE" => s_TRIED_HELPED_ELSEWHERE(v),
            "TRIED_HELPED_REPLIES" => s_TRIED_HELPED_REPLIES(v),
            "TRIED_IT_REPLIES" => s_TRIED_IT_REPLIES(v),
            "TRIED_NO_CHANGE_ELSEWHERE" => s_TRIED_NO_CHANGE_ELSEWHERE(v),
            "TRIED_NO_CHANGE_REPLIES" => s_TRIED_NO_CHANGE_REPLIES(v),
            "TRIED_SORTED_REPLIES" => s_TRIED_SORTED_REPLIES(v),
            "TRIED_WORSE_ELSEWHERE" => s_TRIED_WORSE_ELSEWHERE(v),
            "TRIED_WORSE_REPLIES" => s_TRIED_WORSE_REPLIES(v),
            "TROUBLE_CUES" => s_TROUBLE_CUES(v),
            "UNKNOWN" => s_UNKNOWN(v),
            "UNKNOWN_ABOUT_A_PET" => s_UNKNOWN_ABOUT_A_PET(v),
            "UNKNOWN_ABOUT_SOMEONE" => s_UNKNOWN_ABOUT_SOMEONE(v),
            "UNKNOWN_DECIDE" => s_UNKNOWN_DECIDE(v),
            "UNKNOWN_HEALTH" => s_UNKNOWN_HEALTH(v),
            "UNKNOWN_MONEY" => s_UNKNOWN_MONEY(v),
            "UNKNOWN_REALISTIC" => s_UNKNOWN_REALISTIC(v),
            "UNKNOWN_REALISTIC_CARE" => s_UNKNOWN_REALISTIC_CARE(v),
            "UNKNOWN_TOO_LATE" => s_UNKNOWN_TOO_LATE(v),
            "UNKNOWN_WEIRD_BARE" => s_UNKNOWN_WEIRD_BARE(v),
            "UNKNOWN_WHO" => s_UNKNOWN_WHO(v),
            "UNKNOWN_WORD" => s_UNKNOWN_WORD(v),
            "UNKNOWN_WORD_NAMED" => s_UNKNOWN_WORD_NAMED(v),
            "UNKNOWN_WORRY" => s_UNKNOWN_WORRY(v),
            "VERDICT_ANNOY" => s_VERDICT_ANNOY(v),
            "VERDICT_BAD" => s_VERDICT_BAD(v),
            "VERDICT_FIRST" => s_VERDICT_FIRST(v),
            "VERDICT_HEAVY" => s_VERDICT_HEAVY(v),
            "VERDICT_LUCK" => s_VERDICT_LUCK(v),
            "VERDICT_NUMB" => s_VERDICT_NUMB(v),
            "VERDICT_QUEUE" => s_VERDICT_QUEUE(v),
            "VERDICT_ROUGH" => s_VERDICT_ROUGH(v),
            "VERDICT_SETBACK" => s_VERDICT_SETBACK(v),
            "VERDICT_SHAME" => s_VERDICT_SHAME(v),
            "VERDICT_WORD" => s_VERDICT_WORD(v),
            "VET_WHO" => s_VET_WHO(v),
            "VIEW_AFTER_A_DEATH" => s_VIEW_AFTER_A_DEATH(v),
            "VIEW_ASK_LINES" => s_VIEW_ASK_LINES(v),
            "VIEW_COMPANY" => s_VIEW_COMPANY(v),
            "VIEW_DECIDE" => s_VIEW_DECIDE(v),
            "VIEW_HANDBACK" => s_VIEW_HANDBACK(v),
            "VIEW_LEAD_IN" => s_VIEW_LEAD_IN(v),
            "VIEW_LINES" => s_VIEW_LINES(v),
            "VIEW_MIRROR_LINES" => s_VIEW_MIRROR_LINES(v),
            "VIEW_NO_SUBJECT" => s_VIEW_NO_SUBJECT(v),
            "VIEW_ONLY_WORDS" => s_VIEW_ONLY_WORDS(v),
            "VIEW_ON_GOOD_NEWS" => s_VIEW_ON_GOOD_NEWS(v),
            "VISIT_PRONOUNS" => s_VISIT_PRONOUNS(v),
            "WALKED_INTO_POOL" => s_WALKED_INTO_POOL(v),
            "WASNT_THERE" => s_WASNT_THERE(v),
            "WHERE_SOMETHING_WRONG" => s_WHERE_SOMETHING_WRONG(v),
            "WHOLE_LIST_AFTER" => s_WHOLE_LIST_AFTER(v),
            "WHO_TO_ASK" => s_WHO_TO_ASK(v),
            "WHY_AFTER_TASTE" => s_WHY_AFTER_TASTE(v),
            "WILD_BIRDS" => s_WILD_BIRDS(v),
            "WIN_AND_A_COST" => s_WIN_AND_A_COST(v),
            "WIN_IN_A_SILENCE" => s_WIN_IN_A_SILENCE(v),
            "WITH_YOU_AFTER" => s_WITH_YOU_AFTER(v),
            "WORDING_LINES" => s_WORDING_LINES(v),
            "WORDING_SOMEONE" => s_WORDING_SOMEONE(v),
            "WORD_ANYWHERE" => s_WORD_ANYWHERE(v),
            "WORD_FIXES" => s_WORD_FIXES(v),
            "WORK_ABOUT" => s_WORK_ABOUT(v),
            "WORK_WHO" => s_WORK_WHO(v),
            "WORRY_FILLER" => s_WORRY_FILLER(v),
            "WORRY_WORDS" => s_WORRY_WORDS(v),
            "WOULD_HAVE_BEEN" => s_WOULD_HAVE_BEEN(v),
            "YOURS" => s_YOURS(v),
            "YOU_SUBJECTS" => s_YOU_SUBJECTS(v),
            "_ASKS_ABOUT_THE_JOINT" => s__ASKS_ABOUT_THE_JOINT(v),
            "_A_PERSON" => s__A_PERSON(v),
            "_A_SOCIAL_THING" => s__A_SOCIAL_THING(v),
            "_BOUNDED" => s__BOUNDED(v),
            "_CHOOSING_BETWEEN_TWO" => s__CHOOSING_BETWEEN_TWO(v),
            "_COPULA" => s__COPULA(v),
            "_CREATIVE" => s__CREATIVE(v),
            "_DAY_WORDS" => s__DAY_WORDS(v),
            "_ECHO_FILLER" => s__ECHO_FILLER(v),
            "_ISSUER" => s__ISSUER(v),
            "_LANDED" => s__LANDED(v),
            "_MONEY_NOUN" => s__MONEY_NOUN(v),
            "_NOT_AN_OWN_THING" => s__NOT_AN_OWN_THING(v),
            "_NO_DAY_OF_THEIR_OWN" => s__NO_DAY_OF_THEIR_OWN(v),
            "_OPENING_QUESTIONS" => s__OPENING_QUESTIONS(v),
            "_START_ASK" => s__START_ASK(v),
            "_SUBJECT_AFTER_AM" => s__SUBJECT_AFTER_AM(v),
            "_THING_DONE_LINES" => s__THING_DONE_LINES(v),
            "_THREAD_PEOPLE" => s__THREAD_PEOPLE(v),
            "_VERDICT_DET" => s__VERDICT_DET(v),
            "_VERDICT_LEAD" => s__VERDICT_LEAD(v),
            "_WAS" => s__WAS(v),
            "_WOUND" => s__WOUND(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.fallback_data", module_obj);
    pyrt::xrefs::reg_fn("compose.fallback_data", "the_place_the_answer_is_kept", py_func("the_place_the_answer_is_kept", |args: &[Py]| the_place_the_answer_is_kept(args)));
    pyrt::xrefs::reg_fn("compose.fallback_data", "the_bots_own_answer", py_func("the_bots_own_answer", |args: &[Py]| the_bots_own_answer(args)));
    pyrt::xrefs::reg_access("compose.fallback_data", "ABOUT", g_ABOUT, s_ABOUT);
    pyrt::xrefs::reg_access("compose.fallback_data", "ABOUT_ITSELF_KINDS", g_ABOUT_ITSELF_KINDS, s_ABOUT_ITSELF_KINDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ABOUT_SELF", g_ABOUT_SELF, s_ABOUT_SELF);
    pyrt::xrefs::reg_access("compose.fallback_data", "ABOUT_TALLY", g_ABOUT_TALLY, s_ABOUT_TALLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "ABOUT_THEM_BEFORE_COPULA", g_ABOUT_THEM_BEFORE_COPULA, s_ABOUT_THEM_BEFORE_COPULA);
    pyrt::xrefs::reg_access("compose.fallback_data", "ABOUT_WORDS", g_ABOUT_WORDS, s_ABOUT_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ABSENCE", g_ABSENCE, s_ABSENCE);
    pyrt::xrefs::reg_access("compose.fallback_data", "ABUSE_AFTER", g_ABUSE_AFTER, s_ABUSE_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "ACK", g_ACK, s_ACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "ACKS", g_ACKS, s_ACKS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ACK_AFTER_TROUBLE", g_ACK_AFTER_TROUBLE, s_ACK_AFTER_TROUBLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "ACK_WARM", g_ACK_WARM, s_ACK_WARM);
    pyrt::xrefs::reg_access("compose.fallback_data", "ACTS_FOR_ME", g_ACTS_FOR_ME, s_ACTS_FOR_ME);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_AGAIN", g_ADVICE_AGAIN, s_ADVICE_AGAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_COOL", g_ADVICE_COOL, s_ADVICE_COOL);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_COOL_WORDS", g_ADVICE_COOL_WORDS, s_ADVICE_COOL_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_LENGTH", g_ADVICE_LENGTH, s_ADVICE_LENGTH);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_LENGTH_BARE", g_ADVICE_LENGTH_BARE, s_ADVICE_LENGTH_BARE);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_TIME", g_ADVICE_TIME, s_ADVICE_TIME);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_TURNS", g_ADVICE_TURNS, s_ADVICE_TURNS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_WORDS", g_ADVICE_WORDS, s_ADVICE_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ADVICE_WORSE", g_ADVICE_WORSE, s_ADVICE_WORSE);
    pyrt::xrefs::reg_access("compose.fallback_data", "AFRAID_AFTER", g_AFRAID_AFTER, s_AFRAID_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "AFTER_RESOLVED", g_AFTER_RESOLVED, s_AFTER_RESOLVED);
    pyrt::xrefs::reg_access("compose.fallback_data", "AGREEMENT", g_AGREEMENT, s_AGREEMENT);
    pyrt::xrefs::reg_access("compose.fallback_data", "AGREEMENTS", g_AGREEMENTS, s_AGREEMENTS);
    pyrt::xrefs::reg_access("compose.fallback_data", "AIM_FAILING", g_AIM_FAILING, s_AIM_FAILING);
    pyrt::xrefs::reg_access("compose.fallback_data", "AIM_HEADS", g_AIM_HEADS, s_AIM_HEADS);
    pyrt::xrefs::reg_access("compose.fallback_data", "AIM_NEGATORS", g_AIM_NEGATORS, s_AIM_NEGATORS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ALIVE", g_ALIVE, s_ALIVE);
    pyrt::xrefs::reg_access("compose.fallback_data", "ANIMALS", g_ANIMALS, s_ANIMALS);
    pyrt::xrefs::reg_access("compose.fallback_data", "APOLOGY_ANSWER", g_APOLOGY_ANSWER, s_APOLOGY_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "ASKED_A_RATING", g_ASKED_A_RATING, s_ASKED_A_RATING);
    pyrt::xrefs::reg_access("compose.fallback_data", "ASKED_IF_GOOD", g_ASKED_IF_GOOD, s_ASKED_IF_GOOD);
    pyrt::xrefs::reg_access("compose.fallback_data", "ASKED_IF_I_SAW_LINES", g_ASKED_IF_I_SAW_LINES, s_ASKED_IF_I_SAW_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "ASKING_WORDS", g_ASKING_WORDS, s_ASKING_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ASK_ONLY", g_ASK_ONLY, s_ASK_ONLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "ASK_OUT_ANSWER", g_ASK_OUT_ANSWER, s_ASK_OUT_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "AT_IT_EXTRA", g_AT_IT_EXTRA, s_AT_IT_EXTRA);
    pyrt::xrefs::reg_access("compose.fallback_data", "AT_IT_NOT", g_AT_IT_NOT, s_AT_IT_NOT);
    pyrt::xrefs::reg_access("compose.fallback_data", "AT_THE_TILL", g_AT_THE_TILL, s_AT_THE_TILL);
    pyrt::xrefs::reg_access("compose.fallback_data", "AXIS_MIN", g_AXIS_MIN, s_AXIS_MIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_BAD_TIME_TOLD", g_A_BAD_TIME_TOLD, s_A_BAD_TIME_TOLD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_BETRAYAL", g_A_BETRAYAL, s_A_BETRAYAL);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_BIRTHDAY_STILL_TO_COME", g_A_BIRTHDAY_STILL_TO_COME, s_A_BIRTHDAY_STILL_TO_COME);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_BOOKING_AHEAD", g_A_BOOKING_AHEAD, s_A_BOOKING_AHEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_BOOKING_AHEAD_DATED", g_A_BOOKING_AHEAD_DATED, s_A_BOOKING_AHEAD_DATED);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CARE_HOME", g_A_CARE_HOME, s_A_CARE_HOME);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CARE_HOME_BRING", g_A_CARE_HOME_BRING, s_A_CARE_HOME_BRING);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CARE_HOME_GUILT", g_A_CARE_HOME_GUILT, s_A_CARE_HOME_GUILT);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CARE_HOME_MOVED", g_A_CARE_HOME_MOVED, s_A_CARE_HOME_MOVED);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CARE_HOME_VISITING", g_A_CARE_HOME_VISITING, s_A_CARE_HOME_VISITING);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CHILD_LEAVING", g_A_CHILD_LEAVING, s_A_CHILD_LEAVING);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CHILD_LEFT", g_A_CHILD_LEFT, s_A_CHILD_LEFT);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CORRECTION_TAKEN", g_A_CORRECTION_TAKEN, s_A_CORRECTION_TAKEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_COST_SOMEBODY_ASKS", g_A_COST_SOMEBODY_ASKS, s_A_COST_SOMEBODY_ASKS);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_CROWD_LAUGHED", g_A_CROWD_LAUGHED, s_A_CROWD_LAUGHED);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_DAY_AHEAD", g_A_DAY_AHEAD, s_A_DAY_AHEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_DAY_AHEAD_TENSE", g_A_DAY_AHEAD_TENSE, s_A_DAY_AHEAD_TENSE);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_DAY_ASKED_PLAIN", g_A_DAY_ASKED_PLAIN, s_A_DAY_ASKED_PLAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_DISMISSAL", g_A_DISMISSAL, s_A_DISMISSAL);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_DREAD", g_A_DREAD, s_A_DREAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_FREE_TIME_ASK", g_A_FREE_TIME_ASK, s_A_FREE_TIME_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_GOODNESS_ASKED_OF_ME", g_A_GOODNESS_ASKED_OF_ME, s_A_GOODNESS_ASKED_OF_ME);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_GOOD_TIME", g_A_GOOD_TIME, s_A_GOOD_TIME);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_GOOD_TIME_IN_A_CARE_THREAD", g_A_GOOD_TIME_IN_A_CARE_THREAD, s_A_GOOD_TIME_IN_A_CARE_THREAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_HEDGED_RATING", g_A_HEDGED_RATING, s_A_HEDGED_RATING);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_HOW_THE_TIME_WENT", g_A_HOW_THE_TIME_WENT, s_A_HOW_THE_TIME_WENT);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_JOB_ON", g_A_JOB_ON, s_A_JOB_ON);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_JOB_SOON", g_A_JOB_SOON, s_A_JOB_SOON);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_LAUGH", g_A_LAUGH, s_A_LAUGH);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_LOVE_FOR_PEOPLE", g_A_LOVE_FOR_PEOPLE, s_A_LOVE_FOR_PEOPLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_LOVE_FOR_SOMEONE", g_A_LOVE_FOR_SOMEONE, s_A_LOVE_FOR_SOMEONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_MOOD_WITH_A_CATCH", g_A_MOOD_WITH_A_CATCH, s_A_MOOD_WITH_A_CATCH);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_NAMED_AFTER", g_A_NAMED_AFTER, s_A_NAMED_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_NAMED_AFTER_THEM", g_A_NAMED_AFTER_THEM, s_A_NAMED_AFTER_THEM);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_NAME_STORY", g_A_NAME_STORY, s_A_NAME_STORY);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_NAME_TOLD", g_A_NAME_TOLD, s_A_NAME_TOLD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_NEWS_OF_SOMETHING_ELSE", g_A_NEWS_OF_SOMETHING_ELSE, s_A_NEWS_OF_SOMETHING_ELSE);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_NEWS_OF_WEATHER", g_A_NEWS_OF_WEATHER, s_A_NEWS_OF_WEATHER);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_PAUSE_WORD", g_A_PAUSE_WORD, s_A_PAUSE_WORD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_PERSON_LAUGHED", g_A_PERSON_LAUGHED, s_A_PERSON_LAUGHED);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_PLACE_I_DONT_KNOW", g_A_PLACE_I_DONT_KNOW, s_A_PLACE_I_DONT_KNOW);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_PLAN_THAT_FELL_THROUGH", g_A_PLAN_THAT_FELL_THROUGH, s_A_PLAN_THAT_FELL_THROUGH);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_RECEIPT_OF_NOTHING", g_A_RECEIPT_OF_NOTHING, s_A_RECEIPT_OF_NOTHING);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_REST_OF_A_GOOD_DAY", g_A_REST_OF_A_GOOD_DAY, s_A_REST_OF_A_GOOD_DAY);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_REST_OF_A_HARD_DAY", g_A_REST_OF_A_HARD_DAY, s_A_REST_OF_A_HARD_DAY);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SKILL_ASKED_OF_ME", g_A_SKILL_ASKED_OF_ME, s_A_SKILL_ASKED_OF_ME);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SLIP", g_A_SLIP, s_A_SLIP);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SLIP_IS_NOT_A_FALL", g_A_SLIP_IS_NOT_A_FALL, s_A_SLIP_IS_NOT_A_FALL);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SMALL_DAY", g_A_SMALL_DAY, s_A_SMALL_DAY);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SOCIAL_SLIP", g_A_SOCIAL_SLIP, s_A_SOCIAL_SLIP);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SORRY_ON_ITS_OWN", g_A_SORRY_ON_ITS_OWN, s_A_SORRY_ON_ITS_OWN);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_STATE_NOW", g_A_STATE_NOW, s_A_STATE_NOW);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_STATE_PAST", g_A_STATE_PAST, s_A_STATE_PAST);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SUGGESTED", g_A_SUGGESTED, s_A_SUGGESTED);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_SUGGESTED_BY_SOMEONE", g_A_SUGGESTED_BY_SOMEONE, s_A_SUGGESTED_BY_SOMEONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_AHEAD", g_A_THING_AHEAD, s_A_THING_AHEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_BOUGHT", g_A_THING_BOUGHT, s_A_THING_BOUGHT);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_CAME_OUT_WRONG", g_A_THING_CAME_OUT_WRONG, s_A_THING_CAME_OUT_WRONG);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_DONE", g_A_THING_DONE, s_A_THING_DONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_GONE", g_A_THING_GONE, s_A_THING_GONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_LOOKS_SAD", g_A_THING_LOOKS_SAD, s_A_THING_LOOKS_SAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_ON", g_A_THING_ON, s_A_THING_ON);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_SEEN_TO", g_A_THING_SEEN_TO, s_A_THING_SEEN_TO);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_TAKEN", g_A_THING_TAKEN, s_A_THING_TAKEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_THING_THEY_REGRET", g_A_THING_THEY_REGRET, s_A_THING_THEY_REGRET);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_VERDICT_ON_THE_PERSON", g_A_VERDICT_ON_THE_PERSON, s_A_VERDICT_ON_THE_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_WAIT_ENDING", g_A_WAIT_ENDING, s_A_WAIT_ENDING);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_WANT_IT_OVER", g_A_WANT_IT_OVER, s_A_WANT_IT_OVER);
    pyrt::xrefs::reg_access("compose.fallback_data", "A_WAY_OF_BEING", g_A_WAY_OF_BEING, s_A_WAY_OF_BEING);
    pyrt::xrefs::reg_access("compose.fallback_data", "BABY_FOOD_NAMES", g_BABY_FOOD_NAMES, s_BABY_FOOD_NAMES);
    pyrt::xrefs::reg_access("compose.fallback_data", "BABY_WHO", g_BABY_WHO, s_BABY_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "BAD_STRETCH", g_BAD_STRETCH, s_BAD_STRETCH);
    pyrt::xrefs::reg_access("compose.fallback_data", "BAD_STRETCH_REPLIES", g_BAD_STRETCH_REPLIES, s_BAD_STRETCH_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_FOLLOW_UP_CARE_REPLIES", g_BARE_FOLLOW_UP_CARE_REPLIES, s_BARE_FOLLOW_UP_CARE_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_FOLLOW_UP_REPLIES", g_BARE_FOLLOW_UP_REPLIES, s_BARE_FOLLOW_UP_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_AFTER_NEWS", g_BARE_PICK_AFTER_NEWS, s_BARE_PICK_AFTER_NEWS);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_ASK", g_BARE_PICK_ASK, s_BARE_PICK_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_BRING", g_BARE_PICK_BRING, s_BARE_PICK_BRING);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_DO", g_BARE_PICK_DO, s_BARE_PICK_DO);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_EXPECT", g_BARE_PICK_EXPECT, s_BARE_PICK_EXPECT);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_GET", g_BARE_PICK_GET, s_BARE_PICK_GET);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_KIND", g_BARE_PICK_KIND, s_BARE_PICK_KIND);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_LOOK", g_BARE_PICK_LOOK, s_BARE_PICK_LOOK);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_LOST", g_BARE_PICK_LOST, s_BARE_PICK_LOST);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_ORDER", g_BARE_PICK_ORDER, s_BARE_PICK_ORDER);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_PREPARE", g_BARE_PICK_PREPARE, s_BARE_PICK_PREPARE);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_SIGN", g_BARE_PICK_SIGN, s_BARE_PICK_SIGN);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_TAKE", g_BARE_PICK_TAKE, s_BARE_PICK_TAKE);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_WEAR", g_BARE_PICK_WEAR, s_BARE_PICK_WEAR);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_WEAR_COLD", g_BARE_PICK_WEAR_COLD, s_BARE_PICK_WEAR_COLD);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_WEAR_HOT", g_BARE_PICK_WEAR_HOT, s_BARE_PICK_WEAR_HOT);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_WEAR_WET", g_BARE_PICK_WEAR_WET, s_BARE_PICK_WEAR_WET);
    pyrt::xrefs::reg_access("compose.fallback_data", "BARE_PICK_WHICH", g_BARE_PICK_WHICH, s_BARE_PICK_WHICH);
    pyrt::xrefs::reg_access("compose.fallback_data", "BECOMING", g_BECOMING, s_BECOMING);
    pyrt::xrefs::reg_access("compose.fallback_data", "BECOMING_STATE", g_BECOMING_STATE, s_BECOMING_STATE);
    pyrt::xrefs::reg_access("compose.fallback_data", "BEHIND_ON_MONEY", g_BEHIND_ON_MONEY, s_BEHIND_ON_MONEY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BLACKED_OUT_SIGHT", g_BLACKED_OUT_SIGHT, s_BLACKED_OUT_SIGHT);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_ALSO", g_BODY_ALSO, s_BODY_ALSO);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_BUNGED", g_BODY_BUNGED, s_BODY_BUNGED);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_CLAMMY", g_BODY_CLAMMY, s_BODY_CLAMMY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_COLD", g_BODY_COLD, s_BODY_COLD);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_HOT", g_BODY_HOT, s_BODY_HOT);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_HUNGRY", g_BODY_HUNGRY, s_BODY_HUNGRY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_IS_NOT_A_MACHINE", g_BODY_IS_NOT_A_MACHINE, s_BODY_IS_NOT_A_MACHINE);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_LEAD", g_BODY_LEAD, s_BODY_LEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_LIGHT_HEAD", g_BODY_LIGHT_HEAD, s_BODY_LIGHT_HEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_PALE", g_BODY_PALE, s_BODY_PALE);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_PARTS", g_BODY_PARTS, s_BODY_PARTS);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_PART_QUALIFIER", g_BODY_PART_QUALIFIER, s_BODY_PART_QUALIFIER);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_PART_TAIL", g_BODY_PART_TAIL, s_BODY_PART_TAIL);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_POOLS", g_BODY_POOLS, s_BODY_POOLS);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_PUFFY", g_BODY_PUFFY, s_BODY_PUFFY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_QUEASY", g_BODY_QUEASY, s_BODY_QUEASY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_SAYS_WHY", g_BODY_SAYS_WHY, s_BODY_SAYS_WHY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_SITUATION", g_BODY_SITUATION, s_BODY_SITUATION);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_SPOTTY", g_BODY_SPOTTY, s_BODY_SPOTTY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_STATES", g_BODY_STATES, s_BODY_STATES);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_SUBJECT", g_BODY_SUBJECT, s_BODY_SUBJECT);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_TAIL", g_BODY_TAIL, s_BODY_TAIL);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_THIRSTY", g_BODY_THIRSTY, s_BODY_THIRSTY);
    pyrt::xrefs::reg_access("compose.fallback_data", "BODY_WORD_FOR_A_THING", g_BODY_WORD_FOR_A_THING, s_BODY_WORD_FOR_A_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "BOUGHT_VERB", g_BOUGHT_VERB, s_BOUGHT_VERB);
    pyrt::xrefs::reg_access("compose.fallback_data", "BOUND_FOR", g_BOUND_FOR, s_BOUND_FOR);
    pyrt::xrefs::reg_access("compose.fallback_data", "BURN_AFTER", g_BURN_AFTER, s_BURN_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "BURN_BLISTER", g_BURN_BLISTER, s_BURN_BLISTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "BUYING_WORDS", g_BUYING_WORDS, s_BUYING_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "CALL_ANSWER", g_CALL_ANSWER, s_CALL_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CALL_WORDS", g_CALL_WORDS, s_CALL_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "CAME_BACK", g_CAME_BACK, s_CAME_BACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARD_LINES", g_CARD_LINES, s_CARD_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARD_VALIDITY", g_CARD_VALIDITY, s_CARD_VALIDITY);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_AFTER", g_CARE_AFTER, s_CARE_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_AFTER_ANIMAL", g_CARE_AFTER_ANIMAL, s_CARE_AFTER_ANIMAL);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_AFTER_ANIMAL_STILL", g_CARE_AFTER_ANIMAL_STILL, s_CARE_AFTER_ANIMAL_STILL);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_AFTER_STILL", g_CARE_AFTER_STILL, s_CARE_AFTER_STILL);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_BETTER", g_CARE_BETTER, s_CARE_BETTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_HERE", g_CARE_HERE, s_CARE_HERE);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_HURTS", g_CARE_HURTS, s_CARE_HURTS);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_INJURY", g_CARE_INJURY, s_CARE_INJURY);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_MODULES", g_CARE_MODULES, s_CARE_MODULES);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_NOT_RESOLVED", g_CARE_NOT_RESOLVED, s_CARE_NOT_RESOLVED);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_PHRASES", g_CARE_PHRASES, s_CARE_PHRASES);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_POINTER", g_CARE_POINTER, s_CARE_POINTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_PRONOUNS", g_CARE_PRONOUNS, s_CARE_PRONOUNS);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_RESOLVED", g_CARE_RESOLVED, s_CARE_RESOLVED);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_ROUTE_BACK", g_CARE_ROUTE_BACK, s_CARE_ROUTE_BACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_SEEN", g_CARE_SEEN, s_CARE_SEEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_SEEN_ANIMAL", g_CARE_SEEN_ANIMAL, s_CARE_SEEN_ANIMAL);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_THREAD", g_CARE_THREAD, s_CARE_THREAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_VISIT", g_CARE_VISIT, s_CARE_VISIT);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_VISIT_WARD", g_CARE_VISIT_WARD, s_CARE_VISIT_WARD);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARE_WORDS", g_CARE_WORDS, s_CARE_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARING", g_CARING, s_CARING);
    pyrt::xrefs::reg_access("compose.fallback_data", "CARING_FOR_A_BODY", g_CARING_FOR_A_BODY, s_CARING_FOR_A_BODY);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHECK_PHRASES", g_CHECK_PHRASES, s_CHECK_PHRASES);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHILD_AT_HOME", g_CHILD_AT_HOME, s_CHILD_AT_HOME);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHILD_FAINT", g_CHILD_FAINT, s_CHILD_FAINT);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHILD_FAINT_ANSWER", g_CHILD_FAINT_ANSWER, s_CHILD_FAINT_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_BETWEEN_PEOPLE", g_CHOICE_BETWEEN_PEOPLE, s_CHOICE_BETWEEN_PEOPLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_HEALTH", g_CHOICE_HEALTH, s_CHOICE_HEALTH);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_LIGHT", g_CHOICE_LIGHT, s_CHOICE_LIGHT);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_LIGHT_PICK", g_CHOICE_LIGHT_PICK, s_CHOICE_LIGHT_PICK);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_PEOPLE", g_CHOICE_PEOPLE, s_CHOICE_PEOPLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_PHARMACIST", g_CHOICE_PHARMACIST, s_CHOICE_PHARMACIST);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_STAKES", g_CHOICE_STAKES, s_CHOICE_STAKES);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_THERAPY", g_CHOICE_THERAPY, s_CHOICE_THERAPY);
    pyrt::xrefs::reg_access("compose.fallback_data", "CHOICE_WEIGH", g_CHOICE_WEIGH, s_CHOICE_WEIGH);
    pyrt::xrefs::reg_access("compose.fallback_data", "CLOSED_DOOR", g_CLOSED_DOOR, s_CLOSED_DOOR);
    pyrt::xrefs::reg_access("compose.fallback_data", "COLD_OPEN", g_COLD_OPEN, s_COLD_OPEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "COMFORT_SAID", g_COMFORT_SAID, s_COMFORT_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "COMPLAINT_WORDS", g_COMPLAINT_WORDS, s_COMPLAINT_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "COMPLAIN_WHO", g_COMPLAIN_WHO, s_COMPLAIN_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "CONGRATS", g_CONGRATS, s_CONGRATS);
    pyrt::xrefs::reg_access("compose.fallback_data", "CONTACT_ANSWER", g_CONTACT_ANSWER, s_CONTACT_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CONTRAST", g_CONTRAST, s_CONTRAST);
    pyrt::xrefs::reg_access("compose.fallback_data", "COPULA", g_COPULA, s_COPULA);
    pyrt::xrefs::reg_access("compose.fallback_data", "CRISIS_AFTER", g_CRISIS_AFTER, s_CRISIS_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CRISIS_AFTER_THEM", g_CRISIS_AFTER_THEM, s_CRISIS_AFTER_THEM);
    pyrt::xrefs::reg_access("compose.fallback_data", "CRISIS_BETTER", g_CRISIS_BETTER, s_CRISIS_BETTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "CRISIS_BETTER_THEM", g_CRISIS_BETTER_THEM, s_CRISIS_BETTER_THEM);
    pyrt::xrefs::reg_access("compose.fallback_data", "CRISIS_WORDS", g_CRISIS_WORDS, s_CRISIS_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "CROWD", g_CROWD, s_CROWD);
    pyrt::xrefs::reg_access("compose.fallback_data", "CUT_FOR", g_CUT_FOR, s_CUT_FOR);
    pyrt::xrefs::reg_access("compose.fallback_data", "DARK_THING", g_DARK_THING, s_DARK_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "DATES_KEEP_SLIPPING", g_DATES_KEEP_SLIPPING, s_DATES_KEEP_SLIPPING);
    pyrt::xrefs::reg_access("compose.fallback_data", "DECLINED_PLAN", g_DECLINED_PLAN, s_DECLINED_PLAN);
    pyrt::xrefs::reg_access("compose.fallback_data", "DECLINED_PLAN_GENTLE", g_DECLINED_PLAN_GENTLE, s_DECLINED_PLAN_GENTLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "DENTIST_WHO", g_DENTIST_WHO, s_DENTIST_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "DETAILS_IN_A_WARM_LINE", g_DETAILS_IN_A_WARM_LINE, s_DETAILS_IN_A_WARM_LINE);
    pyrt::xrefs::reg_access("compose.fallback_data", "DEVICE_WHO", g_DEVICE_WHO, s_DEVICE_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "DIE_OF_IT", g_DIE_OF_IT, s_DIE_OF_IT);
    pyrt::xrefs::reg_access("compose.fallback_data", "DISH_DETERMINER", g_DISH_DETERMINER, s_DISH_DETERMINER);
    pyrt::xrefs::reg_access("compose.fallback_data", "DISH_PARTICIPLE", g_DISH_PARTICIPLE, s_DISH_PARTICIPLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "DITHER_ABOUT_A_PERSON", g_DITHER_ABOUT_A_PERSON, s_DITHER_ABOUT_A_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "DITHER_DECIDE", g_DITHER_DECIDE, s_DITHER_DECIDE);
    pyrt::xrefs::reg_access("compose.fallback_data", "DITHER_OPEN_LINES", g_DITHER_OPEN_LINES, s_DITHER_OPEN_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "DITHER_SUBJECT", g_DITHER_SUBJECT, s_DITHER_SUBJECT);
    pyrt::xrefs::reg_access("compose.fallback_data", "DITHER_TWO_WAY_LINES", g_DITHER_TWO_WAY_LINES, s_DITHER_TWO_WAY_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "DONE_NOT_A_PERSON", g_DONE_NOT_A_PERSON, s_DONE_NOT_A_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "DONE_NOT_THIS", g_DONE_NOT_THIS, s_DONE_NOT_THIS);
    pyrt::xrefs::reg_access("compose.fallback_data", "DONE_ONLY_A_THING", g_DONE_ONLY_A_THING, s_DONE_ONLY_A_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "DONE_VERB", g_DONE_VERB, s_DONE_VERB);
    pyrt::xrefs::reg_access("compose.fallback_data", "DOUBLE_BOOKED", g_DOUBLE_BOOKED, s_DOUBLE_BOOKED);
    pyrt::xrefs::reg_access("compose.fallback_data", "EASING_FILLER", g_EASING_FILLER, s_EASING_FILLER);
    pyrt::xrefs::reg_access("compose.fallback_data", "EATEN_KIND", g_EATEN_KIND, s_EATEN_KIND);
    pyrt::xrefs::reg_access("compose.fallback_data", "EATING", g_EATING, s_EATING);
    pyrt::xrefs::reg_access("compose.fallback_data", "ELDER_AID_REPLIES", g_ELDER_AID_REPLIES, s_ELDER_AID_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "ELDER_CLINIC_REPLIES", g_ELDER_CLINIC_REPLIES, s_ELDER_CLINIC_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "ELDER_DOCTOR_REPLIES", g_ELDER_DOCTOR_REPLIES, s_ELDER_DOCTOR_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "ELDER_HELP_REPLIES", g_ELDER_HELP_REPLIES, s_ELDER_HELP_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMBARRASSED", g_EMBARRASSED, s_EMBARRASSED);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMBARRASSMENT_WORD", g_EMBARRASSMENT_WORD, s_EMBARRASSMENT_WORD);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMERGENCY_AFTER", g_EMERGENCY_AFTER, s_EMERGENCY_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMERGENCY_ANSWER", g_EMERGENCY_ANSWER, s_EMERGENCY_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMERGENCY_BETTER", g_EMERGENCY_BETTER, s_EMERGENCY_BETTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMERGENCY_BETTER_THEM", g_EMERGENCY_BETTER_THEM, s_EMERGENCY_BETTER_THEM);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMERGENCY_NOT_WORKED", g_EMERGENCY_NOT_WORKED, s_EMERGENCY_NOT_WORKED);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMERGENCY_WORDS", g_EMERGENCY_WORDS, s_EMERGENCY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "EMPTY", g_EMPTY, s_EMPTY);
    pyrt::xrefs::reg_access("compose.fallback_data", "ENDINGS", g_ENDINGS, s_ENDINGS);
    pyrt::xrefs::reg_access("compose.fallback_data", "EXPERT_WORDS", g_EXPERT_WORDS, s_EXPERT_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FAINT_COMING_ANSWER", g_FAINT_COMING_ANSWER, s_FAINT_COMING_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "FALLBACK_MODULE", g_FALLBACK_MODULE, s_FALLBACK_MODULE);
    pyrt::xrefs::reg_access("compose.fallback_data", "FAVOURITE_GENERAL", g_FAVOURITE_GENERAL, s_FAVOURITE_GENERAL);
    pyrt::xrefs::reg_access("compose.fallback_data", "FAVOURITE_WORDS", g_FAVOURITE_WORDS, s_FAVOURITE_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FILLERS", g_FILLERS, s_FILLERS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FILLING_FOODS", g_FILLING_FOODS, s_FILLING_FOODS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FILLING_WORDS", g_FILLING_WORDS, s_FILLING_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FIRST_PERSON", g_FIRST_PERSON, s_FIRST_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "FLAT", g_FLAT, s_FLAT);
    pyrt::xrefs::reg_access("compose.fallback_data", "FLAT_CORE", g_FLAT_CORE, s_FLAT_CORE);
    pyrt::xrefs::reg_access("compose.fallback_data", "FLAT_LEAD", g_FLAT_LEAD, s_FLAT_LEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "FLOPPY_WAKE", g_FLOPPY_WAKE, s_FLOPPY_WAKE);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST", g_FORECAST, s_FORECAST);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_AFTER_SAFETY", g_FORECAST_AFTER_SAFETY, s_FORECAST_AFTER_SAFETY);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_A_FALL", g_FORECAST_A_FALL, s_FORECAST_A_FALL);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_BODY_WORDS", g_FORECAST_BODY_WORDS, s_FORECAST_BODY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_DURATION", g_FORECAST_DURATION, s_FORECAST_DURATION);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_HE_SHE", g_FORECAST_HE_SHE, s_FORECAST_HE_SHE);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_ILL", g_FORECAST_ILL, s_FORECAST_ILL);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_I_ASK", g_FORECAST_I_ASK, s_FORECAST_I_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_OFF_THEIR_FOOD", g_FORECAST_OFF_THEIR_FOOD, s_FORECAST_OFF_THEIR_FOOD);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_PERMISSION", g_FORECAST_PERMISSION, s_FORECAST_PERMISSION);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_PERSON", g_FORECAST_PERSON, s_FORECAST_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_PET", g_FORECAST_PET, s_FORECAST_PET);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_PET_HARM", g_FORECAST_PET_HARM, s_FORECAST_PET_HARM);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_POOLS", g_FORECAST_POOLS, s_FORECAST_POOLS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_RELATION", g_FORECAST_RELATION, s_FORECAST_RELATION);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_SELF", g_FORECAST_SELF, s_FORECAST_SELF);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_SELF_STRUGGLING", g_FORECAST_SELF_STRUGGLING, s_FORECAST_SELF_STRUGGLING);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_SOMEONE_STRUGGLING", g_FORECAST_SOMEONE_STRUGGLING, s_FORECAST_SOMEONE_STRUGGLING);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORECAST_STRUGGLE", g_FORECAST_STRUGGLE, s_FORECAST_STRUGGLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORGETTING", g_FORGETTING, s_FORGETTING);
    pyrt::xrefs::reg_access("compose.fallback_data", "FORGET_A_THING", g_FORGET_A_THING, s_FORGET_A_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "FOUND_IT", g_FOUND_IT, s_FOUND_IT);
    pyrt::xrefs::reg_access("compose.fallback_data", "FRAME_WORDS", g_FRAME_WORDS, s_FRAME_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FUEL_WORDS", g_FUEL_WORDS, s_FUEL_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "FUME_WORDS", g_FUME_WORDS, s_FUME_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "GADGET_WORDS", g_GADGET_WORDS, s_GADGET_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "GAS_ANSWER", g_GAS_ANSWER, s_GAS_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "GAS_WORDS", g_GAS_WORDS, s_GAS_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "GENERIC_WORDS", g_GENERIC_WORDS, s_GENERIC_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "GIFT_WORDS", g_GIFT_WORDS, s_GIFT_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "GOING_TO_DIE", g_GOING_TO_DIE, s_GOING_TO_DIE);
    pyrt::xrefs::reg_access("compose.fallback_data", "GONE_VERB", g_GONE_VERB, s_GONE_VERB);
    pyrt::xrefs::reg_access("compose.fallback_data", "GOOD_TIME_WORDS", g_GOOD_TIME_WORDS, s_GOOD_TIME_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "GO_ANSWER", g_GO_ANSWER, s_GO_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "GO_TO_BE_SEEN", g_GO_TO_BE_SEEN, s_GO_TO_BE_SEEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "GREETING", g_GREETING, s_GREETING);
    pyrt::xrefs::reg_access("compose.fallback_data", "GREETING_AFTER", g_GREETING_AFTER, s_GREETING_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "GREETING_WORDS", g_GREETING_WORDS, s_GREETING_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEF_AFTER", g_GRIEF_AFTER, s_GRIEF_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE", g_GRIEVANCE, s_GRIEVANCE);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_HARM", g_GRIEVANCE_HARM, s_GRIEVANCE_HARM);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_HARM_PHRASES", g_GRIEVANCE_HARM_PHRASES, s_GRIEVANCE_HARM_PHRASES);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_HEALTH", g_GRIEVANCE_HEALTH, s_GRIEVANCE_HEALTH);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_HEALTH_PHRASES", g_GRIEVANCE_HEALTH_PHRASES, s_GRIEVANCE_HEALTH_PHRASES);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_IMPACT", g_GRIEVANCE_IMPACT, s_GRIEVANCE_IMPACT);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_NOT_A_THING", g_GRIEVANCE_NOT_A_THING, s_GRIEVANCE_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_PEOPLE", g_GRIEVANCE_PEOPLE, s_GRIEVANCE_PEOPLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_PHRASES", g_GRIEVANCE_PHRASES, s_GRIEVANCE_PHRASES);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_PRONOUN", g_GRIEVANCE_PRONOUN, s_GRIEVANCE_PRONOUN);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_STRAIN", g_GRIEVANCE_STRAIN, s_GRIEVANCE_STRAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "GRIEVANCE_STRAIN_SAID", g_GRIEVANCE_STRAIN_SAID, s_GRIEVANCE_STRAIN_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "GROWN_UP_WORDS", g_GROWN_UP_WORDS, s_GROWN_UP_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "HABIT_ACT", g_HABIT_ACT, s_HABIT_ACT);
    pyrt::xrefs::reg_access("compose.fallback_data", "HABIT_PROBLEM_ACT", g_HABIT_PROBLEM_ACT, s_HABIT_PROBLEM_ACT);
    pyrt::xrefs::reg_access("compose.fallback_data", "HABIT_REPLY", g_HABIT_REPLY, s_HABIT_REPLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "HAND", g_HAND, s_HAND);
    pyrt::xrefs::reg_access("compose.fallback_data", "HANDED_BACK", g_HANDED_BACK, s_HANDED_BACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "HANDS_BACK", g_HANDS_BACK, s_HANDS_BACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "HAZARD_AFTER", g_HAZARD_AFTER, s_HAZARD_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "HAZARD_WORDS", g_HAZARD_WORDS, s_HAZARD_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "HEALTH_RECORD", g_HEALTH_RECORD, s_HEALTH_RECORD);
    pyrt::xrefs::reg_access("compose.fallback_data", "HEALTH_RECORD_LINES", g_HEALTH_RECORD_LINES, s_HEALTH_RECORD_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "HEALTH_WHO", g_HEALTH_WHO, s_HEALTH_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "HEALTH_WORDS", g_HEALTH_WORDS, s_HEALTH_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "HEALTH_WORLD", g_HEALTH_WORLD, s_HEALTH_WORLD);
    pyrt::xrefs::reg_access("compose.fallback_data", "HEAVY_NEWS", g_HEAVY_NEWS, s_HEAVY_NEWS);
    pyrt::xrefs::reg_access("compose.fallback_data", "HEDGE_POOL", g_HEDGE_POOL, s_HEDGE_POOL);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELLO_STILL_THERE", g_HELLO_STILL_THERE, s_HELLO_STILL_THERE);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELLO_WITH_YOU", g_HELLO_WITH_YOU, s_HELLO_WITH_YOU);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELP", g_HELP, s_HELP);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELPLINE_NOT_AN_AMBULANCE", g_HELPLINE_NOT_AN_AMBULANCE, s_HELPLINE_NOT_AN_AMBULANCE);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELP_ALREADY", g_HELP_ALREADY, s_HELP_ALREADY);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELP_TAIL", g_HELP_TAIL, s_HELP_TAIL);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELP_TOPIC", g_HELP_TOPIC, s_HELP_TOPIC);
    pyrt::xrefs::reg_access("compose.fallback_data", "HELP_TOPIC_ANY", g_HELP_TOPIC_ANY, s_HELP_TOPIC_ANY);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOLD_AFTER", g_HOLD_AFTER, s_HOLD_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOUSE_VERB", g_HOUSE_VERB, s_HOUSE_VERB);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOW_ARE_YOU", g_HOW_ARE_YOU, s_HOW_ARE_YOU);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOW_ARE_YOU_ASKS", g_HOW_ARE_YOU_ASKS, s_HOW_ARE_YOU_ASKS);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOW_ARE_YOU_GOING_ON", g_HOW_ARE_YOU_GOING_ON, s_HOW_ARE_YOU_GOING_ON);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOW_ARE_YOU_GOING_ON_AFTER", g_HOW_ARE_YOU_GOING_ON_AFTER, s_HOW_ARE_YOU_GOING_ON_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOW_ARE_YOU_TAGS", g_HOW_ARE_YOU_TAGS, s_HOW_ARE_YOU_TAGS);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOW_TO_DECIDE_FILLER", g_HOW_TO_DECIDE_FILLER, s_HOW_TO_DECIDE_FILLER);
    pyrt::xrefs::reg_access("compose.fallback_data", "HOW_TO_DECIDE_LINES", g_HOW_TO_DECIDE_LINES, s_HOW_TO_DECIDE_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "HUMAN_EMPTY", g_HUMAN_EMPTY, s_HUMAN_EMPTY);
    pyrt::xrefs::reg_access("compose.fallback_data", "HURT_EVENT", g_HURT_EVENT, s_HURT_EVENT);
    pyrt::xrefs::reg_access("compose.fallback_data", "IDENTITY", g_IDENTITY, s_IDENTITY);
    pyrt::xrefs::reg_access("compose.fallback_data", "IDIOM_GLAD", g_IDIOM_GLAD, s_IDIOM_GLAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "IDIOM_SHY", g_IDIOM_SHY, s_IDIOM_SHY);
    pyrt::xrefs::reg_access("compose.fallback_data", "ILL_AFTER", g_ILL_AFTER, s_ILL_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "ILL_BEFORE", g_ILL_BEFORE, s_ILL_BEFORE);
    pyrt::xrefs::reg_access("compose.fallback_data", "ILL_SAID_OF_A_BODY", g_ILL_SAID_OF_A_BODY, s_ILL_SAID_OF_A_BODY);
    pyrt::xrefs::reg_access("compose.fallback_data", "ILL_WILL_VERBS", g_ILL_WILL_VERBS, s_ILL_WILL_VERBS);
    pyrt::xrefs::reg_access("compose.fallback_data", "IMPERATIVE", g_IMPERATIVE, s_IMPERATIVE);
    pyrt::xrefs::reg_access("compose.fallback_data", "INVITED", g_INVITED, s_INVITED);
    pyrt::xrefs::reg_access("compose.fallback_data", "INVITED_TO_TALK", g_INVITED_TO_TALK, s_INVITED_TO_TALK);
    pyrt::xrefs::reg_access("compose.fallback_data", "INVITE_ACTIVITY", g_INVITE_ACTIVITY, s_INVITE_ACTIVITY);
    pyrt::xrefs::reg_access("compose.fallback_data", "INVITE_FILLER", g_INVITE_FILLER, s_INVITE_FILLER);
    pyrt::xrefs::reg_access("compose.fallback_data", "INVITE_NEVER", g_INVITE_NEVER, s_INVITE_NEVER);
    pyrt::xrefs::reg_access("compose.fallback_data", "INVITE_PRONOUN", g_INVITE_PRONOUN, s_INVITE_PRONOUN);
    pyrt::xrefs::reg_access("compose.fallback_data", "IN_A_ROOM", g_IN_A_ROOM, s_IN_A_ROOM);
    pyrt::xrefs::reg_access("compose.fallback_data", "I_RATE_A_SKILL", g_I_RATE_A_SKILL, s_I_RATE_A_SKILL);
    pyrt::xrefs::reg_access("compose.fallback_data", "JOINT_WORDS", g_JOINT_WORDS, s_JOINT_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_ASKS_FINE", g_JUDGE_ASKS_FINE, s_JUDGE_ASKS_FINE);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_ASKS_MAD", g_JUDGE_ASKS_MAD, s_JUDGE_ASKS_MAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_ASKS_SLIGHT", g_JUDGE_ASKS_SLIGHT, s_JUDGE_ASKS_SLIGHT);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_BARE_FACES", g_JUDGE_BARE_FACES, s_JUDGE_BARE_FACES);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_FINE", g_JUDGE_FINE, s_JUDGE_FINE);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_FINE_PLAIN", g_JUDGE_FINE_PLAIN, s_JUDGE_FINE_PLAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_LEAD", g_JUDGE_LEAD, s_JUDGE_LEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NOD_WORDS", g_JUDGE_NOD_WORDS, s_JUDGE_NOD_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NOT_A_GERUND", g_JUDGE_NOT_A_GERUND, s_JUDGE_NOT_A_GERUND);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NOT_A_VERDICT", g_JUDGE_NOT_A_VERDICT, s_JUDGE_NOT_A_VERDICT);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NOUN", g_JUDGE_NOUN, s_JUDGE_NOUN);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NO_PLAN", g_JUDGE_NO_PLAN, s_JUDGE_NO_PLAN);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NO_PLAN_FINE", g_JUDGE_NO_PLAN_FINE, s_JUDGE_NO_PLAN_FINE);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NO_PLAN_MAD", g_JUDGE_NO_PLAN_MAD, s_JUDGE_NO_PLAN_MAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_NO_PLAN_MIND", g_JUDGE_NO_PLAN_MIND, s_JUDGE_NO_PLAN_MIND);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_ONLY_A_PLAN", g_JUDGE_ONLY_A_PLAN, s_JUDGE_ONLY_A_PLAN);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_PLAN_HAS_A_HOLE", g_JUDGE_PLAN_HAS_A_HOLE, s_JUDGE_PLAN_HAS_A_HOLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_PLAN_STOP", g_JUDGE_PLAN_STOP, s_JUDGE_PLAN_STOP);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_SANE", g_JUDGE_SANE, s_JUDGE_SANE);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_SANE_PLAIN", g_JUDGE_SANE_PLAIN, s_JUDGE_SANE_PLAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_SELF_JUDGING", g_JUDGE_SELF_JUDGING, s_JUDGE_SELF_JUDGING);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_SOUNDS_FINE", g_JUDGE_SOUNDS_FINE, s_JUDGE_SOUNDS_FINE);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_WISH_ANGRY", g_JUDGE_WISH_ANGRY, s_JUDGE_WISH_ANGRY);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_WORTH", g_JUDGE_WORTH, s_JUDGE_WORTH);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUDGE_WORTH_PLAIN", g_JUDGE_WORTH_PLAIN, s_JUDGE_WORTH_PLAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "JUNK_CUES", g_JUNK_CUES, s_JUNK_CUES);
    pyrt::xrefs::reg_access("compose.fallback_data", "KEEPS_FORGETTING", g_KEEPS_FORGETTING, s_KEEPS_FORGETTING);
    pyrt::xrefs::reg_access("compose.fallback_data", "KNOCKED", g_KNOCKED, s_KNOCKED);
    pyrt::xrefs::reg_access("compose.fallback_data", "KNOCKED_SPOTS", g_KNOCKED_SPOTS, s_KNOCKED_SPOTS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LANDED_BAD", g_LANDED_BAD, s_LANDED_BAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "LANGUAGE_FLUENCY", g_LANGUAGE_FLUENCY, s_LANGUAGE_FLUENCY);
    pyrt::xrefs::reg_access("compose.fallback_data", "LATE_BEHIND_LINES", g_LATE_BEHIND_LINES, s_LATE_BEHIND_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "LATE_NOW_LINES", g_LATE_NOW_LINES, s_LATE_NOW_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "LAUGHED_AT", g_LAUGHED_AT, s_LAUGHED_AT);
    pyrt::xrefs::reg_access("compose.fallback_data", "LAUGHED_AT_THEM", g_LAUGHED_AT_THEM, s_LAUGHED_AT_THEM);
    pyrt::xrefs::reg_access("compose.fallback_data", "LAUGHING_WORDS", g_LAUGHING_WORDS, s_LAUGHING_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LEFT_BEHIND", g_LEFT_BEHIND, s_LEFT_BEHIND);
    pyrt::xrefs::reg_access("compose.fallback_data", "LEFT_POOL", g_LEFT_POOL, s_LEFT_POOL);
    pyrt::xrefs::reg_access("compose.fallback_data", "LEFT_POOL_GONE", g_LEFT_POOL_GONE, s_LEFT_POOL_GONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_ASK", g_LIVE_ASK, s_LIVE_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_DELAY_WORDS", g_LIVE_DELAY_WORDS, s_LIVE_DELAY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_HOURS_ASK", g_LIVE_HOURS_ASK, s_LIVE_HOURS_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_INDOOR", g_LIVE_INDOOR, s_LIVE_INDOOR);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_JOURNEY_ENDS", g_LIVE_JOURNEY_ENDS, s_LIVE_JOURNEY_ENDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_KINDS", g_LIVE_KINDS, s_LIVE_KINDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_NEWS_REPLY", g_LIVE_NEWS_REPLY, s_LIVE_NEWS_REPLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_NOT", g_LIVE_NOT, s_LIVE_NOT);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_NOT_HOURS", g_LIVE_NOT_HOURS, s_LIVE_NOT_HOURS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_OPEN_CLOSE", g_LIVE_OPEN_CLOSE, s_LIVE_OPEN_CLOSE);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_PAPER_WORDS", g_LIVE_PAPER_WORDS, s_LIVE_PAPER_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_ROADS_NOT", g_LIVE_ROADS_NOT, s_LIVE_ROADS_NOT);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_ROAD_NAME", g_LIVE_ROAD_NAME, s_LIVE_ROAD_NAME);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_ROAD_WORDS", g_LIVE_ROAD_WORDS, s_LIVE_ROAD_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_SETTING", g_LIVE_SETTING, s_LIVE_SETTING);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_SITE_THING", g_LIVE_SITE_THING, s_LIVE_SITE_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_SKY", g_LIVE_SKY, s_LIVE_SKY);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_SKY_OFF", g_LIVE_SKY_OFF, s_LIVE_SKY_OFF);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_SKY_WHY", g_LIVE_SKY_WHY, s_LIVE_SKY_WHY);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_SOON", g_LIVE_SOON, s_LIVE_SOON);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_STRONG", g_LIVE_STRONG, s_LIVE_STRONG);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_TAKE_ALLOWED", g_LIVE_TAKE_ALLOWED, s_LIVE_TAKE_ALLOWED);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_TAKE_ASK", g_LIVE_TAKE_ASK, s_LIVE_TAKE_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_TRAVEL_WORDS", g_LIVE_TRAVEL_WORDS, s_LIVE_TRAVEL_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_VENUE", g_LIVE_VENUE, s_LIVE_VENUE);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_VENUE_RE", g_LIVE_VENUE_RE, s_LIVE_VENUE_RE);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_WEAR", g_LIVE_WEAR, s_LIVE_WEAR);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_WEATHER_WORDS", g_LIVE_WEATHER_WORDS, s_LIVE_WEATHER_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "LIVE_YES_NO_OPEN", g_LIVE_YES_NO_OPEN, s_LIVE_YES_NO_OPEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "LOCKED_OUT_LINES", g_LOCKED_OUT_LINES, s_LOCKED_OUT_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "LOCKED_OUT_OF_THE_CAR", g_LOCKED_OUT_OF_THE_CAR, s_LOCKED_OUT_OF_THE_CAR);
    pyrt::xrefs::reg_access("compose.fallback_data", "LOST_A_THING", g_LOST_A_THING, s_LOST_A_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "LOST_A_THING_AGAIN", g_LOST_A_THING_AGAIN, s_LOST_A_THING_AGAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "LOW_DAY_REPLIES", g_LOW_DAY_REPLIES, s_LOW_DAY_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "LUMP", g_LUMP, s_LUMP);
    pyrt::xrefs::reg_access("compose.fallback_data", "MANNERS_ONLY", g_MANNERS_ONLY, s_MANNERS_ONLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "MEDS_WORDS", g_MEDS_WORDS, s_MEDS_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "MEMORY", g_MEMORY, s_MEMORY);
    pyrt::xrefs::reg_access("compose.fallback_data", "MESS_PLACES", g_MESS_PLACES, s_MESS_PLACES);
    pyrt::xrefs::reg_access("compose.fallback_data", "MILESTONE", g_MILESTONE, s_MILESTONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "MILESTONE_EVENT", g_MILESTONE_EVENT, s_MILESTONE_EVENT);
    pyrt::xrefs::reg_access("compose.fallback_data", "MISHAP_BACK", g_MISHAP_BACK, s_MISHAP_BACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "MISSED_STEP_LINES", g_MISSED_STEP_LINES, s_MISSED_STEP_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "MODEL_NAMED_WORDS", g_MODEL_NAMED_WORDS, s_MODEL_NAMED_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "MONEY_WORDS", g_MONEY_WORDS, s_MONEY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "MOOD_AFTER", g_MOOD_AFTER, s_MOOD_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "MOOD_BEFORE", g_MOOD_BEFORE, s_MOOD_BEFORE);
    pyrt::xrefs::reg_access("compose.fallback_data", "MOUTH_WORDS", g_MOUTH_WORDS, s_MOUTH_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "MOVE_ON_AFTER", g_MOVE_ON_AFTER, s_MOVE_ON_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "MYSELF", g_MYSELF, s_MYSELF);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_BILL_LINES", g_MY_BILL_LINES, s_MY_BILL_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_DOING", g_MY_DOING, s_MY_DOING);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_OWN", g_MY_OWN, s_MY_OWN);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_OWN_DAY", g_MY_OWN_DAY, s_MY_OWN_DAY);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_PICK", g_MY_PICK, s_MY_PICK);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_PICK_ASK", g_MY_PICK_ASK, s_MY_PICK_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_PICK_NOTE", g_MY_PICK_NOTE, s_MY_PICK_NOTE);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_RECORDS", g_MY_RECORDS, s_MY_RECORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "MY_WORKINGS", g_MY_WORKINGS, s_MY_WORKINGS);
    pyrt::xrefs::reg_access("compose.fallback_data", "NAMED_BODIES", g_NAMED_BODIES, s_NAMED_BODIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "NAMED_NEGATIVE", g_NAMED_NEGATIVE, s_NAMED_NEGATIVE);
    pyrt::xrefs::reg_access("compose.fallback_data", "NAMED_TYPES", g_NAMED_TYPES, s_NAMED_TYPES);
    pyrt::xrefs::reg_access("compose.fallback_data", "NARROW_ANSWERED", g_NARROW_ANSWERED, s_NARROW_ANSWERED);
    pyrt::xrefs::reg_access("compose.fallback_data", "NARROW_SUBSTANCE", g_NARROW_SUBSTANCE, s_NARROW_SUBSTANCE);
    pyrt::xrefs::reg_access("compose.fallback_data", "NARROW_WORDS", g_NARROW_WORDS, s_NARROW_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "NEGATIVE", g_NEGATIVE, s_NEGATIVE);
    pyrt::xrefs::reg_access("compose.fallback_data", "NEGATIVE_CUES", g_NEGATIVE_CUES, s_NEGATIVE_CUES);
    pyrt::xrefs::reg_access("compose.fallback_data", "NEGATORS", g_NEGATORS, s_NEGATORS);
    pyrt::xrefs::reg_access("compose.fallback_data", "NETWORK_WORDS", g_NETWORK_WORDS, s_NETWORK_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "NEUTRAL", g_NEUTRAL, s_NEUTRAL);
    pyrt::xrefs::reg_access("compose.fallback_data", "NEVER_MIND", g_NEVER_MIND, s_NEVER_MIND);
    pyrt::xrefs::reg_access("compose.fallback_data", "NEWS_TO_COME", g_NEWS_TO_COME, s_NEWS_TO_COME);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOTHING_AT_ALL", g_NOTHING_AT_ALL, s_NOTHING_AT_ALL);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOTHING_SAID_YET", g_NOTHING_SAID_YET, s_NOTHING_SAID_YET);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_AN_ANSWER", g_NOT_AN_ANSWER, s_NOT_AN_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_A_FORM", g_NOT_A_FORM, s_NOT_A_FORM);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_A_NAME", g_NOT_A_NAME, s_NOT_A_NAME);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_A_VERDICT_SUBJECT", g_NOT_A_VERDICT_SUBJECT, s_NOT_A_VERDICT_SUBJECT);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_CHOSEN", g_NOT_CHOSEN, s_NOT_CHOSEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_ENOUGH_REPLY", g_NOT_ENOUGH_REPLY, s_NOT_ENOUGH_REPLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_FIT_ENOUGH", g_NOT_FIT_ENOUGH, s_NOT_FIT_ENOUGH);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_NOTED", g_NOT_NOTED, s_NOT_NOTED);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_PEOPLE_WORDS", g_NOT_PEOPLE_WORDS, s_NOT_PEOPLE_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_THEIR_VERB", g_NOT_THEIR_VERB, s_NOT_THEIR_VERB);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_TOO_OLD", g_NOT_TOO_OLD, s_NOT_TOO_OLD);
    pyrt::xrefs::reg_access("compose.fallback_data", "NOT_WHAT_I_SAID", g_NOT_WHAT_I_SAID, s_NOT_WHAT_I_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "NO_DIRECTION_REPLY", g_NO_DIRECTION_REPLY, s_NO_DIRECTION_REPLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "NO_MORE_OF", g_NO_MORE_OF, s_NO_MORE_OF);
    pyrt::xrefs::reg_access("compose.fallback_data", "NO_WORDS_ASKING", g_NO_WORDS_ASKING, s_NO_WORDS_ASKING);
    pyrt::xrefs::reg_access("compose.fallback_data", "NO_WORDS_AT_THE_CARE", g_NO_WORDS_AT_THE_CARE, s_NO_WORDS_AT_THE_CARE);
    pyrt::xrefs::reg_access("compose.fallback_data", "NO_WORDS_SOMEONE", g_NO_WORDS_SOMEONE, s_NO_WORDS_SOMEONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "NO_WORDS_STUCK", g_NO_WORDS_STUCK, s_NO_WORDS_STUCK);
    pyrt::xrefs::reg_access("compose.fallback_data", "NUMBER_PLACES", g_NUMBER_PLACES, s_NUMBER_PLACES);
    pyrt::xrefs::reg_access("compose.fallback_data", "OCCASION", g_OCCASION, s_OCCASION);
    pyrt::xrefs::reg_access("compose.fallback_data", "OFFICE_RECORD_LINES", g_OFFICE_RECORD_LINES, s_OFFICE_RECORD_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "ONE_SKILL_RATED", g_ONE_SKILL_RATED, s_ONE_SKILL_RATED);
    pyrt::xrefs::reg_access("compose.fallback_data", "ONGOING_WORDS", g_ONGOING_WORDS, s_ONGOING_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "ONLINE_ASK", g_ONLINE_ASK, s_ONLINE_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "ONLY_FRONT", g_ONLY_FRONT, s_ONLY_FRONT);
    pyrt::xrefs::reg_access("compose.fallback_data", "OPINION_ASKS", g_OPINION_ASKS, s_OPINION_ASKS);
    pyrt::xrefs::reg_access("compose.fallback_data", "OURS", g_OURS, s_OURS);
    pyrt::xrefs::reg_access("compose.fallback_data", "OUTCOME_LINES", g_OUTCOME_LINES, s_OUTCOME_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "OWN_PERSON", g_OWN_PERSON, s_OWN_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "OWN_PLACE", g_OWN_PLACE, s_OWN_PLACE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PARTICLE", g_PARTICLE, s_PARTICLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PASSED_OUT", g_PASSED_OUT, s_PASSED_OUT);
    pyrt::xrefs::reg_access("compose.fallback_data", "PASSED_OUT_ANSWER", g_PASSED_OUT_ANSWER, s_PASSED_OUT_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "PASSED_OUT_PERSON", g_PASSED_OUT_PERSON, s_PASSED_OUT_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "PASSING_OUT_A_THING", g_PASSING_OUT_A_THING, s_PASSING_OUT_A_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "PASSWORD_TROUBLE", g_PASSWORD_TROUBLE, s_PASSWORD_TROUBLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PAST_STATE_CUES", g_PAST_STATE_CUES, s_PAST_STATE_CUES);
    pyrt::xrefs::reg_access("compose.fallback_data", "PAUSE", g_PAUSE, s_PAUSE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PAUSE_IN_A_SAFETY_THREAD", g_PAUSE_IN_A_SAFETY_THREAD, s_PAUSE_IN_A_SAFETY_THREAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "PENDING_DOUBT", g_PENDING_DOUBT, s_PENDING_DOUBT);
    pyrt::xrefs::reg_access("compose.fallback_data", "PENDING_LANDED", g_PENDING_LANDED, s_PENDING_LANDED);
    pyrt::xrefs::reg_access("compose.fallback_data", "PENDING_WAITING", g_PENDING_WAITING, s_PENDING_WAITING);
    pyrt::xrefs::reg_access("compose.fallback_data", "PENDING_WHEN", g_PENDING_WHEN, s_PENDING_WHEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "PERSON_WORDS", g_PERSON_WORDS, s_PERSON_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "PEST_WHO", g_PEST_WHO, s_PEST_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "PETROL_WORDS", g_PETROL_WORDS, s_PETROL_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "PET_CONDITION_TROUBLE", g_PET_CONDITION_TROUBLE, s_PET_CONDITION_TROUBLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PET_LOW_REPLY", g_PET_LOW_REPLY, s_PET_LOW_REPLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "PET_TROUBLE", g_PET_TROUBLE, s_PET_TROUBLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PICKS_A_MECHANISM", g_PICKS_A_MECHANISM, s_PICKS_A_MECHANISM);
    pyrt::xrefs::reg_access("compose.fallback_data", "PICK_ONLY_WORDS", g_PICK_ONLY_WORDS, s_PICK_ONLY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "PLAIN_WORDS", g_PLAIN_WORDS, s_PLAIN_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "PLAN", g_PLAN, s_PLAN);
    pyrt::xrefs::reg_access("compose.fallback_data", "PLAN_DONE", g_PLAN_DONE, s_PLAN_DONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PLAN_DONE_ACK", g_PLAN_DONE_ACK, s_PLAN_DONE_ACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "PLAN_KNOCKED", g_PLAN_KNOCKED, s_PLAN_KNOCKED);
    pyrt::xrefs::reg_access("compose.fallback_data", "PLASTER_IT", g_PLASTER_IT, s_PLASTER_IT);
    pyrt::xrefs::reg_access("compose.fallback_data", "POINTING", g_POINTING, s_POINTING);
    pyrt::xrefs::reg_access("compose.fallback_data", "POINTS_LINES", g_POINTS_LINES, s_POINTS_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "POSITIVE", g_POSITIVE, s_POSITIVE);
    pyrt::xrefs::reg_access("compose.fallback_data", "POSITIVE_CUES", g_POSITIVE_CUES, s_POSITIVE_CUES);
    pyrt::xrefs::reg_access("compose.fallback_data", "PRAISE", g_PRAISE, s_PRAISE);
    pyrt::xrefs::reg_access("compose.fallback_data", "PREGNANCY_BLEEDING", g_PREGNANCY_BLEEDING, s_PREGNANCY_BLEEDING);
    pyrt::xrefs::reg_access("compose.fallback_data", "PRESENCE_LEFTOVER", g_PRESENCE_LEFTOVER, s_PRESENCE_LEFTOVER);
    pyrt::xrefs::reg_access("compose.fallback_data", "PRESENT", g_PRESENT, s_PRESENT);
    pyrt::xrefs::reg_access("compose.fallback_data", "PRIVACY_ASK", g_PRIVACY_ASK, s_PRIVACY_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "PRIVACY_SHAPES", g_PRIVACY_SHAPES, s_PRIVACY_SHAPES);
    pyrt::xrefs::reg_access("compose.fallback_data", "PUNT", g_PUNT, s_PUNT);
    pyrt::xrefs::reg_access("compose.fallback_data", "QUESTION_OPENERS", g_QUESTION_OPENERS, s_QUESTION_OPENERS);
    pyrt::xrefs::reg_access("compose.fallback_data", "QUESTION_WORDS", g_QUESTION_WORDS, s_QUESTION_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "QUIET_MODULES", g_QUIET_MODULES, s_QUIET_MODULES);
    pyrt::xrefs::reg_access("compose.fallback_data", "QUOTE_LINES", g_QUOTE_LINES, s_QUOTE_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "RATES_A_SKILL", g_RATES_A_SKILL, s_RATES_A_SKILL);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTIONS_CARE", g_REACTIONS_CARE, s_REACTIONS_CARE);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTIONS_SAID", g_REACTIONS_SAID, s_REACTIONS_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_DISBELIEF", g_REACTION_DISBELIEF, s_REACTION_DISBELIEF);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_DISBELIEF_SAID", g_REACTION_DISBELIEF_SAID, s_REACTION_DISBELIEF_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_LEAD", g_REACTION_LEAD, s_REACTION_LEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_MISHAP", g_REACTION_MISHAP, s_REACTION_MISHAP);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_MISHAP_SAID", g_REACTION_MISHAP_SAID, s_REACTION_MISHAP_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_OUCH", g_REACTION_OUCH, s_REACTION_OUCH);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_OUCH_SAID", g_REACTION_OUCH_SAID, s_REACTION_OUCH_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_RELIEF", g_REACTION_RELIEF, s_REACTION_RELIEF);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_RELIEF_SAID", g_REACTION_RELIEF_SAID, s_REACTION_RELIEF_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_REPEAT", g_REACTION_REPEAT, s_REACTION_REPEAT);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_REPEAT_FIRST", g_REACTION_REPEAT_FIRST, s_REACTION_REPEAT_FIRST);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_WORD", g_REACTION_WORD, s_REACTION_WORD);
    pyrt::xrefs::reg_access("compose.fallback_data", "REACTION_WORDS", g_REACTION_WORDS, s_REACTION_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "REFUSED_REPLIES", g_REFUSED_REPLIES, s_REFUSED_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "RELATIONSHIP_DECISION", g_RELATIONSHIP_DECISION, s_RELATIONSHIP_DECISION);
    pyrt::xrefs::reg_access("compose.fallback_data", "RELIEF", g_RELIEF, s_RELIEF);
    pyrt::xrefs::reg_access("compose.fallback_data", "REPAIR_RULE", g_REPAIR_RULE, s_REPAIR_RULE);
    pyrt::xrefs::reg_access("compose.fallback_data", "REPAIR_WHETHER_LINES", g_REPAIR_WHETHER_LINES, s_REPAIR_WHETHER_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "REPORT_FILLER", g_REPORT_FILLER, s_REPORT_FILLER);
    pyrt::xrefs::reg_access("compose.fallback_data", "REQUEST_CUES", g_REQUEST_CUES, s_REQUEST_CUES);
    pyrt::xrefs::reg_access("compose.fallback_data", "RESOLVED_AFTER", g_RESOLVED_AFTER, s_RESOLVED_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "RESOLVED_AFTER_THEM", g_RESOLVED_AFTER_THEM, s_RESOLVED_AFTER_THEM);
    pyrt::xrefs::reg_access("compose.fallback_data", "RETURNED_TO_A_THING", g_RETURNED_TO_A_THING, s_RETURNED_TO_A_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "RETURNED_TO_IT", g_RETURNED_TO_IT, s_RETURNED_TO_IT);
    pyrt::xrefs::reg_access("compose.fallback_data", "RETURNED_TO_IT_FIRST", g_RETURNED_TO_IT_FIRST, s_RETURNED_TO_IT_FIRST);
    pyrt::xrefs::reg_access("compose.fallback_data", "RING_FOR_HELP", g_RING_FOR_HELP, s_RING_FOR_HELP);
    pyrt::xrefs::reg_access("compose.fallback_data", "RING_THE_POLICE", g_RING_THE_POLICE, s_RING_THE_POLICE);
    pyrt::xrefs::reg_access("compose.fallback_data", "ROUTINE", g_ROUTINE, s_ROUTINE);
    pyrt::xrefs::reg_access("compose.fallback_data", "ROUTINE_NOT", g_ROUTINE_NOT, s_ROUTINE_NOT);
    pyrt::xrefs::reg_access("compose.fallback_data", "ROUTINE_PHRASES", g_ROUTINE_PHRASES, s_ROUTINE_PHRASES);
    pyrt::xrefs::reg_access("compose.fallback_data", "ROUTINE_WORDS", g_ROUTINE_WORDS, s_ROUTINE_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "RULED_OUT_DETERMINER", g_RULED_OUT_DETERMINER, s_RULED_OUT_DETERMINER);
    pyrt::xrefs::reg_access("compose.fallback_data", "RUMINATION_REPLIES", g_RUMINATION_REPLIES, s_RUMINATION_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "RUN_OUT_LINES", g_RUN_OUT_LINES, s_RUN_OUT_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "SAFETY_WORDS", g_SAFETY_WORDS, s_SAFETY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SAME_AGAIN", g_SAME_AGAIN, s_SAME_AGAIN);
    pyrt::xrefs::reg_access("compose.fallback_data", "SAME_AGAIN_WITH_THE_ANSWER", g_SAME_AGAIN_WITH_THE_ANSWER, s_SAME_AGAIN_WITH_THE_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "SCHOOL_TROUBLE", g_SCHOOL_TROUBLE, s_SCHOOL_TROUBLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "SCHOOL_WHO", g_SCHOOL_WHO, s_SCHOOL_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "SCHOOL_WORDS", g_SCHOOL_WORDS, s_SCHOOL_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SECRETS_KEEP_SLIPPING", g_SECRETS_KEEP_SLIPPING, s_SECRETS_KEEP_SLIPPING);
    pyrt::xrefs::reg_access("compose.fallback_data", "SEEN_TO_VERB", g_SEEN_TO_VERB, s_SEEN_TO_VERB);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_BLAME", g_SELF_BLAME, s_SELF_BLAME);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_BLAME_ASKS", g_SELF_BLAME_ASKS, s_SELF_BLAME_ASKS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_EATING_WORDS", g_SELF_EATING_WORDS, s_SELF_EATING_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_FILLER", g_SELF_FILLER, s_SELF_FILLER);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_GROUP", g_SELF_GROUP, s_SELF_GROUP);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_JOB", g_SELF_JOB, s_SELF_JOB);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_JUDGEMENT", g_SELF_JUDGEMENT, s_SELF_JUDGEMENT);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_NOBODY", g_SELF_NOBODY, s_SELF_NOBODY);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_NOUN", g_SELF_NOUN, s_SELF_NOUN);
    pyrt::xrefs::reg_access("compose.fallback_data", "SELF_SUBJECT", g_SELF_SUBJECT, s_SELF_SUBJECT);
    pyrt::xrefs::reg_access("compose.fallback_data", "SICKNESS", g_SICKNESS, s_SICKNESS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SOMEBODY_HURT", g_SOMEBODY_HURT, s_SOMEBODY_HURT);
    pyrt::xrefs::reg_access("compose.fallback_data", "SOMEONE_ELSES_DAY", g_SOMEONE_ELSES_DAY, s_SOMEONE_ELSES_DAY);
    pyrt::xrefs::reg_access("compose.fallback_data", "SOMEONE_ELSES_SHAME", g_SOMEONE_ELSES_SHAME, s_SOMEONE_ELSES_SHAME);
    pyrt::xrefs::reg_access("compose.fallback_data", "SOMEONE_GONE", g_SOMEONE_GONE, s_SOMEONE_GONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "SOMEONE_LEAVING", g_SOMEONE_LEAVING, s_SOMEONE_LEAVING);
    pyrt::xrefs::reg_access("compose.fallback_data", "SOMEONE_SERIOUSLY_ILL", g_SOMEONE_SERIOUSLY_ILL, s_SOMEONE_SERIOUSLY_ILL);
    pyrt::xrefs::reg_access("compose.fallback_data", "SOMETHING_CAME_APART", g_SOMETHING_CAME_APART, s_SOMETHING_CAME_APART);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORE_AMBIGUOUS", g_SORE_AMBIGUOUS, s_SORE_AMBIGUOUS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORE_HANDS", g_SORE_HANDS, s_SORE_HANDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORE_ORDINARY", g_SORE_ORDINARY, s_SORE_ORDINARY);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORE_SERIOUS", g_SORE_SERIOUS, s_SORE_SERIOUS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORE_WORDS", g_SORE_WORDS, s_SORE_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORRY_FOR_ASKING", g_SORRY_FOR_ASKING, s_SORRY_FOR_ASKING);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORRY_ON_ITS_OWN", g_SORRY_ON_ITS_OWN, s_SORRY_ON_ITS_OWN);
    pyrt::xrefs::reg_access("compose.fallback_data", "SORTED_AND_NO_CALL_YET", g_SORTED_AND_NO_CALL_YET, s_SORTED_AND_NO_CALL_YET);
    pyrt::xrefs::reg_access("compose.fallback_data", "STATE_CUES", g_STATE_CUES, s_STATE_CUES);
    pyrt::xrefs::reg_access("compose.fallback_data", "STILL_HERE_AFTER", g_STILL_HERE_AFTER, s_STILL_HERE_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "STRETCH_OF_TIME", g_STRETCH_OF_TIME, s_STRETCH_OF_TIME);
    pyrt::xrefs::reg_access("compose.fallback_data", "STUCK_AFTER_TROUBLE", g_STUCK_AFTER_TROUBLE, s_STUCK_AFTER_TROUBLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "SUDDEN_LOSS", g_SUDDEN_LOSS, s_SUDDEN_LOSS);
    pyrt::xrefs::reg_access("compose.fallback_data", "SUMMONS_RANG", g_SUMMONS_RANG, s_SUMMONS_RANG);
    pyrt::xrefs::reg_access("compose.fallback_data", "SUMMONS_SEEN", g_SUMMONS_SEEN, s_SUMMONS_SEEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "SUMMONS_WRITTEN", g_SUMMONS_WRITTEN, s_SUMMONS_WRITTEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "SYMPTOM", g_SYMPTOM, s_SYMPTOM);
    pyrt::xrefs::reg_access("compose.fallback_data", "SYMPTOM_COLD", g_SYMPTOM_COLD, s_SYMPTOM_COLD);
    pyrt::xrefs::reg_access("compose.fallback_data", "SYMPTOM_WORDS", g_SYMPTOM_WORDS, s_SYMPTOM_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "TAG_CONFIRM", g_TAG_CONFIRM, s_TAG_CONFIRM);
    pyrt::xrefs::reg_access("compose.fallback_data", "TAG_UNSURE", g_TAG_UNSURE, s_TAG_UNSURE);
    pyrt::xrefs::reg_access("compose.fallback_data", "TASTES", g_TASTES, s_TASTES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TASTE_GENERAL", g_TASTE_GENERAL, s_TASTE_GENERAL);
    pyrt::xrefs::reg_access("compose.fallback_data", "TELL_HARD_LINES", g_TELL_HARD_LINES, s_TELL_HARD_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TENANT_WHO", g_TENANT_WHO, s_TENANT_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "THANKS", g_THANKS, s_THANKS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THANKS_AFTER", g_THANKS_AFTER, s_THANKS_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "THANKS_ONLY", g_THANKS_ONLY, s_THANKS_ONLY);
    pyrt::xrefs::reg_access("compose.fallback_data", "THANKS_THEN_SAID", g_THANKS_THEN_SAID, s_THANKS_THEN_SAID);
    pyrt::xrefs::reg_access("compose.fallback_data", "THANKS_WORDS", g_THANKS_WORDS, s_THANKS_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THANK_WORDS", g_THANK_WORDS, s_THANK_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_BOTS_OWN_ANSWER", g_THE_BOTS_OWN_ANSWER, s_THE_BOTS_OWN_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_CALL_STANDS", g_THE_CALL_STANDS, s_THE_CALL_STANDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_CALL_STANDS_ANYWAY", g_THE_CALL_STANDS_ANYWAY, s_THE_CALL_STANDS_ANYWAY);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_DAY", g_THE_DAY, s_THE_DAY);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_DAY_AHEAD", g_THE_DAY_AHEAD, s_THE_DAY_AHEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_FIRST_HOUR_AND_A_CHILD", g_THE_FIRST_HOUR_AND_A_CHILD, s_THE_FIRST_HOUR_AND_A_CHILD);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_GROUND_OPENS", g_THE_GROUND_OPENS, s_THE_GROUND_OPENS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_MOMENT_IS_THEIRS", g_THE_MOMENT_IS_THEIRS, s_THE_MOMENT_IS_THEIRS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_PLACE_ASK", g_THE_PLACE_ASK, s_THE_PLACE_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "THE_PLACE_POOLS", g_THE_PLACE_POOLS, s_THE_PLACE_POOLS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THING_AFTER", g_THING_AFTER, s_THING_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "THING_STATES", g_THING_STATES, s_THING_STATES);
    pyrt::xrefs::reg_access("compose.fallback_data", "THING_SUBJECT", g_THING_SUBJECT, s_THING_SUBJECT);
    pyrt::xrefs::reg_access("compose.fallback_data", "THREAD", g_THREAD, s_THREAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "THREAD_PEOPLE", g_THREAD_PEOPLE, s_THREAD_PEOPLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "THREAD_TURNS", g_THREAD_TURNS, s_THREAD_TURNS);
    pyrt::xrefs::reg_access("compose.fallback_data", "THRESHOLD_CROSSED", g_THRESHOLD_CROSSED, s_THRESHOLD_CROSSED);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIGHT_MONEY", g_TIGHT_MONEY, s_TIGHT_MONEY);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIGHT_MONEY_MORE", g_TIGHT_MONEY_MORE, s_TIGHT_MONEY_MORE);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIPS_AHEAD", g_TIPS_AHEAD, s_TIPS_AHEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIPS_CLAUSE", g_TIPS_CLAUSE, s_TIPS_CLAUSE);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIPS_LINES", g_TIPS_LINES, s_TIPS_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIPS_OWNER", g_TIPS_OWNER, s_TIPS_OWNER);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIPS_THING", g_TIPS_THING, s_TIPS_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIPS_UNIT", g_TIPS_UNIT, s_TIPS_UNIT);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIPS_VERB", g_TIPS_VERB, s_TIPS_VERB);
    pyrt::xrefs::reg_access("compose.fallback_data", "TIRED", g_TIRED, s_TIRED);
    pyrt::xrefs::reg_access("compose.fallback_data", "TOO_MUCH", g_TOO_MUCH, s_TOO_MUCH);
    pyrt::xrefs::reg_access("compose.fallback_data", "TOO_MUCH_PHRASES", g_TOO_MUCH_PHRASES, s_TOO_MUCH_PHRASES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRADER_WORDS", g_TRADER_WORDS, s_TRADER_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRADES_WHO", g_TRADES_WHO, s_TRADES_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRADE_OFF_LEAN", g_TRADE_OFF_LEAN, s_TRADE_OFF_LEAN);
    pyrt::xrefs::reg_access("compose.fallback_data", "TREATMENT_HARM", g_TREATMENT_HARM, s_TREATMENT_HARM);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_HELPED_ELSEWHERE", g_TRIED_HELPED_ELSEWHERE, s_TRIED_HELPED_ELSEWHERE);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_HELPED_REPLIES", g_TRIED_HELPED_REPLIES, s_TRIED_HELPED_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_IT_REPLIES", g_TRIED_IT_REPLIES, s_TRIED_IT_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_NO_CHANGE_ELSEWHERE", g_TRIED_NO_CHANGE_ELSEWHERE, s_TRIED_NO_CHANGE_ELSEWHERE);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_NO_CHANGE_REPLIES", g_TRIED_NO_CHANGE_REPLIES, s_TRIED_NO_CHANGE_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_SORTED_REPLIES", g_TRIED_SORTED_REPLIES, s_TRIED_SORTED_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_WORSE_ELSEWHERE", g_TRIED_WORSE_ELSEWHERE, s_TRIED_WORSE_ELSEWHERE);
    pyrt::xrefs::reg_access("compose.fallback_data", "TRIED_WORSE_REPLIES", g_TRIED_WORSE_REPLIES, s_TRIED_WORSE_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_data", "TROUBLE_CUES", g_TROUBLE_CUES, s_TROUBLE_CUES);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN", g_UNKNOWN, s_UNKNOWN);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_ABOUT_A_PET", g_UNKNOWN_ABOUT_A_PET, s_UNKNOWN_ABOUT_A_PET);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_ABOUT_SOMEONE", g_UNKNOWN_ABOUT_SOMEONE, s_UNKNOWN_ABOUT_SOMEONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_DECIDE", g_UNKNOWN_DECIDE, s_UNKNOWN_DECIDE);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_HEALTH", g_UNKNOWN_HEALTH, s_UNKNOWN_HEALTH);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_MONEY", g_UNKNOWN_MONEY, s_UNKNOWN_MONEY);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_REALISTIC", g_UNKNOWN_REALISTIC, s_UNKNOWN_REALISTIC);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_REALISTIC_CARE", g_UNKNOWN_REALISTIC_CARE, s_UNKNOWN_REALISTIC_CARE);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_TOO_LATE", g_UNKNOWN_TOO_LATE, s_UNKNOWN_TOO_LATE);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_WEIRD_BARE", g_UNKNOWN_WEIRD_BARE, s_UNKNOWN_WEIRD_BARE);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_WHO", g_UNKNOWN_WHO, s_UNKNOWN_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_WORD", g_UNKNOWN_WORD, s_UNKNOWN_WORD);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_WORD_NAMED", g_UNKNOWN_WORD_NAMED, s_UNKNOWN_WORD_NAMED);
    pyrt::xrefs::reg_access("compose.fallback_data", "UNKNOWN_WORRY", g_UNKNOWN_WORRY, s_UNKNOWN_WORRY);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_ANNOY", g_VERDICT_ANNOY, s_VERDICT_ANNOY);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_BAD", g_VERDICT_BAD, s_VERDICT_BAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_FIRST", g_VERDICT_FIRST, s_VERDICT_FIRST);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_HEAVY", g_VERDICT_HEAVY, s_VERDICT_HEAVY);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_LUCK", g_VERDICT_LUCK, s_VERDICT_LUCK);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_NUMB", g_VERDICT_NUMB, s_VERDICT_NUMB);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_QUEUE", g_VERDICT_QUEUE, s_VERDICT_QUEUE);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_ROUGH", g_VERDICT_ROUGH, s_VERDICT_ROUGH);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_SETBACK", g_VERDICT_SETBACK, s_VERDICT_SETBACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_SHAME", g_VERDICT_SHAME, s_VERDICT_SHAME);
    pyrt::xrefs::reg_access("compose.fallback_data", "VERDICT_WORD", g_VERDICT_WORD, s_VERDICT_WORD);
    pyrt::xrefs::reg_access("compose.fallback_data", "VET_WHO", g_VET_WHO, s_VET_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_AFTER_A_DEATH", g_VIEW_AFTER_A_DEATH, s_VIEW_AFTER_A_DEATH);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_ASK_LINES", g_VIEW_ASK_LINES, s_VIEW_ASK_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_COMPANY", g_VIEW_COMPANY, s_VIEW_COMPANY);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_DECIDE", g_VIEW_DECIDE, s_VIEW_DECIDE);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_HANDBACK", g_VIEW_HANDBACK, s_VIEW_HANDBACK);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_LEAD_IN", g_VIEW_LEAD_IN, s_VIEW_LEAD_IN);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_LINES", g_VIEW_LINES, s_VIEW_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_MIRROR_LINES", g_VIEW_MIRROR_LINES, s_VIEW_MIRROR_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_NO_SUBJECT", g_VIEW_NO_SUBJECT, s_VIEW_NO_SUBJECT);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_ONLY_WORDS", g_VIEW_ONLY_WORDS, s_VIEW_ONLY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "VIEW_ON_GOOD_NEWS", g_VIEW_ON_GOOD_NEWS, s_VIEW_ON_GOOD_NEWS);
    pyrt::xrefs::reg_access("compose.fallback_data", "VISIT_PRONOUNS", g_VISIT_PRONOUNS, s_VISIT_PRONOUNS);
    pyrt::xrefs::reg_access("compose.fallback_data", "WALKED_INTO_POOL", g_WALKED_INTO_POOL, s_WALKED_INTO_POOL);
    pyrt::xrefs::reg_access("compose.fallback_data", "WASNT_THERE", g_WASNT_THERE, s_WASNT_THERE);
    pyrt::xrefs::reg_access("compose.fallback_data", "WHERE_SOMETHING_WRONG", g_WHERE_SOMETHING_WRONG, s_WHERE_SOMETHING_WRONG);
    pyrt::xrefs::reg_access("compose.fallback_data", "WHOLE_LIST_AFTER", g_WHOLE_LIST_AFTER, s_WHOLE_LIST_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "WHO_TO_ASK", g_WHO_TO_ASK, s_WHO_TO_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "WHY_AFTER_TASTE", g_WHY_AFTER_TASTE, s_WHY_AFTER_TASTE);
    pyrt::xrefs::reg_access("compose.fallback_data", "WILD_BIRDS", g_WILD_BIRDS, s_WILD_BIRDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "WIN_AND_A_COST", g_WIN_AND_A_COST, s_WIN_AND_A_COST);
    pyrt::xrefs::reg_access("compose.fallback_data", "WIN_IN_A_SILENCE", g_WIN_IN_A_SILENCE, s_WIN_IN_A_SILENCE);
    pyrt::xrefs::reg_access("compose.fallback_data", "WITH_YOU_AFTER", g_WITH_YOU_AFTER, s_WITH_YOU_AFTER);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORDING_LINES", g_WORDING_LINES, s_WORDING_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORDING_SOMEONE", g_WORDING_SOMEONE, s_WORDING_SOMEONE);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORD_ANYWHERE", g_WORD_ANYWHERE, s_WORD_ANYWHERE);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORD_FIXES", g_WORD_FIXES, s_WORD_FIXES);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORK_ABOUT", g_WORK_ABOUT, s_WORK_ABOUT);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORK_WHO", g_WORK_WHO, s_WORK_WHO);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORRY_FILLER", g_WORRY_FILLER, s_WORRY_FILLER);
    pyrt::xrefs::reg_access("compose.fallback_data", "WORRY_WORDS", g_WORRY_WORDS, s_WORRY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "WOULD_HAVE_BEEN", g_WOULD_HAVE_BEEN, s_WOULD_HAVE_BEEN);
    pyrt::xrefs::reg_access("compose.fallback_data", "YOURS", g_YOURS, s_YOURS);
    pyrt::xrefs::reg_access("compose.fallback_data", "YOU_SUBJECTS", g_YOU_SUBJECTS, s_YOU_SUBJECTS);
    pyrt::xrefs::reg_access("compose.fallback_data", "_ASKS_ABOUT_THE_JOINT", g__ASKS_ABOUT_THE_JOINT, s__ASKS_ABOUT_THE_JOINT);
    pyrt::xrefs::reg_access("compose.fallback_data", "_A_PERSON", g__A_PERSON, s__A_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_data", "_A_SOCIAL_THING", g__A_SOCIAL_THING, s__A_SOCIAL_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "_BOUNDED", g__BOUNDED, s__BOUNDED);
    pyrt::xrefs::reg_access("compose.fallback_data", "_CHOOSING_BETWEEN_TWO", g__CHOOSING_BETWEEN_TWO, s__CHOOSING_BETWEEN_TWO);
    pyrt::xrefs::reg_access("compose.fallback_data", "_COPULA", g__COPULA, s__COPULA);
    pyrt::xrefs::reg_access("compose.fallback_data", "_CREATIVE", g__CREATIVE, s__CREATIVE);
    pyrt::xrefs::reg_access("compose.fallback_data", "_DAY_WORDS", g__DAY_WORDS, s__DAY_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_data", "_ECHO_FILLER", g__ECHO_FILLER, s__ECHO_FILLER);
    pyrt::xrefs::reg_access("compose.fallback_data", "_ISSUER", g__ISSUER, s__ISSUER);
    pyrt::xrefs::reg_access("compose.fallback_data", "_LANDED", g__LANDED, s__LANDED);
    pyrt::xrefs::reg_access("compose.fallback_data", "_MONEY_NOUN", g__MONEY_NOUN, s__MONEY_NOUN);
    pyrt::xrefs::reg_access("compose.fallback_data", "_NOT_AN_OWN_THING", g__NOT_AN_OWN_THING, s__NOT_AN_OWN_THING);
    pyrt::xrefs::reg_access("compose.fallback_data", "_NO_DAY_OF_THEIR_OWN", g__NO_DAY_OF_THEIR_OWN, s__NO_DAY_OF_THEIR_OWN);
    pyrt::xrefs::reg_access("compose.fallback_data", "_OPENING_QUESTIONS", g__OPENING_QUESTIONS, s__OPENING_QUESTIONS);
    pyrt::xrefs::reg_access("compose.fallback_data", "_START_ASK", g__START_ASK, s__START_ASK);
    pyrt::xrefs::reg_access("compose.fallback_data", "_SUBJECT_AFTER_AM", g__SUBJECT_AFTER_AM, s__SUBJECT_AFTER_AM);
    pyrt::xrefs::reg_access("compose.fallback_data", "_THING_DONE_LINES", g__THING_DONE_LINES, s__THING_DONE_LINES);
    pyrt::xrefs::reg_access("compose.fallback_data", "_THREAD_PEOPLE", g__THREAD_PEOPLE, s__THREAD_PEOPLE);
    pyrt::xrefs::reg_access("compose.fallback_data", "_VERDICT_DET", g__VERDICT_DET, s__VERDICT_DET);
    pyrt::xrefs::reg_access("compose.fallback_data", "_VERDICT_LEAD", g__VERDICT_LEAD, s__VERDICT_LEAD);
    pyrt::xrefs::reg_access("compose.fallback_data", "_WAS", g__WAS, s__WAS);
    pyrt::xrefs::reg_access("compose.fallback_data", "_WOUND", g__WOUND, s__WOUND);
}
