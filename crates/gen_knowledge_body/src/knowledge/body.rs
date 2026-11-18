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


thread_local! { static G_ACHE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_ABOUT_A_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_YOUR_SHOULDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_YOUR_SHOULDER_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PART_NO_KNOCK_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_PREGNANCY_KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_WENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREAK_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BROKEN_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BROKEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOT_AND_CHEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPRESS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXERCISE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXERCISE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXTRA_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLAG_WITH_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOT_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOUR_STEPS_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAND_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT_FLAGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT_SWELLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_YOUR_HANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_YOUR_HANDS_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNEE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOWER_HALF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEDICINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOLE_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOLE_HAPPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOLE_REMOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMB_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMB_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMB_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMB_FOOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMB_HANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMB_LOWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMB_SHOE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECT_OWNERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_YOUR_FEET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAINKILLER_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE_OF_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RED_FLAG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RED_FLAG_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGION_BIAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RING_AS_A_CALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RING_NOT_ON_A_FINGER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SADDLE_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SADDLE_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEA_WATER_ALARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEA_WATER_ALARM_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEEN_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEE_DOCTOR_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEE_DOCTOR_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEE_TODAY_BACK_AND_CALF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_TISSUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEWHERE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLED_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STANDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEMPERATURE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_AFTER_THE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PARTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIGHT_CHEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOOTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WE_OUR_OURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALL_PART_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALSO_THE_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_IMPLANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ARTHRITIS_CURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ARTHRITIS_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BECOMING_A_DENTIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHECK_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CROWN_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FILLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HEALING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HYGIENIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JOINT_OF_MEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LENGTH_OF_A_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LIST_THAT_IS_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PART_IN_THE_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PART_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PRACTICE_ERRAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_REST_NOT_A_BONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RETURN_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_ROOT_CANAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THING_THAT_IS_NOT_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERB_TOOK_THE_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WHILE_AGO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_AFTER_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_BEFORE_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_CLAUSE_JOIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_DESCRIBED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_IS_A_NOUN_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_IS_A_RETURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_TAKING_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_VERB_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAD_PART_MOVEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAD_PART_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEARD_TRIM_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEARD_TRIM_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BLADE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CANNOT_USE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARRIES_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARTILAGE_PIERCING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHECK_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHICKENPOX_TIMELINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECISION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECISION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRUGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FALL_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEVER_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GONE_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAL_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAL_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAT_OR_ICE_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IT_HAPPENED_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEPT_AWAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIDNEY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOLE_CHANGED_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOLE_CLEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOLE_ORDINARY_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOLE_REMOVE_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSCLE_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEW_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AN_NSAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_BODY_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TAMPON_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_ON_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_TABLET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OFF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ON_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OTHER_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON_OWNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PIERCING_BUMP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PIERCING_NOT_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PIERCING_PIECE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PIERCING_ROUTINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PIERCING_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POSSESSIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANCY_SWELLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANCY_SWELLING_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANCY_SWELLING_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANCY_SWELLING_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANCY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PROTECT_ALLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PROTECT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PROTECT_FILLERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAZOR_BLADE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAZOR_BURN_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REGISTERING_A_DENTIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPORTED_COMPLAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHAVE_BURN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHAVE_FIRST_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SORENESS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SORE_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPORT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STANDING_KNEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STANDING_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STRAIGHT_SHAVE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SWELLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAKE_A_TEMPERATURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAMPON_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_BACK_IS_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_BOOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_BOOKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_PERSONS_OWN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_QUEUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_VISIT_IN_ORDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TO_HURT_A_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAX_FIRST_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKS_ALONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKS_DRILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORKSHOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORKSHOP_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.body: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ACHE_WORDS() -> Py { ensure_init(); G_ACHE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACHE_WORDS(v: Py) { ensure_init(); G_ACHE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_AGAIN() -> Py { ensure_init(); G_ASKED_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_AGAIN(v: Py) { ensure_init(); G_ASKED_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_ABOUT_A_PART() -> Py { ensure_init(); G_ASKING_ABOUT_A_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_ABOUT_A_PART(v: Py) { ensure_init(); G_ASKING_ABOUT_A_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_ABOUT() -> Py { ensure_init(); G_ASK_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_ABOUT(v: Py) { ensure_init(); G_ASK_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_YOUR_SHOULDER() -> Py { ensure_init(); G_AT_YOUR_SHOULDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_YOUR_SHOULDER(v: Py) { ensure_init(); G_AT_YOUR_SHOULDER.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_YOUR_SHOULDER_REPLY() -> Py { ensure_init(); G_AT_YOUR_SHOULDER_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_YOUR_SHOULDER_REPLY(v: Py) { ensure_init(); G_AT_YOUR_SHOULDER_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_AVOID() -> Py { ensure_init(); G_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AVOID(v: Py) { ensure_init(); G_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PART_NO_KNOCK_TEST() -> Py { ensure_init(); G_A_PART_NO_KNOCK_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PART_NO_KNOCK_TEST(v: Py) { ensure_init(); G_A_PART_NO_KNOCK_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_NAMES() -> Py { ensure_init(); G_BACK_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_NAMES(v: Py) { ensure_init(); G_BACK_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_PREGNANCY_KEEP() -> Py { ensure_init(); G_BACK_PREGNANCY_KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_PREGNANCY_KEEP(v: Py) { ensure_init(); G_BACK_PREGNANCY_KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_WENT() -> Py { ensure_init(); G_BACK_WENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_WENT(v: Py) { ensure_init(); G_BACK_WENT.with(|c| *c.borrow_mut() = v); }
pub fn g_BREAK_SAID() -> Py { ensure_init(); G_BREAK_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREAK_SAID(v: Py) { ensure_init(); G_BREAK_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_BROKEN_PARTS() -> Py { ensure_init(); G_BROKEN_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROKEN_PARTS(v: Py) { ensure_init(); G_BROKEN_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_BROKEN_WORDS() -> Py { ensure_init(); G_BROKEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROKEN_WORDS(v: Py) { ensure_init(); G_BROKEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOT_AND_CHEST() -> Py { ensure_init(); G_CLOT_AND_CHEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOT_AND_CHEST(v: Py) { ensure_init(); G_CLOT_AND_CHEST.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPRESS_WORDS() -> Py { ensure_init(); G_COMPRESS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPRESS_WORDS(v: Py) { ensure_init(); G_COMPRESS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDE() -> Py { ensure_init(); G_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDE(v: Py) { ensure_init(); G_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDE_ASK() -> Py { ensure_init(); G_DECIDE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDE_ASK(v: Py) { ensure_init(); G_DECIDE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EXERCISE_AFTER() -> Py { ensure_init(); G_EXERCISE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXERCISE_AFTER(v: Py) { ensure_init(); G_EXERCISE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_EXERCISE_DAY() -> Py { ensure_init(); G_EXERCISE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXERCISE_DAY(v: Py) { ensure_init(); G_EXERCISE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_EXTRA_AVOID() -> Py { ensure_init(); G_EXTRA_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTRA_AVOID(v: Py) { ensure_init(); G_EXTRA_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_FALL() -> Py { ensure_init(); G_FALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FALL(v: Py) { ensure_init(); G_FALL.with(|c| *c.borrow_mut() = v); }
pub fn g_FLAG_WITH_IT() -> Py { ensure_init(); G_FLAG_WITH_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLAG_WITH_IT(v: Py) { ensure_init(); G_FLAG_WITH_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOT_PARTS() -> Py { ensure_init(); G_FOOT_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOT_PARTS(v: Py) { ensure_init(); G_FOOT_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_FOUR_STEPS_REPLY() -> Py { ensure_init(); G_FOUR_STEPS_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOUR_STEPS_REPLY(v: Py) { ensure_init(); G_FOUR_STEPS_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_HAND_PARTS() -> Py { ensure_init(); G_HAND_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAND_PARTS(v: Py) { ensure_init(); G_HAND_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT_FLAGS() -> Py { ensure_init(); G_HOT_FLAGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT_FLAGS(v: Py) { ensure_init(); G_HOT_FLAGS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT_SWELLING() -> Py { ensure_init(); G_HOT_SWELLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT_SWELLING(v: Py) { ensure_init(); G_HOT_SWELLING.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT() -> Py { ensure_init(); G_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT(v: Py) { ensure_init(); G_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT_WORDS() -> Py { ensure_init(); G_HURT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT_WORDS(v: Py) { ensure_init(); G_HURT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_YOUR_HANDS() -> Py { ensure_init(); G_IN_YOUR_HANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_YOUR_HANDS(v: Py) { ensure_init(); G_IN_YOUR_HANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_YOUR_HANDS_REPLY() -> Py { ensure_init(); G_IN_YOUR_HANDS_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_YOUR_HANDS_REPLY(v: Py) { ensure_init(); G_IN_YOUR_HANDS_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINTS() -> Py { ensure_init(); G_JOINTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINTS(v: Py) { ensure_init(); G_JOINTS.with(|c| *c.borrow_mut() = v); }
pub fn g_KNEE_WORDS() -> Py { ensure_init(); G_KNEE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNEE_WORDS(v: Py) { ensure_init(); G_KNEE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LOWER_HALF() -> Py { ensure_init(); G_LOWER_HALF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOWER_HALF(v: Py) { ensure_init(); G_LOWER_HALF.with(|c| *c.borrow_mut() = v); }
pub fn g_MEDICINE() -> Py { ensure_init(); G_MEDICINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEDICINE(v: Py) { ensure_init(); G_MEDICINE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOLE_CHANGE() -> Py { ensure_init(); G_MOLE_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOLE_CHANGE(v: Py) { ensure_init(); G_MOLE_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOLE_HAPPENING() -> Py { ensure_init(); G_MOLE_HAPPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOLE_HAPPENING(v: Py) { ensure_init(); G_MOLE_HAPPENING.with(|c| *c.borrow_mut() = v); }
pub fn g_MOLE_REMOVE() -> Py { ensure_init(); G_MOLE_REMOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOLE_REMOVE(v: Py) { ensure_init(); G_MOLE_REMOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMB_ANYWHERE() -> Py { ensure_init(); G_NUMB_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMB_ANYWHERE(v: Py) { ensure_init(); G_NUMB_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMB_ASK() -> Py { ensure_init(); G_NUMB_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMB_ASK(v: Py) { ensure_init(); G_NUMB_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMB_AVOID() -> Py { ensure_init(); G_NUMB_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMB_AVOID(v: Py) { ensure_init(); G_NUMB_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMB_FOOT() -> Py { ensure_init(); G_NUMB_FOOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMB_FOOT(v: Py) { ensure_init(); G_NUMB_FOOT.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMB_HANDS() -> Py { ensure_init(); G_NUMB_HANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMB_HANDS(v: Py) { ensure_init(); G_NUMB_HANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMB_LOWER() -> Py { ensure_init(); G_NUMB_LOWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMB_LOWER(v: Py) { ensure_init(); G_NUMB_LOWER.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMB_SHOE() -> Py { ensure_init(); G_NUMB_SHOE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMB_SHOE(v: Py) { ensure_init(); G_NUMB_SHOE.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECT_OWNERS() -> Py { ensure_init(); G_OBJECT_OWNERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECT_OWNERS(v: Py) { ensure_init(); G_OBJECT_OWNERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_YOUR_FEET() -> Py { ensure_init(); G_ON_YOUR_FEET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_YOUR_FEET(v: Py) { ensure_init(); G_ON_YOUR_FEET.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PEOPLE() -> Py { ensure_init(); G_OTHER_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PEOPLE(v: Py) { ensure_init(); G_OTHER_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAINKILLER_NAMES() -> Py { ensure_init(); G_PAINKILLER_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAINKILLER_NAMES(v: Py) { ensure_init(); G_PAINKILLER_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_ONE() -> Py { ensure_init(); G_PART_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_ONE(v: Py) { ensure_init(); G_PART_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_NOUNS() -> Py { ensure_init(); G_PERSON_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_NOUNS(v: Py) { ensure_init(); G_PERSON_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE_OF_A_PERSON() -> Py { ensure_init(); G_POSSESSIVE_OF_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE_OF_A_PERSON(v: Py) { ensure_init(); G_POSSESSIVE_OF_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT() -> Py { ensure_init(); G_PREGNANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT(v: Py) { ensure_init(); G_PREGNANT.with(|c| *c.borrow_mut() = v); }
pub fn g_RED_FLAG() -> Py { ensure_init(); G_RED_FLAG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RED_FLAG(v: Py) { ensure_init(); G_RED_FLAG.with(|c| *c.borrow_mut() = v); }
pub fn g_RED_FLAG_CARD() -> Py { ensure_init(); G_RED_FLAG_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RED_FLAG_CARD(v: Py) { ensure_init(); G_RED_FLAG_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_REGIONS() -> Py { ensure_init(); G_REGIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGIONS(v: Py) { ensure_init(); G_REGIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_REGION_BIAS() -> Py { ensure_init(); G_REGION_BIAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGION_BIAS(v: Py) { ensure_init(); G_REGION_BIAS.with(|c| *c.borrow_mut() = v); }
pub fn g_RING_AS_A_CALL() -> Py { ensure_init(); G_RING_AS_A_CALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RING_AS_A_CALL(v: Py) { ensure_init(); G_RING_AS_A_CALL.with(|c| *c.borrow_mut() = v); }
pub fn g_RING_NOT_ON_A_FINGER() -> Py { ensure_init(); G_RING_NOT_ON_A_FINGER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RING_NOT_ON_A_FINGER(v: Py) { ensure_init(); G_RING_NOT_ON_A_FINGER.with(|c| *c.borrow_mut() = v); }
pub fn g_SADDLE_CARD() -> Py { ensure_init(); G_SADDLE_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SADDLE_CARD(v: Py) { ensure_init(); G_SADDLE_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_SADDLE_PARTS() -> Py { ensure_init(); G_SADDLE_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SADDLE_PARTS(v: Py) { ensure_init(); G_SADDLE_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SEA_WATER_ALARM() -> Py { ensure_init(); G_SEA_WATER_ALARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEA_WATER_ALARM(v: Py) { ensure_init(); G_SEA_WATER_ALARM.with(|c| *c.borrow_mut() = v); }
pub fn g_SEA_WATER_ALARM_REPLY() -> Py { ensure_init(); G_SEA_WATER_ALARM_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEA_WATER_ALARM_REPLY(v: Py) { ensure_init(); G_SEA_WATER_ALARM_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_SEEN_ABOUT_IT() -> Py { ensure_init(); G_SEEN_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEEN_ABOUT_IT(v: Py) { ensure_init(); G_SEEN_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_SEE_DOCTOR_AVOID() -> Py { ensure_init(); G_SEE_DOCTOR_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEE_DOCTOR_AVOID(v: Py) { ensure_init(); G_SEE_DOCTOR_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_SEE_DOCTOR_PARTS() -> Py { ensure_init(); G_SEE_DOCTOR_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEE_DOCTOR_PARTS(v: Py) { ensure_init(); G_SEE_DOCTOR_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SEE_TODAY_BACK_AND_CALF() -> Py { ensure_init(); G_SEE_TODAY_BACK_AND_CALF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEE_TODAY_BACK_AND_CALF(v: Py) { ensure_init(); G_SEE_TODAY_BACK_AND_CALF.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_TISSUE() -> Py { ensure_init(); G_SOFT_TISSUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_TISSUE(v: Py) { ensure_init(); G_SOFT_TISSUE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEWHERE_ELSE() -> Py { ensure_init(); G_SOMEWHERE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEWHERE_ELSE(v: Py) { ensure_init(); G_SOMEWHERE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLED_OUT() -> Py { ensure_init(); G_SPELLED_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLED_OUT(v: Py) { ensure_init(); G_SPELLED_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_STANDING() -> Py { ensure_init(); G_STANDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STANDING(v: Py) { ensure_init(); G_STANDING.with(|c| *c.borrow_mut() = v); }
pub fn g_TEMPERATURE_WORDS() -> Py { ensure_init(); G_TEMPERATURE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEMPERATURE_WORDS(v: Py) { ensure_init(); G_TEMPERATURE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_AFTER_THE_WORD() -> Py { ensure_init(); G_THING_AFTER_THE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_AFTER_THE_WORD(v: Py) { ensure_init(); G_THING_AFTER_THE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_PARTS() -> Py { ensure_init(); G_THING_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_PARTS(v: Py) { ensure_init(); G_THING_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PARTY() -> Py { ensure_init(); G_THIRD_PARTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PARTY(v: Py) { ensure_init(); G_THIRD_PARTY.with(|c| *c.borrow_mut() = v); }
pub fn g_TIGHT_CHEST() -> Py { ensure_init(); G_TIGHT_CHEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIGHT_CHEST(v: Py) { ensure_init(); G_TIGHT_CHEST.with(|c| *c.borrow_mut() = v); }
pub fn g_TOOTH_WORDS() -> Py { ensure_init(); G_TOOTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOOTH_WORDS(v: Py) { ensure_init(); G_TOOTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKS() -> Py { ensure_init(); G_WEEKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKS(v: Py) { ensure_init(); G_WEEKS.with(|c| *c.borrow_mut() = v); }
pub fn g_WE_OUR_OURS() -> Py { ensure_init(); G_WE_OUR_OURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WE_OUR_OURS(v: Py) { ensure_init(); G_WE_OUR_OURS.with(|c| *c.borrow_mut() = v); }
pub fn g__ALL_PART_WORDS() -> Py { ensure_init(); G__ALL_PART_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALL_PART_WORDS(v: Py) { ensure_init(); G__ALL_PART_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__ALSO_THE_BACK() -> Py { ensure_init(); G__ALSO_THE_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALSO_THE_BACK(v: Py) { ensure_init(); G__ALSO_THE_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_IMPLANT() -> Py { ensure_init(); G__AN_IMPLANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_IMPLANT(v: Py) { ensure_init(); G__AN_IMPLANT.with(|c| *c.borrow_mut() = v); }
pub fn g__ARTHRITIS_CURE() -> Py { ensure_init(); G__ARTHRITIS_CURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ARTHRITIS_CURE(v: Py) { ensure_init(); G__ARTHRITIS_CURE.with(|c| *c.borrow_mut() = v); }
pub fn g__ARTHRITIS_TAIL() -> Py { ensure_init(); G__ARTHRITIS_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ARTHRITIS_TAIL(v: Py) { ensure_init(); G__ARTHRITIS_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BECOMING_A_DENTIST() -> Py { ensure_init(); G__A_BECOMING_A_DENTIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BECOMING_A_DENTIST(v: Py) { ensure_init(); G__A_BECOMING_A_DENTIST.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHECK_UP() -> Py { ensure_init(); G__A_CHECK_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHECK_UP(v: Py) { ensure_init(); G__A_CHECK_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CROWN_VISIT() -> Py { ensure_init(); G__A_CROWN_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CROWN_VISIT(v: Py) { ensure_init(); G__A_CROWN_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FILLING() -> Py { ensure_init(); G__A_FILLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FILLING(v: Py) { ensure_init(); G__A_FILLING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HEALING_WORD() -> Py { ensure_init(); G__A_HEALING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HEALING_WORD(v: Py) { ensure_init(); G__A_HEALING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HYGIENIST() -> Py { ensure_init(); G__A_HYGIENIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HYGIENIST(v: Py) { ensure_init(); G__A_HYGIENIST.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JOINT_OF_MEAT() -> Py { ensure_init(); G__A_JOINT_OF_MEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JOINT_OF_MEAT(v: Py) { ensure_init(); G__A_JOINT_OF_MEAT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LENGTH_OF_A_VISIT() -> Py { ensure_init(); G__A_LENGTH_OF_A_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LENGTH_OF_A_VISIT(v: Py) { ensure_init(); G__A_LENGTH_OF_A_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LIST_THAT_IS_LONG() -> Py { ensure_init(); G__A_LIST_THAT_IS_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LIST_THAT_IS_LONG(v: Py) { ensure_init(); G__A_LIST_THAT_IS_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PART_IN_THE_TABLE() -> Py { ensure_init(); G__A_PART_IN_THE_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PART_IN_THE_TABLE(v: Py) { ensure_init(); G__A_PART_IN_THE_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PART_NAMED() -> Py { ensure_init(); G__A_PART_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PART_NAMED(v: Py) { ensure_init(); G__A_PART_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PRACTICE_ERRAND() -> Py { ensure_init(); G__A_PRACTICE_ERRAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PRACTICE_ERRAND(v: Py) { ensure_init(); G__A_PRACTICE_ERRAND.with(|c| *c.borrow_mut() = v); }
pub fn g__A_REST_NOT_A_BONE() -> Py { ensure_init(); G__A_REST_NOT_A_BONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_REST_NOT_A_BONE(v: Py) { ensure_init(); G__A_REST_NOT_A_BONE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RETURN_TO() -> Py { ensure_init(); G__A_RETURN_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RETURN_TO(v: Py) { ensure_init(); G__A_RETURN_TO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_ROOT_CANAL() -> Py { ensure_init(); G__A_ROOT_CANAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_ROOT_CANAL(v: Py) { ensure_init(); G__A_ROOT_CANAL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THING_THAT_IS_NOT_A_BODY() -> Py { ensure_init(); G__A_THING_THAT_IS_NOT_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THING_THAT_IS_NOT_A_BODY(v: Py) { ensure_init(); G__A_THING_THAT_IS_NOT_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERB_TOOK_THE_BACK() -> Py { ensure_init(); G__A_VERB_TOOK_THE_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERB_TOOK_THE_BACK(v: Py) { ensure_init(); G__A_VERB_TOOK_THE_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WHILE_AGO() -> Py { ensure_init(); G__A_WHILE_AGO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WHILE_AGO(v: Py) { ensure_init(); G__A_WHILE_AGO.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_AFTER_A_VERB() -> Py { ensure_init(); G__BACK_AFTER_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_AFTER_A_VERB(v: Py) { ensure_init(); G__BACK_AFTER_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_BEFORE_A_PLACE() -> Py { ensure_init(); G__BACK_BEFORE_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_BEFORE_A_PLACE(v: Py) { ensure_init(); G__BACK_BEFORE_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_CLAUSE_JOIN() -> Py { ensure_init(); G__BACK_CLAUSE_JOIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_CLAUSE_JOIN(v: Py) { ensure_init(); G__BACK_CLAUSE_JOIN.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_DESCRIBED() -> Py { ensure_init(); G__BACK_DESCRIBED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_DESCRIBED(v: Py) { ensure_init(); G__BACK_DESCRIBED.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_HOME() -> Py { ensure_init(); G__BACK_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_HOME(v: Py) { ensure_init(); G__BACK_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_IS_A_NOUN_AFTER() -> Py { ensure_init(); G__BACK_IS_A_NOUN_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_IS_A_NOUN_AFTER(v: Py) { ensure_init(); G__BACK_IS_A_NOUN_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_IS_A_RETURN() -> Py { ensure_init(); G__BACK_IS_A_RETURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_IS_A_RETURN(v: Py) { ensure_init(); G__BACK_IS_A_RETURN.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_TAKING_A_THING() -> Py { ensure_init(); G__BACK_TAKING_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_TAKING_A_THING(v: Py) { ensure_init(); G__BACK_TAKING_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_THEIRS() -> Py { ensure_init(); G__BACK_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_THEIRS(v: Py) { ensure_init(); G__BACK_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_VERB_BEFORE() -> Py { ensure_init(); G__BACK_VERB_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_VERB_BEFORE(v: Py) { ensure_init(); G__BACK_VERB_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__BAD_PART_MOVEMENT() -> Py { ensure_init(); G__BAD_PART_MOVEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAD_PART_MOVEMENT(v: Py) { ensure_init(); G__BAD_PART_MOVEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__BAD_PART_WORDS() -> Py { ensure_init(); G__BAD_PART_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAD_PART_WORDS(v: Py) { ensure_init(); G__BAD_PART_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__BEARD_TRIM_REPLY() -> Py { ensure_init(); G__BEARD_TRIM_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEARD_TRIM_REPLY(v: Py) { ensure_init(); G__BEARD_TRIM_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__BEARD_TRIM_WORDS() -> Py { ensure_init(); G__BEARD_TRIM_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEARD_TRIM_WORDS(v: Py) { ensure_init(); G__BEARD_TRIM_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__BLADE_WORDS() -> Py { ensure_init(); G__BLADE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BLADE_WORDS(v: Py) { ensure_init(); G__BLADE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CANNOT_USE_IT() -> Py { ensure_init(); G__CANNOT_USE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CANNOT_USE_IT(v: Py) { ensure_init(); G__CANNOT_USE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__CARRIES_WEIGHT() -> Py { ensure_init(); G__CARRIES_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARRIES_WEIGHT(v: Py) { ensure_init(); G__CARRIES_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__CARTILAGE_PIERCING() -> Py { ensure_init(); G__CARTILAGE_PIERCING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARTILAGE_PIERCING(v: Py) { ensure_init(); G__CARTILAGE_PIERCING.with(|c| *c.borrow_mut() = v); }
pub fn g__CHECK_HOW_TO() -> Py { ensure_init(); G__CHECK_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHECK_HOW_TO(v: Py) { ensure_init(); G__CHECK_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g__CHICKENPOX_TIMELINE() -> Py { ensure_init(); G__CHICKENPOX_TIMELINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHICKENPOX_TIMELINE(v: Py) { ensure_init(); G__CHICKENPOX_TIMELINE.with(|c| *c.borrow_mut() = v); }
pub fn g__DECISION_ASK() -> Py { ensure_init(); G__DECISION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECISION_ASK(v: Py) { ensure_init(); G__DECISION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__DECISION_WORDS() -> Py { ensure_init(); G__DECISION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECISION_WORDS(v: Py) { ensure_init(); G__DECISION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DETERMINERS() -> Py { ensure_init(); G__DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DETERMINERS(v: Py) { ensure_init(); G__DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g__DRUGS() -> Py { ensure_init(); G__DRUGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRUGS(v: Py) { ensure_init(); G__DRUGS.with(|c| *c.borrow_mut() = v); }
pub fn g__FALL_AVOID() -> Py { ensure_init(); G__FALL_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FALL_AVOID(v: Py) { ensure_init(); G__FALL_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__FEVER_ADVICE() -> Py { ensure_init(); G__FEVER_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEVER_ADVICE(v: Py) { ensure_init(); G__FEVER_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g__GONE_BACK() -> Py { ensure_init(); G__GONE_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GONE_BACK(v: Py) { ensure_init(); G__GONE_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAL_PARTS() -> Py { ensure_init(); G__HEAL_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAL_PARTS(v: Py) { ensure_init(); G__HEAL_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAL_TIME() -> Py { ensure_init(); G__HEAL_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAL_TIME(v: Py) { ensure_init(); G__HEAL_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAT_OR_ICE_THE_THING() -> Py { ensure_init(); G__HEAT_OR_ICE_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAT_OR_ICE_THE_THING(v: Py) { ensure_init(); G__HEAT_OR_ICE_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__IT_HAPPENED_TO() -> Py { ensure_init(); G__IT_HAPPENED_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IT_HAPPENED_TO(v: Py) { ensure_init(); G__IT_HAPPENED_TO.with(|c| *c.borrow_mut() = v); }
pub fn g__KEPT_AWAKE() -> Py { ensure_init(); G__KEPT_AWAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEPT_AWAKE(v: Py) { ensure_init(); G__KEPT_AWAKE.with(|c| *c.borrow_mut() = v); }
pub fn g__KIDNEY_WORDS() -> Py { ensure_init(); G__KIDNEY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIDNEY_WORDS(v: Py) { ensure_init(); G__KIDNEY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__MOLE_CHANGED_TEXT() -> Py { ensure_init(); G__MOLE_CHANGED_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOLE_CHANGED_TEXT(v: Py) { ensure_init(); G__MOLE_CHANGED_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__MOLE_CLEAR() -> Py { ensure_init(); G__MOLE_CLEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOLE_CLEAR(v: Py) { ensure_init(); G__MOLE_CLEAR.with(|c| *c.borrow_mut() = v); }
pub fn g__MOLE_ORDINARY_TEXT() -> Py { ensure_init(); G__MOLE_ORDINARY_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOLE_ORDINARY_TEXT(v: Py) { ensure_init(); G__MOLE_ORDINARY_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__MOLE_REMOVE_TEXT() -> Py { ensure_init(); G__MOLE_REMOVE_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOLE_REMOVE_TEXT(v: Py) { ensure_init(); G__MOLE_REMOVE_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSCLE_PARTS() -> Py { ensure_init(); G__MUSCLE_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSCLE_PARTS(v: Py) { ensure_init(); G__MUSCLE_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g__NEW_SUBJECT() -> Py { ensure_init(); G__NEW_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEW_SUBJECT(v: Py) { ensure_init(); G__NEW_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AN_NSAID() -> Py { ensure_init(); G__NOT_AN_NSAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AN_NSAID(v: Py) { ensure_init(); G__NOT_AN_NSAID.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_BODY_PART() -> Py { ensure_init(); G__NOT_A_BODY_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BODY_PART(v: Py) { ensure_init(); G__NOT_A_BODY_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TAMPON_LINE() -> Py { ensure_init(); G__NOT_A_TAMPON_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TAMPON_LINE(v: Py) { ensure_init(); G__NOT_A_TAMPON_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_ON_A_BODY() -> Py { ensure_init(); G__NOT_ON_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_ON_A_BODY(v: Py) { ensure_init(); G__NOT_ON_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_TABLET() -> Py { ensure_init(); G__NOT_THE_TABLET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_TABLET(v: Py) { ensure_init(); G__NOT_THE_TABLET.with(|c| *c.borrow_mut() = v); }
pub fn g__OFF_A_THING() -> Py { ensure_init(); G__OFF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OFF_A_THING(v: Py) { ensure_init(); G__OFF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__ON_A_BODY() -> Py { ensure_init(); G__ON_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ON_A_BODY(v: Py) { ensure_init(); G__ON_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g__OTHER_PEOPLE() -> Py { ensure_init(); G__OTHER_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OTHER_PEOPLE(v: Py) { ensure_init(); G__OTHER_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PARTS() -> Py { ensure_init(); G__PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARTS(v: Py) { ensure_init(); G__PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON_OWNER() -> Py { ensure_init(); G__PERSON_OWNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON_OWNER(v: Py) { ensure_init(); G__PERSON_OWNER.with(|c| *c.borrow_mut() = v); }
pub fn g__PIERCING_BUMP() -> Py { ensure_init(); G__PIERCING_BUMP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PIERCING_BUMP(v: Py) { ensure_init(); G__PIERCING_BUMP.with(|c| *c.borrow_mut() = v); }
pub fn g__PIERCING_NOT_TROUBLE() -> Py { ensure_init(); G__PIERCING_NOT_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PIERCING_NOT_TROUBLE(v: Py) { ensure_init(); G__PIERCING_NOT_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PIERCING_PIECE() -> Py { ensure_init(); G__PIERCING_PIECE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PIERCING_PIECE(v: Py) { ensure_init(); G__PIERCING_PIECE.with(|c| *c.borrow_mut() = v); }
pub fn g__PIERCING_ROUTINE() -> Py { ensure_init(); G__PIERCING_ROUTINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PIERCING_ROUTINE(v: Py) { ensure_init(); G__PIERCING_ROUTINE.with(|c| *c.borrow_mut() = v); }
pub fn g__PIERCING_TROUBLE() -> Py { ensure_init(); G__PIERCING_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PIERCING_TROUBLE(v: Py) { ensure_init(); G__PIERCING_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g__POSSESSIVES() -> Py { ensure_init(); G__POSSESSIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POSSESSIVES(v: Py) { ensure_init(); G__POSSESSIVES.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANCY_SWELLING() -> Py { ensure_init(); G__PREGNANCY_SWELLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANCY_SWELLING(v: Py) { ensure_init(); G__PREGNANCY_SWELLING.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANCY_SWELLING_AVOID() -> Py { ensure_init(); G__PREGNANCY_SWELLING_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANCY_SWELLING_AVOID(v: Py) { ensure_init(); G__PREGNANCY_SWELLING_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANCY_SWELLING_PARTS() -> Py { ensure_init(); G__PREGNANCY_SWELLING_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANCY_SWELLING_PARTS(v: Py) { ensure_init(); G__PREGNANCY_SWELLING_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANCY_SWELLING_REPLY() -> Py { ensure_init(); G__PREGNANCY_SWELLING_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANCY_SWELLING_REPLY(v: Py) { ensure_init(); G__PREGNANCY_SWELLING_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANCY_WORDS() -> Py { ensure_init(); G__PREGNANCY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANCY_WORDS(v: Py) { ensure_init(); G__PREGNANCY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANT_WORDS() -> Py { ensure_init(); G__PREGNANT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANT_WORDS(v: Py) { ensure_init(); G__PREGNANT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PROTECT_ALLOWED() -> Py { ensure_init(); G__PROTECT_ALLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PROTECT_ALLOWED(v: Py) { ensure_init(); G__PROTECT_ALLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g__PROTECT_ASK() -> Py { ensure_init(); G__PROTECT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PROTECT_ASK(v: Py) { ensure_init(); G__PROTECT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PROTECT_FILLERS() -> Py { ensure_init(); G__PROTECT_FILLERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PROTECT_FILLERS(v: Py) { ensure_init(); G__PROTECT_FILLERS.with(|c| *c.borrow_mut() = v); }
pub fn g__RAZOR_BLADE_REPLY() -> Py { ensure_init(); G__RAZOR_BLADE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAZOR_BLADE_REPLY(v: Py) { ensure_init(); G__RAZOR_BLADE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__RAZOR_BURN_REPLY() -> Py { ensure_init(); G__RAZOR_BURN_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAZOR_BURN_REPLY(v: Py) { ensure_init(); G__RAZOR_BURN_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__REGISTERING_A_DENTIST() -> Py { ensure_init(); G__REGISTERING_A_DENTIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REGISTERING_A_DENTIST(v: Py) { ensure_init(); G__REGISTERING_A_DENTIST.with(|c| *c.borrow_mut() = v); }
pub fn g__REPORTED_COMPLAINT() -> Py { ensure_init(); G__REPORTED_COMPLAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPORTED_COMPLAINT(v: Py) { ensure_init(); G__REPORTED_COMPLAINT.with(|c| *c.borrow_mut() = v); }
pub fn g__SHAVE_BURN_WORDS() -> Py { ensure_init(); G__SHAVE_BURN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHAVE_BURN_WORDS(v: Py) { ensure_init(); G__SHAVE_BURN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SHAVE_FIRST_REPLY() -> Py { ensure_init(); G__SHAVE_FIRST_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHAVE_FIRST_REPLY(v: Py) { ensure_init(); G__SHAVE_FIRST_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSE() -> Py { ensure_init(); G__SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE(v: Py) { ensure_init(); G__SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SORENESS_WORDS() -> Py { ensure_init(); G__SORENESS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SORENESS_WORDS(v: Py) { ensure_init(); G__SORENESS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SORE_ANYWHERE() -> Py { ensure_init(); G__SORE_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SORE_ANYWHERE(v: Py) { ensure_init(); G__SORE_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__SPORT_WORDS() -> Py { ensure_init(); G__SPORT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPORT_WORDS(v: Py) { ensure_init(); G__SPORT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__STANDING_KNEE() -> Py { ensure_init(); G__STANDING_KNEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STANDING_KNEE(v: Py) { ensure_init(); G__STANDING_KNEE.with(|c| *c.borrow_mut() = v); }
pub fn g__STANDING_TROUBLE() -> Py { ensure_init(); G__STANDING_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STANDING_TROUBLE(v: Py) { ensure_init(); G__STANDING_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g__STRAIGHT_SHAVE_REPLY() -> Py { ensure_init(); G__STRAIGHT_SHAVE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STRAIGHT_SHAVE_REPLY(v: Py) { ensure_init(); G__STRAIGHT_SHAVE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__SWELLING() -> Py { ensure_init(); G__SWELLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SWELLING(v: Py) { ensure_init(); G__SWELLING.with(|c| *c.borrow_mut() = v); }
pub fn g__TAKE_A_TEMPERATURE() -> Py { ensure_init(); G__TAKE_A_TEMPERATURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAKE_A_TEMPERATURE(v: Py) { ensure_init(); G__TAKE_A_TEMPERATURE.with(|c| *c.borrow_mut() = v); }
pub fn g__TAMPON_WORD() -> Py { ensure_init(); G__TAMPON_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAMPON_WORD(v: Py) { ensure_init(); G__TAMPON_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_BACK_IS_THEIRS() -> Py { ensure_init(); G__THE_BACK_IS_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_BACK_IS_THEIRS(v: Py) { ensure_init(); G__THE_BACK_IS_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_BOOKING() -> Py { ensure_init(); G__THE_BOOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_BOOKING(v: Py) { ensure_init(); G__THE_BOOKING.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_BOOKING_WORDS() -> Py { ensure_init(); G__THE_BOOKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_BOOKING_WORDS(v: Py) { ensure_init(); G__THE_BOOKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_PERSONS_OWN_WORDS() -> Py { ensure_init(); G__THE_PERSONS_OWN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_PERSONS_OWN_WORDS(v: Py) { ensure_init(); G__THE_PERSONS_OWN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_QUEUE() -> Py { ensure_init(); G__THE_QUEUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_QUEUE(v: Py) { ensure_init(); G__THE_QUEUE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_VISIT() -> Py { ensure_init(); G__THE_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_VISIT(v: Py) { ensure_init(); G__THE_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_VISIT_IN_ORDER() -> Py { ensure_init(); G__THE_VISIT_IN_ORDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_VISIT_IN_ORDER(v: Py) { ensure_init(); G__THE_VISIT_IN_ORDER.with(|c| *c.borrow_mut() = v); }
pub fn g__TO_HURT_A_PART() -> Py { ensure_init(); G__TO_HURT_A_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TO_HURT_A_PART(v: Py) { ensure_init(); G__TO_HURT_A_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__WAX_FIRST_REPLY() -> Py { ensure_init(); G__WAX_FIRST_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAX_FIRST_REPLY(v: Py) { ensure_init(); G__WAX_FIRST_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__WAX_WORDS() -> Py { ensure_init(); G__WAX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAX_WORDS(v: Py) { ensure_init(); G__WAX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKS_ALONG() -> Py { ensure_init(); G__WEEKS_ALONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKS_ALONG(v: Py) { ensure_init(); G__WEEKS_ALONG.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKS_DRILL() -> Py { ensure_init(); G__WEEKS_DRILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKS_DRILL(v: Py) { ensure_init(); G__WEEKS_DRILL.with(|c| *c.borrow_mut() = v); }
pub fn g__WORKSHOP() -> Py { ensure_init(); G__WORKSHOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORKSHOP(v: Py) { ensure_init(); G__WORKSHOP.with(|c| *c.borrow_mut() = v); }
pub fn g__WORKSHOP_NOUNS() -> Py { ensure_init(); G__WORKSHOP_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORKSHOP_NOUNS(v: Py) { ensure_init(); G__WORKSHOP_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A part of the body that hurts: the everyday ache people type as \"my <part> hurts\".\n\nPeople do not ask \"what is a sore back\", they type the complaint, so the key here is the\npart plus the word for hurting, never the complaint on its own. Each region carries the\none thing that actually helps it, because \"rest it and take a painkiller\" is the same\nsentence for a back and an ankle and would read as a shrug.\n\nThe region is data: one line per part in REGIONS, the shared words for hurting in HURT,\nand the parts this table must not answer in AVOID, so a sore chest (\"ring 999\"), a sore\nthroat, a sore tooth and a stomach ache keep the answers that already handle them.\n");
    s_ASK_ABOUT(pyrt::xrefs::xget("knowledge.lookup", "ASK_ABOUT"));
    s_RING_AS_A_CALL(pyrt::xrefs::xget("knowledge.lookup", "RING_AS_A_CALL"));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_OTHER_PEOPLE(pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE"));
    s_PERSON_NOUNS(pyrt::xrefs::xget("understanding.kinds", "PERSON_NOUNS"));
    s_TEMPERATURE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("temperature"), py_str("temperatures"), py_str("temp"), py_str("temps"), py_str("fever"), py_str("feverish"), py_str("fevered")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WE_OUR_OURS(call_builtin("frozenset", &[py_tuple(vec![py_str("we"), py_str("us"), py_str("our"), py_str("ours")])]).unwrap_or_else(|e| exc::raise(e)));
    s_POSSESSIVE_OF_A_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("your")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("probe"), py_str("probes"), py_str("thermometer"), py_str("thermometers"), py_str("gauge"), py_str("gauges"), py_str("gage"), py_str("gages"), py_str("sensor"), py_str("sensors"), py_str("dial"), py_str("dials"), py_str("knob"), py_str("knobs"), py_str("setting"), py_str("settings"), py_str("control"), py_str("controls"), py_str("thermostat"), py_str("thermostats"), py_str("stat")]) }
    s_THING_AFTER_THE_WORD(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("wood"), py_str("wooden"), py_str("plank"), py_str("planks"), py_str("planking"), py_str("joist"), py_str("joists"), py_str("batten"), py_str("battens"), py_str("floorboard"), py_str("floorboards"), py_str("skirting"), py_str("moulding"), py_str("molding"), py_str("fence"), py_str("fencing"), py_str("gate"), py_str("shed"), py_str("decking"), py_str("worktop"), py_str("shelf"), py_str("shelves"), py_str("shelving"), py_str("cabinet"), py_str("wardrobe"), py_str("wall"), py_str("walls"), py_str("brick"), py_str("bricks"), py_str("plasterboard"), py_str("stud wall"), py_str("stud"), py_str("studs"), py_str("mirror"), py_str("mirrors"), py_str("bracket"), py_str("brackets"), py_str("picture"), py_str("pictures"), py_str("concrete"), py_str("masonry"), py_str("tile"), py_str("tiles"), py_str("block wall"), py_str("stone wall"), py_str("curtain rail"), py_str("curtain pole"), py_str("anchor"), py_str("anchors"), py_str("hollow wall"), py_str("cavity wall"), py_str("picture hook"), py_str("picture hooks"), py_str("picture frame"), py_str("hanging a picture"), py_str("hang a picture"), py_str("hang the picture"), py_str("hammer"), py_str("hammers"), py_str("hammering"), py_str("drill"), py_str("drills"), py_str("drilling"), py_str("screw"), py_str("screws"), py_str("screwed"), py_str("screwing"), py_str("rawlplug"), py_str("rawlplugs"), py_str("wall plug"), py_str("wall plugs"), py_str("carpentry"), py_str("diy"), py_str("doorframe"), py_str("door frame"), py_str("tyre"), py_str("tyres"), py_str("tire"), py_str("tires"), py_str("wheel"), py_str("wheels"), py_str("puncture"), py_str("punctured")]) }
    s__WORKSHOP_NOUNS(__data2());
    s__WORKSHOP(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WORKSHOP_NOUNS()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__CANNOT_USE_IT(stdlib_call("re", "compile", &[py_str("\\b(?:can'?t|can ?not|couldn'?t|could ?not|unable to|not able to)\\s+(?:walk|stand|put|bear|weight|move|straighten|bend|use|lift|raise|reach)\\b|\\b(?:put|bear)\\s+(?:my\\s+)?weight\\b|\\b(?:gave|gives|giving) way\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_PERSONS_OWN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("me"), py_str("my"), py_str("mine"), py_str("myself"), py_str("ive"), py_str("i've")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("he"), py_str("hes"), py_str("he's"), py_str("she"), py_str("shes"), py_str("she's"), py_str("him"), py_str("her"), py_str("his"), py_str("hers"), py_str("they"), py_str("theirs"), py_str("their"), py_str("them")]) }
    s__SOMEONE_ELSE(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    s__WEEKS_ALONG(stdlib_call("re", "compile", &[py_str("\\b(?:im|i'?m|i am|am)\\s+(?:about\\s+|nearly\\s+|almost\\s+|just\\s+|only\\s+)?\\d{1,2}\\s*(?:weeks?|wks?)\\b|\\b\\d{1,2}\\s*(?:weeks?|wks?)\\s+(?:pregnant|gone|along|in)\\b|\\bweeks (?:gone|along)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SWELLING(stdlib_call("re", "compile", &[py_str("\\b(?:swollen|swelling|swelled|swell|swells|puffy|puffiness|water retention|retaining water|fluid)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PREGNANCY_WORDS(py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("weeks pregnant"), py_str("weeks gone"), py_str("trimester"), py_str("my bump"), py_str("having a baby"), py_str("midwife"), py_str("due in"), py_str("due date")]));
    s__PREGNANCY_SWELLING(py_tuple(vec![py_str("swollen"), py_str("swelling"), py_str("swelled"), py_str("swell up"), py_str("swells"), py_str("swell"), py_str("water retention"), py_str("retaining water")]));
    fn __data4() -> Py { py_tuple(vec![py_str("ankles"), py_str("ankle"), py_str("feet"), py_str("foot"), py_str("legs"), py_str("leg"), py_str("toes"), py_str("toe"), py_str("calves"), py_str("calf"), py_str("hands"), py_str("hand"), py_str("fingers"), py_str("finger")]) }
    s__PREGNANCY_SWELLING_PARTS(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("sprain"), py_str("sprained"), py_str("twisted"), py_str("broke"), py_str("broken"), py_str("fell"), py_str("fall"), py_str("bandage"), py_str("crutches"), py_str("ibuprofen"), py_str("paracetamol"), py_str("painkiller"), py_str("painkillers"), py_str("overdose"), py_str("took too much"), py_str("allergic"), py_str("dog"), py_str("cat"), py_str("are fine"), py_str("is fine"), py_str("not swollen"), py_str("no swelling"), py_str("isnt swollen"), py_str("arent swollen"), py_str("no swelling at all"), py_str("nothing swollen"), py_str("not pregnant"), py_str("isnt pregnant"), py_str("no longer pregnant"), py_str("not expecting"), py_str("not been pregnant")]) }
    s__PREGNANCY_SWELLING_AVOID(__data5());
    s__PREGNANCY_SWELLING_REPLY(py_str("Swollen feet and ankles are ordinary in the last months: the extra blood and the weight of the bump slow the fluid back up your legs. Put your feet up above hip height for twenty minutes a few times a day, keep walking, wear the support tights, and drink normally rather than less. Ring your midwife the same day if it comes on over a day or two, if one ankle or one leg swells on its own, if a leg is hot or sore, or if you have a bad headache, blurry vision or swelling in your face or hands."));
    fn __data6() -> Py { py_tuple(vec![py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("achy"), py_str("pain"), py_str("pains"), py_str("painful"), py_str("sore"), py_str("stiff"), py_str("killing me"), py_str("kills"), py_str("twinge"), py_str("twinges"), py_str("throbbing"), py_str("agony"), py_str("soreness"), py_str("thumping"), py_str("thumps"), py_str("pounding"), py_str("pounds"), py_str("playing up"), py_str("playing me up"), py_str("acts up"), py_str("acting up"), py_str("giving me jip"), py_str("giving me gyp"), py_str("giving me grief"), py_str("gives me jip"), py_str("gives me gyp"), py_str("gives me grief"), py_str("sprain"), py_str("sprained"), py_str("sprains"), py_str("strain"), py_str("strained"), py_str("strains"), py_str("twisted"), py_str("tweak"), py_str("tweaked"), py_str("pulled a muscle"), py_str("pulled my"), py_str("torn"), py_str("went over on"), py_str("rolled"), py_str("pulled"), py_str("tore"), py_str("ripped"), py_str("injured"), py_str("injury"), py_str("injuries"), py_str("done my"), py_str("did my"), py_str("done in"), py_str("did in"), py_str("put my back out"), py_str("put my shoulder out"), py_str("put my knee out"), py_str("a joke"), py_str("in bits"), py_str("wrecked"), py_str("banjaxed"), py_str("a nightmare"), py_str("torture"), py_str("is at me"), py_str("in a state"), py_str("a state"), py_str("hanging off me"), py_str("in agony"), py_str("giving me trouble"), py_str("because of my"), py_str("because of her"), py_str("because of his")]) }
    s_HURT(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my kids"), py_str("my baby"), py_str("my toddler"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my mom"), py_str("my mother"), py_str("my dad"), py_str("my father"), py_str("my friend"), py_str("my dog"), py_str("my cat"), py_str("my puppy"), py_str("my horse"), py_str("the dog"), py_str("the cat")]) }
    s__OTHER_PEOPLE(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("arm"), py_str("leg"), py_str("back"), py_str("knee"), py_str("foot"), py_str("feet"), py_str("hand"), py_str("shoulder"), py_str("neck"), py_str("hip"), py_str("ankle"), py_str("wrist"), py_str("elbow")]) }
    s__PARTS(__data8());
    s_THIRD_PARTY(op_add(&g__OTHER_PEOPLE(), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("his"), py_str("her")])) { let mut who: Py = __item.clone(); for __item in py_iter_or_raise(&g__PARTS()) { let mut part: Py = __item.clone(); __out.push(op_modulo(&py_str("%s %s"), &py_tuple(vec![who.clone(), part.clone()]))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    fn __data9() -> Py { py_tuple(vec![py_str("for a week"), py_str("for two weeks"), py_str("for a fortnight"), py_str("for weeks"), py_str("for a month"), py_str("for months"), py_str("for ages"), py_str("weeks"), py_str("months"), py_str("a week now"), py_str("two weeks"), py_str("three weeks"), py_str("a few weeks")]) }
    s_STANDING(__data9());
    s_FALL(py_tuple(vec![py_str("fall"), py_str("a fall"), py_str("after a fall"), py_str("from a fall"), py_str("banged"), py_str("knocked"), py_str("bump"), py_str("bumped")]));
    s_DECIDE(py_tuple(vec![py_str("see someone"), py_str("see a doctor"), py_str("see the doctor"), py_str("see a gp"), py_str("go to the doctor"), py_str("should i see"), py_str("do i need to see"), py_str("or wait"), py_str("see a physio"), py_str("worth seeing"), py_str("get it looked at"), py_str("should i get it checked")]));
    fn __data10() -> Py { py_tuple(vec![py_str("cant pee"), py_str("can't pee"), py_str("cannot pee"), py_str("cant wee"), py_str("can't wee"), py_str("cant pass urine"), py_str("can't pass urine"), py_str("cant pass water"), py_str("no urine"), py_str("cant urinate"), py_str("can't urinate"), py_str("cannot urinate"), py_str("numb"), py_str("numbness"), py_str("pins and needles"), py_str("cant feel my legs"), py_str("can't feel my legs"), py_str("cant feel my feet"), py_str("cant feel my foot"), py_str("legs give way"), py_str("leg gave way"), py_str("incontinent"), py_str("lost control of my bladder"), py_str("saddle"), py_str("hurts when i breathe"), py_str("hurts to breathe"), py_str("pain when i breathe"), py_str("worse at night"), py_str("wakes me at night"), py_str("wake me at night"), py_str("wakes me up"), py_str("woke me up"), py_str("hot and swollen"), py_str("swollen and hot"), py_str("swollen calf"), py_str("swollen leg"), py_str("hot calf"), py_str("red and hot"), py_str("after a flight"), py_str("after a long flight")]) }
    s_RED_FLAG(__data10());
    fn __data11() -> Py { py_tuple(vec![py_str("back"), py_str("leg"), py_str("legs"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("shin"), py_str("arm"), py_str("arms"), py_str("hip"), py_str("knee"), py_str("foot"), py_str("feet"), py_str("spine"), py_str("side"), py_str("shoulder"), py_str("neck"), py_str("ankle"), py_str("wrist"), py_str("elbow"), py_str("hand"), py_str("pain"), py_str("pains"), py_str("hurts"), py_str("hurting"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("achey"), py_str("ankles"), py_str("knees"), py_str("hands"), py_str("thighs"), py_str("shins"), py_str("hips"), py_str("wrists"), py_str("elbows"), py_str("shoulders"), py_str("toes"), py_str("fingers"), py_str("thumbs"), py_str("heels")]) }
    s__SORE_ANYWHERE(__data11());
    s_EXERCISE_DAY(py_tuple(vec![py_str("gym"), py_str("workout"), py_str("work out"), py_str("exercised"), py_str("weights"), py_str("next day"), py_str("day after")]));
    s_MEDICINE(py_tuple(vec![py_str("ibuprofen"), py_str("ibruprofen"), py_str("ibuprofin"), py_str("nurofen"), py_str("brufen"), py_str("painkiller"), py_str("painkillers"), py_str("pain killer"), py_str("pain killers")]));
    fn __data12() -> Py { py_tuple(vec![py_str("hand luggage"), py_str("hand bag"), py_str("handbag"), py_str("hand luggage only"), py_str("hand cream"), py_str("hand wash"), py_str("hand soap"), py_str("hand sanitiser"), py_str("hand sanitizer"), py_str("hand dryer"), py_str("hand towel"), py_str("hand brake"), py_str("hand held"), py_str("handheld"), py_str("first hand"), py_str("second hand"), py_str("secondhand"), py_str("second-hand"), py_str("on the other hand"), py_str("helping hand"), py_str("hand me"), py_str("hand in hand"), py_str("hand made"), py_str("handmade"), py_str("hand whisk"), py_str("hand mixer"), py_str("hand warmer"), py_str("hand rail"), py_str("handrail"), py_str("hand print"), py_str("handwriting"), py_str("handwriting"), py_str("hands on"), py_str("back garden"), py_str("back door"), py_str("back seat"), py_str("back pay"), py_str("back up"), py_str("cashback"), py_str("back to school"), py_str("back of the car"), py_str("back page"), py_str("back of the fridge"), py_str("back of my throat"), py_str("in the back of"), py_str("on foot"), py_str("foot of the stairs"), py_str("foot of the bed"), py_str("foot of the garden"), py_str("six foot"), py_str("five foot"), py_str("foot long"), py_str("at the foot of"), py_str("arm chair"), py_str("armchair"), py_str("arm of the sofa"), py_str("arm of the chair"), py_str("leg of the table"), py_str("leg of the chair"), py_str("table leg"), py_str("chair leg"), py_str("door handle"), py_str("knee deep"), py_str("hip flask")]) }
    s__NOT_A_BODY_PART(__data12());
    fn __data13() -> Py { py_tuple(vec![py_str("chest"), py_str("heart"), py_str("throat"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("ear"), py_str("ears"), py_str("stomach"), py_str("tummy"), py_str("belly"), py_str("period"), py_str("pregnant"), py_str("pregnancy"), py_str("jaw"), py_str("sick"), py_str("nauseous"), py_str("vomit"), py_str("dizzy"), py_str("faint"), py_str("breathless"), py_str("sweating"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("stink"), py_str("stinks"), py_str("stinking"), py_str("odour"), py_str("odor"), py_str("sweaty"), py_str("sweat"), py_str("mole"), py_str("moles"), py_str("rash"), py_str("rashes"), py_str("spot"), py_str("spots"), py_str("blister"), py_str("blisters"), py_str("wart"), py_str("warts"), py_str("bruise"), py_str("bruises"), py_str("birthmark"), py_str("ingrown"), py_str("athletes foot"), py_str("when its cold"), py_str("when it's cold"), py_str("when it is cold"), py_str("in the cold"), py_str("in this cold"), py_str("cold weather"), py_str("cold and damp"), py_str("when it rains"), py_str("when it rained"), py_str("in the rain"), py_str("when its damp"), py_str("when it's damp"), py_str("damp weather"), py_str("when the weather"), py_str("weather changes"), py_str("in the winter"), py_str("in winter"), py_str("before rain"), py_str("before it rains")]) }
    s_AVOID(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&__data13(), &g__NOT_A_BODY_PART()), &g_EXERCISE_DAY()), &g_STANDING()), &g_THIRD_PARTY()), &g_DECIDE()), &g_FALL()), &g_RED_FLAG()), &g_MEDICINE()));
    fn __data14() -> Py { py_tuple(vec![py_tuple(vec![py_str("back"), py_tuple(vec![py_str("back"), py_str("lower back"), py_str("spine"), py_str("backache")]), py_str("Most back pain is muscular and settles in two to six weeks, and staying active is what helps most: keep walking and carry on with your usual jobs as far as you can rather than taking to your bed, and take paracetamol or ibuprofen if you can. See a GP if it is no better in a fortnight, or today if you cannot pass urine, have numbness between your legs, or it came on after a fall.")]), py_tuple(vec![py_str("knee"), py_tuple(vec![py_str("knee"), py_str("kneecap")]), py_str("A sore knee is usually a strain or the kneecap tracking badly: ease off running, stairs and kneeling, ice it for 20 minutes at a time with a cloth round it for the first two days, then keep it moving gently. Get it looked at if it gives way, locks, or you cannot put your weight on it.")]), py_tuple(vec![py_str("shoulder"), py_tuple(vec![py_str("shoulder"), py_str("shoulders"), py_str("shoulder blade"), py_str("rotator cuff")]), py_str("Most shoulder pain is a strained or trapped tendon, worse when you reach overhead or lie on that side: keep using the arm as far as is comfortable, warm it before you use it, and do the pendulum swing (lean forward, let the arm hang, circle it slowly) a few times a day. See a GP if you cannot lift the arm at all or it started after a fall.")]), py_tuple(vec![py_str("neck"), py_tuple(vec![py_str("neck"), py_str("stiff neck")]), py_str("A stiff neck is usually muscular, from posture or sleeping awkwardly: heat, gentle stretches and a look at your screen height help, and a soft collar for a day or two is fine but not longer, because the muscles need to work. Get seen the same day if it comes with a fever, a bad headache, or light hurting your eyes.")]), py_tuple(vec![py_str("hip"), py_tuple(vec![py_str("hip"), py_str("hips")]), py_str("Hip pain in an adult is usually the joint or the tendon over the outside of the hip bone: keep walking as far as is comfortable, sit with your knees lower than your hips, and take a painkiller before a walk rather than after. See a GP if it hurts when you are resting or at night, or you have started to limp.")]), py_tuple(vec![py_str("ankle"), py_tuple(vec![py_str("ankle"), py_str("ankles")]), py_str("An ankle sprain wants ice for 20 minutes at a time for the first two days, a support wrap and the foot up when you sit, then gentle movement and a slow return to walking on it. Most settle in two to six weeks, and the swelling can take months. Get it looked at the same day if you cannot put any weight on it, it swells badly, or it looks crooked.")]), py_tuple(vec![py_str("foot"), py_tuple(vec![py_str("foot"), py_str("feet"), py_str("heel"), py_str("heels"), py_str("arch"), py_str("plantar")]), py_str("Foot pain is often the plantar fascia, the band under the arch, worst on your first steps in the morning: stretch the calf against a wall before you stand up, roll the arch on a cold bottle, and wear shoes with a firm sole indoors too. A heel that hurts all day, or a foot that is red and hot, needs looking at.")]), py_tuple(vec![py_str("wrist"), py_tuple(vec![py_str("wrist"), py_str("wrists"), py_str("hand"), py_str("hands"), py_str("thumb"), py_str("fingers")]), py_str("Wrist and hand pain is usually an overused tendon or a nerve pressed by the way you grip and type: take breaks, keep the wrist straight rather than bent, and try a support splint from the pharmacy at night. Pins and needles in the fingers that wake you is worth a GP visit.")]), py_tuple(vec![py_str("elbow"), py_tuple(vec![py_str("elbow"), py_str("elbows"), py_str("tennis elbow")]), py_str("Elbow pain on the outside is tennis elbow, an overused tendon, and it settles over weeks to months once you rest from the gripping and lifting that sets it off: a strap just below the elbow and slow strengthening with a light weight help. See a GP if it is no better in six weeks.")]), py_tuple(vec![py_str("arm"), py_tuple(vec![py_str("arm"), py_str("arms"), py_str("upper arm"), py_str("forearm")]), py_str("Most arm pain is a muscle strain or the ache a day after lifting, and some of it comes down from a sore shoulder or elbow: keep using it as far as is comfortable and take a painkiller if you can. See a GP if the arm is weak or numb, and treat a heavy left arm with tightness in the chest, sweating or breathlessness as an emergency.")]), py_tuple(vec![py_str("ribs"), py_tuple(vec![py_str("rib"), py_str("ribs"), py_str("ribcage"), py_str("rib cage"), py_str("ribs hurt")]), py_str("A rib that hurts when you breathe in, cough or press it is usually bruised, not broken, and it mends the same way either way: keep breathing normally rather than in shallow sips, take paracetamol or ibuprofen, and press a cushion against that side when you cough. It eases over three to six weeks. Get seen the same day if you are breathless or coughing blood.")]), py_tuple(vec![py_str("tailbone"), py_tuple(vec![py_str("tailbone"), py_str("tail bone"), py_str("coccyx"), py_str("sacrum")]), py_str("A sore tailbone is usually a knock from a fall or hours sitting on a hard chair, and it mends slowly - a few weeks, sometimes a couple of months. Sit on a soft or doughnut cushion, tilt forward a little rather than leaning back onto it, and take paracetamol or ibuprofen; a warm bath helps the muscles around it. See a GP if it is no better after a few weeks, or if there is numbness or bleeding.")]), py_tuple(vec![py_str("toe"), py_tuple(vec![py_str("toe"), py_str("toes"), py_str("big toe")]), py_str("Most toe pain is a stub, a bruise under the nail or an ingrown edge: soak it in warm salty water, wear shoes with room in them and take paracetamol. A big toe that goes red, hot and swollen overnight, often at night, is gout - that one wants seeing this week, and sooner if you have diabetes and it looks infected.")]), py_tuple(vec![py_str("collarbone"), py_tuple(vec![py_str("collarbone"), py_str("collar bone"), py_str("clavicle")]), py_str("A collarbone that hurts after a knock usually mends in a sling over four to six weeks, and the pain often eases before the bone has joined: keep the hand and elbow moving gently, sleep propped up a little and take paracetamol. Have it looked at with an X-ray if you cannot lift the arm, the bone looks out of line, or it hurts too much to sleep.")]), py_tuple(vec![py_str("groin"), py_tuple(vec![py_str("groin"), py_str("adductor")]), py_str("A sore groin is usually a pulled muscle from twisting, kicking or turning quickly: give the sport that set it off a fortnight off, ice it for 20 minutes at a time for the first two days, then stretch and strengthen gently. Get a lump or a bulge that appears when you cough or lift looked at - that is a hernia - and get a sudden severe pain in one side seen the same day.")]), py_tuple(vec![py_str("leg"), py_tuple(vec![py_str("leg"), py_str("legs"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("thighs"), py_str("hamstring"), py_str("shin")]), py_str("A pulled thigh or calf muscle, or the ache that lands a day after squats, wants the same start: rest it, ice it for 20 minutes at a time for two days, then gentle stretching and a slow return to what you were doing, with the leg kept up when you sit. Calf pain with a hot, swollen leg, or a shin that hurts to touch after a knock, needs looking at.")])]) }
    s_REGIONS(__data14());
    fn __data15() -> Py { py_tuple(vec![py_str("stairs"), py_str("stair"), py_str("staircase"), py_str("steps"), py_str("climb"), py_str("climbing"), py_str("downstairs"), py_str("upstairs"), py_str("up steps"), py_str("down steps"), py_str("click"), py_str("clicks"), py_str("clicking"), py_str("creak"), py_str("creaks"), py_str("creaking"), py_str("grind"), py_str("grinding"), py_str("noise"), py_str("noisy"), py_str("popping"), py_str("crack"), py_str("cracks"), py_str("cracking")]) }
    s_EXTRA_AVOID(py_dict(vec![(py_str("wrist"), py_tuple(vec![py_str("i fell"), py_str("fell over"), py_str("fell on"), py_str("fell down"), py_str("fell off"), py_str("fell and"), py_str("tripped"), py_str("slipped"), py_str("landed on my"), py_str("banged it"), py_str("knocked it"), py_str("crushed")])), (py_str("arm"), py_tuple(vec![py_str("i fell"), py_str("fell over"), py_str("fell on"), py_str("fell down"), py_str("fell off"), py_str("fell and"), py_str("tripped"), py_str("slipped"), py_str("landed on my"), py_str("banged it"), py_str("knocked it"), py_str("crushed")])), (py_str("leg"), py_tuple(vec![py_str("swollen"), py_str("swelling"), py_str("hot"), py_str("flight"), py_str("long sit"), py_str("red"), py_str("tender"), py_str("warm"), py_str("shin"), py_str("shins")])), (py_str("back"), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("my"), py_str("the"), py_str("your")])) { let mut whose: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("knee"), py_str("knees"), py_str("neck"), py_str("throat"), py_str("hand"), py_str("hands"), py_str("ankle"), py_str("calf"), py_str("heel"), py_str("head"), py_str("shin"), py_str("elbow"), py_str("wrist"), py_str("foot")])) { let mut part: Py = __item.clone(); __out.push(op_modulo(&py_str("back of %s %s"), &py_tuple(vec![whose.clone(), part.clone()]))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("bad back"), py_str("a bad back"), py_str("gives trouble"), py_str("on and off"), py_str("gardening"), py_str("after gardening"), py_str("digging"), py_str("after a day of lifting"), py_str("lifting")]))), (py_str("knee"), __data15())]));
    fn __data16() -> Py { py_tuple(vec![py_str("knee"), py_str("knees"), py_str("ankle"), py_str("ankles"), py_str("wrist"), py_str("wrists"), py_str("elbow"), py_str("elbows"), py_str("hip"), py_str("hips"), py_str("shoulder"), py_str("shoulders"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("thumb"), py_str("thumbs"), py_str("joint"), py_str("joints")]) }
    s_JOINTS(__data16());
    s_HOT_SWELLING(py_tuple(vec![py_str("swollen"), py_str("swelling"), py_str("swelled"), py_str("puffy"), py_str("hot"), py_str("red"), py_str("inflamed")]));
    s_SEE_TODAY_BACK_AND_CALF(py_str("That is worth being seen today rather than waiting for it to settle. Pain in the back or side that hurts when you breathe, pain that wakes you at night, or a calf that is hot and swollen - especially after a flight or a long sit - are the shapes a doctor wants to look at the same day: ring 111 or your GP this morning. Go to A&E if your chest hurts suddenly or you are short of breath."));
    fn __data17() -> Py { py_tuple(vec![py_str("leg"), py_str("legs"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("back"), py_str("side"), py_str("spine"), py_str("rib"), py_str("ribs")]) }
    s_CLOT_AND_CHEST(__data17());
    s_HOT_FLAGS(py_tuple(vec![py_str("hot"), py_str("red"), py_str("inflamed"), py_str("burning"), py_str("went red"), py_str("gone red")]));
    fn __data18() -> Py { py_dict(vec![(py_str("feet"), py_str("foot")), (py_str("calves"), py_str("calf")), (py_str("thighs"), py_str("thigh")), (py_str("shins"), py_str("shin")), (py_str("hands"), py_str("hand")), (py_str("ribs"), py_str("rib")), (py_str("ribs hurt"), py_str("rib")), (py_str("tail bone"), py_str("tailbone")), (py_str("big toe"), py_str("toe")), (py_str("collar bone"), py_str("collarbone")), (py_str("fingers"), py_str("finger")), (py_str("thumbs"), py_str("thumb")), (py_str("heels"), py_str("heel")), (py_str("toes"), py_str("toe")), (py_str("knees"), py_str("knee")), (py_str("ankles"), py_str("ankle")), (py_str("elbows"), py_str("elbow")), (py_str("wrists"), py_str("wrist")), (py_str("shoulders"), py_str("shoulder")), (py_str("hips"), py_str("hip")), (py_str("legs"), py_str("leg")), (py_str("arms"), py_str("arm"))]) }
    s_PART_ONE(__data18());
    fn __data19() -> Py { py_tuple(vec![py_str("leg"), py_str("legs"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("groin"), py_str("saddle"), py_str("bottom"), py_str("bum"), py_str("backside"), py_str("butt"), py_str("buttocks"), py_str("crotch"), py_str("genitals"), py_str("knickers"), py_str("undercarriage"), py_str("between my legs"), py_str("down there"), py_str("hips"), py_str("bladder"), py_str("wee"), py_str("wees"), py_str("peed"), py_str("pee"), py_str("urine"), py_str("urinate"), py_str("toilet"), py_str("catheter"), py_str("bowels"), py_str("waterworks")]) }
    s_LOWER_HALF(__data19());
    fn __data20() -> Py { py_tuple(vec![py_str("arm"), py_str("arms"), py_str("hand"), py_str("hands"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("thumbs"), py_str("wrist"), py_str("wrists"), py_str("elbow"), py_str("elbows"), py_str("shoulder"), py_str("shoulders"), py_str("neck"), py_str("tongue"), py_str("face"), py_str("cheek"), py_str("jaw"), py_str("head"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes")]) }
    s_SOMEWHERE_ELSE(__data20());
    s_BACK_NAMES(py_tuple(vec![py_str("back"), py_str("backs"), py_str("backache"), py_str("back ache"), py_str("spine"), py_str("spinal"), py_str("lower back"), py_str("slipped disc"), py_str("sciatica"), py_str("my back")]));
    s_NUMB_ANYWHERE(py_tuple(vec![py_str("numb"), py_str("numbness"), py_str("pins and needles"), py_str("tingling"), py_str("no feeling"), py_str("lost feeling"), py_str("no sensation"), py_str("cant feel"), py_str("gone numb"), py_str("went numb"), py_str("dead"), py_str("asleep")]));
    fn __data21() -> Py { py_tuple(vec![py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("sole"), py_str("soles"), py_str("heel"), py_str("heels"), py_str("ankle"), py_str("ankles"), py_str("leg"), py_str("legs"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("hip"), py_str("hips"), py_str("buttock"), py_str("buttocks"), py_str("bottom"), py_str("bum"), py_str("backside"), py_str("groin"), py_str("saddle"), py_str("between my legs"), py_str("down there")]) }
    s_NUMB_LOWER(__data21());
    s_SPELLED_OUT(py_tuple(vec![py_str("cant feel my feet"), py_str("cant feel my foot"), py_str("cant feel my toes"), py_str("cant feel my legs"), py_str("cant feel my leg")]));
    s_SADDLE_PARTS(py_tuple(vec![py_str("saddle"), py_str("saddle area"), py_str("saddle region"), py_str("groin"), py_str("genitals"), py_str("crotch"), py_str("undercarriage"), py_str("down there")]));
    fn __data22() -> Py { py_tuple(vec![py_str("back"), py_str("backs"), py_str("leg"), py_str("legs"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("spine"), py_str("both legs"), py_str("hips"), py_str("hip"), py_str("give way"), py_str("gave way"), py_str("cant pee"), py_str("cant wee"), py_str("cant pass urine"), py_str("cant go for a wee"), py_str("cant go for a pee"), py_str("cant go to the toilet"), py_str("cant do a wee"), py_str("bladder"), py_str("bowels"), py_str("incontinent"), py_str("no urine"), py_str("wetting myself"), py_str("ache"), py_str("aching"), py_str("pain"), py_str("pains"), py_str("hurts"), py_str("hurting"), py_str("sore"), py_str("hurts to"), py_str("both sides")]) }
    s_FLAG_WITH_IT(__data22());
    s_SADDLE_CARD(py_str("Numbness between the legs, in the saddle area or the groin, is usually pressure on the nerves there: a bike saddle, a long sit, tight jeans or a wallet in a back pocket. It lifts within an hour of standing up. Shorten the rides and the sits, stand up every 20 minutes, and set the bike saddle level and not too high. If it has not gone by the next day, or it comes with back pain, numbness down both legs, or any trouble weeing or opening your bowels, ring 111 the same day and say \"numbness in the saddle area\" - do not wait for that one to settle."));
    s_RED_FLAG_CARD(py_str("Do not wait on that one. A sore back with no urine coming, numbness or pins and needles between your legs or in your feet, or a leg that gives way is the one back problem that has to be seen the same day: ring 999 or go to A&E now, and do not drive yourself. If you are not sure, ring 111 and say those words to them."));
    s_RING_NOT_ON_A_FINGER(op_add(&pyrt::xrefs::xget("knowledge.lookup", "RING_AS_A_CALL"), &py_tuple(vec![py_str("putting it off"), py_str("keep putting off"), py_str("kept putting off"), py_str("keep meaning to"), py_str("meant to ring"), py_str("forgot to ring"), py_str("dreading ringing")])));
    s__PIERCING_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(red|redness|swollen|swelling|hot|weeping|ooz|pus|yellow|green|feverish|fever|spreading|streaking|throbbing|angry|infected|infection|smelly|smell|lump|lumps)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PIERCING_BUMP(stdlib_call("re", "compile", &[py_str("\\b(bump|bumps|keloid|crusty|crust|scab|lump|stud sunk|embed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PIERCING_PIECE(stdlib_call("re", "compile", &[py_str("\\b(piercing|piercings|pierced|earring|earrings|ear ring|nose ring|nose stud|nose hoop|septum|helix|tragus|daith|rook|conch|cartilage|industrial|barbell|labret|stud|hoop|belly bar|belly button|eyebrow|nostril|lip ring|tongue bar|earlobe|ear lobe)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PIERCING_NOT_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(how long|heal|heals|healing|take to heal|when can i change|change the jewellery|aftercare|after care|keloid|crusty|crust|jewellery too tight|swollen in the morning|when can i swim|can i swim|downsize|bump|bumps)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CARTILAGE_PIERCING(stdlib_call("re", "compile", &[py_str("\\b(nose|nostril|septum|helix|tragus|daith|rook|conch|cartilage|industrial|eyebrow|bridge|forward helix)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PIERCING_ROUTINE(py_str(" Clean it twice a day with sterile saline on a clean pad and let water run over it in the shower - no alcohol, tea tree or antibacterial gel, and do not twist or turn it, because that keeps tearing the healing channel. Do not take it out early: a new piercing can close in hours, and if you must, have the piercer swap it. No swimming for two weeks, no sleeping on it, and no makeup or hairspray near it. A red bump that keeps growing, spreading redness, pus or a fever is a piercer or GP question, not a cleaning one."));
    fn __data24() -> Py { py_tuple(vec![py_str("cant pee"), py_str("can't pee"), py_str("cannot pee"), py_str("cant wee"), py_str("can't wee"), py_str("cant pass urine"), py_str("can't pass urine"), py_str("cant pass water"), py_str("no urine"), py_str("cant urinate"), py_str("can't urinate"), py_str("cannot urinate"), py_str("numb"), py_str("numbness"), py_str("pins and needles"), py_str("cant feel my legs"), py_str("can't feel my legs"), py_str("cant feel my feet"), py_str("legs give way"), py_str("leg gave way"), py_str("incontinent"), py_str("saddle"), py_str("cant go for a wee"), py_str("can't go for a wee"), py_str("cant go for a pee"), py_str("can't go for a pee"), py_str("cant have a wee"), py_str("can't have a wee"), py_str("cant do a wee"), py_str("can't do a wee"), py_str("cant go to the toilet"), py_str("can't go to the toilet"), py_str("cant get to the toilet"), py_str("no feeling"), py_str("lost feeling"), py_str("lost all feeling"), py_str("no sensation"), py_str("cant feel"), py_str("can't feel"), py_str("gone numb"), py_str("went numb"), py_str("dont know when i need a wee"), py_str("numb"), py_str("numbness"), py_str("pins and needles"), py_str("tingling"), py_str("don't know when i need a wee"), py_str("numb groin"), py_str("groin is numb"), py_str("groin went numb"), py_str("groin has gone numb"), py_str("numb round my groin"), py_str("numbness in my groin")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("back"), py_str("leg"), py_str("legs"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("shin"), py_str("spine"), py_str("side"), py_str("feet"), py_str("foot"), py_str("hip"), py_str("hips"), py_str("pain"), py_str("pains"), py_str("hurts"), py_str("hurting"), py_str("ache"), py_str("aching"), py_str("toe"), py_str("toes"), py_str("heel"), py_str("heels"), py_str("ankle"), py_str("ankles"), py_str("sole"), py_str("soles"), py_str("saddle"), py_str("groin"), py_str("genitals"), py_str("crotch"), py_str("undercarriage"), py_str("down there")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("cant"), py_str("can't"), py_str("cannot"), py_str("no"), py_str("numb"), py_str("numbness"), py_str("pins"), py_str("needles"), py_str("give"), py_str("gave"), py_str("incontinent"), py_str("saddle"), py_str("feeling"), py_str("sensation"), py_str("know"), py_str("toilet"), py_str("urine"), py_str("going"), py_str("groin")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("hurts when i breathe"), py_str("hurts to breathe"), py_str("pain when i breathe"), py_str("worse at night"), py_str("wakes me at night"), py_str("wake me at night"), py_str("wakes me up"), py_str("woke me up"), py_str("hot and swollen"), py_str("swollen and hot"), py_str("swollen calf"), py_str("swollen leg"), py_str("hot calf"), py_str("red and hot"), py_str("hot and red"), py_str("red and swollen"), py_str("swollen and red"), py_str("warm and red"), py_str("after a flight"), py_str("after a long flight"), py_str("swollen"), py_str("swelling"), py_str("swollen up"), py_str("gone hot"), py_str("hot to touch"), py_str("swelled"), py_str("swelled up"), py_str("swelling up"), py_str("puffy"), py_str("puffed up"), py_str("puffiness")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("hurts"), py_str("hurts to"), py_str("pain"), py_str("worse"), py_str("wakes"), py_str("woke"), py_str("hot"), py_str("swollen"), py_str("red"), py_str("after"), py_str("breathe"), py_str("night"), py_str("swelled"), py_str("puffy"), py_str("puffed")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("sting"), py_str("stings"), py_str("stung"), py_str("bee"), py_str("bees"), py_str("wasp"), py_str("wasps"), py_str("hornet"), py_str("insect"), py_str("insects"), py_str("bite"), py_str("bites"), py_str("bitten"), py_str("nettle"), py_str("nettles"), py_str("ant"), py_str("ants"), py_str("mosquito"), py_str("mosquitos"), py_str("mosquitoes"), py_str("horsefly"), py_str("tick"), py_str("midge"), py_str("midges"), py_str("gnat"), py_str("gnats")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("door"), py_str("doors"), py_str("drawer"), py_str("drawers"), py_str("window"), py_str("windows"), py_str("gate"), py_str("boot"), py_str("lid"), py_str("cupboard"), py_str("slammed"), py_str("slamming"), py_str("slam"), py_str("shut"), py_str("shutting"), py_str("closed"), py_str("trapped"), py_str("trapping"), py_str("caught"), py_str("catching"), py_str("jammed"), py_str("jamming"), py_str("crushed"), py_str("crushing"), py_str("squashed"), py_str("squashing"), py_str("came down on"), py_str("went between"), py_str("between the")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("hinge"), py_str("hinges"), py_str("oil"), py_str("oiling"), py_str("lock"), py_str("locks"), py_str("handle"), py_str("handles"), py_str("knob"), py_str("creak"), py_str("squeak"), py_str("paint"), py_str("draught"), py_str("draft"), py_str("warped"), py_str("sticking"), py_str("adjust"), py_str("marks"), py_str("mark"), py_str("smudge"), py_str("smudges"), py_str("fingerprint"), py_str("fingerprints"), py_str("clean"), py_str("cleaning"), py_str("wipe"), py_str("stain"), py_str("stains"), py_str("glass"), py_str("panel"), py_str("install"), py_str("replace"), py_str("fit a"), py_str("hang"), py_str("how do i get"), py_str("cat"), py_str("dog")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("black"), py_str("went black"), py_str("gone black"), py_str("blue"), py_str("purple"), py_str("dark"), py_str("bruised"), py_str("bruise"), py_str("dropped"), py_str("slammed"), py_str("hammer"), py_str("banged"), py_str("knock"), py_str("knocked"), py_str("caught it"), py_str("trapped"), py_str("after football")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("black"), py_str("blue"), py_str("purple"), py_str("bruise"), py_str("bruised"), py_str("dropped"), py_str("slammed"), py_str("hammer"), py_str("bang"), py_str("knock"), py_str("caught"), py_str("trapped"), py_str("dark"), py_str("throbbing"), py_str("sore")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("fungal"), py_str("fungus"), py_str("thrush"), py_str("yellow"), py_str("crumbly"), py_str("biting"), py_str("bite"), py_str("chew"), py_str("polish"), py_str("gel"), py_str("acrylic"), py_str("manicure"), py_str("varnish")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("toe"), py_str("toes"), py_str("foot"), py_str("feet"), py_str("ankle"), py_str("ankles"), py_str("heel"), py_str("heels"), py_str("leg"), py_str("skin"), py_str("bruise"), py_str("bruises"), py_str("legs"), py_str("shin"), py_str("shins"), py_str("calf"), py_str("calves"), py_str("thigh"), py_str("thighs"), py_str("knee"), py_str("knees"), py_str("hip"), py_str("hips"), py_str("arm"), py_str("arms"), py_str("forearm"), py_str("wrist"), py_str("wrists"), py_str("hand"), py_str("hands"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("thumbs"), py_str("elbow"), py_str("elbows"), py_str("shoulder"), py_str("shoulders")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("black"), py_str("black and blue"), py_str("gone black"), py_str("went black"), py_str("turned black"), py_str("turn black"), py_str("turning black"), py_str("going black"), py_str("gone blue"), py_str("went blue"), py_str("turned blue"), py_str("gone purple"), py_str("went purple"), py_str("turned purple"), py_str("purple")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("damp"), py_str("wall"), py_str("walls"), py_str("ceiling"), py_str("numb"), py_str("numbness"), py_str("cold"), py_str("freezing"), py_str("tingling"), py_str("tingly"), py_str("pins and needles"), py_str("grout"), py_str("sealant"), py_str("silicone"), py_str("tile"), py_str("tiles"), py_str("window"), py_str("frame"), py_str("paint"), py_str("paintwork"), py_str("plaster"), py_str("skirting"), py_str("bathroom"), py_str("shower"), py_str("nail"), py_str("nails"), py_str("toenail"), py_str("toenails"), py_str("fingernail"), py_str("fingernails"), py_str("eye"), py_str("eyes"), py_str("eyelid"), py_str("gum"), py_str("gums"), py_str("tooth"), py_str("teeth"), py_str("tongue"), py_str("lip"), py_str("lips"), py_str("mouth"), py_str("face"), py_str("cheek"), py_str("jaw"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("vet"), py_str("horse"), py_str("car"), py_str("bike"), py_str("boot"), py_str("boots"), py_str("shoe"), py_str("shoes"), py_str("sock"), py_str("socks"), py_str("varnish"), py_str("polish"), py_str("screen"), py_str("phone"), py_str("kettle"), py_str("sink"), py_str("bath"), py_str("bin"), py_str("bags"), py_str("tights"), py_str("leggings")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("tongue"), py_str("mouth"), py_str("thrush"), py_str("fungal"), py_str("fungus"), py_str("yellow"), py_str("black"), py_str("blue"), py_str("purple"), py_str("bruise"), py_str("paint"), py_str("polish"), py_str("varnish"), py_str("gel"), py_str("acrylic"), py_str("dog"), py_str("cat"), py_str("wall"), py_str("paintwork"), py_str("fence"), py_str("shirt")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("skin"), py_str("hands"), py_str("hand"), py_str("face"), py_str("cheeks"), py_str("legs"), py_str("arms"), py_str("elbows"), py_str("heels"), py_str("feet"), py_str("knuckles"), py_str("body"), py_str("moisturiser"), py_str("moisturizer"), py_str("emollient")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("dry"), py_str("drier"), py_str("dryness"), py_str("flaky"), py_str("flakes"), py_str("flaking"), py_str("cracked"), py_str("cracking"), py_str("cracks"), py_str("rough"), py_str("itchy"), py_str("itching"), py_str("tight"), py_str("peeling"), py_str("chapped")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("lips"), py_str("lip"), py_str("mouth"), py_str("corner of my mouth"), py_str("eyes"), py_str("eye"), py_str("hair"), py_str("dandruff"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my kids"), py_str("my baby"), py_str("my toddler"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my mom"), py_str("my mother"), py_str("my dad"), py_str("my father"), py_str("my friend"), py_str("my dog"), py_str("my cat"), py_str("my puppy"), py_str("my horse"), py_str("his skin"), py_str("her skin"), py_str("their skin"), py_str("sore"), py_str("hurts"), py_str("hurt"), py_str("aching"), py_str("ache"), py_str("pain"), py_str("pains"), py_str("painful"), py_str("stiff"), py_str("injury"), py_str("injured"), py_str("egg"), py_str("eggs"), py_str("knuckles crack"), py_str("knuckles cracking"), py_str("knuckle cracks"), py_str("knuckles click"), py_str("knuckles clicking"), py_str("knuckles pop"), py_str("crack my knuckles"), py_str("cracking my knuckles"), py_str("crack your knuckles"), py_str("cracking your knuckles")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("wash"), py_str("washing"), py_str("washed"), py_str("clean"), py_str("cleanse"), py_str("cleanser"), py_str("temperature"), py_str("how hot"), py_str("hot water"), py_str("cold water"), py_str("lukewarm"), py_str("twice a day"), py_str("before bed")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("dry"), py_str("drier"), py_str("drying out"), py_str("dry out"), py_str("dry it out"), py_str("itchy"), py_str("spot"), py_str("spots"), py_str("acne"), py_str("eczema"), py_str("flaky"), py_str("cracked"), py_str("hands"), py_str("hand"), py_str("body"), py_str("hair"), py_str("baby"), py_str("child"), py_str("dishes"), py_str("clothes"), py_str("laundry"), py_str("mask"), py_str("face mask"), py_str("beard"), py_str("shave"), py_str("shaving")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("wash"), py_str("washing"), py_str("washed"), py_str("clean"), py_str("cleaning"), py_str("soap"), py_str("germs"), py_str("bacteria"), py_str("hygiene"), py_str("how long"), py_str("before eating"), py_str("after the toilet"), py_str("after gardening"), py_str("after diy"), py_str("hot water"), py_str("cold water"), py_str("sanitiser"), py_str("hand gel")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("dry"), py_str("drier"), py_str("cracked"), py_str("cracking"), py_str("itchy"), py_str("eczema"), py_str("numb"), py_str("tingling"), py_str("cold"), py_str("warm"), py_str("winter"), py_str("raynaud"), py_str("pain"), py_str("hurt"), py_str("swollen"), py_str("nails"), py_str("nail polish"), py_str("paint"), py_str("oil"), py_str("grease"), py_str("bleach"), py_str("stain"), py_str("dishes"), py_str("washing up"), py_str("laundry"), py_str("washing machine"), py_str("gloves"), py_str("mittens"), py_str("dog"), py_str("cat"), py_str("car"), py_str("window"), py_str("face"), py_str("hair"), py_str("jumper"), py_str("jumpers"), py_str("cardigan"), py_str("cardigans"), py_str("wool"), py_str("woollen"), py_str("woolens"), py_str("notice"), py_str("notice in"), py_str("resign"), py_str("resigning"), py_str("resignation"), py_str("job"), py_str("boss"), py_str("employer"), py_str("manager"), py_str("contract"), py_str("redundancy"), py_str("sacked"), py_str("promotion"), py_str("knitwear"), py_str("knitted"), py_str("cashmere"), py_str("merino"), py_str("silk"), py_str("silky"), py_str("scarf"), py_str("scarves"), py_str("dress"), py_str("dresses"), py_str("shirt"), py_str("shirts"), py_str("blouse"), py_str("sweater"), py_str("sweaters"), py_str("fleece"), py_str("by hand"), py_str("hand wash"), py_str("hand washing"), py_str("hand-wash"), py_str("handwash"), py_str("hand washed"), py_str("stitches"), py_str("stitch"), py_str("sutures"), py_str("suture"), py_str("staples"), py_str("wound"), py_str("wounds"), py_str("gash"), py_str("gashes"), py_str("scab"), py_str("scabs"), py_str("bandage"), py_str("bandages"), py_str("dressing"), py_str("dressings"), py_str("plaster cast"), py_str("graze"), py_str("grazes"), py_str("open wound"), py_str("cut on my hand"), py_str("cut on the hand"), py_str("cut on my finger"), py_str("splinter"), py_str("ingrown")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("moisturiser"), py_str("moisturizer"), py_str("moisturise"), py_str("moisturize"), py_str("moisturising"), py_str("moisturizing"), py_str("face cream"), py_str("day cream"), py_str("night cream"), py_str("body lotion"), py_str("face lotion"), py_str("skin cream"), py_str("cream for my face"), py_str("cream for dry skin")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("need"), py_str("needs"), py_str("should"), py_str("what"), py_str("which"), py_str("choose"), py_str("choosing"), py_str("best"), py_str("use"), py_str("worth"), py_str("buy"), py_str("type"), py_str("kind"), py_str("recommend"), py_str("looking for"), py_str("how do i"), py_str("how to"), py_str("good"), py_str("cheap"), py_str("expensive")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("dry"), py_str("drier"), py_str("dryness"), py_str("flaky"), py_str("flakes"), py_str("flaking"), py_str("cracked"), py_str("cracking"), py_str("cracks"), py_str("rough"), py_str("itchy"), py_str("itching"), py_str("tight"), py_str("peeling"), py_str("chapped"), py_str("sore"), py_str("rash"), py_str("eczema"), py_str("psoriasis"), py_str("red"), py_str("burning"), py_str("sunburn"), py_str("sun cream"), py_str("sunscreen"), py_str("spf"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("baby"), py_str("babies"), py_str("nappy"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my kids")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("skin"), py_str("face"), py_str("forehead"), py_str("nose"), py_str("chin"), py_str("cheeks"), py_str("complexion"), py_str("t-zone"), py_str("t zone"), py_str("oily skin"), py_str("greasy skin"), py_str("shiny skin"), py_str("oily face"), py_str("greasy face")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("greasy"), py_str("oily"), py_str("oil"), py_str("shiny"), py_str("shine"), py_str("gleam"), py_str("getting greasy"), py_str("gets greasy"), py_str("getting oily"), py_str("gets oily"), py_str("getting shiny"), py_str("by lunchtime"), py_str("during the day")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("dry"), py_str("drier"), py_str("flaky"), py_str("tight"), py_str("rash"), py_str("eczema"), py_str("psoriasis"), py_str("spot"), py_str("spots"), py_str("spotty"), py_str("spotting"), py_str("acne"), py_str("hair"), py_str("scalp"), py_str("chip"), py_str("chips"), py_str("sink"), py_str("sinks"), py_str("stainless"), py_str("stainless steel"), py_str("steel"), py_str("kitchen"), py_str("bathroom"), py_str("tap"), py_str("taps"), py_str("worktop"), py_str("cutlery"), py_str("silver"), py_str("brass"), py_str("copper"), py_str("metal"), py_str("floor"), py_str("floors"), py_str("window"), py_str("windows"), py_str("car"), py_str("shoes"), py_str("leather"), py_str("pan"), py_str("frying"), py_str("food"), py_str("oven"), py_str("ovens"), py_str("hob"), py_str("hobs"), py_str("cooker"), py_str("cookers"), py_str("grill"), py_str("griller"), py_str("microwave"), py_str("extractor"), py_str("tray"), py_str("trays"), py_str("baking"), py_str("roasting"), py_str("tin"), py_str("tins"), py_str("saucepan"), py_str("saucepans"), py_str("worktop"), py_str("worktops"), py_str("splashback"), py_str("cupboard"), py_str("cupboards"), py_str("sink"), py_str("dishes"), py_str("dishwasher"), py_str("fridge"), py_str("freezer"), py_str("kettle"), py_str("toaster"), py_str("barbecue"), py_str("bbq"), py_str("wall"), py_str("walls"), py_str("tiles"), py_str("kitchen"), py_str("kitchens"), py_str("bathroom"), py_str("bath"), py_str("shower"), py_str("floor"), py_str("floors"), py_str("garage"), py_str("surface"), py_str("surfaces"), py_str("counter"), py_str("counters"), py_str("unit"), py_str("units"), py_str("car"), py_str("engine"), py_str("bike"), py_str("chain"), py_str("dog"), py_str("cat"), py_str("kitchen"), py_str("kitchens"), py_str("hob"), py_str("hobs"), py_str("cooker"), py_str("oven"), py_str("worktop"), py_str("worktops"), py_str("sink"), py_str("splashback"), py_str("extractor"), py_str("floor"), py_str("floors"), py_str("tiles"), py_str("dishes"), py_str("cooking"), py_str("cooked"), py_str("bbq"), py_str("barbecue"), py_str("grill"), py_str("containers"), py_str("jars"), py_str("suit"), py_str("suits"), py_str("jacket"), py_str("jackets"), py_str("blazer"), py_str("blazers"), py_str("trousers"), py_str("shirt"), py_str("shirts"), py_str("skirt"), py_str("dress"), py_str("dresses"), py_str("coat"), py_str("coats"), py_str("sleeve"), py_str("sleeves"), py_str("elbow"), py_str("elbows"), py_str("fabric"), py_str("leather"), py_str("sofa"), py_str("settee"), py_str("table"), py_str("tables"), py_str("wooden"), py_str("furniture"), py_str("cupboard"), py_str("chair"), py_str("desk"), py_str("shoe"), py_str("shoes"), py_str("boot"), py_str("boots"), py_str("polish"), py_str("polishing"), py_str("polished")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("what"), py_str("which"), py_str("should"), py_str("how do i"), py_str("how to"), py_str("choose"), py_str("best"), py_str("need"), py_str("start"), py_str("simple"), py_str("good"), py_str("order"), py_str("daily"), py_str("routine"), py_str("use")]) }
    fn __data53() -> Py { py_tuple(vec![py_str("spot"), py_str("spots"), py_str("spotty"), py_str("acne"), py_str("blackhead"), py_str("blackheads"), py_str("dry"), py_str("drier"), py_str("flaky"), py_str("itching"), py_str("itchy"), py_str("rash"), py_str("eczema"), py_str("psoriasis"), py_str("mole"), py_str("wrinkles"), py_str("botox"), py_str("sunburn"), py_str("dandruff"), py_str("hair"), py_str("foundation"), py_str("makeup"), py_str("make up"), py_str("dog"), py_str("cat"), py_str("baby"), py_str("babies"), py_str("nappy"), py_str("child"), py_str("kid"), py_str("kids")]) }
    fn __data23() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("body-urgent")); __args.push(py_tuple(vec![__data24(), __data25()])); __args.push(py_func("_red_flag", |args: &[Py]| _red_flag(args))); __kws.extend(vec![("ask".to_string(), __data26()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("body-see-today")); __args.push(py_tuple(vec![__data27(), op_add(&g__SORE_ANYWHERE(), &py_tuple(vec![py_str("swollen"), py_str("swelling"), py_str("hot")]))])); __args.push(py_func("_see_today", |args: &[Py]| _see_today(args))); __kws.extend(vec![("ask".to_string(), __data28()), ("avoid".to_string(), __data29()), ("guard".to_string(), py_func("_the_part_is_a_persons", |args: &[Py]| _the_part_is_a_persons(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("crushed-finger")); __args.push(py_tuple(vec![py_tuple(vec![py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("thumbs"), py_str("fingertip"), py_str("fingertips"), py_str("hand"), py_str("hands"), py_str("knuckle"), py_str("knuckles")]), __data30()])); __args.push(py_str("That is a crush, so the first day is a cold pack - ice in a tea towel, ten minutes at a time, a few times - with the hand up, and paracetamol or ibuprofen for the ache. Take any ring off that finger now, before the swelling. Go to A&E the same day if the fingertip is split or the nail is torn, if blood is building up under the nail, if it throbs hard, if the finger will not straighten or looks the wrong shape, or if it is numb: a trapped fingertip can break the bone under the nail, and that needs an X-ray.")); __kws.extend(vec![("avoid".to_string(), __data31())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("black-nail")); __args.push(py_tuple(vec![py_tuple(vec![py_str("nail"), py_str("nails"), py_str("toenail"), py_str("toenails"), py_str("fingernail"), py_str("fingernails"), py_str("nail bed")]), __data32()])); __args.push(py_str("A knock that turns the nail black is blood trapped under it. Ice it for ten minutes at a time for the first day, keep the hand or foot up, and take paracetamol or ibuprofen for the ache. A small dark patch fades over weeks as the nail grows out. Get it seen the same day if it throbs hard, the nail is lifting, half of it is dark, or the finger will not bend properly: a doctor can let the blood out, and draining it at home can infect it.")); __kws.extend(vec![("ask".to_string(), __data33()), ("avoid".to_string(), __data34())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("limb-went-black")); __args.push(py_tuple(vec![__data35(), __data36()])); __args.push(py_str("A bruise is blood under the skin from a knock: it looks black at first, then fades through purple, green and yellow over one to two weeks. A cold pack for twenty minutes with a cloth round it, a few times in the first two days, eases it. Get it seen the same day if it swells hard and fast, the skin is broken, the part will not take your weight, or a patch goes black with no knock you can remember.")); __kws.extend(vec![("guard".to_string(), py_func("_the_part_is_a_persons", |args: &[Py]| _the_part_is_a_persons(args))), ("avoid".to_string(), __data37())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("white-spots-nails")); __args.push(py_tuple(vec![py_tuple(vec![py_str("nails"), py_str("nail"), py_str("fingernails"), py_str("fingernail"), py_str("toenails"), py_str("toenail")]), py_tuple(vec![py_str("white spots"), py_str("white marks"), py_str("white flecks"), py_str("white patches"), py_str("white lines"), py_str("white dots"), py_str("little white"), py_str("spots on my nails"), py_str("marks on my nails")])])); __args.push(py_str("White spots in a nail are usually small knocks to the nail bed while that part of the nail was forming, and they grow out over a few months - not a calcium or zinc shortage, which is what most people are told. Filing the surface will not shift them and nothing needs doing. White lines across every nail, or half a nail white with a darker band above it, is worth a GP look.")); __kws.extend(vec![("avoid".to_string(), __data38())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dry-skin")); __args.push(py_tuple(vec![__data39(), __data40()])); __args.push(py_str("Dry, tight skin in winter is mostly central heating and long hot showers washing the oils out: short lukewarm showers, pat dry rather than rub, and a thick unperfumed cream within three minutes while the skin is still damp. Cracked hands and heels want a cream with urea or lanolin and cotton gloves overnight. If the cracks bleed or the skin is red and itchy, a pharmacist can give you an emollient.")); __kws.extend(vec![("avoid".to_string(), __data41()), ("guard".to_string(), py_func("_the_part_is_a_persons", |args: &[Py]| _the_part_is_a_persons(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("face-wash")); __args.push(py_tuple(vec![py_tuple(vec![py_str("face"), py_str("facial")]), __data42()])); __args.push(py_str("Lukewarm water and a gentle unperfumed cleanser, morning and evening: warm water and a flannel lift grime and make-up, while hot water strips the oils and leaves the skin tight. Pat dry rather than rub, and put moisturiser on while the skin is still damp; anything that stings is too harsh for a face.")); __kws.extend(vec![("avoid".to_string(), __data43()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hands-wash")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hands"), py_str("hand"), py_str("fingernails"), py_str("fingertips")]), __data44()])); __args.push(py_str("Wet your hands, then soap for twenty seconds - about as long as singing Happy Birthday twice - and cover the backs of the hands, between the fingers, the thumbs and the fingertips under the nails. Rinse and dry on a clean towel, because wet hands pick up germs again, and use hand gel only when there is no sink.")); __kws.extend(vec![("avoid".to_string(), __data45()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dry-scalp")); __args.push(py_tuple(vec![py_tuple(vec![py_str("scalp"), py_str("scalp is"), py_str("head itches")]), py_tuple(vec![py_str("dry"), py_str("drier"), py_str("dryness"), py_str("flaky"), py_str("flakes"), py_str("flaking"), py_str("itchy"), py_str("itching"), py_str("scaly"), py_str("dandruff"), py_str("white bits"), py_str("snowy")])])); __args.push(py_str("A dry, flaky scalp is usually dandruff: a yeast that feeds on the oil on your head, so it is not the same thing as dry skin and body cream will not help it. Wash with an anti-dandruff shampoo - zinc pyrithione or ketoconazole, from any chemist - two or three times a week, work it into the scalp and leave it five minutes before rinsing, and give it a month. Do not scratch the flakes off, and keep washing your hair: a scalp left greasy feeds it. If it is red, sore, or hair is thinning in patches, see a pharmacist or your GP.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("hair"), py_str("shampoo"), py_str("conditioner"), py_str("dye"), py_str("coloured"), py_str("bleach")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("moisturiser")); __args.push(py_tuple(vec![__data46(), __data47()])); __args.push(py_str("A plain unperfumed cream is the whole job: thicker (an ointment, or a cream in a tub) for dry or flaky skin, a light lotion or gel for oily skin, and one with SPF 30 in it for the morning if you want to skip a step. Put it on damp skin within three minutes of a shower, buy the cheap own-brand - the price is packaging - and a pump is cleaner than a tub you dig your fingers into.")); __kws.extend(vec![("avoid".to_string(), __data48())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("oily-skin")); __args.push(py_tuple(vec![__data49(), __data50()])); __args.push(py_str("Wash twice a day with a gentle foaming cleanser rather than soap, which strips the skin and leaves it making more oil, then use a light oil-free gel moisturiser - oily skin still needs one. Do not scrub or wash more than twice; blot the shine at lunchtime instead, and a niacinamide or salicylic acid product is the one extra step worth adding.")); __kws.extend(vec![("avoid".to_string(), __data51())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("skin-routine")); __args.push(py_tuple(vec![py_tuple(vec![py_str("skincare"), py_str("skin care"), py_str("cleanser"), py_str("cleansing"), py_str("face wash"), py_str("glowing skin"), py_str("skin routine"), py_str("face routine"), py_str("my skin looks tired"), py_str("look after my skin"), py_str("look after your skin"), py_str("good skin")]), __data52()])); __args.push(py_str("Keep it to three steps: a gentle cleanser, a moisturiser, and a sunscreen in the morning - add one active, a retinoid or an acid, at night only once those three are a habit. Most routines fail because they are too long, and no jar beats sleep, water and not smoking.")); __kws.extend(vec![("avoid".to_string(), __data53())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    s_ENTRIES(__data23());
    fn __data54() -> Py { py_tuple(vec![py_str("put my back out"), py_str("put out my back"), py_str("put it out"), py_str("threw my back out"), py_str("threw it out"), py_str("my back went"), py_str("back went"), py_str("my back has gone"), py_str("my back is out"), py_str("my back went out"), py_str("my back gave way"), py_str("my back locked"), py_str("my back is in spasm"), py_str("my back went into spasm"), py_str("i tweaked my back"), py_str("i twisted my back"), py_str("my back slipped")]) }
    s_BACK_WENT(__data54());
    s__THE_VISIT(py_set(vec![py_str("appointment"), py_str("appointments"), py_str("checkup"), py_str("check-up"), py_str("check"), py_str("visit"), py_str("filling"), py_str("hygienist"), py_str("root canal")]));
    fn __data55() -> Py { py_set(vec![py_str("wait"), py_str("waits"), py_str("waiting"), py_str("soon"), py_str("sooner"), py_str("quickly"), py_str("book"), py_str("booking"), py_str("booked"), py_str("getting"), py_str("seen"), py_str("available"), py_str("weeks"), py_str("days"), py_str("months"), py_str("list")]) }
    s__THE_QUEUE(__data55());
    s__REGISTERING_A_DENTIST(py_str("Look for practices near you that are taking on new patients, then ring rather than email: each practice keeps its own list and it opens and closes, and the phone gets you a straight answer. Ask on the call whether they take NHS adults and how long the first check-up wait is. If nobody near you has room, ring 111 and ask for the urgent dental service, which will see you and can tell you who is taking patients."));
    s__A_LENGTH_OF_A_VISIT(stdlib_call("re", "compile", &[py_str("\\bhow long (?:does|do|will|would|should)\\b[^?.!]*\\b(?:take|last|wait)\\b|\\bhow long (?:is|are|was)\\b|\\bhow many (?:minutes|hours)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LIST_THAT_IS_LONG(stdlib_call("re", "compile", &[py_str("\\bwaiting list\\b|\\bon the list for\\b|\\blist is (?:months|years|weeks|long|closed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PRACTICE_ERRAND(stdlib_call("re", "compile", &[py_str("\\b(?:register|registering|sign up|join|join up|find|finding|accepting|new patients?|taking (?:on )?(?:new )?patients?|need (?:a|an|an nhs|a new) dentist|get a dentist|looking for a dentist|near me|nearby|local dentist|none|no dentist|moved)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BECOMING_A_DENTIST(stdlib_call("re", "compile", &[py_str("\\b(?:become|becoming|train as|training as|study|studying|degree|qualify as|qualifying as|career|became)\\b[^?.!]*\\b(?:dentist|dentistry|dental school|dental surgeon)\\b|\\b(?:dentist|dentistry|dental school|dental surgeon)\\b[^?.!]*\\b(?:training|course|degree|qualify|career|school)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CHECK_UP(py_str("A check-up with a dentist is booked for about 15 to 20 minutes, and a first visit to a practice often gets half an hour because they take an X-ray and a history."));
    s__A_FILLING(py_str("A filling takes 20 to 30 minutes."));
    s__A_ROOT_CANAL(py_str("A root canal takes 45 minutes to an hour."));
    s__A_HYGIENIST(py_str("A clean with the hygienist takes about 30 minutes."));
    s__A_CROWN_VISIT(py_str("A crown is two visits of about an hour each, one to shape the tooth and one to fit it."));
    s__AN_IMPLANT(py_str("An implant is two short visits and a long wait in between: about an hour to place it under the gum, three to six months for the bone to take to it, then about an hour to fit the crown on top."));
    s__THE_BOOKING(py_str("Say what it is for when you book, and ask for a double appointment if you have more than one thing to look at, so the slot fits."));
    s__THE_VISIT_IN_ORDER(py_tuple(vec![g__A_CHECK_UP(), g__A_FILLING(), g__A_ROOT_CANAL(), g__A_HYGIENIST(), g__A_CROWN_VISIT()]));
    fn __data56() -> Py { py_tuple(vec![py_str("went"), py_str("gone"), py_str("put"), py_str("out"), py_str("threw"), py_str("twisted"), py_str("tweaked"), py_str("slipped"), py_str("locked"), py_str("spasm"), py_str("gave way"), py_str("went out"), py_str("is out"), py_str("goes"), py_str("go"), py_str("going"), py_str("seize"), py_str("seizes"), py_str("seized"), py_str("seizing"), py_str("done my back in"), py_str("done my back"), py_str("pulled my back"), py_str("back has gone"), py_str("backs gone"), py_str("back went"), py_str("in spasm")]) }
    fn __data57() -> Py { py_tuple(vec![py_str("deal"), py_str("contract"), py_str("wedding"), py_str("booking"), py_str("order"), py_str("offer"), py_str("agreement"), py_str("mortgage"), py_str("pull out"), py_str("pull out of"), py_str("going ahead"), py_str("hire purchase"), py_str("stiff"), py_str("stiffness"), py_str("stiffened"), py_str("stiffens"), py_str("stiffening")]) }
    fn __data58() -> Py { py_tuple(vec![py_str("hob"), py_str("cooker"), py_str("burner"), py_str("stove"), py_str("bath"), py_str("curtain"), py_str("doorbell"), py_str("bell"), py_str("phone"), py_str("answering"), py_str("ear"), py_str("ears"), py_str("nose"), py_str("nose ring"), py_str("earring"), py_str("earrings"), py_str("sound"), py_str("sounds"), py_str("noise"), py_str("circle"), py_str("tree"), py_str("bite"), py_str("bites"), py_str("bitten")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("piercing"), py_str("piercings"), py_str("pierced ear"), py_str("pierced ears"), py_str("ear piercing"), py_str("ear piercings"), py_str("nose piercing"), py_str("septum"), py_str("belly piercing"), py_str("belly button piercing"), py_str("cartilage piercing"), py_str("helix"), py_str("tragus"), py_str("daith"), py_str("industrial piercing"), py_str("lip piercing"), py_str("eyebrow piercing"), py_str("nipple piercing"), py_str("new piercing"), py_str("nose ring"), py_str("nose rings"), py_str("septum ring"), py_str("nose hoop"), py_str("ear hoop"), py_str("hoop earring"), py_str("nose stud"), py_str("stud earring"), py_str("ear stud"), py_str("belly bar"), py_str("ring in my nose"), py_str("ring in my ear")]) }
    fn __data60() -> Py { py_tuple(vec![py_str("how long"), py_str("heal"), py_str("heals"), py_str("healing"), py_str("take to heal"), py_str("sore"), py_str("red"), py_str("swollen"), py_str("aftercare"), py_str("clean"), py_str("cleaning"), py_str("clean it"), py_str("care"), py_str("infection"), py_str("bump"), py_str("keloid"), py_str("crusty"), py_str("weeping"), py_str("swimming"), py_str("sleep on"), py_str("change the jewellery"), py_str("take it out"), py_str("when can i change"), py_str("change it"), py_str("changing it"), py_str("swap it"), py_str("downsize"), py_str("shorter bar")]) }
    fn __data61() -> Py { py_tuple(vec![py_str("earring"), py_str("earrings"), py_str("ear ring"), py_str("tongue"), py_str("clit"), py_str("industrial estate"), py_str("helix nebula"), py_str("engagement ring"), py_str("wedding ring"), py_str("ring finger"), py_str("ring main"), py_str("ring final"), py_str("hob ring"), py_str("onion ring"), py_str("boxing ring"), py_str("circus ring"), py_str("key ring"), py_str("keyring"), py_str("napkin ring"), py_str("curtain ring"), py_str("smoke ring")]) }
    fn __data62() -> Py { py_tuple(vec![py_str("hob"), py_str("cooker"), py_str("burner"), py_str("stove"), py_str("bath"), py_str("curtain"), py_str("doorbell"), py_str("bell"), py_str("phone"), py_str("answering"), py_str("ear"), py_str("ears"), py_str("nose"), py_str("sound"), py_str("circle"), py_str("caller"), py_str("callers"), py_str("cold call"), py_str("cold calls"), py_str("nuisance"), py_str("spam"), py_str("robocall"), py_str("robocalls"), py_str("unknown number"), py_str("tree"), py_str("bite"), py_str("bites"), py_str("bitten")]) }
    fn __data63() -> Py { py_tuple(vec![py_str("dentist"), py_str("dental practice"), py_str("dental surgery"), py_str("nhs dentist"), py_str("orthodontist"), py_str("filling"), py_str("fillings"), py_str("root canal"), py_str("hygienist"), py_str("check up"), py_str("check-up"), py_str("checkup"), py_str("crown"), py_str("crowns"), py_str("dental")]) }
    fn __data64() -> Py { py_tuple(vec![py_str("register"), py_str("sign up"), py_str("join"), py_str("find"), py_str("accepting"), py_str("new patients"), py_str("new patient"), py_str("taking patients"), py_str("patients"), py_str("taking"), py_str("on the list"), py_str("no dentist"), py_str("need a dentist"), py_str("looking for"), py_str("get one"), py_str("get a dentist"), py_str("near me"), py_str("nearby"), py_str("local"), py_str("none"), py_str("waiting"), py_str("how long"), py_str("how soon"), py_str("how many minutes"), py_str("last"), py_str("lasts"), py_str("take"), py_str("takes"), py_str("wait"), py_str("waits")]) }
    fn __data65() -> Py { py_tuple(vec![py_str("toothache"), py_str("tooth"), py_str("teeth"), py_str("pain"), py_str("hurts"), py_str("hurting"), py_str("agony"), py_str("bleeding"), py_str("abscess"), py_str("swollen"), py_str("broken"), py_str("chipped"), py_str("knocked out"), py_str("emergency"), py_str("today"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("toddler"), py_str("in pain"), py_str("wisdom"), py_str("abroad"), py_str("holiday"), py_str("holidays"), py_str("travelling"), py_str("traveling"), py_str("expat"), py_str("spain"), py_str("france"), py_str("greece"), py_str("turkey"), py_str("portugal"), py_str("america"), py_str("usa"), py_str("thailand"), py_str("croatia"), py_str("italy"), py_str("complain"), py_str("complaining"), py_str("complaint"), py_str("complaints")]) }
    fn __data66() -> Py { py_tuple(vec![py_str("register"), py_str("sign up"), py_str("join"), py_str("find"), py_str("new patient"), py_str("new patients"), py_str("on the list"), py_str("taking patients"), py_str("moved"), py_str("just moved"), py_str("change"), py_str("switch"), py_str("another"), py_str("different"), py_str("near me"), py_str("nearby"), py_str("local")]) }
    s_ENTRIES(op_add(&py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("back-went")); __args.push(py_tuple(vec![py_tuple(vec![py_str("my back"), py_str("lower back"), py_str("my spine")]), __data56()])); __args.push(py_str("A back that goes like that is almost always a muscle strain, and it is at its worst in the first day or two: keep moving gently rather than taking to your bed, take paracetamol or ibuprofen if you can, and use a hot water bottle on the tight part. Most settle in two to six weeks. Ring 111 today if you cannot pass urine, have numbness between your legs, or the pain came on after a fall, and see a GP if it is no better in a fortnight.")); __kws.extend(vec![("avoid".to_string(), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AVOID()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_EXERCISE_DAY(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &__data57()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ring-stuck")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ring"), py_str("rings"), py_str("wedding ring"), py_str("engagement ring"), py_str("band")]), py_tuple(vec![py_str("stuck"), py_str("swollen finger"), py_str("swollen"), py_str("off my finger"), py_str("off"), py_str("wont come off"), py_str("won't come off"), py_str("cut off"), py_str("tight"), py_str("too tight"), py_str("trapped")])])); __args.push(py_str("Work it off with soap and water or a little oil, twisting rather than pulling, hand raised above your head for a few minutes first to let the swelling go down. Wind a piece of thread or dental floss under the ring from the finger side and unwind it around the finger past the knuckle, tucking the ring along a loop at a time. Get help the same day if the finger goes numb, cold or blue: A&E or a jeweller can cut it off.")); __kws.extend(vec![("avoid".to_string(), op_add(&__data58(), &g_RING_NOT_ON_A_FINGER())), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("ring"), py_str("rings")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("piercing-healing")); __args.push(py_tuple(vec![__data59(), __data60()])); __args.push(py_func("_piercing_healing", |args: &[Py]| _piercing_healing(args))); __kws.extend(vec![("guard".to_string(), py_func("_a_healing_piercing_not_a_troubled_one", |args: &[Py]| _a_healing_piercing_not_a_troubled_one(args))), ("avoid".to_string(), __data61())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ring-stuck-blue")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ring"), py_str("rings"), py_str("wedding ring"), py_str("engagement ring"), py_str("band")]), py_tuple(vec![py_str("gone blue"), py_str("turning blue"), py_str("blue"), py_str("cold"), py_str("black"), py_str("cutting in"), py_str("cut into")])])); __args.push(py_str("Get it off today, and get help if you cannot: a finger that has gone numb, cold, blue or black behind a ring needs the ring cut off, so go to A&E or ring 111 and they will tell you where. While you wait, keep the hand raised above your heart and run it under cool water, and do not keep twisting the ring. A jeweller can cut it off for a few pounds if the finger still looks normal.")); __kws.extend(vec![("avoid".to_string(), op_add(&__data62(), &g_RING_NOT_ON_A_FINGER())), ("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("ring"), py_str("rings")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("register-dentist")); __args.push(py_tuple(vec![__data63(), __data64()])); __args.push(py_func("_the_dentist_errand", |args: &[Py]| _the_dentist_errand(args))); __kws.extend(vec![("guard".to_string(), py_func("_an_errand_the_dentist_card_answers", |args: &[Py]| _an_errand_the_dentist_card_answers(args))), ("avoid".to_string(), __data65()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("register-gp")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gp"), py_str("doctor"), py_str("doctor's"), py_str("doctors surgery"), py_str("doctors practice"), py_str("health centre"), py_str("medical centre"), py_str("surgery"), py_str("nhs")]), __data66()])); __args.push(py_str("Ring the surgeries near your new address and ask if their list is open: most take people from their own area, and a practice can say no if you live outside it, so try the next one rather than arguing. Take photo ID and something with the new address to the first appointment. If you are on regular medicine, ask for the repeat prescription to be set up that week, and tell them you have just moved so the old notes are asked for.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("appointment"), py_str("appointments"), py_str("refer"), py_str("referral"), py_str("sick note"), py_str("fit note"), py_str("ignored"), py_str("dismissed"), py_str("sent me away"), py_str("waiting list"), py_str("results"), py_str("test")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]), &g_ENTRIES()));
    fn __data67() -> Py { py_tuple(vec![py_str("toothache"), py_str("tooth"), py_str("teeth"), py_str("pain"), py_str("hurts"), py_str("emergency"), py_str("check up"), py_str("check-up"), py_str("filling"), py_str("crown"), py_str("root canal"), py_str("hygienist's"), py_str("appointment"), py_str("cost"), py_str("much"), py_str("how much"), py_str("salary"), py_str("paid"), py_str("pay"), py_str("worth it"), py_str("nhs")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-dentist-career-length")); __args.push(py_tuple(vec![py_tuple(vec![py_str("become a dentist"), py_str("becoming a dentist"), py_str("be a dentist"), py_str("train as a dentist"), py_str("training as a dentist"), py_str("study dentistry"), py_str("studying dentistry"), py_str("dentistry degree"), py_str("dental school")]), py_tuple(vec![py_str("how long"), py_str("how many years"), py_str("years"), py_str("long does it take"), py_str("long does it")])])); __args.push(py_str("Five years at dental school, then a year of dental foundation training in a practice before you can work on your own, so six years from starting to being paid as a dentist. The entry grades are the thing that decides it: A-level chemistry and biology are wanted by most schools, and the UCAT test plus an interview come before any offer.")); __kws.extend(vec![("avoid".to_string(), __data67()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__DRUGS(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("nurofen"), py_str("co codamol"), py_str("co-codamol"), py_str("co codrydamol"), py_str("codeine"), py_str("aspirin"), py_str("painkiller"), py_str("painkillers"), py_str("calpol"), py_str("neurofen")]));
    fn __data68() -> Py { py_tuple(vec![py_str("overdose"), py_str("overdosed"), py_str("took too much"), py_str("whole packet"), py_str("allergic"), py_str("reaction"), py_str("child"), py_str("kid"), py_str("baby is"), py_str("my baby"), py_str("son"), py_str("daughter"), py_str("dog"), py_str("cat")]) }
    fn __data69() -> Py { py_tuple(vec![py_str("antibiotic"), py_str("antibiotics"), py_str("heartburn"), py_str("indigestion"), py_str("reflux"), py_str("antacid"), py_str("gaviscon"), py_str("sickness"), py_str("nausea"), py_str("vomiting"), py_str("constipation"), py_str("laxative"), py_str("hayfever"), py_str("hay fever"), py_str("antihistamine"), py_str("antihistamines"), py_str("thrush"), py_str("uti")]) }
    fn __data70() -> Py { py_tuple(vec![py_str("stomach"), py_str("ulcer"), py_str("kidney"), py_str("asthma"), py_str("blood thinner"), py_str("warfarin"), py_str("alcohol"), py_str("drink"), py_str("drinking"), py_str("beer"), py_str("wine"), py_str("pint"), py_str("hangover"), py_str("sleepy"), py_str("drowsy"), py_str("thin your blood"), py_str("how long"), py_str("overdose"), py_str("took too much"), py_str("child"), py_str("kid"), py_str("baby"), py_str("dog"), py_str("cat"), py_str("pregnant"), py_str("breastfeeding"), py_str("allergic"), py_str("hangover"), py_str("period pain not"), py_str("newborn"), py_str("newborns"), py_str("new born"), py_str("neonate"), py_str("infant"), py_str("month old"), py_str("months old"), py_str("week old"), py_str("weeks old")]) }
    fn __data71() -> Py { py_tuple(vec![py_str("child"), py_str("kid"), py_str("baby"), py_str("toddler"), py_str("year old"), py_str("son"), py_str("daughter"), py_str("calpol"), py_str("calprofen"), py_str("nurofen for children"), py_str("childrens"), py_str("children's"), py_str("junior"), py_str("give him"), py_str("give her"), py_str("give them"), py_str("give my"), py_str("shall i give"), py_str("can i give"), py_str("do i give"), py_str("for my son"), py_str("for my daughter"), py_str("for my baby"), py_str("dog"), py_str("cat"), py_str("overdose"), py_str("overdosed"), py_str("took too many"), py_str("too many"), py_str("whole packet"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("allergic"), py_str("reaction"), py_str("how many days"), py_str("how long can i take"), py_str("newborn"), py_str("newborns"), py_str("new born"), py_str("neonate"), py_str("neonates"), py_str("infant"), py_str("infants"), py_str("month old"), py_str("months old"), py_str("week old"), py_str("weeks old"), py_str("under 3 months"), py_str("under three months"), py_str("breastfed"), py_str("formula fed")]) }
    let __aug72 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkillers-pregnancy")); __args.push(py_tuple(vec![g__DRUGS(), py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting a baby"), py_str("expecting"), py_str("weeks pregnant"), py_str("trimester"), py_str("while im pregnant"), py_str("having a baby")])])); __args.push(py_str("Paracetamol is the one to use in pregnancy: the lowest dose that works, for as short a time as you can. Avoid ibuprofen, especially after 20 weeks, and don't take aspirin unless a doctor has prescribed it for you. For anything else - a headache that keeps coming back, heartburn, a bad back - ask your midwife or a pharmacist, who will check it against your stage.")); __kws.extend(vec![("avoid".to_string(), __data68())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkillers-breastfeeding")); __args.push(py_tuple(vec![g__DRUGS(), py_tuple(vec![py_str("breastfeeding"), py_str("breast feeding"), py_str("nursing"), py_str("feeding my baby"), py_str("chestfeeding"), py_str("while im feeding")])])); __args.push(py_str("Paracetamol and ibuprofen are both fine while you are breastfeeding - ibuprofen is the one usually suggested for pain with swelling, taken with food. Avoid aspirin, and go easy on co-codamol and other codeine tablets, which can make a baby drowsy. Check what else is in any cold remedy you take, and a pharmacist will check anything else for free.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("overdose"), py_str("overdosed"), py_str("took too much"), py_str("whole packet"), py_str("allergic"), py_str("reaction"), py_str("son"), py_str("daughter"), py_str("dog"), py_str("cat")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-can-i-take-pregnant-or-feeding")); __args.push(py_tuple(vec![py_tuple(vec![py_str("what can i take"), py_str("can i take anything"), py_str("is there anything i can take"), py_str("anything safe"), py_str("is it safe to take"), py_str("what can i have"), py_str("whats safe to take"), py_str("whats safe")]), py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("weeks pregnant"), py_str("trimester"), py_str("while im pregnant"), py_str("breastfeeding"), py_str("breast feeding"), py_str("nursing"), py_str("while im feeding")])])); __args.push(py_func("_what_can_i_take_pregnant_or_feeding", |args: &[Py]| _what_can_i_take_pregnant_or_feeding(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g__DRUGS(), &__data69()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkillers-which-one")); __args.push(py_tuple(vec![py_tuple(vec![py_str("paracetamol or ibuprofen"), py_str("ibuprofen or paracetamol"), py_str("paracetamol and ibuprofen which"), py_str("which painkiller"), py_str("which is better paracetamol"), py_str("which one is better ibuprofen"), py_str("ibuprofen or paracetamol for a headache")]), py_tuple(vec![py_str("which"), py_str("better"), py_str("or"), py_str("should"), py_str("for a"), py_str("vs"), py_str("versus")])])); __args.push(py_str("Ibuprofen suits pain with swelling or inflammation - a sprain, period pain, toothache - taken with food. Paracetamol suits a plain headache or a temperature, and it is the one to use if ibuprofen upsets your stomach, you are pregnant, or you take a blood thinner. If one alone is not enough they can be taken together, on their own schedules.")); __kws.extend(vec![("avoid".to_string(), __data70())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkillers-dose")); __args.push(py_tuple(vec![g__DRUGS(), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how often can i take"), py_str("dose"), py_str("dosage"), py_str("maximum"), py_str("max"), py_str("safe amount"), py_str("a day"), py_str("per day"), py_str("in 24 hours"), py_str("at a time")])])); __args.push(py_func("_dose_for_the_drug_named", |args: &[Py]| _dose_for_the_drug_named(args))); __kws.extend(vec![("avoid".to_string(), __data71())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pregnant-swollen-ankles")); __args.push(py_tuple(vec![g__PREGNANCY_SWELLING(), g__PREGNANCY_SWELLING_PARTS(), g__PREGNANCY_WORDS()])); __args.push(g__PREGNANCY_SWELLING_REPLY()); __kws.extend(vec![("avoid".to_string(), g__PREGNANCY_SWELLING_AVOID()), ("bias".to_string(), Py::Int(6)), ("guard".to_string(), py_func("_names_a_pregnancy", |args: &[Py]| _names_a_pregnancy(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pregnant-swollen-ankles-in-weeks")); __args.push(op_add(&g__PREGNANCY_SWELLING(), &g__PREGNANCY_SWELLING_PARTS())); __args.push(g__PREGNANCY_SWELLING_REPLY()); __kws.extend(vec![("avoid".to_string(), g__PREGNANCY_SWELLING_AVOID()), ("bias".to_string(), Py::Int(6)), ("shape".to_string(), py_tuple(vec![py_func("_a_figure_in_weeks", |args: &[Py]| _a_figure_in_weeks(args))])), ("guard".to_string(), py_func("_dates_a_pregnancy_in_weeks", |args: &[Py]| _dates_a_pregnancy_in_weeks(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug72);
    fn __data73() -> Py { py_set(vec![py_str("tooth"), py_str("teeth"), py_str("toothache"), py_str("filling"), py_str("fillings"), py_str("crown"), py_str("crowns"), py_str("extraction"), py_str("extracted"), py_str("gum"), py_str("gums"), py_str("denture"), py_str("dentures"), py_str("wisdom"), py_str("brace"), py_str("braces"), py_str("root canal"), py_str("enamel"), py_str("abscess"), py_str("chipped"), py_str("chipped tooth")]) }
    s_TOOTH_WORDS(__data73());
    fn __data74() -> Py { py_set(vec![py_str("book"), py_str("booking"), py_str("booked"), py_str("appointment"), py_str("appointments"), py_str("ring"), py_str("ringing"), py_str("call"), py_str("phone"), py_str("see"), py_str("seen"), py_str("register"), py_str("registering"), py_str("register"), py_str("get in"), py_str("joined"), py_str("joining"), py_str("new"), py_str("urgent"), py_str("emergency"), py_str("pain"), py_str("hurts"), py_str("hurting")]) }
    s_SEEN_ABOUT_IT(__data74());
    s__DECISION_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|do|would|could)\\s+i\\b[^.?!]*\\b(?:see|go|get|be seen|have it|have this|bother|worry)\\b|\\bis it worth\\b|\\bworth (?:seeing|going|getting|a trip)\\b|\\bdo i need to\\b|\\bneed i\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data75() -> Py { py_tuple(vec![py_str("book"), py_str("books"), py_str("booking"), py_str("booked"), py_str("appointment"), py_str("appointments"), py_str("register"), py_str("registering"), py_str("registration"), py_str("online"), py_str("app"), py_str("phone"), py_str("ring"), py_str("call"), py_str("surgery"), py_str("practice"), py_str("clinic"), py_str("cancellation"), py_str("cancellations"), py_str("slot"), py_str("slots"), py_str("reception"), py_str("receptionist"), py_str("walk in"), py_str("111")]) }
    s__THE_BOOKING_WORDS(call_builtin("frozenset", &[__data75()]).unwrap_or_else(|e| exc::raise(e)));
    s__A_RETURN_TO(py_tuple(vec![py_str("back to"), py_str("back at"), py_str("going back"), py_str("go back"), py_str("went back"), py_str("come back"), py_str("coming back"), py_str("got back")]));
    fn __data76() -> Py { py_tuple(vec![py_str("appointment"), py_str("appointments"), py_str("doctor"), py_str("dentist"), py_str("gp"), py_str("surgery"), py_str("doctors practice"), py_str("doctor's practice"), py_str("doctors' practice"), py_str("gp practice"), py_str("medical practice"), py_str("the practice"), py_str("my practice"), py_str("our practice"), py_str("local practice"), py_str("clinic"), py_str("checkup"), py_str("check up")]) }
    fn __data77() -> Py { py_tuple(vec![py_str("book"), py_str("booking"), py_str("make"), py_str("made"), py_str("get"), py_str("arrange"), py_str("appointment"), py_str("see"), py_str("any appointments"), py_str("never has"), py_str("no appointments"), py_str("nothing available"), py_str("fully booked"), py_str("cant get in"), py_str("can't get in"), py_str("none left"), py_str("all booked")]) }
    fn __data78() -> Py { py_tuple(vec![py_str("often"), py_str("frequently"), py_str("regular"), py_str("regularly"), py_str("times"), py_str("interval"), py_str("register"), py_str("registering"), py_str("registration"), py_str("sign up"), py_str("signed up"), py_str("signup"), py_str("off"), py_str("leave"), py_str("holiday"), py_str("boss"), py_str("cover"), py_str("sick"), py_str("ill"), py_str("unwell"), py_str("chest pain"), py_str("bleeding"), py_str("cant breathe"), py_str("can't breathe"), py_str("111"), py_str("999"), py_str("a&e"), py_str("waiting room"), py_str("nervous"), py_str("scared"), py_str("dreading"), py_str("terrified"), py_str("terrifying"), py_str("frightened"), py_str("phobia"), py_str("afford"), py_str("cost"), py_str("how much"), py_str("free"), py_str("is it free"), py_str("do i pay"), py_str("do i have to pay"), py_str("haircut"), py_str("hair cut"), py_str("hair appointment"), py_str("barber"), py_str("barbers"), py_str("hairdresser"), py_str("salon"), py_str("nails"), py_str("manicure"), py_str("pedicure"), py_str("waxing"), py_str("massage"), py_str("facial"), py_str("hair colour"), py_str("colour"), py_str("restaurant"), py_str("table")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("book-an-appointment")); __args.push(py_tuple(vec![__data76(), __data77()])); __args.push(py_func("_book_the_appointment", |args: &[Py]| _book_the_appointment(args))); __kws.extend(vec![("guard".to_string(), py_func("_a_decision_is_not_a_booking", |args: &[Py]| _a_decision_is_not_a_booking(args))), ("avoid".to_string(), __data78())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_SEA_WATER_ALARM(stdlib_call("re", "compile", &[py_str("\\b(?:sick|sickness|throwing up|thrown up|threw up|vomit|vomiting|vomited|dizzy|dizziness|confused|confusion|drowsy|lethargic|floppy|faint|fainted|fainting|unwell|poorly|too much|lots|loads|plenty|gallons|whole bottle|a lot)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SEA_WATER_ALARM_REPLY(py_str("Ring 111 now: being sick, dizzy or confused after swallowing sea water needs a doctor today, and 999 if they are drowsy, floppy or hard to wake. Never make anyone sick to get it out. A mouthful or two does no harm - the salt just makes you thirsty, so rinse your mouth and drink fresh water - and boiling it is no help, because the water steams off and the salt stays."));
    fn __data79() -> Py { py_tuple(vec![py_str("drank"), py_str("drink"), py_str("drinking"), py_str("swallowed"), py_str("swallow"), py_str("mouthful"), py_str("gulp"), py_str("gulped"), py_str("sick"), py_str("vomiting"), py_str("thirsty"), py_str("sipped"), py_str("safe"), py_str("harm"), py_str("boil"), py_str("boiling"), py_str("boiled")]) }
    fn __data80() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("puppies"), py_str("kitten"), py_str("pup"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("hamster"), py_str("rabbit"), py_str("horse"), py_str("pony"), py_str("livestock"), py_str("wash"), py_str("washing"), py_str("rinsed the"), py_str("soak"), py_str("gargle"), py_str("boil pasta"), py_str("boil potatoes"), py_str("boiled pasta"), py_str("cooking water"), py_str("for pasta"), py_str("to boil"), py_str("boiling pasta"), py_str("potatoes"), py_str("bath"), py_str("drown"), py_str("drowned"), py_str("swim"), py_str("swimming"), py_str("swimmer"), py_str("paddle"), py_str("paddling"), py_str("lido"), py_str("beach"), py_str("tide"), py_str("rip current"), py_str("wetsuit")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("body-sea-water")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sea water"), py_str("salt water"), py_str("saltwater"), py_str("seawater"), py_str("salty water"), py_str("sea")]), __data79()])); __args.push(py_func("_sea_water_reply", |args: &[Py]| _sea_water_reply(args))); __kws.extend(vec![("avoid".to_string(), __data80())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_REGION_BIAS(py_dict(vec![(py_str("toe"), Py::Int(3))]));
    fn __data81() -> Py { py_tuple(vec![py_str("run"), py_str("runs"), py_str("running"), py_str("jog"), py_str("jogging"), py_str("jogged"), py_str("ran"), py_str("miles"), py_str("mileage"), py_str("after a run"), py_str("my run"), py_str("long run"), py_str("treadmill"), py_str("triathlon")]) }
    let _ = py_setitem(&g_EXTRA_AVOID(), &py_str("toe"), __data81());
    s_OBJECT_OWNERS(call_builtin("frozenset", &[mcall(&py_str("\ntable tables chair chairs desk desks bed beds sofa sofas couch couches settee settees stool\nstools shelf shelves cupboard cupboards wardrobe wardrobes drawer drawers door doors gate\ngates fence fences car cars bike bikes bicycle bicycles pram prams buggy buggies wheelchair\nwheelchairs glasses sunglasses shoe shoes boot boots trainer trainers sandal sandals ladder\nladders machine machines printer printers laptop laptops phone phones window windows wall\nwalls floor floors ceiling ceilings roof roofs oven ovens fridge fridges\nbag bags strap strap braces brace pillow pillows sleeve sleeves\nchicken chickens turkey turkeys duck ducks goose lamb fish\nbedroom bedrooms room rooms lounge kitchen bathroom loft attic landing hallway study office\ngarage garages garden gardens\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_PARTS(call_builtin("frozenset", &[mcall(&py_str("\nleg legs back arm arms heel heels head foot feet toe toes knee knees hand hands finger\nfingers thumb elbow shoulder hip shin spine joint neck sole\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__PERSON_OWNER(op_bitor(&pyrt::xrefs::xget("understanding.kinds", "PERSON_NOUNS"), &call_builtin("frozenset", &[mcall(&py_str("my your his her their our whose"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s__DETERMINERS(call_builtin("frozenset", &[mcall(&py_str("the a an this that these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HURT_WORDS(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HURT()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&word.clone(), &py_str(" ")))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__POSSESSIVES(call_builtin("frozenset", &[mcall(&py_str("my your his her its their our"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data82() -> Py { py_tuple(vec![py_str("what should i do about"), py_str("what should i do for"), py_str("what can i do about"), py_str("what can i do for"), py_str("what do i do about"), py_str("what do i do for"), py_str("what can i do"), py_str("what should i do"), py_str("whats the best thing for"), py_str("what is the best thing for"), py_str("best thing for"), py_str("best thing to do for"), py_str("anything i can do about"), py_str("anything i can do for"), py_str("is there anything for"), py_str("is there anything i can do"), py_str("what helps"), py_str("what helps with"), py_str("what helps a"), py_str("what helps my"), py_str("what is wrong with"), py_str("whats wrong with"), py_str("what's wrong with"), py_str("what could be wrong with"), py_str("what might be wrong with"), py_str("whats the matter with"), py_str("what is the matter with"), py_str("whats going on with"), py_str("what is going on with"), py_str("whats good for"), py_str("what is good for"), py_str("whats wrong with"), py_str("what is wrong with"), py_str("what could be wrong with"), py_str("what might be wrong with"), py_str("what do you think is wrong with"), py_str("any idea whats wrong with"), py_str("any idea what is wrong with"), py_str("what have i done to"), py_str("what did i do to my"), py_str("whats up with my"), py_str("is something wrong with my"), py_str("should i worry about my"), py_str("how bad is my"), py_str("is it broken"), py_str("is it fractured"), py_str("is it dislocated"), py_str("have i broken"), py_str("have i fractured"), py_str("did i break"), py_str("did i fracture"), py_str("is it torn"), py_str("have i torn"), py_str("is it snapped"), py_str("remedy for"), py_str("any tips for"), py_str("any advice for"), py_str("how do i ease"), py_str("how do i relieve"), py_str("what exercises"), py_str("which exercises"), py_str("any exercises"), py_str("best exercises"), py_str("good exercises"), py_str("exercises for"), py_str("exercise for"), py_str("exercises to"), py_str("exercises help"), py_str("exercises that help"), py_str("what exercise"), py_str("which exercise"), py_str("best exercise"), py_str("any exercise"), py_str("what stretches"), py_str("which stretches"), py_str("any stretches"), py_str("best stretches"), py_str("good stretches"), py_str("stretches for")]) }
    s_ASKING_ABOUT_A_PART(__data82());
    s_ASKED_AGAIN(py_tuple(vec![py_str("what about my"), py_str("how about my"), py_str("what about our"), py_str("how about our"), py_str("what about your"), py_str("how about your")]));
    fn __data83() -> Py { py_tuple(vec![py_str("come"), py_str("comes"), py_str("coming"), py_str("came"), py_str("go"), py_str("goes"), py_str("going"), py_str("went"), py_str("gone"), py_str("bring"), py_str("brings"), py_str("bringing"), py_str("brought"), py_str("send"), py_str("sends"), py_str("sending"), py_str("sent"), py_str("put"), py_str("puts"), py_str("putting"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("take"), py_str("takes"), py_str("taking"), py_str("took"), py_str("taken"), py_str("give"), py_str("gives"), py_str("giving"), py_str("gave"), py_str("given"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("hold"), py_str("holds"), py_str("holding"), py_str("held"), py_str("pay"), py_str("pays"), py_str("paying"), py_str("paid"), py_str("call"), py_str("calls"), py_str("calling"), py_str("called"), py_str("ring"), py_str("rings"), py_str("ringing"), py_str("rang"), py_str("phone"), py_str("phones"), py_str("phoning"), py_str("phoned"), py_str("text"), py_str("texts"), py_str("texting"), py_str("texted"), py_str("write"), py_str("writes"), py_str("writing"), py_str("wrote"), py_str("hand"), py_str("hands"), py_str("handing"), py_str("be"), py_str("is"), py_str("was"), py_str("are"), py_str("were"), py_str("being"), py_str("been"), py_str("way"), py_str("step"), py_str("steps"), py_str("far"), py_str("cut"), py_str("cuts"), py_str("cutting"), py_str("fight"), py_str("fights"), py_str("fighting"), py_str("hit"), py_str("hits"), py_str("hitting")]) }
    s__BACK_VERB_BEFORE(call_builtin("frozenset", &[__data83()]).unwrap_or_else(|e| exc::raise(e)));
    s__ALSO_THE_BACK(call_builtin("frozenset", &[py_tuple(vec![py_str("backache"), py_str("back's"), py_str("spine"), py_str("lumbar"), py_str("sciatica"), py_str("pelvis"), py_str("hips")])]).unwrap_or_else(|e| exc::raise(e)));
    s__BACK_IS_A_NOUN_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("pain"), py_str("aches"), py_str("ache"), py_str("trouble"), py_str("problems"), py_str("problem"), py_str("soreness"), py_str("strain"), py_str("spasms"), py_str("spasm")])]).unwrap_or_else(|e| exc::raise(e)));
    s__BACK_IS_A_RETURN(call_builtin("frozenset", &[py_tuple(vec![py_str("to"), py_str("into"), py_str("onto"), py_str("towards"), py_str("toward")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data84() -> Py { py_tuple(vec![py_str("my"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("your"), py_str("the"), py_str("a"), py_str("an"), py_str("its"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("mine"), py_str("hers"), py_str("theirs"), py_str("ours")]) }
    s__BACK_THEIRS(call_builtin("frozenset", &[__data84()]).unwrap_or_else(|e| exc::raise(e)));
    s__BACK_DESCRIBED(call_builtin("frozenset", &[mcall(&py_str("\nstiff sore bad aching achy painful lower upper middle tight weak dodgy old broken fragile seized\nknackered tired poor\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__BACK_TAKING_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nwant wants wanted wanting need needs needed needing ask asks asked asking owe owes owed\nget gets getting got give gives giving gave given send sends sending sent take takes taking\ntook taken pay pays paying paid bring brings bringing brought keep keeps keeping kept put puts\nputting hand hands handing hold holds holding held call calls calling called ring rings ringing\nrang text texts texting texted write writes writing wrote throw throws threw pull pulls pulled\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__BACK_CLAUSE_JOIN(call_builtin("frozenset", &[py_tuple(vec![py_str("and"), py_str("but"), py_str("then"), py_str("so"), py_str("because"), py_str("when"), py_str("if"), py_str("before"), py_str("after"), py_str("until"), py_str("while"), py_str("or")])]).unwrap_or_else(|e| exc::raise(e)));
    s__KEPT_AWAKE(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|kept|keeping|stops?|stopped|stopping|prevents?|prevented|preventing|held|holding)\\s+me\\s+(?:up\\b|awake\\b|off\\s+my\\s+feet\\b|from\\s+\\w+ing\\b|from\\s+sleep\\b|sleeping\\b|walking\\b|standing\\b|sitting\\b|driving\\b|working\\b|lying\\b|sleep\\b)|\\b(?:wont|won t|wouldnt|would not|doesnt|does not|will not|didnt|did not)\\s+let\\s+me\\s+(?:sleep|sleeping|nap|rest|lie\\s+down|lie\\s+still|lie\\s+flat|sit\\s+down|stand\\s+up|walk|drive|get\\s+up|get\\s+out\\s+of\\s+bed|roll\\s+over)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PART_IN_THE_TABLE(stdlib_call("re", "compile", &[op_modulo(&py_str("\\b(?:%s)\\b"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_REGIONS()) { let __unpk2910 = py_unpack(&__item.clone(), 3, None); let mut _n: Py = __unpk2910[0].clone(); let mut parts: Py = __unpk2910[1].clone(); let mut _r: Py = __unpk2910[2].clone(); for __item in py_iter_or_raise(&parts.clone()) { let mut p: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[p.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]))]).unwrap_or_else(|e| exc::raise(e)));
    s_BREAK_SAID(py_tuple(vec![py_str("broken"), py_str("broke"), py_str("fractured"), py_str("fracture"), py_str("cracked"), py_str("crack"), py_str("snapped")]));
    s_A_PART_NO_KNOCK_TEST(py_tuple(vec![py_str("ribs"), py_str("tailbone"), py_str("groin")]));
    let __aug85 = op_add(&g_ENTRIES(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_REGIONS()) { let __unpk4957 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk4957[0].clone(); let mut parts: Py = __unpk4957[1].clone(); let mut reply: Py = __unpk4957[2].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __s = String::new(); __s.push_str("sore-"); __s.push_str(&fstr(&name.clone(), -1, "")); py_string(__s) }); __args.push(py_tuple(vec![parts.clone(), op_add(&op_add(&op_add(&g_HURT(), &g_ASKING_ABOUT_A_PART()), &g_ASKED_AGAIN()), &_the_breaks_it_says(&[name.clone()]))])); __args.push(reply.clone()); __kws.extend(vec![("avoid".to_string(), op_add(&g_AVOID(), &mcall(&g_EXTRA_AVOID(), "get", &[name.clone(), py_tuple(vec![])]))), ("bias".to_string(), mcall(&g_REGION_BIAS(), "get", &[name.clone(), Py::Int(0)])), ("guard".to_string(), _a_region_guard(&[name.clone()])), ("shape".to_string(), py_tuple(vec![py_func("_a_part_keeps_me_from_something", |args: &[Py]| _a_part_keeps_me_from_something(args))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() });
    s_ENTRIES(__aug85);
    fn __data86() -> Py { py_tuple(vec![py_str("arm"), py_str("arms"), py_str("forearm"), py_str("leg"), py_str("legs"), py_str("thigh"), py_str("thighs"), py_str("calf"), py_str("calves"), py_str("shin"), py_str("shins"), py_str("wrist"), py_str("wrists"), py_str("hand"), py_str("hands"), py_str("elbow"), py_str("elbows"), py_str("shoulder"), py_str("shoulders"), py_str("knee"), py_str("knees"), py_str("ankle"), py_str("ankles"), py_str("foot"), py_str("feet"), py_str("thumb"), py_str("thumbs"), py_str("finger"), py_str("fingers"), py_str("collarbone")]) }
    s_SOFT_TISSUE(__data86());
    s__SOMEONE_ELSE(call_builtin("frozenset", &[mcall(&py_str("\nhe him his she her hers they them their theirs son daughter child kid kids baby toddler\nhusband wife partner boyfriend girlfriend mum mom mother dad father brother sister friend mate\nnephew niece grandson granddaughter grandad grandma granny uncle aunt cousin\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_WHILE_AGO(stdlib_call("re", "compile", &[py_str("\\b(?:last|past)\\s+(?:week|month|year|summer|winter|christmas|autumn|spring)\\b|\\b(?:a|one|two|three|four|five|six|seven|eight|nine|ten|\\d+)\\s*(?:days?|weeks?|months?|years?)\\s+ago\\b|\\b(?:a\\s+)?(?:few|couple\\s+of)\\s+(?:weeks|months|years)\\s+ago\\b|\\b(?:weeks|months|years) ago\\b|\\byears back\\b|\\byears since\\b|\\bwhen (?:he|she|they|my \\w+) (?:was|were)\\b|\\bused to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__IT_HAPPENED_TO(call_builtin("frozenset", &[mcall(&py_str("\nfell fallen falling fell trip tripped tumbling tumbled landed hit smacked banged bashed\nknocked injured sprained twisted cricked wrenched\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__TO_HURT_A_PART(call_builtin("frozenset", &[mcall(&py_str("hurt hurting"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data87() -> Py { py_tuple(vec![py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("injured"), py_str("banged"), py_str("knocked"), py_str("bashed"), py_str("fell"), py_str("fell off"), py_str("fell over"), py_str("fallen"), py_str("tripped"), py_str("tumbled"), py_str("landed on")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hurt-limb-someone")); __args.push(py_tuple(vec![__data87(), g_SOFT_TISSUE()])); __args.push(py_str("Check it where they are: whether it is swollen or bruising fast, whether they will use it at all, and whether it looks bent. Ice it for 15 minutes, keep it raised, and give children's paracetamol if their age allows. A limb they will not use or let you move, one that looks bent, or swelling that comes up fast is A&E the same day; a bang on the head with sickness, drowsiness or a knockout is 999.")); __kws.extend(vec![("guard".to_string(), py_func("_it_is_somebody_elses_part", |args: &[Py]| _it_is_somebody_elses_part(args))), ("avoid".to_string(), py_tuple(vec![py_str("dog"), py_str("cat"), py_str("puppy"), py_str("horse"), py_str("kitten")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data88() -> Py { py_tuple(vec![py_str("for a week"), py_str("for two weeks"), py_str("for a fortnight"), py_str("for three weeks"), py_str("for weeks"), py_str("for a month"), py_str("for months"), py_str("for a year"), py_str("for years"), py_str("for ages"), py_str("for a while"), py_str("all week"), py_str("all month"), py_str("a week now"), py_str("weeks now"), py_str("months now"), py_str("a few weeks"), py_str("a couple of weeks"), py_str("two weeks"), py_str("three weeks"), py_str("weeks"), py_str("months"), py_str("the last few months"), py_str("the past few months"), py_str("the last few weeks"), py_str("the past few weeks"), py_str("on and off for")]) }
    s_WEEKS(__data88());
    fn __data89() -> Py { py_tuple(vec![py_str("run"), py_str("runs"), py_str("running"), py_str("ran"), py_str("jog"), py_str("jogging"), py_str("jogged"), py_str("miles"), py_str("treadmill"), py_str("gym"), py_str("workout"), py_str("work out"), py_str("weights"), py_str("cycling"), py_str("cycle"), py_str("swim"), py_str("swimming"), py_str("football"), py_str("rugby"), py_str("netball"), py_str("hockey"), py_str("tennis"), py_str("marathon"), py_str("half marathon"), py_str("parkrun"), py_str("training"), py_str("training for"), py_str("after every run")]) }
    s__SPORT_WORDS(__data89());
    fn __data90() -> Py { py_dict(vec![(py_str("back"), py_str("two short sessions a week of a slow bridge lying on your back, or standing and lifting one knee at a time")), (py_str("spine"), py_str("two short sessions a week of a slow bridge lying on your back")), (py_str("knee"), py_str("two short sessions a week of sit-to-stand from a chair, ten times each, plus a walk")), (py_str("kneecap"), py_str("two short sessions a week of sit-to-stand from a chair, ten times each")), (py_str("hip"), py_str("two short sessions a week of sit-to-stand from a chair and a walk")), (py_str("shoulder"), py_str("two short sessions a week of a slow wall push-up, and the pendulum swing (lean forward, let the arm hang, circle it slowly)")), (py_str("neck"), py_str("chin tucks, ten at a time, twice a day, and a look at where your screen sits")), (py_str("foot"), py_str("a calf stretch against a wall before your first steps in the morning, and a rolled towel under the arch")), (py_str("heel"), py_str("a calf stretch against a wall before your first steps, and shoes with a firm sole")), (py_str("ankle"), py_str("heel raises holding a chair, ten at a time")), (py_str("wrist"), py_str("a slow curl with a light weight, palm up and palm down, and breaks from gripping")), (py_str("hand"), py_str("a slow curl with a light weight, and breaks from gripping")), (py_str("thumb"), py_str("a slow curl with a light weight, and breaks from gripping")), (py_str("elbow"), py_str("a light weight lifted slowly, palm down then palm up")), (py_str("leg"), py_str("two short sessions a week of a slow squat to a chair, plus a walk")), (py_str("calf"), py_str("two short sessions a week of a slow squat to a chair and heel raises")), (py_str("thigh"), py_str("two short sessions a week of a slow squat to a chair, plus a walk")), (py_str("hamstring"), py_str("a slow squat to a chair and a gentle hamstring stretch after a warm bath")), (py_str("shin"), py_str("a walk, and calf raises holding a chair")), (py_str("rib"), py_str("keeping your breathing normal rather than in shallow sips, and a cushion pressed against that side when you cough")), (py_str("toe"), py_str("shoes with room in them, and a warm soak"))]) }
    s__WEEKS_DRILL(__data90());
    s__PREGNANT_WORDS(py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("trimester"), py_str("baby due")]));
    s__BACK_AFTER_A_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|is|are|be|been|being|am|im|came|come|comes|coming|went|go|goes|going|gone|get|gets|getting|got|give|gives|giving|gave|given|takes?|took|taken|brings?|brought|sends?|sent|pays?|paid|calls?|called|sits?|sat|cuts?|holds?|held|puts?|stepped|steps?|looks?|looked|turns?|turned|fights?|fought|throws?|threw|pulls?|pulled|comes?|coming|mov(?:e|es|ed|ing)|relocat(?:e|es|ed|ing))\\s+(?:(?:it|them|him|her|me|us|you|one)\\s+)?back\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BACK_BEFORE_A_PLACE(stdlib_call("re", "compile", &[py_str("(?<!\\bmy )(?<!\\bhis )(?<!\\bher )(?<!\\byour )(?<!\\btheir )(?<!\\bour )(?<!\\bthe )(?<!\\bthis )(?<!\\bthat )(?<!\\ba )(?<!\\ban )\\bback\\s+(?:at|to|from|in|on)\\s+(?:the\\s+|my\\s+|her\\s+|his\\s+)?(?:work|school|home|college|uni|university|office|nursery|holidays?|shift|shifts|placement|term|nights?|days?|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BACK_HOME(stdlib_call("re", "compile", &[py_str("(?<!\\bmy )(?<!\\bhis )(?<!\\bher )(?<!\\byour )(?<!\\btheir )(?<!\\bour )(?<!\\bthe )(?<!\\bthis )(?<!\\bthat )(?<!\\ba )(?<!\\ban )\\bback\\s+(?:home|in\\s+with|with\\s+(?:my|her|his|our|the)\\s+(?:parents|mum|dad|mother|father|family|folks)|to\\s+(?:my|her|his|our)\\s+(?:parents|mum|dad|mother|father|home))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data91() -> Py { py_tuple(vec![py_str("see someone"), py_str("see a doctor"), py_str("see the doctor"), py_str("see a gp"), py_str("see my gp"), py_str("go to the doctor"), py_str("go to a doctor"), py_str("go to my gp"), py_str("should i see"), py_str("do i need to see"), py_str("worth seeing"), py_str("see a physio"), py_str("see the physio"), py_str("get it looked at"), py_str("get it seen"), py_str("should i get it checked"), py_str("worth getting checked"), py_str("checked out"), py_str("get it checked"), py_str("get my checked"), py_str("should i go to the doctor"), py_str("do i need to get it checked"), py_str("worth going to the"), py_str("should i be worried"), py_str("is it worth going"), py_str("get it looked at"), py_str("get my"), py_str("looked at"), py_str("looked over"), py_str("get it seen to"), py_str("should i bother"), py_str("bother going"), py_str("bother seeing"), py_str("any point in"), py_str("worth it to"), py_str("worth a visit"), py_str("worth a trip"), py_str("worth making an appointment")]) }
    s_DECIDE_ASK(__data91());
    fn __data92() -> Py { py_tuple(vec![py_str("lump"), py_str("lumps"), py_str("gland"), py_str("glands"), py_str("node"), py_str("nodes"), py_str("breast"), py_str("lump on"), py_str("chest"), py_str("heart"), py_str("throat"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("ear"), py_str("ears"), py_str("head"), py_str("headache"), py_str("headaches"), py_str("migraine"), py_str("cough"), py_str("cold"), py_str("flu"), py_str("sore throat"), py_str("stomach"), py_str("tummy"), py_str("belly"), py_str("toothache"), py_str("jaw"), py_str("period"), py_str("periods"), py_str("pregnant"), py_str("pregnancy"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("toddler"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("friend"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("sick"), py_str("nauseous"), py_str("vomit"), py_str("dizzy"), py_str("faint"), py_str("breathless"), py_str("sweating"), py_str("numb"), py_str("numbness"), py_str("pins and needles"), py_str("cant pee"), py_str("can't pee"), py_str("blood"), py_str("bleeding"), py_str("gave way"), py_str("gives way"), py_str("locked"), py_str("locks"), py_str("swollen"), py_str("swelling"), py_str("hot and swollen"), py_str("go back"), py_str("going back"), py_str("went back"), py_str("come back"), py_str("coming back"), py_str("back to"), py_str("back at"), py_str("nursing"), py_str("career"), py_str("careers"), py_str("retrain"), py_str("retraining"), py_str("job"), py_str("jobs"), py_str("degree"), py_str("university"), py_str("college"), py_str("course"), py_str("shift"), py_str("my old")]) }
    s_SEE_DOCTOR_AVOID(op_add(&__data92(), &g_FALL()));
    fn __data93() -> Py { py_tuple(vec![py_str("back"), py_str("leg"), py_str("legs"), py_str("knee"), py_str("knees"), py_str("hip"), py_str("hips"), py_str("shoulder"), py_str("shoulders"), py_str("neck"), py_str("ankle"), py_str("ankles"), py_str("wrist"), py_str("wrists"), py_str("elbow"), py_str("elbows"), py_str("foot"), py_str("feet"), py_str("arm"), py_str("arms"), py_str("hand"), py_str("hands"), py_str("joint"), py_str("joints"), py_str("side"), py_str("spine"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("calf"), py_str("heels"), py_str("heel"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("thumbs"), py_str("pain"), py_str("pains"), py_str("hurts"), py_str("hurting"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("sore")]) }
    s_SEE_DOCTOR_PARTS(__data93());
    s_ACHE_WORDS(py_tuple(vec![py_str("ache"), py_str("aches"), py_str("aching"), py_str("achey"), py_str("achy"), py_str("sore"), py_str("soreness"), py_str("stiff"), py_str("stiffness"), py_str("tender")]));
    fn __data94() -> Py { py_tuple(vec![py_str("after the gym"), py_str("after gym"), py_str("after weights"), py_str("after lifting"), py_str("after a workout"), py_str("after my workout"), py_str("after the workout"), py_str("after exercise"), py_str("after exercising"), py_str("after a session"), py_str("after a class"), py_str("after training"), py_str("after running"), py_str("after a run"), py_str("after the run"), py_str("after a jog"), py_str("the next day"), py_str("day after"), py_str("next day")]) }
    s_EXERCISE_AFTER(__data94());
    fn __data95() -> Py { py_tuple(vec![py_str("sore"), py_str("soreness"), py_str("aching"), py_str("achy"), py_str("achey"), py_str("ache"), py_str("aches"), py_str("stiff"), py_str("stiffness"), py_str("tender"), py_str("hurt"), py_str("hurts"), py_str("hurting")]) }
    s__SORENESS_WORDS(call_builtin("frozenset", &[__data95()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("see-about-a-part")); __args.push(py_tuple(vec![g_DECIDE_ASK(), g_SEE_DOCTOR_PARTS()])); __args.push(py_func("_see_doctor_reply", |args: &[Py]| _see_doctor_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_SEE_DOCTOR_AVOID()), ("guard".to_string(), py_func("_a_part_in_a_decision_is_a_part", |args: &[Py]| _a_part_in_a_decision_is_a_part(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__DECISION_WORDS(call_builtin("frozenset", &[mcall(&py_str("\na about all also am an and any anything are as at be been being both but by can check checked\ncould did do does doing doctor doctors each even ever for from get gets getting go goes going got gp\ngps had has have having he her here him his how i if in into is it its just look looked looking me\nmore most much must my myself need needed needs no nor not now of off on once one only or other our\nout over own rather really same see seeing seen shall she should so some someone something still\nfirst anyway\nsuch sure than that the their them then there these they thing things this those through to too\nunder up us very wait waiting was we were what when where whether which while who why will with\nworth worthwhile would yet you your\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data96() -> Py { py_tuple(vec![py_str("appointment"), py_str("book"), py_str("booking"), py_str("register"), py_str("surgery"), py_str("physio"), py_str("physiotherapy"), py_str("chiropractor"), py_str("mental"), py_str("anxiety"), py_str("anxious"), py_str("depressed"), py_str("depression"), py_str("counselling"), py_str("counselor"), py_str("counsellor"), py_str("therapy"), py_str("therapist"), py_str("panic"), py_str("stress"), py_str("stressed"), py_str("self harm"), py_str("suicidal"), py_str("dentist"), py_str("vet"), py_str("optician"), py_str("wart"), py_str("warts"), py_str("verruca"), py_str("verrucas"), py_str("weekend"), py_str("weekends"), py_str("saturday"), py_str("sunday"), py_str("bank holiday"), py_str("out of hours"), py_str("evening"), py_str("evenings")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("see-about-it")); __args.push(py_tuple(vec![g_DECIDE_ASK(), py_tuple(vec![py_str("doctor"), py_str("gp"), py_str("see someone"), py_str("someone about"), py_str("checked"), py_str("checked out"), py_str("looked at"), py_str("seen")])])); __args.push(py_func("_see_doctor_decision", |args: &[Py]| _see_doctor_decision(args))); __kws.extend(vec![("bias".to_string(), Py::Int(0)), ("guard".to_string(), py_func("_a_bare_decision", |args: &[Py]| _a_bare_decision(args))), ("ask".to_string(), py_tuple(vec![py_str("should i"), py_str("do i"), py_str("can i"), py_str("could i"), py_str("would i"), py_str("shall i"), py_str("is it"), py_str("any point"), py_str("worth")])), ("avoid".to_string(), op_add(&g_SEE_DOCTOR_AVOID(), &__data96()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data97() -> Py { py_tuple(vec![py_str("register"), py_str("registering"), py_str("registration"), py_str("taking new"), py_str("new patients"), py_str("nhs patients"), py_str("how do i get"), py_str("find a"), py_str("find an"), py_str("nearest"), py_str("near me"), py_str("how often"), py_str("how long since"), py_str("havent been"), py_str("haven't been"), py_str("never been"), py_str("years"), py_str("ages"), py_str("scared"), py_str("nervous"), py_str("afraid"), py_str("terrified"), py_str("dreading"), py_str("phobia"), py_str("embarrassed"), py_str("cost"), py_str("costs"), py_str("price"), py_str("how much"), py_str("private"), py_str("nhs band"), py_str("band 1"), py_str("band 2"), py_str("band 3"), py_str("free"), py_str("afford"), py_str("whitening"), py_str("whiten"), py_str("braces"), py_str("implant"), py_str("implants"), py_str("denture"), py_str("dentures"), py_str("appointment at"), py_str("cancel"), py_str("check up"), py_str("cold"), py_str("hot"), py_str("sweet"), py_str("bite"), py_str("biting"), py_str("chew"), py_str("chewing"), py_str("sensitive"), py_str("bleeding"), py_str("bleeds"), py_str("swollen"), py_str("swelling"), py_str("chipped"), py_str("broken"), py_str("hole"), py_str("decay"), py_str("abscess"), py_str("went"), py_str("knocked out"), py_str("fell out"), py_str("came out"), py_str("wisdom"), py_str("gums"), py_str("gum"), py_str("mouth"), py_str("jaw"), py_str("ache")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("see-a-dentist")); __args.push(py_tuple(vec![g_DECIDE_ASK(), py_tuple(vec![py_str("dentist"), py_str("dentists"), py_str("dental"), py_str("nhs dentist"), py_str("dental practice"), py_str("dental surgery"), py_str("emergency dentist"), py_str("out of hours dentist"), py_str("dentist or a doctor")])])); __args.push(py_str("Go by what the tooth is doing: a twinge with cold that settles, or a check-up you are overdue, is an ordinary appointment. Ring the dentist today if it aches at night or keeps you awake, if there is swelling in your face or under the jaw, a fever, a lost filling or crown that hurts, or bleeding that will not stop. A tooth knocked right out goes straight back into the socket, by the crown and not the root, and you are seen within the hour; ring 111 for the out-of-hours dental service if your practice is shut.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("should i"), py_str("do i"), py_str("can i"), py_str("could i"), py_str("would i"), py_str("shall i"), py_str("is it"), py_str("any point"), py_str("worth")])), ("avoid".to_string(), __data97())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__GONE_BACK(py_tuple(vec![py_str("went"), py_str("gone"), py_str("goes"), py_str("put my back out"), py_str("put it out"), py_str("pulled my back"), py_str("threw my back out"), py_str("thrown my back out"), py_str("in spasm"), py_str("locked"), py_str("did my back in")]));
    s__A_VERB_TOOK_THE_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:text|texts|texted|texting|message|messages|messaged|messaging|msg|dm|dm'd|call|calls|called|calling|ring|rings|rang|ringing|phone|phones|phoned|phoning|email|emails|emailed|emailing|write|writes|wrote|written|writing|reply|replies|replied|replying|answer|answers|answered|answering|hear|hears|heard|hearing|get|gets|getting|got|pay|pays|paid|paying|send|sends|sent|sending|bring|brings|brought|push|pushes|pushed|hold|holds|held|look|looks|looked|turn|turns|turned|talk|talks|talked|take|takes|took|step|steps|stepped|sit|sits|sat|lean|leans|leant|lie|lies|lay|fall|falls|fell|walk|walks|walked|move|moves|moved|drive|drives|drove|run|runs|ran|hit|hits|fight|fights|fought|come|comes|came|coming|go|goes|going|went|gone|stretch|stretches|stretched|stretching|shrink|shrinks|shrank|shrunk|shrinking)\\s+(?:(?:me|him|her|us|them|you|it|myself|yourself|himself|herself|themselves)\\s+)?back\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_BACK_IS_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:my|your|his|her|our|their|the|this|that)\\s+back\\b|\\b(?:sore|bad|lower|upper|stiff|aching|soreness|painful)\\s+back\\b|\\bback\\s+(?:pain|ache|aches|aching|hurts?|hurting|problem|trouble|spasm|went|gone)\\b|\\bback\\s+(?:is\\s+)?(?:killing|kills)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    let __aug98 = op_add(&g_ENTRIES(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_REGIONS()) { let __unpk6792 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk6792[0].clone(); let mut parts: Py = __unpk6792[1].clone(); let mut reply: Py = __unpk6792[2].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __s = String::new(); __s.push_str("sore-"); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str("-weeks"); py_string(__s) }); __args.push(py_tuple(vec![parts.clone(), g_WEEKS()])); __args.push(py_func("_sore_weeks_reply", |args: &[Py]| _sore_weeks_reply(args))); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AVOID()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STANDING(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g__GONE_BACK()), &g__SPORT_WORDS()), &py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("trimester"), py_str("weeks pregnant"), py_str("months pregnant")]))), ("guard".to_string(), if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("back")))) { py_func("_the_back_is_the_persons", |args: &[Py]| _the_back_is_the_persons(args)) } else { Py::None })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() });
    s_ENTRIES(__aug98);
    s__BAD_PART_MOVEMENT(py_tuple(vec![py_str("exercises"), py_str("exercise"), py_str("stretches"), py_str("stretching"), py_str("physio")]));
    s__STANDING_TROUBLE(py_tuple(vec![py_str("bad"), py_str("badly"), py_str("dodgy"), py_str("gammy"), py_str("strengthen"), py_str("strengthening"), py_str("stronger"), py_str("build up"), py_str("building up"), py_str("crap")]));
    s__BAD_PART_WORDS(op_add(&g__STANDING_TROUBLE(), &g__BAD_PART_MOVEMENT()));
    fn __data99() -> Py { py_tuple(vec![py_str("back"), py_str("spine"), py_str("knee"), py_str("kneecap"), py_str("hip"), py_str("ankle"), py_str("foot"), py_str("heel"), py_str("leg"), py_str("calf"), py_str("thigh"), py_str("hamstring"), py_str("shin"), py_str("toe")]) }
    s__CARRIES_WEIGHT(__data99());
    let __aug100 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bad-back")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_REGIONS()) { let __unpk9768 = py_unpack(&__item.clone(), 3, None); let mut _name: Py = __unpk9768[0].clone(); let mut parts: Py = __unpk9768[1].clone(); let mut _r: Py = __unpk9768[2].clone(); if py_truthy(&Py::Bool(py_eq(&_name.clone(), &py_str("back")))) {  for __item in py_iter_or_raise(&parts.clone()) { let mut w: Py = __item.clone(); __out.push(w.clone()); } } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), g__BAD_PART_WORDS()])); __args.push(py_str("A back that gives trouble on and off is usually muscular, and what keeps it quiet is strength and movement rather than rest: walk most days, and add two short sessions a week of a slow bridge lying down or sit-to-stand from a chair. Ask your GP about physio if it is stopping you sleep or work, and ring 111 the same day if you cannot pass urine, have numbness between your legs, or it came on after a fall.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("guard".to_string(), py_func("_the_back_is_the_persons", |args: &[Py]| _the_back_is_the_persons(args))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AVOID()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_STANDING(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_EXERCISE_DAY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g__GONE_BACK()), &g__SPORT_WORDS()), &g_WEEKS()), &py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("weeks pregnant"), py_str("months pregnant")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug100);
    s__REPORTED_COMPLAINT(stdlib_call("re", "compile", &[py_str("\\b(?:i (?:get|have|had|keep getting|keep|suffer from|live with)|i(?:'ve|ve) (?:got|had|been having))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let __aug101 = op_add(&g_ENTRIES(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_REGIONS()) { let __unpk8025 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk8025[0].clone(); let mut parts: Py = __unpk8025[1].clone(); let mut _reply: Py = __unpk8025[2].clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("back"), py_str("knee")]), &name.clone()))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("bad-"), &name.clone())); __args.push(py_tuple(vec![parts.clone(), g__BAD_PART_WORDS()])); __args.push(py_func("_bad_part_reply", |args: &[Py]| _bad_part_reply(args))); __kws.extend(vec![("guard".to_string(), py_func("_the_part_is_named_today", |args: &[Py]| _the_part_is_named_today(args))), ("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AVOID()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_STANDING(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_EXERCISE_DAY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g__GONE_BACK()), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SPORT_WORDS()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("swim"), py_str("swimming"), py_str("cycle"), py_str("cycling")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))), &g_WEEKS()), &py_tuple(vec![py_str("pulled"), py_str("pull"), py_str("strained"), py_str("sprain"), py_str("sprained"), py_str("torn"), py_str("tear"), py_str("injured"), py_str("injury")])), &py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("weeks pregnant"), py_str("months pregnant")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } } py_list(__out) })() });
    s_ENTRIES(__aug101);
    fn __data102() -> Py { py_tuple(vec![py_str("protect"), py_str("protects"), py_str("protecting"), py_str("protection"), py_str("look after"), py_str("looking after"), py_str("looks after"), py_str("take care of"), py_str("taking care of"), py_str("takes care of"), py_str("care for"), py_str("caring for"), py_str("keep strong"), py_str("keep it strong"), py_str("keep them strong"), py_str("keep healthy"), py_str("keep it healthy"), py_str("keep supple"), py_str("keep flexible"), py_str("avoid injury"), py_str("avoid injuring"), py_str("prevent injury"), py_str("prevent a strain")]) }
    s__PROTECT_ASK(__data102());
    s__ALL_PART_WORDS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_REGIONS()) { let __unpk138 = py_unpack(&__item.clone(), 3, None); let mut _name: Py = __unpk138[0].clone(); let mut spelled: Py = __unpk138[1].clone(); let mut _r: Py = __unpk138[2].clone(); for __item in py_iter_or_raise(&spelled.clone()) { let mut w: Py = __item.clone(); __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__PROTECT_FILLERS(call_builtin("frozenset", &[mcall(&py_str("\n    a an the and or of to for in on at with my your me it them i we you\n    how do does did can could should would whats what is are be been\n    best way ways good better keep keeps keeping well healthy strong stronger\n    supple flexible care careful carefully properly safely help home\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__PROTECT_ALLOWED(op_bitor(&op_bitor(&g__PROTECT_FILLERS(), &call_builtin("set", &[g__ALL_PART_WORDS()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PROTECT_ASK()) { let mut phrase: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&phrase.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    fn __data103() -> Py { py_tuple(vec![py_str("injury"), py_str("injured"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("sore"), py_str("swollen"), py_str("swelling"), py_str("sun"), py_str("sunburn"), py_str("sunburnt"), py_str("burn"), py_str("burnt"), py_str("burned"), py_str("suncream"), py_str("sun cream"), py_str("sunscreen"), py_str("bites"), py_str("bitten"), py_str("graze"), py_str("gravel"), py_str("burn"), py_str("sting"), py_str("stung"), py_str("eyes"), py_str("eye"), py_str("hearing"), py_str("ears"), py_str("ear"), py_str("teeth"), py_str("tooth"), py_str("gums"), py_str("feet"), py_str("foot"), py_str("heel"), py_str("toes"), py_str("toe"), py_str("plantar"), py_str("pregnant"), py_str("pregnancy"), py_str("expecting")]) }
    let __aug104 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("protect-a-part")); __args.push(py_tuple(vec![g__PROTECT_ASK(), g__ALL_PART_WORDS()])); __args.push(py_func("_protect_reply", |args: &[Py]| _protect_reply(args))); __kws.extend(vec![("guard".to_string(), py_func("_the_ask_names_nothing_else", |args: &[Py]| _the_ask_names_nothing_else(args))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_STANDING(), &g_EXERCISE_DAY()), &g_WEEKS()), &g__GONE_BACK()), &__data103()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug104);
    s_KNEE_WORDS(py_tuple(vec![py_str("knee"), py_str("knees"), py_str("kneecap"), py_str("patella")]));
    fn __data105() -> Py { py_tuple(vec![py_str("bad"), py_str("badly"), py_str("dodgy"), py_str("weak"), py_str("trouble"), py_str("troublesome"), py_str("sore"), py_str("aching"), py_str("aches"), py_str("worn"), py_str("wear and tear"), py_str("arthritis"), py_str("arthritic"), py_str("stiff"), py_str("stiffness"), py_str("exercises"), py_str("exercise"), py_str("strengthen"), py_str("strengthening")]) }
    s__STANDING_KNEE(__data105());
    fn __data106() -> Py { py_tuple(vec![py_str("twisted"), py_str("twist"), py_str("twisting"), py_str("sprain"), py_str("sprained"), py_str("tore"), py_str("torn"), py_str("pop"), py_str("popped"), py_str("swollen"), py_str("swelling"), py_str("swells"), py_str("locked"), py_str("locks"), py_str("gave way"), py_str("gives way"), py_str("gave out"), py_str("gave in"), py_str("snapped"), py_str("fell"), py_str("fall"), py_str("fallen"), py_str("stairs"), py_str("stair"), py_str("upstairs"), py_str("downstairs"), py_str("running"), py_str("run"), py_str("runner"), py_str("marathon"), py_str("football"), py_str("rugby"), py_str("squat"), py_str("squats"), py_str("operated"), py_str("operation"), py_str("replacement"), py_str("replaced"), py_str("keyhole"), py_str("arthroscopy"), py_str("injection"), py_str("steroid"), py_str("pregnant"), py_str("pregnancy"), py_str("how long"), py_str("recovery"), py_str("recover"), py_str("crutches"), py_str("brace"), py_str("support"), py_str("click"), py_str("clicks"), py_str("clicking"), py_str("clicked"), py_str("creak"), py_str("creaks"), py_str("creaking"), py_str("noise"), py_str("noisy"), py_str("grind"), py_str("grinds"), py_str("grinding"), py_str("crackle"), py_str("crackles"), py_str("crackling"), py_str("pops"), py_str("popping"), py_str("rattle"), py_str("rattles"), py_str("walk"), py_str("walking"), py_str("walked"), py_str("steps"), py_str("step count"), py_str("how far")]) }
    let __aug107 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bad-knee")); __args.push(py_tuple(vec![g_KNEE_WORDS(), g__STANDING_KNEE()])); __args.push(py_str("A knee that gives trouble is helped most by the muscle above it: sit-to-stands from a chair, step-ups onto a low step and straight-leg raises, two or three times a week, which builds the thigh muscle that takes the load off the joint. Keep walking on the flat and skip deep squats and running downhill; a physio can set the pace. A knee that locks, gives way or swells hot needs a GP.")); __kws.extend(vec![("avoid".to_string(), __data106())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug107);
    fn __data108() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("weeks"), py_str("months"), py_str("for weeks"), py_str("for months"), py_str("for a month"), py_str("for a fortnight"), py_str("a few weeks"), py_str("two weeks"), py_str("three weeks"), py_str("a week now"), py_str("for ages")]) }
    s_BACK_PREGNANCY_KEEP(__data108());
    s_PREGNANT(py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("pregnancies"), py_str("weeks pregnant"), py_str("months pregnant"), py_str("expecting a baby"), py_str("expecting"), py_str("trimester"), py_str("due in"), py_str("due next month"), py_str("due in a few weeks")]));
    fn __data109() -> Py { py_tuple(vec![py_str("back"), py_str("hurts"), py_str("hurt"), py_str("ache"), py_str("aching"), py_str("backache"), py_str("back ache"), py_str("bad back"), py_str("pain"), py_str("painful"), py_str("sore"), py_str("sciatica"), py_str("killing me"), py_str("agony")]) }
    let __aug110 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("back-pain-pregnant")); __args.push(py_tuple(vec![py_tuple(vec![py_str("back"), py_str("lower back"), py_str("spine"), py_str("backache"), py_str("hips"), py_str("pelvis"), py_str("sciatica")]), g_PREGNANT()])); __args.push(py_str("Back pain is very common in the second half of pregnancy: the weight sits out in front, and the hormone that loosens your ligaments for the birth loosens them everywhere. Paracetamol is the painkiller to use; steer clear of ibuprofen and the other anti-inflammatories unless your midwife or doctor has said they are fine for you. A maternity support belt, flat shoes, a warm bath and sleeping on your side with a pillow between your knees all take the strain off, and your midwife can refer you to a physiotherapist who deals with exactly this. Ring your midwife or maternity unit the same day if the pain comes with bleeding, any waters, a bad headache, blurred vision or swelling in your hands and face, a temperature, stinging when you pass urine, or the baby moving less than usual.")); __kws.extend(vec![("avoid".to_string(), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AVOID()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_BACK_PREGNANCY_KEEP(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("not pregnant"), py_str("not expecting"), py_str("no longer pregnant")]))), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), __data109()), ("guard".to_string(), py_func("_the_back_is_a_back", |args: &[Py]| _the_back_is_a_back(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug110);
    s__FEVER_ADVICE(py_str("Treat how they seem, not the number: cool clothes and plenty to drink do more than the thermometer, plus paracetamol or ibuprofen at the dose on the pack if it is making them miserable. Don't sponge with cold water or use both to chase the number. Get seen the same day for a baby under three months with 38C, a rash that will not fade under a glass, a stiff neck, breathing trouble, a fit, or a child floppy or not drinking; ring 111 if unsure."));
    s__TAKE_A_TEMPERATURE(py_str("Use a digital thermometer under the tongue or in the armpit: wait 20 minutes after a hot or cold drink, tuck the tip under the tongue and keep it there until it beeps. In the armpit, dry the skin first and hold the arm clamped to the body, which reads about half a degree lower than the mouth. An ear thermometer has to point across towards the other eye to read right, and write the number and the time down if you are watching a fever."));
    s__CHICKENPOX_TIMELINE(py_str("They are infectious from about two days before the rash appears until every spot has crusted over, usually five days after the rash first shows, which is when they can go back to school."));
    fn __data111() -> Py { py_tuple(vec![py_str("bring down"), py_str("lower"), py_str("reduce"), py_str("get rid"), py_str("what can i do"), py_str("what should i do"), py_str("how do i"), py_str("should i worry"), py_str("worried"), py_str("how long"), py_str("when should i"), py_str("see a doctor"), py_str("doctor"), py_str("gp"), py_str("hospital"), py_str("paracetamol"), py_str("ibuprofen"), py_str("calpol"), py_str("medicine"), py_str("worry"), py_str("help")]) }
    fn __data112() -> Py { py_tuple(vec![py_str("oven"), py_str("thermostat"), py_str("heating"), py_str("room"), py_str("house"), py_str("home"), py_str("fridge"), py_str("freezer"), py_str("cook"), py_str("bake"), py_str("set the"), py_str("washing"), py_str("hob"), py_str("air fryer"), py_str("boiler"), py_str("bedroom"), py_str("weather"), py_str("outside"), py_str("garden"), py_str("how hot"), py_str("the sea"), py_str("pool"), py_str("chickenpox"), py_str("hay fever"), py_str("hayfever"), py_str("hay-fever"), py_str("pollen"), py_str("allergy season"), py_str("glandular fever"), py_str("glandular"), py_str("scarlet fever"), py_str("scarlet"), py_str("rheumatic fever"), py_str("dengue"), py_str("typhoid"), py_str("malaria")]) }
    fn __data113() -> Py { py_tuple(vec![py_str("take"), py_str("takes"), py_str("taking"), py_str("read"), py_str("reading"), py_str("use"), py_str("using"), py_str("best way"), py_str("under the arm"), py_str("under my arm"), py_str("underarm"), py_str("armpit"), py_str("armpits"), py_str("ear"), py_str("mouth"), py_str("oral"), py_str("forehead"), py_str("accurate"), py_str("accurately")]) }
    fn __data114() -> Py { py_tuple(vec![py_str("meat"), py_str("chicken"), py_str("pork"), py_str("beef"), py_str("lamb"), py_str("turkey"), py_str("joint"), py_str("steak"), py_str("roast"), py_str("cooked"), py_str("cooking"), py_str("doneness"), py_str("oven"), py_str("fan oven"), py_str("air fryer"), py_str("bake"), py_str("baking"), py_str("bread"), py_str("cake"), py_str("sugar"), py_str("jam"), py_str("oil"), py_str("frying"), py_str("boiling water"), py_str("without a thermometer"), py_str("no thermometer"), py_str("hay fever"), py_str("hayfever"), py_str("room temperature"), py_str("room temp"), py_str("butter"), py_str("boiler"), py_str("flow temperature"), py_str("radiator"), py_str("fridge"), py_str("freezer"), py_str("washing machine"), py_str("tumble dryer"), py_str("bath"), py_str("bathing"), py_str("heating"), py_str("greenhouse"), py_str("wine"), py_str("beer"), py_str("water"), py_str("washing up"), py_str("frying pan"), py_str("cooker"), py_str("hob"), py_str("kettle"), py_str("saucepan"), py_str("milk"), py_str("yogurt"), py_str("dough"), py_str("yeast"), py_str("sourdough"), py_str("stock"), py_str("soup"), py_str("sauce"), py_str("sugar"), py_str("jam"), py_str("homebrew"), py_str("aquarium"), py_str("fish tank"), py_str("pond"), py_str("compost"), py_str("kiln"), py_str("sauna"), py_str("sterilis"), py_str("steriliz"), py_str("disinfect"), py_str("clean"), py_str("washing"), py_str("wash the"), py_str("fever"), py_str("feverish"), py_str("chickenpox"), py_str("rash")]) }
    fn __data115() -> Py { py_tuple(vec![py_str("chickenpox"), py_str("chicken pox"), py_str("how long"), py_str("contagious"), py_str("infectious"), py_str("school"), py_str("nursery"), py_str("childminder"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("toddler"), py_str("son"), py_str("daughter"), py_str("little one"), py_str("last"), py_str("calamine"), py_str("cream"), py_str("bath"), py_str("ibuprofen"), py_str("paracetamol"), py_str("back to school"), py_str("off school"), py_str("catch")]) }
    fn __data116() -> Py { py_tuple(vec![py_str("sun"), py_str("sunburn"), py_str("sunbed"), py_str("freckle"), py_str("freckles"), py_str("mole"), py_str("moles"), py_str("acne"), py_str("face wash"), py_str("makeup"), py_str("wall"), py_str("paint"), py_str("grease"), py_str("brass"), py_str("stain"), py_str("stains"), py_str("remove"), py_str("get out"), py_str("spot cream"), py_str("spot treatment"), py_str("best"), py_str("buy"), py_str("which"), py_str("product"), py_str("products")]) }
    let __aug117 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fever-rest")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fever"), py_str("temperature"), py_str("high temperature"), py_str("burning up"), py_str("shivery")]), __data111()])); __args.push(py_func("_fever_reply", |args: &[Py]| _fever_reply(args))); __kws.extend(vec![("avoid".to_string(), __data112())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("take-a-temperature")); __args.push(py_tuple(vec![py_tuple(vec![py_str("thermometer"), py_str("thermometers"), py_str("temperature"), py_str("temps")]), __data113()])); __args.push(py_func("_take_a_temperature", |args: &[Py]| _take_a_temperature(args))); __kws.extend(vec![("avoid".to_string(), __data114()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chickenpox")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chickenpox"), py_str("chicken pox"), py_str("spots"), py_str("blisters"), py_str("scabs")]), __data115()])); __args.push(py_func("_chickenpox_reply", |args: &[Py]| _chickenpox_reply(args))); __kws.extend(vec![("avoid".to_string(), __data116())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug117);
    s_NUMB_HANDS(py_str("Hands and fingers that go numb, often in bed or after holding a phone, or that cannot be felt at all, are usually a nerve being pressed, most often the median nerve at the wrist (carpal tunnel), which is why the thumb and first two fingers tingle and shaking the hand brings it back. Sleep with the wrist straight - a soft splint from a pharmacy keeps it that way - and take breaks from gripping and typing. See a GP if it starts happening in the day, the hand weakens or wastes, or it is numb all the time."));
    fn __data118() -> Py { py_tuple(vec![py_str("numb"), py_str("numbness"), py_str("pins"), py_str("needles"), py_str("tingling"), py_str("dead"), py_str("asleep"), py_str("wake"), py_str("wakes"), py_str("waking"), py_str("feel"), py_str("feeling"), py_str("cannot"), py_str("no feeling"), py_str("lost feeling")]) }
    s_NUMB_ASK(__data118());
    s_NUMB_AVOID(py_tuple(vec![py_str("leg"), py_str("legs"), py_str("feet"), py_str("foot"), py_str("toes"), py_str("back"), py_str("saddle"), py_str("pee"), py_str("urine")]));
    s_HAND_PARTS(py_tuple(vec![py_str("hand"), py_str("hands"), py_str("fingers"), py_str("finger"), py_str("thumb"), py_str("wrist"), py_str("wrists")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("numb-hands-night")); __args.push(py_tuple(vec![py_tuple(vec![py_str("numb"), py_str("numbness"), py_str("pins and needles"), py_str("tingling")]), op_add(&g_HAND_PARTS(), &py_tuple(vec![py_str("night"), py_str("nights"), py_str("in bed"), py_str("asleep"), py_str("wake me"), py_str("wakes me"), py_str("waking up"), py_str("morning"), py_str("driving"), py_str("phone"), py_str("cycling")]))])); __args.push(g_NUMB_HANDS()); __kws.extend(vec![("ask".to_string(), g_NUMB_ASK()), ("avoid".to_string(), g_NUMB_AVOID())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dead-hand")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dead"), py_str("gone to sleep"), py_str("asleep"), py_str("cant feel"), py_str("can't feel"), py_str("cannot feel"), py_str("no feeling"), py_str("lost feeling"), py_str("lost the feeling"), py_str("feel nothing")]), g_HAND_PARTS()])); __args.push(g_NUMB_HANDS()); __kws.extend(vec![("ask".to_string(), g_NUMB_ASK()), ("avoid".to_string(), g_NUMB_AVOID())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_FOOT_PARTS(py_tuple(vec![py_str("foot"), py_str("sole"), py_str("heel"), py_str("instep"), py_str("ankle")]));
    s_NUMB_FOOT(py_str("A foot that has gone numb is usually a nerve being pressed - a tight shoe, a leg crossed over it, kneeling or sitting on it - and the feeling comes back over a few minutes as the pressure comes off. Shake it out, walk on it and change position rather than rubbing it. If it keeps happening, or both feet are numb, it is worth a GP visit for the usual causes: diabetes, a B12 or thyroid problem, a trapped nerve from the back, and narrow shoes. Both feet numb with back pain, or no feeling between the legs with trouble weeing, is the same-day one: ring 111, or 999 if the back has given way. A sudden cold, white or blue foot, or a foot that drags, is 999."));
    fn __data119() -> Py { py_tuple(vec![py_str("shoe"), py_str("shoes"), py_str("boot"), py_str("boots"), py_str("sock"), py_str("socks"), py_str("laces"), py_str("tight"), py_str("sitting"), py_str("crossed"), py_str("kneeling"), py_str("sleep"), py_str("asleep"), py_str("cold"), py_str("white"), py_str("blue"), py_str("swollen"), py_str("broken"), py_str("sprained"), py_str("walking"), py_str("run"), py_str("running"), py_str("cycling"), py_str("bike"), py_str("driving"), py_str("work"), py_str("standing"), py_str("long")]) }
    s_NUMB_SHOE(__data119());
    fn __data120() -> Py { py_tuple(vec![py_str("backache"), py_str("back ache"), py_str("back pain"), py_str("my back hurts"), py_str("my back is sore"), py_str("sore back"), py_str("aching back"), py_str("bad back"), py_str("my back went"), py_str("my back has gone"), py_str("my back is killing"), py_str("back is killing"), py_str("my back is bad"), py_str("back spasm"), py_str("lower back"), py_str("spine"), py_str("spinal"), py_str("slipped disc"), py_str("sciatica"), py_str("between my legs"), py_str("bladder"), py_str("wee"), py_str("weeing"), py_str("peed"), py_str("pee"), py_str("peeing"), py_str("urine"), py_str("urinate"), py_str("catheter"), py_str("incontinent"), py_str("bowels"), py_str("no urine"), py_str("wet myself"), py_str("wetting myself")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("numb-foot")); __args.push(py_tuple(vec![g_FOOT_PARTS(), py_tuple(vec![py_str("numb"), py_str("numbness"), py_str("pins and needles"), py_str("tingling"), py_str("no feeling"), py_str("cant feel"), py_str("can't feel"), py_str("gone numb"), py_str("went numb"), py_str("dead"), py_str("asleep")])])); __args.push(g_NUMB_FOOT()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("numb"), py_str("numbness"), py_str("pins"), py_str("needles"), py_str("tingling"), py_str("no feeling"), py_str("gone numb"), py_str("went numb"), py_str("dead"), py_str("asleep"), py_str("feel")])), ("avoid".to_string(), __data120()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_TIGHT_CHEST(py_str("A tight chest has three common causes and they feel much alike. Asthma: a wheeze, a cough at night or with cold air or exercise, which eases with a blue reliever inhaler. Anxiety: a fast heart, tingling hands, and it comes in waves of ten or twenty minutes, often when you are wound up. A chest infection: a fever, a wet cough and feeling rough, which needs seeing if you are breathless or it is going on. Sudden tightness with pain spreading to the jaw, neck or arm, sweating, or breathlessness while sitting still is 999. If it keeps coming back, see a GP and say when it happens - they may give you a peak flow meter or an inhaler to try."));
    fn __data121() -> Py { py_tuple(vec![py_str("heartburn"), py_str("acid"), py_str("reflux"), py_str("indigestion"), py_str("cough"), py_str("cold"), py_str("flu"), py_str("covid"), py_str("drawers"), py_str("fridge"), py_str("freezer"), py_str("box"), py_str("packing"), py_str("suitcase"), py_str("lift"), py_str("carry"), py_str("exercise"), py_str("running"), py_str("run"), py_str("gym"), py_str("when i run"), py_str("when i exercise"), py_str("dog"), py_str("cat"), py_str("baby"), py_str("child"), py_str("toddler"), py_str("muscle"), py_str("muscles"), py_str("gym")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tight-chest")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chest"), py_str("in my chest"), py_str("my chest")]), py_tuple(vec![py_str("tight"), py_str("tightness"), py_str("tightening"), py_str("tightens"), py_str("constricted"), py_str("band"), py_str("pressure"), py_str("heavy"), py_str("weight")])])); __args.push(g_TIGHT_CHEST()); __kws.extend(vec![("avoid".to_string(), __data121()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data122() -> Py { py_tuple(vec![py_str("dry"), py_str("dryness"), py_str("cracked"), py_str("cracking"), py_str("flaky"), py_str("chapped"), py_str("skin"), py_str("fever"), py_str("feverish"), py_str("temperature"), py_str("shivery"), py_str("shivering"), py_str("chills"), py_str("flu"), py_str("influenza"), py_str("virus")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("aching-joints")); __args.push(py_tuple(vec![py_tuple(vec![py_str("joints"), py_str("joint"), py_str("knuckles"), py_str("knuckle")]), g_HURT()])); __args.push(py_str("Most aching joints are ordinary wear and tear - osteoarthritis - and keeping them moving is what helps most: walk, swim or cycle, strengthen the muscles around the joint, and lose a bit of weight if you can, which takes the load off knees and hips. Paracetamol or an ibuprofen gel eases a flare. A joint that is hot, red and swollen, or sore with a fever, needs seeing the same day; see a GP if it aches most days or your hands are stiff for more than half an hour in the morning.")); __kws.extend(vec![("guard".to_string(), py_func("_the_part_is_a_persons", |args: &[Py]| _the_part_is_a_persons(args))), ("avoid".to_string(), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AVOID()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STANDING(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &__data122())), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data123() -> Py { py_tuple(vec![py_str("lamb"), py_str("pork"), py_str("beef"), py_str("chicken"), py_str("meat"), py_str("roast"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("oven"), py_str("freezer"), py_str("defrost"), py_str("joint of"), py_str("fell"), py_str("fall"), py_str("after a fall"), py_str("injury"), py_str("injured"), py_str("dislocated"), py_str("broken"), py_str("replacement"), py_str("operation"), py_str("surgery"), py_str("after the operation")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("frozen-shoulder")); __args.push(py_tuple(vec![py_tuple(vec![py_str("shoulder"), py_str("shoulders")]), py_tuple(vec![py_str("frozen"), py_str("frozen shoulder"), py_str("locked"), py_str("cant lift"), py_str("can't lift"), py_str("cannot lift"), py_str("cant raise"), py_str("can't raise"), py_str("wont move"), py_str("won't move"), py_str("cant move it"), py_str("can't move it")])])); __args.push(py_str("A shoulder that stiffens up over weeks until you cannot lift the arm, especially turning it outwards, is usually a frozen shoulder: the capsule around the joint tightens, often for no clear reason. It eases in stages, and most people take 1 to 2 years to get most of the movement back. Keep using the arm as far as it goes, warm it before gentle stretches, and take paracetamol or ibuprofen if you can; a GP can refer you for physiotherapy and give a steroid injection early, which helps most in the painful first stage.")); __kws.extend(vec![("avoid".to_string(), __data123())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__ARTHRITIS_TAIL(py_str("it aches after years of use, is stiff first thing and eases as you move, and the treatment is using the joint rather than sparing it. Walking, swimming or cycling and a little strength work a few times a week keep the muscles that support the joint strong, and paracetamol with an ibuprofen gel handles a flare better than most tablets sold for arthritis. Glucosamine and cod liver oil have not held up in trials, no diet cures it, and cold, damp weather can make it ache more without damaging the joint; losing weight if you are carrying extra takes real load off knees and hips. Stiffness that lasts more than an hour in the morning, or a joint that is hot, red and swollen, is the inflammatory kind and wants a GP soon - ask about physiotherapy, and about a steroid injection for one joint that keeps flaring."));
    s__ARTHRITIS_CURE(py_tuple(vec![py_str("cure"), py_str("cured"), py_str("cures"), py_str("curable"), py_str("reverse"), py_str("reversed"), py_str("reversible"), py_str("rid"), py_str("go away"), py_str("gone away"), py_str("permanent"), py_str("permanently")]));
    fn __data124() -> Py { py_tuple(vec![py_str("pain"), py_str("pains"), py_str("painful"), py_str("hurts"), py_str("hurt"), py_str("ache"), py_str("aching"), py_str("aches"), py_str("stiff"), py_str("stiffness"), py_str("hands"), py_str("hand"), py_str("fingers"), py_str("knees"), py_str("knee"), py_str("hips"), py_str("hip"), py_str("thumb"), py_str("joint"), py_str("joints"), py_str("weather"), py_str("cold"), py_str("damp"), py_str("diet"), py_str("food"), py_str("foods"), py_str("exercise"), py_str("exercising"), py_str("walking"), py_str("swimming"), py_str("tablets"), py_str("glucosamine"), py_str("injection"), py_str("manage"), py_str("know"), py_str("cause")]) }
    fn __data125() -> Py { py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pet"), py_str("pets"), py_str("horse"), py_str("rabbit"), py_str("vet"), py_str("replacement"), py_str("replaced"), py_str("hip replacement"), py_str("knee replacement"), py_str("operation"), py_str("surgery")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("arthritis")); __args.push(py_tuple(vec![py_tuple(vec![py_str("arthritis"), py_str("osteoarthritis"), py_str("arthritic"), py_str("arthritus"), py_str("arthrosis"), py_str("arthrits")]), op_add(&pyrt::xrefs::xget("knowledge.lookup", "ASK_ABOUT"), &__data124())])); __args.push(py_func("_arthritis_reply", |args: &[Py]| _arthritis_reply(args))); __kws.extend(vec![("avoid".to_string(), __data125()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data126() -> Py { py_tuple(vec![py_str("stiff"), py_str("stiffness"), py_str("stiffens"), py_str("stiffened"), py_str("stiffening"), py_str("gone stiff"), py_str("cant bend"), py_str("can't bend"), py_str("wont bend"), py_str("won't bend"), py_str("cant make a fist"), py_str("can't make a fist"), py_str("first thing"), py_str("in the morning"), py_str("feel tight")]) }
    fn __data127() -> Py { py_tuple(vec![py_str("typing"), py_str("type"), py_str("keyboard"), py_str("mouse"), py_str("phone"), py_str("grip"), py_str("gripping"), py_str("wrist"), py_str("wrists"), py_str("tendon"), py_str("sprained"), py_str("sprain"), py_str("fell"), py_str("fall"), py_str("injury"), py_str("injured"), py_str("hot"), py_str("red"), py_str("swollen")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("stiff-hands-morning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fingers"), py_str("finger"), py_str("hands"), py_str("hand"), py_str("knuckles"), py_str("knuckle"), py_str("thumbs"), py_str("thumb")]), __data126()])); __args.push(py_str("Stiff fingers in the morning are usually the small joints of the hands starting to wear - osteoarthritis - and that kind loosens within half an hour of moving them: run warm water over the hands first thing, do gentle finger and thumb circles, and keep using them, because rest is what makes a stiff hand stiffer. Paracetamol helps a sore spell and a pharmacist can suggest an ibuprofen gel. See a GP if the stiffness lasts more than an hour, the knuckles are swollen, or both hands are the same, because that pattern is the inflammatory kind and wants treating early.")); __kws.extend(vec![("avoid".to_string(), __data127()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data128() -> Py { py_tuple(vec![py_str("arthritis"), py_str("joints"), py_str("joint"), py_str("hands"), py_str("hand"), py_str("fingers"), py_str("flares"), py_str("flare"), py_str("pain"), py_str("hurts"), py_str("hurt"), py_str("medication"), py_str("methotrexate"), py_str("drugs"), py_str("treatment"), py_str("exercise"), py_str("tired"), py_str("stiff")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rheumatoid-arthritis")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rheumatoid"), py_str("rheumatism"), py_str("rheumatoid arthritis")]), __data128()])); __args.push(py_str("Rheumatoid arthritis is the immune system attacking the lining of the joints, and it is treated early and hard because most of the damage happens in the first year or two: a rheumatology team starts disease-modifying drugs such as methotrexate, with a short course of steroids to settle a flare, and those are what keep the joints working. Keep moving between flares, take the tablets even when you feel well, and tell the team the same day if a joint is hot and swollen or you feel generally unwell, since that can be an infection rather than a flare and the two want opposite treatment.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    fn __data129() -> Py { py_tuple(vec![py_str("ankle"), py_str("ankles"), py_str("wrist"), py_str("wrists"), py_str("arm"), py_str("arms"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("hand"), py_str("hands"), py_str("leg"), py_str("legs"), py_str("knee"), py_str("knees"), py_str("elbow"), py_str("shoulder"), py_str("hip"), py_str("shin"), py_str("heel"), py_str("collarbone"), py_str("collarbones")]) }
    s_BROKEN_PARTS(__data129());
    fn __data130() -> Py { py_tuple(vec![py_str("broken"), py_str("broke"), py_str("fractured"), py_str("fracture"), py_str("cracked"), py_str("crack"), py_str("is it broken"), py_str("or broken"), py_str("or sprained"), py_str("or just sprained"), py_str("do i need an x-ray"), py_str("do i need an xray"), py_str("need an x-ray"), py_str("need an xray"), py_str("how do i know"), py_str("how can i tell"), py_str("how to tell"), py_str("is it a break"), py_str("a break or a sprain"), py_str("broken or sprained"), py_str("after a fall"), py_str("fell on it"), py_str("went over on it"), py_str("landed on it"), py_str("since i fell"), py_str("twisted it")]) }
    s_BROKEN_WORDS(__data130());
    fn __data131() -> Py { py_tuple(vec![py_str("ankle"), py_str("ankles"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("leg"), py_str("legs"), py_str("knee"), py_str("knees"), py_str("shin"), py_str("heel"), py_str("hip"), py_str("hips")]) }
    s_ON_YOUR_FEET(__data131());
    s_IN_YOUR_HANDS(py_tuple(vec![py_str("wrist"), py_str("wrists"), py_str("hand"), py_str("hands"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("elbow"), py_str("elbows")]));
    s_AT_YOUR_SHOULDER(py_tuple(vec![py_str("shoulder"), py_str("shoulders"), py_str("arm"), py_str("arms"), py_str("collarbone")]));
    s_FOUR_STEPS_REPLY(py_str("How much it hurts does not tell you - a bad sprain can hurt more than a small break. Stand on it and take four steps: if you cannot, or it is crooked, numb or swelling fast, go to a minor injuries unit or A&E today for an X-ray. If you can walk on it, treat it as a sprain - ice for 20 minutes, a support wrap, keep it up - and have it checked if it is no better in two days."));
    s_IN_YOUR_HANDS_REPLY(py_str("How much it hurts does not tell you - a bad sprain can hurt more than a small break. Try to use it: make a fist, turn a door handle, lift something light. If you cannot, or the bone looks crooked, the fingers are numb or it is swelling fast, go to a minor injuries unit or A&E today for an X-ray. If you can use it with care, treat it as a sprain - ice for 20 minutes, a support wrap, keep it raised - and have it checked if it is no better in two days."));
    s_AT_YOUR_SHOULDER_REPLY(py_str("How much it hurts does not tell you - a bad sprain can hurt more than a small break. Try to raise the arm sideways to shoulder height and reach across your chest: if you cannot move it, or the joint looks out of shape, the arm or hand is numb or it is swelling fast, go to a minor injuries unit or A&E today for an X-ray. If you can move it with care, treat it as a sprain - ice for 20 minutes, a support wrap, keep it up - and have it checked if it is no better in two days."));
    fn __data132() -> Py { py_tuple(vec![py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my baby"), py_str("my toddler"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my mom"), py_str("my mother"), py_str("my dad"), py_str("my father"), py_str("my friend"), py_str("my dog"), py_str("my cat"), py_str("his"), py_str("her"), py_str("when i was"), py_str("years ago"), py_str("last year"), py_str("healed"), py_str("healing"), py_str("cast"), py_str("crutches"), py_str("boot"), py_str("physio"), py_str("physiotherapy"), py_str("recovery"), py_str("recover"), py_str("how long does it take"), py_str("sticking out"), py_str("through the skin"), py_str("when can i"), py_str("how soon"), py_str("again after"), py_str("play again"), py_str("back to work"), py_str("back to sport"), py_str("back to football"), py_str("back to the gym"), py_str("back to running"), py_str("i broke"), py_str("ive broken"), py_str("i've broken"), py_str("i fractured"), py_str("i cracked"), py_str("think i broke"), py_str("thinks he broke"), py_str("thinks she broke"), py_str("think ive broken"), py_str("think i've broken"), py_str("bone is showing"), py_str("999"), py_str("ambulance"), py_str("unconscious"), py_str("head injury"), py_str("egg"), py_str("eggs"), py_str("hit my head"), py_str("hit his head"), py_str("head injury"), py_str("knocked out"), py_str("unconscious"), py_str("blacked out"), py_str("cannot get up"), py_str("cant get up"), py_str("could not get up"), py_str("neck"), py_str("glasses"), py_str("spectacles"), py_str("specs"), py_str("sunglasses"), py_str("frame"), py_str("frames"), py_str("lenses"), py_str("watch"), py_str("watch strap"), py_str("watchstrap"), py_str("bracelet")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("broken-or-sprained")); __args.push(py_tuple(vec![g_BROKEN_PARTS(), g_BROKEN_WORDS()])); __args.push(py_func("_broken_or_sprained", |args: &[Py]| _broken_or_sprained(args))); __kws.extend(vec![("avoid".to_string(), __data132())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data133() -> Py { py_tuple(vec![py_str("fever"), py_str("hot flush"), py_str("hot flushes"), py_str("menopause"), py_str("night sweat"), py_str("night sweats"), py_str("panic"), py_str("anxious"), py_str("anxiety"), py_str("withdrawal"), py_str("opiate"), py_str("gym"), py_str("workout"), py_str("running"), py_str("hyperhidrosis"), py_str("my son"), py_str("my daughter"), py_str("my baby"), py_str("my child"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my dad")]) }
    fn __data134() -> Py { py_tuple(vec![py_str("brush"), py_str("brushing"), py_str("toothbrush"), py_str("toothpaste"), py_str("my toddler"), py_str("my child"), py_str("my kid"), py_str("my son"), py_str("my daughter"), py_str("baby"), py_str("teething baby"), py_str("teething ring"), py_str("cutting teeth"), py_str("dentist appointment"), py_str("register"), py_str("toothache"), py_str("hurts"), py_str("sensitive"), py_str("filling"), py_str("brace"), py_str("braces"), py_str("whitening strips are safe")]) }
    fn __data135() -> Py { py_tuple(vec![py_str("grow"), py_str("growing"), py_str("longer"), py_str("long"), py_str("stronger"), py_str("strong"), py_str("break"), py_str("breaking"), py_str("brittle"), py_str("peel"), py_str("peeling"), py_str("split"), py_str("splitting"), py_str("flaky"), py_str("weak"), py_str("strengthen"), py_str("strengthening"), py_str("healthy"), py_str("healthier"), py_str("harder"), py_str("faster"), py_str("thicker")]) }
    fn __data136() -> Py { py_tuple(vec![py_str("biting"), py_str("bite"), py_str("cuticle"), py_str("gel"), py_str("acrylic"), py_str("acrylics"), py_str("polish"), py_str("paint"), py_str("manicure"), py_str("pedicure"), py_str("hammer"), py_str("hangnail"), py_str("fungal"), py_str("infection"), py_str("black"), py_str("bruised"), py_str("bruise"), py_str("purple"), py_str("dark"), py_str("slammed"), py_str("slam"), py_str("dropped"), py_str("knocked"), py_str("knock"), py_str("stubbed"), py_str("smashed"), py_str("football"), py_str("door"), py_str("trapped"), py_str("my dog"), py_str("my cat"), py_str("my baby"), py_str("my toddler"), py_str("my child")]) }
    let __aug137 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sweating")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sweat"), py_str("sweats"), py_str("sweaty"), py_str("sweating"), py_str("perspire"), py_str("perspiring"), py_str("perspiration"), py_str("clammy"), py_str("b.o."), py_str("body odour"), py_str("body odor")]), py_tuple(vec![py_str("stop"), py_str("less"), py_str("too much"), py_str("so much"), py_str("a lot"), py_str("excessive"), py_str("reduce"), py_str("under my arms"), py_str("armpits"), py_str("always")])])); __args.push(py_str("Use an antiperspirant rather than a deodorant, put it on last thing at night on dry skin where it works, and choose one with aluminium in it. Loose cotton and a spare shirt help, spicy food and alcohol make it worse, and if you sweat buckets from one place or at night, that is worth a GP visit.")); __kws.extend(vec![("avoid".to_string(), __data133())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("whiten-teeth")); __args.push(py_tuple(vec![py_tuple(vec![py_str("whiten"), py_str("whiter"), py_str("whitening"), py_str("bleach"), py_str("stains"), py_str("stained"), py_str("yellow"), py_str("yellowing"), py_str("brighter")]), py_tuple(vec![py_str("teeth"), py_str("tooth"), py_str("smile"), py_str("my teeth")])])); __args.push(py_str("A dentist or a hygienist scale and polish first, because most yellowing is staining on the outside, and then whitening works: either a professional kit made for your teeth or a legal-strength high-street one, and expect it to take a couple of weeks. Charcoal and bicarbonate toothpastes scrub the enamel and do not whiten it.")); __kws.extend(vec![("avoid".to_string(), __data134())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nails-grow")); __args.push(py_tuple(vec![py_tuple(vec![py_str("nails"), py_str("nail"), py_str("fingernails"), py_str("fingernail"), py_str("toenails"), py_str("toenail")]), __data135()])); __args.push(py_str("Nails grow about 3mm a month, so a full nail takes six months and nothing speeds it up much: keep them filed rather than bitten, use a hand cream or cuticle oil daily, wear gloves for washing up, and take in enough protein and iron. Ridges and white spots are normal; a nail that changes colour or lifts is worth showing a pharmacist.")); __kws.extend(vec![("avoid".to_string(), __data136()), ("guard".to_string(), py_func("_the_nail_is_a_persons", |args: &[Py]| _the_nail_is_a_persons(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug137);
    let __aug138 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("physio-or-stretch")); __args.push(py_tuple(vec![py_tuple(vec![py_str("physio"), py_str("physiotherapy"), py_str("physiotherapist"), py_str("chiropractor"), py_str("osteopath")]), py_tuple(vec![py_str("see"), py_str("should i"), py_str("or just"), py_str("worth"), py_str("need"), py_str("go"), py_str("stretch"), py_str("gp"), py_str("refer")])])); __args.push(py_str("See a physio if the pain has lasted more than a fortnight, keeps coming back, or comes with numbness, weakness or pins and needles - and you can refer yourself on the NHS without seeing a GP first. If it is a recent strain that is already easing, gentle daily stretching plus some strength work (sit-to-stands, bridges, a short walk) is what a physio would give you anyway.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("knee"), py_str("shoulder"), py_str("ankle"), py_str("calf"), py_str("hamstring"), py_str("achilles"), py_str("cost"), py_str("how much"), py_str("price"), py_str("private")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug138);
    fn __data139() -> Py { py_tuple(vec![py_str("dentist"), py_str("dentists"), py_str("dental"), py_str("gp"), py_str("doctor"), py_str("doctors"), py_str("doctor's"), py_str("hospital"), py_str("hospitals"), py_str("nhs"), py_str("surgery"), py_str("clinic"), py_str("nurse"), py_str("midwife"), py_str("consultant"), py_str("surgeon"), py_str("care i got"), py_str("care i received"), py_str("care i had"), py_str("the care i")]) }
    fn __data140() -> Py { py_tuple(vec![py_str("complain"), py_str("complaining"), py_str("complaint"), py_str("complaints"), py_str("made a complaint"), py_str("make a complaint"), py_str("raise a complaint"), py_str("raised a complaint"), py_str("mistake"), py_str("mistakes"), py_str("misdiagnosed"), py_str("misdiagnosis"), py_str("negligence"), py_str("negligent"), py_str("botched"), py_str("wrong tooth"), py_str("wrong tooth out"), py_str("wrong medication"), py_str("wrong medicine"), py_str("hurt me"), py_str("caused harm"), py_str("bad treatment"), py_str("poor treatment"), py_str("did a bad job")]) }
    fn __data141() -> Py { py_tuple(vec![py_str("bill"), py_str("billing"), py_str("invoice"), py_str("parking"), py_str("car park"), py_str("charge"), py_str("charges"), py_str("cost"), py_str("price"), py_str("how much"), py_str("register"), py_str("registering"), py_str("sign up"), py_str("appointment"), py_str("appointments"), py_str("book"), py_str("booking"), py_str("waiting list"), py_str("waiting time"), py_str("food"), py_str("meal"), py_str("cleanliness"), py_str("cleaning")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health-complaint")); __args.push(py_tuple(vec![__data139(), __data140()])); __args.push(py_str("Ask the practice or hospital for its complaints procedure and put it in writing: what happened, when, who you saw and what you want. Ask for a written reply and keep copies. Still unhappy? Ask for a deadlock letter and take it free to the NHS ombudsman - PALS at a hospital will help you word it, and private dental work has the free Dental Complaints Service. If someone was harmed, get the records and advice from AvMA before signing anything.")); __kws.extend(vec![("avoid".to_string(), __data141()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data142() -> Py { py_tuple(vec![py_str("change"), py_str("changed"), py_str("changes"), py_str("changing"), py_str("different"), py_str("bigger"), py_str("growing"), py_str("grown"), py_str("grew"), py_str("larger"), py_str("size"), py_str("shape"), py_str("edge"), py_str("edges"), py_str("border"), py_str("colour"), py_str("color"), py_str("colours"), py_str("colors"), py_str("darker"), py_str("dark"), py_str("black"), py_str("bleeding"), py_str("bled"), py_str("bleeds"), py_str("blood"), py_str("crusty"), py_str("crusting"), py_str("scab"), py_str("scabby"), py_str("itchy"), py_str("itching"), py_str("itches"), py_str("sore"), py_str("tender"), py_str("raised"), py_str("lumpy"), py_str("new"), py_str("suddenly"), py_str("worried"), py_str("worry"), py_str("worrying"), py_str("checked"), py_str("check"), py_str("checking"), py_str("dangerous"), py_str("cancer"), py_str("melanoma"), py_str("malignant"), py_str("normal"), py_str("looks"), py_str("look"), py_str("found"), py_str("keep an eye"), py_str("keep my eye"), py_str("monitor"), py_str("watch"), py_str("hair"), py_str("hairy"), py_str("catching on"), py_str("catches on"), py_str("catch on"), py_str("caught on"), py_str("rubs"), py_str("rubbing"), py_str("rubbed"), py_str("friction"), py_str("irritated"), py_str("appeared"), py_str("appears"), py_str("appearing")]) }
    s_MOLE_CHANGE(__data142());
    fn __data143() -> Py { py_tuple(vec![py_str("change"), py_str("changed"), py_str("changes"), py_str("changing"), py_str("different"), py_str("bigger"), py_str("growing"), py_str("grown"), py_str("grew"), py_str("larger"), py_str("size"), py_str("shape"), py_str("edge"), py_str("edges"), py_str("border"), py_str("colour"), py_str("color"), py_str("colours"), py_str("colors"), py_str("darker"), py_str("dark"), py_str("black"), py_str("bleeding"), py_str("bled"), py_str("bleeds"), py_str("blood"), py_str("crusty"), py_str("crusting"), py_str("scab"), py_str("scabby"), py_str("itchy"), py_str("itching"), py_str("itches"), py_str("sore"), py_str("tender"), py_str("raised"), py_str("lumpy"), py_str("dangerous"), py_str("cancer"), py_str("melanoma"), py_str("malignant"), py_str("worried"), py_str("worry"), py_str("worrying"), py_str("anxious"), py_str("bad"), py_str("serious"), py_str("abnormal"), py_str("cancerous"), py_str("catching"), py_str("catches"), py_str("caught"), py_str("rubs"), py_str("rubbing"), py_str("rubbed"), py_str("friction"), py_str("irritated"), py_str("appeared"), py_str("appears"), py_str("appearing")]) }
    s_MOLE_HAPPENING(__data143());
    s__MOLE_CLEAR(call_builtin("frozenset", &[mcall(&py_str("\nbleeding bled bleeds blood crusty crusting scab scabby itchy itching itches changed changing\nchange changes different bigger larger darker black sore tender lumpy\ncatching catches caught rubbing rubbing rubbed friction appeared appears appearing\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__MOLE_CHANGED_TEXT(py_str("Get it checked by a GP within two weeks. A mole that changes shape, edge, colour or size, or bleeds, itches or crusts on its own, wants a look rather than a wait, though almost all of them turn out to be nothing. The signs worth saying out loud are the ABCDE ones: not symmetrical, a ragged or blurry border, more than one colour, wider than about 6mm, or changing in any way. Photograph it beside a coin for scale and ring the surgery rather than waiting to see whether it settles."));
    s__MOLE_ORDINARY_TEXT(py_str("Almost all moles are harmless, so what matters is knowing what a change looks like rather than checking it every day: the ABCDE test is not symmetrical, a ragged or blurry border, more than one colour, wider than about 6mm, or anything that is changing. Photograph it beside a coin for scale and compare it every few months; a mole that changes, bleeds, itches or crusts on its own wants a GP within two weeks, and a hair growing out of one usually means it is an ordinary mole that has been there a long time."));
    s__CHECK_HOW_TO(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:do|can|should|would)\\s+(?:i|you|we)\\s+(?:check|inspect)|how\\s+to\\s+check|what\\s+(?:do\\s+i\\s+)?look\\s+for"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data144() -> Py { py_tuple(vec![py_str("new"), py_str("another"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("baby"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("friend"), py_str("mother"), py_str("father"), py_str("mum"), py_str("dad"), py_str("sister"), py_str("brother"), py_str("gran"), py_str("nan"), py_str("other")]) }
    s__NEW_SUBJECT(__data144());
    fn __data145() -> Py { py_tuple(vec![py_str("get rid"), py_str("rid of"), py_str("remove"), py_str("removing"), py_str("removal"), py_str("removed"), py_str("laser"), py_str("freeze"), py_str("freezing"), py_str("cut off"), py_str("cut it off"), py_str("tie it off"), py_str("shave off"), py_str("cut a mole"), py_str("cut my mole"), py_str("cut this mole"), py_str("get it cut")]) }
    s_MOLE_REMOVE(__data145());
    s__MOLE_REMOVE_TEXT(py_str("Most moles need no treatment at all, and one that bothers you - catching on clothes, or a GP wanting a closer look - is taken off on the NHS: a short excision under local anaesthetic, with the mole sent for a check. Don't cut, tie off or use a wart or skin-tag kit on it, and don't book a beauty clinic for something a GP should see first."));
    fn __data146() -> Py { py_tuple(vec![py_str("garden"), py_str("gardens"), py_str("lawn"), py_str("lawns"), py_str("grass"), py_str("golf"), py_str("green"), py_str("molehill"), py_str("molehills"), py_str("mole hill"), py_str("hill"), py_str("hills"), py_str("tunnel"), py_str("tunnels"), py_str("trap"), py_str("traps"), py_str("trapping"), py_str("pest"), py_str("pests"), py_str("animal"), py_str("animals"), py_str("wildlife"), py_str("drain"), py_str("drains"), py_str("field"), py_str("fields"), py_str("farm"), py_str("farmer"), py_str("vole"), py_str("voles"), py_str("rodent"), py_str("rodents"), py_str("sauce"), py_str("recipe"), py_str("recipes"), py_str("cook"), py_str("cooking"), py_str("dish"), py_str("dish"), py_str("chilli"), py_str("chocolate"), py_str("poblano"), py_str("chicken"), py_str("acne"), py_str("pimple"), py_str("pimples"), py_str("blackhead"), py_str("blackheads"), py_str("spot cream"), py_str("spot treatment"), py_str("face wash"), py_str("cleanse"), py_str("cleanser"), py_str("routine"), py_str("product"), py_str("products"), py_str("best"), py_str("sunscreen"), py_str("sun cream"), py_str("sunbed"), py_str("sun tan"), py_str("fake tan"), py_str("tanned"), py_str("tanning"), py_str("sunless")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("changing-mole")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mole"), py_str("moles"), py_str("birthmark"), py_str("birthmarks"), py_str("beauty spot"), py_str("beauty spots"), py_str("skin spot")]), op_add(&g_MOLE_CHANGE(), &g_MOLE_REMOVE())])); __args.push(py_func("_mole_reply", |args: &[Py]| _mole_reply(args))); __kws.extend(vec![("avoid".to_string(), __data146()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data147() -> Py { py_tuple(vec![py_str("paracetamol"), py_str("acetaminophen"), py_str("panadol"), py_str("calpol"), py_str("ibuprofen"), py_str("nurofen"), py_str("brufen"), py_str("co codamol"), py_str("co-codamol"), py_str("painkiller"), py_str("painkillers"), py_str("anti inflammatory"), py_str("nsaid"), py_str("nsaids")]) }
    s_PAINKILLER_NAMES(__data147());
    s__NOT_THE_TABLET(py_tuple(vec![py_str("heartburn"), py_str("indigestion"), py_str("reflux"), py_str("antacid"), py_str("gaviscon"), py_str("rennie"), py_str("omeprazole"), py_str("lansoprazole"), py_str("acid reflux")]));
    s__KIDNEY_WORDS(py_tuple(vec![py_str("kidney"), py_str("kidneys"), py_str("renal"), py_str("water tablet"), py_str("water tablets"), py_str("diuretic"), py_str("blood pressure tablet"), py_str("blood pressure tablets"), py_str("blood pressure medication"), py_str("blood pressure pills"), py_str("only one kidney")]));
    s__NOT_AN_NSAID(py_tuple(vec![py_str("paracetamol"), py_str("acetaminophen"), py_str("panadol"), py_str("calpol"), py_str("co codamol"), py_str("co-codamol")]));
    fn __data148() -> Py { py_tuple(vec![py_str("stomach"), py_str("tummy"), py_str("gut"), py_str("upset"), py_str("upsets"), py_str("upsetting"), py_str("empty stomach"), py_str("with food"), py_str("before food"), py_str("after food"), py_str("eat before"), py_str("eat something"), py_str("on food"), py_str("bad for my stomach"), py_str("bad for your stomach"), py_str("sore stomach"), py_str("irritate"), py_str("irritates"), py_str("irritating"), py_str("lining"), py_str("ulcer"), py_str("ulcers"), py_str("bleeding in my stomach"), py_str("stomach bleed"), py_str("does it upset"), py_str("kidney"), py_str("kidneys"), py_str("renal"), py_str("bad for my kidneys"), py_str("water tablet"), py_str("water tablets"), py_str("diuretic"), py_str("blood pressure tablet"), py_str("blood pressure tablets"), py_str("blood pressure medication"), py_str("blood pressure pills")]) }
    fn __data149() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("breastfeed"), py_str("child"), py_str("kid"), py_str("baby"), py_str("toddler"), py_str("dog"), py_str("cat"), py_str("overdose"), py_str("took too many"), py_str("whole packet"), py_str("allergic"), py_str("reaction"), py_str("rash"), py_str("asthma")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkiller-stomach")); __args.push(py_tuple(vec![g_PAINKILLER_NAMES(), __data148()])); __args.push(py_func("_the_organ_the_painkiller_upsets", |args: &[Py]| _the_organ_the_painkiller_upsets(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g__NOT_THE_TABLET(), &__data149()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data150() -> Py { py_tuple(vec![py_str("alcohol"), py_str("drink"), py_str("drinking"), py_str("drunk"), py_str("beer"), py_str("wine"), py_str("vodka"), py_str("gin"), py_str("pint"), py_str("pints"), py_str("booze"), py_str("a few drinks"), py_str("nights out"), py_str("night out")]) }
    fn __data151() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("child"), py_str("kid"), py_str("baby"), py_str("toddler"), py_str("dog"), py_str("cat"), py_str("overdose"), py_str("took too many"), py_str("whole packet"), py_str("alcoholic"), py_str("addiction"), py_str("stop drinking"), py_str("give up alcohol"), py_str("cut down")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkiller-and-alcohol")); __args.push(py_tuple(vec![g_PAINKILLER_NAMES(), __data150()])); __args.push(py_str("Paracetamol and drink is a liver question and ibuprofen and drink is a stomach one, and a normal dose of either with an occasional drink is fine. Paracetamol's warning is about drinking heavily or daily for years, because the liver handles both, and the ceiling stands whatever you have drunk: no more than 4g (eight 500mg tablets) in 24 hours, with no other paracetamol product on top. Ibuprofen mainly raises the risk of stomach irritation and bleeding, so eat something with it and keep to the lowest dose that works. If you drink heavily every day, ask a pharmacist before taking paracetamol at all.")); __kws.extend(vec![("avoid".to_string(), op_add(&g__NOT_THE_TABLET(), &__data151()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data152() -> Py { py_tuple(vec![py_str("child"), py_str("kid"), py_str("baby"), py_str("toddler"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("dog"), py_str("cat"), py_str("how long can i take"), py_str("how long should i take"), py_str("how many days"), py_str("how long does ibuprofen last"), py_str("how long does it last")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkiller-how-fast")); __args.push(py_tuple(vec![g_PAINKILLER_NAMES(), py_tuple(vec![py_str("how long"), py_str("how quickly"), py_str("how fast"), py_str("how soon"), py_str("when does it work"), py_str("take to work"), py_str("kick in"), py_str("kicks in"), py_str("start working"), py_str("starts working"), py_str("how long to work"), py_str("how long until")])])); __args.push(py_str("Tablets taken by mouth start working in about 30 minutes and are at their best at one to two hours; a soluble or effervescent one is quicker, and ibuprofen lysine and the liquid capsules are faster than a plain tablet. Pain that has not shifted at all after two hours, at a proper dose, is a reason to ask a pharmacist rather than to take more sooner than the packet allows.")); __kws.extend(vec![("avoid".to_string(), op_add(&g__NOT_THE_TABLET(), &__data152()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data153() -> Py { py_tuple(vec![py_str("thin your blood"), py_str("thins the blood"), py_str("thin the blood"), py_str("thinning"), py_str("blood thinner"), py_str("blood thinners"), py_str("warfarin"), py_str("clot"), py_str("clotting"), py_str("bleed more"), py_str("bleeding"), py_str("bruise"), py_str("bruises"), py_str("bruising"), py_str("before an operation"), py_str("before surgery"), py_str("before my operation"), py_str("dental work"), py_str("tooth out")]) }
    fn __data154() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("child"), py_str("kid"), py_str("baby"), py_str("toddler"), py_str("dog"), py_str("cat"), py_str("overdose"), py_str("took too many"), py_str("cut myself"), py_str("cut my"), py_str("wound"), py_str("nosebleed"), py_str("heavy period")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkiller-and-the-blood")); __args.push(py_tuple(vec![g_PAINKILLER_NAMES(), __data153()])); __args.push(py_str("Ibuprofen and the other anti-inflammatories make the blood a little slower to clot, which is why they are stopped before an operation or a tooth out - usually a few days before, on the surgeon's or dentist's word - and why they are not mixed with warfarin or a blood thinner without advice. Paracetamol has no effect on clotting at all, so if you bruise easily or bleed for longer than you used to, that is a question for a pharmacist or GP rather than a reason to keep taking it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("should i"), py_str("is it"), py_str("does it"), py_str("do i"), py_str("is"), py_str("does"), py_str("before"), py_str("with"), py_str("ok"), py_str("safe"), py_str("okay")])), ("avoid".to_string(), op_add(&g__NOT_THE_TABLET(), &__data154()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data155() -> Py { py_tuple(vec![py_str("sleepy"), py_str("drowsy"), py_str("drowsiness"), py_str("make me tired"), py_str("makes me tired"), py_str("knock me out"), py_str("knocks me out"), py_str("can i drive"), py_str("drive after"), py_str("drowsy driving"), py_str("sleep"), py_str("keep me awake"), py_str("awake")]) }
    fn __data156() -> Py { py_tuple(vec![py_str("child"), py_str("kid"), py_str("baby"), py_str("toddler"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("dog"), py_str("cat"), py_str("cant sleep"), py_str("can't sleep"), py_str("insomnia"), py_str("sleep aid"), py_str("sleeping tablet"), py_str("melatonin"), py_str("nightmare")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painkiller-and-sleep")); __args.push(py_tuple(vec![g_PAINKILLER_NAMES(), __data155()])); __args.push(py_str("Paracetamol and ibuprofen are not sedatives and neither is meant to make you sleepy - drowsiness is an uncommon side effect of ibuprofen, so if one does leave you dozy it is worth mentioning to a pharmacist. What does make people drowsy is the codeine in co-codamol, and the sleepy feeling is one of the reasons not to drive on it until you know how it treats you. Pain itself is often what stops people sleeping, so a dose at bedtime is fine.")); __kws.extend(vec![("avoid".to_string(), op_add(&g__NOT_THE_TABLET(), &__data156()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__WAX_FIRST_REPLY(py_str("Waxing wants hair about 5mm long, a patch test a day ahead and one firm pull against the growth with the skin held tight; take an antihistamine an hour before if you come up blotchy. Shaving is quicker: end of a warm shower, fresh blade, conditioner instead of foam, and with the hair rather than against it, which is what keeps the red bumps away. An ingrown hair wants a warm compress and an exfoliating glove every other day."));
    s__SHAVE_FIRST_REPLY(py_str("Shave at the end of a warm shower with a fresh blade and conditioner, rinsing the blade often, and on legs and bikini line go with the hair, not against it, which is what keeps the red bumps away. Waxing wants hair 5mm long, a patch test a day ahead and a firm pull against the growth, plus an antihistamine an hour before if you come up blotchy. An ingrown hair wants a warm compress and an exfoliating glove every other day."));
    fn __data157() -> Py { py_set(vec![py_str("wax"), py_str("waxes"), py_str("waxing"), py_str("waxed"), py_str("wax strip"), py_str("wax strips"), py_str("strip"), py_str("strips"), py_str("sugaring"), py_str("hot wax"), py_str("cold wax"), py_str("bikini wax"), py_str("leg wax"), py_str("underarm wax"), py_str("sugar")]) }
    s__WAX_WORDS(__data157());
    fn __data158() -> Py { py_set(vec![py_str("trim"), py_str("trims"), py_str("trimming"), py_str("trimmer"), py_str("trimmers"), py_str("shape"), py_str("shaping"), py_str("line"), py_str("line up"), py_str("lining"), py_str("neckline"), py_str("guard"), py_str("guards"), py_str("shorten"), py_str("tidy"), py_str("tidying"), py_str("beard"), py_str("beards"), py_str("moustache"), py_str("mustache"), py_str("goatee"), py_str("stubble")]) }
    s__BEARD_TRIM_WORDS(__data158());
    fn __data159() -> Py { py_set(vec![py_str("blade"), py_str("blades"), py_str("razor blade"), py_str("razor blades"), py_str("change"), py_str("changing"), py_str("rust"), py_str("rusty"), py_str("rusting"), py_str("going rusty"), py_str("sharpen"), py_str("sharp"), py_str("replace"), py_str("replacing"), py_str("how often"), py_str("blunt"), py_str("dull"), py_str("dragging"), py_str("drags"), py_str("last"), py_str("lasts"), py_str("how long"), py_str("wear out"), py_str("worn out"), py_str("how many shaves"), py_str("pulling"), py_str("pulls"), py_str("pull"), py_str("new razor"), py_str("cartridge"), py_str("cartridges")]) }
    s__BLADE_WORDS(__data159());
    fn __data160() -> Py { py_set(vec![py_str("burn"), py_str("burns"), py_str("burning"), py_str("razor burn"), py_str("sore"), py_str("red"), py_str("redness"), py_str("irritation"), py_str("irritated"), py_str("rash"), py_str("bumps"), py_str("spot"), py_str("spots"), py_str("sting"), py_str("stings"), py_str("stinging"), py_str("stop razor burn"), py_str("prevent razor burn"), py_str("stop razor rash"), py_str("prevent shaving rash"), py_str("avoid razor bumps"), py_str("stop the burn"), py_str("stop it burning"), py_str("prevent the burn"), py_str("avoid the burn"), py_str("stop the rash"), py_str("prevent the rash"), py_str("stop the redness"), py_str("stop the irritation"), py_str("prevent irritation"), py_str("avoid irritation"), py_str("stop the stinging"), py_str("stop the stinging"), py_str("avoid shaving rash")]) }
    s__SHAVE_BURN_WORDS(__data160());
    s__BEARD_TRIM_REPLY(py_str("Trim it dry, on a guard a couple of steps longer than the length you want, and go with the hair first before tidying the edges against it. Take the neckline just above the Adam's apple and the cheeks along the jaw, then finish with the guard off and the blade flat so the lines are sharp."));
    s__STRAIGHT_SHAVE_REPLY(py_str("Warm the hair first - a shower or a flannel held on for two minutes - then use a sharp blade and proper cream, and take short strokes with the grain without pressing. Most nicks come from a blunt blade or going over the same patch again, so rinse and stop there."));
    s__RAZOR_BLADE_REPLY(py_str("Rinse the blade in hot water and stand it up to dry rather than leaving it in the shower, and change it after five to seven shaves, or the moment it drags and pulls instead of gliding: a blunt blade scrapes the skin, which is what most razor burn really is."));
    s__RAZOR_BURN_REPLY(py_str("Rinse with cool water, pat it dry and skip anything with alcohol in it; then a fresh blade, shaving with the grain and short strokes are what stop it coming back. If it stings for days or comes up in spots, shave every other day until it settles, and take anything blistered or spreading to a pharmacist."));
    fn __data161() -> Py { py_tuple(vec![py_str("shave"), py_str("shaves"), py_str("shaving"), py_str("shaved"), py_str("razor"), py_str("razors"), py_str("beard"), py_str("beards"), py_str("moustache"), py_str("mustache"), py_str("goatee"), py_str("stubble"), py_str("beard trimmer"), py_str("trimmer"), py_str("razor blade"), py_str("razor blades"), py_str("wet shave"), py_str("close shave")]) }
    fn __data162() -> Py { py_tuple(vec![py_str("face"), py_str("neck"), py_str("chin"), py_str("jaw"), py_str("jawline"), py_str("cheek"), py_str("cheeks"), py_str("upper lip"), py_str("moustache"), py_str("mustache"), py_str("beard"), py_str("beards"), py_str("blade"), py_str("blades"), py_str("razor blade"), py_str("burn"), py_str("burns"), py_str("razor burn"), py_str("rash"), py_str("irritation"), py_str("cut"), py_str("cuts"), py_str("nick"), py_str("nicks"), py_str("trim"), py_str("trimming"), py_str("shave"), py_str("shaving"), py_str("stubble"), py_str("how often"), py_str("change"), py_str("replace"), py_str("closest"), py_str("smooth"), py_str("pulling"), py_str("pulls"), py_str("pull"), py_str("drag"), py_str("last"), py_str("lasts"), py_str("how long"), py_str("long does"), py_str("wear out"), py_str("worn out"), py_str("blunt"), py_str("dull"), py_str("rust"), py_str("rusty"), py_str("rusting"), py_str("cartridge"), py_str("cartridges"), py_str("drags"), py_str("dragging"), py_str("tugging"), py_str("look after"), py_str("care"), py_str("care for"), py_str("keep"), py_str("head"), py_str("scalp"), py_str("how do i"), py_str("how to"), py_str("best way"), py_str("what should i")]) }
    fn __data163() -> Py { py_tuple(vec![py_str("leg"), py_str("legs"), py_str("bikini"), py_str("underarm"), py_str("underarms"), py_str("armpit"), py_str("armpits"), py_str("eyebrow"), py_str("eyebrows"), py_str("pubic"), py_str("ingrown"), py_str("ingrowing"), py_str("ingrown hair"), py_str("razor bumps"), py_str("shaving rash"), py_str("shaving bumps"), py_str("wax"), py_str("waxes"), py_str("waxing"), py_str("sunburn"), py_str("sun burn"), py_str("scald"), py_str("scalded"), py_str("hot water"), py_str("iron"), py_str("oven"), py_str("hair removal"), py_str("legs and bikini"), py_str("parmesan"), py_str("cheese"), py_str("chocolate"), py_str("fennel"), py_str("truffle"), py_str("horseradish"), py_str("clam"), py_str("clams"), py_str("razor clam"), py_str("razor clams"), py_str("razor shell"), py_str("razor shells"), py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("washing"), py_str("itch"), py_str("itchy"), py_str("itching"), py_str("dandruff"), py_str("grow"), py_str("growing"), py_str("faster"), py_str("thicker"), py_str("patchy"), py_str("beard oil"), py_str("beard balm"), py_str("hedge"), py_str("hedges"), py_str("hedge trimmer"), py_str("hedge cutters"), py_str("garden"), py_str("gardening"), py_str("lawn"), py_str("lawns"), py_str("lawnmower"), py_str("mower"), py_str("mowers"), py_str("strimmer"), py_str("strimmers"), py_str("leaf blower"), py_str("chainsaw"), py_str("secateurs"), py_str("shears"), py_str("shed"), py_str("garage"), py_str("topiary"), py_str("shrub"), py_str("shrubs"), py_str("bush"), py_str("bushes")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("face-shaving")); __args.push(py_tuple(vec![__data161(), __data162()])); __args.push(py_func("_face_shaving", |args: &[Py]| _face_shaving(args))); __kws.extend(vec![("avoid".to_string(), __data163())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data164() -> Py { py_tuple(vec![py_str("wax"), py_str("waxes"), py_str("waxing"), py_str("waxed"), py_str("shave"), py_str("shaves"), py_str("shaving"), py_str("shaved"), py_str("hair removal"), py_str("hair remover"), py_str("epilator"), py_str("epilate"), py_str("epilating"), py_str("razor"), py_str("razors"), py_str("depilatory"), py_str("hair removal cream"), py_str("sugaring"), py_str("sugar my legs"), py_str("sugar legs"), py_str("sugar my bikini"), py_str("sugar my underarms"), py_str("sugar my armpits"), py_str("sugar wax"), py_str("sugar paste"), py_str("sugaring wax"), py_str("ingrown"), py_str("ingrowing"), py_str("ingrown hair"), py_str("ingrown hairs"), py_str("ingrowing hair"), py_str("razor bumps"), py_str("shaving rash"), py_str("shaving bumps")]) }
    fn __data165() -> Py { py_tuple(vec![py_str("leg"), py_str("legs"), py_str("bikini"), py_str("bikini line"), py_str("underarm"), py_str("underarms"), py_str("armpit"), py_str("armpits"), py_str("eyebrow"), py_str("eyebrows"), py_str("beard"), py_str("face"), py_str("upper lip"), py_str("chin"), py_str("pubic"), py_str("hair"), py_str("hairs"), py_str("stubble"), py_str("ingrown"), py_str("ingrowing"), py_str("bumps"), py_str("rash"), py_str("irritation"), py_str("redness"), py_str("smooth"), py_str("how do i"), py_str("best way"), py_str("how to"), py_str("sore"), py_str("hurts"), py_str("painful"), py_str("stings"), py_str("itchy"), py_str("sugaring"), py_str("sugar wax")]) }
    fn __data166() -> Py { py_tuple(vec![py_str("ear"), py_str("ears"), py_str("earwax"), py_str("ear wax"), py_str("blocked ear"), py_str("bunged up ear"), py_str("table"), py_str("tables"), py_str("floor"), py_str("floors"), py_str("furniture"), py_str("car"), py_str("cars"), py_str("skirting"), py_str("candle"), py_str("candles"), py_str("surfboard"), py_str("skis"), py_str("ski"), py_str("snowboard"), py_str("boots"), py_str("wood"), py_str("wooden"), py_str("paintwork"), py_str("leather"), py_str("thread"), py_str("eyebrow pencil"), py_str("tweezers"), py_str("tweeze"), py_str("threading"), py_str("bleach"), py_str("tablecloth"), py_str("tablecloths"), py_str("cloth"), py_str("candle wax"), py_str("candle holder"), py_str("wax off"), py_str("wax out of"), py_str("off the table"), py_str("off my table"), py_str("wax stain"), py_str("beard"), py_str("beards"), py_str("shave my head"), py_str("shaving my head"), py_str("head shave"), py_str("shave my face"), py_str("shaving my face"), py_str("clam"), py_str("clams"), py_str("razor clam"), py_str("razor clams"), py_str("razor shell"), py_str("razor shells"), py_str("oyster"), py_str("oysters"), py_str("mussel"), py_str("mussels"), py_str("shellfish"), py_str("toenail"), py_str("toenails"), py_str("toe nail"), py_str("toe nails"), py_str("ingrown toenail"), py_str("ingrowing toenail"), py_str("fingernail"), py_str("fingernails"), py_str("podiatrist"), py_str("chiropodist"), py_str("blood sugar"), py_str("sugar crash"), py_str("crave"), py_str("craves"), py_str("craving"), py_str("cravings"), py_str("stop wanting"), py_str("wanting sugar"), py_str("sugar addiction"), py_str("addicted to sugar"), py_str("sugar out of my diet"), py_str("eating sugar"), py_str("eat sugar"), py_str("stop eating"), py_str("warmer"), py_str("melt warmer"), py_str("wax pot"), py_str("melter"), py_str("clean the wax"), py_str("clean a wax"), py_str("cut down on sugar"), py_str("cut out sugar"), py_str("cutting out sugar"), py_str("cut sugar"), py_str("eat less sugar"), py_str("less sugar"), py_str("so much sugar"), py_str("eating so much sugar"), py_str("sugar habit"), py_str("sugar at night"), py_str("give up sugar"), py_str("quit sugar"), py_str("too much sugar"), py_str("sugar in my diet"), py_str("sugar in your diet"), py_str("sugar cravings"), py_str("sugar craving"), py_str("stop eating sugar"), py_str("added sugar"), py_str("sugar free"), py_str("sugar-free"), py_str("reduce sugar"), py_str("reducing sugar"), py_str("how much sugar"), py_str("sugar intake"), py_str("sugary"), py_str("fizzy drinks"), py_str("sweet tooth"), py_str("sweets"), py_str("craving sugar"), py_str("crave sugar"), py_str("stop craving"), py_str("sugar rush"), py_str("sugar hit")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hair-removal")); __args.push(py_tuple(vec![__data164(), __data165()])); __args.push(py_func("_hair_removal", |args: &[Py]| _hair_removal(args))); __kws.extend(vec![("avoid".to_string(), __data166())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data167() -> Py { py_tuple(vec![py_str("arm"), py_str("arms"), py_str("wrist"), py_str("wrists"), py_str("leg"), py_str("legs"), py_str("ankle"), py_str("ankles"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("hand"), py_str("hands"), py_str("rib"), py_str("ribs"), py_str("collarbone"), py_str("clavicle"), py_str("shoulder"), py_str("elbow"), py_str("shin"), py_str("tibia"), py_str("fibula"), py_str("knee"), py_str("hip"), py_str("femur"), py_str("bone"), py_str("bones"), py_str("fracture"), py_str("fractured"), py_str("break"), py_str("broken"), py_str("cracked"), py_str("cast"), py_str("plaster")]) }
    s__HEAL_PARTS(__data167());
    fn __data168() -> Py { py_tuple(vec![py_str("how long"), py_str("how many weeks"), py_str("how long till"), py_str("how long until"), py_str("take to heal"), py_str("heal"), py_str("heals"), py_str("healing"), py_str("healed"), py_str("mend"), py_str("mends"), py_str("mending"), py_str("recover"), py_str("recovery"), py_str("back to normal"), py_str("back to work"), py_str("back to sport"), py_str("back to running"), py_str("back to football"), py_str("back to the gym"), py_str("in a cast"), py_str("off work"), py_str("how soon"), py_str("when can i"), py_str("play again"), py_str("playing again"), py_str("back to playing"), py_str("back on it"), py_str("again after")]) }
    s__HEAL_TIME(__data168());
    s__A_REST_NOT_A_BONE(stdlib_call("re", "compile", &[py_str("\\b(?:my|your|our|their|his|her)\\s+(?:lunch|tea|coffee|dinner|toilet|rest|water|cigarette|fag)?\\s*breaks?\\b|\\b(?:lunch|tea|coffee|dinner|toilet)\\s+breaks?\\b|\\bbreaks?\\s+(?:at|from|during|in)\\s+(?:work|the shift|a shift|my shift|shift)\\b|\\bbreaks?\\s+between\\s+shifts\\b|\\bbreaks? at work\\b|\\bshould (?:a|my|our) breaks? be\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HEALING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:heal|heals|healing|healed|mend|mends|mending|knit|knits|knitting|cast|plaster|x-ray|xray|fracture|fractured|broken|pinned|pin|pins|physio|physiotherapy|set|setting)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data169() -> Py { py_tuple(vec![py_str("arm"), py_str("arms"), py_str("wrist"), py_str("wrists"), py_str("leg"), py_str("legs"), py_str("ankle"), py_str("ankles"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("hand"), py_str("hands"), py_str("rib"), py_str("ribs"), py_str("collarbone"), py_str("clavicle"), py_str("shoulder"), py_str("elbow"), py_str("shin"), py_str("tibia"), py_str("fibula"), py_str("knee"), py_str("hip"), py_str("femur"), py_str("bone"), py_str("bones"), py_str("fracture"), py_str("fractured"), py_str("surgery"), py_str("pinned"), py_str("metal plate"), py_str("screw"), py_str("screws")]) }
    s__A_PART_NAMED(__data169());
    s__A_JOINT_OF_MEAT(stdlib_call("re", "compile", &[py_str("\\b(?:lamb|beef|pork|chicken|turkey|duck|goose|brisket|gammon|veal|venison|meat|steak|joint of|of lamb|of beef|of pork|roast)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data170() -> Py { py_tuple(vec![py_str("is it broken"), py_str("or broken"), py_str("or sprained"), py_str("do i need an x-ray"), py_str("do i need an xray"), py_str("how do i know"), py_str("how can i tell"), py_str("years ago"), py_str("last year"), py_str("when i was"), py_str("sticking out"), py_str("through the skin"), py_str("bone is showing"), py_str("999"), py_str("ambulance"), py_str("unconscious"), py_str("neck"), py_str("my back"), py_str("back pain"), py_str("lower back"), py_str("slipped disc"), py_str("spine"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my baby"), py_str("my dog"), py_str("my cat"), py_str("my mum"), py_str("my dad"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("cut"), py_str("cuts"), py_str("wound"), py_str("tattoo"), py_str("piercing"), py_str("burn"), py_str("scab"), py_str("sore"), py_str("gums"), py_str("tooth"), py_str("teeth"), py_str("wisdom tooth"), py_str("nail"), py_str("nails"), py_str("phone"), py_str("screen"), py_str("laptop"), py_str("tv"), py_str("telly"), py_str("washing machine"), py_str("car"), py_str("boiler"), py_str("hoover"), py_str("vacuum"), py_str("glasses"), py_str("heart"), py_str("heartbroken"), py_str("replacement"), py_str("knee replacement"), py_str("hip replacement"), py_str("resurfacing"), py_str("stock"), py_str("broth"), py_str("bouillon"), py_str("simmer"), py_str("carcass"), py_str("for stock"), py_str("for broth"), py_str("bone broth"), py_str("soup"), py_str("cook the bones"), py_str("roast the bones"), py_str("boil the bones"), py_str("beef bones"), py_str("chicken bones"), py_str("lamb bones"), py_str("ham bone"), py_str("bones for"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("cooked"), py_str("boil"), py_str("boils"), py_str("boiled"), py_str("roast"), py_str("roasts"), py_str("roasted"), py_str("oven"), py_str("barbecue"), py_str("bbq"), py_str("grill"), py_str("grilled"), py_str("slow cooker"), py_str("recipe"), py_str("marinade"), py_str("marinate"), py_str("smoker"), py_str("air fryer")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bone-healing")); __args.push(py_tuple(vec![g__HEAL_PARTS(), g__HEAL_TIME()])); __args.push(py_func("_healing", |args: &[Py]| _healing(args))); __kws.extend(vec![("guard".to_string(), py_func("_not_a_break_from_work", |args: &[Py]| _not_a_break_from_work(args))), ("avoid".to_string(), __data170())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data171() -> Py { py_tuple(vec![py_str("muscle"), py_str("muscles"), py_str("muscular"), py_str("sprain"), py_str("sprained"), py_str("strain"), py_str("strained"), py_str("pulled"), py_str("pull"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("achy"), py_str("sore"), py_str("soreness"), py_str("stiff"), py_str("stiffness"), py_str("injury"), py_str("injured"), py_str("swollen"), py_str("swelling"), py_str("twinge"), py_str("cramp"), py_str("cramps"), py_str("tight"), py_str("back"), py_str("neck"), py_str("shoulder"), py_str("shoulders"), py_str("knee"), py_str("knees"), py_str("ankle"), py_str("ankles"), py_str("wrist"), py_str("wrist"), py_str("elbow"), py_str("hip"), py_str("hips"), py_str("thigh"), py_str("calf"), py_str("shin"), py_str("foot"), py_str("feet"), py_str("arm"), py_str("arms"), py_str("leg"), py_str("legs"), py_str("joint"), py_str("joints"), py_str("tendon"), py_str("ligament"), py_str("whiplash"), py_str("flaring"), py_str("flare")]) }
    s_ACHE_WORDS(call_builtin("frozenset", &[__data171()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data172() -> Py { py_tuple(vec![py_str("heat pack"), py_str("heat packs"), py_str("heat pad"), py_str("heat pads"), py_str("ice pack"), py_str("ice packs"), py_str("hot pack"), py_str("cold pack"), py_str("cold packs"), py_str("wheat bag"), py_str("wheat bags"), py_str("ice bath"), py_str("cold compress"), py_str("hot compress"), py_str("frozen peas"), py_str("ice cubes"), py_str("hot water bottle"), py_str("ice"), py_str("icing")]) }
    s_COMPRESS_WORDS(call_builtin("frozenset", &[__data172()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data173() -> Py { py_tuple(vec![py_str("ice it"), py_str("ice that"), py_str("ice my"), py_str("ice the"), py_str("icing it"), py_str("put ice"), py_str("use ice"), py_str("apply ice"), py_str("hold ice"), py_str("hold an ice"), py_str("ice pack on"), py_str("ice pack to"), py_str("pack on"), py_str("put a cold"), py_str("put heat"), py_str("use heat"), py_str("apply heat"), py_str("heat it"), py_str("heat that"), py_str("heat my"), py_str("heat the"), py_str("heat or ice"), py_str("ice or heat"), py_str("hot or cold"), py_str("cold or hot"), py_str("compress on"), py_str("should i ice"), py_str("should i heat"), py_str("frozen peas on"), py_str("peas on"), py_str("heat better than ice"), py_str("hot better than cold"), py_str("better than ice"), py_str("than ice"), py_str("better than heat"), py_str("than heat"), py_str("heat vs ice"), py_str("ice vs heat"), py_str("heat versus"), py_str("ice versus"), py_str("heat or cold"), py_str("cold or heat"), py_str("ice or hot"), py_str("heat and ice")]) }
    s__ON_A_BODY(__data173());
    s__HEAT_OR_ICE_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:heat|heating|ice|icing|hot|cold)\\s+(?:the|my|our|that|this|a|an)\\s+([a-z']+)")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data174() -> Py { py_tuple(vec![py_str("room"), py_str("rooms"), py_str("house"), py_str("houses"), py_str("home"), py_str("flat"), py_str("kitchen"), py_str("bedroom"), py_str("bathroom"), py_str("hall"), py_str("hallway"), py_str("lounge"), py_str("garage"), py_str("shed"), py_str("office"), py_str("conservatory"), py_str("loft"), py_str("attic"), py_str("cellar"), py_str("caravan"), py_str("car"), py_str("van"), py_str("bed"), py_str("bath"), py_str("shower"), py_str("pool"), py_str("greenhouse"), py_str("oven"), py_str("pan"), py_str("pot"), py_str("kettle"), py_str("mug"), py_str("cup"), py_str("bottle"), py_str("thermos"), py_str("cake"), py_str("buns"), py_str("bread"), py_str("milk"), py_str("wine"), py_str("beer"), py_str("water"), py_str("radiator"), py_str("radiators"), py_str("boiler"), py_str("heating"), py_str("thermostat"), py_str("air"), py_str("whole house"), py_str("place"), py_str("window"), py_str("windows"), py_str("door"), py_str("towel"), py_str("towel rail"), py_str("dehumidifier"), py_str("humidifier"), py_str("washing machine"), py_str("dryer"), py_str("tumble"), py_str("dishwasher"), py_str("microwave"), py_str("fridge"), py_str("freezer")]) }
    s__A_THING_THAT_IS_NOT_A_BODY(call_builtin("frozenset", &[__data174()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data175() -> Py { py_tuple(vec![py_str("cook"), py_str("cooks"), py_str("cooked"), py_str("cooking"), py_str("boil"), py_str("boils"), py_str("boiled"), py_str("boiling"), py_str("oven"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("recipe"), py_str("roast"), py_str("roasted"), py_str("fry"), py_str("fried"), py_str("frying"), py_str("steam"), py_str("steamed"), py_str("steaming"), py_str("microwave"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawed"), py_str("saucepan"), py_str("freezer"), py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("ice cube tray"), py_str("ice tray"), py_str("in the fridge"), py_str("black ice"), py_str("gear"), py_str("pull away"), py_str("drive"), py_str("driving"), py_str("road"), py_str("roads"), py_str("tyre"), py_str("tyres"), py_str("windscreen"), py_str("brake"), py_str("braking"), py_str("grit"), py_str("gritting"), py_str("pavement"), py_str("pavements"), py_str("snow"), py_str("icy")]) }
    s__NOT_ON_A_BODY(__data175());
    fn __data176() -> Py { py_tuple(vec![py_str("heat"), py_str("ice"), py_str("hot"), py_str("cold"), py_str("warm"), py_str("warmth"), py_str("heat pack"), py_str("ice pack"), py_str("hot pack"), py_str("cold pack"), py_str("wheat bag"), py_str("heat pad"), py_str("hot water bottle"), py_str("ice bath"), py_str("cold compress"), py_str("frozen peas"), py_str("ice cubes")]) }
    fn __data177() -> Py { py_tuple(vec![py_str("should i use"), py_str("should i put"), py_str("should i apply"), py_str("do i use"), py_str("or heat"), py_str("or ice"), py_str("heat or ice"), py_str("ice or heat"), py_str("which is better"), py_str("which one"), py_str("better for"), py_str("good for"), py_str("better than"), py_str("rather than"), py_str("heat or cold"), py_str("cold or heat"), py_str("heat and cold"), py_str("what should i use"), py_str("help"), py_str("helps"), py_str("can i put"), py_str("can i use"), py_str("can i apply"), py_str("should i"), py_str("how long"), py_str("how often"), py_str("am i meant to")]) }
    fn __data178() -> Py { py_tuple(vec![py_str("stiff neck"), py_str("neck ache"), py_str("crick"), py_str("period"), py_str("periods"), py_str("cramps"), py_str("burn"), py_str("burned"), py_str("burnt"), py_str("scald"), py_str("scalded"), py_str("headache"), py_str("migraine"), py_str("temperature"), py_str("fever"), py_str("nappy rash"), py_str("toothache"), py_str("bruise"), py_str("bruises"), py_str("bruised"), py_str("hot bath"), py_str("bath"), py_str("sore muscles"), py_str("aching muscles"), py_str("dog"), py_str("cat"), py_str("baby"), py_str("pregnant"), py_str("pregnancy"), py_str("cheaper"), py_str("cheap"), py_str("cost"), py_str("costs"), py_str("expensive"), py_str("electricity"), py_str("energy"), py_str("bill"), py_str("bills"), py_str("tariff"), py_str("dehumidifier"), py_str("kilowatt"), py_str("kwh"), py_str("per hour"), py_str("running cost"), py_str("central heating"), py_str("heating on"), py_str("gas heating"), py_str("electric heating"), py_str("heat the house"), py_str("heating the house"), py_str("heating bill"), py_str("heating the flat"), py_str("radiator"), py_str("radiators"), py_str("boiler"), py_str("heat pump")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("heat-or-ice")); __args.push(py_tuple(vec![__data176(), __data177()])); __args.push(py_str("Cold for a fresh knock and heat for a stiff, old ache. Ice in the first 48 hours of a sprain, a bump or a pulled muscle: a bag of frozen peas or an ice pack wrapped in a tea towel, 15 to 20 minutes at a time, every two or three hours, never straight on the skin and never for longer, because that burns. After that, heat - a hot water bottle, a wheat bag or a warm bath, 15 to 20 minutes - loosens a muscle that has tightened up, so it is the one for a stiff neck or a back that has been bad for days. Never heat a fresh injury, a hot swollen joint or anywhere you cannot feel properly, and do not use either on broken skin. Heat on a sprain after two days is fine; ice on a stiff neck is not. If the joint is hot, very swollen or you cannot use it, that is a look-at-it-today rather than a compress.")); __kws.extend(vec![("guard".to_string(), py_func("_a_place_you_would_ice", |args: &[Py]| _a_place_you_would_ice(args))), ("avoid".to_string(), __data178())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data179() -> Py { py_tuple(vec![py_str("hamstring"), py_str("hamstrings"), py_str("pulled hamstring"), py_str("torn hamstring"), py_str("hamstring strain"), py_str("hamstring injury"), py_str("calf"), py_str("calves"), py_str("calf muscle"), py_str("pulled calf"), py_str("torn calf"), py_str("calf strain"), py_str("groin"), py_str("groin strain"), py_str("pulled groin"), py_str("adductor"), py_str("thigh muscle"), py_str("quad"), py_str("quads"), py_str("quadriceps"), py_str("torn muscle"), py_str("tore a muscle"), py_str("muscle tear")]) }
    s__MUSCLE_PARTS(__data179());
    fn __data180() -> Py { py_tuple(vec![py_str("build muscle"), py_str("building muscle"), py_str("muscle mass"), py_str("gain muscle"), py_str("grow muscle"), py_str("put on muscle"), py_str("muscle tone"), py_str("bulk"), py_str("protein"), py_str("shakes"), py_str("sets and reps"), py_str("workout"), py_str("gym"), py_str("soreness"), py_str("doms"), py_str("day after"), py_str("next day"), py_str("delayed onset"), py_str("my dog"), py_str("my cat"), py_str("my puppy"), py_str("my horse"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my baby"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my dad"), py_str("pulled pork"), py_str("pulled beef"), py_str("pulled chicken"), py_str("pulled jackfruit"), py_str("pork"), py_str("beef"), py_str("chicken"), py_str("jackfruit"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("cooked"), py_str("oven"), py_str("slow cooker"), py_str("recipe"), py_str("air fryer"), py_str("bake"), py_str("baked"), py_str("stock"), py_str("sieve"), py_str("drain"), py_str("juice"), py_str("yoghurt"), py_str("yogurt"), py_str("curds"), py_str("heart"), py_str("chest"), py_str("head"), py_str("eye"), py_str("eyes"), py_str("jaw"), py_str("stomach"), py_str("tummy"), py_str("swollen"), py_str("swelling"), py_str("hot"), py_str("red"), py_str("clot"), py_str("numb"), py_str("tingling"), py_str("one side"), py_str("pregnant"), py_str("pregnancy"), py_str("surgery"), py_str("operation"), py_str("replacement")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("muscle-healing")); __args.push(py_tuple(vec![g__MUSCLE_PARTS(), g__HEAL_TIME()])); __args.push(py_func("_muscle_healing", |args: &[Py]| _muscle_healing(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data180())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data181() -> Py { py_tuple(vec![py_str("piercing"), py_str("piercings"), py_str("pierced ear"), py_str("ear piercing"), py_str("nose piercing"), py_str("nose ring"), py_str("nose stud"), py_str("nose hoop"), py_str("septum"), py_str("helix"), py_str("tragus"), py_str("daith"), py_str("cartilage piercing"), py_str("earring hole"), py_str("earring"), py_str("belly piercing"), py_str("belly button piercing"), py_str("belly bar"), py_str("lip piercing"), py_str("eyebrow piercing"), py_str("industrial piercing"), py_str("earlobe")]) }
    fn __data182() -> Py { py_tuple(vec![py_str("red"), py_str("redness"), py_str("swollen"), py_str("swelling"), py_str("hot"), py_str("weeping"), py_str("oozing"), py_str("pus"), py_str("yellow"), py_str("green"), py_str("infected"), py_str("infection"), py_str("spreading"), py_str("streaking"), py_str("throbbing"), py_str("smelly"), py_str("fever"), py_str("angry"), py_str("sore and"), py_str("hurts")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("piercing-infection")); __args.push(py_tuple(vec![__data181(), __data182()])); __args.push(py_func("_piercing_infection", |args: &[Py]| _piercing_infection(args))); __kws.extend(vec![("guard".to_string(), py_func("_a_piercing_in_trouble", |args: &[Py]| _a_piercing_in_trouble(args))), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("aftercare"), py_str("keloid"), py_str("crusty"), py_str("crust"), py_str("when can i change"), py_str("downsize"), py_str("can i swim"), py_str("how do i clean"), py_str("how do i look after")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__TAMPON_WORD(stdlib_call("re", "compile", &[py_str("\\btampons?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_TAMPON_LINE(stdlib_call("re", "compile", &[py_str("\\b(bins?|bin it|dispose|disposal|throw (?:it|them) away|flush\\w*|down the loo|wrapper|applicator|recycl\\w*|compost\\w*|landfill)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data183() -> Py { py_tuple(vec![py_str("how long"), py_str("how often"), py_str("leave"), py_str("leaves"), py_str("stay in"), py_str("stays in"), py_str("staying in"), py_str("keep in"), py_str("keeps in"), py_str("keeping it in"), py_str("in for"), py_str("overnight"), py_str("sleep"), py_str("slept"), py_str("all night"), py_str("asleep"), py_str("fell asleep"), py_str("hours"), py_str("long can"), py_str("long should"), py_str("when should"), py_str("forgot"), py_str("forget"), py_str("change"), py_str("changed"), py_str("changing"), py_str("safe"), py_str("okay"), py_str("ok"), py_str("can i"), py_str("should i"), py_str("wear"), py_str("wearing"), py_str("use"), py_str("using"), py_str("put one in")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tampon-how-long")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tampon"), py_str("tampons")]), __data183()])); __args.push(py_str("Change a tampon every four to six hours and never leave one in for more than eight, so take it out before bed rather than sleep through with the same one. Use the lowest absorbency that still lasts four hours, wash your hands before and after, and if one has been in longer than that take it out now and watch for a fever, a rash like sunburn, vomiting, dizziness or a sore throat: that is toxic shock syndrome, which is rare but serious, so ring 111 for advice. Contact lenses and retainers have their own limits, and pads rather than tampons are the answer during pregnancy and for the first weeks after a baby.")); __kws.extend(vec![("guard".to_string(), py_func("_a_tampon_worn_too_long", |args: &[Py]| _a_tampon_worn_too_long(args))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data184() -> Py { py_tuple(vec![py_str("walk in centre"), py_str("walk-in centre"), py_str("walk in center"), py_str("walk in clinic"), py_str("walk-in clinic"), py_str("walk in service"), py_str("urgent care centre"), py_str("urgent care center"), py_str("urgent treatment centre"), py_str("minor injuries unit"), py_str("minor injuries"), py_str("out of hours gp"), py_str("out of hours doctor"), py_str("the walk in")]) }
    fn __data185() -> Py { py_tuple(vec![py_str("wait"), py_str("waits"), py_str("waiting"), py_str("queue"), py_str("how long"), py_str("long will"), py_str("long do i"), py_str("how quick"), py_str("busy"), py_str("packed"), py_str("hours"), py_str("tonight"), py_str("this evening"), py_str("weekend")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the-wait-at-a-walk-in-centre")); __args.push(__data184()); __args.push(py_str("Nobody can tell you before you go: a walk-in centre sees people in the order they need seeing rather than the order they arrived, so a queue of two hours can move in twenty minutes when something urgent comes in, and a quiet room can still mean a long wait. Ring the centre or look at its own site on the day for the wait it is running, and go early rather than late in the evening. Take a drink, a book and any medicines you take with you, and tell the desk if you get worse while you are sitting there rather than wait your turn.")); __kws.extend(vec![("ask".to_string(), __data185()), ("avoid".to_string(), py_tuple(vec![py_str("how do i get"), py_str("parking"), py_str("postcode"), py_str("address"), py_str("where is"), py_str("nearest"), py_str("walk up"), py_str("walk to"), py_str("how far"), py_str("open on christmas"), py_str("phone number")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__OFF_A_THING(py_tuple(vec![py_str("came off"), py_str("fell off"), py_str("fell from"), py_str("come off"), py_str("fall off"), py_str("went over the"), py_str("over the handlebars"), py_str("came a cropper")]));
    fn __data186() -> Py { py_tuple(vec![py_str("chain"), py_str("chains"), py_str("derailleur"), py_str("gears"), py_str("gearset"), py_str("brake"), py_str("brakes"), py_str("pedal"), py_str("pedals"), py_str("wheel"), py_str("wheels"), py_str("tyre"), py_str("tyres"), py_str("tire"), py_str("tires"), py_str("saddle"), py_str("sprocket"), py_str("cassette"), py_str("handlebar"), py_str("mudguard"), py_str("rack"), py_str("pannier"), py_str("battery"), py_str("motor"), py_str("throttle"), py_str("exhaust"), py_str("spark plug"), py_str("clutch"), py_str("pill"), py_str("medication"), py_str("my meds"), py_str("wagon"), py_str("a lorry"), py_str("the back of a lorry"), py_str("radar"), py_str("the list"), py_str("diet"), py_str("my food"), py_str("the challenge"), py_str("bandwagon"), py_str("sales"), py_str("orders"), py_str("shares"), py_str("price"), py_str("prices"), py_str("paint"), py_str("button"), py_str("handle"), py_str("sticker"), py_str("label"), py_str("wallpaper"), py_str("tiles"), py_str("the map"), py_str("nearly fell"), py_str("nearly came"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("sore"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("pain"), py_str("painful"), py_str("broken"), py_str("break"), py_str("fracture"), py_str("swelling"), py_str("swollen"), py_str("can't move"), py_str("cant move")]) }
    s__FALL_AVOID(__data186());
    fn __data187() -> Py { py_tuple(vec![py_str("bike"), py_str("bikes"), py_str("bicycle"), py_str("cycle"), py_str("horse"), py_str("pony"), py_str("scooter"), py_str("motorbike"), py_str("motorcycle"), py_str("moped"), py_str("ladder"), py_str("trampoline"), py_str("skateboard"), py_str("skates"), py_str("skis"), py_str("sled"), py_str("sledge"), py_str("roof"), py_str("wall"), py_str("stairs"), py_str("steps"), py_str("horse"), py_str("quad"), py_str("go kart"), py_str("swing"), py_str("climbing frame"), py_str("bruise"), py_str("bruised"), py_str("graze"), py_str("grazed"), py_str("gashed"), py_str("landed"), py_str("hit"), py_str("banged"), py_str("knocked"), py_str("arm"), py_str("wrist"), py_str("elbow"), py_str("shoulder"), py_str("collarbone"), py_str("knee"), py_str("hip"), py_str("ankle"), py_str("hand"), py_str("head"), py_str("ribs"), py_str("walk"), py_str("weight")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("came-off-a-thing")); __args.push(py_tuple(vec![g__OFF_A_THING(), __data187()])); __args.push(py_str("Coming off is a fall, so check the head first: a bang that knocked them out, made them sick, left them confused or gave them a bad headache is A&E the same day, and so is anything they cannot remember. Most falls land on a hand or a shoulder, and a wrist that cannot take weight or an arm that cannot be lifted across the chest is worth an X-ray today - the collarbone is the break people miss. Grazes want cool running water, a dressing and no scrubbing. Ice a bruise for 20 minutes at a time with a cloth round it, take paracetamol or ibuprofen if you can, and expect it to be stiffer tomorrow than today. A helmet that took the hit is finished, even with no crack you can see.")); __kws.extend(vec![("avoid".to_string(), g__FALL_AVOID()), ("bias".to_string(), Py::Int(0))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _the_nail_is_a_persons(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("False when the nail in the line is a piece of metal, not one on a person.\n\n    Used as an entry `guard` on the rows that treat a nail as something to cut, bite or cure:\n    those rows read a how-to (\"how do i ... nail ...\") as a question about the habit or the\n    trimming, and the workshop nouns in the line are the reading that says otherwise. A nail\n    that was hit is not affected - that row is about the injury, not the nail.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    words = { let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    }
    if py_truthy(&Py::Bool(!py_contains(&said.clone(), &py_str("nail")))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g__WORKSHOP(), "search", &[said.clone()])));
    Py::None
}
pub fn _the_part_is_a_persons(args: &[Py]) -> Py {
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
    let _ = py_str("False when the body word in the line belongs to a thing, not to the person.\n\n    A chair, a table, a shoe and a sofa have legs, backs and heels too. The reading already\n    knows which noun owns the part (`_a_thing_owns_the_part`), so every row whose key is a\n    part plus a state - a sore back, dry skin, a crack in the skin - stands aside when the\n    owner is a thing, and the repair answer for that thing gets the line instead. The test\n    is used as an entry `guard`, so it runs on every line and reads the same reading every\n    row does.\n    ");
    return Py::Bool(!py_truthy(&_a_thing_owns_the_part(&[understood.clone()])));
    Py::None
}
pub fn the_body_is_the_persons(args: &[Py]) -> Py {
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
    let _ = py_str("False when the body the line names is somebody else's.\n\n    \"my dad is 70\" then \"hes got a bad hip\" is one person's trouble told over two turns, and the\n    carer's line (\"Is it worse at any time of day, or about the same all the time?\") is the right\n    one for it. The reading already knows whose the state is, the way the composer's carer pool\n    does, so a row of the person's own advice stands aside when `state_owner` says \"other\".\n\n    A carer keeps talking about the other person after the turn that said so, and that turn has no\n    owner of its own: \"my mum is 80\" / \"shes got a bad knee\" / \"she says its the weather\" got the\n    mum's knee answered with the drill for the asker's own. Nobody says \"she\" about their own\n    body, so a third person with no first person on the line is somebody else's too.\n    ");
    if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("state_owner")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("other")))) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&words.clone(), &g__THE_PERSONS_OWN_WORDS())) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g__SOMEONE_ELSE())));
    Py::None
}
pub fn the_part_that_keeps_grumbling(args: &[Py]) -> Py {
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
    let _ = py_str("The person's own part, and one they can still use: the standing-condition rows' guard.");
    return { let __b = the_body_is_the_persons(&[understood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__CANNOT_USE_IT(), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]))) } };
    Py::None
}
pub fn _names_a_pregnancy(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says, in words, that someone is pregnant.\n\n    A key group only nudges the score, it does not have to be there: without this the swelling\n    row answered every swelling with a part in it, and \"my feet swell when im on them all day\"\n    got the pregnancy answer.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PREGNANCY_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _dates_a_pregnancy_in_weeks(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let _ = py_str("True when the line dates a pregnancy in weeks and is about swelling.\n\n    Used as a `guard`, because a shape only nudges the score: without this the row answered\n    \"my feet swell when im on them all day\", which says no dates at all.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return _a_figure_in_weeks(&[line.clone()]);
    Py::None
}
pub fn _a_figure_in_weeks(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("A week figure that dates a pregnancy, on a line that is about swelling.\n\n    The swelling has to be there too: \"im 20 weeks and my feet hurt when i run\" says how far\n    along it is and names a part, and the running answer is the one it wants.\n    ");
    line = mcall(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    return { let __b = call_builtin("bool", &[mcall(&g__WEEKS_ALONG(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__SWELLING(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn a_persons_temperature(args: &[Py]) -> Py {
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
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reading says the temperature in the line is somebody's own.\n\n    The one word \"temperature\" is a kitchen, an oven, a room and a wash as well as a fever,\n    and the kitchen row that teaches the meat probe is keyed on it: \"my temperature is 39,\n    what should i do\" and \"i have a temperature of 38 what should i do\" were both answered\n    with \"Push the probe into the thickest part, away from bone, fat and the tin\". The reading\n    has already settled which the line is (`body` is `temperature` only when the illness is the\n    speaker's own), so the rows about a thermometer in a drawer stand aside for it: this is\n    used as their `guard`, the same way `the_stomach_is_what_it_is_about` is.\n\n    Somebody else's fever reads `body` empty, because the engine keeps that key for the\n    speaker, and \"she has a temperature of 39 what should i do\" was the meat probe too. A\n    person named beside the word makes it somebody's own either way, and the words for a\n    person are the engine's own (`kinds.OTHER_PEOPLE`), never a list of names kept here.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[words.clone(), slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) { let __unpk2099 = py_unpack(&__item.clone(), 2, None); let mut one: Py = __unpk2099[0].clone(); let mut other: Py = __unpk2099[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_TEMPERATURE_WORDS(), &one.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THING_AFTER_THE_WORD(), &other.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("body")]), &py_str("temperature")))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TEMPERATURE_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WE_OUR_OURS(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[words.clone(), slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) { let __unpk2345 = py_unpack(&__item.clone(), 2, None); let mut one: Py = __unpk2345[0].clone(); let mut other: Py = __unpk2345[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_POSSESSIVE_OF_A_PERSON(), &one.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TEMPERATURE_WORDS(), &other.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_stomach_is_what_it_is_about(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reading says the line is about the person's stomach.\n\n    Two everyday words mean a thing as well as a part of a person: a stool is a chair, and\n    green leaves keep a week in the drawer. \"loose stools for 3 days\" was answered with the\n    wood-glue repair, and \"my poo has been green for a week\" with the shelf life of salad,\n    because those rows share their words with a chair and a bag of leaves. The reading has\n    already settled which the line is, so the furniture and the salad rows stand aside for\n    it: this is used as their `guard`, the same way `_the_part_is_a_persons` is.\n    ");
    return { let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[{ let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("body")]), &py_str("stomach"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&understood.clone(), "get", &[py_str("own_output")]), &Py::None)) } }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _see_today(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut part: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The same-day paragraph, spoken about the part the line actually named.\n\n    The paragraph was written for a back, a side and a calf. A swollen knee reached it and was\n    told about chest pain and a clot after a flight, which reads as if the bot had not heard\n    which part hurt - worse than saying nothing. So the words of the line (and of the turn it\n    leans on) pick the sentence: a joint speaks for itself, a hot one leads with the flag.\n    ");
    words = _words(&[understood.clone()]);
    part = _part_named(&[words.clone(), understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CLOT_AND_CHEST(), &part.clone())) } }) {
    return g_SEE_TODAY_BACK_AND_CALF();
    }
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g_HOT_FLAGS()]).unwrap_or_else(|e| exc::raise(e)))) {
    return op_add(&op_add(&py_str("A "), &part.clone()), &py_str(" that has gone hot and swollen wants looking at today rather than iced and waited on: ring 111 this morning, and go to A&E if there is a fever with it or you cannot put weight on it at all."));
    }
    return op_add(&op_add(&py_str("A swollen "), &part.clone()), &py_str(" is usually fluid - from a knock or overuse, or from a tablet or the heart when it is on both sides - so have it looked at this week and keep it raised when you sit. Ring 111 today if it is hot, red and spreading, or painful on just one side."));
    Py::None
}
pub fn _part_named(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut _name: Py = Py::None;
    let mut _reply: Py = Py::None;
    let mut part: Py = Py::None;
    let mut spelled: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The part of the body the line names, in the words the person used.\n\n    \"back\" is the one part word that is also a particle, so a caller that has the reading hands\n    it over and a back the line only sends or comes back is not a part: \"my knee hurts and im\n    going back to work\" is a knee, and the rule called it a back.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_is(&understood.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("back"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_back_is_a_back(&[understood.clone()]))) } } } }) {
    words = op_sub(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("back")]));
    }
    let __iter1 = py_iter_or_raise(&g_REGIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _name = __unpk2[0].clone();
    spelled = __unpk2[1].clone();
    _reply = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&spelled.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &word.clone()))) {
    return mcall(&g_PART_ONE(), "get", &[word.clone(), word.clone()]);
    }
    }
    }
    let __iter4 = py_iter_or_raise(&g_JOINTS());
    for __item in __iter4 {
    part = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &part.clone()))) {
    return mcall(&g_PART_ONE(), "get", &[part.clone(), part.clone()]);
    }
    }
    return py_str("");
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when one of these words or phrases is in the line, as a whole word.");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _red_flag(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut key: Py = Py::None;
    let mut own: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut ruled_out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The emergency card, or nothing at all when the numb part is not the lower half.\n\n    \"my back hurts and my hand has gone numb\" is a hand question and keeps the answer it had\n    before this entry could read \"gone numb\"; \"no feeling between my legs and back pain\" is\n    the flag, and so is anything that names the waterworks.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return g_RED_FLAG_CARD();
    }
    said = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("")]), "replace", &[py_str("'"), py_str("")]);
    text = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z]+"), py_str(" "), said.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("words"), py_str("background")]));
    for __item in __iter1 {
    key = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug3 = op_add(&text.clone(), &op_add(&mcall(&mcall(&word.clone(), "lower", &[]), "replace", &[py_str("'"), py_str("")]), &py_str(" ")));
    text = __aug3;
    }
    }
    }
    if py_truthy(&_names(&[text.clone(), g_SPELLED_OUT()])) {
    return g_RED_FLAG_CARD();
    }
    own = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    ruled_out = own.clone();
    let __iter4 = py_iter_or_raise(&py_tuple(vec![py_str("no back pain"), py_str("no backache"), py_str("no back ache"), py_str("no pain in my back"), py_str("without back pain"), py_str("no leg pain"), py_str("no pain in my legs"), py_str("no weakness"), py_str("no trouble weeing"), py_str("no trouble going for a wee"), py_str("no problems weeing"), py_str("no trouble peeing"), py_str("no numbness in my legs"), py_str("no pain"), py_str("my back is fine"), py_str("my back is ok"), py_str("my back is okay"), py_str("back is fine"), py_str("without a bad back"), py_str("no problems with my back")]));
    for __item in __iter4 {
    phrase = __item.clone();
    ruled_out = mcall(&ruled_out.clone(), "replace", &[phrase.clone(), py_str(" ")]);
    }
    if py_truthy(&{ let __b = _names(&[ruled_out.clone(), g_BACK_NAMES()]); if !py_truthy(&__b) { __b } else { { let __b = _names(&[ruled_out.clone(), g_NUMB_ANYWHERE()]); if !py_truthy(&__b) { __b } else { _names(&[ruled_out.clone(), g_NUMB_LOWER()]) } } } }) {
    return g_RED_FLAG_CARD();
    }
    if py_truthy(&{ let __b = _names(&[ruled_out.clone(), g_SADDLE_PARTS()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[ruled_out.clone(), g_FLAG_WITH_IT()]))) } }) {
    return g_SADDLE_CARD();
    }
    if py_truthy(&{ let __b = _names(&[text.clone(), g_LOWER_HALF()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[text.clone(), g_SOMEWHERE_ELSE()]))) } }) {
    return g_RED_FLAG_CARD();
    }
    return py_str("");
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every word of the line, and of the turn it leans on, as a set.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("words"), py_str("background")]));
    for __item in __iter1 {
    key = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "add", &[mcall(&word.clone(), "lower", &[])]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _dose_for_the_drug_named(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut codeine: Py = Py::None;
    let mut ibuprofen: Py = Py::None;
    let mut paracetamol: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The dose of the painkiller the line actually named, first.\n\n    \"how much ibuprofen can i take\" opened on paracetamol tablets, because one entry answers\n    every drug and the first sentence was written for paracetamol. STYLE asks for the answer in\n    the first sentence, and the first sentence of a dose question is the figure for the packet\n    the person is holding. A line that names no drug, or names two, keeps the pair answer.\n    ");
    words = call_builtin("set", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("aspirin"), py_str("disprin"), py_str("anadin"), py_str("naproxen"), py_str("diclofenac"), py_str("voltarol"), py_str("tramadol"), py_str("solpadeine"), py_str("veganin"), py_str("feminax"), py_str("migraleve"), py_str("dydramol"), py_str("paracodol"), py_str("dihydrocodeine")]))) {
    return Py::None;
    }
    paracetamol = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("paracetamol"), py_str("acetaminophen"), py_str("calpol"), py_str("panadol")]))]).unwrap_or_else(|e| exc::raise(e));
    ibuprofen = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("ibuprofen"), py_str("nurofen"), py_str("brufen")]))]).unwrap_or_else(|e| exc::raise(e));
    codeine = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("codeine"), py_str("co-codamol"), py_str("co codamol"), py_str("cocodamol"), py_str("codamol")]))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = codeine.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&paracetamol.clone())) } }) {
    return py_str("Co-codamol is paracetamol and codeine together, so the paracetamol limit is what caps it: two 500/8mg tablets, four times in 24 hours at most, eight a day. The codeine is what makes it constipating and drowsy, so days rather than weeks, and don't drive until you know how it treats you. Don't add paracetamol on top: over 4g a day from any mix of products is a liver emergency.");
    }
    if py_truthy(&{ let __b = ibuprofen.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&paracetamol.clone())) } }) {
    return py_str("Ibuprofen for an adult is 200 to 400mg every four to six hours, up to three times a day, with food, and no more than 1200mg a day without a prescription - six of the 200mg tablets. Skip it if you have a stomach ulcer, kidney trouble or are on a blood thinner, and in the last three months of pregnancy. Use the lowest dose that works, and check a cold remedy does not already contain it.");
    }
    return py_str("Two 500mg paracetamol tablets, four times in 24 hours at most, four hours apart - 4g, or eight tablets, a day, and less if you have liver trouble or drink heavily. Ibuprofen is 200 to 400mg every four to six hours with food, and no more than 1200mg a day without a prescription. The packet is the one to trust, and check a cold remedy does not already contain paracetamol.");
    Py::None
}
pub fn _a_piercing_in_trouble(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about a piercing that is showing a sign of infection.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[py_list(vec![call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__PIERCING_PIECE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__PIERCING_TROUBLE(), "search", &[said.clone()]) } }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g__PIERCING_NOT_TROUBLE(), "search", &[said.clone()])));
    Py::None
}
pub fn _piercing_infection(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut lead: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A sign of infection in a new piercing: keep it clean, keep the jewellery in, and know the\n    line where it stops being a cleaning question.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[py_list(vec![call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])])]);
    lead = py_str("A piercing with spreading redness, heat, pus or swelling is infected, and it needs cleaning and often antibiotics rather than patience.");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bfever\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    lead = py_str("An infected piercing with a fever is one to have looked at today - a fever means it is not staying local.");
    }
    return op_add(&lead.clone(), &py_str(" Leave the jewellery in unless a piercer takes it out: the hole closes over the infection and traps it, and a piercer can fit a longer bar if the swelling has outgrown the one you have. Clean it twice a day with sterile saline on a clean pad, let water run over it in the shower, and stop touching or turning it. Warm salty water soaks for ten minutes twice a day ease the soreness. See a piercer or a GP within a day or two if the redness spreads more than a centimetre a day, the skin round it is hot, there is pus that keeps coming, or the swelling swallows the jewellery - and go straight away if the redness streaks towards the body, or you have a fever with it. Cartilage piercings are the ones that go wrong: a hot, swollen helix or nose that is spreading is worth being seen rather than watched."));
    Py::None
}
pub fn _a_healing_piercing_not_a_troubled_one(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True while the line is about healing a piercing rather than about a sign of infection.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[py_list(vec![call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])])]);
    if py_truthy(&mcall(&g__PIERCING_BUMP(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g__PIERCING_TROUBLE(), "search", &[said.clone()])));
    Py::None
}
pub fn _piercing_healing(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A lobe heals in weeks; a nose or an ear's cartilage takes twice as long, and the line knows\n    which one it asked about.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[py_list(vec![call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])])]);
    if py_truthy(&mcall(&g__CARTILAGE_PIERCING(), "search", &[said.clone()])) {
    return op_add(&py_str("A cartilage piercing - a nose, a nostril, a septum, a helix - takes six to twelve weeks and often months to settle, where an earlobe is six to eight weeks."), &g__PIERCING_ROUTINE());
    }
    return op_add(&py_str("An earlobe takes six to eight weeks to heal and cartilage six to twelve weeks, though cartilage can take months to settle."), &g__PIERCING_ROUTINE());
    Py::None
}
pub fn _an_errand_the_dentist_card_answers(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks how long a visit takes, or asks to find or join a practice.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__A_BECOMING_A_DENTIST(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__A_PRACTICE_ERRAND(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_LIST_THAT_IS_LONG(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__A_LENGTH_OF_A_VISIT(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_lengths_of_the_visit(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut named: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("Every length the visit has, with the one the line named in the first sentence.");
    named = py_list(vec![]);
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("filling"), py_str("fillings")]))) {
    let _ = mcall(&named.clone(), "append", &[g__A_FILLING()]);
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &py_set(vec![py_str("canal")])); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("root canal"))) } }) {
    let _ = mcall(&named.clone(), "append", &[g__A_ROOT_CANAL()]);
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("hygienist"), py_str("hygienist's"), py_str("clean"), py_str("cleaning")]))) {
    let _ = mcall(&named.clone(), "append", &[g__A_HYGIENIST()]);
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("crown"), py_str("crowns")]))) {
    let _ = mcall(&named.clone(), "append", &[g__A_CROWN_VISIT()]);
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("implant"), py_str("implants")]))) {
    let _ = mcall(&named.clone(), "append", &[g__AN_IMPLANT()]);
    }
    parts = op_add(&named.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__THE_VISIT_IN_ORDER()) { let mut part: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&named.clone(), &part.clone()))) {  __out.push(part.clone()); } } py_list(__out) })() });
    return mcall(&py_str(" "), "join", &[op_add(&parts.clone(), &py_list(vec![g__THE_BOOKING()]))]);
    Py::None
}
pub fn _the_dentist_errand(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut a_crown: Py = Py::None;
    let mut a_visit: Py = Py::None;
    let mut done_to_a_tooth: Py = Py::None;
    let mut how_long: Py = Py::None;
    let mut lasts: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut registering: Py = Py::None;
    let mut said: Py = Py::None;
    let mut takes: Py = Py::None;
    let mut the_place: Py = Py::None;
    let mut the_wait: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The length of a dental visit, or the wait for one, when that is what was asked.\n\n    The card is the one that wins \"how long ... dentist\", and this line has three different\n    questions in it. \"How long does a dentist appointment take\" is the visit, and it was answered\n    \"look for practices taking on new patients\" - a hunt for a practice, because \"taking\" is a\n    word the registering answer is written with. \"How long is the wait for a dentist\", \"how long\n    does it take to get a dentist appointment\", \"how long until i can see a dentist\" and \"how\n    soon can i get a dentist appointment\" and \"how long do dentists take to see you\" are the wait,\n    and a wait is what `_book_the_appointment`\n    already knows how to answer in the words of the place named (it answers the GP half of this\n    exactly). \"How do i register with a dentist\" is neither, and keeps the registering answer.\n    ");
    words = call_builtin("set", &[_line_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    occasion = op_bitand(&words.clone(), &py_set(vec![py_str("appointment"), py_str("appointments"), py_str("checkup"), py_str("check-up"), py_str("check"), py_str("visit"), py_str("hygienist"), py_str("hygienist's")]));
    done_to_a_tooth = { let __b = op_bitand(&words.clone(), &py_set(vec![py_str("filling"), py_str("fillings"), py_str("crown"), py_str("crowns"), py_str("canal")])); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("root canal"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("check up"))) } } } };
    the_place = op_bitand(&words.clone(), &py_set(vec![py_str("dentist"), py_str("dentists"), py_str("dental"), py_str("orthodontist"), py_str("tooth"), py_str("teeth"), py_str("practice"), py_str("surgery")]));
    how_long = stdlib_call("re", "search", &[py_str("\\bhow (?:long|many (?:minutes|hours))\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    takes = op_bitand(&words.clone(), &py_set(vec![py_str("take"), py_str("takes"), py_str("taking")]));
    lasts = op_bitand(&words.clone(), &py_set(vec![py_str("last"), py_str("lasts")]));
    the_wait = call_builtin("bool", &[{ let __b = op_bitand(&words.clone(), &op_bitor(&g__THE_QUEUE(), &py_set(vec![py_str("see"), py_str("seen")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("to get"))) } }]).unwrap_or_else(|e| exc::raise(e));
    registering = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("register"), py_str("registering"), py_str("sign up"), py_str("join")]))]).unwrap_or_else(|e| exc::raise(e));
    a_crown = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("crown"), py_str("crowns")]))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = a_crown.clone(); if !py_truthy(&__b) { __b } else { { let __b = lasts.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&takes.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&the_wait.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&registering.clone())) } } } } } } } }) {
    return py_str("A crown lasts ten to fifteen years on average, and some go much longer. What wears it out is usually the tooth and the gum under it rather than the crown itself, so keep cleaning round the edge where it meets the gum, and tell the dentist if it starts to feel loose or the gum bleeds when you floss.");
    }
    a_visit = { let __b = { let __b = occasion.clone(); if !py_truthy(&__b) { __b } else { { let __b = how_long.clone(); if py_truthy(&__b) { __b } else { { let __b = takes.clone(); if py_truthy(&__b) { __b } else { lasts.clone() } } } } } }; if py_truthy(&__b) { __b } else { { let __b = { let __b = done_to_a_tooth.clone(); if !py_truthy(&__b) { __b } else { { let __b = takes.clone(); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("how many (?:minutes|hours)"), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }; if py_truthy(&__b) { __b } else { { let __b = the_place.clone(); if !py_truthy(&__b) { __b } else { { let __b = how_long.clone(); if !py_truthy(&__b) { __b } else { { let __b = takes.clone(); if py_truthy(&__b) { __b } else { lasts.clone() } } } } } } } } } };
    if py_truthy(&{ let __b = a_visit.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&the_wait.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&registering.clone())) } } } }) {
    return _the_lengths_of_the_visit(&[said.clone(), words.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_LIST_THAT_IS_LONG(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&how_long.clone())) } }) {
    return g__REGISTERING_A_DENTIST();
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &op_bitor(&g__THE_QUEUE(), &py_set(vec![py_str("see"), py_str("seen")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("to get"))) } }) {
    return _book_the_appointment(&[memory.clone(), understood.clone()]);
    }
    return g__REGISTERING_A_DENTIST();
    Py::None
}
pub fn _what_can_i_take_pregnant_or_feeding(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut feeding: Py = Py::None;
    let mut pregnant: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The same answer with the half the line asked about first.\n\n    \"what can i take while breastfeeding\" opened \"In pregnancy, paracetamol...\" and never said\n    what to take instead of a feed, so the first sentence answered a question nobody asked.\n    ");
    words = call_builtin("set", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    feeding = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("breastfeeding"), py_str("breast"), py_str("feeding"), py_str("nursing"), py_str("chestfeeding")]))]).unwrap_or_else(|e| exc::raise(e));
    pregnant = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("trimester")]))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = feeding.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pregnant.clone())) } }) {
    return py_str("Paracetamol and ibuprofen are both fine while you are breastfeeding; avoid aspirin and go easy on codeine. Heartburn takes an antacid like Gaviscon, own-brand is the same. A pharmacist will check anything else for free, and see your GP if it lasts days rather than hours.");
    }
    return py_str("In pregnancy, paracetamol at the lowest dose that works, for the shortest time, and no ibuprofen - especially after 20 weeks - or aspirin unless a doctor prescribed it. While breastfeeding, paracetamol and ibuprofen are both fine; avoid aspirin and go easy on codeine. Heartburn takes an antacid like Gaviscon, own-brand is the same. A pharmacist will check the rest, and ring your midwife if it lasts days or comes with bleeding or less movement.");
    Py::None
}
pub fn _bare_words(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a line as typed, for reading one of the last few turns back.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(w.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _book_the_appointment(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut before: Py = Py::None;
    let mut doctors: Py = Py::None;
    let mut named: Py = Py::None;
    let mut places: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The steps to being seen, in the words of the place the line names.\n\n    The card opened on \"8am for most GP surgeries\", and a question about the dentist kept that\n    first sentence: \"how do i book a dentist appointment\" and \"can i get an emergency dentist\n    appointment\" were told to ring a GP surgery at eight, and the dentist came later in the\n    paragraph, after advice that did not apply to a tooth. The steps are the same - ring when the\n    lines open, say what it is for, ask about cancellations - so the same card answers both, with\n    its opening, its fallback and its pain advice written for the place the person named.\n    ");
    words = call_builtin("set", &[_line_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    places = py_set(vec![py_str("dentist"), py_str("dentists"), py_str("dental"), py_str("orthodontist")]);
    doctors = py_set(vec![py_str("doctor"), py_str("doctors"), py_str("gp"), py_str("gps"), py_str("surgery"), py_str("practice"), py_str("clinic"), py_str("nurse")]);
    named = op_bitand(&words.clone(), &places.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &doctors.clone()))) } }) {
    before = py_list(vec![]);
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } } } else { py_list(vec![]) };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&turn.clone(), "get", &[py_str("user")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&before.clone(), "extend", &[_bare_words(&[geti(&turn.clone(), &py_str("user"))])]);
    }
    }
    before = call_builtin("set", &[before.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&before.clone(), &g_TOOTH_WORDS())) {
    named = { let __b = op_bitand(&before.clone(), &places.clone()); if py_truthy(&__b) { __b } else { py_set(vec![py_str("dentist")]) } };
    } else {
    if py_truthy(&{ let __b = op_bitand(&before.clone(), &places.clone()); if !py_truthy(&__b) { __b } else { op_bitand(&before.clone(), &g_SEEN_ABOUT_IT()) } }) {
    named = op_bitand(&before.clone(), &places.clone());
    }
    }
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("long"), py_str("soon"), py_str("quickly"), py_str("wait"), py_str("waits"), py_str("waiting"), py_str("weeks"), py_str("days")]))) {
    if py_truthy(&named.clone()) {
    return py_str("Ask what the wait is when you ring: a routine dental check-up is usually a few days to a couple of weeks, depending on the practice, and a tooth that hurts is seen the same day or the next because most practices keep a few slots back each morning. Ring at opening time for one of those, and 111 can name an urgent NHS dentist if yours has nothing for days.");
    }
    return py_str("Ask what the wait is when you ring: a routine GP appointment is usually a few days to a couple of weeks, and anything urgent is seen the same day because surgeries keep slots back each morning for it. Ring at the moment the lines open for one of those, and 111 can find an urgent dentist or a walk-in centre if your surgery has nothing.");
    }
    if py_truthy(&named.clone()) {
    return py_str("Ring the dental practice as soon as it opens and ask for an urgent slot: most keep a few back each morning, and saying it hurts, or that a tooth has broken or a filling has come out, is what gets you one of them. Ask what they have sooner if the wait is weeks, and ring 111 for the out-of-hours dental service if your practice is shut or cannot see you within a few days - they can also name an urgent NHS dentist. Until then paracetamol or ibuprofen at the dose on the pack, and a pharmacist can help you hold on.");
    }
    return py_str("Ring at the moment the lines open - 8am for most GP surgeries - and ask for the first appointment of the day: same-day slots go in the first half hour. Say in one sentence what it is for, because \"I need it looked at today\" books faster than \"I was wondering\", and ask to go on the cancellation list if the wait is weeks. Dentists are the same: say it hurts and ask what they have sooner, and 111 can find an urgent NHS dentist if yours cannot see you within a few days. If there is nothing at all, ask which day the next slots are released and whether the duty doctor can ring you back, and remember a pharmacist can treat a lot of everyday things the same day.");
    Py::None
}
pub fn _a_decision_is_not_a_booking(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let _ = py_str("False for \"should i see a doctor\", True for \"should i book an appointment\".");
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__DECISION_ASK(), "search", &[clean.clone()])))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_RETURN_TO()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g__THE_BOOKING_WORDS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_already(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut keep: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keep") { v.clone() }
    else { Py::Int(3) };
    let mut turn: Py = Py::None;
    let _ = py_str("True when the bot's last few turns already opened with this advice.");
    opening = mcall(&opening.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&keep.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), &opening.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _sea_water_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut _pet_in_the_talk: Py = Py::None;
    let mut _pet_salt_water: Py = Py::None;
    let mut line: Py = Py::None;
    _pet_in_the_talk = py_func("_pet_in_the_talk", |args: &[Py]| pyrt::xrefs::xcall("knowledge.pets", "_pet_in_the_talk", args));
    _pet_salt_water = py_func("_pet_salt_water", |args: &[Py]| pyrt::xrefs::xcall("knowledge.pets", "_pet_salt_water", args));
    if py_truthy(&py_call(&_pet_in_the_talk, &[memory.clone(), understood.clone()])) {
    if py_truthy(&_said_already(&[py_str("ring the vet now"), memory.clone()])) {
    return py_str("Same as before, and it still stands: ring the vet now and say how much salt water and when. Being sick, wobbly or drowsy in the next few hours is a same-day visit, not a wait and see, and keep the fresh water away until the vet has said otherwise.");
    }
    return py_call(&_pet_salt_water, &[memory.clone(), understood.clone()]);
    }
    line = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&mcall(&g_SEA_WATER_ALARM(), "search", &[line.clone()])) {
    return g_SEA_WATER_ALARM_REPLY();
    }
    return py_str("A mouthful or two of sea water does no harm: the salt in it just makes you thirsty, so rinse your mouth out and drink some fresh water. Boiling it is no help - the water steams off and the salt stays, so it gets saltier. A child or an adult who has swallowed a lot of it - or who is being sick, dizzy or confused - needs 111, and 999 if they are drowsy or hard to wake. Never make anyone sick to get it out.");
    Py::None
}
pub fn _owner_word(args: &[Py]) -> Py {
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
    let _ = py_str("The thing named by a word that can own a part: \"chair\", \"chair's\", \"chairs\".");
    word = mcall(&word.clone(), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("'s")])) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    return { let __b = Py::Bool(py_contains(&g_OBJECT_OWNERS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OBJECT_OWNERS(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } };
    Py::None
}
pub fn _a_thing_owns_the_part(args: &[Py]) -> Py {
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
    let mut between: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut index: Py = Py::None;
    let mut nxt: Py = Py::None;
    let mut preposition: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line's body word belongs to a thing, not to a person.");
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    words = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_THING_PARTS(), &mcall(&word.clone(), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])))) {
    continue;
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { _owner_word(&[geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1)))]) } }) {
    return Py::Bool(true);
    }
    nxt = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(4)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    nxt = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&nxt.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__DETERMINERS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = nxt.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_THING_PARTS(), &geti(&nxt.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { _owner_word(&[geti(&nxt.clone(), &Py::Int(0))]) } } } }) {
    return Py::Bool(true);
    }
    between = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(5)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_str("in"), py_str("inside")]));
    for __item in __iter3 {
    preposition = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&between.clone(), &preposition.clone()))) {
    cut = mcall(&between.clone(), "index", &[preposition.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&between.clone(), &Py::None, &cut.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_HURT_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&between.clone(), &op_add(&cut.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__DETERMINERS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__POSSESSIVES(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { _owner_word(&[geti(&rest.clone(), &Py::Int(0))]) } }) {
    return Py::Bool(true);
    }
    }
    break;
    }
    }
    after = slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(5)), &Py::None);
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&after.clone(), &Py::Int(0)), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))]), &py_str("of"))) } }) {
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__DETERMINERS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__POSSESSIVES(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__PERSON_OWNER(), &geti(&rest.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_THING_PARTS(), &geti(&rest.clone(), &Py::Int(0)))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(_owner_word(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_part_that_hurts(args: &[Py]) -> Py {
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
    let mut _reply: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The part of the body a line complains about, or None.\n\n    A complaint names its cause as well as its part: \"my back has been killing me since i moved\n    the sofa\" says the back and the sofa, and a follow-up (\"its a dull ache on the left side\")\n    is about the back. This is the part the region table would answer for, read from the line\n    alone, so the store can carry the part into a follow-up instead of the cause. The same\n    ownership rule as the entries applies: a chair's back and a door's back are the thing's,\n    not the person's, and a line with no word of hurting (\"i moved the sofa\") names no part.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_HURT_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&found.clone(), &py_str("killing"))) } }))) {
    return Py::None;
    }
    if py_truthy(&_a_thing_owns_the_part(&[py_dict(vec![(py_str("clean"), line.clone()), (py_str("words"), found.clone())])])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_REGIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    parts = __unpk2[1].clone();
    _reply = __unpk2[2].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(Py::Bool(py_contains(&found.clone(), &part.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return name.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _reading_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of a reading, however much of itself the caller passed.\n\n    Some of the calls that reach this table hand over the words and some hand over the line\n    (\"clean\" or \"text\" only), and a guard that reads nothing answers True for every line - which\n    put the back card in front of \"i keep coming back to the same problem\", a line with no body\n    part in it at all. So the words are taken from the line when the caller has not split it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    said = mcall(&reading.clone(), "get", &[py_str("words")]);
    if py_truthy(&said.clone()) {
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    }
    line = { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    return stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_back_is_a_back(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut leaned: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("False when the line's only \"back\" is the particle in \"come back\" or \"is back again\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    words = _reading_words(&[understood.clone()]);
    leaned = { let __b = mcall(&understood.clone(), "get", &[py_str("leans_on")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&call_builtin("isinstance", &[leaned.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_add(&words.clone(), &_reading_words(&[leaned.clone()]));
    words = __aug1;
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__ALSO_THE_BACK(), "intersection", &[words.clone()])) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("back")))) {
    continue;
    }
    if py_truthy(&{ let __b = slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(2)), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__BACK_IS_A_NOUN_AFTER(), &geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__BACK_THEIRS(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__BACK_VERB_BEFORE(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__BACK_DESCRIBED(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(2)), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__BACK_IS_A_RETURN(), &geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))))) } }) {
    continue;
    }
    if py_truthy(&_a_verb_earlier_took_it(&[words.clone(), index.clone()])) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_verb_earlier_took_it(args: &[Py]) -> Py {
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
    let mut seen: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a verb earlier in this clause has this \"back\" as its particle.\n\n    \"get some energy back\" and \"want my money back\" put the thing the verb acts on between the\n    verb and the word, so the neighbour test above cannot see the verb at all and the line kept\n    the back-pain card: \"what can i do to get some energy back\" was answered \"Most back pain is\n    muscular and settles in two to six weeks\", a paragraph about a body nobody mentioned.\n\n    What is between them has to be the thing, so a word that describes the part (\"sore\", \"lower\",\n    \"bad\") or a determiner on its own (\"my back\") keeps the back a back - that is the `return\n    True` the caller already made before this is asked.\n    ");
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&index.clone(), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    at = __item.clone();
    word = geti(&words.clone(), &at.clone());
    if py_truthy(&Py::Bool(py_contains(&g__BACK_CLAUSE_JOIN(), &word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g__BACK_TAKING_A_THING(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_gt(&seen.clone(), &Py::Int(4))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_back_is_a_back_that_is_someones(args: &[Py]) -> Py {
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
    return { let __b = _the_part_is_a_persons(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _the_back_is_a_back(&[understood.clone()]) } };
    Py::None
}
pub fn _the_part_is_not_a_standing_trouble(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("This line is about a fresh knock, not a part that has given trouble for years.\n\n    A part that \"gives trouble\", is \"bad\" or \"dodgy\", or is asked what would strengthen it, has\n    its own standing-condition row (`bad-<part>` below), which says which movement builds the part\n    up rather than how to rest it. Those rows key the same part word as this one, and the movement\n    asking phrases in `ASKING_ABOUT_A_PART` (\"what exercise\", \"exercises for\") make this row match\n    too, so without this the general resting answer won: \"what exercise can i do with a bad knee\"\n    and \"whats a good exercise for bad knees\" went from the strengthening card to \"ease off\n    running\". The parts that never name one of those words keep this row.\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g__REPORTED_COMPLAINT(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STANDING_TROUBLE()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_region_guard(args: &[Py]) -> Py {
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
    let mut _the_back_is_the_persons_and_not_a_standing_trouble: Py = Py::None;
    let mut _the_part_is_the_persons_and_not_a_standing_trouble: Py = Py::None;
    let _ = py_str("The `guard` for one part's region row, built here rather than in the loop.\n\n    A lambda written inside the entry loop would close over the loop's `name` and read the last\n    part at call time, so every row would be guarded as if it were that one.\n    ");
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("back")))) {
pub fn __nested__the_back_is_the_persons_and_not_a_standing_trouble_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _the_back_is_the_persons_and_not_a_standing_trouble: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut _the_part_is_the_persons_and_not_a_standing_trouble: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return { let __b = _the_back_is_a_back_that_is_someones(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _the_part_is_not_a_standing_trouble(&[understood.clone()]) } };
    Py::None
}
    _the_back_is_the_persons_and_not_a_standing_trouble = {{ let _the_back_is_the_persons_and_not_a_standing_trouble = _the_back_is_the_persons_and_not_a_standing_trouble.clone(); let _the_part_is_the_persons_and_not_a_standing_trouble = _the_part_is_the_persons_and_not_a_standing_trouble.clone(); let name = name.clone(); py_func("_the_back_is_the_persons_and_not_a_standing_trouble", move |args: &[Py]| { let mut full = vec![_the_back_is_the_persons_and_not_a_standing_trouble.clone(), _the_part_is_the_persons_and_not_a_standing_trouble.clone(), name.clone()]; full.extend_from_slice(args); __nested__the_back_is_the_persons_and_not_a_standing_trouble_0(&full) }) }};
    return _the_back_is_the_persons_and_not_a_standing_trouble.clone();
    }
pub fn __nested__the_part_is_the_persons_and_not_a_standing_trouble_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _the_back_is_the_persons_and_not_a_standing_trouble: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut _the_part_is_the_persons_and_not_a_standing_trouble: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return { let __b = _the_part_is_a_persons(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _the_part_is_not_a_standing_trouble(&[understood.clone()]) } };
    Py::None
}
    _the_part_is_the_persons_and_not_a_standing_trouble = {{ let _the_back_is_the_persons_and_not_a_standing_trouble = _the_back_is_the_persons_and_not_a_standing_trouble.clone(); let _the_part_is_the_persons_and_not_a_standing_trouble = _the_part_is_the_persons_and_not_a_standing_trouble.clone(); let name = name.clone(); py_func("_the_part_is_the_persons_and_not_a_standing_trouble", move |args: &[Py]| { let mut full = vec![_the_back_is_the_persons_and_not_a_standing_trouble.clone(), _the_part_is_the_persons_and_not_a_standing_trouble.clone(), name.clone()]; full.extend_from_slice(args); __nested__the_part_is_the_persons_and_not_a_standing_trouble_1(&full) }) }};
    return _the_part_is_the_persons_and_not_a_standing_trouble.clone();
    Py::None
}
pub fn _a_part_keeps_me_from_something(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line says a part is stopping the person doing something.");
    line = mcall(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    return call_builtin("bool", &[{ let __b = mcall(&g__KEPT_AWAKE(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_PART_IN_THE_TABLE(), "search", &[line.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_breaks_it_says(args: &[Py]) -> Py {
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
    let _ = py_str("The break words for a row whose part the knock card has no test for; else nothing.");
    return if py_truthy(&Py::Bool(py_contains(&g_A_PART_NO_KNOCK_TEST(), &name.clone()))) { g_BREAK_SAID() } else { py_tuple(vec![]) };
    Py::None
}
pub fn _it_is_somebody_elses_part(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut people: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the part that is hurt belongs to another person, so the line is theirs to answer.\n\n    The sore rows above are written in the second person and step aside for a relative, which is\n    right: \"keep walking and carry on with your usual jobs\" is not advice a parent can pass on.\n    This row is the answer for those lines, and only those: \"i hurt my arm\", \"my legs hurt\" and\n    \"my wrist hurts after a fall\" are still the person's own and keep the rows that answer them.\n    \"My son has a bad back\" names no hurt and no fall, so it never reaches this row at all.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_part_is_a_persons(&[understood.clone()]))) } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    people = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("'s")])) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None) } else { word.clone() }); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&people.clone(), &g__SOMEONE_ELSE())))) {
    return Py::Bool(false);
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g__A_WHILE_AGO(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk2734 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk2734[0].clone(); let mut word: Py = __unpk2734[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_SOFT_TISSUE(), &word.clone()))) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__IT_HAPPENED_TO(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("went over"))) } }) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5038 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk5038[0].clone(); let mut word: Py = __unpk5038[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g__TO_HURT_A_PART(), &word.clone())); if !py_truthy(&__b) { __b } else { op_lt(&at.clone(), &geti(&parts.clone(), &Py::Int(0))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _line_words(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("Only the words of this line, not the turn it leans on.\n\n    `_words` adds the background, and in a conversation about a sore tailbone the sentence \"30\n    weeks pregnant and my back hurts\" was answered about the tailbone: the part has to be the one\n    the person has just named.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    return call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut w: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[w.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&w.clone(), "lower", &[])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _how_long(args: &[Py]) -> Py {
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
    let _ = py_str("How long the line says it has been going on, in the person's own scale.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\byear"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("for months or years");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmonth"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("for months");
    }
    return py_str("for a week or more");
    Py::None
}
pub fn _back_is_a_direction(args: &[Py]) -> Py {
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
    let mut seen: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when every back the line says is a particle or a returning, not a part of the body.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    said = call_builtin("len", &[stdlib_call("re", "findall", &[py_str("\\bback\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    seen = op_add(&op_add(&call_builtin("len", &[mcall(&g__BACK_AFTER_A_VERB(), "findall", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&g__BACK_BEFORE_A_PLACE(), "findall", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("len", &[mcall(&g__BACK_HOME(), "findall", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_ge(&seen.clone(), &said.clone()) } };
    Py::None
}
pub fn _a_back_in_the_chat(args: &[Py]) -> Py {
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
    let _ = py_str("True when a line already in this conversation named a back of its own.\n\n    \"i hurt my back lifting boxes\" then \"the pain came back after a week\" is a question about\n    that back, so the direction in the second line does not take the answer away: the chat has\n    the part, and the entry is the one that speaks for it.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bback\\b"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_back_is_a_direction(&[py_dict(vec![(py_str("text"), said.clone())])]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _sore_weeks_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut article: Py = Py::None;
    let mut drill: Py = Py::None;
    let mut key: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    parts = py_list(vec![said.clone()]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("plain")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&parts.clone(), "append", &[mcall(&value.clone(), "lower", &[])]);
    }
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("words"), py_str("background")]));
    for __item in __iter2 {
    key = __item.clone();
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&parts.clone(), "append", &[mcall(&word.clone(), "lower", &[])]);
    }
    }
    }
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    let _ = mcall(&parts.clone(), "append", &[mcall(&geti(&geti(&turns.clone(), &op_neg(&Py::Int(1))), &py_str("user")), "lower", &[])]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PREGNANT_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[parts.clone()]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    part = { let __b = _part_named(&[_line_words(&[understood.clone()]), understood.clone()]); if py_truthy(&__b) { __b } else { { let __b = _part_named(&[_words(&[understood.clone()]), understood.clone()]); if py_truthy(&__b) { __b } else { py_str("part") } } } };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&part.clone(), &py_str("back"))); if !py_truthy(&__b) { __b } else { { let __b = _back_is_a_direction(&[understood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_back_in_the_chat(&[memory.clone()]))) } } } }) {
    return Py::None;
    }
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("An") } else { py_str("A") };
    drill = mcall(&g__WEEKS_DRILL(), "get", &[part.clone(), py_str("two short sessions a week of the movement you want to return to, stopping short of pain")]);
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&article.clone(), &py_str(" ")), &part.clone()), &py_str(" that has been sore ")), &_how_long(&[understood.clone()])), &py_str(" is usually a tendon or a joint asked for more than it is used to: keep going as far as is comfortable, and add ")), &drill.clone()), &py_str(". If it is no better after six weeks, wakes you up, or keeps coming back, ask your GP about physio. Ring 111 sooner for numbness, a part that gives way, or pain with a fever or weight loss."));
    Py::None
}
pub fn _the_day_after_new_exercise(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let _ = py_str("True when the line is the ache a day or two after new work, not an ache on its own.");
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&_line_words(&[understood.clone()]), &g__SORENESS_WORDS())))) {
    return Py::Bool(false);
    }
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_EXERCISE_AFTER()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _see_doctor_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut article: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("The decision rule, with the part named when the line names one.\n\n    The window is the one every muscle-and-joint card in this file already gives, \"two to six\n    weeks\", because one back cannot settle in two to three weeks here and in two to six weeks in\n    `back-went`: the same person asking twice got two numbers. And the rule comes first, in the\n    line's own words, instead of a preamble about what the question is - the answer is the\n    decision, with the red flags and the next step behind it.\n\n    The ache a day or two after new work is the one shape it steps out of, so that\n    `sport.sore-after-exercise` can say the ache is normal and not damage.\n    ");
    if py_truthy(&_the_day_after_new_exercise(&[understood.clone()])) {
    return Py::None;
    }
    part = _part_named(&[_line_words(&[understood.clone()]), understood.clone()]);
    opening = py_str("Go by how long it has been going: ");
    if py_truthy(&part.clone()) {
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("An") } else { py_str("A") };
    opening = op_add(&op_add(&op_add(&article.clone(), &py_str(" ")), &part.clone()), &py_str(" like that: "));
    }
    return op_add(&opening.clone(), &py_str("give it two to six weeks if it is steady or easing, and book sooner if it is getting worse, waking you at night or stopping you doing normal things. Go today if it came on suddenly or after a fall, or comes with a fever, numbness or pins and needles. Ask about physio when you ring; in most areas you can refer yourself."));
    Py::None
}
pub fn _a_part_in_a_decision_is_a_part(args: &[Py]) -> Py {
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
    words = call_builtin("set", &[_reading_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&words.clone(), &py_str("back"))); if py_truthy(&__b) { __b } else { op_bitand(&op_sub(&words.clone(), &py_set(vec![py_str("back")])), &call_builtin("set", &[g_SEE_DOCTOR_PARTS()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return Py::Bool(true);
    }
    return _the_back_is_a_back(&[understood.clone()]);
    Py::None
}
pub fn _a_bare_decision(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the decision and names nothing else.\n\n    \"do i need to see someone\" is this row's line. \"i think i have a uti, do i need to see\n    someone\" is the uti card's line, and \"my voice has been hoarse for a week, do i need to see\n    someone\" is the hoarse-voice card's: the rule's two-to-six-weeks window was written for a\n    muscle and a joint, and it says nothing useful about either of those.\n    ");
    words = _line_words(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__DECISION_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _see_doctor_decision(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("The same rule for a line that names no part.\n\n    \"should i get a rash checked out\" says no limb, so the physio line at the end of the part\n    answer - \"ask about physio when you ring\" - is advice for a muscle it does not have. The rule\n    and its red flags are the same, so this is the part answer with that one sentence left out\n    whenever no part is named in the line.\n    ");
    reply = _see_doctor_reply(&[memory.clone(), understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&reply.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _part_named(&[_line_words(&[understood.clone()]), understood.clone()]) } }) {
    return reply.clone();
    }
    return mcall(&reply.clone(), "replace", &[py_str(" Ask about physio when you ring; in most areas you can refer yourself."), py_str("")]);
    Py::None
}
pub fn _the_back_is_the_persons(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line's \"back\" is the verb's particle and nothing else.\n\n    Used as an entry `guard` on the sore-back-for-a-week row, which is the one row whose key is\n    \"back\" plus a duration - the shape a text message and a sore back share.\n    ");
    clean = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_VERB_TOOK_THE_BACK(), "search", &[clean.clone()])))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__THE_BACK_IS_THEIRS(), "search", &[mcall(&g__A_VERB_TOOK_THE_BACK(), "sub", &[py_str("  "), clean.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_part_is_named_today(args: &[Py]) -> Py {
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
    let mut typed: Py = Py::None;
    let _ = py_str("This line names the part itself, and it is the person's own part they can still use.\n\n    A later turn about a part is read with the earlier line put back in it, so the ankle of \"my\n    ankle hurts after football\" is in the room when \"is that bad\" is asked - and the two words\n    that fit this row's key are \"bad\" and the part, which is exactly that follow-up. The drill is\n    advice for a part somebody has had trouble with for years, and a sprain from Saturday is not\n    that, so the part has to be in the line as it was typed. A reading with no text of its own is\n    one the store made by putting an earlier subject back in, and it never speaks for this row.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&the_part_that_keeps_grumbling(&[understood.clone()])))) {
    return Py::Bool(false);
    }
    said = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { mcall(&g__REPORTED_COMPLAINT(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    typed = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&said.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[_part_named(&[typed.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bad_part_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut article: Py = Py::None;
    let mut drill: Py = Py::None;
    let mut instead: Py = Py::None;
    let mut part: Py = Py::None;
    part = { let __b = _part_named(&[_line_words(&[understood.clone()])]); if py_truthy(&__b) { __b } else { { let __b = _part_named(&[_words(&[understood.clone()])]); if py_truthy(&__b) { __b } else { py_str("joint") } } } };
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("An") } else { py_str("A") };
    drill = mcall(&g__WEEKS_DRILL(), "get", &[part.clone(), py_str("two short sessions a week of the movement you want to keep, stopping short of pain")]);
    instead = if py_truthy(&Py::Bool(py_contains(&g__CARRIES_WEIGHT(), &part.clone()))) { py_str("Swimming, cycling and a flat walk are the kind ones on the other days.") } else { py_str("Keep the rest of you moving, and use it as far as is comfortable.") };
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&article.clone(), &py_str(" ")), &part.clone()), &py_str(" that gives trouble on and off is usually weak muscle round a joint that does not like one particular movement, and building it up is what keeps it quiet: ")), &drill.clone()), &py_str(". ")), &instead.clone()), &py_str(" Get it looked at if it gives way, locks, or swells up after a knock."));
    Py::None
}
pub fn _the_ask_names_nothing_else(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is only the wish and the part, with no desk, sport or shoe in it.");
    said = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&said.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(_in_the_allow_list(&[w.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _in_the_allow_list(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let _ = py_str("True when the word is the allow list's own, or that word with a plural or -ing on it.");
    if py_truthy(&Py::Bool(py_contains(&g__PROTECT_ALLOWED(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("es"), py_str("s"), py_str("ed"), py_str("d")]));
    for __item in __iter1 {
    tail = __item.clone();
    stem = if py_truthy(&mcall(&word.clone(), "endswith", &[tail.clone()])) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None) } else { py_str("") };
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__PROTECT_ALLOWED(), &stem.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _protect_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut article: Py = Py::None;
    let mut drill: Py = Py::None;
    let mut instead: Py = Py::None;
    let mut part: Py = Py::None;
    part = { let __b = _part_named(&[_line_words(&[understood.clone()]), understood.clone()]); if py_truthy(&__b) { __b } else { { let __b = _part_named(&[_words(&[understood.clone()]), understood.clone()]); if py_truthy(&__b) { __b } else { py_str("joint") } } } };
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("An") } else { py_str("A") };
    drill = mcall(&g__WEEKS_DRILL(), "get", &[part.clone(), py_str("two short sessions a week of the movement you want to keep, stopping short of pain")]);
    if py_truthy(&Py::Bool(py_contains(&g__CARRIES_WEIGHT(), &part.clone()))) {
    instead = py_str("Swimming, cycling and a flat walk are the kind ones on the other days. Warm up before a big job, keep the load close to you when you lift, and change position every half hour rather than holding one.");
    } else {
    instead = py_str("Keep it moving every day rather than resting it, and keep it in the middle of where it moves - not locked straight, not at the end of its range.");
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&article.clone(), &py_str(" ")), &part.clone()), &py_str(" is protected by the muscle round it and by how you use it more than by anything you wear or avoid: ")), &drill.clone()), &py_str(". ")), &instead.clone());
    Py::None
}
pub fn _take_a_temperature(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("without")))) {
    return py_str("Without a thermometer there is no number to take, so go by how they seem: a hot back or neck, flushed cheeks, shivering, and a hand on the skin is a poor guide either way. Don't pile layers on or take them off to chase a guess, and get anyone seen the same day if a baby under three months feels hot, or they are floppy, breathing fast or not drinking.");
    }
    return g__TAKE_A_TEMPERATURE();
    Py::None
}
pub fn _fever_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("how long"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("last"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("how many days"))) } } } }) {
    return py_str("Most fevers from a virus settle within two or three days, and the last day is usually lower than the first. Have it looked at if it has been there five days or is still climbing, or if it goes away and comes back. Treat how they seem rather than the number, and get seen the same day for a baby under three months with 38C, a rash that will not fade under a glass, a stiff neck, breathing trouble or a fit.");
    }
    return g__FEVER_ADVICE();
    Py::None
}
pub fn _chickenpox_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("ibuprofen"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("nurofen"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("brufen"))) } } } }) {
    return py_str("No - avoid ibuprofen with chickenpox, because it is linked to a serious skin infection in children who have it. Use paracetamol at the dose on the pack, calamine or a cool bath for the itch, and keep the nails short so the spots do not scar. They are infectious from two days before the rash until every spot has crusted over, usually five days, which is when they can go back to school.");
    }
    return op_add(&g__CHICKENPOX_TIMELINE(), &py_str(" Calamine or a cool bath eases the itch, paracetamol helps the temperature and short nails stop the spots scarring; avoid ibuprofen. Ring the GP if a newborn, a pregnant woman or anyone with a weak immune system has been in contact."));
    Py::None
}
pub fn _arthritis_reply(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut words: Py = Py::None;
    words = call_builtin("set", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g__ARTHRITIS_CURE()]).unwrap_or_else(|e| exc::raise(e)))) {
    return op_add(&py_str("Arthritis cannot be cured, but the ordinary wear-and-tear kind is managed well: "), &g__ARTHRITIS_TAIL());
    }
    return op_add(&py_str("Ordinary arthritis - osteoarthritis - is the wear-and-tear kind: "), &g__ARTHRITIS_TAIL());
    Py::None
}
pub fn _broken_or_sprained(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut words: Py = Py::None;
    let _ = py_str("The same check, said in the test the part can actually take.");
    if py_truthy(&_a_thing_owns_the_part(&[{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])) {
    return Py::None;
    }
    words = _words(&[understood.clone()]);
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g_IN_YOUR_HANDS()]).unwrap_or_else(|e| exc::raise(e)))) {
    return g_IN_YOUR_HANDS_REPLY();
    }
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g_AT_YOUR_SHOULDER()]).unwrap_or_else(|e| exc::raise(e)))) {
    return g_AT_YOUR_SHOULDER_REPLY();
    }
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g_ON_YOUR_FEET()]).unwrap_or_else(|e| exc::raise(e)))) {
    return g_FOUR_STEPS_REPLY();
    }
    return g_FOUR_STEPS_REPLY();
    Py::None
}
pub fn _a_bare_followup(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for a short line about the thing just discussed rather than a new mole.\n\n    It has to name nothing of its own: no mole noun (\"are moles normal\" is its own question) and\n    no other person or new mole (\"my child has a new mole\").\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("mole"), py_str("moles"), py_str("birthmark"), py_str("birthmarks"), py_str("spot"), py_str("spots")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NEW_SUBJECT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _mole_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The two-week card for a mole that is doing something, the same facts for the other one.");
    words = _words(&[understood.clone()]);
    said_words = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MOLE_REMOVE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&said_words.clone(), "lower", &[]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g_MOLE_HAPPENING()]).unwrap_or_else(|e| exc::raise(e))))) } }) {
    return g__MOLE_REMOVE_TEXT();
    }
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &py_set(vec![py_str("hair"), py_str("hairy")])); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g__MOLE_CLEAR()))) } }) {
    return g__MOLE_ORDINARY_TEXT();
    }
    if py_truthy(&mcall(&g__CHECK_HOW_TO(), "search", &[said.clone()])) {
    return g__MOLE_ORDINARY_TEXT();
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &call_builtin("set", &[g_MOLE_HAPPENING()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &py_set(vec![py_str("check"), py_str("checked"), py_str("checking")])) } }) {
    return g__MOLE_CHANGED_TEXT();
    }
    if py_truthy(&{ let __b = _a_bare_followup(&[said.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str("get it checked by a gp")))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return g__MOLE_CHANGED_TEXT();
    }
    return g__MOLE_ORDINARY_TEXT();
    Py::None
}
pub fn _the_organ_the_painkiller_upsets(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut holds_nsaid: Py = Py::None;
    let mut holds_paracetamol: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    holds_paracetamol = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_AN_NSAID()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    holds_nsaid = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("ibuprofen"), py_str("nurofen"), py_str("brufen"), py_str("naproxen"), py_str("aspirin"), py_str("diclofenac"), py_str("nsaid"), py_str("anti inflammatory"), py_str("anti-inflammatory"), py_str("voltarol"), py_str("indomethacin")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = holds_paracetamol.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&holds_nsaid.clone())) } }) {
    return py_str("Paracetamol is the painkiller to use here: it is not an anti-inflammatory, so it does not cut the blood flow the kidneys rely on or irritate the stomach the way ibuprofen does, and a blood pressure tablet or a water tablet is no reason to avoid it. Stay within the 4g a day limit (less if a GP has said so, and less if you have liver trouble or drink heavily), and space the doses out. If it is not touching the pain, that is worth a word with a pharmacist rather than more tablets.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__KIDNEY_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("An anti-inflammatory cuts the blood flow the kidneys rely on, so it is the wrong choice if yours are already poor, if you are dehydrated, or if you take a water tablet or a blood pressure tablet - that mix is the one pharmacists watch for. Paracetamol is the painkiller to use instead, and it is worth asking a pharmacist or GP before starting ibuprofen at all if you have kidney disease or only one kidney.");
    }
    return py_str("Ibuprofen and the other anti-inflammatories (naproxen, aspirin, diclofenac) can irritate the stomach lining, so take them with or after food rather than on an empty stomach, at the lowest dose that works and for the shortest time. Don't take them at all if you have had a stomach ulcer or a bleed, or if you are on a blood thinner - paracetamol is the one to use instead. Get seen the same day for a pain in the top of the stomach that is severe, for black tarry poo, or for vomiting blood or anything that looks like coffee grounds.");
    Py::None
}
pub fn _hair_removal(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("Open with the half the line asked about.\n\n    STYLE wants the answer in the first sentence, and \"how do i wax my legs\" was opening with\n    the shave: the waxing words were there, in the middle. A line that names wax starts with the\n    wax, and a line that only names the blade or the bumps keeps the shave first.\n    ");
    if py_truthy(&op_bitand(&_words(&[understood.clone()]), &g__WAX_WORDS())) {
    return g__WAX_FIRST_REPLY();
    }
    return g__SHAVE_FIRST_REPLY();
    Py::None
}
pub fn _face_shaving(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut words: Py = Py::None;
    let _ = py_str("The beard, the blade, the burn or the shave, whichever the line is about.");
    words = _words(&[understood.clone()]);
    if py_truthy(&op_bitand(&words.clone(), &g__BLADE_WORDS())) {
    return g__RAZOR_BLADE_REPLY();
    }
    if py_truthy(&op_bitand(&words.clone(), &g__SHAVE_BURN_WORDS())) {
    return g__RAZOR_BURN_REPLY();
    }
    if py_truthy(&op_bitand(&words.clone(), &g__BEARD_TRIM_WORDS())) {
    return g__BEARD_TRIM_REPLY();
    }
    return g__STRAIGHT_SHAVE_REPLY();
    Py::None
}
pub fn _not_a_break_from_work(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line's \"break\" is a rest, so the healing row keeps its bones.\n\n    \"how long is my break\" was answered \"About six weeks for most bones - six for a wrist or\n    collarbone\", and so were \"how long should a break be\" and \"how long are my breaks at work\".\n    A line that names a part - a wrist, a leg, a bone, a fracture - is a bone question whatever\n    else it says, and a line that says whose break it is, with no part in it, is a rest.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g__A_JOINT_OF_MEAT(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g__A_PART_NAMED()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_HEALING_WORD(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g__A_REST_NOT_A_BONE(), "search", &[said.clone()])));
    Py::None
}
pub fn _healing(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut slow: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("One answer, with the part's own number when the line names one.");
    words = call_builtin("set", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    slow = op_bitand(&words.clone(), &py_set(vec![py_str("leg"), py_str("legs"), py_str("shin"), py_str("tibia"), py_str("fibula"), py_str("femur"), py_str("ankle"), py_str("ankles"), py_str("knee"), py_str("hip"), py_str("foot"), py_str("feet")]));
    if py_truthy(&slow.clone()) {
    return py_str("Bones in the leg take the longest: about 12 weeks for a shin or ankle, and a thigh bone can be four months or more. The cast usually comes off before the bone is fully strong - it is the X-ray at each clinic visit that decides - and the muscles and joint need a few weeks after that to feel normal, so build back slowly rather than testing it early.");
    }
    return py_str("About six weeks for most bones - six for a wrist or collarbone, six to eight for an arm, and longer for a leg or anything that had to be pinned, up to 12 weeks or more. Children heal faster, often in half the time. The cast comes off before the bone is fully strong and the joint takes a few more weeks to feel normal, and smoking slows bone healing a lot, so it is worth stopping while it knits.");
    Py::None
}
pub fn _a_place_you_would_ice(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about a body, an ache, or something you put on one.\n\n    A `guard` and not an `ask`: `score` lets a line that wants something past the ask list, and\n    \"heat\" is the boiler's word too - so the words that make it a body question have to be read\n    out of the whole line rather than listed as an ask.\n\n    The compress words are not enough on their own: \"how long do frozen peas take to cook\" and\n    \"how long before ice freezes\" both name a compress and are neither of them a body question,\n    and this row was answering both with the ice-or-heat card while the kitchen's own row for\n    frozen peas sat one match away.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    line = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_ON_A_BODY()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g__HEAT_OR_ICE_THE_THING(), "findall", &[line.clone()]));
    for __item in __iter1 {
    thing = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_THING_THAT_IS_NOT_A_BODY(), &thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ACHE_WORDS(), &thing.clone())) } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ON_A_BODY()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ACHE_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _muscle_healing(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut slow: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The muscle's own numbers, with the slower end named when the line says which muscle.\n\n    A groin and a hamstring do not heal on the same timetable, so the muscle the line or the\n    thread names picks the number: \"how long before i can play again\", said after a groin pull,\n    is asking about the groin.\n    ");
    words = op_bitor(&call_builtin("set", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("background")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    slow = op_bitand(&words.clone(), &py_set(vec![py_str("hamstring"), py_str("calf"), py_str("calves"), py_str("tear"), py_str("tore"), py_str("torn"), py_str("badly"), py_str("completely"), py_str("snapped")]));
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("groin"), py_str("adductor")]))) {
    return py_str("A groin strain is usually two to six weeks, and the milder ones are back in a fortnight: the first week is ice and rest from the twisting and kicking, then gentle stretching, and back to football or five-a-side only when you can sprint and change direction without pain. A sudden sharp pain, or a lump that appears in the groin when you cough, wants a physio rather than time.");
    }
    if py_truthy(&slow.clone()) {
    return py_str("A torn hamstring or calf is four to eight weeks, and the last third of that is the part people rush: run again only when you can move it through its full range without pain and it feels as strong as the other side. A sudden snap, a lot of bruising, or a dent in the muscle wants a physio this week rather than rest alone.");
    }
    return py_str("A pulled thigh muscle is usually two to three weeks: ice it for 15 minutes every few hours for the first two days, then keep it gently moving, because gentle movement heals faster than total rest. Go back to sport when it moves fully without pain and feels as strong as the other side, and see a physio if it is no better after a fortnight.");
    Py::None
}
pub fn _a_tampon_worn_too_long(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about wearing a tampon, and not about getting rid of one.\n\n    \"can i use a tampon while pregnant\" is answered here too - the reply ends with pads rather\n    than tampons during pregnancy - so pregnancy does not stand the row down. What does stand it\n    down is a line about the bin, the loo or the wrapper: the rubbish row owns those.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[py_list(vec![call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])])]);
    return { let __b = call_builtin("bool", &[mcall(&g__TAMPON_WORD(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__NOT_A_TAMPON_LINE(), "search", &[said.clone()]))) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.body".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACHE_WORDS" => Some(g_ACHE_WORDS()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ASKED_AGAIN" => Some(g_ASKED_AGAIN()),
            "ASKING_ABOUT_A_PART" => Some(g_ASKING_ABOUT_A_PART()),
            "ASK_ABOUT" => Some(g_ASK_ABOUT()),
            "AT_YOUR_SHOULDER" => Some(g_AT_YOUR_SHOULDER()),
            "AT_YOUR_SHOULDER_REPLY" => Some(g_AT_YOUR_SHOULDER_REPLY()),
            "AVOID" => Some(g_AVOID()),
            "A_PART_NO_KNOCK_TEST" => Some(g_A_PART_NO_KNOCK_TEST()),
            "BACK_NAMES" => Some(g_BACK_NAMES()),
            "BACK_PREGNANCY_KEEP" => Some(g_BACK_PREGNANCY_KEEP()),
            "BACK_WENT" => Some(g_BACK_WENT()),
            "BREAK_SAID" => Some(g_BREAK_SAID()),
            "BROKEN_PARTS" => Some(g_BROKEN_PARTS()),
            "BROKEN_WORDS" => Some(g_BROKEN_WORDS()),
            "CLOT_AND_CHEST" => Some(g_CLOT_AND_CHEST()),
            "COMPRESS_WORDS" => Some(g_COMPRESS_WORDS()),
            "DECIDE" => Some(g_DECIDE()),
            "DECIDE_ASK" => Some(g_DECIDE_ASK()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EXERCISE_AFTER" => Some(g_EXERCISE_AFTER()),
            "EXERCISE_DAY" => Some(g_EXERCISE_DAY()),
            "EXTRA_AVOID" => Some(g_EXTRA_AVOID()),
            "FALL" => Some(g_FALL()),
            "FLAG_WITH_IT" => Some(g_FLAG_WITH_IT()),
            "FOOT_PARTS" => Some(g_FOOT_PARTS()),
            "FOUR_STEPS_REPLY" => Some(g_FOUR_STEPS_REPLY()),
            "HAND_PARTS" => Some(g_HAND_PARTS()),
            "HOT_FLAGS" => Some(g_HOT_FLAGS()),
            "HOT_SWELLING" => Some(g_HOT_SWELLING()),
            "HURT" => Some(g_HURT()),
            "HURT_WORDS" => Some(g_HURT_WORDS()),
            "IN_YOUR_HANDS" => Some(g_IN_YOUR_HANDS()),
            "IN_YOUR_HANDS_REPLY" => Some(g_IN_YOUR_HANDS_REPLY()),
            "JOINTS" => Some(g_JOINTS()),
            "KNEE_WORDS" => Some(g_KNEE_WORDS()),
            "LOWER_HALF" => Some(g_LOWER_HALF()),
            "MEDICINE" => Some(g_MEDICINE()),
            "MOLE_CHANGE" => Some(g_MOLE_CHANGE()),
            "MOLE_HAPPENING" => Some(g_MOLE_HAPPENING()),
            "MOLE_REMOVE" => Some(g_MOLE_REMOVE()),
            "NUMB_ANYWHERE" => Some(g_NUMB_ANYWHERE()),
            "NUMB_ASK" => Some(g_NUMB_ASK()),
            "NUMB_AVOID" => Some(g_NUMB_AVOID()),
            "NUMB_FOOT" => Some(g_NUMB_FOOT()),
            "NUMB_HANDS" => Some(g_NUMB_HANDS()),
            "NUMB_LOWER" => Some(g_NUMB_LOWER()),
            "NUMB_SHOE" => Some(g_NUMB_SHOE()),
            "OBJECT_OWNERS" => Some(g_OBJECT_OWNERS()),
            "ON_YOUR_FEET" => Some(g_ON_YOUR_FEET()),
            "OTHER_PEOPLE" => Some(g_OTHER_PEOPLE()),
            "PAINKILLER_NAMES" => Some(g_PAINKILLER_NAMES()),
            "PART_ONE" => Some(g_PART_ONE()),
            "PERSON_NOUNS" => Some(g_PERSON_NOUNS()),
            "POSSESSIVE_OF_A_PERSON" => Some(g_POSSESSIVE_OF_A_PERSON()),
            "PREGNANT" => Some(g_PREGNANT()),
            "RED_FLAG" => Some(g_RED_FLAG()),
            "RED_FLAG_CARD" => Some(g_RED_FLAG_CARD()),
            "REGIONS" => Some(g_REGIONS()),
            "REGION_BIAS" => Some(g_REGION_BIAS()),
            "RING_AS_A_CALL" => Some(g_RING_AS_A_CALL()),
            "RING_NOT_ON_A_FINGER" => Some(g_RING_NOT_ON_A_FINGER()),
            "SADDLE_CARD" => Some(g_SADDLE_CARD()),
            "SADDLE_PARTS" => Some(g_SADDLE_PARTS()),
            "SEA_WATER_ALARM" => Some(g_SEA_WATER_ALARM()),
            "SEA_WATER_ALARM_REPLY" => Some(g_SEA_WATER_ALARM_REPLY()),
            "SEEN_ABOUT_IT" => Some(g_SEEN_ABOUT_IT()),
            "SEE_DOCTOR_AVOID" => Some(g_SEE_DOCTOR_AVOID()),
            "SEE_DOCTOR_PARTS" => Some(g_SEE_DOCTOR_PARTS()),
            "SEE_TODAY_BACK_AND_CALF" => Some(g_SEE_TODAY_BACK_AND_CALF()),
            "SOFT_TISSUE" => Some(g_SOFT_TISSUE()),
            "SOMEWHERE_ELSE" => Some(g_SOMEWHERE_ELSE()),
            "SPELLED_OUT" => Some(g_SPELLED_OUT()),
            "STANDING" => Some(g_STANDING()),
            "TEMPERATURE_WORDS" => Some(g_TEMPERATURE_WORDS()),
            "THING_AFTER_THE_WORD" => Some(g_THING_AFTER_THE_WORD()),
            "THING_PARTS" => Some(g_THING_PARTS()),
            "THIRD_PARTY" => Some(g_THIRD_PARTY()),
            "TIGHT_CHEST" => Some(g_TIGHT_CHEST()),
            "TOOTH_WORDS" => Some(g_TOOTH_WORDS()),
            "WEEKS" => Some(g_WEEKS()),
            "WE_OUR_OURS" => Some(g_WE_OUR_OURS()),
            "_ALL_PART_WORDS" => Some(g__ALL_PART_WORDS()),
            "_ALSO_THE_BACK" => Some(g__ALSO_THE_BACK()),
            "_AN_IMPLANT" => Some(g__AN_IMPLANT()),
            "_ARTHRITIS_CURE" => Some(g__ARTHRITIS_CURE()),
            "_ARTHRITIS_TAIL" => Some(g__ARTHRITIS_TAIL()),
            "_A_BECOMING_A_DENTIST" => Some(g__A_BECOMING_A_DENTIST()),
            "_A_CHECK_UP" => Some(g__A_CHECK_UP()),
            "_A_CROWN_VISIT" => Some(g__A_CROWN_VISIT()),
            "_A_FILLING" => Some(g__A_FILLING()),
            "_A_HEALING_WORD" => Some(g__A_HEALING_WORD()),
            "_A_HYGIENIST" => Some(g__A_HYGIENIST()),
            "_A_JOINT_OF_MEAT" => Some(g__A_JOINT_OF_MEAT()),
            "_A_LENGTH_OF_A_VISIT" => Some(g__A_LENGTH_OF_A_VISIT()),
            "_A_LIST_THAT_IS_LONG" => Some(g__A_LIST_THAT_IS_LONG()),
            "_A_PART_IN_THE_TABLE" => Some(g__A_PART_IN_THE_TABLE()),
            "_A_PART_NAMED" => Some(g__A_PART_NAMED()),
            "_A_PRACTICE_ERRAND" => Some(g__A_PRACTICE_ERRAND()),
            "_A_REST_NOT_A_BONE" => Some(g__A_REST_NOT_A_BONE()),
            "_A_RETURN_TO" => Some(g__A_RETURN_TO()),
            "_A_ROOT_CANAL" => Some(g__A_ROOT_CANAL()),
            "_A_THING_THAT_IS_NOT_A_BODY" => Some(g__A_THING_THAT_IS_NOT_A_BODY()),
            "_A_VERB_TOOK_THE_BACK" => Some(g__A_VERB_TOOK_THE_BACK()),
            "_A_WHILE_AGO" => Some(g__A_WHILE_AGO()),
            "_BACK_AFTER_A_VERB" => Some(g__BACK_AFTER_A_VERB()),
            "_BACK_BEFORE_A_PLACE" => Some(g__BACK_BEFORE_A_PLACE()),
            "_BACK_CLAUSE_JOIN" => Some(g__BACK_CLAUSE_JOIN()),
            "_BACK_DESCRIBED" => Some(g__BACK_DESCRIBED()),
            "_BACK_HOME" => Some(g__BACK_HOME()),
            "_BACK_IS_A_NOUN_AFTER" => Some(g__BACK_IS_A_NOUN_AFTER()),
            "_BACK_IS_A_RETURN" => Some(g__BACK_IS_A_RETURN()),
            "_BACK_TAKING_A_THING" => Some(g__BACK_TAKING_A_THING()),
            "_BACK_THEIRS" => Some(g__BACK_THEIRS()),
            "_BACK_VERB_BEFORE" => Some(g__BACK_VERB_BEFORE()),
            "_BAD_PART_MOVEMENT" => Some(g__BAD_PART_MOVEMENT()),
            "_BAD_PART_WORDS" => Some(g__BAD_PART_WORDS()),
            "_BEARD_TRIM_REPLY" => Some(g__BEARD_TRIM_REPLY()),
            "_BEARD_TRIM_WORDS" => Some(g__BEARD_TRIM_WORDS()),
            "_BLADE_WORDS" => Some(g__BLADE_WORDS()),
            "_CANNOT_USE_IT" => Some(g__CANNOT_USE_IT()),
            "_CARRIES_WEIGHT" => Some(g__CARRIES_WEIGHT()),
            "_CARTILAGE_PIERCING" => Some(g__CARTILAGE_PIERCING()),
            "_CHECK_HOW_TO" => Some(g__CHECK_HOW_TO()),
            "_CHICKENPOX_TIMELINE" => Some(g__CHICKENPOX_TIMELINE()),
            "_DECISION_ASK" => Some(g__DECISION_ASK()),
            "_DECISION_WORDS" => Some(g__DECISION_WORDS()),
            "_DETERMINERS" => Some(g__DETERMINERS()),
            "_DRUGS" => Some(g__DRUGS()),
            "_FALL_AVOID" => Some(g__FALL_AVOID()),
            "_FEVER_ADVICE" => Some(g__FEVER_ADVICE()),
            "_GONE_BACK" => Some(g__GONE_BACK()),
            "_HEAL_PARTS" => Some(g__HEAL_PARTS()),
            "_HEAL_TIME" => Some(g__HEAL_TIME()),
            "_HEAT_OR_ICE_THE_THING" => Some(g__HEAT_OR_ICE_THE_THING()),
            "_IT_HAPPENED_TO" => Some(g__IT_HAPPENED_TO()),
            "_KEPT_AWAKE" => Some(g__KEPT_AWAKE()),
            "_KIDNEY_WORDS" => Some(g__KIDNEY_WORDS()),
            "_MOLE_CHANGED_TEXT" => Some(g__MOLE_CHANGED_TEXT()),
            "_MOLE_CLEAR" => Some(g__MOLE_CLEAR()),
            "_MOLE_ORDINARY_TEXT" => Some(g__MOLE_ORDINARY_TEXT()),
            "_MOLE_REMOVE_TEXT" => Some(g__MOLE_REMOVE_TEXT()),
            "_MUSCLE_PARTS" => Some(g__MUSCLE_PARTS()),
            "_NEW_SUBJECT" => Some(g__NEW_SUBJECT()),
            "_NOT_AN_NSAID" => Some(g__NOT_AN_NSAID()),
            "_NOT_A_BODY_PART" => Some(g__NOT_A_BODY_PART()),
            "_NOT_A_TAMPON_LINE" => Some(g__NOT_A_TAMPON_LINE()),
            "_NOT_ON_A_BODY" => Some(g__NOT_ON_A_BODY()),
            "_NOT_THE_TABLET" => Some(g__NOT_THE_TABLET()),
            "_OFF_A_THING" => Some(g__OFF_A_THING()),
            "_ON_A_BODY" => Some(g__ON_A_BODY()),
            "_OTHER_PEOPLE" => Some(g__OTHER_PEOPLE()),
            "_PARTS" => Some(g__PARTS()),
            "_PERSON_OWNER" => Some(g__PERSON_OWNER()),
            "_PIERCING_BUMP" => Some(g__PIERCING_BUMP()),
            "_PIERCING_NOT_TROUBLE" => Some(g__PIERCING_NOT_TROUBLE()),
            "_PIERCING_PIECE" => Some(g__PIERCING_PIECE()),
            "_PIERCING_ROUTINE" => Some(g__PIERCING_ROUTINE()),
            "_PIERCING_TROUBLE" => Some(g__PIERCING_TROUBLE()),
            "_POSSESSIVES" => Some(g__POSSESSIVES()),
            "_PREGNANCY_SWELLING" => Some(g__PREGNANCY_SWELLING()),
            "_PREGNANCY_SWELLING_AVOID" => Some(g__PREGNANCY_SWELLING_AVOID()),
            "_PREGNANCY_SWELLING_PARTS" => Some(g__PREGNANCY_SWELLING_PARTS()),
            "_PREGNANCY_SWELLING_REPLY" => Some(g__PREGNANCY_SWELLING_REPLY()),
            "_PREGNANCY_WORDS" => Some(g__PREGNANCY_WORDS()),
            "_PREGNANT_WORDS" => Some(g__PREGNANT_WORDS()),
            "_PROTECT_ALLOWED" => Some(g__PROTECT_ALLOWED()),
            "_PROTECT_ASK" => Some(g__PROTECT_ASK()),
            "_PROTECT_FILLERS" => Some(g__PROTECT_FILLERS()),
            "_RAZOR_BLADE_REPLY" => Some(g__RAZOR_BLADE_REPLY()),
            "_RAZOR_BURN_REPLY" => Some(g__RAZOR_BURN_REPLY()),
            "_REGISTERING_A_DENTIST" => Some(g__REGISTERING_A_DENTIST()),
            "_REPORTED_COMPLAINT" => Some(g__REPORTED_COMPLAINT()),
            "_SHAVE_BURN_WORDS" => Some(g__SHAVE_BURN_WORDS()),
            "_SHAVE_FIRST_REPLY" => Some(g__SHAVE_FIRST_REPLY()),
            "_SOMEONE_ELSE" => Some(g__SOMEONE_ELSE()),
            "_SORENESS_WORDS" => Some(g__SORENESS_WORDS()),
            "_SORE_ANYWHERE" => Some(g__SORE_ANYWHERE()),
            "_SPORT_WORDS" => Some(g__SPORT_WORDS()),
            "_STANDING_KNEE" => Some(g__STANDING_KNEE()),
            "_STANDING_TROUBLE" => Some(g__STANDING_TROUBLE()),
            "_STRAIGHT_SHAVE_REPLY" => Some(g__STRAIGHT_SHAVE_REPLY()),
            "_SWELLING" => Some(g__SWELLING()),
            "_TAKE_A_TEMPERATURE" => Some(g__TAKE_A_TEMPERATURE()),
            "_TAMPON_WORD" => Some(g__TAMPON_WORD()),
            "_THE_BACK_IS_THEIRS" => Some(g__THE_BACK_IS_THEIRS()),
            "_THE_BOOKING" => Some(g__THE_BOOKING()),
            "_THE_BOOKING_WORDS" => Some(g__THE_BOOKING_WORDS()),
            "_THE_PERSONS_OWN_WORDS" => Some(g__THE_PERSONS_OWN_WORDS()),
            "_THE_QUEUE" => Some(g__THE_QUEUE()),
            "_THE_VISIT" => Some(g__THE_VISIT()),
            "_THE_VISIT_IN_ORDER" => Some(g__THE_VISIT_IN_ORDER()),
            "_TO_HURT_A_PART" => Some(g__TO_HURT_A_PART()),
            "_WAX_FIRST_REPLY" => Some(g__WAX_FIRST_REPLY()),
            "_WAX_WORDS" => Some(g__WAX_WORDS()),
            "_WEEKS_ALONG" => Some(g__WEEKS_ALONG()),
            "_WEEKS_DRILL" => Some(g__WEEKS_DRILL()),
            "_WORKSHOP" => Some(g__WORKSHOP()),
            "_WORKSHOP_NOUNS" => Some(g__WORKSHOP_NOUNS()),
            "_a_back_in_the_chat" => Some(py_func("_a_back_in_the_chat", |args: &[Py]| _a_back_in_the_chat(args))),
            "_a_bare_decision" => Some(py_func("_a_bare_decision", |args: &[Py]| _a_bare_decision(args))),
            "_a_bare_followup" => Some(py_func("_a_bare_followup", |args: &[Py]| _a_bare_followup(args))),
            "_a_decision_is_not_a_booking" => Some(py_func("_a_decision_is_not_a_booking", |args: &[Py]| _a_decision_is_not_a_booking(args))),
            "_a_figure_in_weeks" => Some(py_func("_a_figure_in_weeks", |args: &[Py]| _a_figure_in_weeks(args))),
            "_a_healing_piercing_not_a_troubled_one" => Some(py_func("_a_healing_piercing_not_a_troubled_one", |args: &[Py]| _a_healing_piercing_not_a_troubled_one(args))),
            "_a_part_in_a_decision_is_a_part" => Some(py_func("_a_part_in_a_decision_is_a_part", |args: &[Py]| _a_part_in_a_decision_is_a_part(args))),
            "_a_part_keeps_me_from_something" => Some(py_func("_a_part_keeps_me_from_something", |args: &[Py]| _a_part_keeps_me_from_something(args))),
            "_a_piercing_in_trouble" => Some(py_func("_a_piercing_in_trouble", |args: &[Py]| _a_piercing_in_trouble(args))),
            "_a_place_you_would_ice" => Some(py_func("_a_place_you_would_ice", |args: &[Py]| _a_place_you_would_ice(args))),
            "_a_region_guard" => Some(py_func("_a_region_guard", |args: &[Py]| _a_region_guard(args))),
            "_a_tampon_worn_too_long" => Some(py_func("_a_tampon_worn_too_long", |args: &[Py]| _a_tampon_worn_too_long(args))),
            "_a_thing_owns_the_part" => Some(py_func("_a_thing_owns_the_part", |args: &[Py]| _a_thing_owns_the_part(args))),
            "_a_verb_earlier_took_it" => Some(py_func("_a_verb_earlier_took_it", |args: &[Py]| _a_verb_earlier_took_it(args))),
            "_an_errand_the_dentist_card_answers" => Some(py_func("_an_errand_the_dentist_card_answers", |args: &[Py]| _an_errand_the_dentist_card_answers(args))),
            "_arthritis_reply" => Some(py_func("_arthritis_reply", |args: &[Py]| _arthritis_reply(args))),
            "_back_is_a_direction" => Some(py_func("_back_is_a_direction", |args: &[Py]| _back_is_a_direction(args))),
            "_bad_part_reply" => Some(py_func("_bad_part_reply", |args: &[Py]| _bad_part_reply(args))),
            "_bare_words" => Some(py_func("_bare_words", |args: &[Py]| _bare_words(args))),
            "_book_the_appointment" => Some(py_func("_book_the_appointment", |args: &[Py]| _book_the_appointment(args))),
            "_broken_or_sprained" => Some(py_func("_broken_or_sprained", |args: &[Py]| _broken_or_sprained(args))),
            "_chickenpox_reply" => Some(py_func("_chickenpox_reply", |args: &[Py]| _chickenpox_reply(args))),
            "_dates_a_pregnancy_in_weeks" => Some(py_func("_dates_a_pregnancy_in_weeks", |args: &[Py]| _dates_a_pregnancy_in_weeks(args))),
            "_dose_for_the_drug_named" => Some(py_func("_dose_for_the_drug_named", |args: &[Py]| _dose_for_the_drug_named(args))),
            "_face_shaving" => Some(py_func("_face_shaving", |args: &[Py]| _face_shaving(args))),
            "_fever_reply" => Some(py_func("_fever_reply", |args: &[Py]| _fever_reply(args))),
            "_hair_removal" => Some(py_func("_hair_removal", |args: &[Py]| _hair_removal(args))),
            "_healing" => Some(py_func("_healing", |args: &[Py]| _healing(args))),
            "_how_long" => Some(py_func("_how_long", |args: &[Py]| _how_long(args))),
            "_in_the_allow_list" => Some(py_func("_in_the_allow_list", |args: &[Py]| _in_the_allow_list(args))),
            "_it_is_somebody_elses_part" => Some(py_func("_it_is_somebody_elses_part", |args: &[Py]| _it_is_somebody_elses_part(args))),
            "_line_words" => Some(py_func("_line_words", |args: &[Py]| _line_words(args))),
            "_mole_reply" => Some(py_func("_mole_reply", |args: &[Py]| _mole_reply(args))),
            "_muscle_healing" => Some(py_func("_muscle_healing", |args: &[Py]| _muscle_healing(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_names_a_pregnancy" => Some(py_func("_names_a_pregnancy", |args: &[Py]| _names_a_pregnancy(args))),
            "_not_a_break_from_work" => Some(py_func("_not_a_break_from_work", |args: &[Py]| _not_a_break_from_work(args))),
            "_owner_word" => Some(py_func("_owner_word", |args: &[Py]| _owner_word(args))),
            "_part_named" => Some(py_func("_part_named", |args: &[Py]| _part_named(args))),
            "_piercing_healing" => Some(py_func("_piercing_healing", |args: &[Py]| _piercing_healing(args))),
            "_piercing_infection" => Some(py_func("_piercing_infection", |args: &[Py]| _piercing_infection(args))),
            "_protect_reply" => Some(py_func("_protect_reply", |args: &[Py]| _protect_reply(args))),
            "_reading_words" => Some(py_func("_reading_words", |args: &[Py]| _reading_words(args))),
            "_red_flag" => Some(py_func("_red_flag", |args: &[Py]| _red_flag(args))),
            "_said_already" => Some(py_func("_said_already", |args: &[Py]| _said_already(args))),
            "_sea_water_reply" => Some(py_func("_sea_water_reply", |args: &[Py]| _sea_water_reply(args))),
            "_see_doctor_decision" => Some(py_func("_see_doctor_decision", |args: &[Py]| _see_doctor_decision(args))),
            "_see_doctor_reply" => Some(py_func("_see_doctor_reply", |args: &[Py]| _see_doctor_reply(args))),
            "_see_today" => Some(py_func("_see_today", |args: &[Py]| _see_today(args))),
            "_sore_weeks_reply" => Some(py_func("_sore_weeks_reply", |args: &[Py]| _sore_weeks_reply(args))),
            "_take_a_temperature" => Some(py_func("_take_a_temperature", |args: &[Py]| _take_a_temperature(args))),
            "_the_ask_names_nothing_else" => Some(py_func("_the_ask_names_nothing_else", |args: &[Py]| _the_ask_names_nothing_else(args))),
            "_the_back_is_a_back" => Some(py_func("_the_back_is_a_back", |args: &[Py]| _the_back_is_a_back(args))),
            "_the_back_is_a_back_that_is_someones" => Some(py_func("_the_back_is_a_back_that_is_someones", |args: &[Py]| _the_back_is_a_back_that_is_someones(args))),
            "_the_back_is_the_persons" => Some(py_func("_the_back_is_the_persons", |args: &[Py]| _the_back_is_the_persons(args))),
            "_the_breaks_it_says" => Some(py_func("_the_breaks_it_says", |args: &[Py]| _the_breaks_it_says(args))),
            "_the_day_after_new_exercise" => Some(py_func("_the_day_after_new_exercise", |args: &[Py]| _the_day_after_new_exercise(args))),
            "_the_dentist_errand" => Some(py_func("_the_dentist_errand", |args: &[Py]| _the_dentist_errand(args))),
            "_the_lengths_of_the_visit" => Some(py_func("_the_lengths_of_the_visit", |args: &[Py]| _the_lengths_of_the_visit(args))),
            "_the_nail_is_a_persons" => Some(py_func("_the_nail_is_a_persons", |args: &[Py]| _the_nail_is_a_persons(args))),
            "_the_organ_the_painkiller_upsets" => Some(py_func("_the_organ_the_painkiller_upsets", |args: &[Py]| _the_organ_the_painkiller_upsets(args))),
            "_the_part_is_a_persons" => Some(py_func("_the_part_is_a_persons", |args: &[Py]| _the_part_is_a_persons(args))),
            "_the_part_is_named_today" => Some(py_func("_the_part_is_named_today", |args: &[Py]| _the_part_is_named_today(args))),
            "_the_part_is_not_a_standing_trouble" => Some(py_func("_the_part_is_not_a_standing_trouble", |args: &[Py]| _the_part_is_not_a_standing_trouble(args))),
            "_what_can_i_take_pregnant_or_feeding" => Some(py_func("_what_can_i_take_pregnant_or_feeding", |args: &[Py]| _what_can_i_take_pregnant_or_feeding(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "a_persons_temperature" => Some(py_func("a_persons_temperature", |args: &[Py]| a_persons_temperature(args))),
            "entry" => Some(g_entry()),
            "the_body_is_the_persons" => Some(py_func("the_body_is_the_persons", |args: &[Py]| the_body_is_the_persons(args))),
            "the_part_that_hurts" => Some(py_func("the_part_that_hurts", |args: &[Py]| the_part_that_hurts(args))),
            "the_part_that_keeps_grumbling" => Some(py_func("the_part_that_keeps_grumbling", |args: &[Py]| the_part_that_keeps_grumbling(args))),
            "the_stomach_is_what_it_is_about" => Some(py_func("the_stomach_is_what_it_is_about", |args: &[Py]| the_stomach_is_what_it_is_about(args))),
            "re" => Some(stdlib_module("re")),
            "string" => Some(stdlib_module("string")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACHE_WORDS" => s_ACHE_WORDS(v),
            "ANSWERS" => s_ANSWERS(v),
            "ASKED_AGAIN" => s_ASKED_AGAIN(v),
            "ASKING_ABOUT_A_PART" => s_ASKING_ABOUT_A_PART(v),
            "ASK_ABOUT" => s_ASK_ABOUT(v),
            "AT_YOUR_SHOULDER" => s_AT_YOUR_SHOULDER(v),
            "AT_YOUR_SHOULDER_REPLY" => s_AT_YOUR_SHOULDER_REPLY(v),
            "AVOID" => s_AVOID(v),
            "A_PART_NO_KNOCK_TEST" => s_A_PART_NO_KNOCK_TEST(v),
            "BACK_NAMES" => s_BACK_NAMES(v),
            "BACK_PREGNANCY_KEEP" => s_BACK_PREGNANCY_KEEP(v),
            "BACK_WENT" => s_BACK_WENT(v),
            "BREAK_SAID" => s_BREAK_SAID(v),
            "BROKEN_PARTS" => s_BROKEN_PARTS(v),
            "BROKEN_WORDS" => s_BROKEN_WORDS(v),
            "CLOT_AND_CHEST" => s_CLOT_AND_CHEST(v),
            "COMPRESS_WORDS" => s_COMPRESS_WORDS(v),
            "DECIDE" => s_DECIDE(v),
            "DECIDE_ASK" => s_DECIDE_ASK(v),
            "ENTRIES" => s_ENTRIES(v),
            "EXERCISE_AFTER" => s_EXERCISE_AFTER(v),
            "EXERCISE_DAY" => s_EXERCISE_DAY(v),
            "EXTRA_AVOID" => s_EXTRA_AVOID(v),
            "FALL" => s_FALL(v),
            "FLAG_WITH_IT" => s_FLAG_WITH_IT(v),
            "FOOT_PARTS" => s_FOOT_PARTS(v),
            "FOUR_STEPS_REPLY" => s_FOUR_STEPS_REPLY(v),
            "HAND_PARTS" => s_HAND_PARTS(v),
            "HOT_FLAGS" => s_HOT_FLAGS(v),
            "HOT_SWELLING" => s_HOT_SWELLING(v),
            "HURT" => s_HURT(v),
            "HURT_WORDS" => s_HURT_WORDS(v),
            "IN_YOUR_HANDS" => s_IN_YOUR_HANDS(v),
            "IN_YOUR_HANDS_REPLY" => s_IN_YOUR_HANDS_REPLY(v),
            "JOINTS" => s_JOINTS(v),
            "KNEE_WORDS" => s_KNEE_WORDS(v),
            "LOWER_HALF" => s_LOWER_HALF(v),
            "MEDICINE" => s_MEDICINE(v),
            "MOLE_CHANGE" => s_MOLE_CHANGE(v),
            "MOLE_HAPPENING" => s_MOLE_HAPPENING(v),
            "MOLE_REMOVE" => s_MOLE_REMOVE(v),
            "NUMB_ANYWHERE" => s_NUMB_ANYWHERE(v),
            "NUMB_ASK" => s_NUMB_ASK(v),
            "NUMB_AVOID" => s_NUMB_AVOID(v),
            "NUMB_FOOT" => s_NUMB_FOOT(v),
            "NUMB_HANDS" => s_NUMB_HANDS(v),
            "NUMB_LOWER" => s_NUMB_LOWER(v),
            "NUMB_SHOE" => s_NUMB_SHOE(v),
            "OBJECT_OWNERS" => s_OBJECT_OWNERS(v),
            "ON_YOUR_FEET" => s_ON_YOUR_FEET(v),
            "OTHER_PEOPLE" => s_OTHER_PEOPLE(v),
            "PAINKILLER_NAMES" => s_PAINKILLER_NAMES(v),
            "PART_ONE" => s_PART_ONE(v),
            "PERSON_NOUNS" => s_PERSON_NOUNS(v),
            "POSSESSIVE_OF_A_PERSON" => s_POSSESSIVE_OF_A_PERSON(v),
            "PREGNANT" => s_PREGNANT(v),
            "RED_FLAG" => s_RED_FLAG(v),
            "RED_FLAG_CARD" => s_RED_FLAG_CARD(v),
            "REGIONS" => s_REGIONS(v),
            "REGION_BIAS" => s_REGION_BIAS(v),
            "RING_AS_A_CALL" => s_RING_AS_A_CALL(v),
            "RING_NOT_ON_A_FINGER" => s_RING_NOT_ON_A_FINGER(v),
            "SADDLE_CARD" => s_SADDLE_CARD(v),
            "SADDLE_PARTS" => s_SADDLE_PARTS(v),
            "SEA_WATER_ALARM" => s_SEA_WATER_ALARM(v),
            "SEA_WATER_ALARM_REPLY" => s_SEA_WATER_ALARM_REPLY(v),
            "SEEN_ABOUT_IT" => s_SEEN_ABOUT_IT(v),
            "SEE_DOCTOR_AVOID" => s_SEE_DOCTOR_AVOID(v),
            "SEE_DOCTOR_PARTS" => s_SEE_DOCTOR_PARTS(v),
            "SEE_TODAY_BACK_AND_CALF" => s_SEE_TODAY_BACK_AND_CALF(v),
            "SOFT_TISSUE" => s_SOFT_TISSUE(v),
            "SOMEWHERE_ELSE" => s_SOMEWHERE_ELSE(v),
            "SPELLED_OUT" => s_SPELLED_OUT(v),
            "STANDING" => s_STANDING(v),
            "TEMPERATURE_WORDS" => s_TEMPERATURE_WORDS(v),
            "THING_AFTER_THE_WORD" => s_THING_AFTER_THE_WORD(v),
            "THING_PARTS" => s_THING_PARTS(v),
            "THIRD_PARTY" => s_THIRD_PARTY(v),
            "TIGHT_CHEST" => s_TIGHT_CHEST(v),
            "TOOTH_WORDS" => s_TOOTH_WORDS(v),
            "WEEKS" => s_WEEKS(v),
            "WE_OUR_OURS" => s_WE_OUR_OURS(v),
            "_ALL_PART_WORDS" => s__ALL_PART_WORDS(v),
            "_ALSO_THE_BACK" => s__ALSO_THE_BACK(v),
            "_AN_IMPLANT" => s__AN_IMPLANT(v),
            "_ARTHRITIS_CURE" => s__ARTHRITIS_CURE(v),
            "_ARTHRITIS_TAIL" => s__ARTHRITIS_TAIL(v),
            "_A_BECOMING_A_DENTIST" => s__A_BECOMING_A_DENTIST(v),
            "_A_CHECK_UP" => s__A_CHECK_UP(v),
            "_A_CROWN_VISIT" => s__A_CROWN_VISIT(v),
            "_A_FILLING" => s__A_FILLING(v),
            "_A_HEALING_WORD" => s__A_HEALING_WORD(v),
            "_A_HYGIENIST" => s__A_HYGIENIST(v),
            "_A_JOINT_OF_MEAT" => s__A_JOINT_OF_MEAT(v),
            "_A_LENGTH_OF_A_VISIT" => s__A_LENGTH_OF_A_VISIT(v),
            "_A_LIST_THAT_IS_LONG" => s__A_LIST_THAT_IS_LONG(v),
            "_A_PART_IN_THE_TABLE" => s__A_PART_IN_THE_TABLE(v),
            "_A_PART_NAMED" => s__A_PART_NAMED(v),
            "_A_PRACTICE_ERRAND" => s__A_PRACTICE_ERRAND(v),
            "_A_REST_NOT_A_BONE" => s__A_REST_NOT_A_BONE(v),
            "_A_RETURN_TO" => s__A_RETURN_TO(v),
            "_A_ROOT_CANAL" => s__A_ROOT_CANAL(v),
            "_A_THING_THAT_IS_NOT_A_BODY" => s__A_THING_THAT_IS_NOT_A_BODY(v),
            "_A_VERB_TOOK_THE_BACK" => s__A_VERB_TOOK_THE_BACK(v),
            "_A_WHILE_AGO" => s__A_WHILE_AGO(v),
            "_BACK_AFTER_A_VERB" => s__BACK_AFTER_A_VERB(v),
            "_BACK_BEFORE_A_PLACE" => s__BACK_BEFORE_A_PLACE(v),
            "_BACK_CLAUSE_JOIN" => s__BACK_CLAUSE_JOIN(v),
            "_BACK_DESCRIBED" => s__BACK_DESCRIBED(v),
            "_BACK_HOME" => s__BACK_HOME(v),
            "_BACK_IS_A_NOUN_AFTER" => s__BACK_IS_A_NOUN_AFTER(v),
            "_BACK_IS_A_RETURN" => s__BACK_IS_A_RETURN(v),
            "_BACK_TAKING_A_THING" => s__BACK_TAKING_A_THING(v),
            "_BACK_THEIRS" => s__BACK_THEIRS(v),
            "_BACK_VERB_BEFORE" => s__BACK_VERB_BEFORE(v),
            "_BAD_PART_MOVEMENT" => s__BAD_PART_MOVEMENT(v),
            "_BAD_PART_WORDS" => s__BAD_PART_WORDS(v),
            "_BEARD_TRIM_REPLY" => s__BEARD_TRIM_REPLY(v),
            "_BEARD_TRIM_WORDS" => s__BEARD_TRIM_WORDS(v),
            "_BLADE_WORDS" => s__BLADE_WORDS(v),
            "_CANNOT_USE_IT" => s__CANNOT_USE_IT(v),
            "_CARRIES_WEIGHT" => s__CARRIES_WEIGHT(v),
            "_CARTILAGE_PIERCING" => s__CARTILAGE_PIERCING(v),
            "_CHECK_HOW_TO" => s__CHECK_HOW_TO(v),
            "_CHICKENPOX_TIMELINE" => s__CHICKENPOX_TIMELINE(v),
            "_DECISION_ASK" => s__DECISION_ASK(v),
            "_DECISION_WORDS" => s__DECISION_WORDS(v),
            "_DETERMINERS" => s__DETERMINERS(v),
            "_DRUGS" => s__DRUGS(v),
            "_FALL_AVOID" => s__FALL_AVOID(v),
            "_FEVER_ADVICE" => s__FEVER_ADVICE(v),
            "_GONE_BACK" => s__GONE_BACK(v),
            "_HEAL_PARTS" => s__HEAL_PARTS(v),
            "_HEAL_TIME" => s__HEAL_TIME(v),
            "_HEAT_OR_ICE_THE_THING" => s__HEAT_OR_ICE_THE_THING(v),
            "_IT_HAPPENED_TO" => s__IT_HAPPENED_TO(v),
            "_KEPT_AWAKE" => s__KEPT_AWAKE(v),
            "_KIDNEY_WORDS" => s__KIDNEY_WORDS(v),
            "_MOLE_CHANGED_TEXT" => s__MOLE_CHANGED_TEXT(v),
            "_MOLE_CLEAR" => s__MOLE_CLEAR(v),
            "_MOLE_ORDINARY_TEXT" => s__MOLE_ORDINARY_TEXT(v),
            "_MOLE_REMOVE_TEXT" => s__MOLE_REMOVE_TEXT(v),
            "_MUSCLE_PARTS" => s__MUSCLE_PARTS(v),
            "_NEW_SUBJECT" => s__NEW_SUBJECT(v),
            "_NOT_AN_NSAID" => s__NOT_AN_NSAID(v),
            "_NOT_A_BODY_PART" => s__NOT_A_BODY_PART(v),
            "_NOT_A_TAMPON_LINE" => s__NOT_A_TAMPON_LINE(v),
            "_NOT_ON_A_BODY" => s__NOT_ON_A_BODY(v),
            "_NOT_THE_TABLET" => s__NOT_THE_TABLET(v),
            "_OFF_A_THING" => s__OFF_A_THING(v),
            "_ON_A_BODY" => s__ON_A_BODY(v),
            "_OTHER_PEOPLE" => s__OTHER_PEOPLE(v),
            "_PARTS" => s__PARTS(v),
            "_PERSON_OWNER" => s__PERSON_OWNER(v),
            "_PIERCING_BUMP" => s__PIERCING_BUMP(v),
            "_PIERCING_NOT_TROUBLE" => s__PIERCING_NOT_TROUBLE(v),
            "_PIERCING_PIECE" => s__PIERCING_PIECE(v),
            "_PIERCING_ROUTINE" => s__PIERCING_ROUTINE(v),
            "_PIERCING_TROUBLE" => s__PIERCING_TROUBLE(v),
            "_POSSESSIVES" => s__POSSESSIVES(v),
            "_PREGNANCY_SWELLING" => s__PREGNANCY_SWELLING(v),
            "_PREGNANCY_SWELLING_AVOID" => s__PREGNANCY_SWELLING_AVOID(v),
            "_PREGNANCY_SWELLING_PARTS" => s__PREGNANCY_SWELLING_PARTS(v),
            "_PREGNANCY_SWELLING_REPLY" => s__PREGNANCY_SWELLING_REPLY(v),
            "_PREGNANCY_WORDS" => s__PREGNANCY_WORDS(v),
            "_PREGNANT_WORDS" => s__PREGNANT_WORDS(v),
            "_PROTECT_ALLOWED" => s__PROTECT_ALLOWED(v),
            "_PROTECT_ASK" => s__PROTECT_ASK(v),
            "_PROTECT_FILLERS" => s__PROTECT_FILLERS(v),
            "_RAZOR_BLADE_REPLY" => s__RAZOR_BLADE_REPLY(v),
            "_RAZOR_BURN_REPLY" => s__RAZOR_BURN_REPLY(v),
            "_REGISTERING_A_DENTIST" => s__REGISTERING_A_DENTIST(v),
            "_REPORTED_COMPLAINT" => s__REPORTED_COMPLAINT(v),
            "_SHAVE_BURN_WORDS" => s__SHAVE_BURN_WORDS(v),
            "_SHAVE_FIRST_REPLY" => s__SHAVE_FIRST_REPLY(v),
            "_SOMEONE_ELSE" => s__SOMEONE_ELSE(v),
            "_SORENESS_WORDS" => s__SORENESS_WORDS(v),
            "_SORE_ANYWHERE" => s__SORE_ANYWHERE(v),
            "_SPORT_WORDS" => s__SPORT_WORDS(v),
            "_STANDING_KNEE" => s__STANDING_KNEE(v),
            "_STANDING_TROUBLE" => s__STANDING_TROUBLE(v),
            "_STRAIGHT_SHAVE_REPLY" => s__STRAIGHT_SHAVE_REPLY(v),
            "_SWELLING" => s__SWELLING(v),
            "_TAKE_A_TEMPERATURE" => s__TAKE_A_TEMPERATURE(v),
            "_TAMPON_WORD" => s__TAMPON_WORD(v),
            "_THE_BACK_IS_THEIRS" => s__THE_BACK_IS_THEIRS(v),
            "_THE_BOOKING" => s__THE_BOOKING(v),
            "_THE_BOOKING_WORDS" => s__THE_BOOKING_WORDS(v),
            "_THE_PERSONS_OWN_WORDS" => s__THE_PERSONS_OWN_WORDS(v),
            "_THE_QUEUE" => s__THE_QUEUE(v),
            "_THE_VISIT" => s__THE_VISIT(v),
            "_THE_VISIT_IN_ORDER" => s__THE_VISIT_IN_ORDER(v),
            "_TO_HURT_A_PART" => s__TO_HURT_A_PART(v),
            "_WAX_FIRST_REPLY" => s__WAX_FIRST_REPLY(v),
            "_WAX_WORDS" => s__WAX_WORDS(v),
            "_WEEKS_ALONG" => s__WEEKS_ALONG(v),
            "_WEEKS_DRILL" => s__WEEKS_DRILL(v),
            "_WORKSHOP" => s__WORKSHOP(v),
            "_WORKSHOP_NOUNS" => s__WORKSHOP_NOUNS(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.body", module_obj);
    pyrt::xrefs::reg_fn("knowledge.body", "_the_nail_is_a_persons", py_func("_the_nail_is_a_persons", |args: &[Py]| _the_nail_is_a_persons(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_part_is_a_persons", py_func("_the_part_is_a_persons", |args: &[Py]| _the_part_is_a_persons(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "the_body_is_the_persons", py_func("the_body_is_the_persons", |args: &[Py]| the_body_is_the_persons(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "the_part_that_keeps_grumbling", py_func("the_part_that_keeps_grumbling", |args: &[Py]| the_part_that_keeps_grumbling(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_names_a_pregnancy", py_func("_names_a_pregnancy", |args: &[Py]| _names_a_pregnancy(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_dates_a_pregnancy_in_weeks", py_func("_dates_a_pregnancy_in_weeks", |args: &[Py]| _dates_a_pregnancy_in_weeks(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_figure_in_weeks", py_func("_a_figure_in_weeks", |args: &[Py]| _a_figure_in_weeks(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "a_persons_temperature", py_func("a_persons_temperature", |args: &[Py]| a_persons_temperature(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "the_stomach_is_what_it_is_about", py_func("the_stomach_is_what_it_is_about", |args: &[Py]| the_stomach_is_what_it_is_about(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_see_today", py_func("_see_today", |args: &[Py]| _see_today(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_part_named", py_func("_part_named", |args: &[Py]| _part_named(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_names", py_func("_names", |args: &[Py]| _names(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_red_flag", py_func("_red_flag", |args: &[Py]| _red_flag(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_dose_for_the_drug_named", py_func("_dose_for_the_drug_named", |args: &[Py]| _dose_for_the_drug_named(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_piercing_in_trouble", py_func("_a_piercing_in_trouble", |args: &[Py]| _a_piercing_in_trouble(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_piercing_infection", py_func("_piercing_infection", |args: &[Py]| _piercing_infection(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_healing_piercing_not_a_troubled_one", py_func("_a_healing_piercing_not_a_troubled_one", |args: &[Py]| _a_healing_piercing_not_a_troubled_one(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_piercing_healing", py_func("_piercing_healing", |args: &[Py]| _piercing_healing(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_an_errand_the_dentist_card_answers", py_func("_an_errand_the_dentist_card_answers", |args: &[Py]| _an_errand_the_dentist_card_answers(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_lengths_of_the_visit", py_func("_the_lengths_of_the_visit", |args: &[Py]| _the_lengths_of_the_visit(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_dentist_errand", py_func("_the_dentist_errand", |args: &[Py]| _the_dentist_errand(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_what_can_i_take_pregnant_or_feeding", py_func("_what_can_i_take_pregnant_or_feeding", |args: &[Py]| _what_can_i_take_pregnant_or_feeding(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_bare_words", py_func("_bare_words", |args: &[Py]| _bare_words(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_book_the_appointment", py_func("_book_the_appointment", |args: &[Py]| _book_the_appointment(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_decision_is_not_a_booking", py_func("_a_decision_is_not_a_booking", |args: &[Py]| _a_decision_is_not_a_booking(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_said_already", py_func("_said_already", |args: &[Py]| _said_already(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_sea_water_reply", py_func("_sea_water_reply", |args: &[Py]| _sea_water_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_owner_word", py_func("_owner_word", |args: &[Py]| _owner_word(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_thing_owns_the_part", py_func("_a_thing_owns_the_part", |args: &[Py]| _a_thing_owns_the_part(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "the_part_that_hurts", py_func("the_part_that_hurts", |args: &[Py]| the_part_that_hurts(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_reading_words", py_func("_reading_words", |args: &[Py]| _reading_words(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_back_is_a_back", py_func("_the_back_is_a_back", |args: &[Py]| _the_back_is_a_back(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_verb_earlier_took_it", py_func("_a_verb_earlier_took_it", |args: &[Py]| _a_verb_earlier_took_it(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_back_is_a_back_that_is_someones", py_func("_the_back_is_a_back_that_is_someones", |args: &[Py]| _the_back_is_a_back_that_is_someones(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_part_is_not_a_standing_trouble", py_func("_the_part_is_not_a_standing_trouble", |args: &[Py]| _the_part_is_not_a_standing_trouble(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_region_guard", py_func("_a_region_guard", |args: &[Py]| _a_region_guard(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_part_keeps_me_from_something", py_func("_a_part_keeps_me_from_something", |args: &[Py]| _a_part_keeps_me_from_something(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_breaks_it_says", py_func("_the_breaks_it_says", |args: &[Py]| _the_breaks_it_says(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_it_is_somebody_elses_part", py_func("_it_is_somebody_elses_part", |args: &[Py]| _it_is_somebody_elses_part(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_line_words", py_func("_line_words", |args: &[Py]| _line_words(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_how_long", py_func("_how_long", |args: &[Py]| _how_long(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_back_is_a_direction", py_func("_back_is_a_direction", |args: &[Py]| _back_is_a_direction(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_back_in_the_chat", py_func("_a_back_in_the_chat", |args: &[Py]| _a_back_in_the_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_sore_weeks_reply", py_func("_sore_weeks_reply", |args: &[Py]| _sore_weeks_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_day_after_new_exercise", py_func("_the_day_after_new_exercise", |args: &[Py]| _the_day_after_new_exercise(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_see_doctor_reply", py_func("_see_doctor_reply", |args: &[Py]| _see_doctor_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_part_in_a_decision_is_a_part", py_func("_a_part_in_a_decision_is_a_part", |args: &[Py]| _a_part_in_a_decision_is_a_part(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_bare_decision", py_func("_a_bare_decision", |args: &[Py]| _a_bare_decision(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_see_doctor_decision", py_func("_see_doctor_decision", |args: &[Py]| _see_doctor_decision(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_back_is_the_persons", py_func("_the_back_is_the_persons", |args: &[Py]| _the_back_is_the_persons(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_part_is_named_today", py_func("_the_part_is_named_today", |args: &[Py]| _the_part_is_named_today(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_bad_part_reply", py_func("_bad_part_reply", |args: &[Py]| _bad_part_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_ask_names_nothing_else", py_func("_the_ask_names_nothing_else", |args: &[Py]| _the_ask_names_nothing_else(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_in_the_allow_list", py_func("_in_the_allow_list", |args: &[Py]| _in_the_allow_list(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_protect_reply", py_func("_protect_reply", |args: &[Py]| _protect_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_take_a_temperature", py_func("_take_a_temperature", |args: &[Py]| _take_a_temperature(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_fever_reply", py_func("_fever_reply", |args: &[Py]| _fever_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_chickenpox_reply", py_func("_chickenpox_reply", |args: &[Py]| _chickenpox_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_arthritis_reply", py_func("_arthritis_reply", |args: &[Py]| _arthritis_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_broken_or_sprained", py_func("_broken_or_sprained", |args: &[Py]| _broken_or_sprained(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_bare_followup", py_func("_a_bare_followup", |args: &[Py]| _a_bare_followup(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_mole_reply", py_func("_mole_reply", |args: &[Py]| _mole_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_the_organ_the_painkiller_upsets", py_func("_the_organ_the_painkiller_upsets", |args: &[Py]| _the_organ_the_painkiller_upsets(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_hair_removal", py_func("_hair_removal", |args: &[Py]| _hair_removal(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_face_shaving", py_func("_face_shaving", |args: &[Py]| _face_shaving(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_not_a_break_from_work", py_func("_not_a_break_from_work", |args: &[Py]| _not_a_break_from_work(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_healing", py_func("_healing", |args: &[Py]| _healing(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_place_you_would_ice", py_func("_a_place_you_would_ice", |args: &[Py]| _a_place_you_would_ice(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_muscle_healing", py_func("_muscle_healing", |args: &[Py]| _muscle_healing(args)));
    pyrt::xrefs::reg_fn("knowledge.body", "_a_tampon_worn_too_long", py_func("_a_tampon_worn_too_long", |args: &[Py]| _a_tampon_worn_too_long(args)));
    pyrt::xrefs::reg_access("knowledge.body", "ACHE_WORDS", g_ACHE_WORDS, s_ACHE_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.body", "ASKED_AGAIN", g_ASKED_AGAIN, s_ASKED_AGAIN);
    pyrt::xrefs::reg_access("knowledge.body", "ASKING_ABOUT_A_PART", g_ASKING_ABOUT_A_PART, s_ASKING_ABOUT_A_PART);
    pyrt::xrefs::reg_access("knowledge.body", "ASK_ABOUT", g_ASK_ABOUT, s_ASK_ABOUT);
    pyrt::xrefs::reg_access("knowledge.body", "AT_YOUR_SHOULDER", g_AT_YOUR_SHOULDER, s_AT_YOUR_SHOULDER);
    pyrt::xrefs::reg_access("knowledge.body", "AT_YOUR_SHOULDER_REPLY", g_AT_YOUR_SHOULDER_REPLY, s_AT_YOUR_SHOULDER_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "AVOID", g_AVOID, s_AVOID);
    pyrt::xrefs::reg_access("knowledge.body", "A_PART_NO_KNOCK_TEST", g_A_PART_NO_KNOCK_TEST, s_A_PART_NO_KNOCK_TEST);
    pyrt::xrefs::reg_access("knowledge.body", "BACK_NAMES", g_BACK_NAMES, s_BACK_NAMES);
    pyrt::xrefs::reg_access("knowledge.body", "BACK_PREGNANCY_KEEP", g_BACK_PREGNANCY_KEEP, s_BACK_PREGNANCY_KEEP);
    pyrt::xrefs::reg_access("knowledge.body", "BACK_WENT", g_BACK_WENT, s_BACK_WENT);
    pyrt::xrefs::reg_access("knowledge.body", "BREAK_SAID", g_BREAK_SAID, s_BREAK_SAID);
    pyrt::xrefs::reg_access("knowledge.body", "BROKEN_PARTS", g_BROKEN_PARTS, s_BROKEN_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "BROKEN_WORDS", g_BROKEN_WORDS, s_BROKEN_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "CLOT_AND_CHEST", g_CLOT_AND_CHEST, s_CLOT_AND_CHEST);
    pyrt::xrefs::reg_access("knowledge.body", "COMPRESS_WORDS", g_COMPRESS_WORDS, s_COMPRESS_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "DECIDE", g_DECIDE, s_DECIDE);
    pyrt::xrefs::reg_access("knowledge.body", "DECIDE_ASK", g_DECIDE_ASK, s_DECIDE_ASK);
    pyrt::xrefs::reg_access("knowledge.body", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.body", "EXERCISE_AFTER", g_EXERCISE_AFTER, s_EXERCISE_AFTER);
    pyrt::xrefs::reg_access("knowledge.body", "EXERCISE_DAY", g_EXERCISE_DAY, s_EXERCISE_DAY);
    pyrt::xrefs::reg_access("knowledge.body", "EXTRA_AVOID", g_EXTRA_AVOID, s_EXTRA_AVOID);
    pyrt::xrefs::reg_access("knowledge.body", "FALL", g_FALL, s_FALL);
    pyrt::xrefs::reg_access("knowledge.body", "FLAG_WITH_IT", g_FLAG_WITH_IT, s_FLAG_WITH_IT);
    pyrt::xrefs::reg_access("knowledge.body", "FOOT_PARTS", g_FOOT_PARTS, s_FOOT_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "FOUR_STEPS_REPLY", g_FOUR_STEPS_REPLY, s_FOUR_STEPS_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "HAND_PARTS", g_HAND_PARTS, s_HAND_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "HOT_FLAGS", g_HOT_FLAGS, s_HOT_FLAGS);
    pyrt::xrefs::reg_access("knowledge.body", "HOT_SWELLING", g_HOT_SWELLING, s_HOT_SWELLING);
    pyrt::xrefs::reg_access("knowledge.body", "HURT", g_HURT, s_HURT);
    pyrt::xrefs::reg_access("knowledge.body", "HURT_WORDS", g_HURT_WORDS, s_HURT_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "IN_YOUR_HANDS", g_IN_YOUR_HANDS, s_IN_YOUR_HANDS);
    pyrt::xrefs::reg_access("knowledge.body", "IN_YOUR_HANDS_REPLY", g_IN_YOUR_HANDS_REPLY, s_IN_YOUR_HANDS_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "JOINTS", g_JOINTS, s_JOINTS);
    pyrt::xrefs::reg_access("knowledge.body", "KNEE_WORDS", g_KNEE_WORDS, s_KNEE_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "LOWER_HALF", g_LOWER_HALF, s_LOWER_HALF);
    pyrt::xrefs::reg_access("knowledge.body", "MEDICINE", g_MEDICINE, s_MEDICINE);
    pyrt::xrefs::reg_access("knowledge.body", "MOLE_CHANGE", g_MOLE_CHANGE, s_MOLE_CHANGE);
    pyrt::xrefs::reg_access("knowledge.body", "MOLE_HAPPENING", g_MOLE_HAPPENING, s_MOLE_HAPPENING);
    pyrt::xrefs::reg_access("knowledge.body", "MOLE_REMOVE", g_MOLE_REMOVE, s_MOLE_REMOVE);
    pyrt::xrefs::reg_access("knowledge.body", "NUMB_ANYWHERE", g_NUMB_ANYWHERE, s_NUMB_ANYWHERE);
    pyrt::xrefs::reg_access("knowledge.body", "NUMB_ASK", g_NUMB_ASK, s_NUMB_ASK);
    pyrt::xrefs::reg_access("knowledge.body", "NUMB_AVOID", g_NUMB_AVOID, s_NUMB_AVOID);
    pyrt::xrefs::reg_access("knowledge.body", "NUMB_FOOT", g_NUMB_FOOT, s_NUMB_FOOT);
    pyrt::xrefs::reg_access("knowledge.body", "NUMB_HANDS", g_NUMB_HANDS, s_NUMB_HANDS);
    pyrt::xrefs::reg_access("knowledge.body", "NUMB_LOWER", g_NUMB_LOWER, s_NUMB_LOWER);
    pyrt::xrefs::reg_access("knowledge.body", "NUMB_SHOE", g_NUMB_SHOE, s_NUMB_SHOE);
    pyrt::xrefs::reg_access("knowledge.body", "OBJECT_OWNERS", g_OBJECT_OWNERS, s_OBJECT_OWNERS);
    pyrt::xrefs::reg_access("knowledge.body", "ON_YOUR_FEET", g_ON_YOUR_FEET, s_ON_YOUR_FEET);
    pyrt::xrefs::reg_access("knowledge.body", "OTHER_PEOPLE", g_OTHER_PEOPLE, s_OTHER_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.body", "PAINKILLER_NAMES", g_PAINKILLER_NAMES, s_PAINKILLER_NAMES);
    pyrt::xrefs::reg_access("knowledge.body", "PART_ONE", g_PART_ONE, s_PART_ONE);
    pyrt::xrefs::reg_access("knowledge.body", "PERSON_NOUNS", g_PERSON_NOUNS, s_PERSON_NOUNS);
    pyrt::xrefs::reg_access("knowledge.body", "POSSESSIVE_OF_A_PERSON", g_POSSESSIVE_OF_A_PERSON, s_POSSESSIVE_OF_A_PERSON);
    pyrt::xrefs::reg_access("knowledge.body", "PREGNANT", g_PREGNANT, s_PREGNANT);
    pyrt::xrefs::reg_access("knowledge.body", "RED_FLAG", g_RED_FLAG, s_RED_FLAG);
    pyrt::xrefs::reg_access("knowledge.body", "RED_FLAG_CARD", g_RED_FLAG_CARD, s_RED_FLAG_CARD);
    pyrt::xrefs::reg_access("knowledge.body", "REGIONS", g_REGIONS, s_REGIONS);
    pyrt::xrefs::reg_access("knowledge.body", "REGION_BIAS", g_REGION_BIAS, s_REGION_BIAS);
    pyrt::xrefs::reg_access("knowledge.body", "RING_AS_A_CALL", g_RING_AS_A_CALL, s_RING_AS_A_CALL);
    pyrt::xrefs::reg_access("knowledge.body", "RING_NOT_ON_A_FINGER", g_RING_NOT_ON_A_FINGER, s_RING_NOT_ON_A_FINGER);
    pyrt::xrefs::reg_access("knowledge.body", "SADDLE_CARD", g_SADDLE_CARD, s_SADDLE_CARD);
    pyrt::xrefs::reg_access("knowledge.body", "SADDLE_PARTS", g_SADDLE_PARTS, s_SADDLE_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "SEA_WATER_ALARM", g_SEA_WATER_ALARM, s_SEA_WATER_ALARM);
    pyrt::xrefs::reg_access("knowledge.body", "SEA_WATER_ALARM_REPLY", g_SEA_WATER_ALARM_REPLY, s_SEA_WATER_ALARM_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "SEEN_ABOUT_IT", g_SEEN_ABOUT_IT, s_SEEN_ABOUT_IT);
    pyrt::xrefs::reg_access("knowledge.body", "SEE_DOCTOR_AVOID", g_SEE_DOCTOR_AVOID, s_SEE_DOCTOR_AVOID);
    pyrt::xrefs::reg_access("knowledge.body", "SEE_DOCTOR_PARTS", g_SEE_DOCTOR_PARTS, s_SEE_DOCTOR_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "SEE_TODAY_BACK_AND_CALF", g_SEE_TODAY_BACK_AND_CALF, s_SEE_TODAY_BACK_AND_CALF);
    pyrt::xrefs::reg_access("knowledge.body", "SOFT_TISSUE", g_SOFT_TISSUE, s_SOFT_TISSUE);
    pyrt::xrefs::reg_access("knowledge.body", "SOMEWHERE_ELSE", g_SOMEWHERE_ELSE, s_SOMEWHERE_ELSE);
    pyrt::xrefs::reg_access("knowledge.body", "SPELLED_OUT", g_SPELLED_OUT, s_SPELLED_OUT);
    pyrt::xrefs::reg_access("knowledge.body", "STANDING", g_STANDING, s_STANDING);
    pyrt::xrefs::reg_access("knowledge.body", "TEMPERATURE_WORDS", g_TEMPERATURE_WORDS, s_TEMPERATURE_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "THING_AFTER_THE_WORD", g_THING_AFTER_THE_WORD, s_THING_AFTER_THE_WORD);
    pyrt::xrefs::reg_access("knowledge.body", "THING_PARTS", g_THING_PARTS, s_THING_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "THIRD_PARTY", g_THIRD_PARTY, s_THIRD_PARTY);
    pyrt::xrefs::reg_access("knowledge.body", "TIGHT_CHEST", g_TIGHT_CHEST, s_TIGHT_CHEST);
    pyrt::xrefs::reg_access("knowledge.body", "TOOTH_WORDS", g_TOOTH_WORDS, s_TOOTH_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "WEEKS", g_WEEKS, s_WEEKS);
    pyrt::xrefs::reg_access("knowledge.body", "WE_OUR_OURS", g_WE_OUR_OURS, s_WE_OUR_OURS);
    pyrt::xrefs::reg_access("knowledge.body", "_ALL_PART_WORDS", g__ALL_PART_WORDS, s__ALL_PART_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_ALSO_THE_BACK", g__ALSO_THE_BACK, s__ALSO_THE_BACK);
    pyrt::xrefs::reg_access("knowledge.body", "_AN_IMPLANT", g__AN_IMPLANT, s__AN_IMPLANT);
    pyrt::xrefs::reg_access("knowledge.body", "_ARTHRITIS_CURE", g__ARTHRITIS_CURE, s__ARTHRITIS_CURE);
    pyrt::xrefs::reg_access("knowledge.body", "_ARTHRITIS_TAIL", g__ARTHRITIS_TAIL, s__ARTHRITIS_TAIL);
    pyrt::xrefs::reg_access("knowledge.body", "_A_BECOMING_A_DENTIST", g__A_BECOMING_A_DENTIST, s__A_BECOMING_A_DENTIST);
    pyrt::xrefs::reg_access("knowledge.body", "_A_CHECK_UP", g__A_CHECK_UP, s__A_CHECK_UP);
    pyrt::xrefs::reg_access("knowledge.body", "_A_CROWN_VISIT", g__A_CROWN_VISIT, s__A_CROWN_VISIT);
    pyrt::xrefs::reg_access("knowledge.body", "_A_FILLING", g__A_FILLING, s__A_FILLING);
    pyrt::xrefs::reg_access("knowledge.body", "_A_HEALING_WORD", g__A_HEALING_WORD, s__A_HEALING_WORD);
    pyrt::xrefs::reg_access("knowledge.body", "_A_HYGIENIST", g__A_HYGIENIST, s__A_HYGIENIST);
    pyrt::xrefs::reg_access("knowledge.body", "_A_JOINT_OF_MEAT", g__A_JOINT_OF_MEAT, s__A_JOINT_OF_MEAT);
    pyrt::xrefs::reg_access("knowledge.body", "_A_LENGTH_OF_A_VISIT", g__A_LENGTH_OF_A_VISIT, s__A_LENGTH_OF_A_VISIT);
    pyrt::xrefs::reg_access("knowledge.body", "_A_LIST_THAT_IS_LONG", g__A_LIST_THAT_IS_LONG, s__A_LIST_THAT_IS_LONG);
    pyrt::xrefs::reg_access("knowledge.body", "_A_PART_IN_THE_TABLE", g__A_PART_IN_THE_TABLE, s__A_PART_IN_THE_TABLE);
    pyrt::xrefs::reg_access("knowledge.body", "_A_PART_NAMED", g__A_PART_NAMED, s__A_PART_NAMED);
    pyrt::xrefs::reg_access("knowledge.body", "_A_PRACTICE_ERRAND", g__A_PRACTICE_ERRAND, s__A_PRACTICE_ERRAND);
    pyrt::xrefs::reg_access("knowledge.body", "_A_REST_NOT_A_BONE", g__A_REST_NOT_A_BONE, s__A_REST_NOT_A_BONE);
    pyrt::xrefs::reg_access("knowledge.body", "_A_RETURN_TO", g__A_RETURN_TO, s__A_RETURN_TO);
    pyrt::xrefs::reg_access("knowledge.body", "_A_ROOT_CANAL", g__A_ROOT_CANAL, s__A_ROOT_CANAL);
    pyrt::xrefs::reg_access("knowledge.body", "_A_THING_THAT_IS_NOT_A_BODY", g__A_THING_THAT_IS_NOT_A_BODY, s__A_THING_THAT_IS_NOT_A_BODY);
    pyrt::xrefs::reg_access("knowledge.body", "_A_VERB_TOOK_THE_BACK", g__A_VERB_TOOK_THE_BACK, s__A_VERB_TOOK_THE_BACK);
    pyrt::xrefs::reg_access("knowledge.body", "_A_WHILE_AGO", g__A_WHILE_AGO, s__A_WHILE_AGO);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_AFTER_A_VERB", g__BACK_AFTER_A_VERB, s__BACK_AFTER_A_VERB);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_BEFORE_A_PLACE", g__BACK_BEFORE_A_PLACE, s__BACK_BEFORE_A_PLACE);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_CLAUSE_JOIN", g__BACK_CLAUSE_JOIN, s__BACK_CLAUSE_JOIN);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_DESCRIBED", g__BACK_DESCRIBED, s__BACK_DESCRIBED);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_HOME", g__BACK_HOME, s__BACK_HOME);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_IS_A_NOUN_AFTER", g__BACK_IS_A_NOUN_AFTER, s__BACK_IS_A_NOUN_AFTER);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_IS_A_RETURN", g__BACK_IS_A_RETURN, s__BACK_IS_A_RETURN);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_TAKING_A_THING", g__BACK_TAKING_A_THING, s__BACK_TAKING_A_THING);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_THEIRS", g__BACK_THEIRS, s__BACK_THEIRS);
    pyrt::xrefs::reg_access("knowledge.body", "_BACK_VERB_BEFORE", g__BACK_VERB_BEFORE, s__BACK_VERB_BEFORE);
    pyrt::xrefs::reg_access("knowledge.body", "_BAD_PART_MOVEMENT", g__BAD_PART_MOVEMENT, s__BAD_PART_MOVEMENT);
    pyrt::xrefs::reg_access("knowledge.body", "_BAD_PART_WORDS", g__BAD_PART_WORDS, s__BAD_PART_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_BEARD_TRIM_REPLY", g__BEARD_TRIM_REPLY, s__BEARD_TRIM_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "_BEARD_TRIM_WORDS", g__BEARD_TRIM_WORDS, s__BEARD_TRIM_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_BLADE_WORDS", g__BLADE_WORDS, s__BLADE_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_CANNOT_USE_IT", g__CANNOT_USE_IT, s__CANNOT_USE_IT);
    pyrt::xrefs::reg_access("knowledge.body", "_CARRIES_WEIGHT", g__CARRIES_WEIGHT, s__CARRIES_WEIGHT);
    pyrt::xrefs::reg_access("knowledge.body", "_CARTILAGE_PIERCING", g__CARTILAGE_PIERCING, s__CARTILAGE_PIERCING);
    pyrt::xrefs::reg_access("knowledge.body", "_CHECK_HOW_TO", g__CHECK_HOW_TO, s__CHECK_HOW_TO);
    pyrt::xrefs::reg_access("knowledge.body", "_CHICKENPOX_TIMELINE", g__CHICKENPOX_TIMELINE, s__CHICKENPOX_TIMELINE);
    pyrt::xrefs::reg_access("knowledge.body", "_DECISION_ASK", g__DECISION_ASK, s__DECISION_ASK);
    pyrt::xrefs::reg_access("knowledge.body", "_DECISION_WORDS", g__DECISION_WORDS, s__DECISION_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_DETERMINERS", g__DETERMINERS, s__DETERMINERS);
    pyrt::xrefs::reg_access("knowledge.body", "_DRUGS", g__DRUGS, s__DRUGS);
    pyrt::xrefs::reg_access("knowledge.body", "_FALL_AVOID", g__FALL_AVOID, s__FALL_AVOID);
    pyrt::xrefs::reg_access("knowledge.body", "_FEVER_ADVICE", g__FEVER_ADVICE, s__FEVER_ADVICE);
    pyrt::xrefs::reg_access("knowledge.body", "_GONE_BACK", g__GONE_BACK, s__GONE_BACK);
    pyrt::xrefs::reg_access("knowledge.body", "_HEAL_PARTS", g__HEAL_PARTS, s__HEAL_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "_HEAL_TIME", g__HEAL_TIME, s__HEAL_TIME);
    pyrt::xrefs::reg_access("knowledge.body", "_HEAT_OR_ICE_THE_THING", g__HEAT_OR_ICE_THE_THING, s__HEAT_OR_ICE_THE_THING);
    pyrt::xrefs::reg_access("knowledge.body", "_IT_HAPPENED_TO", g__IT_HAPPENED_TO, s__IT_HAPPENED_TO);
    pyrt::xrefs::reg_access("knowledge.body", "_KEPT_AWAKE", g__KEPT_AWAKE, s__KEPT_AWAKE);
    pyrt::xrefs::reg_access("knowledge.body", "_KIDNEY_WORDS", g__KIDNEY_WORDS, s__KIDNEY_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_MOLE_CHANGED_TEXT", g__MOLE_CHANGED_TEXT, s__MOLE_CHANGED_TEXT);
    pyrt::xrefs::reg_access("knowledge.body", "_MOLE_CLEAR", g__MOLE_CLEAR, s__MOLE_CLEAR);
    pyrt::xrefs::reg_access("knowledge.body", "_MOLE_ORDINARY_TEXT", g__MOLE_ORDINARY_TEXT, s__MOLE_ORDINARY_TEXT);
    pyrt::xrefs::reg_access("knowledge.body", "_MOLE_REMOVE_TEXT", g__MOLE_REMOVE_TEXT, s__MOLE_REMOVE_TEXT);
    pyrt::xrefs::reg_access("knowledge.body", "_MUSCLE_PARTS", g__MUSCLE_PARTS, s__MUSCLE_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "_NEW_SUBJECT", g__NEW_SUBJECT, s__NEW_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.body", "_NOT_AN_NSAID", g__NOT_AN_NSAID, s__NOT_AN_NSAID);
    pyrt::xrefs::reg_access("knowledge.body", "_NOT_A_BODY_PART", g__NOT_A_BODY_PART, s__NOT_A_BODY_PART);
    pyrt::xrefs::reg_access("knowledge.body", "_NOT_A_TAMPON_LINE", g__NOT_A_TAMPON_LINE, s__NOT_A_TAMPON_LINE);
    pyrt::xrefs::reg_access("knowledge.body", "_NOT_ON_A_BODY", g__NOT_ON_A_BODY, s__NOT_ON_A_BODY);
    pyrt::xrefs::reg_access("knowledge.body", "_NOT_THE_TABLET", g__NOT_THE_TABLET, s__NOT_THE_TABLET);
    pyrt::xrefs::reg_access("knowledge.body", "_OFF_A_THING", g__OFF_A_THING, s__OFF_A_THING);
    pyrt::xrefs::reg_access("knowledge.body", "_ON_A_BODY", g__ON_A_BODY, s__ON_A_BODY);
    pyrt::xrefs::reg_access("knowledge.body", "_OTHER_PEOPLE", g__OTHER_PEOPLE, s__OTHER_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.body", "_PARTS", g__PARTS, s__PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "_PERSON_OWNER", g__PERSON_OWNER, s__PERSON_OWNER);
    pyrt::xrefs::reg_access("knowledge.body", "_PIERCING_BUMP", g__PIERCING_BUMP, s__PIERCING_BUMP);
    pyrt::xrefs::reg_access("knowledge.body", "_PIERCING_NOT_TROUBLE", g__PIERCING_NOT_TROUBLE, s__PIERCING_NOT_TROUBLE);
    pyrt::xrefs::reg_access("knowledge.body", "_PIERCING_PIECE", g__PIERCING_PIECE, s__PIERCING_PIECE);
    pyrt::xrefs::reg_access("knowledge.body", "_PIERCING_ROUTINE", g__PIERCING_ROUTINE, s__PIERCING_ROUTINE);
    pyrt::xrefs::reg_access("knowledge.body", "_PIERCING_TROUBLE", g__PIERCING_TROUBLE, s__PIERCING_TROUBLE);
    pyrt::xrefs::reg_access("knowledge.body", "_POSSESSIVES", g__POSSESSIVES, s__POSSESSIVES);
    pyrt::xrefs::reg_access("knowledge.body", "_PREGNANCY_SWELLING", g__PREGNANCY_SWELLING, s__PREGNANCY_SWELLING);
    pyrt::xrefs::reg_access("knowledge.body", "_PREGNANCY_SWELLING_AVOID", g__PREGNANCY_SWELLING_AVOID, s__PREGNANCY_SWELLING_AVOID);
    pyrt::xrefs::reg_access("knowledge.body", "_PREGNANCY_SWELLING_PARTS", g__PREGNANCY_SWELLING_PARTS, s__PREGNANCY_SWELLING_PARTS);
    pyrt::xrefs::reg_access("knowledge.body", "_PREGNANCY_SWELLING_REPLY", g__PREGNANCY_SWELLING_REPLY, s__PREGNANCY_SWELLING_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "_PREGNANCY_WORDS", g__PREGNANCY_WORDS, s__PREGNANCY_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_PREGNANT_WORDS", g__PREGNANT_WORDS, s__PREGNANT_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_PROTECT_ALLOWED", g__PROTECT_ALLOWED, s__PROTECT_ALLOWED);
    pyrt::xrefs::reg_access("knowledge.body", "_PROTECT_ASK", g__PROTECT_ASK, s__PROTECT_ASK);
    pyrt::xrefs::reg_access("knowledge.body", "_PROTECT_FILLERS", g__PROTECT_FILLERS, s__PROTECT_FILLERS);
    pyrt::xrefs::reg_access("knowledge.body", "_RAZOR_BLADE_REPLY", g__RAZOR_BLADE_REPLY, s__RAZOR_BLADE_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "_RAZOR_BURN_REPLY", g__RAZOR_BURN_REPLY, s__RAZOR_BURN_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "_REGISTERING_A_DENTIST", g__REGISTERING_A_DENTIST, s__REGISTERING_A_DENTIST);
    pyrt::xrefs::reg_access("knowledge.body", "_REPORTED_COMPLAINT", g__REPORTED_COMPLAINT, s__REPORTED_COMPLAINT);
    pyrt::xrefs::reg_access("knowledge.body", "_SHAVE_BURN_WORDS", g__SHAVE_BURN_WORDS, s__SHAVE_BURN_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_SHAVE_FIRST_REPLY", g__SHAVE_FIRST_REPLY, s__SHAVE_FIRST_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "_SOMEONE_ELSE", g__SOMEONE_ELSE, s__SOMEONE_ELSE);
    pyrt::xrefs::reg_access("knowledge.body", "_SORENESS_WORDS", g__SORENESS_WORDS, s__SORENESS_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_SORE_ANYWHERE", g__SORE_ANYWHERE, s__SORE_ANYWHERE);
    pyrt::xrefs::reg_access("knowledge.body", "_SPORT_WORDS", g__SPORT_WORDS, s__SPORT_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_STANDING_KNEE", g__STANDING_KNEE, s__STANDING_KNEE);
    pyrt::xrefs::reg_access("knowledge.body", "_STANDING_TROUBLE", g__STANDING_TROUBLE, s__STANDING_TROUBLE);
    pyrt::xrefs::reg_access("knowledge.body", "_STRAIGHT_SHAVE_REPLY", g__STRAIGHT_SHAVE_REPLY, s__STRAIGHT_SHAVE_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "_SWELLING", g__SWELLING, s__SWELLING);
    pyrt::xrefs::reg_access("knowledge.body", "_TAKE_A_TEMPERATURE", g__TAKE_A_TEMPERATURE, s__TAKE_A_TEMPERATURE);
    pyrt::xrefs::reg_access("knowledge.body", "_TAMPON_WORD", g__TAMPON_WORD, s__TAMPON_WORD);
    pyrt::xrefs::reg_access("knowledge.body", "_THE_BACK_IS_THEIRS", g__THE_BACK_IS_THEIRS, s__THE_BACK_IS_THEIRS);
    pyrt::xrefs::reg_access("knowledge.body", "_THE_BOOKING", g__THE_BOOKING, s__THE_BOOKING);
    pyrt::xrefs::reg_access("knowledge.body", "_THE_BOOKING_WORDS", g__THE_BOOKING_WORDS, s__THE_BOOKING_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_THE_PERSONS_OWN_WORDS", g__THE_PERSONS_OWN_WORDS, s__THE_PERSONS_OWN_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_THE_QUEUE", g__THE_QUEUE, s__THE_QUEUE);
    pyrt::xrefs::reg_access("knowledge.body", "_THE_VISIT", g__THE_VISIT, s__THE_VISIT);
    pyrt::xrefs::reg_access("knowledge.body", "_THE_VISIT_IN_ORDER", g__THE_VISIT_IN_ORDER, s__THE_VISIT_IN_ORDER);
    pyrt::xrefs::reg_access("knowledge.body", "_TO_HURT_A_PART", g__TO_HURT_A_PART, s__TO_HURT_A_PART);
    pyrt::xrefs::reg_access("knowledge.body", "_WAX_FIRST_REPLY", g__WAX_FIRST_REPLY, s__WAX_FIRST_REPLY);
    pyrt::xrefs::reg_access("knowledge.body", "_WAX_WORDS", g__WAX_WORDS, s__WAX_WORDS);
    pyrt::xrefs::reg_access("knowledge.body", "_WEEKS_ALONG", g__WEEKS_ALONG, s__WEEKS_ALONG);
    pyrt::xrefs::reg_access("knowledge.body", "_WEEKS_DRILL", g__WEEKS_DRILL, s__WEEKS_DRILL);
    pyrt::xrefs::reg_access("knowledge.body", "_WORKSHOP", g__WORKSHOP, s__WORKSHOP);
    pyrt::xrefs::reg_access("knowledge.body", "_WORKSHOP_NOUNS", g__WORKSHOP_NOUNS, s__WORKSHOP_NOUNS);
    pyrt::xrefs::reg_access("knowledge.body", "entry", g_entry, s_entry);
}
