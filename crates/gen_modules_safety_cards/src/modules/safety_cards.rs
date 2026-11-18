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


thread_local! { static G_ALLERGY_AIRWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALLERGY_FACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALLERGY_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALLERGY_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALLERGY_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALLERGY_SIGN_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TOOTH_DAMAGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TOOTH_OWNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAT_BITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BITE_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLEEDING_SMALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREATH_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURNT_BODY_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURN_ON_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COME_OFF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTERFACTUAL_IDIOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTERFACTUAL_WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRUSH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRUSH_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRUSH_HEAVY_ON_A_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRUSH_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRUSH_ON_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_BODY_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_ON_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_ON_THROAT_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOG_ATTACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EFFORT_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EFFORT_DIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EFFORT_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EFFORT_NOT_AN_ACCIDENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRATER_CUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAD_OUT_COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAD_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAL_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAT_BREATH_RED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAT_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAT_NOT_WEATHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIT_IN_FUN_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT_COMFORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT_SURFACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT_WAX_ON_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HUMAN_BITTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HUMAN_OWN_INJURY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HYPERBOLE_WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HYPO_SIGN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INJURY_HOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSECT_BITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSECT_BITE_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSECT_BITE_NEEDS_A_LOOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGH_BREATH_RED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGH_OTHER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGH_TAKES_BREATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUGH_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMB_IS_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMP_ONGOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMP_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIPS_SWELL_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_OWN_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEAR_HIT_ON_A_ROAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEAR_HIT_ON_A_ROAD_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PET_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_BURN_ON_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_JUST_A_NEAR_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_OFF_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_OFF_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_HANGING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_ANIMAL_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_APPOSITIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_APPOSITIVE_SUB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_BREATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_BREATH_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_CANT_SETTLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_EFFORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_NAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_OWNS_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_OWNS_THE_WOUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_PASSES_BLOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_WEAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_WOUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_PLACE_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RING_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCALD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCORCHED_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_HARM_HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_HARM_REPEAT_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOULD_I_CALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGHT_EVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGHT_LIFE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRUCK_BY_A_VEHICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRUCK_BY_A_VEHICLE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRUCK_MY_BIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRUCK_NOT_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUGAR_READING_LOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TICK_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TICK_BITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOOTH_CHIPPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOOTH_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATER_DANGER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAX_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_JOINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WILD_BITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LOW_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_READING_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_APPOSITIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_DAMAGE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.safety_cards: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ALLERGY_AIRWAY() -> Py { ensure_init(); G_ALLERGY_AIRWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_AIRWAY(v: Py) { ensure_init(); G_ALLERGY_AIRWAY.with(|c| *c.borrow_mut() = v); }
pub fn g_ALLERGY_FACE() -> Py { ensure_init(); G_ALLERGY_FACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_FACE(v: Py) { ensure_init(); G_ALLERGY_FACE.with(|c| *c.borrow_mut() = v); }
pub fn g_ALLERGY_FOOD() -> Py { ensure_init(); G_ALLERGY_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_FOOD(v: Py) { ensure_init(); G_ALLERGY_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_ALLERGY_NOW() -> Py { ensure_init(); G_ALLERGY_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_NOW(v: Py) { ensure_init(); G_ALLERGY_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_ALLERGY_OFF() -> Py { ensure_init(); G_ALLERGY_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_OFF(v: Py) { ensure_init(); G_ALLERGY_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_ALLERGY_SIGN_NAMED() -> Py { ensure_init(); G_ALLERGY_SIGN_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_SIGN_NAMED(v: Py) { ensure_init(); G_ALLERGY_SIGN_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TOOTH_DAMAGED() -> Py { ensure_init(); G_A_TOOTH_DAMAGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TOOTH_DAMAGED(v: Py) { ensure_init(); G_A_TOOTH_DAMAGED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TOOTH_OWNER() -> Py { ensure_init(); G_A_TOOTH_OWNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TOOTH_OWNER(v: Py) { ensure_init(); G_A_TOOTH_OWNER.with(|c| *c.borrow_mut() = v); }
pub fn g_BAT_BITE() -> Py { ensure_init(); G_BAT_BITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAT_BITE(v: Py) { ensure_init(); G_BAT_BITE.with(|c| *c.borrow_mut() = v); }
pub fn g_BITE_DONE() -> Py { ensure_init(); G_BITE_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BITE_DONE(v: Py) { ensure_init(); G_BITE_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_BLEEDING_SMALL() -> Py { ensure_init(); G_BLEEDING_SMALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLEEDING_SMALL(v: Py) { ensure_init(); G_BLEEDING_SMALL.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY() -> Py { ensure_init(); G_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); G_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_BREATH_BARE() -> Py { ensure_init(); G_BREATH_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREATH_BARE(v: Py) { ensure_init(); G_BREATH_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_BURNT_BODY_VERB() -> Py { ensure_init(); G_BURNT_BODY_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURNT_BODY_VERB(v: Py) { ensure_init(); G_BURNT_BODY_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_BURN_ON_BODY() -> Py { ensure_init(); G_BURN_ON_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURN_ON_BODY(v: Py) { ensure_init(); G_BURN_ON_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_BURN_VERB() -> Py { ensure_init(); G_BURN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURN_VERB(v: Py) { ensure_init(); G_BURN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_COME_OFF_IT() -> Py { ensure_init(); G_COME_OFF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COME_OFF_IT(v: Py) { ensure_init(); G_COME_OFF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTERFACTUAL_IDIOM() -> Py { ensure_init(); G_COUNTERFACTUAL_IDIOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTERFACTUAL_IDIOM(v: Py) { ensure_init(); G_COUNTERFACTUAL_IDIOM.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTERFACTUAL_WISH() -> Py { ensure_init(); G_COUNTERFACTUAL_WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTERFACTUAL_WISH(v: Py) { ensure_init(); G_COUNTERFACTUAL_WISH.with(|c| *c.borrow_mut() = v); }
pub fn g_CRUSH() -> Py { ensure_init(); G_CRUSH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRUSH(v: Py) { ensure_init(); G_CRUSH.with(|c| *c.borrow_mut() = v); }
pub fn g_CRUSH_HARD() -> Py { ensure_init(); G_CRUSH_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRUSH_HARD(v: Py) { ensure_init(); G_CRUSH_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g_CRUSH_HEAVY_ON_A_PRONOUN() -> Py { ensure_init(); G_CRUSH_HEAVY_ON_A_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRUSH_HEAVY_ON_A_PRONOUN(v: Py) { ensure_init(); G_CRUSH_HEAVY_ON_A_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_CRUSH_OFF() -> Py { ensure_init(); G_CRUSH_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRUSH_OFF(v: Py) { ensure_init(); G_CRUSH_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_CRUSH_ON_A_PERSON() -> Py { ensure_init(); G_CRUSH_ON_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRUSH_ON_A_PERSON(v: Py) { ensure_init(); G_CRUSH_ON_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_BODY_PART() -> Py { ensure_init(); G_CUT_BODY_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_BODY_PART(v: Py) { ensure_init(); G_CUT_BODY_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_ON_BODY() -> Py { ensure_init(); G_CUT_ON_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_ON_BODY(v: Py) { ensure_init(); G_CUT_ON_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_ON_THROAT_BODY() -> Py { ensure_init(); G_CUT_ON_THROAT_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_ON_THROAT_BODY(v: Py) { ensure_init(); G_CUT_ON_THROAT_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_DOG_ATTACK() -> Py { ensure_init(); G_DOG_ATTACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOG_ATTACK(v: Py) { ensure_init(); G_DOG_ATTACK.with(|c| *c.borrow_mut() = v); }
pub fn g_EFFORT_BEFORE() -> Py { ensure_init(); G_EFFORT_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EFFORT_BEFORE(v: Py) { ensure_init(); G_EFFORT_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_EFFORT_DIED() -> Py { ensure_init(); G_EFFORT_DIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EFFORT_DIED(v: Py) { ensure_init(); G_EFFORT_DIED.with(|c| *c.borrow_mut() = v); }
pub fn g_EFFORT_DOING() -> Py { ensure_init(); G_EFFORT_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EFFORT_DOING(v: Py) { ensure_init(); G_EFFORT_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_EFFORT_NOT_AN_ACCIDENT() -> Py { ensure_init(); G_EFFORT_NOT_AN_ACCIDENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EFFORT_NOT_AN_ACCIDENT(v: Py) { ensure_init(); G_EFFORT_NOT_AN_ACCIDENT.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP() -> Py { ensure_init(); G_FOLLOW_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP(v: Py) { ensure_init(); G_FOLLOW_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_WORDS() -> Py { ensure_init(); G_FOOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_WORDS(v: Py) { ensure_init(); G_FOOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GRATER_CUT() -> Py { ensure_init(); G_GRATER_CUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRATER_CUT(v: Py) { ensure_init(); G_GRATER_CUT.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAD_OUT_COLD() -> Py { ensure_init(); G_HEAD_OUT_COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAD_OUT_COLD(v: Py) { ensure_init(); G_HEAD_OUT_COLD.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAD_REPLIES() -> Py { ensure_init(); G_HEAD_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAD_REPLIES(v: Py) { ensure_init(); G_HEAD_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAL_TIME() -> Py { ensure_init(); G_HEAL_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAL_TIME(v: Py) { ensure_init(); G_HEAL_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAT_BREATH_RED() -> Py { ensure_init(); G_HEAT_BREATH_RED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAT_BREATH_RED(v: Py) { ensure_init(); G_HEAT_BREATH_RED.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAT_CAUSE() -> Py { ensure_init(); G_HEAT_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAT_CAUSE(v: Py) { ensure_init(); G_HEAT_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAT_NOT_WEATHER() -> Py { ensure_init(); G_HEAT_NOT_WEATHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAT_NOT_WEATHER(v: Py) { ensure_init(); G_HEAT_NOT_WEATHER.with(|c| *c.borrow_mut() = v); }
pub fn g_HIT_IN_FUN_HARD() -> Py { ensure_init(); G_HIT_IN_FUN_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIT_IN_FUN_HARD(v: Py) { ensure_init(); G_HIT_IN_FUN_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT_COMFORT() -> Py { ensure_init(); G_HOT_COMFORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT_COMFORT(v: Py) { ensure_init(); G_HOT_COMFORT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT_SURFACE() -> Py { ensure_init(); G_HOT_SURFACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT_SURFACE(v: Py) { ensure_init(); G_HOT_SURFACE.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT_WAX_ON_BODY() -> Py { ensure_init(); G_HOT_WAX_ON_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT_WAX_ON_BODY(v: Py) { ensure_init(); G_HOT_WAX_ON_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_HUMAN_BITTEN() -> Py { ensure_init(); G_HUMAN_BITTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HUMAN_BITTEN(v: Py) { ensure_init(); G_HUMAN_BITTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_HUMAN_OWN_INJURY() -> Py { ensure_init(); G_HUMAN_OWN_INJURY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HUMAN_OWN_INJURY(v: Py) { ensure_init(); G_HUMAN_OWN_INJURY.with(|c| *c.borrow_mut() = v); }
pub fn g_HYPERBOLE_WISH() -> Py { ensure_init(); G_HYPERBOLE_WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HYPERBOLE_WISH(v: Py) { ensure_init(); G_HYPERBOLE_WISH.with(|c| *c.borrow_mut() = v); }
pub fn g_HYPO_SIGN() -> Py { ensure_init(); G_HYPO_SIGN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HYPO_SIGN(v: Py) { ensure_init(); G_HYPO_SIGN.with(|c| *c.borrow_mut() = v); }
pub fn g_INJURY_HOW() -> Py { ensure_init(); G_INJURY_HOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INJURY_HOW(v: Py) { ensure_init(); G_INJURY_HOW.with(|c| *c.borrow_mut() = v); }
pub fn g_INSECT_BITE() -> Py { ensure_init(); G_INSECT_BITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSECT_BITE(v: Py) { ensure_init(); G_INSECT_BITE.with(|c| *c.borrow_mut() = v); }
pub fn g_INSECT_BITE_BODY() -> Py { ensure_init(); G_INSECT_BITE_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSECT_BITE_BODY(v: Py) { ensure_init(); G_INSECT_BITE_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_INSECT_BITE_NEEDS_A_LOOK() -> Py { ensure_init(); G_INSECT_BITE_NEEDS_A_LOOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSECT_BITE_NEEDS_A_LOOK(v: Py) { ensure_init(); G_INSECT_BITE_NEEDS_A_LOOK.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGH_BREATH_RED() -> Py { ensure_init(); G_LAUGH_BREATH_RED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGH_BREATH_RED(v: Py) { ensure_init(); G_LAUGH_BREATH_RED.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGH_OTHER_PERSON() -> Py { ensure_init(); G_LAUGH_OTHER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGH_OTHER_PERSON(v: Py) { ensure_init(); G_LAUGH_OTHER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGH_TAKES_BREATH() -> Py { ensure_init(); G_LAUGH_TAKES_BREATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGH_TAKES_BREATH(v: Py) { ensure_init(); G_LAUGH_TAKES_BREATH.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUGH_WORD() -> Py { ensure_init(); G_LAUGH_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUGH_WORD(v: Py) { ensure_init(); G_LAUGH_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMB_IS_A_THING() -> Py { ensure_init(); G_LIMB_IS_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMB_IS_A_THING(v: Py) { ensure_init(); G_LIMB_IS_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMP_ONGOING() -> Py { ensure_init(); G_LIMP_ONGOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMP_ONGOING(v: Py) { ensure_init(); G_LIMP_ONGOING.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMP_PERSON() -> Py { ensure_init(); G_LIMP_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMP_PERSON(v: Py) { ensure_init(); G_LIMP_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_LIPS_SWELL_BARE() -> Py { ensure_init(); G_LIPS_SWELL_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIPS_SWELL_BARE(v: Py) { ensure_init(); G_LIPS_SWELL_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_MILD() -> Py { ensure_init(); G_MILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MILD(v: Py) { ensure_init(); G_MILD.with(|c| *c.borrow_mut() = v); }
pub fn g_MINOR() -> Py { ensure_init(); G_MINOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINOR(v: Py) { ensure_init(); G_MINOR.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_OWN_PART() -> Py { ensure_init(); G_MY_OWN_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_OWN_PART(v: Py) { ensure_init(); G_MY_OWN_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_NEAR_HIT_ON_A_ROAD() -> Py { ensure_init(); G_NEAR_HIT_ON_A_ROAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEAR_HIT_ON_A_ROAD(v: Py) { ensure_init(); G_NEAR_HIT_ON_A_ROAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NEAR_HIT_ON_A_ROAD_PLACE() -> Py { ensure_init(); G_NEAR_HIT_ON_A_ROAD_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEAR_HIT_ON_A_ROAD_PLACE(v: Py) { ensure_init(); G_NEAR_HIT_ON_A_ROAD_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PET_LINE() -> Py { ensure_init(); G_NOT_A_PET_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PET_LINE(v: Py) { ensure_init(); G_NOT_A_PET_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_BURN_ON_BODY() -> Py { ensure_init(); G_NOT_BURN_ON_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_BURN_ON_BODY(v: Py) { ensure_init(); G_NOT_BURN_ON_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_JUST_A_NEAR_THING() -> Py { ensure_init(); G_NOT_JUST_A_NEAR_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_JUST_A_NEAR_THING(v: Py) { ensure_init(); G_NOT_JUST_A_NEAR_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_OFF() -> Py { ensure_init(); G_PART_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_OFF(v: Py) { ensure_init(); G_PART_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_OFF_NOT() -> Py { ensure_init(); G_PART_OFF_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_OFF_NOT(v: Py) { ensure_init(); G_PART_OFF_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_OFF_PART() -> Py { ensure_init(); G_PART_OFF_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_OFF_PART(v: Py) { ensure_init(); G_PART_OFF_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_HANGING() -> Py { ensure_init(); G_PERSON_HANGING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_HANGING(v: Py) { ensure_init(); G_PERSON_HANGING.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_ANIMAL_HURT() -> Py { ensure_init(); G_PET_ANIMAL_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_ANIMAL_HURT(v: Py) { ensure_init(); G_PET_ANIMAL_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_APPOSITIVE() -> Py { ensure_init(); G_PET_APPOSITIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_APPOSITIVE(v: Py) { ensure_init(); G_PET_APPOSITIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_APPOSITIVE_SUB() -> Py { ensure_init(); G_PET_APPOSITIVE_SUB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_APPOSITIVE_SUB(v: Py) { ensure_init(); G_PET_APPOSITIVE_SUB.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_BREATH() -> Py { ensure_init(); G_PET_BREATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_BREATH(v: Py) { ensure_init(); G_PET_BREATH.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_BREATH_NOW() -> Py { ensure_init(); G_PET_BREATH_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_BREATH_NOW(v: Py) { ensure_init(); G_PET_BREATH_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_CANT_SETTLE() -> Py { ensure_init(); G_PET_CANT_SETTLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_CANT_SETTLE(v: Py) { ensure_init(); G_PET_CANT_SETTLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_EFFORT() -> Py { ensure_init(); G_PET_EFFORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_EFFORT(v: Py) { ensure_init(); G_PET_EFFORT.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_NAIL() -> Py { ensure_init(); G_PET_NAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_NAIL(v: Py) { ensure_init(); G_PET_NAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_OWNS_A_THING() -> Py { ensure_init(); G_PET_OWNS_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_OWNS_A_THING(v: Py) { ensure_init(); G_PET_OWNS_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_OWNS_THE_WOUND() -> Py { ensure_init(); G_PET_OWNS_THE_WOUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_OWNS_THE_WOUND(v: Py) { ensure_init(); G_PET_OWNS_THE_WOUND.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_PASSES_BLOOD() -> Py { ensure_init(); G_PET_PASSES_BLOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_PASSES_BLOOD(v: Py) { ensure_init(); G_PET_PASSES_BLOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_WEAK() -> Py { ensure_init(); G_PET_WEAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_WEAK(v: Py) { ensure_init(); G_PET_WEAK.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_WOUND() -> Py { ensure_init(); G_PET_WOUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_WOUND(v: Py) { ensure_init(); G_PET_WOUND.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_FINE() -> Py { ensure_init(); G_PURSUIT_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_FINE(v: Py) { ensure_init(); G_PURSUIT_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_PLACE_HOME() -> Py { ensure_init(); G_PURSUIT_PLACE_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_PLACE_HOME(v: Py) { ensure_init(); G_PURSUIT_PLACE_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_RING_STUCK() -> Py { ensure_init(); G_RING_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RING_STUCK(v: Py) { ensure_init(); G_RING_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_SCALD() -> Py { ensure_init(); G_SCALD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCALD(v: Py) { ensure_init(); G_SCALD.with(|c| *c.borrow_mut() = v); }
pub fn g_SCORCHED_A_PERSON() -> Py { ensure_init(); G_SCORCHED_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCORCHED_A_PERSON(v: Py) { ensure_init(); G_SCORCHED_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_HARM_HABIT() -> Py { ensure_init(); G_SELF_HARM_HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_HABIT(v: Py) { ensure_init(); G_SELF_HARM_HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_HARM_REPEAT_CAUSE() -> Py { ensure_init(); G_SELF_HARM_REPEAT_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_REPEAT_CAUSE(v: Py) { ensure_init(); G_SELF_HARM_REPEAT_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOULD_I_CALL() -> Py { ensure_init(); G_SHOULD_I_CALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOULD_I_CALL(v: Py) { ensure_init(); G_SHOULD_I_CALL.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGHT_EVER() -> Py { ensure_init(); G_SIGHT_EVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGHT_EVER(v: Py) { ensure_init(); G_SIGHT_EVER.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGHT_LIFE_REPLIES() -> Py { ensure_init(); G_SIGHT_LIFE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGHT_LIFE_REPLIES(v: Py) { ensure_init(); G_SIGHT_LIFE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_STING_WORDS() -> Py { ensure_init(); G_STING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STING_WORDS(v: Py) { ensure_init(); G_STING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STRUCK_BY_A_VEHICLE() -> Py { ensure_init(); G_STRUCK_BY_A_VEHICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRUCK_BY_A_VEHICLE(v: Py) { ensure_init(); G_STRUCK_BY_A_VEHICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_STRUCK_BY_A_VEHICLE_NOT() -> Py { ensure_init(); G_STRUCK_BY_A_VEHICLE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRUCK_BY_A_VEHICLE_NOT(v: Py) { ensure_init(); G_STRUCK_BY_A_VEHICLE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_STRUCK_MY_BIKE() -> Py { ensure_init(); G_STRUCK_MY_BIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRUCK_MY_BIKE(v: Py) { ensure_init(); G_STRUCK_MY_BIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_STRUCK_NOT_NOW() -> Py { ensure_init(); G_STRUCK_NOT_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRUCK_NOT_NOW(v: Py) { ensure_init(); G_STRUCK_NOT_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_SUGAR_READING_LOW() -> Py { ensure_init(); G_SUGAR_READING_LOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUGAR_READING_LOW(v: Py) { ensure_init(); G_SUGAR_READING_LOW.with(|c| *c.borrow_mut() = v); }
pub fn g_TICK_ALREADY() -> Py { ensure_init(); G_TICK_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TICK_ALREADY(v: Py) { ensure_init(); G_TICK_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_TICK_BITE() -> Py { ensure_init(); G_TICK_BITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TICK_BITE(v: Py) { ensure_init(); G_TICK_BITE.with(|c| *c.borrow_mut() = v); }
pub fn g_TOOTH_CHIPPED() -> Py { ensure_init(); G_TOOTH_CHIPPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOOTH_CHIPPED(v: Py) { ensure_init(); G_TOOTH_CHIPPED.with(|c| *c.borrow_mut() = v); }
pub fn g_TOOTH_OUT() -> Py { ensure_init(); G_TOOTH_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOOTH_OUT(v: Py) { ensure_init(); G_TOOTH_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER_DANGER() -> Py { ensure_init(); G_WATER_DANGER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER_DANGER(v: Py) { ensure_init(); G_WATER_DANGER.with(|c| *c.borrow_mut() = v); }
pub fn g_WAX_BODY() -> Py { ensure_init(); G_WAX_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAX_BODY(v: Py) { ensure_init(); G_WAX_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_JOINT() -> Py { ensure_init(); G_WEIGHT_JOINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_JOINT(v: Py) { ensure_init(); G_WEIGHT_JOINT.with(|c| *c.borrow_mut() = v); }
pub fn g_WILD_BITE() -> Py { ensure_init(); G_WILD_BITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WILD_BITE(v: Py) { ensure_init(); G_WILD_BITE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LOW_FIGURE() -> Py { ensure_init(); G__A_LOW_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LOW_FIGURE(v: Py) { ensure_init(); G__A_LOW_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_READING_NOUN() -> Py { ensure_init(); G__A_READING_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_READING_NOUN(v: Py) { ensure_init(); G__A_READING_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_APPOSITIVE() -> Py { ensure_init(); G__PET_APPOSITIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_APPOSITIVE(v: Py) { ensure_init(); G__PET_APPOSITIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_DAMAGE_THING() -> Py { ensure_init(); G__PET_DAMAGE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_DAMAGE_THING(v: Py) { ensure_init(); G__PET_DAMAGE_THING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Text and pattern tables that only `safety.py` reads.\n\n`bot/modules/safety.py` is at the 1 MiB file limit the Hub puts on one file, and this is\nthe cheapest room to make: these are literal tables with no reference to anything else\nin the module, so they move without changing a single reply. `safety.py` imports them\nback, and every name is still reachable as `safety.<NAME>`.\n");
    fn __data1() -> Py { py_tuple(vec![py_str("\\b(?:dog|dogs|puppy)\\b\\s+(?:has\\s+|had\\s+|'s\\s+|just\\s+)?(?:bit|bitten|nipped)\\s+(?:me|him|her|them|my|his|their)\\b"), py_str("\\b(?:dogs?|puppy)\\b.{0,25}\\b(?:bit\\s+(?:me|him|her|them|my|his|their|a|the)|bitten|nipped|snapped\\s+at|went\\s+for)\\b.{0,30}\\b(?:me|my|his|her|their|the\\s+(?:child|kid|baby)|a\\s+(?:child|kid|baby))\\b"), py_str("\\b(?:bitten|nipped|bit\\s+(?:me|my|him|her|them))\\b.{0,25}\\b(?:by\\s+a\\s+dog|dog|puppy)\\b"), py_str("\\b(?:bitten|bit\\s+(?:me|my|him|her|them))\\b.{0,20}\\b(?:face|cheek|lip|hand|finger|arm|leg|ankle)\\b"), py_str("\\b(?:bit|bites?|bitten|nipped)\\s+(?:me|my\\s+(?:hand|hands|finger|fingers|thumb|wrist|arm|arms|leg|legs|foot|feet|toe|toes|face|cheek|nose|ear|lip|shoulder|chest|belly|back|head|neck|skin))\\b[^.?!]{0,30}\\b(?:bled|bleeds?|bleeding|blood|bloody|broke\\s+(?:the|my)\\s+skin|broke\\s+skin|marks?|drew\\s+blood|draw\\s+blood|drawn\\s+blood|infected|needs?\\s+stitches|wont\\s+stop\\s+bleeding|won't\\s+stop\\s+bleeding)\\b"), py_str("\\b(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit|hamster|parrot|budgie|ferret|gerbil)\\b[^.?!]{0,30}\\b(?:attacked|savaged|mauled|set\\s+about|went\\s+for|went\\s+at|lurched\\s+at|turned\\s+on)\\s+(?:me|him|her|them|us)\\b"), py_str("\\b(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit)\\b[^.?!]{0,20}\\b(?:attacked|savaged|mauled|went\\s+for|went\\s+at|turned\\s+on)\\s+(?:my|his|her|their|our)\\s+(?:hand|hands|finger|fingers|thumb|arm|arms|leg|legs|foot|feet|face|cheek|lip|ankle|head|neck|shoulder|ear|ears|nose|eye|wrist|toe|toes|thigh|shin|knee|stomach|back)\\b"), py_str("\\b(?:i|we)\\b[^.?!]{0,15}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been)\\s+(?:attacked|savaged|mauled)\\s+by\\b.{0,20}\\b(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit|hamster|parrot|budgie|ferret|gerbil)\\b"), py_str("\\b(?:i|we)\\b[^.?!]{0,15}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been)\\s+(?:bit|bitten|nipped)\\s+by\\b.{0,20}\\b(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit|hamster|parrot|budgie|ferret)\\b"), py_str("\\b(?:my|his|her|their|our)\\s+(?:hand|hands|finger|fingers|thumb|arm|arms|leg|legs|foot|feet|face|cheek|lip|ankle|head|neck|shoulder|ear|ears|nose|eye|wrist|toe|toes)\\b[^.?!]{0,15}\\b(?:was|were|got|has\\s+been|had\\s+been)\\s+(?:bit|bitten|nipped)\\s+by\\b.{0,20}\\b(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|pony|rabbit|hamster|parrot|budgie|ferret)\\b"), py_str("\\b(?:dog|dogs|cat|cats|puppy|puppies|kitten|kittens|pet|pets|horse)\\b[^.?!]{0,15}\\bbites?\\b[^.?!]{0,30}\\b(?:infected|infection|septic|pus|ooz\\w*|weep\\w*|swollen|swelling|red|hot|stitches?|tetanus|an?\\s*(?:and|&)\\s*e|hospital|antibiotics?)\\b"), py_str("\\b(?:infected|septic|oozing|swollen|stitches?|tetanus|antibiotics?)\\b[^.?!]{0,25}\\b(?:dog|cat|puppy|kitten)\\s?bites?\\b"), py_str("\\btetanus\\b[^.?!]{0,30}\\b(?:bite|bitten|bit|dog|cat|puppy|kitten|scratch)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BITE_DONE"); }
    s_BITE_DONE(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.ALLERGY_NOW"); }
    s_ALLERGY_NOW(py_tuple(vec![py_str("\\b(?:i|im|i'?m|we|we'?re|he|she|they|my\\s+\\w+)\\b[^.?!]{0,14}\\b(?:having|getting|developing|going\\s+into)\\b[^.?!]{0,20}\\b(?:allergic\\s+reaction|reaction|anaphyla\\w*)\\b"), py_str("\\b(?:allergic\\s+reaction|reaction)\\b[^.?!]{0,14}\\b(?:right\\s+now|now|starting|coming\\s+on|just\\s+happened)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.ALLERGY_SIGN_NAMED"); }
    s_ALLERGY_SIGN_NAMED(py_tuple(vec![py_str("\\b(?:swell\\w*|swollen|puff(?:ed|y)?|bloat\\w*|lips?|tongue|throat|mouth|face)\\b"), py_str("\\b(?:breath\\w*|wheez\\w*|stridor|hoarse|croak\\w*|voice)\\b"), py_str("\\b(?:dizzy|faint\\w*|light\\s?headed|collapse\\w*|passing\\s+out|vomit\\w*|throwing\\s+up|sick)\\b"), py_str("\\b(?:sting|stung|epipen|injector)\\b"), py_str("\\b(?:hives?|rash\\w*|itch\\w*|blotch\\w*|red\\s+spots?|nettle\\s+rash)\\b"), py_str("\\b(?:fine\\s+now|was\\s+fine|settled|went\\s+away|calmed\\s+down|last\\s+(?:year|week|month)|years\\s+ago|as\\s+a\\s+kid|used\\s+to|once|back\\s+then)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.GRATER_CUT"); }
    s_GRATER_CUT(py_tuple(vec![py_str("\\bgrat(?:e|es|ed|ing)\\b[^.?!]{0,24}\\b(?:my|his|her|their|the)\\s+(?:finger|thumb|hand|knuckle|nail|fingertip|palm)")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.STRUCK_BY_A_VEHICLE"); }
    s_STRUCK_BY_A_VEHICLE(py_tuple(vec![py_str("\\b(?:nearly|almost|close\\s+to|came\\s+close\\s+to|just\\s+about)\\b[^.?!]{0,20}\\b(?:(?:got|get|gets|getting|been|be|being|was|were|had)\\s+)?(?:run\\s+over|hit|struck|clipped|swiped|knocked\\s+(?:off|down|over))\\b[^.?!]{0,16}\\b(?:by|by\\s+a|by\\s+the)\\b"), py_str("\\b(?:nearly|almost|close\\s+to|came\\s+close\\s+to|just\\s+about)\\b[^.?!]{0,20}\\b(?:got|get|gets|getting|been|be|being|was|were|had)?\\s*(?:run\\s+over|knocked\\s+off|knocked\\s+down|knocked\\s+over|clipped|swiped|rammed|mow(?:ed|n)\\b[^.?!]{0,10}\\bdown)\\b"), py_str("\\b(?:bus|car|lorry|truck|van|tram|train|taxi|cab|motorbike|motorcycle|moped|scooter|bike|bicycle|cyclist|driver)\\b[^.?!]{0,12}\\b(?:nearly|almost)\\s+(?:hit|missed|struck|clipped|knocked)\\s+(?:me|us|him|her|them|my\\s+\\w+)\\b"), py_str("\\b(?:nearly|almost)\\b[^.?!]{0,16}\\b(?:went|go|going|gone|fell|ended\\s+up)\\b[^.?!]{0,14}\\bunder\\s+(?:a|the|an)\\s+(?:bus|car|lorry|truck|van|tram|train)\\b"), py_str("\\b(?:nearly|almost|close\\s+to|just\\s+about)\\b[^.?!]{0,10}\\b(?:ran|run|runs)\\s+(?:me|us)\\s+(?:over|down)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.STRUCK_MY_BIKE"); }
    s_STRUCK_MY_BIKE(py_tuple(vec![py_str("\\b(?:my|our|his|her|their|the)\\s+(?:bike|bicycle|motorbike|motorcycle|scooter|moped)\\b[^.?!]{0,24}\\b(?:got|get|gets|was|were|is|are|been|had\\s+been)\\s*(?:clipped|hit|struck|swiped|knocked|nudged|shunted|rammed|sideswiped)\\b[^.?!]{0,18}\\b(?:van|car|lorry|truck|bus|coach|taxi|cab|tractor|driver)\\b"), py_str("\\b(?:a|an|the|some|one|another)\\s+(?:van|car|lorry|truck|bus|coach|taxi|cab|tractor|driver)\\b[^.?!]{0,16}\\b(?:clipped|hit|struck|swiped|knocked|nudged|shunted|rammed|sideswiped)\\b[^.?!]{0,12}\\b(?:my|our|his|her|their|the)\\s+(?:bike|bicycle|motorbike|motorcycle|scooter|moped)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.NEAR_HIT_ON_A_ROAD"); }
    s_NEAR_HIT_ON_A_ROAD(py_tuple(vec![py_str("\\b(?:nearly|almost|close\\s+to|just\\s+about)\\b[^.?!]{0,12}\\b(?:hit|hits|struck|ran\\s+over|run\\s+over|knocked|clipped|caught)\\s+(?:me|us)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.NEAR_HIT_ON_A_ROAD_PLACE"); }
    s_NEAR_HIT_ON_A_ROAD_PLACE(py_tuple(vec![py_str("\\b(?:crossing|zebra|pelican|crossroads|pavement|kerb|curb|road|street|junction|roundabout|bollard|cycle\\s*lane|bike\\s*lane|lay-?by|motorway|dual\\s+carriageway|bus\\s+stop|car\\s+park|driveway|level\\s+crossing|lights)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.STRUCK_BY_A_VEHICLE_NOT"); }
    s_STRUCK_BY_A_VEHICLE_NOT(py_tuple(vec![py_str("\\b(?:step(?:ped|ping)?|walk(?:ed|ing)?|jump(?:ed|ing)?|stand(?:ing)?|stood|lay|laid|sat|sitting|straddl\\w*|leant|leaned)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.INSECT_BITE_BODY"); }
    s_INSECT_BITE_BODY(py_str("(?:arms?|hands?|fingers?|thumb|legs?|feet|foot|ankles?|wrists?|neck|face|cheek|head|back|shoulders?|knees?|elbow|shin|toes?|hip|stomach|belly|ears?|eyes?|chest|forehead|scalp)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.INSECT_BITE_NEEDS_A_LOOK"); }
    s_INSECT_BITE_NEEDS_A_LOOK(py_tuple(vec![py_str("\\b(?:hands?|wrists?|fingers?|thumb|face|cheek|eyes?|knees?|elbows?|joints?)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.INSECT_BITE"); }
    s_INSECT_BITE(py_tuple(vec![op_add(&op_add(&py_str("\\b(?:mosquito(?:es)?|midges?|gnats?|fleas?|horse\\s?fly|horseflies|cleg|spiders?|bed\\s?bugs?|sand\\s?fly|ants?)\\b[^.?!]{0,24}\\b(?:bit|bitten|bite|nipped|stung?)\\b[^.?!]{0,10}\\b(?:me|him|her|them|us)\\b[^.?!]{0,8}\\b(?:on|in|under|between)\\b[^.?!]{0,8}\\b(?:the|my|his|her|their|both)\\b\\s*"), &g_INSECT_BITE_BODY()), &py_str("\\b")), op_add(&op_add(&py_str("\\b(?:mosquito(?:es)?|midges?|gnats?|fleas?|horse\\s?fly|horseflies|cleg|spiders?|bed\\s?bugs?|ants?)\\b[^.?!]{0,24}\\b(?:bit|bitten|bite|nipped|stung?)\\b\\s+(?:my|his|her|their)\\s+"), &g_INSECT_BITE_BODY()), &py_str("\\b")), py_str("\\b(?:i|we)\\b[^.?!]{0,15}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been)\\b[^.?!]{0,10}\\b(?:bit|bitten|stung)\\s+by\\s+(?:a|an|the)?\\s*(?:mosquito|midge|gnat|flea|ant|spider|horse\\s?fly)\\b[^.?!]{0,10}\\b(?:on|in)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.TICK_BITE"); }
    s_TICK_BITE(py_tuple(vec![py_str("\\bticks?\\b[^.?!]{0,24}\\b(?:bit|bitten|bite|on\\s+me|on\\s+my|attached|dug\\s+in|burrow\\w*|stuck\\s+in|in\\s+my)\\b"), py_str("\\b(?:a\\s+|the\\s+)?tick\\s+(?:bite|bites)\\b"), py_str("\\b(?:i|we)\\b[^.?!]{0,15}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been|might\\s+have\\s+been)\\s+(?:bit|bitten)\\s+by\\s+(?:a|an|the|some)?\\s*tick\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.TICK_ALREADY"); }
    s_TICK_ALREADY(py_tuple(vec![py_str("\\b(?:found|pulled|removed|took|got\\s+(?:it|one))\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BAT_BITE"); }
    s_BAT_BITE(py_tuple(vec![py_str("\\b(?:bat|bats)\\b[^.?!]{0,24}\\b(?:bit|bitten|bite|scratch(?:ed|es)?|claw(?:ed)?|nipped)\\b[^.?!]{0,24}\\b(?:me|him|her|them|us|my|his|her|their)\\b"), py_str("\\b(?:i|we)\\b[^.?!]{0,15}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been|might\\s+have\\s+been)\\s+(?:bit|bitten|nipped|scratched|clawed)\\s+by\\s+(?:a|an|the)?\\s*bat\\b"), py_str("\\b(?:a|an|the|that|this|my|his|her|their)\\s+bat\\s+(?:bite|bites|scratch|scratches)\\b(?!\\s+(?:a|an|the|it)\\b)"), py_str("\\bbat\\b[^.?!]{0,40}\\b(?:woke\\s+up\\s+with|marks?|a\\s+mark|scratch\\w*)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.WILD_BITE"); }
    s_WILD_BITE(py_tuple(vec![py_str("\\b(?:fox|foxes|badger|badgers|squirrel|squirrels|rat|rats|hedgehog|hedgehogs|mouse|mice|deer|otter|weasel|stoat|mole|vole|seagull|gull|swan|goose|geese|crow|magpie|pigeon|kestrel|buzzard|owl|tick|ticks|wild\\s+animal)\\b[^.?!]{0,24}\\b(?:bit|bitten|bite|nipped|scratched|clawed|went\\s+for|attacked)\\b[^.?!]{0,20}\\b(?:me|him|her|them|us|my|his|her|their)\\b"), py_str("\\b(?:i|we)\\b[^.?!]{0,15}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been)\\s+(?:bit|bitten|nipped|scratched|clawed)\\s+by\\s+(?:a|an|the|some)?\\s*(?:fox|badger|squirrel|rat|hedgehog|mouse|deer|otter|seagull|gull|swan|goose|wild\\s+animal)\\b"), py_str("\\b(?:a|an|the|that|this|my|his|her|their)\\s+(?:fox|badger|squirrel|rat|hedgehog)\\s+(?:bite|bites|scratch)\\b(?!\\s+(?:a|an|the|it)\\b)")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.COUNTERFACTUAL_WISH"); }
    s_COUNTERFACTUAL_WISH(py_tuple(vec![py_str("\\bwis?h\\b[^.?!]{0,24}\\b(?:i|he|she|they|it|the\\s+\\w+|that\\s+\\w+)\\b[^.?!]{0,16}\\b(?:had|have|would\\s+have|'d)\\b[^.?!]{0,12}\\b(?:died|been\\s+killed|killed\\s+me|not\\s+survived|never\\s+woke\\s+up|ended\\s+it|gone\\s+ahead|done\\s+it|finished\\s+it|not\\s+woken\\s+up)\\b"), py_str("\\bwis?h\\b[^.?!]{0,16}\\b(?:i|he|she|they)\\b[^.?!]{0,10}\\b(?:hadn'?t|had\\s+not|never)\\b[^.?!]{0,10}\\b(?:survived|woken\\s+up|made\\s+it|come\\s+round|pulled\\s+through)\\b"), py_str("\\b(?:nearly|almost)\\b[^.?!]{0,30}\\b(?:died|killed\\s+(?:my|him|her)self|got\\s+hit|been\\s+hit|run\\s+over|knocked\\s+(?:me\\s+)?(?:off|down|over)|gone\\s+under)\\b[^.?!]{0,40}\\bwis?h\\b[^.?!]{0,30}\\b(?:it|that|i|they)\\b[^.?!]{0,12}\\b(?:had|would\\s+have|'d)\\b[^.?!]{0,16}\\b(?:worked|happened|gone\\s+through|succeeded|done\\s+it|killed\\s+me|finished\\s+(?:it|the\\s+job|me\\s+off)|taken\\s+me|done\\s+for\\s+me)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.EFFORT_BEFORE"); }
    s_EFFORT_BEFORE(py_str("\\b(?:came|come|fell|fallen|slipped|tripped|tumbled|crashed|went\\s+over|got\\s+knocked)\\b[^.?!]{0,24}\\b(?:nearly|almost)\\b[^.?!]{0,20}\\b(?:killed|kill)\\s+my\\s*self\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.EFFORT_DIED"); }
    s_EFFORT_DIED(py_str("\\b(?:nearly|almost|came\\s+close\\s+to|so\\s+close\\s+to)\\b[^.?!]{0,20}\\b(?:died|killed\\s+(?:my|him|her|them)self)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.COME_OFF_IT"); }
    s_COME_OFF_IT(py_str("\\b(?:off|on|from)\\s+(?:(?:my|his|her|their|our|a|the)\\s+)?(?:bike|bicycle|motorbike|motorcycle|scooter|moped|horse|pony|quad|skateboard|ladder|roof|scaffold|balcony|wall|fence|trampoline|stairs|steps|staircase|road|motorway|kerb|car|van|truck|lorry|tractor|boat)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.EFFORT_DOING"); }
    s_EFFORT_DOING(py_str("\\b(?:nearly|almost)\\b[^.?!]{0,24}\\b(?:killed|kill)\\s+(?:my|him|her|them)self\\s+(?P<doing>[a-z']+ing)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.EFFORT_NOT_AN_ACCIDENT"); }
    s_EFFORT_NOT_AN_ACCIDENT(py_tuple(vec![py_str("\\b(?:on\\s+purpose|deliberately|by\\s+choice|meant\\s+to|tried\\s+to|wanted\\s+to)\\b"), py_str("\\b(?:wish|wished|hope|hoped)\\b"), py_str("\\b(?:so\\s+i\\s+(?:would|could)\\s+die|to\\s+die)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.COUNTERFACTUAL_IDIOM"); }
    s_COUNTERFACTUAL_IDIOM(py_tuple(vec![py_str("\\b(?:died|killed\\s+(?:my|him|her)self|killed\\s+me)\\s+(?:laughing|of\\s+(?:embarrassment|boredom|shame|thirst|curiosity))\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PERSON_HANGING"); }
    s_PERSON_HANGING(py_tuple(vec![py_str("\\b(?:he|she|they|hes|shes|theyre|someone|somebody|(?:my|his|her|their|our)\\s+(?:brother|sister|mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|husband|wife|partner|boyfriend|girlfriend|ex|uncle|aunt|nan|nana|gran|granny|grandad|grandpa|grandma|grandmother|grandfather|cousin|friend|mate|flatmate|housemate|neighbour|neighbor|carer|stepdad|stepson|stepdaughter|nephew|niece)s?)\\b[^.?!]{0,20}?'?s?\\s+(?:is\\s+|was\\s+)?(?:hanging|hung)\\b(?!\\s+(?:the|a|an|his|her|their|my|our|your|some|those|these|up|out|around|about|over|back|christmas|fairy|lights|decorations|baubles|garland|wreath|washing|laundry|clothes|out\\s+with|wallpaper|tinsel|foil|lining|bunting|paper|himself|herself|themselves|my\\s*self|myself|on\\s+the\\s+(?:phone|line|doorstep|door|bell|internet|web|sofa|fence|gate|wall))\\b)(?!\\s+(?:by\\s+a\\s+thread|by\\s+(?:his|her|their|my)\\s+fingertips?|in\\s+there)\\b)(?!\\s+on\\b\\s*(?:$|[.,!?;]|\\bin\\s+there\\b|at\\b))(?!\\s+(?:on|from|off)\\s+(?:the\\s+)?(?:monkey\\s+bars|wall\\s+bars|climbing\\s+frame|trampoline|slide|swing|bars|sofa|couch|settee|bed|bunk|bunk\\s+bed|chair|table|bike|swing\\s+set)\\b)(?!\\s+upside\\s+down\\b)(?!\\s+(?:on\\s+)?for\\s+dear\\s+life\\b)(?!\\s+[a-z]+s\\b)(?!\\s+round\\b)(?!\\s+with\\s+(?:(?:his|her|their|my|our)\\s+|the\\s+)?(?:mates?|friends?|family|kids|lads|girls|boys|crowd)\\b)(?!\\s+(?:by|at)\\s+the\\s+(?:bar|counter|pub)\\b)")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HEAD_OUT_COLD"); }
    s_HEAD_OUT_COLD(py_tuple(vec![py_str("\\bout\\s+cold\\b[^.?!]{0,30}\\b(?:tackle[ds]?|rugby|football|boxing|match|game|scrum|fell|fall|fallen|collided|collision|crashed?|crash|car|van|bike|motorbike|lorry|stairs|steps|ladder|wall|door|branch|tree|goalpost|post|railing|kerb|curb|round|rounds|canvas|ring|ring\\s+rope)\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,30}\\b(?:hit|banged?|struck|smacked|cracked|knocked)\\b[^.?!]{0,12}\\bheads?\\b"), py_str("\\bheads?\\b[^.?!]{0,20}\\b(?:injur\\w+|wound|trauma)\\b[^.?!]{0,20}\\bout\\s+cold\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,60}\\b(?:after|from|when|because)\\b[^.?!]{0,30}\\b(?:a\\s+bang|bang(?:ed)?|hitting|hit|blew|blow|knocked|knock|fall|fell|falling|blow\\s+to|kick|punch|crash|accident|smack)\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,40}\\b(?:bleeding|bled|blood|fitting|had\\s+a\\s+fit(?!\\s+of\\s+(?:giggles|the\\s+giggles|laughing))|seizure|convuls\\w*|stopped\\s+breathing|not\\s+breathing|vomit\\w*|threw\\s+up|being\\s+sick)\\b"), py_str("\\b(?:hit|banged?|knocked|struck|clipped|collided|fell|fallen|crashed|tackled)\\b[^.?!]{0,30}\\bout\\s+cold\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,20}\\b(?:floor|ground|street|pavement|road|sofa|couch)\\b"), py_str("\\bfound\\s+(?:him|her|them|my\\s+\\w+|\\w+)\\b[^.?!]{0,25}\\bout\\s+cold\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,30}\\bwhen\\s+i\\s+(?:found|got\\s+home|got\\s+in|got\\s+back|came\\s+in|came\\s+home|arrived)\\b"), py_str("\\bout\\s+cold\\s+for\\s+(?:more\\s+than\\s+a\\s+minute|(?:two|three|four|five|six|seven|eight|nine|ten|fifteen|twenty|thirty|forty|fifty|sixty|\\d{2,})\\s*(?:mins?|minutes?|hours?|hrs?)|ages|hours)\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,20}\\b(?:blue|grey|gray|clammy|mottled|ashen)\\b"), py_str("\\bout\\s+cold\\b[^.?!]{0,40}\\b(?:wet\\s+my\\s*self|wet\\s+the\\s+bed|wet\\s+(?:him|her)\\s*self|soaked\\s+my\\s*self|peed\\s+my\\s*self|peed\\s+(?:him|her)\\s*self|incontinent)\\b")]));
    fn __data2() -> Py { py_tuple(vec![py_str("\\b(?:fell|fallen|falling|came|come|comes|slipped|toppled|tipped)\\s+off\\s+(?:\\w+\\s+)?(?:bike|bicycle|motorbike|motorcycle|scooter|moped|skateboard|horse)\\b[^.?!]{0,40}\\b(?:hit|hits|banged|bangs|struck|strikes)\\s+(?:my|his|her|their|our|your)\\s+(?:head|face|chin|nose|knee|elbow|arm|leg|hip|shoulder|hand|wrist|ankle|back|chest)\\b"), py_str("\\bnearly\\b"), py_str("\\balmost\\b"), py_str("\\bjust\\s+about\\b"), py_str("\\bnarrowly\\b"), py_str("\\bclose\\s+call\\b"), py_str("\\bwhat\\s+if\\b"), py_str("\\bif\\s+(?:i|he|she|they|we|my\\s+\\w+)\\b"), py_str("\\bimagine\\b"), py_str("\\bsuppose\\b"), py_str("\\b(?:years?|decades?)\\s+(?:ago|back)\\b"), py_str("\\bin\\s+(?:19|20)\\d\\d\\b"), py_str("\\bwhen\\s+(?:i|he|she|they)\\s+(?:was|were)\\s+(?:a\\s+|an\\s+)?(?:kid|child|little|small|young|boy|girl|teenager|teen)\\b"), py_str("\\blast\\s+(?:year|summer|winter|christmas|xmas|term|month|week)\\b"), py_str("\\bmissed\\s+(?:me|him|her|them|us|my\\s+\\w+)\\b"), py_str("\\bfrom\\s+behind\\b"), py_str("\\bin\\s+the\\s+back\\b"), py_str("\\brear-?end\\w*\\b"), py_str("\\bwhiplash\\b"), py_str("\\bshunt\\w*\\b"), py_str("\\bwrote\\s+(?:it|the\\s+car)\\s+off\\b"), py_str("\\bover\\s+to\\b"), py_str("\\bover\\s+(?:the\\s+|a\\s+|our\\s+|my\\s+|your\\s+)?(?:plan|plans|details|budget|figures|numbers|points|list|lists|options|wording|email|emails|costs|cost|rules|schedule|story|recipe|contract|draft|accounts|it|them|this|that)\\b"), py_str("\\bran\\s+over\\s+time\\b"), py_str("\\bover\\s+time\\b"), py_str("\\b(?:you|we|i|they)\\s+(?:can|could|should|must)\\s+run\\s+over\\b"), py_str("\\b(?:foxe?s?|badgers?|hedgehogs?|deer|squirrels?|owls?|swans?|pigeons?|hares?)\\b\\s+(?:\\w+\\s+){0,3}(?:run|ran|runs|got|get|was|were|is)?\\s*(?:over|hit|hits|struck|clipped|knocked)\\b"), py_str("\\bby\\s+(?:a\\s+|the\\s+)?(?:trolley|shopping\\s+trolley|pram|pushchair|buggy|wheelchair|mobility\\s+scooter|crowd|queue)\\b"), py_str("\\b(?:a|an|the|its|his|her|their|my|our)\\s+(?:\\w+\\s+)?doors?\\b"), py_str("\\b(?:hit|hits|struck|strikes|clipped|clips)\\s+(?:me|him|her|them|us)\\s+with\\b"), py_str("\\b(?:mum|mom|mother|dad|father|son|daughter|husband|wife|partner|boyfriend|girlfriend|brother|sister|friend|child|kid|baby|boy|girl|man|woman|lady|neighbou?r|colleague|mate|gran|granny|nan|grandad|grandma|grandpa|passenger|cyclist|pedestrian|toddler|teenager|nephew|niece|cousin|aunt|uncle|stepson|stepdaughter|stepdad|stepmum|driver|owner)\\s*['\\u2019]?s?\\s+(?:car|cars|bike|bikes|van|vans|lorry|lorries|truck|trucks|bus|buses|wall|walls|fence|fences|gate|gates|hedge|hedges|house|window|windows|bins?|garden|garage|drive|driveway|shed|roof|bumper|wing|mirror|gatepost|door|school|schools|nursery|playground|office|work|workplace|shop|shops|church|building|home|flat|room)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.STRUCK_NOT_NOW"); }
    s_STRUCK_NOT_NOW(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SELF_HARM_HABIT"); }
    s_SELF_HARM_HABIT(py_tuple(vec![py_str("\\bi\\s+(?:keep|keeps|kept|keep\\s+on)\\s+(?:cutting|hurting|burning)\\b(?=\\s*(?:$|[.,!?]|and\\b|again\\b|so\\b|all\\b|every\\b))"), py_str("\\b(?:keep|keeps|kept|keep\\s+on)\\s+(?:cutting|hurting|burning|scratching|hitting|banging|punching|choking|starving)\\s+(?:(?:my|him|her|your)\\s*self|them\\s*selves|them\\s*self)\\b"), py_str("\\b(?:we'?ve|i'?ve|we|i)\\s+(?:have|has|had|'?ve)?\\s*been\\s+(?:on\\s+)?cutting\\b(?=\\s*(?:$|[.,!?]|a\\s+lot\\b|recently\\b|lately\\b|too\\b|most\\b|badly\\b|every\\s+day\\b|now\\b|on\\s+and\\s+off\\b|again\\b|for\\s+(?:weeks|months|years|ages|days|a\\s+while|a\\s+bit|a\\s+long\\s+time)\\b|since\\s+(?:i\\s+was|then|last\\s+year|christmas|primary|school|my\\s+teens|forever)\\b|and\\s+(?:i|it)\\b|so\\s+(?:much|badly|many|hard)\\b))"), py_str("\\bi\\s+cut\\s+again\\b(?!\\s+(?:on|with|at|in|while|after|during|doing|from|using|because|the\\s+(?:knife|bread|glass|gym|run))\\b)")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HEAL_TIME"); }
    s_HEAL_TIME(py_dict(vec![(py_str("sprain"), py_tuple(vec![py_str("It depends how bad it is. A mild sprain is usually much better in one to two weeks and normal again in four to six; a bad one with a lot of bruising can take eight to twelve weeks, and the first 48 hours of rest, ice and keeping it raised are what set that up. Start gentle movement and walking on it as the pain allows rather than staying off it, and get it seen if you cannot put weight on it at all, it is very swollen, or it is no better after a week."), py_str("Most sprains are much better in one to two weeks and properly right in four to six; a bad one can take a couple of months. Ice and rest for the first two days, then gentle walking and moving it as far as it lets you. A joint you cannot put weight on, or one that is no better after a week, is worth having looked at.")])), (py_str("burn"), py_tuple(vec![py_str("A small burn usually settles in a week to ten days, and a blistering one takes about two to three weeks to heal from underneath. Keep it clean and covered and leave blisters alone; anything bigger than your palm, on the face or hands, or slow to heal after two weeks should be seen."), py_str("A small burn is mostly fine in seven to ten days, and a blistered one in about two to three weeks. Leave the blisters alone and keep it covered; a burn that is not healing after a fortnight, or is on the face or hands, needs a nurse.")])), (py_str("blister"), py_tuple(vec![py_str("A rubbed blister drains and dries up in about three to five days and the skin under it is new in a week or two; leave the roof on, cover it with a padded plaster and keep it dry. One that is hot, spreading or weeping pus is the doctor's rather than time."), py_str("Most blisters settle in three to five days and are properly new skin in a week or two. Cover it, don't pop it, and keep it dry; hot, spreading or pus needs a doctor.")])), (py_str("cut"), py_tuple(vec![py_str("A small cut closes over in a few days and is usually fully healed in a week or two, with the edges kept together and the dressing changed. Redness spreading, pus, or a cut that gapes needs to be seen the same day rather than waited on."), py_str("Small cuts close in a few days and are properly healed in a week or two. Keep it clean and covered, and get it looked at if it goes red and hot, weeps pus, or will not close.")])), (py_str("sunburn"), py_tuple(vec![py_str("The redness of sunburn usually settles in three to five days and the peeling in about a week. Cool showers, plenty of water and leaving the peeling skin alone are what help; shivering, dizziness or bad blisters need a doctor."), py_str("Sunburn redness mostly fades in three to five days and the peeling takes about a week. Cool showers and water help, and leave the skin alone; shivery, dizzy or badly blistered is a doctor.")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HEAD_REPLIES"); }
    s_HEAD_REPLIES(py_dict(vec![(py_str("head_bang"), py_tuple(vec![py_str("A bang on the head is usually fine, but the next 24 hours are the ones to watch: being sick, being knocked out, or feeling confused or unusually sleepy means an ambulance or A&E rather than wait, and 111 will advise you on anything milder. Are you up and talking normally now, or has anything changed since it happened?"), py_str("A bumped head hurts for a day or two and an ice pack on it for ten minutes is plenty, but a knock-out, being sick, or feeling confused or sleepier than usual means an ambulance or A&E rather than wait - 111 for anything milder. Are you up and talking normally now, or has anything changed since it happened?")])), (py_str("head_bang_other"), py_tuple(vec![py_str("A bang on the head is usually fine, but the next 24 hours are the ones to watch: being sick, being knocked out, or {s} seeming confused or unusually sleepy means an ambulance or A&E rather than wait, and 111 will advise you on anything milder. Is {s} up and talking normally now, or has anything changed since it happened?"), py_str("A bumped head hurts for a day or two, but a knock-out, being sick, or {s} seeming confused or sleepier than usual means an ambulance or A&E rather than wait - 111 for anything milder. Is {s} up and talking normally now, or has anything changed since it happened?")])), (py_str("head_bang_red"), py_tuple(vec![py_str("That is a 999 call rather than wait and see: being knocked out, confused, drowsy or sick after a bang on the head is a head injury until a doctor says otherwise. Keep still and awake, stay with someone, and don't drive yourself."), py_str("Ring 999 or 911 now rather than waiting to see: confusion, drowsiness or sickness after a bang on the head is a head injury until a doctor says otherwise. Keep awake and still while you wait, and don't take anything for the headache until someone has seen you.")])), (py_str("head_bang_red_other"), py_tuple(vec![py_str("That is a 999 call rather than wait and see: being knocked out, confused, drowsy or sick after a bang on the head is a head injury until a doctor says otherwise. Stay with {o}, keep {o} awake and still, and don't let {o} drive."), py_str("Ring 999 or 911 now rather than waiting to see: confusion, drowsiness or sickness after a bang on the head is a head injury until a doctor says otherwise. Keep {o} awake and still, and stay with {o} until help comes.")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HUMAN_BITTEN"); }
    s_HUMAN_BITTEN(py_tuple(vec![py_str("\\b(?:bit|bitten|nipped|snapped\\s+at|went\\s+for|attacked)\\s+(?:me|him|her|us|them)\\b"), py_str("\\b(?:bit|bitten|nipped)\\s+(?:my|his|her|their|our)\\s+(?:hand|hands|finger|fingers|thumb|arm|arms|leg|legs|foot|feet|face|cheek|lip|ankle|head|neck|shoulder|ear|ears|nose|eye|wrist|toe|toes)\\b"), py_str("\\b(?:bit|bitten|nipped)\\s+(?:the|a)\\s+(?:child|kid|baby|boy|girl|man|woman|postman|postie|vet)\\b"), py_str("\\b(?:my|our|her|his|their)\\s+(?:son|daughter|child|kid|baby|husband|wife|partner|mum|mom|dad|father|mother|friend|neighbour|neighbor|postman|postie)\\b[^.?!]{0,20}\\b(?:was|were|got|has\\s+been|had\\s+been)\\s+(?:bit|bitten)\\b"), py_str("\\b(?:i|we)\\b[^.?!]{0,12}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been)\\s+(?:bit|bitten)\\b"), py_str("\\b(?:i|we)\\b[^.?!]{0,15}\\b(?:was|were|got|have\\s+been|has\\s+been|had\\s+been)\\s+(?:attacked|savaged|mauled|scratched|clawed)\\b"), py_str("\\b(?:my|his|her|their|our)\\s+(?:hand|hands|finger|fingers|thumb|arm|arms|leg|legs|foot|feet|face|cheek|lip|ankle|head|neck|shoulder|ear|ears|nose|eye|wrist|toe|toes)\\b[^.?!]{0,15}\\b(?:was|were|got|has\\s+been|had\\s+been)\\s+(?:bit|bitten)\\b"), py_str("\\b(?:scratch(?:ed|es|ing)?|claw(?:ed|s|ing)?)\\s+(?:me|him|her|us|them)\\b"), py_str("\\b(?:scratch(?:ed|es|ing)?|claw(?:ed|s|ing)?)\\s+(?:my|his|her|their|our)\\s+(?:hand|hands|finger|fingers|thumb|arm|arms|leg|legs|foot|feet|face|cheek|lip|ankle|head|neck|shoulder|ear|ears|nose|eye|wrist|toe|toes|skin)\\b")]));
    fn __data3() -> Py { py_tuple(vec![py_str("\\bthrough\\s+my\\s+nose\\b"), py_str("\\bfood\\s+poisoning\\b"), py_str("\\bpoison\\s+ivy\\b"), py_str("\\bpoisonous\\s+(?:plant|mushroom|snake|spider)\\b"), py_str("\\bfire\\s+(?:alarm|drill|station|escape|pit|place|works|extinguisher|guard|fighter)s?\\b"), py_str("\\b(?:camp|bon)fire\\b"), py_str("\\bfireworks?\\b"), py_str("\\bfired?\\s+(?:from|up)\\b"), py_str("\\bon\\s+fire\\b.*\\b(?:team|player|form|streak)\\b"), py_str("\\b(?:bleed|bled|bleeding)\\s+(?:the|my|a)\\s+(?:radiator|pipe|boiler|brakes?)\\b"), py_str("\\bbleed(?:ing)?\\s+the\\s+(?:heating|radiators?)\\b"), py_str("\\bblood\\s+(?:test|pressure|donor|donation|type|sugar|group)\\b"), py_str("\\bbloody\\b"), py_str("\\bkill(?:ing)?\\s+(?:time|the\\s+weeds)\\b"), py_str("\\bdead\\s+(?:battery|phone|line|end|weight)\\b"), py_str("\\bdie\\s+cast\\b"), py_str("\\bcut\\s+(?:my|the)\\s+(?:hair|nails|lawn|grass)\\b"), py_str("\\bcut\\s+it\\s+out\\b"), py_str("\\bwak(?:e|ing)\\s+up\\s+(?:in|before)\\s+(?:the\\s+morning|noon|\\d)"), py_str("\\bchest\\s+of\\s+drawers\\b"), py_str("\\bfire\\s+me\\b"), py_str("\\brained\\s+off\\b"), py_str("\\bstabbed\\s+in\\s+the\\s+back\\b"), py_str("\\bhit\\s+the\\s+nail\\s+on\\s+the\\s+head\\b"), py_str("\\bhead\\s*-?\\s*bang(?:ing|er|ers)\\b"), py_str("\\b(?:i|im|i'?m|i'?ve)\\b[^.?!]{0,20}\\bbang(?:ing|ed)?\\s+my\\s+head\\s+against\\s+(?:a\\s+|the\\s+)?(?:wall|brick|desk|door|table)\\b"), py_str("\\bbang(?:ing|ed)?\\s+(?:your|his|her)\\s+head\\s+against\\s+(?:a\\s+|the\\s+)?(?:wall|brick|desk|door|table)\\b"), py_str("\\bhead\\s+(?:over\\s+heels|start|of\\s+(?:the\\s+)?(?:department|house|school|state)|count)\\b"), py_str("\\bknock(?:ed|s)?\\s+out\\s+of\\s+(?:the\\s+)?(?:cup|competition|tournament|playoffs?|championship|league|competition|semis?|final|series|race)\\b"), py_str("\\bknock(?:ed|s)?\\s+(?:it|this|that|things)\\s+out\\s+of\\s+the\\s+park\\b"), py_str("\\b(?:dog|puppy|pup|cat|kitten|horse|pony|rabbit|hamster|parrot|bird)\\s+(?:is|was|seems|gets|looks|has\\s+got)\\s+(?:really\\s+|very\\s+|so\\s+|a\\s+bit\\s+)?(?:scared|afraid|terrified|frightened|nervous|anxious)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.MILD"); }
    s_MILD(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SCALD"); }
    s_SCALD(py_tuple(vec![py_str("\\b(?:burn\\w*|scald\\w*|boiling\\s+water|hot\\s+oil|steam|pan\\s+of\\s+(?:water|oil|fat))\\b[^.?!]{0,40}\\b(?:skin\\s+)?(?:is\\s+|gone\\s+)?(?:peeling|peeled|peeling\\s+off|waxy|charred|blackened)\\b"), py_str("\\b(?:kettle|saucepan|pan)\\b[^.?!]{0,40}\\b(?:skin|hand|arm|foot|feet|leg|face|wrist|fingers?|chest|shoulder|thigh|thumb|toes?)\\b[^.?!]{0,30}\\b(?:peeling|peeled|waxy|charred|blackened)\\b"), py_str("\\b(?:skin|hand|arm|foot|feet|leg|face|wrist|fingers?|chest|shoulder|thigh|thumb|toes?)\\b[^.?!]{0,30}\\b(?:peeling|peeled|waxy|charred|blackened)\\b[^.?!]{0,30}\\b(?:kettle|saucepan|pan)\\b"), py_str("\\b(?:went|gone|turned|turning|going|is|was)\\s+white\\b[^.?!]{0,40}\\b(?:burn\\w*|scald\\w*|boiling|hot\\s+oil|steam|kettle|water\\s+(?:hit|landed|went))\\b"), py_str("\\b(?:burn\\w*|scald\\w*|boiling|hot\\s+oil|steam|kettle)\\b[^.?!]{0,40}\\b(?:went|gone|turned|turning|going|is|was)\\s+white\\b"), py_str("\\b(?:baby|toddler|child|son|daughter|kid|little\\s+one)\\b[^.?!]{0,40}\\b(?:kettle|saucepan|pan)\\b[^.?!]{0,40}\\b(?:over|on\\s+(?:him|her|them)sel(?:f|ves)|on\\s+him|on\\s+her)\\b"), py_str("\\b(?:boiling|scalding)\\s+(?:water|oil|fat|soup|tea|coffee)\\b[^.?!]{0,30}\\b(?:over|on|onto)\\s+(?:my|his|her|their|the)\\s+(?:baby|toddler|child|son|daughter|kid)\\b"), py_str("\\b(?:baby|toddler|child|son|daughter|kid|little\\s+one)\\b[^.?!]{0,40}\\b(?:boiling|scalding)\\s+(?:water|oil|fat|soup|tea|coffee|kettle|pan)\\b"), py_str("\\b(?:boiling|scalding)\\s+(?:water|oil|fat|soup|tea|coffee|kettle|pan)\\b[^.?!]{0,40}\\b(?:him|her|them)sel(?:f|ves)\\b"), py_str("\\b(?:baby|toddler|child|kid|little\\s+one|boy|girl)\\b[^.?!]{0,30}\\b(?:scalded|burnt|burned)\\s+(?:him|her|them)sel(?:f|ves)\\b"), py_str("\\b(?:son|daughter|boy|girl|child|kid|niece|nephew)\\b[^.?!]{0,40}\\b(?:scalded|burnt|burned)\\b[^.?!]{0,50}\\b(?:screaming|screamed|crying|badly|all\\s+over|blister\\w*|peeling)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HYPERBOLE_WISH"); }
    s_HYPERBOLE_WISH(py_tuple(vec![py_str("\\b(?:i'?m|im|i\\s+am|i\\s+feel|feeling)\\s+(?:(?:so|too|that|this|really|proper|bloody|absolutely|utterly|completely|dead)\\s+)?(?:tired|exhausted|bored|knackered|shattered|worn\\s+out|sleepy|hungry|thirsty|hot|cold|sore|aching|full|stuffed|busy|fed\\s+up|embarrassed|mortified|old|poor|skint|stressed|nervous|scared|itchy|sweaty|dirty|covered\\s+in\\s+\\w+|hungover|hung\\s?over|ill|poorly|unwell|headachy|aching|achy|cream\\s+crackered|croaky|snotty|sniffly)\\b[^.?!]{0,24}\\b(?:wish|wished|wishing)\\b"), py_str("\\b\\w+(?:\\s+\\w+)?\\s+(?:is|are|was|were)\\s+(?:(?:so|too|that|this|really|proper|bloody)\\s+)?(?:boring|dull|slow|tiring|tedious|repetitive|long|hot|cold|loud|noisy|busy|quiet|rubbish|crap|annoying|fiddly|heavy|stiff|itchy|sweaty|grim|depressing|pointless|bad|worse|awful|terrible|horrible|dreadful|miserable|sore|painful|aching|achy|hungover|hung\\s?over|exhausting|stressful|tough|hard|chaotic|mental|hectic|shocking|useless)\\b[^.?!]{0,24}\\b(?:wish|wished|wishing)\\b"), py_str("\\b(?:my\\s+)?\\w+\\s+(?:is|are|have\\s+been)\\s+killing\\s+me\\b[^.?!]{0,24}\\b(?:wish|wished|wishing)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:head|heads|feet|foot|knees?|back|legs?|arms?|shoulders?|hips?|teeth|tooth|neck|stomach|belly|ankles?|wrists?|hands?|thighs?|calves|calf|elbows?|joints?|shoulder\\s+blade|lower\\s+back|sciatica)\\b[^.?!]{0,28}\\b(?:hurt|hurts|hurting|aching|aches?|sore|agony|killing\\s+me|throbbing|stiff|bad)\\b[^.?!]{0,20}\\b(?:wish|wished|wishing)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PURSUIT_PLACE_HOME"); }
    s_PURSUIT_PLACE_HOME(py_tuple(vec![py_str("\\b(?:outside|in\\s+front\\s+of|at\\s+the\\s+end\\s+of|round\\s+the\\s+back\\s+of)\\s+(?:my|our)\\s+(?:house|home|flat|door|doorstep|front\\s+door|window|gate|drive|driveway|garden)\\b"), py_str("\\b(?:banging|hammering|pounding|knocking|ringing)\\s+(?:on|at)\\s+(?:my|our|the)\\s+(?:door|doorbell|window|buzzer)\\b"), py_str("\\b(?:waiting|waits|sat|sitting|stood|standing|parked|lurking|hiding|hovering|hanging\\s+around)\\s+(?:right\\s+|still\\s+|now\\s+)?(?:outside|in\\s+front\\s+of|at\\s+the\\s+end\\s+of|round\\s+the\\s+back\\s+of)\\s+(?:my|our)\\s+(?:house|home|flat|door|doorstep|front\\s+door|window|gate|drive|driveway|garden)\\b"), py_str("\\b(?:turned|turns|turning|showed|shows|showing)\\s+up\\s+(?:at|outside|in|on)\\s+(?:my|our)\\s+(?:house|home|flat|door|doorstep|work|office|workplace|mum'?s|mum|mother'?s|parents'?|school|gate)\\b"), py_str("\\b(?:turned|turns|turning|showed|shows|showing)\\s+up\\s+(?:outside|unannounced|uninvited)\\b"), py_str("\\b(?:at|on)\\s+(?:my|our)\\s+(?:front\\s+)?(?:door|doorstep|buzzer)\\b"), py_str("\\b(?:following|followed|follows)\\s+(?:me|us)\\b(?!\\s+(?:back\\s+)?(?:on|at|in)\\s+(?:instagram|insta|facebook|fb|twitter|x|tiktok|socials?|social\\s+media|linkedin|youtube|reddit|snapchat|strava|whatsapp|threads|the\\s+app|here))"), py_str("\\bstalk(?:s|ing|ed)\\s+(?:me|us)\\b(?!\\s+(?:back\\s+)?(?:on|at|in)\\s+(?:instagram|insta|facebook|fb|twitter|x|tiktok|socials?|social\\s+media|linkedin|youtube|reddit|snapchat|strava|whatsapp|threads))"), py_str("\\b(?:watching|watched|watches)\\s+(?:my|our)\\s+(?:house|home|flat)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.INJURY_HOW"); }
    s_INJURY_HOW(py_tuple(vec![py_str("\\b(?:hurt|hurting|injured|injuring|tweaked|strained|pulled|sprained|wrenched)\\s+my\\s*self\\s+(?:while\\s+|when\\s+)?(?:lifting|carrying|moving|shifting|digging|running|jogging|walking|hiking|cycling|swimming|training|exercising|boxing|climbing|dancing|skating|skiing|rowing|playing|gardening|cooking|chopping|cleaning|hoovering|decorating|painting|diy|stretching|working\\s+out|squatting|deadlifting|shopping|ice\\s+skating)\\b"), py_str("\\b(?:hurt|hurting|injured|injuring)\\s+my\\s*self\\b[^.?!]{0,24}\\b(?:in|at|on)\\s+(?:the|my|a)\\s+(?:kitchen|garden|garage|gym|office|stairs|bathroom|shower|shed|workshop|pitch|field|court|track|pool|sea|road|street|bus|train|ladder|roof|site|building\\s+site|work)\\b"), py_str("\\b(?:hurt|hurting|injured|injuring)\\s+my\\s*self\\s+at\\s+work\\b"), py_str("\\b(?:hurt|hurting|injured|injuring)\\s+my\\s*self\\s+(?:on|with)\\s+(?:a|an|the|some)\\s+(?:\\w+\\s+){0,2}(?:door|shelf|drawer|car|bike|ladder|knife|drill|saw|pan|oven|hob|kettle|weight|weights|barbell|dumbbell|box|bag|suitcase|pram|wheelchair|step|stair|curb|kerb)\\b"), py_str("\\b(?:hurt|hurting|injured|injuring)\\s+my\\s*self\\b[^.?!]{0,30}\\band\\s+(?:it|my\\s+\\w+)\\s+(?:still\\s+)?(?:hurts|aches|is\\s+sore|swelled|swollen|is\\s+bruised)\\b"), py_str("\\b(?:fell|fallen|falling|tripped|slipped)\\b[^.?!]{0,32}\\b(?:hurt|injured|cut|cutting|nicked|sliced|grazed|scraped)\\s+my\\s*self\\b"), py_str("\\b(?:hurt|hurting|injured|injuring|cut|cutting|nicked|sliced)\\s+my\\s*self\\s+(?:falling|tripping|slipping)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LIPS_SWELL_BARE"); }
    s_LIPS_SWELL_BARE(py_tuple(vec![py_str("\\b(?:my|his|her|their)\\s+(?:lips?|tongue|throat|face)\\s+(?:are\\s+|is\\s+|'s\\s+)?(?:swelling|swollen|swelled|puff\\w*\\s+up|puffed\\s+up|puffing\\s+up|blowing\\s+up)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:throat|tongue|uvula)\\b[^.?!]{0,15}\\b(?:is|'s|feels?)\\s+(?:closing|tight|swelling|swollen)\\b"), py_str("\\b(?:my|his|her|their)\\s+(?:uvula)\\s+(?:is\\s+|'s\\s+)?(?:swelling|swollen)\\b"), py_str("\\b(?:stung|stings?)\\b[^.?!]{0,40}\\b(?:dizzy|dizziness|faint\\w*|light\\s?headed|clammy)\\b"), py_str("\\b(?:dizzy|dizziness|faint\\w*|light\\s?headed|clammy)\\b[^.?!]{0,40}\\b(?:stung|stings?)\\b"), py_str("\\b(?:stung|stings?)\\b[^.?!]{0,40}\\b(?:throat|tongue)\\b[^.?!]{0,12}\\b(?:feels?|is|going|getting)\\s+(?:a\\s+bit\\s+|a\\s+little\\s+|really\\s+|very\\s+|so\\s+)?(?:funny|odd|weird|strange|thick|tickl\\w*|itch\\w*)\\b"), py_str("\\b(?:throat|tongue)\\b[^.?!]{0,12}\\b(?:feels?|is)\\s+(?:a\\s+bit\\s+|a\\s+little\\s+|really\\s+|very\\s+|so\\s+)?(?:funny|odd|weird|strange|thick|tickl\\w*|itch\\w*)\\b[^.?!]{0,40}\\b(?:stung|stings?)\\b"), py_str("\\b(?:throat|tongue)\\b[^.?!]{0,15}\\b(?:tickl\\w*|itches|itching)\\b[^.?!]{0,40}\\b(?:stung|stings?)\\b"), py_str("\\b(?:stung|stings?)\\b[^.?!]{0,40}\\b(?:throat|tongue)\\b[^.?!]{0,15}\\b(?:tickl\\w*|itches|itching)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.DOG_ATTACK"); }
    s_DOG_ATTACK(py_tuple(vec![py_str("\\bdogs?\\b.{0,30}\\b(?:attacking|mauling|savaging|biting|went\\s+for)\\b(?!\\s+(?:his|her|its|their|the|a)\\s+(?:paw|paws|tail|leg|legs|foot|feet|ear|ears|fur|skin|lead|fence|toy|bed|nappy|blanket|cushion))\\b"), py_str("\\bdogs?\\s+(?:has\\s+|have\\s+)?bit\\s+(?:me|my|his|her|him|them|a\\s+(?:child|kid|baby))\\b"), py_str("\\b(?:bit|bitten)\\s+by\\s+a\\s+dog\\b"), py_str("\\b(?:attacking|mauling)\\s+(?:him|her|them|me|my\\s+\\w+)\\b.{0,20}\\bdogs?\\b"), py_str("\\b(?:being\\s+)?attacked\\s+by\\s+(?:a\\s+)?dogs?\\b"), py_str("\\bdogs?\\b.{0,25}\\b(?:is|are|was|were)?\\s*(?:going\\s+for|after|lunging\\s+at|snapping\\s+at|growling\\s+at)\\s+(?:me|my|him|her|them)\\b"), py_str("\\bdogs?\\b[^.?!]{0,30}\\b(?:bit|bitten|biting|nipped|nipping|snapped\\s+at)\\b[^.?!]{0,25}\\b(?:me|us|my\\s+(?:hand|hands|arm|arms|leg|legs|foot|feet|face|finger|fingers|head|shoulder|wrist|ankle|neck|cheek|nose|toe|toes|chin))\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards._A_LOW_FIGURE"); }
    s__A_LOW_FIGURE(py_str("(?:0\\.[1-9]|[1-3](?:\\.\\d)?)(?:\\s*mmol)?\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards._A_READING_NOUN"); }
    s__A_READING_NOUN(py_str("(?:(?:blood\\s+)?sugars?|(?:blood\\s+)?glucose)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SUGAR_READING_LOW"); }
    s_SUGAR_READING_LOW(py_tuple(vec![op_add(&op_add(&op_add(&py_str("\\b"), &g__A_READING_NOUN()), &py_str("\\s+(?:levels?\\s+|readings?\\s+)?(?:is|was|are|at|reads?|reading|of|:)\\s*")), &g__A_LOW_FIGURE()), op_add(&py_str("\\bblood\\s+sugars?\\s+(?:levels?\\s+|readings?\\s+)?(?:of\\s+|:\\s*)?"), &g__A_LOW_FIGURE()), op_add(&py_str("\\b(?:blood|my|his|her|their|the)\\s+glucose\\s+(?:levels?\\s+|readings?\\s+)?(?:of\\s+|:\\s*)?"), &g__A_LOW_FIGURE()), op_add(&py_str("\\bglucose\\s+(?:levels?|readings?)\\s+(?:is\\s+|was\\s+|of\\s+|:\\s*)?"), &g__A_LOW_FIGURE())]));
    fn __data4() -> Py { { let mut __v: Vec<Py> = Vec::new(); __v.push(py_str("\\b(?:im|i'?m|am|is|are|'s|'re|hes|he'?s|shes|she'?s|theyre|they'?re|youre|you'?re|was|were)\\s+(?:having|getting|going\\s+into)\\s+(?:a\\s+|another\\s+)?hypo\\b")); __v.push(py_str("\\b(?:and|then|now)\\s+(?:(?:hes|he'?s|shes|she'?s|theyre|they'?re|im)\\s+)?(?:having|getting|going\\s+into)\\s+(?:a\\s+|another\\s+)?hypo\\b")); __v.push(py_str("\\bthink\\s+(?:i'?m|im|i\\s+am)\\s+(?:having|getting)\\s+(?:a\\s+|another\\s+)?hypo\\b")); __v.push(py_str("\\bconfus\\w*")); __v.push(py_str("\\bdisorient\\w*")); __v.push(py_str("\\bsweat\\w*")); __v.push(py_str("\\bclammy\\b")); __v.push(py_str("\\bshak\\w*")); __v.push(py_str("\\btrembl\\w*")); __v.push(py_str("\\bjitter\\w*")); __v.push(py_str("\\bdrows\\w*")); __v.push(py_str("\\b(?:very\\s+)?sleepy\\b")); __v.push(py_str("\\bslurr\\w*")); __v.push(py_str("\\bacting\\s+(?:drunk|odd|strange|weird|pissed|funny)\\b")); __v.push(py_str("\\bnot\\s+(?:him|her|them|himself|herself|themselves)\\b")); __v.push(py_str("\\bwobbly\\b")); __v.push(py_str("\\b(?:pale|grey|gray|white)\\s+as\\s+a\\s+sheet\\b")); __v.push(py_str("\\b(?:won'?t|wont)\\s+wake\\b")); __v.push(py_str("\\bfeels?\\s+(?:faint|funny|wobbly|drunk)\\b")); __v.push(py_str("\\bcan'?t\\s+stay\\s+awake\\b")); __v.push(py_str("\\b(?:passed|passing)\\s+out\\b")); __v.push(py_str("\\b(?:blacked|blacking)\\s+out\\b")); __v.push(py_str("\\bout\\s+cold\\b")); __v.push(py_str("\\bfaint\\w*")); __v.push(py_str("\\bcollaps\\w*")); __v.push(py_str("\\bkeel\\w*\\s+over\\b")); __v.push(py_str("\\ba\\s+fit\\b")); __v.push(py_str("\\bseizure\\w*")); __v.push(py_str("\\bconvuls\\w*")); __v.push(py_str("\\bcoma\\b")); __v.push(py_str("\\bunconscious\\b")); __v.push(py_str("\\bunresponsive\\b")); __v.push(py_str("\\bnot\\s+making\\s+(?:any\\s+)?sense\\b")); __v.push(py_str("\\bgibberish\\b")); __v.push(py_str("\\b(?:talking|talked|saying)\\s+(?:nonsense|rubbish)\\b")); __v.extend(py_iter_or_raise(&g_SUGAR_READING_LOW())); __v.push(py_str("\\b(?:blood\\s+)?(?:sugars?|glucose)\\s+(?:is\\s+|was\\s+|has\\s+|have\\s+|had\\s+|levels?\\s+(?:is\\s+|are\\s+)?)?(?:low|dropped|dropping|dipped|crashing|too\\s+low)\\b")); __v.push(py_str("\\b(?:blood\\s+)?(?:sugars?|glucose)\\s+level\\w*\\s+(?:is\\s+|are\\s+)?(?:low|dropping|dropped)\\b")); __v.push(py_str("\\b(?:blood\\s+)?glucose\\s+(?:is\\s+|was\\s+)?(?:falling|fallen|plummet\\w*)\\b")); py_tuple(__v) } }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HYPO_SIGN"); }
    s_HYPO_SIGN(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SELF_HARM_REPEAT_CAUSE"); }
    s_SELF_HARM_REPEAT_CAUSE(py_tuple(vec![py_str("\\b(?:boiler|wifi|wi-?fi|internet|router|broadband|signal|connection|car|van|engine|motorbike|bike|mower|lawnmower|laptop|phone|computer|screen|tv|telly|radio|fridge|freezer|fan|washing\\s+machine|dishwasher|dryer|heater|radiator|heating|oven|hob|cooker|grill|kettle|toaster|iron|straighteners|hairdryer|hair\\s+dryer|light|lights|bulb|plug|socket|charger|cable|battery|drill|saw|knife|blade|glass|bleach|acid|pan|pot|plate|shower|tap|mattress|brakes?|clutch|gears?|fuse|power|electricity|sun|sunburn|candle|lighter|fire|barbecue|barbeque|stove|radiator|ice|snow|shelf|shelves|cupboard|ceiling|door|desk|chair|sofa|table|worktop|counter|bed|floor|stairs|nails?)\\b"), py_str("\\b(?:eczema|psoriasis|dandruff|rash|itch\\w*|dry\\s+skin|allergy|allergic|hives|heating|in\\s+the\\s+night|at\\s+night|in\\s+my\\s+sleep)\\b"), py_str("\\b(?:lifting|carrying|moving|shifting|digging|painting|gardening|running|jogging|walking| cycling|swimming|exercising|training|playing|football|rugby|netball|gym|workout|squats?|deadlifts?|weights|boxes?|furniture|bags?|shopping|hoovering|cleaning|scrubbing|typing|at\\s+work|in\\s+the\\s+garden)\\b"), py_str("\\b(?:after\\s+(?:work|the\\s+gym|a\\s+run|a\\s+walk|lifting|football|netball|rugby|a\\s+shift|the\\s+shopping)|when\\s+i\\s+(?:lift|run|cycle|walk|play|train|work\\s+out|carry|exercise|climb|swim|dig|garden))\\b")]));
    fn __data5() -> Py { py_tuple(vec![py_str("\\bwhile\\s+(?:we|i|they|you)\\s+wait\\b"), py_str("\\bwhat\\s+(?:do|should)\\s+(?:i|we)\\s+do\\b"), py_str("\\bwhat\\s+now\\b"), py_str("\\banything\\s+else\\b"), py_str("\\bis\\s+(?:that|it|this)\\s+(?:ok|okay|right|normal|safe|dangerous)\\b"), py_str("\\bis\\s+it\\s+safe\\s+to\\b"), py_str("\\bthe\\s+ambulance\\b"), py_str("\\bthey'?re\\s+(?:coming|on\\s+their\\s+way)\\b"), py_str("\\bhelp\\s+is\\s+(?:coming|on\\s+the\\s+way)\\b"), py_str("\\bstill\\s+waiting\\b"), py_str("\\bshould\\s+i\\s+(?:use|give|move|leave|go|call|keep|do|put|take|wash|clean|rinse|cover|wrap|bandage)\\b"), py_str("\\bcan\\s+i\\s+(?:give|move|leave|go|use|do|put|take|wash|clean|rinse|cover|wrap|bandage|walk|stand|sit|lie|help|carry)\\b"), py_str("\\bwhat\\s+(?:else\\s+)?can\\s+i\\s+do\\b"), py_str("\\bkeep\\s+(?:them|him|her)\\s+(?:warm|still|awake)\\b"), py_str("\\b(?:cpr|dispatcher|call\\s+handler)\\b"), py_str("\\b(?:breathing|feels?|looks?|seems?)\\s+(?:better|easier|worse|normally|again)\\b"), py_str("\\bthey\\s+(?:seem|look|are)\\s+(?:better|worse|okay|ok)\\b"), py_str("\\b(?:i'm|im|we're|we\\s+are|i\\s+am)\\s+(?:outside|out|safe|clear)\\b"), py_str("\\b(?:fire\\s+(?:brigade|service|fighters?)|paramedics?|ambulance)\\b"), py_str("\\b(?:he|she|they|it)(?:'s|'re|\\s+is|\\s+are)\\s+(?:awake|breathing|okay|ok)\\b"), py_str("\\b(?:it|the\\s+bleeding)\\s+(?:has\\s+)?stopped\\b"), py_str("\\b(?:called|rang|phoned)\\s+(?:999|911|112)\\b"), py_str("\\bon\\s+(?:the\\s+phone|hold)\\b"), py_str("\\b(?:still|really)\\s+(?:scared|frightened|terrified|afraid|shaky|panicking|panicky)\\b"), py_str("\\bstill\\s+(?:soaking|bleeding|coming|pouring)\\b"), py_str("\\bsoaking\\s+through\\b"), py_str("\\bit'?s\\s+still\\s+\\w+ing\\b"), py_str("\\bshould\\s+i\\s+(?:try|open|shut|close|turn|grab|hold|press|cover|warm|sit|lie|stand|walk|drive|eat|drink|wake|ring|phone|tell|say|text|message|put|take|wash|clean|rinse|wrap|bandage)\\b"), py_str("\\bcan\\s+i\\s+(?:open|shut|close|turn|give|move|go|drive|eat|drink|wake|ring|phone|put|take|wash|clean|rinse|cover|wrap|bandage|walk|stand|sit|lie|help|leave|try|carry|hold|press)\\b"), py_str("\\bis\\s+it\\s+(?:ok|okay|safe|alright|fine)\\s+to\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.FOLLOW_UP"); }
    s_FOLLOW_UP(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("\\b(?:my|his|her|their|the|your)\\s+ms\\b"), py_str("\\bmultiple\\s+sclerosis\\b"), py_str("\\bparkinsons?\\b"), py_str("\\bparkinson'?s\\b"), py_str("\\b(?:brain|head|spinal)\\s+injur\\w+"), py_str("\\bcerebral\\s+palsy\\b"), py_str("\\bspina\\s+bifida\\b"), py_str("\\bmuscular\\s+dystrophy\\b"), py_str("\\bamputat\\w+\\b"), py_str("\\bprosthe\\w+\\s+(?:leg|legs|limb|limbs|knee|knees|ankle|ankles|foot|feet|hip|hips|thigh|thighs|calf|calves)\\b"), py_str("\\b(?:leg|legs|limb|limbs|knee|knees|ankle|ankles|foot|feet|hip|hips|thigh|thighs|calf|calves)\\s+prosthe\\w+"), py_str("(?<!eye )(?<!ocular )\\bprosthe\\w+\\b(?!\\s+(?:eye|eyes|eyeball|socket))"), py_str("\\bpolio\\b"), py_str("\\bdrop\\s+foot\\b"), py_str("\\bsince\\s+(?:he|she|they|i)\\s+(?:was|were)\\s+(?:little|born|a\\s+baby|small)\\b"), py_str("\\bsince\\s+(?:birth|childhood|i\\s+was\\s+born|he\\s+was\\s+born|she\\s+was\\s+born)\\b"), py_str("\\b(?:all|all\\s+of)\\s+(?:my|his|her|their)\\s+life\\b"), py_str("\\bborn\\s+(?:blind|with)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LIMP_ONGOING"); }
    s_LIMP_ONGOING(__data6());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SHOULD_I_CALL"); }
    s_SHOULD_I_CALL(py_tuple(vec![py_str("^[\\s,]*(?:so|ok|okay|right|well|then)?[\\s,]*(?:do\\s+you\\s+think|i\\s+think|honestly|really)?[\\s,]*(?:i|we)?\\s*(?:should|shall|do)\\s*(?:i|we)?\\s*(?:call|ring|phone|dial)\\s+(?:an?\\s+)?(?:ambulance|999|911|112|emergency\\s+services)\\b[\\s?!.]*(?:please|thanks|thank\\s+you|or\\s+not|yes\\s+or\\s+no)?[\\s?!.]*$"), py_str("^[\\s,]*(?:should|shall|do)\\s+(?:i|we)\\s+(?:call|ring|phone|dial)\\s+(?:an?\\s+)?(?:ambulance|999|911|112|emergency\\s+services)\\b[^?.!]*\\b(?:or|otherwise)\\b[^?.!]*[?!.]*$"), py_str("^[\\s,]*(?:is|isnt|isn'?t)\\s+(?:that|this|it)\\s+(?:really\\s+)?(?:a\\s+)?(?:999|911|112)\\s+job\\b[\\s?!.]*$"), py_str("^[\\s,]*(?:so|ok|okay|right|well|then)?[\\s,]*(?:do\\s+you\\s+think|i\\s+think|honestly|really)?[\\s,]*(?:should|shall|do)?\\s*(?:i|we)?\\s*(?:need\\s+to|have\\s+to|bother(?:\\s+\\w+)?)\\s*(?:calling|ringing|phoning|dialing|dialling|call|ring|phone|dial)?[\\s,]*\\s*(?:an?\\s+)?(?:ambulance|999|911|112|emergency\\s+services)\\b[\\s?!.]*(?:please|thanks|or\\s+not|yes\\s+or\\s+no)?[\\s?!.]*$"), py_str("^[\\s,]*(?:so|ok|okay|right|well|then)?[\\s,]*(?:do\\s+you\\s+think|i\\s+think|honestly|really)?[\\s,]*(?:do\\s+i\\s+|do\\s+we\\s+|i\\s+)?(?:need|needs|want|wants)\\s+(?:an?\\s+)?(?:ambulance|999|911|112|emergency\\s+services)\\b[\\s?!.]*(?:please|thanks|or\\s+not|yes\\s+or\\s+no)?[\\s?!.]*$")]));
    fn __data7() -> Py { py_tuple(vec![py_str("\\b(?:i\\s+)?(?:just\\s+)?(?:cant|can'?t|couldnt|couldn'?t)\\s+(?:do\\s+(?:this|it)|take\\s+(?:it|this)|go\\s+on|carry\\s+on|keep\\s+going|keep\\s+on)\\s*(?:any\\s?more|anymore)\\b"), py_str("\\b(?:i\\s+)?(?:just\\s+)?(?:don'?t|dont|do\\s+not)\\s+care\\s+(?:if\\s+i\\s+(?:live|die|wake|make\\s+it|survive)\\b|any\\s?more\\b|anymore\\b|about\\s+(?:anything|myself|it)\\b|whether\\s+i\\s+(?:live|die)\\b)"), py_str("\\b(?:i\\s+)?(?:just\\s+)?(?:feel|feeling|felt|am|im)\\s+(?:so\\s+|really\\s+|totally\\s+)?(?:empty|numb|hollow|dead\\s+inside)\\b"), py_str("\\bwish\\s+(?:i|it|that\\s+i|the\\s+car|that\\s+car)\\b[^.?!]{0,24}\\b(?:crashed|died|gone\\s+through|not\\s+woken|killed\\s+me|hit\\s+me|finished\\s+it|done\\s+it|succeeded|worked)\\b"), py_str("\\b(?:would|would'?ve|would\\s+have)\\s+(?:be\\s+)?(?:better|easier)\\s+if\\s+i\\b"), py_str("\\b(?:i\\s+)?(?:honestly\\s+)?(?:don'?t|dont)\\s+want\\s+to\\s+be\\s+here\\b"), py_str("\\bnot\\s+worth\\s+(?:being\\s+here|carrying\\s+on|going\\s+on)\\b"), py_str("\\b(?:havent|have\\s+not)\\s+(?:got\\s+)?(?:anything|much)\\s+left\\b"), py_str("\\b(?:i\\s+)?(?:really\\s+|just\\s+|honestly\\s+|so\\s+)?hate\\s+my\\s*self\\b"), py_str("\\bwish\\s+(?:i|that\\s+i)\\s+(?:wasnt|wasn'?t|werent|weren'?t|was\\s+not|were\\s+not)\\b"), py_str("\\b(?:want|wanted|wanna|wishing|wish)\\s+(?:to\\s+)?(?:disappear|vanish|stop\\s+existing|cease\\s+to\\s+exist|not\\s+exist|be\\s+gone|be\\s+dead|be\\s+here\\s+any\\s?more)\\b"), py_str("\\b(?:cant|can'?t|couldnt|couldn'?t)\\s+(?:take|stand)\\s+(?:it|this|much\\s+more)\\b"), py_str("\\bnot\\s+(?:wake|waking)\\s+up\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.NOT_JUST_A_NEAR_THING"); }
    s_NOT_JUST_A_NEAR_THING(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LIMP_PERSON"); }
    s_LIMP_PERSON(py_tuple(vec![py_str("\\b(?:i|he|she|they|we)\\s+(?:am|is|are|was|were|have\\s+been|has\\s+been)\\s+(?:still\\s+|now\\s+|really\\s+|badly\\s+|left\\s+)?limp(?:ing)?\\b"), py_str("\\bmy\\s+(?:son|daughter|child|boy|girl|kid|toddler|baby|husband|wife|partner|mum|mom|dad|mother|father|nan|grandad|grandma|grandson|granddaughter|niece|nephew|friend|sister|brother)\\s+(?:is|was|keeps?|has\\s+been|had\\s+been)\\s+(?:still\\s+)?limp(?:ing)?\\b"), py_str("\\b(?:son|daughter|child|boy|girl|kid|toddler|baby|husband|wife|partner|mum|mom|dad|mother|father|nan|nana|grandad|grandma|grandson|granddaughter|niece|nephew|friend|sister|brother)\\s+limp(?:s|ed)\\b"), py_str("\\b(?:i|he|she)\\s+limp(?:s|ed)\\b"), py_str("\\b(?:keeps?\\s+limping|started\\s+limping|stop\\s+limping|stopped\\s+limping|been\\s+limping|still\\s+limping|stop\\s+(?:\\w+\\s+){0,2}limping)\\b"), py_str("\\b(?:my|his|her|their|our)\\s+(?:\\w+\\s+){0,2}limp(?:ing)\\b"), py_str("\\b(?:has|have|had|has\\s+got|have\\s+got|had\\s+got|developed|developing|been\\s+left\\s+with)\\s+(?:a\\s+|an\\s+|this\\s+|that\\s+)?(?:bad\\s+|slight\\s+|really\\s+|sore\\s+)?limp\\b"), py_str("\\b(?:walks|walking|walked|hobbles|hobbling)\\s+with\\s+(?:a\\s+|this\\s+)?limp\\b"), py_str("\\bgot\\s+(?:a\\s+|this\\s+)?(?:bad\\s+|slight\\s+)?limp\\b"), py_str("\\bwith\\s+(?:a\\s+)?(?:bad\\s+|slight\\s+)?limp\\b"), py_str("\\b(?:leg|knee|ankle|foot|hip|back)\\s+(?:is\\s+)?limp(?:ing)?\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PURSUIT_FINE"); }
    s_PURSUIT_FINE(py_tuple(vec![py_str("\\b(?:to|with|for)\\s+(?:the\\s+)?(?:kids|children|child|dog|stuff|things|bags)\\b"), py_str("\\b(?:dropping|dropped|drop)\\s+(?:off|the|it|them)\\b"), py_str("\\b(?:picking|picked|pick)\\s+(?:up|the|them)\\b"), py_str("\\b(?:handover|hand\\s*over|collecting|collected|collect)\\b"), py_str("\\b(?:comes|coming|came)\\s+(?:to|for|round)\\b"), py_str("\\b(?:pick(?:ing|ed|s)?\\s+(?:me|us|him|her|them)\\s+up|pick\\s*-?\\s*up|lift\\s+(?:home|to\\s+work|back)|give\\s+(?:me|us|him|her|them)\\s+a\\s+lift|for\\s+a\\s+lift|to\\s+be\\s+let\\s+in|let\\s+(?:me|us|him|her)\\s+in|locked\\s+(?:out|himself\\s+out|herself\\s+out))\\b"), py_str("\\b(?:follow\\w*|stalk\\w*)\\s+(?:me|us)\\b[^.?!]{0,12}\\b(?:instagram|insta|facebook|fb|twitter|tiktok|socials?|social\\s+media|linkedin|youtube|reddit|snapchat|strava|whatsapp|threads)\\b"), py_str("\\b(?:husband|wife|partner|boyfriend|girlfriend|fiance|fiancee)\\b[^.?!]{0,30}\\b(?:outside|at|near|by)\\s+(?:the|her|his|their)\\s+(?:school|work|office|nursery|college|uni|university|gates?)\\b"), py_str("\\b(?:scared|frightened|worried|afraid|panicking|nervous)\\b[^.?!]{0,30}\\b(?:late|on\\s+time|in\\s+trouble)\\b"), py_str("\\b(?:shouting|yelling|screaming)\\s+at\\s+(?:the\\s+)?(?:football|telly|tv|match|game|radio|news|ref|referee)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LIMB_IS_A_THING"); }
    s_LIMB_IS_A_THING(py_tuple(vec![py_str("\\b(?:arm|leg)\\s+(?:of|on)\\s+(?:my|the|our|his|her|their|your)\\s+(?:[\\w-]+\\s+){0,2}(?:glasses|specs|spectacles|sunglasses|monitor|screen|telly|tv|lamp|light|tripod|mirror|chair|table|desk|sofa|settee|stool|bed|shelf|shelves|gate|door|bike|scooter|guitar|violin|rack|stand|pram|pushchair|buggy|jeans|trousers|pants|joggers|jogging bottoms|leggings|shorts|chinos|cords|combats|pyjamas|pyjama bottoms|pjamas|pjs|tights|skirt|dress|dresses|shirt|shirts|coat|coats|jacket|jackets|sock|socks|boot|boots|shoe|shoes|slipper|slippers)\\b"), py_str("\\b(?:glasses|specs|spectacles|sunglasses|monitor|screen|chair|table|desk|sofa|settee|stool|lamp|tripod|mirror|pram|pushchair|buggy|guitar|violin|bike|scooter|jeans|trousers|leggings|shorts|pyjamas|jacket|coat|shirt|dress)\\b(?:'s)?\\s+(?:arm|leg)\\b"), py_str("\\b(?:the|a|my|our|his|her|their|your)\\s+(?:glasses|specs|spectacles|sunglasses|chair|table|desk|sofa|settee|stool|lamp|monitor|screen|tripod|mirror|pram|pushchair|buggy|jeans|trousers|leggings|shorts|pyjamas|jacket|coat|shirt|dress)\\b[^.?!]{0,20}\\b(?:bent|deformed)\\b"), py_str("\\b(?:trouser|trousers|jeans|jean|pyjama|pyjamas|pajama|pajamas|jacket|shirt|coat|sleeve|chair|table|sofa|settee|desk|stool|lamp|monitor|screen|glasses|door|gate)\\s+(?:leg|legs|arm|arms)\\b")]));
    fn __data8() -> Py { py_tuple(vec![py_str("\\bi\\s+(?:just\\s+|was\\s+|were\\s+|am\\s+)?laugh(?:ing|ed|s)?\\b"), py_str("\\bi\\s+(?:just\\s+|was\\s+|were\\s+|am\\s+)?(?:only\\s+|just\\s+)?(?:joking|kidding|kiddin)\\b(?![^.?!]*\\bwe\\s+(?:both\\s+)?(?:laughed|were\\s+laughing|laughing)\\b)"), py_str("\\bwhile\\s+(?:i\\s+was\\s+)?(?:joking|kidding)\\b(?![^.?!]*\\bwe\\s+(?:both\\s+)?(?:laughed|were\\s+laughing|laughing)\\b)"), py_str("\\bit\\s+(?:stung|stings?|hurt|hurts|smarts?|throbs?|burn(?:s|ed)?)\\b"), py_str("\\bduring\\s+(?:the\\s+)?play\\b"), py_str("\\b(?:face|head|throat|neck|nose|eyes?|jaw|ribs?|stomach|belly|genitals?|chest)\\b"), py_str("\\b(?:hard|harder|really\\s+hurt|properly|painful|bruis\\w*|blood|bled|bleeding|mark)\\b"), py_str("\\b(?:afraid|scared|frightened|terrified)\\b"), py_str("\\b(?:wouldn'?t\\s+stop|didn'?t\\s+stop|kept\\s+(?:on|going)|not\\s+the\\s+first\\s+time|every\\s+day|all\\s+the\\s+time)\\b"), py_str("\\b(?:every\\s+(?:single\\s+)?(?:time|week|night|month|birthday)|each\\s+time|whenever|constantly|regularly|always\\s+(?:hits?|hit|slapp?(?:s|ped)?|punch(?:es|ed)?|kick(?:s|ed)?|shov(?:es|ed)|push(?:es|ed)|beats?|hurts?))\\b"), py_str("\\b(?:knife|belt|bottle|weapon|stick|pan|chair|bat)\\b"), py_str("\\b(?:said|says|saying|say|called|calls|calling|claimed|claims|claiming|told|tells|telling)\\b[^.?!]{0,25}\\b(?:joke|joking|laugh|laughing|fun|banter|mess(?:ing)?\\s+about)\\b"), py_str("\\b(?:it\\s+was|it\\s+were)\\s+(?:only\\s+|just\\s+|all\\s+)?(?:a\\s+)?(?:joke|joking|banter|laugh)\\b"), py_str("\\b(?:only|just)\\s+joking\\b"), py_str("\\b(?:down\\s+the\\s+stairs|downstairs|fell|fallen|falling|fell\\s+over|tripped\\s+me|knocked\\s+me\\s+(?:over|down)|off\\s+the\\s+(?:ladder|chair|bike)|into\\s+the\\s+road)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HIT_IN_FUN_HARD"); }
    s_HIT_IN_FUN_HARD(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("\\b(?:under|below)\\s+(?:the\\s+|his\\s+|her\\s+|their\\s+)?(?:water|surface)\\b"), py_str("\\b(?:fell|fallen|falls?|went|goes|gone|slipped|slips?|dropped|drops?|jumped|jumps?|pushed|pushed\\s+in|tipped|tumbled|sank|sunk|wa(s|s)\\s+in|were\\s+in|is\\s+in|went\\s+in)\\b[^.?!]{0,20}\\b(?:in|into)\\s+(?:the\\s+)?(?:water|pond|river|lake|sea|canal|pool|stream|reservoir|loch|harbour|harbor|lagoon|deep\\s+end|shallow\\s+end)\\b"), py_str("\\b(?:went\\s+under|goes\\s+under|gone\\s+under|disappeared\\s+under|underwater)\\b"), py_str("\\b(?:sank|sunk|sinking)\\s+(?:under|below|to\\s+the\\s+bottom)\\b"), py_str("\\b(?:sank|sunk|sinking)\\s+(?:in|into)\\s+the\\s+(?:water|pond|river|lake|sea|canal|pool|stream|reservoir|loch|harbour|harbor|lagoon)\\b"), py_str("\\b(?:slipped|slips?|fell|fallen|sank|sunk|tipped|tumbled|rolled)\\s+under\\b[^.?!]{0,25}\\b(?:bath|tub)\\b"), py_str("\\b(?:bath|tub)\\b[^.?!]{0,20}\\b(?:went|goes|gone|sank|sunk|slips?|slipped|fell|fallen)\\s+under\\b"), py_str("\\bunder\\s+for\\s+(?:\\d+|a\\s+few|several|two|three|four|five|ten|twenty|thirty)\\s*(?:seconds?|secs?|minutes?|mins?)\\b"), py_str("\\bface\\s+down\\b[^.?!]{0,20}\\b(?:water|bath|tub|pool|pond)\\b"), py_str("\\bin\\s+the\\s+water\\b[^.?!]{0,30}\\bcan'?t\\s+see\\b"), py_str("\\b(?:can'?t|cant|couldn'?t|could\\s+not)\\s+see\\b[^.?!]{0,25}\\b(?:in|under)\\s+(?:the\\s+)?(?:water|pool|sea|pond|river|lake|canal)\\b"), py_str("\\b(?:pulled|drags?|dragged|lifted|hauled|got|gets|getting|fished)\\s+(?:[\\w']+\\s+){0,3}out\\s+of\\s+the\\s+(?:water|pool|sea|pond|river|lake|canal)\\b"), py_str("\\bnearly\\s+drowned\\b[^.?!]{0,20}\\b(?:water|pond|river|lake|sea|pool|canal|reservoir|loch|deep\\s+end|bath|tub)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.WATER_DANGER"); }
    s_WATER_DANGER(__data9());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards._PET_DAMAGE_THING"); }
    s__PET_DAMAGE_THING(py_str("(?:sofas?|couches?|settees?|armchairs?|chairs?|stools?|seats?|curtains?|blinds?|carpets?|rugs?|mats?|bedding|duvet|pillows?|cushions?|blankets?|throws?|towels?|linen|clothes|jumpers?|cardigans?|shirts?|dress(?:es)?|trousers|jeans|skirts?|coats?|jackets?|sleeves?|socks?|tights|shoes?|slippers?|trainers?|boots?|laces?|shoelaces?|hats?|scarves?|bags?|rucksacks?|backpacks?|handbags?|wallets?|purses?|phones?|mobiles?|cables?|chargers?|wires?|leads?|laptops?|keyboards?|mice|remotes?|tvs?|televisions?|screens?|monitors?|headphones|speakers?|wallpaper|walls?|doors?|doorways?|doorframes?|skirtings?|paint|floors?|floorboards?|tables?|desks?|shelves|shelf|bookcases?|cupboards?|wardrobes?|cabinets?|worktops?|counters?|sideboards?|garden|lawn|grass|fences?|gates?|sheds?|decking|patios?|plants?|flowers?|houseplants?|trees?|bark|scratch\\s*posts?|posts?|toys?|tunnels?|carpet|furniture|woodwork|paintwork|bannister|banister|radiator|switch|light\\s*switch)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BLEEDING_SMALL"); }
    s_BLEEDING_SMALL(py_tuple(vec![py_str("\\b(?:he|she|they|his|her|their|my\\s+(?:dad|mum|mom|father|mother|son|daughter|child|kid|baby|husband|wife|partner|brother|sister|nan|gran|grandma|grandad|friend|neighbour|neighbor|boy|girl|man|woman|lad|mate|colleague))\\b.{0,25}\\b(?:is|are|'s|was|keeps?|has\\s+been)\\s+bleeding\\b(?!\\s+(?:me\\s+)?(?:dry|white))"), py_str("\\b(?:my|his|her|their|your)\\s+(?:hand|arm|leg|foot|head|finger|thumb|wrist|knee|elbow|face|lip|scalp|shin|shoulder|ear|cheek|mouth|brow|eyebrow|chin|jaw|forearm|upper\\s+arm|thigh|calf|palm|heel|toe|knuckle|knuckles|belly|chest|neck|back)\\s+(?:is\\s+|are\\s+|keeps?\\s+|started\\s+|won'?t\\s+stop\\s+)?(?:bleeding|bled)\\b"), py_str("\\b(?:cut|sliced|nicked|gashed|slashed)\\b.{0,25}\\b(?:won'?t|wont|doesn'?t|didn'?t|keeps?)\\s+stop\\s+bleeding\\b"), op_add(&op_add(&py_str("\\b(?:dogs?|cats?|pupp(?:y|ies)|kittens?|pets?)\\b.{0,25}\\b(?:bit|bitten|scratched|clawed|claw|nicked)\\b.{0,20}\\b(?:(?:me|us)\\b|(?:him|her|them)\\b(?!\\s+\\w)|(?:my|his|her|their|your|our)\\s+(?!(?:\\w+['’]s\\s+)?(?:[a-z]+\\s+){0,2}(?:"), &g__PET_DAMAGE_THING()), &py_str(")\\b)\\w)"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.A_TOOTH_OWNER"); }
    s_A_TOOTH_OWNER(py_str("(?:my|his|her|their|the|your|our|a|an|son'?s|daughter'?s|kid'?s|child'?s|husband'?s|wife'?s|partner'?s|friend'?s|mum'?s|dad'?s)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.TOOTH_OUT"); }
    s_TOOTH_OUT(py_tuple(vec![op_add(&op_add(&py_str("\\b"), &g_A_TOOTH_OWNER()), &py_str("\\s+(?:front\\s+|back\\s+|adult\\s+|baby\\s+|milk\\s+)?(?:tooth|teeth)\\b[^.?!]{0,20}\\b(?:knock(?:ed|s|ing)?\\s+out|came\\s+out|comes?\\s+out|fell\\s+out|falls?\\s+out|is\\s+hanging)\\b")), py_str("\\b(?:knock|knocks|knocked|knocking|kick|kicks|kicked|kicking|punch|punches|punched|punching|smash|smashes|smashed|smashing)\\s+(?:my|his|her|their|our|your|a|an|out\\s+)\\s*(?:front\\s+|back\\s+)?tooth\\b"), py_str("\\b(?:knock|knocks|knocked|knocking|kicking|kicked|punch|punched|punching|smash|smashed|smashing)\\s+(?:my|his|her|their|the|our|your|out\\s+)\\b[^.?!]{0,18}\\bteeth\\b"), py_str("\\b(?:knocked|came|fell)\\s+(?:out\\s+)?(?:\\w+\\s+){0,4}(?:teeth|tooth)\\b"), py_str("\\b(?:teeth|tooth)\\s+(?:got\\s+|were\\s+|was\\s+|both\\s+)?(?:knocked|came|fell)\\s+out\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.A_TOOTH_DAMAGED"); }
    s_A_TOOTH_DAMAGED(py_str("(?:chips?|chipped|chipping|cracks?|cracked|cracking|breaks?|breaking|broke|broken|busts?|busted|busting|snaps?|snapped|snapping)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.TOOTH_CHIPPED"); }
    s_TOOTH_CHIPPED(py_tuple(vec![op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_A_TOOTH_OWNER()), &py_str("\\s+(?:front\\s+|back\\s+|baby\\s+|adult\\s+)?(?:tooth|teeth)\\b[^.?!]{0,20}\\b")), &g_A_TOOTH_DAMAGED()), &py_str("\\b(?!\\s*out)")), op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_A_TOOTH_DAMAGED()), &py_str("\\s+")), &g_A_TOOTH_OWNER()), &py_str("\\s+(?:front\\s+|back\\s+|baby\\s+|adult\\s+)?tooth\\b(?!\\s*out)"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.WEIGHT_JOINT"); }
    s_WEIGHT_JOINT(py_tuple(vec![py_str("\\b(?:ankle|ankles|knee|knees|foot|feet|heel|heels|toe|toes|leg|legs|wrist|wrists|hand|hands|elbow|elbows|shoulder|shoulders|finger|fingers|thumb|thumbs)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BODY"); }
    s_BODY(py_str("(?:hands?|fingers?|fingertips?|thumbs?|arms?|wrists?|elbows?|tongue|mouth|throat|lips?|faces?|cheeks?|nose|ears?|eyes?|feet|foot|toes?|legs?|knees?|shins?|chest|backs?|skin|palms?|knuckles?|head|scalp|neck|stomach|belly)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SCORCHED_A_PERSON"); }
    s_SCORCHED_A_PERSON(py_tuple(vec![op_add(&py_str("\\b(?:scorch(?:ed|es|ing)?|singe(?:d|s|ing)?|scold(?:ed|ing))\\s+(?:my|his|her|their|your|our|the|a)\\s+(?:(?:left|right|own|whole|poor|other|inner|top)\\s+)?"), &g_BODY()), op_add(&py_str("\\b(?:a|an|the|this|that)\\s+(?:bad|nasty|small|minor|little|sore|red|fresh|ugly)?\\s*(?:scorch|singe)(?:\\s+marks?)?\\s+(?:on|to)\\s+(?:my|his|her|their|your|the)\\s+"), &g_BODY()), op_add(&op_add(&py_str("\\bmy\\s+"), &g_BODY()), &py_str("\\s+(?:is|are|was|were|got)\\s+(?:scorched|singed)\\b")), op_add(&py_str("\\b(?:scorch(?:ed|es|ing)?|singe(?:d|s|ing)?|scold(?:ed|ing))\\s+(?:the\\s+)?(?:side|back|top|front|inside|outside|roof|tip)\\s+of\\s+(?:my|his|her|their|your|our)\\s+"), &g_BODY()), py_str("\\b(?:got|get|gets)\\s+(?:scorched|singed)\\b[^.?!]{0,25}\\b(?:by|on|from|with)\\s+(?:the\\s+|a\\s+|an\\s+|my\\s+|his\\s+|her\\s+|some\\s+)?(?:steam|sun|iron|hob|grill|oven|fire|flames?|heat|barbecue|bbq|pan|kettle|hot\\s+water|oil|saucepan|toaster|sand)\\b"), op_add(&py_str("(?!(?:.*?)\\bkettle\\b)\\b(?:iron|pan|saucepans?|frying\\s+pan|chip\\s+pan|grill|oven|tray|baking\\s+tray|pot|fryer|toaster|jug|hotplate|cooker)\\b[^.?!]{0,25}\\b(?:fell|fall|falls|dropped|tipped|tips|slipped|slid|touched|touching|came\\s+down|knocked\\s+over)\\b[^.?!]{0,25}\\b(?:on|onto|over|into|across)\\s+(?:my|his|her|their|our|your)\\s+"), &g_BODY()), op_add(&py_str("(?!(?:.*?)\\bkettle\\b)\\b(?:iron|pan|saucepan|hob|grill|oven|pot|fryer|toaster|tray)\\b[^.?!]{0,15}\\b(?:touched|touching|brushed|grazed|nicked|caught)\\b\\s+(?:my|his|her|their|our|your)\\s+"), &g_BODY()), op_add(&py_str("\\b(?:oil|fat|grease)\\b[^.?!]{0,15}\\b(?:splash(?:ed|es)?|spat|spits?|popped|sprayed|jumped)\\b[^.?!]{0,20}\\b(?:on|onto|over)\\s+(?:my|his|her|their|our|your)\\s+"), &g_BODY()), py_str("\\b(?:scorch(?:ed|es|ing)?|singe(?:d|s|ing)?|scold(?:ed|ing))\\s+(?:me|my\\s*self|your\\s*self|him\\s*self|her\\s*self|them\\s*selves)\\b[^.?!]{0,20}\\b(?:on|with)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BURN_ON_BODY"); }
    s_BURN_ON_BODY(py_str("(?:hands?|fingers?|fingertips?|thumbs?|arms?|wrists?|elbows?|tongue|mouth|throat|lips?|faces?|cheeks?|nose|ears?|eyes?|feet|foot|toes?|legs?|knees?|shins?|chest|backs?|skin|palms?|knuckles?|head|scalp|neck|stomach|belly)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.NOT_BURN_ON_BODY"); }
    s_NOT_BURN_ON_BODY(op_add(&op_add(&py_str("(?:on|to|in|across)\\s+(?:(?:my|his|her|their|your|our|the|a|this|that)\\s+)?(?!"), &g_BURN_ON_BODY()), &py_str("|me\\b|him\\b|her\\b|them\\b|us\\b|myself\\b|yourself\\b)\\w")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HOT_SURFACE"); }
    s_HOT_SURFACE(py_str("\\b(?:(?:hot\\s+)?(?:hob|grill|oven\\s+door|oven\\s+shelf|hotplate|cooker\\s+ring)|(?:hot|boiling|scalding|burning)\\s+(?:iron|pan|kettle|saucepan|dish|tray|oil|fat|spoon|tin|water))\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HOT_COMFORT"); }
    s_HOT_COMFORT(py_str("\\b(?:hot\\s+water\\s+bottl\\w*|ice\\s+pack|heat\\s+pack|wheat\\s+bag|hot\\s+towel|hot\\s+drink|hot\\s+chocolate|hot\\s+bath|hot\\s+shower|hot\\s+water\\s+tap)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CUT_BODY_PART"); }
    s_CUT_BODY_PART(py_str("(?:hands?|fingers?|fingertips?|thumbs?|arms?|wrists?|elbows?|lips?|faces?|cheeks?|nose|ears?|eyes?|forehead|feet|foot|toes?|legs?|knees?|shins?|chest|palms?|knuckles?|head|scalp|neck|stomach|belly|thighs?|calves|calf|heels?|shins?|ankles?|shoulders?|forearms?|skin)s?\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_OWNS_THE_WOUND"); }
    s_PET_OWNS_THE_WOUND(py_str("(?:my|our|the|his|her|their)\\s+(?:dog|dogs|cat|cats|puppy|puppies|kitten|kittens|pup|pet|parrot|budgie|hamster|rabbit|bunny|horse|pony|guinea\\s+pig|tortoise|ferret|gerbil|rat|lizard|snake|hen|chicken)\\b[^.?!]{0,25}\\b(?:has|have|had|got|with|cut|is|was)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.NOT_A_PET_LINE"); }
    s_NOT_A_PET_LINE(py_str("\\A(?!.*\\b(?:dog|dogs|puppy|pup|pups|cat|cats|kitten|kittens|rabbit|hamster|guinea|horse|pony|bird|parrot|pet|pets|vet|paw|paws|hoof|fur|tail|beak|snake|lizard)\\b).*?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CUT_ON_BODY"); }
    s_CUT_ON_BODY(py_str("^(?!.*\\b(?:dog|dogs|puppy|pup|pups|cat|cats|kitten|kittens|rabbit|hamster|guinea|horse|pony|bird|parrot|pet|pets|vet|paw|paws|hoof|fur|tail|beak|snake|lizard)\\b).*?\\b(?:have|has|had|got|get|gets|getting|gave|gives?|with|from)\\b[^.?!]{0,25}\\b(?:cut|gash|graze|scrape|scratch|nick)\\s+(?:on|to|in|above|below)\\s+(?:my|his|her|their|your|the)\\s+(?:hands?|fingers?|fingertips?|thumbs?|arms?|elbows?|lips?|faces?|cheeks?|nose|ears?|eyes?|eyebrows?|forehead|feet|foot|toes?|legs?|knees?|shins?|chest|backs?|palms?|knuckles?|head|scalp|stomach|belly|thighs?|calves|calf|heels?|ankles?|shoulders?|hips?)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CUT_ON_THROAT_BODY"); }
    s_CUT_ON_THROAT_BODY(py_str("\\b(?:cut|gash|scratch|graze|scrape|nick|slice)\\b(?:\\s+(?:on|in|to|across)\\s+(?:my|his|her|their|the|your)\\s+|\\s+(?:my|his|her|their|your)\\s+|\\s+)(?:wrists?|neck|throat)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.RING_STUCK"); }
    s_RING_STUCK(py_tuple(vec![py_str("\\bring\\b"), py_str("\\bwedding\\s+ring\\b"), py_str("\\bengagement\\s+ring\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_PASSES_BLOOD"); }
    s_PET_PASSES_BLOOD(py_str("[^.?!]*\\b(?:wee\\w*|urine|urinating|pee\\w*|piss\\w*|stool|poo\\w*|vomit\\w*|sick|diarrh\\w*|tummy|gut)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_WOUND"); }
    s_PET_WOUND(py_tuple(vec![op_add(&op_add(&op_add(&op_add(&py_str("\\A(?=.*"), &g_PET_OWNS_THE_WOUND()), &py_str(")(?!")), &g_PET_PASSES_BLOOD()), &py_str(").*?\\b(?:cut|gash|graze|scrape|scratch|nick|wound|sore|blister|bleed\\w*|bled|blood)\\b"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_ANIMAL_HURT"); }
    s_PET_ANIMAL_HURT(py_tuple(vec![py_str("\\b(?:my|our|the|his|her|their)\\s+(?:dogs?|cats?|pupp(?:y|ies)|kittens?|pup|pets?|parrot|budgie|hamster|rabbit|bunny|horse|pony|guinea\\s+pig|tortoise|ferret|gerbil|rats?|lizard|snake|hen|chicken)\\b[^.?!]{0,32}?\\b(?:burnt|burned|scalded|grazed|cut|nicked|splinter|blister\\w*|broke|broken|bleed\\w*|bled|blood|wound|gash\\w*|scraped|scrapes)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_OWNS_A_THING"); }
    s_PET_OWNS_A_THING(py_tuple(vec![py_str("\\b(?:dogs?|cats?|pupp(?:y|ies)|kittens?|pup|pets?|parrot|budgie|hamster|rabbit|bunny|horse|pony|guinea\\s+pig|tortoise|ferret|gerbil|rats?|lizard|snake|hen|chicken)(?:'s)?\\s+(?:flap|lead|leash|harness|collar|tag|bowl|bed|basket|blanket|toy|toys|coat|jacket|gate|door|hutch|kennel|cage|tank|claw|claws|nail|nails)\\b"), py_str("\\b(?:broke|broken|split|torn|lost|caught|trimmed|clipped)\\s+(?:a|his|her|its|the)\\s+(?:nail|nails|claw|claws)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HUMAN_OWN_INJURY"); }
    s_HUMAN_OWN_INJURY(py_tuple(vec![py_str("\\b(?:my|our|his|her|their)\\s+(?:hand|hands|finger|fingers|thumb|thumbs|wrist|wrists|arm|arms|elbow|shoulder|knee|shin|calf|ankle|foot|feet|toe|toes|face|cheek|lip|nose|head|back|chest|stomach|leg|legs)\\b[^.?!]{0,25}?\\b(?:burnt|burned|scalded|sprained|grazed|cut|nicked|blister\\w*|broke|broken|bleed\\w*|bled|blood)\\b"), py_str("\\b(?:burnt|burned|scalded|sprained|grazed|cut|nicked|scratched|bit|bitten|nipped|broke|broken)\\b[^.?!]{0,20}?\\b(?:my|our|their)\\s+(?:hand|hands|finger|fingers|thumb|thumbs|wrist|wrists|arm|arms|elbow|shoulder|knee|shin|calf|ankle|foot|feet|toe|toes|face|cheek|lip|nose|head|back|chest|stomach|leg|legs)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_NAIL"); }
    s_PET_NAIL(py_tuple(vec![py_str("\\b(?:my|our|the|his|her|their)\\s+(?:dogs?|cats?|puppy|puppies|kitten|kittens|pup|pet|parrot|budgie|rabbit|bunny|hamster|horse|pony|guinea\\s+pig|ferret|gerbil|lizard)\\b[^.?!]{0,30}?\\b(?:cut|clipped|trimmed|nicked|caught|tore|torn|snapped)\\b[^.?!]{0,20}?\\b(?:nail|nails|claw|claws|quick)\\b"), py_str("\\b(?:my|our|the|his|her|their)\\s+(?:dogs?|cats?|puppy|puppies|kitten|kittens|pup|pet)'?s?\\s+(?:nail|nails|claw|claws|quick)\\b[^.?!]{0,30}?\\b(?:too\\s+short|cut\\s+too|clipped\\s+too|trimmed\\s+too|nicked|torn|snapped|split)\\b"), py_str("\\bi\\s+(?:cut|clipped|trimmed|nicked)\\s+(?:the\\s+|his\\s+|her\\s+|its\\s+)?quick\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BURN_VERB"); }
    s_BURN_VERB(py_str("(?:burnt|burned|burn|scald(?:ed|s)?|scorch(?:ed|es|ing)?|singe(?:d|s|ing)?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SIGHT_EVER"); }
    s_SIGHT_EVER(py_tuple(vec![py_str("\\b(?:sight|eyesight|vision|eye|eyes)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.SIGHT_LIFE_REPLIES"); }
    s_SIGHT_LIFE_REPLIES(py_tuple(vec![py_str("Sight that has always been like that is not an emergency, but it is worth a proper eye test: an optician can test it and your GP can refer you, and there is practical help too - a certificate for sight loss and the support that comes with it. If it has changed even a little this week, that is a same-day call instead.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BURNT_BODY_VERB"); }
    s_BURNT_BODY_VERB(py_tuple(vec![py_str("\\b(?:my|his|her|their|your|our)\\s+(?:faces?|skin|hands?|arms?|legs?|neck|chest|shoulders?|backs?|tummy|belly|feet|foot|fingers?|fingertips?|thumbs?|thighs?|palms?|wrists?|lips?|ears?|cheeks?|chin|forehead|scalp|eyebrows?|knees?|elbows?|calves|calf|shins?|toes?)\\b[^.?!]{0,15}\\b(?:is|are|was|were|got|gets|has\\s+got|have\\s+got|looks?)\\s+(?:all\\s+)?(?:burnt|burned|scalded|blister\\w*)\\b(?![^.?!]{0,20}\\b(?:from|because\\s+of|due\\s+to|off)\\s+(?:the\\s+|all\\s+the\\s+)?(?:cry\\w*|tears?|weep\\w*|sob\\w*)\\b)"), py_str("\\b(?:my|his|her|their|your|our)\\s+(?:faces?|skin|hands?|arms?|legs?|neck|chest|shoulders?|backs?|tummy|belly|feet|foot|fingers?|fingertips?|thumbs?|thighs?|palms?|wrists?|lips?|ears?|cheeks?|chin|forehead|scalp|knees?|elbows?|shins?|toes?)\\b[^.?!]{0,15}\\b(?:is|are|was|were|got|gets|looks?)\\s+(?:all\\s+)?red\\s+raw\\b(?=[^.?!]{0,30}\\b(?:sun|sunburn\\w*|sunburnt|hot|boil\\w*|oven|kettle|pan|steam|steam\\w*|iron|wax|scald\\w*|fire|flame\\w*|barbecue|bbq|cooker|hob|radiator|heat\\w*|heated|straightener\\w*|blow\\s+dry\\w*|hot\\s+water)\\b|\\Z)"), py_str("\\bburn(?:t|ed)\\b[^.?!]{0,10}\\b(?:my|his|her|their|your|our)\\s+(?:hands?|fingers?|fingertips?|thumbs?|arms?|wrists?|lips?|faces?|cheeks?|nose|ears?|legs?|knees?|chest|skin|neck|chin|forehead|eyebrows?|scalp|back|thighs?|feet|foot|toes?|stomach|belly|shoulders?)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.WAX_BODY"); }
    s_WAX_BODY(py_str("(?:hands?|fingers?|fingertips?|thumbs?|arms?|wrists?|legs?|thighs?|knees?|feet|foot|toes?|lips?|faces?|cheeks?|chin|skin|stomach|belly|chest|neck|back|shoulders?|ears?)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HOT_WAX_ON_BODY"); }
    s_HOT_WAX_ON_BODY(py_tuple(vec![op_add(&py_str("\\b(?:hot|melted|molten|warm|boiling)\\s+wax\\b[^.?!]{0,25}\\b(?:on|onto|over|down|across)\\b[^.?!]{0,15}\\b(?:my|his|her|their|your|our)\\s+"), &g_WAX_BODY()), op_add(&py_str("\\b(?:wax|waxing\\s+wax|wax\\s+pot|wax\\s+burner)\\b[^.?!]{0,25}\\b(?:spilt|spilled|dripped|dribbled|poured|splashed|leaked|came\\s+off)\\b[^.?!]{0,25}\\b(?:my|his|her|their|your|our)\\s+"), &g_WAX_BODY())]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PART_OFF_PART"); }
    s_PART_OFF_PART(py_str("(?:fingers?|fingertips?|thumbs?|toes?|hands?|feet|foot|arms?|legs?|ears?|noses?|lips?|tongues?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PART_OFF_NOT"); }
    s_PART_OFF_NOT(py_str("\\A(?!.*\\b(?:prosthe\\w+|false|glass|denture|dentures|nails?|toenails?|fingernails?|hair|tooth|teeth|wig|hairpiece|rings?|earrings?|bracelet|watch|plaster|cast|bandages?|dressings?|brace|retainer|contacts?|tags?|labels?|buttons?|sticker|stuck\\s+on)\\b).*?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PART_OFF"); }
    s_PART_OFF(py_tuple(vec![op_add(&op_add(&op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:my|his|her|their|your|our)\\s+")), &g_PART_OFF_PART()), &py_str("\\b[^.?!]{0,12}\\b(?:hanging\\s+off|hanging\\s+by|hanging\\s+on\\s+by|half\\s+off|hanging\\s+loose|hangs?\\s+off)\\b")), op_add(&op_add(&op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:my|his|her|their|your|our)\\s+")), &g_PART_OFF_PART()), &py_str("\\b[^.?!]{0,15}\\b(?:has|have|had|was|were|is|are|got|been)?\\s*(?:been\\s+)?(?:cut|chopped|sliced|severed|taken|took|bitten|ripped|torn|broke|snapped)\\s+off\\b")), op_add(&op_add(&op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:cut|sliced|chopped|severed|took|nipped|bit|ripped|tore)\\s+(?:the\\s+)?(?:top|tip|end|part)\\s+of\\s+(?:my|his|her|their|your)\\s+")), &g_PART_OFF_PART()), &py_str("\\s+off\\b")), op_add(&op_add(&op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:cut|sliced|chopped|severed|amputated)\\s+(?:my|his|her|their|your)\\s+")), &g_PART_OFF_PART()), &py_str("\\s+off\\b")), op_add(&op_add(&op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:severed|amputated)\\s+(?:my|his|her|their|your)\\s+")), &g_PART_OFF_PART()), &py_str("\\b")), op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:the\\s+)?(?:top|tip|end)\\s+of\\s+(?:my|his|her|their|your)\\s+(?:fingers?|thumbs?|toes?|noses?|ears?)\\s+(?:came|come|has\\s+come|had\\s+come|is|was)\\s+off\\b")), op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:my|his|her|their|your)\\s+(?:hands?|arms?|fingers?)\\s+(?:got|was|is|were|has\\s+been|had\\s+been)?\\s*(?:caught|trapped|stuck|pulled|dragged|crushed)\\s+(?:in|into|under)\\s+(?:the|a)\\s+(?:machine|machinery|press|lathe|blender|mincer|saw|chipper|shredder|mower|drill|mixer|guillotine|baler|combine)\\b")), op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:got|get|put|puts|had|has|have|stuck|shoved|pushed|caught)\\s+(?:my|his|her|their|your)\\s+(?:hands?|arms?|fingers?)\\s+(?:in|into|under)\\s+(?:the|a)\\s+(?:machine|machinery|press|lathe|blender|mincer|saw|chipper|shredder|mower|drill|mixer|guillotine|baler|combine)\\b")), op_add(&op_add(&op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:my|his|her|their|your)\\s+")), &g_PART_OFF_PART()), &py_str("\\b[^.?!]{0,12}\\b(?:mashed|mangled|crushed|pulped|squashed\\s+flat|flattened)\\b")), op_add(&g_PART_OFF_NOT(), &py_str("\\b(?:crushed|smashed|squashed)\\s+(?:my|his|her|their|your)\\s+(?:fingers?|thumbs?|hands?|toes?|feet|foot)\\b"))]));
    fn __data11() -> Py { py_tuple(vec![py_str("\\b(?:cut|cuts|sliced|slices|nicked|nicks|grazed|grazes|scraped|scrapes)\\s+(?:my|his|her|their|the)\\s+(?:finger|thumb|hand|arm|leg|foot|toe|knee|head|face|lip|eye|ear|tongue|palm|heel|knuckle|shin|elbow|fingertip)s?\\b"), py_str("\\b(?:scratched|scratches)\\s+(?:my|his|her|their|the)\\s+(?:finger|thumb|hand|arm|leg|foot|toe|knee|head|face|lip|ear|tongue|palm|heel|knuckle|shin|elbow|fingertip|forearm|forehead|chin)s?\\b"), op_add(&op_add(&py_str("\\A(?!.*"), &g_PET_OWNS_THE_WOUND()), &py_str(").*?\\b(?:cut|gash|scratch|graze|scrape|nick|slice)\\b(?:\\s+(?:on|in|to|across)\\s+(?:my|his|her|their|the|your)\\s+|\\s+(?:my|his|her|their|your)\\s+|\\s+)(?:finger|thumb|hand|arm|leg|foot|toe|knee|head|face|lip|eye|ear|tongue|palm|heel|knuckle|shin|elbow|fingertip|forearm|forehead|chin)s?\\b")), py_str("\\bpaper\\s+cut\\b"), py_str("\\bcut\\s+my\\s*self\\b"), py_str("\\b(?:cut|cutting|nicked|sliced|grazed|scraped)\\s+(?:him|her|them)sel(?:f|ves)\\b"), g_CUT_ON_BODY(), py_str("\\b(?:my|his|her|their|your|the)\\s+(?:cut|gash|graze|scrape|scratch)\\b[^.?!]{0,20}\\bbleeding\\b"), op_add(&op_add(&g_NOT_A_PET_LINE(), &py_str("\\b(?:a|an|this|that|my|his|her|their)\\s+(?:nasty|deep|bad|sore|bleeding|weeping)?\\s*(?:cut|gash|graze|scrape|scratch|nick|slice)\\s+(?:on|in|to|across|near)\\s+(?:my|his|her|their|your)\\s+")), &g_CUT_BODY_PART()), op_add(&g_NOT_A_PET_LINE(), &py_str("\\b(?:have|has|got|get|had)\\s+(?:a|an|this|that)\\s+(?:small|little|nasty|deep|bad|tiny|sore|long|clean|weeping)\\s+(?:cut|gash|graze|scrape|scratch|nick)\\b")), op_add(&op_add(&g_NOT_A_PET_LINE(), &py_str("\\b(?:have|has|had|got|get)\\s+(?:a|an)\\s+(?:cut|gashed|grazed|scraped|scratched)\\s+")), &g_CUT_BODY_PART()), py_str("(?!(?:.*?)\\b(?:car|cars|van|bike|motorbike|wheel|wheels|bumper|alloy|paint|door|doors|wall|table|worktop)\\b)\\b(?:have|had|got)\\s+(?:a|an)\\s+(?:graze|scrape|gash)\\b"), py_str("\\b(?:nicked|sliced|grazed|scratched|scraped|caught)\\s+my\\s*self\\b"), py_str("\\b(?:nicked|sliced|grazed|scratched|scraped|caught)\\s+(?:him|her|them)sel(?:f|ves)\\b")]) }
    fn __data12() -> Py { { let mut __v: Vec<Py> = Vec::new(); __v.push(op_add(&op_add(&py_str("\\b"), &g_BURN_VERB()), &py_str("\\s+(?:my|his|her|their|me|myself|yourself|himself|herself|themselves)\\b(?!\\s+(?:bridges|candle\\s+at|holes?|midnight|hair|fringe|bangs|ponytail|plait|braid)\\b)"))); __v.extend(py_iter_or_raise(&g_BURNT_BODY_VERB())); __v.push(py_str("\\b(?:small|minor|little)\\s+(?:burn|scald)s?\\b")); __v.push(py_str("\\bblister\\w*\\s+(?:from|off|after)\\s+(?:the\\s+|a\\s+|my\\s+|his\\s+|her\\s+|their\\s+)?(?:kettle|oven|hob|grill|toaster|iron|pan|fryer|barbecue|bbq|fire|flames?|steam|boiling\\s+water|hot\\s+water|hot\\s+oil|hot\\s+fat|hot\\s+pan)\\b")); __v.push(py_str("\\b(?:burn|burnt|burned|burning|scalds?|scalded|scalding)\\b[^.?!]{0,25}\\bblister\\w*\\b")); __v.push(geti(&g_HOT_WAX_ON_BODY(), &Py::Int(0))); __v.push(geti(&g_HOT_WAX_ON_BODY(), &Py::Int(1))); __v.push(op_add(&op_add(&py_str("(?!(?:.*?)\\b"), &g_NOT_BURN_ON_BODY()), &py_str(")\\b(?:got|get|have|has|had)\\s+(?:a|an|this|that|some)\\s+(?:bad|nasty|nasty|small|minor|little|sore|red|nasty)*\\s*(?:burn|scald)s?\\b"))); __v.push(py_str("\\b(?:a|the|this|that)\\s+(?:bad|nasty|small|minor|little|sore|red)?\\s*burn\\s+(?:from|off)\\b")); __v.push(op_add(&py_str("\\b(?:a|the|this|that)\\s+(?:bad|nasty|small|minor|little|sore|red)?\\s*burn\\s+(?:on|to)\\s+(?:my|his|her|their|your|the)\\s+"), &g_BURN_ON_BODY())); __v.push(py_str("\\b(?:burnt|burned|scalded|scald|burn)\\s+(?:the\\s+roof\\s+of\\s+my|inside\\s+of\\s+my)\\b")); __v.push(op_add(&op_add(&py_str("\\b"), &g_BURN_VERB()), &py_str("\\s+(?:my|his|her|their|me|on\\s+my)\\s+(?:hand|finger|arm|wrist|thumb|tongue|mouth|foot|leg)s?\\b"))); __v.push(py_str("\\b(?:put|putting|puts|placed?|placing|rest(?:ed|ing)?|touch(?:ed|ing)?|touches|grab(?:bed|bing)?|grabs|picked?\\s+up|lean(?:ed|t|ing)?\\s+on|sat\\s+on|slipped)\\s+(?:my|his|her|their)\\s+(?:hand|hands|finger|fingers|thumb|palm|forearms?|arm|wrist|bottom|bum|leg|legs|back)\\b[^.?!]{0,25}\\b(?:hob|grill|oven\\s+door|oven\\s+shelf|hotplate|cooker\\s+ring|cooker)\\b")); __v.push(py_str("\\b(?:put|puts|putting|stuck|stick|sticks|sticking|thrust|shoved|shove|dipped|dip|place[ds]?|placing)\\s+(?:my|his|her|their|our|your)\\s+(?:hand|hands|finger|fingers|thumb|thumb|arm|arms|foot|feet|fist)\\b[^.?!]{0,15}\\b(?:in|into|through|over)\\s+(?:the|a|an)\\s+(?:fire|flames|flame|campfire|bonfire|barbecue|barbeque|bbq|coals|embers|burning\\s+coals)\\b")); __v.push(py_str("\\b(?:my|his|her|their|our|your)\\s+(?:hand|hands|fingers?|arm|arms|foot|feet)\\b[^.?!]{0,12}\\b(?:is|was|were|got|went|had\\s+gone)\\b[^.?!]{0,6}\\b(?:in|into|through)\\s+(?:the|a|an)\\s+(?:fire|flames|campfire|bonfire|coals|embers)\\b")); __v.push(py_str("\\b(?:brushed|brushing|brushes|graze[ds]?|grazing|scraped|rubbed|caught)\\s+(?:against|past|up\\s+against|on)?\\s*(?:(?:the|a|an|my|his|her|their|our)\\s+)?(?:hot|boiling|scalding|red\\s?hot|searing|sizzling)\\s+(?:grill|hob|ring|oven|oven\\s+door|oven\\s+shelf|iron|pan|pot|kettle|cooker|hotplate|radiator|heater|burner|stove|saucepan|element|plate|tray|shelf)\\b(?![^.!?]{0,25}\\b(?:was|were|is|are|had\\s+been|has\\s+been)\\s+(?:already\\s+|still\\s+|stone\\s+|completely\\s+)?(?:cold|off|not\\s+hot|not\\s+warm|switched\\s+off|turned\\s+off|unplugged|cooled)\\b)")); __v.push(py_str("\\b(?:hot|boiling|scalding|searing|sizzling)\\s+(?:oil|fat|water|stock|soup|gravy|syrup|sugar)\\b[^.?!]{0,10}\\b(?:on|over|onto|down|across)\\s+(?:my|his|her|their|our|your)\\s+(?:hand|hands|arm|arms|wrist|wrists|leg|legs|foot|feet|lap|chest|tummy|belly|neck|face|skin|shoulder|shoulders|back|thigh|thighs|knee|knees|fingers?|thumb|toes?|ankle|body|mouth|chin)\\b")); __v.push(op_add(&op_add(&op_add(&py_str("(?!(?:.*?)\\b"), &g_HOT_COMFORT()), &py_str(")\\b(?:touch(?:ed|ing)?|touches|grab(?:bed|bing)?|grabs|held|holding|lean(?:ed|t|ing)?\\s+on|rest(?:ed|ing)?\\s+(?:on|onto|against|over)|sat\\s+on|slipped)\\b[^.?!]{0,25}(?:(?:the|a|an|my|his|her|their|our)\\s+)?")), &g_HOT_SURFACE())); __v.push(op_add(&op_add(&op_add(&op_add(&py_str("(?!(?:.*?)\\b"), &g_HOT_COMFORT()), &py_str(")\\brest(?:ed|ing)?\\b[^.?!]{0,20}(?:(?:the|a|an|my|his|her|their|our)\\s+)?")), &g_HOT_SURFACE()), &py_str("\\b[^.?!]{0,12}\\b(?:on|onto|against|over|across)\\s+(?:my|his|her|their|our|your)\\s+(?:hand|hands|arm|arms|finger|fingers|thumb|wrist|leg|legs|knee|knees|lap|foot|feet|chest|face|skin|shoulder|shoulders|thigh|back|belly|tummy)\\b"))); __v.push(op_add(&op_add(&py_str("(?!(?:.*?)\\b"), &g_HOT_COMFORT()), &py_str(")\\b(?:put|putting|puts|placed?|placing|rest(?:ed|ing)?|touch(?:ed|ing)?|touches|grab(?:bed|bing)?|grabs|picked?\\s+up|lean(?:ed|t|ing)?\\s+on|sat\\s+on|slipped)\\s+(?:my|his|her|their)\\s+(?:hand|hands|finger|fingers|thumb|palm|forearms?|arm|wrist|bottom|bum|leg|legs|back)\\b[^.?!]{0,25}\\b(?:hot|boiling|scalding|burning)\\s+(?:iron|water|pan|kettle|saucepan|dish|tray|oil|fat|spoon|tin)\\b"))); __v.push(py_str("\\b(?:spill\\w*|spilt|pour\\w*|tip\\w*|knock\\w*|get\\w*|got|drop\\w*|dropped|splash\\w*|throw\\w*|threw|chuck\\w*)\\s+(?:boiling|hot|scalding)\\s+(?:water|oil|fat|soup|tea|coffee)\\s+(?:on|over|onto)\\s+(?:(?:my|his|her|your|their|our)\\s+(?:hand|hands|arm|arms|wrist|wrists|leg|legs|foot|feet|lap|chest|tummy|belly|neck|face|skin|shoulder|shoulders|back|thigh|thighs|knee|knees|fingers?|thumb|toes?|ankle|body|mouth|chin)|me|him|her|them|us|my\\s*self|your\\s*self|him\\s*self|her\\s*self|them\\s*selves|our\\s*selves)\\b")); __v.push(py_str("\\bmy\\s+(?:hand|finger|arm|wrist|thumb|tongue|mouth|foot|leg|skin)\\b.{0,15}\\b(?:is|got|keeps?\\s+getting)\\s+(?:burnt|burned|scalded|blistered|scorched|singed)\\b")); __v.extend(py_iter_or_raise(&g_SCORCHED_A_PERSON())); __v.push(py_str("\\b(?:put|rub|apply|use|spread)\\b[^.?!]{0,30}\\b(?:on|onto)\\s+(?:a|the|my|this|that)\\s+(?:bad\\s+|red\\s+|small\\s+|nasty\\s+|serious\\s+|little\\s+|sore\\s+|fresh\\s+)*(?:burn|scald)s?\\b")); __v.push(py_str("\\b(?:burn|burns|burning|scald|scalds|scalding)\\s+(?:my|your|his|her|their|me|myself|yourself|himself|herself|themselves)\\b(?!\\s+(?:bridges|candle\\s+at|holes?|midnight)\\b)")); __v.push(py_str("\\b(?:get|got|gets)\\s+(?:burnt|burned|scalded)\\b(?!\\s*(?:out\\b|by\\b|on\\s+(?:that|the\\s+deal|a\\s+deal)\\b))")); __v.push(py_str("\\b(?:burn|burns|burning|scald|scalds|scalding)\\b[^.?!]{0,20}\\b(?:my|your|his|her|their)\\s+(?:hand|hands|finger|fingers|thumb|arm|wrist|tongue|mouth|foot|feet|leg|skin|face|neck)\\b")); __v.push(py_str("\\b(?:first\\s+aid\\s+for|treat|treating|soothe|helps?|do\\s+for|good\\s+for|best\\s+(?:thing\\s+)?for)\\b[^.?!]{0,20}\\b(?:a|the|this)\\s+(?:bad\\s+|red\\s+|small\\s+|nasty\\s+|serious\\s+)?(?:burn|scald)\\b")); py_tuple(__v) } }
    fn __data13() -> Py { py_tuple(vec![py_str("\\b(?:broken|broke|fractured)\\s+(?:my|his|her|their|a)\\b.{0,25}\\b(?:arm|leg|wrist|ankle|hip|finger|thumb|toe|ribs?|collarbone|shoulder|foot|hand|nose|knee|elbow)\\b"), py_str("\\bmy\\s+(?:ribs?)\\s+(?:is|are|'s|'re|feels?|looks?)\\s+(?:broken|cracked|fractured|snapped)\\b"), py_str("\\b(?:cracked|crack|snapped|snap|broke|broken|fractured)\\s+(?:a|my|his|her|their|two|three)\\s+(?:ribs?|rib\\s?cage)\\b"), py_str("\\bthink\\s+(?:i'?ve\\s+)?(?:broken|fractured)\\b"), py_str("\\b(?:bone|wrist|arm|leg)\\b.{0,25}\\b(?:sticking|poking|piercing)\\s+out\\b"), py_str("\\b(?:my|his|her|their)\\s+ribs?\\b[^.?!]{0,15}\\b(?:is|are|was|were|got|might\\s+be|may\\s+be|feels?)\\b[^.?!]{0,10}\\b(?:broken|cracked|fractured)\\b"), py_str("\\b(?:cracked|crack|broke|broken|fractured)\\s+(?:a|my|his|her|their)\\s+ribs?\\b(?!\\s+(?:of\\s+beef|of\\s+pork|joint|roast|recipe|rub|rack))"), py_str("\\bthink\\s+(?:i'?ve\\s+|i\\s+)?(?:cracked|broken|fractured)\\s+(?:a\\s+|my\\s+)?ribs?\\b"), py_str("\\b(?:broken|cracked|fractured)\\s+ribs?\\b(?!\\s*(?:of\\s+(?:beef|pork|lamb)|joint|roast|recipe|rub|rack|in\\s+the))"), py_str("(?:^|[,.!?;]|\\band\\b|\\bbut\\b|\\bso\\b|\\bi\\s+think\\b|\\bi\\s+reckon\\b)\\s*(?:my|his|her|their)\\s+jaw\\b[^.?!]{0,15}\\b(?:is|are|'?s|'?re|was|were)\\s+(?:broken|fractured)\\b(?!\\s+out\\b)"), py_str("(?:^|[,.!?;]|\\band\\b|\\bbut\\b|\\bso\\b|\\bi\\s+think\\b|\\bi\\s+reckon\\b)\\s*(?:my|his|her|their)\\s+nose\\b[^.?!]{0,15}\\b(?:is|are|'?s|'?re|was|were)\\s+(?:broken|cracked|fractured|snapped)\\b(?!\\s+out\\b)"), py_str("(?:^|[,.!?;]|\\band\\b|\\bbut\\b|\\bso\\b|\\bi\\s+think\\b|\\bi\\s+reckon\\b)\\s*(?:my|his|her|their)\\s+collarbone\\b[^.?!]{0,15}\\b(?:is|are|'?s|'?re|was|were)\\s+snapped\\b"), py_str("\\b(?:broken|broke|fractured)\\s+(?:my|his|her|their)\\s+jaw\\b"), py_str("\\b(?:broken|broke|fractured|cracked|snapped)\\s+(?:my|his|her|their)\\s+nose\\b"), py_str("\\b(?:snapped)\\s+(?:my|his|her|their)\\s+collarbone\\b"), py_str("\\b(?:my|his|her|their|your)\\s+(?:fingers?|thumb|toes?)\\b[^.?!]{0,20}\\b(?:is|are|looks?|went|gone|got)\\s+(?:all\\s+)?(?:bent|bent\\s+back|twisted|out\\s+of\\s+shape|at\\s+a\\s+funny\\s+angle)\\b")]) }
    fn __data10() -> Py { py_tuple(vec![py_tuple(vec![py_str("mouth_burn"), py_tuple(vec![py_str("\\b(?:burnt|burned|scalded)\\s+(?:the\\s+roof\\s+of\\s+my|inside\\s+of\\s+my|my\\s+(?:tongue|mouth|throat))\\b")]), py_tuple(vec![py_str("Sip cold water or suck an ice cube, and skip hot, salty or citrus food for a couple of days; the roof of your mouth usually settles within a week. Get it looked at if it's a big white patch or still sore after a week."), py_str("Cold water or an ice cube is the fix, and give hot, salty and citrus food a miss for a few days. It normally heals in about a week; anything large or still painful after that should be checked.")])]), py_tuple(vec![py_str("tooth_chipped"), g_TOOTH_CHIPPED(), py_tuple(vec![py_str("A chipped tooth is a dentist's, not an ambulance: if a piece came off, keep it in milk, and ring 111 for an urgent dental appointment. Go today if it is bleeding, hurts a lot, or you can see a pink or red centre in it."), py_str("Ring 111 and ask for an urgent dentist, usually within a day or two; keep any piece that came off in milk and eat on the other side. Bleeding that will not stop, or a bad knock with it, wants to be seen today.")])]), py_tuple(vec![py_str("tooth_out"), g_TOOTH_OUT(), py_tuple(vec![py_str("Pick the tooth up by the crown and never the root, put it straight back in the socket if you can, or keep it in milk or saline: a dentist within the hour gives it the best chance, so ring 111 for the emergency dentist now. A bang on the head as well, or bleeding you cannot stop, is 999 or A&E."), py_str("Hold it by the crown, put it back in the socket if you can and bite gently on a cloth, or keep it in milk or saline, and ring 111 for an emergency dentist now - the first hour is what matters. If there was a bang on the head or the bleeding will not stop, ring 999.")])]), py_tuple(vec![py_str("should_i_call"), g_SHOULD_I_CALL(), py_tuple(vec![py_str("That is a 999 or 911 call, not a judgement call: if the question is in your head at all, ring and say what you can see - they would far rather have the call than not, and being wrong about it costs you nothing."), py_str("Ring 999 or 911 and say what you can see; the call taker decides what to send, and they would rather hear from you than not. If it turns out to be nothing much, nobody will mind.")])]), py_tuple(vec![py_str("part_off"), g_PART_OFF(), py_tuple(vec![py_str("A part that is off, or hanging on by a thread, is a hospital job now rather than a plaster: press a clean cloth on the wound and keep pressing, and if a piece has come away, wrap it in a clean cloth, seal it in a bag and keep the bag cool - never the piece straight on ice. Ring 999 or 911 if the bleeding will not slow or they go pale or dizzy; if it has slowed, go straight to A&E."), py_str("Do not wait on that one: press a clean cloth on it and hold it without lifting it to look, and if a piece has come away, wrap it in a clean cloth, seal it in a bag and keep the bag cool. Ring 999 or 911 if the bleeding will not stop or they look pale or clammy - otherwise A&E now, not tomorrow.")])]), py_tuple(vec![py_str("bleeding"), g_BLEEDING_SMALL(), py_tuple(vec![py_str("Press a clean cloth on the cut and hold it there without lifting it to look for ten minutes; most cuts stop by then. If it soaks through, the edges gape, or they go pale or dizzy, that's 999 or 911."), py_str("Press a clean cloth on it and keep pressing for ten minutes without lifting it to look; add another cloth on top if it soaks through. If it's still going after twenty minutes or they go pale or dizzy, that's 999 or 911.")])]), py_tuple(vec![py_str("cut_wrist"), g_CUT_ON_THROAT_BODY(), py_tuple(vec![py_str("Press a clean cloth on it and hold for ten minutes, and get it looked at if it gapes or keeps bleeding. If you did that to yourself, please tell someone today - Samaritans is 116 123, free and open any hour, and one cut on a wrist is not a fuss."), py_str("Hold a clean cloth on it for ten minutes; if it needs a stitch or will not stop, that is A&E. And if that was on purpose, tell someone today - 116 123 is Samaritans, free and open any hour.")])]), py_tuple(vec![py_str("pet_nail"), g_PET_NAIL(), py_tuple(vec![py_str("Press a pinch of flour, cornflour or a styptic pencil on the end of the nail and hold it there for five minutes without lifting it to look - a cut quick bleeds much more than the nick deserves, and it stops with pressure. Keep them off the pavement and the stairs for the rest of the day, and ring the vet if it is still going after ten minutes, if the nail is torn or hanging, or if a cat has caught a claw."), py_str("Flour or cornflour pressed on the tip for five minutes is the first thing, and most stop there; a torn nail or one that keeps bleeding needs the vet today, and so does a cat's caught claw, which snags again until it is trimmed back. Soft ground and no jumping for the rest of the day.")])]), py_tuple(vec![py_str("pet_wound"), g_PET_WOUND(), py_tuple(vec![py_str("Bathe it with cool water or salt water and pat it dry, then keep the animal off it - a buster collar or a sock over a paw - because licking keeps a wound wet and slow to heal. Ring the vet today if it gapes open, keeps bleeding, or is on a paw, an ear or near an eye, and put nothing from the bathroom cabinet on it."), py_str("Cool water or salt water, pat it dry, and keep the animal from licking it - a cone or a sock over a paw does that better than telling it off. Anything that gapes, keeps bleeding, or is on a paw, an ear or an eye is a vet visit today rather than a wait-and-see, and nothing from the bathroom cabinet goes on it: a pet licks a plaster off and the cream is not made for animals.")])]), py_tuple(vec![py_str("cut"), __data11(), py_tuple(vec![py_str("Press a clean cloth on it and keep pressing for ten minutes without lifting it to look; most small cuts stop by then. If the edges gape, it's deep, or it's still bleeding after twenty minutes of steady pressure, get it looked at."), py_str("Wash it under the tap, press a clean cloth down and hold for ten minutes; that's what stops most cuts. A cut that gapes open, is deep, or keeps bleeding through the cloth needs a nurse or A&E.")])]), py_tuple(vec![py_str("burn"), __data12(), py_tuple(vec![py_str("Cool it under cool running water for twenty minutes, then cover it loosely with cling film or a clean non-fluffy cloth. Don't use ice, butter or cream, and get it checked if it's bigger than your palm, on the face or hands, or blistered."), py_str("Twenty minutes under cool running water is the thing that helps, starting straight away. Cover it loosely and skip ice and butter; a burn that blisters or is on the face, hands or joints should be seen.")])]), py_tuple(vec![py_str("nosebleed"), py_tuple(vec![py_str("\\bnose\\s*bleed\\b"), py_str("\\bnose\\s+(?:is\\s+|keeps\\s+|started\\s+)?bleeding\\b"), py_str("\\bnose\\b.{0,20}\\bbleed\\w*\\b")]), py_tuple(vec![py_str("Sit up, lean forward and pinch the soft part of your nose for ten minutes without checking. Spit out any blood rather than swallowing it; if it's still going after twenty minutes, get help."), py_str("Lean forward, pinch the soft part of the nose and hold for ten minutes; leaning back just sends blood down your throat. If it keeps coming back or lasts over twenty minutes, see someone.")])]), py_tuple(vec![py_str("weight_limb_long"), g_WEIGHT_JOINT(), py_tuple(vec![py_str("Weeks or months of it is a GP job rather than A&E: book an appointment, say how long it has been going on and what sets it off, and keep off it as much as you can meanwhile. Anything sudden, or clearly worse than usual, is the exception - that is A&E the same day.")])]), py_tuple(vec![py_str("ring_stuck"), g_RING_STUCK(), py_tuple(vec![py_str("Get the ring off now, before the finger swells any further: hold your hand above your head for a few minutes, then work it round with soap and water or a little cooking oil, twisting rather than pulling. If it will not move, wind dental floss or thin thread under the band and unwrap it towards the fingertip; a jeweller can cut it off, and so can A&E. If the finger has gone numb, cold or blue, go today rather than waiting - a ring left on a swelling finger can cut the blood off within hours."), py_str("Get it off today if you can: hand up above your head for a few minutes, then soap and water or a little oil, twisting rather than pulling, and do not keep at it. Floss wound under the ring and unwrapped towards the tip will walk it off. If it will not move, or the finger is numb, cold or blue, that is A&E or 111 today; a jeweller can cut it off too.")])]), py_tuple(vec![py_str("weight_limb"), g_WEIGHT_JOINT(), py_tuple(vec![py_str("A joint that won't take weight is an X-ray today rather than a wait-and-see: go to A&E or a minor injuries unit, and keep it still on the way. Ice it and keep it raised until it has been seen."), py_str("Don't wait to see whether it eases: a joint that won't take weight wants an X-ray today, at A&E or a minor injuries unit. Keep it still and raised on the way there.")])]), py_tuple(vec![py_str("sprain"), py_tuple(vec![py_str("\\bsprain(?:ed)?\\s+(?:my|his|her|their)\\b"), py_str("\\bsprain(?:ed)?\\s+(?:a|an|the|both|your)?\\s*(?:ankle|ankles|knee|knees|wrist|wrists|thumb|elbow)s?\\b"), py_str("\\btwisted\\s+my\\s+(?:ankle|knee|wrist)\\b"), py_str("\\brolled\\s+my\\s+ankle\\b"), py_str("\\b(?:went\\s+)?over\\s+on\\s+my\\s+ankle\\b"), py_str("\\bhurt\\s+(?:my|his|her|their)\\s+(?:knee|ankle|wrist|shoulder|elbow)\\b"), py_str("\\b(?:ankle|knee|wrist|elbow|foot|finger|thumb|shoulder)\\b.{0,40}\\b(?:swollen|swelling|puffy|bruised|bruising)\\b"), py_str("\\b(?:swollen|swelling|puffy|bruised|bruising)\\b.{0,40}\\b(?:ankle|knee|wrist|elbow|foot|finger|thumb|shoulder)\\b")]), py_tuple(vec![py_str("Rest it, put an ice pack on for 15 minutes at a time, and wrap it snugly but not tight; keep it up when you sit down. If you can't put weight on it or it swells badly, get it checked."), py_str("Ice for 15 minutes, raise it, and take it easy for a day or two. If you can't bear weight or it's very swollen and bruising fast, have it looked at.")])]), py_tuple(vec![py_str("sight_long"), g_SIGHT_EVER(), g_SIGHT_LIFE_REPLIES()]), py_tuple(vec![py_str("limp_long"), g_LIMP_PERSON(), py_tuple(vec![py_str("A limp that has been going on for weeks is a GP job rather than A&E: book an appointment, say how long it has been there and what makes it worse, and keep up what you can do without pain meanwhile. Get it seen sooner if it is getting worse, if the joint gives way, or if there is swelling or heat in it.")])]), py_tuple(vec![py_str("limp"), g_LIMP_PERSON(), py_tuple(vec![py_str("Check it while they are sitting: look for swelling and bruising, run a hand down the shin and press gently round the ankle and the knee, and see which way it hurts. Ice it for 15 minutes, rest it and give children's paracetamol if the age allows; if they will not put weight on it, the joint is swollen or hot, or a small child will not walk at all, that is the same day at a minor injuries unit."), py_str("A limp that comes after a knock is rest, ice for 15 minutes and a day or two off it. Get it looked at the same day if you cannot put weight on it, if it is swollen or hot, or if a small child has stopped walking on it altogether.")])]), py_tuple(vec![py_str("sunburn"), py_tuple(vec![py_str("\\bsun\\s?burnt\\b"), py_str("\\bsun\\s?burn(?:ed)?\\b"), py_str("\\bsunburn\\b")]), py_tuple(vec![py_str("Get out of the sun, cool the skin with a shower or a damp cloth, and drink plenty of water; calamine or after-sun takes the sting out. Blisters, shivering or feeling faint need a doctor, and keep it covered for a few days."), py_str("Cool showers, plenty of water and a loose cotton shirt are what help; leave blisters alone. If someone is shivery, dizzy or badly blistered, that needs a doctor.")])]), py_tuple(vec![py_str("splinter"), py_tuple(vec![py_str("\\bsplinter\\b")]), py_tuple(vec![py_str("Soak it in warm water for ten minutes and pull it out with tweezers in the direction it went in. If it's deep, under a nail, or the area gets red and hot, get it looked at."), py_str("Warm water first, then tweezers along the line it went in. Anything deep or going red and hot should be checked.")])]), py_tuple(vec![py_str("broken"), __data13(), py_tuple(vec![py_str("Keep it still and supported, ice it for twenty minutes and take paracetamol; don't try to straighten it yourself. Go to A&E or a minor injuries unit, and ring 999 if the bone is through the skin, it looks bent, or the fingers or toes go numb or white."), py_str("Support it where it is, ice it and take painkillers; a bent limb, bone through the skin or numb fingers is 999, otherwise get it checked at A&E or a minor injuries unit today.")])])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.MINOR"); }
    s_MINOR(__data10());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LAUGH_WORD"); }
    s_LAUGH_WORD(py_tuple(vec![py_str("\\blaugh\\w*\\b"), py_str("\\bgiggl\\w*\\b"), py_str("\\bchuckl\\w*\\b"), py_str("\\blol\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LAUGH_TAKES_BREATH"); }
    s_LAUGH_TAKES_BREATH(py_tuple(vec![py_str("\\bbreath\\w*\\b[^.!?]{0,30}\\b(?:when|whenever|if|while|after)\\b[^.!?]{0,20}\\blaugh\\w*\\b"), py_str("\\b(?:when|whenever|while|if|after)\\b[^.!?]{0,15}\\blaugh\\w*\\b[^.!?]{0,40}\\bbreath\\w*\\b"), py_str("\\blaugh\\w*\\b[^.!?]{0,40}\\b(?:can'?t|cannot|couldn'?t|makes?\\s+me|lose|lost|losing)\\b[^.!?]{0,20}\\bbreath\\w*\\b"), py_str("\\blaugh\\w*\\b[^.!?]{0,40}\\b(?:out\\s+of\\s+(?:breath|puff)|breathless|puffed|winded)\\b"), py_str("\\b(?:lose|lost|losing)\\s+(?:my|his|her|their|our)\\s+breath\\b[^.!?]{0,25}\\blaugh\\w*\\b"), py_str("\\bbreath\\b[^.!?]{0,25}\\blaugh\\w*\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LAUGH_BREATH_RED"); }
    s_LAUGH_BREATH_RED(py_tuple(vec![py_str("\\b(?:stopped\\s+breathing|isn'?t\\s+breathing|not\\s+breathing|gasping|turned\\s+blue|went\\s+blue|blue\\s+lips|pass(?:ed)?\\s+out|blacked\\s+out|fainted|faint|collapsed|hurts?|wheez\\w*|at\\s+night|in\\s+the\\s+(?:night|evening)|during\\s+the\\s+night|night[\\s-]?time|wok(?:e|en)(?:\\s+up)?|wakes?\\s+(?:me|up)|waking\\s+(?:up|me)|coughing\\s+up|blood|dizz\\w*|light[\\s-]?headed|see\\s+stars|blurr\\w*|\\bnow\\b|\\bat\\s+all\\b|\\bstill\\b|\\bnot\\s+(?:coming|getting|back|stopping)\\b|\\b(?:hasn'?t|havent|haven'?t|didnt|didn'?t|won'?t|wont)\\s+(?:come|got|get|stop)\\b|\\b(?:been\\s+going|going|gone)\\s+on\\s+for\\b|\\bfor\\s+\\d+\\s+(?:minutes?|hours?)\\s+now\\b|(?:\\bfor|\\btakes?)\\s+(?:ages|a\\s+(?:long\\s+)?(?:while|time)|minutes)\\b)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.LAUGH_OTHER_PERSON"); }
    s_LAUGH_OTHER_PERSON(py_tuple(vec![py_str("\\b(?:he|she|they)\\b[^.?!]{0,25}\\b(?:couldn'?t|couldnt|stopped|wasnt|was\\s+not|isnt|is\\s+not)\\b[^.?!]{0,15}\\bbreath\\w*"), py_str("\\b(?:my|our|her|his|their)\\s+(?:son|daughter|child|kid|baby|boy|girl|husband|wife|partner|mum|mom|dad|father|mother|brother|sister|nan|grandma|grandad|grandmother|grandfather|friend|mate|neighbour|neighbor)\\b[^.?!]{0,25}\\b(?:was|were)\\s+laugh\\w*"), py_str("\\b(?:someone|somebody)\\b[^.?!]{0,20}\\b(?:couldn'?t|couldnt|stopped)\\b[^.?!]{0,12}\\bbreath\\w*")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_BREATH"); }
    s_PET_BREATH(py_tuple(vec![py_str("\\b(?:dog|dogs|pupp\\w*|cat|cats|kitten|kittens|pet|parrot|budgie|hamster|rabbit|bunny|horse|pony|guinea\\s+pig|tortoise|ferret|gerbil|rat|lizard|snake|animal)\\b\\s*(?:,\\s*(?:he|she|it)\\s+)?(?:is|are|was|were|gets?|got|keeps?|seems?|looks?|sounds?|'s|has\\s+been|have\\s+been)\\s+(?:\\w+\\s+){0,2}(?:breathless|out\\s+of\\s+(?:puff|breath)|puffed|winded)\\b"), py_str("\\b(?:dog|dogs|pupp\\w*|cat|cats|kitten|kittens|pet|parrot|budgie|hamster|rabbit|bunny|horse|pony|guinea\\s+pig|tortoise|ferret|gerbil|rat|lizard|snake|animal)\\b\\s*(?:,\\s*(?:he|she|it)\\s+)?(?:(?:is|was|are|were|keeps?|'s)\\s+)?(?:can'?t|cannot|couldn'?t|could\\s+not|is\\s+not|isn'?t|struggl\\w*)\\s+(?:to\\s+)?(?:catch|get|find)\\s+(?:his|her|its|their)\\s+breath\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_BREATH_NOW"); }
    s_PET_BREATH_NOW(py_tuple(vec![py_str("\\b(?:can'?t|cannot|couldn'?t|could\\s+not|struggl\\w*|struggled)\\s+(?:to\\s+)?(?:catch|get|find)\\s+(?:his|her|its|their)\\s+breath\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_EFFORT"); }
    s_PET_EFFORT(py_tuple(vec![py_str("\\b(?:play\\w*|running|ran(?:\\s+about)?|runs?\\b|fetch\\w*|walk\\w*|park|chase\\w*|ball|exercise\\w*|training|agility|swim\\w*|trot\\w*|canter\\w*|gallop\\w*|ride|rides|riding|ridden|lunge\\w*|hack\\w*|jump\\w*|lesson|schooling)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards._PET_APPOSITIVE"); }
    s__PET_APPOSITIVE(py_str("\\b(?:dog|dogs|pupp\\w*|cat|cats|kitten|kittens|pet|parrot|budgie|hamster|rabbit|bunny|horse|pony|guinea\\s+pig|tortoise|ferret|gerbil|rat|lizard|snake|animal)\\b\\s*,\\s*(?:he|she|it)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_APPOSITIVE"); }
    s_PET_APPOSITIVE(py_tuple(vec![g__PET_APPOSITIVE()]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_APPOSITIVE_SUB"); }
    s_PET_APPOSITIVE_SUB(stdlib_call("re", "compile", &[g__PET_APPOSITIVE(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_CANT_SETTLE"); }
    s_PET_CANT_SETTLE(py_tuple(vec![py_str("\\b(?:can'?t|cannot|won'?t|wont|not)\\s+(?:get\\s+)?(?:comfortable|comfy|settle[ds]?)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HEAT_CAUSE"); }
    s_HEAT_CAUSE(py_tuple(vec![py_str("\\b(?:this|the|all\\s+this)\\s+heat\\b|\\bheat\\s?wave\\b|\\bhumidi(?:d|ty)\\b|\\bhumidity\\b|\\b(?:hot|boiling|muggy|stifling|sweltering)\\s+(?:weather|day|room|office|night|bus|tube)\\b|\\b(?:so|too|really|very)\\s+(?:hot|humid|muggy|stuffy)\\b|\\bheat\\s+of\\s+the\\s+day\\b|\\b\\d{2,3}\\s*degrees\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HEAT_NOT_WEATHER"); }
    s_HEAT_NOT_WEATHER(py_tuple(vec![py_str("\\b(?:chilli?|chili|curry|pepper\\w*|spic(?:y|e|ier|iest)|wasabi|horseradish|mustard|salsa|sauce|wings|broth|soup|kebab|taco\\w*|jalape\\w*|paprika|ginger|oven|grill|barbecue|bbq|bonfire|sauna|steam(?:er|y)?|toaster?)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.BREATH_BARE"); }
    s_BREATH_BARE(py_tuple(vec![py_str("\\b(?:can'?t|cannot|couldn'?t|hard\\s+to|difficult\\s+to)\\s*(?:breathe|breath\\b|get\\s+my\\s+breath|catch\\s+my\\s+breath)\\b|\\bstruggl\\w*\\s+(?:to\\s+)?breathe\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.HEAT_BREATH_RED"); }
    s_HEAT_BREATH_RED(py_tuple(vec![py_str("\\b(?:faint\\w*|dizz\\w*|light[\\s-]?headed|confus\\w*|sick|vomit\\w*|one\\s+side|slurr\\w*|for\\s+hours?|all\\s+day|since\\s+(?:yesterday|this\\s+morning)|asthma|inhaler|COPD\\b|not\\s+settl\\w*|wont\\s+settle|won'?t\\s+settle|no\\s+better|getting\\s+worse)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.ALLERGY_FOOD"); }
    s_ALLERGY_FOOD(py_tuple(vec![py_str("\\b(?:swelling|swollen|swelled|puff\\w*\\s+up|puffed\\s+up|puffing\\s+up|puffy|hives?|urticaria|welts?|blotch\\w*|rash)\\b[^.!?]{0,45}\\b(?:after|from|since|because\\s+of)\\b[^.!?]{0,25}\\b(?:eat\\w*|ate|eaten|having|had|drank|drink\\w*|tried|tast\\w*|bit\\s+into|food|meal|breakfast|lunch|dinner|snack|formula|milk|sting\\w*|stung|bee|wasp|antibiotic\\w*|medicine|tablet\\w*)\\b"), py_str("\\b(?:after|from)\\b[^.!?]{0,15}\\b(?:eating|having|drinking|trying|tasting|taking)\\b[^.!?]{0,30}\\b(?:swelling|swollen|swelled|puff\\w*|puffy|hives?|urticaria|welts?|rash|itch\\w*|tingl\\w*|blotch\\w*)\\b"), py_str("\\b(?:stung|stings?)\\b[^.!?]{0,30}\\b(?:and|,)\\b[^.!?]{0,20}\\b(?:now\\s+)?\\b(?:her|his|their|my|our|the)\\b[^.!?]{0,18}\\b(?:hand|arm|foot|leg|face|lips?|tongue|throat|finger|ankle|wrist|cheeks?)\\b[^.!?]{0,20}\\b(?:swell\\w*|swollen|puff\\w*|itch\\w*|hives?|rash)\\b"), py_str("\\b(?:ate|eaten|eating|had|tried|drank|drinking)\\b[^.!?]{0,30}\\b(?:and|,)\\b[^.!?]{0,20}\\b(?:her|his|their|my|our|the)\\b[^.!?]{0,18}\\b(?:lips?|tongue|throat|face|mouth|cheeks?)\\b[^.!?]{0,20}\\b(?:puff\\w*|puffy|swell\\w*|swollen|itch\\w*|tingl\\w*|hives?|rash)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.ALLERGY_AIRWAY"); }
    s_ALLERGY_AIRWAY(py_tuple(vec![py_str("\\b(?:tongue|throat|voice|swallow\\w*|breath\\w*|can'?t\\s+breathe|wheez\\w*|faint\\w*|dizz\\w*|limp|floppy|collapse\\w*|unconscious|shock)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.ALLERGY_OFF"); }
    s_ALLERGY_OFF(py_tuple(vec![py_str("\\b(?:chapped|cold\\s+sore|piercing|teething|dummy|chewed|bit\\s+my\\s+lip|bitten\\s+lip|sunburn\\w*|in\\s+the\\s+cold|from\\s+the\\s+cold|dehydrat\\w*|mouth\\s+ulcer|bump\\w*\\s+it)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.ALLERGY_FACE"); }
    s_ALLERGY_FACE(py_tuple(vec![py_str("\\b(?:lips?|tongue|throat|face|mouth|cheeks?)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.STING_WORDS"); }
    s_STING_WORDS(py_tuple(vec![py_str("\\b(?:bee|bees|wasp|wasps|hornet\\w*|stung|stings?|bumble\\s?bee\\w*)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.FOOD_WORDS"); }
    s_FOOD_WORDS(py_tuple(vec![py_str("\\b(?:eat\\w*|ate|eaten|food|meal|breakfast|lunch|dinner|snack|formula|milk|drank|drink\\w*)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CRUSH_ON_A_PERSON"); }
    s_CRUSH_ON_A_PERSON(py_tuple(vec![py_str("\\b(?:someone|somebody|a\\s+(?:man|woman|child|kid|bloke|lady|person)|him|her|them|people|anyone|everyone)\\b"), py_str("\\b(?:my|his|her|their|our|your)\\s+(?:husband|wife|partner|fianc|boyfriend|girlfriend|son|daughter|child|kid|kids|baby|boy|girl|mum|mom|mother|dad|father|brother|sister|friend|mate|neighbour|neighbor|gran|granny|grandma|grandad|grandpa|uncle|aunt|cousin|nan|nana|colleague|boss|carer|lodger|housemate|flatmate|dog|cat|puppy|kitten|pet|hands?|fingers?|thumbs?|arms?|wrists?|elbows?|legs?|feet|foot|toes?|knees?|ankles?|shins?|chest|back|head|neck|face|shoulders?|hips?|ribs|pelvis|thighs?|calf|calves|torso|spine|belly|stomach|lap)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CRUSH_HEAVY_ON_A_PRONOUN"); }
    s_CRUSH_HEAVY_ON_A_PRONOUN(py_str("\\b(?:roof|roofs|roofing|chimney|ceiling|ceilings|wall|walls|beam|beams|joist|joists|scaffold\\w*|slab|slabs|brick|bricks|masonry|rubble|debris|wardrobe|cupboard|shelves|shelf|bookcase|cabinet|fridge|freezer|door|ladder|television|tv|mirror|glass|window|tiles?|slates?|gutter\\w*|fence|fencing|panel|panels|gate|garage\\s+door|car|van|lorry|truck|container|pallet|machinery|machine|tree|trees|branch|branches|trunk|limb)\\b[^.!?]{0,30}\\b(?:fallen|fell|come\\s+down|came\\s+down|collapsed|toppled|tumbled|gave\\s+way)\\b[^.!?]{0,20}\\b(?:on|onto)\\b[^.!?]{0,15}\\b(?:me|us|myself|yourself|himself|herself|themselves|everybody|everyone|someone|somebody|anyone|anybody)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CRUSH"); }
    s_CRUSH(py_tuple(vec![g_CRUSH_HEAVY_ON_A_PRONOUN(), op_add(&op_add(&py_str("\\b(?:collapsed|fallen|fell|come\\s+down|came\\s+down|tumbled|toppled|gave\\s+way|give\\s+way)\\b[^.!?]{0,30}\\b(?:and|,|with)\\b[^.!?]{0,25}(?:"), &mcall(&py_str("|"), "join", &[g_CRUSH_ON_A_PERSON()])), &py_str(")[^.!?]{0,25}\\b(?:under|underneath|trapped|stuck|pinned|buried)\\b")), op_add(&op_add(&py_str("\\b(?:fallen|fell|came\\s+down|come\\s+down|collapsed|toppled|toppled\\s+over)\\b[^.!?]{0,20}\\b(?:on|onto)\\b[^.!?]{0,15}(?:"), &mcall(&py_str("|"), "join", &[g_CRUSH_ON_A_PERSON()])), &py_str(")")), py_str("\\b(?:someone|somebody|a\\s+(?:man|woman|child|kid|person)|my\\s+\\w+|he|she|they|i)\\b[^.!?]{0,25}\\b(?:crushed|pinned|buried|trapped)\\b[^.!?]{0,15}\\b(?:by|under|underneath)\\b"), py_str("\\b(?:crushed|pinned)\\s+(?:by|under)\\b[^.!?]{0,25}\\b(?:machine|machinery|truck|van|car|lorry|wall|ceiling|roof|beam|scaffold\\w*|slab|container|pallet|forklift|tractor|rubble|debris)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CRUSH_OFF"); }
    s_CRUSH_OFF(py_tuple(vec![py_str("\\b(?:duvet|blanket|quilt|covers|pillow|cushions?|bed|toddler|cat|dog|puppy|kitten)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.CRUSH_HARD"); }
    s_CRUSH_HARD(py_tuple(vec![py_str("\\b(?:machine\\w*|truck|van|car|lorry|wall|walls|ceiling|roof|beam|scaffold\\w*|slab|container|pallet|forklift|tractor|rubble|debris|tree|branch|gate|lift|door)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.MY_OWN_PART"); }
    s_MY_OWN_PART(py_tuple(vec![py_str("\\bmy\\s+(?:arm|arms|leg|legs|hand|hands|face|side|foot|feet|mouth|eye|eyes|lip|lips|cheek|shoulder|hip|knee|grip|voice|speech|vision|tongue|fingers|toes)\\b"), py_str("\\b(?:i|im|i'm)\\b[^.?!]{0,14}\\b(?:can'?t|cannot|couldn'?t|struggling\\s+to|unable\\s+to)\\s+(?:see|walk|feel|talk|speak|hold|move|smile|swallow)\\b"), py_str("\\b(?:i|im|i'm)\\b[^.?!]{0,8}\\b(?:have|ve|'ve)\\s+(?:gone|become)\\s+(?:weak|numb|blind)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_cards.PET_WEAK"); }
    s_PET_WEAK(py_tuple(vec![py_str("\\b(?:on\\s+)?(?:one|his|her|its|their|the|left|right)\\s+side\\b"), py_str("\\b(?:gone|gone\\s+all|has\\s+gone|have\\s+gone)\\s+weak\\b"), py_str("\\b(?:is|are|seems|looks|been)\\s+weak\\s+(?:on|in|down)\\b"), py_str("\\b(?:half|one\\s+half)\\s+of\\s+(?:his|her|its|their)\\s+(?:body|face)\\b"), py_str("\\bdroop\\w*|\\bdropping\\b"), py_str("\\bdragging\\b|\\bdragged\\b"), py_str("\\btilt(?:ed|ing)\\b"), py_str("\\b(?:back|front|hind)\\s+legs?\\b")]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.safety_cards".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALLERGY_AIRWAY" => Some(g_ALLERGY_AIRWAY()),
            "ALLERGY_FACE" => Some(g_ALLERGY_FACE()),
            "ALLERGY_FOOD" => Some(g_ALLERGY_FOOD()),
            "ALLERGY_NOW" => Some(g_ALLERGY_NOW()),
            "ALLERGY_OFF" => Some(g_ALLERGY_OFF()),
            "ALLERGY_SIGN_NAMED" => Some(g_ALLERGY_SIGN_NAMED()),
            "A_TOOTH_DAMAGED" => Some(g_A_TOOTH_DAMAGED()),
            "A_TOOTH_OWNER" => Some(g_A_TOOTH_OWNER()),
            "BAT_BITE" => Some(g_BAT_BITE()),
            "BITE_DONE" => Some(g_BITE_DONE()),
            "BLEEDING_SMALL" => Some(g_BLEEDING_SMALL()),
            "BODY" => Some(g_BODY()),
            "BREATH_BARE" => Some(g_BREATH_BARE()),
            "BURNT_BODY_VERB" => Some(g_BURNT_BODY_VERB()),
            "BURN_ON_BODY" => Some(g_BURN_ON_BODY()),
            "BURN_VERB" => Some(g_BURN_VERB()),
            "COME_OFF_IT" => Some(g_COME_OFF_IT()),
            "COUNTERFACTUAL_IDIOM" => Some(g_COUNTERFACTUAL_IDIOM()),
            "COUNTERFACTUAL_WISH" => Some(g_COUNTERFACTUAL_WISH()),
            "CRUSH" => Some(g_CRUSH()),
            "CRUSH_HARD" => Some(g_CRUSH_HARD()),
            "CRUSH_HEAVY_ON_A_PRONOUN" => Some(g_CRUSH_HEAVY_ON_A_PRONOUN()),
            "CRUSH_OFF" => Some(g_CRUSH_OFF()),
            "CRUSH_ON_A_PERSON" => Some(g_CRUSH_ON_A_PERSON()),
            "CUT_BODY_PART" => Some(g_CUT_BODY_PART()),
            "CUT_ON_BODY" => Some(g_CUT_ON_BODY()),
            "CUT_ON_THROAT_BODY" => Some(g_CUT_ON_THROAT_BODY()),
            "DOG_ATTACK" => Some(g_DOG_ATTACK()),
            "EFFORT_BEFORE" => Some(g_EFFORT_BEFORE()),
            "EFFORT_DIED" => Some(g_EFFORT_DIED()),
            "EFFORT_DOING" => Some(g_EFFORT_DOING()),
            "EFFORT_NOT_AN_ACCIDENT" => Some(g_EFFORT_NOT_AN_ACCIDENT()),
            "FOLLOW_UP" => Some(g_FOLLOW_UP()),
            "FOOD_WORDS" => Some(g_FOOD_WORDS()),
            "GRATER_CUT" => Some(g_GRATER_CUT()),
            "HEAD_OUT_COLD" => Some(g_HEAD_OUT_COLD()),
            "HEAD_REPLIES" => Some(g_HEAD_REPLIES()),
            "HEAL_TIME" => Some(g_HEAL_TIME()),
            "HEAT_BREATH_RED" => Some(g_HEAT_BREATH_RED()),
            "HEAT_CAUSE" => Some(g_HEAT_CAUSE()),
            "HEAT_NOT_WEATHER" => Some(g_HEAT_NOT_WEATHER()),
            "HIT_IN_FUN_HARD" => Some(g_HIT_IN_FUN_HARD()),
            "HOT_COMFORT" => Some(g_HOT_COMFORT()),
            "HOT_SURFACE" => Some(g_HOT_SURFACE()),
            "HOT_WAX_ON_BODY" => Some(g_HOT_WAX_ON_BODY()),
            "HUMAN_BITTEN" => Some(g_HUMAN_BITTEN()),
            "HUMAN_OWN_INJURY" => Some(g_HUMAN_OWN_INJURY()),
            "HYPERBOLE_WISH" => Some(g_HYPERBOLE_WISH()),
            "HYPO_SIGN" => Some(g_HYPO_SIGN()),
            "INJURY_HOW" => Some(g_INJURY_HOW()),
            "INSECT_BITE" => Some(g_INSECT_BITE()),
            "INSECT_BITE_BODY" => Some(g_INSECT_BITE_BODY()),
            "INSECT_BITE_NEEDS_A_LOOK" => Some(g_INSECT_BITE_NEEDS_A_LOOK()),
            "LAUGH_BREATH_RED" => Some(g_LAUGH_BREATH_RED()),
            "LAUGH_OTHER_PERSON" => Some(g_LAUGH_OTHER_PERSON()),
            "LAUGH_TAKES_BREATH" => Some(g_LAUGH_TAKES_BREATH()),
            "LAUGH_WORD" => Some(g_LAUGH_WORD()),
            "LIMB_IS_A_THING" => Some(g_LIMB_IS_A_THING()),
            "LIMP_ONGOING" => Some(g_LIMP_ONGOING()),
            "LIMP_PERSON" => Some(g_LIMP_PERSON()),
            "LIPS_SWELL_BARE" => Some(g_LIPS_SWELL_BARE()),
            "MILD" => Some(g_MILD()),
            "MINOR" => Some(g_MINOR()),
            "MY_OWN_PART" => Some(g_MY_OWN_PART()),
            "NEAR_HIT_ON_A_ROAD" => Some(g_NEAR_HIT_ON_A_ROAD()),
            "NEAR_HIT_ON_A_ROAD_PLACE" => Some(g_NEAR_HIT_ON_A_ROAD_PLACE()),
            "NOT_A_PET_LINE" => Some(g_NOT_A_PET_LINE()),
            "NOT_BURN_ON_BODY" => Some(g_NOT_BURN_ON_BODY()),
            "NOT_JUST_A_NEAR_THING" => Some(g_NOT_JUST_A_NEAR_THING()),
            "PART_OFF" => Some(g_PART_OFF()),
            "PART_OFF_NOT" => Some(g_PART_OFF_NOT()),
            "PART_OFF_PART" => Some(g_PART_OFF_PART()),
            "PERSON_HANGING" => Some(g_PERSON_HANGING()),
            "PET_ANIMAL_HURT" => Some(g_PET_ANIMAL_HURT()),
            "PET_APPOSITIVE" => Some(g_PET_APPOSITIVE()),
            "PET_APPOSITIVE_SUB" => Some(g_PET_APPOSITIVE_SUB()),
            "PET_BREATH" => Some(g_PET_BREATH()),
            "PET_BREATH_NOW" => Some(g_PET_BREATH_NOW()),
            "PET_CANT_SETTLE" => Some(g_PET_CANT_SETTLE()),
            "PET_EFFORT" => Some(g_PET_EFFORT()),
            "PET_NAIL" => Some(g_PET_NAIL()),
            "PET_OWNS_A_THING" => Some(g_PET_OWNS_A_THING()),
            "PET_OWNS_THE_WOUND" => Some(g_PET_OWNS_THE_WOUND()),
            "PET_PASSES_BLOOD" => Some(g_PET_PASSES_BLOOD()),
            "PET_WEAK" => Some(g_PET_WEAK()),
            "PET_WOUND" => Some(g_PET_WOUND()),
            "PURSUIT_FINE" => Some(g_PURSUIT_FINE()),
            "PURSUIT_PLACE_HOME" => Some(g_PURSUIT_PLACE_HOME()),
            "RING_STUCK" => Some(g_RING_STUCK()),
            "SCALD" => Some(g_SCALD()),
            "SCORCHED_A_PERSON" => Some(g_SCORCHED_A_PERSON()),
            "SELF_HARM_HABIT" => Some(g_SELF_HARM_HABIT()),
            "SELF_HARM_REPEAT_CAUSE" => Some(g_SELF_HARM_REPEAT_CAUSE()),
            "SHOULD_I_CALL" => Some(g_SHOULD_I_CALL()),
            "SIGHT_EVER" => Some(g_SIGHT_EVER()),
            "SIGHT_LIFE_REPLIES" => Some(g_SIGHT_LIFE_REPLIES()),
            "STING_WORDS" => Some(g_STING_WORDS()),
            "STRUCK_BY_A_VEHICLE" => Some(g_STRUCK_BY_A_VEHICLE()),
            "STRUCK_BY_A_VEHICLE_NOT" => Some(g_STRUCK_BY_A_VEHICLE_NOT()),
            "STRUCK_MY_BIKE" => Some(g_STRUCK_MY_BIKE()),
            "STRUCK_NOT_NOW" => Some(g_STRUCK_NOT_NOW()),
            "SUGAR_READING_LOW" => Some(g_SUGAR_READING_LOW()),
            "TICK_ALREADY" => Some(g_TICK_ALREADY()),
            "TICK_BITE" => Some(g_TICK_BITE()),
            "TOOTH_CHIPPED" => Some(g_TOOTH_CHIPPED()),
            "TOOTH_OUT" => Some(g_TOOTH_OUT()),
            "WATER_DANGER" => Some(g_WATER_DANGER()),
            "WAX_BODY" => Some(g_WAX_BODY()),
            "WEIGHT_JOINT" => Some(g_WEIGHT_JOINT()),
            "WILD_BITE" => Some(g_WILD_BITE()),
            "_A_LOW_FIGURE" => Some(g__A_LOW_FIGURE()),
            "_A_READING_NOUN" => Some(g__A_READING_NOUN()),
            "_PET_APPOSITIVE" => Some(g__PET_APPOSITIVE()),
            "_PET_DAMAGE_THING" => Some(g__PET_DAMAGE_THING()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALLERGY_AIRWAY" => s_ALLERGY_AIRWAY(v),
            "ALLERGY_FACE" => s_ALLERGY_FACE(v),
            "ALLERGY_FOOD" => s_ALLERGY_FOOD(v),
            "ALLERGY_NOW" => s_ALLERGY_NOW(v),
            "ALLERGY_OFF" => s_ALLERGY_OFF(v),
            "ALLERGY_SIGN_NAMED" => s_ALLERGY_SIGN_NAMED(v),
            "A_TOOTH_DAMAGED" => s_A_TOOTH_DAMAGED(v),
            "A_TOOTH_OWNER" => s_A_TOOTH_OWNER(v),
            "BAT_BITE" => s_BAT_BITE(v),
            "BITE_DONE" => s_BITE_DONE(v),
            "BLEEDING_SMALL" => s_BLEEDING_SMALL(v),
            "BODY" => s_BODY(v),
            "BREATH_BARE" => s_BREATH_BARE(v),
            "BURNT_BODY_VERB" => s_BURNT_BODY_VERB(v),
            "BURN_ON_BODY" => s_BURN_ON_BODY(v),
            "BURN_VERB" => s_BURN_VERB(v),
            "COME_OFF_IT" => s_COME_OFF_IT(v),
            "COUNTERFACTUAL_IDIOM" => s_COUNTERFACTUAL_IDIOM(v),
            "COUNTERFACTUAL_WISH" => s_COUNTERFACTUAL_WISH(v),
            "CRUSH" => s_CRUSH(v),
            "CRUSH_HARD" => s_CRUSH_HARD(v),
            "CRUSH_HEAVY_ON_A_PRONOUN" => s_CRUSH_HEAVY_ON_A_PRONOUN(v),
            "CRUSH_OFF" => s_CRUSH_OFF(v),
            "CRUSH_ON_A_PERSON" => s_CRUSH_ON_A_PERSON(v),
            "CUT_BODY_PART" => s_CUT_BODY_PART(v),
            "CUT_ON_BODY" => s_CUT_ON_BODY(v),
            "CUT_ON_THROAT_BODY" => s_CUT_ON_THROAT_BODY(v),
            "DOG_ATTACK" => s_DOG_ATTACK(v),
            "EFFORT_BEFORE" => s_EFFORT_BEFORE(v),
            "EFFORT_DIED" => s_EFFORT_DIED(v),
            "EFFORT_DOING" => s_EFFORT_DOING(v),
            "EFFORT_NOT_AN_ACCIDENT" => s_EFFORT_NOT_AN_ACCIDENT(v),
            "FOLLOW_UP" => s_FOLLOW_UP(v),
            "FOOD_WORDS" => s_FOOD_WORDS(v),
            "GRATER_CUT" => s_GRATER_CUT(v),
            "HEAD_OUT_COLD" => s_HEAD_OUT_COLD(v),
            "HEAD_REPLIES" => s_HEAD_REPLIES(v),
            "HEAL_TIME" => s_HEAL_TIME(v),
            "HEAT_BREATH_RED" => s_HEAT_BREATH_RED(v),
            "HEAT_CAUSE" => s_HEAT_CAUSE(v),
            "HEAT_NOT_WEATHER" => s_HEAT_NOT_WEATHER(v),
            "HIT_IN_FUN_HARD" => s_HIT_IN_FUN_HARD(v),
            "HOT_COMFORT" => s_HOT_COMFORT(v),
            "HOT_SURFACE" => s_HOT_SURFACE(v),
            "HOT_WAX_ON_BODY" => s_HOT_WAX_ON_BODY(v),
            "HUMAN_BITTEN" => s_HUMAN_BITTEN(v),
            "HUMAN_OWN_INJURY" => s_HUMAN_OWN_INJURY(v),
            "HYPERBOLE_WISH" => s_HYPERBOLE_WISH(v),
            "HYPO_SIGN" => s_HYPO_SIGN(v),
            "INJURY_HOW" => s_INJURY_HOW(v),
            "INSECT_BITE" => s_INSECT_BITE(v),
            "INSECT_BITE_BODY" => s_INSECT_BITE_BODY(v),
            "INSECT_BITE_NEEDS_A_LOOK" => s_INSECT_BITE_NEEDS_A_LOOK(v),
            "LAUGH_BREATH_RED" => s_LAUGH_BREATH_RED(v),
            "LAUGH_OTHER_PERSON" => s_LAUGH_OTHER_PERSON(v),
            "LAUGH_TAKES_BREATH" => s_LAUGH_TAKES_BREATH(v),
            "LAUGH_WORD" => s_LAUGH_WORD(v),
            "LIMB_IS_A_THING" => s_LIMB_IS_A_THING(v),
            "LIMP_ONGOING" => s_LIMP_ONGOING(v),
            "LIMP_PERSON" => s_LIMP_PERSON(v),
            "LIPS_SWELL_BARE" => s_LIPS_SWELL_BARE(v),
            "MILD" => s_MILD(v),
            "MINOR" => s_MINOR(v),
            "MY_OWN_PART" => s_MY_OWN_PART(v),
            "NEAR_HIT_ON_A_ROAD" => s_NEAR_HIT_ON_A_ROAD(v),
            "NEAR_HIT_ON_A_ROAD_PLACE" => s_NEAR_HIT_ON_A_ROAD_PLACE(v),
            "NOT_A_PET_LINE" => s_NOT_A_PET_LINE(v),
            "NOT_BURN_ON_BODY" => s_NOT_BURN_ON_BODY(v),
            "NOT_JUST_A_NEAR_THING" => s_NOT_JUST_A_NEAR_THING(v),
            "PART_OFF" => s_PART_OFF(v),
            "PART_OFF_NOT" => s_PART_OFF_NOT(v),
            "PART_OFF_PART" => s_PART_OFF_PART(v),
            "PERSON_HANGING" => s_PERSON_HANGING(v),
            "PET_ANIMAL_HURT" => s_PET_ANIMAL_HURT(v),
            "PET_APPOSITIVE" => s_PET_APPOSITIVE(v),
            "PET_APPOSITIVE_SUB" => s_PET_APPOSITIVE_SUB(v),
            "PET_BREATH" => s_PET_BREATH(v),
            "PET_BREATH_NOW" => s_PET_BREATH_NOW(v),
            "PET_CANT_SETTLE" => s_PET_CANT_SETTLE(v),
            "PET_EFFORT" => s_PET_EFFORT(v),
            "PET_NAIL" => s_PET_NAIL(v),
            "PET_OWNS_A_THING" => s_PET_OWNS_A_THING(v),
            "PET_OWNS_THE_WOUND" => s_PET_OWNS_THE_WOUND(v),
            "PET_PASSES_BLOOD" => s_PET_PASSES_BLOOD(v),
            "PET_WEAK" => s_PET_WEAK(v),
            "PET_WOUND" => s_PET_WOUND(v),
            "PURSUIT_FINE" => s_PURSUIT_FINE(v),
            "PURSUIT_PLACE_HOME" => s_PURSUIT_PLACE_HOME(v),
            "RING_STUCK" => s_RING_STUCK(v),
            "SCALD" => s_SCALD(v),
            "SCORCHED_A_PERSON" => s_SCORCHED_A_PERSON(v),
            "SELF_HARM_HABIT" => s_SELF_HARM_HABIT(v),
            "SELF_HARM_REPEAT_CAUSE" => s_SELF_HARM_REPEAT_CAUSE(v),
            "SHOULD_I_CALL" => s_SHOULD_I_CALL(v),
            "SIGHT_EVER" => s_SIGHT_EVER(v),
            "SIGHT_LIFE_REPLIES" => s_SIGHT_LIFE_REPLIES(v),
            "STING_WORDS" => s_STING_WORDS(v),
            "STRUCK_BY_A_VEHICLE" => s_STRUCK_BY_A_VEHICLE(v),
            "STRUCK_BY_A_VEHICLE_NOT" => s_STRUCK_BY_A_VEHICLE_NOT(v),
            "STRUCK_MY_BIKE" => s_STRUCK_MY_BIKE(v),
            "STRUCK_NOT_NOW" => s_STRUCK_NOT_NOW(v),
            "SUGAR_READING_LOW" => s_SUGAR_READING_LOW(v),
            "TICK_ALREADY" => s_TICK_ALREADY(v),
            "TICK_BITE" => s_TICK_BITE(v),
            "TOOTH_CHIPPED" => s_TOOTH_CHIPPED(v),
            "TOOTH_OUT" => s_TOOTH_OUT(v),
            "WATER_DANGER" => s_WATER_DANGER(v),
            "WAX_BODY" => s_WAX_BODY(v),
            "WEIGHT_JOINT" => s_WEIGHT_JOINT(v),
            "WILD_BITE" => s_WILD_BITE(v),
            "_A_LOW_FIGURE" => s__A_LOW_FIGURE(v),
            "_A_READING_NOUN" => s__A_READING_NOUN(v),
            "_PET_APPOSITIVE" => s__PET_APPOSITIVE(v),
            "_PET_DAMAGE_THING" => s__PET_DAMAGE_THING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.safety_cards", module_obj);
    pyrt::xrefs::reg_access("modules.safety_cards", "ALLERGY_AIRWAY", g_ALLERGY_AIRWAY, s_ALLERGY_AIRWAY);
    pyrt::xrefs::reg_access("modules.safety_cards", "ALLERGY_FACE", g_ALLERGY_FACE, s_ALLERGY_FACE);
    pyrt::xrefs::reg_access("modules.safety_cards", "ALLERGY_FOOD", g_ALLERGY_FOOD, s_ALLERGY_FOOD);
    pyrt::xrefs::reg_access("modules.safety_cards", "ALLERGY_NOW", g_ALLERGY_NOW, s_ALLERGY_NOW);
    pyrt::xrefs::reg_access("modules.safety_cards", "ALLERGY_OFF", g_ALLERGY_OFF, s_ALLERGY_OFF);
    pyrt::xrefs::reg_access("modules.safety_cards", "ALLERGY_SIGN_NAMED", g_ALLERGY_SIGN_NAMED, s_ALLERGY_SIGN_NAMED);
    pyrt::xrefs::reg_access("modules.safety_cards", "A_TOOTH_DAMAGED", g_A_TOOTH_DAMAGED, s_A_TOOTH_DAMAGED);
    pyrt::xrefs::reg_access("modules.safety_cards", "A_TOOTH_OWNER", g_A_TOOTH_OWNER, s_A_TOOTH_OWNER);
    pyrt::xrefs::reg_access("modules.safety_cards", "BAT_BITE", g_BAT_BITE, s_BAT_BITE);
    pyrt::xrefs::reg_access("modules.safety_cards", "BITE_DONE", g_BITE_DONE, s_BITE_DONE);
    pyrt::xrefs::reg_access("modules.safety_cards", "BLEEDING_SMALL", g_BLEEDING_SMALL, s_BLEEDING_SMALL);
    pyrt::xrefs::reg_access("modules.safety_cards", "BODY", g_BODY, s_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "BREATH_BARE", g_BREATH_BARE, s_BREATH_BARE);
    pyrt::xrefs::reg_access("modules.safety_cards", "BURNT_BODY_VERB", g_BURNT_BODY_VERB, s_BURNT_BODY_VERB);
    pyrt::xrefs::reg_access("modules.safety_cards", "BURN_ON_BODY", g_BURN_ON_BODY, s_BURN_ON_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "BURN_VERB", g_BURN_VERB, s_BURN_VERB);
    pyrt::xrefs::reg_access("modules.safety_cards", "COME_OFF_IT", g_COME_OFF_IT, s_COME_OFF_IT);
    pyrt::xrefs::reg_access("modules.safety_cards", "COUNTERFACTUAL_IDIOM", g_COUNTERFACTUAL_IDIOM, s_COUNTERFACTUAL_IDIOM);
    pyrt::xrefs::reg_access("modules.safety_cards", "COUNTERFACTUAL_WISH", g_COUNTERFACTUAL_WISH, s_COUNTERFACTUAL_WISH);
    pyrt::xrefs::reg_access("modules.safety_cards", "CRUSH", g_CRUSH, s_CRUSH);
    pyrt::xrefs::reg_access("modules.safety_cards", "CRUSH_HARD", g_CRUSH_HARD, s_CRUSH_HARD);
    pyrt::xrefs::reg_access("modules.safety_cards", "CRUSH_HEAVY_ON_A_PRONOUN", g_CRUSH_HEAVY_ON_A_PRONOUN, s_CRUSH_HEAVY_ON_A_PRONOUN);
    pyrt::xrefs::reg_access("modules.safety_cards", "CRUSH_OFF", g_CRUSH_OFF, s_CRUSH_OFF);
    pyrt::xrefs::reg_access("modules.safety_cards", "CRUSH_ON_A_PERSON", g_CRUSH_ON_A_PERSON, s_CRUSH_ON_A_PERSON);
    pyrt::xrefs::reg_access("modules.safety_cards", "CUT_BODY_PART", g_CUT_BODY_PART, s_CUT_BODY_PART);
    pyrt::xrefs::reg_access("modules.safety_cards", "CUT_ON_BODY", g_CUT_ON_BODY, s_CUT_ON_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "CUT_ON_THROAT_BODY", g_CUT_ON_THROAT_BODY, s_CUT_ON_THROAT_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "DOG_ATTACK", g_DOG_ATTACK, s_DOG_ATTACK);
    pyrt::xrefs::reg_access("modules.safety_cards", "EFFORT_BEFORE", g_EFFORT_BEFORE, s_EFFORT_BEFORE);
    pyrt::xrefs::reg_access("modules.safety_cards", "EFFORT_DIED", g_EFFORT_DIED, s_EFFORT_DIED);
    pyrt::xrefs::reg_access("modules.safety_cards", "EFFORT_DOING", g_EFFORT_DOING, s_EFFORT_DOING);
    pyrt::xrefs::reg_access("modules.safety_cards", "EFFORT_NOT_AN_ACCIDENT", g_EFFORT_NOT_AN_ACCIDENT, s_EFFORT_NOT_AN_ACCIDENT);
    pyrt::xrefs::reg_access("modules.safety_cards", "FOLLOW_UP", g_FOLLOW_UP, s_FOLLOW_UP);
    pyrt::xrefs::reg_access("modules.safety_cards", "FOOD_WORDS", g_FOOD_WORDS, s_FOOD_WORDS);
    pyrt::xrefs::reg_access("modules.safety_cards", "GRATER_CUT", g_GRATER_CUT, s_GRATER_CUT);
    pyrt::xrefs::reg_access("modules.safety_cards", "HEAD_OUT_COLD", g_HEAD_OUT_COLD, s_HEAD_OUT_COLD);
    pyrt::xrefs::reg_access("modules.safety_cards", "HEAD_REPLIES", g_HEAD_REPLIES, s_HEAD_REPLIES);
    pyrt::xrefs::reg_access("modules.safety_cards", "HEAL_TIME", g_HEAL_TIME, s_HEAL_TIME);
    pyrt::xrefs::reg_access("modules.safety_cards", "HEAT_BREATH_RED", g_HEAT_BREATH_RED, s_HEAT_BREATH_RED);
    pyrt::xrefs::reg_access("modules.safety_cards", "HEAT_CAUSE", g_HEAT_CAUSE, s_HEAT_CAUSE);
    pyrt::xrefs::reg_access("modules.safety_cards", "HEAT_NOT_WEATHER", g_HEAT_NOT_WEATHER, s_HEAT_NOT_WEATHER);
    pyrt::xrefs::reg_access("modules.safety_cards", "HIT_IN_FUN_HARD", g_HIT_IN_FUN_HARD, s_HIT_IN_FUN_HARD);
    pyrt::xrefs::reg_access("modules.safety_cards", "HOT_COMFORT", g_HOT_COMFORT, s_HOT_COMFORT);
    pyrt::xrefs::reg_access("modules.safety_cards", "HOT_SURFACE", g_HOT_SURFACE, s_HOT_SURFACE);
    pyrt::xrefs::reg_access("modules.safety_cards", "HOT_WAX_ON_BODY", g_HOT_WAX_ON_BODY, s_HOT_WAX_ON_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "HUMAN_BITTEN", g_HUMAN_BITTEN, s_HUMAN_BITTEN);
    pyrt::xrefs::reg_access("modules.safety_cards", "HUMAN_OWN_INJURY", g_HUMAN_OWN_INJURY, s_HUMAN_OWN_INJURY);
    pyrt::xrefs::reg_access("modules.safety_cards", "HYPERBOLE_WISH", g_HYPERBOLE_WISH, s_HYPERBOLE_WISH);
    pyrt::xrefs::reg_access("modules.safety_cards", "HYPO_SIGN", g_HYPO_SIGN, s_HYPO_SIGN);
    pyrt::xrefs::reg_access("modules.safety_cards", "INJURY_HOW", g_INJURY_HOW, s_INJURY_HOW);
    pyrt::xrefs::reg_access("modules.safety_cards", "INSECT_BITE", g_INSECT_BITE, s_INSECT_BITE);
    pyrt::xrefs::reg_access("modules.safety_cards", "INSECT_BITE_BODY", g_INSECT_BITE_BODY, s_INSECT_BITE_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "INSECT_BITE_NEEDS_A_LOOK", g_INSECT_BITE_NEEDS_A_LOOK, s_INSECT_BITE_NEEDS_A_LOOK);
    pyrt::xrefs::reg_access("modules.safety_cards", "LAUGH_BREATH_RED", g_LAUGH_BREATH_RED, s_LAUGH_BREATH_RED);
    pyrt::xrefs::reg_access("modules.safety_cards", "LAUGH_OTHER_PERSON", g_LAUGH_OTHER_PERSON, s_LAUGH_OTHER_PERSON);
    pyrt::xrefs::reg_access("modules.safety_cards", "LAUGH_TAKES_BREATH", g_LAUGH_TAKES_BREATH, s_LAUGH_TAKES_BREATH);
    pyrt::xrefs::reg_access("modules.safety_cards", "LAUGH_WORD", g_LAUGH_WORD, s_LAUGH_WORD);
    pyrt::xrefs::reg_access("modules.safety_cards", "LIMB_IS_A_THING", g_LIMB_IS_A_THING, s_LIMB_IS_A_THING);
    pyrt::xrefs::reg_access("modules.safety_cards", "LIMP_ONGOING", g_LIMP_ONGOING, s_LIMP_ONGOING);
    pyrt::xrefs::reg_access("modules.safety_cards", "LIMP_PERSON", g_LIMP_PERSON, s_LIMP_PERSON);
    pyrt::xrefs::reg_access("modules.safety_cards", "LIPS_SWELL_BARE", g_LIPS_SWELL_BARE, s_LIPS_SWELL_BARE);
    pyrt::xrefs::reg_access("modules.safety_cards", "MILD", g_MILD, s_MILD);
    pyrt::xrefs::reg_access("modules.safety_cards", "MINOR", g_MINOR, s_MINOR);
    pyrt::xrefs::reg_access("modules.safety_cards", "MY_OWN_PART", g_MY_OWN_PART, s_MY_OWN_PART);
    pyrt::xrefs::reg_access("modules.safety_cards", "NEAR_HIT_ON_A_ROAD", g_NEAR_HIT_ON_A_ROAD, s_NEAR_HIT_ON_A_ROAD);
    pyrt::xrefs::reg_access("modules.safety_cards", "NEAR_HIT_ON_A_ROAD_PLACE", g_NEAR_HIT_ON_A_ROAD_PLACE, s_NEAR_HIT_ON_A_ROAD_PLACE);
    pyrt::xrefs::reg_access("modules.safety_cards", "NOT_A_PET_LINE", g_NOT_A_PET_LINE, s_NOT_A_PET_LINE);
    pyrt::xrefs::reg_access("modules.safety_cards", "NOT_BURN_ON_BODY", g_NOT_BURN_ON_BODY, s_NOT_BURN_ON_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "NOT_JUST_A_NEAR_THING", g_NOT_JUST_A_NEAR_THING, s_NOT_JUST_A_NEAR_THING);
    pyrt::xrefs::reg_access("modules.safety_cards", "PART_OFF", g_PART_OFF, s_PART_OFF);
    pyrt::xrefs::reg_access("modules.safety_cards", "PART_OFF_NOT", g_PART_OFF_NOT, s_PART_OFF_NOT);
    pyrt::xrefs::reg_access("modules.safety_cards", "PART_OFF_PART", g_PART_OFF_PART, s_PART_OFF_PART);
    pyrt::xrefs::reg_access("modules.safety_cards", "PERSON_HANGING", g_PERSON_HANGING, s_PERSON_HANGING);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_ANIMAL_HURT", g_PET_ANIMAL_HURT, s_PET_ANIMAL_HURT);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_APPOSITIVE", g_PET_APPOSITIVE, s_PET_APPOSITIVE);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_APPOSITIVE_SUB", g_PET_APPOSITIVE_SUB, s_PET_APPOSITIVE_SUB);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_BREATH", g_PET_BREATH, s_PET_BREATH);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_BREATH_NOW", g_PET_BREATH_NOW, s_PET_BREATH_NOW);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_CANT_SETTLE", g_PET_CANT_SETTLE, s_PET_CANT_SETTLE);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_EFFORT", g_PET_EFFORT, s_PET_EFFORT);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_NAIL", g_PET_NAIL, s_PET_NAIL);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_OWNS_A_THING", g_PET_OWNS_A_THING, s_PET_OWNS_A_THING);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_OWNS_THE_WOUND", g_PET_OWNS_THE_WOUND, s_PET_OWNS_THE_WOUND);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_PASSES_BLOOD", g_PET_PASSES_BLOOD, s_PET_PASSES_BLOOD);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_WEAK", g_PET_WEAK, s_PET_WEAK);
    pyrt::xrefs::reg_access("modules.safety_cards", "PET_WOUND", g_PET_WOUND, s_PET_WOUND);
    pyrt::xrefs::reg_access("modules.safety_cards", "PURSUIT_FINE", g_PURSUIT_FINE, s_PURSUIT_FINE);
    pyrt::xrefs::reg_access("modules.safety_cards", "PURSUIT_PLACE_HOME", g_PURSUIT_PLACE_HOME, s_PURSUIT_PLACE_HOME);
    pyrt::xrefs::reg_access("modules.safety_cards", "RING_STUCK", g_RING_STUCK, s_RING_STUCK);
    pyrt::xrefs::reg_access("modules.safety_cards", "SCALD", g_SCALD, s_SCALD);
    pyrt::xrefs::reg_access("modules.safety_cards", "SCORCHED_A_PERSON", g_SCORCHED_A_PERSON, s_SCORCHED_A_PERSON);
    pyrt::xrefs::reg_access("modules.safety_cards", "SELF_HARM_HABIT", g_SELF_HARM_HABIT, s_SELF_HARM_HABIT);
    pyrt::xrefs::reg_access("modules.safety_cards", "SELF_HARM_REPEAT_CAUSE", g_SELF_HARM_REPEAT_CAUSE, s_SELF_HARM_REPEAT_CAUSE);
    pyrt::xrefs::reg_access("modules.safety_cards", "SHOULD_I_CALL", g_SHOULD_I_CALL, s_SHOULD_I_CALL);
    pyrt::xrefs::reg_access("modules.safety_cards", "SIGHT_EVER", g_SIGHT_EVER, s_SIGHT_EVER);
    pyrt::xrefs::reg_access("modules.safety_cards", "SIGHT_LIFE_REPLIES", g_SIGHT_LIFE_REPLIES, s_SIGHT_LIFE_REPLIES);
    pyrt::xrefs::reg_access("modules.safety_cards", "STING_WORDS", g_STING_WORDS, s_STING_WORDS);
    pyrt::xrefs::reg_access("modules.safety_cards", "STRUCK_BY_A_VEHICLE", g_STRUCK_BY_A_VEHICLE, s_STRUCK_BY_A_VEHICLE);
    pyrt::xrefs::reg_access("modules.safety_cards", "STRUCK_BY_A_VEHICLE_NOT", g_STRUCK_BY_A_VEHICLE_NOT, s_STRUCK_BY_A_VEHICLE_NOT);
    pyrt::xrefs::reg_access("modules.safety_cards", "STRUCK_MY_BIKE", g_STRUCK_MY_BIKE, s_STRUCK_MY_BIKE);
    pyrt::xrefs::reg_access("modules.safety_cards", "STRUCK_NOT_NOW", g_STRUCK_NOT_NOW, s_STRUCK_NOT_NOW);
    pyrt::xrefs::reg_access("modules.safety_cards", "SUGAR_READING_LOW", g_SUGAR_READING_LOW, s_SUGAR_READING_LOW);
    pyrt::xrefs::reg_access("modules.safety_cards", "TICK_ALREADY", g_TICK_ALREADY, s_TICK_ALREADY);
    pyrt::xrefs::reg_access("modules.safety_cards", "TICK_BITE", g_TICK_BITE, s_TICK_BITE);
    pyrt::xrefs::reg_access("modules.safety_cards", "TOOTH_CHIPPED", g_TOOTH_CHIPPED, s_TOOTH_CHIPPED);
    pyrt::xrefs::reg_access("modules.safety_cards", "TOOTH_OUT", g_TOOTH_OUT, s_TOOTH_OUT);
    pyrt::xrefs::reg_access("modules.safety_cards", "WATER_DANGER", g_WATER_DANGER, s_WATER_DANGER);
    pyrt::xrefs::reg_access("modules.safety_cards", "WAX_BODY", g_WAX_BODY, s_WAX_BODY);
    pyrt::xrefs::reg_access("modules.safety_cards", "WEIGHT_JOINT", g_WEIGHT_JOINT, s_WEIGHT_JOINT);
    pyrt::xrefs::reg_access("modules.safety_cards", "WILD_BITE", g_WILD_BITE, s_WILD_BITE);
    pyrt::xrefs::reg_access("modules.safety_cards", "_A_LOW_FIGURE", g__A_LOW_FIGURE, s__A_LOW_FIGURE);
    pyrt::xrefs::reg_access("modules.safety_cards", "_A_READING_NOUN", g__A_READING_NOUN, s__A_READING_NOUN);
    pyrt::xrefs::reg_access("modules.safety_cards", "_PET_APPOSITIVE", g__PET_APPOSITIVE, s__PET_APPOSITIVE);
    pyrt::xrefs::reg_access("modules.safety_cards", "_PET_DAMAGE_THING", g__PET_DAMAGE_THING, s__PET_DAMAGE_THING);
}
