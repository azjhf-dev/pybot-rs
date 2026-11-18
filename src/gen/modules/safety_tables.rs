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


pub static ABOUT_THE_SPEAKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_BENIGN_REASON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_BLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_BUTTONHOLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_CHILD_REFUSAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_EMPLOYER_REFUSAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_FINE_ANIMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_FINE_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_HANDS_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_KIDS_AT_HOME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_KIDS_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_PARENT_REFUSAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_PARENT_REFUSAL_HARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_PLAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_PLAY_HARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_QUARREL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_ROOM_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_ROOM_FAMILY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_SCARED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_SCARED_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACCIDENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADULT_HIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFTER_EXERCISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGE_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALARM_NUISANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AMBULANCE_CASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANIMAL_KICK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANY_REFUSAL_TO_GO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ITCH_LOOKALIKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static APPLIANCE_SMOKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_ABOUT_THE_INJURY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_GO_BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_IF_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_IF_GOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_CALL_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_GO_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASPIRIN_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BREATHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FOLLOW_UP_ABOUT_THE_INJURY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_GROWN_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HAND_ON_THE_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HURT_OF_A_BODY_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLAN_ORDINARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLAN_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REAL_DANGER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WISH_OR_TALK_OF_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WORKOUT_CUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BABY_FELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BABY_FELL_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BABY_FELL_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BABY_FELL_SURFACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BATTERY_BULGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BATTERY_FIRE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BITE_ON_ANIMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLACKOUT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLACKOUT_BREATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLACKOUT_EFFORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLACKOUT_FLOOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLACKOUT_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLACKOUT_REPEAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLACKOUT_SOMEONE_DOWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLEACH_MIX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLEACH_MIX_Q: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLEEDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLIND_CORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLISTER_CAUSE_BURN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLISTER_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLISTER_STATE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLOW_ON_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_CLOTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_COLD_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_OTHER_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_PERSON_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_STRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLUE_TINGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BONE_ORDINARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BONE_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BONE_THROAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_CAUSE_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_EFFORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_EFFORT_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_LAUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_LAUGH_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_PAST_OVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_REST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREATH_SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURNT_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURNT_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURN_MARK_ON_A_BODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALL_HAS_A_SIGN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALL_HELP_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CASE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEM_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_EFFORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_IN_A_HAND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_IN_A_HAND_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_SLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_SPREADING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_URGENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHEST_WORK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILDHOOD_FIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_FEVER_NOT_EATING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_FEVER_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_GROWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_SCHOOL_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_YOUNG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_AIR_OK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_NOT_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_OVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_PREVENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_SETTLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOKE_STILL_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHRONIC_BREATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLEANABLE_SURFACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COLD_ILLNESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COLLAPSE_HINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSED_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUGH_BLOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CO_GOING_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CO_LEAK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CO_SIGNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CO_TEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_BABY_SCENE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_CHILD_SCENE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_HOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_HOWTO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_HOW_BABY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_HOW_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_NOT_EFFORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_NO_PULSE_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_ROOM_EMPTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_ROUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_STOP_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CPR_WHEN_ALIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CRISIS_PLAN_CARRY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CRISIS_PLAN_ORDINARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CRISIS_PLAN_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CRISIS_PLAN_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CRISIS_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CUT_NOT_STOPPING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_TOTAL_SIGN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_TOTAL_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENTAL_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETAIL_AFTER_ESCALATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEVICE_BURNING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEVICE_OLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DIE_OF_THE_FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DISTRESS_BESIDE_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOG_HABIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOG_MOUTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOG_SELF_BITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOSE_AMOUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOSE_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOSE_COURSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOSE_COURSE_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOSE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DRINK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DROWNED_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DROWNING_HYPERBOLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DRUG_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EDGE_OF_A_DROP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_ANYWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_BENIGN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_HEART_CLUSTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_PATTERN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_RED_HEART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_TOO_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELECTRIC_BURN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELECTRIC_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELECTRIC_SHOCK_DEFINITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELECTRIC_SHOCK_DOWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static END_IT_MEANS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static END_IT_WITH_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static END_IT_WITH_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static END_MEANS_OTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static END_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXERTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXERTION_CLASS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXERTION_SMALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EYE_CHEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EYE_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FAINT_STORY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_ANSWERED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_CHECK_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_CORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_HIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_REPEAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALL_WORD_IDIOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEMALE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEVER_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEVER_RASH_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEVER_REPORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FICTION_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRE_DRILL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRE_ELSEWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRE_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRE_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRE_SCALE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FITTING_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FITTING_MANNER_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIT_ALL_BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIT_CASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIT_ROUTINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIT_STILL_RINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIT_THIS_WEEK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLOPPY_RED_FLAG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLOPPY_SLEEPY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLOPPY_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOLLOW_UP_ANCHOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOLLOW_UP_MORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOOD_CUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FORAGED_EATEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FORAGED_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRICTION_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRIGHTENED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRIGHTENED_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GARMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GASPING_THIRST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GASP_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GASP_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAS_LEFT_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAS_ON_NOT_RUNNING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAS_ON_YOU: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAS_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAS_SMELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVING_UP_GROUP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVING_UP_HELD_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GO_IN_NOT_ABOUT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GO_IS_A_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRUNTING_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAIR_SCORCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAIR_SCORCH_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANGED_LONG_AGO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANGING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HARD_TO_WAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEADACHE_BAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEADACHE_STORY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEADACHE_SUDDEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAD_BANG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAD_KNOCKOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAD_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAL_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEARD_ASSAULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELD_CRISIS_DETAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELD_CRISIS_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ARRIVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIT_IN_FUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIT_THE_LAUGH_IS_HIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO_TREAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUMAN_TOO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HYPERBOLE_BLOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HYPO_HOW_KNOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HYPO_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HYPO_STRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HYPO_STRONG_OTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INJURY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INTRUDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KICKED_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMP_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMP_MECHANISM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMP_OTHER_LIMB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMP_SOMEBODY_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIPS_COLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIPS_SWELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIPS_SWELL_BARE_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIPS_TINGLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIPS_TINGLE_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIPS_TINGLE_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIPS_TINGLE_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCKED_IN_RISK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOKS_REAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MACHINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEAL_BREATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEDICINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MENINGITIS_OTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MILD_BODY_SIGN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MILD_CHOKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MILD_PRODUCT_SMALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MILD_SWALLOWED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MORNING_ROUTINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOUTH_BLEED_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_SICK_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEAR_FIRE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEAR_FIRE_OVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEAR_THING_OVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEWBORN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NIGHT_BREATH_OTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NIGHT_BREATH_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NIGHT_BREATH_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOSE_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_WOUND_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_NOW_FICTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_STOPPING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OBJECT_IN_HEAD_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OLDER_CHILD_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OLD_FALL_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OLD_FALL_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OLD_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OLD_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_RISK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_SELF_HARM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OUT_COLD_ASLEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OUT_COLD_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OUT_COLD_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OVERDOSE_DRUG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OVERDOSE_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OVERDOSE_OF_A_TREAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_CRY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_INJURY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_INJURY_IN_MY_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_INJURY_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_LIMB_AT_AN_ANGLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PALE_EFFORT_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PALE_EXERTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PALE_HEAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PALE_SECOND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAN_FIRE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PARENT_REFUSAL_BLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_ATTEMPT_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_WISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_WISH_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_AS_ATTACKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_BURNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_ALL_RIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_GASP_EXPLAINED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_GASP_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_GASP_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_RESPOND_AT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_TOOTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAYFUL_HIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POISON_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POO_BLACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POO_BLOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POO_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREGNANCY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUNCTURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUSHED_AWAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUSHED_AWAY_BUT_SOMETHING_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUSHED_AWAY_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUSHED_AWAY_THEN_RESTRAINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RASH_FADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RASH_HEADACHE_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RASH_PRESSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RAZOR_BURN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECENT_FIT_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECENT_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RESPOND_SENSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RESTRAINT_BESIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RING_DANGER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RING_NOT_WORN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROUTINE_REASON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROUTINE_WAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAFE_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_AS_IT_HAPPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEIZURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEIZURE_FIGURATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEIZURE_STORY_GUARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_CUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_BY_A_MEANS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_CUT_NO_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_DELIBERATE_ACT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_DESERVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_LOOKALIKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_NOT_EATING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_OWN_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_REASON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_SCARS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_SCARS_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_TOPIC_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_WISHES_A_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_RUMINATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_TALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEPSIS_SIGN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHAKEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHAKEN_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOCK_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_OF_BREATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOT_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIGHT_A_WHILE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLUR_RED_BESIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLUR_THICK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SMALL_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SMALL_CHILD_LASH_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SMOKE_IS_A_SMELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SNAKE_LOOKALIKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOCKET_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOCKET_THING_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT_BLEED_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSES_DESPAIR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSE_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOME_OF_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPELL_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPIKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPINE_BACK_WORK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPINE_BREAK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPOKEN_ATTEMPT_UNDATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STIFF_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STIFF_NECK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_GOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STING_SETTLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOOL_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRANGLED_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRANGLE_THING_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STREAK_FEVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STROKE_HAND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STROKE_HAND_BESIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STROKE_HAND_CHRONIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STROKE_HAND_HARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STROKE_WEAK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STROKE_WEAK_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STUCK_MACHINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUICIDE_ABOUT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUICIDE_TOPIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWALLOWED_BAD_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWALLOWED_BAD_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWALLOWED_ORDINARY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWALLOWED_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWALLOW_GONE_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWALLOW_GONE_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_HABIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TESTIS_LUMP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_BURNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_BURNT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_THAT_IS_SUICIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THIRD_PERSON_ACTOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THROAT_TIGHT_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THROAT_TIGHT_EFFORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THROAT_TIGHT_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TINY_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TINY_AGE_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOOK_OUT_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOOTH_OUT_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TORSION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TORSION_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TORSION_ONSET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRAPPED_HELD_BY_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRAPPED_VEHICLE_NOT_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TYPED_TYPOS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VEHICLE_BURNING_SMELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VEHICLE_BURNING_SMELL_HARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VEHICLE_EXHAUST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VEHICLE_FIRE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VEHICLE_FIRE_OVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VIOLENT_THREAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VISIT_OR_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAKE_RED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAKE_UP_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_DROWNING_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_DROWNING_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_HEAD_FEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_HUMAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_IN_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_NOT_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_PERSON_TOO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_THING_DROPPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATER_URGENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEIGHT_HIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEIGHT_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEIGHT_OFF_FEET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_BLUE_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_BLUE_IDIOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_HELP_COMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_HELP_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHOLE_BODY_RED_FLAG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WISH_BY_MODAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WISH_DEAD_HYPERBOLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WISH_DEAD_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WISH_DEAD_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WISH_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_ALONE_AT_HOME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_ASK_DANGER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_DANGER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_HAIR_OF_THE_DOG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_HALLUCINATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_NOT_ALCOHOL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_ROUTINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_SENSES_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_STRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHDRAWAL_SYMPTOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WONT_COME_ROUND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORDS_OUT_OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORLD_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WOUND_STREAK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WOUND_STREAK_FEVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WOUND_STREAK_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AGREEMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ANY_MEANS_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AWAY_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_MEANS_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PURPOSE_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_THIRST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DENIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FITTING_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FLAT_WISH_HOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _GASP_NOT_A_THIRST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HEIGHT_MEANS_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _INTO_THE_WAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OFF_AN_EDGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OFF_A_HEIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SMELLY_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SMOKE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STREAK_MARK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STREAK_MOVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STREAK_ON_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STREAK_REDNESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STRUCK_RELATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THIRD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_THE_SPEAKER() -> Py { ensure_init(); ABOUT_THE_SPEAKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_SPEAKER(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THE_SPEAKER.write() { *w = v; } }
pub fn g_ABUSE_BENIGN_REASON() -> Py { ensure_init(); ABUSE_BENIGN_REASON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_BENIGN_REASON(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_BENIGN_REASON.write() { *w = v; } }
pub fn g_ABUSE_BLOW() -> Py { ensure_init(); ABUSE_BLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_BLOW(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_BLOW.write() { *w = v; } }
pub fn g_ABUSE_BUTTONHOLED() -> Py { ensure_init(); ABUSE_BUTTONHOLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_BUTTONHOLED(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_BUTTONHOLED.write() { *w = v; } }
pub fn g_ABUSE_CHILD_REFUSAL() -> Py { ensure_init(); ABUSE_CHILD_REFUSAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_CHILD_REFUSAL(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_CHILD_REFUSAL.write() { *w = v; } }
pub fn g_ABUSE_EMPLOYER_REFUSAL() -> Py { ensure_init(); ABUSE_EMPLOYER_REFUSAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_EMPLOYER_REFUSAL(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_EMPLOYER_REFUSAL.write() { *w = v; } }
pub fn g_ABUSE_FINE_ANIMAL() -> Py { ensure_init(); ABUSE_FINE_ANIMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_FINE_ANIMAL(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_FINE_ANIMAL.write() { *w = v; } }
pub fn g_ABUSE_FINE_PLACE() -> Py { ensure_init(); ABUSE_FINE_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_FINE_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_FINE_PLACE.write() { *w = v; } }
pub fn g_ABUSE_HANDS_ON() -> Py { ensure_init(); ABUSE_HANDS_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_HANDS_ON(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_HANDS_ON.write() { *w = v; } }
pub fn g_ABUSE_KIDS_AT_HOME() -> Py { ensure_init(); ABUSE_KIDS_AT_HOME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_KIDS_AT_HOME(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_KIDS_AT_HOME.write() { *w = v; } }
pub fn g_ABUSE_KIDS_CARD() -> Py { ensure_init(); ABUSE_KIDS_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_KIDS_CARD(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_KIDS_CARD.write() { *w = v; } }
pub fn g_ABUSE_PARENT_REFUSAL() -> Py { ensure_init(); ABUSE_PARENT_REFUSAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_PARENT_REFUSAL(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_PARENT_REFUSAL.write() { *w = v; } }
pub fn g_ABUSE_PARENT_REFUSAL_HARD() -> Py { ensure_init(); ABUSE_PARENT_REFUSAL_HARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_PARENT_REFUSAL_HARD(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_PARENT_REFUSAL_HARD.write() { *w = v; } }
pub fn g_ABUSE_PLAY() -> Py { ensure_init(); ABUSE_PLAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_PLAY(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_PLAY.write() { *w = v; } }
pub fn g_ABUSE_PLAY_HARD() -> Py { ensure_init(); ABUSE_PLAY_HARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_PLAY_HARD(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_PLAY_HARD.write() { *w = v; } }
pub fn g_ABUSE_QUARREL() -> Py { ensure_init(); ABUSE_QUARREL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_QUARREL(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_QUARREL.write() { *w = v; } }
pub fn g_ABUSE_ROOM_CARD() -> Py { ensure_init(); ABUSE_ROOM_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_ROOM_CARD(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_ROOM_CARD.write() { *w = v; } }
pub fn g_ABUSE_ROOM_FAMILY() -> Py { ensure_init(); ABUSE_ROOM_FAMILY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_ROOM_FAMILY(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_ROOM_FAMILY.write() { *w = v; } }
pub fn g_ABUSE_SCARED() -> Py { ensure_init(); ABUSE_SCARED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_SCARED(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_SCARED.write() { *w = v; } }
pub fn g_ABUSE_SCARED_REPLY() -> Py { ensure_init(); ABUSE_SCARED_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_SCARED_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_SCARED_REPLY.write() { *w = v; } }
pub fn g_ACCIDENT() -> Py { ensure_init(); ACCIDENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACCIDENT(v: Py) { ensure_init(); if let Ok(mut w) = ACCIDENT.write() { *w = v; } }
pub fn g_ADULT_HIT() -> Py { ensure_init(); ADULT_HIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADULT_HIT(v: Py) { ensure_init(); if let Ok(mut w) = ADULT_HIT.write() { *w = v; } }
pub fn g_AFTER_EXERCISE() -> Py { ensure_init(); AFTER_EXERCISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_EXERCISE(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_EXERCISE.write() { *w = v; } }
pub fn g_AGE_PERSON() -> Py { ensure_init(); AGE_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGE_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = AGE_PERSON.write() { *w = v; } }
pub fn g_ALARM_NUISANCE() -> Py { ensure_init(); ALARM_NUISANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALARM_NUISANCE(v: Py) { ensure_init(); if let Ok(mut w) = ALARM_NUISANCE.write() { *w = v; } }
pub fn g_AMBULANCE_CASES() -> Py { ensure_init(); AMBULANCE_CASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AMBULANCE_CASES(v: Py) { ensure_init(); if let Ok(mut w) = AMBULANCE_CASES.write() { *w = v; } }
pub fn g_ANIMAL_KICK() -> Py { ensure_init(); ANIMAL_KICK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_KICK(v: Py) { ensure_init(); if let Ok(mut w) = ANIMAL_KICK.write() { *w = v; } }
pub fn g_ANY_REFUSAL_TO_GO() -> Py { ensure_init(); ANY_REFUSAL_TO_GO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANY_REFUSAL_TO_GO(v: Py) { ensure_init(); if let Ok(mut w) = ANY_REFUSAL_TO_GO.write() { *w = v; } }
pub fn g_AN_ITCH_LOOKALIKE() -> Py { ensure_init(); AN_ITCH_LOOKALIKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ITCH_LOOKALIKE(v: Py) { ensure_init(); if let Ok(mut w) = AN_ITCH_LOOKALIKE.write() { *w = v; } }
pub fn g_APPLIANCE_SMOKE() -> Py { ensure_init(); APPLIANCE_SMOKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_APPLIANCE_SMOKE(v: Py) { ensure_init(); if let Ok(mut w) = APPLIANCE_SMOKE.write() { *w = v; } }
pub fn g_ASKS_ABOUT_THE_INJURY() -> Py { ensure_init(); ASKS_ABOUT_THE_INJURY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_THE_INJURY(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_ABOUT_THE_INJURY.write() { *w = v; } }
pub fn g_ASKS_GO_BARE() -> Py { ensure_init(); ASKS_GO_BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_GO_BARE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_GO_BARE.write() { *w = v; } }
pub fn g_ASKS_IF_CALL() -> Py { ensure_init(); ASKS_IF_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_IF_CALL(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_IF_CALL.write() { *w = v; } }
pub fn g_ASKS_IF_GOING() -> Py { ensure_init(); ASKS_IF_GOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_IF_GOING(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_IF_GOING.write() { *w = v; } }
pub fn g_ASK_CALL_REPLIES() -> Py { ensure_init(); ASK_CALL_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_CALL_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = ASK_CALL_REPLIES.write() { *w = v; } }
pub fn g_ASK_GO_REPLIES() -> Py { ensure_init(); ASK_GO_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_GO_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = ASK_GO_REPLIES.write() { *w = v; } }
pub fn g_ASPIRIN_REPLY() -> Py { ensure_init(); ASPIRIN_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASPIRIN_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = ASPIRIN_REPLY.write() { *w = v; } }
pub fn g_A_BREATHER() -> Py { ensure_init(); A_BREATHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BREATHER(v: Py) { ensure_init(); if let Ok(mut w) = A_BREATHER.write() { *w = v; } }
pub fn g_A_FOLLOW_UP_ABOUT_THE_INJURY() -> Py { ensure_init(); A_FOLLOW_UP_ABOUT_THE_INJURY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FOLLOW_UP_ABOUT_THE_INJURY(v: Py) { ensure_init(); if let Ok(mut w) = A_FOLLOW_UP_ABOUT_THE_INJURY.write() { *w = v; } }
pub fn g_A_GROWN_AGE() -> Py { ensure_init(); A_GROWN_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_GROWN_AGE(v: Py) { ensure_init(); if let Ok(mut w) = A_GROWN_AGE.write() { *w = v; } }
pub fn g_A_HAND_ON_THE_PERSON() -> Py { ensure_init(); A_HAND_ON_THE_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HAND_ON_THE_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = A_HAND_ON_THE_PERSON.write() { *w = v; } }
pub fn g_A_HURT_OF_A_BODY_PART() -> Py { ensure_init(); A_HURT_OF_A_BODY_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HURT_OF_A_BODY_PART(v: Py) { ensure_init(); if let Ok(mut w) = A_HURT_OF_A_BODY_PART.write() { *w = v; } }
pub fn g_A_PERSON() -> Py { ensure_init(); A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON.write() { *w = v; } }
pub fn g_A_PLAN_ORDINARY() -> Py { ensure_init(); A_PLAN_ORDINARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_ORDINARY(v: Py) { ensure_init(); if let Ok(mut w) = A_PLAN_ORDINARY.write() { *w = v; } }
pub fn g_A_PLAN_SAID() -> Py { ensure_init(); A_PLAN_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_PLAN_SAID.write() { *w = v; } }
pub fn g_A_REAL_DANGER() -> Py { ensure_init(); A_REAL_DANGER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REAL_DANGER(v: Py) { ensure_init(); if let Ok(mut w) = A_REAL_DANGER.write() { *w = v; } }
pub fn g_A_WISH_OR_TALK_OF_IT() -> Py { ensure_init(); A_WISH_OR_TALK_OF_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WISH_OR_TALK_OF_IT(v: Py) { ensure_init(); if let Ok(mut w) = A_WISH_OR_TALK_OF_IT.write() { *w = v; } }
pub fn g_A_WORKOUT_CUE() -> Py { ensure_init(); A_WORKOUT_CUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WORKOUT_CUE(v: Py) { ensure_init(); if let Ok(mut w) = A_WORKOUT_CUE.write() { *w = v; } }
pub fn g_BABY_FELL() -> Py { ensure_init(); BABY_FELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BABY_FELL(v: Py) { ensure_init(); if let Ok(mut w) = BABY_FELL.write() { *w = v; } }
pub fn g_BABY_FELL_OFF() -> Py { ensure_init(); BABY_FELL_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BABY_FELL_OFF(v: Py) { ensure_init(); if let Ok(mut w) = BABY_FELL_OFF.write() { *w = v; } }
pub fn g_BABY_FELL_RED() -> Py { ensure_init(); BABY_FELL_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BABY_FELL_RED(v: Py) { ensure_init(); if let Ok(mut w) = BABY_FELL_RED.write() { *w = v; } }
pub fn g_BABY_FELL_SURFACE() -> Py { ensure_init(); BABY_FELL_SURFACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BABY_FELL_SURFACE(v: Py) { ensure_init(); if let Ok(mut w) = BABY_FELL_SURFACE.write() { *w = v; } }
pub fn g_BATTERY_BULGE() -> Py { ensure_init(); BATTERY_BULGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BATTERY_BULGE(v: Py) { ensure_init(); if let Ok(mut w) = BATTERY_BULGE.write() { *w = v; } }
pub fn g_BATTERY_FIRE() -> Py { ensure_init(); BATTERY_FIRE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BATTERY_FIRE(v: Py) { ensure_init(); if let Ok(mut w) = BATTERY_FIRE.write() { *w = v; } }
pub fn g_BITE_ON_ANIMAL() -> Py { ensure_init(); BITE_ON_ANIMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BITE_ON_ANIMAL(v: Py) { ensure_init(); if let Ok(mut w) = BITE_ON_ANIMAL.write() { *w = v; } }
pub fn g_BLACKOUT_A_THING() -> Py { ensure_init(); BLACKOUT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLACKOUT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = BLACKOUT_A_THING.write() { *w = v; } }
pub fn g_BLACKOUT_BREATH() -> Py { ensure_init(); BLACKOUT_BREATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLACKOUT_BREATH(v: Py) { ensure_init(); if let Ok(mut w) = BLACKOUT_BREATH.write() { *w = v; } }
pub fn g_BLACKOUT_EFFORT() -> Py { ensure_init(); BLACKOUT_EFFORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLACKOUT_EFFORT(v: Py) { ensure_init(); if let Ok(mut w) = BLACKOUT_EFFORT.write() { *w = v; } }
pub fn g_BLACKOUT_FLOOR() -> Py { ensure_init(); BLACKOUT_FLOOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLACKOUT_FLOOR(v: Py) { ensure_init(); if let Ok(mut w) = BLACKOUT_FLOOR.write() { *w = v; } }
pub fn g_BLACKOUT_OFF() -> Py { ensure_init(); BLACKOUT_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLACKOUT_OFF(v: Py) { ensure_init(); if let Ok(mut w) = BLACKOUT_OFF.write() { *w = v; } }
pub fn g_BLACKOUT_REPEAT() -> Py { ensure_init(); BLACKOUT_REPEAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLACKOUT_REPEAT(v: Py) { ensure_init(); if let Ok(mut w) = BLACKOUT_REPEAT.write() { *w = v; } }
pub fn g_BLACKOUT_SOMEONE_DOWN() -> Py { ensure_init(); BLACKOUT_SOMEONE_DOWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLACKOUT_SOMEONE_DOWN(v: Py) { ensure_init(); if let Ok(mut w) = BLACKOUT_SOMEONE_DOWN.write() { *w = v; } }
pub fn g_BLEACH_MIX() -> Py { ensure_init(); BLEACH_MIX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLEACH_MIX(v: Py) { ensure_init(); if let Ok(mut w) = BLEACH_MIX.write() { *w = v; } }
pub fn g_BLEACH_MIX_Q() -> Py { ensure_init(); BLEACH_MIX_Q.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLEACH_MIX_Q(v: Py) { ensure_init(); if let Ok(mut w) = BLEACH_MIX_Q.write() { *w = v; } }
pub fn g_BLEEDING() -> Py { ensure_init(); BLEEDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLEEDING(v: Py) { ensure_init(); if let Ok(mut w) = BLEEDING.write() { *w = v; } }
pub fn g_BLIND_CORD() -> Py { ensure_init(); BLIND_CORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLIND_CORD(v: Py) { ensure_init(); if let Ok(mut w) = BLIND_CORD.write() { *w = v; } }
pub fn g_BLISTER_CAUSE_BURN() -> Py { ensure_init(); BLISTER_CAUSE_BURN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLISTER_CAUSE_BURN(v: Py) { ensure_init(); if let Ok(mut w) = BLISTER_CAUSE_BURN.write() { *w = v; } }
pub fn g_BLISTER_STATE() -> Py { ensure_init(); BLISTER_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLISTER_STATE(v: Py) { ensure_init(); if let Ok(mut w) = BLISTER_STATE.write() { *w = v; } }
pub fn g_BLISTER_STATE_REPLIES() -> Py { ensure_init(); BLISTER_STATE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLISTER_STATE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = BLISTER_STATE_REPLIES.write() { *w = v; } }
pub fn g_BLOW_ON_ME() -> Py { ensure_init(); BLOW_ON_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLOW_ON_ME(v: Py) { ensure_init(); if let Ok(mut w) = BLOW_ON_ME.write() { *w = v; } }
pub fn g_BLUE() -> Py { ensure_init(); BLUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE(v: Py) { ensure_init(); if let Ok(mut w) = BLUE.write() { *w = v; } }
pub fn g_BLUE_CLOTH() -> Py { ensure_init(); BLUE_CLOTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_CLOTH(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_CLOTH.write() { *w = v; } }
pub fn g_BLUE_COLD_CAUSE() -> Py { ensure_init(); BLUE_COLD_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_COLD_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_COLD_CAUSE.write() { *w = v; } }
pub fn g_BLUE_FINE() -> Py { ensure_init(); BLUE_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_FINE(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_FINE.write() { *w = v; } }
pub fn g_BLUE_FOOD() -> Py { ensure_init(); BLUE_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_FOOD.write() { *w = v; } }
pub fn g_BLUE_OTHER_PERSON() -> Py { ensure_init(); BLUE_OTHER_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_OTHER_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_OTHER_PERSON.write() { *w = v; } }
pub fn g_BLUE_PERSON_SUBJECT() -> Py { ensure_init(); BLUE_PERSON_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_PERSON_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_PERSON_SUBJECT.write() { *w = v; } }
pub fn g_BLUE_STRONG() -> Py { ensure_init(); BLUE_STRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_STRONG(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_STRONG.write() { *w = v; } }
pub fn g_BLUE_TINGE() -> Py { ensure_init(); BLUE_TINGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLUE_TINGE(v: Py) { ensure_init(); if let Ok(mut w) = BLUE_TINGE.write() { *w = v; } }
pub fn g_BODY_PART() -> Py { ensure_init(); BODY_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_PART(v: Py) { ensure_init(); if let Ok(mut w) = BODY_PART.write() { *w = v; } }
pub fn g_BONE_ORDINARY() -> Py { ensure_init(); BONE_ORDINARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BONE_ORDINARY(v: Py) { ensure_init(); if let Ok(mut w) = BONE_ORDINARY.write() { *w = v; } }
pub fn g_BONE_PAST() -> Py { ensure_init(); BONE_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BONE_PAST(v: Py) { ensure_init(); if let Ok(mut w) = BONE_PAST.write() { *w = v; } }
pub fn g_BONE_THROAT() -> Py { ensure_init(); BONE_THROAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BONE_THROAT(v: Py) { ensure_init(); if let Ok(mut w) = BONE_THROAT.write() { *w = v; } }
pub fn g_BREATH_CAUSE() -> Py { ensure_init(); BREATH_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_CAUSE.write() { *w = v; } }
pub fn g_BREATH_CAUSE_RED() -> Py { ensure_init(); BREATH_CAUSE_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_CAUSE_RED(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_CAUSE_RED.write() { *w = v; } }
pub fn g_BREATH_EFFORT() -> Py { ensure_init(); BREATH_EFFORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_EFFORT(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_EFFORT.write() { *w = v; } }
pub fn g_BREATH_EFFORT_RED() -> Py { ensure_init(); BREATH_EFFORT_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_EFFORT_RED(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_EFFORT_RED.write() { *w = v; } }
pub fn g_BREATH_LAUGH() -> Py { ensure_init(); BREATH_LAUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_LAUGH(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_LAUGH.write() { *w = v; } }
pub fn g_BREATH_LAUGH_REPLY() -> Py { ensure_init(); BREATH_LAUGH_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_LAUGH_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_LAUGH_REPLY.write() { *w = v; } }
pub fn g_BREATH_PAST_OVER() -> Py { ensure_init(); BREATH_PAST_OVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_PAST_OVER(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_PAST_OVER.write() { *w = v; } }
pub fn g_BREATH_RED() -> Py { ensure_init(); BREATH_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_RED(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_RED.write() { *w = v; } }
pub fn g_BREATH_REST() -> Py { ensure_init(); BREATH_REST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_REST(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_REST.write() { *w = v; } }
pub fn g_BREATH_SHORT() -> Py { ensure_init(); BREATH_SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREATH_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = BREATH_SHORT.write() { *w = v; } }
pub fn g_BURNT_FOOD() -> Py { ensure_init(); BURNT_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURNT_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = BURNT_FOOD.write() { *w = v; } }
pub fn g_BURNT_SELF() -> Py { ensure_init(); BURNT_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURNT_SELF(v: Py) { ensure_init(); if let Ok(mut w) = BURNT_SELF.write() { *w = v; } }
pub fn g_BURN_MARK_ON_A_BODY() -> Py { ensure_init(); BURN_MARK_ON_A_BODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURN_MARK_ON_A_BODY(v: Py) { ensure_init(); if let Ok(mut w) = BURN_MARK_ON_A_BODY.write() { *w = v; } }
pub fn g_CALL_HAS_A_SIGN() -> Py { ensure_init(); CALL_HAS_A_SIGN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALL_HAS_A_SIGN(v: Py) { ensure_init(); if let Ok(mut w) = CALL_HAS_A_SIGN.write() { *w = v; } }
pub fn g_CALL_HELP_REPLY() -> Py { ensure_init(); CALL_HELP_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALL_HELP_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = CALL_HELP_REPLY.write() { *w = v; } }
pub fn g_CASE_WORDS() -> Py { ensure_init(); CASE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CASE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = CASE_WORDS.write() { *w = v; } }
pub fn g_CHEM_WORD() -> Py { ensure_init(); CHEM_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEM_WORD(v: Py) { ensure_init(); if let Ok(mut w) = CHEM_WORD.write() { *w = v; } }
pub fn g_CHEST_ASKED() -> Py { ensure_init(); CHEST_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_ASKED.write() { *w = v; } }
pub fn g_CHEST_EFFORT() -> Py { ensure_init(); CHEST_EFFORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_EFFORT(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_EFFORT.write() { *w = v; } }
pub fn g_CHEST_IN_A_HAND() -> Py { ensure_init(); CHEST_IN_A_HAND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_IN_A_HAND(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_IN_A_HAND.write() { *w = v; } }
pub fn g_CHEST_IN_A_HAND_TAIL() -> Py { ensure_init(); CHEST_IN_A_HAND_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_IN_A_HAND_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_IN_A_HAND_TAIL.write() { *w = v; } }
pub fn g_CHEST_SLOW() -> Py { ensure_init(); CHEST_SLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_SLOW(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_SLOW.write() { *w = v; } }
pub fn g_CHEST_SPREADING() -> Py { ensure_init(); CHEST_SPREADING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_SPREADING(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_SPREADING.write() { *w = v; } }
pub fn g_CHEST_URGENT() -> Py { ensure_init(); CHEST_URGENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_URGENT(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_URGENT.write() { *w = v; } }
pub fn g_CHEST_WORK() -> Py { ensure_init(); CHEST_WORK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHEST_WORK(v: Py) { ensure_init(); if let Ok(mut w) = CHEST_WORK.write() { *w = v; } }
pub fn g_CHILDHOOD_FIT() -> Py { ensure_init(); CHILDHOOD_FIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILDHOOD_FIT(v: Py) { ensure_init(); if let Ok(mut w) = CHILDHOOD_FIT.write() { *w = v; } }
pub fn g_CHILD_FEVER_NOT_EATING() -> Py { ensure_init(); CHILD_FEVER_NOT_EATING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_FEVER_NOT_EATING(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_FEVER_NOT_EATING.write() { *w = v; } }
pub fn g_CHILD_FEVER_RED() -> Py { ensure_init(); CHILD_FEVER_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_FEVER_RED(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_FEVER_RED.write() { *w = v; } }
pub fn g_CHILD_GROWN() -> Py { ensure_init(); CHILD_GROWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_GROWN(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_GROWN.write() { *w = v; } }
pub fn g_CHILD_SCHOOL_AGE() -> Py { ensure_init(); CHILD_SCHOOL_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_SCHOOL_AGE(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_SCHOOL_AGE.write() { *w = v; } }
pub fn g_CHILD_YOUNG() -> Py { ensure_init(); CHILD_YOUNG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_YOUNG(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_YOUNG.write() { *w = v; } }
pub fn g_CHOKE_AIR_OK() -> Py { ensure_init(); CHOKE_AIR_OK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_AIR_OK(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_AIR_OK.write() { *w = v; } }
pub fn g_CHOKE_NOT_PERSON() -> Py { ensure_init(); CHOKE_NOT_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_NOT_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_NOT_PERSON.write() { *w = v; } }
pub fn g_CHOKE_OVER() -> Py { ensure_init(); CHOKE_OVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_OVER(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_OVER.write() { *w = v; } }
pub fn g_CHOKE_PERSON() -> Py { ensure_init(); CHOKE_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_PERSON.write() { *w = v; } }
pub fn g_CHOKE_PREVENT() -> Py { ensure_init(); CHOKE_PREVENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_PREVENT(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_PREVENT.write() { *w = v; } }
pub fn g_CHOKE_RED() -> Py { ensure_init(); CHOKE_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_RED(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_RED.write() { *w = v; } }
pub fn g_CHOKE_SETTLED() -> Py { ensure_init(); CHOKE_SETTLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_SETTLED(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_SETTLED.write() { *w = v; } }
pub fn g_CHOKE_STILL_RED() -> Py { ensure_init(); CHOKE_STILL_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOKE_STILL_RED(v: Py) { ensure_init(); if let Ok(mut w) = CHOKE_STILL_RED.write() { *w = v; } }
pub fn g_CHRONIC_BREATH() -> Py { ensure_init(); CHRONIC_BREATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHRONIC_BREATH(v: Py) { ensure_init(); if let Ok(mut w) = CHRONIC_BREATH.write() { *w = v; } }
pub fn g_CLEANABLE_SURFACE() -> Py { ensure_init(); CLEANABLE_SURFACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLEANABLE_SURFACE(v: Py) { ensure_init(); if let Ok(mut w) = CLEANABLE_SURFACE.write() { *w = v; } }
pub fn g_COLD_ILLNESS() -> Py { ensure_init(); COLD_ILLNESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COLD_ILLNESS(v: Py) { ensure_init(); if let Ok(mut w) = COLD_ILLNESS.write() { *w = v; } }
pub fn g_COLLAPSE_HINT() -> Py { ensure_init(); COLLAPSE_HINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COLLAPSE_HINT(v: Py) { ensure_init(); if let Ok(mut w) = COLLAPSE_HINT.write() { *w = v; } }
pub fn g_CONFUSED_THING() -> Py { ensure_init(); CONFUSED_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSED_THING(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSED_THING.write() { *w = v; } }
pub fn g_COUGH_BLOOD() -> Py { ensure_init(); COUGH_BLOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUGH_BLOOD(v: Py) { ensure_init(); if let Ok(mut w) = COUGH_BLOOD.write() { *w = v; } }
pub fn g_CO_GOING_OFF() -> Py { ensure_init(); CO_GOING_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CO_GOING_OFF(v: Py) { ensure_init(); if let Ok(mut w) = CO_GOING_OFF.write() { *w = v; } }
pub fn g_CO_LEAK() -> Py { ensure_init(); CO_LEAK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CO_LEAK(v: Py) { ensure_init(); if let Ok(mut w) = CO_LEAK.write() { *w = v; } }
pub fn g_CO_SIGNS() -> Py { ensure_init(); CO_SIGNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CO_SIGNS(v: Py) { ensure_init(); if let Ok(mut w) = CO_SIGNS.write() { *w = v; } }
pub fn g_CO_TEST() -> Py { ensure_init(); CO_TEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CO_TEST(v: Py) { ensure_init(); if let Ok(mut w) = CO_TEST.write() { *w = v; } }
pub fn g_CPR_BABY_SCENE() -> Py { ensure_init(); CPR_BABY_SCENE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_BABY_SCENE(v: Py) { ensure_init(); if let Ok(mut w) = CPR_BABY_SCENE.write() { *w = v; } }
pub fn g_CPR_CHILD() -> Py { ensure_init(); CPR_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = CPR_CHILD.write() { *w = v; } }
pub fn g_CPR_CHILD_SCENE() -> Py { ensure_init(); CPR_CHILD_SCENE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_CHILD_SCENE(v: Py) { ensure_init(); if let Ok(mut w) = CPR_CHILD_SCENE.write() { *w = v; } }
pub fn g_CPR_HOW() -> Py { ensure_init(); CPR_HOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_HOW(v: Py) { ensure_init(); if let Ok(mut w) = CPR_HOW.write() { *w = v; } }
pub fn g_CPR_HOWTO() -> Py { ensure_init(); CPR_HOWTO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_HOWTO(v: Py) { ensure_init(); if let Ok(mut w) = CPR_HOWTO.write() { *w = v; } }
pub fn g_CPR_HOW_BABY() -> Py { ensure_init(); CPR_HOW_BABY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_HOW_BABY(v: Py) { ensure_init(); if let Ok(mut w) = CPR_HOW_BABY.write() { *w = v; } }
pub fn g_CPR_HOW_CHILD() -> Py { ensure_init(); CPR_HOW_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_HOW_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = CPR_HOW_CHILD.write() { *w = v; } }
pub fn g_CPR_NOT_EFFORT() -> Py { ensure_init(); CPR_NOT_EFFORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_NOT_EFFORT(v: Py) { ensure_init(); if let Ok(mut w) = CPR_NOT_EFFORT.write() { *w = v; } }
pub fn g_CPR_NO_PULSE_NOW() -> Py { ensure_init(); CPR_NO_PULSE_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_NO_PULSE_NOW(v: Py) { ensure_init(); if let Ok(mut w) = CPR_NO_PULSE_NOW.write() { *w = v; } }
pub fn g_CPR_ROOM_EMPTY() -> Py { ensure_init(); CPR_ROOM_EMPTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_ROOM_EMPTY(v: Py) { ensure_init(); if let Ok(mut w) = CPR_ROOM_EMPTY.write() { *w = v; } }
pub fn g_CPR_ROUGH() -> Py { ensure_init(); CPR_ROUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_ROUGH(v: Py) { ensure_init(); if let Ok(mut w) = CPR_ROUGH.write() { *w = v; } }
pub fn g_CPR_START() -> Py { ensure_init(); CPR_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_START(v: Py) { ensure_init(); if let Ok(mut w) = CPR_START.write() { *w = v; } }
pub fn g_CPR_STOP() -> Py { ensure_init(); CPR_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_STOP(v: Py) { ensure_init(); if let Ok(mut w) = CPR_STOP.write() { *w = v; } }
pub fn g_CPR_STOP_REPLY() -> Py { ensure_init(); CPR_STOP_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_STOP_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = CPR_STOP_REPLY.write() { *w = v; } }
pub fn g_CPR_WHEN_ALIVE() -> Py { ensure_init(); CPR_WHEN_ALIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CPR_WHEN_ALIVE(v: Py) { ensure_init(); if let Ok(mut w) = CPR_WHEN_ALIVE.write() { *w = v; } }
pub fn g_CRISIS_PLAN_CARRY() -> Py { ensure_init(); CRISIS_PLAN_CARRY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CRISIS_PLAN_CARRY(v: Py) { ensure_init(); if let Ok(mut w) = CRISIS_PLAN_CARRY.write() { *w = v; } }
pub fn g_CRISIS_PLAN_ORDINARY() -> Py { ensure_init(); CRISIS_PLAN_ORDINARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CRISIS_PLAN_ORDINARY(v: Py) { ensure_init(); if let Ok(mut w) = CRISIS_PLAN_ORDINARY.write() { *w = v; } }
pub fn g_CRISIS_PLAN_REPLY() -> Py { ensure_init(); CRISIS_PLAN_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CRISIS_PLAN_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = CRISIS_PLAN_REPLY.write() { *w = v; } }
pub fn g_CRISIS_PLAN_SAID() -> Py { ensure_init(); CRISIS_PLAN_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CRISIS_PLAN_SAID(v: Py) { ensure_init(); if let Ok(mut w) = CRISIS_PLAN_SAID.write() { *w = v; } }
pub fn g_CRISIS_WORD() -> Py { ensure_init(); CRISIS_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CRISIS_WORD(v: Py) { ensure_init(); if let Ok(mut w) = CRISIS_WORD.write() { *w = v; } }
pub fn g_CUT_NOT_STOPPING() -> Py { ensure_init(); CUT_NOT_STOPPING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CUT_NOT_STOPPING(v: Py) { ensure_init(); if let Ok(mut w) = CUT_NOT_STOPPING.write() { *w = v; } }
pub fn g_DAY_TOTAL_SIGN() -> Py { ensure_init(); DAY_TOTAL_SIGN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_TOTAL_SIGN(v: Py) { ensure_init(); if let Ok(mut w) = DAY_TOTAL_SIGN.write() { *w = v; } }
pub fn g_DAY_TOTAL_WORD() -> Py { ensure_init(); DAY_TOTAL_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_TOTAL_WORD(v: Py) { ensure_init(); if let Ok(mut w) = DAY_TOTAL_WORD.write() { *w = v; } }
pub fn g_DENTAL_WORDS() -> Py { ensure_init(); DENTAL_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENTAL_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DENTAL_WORDS.write() { *w = v; } }
pub fn g_DETAIL_AFTER_ESCALATION() -> Py { ensure_init(); DETAIL_AFTER_ESCALATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETAIL_AFTER_ESCALATION(v: Py) { ensure_init(); if let Ok(mut w) = DETAIL_AFTER_ESCALATION.write() { *w = v; } }
pub fn g_DEVICE_BURNING() -> Py { ensure_init(); DEVICE_BURNING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEVICE_BURNING(v: Py) { ensure_init(); if let Ok(mut w) = DEVICE_BURNING.write() { *w = v; } }
pub fn g_DEVICE_OLD() -> Py { ensure_init(); DEVICE_OLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEVICE_OLD(v: Py) { ensure_init(); if let Ok(mut w) = DEVICE_OLD.write() { *w = v; } }
pub fn g_DIE_OF_THE_FEELING() -> Py { ensure_init(); DIE_OF_THE_FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DIE_OF_THE_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = DIE_OF_THE_FEELING.write() { *w = v; } }
pub fn g_DISTRESS_BESIDE_IT() -> Py { ensure_init(); DISTRESS_BESIDE_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DISTRESS_BESIDE_IT(v: Py) { ensure_init(); if let Ok(mut w) = DISTRESS_BESIDE_IT.write() { *w = v; } }
pub fn g_DOG_HABIT() -> Py { ensure_init(); DOG_HABIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOG_HABIT(v: Py) { ensure_init(); if let Ok(mut w) = DOG_HABIT.write() { *w = v; } }
pub fn g_DOG_MOUTHING() -> Py { ensure_init(); DOG_MOUTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOG_MOUTHING(v: Py) { ensure_init(); if let Ok(mut w) = DOG_MOUTHING.write() { *w = v; } }
pub fn g_DOG_SELF_BITE() -> Py { ensure_init(); DOG_SELF_BITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOG_SELF_BITE(v: Py) { ensure_init(); if let Ok(mut w) = DOG_SELF_BITE.write() { *w = v; } }
pub fn g_DOSE_AMOUNT() -> Py { ensure_init(); DOSE_AMOUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOSE_AMOUNT(v: Py) { ensure_init(); if let Ok(mut w) = DOSE_AMOUNT.write() { *w = v; } }
pub fn g_DOSE_ASKED() -> Py { ensure_init(); DOSE_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOSE_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = DOSE_ASKED.write() { *w = v; } }
pub fn g_DOSE_COURSE() -> Py { ensure_init(); DOSE_COURSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOSE_COURSE(v: Py) { ensure_init(); if let Ok(mut w) = DOSE_COURSE.write() { *w = v; } }
pub fn g_DOSE_COURSE_KEEP() -> Py { ensure_init(); DOSE_COURSE_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOSE_COURSE_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = DOSE_COURSE_KEEP.write() { *w = v; } }
pub fn g_DOSE_WORDS() -> Py { ensure_init(); DOSE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOSE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DOSE_WORDS.write() { *w = v; } }
pub fn g_DRINK_WORDS() -> Py { ensure_init(); DRINK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DRINK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DRINK_WORDS.write() { *w = v; } }
pub fn g_DROWNED_THING() -> Py { ensure_init(); DROWNED_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DROWNED_THING(v: Py) { ensure_init(); if let Ok(mut w) = DROWNED_THING.write() { *w = v; } }
pub fn g_DROWNING_HYPERBOLE() -> Py { ensure_init(); DROWNING_HYPERBOLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DROWNING_HYPERBOLE(v: Py) { ensure_init(); if let Ok(mut w) = DROWNING_HYPERBOLE.write() { *w = v; } }
pub fn g_DRUG_NAME() -> Py { ensure_init(); DRUG_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DRUG_NAME(v: Py) { ensure_init(); if let Ok(mut w) = DRUG_NAME.write() { *w = v; } }
pub fn g_EDGE_OF_A_DROP() -> Py { ensure_init(); EDGE_OF_A_DROP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EDGE_OF_A_DROP(v: Py) { ensure_init(); if let Ok(mut w) = EDGE_OF_A_DROP.write() { *w = v; } }
pub fn g_EFFORT_AFTER() -> Py { ensure_init(); EFFORT_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_AFTER.write() { *w = v; } }
pub fn g_EFFORT_ANYWHERE() -> Py { ensure_init(); EFFORT_ANYWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_ANYWHERE(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_ANYWHERE.write() { *w = v; } }
pub fn g_EFFORT_BENIGN() -> Py { ensure_init(); EFFORT_BENIGN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_BENIGN(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_BENIGN.write() { *w = v; } }
pub fn g_EFFORT_HEART_CLUSTER() -> Py { ensure_init(); EFFORT_HEART_CLUSTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_HEART_CLUSTER(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_HEART_CLUSTER.write() { *w = v; } }
pub fn g_EFFORT_PATTERN() -> Py { ensure_init(); EFFORT_PATTERN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_PATTERN(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_PATTERN.write() { *w = v; } }
pub fn g_EFFORT_RED_HEART() -> Py { ensure_init(); EFFORT_RED_HEART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_RED_HEART(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_RED_HEART.write() { *w = v; } }
pub fn g_EFFORT_TOO_LONG() -> Py { ensure_init(); EFFORT_TOO_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_TOO_LONG(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_TOO_LONG.write() { *w = v; } }
pub fn g_ELECTRIC_BURN() -> Py { ensure_init(); ELECTRIC_BURN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELECTRIC_BURN(v: Py) { ensure_init(); if let Ok(mut w) = ELECTRIC_BURN.write() { *w = v; } }
pub fn g_ELECTRIC_FINE() -> Py { ensure_init(); ELECTRIC_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELECTRIC_FINE(v: Py) { ensure_init(); if let Ok(mut w) = ELECTRIC_FINE.write() { *w = v; } }
pub fn g_ELECTRIC_SHOCK_DEFINITION() -> Py { ensure_init(); ELECTRIC_SHOCK_DEFINITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELECTRIC_SHOCK_DEFINITION(v: Py) { ensure_init(); if let Ok(mut w) = ELECTRIC_SHOCK_DEFINITION.write() { *w = v; } }
pub fn g_ELECTRIC_SHOCK_DOWN() -> Py { ensure_init(); ELECTRIC_SHOCK_DOWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELECTRIC_SHOCK_DOWN(v: Py) { ensure_init(); if let Ok(mut w) = ELECTRIC_SHOCK_DOWN.write() { *w = v; } }
pub fn g_END_IT_MEANS() -> Py { ensure_init(); END_IT_MEANS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_END_IT_MEANS(v: Py) { ensure_init(); if let Ok(mut w) = END_IT_MEANS.write() { *w = v; } }
pub fn g_END_IT_WITH_A_THING() -> Py { ensure_init(); END_IT_WITH_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_END_IT_WITH_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = END_IT_WITH_A_THING.write() { *w = v; } }
pub fn g_END_IT_WITH_NOUN() -> Py { ensure_init(); END_IT_WITH_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_END_IT_WITH_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = END_IT_WITH_NOUN.write() { *w = v; } }
pub fn g_END_MEANS_OTHER() -> Py { ensure_init(); END_MEANS_OTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_END_MEANS_OTHER(v: Py) { ensure_init(); if let Ok(mut w) = END_MEANS_OTHER.write() { *w = v; } }
pub fn g_END_THING() -> Py { ensure_init(); END_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_END_THING(v: Py) { ensure_init(); if let Ok(mut w) = END_THING.write() { *w = v; } }
pub fn g_EXERTION() -> Py { ensure_init(); EXERTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXERTION(v: Py) { ensure_init(); if let Ok(mut w) = EXERTION.write() { *w = v; } }
pub fn g_EXERTION_CLASS() -> Py { ensure_init(); EXERTION_CLASS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXERTION_CLASS(v: Py) { ensure_init(); if let Ok(mut w) = EXERTION_CLASS.write() { *w = v; } }
pub fn g_EXERTION_SMALL() -> Py { ensure_init(); EXERTION_SMALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXERTION_SMALL(v: Py) { ensure_init(); if let Ok(mut w) = EXERTION_SMALL.write() { *w = v; } }
pub fn g_EYE_CHEM() -> Py { ensure_init(); EYE_CHEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EYE_CHEM(v: Py) { ensure_init(); if let Ok(mut w) = EYE_CHEM.write() { *w = v; } }
pub fn g_EYE_FINE() -> Py { ensure_init(); EYE_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EYE_FINE(v: Py) { ensure_init(); if let Ok(mut w) = EYE_FINE.write() { *w = v; } }
pub fn g_FAINT_STORY() -> Py { ensure_init(); FAINT_STORY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FAINT_STORY(v: Py) { ensure_init(); if let Ok(mut w) = FAINT_STORY.write() { *w = v; } }
pub fn g_FALL_ANSWERED() -> Py { ensure_init(); FALL_ANSWERED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_ANSWERED(v: Py) { ensure_init(); if let Ok(mut w) = FALL_ANSWERED.write() { *w = v; } }
pub fn g_FALL_CHECK_OFF() -> Py { ensure_init(); FALL_CHECK_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_CHECK_OFF(v: Py) { ensure_init(); if let Ok(mut w) = FALL_CHECK_OFF.write() { *w = v; } }
pub fn g_FALL_CORE() -> Py { ensure_init(); FALL_CORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_CORE(v: Py) { ensure_init(); if let Ok(mut w) = FALL_CORE.write() { *w = v; } }
pub fn g_FALL_FINE() -> Py { ensure_init(); FALL_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_FINE(v: Py) { ensure_init(); if let Ok(mut w) = FALL_FINE.write() { *w = v; } }
pub fn g_FALL_HIP() -> Py { ensure_init(); FALL_HIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_HIP(v: Py) { ensure_init(); if let Ok(mut w) = FALL_HIP.write() { *w = v; } }
pub fn g_FALL_NOW() -> Py { ensure_init(); FALL_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_NOW(v: Py) { ensure_init(); if let Ok(mut w) = FALL_NOW.write() { *w = v; } }
pub fn g_FALL_PAST() -> Py { ensure_init(); FALL_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_PAST(v: Py) { ensure_init(); if let Ok(mut w) = FALL_PAST.write() { *w = v; } }
pub fn g_FALL_RED() -> Py { ensure_init(); FALL_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_RED(v: Py) { ensure_init(); if let Ok(mut w) = FALL_RED.write() { *w = v; } }
pub fn g_FALL_REPEAT() -> Py { ensure_init(); FALL_REPEAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_REPEAT(v: Py) { ensure_init(); if let Ok(mut w) = FALL_REPEAT.write() { *w = v; } }
pub fn g_FALL_WORD_IDIOM() -> Py { ensure_init(); FALL_WORD_IDIOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALL_WORD_IDIOM(v: Py) { ensure_init(); if let Ok(mut w) = FALL_WORD_IDIOM.write() { *w = v; } }
pub fn g_FEMALE_WORDS() -> Py { ensure_init(); FEMALE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEMALE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FEMALE_WORDS.write() { *w = v; } }
pub fn g_FEVER_ASK() -> Py { ensure_init(); FEVER_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEVER_ASK(v: Py) { ensure_init(); if let Ok(mut w) = FEVER_ASK.write() { *w = v; } }
pub fn g_FEVER_RASH_FINE() -> Py { ensure_init(); FEVER_RASH_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEVER_RASH_FINE(v: Py) { ensure_init(); if let Ok(mut w) = FEVER_RASH_FINE.write() { *w = v; } }
pub fn g_FEVER_REPORT() -> Py { ensure_init(); FEVER_REPORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEVER_REPORT(v: Py) { ensure_init(); if let Ok(mut w) = FEVER_REPORT.write() { *w = v; } }
pub fn g_FICTION_NOW() -> Py { ensure_init(); FICTION_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FICTION_NOW(v: Py) { ensure_init(); if let Ok(mut w) = FICTION_NOW.write() { *w = v; } }
pub fn g_FIRE() -> Py { ensure_init(); FIRE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRE(v: Py) { ensure_init(); if let Ok(mut w) = FIRE.write() { *w = v; } }
pub fn g_FIRE_DRILL() -> Py { ensure_init(); FIRE_DRILL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRE_DRILL(v: Py) { ensure_init(); if let Ok(mut w) = FIRE_DRILL.write() { *w = v; } }
pub fn g_FIRE_ELSEWHERE() -> Py { ensure_init(); FIRE_ELSEWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRE_ELSEWHERE(v: Py) { ensure_init(); if let Ok(mut w) = FIRE_ELSEWHERE.write() { *w = v; } }
pub fn g_FIRE_NOW() -> Py { ensure_init(); FIRE_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRE_NOW(v: Py) { ensure_init(); if let Ok(mut w) = FIRE_NOW.write() { *w = v; } }
pub fn g_FIRE_OUT() -> Py { ensure_init(); FIRE_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRE_OUT(v: Py) { ensure_init(); if let Ok(mut w) = FIRE_OUT.write() { *w = v; } }
pub fn g_FIRE_SCALE() -> Py { ensure_init(); FIRE_SCALE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRE_SCALE(v: Py) { ensure_init(); if let Ok(mut w) = FIRE_SCALE.write() { *w = v; } }
pub fn g_FITTING_IN() -> Py { ensure_init(); FITTING_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FITTING_IN(v: Py) { ensure_init(); if let Ok(mut w) = FITTING_IN.write() { *w = v; } }
pub fn g_FITTING_MANNER_TAIL() -> Py { ensure_init(); FITTING_MANNER_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FITTING_MANNER_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = FITTING_MANNER_TAIL.write() { *w = v; } }
pub fn g_FIT_ALL_BETTER() -> Py { ensure_init(); FIT_ALL_BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIT_ALL_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = FIT_ALL_BETTER.write() { *w = v; } }
pub fn g_FIT_CASES() -> Py { ensure_init(); FIT_CASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIT_CASES(v: Py) { ensure_init(); if let Ok(mut w) = FIT_CASES.write() { *w = v; } }
pub fn g_FIT_ROUTINE() -> Py { ensure_init(); FIT_ROUTINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIT_ROUTINE(v: Py) { ensure_init(); if let Ok(mut w) = FIT_ROUTINE.write() { *w = v; } }
pub fn g_FIT_STILL_RINGS() -> Py { ensure_init(); FIT_STILL_RINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIT_STILL_RINGS(v: Py) { ensure_init(); if let Ok(mut w) = FIT_STILL_RINGS.write() { *w = v; } }
pub fn g_FIT_THIS_WEEK() -> Py { ensure_init(); FIT_THIS_WEEK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIT_THIS_WEEK(v: Py) { ensure_init(); if let Ok(mut w) = FIT_THIS_WEEK.write() { *w = v; } }
pub fn g_FLAMES() -> Py { ensure_init(); FLAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLAMES(v: Py) { ensure_init(); if let Ok(mut w) = FLAMES.write() { *w = v; } }
pub fn g_FLOPPY_RED_FLAG() -> Py { ensure_init(); FLOPPY_RED_FLAG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLOPPY_RED_FLAG(v: Py) { ensure_init(); if let Ok(mut w) = FLOPPY_RED_FLAG.write() { *w = v; } }
pub fn g_FLOPPY_SLEEPY() -> Py { ensure_init(); FLOPPY_SLEEPY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLOPPY_SLEEPY(v: Py) { ensure_init(); if let Ok(mut w) = FLOPPY_SLEEPY.write() { *w = v; } }
pub fn g_FLOPPY_THING() -> Py { ensure_init(); FLOPPY_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLOPPY_THING(v: Py) { ensure_init(); if let Ok(mut w) = FLOPPY_THING.write() { *w = v; } }
pub fn g_FOLLOW_UP_ANCHOR() -> Py { ensure_init(); FOLLOW_UP_ANCHOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_ANCHOR(v: Py) { ensure_init(); if let Ok(mut w) = FOLLOW_UP_ANCHOR.write() { *w = v; } }
pub fn g_FOLLOW_UP_MORE() -> Py { ensure_init(); FOLLOW_UP_MORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_MORE(v: Py) { ensure_init(); if let Ok(mut w) = FOLLOW_UP_MORE.write() { *w = v; } }
pub fn g_FOOD_CUT() -> Py { ensure_init(); FOOD_CUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOOD_CUT(v: Py) { ensure_init(); if let Ok(mut w) = FOOD_CUT.write() { *w = v; } }
pub fn g_FORAGED_EATEN() -> Py { ensure_init(); FORAGED_EATEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FORAGED_EATEN(v: Py) { ensure_init(); if let Ok(mut w) = FORAGED_EATEN.write() { *w = v; } }
pub fn g_FORAGED_FINE() -> Py { ensure_init(); FORAGED_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FORAGED_FINE(v: Py) { ensure_init(); if let Ok(mut w) = FORAGED_FINE.write() { *w = v; } }
pub fn g_FRICTION_WORD() -> Py { ensure_init(); FRICTION_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRICTION_WORD(v: Py) { ensure_init(); if let Ok(mut w) = FRICTION_WORD.write() { *w = v; } }
pub fn g_FRIGHTENED() -> Py { ensure_init(); FRIGHTENED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRIGHTENED(v: Py) { ensure_init(); if let Ok(mut w) = FRIGHTENED.write() { *w = v; } }
pub fn g_FRIGHTENED_REPLY() -> Py { ensure_init(); FRIGHTENED_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRIGHTENED_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = FRIGHTENED_REPLY.write() { *w = v; } }
pub fn g_GARMENT() -> Py { ensure_init(); GARMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GARMENT(v: Py) { ensure_init(); if let Ok(mut w) = GARMENT.write() { *w = v; } }
pub fn g_GASPING_THIRST() -> Py { ensure_init(); GASPING_THIRST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GASPING_THIRST(v: Py) { ensure_init(); if let Ok(mut w) = GASPING_THIRST.write() { *w = v; } }
pub fn g_GASP_FINE() -> Py { ensure_init(); GASP_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GASP_FINE(v: Py) { ensure_init(); if let Ok(mut w) = GASP_FINE.write() { *w = v; } }
pub fn g_GASP_RED() -> Py { ensure_init(); GASP_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GASP_RED(v: Py) { ensure_init(); if let Ok(mut w) = GASP_RED.write() { *w = v; } }
pub fn g_GAS_LEFT_ON() -> Py { ensure_init(); GAS_LEFT_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAS_LEFT_ON(v: Py) { ensure_init(); if let Ok(mut w) = GAS_LEFT_ON.write() { *w = v; } }
pub fn g_GAS_ON_NOT_RUNNING() -> Py { ensure_init(); GAS_ON_NOT_RUNNING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAS_ON_NOT_RUNNING(v: Py) { ensure_init(); if let Ok(mut w) = GAS_ON_NOT_RUNNING.write() { *w = v; } }
pub fn g_GAS_ON_YOU() -> Py { ensure_init(); GAS_ON_YOU.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAS_ON_YOU(v: Py) { ensure_init(); if let Ok(mut w) = GAS_ON_YOU.write() { *w = v; } }
pub fn g_GAS_PAST() -> Py { ensure_init(); GAS_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAS_PAST(v: Py) { ensure_init(); if let Ok(mut w) = GAS_PAST.write() { *w = v; } }
pub fn g_GAS_SMELL() -> Py { ensure_init(); GAS_SMELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAS_SMELL(v: Py) { ensure_init(); if let Ok(mut w) = GAS_SMELL.write() { *w = v; } }
pub fn g_GIVING_UP_GROUP() -> Py { ensure_init(); GIVING_UP_GROUP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVING_UP_GROUP(v: Py) { ensure_init(); if let Ok(mut w) = GIVING_UP_GROUP.write() { *w = v; } }
pub fn g_GIVING_UP_HELD_ON() -> Py { ensure_init(); GIVING_UP_HELD_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVING_UP_HELD_ON(v: Py) { ensure_init(); if let Ok(mut w) = GIVING_UP_HELD_ON.write() { *w = v; } }
pub fn g_GO_IN_NOT_ABOUT_IT() -> Py { ensure_init(); GO_IN_NOT_ABOUT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GO_IN_NOT_ABOUT_IT(v: Py) { ensure_init(); if let Ok(mut w) = GO_IN_NOT_ABOUT_IT.write() { *w = v; } }
pub fn g_GO_IS_A_CALL() -> Py { ensure_init(); GO_IS_A_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GO_IS_A_CALL(v: Py) { ensure_init(); if let Ok(mut w) = GO_IS_A_CALL.write() { *w = v; } }
pub fn g_GRUNTING_FINE() -> Py { ensure_init(); GRUNTING_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRUNTING_FINE(v: Py) { ensure_init(); if let Ok(mut w) = GRUNTING_FINE.write() { *w = v; } }
pub fn g_HAIR_SCORCH() -> Py { ensure_init(); HAIR_SCORCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAIR_SCORCH(v: Py) { ensure_init(); if let Ok(mut w) = HAIR_SCORCH.write() { *w = v; } }
pub fn g_HAIR_SCORCH_ANSWER() -> Py { ensure_init(); HAIR_SCORCH_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAIR_SCORCH_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = HAIR_SCORCH_ANSWER.write() { *w = v; } }
pub fn g_HANGED_LONG_AGO() -> Py { ensure_init(); HANGED_LONG_AGO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANGED_LONG_AGO(v: Py) { ensure_init(); if let Ok(mut w) = HANGED_LONG_AGO.write() { *w = v; } }
pub fn g_HANGING() -> Py { ensure_init(); HANGING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANGING(v: Py) { ensure_init(); if let Ok(mut w) = HANGING.write() { *w = v; } }
pub fn g_HARD_TO_WAKE() -> Py { ensure_init(); HARD_TO_WAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HARD_TO_WAKE(v: Py) { ensure_init(); if let Ok(mut w) = HARD_TO_WAKE.write() { *w = v; } }
pub fn g_HEADACHE_BAD() -> Py { ensure_init(); HEADACHE_BAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEADACHE_BAD(v: Py) { ensure_init(); if let Ok(mut w) = HEADACHE_BAD.write() { *w = v; } }
pub fn g_HEADACHE_STORY() -> Py { ensure_init(); HEADACHE_STORY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEADACHE_STORY(v: Py) { ensure_init(); if let Ok(mut w) = HEADACHE_STORY.write() { *w = v; } }
pub fn g_HEADACHE_SUDDEN() -> Py { ensure_init(); HEADACHE_SUDDEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEADACHE_SUDDEN(v: Py) { ensure_init(); if let Ok(mut w) = HEADACHE_SUDDEN.write() { *w = v; } }
pub fn g_HEAD_BANG() -> Py { ensure_init(); HEAD_BANG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAD_BANG(v: Py) { ensure_init(); if let Ok(mut w) = HEAD_BANG.write() { *w = v; } }
pub fn g_HEAD_KNOCKOUT() -> Py { ensure_init(); HEAD_KNOCKOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAD_KNOCKOUT(v: Py) { ensure_init(); if let Ok(mut w) = HEAD_KNOCKOUT.write() { *w = v; } }
pub fn g_HEAD_RED() -> Py { ensure_init(); HEAD_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAD_RED(v: Py) { ensure_init(); if let Ok(mut w) = HEAD_RED.write() { *w = v; } }
pub fn g_HEAL_WORDS() -> Py { ensure_init(); HEAL_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAL_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HEAL_WORDS.write() { *w = v; } }
pub fn g_HEARD_ASSAULT() -> Py { ensure_init(); HEARD_ASSAULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEARD_ASSAULT(v: Py) { ensure_init(); if let Ok(mut w) = HEARD_ASSAULT.write() { *w = v; } }
pub fn g_HEAT() -> Py { ensure_init(); HEAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAT(v: Py) { ensure_init(); if let Ok(mut w) = HEAT.write() { *w = v; } }
pub fn g_HELD_CRISIS_DETAIL() -> Py { ensure_init(); HELD_CRISIS_DETAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELD_CRISIS_DETAIL(v: Py) { ensure_init(); if let Ok(mut w) = HELD_CRISIS_DETAIL.write() { *w = v; } }
pub fn g_HELD_CRISIS_OFF() -> Py { ensure_init(); HELD_CRISIS_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELD_CRISIS_OFF(v: Py) { ensure_init(); if let Ok(mut w) = HELD_CRISIS_OFF.write() { *w = v; } }
pub fn g_HELP_ARRIVED() -> Py { ensure_init(); HELP_ARRIVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ARRIVED(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ARRIVED.write() { *w = v; } }
pub fn g_HELP_SOMEONE() -> Py { ensure_init(); HELP_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = HELP_SOMEONE.write() { *w = v; } }
pub fn g_HIT_IN_FUN() -> Py { ensure_init(); HIT_IN_FUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIT_IN_FUN(v: Py) { ensure_init(); if let Ok(mut w) = HIT_IN_FUN.write() { *w = v; } }
pub fn g_HIT_THE_LAUGH_IS_HIS() -> Py { ensure_init(); HIT_THE_LAUGH_IS_HIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIT_THE_LAUGH_IS_HIS(v: Py) { ensure_init(); if let Ok(mut w) = HIT_THE_LAUGH_IS_HIS.write() { *w = v; } }
pub fn g_HOW_LONG() -> Py { ensure_init(); HOW_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_LONG(v: Py) { ensure_init(); if let Ok(mut w) = HOW_LONG.write() { *w = v; } }
pub fn g_HOW_TO_TREAT() -> Py { ensure_init(); HOW_TO_TREAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_TREAT(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO_TREAT.write() { *w = v; } }
pub fn g_HUMAN_TOO() -> Py { ensure_init(); HUMAN_TOO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUMAN_TOO(v: Py) { ensure_init(); if let Ok(mut w) = HUMAN_TOO.write() { *w = v; } }
pub fn g_HYPERBOLE_BLOCK() -> Py { ensure_init(); HYPERBOLE_BLOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HYPERBOLE_BLOCK(v: Py) { ensure_init(); if let Ok(mut w) = HYPERBOLE_BLOCK.write() { *w = v; } }
pub fn g_HYPO_HOW_KNOW() -> Py { ensure_init(); HYPO_HOW_KNOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HYPO_HOW_KNOW(v: Py) { ensure_init(); if let Ok(mut w) = HYPO_HOW_KNOW.write() { *w = v; } }
pub fn g_HYPO_QUESTION() -> Py { ensure_init(); HYPO_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HYPO_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = HYPO_QUESTION.write() { *w = v; } }
pub fn g_HYPO_STRONG() -> Py { ensure_init(); HYPO_STRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HYPO_STRONG(v: Py) { ensure_init(); if let Ok(mut w) = HYPO_STRONG.write() { *w = v; } }
pub fn g_HYPO_STRONG_OTHER() -> Py { ensure_init(); HYPO_STRONG_OTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HYPO_STRONG_OTHER(v: Py) { ensure_init(); if let Ok(mut w) = HYPO_STRONG_OTHER.write() { *w = v; } }
pub fn g_INJURY_WORDS() -> Py { ensure_init(); INJURY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INJURY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = INJURY_WORDS.write() { *w = v; } }
pub fn g_INTRUDER() -> Py { ensure_init(); INTRUDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INTRUDER(v: Py) { ensure_init(); if let Ok(mut w) = INTRUDER.write() { *w = v; } }
pub fn g_KICKED_PERSON() -> Py { ensure_init(); KICKED_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KICKED_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = KICKED_PERSON.write() { *w = v; } }
pub fn g_LIMP_LONG() -> Py { ensure_init(); LIMP_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMP_LONG(v: Py) { ensure_init(); if let Ok(mut w) = LIMP_LONG.write() { *w = v; } }
pub fn g_LIMP_MECHANISM() -> Py { ensure_init(); LIMP_MECHANISM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMP_MECHANISM(v: Py) { ensure_init(); if let Ok(mut w) = LIMP_MECHANISM.write() { *w = v; } }
pub fn g_LIMP_OTHER_LIMB() -> Py { ensure_init(); LIMP_OTHER_LIMB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMP_OTHER_LIMB(v: Py) { ensure_init(); if let Ok(mut w) = LIMP_OTHER_LIMB.write() { *w = v; } }
pub fn g_LIMP_SOMEBODY_ELSE() -> Py { ensure_init(); LIMP_SOMEBODY_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMP_SOMEBODY_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = LIMP_SOMEBODY_ELSE.write() { *w = v; } }
pub fn g_LIPS_COLD() -> Py { ensure_init(); LIPS_COLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIPS_COLD(v: Py) { ensure_init(); if let Ok(mut w) = LIPS_COLD.write() { *w = v; } }
pub fn g_LIPS_SWELL() -> Py { ensure_init(); LIPS_SWELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIPS_SWELL(v: Py) { ensure_init(); if let Ok(mut w) = LIPS_SWELL.write() { *w = v; } }
pub fn g_LIPS_SWELL_BARE_OFF() -> Py { ensure_init(); LIPS_SWELL_BARE_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIPS_SWELL_BARE_OFF(v: Py) { ensure_init(); if let Ok(mut w) = LIPS_SWELL_BARE_OFF.write() { *w = v; } }
pub fn g_LIPS_TINGLE() -> Py { ensure_init(); LIPS_TINGLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIPS_TINGLE(v: Py) { ensure_init(); if let Ok(mut w) = LIPS_TINGLE.write() { *w = v; } }
pub fn g_LIPS_TINGLE_FOOD() -> Py { ensure_init(); LIPS_TINGLE_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIPS_TINGLE_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = LIPS_TINGLE_FOOD.write() { *w = v; } }
pub fn g_LIPS_TINGLE_OFF() -> Py { ensure_init(); LIPS_TINGLE_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIPS_TINGLE_OFF(v: Py) { ensure_init(); if let Ok(mut w) = LIPS_TINGLE_OFF.write() { *w = v; } }
pub fn g_LIPS_TINGLE_RED() -> Py { ensure_init(); LIPS_TINGLE_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIPS_TINGLE_RED(v: Py) { ensure_init(); if let Ok(mut w) = LIPS_TINGLE_RED.write() { *w = v; } }
pub fn g_LOCKED_IN_RISK() -> Py { ensure_init(); LOCKED_IN_RISK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCKED_IN_RISK(v: Py) { ensure_init(); if let Ok(mut w) = LOCKED_IN_RISK.write() { *w = v; } }
pub fn g_LOOKS_REAL() -> Py { ensure_init(); LOOKS_REAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOKS_REAL(v: Py) { ensure_init(); if let Ok(mut w) = LOOKS_REAL.write() { *w = v; } }
pub fn g_LOSS_SAID() -> Py { ensure_init(); LOSS_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_SAID(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_SAID.write() { *w = v; } }
pub fn g_LOSS_SUBJECT() -> Py { ensure_init(); LOSS_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_SUBJECT.write() { *w = v; } }
pub fn g_MACHINE() -> Py { ensure_init(); MACHINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MACHINE(v: Py) { ensure_init(); if let Ok(mut w) = MACHINE.write() { *w = v; } }
pub fn g_MEAL_BREATH() -> Py { ensure_init(); MEAL_BREATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEAL_BREATH(v: Py) { ensure_init(); if let Ok(mut w) = MEAL_BREATH.write() { *w = v; } }
pub fn g_MEDICINE() -> Py { ensure_init(); MEDICINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEDICINE(v: Py) { ensure_init(); if let Ok(mut w) = MEDICINE.write() { *w = v; } }
pub fn g_MENINGITIS_OTHER() -> Py { ensure_init(); MENINGITIS_OTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MENINGITIS_OTHER(v: Py) { ensure_init(); if let Ok(mut w) = MENINGITIS_OTHER.write() { *w = v; } }
pub fn g_MILD_BODY_SIGN() -> Py { ensure_init(); MILD_BODY_SIGN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MILD_BODY_SIGN(v: Py) { ensure_init(); if let Ok(mut w) = MILD_BODY_SIGN.write() { *w = v; } }
pub fn g_MILD_CHOKE() -> Py { ensure_init(); MILD_CHOKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MILD_CHOKE(v: Py) { ensure_init(); if let Ok(mut w) = MILD_CHOKE.write() { *w = v; } }
pub fn g_MILD_PRODUCT_SMALL() -> Py { ensure_init(); MILD_PRODUCT_SMALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MILD_PRODUCT_SMALL(v: Py) { ensure_init(); if let Ok(mut w) = MILD_PRODUCT_SMALL.write() { *w = v; } }
pub fn g_MILD_SWALLOWED() -> Py { ensure_init(); MILD_SWALLOWED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MILD_SWALLOWED(v: Py) { ensure_init(); if let Ok(mut w) = MILD_SWALLOWED.write() { *w = v; } }
pub fn g_MORNING_ROUTINE() -> Py { ensure_init(); MORNING_ROUTINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MORNING_ROUTINE(v: Py) { ensure_init(); if let Ok(mut w) = MORNING_ROUTINE.write() { *w = v; } }
pub fn g_MOUTH_BLEED_NOW() -> Py { ensure_init(); MOUTH_BLEED_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOUTH_BLEED_NOW(v: Py) { ensure_init(); if let Ok(mut w) = MOUTH_BLEED_NOW.write() { *w = v; } }
pub fn g_NAMED_SICK_FINE() -> Py { ensure_init(); NAMED_SICK_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_SICK_FINE(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_SICK_FINE.write() { *w = v; } }
pub fn g_NEAR_FIRE() -> Py { ensure_init(); NEAR_FIRE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEAR_FIRE(v: Py) { ensure_init(); if let Ok(mut w) = NEAR_FIRE.write() { *w = v; } }
pub fn g_NEAR_FIRE_OVER() -> Py { ensure_init(); NEAR_FIRE_OVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEAR_FIRE_OVER(v: Py) { ensure_init(); if let Ok(mut w) = NEAR_FIRE_OVER.write() { *w = v; } }
pub fn g_NEAR_THING_OVER() -> Py { ensure_init(); NEAR_THING_OVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEAR_THING_OVER(v: Py) { ensure_init(); if let Ok(mut w) = NEAR_THING_OVER.write() { *w = v; } }
pub fn g_NEWBORN() -> Py { ensure_init(); NEWBORN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEWBORN(v: Py) { ensure_init(); if let Ok(mut w) = NEWBORN.write() { *w = v; } }
pub fn g_NIGHT_BREATH_OTHER() -> Py { ensure_init(); NIGHT_BREATH_OTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NIGHT_BREATH_OTHER(v: Py) { ensure_init(); if let Ok(mut w) = NIGHT_BREATH_OTHER.write() { *w = v; } }
pub fn g_NIGHT_BREATH_PAST() -> Py { ensure_init(); NIGHT_BREATH_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NIGHT_BREATH_PAST(v: Py) { ensure_init(); if let Ok(mut w) = NIGHT_BREATH_PAST.write() { *w = v; } }
pub fn g_NIGHT_BREATH_RED() -> Py { ensure_init(); NIGHT_BREATH_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NIGHT_BREATH_RED(v: Py) { ensure_init(); if let Ok(mut w) = NIGHT_BREATH_RED.write() { *w = v; } }
pub fn g_NOSE_RED() -> Py { ensure_init(); NOSE_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOSE_RED(v: Py) { ensure_init(); if let Ok(mut w) = NOSE_RED.write() { *w = v; } }
pub fn g_NOT_A_WOUND_OFF() -> Py { ensure_init(); NOT_A_WOUND_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_WOUND_OFF(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_WOUND_OFF.write() { *w = v; } }
pub fn g_NOT_NOW() -> Py { ensure_init(); NOT_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_NOW(v: Py) { ensure_init(); if let Ok(mut w) = NOT_NOW.write() { *w = v; } }
pub fn g_NOT_NOW_FICTION() -> Py { ensure_init(); NOT_NOW_FICTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_NOW_FICTION(v: Py) { ensure_init(); if let Ok(mut w) = NOT_NOW_FICTION.write() { *w = v; } }
pub fn g_NOT_STOPPING() -> Py { ensure_init(); NOT_STOPPING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_STOPPING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_STOPPING.write() { *w = v; } }
pub fn g_OBJECT_IN_HEAD_THING() -> Py { ensure_init(); OBJECT_IN_HEAD_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OBJECT_IN_HEAD_THING(v: Py) { ensure_init(); if let Ok(mut w) = OBJECT_IN_HEAD_THING.write() { *w = v; } }
pub fn g_OLDER_CHILD_NAMED() -> Py { ensure_init(); OLDER_CHILD_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OLDER_CHILD_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = OLDER_CHILD_NAMED.write() { *w = v; } }
pub fn g_OLD_FALL_FINE() -> Py { ensure_init(); OLD_FALL_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OLD_FALL_FINE(v: Py) { ensure_init(); if let Ok(mut w) = OLD_FALL_FINE.write() { *w = v; } }
pub fn g_OLD_FALL_NOT() -> Py { ensure_init(); OLD_FALL_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OLD_FALL_NOT(v: Py) { ensure_init(); if let Ok(mut w) = OLD_FALL_NOT.write() { *w = v; } }
pub fn g_OLD_PERSON() -> Py { ensure_init(); OLD_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OLD_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = OLD_PERSON.write() { *w = v; } }
pub fn g_OLD_TIME() -> Py { ensure_init(); OLD_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OLD_TIME(v: Py) { ensure_init(); if let Ok(mut w) = OLD_TIME.write() { *w = v; } }
pub fn g_OTHER_RISK() -> Py { ensure_init(); OTHER_RISK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_RISK(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_RISK.write() { *w = v; } }
pub fn g_OTHER_SELF_HARM() -> Py { ensure_init(); OTHER_SELF_HARM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_SELF_HARM(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_SELF_HARM.write() { *w = v; } }
pub fn g_OUT_COLD_ASLEEP() -> Py { ensure_init(); OUT_COLD_ASLEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OUT_COLD_ASLEEP(v: Py) { ensure_init(); if let Ok(mut w) = OUT_COLD_ASLEEP.write() { *w = v; } }
pub fn g_OUT_COLD_PERSON() -> Py { ensure_init(); OUT_COLD_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OUT_COLD_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = OUT_COLD_PERSON.write() { *w = v; } }
pub fn g_OUT_COLD_REPLY() -> Py { ensure_init(); OUT_COLD_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OUT_COLD_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = OUT_COLD_REPLY.write() { *w = v; } }
pub fn g_OVERDOSE_DRUG() -> Py { ensure_init(); OVERDOSE_DRUG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OVERDOSE_DRUG(v: Py) { ensure_init(); if let Ok(mut w) = OVERDOSE_DRUG.write() { *w = v; } }
pub fn g_OVERDOSE_FOOD() -> Py { ensure_init(); OVERDOSE_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OVERDOSE_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = OVERDOSE_FOOD.write() { *w = v; } }
pub fn g_OVERDOSE_OF_A_TREAT() -> Py { ensure_init(); OVERDOSE_OF_A_TREAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OVERDOSE_OF_A_TREAT(v: Py) { ensure_init(); if let Ok(mut w) = OVERDOSE_OF_A_TREAT.write() { *w = v; } }
pub fn g_OWN_CRY() -> Py { ensure_init(); OWN_CRY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_CRY(v: Py) { ensure_init(); if let Ok(mut w) = OWN_CRY.write() { *w = v; } }
pub fn g_OWN_INJURY() -> Py { ensure_init(); OWN_INJURY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_INJURY(v: Py) { ensure_init(); if let Ok(mut w) = OWN_INJURY.write() { *w = v; } }
pub fn g_OWN_INJURY_IN_MY_CARE() -> Py { ensure_init(); OWN_INJURY_IN_MY_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_INJURY_IN_MY_CARE(v: Py) { ensure_init(); if let Ok(mut w) = OWN_INJURY_IN_MY_CARE.write() { *w = v; } }
pub fn g_OWN_INJURY_QUESTION() -> Py { ensure_init(); OWN_INJURY_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_INJURY_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = OWN_INJURY_QUESTION.write() { *w = v; } }
pub fn g_OWN_LIMB_AT_AN_ANGLE() -> Py { ensure_init(); OWN_LIMB_AT_AN_ANGLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_LIMB_AT_AN_ANGLE(v: Py) { ensure_init(); if let Ok(mut w) = OWN_LIMB_AT_AN_ANGLE.write() { *w = v; } }
pub fn g_PALE_EFFORT_RED() -> Py { ensure_init(); PALE_EFFORT_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PALE_EFFORT_RED(v: Py) { ensure_init(); if let Ok(mut w) = PALE_EFFORT_RED.write() { *w = v; } }
pub fn g_PALE_EXERTION() -> Py { ensure_init(); PALE_EXERTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PALE_EXERTION(v: Py) { ensure_init(); if let Ok(mut w) = PALE_EXERTION.write() { *w = v; } }
pub fn g_PALE_HEAT() -> Py { ensure_init(); PALE_HEAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PALE_HEAT(v: Py) { ensure_init(); if let Ok(mut w) = PALE_HEAT.write() { *w = v; } }
pub fn g_PALE_SECOND() -> Py { ensure_init(); PALE_SECOND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PALE_SECOND(v: Py) { ensure_init(); if let Ok(mut w) = PALE_SECOND.write() { *w = v; } }
pub fn g_PAN_FIRE() -> Py { ensure_init(); PAN_FIRE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAN_FIRE(v: Py) { ensure_init(); if let Ok(mut w) = PAN_FIRE.write() { *w = v; } }
pub fn g_PARENT_REFUSAL_BLOW() -> Py { ensure_init(); PARENT_REFUSAL_BLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PARENT_REFUSAL_BLOW(v: Py) { ensure_init(); if let Ok(mut w) = PARENT_REFUSAL_BLOW.write() { *w = v; } }
pub fn g_PAST_ATTEMPT_NOT() -> Py { ensure_init(); PAST_ATTEMPT_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_ATTEMPT_NOT(v: Py) { ensure_init(); if let Ok(mut w) = PAST_ATTEMPT_NOT.write() { *w = v; } }
pub fn g_PAST_WISH() -> Py { ensure_init(); PAST_WISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_WISH(v: Py) { ensure_init(); if let Ok(mut w) = PAST_WISH.write() { *w = v; } }
pub fn g_PAST_WISH_NOT() -> Py { ensure_init(); PAST_WISH_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_WISH_NOT(v: Py) { ensure_init(); if let Ok(mut w) = PAST_WISH_NOT.write() { *w = v; } }
pub fn g_PERSON() -> Py { ensure_init(); PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = PERSON.write() { *w = v; } }
pub fn g_PERSON_AS_ATTACKER() -> Py { ensure_init(); PERSON_AS_ATTACKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_AS_ATTACKER(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_AS_ATTACKER.write() { *w = v; } }
pub fn g_PERSON_BURNT() -> Py { ensure_init(); PERSON_BURNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_BURNT(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_BURNT.write() { *w = v; } }
pub fn g_PET_ALL_RIGHT() -> Py { ensure_init(); PET_ALL_RIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_ALL_RIGHT(v: Py) { ensure_init(); if let Ok(mut w) = PET_ALL_RIGHT.write() { *w = v; } }
pub fn g_PET_ANY() -> Py { ensure_init(); PET_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_ANY(v: Py) { ensure_init(); if let Ok(mut w) = PET_ANY.write() { *w = v; } }
pub fn g_PET_GASP_EXPLAINED() -> Py { ensure_init(); PET_GASP_EXPLAINED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_GASP_EXPLAINED(v: Py) { ensure_init(); if let Ok(mut w) = PET_GASP_EXPLAINED.write() { *w = v; } }
pub fn g_PET_GASP_FINE() -> Py { ensure_init(); PET_GASP_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_GASP_FINE(v: Py) { ensure_init(); if let Ok(mut w) = PET_GASP_FINE.write() { *w = v; } }
pub fn g_PET_GASP_RED() -> Py { ensure_init(); PET_GASP_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_GASP_RED(v: Py) { ensure_init(); if let Ok(mut w) = PET_GASP_RED.write() { *w = v; } }
pub fn g_PET_NAMES() -> Py { ensure_init(); PET_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = PET_NAMES.write() { *w = v; } }
pub fn g_PET_PAST() -> Py { ensure_init(); PET_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_PAST(v: Py) { ensure_init(); if let Ok(mut w) = PET_PAST.write() { *w = v; } }
pub fn g_PET_RESPOND_AT() -> Py { ensure_init(); PET_RESPOND_AT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_RESPOND_AT(v: Py) { ensure_init(); if let Ok(mut w) = PET_RESPOND_AT.write() { *w = v; } }
pub fn g_PET_TOOTH() -> Py { ensure_init(); PET_TOOTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_TOOTH(v: Py) { ensure_init(); if let Ok(mut w) = PET_TOOTH.write() { *w = v; } }
pub fn g_PLAYFUL_HIT() -> Py { ensure_init(); PLAYFUL_HIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAYFUL_HIT(v: Py) { ensure_init(); if let Ok(mut w) = PLAYFUL_HIT.write() { *w = v; } }
pub fn g_POISON_SAID() -> Py { ensure_init(); POISON_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POISON_SAID(v: Py) { ensure_init(); if let Ok(mut w) = POISON_SAID.write() { *w = v; } }
pub fn g_POO_BLACK() -> Py { ensure_init(); POO_BLACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POO_BLACK(v: Py) { ensure_init(); if let Ok(mut w) = POO_BLACK.write() { *w = v; } }
pub fn g_POO_BLOOD() -> Py { ensure_init(); POO_BLOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POO_BLOOD(v: Py) { ensure_init(); if let Ok(mut w) = POO_BLOOD.write() { *w = v; } }
pub fn g_POO_FINE() -> Py { ensure_init(); POO_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POO_FINE(v: Py) { ensure_init(); if let Ok(mut w) = POO_FINE.write() { *w = v; } }
pub fn g_PREGNANCY_WORDS() -> Py { ensure_init(); PREGNANCY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREGNANCY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = PREGNANCY_WORDS.write() { *w = v; } }
pub fn g_PUNCTURE() -> Py { ensure_init(); PUNCTURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUNCTURE(v: Py) { ensure_init(); if let Ok(mut w) = PUNCTURE.write() { *w = v; } }
pub fn g_PUSHED_AWAY() -> Py { ensure_init(); PUSHED_AWAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUSHED_AWAY(v: Py) { ensure_init(); if let Ok(mut w) = PUSHED_AWAY.write() { *w = v; } }
pub fn g_PUSHED_AWAY_BUT_SOMETHING_ELSE() -> Py { ensure_init(); PUSHED_AWAY_BUT_SOMETHING_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUSHED_AWAY_BUT_SOMETHING_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = PUSHED_AWAY_BUT_SOMETHING_ELSE.write() { *w = v; } }
pub fn g_PUSHED_AWAY_REPLY() -> Py { ensure_init(); PUSHED_AWAY_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUSHED_AWAY_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = PUSHED_AWAY_REPLY.write() { *w = v; } }
pub fn g_PUSHED_AWAY_THEN_RESTRAINT() -> Py { ensure_init(); PUSHED_AWAY_THEN_RESTRAINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUSHED_AWAY_THEN_RESTRAINT(v: Py) { ensure_init(); if let Ok(mut w) = PUSHED_AWAY_THEN_RESTRAINT.write() { *w = v; } }
pub fn g_RASH_FADE() -> Py { ensure_init(); RASH_FADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RASH_FADE(v: Py) { ensure_init(); if let Ok(mut w) = RASH_FADE.write() { *w = v; } }
pub fn g_RASH_HEADACHE_FINE() -> Py { ensure_init(); RASH_HEADACHE_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RASH_HEADACHE_FINE(v: Py) { ensure_init(); if let Ok(mut w) = RASH_HEADACHE_FINE.write() { *w = v; } }
pub fn g_RASH_PRESSED() -> Py { ensure_init(); RASH_PRESSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RASH_PRESSED(v: Py) { ensure_init(); if let Ok(mut w) = RASH_PRESSED.write() { *w = v; } }
pub fn g_RAZOR_BURN() -> Py { ensure_init(); RAZOR_BURN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RAZOR_BURN(v: Py) { ensure_init(); if let Ok(mut w) = RAZOR_BURN.write() { *w = v; } }
pub fn g_RECENT_FIT_FINE() -> Py { ensure_init(); RECENT_FIT_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECENT_FIT_FINE(v: Py) { ensure_init(); if let Ok(mut w) = RECENT_FIT_FINE.write() { *w = v; } }
pub fn g_RECENT_TIME() -> Py { ensure_init(); RECENT_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECENT_TIME(v: Py) { ensure_init(); if let Ok(mut w) = RECENT_TIME.write() { *w = v; } }
pub fn g_RESPOND_SENSE() -> Py { ensure_init(); RESPOND_SENSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RESPOND_SENSE(v: Py) { ensure_init(); if let Ok(mut w) = RESPOND_SENSE.write() { *w = v; } }
pub fn g_RESTRAINT_BESIDE() -> Py { ensure_init(); RESTRAINT_BESIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RESTRAINT_BESIDE(v: Py) { ensure_init(); if let Ok(mut w) = RESTRAINT_BESIDE.write() { *w = v; } }
pub fn g_RING_DANGER() -> Py { ensure_init(); RING_DANGER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RING_DANGER(v: Py) { ensure_init(); if let Ok(mut w) = RING_DANGER.write() { *w = v; } }
pub fn g_RING_NOT_WORN() -> Py { ensure_init(); RING_NOT_WORN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RING_NOT_WORN(v: Py) { ensure_init(); if let Ok(mut w) = RING_NOT_WORN.write() { *w = v; } }
pub fn g_ROUTINE_REASON() -> Py { ensure_init(); ROUTINE_REASON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_REASON(v: Py) { ensure_init(); if let Ok(mut w) = ROUTINE_REASON.write() { *w = v; } }
pub fn g_ROUTINE_WAKE() -> Py { ensure_init(); ROUTINE_WAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_WAKE(v: Py) { ensure_init(); if let Ok(mut w) = ROUTINE_WAKE.write() { *w = v; } }
pub fn g_SAFE_NOW() -> Py { ensure_init(); SAFE_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAFE_NOW(v: Py) { ensure_init(); if let Ok(mut w) = SAFE_NOW.write() { *w = v; } }
pub fn g_SAID_AS_IT_HAPPENS() -> Py { ensure_init(); SAID_AS_IT_HAPPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_IT_HAPPENS(v: Py) { ensure_init(); if let Ok(mut w) = SAID_AS_IT_HAPPENS.write() { *w = v; } }
pub fn g_SEIZURE() -> Py { ensure_init(); SEIZURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEIZURE(v: Py) { ensure_init(); if let Ok(mut w) = SEIZURE.write() { *w = v; } }
pub fn g_SEIZURE_FIGURATIVE() -> Py { ensure_init(); SEIZURE_FIGURATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEIZURE_FIGURATIVE(v: Py) { ensure_init(); if let Ok(mut w) = SEIZURE_FIGURATIVE.write() { *w = v; } }
pub fn g_SEIZURE_STORY_GUARD() -> Py { ensure_init(); SEIZURE_STORY_GUARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEIZURE_STORY_GUARD(v: Py) { ensure_init(); if let Ok(mut w) = SEIZURE_STORY_GUARD.write() { *w = v; } }
pub fn g_SELF_CUT() -> Py { ensure_init(); SELF_CUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_CUT(v: Py) { ensure_init(); if let Ok(mut w) = SELF_CUT.write() { *w = v; } }
pub fn g_SELF_HARM_AGAIN() -> Py { ensure_init(); SELF_HARM_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_AGAIN.write() { *w = v; } }
pub fn g_SELF_HARM_BY_A_MEANS() -> Py { ensure_init(); SELF_HARM_BY_A_MEANS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_BY_A_MEANS(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_BY_A_MEANS.write() { *w = v; } }
pub fn g_SELF_HARM_CUT_NO_OBJECT() -> Py { ensure_init(); SELF_HARM_CUT_NO_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_CUT_NO_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_CUT_NO_OBJECT.write() { *w = v; } }
pub fn g_SELF_HARM_DELIBERATE_ACT() -> Py { ensure_init(); SELF_HARM_DELIBERATE_ACT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_DELIBERATE_ACT(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_DELIBERATE_ACT.write() { *w = v; } }
pub fn g_SELF_HARM_DESERVE() -> Py { ensure_init(); SELF_HARM_DESERVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_DESERVE(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_DESERVE.write() { *w = v; } }
pub fn g_SELF_HARM_LOOKALIKE() -> Py { ensure_init(); SELF_HARM_LOOKALIKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_LOOKALIKE(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_LOOKALIKE.write() { *w = v; } }
pub fn g_SELF_HARM_NOT_EATING() -> Py { ensure_init(); SELF_HARM_NOT_EATING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_NOT_EATING(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_NOT_EATING.write() { *w = v; } }
pub fn g_SELF_HARM_OWN_PART() -> Py { ensure_init(); SELF_HARM_OWN_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_OWN_PART(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_OWN_PART.write() { *w = v; } }
pub fn g_SELF_HARM_REASON() -> Py { ensure_init(); SELF_HARM_REASON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_REASON(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_REASON.write() { *w = v; } }
pub fn g_SELF_HARM_SCARS() -> Py { ensure_init(); SELF_HARM_SCARS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_SCARS(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_SCARS.write() { *w = v; } }
pub fn g_SELF_HARM_SCARS_NOT() -> Py { ensure_init(); SELF_HARM_SCARS_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_SCARS_NOT(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_SCARS_NOT.write() { *w = v; } }
pub fn g_SELF_HARM_TOPIC_OFF() -> Py { ensure_init(); SELF_HARM_TOPIC_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_TOPIC_OFF(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_TOPIC_OFF.write() { *w = v; } }
pub fn g_SELF_HARM_WISHES_A_PART() -> Py { ensure_init(); SELF_HARM_WISHES_A_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_WISHES_A_PART(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_WISHES_A_PART.write() { *w = v; } }
pub fn g_SELF_RUMINATION() -> Py { ensure_init(); SELF_RUMINATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_RUMINATION(v: Py) { ensure_init(); if let Ok(mut w) = SELF_RUMINATION.write() { *w = v; } }
pub fn g_SELF_TALK() -> Py { ensure_init(); SELF_TALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_TALK(v: Py) { ensure_init(); if let Ok(mut w) = SELF_TALK.write() { *w = v; } }
pub fn g_SEPSIS_SIGN() -> Py { ensure_init(); SEPSIS_SIGN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEPSIS_SIGN(v: Py) { ensure_init(); if let Ok(mut w) = SEPSIS_SIGN.write() { *w = v; } }
pub fn g_SHAKEN() -> Py { ensure_init(); SHAKEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHAKEN(v: Py) { ensure_init(); if let Ok(mut w) = SHAKEN.write() { *w = v; } }
pub fn g_SHAKEN_REPLY() -> Py { ensure_init(); SHAKEN_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHAKEN_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = SHAKEN_REPLY.write() { *w = v; } }
pub fn g_SHOCK_ANY() -> Py { ensure_init(); SHOCK_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOCK_ANY(v: Py) { ensure_init(); if let Ok(mut w) = SHOCK_ANY.write() { *w = v; } }
pub fn g_SHORT_OF_BREATH() -> Py { ensure_init(); SHORT_OF_BREATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_OF_BREATH(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_OF_BREATH.write() { *w = v; } }
pub fn g_SHOT() -> Py { ensure_init(); SHOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOT(v: Py) { ensure_init(); if let Ok(mut w) = SHOT.write() { *w = v; } }
pub fn g_SHOT_FINE() -> Py { ensure_init(); SHOT_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOT_FINE(v: Py) { ensure_init(); if let Ok(mut w) = SHOT_FINE.write() { *w = v; } }
pub fn g_SIGHT_A_WHILE() -> Py { ensure_init(); SIGHT_A_WHILE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIGHT_A_WHILE(v: Py) { ensure_init(); if let Ok(mut w) = SIGHT_A_WHILE.write() { *w = v; } }
pub fn g_SLUR_RED_BESIDE() -> Py { ensure_init(); SLUR_RED_BESIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLUR_RED_BESIDE(v: Py) { ensure_init(); if let Ok(mut w) = SLUR_RED_BESIDE.write() { *w = v; } }
pub fn g_SLUR_THICK() -> Py { ensure_init(); SLUR_THICK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLUR_THICK(v: Py) { ensure_init(); if let Ok(mut w) = SLUR_THICK.write() { *w = v; } }
pub fn g_SMALL_CHILD() -> Py { ensure_init(); SMALL_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SMALL_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = SMALL_CHILD.write() { *w = v; } }
pub fn g_SMALL_CHILD_LASH_REPLIES() -> Py { ensure_init(); SMALL_CHILD_LASH_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SMALL_CHILD_LASH_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = SMALL_CHILD_LASH_REPLIES.write() { *w = v; } }
pub fn g_SMOKE_IS_A_SMELL() -> Py { ensure_init(); SMOKE_IS_A_SMELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SMOKE_IS_A_SMELL(v: Py) { ensure_init(); if let Ok(mut w) = SMOKE_IS_A_SMELL.write() { *w = v; } }
pub fn g_SNAKE_LOOKALIKE() -> Py { ensure_init(); SNAKE_LOOKALIKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SNAKE_LOOKALIKE(v: Py) { ensure_init(); if let Ok(mut w) = SNAKE_LOOKALIKE.write() { *w = v; } }
pub fn g_SOCKET_OBJECT() -> Py { ensure_init(); SOCKET_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOCKET_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = SOCKET_OBJECT.write() { *w = v; } }
pub fn g_SOCKET_THING_FINE() -> Py { ensure_init(); SOCKET_THING_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOCKET_THING_FINE(v: Py) { ensure_init(); if let Ok(mut w) = SOCKET_THING_FINE.write() { *w = v; } }
pub fn g_SOFT_BLEED_OFF() -> Py { ensure_init(); SOFT_BLEED_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT_BLEED_OFF(v: Py) { ensure_init(); if let Ok(mut w) = SOFT_BLEED_OFF.write() { *w = v; } }
pub fn g_SOMEONE_ELSES_DESPAIR() -> Py { ensure_init(); SOMEONE_ELSES_DESPAIR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_DESPAIR(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSES_DESPAIR.write() { *w = v; } }
pub fn g_SOMEONE_ELSE_OWN() -> Py { ensure_init(); SOMEONE_ELSE_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE_OWN(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSE_OWN.write() { *w = v; } }
pub fn g_SOME_OF_IT() -> Py { ensure_init(); SOME_OF_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOME_OF_IT(v: Py) { ensure_init(); if let Ok(mut w) = SOME_OF_IT.write() { *w = v; } }
pub fn g_SPELL_RED() -> Py { ensure_init(); SPELL_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPELL_RED(v: Py) { ensure_init(); if let Ok(mut w) = SPELL_RED.write() { *w = v; } }
pub fn g_SPIKED() -> Py { ensure_init(); SPIKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPIKED(v: Py) { ensure_init(); if let Ok(mut w) = SPIKED.write() { *w = v; } }
pub fn g_SPINE_BACK_WORK() -> Py { ensure_init(); SPINE_BACK_WORK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPINE_BACK_WORK(v: Py) { ensure_init(); if let Ok(mut w) = SPINE_BACK_WORK.write() { *w = v; } }
pub fn g_SPINE_BREAK() -> Py { ensure_init(); SPINE_BREAK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPINE_BREAK(v: Py) { ensure_init(); if let Ok(mut w) = SPINE_BREAK.write() { *w = v; } }
pub fn g_SPOKEN_ATTEMPT_UNDATED() -> Py { ensure_init(); SPOKEN_ATTEMPT_UNDATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPOKEN_ATTEMPT_UNDATED(v: Py) { ensure_init(); if let Ok(mut w) = SPOKEN_ATTEMPT_UNDATED.write() { *w = v; } }
pub fn g_STIFF_FINE() -> Py { ensure_init(); STIFF_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STIFF_FINE(v: Py) { ensure_init(); if let Ok(mut w) = STIFF_FINE.write() { *w = v; } }
pub fn g_STIFF_NECK() -> Py { ensure_init(); STIFF_NECK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STIFF_NECK(v: Py) { ensure_init(); if let Ok(mut w) = STIFF_NECK.write() { *w = v; } }
pub fn g_STILL_GOING() -> Py { ensure_init(); STILL_GOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_GOING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_GOING.write() { *w = v; } }
pub fn g_STING_SETTLED() -> Py { ensure_init(); STING_SETTLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STING_SETTLED(v: Py) { ensure_init(); if let Ok(mut w) = STING_SETTLED.write() { *w = v; } }
pub fn g_STOOL_LONG() -> Py { ensure_init(); STOOL_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOOL_LONG(v: Py) { ensure_init(); if let Ok(mut w) = STOOL_LONG.write() { *w = v; } }
pub fn g_STRANGLED_SELF() -> Py { ensure_init(); STRANGLED_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRANGLED_SELF(v: Py) { ensure_init(); if let Ok(mut w) = STRANGLED_SELF.write() { *w = v; } }
pub fn g_STRANGLE_THING_SUBJECT() -> Py { ensure_init(); STRANGLE_THING_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRANGLE_THING_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = STRANGLE_THING_SUBJECT.write() { *w = v; } }
pub fn g_STREAK_FEVER() -> Py { ensure_init(); STREAK_FEVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STREAK_FEVER(v: Py) { ensure_init(); if let Ok(mut w) = STREAK_FEVER.write() { *w = v; } }
pub fn g_STROKE_HAND() -> Py { ensure_init(); STROKE_HAND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STROKE_HAND(v: Py) { ensure_init(); if let Ok(mut w) = STROKE_HAND.write() { *w = v; } }
pub fn g_STROKE_HAND_BESIDE() -> Py { ensure_init(); STROKE_HAND_BESIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STROKE_HAND_BESIDE(v: Py) { ensure_init(); if let Ok(mut w) = STROKE_HAND_BESIDE.write() { *w = v; } }
pub fn g_STROKE_HAND_CHRONIC() -> Py { ensure_init(); STROKE_HAND_CHRONIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STROKE_HAND_CHRONIC(v: Py) { ensure_init(); if let Ok(mut w) = STROKE_HAND_CHRONIC.write() { *w = v; } }
pub fn g_STROKE_HAND_HARD() -> Py { ensure_init(); STROKE_HAND_HARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STROKE_HAND_HARD(v: Py) { ensure_init(); if let Ok(mut w) = STROKE_HAND_HARD.write() { *w = v; } }
pub fn g_STROKE_WEAK() -> Py { ensure_init(); STROKE_WEAK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STROKE_WEAK(v: Py) { ensure_init(); if let Ok(mut w) = STROKE_WEAK.write() { *w = v; } }
pub fn g_STROKE_WEAK_OFF() -> Py { ensure_init(); STROKE_WEAK_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STROKE_WEAK_OFF(v: Py) { ensure_init(); if let Ok(mut w) = STROKE_WEAK_OFF.write() { *w = v; } }
pub fn g_STUCK_MACHINE() -> Py { ensure_init(); STUCK_MACHINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STUCK_MACHINE(v: Py) { ensure_init(); if let Ok(mut w) = STUCK_MACHINE.write() { *w = v; } }
pub fn g_SUICIDE_ABOUT_A_THING() -> Py { ensure_init(); SUICIDE_ABOUT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUICIDE_ABOUT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = SUICIDE_ABOUT_A_THING.write() { *w = v; } }
pub fn g_SUICIDE_TOPIC() -> Py { ensure_init(); SUICIDE_TOPIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUICIDE_TOPIC(v: Py) { ensure_init(); if let Ok(mut w) = SUICIDE_TOPIC.write() { *w = v; } }
pub fn g_SWALLOWED_BAD_REPLY() -> Py { ensure_init(); SWALLOWED_BAD_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWALLOWED_BAD_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = SWALLOWED_BAD_REPLY.write() { *w = v; } }
pub fn g_SWALLOWED_BAD_SELF() -> Py { ensure_init(); SWALLOWED_BAD_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWALLOWED_BAD_SELF(v: Py) { ensure_init(); if let Ok(mut w) = SWALLOWED_BAD_SELF.write() { *w = v; } }
pub fn g_SWALLOWED_ORDINARY() -> Py { ensure_init(); SWALLOWED_ORDINARY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWALLOWED_ORDINARY(v: Py) { ensure_init(); if let Ok(mut w) = SWALLOWED_ORDINARY.write() { *w = v; } }
pub fn g_SWALLOWED_SELF() -> Py { ensure_init(); SWALLOWED_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWALLOWED_SELF(v: Py) { ensure_init(); if let Ok(mut w) = SWALLOWED_SELF.write() { *w = v; } }
pub fn g_SWALLOW_GONE_OFF() -> Py { ensure_init(); SWALLOW_GONE_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWALLOW_GONE_OFF(v: Py) { ensure_init(); if let Ok(mut w) = SWALLOW_GONE_OFF.write() { *w = v; } }
pub fn g_SWALLOW_GONE_RED() -> Py { ensure_init(); SWALLOW_GONE_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWALLOW_GONE_RED(v: Py) { ensure_init(); if let Ok(mut w) = SWALLOW_GONE_RED.write() { *w = v; } }
pub fn g_TALK_HABIT() -> Py { ensure_init(); TALK_HABIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_HABIT(v: Py) { ensure_init(); if let Ok(mut w) = TALK_HABIT.write() { *w = v; } }
pub fn g_TESTIS_LUMP() -> Py { ensure_init(); TESTIS_LUMP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TESTIS_LUMP(v: Py) { ensure_init(); if let Ok(mut w) = TESTIS_LUMP.write() { *w = v; } }
pub fn g_THING_BURNT() -> Py { ensure_init(); THING_BURNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_BURNT(v: Py) { ensure_init(); if let Ok(mut w) = THING_BURNT.write() { *w = v; } }
pub fn g_THING_BURNT_WORDS() -> Py { ensure_init(); THING_BURNT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_BURNT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = THING_BURNT_WORDS.write() { *w = v; } }
pub fn g_THING_THAT_IS_SUICIDE() -> Py { ensure_init(); THING_THAT_IS_SUICIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_THAT_IS_SUICIDE(v: Py) { ensure_init(); if let Ok(mut w) = THING_THAT_IS_SUICIDE.write() { *w = v; } }
pub fn g_THIRD_PERSON_ACTOR() -> Py { ensure_init(); THIRD_PERSON_ACTOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THIRD_PERSON_ACTOR(v: Py) { ensure_init(); if let Ok(mut w) = THIRD_PERSON_ACTOR.write() { *w = v; } }
pub fn g_THROAT_TIGHT_CAUSE() -> Py { ensure_init(); THROAT_TIGHT_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THROAT_TIGHT_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = THROAT_TIGHT_CAUSE.write() { *w = v; } }
pub fn g_THROAT_TIGHT_EFFORT() -> Py { ensure_init(); THROAT_TIGHT_EFFORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THROAT_TIGHT_EFFORT(v: Py) { ensure_init(); if let Ok(mut w) = THROAT_TIGHT_EFFORT.write() { *w = v; } }
pub fn g_THROAT_TIGHT_RED() -> Py { ensure_init(); THROAT_TIGHT_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THROAT_TIGHT_RED(v: Py) { ensure_init(); if let Ok(mut w) = THROAT_TIGHT_RED.write() { *w = v; } }
pub fn g_TINY_AGE() -> Py { ensure_init(); TINY_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TINY_AGE(v: Py) { ensure_init(); if let Ok(mut w) = TINY_AGE.write() { *w = v; } }
pub fn g_TINY_AGE_FINE() -> Py { ensure_init(); TINY_AGE_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TINY_AGE_FINE(v: Py) { ensure_init(); if let Ok(mut w) = TINY_AGE_FINE.write() { *w = v; } }
pub fn g_TOOK_OUT_OF() -> Py { ensure_init(); TOOK_OUT_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOOK_OUT_OF(v: Py) { ensure_init(); if let Ok(mut w) = TOOK_OUT_OF.write() { *w = v; } }
pub fn g_TOOTH_OUT_FINE() -> Py { ensure_init(); TOOTH_OUT_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOOTH_OUT_FINE(v: Py) { ensure_init(); if let Ok(mut w) = TOOTH_OUT_FINE.write() { *w = v; } }
pub fn g_TORSION() -> Py { ensure_init(); TORSION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TORSION(v: Py) { ensure_init(); if let Ok(mut w) = TORSION.write() { *w = v; } }
pub fn g_TORSION_FINE() -> Py { ensure_init(); TORSION_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TORSION_FINE(v: Py) { ensure_init(); if let Ok(mut w) = TORSION_FINE.write() { *w = v; } }
pub fn g_TORSION_ONSET() -> Py { ensure_init(); TORSION_ONSET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TORSION_ONSET(v: Py) { ensure_init(); if let Ok(mut w) = TORSION_ONSET.write() { *w = v; } }
pub fn g_TRAPPED_HELD_BY_A_PERSON() -> Py { ensure_init(); TRAPPED_HELD_BY_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRAPPED_HELD_BY_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = TRAPPED_HELD_BY_A_PERSON.write() { *w = v; } }
pub fn g_TRAPPED_VEHICLE_NOT_NOW() -> Py { ensure_init(); TRAPPED_VEHICLE_NOT_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRAPPED_VEHICLE_NOT_NOW(v: Py) { ensure_init(); if let Ok(mut w) = TRAPPED_VEHICLE_NOT_NOW.write() { *w = v; } }
pub fn g_TYPED_TYPOS() -> Py { ensure_init(); TYPED_TYPOS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TYPED_TYPOS(v: Py) { ensure_init(); if let Ok(mut w) = TYPED_TYPOS.write() { *w = v; } }
pub fn g_VEHICLE_BURNING_SMELL() -> Py { ensure_init(); VEHICLE_BURNING_SMELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VEHICLE_BURNING_SMELL(v: Py) { ensure_init(); if let Ok(mut w) = VEHICLE_BURNING_SMELL.write() { *w = v; } }
pub fn g_VEHICLE_BURNING_SMELL_HARD() -> Py { ensure_init(); VEHICLE_BURNING_SMELL_HARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VEHICLE_BURNING_SMELL_HARD(v: Py) { ensure_init(); if let Ok(mut w) = VEHICLE_BURNING_SMELL_HARD.write() { *w = v; } }
pub fn g_VEHICLE_EXHAUST() -> Py { ensure_init(); VEHICLE_EXHAUST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VEHICLE_EXHAUST(v: Py) { ensure_init(); if let Ok(mut w) = VEHICLE_EXHAUST.write() { *w = v; } }
pub fn g_VEHICLE_FIRE() -> Py { ensure_init(); VEHICLE_FIRE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VEHICLE_FIRE(v: Py) { ensure_init(); if let Ok(mut w) = VEHICLE_FIRE.write() { *w = v; } }
pub fn g_VEHICLE_FIRE_OVER() -> Py { ensure_init(); VEHICLE_FIRE_OVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VEHICLE_FIRE_OVER(v: Py) { ensure_init(); if let Ok(mut w) = VEHICLE_FIRE_OVER.write() { *w = v; } }
pub fn g_VIOLENT_THREAT() -> Py { ensure_init(); VIOLENT_THREAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VIOLENT_THREAT(v: Py) { ensure_init(); if let Ok(mut w) = VIOLENT_THREAT.write() { *w = v; } }
pub fn g_VISIT_OR_TIME() -> Py { ensure_init(); VISIT_OR_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VISIT_OR_TIME(v: Py) { ensure_init(); if let Ok(mut w) = VISIT_OR_TIME.write() { *w = v; } }
pub fn g_WAKE_RED() -> Py { ensure_init(); WAKE_RED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAKE_RED(v: Py) { ensure_init(); if let Ok(mut w) = WAKE_RED.write() { *w = v; } }
pub fn g_WAKE_UP_SOMEONE() -> Py { ensure_init(); WAKE_UP_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAKE_UP_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = WAKE_UP_SOMEONE.write() { *w = v; } }
pub fn g_WATER_DROWNING_FINE() -> Py { ensure_init(); WATER_DROWNING_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_DROWNING_FINE(v: Py) { ensure_init(); if let Ok(mut w) = WATER_DROWNING_FINE.write() { *w = v; } }
pub fn g_WATER_DROWNING_NOW() -> Py { ensure_init(); WATER_DROWNING_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_DROWNING_NOW(v: Py) { ensure_init(); if let Ok(mut w) = WATER_DROWNING_NOW.write() { *w = v; } }
pub fn g_WATER_FINE() -> Py { ensure_init(); WATER_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_FINE(v: Py) { ensure_init(); if let Ok(mut w) = WATER_FINE.write() { *w = v; } }
pub fn g_WATER_HEAD_FEAR() -> Py { ensure_init(); WATER_HEAD_FEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_HEAD_FEAR(v: Py) { ensure_init(); if let Ok(mut w) = WATER_HEAD_FEAR.write() { *w = v; } }
pub fn g_WATER_HUMAN() -> Py { ensure_init(); WATER_HUMAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_HUMAN(v: Py) { ensure_init(); if let Ok(mut w) = WATER_HUMAN.write() { *w = v; } }
pub fn g_WATER_IN_NOW() -> Py { ensure_init(); WATER_IN_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_IN_NOW(v: Py) { ensure_init(); if let Ok(mut w) = WATER_IN_NOW.write() { *w = v; } }
pub fn g_WATER_NOT_A_PERSON() -> Py { ensure_init(); WATER_NOT_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_NOT_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = WATER_NOT_A_PERSON.write() { *w = v; } }
pub fn g_WATER_PERSON_TOO() -> Py { ensure_init(); WATER_PERSON_TOO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_PERSON_TOO(v: Py) { ensure_init(); if let Ok(mut w) = WATER_PERSON_TOO.write() { *w = v; } }
pub fn g_WATER_SELF() -> Py { ensure_init(); WATER_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_SELF(v: Py) { ensure_init(); if let Ok(mut w) = WATER_SELF.write() { *w = v; } }
pub fn g_WATER_THING_DROPPED() -> Py { ensure_init(); WATER_THING_DROPPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_THING_DROPPED(v: Py) { ensure_init(); if let Ok(mut w) = WATER_THING_DROPPED.write() { *w = v; } }
pub fn g_WATER_URGENT() -> Py { ensure_init(); WATER_URGENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATER_URGENT(v: Py) { ensure_init(); if let Ok(mut w) = WATER_URGENT.write() { *w = v; } }
pub fn g_WEIGHT_HIP() -> Py { ensure_init(); WEIGHT_HIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_HIP(v: Py) { ensure_init(); if let Ok(mut w) = WEIGHT_HIP.write() { *w = v; } }
pub fn g_WEIGHT_LONG() -> Py { ensure_init(); WEIGHT_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_LONG(v: Py) { ensure_init(); if let Ok(mut w) = WEIGHT_LONG.write() { *w = v; } }
pub fn g_WEIGHT_OFF_FEET() -> Py { ensure_init(); WEIGHT_OFF_FEET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_OFF_FEET(v: Py) { ensure_init(); if let Ok(mut w) = WEIGHT_OFF_FEET.write() { *w = v; } }
pub fn g_WENT_BLUE_FINE() -> Py { ensure_init(); WENT_BLUE_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_BLUE_FINE(v: Py) { ensure_init(); if let Ok(mut w) = WENT_BLUE_FINE.write() { *w = v; } }
pub fn g_WENT_BLUE_IDIOM() -> Py { ensure_init(); WENT_BLUE_IDIOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_BLUE_IDIOM(v: Py) { ensure_init(); if let Ok(mut w) = WENT_BLUE_IDIOM.write() { *w = v; } }
pub fn g_WHEN_HELP_COMES() -> Py { ensure_init(); WHEN_HELP_COMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_HELP_COMES(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_HELP_COMES.write() { *w = v; } }
pub fn g_WHEN_HELP_REPLY() -> Py { ensure_init(); WHEN_HELP_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_HELP_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_HELP_REPLY.write() { *w = v; } }
pub fn g_WHOLE_BODY_RED_FLAG() -> Py { ensure_init(); WHOLE_BODY_RED_FLAG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHOLE_BODY_RED_FLAG(v: Py) { ensure_init(); if let Ok(mut w) = WHOLE_BODY_RED_FLAG.write() { *w = v; } }
pub fn g_WISH_BY_MODAL() -> Py { ensure_init(); WISH_BY_MODAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WISH_BY_MODAL(v: Py) { ensure_init(); if let Ok(mut w) = WISH_BY_MODAL.write() { *w = v; } }
pub fn g_WISH_DEAD_HYPERBOLE() -> Py { ensure_init(); WISH_DEAD_HYPERBOLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WISH_DEAD_HYPERBOLE(v: Py) { ensure_init(); if let Ok(mut w) = WISH_DEAD_HYPERBOLE.write() { *w = v; } }
pub fn g_WISH_DEAD_PLACE() -> Py { ensure_init(); WISH_DEAD_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WISH_DEAD_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = WISH_DEAD_PLACE.write() { *w = v; } }
pub fn g_WISH_DEAD_WHEN() -> Py { ensure_init(); WISH_DEAD_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WISH_DEAD_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = WISH_DEAD_WHEN.write() { *w = v; } }
pub fn g_WISH_NAMED() -> Py { ensure_init(); WISH_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WISH_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = WISH_NAMED.write() { *w = v; } }
pub fn g_WITHDRAWAL_ALONE_AT_HOME() -> Py { ensure_init(); WITHDRAWAL_ALONE_AT_HOME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_ALONE_AT_HOME(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_ALONE_AT_HOME.write() { *w = v; } }
pub fn g_WITHDRAWAL_ASK_DANGER() -> Py { ensure_init(); WITHDRAWAL_ASK_DANGER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_ASK_DANGER(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_ASK_DANGER.write() { *w = v; } }
pub fn g_WITHDRAWAL_DANGER() -> Py { ensure_init(); WITHDRAWAL_DANGER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_DANGER(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_DANGER.write() { *w = v; } }
pub fn g_WITHDRAWAL_HAIR_OF_THE_DOG() -> Py { ensure_init(); WITHDRAWAL_HAIR_OF_THE_DOG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_HAIR_OF_THE_DOG(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_HAIR_OF_THE_DOG.write() { *w = v; } }
pub fn g_WITHDRAWAL_HALLUCINATION() -> Py { ensure_init(); WITHDRAWAL_HALLUCINATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_HALLUCINATION(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_HALLUCINATION.write() { *w = v; } }
pub fn g_WITHDRAWAL_NAMED() -> Py { ensure_init(); WITHDRAWAL_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_NAMED.write() { *w = v; } }
pub fn g_WITHDRAWAL_NOT_ALCOHOL() -> Py { ensure_init(); WITHDRAWAL_NOT_ALCOHOL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_NOT_ALCOHOL(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_NOT_ALCOHOL.write() { *w = v; } }
pub fn g_WITHDRAWAL_ROUTINE() -> Py { ensure_init(); WITHDRAWAL_ROUTINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_ROUTINE(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_ROUTINE.write() { *w = v; } }
pub fn g_WITHDRAWAL_SENSES_FINE() -> Py { ensure_init(); WITHDRAWAL_SENSES_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_SENSES_FINE(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_SENSES_FINE.write() { *w = v; } }
pub fn g_WITHDRAWAL_STOP() -> Py { ensure_init(); WITHDRAWAL_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_STOP(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_STOP.write() { *w = v; } }
pub fn g_WITHDRAWAL_STRONG() -> Py { ensure_init(); WITHDRAWAL_STRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_STRONG(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_STRONG.write() { *w = v; } }
pub fn g_WITHDRAWAL_SYMPTOM() -> Py { ensure_init(); WITHDRAWAL_SYMPTOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHDRAWAL_SYMPTOM(v: Py) { ensure_init(); if let Ok(mut w) = WITHDRAWAL_SYMPTOM.write() { *w = v; } }
pub fn g_WONT_COME_ROUND() -> Py { ensure_init(); WONT_COME_ROUND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WONT_COME_ROUND(v: Py) { ensure_init(); if let Ok(mut w) = WONT_COME_ROUND.write() { *w = v; } }
pub fn g_WORDS_OUT_OCCASION() -> Py { ensure_init(); WORDS_OUT_OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORDS_OUT_OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = WORDS_OUT_OCCASION.write() { *w = v; } }
pub fn g_WORLD_QUESTION() -> Py { ensure_init(); WORLD_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORLD_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = WORLD_QUESTION.write() { *w = v; } }
pub fn g_WOUND_STREAK() -> Py { ensure_init(); WOUND_STREAK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WOUND_STREAK(v: Py) { ensure_init(); if let Ok(mut w) = WOUND_STREAK.write() { *w = v; } }
pub fn g_WOUND_STREAK_FEVER() -> Py { ensure_init(); WOUND_STREAK_FEVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WOUND_STREAK_FEVER(v: Py) { ensure_init(); if let Ok(mut w) = WOUND_STREAK_FEVER.write() { *w = v; } }
pub fn g_WOUND_STREAK_OFF() -> Py { ensure_init(); WOUND_STREAK_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WOUND_STREAK_OFF(v: Py) { ensure_init(); if let Ok(mut w) = WOUND_STREAK_OFF.write() { *w = v; } }
pub fn g__AGREEMENT() -> Py { ensure_init(); _AGREEMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AGREEMENT(v: Py) { ensure_init(); if let Ok(mut w) = _AGREEMENT.write() { *w = v; } }
pub fn g__ANY_MEANS_VERB() -> Py { ensure_init(); _ANY_MEANS_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ANY_MEANS_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _ANY_MEANS_VERB.write() { *w = v; } }
pub fn g__AWAY_TAIL() -> Py { ensure_init(); _AWAY_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AWAY_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _AWAY_TAIL.write() { *w = v; } }
pub fn g__A_MEANS_PLACE() -> Py { ensure_init(); _A_MEANS_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_MEANS_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = _A_MEANS_PLACE.write() { *w = v; } }
pub fn g__A_PURPOSE_AFTER() -> Py { ensure_init(); _A_PURPOSE_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PURPOSE_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = _A_PURPOSE_AFTER.write() { *w = v; } }
pub fn g__A_THIRST() -> Py { ensure_init(); _A_THIRST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_THIRST(v: Py) { ensure_init(); if let Ok(mut w) = _A_THIRST.write() { *w = v; } }
pub fn g__COPULA() -> Py { ensure_init(); _COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COPULA(v: Py) { ensure_init(); if let Ok(mut w) = _COPULA.write() { *w = v; } }
pub fn g__DENIED() -> Py { ensure_init(); _DENIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DENIED(v: Py) { ensure_init(); if let Ok(mut w) = _DENIED.write() { *w = v; } }
pub fn g__FITTING_A_THING() -> Py { ensure_init(); _FITTING_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FITTING_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = _FITTING_A_THING.write() { *w = v; } }
pub fn g__FLAT_WISH_HOW() -> Py { ensure_init(); _FLAT_WISH_HOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FLAT_WISH_HOW(v: Py) { ensure_init(); if let Ok(mut w) = _FLAT_WISH_HOW.write() { *w = v; } }
pub fn g__GASP_NOT_A_THIRST() -> Py { ensure_init(); _GASP_NOT_A_THIRST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__GASP_NOT_A_THIRST(v: Py) { ensure_init(); if let Ok(mut w) = _GASP_NOT_A_THIRST.write() { *w = v; } }
pub fn g__HEIGHT_MEANS_VERB() -> Py { ensure_init(); _HEIGHT_MEANS_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HEIGHT_MEANS_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _HEIGHT_MEANS_VERB.write() { *w = v; } }
pub fn g__INTO_THE_WAY() -> Py { ensure_init(); _INTO_THE_WAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__INTO_THE_WAY(v: Py) { ensure_init(); if let Ok(mut w) = _INTO_THE_WAY.write() { *w = v; } }
pub fn g__OFF_AN_EDGE() -> Py { ensure_init(); _OFF_AN_EDGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OFF_AN_EDGE(v: Py) { ensure_init(); if let Ok(mut w) = _OFF_AN_EDGE.write() { *w = v; } }
pub fn g__OFF_A_HEIGHT() -> Py { ensure_init(); _OFF_A_HEIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OFF_A_HEIGHT(v: Py) { ensure_init(); if let Ok(mut w) = _OFF_A_HEIGHT.write() { *w = v; } }
pub fn g__PERSON() -> Py { ensure_init(); _PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PERSON(v: Py) { ensure_init(); if let Ok(mut w) = _PERSON.write() { *w = v; } }
pub fn g__SMELLY_THING() -> Py { ensure_init(); _SMELLY_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SMELLY_THING(v: Py) { ensure_init(); if let Ok(mut w) = _SMELLY_THING.write() { *w = v; } }
pub fn g__SMOKE_THING() -> Py { ensure_init(); _SMOKE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SMOKE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _SMOKE_THING.write() { *w = v; } }
pub fn g__STREAK_MARK() -> Py { ensure_init(); _STREAK_MARK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STREAK_MARK(v: Py) { ensure_init(); if let Ok(mut w) = _STREAK_MARK.write() { *w = v; } }
pub fn g__STREAK_MOVE() -> Py { ensure_init(); _STREAK_MOVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STREAK_MOVE(v: Py) { ensure_init(); if let Ok(mut w) = _STREAK_MOVE.write() { *w = v; } }
pub fn g__STREAK_ON_A_PERSON() -> Py { ensure_init(); _STREAK_ON_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STREAK_ON_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = _STREAK_ON_A_PERSON.write() { *w = v; } }
pub fn g__STREAK_REDNESS() -> Py { ensure_init(); _STREAK_REDNESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STREAK_REDNESS(v: Py) { ensure_init(); if let Ok(mut w) = _STREAK_REDNESS.write() { *w = v; } }
pub fn g__STRUCK_RELATIVE() -> Py { ensure_init(); _STRUCK_RELATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STRUCK_RELATIVE(v: Py) { ensure_init(); if let Ok(mut w) = _STRUCK_RELATIVE.write() { *w = v; } }
pub fn g__THIRD() -> Py { ensure_init(); _THIRD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THIRD(v: Py) { ensure_init(); if let Ok(mut w) = _THIRD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The tables and helpers `bot/modules/safety.py` no longer holds inline.\n\nMoved here whole, name by name, with the comment above each one and nothing changed: the\nfile had grown past the size the Hub will take. `safety_tables` imports every name back, so\n`safety_tables.PLAN_ASKS` is the object it was and nothing that reads it can tell.\n");
    s_FEMALE_WORDS(py_tuple(vec![py_str("\\bm(?:u|o)m\\b"), py_str("\\bmother\\b"), py_str("\\bwife\\b"), py_str("\\bgirlfriend\\b"), py_str("\\bsister\\b"), py_str("\\bdaughter\\b"), py_str("\\blittle\\s+girl\\b"), py_str("\\bgirl\\b"), py_str("\\baunt\\b"), py_str("\\bnan\\b"), py_str("\\bgran\\b"), py_str("\\bgranny\\b"), py_str("\\bgrandma\\b"), py_str("\\bgrandmother\\b"), py_str("\\bniece\\b"), py_str("\\bshe\\b"), py_str("\\bher\\b")]));
    s__AGREEMENT(py_tuple(vec![py_tuple(vec![py_str("\\bthey\\s+stop\\b"), py_str("stop")]), py_tuple(vec![py_str("\\bthey\\s+need\\b"), py_str("need")]), py_tuple(vec![py_str("\\bthey\\s+feel\\b"), py_str("feel")]), py_tuple(vec![py_str("\\bthey\\s+seem\\b"), py_str("seem")]), py_tuple(vec![py_str("\\bthey\\s+come\\b"), py_str("come")]), py_tuple(vec![py_str("\\bthey\\s+look\\b"), py_str("look")]), py_tuple(vec![py_str("\\bthey\\s+get\\b"), py_str("get")]), py_tuple(vec![py_str("\\bthey\\s+go\\b"), py_str("go")]), py_tuple(vec![py_str("\\bthey\\s+start\\b"), py_str("start")]), py_tuple(vec![py_str("\\bthey\\s+keep\\b"), py_str("keep")]), py_tuple(vec![py_str("\\bthey\\s+have\\b"), py_str("have")]), py_tuple(vec![py_str("\\bthey\\s+were\\b"), py_str("were")]), py_tuple(vec![py_str("\\bthey\\s+are\\b"), py_str("are")]), py_tuple(vec![py_str("\\bthey\\s+do\\b"), py_str("do")])]));
    s_FIT_CASES(py_tuple(vec![py_str("medical"), py_str("chest"), py_str("breathing"), py_str("breath_past"), py_str("unconscious"), py_str("seizure"), py_str("stroke"), py_str("bleeding"), py_str("drowning"), py_str("fire"), py_str("poison"), py_str("minor"), py_str("heat"), py_str("fall"), py_str("scald"), py_str("red_flags"), py_str("stiff_neck"), py_str("sepsis_signs"), py_str("newborn_fever"), py_str("hypo"), py_str("old_fall"), py_str("shot"), py_str("eye_chem"), py_str("bone_out"), py_str("headache"), py_str("cough_blood"), py_str("fall_hip"), py_str("lips_swell"), py_str("breath_rest"), py_str("pregnancy_bp"), py_str("pet_loss"), py_str("baby_fell"), py_str("baby_fell_red"), py_str("puncture"), py_str("appendix"), py_str("stuck_machine"), py_str("poo_blood"), py_str("torsion"), py_str("safety"), py_str("swallow_gone")]));
    s_SWALLOWED_SELF(py_tuple(vec![py_str("Ring 111 (or your local urgent line) today rather than waiting, and keep a spare of whatever it was to show them; don't make yourself sick. If you start coughing, drooling, or finding it hard to swallow or breathe, that is 999 or 911 straight away."), py_str("Ring 111 (or your local urgent line) today with the packet or a spare in your hand, and don't make yourself sick or eat and drink for now. Coughing, drooling or trouble swallowing or breathing means 999 or 911 instead.")]));
    s_DOSE_WORDS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20)), (py_str("a\\s+couple"), Py::Int(2)), (py_str("half\\s+a\\s+dozen"), Py::Int(6)), (py_str("a\\s+dozen"), Py::Int(12))]));
    s_ASKS_IF_CALL(py_tuple(vec![py_str("\\b(?:should|shall|do|does|would|must|need|is\\s+it\\s+worth)\\s+(?:i|we|you|he|she|they)\\b[^.?!]{0,24}\\b(?:call|ring|phone|dial|get)\\s+(?:an?\\s+|the\\s+)?(?:ambulance|999|911|112|a\\s*&?\\s*e|a\\s*and\\s*e|emergency\\s+services|paramedics|out\\s+of\\s+hours)\\b"), py_str("\\b(?:do|does)\\s+(?:i|we|he|she|they)\\s+need\\s+(?:an?\\s+)?(?:ambulance|999|911|a\\s*&?\\s*e|a\\s+and\\s+e|the\\s+er|paramedics)\\b"), py_str("\\bis\\s+(?:this|that|it)\\s+(?:an?\\s+)?(?:emergency|999\\s+call|ambulance\\s+job)\\b"), py_str("\\b(?:am|are|is)\\s+(?:i|we|he|she|they)\\s+(?:meant\\s+to|supposed\\s+to)\\s+(?:call|ring)\\b[^.?!]{0,20}\\b(?:999|911|ambulance)\\b")]));
    s_CALL_HAS_A_SIGN(py_tuple(vec![py_str("\\bchest\\b"), py_str("\\bbreath\\w*"), py_str("\\bbleed\\w*"), py_str("\\bunconscious\\b"), py_str("\\bunresponsive\\b"), py_str("\\b(?:fit|seizure|seized|fitting)\\b"), py_str("\\bstroke\\b"), py_str("\\bblue\\b"), py_str("\\bswallow\\w*"), py_str("\\boverdos\\w*"), py_str("\\bpoison\\w*"), py_str("\\bburn\\w*"), py_str("\\bchoking\\b"), py_str("\\bcollapsed\\b"), py_str("\\bnot\\s+waking\\b"), py_str("\\b(?:pain|hurts?|hurting|ache\\w*)\\b"), py_str("\\bfever\\b"), py_str("\\brash\\b"), py_str("\\bvomit\\w*"), py_str("\\bblood\\b"), py_str("\\bfell\\b"), py_str("\\bcut\\b")]));
    s_ASKS_IF_GOING(py_tuple(vec![py_str("\\b(?:should|shall|do|does|would|must|need|is\\s+it\\s+worth)\\s+(?:i|we|you|he|she|they)\\b[^.?!]{0,24}\\b(?:take|bring|get|drive|walk)\\s+(?:\\w+\\s+){0,2}?(?:to|into)\\s+(?:a\\s*&?\\s*e|a\\s*and\\s*e|the\\s+er|er|casualty|the\\s+hospital|hospital|the\\s+emergency\\s+room|out\\s+of\\s+hours)\\b"), py_str("\\b(?:should|shall|do|does|would|must|need)\\s+(?:i|we|you|he|she|they)\\s+go\\s+(?:in|to|into)\\s+(?:a\\s*&?\\s*e|a\\s*and\\s*e|the\\s+hospital|hospital|the\\s+er|er|casualty|the\\s+emergency\\s+room|out\\s+of\\s+hours)\\b")]));
    s_ASK_GO_REPLIES(py_tuple(vec![py_str("If it can't wait until the morning, take them in — A&E is for tonight's problems, and you don't need an ambulance to go. If you're not sure, ring 111 first and describe it."), py_str("Go in if it can't wait for a GP appointment, and ring 111 if you're unsure which door to use. Take the medicine box with you and say what happened at the start of the story.")]));
    s_AMBULANCE_CASES(call_builtin("frozenset", &[py_tuple(vec![py_str("chest"), py_str("chest_slow"), py_str("stroke"), py_str("breathing"), py_str("not_breathing"), py_str("unconscious"), py_str("bleeding"), py_str("choking"), py_str("seizure"), py_str("overdose"), py_str("poison"), py_str("drowning"), py_str("hanging"), py_str("heat"), py_str("head_injury")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_GO_BARE(py_tuple(vec![py_str("\\bgo\\s+in\\b"), py_str("\\bgo\\s+to\\s+(?:a\\s*&?\\s*e|a\\s*and\\s*e|the\\s+hospital|hospital|casualty|the\\s+er|the\\s+emergency\\s+room)\\b")]));
    s_GO_IN_NOT_ABOUT_IT(py_tuple(vec![py_str("\\b(?:work|office|job|shift|school|college|uni(?:versity)?|nursery|childminder)\\b")]));
    s_GO_IS_A_CALL(py_tuple(vec![py_str("Going is the call, not a drive: ring 999 or 911 and say what is happening, and let them bring you in. ")]));
    s_ASK_CALL_REPLIES(py_tuple(vec![py_str("If you're asking, that usually means ring: 999 or 911 now if someone can't breathe, won't wake up, has chest pain or is bleeding heavily, and 111 or your GP today for anything less than that. Nobody minds you ringing and being wrong."), py_str("When in doubt, ring them and say what you can see — 999 or 911 if it's someone's breathing or waking, 111 or your surgery for the rest. They would far rather take the call than miss one."), py_str("Ring 999 or 911 if someone can't breathe, won't wake, has chest pain or is losing blood fast; 111 or your GP today otherwise. Say what you're seeing out loud and let them sort the rest.")]));
    s_SWALLOWED_BAD_REPLY(py_tuple(vec![py_str("Ring 999 or 911 now: a battery, a magnet or anything sharp or long can burn or block within hours, however well you feel. Don't make them sick, don't give them food or drink, and have an identical one ready to name on the call."), py_str("That is a 999 or 911 call now, not a wait-and-see: a battery burns, and a magnet or anything sharp or long can block or tear on its way through. Don't make them sick and don't give them anything to eat or drink; keep a spare of whatever it was to show the staff.")]));
    s_SWALLOWED_BAD_SELF(py_tuple(vec![py_str("Ring 999 or 911 now: a battery, a magnet or anything sharp or long can burn or block within hours, however well you feel. Don't make yourself sick and don't eat or drink anything; keep a spare of whatever it was to show the staff."), py_str("That is a 999 or 911 call now, not a wait-and-see: a battery burns, and a magnet or anything sharp or long can block or tear on its way through. Don't make yourself sick, don't eat or drink anything, and take a spare of whatever it was with you.")]));
    s_TYPED_TYPOS(py_dict(vec![(py_str("siezure"), py_str("seizure")), (py_str("siezures"), py_str("seizures")), (py_str("siesure"), py_str("seizure")), (py_str("siesures"), py_str("seizures")), (py_str("siezuring"), py_str("seizing")), (py_str("siezed"), py_str("seized")), (py_str("braethe"), py_str("breathe")), (py_str("braethes"), py_str("breathes")), (py_str("braething"), py_str("breathing")), (py_str("braethed"), py_str("breathed")), (py_str("breate"), py_str("breathe")), (py_str("breatheing"), py_str("breathing")), (py_str("kll"), py_str("kill")), (py_str("klling"), py_str("killing")), (py_str("k!ll"), py_str("kill")), (py_str("k1ll"), py_str("kill")), (py_str("k!lling"), py_str("killing")), (py_str("k1lling"), py_str("killing"))]));
    s_LOOKS_REAL(call_builtin("frozenset", &[mcall(&py_str("seize seized seizes seizing secure secured securer secures securely security securing securer security breadth breadths cheat cheats cheated cheating cheater chests flop flops flopped flopping wreathe wreathes wreathed wreathing breather breathers breathable breathers floppy-disk choked chokes choking-hazard overdo overdos overdoes overdos overdid overdoing overdone flappy flippy"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_NOW_FICTION(py_tuple(vec![py_str("\\bfor\\s+(?:a|my|the)\\s+(?:story|novel|book|film|script|screenplay|essay|assignment|character)\\b"), py_str("\\bin\\s+(?:a|my|the)\\s+(?:story|novel|book|film|game|dream)\\b"), py_str("\\b(?:story|novel|script|screenplay|essay|film)\\b.*\\b(?:character|plot|scene)\\b"), py_str("\\bmy\\s+(?:character|protagonist)\\b")]));
    s_NOT_NOW(add(&g_NOT_NOW_FICTION(), &py_tuple(vec![py_str("\\bif\\s+(?:someone|a\\s+person|somebody|you|your\\s+friend)\\b"), py_str("\\bwhat\\s+(?:do|should)\\s+(?:you|i|we)\\s+do\\s+(?:if|when)\\b"), py_str("\\bwhat\\s+(?:if|would\\s+happen\\s+if)\\b"), py_str("\\bhow\\s+do\\s+you\\s+(?:help|treat|deal\\s+with)\\b"), py_str("\\bhow\\s+(?:can|do|should)\\s+(?:i|we|you)\\s+(?:help|support|talk\\s+to|deal\\s+with|approach|comfort)\\b"), py_str("\\bfirst\\s+aid\\s+(?:for|course|kit|training)\\b"), py_str("\\b(?:qualified|trained|training|work(?:ing)?\\s+as)\\b.*\\b(?:nurse|doctor|paramedic|lifeguard)\\b"), py_str("\\b(?:how|what)\\b.{0,15}\\b(?:know|tell|spot|recognise|recognize|signs?|symptoms?)\\b"), py_str("\\bwhat\\s+does\\b.{0,25}\\b(?:feel\\s+like|look\\s+like|mean|do\\s+to\\s+you)\\b"), py_str("\\b(?:signs?|symptoms?|causes?|cause[sd]?|prevent\\w*|reduce\\s+the\\s+risk)\\b.{0,25}\\b(?:of\\s+)?(?:a\\s+)?(?:stroke|seizures?|choking|anaphyla\\w*|asthma|hypo\\w*|heart\\s+attack)\\b")])));
    s_WORLD_QUESTION(py_tuple(vec![py_str("\\bwhat\\s+(?:causes?|are\\s+the\\s+causes?)\\b"), py_str("\\bwhy\\s+(?:do|does|would)\\b"), py_str("\\b(?:symptoms?|signs?|treatment|rate|rates|statistics|study|research|awareness|prevention)\\b"), py_str("\\bwhat\\s+(?:is|are)\\s+(?:the\\s+)?(?:difference|best\\s+way)\\b"), py_str("\\bhow\\s+(?:common|likely|dangerous|many|often)\\b"), py_str("\\b(?:book|film|documentary|series|article)\\s+about\\b"), py_str("\\bcarbon\\s+monoxide\\b.{0,30}\\b(?:dangerous|kill\\w*|harm\\w*|fatal|safe)\\b"), py_str("\\b(?:is|are|can|does|do)\\b.{0,25}\\b(?:carbon\\s+monoxide|co\\s+(?:alarm|detector|monitor))\\b.{0,30}\\b(?:dangerous|kill\\w*|harm\\w*|fatal|safe)\\b")]));
    s_ACCIDENT(add(&py_tuple(vec![py_str("\\b(?:cut|cutting|nicked|sliced|grazed|scraped|hurt|burn\\w*|scalded|injured)\\s+my\\s*self\\b.{0,24}\\b(?:while\\s+(?:shaving|chopping|cooking|slicing|peeling|grating|opening|washing|cleaning|gardening|trimming|diy|cooking\\s+dinner|making)|at\\s+(?:work|netball|football|rugby|hockey|school|the\\s+gym|training|practice)|playing\\s+(?:football|rugby|netball|hockey|basketball|tennis|squash|badminton)|doing\\s+(?:diy|the\\s+gardening|the\\s+decorating)|accidentally|by\\s+(?:accident|mistake)|on\\s+accident)\\b"), py_str("\\b(?:cut|cutting|nicked|sliced|grazed|scraped|hurt)\\s+my\\s*self\\s+(?:again\\s+)?(?:shaving|chopping|cooking|slicing|peeling|grating|opening|washing|cleaning|gardening|trimming|diy)\\b"), py_str("\\b(?:cut|cutting|nicked|sliced|grazed|scraped|hurt|burn\\w*|scalded)\\s+my\\s*self(?:\\s+\\w+){0,2}\\s+(?:on|with)\\s+(?:(?:a|an|the|some)\\s+)?(?:\\w+\\s+){0,5}(?:knife|glass|tin|can|blade|lid|grater|packet|jar|pan|oven|kettle|saw|drill|branch|wire|needle|pin|scissors|hot\\s+water|boiling\\s+water)\\b")]), &crate::gen::modules::safety_cards::g_INJURY_HOW()));
    s_SELF_HARM_AGAIN(py_tuple(vec![py_str("\\bmyself\\s+again\\b"), py_str("\\b(?:cut|cutting|cuts|sliced|nicked|scratched)\\s+my\\s*self\\b[^.?!]{0,30}\\b(?:glass|mirror|shards?|blade|razor)\\b[^.?!]{0,8}\\bagain\\b")]));
    s_CRISIS_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:dead|died|die|dying|death|kill|killing|suicide|suicidal|exist|existing|ending|end\\s+it|hopeless|worthless|burden|disappear|better\\s+off|no\\s+reason|give\\s+up|\\bi\\s+have\\s+a\\s+plan|see\\s+the\\s+point|what'?s\\s+the\\s+point|(?:my|the|his|her|their)\\s+(?:last\\s+|first\\s+|second\\s+|previous\\s+|suicide\\s+|own\\s+)?attempt\\b(?!\\s+at\\b)|suicide\\s+attempt|attempted\\s+suicide|overdos\\w*|tried\\s+to\\s+(?:kill|end)\\s+my\\s*self)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CRISIS_PLAN_SAID(py_tuple(vec![py_str("\\bi\\s+know\\s+how\\s+i'?d\\s+(?:do\\s+it|end\\s+it|go\\b|be\\s+done)\\b"), py_str("\\b(?:i'?d|i\\s+would)\\s+know\\s+(?:exactly\\s+)?how\\s+to\\s+do\\s+it\\b"), py_str("\\bsuicide\\s+note\\b|\\bgoodbye\\s+letter\\b"), py_str("\\b(?:wrote|written|writing)\\s+(?:the|my)\\s+note\\b"), py_str("\\bthe\\s+note\\s+is\\s+written\\b"), py_str("\\b(?:gave|giving|given)\\s+(?:all\\s+)?my\\s+(?:\\w+\\s+)?(?:things|stuff|belongings|possessions|clothes|books|records)\\s+(?:all\\s+)?away\\b(?!\\s+to\\s+(?:a|the|my)?\\s*(?:charity|shops?|charity\\s+shop|hospice|tip|car\\s+boot|boot\\s+sale|jumble\\s+sale))"), py_str("\\b(?:said|saying|written)\\s+(?:my\\s+)?goodbyes?\\b"), py_str("\\bwon'?t\\s+be\\s+around\\b(?!\\s+(?:next|on|today|tomorrow|tonight|this|at|for|till|until|before|during|in|friday|monday|tuesday|wednesday|thursday|saturday|sunday)\\b)"), py_str("\\bnot\\s+(?:be\\s+)?around\\s+(?:much\\s+)?(?:longer|after\\s+this)\\b"), py_str("\\b(?:got|bought|have|saved)\\s+(?:the|a)\\s+(?:pills|tablets|rope|noose|razors?)\\s+(?:ready|sorted|saved|lined\\s+up|in\\s+my\\s+(?:bag|pocket|drawer|room))\\b"), py_str("\\btied\\s+a\\s+noose\\b")]));
    s_SELF_HARM_REASON(py_tuple(vec![py_str("\\b(?:want|wanna|need)\\s+to\\s+(?:keep\\s+going|carry\\s+on|go\\s+on|give\\s+up|end\\s+it|stop|disappear|not\\s+be\\s+here|die)\\b"), py_str("\\b(?:on\\s+purpose|deliberately|every\\s+(?:day|night|week)|all\\s+the\\s+time|to\\s+cope|to\\s+feel|to\\s+punish|to\\s+calm\\s+down|to\\s+hurt\\s+my\\s*self|to\\s+see\\s+(?:the\\s+)?blood|to\\s+relieve|to\\s+stop\\s+the\\s+noise|to\\s+feel\\s+(?:something|anything|real|pain)|when\\s+(?:i|im|i'?m)\\s+(?:get\\s+|got\\s+|am\\s+|feel\\s+)?(?:overwhelmed|upset|anxious|stressed|panicky|low|angry|numb))\\b")]));
    s_END_MEANS_OTHER(py_tuple(vec![py_str("\\b(?:he|she|they|my\\s+(?:mum|mom|mother|dad|father|son|daughter|child|kid|brother|sister|step\\w+|husband|wife|partner|boyfriend|girlfriend|fella|ex|friend|mate|flatmate|housemate|neighbou?r|colleague|boss|classmate|patient|client|lodger|aunt|auntie|uncle|cousin|nephew|niece|gran|nana|nan|grandad|grandpa|grandson|granddaughter))\\b[^.?!]{0,30}\\b(?:end(?:ing)?\\s+(?:it|(?:his|her|their)\\s+(?:own\\s+)?life)|kill\\s+(?:him|her|them)sel(?:f|ves)|take\\s+(?:his|her|their)\\s+own\\s+life)\\b")]));
    s_END_IT_WITH_NOUN(py_str("(?:husband|wife|partner|boyfriend|girlfriend|fella|missus|fiance|fiancee|ex|marriage|relationship|dad|mum|mom|father|mother|son|daughter|child|children|brother|sister|family|parents|inlaws|in\\s+laws|flatmate|housemate|tenant|colleague|mate|friend|friends|boss|job|course)"));
    s_SELF_HARM_DESERVE(py_tuple(vec![py_str("\\bdeserve[sd]?\\s+(?:it|this|pain)\\b"), py_str("\\bi\\s+deserve[sd]?\\s+(?:to\\s+)?(?:hurt|suffer|die|be\\s+in\\s+pain|the\\s+pain)\\b")]));
    s_WISH_BY_MODAL(py_tuple(vec![py_str("\\bi\\s+(?:should|ought\\s+to|might\\s+as\\s+well|may\\s+as\\s+well)\\s+(?:just\\s+|probably\\s+|really\\s+|honestly\\s+|seriously\\s+)?(?:die\\b(?!\\s+(?:my|the|it|them|her|his|your|laughing|of\\s+laugh(?:ing|ter))\\b)|be\\s+dead\\b(?!\\s+(?:after|following)\\b)|kill\\s+my\\s*self\\b|end\\s+my(?:\\s+own)?\\s+life\\b|not\\s+be\\s+here\\b(?!\\s*[^.?!]{0,40}\\bshould\\s+be\\s+(?!dead|gone))|be\\s+gone\\b(?!\\s+(?:by|before)\\b))"), py_str("\\bi\\s+(?:should|ought\\s+to|might\\s+as\\s+well|may\\s+as\\s+well)\\s+(?:\\w+\\s+){0,2}end\\s+(?:it|things)\\s*(?:[.?!]|$|tonight\\b|now\\b|for\\s+good\\b)")]));
    s__FLAT_WISH_HOW(py_str("|like\\s+(?:this|that|a\\s+(?:prisoner|slave|dog|zombie|captive))|(?:this|that)\\s+way|(?:this|my)\\s+life|(?:my\\s+)?life\\s+for\\s+(?:others|other\\s+people|everyone\\s+else|everybody\\s+else)|for\\s+(?:other\\s+people|others|everyone\\s+else|everybody\\s+else)"));
    s_END_THING(py_tuple(vec![py_str("\\b(?:end(?:ing)?|close|closing|finish(?:ing)?|wrap(?:ping)?\\s+up)\\b[^.?!]{0,24}\\b(?:this|that|the|my|an?)\\s+(?:email|e-mail|letter|report|essay|list|form|filing|tax\\s+return|argument|row|draft|chapter|book|project|job|task|sentence|code|story|application|coursework)\\b"), py_str("\\b(?:this|that|the|my|an?)\\s+(?:email|e-mail|letter|report|essay|list|form|filing|tax\\s+return|argument|row|draft|chapter|book|project|job|task|sentence|code|story|application|coursework)\\b[^.?!]{0,24}\\b(?:end(?:ing)?|close|closing|finish(?:ing)?)\\b")]));
    s_GIVING_UP_HELD_ON(py_tuple(vec![py_str("\\b(?:can'?t|cannot|can\\s+not|won'?t|will\\s+not|don'?t|do\\s+not|didn'?t|did\\s+not|doesn'?t|does\\s+not|shouldn'?t|should\\s+not|mustn'?t|must\\s+not|wouldn'?t|would\\s+not|never|not|no\\s+longer)\\s+(?:ever\\s+)?(?:giv(?:e|es|ing)|gave|giv(?:en))\\s+up\\b")]));
    s_SELF_HARM_NOT_EATING(py_tuple(vec![py_str("\\b(?:stopped|stop|quit|not|haven'?t|hasn'?t|hadn'?t|didn'?t|have\\s+not|has\\s+not|had\\s+not|did\\s+not)\\s+(?:been\\s+)?(?:eating|drinking|eaten|ate|eat|drink|drunk)\\b(?=\\s*(?:$|[.,!?;]|on\\s+purpose|deliberately|again\\b|anymore\\b|at\\s+all\\b|properly\\b|much\\b|for\\b|in\\s+(?:days|weeks|a\\s+week|a\\s+while)\\b|since\\b))"), py_str("\\b(?:haven'?t|hasn'?t|hadn'?t|have\\s+not|has\\s+not|had\\s+not)\\s+(?:eaten|had)\\s+(?:anything|much)\\b(?=(?:\\s+to\\s+(?:eat|drink))?\\s*(?:$|[.,!?;]|on\\s+purpose|deliberately|in\\s+days\\b|for\\s+days\\b|all\\s+day\\b|today\\b))"), py_str("\\bbarely\\s+(?:eaten|eating|eat|ate|drunk|drinking|drink)\\b"), py_str("\\b(?:ate|eat|eating)\\s+nothing\\b(?=\\s*(?:[.,!?;]|$|on\\s+purpose|deliberately))"), py_str("\\b(?:skip|skipped|skipping)\\s+(?:a\\s+meal|meals|food)\\b(?=\\s*(?:[.,!?;]|$|on\\s+purpose|deliberately))"), py_str("\\b(?:went|go|going|gone)\\s+without\\s+(?:food|eating)\\b(?=\\s*(?:[.,!?;]|$|on\\s+purpose|deliberately))"), py_str("\\brefused\\s+(?:food|to\\s+eat)\\b(?=\\s*(?:[.,!?;]|$|on\\s+purpose|deliberately))")]));
    s_SELF_HARM_DELIBERATE_ACT(py_tuple(vec![py_str("\\b(?:hurt|hurting|cut|cutting|burn|burnt|burned|burning|scald|scalded|scalding|bruise|bruised|bruising|starve|starved|starving|scratch|scratched|scratching|hit|hitting|bang|banged|banging|harm|harmed|harming|poison|poisoned|poisoning|smash|smashed|smashing|bite|biting|bit|bitten|break|breaks|broke|broken|breaking|punish|punished|punishing|stab|stabbed|stabbing|slit|slitting|slash|slashed|punch|punched|punching|choke|choked|choking|strangle|strangled|strangling|suffocate|suffocated|suffocating|smother|smothered|smothering|wound|wounded|wounding|electrocute|electrocuted|electrocuting|slice|sliced|slicing|hang|hanged|hanging|hung|drown|drowned|drowning)\\b[^.?!]{0,24}\\b(?:my\\s*self|me|my\\s+(?:own\\s+)?(?:arm|arms|wrist|wrists|leg|legs|thigh|thighs|stomach|belly|head|hand|hands|finger|fingers|skin|face|eye|eyes|neck|throat|mouth|tongue|jaw|lip|lips|ear|ears|nose|cheek|cheeks|forehead|temple|temples|back|shoulder|shoulders|rib|ribs|knee|knees|foot|feet|ankle|ankles|hip|hips|chest|body|abdomen|waist|spine|bone|bones|vein|veins|groin))\\b"), py_str("\\bgave\\s+my\\s*self\\s+(?:a\\s+)?(?:black\\s+eye|black\\s+eyes|bruise|burn|cut|gash)\\b"), py_str("\\b(?:shoot|shoots|shooting|shot)\\s+(?:my\\s*self|me|my\\s+(?:own\\s+)?(?:arm|arms|leg|legs|thigh|thighs|stomach|belly|head|hand|hands|chest|body|foot|feet|neck|mouth|face))\\b"), py_str("\\b(?:set|setting)\\s+(?:my\\s*self|me|my\\s+(?:own\\s+)?(?:arm|arms|leg|legs|hand|hands|clothes|clothing|shirt|shirts|trousers|jumper|jumpers|coat|hoodie|sleeve|sleeves|body))\\s+(?:on\\s+fire|alight|ablaze)\\b"), py_str("\\bset\\s+fire\\s+to\\s+(?:my\\s*self|me|my\\s+(?:arm|arms|leg|legs|hand|hands|clothes|clothing|shirt|shirts|trousers|jumper|jumpers|coat|hoodie|sleeve|sleeves|body))\\b"), py_str("\\bpoured\\s+(?:petrol|gasoline|fuel|lighter\\s+fluid|white\\s+spirit)\\s+(?:on|over)\\s+(?:my\\s*self|me|my\\s+(?:arm|arms|leg|legs|hand|hands|clothes|clothing|shirt|shirts|trousers|jumper|jumpers|coat|hoodie|sleeve|sleeves|body))\\b"), py_str("\\b(?:made|make|making)\\s+my\\s*self\\s+(?:sick|throw\\s+up|vomit|pass\\s+out|bleed)\\b"), py_str("\\b(?:cut|cutting|cut\\s+up|burn|burnt|burned|burning|hurt|hurting|starve|starved|starving|bleed|bleeding|overdose|overdosed)\\b(?:\\s+(?:too\\s+)?(?:deep|deeper|badly))?\\s+(?:on\\s+purpose|deliberately)\\b"), g_SELF_HARM_NOT_EATING(), py_str("\\b(?:took|take|taking|taken|had|have|having|swallowed|swallow|swallowing|downed|downing|popped|popping|necked|drank|drink|drinking|drunk|ate|eat|eating|eaten)\\s+(?:(?:too\\s+many|some|the|those|these|my|his|her|their|a|an|every|all(?:\\s+(?:my|his|her|their|the|these|those))?)\\s+)?(?:pills?|tablets?|paracetamol|ibuprofen|painkillers?|meds|medicines?|sleeping\\s+pills?|antidepressants?|tranquill?i[sz]ers?|insulin|bleach|detergent|disinfectant|antifreeze|weed\\s?killer|hand\\s+saniti[sz]er|paint\\s?thinner|white\\s+spirit|drain\\s+cleaner|cleaning\\s+products?|cleaning\\s+fluid)\\b"), py_str("\\boverdos(?:e|ed|ing)\\b"), py_str("\\b(?:pull|pulled|pulling)\\s+my\\s+(?:own\\s+)?hair\\s+out\\b"), py_str("\\b(?:held|hold|holding)\\s+my\\s+(?:own\\s+)?head\\s+under\\s+(?:the\\s+)?water\\b"), py_str("\\b(?:put|puts|putting|place|placed|placing)\\s+my\\s+(?:own\\s+)?(?:hand|hands|arm|arms|finger|fingers|foot|feet)\\s+(?:in|into|on)\\s+(?:the\\s+)?(?:fire|flames|boiling\\s+water|hot\\s+water|stove|hob)\\b"), py_str("\\b(?:crash|crashed|crashing)\\s+(?:my|the|our)\\s+(?:car|van|motorbike|truck)\\b"), py_str("\\b(?:threw|throw|throwing)\\s+my\\s*self\\s+(?:down\\s+(?:the\\s+)?stairs|under\\s+(?:a|the)\\s+bus)\\b"), py_str("\\b(?:jump(?:ed|ing|s)?|fell|falling|(?:throw|threw|throwing)\\s+my\\s*self)\\b[^.?!]{0,10}\\b(?:off|from)\\s+(?:a|the|this)\\s+(?:bridge|roof|building|cliff|balcony|ledge|car\\s*park)\\b"), py_str("\\b(?:ran|run|running|walked|walk|walking|stepped|step|stepping|darted|dart|stood|stand|standing|lay|lying|lie)\\s+(?:into|in\\s+front\\s+of)\\s+(?:traffic|a\\s+bus|a\\s+train|a\\s+car|a\\s+lorry|a\\s+truck)\\b"), py_str("\\b(?:stood|stand|standing|lay|lying|lie|sat|sit|sitting)\\s+on\\s+(?:the\\s+tracks|the\\s+track|the\\s+railway\\s+line|a\\s+train\\s+track|the\\s+line)\\b"), py_str("\\b(?:used|use|using|took|take|taking|held|holding)\\s+(?:a|the|my)\\s+(?:knife|razor|blade|scissors|piece\\s+of\\s+glass|glass)\\s+(?:on|to|against)\\s+(?:my\\s*self|me|my\\s+(?:arm|arms|wrist|wrists|leg|legs|thigh|thighs|stomach|belly|head|hand|hands|finger|fingers|skin|face|eye|eyes|neck|back|shoulder|shoulders|rib|ribs|knee|knees|foot|feet|ankle|ankles|hip|hips|chest|body))\\b")]));
    s_HYPERBOLE_BLOCK(py_tuple(vec![py_str("\\b(?:my|his|her|their|our|the)\\s+(?:whole\\s+)?(?:life|marriage|relationship|family|husband|wife|partner|boyfriend|girlfriend|fianc\\w+|mum|mom|mother|dad|father|son|daughter|child|children|kids?|baby|brother|sister|boss|manager|health|mind|sanity)\\b"), py_str("\\beverything\\b"), py_str("\\bnothing\\b"), py_str("\\bno\\s+point\\b"), py_str("\\bworthless\\b"), py_str("\\bhopeless\\b"), py_str("\\bunbearable\\b"), py_str("\\b(?:died|death|funeral|buried|passed\\s+away|lost\\s+my|left\\s+me|walked\\s+out|divorce\\w*|separation|abuse\\w*|hits?\\s+me|cancer|tumour|tumor|diagnosis|miscarriage|stillborn|overdose\\w*|relapse\\w*|evicted|homeless|bankrupt)\\b")]));
    s_SOMEONE_ELSES_DESPAIR(py_tuple(vec![py_str("(?:can'?t|cannot|can\\s+not)\\s+go\\s+on\\b"), py_str("(?:there'?s|there\\s+is)\\s+no\\s+point\\b"), py_str("\\bno\\s+point\\s+(?:in\\s+)?(?:anything|going\\s+on|carrying\\s+on|anymore)\\b"), py_str("\\bno\\s+way\\s+out\\b"), py_str("\\b(?:is|are|feels?)\\s+hopeless\\b"), py_str("\\b(?:tired|sick)\\s+of\\s+(?:everything|it\\s+all|life|living)\\b"), py_str("\\b(?:wants?|wanted)\\s+to\\s+(?:die|end\\s+it)\\b"), py_str("\\b(?:doesn'?t|does\\s+not|don'?t|do\\s+not)\\s+want\\s+to\\s+(?:be\\s+here|live|go\\s+on|wake\\s+up)\\b")]));
    s_AFTER_EXERCISE(py_tuple(vec![py_str("\\b(?:after|following|post)[-\\s]+(?:[a-z]+\\s+){0,2}(?:gym|workout|work\\s?out|run|running|jog|jogging|exercise|exercising|training|session|swim|swimming|cycle|ride|football|rugby|netball|basketball|tennis|hockey|lifting|weights|spinning|hiit|class|match|game|climb|climbing|rowing|cardio|walk|walking|hike|hiking|steps|mile|miles|5k|10k|stairs)\\b"), py_str("\\b(?:at|in)\\s+the\\s+gym\\b"), py_str("\\bwhen\\s+(?:i\\s+|im\\s+|i'm\\s+)?(?:go\\s+|am\\s+|start\\s+)?(?:running|jogging|walking|climbing|cycling|swimming|exercising|training|lifting|playing|rowing|hiking|shovelling|shoveling)\\b"), py_str("\\bwhen\\s+i\\s+(?:run|jog|walk|climb|cycle|swim|exercise|train|play|lift|row|hike|shovel|shine)\\b"), py_str("\\b(?:on|up)\\s+the\\s+stairs\\b"), py_str("\\b(?:walking|running|jogging|climbing|cycling|swimming|lifting|exercising)\\s+(?:up|uphill|fast|hard|home|the\\s+(?:hill|stairs))\\b")]));
    s_CHEST_EFFORT(py_tuple(vec![py_str("Chest pain that comes on with effort is worth checking today, not next week: ring 111 or your GP first thing. If it turns to pressure, spreads to the arm or jaw, or comes with breathlessness or sweating, ring 999 or 911 straight away."), py_str("Pain in the chest on effort should be looked at today - ring 111 or your GP first thing, and let them hear when it comes on. Ring 999 or 911 straight away if it becomes pressure, spreads to the arm or jaw, or comes with breathlessness or sweating.")]));
    s_CHEST_ASKED(py_tuple(vec![py_str("Chest pain is worth having looked at rather than waiting: ring 111 or your GP today. If it feels like pressure or tightness, spreads to the arm, jaw or back, or comes with breathlessness, sweating or feeling sick, ring 999 or 911 straight away."), py_str("Most chest pain is not the heart, but it is not something to work out on your own either: get it looked at today - 111 or your GP. Ring 999 or 911 if it is pressure or tightness, spreads to the arm or jaw, or comes with breathlessness, sweating or feeling faint.")]));
    s_NOT_STOPPING(py_tuple(vec![py_str("\\b(?:bleeding|blood)\\s+(?:through|soak\\w*\\s+through|through\\s+to)\\b.{0,20}\\b(?:cloth|bandage|dressing|towel|plaster|gauze|padding|scarf|t-?shirt|sock|sleeve)s?\\b"), py_str("\\b(?:blood|bleed\\w*|cut|wound|gash)\\b.{0,40}\\b(?:soaked|soaking|soaks|soak)\\s+through\\b"), py_str("\\b(?:soaked|soaking|soaks)\\s+through\\b.{0,20}\\b(?:blood|bleed\\w*)\\b"), py_str("\\bblood\\s+(?:is\\s+)?(?:dripping|pouring|running)\\b"), py_str("\\b(?:bleeding|blood)\\s+everywhere\\b")]));
    s_SOFT_BLEED_OFF(py_tuple(vec![py_str("\\ba\\s+bit\\b"), py_str("\\ba\\s+little\\b"), py_str("\\bslightly\\b"), py_str("\\b(?:small|paper|shaving)\\s+cut\\b"), py_str("\\b(?:gums?|tooth|teeth|mouth\\s+ulcer)\\b"), py_str("\\bperiod\\b"), py_str("\\bhemorrhoid"), py_str("\\b(?:tattoo|steak|meat|paint|dye|ink|marker|sauce|juice|tomato|jam|stain|wall|pipes?)\\b"), py_str("\\b(?:has|have|had|it|they)\\s+(?:now\\s+)?stopp?ed\\b"), py_str("\\bno\\s+longer\\s+bleeding\\b")]));
    s_BODY_PART(py_str("\\b(?:my|his|her|their|your|our|the)\\s+(?:hand|hands|finger|fingers|thumb|arm|arms|leg|legs|knee|knees|foot|feet|head|lip|face|chin|shin|wrist|ankle|toe|toes|shoulder|side|back|neck|chest|skin|palm|knuckle|forearm|thigh|calf|ear|mouth|eyebrow|scalp|heel|elbow)\\b"));
    s_CHEST_SPREADING(py_tuple(vec![py_str("\\b(?:(?:arm|arms|jaw|shoulders?)\\s+(?:went\\s+|is\\s+|are\\s+|feels?\\s+|have\\s+gone\\s+)?(?:numb|tingl\\w*|pain\\w*|ache\\w*|hurts?|heav\\w*)|numb(?:ness|ed|ing|s)?|tingl\\w*|sweat\\w*|clammy|sick|nausea\\w*|vomit\\w*|breathless\\w*|dizzy|faint\\w*|grey|gray|pale|crushing\\s+pain)\\b")]));
    s_A_PERSON(py_tuple(vec![py_str("\\b(?:i|me|myself|he|she|him|her|they|them|his|hers|their|someone|somebody|anyone|man|woman|boy|girl|child|children|baby|babies|toddler|patient|kid|kids|mum|mom|mother|dad|father|son|daughter|husband|wife|partner|boyfriend|girlfriend|brother|sister|friend|gran|grandma|grandad|grandpa|nan|uncle|aunt|niece|nephew|colleague|neighbour|neighbor|people|men|women|workers?|guests?|everyone|anybody)\\b")]));
    s_GIVING_UP_GROUP(py_tuple(vec![py_str("\\b(?:team|teams|club|squad|side|players?|group|class|band|orchestra|choir|committee|company|staff|crew|panel|jury|government|opposition|management|union|audience|crowd|fans?|supporters?|students?|pupils?|workers?|colleagues?|strikers?)\\b[^.?!]{0,30}\\b(?:giv(?:e|es|ing)|gave|giv(?:en))\\s+up\\b")]));
    s__DENIED(py_tuple(vec![py_str("\\b(?:i'?m|im|i\\s+am|i\\s+was)\\s+not\\s+(?:feeling\\s+|really\\s+|actually\\s+)?(?:suicidal|depressed\\s+enough|thinking\\s+(?:about|of)\\s+(?:suicide|killing|hurting)|going\\s+to\\s+(?:kill\\s+my\\s*self|hurt\\s+my\\s*self)|about\\s+to\\s+(?:kill\\s+my\\s*self|end\\s+it))"), py_str("\\bi\\s+(?:would\\s+never|wouldn'?t|will\\s+not|won'?t)\\s+(?:kill\\s+my\\s*self|hurt\\s+my\\s*self|harm\\s+my\\s*self|cut\\s+my\\s*self|end\\s+(?:it|my\\s+life)|top\\s+my\\s*self|die)\\b"), py_str("\\bi\\s+(?:dont|don'?t|do\\s+not)\\s+(?:want|wanna)\\s+(?:to\\s+)?(?:kill\\s+my\\s*self|harm\\s+my\\s*self|end\\s+it|top\\s+my\\s*self)\\b"), py_str("\\bi\\s+(?:have\\s+)?never\\s+(?:felt\\s+suicidal|(?:wanted|want)\\s+to\\s+die|thought\\s+(?:about|of)\\s+suicide|hurt\\s+my\\s*self)\\b"), py_str("\\bi\\s+(?:would\\s+)?never\\s+want\\s+to\\s+die\\b"), py_str("\\b(?:not|never)\\s+(?:suicidal|thinking\\s+about\\s+suicide)\\b")]));
    s_DAY_TOTAL_WORD(py_tuple(vec![py_str("\\b(?:today|tonight|this\\s+morning|this\\s+afternoon|this\\s+evening|so\\s+far|this\\s+week|over\\s+the\\s+day|across\\s+the\\s+day|during\\s+the\\s+day|in\\s+a\\s+day|in\\s+the\\s+day|in\\s+24\\s+hours|since\\s+(?:this\\s+morning|last\\s+night|yesterday|6\\s?am|lunch))\\b")]));
    s_DAY_TOTAL_SIGN(py_tuple(vec![py_str("\\b(?:feel|felt|feeling)\\s+(?:sick|ill|rough|queasy|nauseous|dizzy|faint)\\b"), py_str("\\b(?:vomit\\w*|throwing\\s+up|threw\\s+up|been\\s+sick|being\\s+sick)\\b"), py_str("\\b(?:stomach|tummy|abdomen|belly)\\b"), py_str("\\byellow\\b"), py_str("\\bbruis\\w*\\b"), py_str("\\b(?:sweat\\w*|shaking|shaky|ringing\\s+in\\s+my\\s+ears)\\b"), py_str("\\b(?:overdos\\w*|too\\s+many|whole\\s+(?:packet|pack|box|bottle))\\b")]));
    s_SAID_AS_IT_HAPPENS(py_tuple(vec![py_str("\\bright\\s+now\\b"), py_str("\\bnow\\b"), py_str("\\bsuddenly\\b"), py_str("\\bcurrently\\b"), py_str("\\bat\\s+the\\s+moment\\b"), py_str("\\bas\\s+we\\s+speak\\b"), py_str("\\bstill\\b"), py_str("\\bhappening\\b"), py_str("\\b(?:my|our|his|her|their|the)\\s+(?:son|daughter|child|kid|kids|baby|babies|toddler|husband|wife|partner|boyfriend|girlfriend|mum|mom|mother|dad|father|friend|mate|neighbour|neighbor|brother|sister|grandma|grandad|granny|nan|nana|aunt|uncle|cousin|flatmate|roommate|lodger|carer|patient|client|dog|cat|puppy|kitten)(?:'s|s)?\\b"), py_str("\\bstop(?:ped|ping|s)?\\s+breathing\\b"), py_str("\\bnot\\s+breathing\\b"), py_str("\\bisn'?t\\s+breathing\\b"), py_str("\\bwon'?t\\s+wake\\b"), py_str("\\bwont\\s+wake\\b"), py_str("\\bno\\s+pulse\\b"), py_str("\\bunresponsive\\b")]));
    s_OWN_CRY(py_tuple(vec![py_str("\\b(?:i|im|i'm|i've|me|my)\\b[^.?!]{0,25}\\b(?:suicid\\w*|self[-\\s]?harm\\w*|kill\\s+my\\s*self|want\\s+to\\s+die|wants?\\s+to\\s+die|not\\s+want\\s+to\\s+(?:live|be\\s+here)|don'?t\\s+want\\s+to\\s+(?:live|be\\s+here)|end\\s+it\\s+all|better\\s+off\\s+dead|hurt\\s+my\\s*self|harm(?:ing)?\\s+my\\s*self|cutting\\s+my\\s*self|hope?less)\\b"), py_str("\\b(?:suicid\\w*|self[-\\s]?harm\\w*)\\b[^.?!]{0,25}\\b(?:i|me|my)\\b")]));
    s_SELF_HARM_CUT_NO_OBJECT(py_tuple(vec![py_str("\\b(?:i\\s+)?(?:want|wanna|wanting|need|needing|feel\\s+like|feeling\\s+like|felt\\s+like|thinking\\s+(?:of|about)|planning\\s+(?:to|on)|going\\s+to|about\\s+to)\\s+(?:to\\s+)?(?:just\\s+)?cut(?:ting)?\\s*(?:$|[.?!;,]|\\b(?:something|tonight|again)\\b|\\bso\\s+(?:bad|much|badly)\\b|\\breally\\s+bad(?:ly)?\\b)")]));
    s_ABUSE_FINE_PLACE(py_tuple(vec![py_str("(?:^|\\s)(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad)|(?:the|our|my)\\s+neighbours?|next\\s+door)\\b[^.?!]{0,30}\\bthreaten(?:s|ed|ing)?\\s+me\\b[^.?!]{0,40}\\b(?:at\\s+work|from\\s+work)\\b"), py_str("(?:^|\\s)(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad))\\s+(?:at\\s+work|from\\s+work)\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\s+me\\b")]));
    s_VIOLENT_THREAT(py_tuple(vec![py_str("\\b(?:kill|kills|killed|killing|murder|murders|stab(?:s|bed|bing)?|shoot|shoots|shot|gun|guns|knife|knives|weapon|weapons|strangle|strangled|choke|choked|rape|raped|beat|beats|beating|punch|punches|punched|slap|slaps|slapped|hit|hits|hitting|hurt|hurts|hurting|harm|harms|attack|attacks|attacked|throat|in\\s+danger|scared|terrified|afraid|following\\s+me|waiting\\s+outside|come\\s+after\\s+me)\\b")]));
    s_ABUSE_PLAY(py_tuple(vec![py_str("\\bmy\\s+(?:kid|kids|child|children|son|daughter|sister|brother|mate|mates|friend|friends|cousin|flatmate|housemate|boy|girl)\\b[^.?!]{0,25}\\b(?:push(?:ed|es)|shov(?:ed|es)|thr(?:ew|ows)|dump(?:ed|s)|pull(?:ed|s)|drag(?:ged|s))\\s+(?:me|us)\\s+(?:on)?to\\s+(?:the\\s+|a\\s+|my\\s+)?(?:sofa|couch|bed|floor|cushions?|beanbag|trampoline|grass|mattress)\\b")]));
    s_ABUSE_PLAY_HARD(py_tuple(vec![py_str("\\b(?:held|holding|hold|pinned|trapped|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|slap(?:ped|s|ping)?|throat|neck|choke(?:d|s|ing)?|strangl(?:ed|es|ing)|kill\\w*|murder\\w*|rape\\w*|bruis(?:e|es|ed|ing)|blood|bleed\\w*|hit|hits|hitting|beat|beaten|beating|drunk|scared|afraid|terrified|frightened|worthless|useless|stupid|thick|fat|ugly|whore|slut|bitch|cunt|shut\\s+up)\\b")]));
    s_ABUSE_HANDS_ON(py_tuple(vec![py_str("\\b(?:push(?:ed|es|ing)|shov(?:ed|es|ing)|grab(?:bed|s|bing)|drag(?:ged|s|ging)|pull(?:ed|s|ing)|held|holding|hold|pinn(?:ed|ing)|press(?:ed|ing)|slam(?:med|s|ming)|thr(?:ew|own|owing)|kick(?:ed|s|ing)|punch(?:ed|es|ing)|hit|hits|hitting|slapp?(?:ed|ing)|chok(?:ed|es|ing)|strangl(?:ed|es|ing)|bit|bitten|bent\\s+my|twist(?:ed|ing)\\s+my|yank(?:ed|ing)|wrench(?:ed|ing))\\s+(?:me|us|my|our)\\b"), py_str("\\bby\\s+the\\s+(?:hair|throat|neck|arm|wrist|collar)\\b"), py_str("\\b(?:held|pinned|pressed|slammed|shoved|pushed|threw)\\s+(?:me|us)\\s+(?:against|onto|into|to)\\b")]));
    s_ABUSE_QUARREL(py_tuple(vec![py_str("\\b(?:corner(?:ed|s)?|pinned|trapped|stuck|backed|blocked|held|shout(?:ed|s|ing)?\\s+at|yell(?:ed|s|ing)?\\s+at|argu(?:ed|es|ing))\\b[^.?!]{0,30}\\b(?:about|over)\\s+(?:the\\s+|a\\s+|my\\s+|our\\s+|his\\s+|her\\s+|these\\s+|those\\s+)?(?:deadline|deadlines|rota|roster|budget|essay|review|appraisal|numbers|figures|report|proposal|invoice|quotes?|tax|paper|project|contract|bills?|homework|exam|presentation|spreadsheet|work|targets?|quotas?|results?|grades?|coursework|assignment|bins?|rubbish|recycling|bin\\s+day|shopping|chores?|mess|washing|washing[\\s-]up|dishes|dinner|tea|bedtime|school|laundry|ironing|hoovering|litter|garden|hedge|car|keys|holiday|guests?|tv|telly|remote|football|dog|cat|nappies|nappy|toys?|clutter|tidying)\\b")]));
    s_ABUSE_CHILD_REFUSAL(py_tuple(vec![py_str("\\b(?:my|our)\\s+(?:son|sons|daughter|daughters|kid|kids|child|children|baby|toddler|teen|teens|teenager|teenagers|boy|girl|boys|girls)\\b[^.?!]{0,25}\\b(?:won'?t|wont|wouldn'?t|wouldnt|didn'?t|doesn'?t|(?:will|would|do|does|did)\\s+not|refus(?:es?|ed)\\s+to|never\\s+lets?)\\s+(?:let|allow)\\s+me\\b")]));
    s_ABUSE_PARENT_REFUSAL(py_tuple(vec![add(&add(&py_str("\\b(?:my\\s+)?(?:mum|mom|mother|dad|father|parents|step\\s?dad|step\\s?mum|step\\s?mom|step\\s?parents|guardian|guardians)\\b[^.?!]{0,25}\\b(?:won'?t|wont|wouldn'?t|wouldnt|didn'?t|doesn'?t|(?:will|would|do|does|did)\\s+not|refus(?:es?|ed)\\s+to|never\\s+lets?)\\s+(?:let|allow)\\s+me\\s+"), &crate::gen::modules::safety_phrases::g__REFUSED_EXIT()), &py_str("\\b"))]));
    s_ANY_REFUSAL_TO_GO(py_tuple(vec![add(&add(&py_str("\\b(?:won'?t|wont|wouldn'?t|wouldnt|didn'?t|doesn'?t|(?:will|would|do|does|did)\\s+not|refus(?:es?|ed|ing)\\s+to|never\\s+lets?|not\\s+letting|(?:is|was|are|were)n'?t\\s+letting)\\s+(?:let|allow)\\s+me\\s+"), &crate::gen::modules::safety_phrases::g__REFUSED_EXIT()), &py_str("\\b"))]));
    s_A_HAND_ON_THE_PERSON(py_tuple(vec![py_str("\\b(?:grab(?:bed|s|bing)|seiz(?:ed|es|ing)|twist(?:ed|ing)|yank(?:ed|ing)|wrench(?:ed|ing)|pinn(?:ed|ing)|pins|pinning|held|hold|holding|squeez(?:ed|ing)|shook|shak(?:en|es|ing)|pull(?:ed|s|ing)|slam(?:med|s|ming)|press(?:ed|ing)|clamp(?:ed|ing)|lift(?:ed|ing))\\s+(?:me|us|my|our)\\s*(?:by\\s+the\\s+)?(?:arm|arms|wrist|wrists|hair|throat|throats|neck|necks|collar|collars|shoulder|shoulders|hand|hands|face|jaw|mouth|leg|legs|waist|back|fist|fists|clothes|shirt|top|jumper|hood|scarf|belt)\\b"), py_str("\\b(?:push(?:ed|es|ing)|shov(?:ed|es|ing)|drag(?:ged|s|ging)|kick(?:ed|s|ing)|punch(?:ed|es|ing)|slapp?(?:ed|ing)|hit|hits|hitting|chok(?:ed|es|ing)|strangl(?:ed|es|ing)|bit|bitten|thr(?:ew|own|owing)|flung)\\s+(?:me|us|him|her|them)\\b(?!\\s+to\\b)(?:\\s+(?:by|against|across|onto|into|down|over|off|out\\s+of|in\\s+the|on\\s+the)\\b|[\\s.,!?]|$|\\s+(?:again|hard|back)\\b)"), py_str("\\b(?:grab(?:bed|s|bing)|shov(?:ed|es|ing)|shook|kick(?:ed|es)|punch(?:ed|es))\\s+(?:me|us|him|her|them)\\s*(?:[.,!?]|$|and\\b|while\\b|so\\b|then\\b)")]));
    s_ABUSE_EMPLOYER_REFUSAL(py_tuple(vec![py_str("\\b(?:boss|manager|supervisor|employer|company|firm|agency|payroll|hr|rota|rotas|shift|shifts|roster|office|workplace|job)\\b[^.?!]{0,40}?\\b(?:won'?t|wont|wouldn'?t|wouldnt|will\\s+not|doesn'?t|doesnt|does\\s+not|refuses?|refused|not\\s+letting)\\b[^.?!]{0,15}?\\b(?:let|allow)\\s+me\\b")]));
    s_ABUSE_PARENT_REFUSAL_HARD(py_tuple(vec![py_str("\\b(?:hit|hits|hitting|punched|punching|beat|beaten|beating|kick(?:ed|s|ing)|chok(?:ed|es|ing)|strangl(?:ed|es|ing)|slapp?(?:ed|s|ing)?|threw|thrown|hurt\\s+me|hurts\\s+me|bruis(?:e|es|ed)|blood|black\\s+eye|split\\s+lip|dragged|dragging|scare(?:s|d)?|scaring|terrified|afraid|frightened|frighten(?:s|ing)?|raped|assaulted|threaten(?:s|ed|ing)?|locked|lock(?:s|ing)\\s+me|shut\\s+me|starve\\w*|no\\s+food|belt|punish(?:ed|es|ing)?|burn(?:s|ed|ing)\\s+me|handcuff\\w*|tied\\s+me)\\b")]));
    s_PARENT_REFUSAL_BLOW(py_tuple(vec![py_str("\\b(?:hit|hits|hitting|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|beat|beats|beaten|beating|slapp?(?:ed|s|ing)?|chok(?:ed|es|ing)|strangl(?:ed|es|ing)|grabb(?:ed|es)|drag(?:ged|s|ging)|shov(?:ed|es)|push(?:ed|es)|thr(?:ew|own)|lock(?:ed|s|ing)|shut|shuts|shutting)\\s+(?:me|us)\\b"), py_str("(?<!\\bi\\s)(?<!\\bwe\\s)\\b(?:hit|hits|hitting|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|beat|beats|beaten|beating|slapp?(?:ed|s|ing)?|grabb(?:ed|es)|drag(?:ged|s|ging)|shov(?:ed|es)|push(?:ed|es)|thr(?:ew|own)|lock(?:ed|s|ing)|shut|shuts|shutting)\\s+my\\s+(?:arm|leg|face|head|back|ribs?|stomach|throat|hair|wrist|shoulder|neck|side|chest|knee|hip|belly|jaw|nose|hand|foot|feet|eye|ear)\\b"), py_str("\\b(?:hurt|hurts|hurt|hurting)\\s+(?:me|my\\s+\\w+|us)\\b"), py_str("\\b(?:he|she|they|dad|mum|mother|father|step\\s?dad|step\\s?mum|parents?|guardian)\\b[^.]{0,12}\\b(?:scares?|scared|scaring|terrif(?:ies|ied)|frighten(?:s|ed|ing)?)\\s+(?:me|us)\\b"), py_str("\\b(?:afraid|frightened|scared)\\s+of\\s+(?:him|her|them|my\\s+(?:dad|mum|stepdad|stepmum))\\b"), py_str("\\b(?:threaten(?:s|ed|ing)?\\s+(?:me|us|to\\s+kill)|gave\\s+me\\s+a\\s+(?:bruise|black\\s+eye|split\\s+lip)|left\\s+me\\s+with\\s+a\\s+(?:bruise|black\\s+eye|split\\s+lip)|belt|burn(?:s|ed|ing)?\\s+me|starve\\w*|no\\s+food|handcuff\\w*|tied\\s+me|rap(?:e|ed|es)|assault(?:ed|s)?)\\b")]));
    s_ABUSE_BENIGN_REASON(py_tuple(vec![py_str("\\b(?:corner(?:ed|s)?|trapped|backed|blocked|pinned|shov(?:ed|es)|push(?:ed|es)|held)\\s+(?:me|us)\\b[^.?!]{0,30}\\bfor\\s+(?:a\\s+|the\\s+|my\\s+|your\\s+|his\\s+|her\\s+)?(?:hug|selfie|photo|picture|cup\\s+of\\s+tea|tea|coffee|cake|biscuit|dinner|lunch|breakfast|leak|boiler|fuse|hoover|washing|shopping|presents?|surprise|tickets?|letter|parcel|book|coat|keys)\\b"), py_str("\\b(?:corner(?:ed|s)?|trapped|backed|blocked|pinned|shov(?:ed|es)|push(?:ed|es)|held)\\s+(?:me|us)\\b[^.?!]{0,30}\\bto\\s+(?:give|show|hand|fetch|get|check|fix|make|find|bring|grab|take|give\\s+over)\\s+(?:me\\s+|us\\s+)?(?:a\\s+|the\\s+|my\\s+|your\\s+|his\\s+|her\\s+)?(?:hug|selfie|photo|picture|cup\\s+of\\s+tea|tea|coffee|cake|biscuit|dinner|lunch|breakfast|leak|boiler|fuse|hoover|washing|shopping|presents?|surprise|tickets?|letter|parcel|book|coat|keys)\\b"), py_str("\\b(?:blocked|shoved|shoved?|pushed|backed|pinned)\\s+(?:me|us)\\s+[^.?!]{0,20}\\bwith\\s+the\\s+(?:hoover|vacuum|washing|shopping|pushchair|buggy|pram|bags?|ironing|car\\s+seat|bike|laundry|bin|mop|ironing\\s+board)\\b"), py_str("\\b(?:corner(?:ed|s)?|pinned|trapped|backed|blocked)\\b[^.?!]{0,40}\\bto\\s+(?:talk|ask|discuss|pitch|sell|persuade|convince|apologise|apologize|chat|catch\\s+up|explain|complain|run\\s+(?:something|an\\s+idea)\\s+by|check|tell)\\b")]));
    s_ABUSE_BUTTONHOLED(py_tuple(vec![py_str("\\b(?:(?:corner(?:ed|s)?|buttonhol(?:ed|es|ing)|pinned|trapped|backed|blocked)\\s+(?:me|us)|(?:had|has|have|got|keeps|keeping|kept)\\s+(?:me|us)\\s+cornered)\\b[^.?!]{0,40}\\babout\\s+(?:the|a|an|my|his|her|their|our|some|this|that)\\s+(?!knife|gun|blade|weapon|pills?|tablets?|money|rent|debt|affair|phone|keys|kids|children|divorce|will|deeds|police|hospital|bruises?|drink|drugs?|door|escape|leaving)\\w+")]));
    s_ABUSE_KIDS_CARD(py_tuple(vec![py_str("\\b(?:scared|afraid|frightened|terrified|panic(?:king|ked)?|crying|sobbing|shaking|shook|hurt\\s+me|hurts\\s+me|hit|hits|hitting|punched|punch|kick(?:ed|s)|slapped|slap|beaten|beat\\s+me|choked|choking|strangled|blood|bruis(?:e|es|ed)|black\\s+eye|split\\s+lip|raped|assaulted|worthless|useless|stupid|thick|fat|ugly|drunk|threat(?:ened|ens|s)?)\\b")]));
    s_ABUSE_KIDS_AT_HOME(py_tuple(vec![py_str("\\b(?:my|the)\\s+(?:kids|children|child|toddler|baby|boys|girls|twins|little\\s+ones)\\b[^.?!]{0,25}\\b(?:have|has|had|keep|keeps|keeping|kept)\\s+(?:me|us)\\s+(?:locked|stuck|indoors|in\\s+the\\s+house|stuck\\s+in)\\b"), py_str("\\b(?:my|the)\\s+(?:kids|children|child|toddler|baby|boys|girls|twins|little\\s+ones)\\b[^.?!]{0,25}\\b(?:won'?t|wont|will\\s+not|wouldn'?t|refuses?\\s+to|never\\s+lets?\\s+me)\\s+(?:let|allow)\\s+me\\s+(?:leave|go\\s+out|out|go\\s+anywhere|sleep|sit\\s+down|put\\s+them\\s+down)\\b")]));
    s_ABUSE_BLOW(py_tuple(vec![py_str("\\b(?:hit|hits|hitting|punch(?:es|ed|ing)?|beat(?:s|en|ing)?|kick(?:s|ed|ing)?|chok(?:es|ed|ing)|strangl(?:e|es|ed|ing)|slapp?(?:s|ed|ing)?|threw|thrown|hurt\\s+me|hurts\\s+me|bruis(?:e|es|ed)|blood|black\\s+eye|split\\s+lip|drag(?:s|ged|ging)\\s+me|would\\s+not\\s+let\\s+me|wont\\s+let\\s+me|won'?t\\s+let\\s+me|scared|terrified|afraid|rape|assault)\\b")]));
    s_ABUSE_ROOM_FAMILY(py_tuple(vec![py_str("\\b(?:my\\s+(?:son|daughter|child|children|kid|kids|baby|toddler|boy|girl|mum|mom|mam|mother|dad|father|sister|brother|parent|parents|friend|friends|mate|mates|flatmate|housemate|roommate|colleague|co-?worker|cousin|nephew|niece|aunt|uncle|nan|nana|gran|granny|grandma|grandad|grandpa|in-laws?|boss|manager|carer)\\b|they)\\b[^.?!]{0,40}\\b(?:corner(?:ed|s)?|trapped|trapping|traps|backed|backing|blocked|blocking|pinned|pinning)\\b")]));
    s_ABUSE_ROOM_CARD(py_tuple(vec![py_str("\\b(?:scared|afraid|frightened|terrified|panic(?:king|ked)?|crying|sobbing|shaking|shook|hurt\\s+me|hurts\\s+me|hit|hits|hitting|punched|punch|kick(?:ed|s)|slapped|slap|beaten|beat\\s+me|choked|choking|strangled|blood|bruis(?:e|es|ed)|black\\s+eye|split\\s+lip|raped|assaulted|worthless|useless|stupid|thick|fat|ugly|drunk|again|threat(?:ened|ens|s)?|would\\s+not\\s+let|wouldn'?t\\s+let|won'?t\\s+let|wont\\s+let|refus(?:ed|es))\\b"), py_str("\\b(?:let|allow)\\s+me\\s+(?:go|out|past|leave|through|by)\\b")]));
    s_ABUSE_FINE_ANIMAL(py_tuple(vec![py_str("\\b(?:[\\w']+\\s+)?(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit|parrot|hamster|gerbil|ferret|snake|lizard|bird|guinea\\s+pig)\\s+(?:bit|bite|bites|biting|nipped|snapped\\s+at|went\\s+for|attacked|scratched|clawed)\\s+(?:me\\b|us\\b|him\\b|her\\b|them\\b|my\\s+(?:hand|hands|leg|legs|arm|arms|face|finger|fingers|foot|feet|ankle|wrist|head|neck|lip|cheek|nose|ear|ear|thigh|shoulder))"), py_str("\\b(?:attacked|bit|bitten|nipped|savaged|mauled|scratched|clawed|stung)\\b[^.?!]{0,15}\\bby\\s+(?:(?:a|an|the|my|our|his|her|their)\\s+)?(?:[\\w'’]+\\s+){0,2}(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit|parrot|hamster|gerbil|ferret|snake|lizard|bird|guinea\\s+pig|bees?|wasps?|hornets?|midges?|gnats?|mosquito(?:e)?s?|flies|fly|insects?|bugs?)\\b")]));
    s_PLAYFUL_HIT(py_tuple(vec![py_str("\\b(?:my\\s+)?(?:son|daughter|child|kid|toddler|baby|nephew|niece|grandson|granddaughter)\\b[^.?!]{0,40}\\b(?:hit|hits|slapp?(?:ed|s)?|punch(?:ed|es)?|kicked?)\\b[^.?!]{0,40}\\b(?:play(?:ing)?|playfully|play-?fighting|mucking\\s+about|messing\\s+about|wrestl(?:e|ing)|tickl(?:e|ing)|accidentally|by\\s+accident|by\\s+mistake|didn'?t\\s+mean|didnt\\s+mean|not\\s+on\\s+purpose|tantrum)\\b"), py_str("\\b(?:play(?:ing)?|wrestl(?:e|ing)|tickl(?:e|ing)|mucking\\s+about|messing\\s+about)\\b[^.?!]{0,40}\\b(?:my\\s+)?(?:son|daughter|child|kid|toddler|baby)\\b[^.?!]{0,40}\\b(?:hit|slapp?(?:ed|s)?|punch(?:ed|es)?|kicked?)\\b"), py_str("\\b(?:my\\s+)?(?:son|daughter|child|kid|toddler|baby|nephew|niece|grandson|granddaughter)\\b[^.?!]{0,30}\\b(?:hit|hits|slapp?(?:ed|s)?|punch(?:ed|es)?|kicked?|head-?butt(?:ed|s)?)\\b[^.?!]{0,30}\\b(?:toy|ball|pillow|block|blocks|stick|book|cup|spoon|remote|brick|lego)\\b"), py_str("\\bh(?:it|its|itting)\\s+my\\s+(?:hand|arm|finger)s?\\s+(?:away|off|aside|back)\\b")]));
    s_HIT_IN_FUN(py_tuple(vec![py_str("\\b(?:hit|hits|slapp?(?:ed|s)?|punch(?:ed|es)?|elbow(?:ed|s)?|kick(?:ed|s)?|shov(?:ed|es)|push(?:ed|es)?|tap(?:ped|s)?|nudg(?:ed|es)?|whack(?:ed|s)?)\\b[^.?!]{0,40}\\b(?:laugh(?:ing|ed)?|jok(?:e|ing|ed)|kidding|in\\s+fun|for\\s+fun|for\\s+a\\s+laugh|play(?:ing|ful)?|play-?fighting|fooling\\s+about|fooling\\s+around|messing\\s+about|mucking\\s+about|teasing)\\b"), py_str("\\b(?:play(?:ing)?|laughing|joking|joking\\s+around|messing\\s+about|fooling\\s+about|as\\s+a\\s+joke|in\\s+fun|for\\s+a\\s+laugh)\\b[^.?!]{0,40}\\b(?:hit|hits|slapp?(?:ed|s)?|punch(?:ed|es)?|elbow(?:ed|s)?|kick(?:ed|s)?|shov(?:ed|es)|push(?:ed|es)?)\\b")]));
    s_HIT_THE_LAUGH_IS_HIS(py_tuple(vec![py_str("\\b(?:and|then|but|so)\\s+(?:he|she|they|everyone|everybody|all\\s+of\\s+them|his\\s+mates|her\\s+mates|the\\s+others)\\s+(?:laughed|laughs|was\\s+laughing|were\\s+laughing|is\\s+laughing|are\\s+laughing|grinned|smiled)\\b"), py_str("\\b(?:and|then)\\s+(?:laughed|laughs|laughing)\\b"), py_str("\\b(?:laugh(?:ed|ing)?\\s+it\\s+off|laughed\\s+it\\s+away)\\b"), py_str("\\b(?:everyone|everybody|all\\s+his\\s+mates|all\\s+her\\s+mates)\\s+(?:laughed|was\\s+laughing|were\\s+laughing)\\b"), py_str("\\b(?:he|she|they)\\s+(?:was|were|is|are)\\s+(?:only\\s+|just\\s+|always\\s+)?(?:joking|kidding|laughing|mucking\\s+about|messing\\s+about|playing|play-?fighting)\\b"), py_str("\\b(?:said|says|saying|told|tells|telling|claimed|claims)\\b[^.?!]{0,25}\\b(?:he|she|they)\\s+(?:was|were|is)\\s+(?:only\\s+|just\\s+)?(?:joking|kidding)\\b"), py_str("\\b(?:he|she|they)\\s+(?:said|says|told\\s+me|tells\\s+me|keeps\\s+saying)\\b[^.?!]{0,20}\\b(?:joke|joking|fun|banter|laugh)\\b")]));
    s_PUSHED_AWAY_REPLY(py_tuple(vec![py_str("A hand pushed away usually is not a blow - but only you know how it landed. If it hurt, or it frightened you, tell me and I will take it seriously."), py_str("That sounds like a hand moved off rather than a hit. If there is more to it than that - it landed hard, or it is not the first time - say so and I will treat it as the real thing."), py_str("Fair enough, that is a hand being moved aside rather than somebody being hit. Was it just that, or did it feel like more? I would rather ask than assume.")]));
    s_PUSHED_AWAY_THEN_RESTRAINT(py_tuple(vec![py_str("\\b(?:fell|fall|fallen|tripped|slipped)\\b"), py_str("\\b(?:hit|banged|knocked|smashed|bumped)\\s+(?:my|the|his|her)\\s+(?:head|face|nose|arm|shoulder|back|knee|hip|hand|elbow|wall|door|frame|floor|radiator|table|corner|worktop|shelf|sideboard|stairs|banister|bed)\\b"), py_str("\\b(?:so|because|till|until)\\s+(?:i|he|she)\\s+(?:couldn'?t|could\\s+not|cant|wasn'?t\\s+able)\\b"), py_str("\\b(?:couldn'?t|could\\s+not|cant|wouldn'?t|would\\s+not)\\s+(?:leave|get\\s+(?:away|out)|go|move)\\b"), py_str("\\b(?:wouldn'?t|would\\s+not|won'?t)\\s+let\\s+me\\b"), py_str("\\b(?:trapped|held|pinned|dragged|strangled|choked|threatened|hurt|hurts|hurting|bleeding|blood|bruise[ds]?|sore|swollen|hard)\\b")]));
    s_PUSHED_AWAY_BUT_SOMETHING_ELSE(py_tuple(vec![py_str("\\b(?:pan|pans|hob|oven|stove|grill|kettle|iron|saucepan|frying|boiling|heat|fire|smoke|babies|baby|child|children|kid|kids|son|daughter|toddler|pregnant|hospital|ambulance|police|999|911|work|office|shift|meeting|game|match|film|tv|phone)\\b")]));
    s_PUSHED_AWAY(py_tuple(vec![py_str("\\b(?:push(?:es|ed|ing)?|shov(?:e|es|ed|ing)|slapp?(?:s|ed|ing)?|batt?(?:s|ed|ing)?)\\s+my\\s+(?:hand|hands|arm|arms|finger|fingers|shoulder|shoulders|face|head)\\s+(?:away|off|aside)\\b")]));
    s_RESTRAINT_BESIDE(py_tuple(vec![py_str("\\b(?:held|hold|holding|pinn(?:ed|ing)|dragg(?:ed|ing)|wrestl(?:ed|ing)|restrain(?:ed|ing)?|grabb(?:ed|ing)\\s+(?:me|my\\s+(?:arm|arms|wrist|wrists|throat|neck|hair|hand|hands|shoulders?|waist|leg|legs|body))|(?:shov(?:ed|ing)|push(?:ed|ing))\\s+me|lock(?:ed|ing)|block(?:ed|ing)|shout(?:ed|ing)|yell(?:ed|ing)|scream(?:ed|ing)|threaten(?:ed|ing)?|fell|falling|wouldn'?t\\s+let\\s+me|would\\s+not\\s+let\\s+me|wouldn'?t\\s+let\\s+go|strangl(?:ed|es|ing)|chok(?:ed|es|ing)|hard)\\b")]));
    s_BLOW_ON_ME(py_tuple(vec![py_str("\\b(?:hit|hits|hitting|slapp?(?:ed|s|ing)?|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|beat|beats|beating|struck|thump(?:ed|s|ing)?|whack(?:ed|s|ing)?|chok(?:ed|es|ing)|strangl(?:ed|es|ing))\\s+(?:me|us)\\b"), py_str("\\b(?:hit|hits|hitting|slapp?(?:ed|s|ing)?|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|beat|beats|beating|struck|thump(?:ed|s|ing)?|whack(?:ed|s|ing)?)\\s+my\\s+(?:face|head|back|arm|arms|leg|legs|ribs?|stomach|belly|jaw|eye|eyes|ear|ears|neck|throat|shoulders?|chest|side|hips?|thigh|mouth|nose|teeth|wrist|knee)\\b"), py_str("\\b(?:hit|hits|hitting|slapp?(?:ed|s|ing)?|punch(?:ed|es|ing)?|kick(?:ed|s|ing)?|beat|beats|beating|struck|thump(?:ed|s|ing)?|whack(?:ed|s|ing)?)\\s+my\\s+(?:husband|wife|partner|boyfriend|girlfriend|son|daughter|child|kid|baby|mum|mom|mother|dad|father|brother|sister|friend|mate)\\b")]));
    s_A_GROWN_AGE(stdlib_call("re", "compile", &[py_str("\\b(?:teen(?:ager|aged)?|adolescen\\w+|grown\\s+up|(?:five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20)\\s*(?:years?|yrs?)\\s*(?:old)?\\b|(?:hes|shes|theyre|he'?s|she'?s|they'?re|he\\s+is|she\\s+is|they\\s+are|aged?)\\s+(?:five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20)\\b(?!\\s*months?))"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SMALL_CHILD_LASH_REPLIES(py_tuple(vec![py_str("That is what a small child does when the feeling is too big for the words, and it still hurts when it lands. Say 'no - that hurts' once, in the same words each time, put them down for a minute, then give them the words: 'you wanted that toy'."), py_str("Nothing is wrong with them, or with you - hitting and biting are among the last things a small child drops. The same short reply every time is what teaches it: 'no, that hurts', a minute of no attention, then back to normal."), py_str("It stings, and it is tiring, and it is the age rather than you: a small child has no other way to say the feeling yet. Catching it just before helps most - tired, hungry, or in a crowd - and one flat line when it lands, 'no - that hurts', with no big reaction to make it a game.")]));
    s_A_REAL_DANGER(py_tuple(vec![py_str("\\b(?:kill(?:ed|s|ing)?|murder\\w*|knife|knives|gun|weapon|blade|strangl\\w*|chok(?:e|ed|ing)|throat|threaten\\w*|scared|terrified|afraid|frightened|drink(?:s|ing)?|drunk|beers?|pub|pints?|wine|whisky|vodka|flinch\\w*|panic\\w*|walk(?:ing)?\\s+on\\s+eggshells|every\\s+(?:single\\s+)?day|nightly|kill\\s+my\\s*self|suicide|the\\s+police|(?:won'?t|will\\s+not|wouldn'?t|would\\s+not|doesn'?t|does\\s+not|never)\\s+let(?:ting)?\\s+me\\s+(?:leave|out|go|see))\\b")]));
    s_ADULT_HIT(py_tuple(vec![py_str("\\b(?:my\\s+)?(?:husband|wife|partner|boyfriend|girlfriend|ex|fiance|fiancee|dad|mum|mom|father|mother|stepdad|stepmum|stepmom|carer|uncle|aunt)\\b[^.?!]{0,30}\\b(?:hit|hits|slapp?(?:ed|s)?|punch(?:ed|es)?|kicked?|beat|beats|struck|push(?:ed|es|ing)?|shov(?:ed|es|ing))\\b")]));
    s_PERSON_AS_ATTACKER(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &crate::gen::modules::safety_phrases::g_ABUSER()), &py_str(")\\b(?:(?!\\b(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit|parrot|hamster|gerbil|ferret|snake|lizard|bird|guinea\\s+pig|bees?|wasps?|hornets?|midges?|gnats?|mosquito(?:e)?s?|flies|insects?)\\b)[^.?!]){0,30}\\b")), &crate::gen::modules::safety_phrases::g_ABUSE_VERB()), &py_str("\\s+me\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_SUBJECT(py_tuple(vec![py_str("\\b(?:the\\s+)?(?:death|dying|loss|losing|lost|passing|funeral|burial|cremation|news|diagnosis|divorce|split|separation|breakup|break-up|illness|cancer)\\s+of\\s+(?:my|our|the|his|her|their)\\s+(?:\\w+\\s+)?\\w+"), py_str("\\b(?:my|his|her|their|our)\\s+\\w+(?:'s|s')?\\s+(?:death|dying|died|dead|departure|loss|passing|funeral|diagnosis|illness|cancer)\\b"), py_str("\\b(?:losing|lost|lose)\\s+(?:my|our|the|his|her)\\s+\\w+"), py_str("\\b(?:the\\s+(?:day|week|year|month|news|thought|idea|fact)\\b[^.?!]{0,30}?\\b(?:died|dying|death|passed|gone|left\\s+us)\\b)"), py_str("\\b(?:my|our|his|her|their)\\s+(?:\\w+\\s+)?\\w+(?:'s|s')?\\s+(?:passed\\s+(?:away|on)|left\\s+(?:me|us)|walked\\s+out|moved\\s+out|went\\s+away|is\\s+gone|has\\s+gone|died|has\\s+died)")]));
    s__PERSON(py_str("(?:him|her|them|he|she|they|his|their|mum|mom|mam|mother|dad|father|stepdad|stepfather|nan|nana|gran|granny|grandma|grandad|granddad|grandpa|son|daughter|child|baby|brother|sister|wife|husband|hubby|partner|boyfriend|girlfriend|ex|friend|mate|flatmate|housemate|roommate|neighbou?r|uncle|aunt|cousin|in-?laws?|carer|staff|someone|somebody|anyone|anybody)"));
    s__AWAY_TAIL(py_str("on\\s+(?:the\\s+|a\\s+|this\\s+|my\\s+)?(?:motorway|dual\\s+carriageway|road|roads|stairs|staircase|steps|ladder|ladders|bike|bicycle|motorbike|scooter|horse|roof|scaffolding|ice|pavement|platform|escalator)\\b|up\\s+(?:a\\s+|the\\s+|this\\s+|my\\s+)?(?:ladder|ladders|roof|scaffolding|stairs)\\b|in\\s+(?:the\\s+)?(?:dark|car|van|traffic|bus|train|tube|town|city|sea|crowd)\\b|at\\s+(?:the\\s+)?(?:gym|pool|beach|match|festival|club)\\b|(?:driving|walking|cycling|running|jogging|lifting|swimming|climbing|hiking|travelling|commuting|shopping)\\b|around\\s+(?:dogs?|animals?|strangers|men|crowds?|traffic)\\b|with\\s+(?:this|my|the)\\s+(?:knee|back|hip|shoulder|leg|ankle|arm|wrist|neck|heart|health|balance|eyesight|grip)\\b"));
    s_FLAMES(py_tuple(vec![py_str("\\bflames?\\b[^.?!]{0,25}\\b(?:coming|pouring|shooting|bursting|licking|roaring|spreading|spread|out\\s+of|from|everywhere|on\\s+the\\s+(?:ceiling|wall|floor))\\b"), py_str("\\b(?:coming|pouring|shooting|bursting|licking|roaring)\\s+(?:out\\s+of|out|from|up)\\b[^.?!]{0,25}\\bflames?\\b"), py_str("\\b(?:is|are|was|were|went|go|goes|going|it'?s)\\s+(?:up\\s+)?in\\s+flames\\b"), py_str("\\b(?:see|saw|can\\s+see|i\\s+can\\s+see|there\\s+are|there\\s+is)\\s+(?:\\w+\\s+){0,3}flames?\\b"), py_str("\\bflames?\\s+(?:everywhere|all\\s+over|in\\s+the\\s+(?:room|kitchen|hall|house|flat))\\b")]));
    s_BURNT_FOOD(py_tuple(vec![py_str("\\b(?:burnt|burned|scorched|charred)\\s+(?:the\\s+|my\\s+|a\\s+)?(?:toast|toastie|dinner|tea|supper|lunch|bread|rice|pasta|chicken|meat|pie|pizza|beans|potatoes|chips|sauce|soup|cake|biscuits?|muffins?|pastry|fish|veg|vegetables|garlic|onions?|bacon|sausages?|pancakes?|porridge|toastie|crumpets?|pan|pot|wok|saucepan|cooker|food|roast|stew|curry|casserole|lasagne|lasagna|toastie)\\b"), py_str("\\b(?:the|my)\\s+(?:toast|dinner|tea|bread|rice|chicken|meat|pie|pizza|sauce|soup|cake|garlic|bacon|sausages?|porridge)\\s+(?:is|was|has|'s|got)\\s+(?:burnt|burned|charred|scorched|smoking)\\b"), py_str("\\bi\\s+(?:burnt|burned|scorched|charred)\\s+(?:it|them|something)\\b"), py_str("\\b(?:let|left)\\s+(?:the|my)\\s+(?:toast|dinner|pan|sauce|rice|chicken|pizza|porridge)\\b"), py_str("\\b(?:smoke|smoking|smoking\\s+out)\\s+(?:from|out\\s+of)\\s+(?:the|my)\\s+(?:toast|toaster|oven|cooker|grill|hob|pan)\\b")]));
    s_FIRE_OUT(py_tuple(vec![py_str("\\b(?:on|caught)\\s+fire\\b"), py_str("\\bflames?\\b"), py_str("\\b(?:alight|ablaze|out\\s+of\\s+control|spreading|spread)\\b"), py_str("\\bevery[\\s-]?where\\b"), py_str("\\ball\\s+over\\b"), py_str("\\bcan'?t\\s+see\\b"), py_str("\\b(?:chip\\s+pan|grease|deep\\s+fat|fryer)\\b[^.?!]{0,20}\\b(?:fire|flames?|alight)\\b")]));
    s_FIRE_ELSEWHERE(py_tuple(vec![py_str("\\b(?:on|caught)\\s+fire\\b"), py_str("\\bsmok\\w*\\b"), py_str("\\b(?:everywhere|spreading|spread|alight|ablaze|out\\s+of\\s+control)\\b"), py_str("\\b(?:ceiling|walls?|curtains?|sofa|settee|couch|mattress)\\b"), py_str("\\b(?:alarm|trapped|can'?t\\s+get\\s+out|cannot\\s+get\\s+out)\\b"), py_str("\\b(?:house|flat|kitchen|room|living\\s+room|hall|hallway|landing|building|car|garage|shed|chimney)\\b[^.?!]{0,15}\\b(?:fire|burning|flames?)\\b")]));
    s_GARMENT(py_str("\\b(?:coat|jacket|shirt|t-?shirt|jumper|sweater|hoodie|cardigan|fleece|top|dress|skirt|trousers|pants|jeans|pyjamas?|pj'?s|dressing\\s+gown|gown|robe|scarf|apron|overalls?|sleeve|sleeves|tie|clothes|hair|trouser\\s+leg|gloves?|mittens?|socks?|shorts|leggings|vest|sleepsuit|onesie|babygro|baby\\s*gro|romper|nappy|nightie|nightdress|dungarees|blouse|waistcoat|anorak|raincoat|mac|frock)\\b"));
    s_PAN_FIRE(py_tuple(vec![py_str("\\b(?:chip\\s*pan|chip\\s*pot|frying\\s+pan|pan|wok|deep\\s*fat|fryer)\\b[^.?!]{0,20}\\b(?:is\\s+|was\\s+|'s\\s+)?(?:on\\s+fire|alight|ablaze|caught\\s+fire)\\b"), py_str("\\b(?:oil|fat|grease)\\b[^.?!]{0,25}\\b(?:pan|wok|fryer)\\b[^.?!]{0,15}\\b(?:on\\s+fire|alight|caught\\s+fire)\\b"), py_str("\\b(?:pan|wok|fryer)\\b[^.?!]{0,20}\\b(?:oil|fat|grease)\\b[^.?!]{0,15}\\b(?:on\\s+fire|alight|caught\\s+fire)\\b"), py_str("\\b(?:chip\\s*pan|chip\\s*pot|frying\\s+pan|pan|wok|deep\\s*fat|fryer)\\b[^.?!]{0,20}\\b(?:caught|catches|took|takes|went\\s+up\\s+in)\\s+(?:the\\s+|a\\s+)?flames?\\b"), py_str("\\b(?:chip\\s*pan|chip\\s*pot|frying\\s+pan|pan|wok|deep\\s*fat|fryer)\\b[^.?!]{0,10}\\b(?:is|was|were|are)\\s+(?:in\\s+flames|blazing)\\b")]));
    s_VEHICLE_FIRE(py_tuple(vec![py_str("\\b(?:my|our|the)\\s+(?:car|van|motorbike|moped|scooter|lorry|truck|caravan|camper|engine|bonnet|dashboard)\\b[^.?!]{0,30}\\b(?:is\\s+|'?s\\s+|are\\s+|was\\s+|were\\s+)?(?:on\\s+fire|caught\\s+fire|alight|ablaze|in\\s+flames)\\b"), py_str("\\b(?:bonnet|dashboard)\\b[^.?!]{0,20}\\b(?:is\\s+|'?s\\s+|was\\s+)?burning\\b"), py_str("\\b(?:smoke|flames?)\\b[^.?!]{0,25}\\b(?:out\\s+of|from(?:\\s+under)?|under)\\s+(?:the\\s+|my\\s+|our\\s+)?(?:bonnet|engine\\s+bay|engine|dashboard|car|van)\\b"), py_str("\\b(?:bonnet|engine\\s+bay)\\b[^.?!]{0,20}\\bsmok\\w*\\b"), py_str("\\b(?:smok\\w*|flames?)\\b[^.?!]{0,15}\\b(?:in|into|inside|filling|through)\\s+(?:the\\s+|my\\s+|our\\s+)?(?:car|van|camper|caravan|cabin)\\b"), py_str("\\b(?:cabin|car|van)\\b[^.?!]{0,15}\\bfull\\s+of\\s+smoke\\b"), py_str("\\bexhaust\\b[^.?!]{0,30}\\b(?:flames?|fire|alight|ablaze)\\b"), py_str("\\b(?:flames?|fire)\\b[^.?!]{0,30}\\bexhaust\\b"), py_str("\\bthe\\s+exhaust\\b[^.?!]{0,10}\\bis\\s+burning\\b"), py_str("\\bsmell\\w*\\s+(?:of\\s+)?burning\\b[^.?!]{0,25}\\b(?:bonnet|engine(?:\\s+(?:bay|compartment))?|dashboard)\\b"), py_str("\\b(?:bonnet|engine\\s+bay|dashboard)\\b[^.?!]{0,20}\\bsmell\\w*\\b[^.?!]{0,10}\\bburn\\w*\\b"), py_str("\\b(?:my|our|the)\\s+(?:car|van|motorbike|moped|scooter|lorry|truck|caravan|camper)\\b(?![^.?!]{0,25}\\b(?:oil|fuel|petrol|diesel)\\b)[^.?!]{0,20}\\bburning\\b"), py_str("(?=.*\\bburn\\w*\\b)(?=.*\\bsmok\\w*\\b)(?=.*\\b(?:engine|bonnet|dashboard|car|van|exhaust)\\b)(?!.*\\b(?:oil|fuel|petrol|diesel|calories)\\b)")]));
    s_VEHICLE_EXHAUST(py_tuple(vec![py_str("\\b(?:smok\\w*|steam|fumes?)\\b[^.?!]{0,25}\\b(?:from|out\\s+of|out\\s+the)\\s+(?:the\\s+|my\\s+|our\\s+)?exhaust\\b"), py_str("\\bexhaust\\b[^.?!]{0,25}\\b(?:smok\\w*|fumes?)\\b"), py_str("\\b(?:burning|using|losing)\\s+(?:a\\s+bit\\s+of\\s+|some\\s+)?(?:oil|fuel)\\b[^.?!]{0,25}\\b(?:smoking|smoky|smokes)\\b")]));
    s_VEHICLE_BURNING_SMELL(py_tuple(vec![py_str("\\b(?:smell\\w*|smells?|smelling|stink\\w*|odou?r)\\b[^.?!]{0,12}\\b(?:of|like|off)\\s+(?:a\\s+)?burn\\w*\\b"), py_str("\\bburn\\w*\\s+(?:smell|odou?r|stink)\\b"), py_str("\\b(?:a\\s+)?burning\\s+(?:smell|odou?r)\\b"), py_str("\\bsmoke\\s+smell\\b"), py_str("\\b(?:smell\\w*|stink\\w*|odou?r)\\b[^.?!]{0,12}\\b(?:of|like|off)\\s+smoke\\b"), py_str("\\b(?:smoky|smokey)\\s+smell\\b")]));
    s_VEHICLE_BURNING_SMELL_HARD(py_tuple(vec![py_str("\\b(?:on\\s+fire|caught\\s+fire|alight|ablaze|in\\s+flames|flames?|bonnet|engine\\s+bay|dashboard|inside\\s+the\\s+car|coming\\s+into\\s+the\\s+car)\\b"), py_str("\\bthere\\s+(?:is|was|were|are)\\s+(?:a\\s+lot\\s+of\\s+|loads\\s+of\\s+|thick\\s+)?smoke\\b"), py_str("\\bsmoke\\s+(?:coming|pouring|billowing|filling|everywhere|belching|pumping)\\b"), py_str("\\bfull\\s+of\\s+smoke\\b")]));
    s_VEHICLE_FIRE_OVER(py_tuple(vec![py_str("(?=.*\\b(?:there\\s+was|there\\s+were|smoke\\s+came|i\\s+saw|i\\s+smelt|i\\s+smelled|it\\s+was|we\\s+had|caught\\s+fire|had\\s+a\\s+(?:small\\s+)?fire)\\b)(?=.*\\b(?:fixed|put\\s+(?:it\\s+)?out|sorted|fine\\s+now|ok\\s+now|seems?\\s+(?:fine|ok|alright|to\\s+be\\s+(?:fine|ok))|all\\s+(?:fine|ok)|no\\s+(?:damage|problem|harm))\\b)")]));
    s_NEAR_FIRE(py_tuple(vec![py_str("\\b(?:nearly|almost|close\\s+to|about\\s+to)\\b[^.?!]{0,40}\\b(?:fire|fires|flames?|alight|ablaze|blazing|burning|burn|burnt|burned|catch|caught|catchin(?:g)?|smoke|smoking|smouldering|smoldering|torched?|caught\\s+light|took\\s+light|went\\s+up)\\b")]));
    s_FIRE_DRILL(py_tuple(vec![py_str("\\bfire\\s+drill\\b"), py_str("\\bfalse\\s+alarm\\b")]));
    s_NEAR_THING_OVER(py_tuple(vec![py_str("\\b(?:it|he|she|they)\\s+(?:was|were|is|came|come|did|didn'?t|wouldn'?t|never|just)\\b[^.?!]{0,26}\\b(?:van|car|lorry|truck|bus|taxi|cab|bike|bicycle|driver|fella|man|woman|bloke|idiot|speed\\w*|fast|close|corner|junction|lights|crossing|pavement|road|kerb)\\b"), py_str("\\b(?:they|he|she|it)\\s+(?:didn'?t|did\\s+not|never)\\s+(?:stop|stop\\w*|slow|slow\\w*|look|even\\s+(?:stop\\w*|slow\\w*|look))\\b"), py_str("\\bi\\s+(?:had|needed)\\s+to\\s+(?:jump|run|dive|leap|step|swerve|pull|shout|grab|yank|scramble)\\b"), py_str("\\b(?:jumped|dived|leapt|leaped|ran|scrambled|swerved)\\s+(?:back|out\\s+of\\s+the\\s+way)\\b"), py_str("\\b(?:came|come|coming|appeared)\\s+out\\s+of\\s+nowhere\\b"), py_str("\\b(?:shook\\s+me\\s+up|really\\s+shook\\s+me|shaken\\s+up|hands?\\s+(?:are|were)\\s+shaking|still\\s+shaking|heart\\s+was\\s+racing)\\b"), py_str("\\bno\\s+(?:it|they|he|she)\\s+(?:didn'?t|did\\s+not)\\b"), py_str("\\b(?:nearly|almost)\\s+(?:had|hit|got)\\s+me\\b")]));
    s_NEAR_FIRE_OVER(py_tuple(vec![py_str("\\b(?:didnt|didn'?t|never|did\\s+not)\\s+(?:catch|caught|go\\s+up|went\\s+up|take|light|ignite|spread)\\b"), py_str("\\b(?:caught|got|put)\\s+it\\s+(?:in\\s+time|before|quick|early|out)\\b"), py_str("\\bno\\s+(?:real\\s+)?(?:damage|harm|fire)\\b"), py_str("\\bit\\s+(?:was|is)\\s+(?:fine|nothing|ok|alright|just\\s+smoke|only\\s+smoke)\\b")]));
    s_FIRE(add(&py_tuple(vec![py_str("\\b(?:house|flat|apartment|kitchen|home|building|shed|garage|car|roof|roofs?|attic|loft|chimney|chimney\\s*pot|porch|balcony|fence|hedge|caravan|motorhome|camper|tent|bungalow|cottage|semi|hall|hallway|landing|stairs?|stairwell|stairway|outhouse|lean-to|bedroom|bedrooms?|bathroom|bath\\s*room|bathrooms?|living\\s*room|living\\s*rooms?|sitting\\s*room|front\\s*room|back\\s*room|dining\\s*room|spare\\s*room|utility\\s*room|bed\\s*room|lounge|study|conservatory|extension|cellar|basement|room|rooms|ceiling|ceilings|wall|walls|floor|floors|floorboards?|skirting|door|doors|window|windows|window\\s*frame|banister|carpet|rug|wardrobe|cupboard|bed)\\s+(?:is\\s+|'s\\s+|are\\s+)?(?:on\\s+fire|burning|going\\s+up|alight|ablaze)\\b"), py_str("\\b(?:roof|attic|loft|chimney|chimney\\s*pot|porch|balcony|fence|hedge|caravan|motorhome|camper|tent|outhouse|shed|garage)\\b[^.?!]{0,20}\\bcaught\\s+(?:on\\s+)?fire\\b"), py_str("\\bthere'?s\\s+a\\s+fire\\b"), py_str("\\bfire\\s+in\\s+(?:the|my|our)\\s+(?:house|flat|kitchen|building|room|garage|car)\\b"), py_str("\\bsmoke\\s+(?:is\\s+)?(?:pouring|billowing|filling|coming\\s+(?:from|out))\\b"), py_str("\\b(?:can't|cannot|can\\s+not)\\s+get\\s+out\\b.{0,40}\\b(?:smoke|fire|flames?|burning|alarm|trapped)\\b"), py_str("\\b(?:trapped|blocked\\s+in|locked\\s+in)\\b.{0,30}\\b(?:can't|cannot)\\s+get\\s+out\\b"), py_str("\\b(?:can't|cannot)\\s+breathe\\b.{0,20}\\bsmoke\\b"), py_str("(?<!\\bi )(?<!\\bI )(?<!\\bhe )(?<!\\bshe )(?<!\\bwe )(?<!\\bthey )\\bsmoke\\s+(?:everywhere|in\\s+(?:the|my)\\s+(?:house|flat|room|kitchen))\\b"), py_str("\\b(?:my|the)\\s+(?:house|kitchen|flat)\\s+(?:is\\s+)?burning\\b"), py_str("\\bfire\\s+(?:has\\s+)?(?:started|spread)\\b"), py_str("\\b(?:smoke|fire)\\s+alarm\\b.{0,30}\\b(?:going\\s+off|sounding|blaring|wailing)\\b"), py_str("(?<!\\bi )(?<!\\bI )(?<!\\bhe )(?<!\\bshe )(?<!\\bwe )(?<!\\bthey )\\bsmoke\\s+(?:in|coming\\s+into|filling|pouring\\s+into)\\s+(?:the|my|our)\\b"), py_str("\\b(?:chip\\s+pan|pan|toaster|oven|grill|curtains?|sofa|cushion|bin|mattress|socket|plug|car|fridge|washing\\s+machine|tumble\\s*dryer|dryer|dishwasher|microwave|kettle|freezer|hoover|vacuum)\\b.{0,25}\\b(?:caught|on)\\s+fire\\b"), py_str("\\bflames?\\s+(?:in|on|inside|from|coming\\s+out\\s+of|out\\s+of)\\s+(?:the|my|our)\\s+(?:bin|bins|sofa|settee|couch|chair|car|curtains?|cushion|bed|mattress|wardrobe|cupboard|shed|garage|oven|microwave|kettle|pan|toaster|socket|plug|wires?|lamp|lights?|tv|telly|television|banister|rug|carpet|door)\\b"), py_str("\\bthere'?s\\s+smoke\\b"), py_str("\\bsmoke\\b[^.?!]{0,30}\\b(?:filling|filled)\\s+(?:up\\s+)?(?:the|my|our)\\s+(?:house|flat|kitchen|room|living\\s+room|bedroom|hall|hallway|landing|building|car|garage|shed)\\b"), py_str("\\b(?:the|my|our)\\s+(?:house|flat|kitchen|room|living\\s+room|bedroom|hall|hallway|landing|building|car|garage|shed)\\b[^.?!]{0,15}\\b(?:filling\\s+(?:up\\s+)?with\\s+smoke|full\\s+of\\s+smoke)\\b"), py_str("\\bsmoke\\s+(?:is\\s+|was\\s+)?(?:coming|pouring|seeping|drifting|billowing)\\s+(?:under|through|round|around)\\s+(?:the|a|my|our)\\s+(?:door|doors|floor\\s?boards?|floor|ceiling|wall|walls)\\b"), py_str("(?<!\\bi )(?<!\\bI )(?<!\\bhe )(?<!\\bshe )(?<!\\bwe )(?<!\\bthey )\\bsmoke\\s+(?:on|in)\\s+(?:the|my|our)\\s+(?:stairs|stairwell|landing|hall|hallway|corridor)\\b")]), &g_FLAMES()));
    s__SMOKE_THING(py_str("(?:oven|hob|grill|toaster|kettle|microwave|air\\s?fryer|washing\\s+machine|washer|tumble\\s+dryer|dryer|dishwasher|fridge|freezer|hoover|vacuum|laptop|computer|phone|mobile|charger|plug|socket|tv|telly|television|iron|hair\\s?dryer|printer|boiler|radiator|heater|extractor|fuse\\s?box)"));
    s_APPLIANCE_SMOKE(py_tuple(vec![add(&add(&py_str("\\b"), &g__SMOKE_THING()), &py_str("\\b[^.?!]{0,25}\\bsmok\\w*")), add(&py_str("\\bsmok\\w*\\s+(?:is\\s+)?(?:coming|pouring|billowing|rising|streaming)\\s+(?:out\\s+)?(?:of|from)\\s+(?:the|my|our|your)?\\s*(?:\\w+\\s+){0,3}"), &g__SMOKE_THING()), add(&py_str("\\bsmoke\\s+(?:out\\s+)?(?:of|from)\\s+(?:the|my|our|your)?\\s*(?:\\w+\\s+){0,3}"), &g__SMOKE_THING())]));
    s_FIRE_SCALE(py_tuple(vec![py_str("\\b(?:flames?|alight|ablaze|caught\\s+fire|on\\s+fire|set\\s+alight)\\b"), py_str("\\bsmoke\\s+(?:everywhere|filling|pouring\\s+into|coming\\s+into|billowing)\\b"), py_str("\\bsmoke\\s+(?:is\\s+|was\\s+)?(?:pouring|billowing|belching|roaring)\\b"), py_str("\\b(?:kitchen|room|living\\s+room|hall|hallway|house|flat|landing|stairs|stairwell|bedroom)\\b[^.?!]{0,15}\\bfilling\\s+(?:up\\s+)?with\\s+smoke\\b"), py_str("\\bsmoke\\s+(?:in|on)\\s+(?:the|my|our)\\s+(?:hall|hallway|stairs|stairwell|landing|bedroom|room|house|flat|kitchen|toilet|bathroom)\\b"), py_str("\\b(?:whole|entire)\\s+(?:house|flat|kitchen|room|building)\\b"), py_str("\\b(?:can'?t|cannot)\\s+get\\s+out\\b"), py_str("\\b(?:spread\\w*|took\\s+hold|getting\\s+(?:bigger|worse))\\b"), py_str("\\b(?:smoke|fire)\\s+alarm\\b[^.?!]{0,30}\\b(?:going\\s+off|sounding|blaring)[^.?!]{0,40}\\bsmoke\\b")]));
    s_ALARM_NUISANCE(py_tuple(vec![py_str("\\b(?:smoke|fire)\\s+(?:alarm|detector)\\b[^.?!]{0,40}\\b(?:goes\\s+off|going\\s+off|sounds|sounding|beeps|beeping|chirp\\w*|wails?|keeps\\s+going\\s+off|wont\\s+stop|won'?t\\s+stop|stop\\s+it)\\b[^.?!]{0,40}\\b(?:when|cooking|toast|toasting|grill|grilling|oven|hob|fry|frying|roast|roasting|bake|baking|shower|steam|kitchen|cooker)\\b")]));
    s__SMELLY_THING(py_str("coat|jacket|clothes|clothing|jumper|shirt|top|dress|trousers|jeans|skirt|hoodie|fleece|cardigan|suit|uniform|hat|scarf|gloves|shoes|trainers|bag|rucksack|hair|curtains?|cushions?|sofa|settee|couch|mattress|bedding|duvet|pillow|blanket|rug|carpet|car|van|tent|sleeping\\s+bag|washing|laundry|sheets?"));
    s_SMOKE_IS_A_SMELL(py_tuple(vec![py_str("\\b(?:smell|smells|smelly|stinks?|stinking|reek\\w*|whiff|odour|odor)\\b[^.?!]{0,40}\\bsmoke\\b"), py_str("\\bsmoke\\s+(?:smell|stain\\w*)\\b"), py_str("\\b(?:get|getting|got|remove|removing|clear|take|taking)\\b[^.?!]{0,30}\\bsmoke\\b[^.?!]{0,20}\\b(?:out|off)\\b"), py_str("\\bsmoke\\s+out\\s+of\\b")]));
    s_BATTERY_FIRE(py_tuple(vec![py_str("\\b(?:phone|laptop|tablet|ipad|battery|batteries|power\\s?bank|e-?bike|e-?scooter|hoverboard|charger|drone)\\b[^.?!]{0,20}\\b(?:is\\s+|'s\\s+|are\\s+)?(?:on\\s+fire|caught\\s+fire|smoking|sparking|melting|burning(?!\\s+(?:up|hot|my|his|her|their|a|the))|burnt)\\b"), py_str("\\b(?:battery|batteries)\\s+(?:is\\s+|are\\s+)?(?:on\\s+fire|caught\\s+fire|smoking|burning(?!\\s+(?:up|hot|my|his|her|their|a|the)))\\b")]));
    s_DEVICE_BURNING(py_tuple(vec![py_str("\\b(?:phone|laptop|tablet|ipad|battery|batteries|power\\s?bank|e-?bike|e-?scooter|hoverboard|charger|drone)\\b[^.?!]{0,30}\\b(?:flames?|alight|ablaze|burning|burnt|caught\\s+fire|on\\s+fire)\\b"), py_str("\\bflames?\\b[^.?!]{0,25}\\b(?:out\\s+of|from)\\b[^.?!]{0,10}\\b(?:phone|laptop|tablet|ipad|battery|power\\s?bank|charger)\\b")]));
    s_MEDICINE(py_str("(?:calpol|nurofen|benylin|infant\\s+paracetamol|children'?s\\s+(?:paracetamol|ibuprofen)|junior\\s+ibuprofen|cough\\s+(?:medicine|syrup)|cold\\s+medicine|antibiotics?|pills|tablets|medicine|medicines|meds|paracetamol|ibuprofen|painkillers|antidepressants|insulin|eye\\s+drops|vitamins?|iron\\s+tablets|sleeping\\s+pills)"));
    s_DOSE_AMOUNT(py_str("\\b(?:too\\s+many|too\\s+much|double|twice|extra)\\s+(?:doses?|spoonfuls?|mouthfuls?|syringes?)\\b|\\b(?:twice|double|three\\s+times)\\s+(?:the|her|his|their|your)\\s+dose\\b|\\b(?:two|three|four|several)\\s+(?:doses?|spoonfuls?)\\b[^.?!]{0,15}\\b(?:at\\s+once|together|at\\s+the\\s+same\\s+time|in\\s+(?:an|one|1|a\\s+single)\\s+hour|within\\s+an\\s+hour|in\\s+\\d+\\s+minutes|back\\s+to\\s+back)\\b"));
    s_DOSE_COURSE(py_tuple(vec![py_str("\\b(?:doses?|spoonfuls?|syringes?)\\b[^.?!]{0,20}\\b(?:apart|(?:\\d+|two|three|four|five|six|several)\\s+hours?\\s+(?:apart|later|between|gap)|every\\s+(?:\\d+|two|three|four|six)\\s+hours?)\\b"), py_str("\\bevery\\s+(?:\\d+|two|three|four|six)\\s+hours?\\b[^.?!]{0,20}\\b(?:doses?|spoonfuls?)\\b"), py_str("\\b(?:two|three|four|five|six)\\s+(?:doses?|spoonfuls?)\\b[^.?!]{0,20}\\b(?:today|so\\s+far|since\\s+(?:this\\s+morning|last\\s+night|yesterday)|this\\s+(?:morning|afternoon|evening|week)|in\\s+24\\s+hours|over\\s+the\\s+day)\\b")]));
    s_DOSE_COURSE_KEEP(py_tuple(vec![py_str("\\btoo\\s+many\\b"), py_str("\\boverdos\\w*"), py_str("\\bhandful\\b"), py_str("\\bwhole\\s+(?:packet|pack|box|bottle)\\b"), py_str("\\ball\\s+(?:of\\s+)?(?:my|his|her|their)\\b"), py_str("\\b(?:sleepy|drowsy|floppy|limp|unrousable|hard\\s+to\\s+wake|wont\\s+wake)\\b"), py_str("\\bvomit\\w*"), py_str("\\bbeing\\s+sick\\b"), py_str("\\bthrowing\\s+up\\b"), py_str("\\bnot\\s+himself\\b"), py_str("\\bnot\\s+herself\\b")]));
    s_DOSE_ASKED(py_tuple(vec![py_str("\\b(?:can|could|should|shall|may|might)\\s+(?:i|we)\\s+(?:give|dose|add|top\\s+up|double)\\b"), py_str("\\bis\\s+it\\s+(?:ok|okay|alright|safe|fine)\\s+to\\s+(?:give|dose|add|top\\s+up)\\b"), py_str("\\bwould\\s+(?:it|that)\\s+be\\s+(?:ok|okay|alright|safe|fine)\\b"), py_str("\\bis\\s+(?:it|that|this)\\s+too\\s+(?:many|much)\\b"), py_str("\\bis\\s+(?:it|that|this)\\s+(?:ok|okay|alright|fine)\\s+(?:for|to\\s+give)\\b"), py_str("\\bhow\\s+(?:many|much)\\b[^.?!]{0,20}\\b(?:doses?|calpol|nurofen|paracetamol|ibuprofen)\\b")]));
    s_OVERDOSE_OF_A_TREAT(py_tuple(vec![py_str("\\boverdos(?:e|ed|ing)\\s+(?:on\\s+)?(?:(?:the|a|an|my|his|her|their)\\s+)?(?:coffee|caffeine|tea|chocolate|cake|sugar|sweets|crisps|biscuits|cheese|ice\\s+cream|tv|telly|netflix|box\\s+sets?|sleep|sun|sunshine|shopping|gossip|christmas\\s+music)\\b"), py_str("\\ban\\s+overdose\\s+of\\s+(?:coffee|caffeine|tea|chocolate|cake|sugar|sweets|crisps|biscuits|cheese|ice\\s+cream|tv|telly|netflix|box\\s+sets?|sleep|sun|sunshine|shopping|gossip|work|meetings?|emails?|paperwork|spreadsheets?|christmas\\s+music)\\b")]));
    s_MILD_SWALLOWED(py_tuple(vec![py_str("(?![^?]{0,70}\\b(?:child|kid|kids|son|daughter|baby|babies|toddler|boy|girl|grandson|granddaughter|niece|nephew)\\b)(?![^?]{0,70}\\b(?:bottle|bottles|whole|tube|tubes|packet|litre|litres|pint|glass\\s+full)\\b)\\b(?:i|ive|i've|im|i'm)\\b[^.?!]{0,20}\\b(?:swallowed|swallow|drank|drunk|drinking|licked|sucked|got|took|sipped|swigged|necked|glugged)\\b[^.?!]{0,15}\\b(?:a\\s+bit|a\\s+little|a\\s+small\\s+amount|a\\s+mouthful|a\\s+sip|one\\s+sip|one\\s+mouthful|a\\s+taste|a\\s+drop|a\\s+dab|a\\s+smear|a\\s+swig|some)\\b[^.?!]{0,25}\\b(?:mouthwash|mouth\\s+wash|toothpaste|shampoo|conditioner|shower\\s+gel|soap|body\\s+lotion|hand\\s+cream|sun\\s?cream|moisturiser|moisturizer|bubble\\s+bath|hand\\s+soap|bath\\s+soap)\\b"), py_str("\\b(?:mouthwash|mouth\\s+wash|toothpaste|shampoo|shower\\s+gel)\\b[^.?!]{0,15}\\b(?:got|went|sprayed|squirted)\\b[^.?!]{0,10}\\b(?:in|into)\\s+my\\s+mouth\\b")]));
    s_SOME_OF_IT(py_str("(?:some\\s+(?:of\\s+the\\s+)?|a\\s+little\\s+(?:bit\\s+)?(?:of\\s+)?|little\\s+(?:of\\s+)?|a\\s+(?:bit|drop|splash|swig|mouthful|sip|handful|piece|lick|spoonful|tablespoon|teaspoon|glass|cup|swallow|bottle|tube|packet|can|carton|jar)\\s+(?:of\\s+)?|a\\s+small\\s+amount\\s+(?:of\\s+)?|half\\s+a\\s+(?:glass|cup|bottle)\\s+(?:of\\s+)?|(?:the|my|his|her|their|our|these|those)\\s+)?"));
    s_MILD_PRODUCT_SMALL(py_tuple(vec![py_str("\\b(?:toothpaste|shampoo|conditioner|shower\\s+gel|soap|body\\s+lotion|hand\\s+cream|sun\\s?cream|moisturiser|moisturizer|bubble\\s+bath)\\b")]));
    s_TOOK_OUT_OF(py_tuple(vec![py_str("\\b(?:took|taken|taking|take|got|get|getting|fetched|fetch)\\b[^.?!]{0,25}\\b(?:out\\s+of|from|off|down\\s+from|out\\s+the)\\s+(?:the\\s+|a\\s+|my\\s+|his\\s+|her\\s+)?(?:car|cupboard|garage|shed|bathroom|kitchen|boot|cabinet|box|bag|shelf|drawer|bottle|tin|jar|bucket|machine|van|garden|work|locker|store|closet|cupboards|shelves)\\b")]));
    s_SWALLOWED_ORDINARY(py_tuple(vec![py_str("\\b(?:child|kid|kids|son|daughter|baby|babies|toddler|boy|girl|he|she|they|grandson|granddaughter|niece|nephew|my\\s+(?:son|daughter|child|kid|baby|toddler|boy|girl|grandson|granddaughter|niece|nephew|husband|wife|partner|friend|mum|mom|dad|nan|gran))\\b[^.?!]{0,25}\\b(?:swallowed|swallowing|ate|eaten|eating|drank|drunk)\\b[^.?!]{0,20}\\b(?:wax\\s+crayons?|crayons?|chalk(?!\\s*paint)|toothpaste|bars?\\s+of\\s+soap|soap|play\\s?dough|(?:blu|blue)\\s?tack?)\\b")]));
    s_GAS_SMELL(py_tuple(vec![py_str("\\b(?:smell|smells|smelling|smelled|smelt|whiff)\\s+(?:of\\s+|like\\s+)?(?:a\\s+)?(?:gas|gas\\s+leak)\\b"), py_str("\\b(?:can|could)\\s+(?:i|you|we|she|he|they)\\s+smell\\s+gas\\b"), py_str("\\bi\\s+(?:can\\s+)?smell\\s+gas\\b"), py_str("\\b(?:strong|funny|odd|strange|faint)\\s+(?:smell|whiff)\\s+of\\s+gas\\b"), py_str("\\b(?:gas|leak)\\s+smell\\b"), py_str("\\bgas\\s+(?:leak|leaking|leaked)\\b"), py_str("\\bleak(?:ing|s|ed)?\\s+(?:out\\s+)?(?:\\w+\\s+)?gas\\b"), py_str("\\bgas\\s+(?:is\\s+)?(?:leaking|escaping|coming\\s+(?:out|from)|seeping)\\b"), py_str("\\b(?:smell\\w*|whiff\\w*)\\b[^.?!]{0,30}\\bgas\\b"), py_str("\\bgas\\b[^.?!]{0,20}\\b(?:smell|stink|reek)\\w*\\b"), py_str("\\b(?:boiler|cooker|hob|heater|oven|fire|gas\\s+fire)\\b.{0,30}\\bsmell\\w*\\b.{0,30}\\b(?:gas|fumes?|rotten|sulphur|sulfur|eggs?|headach\\w*|dizzy|nauseous)\\b"), py_str("\\bsmell\\w*\\s+(?:of\\s+|like\\s+)?(?:rotten\\s+eggs?|sulphur|sulfur)\\b.{0,30}\\b(?:boiler|cooker|hob|heater|gas)\\b")]));
    s_BLEACH_MIX(py_tuple(vec![py_str("\\A(?=.*\\bbleach\\w*\\b)(?=.*\\b(?:vinegar|ammonia|acid|acidic|toilet\\s+cleaner|cistern\\s+cleaner|descaler|de\\s?scaler|limescale|drain\\s+unblocker|oven\\s+cleaner|another\\s+cleaner|two\\s+cleaners|other\\s+cleaner|bleach\\s+tablets?)\\b)(?!.*\\b(?:separately|separate|different\\s+days?|not\\s+together|never\\s+mix|washed\\s+it|rinsed\\s+it|in\\s+another\\s+room|one\\s+after\\s+the\\s+other)\\b).*"), py_str("\\A(?!.*\\b(?:tap|taps|water|waters|pool|swimming|drinking|shower|kettle|cold\\s+water)\\b).*?\\b(?:smell|smells|smelling|smelt|whiff|reek\\w*)\\b[^.?!]{0,25}\\bchlorine\\b"), py_str("\\bchlorine\\s+(?:gas|fumes?)\\b"), py_str("\\bbreathed\\s+in\\b[^.?!]{0,25}\\b(?:bleach|chlorine|cleaner|cleaning\\s+fumes?)\\b")]));
    s_BLEACH_MIX_Q(py_tuple(vec![py_str("\\b(?:can|could|should|may)\\s+(?:i|we|you)\\s+(?:mix|put|add|use)\\b[^.?!]{0,30}\\bbleach\\b(?=[^.?!]{0,30}\\b(?:vinegar|ammonia|acid|acidic|toilet\\s+cleaner|descaler|limescale)\\b)"), py_str("\\b(?:is\\s+it\\s+(?:ok|okay|safe|fine)|would\\s+it\\s+(?:be\\s+)?(?:ok|okay|safe|fine))\\b[^.?!]{0,30}\\b(?:mix|with)\\b[^.?!]{0,20}\\bbleach\\b"), py_str("\\bwhat\\s+happens\\s+(?:if|when)\\s+(?:i|we|you)\\s+mix\\b[^.?!]{0,30}\\bbleach\\b"), py_str("\\bbleach\\b[^.?!]{0,20}\\b(?:and|with)\\b[^.?!]{0,15}\\b(?:vinegar|ammonia|toilet\\s+cleaner|acid)\\b[^.?!]{0,20}\\b(?:together|at\\s+the\\s+same\\s+time|ok|okay|safe|fine)\\b")]));
    s_CO_GOING_OFF(py_tuple(vec![py_str("\\b(?:carbon\\s+monoxide|co)\\s+(?:alarm|detector|monitor)s?\\b.{0,30}\\b(?:beep\\w*|chirp\\w*|going\\s+off|going\\s+beep|went\\s+off|gone\\s+off|sounding|sounds|sounded|wailing|ringing)\\b"), py_str("\\b(?:beep\\w*|chirp\\w*|going\\s+off|went\\s+off|gone\\s+off|sounding|sounds|sounded|wailing|ringing)\\b.{0,40}\\b(?:carbon\\s+monoxide|co)\\s+(?:alarm|detector|monitor)s?\\b")]));
    s_ELECTRIC_BURN(py_tuple(vec![py_str("\\b(?:phone|mobile|charger|charging\\s+brick|laptop|tablet|ipad|battery|batteries|power\\s?bank|adapter|adaptor|plug|socket|extension\\s+lead|lead|cable|wire|wiring|fuse\\s?box|vape|e-?cig\\w*|hoverboard|scooter|drill|toothbrush|headphones?|earbuds?|console|charger\\s+port|charging\\s+port)\\b[^.?!]{0,45}\\b(?:smell\\w*|smelling|smoke|smoking|smoked|smoulder\\w*|melt(?:ing|ed|s)?|spark(?:ing|s|ed)?|scorch\\w*|charred|caught\\s+fire|catch(?:es|ing)?\\s+fire|on\\s+fire|flames?|bulging|bulged|puffed\\s+up|swollen)\\b"), py_str("\\b(?:smell\\w*|smelling)\\s+(?:of\\s+|like\\s+)?(?:burning|melting|plastic|acrid)\\b[^.?!]{0,40}\\b(?:phone|charger|plug|socket|lead|cable|wire|wiring|battery|laptop|tablet|adapter|adaptor|power\\s?bank)\\b"), py_str("\\b(?:plug|socket|extension\\s+lead|adapter|adaptor|charger)\\b.{0,30}\\b(?:is|are|'s|'re|was|were)\\s+(?:melting|melted|scorched|charred|blackened|buzzing|sparking)\\b"), py_str("\\bbatter(?:y|ies)\\b.{0,20}\\b(?:bulging|bulged|swollen|puffed\\s+up|hot\\s+to\\s+touch)\\b")]));
    s_BATTERY_BULGE(py_tuple(vec![py_str("\\b(?:phone|mobile|laptop|tablet|ipad|battery|batteries|power\\s?bank|vape|e-?cig\\w*|hoverboard|scooter|drone|toothbrush|headphones?|earbuds?|watch)\\b[^.?!]{0,25}\\b(?:bulg(?:ing|ed|es)|swollen|swelling|puffed\\s+up|puffing\\s+up)\\b"), py_str("\\b(?:bulg(?:ing|ed|es)|swollen|puffed\\s+up)\\b[^.?!]{0,15}\\b(?:battery|batteries)\\b")]));
    s_ELECTRIC_FINE(py_tuple(vec![py_str("\\b(?:gets?|getting|is|are)\\s+(?:quite\\s+|really\\s+|a\\s+bit\\s+|very\\s+)?(?:warm|hot)\\b(?!.{0,20}\\b(?:smell\\w*|smoke|smoking|melt\\w*|spark\\w*|bulging|swollen)\\b)"), py_str("\\b(?:frayed|worn|broken|cracked|tangled)\\s+(?:lead|cable|wire)\\b(?!.{0,25}\\b(?:smell\\w*|smoke|smoking|melt\\w*|spark\\w*)\\b)"), py_str("\\b(?:battery|charge)\\s+(?:life|lasts?|lasted|drains?|dropping|dies|died|percentage)\\b"), py_str("\\bhow\\s+(?:do|can|should)\\s+(?:i|we|you)\\s+(?:stop|keep|make|get)\\b.{0,30}\\b(?:phone|battery|charger|laptop)\\b.{0,20}\\b(?:cool|cooler|warm|hot|charged\\s+up)\\b"), py_str("\\b(?:recycle|recycling|dispose|disposal|take|bin)\\b.{0,30}\\b(?:battery|batteries|phone)\\b")]));
    s_FORAGED_EATEN(py_tuple(vec![py_str("\\b(?:swallowed|swallowing|drank|drunk|eaten|ate|eating|licked|sucked|chewed|nibbled|tasted|bit\\s+into|put\\s+(?:it\\s+)?in\\s+(?:his|her|their|my)\\s+mouth)\\b[^.?!]{0,30}\\b(?:wild\\s+mushrooms?|toadstools?|puffballs?|liberty\\s+caps?|magic\\s+mushrooms?|foraged|foraging|fungus|fungi)\\b"), py_str("\\b(?:swallowed|swallowing|eaten|ate|eating|licked|sucked|chewed|nibbled|tasted|bit\\s+into)\\b[^.?!]{0,30}\\b(?:berries|berry|leaf|leaves|plants?|seeds?|nuts?)\\b[^.?!]{0,25}\\b(?:from|off|in|out\\s+of)\\s+(?:the\\s+|a\\s+|our\\s+|my\\s+)?(?:garden|hedge|bush|bushes|tree|field|woods?|woodland|park|wall|plants?|border|allotment|lane|path|verge)\\b"), py_str("\\b(?:my|our)\\s+(?:son|daughter|child|kid|baby|toddler)\\b[^.?!]{0,25}\\b(?:ate|eaten|eating|swallowed|nibbled|chewed|sucked)\\b[^.?!]{0,25}\\b(?:flowers?|plants?|leaf|leaves|berries|berry|seeds?)\\b[^.?!]{0,25}\\b(?:from|off|in|outside)\\s+(?:the\\s+|a\\s+)?(?:garden|hedge|bush|park|woods?|field|ground|outside|path|allotment)\\b"), py_str("\\b(?:picked|found|gathered|brought\\s+home|brought\\s+me|handed\\s+me)\\b[^.?!]{0,25}\\b(?:mushrooms?|toadstools?|berries|berry)\\b[^.?!]{0,20}\\b(?:and|then)\\b[^.?!]{0,15}\\b(?:ate|eaten|eating|swallowed|nibbled|chewed)\\b")]));
    s_FORAGED_FINE(py_tuple(vec![py_str("\\b(?:risotto|soup|pie|pizza|pasta|curry|sauce|omelette|stir\\s?fry|salad|dish|recipe|toast|restaurant|menu|cake|jam|pesto|smoothie|juice|wine|ketchup|garlic|elderflower)\\b"), py_str("\\b(?:picked\\s+up|bought|shop|supermarket|market|growing|grow|planted|seeds?\\s+to|want\\s+to\\s+grow|identify\\w*|what\\s+are\\s+these|which\\s+mushrooms?)\\b"), py_str("\\b(?:cooked|cook|fried|roast\\w*|boiled|baked|washed|peeled)\\b")]));
    s_CO_LEAK(py_tuple(vec![py_str("\\bcarbon\\s+monoxide\\b.{0,40}\\b(?:leak\\w*|poison\\w*|inhaled|breath\\w*|fumes?|giving\\s+(?:off|out)|emitting|exposure)\\b"), py_str("\\b(?:leak\\w*|giving\\s+(?:off|out)|emitting|inhaled|poison\\w*)\\b.{0,40}\\bcarbon\\s+monoxide\\b"), py_str("\\bthere'?s\\s+(?:some\\s+|a\\s+lot\\s+of\\s+)?(?:carbon\\s+monoxide|co)\\b"), py_str("\\bcarbon\\s+monoxide\\b\\s+(?:in|inside|all\\s+over|coming\\s+(?:in|through))\\s+(?:the|my|our)\\b")]));
    s_CO_SIGNS(py_tuple(vec![py_str("\\b(?:boiler|flue|cooker|hob|gas\\s+fire|fireplace|wood\\s*burner|woodburner|back\\s+boiler|heater|oven)\\b.{0,50}\\bfum(?:e|es|ing|y)\\b"), py_str("\\bcarbon\\s+monoxide\\b.{0,60}\\b(?:dizzy|dizziness|headach\\w*|nauseous|nausea|sick|vomit\\w*|drowsy|sleepy|confused|faint\\w*|light.?headed|breathless)\\b"), py_str("\\b(?:dizzy|dizziness|headach\\w*|nauseous|nausea|sick|vomit\\w*|drowsy|sleepy|confused|faint\\w*|light.?headed|breathless)\\b.{0,60}\\bcarbon\\s+monoxide\\b")]));
    s_CO_TEST(py_tuple(vec![py_str("\\bhow\\s+(?:do|can|should)\\s+(?:i|we|you)\\s+(?:test|check|service|clean|reset|change\\s+the\\s+battery)\\b.{0,40}\\b(?:carbon\\s+monoxide|co)\\s+(?:alarm|detector|monitor)\\b"), py_str("\\b(?:carbon\\s+monoxide|co)\\s+(?:alarm|detector|monitor)\\b.{0,40}\\b(?:test|tested|testing|service|servicing|calibrat\\w+|checked)\\b.{0,40}\\?"), py_str("\\bhow\\s+often\\b.{0,30}\\b(?:carbon\\s+monoxide|co)\\s+(?:alarm|detector|monitor)\\b"), py_str("\\bwhere\\s+(?:should|do)\\s+(?:i|we|you)\\s+(?:put|place|fit|mount|install)\\b.{0,30}\\b(?:carbon\\s+monoxide|co)\\s+(?:alarm|detector|monitor)\\b")]));
    s__GASP_NOT_A_THIRST(py_str("air|breath|breathe|breathing|oxygen|help|doctor|nurse|ambulance|paramedic|paramedics|inhaler|nebuliser|nebulizer|ventilator|respirator|999|911|111|rescue|defibrillator|aed"));
    s__A_THIRST(add(&add(&py_str("(?:a|an|the|some|any|my|me|that|one)\\s+(?!(?:[\\w'-]+\\s+){0,1}(?:"), &g__GASP_NOT_A_THIRST()), &py_str(")\\b)(?:[\\w'-]+)")));
    s_GASP_FINE(py_tuple(vec![add(&py_str("\\bgasping\\s+(?:for|after|at)\\s+"), &g__A_THIRST()), py_str("\\bgasping\\s+(?:for|after)\\s+(?:a|an|the|some|any|my|that)\\s+(?:run|walk|jog|workout|gym|session|swim|cycle|ride|bike|spin|class|game|match|training|hill|stairs|flight\\s+of\\s+stairs|shopping|hoovering|dance|dancing|song|tune|race|marathon)\\b"), add(&py_str("\\bgasps?\\s+for\\s+"), &g__A_THIRST()), py_str("\\bgasping\\s+(?:on|up|coming\\s+up|going\\s+up|climbing)\\s+(?:the\\s+|a\\s+|my\\s+)?(?:stairs|steps|hill|slope|incline|road|bank|staircase)\\b"), py_str("\\b(?:i|we|he|she|they)\\s+(?:was|were|'m|'re|am|are)\\s+gasping\\s+after\\b"), py_str("\\bgasping\\s+after\\s+(?:that|the|a|my|this)\\b")]));
    s_GASP_RED(py_tuple(vec![py_str("\\bcan'?t\\s+breathe\\b"), py_str("\\bcannot\\s+breathe\\b"), py_str("\\bgasping\\s+for\\s+(?:air|breath|oxygen)\\b"), py_str("\\bnot\\s+breathing\\b"), py_str("\\bturning\\s+blue\\b"), py_str("\\b(?:collapsed|unconscious|passed\\s+out)\\b"), py_str("\\b(?:lips|face)\\s+(?:are\\s+|'re\\s+)?(?:blue|grey|gray)\\b"), py_str("\\bchoking\\b"), py_str("\\b(?:his|her|their)\\s+(?:chest|breathing)\\b")]));
    s_BREATH_PAST_OVER(py_tuple(vec![py_str("\\b(?:years?|months?|decades?)\\s+ago\\b"), py_str("\\bwhen\\s+(?:he|she|they|i)\\s+was\\s+(?:born|a\\s+baby|little|small|young|a\\s+(?:child|kid|toddler))\\b"), py_str("\\bas\\s+a\\s+(?:baby|child|kid|toddler|newborn)\\b"), py_str("\\bback\\s+then\\b"), py_str("\\blast\\s+(?:year|decade|winter|summer)\\b"), py_str("\\bin\\s+(?:the\\s+)?(?:19|20)\\d\\d\\b")]));
    s_CHEST_IN_A_HAND_TAIL(py_str("(?:\\s*[,.!?;:]|\\s*$|\\s+(?:and|so|but|then|too|now|again|because|with|when|as|while)\\b|\\s+(?:sweat\\w*|clammy|grey|gray|pale|breathless|short\\s+of\\s+breath|sick|dizzy|nause\\w*|faint\\w*|hurts?|hurting|pain\\w*|ache\\w*|tight\\w*|pressure|fell|fallen|collaps\\w*|went\\s+down)\\b)"));
    s_CHEST_IN_A_HAND(py_tuple(vec![add(&add(&py_str("\\A(?![^.!?]*\\b(?:laugh\\w*|giggl\\w*|jok\\w*|hysteric\\w*|sides?|meme|tiktok)\\b)(?=[^.!?]*\\b(?:clutch\\w*|gripp?\\w*|grabb?\\w*|holding|holds?|held)\\s+(?:at\\s+)?(?:his|her|their)\\s+chest\\b"), &g_CHEST_IN_A_HAND_TAIL()), &py_str(")")), add(&add(&py_str("\\A(?![^.!?]*\\b(?:laugh\\w*|giggl\\w*|jok\\w*|hysteric\\w*|sides?)\\b)(?=[^.!?]*\\b(?:clutch\\w*|gripp?\\w*|grabb?\\w*|holding|holds?|held)\\s+(?:at\\s+)?(?:my|the)\\s+chest\\b"), &g_CHEST_IN_A_HAND_TAIL()), &py_str(")(?=[^.!?]*\\b(?:sweat\\w*|clammy|grey|gray|pale|breathless|short\\s+of\\s+breath|sick|dizzy|nause\\w*|faint\\w*|fell|fallen|collaps\\w*|pass(?:ed|ing)?\\s+out|went\\s+down|hurts?|hurting|pain\\w*|ache\\w*|tight\\w*|pressure)\\b)"))]));
    s_DENTAL_WORDS(py_str("dentist|dental|fillings?|anaesthetic|anesthetic|numbing|novocaine|injection|wisdom\\s+tooth|root\\s+canal|extraction|extracted|(?:tooth|teeth|molar)\\s+(?:\\w+\\s+){0,2}(?:out|removed|pulled|taken|extracted)|(?:removed|pulled|extracted|took|take)\\s+(?:a\\s+|my\\s+|his\\s+|her\\s+|their\\s+|the\\s+)?(?:tooth|teeth|molar)"));
    s_DRINK_WORDS(py_str("drink|drinking|drunk|pints?|wine|beer|alcohol|pub|party|tipsy|night\\s+out|had\\s+a\\s+few|a\\s+few\\s+(?:pints|drinks|beers|wines)|a\\s+couple\\s+of\\s+(?:drinks|pints|beers|wines)|been\\s+on\\s+it|on\\s+the\\s+lash|bladdered|squiffy|merry|half\\s+cut|pissed|steaming|stag\\s+do|hen\\s+do|wedding\\s+do|celebrat\\w*|one\\s+too\\s+many|a\\s+skinful|three\\s+sheets"));
    s_SLUR_RED_BESIDE(py_str("weak\\w*|numb(?:ness|ed|ing|s)?|droop\\w*|confus\\w*|can'?t\\s+move|can\\s+not\\s+move|one\\s+side|left\\s+side|right\\s+side|slumped|limp\\s+(?:arm|leg|hand|side)|dead\\s+(?:arm|leg|hand|side|foot|fingers)|(?:arm|leg|hand|side|foot|face)\\s+(?:is\\s+|feels?\\s+|went\\s+)?(?:weak|numb|dead|heavy)"));
    s_SMALL_CHILD(py_str("toddler|baby|infant|newborn|(?:one|two|1|2)[\\s-]?year[\\s-]?old|1[0-9][\\s-]?month[\\s-]?old|1[89]\\s*months|preschooler|small child|little one|(?:son|daughter|boy|girl|child|kid)\\s+is\\s+(?:one|two|three|1|2|3)\\b"));
    s_SLUR_THICK(py_str("(?:slurr\\w*|slurs?|slurred|speech\\s+(?:is|are|sounds?|comes)|words?\\s+(?:are|is|come|coming|sound|sounds)|mush(?!room)\\w*|thick\\s+speech)"));
    s_WORDS_OUT_OCCASION(py_str("\\b(?:in|during|at)\\s+(?:a\\s+|the\\s+|my\\s+|his\\s+|her\\s+|their\\s+|our\\s+)?(?:meetings?|interviews?|presentations?|class(?:es)?|lectures?|part(?:y|ies)|phone\\s?calls?|video\\s?calls?|calls?|seminars?|assembl(?:y|ies))\\b|\\bwhen(?:ever)?\\s+(?:i|he|she|they)\\s+(?:talk|speak|meet|phone|ring|present|have\\s+to\\s+speak|stand\\s+up)\\b|\\bmeet(?:ing)?\\s+(?:new\\s+)?(?:people|someone|strangers)\\b|\\bin\\s+front\\s+of\\s+(?:people|everyone|the\\s+class|a\\s+crowd|others)\\b|\\bpublic\\s+speaking\\b|\\bon\\s+the\\s+phone\\b|\\b(?:nerv\\w*|anxious|anxiety|panic\\w*|shy|embarrass\\w*|scared|self[- ]conscious|introvert\\w*)\\b"));
    s_STROKE_HAND(py_tuple(vec![py_str("\\b(?:can'?t|cannot|couldn'?t|could\\s+not|unable\\s+to|not\\s+able\\s+to|struggl\\w*\\s+to)\\s+(?:hold|grip|lift)\\s+(?:my|his|her|their|our|the)\\s+(?:cup|mug|glass|drink|knife|fork|pen|phone|keys|bag|book|plate|spoon|toothbrush|comb|hairbrush)\\b"), py_str("\\b(?:keeps?|kept|is|was|are|were|started)\\s+(?:on\\s+)?dropping\\s+(?:my|his|her|their|our|the)\\s+(?:cup|mug|glass|drink|knife|fork|cutlery|pen|phone|keys|bag|book|plate|spoon|toothbrush|remote|knitting)\\b"), py_str("\\b(?:dropped|dropping)\\s+(?:my|his|her|their|our|the)\\s+(?:cup|mug|glass|drink|knife|fork|cutlery|pen|phone|keys|bag|book|plate|spoon|toothbrush|remote|knitting)\\b")]));
    s_STROKE_HAND_HARD(py_tuple(vec![py_str("\\b(?:droop\\w*|face\\s+(?:is\\s+|has\\s+|s\\s+)?(?:dropped|drooping|gone\\s+on\\s+one\\s+side)|slurr\\w*|slurred|speech\\s+(?:is\\s+|was\\s+|has\\s+)?(?:slurred|wrong|gone|thick)|can'?t\\s+speak|cant\\s+speak|wrong\\s+words?|words?\\s+wrong|not\\s+making\\s+sense|double\\s+vision|drool\\w*|swallow\\w*|doesn'?t\\s+know\\s+(?:where|what|who)|didn'?t\\s+know\\s+(?:where|what|who)|(?:one\\s+side|left\\s+side|right\\s+side|one\\s+arm|one\\s+leg|one\\s+hand|one\\s+foot|(?:the\\s+)?left\\s+(?:arm|leg|hand|foot)|(?:the\\s+)?right\\s+(?:arm|leg|hand|foot))\\s+(?:is\\s+|has\\s+gone\\s+|went\\s+|feels?\\s+)?(?:weak\\w*|numb(?:ness|ed|ing|s)?)|(?:face|lip|mouth|tongue)\\s+(?:feels?\\s+|is\\s+|has\\s+gone\\s+|went\\s+)?numb(?:ness|ed|ing|s)?)")]));
    s_STROKE_HAND_BESIDE(add(&py_tuple(vec![py_str("\\b(?:confus\\w*|disorient\\w*|doesn'?t\\s+know\\s+(?:where|what|who)|droop\\w*|face\\s+(?:is\\s+|has\\s+|s\\s+)?(?:dropped|drooping|gone\\s+on\\s+one\\s+side)|slurr\\w*|slurred|speech\\s+(?:is\\s+|was\\s+|has\\s+)?(?:slurred|wrong|gone|thick)|can'?t\\s+speak|cant\\s+speak|wrong\\s+words?|words?\\s+wrong|not\\s+making\\s+sense|vision|can'?t\\s+see|cant\\s+see|blurr\\w*|double\\s+vision|drool\\w*|swallow\\w*)\\b")]), &g_STROKE_HAND_HARD()));
    s_STROKE_HAND_CHRONIC(py_tuple(vec![py_str("\\b(?:sometimes|occasionally|always|for\\s+years|for\\s+months|for\\s+ages|since\\s+(?:the|her|his|their|last|she\\s+was|he\\s+was|they\\s+were)|it'?s\\s+been\\s+like\\s+this|arthritis|arthrit\\w*|parkinson\\w*|multiple\\s+sclerosis|\\bms\\b|cerebral\\s+palsy|carpal\\s+tunnel|raynaud\\w*|circulation|when\\s+it'?s\\s+cold|when\\s+it\\s+is\\s+cold|in\\s+the\\s+cold|when\\s+the\\s+weather\\s+(?:is\\s+)?(?:cold|turns)|after\\s+the\\s+flu|the\\s+flu\\b|long\\s+covid|neuropath\\w*|chemo\\w*|old\\s+stroke|stroke\\s+years\\s+ago|dementia\\s+already)\\b")]));
    s_STROKE_WEAK(py_tuple(vec![py_str("\\b(?:weak\\w*|numb|dead|useless|heavy|droop\\w*|gone\\s+(?:all\\s+)?(?:weak|numb|dead))\\b[^.?!]{0,25}\\b(?:one\\s+side|left\\s+side|right\\s+side|down\\s+one\\s+side)\\b"), py_str("\\b(?:one\\s+side|left\\s+side|right\\s+side)\\b[^.?!]{0,25}\\b(?:weak\\w*|numb|dead|useless|heavy|droop\\w*|gone\\s+(?:all\\s+)?(?:weak|numb|dead)|won'?t\\s+work|wont\\s+work|not\\s+working)\\b"), py_str("\\b(?:face|cheek|mouth|smile|eyelid|one\\s+eye)\\b[^.?!]{0,25}\\b(?:droop\\w*|has\\s+gone\\s+(?:slack|droopy|funny|loose)|sagging|lopsided|gone\\s+slack)\\b"), py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+(?:feel|move|use|hold)\\s+(?:my|his|her|their)\\s+(?:arm|hand|leg|side|left\\s+arm|right\\s+arm|left\\s+leg|right\\s+leg)\\b"), py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+lift\\s+(?:my|his|her|their)\\s+(?:arm|hand)\\b(?!\\s*(?:above|up|over|high|any\\s*more|at\\s+all|very|to\\s+the|past|out))"), py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+(?:lift|raise)\\s+(?:both\\s+(?:of\\s+)?)?(?:my|his|her|their)\\s+arms\\b(?!\\s*(?:above|up|over|high|any\\s*more|at\\s+all|very|to\\s+the|past|out|since|because))"), py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+(?:lift|raise)\\s+both\\s+arms\\b(?!\\s*(?:above|up|over|high|any\\s*more|at\\s+all|very|to\\s+the|past|out))"), py_str("\\b(?:my|his|her|their)\\s+(?:side|left\\s+(?:arm|hand|leg|side)|right\\s+(?:arm|hand|leg|side))\\b[^.?!]{0,15}\\b(?:has\\s+|have\\s+|had\\s+|went\\s+|is\\s+|feels?\\s+|feels\\s+like\\s+its\\s+)?(?:gone|went|all)\\s+(?:all\\s+)?(?:numb|dead|useless|weak|funny|heavy|like\\s+lead)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:arm|hand)\\b[^.?!]{0,15}\\b(?:has\\s+|have\\s+|had\\s+|went\\s+|is\\s+|feels?\\s+|feels\\s+like\\s+its\\s+)?(?:gone|went|all)\\s+(?:all\\s+)?(?:dead|useless|weak)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:arm|hand)\\b[^.?!]{0,25}\\bnumb(?:ness|ed|ing|s)?\\b[^.?!]{0,25}\\b(?:for\\s+(?:a\\s+)?(?:few|several|couple\\s+of|two|three|ten|twenty|\\d+)\\s+(?:minutes?|mins?|hours?|hrs?)|came\\s+and\\s+went|then\\s+it\\s+went|out\\s+of\\s+nowhere|suddenly|this\\s+(?:morning|afternoon|evening))\\b"), py_str("\\b(?:lost|loss\\s+of)\\s+(?:the\\s+)?(?:sight|vision)\\s+in\\s+(?:(?:my|his|her|their)\\s+)?(?:left|right|one|an?)\\s+eye\\b"), py_str("\\b(?:can'?t|cannot)\\s+see\\s+(?:out\\s+of|with|in)\\s+(?:(?:my|his|her|their)\\s+)?(?:left|right|one)\\s+(?:of\\s+(?:my|his|her|their)\\s+)?eyes?\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:sight|vision|eye)\\b[^.?!]{0,15}\\b(?:has\\s+|have\\s+|had\\s+)?(?:gone|went|is\\s+gone)\\b(?!\\s+(?:all\\s+)?(?:blurry|blurred|fuzzy|dim|funny|worse|bad|downhill|soft|double)\\b)(?![^.?!]{0,25}\\b(?:black|red|puffy|swollen|bloodshot|bruised)\\b)(?![^.?!]{0,25}\\bwhen\\s+(?:i|he|she|they)\\s+(?:stand|stands|get\\s+up|gets\\s+up|bend|bends|stood|got\\s+up)\\b)(?![^.?!]{0,25}\\b(?:briefly|momentarily|for\\s+(?:a|an|a\\s+few)\\s+(?:second|seconds|moment|minute|minutes)|then\\s+(?:it\\s+)?came\\s+back|and\\s+came\\s+back)\\b)"), py_str("\\b(?:blind|sightless)\\s+in\\s+(?:my|his|her|their|one)\\s+(?:left|right|one)\\s+eye\\b"), py_str("\\b(?:gone|going|went)\\s+blind\\b(?!\\s+(?:drunk|fold|folded|baked?))(?![^.?!]{0,25}\\b(?:briefly|momentarily|for\\s+(?:a|an|a\\s+few)\\s+(?:second|seconds|moment|minute|minutes)|then\\s+(?:it\\s+)?came\\s+back|and\\s+came\\s+back)\\b)")]));
    s_SIGHT_A_WHILE(py_tuple(vec![py_str("\\bfor\\s+(?:\\d+|a\\s+couple\\s+of|couple\\s+of|a\\s+few|few|several|many|a|an|two|three|four|five|six|seven|eight|nine|ten)\\s+(?:days?|weeks?)\\b"), py_str("\\b(?:a\\s+couple\\s+of\\s+(?:days|weeks)|a\\s+few\\s+(?:days|weeks)|\\d+\\s+(?:days|weeks))\\s+ago\\b"), py_str("\\byesterday\\b"), py_str("\\blast\\s+(?:night|week)\\b")]));
    s_STROKE_WEAK_OFF(py_tuple(vec![py_str("\\b(?:cold|freezing|frozen|snow|outside|slept\\s+on|sleeping\\s+on|lay\\s+on|lying\\s+on|sat\\s+on|sat\\s+funny|trapped\\s+nerve|pins\\s+and\\s+needles|gym|weights|workout|leg\\s+day|dentist|filling|anaesthetic|anesthetic|migraine|hungover|drunk|massage|blood\\s+test|injection|vaccine|jab)\\b"), py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+lift\\s+(?:my|his|her|their)\\s+(?:arm|hand)\\b[^.?!]{0,40}\\b(?:because|since|due|after|from|thanks)\\b[^.?!]{0,30}\\b(?:shoulder|rotator|elbow|wrist|sling|cast|crutch|collarbone|collar\\s+bone|operation|surgery|operated|post-?op|broken|fracture\\w*|cracked|pulled|strained|sprain\\w*|arthritis|arthritic|injured|injury|tendon|ligament|bursitis|frozen)\\b"), py_str("\\b(?:shoulder|rotator|elbow|wrist|sling|cast|collarbone|operation|surgery|broken|fracture\\w*|strained|sprain\\w*|arthritis|injured|injury)\\b[^.?!]{0,40}\\b(?:can'?t|cannot)\\s+lift\\s+(?:my|his|her|their)\\s+(?:arm|hand)\\b"), py_str("\\blift\\s+(?:my|his|her|their)\\s+(?:arm|hand)\\b[^.?!]{0,20}\\bto\\s+(?:reach|touch|grab|hold|carry|wave|put|scratch|open|close|comb|wash|brush|dry|dress|text|type)\\b"), py_str("\\b(?:one|an?)\\s+eye\\s+(?:closed|shut|covered|patched|winking)\\b"), py_str("\\b(?:hands|feet|foot|toes|fingers|legs)\\b"), py_str("\\b(?:both|two)\\b[^.?!]{0,20}\\b(?:numb|dead|tingl\\w*|pins\\s+and\\s+needles|asleep)\\b"), py_str("\\b(?:numb|dead|tingl\\w*|pins\\s+and\\s+needles|asleep)\\b[^.?!]{0,20}\\b(?:both|two)\\b"), py_str("\\barms\\b[^.?!]{0,25}\\b(?:numb|dead|tingl\\w*|pins\\s+and\\s+needles|asleep|fell\\s+asleep)\\b"), py_str("\\b(?:numb|dead|tingl\\w*|pins\\s+and\\s+needles|asleep)\\b[^.?!]{0,25}\\barms\\b"), py_str("\\b(?:sitting|sat|seated|stood|standing|kneeling|crouched|leaning|leant|slouched|crossed\\s+(?:my|his|her|their)\\s+legs|asleep|sleep\\w*|in\\s+the\\s+night|at\\s+night|overnight|in\\s+bed|holding|held|carrying|carried|gripping|gripped|grip|pressed|pressure|squeez\\w*|shopping|bags?|phone|driving|driven|typing|writing|playing|knitting|sewing|guitar|piano|cinema|sofa|desk|train|bus|theatre|meeting|lecture|church|sleep\\s+on\\s+it)\\b"), py_str("\\b(?:hayfever|hay\\s+fever|allerg\\w*|conjunctivitis|sty|stye|contact\\s+lens|lens|cataract\\w*|glaucoma|laser\\s+surgery|new\\s+glasses|optician|itch\\w*|screen|flu|cold|sinus\\w*|infected|infection)\\b"), py_str("\\b(?:eye\\s*shadow|eye\\s*liner|eyeliner|mascara|make-?up|foundation|moisturis\\w*|sunscreen|sun\\s?cream|shampoo|soap|shower\\s+gel|toothpaste|glasses|specs|goggles|contact\\s+lenses?)\\b"), py_str("\\bfor\\s+(?:\\d+\\s+|a\\s+few\\s+|many\\s+|most\\s+of\\s+my\\s+)?(?:years|decades|months)\\b"), py_str("\\b(?:since\\s+(?:(?:i|he|she|they|we)\\s+(?:was|were)\\s+|(?:a|an)\\s+)(?:a\\s+)?(?:child|kid|baby|born|birth|little)|all\\s+my\\s+life|my\\s+whole\\s+life|for\\s+as\\s+long\\s+as\\s+i\\s+can\\s+remember|from\\s+birth|born\\s+(?:blind|with)|was\\s+born|as\\s+a\\s+(?:child|kid|baby)|(?:many\\s+)?years\\s+ago|a\\s+lazy\\s+eye|lazy\\s+eye|always\\s+(?:been|had))\\b"), py_str("\\bsince\\s+(?:birth|childhood|i\\s+was\\s+(?:little|a\\s+kid|a\\s+baby))\\b"), py_str("\\bborn\\s+(?:blind|deaf|with\\s+one\\s+eye|without\\s+(?:an|one)\\s+eye)\\b")]));
    s_RESPOND_SENSE(py_str("to\\s+(?:(?:the|my|his|her|their|these|those)\\s+)?(?:tablets?|pills?|medication|medicine|meds|antibiotics?|injections?|treatment|chemo|therapy|dose|drugs?|texts?|messages?|emails?|calls?|whatsapp|letters?|tickets?|names?|phones?|mobiles?|landline|voicemail|answerphone|doorbell)\\b"));
    s_PERSON(py_str("(?:mum|mom|mother|dad|father|son|daughter|husband|wife|partner|boyfriend|girlfriend|brother|sister|friend|flatmate|roommate|baby|child|kid|kids|children|grand(?:child|children|son|daughter|kid)s?|step(?:child|children|son|daughter)s?|boy|girl|man|woman|lady|guy|bloke|neighbou?r|colleague|boss|gran|granny|nan|grandad|grandma|grandpa|mate|carers?|care\\s+workers?|support\\s+workers?|social\\s+workers?|personal\\s+assistants?|nurses?|midwives?|midwife|doctors?|gps?|physios?|therapists?|patient|pupil|student|teacher|driver|passenger|singer|player|runner|swimmer)"));
    s_HELP_SOMEONE(py_tuple(vec![add(&add(&py_str("\\b(?:(?:want|wanted|need|needed|trying|try)\\s+to\\s+)?(?:help|support|approach|talk\\s+to|deal\\s+with)\\s+(?:a|an|my|some)\\s+"), &g_PERSON()), &py_str("\\b[^.?!]{0,40}\\b(?:suicid\\w*|self[-\\s]?harm\\w*|cutting|hurting\\s+(?:him|her|them)sel(?:f|ves)|wants?\\s+to\\s+die|starving\\s+(?:him|her|them)sel(?:f|ves))\\b"))]));
    s_SUICIDE_TOPIC(py_tuple(vec![py_str("\\b(?:article|book|documentar(?:y|ies)|film|movie|programme|program|podcast|episode|news|paper|report|study|studies|research|statistics?|stats|figures?|rates?|numbers|data|training|trained|counsell?or|counselling|counseling|helpline|charity|campaign|awareness|prevention|policy|lecture|course|class|essay|thesis|dissertation|talk|seminar|conference|day|week|month)\\b[^.?!]{0,30}\\bsuicid"), py_str("\\bsuicid\\w*\\b[^.?!]{0,20}\\b(?:rates?|statistics?|figures?|numbers|prevention|awareness|campaign|helpline|charity|counsell?or|research|paper)\\b")]));
    s_SHAKEN(py_tuple(vec![py_str("\\b(?:i|we|he|she|they)\\s+(?:shook|shaken|have\\s+shaken|has\\s+shaken)\\s+(?:him|her|them|the\\s+(?:baby|newborn|infant|toddler|child)|my\\s+(?:baby|newborn|infant|son|daughter|child|kid|boy|girl|toddler))\\b"), py_str("\\b(?:shook|shaken)\\s+(?:the|my|our|his|her)\\s+(?:baby|newborn|infant|toddler)\\b"), py_str("\\b(?:baby|newborn|infant|toddler)\\b[^.?!]{0,25}\\b(?:was|were|got|has\\s+been|had\\s+been)\\s+shaken\\b")]));
    s_SHAKEN_REPLY(py_tuple(vec![py_str("A baby who has been shaken needs checking now, even if they seem fine: bleeding inside the head can take hours to show. Ring 999 or 911 if they are floppy, drowsy, sick, fitting or not feeding normally, otherwise take them to A&E today - and if you feel close to shaking them, put them down safely and step away for a minute."), py_str("That needs a doctor today: shaking a baby can bleed inside the head without a mark on them. Ring 999 or 911 if they go floppy, drowsy, vomit or fit, and otherwise go straight to A&E - putting the baby down safely and walking away for a minute is the right move when you are at the end of your rope.")]));
    s_THIRD_PERSON_ACTOR(py_tuple(vec![py_str("\\b(?:he|she|they|my\\s+(?:friend|mate|boyfriend|girlfriend|partner|husband|wife|son|daughter|child|kid|mum|mom|mother|father|dad|brother|sister|step\\w+|cousin|nephew|niece|grandson|granddaughter|gran|nan|ex|flatmate|housemate|neighbou?r|colleague|boss|classmate|patient|client|lodger|aunt|auntie|uncle))\\b")]));
    s_OTHER_SELF_HARM(py_tuple(vec![py_str("\\b(?:my\\s+(?!own\\b)\\w+|he|hes|she|shes|they|someone|somebody)\\b.{0,20}\\b(?:cuts?|cutting|hits?|hitting|burns?|burning|hurts?|hurting|pinch(?:es|ing)|scratch(?:es|ing)|bites?|biting|bangs?|banging|starves?|starving|scalds?|scalding)\\s+(?:him|her|them)sel(?:f|ves)\\b(?=\\s+(?:on\\s+purpose|again\\b|when\\s+(?:he|she|they)\\s*(?:'s|s|is|are|was|were)\\b|to\\s+(?:hurt|punish|cope)|because\\b))"), py_str("\\b(?:my\\s+(?!own\\b)\\w+|he|hes|she|shes|they|someone|somebody)\\b.{0,20}\\b(?:cuts?|cutting|hits?|hitting|burns?|burning|hurting|pinch(?:es|ing)|scratch(?:es|ing)|bangs?|banging|starves?|starving|scalds?|scalding)\\s+(?:him|her|them)sel(?:f|ves)\\b(?!\\s+(?:on|with|in|at|by|while|against|accidentally|shaving|cooking|chopping|opening|playing|making|ironing|cycling|running|trying)\\b)")]));
    s_AGE_PERSON(py_tuple(vec![py_str("(?:\\d{1,2}[\\s-]*(?:month|months|week|weeks|year|years|yr|yrs|mo|mth|mths)[\\s-]*old\\b|(?:\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)[\\s-]*(?:months?|weeks?|years?|yrs?)[\\s-]+old\\b|\\b(?:newborn|newborns|toddlers?|infants?|teenagers?|teens?|preschoolers?|schoolboy|schoolgirl)\\b)")]));
    s_BLUE_TINGE(py_tuple(vec![py_str("(?=.*\\b(?:blue|grey|gray)\\s+tinge\\b)(?=.*\\b(?:lips?|mouth|face|skin|fingers?|fingertips?|hands?|feet|toes|nails?|cheeks?|tongue|gums)\\b)"), py_str("(?=.*\\btinge\\s+of\\s+(?:blue|grey|gray)\\b)(?=.*\\b(?:lips?|mouth|face|skin|fingers?|fingertips?|hands?|feet|toes|nails?|cheeks?|tongue|gums)\\b)")]));
    s_BLUE(py_tuple(vec![py_str("\\b(?:going|went|turned|turning|gone|goes|turns?)\\s+blue\\b"), py_str("\\b(?:lips?|face|fingertips?|fingers?|nail\\s?beds?|around\\s+the\\s+mouth|skin|cheeks?|tongue|gums|hands?|feet|toes|legs?|mouth)\\b.{0,20}\\b(?:blue|grey|gray)\\b"), g_BLUE_TINGE(), py_str("\\b(?:blue|grey|gray)\\s+(?:around\\s+the\\s+lips?|round\\s+the\\s+lips?|in\\s+the\\s+face|lips?)\\b"), py_str("\\b(?:blue|grey|gray)\\s+all\\s+over\\b"), add(&add(&add(&add(&py_str("\\b(?:i|i'm|he|he's|she|she's|they|they're|we|you|someone|somebody|(?:my|the|our)\\s+"), &g_PERSON()), &py_str("|")), &g_PERSON()), &py_str(")\\s+(?:is|was|are|were|looks?|looking|looked|appears?|seems?|becomes?|becoming|has\\s+gone|had\\s+gone|went|turned|turning|going|gone|getting|gets)\\s+(?:(?:a|bit|really|very|so|all|quite|pale|grey|gray|cold|clammy|sweating|confused|shivering|breathless|struggling|limp|floppy|sleepy|and)\\s+){0,4}(?:blue|grey|gray)\\b"))]));
    s_PET_ANY(py_tuple(vec![py_str("\\b(?:dog|dogs|puppy|puppies|pup|pups|cat|cats|kitten|kittens|rabbit|rabbits|bunny|bunnies|hamster|hamsters|guinea\\s+pig|guinea\\s+pigs|parrot|parrots|budgie|budgies|bird|birds|ferret|ferrets|horse|horses|pony|ponies|tortoise|tortoises|snake|snakes|lizard|lizards|gerbil|gerbils|rat|rats|pet|pets)\\b")]));
    s_BLUE_STRONG(py_tuple(vec![py_str("\\b(?:going|turned|turning|gone|goes|go|turns?|getting|gets|is\\s+turning)\\s+(?:blue|grey|gray)\\b"), py_str("\\b(?:lips?|face|fingertips?|fingers?|nail\\s?beds?)\\b.{0,20}\\b(?:blue|grey|gray)\\b"), py_str("\\b(?:blue|grey|gray)\\s+all\\s+over\\b")]));
    s_LIPS_COLD(py_tuple(vec![py_str("\\b(?:lips?|mouth|face)\\b[^.!?]{0,30}\\b(?:cold|freezing|frozen|chilly|perishing|the\\s+snow|the\\s+sea|the\\s+pool|the\\s+swim|a\\s+swim|swimming)\\b"), py_str("\\b(?:cold|freezing|frozen|chilly)\\b[^.!?]{0,30}\\b(?:lips?)\\b"), py_str("\\b(?:out\\s+in\\s+the\\s+cold|in\\s+this\\s+cold|from\\s+the\\s+cold)\\b")]));
    s_COLD_ILLNESS(py_tuple(vec![py_str("\\b(?:have|has|had|having|got|caught|catch|fighting|full\\s+of|down\\s+with|coming\\s+down\\s+with|suffering\\s+from|come\\s+down\\s+with|getting)\\s+(?:a|the|my|his|her|their|this|such\\s+a|a\\s+really)\\s+(?:\\w+\\s+){0,2}cold\\b"), py_str("\\ba\\s+cold\\b[^.!?]{0,20}\\b(?:chest|flu|fever|temperature|wheeze|coughing)\\b"), py_str("\\b(?:my|his|her|their|the)\\s+(?:\\w+\\s+){0,2}cold\\b[^.!?]{0,20}\\b(?:wont\\s+shift|worse|gone\\s+to\\s+my\\s+chest)\\b")]));
    s_BLUE_FOOD(py_tuple(vec![py_str("\\b(?:tomato(?:es)?|cheese|milk|chicken|beef|pork|lamb|meat|fish|ham|bacon|bread|toast|pickle(?:s|d)?|chutney|kimchi|sauerkraut|dough|batter|rice|pasta|potato(?:es)?|onion(?:s)?|garlic|fruit|apple(?:s)?|banana(?:s)?|orange(?:s)?|lemon(?:s)?|berries|strawberr(?:y|ies)|grape(?:s)?|cabbage|lettuce|salad|vegetables?|soup|sauce|smoothie|juice|wine|beer|yogurt|yoghurt|cream|butter|egg(?:s)?|mushroom(?:s)?|courgette(?:s)?|cake|biscuit(?:s)?|leftovers?)\\b")]));
    s_BLUE_PERSON_SUBJECT(py_tuple(vec![add(&add(&py_str("\\b(?:i|i'm|i\\s+am|he|he's|she|she's|they|they're|we|we're|you|someone|somebody|anyone|baby|child|children|kid|kids|toddler|infant|boy|girl|man|woman|mum|mom|mother|dad|father|son|daughter|husband|wife|partner|friend|gran|grandma|grandad|nan|(?:my|the|our)\\s+"), &g_PERSON()), &py_str(")\\b[^.!?]{0,20}\\b(?:is|was|are|were|went|gone|goes|going|turned|turning|turns?|looks?|looking|looked|appears?|seems?|become|becomes|becoming|getting|gets)\\s+(?:\\w+\\s+){0,3}(?:blue|grey|gray)\\b"))]));
    s_BLUE_OTHER_PERSON(py_tuple(vec![py_str("\\b(?:he|she|him|her|they|them|his|hers|their|someone|somebody|anyone|man|woman|boy|girl|child|children|baby|babies|toddler|patient|kid|kids|mum|mom|mother|dad|father|son|daughter|husband|wife|partner|boyfriend|girlfriend|brother|sister|friend|gran|grandma|grandad|grandpa|nan|uncle|aunt|niece|nephew|colleague|neighbour|neighbor|people|men|women|workers?|guests?|everyone|anybody)\\b")]));
    s_BLUE_CLOTH(py_tuple(vec![py_str("\\b(?:jeans|jumper|jumpers|top|tops|shirt|shirts|t-?shirts?|blouse|dress|dresses|trousers|pants|shorts|skirt|coat|jacket|hoodie|cardigan|socks?|tights|knickers|underwear|bra|pyjamas|pjs|washing|laundry|whites|wash|fabric|clothes|clothing|cloth|wool|denim|cotton|linen|curtains?|sheets?|towels?|bedding|duvet|cover|cushion|carpet|rug|dye|dyed|dyeing|bleach|paint|paintbrush|pen|ink|felt\\s?tip|highlighter)\\b")]));
    s_BLUE_FINE(py_tuple(vec![py_str("\\b(?:feel|feeling|feels|felt)\\s+(?:a\\s+bit\\s+|really\\s+|so\\s+|very\\s+)?(?:blue|grey|gray)\\b"), py_str("\\b(?:i'?m|i\\s+am)\\s+(?:a\\s+bit\\s+|really\\s+|so\\s+)?blue\\b"), py_str("\\b(?:mood|day|monday|week|film|movie|song|album|book)\\b.{0,15}\\b(?:blue|grey|gray)\\b"), py_str("\\b(?:blue|grey|gray)\\s+(?:mood|day|monday|car|door|paint|paintwork|wall|walls|sky|carpet|sofa|dress|shirt|top|jeans|coat|eyes?|bin|bag|towel|fabric|nail\\s+varnish|polish|cheese|milk|ink|pen|light|screen|toothbrush|hair|tie|shoes?|trainers?|hat|scarf)\\b"), py_str("\\bblue\\s+cheese\\b"), py_str("\\b(?:room|rooms|wall|walls|ceiling|kitchen|bathroom|bedroom|hall|hallway|lounge|floor|woodwork|paintwork|curtains?|blinds?|lighting|light)\\b[^.?!]{0,25}\\b(?:blue|grey|gray)\\b"), py_str("\\b(?:paint|painting|painted|dye|dyed|sprayed|spray)\\b[^.?!]{0,25}\\b(?:blue|grey|gray)\\b"), py_str("\\b(?:wearing|wears?|bought|buy|buying|got|getting|choose|chooses|chose|chosen|picked|pick|like|likes|love|loves|prefer|prefers|want|wants|wanted|need|needs|needed|ordered|order|matches|match)\\b[^.?!]{0,25}\\b(?:blue|grey|gray)\\b"), py_str("\\b(?:blue|grey|gray)\\b[^.?!]{0,20}\\b(?:paint|dye|shirt|dress|top|coat|jeans|hat|scarf|shoes|trainers|car|door|wall|carpet|sofa|bag|towel|team|kit|badge|light|card|ticket|folder|mug|cup|plate|bike|bicycle|pram|buggy|room|bedroom|bathroom)\\b")]));
    s_SELF_TALK(py_tuple(vec![py_str("\\b(?:tell|tells|telling|told|ask|asks|asking|asked|say|says|saying|said|think|thinks|thinking|thought|remind|reminding|reminded|convince|convincing|convinced|promise|promising|promised|let|letting|made|make|making|allow|allowing|allowed|permit|permitting|permitted|forgive|forgiving|gave|give|giving|reassure|reassuring|reassured|comfort|comforting|talk|talking|keeping)\\b[^.?!]{0,16}\\bmy\\s*self\\b")]));
    s_ROUTINE_WAKE(py_tuple(vec![py_str("\\b(?:can'?t|cant|cannot|won'?t|wont|isn'?t|is\\s+not|not)\\s+wake\\s+(?:up\\s+)?(?:for|to|before|in\\s+time\\s+for|on\\s+time\\s+for)\\s+(?:school|work|college|uni|university|nursery|childminder|the\\s+(?:school\\s+)?(?:bus|train)|(?:his|her|their|my|an?)\\s+(?:exam|exams|test|appointment|meeting|interview|shift|flight|lesson|lecture|class|early\\s+start))\\b"), py_str("\\b(?:can'?t|cant|cannot|won'?t|wont)\\s+wake\\s+(?:up\\s+)?to\\s+get\\s+(?:to|ready\\s+for)\\s+(?:school|work|college|uni|university|nursery)\\b"), py_str("\\b(?:for|to)\\s+(?:school|work|college|uni|university|nursery)\\s+(?:this\\s+)?(?:morning|today)\\b.{0,20}\\b(?:won'?t|wont|can'?t|cant)\\s+wake\\b")]));
    s_WAKE_RED(py_tuple(vec![py_str("\\b(?:not|isn'?t|stopped)\\s+breathing\\b"), py_str("\\bblue\\b"), py_str("\\blips\\b"), py_str("\\b(?:seizure|fit|fits|limp|floppy|unconscious|unresponsive|pulse|ambulance)\\b"), py_str("\\b(?:temperature|fever|feverish)\\b"), py_str("\\bburning\\s+up\\b"), py_str("\\bhot\\s+to\\s+(?:the\\s+)?touch\\b"), py_str("\\b(?:grey|gray|pale|clammy|sweating)\\b"), py_str("\\bvomit\\w*\\b"), py_str("\\b(?:been\\s+)?sick\\s+(?:all\\s+night|all\\s+day|twice|three\\s+times|since)\\b"), py_str("\\bslurr\\w*\\b"), py_str("\\bnot\\s+making\\s+sense\\b"), py_str("\\bdoesn'?t\\s+make\\s+sense\\b"), py_str("\\bconfused\\b"), py_str("\\bcan'?t\\s+(?:speak|talk)\\b"), py_str("\\bface\\s+(?:is\\s+|looks\\s+)?droop\\w*\\b"), py_str("\\b(?:one\\s+side|his\\s+side|her\\s+side)\\s+(?:is\\s+)?(?:weak|limp|dropping)\\b")]));
    s__COPULA(py_str("(?:is|are|was|were|keeps?|kept|started|he'?s|hes|she'?s|shes|they'?re|theyre)"));
    s__FITTING_A_THING(py_str("(?!\\s+(?:in|on|into|up|together|his|her|their|my|your|our|the|a|an|new|some|another|kitchen|bathroom|windows?|doors?|floor|carpet|wardrobe|cabinets?|shelves|shelf|tiles|laminate|boiler|units?|radiators?|dishwasher|oven|cooker|shower|toilet|sink|sofa|bed|lino)\\b)"));
    s_SEIZURE(py_tuple(vec![py_str("\\bseizur\\w*\\b"), py_str("\\b(?:had|has|having|have)\\s+a\\s+fit\\b"), py_str("\\bconvuls(?:ing|ions?)\\b"), add(&add(&add(&py_str("\\b"), &g__COPULA()), &py_str("\\s+fitting\\b")), &g__FITTING_A_THING()), add(&add(&py_str("\\b"), &g__COPULA()), &py_str("\\s+seizing\\b(?!\\s+up)"))]));
    s_FIT_ROUTINE(py_tuple(vec![py_str("\\b(?:every\\s+(?:few|couple\\s+of|so\\s+many)\\s+(?:months|weeks|years)|twice\\s+a\\s+year|once\\s+a\\s+year|a\\s+(?:few|couple)\\s+of\\s+times\\s+a\\s+year|now\\s+and\\s+(?:then|again)|from\\s+time\\s+to\\s+time|keppra|epilep\\w*)\\b"), py_str("\\bfits?\\s+(?:when|if|whenever)\\b[^.?!]{0,30}\\b(?:tired|unwell|poorly|ill|stressed|miss\\w*|forget\\w*|skip\\w*|meds?|medicine|tablets?|keppra)\\b")]));
    s_SEIZURE_STORY_GUARD(py_tuple(vec![py_str("\\bseizure\\s+(?:laughing|from\\s+laughing)\\b"), py_str("\\bseizures?\\s+of\\s+(?:laughter|laughing|giggl\\w*|coughing|sneez\\w*)\\b"), py_str("\\b(?:had|having|have|get)\\s+a\\s+seizure\\s+(?:laughing|at\\s+that)\\b"), py_str("\\bfits?\\s+of\\s+(?:cough\\w*|sneez\\w*|giggl\\w*|laughter|laughing|the\\s+giggles|temper|pique)\\b"), py_str("\\b(?:nearly|almost|thought\\s+i'?d)\\s+(?:had\\s+)?(?:a\\s+)?(?:seizure|fit)\\b")]));
    s_FIT_STILL_RINGS(py_tuple(vec![py_str("\\b(?:can'?t|cannot|not|stopped|isn'?t|stop)\\s+breath"), py_str("\\bwon'?t\\s+stop\\b"), py_str("\\bisn'?t\\s+stopping\\b"), py_str("\\bkeeps?\\s+going\\b"), py_str("\\bunconscious\\b"), py_str("\\bcollapsed\\b"), py_str("\\bon\\s+the\\s+floor\\b"), py_str("\\bconvuls\\w*"), py_str("\\bturning\\s+blue\\b"), py_str("\\bblue\\s+lips\\b"), py_str("\\bstiff\\b"), py_str("\\bshaking\\s+all\\s+over\\b"), py_str("\\bambulance\\b")]));
    s_CHILDHOOD_FIT(py_tuple(vec![py_str("\\b(?:as|when\\s+i\\s+was)\\s+(?:a\\s+|an\\s+)?(?:kid|child|baby|toddler|tot|infant|nipper|young|little|small|at\\s+school)\\b"), py_str("\\b(?:in|during)\\s+my\\s+childhood\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:about\\s+)?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|1|2|3|4|5|6|7|8|9|10|11|12)\\b")]));
    s_FIT_THIS_WEEK(py_tuple(vec![py_str("\\b(?:last\\s+(?:week|weekend|friday|saturday|sunday|monday|tuesday|wednesday|thursday)|earlier\\s+this\\s+week|the\\s+other\\s+(?:day|night)|yesterday|a\\s+(?:few|couple\\s+of)\\s+days?\\s+ago|\\d+\\s+days?\\s+ago|on\\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday))\\b")]));
    s_SEIZURE_FIGURATIVE(py_tuple(vec![py_str("\\bseizure\\s+(?:laughing|from\\s+laughing)\\b"), py_str("\\b(?:had|having|have|get)\\s+a\\s+seizure\\s+(?:laughing|when|at\\s+that)\\b"), py_str("\\b(?:nearly|almost|thought\\s+i'?d)\\s+(?:had\\s+)?(?:a\\s+)?(?:seizure|fit)\\b"), py_str("\\bfit\\s+of\\s+(?:cough\\w*|laughter|laughing|sneez\\w*|giggles|itch\\w*|temper|pique)\\b")]));
    s_MILD_CHOKE(py_tuple(vec![py_str("\\bi\\s+chok(?:ed|ing)\\s+on\\s+(?:(?:my|your|his|her|their|a|an|some|the)\\s+)?(?:own\\s+)?(?:drink|tea|coffee|water|juice|beer|wine|ice|pill|tablet|vitamin|crisp|chip|crumb|sweet|grape|bit\\s+of\\s+\\w+|mouthful|spit(?:tle)?|saliva|it)\\b"), py_str("\\bi\\s+chok(?:ed)\\s+(?:at|in|during)\\s+(?:the\\s+)?\\w+"), py_str("\\bi\\s+(?:nearly|almost)\\s+choked\\b"), py_str("\\bchoked?\\s+(?:on\\s+)?laughing\\b")]));
    s_CHOKE_RED(py_tuple(vec![py_str("\\b(?:can'?t|cannot|couldn'?t|unable\\s+to)\\s+(?:breathe|breath|brethe|speak|talk|make\\s+a\\s+sound)\\b"), py_str("\\b(?:went|turned|going|turning|is|looks?)\\s+blue\\b"), py_str("\\b(?:lips?|face)\\s+(?:are|is|going)\\s+blue\\b"), py_str("\\b(?:still|now|and)\\s+choking\\b"), py_str("\\b(?:collapsed|unconscious|gone\\s+limp|floppy)\\b"), py_str("\\b(?:holding|clutching|grabbing)\\s+(?:his|her|their|my)\\s+throat\\b"), py_str("\\bchoking\\s+(?:and|but)\\s+(?:can'?t|cannot)\\b")]));
    s_CHOKE_STILL_RED(py_tuple(vec![py_str("\\bcan'?t\\s+breathe\\b"), py_str("\\bcannot\\s+breathe\\b"), py_str("\\bcan'?t\\s+(?:speak|talk|get\\s+(?:any\\s+)?(?:air|breath))\\b"), py_str("\\bnot\\s+breathing\\b"), py_str("\\b(?:turning|going|gone)\\s+blue\\b"), py_str("\\b(?:lips|face|mouth)\\s+(?:are\\s+|'re\\s+|is\\s+)?(?:blue|grey|gray)\\b"), py_str("\\bno\\s+air\\b"), py_str("\\b(?:airway|throat)\\s+(?:is\\s+|'s\\s+)?(?:blocked|closing|swollen)\\b"), py_str("\\bstill\\s+choking\\b"), py_str("\\bstuck\\s+in\\s+(?:my|his|her|their)\\s+throat\\b"), py_str("\\b(?:passed|passing|faint|fainted|collapsed|unconscious)\\b")]));
    s_WENT_BLUE_IDIOM(py_tuple(vec![py_str("\\b(?:went|going|go|turn(?:ed|s)?|turning)\\s+blue\\s+in\\s+the\\s+face\\b.{0,60}\\b(?:laugh\\w*|joke\\w*|crying|shouting|screaming|kidding|breath\\w*)\\b"), py_str("\\b(?:laugh\\w*|joke\\w*|crying|holding\\s+(?:my|his|her|your)\\s+breath|held\\s+(?:my|his|her|your)\\s+breath)\\b.{0,60}\\b(?:went|going|go|turned?|turning)\\s+blue\\s+in\\s+the\\s+face\\b"), py_str("\\b(?:hold|holds|holding|held)\\s+(?:my|his|her|their|your|the)?\\s*breath\\b[^.?!]{0,60}\\b(?:go\\w*|went|turn(?:ed|s|ing)?|turning)\\s+blue\\b"), py_str("\\b(?:go\\w*|went|turn(?:ed|s|ing)?|turning)\\s+blue\\b[^.?!]{0,60}\\b(?:hold|holds|holding|held)\\s+(?:my|his|her|their|your|the)?\\s*breath\\b"), py_str("\\b(?:went|going|go|goes|turn(?:ed|s|ing)?|turning)\\s+blue\\b[^.?!]{0,50}\\b(?:laugh\\w*|joke\\w*|kidding|giggling)\\b"), py_str("\\b(?:laugh\\w*|joke\\w*|kidding|giggling)\\b[^.?!]{0,50}\\b(?:went|going|go|goes|turn(?:ed|s|ing)?|turning)\\s+blue\\b")]));
    s_WENT_BLUE_FINE(py_tuple(vec![py_str("\\braynaud\\w*\\b"), py_str("\\bin\\s+the\\s+cold\\b"), py_str("\\bwhen\\s+(?:it'?s|its)\\s+cold\\b"), py_str("\\bwith\\s+(?:the\\s+)?cold\\b"), py_str("\\bafter\\s+(?:a\\s+)?(?:swim|cold\\s+swim|bath)\\b"), py_str("\\bthe\\s+wash\\b"), py_str("\\bwash(?:ing)?\\s+machine\\b"), py_str("\\bjeans\\b"), py_str("\\bbruise\\b"), py_str("\\bcold\\b"), py_str("\\bfreez\\w*"), py_str("\\bwinter\\b"), py_str("\\bsnow\\b"), py_str("\\bice\\b")]));
    s_BLUE_COLD_CAUSE(py_tuple(vec![py_str("\\bwith\\s+(?:the\\s+)?cold\\b"), py_str("\\bin\\s+the\\s+cold\\b"), py_str("\\bwhen\\s+(?:it'?s|its)\\s+cold\\b"), py_str("\\bin\\s+(?:this|that)\\s+cold\\b"), py_str("\\bafter\\s+(?:a\\s+)?(?:swim|cold\\s+swim|bath)\\b"), py_str("\\bfrom\\s+the\\s+cold\\b"), py_str("\\b(?:he|she|they|i|we|it)\\s*(?:'s|s|is|are|am|was|were|feels?|felt)\\s+(?:so\\s+|very\\s+|really\\s+|a\\s+bit\\s+|already\\s+|freezing\\s+)?cold\\b"), py_str("\\bfreezing\\b"), py_str("\\bshiver\\w*"), py_str("\\bhypotherm\\w*")]));
    s_MACHINE(py_str("\\b(?:blender|food\\s+processor|liquidis(?:er|or)|mincer|meat\\s+grinder|mangle|shredder|chipper|wood\\s+chipper|washing\\s+machine|tumble\\s+dryer|spin\\s+dryer|drill|circular\\s+saw|chainsaw|bench\\s+saw|lathe|hydraulic\\s+press|conveyor|escalator|elevator|mower|lawnmower|strimmer|hedge\\s+trimmer|auger|cement\\s+mixer|pasta\\s+machine)\\b"));
    s_STUCK_MACHINE(py_tuple(vec![add(&py_str("\\b(?:hand|hands|arm|arms|finger|fingers|thumb|wrist|foot|leg|toe|toes|hair|knee|head|clothes|sleeve|jumper|coat|scarf|glove|jewellery|ring)\\b[^.?!]{0,40}\\b(?:stuck|caught|trapped|jammed|wedged|pinned|crushed|sucked\\s+in)\\b[^.?!]{0,40}\\b"), &g_MACHINE()), add(&add(&py_str("\\b"), &g_MACHINE()), &py_str("\\b[^.?!]{0,40}\\b(?:has|have|got|took|caught|grabbed|crushed|sucked|chewed)\\b[^.?!]{0,30}\\b(?:my|his|her|their|the)\\s+(?:hand|arm|finger|fingers|sleeve|hair|jumper|coat)\\b")), add(&add(&py_str("\\b"), &g_MACHINE()), &py_str("\\b[^.?!]{0,40}\\b(?:stuck|caught|trapped|jammed|wedged|crushed)\\b(?=[^.?!]{0,70}\\b(?:my|his|her|their|the)\\s+(?:hand|hands|arm|arms|finger|fingers|thumb|wrist|foot|feet|leg|legs|toe|toes|hair|knee|head|sleeve|jumper|coat|scarf|glove)\\b)")), add(&add(&py_str("\\b(?:hand|hands|arm|arms|finger|fingers|thumb|wrist|foot|feet|leg|legs|toe|toes|hair|knee|sleeve|jumper|coat|scarf|glove)\\b[^.?!]{0,40}\\b"), &g_MACHINE()), &py_str("\\b[^.?!]{0,40}\\b(?:stuck|caught|trapped|jammed|wedged|crushed)\\b"))]));
    s_BLEEDING(py_tuple(vec![py_str("\\b(?:bleeding|blood)\\s+(?:badly|heavily|everywhere|a\\s+lot|so\\s+much|profusely)\\b"), py_str("\\bso\\s+much\\s+blood\\b"), py_str("\\bblood\\s+everywhere\\b"), py_str("\\bspurting\\b"), py_str("\\b(?:won't|wont|doesn't)\\s+stop\\s+bleeding\\b.{0,40}\\b(?:hour|hours|pressure|bandage)\\b"), py_str("\\b(?:stabbed|stab\\s+wound)\\b"), py_str("\\bdeep\\s+cut\\b"), py_str("\\b(?:cut|slit|sliced)\\s+(?:(?:my|his|her|their|a)\\s+)?(?:wrist|neck|throat|thigh)s?\\b"), py_str("\\b(?:bleeding|blood)\\s+(?:through|soaking)\\s+(?:the|my)\\s+(?:bandage|towel|clothes)\\b"), py_str("\\bcan't\\s+stop\\s+the\\s+bleeding\\b"), py_str("^(?!.*\\b(?:nose|nostril|nosebleed|period|periods|menstrual|poo|stool|faeces|feces|wee|urine|vomit|vomited|sick|nappy|diaper|film|films|movie|movies|tv|telly|programme|show|episode|scene|game|games|book|novel|documentary|costume|fake|prop|ketchup|paint|cut|cuts|cutting|gash|gashes|sliced|slices|wound|wounded|grazed|scratched)\\b).*?\\b(?:a\\s+lot\\s+of|lots\\s+of|loads\\s+of|puddles?\\s+of|pools?\\s+of|so\\s+much|that\\s+much)\\s+blood\\b"), py_str("^(?!.*\\b(?:nose|nostril|nosebleed|period|periods|menstrual|poo|stool|faeces|feces|wee|urine|vomit|vomited|sick|nappy|diaper|film|films|movie|movies|tv|telly|programme|show|episode|scene|game|games|book|novel|documentary|costume|fake|prop|ketchup|paint)\\b).*?(?:\\b(?:the\\s+)?blood\\s+(?:won'?t|wont|doesn'?t|doesnt|isn'?t)\\s+stopp?(?:ed|ing)?\\b|\\bcan'?t\\s+(?:get\\s+)?(?:it|the\\s+bleeding|the\\s+blood)\\s+to\\s+stop\\b)"), py_str("\\b(?:slash(?:ed|es|ing)?|slit)\\s+(?:(?:my|his|her|their|a)\\s+)?(?:arms?|wrists?|hands?|legs?|thighs?|neck|throat)\\b")]));
    s_DROWNING_HYPERBOLE(py_tuple(vec![py_str("\\b(?:nearly|almost)\\s+drowned\\s+(?:laughing|in\\s+(?:it|them|tears))\\b"), py_str("\\bdrown(?:ing|ed)?\\s+in\\s+(?:paperwork|work|debt|emails?|chores|admin|deadlines?|homework|bills|laundry|it|them|wine|beer|sorrows)\\b"), py_str("\\bdrowned\\s+(?:my|our|his|her|their)\\s+sorrows\\b")]));
    s_WATER_DROWNING_NOW(py_tuple(vec![py_str("\\b(?:he|she|they|hes|shes|theyre|he's|she's|they're)\\b[^.?!]{0,15}\\bdrown(?:ing|s)\\b"), py_str("\\b(?:the|a|my|our)\\s+(?:kid|kids|baby|child|children|boy|girl|toddler|little\\s+one|son|daughter|husband|wife|partner|mum|mom|dad|father|mother|friend|mate|brother|sister|neighbour|neighbor|man|woman|lad|person|someone|somebody)\\b[^.?!]{0,15}\\bdrown(?:ing|s)\\b"), py_str("\\b(?:is|are)\\s+drowning\\b"), py_str("\\b(?:im|i'?m|i\\s+am)\\s+drowning\\b[^.?!]{0,25}\\b(?:water|sea|ocean|pool|lake|river|canal|bath|bathtub|tub|waves?|deep\\s+end|swimming|under\\s+the\\s+surface)\\b"), py_str("\\b\\d+\\s+(?:year\\s+old|yo)\\b[^.?!]{0,15}\\bdrowning\\b")]));
    s_WATER_THING_DROPPED(py_tuple(vec![py_str("\\b(?:dropped|drops?|slipped|slips?|tipped|tips?|knocked|knocks?|kicked|kicks?|threw|left|lost|dunked|dipped)\\s+(?:[\\w']+\\s+){0,2}(?:phone|mobile|keys?|wallet|purse|watch|glasses|book|towel|bag|coat|hat|money|passport|teddy|dummy|nappy|remote|toy|ball|boot|trainer|tools?|sponge|cloth|loaf|bread|toast|cutlery|mug|cup)\\b")]));
    s_WATER_PERSON_TOO(py_tuple(vec![py_str("\\b(?:went|goes|gone|sank|sunk|sinking|fell|fallen|falls?)\\s+(?:in|into|under)\\b[^.?!]{0,20}\\b(?:water|pool|sea|river|lake|pond|canal|bath)\\b"), py_str("\\b(?:went|goes|gone)\\s+under\\b"), py_str("\\b(?:not\\s+breathing|no\\s+pulse|unresponsive|blue\\s+lips|limp)\\b"), py_str("\\b(?:my|his|her|their|our)\\s+(?:son|daughter|child|baby|kid|boy|girl|toddler|husband|wife|partner|friend|mate|mum|mom|dad|brother|sister)\\b[^.?!]{0,25}\\b(?:drowning|drowned|under|in\\s+the\\s+water)\\b")]));
    s_WATER_DROWNING_FINE(py_tuple(vec![py_str("\\bdrown\\w*\\s+(?:in\\s+|under\\s+)?(?:emails?|work|works|debt|debts|paperwork|admin|bills?|tears|weeds|jobs?|chores?|stress|sorrows|booze|wine|beer|noise|it\\s+all|everything|the\\s+lot|chips|gravy|ketchup|sauce|salt|vinegar|cream|syrup|oil)\\b"), py_str("\\b(?:plant|plants|garden|lawn|grass|seedlings?|crops?|flowers?|houseplant|basil|tomato\\w*|hanging\\s+basket)\\b"), py_str("\\blaugh(?:ing|ed|s)\\b"), py_str("\\bdrowned\\s+(?:out|by\\s+the\\s+noise)\\b"), py_str("\\b(?:feel|feels|felt|feeling|like)\\s+(?:im|i'?m|i\\s+am|like\\s+im)\\s+drown")]));
    s_WATER_NOT_A_PERSON(py_tuple(vec![py_str("\\b(?:roads?|streets?|garden|lawn|fields?|basement|cellar|kitchen|hall|drive|driveway|lane|path|patio|bridge|cars?|vans?|shed|garage|town|village|pitch|allotment|house|flat|loft|attic|cupboard)\\b"), py_str("\\b(?:flood\\w*|burst\\s+pipe|leak\\w*)\\b"), py_str("\\bunder\\s+(?:a|an|the)\\s+(?:bus|car|lorry|truck|van|tram|train|taxi)\\b"), py_str("\\b(?:phone|mobile|keys?|wallet|purse|watch|glasses|book|towel|shoes?|socks?|clothes|washing|dishes|plates?|mug|cup|kettle|loaf|bread|food|dinner|rice|pasta|eggs?|toys?|ball|dummy|nappy|ring|coin|remote|bag|coat|hat|money|passport|boot|trainer|teddy|blanket)\\b\\s+(?:has\\s+|had\\s+)?(?:went|goes|gone|fell|falls?|slipped|drops?|dropped|is|was|got|gets)\\b"), py_str("\\b(?:boats?|dingh(?:y|ies)|canoes?|kayaks?|ship|ferry|raft)\\b\\s+(?:has\\s+|had\\s+)?(?:went|goes|gone|sank|sunk|tipped|turned\\s+over|capsized|is|was|got)\\b")]));
    s_WATER_SELF(py_tuple(vec![py_str("\\bi\\b[^.?!]{0,25}\\b(?:fell|fallen|slipped|slips?|went|goes|gone|sank|sunk|jumped|dropped|tipped|tumbled|drowned|drowning|got\\s+pulled|got\\s+dragged|was\\s+pulled|was\\s+pushed|was\\s+dragged|was\\s+thrown|was\\s+tipped|got\\s+knocked)\\b[^.?!]{0,30}\\b(?:water|pond|river|lake|sea|pool|canal|reservoir|loch|harbour|harbor|lagoon|deep\\s+end|shallow\\s+end|bath|tub|under)\\b"), py_str("\\b(?:pushed|pulled|threw|thrown|dragged|tipped|shoved|knocked)\\s+me\\s+(?:in|into|under)\\b"), py_str("\\b(?:pulled|hauled|dragged|lifted|fished|got|gets)\\s+me\\s+out\\s+of\\s+the\\s+(?:water|pond|river|lake|sea|pool|canal)\\b"), py_str("\\bi\\s+(?:got|was|were|am)\\s+(?:pushed|pulled|thrown|dragged|tipped|shoved|knocked)\\b"), py_str("\\bwe\\b[^.?!]{0,25}\\b(?:went|goes|gone|fell|slipped|sank)\\b[^.?!]{0,20}\\bunder\\b")]));
    s_WATER_HUMAN(py_tuple(vec![py_str("\\b(?:baby|babies|newborn|infant|son|daughter|child|children|kid|kids|boy|girl|toddler|little\\s+one|niece|nephew|grandson|granddaughter|husband|wife|partner|mum|mom|mother|dad|father|brother|sister|friend|mate|lads?|guys?|man|woman|person|people|he|she|him|her|them|they|someone|somebody|anyone|i|me|my\\s+(?:son|daughter|child|baby|boy|girl|husband|wife|partner|mum|mom|dad|friend|brother|sister)|we|us)\\b")]));
    s_WATER_FINE(py_tuple(vec![py_str("\\b(?:swimming\\s+(?:lesson|lessons|class|club|badge)|learnt?\\s+to\\s+swim|learning\\s+to\\s+swim|splash\\w*|playing|playing\\s+about|having\\s+fun|laughing|came\\s+up|came\\s+back\\s+up|popped\\s+up|surfaced|came\\s+to\\s+the\\s+surface|loves?\\s+(?:going\\s+under|the\\s+water)|likes?\\s+(?:going\\s+under|the\\s+water)|enjoys\\s+the\\s+water)\\b")]));
    s_WATER_HEAD_FEAR(py_tuple(vec![py_str("\\b(?:can'?t|cant|cannot|couldn'?t|won'?t|wont|unable|struggl\\w*|scared|afraid|terrified|frightened|nervous|hate|hates|hated|panic\\w*|refus\\w*|fear|fears|feared|fearful|phobia|phobic|dread|dreads|dreaded|dreading)\\b[^.?!]{0,40}\\b(?:under\\s*(?:the\\s+)?(?:water|surface)|underwater|go(?:ing|es)?\\s+under)\\b"), py_str("\\b(?:put|puts|putting|get|gets|getting|duck|ducks|ducking|dip|dips|dipping|stick|sticks|sticking|hold|holds|holding|keeping|lower\\w*)\\b[^.?!]{0,20}\\bhead\\b[^.?!]{0,20}\\b(?:under\\s*(?:the\\s+)?(?:water|surface)|underwater)\\b"), py_str("\\bhead\\b[^.?!]{0,20}\\b(?:under\\s*(?:the\\s+)?(?:water|surface)|underwater)\\b[^.?!]{0,25}\\b(?:swim\\w*|pool|lesson\\w*|bath|shower|float\\w*|splash\\w*)\\b"), py_str("\\b(?:swim\\w*|pool|lesson\\w*|bath|shower|float\\w*|splash\\w*)\\b[^.?!]{0,25}\\bhead\\b[^.?!]{0,20}\\b(?:under\\s*(?:the\\s+)?(?:water|surface)|underwater)\\b"), py_str("\\b(?:scared|afraid|frightened|nervous|terrified|panic\\w*|dread\\w*)\\s+(?:of|about|to)\\s+(?:\\w+\\s+){0,2}(?:jump\\w*|get|getting|go|going|dive|diving|swim\\w*)\\s+(?:in|into)\\s+(?:the\\s+)?(?:water|pool|sea|lake|deep\\s+end|shallow\\s+end)\\b"), py_str("\\b(?:won'?t|wont|refus\\w*|won'?t|hates?|hated)\\s+(?:\\w+\\s+){0,2}(?:jump\\w*|get|getting|go|going|dive|diving|swim\\w*)\\s+(?:in|into)\\s+(?:the\\s+)?(?:water|pool|sea|lake|deep\\s+end|shallow\\s+end)\\b"), py_str("\\b(?:scream\\w*|cry\\w*|hate[sd]?|scared|afraid|frightened|terrified|nervous)\\b[^.?!]{0,30}\\bwater\\b[^.?!]{0,20}\\b(?:face|head|hair)\\b"), py_str("\\b(?:hold|holds|holding|held)\\b[^.?!]{0,25}\\b(?:my|his|her|their|your)\\s+breath\\b[^.?!]{0,35}\\b(?:when|whenever|every\\s+time|each\\s+time|before)\\b[^.?!]{0,30}\\b(?:go|goes|going|went|dive\\w*|dove|jump\\w*|swim\\w*|duck\\w*|put\\w*|get\\w*|stick\\w*|dip\\w*)\\b[^.?!]{0,20}\\b(?:under|underwater|in)\\b")]));
    s_WATER_IN_NOW(py_tuple(vec![py_str("\\b(?:jumped|jumping|dived|diving|dove|leapt|leaped|sank|sunk|sinking)\\b[^.?!]{0,15}\\b(?:in|into|under)\\b"), py_str("\\b(?:fell|fallen|falls?|went|goes|gone|slipped|slips?|tipped|tumbled|pushed|dropped)\\b[^.?!]{0,20}\\b(?:in|into)\\b[^.?!]{0,20}\\b(?:water|pool|sea|lake|river|canal|pond|deep\\s+end|shallow\\s+end|bath)\\b"), py_str("\\b(?:is|hes|shes|theys|they're|was|were|are|am)\\s+(?:still\\s+)?(?:in|under)\\s+(?:the\\s+)?(?:water|pool|sea|lake|river|canal|pond|deep\\s+end)\\b"), py_str("\\b(?:went\\s+under|gone\\s+under|deep\\s+end|shallow\\s+end)\\b")]));
    s_WATER_URGENT(py_tuple(vec![py_str("\\b(?:breath\\w*|blue|limp|unresponsive|not\\s+moving|not\\s+breathing|still\\s+not|hasn'?t\\s+(?:come|moved)|didn'?t\\s+come|still\\s+under|under\\s+for|get\\s+(?:him|her|them)\\s+(?:up|out)|find\\s+(?:him|her|them)|can'?t\\s+(?:see|swim|get|wake)|cant\\s+(?:see|swim|get|wake)|struggl\\w*|scream\\w*|panic\\w*|swim\\s+back|swallow\\w*)\\b")]));
    s_DROWNED_THING(py_tuple(vec![py_str("\\b(?:drown(?:ed|ing)?)\\s+(?:my|our|the|his|her|their|this|that|a)\\s+(?:plant|plants|houseplant|houseplants|seedlings?|basil|tomato(?:es)?|chillies|chilli|flowers?|lawn|grass|garden|window\\s+box|hanging\\s+basket|pot\\s+plant|succulent|cactus|fern|herbs?|crops?|phone|chips|toast|cereal|salad|sorrows|cares|fears|sulks|fire)\\b"), py_str("\\b(?:the|my|our)\\s+(?:plant|plants|houseplant|seedlings?|basil|lawn|garden|flowers?|herbs?|crops?)\\b[^.?!]{0,20}\\b(?:nearly\\s+)?drown(?:ed|ing)?\\b")]));
    s_HANGING(py_tuple(vec![crate::gen::modules::safety_cards::g_PERSON_HANGING(), py_str("\\b(?:hung|hanged|hang|hanging)\\s+(?:himself|herself|themselves|my\\s*self)\\b(?!\\s+(?:upside|on\\s+the\\s+(?:bars|swing|wall)|from\\s+the\\s+(?:bars|swing)|out\\s+to\\s+dry|up\\s+to\\s+dry|the\\s+(?:washing|laundry|clothes)\\b))"), py_str("\\b(?:hung|hanged|hang|hanging)\\s+(?:him|her|them)\\b\\s+(?:from|in|on|to)\\s+(?:the\\s+|a\\s+)?(?:rafters|beam|balcony|banister|railing|railings|stair(?:s|case)|ceiling|hook|tree|treehouse|door|bridge|window|bars|shower|garage|shed|loft|basement|bathroom|bedroom)\\b"), py_str("\\b(?:is|was|'s)\\s+(?:hanging|hung)\\b.{0,25}\\bfound\\b"), py_str("\\bfound\\s+(?:him|her|them|my\\s+\\w+)\\s+hanging\\b"), py_str("\\bfound\\s+(?:him|her|them|my\\s+\\w+)\\s+strangl\\w+\\b"), py_str("\\bstrangl(?:ed|ing)\\s+(?:me|him|her|them|himself|herself|themselves|someone|somebody)\\b"), add(&add(&py_str("\\bstrangl(?:ed|ing)\\s+(?:my|his|her|their|our)\\s+"), &crate::gen::modules::safety_phrases::g_ABUSER()), &py_str("\\b")), py_str("\\bstrangulation\\b"), py_str("\\bnoose\\b"), py_str("\\bstrangl(?:ed|ing)\\b.{0,20}\\bneck\\b"), py_str("\\b(?:ligature|rope|belt)\\s+(?:round|around|tight\\s+round)\\s+(?:his|her|their|my)\\s+neck\\b")]));
    s_SNAKE_LOOKALIKE(py_tuple(vec![py_str("\\b(?:snake\\s+plant|snake\\s+plants|snakes?\\s+and\\s+ladders|snake\\s+print|snake\\s+skin|snake\\s+charmer|snake\\s+bite\\s+kit|snake\\s+in\\s+the\\s+garden|saw\\s+a\\s+snake|grass\\s+snake|slow\\s+worm|snake\\s+in\\s+(?:my|the)\\s+(?:boot|shed|grass|pond))\\b"), py_str("\\b(?:stung|bit|bitten)\\s+by\\s+the\\s+(?:price|bill|cost|tag)\\b")]));
    s_ANIMAL_KICK(py_tuple(vec![py_str("\\b(?:horse|pony|horses|cow|cows|bull|bullock|donkey|mule|llama|alpaca)\\b[^.?!]{0,30}\\b(?:kicked|kick|kicks|trampled|trampling|stamped|stood\\s+on|stood\\s+on\\s+(?:my|his|her|me))\\b"), py_str("\\b(?:kicked|trampled|stamped\\s+on|stood\\s+on)\\s+by\\s+(?:a|an|the|my|his|her)\\s*(?:horse|pony|cow|bull|donkey|mule)\\b")]));
    s_BLIND_CORD(py_tuple(vec![py_str("\\b(?:blind|blinds|curtain|curtains|venetian|roman)\\s+cords?\\b[^.?!]{0,25}\\b(?:neck|round|around|tangled|caught|wrapped|hanging|choking|tight)\\b"), py_str("\\b(?:got|caught|tangled|wrapped|hanging|choking|stuck)\\b[^.?!]{0,25}\\b(?:blind|curtain|venetian|roman)s?\\s*cords?\\b"), py_str("\\b(?:tangled|caught|wrapped|hanging|stuck)\\s+(?:in|on|round|around)\\s+the\\s+(?:blind|blinds|curtain|curtains)\\b")]));
    s_STRANGLED_SELF(py_tuple(vec![py_str("\\bstrangl\\w*\\s+me\\b"), py_str("\\bthrottl\\w*\\s+me\\b"), py_str("\\bi\\s+(?:was|got|been|have\\s+been|had\\s+been)\\s+strangl\\w+"), py_str("\\bi\\s+(?:was|got|been)\\s+throttl\\w+"), py_str("\\bstrangled\\s+by\\s+(?:my|him|her|them|an?)\\b"), add(&add(&py_str("\\b(?:he|she|they|someone|somebody|this\\s+(?:man|woman|guy|lad|girl|bloke)|a\\s+(?:man|woman|bloke|lad|lady|girl|guy|stranger|boy)|my\\b[^.?!]{0,12}\\b"), &crate::gen::modules::safety_phrases::g_ABUSER()), &py_str(")[^.?!]{0,25}\\b(?:choked|chokes|is\\s+choking)\\s+me\\b")), py_str("\\b(?:grab(?:s|bed|bing)?|held|holds|holding|seiz(?:ed|es|ing)|pinned|shoved|pushed)\\s+me\\s+by\\s+the\\s+(?:throat|neck)\\b"), py_str("\\b(?:put|puts|putting|laid|laying|lay|wrapped|wrapping|wraps|had|has|slid|slipped|clamped|locked|forced)\\s+(?:his|her|their|both)\\s+(?:hands?|fingers|fist|forearm)\\s+(?:round|around|on|across|to)\\s+my\\s+(?:throat|neck)\\b"), py_str("\\b(?:put|puts|putting|wrapped|wrapping|wraps|threw|had|got|slid|slipped|clamped)\\s+(?:his|her|their|both)\\s+arms?\\s+(?:round|around|on|across)\\s+my\\s+throat\\b"), py_str("\\b(?:a\\s+hand|his\\s+hands?|her\\s+hands?|their\\s+hands?|someone'?s\\s+hands?|somebody'?s\\s+hands?)\\s+(?:round|around|on|across)\\s+my\\s+throat\\b"), py_str("\\b(?:a\\s+hand|his\\s+hands?|her\\s+hands?|their\\s+hands?|someone'?s\\s+hands?|somebody'?s\\s+hands?)\\s+(?:round|around)\\s+my\\s+neck\\b"), py_str("\\b(?:a\\s+hand|his\\s+hands?|her\\s+hands?|their\\s+hands?|someone'?s\\s+hands?|somebody'?s\\s+hands?)\\s+(?:on|across)\\s+my\\s+neck\\b(?=[^.?!]{0,30}\\b(?:tight(?:ly|en\\w*)?|grip\\w*|squeez\\w*|press\\w*|pin(?:ned|ning|s)?|clamp\\w*|hard|crush\\w*|chok\\w*)\\b)"), py_str("\\bstrangl(?:ed|ing|es|e)\\s+my\\s+(?:neck|throat)\\b"), py_str("\\bchok(?:ed|ing|es)\\s+my\\s+(?:neck|throat)\\b"), py_str("\\b(?:nearly|almost)\\s+(?:got|was|been|had\\s+me)\\s+(?:strangled|throttled|choked)\\b")]));
    s_STRANGLE_THING_SUBJECT(py_tuple(vec![py_str("\\b(?:the|this|that|my|our|his|her|their)\\s+(?:[a-z']+\\s+){0,2}(?:jeans|trousers|shirt|top|bra|shoes?|boots|socks?|belt|watch|coat|jumper|hoodie|heat|humidity|workload|work|job|debt|debts|bills?|traffic|stress|anxiety|panic|pressure|paperwork|bureaucracy|admin|commute|queue|noise|mortgage|rent|council|kids?|in-?laws?|exams?|revision|deadlines?|red\\s+tape|system|money|poverty|town|city|weather)\\b[^.?!]{0,20}\\bstrangl(?:e|es|ing|ed)\\b"), py_str("\\b(?:jeans|trousers|shirt|bra|heat|humidity|workload|work|job|debt|bills?|traffic|stress|anxiety|panic|pressure|paperwork|bureaucracy|admin|commute|queue|noise|mortgage|rent|council|kids|in-?laws|exams?|revision|deadlines?|red\\s+tape|system|money|poverty|life|everything)\\b(?:\\s+[a-z']+){0,2}\\s+(?:is|are|was|were|keeps?)\\s+strangl(?:ing|ed)\\b")]));
    s_KICKED_PERSON(py_tuple(vec![py_str("\\b(?:kicked|kicks?|trampled|trampling|stamped\\s+on|stood\\s+on)\\s+(?:me|us|my|our)\\b"), py_str("\\b(?:i|we)\\s+(?:was|were|got|have\\s+been|had\\s+been|'ve\\s+been)\\b[^.?!]{0,25}\\b(?:kicked|trampled|stamped\\s+on|stood\\s+on|sat\\s+on|thrown|tossed)\\b"), py_str("\\b(?:my|his|her|their|our)\\s+(?:son|daughter|child|kid|baby|boy|girl|husband|wife|partner|mum|mom|mother|dad|father|brother|sister|friend|nan|gran|grandad|grandma)\\b[^.?!]{0,25}\\b(?:was|were|got|had\\s+been)\\b[^.?!]{0,15}\\b(?:kicked|trampled|stamped\\s+on|stood\\s+on)\\b")]));
    s_SELF_CUT(py_tuple(vec![py_str("\\b(?:cut|cutting|sliced|slashed|gashed|slit|nicked|scratched)\\s+(?:my\\s*self|my\\s+(?:arm|arms|wrist|wrists|hand|hands|leg|legs|thigh|thighs|finger|fingers|foot|feet))\\b"), py_str("\\b(?:i\\s+)?(?:keep|keeps|kept|am|im|'m)\\s+(?:on\\s+)?cutting\\b")]));
    s_CUT_NOT_STOPPING(py_tuple(vec![py_str("\\b(?:won'?t|wont|doesn'?t|doesnt|didn'?t|didnt|not|never|keeps?)\\s+stop(?:ped|ping)?\\s+(?:bleeding|blood|the\\s+blood)\\b"), py_str("\\b(?:the\\s+)?blood\\s+(?:wont|won'?t|doesnt|doesn'?t)\\s+stop\\b"), py_str("\\bblood\\s+(?:everywhere|every\\s*where|a\\s+lot|lots|badly|heavily|pouring)\\b"), py_str("\\bsoaking\\s+(?:through|the)\\b")]));
    s_GAS_ON_YOU(py_tuple(vec![py_str("\\b(?:my|his|her|their|your|the)\\s+(?:hands?|clothes|shirt|coat|jumper|fingers|gloves|cloths?)\\b[^.?!]{0,25}\\b(?:smell|smells|smelling|smelt|reek|reeks|stink|stinks)\\b"), py_str("\\b(?:smell|smells|smelling|smelt)\\s+(?:of\\s+|like\\s+)?gas\\b[^.?!]{0,20}\\b(?:on|from)\\s+(?:my|his|her|their|your)\\s+(?:hands?|clothes|shirt|coat|jumper|fingers|gloves)\\b")]));
    s_GAS_ON_NOT_RUNNING(py_str("(?!\\s+(?:the\\s+)?(?:table|side|counter|worktop|shelf|floor|bed|sofa|desk|hook|wall|windowsill|ledge|rack|top\\s+shelf|board|tray|plate|doorstep))(?!\\s+(?:for|to\\s+keep)\\s+(?:the\\s+)?(?:boiler|pilot|pilot\\s+light|heating|hot\\s+water|water|radiators?|fire|shower|bath|oven|cooker|fridge|freezer|central\\s+heating))\\b"));
    s_GAS_LEFT_ON(py_tuple(vec![add(&py_str("\\b(?:left|leave|leaving|forgot|forgotten|think\\s+i\\s+(?:left|turned))\\b[^.?!]{0,30}\\b(?:gas|hob|burner|ring|cooker)\\b[^.?!]{0,12}\\bon\\b"), &g_GAS_ON_NOT_RUNNING()), py_str("\\b(?:turned|switched|put)\\s+(?:the\\s+)?(?:gas|hob|burner|ring|cooker)\\b[^.?!]{0,15}\\b(?:on|up)\\b[^.?!]{0,25}\\b(?:forgot|forgotten|walked\\s+away|left\\s+it)\\b"), add(&py_str("\\b(?:did|have|had)\\s+(?:i|you|we)\\s+\\b(?:leave|left)\\b[^.?!]{0,20}\\b(?:gas|hob|burner|cooker)\\b[^.?!]{0,10}\\bon\\b"), &g_GAS_ON_NOT_RUNNING()), py_str("\\b(?:gas|hob|burner|ring\\s+on\\s+the\\s+cooker)\\b[^.?!]{0,20}\\b(?:is\\s+)?(?:still\\s+)?on\\b[^.?!]{0,30}\\b(?:cant|cannot|can'?t|not\\s+sure|no\\s+idea|don'?t\\s+know)\\b"), py_str("\\b(?:gas|hob|burner)\\b[^.?!]{0,20}\\b(?:hissing|running|going\\s+and\\s+i)\\b")]));
    s_GAS_PAST(py_tuple(vec![py_str("\\b(?:smelt|smelled|smelling|smell)\\b[^.?!]{0,40}\\b(?:last\\s+(?:year|week|month|winter|summer)|years?\\s+ago|months?\\s+ago|weeks?\\s+ago|yesterday|ages\\s+ago|once|when\\s+i\\s+was|as\\s+a\\s+kid|at\\s+(?:my|her|his|our)\\s+\\w+'s)\\b"), py_str("\\b(?:last\\s+(?:year|week|month)|years?\\s+ago|when\\s+i\\s+was|ages\\s+ago)\\b[^.?!]{0,30}\\b(?:smelt|smelled|smell)\\b")]));
    s_INTRUDER(py_tuple(vec![py_str("\\b(?:someone|somebody|a\\s+man|a\\s+person|intruder|burglar)\\s+(?:is\\s+)?(?:trying\\s+to\\s+get\\s+in|breaking\\s+in|breaking\\s+into\\s+(?:the|my|our)\\s+(?:house|flat|home|kitchen|bedroom)|climbing\\s+(?:in|through|over)|in\\s+(?:the|my)\\s+(?:house|flat|home|garden|kitchen|living\\s+room)|inside\\s+(?:the|my)\\s+house)\\b"), py_str("\\b(?:there's|there\\s+is|theres)\\s+(?:someone|somebody|a\\s+man)\\s+(?:in|inside|at)\\s+(?:the|my)\\s+(?:house|flat|home|garden|window)\\b(?!\\s+(?:with|holding|delivering)\\s+(?:a|the|my)\\s+(?:parcel|package|delivery|letter|clipboard|takeaway|pizza|parcels))"), py_str("\\b(?:a\\s+man|someone|somebody)\\s+(?:is\\s+)?following\\s+me\\b(?!\\s+(?:on|around)\\s+(?:instagram|insta|twitter|x|tiktok|facebook|snapchat|social|the\\s+internet|online))"), py_str("\\b(?:someone|somebody|a\\s+man)\\s+(?:is\\s+)?(?:banging|pounding|kicking|hammering|smashing)\\s+(?:on\\s+)?(?:the|my)\\s+(?:door|window)\\b"), py_str("\\b(?:someone|somebody|a\\s+man|he|she|they)(?:'s|\\s+is)?\\s+(?:trying|tried)\\s+to\\s+(?:get\\s+in|break\\s+(?:in\\b(?!\\s+(?:his|her|their|my|your|these|those|new|a\\s+new)\\s)|the\\s+door|down\\s+the\\s+door))\\b"), py_str("\\b(?:i\\s+think|i\\s+heard)\\s+(?:someone|somebody)\\s+(?:is\\s+)?(?:in|inside|downstairs|upstairs)\\b")]));
    s_GASPING_THIRST(py_tuple(vec![add(&add(&py_str("\\bgasping\\s+(?:for|at)\\s+(?:"), &g__A_THIRST()), &py_str(")")), py_str("\\bgasping\\s+(?:for|at)\\s+(?:something\\s+to\\s+drink|something\\s+to\\s+eat)\\b"), py_str("\\bgasping\\s+(?:after|during|at)\\s+(?:a\\s+|that\\s+|the\\s+|my\\s+)?(?:run|jog|workout|gym|session|match|game|swim|cycle|bike\\s+ride|climb|hill|walk|climbing|going\\s+up|running)\\b"), py_str("\\bgasp\\w*\\s+(?:for|on)\\s+(?:fuel|petrol|diesel|power|charge|bandwidth|data)\\b")]));
    s_EXERTION(py_tuple(vec![py_str("\\b(?:after|during|while|when|whenever)\\s+(?:a\\s+|the\\s+|my\\s+)?(?:run|running|jog|jogging|gym|workout|exercise|exercising|cycling|ride|swim|swimming|walk|walking|football|netball|climbing|lifting|weights|stairs|cardio)\\b"), py_str("\\b(?:after|during|while|when|whenever)\\s+(?:im|i'm|ive|i've|were|we're|hes|he's|shes|she's|theyre|they're)\\s+(?:run|running|jog|jogging|gym|workout|exercis\\w*|cycl\\w*|rid(?:e|ing)|swim\\w*|walk\\w*|climb\\w*|lift\\w*|play\\w*|football|netball|squash|tennis|stairs|cardio|train\\w*|row\\w*|hik\\w*|danc\\w*|aerobics|hiit|spinning|weights?|press-?ups?|squats)\\b"), py_str("\\b(?:after|during|while|when|whenever)\\s+(?:i|we|he|she|they|my\\s+\\w+)\\s+(?:am\\s+|'m\\s+|is\\s+|was\\s+|are\\s+|start\\s+|do\\s+|play\\s+|go\\s+|come\\s+)?(?:run|running|jog|gym|workout|exercis\\w*|cycl\\w*|rid(?:e|ing)|swim\\w*|walk\\w*|climb\\w*|lift\\w*|play\\w*|football|netball|squash|tennis|stairs|cardio|train\\w*|row\\w*|hik\\w*|danc\\w*|aerobics|hiit|spinning|weights?|press-?ups?|squats)\\b"), py_str("\\b(?:going|walking|running)\\s+up\\s+(?:the\\s+)?(?:stairs|hill)\\b"), py_str("\\b(?:going|walking|running)\\s+(?:up\\s+)?upstairs\\b"), py_str("\\b(?:on|up|down|going\\s+up)\\s+(?:the\\s+|some\\s+)?(?:stairs|hill|upstairs)\\b")]));
    s_BREATH_CAUSE(py_tuple(vec![py_str("\\b(?:nose|nostrils?|blocked|bunged|stuffed|congestion|congested|hayfever|hay\\s+fever|sinus\\w*|blocked\\s+up)\\b"), py_str("\\b(?:a|this|that|the|my|your|his|her|stinking|rotten|bad|nasty|heavy|terrible|awful|horrible|proper|real|slight|streaming|head|chest|summer|winter|24\\s?hour)\\s+cold\\b"), py_str("\\b(?:got|have|had|caught|catch|fighting|coming\\s+down|full\\s+of)\\s+(?:a\\s+|the\\s+|this\\s+)?cold\\b"), py_str("\\b(?:flu|virus|flu\\s+bug)\\b[^.?!]{0,30}\\b(?:nose|blocked|bunged|stuffed|congestion|congested|sinus\\w*)\\b"), py_str("\\bchesty\\b")]));
    s_BREATH_SHORT(py_tuple(vec![py_str("\\b(?:short|out)\\s+of\\s+(?:breath|puff)\\b"), py_str("\\bbreathless\\b"), py_str("\\bpuffed\\s+out\\b"), py_str("\\b(?:struggling|struggle|hard|harder|difficult\\w*|trouble)\\b[^.?!]{0,18}\\bbreath\\w*\\b"), py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+breath\\w*\\b"), py_str("\\bbreathing\\s+(?:heavily|hard)\\b")]));
    s_EXERTION_SMALL(py_tuple(vec![py_str("\\b(?:on|up|down|climbing|walking\\s+up|going\\s+up)\\s+(?:the\\s+|a\\s+|some\\s+)?(?:stairs|staircase|steps|hill|slope|hillock)\\b"), py_str("\\b(?:going|walking|running|coming)\\s+upstairs\\b"), py_str("\\b(?:walk\\w*|stroll\\w*|potter\\w*|get(?:ting)?)\\s+(?:to\\s+|into\\s+)?(?:the\\s+)?(?:shops?|shop|bus|station|school|work|village|town)\\b"), py_str("\\b(?:small|short|little|gentle)\\s+(?:walk|walks|effort|hill|stroll)\\b"), py_str("\\b(?:a|one|two|three)\\s+flights?\\s+of\\s+stairs\\b"), py_str("\\b(?:after|on|from)\\s+the\\s+stairs\\b"), py_str("\\b(?:the|some)\\s+stairs\\b"), py_str("\\b(?:walking|walked|went|going)\\s+up\\s+(?:the\\s+|a\\s+)?(?:hill|slope)\\b"), py_str("\\b(?:getting|get|got)\\s+dressed\\b"), py_str("\\b(?:around|round)\\s+the\\s+(?:house|garden|park)\\b")]));
    s_MEAL_BREATH(py_tuple(vec![py_str("\\b(?:after|before|when|while)\\s+(?:i\\s+|she\\s+|he\\s+|they\\s+)?(?:eating|a\\s+(?:big\\s+|heavy\\s+|large\\s+|huge\\s+|proper\\s+)?meal|dinner|lunch|breakfast|tea)\\b"), py_str("\\b(?:eating|a\\s+meal|dinner|lunch|breakfast)\\b[^.?!]{0,20}\\b(?:breath|puffed|winded)\\w*\\b"), py_str("\\b(?:short|out)\\s+of\\s+(?:breath|puff)\\b[^.?!]{0,20}\\b(?:eat|eating|meal|dinner|lunch)\\b")]));
    s_EFFORT_ANYWHERE(py_tuple(vec![py_str("\\b(?:run|runs|running|jog\\w*|gym|workout|exercis\\w*|cycl\\w*|swim\\w*|football|netball|squash|tennis|cardio|stairs|hills?|climb\\w*|walk\\w*|hike|hiking|danc\\w*|hiit|weights?|press-?ups?|squats|spin|rowing|skipping|badminton|rugby|cricket|climbing|box\\w*|class|classes|session|ran|singing|sing|laughing|talking|carrying|shopping|cleaning|hoovering|garden\\w*|digging)\\b")]));
    s_SHORT_OF_BREATH(py_tuple(vec![py_str("\\b(?:out\\s+of|short(?:ness)?\\s+of)\\s+breath\\b"), py_str("\\b(?:still|now|very|quite|really|so|a\\s+bit|a\\s+little|always|already)\\s+(?:out\\s+of|short(?:ness)?\\s+of)\\s+breath\\b")]));
    s_A_BREATHER(py_tuple(vec![py_str("\\b(?:i|me|we|us|he|him|she|they|them|you|husband|wife|partner|boyfriend|girlfriend|mum|mom|mother|dad|father|son|daughter|baby|child|kid|nan|gran|grandma|grandad|brother|sister|friend|man|woman|boy|girl|patient|neighbour|neighbor|dogs?|cats?|pupp(?:y|ies)|kittens?|pets?|parrot|hamster|rabbit)\\b")]));
    s_BREATH_RED(py_tuple(vec![py_str("\\b(?:blue|lips?|can'?t\\s+speak|cannot\\s+speak|unable\\s+to\\s+speak|at\\s+rest|resting|in\\s+bed|woke\\s+me|sudden\\w*|getting\\s+worse|chest\\s+pains?|chest\\b[^.!?]{0,20}\\b(?:hurt\\w*|tight\\w*|pressure|heav\\w*|squeez\\w*)|(?:tight\\w*|pressure|heavy|heaviness)\\b[^.!?]{0,15}\\bchest\\b|swell\\w*|anaphyla\\w*|epipen|wheez\\w*|asthma\\s+attack|\\basthma\\b[^.!?]{0,15}\\b(?:bad|badly|worse|severe|uncontrolled|out\\s+of\\s+control)\\b|inhaler\\b[^.!?]{0,25}\\b(?:not\\s+working|isn'?t\\s+working|isn'?t\\s+helping|not\\s+helping)\\b|stopped\\s+breathing|isn'?t\\s+breathing|not\\s+breathing|gasping|coughing\\s+up|blood)\\b")]));
    s_BREATH_CAUSE_RED(add(&g_BREATH_RED(), &py_tuple(vec![py_str("\\basthma\\b"), py_str("\\binhaler\\b"), py_str("\\bwheez\\w*"), py_str("\\bchest\\b"), py_str("\\btongue\\b"), py_str("\\bstridor\\b"), py_str("\\bcroup\\b"), py_str("\\bthroat\\b[^.?!]{0,20}\\b(?:clos\\w*|swell\\w*|tight\\w*|shut|seal\\w*)\\b"), py_str("\\b(?:clos|swell|tight)\\w*\\s+(?:up\\s+)?(?:in\\s+|of\\s+)?(?:my\\s+|the\\s+|his\\s+|her\\s+)?throat\\b"), py_str("\\bcant\\s+swallow\\b"), py_str("\\bcan'?t\\s+swallow\\b"), py_str("\\bdifficulty\\s+swallowing\\b"), py_str("\\bswallow(?:ing)?\\s+(?:is\\s+)?(?:hard|painful\\s+and\\s+getting\\s+worse)\\b"), py_str("\\b(?:feel(?:ing)?|am|im|so|very|really|freezing|frozen|shivering|shivery|shivers|bit)\\s+cold\\b"), py_str("\\bcold\\s+(?:and|or)\\s+(?:shivering|shivery|clammy|sweating|weak|floppy|confused)\\b"), py_str("\\bcold\\s+to\\s+(?:the\\s+)?touch\\b")])));
    s_HUMAN_TOO(py_tuple(vec![py_str("\\b(?:my|our|her|his|their)\\s+(?:son|daughter|child|kid|baby|boy|girl|mum|mom|dad|father|mother|husband|wife|partner|brother|sister|nan|gran|grandma|grandad|friend|neighbour|neighbor)\\b"), py_str("\\b(?:i|i'm|im|he|she|they|we|you|someone|somebody)\\b[^.?!]{0,12}\\b(?:can'?t|cannot|can\\s+not|couldn'?t|struggling\\s+to|unable\\s+to)\\s+(?:breath\\w*|(?:get|catch)\\s+(?:my|his|her|their|our|a)\\s+breath\\w*|wake\\s+up|stay\\s+awake)"), py_str("\\b(?:i|i'm|im|he|she|they|we)\\b[^.?!]{0,10}\\b(?:am|is|are|'m|'s|'re|was|were)\\s+(?:bleeding|unconscious|unresponsive|collapsed|choking)\\b"), py_str("\\b(?:im|i'm)\\s+(?:bleeding|unconscious|unresponsive|collapsed|choking)\\b")]));
    s_PET_PAST(py_tuple(vec![py_str("\\b(?:last|this)\\s+(?:year|month|week)\\b"), py_str("\\byears\\s+ago\\b"), py_str("\\bages\\s+ago\\b"), py_str("\\bwhen\\s+(?:he|she|it|they)\\s+(?:\\w+\\s+){0,3}(?:was|were)\\s+(?:a\\s+)?(?:puppy|kitten|young)\\b"), py_str("\\bused\\s+to\\b"), py_str("\\bevery\\s+(?:few\\s+)?(?:days?|weeks?|months?|years?)\\b"), py_str("\\bis\\s+that\\s+(?:normal|usual|common)\\b")]));
    s_PET_GASP_FINE(py_tuple(vec![py_str("\\bgasp\\w*\\b[^.?!]{0,30}\\b(?:after|from|during|when(?:ever)?|on|in)\\b[^.?!]{0,25}\\b(?:walk|run|runn\\w*|plays?|playing|played|exercis\\w*|fetch|ball|games?|zoomies|chas\\w*|charged|park|garden|hill|stairs|beach|frisbee|swim\\w*|train\\w*)\\w*\\b"), py_str("\\bgasp\\w*\\b[^.?!]{0,25}\\b(?:in|from)\\s+(?:the|this|that)\\s+(?:heat|sun|hot\\s+weather|warmth)\\b"), py_str("\\bgasp\\w*\\s+at\\s+(?:the|a|an|his|her|its|their|my|other|next)\\b"), py_str("\\b(?:in|on|during)\\s+(?:the|a|his|her|its|their)\\s+(?:car|drive|journey|train|bus)\\b"), py_str("\\bgasp\\w*\\b[^.?!]{0,15}\\b(?:a\\s+bit|a\\s+little|slightly)\\b"), py_str("\\bgasp\\w*\\s+(?:for|after)\\s+(?:(?:a|an|the|some|its|his|her|their|my)\\s+)?(?:a\\s+)?(?:drink|water|food|biscuit|treat|dinner|tea)\\b")]));
    s_PET_GASP_EXPLAINED(py_tuple(vec![py_str("\\bgasp\\w*\\b[^.?!]{0,40}\\bwhen\\s+(?:he|she|it|they)(?:'?s|s|\\s|\\b)"), py_str("\\bgasp\\w*\\b[^.?!]{0,40}\\b(?:because|whenever|every\\s+time)\\b"), py_str("\\b(?:sneez\\w*|cough\\w*|hiccup\\w*|laugh\\w*|retch\\w*|splutter\\w*|purr\\w*)\\b[^.?!]{0,25}\\bgasp\\w*\\b"), py_str("\\bgasp\\w*\\b[^.?!]{0,30}\\b(?:on|off)\\s+the\\s+lead\\b"), py_str("\\bgasp\\w*\\b[^.?!]{0,30}\\bafter\\s+(?:a\\s+|an\\s+|the\\s+|his\\s+|her\\s+|its\\s+|their\\s+|our\\s+|my\\s+)?(?:drink\\w*|water|food|meal|dinner|tea|breakfast|eating|ate|gulping|too\\s+fast|too\\s+quickly)\\b")]));
    s_PET_GASP_RED(py_tuple(vec![py_str("\\bcan'?t\\s+breathe\\b"), py_str("\\bnot\\s+breathing\\b"), py_str("\\bblue\\b"), py_str("\\bpale\\b"), py_str("\\bunconscious\\b"), py_str("\\bchok\\w*"), py_str("\\bcollaps\\w*"), py_str("\\bwoke\\s+up\\b"), py_str("\\bwakes?\\s+up\\b"), py_str("\\bkept?\\s+waking\\b"), py_str("\\blimp\\b"), py_str("\\bstill\\b"), py_str("\\bfor\\s+(?:\\d+|a\\s+few|ten|twenty|half\\s+an?\\s+hour)\\b"), py_str("\\bgums?\\b"), py_str("\\bbright\\s+red\\b"), py_str("\\b(?:grey|gray|white)\\s+gums?\\b")]));
    s_HEAD_BANG(py_tuple(vec![py_str("\\b(?:bang|bangs|banged|banging|hit|hits|hitting|knock|knocks|knocked|knocking|strike|struck|smack(?:s|ed|ing)?|crack(?:s|ed|ing)?|bump(?:s|ed|ing)?|bashed?|bashing|smashed?|smashing|slammed?)\\s+(?:my|his|her|their|the|me\\s+in\\s+the|him\\s+in\\s+the|her\\s+in\\s+the|on\\s+the|in\\s+the)\\s+head\\b"), py_str("\\bhead\\s+(?:injury|injuries|trauma|wound)\\b"), py_str("\\b(?:got|gets|getting)\\s+(?:hit|struck|clipped|caught|banged)\\s+(?:on|in)\\s+the\\s+head\\b"), py_str("\\bhit\\s+(?:on|in)\\s+the\\s+head\\s+(?:by|with)\\b"), py_str("\\b(?:fell|fallen|fall|falls|tripped|slipped|toppled|tumbled)\\s+(?:over|down|off|from|onto|on\\s+to)\\b.{0,25}\\bhead\\b"), py_str("\\bhead\\s+(?:first|hit|banged|struck)\\s+(?:on|into|off|against)\\b")]));
    s_OUT_COLD_PERSON(py_tuple(vec![py_str("\\b(?:i|he|she|they|we|you)\\b[^.?!]{0,15}\\bout\\s+cold\\b"), add(&add(&py_str("\\b(?:my|his|her|their|our|your|the)\\s+"), &g_PERSON()), &py_str("\\b[^.?!]{0,20}\\bout\\s+cold\\b")), py_str("\\b(?:out\\s+cold)\\b[^.?!]{0,20}\\b(?:floor|ground|sofa|couch|bed|street|pavement|road|bathroom|kitchen|hospital)\\b"), py_str("\\b(?:fell|fallen|collapsed|dropped|banged|hit|knocked|tripped)\\b[^.?!]{0,25}\\bout\\s+cold\\b")]));
    s_HEAD_KNOCKOUT(py_tuple(vec![py_str("\\b(?:was|were|been|got|gets|getting|being|had|has)\\s+knocked\\s+out\\b(?!\\s+(?:of|in|by)\\b)"), py_str("\\bknocked\\s+(?:him|her|them|my\\s*self|him\\s*self|her\\s*self)\\s*out\\b"), crate::gen::modules::safety_cards::g_HEAD_OUT_COLD(), py_str("\\bout\\s+cold\\b[^.?!]{0,20}\\b(?:floor|ground|street|pavement|road|bathroom|kitchen|hospital)\\b")]));
    s_TOOTH_OUT_FINE(py_tuple(vec![py_str("\\bmilk\\s+tooth\\b"), py_str("\\bbaby\\s+tooth\\b"), py_str("\\bwobbly\\b"), py_str("\\bloose\\s+tooth\\b"), py_str("\\btooth\\s+fairy\\b"), py_str("\\btoothache\\b"), py_str("\\b(?:last|this)\\s+(?:year|month|week)\\b"), py_str("\\byears?\\s+ago\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little)\\b"), py_str("\\b(?:hole|socket|gap)\\b[^.?!]{0,25}\\btooth\\b"), py_str("\\btooth\\b[^.?!]{0,25}\\b(?:hole|socket)\\b"), py_str("\\bwhere\\s+(?:my|his|her|their)\\s+tooth\\s+(?:came|fell|was)\\b"), py_str("\\bgum\\b[^.?!]{0,30}\\b(?:tooth|teeth)\\b"), py_str("\\bcrowns?\\b"), py_str("\\b(?:son|daughter|kid|child|children)'?s?\\s+(?:front\\s+|back\\s+|adult\\s+|baby\\s+|milk\\s+)?(?:tooth|teeth)\\b[^.?!]{0,10}\\b(?:came|fell)\\s+out\\b")]));
    s_PET_TOOTH(py_tuple(vec![py_str("\\b(?:my|our|his|her|their|the)\\s+(?:\\w+\\s+){0,2}(?:cat|cats|dog|dogs|puppy|puppies|kitten|kittens|rabbit|rabbits|bunny|bunnies|hamster|guinea\\s+pig|horse|pony|parrot|ferret|gerbil|rat|mouse|tortoise|snake|lizard|pup)\\b[^.?!]{0,40}\\b(?:tooth|teeth|tusk|beak)\\b"), py_str("\\b(?:tooth|teeth|tusk)\\b[^.?!]{0,25}\\b(?:of|from)\\s+(?:my|our|the)\\s+(?:cat|dog|puppy|kitten|rabbit|hamster|horse|parrot)\\b"), py_str("\\b(?:cat|dog|puppy|kitten|rabbit|hamster|horse|parrot)'?s\\s+(?:\\w+\\s+){0,2}(?:tooth|teeth|tusk)\\b")]));
    s_HEAD_RED(py_tuple(vec![py_str("\\bknock(?:ed|s)?\\s+(?:him|her|them|my\\s*self|him\\s*self|her\\s*self)?\\s*out\\b"), py_str("\\b(?:was|were|been|got|had|being)\\s+knocked\\s+out\\b"), g_OUT_COLD_PERSON(), py_str("\\bunconscious\\b"), py_str("\\bpassed\\s+out\\b"), py_str("\\bblacked\\s+out\\b"), py_str("\\bkeeled\\s+over\\b"), py_str("\\bconfus(?:ed|ion)\\b(?!\\s+about\\s+(?!where\\b|who\\b|how\\b))"), py_str("\\bdidn'?t\\s+know\\s+(?:where|who)\\b"), py_str("\\b(?:not|isn'?t|isnt|ain'?t|wasn'?t|doesn'?t)\\s+making\\s+sense\\b"), py_str("\\b(?:talking|chatting)\\s+(?:rubbish|nonsense|gibberish)\\b"), py_str("\\bslurr\\w*\\b"), py_str("\\bdouble\\s+vision\\b"), py_str("\\bpupils?\\b.{0,15}\\b(?:different|unequal|dilated|blown|one\\s+(?:bigger|smaller|larger))\\b"), py_str("\\bdrows(?:y|iness|ily)?\\b"), py_str("\\b(?:very|really|so)\\s+sleepy\\b"), py_str("\\bcan'?t\\s+(?:wake|stay\\s+awake)\\b"), py_str("\\b(?:keep|keeps|kept)\\s+(?:on\\s+)?(?:dozing(?:\\s+off)?|nodding\\s+off|drifting\\s+off|falling\\s+asleep|zonking\\s+out)\\b"), py_str("\\b(?:dozing|nodding|drifting|zonking)\\s+(?:off|out)\\b"), py_str("\\bcan'?t\\s+keep\\s+(?:my|his|her|their)?\\s*eyes\\s+open\\b"), py_str("\\b(?:feel|feels|feeling|felt)\\s+(?:a\\s+bit\\s+|really\\s+|very\\s+|so\\s+)?(?:sleepy|drowsy|confused|not\\s+with\\s+it)\\b"), py_str("\\b(?:being|been|keeps?)\\s+sick\\b"), py_str("\\bvomit\\w*\\b"), py_str("\\b(?:throw(?:ing|n)?|threw)\\s+up\\b"), py_str("\\b(?:was|were|got|gets|getting|kept)\\s+(?:a\\s+bit\\s+|really\\s+|very\\s+)?sick\\b"), py_str("\\bfaint(?:ed|s|ing)?\\b"), py_str("\\bseizure\\b"), py_str("\\b(?:having|had)\\s+a\\s+fit\\b"), py_str("\\b(?:is|was|were|had|has|kept)\\s+fitting\\b(?!\\s+(?:in|on|into|his|her|their|my|your|the|a)\\b)"), py_str("\\bbleed(?:ing)?\\s+(?:from|out\\s+of)\\s+(?:the\\s+|his\\s+|her\\s+)?ears?\\b"), py_str("\\bclear\\s+fluid\\b"), py_str("\\bwarfarin\\b"), py_str("\\bblood\\s+thinners?\\b"), py_str("\\bclopidogrel\\b"), py_str("\\b(?:worse|worst)\\s+headache\\b"), py_str("\\bheadache\\s+(?:is\\s+)?(?:getting|getting\\s+)\\s+worse\\b"), py_str("\\bneck\\s+(?:is\\s+)?stiff\\b"), py_str("\\bcan'?t\\s+move\\s+(?:my|his|her)\\s+neck\\b")]));
    s_FALL_WORD_IDIOM(py_tuple(vec![py_str("\\b(?:slip|slips|slipped|slipping)\\s+up\\b"), py_str("\\btrip(?:s|ped|ping)?\\s+over\\s+(?:my|his|her|their|your|our)\\s+(?:words|tongue)\\b"), py_str("\\b(?:fall|fell|falling)\\s+(?:out|in)\\s+with\\b"), py_str("\\b(?:fall|fell|falling)\\s+(?:out|owt)\\s+(?:wi|wiv|wid|with|over|about)\\b")]));
    s_PET_ALL_RIGHT(py_tuple(vec![py_str("\\b(?:but|and|though|although|except|only)\\b.{0,40}\\b(?:fine|ok|okay|alright|all\\s+right|normal|himself|herself|no\\s+different|not\\s+himself)\\b"), py_str("\\b(?:seems?|looks?|appears?)\\s+(?:fine|ok|okay|alright|normal|himself|herself)\\b"), py_str("\\b(?:he|she|it|they|hes|shes|its|theyre)(?:'s|s)?\\s+(?:is|are)?\\s*(?:fine|ok|okay|alright|normal)\\b"), py_str("\\bjust\\s+(?:ate|eaten|had\\s+a\\s+(?:big\\s+)?meal|a\\s+big\\s+meal|tired|sleeping|worn\\s+out|getting\\s+old|old|woken\\s+up|having\\s+a\\s+lazy\\s+day)\\b"), py_str("\\b(?:1[0-9]|[2-9][0-9])\\s*(?:years?\\s+old|and\\s+stiff|now)\\b|\\b(?:is|getting)\\s+(?:old|stiff)\\b"), py_str("\\bstiff\\s+(?:in\\s+the\\s+morning|first\\s+thing)\\b"), py_str("\\b(?:the\\s+)?vet\\s+(?:said|says|told|checked|has\\s+seen|saw|thinks?)\\b")]));
    s_PET_RESPOND_AT(py_str("(?!\\s+(?:to|for|when|if)\\b[^.?!]{0,30}\\b(?:i|we|me|us|him|her|them|my|our|voice|calling|call|calls|say|says|said|speak|talk|asks?|asked|names?|treats?|food|dinner|toys?|recall|commands?|laser|catnip|nip|clicker|whistle|lead|ball|kibble|chews?|train\\w*|housetrain\\w*|sits?|stays?|heels?|tricks?|classes?|obedience|sessions?|cues?)\\b)"));
    s_CHOKE_OVER(py_tuple(vec![py_str("\\bchoked\\b.{0,40}\\b(?:at\\s+(?:lunch|dinner|breakfast)|yesterday|last\\s+night|this\\s+morning|earlier|a\\s+while\\s+ago|years\\s+ago|ages\\s+ago|once|before|again|last\\s+year|last\\s+week|last\\s+month|months?\\s+ago|when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little|young|small)|as\\s+a\\s+(?:kid|child|baby|teen|little\\s+one))\\b"), py_str("\\bchoked\\b[^.?!]{0,30}\\bin\\s+(?:19|20)\\d\\d\\b"), py_str("\\b(?:used\\s+to|always)\\s+choke\\b")]));
    s_CHRONIC_BREATH(py_tuple(vec![py_str("\\b(?:for|going\\s+on\\s+for|over\\s+the\\s+(?:last|past))\\s+(?:about\\s+|nearly\\s+|almost\\s+|over\\s+)?(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|a|an|a\\s+few|a\\s+couple\\s+of|several|many|last)\\s+(?:day|days|week|weeks|month|months|year|years)\\b"), py_str("\\bfor\\s+(?:the\\s+)?(?:last\\s+|past\\s+|few\\s+|several\\s+|a\\s+few\\s+|a\\s+couple\\s+of\\s+)?(?:days|weeks|months|years)\\b"), py_str("\\b(?:it'?s|its|it\\s+has|this\\s+has)\\s+been\\s+(?:like\\s+this\\s+|going\\s+on\\s+)?(?:about\\s+|nearly\\s+|over\\s+)?(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|a|an|a\\s+few|a\\s+couple\\s+of|several|many)\\s+(?:day|days|week|weeks|month|months|year|years)\\b"), py_str("\\bsince\\s+(?:last\\s+)?(?:christmas|new\\s+year|the\\s+summer|the\\s+holidays|easter|then|march|april|may|june|july|august|september|october|november|december)\\b"), py_str("\\bthese\\s+(?:last|past)\\s+(?:few\\s+|couple\\s+of\\s+|several\\s+)?(?:days|weeks|months)\\b")]));
    s_NIGHT_BREATH_OTHER(py_tuple(vec![py_str("\\b(?:my|our|his|her|their)\\s+(?:mum|mom|mother|dad|father|son|daughter|baby|child|kid|boy|girl|partner|husband|wife|brother|sister|gran|granny|nan|grandma|grandad|grandpa|friend|neighbour|neighbor|flatmate|roommate|niece|nephew|aunt|uncle)s?\\b"), py_str("\\b(?:he|she|they)\\s+(?:woke|woken|has\\s+woken|had\\s+woken)\\b")]));
    s_NIGHT_BREATH_PAST(py_tuple(vec![add(&add(&py_str("\\A(?!.*?(?:"), &mcall(&py_str("|"), "join", &[g_NIGHT_BREATH_OTHER()])), &py_str("))(?![^.?!]*\\bfor\\s+(?:breath|air)\\b).*?\\b(?:woke|woken)\\s+up\\b[^.?!]{0,40}\\b(?:breath\\w*|gasp\\w*)\\b"))]));
    s_NIGHT_BREATH_RED(py_tuple(vec![py_str("\\b(?:can'?t|cannot|can\\s+not|unable\\s+to|struggling\\s+to)\\s+breath\\w*\\b|\\b(?:blue|lips?|can'?t\\s+speak|cannot\\s+speak|chest\\s+pains?|right\\s+now|at\\s+the\\s+moment|as\\s+we\\s+speak|just\\s+happened|a\\s+minute\\s+ago|still|getting\\s+worse|stopped\\s+breathing|not\\s+breathing|isn'?t\\s+breathing|asthma\\s+attack|wheez\\w*|blood|anaphyla\\w*|epipen|swell\\w*)\\b")]));
    s_TALK_HABIT(py_tuple(vec![py_str("\\b(?:when|whenever|while|after|if)\\s+(?:i|we)\\s+(?:talk|talks|speak|sing|read|laugh|shout|chat)\\b[^.!?]{0,20}\\b(?:for\\s+(?:a\\s+long\\s+time|a\\s+while|ages|long|hours)|a\\s+lot)\\b"), py_str("\\b(?:talk|talking|sing|singing|read|reading|laugh|laughing)\\w*\\s+(?:for|over)\\s+(?:a\\s+long\\s+time|a\\s+while|ages)\\b"), py_str("\\b(?:out\\s+loud|aloud|by\\s+heart)\\b")]));
    s_BREATH_LAUGH(py_tuple(vec![py_str("\\b(?:when|whenever|while|after|from|every\\s+time|each\\s+time|if)\\b[^.!?]{0,20}\\blaugh\\w*\\b"), py_str("\\blaugh\\w*\\b[^.!?]{0,25}\\b(?:breath(?:less)?|puffed|winded|gasp\\w*)\\b"), py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+(?:catch|get|find)\\s+(?:my|his|her|their|a)\\s+breath\\s+laugh\\w*\\b")]));
    s_BREATH_LAUGH_REPLY(py_tuple(vec![py_str("Laughing hard empties the lungs and leaves anyone gasping for a few seconds; it passes once the laughing stops, and it is not a sign of bad lungs on its own. Worth a GP if the breathlessness turns up when nobody is laughing, if a cough or a wheeze comes with it, or if it is new and getting worse."), py_str("A hard laugh leaves nobody with air for a moment, and it settles as soon as the laughing stops, so there is usually nothing in it. Worth a GP if the breathlessness turns up without the laugh, or comes with a cough, a wheeze or chest pain.")]));
    s_CHOKE_AIR_OK(py_tuple(vec![py_str("\\b(?:can|could|able\\s+to)\\s+(?:still\\s+)?(?:cough|talk|speak|breathe|say)\\w*\\b"), py_str("\\bcoughing\\s+(?:well|fine|ok|okay|strongly|strong|up)\\b"), py_str("\\b(?:he|she|they|i)\\s+(?:is|are|am|'s)\\s+(?:coughing|talking|speaking)\\b"), py_str("\\bstill\\s+(?:coughing|talking|speaking)\\b")]));
    s_CHOKE_NOT_PERSON(py_tuple(vec![py_str("\\b(?:kitchen\\s+sink|sink|drains?|pipes?|toilet|loo|garden|lawn|flowerbeds?|engine|filter|radiator|chimney|gutter\\w*|weeds?|moss|bog|traffic|paperwork|inbox|diary|sourdough|mower|lawnmower|hoover|vacuum|dishwasher|printer|motorbike|scooter|tears?)\\b"), py_str("\\b(?:bath\\s*tub|bathtub|baths?|tub|basins?|hand\\s?basins?|wash\\s?basins?|showers?|shower\\s+tray|washing\\s+machine|washer|hose|pipework|downpipe|gulley|drainpipe|water\\s+butt|pump|cistern|plughole|plug\\s+hole)\\b"), py_str("\\bchoked\\s+(?:with|on)\\s+(?:weeds|leaves|moss|traffic|paperwork|debt|hair|roots|fat)\\b")]));
    s_CHOKE_PREVENT(py_tuple(vec![py_str("\\bhow\\s+(?:do|can|should|could|would)\\s+(?:i|we|you)\\s+(?:stop|prevent|avoid|keep)\\b"), py_str("\\bhow\\s+to\\s+(?:stop|prevent|avoid|keep)\\b"), py_str("\\b(?:stop|prevent|avoid|keep)\\s+(?:my|him|her|them|a|the|your|me\\s+from)\\s+(?:\\w+\\s+){0,2}chok"), py_str("\\bbest\\s+way\\s+to\\s+(?:stop|avoid|prevent|keep)\\b"), py_str("\\b(?:is|was|am)\\s+(?:my|the|a|she|he|it)\\s+(?:\\w+\\s+){0,2}choking\\s+if\\b"), py_str("\\bwhy\\s+do\\s+(?:babies|toddlers|kids|children)\\b"), py_str("\\b(?:so|that)\\s+(?:he|she|they|it)?\\s*(?:doesn'?t|does\\s+not|wont|won'?t|never)\\s+choke\\b"), py_str("\\bchoking\\s+(?:hazard|risk)\\b")]));
    s_CHOKE_SETTLED(py_tuple(vec![py_str("\\blaugh\\w*\\b"), py_str("\\bgiggl\\w*\\b"), py_str("\\bchuckl\\w*\\b"), py_str("\\b(?:got|get|getten)\\s+it\\s+(?:out|up)\\b"), py_str("\\bcoughed\\s+it\\s+up\\b"), py_str("\\b(?:it|came|went)\\s+(?:up|out)\\b"), py_str("\\b(?:he|she|they|i)(?:'s|s|\\s+is|\\s+am|\\s+are)?\\s+(?:fine|ok|okay|alright)\\s+now\\b"), py_str("\\bthat\\s+was\\s+close\\b")]));
    s_CHOKE_PERSON(py_tuple(vec![py_str("\\b(?:my|his|her|their|our|the)\\s+(?:\\w+\\s+){0,1}(?:mum|mom|mother|dad|father|son|daughter|husband|wife|partner|brother|sister|friend|baby|child|kid|boy|girl|man|woman|gran|granny|nan|grandad|grandma|grandpa|toddler|newborn|infant|mate|neighbou?r)\\b"), py_str("\\b(?:he|she|they)\\s+(?:is\\s+|are\\s+|was\\s+|were\\s+)?choking\\b"), py_str("\\b(?:he|she|they)\\s+choked\\b")]));
    s_FLOPPY_THING(py_tuple(vec![py_str("\\bflopp\\w*[\\s-]+(?:disk|disc|drive|hat|hats|brim|brimmed|hair|haired|fringe|ears?|eared|bow|toy|teddy|puppet|sleeve|leaves|book|bag|flag|cover|blanket|flower|plant|bunny|rabbit|dog|puppy|kitten|cat|doll|ears|eared)\\b"), py_str("\\b(?:hair|lettuce|salad|leaves|plant|flowers?|washing|sheets|flag|sail|balloon|fabric|battery)\\b.{0,20}\\b(?:limp|floppy)\\b")]));
    s_FLOPPY_SLEEPY(py_tuple(vec![py_str("\\bflopp\\w*[^.]{0,40}\\bwhen\\s+(?:they\\s+are\\s+|she\\s+is\\s+|he\\s+is\\s+|hes\\s+|shes\\s+)?(?:very\\s+)?(?:sleepy|sleeping|asleep|drowsy|tired|nodding\\s+off|dropping\\s+off)(?:\\s+(?:he|she|they|it)\\s+(?:falls?|drops?|nods?)\\s+(?:a\\s+)?(?:asleep|off|sleep)\\b)?"), py_str("\\b(?:goes|go|gets|get|turns?)\\s+flopp\\w*[^.]{0,40}\\bwhen\\s+(?:very\\s+)?(?:sleepy|sleeping|asleep|drowsy|tired|nodding\\s+off|dropping\\s+off)\\b"), py_str("\\bflopp\\w*[^.]{0,40}\\bwhen\\s+(?:he|she|they|it)\\s+(?:falls?|drops?|nods?)\\s+(?:a\\s+)?(?:asleep|off|sleep)\\b"), py_str("\\b(?:goes|go|gets|get|turns?)\\s+flopp\\w*[^.]{0,40}\\b[^.]{0,20}?\\b(?:falls?|drops?|nods?)\\s+(?:a\\s+)?(?:asleep|off)\\b")]));
    s_FLOPPY_RED_FLAG(py_tuple(vec![py_str("\\bwont\\s+wake\\b"), py_str("\\bwon'?t\\s+wake\\b"), py_str("\\bhard\\s+to\\s+wake\\b"), py_str("\\bnot\\s+waking\\b"), py_str("\\bnot\\s+feeding\\b"), py_str("\\bwont\\s+feed\\b"), py_str("\\bwon'?t\\s+feed\\b"), py_str("\\bstopped\\s+feeding\\b"), py_str("\\btemperature\\b"), py_str("\\bfever\\b"), py_str("\\bbreath\\w*\\b"), py_str("\\bunresponsive\\b"), py_str("\\blife-?less\\b"), py_str("\\bvery\\s+sleepy\\b"), py_str("\\btoo\\s+sleepy\\b")]));
    s_HEAT(py_tuple(vec![py_str("\\bheat\\s?stroke\\b"), py_str("\\bheat\\s+exhaustion\\b"), py_str("\\bsun\\s?stroke\\b"), py_str("\\b(?:overheated|over\\s?heated)\\b.{0,25}\\b(?:sick|dizzy|faint|confused|headache)\\b"), py_str("\\b(?:sick|dizzy|faint|confused)\\b.{0,25}\\b(?:from|in)\\s+(?:the\\s+)?heat\\b"), py_str("\\b(?:overheat\\w*|boiling\\s+hot)\\b.{0,30}\\b(?:confused|dizzy|faint\\w*|sick|vomit\\w*|collapse\\w*|unconscious|can'?t\\s+stand|couldn'?t\\s+stand|cant\\s+stand|headache)\\b"), py_str("\\b(?:confused|dizzy|faint\\w*|vomit\\w*|collapse\\w*)\\b.{0,30}\\b(?:overheat\\w*|boiling\\s+hot)\\b"), py_str("\\b(?:baby|babies|toddler|newborn|infant|child|kid|son|daughter|boy|girl)\\b.{0,40}\\b(?:boiling\\s+hot|burning\\s+up|overheat\\w*)\\b")]));
    s_SELF_HARM_TOPIC_OFF(py_tuple(vec![py_str("\\b(?:course|essay|dissertation|talk|lecture|workshop|training|charity|article|book|stud(?:y|ies)|research|film|documentary|paper|conference|lesson|class|exam|revision|homework|awareness)\\b"), py_str("\\bself[-\\s]?harm\\s+(?:counsell?or|nurse|worker|charity|helpline|awareness|prevention|team|service)\\b")]));
    s_SELF_HARM_LOOKALIKE(py_tuple(vec![py_str("\\b(?:on|with|against|from|by|through)\\s+(?:a|an|the|some|his|her|their)?\\s*(?:tin|knife|blade|glass|sharp|nail|wire|cooker|oven|iron|kettle|pan|nettles?|bike|wall|door|stone|gravel|rose|thorn|hook|scissors|grater|hob|razor|bat|ball|toy|swing|frame|trampoline|slide|scooter|skateboard|climbing\\s+frame|bouncy\\s+castle|barbed\\s+wire)\\b"), py_str("\\b(?:at|playing|during|in)\\s+(?:netball|football|rugby|hockey|gym|sport|training|school|playtime|the\\s+park|the\\s+playground)\\b"), py_str("\\b(?:play|plays|playing|played)\\s+(?:\\w+\\s+){0,2}(?:netball|football|rugby|hockey|sport|sports|tennis|basketball|cricket|golf|squash|badminton|rounders|outside|the\\s+park)\\b"), py_str("\\bin\\s+(?:his|her|their|my)\\s+sleep\\b"), py_str("\\bwhile\\s+(?:he|she|they|i)\\s+(?:'?s\\s+|is\\s+|was\\s+)?(?:asleep|sleeping)\\b"), py_str("\\bin\\s+(?:his|her|their|my)\\s+bed\\b"), py_str("\\b(?:shaving|chopping|cooking|gardening|diy|ironing|baking|haircut)\\b"), py_str("\\bby\\s+accident\\b"), py_str("\\baccidentally\\b")]));
    s_AN_ITCH_LOOKALIKE(py_tuple(vec![py_str("\\b(?:scratch\\w*|rub\\w*|pick\\w*)\\b[^.?!]{0,25}?\\b(?:itch(?:y|ing|es)?|eczema|rash|spots|scabs|hives|dandruff|dry\\s+skin|head\\s+lice|nits|threadworms?|chicken\\s?pox|scabies|psoriasis|ringworm|allerg(?:y|ic)|sore|at\\s+night|at\\s+bedtime|all\\s+night|every\\s+night|in\\s+the\\s+night|a\\s+lot|all\\s+the\\s+time|constantly|non[\\s-]?stop)\\b"), py_str("\\b(?:itch(?:y|ing|es)?|eczema|rash|spots|scabs|hives|dandruff|dry\\s+skin|head\\s+lice|nits|threadworms?|chicken\\s?pox|scabies|psoriasis|ringworm|allerg(?:y|ic))\\b[^.?!]{0,30}?\\b(?:scratch\\w*|rub\\w*|pick\\w*)\\b")]));
    s_A_WISH_OR_TALK_OF_IT(py_tuple(vec![py_str("\\b(?:want|wants|wanted|wanting|wish|wishes|talk|talks|talking|mention|mentions|mentioned|say|says|said|saying|tells?|told|thinking|thinks|thought|feels?|keeps?)\\b[^.?!]{0,22}\\b(?:die|dying|end\\s+(?:it|it\\s+all|things)|ending\\s+it|suicide|suicidal|kill\\s+(?:my|him|her|them)sel(?:f|ves)|not\\s+be\\s+(?:here|around))\\b")]));
    s_DISTRESS_BESIDE_IT(py_tuple(vec![py_str("\\b(?:crying|sobbing|in\\s+tears|tearful|depressed|miserable|hopeless|worthless|can'?t\\s+cope|cant\\s+cope|falling\\s+apart|numb|suicidal|self[\\s-]?harm|want\\s+to\\s+die|killing\\s+my\\s*self)\\b"), py_str("\\b(?:dont|don'?t|do\\s+not)\\s+want\\s+to\\s+be\\s+here\\b"), py_str("\\bbetter\\s+off\\s+without\\s+me\\b"), py_str("\\bno\\s+point\\s+(?:in\\s+)?(?:going\\s+on|living)\\b"), py_str("\\b(?:cant|can'?t)\\s+go\\s+on\\b"), py_str("\\bend\\s+it\\s+all\\b"), py_str("\\b(?:worried|worrying|concerned|frightened|scared|afraid)\\s+(?:about|for|that)\\b"), py_str("\\b(?:not|isn'?t|hasn'?t|has\\s+not)\\s+(?:been\\s+)?(?:her|him|them)sel(?:f|ves)\\b"), py_str("\\b(?:gone\\s+quiet|has\\s+been\\s+quiet|stopped\\s+eating|not\\s+eating|off\\s+her\\s+food|off\\s+his\\s+food)\\b")]));
    s_WISH_DEAD_HYPERBOLE(py_tuple(vec![py_str("\\bso\\s+(?:tired|exhausted|knackered|worn\\s+out|sleepy|drowsy|bored|hungry|starving|thirsty|full|stuffed|hot|cold|freezing|sore|aching|busy|snowed\\s+under|laughing|embarrassed|hungover|old|fat|slow)\\b(?!\\s+of\\s+(?:it|all|life|everything|this|the\\s+world)\\b)[^.?!]{0,40}\\bwish\\w*\\b"), py_str("\\blaugh(?:ed|ing|s)?\\b[^.?!]{0,30}\\bwish\\w*\\b")]));
    s_WISH_DEAD_WHEN(py_str("(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec|monday|tuesday|wednesday|thursday|friday|saturday|sunday|(?:my|our|the|his|her)?\\s*(?:birthday|wedding\\s+day)|christmas|xmas|easter|(?:the\\s+)?new\\s+years?|spring|summer|autumn|winter|half\\s+term|(?:the\\s+)?weekend|today|tonight|tomorrow|this\\s+(?:week|month|year|weekend|evening|morning)|next\\s+\\w+|a\\s+(?:day|week|fortnight|month|year|minute|hour)|an\\s+hour|\\d+\\s*(?:days?|weeks?|months?|years?|hours?)|the\\s+end\\s+of\\s+the\\s+(?:week|month|year))"));
    s_WISH_DEAD_PLACE(py_tuple(vec![add(&add(&py_str("\\bi\\s+wish\\w*\\s+i\\s+(?:wasnt|wasn'?t|was\\s+not)\\s+(?:here|around)\\s+(?:at\\b(?!\\s*(?:all\\b|the\\s+time\\b))|in|on|with|near)\\s+(?!(?:the\\s+)?"), &g_WISH_DEAD_WHEN()), &py_str("\\b)")), add(&add(&py_str("\\bi\\s+(?:dont|don'?t|didnt|didn'?t)\\s+want\\s+to\\s+be\\s+(?:here|around)\\s+(?:at\\b(?!\\s*(?:all\\b|the\\s+time\\b))|in|on|with|near)\\s+(?!(?:the\\s+)?"), &g_WISH_DEAD_WHEN()), &py_str("\\b)"))]));
    s_LOSS_SAID(py_tuple(vec![py_str("\\b(?:died|dies|dying|dead|passed\\s+(?:away|on)|funeral|burial|cremation|buried|killed\\s+(?:him|her|them)self|killed\\s+himself|took\\s+(?:his|her|their)\\s+own\\s+life|ended\\s+(?:his|her|their)\\s+own\\s+life|committed\\s+suicide|lost\\s+(?:my|our|his|her|their|the)\\s+\\w+|we\\s+lost\\s+\\w+)\\b")]));
    s_SELF_RUMINATION(py_tuple(vec![py_str("\\b(?:keep|kept|still|can'?t\\s+stop|cannot\\s+stop)\\b[^.?!]{0,40}\\b(?:thinking|think|thought|wondering|imagining)\\b"), py_str("\\b(?:thinking|thought|wondering)\\s+(?:about|of)\\s+(?:doing\\s+)?it\\b"), py_str("\\bmade\\s+me\\s+(?:think|wonder)\\s+(?:about|of)\\b"), py_str("\\bthink(?:s|ing)?\\s+(?:about|of)?\\s*it\\s+(?:every|all\\s+the|each)\\s+(?:night|day|time|morning)\\b"), py_str("\\b(?:feel\\s+like|felt\\s+like|want\\s+to|wanted\\s+to|going\\s+to|gonna|might|tempted\\s+to)\\s+(?:do|doing|try|trying|end)\\s+(?:it|something|this|things)\\b")]));
    s_THING_THAT_IS_SUICIDE(py_str("(?:job|jobs|work|shift|shifts|rota|commute|drive|driving|cycle|ride|run|route|road|lane|motorway|m25|m1|m6|a1|a3|a4|climb|descent|hike|walk|hill|slope|piste|traffic|weather|plan|idea|option|choice|move|career|course|diet|exam|interview|pitch|season|fixture|trip|deal|deadline|rent|mortgage|timetable|schedule|sale|launch|market|game|match|tactic|team)"));
    s_SUICIDE_ABOUT_A_THING(py_tuple(vec![add(&add(&py_str("\\b(?:this|that|the|my|his|her|our|their|your|a|an)\\s+(?:\\w+\\s+){0,2}"), &g_THING_THAT_IS_SUICIDE()), &py_str("\\b[^.!?]{0,30}\\b(?:is|was|'s|would\\s+be|will\\s+be|must\\s+be)\\s+(?:pure\\s+|total\\s+|absolute\\s+|outright\\s+)?suicid(?:e|al)\\b")), py_str("\\bcareer\\s+suicid(?:e|al)\\b"), py_str("\\b(?:it'?s|its|it\\s+is|it\\s+was|it'?d\\s+be|it\\s+would\\s+be)\\s+(?:pure\\s+|total\\s+|absolute\\s+)?suicid(?:e|al)\\s+to\\b"), py_str("\\b(?!(?:going|carrying)\\s+on\\s+(?:is|was|would|'?s|and|like|any|much\\s+further)\\b)(?:going|driving|cycling|walking|climbing|running|commuting|working|eating|drinking|flying|sailing|swimming|skiing|snowboarding|surfing|dating|studying|shopping|travelling|traveling|moving|wearing|betting|buying|opening|marrying|getting|booking|hiking|camping|jogging|rowing|parking|queuing)(?:\\s+(?:out|in|on|up|down|through|with|to|for|at|off|across|around|round|over|back)){1,2}\\s+(?:(?:that|this|these|those|the|a|an|my|his|her|their|our|your)\\s+\\w|work\\b|school\\b|college\\b|uni\\b)[^.?!]{0,40}\\b(?:is|was|would\\s+be|'?s)\\s+(?:pure\\s+|total\\s+|absolute\\s+|outright\\s+)?suicid(?:e|al)\\b"), py_str("\\b(?:doing|trying|taking|starting|going)\\s+(?:that|this|it)\\b[^.?!]{0,30}\\b(?:is|was|would\\s+be|'?s)\\s+(?:pure\\s+|total\\s+|absolute\\s+|outright\\s+)?suicid(?:e|al)\\b")]));
    s_DIE_OF_THE_FEELING(py_tuple(vec![py_str("\\b(?:want\\w*|wish\\w*|feel\\w*|felt|think\\w*|thought|going|gonna|nearly|almost)\\b(?:[\\s,]+[a-z']+){0,3}\\s+(?:to\\s+)?(?:die|dying|died)\\s+(?:of|from)\\s+(?:embarrass\\w*|boredom|thirst|curiosity|laughter|shame|fright)\\b"), py_str("\\b(?:die|dyed|dyeing|dying)\\s+my\\s+hair\\b")]));
    s_WISH_NAMED(py_tuple(vec![py_str("\\bsuicid(?:e|al)\\b"), py_str("\\bself[-\\s]?harm(?:ing|ed)?\\b"), py_str("\\b(?:want|wanted|wanting|need|needed|like)\\s+to\\s+die\\b(?!\\s+(?:of|laughing))"), py_str("\\b(?:think|thinks|thinking|thought|thoughts?)\\s+(?:about|of)\\s+(?:dying|suicide|self[-\\s]?harm|killing\\s+my\\s*self|ending\\s+it)\\b"), py_str("\\bend(?:ing)?\\s+(?:it|my\\s+(?:own\\s+)?life|it\\s+all)\\b"), py_str("\\b(?:don'?t|do\\s+not|doesn'?t)\\s+want\\s+to\\s+(?:live|be\\s+here|exist|wake\\s+up|go\\s+on)\\b"), py_str("\\bbetter\\s+off\\s+(?:dead|without\\s+me|if\\s+i\\s+was\\s+gone|if\\s+i\\s+weren'?t\\s+here)\\b"), py_str("\\b(?:hurt|hurting|cut|cutting|kill|killing|harm|harming)\\s+my\\s*self\\b")]));
    s_FIT_ALL_BETTER(py_tuple(vec![py_str("\\b(?:is|was|feels?|seems?|looks?)\\s+(?:much\\s+|totally\\s+|completely\\s+|absolutely\\s+)?(?:fine|ok|okay|alright|better|himself|herself|themselves|normal)\\s+(?:again\\s+)?now\\b"), py_str("\\b(?:now|since\\s+then)\\s+(?:he|she|they)\\s+(?:is|are|seems?|feels?|was)\\s+(?:fine|ok|okay|alright|better|normal)\\b")]));
    s_STILL_GOING(py_tuple(vec![py_str("\\b(?:can'?t|cannot|can\\s+not|unable\\s+to)\\s+(?:get\\s+up|move|walk|stand|feel)\\b"), py_str("\\b(?:stuck|trapped)\\b"), py_str("\\bwon'?t\\s+(?:wake|get\\s+up)\\b"), py_str("\\bis\\s+still\\s+(?:on\\s+the\\s+floor|there|stuck|bleeding|in\\s+pain)\\b"), py_str("\\bhasn'?t\\s+(?:stopped|shifted|got\\s+up)\\b"), py_str("\\bstill\\s+(?:in|on)\\s+(?:bed|the\\s+floor)\\b")]));
    s_OLD_TIME(py_tuple(vec![py_str("\\b(?:in\\s+)?(?:19|20)\\d\\d\\b"), py_str("\\b(?:years?|months?|decades?|weeks?)\\s+ago\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:\\d+|a\\s+kid|a\\s+child|young|little|at\\s+school)\\b"), py_str("\\bwhen\\s+(?:i|he|she|they|we)\\s+(?:was|were)\\s+(?:\\d+|a\\s+kid|a\\s+child|a\\s+baby|a\\s+toddler|a\\s+tot|an\\s+infant|young|little|small|at\\s+school|born)\\b"), py_str("\\blast\\s+(?:year|summer|winter|decade|century|week|month)\\b"), py_str("\\bused\\s+to\\b"), py_str("\\b(?:fell|fallen|had\\s+a\\s+fall|broke|fractured|dislocated|sprained)\\b[^.?!]{0,30}\\blast\\s+(?:week|month)\\b")]));
    s_DRUG_NAME(py_str("(?:pills?|tablets?|capsules?|paracetamol|ibuprofen|aspirin|painkillers?|sleeping\\s+pills?|antidepressants?|co-?codamol|codeine|tramadol|morphine|methadone|insulin|lithium|valium|diazepam|temazepam|xanax|heroin|heroine|cocaine|opioids?|opiates?|drugs?|meds|medication|medicines?|antihistamines?|antibiotics?|iron\\s+tablets|vitamins?|laxatives?|diuretics?|beta\\s+blockers?|statins?)"));
    s_EDGE_OF_A_DROP(py_tuple(vec![py_str("\\b(?:stood|standing|sat|sitting|walked)\\b[^.?!]{0,10}\\b(?:on|at)\\s+the\\s+edge\\s+of\\s+(?:a\\s+|the\\s+)?(?:bridge|roof|building|cliff|car\\s*park)\\b"), py_str("\\b(?:stood|standing|sat|sitting|walked)\\b[^.?!]{0,10}\\b(?:on|at)\\s+the\\s+edge\\s+of\\s+(?:a\\s+|the\\s+)?(?:platform|tracks?)\\b(?=[^.?!]*\\b(?:thought|thinking|wanted|want|nearly|almost|about\\s+to|going\\s+to|close\\s+to|jump|jumped|looked\\s+down|someone|police|pulled|grabbed|swaying|drunk|end\\s+it|easy)\\b)")]));
    s_END_IT_WITH_A_THING(py_tuple(vec![py_str("\\bend(?:ed|ing|s)?\\s+it\\s+(?:with|by|using|between)\\b"), py_str("\\bend(?:ed|ing|s)?\\s+it\\s+(?:by|off|on|under|in)\\s+(?:the\\s+)?(?:bridge|train|tracks|railway)\\b")]));
    s_END_IT_MEANS(py_tuple(vec![py_str("\\b(?:pills?|tablets?|painkillers?|paracetamol|antidepressants?|meds?|medication|an?\\s+overdose|overdosing|a\\s+knife|the\\s+knife|a\\s+razor|a\\s+blade|a\\s+rope|a\\s+belt|a\\s+ligature|a\\s+gun|a\\s+shotgun|gas|the\\s+car|a\\s+car|the\\s+train|a\\s+train|drowning|a\\s+bridge|the\\s+bridge|something\\s+sharp|hang(?:ing|ed)?|alcohol|drink|drinking|vodka|whisky|whiskey|gin|tablets|my\\s+wrists?|my\\s+own\\s+life)\\b")]));
    s_RECENT_TIME(py_tuple(vec![py_str("\\b(?:yesterday|last\\s+night|last\\s+evening|this\\s+(?:morning|afternoon|evening)|earlier(?:\\s+today)?|a\\s+few\\s+(?:days?|weeks?)\\s+ago|a\\s+couple\\s+of\\s+(?:days?|weeks?)\\s+ago|recently|on\\s+(?:mon|tues|wednes|thurs|fri|satur|sun)day|(?:at|over|during)\\s+the\\s+weekend|last\\s+weekend)\\b")]));
    s_SELF_HARM_SCARS(py_tuple(vec![py_str("\\b(?:have|got|has|had|covered\\s+in|full\\s+of)\\b[^.?!]{0,15}\\b(?:cuts|scars|scratches|burn\\s*marks|slashes)\\b[^.?!]{0,12}\\b(?:on|across|down|up)\\s+(?:both\\s+)?(?:my|his|her|their|your)\\s+(?:wrists?|arms?|forearms?|thighs?|tummy|stomach|belly)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:wrists?|forearms?|thighs?)\\b[^.?!]{0,10}\\b(?:are|is|r)\\s+(?:all\\s+)?(?:scars?|scarred|cut\\s+up|covered\\s+in\\s+scars?)\\b")]));
    s_SELF_HARM_SCARS_NOT(py_tuple(vec![py_str("\\b(?:from|off)\\s+(?:the|a|an|my|our|his|her|their)\\s+(?:cat|dog|puppy|kitten|pet|rabbit|hamster|hedgehog|parrot|budgie|horse|pony|bike|motorbike|car|van|ladder|shelf|barbecue|oven|hob|cooker|kettle|toaster|knife|blade|glass|tin|can|tool|drill|saw|mower|gate|fence|nails?|rose|bush|brambles?|thorns?|shower|tap|bath|door|window|cupboard|drawer|work|job|office|school|nursery|garden|allotment|kitchen|garage|shed|building\\s+site|farm|football|rugby|hockey|netball|climbing|cycling|gardening|diy|cooking|chopping|shaving|washing\\s+up|hoovering|cleaning|moving|flat|house|home)\\b"), py_str("\\b(?:at|from)\\s+(?:work|the\\s+garden|the\\s+allotment|the\\s+tip|the\\s+shops?)\\b"), py_str("\\b(?:on|off)\\s+(?:a|an|the|some)\\s+(?:knife|blade|glass|tin|can|nail|screw|wire|fence|barbed\\s+wire|thorn|rock|stone|kerb|roof|wall|door|drawer|bike|motorbike|car|mower|tool)\\b"), py_str("\\b(?:while|when)\\s+(?:i\\s+was\\s+|i'm\\s+)?(?:cooking|chopping|shaving|carving|cutting|peeling|washing\\s+up|gardening|diy|working|cleaning|hoovering|painting|decorating)\\b"), py_str("\\b(?:acne|spot|spots|chickenpox|eczema|psoriasis|rash|allergy|allergic|hives|itch\\w*|surgery|operation|scarred\\s+from\\s+the)\\b")]));
    s_SPOKEN_ATTEMPT_UNDATED(py_tuple(vec![g_SELF_HARM_SCARS(), py_str("\\b(?:tried|try|tries|trying|attempted|attempt|attempts|attempting|went\\s+to|nearly|almost)\\b[^.?!]{0,25}\\b(?:end(?:ed|ing)?\\s+it|kill(?:ing)?\\s+my\\s*self|end\\s+my\\s+life|(?:take|took)\\s+my\\s+own\\s+life|end(?:ed)?\\s+my\\s+life|suicide|hang(?:ing|ed)?\\s+my\\s*self|jump\\s+off)\\b")]));
    s_PAST_ATTEMPT_NOT(py_tuple(vec![py_str("\\b(?:on\\s+(?:(?:a|an|the|some|my)\\s+)?(?!purpose\\b)[a-z]+|while\\s+(?:cooking|chopping|shaving|carving|cutting|peeling|washing\\s+up|gardening|diy|working)|chopping|carving|peeling|shaving|gardening|trimming|pruning)\\b"), py_str("\\b(?:laughing|of\\s+laughter|off\\s+my\\s+bike|on\\s+the\\s+stairs|tripping|embarrass\\w*|mortified)\\b")]));
    s_PAST_WISH(py_tuple(vec![py_str("\\bi\\b[^.?!]{0,24}\\b(?:wanted|wanting|thought\\s+about|thought\\s+of|thoughts\\s+about|kept\\s+thinking\\s+about|kept\\s+thinking\\s+of|thinking\\s+about|thinking\\s+of|planned|planning|was\\s+going\\s+to|was\\s+planning\\s+to|felt\\s+like|considered|dreamt\\s+about)\\b[^.?!]{0,18}\\b(?:kill\\s+my\\s*self|killing\\s+my\\s*self|end\\s+it\\s+all|ending\\s+it|end\\s+my\\s+life|ending\\s+my\\s+life|die|dying|suicide|suicidal|not\\s+be\\s+here|disappear|hurt\\s+my\\s*self|cutting\\s+my\\s*self|self\\s*harm)\\b"), py_str("\\bi\\b[^.?!]{0,20}\\b(?:felt|was|were|have\\s+been|had\\s+been|been|got)\\b[^.?!]{0,8}\\b(?:suicidal|feeling\\s+suicidal)\\b")]));
    s_EFFORT_AFTER(stdlib_call("re", "compile", &[py_str("\\b(?:nearly|almost)\\b[^.?!]{0,24}\\b(?:killed|kill)\\s+my\\s*self\\s+(?:at\\s+the\\s+(?:gym|pool|park|tip)|on\\s+(?:the\\s+|my\\s+|a\\s+)?(?:bike|bicycle|stairs|steps|staircase|ladder|roof|treadmill|run|road|motorway|trampoline|pitch|hill|scaffold)\\b|off\\s+(?:the\\s+|my\\s+|a\\s+)?(?:(?:back|front)\\s+of\\s+(?:a|the|my)?\\s*)?(?:bike|bicycle|motorbike|motorcycle|moped|scooter|horse|pony|ladder|roof|wall|scaffold|trampoline|skateboard|skis|boat|truck|lorry|van|trailer)\\b|(?P<doing>[a-z']+ing)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_WISH(add(&g_PAST_WISH(), &crate::gen::modules::safety_cards::g_COUNTERFACTUAL_WISH()));
    s_PAST_WISH_NOT(py_tuple(vec![py_str("\\b(?:of|from)\\s+(?:embarrass\\w*|boredom|thirst|curiosity|laughter|shame|fright)\\b"), py_str("\\b(?:die|dying|died)\\s+(?:laughing|of)\\b"), py_str("\\bthought\\s+(?:i|that\\s+i)\\s+(?:was|were)\\s+(?:going\\s+to|about\\s+to|gunna)\\s+die\\b"), py_str("\\bfelt\\s+like\\s+(?:i|that\\s+i)\\s+(?:was|were)\\s+(?:going\\s+to|about\\s+to)\\s+die\\b"), py_str("\\b(?:scared|afraid|terrified|convinced)\\b")]));
    s_OVERDOSE_FOOD(py_tuple(vec![py_str("\\boverdos(?:e|ed|ing)\\b[^.?!]{0,12}\\b(?:on|of|with)\\s+(?:the\\s+|a\\s+|some\\s+)?(?:cake|cakes|chocolate|sweets|candy|biscuits?|cookies|crisps|chips|chinese|curry|pizza|ice\\s*cream|cheese|coffee|tea|netflix|tv|telly|sugar|bread|pastr(?:y|ies)|do(?:ugh)?nuts|burgers?|takeaways?|naps|love|pies?|pudding|trifle|turkey|roast|stuffing|mince|gravy|easter\\s+eggs?)\\b")]));
    s_OVERDOSE_DRUG(py_tuple(vec![py_str("\\b(?:pills?|tablets?|paracetamol|ibuprofen|painkillers?|meds|medicines?|sleeping\\s+pills?|antidepressants?|insulin|heroin|heroine|cocaine|methadone|tramadol|codeine|morphine|opioid|opiates?|drugs?|liquid|bottle|packet|pack|box|strip|blister)\\b"), py_str("\\b(?:alcohol|vodka|whisky|whiskey|wine|beer|gin|rum|brandy|tequila|lager|cider|champagne|prosecco|ros\\u00e9|spirits?|drink|drinking|booze)\\b")]));
    s_CHEST_WORK(py_tuple(vec![py_str("\\bchest\\b.{0,30}\\b(?:after|during|when)\\s+(?:the\\s+|a\\s+|my\\s+)?(?:gym|run|running|jog|jogging|workout|exercise|exercising|swim|swimming|cycle|cycling|football|netball|walk|walking|stairs|climbing|training|weights|lifting)\\b"), py_str("\\bchest\\b.{0,30}\\b(?:when\\s+i\\s+(?:run|walk|climb|exercise|train|cycle|swim|lift))\\b"), py_str("\\bafter\\s+(?:the\\s+|a\\s+)?(?:gym|run|workout|training)\\b.{0,30}\\bchest\\b")]));
    s_CHEST_SLOW(py_tuple(vec![py_str("\\bchest\\b.{0,45}\\b(?:for\\s+(?:a\\s+(?:few|couple)\\s+(?:days|weeks)|\\d+|two|three|four|five|several|weeks?|months?|days?)\\s*(?:days|weeks|months|years)?|for\\s+months|for\\s+ages|comes?\\s+and\\s+goes|keeps?\\s+coming\\s+back|on\\s+and\\s+off|since\\s+(?:last|christmas)|\\d+\\s+(?:days|weeks|months)\\s+now)\\b")]));
    s_CHEST_URGENT(py_tuple(vec![py_str("\\b(?:can'?t|cannot|couldn'?t)\\s+breathe\\b"), py_str("\\bshort\\s+of\\s+breath\\b"), py_str("\\b(?:crushing|squeezing|heavy)\\b"), py_str("\\bspreading\\b"), py_str("\\bsweat\\w*\\b"), py_str("\\bcollapsed\\b"), py_str("\\b(?:going|gone|turning)\\s+blue\\b"), py_str("\\bpassed\\s+out\\b"), py_str("\\bright\\s+now\\b"), py_str("\\bthis\\s+(?:morning|afternoon|evening)\\b"), py_str("\\bjust\\s+happened\\b"), py_str("\\b(?:sudden(?:ly)?|all\\s+of\\s+a\\s+sudden|out\\s+of\\s+nowhere)\\b"), py_str("\\b(?:came|come|comes|coming)\\s+on\\s+(?:suddenly|all\\s+of\\s+a\\s+sudden|out\\s+of\\s+nowhere)\\b")]));
    s_DOG_HABIT(py_tuple(vec![py_str("\\b(?:keeps?|kept|always|sometimes|occasionally|now\\s+and\\s+then|every\\s+time|has\\s+started|started|used\\s+to|since\\s+(?:he|she|the\\s+dog)|has\\s+become|getting\\s+worse|how\\s+do\\s+i\\s+stop|how\\s+to\\s+stop|stop\\s+(?:him|her|it|them)\\b|any\\s+advice|train|training|behaviourist|behaviour)\\b"), py_str("\\b(?:when|whenever|while|if|as\\s+soon\\s+as|before|after)\\s+(?:i|we|he|she|they|you|someone|anyone|the\\s+(?:vet|postman|kids))\\b"), py_str("\\b(?:take|taking|touch|touching|reach\\s+for|go\\s+near|going\\s+near|stroke|stroking|pet|petting|pick\\s+up|picking\\s+up|move|moving)\\b[^.?!]{0,20}\\b(?:his|her|their|the)\\s+(?:food|bowl|bone|toy|bed|lead|collar|chew)\\b")]));
    s_DOG_MOUTHING(py_tuple(vec![py_str("\\b(?:bit|bite|bites|biting|nipped|nipping|nips|snapped\\s+at|chewing|chewed)\\s+(?:at\\s+)?(?:the|a|an)\\s+(?:lead|leash|toy|stick|bone|cushion|blanket|sofa|bed|sleeve|shoe|shoes|trouser\\s+leg)\\b[^.?!]{0,30}\\band\\s+(?:my|me)\\b")]));
    s_DOG_SELF_BITE(py_tuple(vec![py_str("\\b(?:bit|bite|bites|biting|nipped|nipping|nips|snapped)\\b[^.?!]{0,15}\\b(?:himself|herself|itself|themselves)\\b"), py_str("\\b(?:bit|bite|bites|biting|nipped|nipping|nips|snapped)\\s+(?:at\\s+)?(?:his|her|its|their)\\s+(?:own\\s+)?(?:paws?|feet|foot|pads?|tail|legs?|fur|coat|skin|ears?|flank|back|bed|blanket|lead|cage|bedding)\\b"), py_str("\\b(?:bit|bite|bites|biting|nipped|nipping|nips|snapped)\\s+(?:at\\s+)?(?:the|a|an)\\s+(?:lead|leash|cage|blanket|bedding|bed|toy|stick|bone|chew|kong|ball|cushion|pillow|sofa|gate|fence|door)\\b")]));
    s_PET_NAMES(py_str("(?:cat|cats|dog|dogs|puppy|puppies|kitten|kittens|pup|pups|rabbit|rabbits|bunny|hamster|hamsters|guinea\\s+pig|guinea\\s+pigs|pony|ponies|horse|horses|pet|pets|bird|birds|parrot|parrots|budgie|budgies|ferret|gerbil|tortoise|chicken|chickens|hen|hens|duck|ducks|goat|goats|sheep|lamb|lambs)"));
    s_BITE_ON_ANIMAL(py_tuple(vec![add(&add(&py_str("\\b(?:bit|bite|bites|biting|bitten|nipped|nips|snapped\\s+at|snaps\\s+at|went\\s+for|goes\\s+for|going\\s+for|attacked|attacks|attacking|mauled|mauls|savaged|savages|latched\\s+onto)\\b[^.?!]{0,30}\\b(?:my|our|the|his|her|their|next\\s+doors?|neighbou?rs?)\\s+"), &g_PET_NAMES()), &py_str("\\b")), add(&add(&py_str("\\b(?:my|our|the|his|her|their|next\\s+doors?|neighbou?rs?)\\s+"), &g_PET_NAMES()), &py_str("\\b[^.?!]{0,30}\\b(?:got|was|were|has\\s+been|had\\s+been|have\\s+been|been)\\s+(?:bit|bitten|attacked|mauled|savaged)\\b(?!\\s+by\\s+(?:fleas?|ticks?|bees?|wasps?|insects?|ants?|mites?|mosquitoes?)\\b)"))]));
    s_FICTION_NOW(py_tuple(vec![py_str("\\bfor\\s+(?:a|my|the)\\s+(?:story|novel|book|film|script|screenplay|essay|assignment|character)\\b"), py_str("\\bin\\s+(?:a|my|the)\\s+(?:story|novel|book|film|game|dream)\\b"), py_str("\\b(?:story|novel|script|screenplay|essay|film)\\b.*\\b(?:character|plot|scene)\\b"), py_str("\\bmy\\s+(?:character|protagonist)\\b")]));
    s_OTHER_RISK(py_tuple(vec![add(&add(&add(&add(&add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_PERSON()), &py_str("|the\\s+")), &g_PERSON()), &py_str("|")), &g_PERSON()), &py_str(")\\b.{0,30}\\b(?:wants?\\s+to\\s+(?:die|kill)|wanna\\s+die|kill\\s+(?:him|her)self|kill\\s+themselves|suicid\\w*|self[-\\s]?harm\\w*|end\\s+(?:it|their\\s+life)|doesn'?t\\s+want\\s+to\\s+(?:live|be\\s+here)|better\\s+off\\s+dead|tried\\s+to\\s+kill|has\\s+taken\\s+(?:too\\s+many|a\\s+lot\\s+of|all\\s+(?:his|her|the)\\s+pills))\\b")), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_PERSON()), &py_str(")\\b.{0,25}\\b(?:hurt|hurts|hurting|cut|cuts|cutting|harm|harming|burning|starving)\\s+(?:him|her|them)self\\b.{0,25}\\b(?:on\\s+purpose|again|keeps?|kept|every\\s+day|at\\s+night|to\\s+(?:cope|feel)|because|won'?t\\s+stop|can'?t\\s+stop|deliberately|in\\s+secret|in\\s+private)\\b")), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_PERSON()), &py_str(")\\b.{0,30}\\b(?:has\\s+|'s\\s+)?locked\\s+(?:him|her|them)self\\s+(?:in|away)\\b.{0,30}\\b(?:pills|tablets|pills?|knife|blades?|something\\s+to\\s+use|razor)\\b")), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_PERSON()), &py_str(")\\b.{0,25}\\b(?:keeps?|kept)\\s+(?:hurting|cutting|burning|starving)\\s+(?:him|her|them)self\\b"))]));
    s_SAFE_NOW(py_tuple(vec![py_str("\\b(?:i'm|im|i\\s+am|we're|we\\s+are|they're|they\\s+are|he's|she's)\\s+(?:outside|out|safe|clear)\\b"), py_str("\\b(?:breathing|feels?|looks?|seems?)\\s+(?:better|easier|normally|again)\\b"), py_str("\\bthey\\s+(?:seem|look|are)\\s+(?:better|okay|ok|awake)\\b"), py_str("\\b(?:fire\\s+(?:brigade|service|fighters?)|paramedics?|ambulance)\\b.{0,25}\\b(?:here|arrived|came|with\\s+(?:us|me))\\b"), py_str("\\b(?:it|the\\s+bleeding)\\s+(?:has\\s+)?stopped\\b")]));
    s_SPINE_BREAK(py_tuple(vec![py_str("\\b(?:my|the)\\s+(?:back|neck|spine|spinal)\\b(?![^.?!]{0,15}\\b(?:gate|door|window|fence|garden|yard|wall|room|lawn|patio|light|pack|passage|road|bumper|seat)\\b)[^.?!]{0,20}\\b(?:is|are|'?s|'?re|was|were|feels?|looks?)\\s+(?:broken|cracked|fractured|snapped)\\b(?!\\s+out\\b)")]));
    s_SPINE_BACK_WORK(py_tuple(vec![py_str("\\b(?:from|after|with|doing|lifting|moving|carrying|digging|gardening|shovelling|shoveling|hoovering|hoover|cleaning|housework|diy|build(?:ing)?)\\b"), py_str("\\b(?:from\\s+all\\s+the|from\\s+the\\s+lifting|from\\s+lifting|all\\s+the\\s+lifting|after\\s+moving|after\\s+the\\s+move|from\\s+the\\s+move|moving\\s+the)\\b")]));
    s_FOLLOW_UP_ANCHOR(py_tuple(vec![py_str("\\b(?:999|911|112|ambulance|paramedics?|fire|brigade|smoke|stairs|outside|safe|clear|breath\\w*|cpr|pulse|conscious|awake|asleep|bleeding|blood|burn\\w*|hospital|doctor|a&e|nurse|poison\\w*|pills?|tablets?|medic\\w*|medicine|epipen|inhaler|police|help|waiting|wait|call\\w*|now|still|they|them|him|her|she|he|it|ok|okay|done|yes|no|thanks|anything|something|else|that|this|scared|frightened|terrified|afraid|soak\\w*|through|still)\\b")]));
    s_HELP_ARRIVED(py_tuple(vec![py_str("\\b(?:ambulance|paramedics?|fire\\s+(?:brigade|service|engine)|police|crew)\\b[^.?!]{0,25}\\b(?:is|are|'s|was|were)\\s+(?:here|outside|coming|on\\s+(?:its|their|the)\\s+way|arriving|arrived|pulling\\s+up|at\\s+the\\s+door)\\b"), py_str("\\b(?:they|someone|somebody|my\\s+(?:mum|mom|dad|husband|wife|partner|sister|brother|son|daughter|friend|neighbour|neighbor))\\b[^.?!]{0,15}\\b(?:is|are|'s)\\s+(?:here|outside|with\\s+me|on\\s+(?:their|his|her)\\s+way|coming\\s+(?:over|round))\\b"), py_str("\\b(?:im|i'?m)\\s+not\\s+on\\s+my\\s+own\\b")]));
    s_FOLLOW_UP_MORE(py_tuple(vec![py_str("\\bshould\\s+i\\s+(?:try|open|shut|close|turn|grab|hold|press|cover|warm|sit|lie|stand|walk|drive|eat|drink|wake|ring|phone|tell|say|text|message)\\b"), py_str("\\bcan\\s+i\\s+(?:open|shut|close|turn|give|move|go|drive|eat|drink|wake|ring|phone)\\b"), py_str("\\bis\\s+it\\s+(?:ok|okay|safe|alright|fine)\\s+to\\b")]));
    s_CASE_WORDS(py_dict(vec![(py_str("gas"), py_str("\\b(?:gas|meter|windows?|doors?|flames?|lights?|switches|smell|boiler|hob)\\b")), (py_str("fire"), py_str("\\b(?:fire|smoke|flames?|stairs?|windows?|doors?|building|flat|brigade|outside|neighbours?)\\b")), (py_str("dog_attack"), py_str("\\b(?:dog|dogs|collar|lead|leash|between|barrier|shout\\w*)\\b")), (py_str("dog_bite"), py_str("\\b(?:bite|bitten|wound|dog|dressing|tetanus)\\b")), (py_str("bleeding"), py_str("\\b(?:blood|cloth|bandage|dressing|pressure|bleed\\w*|wound|soak\\w*|through|stop\\w*|stitch\\w*|scar)\\b")), (py_str("self_harm"), py_str("\\b(?:crisis|line|alone|phone|talk\\w*|tell|say|mum|dad|mother|father|friend|partner|wife|husband|family|sister|brother|gp|doctor)\\b")), (py_str("chest"), py_str("\\b(?:chest|arm|jaw|aspirin|door|sit\\w*|walk\\w*|stand\\w*|move|drive|climb\\w*)\\b")), (py_str("chest_slow"), py_str("\\b(?:chest|tight\\w*|gp|doctor|aspirin|weeks?|pain|breath\\w*)\\b")), (py_str("unconscious"), py_str("\\b(?:breathing|side|pulse|recovery)\\b")), (py_str("breathing"), py_str("\\b(?:inhaler|epipen|upright|breath\\w*|wheez\\w*|lips?)\\b")), (py_str("stroke"), py_str("\\b(?:face|arm|speech|slurr\\w*|time|food|drink)\\b")), (py_str("head_injury"), py_str("\\b(?:head|awake|wake|drowsy|sleepy|sick|vomit\\w*)\\b")), (py_str("poison"), py_str("\\b(?:packet|container|bottle|pills?|tablets?|sick|vomit\\w*)\\b")), (py_str("overdose"), py_str("\\b(?:packet|container|bottle|pills?|tablets?|sick|vomit\\w*|breath\\w*)\\b")), (py_str("hanging"), py_str("\\b(?:rope|neck|breath\\w*|side|cpr)\\b")), (py_str("seizure"), py_str("\\b(?:fitting|fit|seizure|time|mouth|side)\\b")), (py_str("heat"), py_str("\\b(?:shade|water|cool\\w*|fan|sweat\\w*)\\b")), (py_str("general"), py_str("\\b(?:999|911|ambulance|paramedics?|police|help|waiting|wait|call|now|they|them|him|her|ok|safe|clear)\\b"))]));
    s_LOCKED_IN_RISK(py_tuple(vec![py_str("\\b(?:is\\s+)?(?:locked|shut|locks?|locked\\s+himself|locked\\s+herself|locked\\s+themselves)\\b.{0,30}\\b(?:in|inside|behind)\\b.{0,40}\\b(?:pills?|tablets?|medication|medicine|knife|razors?|blades?|scissors|gun)\\b"), py_str("\\b(?:pills?|tablets?|knife|razors?|blades?)\\b.{0,30}\\b(?:locked|shut|barricaded)\\b")]));
    s_FIRE_NOW(py_tuple(vec![py_str("\\b(?:smoke|fire)\\s+alarm\\b.{0,30}\\b(?:going\\s+off|sounding|blaring|wailing)\\b"), py_str("\\bsmoke\\s+(?:in|coming\\s+into|filling|pouring\\s+into|everywhere)\\b"), py_str("\\b(?:on|caught)\\s+fire\\b"), py_str("\\bfire\\s+(?:has\\s+)?(?:started|spread)\\b"), py_str("\\b(?:can't|cannot)\\s+get\\s+out\\b.{0,40}\\b(?:smoke|fire|flames?)\\b")]));
    s_WEIGHT_OFF_FEET(py_tuple(vec![py_str("\\b(?:can'?t|cant|cannot|unable\\s+to|won'?t|wont)\\s+(?:put|bear|take|stand\\s+on)\\s+(?:any\\s+)?weight\\b"), py_str("\\b(?:can'?t|cant|cannot|unable\\s+to)\\s+put\\s+(?:my|his|her|their|your)\\s+(?:foot|feet)\\s+down\\b"), py_str("\\b(?:can'?t|cant|cannot|unable\\s+to)\\s+stand\\s+on\\s+(?:my|his|her|their|the)\\s+(?:ankle|knee|foot|feet|leg|toe|toes|heel)\\b")]));
    s_WEIGHT_HIP(py_tuple(vec![py_str("\\b(?:hip|hips|pelvis|pelvic|thigh|thighs|groin|femur)\\b"), py_str("\\b(?:on\\s+the\\s+(?:floor|ground)|lying|can'?t\\s+get\\s+up|cant\\s+get\\s+up|not\\s+moving)\\b"), py_str("\\b(?:fell|fallen|fall|falling|slipped|tripped|collapsed|landed)\\b[^.?!]{0,30}\\b(?:ladder|roof|scaffold\\w*|stairs|steps|wall|cliff|horse|bunk|loft|trampoline|balcony)\\b"), py_str("\\b(?:ladder|roof|scaffold\\w*|stairs|steps|cliff|horse|bunk|loft|balcony)\\b[^.?!]{0,30}\\b(?:fell|fallen|slipped|landed)\\b"), py_str("\\b(?:my|his|her|their|your|our)\\s+(?:\\w+'s\\s+)?(?:arm|leg|wrist|ankle|shoulder|elbow|knee|foot)\\b[^.?!]{0,25}\\b(?:bent|deformed|at\\s+a\\s+funny\\s+angle|sticking\\s+out|back\\s+to\\s+front)\\b")]));
    s_WEIGHT_LONG(py_tuple(vec![py_str("\\b(?:every|each)\\s+(?:morning|day|night)\\b"), py_str("\\bin\\s+the\\s+mornings?\\b"), py_str("\\blately\\b"), py_str("\\bfor\\s+(?:the\\s+)?(?:last\\s+|past\\s+|few\\s+|several\\s+|a\\s+few\\s+|a\\s+couple\\s+of\\s+)?(?:weeks|months|years)\\b"), py_str("\\b(?:weeks|months|years)\\s+(?:now|ago)\\b"), py_str("\\bfor\\s+ages\\b"), py_str("\\b(?:long\\s+time|while)\\s+now\\b")]));
    s_FALL_CORE(py_tuple(vec![py_str("\\b(?:fell|fallen|falling|tumbled|came\\s+off|come\\s+off)\\s+(?:off|from|down)\\s+(?:a\\s+|the\\s+|his\\s+|her\\s+|their\\s+|my\\s+)?(?:ladder|roof|scaffold\\w*|stairs|steps|wall|cliff|horse|top\\s+bunk|loft|roof\\s+rack)\\b"), py_str("\\b(?:fell|fallen|tumbled)\\s+down\\s+(?:the\\s+|a\\s+)?(?:stairs|steps|flight\\s+of\\s+stairs)\\b"), py_str("\\b(?:fell|fallen|tumbled)\\s+(?:off|from)\\s+(?:a\\s+|the\\s+)?(?:trampoline|bike|swing|slide|climbing\\s+frame|bunk\\s+bed)\\b[^.?!]{0,40}\\b(?:head|neck|back|spine|unconscious|knocked\\s+out|not\\s+moving|isn'?t\\s+moving|won'?t\\s+wake|wont\\s+wake|can'?t\\s+feel|vomit\\w*)\\b"), py_str("\\b(?:fell|fallen|tripped)\\b[^.?!]{0,30}\\b(?:broke|broken|fractured)\\s+(?:my|his|her|their)\\s+(?:hip|pelvis|back|neck|skull|spine)\\b"), py_str("\\b(?:hip|pelvis)\\b[^.?!]{0,25}\\b(?:looks?\\s+(?:wrong|odd|out\\s+of\\s+place)|is\\s+(?:out\\s+of\\s+place|deformed|mangled)|won'?t\\s+bear\\s+weight)\\b"), py_str("\\b(?:fell|fallen|fall|slipped|tripped|collapsed)\\b[^.?!]{0,25}\\b(?:can'?t|cant|cannot|not\\s+able\\s+to|unable\\s+to)\\s+(?:get|getting|stand)\\s*(?:\\w+\\s+)?(?:up|back\\s+up)\\b"), py_str("\\b(?:fell|fallen|fall|slipped|collapsed|fainted)\\b[^.?!]{0,40}\\b(?:still\\s+(?:on\\s+the\\s+floor|lying|down)|(?:he|she|they)\\s*(?:'s|s|\\s+is|\\s+are)\\s+(?:still\\s+)?(?:on\\s+the\\s+floor|lying\\s+(?:there|on)))\\b"), py_str("\\b(?:on\\s+the\\s+(?:floor|ground)|fell|fallen|slipped|collapsed)\\b[^.?!]{0,35}\\b(?:can'?t|cant|cannot|not|won'?t|wont)\\s+(?:move|moving)\\b"), add(&add(&py_str("\\b(?:my|his|her|their|the)\\s+"), &g_PERSON()), &py_str("\\b[^.?!]{0,20}\\b(?:can'?t|cant|cannot)\\s+(?:move|feel\\s+(?:his|her|their)\\s+(?:legs?|arms?|body))\\b")), geti(&g_WEIGHT_OFF_FEET(), &Py::Int(0)), py_str("\\b(?:can'?t|cant|cannot|unable\\s+to|won'?t|wont)\\s+(?:put|bear|take|stand\\s+on)\\s+(?:any\\s+)?weight\\b"), py_str("\\b(?:fell|fallen|fall|slipped|collapsed|on\\s+the\\s+(?:floor|ground))\\b[^.?!]{0,40}\\b(?:can'?t|cant|cannot|not\\s+able\\s+to|unable\\s+to|struggl\\w+\\s+to)\\s+(?:lift|pick|get)\\s+(?:her|him|them|my\\s+(?:mum|mom|mother|dad|father|wife|husband|partner|nan|gran|neighbou?r|friend|brother|sister))\\b"), py_str("\\b(?:can'?t|cant|cannot)\\s+lift\\s+(?:her|him|them|my\\s+\\w+)\\s+off\\s+the\\s+(?:floor|ground)\\b")]));
    s_OWN_LIMB_AT_AN_ANGLE(py_tuple(vec![py_str("\\b(?:my|his|her|their|our|your|the)\\s+(?:arm|arms|leg|legs|knee|knees|ankle|ankles|wrist|wrists|elbow|elbows|shin|shins|thigh|thighs|hip|hips|shoulder|shoulders|jaw|foot|feet|hand|hands)\\b[^.?!]{0,25}\\b(?:bent|deformed|at\\s+a\\s+funny\\s+angle|sticking\\s+out|back\\s+to\\s+front)\\b")]));
    s_FALL_PAST(py_tuple(vec![py_str("\\b(?:last|this)\\s+(?:year|month|week|night|winter|summer|spring|autumn|xmas|christmas)\\b"), py_str("\\b(?:\\d+|a|one|two|three|four|five|six)\\s+(?:yrs?|years?|months?|weeks?)\\s+ago\\b"), py_str("\\b(?:years|months|weeks|ages)\\s+ago\\b"), py_str("\\bused\\s+to\\b"), py_str("\\byesterday\\b"), py_str("\\ba\\s+while\\s+ago\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:a\\s+kid|young|little|a\\s+child|small)\\b"), py_str("\\bwhen\\s+(?:he|she|they)\\s+(?:was|were)\\s+(?:a\\s+)?(?:kid|child|boy|girl|young|little|small|at\\s+school|a\\s+teenager)\\b"), py_str("\\b(?:had|got)\\s+it\\s+checked\\b"), py_str("\\bwas\\s+in\\s+hospital\\s+(?:for|with|after)\\b"), py_str("\\b(?:back)\\s+then\\b"), py_str("\\bwhen\\s+he\\s+was\\s+(?:a\\s+)?(?:baby|small|little)\\b")]));
    s_FALL_NOW(py_tuple(vec![py_str("\\bright\\s+now\\b"), py_str("\\bthis\\s+(?:morning|afternoon|evening)\\b"), py_str("\\bjust\\b"), py_str("\\btoday\\b"), py_str("\\bminutes?\\s+ago\\b"), py_str("\\ban\\s+hour\\s+ago\\b"), py_str("\\bcan'?t\\s+(?:get|stand|sit|feel|move)\\b"), py_str("\\bunable\\s+to\\s+(?:get|stand|move)\\b"), py_str("\\bnot\\s+able\\s+to\\s+(?:get|stand|move)\\b"), py_str("\\ban\\s+ambulance\\b"), py_str("\\bnot\\s+breathing\\b"), py_str("\\bunconscious\\b"), py_str("\\bbleeding\\b"), py_str("\\bhead\\s+injur\\w*\\b"), py_str("\\bstill\\s+(?:on\\s+the\\s+floor|lying|down)\\b")]));
    s_FALL_FINE(py_tuple(vec![py_str("\\b(?:but|and)\\s+(?:he|she|they|it)\\s*(?:'s|s|\\s+is|\\s+was)?\\s*(?:ok|okay|fine|alright|all\\s+right)\\b"), py_str("[,;]\\s*(?:but\\s+)?(?:hes|shes|theyre|it\\s+is|he\\s+is|she\\s+is|they\\s+are|he'?s|she'?s|they'?re)\\s+(?:now\\s+)?(?:ok|okay|fine|alright|all\\s+right)\\b"), py_str("\\b(?:says?|said|tells?|told)\\s+(?:me\\s+)?(?:he|she|they)\\s*(?:'s|s|\\s+was|\\s+is)?\\s*(?:ok|okay|fine|alright)\\b"), py_str("\\b(?:got\\s+up|walked\\s+it\\s+off|laughed\\s+it\\s+off|laughed|ran\\s+off|is\\s+walking|shes\\s+walking|hes\\s+walking|nothing\\s+broken|no\\s+harm|was\\s+fine)\\b"), py_str("\\b(?:i'?m|im)\\s+(?:ok|okay|fine|alright)\\b"), py_str("\\bstill\\s+(?:fine|ok|okay|walking)\\b"), py_str("\\b(?:picture|mirror|shelf|shelves|plaster|paint|tiles?|clock|poster|frame|curtains?|blind|radiator|handle|door|window|bricks?|render|gutter(?:ing)?|shingles?|hook|rail|coving|wallpaper|wreath|chimney|cable|lights?|fixture|fitting)\\b[^.?!]{0,30}\\b(?:fell|falls?|falling|came\\s+off|come\\s+off|coming\\s+off|keeps?\\s+falling)\\b(?!\\s+(?:on|onto|over)\\s+(?:me|my\\s+\\w+|him|her|them|his\\s+\\w+|her\\s+\\w+|their\\s+\\w+))"), py_str("\\bit\\s+(?:keeps?\\s+falling|fell|falls|is\\s+falling|came)\\s+off\\s+(?:the|a)\\s+(?:wall|shelf|hook|ceiling|door|window|board)\\b"), py_str("\\b(?:keeps?|is\\s+always)\\s+falling\\s+off\\s+(?:the|a|my|our)\\s+(?:wall|shelf|ceiling|hook|door|window|board|brackets?)\\b")]));
    s_FALL_REPEAT(py_tuple(vec![py_str("\\b(?:keep|keeps|kept)\\s+(?:on\\s+)?(?:falling|tripping|slipping|going\\s+over|having\\s+falls?)\\b(?!\\s+(?:asleep|out|for|behind|apart|short|through|into|in\\s+love))"), py_str("\\b(?:falls?|falling|trips?|tripping)\\b[^.?!]{0,25}\\b(?:all\\s+the\\s+time|every\\s+day|each\\s+day|a\\s+lot|constantly|regularly|often)\\b"), py_str("\\b(?:is|am|are)\\s+always\\s+(?:falling|tripping|slipping)\\b"), py_str("\\b(?:has|have|had)\\s+been\\s+(?:falling|tripping|slipping|going\\s+over)\\b(?!\\s+(?:asleep|out|for|behind|apart|short|through|into|in\\s+love))"), py_str("\\b(?:had|have|has)\\s+(?:a\\s+)?(?:few|couple\\s+of|several|two|three|more)\\s+falls?\\b")]));
    s_FALL_CHECK_OFF(add(&add(&py_tuple(vec![py_str("\\b(?:nearly|almost|could\\s+have|might\\s+have|would\\s+have)\\b[^.?!]{0,15}\\b(?:fell|fallen|fall|slipped|tripped)\\b"), py_str("\\blaugh\\w*\\b"), py_str("\\b(?:fell|fallen|fall|falling)\\s+(?:asleep|for|in\\s+love|behind|apart|short|through|into)\\b"), py_str("\\b(?:fell|fallen|fall|falling)\\s+out\\b(?!\\s+of\\b)"), py_str("\\b(?:fell|fallen|fall|falling)\\s+off\\s+the\\s+wag[go]n\\b"), py_str("\\b(?:hair|wig|snow|rain|leaves|price|prices|temperature|pound|stock|market)\\b"), py_str("\\b(?:dont|don'?t|didnt|didn'?t|never|not)\\b[^.?!]{0,10}\\b(?:fell|fallen)\\b")]), &g_FALL_WORD_IDIOM()), &py_tuple(vec![py_str("\\b(?:slip|slips|slipped|slipping)\\s+(?:up|behind|past|by|through|away|me|him|her|them|us)\\b"), py_str("\\b(?:slip|slips|slipped|slipping)\\s+out\\b(?!\\s*of\\b)"), py_str("\\b(?:slip|slips|slipped|slipping|trip|trips|tripped)\\b[^.?!]{0,25}\\b(?:work|works|schedule|deadlines?|grades?|marks?|standards?|quality|sales|figures?|prices?|markets?|memor(?:y|ies)|attention|a\\s+note)\\b"), py_str("\\b(?:work|schedule|deadlines?|grades?|standards?|quality|sales|figures?|prices?|markets?|the\\s+quarter)\\b[^.?!]{0,25}\\b(?:has|have|had|keeps|keep|is|was)\\b[^.?!]{0,10}\\b(?:slip|slips|slipped|slipping)\\b")])));
    s_FALL_ANSWERED(py_tuple(vec![py_str("\\b(?:not|isn'?t|isnt|aren'?t|arent|stopped|stop|stopt)\\s+breathing\\b"), py_str("\\b(?:not|isn'?t|isnt|aren'?t|arent)\\s+responding\\b"), py_str("\\bcan'?t\\s+breathe\\b"), py_str("\\bcant\\s+breathe\\b"), py_str("\\bno\\s+pulse\\b"), py_str("\\bwithout\\s+a\\s+pulse\\b"), py_str("\\bno\\s+heartbeat\\b"), py_str("\\bunresponsive\\b(?!\\s*(?:to|at|when|whenever|if|because|as|after|with|about)\\b)"), py_str("\\bnot\\s+waking\\b"), py_str("\\bstill\\s+not\\s+waking\\b")]));
    s_FALL_RED(py_tuple(vec![py_str("\\b(?:hit|hits|struck|landed\\s+on|came\\s+down\\s+on|fell\\s+on)\\b[^.?!]{0,15}\\b(?:me|my\\s+\\w+|him|her|them|his\\s+\\w+|her\\s+\\w+|their\\s+\\w+)\\b"), py_str("\\b(?:unconscious|knocked\\s+out|not\\s+moving|isn'?t\\s+moving|won'?t\\s+wake|wont\\s+wake|can'?t\\s+feel|cant\\s+feel|numb|vomit\\w*|bleed\\w*|hospital|ambulance|a\\s+and\\s+e|a&e|broken|broke|fractured|stiff\\s+neck)\\b"), py_str("\\b(?:head|neck|back|spine)\\b[^.?!]{0,20}\\b(?:hit|bang\\w*|hurt|pounding|bleeding)\\b")]));
    s_RASH_FADE(py_tuple(vec![py_str("\\b(?:doesn'?t|does\\s+not|don'?t|dont|won'?t|wont|isn'?t|is\\s+not|not|never)\\s+(?:fade|fades|fading|blanch|blanches|go\\s+away|goes\\s+away|disappear|disappears|turn\\s+white|come\\s+off|rub\\s+off)\\b"), py_str("\\b(?:doesn'?t|does\\s+not|won'?t|wont)\\s+(?:blanch|fade)\\b"), py_str("\\bnon[\\s-]?blanch\\w*"), py_str("\\b(?:still|stays?|remains?|same)\\b[^.?!]{0,20}\\b(?:press\\w*|glass|push\\w*)\\b"), py_str("\\b(?:press\\w*|glass|push\\w*)\\b[^.?!]{0,25}\\b(?:still|stays?|remains?|same|doesn'?t|does\\s+not|won'?t|wont)\\b"), py_str("\\bunder\\s+a\\s+glass\\b"), py_str("\\b(?:go|goes|going|went|come|comes|coming|came)\\s+(?:back|straight\\s+back)\\b")]));
    s_RASH_PRESSED(py_tuple(vec![py_str("\\bnon[\\s-]?blanch\\w*"), py_str("\\b(?:doesn'?t|does\\s+not|don'?t|dont|won'?t|wont|isn'?t|is\\s+not|not)\\s+blanch\\w*"), py_str("\\b(?:doesn'?t|does\\s+not|don'?t|dont|won'?t|wont|isn'?t|is\\s+not|not)\\s+(?:fade|fades|fading|turn\\s+white)\\b[^.?!]{0,25}\\b(?:press\\w*|glass|push\\w*)\\b"), py_str("\\b(?:press\\w*|glass|push\\w*)\\b[^.?!]{0,25}\\b(?:doesn'?t|does\\s+not|don'?t|dont|won'?t|isn'?t|not)\\s+(?:fade|blanch|turn\\s+white)\\b"), py_str("\\bunder\\s+a\\s+glass\\b"), py_str("\\bsame\\b[^.?!]{0,15}\\bglass\\b")]));
    s_BLACKOUT_EFFORT(py_tuple(vec![py_str("\\b(?:gym|workout|work\\s?out|exercis\\w*|training|treadmill|weights|crossfit|hiit|spin(?:ning)?|rowing|rowing\\s+machine|cross\\s+trainer|running|runs|jog\\w*|sprint\\w*|football|rugby|soccer|netball|hockey|tennis|badminton|squash|basketball|swim\\w*|cycling|cycled|cycle|bike|triathlon|marathon|park\\s?run|match|sports?\\s+day|pe\\s+lesson|games\\s+lesson|climbing|hillwalk\\w*|hiking)\\b")]));
    s_BLACKOUT_FLOOR(py_tuple(vec![py_str("\\b(?:woke\\s+up\\s+on\\s+the\\s+(?:floor|ground|pavement|street)|woke\\s+on\\s+the\\s+(?:floor|ground)|came\\s+round\\s+on\\s+the\\s+(?:floor|ground)|came\\s+to\\s+on\\s+the\\s+(?:floor|ground)|found\\s+(?:him|her|them|my\\s+\\w+)\\s+on\\s+the\\s+(?:floor|ground)|on\\s+the\\s+floor\\s+when\\s+i\\s+(?:woke|came\\s+round|came\\s+to))\\b")]));
    s_BLACKOUT_REPEAT(py_tuple(vec![py_str("\\b(?:keep(?:s|ing)?\\s+(?:on\\s+)?(?:passing|blacking|faint\\w*)|(?:passed|blacked|faint\\w*)\\s+out\\b[^.?!]{0,20}\\bagain\\b|\\b(?:twice|three\\s+times|four\\s+times|several\\s+times|a\\s+few\\s+times|\\bover\\s+and\\s+over)\\b[^.?!]{0,25}\\b(?:passed\\s+out|blacked\\s+out|fainted)\\b|\\b(?:passed\\s+out|blacked\\s+out|fainted)\\b[^.?!]{0,25}\\b(?:twice|again|three\\s+times|several\\s+times|a\\s+few\\s+times)\\b)")]));
    s_BLACKOUT_BREATH(py_tuple(vec![py_str("\\b(?:held|hold|holding)\\b[^.?!]{0,20}\\bbreath\\b[^.?!]{0,40}\\b(?:too\\s+long|until|till|and\\s+(?:blacked|passed|fainted|went\\s+under))\\b")]));
    s_BLACKOUT_SOMEONE_DOWN(py_tuple(vec![py_str("\\bmy\\s+(?:wife|husband|partner|mum|mom|mother|dad|father|son|daughter|child|kid|baby|friend|mate|brother|sister|nan|gran|grandma|grandad|grandpa|neighbour|neighbor|flatmate|colleague|boss|boyfriend|girlfriend)\\b[^.?!]{0,30}\\b(?:pass(?:ed)?\\s+out|black(?:ed)?\\s+out|faint(?:ed)?|collapsed|went\\s+limp|out\\s+cold)\\b")]));
    s_BLACKOUT_A_THING(py_tuple(vec![py_str("\\b(?:screens?|phones?|telly|tv|television|monitor|display|laptop|computer|system|server|lights?|streetlights?|power|electricity|wifi|wi-?fi|signal|cameras?|windows?|curtains?|blinds?|bulbs?|fuses?|projector|kindle)\\b[^.?!]{0,30}\\b(?:black(?:ed|ing)?\\s+out|keep(?:s|ing)?\\s+blacking\\s+out)\\b"), py_str("\\bblack(?:ed|ing)?\\s+out\\b[^.?!]{0,20}\\b(?:screens?|phones?|telly|tv|lights?|windows?|curtains?|power|signal|monitor|camera)\\b"), py_str("\\bkeep(?:s|ing)?\\s+blacking\\s+out\\b[^.?!]{0,20}\\b(?:screen|phone|telly|tv|monitor|lights?|signal)\\b")]));
    s_BLACKOUT_OFF(py_tuple(vec![py_str("\\b(?:hot\\s+(?:room|day|bath|shower|weather)|heat|heatwave|blood\\s+test|bloods|giving\\s+blood|gave\\s+blood|donated\\s+blood|blood\\s+donor|needle|injection|jab|vaccine|hungry|not\\s+eaten|skipped\\s+(?:breakfast|lunch|dinner|a\\s+meal)|standing\\s+up|stood\\s+up|got\\s+up\\s+too\\s+fast|period|drunk|drinking|alcohol|hungover|hangover|laughing|laughed|laughs|joke|film|movie|programme|episode|book)\\b"), py_str("\\b(?:years?|months?)\\s+ago\\b")]));
    s_STIFF_NECK(py_tuple(vec![py_str("\\bstiff\\s+neck\\b"), py_str("\\bneck\\s+(?:is|feels|has\\s+gone|went)\\s+(?:very\\s+|so\\s+|really\\s+)?stiff\\b"), py_str("\\bneck\\s+stiffness\\b"), py_str("\\bcan'?t\\s+bend\\s+(?:my|his|her|their|the)\\s+(?:neck|chin)\\b"), py_str("\\bchin\\s+(?:is\\s+)?(?:stuck|touching|on|to)\\s+(?:my|his|her|their)\\s+chest\\b")]));
    s_MENINGITIS_OTHER(py_tuple(vec![py_str("\\bheadache\\w*"), py_str("\\bhead\\s+(?:is\\s+)?(?:killing|banging|thumping|pounding|hurting|sore)\\b"), py_str("\\bhead\\s+hurts\\b"), py_str("\\bmy\\s+head\\b"), py_str("\\blight\\b"), py_str("\\bsunlight\\b"), py_str("\\bbright\\b"), py_str("\\brash\\w*"), py_str("\\bspots?\\b"), py_str("\\bconfus\\w*"), py_str("\\bdrows\\w*"), py_str("\\bvomit\\w*"), py_str("\\bbeing\\s+sick\\b"), py_str("\\bthrowing\\s+up\\b"), py_str("\\bseizure\\b"), py_str("\\bfloppy\\b"), py_str("\\bunrespons\\w*")]));
    s_STIFF_FINE(py_tuple(vec![py_str("\\bfrom\\s+(?:sleeping|sleep|the\\s+gym|work|the\\s+computer|lifting|stress|the\\s+pillow|the\\s+cold|a\\s+cold|driving|the\\s+desk|studying|gardening)\\b"), py_str("\\bslept\\s+(?:funny|awkward|badly|on\\s+it)\\b"), py_str("\\b(?:crick|kink)\\s+in\\s+(?:it|my\\s+neck)\\b"), py_str("\\bneck\\s+(?:ache|pain)\\s+(?:from|after)\\b")]));
    s_SEPSIS_SIGN(py_tuple(vec![py_str("\\bconfus(?:ed|ion)\\b"), py_str("\\bnot\\s+making\\s+sense\\b"), py_str("\\bdrows\\w*"), py_str("\\bvery\\s+sleepy\\b"), py_str("\\btoo\\s+sleepy\\b"), py_str("\\bhard\\s+to\\s+wake\\b"), py_str("\\bwon'?t\\s+wake\\b"), py_str("\\bwont\\s+wake\\b"), py_str("\\bfloppy\\b"), py_str("\\blimp\\b"), py_str("\\bunrespons\\w*"), py_str("\\bnot\\s+themselves\\b"), py_str("\\bnot\\s+(?:him|her)self\\b"), py_str("\\bbreathing\\s+(?:fast|quickly|funny|strangely)\\b"), py_str("\\bgrunting\\b"), py_str("\\bblue\\s+(?:lips?|round\\s+the\\s+lips?)\\b"), py_str("\\bmottled\\b"), py_str("\\blifeless\\b"), py_str("\\bshivering\\b"), py_str("\\bclammy\\b"), py_str("\\bvery\\s+cold\\b")]));
    s_NEWBORN(py_tuple(vec![py_str("\\bnewborn\\b"), py_str("\\bnew\\s?born\\b"), py_str("\\b\\d+\\s*(?:days?|weeks?)\\s+old\\b"), py_str("\\b(?:under|less\\s+than|not\\s+yet)\\s+(?:3|three)\\s+months?\\b"), py_str("\\b(?:under|less\\s+than|not\\s+yet)\\s+(?:2|two)\\s+months?\\b"), py_str("\\b(?:3|6)\\s+weeks?\\s+old\\b"), py_str("\\b(?:is|only|just|nearly)\\s+(?:1|2|one|two)\\s+months?\\b"), py_str("\\b(?:baby|newborn|infant|son|daughter|boy|girl|little\\s+one)\\b[^.?!]{0,12}(?:is|only|just|nearly)\\s+(?:[1-9]|1[0-2]|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+weeks?\\b(?![-\\s]*(?:pregnan\\w*|along|into|of))"), py_str("\\b(?:1|2|one|two)[-\\s]*months?[-\\s]*old\\b"), py_str("\\b(?:4|5|6|7|8|nine|nine|eight|seven|six|five|four)\\s+weeks?\\s+old\\b")]));
    s_DEVICE_OLD(py_tuple(vec![py_str("\\b(?:oven|boiler|room|water|thermostat|fridge|freezer|shower|bath|heating|radiator|washing\\s+machine|tumble\\s+dryer|air|weather|engine|car|kettle|dishwasher|mattress|sofa|roof|carpet|house|flat|washing\\s+machine|dryer|heater|fan|pump|motor)\\b[^.?!]{0,10}\\b(?:is|was|are|were|only|just|nearly|now)\\b[^.?!]{0,10}\\b\\d+\\s*(?:days?|weeks?)\\s+old\\b")]));
    s_CONFUSED_THING(py_tuple(vec![py_str("\\bconfus(?:ed|ion)\\s+(?:about|by|with|over|on)\\s+(?:(?:my|the|his|her|their|a|this|that)\\s+)?(?:\\w+\\s+){0,3}(?:homework|question|instructions?|settings?|app|recipe|exam|form|bill|tickets?|map|directions|order|complaint|book|film|movie|email|message|answer|word|plan|tablets?|pills?|meds|medication|insulin|doses?|dose|reading|reading[s]?|numbers?|labels?|charges?|where|which|who|when|how)s?\\b")]));
    s_TINY_AGE(py_tuple(vec![py_str("\\bnew\\s?born\\b"), py_str("\\bjust\\s+born\\b"), py_str("\\bonly\\s+just\\s+born\\b"), py_str("\\b(?:1|2|3|4|5|6|7|8|9|10|11|12|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s*(?:days?|weeks?)\\s+old\\b"), py_str("\\b(?:under|less\\s+than|not\\s+yet)\\s+(?:3|three)\\s+months?\\b"), py_str("\\b(?:under|less\\s+than|not\\s+yet)\\s+(?:2|two)\\s+months?\\b"), py_str("\\b(?:baby|babies|newborn|infant|son|daughter|boy|girl|little\\s+one)\\s+(?:is|was|now|only)\\s+(?:1|2|3|4|5|6|7|8|9|10|11|12|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s*(?:days?|weeks?)\\b")]));
    s_OLDER_CHILD_NAMED(py_tuple(vec![py_str("\\b(?:my|our)\\s+(?:son|daughter|boy|girl|child|toddler|kid)\\b[^.!?]{0,12}\\b(?:is|was|aged)\\s+(?:\\d|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen)\\b"), py_str("\\b(?:my|our)\\s+(?:son|daughter|boy|girl|child|toddler|kid)\\b[^.!?]{0,12}\\b(?:aged|age)\\s+(?:\\d|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b")]));
    s_TINY_AGE_FINE(py_tuple(vec![py_str("\\b(?:1[3-9]|[2-9]\\d)\\s*weeks?\\s+old\\b"), py_str("\\b\\d+\\s+months?\\s+old\\b"), py_str("\\b\\d+\\s+years?\\s+old\\b"), py_str("\\bover\\s+(?:3|three|6|six)\\s+months\\b"), py_str("\\b(?:1[3-9]|[2-9]\\d|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty)\\s*weeks?\\b")]));
    s_ABOUT_THE_SPEAKER(py_tuple(vec![py_str("\\bmy\\s+(?:own\\s+)?(?:temp|temps|temperature|fever)\\b"), py_str("\\bi\\s+(?:have|had|got|ve\\s+got|am\\s+running|feel|felt)\\b.{0,24}\\b(?:temp|temps|temperature|fever|feverish|hot|boiling)\\b"), py_str("\\bi\\s*(?:'m|m)\\s+(?:running\\s+a\\s+)?(?:a\\s+)?(?:temp|temperature|fever|feverish|hot|boiling)\\b"), py_str("\\bi\\s+feel\\s+(?:hot|feverish|boiling)\\b"), py_str("\\bmine\\s+(?:is|was)\\b")]));
    s_PALE_SECOND(py_tuple(vec![py_str("\\bclammy\\b"), py_str("\\bsweaty\\b"), py_str("\\bshivery\\b"), py_str("\\bshaking\\b"), py_str("\\bsleepy\\b"), py_str("\\bdrows\\w*"), py_str("\\bfloppy\\b"), py_str("\\blimp\\b"), py_str("\\bconfus\\w*"), py_str("\\bdizzy\\b"), py_str("\\bvomit\\w*"), py_str("\\bbeing\\s+sick\\b"), py_str("\\bnot\\s+(?:him|her|them)self\\b"), py_str("\\bsweat\\w*\\b"), py_str("\\bblue\\b"), py_str("\\bvery\\s+cold\\b"), py_str("\\bcold\\s+and\\b"), py_str("\\bcold\\s+hands\\b"), py_str("\\band\\s+(?:very\\s+)?cold\\b")]));
    s_PALE_HEAT(py_tuple(vec![py_str("\\b(?:from|in|out\\s+in)\\s+the\\s+(?:heat|sun|sunshine)\\b"), py_str("\\b(?:hot\\s+day|heat\\s?wave|sunbathing|sunburn\\w*|greenhouse|conservatory)\\b"), py_str("\\b(?:after|during|from|in|on)\\s+(?:the\\s+|a\\s+|his\\s+|her\\s+)?(?:journey|car|train|bus|coach|flight|plane|tube|heating|radiator|stuffy\\s+room|hot\\s+room|sunbed)\\b")]));
    s_PALE_EXERTION(py_tuple(vec![py_str("\\b(?:after|during|at|from|in)\\s+(?:(?:a\\s+|the\\s+)?(?:\\w+\\s+){0,2}(?:race|run|match|game|session|training|lesson|class|practice|workout|circuit|walk|hike|ride)|cross\\s?country|school\\s+run|football|rugby|netball|hockey|the\\s+gym|sports?\\s+day|swimming|a\\s+swim|cycling|a\\s+bike\\s+ride|pe)\\b"), py_str("\\b(?:after|during|at|from|in)\\s+(?:his|her|my|their|our|the|a|an)?\\s*(?:tennis|badminton|squash|cricket|athletics|gymnastics|dance|dancing|ballet|judo|karate|martial\\s+arts|taekwondo|boxing|climbing|rowing|trampolining|skating|skateboarding|snowboarding|skiing|riding|horse\\s+riding|sports?|games?|practice|training|workout|walk|walking|hike|hiking|ramble)\\b")]));
    s_PALE_EFFORT_RED(py_tuple(vec![py_str("\\bfloppy\\b"), py_str("\\blimp\\b"), py_str("\\b(?:wont|won't|not|can't|cant|unable\\s+to)\\s+wake\\b"), py_str("\\bunresponsive\\b"), py_str("\\bunconscious\\b"), py_str("\\bcollapsed\\b"), py_str("\\bblue\\b"), py_str("\\bgrey\\b"), py_str("\\bgray\\b"), py_str("\\bmottled\\b"), py_str("\\bbreathing\\s+(?:fast|quickly|hard)\\b"), py_str("\\bgrunting\\b"), py_str("\\bnot\\s+breathing\\b")]));
    s_NAMED_SICK_FINE(py_tuple(vec![py_str("\\bin\\s+(?:hospital|icu|intensive\\s+care|a\\s+bed)\\b"), py_str("\\bbeing\\s+treated\\b"), py_str("\\b(?:on|took|having)\\s+antibiotics\\b"), py_str("\\bantibiotics\\b"), py_str("\\bhad\\s+(?:it|this|meningitis|sepsis|pneumonia)\\b"), py_str("\\byears?\\s+ago\\b"), py_str("\\bwhen\\s+(?:he|she|they)\\s+(?:was|were)\\s+(?:little|a\\s+baby|young|small|\\d+)\\b"), py_str("\\brecovered\\b"), py_str("\\bgetting\\s+better\\b"), py_str("\\bnow\\s+fine\\b"), py_str("\\bback\\s+in\\s+\\d{4}\\b"), py_str("\\bis\\s+(?:over|recovering)\\b"), py_str("\\bcame\\s+through\\b"), py_str("\\bthankfully\\b"), py_str("\\bin\\s+(?:his|her|their|the)\\s+(?:notes|file|records|chart|history)\\b"), py_str("\\bhistory\\s+of\\b"), py_str("\\bwas\\s+(?:treated|admitted|diagnosed)\\b"), py_str("\\btreated\\s+for\\b"), py_str("\\bgot\\s+over\\b"), py_str("\\bfought\\s+(?:it|that|this)\\s+off\\b"), py_str("\\bsurvived\\b"), py_str("\\bdischarged\\b"), py_str("\\bno\\s+longer\\b"), py_str("\\bused\\s+to\\b"), py_str("\\b(?:was|were|been)\\s+ruled\\s+out\\b"), py_str("\\bruled\\s+out\\b"), py_str("\\bchecked\\s+for\\b")]));
    s_FEVER_ASK(py_tuple(vec![py_str("^\\s*(?:\\w+\\s+){0,2}(?:how|what|whats|what'?s|when|why|which|where|should|can|could|do|does|will|would|shall)\\b"), py_str("^\\s*(?:is|are|was|were|am|do|does)\\s+(?:my|his|her|our|their|the|this|that|a|an)\\b")]));
    s_FEVER_REPORT(py_tuple(vec![py_str("\\b(?:has|have|had|got|is\\s+running|is\\s+reading|reading|reads|feels|felt|says|said|hit|spiked|went\\s+up\\s+to|running)\\b[^.?!]{0,16}\\b(?:temperature|temps?|fever|feverish|hot)\\b"), py_str("\\b(?:temperature|temp)\\s+(?:is|was|reading|at|hit|spiked)\\s*(?:\\d|high|up|through\\s+the\\s+roof)")]));
    s_CHILD_SCHOOL_AGE(py_tuple(vec![py_str("\\b(?:5|6|7|8|9|five|six|seven|eight|nine)\\s*(?:year|yr)s?[-\\s]*old\\b"), py_str("\\b(?:my|his|her|their|our|the)\\s+(?:son|daughter|boy|girl|child|kid|little\\s+one)s?\\s+(?:is|was|aged|of)\\s+(?:5|6|7|8|9|five|six|seven|eight|nine)\\b(?!\\s*(?:month|week|day|yr\\s*\\d|mo\\b))"), py_str("\\b(?:aged|age)\\s+(?:5|6|7|8|9|five|six|seven|eight|nine)\\b")]));
    s_CHILD_FEVER_NOT_EATING(py_tuple(vec![py_str("\\b(?:not|isnt|isn'?t|wasnt|wasn'?t|wont|won'?t|hasnt|hasn'?t|havent|haven'?t|never)\\s+(?:been\\s+)?(?:eating|drinking|taking\\s+(?:fluids?|drinks?|bottles?)|had\\s+(?:a\\s+)?(?:drink|anything|fluids?))\\b"), py_str("\\b(?:refus\\w*|off\\s+(?:his|her|their)\\s+(?:food|drink|bottle|feeds?))\\b"), py_str("\\bno\\s+(?:appetite|interest\\s+in\\s+(?:food|drink))\\b"), py_str("\\b(?:dehydrated|dried\\s+out)\\b")]));
    s_CHILD_FEVER_RED(py_tuple(vec![py_str("\\b(?:floppy|floppy\\s+as\\s+a\\s+rag|drowsy|drowsier|unresponsive|unconscious|wont\\s+wake|won'?t\\s+wake|hard\\s+to\\s+wake|limp|listless|not\\s+(?:himself|herself|themselves)|breathing\\s+(?:fast|hard|quickly|badly)|struggling\\s+to\\s+breathe|grunting|groaning|blue\\s+lips|blotchy|mottled|stiff\\s+neck|screaming|inconsolable|seizure|fit|convulsion|shaking|shivering\\s+hard)\\b"), py_str("\\b(?:3[9]|4[0-3])(?:\\.\\d)?\\b"), py_str("\\b(?:forty|thirty-nine)\\b"), py_str("\\b(?:rash|spots|non[\\s-]?blanching)\\b"), py_str("\\b(?:not\\s+drinking|wont\\s+drink|won'?t\\s+drink|refusing\\s+to\\s+drink|not\\s+taking\\s+(?:fluids?|drinks?)|refusing\\s+(?:drinks?|fluids?|the\\s+bottle)|keeps\\s+(?:being\\s+sick|throwing\\s+up|vomiting)|being\\s+sick|throwing\\s+up|vomiting|cant\\s+keep\\s+(?:fluids?|drinks?|water)\\s+down|no\\s+(?:wet\\s+)?napp(?:y|ies)|sunken\\s+eyes)\\b")]));
    s_CHILD_GROWN(py_tuple(vec![py_str("\\b(?:1[0-9]|[2-9]\\d)\\s*(?:year|yr)s?[-\\s]*old\\b"), py_str("\\b(?:my|his|her|their|our|the)\\s+(?:son|daughter|boy|girl|child|kid|little\\s+one)s?\\s+(?:is|was|aged|of)\\s+(?:1[0-9]|[2-9]\\d)\\b"), py_str("\\b(?:aged|age)\\s+(?:1[0-9]|[2-9]\\d)\\b"), py_str("\\b(?:teenager|teen|teens|adolescent)\\b")]));
    s_CHILD_YOUNG(py_tuple(vec![py_str("\\b(?:toddler|baby|babies|infant|newborn|little\\s+one|preschool|nursery|\\b\\d+\\s*(?:month|year)s?\\s+old|\\b(?:one|two|three|four|five|six|1|2|3|4|5|6)\\s+years?\\s+old)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:son|daughter|boy|girl|kid|child|niece|nephew)\\b[^.?!]{0,12}\\b(?:is|just|only|nearly|almost)\\s+(?:one|two|three|four|1|2|3|4)\\b")]));
    s_RECENT_FIT_FINE(py_tuple(vec![py_str("\\b(?:laughing|at\\s+that|at\\s+the\\s+price|when\\s+i\\s+saw|when\\s+i\\s+heard|nearly|almost|felt\\s+like|thought\\s+i\\s+would)\\b"), py_str("\\bfit\\s+of\\b"), py_str("\\b(?:every|each|once|twice|three\\s+times|a\\s+month|a\\s+year|regularly|sometimes|usually|last\\s+year|years\\s+ago|used\\s+to)\\b"), py_str("\\b(?:last\\s+(?:week|month|summer|winter|spring|autumn)|weeks?\\s+ago|months?\\s+ago|a\\s+(?:few|couple)\\s+(?:weeks|months)\\s+ago|back\\s+in\\s+\\d{4}|when\\s+(?:he|she|they)\\s+(?:was|little)|as\\s+a\\s+(?:baby|kid|child))\\b"), py_str("\\b(?:epilep\\w+|condition|medication|meds|tablets?|keppra|epilim)\\b")]));
    s_GRUNTING_FINE(py_tuple(vec![py_str("\\b(?:in\\s+(?:his|her|their|its|the)\\s+(?:sleep|cot|crib)|while\\s+(?:a)?sleep\\w*|asleep|sleeping|at\\s+night|when\\s+sleeping|grunting\\s+baby\\s+syndrome)\\b"), py_str("\\bgrunt(?:s|ing|ed)?\\s+(?:at|over|about|through|in\\s+reply)\\b")]));
    s_FEVER_RASH_FINE(py_tuple(vec![py_str("\\b(?:heat|nappy|diaper|teething|teething|prickly\\s+heat|sweat)\\s+rash\\b"), py_str("\\bchicken\\s?pox\\b"), py_str("\\bhand[,\\s]+foot\\b"), py_str("\\beczema\\b"), py_str("\\bnettle\\s+rash\\b"), py_str("\\broseola\\b"), py_str("\\bhas\\s+had\\s+it\\s+before\\b"), py_str("\\bfifth\\s+disease\\b"), py_str("\\bsixth\\s+disease\\b"), py_str("\\bfungal\\b"), py_str("\\bthrush\\b")]));
    s_HYPO_STRONG(py_tuple(vec![py_str("\\bsweat\\w*"), py_str("\\bclammy\\b"), py_str("\\bshak\\w*"), py_str("\\btrembl\\w*"), py_str("\\bdrows\\w*"), py_str("\\bslurr\\w*"), py_str("\\bwon'?t\\s+wake\\b"), py_str("\\bcan'?t\\s+stay\\s+awake\\b"), py_str("\\b(?:a|another)\\s+fit\\b"), py_str("\\bseizure\\w*"), py_str("\\bcoma\\b"), py_str("\\bunconscious\\b"), py_str("\\bpassed\\s+out\\b"), py_str("\\bfaint\\w*"), py_str("\\bacting\\s+(?:drunk|odd|strange|weird)\\b")]));
    s_HYPO_QUESTION(py_tuple(vec![py_str("\\b(?:is|was)\\s+(?:it|that)\\s+(?:normal|ok|okay|safe|dangerous|common)\\b"), py_str("\\bwhat\\s+(?:can|could|should)\\s+(?:i|he|she|they|we)\\s+(?:eat|drink|have|snack)\\b"), py_str("\\b(?:diet|recipes?|meal\\s+(?:plan|ideas?)|what\\s+to\\s+cook)\\b"), py_str("\\b(?:can|could|may|should)\\s+(?:a\\s+)?\\bdiabetics?\\b"), py_str("\\bdiabetics?\\s+(?:eat|drink|have|take)\\b"), py_str("\\b(?:symptoms?|signs?)\\s+of\\s+(?:a\\s+)?(?:hypo\\w*|low\\s+blood\\s+sugar)\\b"), py_str("\\bhow\\s+(?:common|likely|dangerous|many|often)\\b"), py_str("\\bwhat\\s+(?:does|do)\\b[^.?!]{0,25}\\b(?:feel|look|mean)\\b")]));
    s_HYPO_STRONG_OTHER(py_tuple(vec![py_str("\\bsweat\\w*"), py_str("\\bclammy\\b"), py_str("\\bshak\\w*"), py_str("\\btrembl\\w*"), py_str("\\bdrows\\w*"), py_str("\\bconfus\\w*"), py_str("\\bdisorient\\w*"), py_str("\\bslurr\\w*"), py_str("\\b(?:a|another)\\s+fit\\b"), py_str("\\bseizure\\w*"), py_str("\\bcoma\\b"), py_str("\\bunconscious\\b"), py_str("\\bpassed\\s+out\\b"), py_str("\\bfaint\\w*"), py_str("\\bacting\\s+(?:drunk|odd|strange|weird)\\b"), py_str("\\bcan'?t\\s+stay\\s+awake\\b"), py_str("\\bnot\\s+making\\s+(?:any\\s+)?sense\\b")]));
    s_HYPO_HOW_KNOW(py_tuple(vec![py_str("\\bhow\\s+(?:do|can|could|would|will|should)\\s+(?:i|you|we|he|she|they)\\s+(?:know|tell|spot|avoid|prevent|stop|reduce)\\b"), py_str("\\bhow\\s+to\\s+(?:avoid|prevent|stop|spot|recognise|recognize)\\b"), py_str("\\bwhat\\s+does\\b[^.?!]{0,25}\\b(?:feel|look)\\s+like\\b"), py_str("\\b(?:avoid|avoiding|prevent|preventing|stop\\s+myself|keep\\s+(?:from|it\\s+from)|ward\\s+off|less\\s+likely)\\b"), py_str("\\b(?:worried|worry|worrying|scared|afraid|frightened|anxious)\\s+(?:about|of|that)\\b"), py_str("\\b(?:risk\\s+of|in\\s+case\\s+(?:i|he|she|they))\\b")]));
    s_OBJECT_IN_HEAD_THING(py_str("(?:beads?|batter(?:y|ies)|lego|bricks?|peas?|beans?|sweets?|corn(?:flake)?s?|cereal|raisins?|sultanas?|nuts?|peanuts?|popcorn|marbles?|stones?|pebbles?|seeds?|pips?|crayons?|pencils?|erasers?|rubbers?|toys?|playmobil|rice|pasta|papers?|tissues?|plastic|coins?|magnets?|insects?|bugs?|flies|fly|spiders?|earwigs?|moths?|crumbs?|something|a\\s+thing|anything)"));
    s_BONE_THROAT(py_tuple(vec![py_str("\\b(?:fish\\s?bone|fishbone|chicken\\s+bone|lamb\\s+bone|pork\\s+bone|beef\\s+bone|turkey\\s+bone|rabbit\\s+bone|duck\\s+bone|bone|bones)\\b[^.?!]{0,25}\\b(?:stuck|caught|lodged|wedged|scraping|scratching|gone\\s+down\\s+the\\s+wrong\\s+way)\\b"), py_str("\\b(?:stuck|caught|lodged|wedged)\\b[^.?!]{0,20}\\b(?:in|down)\\s+(?:my|his|her|their|the|your)\\s+(?:throat|gullet|neck|windpipe)\\b"), py_str("\\b(?:swallowed|swallowing|ate|eaten|eating|got\\s+down)\\b[^.?!]{0,20}\\b(?:fish\\s?bone|bone|bones)\\b"), py_str("\\b(?:bone|bones|fishbone|shell|husk|pip|kernel)\\b[^.?!]{0,15}\\b(?:in|down)\\s+(?:my|his|her|their)\\s+throat\\b")]));
    s_BONE_ORDINARY(py_tuple(vec![py_str("\\b(?:stock|broth|soup|carcass|recipe|roast|gravy|stew|casserole|curry|fry|bake|baking)\\b"), py_str("\\b(?:dog|cat|puppy|kitten|pup|parrot|pet|bird|hamster)\\b"), py_str("\\b(?:broken|breaks?|broke|fractur\\w*|density|marrow|graft|spur|cancer|cyst|scan|x-?ray|hip|leg|arm|wrist|ankle|shoulder|knee|spine|back|ribs?|skull|jaw|toe|foot)\\b[^.?!]{0,15}\\bbone\\b"), py_str("\\bbone\\b[^.?!]{0,15}\\b(?:density|loss|pain|graft|marrow|spur|china|dry|tired|idle|lazy)\\b"), py_str("\\b(?:wishbone|backbone|bare\\s+bones|no\\s+bones\\s+about|bone\\s+of\\s+contention|bone\\s+to\\s+pick|funny\\s+bone|t-?bone|herringbone)\\b"), py_str("\\b(?:chew|chewed|chewing|gnaw\\w*)\\b[^.?!]{0,15}\\bbone\\b")]));
    s_BONE_PAST(py_tuple(vec![py_str("\\b(?:last|this)\\s+(?:year|month|week|christmas|summer)\\b"), py_str("\\b(?:years?|months?|weeks?)\\s+(?:ago|back)\\b"), py_str("\\bin\\s+(?:19|20)\\d\\d\\b"), py_str("\\b(?:as|when\\s+i\\s+was)\\s+a\\s+(?:kid|child|baby|teen|little\\s+one)\\b"), py_str("\\bused\\s+to\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:little|small|young)\\b"), py_str("\\bages\\s+ago\\b")]));
    s_OLD_PERSON(py_tuple(vec![py_str("\\b(?:is|was|'s|s|at|turned)\\s+(?:6[5-9]|[7-9][0-9]|1[0-2][0-9])\\b"), py_str("\\b(?:i'?m|im|i\\s+am)\\s+(?:6[5-9]|[7-9][0-9]|1[0-2][0-9])\\b"), py_str("\\b(?:6[5-9]|[7-9][0-9]|1[0-2][0-9])\\s*(?:years?[\\s-]?old|yrs?\\s+old|yr[\\s-]?old|yo)\\b"), py_str("\\baged\\s+(?:6[5-9]|[7-9][0-9]|1[0-2][0-9])\\b"), py_str("\\bover\\s+(?:65|70|75|80|85|90|95)\\b"), py_str("\\b(?:in|into|on)\\s+(?:his|her|their)\\s+(?:mid[\\s-]?)?(?:60s|70s|80s|90s)\\b"), py_str("\\b(?:elderly|pensioner|oap)\\b"), py_str("\\bold\\s+(?:man|woman|lady|gentleman|chap|boy|girl|dear|soul|mum|dad|mother|father|mam|gran|grandad|neighbou?r)\\b"), py_str("\\b(?:gran|granny|nan|nana|grandma|grandmother|grandad|granddad|grandpa|grandfather)\\b")]));
    s_OLD_FALL_FINE(py_tuple(vec![py_str("\\b(?:but|and|though)\\s+(?:he|she|they|it)\\s*(?:'s|s|\\s+is|\\s+was)?\\s*(?:ok|okay|fine|alright|all\\s+right)\\b"), py_str("[,;]\\s*(?:but\\s+)?(?:hes|shes|theyre|he\\s+is|she\\s+is|they\\s+are|he'?s|she'?s|they'?re)\\s+(?:now\\s+)?(?:ok|okay|fine|alright|all\\s+right)\\b"), py_str("\\b(?:says?|said|tells?|told)\\s+(?:me\\s+)?(?:he|she|they)\\s*(?:'s|s|\\s+was|\\s+is)?\\s*(?:ok|okay|fine|alright)\\b"), py_str("\\b(?:nothing\\s+broken|no\\s+harm|just\\s+bruised|only\\s+bruised|walked\\s+it\\s+off|laughed\\s+it\\s+off|laughed\\s+about\\s+it)\\b"), py_str("\\b(?:had|got)\\s+(?:it|him|her|them)\\s+(?:checked|looked\\s+at)\\b"), py_str("\\b(?:seen|checked)\\s+by\\s+(?:the\\s+)?(?:doctor|gp|nurse|paramedics?)\\b"), py_str("\\b(?:been\\s+)?(?:x-?rayed|x-?rays?)\\b"), py_str("\\bin\\s+(?:the\\s+)?hospital\\b"), py_str("\\b(?:has|have|had)\\s+been\\s+(?:seen|checked)\\b")]));
    s_FALL_HIP(py_tuple(vec![py_str("\\b(?:fell|fallen|fall|falls|slipped|tripped|went\\s+over|had\\s+a\\s+fall|toppled)\\b[^.?!]{0,45}\\b(?:hip|pelvis)\\b[^.?!]{0,25}\\b(?:hurt\\w*|pain\\w*|sore|ache\\w*|aching|broke\\w*|click\\w*|stiff|gave\\s+way|is\\s+bad)\\b"), py_str("\\b(?:hurt\\w*|pain\\w*|sore|ache\\w*|aching|broke\\w*|click\\w*)\\b[^.?!]{0,25}\\b(?:hip|pelvis)\\b[^.?!]{0,45}\\b(?:fell|fallen|fall|slipped|tripped|went\\s+over|had\\s+a\\s+fall)\\b"), py_str("\\b(?:fell|fallen|fall|falls)\\b[^.?!]{0,40}\\b(?:hurt|broke\\w*|injured|done|damaged)\\b\\s+(?:his|her|their|my|the|a)\\s+(?:hip|pelvis)\\b")]));
    s_OLD_FALL_NOT(py_tuple(vec![py_str("\\bfell\\s+(?:asleep|for\\s+(?:it|him|her|them|the)|in\\s+love|out\\s+(?:with|over)|apart|through|short|behind|flat|ill|silent|prey)\\b"), py_str("\\bfell\\s+(?:over\\s+|about\\s+)?laughing\\b"), py_str("\\bwent\\s+down\\s+(?:well|badly|a\\s+treat|like)\\b"), py_str("\\bwent\\s+over\\s+on\\s+(?:my|his|her|their)\\s+(?:ankle|foot)\\b"), py_str("\\bthe\\s+(?:price|prices|market|temperature|rain|snow|share|shares)\\b"), py_str("\\bfall(?:ing)?\\s+(?:asleep|for|in\\s+love|out|apart|through|short|behind)\\b")]));
    s_SHOT(py_tuple(vec![py_str("\\b(?:has|have|had|been|got|gets|was|were)\\s+(?:been\\s+)?shot\\b"), py_str("\\bshot\\s+(?:him|her|them|me|my\\s*self|himself|herself|themselves|in\\s+the|through)\\b"), py_str("\\bshot\\s+(?:a\\s+|the\\s+)?(?:man|woman|person|boy|girl|kid|child|policeman|officer)\\b"), py_str("\\b(?:gunshot|someone\\s+shot|they\\s+shot|he\\s+shot|she\\s+shot|shots\\s+fired)\\b")]));
    s_SHOT_FINE(py_tuple(vec![py_str("\\b(?:paintball\\w*|bb\\s+gun|airsoft|nerf|toy\\s+gun|water\\s+pistol|squirt\\s+gun)\\b"), py_str("\\b(?:flu|covid|tetanus|booster|vaccine|vaccination|immunisation|immunization|jab)s?\\b"), py_str("\\b(?:photo|photograph|picture|film|movie|camera|scene|screenshot|screengrab|video)s?\\b"), py_str("\\ba\\s+shot\\s+of\\b"), py_str("\\bshot\\s+glass(?:es)?\\b"), py_str("\\bshot\\s+put\\b"), py_str("\\b(?:long|wide|close|medium)\\s+shot\\b"), py_str("\\bshot\\s+(?:up|down)\\b"), py_str("\\ba\\s+shot\\s+in\\s+the\\s+dark\\b"), py_str("\\bshot\\s+(?:my\\s*self|him|her|them)sel(?:f|ves)\\s+in\\s+the\\s+foot\\s+(?:with|over|on)\\s+(?:that|this|the)\\b"), py_str("\\b(?:goal|baskets?|balls?|penalt(?:y|ies)|hoop|free\\s+kick|target|round|bird|rabbit|deer|fox|pheasant|clay\\s+pigeon)s?\\b")]));
    s_CHEM_WORD(py_str("\\b(?:acid|bleach|cleaners?|detergent|powder|powders|drain\\s+cleaner|caustic|chemical\\w*|lye|ammonia|white\\s+spirit|paint\\s+thinner|thinners?|solvent|oven\\s+cleaner|petrol|paraffin|chlorine|antifreeze|disinfectant|diesel|turpentine|nail\\s+varnish\\s+remover)\\b"));
    s_EYE_CHEM(py_tuple(vec![add(&slice_or_raise(&g_CHEM_WORD(), &Py::None, &neg(&Py::Int(2)), &Py::None), &py_str("[^.?!]{0,25}\\b(?:in|into|inside|over)\\s+(?:my|his|her|their|your|both|one|the)\\s+eyes?\\b")), add(&py_str("\\beyes?\\b[^.?!]{0,20}\\b(?:full\\s+of|covered\\s+in|splashed\\s+with|with)\\b[^.?!]{0,15}"), &g_CHEM_WORD()), add(&add(&py_str("\\b(?:splashed|squirted|sprayed|poured)\\b[^.?!]{0,20}"), &g_CHEM_WORD()), &py_str("[^.?!]{0,20}\\b(?:in|into)\\s+(?:my|his|her|their|your)\\s+eyes?\\b")), py_str("\\b(?:got|get|gets|getting|had|has|have|sprayed|spray|splashed|splash|poured|threw|squirted|dabbed|rubbed|put|smeared|scooped)\\b[^.?!]{0,30}\\b(?:washing\\s+powder|washing\\s+up\\s+liquid|deodorant|hairspray|hair\\s+spray|perfume|aftershave|ac?etone|nail\\s+varnish|superglue|glue|silicone|limescale|toilet\\s+cleaner|carpet\\s+cleaner|screen\\s+wash)\\b[^.?!]{0,25}\\b(?:eye|eyes)\\b"), py_str("\\b(?:powder|powders|detergent|cleaner|cleaners|bleach|spray)\\b[^.?!]{0,20}\\b(?:out of|in|into|off)\\s+(?:my|his|her|their|your)\\s+eyes?\\b"), py_str("\\b(?:washing\\s+powder|deodorant|hairspray|perfume|aftershave|nail\\s+varnish|superglue|limescale|toilet\\s+cleaner)\\b[^.?!]{0,30}\\b(?:in|into|went)\\b[^.?!]{0,12}\\b(?:my|his|her|their|the)\\s+(?:\\w+\\s+)?eyes?\\b")]));
    s_EYE_FINE(py_tuple(vec![py_str("\\b(?:acid\\s+reflux|heartburn|acid\\s+stomach|stomach\\s+acid|acid\\s+rain|acid\\s+jazz|amino\\s+acids?|folic\\s+acid|acid\\s+test|acid\\s+house|acid\\s+wash)\\b"), py_str("\\b(?:clean|cleaning|cleaned)\\s+(?:the\\s+|my\\s+)?(?:bathroom|kitchen|oven|drain|shower|toilet|windows?)\\b"), py_str("\\b(?:keep|keeps|kept|store|stored|out\\s+of\\s+reach)\\b[^.?!]{0,30}\\beyes?\\b")]));
    s_FOOD_CUT(py_tuple(vec![py_str("\\b(?:avocado|avocados|bread|cake|loaf|cheese|tomato|tomatoes|onion|onions|lettuce|salad|cabbage|apple|apples|banana|bananas|lemon|oranges?|melon|mango|pepper|peppers|cucumber|carrot|carrots|potato|potatoes|celery|fruit|herbs?|chilli|chillies|garlic|ginger|lemon|lime|bacon|ham|chicken|beef|pork|lamb|fish|sausage|sausages|steak|meat|pizza|lasagne|quiche|pie|sandwich|wrap|roll|bagel|pastry|croissant|melon|watermelon|pineapple|veg|vegetables?|courgette|aubergine|broccoli|cauliflower|squash|pumpkin)\\b")]));
    s_HEADACHE_BAD(py_tuple(vec![py_str("\\b(?:worst|bad|terrible|awful|severe|unbearable|splitting|thumping|excruciating)\\b[^.?!]{0,15}\\bhead(?:ache)?\\b"), py_str("\\bhead(?:ache)?\\b[^.?!]{0,30}\\b(?:worst|unbearable|severe|excruciating|terrible|awful|bad)\\b"), py_str("\\bworst\\s+head(?:ache)?\\s+of\\s+my\\s+life\\b")]));
    s_HEADACHE_SUDDEN(py_tuple(vec![py_str("\\bsudden\\w*"), py_str("\\bout\\s+of\\s+(?:nowhere|the\\s+blue)\\b"), py_str("\\ball\\s+of\\s+a\\s+sudden\\b"), py_str("\\bthunderclap\\b"), py_str("\\bwithin\\s+(?:a\\s+)?(?:minute|seconds?)\\b"), py_str("\\blike\\s+a\\s+(?:bang|clap|thunder|blow)\\b"), py_str("\\bexploded\\b"), py_str("\\bcame\\s+on\\b[^.?!]{0,15}\\b(?:suddenly|fast|quickly|in\\s+seconds|like)\\b")]));
    s_HEADACHE_STORY(py_tuple(vec![py_str("\\b(?:not|isnt|isn'?t)\\s+(?:that|too|so|very)\\s+bad\\b"), py_str("\\ba\\s+bit\\b"), py_str("\\bslight\\w*"), py_str("\\bmild\\b"), py_str("\\bmanageable\\b"), py_str("\\bnot\\s+too\\s+bad\\b"), py_str("\\b(?:last\\s+(?:year|month|week)|years?\\s+ago|when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little)|used\\s+to|went\\s+away|gone\\s+now|had\\s+one\\s+before|still\\s+there)\\b"), py_str("\\b(?:hangover|migraine|dehydrat\\w*|sinus\\w*|hay\\s?fever|screen|computer|tension\\s+headache|stress\\w*|period|flu|cold|cough|headache\\s+tablets?|paracetamol|ibuprofen)\\b")]));
    s_RASH_HEADACHE_FINE(py_tuple(vec![py_str("\\b(?:heat|nappy|diaper|teething|prickly\\s+heat|sweat|shav(?:e|ing)|razor|sunburn|sun\\s+burn|acne|hives|nettle|eczema|dermatitis|allerg\\w*|hay\\s?fever|thrush|fungal)\\b[^.?!]{0,15}\\brash\\b"), py_str("\\brash\\b[^.?!]{0,15}\\b(?:from|after|off)\\s+(?:a\\s+|the\\s+|my\\s+|her\\s+|his\\s+)?(?:shav\\w*|razor|wax\\w*|sun|heat|perfume|new\\s+cream|cream|washing\\s+powder|plaster|tape|nappy|diaper)\\b"), py_str("\\b(?:rash|rashes|spots|marks|dots|blotches)\\b[^.?!]{0,20}\\b(?:from|after|off|where)\\b"), py_str("\\b(?:chicken\\s?pox|hand[,\\s]+foot|roseola|fifth\\s+disease|sixth\\s+disease|shingles)\\b"), py_str("\\b(?:had\\s+it\\s+before|last\\s+(?:week|month|year)|weeks?\\s+ago|years?\\s+ago)\\b"), py_str("\\b(?:spots|pimples|blackheads|whiteheads|zits|acne|breakouts?)\\b[^.?!]{0,25}\\b(?:on|around|across)\\s+(?:my\\s+|his\\s+|her\\s+|their\\s+|the\\s+)?(?:face|forehead|chin|cheeks?|nose|jaw|back|shoulders?)\\b")]));
    s_COUGH_BLOOD(py_tuple(vec![py_str("\\b(?:cough\\w*|spit\\w*|bringing|bring)\\b[^.?!]{0,20}\\b(?:up|out)\\b[^.?!]{0,20}\\bblood\\b"), py_str("\\bblood\\b[^.?!]{0,20}\\b(?:when\\s+i\\s+cough|coughing|in\\s+my\\s+(?:sputum|phlegm|spit|mucus))\\b"), py_str("\\b(?:coughing|spitting)\\s+(?:up\\s+)?blood\\b"), py_str("\\bbloody\\s+(?:sputum|phlegm)\\b")]));
    s_BREATH_REST(py_tuple(vec![py_str("\\b(?:suddenly|all\\s+of\\s+a\\s+sudden|out\\s+of\\s+nowhere|out\\s+of\\s+the\\s+blue|just\\s+now|came\\s+on)\\b.{0,30}\\b(?:short\\s+of\\s+breath|breathless|out\\s+of\\s+breath|can'?t\\s+breath\\w*|struggling\\s+to\\s+breath\\w*)\\b"), py_str("\\b(?:short\\s+of\\s+breath|shortness\\s+of\\s+breath|breathless)\\b.{0,30}\\b(?:sitting|sat\\s+down|lying|in\\s+bed|at\\s+rest|resting|doing\\s+nothing|watching\\s+tv|in\\s+a\\s+chair)\\b")]));
    s_SPIKED(py_tuple(vec![py_str("\\b(?:drink|drinks|pint|glass|cocktail|bottle|water)\\b[^.?!]{0,25}\\b(?:was|got|is)\\s+(?:spiked|drugged|dosed)\\b"), py_str("\\b(?:spiked|drugged|dosed)\\s+(?:my|his|her|their|the|someone'?s)\\s+(?:drink|drinks|pint|glass|bottle|water)\\b"), py_str("\\bi\\s+(?:think\\s+|reckon\\s+|pretty\\s+sure\\s+)?(?:got|was|been)\\s+(?:spiked|drugged|dosed)\\b"), py_str("\\bsomeone\\s+(?:put|slipped|dropped|spiked)\\s+(?:something|a\\s+pill|drugs?|roofies?)\\s+(?:in|into)\\s+(?:my|his|her|their|the)\\s+(?:drink|pint|glass|water|bottle)\\b"), py_str("\\broofies?\\b"), py_str("\\bdrink\\s+was\\s+spiked\\b"), add(&add(&add(&add(&py_str("\\b(?:i|he|she|they|my\\s+"), &g_PERSON()), &py_str("|a\\s+")), &g_PERSON()), &py_str(")\\b(?![^.?!]{0,20}\\bdrink\\b)[^.?!]{0,20}\\b(?:got|was|were|been)\\s+(?:spiked|drugged|dosed)\\b"))]));
    s_HEARD_ASSAULT(py_tuple(vec![py_str("\\b(?:screaming|screams|shouting|yelling|banging|smashing|glass\\s+breaking|crashing)\\b[^.?!]{0,40}\\b(?:hurting|hitting|beating|attacking|choking|dragging|kicking)\\s+(?:her|him|them|each\\s+other|his\\s+(?:wife|girlfriend|partner|kid|son|daughter)|her\\s+(?:husband|boyfriend|partner|kid|son|daughter))\\b"), py_str("\\b(?:i\\s+can\\s+hear|i\\s+heard|we\\s+can\\s+hear)\\b[^.?!]{0,40}\\b(?:screaming|shouting|smashing|glass\\s+breaking|banging|someone\\s+crying)\\b[^.?!]{0,40}\\b(?:next\\s+door|upstairs|downstairs|neighbou?rs?|flat\\s+above|flat\\s+below|from\\s+the\\s+flat)\\b"), py_str("\\b(?:my|our|the)\\s+neighbou?rs?\\b[^.?!]{0,40}\\b(?:hurting|hitting|beating|attacking|fighting|smashing\\s+the\\s+place)\\b"), py_str("\\b(?:a\\s+|some\\s+)?(?:woman|man|girl|boy|kid|child|lady|bloke|fella|someone|somebody|neighbou?r)\\b[^.?!]{0,30}\\b(?:screaming|sobbing|shouting\\s+for\\s+help|crying\\s+for\\s+help|begging)\\b[^.?!]{0,40}\\b(?:next\\s+door|upstairs|downstairs|the\\s+flat|the\\s+house|the\\s+street|outside\\s+(?:my|our|the))\\b")]));
    s_BREATH_EFFORT(py_tuple(vec![py_str("\\b(?:ran|run|running|jogged|jogging|sprinted|hurried|rushed|chased|cycled|raced|legged\\s+it)\\b[^.?!]{0,25}\\b(?:for|to|after|up|down)\\s+(?:the\\s+|a\\s+|my\\s+)?(?:bus|train|tram|tube|coach|plane|flight|stairs|steps|ladder|hill)\\b"), py_str("\\b(?:ran|jogged|sprinted|walked)\\s+(?:up|down)\\s+the\\s+(?:stairs|steps|hill|road)\\b")]));
    s_EXERTION_CLASS(py_tuple(vec![py_str("\\b(?:spin(?:ning)?|zumba|pilates|cross-?fit|box(?:ing)?|kickbox\\w*|skipping|badminton|rugby|cricket|hike|hiking|treadmill|cross-?trainer|elliptical|aerobics|hiit|circuit|rowing|boot\\s?camp|exercise\\s+class|fitness\\s+class|spin\\s+class|aqua\\s?aerobics|squash|body\\s?pump|body\\s?combat|pt\\s+session|gym\\s+class)\\b"), py_str("\\b(?:class|classes|session|workout)\\b")]));
    s_EFFORT_RED_HEART(py_tuple(vec![py_str("\\b(?:faint\\w*|dizzy|light-?headed|numb|tingl\\w*|passed\\s+out|collapsed|blacked\\s+out|vomit\\w*|nause\\w*|queasy|palpitations?|fluttering)\\b"), py_str("\\b(?:feel|feels|feeling|felt)\\s+(?:sick|ill|queasy|faint)\\b"), py_str("\\b(?:arm|arms|jaw|shoulder|shoulders|neck)\\b[^.?!]{0,20}\\b(?:hurt\\w*|pain\\w*|ache\\w*|tight\\w*|heavy|sore|numb)\\b"), py_str("\\b(?:hurt\\w*|pain\\w*|ache\\w*|tight\\w*|heavy|sore)\\b[^.?!]{0,15}\\b(?:arm|arms|jaw|shoulder|shoulders|neck)\\b")]));
    s_EFFORT_HEART_CLUSTER(py_tuple(vec![py_str("\\b(?:clammy|light-?headed|dizzy|dizziness|faint\\w*|woozy|queasy|nauseous|nausea)\\b"), py_str("\\b(?:grey|gray|pale|ashy)\\b[^.?!]{0,12}\\b(?:face|skin|lips?)\\b"), py_str("\\b(?:cold|damp|drench(?:ed|ing)|soaked|wet)\\s+(?:with\\s+|in\\s+|and\\s+)?(?:sweat\\w*|clammy)\\b"), py_str("\\bsweat\\w*\\b[^.?!]{0,24}\\b(?:dizzy|light-?headed|faint\\w*|woozy|sick|queasy|nauseous|clammy)\\b")]));
    s_EFFORT_PATTERN(py_tuple(vec![py_str("\\b(?:always|every\\s+time|each\\s+time|usually|normally|whenever)\\b"), py_str("\\bfor\\s+(?:months|years|weeks|ages|a\\s+while)\\b"), py_str("\\b(?:since|from)\\s+(?:last\\s+year|childhood|i\\s+was\\s+(?:a\\s+)?(?:kid|child|younger|at\\s+school))\\b"), py_str("\\b(?:keeps?|kept)\\s+(?:happening|coming\\s+back|doing\\s+it)\\b"), py_str("\\bon\\s+and\\s+off\\b")]));
    s_EFFORT_BENIGN(py_tuple(vec![py_str("\\b(?:pregnan\\w*|expecting\\s+a\\s+baby|trimester)\\b"), py_str("\\b(?:a|the|this|my)\\s+(?:cold|flu|covid|coronavirus|chest\\s+infection|virus)\\b"), py_str("\\bhay\\s?fever\\b|\\bpollen\\b|\\ballergie\\w*\\b|\\bhistamine\\b"), py_str("\\b(?:hot\\s+day|heat\\s?wave|in\\s+this\\s+heat|hot\\s+weather|hot\\s+building|boiling)\\b"), py_str("\\b(?:unfit|out\\s+of\\s+shape|overweight|put\\s+on\\s+weight)\\b")]));
    s_EFFORT_TOO_LONG(py_tuple(vec![py_str("\\b(?:for|it'?s\\s+been|its\\s+been|been|about|over|more\\s+than|nearly|almost)\\s+(?:an?\\s+hour|half\\s+an\\s+hour|(?:eleven|twelve|fifteen|twenty|twenty-five|thirty|forty|fifty|sixty|\\d{2,})\\s*(?:min|mins|minutes?)|ages)\\b"), py_str("\\b(?:over|more\\s+than)\\s+ten\\s+minutes\\b"), py_str("\\ball\\s+(?:day|morning|afternoon|evening)\\b")]));
    s_BREATH_EFFORT_RED(py_tuple(vec![py_str("\\b(?:blue|grey|gray|collapsed|unconscious|passed\\s+out|crushing|crushed|chest\\s+pain|pins\\s+and\\s+needles|numb)\\b"), py_str("\\b(?:still|now|and)\\b[^.?!]{0,20}\\b(?:can'?t|cannot|cant)\\s+breathe\\b"), py_str("\\b(?:can'?t|cannot|cant|unable\\s+to)\\s+(?:catch|get)\\s+(?:my|her|his|their)\\s+breath\\b"), py_str("\\b(?:hours?|days?)\\s+(?:later|ago)\\b")]));
    s_LIPS_SWELL(py_tuple(vec![py_str("\\b(?:lips?|tongue|throat|face)\\b[^.?!]{0,30}\\b(?:swelling|swollen|swelled|puff\\w*\\s+up|puffed\\s+up|puffy|blowing\\s+up)\\b[^.?!]{0,40}\\b(?:sting\\w*|bee|wasp|nut|nuts|peanut\\w*|shellfish|food|medicine|antibiotic\\w*|prawn|egg|dairy|allerg\\w*|throat|tongue|breath\\w*|can'?t\\s+swallow|epipen)\\b"), py_str("\\b(?:sting\\w*|bee|wasp|nut|peanut\\w*|shellfish|food|antibiotic\\w*|allerg\\w*)\\b[^.?!]{0,40}\\b(?:lips?|tongue|throat|face)\\b[^.?!]{0,25}\\b(?:swelling|swollen|swelled|puff\\w*|puffy)\\b"), py_str("\\b(?:throat|tongue|uvula)\\b[^.?!]{0,20}\\b(?:swelling|swollen|closing|tight)\\b"), py_str("\\b(?:tight|tightening)\\s+throat\\b")]));
    s_LIPS_SWELL_BARE_OFF(py_tuple(vec![py_str("\\b(?:cold\\s+sore|chapped|chaps|dry\\s+lips|chap\\s?stick|lip\\s+balm|piercing|filler\\w*|collagen|blister|ulcer|brace|braces|wisdom\\s+tooth|gum|herpes|bit\\s+my\\s+lip|bit\\s+my\\s+tongue|bite\\s+my\\s+lip|cut\\s+my\\s+lip|burnt\\s+my\\s+mouth|hot\\s+food|curry|chilli|hangover|dehydrated|kissed|kissing|numb)\\b"), py_str("\\A(?![\\s\\S]*\\b(?:faint\\w*|dizz\\w*|breath\\w*|throat|tongue|wheez\\w*|swallow\\w*|collaps\\w*|vomit\\w*|unconscious|itch\\w*)\\b)[\\s\\S]*\\b(?:yesterday|last\\s+(?:night|week|month|year)|years?\\s+ago|used\\s+to|always\\s+(?:swells|does|goes)|as\\s+a\\s+(?:kid|child)|in\\s+the\\s+(?:cold|winter|sun)|(?:from|because\\s+of|after|with)\\s+(?:the\\s+)?(?:cold|wind|sun|weather|hayfever))\\b"), py_str("\\A(?![\\s\\S]*\\b(?:faint\\w*|dizz\\w*|breath\\w*|throat|tongue|wheez\\w*|swallow\\w*|collaps\\w*|vomit\\w*|unconscious|itch\\w*|swollen\\s+(?:shut|closed))\\b)[\\s\\S]*\\blips?\\b[^.?!]{0,25}\\bin\\s+(?:the\\s+)?(?:cold|wind|sun|winter)\\b"), py_str("\\A(?:if|when|what\\s+if|should\\s+i|would)\\b"), py_str("\\A(?![\\s\\S]*\\b(?:throat|tongue|uvula|breath\\w*|wheez\\w*|swallow\\w*|faint\\w*|dizz\\w*|collaps\\w*|unconscious|sting\\w*|stung|bee|wasp|hives|itch\\w*|vomit\\w*|reaction|allerg\\w*|epipen|shut)\\b)[\\s\\S]*\\b(?:face|eyes?|lips?)\\b[^.?!]{0,30}\\b(?:from|after|because\\s+of|with)\\s+(?:all\\s+(?:the|that|my)\\s+)?(?:crying|sobbing|a\\s+good\\s+cry)\\b"), py_str("\\A(?![\\s\\S]*\\b(?:throat|tongue|uvula|breath\\w*|wheez\\w*|swallow\\w*|faint\\w*|dizz\\w*|collaps\\w*|unconscious|sting\\w*|stung|bee|wasp|hives|itch\\w*|vomit\\w*|reaction|allerg\\w*|epipen|shut)\\b)[\\s\\S]*\\b(?:i\\s+(?:have\\s+|had\\s+|was\\s+|been\\s+|just\\s+)?(?:cried|crying|sobbed|sobbing)|(?:cried|crying|sobbing)\\s+so\\s+much)\\b[^.?!]{0,30}\\b(?:face|eyes?|lips?)\\b[^.?!]{0,25}\\b(?:puffy|puffed\\s+up|swollen|swelling|swelled|blotchy|red)\\b"), py_str("\\b(?:what|should)\\s+(?:do\\s+)?i\\s+do\\s+if\\b")]));
    s_LIPS_TINGLE(py_tuple(vec![py_str("\\b(?:lips?|tongue|mouth|throat|face)\\b[^.?!]{0,25}\\b(?:tingl\\w*|pins\\s+and\\s+needles|itch\\w*|prickl\\w*|numb(?:ness|ed|ing|s)?)\\b"), py_str("\\b(?:tingl\\w*|pins\\s+and\\s+needles|itch\\w*|prickl\\w*|numb(?:ness|ed|ing|s)?)\\b[^.?!]{0,25}\\b(?:lips?|tongue|mouth|throat|face)\\b"), py_str("\\bthroat\\b[^.?!]{0,20}\\b(?:feels?|is|going|getting)\\s+(?:a\\s+bit\\s+|a\\s+little\\s+|really\\s+|very\\s+|so\\s+|properly\\s+)?(?:funny|odd|weird|strange|thick|funny\\s+feeling)\\b")]));
    s_STING_SETTLED(py_tuple(vec![py_str("\\b(?:im|i'?m|i\\s+am|its|it'?s|it\\s+is|that'?s|thats)\\s+(?:all\\s+)?(?:fine|ok|okay|better|gone|settled|passed|over|cleared\\s+up)\\s+now\\b"), py_str("\\b(?:it|that)\\s+(?:has\\s+|'?s\\s+|is\\s+)?(?:passed|worn\\s+off|gone\\s+off|settled|cleared\\s+up)\\b"), py_str("\\b(?:feel|feeling)\\s+(?:fine|ok|okay|better)\\s+now\\b")]));
    s_LIPS_TINGLE_FOOD(py_tuple(vec![py_str("\\b(?:ate|eaten|eating|eats|swallowed)\\b"), py_str("\\b(?:after|since)\\s+(?:the\\s+|my\\s+|a\\s+|that\\s+)?(?:takeaway|take-?out|meal|dinner|lunch|breakfast|snack|dessert|pudding|cake|sushi|curry|pizza|pasta|sandwich|salad|noodles|rice|fish|chips|shellfish|prawns?|nuts?|peanut\\w*|fruit|berries|strawberr\\w*|kiwi|mango|avocado|tomato|egg|eggs|milk|cheese|soy|bread|wheat|honey|wine|beer)\\b")]));
    s_LIPS_TINGLE_RED(py_tuple(vec![py_str("\\b(?:swell\\w*|swollen|puff(?:ed|y)?|bloat\\w*)\\b"), py_str("\\breaction\\b"), py_str("\\bepipen\\b"), py_str("\\banaphyla\\w*\\b"), py_str("\\b(?:faint\\w*|dizzy|light\\s?headed|giddy|collapse\\w*|passing\\s+out)\\b"), py_str("\\b(?:breath\\w*|wheez\\w*|stridor|throat\\s+(?:tight|closing))\\b"), py_str("\\b(?:vomit\\w*|throwing\\s+up|sick)\\b"), py_str("\\b(?:thick|thickened|thickening|heavy|wooden|rubbery)\\b[^.?!]{0,20}\\b(?:tongue|lips?)\\b"), py_str("\\b(?:tongue|lips?)\\b[^.?!]{0,20}\\b(?:feels?|is|has\\s+gone|has\\s+got|going)\\s+(?:a\\s+bit\\s+|a\\s+little\\s+|really\\s+|very\\s+|so\\s+|all\\s+)?(?:thick|thickened|thickening|heavy|wooden|rubbery)\\b")]));
    s_LIPS_TINGLE_OFF(py_tuple(vec![py_str("\\b(?:dentist|dental|hygienist|filling|root\\s+canal|tooth\\s+out|wisdom\\s+tooth|injection|anaesthetic|anesthetic|numbing\\s+gel|novocaine|lidocaine)\\b"), py_str("\\b(?:panic|panicking|panic\\s+attack|anxiety|anxious|hyperventilat\\w*|over-?breath\\w*|breathing\\s+too\\s+fast|shaking|palpitations)\\b"), py_str("\\b(?:hands?|fingers?|fingertips|feet|toes?|arms?|legs?)\\b(?:(?!\\b(?:lips?|tongue|mouth|throat|face)\\b)[^.?!]){0,30}\\b(?:tingl\\w*|numb(?:ness|ed|ing|s)?|pins\\s+and\\s+needles|prickling)\\b"), py_str("\\b(?:tingl\\w*|numb(?:ness|ed|ing|s)?|pins\\s+and\\s+needles|prickling)\\b(?:(?!\\b(?:lips?|tongue|mouth|throat|face)\\b)[^.?!]){0,30}\\b(?:hands?|fingers?|fingertips|feet|toes?|arms?|legs?)\\b"), py_str("\\b(?:cold\\s+sore|cold\\s+weather|in\\s+the\\s+cold|out\\s+in\\s+the\\s+cold|wind|sunburn|chapped|freezing|numb\\s+with\\s+the\\s+cold)\\b"), py_str("\\b(?:b12|vitamin\\s+b12|deficien\\w*|multiple\\s+sclerosis|\\bms\\b|migraine|nerve\\s+damage|carpal|raynaud\\w*)\\b")]));
    s_SWALLOW_GONE_OFF(py_tuple(vec![py_str("\\b(?:pills?|tablets?|capsules?|medicines?|medicine|supplements?|vitamins?)\\b"), py_str("\\b(?:pride|insult|my\\s+words?|his\\s+words?)\\b"), py_str("\\b(?:dog|dogs|puppy|cat|cats|kitten|rabbit|hamster|guinea\\s+pig|horse|pony|bird|parrot|budgie|pet|pets)\\b"), py_str("\\b(?:bone|fishbone|gristle|a\\s+bit\\s+of\\s+(?:food|meat|steak|bread)|food)\\b[^.?!]{0,30}\\b(?:stuck|sticking|caught|lodged|went\\s+down\\s+the\\s+wrong)\\b"), py_str("\\b(?:stuck|sticking|lodged|caught)\\b[^.?!]{0,30}\\b(?:throat|gullet|windpipe)\\b"), py_str("\\b(?:fish\\s*bone|bone)\\b"), py_str("\\b(?:swallowed|gulped)\\b")]));
    s_SWALLOW_GONE_RED(py_tuple(vec![py_str("\\b(?:drool\\w*|dribbl\\w*)\\b"), py_str("\\b(?:lips?|tongue|throat|uvula|neck)\\b[^.?!]{0,24}\\b(?:swell\\w*|swollen|closing|thick)\\b"), py_str("\\b(?:swell\\w*|swollen|puff\\w*)\\b[^.?!]{0,24}\\b(?:lips?|tongue|throat|neck)\\b"), py_str("\\b(?:muffled|no\\s+voice|voice\\s+(?:has\\s+)?gone|lost\\s+my\\s+voice)\\b"), py_str("\\b(?:my\\s+(?:own\\s+)?spit|saliva)\\b")]));
    s_THROAT_TIGHT_CAUSE(py_tuple(vec![py_str("\\b(?:nervous|anxious|anxiety|worried|interview|presentation|speech|exam|speaking\\s+up)\\b"), py_str("\\b(?:blocked\\s+nose|nose\\s+is\\s+blocked|stuffy|stuffed\\s+up|bunged\\s+up|bunged|runny\\s+nose|nose\\s+(?:is\\s+)?running|congest\\w*|sinus\\w*|hayfever|hay\\s+fever|allerg\\w*)\\b"), py_str("\\b(?:in\\s+the\\s+morning|first\\s+thing|when\\s+i\\s+wake\\s+up|on\\s+waking|at\\s+night)\\b"), py_str("\\b(?:run|running|ran|jog\\w*|exercise|workout|gym|swim\\w*|cycl\\w*|bike|cardio|football|rugby|netball|hockey|squash|tennis|lifting|weights|sprinting|training)\\b"), py_str("\\b(?:sore|croaky|hoarse|dry|scratchy)\\s+throat\\b"), py_str("\\b(?:singing|sang|shouted|shouting|talking\\s+all\\s+day|yesterday|last\\s+night|weeks?\\s+ago|for\\s+days|hayfever|hay\\s+fever|allergies|post\\s?nasal|reflux|silent\\s+reflux|tonsils?|tonsillitis|strep|cold|flu|covid)\\b"), py_str("\\b(?:sometimes|occasionally|now\\s+and\\s+then)\\b")]));
    s_THROAT_TIGHT_EFFORT(py_tuple(vec![py_str("\\b(?:when|while|after|during)\\s+(?:i\\s+|im\\s+|i'?m\\s+)?(?:run|running|jog\\w*|exercis\\w*|workout|swim\\w*|cycl\\w*|train\\w*|lift\\w*|row\\w*|climb\\w*)\\b"), py_str("\\b(?:after|during|in)\\s+(?:a|the|my)\\s+(?:run|jog|workout|swim|cycle|ride|game|match|class|session|training|race)\\b"), py_str("\\b(?:gym|swimming|cycling|cardio|football|rugby|netball|hockey|squash|tennis|aerobics|spin\\s+class|bike\\s+ride|long\\s+run|sprints?|weights|lifting|hiit)\\b")]));
    s_THROAT_TIGHT_RED(py_tuple(vec![py_str("\\b(?:tongue|uvula)\\b"), py_str("\\b(?:swell\\w*|swollen|puff(?:ed|y)?|bloat\\w*)\\b[^.?!]{0,30}\\b(?:lips?|tongue|face|eyelids?|throat|uvula)\\b"), py_str("\\b(?:lips?|face|eyelids?)\\b[^.?!]{0,30}\\b(?:swell\\w*|swollen|puff(?:ed|y)?)\\b"), py_str("\\b(?:can'?t|cannot|couldn'?t|hard\\s+to|difficult\\s+to|difficulty)\\s+(?:swallow|breathe|breath)\\b"), py_str("\\b(?:breath\\w*|wheez\\w*|stridor|epipen|anaphyla\\w*)\\b"), py_str("\\bthroat\\b[^.?!]{0,12}\\b(?:is|feels?|'s|was)?\\s*(?:closing|closed|shutting|clamped)\\b"), py_str("\\b(?:closing|closed)\\s+(?:up\\s+)?my\\s+throat\\b"), py_str("\\bvoic\\w*\\b[^.?!]{0,20}\\b(?:gone|going|changed|changing|different|hoarse|raspy)\\b"), py_str("\\b(?:lips?|tongue|mouth|face)\\b[^.?!]{0,20}\\b(?:tingl\\w*|numb(?:ness|ed|ing|s)?|prickl\\w*)\\b"), py_str("\\b(?:peanuts?|nuts?|shellfish|prawns?|bee|wasp|sting\\w*|stung|penicillin|antibiotic\\w*)\\b")]));
    s_NOT_A_WOUND_OFF(py_tuple(vec![py_str("\\bit\\b"), py_str("\\b(?:pen|ink|printer|paint|brush|steak|meat|chicken|beef|lamb|sausage|burger|cake|tomato|beetroot|cherry|berry|berries|fruit|juice|sauce|jam|dye|radiator|pipe|tap|boiler|ceiling|mattress|wall|drain|nail\\s+varnish)\\b")]));
    s_NOSE_RED(py_tuple(vec![py_str("\\b(?:won'?t|wont|doesn'?t|doesnt|isn'?t|not)\\s+stopp?(?:ed|ing)?\\b"), py_str("\\bkeeps?\\s+(?:on\\s+)?bleeding\\b"), py_str("\\bstill\\s+bleeding\\b"), py_str("\\b(?:half\\s+an?\\s+hour|an?\\s+hour|hours|two\\s+hours|twenty\\s+minutes|20\\s+minutes|ten\\s+minutes|10\\s+minutes|fifteen\\s+minutes|15\\s+minutes)\\b"), py_str("\\b(?:pale|grey|gray|dizzy|faint|fainted|fainting|clammy|sweaty|unconscious|shaking)\\b"), py_str("\\b(?:blood\\s+thinners?|warfarin|clopidogrel|apixaban|rivaroxaban|aspirin|heparin)\\b"), py_str("\\b(?:after|from)\\s+(?:a\\s+|the\\s+)?(?:bang|blow|knock|hit|bump|fall|punch|punching|thump|smack)\\b"), py_str("\\bhit\\s+(?:my|his|her|their|the)\\s+(?:head|face|nose)\\b"), py_str("\\b(?:nose|nosebleed)\\b.{0,25}\\b(?:broken|fractured|crooked|out\\s+of\\s+place)\\b")]));
    s_MOUTH_BLEED_NOW(py_tuple(vec![py_str("\\b(?:won'?t|wont|doesn'?t|doesnt)\\s+stopp?(?:ed|ing)?\\b"), py_str("\\bnot\\s+stopp?ing\\b"), py_str("\\bkeeps?\\s+(?:on\\s+)?bleeding\\b"), py_str("\\bstill\\s+bleeding\\b"), py_str("\\bwon'?t\\s+stop\\s+bleeding\\b"), py_str("\\bbleeding\\s+(?:a\\s+lot|badly|loads|everywhere)\\b"), py_str("\\bbleeding\\s+all\\s+over\\b"), py_str("\\bblood\\s+in\\s+(?:my|his|her|their)\\s+mouth\\b")]));
    s__THIRD(py_dict(vec![(py_str("feel"), py_str("feels")), (py_str("keep"), py_str("keeps")), (py_str("seem"), py_str("seems")), (py_str("get"), py_str("gets")), (py_str("go"), py_str("goes")), (py_str("need"), py_str("needs")), (py_str("stop"), py_str("stops")), (py_str("look"), py_str("looks")), (py_str("come"), py_str("comes")), (py_str("start"), py_str("starts")), (py_str("do"), py_str("does")), (py_str("have"), py_str("has")), (py_str("are"), py_str("is")), (py_str("were"), py_str("was")), (py_str("aren't"), py_str("isn't")), (py_str("weren't"), py_str("wasn't"))]));
    s_PREGNANCY_WORDS(py_str("(?:pregnant|pregnancy|expecting\\s+a\\s+baby|weeks?\\s+pregnant|baby\\s+due|\\d+\\s+weeks?\\s+(?:along|gone|in)|(?:^|\\b)(?:i'?m|im|i\\s+am|my\\s+(?:wife|girlfriend|partner|fiancee|fianc\\w+))(?:\\s+is|\\s+was)?\\s+\\d+\\s+weeks?\\b(?!\\s+(?:into|old|ago|in|since|on|of|back)\\b))"));
    s_SOCKET_OBJECT(py_tuple(vec![py_str("\\b(?:fingers?|fingertips?|fork|forks|knife|knives|scissors|nails?|pins?|paper\\s?clips?|screwdrivers?|hair\\s?pins?|keys?|coins?|sticks?|pens?|pencils?|toys?|metal|wire|needles?|blades?|tools?|utensils?|spoons?|chopsticks?|tweezers|batter(?:y|ies)|earbuds?|bracelet|necklace)\\b")]));
    s_SOCKET_THING_FINE(py_tuple(vec![py_str("\\b(?:adapter|adaptors?|travel|holiday\\s+home|abroad|in\\s+(?:spain|france|greece|italy|portugal|turkey|america|the\\s+us)|type\\s+[a-o]\\b|voltage)"), py_str("\\b(?:loose|wobbly|cracked|broken|fallen\\s+off|came\\s+off|replac\\w*|rewir\\w*|install\\w*|fit\\s+(?:a|the)\\s+(?:new\\s+)?socket|electrician|extends?\\s+the\\s+socket|extension\\s+lead|surge)"), py_str("\\b(?:years?\\s+ago|last\\s+(?:year|decade|century)|when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little|small|young)|as\\s+a\\s+(?:kid|child)|in\\s+the\\s+(?:19|20)\\d\\d)"), py_str("\\b(?:how\\s+(?:does|do)\\s+(?:a|the)\\s+socket\\s+work|wiring|wire|which\\s+plug|plug\\s+on\\s+a\\s+(?:wire|cable|lead)|fuse\\s+in\\s+the\\s+plug)\\b")]));
    s_ELECTRIC_SHOCK_DOWN(py_tuple(vec![py_str("\\b(?:not|isn'?t|isnt|is\\s+not|wasn'?t|wasnt|was\\s+not|stopped|stop|stops|won'?t|wont)\\s+(?:breathing|breath\\w*)\\b"), py_str("\\b(?:not|isn'?t|isnt|is\\s+not)\\s+(?:breathing|breath\\w*)\\b"), py_str("\\bno\\s+(?:pulse|breath\\w*|heartbeat)\\b"), py_str("\\b(?:unconscious|unresponsive|knocked\\s+out|passed\\s+out|passing\\s+out|out\\s+cold|collapsed|collaps\\w*|in\\s+a\\s+heap|limp|floppy|blue\\s+lips|going\\s+blue)\\b"), py_str("\\b(?:cpr|resuscitat\\w*|revive\\w*|recovery\\s+position|defib\\w*|heart\\s+stopped)\\b"), py_str("\\b(?:is|isnt|isn'?t)\\s+not\\s+responding\\b|\\bnot\\s+responding\\b")]));
    s_SHOCK_ANY(py_tuple(vec![py_str("\\b(?:got|get|gets|getting|had|has|have|having|took|taken|suffered|given|gave|felt)\\s+(?:me\\s+|him\\s+|her\\s+|them\\s+)?(?:a\\s+|an\\s+|the\\s+)?(?:\\w+\\s+){0,2}shock\\b"), py_str("\\bshock(?:ed)?\\b"), py_str("\\b(?:zapped|electrocut\\w*)\\b"), py_str("\\blive\\s+(?:wire|cable|rail|terminal)\\b")]));
    s_ELECTRIC_SHOCK_DEFINITION(py_tuple(vec![py_str("\\b(?:what|why|how)\\s+(?:is|are|causes?|caused|does|do|did)\\b[^.?!]{0,30}\\bshock\\b"), py_str("\\bshock\\b[^.?!]{0,20}\\b(?:(?<!at\\s)(?<!from\\s)(?<!of\\s)work|works|happen|happens|affect|affects|do\\s+to|does\\s+to|cause|causes)\\b"), py_str("\\b(?:explain|definition\\s+of|meaning\\s+of)\\b[^.?!]{0,25}\\bshock\\b")]));
    s_POO_BLOOD(py_tuple(vec![py_str("\\bblood\\s+in\\s+(?:my|his|her|their|the)\\s+(?:poo|poo's|poop|stool|stools|shit|bowel\\s+movement|number\\s+two|toilet|loo)\\b"), py_str("\\b(?:pooed|pooped|crapped)\\s+(?:out\\s+)?(?:some\\s+)?blood\\b"), py_str("\\bblood\\s+came\\s+out\\s+(?:when|after)\\s+(?:i|he|she|they)\\s+(?:pooed|pooped|went)\\b"), py_str("\\b(?:poo|poop|stool|stools|shit|diarrhoea|diarrhea)\\b[^.?!]{0,20}\\b(?:with|has|had|have|containing|covered\\s+in)\\b[^.?!]{0,10}\\bblood\\b"), py_str("\\bbloody\\s+(?:poo|poop|stool|stools|shit|diarrhoea|diarrhea|bowel\\s+movement)\\b"), py_str("\\bred\\s+blood\\s+(?:in|on)\\s+(?:the\\s+)?(?:toilet|paper|loo|pan)\\b"), py_str("\\b(?:when\\s+i\\s+wipe|when\\s+he\\s+wipes|when\\s+she\\s+wipes|after\\s+i\\s+wipe)\\b[^.?!]{0,25}\\bblood\\b")]));
    s_POO_BLACK(py_tuple(vec![py_str("\\b(?:poo|poop|stool|stools|shit|number\\s+two)\\b[^.?!]{0,20}\\b(?:is|are|was|were|went|has\\s+gone|have\\s+gone|looks?|looked|turned|come\\s+out)\\s+(?:all\\s+)?(?:black|tarry|tar-?like|like\\s+tar|dark\\s+(?:red|reddish))\\b"), py_str("\\b(?:black|tarry)\\s+(?:and\\s+tarry\\s+)?(?:poo|poop|stool|stools|shit|diarrhoea|diarrhea)\\b"), py_str("\\b(?:poo|poop|stool|stools|shit|bowel\\s+movement)\\b[^.?!]{0,30}\\b(?:black|tarry|tar-?like|like\\s+tar)\\b"), py_str("\\b(?:tarry|black)\\s+(?:poo|poop|stool|stools|shit)\\b"), py_str("\\b(?:melaena|melena)\\b")]));
    s_POO_FINE(py_tuple(vec![py_str("\\b(?:blood\\s+sugar|blood\\s+pressure|blood\\s+test|blood\\s+group|bloody\\s+murder|bloody\\s+hell|blood\\s+money)\\b"), py_str("\\b(?:iron\\s+(?:tablets?|pills?|supplements?|infusion)|ferrous|bismuth|pepto|charcoal\\s+tablets?|kaolin)\\b"), py_str("\\b(?:black\\s+pudding|black\\s+beans?|black\\s+currents?|blackberries|blueberries|beetroot|beet\\s+juice|licorice|liquorice|spinach|food\\s+colouring)\\b"), py_str("\\b(?:stool\\s+test|test\\s+kit|screening\\s+kit|bowel\\s+screening|fit\\s+kit|faecal\\s+immunochemical|colonoscopy|endoscopy)\\b"), py_str("\\b(?:years?\\s+ago|decades?\\s+ago|as\\s+a\\s+kid|when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little)|in\\s+(?:a|the)\\s+film|on\\s+tv)\\b"), py_str("\\b(?:my|the|our)\\s+(?:dog|cat|puppy|kitten|rabbit|hamster|guinea\\s+pig|horse|bird|parrot|budgie|pet)s?\\b")]));
    s_STOOL_LONG(py_tuple(vec![py_str("\\b(?:for\\s+(?:a\\s+|the\\s+last\\s+|about\\s+a\\s+|over\\s+a\\s+|nearly\\s+a\\s+)?(?:\\d+\\s+)?(?:week|weeks|month|months|year|years))\\b"), py_str("\\b(?:since\\s+(?:i\\s+was\\s+\\w+|christmas|last\\s+year|the\\s+summer)|years\\s+now|months\\s+now|weeks\\s+now|a\\s+while\\s+now|ages|all\\s+my\\s+life|on\\s+and\\s+off)\\b")]));
    s_TORSION(py_tuple(vec![py_str("\\b(?:testicle|testicles|testis|ball|balls|ball\\s+sack|ballsack|scrotum|scrotal|bollock|bollocks|nad|nads|nuts)\\b[^.?!]{0,30}\\b(?:pain|painful|achy|ache|aching|sore|swollen|swelling|tender|lump|twisted|hard|heavy|torsion|hurts?|hurting)\\b"), py_str("\\b(?:testicle|testicles|testis|ball|balls|bollock|bollocks|scrotum|nad|nads)\\b[^.?!]{0,25}\\b(?:woke|woken|wakes|waking)\\b[^.?!]{0,15}\\b(?:me|him|her|them)\\b"), py_str("\\b(?:pain|ache|swelling|swollen|lump|tender|tenderness|hurt\\w*)\\b[^.?!]{0,20}\\b(?:in|of|on|at)\\s+(?:my|his|her|their|one|the)\\s+(?:testicle|testicles|ball|balls|ball\\s+sack|scrotum|bollock|side\\s+of\\s+the\\s+scrotum)\\b"), py_str("\\b(?:one|a)\\s+(?:of\\s+my\\s+|of\\s+his\\s+|of\\s+her\\s+)?(?:ball|testicle|bollock)s?\\b[^.?!]{0,20}\\b(?:bigger|swollen|sore|hurts?|aching|tender)\\b"), py_str("\\b(?:testicle|testicles|testicular|testis|ball|balls|bollock)\\b[^.?!]{0,25}\\b(?:torsion|twisted)\\b"), py_str("\\btesticular\\s+torsion\\b"), py_str("\\btorsion\\s+(?:in|of)\\s+(?:the|my|his|her|their)\\s+(?:testicle|ball)\\b"), py_str("\\b(?:sore|swollen|painful|tender|hard|twisted)\\b[^.?!]{0,20}\\b(?:testicle|testicles|scrotum|bollock|ball|balls)\\b")]));
    s_TORSION_FINE(py_tuple(vec![py_str("\\b(?:blue\\s+balls|jock\\s+itch|athletes?\\s+foot|sweaty|itch\\w*|chafe\\w*|chafing|shaving|shaved|wax\\w*|trim\\w*|razor|rash|dry\\s+skin|fungal|thrush|spots?|pimples?)\\b"), py_str("\\b(?:after|from)\\s+(?:cycling|the\\s+bike|a\\s+bike\\s+ride|a\\s+long\\s+ride|the\\s+gym|the\\s+squat\\s+rack|football|running|a\\s+run|sex|the\\s+marathon|riding)\\b"), py_str("\\bwhen\\s+(?:i|he|she|they)\\s+(?:run|cycle|ride|train|lift|walk|swim)\\b"), py_str("\\b(?:how\\s+(?:do|should)\\s+i\\s+check|self\\s+exam|check(?:ing)?\\s+(?:them|it)\\s+for|testicular\\s+cancer\\s+(?:awareness|check|screening)|what\\s+causes\\s+testicular)\\b")]));
    s_TESTIS_LUMP(py_tuple(vec![py_str("\\b(?:lump|lumps|bump|bumps|hard\\s+spot|pea-?sized)\\b"), py_str("\\b(?:one|a)\\s+(?:of\\s+(?:my|his|her|their)\\s+)?(?:ball|testicle|bollock)s?\\b[^.?!]{0,25}\\b(?:bigger|larger|enlarged|different)\\b"), py_str("\\b(?:bigger|larger|enlarged)\\b[^.?!]{0,20}\\b(?:than\\s+the\\s+other|ball|testicle)\\b")]));
    s_TORSION_ONSET(py_tuple(vec![py_str("\\b(?:sudden\\w*|all\\s+of\\s+a\\s+sudden|out\\s+of\\s+nowhere|came\\s+on|come\\s+on|started|start(?:s|ing)?|woke|woken|wakes|overnight|last\\s+night|this\\s+morning|this\\s+afternoon|an?\\s+hour\\s+ago|few\\s+hours|just\\s+now|right\\s+now|now\\s+(?:sore|swollen|hurting|painful)|one\\s+side|on\\s+one\\s+side|since\\s+(?:this|last)\\s+(?:morning|night)|in\\s+the\\s+last\\s+(?:hour|few\\s+hours))\\b")]));
    s_PUNCTURE(py_tuple(vec![py_str("\\b(?:stepped|stood|trod|trodden)\\b[^.?!]{0,20}\\b(?:on|in)\\b[^.?!]{0,20}\\b(?:a\\s+|the\\s+|some\\s+)?\\b(?:nail|screw|tack|needle|glass|barbed\\s+wire|spike|thorn)\\b"), py_str("\\b(?:nail|screw|tack|needle|spike|thorn|piece\\s+of\\s+glass)\\b[^.?!]{0,25}\\b(?:through|into|in)\\s+(?:my|his|her|their)\\s+(?:foot|shoe|boot|trainer|sock|hand|hand|palm|leg)\\b"), py_str("\\b(?:nail|screw|needle)\\s+(?:went|has\\s+gone|gone)\\s+(?:right\\s+)?(?:through|into)\\s+(?:my|his|her|their)\\b")]));
    s_BABY_FELL_RED(py_tuple(vec![py_str("\\b(?:sick|vomit\\w*|threw\\s+up|being\\s+sick|sleepy|drowsy|floppy|limp|wont\\s+wake|won'?t\\s+wake|unconscious|knocked\\s+out|out\\s+cold|blood|seizure|fitting|fit|breathing\\s+(?:funny|odd|strange)|not\\s+breathing|gasping|turning\\s+blue|not\\s+themselves|not\\s+feeding|slept\\s+(?:a\\s+lot|for\\s+ages|all\\s+day|since)|asleep|is\\s+sleeping|fell\\s+asleep|hard\\s+to\\s+wake|sleeping\\s+a\\s+lot|off\\s+their\\s+food|not\\s+waking\\s+(?:up|properly))\\b")]));
    s_BABY_FELL(py_tuple(vec![py_str("\\b(?:my|our|the)\\s+(?:baby|infant|newborn|toddler|little\\s+one|son|daughter|boy|girl|child)\\b[^.?!]{0,30}\\b(?:fell|fallen|falling|rolled|dropped|tumbled|toppled)\\b[^.?!]{0,15}\\b(?:off|from)\\b[^.?!]{0,20}\\b(?:bed|sofa|settee|couch|changing\\s+table|table|chair|highchair|pram|buggy|pushchair)\\b"), py_str("\\b(?:fell|fallen|falling|rolled|dropped|tumbling|tumbled)\\s+(?:off|from)\\s+(?:the\\s+|my\\s+|a\\s+)?(?:bed|sofa|settee|couch|changing\\s+table|highchair|pram|pushchair)\\b[^.?!]{0,30}\\b(?:baby|infant|newborn|son|daughter|boy|girl|little\\s+one)\\b"), py_str("\\b(?:baby|infant|newborn|toddler|\\d+\\s*(?:months?|weeks?)\\s+old)\\b[^.?!]{0,40}\\b(?:fell|fallen|falling|rolled|dropped|tumbled|toppled)\\b")]));
    s_BABY_FELL_OFF(py_tuple(vec![py_str("\\bf(?:ell|alls?|alling)\\s+(?:a\\s+)?asleep\\b"), py_str("\\b(?:dropp(?:ed|ing|s)|nod(?:ded|ding|s))\\s+(?:off|to\\s+sleep|asleep)\\b"), py_str("\\b(?:went|gone|goes|going)\\s+(?:off\\s+)?to\\s+sleep\\b"), py_str("\\bfell\\s+(?:in\\s+love|out\\s+(?:of|with)|over\\s+(?:nothing|again|laughing))\\b"), py_str("\\brolled\\s+(?:over|onto\\s+(?:his|her|their)\\s+(?:front|tummy|back))\\b"), py_str("\\bdropped\\s+(?:his|her|their|the)\\s+(?:dummy|comforter|toy|bottle|cup|spoon|muslin)\\b")]));
    s_BABY_FELL_SURFACE(py_tuple(vec![py_str("\\b(?:fell|fallen|falling|rolled|dropped|tumbled|toppled)\\s+(?:off|from)\\s+(?:the\\s+|my\\s+|a\\s+|her\\s+|his\\s+)?(?:bed|sofa|settee|couch|changing\\s+table|table|chair|highchair|pram|buggy|pushchair|stairs|steps|ladder|wall|bike)\\b"), py_str("\\b(?:bed|sofa|settee|couch|changing\\s+table|highchair|pram|buggy|pushchair)\\b[^.?!]{0,12}\\b(?:fell|fallen|falling|rolled|dropped|tumbled|toppled)\\b")]));
    s_HANGED_LONG_AGO(py_tuple(vec![py_str("\\bwhen\\s+(?:i|we|he|she|they)\\s+(?:were|was)\\s+(?:kids?|children|child|teenagers?|teen|teens|young|little|small|at\\s+school)\\b"), py_str("\\bwhen\\s+(?:he|she|they|my\\s+\\w+)\\s+was\\s+(?:young|a\\s+kid|a\\s+child|little|small|at\\s+school)\\b"), py_str("\\b(?:growing\\s+up|as\\s+a\\s+(?:kid|child|teenager)|in\\s+(?:my|his|her|their)\\s+(?:teens|twenties))\\b"), py_str("\\b(?:years?|decades?)\\s+(?:ago|back)\\b"), py_str("\\b(?:a\\s+)?(?:long|good\\s+few|good)\\s+(?:time|while|few\\s+years?)\\s+(?:ago|back)\\b"), py_str("\\b(?:long|time)\\s+ago\\b"), py_str("\\bin\\s+(?:19|20)\\d\\d\\b"), py_str("\\blast\\s+(?:year|term|spring|summer|autumn|fall|winter|christmas|xmas|month|week|night|new\\s+year|easter)\\b")]));
    s__STRUCK_RELATIVE(py_str("(?:mum|mom|mother|dad|father|son|daughter|husband|wife|partner|boyfriend|girlfriend|brother|sister|friend|baby|child|kid|boy|girl|man|woman|lady|guy|bloke|neighbou?r|colleague|gran|granny|nan|grandad|grandma|grandpa|mate|passenger|cyclist|pedestrian|toddler|teenager|nephew|niece|cousin|aunt|uncle|fiance|fiancee|stepson|stepdaughter|stepdad|stepmum|person|people)"));
    s_TRAPPED_HELD_BY_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|i|my\\s+(?:husband|wife|partner|ex|boyfriend|girlfriend|dad|mum|mom|father|mother|brother|sister|son|daughter|friend|mate|carer|uncle|aunt|cousin|nan|gran|granny|grandad|grandma|grandpa|stepdad|stepmum|flatmate|housemate|neighbou?r)|the\\s+(?:man|woman|lady|guy|driver|bloke|father|mother))\\b[^.?!]{0,20}\\b(?:trapped|pinned|locked|kept|keeps|holding|held|shut|stuck)\\s+(?:me|us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TRAPPED_VEHICLE_NOT_NOW(py_tuple(vec![py_str("\\bfor\\s+(?:hours|ages|a\\s+while|an\\s+hour|\\d+\\s*(?:hours?|minutes?|mins?))\\b"), py_str("\\btraffic\\b"), py_str("\\bjam\\b"), py_str("\\b(?:car|child|booster)\\s+seat\\b"), py_str("\\bbatter(?:y|ies)\\b"), py_str("\\bflat\\b"), py_str("\\bbreakdown\\b"), py_str("\\bpetrol\\b"), py_str("\\bdiesel\\b"), py_str("\\bpuncture\\b"), py_str("\\bcharged?\\b"), py_str("\\bparking\\b"), py_str("\\block(?:ed)?\\s+out\\b"), py_str("\\b(?:keys?|phone|mobile|wallet|purse|bag|handbag|teddy|dummy|toy|ball|money|cards?|pram|buggy|medicines?|pills?)\\b[^.?!]{0,25}\\b(?:locked|trapped|stuck|shut|wedged)\\b"), py_str("\\b(?:has|have|had|keeps?|kept|got)\\s+(?:me|him|her|us|them)\\s+(?:trapped|stuck|pinned)\\b"), py_str("\\byesterday\\b"), py_str("\\blast\\s+night\\b"), py_str("\\bago\\b"), py_str("\\blast\\s+(?:week|year|month|summer|winter)\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little|young|small)\\b"), py_str("\\bwhat\\s+if\\b"), py_str("\\bif\\s+(?:i|he|she|they|we|my\\s+\\w+)\\b")]));
    s_FAINT_STORY(py_tuple(vec![add(&add(&add(&add(&add(&add(&py_str("\\b(?:i|he|she|they|we|my\\s+"), &g_PERSON()), &py_str("|the\\s+")), &g_PERSON()), &py_str("|")), &g_PERSON()), &py_str(")\\s+(?:\\w+\\s+){0,3}(?:fainted|fainting|faints|passed\\s+out|passing\\s+out|blacked\\s+out|blacking\\s+out|collapsed|was\\s+unconscious|were\\s+unconscious|been\\s+unconscious|(?:was|were|been|went)\\s+out\\s+cold|out\\s+cold)\\b")), py_str("\\b(?:had\\s+)?(?:a\\s+)?(?:funny\\s+turn|blackout|fainting\\s+fit)\\b")]));
    s_OUT_COLD_REPLY(py_str("Ring 111 today rather than waiting it out and say how long they were out and what they were doing - a first blackout is looked into every time, and a child is seen that day rather than this week. If they were out for more than a minute, went down during sport, hit their head, or have not come round, that is 999."));
    s_OUT_COLD_ASLEEP(py_tuple(vec![py_str("\\bout\\s+cold\\b[^.?!]{0,20}\\b(?:by|until|till|til|all\\s+night|overnight|the\\s+whole|as\\s+soon\\s+as|bed|beds|bedtime|pillow|nap|naps|bottle|feed|sleep\\w*)\\b"), py_str("\\b(?:by|until|till|til|as\\s+soon\\s+as)\\s+(?:\\w+\\s+){0,2}\\bout\\s+cold\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,20}\\b(?:after|before|from)\\s+(?:the|a|his|her|my|their|our|that|this)?\\s*(?:run|walk|feed|bottle|shift|night\\s+shift|work|meal|dinner|lunch|breakfast|food|flight|drive|journey|day|game|match|film|movie|bath|swim|party|pub|wine|beer|drink\\w*|heat|exhaust\\w*|jet\\s?lag|gym|training|school|college|class|exercise|tiring\\s+day|long\\s+day|night\\s+out|night\\s+in|table|sofa|exercise)\\b")]));
    s_WITHDRAWAL_STOP(py_tuple(vec![py_str("\\b(?:stop(?:ped|ping)?|quit(?:ting)?|gave\\s+up|giving\\s+up|give\\s+up|come\\s+off|coming\\s+off|came\\s+off|get\\s+off|getting\\s+off|got\\s+off|off|detox\\w*|withdraw\\w*|cut\\s+down\\s+on|cut\\s+out|not\\s+had\\s+a\\s+drink|havent\\s+had\\s+a\\s+drink|didn'?t\\s+have\\s+a\\s+drink|without\\s+a\\s+drink|no\\s+booze|no\\s+alcohol|dry\\s+for|on\\s+the\\s+wagon)\\b[^.?!]{0,30}\\b(?:drink\\w*|alcohol|booze|beer|wine|vodka|whisky|whiskey|spirits?|pints?|lager|cider|brandy|rum|gin|shots?)\\b"), py_str("\\b(?:havent|hasn'?t|hadn'?t|never)\\s+had\\s+(?:a\\s+|any\\s+)?(?:drink|alcohol|booze|pint)\\b"), py_str("\\b(?:no|zero)\\s+(?:drinks?|alcohol|booze)\\s+(?:for|in|since)\\b"), py_str("\\b(?:drink\\w*|alcohol|booze|beer|wine|vodka|whisky|whiskey)\\b[^.?!]{0,30}\\b(?:withdraw\\w*|detox\\w*|dts?)\\b")]));
    s_WITHDRAWAL_NOT_ALCOHOL(py_tuple(vec![py_str("\\b(?:coffee|caffeine|tea|fizzy|juice|milk|water|squash|coke|cola|soda|lemonade|energy\\s+drinks?|sugar\\w*|sweets|smok\\w*|vap\\w*|nicotine|fags?|cigs?|cigarettes?|rollies|weed|cannabis|drugs?|painkillers?|screens?|tv|telly|gaming|gambl\\w*|porn|shopping|meat|dairy|gluten|phone|social\\s+media|instagram|facebook|tiktok|doomscroll\\w*)\\b")]));
    s_WITHDRAWAL_ALONE_AT_HOME(py_tuple(vec![py_str("\\b(?:detox\\w*|coming\\s+off|stopping|stopped|quitting|quit)\\b[^.?!]{0,40}\\b(?:at\\s+home|by\\s+myself|on\\s+my\\s+own|alone|myself|on\\s+his\\s+own|on\\s+her\\s+own)\\b[^.?!]{0,40}\\b(?:drink\\w*|alcohol|booze|beer|wine|vodka|whisky|whiskey)\\b"), py_str("\\b(?:detox\\w*)\\b[^.?!]{0,40}\\b(?:at\\s+home|by\\s+myself|on\\s+my\\s+own|alone|myself)\\b")]));
    s_WITHDRAWAL_ROUTINE(py_tuple(vec![py_str("\\bshak\\w*\\b[^.?!]{0,40}\\b(?:until|when|if|before)\\b[^.?!]{0,25}\\b(?:i|he|she)\\s+(?:have|has|had|get|gets|got|need|needs)\\s+(?:a|my|his|her)?\\s*(?:drink|beer|vodka|wine|gin|whisky|whiskey|pint)\\b"), py_str("\\b(?:need|needs|have|has|had)\\s+(?:a|my|his|her)\\s+(?:drink|beer|vodka|wine|gin|whisky|whiskey|pint)\\b[^.?!]{0,30}\\b(?:to\\s+)?(?:stop|stops|stopped|settle|ease)\\s+(?:the\\s+)?(?:shak\\w*|trembl\\w*|shakes|nerves)\\b"), py_str("\\bthe\\s+shakes\\s+(?:in\\s+the\\s+morning|every\\s+morning|when\\s+i\\s+wake)\\b"), py_str("\\bshak\\w*\\b[^.?!]{0,40}\\b(?:if|when|unless)\\b[^.?!]{0,25}\\b(?:i\\s+)?(?:dont|don'?t|do\\s+not|havent|haven'?t|miss|skip|go\\s+without)\\b[^.?!]{0,20}\\b(?:drink|drinks|beer|pint)\\b")]));
    s_WITHDRAWAL_SYMPTOM(py_tuple(vec![py_str("\\bshak\\w*"), py_str("\\bshaky\\b"), py_str("\\btrembl\\w*"), py_str("\\btremors?\\b"), py_str("\\bsweat\\w*"), py_str("\\bsweaty\\b"), py_str("\\bclammy\\b"), py_str("\\bnause\\w*"), py_str("\\bvomit\\w*"), py_str("\\bbeing\\s+sick\\b"), py_str("\\bthrowing\\s+up\\b"), py_str("\\bcan'?t\\s+sleep\\b"), py_str("\\bcant\\s+sleep\\b"), py_str("\\bnot\\s+sleeping\\b"), py_str("\\binsomnia\\b"), py_str("\\bheart\\s+(?:is\\s+)?(?:racing|pounding)\\b"), py_str("\\bpalpitations?\\b"), py_str("\\banxious\\b"), py_str("\\banxiety\\b"), py_str("\\bjumpy\\b"), py_str("\\bon\\s+edge\\b"), py_str("\\bnight\\s+sweats?\\b"), py_str("\\bfeel\\w*\\s+(?:awful|terrible|rough|dreadful|ill|horrible)\\b"), py_str("\\bheadache\\b"), py_str("\\bshiver\\w*"), py_str("\\bcraving\\w*")]));
    s_WITHDRAWAL_STRONG(py_tuple(vec![py_str("\\bshak\\w*"), py_str("\\bshaky\\b"), py_str("\\btrembl\\w*"), py_str("\\btremors?\\b"), py_str("\\bsweat\\w*"), py_str("\\bsweaty\\b"), py_str("\\bclammy\\b"), py_str("\\bheart\\s+(?:is\\s+)?(?:racing|pounding)\\b"), py_str("\\bpalpitations?\\b"), py_str("\\bhallucinat\\w*"), py_str("\\b(?:see|saw|seen|seeing)\\s+things\\b"), py_str("\\b(?:hear|heard|hearing)\\s+(?:things|voices)\\b"), py_str("\\bfits?\\b"), py_str("\\bseizures?\\b"), py_str("\\bdelirium\\b"), py_str("\\bconfus\\w*")]));
    s_WITHDRAWAL_DANGER(py_tuple(vec![py_str("\\bis\\s+(?:that|it|this)\\s+(?:dangerous|serious|bad|normal)\\b"), py_str("\\bcan\\s+(?:it|that|this)\\s+(?:kill|be\\s+dangerous|hurt\\s+me)\\b"), py_str("\\bwill\\s+(?:i|he|she|they)\\s+be\\s+ok\\b"), py_str("\\bshould\\s+i\\s+(?:worry|be\\s+worried)\\b"), py_str("\\bam\\s*i\\s+in\\s+danger\\b"), py_str("\\bwhy\\s+(?:does|do)\\b[^.?!]{0,20}\\bshak\\w*")]));
    s_WITHDRAWAL_ASK_DANGER(py_tuple(vec![py_str("\\b(?:call|ring|phone|dial)\\s+(?:an?\\s+)?(?:999|911|112|ambulance|emergency)\\b"), py_str("\\bshould\\s+(?:i|we|he|she|they)\\s+(?:call|ring|phone|worry|be\\s+worried)\\b"), py_str("\\bdo\\s+(?:i|we)\\s+need\\s+(?:an?\\s+)?ambulance\\b"), py_str("\\b(?:can|could|will|would)\\s+(?:i|you|he|she|they)\\s+(?:die|be\\s+in\\s+danger)\\b"), py_str("\\bis\\s+(?:it|that|this)\\s+(?:dangerous|serious|deadly|fatal)\\b"), py_str("\\bis\\b[^.?!]{0,30}\\b(?:dangerous|deadly|fatal|life[-\\s]?threatening)\\b")]));
    s_WITHDRAWAL_HAIR_OF_THE_DOG(py_tuple(vec![py_str("\\b(?:a\\s+)?(?:beer|pint|lager|glass\\s+of\\s+wine|wine|vodka|drink|drinks|stella|whisky)\\b[^.?!]{0,40}\\b(?:help|helps|sleep|shake|shakes|shaking|stop|settle|ease|calm|take\\s+the\\s+edge)\\b"), py_str("\\bhair\\s+of\\s+the\\s+dog\\b"), py_str("\\b(?:should|can|could|shall)\\s+i\\s+(?:have|drink|get)\\s+(?:a|another|one)\\b"), py_str("\\b(?:is\\s+it\\s+ok|is\\s+it\\s+alright|would\\s+it\\s+be\\s+ok)\\s+to\\s+(?:have|drink)\\b")]));
    s_WITHDRAWAL_NAMED(py_tuple(vec![py_str("\\bwithdraw\\w*"), py_str("\\bdetox\\w*"), py_str("\\bdelirium\\s+tremens\\b"), py_str("\\bdts\\b"), py_str("\\b(?:coming|come|came|getting|get|got|am|im|i'?m)\\s+off\\s+(?:the\\s+|my\\s+)?(?:drink|booze|alcohol|wine|beer)\\b")]));
    s_WITHDRAWAL_HALLUCINATION(py_tuple(vec![py_str("\\b(?:see|sees|saw|seen|seeing)\\s+things\\b"), py_str("\\b(?:hear|hears|heard|hearing)\\s+(?:things|voices)\\b"), py_str("\\bvoices\\b"), py_str("\\b(?:taste|tastes|tasted|tasting)\\s+things\\b"), py_str("\\b(?:smell|smells|smelt|smelled|smelling)\\s+things\\b"), py_str("\\b(?:see|sees|saw|seeing|hear|hears|heard|hearing|smell|smells|smelt|smelling|taste|tastes|tasted|tasting)\\b[^.?!]{0,25}\\b(?:that|which)\\s+(?:isnt|isn'?t|is\\s+not|wasnt|aren'?t|arent)\\s+(?:really\\s+)?there\\b")]));
    s_WITHDRAWAL_SENSES_FINE(py_tuple(vec![py_str("\\b(?:see|sees|seeing)\\s+things\\s+(?:differently|clearly|straight|in\\s+a\\s+different\\s+way|in\\s+a\\s+different\\s+light|from\\s+a\\s+different\\s+angle|more\\s+clearly)\\b"), py_str("\\b(?:taste|tastes|tasting|smell|smells|smelling)\\s+things\\s+(?:again|properly|at\\s+last|now)\\b"), py_str("\\bcan\\s+(?:taste|smell)\\s+things\\b(?!\\s*(?:and|but)\\b)")]));
    s_HAIR_SCORCH(py_tuple(vec![py_str("\\b(?:singe[ds]?|singeing|scorch(?:ed|es|ing)?|burnt|burned)\\b[^.?!]{0,25}\\b(?:my|his|her|their|your)\\s+(?:hair|fringe|bangs|eyebrows?|eyelashes|lashes|beard|moustache|mustache)\\b"), py_str("\\b(?:my|his|her|their|your)\\s+(?:hair|fringe|bangs|eyebrows?|eyelashes|lashes|beard|moustache|mustache)\\b[^.?!]{0,20}\\b(?:is|are|got|was|were|looks?|keeps?)?\\s*(?:singed|scorched|burnt|burned|crispy)\\b")]));
    s_HAIR_SCORCH_ANSWER(py_tuple(vec![py_str("That is only hair, and hair grows back - trim the crunchy ends and it stops smelling. The part to check is the skin underneath: if the forehead, the eyelid or the scalp is red, blistered or sore, run it under cool water for twenty minutes and get it looked at today, because a burnt eyelid or brow can swell."), py_str("Nothing to do about the hair itself except trim the crispy ends off - it grows back. Do look at the skin though: red, blistered or sore skin under a singed fringe or brow wants twenty minutes under cool water and a same-day check, since burnt skin near the eye swells.")]));
    s_BURN_MARK_ON_A_BODY(py_tuple(vec![py_str("\\b(?:burn|burns|burned|burnt|scorch|scorches|scorched|singe|singes|singed)\\s+marks?\\s+(?:on|to)\\s+(?:my|his|her|their|your)\\s+(?:hand|hands|finger|fingers|thumb|arm|arms|wrist|leg|legs|foot|feet|skin|face|chest|neck|shoulder|shoulders|back|thigh|tummy|belly)\\b")]));
    s_LIMP_MECHANISM(py_tuple(vec![py_str("\\bgiv(?:e|es|ing|en)\\s+way\\b"), py_str("\\bkeeps?\\s+rolling\\b"), py_str("\\b(?:ankle|knee|foot|hip)s?\\s+roll(?:s|ed)?\\b"), py_str("\\broll(?:s|ed|ing)\\s+(?:my|his|her|their|the)\\s+(?:knee|ankle|shoulder|hip)\\b"), py_str("\\bunstable\\b"), py_str("\\b(?:knee|ankle|shoulder|hip|joint)s?\\s+(?:is|feels?|has\\s+been)\\s+(?:giving\\s+way|giving\\s+out|unstable|loose)\\b")]));
    s_LIMP_OTHER_LIMB(py_tuple(vec![py_str("\\blimp\\s+(?:wrist|wrists?|hands?|arms?|fingers?|thumbs?|elbows?|shoulders?)\\b"), py_str("\\blimp\\s+(?:in|on)\\s+(?:his|her|my|their|the)\\s+(?:wrist|wrists?|hands?|arms?|fingers?|thumbs?|elbows?|shoulders?)\\b"), py_str("\\b(?:wrist|wrists?|hands?|arms?|fingers?|thumbs?|elbows?|shoulders?)\\s+(?:has\\s+|have\\s+)?(?:gone\\s+|all\\s+)?limp\\b")]));
    s_LIMP_LONG(py_tuple(vec![py_str("\\bfor\\s+(?:the\\s+(?:last|past)\\s+)?(?:(?:[0-9]+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\\s+couple(?:\\s+of)?|a\\s+few|few|several|many|over\\s+a|about\\s+a|nearly\\s+a|almost\\s+a|half\\s+a|a|an|the)\\s+)?(?:week|weeks|month|months|year|years|ages)\\b"), py_str("\\bsince\\s+(?:last|christmas|the\\s+summer|a\\s+\\w+\\s+ago|\\d{4})\\b"), py_str("\\b(?:weeks|months|years)\\s+now\\b")]));
    s_LIMP_SOMEBODY_ELSE(py_tuple(vec![py_str("\\b(?:my|his|her|their|our)\\s+(?:mum|mom|mother|dad|father|son|daughter|boy|girl|child|kid|little\\s+one|baby|nan|nana|gran|granny|grandma|grandad|grandpa|husband|wife|partner|brother|sister|uncle|aunt|friend|neighbour|neighbor)\\b"), py_str("\\b(?:he|she|they)\\s+(?:has|have|had|got|is|was|were|walks|walk|limps|limp|hobbles|hobble)\\b")]));
    s_RING_DANGER(py_tuple(vec![py_str("\\bswollen\\b"), py_str("\\bswelling\\b"), py_str("\\bswelled\\b"), py_str("\\bswell\\b"), py_str("\\btoo\\s+tight\\b"), py_str("\\bthrobbing\\b"), py_str("\\bagony\\b"), py_str("\\bcutting\\s+(?:in|into)\\b"), py_str("\\bcut\\s+into\\b"), py_str("\\b(?:gone|going|turned|turning)\\s+blue\\b"), py_str("\\bblue\\b"), py_str("\\bpurple\\b"), py_str("\\bnumb\\b"), py_str("\\bblack\\b")]));
    s_RING_NOT_WORN(py_tuple(vec![py_str("\\b(?:don'?t|do\\s+not|doesn'?t|didn'?t|never|not)\\s+(?:wear|wearing|wore)\\s+(?:a\\s+|any\\s+|my\\s+)?(?:ring|rings|jewellery|jewelry)\\b"), py_str("\\bno\\s+rings?\\b"), py_str("\\bno\\s+ring\\s+on\\b"), py_str("\\b(?:don'?t|do\\s+not|doesn'?t)\\s+(?:have|wear|got)\\s+(?:a\\s+|any\\s+|my\\s+)?rings?\\s+on\\b"), py_str("\\bwithout\\s+(?:a\\s+|my\\s+|any\\s+)?rings?\\b"), py_str("\\b(?:took|take|taken|got|get|getting)\\s+(?:my|the|her|his|their)\\s+rings?\\s+off\\b"), py_str("\\b(?:my|the|her|his|their)\\s+rings?\\s+(?:is|was|are|were)?\\s*(?:already\\s+)?off\\b"), py_str("\\b(?:ring|rings)\\s+(?:is|are|was|were)\\s+off\\b"), py_str("\\bwhere\\s+(?:my|the|her|his|their)\\s+rings?\\s+(?:normally|usually|used\\s+to\\s+be)\\b")]));
    s_HOW_TO_TREAT(py_tuple(vec![py_str("\\b(?:how\\s+(?:do|should|can)\\s+(?:i|you|we)|what'?s\\s+the\\s+best\\s+way\\s+to)\\b.{0,30}\\b(?:treat|care\\s+for|clean|dress|look\\s+after|soothe|deal\\s+with)\\b"), py_str("\\bwhat\\s+(?:should|do|can)\\s+(?:i|we)\\s+do\\s+(?:about|for|with)\\b.{0,40}\\b(?:burn|scald|nosebleed|sprain\\w*|splinter|bruise|graze|sunburn|twisted\\s+ankle)\\b"), py_str("\\bwhat\\s+(?:should|do|can)\\s+(?:i|we)\\s+do\\s+(?:about|for|with)\\s+(?:a|this|that|my|his|her)\\s+cut\\b(?!\\s+(?:in|to|from|of|price|budget|hours|pay|staff|spending)\\b)"), py_str("\\b(?:treat|care\\s+for|look\\s+after)\\b.{0,20}\\b(?:a|an|my|the)\\s+(?:burn|cut|scald|nosebleed|sprain|splinter|bruise|graze)\\b")]));
    s_OWN_INJURY_QUESTION(py_tuple(vec![py_str("\\bwhat\\s+(?:if|do\\s+(?:i|you)\\s+do\\s+(?:if|when)|should\\s+i\\s+do\\s+(?:if|when)|happens\\s+if)\\b"), py_str("\\bhow\\s+(?:do|should|can)\\s+i\\s+(?:treat|care\\s+for|dress|clean|soothe)\\b"), py_str("\\bwhat(?:'?s|\\s+is)\\s+the\\s+(?:first\\s+aid|best\\s+way\\s+to\\s+treat)\\b")]));
    s_OWN_INJURY_IN_MY_CARE(py_tuple(vec![py_str("\\b(?:my|our|your)\\s+(?:son|daughter|child|kid|kids|children|baby|babies|toddler|infant|little\\s+one|husband|wife|partner|boyfriend|girlfriend|mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nan|nana|niece|nephew|grandson|granddaughter)\\b")]));
    s_THING_BURNT_WORDS(py_str("(?:onion|garlic|toast|bread|cake|biscuits?|potatoes?|rice|pasta|sauce|soup|stew|curry|dinner|lunch|tea|food|meat|chicken|steak|fish|veg|vegetables?|pizza|pan|pots?|saucepans?|wok|oven|grill|toaster|tray|tin|dish|baking|casserole|bottom|base|crust|oil|butter|milk|sugar|carpet|shirt|iron|cd|cds|dvd|dvds|disc|discs|disk|disks|candle|candles)"));
    s_THING_BURNT(py_tuple(vec![add(&add(&py_str("\\b(?:burnt|burned|scorched|charred|blackened|burning|burns?)\\s+(?:(?:my|the|our|a|an|his|her|their|your|some|black|brown|dark|sticky|thick|hard|orange|crispy)\\s+)*"), &g_THING_BURNT_WORDS()), &py_str("\\b")), add(&add(&py_str("\\b"), &g_THING_BURNT_WORDS()), &py_str("\\s+(?:(?:is|was|got|gets|looks|has|had|keeps\\s+getting)\\s+)?(?:\\w+\\s+){0,2}(?:burnt|burned|scorched|charred)\\b")), py_str("\\bburnt\\s+(?:onto|on\\s+to|to\\s+the\\s+bottom)\\b"), py_str("\\bburnt[\\s-]*on\\b[^.?!]{0,30}\\boff\\b"), py_str("\\b(?:stuck|stuck-on|stuck\\s+on)\\b[^.?!]{0,30}\\bburn\\w*\\b"), py_str("\\b(?:burnt|burned|scorched|charred|blackened)\\b(?:\\s+\\w+){0,3}\\s+(?:tin|tins|trays?|pans?|pots?|saucepans?|wok|dishes?|sheet|shelf|shelves|rack|mould|springform|traybake)\\b"), py_str("\\bburn(?:ed|t|s|ing)?\\s+(?:\\w+\\s+){0,2}(?:cd|cds|dvd|dvds|disc|discs|disk|disks|candle|candles|incense)\\b")]));
    s_BURNT_SELF(py_tuple(vec![py_str("\\b(?:burnt|burned|scalded)\\s+(?:myself|me|himself|herself|themselves)\\b"), py_str("\\b(?:i|he|she|they)\\s+(?:was|were|got|am|is)\\s+(?:burnt|burned|scalded)\\b"), py_str("\\b(?:i|we|he|she|they)\\s+(?:got|get|have|has|had)\\s+(?:a|an|my|his|her|their|me|some)?\\s*(?:bad|nasty|little|nasty|small|sore|red)?\\s*burn\\b"), py_str("\\b(?:a|the)\\s+(?:bad|nasty|little|small|sore|red)?\\s*burn\\s+(?:from|off)\\b"), add(&py_str("\\b(?:a|the)\\s+(?:bad|nasty|little|small|sore|red)?\\s*burn\\s+(?:on|to)\\s+(?:my|his|her|their)\\s+"), &crate::gen::modules::safety_cards::g_BURN_ON_BODY())]));
    s_RAZOR_BURN(py_tuple(vec![py_str("\\b(?:razor\\w*|sha(?:v|ve|ves|ved|ving)|shaving|shave|epilat\\w+|wax(?:ed|ing)?|pluck\\w*|tweezers?)\\b[^.?!]{0,25}\\bburn\\w*\\b"), py_str("\\bburn\\w*\\b[^.?!]{0,25}\\b(?:from|after|by|off)\\s+(?:a\\s+|the\\s+|my\\s+|her\\s+|his\\s+)?(?:razor\\w*|sha(?:v|ve|ves|ved|ving)|shaving|epilat\\w+|wax(?:ed|ing)?)\\b")]));
    s_FRICTION_WORD(py_tuple(vec![py_str("\\b(?:boots?|shoes?|trainers?|socks?|slippers?|sandals?|flip[\\s-]?flops?|heels?|rucksack|backpack|gloves?|rake|shovel|handle|rowing|oar|gym|running|runs|hiking|hike|walking|walked|marathon|new\\s+(?:boots?|shoes?|trainers?))\\b"), py_str("\\b(?:from|by)\\s+(?:the\\s+)?(?:rubbing|friction)\\b"), py_str("\\bblister\\s+(?:plaster|pad|patch)\\b"), py_str("\\b(?:pop|pops|popping|popped|burst|bursting|drain|draining)\\b"), py_str("\\b(?:how\\s+(?:do\\s+i|to|should\\s+i)|should\\s+i|what\\s+(?:should\\s+i\\s+do|do\\s+i\\s+do))\\s+(?:about|for|with)?\\s*(?:treat|treating|deal\\s+with|look\\s+after|cover|dress|care)\\b")]));
    s_BLISTER_STATE(py_tuple(vec![py_str("\\b(?:my|his|her|their)\\s+(?:hand|hands|finger|fingers|thumb|thumbs|palm|palms|arm|arms|wrist|wrists|leg|legs|thigh|thighs|shoulder|shoulders|back|neck|face|armpit)\\b[^.?!]{0,25}\\bblister\\w*\\b"), py_str("\\b(?:i|he|she|they)\\s+(?:have|has|got|ve\\s+got)\\s+(?:a|an|some|a\\s+few)\\s+blister\\w*\\s+(?:on|across|under)\\s+(?:my|his|her|their)\\s+(?:hand|hands|finger|fingers|thumb|palm|palms|arm|arms|wrist|writs|leg|legs|thigh|thighs|shoulder|shoulders|back|neck|armpit)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:hand|hands|finger|fingers|thumb|palm|palms|arm|arms|wrist|wrists|leg|legs|thigh|thighs|shoulder|shoulders|back|neck|armpit)\\b[^.?!]{0,20}\\b(?:is|are|got|has|have|keeps?)\\b[^.?!]{0,12}\\bblister\\w*\\b")]));
    s_BLISTER_CAUSE_BURN(py_tuple(vec![py_str("\\b(?:bleach\\w*|chemical\\w*|acid\\w*|detergent\\w*|clean(?:ing|ed|ers?)\\b|disinfectant\\w*|degreaser\\w*|drain\\s+unblocker|caustic\\w*|lye|ammonia|white\\s+spirit|paint\\s+thinner|turpentine|paraffin|petrol|soda\\s+crystals)\\b"), py_str("\\b(?:sun\\w*|uv\\s+light|sunbed|sunlamp|nettles?|stinging\\s+nettles?|hogweed)\\b")]));
    s_BLISTER_STATE_REPLIES(py_tuple(vec![py_str("Leave it whole and cover it with a blister plaster or a clean non-fluffy dressing, and take the pressure off whatever rubbed it. Don't burst it: a blister that gets hot, spreading or weeping, or one you cannot keep off, should be seen."), py_str("Leave it alone and cover it with a blister plaster, and stop whatever is rubbing it. Don't pop it; if it gets hot, red or weeping, or you are diabetic, get it looked at.")]));
    s_HOW_LONG(py_tuple(vec![py_str("\\bhow\\s+long\\b"), py_str("\\bhow\\s+many\\s+(?:days|weeks)\\b"), py_str("\\bhow\\s+long\\s+(?:will|would|does|do|till|until|before|to)\\b"), py_str("\\bwhen\\s+(?:will|does|should)\\s+it\\s+(?:heal|be\\s+(?:better|right|healed|back))\\b"), py_str("\\btakes?\\s+to\\s+heal\\b"), py_str("\\bhealing\\s+time\\b"), py_str("\\brecovery\\s+time\\b")]));
    s_HEAL_WORDS(py_tuple(vec![py_tuple(vec![py_str("sprain"), py_str("\\bsprain\\w*|\\btwisted\\s+(?:my|an?|the)\\s+(?:ankle|knee|wrist)|\\b(?:rolled|went)\\s+over\\s+on\\s+(?:my|an?|the)\\s+ankle\\b")]), py_tuple(vec![py_str("burn"), py_str("\\bburn(?:s|ed|t|ing)?\\b|\\bscald\\w*")]), py_tuple(vec![py_str("cut"), py_str("\\bcuts?\\b|\\bgash\\w*|\\bgraz(?:e|ed|es)\\b")]), py_tuple(vec![py_str("blister"), py_str("\\bblisters?\\b")])]));
    s_INJURY_WORDS(py_tuple(vec![py_tuple(vec![py_str("mouth_burn"), py_tuple(vec![py_str("\\btongue\\b"), py_str("\\broof\\s+of\\s+(?:my|the)\\s+mouth")])]), py_tuple(vec![py_str("burn"), py_tuple(vec![py_str("\\bburn(?:s|t|ed|ing)?\\b"), py_str("\\bscald(?:s|ed|ing)?\\b"), py_str("\\bblister(?:s|ed|ing)?\\b")])]), py_tuple(vec![py_str("sunburn"), py_tuple(vec![py_str("\\bsun\\s?burn(?:s|t|ed|ing)?\\b")])]), py_tuple(vec![py_str("cut"), py_tuple(vec![py_str("\\bcut(?:s|ting)?\\b"), py_str("\\bgraze(?:s|d|ing)?\\b"), py_str("\\bscrape(?:s|d|ing)?\\b"), py_str("\\bpaper\\s+cuts?\\b")])]), py_tuple(vec![py_str("nosebleed"), py_tuple(vec![py_str("\\bnose\\s*bleed")])]), py_tuple(vec![py_str("sprain"), py_tuple(vec![py_str("\\bsprain"), py_str("\\btwisted"), py_str("\\bankle\\b"), py_str("\\bknee\\b")])]), py_tuple(vec![py_str("splinter"), py_tuple(vec![py_str("\\bsplinter")])])]));
    s_OWN_INJURY(py_tuple(vec![py_str("\\b(?:burn\\w*|scald\\w*|cut\\w*|bled|bleeding|sprain\\w*|fractur\\w*|broke|broken|blister\\w*|spill\\w*|spilt|pour\\w*|tip\\w*|knock\\w*|dropped|splash\\w*|threw)\\b[^.?!]{0,25}\\b(?:my|me|myself|your|yourself)\\b"), py_str("\\b(?:my|me|myself|your|yourself)\\b[^.?!]{0,25}\\b(?:burn\\w*|scald\\w*|cut\\w*|bleed\\w*|sprain\\w*|fractur\\w*|broken|blister\\w*)\\b"), py_str("\\b(?:i|we|you)\\s+(?:get|got|gets|am|is|was)\\s+(?:burnt|burned|scalded|blistered)\\b"), py_str("\\b(?:first\\s+aid\\s+for|treat|treating|soothe|do\\s+for|good\\s+for|best\\s+(?:thing\\s+)?for)\\b[^.?!]{0,15}\\b(?:a|the|this)?\\s*(?:bad\\s+|red\\s+|nasty\\s+|serious\\s+)?(?:burn|scald)\\b")]));
    s_SOMEONE_ELSE_OWN(py_tuple(vec![py_str("\\b(?:my|our|the|a)\\s+(?:son|boys?|daughter|girl|child|kid|kids|baby|toddler|husband|wife|partner|mum|mom|mother|dad|father|brother|sister|friend|mate|nana|nan|gran|grandma|grandad|grandpa|neighbou?r|colleague|boss|flatmate|housemate|dog|cat|puppy|kitten)\\b")]));
    s_CLEANABLE_SURFACE(py_tuple(vec![py_str("\\b(?:pans?|pots?|saucepans?|woks?|ovens?|grills?|hobs?|cookers?|rings?|trays?|tins?|dishes?|dish|baking|sheets?|plates?|bowls?|worktops?|sink|racks?|griddles?|barbecues?|bbqs?|burners?|casseroles?|toasters?|glasses?|glass|door)\\b")]));
    s_PERSON_BURNT(py_tuple(vec![py_str("\\b(?:got|get|gets|getting)\\s+(?:burnt|burned|scalded)\\b(?=\\s*(?:$|[.,!?]|on\\b|at\\b|in\\b|by\\b|from\\b|when\\b|while\\b|and\\b|so\\b|because\\b))"), py_str("\\b(?:i|he|she|they|we)\\s+(?:burnt|burned|scalded)\\b")]));
    s_WHOLE_BODY_RED_FLAG(py_tuple(vec![py_str("\\b(?:hard\\s+to\\s+wake|won'?t\\s+wake|wont\\s+wake|not\\s+waking|floppy|and\\s+limp\\b(?!ing)|limp\\s+and\\s+(?:floppy|blue))\\b"), py_str("\\b(?:confused|confusion|disorientated|delirious|not\\s+making\\s+sense|unresponsive|not\\s+responding|vacant)\\b"), py_str("\\b(?:seizure|seizures|convulsion|convulsions|fitting|having\\s+a\\s+fit|fitted)\\b"), py_str("\\b(?:struggling\\s+to\\s+breathe|breathing\\s+(?:fast|quickly|hard)|grunting|blue\\s+lips|grey\\s+lips|gray\\s+lips|lips\\s+are\\s+blue|clammy)\\b"), py_str("\\b(?:stiff\\s+neck|non[-\\s]?blanching)\\b")]));
    s_MILD_BODY_SIGN(py_tuple(vec![py_str("\\b(?:drowsy|drowsiness|sleepy|too\\s+sleepy|listless)\\b"), py_str("\\b(?:vomiting|vomited|throwing\\s+up|thrown\\s+up|projectile\\s+vomit\\w*)\\b"), py_str("\\b(?:not\\s+(?:himself|herself|themselves)|unlike\\s+(?:him|her)self)\\b"), py_str("\\b(?:not\\s+(?:feeding|feeding\\s+well|eating|eating\\s+much|drinking|drinking\\s+much)|off\\s+(?:his|her|their)\\s+(?:food|feeds|milk|bottles?)|refusing\\s+(?:feeds|food|bottles?|the\\s+bottle))\\b"), py_str("\\b(?:i|he|she|they|we|baby|child|son|daughter|kid|boy|girl|toddler|mum|mom|dad|mother|father|nan|nana|grandad|grandma|husband|wife|partner|friend|sister|brother|niece|nephew)\\b[^.?!]{0,20}\\b(?:not\\s+right|not\\s+(?:his|her|their)\\s+usual\\s+self)\\b(?!\\s+(?:now|away|off|there|here|yet))")]));
    s_ASKS_ABOUT_THE_INJURY(py_tuple(vec![py_str("\\bhow\\s+long\\b"), py_str("\\bhow\\s+many\\s+(?:days|weeks)\\b"), py_str("\\btakes?\\s+to\\s+heal\\b"), py_str("\\bhealing\\s+time\\b"), py_str("\\brecovery\\s+time\\b"), py_str("\\bhow\\s+(?:often|soon)\\b"), py_str("\\bshould\\s+i\\s+(?:cover|bandage|dress|wrap|clean|wash|soak|pop|burst)\\b"), py_str("\\b(?:do|should)\\s+i\\s+need\\s+(?:a|to\\s+see)\\b"), py_str("\\bis\\s+it\\s+(?:meant|supposed)\\s+to\\b")]));
    s_A_FOLLOW_UP_ABOUT_THE_INJURY(call_builtin("frozenset", &[mcall(&py_str("\nhow long will would does do did take takes taken heal heals healed healing time times day days week weeks\n till until til\nmany much soon often better right normal back recover recovered recovery\nit that this these they them its theirs one\ni my me should can could shall may might\na an the to be been being is are was were of about on in at for with and or then now not\ncover covered covering bandage bandaged dressing dress wrapped wrap plaster plasters\nclean cleaning wash washing soak soaking pop burst drain\nneed needs see seen looked look doctor nurse gp hospital a and e\nsore red swollen painful blistering itchy scabby scab\nproperly fully completely still actually now then anyway please thanks\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CPR_HOWTO(py_tuple(vec![py_str("\\bcpr\\b"), py_str("\\bcompressions?\\b"), py_str("\\brescue\\s+breaths?\\b"), py_str("\\bchest\\s+compress\\w*\\b"), py_str("\\bhands[\\s-]?only\\b"), py_str("\\bhow\\s+many\\s+compressions\\b"), py_str("\\bheart\\s+massage\\b"), py_str("\\bp(?:us|u)sh\\w*\\b[^.?!]{0,15}\\bchest\\b"), py_str("\\bchest\\b[^.?!]{0,15}\\bp(?:us|u)sh\\w*\\b"), py_str("\\b(?:how|do|does|should|shall|am|is|what)\\b[^.?!]{0,20}\\bp(?:us|u)sh\\w*\\b[^.?!]{0,12}\\b(?:hard|harder|fast|faster|deep|deeper|many|quickly|fast)\\b"), py_str("\\bhow\\s+(?:hard|harder|fast|deeper|deep|many|quickly|often)\\b[^.?!]{0,20}\\bp(?:us|u)sh\\w*\\b"), py_str("\\bhow\\s+(?:hard|fast|deep)\\b[^.?!]{0,12}\\bcompressions?\\b")]));
    s_CPR_ROOM_EMPTY(py_tuple(vec![py_str("\\bnot\\s+breathing\\b"), py_str("\\bstopped\\s+breathing\\b"), py_str("\\bisn'?t\\s+breathing\\b"), py_str("\\bunconscious\\b"), py_str("\\bcollapsed\\b"), py_str("\\bno\\s+pulse\\b"), py_str("\\bnot\\s+waking\\b"), py_str("\\b(?:gone|turning|going|is|are)\\s+blue\\b"), py_str("\\b(?:is|are)\\s+(?:dead|dying)\\b"), py_str("\\bchoking\\b"), py_str("\\b(?:he|she|they|my\\s+\\w+)\\s+(?:is|are|was)\\s+(?:down|out)\\b")]));
    s_CPR_NOT_EFFORT(py_tuple(vec![py_str("\\b(?:gym|workout|rep|reps|sets?|weights?|lift\\w*|sprint\\w*|run|runs|running|jog\\w*|train\\w*|cycl\\w*|bike|swim\\w*|row\\w*|hike|walk\\w*|stud\\w*|revision|exam|business|project|team|manager|career|pram|pushchair|wheelchair|trolley|barrow|mower|hoover|vacuum|bike|car|van|drill|screw|nail|brush|paint\\w*)\\b"), py_str("\\bp(?:us|u)sh[\\s-]?ups?\\b"), py_str("\\bpress[\\s-]?ups?\\b"), py_str("\\bpull[\\s-]?ups?\\b"), py_str("\\bchin[\\s-]?ups?\\b"), py_str("\\bsit[\\s-]?ups?\\b"), py_str("\\bsquats?\\b"), py_str("\\blunges?\\b"), py_str("\\bplanks?\\b"), py_str("\\bcrunches?\\b"), py_str("\\bburpees?\\b"), py_str("\\bstar\\s+jumps?\\b")]));
    s_CPR_WHEN_ALIVE(py_tuple(vec![py_str("Not if they are breathing or have a pulse - compressions are for someone with neither. Put them in the recovery position, keep watching their breathing and ring 999 or 911 if you are worried; if they stop breathing normally, start CPR: 30 compressions, then 2 rescue breaths."), py_str("Don't do compressions on someone who is breathing or has a pulse: roll them into the recovery position, stay with them and ring 999 or 911. If their breathing stops, it is 30 compressions and 2 rescue breaths, kept going until help arrives.")]));
    s_CPR_NO_PULSE_NOW(py_tuple(vec![py_str("Ring 999 or 911 now and start: no pulse and not breathing normally is CPR - 30 compressions in the centre of the chest, then 2 rescue breaths, kept going until the ambulance takes over."), py_str("No pulse means CPR straight away, so ring 999 or 911 and start: 30 compressions, 2 rescue breaths, about 5-6 cm deep at 100-120 a minute, without stopping until help arrives.")]));
    s_CPR_START(py_tuple(vec![py_str("If they're not breathing normally, start now: 30 hard, fast compressions in the centre of the chest, then 2 rescue breaths, and keep going without stopping until the ambulance arrives."), py_str("Yes - start now if they're not breathing normally, and stay on the line with the 999 or 911 call handler: 30 compressions in the centre of the chest, then 2 rescue breaths, without stopping until help takes over.")]));
    s_CPR_HOW(py_tuple(vec![py_str("Thirty compressions then two rescue breaths, about 5-6 cm deep in the centre of the chest at 100-120 a minute, kept going until the ambulance arrives."), py_str("Push hard and fast in the middle of the chest: 30 compressions, then 2 rescue breaths, about 5-6 cm deep, letting the chest come all the way back up between pushes.")]));
    s_CPR_STOP(py_tuple(vec![py_str("\\b(?:should|do|shall|can|when\\s+(?:should|do|can))\\s+(?:i|we)\\s+stop\\b"), py_str("\\bstop\\s+(?:when|once|as\\s+soon\\s+as)\\b.{0,30}\\b(?:ambulance|paramedics?|crew|help|arrive|arrives|come|comes)\\b"), py_str("\\bhow\\s+long\\s+(?:do|should)\\s+(?:i|we)\\s+(?:keep|carry)\\b"), py_str("\\buntil\\s+when\\b"), py_str("\\bwhen\\s+(?:do|should)\\s+(?:i|we)\\s+(?:stop|give\\s+up)\\b")]));
    s_CPR_STOP_REPLY(py_tuple(vec![py_str("Keep going until the ambulance crew take over from you - they will tell you when to stop. If you are too exhausted to carry on, say so on the call and they will talk you through it."), py_str("Don't stop until the crew take over, even if they seem to come round: keep the compressions going and let the paramedics decide.")]));
    s_CPR_CHILD(py_tuple(vec![py_str("\\b(?:child|children|kid|kids|toddler|toddlers|premature|school\\s?age\\w*)\\b"), py_str("\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|[1-9]|1[0-2])\\s*(?:year|yr)s?\\s*old\\b"), py_str("\\blittle\\s+one\\b"), py_str("\\b(?:son|daughter|boy|girl|grandson|granddaughter|nephew|niece)\\b[^.?!]{0,12}\\b(?:is|aged|of)\\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|[1-9]|1[0-2])\\b")]));
    s_CPR_HOW_BABY(py_tuple(vec![py_str("A baby gets five rescue breaths first - your mouth over their mouth and nose, small puffs - then 15 compressions with two fingers in the middle of the chest, about a third of the depth, and keep going 15 to 2. Ring 999 or 911 now and say it is a baby."), py_str("For a baby: five small rescue breaths, then 15 compressions with two fingers in the centre of the chest, about a third of the depth, then two more breaths - and keep cycling. Ring 999 or 911 now and tell them the baby's age.")]));
    s_CPR_HOW_CHILD(py_tuple(vec![py_str("A child gets five rescue breaths first, then 15 compressions with the heel of one hand (two for a bigger child) in the middle of the chest, about a third of the depth, and keep going 15 to 2. Ring 999 or 911 now."), py_str("For a child: five rescue breaths, then 15 compressions with the heel of one hand in the centre of the chest, about a third of the depth, then two breaths, and keep cycling. Ring 999 or 911 now.")]));
    s_CPR_BABY_SCENE(py_tuple(vec![py_str("That's CPR now for a baby: ring 999 or 911 on speaker, give five small rescue breaths - your mouth over their mouth and nose - then 15 compressions with two fingers in the middle of the chest to about a third of the depth. Keep going 15 to 2 until help takes over."), py_str("Ring 999 or 911 now and start for a baby: five rescue breaths, then 15 compressions with two fingers in the centre of the chest, and keep cycling 15 to 2 until the ambulance arrives.")]));
    s_CPR_CHILD_SCENE(py_tuple(vec![py_str("That's CPR now for a child: ring 999 or 911 on speaker, give five rescue breaths, then 15 compressions with the heel of one hand in the middle of the chest to about a third of the depth, and keep going 15 to 2 until help takes over."), py_str("Ring 999 or 911 now and start: five rescue breaths, then 15 compressions with the heel of one hand, and keep cycling 15 to 2 until the ambulance arrives.")]));
    s_CPR_ROUGH(py_tuple(vec![py_str("Hard and fast is right: about 5-6 cm deep in the centre of the chest. If a rib gives, keep going - cracked ribs heal, and stopping is the bigger danger."), py_str("Deep and fast is what's needed, so don't ease off. If ribs crack, carry on; only stop when help takes over or you are too exhausted to continue.")]));
    s_CALL_HELP_REPLY(py_tuple(vec![py_str("Yes - ring 999 or 911 now if you haven't already, say what you've seen and stay on the line until the call handler tells you what to do."), py_str("If you haven't already, yes: ring 999 or 911 and describe exactly what's happening. Stay on the line - the call handler can talk you through the next step.")]));
    s_ASPIRIN_REPLY(py_tuple(vec![py_str("If they're not allergic to it, give one 300mg aspirin to chew while you wait, and tell the 999 call handler you have. If they're allergic, or the handler says no, don't."), py_str("Chewed aspirin helps if it isn't going to be a problem for them: one 300mg tablet, then say on the call that you've given it. Skip it if they're allergic or the call handler says not to.")]));
    s_WHEN_HELP_COMES(py_tuple(vec![py_str("\\bhow\\s+long\\b.{0,40}\\b(?:ambulance|paramedics?|fire|police|help)\\b"), py_str("\\bwhen\\s+(?:will|are|do)\\b.{0,40}\\b(?:ambulance|paramedics?|fire\\s+(?:brigade|service)|police|they)\\b.{0,20}\\b(?:coming|get|here|arrive)\\b"), py_str("\\bare\\s+they\\s+(?:coming|on\\s+their\\s+way)\\b")]));
    s_WHEN_HELP_REPLY(py_tuple(vec![py_str("They're usually with you within minutes for something like this: keep the door unlocked, the phone on speaker, and let the crew straight in when they knock."), py_str("They are usually there quickly for something like this: leave the door unlocked and the phone on speaker, and the crew will knock and come in.")]));
    s_FITTING_IN(py_tuple(vec![py_str("\\bfitting\\s+(?:in|into|with|up|together)\\b"), py_str("\\ba\\s+fitting\\b"), py_str("\\bfittings?\\s+for\\b(?!\\s+(?:the\\s+)?(?:first|last|second|third|final|an?\\s+\\w+\\s+time|hours?|days?|weeks?|months?|a\\s+while|ages))")]));
    s_FITTING_MANNER_TAIL(stdlib_call("re", "compile", &[py_str("\\s*(?:(?:today|yesterday|lately|recently|nowadays)\\b.*|(?:at|in|into|with)\\s+(?:the\\s+|his\\s+|her\\s+|their\\s+|our\\s+|a\\s+)?(?:new\\s+)?(?:nursery|preschool|playgroup|school|class|clubs?|teams?|groups?|other\\s+kids|the\\s+others)\\b.*)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_FRIGHTENED(py_tuple(vec![py_str("\\b(?:can\\s+you\\s+hear\\s+me|are\\s+you\\s+(?:still\\s+)?there|you\\s+still\\s+there)\\b"), py_str("\\bi'?m\\s+(?:(?:really|so|very|that|proper|properly|absolutely|still|dead|scared\\s+witless)\\s+)*(?:scared|frightened|terrified|panicking|shaking|bricking\\s+it)\\b"), py_str("\\bi\\s+can'?t\\s+stop\\s+shaking\\b"), py_str("\\b(?:going\\s+to|gonna|might|may|could)\\s+lose\\s+(?:him|her|them)\\b"), py_str("\\b(?:he|she|they)\\s+(?:(?:might|may|could)\\s+(?:not\\s+)?(?:make\\s+it|die|survive)|(?:is|are)\\s+going\\s+to\\s+die)\\b"), py_str("\\b(?:will|is)\\s+(?:he|she|they)\\s+(?:going\\s+to\\s+)?be\\s+(?:ok|okay|alright|fine)\\b"), py_str("\\b(?:am|did)\\s+i\\s+do(?:ing)?\\s+(?:it|this)?\\s*right\\b")]));
    s_FRIGHTENED_REPLY(py_tuple(vec![py_str("I'm here. You're doing the right thing by staying with them: keep the 999 or 911 call handler on speaker, say out loud what you can see, and keep talking to them until the ambulance comes."), py_str("That's a frightening thing to be in the middle of. Keep the call handler on speaker, tell the handler everything you notice, and stay with the person until the ambulance arrives.")]));
    s_ABUSE_SCARED(add(&g_FRIGHTENED(), &py_tuple(vec![py_str("\\bi(?:'?m|\\s+am)\\s+(?:(?:really|so|very|that|proper|properly|absolutely|still|dead)\\s+)*(?:scared|frightened|terrified|panicking|shaking|shook|bricking\\s+it)\\b"), py_str("\\bi\\s+keep\\s+(?:shaking|trembling)\\b"), py_str("\\bmy\\s+hands?\\s+(?:keep\\s+|won'?t\\s+stop\\s+|cant\\s+stop\\s+)?shak(?:e|es|ing)\\b"), py_str("\\bi\\s+(?:feel|get)\\s+(?:scared|frightened|terrified)\\b"), py_str("\\b(?:i'?m|i\\s+am)\\s+(?:scared|frightened|terrified)\\s+(?:of|about)\\s+(?:him|her|them)\\b")])));
    s_ABUSE_SCARED_REPLY(py_tuple(vec![py_str("That fear is worth trusting rather than pushing down: get somewhere you can shut a door, keep your phone charged and with you, and ring 999 or 911 if they come back."), py_str("You don't have to sit with that on your own tonight - is there a neighbour or a friend whose door you could knock on? 999 or 911 if they come back, or if it gets worse before then."), py_str("Being frightened is reason enough to get out of the room: phone and keys with you, go to someone you trust, and ring 999 or 911 if they try to stop you leaving."), py_str("That is a frightening way to live, and you are not overreacting. Have a bag by the door, keep somewhere to go in mind tonight, and ring 999 or 911 the moment you feel he might start again.")]));
    s_DETAIL_AFTER_ESCALATION(py_tuple(vec![py_str("\\b(?:it|this|he|she|they)\\s+(?:started|began|happened|came\\s+on|kicked\\s+off)\\b"), py_str("\\b(?:this\\s+morning|this\\s+afternoon|this\\s+evening|earlier\\s+today|yesterday|last\\s+night|an\\s+hour\\s+ago|\\d+\\s+(?:minutes?|hours?|days?)\\s+ago)\\b"), py_str("\\bsince\\s+(?:this\\s+morning|yesterday|last\\s+night|about\\s+\\d)\\b")]));
    s_HELD_CRISIS_DETAIL(add(&g_CRISIS_PLAN_SAID(), &py_tuple(vec![py_str("\\b(?:have|has|got|made|make|worked\\s+out|thought\\s+of|thought\\s+about|picked|chosen)\\s+(?:a|the|my|one)\\s+plan\\b"), py_str("\\bthe\\s+plan\\s+is\\s+(?:ready|set|made|done)\\b"), py_str("\\b(?:wrote|written|writing)\\s+(?:a|the|my)\\s+(?:notes?|letters?)\\b"), py_str("\\bi'?ve\\s+(?:written\\s+it|got\\s+it)\\s+down\\b"), py_str("\\b(?:got|have|bought|saved|sorted)\\s+(?:the|a|some)\\s+(?:pills|tablets|meds|medicine|painkillers|rope|noose|razor\\w*|blades?|gun|knife|paracetamol)\\b"), py_str("\\b(?:the\\s+)?(?:pills|tablets|rope|noose|razors?|blades?)\\s+(?:are|is)\\s+(?:ready|sorted|nearby|in\\s+my\\s+\\w+)\\b"), py_str("\\b(?:picked|chosen|set|know|got)\\s+(?:a|the|my)\\s+(?:date|day|time)\\b"), py_str("\\bi\\s+know\\s+(?:when|where|how\\s+much)\\b"), py_str("\\bi\\s+know\\s+which\\s+(?:place|spot|bridge|building|roof|window|tree|way\\s+out|road)\\b"), py_str("\\bhow\\s+long\\s+(?:till|until|before)\\s+(?:that|then|it)\\b"), py_str("\\bhow\\s+long\\s+(?:do|have|would)\\s+i\\s+(?:have|got)\\b"), py_str("\\bwhen\\s+(?:everyone'?s|everybody'?s|they'?re)\\s+(?:asleep|gone|out|away)\\b"), py_str("\\bi'?m\\s+(?:going|gonna)\\s+to\\s+(?:do\\s+it|go\\s+through\\s+with\\s+it)\\b"), py_str("\\bi'?ll\\s+(?:do\\s+it|go\\s+through\\s+with\\s+it)\\b"), py_str("\\b(?:my|a)\\s+(?:sister|brother|friend|mum|mom|dad|mother|father|partner|wife|husband|neighbour|neighbor|daughter|son|cousin|mate|flatmate|housemate|carer)\\b[^.?!]{0,40}\\b(?:coming|comes|come|coming\\s+over|on\\s+(?:her|his|their)\\s+way|is\\s+with\\s+me|here\\s+(?:tonight|now|with\\s+me)|staying|made\\s+it|got\\s+here)\\b"), py_str("\\b(?:im|i'?m)\\s+not\\s+on\\s+my\\s+own\\b"), py_str("\\b(?:someone|somebody)\\s+is\\s+with\\s+me\\b"), py_str("\\b(?:put|placed|hid|hidden|hide|kept|left|stashed)\\b[^.?!]{0,24}\\b(?:notes?|letters?|razors?|blades?|rope|noose|gun)\\b[^.?!]{0,20}\\b(?:in|on|under|behind|inside)\\s+(?:my|the|a)\\b"), py_str("\\b(?:pills|tablets|meds|bottle)\\b[^.?!]{0,18}\\b(?:in|on|under|behind|inside)\\s+(?:my|the)\\s+(?:pocket|bag|handbag|rucksack|coat|jacket|sock|shoe|locker|desk|bedside|drawer)\\b"), py_str("\\b(?:hid|hidden|stashed)\\b[^.?!]{0,20}\\b(?:notes?|letters?|razors?|blades?|rope|noose)\\b"), py_str("\\b(?:moved|shifted|took|taken|carried|brought)\\b[^.?!]{0,20}\\b(?:notes?|letters?|tablets?|pills?|razors?|blades?|rope|noose|gun)\\b"), py_str("\\b(?:notes?|letters?|tablets|pills|razors?|blades?|rope|noose|gun)\\b[^.?!]{0,14}\\b(?:is|are)\\s+(?:already\\s+)?(?:in|on|under|behind|inside)\\s+(?:my|the|a)\\b"), py_str("\\b(?:filled|packed|loaded|measured|counted|lined\\s+up)\\s+(?:the|a|my)\\s+(?:bottle|glass|syringe|needle|bag|box|tub|container)\\b"), py_str("\\b(?:have|got|bought|sorted|saved)\\s+(?:the|all\\s+the)\\s+stuff\\b"), py_str("\\b(?:bought|got|have|sorted)\\s+(?:everything|what)\\s+i\\s+need\\b")])));
    s_HELD_CRISIS_OFF(py_tuple(vec![py_str("\\bsafety\\s+plan\\b"), py_str("\\bplan\\s+(?:with|from)\\s+(?:my\\s+)?(?:therapist|counsell?or|doctor|gp|nurse|psychiatrist|social\\s+worker|support\\s+worker|cpn|key\\s+worker)\\b"), py_str("\\bplan\\s+to\\s+(?:ring|call|see|speak|talk|get\\s+help|ask\\s+for\\s+help)\\b"), py_str("\\bplan\\s+for\\s+(?:dinner|tea|lunch|breakfast|christmas|tonight'?s|the\\s+weekend|tomorrow'?s|the\\s+party|the\\s+trip|the\\s+holiday|work)\\b"), py_str("\\bcrisis\\s+line\\b"), py_str("\\bsamaritans\\b"), py_str("\\b116\\s*123\\b"), py_str("\\b988\\b"), py_str("\\b(?:im|i'?m|i\\s+am)\\s+(?:ok|okay|fine|safe|better|alright|calmer)\\b"), py_str("\\b(?:feel|feeling)\\s+(?:better|calmer|safer|a\\s+bit\\s+better)\\b"), py_str("\\bnot\\s+(?:going\\s+to|gonna)\\s+(?:do\\s+it|do\\s+anything)\\b"), py_str("\\b(?:i\\s+)?(?:will|'ll|ll)\\s+(?:ring|call|see\\s+someone|get\\s+help)\\b"), py_str("\\b(?:rang|called|texted)\\s+(?:them|the\\s+line|someone|my\\s+\\w+)\\b"), py_str("\\bthank(?:s|\\s*you)\\b"), py_str("\\b(?:note|letter)\\s+(?:to|for)\\s+(?:my|his|her|their|the|a|an)\\b"), py_str("\\b(?:don'?t|do\\s+not|havent|haven'?t|not|no\\s+longer)\\s+(?:have\\s+)?(?:a|any|the)\\s+plan\\b"), py_str("\\bno\\s+plan\\b")]));
    s_CRISIS_PLAN_CARRY(py_tuple(vec![py_str("\\b(?:wrote|writing|written|left|leaving|leave)\\s+(?:a|the|my)\\s+(?:note|letter)\\b"), py_str("\\bthe\\s+notes?\\s+(?:is|are)\\s+(?:in|on|under|by|at|next\\s+to|inside)\\b"), py_str("\\b(?:picked|pick|chosen|chose|set|setting|decided\\s+on)\\s+(?:a|the|my)?\\s*(?:date|day)\\b"), py_str("\\b(?:got|get|bought|buy|have|having|stockpiled|stockpiling|saved\\s+up|saving\\s+up|ordered|ordering|hid|hiding)\\s+(?:the|a|some|enough|loads\\s+of|up)?\\s*(?:pills|tablets|paracetamol|painkillers|rope|noose|razors?|blades?|knife|gun)\\b"), py_str("\\bi(?:'ve|ve)?\\s+(?:have|got)\\s+a\\s+plan\\b"), py_str("\\bi\\s+know\\s+how\\s+(?:i'?m|im|i\\s+am|i'?d|id)\\s+(?:going\\s+to|gonna|to|would)\\s+(?:do\\s+it|end\\s+it|kill\\s+myself|finish\\s+it)\\b"), py_str("\\b(?:said|saying|writing)\\s+(?:my\\s+)?goodbyes?\\b"), py_str("\\b(?:picked|found|chosen)\\s+(?:a|the)\\s+(?:place|spot|bridge|roof|car\\s+park)\\b"), py_str("\\bnot\\s+going\\s+to\\s+be\\s+(?:here|around)\\s+(?:much\\s+)?(?:longer|after\\s+this)\\b"), py_str("\\bi\\s+won'?t\\s+be\\s+(?:here|around)\\s+(?:for\\s+)?(?:next\\s+\\w+|much\\s+longer|christmas|the\\s+weekend|long)\\b"), py_str("\\b(?:i'?m|im|i\\s+am|i\\s+will|ill|i'?ll)\\s+(?:going\\s+to\\s+|gonna\\s+)?(?:do\\s+it|end\\s+it|finish\\s+it|go\\s+through\\s+with\\s+it|kill\\s+my\\s*self)\\b"), py_str("\\b(?:just\\s+)?need\\s+(?:the|some|a\\s+bit\\s+of|more)\\s+nerve\\b(?![\\s\\S]*\\b(?:ring|call|phone|contact|book|tell|text|see\\s+(?:the|my|a)|gp|doctor|nurse|samaritans|111|999|a\\s?&\\s?e|hospital|therapist|counsell?or)\\b)"), py_str("\\b(?:got|bought|have|have\\s+got|found)\\s+(?:the|all\\s+the)\\s*(?:stuff|things|kit|equipment)\\b(?=\\s*(?:$|[.,!?]|ready\\b|now\\b|today\\b|tonight\\b))")]));
    s_CRISIS_PLAN_ORDINARY(py_tuple(vec![py_str("\\b(?:thank\\s*you|thankyou|birthday|christmas|sick|absence|shopping|grocery|to[\\s-]do|sticky|reminder|credit|debit|bank)\\b[^.?!]{0,14}\\bnotes?\\b"), py_str("\\bnotes?\\b[^.?!]{0,24}\\b(?:about\\s+the|for\\s+the|for\\s+my\\s+(?:work|boss|school|doctor|landlord|mum|dad|teacher|team|rent|car|house|flat)|re:|saying\\s+thank)\\b"), py_str("\\bnotes?\\s+(?:app|on\\s+my\\s+phone|in\\s+my\\s+phone)\\b"), py_str("\\bletters?\\b[^.?!]{0,26}\\b(?:to\\s+the\\s+(?:council|bank|landlord|school|doctor|hospital|company|insurer|insurance|dvla|hmrc|mp|jobcentre|university|college)|about\\s+the\\s+(?:rent|bill|debt|job|course|house|flat))\\b"), py_str("\\b(?:date|day)\\b[^.?!]{0,18}\\b(?:for\\s+(?:the\\s+)?(?:wedding|holiday|exam|interview|dentist|doctor|scan|move|party|trip|operation|op|viewing)|of\\s+the\\s+(?:wedding|exam))\\b"), py_str("\\b(?:my|the|his|her)\\s+(?:prescription|antibiotics|contraceptive|blood\\s+pressure|diabetes|epilepsy|thyroid|thyroxine|metformin|statins?)\\b"), py_str("\\b(?:pills|tablets|painkillers|paracetamol)\\b[^.?!]{0,20}\\b(?:from\\s+the|under\\s+prescription|prescribed|for\\s+my\\s+(?:back|knee|period|headache|migraine|arthritis|pain|hip|shoulder|neck|teeth|tooth))\\b"), py_str("\\bstuff\\b[^.?!]{0,20}\\bfor\\s+the\\s+(?:cake|party|trip|dinner|recipe|barbecue|bbq|birthday|wedding|christmas|exam|interview|move|house|flat|garden|bathroom|kitchen|car)\\b"), py_str("\\bi\\s+know\\s+how\\s+much\\b[^.?!]{0,24}\\b(?:costs?|cost\\s+me|pays?|weighs?|i\\s+owe|it\\s+is|it's)\\b"), py_str("\\bmeasured\\b[^.?!]{0,24}\\b(?:milk|water|flour|sugar|rice|ingredients?|grams|ml|ounces|paint|timber|wood)\\b"), py_str("\\b(?:rope|knife|blade|blades)\\b[^.?!]{0,24}\\b(?:for\\s+the\\s+(?:car|trailer|garden|washing|bikes?|roof\\s+rack|kitchen|cooking|dinner|bread|onions?|swings?|clothes\\s+line|washing\\s+line|boat|trampoline|tree\\s+house|shed|gate|fence)|to\\s+(?:tie|tow|cut|trim|slash|open|prune|chop|slice|fillet|carve|peel))\\b")]));
    s_CRISIS_PLAN_REPLY(py_tuple(vec![py_str("Thank you for telling me. That is a plan, and a plan needs help now rather than later: ring 999 or 911, or get yourself to A&E and say what you have ready - and please do not stay on your own: put the tablets or the note out of reach and ask someone to come and sit with you."), py_str("That is a plan, so please do not carry it on your own: ring 999 or 911, or go to A&E and say what you have got ready, and have someone come to you and take the means away. Samaritans are on 116 123 any hour if you need a voice before that.")]));
    s_WAKE_UP_SOMEONE(py_tuple(vec![add(&add(&py_str("\\b(?:can'?t|cant|cannot|can\\s+not|unable\\s+to|struggling\\s+to|having\\s+trouble)\\b[^.?!]{0,24}?\\b(?:wake|waken|waking|rouse|wake\\s+up)\\s+(?:my|the|our|his|her|their)\\s+"), &g_PERSON()), &py_str("\\b(?![^.?!]{0,12}\\b(?:for|to|before|in\\s+time\\s+for)\\s+(?:(?:his|her|their|the|a)\\s+)?(?:school|work|college|uni|university|nursery|childminder|shift|the\\s+(?:bus|train))\\b)")), add(&add(&py_str("\\b(?:can'?t|cant|cannot|unable\\s+to|struggling\\s+to|having\\s+trouble)\\b[^.?!]{0,20}?\\b(?:get|keep)\\s+(?:my|the|our|his|her|their)\\s+"), &g_PERSON()), &py_str("\\b[^.?!]{0,15}?\\b(?:wake|waken|rouse|stir)\\b"))]));
    s_ROUTINE_REASON(py_tuple(vec![py_str("\\b(?:school|work|college|uni|university|nursery|childminder|shifts?|alarms?|bus|train|flight|exams?|test|interview|appointment|meeting|class|lecture|lesson|paper\\s+round|paper\\s+deliveries|early\\s+start|mornings?)\\b")]));
    s_SPELL_RED(py_tuple(vec![py_str("\\b(?:temperatur\\w+|fever\\w*|breath\\w*|blue|grey|gray|limp|floppy|floppiness|seizure|fit|shaking|rash|stiff\\s+neck|dribbling|drooling|unresponsive)\\b")]));
    s_MORNING_ROUTINE(py_tuple(vec![py_str("\\b(?:in|of)\\s+the\\s+mornings?\\b|\\bmornings\\b|\\bbefore\\s+(?:school|work|nursery|breakfast)\\b|\\bon\\s+school\\s+days\\b")]));
    s_HARD_TO_WAKE(py_tuple(vec![py_str("\\b(?:can'?t|cannot|can\\s+not)\\s+be\\s+(?:woken|roused|wakened|stirred)\\b"), py_str("\\b(?:very|so|really|extremely|quite)?\\s*(?:sleepy|drowsy|lethargic|floppy|limp)\\b[^.?!]{0,25}\\b(?:hard|difficult|impossible|not\\s+easy)\\s+to\\s+(?:wake|rouse|waken|stir)\\b"), py_str("\\b(?:hard|difficult|impossible|not\\s+easy)\\s+to\\s+(?:wake|rouse|waken|stir)\\b[^.?!]{0,25}\\b(?:very\\s+|so\\s+)?(?:sleepy|drowsy|floppy|limp|lethargic|not\\s+(?:himself|herself|themselves))\\b"), py_str("\\b(?:baby|babies|newborn|infant)\\b[^.?!]{0,25}\\b(?:hard|difficult|impossible)\\s+to\\s+(?:wake|rouse|waken|stir)\\b"), py_str("\\b(?:hard|difficult|impossible)\\s+to\\s+(?:wake|rouse|waken|stir)\\b[^.?!]{0,25}\\b(?:baby|newborn|infant)\\b"), add(&add(&add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_PERSON()), &py_str("|the\\s+")), &g_PERSON()), &py_str(")\\b[^.?!]{0,15}\\b(?:is|was|seems?|seemed|has\\s+been|had\\s+been)\\b[^.?!]{0,8}\\b(?:hard|difficult|impossible)\\s+to\\s+(?:rouse|waken|stir)\\b"))]));
    s_VISIT_OR_TIME(py_tuple(vec![py_str("\\b(?:tonight|later|tomorrow|this\\s+(?:evening|afternoon|morning|weekend)|after\\s+all|next\\s+(?:week|weekend)|until\\s+\\w+|for\\s+(?:dinner|tea|coffee|lunch|supper|a\\s+bit))\\b")]));
    s_COLLAPSE_HINT(py_tuple(vec![py_str("\\b(?:fell|fallen|fall|bang|banged|hit|head|collapsed|floor|ground|pavement|breathing|ambulance|pulse|overdose|swallowed|diabetic|seizure|the\\s+water|pool)\\b")]));
    s_WONT_COME_ROUND(py_tuple(vec![add(&add(&add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_PERSON()), &py_str("|the\\s+")), &g_PERSON()), &py_str(")\\b[^.?!]{0,12}\\b(?:won'?t|wont|isn'?t|isnt|is\\s+not|not|hasn'?t|hasnt|didn'?t|didnt|can'?t|cant|cannot)\\b[^.?!]{0,10}\\b(?:(?:com(?:e|ing))\\s+round\\b(?!\\s*(?:for|to)\\s+(?:dinner|tea|coffee|lunch|supper|visit|see|pick|my|his|her|their|our|the\\s+(?:idea|way|view|opinion|plan)))|com(?:e|ing)\\s+to\\b(?!\\s+(?:the|a|an|my|your|his|her|their|our|this|that|it|us|me|him|them)\\b)|rous(?:e|es|ing|ed|able)\\b)")), add(&add(&add(&add(&py_str("\\b(?:can'?t|cant|cannot|unable\\s+to|struggling\\s+to|not\\s+able\\s+to)\\b[^.?!]{0,15}\\b(?:bring|get|rouse|wake)\\s+(?:him|her|them|my\\s+"), &g_PERSON()), &py_str("|the\\s+")), &g_PERSON()), &py_str(")\\b(?:[^.?!]{0,15}\\bround\\b(?!\\s+(?:for|to)\\s+(?:dinner|tea|coffee|lunch|supper|see|my|his|her|their|our|the\\s+(?:idea|way|view|opinion)))|[^.?!]{0,6}\\bawake\\b|[^.?!]{0,8}\\bto\\s+(?:wake|waken|stir|rouse)\\b|[^.?!]{0,6}\\bto\\b(?=\\s*[,.!?]|\\s*$))"))]));
    s__STREAK_MARK(py_tuple(vec![py_str("\\bred\\s+(?:line|streak|track)s?\\b"), py_str("\\breddish\\s+(?:line|streak)s?\\b")]));
    s__STREAK_REDNESS(py_str("\\b(?:redness|red\\s+skin|red\\s+area|red\\s+patch|the\\s+red\\s+part|infection)\\b"));
    s__STREAK_MOVE(py_tuple(vec![py_str("\\b(?:spreading|spread|creeping|crept|tracking|tracked|travell?ing|travell?ed|moving|moved|running|ran|going|gone|goes|up|down)\\b")]));
    s__STREAK_ON_A_PERSON(py_tuple(vec![py_str("\\b(?:cut|cuts|cuts?\\s+myself|gash|graze[ds]?|scrape[ds]?|scratch(?:es|ed)?|wound|wounds|sore|splinter|thorn|blister|bite|bites|bitten|burn(?:s|ed|t)?|sting(?:s|ed)?|puncture|operation|opsite|stitches?|staple[ds]?|dressing|bandage[ds]?|plaster|tattoo)\\b")]));
    s_WOUND_STREAK(py_tuple(vec![add(&add(&add(&add(&add(&add(&py_str("\\A(?=.*"), &geti(&g__STREAK_MARK(), &Py::Int(0))), &py_str(")(?=.*")), &geti(&g__STREAK_ON_A_PERSON(), &Py::Int(0))), &py_str("|.*\\b")), &crate::gen::modules::safety_cards::g_BODY()), &py_str(")")), add(&add(&add(&add(&add(&add(&py_str("\\A(?=.*"), &geti(&g__STREAK_MARK(), &Py::Int(1))), &py_str(")(?=.*")), &geti(&g__STREAK_ON_A_PERSON(), &Py::Int(0))), &py_str("|.*\\b")), &crate::gen::modules::safety_cards::g_BODY()), &py_str(")")), add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\A(?=.*"), &g__STREAK_REDNESS()), &py_str("[^.?!]{0,30}")), &geti(&g__STREAK_MOVE(), &Py::Int(0))), &py_str(")(?=.*")), &geti(&g__STREAK_ON_A_PERSON(), &Py::Int(0))), &py_str("|.*\\b")), &crate::gen::modules::safety_cards::g_BODY()), &py_str(")")), add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\A(?=.*"), &geti(&g__STREAK_MOVE(), &Py::Int(0))), &py_str("[^.?!]{0,30}")), &g__STREAK_REDNESS()), &py_str(")(?=.*")), &geti(&g__STREAK_ON_A_PERSON(), &Py::Int(0))), &py_str("|.*\\b")), &crate::gen::modules::safety_cards::g_BODY()), &py_str(")"))]));
    s_WOUND_STREAK_OFF(py_tuple(vec![py_str("\\bhair\\b"), py_str("\\bdye\\w*\\b"), py_str("\\bhighlight\\w*\\b"), py_str("\\bnail\\s+polish\\b"), py_str("\\blipstick\\b"), py_str("\\btrain\\b"), py_str("\\btube\\b"), py_str("\\bmetro\\b"), py_str("\\bbus\\b"), py_str("\\broad\\b"), py_str("\\blane\\b"), py_str("\\bmap\\b"), py_str("\\bsky\\b"), py_str("\\bsunset\\b"), py_str("\\brainbow\\b"), py_str("\\bpaint\\w*\\b"), py_str("\\bmarker\\b"), py_str("\\bpen\\b"), py_str("\\bscreen\\b"), py_str("\\btv\\b"), py_str("\\btele\\b"), py_str("\\bfilm\\b"), py_str("\\bbook\\b"), py_str("\\bpaper\\b"), py_str("\\bflag\\b"), py_str("\\bteam\\b"), py_str("\\bspider\\s+vein\\w*\\b"), py_str("\\bstretch\\s+mark\\w*\\b"), py_str("\\bnappy\\b"), py_str("\\bnapkin\\b"), py_str("\\bcrease\\w*\\b"), py_str("\\btyre\\w*\\b"), py_str("\\bcar\\b"), py_str("\\bwall\\b"), py_str("\\bfloor\\b"), py_str("\\bkitchen\\b")]));
    s_STREAK_FEVER(py_tuple(vec![py_str("\\bfever\\w*\\b"), py_str("\\btemperatures?\\b"), py_str("\\btemps?\\b"), py_str("\\bshiver\\w*\\b"), py_str("\\brigors\\b"), py_str("\\bclammy\\b"), py_str("\\bvery\\s+unwell\\b"), py_str("\\bunwell\\b"), py_str("\\bfluey\\b"), py_str("\\bhot\\b"), py_str("\\b\\d{2}(?:\\.\\d)?\\s*(?:c\\b|degrees)\\b")]));
    s_WOUND_STREAK_FEVER(_with_streak_fever(&[g_WOUND_STREAK()]));
    s__A_PURPOSE_AFTER(py_str("(?!\\s*,?\\s*(?:to|and|so|then)\\s+(?:fix|get|catch|grab|fetch|collect|check|clean|clear|put|take|pick|mend|paint|water|feed|empty|see|look|have|find|hang|drop|pass|give|bring|do)\\b)"));
    s__OFF_A_HEIGHT(add(&py_str("off(?:\\s+of)?\\s+(?:a|an|the|my|our)?\\s*(?:bridge|roof|building|block|cliff|balcony|ledge|height|car\\s*park)\\b"), &g__A_PURPOSE_AFTER()));
    s__OFF_AN_EDGE(add(&py_str("off(?:\\s+of)?\\s+(?:a|an|the|my|our)?\\s*(?:platform|pier|wall|bank|edge)\\b"), &g__A_PURPOSE_AFTER()));
    s__INTO_THE_WAY(add(&py_str("(?:in\\s+front\\s+of|under|onto)\\s+(?:a|an|the|my|our)?\\s*(?:train|trains|bus|buses|lorry|truck|coach|traffic|tracks?|rails?|line)\\b"), &g__A_PURPOSE_AFTER()));
    s_SELF_HARM_WISHES_A_PART(py_tuple(vec![py_str("\\b(?:want|wanna|wanted|wanting|need|needing|going|gonna|plan|planning|tempted|urge|feel\\s+like|feeling\\s+like|think\\s+about|thinking\\s+about|thought\\s+about)\\s+(?:to\\s+)?(?:cut|cutting|burn|burning|hurt|hurting|harm|harming|scratch|scratching|scald|scalding|starve|starving|bruise|bruising)\\s+my\\s+(?:own\\s+)?(?:skin|arms?|legs?|wrists?|thighs?|forearms?|belly|stomach|hips?|chest|hands?|feet|foot|scalp|neck|shoulders?|back|face)\\b(?!\\s+(?:hair|nails?|toenails?|fingernails?|fat|calories?|carbs?)\\b)")]));
    s__HEIGHT_MEANS_VERB(py_str("throw|throwing|threw|jump|jumping|jumped|leap|leaping|leapt|fall|falling|fell"));
    s__ANY_MEANS_VERB(add(&g__HEIGHT_MEANS_VERB(), &py_str("|step|stepping|stepped|walk|walking|walked|lie|lying|lay")));
    s__A_MEANS_PLACE(add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&py_str("(?:(?:"), &g__ANY_MEANS_VERB()), &py_str(")\\s+")), &g__INTO_THE_WAY()), &py_str("|(?:")), &g__HEIGHT_MEANS_VERB()), &py_str(")\\s+(?:")), &g__OFF_A_HEIGHT()), &py_str("|")), &g__OFF_AN_EDGE()), &py_str("))")));
    s_SELF_HARM_BY_A_MEANS(py_tuple(vec![add(&add(&add(&py_str("\\b(?:"), &g__ANY_MEANS_VERB()), &py_str(")\\s+(?:my\\s*self|me)\\s*")), &g__INTO_THE_WAY()), add(&add(&add(&add(&add(&add(&py_str("\\b(?:"), &g__HEIGHT_MEANS_VERB()), &py_str(")\\s+(?:my\\s*self|me)\\s*(?:")), &g__OFF_A_HEIGHT()), &py_str("|")), &g__OFF_AN_EDGE()), &py_str(")")), py_str("\\b(?:hang|hanging|strangle|shoot|shooting|stab|poison|overdose|overdosing)\\s+(?:my\\s*self|me)\\b"), py_str("\\b(?:with|using|by)\\s+(?:a\\s+|the\\s+)?(?:rope|knife|blade|razor|gun|pills|tablets|paracetamol|belt)\\b[^.?!]{0,24}\\b(?:kill|hurt|harm|cut|hang|shoot)\\s+my\\s*self\\b"), add(&py_str("\\b(?:i\\s+want|i\\s+wanna|im|i'?m|i\\s+am|i\\s+could|i\\s+would|i\\s+might|i\\s+should|im\\s+going|i'?m\\s+going|going|thinking\\s+about|thought\\s+about|tempted|planning)\\b[^.?!]{0,30}\\b"), &g__A_MEANS_PLACE()), add(&py_str("^\\s*(?:so\\s+|and\\s+|then\\s+)?"), &g__A_MEANS_PLACE())]));
    s_POISON_SAID(py_tuple(vec![py_str("\\b(?:swallowed|drank|drunk|eaten|ate|took|downed|inhaled|gulped|licked)\\s+(?:some\\s+|a\\s+bit\\s+of\\s+|a\\s+mouthful\\s+of\\s+|loads\\s+of\\s+|the\\s+rest\\s+of\\s+|half\\s+a\\s+bottle\\s+of\\s+|a\\s+whole\\s+bottle\\s+of\\s+)?(?:bleach|detergent|disinfectant|antifreeze|weed\\s?killer|hand\\s+saniti[sz]er|drain\\s+cleaner|soap|shampoo|mouthwash|shower\\s+gel|cleaning\\s+products?|paint\\s?thinner|white\\s+spirit)\\b"), py_str("\\b(?:double|triple)\\s+dos(?:ed|ing)\\b"), py_str("\\b(?:doubl|tripl)(?:ed|ing)\\s+(?:up\\s+)?(?:on\\s+)?(?:my|the)\\s+(?:dose|insulin|meds|medication|tablets|pills)\\b")]));
    s_A_HURT_OF_A_BODY_PART(py_str("\\b(?:hurt|hurting|harm|harming|work|working|push|pushing)\\s+my\\s+(?:own\\s+)?(?:arms?|legs?|shoulders?|muscles?|back|abs|core|chest|glutes?|calves)\\b"));
    s_A_WORKOUT_CUE(py_str("\\b(?:gym|work\\s?outs?|weights?|weight\\s?lifting|lifting|dumbbells?|barbells?|kettlebells?|treadmill|rowing|spin(?:ning)?\\s+class|physio\\w*|training\\s+session|leg\\s+day|arm\\s+day|pt\\s+session)\\b"));
    s_A_PLAN_SAID(py_tuple(vec![py_str("\\bi\\s+know\\s+how\\s+(?:i'?d|id|i\\s+would)\\s+(?:do\\s+it|end\\s+it|be\\s+done|go\\b)"), py_str("\\bi\\s+know\\s+how\\s+(?:i'?m|im|i\\s+am)\\s+(?:going\\s+to|gonna)\\s+(?:do\\s+it|end\\s+it|be\\s+done)\\b"), py_str("\\b(?:my|the|our)\\s+plan\\s*(?:is|'s|was)\\s+to\\s+(?:jump|cut|hang|overdose|swallow|take|use|go)\\b(?:\\s+(?:it|them|all\\s+of\\s+them|them\\s+all|the\\s+lot|too\\s+many|an?\\s+overdose|my\\s+life|my\\s*self|off\\s+(?:a|the)\\s+(?:bridge|roof|building|cliff|balcony)|in\\s+front\\s+of\\s+(?:a|the)\\s+(?:train|bus)|my\\s+(?:wrists?|throat|arms?|legs?|neck)))?(?:\\s+(?:all\\s+at\\s+once|at\\s+once|right\\s+now|tonight|today|now))?\\s*[.?!]*\\s*$"), py_str("\\bi\\s+(?:have|got|'ve\\s+got|ve\\s+got)\\s+a\\s+plan\\s+to\\s+(?:do\\s+it|end\\s+it|end\\s+my\\s+life|kill\\s+my\\s*self|take\\s+(?:all\\s+of\\s+them|them\\s+all|the\\s+lot)|jump|overdose)\\b"), py_str("\\b(?:im|i'?m|i\\s+am)\\s+going\\s+to\\s+do\\s+it\\s*[.?!]*$"), py_str("^\\s*(?:so\\s+)?(?:i'?m\\s+)?going\\s+to\\s+(?:take|swallow)\\s+(?:all\\s+of\\s+them|them\\s+all|the\\s+(?:whole\\s+)?packet|the\\s+lot)\\s*[.?!]*\\s*$")]));
    s_A_PLAN_ORDINARY(py_tuple(vec![py_str("\\bplan\\s+(?:is|'s|was)\\s+to\\s+(?:pay|save|book|call|ring|text|email|write|start|finish|order|cancel|visit|see|meet|apply|clean|sort|tidy|walk|run|train|study|revise|move|paint|cook|buy|sell|fly|drive|go\\s+(?:to|back|home|out)|get\\s+(?:the|a|my|some)|have\\s+(?:a|some|the|lunch|dinner|breakfast))\\b"), py_str("\\bgoing\\s+to\\s+do\\s+it\\s+(?:after|before|on|at|in|next|this|tomorrow|tonight|later|when|once|every|twice|again|three|two|with|for|if|and)\\b"), py_str("\\bwe\\s+(?:were|are)\\s+going\\s+to\\s+do\\s+it\\b"), py_str("\\bjump\\s*-?\\s*start\\b|\\bjump\\s+(?:the\\s+queue|the\\s+gun|start\\s+the\\s+car)\\b"), py_str("\\bknow\\s+how\\s+(?:i'?d|id)\\s+do\\s+it\\s+(?:now|too|myself|already)\\b"), py_str("\\bknow\\s+how\\s+(?:i'?d|id|i\\s+would)\\s+(?:do\\s+it|end\\s+it)\\b\\s*(?:[-\\u2013,:;]\\s*)?(?:you|u|it'?s|its|then|just|by|with|using|through|via)\\b"), py_str("\\b(?:do\\s+it|end\\s+it)\\s+(?:if|when|once|with|by|using|through|via|for|on|at|in)\\b")]));
    s_SELF_HARM_OWN_PART(py_tuple(vec![g_SELF_HARM_WISHES_A_PART(), g_SELF_HARM_BY_A_MEANS()]));
}
pub fn _with_streak_fever(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pats: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pats") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pats"))) };
    let mut extra: Py = Py::None;
    let mut p: Py = Py::None;
    let _ = py_str("The same streak shapes, read only when the line also has a fever in it.");
    extra = add(&add(&py_str("(?=[\\s\\S]*(?:"), &mcall(&py_str("|"), "join", &[g_STREAK_FEVER()])), &py_str("))"));
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pats.clone()) { let mut p: Py = __item.clone(); __out.push(add(&p.clone(), &extra.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.safety_tables".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_SPEAKER" => Some(g_ABOUT_THE_SPEAKER()),
            "ABUSE_BENIGN_REASON" => Some(g_ABUSE_BENIGN_REASON()),
            "ABUSE_BLOW" => Some(g_ABUSE_BLOW()),
            "ABUSE_BUTTONHOLED" => Some(g_ABUSE_BUTTONHOLED()),
            "ABUSE_CHILD_REFUSAL" => Some(g_ABUSE_CHILD_REFUSAL()),
            "ABUSE_EMPLOYER_REFUSAL" => Some(g_ABUSE_EMPLOYER_REFUSAL()),
            "ABUSE_FINE_ANIMAL" => Some(g_ABUSE_FINE_ANIMAL()),
            "ABUSE_FINE_PLACE" => Some(g_ABUSE_FINE_PLACE()),
            "ABUSE_HANDS_ON" => Some(g_ABUSE_HANDS_ON()),
            "ABUSE_KIDS_AT_HOME" => Some(g_ABUSE_KIDS_AT_HOME()),
            "ABUSE_KIDS_CARD" => Some(g_ABUSE_KIDS_CARD()),
            "ABUSE_PARENT_REFUSAL" => Some(g_ABUSE_PARENT_REFUSAL()),
            "ABUSE_PARENT_REFUSAL_HARD" => Some(g_ABUSE_PARENT_REFUSAL_HARD()),
            "ABUSE_PLAY" => Some(g_ABUSE_PLAY()),
            "ABUSE_PLAY_HARD" => Some(g_ABUSE_PLAY_HARD()),
            "ABUSE_QUARREL" => Some(g_ABUSE_QUARREL()),
            "ABUSE_ROOM_CARD" => Some(g_ABUSE_ROOM_CARD()),
            "ABUSE_ROOM_FAMILY" => Some(g_ABUSE_ROOM_FAMILY()),
            "ABUSE_SCARED" => Some(g_ABUSE_SCARED()),
            "ABUSE_SCARED_REPLY" => Some(g_ABUSE_SCARED_REPLY()),
            "ACCIDENT" => Some(g_ACCIDENT()),
            "ADULT_HIT" => Some(g_ADULT_HIT()),
            "AFTER_EXERCISE" => Some(g_AFTER_EXERCISE()),
            "AGE_PERSON" => Some(g_AGE_PERSON()),
            "ALARM_NUISANCE" => Some(g_ALARM_NUISANCE()),
            "AMBULANCE_CASES" => Some(g_AMBULANCE_CASES()),
            "ANIMAL_KICK" => Some(g_ANIMAL_KICK()),
            "ANY_REFUSAL_TO_GO" => Some(g_ANY_REFUSAL_TO_GO()),
            "AN_ITCH_LOOKALIKE" => Some(g_AN_ITCH_LOOKALIKE()),
            "APPLIANCE_SMOKE" => Some(g_APPLIANCE_SMOKE()),
            "ASKS_ABOUT_THE_INJURY" => Some(g_ASKS_ABOUT_THE_INJURY()),
            "ASKS_GO_BARE" => Some(g_ASKS_GO_BARE()),
            "ASKS_IF_CALL" => Some(g_ASKS_IF_CALL()),
            "ASKS_IF_GOING" => Some(g_ASKS_IF_GOING()),
            "ASK_CALL_REPLIES" => Some(g_ASK_CALL_REPLIES()),
            "ASK_GO_REPLIES" => Some(g_ASK_GO_REPLIES()),
            "ASPIRIN_REPLY" => Some(g_ASPIRIN_REPLY()),
            "A_BREATHER" => Some(g_A_BREATHER()),
            "A_FOLLOW_UP_ABOUT_THE_INJURY" => Some(g_A_FOLLOW_UP_ABOUT_THE_INJURY()),
            "A_GROWN_AGE" => Some(g_A_GROWN_AGE()),
            "A_HAND_ON_THE_PERSON" => Some(g_A_HAND_ON_THE_PERSON()),
            "A_HURT_OF_A_BODY_PART" => Some(g_A_HURT_OF_A_BODY_PART()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_PLAN_ORDINARY" => Some(g_A_PLAN_ORDINARY()),
            "A_PLAN_SAID" => Some(g_A_PLAN_SAID()),
            "A_REAL_DANGER" => Some(g_A_REAL_DANGER()),
            "A_WISH_OR_TALK_OF_IT" => Some(g_A_WISH_OR_TALK_OF_IT()),
            "A_WORKOUT_CUE" => Some(g_A_WORKOUT_CUE()),
            "BABY_FELL" => Some(g_BABY_FELL()),
            "BABY_FELL_OFF" => Some(g_BABY_FELL_OFF()),
            "BABY_FELL_RED" => Some(g_BABY_FELL_RED()),
            "BABY_FELL_SURFACE" => Some(g_BABY_FELL_SURFACE()),
            "BATTERY_BULGE" => Some(g_BATTERY_BULGE()),
            "BATTERY_FIRE" => Some(g_BATTERY_FIRE()),
            "BITE_ON_ANIMAL" => Some(g_BITE_ON_ANIMAL()),
            "BLACKOUT_A_THING" => Some(g_BLACKOUT_A_THING()),
            "BLACKOUT_BREATH" => Some(g_BLACKOUT_BREATH()),
            "BLACKOUT_EFFORT" => Some(g_BLACKOUT_EFFORT()),
            "BLACKOUT_FLOOR" => Some(g_BLACKOUT_FLOOR()),
            "BLACKOUT_OFF" => Some(g_BLACKOUT_OFF()),
            "BLACKOUT_REPEAT" => Some(g_BLACKOUT_REPEAT()),
            "BLACKOUT_SOMEONE_DOWN" => Some(g_BLACKOUT_SOMEONE_DOWN()),
            "BLEACH_MIX" => Some(g_BLEACH_MIX()),
            "BLEACH_MIX_Q" => Some(g_BLEACH_MIX_Q()),
            "BLEEDING" => Some(g_BLEEDING()),
            "BLIND_CORD" => Some(g_BLIND_CORD()),
            "BLISTER_CAUSE_BURN" => Some(g_BLISTER_CAUSE_BURN()),
            "BLISTER_STATE" => Some(g_BLISTER_STATE()),
            "BLISTER_STATE_REPLIES" => Some(g_BLISTER_STATE_REPLIES()),
            "BLOW_ON_ME" => Some(g_BLOW_ON_ME()),
            "BLUE" => Some(g_BLUE()),
            "BLUE_CLOTH" => Some(g_BLUE_CLOTH()),
            "BLUE_COLD_CAUSE" => Some(g_BLUE_COLD_CAUSE()),
            "BLUE_FINE" => Some(g_BLUE_FINE()),
            "BLUE_FOOD" => Some(g_BLUE_FOOD()),
            "BLUE_OTHER_PERSON" => Some(g_BLUE_OTHER_PERSON()),
            "BLUE_PERSON_SUBJECT" => Some(g_BLUE_PERSON_SUBJECT()),
            "BLUE_STRONG" => Some(g_BLUE_STRONG()),
            "BLUE_TINGE" => Some(g_BLUE_TINGE()),
            "BODY_PART" => Some(g_BODY_PART()),
            "BONE_ORDINARY" => Some(g_BONE_ORDINARY()),
            "BONE_PAST" => Some(g_BONE_PAST()),
            "BONE_THROAT" => Some(g_BONE_THROAT()),
            "BREATH_CAUSE" => Some(g_BREATH_CAUSE()),
            "BREATH_CAUSE_RED" => Some(g_BREATH_CAUSE_RED()),
            "BREATH_EFFORT" => Some(g_BREATH_EFFORT()),
            "BREATH_EFFORT_RED" => Some(g_BREATH_EFFORT_RED()),
            "BREATH_LAUGH" => Some(g_BREATH_LAUGH()),
            "BREATH_LAUGH_REPLY" => Some(g_BREATH_LAUGH_REPLY()),
            "BREATH_PAST_OVER" => Some(g_BREATH_PAST_OVER()),
            "BREATH_RED" => Some(g_BREATH_RED()),
            "BREATH_REST" => Some(g_BREATH_REST()),
            "BREATH_SHORT" => Some(g_BREATH_SHORT()),
            "BURNT_FOOD" => Some(g_BURNT_FOOD()),
            "BURNT_SELF" => Some(g_BURNT_SELF()),
            "BURN_MARK_ON_A_BODY" => Some(g_BURN_MARK_ON_A_BODY()),
            "CALL_HAS_A_SIGN" => Some(g_CALL_HAS_A_SIGN()),
            "CALL_HELP_REPLY" => Some(g_CALL_HELP_REPLY()),
            "CASE_WORDS" => Some(g_CASE_WORDS()),
            "CHEM_WORD" => Some(g_CHEM_WORD()),
            "CHEST_ASKED" => Some(g_CHEST_ASKED()),
            "CHEST_EFFORT" => Some(g_CHEST_EFFORT()),
            "CHEST_IN_A_HAND" => Some(g_CHEST_IN_A_HAND()),
            "CHEST_IN_A_HAND_TAIL" => Some(g_CHEST_IN_A_HAND_TAIL()),
            "CHEST_SLOW" => Some(g_CHEST_SLOW()),
            "CHEST_SPREADING" => Some(g_CHEST_SPREADING()),
            "CHEST_URGENT" => Some(g_CHEST_URGENT()),
            "CHEST_WORK" => Some(g_CHEST_WORK()),
            "CHILDHOOD_FIT" => Some(g_CHILDHOOD_FIT()),
            "CHILD_FEVER_NOT_EATING" => Some(g_CHILD_FEVER_NOT_EATING()),
            "CHILD_FEVER_RED" => Some(g_CHILD_FEVER_RED()),
            "CHILD_GROWN" => Some(g_CHILD_GROWN()),
            "CHILD_SCHOOL_AGE" => Some(g_CHILD_SCHOOL_AGE()),
            "CHILD_YOUNG" => Some(g_CHILD_YOUNG()),
            "CHOKE_AIR_OK" => Some(g_CHOKE_AIR_OK()),
            "CHOKE_NOT_PERSON" => Some(g_CHOKE_NOT_PERSON()),
            "CHOKE_OVER" => Some(g_CHOKE_OVER()),
            "CHOKE_PERSON" => Some(g_CHOKE_PERSON()),
            "CHOKE_PREVENT" => Some(g_CHOKE_PREVENT()),
            "CHOKE_RED" => Some(g_CHOKE_RED()),
            "CHOKE_SETTLED" => Some(g_CHOKE_SETTLED()),
            "CHOKE_STILL_RED" => Some(g_CHOKE_STILL_RED()),
            "CHRONIC_BREATH" => Some(g_CHRONIC_BREATH()),
            "CLEANABLE_SURFACE" => Some(g_CLEANABLE_SURFACE()),
            "COLD_ILLNESS" => Some(g_COLD_ILLNESS()),
            "COLLAPSE_HINT" => Some(g_COLLAPSE_HINT()),
            "CONFUSED_THING" => Some(g_CONFUSED_THING()),
            "COUGH_BLOOD" => Some(g_COUGH_BLOOD()),
            "CO_GOING_OFF" => Some(g_CO_GOING_OFF()),
            "CO_LEAK" => Some(g_CO_LEAK()),
            "CO_SIGNS" => Some(g_CO_SIGNS()),
            "CO_TEST" => Some(g_CO_TEST()),
            "CPR_BABY_SCENE" => Some(g_CPR_BABY_SCENE()),
            "CPR_CHILD" => Some(g_CPR_CHILD()),
            "CPR_CHILD_SCENE" => Some(g_CPR_CHILD_SCENE()),
            "CPR_HOW" => Some(g_CPR_HOW()),
            "CPR_HOWTO" => Some(g_CPR_HOWTO()),
            "CPR_HOW_BABY" => Some(g_CPR_HOW_BABY()),
            "CPR_HOW_CHILD" => Some(g_CPR_HOW_CHILD()),
            "CPR_NOT_EFFORT" => Some(g_CPR_NOT_EFFORT()),
            "CPR_NO_PULSE_NOW" => Some(g_CPR_NO_PULSE_NOW()),
            "CPR_ROOM_EMPTY" => Some(g_CPR_ROOM_EMPTY()),
            "CPR_ROUGH" => Some(g_CPR_ROUGH()),
            "CPR_START" => Some(g_CPR_START()),
            "CPR_STOP" => Some(g_CPR_STOP()),
            "CPR_STOP_REPLY" => Some(g_CPR_STOP_REPLY()),
            "CPR_WHEN_ALIVE" => Some(g_CPR_WHEN_ALIVE()),
            "CRISIS_PLAN_CARRY" => Some(g_CRISIS_PLAN_CARRY()),
            "CRISIS_PLAN_ORDINARY" => Some(g_CRISIS_PLAN_ORDINARY()),
            "CRISIS_PLAN_REPLY" => Some(g_CRISIS_PLAN_REPLY()),
            "CRISIS_PLAN_SAID" => Some(g_CRISIS_PLAN_SAID()),
            "CRISIS_WORD" => Some(g_CRISIS_WORD()),
            "CUT_NOT_STOPPING" => Some(g_CUT_NOT_STOPPING()),
            "DAY_TOTAL_SIGN" => Some(g_DAY_TOTAL_SIGN()),
            "DAY_TOTAL_WORD" => Some(g_DAY_TOTAL_WORD()),
            "DENTAL_WORDS" => Some(g_DENTAL_WORDS()),
            "DETAIL_AFTER_ESCALATION" => Some(g_DETAIL_AFTER_ESCALATION()),
            "DEVICE_BURNING" => Some(g_DEVICE_BURNING()),
            "DEVICE_OLD" => Some(g_DEVICE_OLD()),
            "DIE_OF_THE_FEELING" => Some(g_DIE_OF_THE_FEELING()),
            "DISTRESS_BESIDE_IT" => Some(g_DISTRESS_BESIDE_IT()),
            "DOG_HABIT" => Some(g_DOG_HABIT()),
            "DOG_MOUTHING" => Some(g_DOG_MOUTHING()),
            "DOG_SELF_BITE" => Some(g_DOG_SELF_BITE()),
            "DOSE_AMOUNT" => Some(g_DOSE_AMOUNT()),
            "DOSE_ASKED" => Some(g_DOSE_ASKED()),
            "DOSE_COURSE" => Some(g_DOSE_COURSE()),
            "DOSE_COURSE_KEEP" => Some(g_DOSE_COURSE_KEEP()),
            "DOSE_WORDS" => Some(g_DOSE_WORDS()),
            "DRINK_WORDS" => Some(g_DRINK_WORDS()),
            "DROWNED_THING" => Some(g_DROWNED_THING()),
            "DROWNING_HYPERBOLE" => Some(g_DROWNING_HYPERBOLE()),
            "DRUG_NAME" => Some(g_DRUG_NAME()),
            "EDGE_OF_A_DROP" => Some(g_EDGE_OF_A_DROP()),
            "EFFORT_AFTER" => Some(g_EFFORT_AFTER()),
            "EFFORT_ANYWHERE" => Some(g_EFFORT_ANYWHERE()),
            "EFFORT_BENIGN" => Some(g_EFFORT_BENIGN()),
            "EFFORT_HEART_CLUSTER" => Some(g_EFFORT_HEART_CLUSTER()),
            "EFFORT_PATTERN" => Some(g_EFFORT_PATTERN()),
            "EFFORT_RED_HEART" => Some(g_EFFORT_RED_HEART()),
            "EFFORT_TOO_LONG" => Some(g_EFFORT_TOO_LONG()),
            "ELECTRIC_BURN" => Some(g_ELECTRIC_BURN()),
            "ELECTRIC_FINE" => Some(g_ELECTRIC_FINE()),
            "ELECTRIC_SHOCK_DEFINITION" => Some(g_ELECTRIC_SHOCK_DEFINITION()),
            "ELECTRIC_SHOCK_DOWN" => Some(g_ELECTRIC_SHOCK_DOWN()),
            "END_IT_MEANS" => Some(g_END_IT_MEANS()),
            "END_IT_WITH_A_THING" => Some(g_END_IT_WITH_A_THING()),
            "END_IT_WITH_NOUN" => Some(g_END_IT_WITH_NOUN()),
            "END_MEANS_OTHER" => Some(g_END_MEANS_OTHER()),
            "END_THING" => Some(g_END_THING()),
            "EXERTION" => Some(g_EXERTION()),
            "EXERTION_CLASS" => Some(g_EXERTION_CLASS()),
            "EXERTION_SMALL" => Some(g_EXERTION_SMALL()),
            "EYE_CHEM" => Some(g_EYE_CHEM()),
            "EYE_FINE" => Some(g_EYE_FINE()),
            "FAINT_STORY" => Some(g_FAINT_STORY()),
            "FALL_ANSWERED" => Some(g_FALL_ANSWERED()),
            "FALL_CHECK_OFF" => Some(g_FALL_CHECK_OFF()),
            "FALL_CORE" => Some(g_FALL_CORE()),
            "FALL_FINE" => Some(g_FALL_FINE()),
            "FALL_HIP" => Some(g_FALL_HIP()),
            "FALL_NOW" => Some(g_FALL_NOW()),
            "FALL_PAST" => Some(g_FALL_PAST()),
            "FALL_RED" => Some(g_FALL_RED()),
            "FALL_REPEAT" => Some(g_FALL_REPEAT()),
            "FALL_WORD_IDIOM" => Some(g_FALL_WORD_IDIOM()),
            "FEMALE_WORDS" => Some(g_FEMALE_WORDS()),
            "FEVER_ASK" => Some(g_FEVER_ASK()),
            "FEVER_RASH_FINE" => Some(g_FEVER_RASH_FINE()),
            "FEVER_REPORT" => Some(g_FEVER_REPORT()),
            "FICTION_NOW" => Some(g_FICTION_NOW()),
            "FIRE" => Some(g_FIRE()),
            "FIRE_DRILL" => Some(g_FIRE_DRILL()),
            "FIRE_ELSEWHERE" => Some(g_FIRE_ELSEWHERE()),
            "FIRE_NOW" => Some(g_FIRE_NOW()),
            "FIRE_OUT" => Some(g_FIRE_OUT()),
            "FIRE_SCALE" => Some(g_FIRE_SCALE()),
            "FITTING_IN" => Some(g_FITTING_IN()),
            "FITTING_MANNER_TAIL" => Some(g_FITTING_MANNER_TAIL()),
            "FIT_ALL_BETTER" => Some(g_FIT_ALL_BETTER()),
            "FIT_CASES" => Some(g_FIT_CASES()),
            "FIT_ROUTINE" => Some(g_FIT_ROUTINE()),
            "FIT_STILL_RINGS" => Some(g_FIT_STILL_RINGS()),
            "FIT_THIS_WEEK" => Some(g_FIT_THIS_WEEK()),
            "FLAMES" => Some(g_FLAMES()),
            "FLOPPY_RED_FLAG" => Some(g_FLOPPY_RED_FLAG()),
            "FLOPPY_SLEEPY" => Some(g_FLOPPY_SLEEPY()),
            "FLOPPY_THING" => Some(g_FLOPPY_THING()),
            "FOLLOW_UP_ANCHOR" => Some(g_FOLLOW_UP_ANCHOR()),
            "FOLLOW_UP_MORE" => Some(g_FOLLOW_UP_MORE()),
            "FOOD_CUT" => Some(g_FOOD_CUT()),
            "FORAGED_EATEN" => Some(g_FORAGED_EATEN()),
            "FORAGED_FINE" => Some(g_FORAGED_FINE()),
            "FRICTION_WORD" => Some(g_FRICTION_WORD()),
            "FRIGHTENED" => Some(g_FRIGHTENED()),
            "FRIGHTENED_REPLY" => Some(g_FRIGHTENED_REPLY()),
            "GARMENT" => Some(g_GARMENT()),
            "GASPING_THIRST" => Some(g_GASPING_THIRST()),
            "GASP_FINE" => Some(g_GASP_FINE()),
            "GASP_RED" => Some(g_GASP_RED()),
            "GAS_LEFT_ON" => Some(g_GAS_LEFT_ON()),
            "GAS_ON_NOT_RUNNING" => Some(g_GAS_ON_NOT_RUNNING()),
            "GAS_ON_YOU" => Some(g_GAS_ON_YOU()),
            "GAS_PAST" => Some(g_GAS_PAST()),
            "GAS_SMELL" => Some(g_GAS_SMELL()),
            "GIVING_UP_GROUP" => Some(g_GIVING_UP_GROUP()),
            "GIVING_UP_HELD_ON" => Some(g_GIVING_UP_HELD_ON()),
            "GO_IN_NOT_ABOUT_IT" => Some(g_GO_IN_NOT_ABOUT_IT()),
            "GO_IS_A_CALL" => Some(g_GO_IS_A_CALL()),
            "GRUNTING_FINE" => Some(g_GRUNTING_FINE()),
            "HAIR_SCORCH" => Some(g_HAIR_SCORCH()),
            "HAIR_SCORCH_ANSWER" => Some(g_HAIR_SCORCH_ANSWER()),
            "HANGED_LONG_AGO" => Some(g_HANGED_LONG_AGO()),
            "HANGING" => Some(g_HANGING()),
            "HARD_TO_WAKE" => Some(g_HARD_TO_WAKE()),
            "HEADACHE_BAD" => Some(g_HEADACHE_BAD()),
            "HEADACHE_STORY" => Some(g_HEADACHE_STORY()),
            "HEADACHE_SUDDEN" => Some(g_HEADACHE_SUDDEN()),
            "HEAD_BANG" => Some(g_HEAD_BANG()),
            "HEAD_KNOCKOUT" => Some(g_HEAD_KNOCKOUT()),
            "HEAD_RED" => Some(g_HEAD_RED()),
            "HEAL_WORDS" => Some(g_HEAL_WORDS()),
            "HEARD_ASSAULT" => Some(g_HEARD_ASSAULT()),
            "HEAT" => Some(g_HEAT()),
            "HELD_CRISIS_DETAIL" => Some(g_HELD_CRISIS_DETAIL()),
            "HELD_CRISIS_OFF" => Some(g_HELD_CRISIS_OFF()),
            "HELP_ARRIVED" => Some(g_HELP_ARRIVED()),
            "HELP_SOMEONE" => Some(g_HELP_SOMEONE()),
            "HIT_IN_FUN" => Some(g_HIT_IN_FUN()),
            "HIT_THE_LAUGH_IS_HIS" => Some(g_HIT_THE_LAUGH_IS_HIS()),
            "HOW_LONG" => Some(g_HOW_LONG()),
            "HOW_TO_TREAT" => Some(g_HOW_TO_TREAT()),
            "HUMAN_TOO" => Some(g_HUMAN_TOO()),
            "HYPERBOLE_BLOCK" => Some(g_HYPERBOLE_BLOCK()),
            "HYPO_HOW_KNOW" => Some(g_HYPO_HOW_KNOW()),
            "HYPO_QUESTION" => Some(g_HYPO_QUESTION()),
            "HYPO_STRONG" => Some(g_HYPO_STRONG()),
            "HYPO_STRONG_OTHER" => Some(g_HYPO_STRONG_OTHER()),
            "INJURY_WORDS" => Some(g_INJURY_WORDS()),
            "INTRUDER" => Some(g_INTRUDER()),
            "KICKED_PERSON" => Some(g_KICKED_PERSON()),
            "LIMP_LONG" => Some(g_LIMP_LONG()),
            "LIMP_MECHANISM" => Some(g_LIMP_MECHANISM()),
            "LIMP_OTHER_LIMB" => Some(g_LIMP_OTHER_LIMB()),
            "LIMP_SOMEBODY_ELSE" => Some(g_LIMP_SOMEBODY_ELSE()),
            "LIPS_COLD" => Some(g_LIPS_COLD()),
            "LIPS_SWELL" => Some(g_LIPS_SWELL()),
            "LIPS_SWELL_BARE_OFF" => Some(g_LIPS_SWELL_BARE_OFF()),
            "LIPS_TINGLE" => Some(g_LIPS_TINGLE()),
            "LIPS_TINGLE_FOOD" => Some(g_LIPS_TINGLE_FOOD()),
            "LIPS_TINGLE_OFF" => Some(g_LIPS_TINGLE_OFF()),
            "LIPS_TINGLE_RED" => Some(g_LIPS_TINGLE_RED()),
            "LOCKED_IN_RISK" => Some(g_LOCKED_IN_RISK()),
            "LOOKS_REAL" => Some(g_LOOKS_REAL()),
            "LOSS_SAID" => Some(g_LOSS_SAID()),
            "LOSS_SUBJECT" => Some(g_LOSS_SUBJECT()),
            "MACHINE" => Some(g_MACHINE()),
            "MEAL_BREATH" => Some(g_MEAL_BREATH()),
            "MEDICINE" => Some(g_MEDICINE()),
            "MENINGITIS_OTHER" => Some(g_MENINGITIS_OTHER()),
            "MILD_BODY_SIGN" => Some(g_MILD_BODY_SIGN()),
            "MILD_CHOKE" => Some(g_MILD_CHOKE()),
            "MILD_PRODUCT_SMALL" => Some(g_MILD_PRODUCT_SMALL()),
            "MILD_SWALLOWED" => Some(g_MILD_SWALLOWED()),
            "MORNING_ROUTINE" => Some(g_MORNING_ROUTINE()),
            "MOUTH_BLEED_NOW" => Some(g_MOUTH_BLEED_NOW()),
            "NAMED_SICK_FINE" => Some(g_NAMED_SICK_FINE()),
            "NEAR_FIRE" => Some(g_NEAR_FIRE()),
            "NEAR_FIRE_OVER" => Some(g_NEAR_FIRE_OVER()),
            "NEAR_THING_OVER" => Some(g_NEAR_THING_OVER()),
            "NEWBORN" => Some(g_NEWBORN()),
            "NIGHT_BREATH_OTHER" => Some(g_NIGHT_BREATH_OTHER()),
            "NIGHT_BREATH_PAST" => Some(g_NIGHT_BREATH_PAST()),
            "NIGHT_BREATH_RED" => Some(g_NIGHT_BREATH_RED()),
            "NOSE_RED" => Some(g_NOSE_RED()),
            "NOT_A_WOUND_OFF" => Some(g_NOT_A_WOUND_OFF()),
            "NOT_NOW" => Some(g_NOT_NOW()),
            "NOT_NOW_FICTION" => Some(g_NOT_NOW_FICTION()),
            "NOT_STOPPING" => Some(g_NOT_STOPPING()),
            "OBJECT_IN_HEAD_THING" => Some(g_OBJECT_IN_HEAD_THING()),
            "OLDER_CHILD_NAMED" => Some(g_OLDER_CHILD_NAMED()),
            "OLD_FALL_FINE" => Some(g_OLD_FALL_FINE()),
            "OLD_FALL_NOT" => Some(g_OLD_FALL_NOT()),
            "OLD_PERSON" => Some(g_OLD_PERSON()),
            "OLD_TIME" => Some(g_OLD_TIME()),
            "OTHER_RISK" => Some(g_OTHER_RISK()),
            "OTHER_SELF_HARM" => Some(g_OTHER_SELF_HARM()),
            "OUT_COLD_ASLEEP" => Some(g_OUT_COLD_ASLEEP()),
            "OUT_COLD_PERSON" => Some(g_OUT_COLD_PERSON()),
            "OUT_COLD_REPLY" => Some(g_OUT_COLD_REPLY()),
            "OVERDOSE_DRUG" => Some(g_OVERDOSE_DRUG()),
            "OVERDOSE_FOOD" => Some(g_OVERDOSE_FOOD()),
            "OVERDOSE_OF_A_TREAT" => Some(g_OVERDOSE_OF_A_TREAT()),
            "OWN_CRY" => Some(g_OWN_CRY()),
            "OWN_INJURY" => Some(g_OWN_INJURY()),
            "OWN_INJURY_IN_MY_CARE" => Some(g_OWN_INJURY_IN_MY_CARE()),
            "OWN_INJURY_QUESTION" => Some(g_OWN_INJURY_QUESTION()),
            "OWN_LIMB_AT_AN_ANGLE" => Some(g_OWN_LIMB_AT_AN_ANGLE()),
            "PALE_EFFORT_RED" => Some(g_PALE_EFFORT_RED()),
            "PALE_EXERTION" => Some(g_PALE_EXERTION()),
            "PALE_HEAT" => Some(g_PALE_HEAT()),
            "PALE_SECOND" => Some(g_PALE_SECOND()),
            "PAN_FIRE" => Some(g_PAN_FIRE()),
            "PARENT_REFUSAL_BLOW" => Some(g_PARENT_REFUSAL_BLOW()),
            "PAST_ATTEMPT_NOT" => Some(g_PAST_ATTEMPT_NOT()),
            "PAST_WISH" => Some(g_PAST_WISH()),
            "PAST_WISH_NOT" => Some(g_PAST_WISH_NOT()),
            "PERSON" => Some(g_PERSON()),
            "PERSON_AS_ATTACKER" => Some(g_PERSON_AS_ATTACKER()),
            "PERSON_BURNT" => Some(g_PERSON_BURNT()),
            "PET_ALL_RIGHT" => Some(g_PET_ALL_RIGHT()),
            "PET_ANY" => Some(g_PET_ANY()),
            "PET_GASP_EXPLAINED" => Some(g_PET_GASP_EXPLAINED()),
            "PET_GASP_FINE" => Some(g_PET_GASP_FINE()),
            "PET_GASP_RED" => Some(g_PET_GASP_RED()),
            "PET_NAMES" => Some(g_PET_NAMES()),
            "PET_PAST" => Some(g_PET_PAST()),
            "PET_RESPOND_AT" => Some(g_PET_RESPOND_AT()),
            "PET_TOOTH" => Some(g_PET_TOOTH()),
            "PLAYFUL_HIT" => Some(g_PLAYFUL_HIT()),
            "POISON_SAID" => Some(g_POISON_SAID()),
            "POO_BLACK" => Some(g_POO_BLACK()),
            "POO_BLOOD" => Some(g_POO_BLOOD()),
            "POO_FINE" => Some(g_POO_FINE()),
            "PREGNANCY_WORDS" => Some(g_PREGNANCY_WORDS()),
            "PUNCTURE" => Some(g_PUNCTURE()),
            "PUSHED_AWAY" => Some(g_PUSHED_AWAY()),
            "PUSHED_AWAY_BUT_SOMETHING_ELSE" => Some(g_PUSHED_AWAY_BUT_SOMETHING_ELSE()),
            "PUSHED_AWAY_REPLY" => Some(g_PUSHED_AWAY_REPLY()),
            "PUSHED_AWAY_THEN_RESTRAINT" => Some(g_PUSHED_AWAY_THEN_RESTRAINT()),
            "RASH_FADE" => Some(g_RASH_FADE()),
            "RASH_HEADACHE_FINE" => Some(g_RASH_HEADACHE_FINE()),
            "RASH_PRESSED" => Some(g_RASH_PRESSED()),
            "RAZOR_BURN" => Some(g_RAZOR_BURN()),
            "RECENT_FIT_FINE" => Some(g_RECENT_FIT_FINE()),
            "RECENT_TIME" => Some(g_RECENT_TIME()),
            "RESPOND_SENSE" => Some(g_RESPOND_SENSE()),
            "RESTRAINT_BESIDE" => Some(g_RESTRAINT_BESIDE()),
            "RING_DANGER" => Some(g_RING_DANGER()),
            "RING_NOT_WORN" => Some(g_RING_NOT_WORN()),
            "ROUTINE_REASON" => Some(g_ROUTINE_REASON()),
            "ROUTINE_WAKE" => Some(g_ROUTINE_WAKE()),
            "SAFE_NOW" => Some(g_SAFE_NOW()),
            "SAID_AS_IT_HAPPENS" => Some(g_SAID_AS_IT_HAPPENS()),
            "SEIZURE" => Some(g_SEIZURE()),
            "SEIZURE_FIGURATIVE" => Some(g_SEIZURE_FIGURATIVE()),
            "SEIZURE_STORY_GUARD" => Some(g_SEIZURE_STORY_GUARD()),
            "SELF_CUT" => Some(g_SELF_CUT()),
            "SELF_HARM_AGAIN" => Some(g_SELF_HARM_AGAIN()),
            "SELF_HARM_BY_A_MEANS" => Some(g_SELF_HARM_BY_A_MEANS()),
            "SELF_HARM_CUT_NO_OBJECT" => Some(g_SELF_HARM_CUT_NO_OBJECT()),
            "SELF_HARM_DELIBERATE_ACT" => Some(g_SELF_HARM_DELIBERATE_ACT()),
            "SELF_HARM_DESERVE" => Some(g_SELF_HARM_DESERVE()),
            "SELF_HARM_LOOKALIKE" => Some(g_SELF_HARM_LOOKALIKE()),
            "SELF_HARM_NOT_EATING" => Some(g_SELF_HARM_NOT_EATING()),
            "SELF_HARM_OWN_PART" => Some(g_SELF_HARM_OWN_PART()),
            "SELF_HARM_REASON" => Some(g_SELF_HARM_REASON()),
            "SELF_HARM_SCARS" => Some(g_SELF_HARM_SCARS()),
            "SELF_HARM_SCARS_NOT" => Some(g_SELF_HARM_SCARS_NOT()),
            "SELF_HARM_TOPIC_OFF" => Some(g_SELF_HARM_TOPIC_OFF()),
            "SELF_HARM_WISHES_A_PART" => Some(g_SELF_HARM_WISHES_A_PART()),
            "SELF_RUMINATION" => Some(g_SELF_RUMINATION()),
            "SELF_TALK" => Some(g_SELF_TALK()),
            "SEPSIS_SIGN" => Some(g_SEPSIS_SIGN()),
            "SHAKEN" => Some(g_SHAKEN()),
            "SHAKEN_REPLY" => Some(g_SHAKEN_REPLY()),
            "SHOCK_ANY" => Some(g_SHOCK_ANY()),
            "SHORT_OF_BREATH" => Some(g_SHORT_OF_BREATH()),
            "SHOT" => Some(g_SHOT()),
            "SHOT_FINE" => Some(g_SHOT_FINE()),
            "SIGHT_A_WHILE" => Some(g_SIGHT_A_WHILE()),
            "SLUR_RED_BESIDE" => Some(g_SLUR_RED_BESIDE()),
            "SLUR_THICK" => Some(g_SLUR_THICK()),
            "SMALL_CHILD" => Some(g_SMALL_CHILD()),
            "SMALL_CHILD_LASH_REPLIES" => Some(g_SMALL_CHILD_LASH_REPLIES()),
            "SMOKE_IS_A_SMELL" => Some(g_SMOKE_IS_A_SMELL()),
            "SNAKE_LOOKALIKE" => Some(g_SNAKE_LOOKALIKE()),
            "SOCKET_OBJECT" => Some(g_SOCKET_OBJECT()),
            "SOCKET_THING_FINE" => Some(g_SOCKET_THING_FINE()),
            "SOFT_BLEED_OFF" => Some(g_SOFT_BLEED_OFF()),
            "SOMEONE_ELSES_DESPAIR" => Some(g_SOMEONE_ELSES_DESPAIR()),
            "SOMEONE_ELSE_OWN" => Some(g_SOMEONE_ELSE_OWN()),
            "SOME_OF_IT" => Some(g_SOME_OF_IT()),
            "SPELL_RED" => Some(g_SPELL_RED()),
            "SPIKED" => Some(g_SPIKED()),
            "SPINE_BACK_WORK" => Some(g_SPINE_BACK_WORK()),
            "SPINE_BREAK" => Some(g_SPINE_BREAK()),
            "SPOKEN_ATTEMPT_UNDATED" => Some(g_SPOKEN_ATTEMPT_UNDATED()),
            "STIFF_FINE" => Some(g_STIFF_FINE()),
            "STIFF_NECK" => Some(g_STIFF_NECK()),
            "STILL_GOING" => Some(g_STILL_GOING()),
            "STING_SETTLED" => Some(g_STING_SETTLED()),
            "STOOL_LONG" => Some(g_STOOL_LONG()),
            "STRANGLED_SELF" => Some(g_STRANGLED_SELF()),
            "STRANGLE_THING_SUBJECT" => Some(g_STRANGLE_THING_SUBJECT()),
            "STREAK_FEVER" => Some(g_STREAK_FEVER()),
            "STROKE_HAND" => Some(g_STROKE_HAND()),
            "STROKE_HAND_BESIDE" => Some(g_STROKE_HAND_BESIDE()),
            "STROKE_HAND_CHRONIC" => Some(g_STROKE_HAND_CHRONIC()),
            "STROKE_HAND_HARD" => Some(g_STROKE_HAND_HARD()),
            "STROKE_WEAK" => Some(g_STROKE_WEAK()),
            "STROKE_WEAK_OFF" => Some(g_STROKE_WEAK_OFF()),
            "STUCK_MACHINE" => Some(g_STUCK_MACHINE()),
            "SUICIDE_ABOUT_A_THING" => Some(g_SUICIDE_ABOUT_A_THING()),
            "SUICIDE_TOPIC" => Some(g_SUICIDE_TOPIC()),
            "SWALLOWED_BAD_REPLY" => Some(g_SWALLOWED_BAD_REPLY()),
            "SWALLOWED_BAD_SELF" => Some(g_SWALLOWED_BAD_SELF()),
            "SWALLOWED_ORDINARY" => Some(g_SWALLOWED_ORDINARY()),
            "SWALLOWED_SELF" => Some(g_SWALLOWED_SELF()),
            "SWALLOW_GONE_OFF" => Some(g_SWALLOW_GONE_OFF()),
            "SWALLOW_GONE_RED" => Some(g_SWALLOW_GONE_RED()),
            "TALK_HABIT" => Some(g_TALK_HABIT()),
            "TESTIS_LUMP" => Some(g_TESTIS_LUMP()),
            "THING_BURNT" => Some(g_THING_BURNT()),
            "THING_BURNT_WORDS" => Some(g_THING_BURNT_WORDS()),
            "THING_THAT_IS_SUICIDE" => Some(g_THING_THAT_IS_SUICIDE()),
            "THIRD_PERSON_ACTOR" => Some(g_THIRD_PERSON_ACTOR()),
            "THROAT_TIGHT_CAUSE" => Some(g_THROAT_TIGHT_CAUSE()),
            "THROAT_TIGHT_EFFORT" => Some(g_THROAT_TIGHT_EFFORT()),
            "THROAT_TIGHT_RED" => Some(g_THROAT_TIGHT_RED()),
            "TINY_AGE" => Some(g_TINY_AGE()),
            "TINY_AGE_FINE" => Some(g_TINY_AGE_FINE()),
            "TOOK_OUT_OF" => Some(g_TOOK_OUT_OF()),
            "TOOTH_OUT_FINE" => Some(g_TOOTH_OUT_FINE()),
            "TORSION" => Some(g_TORSION()),
            "TORSION_FINE" => Some(g_TORSION_FINE()),
            "TORSION_ONSET" => Some(g_TORSION_ONSET()),
            "TRAPPED_HELD_BY_A_PERSON" => Some(g_TRAPPED_HELD_BY_A_PERSON()),
            "TRAPPED_VEHICLE_NOT_NOW" => Some(g_TRAPPED_VEHICLE_NOT_NOW()),
            "TYPED_TYPOS" => Some(g_TYPED_TYPOS()),
            "VEHICLE_BURNING_SMELL" => Some(g_VEHICLE_BURNING_SMELL()),
            "VEHICLE_BURNING_SMELL_HARD" => Some(g_VEHICLE_BURNING_SMELL_HARD()),
            "VEHICLE_EXHAUST" => Some(g_VEHICLE_EXHAUST()),
            "VEHICLE_FIRE" => Some(g_VEHICLE_FIRE()),
            "VEHICLE_FIRE_OVER" => Some(g_VEHICLE_FIRE_OVER()),
            "VIOLENT_THREAT" => Some(g_VIOLENT_THREAT()),
            "VISIT_OR_TIME" => Some(g_VISIT_OR_TIME()),
            "WAKE_RED" => Some(g_WAKE_RED()),
            "WAKE_UP_SOMEONE" => Some(g_WAKE_UP_SOMEONE()),
            "WATER_DROWNING_FINE" => Some(g_WATER_DROWNING_FINE()),
            "WATER_DROWNING_NOW" => Some(g_WATER_DROWNING_NOW()),
            "WATER_FINE" => Some(g_WATER_FINE()),
            "WATER_HEAD_FEAR" => Some(g_WATER_HEAD_FEAR()),
            "WATER_HUMAN" => Some(g_WATER_HUMAN()),
            "WATER_IN_NOW" => Some(g_WATER_IN_NOW()),
            "WATER_NOT_A_PERSON" => Some(g_WATER_NOT_A_PERSON()),
            "WATER_PERSON_TOO" => Some(g_WATER_PERSON_TOO()),
            "WATER_SELF" => Some(g_WATER_SELF()),
            "WATER_THING_DROPPED" => Some(g_WATER_THING_DROPPED()),
            "WATER_URGENT" => Some(g_WATER_URGENT()),
            "WEIGHT_HIP" => Some(g_WEIGHT_HIP()),
            "WEIGHT_LONG" => Some(g_WEIGHT_LONG()),
            "WEIGHT_OFF_FEET" => Some(g_WEIGHT_OFF_FEET()),
            "WENT_BLUE_FINE" => Some(g_WENT_BLUE_FINE()),
            "WENT_BLUE_IDIOM" => Some(g_WENT_BLUE_IDIOM()),
            "WHEN_HELP_COMES" => Some(g_WHEN_HELP_COMES()),
            "WHEN_HELP_REPLY" => Some(g_WHEN_HELP_REPLY()),
            "WHOLE_BODY_RED_FLAG" => Some(g_WHOLE_BODY_RED_FLAG()),
            "WISH_BY_MODAL" => Some(g_WISH_BY_MODAL()),
            "WISH_DEAD_HYPERBOLE" => Some(g_WISH_DEAD_HYPERBOLE()),
            "WISH_DEAD_PLACE" => Some(g_WISH_DEAD_PLACE()),
            "WISH_DEAD_WHEN" => Some(g_WISH_DEAD_WHEN()),
            "WISH_NAMED" => Some(g_WISH_NAMED()),
            "WITHDRAWAL_ALONE_AT_HOME" => Some(g_WITHDRAWAL_ALONE_AT_HOME()),
            "WITHDRAWAL_ASK_DANGER" => Some(g_WITHDRAWAL_ASK_DANGER()),
            "WITHDRAWAL_DANGER" => Some(g_WITHDRAWAL_DANGER()),
            "WITHDRAWAL_HAIR_OF_THE_DOG" => Some(g_WITHDRAWAL_HAIR_OF_THE_DOG()),
            "WITHDRAWAL_HALLUCINATION" => Some(g_WITHDRAWAL_HALLUCINATION()),
            "WITHDRAWAL_NAMED" => Some(g_WITHDRAWAL_NAMED()),
            "WITHDRAWAL_NOT_ALCOHOL" => Some(g_WITHDRAWAL_NOT_ALCOHOL()),
            "WITHDRAWAL_ROUTINE" => Some(g_WITHDRAWAL_ROUTINE()),
            "WITHDRAWAL_SENSES_FINE" => Some(g_WITHDRAWAL_SENSES_FINE()),
            "WITHDRAWAL_STOP" => Some(g_WITHDRAWAL_STOP()),
            "WITHDRAWAL_STRONG" => Some(g_WITHDRAWAL_STRONG()),
            "WITHDRAWAL_SYMPTOM" => Some(g_WITHDRAWAL_SYMPTOM()),
            "WONT_COME_ROUND" => Some(g_WONT_COME_ROUND()),
            "WORDS_OUT_OCCASION" => Some(g_WORDS_OUT_OCCASION()),
            "WORLD_QUESTION" => Some(g_WORLD_QUESTION()),
            "WOUND_STREAK" => Some(g_WOUND_STREAK()),
            "WOUND_STREAK_FEVER" => Some(g_WOUND_STREAK_FEVER()),
            "WOUND_STREAK_OFF" => Some(g_WOUND_STREAK_OFF()),
            "_AGREEMENT" => Some(g__AGREEMENT()),
            "_ANY_MEANS_VERB" => Some(g__ANY_MEANS_VERB()),
            "_AWAY_TAIL" => Some(g__AWAY_TAIL()),
            "_A_MEANS_PLACE" => Some(g__A_MEANS_PLACE()),
            "_A_PURPOSE_AFTER" => Some(g__A_PURPOSE_AFTER()),
            "_A_THIRST" => Some(g__A_THIRST()),
            "_COPULA" => Some(g__COPULA()),
            "_DENIED" => Some(g__DENIED()),
            "_FITTING_A_THING" => Some(g__FITTING_A_THING()),
            "_FLAT_WISH_HOW" => Some(g__FLAT_WISH_HOW()),
            "_GASP_NOT_A_THIRST" => Some(g__GASP_NOT_A_THIRST()),
            "_HEIGHT_MEANS_VERB" => Some(g__HEIGHT_MEANS_VERB()),
            "_INTO_THE_WAY" => Some(g__INTO_THE_WAY()),
            "_OFF_AN_EDGE" => Some(g__OFF_AN_EDGE()),
            "_OFF_A_HEIGHT" => Some(g__OFF_A_HEIGHT()),
            "_PERSON" => Some(g__PERSON()),
            "_SMELLY_THING" => Some(g__SMELLY_THING()),
            "_SMOKE_THING" => Some(g__SMOKE_THING()),
            "_STREAK_MARK" => Some(g__STREAK_MARK()),
            "_STREAK_MOVE" => Some(g__STREAK_MOVE()),
            "_STREAK_ON_A_PERSON" => Some(g__STREAK_ON_A_PERSON()),
            "_STREAK_REDNESS" => Some(g__STREAK_REDNESS()),
            "_STRUCK_RELATIVE" => Some(g__STRUCK_RELATIVE()),
            "_THIRD" => Some(g__THIRD()),
            "_with_streak_fever" => Some(py_func("_with_streak_fever", |args: &[Py]| _with_streak_fever(args))),
            _ => None,
        }),
        set: None,
    }))
}
