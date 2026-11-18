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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_HEALTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUTRIENT_ROLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACK_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PORRIDGE_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BARE_OK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DAY_TEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DAY_WOMAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_GROWN_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATEMENT_ABOUT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATE_NOT_SPOILAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THERMOMETER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERDICT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CEREAL_BOWLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CEREAL_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILD_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COW_MILK_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EAT_BETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EDIBILITY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FUSSY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GOING_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ME_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILK_AND_THE_PLANET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILK_FOR_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MORE_OF_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMED_TAKEAWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PERSON_OR_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOURISHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PACK_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLANT_MILKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ROLE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SWEET_DRINKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_FRIDGE_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_MILK_RANKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_PLANET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VEG_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VEG_A_GROWN_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VEG_FIVE_A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VEG_WHO_IS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHICH_MILK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__food_avoid: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__food_id: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__food_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__food_names: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__raw_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_food_is_being_kept: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_food_has_gone_off: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_not_a_count_for_the_box_or_the_dish: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_not_a_present: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.nutrition: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING_VERBS() -> Py { ensure_init(); G_COOKING_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING_VERBS(v: Py) { ensure_init(); G_COOKING_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_WORDS() -> Py { ensure_init(); G_DAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_WORDS(v: Py) { ensure_init(); G_DAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_HEALTH() -> Py { ensure_init(); G_FOOD_HEALTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_HEALTH(v: Py) { ensure_init(); G_FOOD_HEALTH.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_AVOID() -> Py { ensure_init(); G_HEALTH_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_AVOID(v: Py) { ensure_init(); G_HEALTH_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_WORDS() -> Py { ensure_init(); G_HEALTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_WORDS(v: Py) { ensure_init(); G_HEALTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_OF() -> Py { ensure_init(); G_MORE_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_OF(v: Py) { ensure_init(); G_MORE_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_NUTRIENT_ROLE() -> Py { ensure_init(); G_NUTRIENT_ROLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUTRIENT_ROLE(v: Py) { ensure_init(); G_NUTRIENT_ROLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PACK_DATE() -> Py { ensure_init(); G_PACK_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACK_DATE(v: Py) { ensure_init(); G_PACK_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_PORRIDGE_VERDICT() -> Py { ensure_init(); G_PORRIDGE_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PORRIDGE_VERDICT(v: Py) { ensure_init(); G_PORRIDGE_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_WORDS() -> Py { ensure_init(); G__AGE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_WORDS(v: Py) { ensure_init(); G__AGE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BARE_OK_ASK() -> Py { ensure_init(); G__A_BARE_OK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BARE_OK_ASK(v: Py) { ensure_init(); G__A_BARE_OK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHILD() -> Py { ensure_init(); G__A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHILD(v: Py) { ensure_init(); G__A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DAY() -> Py { ensure_init(); G__A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DAY(v: Py) { ensure_init(); G__A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DAY_TEEN() -> Py { ensure_init(); G__A_DAY_TEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DAY_TEEN(v: Py) { ensure_init(); G__A_DAY_TEEN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DAY_WOMAN() -> Py { ensure_init(); G__A_DAY_WOMAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DAY_WOMAN(v: Py) { ensure_init(); G__A_DAY_WOMAN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DAY_WORDS() -> Py { ensure_init(); G__A_DAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DAY_WORDS(v: Py) { ensure_init(); G__A_DAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DOSE() -> Py { ensure_init(); G__A_DOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DOSE(v: Py) { ensure_init(); G__A_DOSE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_GROWN_UP() -> Py { ensure_init(); G__A_GROWN_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_GROWN_UP(v: Py) { ensure_init(); G__A_GROWN_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATEMENT_ABOUT_ME() -> Py { ensure_init(); G__A_STATEMENT_ABOUT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATEMENT_ABOUT_ME(v: Py) { ensure_init(); G__A_STATEMENT_ABOUT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATE_NOT_SPOILAGE() -> Py { ensure_init(); G__A_STATE_NOT_SPOILAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATE_NOT_SPOILAGE(v: Py) { ensure_init(); G__A_STATE_NOT_SPOILAGE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THERMOMETER() -> Py { ensure_init(); G__A_THERMOMETER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THERMOMETER(v: Py) { ensure_init(); G__A_THERMOMETER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERDICT_ASK() -> Py { ensure_init(); G__A_VERDICT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERDICT_ASK(v: Py) { ensure_init(); G__A_VERDICT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__CEREAL_BOWLS() -> Py { ensure_init(); G__CEREAL_BOWLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CEREAL_BOWLS(v: Py) { ensure_init(); G__CEREAL_BOWLS.with(|c| *c.borrow_mut() = v); }
pub fn g__CEREAL_SHORT() -> Py { ensure_init(); G__CEREAL_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CEREAL_SHORT(v: Py) { ensure_init(); G__CEREAL_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILD_AGE() -> Py { ensure_init(); G__CHILD_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILD_AGE(v: Py) { ensure_init(); G__CHILD_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__COW_MILK_REPLY() -> Py { ensure_init(); G__COW_MILK_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COW_MILK_REPLY(v: Py) { ensure_init(); G__COW_MILK_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__DISLIKE() -> Py { ensure_init(); G__DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISLIKE(v: Py) { ensure_init(); G__DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g__EAT_BETTER() -> Py { ensure_init(); G__EAT_BETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EAT_BETTER(v: Py) { ensure_init(); G__EAT_BETTER.with(|c| *c.borrow_mut() = v); }
pub fn g__EDIBILITY_ASK() -> Py { ensure_init(); G__EDIBILITY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EDIBILITY_ASK(v: Py) { ensure_init(); G__EDIBILITY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__FUSSY() -> Py { ensure_init(); G__FUSSY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FUSSY(v: Py) { ensure_init(); G__FUSSY.with(|c| *c.borrow_mut() = v); }
pub fn g__GOING_OFF() -> Py { ensure_init(); G__GOING_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GOING_OFF(v: Py) { ensure_init(); G__GOING_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g__ME_ASKING() -> Py { ensure_init(); G__ME_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ME_ASKING(v: Py) { ensure_init(); G__ME_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g__MILK_AND_THE_PLANET() -> Py { ensure_init(); G__MILK_AND_THE_PLANET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILK_AND_THE_PLANET(v: Py) { ensure_init(); G__MILK_AND_THE_PLANET.with(|c| *c.borrow_mut() = v); }
pub fn g__MILK_FOR_A_CHILD() -> Py { ensure_init(); G__MILK_FOR_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILK_FOR_A_CHILD(v: Py) { ensure_init(); G__MILK_FOR_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__MORE_OF_ASK() -> Py { ensure_init(); G__MORE_OF_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MORE_OF_ASK(v: Py) { ensure_init(); G__MORE_OF_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMED_TAKEAWAY() -> Py { ensure_init(); G__NAMED_TAKEAWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMED_TAKEAWAY(v: Py) { ensure_init(); G__NAMED_TAKEAWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PERSON_OR_PET() -> Py { ensure_init(); G__NOT_A_PERSON_OR_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PERSON_OR_PET(v: Py) { ensure_init(); G__NOT_A_PERSON_OR_PET.with(|c| *c.borrow_mut() = v); }
pub fn g__NOURISHING() -> Py { ensure_init(); G__NOURISHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOURISHING(v: Py) { ensure_init(); G__NOURISHING.with(|c| *c.borrow_mut() = v); }
pub fn g__PACK_DATE() -> Py { ensure_init(); G__PACK_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PACK_DATE(v: Py) { ensure_init(); G__PACK_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLANT_MILKS() -> Py { ensure_init(); G__PLANT_MILKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLANT_MILKS(v: Py) { ensure_init(); G__PLANT_MILKS.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANT() -> Py { ensure_init(); G__PREGNANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANT(v: Py) { ensure_init(); G__PREGNANT.with(|c| *c.borrow_mut() = v); }
pub fn g__ROLE_ASK() -> Py { ensure_init(); G__ROLE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ROLE_ASK(v: Py) { ensure_init(); G__ROLE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSE() -> Py { ensure_init(); G__SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE(v: Py) { ensure_init(); G__SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SWEET_DRINKS() -> Py { ensure_init(); G__SWEET_DRINKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SWEET_DRINKS(v: Py) { ensure_init(); G__SWEET_DRINKS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_FRIDGE_CHECK() -> Py { ensure_init(); G__THE_FRIDGE_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_FRIDGE_CHECK(v: Py) { ensure_init(); G__THE_FRIDGE_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_MILK_RANKING() -> Py { ensure_init(); G__THE_MILK_RANKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_MILK_RANKING(v: Py) { ensure_init(); G__THE_MILK_RANKING.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_PLANET() -> Py { ensure_init(); G__THE_PLANET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_PLANET(v: Py) { ensure_init(); G__THE_PLANET.with(|c| *c.borrow_mut() = v); }
pub fn g__VEG_A_CHILD() -> Py { ensure_init(); G__VEG_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VEG_A_CHILD(v: Py) { ensure_init(); G__VEG_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__VEG_A_GROWN_UP() -> Py { ensure_init(); G__VEG_A_GROWN_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VEG_A_GROWN_UP(v: Py) { ensure_init(); G__VEG_A_GROWN_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__VEG_FIVE_A_DAY() -> Py { ensure_init(); G__VEG_FIVE_A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VEG_FIVE_A_DAY(v: Py) { ensure_init(); G__VEG_FIVE_A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__VEG_WHO_IS_IT() -> Py { ensure_init(); G__VEG_WHO_IS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VEG_WHO_IS_IT(v: Py) { ensure_init(); G__VEG_WHO_IS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__WHICH_MILK() -> Py { ensure_init(); G__WHICH_MILK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHICH_MILK(v: Py) { ensure_init(); G__WHICH_MILK.with(|c| *c.borrow_mut() = v); }
pub fn g__food_avoid() -> Py { ensure_init(); G__food_avoid.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__food_avoid(v: Py) { ensure_init(); G__food_avoid.with(|c| *c.borrow_mut() = v); }
pub fn g__food_id() -> Py { ensure_init(); G__food_id.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__food_id(v: Py) { ensure_init(); G__food_id.with(|c| *c.borrow_mut() = v); }
pub fn g__food_line() -> Py { ensure_init(); G__food_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__food_line(v: Py) { ensure_init(); G__food_line.with(|c| *c.borrow_mut() = v); }
pub fn g__food_names() -> Py { ensure_init(); G__food_names.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__food_names(v: Py) { ensure_init(); G__food_names.with(|c| *c.borrow_mut() = v); }
pub fn g__raw_entry() -> Py { ensure_init(); G__raw_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__raw_entry(v: Py) { ensure_init(); G__raw_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_a_food_is_being_kept() -> Py { ensure_init(); G_a_food_is_being_kept.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_food_is_being_kept(v: Py) { ensure_init(); G_a_food_is_being_kept.with(|c| *c.borrow_mut() = v); }
pub fn g_food_has_gone_off() -> Py { ensure_init(); G_food_has_gone_off.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_food_has_gone_off(v: Py) { ensure_init(); G_food_has_gone_off.with(|c| *c.borrow_mut() = v); }
pub fn g_not_a_count_for_the_box_or_the_dish() -> Py { ensure_init(); G_not_a_count_for_the_box_or_the_dish.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_not_a_count_for_the_box_or_the_dish(v: Py) { ensure_init(); G_not_a_count_for_the_box_or_the_dish.with(|c| *c.borrow_mut() = v); }
pub fn g_not_a_present() -> Py { ensure_init(); G_not_a_present.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_not_a_present(v: Py) { ensure_init(); G_not_a_present.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What is in food, and how much of it to eat: the everyday healthy-eating questions.\n\nThe answers lane owns this domain. People ask these as facts (\"how many calories in a\nbanana\"), as worries (\"is pasta fattening\", \"are eggs bad for cholesterol\") and as habits\nthey want to change (\"how do I stop eating sugar at night\"). Entries are keyed by the food\nor the nutrient, not by the wording, so one row answers \"how much sugar is in coke\" and\n\"is there a lot of sugar in a can of coke\".\n\nCooking a dish lives in cooking.py and dishes.py; a portion of pasta or rice in measures.py;\nprotein for training and food around a run in sport.py. This module is the nutrition side:\ncalories, sugar, salt, fibre, fat, alcohol, caffeine, and the habits that people ask about by\nname. Numbers are UK guidance for an ordinary adult (a 2,000 kcal a day reference intake),\nsaid plainly, with the \"see your GP\" line only where a diet question is really a health one.\n");
    s_COOKING_VERBS(pyrt::xrefs::xget("knowledge.cooking", "COOKING_VERBS"));
    s_DAY_WORDS(pyrt::xrefs::xget("knowledge.lookup", "DAY_WORDS"));
    s_PACK_DATE(pyrt::xrefs::xget("knowledge.lookup", "PACK_DATE"));
    s_a_food_is_being_kept(py_func("a_food_is_being_kept", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "a_food_is_being_kept", args)));
    s__raw_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_food_has_gone_off(py_func("food_has_gone_off", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "food_has_gone_off", args)));
    s_not_a_count_for_the_box_or_the_dish(py_func("not_a_count_for_the_box_or_the_dish", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "not_a_count_for_the_box_or_the_dish", args)));
    s_not_a_present(py_func("not_a_present", |args: &[Py]| pyrt::xrefs::xcall("knowledge.shopping", "not_a_present", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("health"), py_str("food-and-drink"), py_str("fitness")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._PACK_DATE"); }
    s__PACK_DATE(g_PACK_DATE());
    fn __data1() -> Py { py_tuple(vec![py_str("my dog"), py_str("my cat"), py_str("puppy"), py_str("kitten"), py_str("dog"), py_str("cat"), py_str("hamster"), py_str("rabbit"), py_str("parrot"), py_str("baby"), py_str("babies"), py_str("newborn"), py_str("toddler"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my kids"), py_str("children"), py_str("child"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("nursing"), py_str("year old"), py_str("years old"), py_str("year-old"), py_str("year old son"), py_str("my four year old"), py_str("under one"), py_str("weaning"), py_str("allergic"), py_str("allergy"), py_str("intolerance"), py_str("coeliac"), py_str("diabetic"), py_str("diabetes"), py_str("kidney"), py_str("kidneys"), py_str("warfarin"), py_str("medication"), py_str("on tablets")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._NOT_A_PERSON_OR_PET"); }
    s__NOT_A_PERSON_OR_PET(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_DOSE"); }
    s__A_DOSE(stdlib_call("re", "compile", &[py_str("\\b(?:vitamins?|multivitamins?|supplements?|probiotics?|iron|folic\\s+acid|calcium|tablets?|pills?|capsules?|creatine|magnesium|omega\\s?3|collagen|melatonin|electrolytes?|ibuprofen|paracetamol|aspirin|naproxen|painkillers?|medicines?|medication|antibiotics?|steroids?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("dont eat"), py_str("don't eat"), py_str("do not eat"), py_str("im vegetarian"), py_str("i'm vegetarian"), py_str("im vegan"), py_str("i'm vegan"), py_str("no dairy"), py_str("dairy free"), py_str("lactose"), py_str("cut out"), py_str("gave up"), py_str("quit meat"), py_str("dont drink"), py_str("don't drink"), py_str("im allergic"), py_str("i'm allergic")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_STATEMENT_ABOUT_ME"); }
    s__A_STATEMENT_ABOUT_ME(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("dont eat veg"), py_str("don't eat veg"), py_str("dont eat vegetables"), py_str("don't eat vegetables"), py_str("dont eat any veg"), py_str("don't eat any veg"), py_str("dont eat fruit"), py_str("don't eat fruit"), py_str("dont eat salad"), py_str("don't eat salad"), py_str("dont eat greens"), py_str("don't eat greens"), py_str("never eat veg"), py_str("never eat vegetables"), py_str("dont like veg"), py_str("don't like veg"), py_str("dont like vegetables"), py_str("don't like vegetables")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._DISLIKE"); }
    s__DISLIKE(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str(" i wont eat"), py_str(" i won't eat"), py_str(" i refuse"), py_str(" i dont eat"), py_str(" i don't eat"), py_str(" i never eat"), py_str(" i hate veg"), py_str(" i hate vegetables"), py_str(" i hate vegtables"), py_str(" i cant eat"), py_str(" i can't eat"), py_str(" i cant stand"), py_str(" i can't stand"), py_str(" i want to like"), py_str(" i want to eat"), py_str(" i want to start eating"), py_str(" i wish i liked"), py_str(" i wish i could eat"), py_str(" i'd like to like"), py_str(" id like to like"), py_str(" im a fussy"), py_str(" i am a fussy"), py_str(" im picky"), py_str(" i am picky"), py_str(" help me like"), py_str(" for myself"), py_str(" myself")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._ME_ASKING"); }
    s__ME_ASKING(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str(" kid"), py_str(" kids"), py_str(" child"), py_str(" children"), py_str(" son"), py_str(" sons"), py_str(" daughter"), py_str(" daughters"), py_str(" lad"), py_str(" lads"), py_str(" lass"), py_str(" wee one"), py_str(" toddler"), py_str(" baby"), py_str(" year old"), py_str(" years old"), py_str(" year-old"), py_str(" nursery"), py_str(" my boy "), py_str(" my girl "), py_str(" teenager"), py_str(" weaning"), py_str(" grandson"), py_str(" granddaughter"), py_str(" grandchild"), py_str(" stepson"), py_str(" stepdaughter"), py_str(" niece"), py_str(" nephew"), py_str(" boy"), py_str(" girl"), py_str(" little one"), py_str(" lil un"), py_str(" lil one"), py_str(" our kid"), py_str(" youngster"), py_str(" nipper"), py_str(" infant"), py_str(" preschool"), py_str(" pre school"), py_str(" school age")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_CHILD"); }
    s__A_CHILD(__data5());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._FUSSY"); }
    s__FUSSY(py_tuple(vec![py_str(" fussy"), py_str(" fussy eater"), py_str(" picky"), py_str(" picky eater"), py_str(" faddy")]));
    fn __data6() -> Py { py_tuple(vec![py_str(" husband"), py_str(" wife"), py_str(" partner"), py_str(" boyfriend"), py_str(" girlfriend"), py_str(" fiance"), py_str(" fiancee"), py_str(" fiancé"), py_str(" fiancée"), py_str(" other half"), py_str(" my man"), py_str(" my fella"), py_str(" mate"), py_str(" mates"), py_str(" friend"), py_str(" flatmate"), py_str(" housemate"), py_str(" brother"), py_str(" sister"), py_str(" mum"), py_str(" mom"), py_str(" mother"), py_str(" dad"), py_str(" father"), py_str(" adult"), py_str(" grown up"), py_str(" grownup"), py_str(" colleague"), py_str(" lodger"), py_str(" in laws"), py_str(" mother in law"), py_str(" father in law")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_GROWN_UP"); }
    s__A_GROWN_UP(__data6());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._VEG_FIVE_A_DAY"); }
    s__VEG_FIVE_A_DAY(py_str("Five a day is 400g, and a portion is a handful - about 80g - so one carrot, one apple, three heaped tablespoons of peas or a cereal bowl of salad each count. The easy version is to add rather than swap: a handful of frozen peas or spinach into whatever you are cooking anyway, and fruit where you can see it."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._VEG_A_CHILD"); }
    s__VEG_A_CHILD(py_str("A child refusing veg is normal and nearly all of them grow out of it. Keep putting a small amount on the plate with no pressure and no pudding bribe, keep serving the two or three they do eat (peas, sweetcorn, raw carrot), and grate or blend the rest into what they already like - carrot and courgette into a bolognese, spinach into soup, cauliflower into mash. Eat the same food yourself at the same table, and expect eight or ten tries before a new one is accepted: a refused spoon today is not a lost cause."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._VEG_A_GROWN_UP"); }
    s__VEG_A_GROWN_UP(py_str("Cook one meal and serve it in parts, keeping the vegetables separate so a portion can be taken or left without it becoming a scene, and put a new one beside something they already like rather than on its own. Policing the plate makes it worse at any age - a handful of dinners eaten happily beats a row twice a week - and hiding it is fair too: grate carrot or courgette into a bolognese, spinach into soup, cauliflower into mash. A refused vegetable usually needs eight or ten calm tries before it is accepted, so one refusal is not a verdict."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._VEG_WHO_IS_IT"); }
    s__VEG_WHO_IS_IT(py_str("Who is it who won't eat them - you, a partner, or a child? What helps is different for each: five a day is a plate to add to, a grown-up gets a portion they can take or leave, and a child gets no pressure and eight or ten tries."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._SOMEONE_ELSE"); }
    s__SOMEONE_ELSE(py_tuple(vec![py_str(" he "), py_str(" she "), py_str(" him "), py_str(" her "), py_str(" his "), py_str(" hers "), py_str(" the kids "), py_str(" the children ")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._CHILD_AGE"); }
    s__CHILD_AGE(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,3})\\s*(?:year|yr)s?[\\s-]*old\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._AGE_WORDS"); }
    s__AGE_WORDS(py_tuple(vec![py_str(" year old"), py_str(" years old"), py_str(" year-old")]));
    fn __data7() -> Py { py_tuple(vec![py_str("eat better"), py_str("eat healthier"), py_str("eating better"), py_str("eating healthier"), py_str("eat more healthily"), py_str("improve my diet"), py_str("eat properly"), py_str("eat well"), py_str("better diet"), py_str("healthy diet"), py_str("healthy eating"), py_str("eat clean"), py_str("eat like")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._EAT_BETTER"); }
    s__EAT_BETTER(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_DAY"); }
    s__A_DAY(py_dict(vec![(py_str("calcium"), py_dict(vec![(py_str("adult"), py_str("About 700mg of calcium a day for an adult, which is a pint of milk or a pot of yoghurt and a matchbox of cheese. Cheese, tinned sardines, almonds and fortified cereal all count, and the figure is the same for men and women.")), (py_str("teen"), py_str("Teenagers need about 1,000mg of calcium a day for a boy and 800mg for a girl, which is a pint of milk plus a pot of yoghurt, with cheese or tinned sardines through the week. Those are the years the bones lay down their density, so it matters more now than it will at 30."))])), (py_str("iron"), py_dict(vec![(py_str("adult"), py_str("Men need about 9mg of iron a day and women about 15mg until the menopause, which is why women run low far more often. Red meat, lentils, beans, fortified cereal and dark green veg are the sources; vitamin C in the same meal helps you take it up, and tea or coffee with the meal blocks it.")), (py_str("woman"), py_str("About 15mg of iron a day, which is nearly twice what a man needs, because of the monthly loss: lentils, beans, fortified cereal, red meat and dark green veg are the sources, and vitamin C in the same meal helps you take it up. Tea and coffee with food block it, so leave a gap either side.")), (py_str("teen"), py_str("A teenage boy needs about 11mg of iron a day and a teenage girl about 15mg, because she loses some every month and both of them are growing fast. Lentils, beans, fortified cereal, red meat and dark green veg are the sources, and vitamin C at the same meal helps; tea and coffee with food work against it."))])), (py_str("protein"), py_dict(vec![(py_str("adult"), py_str("About 0.75g of protein per kilo of body weight a day, which is roughly 50g for a 65kg adult: a palm-sized portion at each meal covers it, from eggs, milk, beans, lentils, chicken or fish. You do not need to count grams to get there.")), (py_str("teen"), py_str("A teenager needs about the same protein per kilo as an adult, so 45g to 55g a day for most of them: a palm-sized portion at each meal, and more if they are training hard or growing fast. Eggs, milk, beans, lentils, chicken and fish all count."))])), (py_str("magnesium"), py_dict(vec![(py_str("adult"), py_str("Men need about 300mg of magnesium a day and women about 270mg, and wholemeal bread, nuts, beans, green veg and dark chocolate cover it without any supplement. A supplement is for a low a doctor has actually found."))])), (py_str("potassium"), py_dict(vec![(py_str("adult"), py_str("The target is about 3,500mg a day, and most people fall short of it. A banana is about 430mg, so potatoes, beans, milk, fruit and vegetables are what closes the gap; there is no need for a supplement."))])), (py_str("vitamin d"), py_dict(vec![(py_str("adult"), py_str("Take 10 micrograms (400 IU) of vitamin D a day through autumn and winter, roughly October to March, and all year if you cover up or stay indoors. Take it as a supplement rather than hoping for it from food: oily fish, eggs and fortified spread will not get you there.")), (py_str("teen"), py_str("Teenagers want the same 10 micrograms (400 IU) of vitamin D a day, from a supplement from October to March and all year if they cover up or are hardly outside. It is the one vitamin worth buying, because the food eaten will not supply it and the UK sun does not in winter."))])), (py_str("vitamin c"), py_dict(vec![(py_str("adult"), py_str("Aim for 40mg of vitamin C a day: one orange, a couple of peppers or a handful of broccoli covers it, so a supplement is money for nothing unless a doctor has told you otherwise. It also helps you take up the iron in beans and cereal at the same meal."))])), (py_str("vitamin b12"), py_dict(vec![(py_str("adult"), py_str("You need 1.5 micrograms of vitamin B12 a day, which any diet with meat, fish, covers easily. Vegans need it from fortified milk or cereal or a tablet, because plants do not make it and the stores you are born with run out after a few years."))])), (py_str("folic acid"), py_dict(vec![(py_str("adult"), py_str("Take 200 micrograms of folic acid a day as an adult, from a supplement or fortified cereal. If you are trying for a baby, or in the first 12 weeks of pregnancy, it is 400 micrograms and it wants starting before you conceive."))])), (py_str("zinc"), py_dict(vec![(py_str("adult"), py_str("Men need 9.5mg of zinc a day and women 7mg, which meat, beans, nuts, cheese and wholegrain cereal cover. More than 25mg a day for a long stretch works against copper, so a big zinc supplement is not a thing to take on a guess."))]))]));
    fn __data8() -> Py { py_tuple(vec![py_tuple(vec![py_str("vitamin b12"), py_str("vitamin b12")]), py_tuple(vec![py_str("vitamin b-12"), py_str("vitamin b12")]), py_tuple(vec![py_str("b12"), py_str("vitamin b12")]), py_tuple(vec![py_str("folic acid"), py_str("folic acid")]), py_tuple(vec![py_str("folate"), py_str("folic acid")]), py_tuple(vec![py_str("folic"), py_str("folic acid")]), py_tuple(vec![py_str("vitamin c"), py_str("vitamin c")]), py_tuple(vec![py_str("vit c"), py_str("vitamin c")]), py_tuple(vec![py_str("vitamin d"), py_str("vitamin d")]), py_tuple(vec![py_str("vit d"), py_str("vitamin d")]), py_tuple(vec![py_str("calcium"), py_str("calcium")]), py_tuple(vec![py_str("magnesium"), py_str("magnesium")]), py_tuple(vec![py_str("potassium"), py_str("potassium")]), py_tuple(vec![py_str("zinc"), py_str("zinc")]), py_tuple(vec![py_str("iron"), py_str("iron")]), py_tuple(vec![py_str("protein"), py_str("protein")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_DAY_WORDS"); }
    s__A_DAY_WORDS(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("teens"), py_str("teenage"), py_str("16 year"), py_str("15 year"), py_str("14 year"), py_str("13 year"), py_str("17 year"), py_str("18 year"), py_str("year 10"), py_str("year 11"), py_str("secondary school")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_DAY_TEEN"); }
    s__A_DAY_TEEN(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("woman"), py_str("women"), py_str("female"), py_str("girl"), py_str("girls"), py_str("period"), py_str("periods"), py_str("menstrual"), py_str("menstruating"), py_str("menopause"), py_str("pregnant"), py_str("trying to conceive"), py_str("breastfeeding")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_DAY_WOMAN"); }
    s__A_DAY_WOMAN(__data10());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._CEREAL_BOWLS"); }
    s__CEREAL_BOWLS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("granola"), py_str("crunchy clusters"), py_str("crunchy nut clusters")]), py_str("Granola is oats with oil and sugar baked on, so a 50g serving usually carries about 11 to 15g of sugar, and a bowl of it with milk is closer to a pudding than to porridge. It is not a bad breakfast, it is a portion problem: measure 40g into a bowl rather than pouring from the bag, pick a bag with under 10g of sugar per 100g, and eat it with plain yoghurt and fruit, which is the fibre and the protein that make it last until lunch.")]), py_tuple(vec![py_tuple(vec![py_str("muesli"), py_str("bircher")]), py_str("Muesli is oats, nuts and dried fruit with no baking, so it carries less added sugar than granola - usually 7g to 12g per 50g, most of it from the raisins and dates. The fibre is good and the oats are wholegrain; measure it, because a mugful of muesli plus milk is a large bowl of energy, and the dried fruit is what adds up.")]), py_tuple(vec![py_tuple(vec![py_str("cornflakes"), py_str("corn flakes"), py_str("rice krispies"), py_str("coco pops"), py_str("frosties")]), py_str("A plain cornflake is mostly refined flour with a little sugar, and a 30g bowl is about 8g of sugar before the milk. It is not poison, and a fortified one does give you your B vitamins and iron; it is just the breakfast that keeps you least full for its calories, so add fruit or yoghurt to it rather than a second bowl.")]), py_tuple(vec![py_tuple(vec![py_str("bran flakes"), py_str("all bran"), py_str("weetabix"), py_str("shreddies"), py_str("shredded wheat"), py_str("wheat biscuits")]), py_str("These are the wholegrain end of the cereal shelf: bran flakes and Weetabix carry several times the fibre of cornflakes for about the same calories, and Shredded Wheat has nothing added at all. Two Weetabix with milk and a banana is a genuinely filling breakfast with about 4g of fibre, and the sugar comes from the banana rather than the box.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._CEREAL_SHORT"); }
    s__CEREAL_SHORT(py_dict(vec![(py_str("granola"), py_str("Granola is oats with oil and sugar baked on: fine, but it is a 40g portion - a 50g serving carries about 11g to 15g of sugar, so pick a bag under 10g per 100g.")), (py_str("muesli"), py_str("Muesli is unbaked oats, nuts and dried fruit: more fibre than granola and less added sugar, though the raisins still count, so measure the bowl.")), (py_str("cornflakes"), py_str("Cornflakes are refined flour with about 8g of sugar in a 30g bowl: the least filling of them for the calories.")), (py_str("bran flakes"), py_str("Bran flakes and Weetabix are the wholegrain end, with several times the fibre of cornflakes for about the same calories.")), (py_str("porridge"), py_str("Porridge is the plainest and the best everyday bowl: no added sugar, about 4g of fibre in 40g, and it keeps you full until lunch."))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.PORRIDGE_VERDICT"); }
    s_PORRIDGE_VERDICT(py_str("Porridge is the better everyday bowl: oats are wholegrain, and a 40g serving has about 4g of fibre and no added sugar, so it keeps you full until lunch. Plain cereal does the same job with less fibre; granola and crunchy clusters are the ones to watch, because a 50g serving can carry 12 to 15g of sugar. If you want cold cereal, pick a wholegrain one with under 10g of sugar per 100g."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._PREGNANT"); }
    s__PREGNANT(py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("trying for a baby"), py_str("trying to conceive"), py_str("expecting a baby"), py_str("we are trying"), py_str("first 12 weeks"), py_str("antenatal"), py_str("breastfeeding"), py_str("breast feeding"), py_str("months pregnant"), py_str("weeks pregnant")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.MORE_OF"); }
    s_MORE_OF(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("iron"), py_str("ferritin"), py_str("anaemia"), py_str("anemia"), py_str("anaemic"), py_str("anemic")]), py_str("Red meat, liver, lentils, beans, chickpeas, tofu, fortified cereal and dark green veg are the everyday sources, and a bowl of fortified cereal or a lentil curry is as good as a steak. Have vitamin C in the same meal - a glass of orange juice, peppers, tomatoes - because it can double how much you take up, and keep tea and coffee an hour away from the meal, since the tannins block it. A low a blood test has found is treated with tablets.")]), py_tuple(vec![py_tuple(vec![py_str("calcium")]), py_str("A pint of milk, or a pot of yoghurt and a matchbox-sized piece of cheese, covers the 700mg an adult needs. Tinned sardines or salmon with the bones, tofu set with calcium, fortified plant milk, and kale, broccoli or watercress are the ones that beat the dairy doubters. Vitamin D is what gets it into the bones, which is why the two are talked about together.")]), py_tuple(vec![py_tuple(vec![py_str("vitamin d"), py_str("vit d")]), py_str("Sun is the main source and food will not cover it: from about October to March in the UK the sun is too weak, so 10 micrograms (400 IU) a day from a supplement is the standard advice for everyone. Oil-rich fish, egg yolk, and fortified cereal or spread have some, but you would need a lot of them. Do not take more than 100 micrograms a day.")]), py_tuple(vec![py_tuple(vec![py_str("b12"), py_str("vitamin b12"), py_str("b-12")]), py_str("B12 is only in animal foods: meat, fish, milk, cheese and eggs, so a small pot of yoghurt or a portion of fish most days covers it. If you eat vegan, or very little of those, take a supplement (about 10 micrograms a day or 2000 a week) or choose plant milks with B12 added, because the body's stores last years and the shortage creeps up unnoticed.")]), py_tuple(vec![py_tuple(vec![py_str("potassium")]), py_str("Bananas get the credit, but potatoes, beans, lentils, tomatoes, spinach and orange juice have as much or more. Adults want about 3,500mg a day, and five portions of veg plus a potato or a banana gets most people there. If a kidney problem has been diagnosed, ask before adding potassium-rich food.")]), py_tuple(vec![py_tuple(vec![py_str("magnesium")]), py_str("Nuts, seeds, wholemeal bread, porridge oats, beans, lentils and dark green veg are the sources, and about 300mg a day covers an adult. A handful of nuts and a bowl of porridge is most of it. Supplements are only worth it if a blood test has shown a low.")]), py_tuple(vec![py_tuple(vec![py_str("zinc")]), py_str("Meat, shellfish, eggs, milk, beans, lentils and wholemeal bread are the sources, and adults need about 9.5mg a day for men and 7mg for women. Oysters, beef and pumpkin seeds are the richest. Taking zinc long-term at high doses upsets the copper balance, so a supplement is for a low a blood test has found.")]), py_tuple(vec![py_tuple(vec![py_str("vitamin c"), py_str("vit c")]), py_str("Peppers, oranges, kiwi, broccoli, tomatoes and potatoes with the skin on are the everyday sources, and a glass of orange juice is plenty for the day. Vitamin C is easily destroyed by heat, so steam or eat them raw where you can, and five portions of fruit and veg covers an adult without anything extra.")]), py_tuple(vec![py_tuple(vec![py_str("folate"), py_str("folic acid")]), py_str("Green veg, lentils, beans, oranges and fortified cereal are the everyday sources, and about 200 micrograms a day covers an adult without trying. Anyone pregnant, or trying for a baby, is advised 400 micrograms of folic acid a day as a supplement from before conception until 12 weeks, and that is not something food alone can cover.")]), py_tuple(vec![py_tuple(vec![py_str("omega 3"), py_str("omega-3"), py_str("oily fish"), py_str("fish oil")]), py_str("One portion of oily fish a week is the advice: salmon, mackerel, sardines, herring, trout, pilchards or fresh tuna, about 140g cooked. Tinned sardines or mackerel count and are the cheapest way. If you do not eat fish, walnuts, chia, flaxseed and rapeseed oil give the plant kind, which the body converts only a little of, so a supplement is reasonable for a vegan.")])]));
    fn __data11() -> Py { py_tuple(vec![py_str("more"), py_str("get more"), py_str("getting more"), py_str("increase"), py_str("boost"), py_str("add"), py_str("sources"), py_str("source"), py_str("foods"), py_str("which foods"), py_str("what foods"), py_str("high in"), py_str("rich in"), py_str("best way"), py_str("best ways"), py_str("how do i get"), py_str("how can i get"), py_str("want more"), py_str("need more"), py_str("plenty of"), py_str("top up"), py_str("up my"), py_str("deficient"), py_str("deficiency"), py_str("enough"), py_str("eat for"), py_str("have for"), py_str("should i eat"), py_str("can i eat for"), py_str("could i eat for"), py_str("do i eat"), py_str("what to eat"), py_str("should i have")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._MORE_OF_ASK"); }
    s__MORE_OF_ASK(__data11());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.NUTRIENT_ROLE"); }
    s_NUTRIENT_ROLE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("iron"), py_str("ferritin")]), py_str("Iron carries oxygen round your body in red blood cells, which is why a shortage shows up as tiredness, breathlessness on the stairs, pallor and brittle nails or hair loss. Red meat, liver, lentils, beans, fortified cereal and dark green veg are the everyday sources, and vitamin C in the same meal helps you take it up. Too much iron is harmful, so tablets are for a low a blood test has found rather than for a guess.")]), py_tuple(vec![py_tuple(vec![py_str("calcium")]), py_str("Calcium is what bones and teeth are built from, and it also matters for muscles and for blood clotting: an adult needs about 700mg a day, which is a pint of milk or a pot of yoghurt and a matchbox of cheese. Vitamin D is what gets it into the bones, so the two go together. A supplement is worth it only if you cannot manage the dairy, and 500mg is plenty.")]), py_tuple(vec![py_tuple(vec![py_str("vitamin d"), py_str("vit d")]), py_str("Vitamin D is what lets your body take up calcium, so it is the bones and muscles that suffer without it, and a low is linked with tiredness and aching. Sun makes most of it, so in the UK take 10 micrograms (400 IU) a day from October to March, and all year if you cover up or are indoors a lot. More than 100 micrograms a day is too much.")]), py_tuple(vec![py_tuple(vec![py_str("vitamin c"), py_str("vit c")]), py_str("Vitamin C is for skin, gums, blood vessels and healing, and it can double how much iron you take up from a meal. 40mg a day is plenty, and one orange, a couple of peppers or a handful of broccoli covers it, so a supplement is money for nothing. It is destroyed by heat, so steam rather than boil where you can.")]), py_tuple(vec![py_tuple(vec![py_str("vitamin b12"), py_str("b12"), py_str("b-12")]), py_str("B12 is for nerves and red blood cells, and a shortage creeps up over years because the liver stores it: tingling hands, tiredness, a sore tongue and low mood are the usual signs. It is only in animal foods - meat, fish, milk, eggs - so if you eat vegan take it from fortified milk or cereal, or a tablet of about 10 micrograms a day.")]), py_tuple(vec![py_tuple(vec![py_str("magnesium")]), py_str("Magnesium is used in hundreds of reactions, from muscles and nerves to blood sugar, and an adult needs about 300mg a day. Nuts, seeds, wholemeal bread, porridge, beans, lentils and dark chocolate cover it, so a supplement buys little unless a blood test has found a low.")]), py_tuple(vec![py_tuple(vec![py_str("zinc")]), py_str("Zinc is for healing, immunity and the sense of taste and smell, and adults need about 9.5mg a day, 7mg for women: meat, shellfish, eggs, milk, beans and wholemeal bread cover it. More than 25mg a day for months works against copper, so a big zinc supplement is not worth it.")]), py_tuple(vec![py_tuple(vec![py_str("potassium")]), py_str("Potassium is what keeps the heart beating evenly and the muscles working, and adults want about 3,500mg a day. Potatoes, beans, lentils, milk, bananas and tomatoes are the everyday sources, and most people fall short of it without trying. If a kidney problem has been diagnosed, ask before adding potassium-rich food.")]), py_tuple(vec![py_tuple(vec![py_str("folate"), py_str("folic acid")]), py_str("Folate is for making new cells, which is why it matters most in early pregnancy: 400 micrograms a day from before conception until week 12 cuts the risk of spina bifida. Outside pregnancy, green veg, beans, lentils and fortified cereal cover the 200 micrograms an adult needs.")])]));
    fn __data12() -> Py { py_tuple(vec![py_str("good for"), py_str("bad for"), py_str("what does"), py_str("does it do"), py_str("why do i need"), py_str("why do you need"), py_str("benefits"), py_str("benefit"), py_str("important"), py_str("needed"), py_str("necessary"), py_str("what is it for"), py_str("point of"), py_str("any good"), py_str("useful"), py_str("role"), py_str("what is the point")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._ROLE_ASK"); }
    s__ROLE_ASK(__data12());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._PLANT_MILKS"); }
    s__PLANT_MILKS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("soy milk"), py_str("soya milk"), py_str("soy bean milk"), py_str("soybean milk"), py_str("soy"), py_str("soya")]), py_str("Soya is the closest to milk: about 7g of protein in 200ml, the same as semi-skimmed, and it is the one to take if protein is what you are replacing. Check the carton says calcium and B12 added - those come from the fortification, not from the bean - and buy it unsweetened, because the sweetened soya is nearer a pudding.")]), py_tuple(vec![py_tuple(vec![py_str("oat milk"), py_str("oat drink"), py_str("oatly"), py_str("oat")]), py_str("Oat is the creamiest and the best in coffee and porridge: about 2g of protein in 200ml, and 7 to 8g of sugar that comes from the oats themselves rather than from a spoon. Buy the barista or no-added-sugar carton, and check it is fortified with calcium and B12, because unsweetened oat is thin on both.")]), py_tuple(vec![py_tuple(vec![py_str("almond milk"), py_str("almond drink"), py_str("almond")]), py_str("Almond is the lightest: about 1g of protein and 50 calories in 200ml, and thin in coffee. It is worth having only if the carton is fortified, since almonds are a small part of it, and it is no good at all for a nut allergy.")]), py_tuple(vec![py_tuple(vec![py_str("coconut milk drink"), py_str("coconut drink"), py_str("coconut milk")]), py_str("The coconut drink in a carton has almost no protein - about 0.5g in 200ml - and around 40 calories, and it tastes of coconut, so it is for a smoothie or a curry rather than a cup of tea. The tin of coconut milk is a cooking ingredient with 20g of fat a serving, not a milk to drink.")]), py_tuple(vec![py_tuple(vec![py_str("rice milk"), py_str("rice drink"), py_str("rice")]), py_str("Rice milk is sweet without added sugar and has about 1g of protein in 200ml, the least of them. It is not the drink for a child under five, because of the arsenic rice takes up; a fortified soya or oat carton is what a toddler should have instead.")]), py_tuple(vec![py_tuple(vec![py_str("hemp milk"), py_str("hemp drink"), py_str("hemp")]), py_str("Hemp has about 3g of protein in 200ml, a nutty taste and it does not split in hot coffee. It is the one to take if soya and nut milks are both out.")]), py_tuple(vec![py_tuple(vec![py_str("pea milk"), py_str("pea protein milk"), py_str("pea")]), py_str("Pea milk has about 6g of protein in 200ml, close to soya, and it froths well. It tastes of pea on its own, so it is better in coffee than in a glass.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._THE_MILK_RANKING"); }
    s__THE_MILK_RANKING(py_str("For protein, soya or pea: about 6 to 7g in 200ml, against 1 to 2g for oat, almond and rice. For coffee and porridge, oat, because it is creamy and does not split. For the fewest calories, almond or the coconut drink, at about 40 to 50 a glass. Whichever you pick, take the unsweetened one and check the carton says calcium and B12 added: calcium, iodine and B12 are the three milk supplies and a plant carton does not unless it says so."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._MILK_AND_THE_PLANET"); }
    s__MILK_AND_THE_PLANET(py_str("Yes, on carbon every plant milk is far below dairy: a litre of oat or soya is about a third of dairy milk's emissions and a fraction of its land, rice is a little higher because paddy fields make methane, and almond is the lowest of them but the thirstiest, at several hundred litres of water a litre. So take oat or soya if it is water you are counting as well as carbon, and remember the carton matters much less than the swap: the milk nobody drinks is the worst of the lot."));
    fn __data13() -> Py { py_tuple(vec![py_str("environment"), py_str("environmental"), py_str("environmentally"), py_str("emissions"), py_str("carbon"), py_str("co2"), py_str("co 2"), py_str("planet"), py_str("climate"), py_str("sustainable"), py_str("sustainability"), py_str("land use"), py_str("water use"), py_str("footprint"), py_str("greenhouse"), py_str("methane"), py_str("eco friendly"), py_str("eco-friendly"), py_str("greener"), py_str("greenest"), py_str("global warming"), py_str("greenhouse gas"), py_str("how much water"), py_str("litres of water"), py_str("liters of water"), py_str("water footprint"), py_str("gallons of water")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._THE_PLANET"); }
    s__THE_PLANET(__data13());
    fn __data14() -> Py { py_tuple(vec![py_str("baby"), py_str("babies"), py_str("toddler"), py_str("toddlers"), py_str("weaning"), py_str("weaned"), py_str("wean"), py_str("breastfed"), py_str("formula"), py_str("one year old"), py_str("1 year old"), py_str("two year old"), py_str("2 year old"), py_str("under 5"), py_str("under five"), py_str("under two"), py_str("under 2"), py_str("my kid"), py_str("my child"), py_str("my son"), py_str("my daughter")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._MILK_FOR_A_CHILD"); }
    s__MILK_FOR_A_CHILD(__data14());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._COW_MILK_REPLY"); }
    s__COW_MILK_REPLY(py_str("Milk is good for you: a 200ml glass has about 7g of protein and a third of an adult's daily calcium, and it is one of the cheapest ways to get both. Semi-skimmed is the sensible default for most adults - the same protein and calcium as whole milk with about half the fat. Whole milk is the right one for a child under two, and a fortified plant milk is the swap if you do not do dairy."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._THE_FRIDGE_CHECK"); }
    s__THE_FRIDGE_CHECK(py_str("If the question was about the bottle in your fridge rather than about milk itself, smell it: sour or lumpy and it goes, whatever the date says."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_BARE_OK_ASK"); }
    s__A_BARE_OK_ASK(py_tuple(vec![py_str("ok"), py_str("okay"), py_str("alright"), py_str("fine"), py_str("safe"), py_str("any good"), py_str("worth it")]));
    fn __data15() -> Py { py_tuple(vec![py_str("good"), py_str("healthy"), py_str("fattening"), py_str("bad"), py_str("calcium"), py_str("protein"), py_str("vitamin"), py_str("vitamins"), py_str("nutrient"), py_str("nutrients"), py_str("diet"), py_str("fatten"), py_str("health"), py_str("benefit"), py_str("benefits"), py_str("need"), py_str("better"), py_str("worse"), py_str("calorie"), py_str("calories"), py_str("strong"), py_str("stronger")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._NOURISHING"); }
    s__NOURISHING(__data15());
    fn __data16() -> Py { py_tuple(vec![py_str("which milk"), py_str("which is best"), py_str("what milk"), py_str("best milk"), py_str("best alternative"), py_str("plant milk"), py_str("plant milks"), py_str("plant based"), py_str("plant-based"), py_str("dairy alternative"), py_str("dairy alternatives"), py_str("milk alternative"), py_str("milk alternatives"), py_str("non dairy"), py_str("non-dairy"), py_str("instead of milk"), py_str("replace milk"), py_str("switch from milk"), py_str("switch to"), py_str("milk is best"), py_str("healthiest milk")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._WHICH_MILK"); }
    s__WHICH_MILK(__data16());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_THERMOMETER"); }
    s__A_THERMOMETER(stdlib_call("re", "compile", &[py_str("\\b(?:temperature|temperatures|temp|temps|thermometer|internal|oven|gas mark|how long|minutes|mins|done|doneness|medium|rare|medium-rare|well done)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data17() -> Py { py_tuple(vec![py_str("lemonade"), py_str("squash"), py_str("cordial"), py_str("irn bru"), py_str("lucozade"), py_str("sprite"), py_str("fanta"), py_str("7up"), py_str("cherryade"), py_str("tonic water"), py_str("ginger beer"), py_str("cream soda"), py_str("soda"), py_str("pop"), py_str("fizzy drink"), py_str("soft drink")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._SWEET_DRINKS"); }
    s__SWEET_DRINKS(__data17());
    fn __data19() -> Py { py_tuple(vec![py_str("sugar"), py_str("healthy"), py_str("good for"), py_str("how much"), py_str("calorie"), py_str("calories"), py_str("can i drink"), py_str("should i drink"), py_str("bad for"), py_str("ok to drink"), py_str("okay to drink"), py_str("as bad as"), py_str("better than"), py_str("worse than")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("coke"), py_str("cola"), py_str("coca cola"), py_str("pepsi"), py_str("lemonade"), py_str("squash"), py_str("fizzy drink"), py_str("fizzy drinks"), py_str("soft drink"), py_str("soft drinks"), py_str("soda"), py_str("pop"), py_str("cordial"), py_str("irn bru"), py_str("lucozade"), py_str("sprite"), py_str("fanta"), py_str("7up"), py_str("tonic water"), py_str("ginger beer"), py_str("cream soda"), py_str("cherryade")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("bad for"), py_str("good for"), py_str("healthy"), py_str("unhealthy"), py_str("is it ok"), py_str("is it okay"), py_str("ok to drink"), py_str("okay to drink"), py_str("how bad"), py_str("should i drink"), py_str("can i drink"), py_str("is it bad"), py_str("worth drinking"), py_str("better than")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("diet"), py_str("zero"), py_str("sugar free"), py_str("sugar-free"), py_str("sweetener"), py_str("sweeteners"), py_str("aspartame"), py_str("how much sugar in"), py_str("how many calories in"), py_str("calories in"), py_str("juice"), py_str("smoothie"), py_str("energy drink"), py_str("sports drink"), py_str("recipe"), py_str("cocktail"), py_str("mixer"), py_str("spill"), py_str("spilled"), py_str("stain"), py_str("carpet"), py_str("sofa"), py_str("fridge"), py_str("freeze"), py_str("ice cube"), py_str("bottle"), py_str("can i keep"), py_str("goes flat"), py_str("out of date"), py_str("fizzy water"), py_str("sparkling water"), py_str("carbonated"), py_str("soda water"), py_str("club soda")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("bad for"), py_str("good for"), py_str("healthy"), py_str("unhealthy"), py_str("safe"), py_str("is it ok"), py_str("is it okay"), py_str("ok to drink"), py_str("teeth"), py_str("instead of"), py_str("count"), py_str("hydrate"), py_str("drink"), py_str("day")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("flavoured"), py_str("flavored"), py_str("sweetener"), py_str("sugar free"), py_str("sugar-free"), py_str("diet"), py_str("lemonade"), py_str("coke"), py_str("cola"), py_str("squash"), py_str("goes flat"), py_str("gone flat"), py_str("went flat"), py_str("machine"), py_str("soda stream"), py_str("sodastream"), py_str("canister"), py_str("bottle of gas"), py_str("make my own"), py_str("carbonate at home")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("bad"), py_str("good"), py_str("healthy"), py_str("unhealthy"), py_str("calorie"), py_str("calories"), py_str("teeth"), py_str("tooth"), py_str("hurt"), py_str("stomach"), py_str("swallow"), py_str("eat"), py_str("day"), py_str("much"), py_str("any")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("nicotine"), py_str("quit"), py_str("quitting"), py_str("stop smoking"), py_str("smoking"), py_str("smoke"), py_str("vape"), py_str("vaping"), py_str("patch"), py_str("patches"), py_str("teething"), py_str("baby"), py_str("toddler"), py_str("sore"), py_str("bleed"), py_str("bleeding"), py_str("gums are"), py_str("my gums"), py_str("gum is"), py_str("gum has"), py_str("gum feels"), py_str("gum hurts"), py_str("gum looks"), py_str("my gum"), py_str("the gum"), py_str("gum under"), py_str("gum line"), py_str("gumline"), py_str("gum disease"), py_str("gum infection"), py_str("swollen"), py_str("receding"), py_str("dentist"), py_str("hair"), py_str("carpet"), py_str("jeans"), py_str("clothes"), py_str("shoe"), py_str("floor"), py_str("table"), py_str("stuck"), py_str("off my"), py_str("out of")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("stop"), py_str("cut"), py_str("cutting"), py_str("reduce"), py_str("reducing"), py_str("lower"), py_str("give up"), py_str("quit"), py_str("less"), py_str("crave"), py_str("cravings"), py_str("addicted"), py_str("addiction"), py_str("at night"), py_str("evening"), py_str("after dinner"), py_str("cant have much"), py_str("cant eat much"), py_str("cannot have much"), py_str("cannot eat much"), py_str("shouldnt have"), py_str("shouldnt eat")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("stick"), py_str("sticks"), py_str("sticking"), py_str("stuck"), py_str("tin"), py_str("tins"), py_str("tray"), py_str("trays"), py_str("grease"), py_str("greased"), py_str("parchment"), py_str("springform"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("oven"), py_str("flour"), py_str("dough"), py_str("batter"), py_str("pan"), py_str("pans"), py_str("paper"), py_str("burn"), py_str("burns"), py_str("burnt"), py_str("burned"), py_str("burning"), py_str("no sugar in my tea"), py_str("no sugar in my coffee"), py_str("no sugar in my cup"), py_str("no sugar for me"), py_str("no sugar thanks"), py_str("without sugar in my tea"), py_str("without sugar in my coffee"), py_str("just milk thanks"), py_str("stale"), py_str("going stale"), py_str("go stale"), py_str("goes stale"), py_str("gone off"), py_str("keep fresh"), py_str("keeps fresh"), py_str("keep it fresh"), py_str("crumbly"), py_str("crumbles"), py_str("crumb"), py_str("crumbs"), py_str("crumbling"), py_str("falls apart"), py_str("falling apart"), py_str("soggy"), py_str("sank"), py_str("sunken"), py_str("didnt rise"), py_str("risen"), py_str("dry"), py_str("drying"), py_str("dried"), py_str("sink"), py_str("sinks"), py_str("sinking"), py_str("sunk"), py_str("flat"), py_str("recipe"), py_str("recipes"), py_str("icing"), py_str("frosting"), py_str("sponge"), py_str("sponges"), py_str("loaf"), py_str("why"), py_str("cause"), py_str("causes"), py_str("what causes")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("bad for me"), py_str("bad for you"), py_str("bad for us"), py_str("bad"), py_str("healthy"), py_str("healthier"), py_str("healthiest"), py_str("unhealthy"), py_str("poison"), py_str("poisonous"), py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("addictive"), py_str("addicted"), py_str("addiction"), py_str("cause diabetes"), py_str("causes diabetes"), py_str("diabetes"), py_str("worse"), py_str("really"), py_str("truth"), py_str("is it"), py_str("is sugar"), py_str("are sugars")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("cut down"), py_str("cut out"), py_str("cutting"), py_str("give up"), py_str("quit"), py_str("stop eating"), py_str("crave"), py_str("craves"), py_str("craving"), py_str("cravings"), py_str("swap"), py_str("swaps"), py_str("instead"), py_str("replace"), py_str("how much sugar in"), py_str("in a can"), py_str("in coke"), py_str("in a coke"), py_str("in a glass"), py_str("teaspoons"), py_str("what is sugaring"), py_str("sugaring better"), py_str("sugaring or"), py_str("leg sugaring"), py_str("body sugaring"), py_str("sugaring hair"), py_str("wax"), py_str("waxing"), py_str("shave"), py_str("shaving"), py_str("ingrown"), py_str("scrub"), py_str("hair"), py_str("skin"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("dog"), py_str("cat"), py_str("bake"), py_str("baking"), py_str("cake"), py_str("icing"), py_str("meringue"), py_str("caramel"), py_str("toffee"), py_str("substitute for sugar"), py_str("replacement for sugar")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("snacking"), py_str("snack"), py_str("snacks"), py_str("eat at night"), py_str("eating at night"), py_str("after dinner"), py_str("snacking late at night"), py_str("eat late at night"), py_str("eating late at night"), py_str("hungry late at night"), py_str("hungry at midnight"), py_str("snack at midnight"), py_str("crisps at midnight"), py_str("eat at midnight"), py_str("eating at midnight"), py_str("snacking at midnight"), py_str("hungry at midnight"), py_str("in the evening snack"), py_str("snacking in the evening"), py_str("snack in the evening"), py_str("eat in the evening"), py_str("eating in the evening"), py_str("evening snacking"), py_str("eat too much in the evening"), py_str("eat so much in the evening"), py_str("sweets in the evening"), py_str("crisps in the evening"), py_str("biscuits in the evening"), py_str("hungry in the evening")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("being sick"), py_str("feel sick"), py_str("feels sick"), py_str("feeling sick"), py_str("sick after"), py_str("throws up"), py_str("throwing up"), py_str("threw up"), py_str("throw up"), py_str("vomit"), py_str("vomits"), py_str("vomiting"), py_str("nausea"), py_str("nauseous"), py_str("diarrhoea"), py_str("diarrhea"), py_str("unwell"), py_str("stomach bug"), py_str("food poisoning"), py_str("indigestion")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("more"), py_str("eat"), py_str("how do i"), py_str("get"), py_str("fussy"), py_str("hate"), py_str("kids"), py_str("five a day"), py_str("portions"), py_str("refuses"), py_str("refuse"), py_str("wont eat"), py_str("picky"), py_str("wont touch"), py_str("won't touch"), py_str("wont try"), py_str("won't try"), py_str("wont eat any"), py_str("turns his nose"), py_str("turns her nose"), py_str("turns their nose"), py_str("hates veg"), py_str("hates greens"), py_str("hates vegetables"), py_str("doesnt like veg"), py_str("doesn't like veg"), py_str("off his food"), py_str("off her food"), py_str("faddy"), py_str("picky eater")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("dressing"), py_str("vinaigrette"), py_str("mayonnaise"), py_str("wash"), py_str("recipe"), py_str("with dinner"), py_str("side salad"), py_str("what goes with"), py_str("slimy"), py_str("limp"), py_str("wilted"), py_str("wilting"), py_str("last"), py_str("keep a bag"), py_str("goes off"), py_str("gone off"), py_str("fridge"), py_str("freezer"), py_str("store"), py_str("throwing"), py_str("throw away"), py_str("waste"), py_str("binned"), py_str("bin"), py_str("unused"), py_str("left over"), py_str("leftover"), py_str("doesnt get used"), py_str("goes to waste"), py_str("lawn"), py_str("lawns"), py_str("grass"), py_str("mow"), py_str("mowing"), py_str("mowed"), py_str("turf"), py_str("greener"), py_str("green again"), py_str("brown patches"), py_str("for dinner"), py_str("for tea"), py_str("for supper"), py_str("for lunch"), py_str("for tea tonight"), py_str("quick dinner"), py_str("quick tea"), py_str("dinner idea"), py_str("dinner ideas"), py_str("one of them"), py_str("one of the kids"), py_str("one of my kids"), py_str("one of the children"), py_str("one of ours"), py_str("one of mine"), py_str("tea idea"), py_str("tea ideas"), py_str("meal idea"), py_str("meal ideas"), py_str("what can i cook"), py_str("what should i cook"), py_str("what should i make"), py_str("meal plan")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("more"), py_str("fussy"), py_str("hate"), py_str("kids"), py_str("kid"), py_str("children"), py_str("five a day"), py_str("portions"), py_str("wont eat"), py_str("sparingly"), py_str("eet"), py_str("vegtables"), py_str("how do i get"), py_str("refuse"), py_str("refuses"), py_str("refused"), py_str("refusing"), py_str("wont touch"), py_str("won't touch"), py_str("wont try"), py_str("won't try"), py_str("turns his nose"), py_str("turns her nose"), py_str("turns their nose"), py_str("turns up his nose"), py_str("turns up her nose"), py_str("doesnt like veg"), py_str("doesn't like veg"), py_str("off his food"), py_str("off her food"), py_str("picky eater"), py_str("faddy"), py_str("enough"), py_str("barely"), py_str("hardly"), py_str("not eating"), py_str("dont eat"), py_str("don't eat"), py_str("should eat"), py_str("ought to eat"), py_str("need to eat")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("balanced meal"), py_str("balanced diet"), py_str("healthy diet"), py_str("eat healthily"), py_str("healthy eating"), py_str("eat better"), py_str("eat healthier"), py_str("eating better"), py_str("eating healthier"), py_str("eat more healthily"), py_str("improve my diet"), py_str("eat properly"), py_str("eat well"), py_str("clean up my diet"), py_str("better diet")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("what"), py_str("how"), py_str("should i"), py_str("look like"), py_str("is"), py_str("start"), py_str("way"), py_str("tips"), py_str("advice"), py_str("help"), py_str("trying"), py_str("trying to"), py_str("trying out"), py_str("want"), py_str("want to"), py_str("wanted to"), py_str("need to"), py_str("trying hard"), py_str("attempting")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("hate cooking"), py_str("hate to cook"), py_str("dont cook"), py_str("don't cook"), py_str("cant cook"), py_str("can't cook"), py_str("no cooking"), py_str("without cooking"), py_str("dont like cooking"), py_str("don't like cooking"), py_str("no time to cook"), py_str("hate the kitchen"), py_str("cant be bothered")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("hate cooking"), py_str("hate to cook"), py_str("dont cook"), py_str("don't cook"), py_str("cant cook"), py_str("can't cook"), py_str("no cooking"), py_str("without cooking"), py_str("dont like cooking"), py_str("don't like cooking"), py_str("no time to cook"), py_str("no time"), py_str("cant be bothered"), py_str("hate the kitchen"), py_str("cant use an oven"), py_str("only a kettle")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how long"), py_str("cook"), py_str("boil"), py_str("recipe"), py_str("for six"), py_str("for four"), py_str("for two"), py_str("per person"), py_str("leftover"), py_str("freeze"), py_str("reheat"), py_str("brown rice"), py_str("white rice"), py_str("wholegrain"), py_str("wholewheat"), py_str("wholemeal"), py_str("brown pasta"), py_str("instead of butter"), py_str("instead of oil"), py_str("instead of margarine"), py_str("instead of lard"), py_str("instead of suet"), py_str("flour"), py_str("flours")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("brown rice"), py_str("white rice"), py_str("wholegrain rice"), py_str("brown pasta"), py_str("wholewheat pasta"), py_str("wholemeal pasta"), py_str("brown or white rice"), py_str("white or brown rice"), py_str("brown or white pasta"), py_str("brown or white"), py_str("white or brown"), py_str("wholemeal or white"), py_str("white or wholemeal"), py_str("brown"), py_str("white")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("better"), py_str("healthier"), py_str("which"), py_str("than"), py_str("difference"), py_str("instead"), py_str("worth"), py_str("or white"), py_str("or brown"), py_str("around"), py_str("should"), py_str("eat"), py_str("swap"), py_str("switch")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("bread"), py_str("flour"), py_str("sugar"), py_str("sauce"), py_str("onion"), py_str("stock"), py_str("meat"), py_str("mince"), py_str("pudding"), py_str("rice pudding"), py_str("tea"), py_str("coffee"), py_str("wine"), py_str("basmati"), py_str("risotto"), py_str("how long"), py_str("how much water"), py_str("cook"), py_str("boil"), py_str("wash"), py_str("laundry"), py_str("detergent"), py_str("bleach"), py_str("washing machine"), py_str("tumble dryer"), py_str("degrees")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("frozen"), py_str("freezer"), py_str("tinned"), py_str("tin"), py_str("canned"), py_str("can of"), py_str("from a tin"), py_str("frozen peas"), py_str("frozen veg"), py_str("frozen vegetables"), py_str("frozen fruit"), py_str("frozen fish"), py_str("tinned tomatoes"), py_str("tinned beans"), py_str("tinned fruit"), py_str("canned tomatoes")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("as good as"), py_str("as healthy"), py_str("as nutritious"), py_str("less nutritious"), py_str("more nutritious"), py_str("better than"), py_str("worse than"), py_str("healthy"), py_str("healthy as"), py_str("nutritious"), py_str("as fresh"), py_str("than fresh"), py_str("just as good"), py_str("vitamins"), py_str("fresh or frozen"), py_str("frozen or fresh"), py_str("tinned or fresh"), py_str("fresh or tinned"), py_str("worth it"), py_str("good for you"), py_str("cheaper"), py_str("cheapest"), py_str("cost"), py_str("costs"), py_str("price"), py_str("budget"), py_str("smoothie"), py_str("smoothies")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("how many"), py_str("how much"), py_str("portion"), py_str("portions"), py_str("recipe"), py_str("cook"), py_str("boil"), py_str("blanch"), py_str("defrost"), py_str("thaw"), py_str("reheat"), py_str("freezer broke"), py_str("freezer is"), py_str("freezer not"), py_str("power cut"), py_str("fridge broke"), py_str("still frozen"), py_str("left out"), py_str("went off"), py_str("off food"), py_str("refreeze"), py_str("how long"), py_str("keeps"), py_str("lasting"), py_str("covering"), py_str("wedding"), py_str("cake"), py_str("tuna"), py_str("salmon"), py_str("sardines"), py_str("sardine"), py_str("mackerel"), py_str("oily fish"), py_str("omega"), py_str("fish healthy"), py_str("fish good for"), py_str("fish in a tin"), py_str("tinned fish"), py_str("canned fish"), py_str("protein"), py_str("calories"), py_str("pizza"), py_str("pizzas"), py_str("chips"), py_str("oven chips"), py_str("ready meal"), py_str("ready meals"), py_str("nugget"), py_str("nuggets"), py_str("fish fingers"), py_str("fish finger"), py_str("ice cream"), py_str("pastry"), py_str("pie"), py_str("pies"), py_str("burger"), py_str("burgers"), py_str("takeaway"), py_str("takeaways"), py_str("sausage roll"), py_str("sausage rolls"), py_str("soup")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("cholesterol"), py_str("heart"), py_str("bad for"), py_str("safe"), py_str("good for"), py_str("healthy"), py_str("can i eat"), py_str("should i"), py_str("a day"), py_str("a week"), py_str("daily"), py_str("ok to eat"), py_str("okay to eat"), py_str("fine to eat"), py_str("every day"), py_str("everyday"), py_str("each day"), py_str("how often"), py_str("too many")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("crack"), py_str("cracked"), py_str("cracks"), py_str("shell"), py_str("shells"), py_str("fridge"), py_str("raw egg"), py_str("raw eggs"), py_str("raw"), py_str("how many minutes"), py_str("how many mins"), py_str("how long"), py_str("minutes"), py_str("minute"), py_str("timer"), py_str("raw"), py_str("runny"), py_str("undercooked"), py_str("pregnant"), py_str("pregnancy"), py_str("lion"), py_str("past the date"), py_str("past its date"), py_str("past their date"), py_str("past the date on"), py_str("out of date"), py_str("gone off"), py_str("use by"), py_str("best before"), py_str("float"), py_str("floats")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("protein in"), py_str("grams of protein"), py_str("grammes of protein"), py_str("how much protein"), py_str("carbs in"), py_str("how many carbs"), py_str("how much fat"), py_str("fat in"), py_str("how much sugar"), py_str("sugar in"), py_str("fibre in"), py_str("how much fibre"), py_str("serve"), py_str("serving"), py_str("served"), py_str("go with"), py_str("goes with"), py_str("going with"), py_str("side"), py_str("sides"), py_str("side dish"), py_str("what to have with"), py_str("have with it"), py_str("pink"), py_str("raw"), py_str("undercooked"), py_str("bloody"), py_str("still ok"), py_str("ok to eat"), py_str("safe to eat"), py_str("season"), py_str("seasoning"), py_str("seasoned"), py_str("spice"), py_str("spices"), py_str("spiced"), py_str("herb"), py_str("herbs"), py_str("rub"), py_str("rubbed"), py_str("marinade"), py_str("marinate"), py_str("flavour"), py_str("flavor"), py_str("what should i make"), py_str("what can i make"), py_str("what should i cook"), py_str("what can i cook"), py_str("make for tea"), py_str("make for dinner"), py_str("make for lunch"), py_str("make with"), py_str("meal with"), py_str("dinner with"), py_str("tea with"), py_str("recipe"), py_str("swap for"), py_str("swaps for"), py_str("swapped"), py_str("substitute"), py_str("instead of mince"), py_str("i've got"), py_str("i have got"), py_str("ive got"), py_str("mince pie"), py_str("mince pies"), py_str("mincemeat"), py_str("a kilo"), py_str("per kilo"), py_str("per kg"), py_str("price"), py_str("prices"), py_str("priced"), py_str("cost"), py_str("costs"), py_str("cheaper"), py_str("cheapest"), py_str("do i need"), py_str("will i need"), py_str("should i buy"), py_str("shall i buy"), py_str("need to buy"), py_str("per person"), py_str("per head"), py_str("grams"), py_str("grammes"), py_str("kg"), py_str("kilos"), py_str("how much mince"), py_str("how much beef"), py_str("how much lamb"), py_str("how much pork")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("nutrition label"), py_str("nutrition labels"), py_str("food label"), py_str("food labels"), py_str("nutrition"), py_str("nutritional information"), py_str("traffic light"), py_str("traffic lights"), py_str("ingredients"), py_str("ingredients list"), py_str("packet"), py_str("packaging"), py_str("back of the packet"), py_str("label")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("read"), py_str("reading"), py_str("understand"), py_str("mean"), py_str("means"), py_str("high"), py_str("low"), py_str("check"), py_str("how do i read"), py_str("what does"), py_str("explain"), py_str("compare"), py_str("per 100g"), py_str("per 100 g"), py_str("serving"), py_str("servings"), py_str("portion")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("clothes"), py_str("shirt"), py_str("jumper"), py_str("trousers"), py_str("garment"), py_str("washing"), py_str("wash"), py_str("iron"), py_str("ironing"), py_str("care label"), py_str("parcel"), py_str("address"), py_str("price"), py_str("jar"), py_str("tin"), py_str("can"), py_str("freezer"), py_str("wine"), py_str("beer")]) }
    fn __data53() -> Py { py_tuple(vec![py_str("too much"), py_str("how much"), py_str("limit"), py_str("healthy"), py_str("reduce"), py_str("cut"), py_str("how many"), py_str("a day"), py_str("teaspoons"), py_str("bad for you"), py_str("bad for me"), py_str("good for you"), py_str("unhealthy"), py_str("is it bad"), py_str("is salt bad"), py_str("less"), py_str("lower"), py_str("cut down"), py_str("cut back"), py_str("cant eat much"), py_str("cant have much"), py_str("cannot eat much"), py_str("not much"), py_str("shouldnt eat"), py_str("should not eat"), py_str("shouldnt have"), py_str("has to eat less"), py_str("watch the salt"), py_str("no salt"), py_str("avoid")]) }
    fn __data54() -> Py { py_tuple(vec![py_str("how much"), py_str("a day"), py_str("need"), py_str("get more"), py_str("constipated"), py_str("healthy"), py_str("enough"), py_str("barely"), py_str("hardly"), py_str("get"), py_str("should eat"), py_str("dont eat"), py_str("don't eat"), py_str("eat"), py_str("eating"), py_str("food"), py_str("foods"), py_str("more"), py_str("increase"), py_str("best"), py_str("good sources"), py_str("sources"), py_str("what"), py_str("diet"), py_str("in my diet"), py_str("good"), py_str("good for you"), py_str("benefits"), py_str("help")]) }
    fn __data55() -> Py { py_tuple(vec![py_str("more"), py_str("enough"), py_str("get"), py_str("increase"), py_str("sources"), py_str("best"), py_str("good"), py_str("what"), py_str("eat"), py_str("eating"), py_str("food"), py_str("foods"), py_str("need"), py_str("should eat"), py_str("dont eat"), py_str("don't eat"), py_str("in my diet"), py_str("add")]) }
    fn __data56() -> Py { py_tuple(vec![py_str("shake"), py_str("powder"), py_str("whey"), py_str("creatine"), py_str("supplement"), py_str("supplements"), py_str("steroid"), py_str("bulk"), py_str("bulking"), py_str("bodybuilder"), py_str("how much protein in"), py_str("in an egg"), py_str("in chicken"), py_str("how much")]) }
    fn __data57() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("a day"), py_str("per day"), py_str("daily"), py_str("mg"), py_str("micrograms"), py_str("supplement"), py_str("supplements"), py_str("tablet"), py_str("tablets"), py_str("dose"), py_str("dosage"), py_str("overdose"), py_str("too much"), py_str("blood test"), py_str("shirt"), py_str("shirts"), py_str("crease"), py_str("creases"), py_str("steam"), py_str("clothes"), py_str("trousers"), py_str("board"), py_str("rust"), py_str("pan"), py_str("pans"), py_str("soleplate")]) }
    fn __data58() -> Py { py_tuple(vec![py_str("low in iron"), py_str("iron low"), py_str("iron levels"), py_str("iron level"), py_str("iron deficiency"), py_str("deficient in iron"), py_str("anaemic"), py_str("anaemia"), py_str("anemia"), py_str("anemic"), py_str("need iron"), py_str("lack of iron"), py_str("iron tablets"), py_str("iron supplement")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("worth"), py_str("should i"), py_str("think"), py_str("wonder"), py_str("worried"), py_str("always tired"), py_str("tired all"), py_str("pale"), py_str("breathless"), py_str("my"), py_str("i"), py_str("am"), py_str("need"), py_str("take"), py_str("start"), py_str("check"), py_str("test"), py_str("blood"), py_str("doctor"), py_str("gp"), py_str("levels")]) }
    fn __data60() -> Py { py_tuple(vec![py_str("shirt"), py_str("shirts"), py_str("crease"), py_str("creases"), py_str("steam"), py_str("clothes"), py_str("trousers"), py_str("board"), py_str("rust"), py_str("pan"), py_str("pans"), py_str("soleplate"), py_str("iron in"), py_str("in spinach"), py_str("in cereal"), py_str("in food"), py_str("in beef"), py_str("in liver"), py_str("in lentils"), py_str("supplement dose"), py_str("with tea"), py_str("with coffee"), py_str("with milk"), py_str("with food"), py_str("with water"), py_str("with orange juice"), py_str("with breakfast"), py_str("take with"), py_str("take it with"), py_str("antacid"), py_str("antacids"), py_str("gaviscon"), py_str("rennie"), py_str("omeprazole"), py_str("lansoprazole")]) }
    fn __data61() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("a day"), py_str("per day"), py_str("daily"), py_str("need"), py_str("needed"), py_str("should i take"), py_str("should i have"), py_str("should i be"), py_str("recommended"), py_str("mg"), py_str("micrograms"), py_str("enough"), py_str("do i need"), py_str("much of")]) }
    fn __data62() -> Py { py_tuple(vec![py_str("is in"), py_str("are in"), py_str("there in"), py_str("a test"), py_str("blood test"), py_str("test cost"), py_str("cost"), py_str("costs"), py_str("price"), py_str("cheapest"), py_str("how much calcium in"), py_str("how much iron in"), py_str("how much protein in"), py_str("how much vitamin d in"), py_str("how much vitamin c in"), py_str("how much vitamin b12 in"), py_str("how much b12 in"), py_str("how much magnesium in"), py_str("how much potassium in"), py_str("how much zinc in"), py_str("how much folate in"), py_str("supplement"), py_str("supplements"), py_str("multivitamin"), py_str("shirt"), py_str("shirts"), py_str("crease"), py_str("creases"), py_str("creased"), py_str("steam"), py_str("steamer"), py_str("clothes"), py_str("trousers"), py_str("linen"), py_str("fabric"), py_str("board"), py_str("rust"), py_str("pan"), py_str("pans"), py_str("soleplate"), py_str("water tank"), py_str("limescale"), py_str("supplement dose"), py_str("deficiency"), py_str("deficient"), py_str("anaemic"), py_str("anaemia"), py_str("low iron"), py_str("salt"), py_str("sodium"), py_str("sugar"), py_str("water"), py_str("calories"), py_str("calorie")]) }
    fn __data63() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("a day"), py_str("per day"), py_str("daily"), py_str("mg"), py_str("micrograms"), py_str("supplement"), py_str("supplements"), py_str("multivitamin"), py_str("tablet"), py_str("tablets"), py_str("dose"), py_str("dosage"), py_str("blood test"), py_str("deficiency"), py_str("deficient"), py_str("foods"), py_str("high in"), py_str("rich in"), py_str("sources"), py_str("which foods"), py_str("what foods"), py_str("shirt"), py_str("shirts"), py_str("crease"), py_str("creases"), py_str("creased"), py_str("steam"), py_str("steamer"), py_str("clothes"), py_str("trousers"), py_str("linen"), py_str("fabric"), py_str("board"), py_str("rust"), py_str("pan"), py_str("pans"), py_str("soleplate"), py_str("water tank"), py_str("limescale"), py_str("golf"), py_str("gym"), py_str("weights"), py_str("heat"), py_str("hot"), py_str("cold"), py_str("plug"), py_str("fuse"), py_str("sparks"), py_str("spitting"), py_str("spray"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("breast feeding"), py_str("trying for a baby"), py_str("baby"), py_str("babies"), py_str("child"), py_str("children"), py_str("toddler"), py_str("kids")]) }
    fn __data64() -> Py { py_tuple(vec![py_str("how much should i weigh"), py_str("healthy weight"), py_str("ideal weight"), py_str("my weight"), py_str("bmi"), py_str("overweight"), py_str("underweight"), py_str("obese"), py_str("what weight should i be"), py_str("what weight should i aim"), py_str("what weight should i target"), py_str("what should i weigh"), py_str("what should my weight be"), py_str("what weight is healthy"), py_str("goal weight"), py_str("target weight"), py_str("how heavy should i be"), py_str("what weight do i need to be")]) }
    fn __data65() -> Py { py_tuple(vec![py_str("weigh up"), py_str("weighing up"), py_str("weigh it up"), py_str("weigh them up"), py_str("weighing it up"), py_str("weigh the pros"), py_str("weigh the options"), py_str("lift"), py_str("lifting"), py_str("lifted"), py_str("bench"), py_str("squat"), py_str("squats"), py_str("deadlift"), py_str("dumbbell"), py_str("dumbbells"), py_str("barbell"), py_str("kettlebell"), py_str("reps"), py_str("sets"), py_str("work out"), py_str("calculate"), py_str("calculation"), py_str("formula")]) }
    fn __data66() -> Py { py_tuple(vec![py_str("need"), py_str("should"), py_str("worth"), py_str("do i"), py_str("have to"), py_str("how"), py_str("any point"), py_str("need to"), py_str("does"), py_str("did"), py_str("work"), py_str("works"), py_str("any good"), py_str("help"), py_str("helpful"), py_str("necessary"), py_str("bother")]) }
    fn __data67() -> Py { py_tuple(vec![py_str("eat"), py_str("eating"), py_str("food"), py_str("meal"), py_str("breakfast"), py_str("lunch"), py_str("dinner"), py_str("carbs"), py_str("carbohydrate"), py_str("protein"), py_str("fuel"), py_str("before or after"), py_str("before and after"), py_str("eat first or"), py_str("which is better")]) }
    fn __data68() -> Py { py_tuple(vec![py_str("stretch"), py_str("stretching"), py_str("stretches"), py_str("warm up"), py_str("warmup"), py_str("cool down"), py_str("mobility"), py_str("foam roll"), py_str("before bed"), py_str("in the morning"), py_str("ibuprofen"), py_str("paracetamol"), py_str("aspirin"), py_str("naproxen"), py_str("painkiller"), py_str("painkillers"), py_str("tablet"), py_str("tablets"), py_str("medicine"), py_str("medicines"), py_str("medication"), py_str("antibiotic"), py_str("antibiotics"), py_str("steroid"), py_str("steroids"), py_str("my train"), py_str("the train"), py_str("a train"), py_str("train home"), py_str("train journey"), py_str("flight"), py_str("plane"), py_str("ferry")]) }
    fn __data69() -> Py { py_tuple(vec![py_str("stressed"), py_str("sad"), py_str("bored"), py_str("upset"), py_str("lonely"), py_str("tired"), py_str("anxious"), py_str("angry"), py_str("why"), py_str("stop"), py_str("help"), py_str("how"), py_str("when")]) }
    fn __data70() -> Py { py_tuple(vec![py_str("guilty"), py_str("guilt"), py_str("ashamed"), py_str("hate myself"), py_str("bingeing"), py_str("binge"), py_str("purge"), py_str("made myself sick"), py_str("overate"), py_str("guilty about food"), py_str("guilty about eating"), py_str("guilt about food"), py_str("guilty after eating"), py_str("guilty for eating"), py_str("guilty about what i eat"), py_str("food guilt"), py_str("guilty about my weight")]) }
    fn __data71() -> Py { py_tuple(vec![py_str("say no"), py_str("saying no"), py_str("refuse"), py_str("refusing"), py_str("turn down"), py_str("turning down"), py_str("decline"), py_str("declining"), py_str("radiator"), py_str("radiators"), py_str("boiler"), py_str("boilers"), py_str("pipe"), py_str("pipes"), py_str("piping"), py_str("plumbing"), py_str("plumber"), py_str("heating"), py_str("central heating"), py_str("heat pump"), py_str("valve"), py_str("valves"), py_str("thermostat"), py_str("sump"), py_str("engine"), py_str("coolant"), py_str("brake"), py_str("brakes"), py_str("hydraulic"), py_str("airlock"), py_str("drain"), py_str("drains"), py_str("water system"), py_str("cooling system"), py_str("immersion heater"), py_str("hot water"), py_str("underfloor"), py_str("flush"), py_str("flushing"), py_str("email"), py_str("emails"), py_str("inbox"), py_str("mailbox"), py_str("file"), py_str("files"), py_str("data"), py_str("cache"), py_str("computer"), py_str("laptop"), py_str("phone"), py_str("photo"), py_str("photos"), py_str("folder"), py_str("folders"), py_str("hard drive"), py_str("facebook"), py_str("instagram"), py_str("twitter"), py_str("social media"), py_str("loft"), py_str("garage"), py_str("cupboard"), py_str("wardrobe"), py_str("shed"), py_str("garden"), py_str("house"), py_str("server"), py_str("spreadsheet"), py_str("contacts"), py_str("downloads"), py_str("windows")]) }
    fn __data72() -> Py { py_tuple(vec![py_str("battery"), py_str("batteries"), py_str("phone"), py_str("laptop"), py_str("charger"), py_str("charging"), py_str("charge"), py_str("how fast"), py_str("how fast can"), py_str("how fast should"), py_str("how fast do"), py_str("how fast will"), py_str("fastest"), py_str("mph"), py_str("speed"), py_str("pace")]) }
    fn __data73() -> Py { py_tuple(vec![py_str("date"), py_str("dates"), py_str("out of date"), py_str("past its date"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("gone off"), py_str("mouldy"), py_str("mould"), py_str("fridge"), py_str("freeze"), py_str("tomorrow")]) }
    fn __data74() -> Py { py_tuple(vec![py_str("spice"), py_str("spices"), py_str("spicy"), py_str("herb"), py_str("herbs"), py_str("seasoning"), py_str("pepper"), py_str("black pepper"), py_str("chilli"), py_str("chili"), py_str("cumin"), py_str("cinnamon"), py_str("turmeric"), py_str("garlic"), py_str("ginger"), py_str("paprika"), py_str("curry powder"), py_str("mixed herbs")]) }
    fn __data75() -> Py { py_tuple(vec![py_str("bad for you"), py_str("bad for me"), py_str("good for you"), py_str("good for me"), py_str("healthy"), py_str("unhealthy"), py_str("fattening"), py_str("ok"), py_str("okay"), py_str("safe"), py_str("fine"), py_str("harm"), py_str("worth"), py_str("a day"), py_str("every day"), py_str("too much"), py_str("put in"), py_str("put cinnamon"), py_str("in my coffee"), py_str("in coffee"), py_str("in my tea"), py_str("in tea"), py_str("in cooking"), py_str("add to"), py_str("sprinkle"), py_str("put"), py_str("add"), py_str("use"), py_str("go in"), py_str("goes in"), py_str("how much"), py_str("in a chilli"), py_str("in a curry"), py_str("in a stew"), py_str("in a sauce"), py_str("in a soup"), py_str("in soup"), py_str("in the chilli"), py_str("in the curry")]) }
    fn __data76() -> Py { py_tuple(vec![py_str("baby"), py_str("babies"), py_str("baby led"), py_str("weaning"), py_str("wean"), py_str("puree"), py_str("months old"), py_str("month old"), py_str("toddler"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("pregnant"), py_str("con carne"), py_str("tikka masala"), py_str("bolognese"), py_str("stir fry"), py_str("cottage pie"), py_str("fish pie"), py_str("lentil soup"), py_str("chicken salad"), py_str("chicken sandwich")]) }
    fn __data77() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("gone off"), py_str("going off"), py_str("off"), py_str("mouldy"), py_str("mould"), py_str("ripe"), py_str("ripen"), py_str("unripe"), py_str("green"), py_str("plant"), py_str("growing"), py_str("grow"), py_str("blight"), py_str("greenhouse"), py_str("chutney"), py_str("relish"), py_str("sauce recipe"), py_str("growbag")]) }
    fn __data78() -> Py { py_tuple(vec![py_str("better"), py_str("healthier"), py_str("worse"), py_str("or"), py_str("which"), py_str("than"), py_str("instead"), py_str("good for"), py_str("bad for"), py_str("sugar"), py_str("filling"), py_str("worth"), py_str("swap"), py_str("healthy"), py_str("better than"), py_str("healthier than"), py_str("worse than"), py_str("instead of"), py_str("rather than"), py_str("or porridge"), py_str("porridge or"), py_str("cereal or"), py_str("or cereal"), py_str("is it ok"), py_str("ok"), py_str("okay"), py_str("alright"), py_str("all right"), py_str("fine"), py_str("safe"), py_str("bad"), py_str("is porridge ok"), py_str("good or bad"), py_str("any good"), py_str("worth eating"), py_str("should i eat"), py_str("as good as"), py_str("as healthy as"), py_str("as filling as"), py_str("just as good")]) }
    fn __data79() -> Py { py_tuple(vec![py_str("overnight"), py_str("recipe"), py_str("cook"), py_str("cooking"), py_str("how do i make"), py_str("bake"), py_str("baking"), py_str("cold"), py_str("reheat"), py_str("reheating"), py_str("leftover"), py_str("left over"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("keeps"), py_str("how long"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("weaning"), py_str("my son"), py_str("my daughter"), py_str("cheaper"), py_str("price"), py_str("cost"), py_str("calories in"), py_str("bread"), py_str("flapjack"), py_str("cookie"), py_str("cookies"), py_str("biscuit"), py_str("smoothie"), py_str("milk"), py_str("gluten"), py_str("weaning"), py_str("cheaper"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("how much"), py_str("per kilo"), py_str("per 100g"), py_str("on offer"), py_str("bargain"), py_str("value"), py_str("box")]) }
    fn __data80() -> Py { py_tuple(vec![py_str("cereal"), py_str("cereals"), py_str("cornflakes"), py_str("muesli"), py_str("granola"), py_str("shreddies"), py_str("weetabix"), py_str("bran flakes"), py_str("porridge"), py_str("porridge oats"), py_str("oats"), py_str("oatmeal"), py_str("crunchy nut"), py_str("rice krispies"), py_str("coco pops"), py_str("frosties"), py_str("special k"), py_str("cheerios")]) }
    fn __data81() -> Py { py_tuple(vec![py_str("how much"), py_str("much"), py_str("portion"), py_str("portions"), py_str("serving"), py_str("servings"), py_str("per person"), py_str("how many"), py_str("should i eat"), py_str("should i have"), py_str("am i meant to"), py_str("bowl"), py_str("bowlful"), py_str("how much of"), py_str("too much"), py_str("enough")]) }
    fn __data82() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("make"), py_str("made"), py_str("recipe"), py_str("microwave"), py_str("hob"), py_str("pan"), py_str("overnight"), py_str("soak"), py_str("bake"), py_str("baking"), py_str("bread"), py_str("flapjack"), py_str("cookie"), py_str("cookies"), py_str("biscuit"), py_str("bar"), py_str("bars"), py_str("smoothie"), py_str("weaning"), py_str("baby"), py_str("toddler"), py_str("kids"), py_str("sugar"), py_str("calories"), py_str("calorie"), py_str("healthy"), py_str("healthier"), py_str("better"), py_str("milk"), py_str("water"), py_str("fibre"), py_str("fiber"), py_str("protein"), py_str("carbs"), py_str("carb"), py_str("iron"), py_str("vitamin"), py_str("vitamins"), py_str("salt"), py_str("nutrients"), py_str("nutrient"), py_str("worse"), py_str("gluten"), py_str("allergy"), py_str("allergic"), py_str("cupboard"), py_str("cupboards"), py_str("pantry"), py_str("larder"), py_str("moths"), py_str("weevils"), py_str("mice"), py_str("store"), py_str("storage"), py_str("keep"), py_str("keeps")]) }
    fn __data83() -> Py { py_tuple(vec![py_str("sweetener"), py_str("sweeteners"), py_str("aspartame"), py_str("sucralose"), py_str("stevia"), py_str("splenda"), py_str("canderel"), py_str("diet coke"), py_str("diet drinks"), py_str("diet drink"), py_str("coke zero"), py_str("sugar free"), py_str("sugar-free")]) }
    fn __data84() -> Py { py_tuple(vec![py_str("bad"), py_str("safe"), py_str("worse"), py_str("healthier"), py_str("cancer"), py_str("ok"), py_str("okay"), py_str("alright"), py_str("use"), py_str("better"), py_str("fine"), py_str("harm"), py_str("harmful"), py_str("instead"), py_str("good")]) }
    fn __data85() -> Py { py_tuple(vec![py_str("sugar"), py_str("better"), py_str("healthier"), py_str("than"), py_str("instead"), py_str("less"), py_str("swap"), py_str("worse"), py_str("or"), py_str("which"), py_str("good for"), py_str("good for you"), py_str("healthy"), py_str("healthier"), py_str("bad for"), py_str("bad for you")]) }
    fn __data86() -> Py { py_tuple(vec![py_str("better"), py_str("healthier"), py_str("which"), py_str("than"), py_str("or"), py_str("use"), py_str("fry"), py_str("frying"), py_str("cook"), py_str("cooking"), py_str("worth"), py_str("bad"), py_str("good"), py_str("healthy")]) }
    fn __data87() -> Py { py_tuple(vec![py_str("grow"), py_str("growing"), py_str("plant"), py_str("planting"), py_str("harvest"), py_str("sprouting"), py_str("chit"), py_str("chitting"), py_str("store"), py_str("storing"), py_str("boil"), py_str("boiling"), py_str("roast"), py_str("roasting"), py_str("mash"), py_str("chips"), py_str("wedges"), py_str("recipe"), py_str("cook"), py_str("salad"), py_str("soup"), py_str("gnocchi")]) }
    fn __data88() -> Py { py_tuple(vec![py_str("calories"), py_str("calorie"), py_str("fattening"), py_str("fatten"), py_str("healthy"), py_str("healthier"), py_str("unhealthy"), py_str("bad for you"), py_str("good for you"), py_str("slimming"), py_str("diet"), py_str("weight loss"), py_str("waistline"), py_str("carbs"), py_str("carb"), py_str("keto"), py_str("nutrition"), py_str("nutritious"), py_str("how many calories")]) }
    fn __data89() -> Py { py_tuple(vec![py_str("how do i make"), py_str("how do you make"), py_str("recipe"), py_str("parboil"), py_str("crispy"), py_str("crisp"), py_str("keep"), py_str("keeps"), py_str("freeze"), py_str("reheat"), py_str("leftover"), py_str("leftovers"), py_str("jacket"), py_str("baked"), py_str("mash"), py_str("chips"), py_str("crisps"), py_str("wedges"), py_str("salad"), py_str("soup"), py_str("grow"), py_str("plant"), py_str("sprout"), py_str("store")]) }
    fn __data90() -> Py { py_tuple(vec![py_str("skip lunch"), py_str("skip dinner"), py_str("skip a meal"), py_str("skip meals"), py_str("skipping meals"), py_str("skipping lunch"), py_str("skipping dinner"), py_str("miss lunch"), py_str("missed lunch"), py_str("miss a meal"), py_str("no lunch"), py_str("not eating lunch"), py_str("skip tea")]) }
    fn __data91() -> Py { py_tuple(vec![py_str("good for"), py_str("as good"), py_str("good"), py_str("healthy"), py_str("better"), py_str("fattening"), py_str("bad for"), py_str("which"), py_str("or"), py_str("than"), py_str("calcium"), py_str("protein"), py_str("worth"), py_str("need"), py_str("how much"), py_str("how many"), py_str("a day"), py_str("per day"), py_str("pint"), py_str("pints"), py_str("glass"), py_str("glasses"), py_str("should i drink"), py_str("drink a day"), py_str("can i have"), py_str("can i drink"), py_str("ok to have"), py_str("ok to drink"), py_str("should i have"), py_str("should i drink"), py_str("should i eat"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("safe"), py_str("alright"), py_str("any good"), py_str("worth it"), py_str("better than")]) }
    fn __data92() -> Py { py_tuple(vec![py_str("breast"), py_str("formula"), py_str("weaning"), py_str("allergy"), py_str("allergies"), py_str("intolerant"), py_str("intolerance"), py_str("lactose"), py_str("gone off"), py_str("off milk"), py_str("sour"), py_str("expired"), py_str("curdle"), py_str("curdled"), py_str("how long"), py_str("freeze"), py_str("goat"), py_str("make"), py_str("recipe"), py_str("sauce"), py_str("froth"), py_str("frothing"), py_str("steam"), py_str("coffee"), py_str("tea"), py_str("in porridge"), py_str("for porridge"), py_str("porridge"), py_str("in a sauce"), py_str("in the sauce"), py_str("in a cake"), py_str("in the cake"), py_str("bake"), py_str("baking"), py_str("cook"), py_str("cooking"), py_str("in a recipe"), py_str("for the recipe"), py_str("in batter"), py_str("in a smoothie"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("how much is"), py_str("these days"), py_str("pence"), py_str("pounds"), py_str("cheap"), py_str("cheapest"), py_str("supermarket"), py_str("shop"), py_str("pudding"), py_str("puddings"), py_str("dessert"), py_str("desserts"), py_str("cant eat"), py_str("can't eat"), py_str("cannot eat"), py_str("dairy free"), py_str("no dairy"), py_str("what can i")]) }
    fn __data93() -> Py { py_tuple(vec![py_str("after a run"), py_str("after the run"), py_str("after running"), py_str("after i run"), py_str("after my run"), py_str("after a saturday run"), py_str("after the gym"), py_str("after a workout"), py_str("after my workout"), py_str("after exercise"), py_str("afterwards"), py_str("post workout"), py_str("post-workout"), py_str("recover")]) }
    fn __data94() -> Py { py_tuple(vec![py_str("kettle"), py_str("boil"), py_str("brewing"), py_str("brew"), py_str("teapot"), py_str("cups of tea"), py_str("put in"), py_str("how much water"), py_str("milk"), py_str("steep"), py_str("how much"), py_str("how many"), py_str("limit"), py_str("a day"), py_str("per day")]) }
    fn __data95() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("cut down"), py_str("cutting down"), py_str("stop"), py_str("reduce"), py_str("limit"), py_str("week"), py_str("safe"), py_str("healthy"), py_str("too much"), py_str("a day"), py_str("per day"), py_str("should i"), py_str("good for you"), py_str("good for me"), py_str("is it good")]) }
    fn __data96() -> Py { py_tuple(vec![py_str("alcohol"), py_str("units"), py_str("beer"), py_str("wine"), py_str("booze"), py_str("drinking"), py_str("cut down"), py_str("limit"), py_str("a week"), py_str("per week"), py_str("too much"), py_str("safe"), py_str("reduce"), py_str("stop")]) }
    fn __data97() -> Py { py_tuple(vec![py_str("water"), py_str("fluids"), py_str("hydrate"), py_str("thirsty"), py_str("water bottle"), py_str("bottles of water"), py_str("united kingdom"), py_str("united states"), py_str("great britain"), py_str("kingdom"), py_str("countries"), py_str("country"), py_str("states"), py_str("nation"), py_str("nations"), py_str("glasses of water"), py_str("kettle"), py_str("squash"), py_str("juice"), py_str("milk"), py_str("full fat milk"), py_str("full-fat milk"), py_str("orange juice"), py_str("fizzy"), py_str("smoothie"), py_str("milkshake"), py_str("cordial"), py_str("kombucha"), py_str("driving"), py_str("drive"), py_str("drink driving"), py_str("behind the wheel"), py_str("breathalyser"), py_str("the morning after"), py_str("before driving"), py_str("drive home"), py_str("want to cut down"), py_str("want to cut"), py_str("want to reduce"), py_str("want to stop"), py_str("drink less"), py_str("drinking less"), py_str("stop drinking"), py_str("stopping drinking"), py_str("quit drinking"), py_str("give up drinking"), py_str("give up the drink"), py_str("cut down on drinking"), py_str("cut down on the drinking"), py_str("reduce my drinking"), py_str("i drink too much"), py_str("coffee"), py_str("coffees"), py_str("tea"), py_str("teas"), py_str("caffeine"), py_str("cola"), py_str("energy drink"), py_str("energy drinks"), py_str("baby"), py_str("babies"), py_str("newborn"), py_str("newborns"), py_str("infant"), py_str("infants"), py_str("toddler"), py_str("toddlers"), py_str("my baby"), py_str("month old"), py_str("months old"), py_str("week old"), py_str("weeks old"), py_str("year old"), py_str("years old"), py_str("formula"), py_str("breast milk"), py_str("breastfeeding"), py_str("ounces"), py_str("feed"), py_str("feeds"), py_str("feeding")]) }
    fn __data98() -> Py { py_tuple(vec![py_str("alcohol"), py_str("wine"), py_str("beer"), py_str("lager"), py_str("cider"), py_str("prosecco"), py_str("champagne"), py_str("gin"), py_str("vodka"), py_str("a drink"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("units"), py_str("booze")]) }
    fn __data99() -> Py { py_tuple(vec![py_str("breastfeeding"), py_str("breast feeding"), py_str("breast-feeding"), py_str("breastfeeding my"), py_str("nursing"), py_str("feeding my baby"), py_str("feeding the baby"), py_str("expressing"), py_str("pumping"), py_str("breast milk"), py_str("breastmilk"), py_str("while im feeding"), py_str("while breastfeeding"), py_str("milk supply"), py_str("my supply"), py_str("supply")]) }
    fn __data100() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("trimester"), py_str("trying for a baby"), py_str("coffee"), py_str("coffees"), py_str("caffeine"), py_str("espresso"), py_str("latte"), py_str("cappuccino"), py_str("tea"), py_str("teas"), py_str("green tea"), py_str("herbal tea"), py_str("peppermint tea"), py_str("camomile tea"), py_str("cup of tea"), py_str("cola"), py_str("coke"), py_str("pepsi"), py_str("fizzy drink"), py_str("fizzy water"), py_str("water"), py_str("juice"), py_str("orange juice"), py_str("apple juice"), py_str("squash"), py_str("lemonade"), py_str("energy drink"), py_str("sports drink"), py_str("milkshake"), py_str("smoothie"), py_str("hot chocolate"), py_str("drink milk"), py_str("drinks milk"), py_str("glass of milk"), py_str("cows milk"), py_str("cow's milk"), py_str("soya milk"), py_str("oat milk"), py_str("almond milk"), py_str("a milky drink"), py_str("my baby have"), py_str("my baby drink"), py_str("my baby has"), py_str("give my baby"), py_str("baby beer"), py_str("baby wine"), py_str("toddler"), py_str("coffee"), py_str("caffeine"), py_str("decaf"), py_str("tea"), py_str("cola"), py_str("coke"), py_str("diet coke"), py_str("energy drink"), py_str("energy drinks"), py_str("water"), py_str("juice"), py_str("squash"), py_str("lemonade"), py_str("hot chocolate"), py_str("matcha"), py_str("chai"), py_str("rooibos"), py_str("fennel tea"), py_str("nettle tea"), py_str("herbal"), py_str("peppermint"), py_str("camomile"), py_str("chamomile")]) }
    fn __data101() -> Py { py_tuple(vec![py_str("eat"), py_str("eating"), py_str("food"), py_str("foods"), py_str("diet"), py_str("curry"), py_str("curries"), py_str("spicy"), py_str("spice"), py_str("chilli"), py_str("chili"), py_str("garlic"), py_str("onion"), py_str("onions"), py_str("peanut"), py_str("peanuts"), py_str("nuts"), py_str("nut"), py_str("sushi"), py_str("beans"), py_str("cabbage"), py_str("broccoli"), py_str("citrus"), py_str("tomato"), py_str("tomatoes"), py_str("dairy"), py_str("cheese"), py_str("milk"), py_str("juice"), py_str("orange juice"), py_str("apple juice"), py_str("squash"), py_str("fizzy"), py_str("caffeine free"), py_str("decaf"), py_str("anything i eat"), py_str("what can i eat"), py_str("avoid any foods"), py_str("foods to avoid"), py_str("make my baby"), py_str("windy"), py_str("fussy"), py_str("gassy")]) }
    fn __data102() -> Py { py_tuple(vec![py_str("coffee"), py_str("coffees"), py_str("caffeine"), py_str("espresso"), py_str("latte"), py_str("cappuccino"), py_str("tap"), py_str("tap water"), py_str("bottled water"), py_str("boiled"), py_str("boil"), py_str("chlorine"), py_str("fluoride"), py_str("water filter"), py_str("safe to drink"), py_str("hard water"), py_str("soft water"), py_str("tea"), py_str("teas"), py_str("green tea"), py_str("cola"), py_str("energy drink"), py_str("energy drinks"), py_str("alcohol"), py_str("wine"), py_str("beer"), py_str("units"), py_str("paracetamol"), py_str("ibuprofen"), py_str("medicine"), py_str("medication"), py_str("antibiotic"), py_str("pregnant"), py_str("pregnancy"), py_str("my baby have"), py_str("give my baby"), py_str("my baby eat"), py_str("weaning"), py_str("month old"), py_str("months old"), py_str("year old"), py_str("how much milk"), py_str("how many ounces"), py_str("recipe"), py_str("cook"), py_str("bake")]) }
    fn __data103() -> Py { py_tuple(vec![py_str("alcohol"), py_str("drink"), py_str("drinking"), py_str("wine"), py_str("beer"), py_str("cider"), py_str("prosecco"), py_str("gin"), py_str("vodka"), py_str("a glass"), py_str("glass of"), py_str("a drink"), py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("worried")]) }
    fn __data104() -> Py { py_tuple(vec![py_str("alcohol"), py_str("drink"), py_str("drinking"), py_str("drank"), py_str("wine"), py_str("beer"), py_str("cider"), py_str("prosecco"), py_str("gin"), py_str("vodka"), py_str("booze"), py_str("a glass"), py_str("glass of")]) }
    fn __data105() -> Py { py_tuple(vec![py_str("breastfeeding"), py_str("nursing"), py_str("feeding a baby"), py_str("my dog"), py_str("my cat"), py_str("milk"), py_str("milkshake"), py_str("milkshakes"), py_str("juice"), py_str("orange juice"), py_str("squash"), py_str("smoothie"), py_str("smoothies"), py_str("water"), py_str("bottled water"), py_str("fizzy water"), py_str("coffee"), py_str("tea"), py_str("decaf"), py_str("decaff"), py_str("decaffeinated"), py_str("cola"), py_str("diet coke"), py_str("coke"), py_str("energy drink"), py_str("energy drinks"), py_str("hot chocolate"), py_str("lemonade")]) }
    fn __data106() -> Py { py_tuple(vec![py_str("drink more"), py_str("make myself"), py_str("forget"), py_str("remember"), py_str("hate water"), py_str("dont like water"), py_str("don't like water"), py_str("taste of water"), py_str("never drink"), py_str("keep forgetting"), py_str("struggle to drink"), py_str("getting enough"), py_str("how do i drink"), py_str("barely drink"), py_str("hardly drink"), py_str("barely any water"), py_str("hardly any water"), py_str("dont drink enough"), py_str("don't drink enough"), py_str("not drinking enough"), py_str("drink enough water"), py_str("dehydrated"), py_str("drink much"), py_str("drink any"), py_str("drink very little"), py_str("drink little"), py_str("drink nothing"), py_str("water at all"), py_str("any water"), py_str("much water"), py_str("little water"), py_str("rarely drink"), py_str("seldom drink")]) }
    fn __data107() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("litres"), py_str("liters"), py_str("pints"), py_str("glasses a day"), py_str("should i drink"), py_str("should i be drinking"), py_str("do i need to drink"), py_str("am i drinking enough"), py_str("too much"), py_str("so much"), py_str("stop drinking"), py_str("thirsty"), py_str("thirst"), py_str("quench"), py_str("drink enough water"), py_str("drinking enough water"), py_str("kettle"), py_str("boil"), py_str("teapot"), py_str("bottle feed"), py_str("formula"), py_str("plant"), py_str("garden"), py_str("tap"), py_str("taps"), py_str("faucet"), py_str("shower"), py_str("pipe"), py_str("pipes"), py_str("boiler"), py_str("cistern"), py_str("radiator"), py_str("pressure"), py_str("coming out"), py_str("comes out"), py_str("hot water"), py_str("cold water"), py_str("drain"), py_str("leak"), py_str("leaking"), py_str("mould"), py_str("bath"), py_str("sink")]) }
    fn __data108() -> Py { py_tuple(vec![py_str("phone"), py_str("iphone"), py_str("ipad"), py_str("tablet"), py_str("kindle"), py_str("charger"), py_str("charging"), py_str("charge"), py_str("battery"), py_str("laptop"), py_str("computer"), py_str("pc"), py_str("wifi"), py_str("wi fi"), py_str("internet"), py_str("download"), py_str("downloads"), py_str("downloading"), py_str("app"), py_str("apps"), py_str("printer"), py_str("boiler"), py_str("heating"), py_str("car"), py_str("engine"), py_str("toothbrush"), py_str("hoover"), py_str("vacuum"), py_str("bath"), py_str("baths"), py_str("sink"), py_str("sinks"), py_str("shower"), py_str("drain"), py_str("drains"), py_str("draining"), py_str("drainage"), py_str("plughole"), py_str("plug hole"), py_str("toilet"), py_str("pipe"), py_str("pipes"), py_str("plug")]) }
    fn __data109() -> Py { py_tuple(vec![py_str("meal"), py_str("dinner"), py_str("tea"), py_str("lunch"), py_str("supper"), py_str("dish"), py_str("that"), py_str("this"), py_str("it"), py_str("soup"), py_str("salad"), py_str("pasta"), py_str("rice"), py_str("noodles"), py_str("toast"), py_str("sandwich"), py_str("stew"), py_str("curry"), py_str("sauce"), py_str("tomato"), py_str("tomatoes"), py_str("eat"), py_str("eating"), py_str("cook"), py_str("cooking"), py_str("supper")]) }
    fn __data110() -> Py { py_tuple(vec![py_str("car"), py_str("cars"), py_str("van"), py_str("petrol"), py_str("diesel"), py_str("fuel"), py_str("the tank"), py_str("tyres"), py_str("borrow"), py_str("borrowing"), py_str("lends it"), py_str("keeps borrowing"), py_str("after a filling"), py_str("after my filling"), py_str("after the filling"), py_str("after the dentist"), py_str("after the dentist appointment"), py_str("calories"), py_str("how many calories"), py_str("lose weight"), py_str("losing weight"), py_str("slimming"), py_str("diet"), py_str("gain weight"), py_str("how much should i eat"), py_str("waking up hungry"), py_str("wake up hungry"), py_str("in the night"), py_str("at night"), py_str("why am i always hungry"), py_str("always hungry"), py_str("all the time"), py_str("no appetite"), py_str("not eating"), py_str("how long to cook"), py_str("recipe"), py_str("how do i make"), py_str("what should i eat"), py_str("what can i eat"), py_str("what shall i eat"), py_str("what do i eat"), py_str("meal ideas"), py_str("ideas for dinner"), py_str("washing machine"), py_str("machine"), py_str("washer"), py_str("dishwasher"), py_str("tumble dryer"), py_str("dryer"), py_str("boiler"), py_str("kettle"), py_str("bath"), py_str("radiator"), py_str("shower"), py_str("toilet"), py_str("loo"), py_str("loos"), py_str("lav"), py_str("lavatory"), py_str("wc"), py_str("bog"), py_str("cistern"), py_str("cisterns"), py_str("tank"), py_str("the tank"), py_str("header tank"), py_str("flush"), py_str("flushes"), py_str("flushing"), py_str("overflow"), py_str("dental"), py_str("dentist"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("cavity"), py_str("toothache"), py_str("hurts"), py_str("hurting"), py_str("hurt"), py_str("sensitive"), py_str("ache"), py_str("aching"), py_str("form"), py_str("forms"), py_str("paperwork"), py_str("questionnaire"), py_str("questionnaires"), py_str("application"), py_str("applications"), py_str("self assessment"), py_str("tax return"), py_str("the box"), py_str("this box"), py_str("that box"), py_str("fill in"), py_str("fills in"), py_str("filling in"), py_str("filled in"), py_str("fill out"), py_str("fills out"), py_str("filling out"), py_str("filled out"), py_str("fill this in"), py_str("fill it in"), py_str("fill that in"), py_str("fill this out"), py_str("fill that out"), py_str("fill it out")]) }
    fn __data111() -> Py { py_tuple(vec![py_str("dinner"), py_str("dinners"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("lunches"), py_str("meal"), py_str("meals"), py_str("eat"), py_str("eating"), py_str("cook"), py_str("cooking"), py_str("plate")]) }
    fn __data112() -> Py { py_tuple(vec![py_str("how many calories in"), py_str("calories in a"), py_str("calories in an"), py_str("calories in my"), py_str("how many calories are in"), py_str("how many calories does"), py_str("how many calories should"), py_str("count calories"), py_str("counting calories"), py_str("track calories"), py_str("calorie counting"), py_str("a day"), py_str("per day"), py_str("day should"), py_str("should i eat"), py_str("how many"), py_str("how much"), py_str("lose weight"), py_str("losing weight"), py_str("weight loss"), py_str("slimming world"), py_str("keto"), py_str("fasting"), py_str("intermittent"), py_str("in a banana"), py_str("in an egg"), py_str("recipe"), py_str("how do i make"), py_str("how long"), py_str("calorie deficit"), py_str("burn"), py_str("exercise"), py_str("gym")]) }
    fn __data18() -> Py { py_list(vec![entry(&[py_str("sugar-in-coke"), py_tuple(vec![py_tuple(vec![py_str("coke"), py_str("coca cola"), py_str("cola"), py_str("fizzy drink"), py_str("soft drink"), py_str("pepsi"), py_str("lemonade")]), py_tuple(vec![py_str("sugar"), py_str("calorie"), py_str("calories"), py_str("how much sugar"), py_str("teaspoons"), py_str("spoonfuls")])]), py_str("A 330ml can of regular Coke has 35g of sugar, about seven teaspoons, and 139 calories. Diet and zero versions have none, and the newer zero-sugar versions taste much closer to the original than they used to.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sugar-in-juice")); __args.push(py_tuple(vec![py_tuple(vec![py_str("orange juice"), py_str("fruit juice"), py_str("juice"), py_str("smoothie")]), __data19()])); __args.push(py_str("Fruit juice has as much sugar as the equivalent amount of Coke - about 20g in a 200ml glass of orange juice - with none of the fibre of the fruit and none of the chewing that makes you feel full. One small glass a day with a meal is the usual advice; a smoothie is not much better once the fruit is blended.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("statin"), py_str("statins"), py_str("medication"), py_str("medicines"), py_str("medicine"), py_str("pills"), py_str("tablets"), py_str("warfarin"), py_str("blood pressure tablets"), py_str("antibiotics")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sugary-drink")); __args.push(py_tuple(vec![__data20(), __data21()])); __args.push(py_func("_sugary_drink_reply", |args: &[Py]| _sugary_drink_reply(args))); __kws.extend(vec![("avoid".to_string(), __data22())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sparkling-water")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sparkling water"), py_str("fizzy water"), py_str("carbonated water"), py_str("soda water"), py_str("club soda"), py_str("sparkling water drink")]), __data23()])); __args.push(py_str("Sparkling water is water with carbon dioxide dissolved in it, so there is no sugar and no calories in it and it counts towards your fluids like still water. The fizz is a weak acid, which is why dentists say drink it with a meal rather than sip it all day, and it can leave you feeling bloated. The flavoured bottles are the ones to read: many of them carry sugar or sweetener.")); __kws.extend(vec![("avoid".to_string(), __data24())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chewing-gum-sweet")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chewing gum"), py_str("chewing-gum"), py_str("gum")]), __data25()])); __args.push(py_str("Sugar-free gum is good for your teeth: the chewing makes saliva, which washes the acid off, and a regular piece holds only about 2g of sugar anyway, so sugar-free is the one to pick. Give it to nobody's dog, though - the xylitol in sugar-free gum is poisonous to them.")); __kws.extend(vec![("avoid".to_string(), __data26())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("belly-fat"), py_tuple(vec![py_tuple(vec![py_str("belly fat"), py_str("stomach fat"), py_str("tummy fat"), py_str("spot reduce"), py_str("love handles")]), py_tuple(vec![py_str("lose"), py_str("get rid"), py_str("reduce"), py_str("burn"), py_str("how do i"), py_str("can i")])]), py_str("You cannot choose where fat comes off: sit-ups build the muscle under it, not the layer on top. A calorie deficit shrinks fat everywhere, and where it goes first is down to genetics, so for most people the waist is the last place to change.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sugar-craving")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sugar"), py_str("sweets"), py_str("sweet"), py_str("chocolate"), py_str("biscuits"), py_str("cake"), py_str("craving"), py_str("cravings")]), __data27()])); __args.push(py_str("Cut the sugar you drink first, because liquid sugar does not fill you up at all, then make sure you are eating protein and fibre at meals: most evening sugar cravings are hunger from a lunch that was too small or too white. Keep the sweet thing out of the house rather than relying on willpower at nine at night, and expect three or four days of feeling deprived before it settles.")); __kws.extend(vec![("avoid".to_string(), __data28()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sugar-swaps")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sugar"), py_str("sugary"), py_str("sweets"), py_str("biscuits"), py_str("chocolate")]), py_tuple(vec![py_str("cut out"), py_str("give up"), py_str("swap"), py_str("swaps"), py_str("instead"), py_str("replace"), py_str("alternative"), py_str("less sugar"), py_str("reduce"), py_str("cut down"), py_str("eat instead")])])); __args.push(py_str("Swap rather than erase: a yoghurt with fruit instead of a yoghurt with sugar in it, two squares of dark chocolate instead of a bar of milk, nuts or cheese instead of biscuits, and sparkling water or squash instead of fizzy drinks. Keep one sweet thing you really like, at a set time, rather than cutting sugar out completely - the all-or-nothing version is the one that collapses in a fortnight.")); __kws.extend(vec![("keep".to_string(), py_tuple(vec![py_str("cut out")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sugar-plain")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sugar"), py_str("sugars"), py_str("sugary"), py_str("added sugar"), py_str("free sugar"), py_str("free sugars"), py_str("white sugar")]), __data29()])); __args.push(py_str("In the amount most people eat, sugar is not poison, but it is the easiest calorie to take in unnoticed and the main cause of tooth decay. It is habit-forming rather than addictive like nicotine. Brown sugar is white sugar with molasses: same calories, same limit. Keep free sugars - the added sort, plus honey, syrup and juice - under 30g a day, about seven teaspoons, which one can of Coke uses up. Whole fruit is the exception: the fibre slows it down.")); __kws.extend(vec![("avoid".to_string(), __data30())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("snacking-night")); __args.push(py_tuple(vec![__data31(), py_tuple(vec![py_str("stop"), py_str("how do i"), py_str("why"), py_str("keep"), py_str("cant"), py_str("can't"), py_str("help"), py_str("avoid")])])); __args.push(py_str("Evening snacking is usually habit plus a lunch that was too small, not greed. Eat a proper lunch with protein and fibre in it, have your evening meal a bit later, then brush your teeth straight after it and leave the kitchen - it is far easier to change the routine than to resist the cupboard.")); __kws.extend(vec![("avoid".to_string(), __data32()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eat-more-veg")); __args.push(py_tuple(vec![py_tuple(vec![py_str("vegetables"), py_str("veg"), py_str("greens"), py_str("salad"), py_str("fruit and veg")]), __data33()])); __args.push(py_func("_veg_reply", |args: &[Py]| _veg_reply(args))); __kws.extend(vec![("avoid".to_string(), op_add(&__data34(), &g__DISLIKE())), ("ask".to_string(), __data35()), ("keep".to_string(), py_tuple(vec![py_str("my kids"), py_str("my kid"), py_str("children"), py_str("child"), py_str("kids"), py_str("toddler"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("dont eat"), py_str("don't eat"), py_str("do not eat")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("balanced-meal")); __args.push(py_tuple(vec![__data36(), __data37()])); __args.push(py_str("Half the plate vegetables or salad, a quarter protein (fish, chicken, beans, eggs), a quarter starchy carbohydrate, plus a thumb of fat and fruit for pudding. If most meals look roughly like that and you drink water rather than fizzy drinks, you do not need to count anything.")); __kws.extend(vec![("avoid".to_string(), __data38()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eat-better-no-cooking")); __args.push(py_tuple(vec![g__EAT_BETTER(), __data39()])); __args.push(py_str("You can eat well without cooking if you build the plate from the shop: a bag of salad or a punnet of tomatoes, ready-cooked chicken or a tin of tuna or chickpeas, and bread, couscous or a microwave pouch of rice. Frozen veg straight from the microwave is the cheapest half-plate of vegetables there is, and a pot of yogurt with fruit is pudding. The only rule that matters is the half plate of vegetables, and that needs no hob.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("how long"), py_str("oven"), py_str("roast"), py_str("bake"), py_str("baking"), py_str("fry"), py_str("boil"), py_str("what can i cook"), py_str("what can i make")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("carbs-are-bad"), py_tuple(vec![py_tuple(vec![py_str("carbs"), py_str("carbohydrate"), py_str("carbohydrates"), py_str("bread"), py_str("pasta"), py_str("rice"), py_str("potatoes")]), py_tuple(vec![py_str("bad"), py_str("fattening"), py_str("good for you"), py_str("unhealthy"), py_str("should i avoid"), py_str("should i cut"), py_str("give up"), py_str("cut out")])]), py_str("Carbohydrate is not fattening in itself - it has 4 calories a gram, the same as protein and less than half of fat's 9 - and wholegrain versions carry the fibre that keeps you full. What adds up is the fat and sugar that arrives with it (chips, garlic bread, cream sauces), and the portions. Wholegrain carbs are worth keeping.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-fattening")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("noodles")]), py_tuple(vec![py_str("fattening"), py_str("bad for you"), py_str("make you fat"), py_str("healthy"), py_str("lose weight"), py_str("diet")])])); __args.push(py_str("Pasta on its own is about 130 calories per 100g cooked and no more fattening than rice or bread: it is the portion (75 to 100g dry) and what goes on it - cream, cheese, oil - that adds up. Wholewheat pasta has more fibre, and cooling and reheating it makes some of the starch harder to digest, which blunts the blood sugar rise a little.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rice-or-pasta")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("pasta"), py_str("noodles"), py_str("bread"), py_str("potatoes"), py_str("potato")]), py_tuple(vec![py_str("worse"), py_str("better"), py_str("slimming"), py_str("fattening"), py_str("healthier"), py_str("or rice"), py_str("or pasta"), py_str("instead of"), py_str("rather than"), py_str("which is")])])); __args.push(py_str("Rice, pasta and boiled potatoes are much the same for you: about 130 to 150 calories per 100g cooked, so the portion is what matters - 75 to 100g dry of rice or pasta, or a potato the size of your fist. Pick whichever you will actually eat and fill half the plate with vegetables; the wholemeal versions are a small upgrade, not a different food.")); __kws.extend(vec![("avoid".to_string(), __data40()), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_not_a_keeping_question", |args: &[Py]| _not_a_keeping_question(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("brown-rice")); __args.push(py_tuple(vec![__data41(), __data42()])); __args.push(py_func("_wholegrain", |args: &[Py]| _wholegrain(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data43())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("frozen-and-tinned")); __args.push(py_tuple(vec![__data44(), __data45()])); __args.push(py_func("_frozen_and_tinned_reply", |args: &[Py]| _frozen_and_tinned_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data46())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eggs-cholesterol")); __args.push(py_tuple(vec![py_tuple(vec![py_str("egg"), py_str("eggs")]), py_tuple(vec![py_str("cholesterol"), py_str("bad for"), py_str("heart"), py_str("how many"), py_str("safe"), py_str("good for"), py_str("can i eat"), py_str("should i eat"), py_str("can i have"), py_str("every day"), py_str("everyday"), py_str("daily")])])); __args.push(py_str("Eggs are back on the everyday list: the cholesterol in food turned out to matter far less than the saturated fat in your diet, so most people can eat them daily. If you have familial high cholesterol or heart disease, ask your GP - otherwise one or two a day is fine, and they are one of the cheapest good sources of protein.")); __kws.extend(vec![("ask".to_string(), __data47()), ("guard".to_string(), g_not_a_count_for_the_box_or_the_dish()), ("avoid".to_string(), __data48()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("red-meat")); __args.push(py_tuple(vec![py_tuple(vec![py_str("red meat"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("bacon"), py_str("sausages"), py_str("processed meat"), py_str("ham"), py_str("steak"), py_str("mince"), py_str("minced beef")]), py_tuple(vec![py_str("bad"), py_str("healthy"), py_str("unhealthy"), py_str("cancer"), py_str("how much"), py_str("how often"), py_str("should i"), py_str("safe"), py_str("good")])])); __args.push(py_str("The UK advice is no more than 70g of red meat a day, and to go easy on processed meat (bacon, ham, sausages) because the cancer link is strongest there. Lean red meat is a good source of iron and zinc, so it is a question of amount and frequency rather than giving it up: two or three meat meals a week and fish or beans the rest.")); __kws.extend(vec![("avoid".to_string(), __data49()), ("guard".to_string(), py_func("not_a_cooking_question", |args: &[Py]| not_a_cooking_question(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("fruit-sugar"), py_tuple(vec![py_tuple(vec![py_str("fruit"), py_str("fructose"), py_str("grapes"), py_str("mango")]), py_tuple(vec![py_str("too much"), py_str("sugar"), py_str("fattening"), py_str("bad"), py_str("how much fruit")])]), py_str("Whole fruit is not the same as added sugar: the fibre and water slow it down and one piece is about 60 calories. The only people who need to limit it are those told to by a doctor, and even then it is juice and dried fruit that matter most. Two or three pieces a day is a normal amount.")]), entry(&[py_str("fried-food"), py_tuple(vec![py_tuple(vec![py_str("fried"), py_str("frying"), py_str("chips"), py_str("takeaway"), py_str("takeaways"), py_str("kebab"), py_str("fried chicken")]), py_tuple(vec![py_str("healthy"), py_str("bad"), py_str("how often"), py_str("unhealthy"), py_str("ok to eat"), py_str("safe")])]), py_str("Frying adds about 50 to 100 calories per portion of oil soaked in, and oven chips have roughly a third of the fat of fried ones. Frying at the right temperature helps: if the oil is hot enough the food steams from inside and takes on less. Once or twice a week is not the problem; it is the rest of the plate that matters.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-label")); __args.push(py_tuple(vec![__data50(), __data51()])); __args.push(py_str("Work in per 100g, not per serving, because serving sizes are chosen to look small. On that column, high is more than 17.5g of fat, 5g of saturates, 22.5g of sugar or 1.5g of salt; low is 3g of fat, 1.5g of saturates, 5g of sugar or 0.3g of salt. Traffic-light labels colour the same numbers for you. Check the ingredients list too: whatever is listed first is what there is most of, and sugar has many names - syrup, concentrate, juice, maltodextrin - so a label can be sugar-heavy without the word appearing near the top.")); __kws.extend(vec![("avoid".to_string(), __data52())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("salt-too-much"), py_tuple(vec![py_tuple(vec![py_str("salt"), py_str("salty"), py_str("sodium")]), __data53()]), py_str("The limit for an adult is 6g of salt a day, about a level teaspoon, and most people eat 8g without ever adding any: bread, cheese, bacon, sauces and ready meals are where it hides. Check the label - 1.5g of salt per 100g is high, 0.3g is low - and taste before you add more at the table.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fibre")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fibre"), py_str("fiber"), py_str("roughage")]), __data54()])); __args.push(py_str("Aim for 30g of fibre a day, and most people get half that. The easy sources are beans and lentils (7 to 8g a tin), wholegrain bread and pasta, porridge oats, and fruit with the skin on. Increase it slowly and drink more water, or you will feel bloated for a week.")); __kws.extend(vec![("keep".to_string(), py_tuple(vec![py_str("dont eat"), py_str("don't eat"), py_str("do not eat")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("more-protein")); __args.push(py_tuple(vec![py_tuple(vec![py_str("protein"), py_str("proteins")]), __data55()])); __args.push(py_str("Aim for a palm-sized portion of protein at each meal - eggs, milk or yoghurt at breakfast, beans, lentils or chicken at lunch, fish or mince at dinner - which lands most adults near the 0.75g per kilo of body weight they need. Greek yoghurt, cottage cheese, tinned fish, eggs, tofu and lentils are the cheapest ways to add it.")); __kws.extend(vec![("avoid".to_string(), __data56())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("more-of-a-nutrient")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MORE_OF()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk1[0].clone(); let mut _text: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), g__MORE_OF_ASK()])); __args.push(py_func("_more_of", |args: &[Py]| _more_of(args))); __kws.extend(vec![("avoid".to_string(), __data57()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("low-iron")); __args.push(py_tuple(vec![__data58(), __data59()])); __args.push(py_func("_low_iron", |args: &[Py]| _low_iron(args))); __kws.extend(vec![("avoid".to_string(), __data60()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("vitamins"), py_tuple(vec![py_tuple(vec![py_str("vitamins"), py_str("supplements"), py_str("multivitamin"), py_str("vitamin")]), py_tuple(vec![py_str("should i take"), py_str("need"), py_str("worth"), py_str("any point"), py_str("take")])]), py_str("For most people with a varied diet the only supplement worth taking is vitamin D in autumn and winter (10 micrograms a day), plus folic acid if you are pregnant or trying. A multivitamin mostly makes expensive urine; if you eat little fish, vegans should add B12.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nutrient-a-day")); __args.push(py_tuple(vec![py_tuple(vec![py_str("calcium"), py_str("iron"), py_str("protein"), py_str("magnesium"), py_str("potassium"), py_str("zinc"), py_str("folic acid"), py_str("folate"), py_str("vitamin d"), py_str("vitamin c"), py_str("vitamin b12"), py_str("b12")]), __data61()])); __args.push(py_func("_a_day", |args: &[Py]| _a_day(args))); __kws.extend(vec![("avoid".to_string(), __data62()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nutrient-role")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NUTRIENT_ROLE()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk1[0].clone(); let mut _text: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), g__ROLE_ASK()])); __args.push(py_func("_nutrient_role", |args: &[Py]| _nutrient_role(args))); __kws.extend(vec![("avoid".to_string(), __data63()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pregnancy-nutrients")); __args.push(py_tuple(vec![g__PREGNANT(), py_tuple(vec![py_str("folic"), py_str("folate"), py_str("iron"), py_str("calcium"), py_str("b12"), py_str("vitamin a"), py_str("vitamin d"), py_str("vit d"), py_str("liver")])])); __args.push(py_func("_pregnancy_vitamins", |args: &[Py]| _pregnancy_vitamins(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("cost"), py_str("costs"), py_str("price"), py_str("cheapest"), py_str("how much does"), py_str("is in"), py_str("are in"), py_str("how much iron in"), py_str("how much folic acid in"), py_str("calories"), py_str("sugar"), py_str("salt")])), ("keep".to_string(), op_add(&g__PREGNANT(), &py_tuple(vec![py_str("baby"), py_str("babies")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("healthy-weight")); __args.push(py_tuple(vec![__data64(), py_tuple(vec![py_str("what"), py_str("should"), py_str("healthy"), py_str("is"), py_str("am i"), py_str("how"), py_str("check")])])); __args.push(py_str("Healthy weight is a range, not a number: a BMI of 18.5 to 24.9 for most adults, which for someone 1.70m tall is roughly 54kg to 72kg. BMI gets it wrong for very muscular people and for older people who have lost muscle, so add a waist measurement - under 94cm for a man and 80cm for a woman is the usual guide. If your weight has changed without trying, that is worth a word with a GP.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&pyrt::xrefs::xget("knowledge.pets", "ANIMAL_NOUNS"), &__data65()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count-calories")); __args.push(py_tuple(vec![py_tuple(vec![py_str("count calories"), py_str("counting calories"), py_str("count your calories"), py_str("track calories"), py_str("tracking calories"), py_str("calorie counting"), py_str("log my food"), py_str("food diary"), py_str("myfitnesspal")]), __data66()])); __args.push(py_str("You do not need to count calories to lose weight, and most people who do stop within a month. What works is changing a few things you can see: no liquid calories, half the plate vegetables, weighing out pasta and rice rather than eyeballing them. If you like numbers, a two-week food diary tells you where the calories actually are, which is more useful than tracking forever.")); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("before-or-after-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("before or after"), py_str("before and after"), py_str("which is better"), py_str("eat first or")]), py_tuple(vec![py_str("eat"), py_str("eating"), py_str("food"), py_str("meal"), py_str("carbs"), py_str("protein"), py_str("fuel")])])); __args.push(py_str("Both, for different reasons: something small and mostly carbohydrate one to two hours before gives you the energy for the session, and some protein within a couple of hours after helps the muscle repair. If you only do one, the after matters more for strength training and the before matters more for a long run.")); __kws.extend(vec![("ask".to_string(), __data67()), ("avoid".to_string(), __data68()), ("guard".to_string(), py_func("not_a_dose", |args: &[Py]| not_a_dose(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("emotional-eating"), py_tuple(vec![py_tuple(vec![py_str("eat when"), py_str("eating when"), py_str("comfort eating"), py_str("emotional eating"), py_str("eat my feelings"), py_str("food when"), py_str("i eat too much"), py_str("cant stop eating"), py_str("can't stop eating"), py_str("overeating")]), __data69()]), py_str("Eating to manage a feeling is very ordinary, and the feeling does not go away because you tell yourself off for it. The practical version: eat proper meals so hunger is not part of it, keep the triggering food out of the house, and put a ten-minute walk or a phone call between the urge and the cupboard - the urge usually passes. If it happens most days and you feel out of control, that is worth talking to a GP about; Beat's helpline (0808 801 0677) is free and does not judge.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-guilt")); __args.push(py_tuple(vec![__data70(), py_tuple(vec![py_str("eating"), py_str("ate"), py_str("food"), py_str("after"), py_str("help"), py_str("stop"), py_str("why"), py_str("how")])])); __args.push(py_str("One big meal does not undo anything - a day of overeating is a few hundred calories over, not a week's worth - so the useful move is to go back to a normal meal at the normal time rather than to skip or punish yourself. If you are bingeing regularly, or making yourself sick or using laxatives, please tell a GP: that is treatable, and Beat (0808 801 0677) will talk it through with you first.")); __kws.extend(vec![("avoid".to_string(), __data71())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("carbs-at-night"), py_tuple(vec![py_tuple(vec![py_str("carbs"), py_str("carbohydrate"), py_str("pasta"), py_str("rice"), py_str("bread"), py_str("potatoes")]), py_tuple(vec![py_str("at night"), py_str("in the evening"), py_str("before bed"), py_str("late"), py_str("night")])]), py_str("Carbs in the evening do not turn into fat any faster than carbs at lunch: your body stores what it does not use whenever you eat it. A plate of pasta at nine is only a problem if it is on top of a full day of eating or sits so heavy it spoils your sleep. Try to leave two hours between dinner and bed.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fasting")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fasting"), py_str("fast"), py_str("16 8"), py_str("5 2"), py_str("intermittent")]), py_tuple(vec![py_str("good for you"), py_str("healthy"), py_str("work"), py_str("worth"), py_str("should i"), py_str("safe"), py_str("lose weight"), py_str("any good")])])); __args.push(py_str("Fasting works for weight loss because it makes eating less happen almost by accident, not because it does anything special: 16:8 and 5:2 come out level with ordinary calorie cutting in the trials. Pick it only if skipping breakfast suits you - if it leaves you raiding the cupboard at nine at night, it will not work. Not for anyone pregnant, underweight or with a history of eating disorders.")); __kws.extend(vec![("avoid".to_string(), __data72())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cheese-bad")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cheese"), py_str("cheddar"), py_str("dairy")]), py_tuple(vec![py_str("bad for you"), py_str("healthy"), py_str("fattening"), py_str("unhealthy"), py_str("good for you"), py_str("eat"), py_str("how much")])])); __args.push(py_str("Cheese is calorie-dense - about 400 calories per 100g of cheddar - but a portion is only 30g, roughly a matchbox, and it brings calcium and protein with the fat. It is not a food to avoid, it is a food to weigh: grated rather than sliced goes further.")); __kws.extend(vec![("avoid".to_string(), __data73())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spices-are-fine")); __args.push(py_tuple(vec![__data74(), __data75()])); __args.push(py_str("Spices are not bad for you: they are the cheapest way to make plain food taste of something, which is why a boring diet gets eaten less. Turmeric, cinnamon, garlic, ginger and black pepper do no harm in the amounts that go in cooking, and the salt and the heat are what to watch - a hot dish is painful rather than harmful. Watch the blends and the ready-made sauces, because they carry the salt.")); __kws.extend(vec![("avoid".to_string(), __data76())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tomato-nutrition")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("cherry tomatoes"), py_str("plum tomatoes"), py_str("tinned tomatoes"), py_str("tomato juice")]), py_tuple(vec![py_str("healthy"), py_str("good for"), py_str("bad for"), py_str("fattening"), py_str("unhealthy"), py_str("nutrition"), py_str("nutritious"), py_str("vitamins"), py_str("calories"), py_str("how many calories"), py_str("worth eating"), py_str("count as")])])); __args.push(py_str("Tomatoes are one of the good everyday foods: about 18 calories per 100g, so a whole one is roughly 20, and they bring vitamin C, potassium and lycopene. Cooked with a little oil the lycopene is easier for the body to use, so tinned tomatoes and a home-made sauce count as well as fresh ones - the salt and sugar in a bought jar are what to watch, not the tomatoes.")); __kws.extend(vec![("avoid".to_string(), __data77())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("sushi"), py_tuple(vec![py_tuple(vec![py_str("sushi"), py_str("sashimi")]), py_tuple(vec![py_str("healthy"), py_str("good for you"), py_str("fattening"), py_str("calories"), py_str("is it")])]), py_str("Sushi is a good everyday choice: fish, rice and vegetables, with very little fat, and a supermarket box is usually 300 to 500 calories. The traps are the fried fillings, the mayonnaise-heavy boxes and the soy sauce, which is salty enough to matter if you pour in the whole sachet.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wholemeal-bread")); __args.push(py_tuple(vec![py_tuple(vec![py_str("brown bread"), py_str("wholemeal"), py_str("white bread"), py_str("granary bread"), py_str("wholegrain bread"), py_str("seeded bread")]), py_tuple(vec![py_str("better"), py_str("healthier"), py_str("which"), py_str("should i"), py_str("than"), py_str("difference"), py_str("bad for"), py_str("good for"), py_str("worth")])])); __args.push(py_str("Wholemeal is the better everyday loaf: the same calories as white, but two to three times the fibre, and the fibre is what keeps you full and helps your gut. Brown bread is not the same thing - it is often white flour with colouring and a little bran added, so look for the word wholemeal or wholegrain on the label. It is worth the swap; it is not worth worrying about when there is only white in the house.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("rice"), py_str("pasta"), py_str("noodles"), py_str("potato"), py_str("potatoes")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, entry(&[py_str("butter-margarine"), py_tuple(vec![py_tuple(vec![py_str("butter"), py_str("margarine"), py_str("spread"), py_str("lurpak")]), py_tuple(vec![py_str("better"), py_str("healthier"), py_str("or"), py_str("which"), py_str("should i use")])]), py_str("Both are about 100 calories a tablespoon, so the difference is small. Butter is saturated fat; the olive- and rapeseed-oil spreads are the healthier swap if you use a lot of it, and the ones with added plant sterols do lower cholesterol a little. If you use a scraping on toast, use whichever you like more.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("porridge-vs-cereal")); __args.push(py_tuple(vec![py_tuple(vec![py_str("porridge"), py_str("oats"), py_str("oatmeal"), py_str("cereal"), py_str("cereals"), py_str("cornflakes"), py_str("muesli"), py_str("granola"), py_str("shreddies"), py_str("weetabix"), py_str("bran flakes"), py_str("crunchy nut")]), __data78()])); __args.push(py_func("_cereal_verdict", |args: &[Py]| _cereal_verdict(args))); __kws.extend(vec![("avoid".to_string(), __data79()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cereal-portion")); __args.push(py_tuple(vec![__data80(), __data81()])); __args.push(py_str("A 40g serving of cereal is one small bowl - check the box, because granola and muesli are heavier per spoon than cornflakes, and most people pour about twice the serving into a normal bowl. For porridge, 40g of dry oats is one portion, which is about four heaped tablespoons or half a mug; it swells to a proper bowl with 250ml of milk or water. If a 40g bowl leaves you hungry by ten, add a banana or a spoon of yogurt rather than more cereal, and if you are measuring granola, weigh it once - a 50g serving can carry 12 to 15g of sugar.")); __kws.extend(vec![("avoid".to_string(), __data82()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sweeteners")); __args.push(py_tuple(vec![__data83(), __data84()])); __args.push(py_str("Sweeteners are safe at the amounts in drinks and yoghurts: aspartame, sucralose and stevia have been tested for decades, and the cancer alarm about aspartame is a hazard rating that would need something like twenty cans a day. The honest catch is that diet drinks do not seem to help people lose weight - they do not fill you up, and they keep a sweet tooth sweet. Water, tea or coffee is the cheaper swap.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("honey-vs-sugar")); __args.push(py_tuple(vec![py_tuple(vec![py_str("honey")]), __data85()])); __args.push(py_str("Honey is sugar with a nicer story: it is about 80 per cent sugar, and a teaspoon has roughly the same calories as a teaspoon of sugar, so it counts towards the 30g a day free-sugar limit in exactly the same way. It tastes sweeter, so you may use less of it. The one real difference is that honey is not for babies under one.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("sore throat"), py_str("cough"), py_str("cold"), py_str("lemon"), py_str("jar"), py_str("local honey"), py_str("hayfever"), py_str("hay fever"), py_str("allergy"), py_str("allergies")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cooking-oil")); __args.push(py_tuple(vec![py_tuple(vec![py_str("olive oil"), py_str("sunflower oil"), py_str("vegetable oil"), py_str("rapeseed oil"), py_str("coconut oil"), py_str("groundnut oil"), py_str("cooking oil"), py_str("frying oil"), py_str("rapeseed"), py_str("sunflower")]), __data86()])); __args.push(py_str("For everyday cooking, olive oil and rapeseed oil are the best all-rounders: mostly unsaturated fat, and rapeseed has the most omega-3 of the ordinary oils. Extra virgin olive oil is fine for frying at normal pan heat. Coconut oil is the one to hold back on, because it is about 85 per cent saturated fat. Whichever you use it is still about 120 calories a tablespoon, so how much matters more than which.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("hair"), py_str("skin"), py_str("drizzle"), py_str("dressing"), py_str("recipe"), py_str("substitute for"), py_str("run out"), py_str("gone off"), py_str("rancid"), py_str("smoke point"), py_str("splash")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("baked-potato")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baked potato"), py_str("baked potatoes"), py_str("jacket potato"), py_str("jacket potatoes"), py_str("potato"), py_str("potatoes")]), py_tuple(vec![py_str("healthy"), py_str("good for"), py_str("fattening"), py_str("bad for"), py_str("diet"), py_str("calories"), py_str("carbs"), py_str("better"), py_str("lose weight"), py_str("slimming"), py_str("weight")])])); __args.push(py_str("A jacket potato is a good everyday carbohydrate: about 200 calories for a medium one, more fibre than rice or pasta if you eat the skin, and it fills you up. What decides the meal is the topping - beans, tuna, cottage cheese or chilli are fine; a lake of butter, cheese and mayo turns it into the calorie bomb people blame the potato for.")); __kws.extend(vec![("avoid".to_string(), __data87())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("roast-potato-nutrition")); __args.push(py_tuple(vec![py_tuple(vec![py_str("roast potato"), py_str("roast potatoes"), py_str("roasties"), py_str("roastie"), py_str("roasted potatoes"), py_str("roasted potato")]), __data88()])); __args.push(py_str("About 200 calories per 100g as a side, nearer 300 if the tray is deep in oil - the potato itself is only about 90 before the fat goes on, so it is a carbohydrate like any other and the oil is what adds up. A thin film that just coats them, on a tray that is hot rather than swimming, is what keeps a roast potato an everyday thing.")); __kws.extend(vec![("avoid".to_string(), __data89())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("skip-a-meal")); __args.push(py_tuple(vec![__data90(), py_tuple(vec![py_str("bad"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("healthy"), py_str("worse"), py_str("matter"), py_str("harm"), py_str("should i"), py_str("is it"), py_str("problem"), py_str("wrong")])])); __args.push(py_str("Skipping a meal now and then is fine - a busy day, a lie-in, a big late breakfast - and nothing bad happens. What matters is the total you eat and drink across the day, and whether you get so hungry that you raid the cupboard at four. Skipping meals on purpose to control your weight tends to backfire; skipping one because you are not hungry is not a problem.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("fasting"), py_str("fast"), py_str("16:8"), py_str("intermittent")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("milk-choice")); __args.push(py_tuple(vec![py_tuple(vec![py_str("milk"), py_str("whole milk"), py_str("semi skimmed"), py_str("skimmed milk"), py_str("full fat milk"), py_str("oat milk"), py_str("soya milk"), py_str("almond milk"), py_str("dairy")]), __data91()])); __args.push(py_func("_a_plant_milk", |args: &[Py]| _a_plant_milk(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&__data92(), &g_DAY_WORDS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eat-after-run")); __args.push(py_tuple(vec![py_tuple(vec![py_str("eat"), py_str("eating"), py_str("food"), py_str("protein"), py_str("recover"), py_str("recovery")]), __data93()])); __args.push(py_str("Within an hour or so, something with carbohydrate and 20 to 30g of protein: milk and a banana, yoghurt with fruit, eggs on toast, a chicken sandwich. The window is wider than the gym lore says - the total protein and calories across the day matter more than the half hour - but eating soon after does help you feel human again.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breakfast-important")); __args.push(py_tuple(vec![py_tuple(vec![py_str("breakfast"), py_str("morning")]), py_tuple(vec![py_str("important"), py_str("skip"), py_str("skipping"), py_str("most important"), py_str("necessary"), py_str("eat"), py_str("should i")])])); __args.push(py_str("Breakfast is not magic, and skipping it does not make you fat or thin on its own: what matters is the total day. If you are hungry in the morning, eat something with protein and fibre - eggs, porridge, yoghurt and fruit - because it makes the mid-morning biscuit much less likely. If you are genuinely not hungry, waiting until lunch is fine.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("important"), py_str("skip"), py_str("skipping"), py_str("necessary"), py_str("is it"), py_str("does it matter"), py_str("true"), py_str("worth"), py_str("need to"), py_str("is breakfast")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coffee-good")); __args.push(py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("tea"), py_str("espresso"), py_str("latte")]), py_tuple(vec![py_str("bad for you"), py_str("healthy"), py_str("good for you"), py_str("heart"), py_str("safe"), py_str("caffeine"), py_str("good")])])); __args.push(py_func("_coffee_or_tea", |args: &[Py]| _coffee_or_tea(args))); __kws.extend(vec![("avoid".to_string(), __data94())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("alcohol-units")); __args.push(py_tuple(vec![py_tuple(vec![py_str("alcohol"), py_str("drinking"), py_str("beer"), py_str("wine"), py_str("booze"), py_str("units")]), __data95()])); __args.push(py_str("The UK limit is 14 units a week for men and women, spread over three or more days, and a unit is a small glass of wine or half a pint of ordinary beer - so a bottle of wine is about nine. Cutting down works better with rules than willpower: no drink on two nights a week, no drink in the house, and a smaller glass.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), g_not_a_present()), ("ask".to_string(), __data96()), ("avoid".to_string(), __data97())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("alcohol-while-breastfeeding")); __args.push(py_tuple(vec![__data98(), __data99()])); __args.push(py_str("Feed first, then have the drink: alcohol is in your milk at its highest about half an hour to an hour after a glass and is gone in two to three hours for one drink, and there is no need to pump and dump - that only empties the milk, it does not speed the alcohol up. So an odd glass with a meal is fine, especially if you feed just before it or wait a couple of hours after. Getting properly drunk is the thing to avoid, and so is sharing a bed or a sofa with the baby after drinking.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("keep".to_string(), py_tuple(vec![py_str("breastfeeding"), py_str("nursing"), py_str("my baby"), py_str("baby"), py_str("babies")])), ("avoid".to_string(), __data100())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-while-breastfeeding")); __args.push(py_tuple(vec![py_tuple(vec![py_str("breastfeeding"), py_str("breast feeding"), py_str("breast-feeding"), py_str("nursing"), py_str("lactating"), py_str("chestfeeding"), py_str("while im feeding"), py_str("while breastfeeding"), py_str("still feeding")]), __data101()])); __args.push(py_str("There is no list of foods to avoid while you are breastfeeding: eat normally and often, because feeding makes you hungry. Curry, garlic, onions and spicy food are all fine - your baby may notice the flavour in your milk and be a bit fussy for a day, and that passes without either of you giving anything up. Peanuts and other nuts are fine too unless your baby has already reacted to them. The fish rules from pregnancy stay: no shark, swordfish or marlin, and tuna to about two fresh steaks or four medium tins a week. If you think one food is upsetting them, note the day and try it again in a fortnight rather than cutting out a whole food group.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("keep".to_string(), py_tuple(vec![py_str("breastfeeding"), py_str("breast feeding"), py_str("breast-feeding"), py_str("nursing"), py_str("lactating"), py_str("chestfeeding"), py_str("while im feeding")])), ("avoid".to_string(), __data102())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("alcohol-in-pregnancy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("expecting a baby"), py_str("trying to conceive"), py_str("trying for a baby")]), __data103()])); __args.push(py_str("No amount of alcohol is known to be safe in pregnancy, so the advice is none at all. But a drink or two before you knew is very unlikely to have done any harm, and the worry is worse than the drink: most women who had one went on to have a healthy baby. Tell your midwife so it is in your notes, and if stopping is hard, ask for help rather than doing it alone.")); __kws.extend(vec![("keep".to_string(), py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("trying to conceive"), py_str("trying for a baby")])), ("ask".to_string(), __data104()), ("avoid".to_string(), __data105())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("caffeine-before-bed")); __args.push(py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("tea"), py_str("caffeine"), py_str("espresso"), py_str("energy drink"), py_str("green tea")]), py_tuple(vec![py_str("before bed"), py_str("at night"), py_str("in the evening"), py_str("bedtime"), py_str("evening"), py_str("keep you awake"), py_str("cant sleep"), py_str("can't sleep"), py_str("sleep"), py_str("awake"), py_str("bed")])])); __args.push(py_str("Caffeine is still in your body hours later: about half of it is gone after five or six hours, so a 4pm coffee is still a quarter there at midnight. Have your last ordinary cup by early afternoon and switch to decaf, herbal tea or water after that. If you are sensitive, even one after 6pm can cost you the deep sleep that does the repairing.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("before bed"), py_str("at night"), py_str("in the evening"), py_str("bedtime"), py_str("evening"), py_str("keep you awake"), py_str("sleep"), py_str("awake"), py_str("bed"), py_str("cant sleep"), py_str("can't sleep"), py_str("late")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("water-habit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("water"), py_str("hydrate"), py_str("hydration"), py_str("drinking"), py_str("fluid"), py_str("fluids")]), __data106()])); __args.push(py_str("Fill a bottle in the morning and keep it where you sit, and put a glass by the kettle so every cup of tea comes with one. Drinking with meals and after every trip to the loo is the cheapest trigger. Squash, tea and milk count towards your fluids, so if you dislike plain water, flavour it with lemon or have a diluted squash instead.")); __kws.extend(vec![("avoid".to_string(), __data107()), ("keep".to_string(), py_tuple(vec![py_str("dont drink"), py_str("don't drink"), py_str("do not drink")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eat-slowly")); __args.push(py_tuple(vec![py_tuple(vec![py_str("slowly"), py_str("slow"), py_str("chew"), py_str("full up"), py_str("still hungry")]), py_tuple(vec![py_str("eat"), py_str("eating"), py_str("meal"), py_str("chewing"), py_str("why")])])); __args.push(py_str("It takes about twenty minutes for the fullness signal to arrive, so a meal eaten in five leaves you looking for more afterwards. Put the fork down between mouthfuls, drink water with the meal and eat at a table without a screen - those three do more than any rule about chewing a set number of times.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("chew"), py_str("chewing"), py_str("full up"), py_str("fuller"), py_str("still hungry"), py_str("too fast"), py_str("slow down"), py_str("slowly"), py_str("meal"), py_str("mealtime"), py_str("meal times")])), ("avoid".to_string(), __data108())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-filling")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fill me up"), py_str("fills me up"), py_str("fill you up"), py_str("filling"), py_str("keep me full"), py_str("keep you full"), py_str("stay full"), py_str("stays with me"), py_str("still hungry"), py_str("hungry again"), py_str("hold me over"), py_str("keep me going")]), __data109()])); __args.push(py_str("Whether a dish holds you comes down to two things, and pasta or rice with a tomato sauce has neither: there is almost no protein in it and very little fat. Add a tin of chickpeas or beans, a boiled egg, a handful of grated cheese or a thick slice of bread and the same dish keeps you full for hours instead of one. The other half is the amount: a portion of dry pasta about the size of your closed fist is right for one person, and a tin of tomatoes counts as the vegetable, not the filling. The things that fill you up for least money are beans, lentils, potatoes, eggs, oats, yoghurt and wholemeal bread, and a soup or a green salad on its own rarely lasts past two hours.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("fill me up"), py_str("fills me up"), py_str("fill you up"), py_str("filling"), py_str("keep me full"), py_str("keep you full"), py_str("stay full"), py_str("stays with me"), py_str("still hungry"), py_str("hungry again"), py_str("hold me over"), py_str("keep me going")])), ("avoid".to_string(), __data110()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("light-dinner")); __args.push(py_tuple(vec![py_tuple(vec![py_str("healthy"), py_str("healthier"), py_str("light"), py_str("lightest"), py_str("low calorie"), py_str("low-calorie"), py_str("calorie"), py_str("calories"), py_str("kcal"), py_str("slimming"), py_str("diet")]), __data111()])); __args.push(py_str("A healthy dinner is a palm of protein, a fist of potato, rice or pasta, and half the plate vegetables: 150g of chicken with 200g of new potatoes and a big pile of broccoli comes to about 400 calories, so a 500-calorie cap is easy to stay under, and a salmon fillet or a fist of chickpeas makes the same plate again. Grill or roast rather than fry, and measure the oil, because a tablespoon of it is 100 calories on its own.")); __kws.extend(vec![("avoid".to_string(), __data112())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("weigh-weekly")); __args.push(py_tuple(vec![py_tuple(vec![py_str("weigh"), py_str("weighing"), py_str("scale"), py_str("scales"), py_str("weight")]), py_tuple(vec![py_str("how often"), py_str("every day"), py_str("every morning"), py_str("daily"), py_str("weekly")])])); __args.push(py_str("Weigh yourself once a week, same time, same clothes, rather than daily: body water swings 2 to 3 pounds a day and daily weighing mostly tells you about salt and hormones. Judge the trend over a month, and use a tape measure round your waist alongside it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how often"), py_str("often"), py_str("every day"), py_str("every morning"), py_str("daily"), py_str("weekly"), py_str("day")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.ENTRIES"); }
    s_ENTRIES(__data18());
    fn __data113() -> Py { py_tuple(vec![py_str("healthy"), py_str("healthier"), py_str("unhealthy"), py_str("good for you"), py_str("good for me"), py_str("good for us"), py_str("bad for you"), py_str("bad for me"), py_str("fattening"), py_str("make me fat"), py_str("ok to eat"), py_str("okay to eat"), py_str("fine to eat"), py_str("any good"), py_str("worth eating"), py_str("good for my heart"), py_str("bad for my heart"), py_str("should i eat"), py_str("should i avoid"), py_str("low fat"), py_str("good source"), py_str("nutritious"), py_str("can i eat"), py_str("can i still eat"), py_str("am i allowed to eat"), py_str("is it bad to eat"), py_str("is it ok to have"), py_str("is it okay to have"), py_str("good for"), py_str("good about"), py_str("health benefits"), py_str("benefits of"), py_str("what does it do for"), py_str("superfood"), py_str("a day"), py_str("per day"), py_str("every day"), py_str("everyday"), py_str("daily"), py_str("can i eat"), py_str("can i have"), py_str("can i drink"), py_str("am i allowed"), py_str("should i have"), py_str("should i eat"), py_str("should i drink"), py_str("how much should i eat"), py_str("how much can i eat"), py_str("how much of it"), py_str("how much of this"), py_str("what portion"), py_str("how big a portion"), py_str("portion size"), py_str("too much"), py_str("how much is too much")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.HEALTH_WORDS"); }
    s_HEALTH_WORDS(__data113());
    fn __data114() -> Py { py_tuple(vec![py_str("how do i"), py_str("how long"), py_str("have with"), py_str("have it with"), py_str("eat with"), py_str("goes with"), py_str("go with"), py_str("serve with"), py_str("serving with"), py_str("pairs with"), py_str("alongside"), py_str("on the side"), py_str("as a side"), py_str("side dish"), py_str("side dishes"), py_str("what side"), py_str("which side"), py_str("how much for"), py_str("how much does"), py_str("how much to"), py_str("how much do i need"), py_str("how much water"), py_str("how much milk"), py_str("how much flour"), py_str("per person"), py_str("for four"), py_str("for six"), py_str("for two"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("freeze"), py_str("frozen"), py_str("reheat"), py_str("leftover"), py_str("leftovers"), py_str("keep"), py_str("brown rice"), py_str("white rice"), py_str("wholegrain"), py_str("wholewheat"), py_str("wholemeal"), py_str("better"), py_str("which is"), py_str("difference"), py_str("or rice"), py_str("or pasta"), py_str("instead of"), py_str("cold"), py_str("cough"), py_str("sore throat"), py_str("symptom"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("garden"), py_str("allotment"), py_str("seed"), py_str("pot"), py_str("pots"), py_str("soil"), py_str("compost"), py_str("window box"), py_str("container"), py_str("pink"), py_str("raw"), py_str("undercooked"), py_str("bloody"), py_str("next day"), py_str("the next day"), py_str("the day after"), py_str("day after"), py_str("day later"), py_str("days later"), py_str("yesterday"), py_str("tomorrow"), py_str("in the fridge"), py_str("out of the fridge"), py_str("out of date"), py_str("past its date"), py_str("past the date"), py_str("use by"), py_str("still ok"), py_str("still good"), py_str("iron"), py_str("vitamin"), py_str("vitamins"), py_str("calcium"), py_str("protein"), py_str("fibre"), py_str("fiber"), py_str("potassium"), py_str("omega 3"), py_str("omega-3"), py_str("sugar"), py_str("salt"), py_str("yellow"), py_str("yellowing"), py_str("gone yellow"), py_str("gone off"), py_str("slimy"), py_str("mouldy"), py_str("moldy"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("went off"), py_str("past the date"), py_str("out of date"), py_str("left out"), py_str("overnight"), py_str("out of the fridge"), py_str("before a run"), py_str("after a run"), py_str("before bed"), py_str("brush"), py_str("brushing"), py_str("diabetes"), py_str("diabetic"), py_str("allergy"), py_str("allergic"), py_str("egg free"), py_str("egg-free"), py_str("nut free"), py_str("nut-free"), py_str("dairy free"), py_str("gluten free"), py_str("vegan"), py_str("vegetarian"), py_str("pregnant"), py_str("baby"), py_str("toddler"), py_str("leftover"), py_str("leftovers"), py_str("out to eat"), py_str("takeaway"), py_str("restaurant")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.HEALTH_AVOID"); }
    s_HEALTH_AVOID(__data114());
    fn __data116() -> Py { py_tuple(vec![py_str("feed fish"), py_str("feeds fish"), py_str("feeding fish"), py_str("fed fish"), py_str("feed a fish"), py_str("feed the fish"), py_str("feed my fish"), py_str("fish food"), py_str("goldfish"), py_str("aquarium"), py_str("fish tank"), py_str("tropical fish"), py_str("koi carp"), py_str("can i give my cat"), py_str("can i give my dog"), py_str("give my cat"), py_str("give my dog"), py_str("can my cat eat"), py_str("can my dog eat"), py_str("for my cat"), py_str("for my dog")]) }
    fn __data117() -> Py { py_tuple(vec![py_str("prawn"), py_str("prawns"), py_str("king prawns"), py_str("scallop"), py_str("scallops"), py_str("mussel"), py_str("mussels"), py_str("crab"), py_str("squid"), py_str("calamari"), py_str("oyster"), py_str("oysters"), py_str("shellfish"), py_str("prawn cocktail")]) }
    fn __data118() -> Py { py_tuple(vec![py_str("crab apple"), py_str("crab apples"), py_str("went off"), py_str("gone off"), py_str("smells off"), py_str("smells"), py_str("smell"), py_str("shell"), py_str("shells"), py_str("picked"), py_str("picking"), py_str("allergy"), py_str("allergic"), py_str("pregnant"), py_str("pregnancy"), py_str("open"), py_str("opened"), py_str("opening"), py_str("unopened"), py_str("shut"), py_str("closed"), py_str("dead"), py_str("didnt"), py_str("didn't"), py_str("stay shut"), py_str("stays shut"), py_str("stayed shut")]) }
    fn __data119() -> Py { py_tuple(vec![py_str("crack"), py_str("cracked"), py_str("shell"), py_str("shells"), py_str("rubbery"), py_str("fluffy"), py_str("filling"), py_str("fold"), py_str("cholesterol"), py_str("how many"), py_str("a day"), py_str("bad for you"), py_str("bad for me")]) }
    fn __data120() -> Py { py_tuple(vec![py_str("smoothie"), py_str("loaf"), py_str("cake"), py_str("bread"), py_str("banana bread"), py_str("muffin"), py_str("muffins"), py_str("ice cream"), py_str("frozen"), py_str("ripen"), py_str("ripe"), py_str("green"), py_str("peel"), py_str("slug"), py_str("fly"), py_str("flies"), py_str("juice"), py_str("juices"), py_str("cider"), py_str("smoothies"), py_str("dried"), py_str("crisps"), py_str("chips"), py_str("tree"), py_str("trees"), py_str("grow")]) }
    fn __data121() -> Py { py_tuple(vec![py_str("free"), py_str("flour"), py_str("bread"), py_str("recipe"), py_str("bake"), py_str("baking"), py_str("cake"), py_str("coeliac"), py_str("celiac"), py_str("allergy"), py_str("allergic"), py_str("intolerant"), py_str("intolerance"), py_str("sensitivity"), py_str("cant eat"), py_str("can't eat"), py_str("cannot eat"), py_str("without"), py_str("avoid"), py_str("cut out"), py_str("cutting out"), py_str("for lunch"), py_str("for breakfast"), py_str("for dinner"), py_str("for tea"), py_str("for supper"), py_str("for pudding"), py_str("what can i have"), py_str("what can i eat"), py_str("lunch"), py_str("breakfast"), py_str("dinner")]) }
    fn __data122() -> Py { py_tuple(vec![py_str("smoothie"), py_str("jam"), py_str("crumble"), py_str("pie"), py_str("cake"), py_str("muffin"), py_str("muffins"), py_str("freeze"), py_str("frozen"), py_str("ripen"), py_str("ripe"), py_str("keep"), py_str("wine"), py_str("slug"), py_str("slugs"), py_str("birds"), py_str("net")]) }
    fn __data123() -> Py { py_tuple(vec![py_str("recipe"), py_str("cake"), py_str("brownie"), py_str("brownies"), py_str("melt"), py_str("melted"), py_str("temper"), py_str("mould"), py_str("mold"), py_str("bloom"), py_str("gift"), py_str("easter"), py_str("hot chocolate"), py_str("store"), py_str("keep"), py_str("biscuit"), py_str("biscuits")]) }
    fn __data124() -> Py { py_tuple(vec![py_str("margarine"), py_str("spread"), py_str("than"), py_str("instead"), py_str("cook"), py_str("cooking"), py_str("fry"), py_str("frying"), py_str("bake"), py_str("baking"), py_str("soften"), py_str("melt"), py_str("melted"), py_str("goes off"), py_str("rancid"), py_str("churn"), py_str("beans"), py_str("peanut butter"), py_str("peanut"), py_str("peanuts"), py_str("nut butter"), py_str("almond butter"), py_str("cocoa butter"), py_str("shea butter"), py_str("bread and butter"), py_str("butterfly"), py_str("buttermilk")]) }
    fn __data125() -> Py { py_tuple(vec![py_str("forage"), py_str("foraging"), py_str("pick"), py_str("picking"), py_str("poisonous"), py_str("grow"), py_str("growing"), py_str("kit"), py_str("slug"), py_str("slugs"), py_str("fly"), py_str("flies"), py_str("slimy"), py_str("recipe"), py_str("soup"), py_str("stuffed"), py_str("stew"), py_str("gravy")]) }
    fn __data115() -> Py { py_tuple(vec![py_tuple(vec![py_str("rice-healthy"), py_tuple(vec![py_str("rice")]), py_str("Rice on its own is about 130 calories per 100g cooked and not fattening: what goes with it and how much of it decides that. A mug of cooked rice, about 150g, is a normal portion, and brown rice has around three times the fibre of white."), py_tuple(vec![])]), py_tuple(vec![py_str("bread-healthy"), py_tuple(vec![py_str("bread"), py_str("toast"), py_str("sourdough"), py_str("wrap"), py_str("wraps"), py_str("pitta"), py_str("bagel")]), py_str("Bread is fine: two slices of wholegrain or seeded is about 6g of fibre and a fair base for a meal. The white sliced loaf has less fibre and less staying power, so keep it for toast and make the everyday one wholegrain."), py_tuple(vec![py_str("burnt"), py_str("burned"), py_str("charred"), py_str("scorched"), py_str("burnt toast"), py_str("burned toast"), py_str("black")])]), py_tuple(vec![py_str("lentils-healthy"), py_tuple(vec![py_str("lentils"), py_str("lentil"), py_str("beans"), py_str("chickpeas"), py_str("pulses"), py_str("hummus")]), py_str("Lentils and beans are one of the best buys in the shop: a drained tin has around 8g of protein and 6g of fibre for about 120 calories. They count towards your five a day and they keep you full far longer than the same weight of rice."), py_tuple(vec![])]), py_tuple(vec![py_str("oily-fish-healthy"), py_tuple(vec![py_str("salmon"), py_str("mackerel"), py_str("sardines"), py_str("tuna"), py_str("fish"), py_str("kippers"), py_str("trout")]), py_str("Oily fish is worth two portions a week: the omega-3 in salmon, mackerel and sardines is what the heart and the joints use, and a 140g cooked portion is a serving. Tinned sardines and mackerel count and are the cheapest way to do it; tinned tuna is the lean one, so it gives protein rather than omega-3."), __data116()]), py_tuple(vec![py_str("shellfish-healthy"), __data117(), py_str("Shellfish is the leanest protein in the shop: 100g of prawns is about 100 calories and 20g of protein with barely any fat, and scallops and mussels are much the same, with iron and zinc in the molluscs. Prawns carry cholesterol, but that matters far less than the saturated fat in a diet, so it is the mayonnaise, the butter or the breadcrumbs that make a prawn dish heavy rather than the prawns."), __data118()]), py_tuple(vec![py_str("chicken-healthy"), py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("chicken breast"), py_str("chicken thighs")]), py_str("Chicken is lean protein - about 30g in a breast for 160 calories - so it is what you cook it in that matters. Skin off and not fried, it is one of the better everyday meats; the breaded and the deep-fried versions carry the fat."), py_tuple(vec![])]), py_tuple(vec![py_str("yoghurt-healthy"), py_tuple(vec![py_str("yoghurt"), py_str("yogurt"), py_str("greek yoghurt"), py_str("fromage frais"), py_str("skyr")]), py_str("Plain yoghurt is a good buy: about 10g of protein and a third of a day's calcium in a 150g pot. The fruit ones and the children's tubes are the ones with 12g of sugar or more, so buy plain and add your own fruit or a spoon of jam."), py_tuple(vec![])]), py_tuple(vec![py_str("eggs-healthy"), py_tuple(vec![py_str("omelette"), py_str("omelettes"), py_str("frittata"), py_str("scrambled egg"), py_str("scrambled eggs"), py_str("boiled egg"), py_str("boiled eggs"), py_str("poached egg"), py_str("poached eggs"), py_str("fried egg"), py_str("fried eggs")]), py_str("An egg is one of the cheapest good foods there is: about 78 calories and 6g of protein, with the B12 and vitamin D that people buy supplements for. The cholesterol in food turned out to matter far less than the saturated fat in the diet, so one or two a day is fine for most people. What makes an omelette heavy is the butter and the cheese, not the eggs."), __data119()]), py_tuple(vec![py_str("greens-healthy"), py_tuple(vec![py_str("spinach"), py_str("kale"), py_str("chard"), py_str("leafy greens"), py_str("watercress"), py_str("spring greens"), py_str("cavolo nero"), py_str("swiss chard")]), py_str("Spinach, kale and the other leafy greens are among the best value in the shop at about 25 to 35 calories per 100g: a bag wilted in a pan is one of your five a day and brings folate, vitamin C and vitamin K, plus nitrates that help blood pressure. The iron in spinach is real (2.7mg per 100g) but poorly absorbed, so eat it with something containing vitamin C - lemon, peppers, a glass of juice. One caution: if you get kidney stones, spinach is the leaf to go easy on, because of its oxalate."), py_tuple(vec![py_str("smoothie"), py_str("salad"), py_str("soup"), py_str("slug"), py_str("slugs"), py_str("limp"), py_str("wilted"), py_str("bag of"), py_str("dressing")])]), py_tuple(vec![py_str("broccoli-healthy"), py_tuple(vec![py_str("broccoli"), py_str("cauliflower"), py_str("sprouts"), py_str("brussels sprouts"), py_str("cabbage"), py_str("romanesco"), py_str("cavolo nero")]), py_str("The brassicas are worth eating often and cost about 30 to 40 calories per 100g: broccoli has more vitamin C than an orange per 100g, and the whole family brings folate, vitamin K and fibre. Steam it for 4 or 5 minutes rather than boiling it to bits, since long cooking is what loses the vitamin C. There is nothing in them that detoxes anything - the reason to eat them is that they fill half a plate for very few calories."), py_tuple(vec![py_str("smoothie"), py_str("salad"), py_str("soup"), py_str("slug"), py_str("slugs"), py_str("patch"), py_str("are sprouting"), py_str("is sprouting"), py_str("have sprouted"), py_str("has sprouted")])]), py_tuple(vec![py_str("carrots-healthy"), py_tuple(vec![py_str("carrot"), py_str("carrots")]), py_str("Carrots are about 40 calories per 100g and one decent carrot gives you more than a day's vitamin A, as beta-carotene. Cooked with a little fat - butter, oil, in a stew - you absorb more of it than from raw sticks, so both are worth eating. They do not turn you orange unless you eat them by the bagful for weeks, and they are one of the cheapest vegetables there is."), py_tuple(vec![py_str("smoothie"), py_str("soup"), py_str("cake"), py_str("muffin"), py_str("muffins"), py_str("juice"), py_str("slug"), py_str("slugs"), py_str("fly"), py_str("flies")])]), py_tuple(vec![py_str("banana-healthy"), py_tuple(vec![py_str("banana"), py_str("bananas"), py_str("apple"), py_str("apples"), py_str("orange"), py_str("oranges"), py_str("pear"), py_str("pears")]), py_str("A banana is about 100 calories with 3g of fibre and around 400mg of potassium, and the sugar in it (about 14g) comes with that fibre and water, so it is a snack rather than a problem - fresh fruit is not what the 30g free-sugar limit is aimed at. A greener one has more resistant starch, which is good for the gut; a spotted one is sweeter and easier to digest before a run. One or two a day is fine, and the same holds for apples, oranges and pears at 60 to 80 calories each."), __data120()]), py_tuple(vec![py_str("avocado-healthy"), py_tuple(vec![py_str("avocado"), py_str("avocados"), py_str("guacamole")]), py_str("Avocado is a good food that is easy to overdo: half a medium one is about 150 calories and 15g of fat, nearly all of it monounsaturated, which is the kind that helps cholesterol rather than harming it. It also brings about 5g of fibre and more potassium than a banana per 100g. Half a day is a sensible portion; a whole one on toast is a meal, not a spread."), py_tuple(vec![py_str("ripen"), py_str("ripe"), py_str("brown"), py_str("black"), py_str("stone"), py_str("keep"), py_str("freeze"), py_str("toast"), py_str("smash"), py_str("guacamole")])]), py_tuple(vec![py_str("gluten-healthy"), py_tuple(vec![py_str("gluten"), py_str("seitan"), py_str("vital wheat gluten")]), py_str("Gluten is just the protein in wheat, barley and rye, and it is only a problem for the roughly one in a hundred people with coeliac disease, or for a wheat allergy - for everyone else there is nothing to avoid. If bread and pasta leave you bloated, windy or tired, ask the GP for a coeliac blood test before you cut gluten out, because the test only works while you are still eating it. Wholegrain wheat is one of the best sources of fibre in an ordinary diet, so it is worth keeping unless a test says otherwise."), __data121()]), py_tuple(vec![py_str("nuts-healthy"), py_tuple(vec![py_str("nuts"), py_str("almonds"), py_str("walnuts"), py_str("cashews"), py_str("hazelnuts"), py_str("pistachios"), py_str("pecans"), py_str("brazil nuts"), py_str("mixed nuts")]), py_str("A 30g handful of nuts is about 175 calories and one of the better snacks you can buy: 6g of protein, 3g of fibre and fat that is mostly unsaturated, and people who eat them regularly tend to weigh less, not more, because they are satisfying. Walnuts are the one with omega-3 (plant ALA). Buy them unsalted and unsweetened, and count the handful out rather than eating from the bag."), py_tuple(vec![py_str("squirrel"), py_str("birds"), py_str("bird"), py_str("feed the"), py_str("nut free"), py_str("nut-free"), py_str("recipe"), py_str("cake"), py_str("roast"), py_str("roasted"), py_str("toast"), py_str("cost")])]), py_tuple(vec![py_str("peanut-butter-healthy"), py_tuple(vec![py_str("peanut butter"), py_str("peanut"), py_str("peanuts"), py_str("almond butter")]), py_str("Peanut butter is a decent everyday food: two tablespoons is about 190 calories with 7g of protein and 2g of fibre, and it keeps you full. The catch is the jar - buy one whose only ingredients are peanuts and salt, since the sweetened ones add sugar and palm oil - and use a level tablespoon rather than a spoon over the sink."), py_tuple(vec![py_str("jar"), py_str("recipe"), py_str("cookie"), py_str("cookies"), py_str("biscuit"), py_str("cake"), py_str("smell"), py_str("rancid"), py_str("stir")])]), py_tuple(vec![py_str("berries-healthy"), py_tuple(vec![py_str("berries"), py_str("blueberries"), py_str("strawberries"), py_str("raspberries"), py_str("blackberries"), py_str("cherries"), py_str("grapes")]), py_str("Berries are the best fruit for the calories - about 30 to 60 per 100g - and a 80g portion counts towards your five a day, with vitamin C, fibre and the red and purple pigments that give them their colour. Frozen ones are just as good and a lot cheaper out of season. A punnet is two or three portions, so tip half into a bowl rather than eating from the box."), __data122()]), py_tuple(vec![py_str("dark-chocolate-healthy"), py_tuple(vec![py_str("dark chocolate"), py_str("chocolate"), py_str("cocoa"), py_str("milk chocolate")]), py_str("Dark chocolate is the one to keep in the house if you want chocolate: 70 per cent cocoa is about 600 calories per 100g, so two squares is 120 or so, and it carries iron and magnesium with less sugar than milk chocolate. The heart-health claims come from cocoa flavanols and need far more than a square a day, so treat it as a nicer 120 calories, not a supplement."), __data123()]), py_tuple(vec![py_str("butter-healthy"), py_tuple(vec![py_str("butter"), py_str("ghee"), py_str("lard"), py_str("double cream")]), py_str("Butter is fine in small amounts and is not a health food: about 700 calories per 100g and half of it saturated fat, so a teaspoon on vegetables is 35 calories and a thick spread on two slices is nearer 200. The guideline is 20g of saturated fat a day for a woman and 30g for a man, which is why oil is the better everyday cooking fat and butter is the one for the taste."), __data124()]), py_tuple(vec![py_str("mushrooms-healthy"), py_tuple(vec![py_str("mushrooms"), py_str("mushroom"), py_str("chestnut mushrooms"), py_str("portobello"), py_str("shiitake")]), py_str("Mushrooms are about 20 calories per 100g, so a whole punnet is a portion for under 50: they bring 3g of protein, B vitamins including riboflavin and niacin, and selenium, plus the savoury taste that makes them useful in a meal that is meant to fill you up. The vitamin D some packs advertise comes from being treated with UV light - it is real, but a few minutes in the sun does the same job."), __data125()])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition.FOOD_HEALTH"); }
    s_FOOD_HEALTH(__data115());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._GOING_OFF"); }
    s__GOING_OFF(stdlib_call("re", "compile", &[py_str("\\b(?:rotting|rotten|rotted|going soft|gone soft|mouldy|moldy|mould|mold|gone off|going off|out of date|past it|bruised|overripe|over-ripe|slimy|wilted|wilting|sprouting|sprouted|furry|fuzzed|smells off|smelling off)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_STATE_NOT_SPOILAGE"); }
    s__A_STATE_NOT_SPOILAGE(stdlib_call("re", "compile", &[py_str("\\b(?:overripe|over-ripe|bruised|going soft|gone soft|wilted|wilting|sprouting|sprouted)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._A_VERDICT_ASK"); }
    s__A_VERDICT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:good for (?:you|me|us)|bad for (?:you|me)|healthy|unhealthy|fattening|nutritious|healthier|healthiest|vitamins?|minerals?|calories|how many calories)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._EDIBILITY_ASK"); }
    s__EDIBILITY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can i (?:still )?(?:eat|use|keep)|is it (?:ok|okay|safe|still)|is (?:it|that|this) bad|safe to eat|still eat|still good|still fine|still use|good to eat|throw (?:it|them|this|that)|throw (?:it |them )?(?:away|out)|bin it|keep it|keep them|how long|what can i do with|use (?:it|them)|off|spoilt|spoiled)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    let __iter126 = py_iter_or_raise(&g_FOOD_HEALTH());
    for __item in __iter126 {
    let __unpk127 = py_unpack(&__item.clone(), 4, None);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._food_id"); }
    s__food_id(__unpk127[0].clone());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._food_names"); }
    s__food_names(__unpk127[1].clone());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._food_line"); }
    s__food_line(__unpk127[2].clone());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._food_avoid"); }
    s__food_avoid(__unpk127[3].clone());
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__food_id()); __args.push(py_tuple(vec![g__food_names(), g_HEALTH_WORDS()])); __args.push(g__food_line()); __kws.extend(vec![("avoid".to_string(), op_add(&g_HEALTH_AVOID(), &g__food_avoid())), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_not_a_verdict_on_what_is_going_off", |args: &[Py]| _not_a_verdict_on_what_is_going_off(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    }
    fn __data128() -> Py { py_tuple(vec![py_str("how often"), py_str("how many"), py_str("how much"), py_str("a week"), py_str("per week"), py_str("every day"), py_str("every other day"), py_str("twice a week"), py_str("times a week"), py_str("in a week"), py_str("safe"), py_str("limit"), py_str("too much"), py_str("maximum")]) }
    fn __data129() -> Py { py_tuple(vec![py_str("smell"), py_str("went off"), py_str("off fish"), py_str("freshness"), py_str("defrost"), py_str("thaw"), py_str("recipe"), py_str("recipes"), py_str("sandwich"), py_str("salad"), py_str("pasta"), py_str("bake"), py_str("feed"), py_str("my cat"), py_str("my dog"), py_str("cat food"), py_str("dog food"), py_str("healthy"), py_str("good for you"), py_str("fattening"), py_str("nutritious"), py_str("calories"), py_str("protein"), py_str("carb"), py_str("carbs"), py_str("carbohydrate"), py_str("fat"), py_str("fats"), py_str("iron"), py_str("vitamin"), py_str("vitamins"), py_str("minerals"), py_str("omega"), py_str("nutrition"), py_str("nutrients"), py_str("ingredients")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tuna-a-week")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tuna")]), __data128()])); __args.push(py_str("Four medium tins a week, or two fresh tuna steaks: that is the limit for an adult, and it is about the mercury rather than the fat. Tuna is the one oily fish that does not count towards the two portions a week - canning strips most of the omega-3 - so it is protein, not a fish-oil dose. If you are pregnant or trying, keep to the same four tins; a child gets about half a tin as a portion.")); __kws.extend(vec![("avoid".to_string(), __data129()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    fn __data130() -> Py { py_tuple(vec![py_str("good for you"), py_str("good for me"), py_str("good for us"), py_str("bad for you"), py_str("bad for me"), py_str("fattening"), py_str("make me fat"), py_str("calorie"), py_str("calories"), py_str("heart"), py_str("cholesterol"), py_str("any good"), py_str("worth eating"), py_str("nutritious"), py_str("superfood"), py_str("every day"), py_str("daily"), py_str("a day"), py_str("should i use"), py_str("better than")]) }
    fn __data131() -> Py { py_tuple(vec![py_str("pick"), py_str("choose"), py_str("choosing"), py_str("buy"), py_str("buying"), py_str("best"), py_str("brand"), py_str("bottle"), py_str("tin"), py_str("harvest"), py_str("supermarket"), py_str("shop"), py_str("price"), py_str("cheap"), py_str("expensive"), py_str("label"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("stored"), py_str("fry"), py_str("fries"), py_str("frying"), py_str("fryer"), py_str("deep fat"), py_str("chip pan"), py_str("roast potatoes"), py_str("cake"), py_str("cakes"), py_str("baking"), py_str("sponge"), py_str("muffin"), py_str("car"), py_str("engine"), py_str("bike"), py_str("hair"), py_str("skin"), py_str("essential oil"), py_str("paint"), py_str("lamp"), py_str("olive tree"), py_str("grow"), py_str("growing")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("olive-oil-healthy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("olive oil"), py_str("olive oils"), py_str("extra virgin"), py_str("extra virgin olive oil"), py_str("evoo")]), __data130()])); __args.push(py_str("Olive oil is the best everyday fat you can keep in the kitchen: mostly monounsaturated, and the extra virgin sort brings the polyphenols that are the reason it is worth using raw. It is still 9 calories a gram, so a tablespoon is about 120 calories - the drizzle counts, and a recipe's 'glug' can be three of them. Pour it over a finished dish or bread for the taste, cook with a cheaper oil when the pan is hot, and read 'light' as light in taste, not in calories.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_HEALTH_AVOID(), &__data131())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    fn __data132() -> Py { py_tuple(vec![py_str("good"), py_str("healthy"), py_str("healthier"), py_str("unhealthy"), py_str("for you"), py_str("calorie"), py_str("calories"), py_str("eat"), py_str("eating"), py_str("every day"), py_str("daily"), py_str("snack"), py_str("snacks"), py_str("topping"), py_str("toppings"), py_str("put"), py_str("filling"), py_str("fill"), py_str("diet"), py_str("weight"), py_str("lose"), py_str("kids"), py_str("breakfast"), py_str("gluten")]) }
    fn __data133() -> Py { py_tuple(vec![py_str("risotto"), py_str("basmati"), py_str("brown rice"), py_str("white rice"), py_str("fried rice"), py_str("rice pudding"), py_str("krispie"), py_str("crispy"), py_str("krispies"), py_str("puffed rice"), py_str("rice cooker"), py_str("sushi"), py_str("paella"), py_str("how long"), py_str("how do i cook"), py_str("how do i make"), py_str("freeze"), py_str("frozen"), py_str("reheat"), py_str("leftover"), py_str("cupboard"), py_str("mould"), py_str("weevil"), py_str("pantry")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rice-cakes")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice cake"), py_str("rice cakes"), py_str("rice cracker"), py_str("rice crackers"), py_str("corn cake"), py_str("corn cakes")]), __data132()])); __args.push(py_str("They are about 30 to 35 calories each, so a plain one is low fat and low salt - but it is nearly all air, and air does not fill anyone up for long. Treat it as a plate rather than a snack: cottage cheese, peanut butter, hummus, mashed avocado or a boiled egg on top turns two cakes into something that lasts to lunch. The flavoured and chocolate-coated ones are the ones to watch, because the salt or sugar on them is why people eat six. Most plain ones are gluten free, though the flavoured sort is worth reading the packet for.")); __kws.extend(vec![("avoid".to_string(), __data133()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    fn __data134() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("how many"), py_str("how much"), py_str("in a bottle"), py_str("in a glass"), py_str("wine"), py_str("beer"), py_str("lager"), py_str("gin"), py_str("vodka"), py_str("whisky"), py_str("prosecco"), py_str("pint"), py_str("bottle"), py_str("safe"), py_str("limit"), py_str("week"), py_str("day"), py_str("drink"), py_str("drinking"), py_str("count"), py_str("calculate"), py_str("work out")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit-of-alcohol")); __args.push(py_tuple(vec![py_tuple(vec![py_str("unit of alcohol"), py_str("units of alcohol"), py_str("unit"), py_str("units"), py_str("alcohol unit"), py_str("alcohol units")]), __data134()])); __args.push(py_str("A unit is 10ml of pure alcohol, which is a small glass of wine at 12 per cent, half a pint of ordinary lager or a single 25ml spirit - so you work it out as strength times volume. The UK guideline is no more than 14 units a week for men and women, spread over three days or more, and a 13 per cent bottle of wine is about nine or ten units, so a bottle most nights is over it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("unit of alcohol"), py_str("units of alcohol"), py_str("alcohol unit"), py_str("units")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    fn __data135() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("mean"), py_str("means"), py_str("low"), py_str("high"), py_str("diet"), py_str("diabetes"), py_str("blood sugar"), py_str("lose weight"), py_str("weight"), py_str("food"), py_str("foods"), py_str("chart"), py_str("list"), py_str("use"), py_str("worth"), py_str("matter"), py_str("better")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("glycaemic-index")); __args.push(py_tuple(vec![py_tuple(vec![py_str("glycaemic index"), py_str("glycemic index"), py_str("gi"), py_str("low gi"), py_str("high gi"), py_str("glycaemic load")]), __data135()])); __args.push(py_str("The glycaemic index rates a food by how fast its carbohydrate raises blood sugar compared with pure glucose, from 0 to 100: a banana is about 50, white bread about 75, porridge oats about 55, lentils about 30. Low is 55 or under, and the trick is the company the carbohydrate keeps - fat, protein and fibre all slow it down, which is why a boiled potato beats a baked one and lentils barely move the number.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("glycaemic index"), py_str("glycemic index"), py_str("low gi"), py_str("high gi"), py_str("glycaemic load")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    fn __data136() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("mean"), py_str("means"), py_str("bad"), py_str("bad for you"), py_str("avoid"), py_str("cut out"), py_str("healthy"), py_str("unhealthy"), py_str("list"), py_str("food"), py_str("foods"), py_str("eat"), py_str("diet"), py_str("how much"), py_str("ok"), py_str("better"), py_str("cook"), py_str("homemade"), py_str("why")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ultra-processed-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ultra processed"), py_str("ultra-processed"), py_str("ultraprocessed"), py_str("upf"), py_str("processed food"), py_str("processed foods"), py_str("ultra processed food"), py_str("ultra processed foods")]), __data136()])); __args.push(py_str("Ultra-processed food is the industrial end of the scale: things made mostly from extracts and additives - emulsifiers, colourings, flavourings, sweeteners - that you would not cook with at home, like fizzy drinks, crisps, sausages, ready meals and most breakfast cereal. The evidence is that people eat more of it and put on weight, but the class is broad enough that hummus and wholemeal bread land in it, so judge by the ingredient list rather than the label. Cooking from plain ingredients is the practical answer.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("ultra processed"), py_str("ultra-processed"), py_str("upf"), py_str("processed food"), py_str("processed foods")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    fn __data137() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("mean"), py_str("means"), py_str("carbon"), py_str("climate"), py_str("better"), py_str("worse"), py_str("local"), py_str("buy local"), py_str("matter"), py_str("important"), py_str("reduce"), py_str("cut"), py_str("vegetarian"), py_str("meat"), py_str("imported"), py_str("seasonal"), py_str("kite")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-miles")); __args.push(py_tuple(vec![py_tuple(vec![py_str("food miles"), py_str("food mile"), py_str("air miles"), py_str("food miles of")]), __data137()])); __args.push(py_str("Food miles are the distance food travels from where it is grown to your plate, and they matter less than most people think: transport is around a tenth of the emissions in a typical shopping basket, while what the food is and how it was made is most of the rest. Air-freighted green beans and out-of-season asparagus are the exceptions worth avoiding; eating less meat and wasting less food cuts far more than buying only local.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("food miles"), py_str("food mile"), py_str("air miles")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nutrition._NAMED_TAKEAWAY"); }
    s__NAMED_TAKEAWAY(stdlib_call("re", "compile", &[py_str("\\b(?:takeaway|takeaways|take out|takeout|fast food)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data138() -> Py { py_tuple(vec![py_str("junk food"), py_str("junk"), py_str("fast food"), py_str("takeaway"), py_str("takeaways"), py_str("take out"), py_str("crisps"), py_str("chips"), py_str("sweets"), py_str("chocolate"), py_str("cakes"), py_str("biscuits"), py_str("fizzy drinks"), py_str("processed food"), py_str("ready meals")]) }
    fn __data139() -> Py { py_tuple(vec![py_str("stop"), py_str("stop eating"), py_str("cut"), py_str("cutting"), py_str("cut down"), py_str("reduce"), py_str("reducing"), py_str("give up"), py_str("quit"), py_str("less"), py_str("too much"), py_str("every day"), py_str("all the time"), py_str("habit")]) }
    fn __data140() -> Py { py_tuple(vec![py_str("email"), py_str("emails"), py_str("inbox"), py_str("spam"), py_str("mail"), py_str("mails"), py_str("letterbox"), py_str("letter"), py_str("letters"), py_str("post"), py_str("postman"), py_str("texts"), py_str("text message"), py_str("messages"), py_str("recipe"), py_str("recipes"), py_str("bake"), py_str("baking"), py_str("oven"), py_str("fryer"), py_str("frying"), py_str("air fryer"), py_str("how do i cook"), py_str("how long"), py_str("temperature"), py_str("homemade"), py_str("make"), py_str("making"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("child"), py_str("kids"), py_str("toddler"), py_str("baby"), py_str("drink"), py_str("drinking"), py_str("fizzy"), py_str("coke"), py_str("cola"), py_str("juice"), py_str("squash"), py_str("soda"), py_str("energy drink"), py_str("alcohol"), py_str("beer"), py_str("wine"), py_str("crisps"), py_str("chocolate"), py_str("sweets"), py_str("biscuits"), py_str("cake"), py_str("cakes"), py_str("spend"), py_str("spends"), py_str("spending"), py_str("spent"), py_str("money"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("pay"), py_str("paying"), py_str("paid"), py_str("budget"), py_str("waste"), py_str("wasting"), py_str("save"), py_str("saving"), py_str("£"), py_str("£s"), py_str("cost"), py_str("costs"), py_str("cheaper"), py_str("afford")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("junk-food")); __args.push(py_tuple(vec![__data138(), __data139()])); __args.push(py_str("Swap rather than ban, because the ban is what makes the 9pm trip to the shop happen: keep something you actually like at home that takes two minutes - frozen chips in the oven, popcorn, yoghurt with fruit, toast and peanut butter. Eat protein and fibre at lunch, because a day of white carbs is what makes the evening raid feel compulsory, and do not shop hungry: the aisle is the thing that decides it, not willpower. One takeaway a week, on a day you choose, holds better than none at all.")); __kws.extend(vec![("avoid".to_string(), __data140()), ("guard".to_string(), py_func("_not_a_named_takeaway", |args: &[Py]| _not_a_named_takeaway(args))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    fn __data141() -> Py { py_tuple(vec![py_str("break a fast"), py_str("break my fast"), py_str("break the fast"), py_str("breaking a fast"), py_str("breaking my fast"), py_str("broke my fast"), py_str("end a fast"), py_str("end my fast"), py_str("end the fast"), py_str("ending a fast"), py_str("finish a fast"), py_str("finished my fast"), py_str("come off a fast"), py_str("coming off a fast"), py_str("first meal after"), py_str("after a fast"), py_str("after fasting"), py_str("after my fast"), py_str("out of a fast")]) }
    fn __data142() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("should"), py_str("eat"), py_str("best"), py_str("safely"), py_str("properly"), py_str("start"), py_str("i"), py_str("my"), py_str("do"), py_str("can"), py_str("when")]) }
    fn __data143() -> Py { py_tuple(vec![py_str("break my arm"), py_str("break a leg"), py_str("broken bone"), py_str("break a bone"), py_str("break up"), py_str("breaking up"), py_str("broke up"), py_str("break the law"), py_str("break in"), py_str("breaking news"), py_str("break the ice"), py_str("break a window"), py_str("break a habit"), py_str("bad habit"), py_str("break the news"), py_str("lunch break"), py_str("tea break"), py_str("coffee break"), py_str("break even"), py_str("break down"), py_str("broken down"), py_str("fast car"), py_str("fastest"), py_str("how fast"), py_str("speed")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breaking-a-fast")); __args.push(py_tuple(vec![__data141(), __data142()])); __args.push(py_str("Start small and gentle: a piece of fruit, a yoghurt or a slice of toast, then wait twenty minutes before a proper meal. Fat, sugar and coffee on an empty stomach are what make people feel sick, shaky and headachy, so go easy on those and have a glass of water first. If you have gone more than a day without eating, break it over an hour or two rather than in one sitting, and ask a GP first if you take diabetes medication.")); __kws.extend(vec![("avoid".to_string(), __data143()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
}
pub fn _frozen_and_tinned_reply(args: &[Py]) -> Py {
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
    let mut fruit: Py = Py::None;
    let mut price: Py = Py::None;
    let mut said: Py = Py::None;
    let mut smoothie: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Fresh against frozen, asked as health, as money, as a smoothie, or of fruit.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    smoothie = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("smoothie"), py_str("smoothies"), py_str("blender"), py_str("shake")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    price = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" cheap"), py_str(" cheaper"), py_str(" cheapest"), py_str(" cost"), py_str(" costs"), py_str(" price"), py_str(" prices"), py_str(" budget"), py_str(" dearer"), py_str(" value")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    fruit = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" fruit"), py_str(" fruits"), py_str(" peach"), py_str(" peaches"), py_str(" mango"), py_str(" pineapple"), py_str(" pears"), py_str(" cherries")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("tinned"), py_str("tin of"), py_str("canned"), py_str("can of"), py_str(" from a tin")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&smoothie.clone()) {
    return py_str("Frozen berries and frozen fruit are the better buy for a smoothie: they are picked ripe and frozen within hours, so they keep more vitamin C than soft fruit that has sat in a punnet, and frozen chunks chill the drink and thicken it without ice. Tip them in from frozen with the yoghurt or milk and blend - no need to thaw, and no added sugar, unlike the fruit in syrup.");
    }
    if py_truthy(&price.clone()) {
    return py_str("Almost always yes: frozen vegetables cost less than fresh for the same weight, because nothing has to be sorted, shipped soft or thrown away, and tinned beans, chickpeas and tomatoes are cheaper still. Frozen also wastes nothing, since you pour out what you need and the bag goes back. The one thing worth paying for fresh is a vegetable you are going to eat raw, where the texture is the point.");
    }
    if py_truthy(&fruit.clone()) {
    return py_str("Tinned fruit counts, with one catch: it is picked ripe and tinned within hours, so the vitamins survive, but fruit in syrup is a pudding rather than a portion - a couple of tinned peach halves in syrup carry about 15g of sugar. Buy the ones in juice, or rinse the syrup off, and tinned fruit does the same job as fresh. Frozen berries are the better buy of the two, since nothing is added to them at all.");
    }
    return py_str("Frozen and tinned are as good as fresh, and often better: vegetables are frozen within hours of picking, so their vitamins outlast a cabbage that has sat in the fridge all week, and tinned tomatoes are better than the pale ones flown in out of season. Frozen peas, sweetcorn, spinach and berries are the best buys, and tinned beans, chickpeas and sardines are cheap protein with nothing added. The tin or the bag is not the problem - read the label: brine, syrup, oil and sauces are where the salt and sugar come from. Frozen fish is fine too, thawed overnight in the fridge and cooked from there.");
    Py::None
}
pub fn entry(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut id: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "id") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: id"))) };
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut avoid: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "avoid") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bias: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bias") { v.clone() }
    else { Py::Int(0) };
    let mut ask: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_tuple(vec![]) };
    let mut slips: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "slips") { v.clone() }
    else { py_tuple(vec![]) };
    let mut keep: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keep") { v.clone() }
    else { py_tuple(vec![]) };
    let mut guard: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "guard") { v.clone() }
    else { Py::None };
    let mut guard_words: Py = Py::None;
    let mut inner: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A nutrition entry. Every one of them passes on a children's or a pet line.\n\n    A child's portions and a dog's food are different answers, and both are asked in the same\n    words (\"how much should I feed my son\", \"can my dog eat grapes\"), so the guard sits here\n    rather than in forty separate `avoid` lists. `keep` names the guard words an entry answers\n    anyway: \"how do I get my kids to eat veg\" is a question for exactly that entry.\n\n    The pack date is the second guard, for the same reason: \"is it ok to eat bread past the date\"\n    is a question about whether the loaf is still edible, and the fibre row answered it with \"two\n    slices is about 6g of fibre\". What is in a food is not what has happened to it, so every row\n    here stands down on the words that mean the day printed on the pack (`PACK_DATE`) and on the\n    line that says the food has gone off at all (`food_has_gone_off`): \"is my chicken still ok to\n    eat\" was answered with what is in a chicken.\n    ");
    if py_truthy(&Py::Bool(py_is(&guard.clone(), &Py::None))) {
    guard = { let ask = ask.clone(); let avoid = avoid.clone(); let bias = bias.clone(); let guard = guard.clone(); let guard_words = guard_words.clone(); let id = id.clone(); let inner = inner.clone(); let keep = keep.clone(); let key = key.clone(); let reply = reply.clone(); let slips = slips.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let ask = ask.clone(); let avoid = avoid.clone(); let bias = bias.clone(); let guard = guard.clone(); let guard_words = guard_words.clone(); let id = id.clone(); let inner = inner.clone(); let keep = keep.clone(); let key = key.clone(); let reply = reply.clone(); let slips = slips.clone(); let word = word.clone(); let read: Py = __kwp(args, 0, "read").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "food_has_gone_off", &[read.clone()]))) }) };
    } else {
    inner = guard.clone();
    guard = { let ask = ask.clone(); let avoid = avoid.clone(); let bias = bias.clone(); let guard = guard.clone(); let guard_words = guard_words.clone(); let id = id.clone(); let inner = inner.clone(); let keep = keep.clone(); let key = key.clone(); let reply = reply.clone(); let slips = slips.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let ask = ask.clone(); let avoid = avoid.clone(); let bias = bias.clone(); let guard = guard.clone(); let guard_words = guard_words.clone(); let id = id.clone(); let inner = inner.clone(); let keep = keep.clone(); let key = key.clone(); let reply = reply.clone(); let slips = slips.clone(); let word = word.clone(); let read: Py = __kwp(args, 0, "read").unwrap_or(Py::None); { let __b = py_call(&inner, &[read.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "food_has_gone_off", &[read.clone()]))) } } }) };
    }
    guard_words = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&g__NOT_A_PERSON_OR_PET(), &g__A_STATEMENT_ABOUT_ME()), &g__PACK_DATE())) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&keep.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    avoid = op_add(&call_builtin("tuple", &[avoid.clone()]).unwrap_or_else(|e| exc::raise(e)), &guard_words.clone());
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(id.clone()); __args.push(key.clone()); __args.push(reply.clone()); __kws.extend(vec![("avoid".to_string(), avoid.clone()), ("bias".to_string(), bias.clone()), ("ask".to_string(), ask.clone()), ("slips".to_string(), slips.clone()), ("guard".to_string(), guard.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) };
    Py::None
}
pub fn not_a_dose(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line times a dose rather than a session or a meal.\n\n    Public because the same shape reaches other timing rows: the subject of \"before or after what\"\n    is the session, the dose or the journey, and a row answers only its own subject.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return Py::Bool(!py_truthy(&mcall(&g__A_DOSE(), "search", &[said.clone()])));
    Py::None
}
pub fn _a_word_in(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whole words only: \" my boy \" would otherwise be found inside \" my boyfriend \", and a\n    partner would be answered as a child.");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "startswith", &[py_str(" ")]); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str(" ")]) } }) {
    if py_truthy(&Py::Bool(py_contains(&line.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&word.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_child_in(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A child the line names: a son or a 4 year old, but not a 40 year old husband.");
    let __iter1 = py_iter_or_raise(&g__A_CHILD());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__AGE_WORDS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&_a_word_in(&[line.clone(), py_tuple(vec![word.clone()])])) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&mcall(&g__CHILD_AGE(), "finditer", &[line.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&op_lt(&call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(18))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _who_in_line(args: &[Py]) -> Py {
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
    let mut a_child: Py = Py::None;
    let _ = py_str("Who a line says the refusal is about: \"me\", \"grown-up\", \"child\", or None.\n\n    The order is the whole job. The speaker first, because \"i wont eat them\" is a person asking\n    about their own plate. Then an adult the line names, but only when the line has not named a\n    child in the same breath. Only then a child, and \"fussy\"/\"picky\" count as a child only\n    because an adult was not named: \"my husband is a fussy eater\" is a grown-up.\n    ");
    if py_truthy(&_a_word_in(&[line.clone(), g__ME_ASKING()])) {
    return py_str("me");
    }
    a_child = _a_child_in(&[line.clone()]);
    if py_truthy(&{ let __b = _a_word_in(&[line.clone(), g__A_GROWN_UP()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_child.clone())) } }) {
    return py_str("grown-up");
    }
    if py_truthy(&{ let __b = a_child.clone(); if py_truthy(&__b) { __b } else { _a_word_in(&[line.clone(), g__FUSSY()]) } }) {
    return py_str("child");
    }
    return Py::None;
    Py::None
}
pub fn _who_this_is_about(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said_now: Py = Py::None;
    let mut somebody_else: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Who the refusal is about, reading back through the conversation when the line says nobody.\n\n    \"i want to like them\" and \"what can i do\" name no person at all, so they were answered as a\n    child on the strength of the word \"veg\"; the person was in the turn before, and that is where\n    they are read from: \"i wont eat vegetables\" / \"i want to like them\" is the speaker, and\n    \"my husband is a fussy eater\" / \"he wont touch veg\" is the husband. A pronoun for somebody\n    else stops the read-back, because then the line is about them and the earlier speaker is the\n    wrong answer. With nothing to go on the answer is None and the caller asks rather than\n    assuming a child.\n    ");
    line = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    who = _who_in_line(&[line.clone()]);
    if py_truthy(&who.clone()) {
    return who.clone();
    }
    somebody_else = _a_word_in(&[line.clone(), g__SOMEONE_ELSE()]);
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said_now = mcall(&line.clone(), "strip", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = op_add(&op_add(&py_str(" "), &mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&text.clone(), "strip", &[]), &said_now.clone())) } }) {
    continue;
    }
    who = _who_in_line(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&who.clone(), &py_str("me"))); if !py_truthy(&__b) { __b } else { somebody_else.clone() } }) {
    return Py::None;
    }
    if py_truthy(&who.clone()) {
    return who.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _veg_reply(args: &[Py]) -> Py {
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
    let mut who: Py = Py::None;
    let _ = py_str("Five a day for the person asking, and a different answer for the person they name: the\n    adult advice (add a handful to what you cook) is not what helps a child who refuses the\n    stuff, and a grown-up who refuses it is not a child either.\n\n    The person is read from the line first and from the conversation after it, and a line that\n    says a pronoun and nothing anywhere else gets asked who it is: a child is named (\"my son\",\n    \"my 4 year old\") and an adult is named, so \"he\" on its own is nobody to guess at.\n    ");
    who = _who_this_is_about(&[memory.clone(), understood.clone()]);
    if py_truthy(&Py::Bool(py_eq(&who.clone(), &py_str("me")))) {
    return g__VEG_FIVE_A_DAY();
    }
    if py_truthy(&Py::Bool(py_eq(&who.clone(), &py_str("grown-up")))) {
    return g__VEG_A_GROWN_UP();
    }
    if py_truthy(&Py::Bool(py_eq(&who.clone(), &py_str("child")))) {
    return g__VEG_A_CHILD();
    }
    return g__VEG_WHO_IS_IT();
    Py::None
}
pub fn _the_bowl_named(args: &[Py]) -> Py {
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
    let mut bowls: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut spots: Py = Py::None;
    let mut verdict: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every bowl the line names, in the order the words appear in it.");
    line = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g__CEREAL_BOWLS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    verdict = __unpk2[1].clone();
    spots = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&line.clone(), &op_add(&py_str(" "), &name.clone())))) {  __out.push(mcall(&line.clone(), "find", &[op_add(&py_str(" "), &name.clone())])); } } py_list(__out) })() };
    if py_truthy(&spots.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_list(vec![call_builtin("min", &[spots.clone()]).unwrap_or_else(|e| exc::raise(e)), geti(&names.clone(), &Py::Int(0)), verdict.clone()])]);
    }
    }
    spots = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("porridge"), py_str("oat"), py_str("oats"), py_str("oatmeal")])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&line.clone(), &op_add(&py_str(" "), &word.clone())))) {  __out.push(mcall(&line.clone(), "find", &[op_add(&py_str(" "), &word.clone())])); } } py_list(__out) })() };
    if py_truthy(&spots.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_list(vec![call_builtin("min", &[spots.clone()]).unwrap_or_else(|e| exc::raise(e)), py_str("porridge"), g_PORRIDGE_VERDICT()])]);
    }
    let _ = mcall(&found.clone(), "sort", &[]);
    let __unpk3 = py_unpack(&py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), py_list(vec![])]), 2, None);
    seen = __unpk3[0].clone();
    bowls = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&found.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 3, None);
    at = __unpk5[0].clone();
    name = __unpk5[1].clone();
    verdict = __unpk5[2].clone();
    if py_truthy(&Py::Bool(py_contains(&seen.clone(), &name.clone()))) {
    continue;
    }
    let _ = mcall(&seen.clone(), "add", &[name.clone()]);
    let _ = mcall(&bowls.clone(), "append", &[py_tuple(vec![at.clone(), name.clone(), verdict.clone()])]);
    }
    return bowls.clone();
    Py::None
}
pub fn _cereal_verdict(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut named: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("The verdict on the bowl the person named, and the comparison only when they named two.");
    named = _the_bowl_named(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return g_PORRIDGE_VERDICT();
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&geti(&named.clone(), &Py::Int(0)), &Py::Int(2));
    }
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&named.clone(), &Py::Int(0)), geti(&named.clone(), &Py::Int(1))]), 2, None);
    first = __unpk1[0].clone();
    second = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&py_set(vec![geti(&first.clone(), &Py::Int(1)), geti(&second.clone(), &Py::Int(1))]), &py_set(vec![py_str("porridge"), py_str("granola")])))) {
    return py_str("Porridge is the better everyday bowl of the two: oats and water with no added sugar, about 4g of fibre in 40g, and it keeps you full. Granola is the same oats with oil and sugar baked on - 11g to 15g of sugar in a 50g serving - so it is a pudding's worth of sugar with a good fibre story. If you like the crunch, granola on top of plain yoghurt, 20g rather than 50g, is the compromise.");
    }
    if py_truthy(&Py::Bool(py_eq(&py_set(vec![geti(&first.clone(), &Py::Int(1)), geti(&second.clone(), &Py::Int(1))]), &py_set(vec![py_str("porridge"), py_str("muesli")])))) {
    return py_str("Porridge is the plainer of the two and has no added sugar; muesli brings nuts and dried fruit, so it has more fibre and more sugar - 7g to 12g in 50g. Either is a good breakfast; muesli needs measuring, porridge needs nothing but oats.");
    }
    return op_add(&op_add(&mcall(&g__CEREAL_SHORT(), "get", &[geti(&first.clone(), &Py::Int(1)), op_add(&geti(&mcall(&geti(&first.clone(), &Py::Int(2)), "split", &[py_str(".")]), &Py::Int(0)), &py_str("."))]), &py_str(" ")), &mcall(&g__CEREAL_SHORT(), "get", &[geti(&second.clone(), &Py::Int(1)), op_add(&geti(&mcall(&geti(&second.clone(), &Py::Int(2)), "split", &[py_str(".")]), &Py::Int(0)), &py_str("."))]));
    Py::None
}
pub fn _a_day(args: &[Py]) -> Py {
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
    let mut FOOD_NUTRIENTS: Py = Py::None;
    let mut _nutrient_food: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut name: Py = Py::None;
    let mut nutrient: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The daily figure for the nutrient the line names: the woman's number or the teenager's\n    where it differs, the adult's otherwise.\n\n    A teenager is a person whose numbers are different rather than a child, which is why this one\n    entry carries two replies instead of handing the line to the children's guard, and the iron\n    figure is the one that also turns on the sex of the person asking.\n    ");
    clean = op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[])), &py_str(" "));
    name = Py::None;
    let __iter1 = py_iter_or_raise(&g__A_DAY_WORDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    nutrient = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&clean.clone(), &word.clone()))) {
    name = nutrient.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    FOOD_NUTRIENTS = pyrt::xrefs::xget("knowledge.cooking", "FOOD_NUTRIENTS");
    _nutrient_food = py_func("_nutrient_food", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_nutrient_food", args));
    if py_truthy(&Py::Bool(!py_is(&py_call(&_nutrient_food, &[clean.clone(), FOOD_NUTRIENTS.clone()]), &Py::None))) {
    return Py::None;
    }
    figures = geti(&g__A_DAY(), &name.clone());
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_DAY_TEEN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&figures.clone(), &py_str("teen"))) } }) {
    return geti(&figures.clone(), &py_str("teen"));
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_DAY_WOMAN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&figures.clone(), &py_str("woman"))) } }) {
    return geti(&figures.clone(), &py_str("woman"));
    }
    return geti(&figures.clone(), &py_str("adult"));
    Py::None
}
pub fn _pregnancy_vitamins(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The supplement answer for a pregnancy, which is not the adult figure.\n\n    A pregnant person is the one group whose numbers are not the adult table: folic acid is\n    double for the first 12 weeks, vitamin D is wanted all year rather than in winter, and an\n    iron supplement is not something to start on a guess. The nutrition guard would otherwise\n    send every one of these lines to the conversation lane as a statement about the speaker.\n    ");
    line = op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("vitamin a"), py_str("liver"), py_str("cod liver"), py_str("retinol")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Skip vitamin A supplements and liver while you are pregnant: too much of it is harmful to the baby, and it is the one vitamin people take by accident, in a multivitamin or a cod liver oil. An ordinary multivitamin is not the one to take - a pregnancy one leaves the vitamin A out.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("folic"), py_str("folate")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Take 400 micrograms of folic acid every day from before you conceive until the end of week 12, which is double the ordinary 200: it cuts the risk of spina bifida, and food alone will not get you there. After 12 weeks, and while breastfeeding, the ordinary 200 is enough.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("vitamin d"), py_str("vit d")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Take 10 micrograms (400 IU) of vitamin D every day, all year, while you are pregnant and while you breastfeed. The figure is the same as everyone else's, but you cannot count on the sun or on food, so it comes from a supplement.");
    }
    if py_truthy(&Py::Bool(py_contains(&line.clone(), &py_str("iron")))) {
    return py_str("You do not need an iron supplement unless a blood test shows you are low: your blood count is checked at your booking appointment and again at about 28 weeks, and low iron is treated then. Vitamin C with a meal helps you take up the iron in food, and tea or coffee with it works against you.");
    }
    if py_truthy(&Py::Bool(py_contains(&line.clone(), &py_str("calcium")))) {
    return py_str("You need the same 700mg of calcium a day as anyone else - a pint of milk, or a pot of yoghurt and a matchbox of cheese. The baby takes what it needs from your bones whatever you eat, so the dairy is for your bones, not theirs, and a supplement is only for a low a blood test has found.");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("b12"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("vegan"))) } }) {
    return py_str("Vitamin B12 matters in pregnancy and breastfeeding, and it comes from meat, fish, milk and eggs: if you are vegan, take a B12 supplement or a fortified milk, because the baby's supply comes from yours and the stores run down quietly.");
    }
    return py_str("Folic acid, 400 micrograms a day until the end of week 12, and vitamin D, 10 micrograms a day all year, are the two worth taking in pregnancy. Leave out vitamin A and liver, and take iron only for a low a blood test has found.");
    Py::None
}
pub fn _more_of(args: &[Py]) -> Py {
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
    let mut FOOD_NUTRIENTS: Py = Py::None;
    let mut _nutrient_food: Py = Py::None;
    let mut line: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Where to get more of the nutrient this line names, or None when it names none.");
    line = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    FOOD_NUTRIENTS = pyrt::xrefs::xget("knowledge.cooking", "FOOD_NUTRIENTS");
    _nutrient_food = py_func("_nutrient_food", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_nutrient_food", args));
    if py_truthy(&Py::Bool(!py_is(&py_call(&_nutrient_food, &[line.clone(), FOOD_NUTRIENTS.clone()]), &Py::None))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_MORE_OF());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    text = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { mcall(&mcall(&line.clone(), "strip", &[]), "startswith", &[op_add(&word.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return text.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _low_iron(args: &[Py]) -> Py {
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
    let _ = py_str("A person who thinks their iron is low wants a blood test before a shopping list.");
    return py_str("That is worth a blood test rather than a guess: tiredness, breathlessness on the stairs, pallor, brittle nails and hair loss are the usual signs, and your GP can check a full blood count in a few days. Do not start iron tablets on suspicion - too much iron is harmful - but do eat iron-rich food meanwhile: red meat, lentils, beans, fortified cereal and dark green veg, with vitamin C in the same meal and tea or coffee kept an hour away.");
    Py::None
}
pub fn _nutrient_role(args: &[Py]) -> Py {
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
    let mut FOOD_NUTRIENTS: Py = Py::None;
    let mut _nutrient_food: Py = Py::None;
    let mut line: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What the nutrient this line names does, or None when it names none.");
    line = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    FOOD_NUTRIENTS = pyrt::xrefs::xget("knowledge.cooking", "FOOD_NUTRIENTS");
    _nutrient_food = py_func("_nutrient_food", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_nutrient_food", args));
    if py_truthy(&Py::Bool(!py_is(&py_call(&_nutrient_food, &[line.clone(), FOOD_NUTRIENTS.clone()]), &Py::None))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_NUTRIENT_ROLE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    text = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { mcall(&mcall(&line.clone(), "strip", &[]), "startswith", &[op_add(&word.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return text.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_bare_word(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when one of `words` is said on its own, at either end of the line as well as inside.\n\n    `\" ok \"` alone missed the commonest way this is asked: \"is milk ok\" ends with the word, and\n    an \"ok\" at the very front is the same filler (\"ok is milk fine\"). Matching the ends is what\n    keeps that line's fridge check; without it the row answered the bare \"is milk ok\" with the\n    paragraph about milk being good for you, which does not answer it.\n    ");
    said = op_add(&op_add(&py_str(" "), &mcall(&said.clone(), "strip", &[])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" "); py_string(__s) })); if py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "startswith", &[{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" "); py_string(__s) }]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&said.clone(), "rstrip", &[]), &{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); py_string(__s) })) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); py_string(__s) }])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_plant_milk(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_about_it: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The carton the line named, or the ranking when it asks which milk to take.");
    said = op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MILK_FOR_A_CHILD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&py_str(" "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__THE_PLANET()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[op_add(&py_str(" "), &word.clone())]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str("s ")))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__MILK_AND_THE_PLANET();
    }
    named = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g__PLANT_MILKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    said_about_it = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[op_add(&py_str(" "), &word.clone())]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str("s ")))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&named.clone(), "append", &[py_tuple(vec![geti(&words.clone(), &Py::Int(0)), said_about_it.clone()])]);
    }
    }
    kept = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&named.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    name = __unpk4[0].clone();
    text = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut other: Py = __unpk1[0].clone(); let mut _u: Py = __unpk1[1].clone(); __out.push({ let __b = Py::Bool(!py_eq(&name.clone(), &other.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&other.clone(), &name.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    let _ = mcall(&kept.clone(), "append", &[py_tuple(vec![name.clone(), text.clone()])]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&geti(&kept.clone(), &Py::Int(0)), &Py::Int(1));
    }
    if py_truthy(&op_gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return op_add(&op_add(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&kept.clone(), &Py::None, &Py::Int(2), &Py::None)) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk1[0].clone(); let mut text: Py = __unpk1[1].clone(); __out.push(text.clone()); } py_list(__out) })() }]), &py_str(" ")), &py_str("If it is protein you are after, take the one with more of it; if it is the cup of tea or the coffee, take the one that does not split."));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WHICH_MILK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__THE_MILK_RANKING();
    }
    if py_truthy(&{ let __b = _a_bare_word(&[said.clone(), g__A_BARE_OK_ASK()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOURISHING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return op_add(&op_add(&g__COW_MILK_REPLY(), &py_str(" ")), &g__THE_FRIDGE_CHECK());
    }
    return g__COW_MILK_REPLY();
    Py::None
}
pub fn _coffee_or_tea(args: &[Py]) -> Py {
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
    let mut coffee: Py = Py::None;
    let mut line: Py = Py::None;
    let mut tea: Py = Py::None;
    let _ = py_str("A verdict on the drink the line named. \"is tea good for you\" was the coffee answer, word\n    for word, because one row covers both drinks and the reply opened \"Coffee is not the\n    villain it was\" - a sentence about the wrong cup.");
    line = op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])), &py_str(" "));
    tea = { let __b = Py::Bool(py_contains(&line.clone(), &py_str(" tea"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("teas "))) } };
    coffee = { let __b = Py::Bool(py_contains(&line.clone(), &py_str("coffee"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &py_str("espresso"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("latte"))) } } } };
    if py_truthy(&{ let __b = tea.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&coffee.clone())) } }) {
    return py_str("Tea is fine: four or five cups of ordinary black tea is about half the caffeine of the same number of coffees, it counts towards your fluid, and the tea itself has no calories - the sugar and the biscuits are what add up. Herbal and green teas are caffeine-free or nearly, so they are not the ones to worry about in the evening. If you are anaemic or take iron tablets, keep the cup an hour away from a meal, because the tannins cut how much iron you take up.");
    }
    return py_str("Coffee is not the villain it was: up to about four cups a day is linked to no harm for most adults, and it may be mildly protective for the heart and liver. The trouble is what goes in it (a latte is 120 calories) and drinking it late enough to spoil your sleep. Filter coffee raises cholesterol a little more than espresso, which is why paper filters are still worth using.");
    Py::None
}
pub fn _not_a_keeping_question(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is a which-is-better question rather than a keeping one.");
    return Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "a_food_is_being_kept", &[understood.clone()])));
    Py::None
}
pub fn _wholegrain(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut pasta: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut rice: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The wholemeal swap for the grain the line names: rice, or pasta.");
    said = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }])), &py_str(" "));
    pasta = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" pasta"), py_str(" spaghetti"), py_str(" macaroni"), py_str(" noodles"), py_str(" penne"), py_str(" fusilli"), py_str(" tagliatelle")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    rice = Py::Bool(py_contains(&said.clone(), &py_str(" rice")));
    head = if py_truthy(&{ let __b = pasta.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&rice.clone())) } }) { py_str("Wholewheat pasta keeps the bran and the germ, so it has about three times the fibre of white") } else { py_str("Brown rice keeps the bran and the germ, so it has about three times the fibre of white") };
    rest = py_str("White rice and pasta are not bad - they just give you less fibre and less staying power. Both keep their calories, around 130 to 150 per 100g cooked, so the swap is about fibre and fullness, not about weight.");
    if py_truthy(&{ let __b = pasta.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&rice.clone())) } }) {
    rest = py_str("White pasta is not bad - it just gives you less fibre and less staying power. It keeps its calories, around 130 to 150 per 100g cooked, so the swap is about fibre and fullness, not about weight. Wholewheat takes a minute or two longer and holds sauce well; the half-and-half bag is an easy start if the family notices the change.");
    return op_add(&op_add(&head.clone(), &py_str(", and it has a slower rise in blood sugar too. ")), &rest.clone());
    }
    return op_add(&op_add(&head.clone(), &py_str(" and a slower rise in blood sugar; wholewheat pasta is the same idea against white pasta. ")), &rest.clone());
    Py::None
}
pub fn not_a_cooking_question(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line is about a food rather than about how it is cooked.\n\n    False as soon as the line says how hot, how long, or how it is done - \"what temperature should\n    i cook pork at\", \"what gas mark for a leg of lamb\", \"when is the turkey done\", \"what temp for a\n    medium rare steak\" - whatever the food is called and however the question is worded. True for\n    the questions the allowance answers: \"is red meat bad for you\", \"how much red meat should i\n    eat\", \"how often can i eat steak\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    clean = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clean.clone()) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&op_modulo(&py_str("%s %s"), &py_tuple(vec![{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, clean.clone()])), "lower", &[]), "split", &[])]);
    if py_truthy(&mcall(&g_COOKING_VERBS(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g__A_THERMOMETER(), "search", &[said.clone()])));
    Py::None
}
pub fn _sugary_drink_reply(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    text = op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[])), &py_str(" "));
    named = py_str("");
    let __iter1 = py_iter_or_raise(&g__SWEET_DRINKS());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))) {
    named = word.clone();
    break;
    }
    }
    tail = py_str("The sugar is the whole problem: it comes as liquid, so it does not fill you up, and it is what dentists see. Diet and zero versions cut the sugar and are the sensible swap rather than a healthy drink; water, or fizzy water with a slice of lemon, is the free one.");
    if py_truthy(&named.clone()) {
    return op_modulo(&py_str("A glass of %s is about as sugary as cola - roughly 35g in a 330ml glass, seven teaspoons - and an adult should stay under 30g a day in total. %s"), &py_tuple(vec![named.clone(), tail.clone()]));
    }
    return op_add(&py_str("A 330ml can of cola has about 35g of sugar, seven teaspoons, and lemonade, squash and the rest are in the same range - an adult should stay under 30g a day in total. "), &tail.clone());
    Py::None
}
pub fn _not_a_verdict_on_what_is_going_off(args: &[Py]) -> Py {
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
    let mut leans: Py = Py::None;
    let mut past: Py = Py::None;
    let _ = py_str("False when the food the line is about is going off: that is a keeping question.\n\n    A guard says whether the row may answer, so this says no to exactly that case and yes to\n    every other.\n    ");
    understood = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g__GOING_OFF(), "search", &[clean.clone()])) {
    if py_truthy(&{ let __b = mcall(&g__A_STATE_NOT_SPOILAGE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_VERDICT_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__EDIBILITY_ASK(), "search", &[clean.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    }
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    past = mcall(&call_builtin("str", &[{ let __b = mcall(&leans.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&g__GOING_OFF(), "search", &[past.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__EDIBILITY_ASK(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _not_a_named_takeaway(args: &[Py]) -> Py {
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
    let _ = py_str("False for a line whose only fast food is a takeaway, which the takeaway card answers.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__NAMED_TAKEAWAY(), "search", &[clean.clone()])))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bjunk\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.nutrition".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "COOKING_VERBS" => Some(g_COOKING_VERBS()),
            "DAY_WORDS" => Some(g_DAY_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FOOD_HEALTH" => Some(g_FOOD_HEALTH()),
            "HEALTH_AVOID" => Some(g_HEALTH_AVOID()),
            "HEALTH_WORDS" => Some(g_HEALTH_WORDS()),
            "MORE_OF" => Some(g_MORE_OF()),
            "NUTRIENT_ROLE" => Some(g_NUTRIENT_ROLE()),
            "PACK_DATE" => Some(g_PACK_DATE()),
            "PORRIDGE_VERDICT" => Some(g_PORRIDGE_VERDICT()),
            "_AGE_WORDS" => Some(g__AGE_WORDS()),
            "_A_BARE_OK_ASK" => Some(g__A_BARE_OK_ASK()),
            "_A_CHILD" => Some(g__A_CHILD()),
            "_A_DAY" => Some(g__A_DAY()),
            "_A_DAY_TEEN" => Some(g__A_DAY_TEEN()),
            "_A_DAY_WOMAN" => Some(g__A_DAY_WOMAN()),
            "_A_DAY_WORDS" => Some(g__A_DAY_WORDS()),
            "_A_DOSE" => Some(g__A_DOSE()),
            "_A_GROWN_UP" => Some(g__A_GROWN_UP()),
            "_A_STATEMENT_ABOUT_ME" => Some(g__A_STATEMENT_ABOUT_ME()),
            "_A_STATE_NOT_SPOILAGE" => Some(g__A_STATE_NOT_SPOILAGE()),
            "_A_THERMOMETER" => Some(g__A_THERMOMETER()),
            "_A_VERDICT_ASK" => Some(g__A_VERDICT_ASK()),
            "_CEREAL_BOWLS" => Some(g__CEREAL_BOWLS()),
            "_CEREAL_SHORT" => Some(g__CEREAL_SHORT()),
            "_CHILD_AGE" => Some(g__CHILD_AGE()),
            "_COW_MILK_REPLY" => Some(g__COW_MILK_REPLY()),
            "_DISLIKE" => Some(g__DISLIKE()),
            "_EAT_BETTER" => Some(g__EAT_BETTER()),
            "_EDIBILITY_ASK" => Some(g__EDIBILITY_ASK()),
            "_FUSSY" => Some(g__FUSSY()),
            "_GOING_OFF" => Some(g__GOING_OFF()),
            "_ME_ASKING" => Some(g__ME_ASKING()),
            "_MILK_AND_THE_PLANET" => Some(g__MILK_AND_THE_PLANET()),
            "_MILK_FOR_A_CHILD" => Some(g__MILK_FOR_A_CHILD()),
            "_MORE_OF_ASK" => Some(g__MORE_OF_ASK()),
            "_NAMED_TAKEAWAY" => Some(g__NAMED_TAKEAWAY()),
            "_NOT_A_PERSON_OR_PET" => Some(g__NOT_A_PERSON_OR_PET()),
            "_NOURISHING" => Some(g__NOURISHING()),
            "_PACK_DATE" => Some(g__PACK_DATE()),
            "_PLANT_MILKS" => Some(g__PLANT_MILKS()),
            "_PREGNANT" => Some(g__PREGNANT()),
            "_ROLE_ASK" => Some(g__ROLE_ASK()),
            "_SOMEONE_ELSE" => Some(g__SOMEONE_ELSE()),
            "_SWEET_DRINKS" => Some(g__SWEET_DRINKS()),
            "_THE_FRIDGE_CHECK" => Some(g__THE_FRIDGE_CHECK()),
            "_THE_MILK_RANKING" => Some(g__THE_MILK_RANKING()),
            "_THE_PLANET" => Some(g__THE_PLANET()),
            "_VEG_A_CHILD" => Some(g__VEG_A_CHILD()),
            "_VEG_A_GROWN_UP" => Some(g__VEG_A_GROWN_UP()),
            "_VEG_FIVE_A_DAY" => Some(g__VEG_FIVE_A_DAY()),
            "_VEG_WHO_IS_IT" => Some(g__VEG_WHO_IS_IT()),
            "_WHICH_MILK" => Some(g__WHICH_MILK()),
            "_a_bare_word" => Some(py_func("_a_bare_word", |args: &[Py]| _a_bare_word(args))),
            "_a_child_in" => Some(py_func("_a_child_in", |args: &[Py]| _a_child_in(args))),
            "_a_day" => Some(py_func("_a_day", |args: &[Py]| _a_day(args))),
            "_a_plant_milk" => Some(py_func("_a_plant_milk", |args: &[Py]| _a_plant_milk(args))),
            "_a_word_in" => Some(py_func("_a_word_in", |args: &[Py]| _a_word_in(args))),
            "_cereal_verdict" => Some(py_func("_cereal_verdict", |args: &[Py]| _cereal_verdict(args))),
            "_coffee_or_tea" => Some(py_func("_coffee_or_tea", |args: &[Py]| _coffee_or_tea(args))),
            "_food_avoid" => Some(g__food_avoid()),
            "_food_id" => Some(g__food_id()),
            "_food_line" => Some(g__food_line()),
            "_food_names" => Some(g__food_names()),
            "_frozen_and_tinned_reply" => Some(py_func("_frozen_and_tinned_reply", |args: &[Py]| _frozen_and_tinned_reply(args))),
            "_low_iron" => Some(py_func("_low_iron", |args: &[Py]| _low_iron(args))),
            "_more_of" => Some(py_func("_more_of", |args: &[Py]| _more_of(args))),
            "_not_a_keeping_question" => Some(py_func("_not_a_keeping_question", |args: &[Py]| _not_a_keeping_question(args))),
            "_not_a_named_takeaway" => Some(py_func("_not_a_named_takeaway", |args: &[Py]| _not_a_named_takeaway(args))),
            "_not_a_verdict_on_what_is_going_off" => Some(py_func("_not_a_verdict_on_what_is_going_off", |args: &[Py]| _not_a_verdict_on_what_is_going_off(args))),
            "_nutrient_role" => Some(py_func("_nutrient_role", |args: &[Py]| _nutrient_role(args))),
            "_pregnancy_vitamins" => Some(py_func("_pregnancy_vitamins", |args: &[Py]| _pregnancy_vitamins(args))),
            "_raw_entry" => Some(g__raw_entry()),
            "_sugary_drink_reply" => Some(py_func("_sugary_drink_reply", |args: &[Py]| _sugary_drink_reply(args))),
            "_the_bowl_named" => Some(py_func("_the_bowl_named", |args: &[Py]| _the_bowl_named(args))),
            "_veg_reply" => Some(py_func("_veg_reply", |args: &[Py]| _veg_reply(args))),
            "_who_in_line" => Some(py_func("_who_in_line", |args: &[Py]| _who_in_line(args))),
            "_who_this_is_about" => Some(py_func("_who_this_is_about", |args: &[Py]| _who_this_is_about(args))),
            "_wholegrain" => Some(py_func("_wholegrain", |args: &[Py]| _wholegrain(args))),
            "a_food_is_being_kept" => Some(g_a_food_is_being_kept()),
            "entry" => Some(py_func("entry", |args: &[Py]| entry(args))),
            "food_has_gone_off" => Some(g_food_has_gone_off()),
            "not_a_cooking_question" => Some(py_func("not_a_cooking_question", |args: &[Py]| not_a_cooking_question(args))),
            "not_a_count_for_the_box_or_the_dish" => Some(g_not_a_count_for_the_box_or_the_dish()),
            "not_a_dose" => Some(py_func("not_a_dose", |args: &[Py]| not_a_dose(args))),
            "not_a_present" => Some(g_not_a_present()),
            "pets" => Some(pyrt::xrefs::xmodule("knowledge.pets")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "COOKING_VERBS" => s_COOKING_VERBS(v),
            "DAY_WORDS" => s_DAY_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "FOOD_HEALTH" => s_FOOD_HEALTH(v),
            "HEALTH_AVOID" => s_HEALTH_AVOID(v),
            "HEALTH_WORDS" => s_HEALTH_WORDS(v),
            "MORE_OF" => s_MORE_OF(v),
            "NUTRIENT_ROLE" => s_NUTRIENT_ROLE(v),
            "PACK_DATE" => s_PACK_DATE(v),
            "PORRIDGE_VERDICT" => s_PORRIDGE_VERDICT(v),
            "_AGE_WORDS" => s__AGE_WORDS(v),
            "_A_BARE_OK_ASK" => s__A_BARE_OK_ASK(v),
            "_A_CHILD" => s__A_CHILD(v),
            "_A_DAY" => s__A_DAY(v),
            "_A_DAY_TEEN" => s__A_DAY_TEEN(v),
            "_A_DAY_WOMAN" => s__A_DAY_WOMAN(v),
            "_A_DAY_WORDS" => s__A_DAY_WORDS(v),
            "_A_DOSE" => s__A_DOSE(v),
            "_A_GROWN_UP" => s__A_GROWN_UP(v),
            "_A_STATEMENT_ABOUT_ME" => s__A_STATEMENT_ABOUT_ME(v),
            "_A_STATE_NOT_SPOILAGE" => s__A_STATE_NOT_SPOILAGE(v),
            "_A_THERMOMETER" => s__A_THERMOMETER(v),
            "_A_VERDICT_ASK" => s__A_VERDICT_ASK(v),
            "_CEREAL_BOWLS" => s__CEREAL_BOWLS(v),
            "_CEREAL_SHORT" => s__CEREAL_SHORT(v),
            "_CHILD_AGE" => s__CHILD_AGE(v),
            "_COW_MILK_REPLY" => s__COW_MILK_REPLY(v),
            "_DISLIKE" => s__DISLIKE(v),
            "_EAT_BETTER" => s__EAT_BETTER(v),
            "_EDIBILITY_ASK" => s__EDIBILITY_ASK(v),
            "_FUSSY" => s__FUSSY(v),
            "_GOING_OFF" => s__GOING_OFF(v),
            "_ME_ASKING" => s__ME_ASKING(v),
            "_MILK_AND_THE_PLANET" => s__MILK_AND_THE_PLANET(v),
            "_MILK_FOR_A_CHILD" => s__MILK_FOR_A_CHILD(v),
            "_MORE_OF_ASK" => s__MORE_OF_ASK(v),
            "_NAMED_TAKEAWAY" => s__NAMED_TAKEAWAY(v),
            "_NOT_A_PERSON_OR_PET" => s__NOT_A_PERSON_OR_PET(v),
            "_NOURISHING" => s__NOURISHING(v),
            "_PACK_DATE" => s__PACK_DATE(v),
            "_PLANT_MILKS" => s__PLANT_MILKS(v),
            "_PREGNANT" => s__PREGNANT(v),
            "_ROLE_ASK" => s__ROLE_ASK(v),
            "_SOMEONE_ELSE" => s__SOMEONE_ELSE(v),
            "_SWEET_DRINKS" => s__SWEET_DRINKS(v),
            "_THE_FRIDGE_CHECK" => s__THE_FRIDGE_CHECK(v),
            "_THE_MILK_RANKING" => s__THE_MILK_RANKING(v),
            "_THE_PLANET" => s__THE_PLANET(v),
            "_VEG_A_CHILD" => s__VEG_A_CHILD(v),
            "_VEG_A_GROWN_UP" => s__VEG_A_GROWN_UP(v),
            "_VEG_FIVE_A_DAY" => s__VEG_FIVE_A_DAY(v),
            "_VEG_WHO_IS_IT" => s__VEG_WHO_IS_IT(v),
            "_WHICH_MILK" => s__WHICH_MILK(v),
            "_food_avoid" => s__food_avoid(v),
            "_food_id" => s__food_id(v),
            "_food_line" => s__food_line(v),
            "_food_names" => s__food_names(v),
            "_raw_entry" => s__raw_entry(v),
            "a_food_is_being_kept" => s_a_food_is_being_kept(v),
            "food_has_gone_off" => s_food_has_gone_off(v),
            "not_a_count_for_the_box_or_the_dish" => s_not_a_count_for_the_box_or_the_dish(v),
            "not_a_present" => s_not_a_present(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.nutrition", module_obj);
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_frozen_and_tinned_reply", py_func("_frozen_and_tinned_reply", |args: &[Py]| _frozen_and_tinned_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "entry", py_func("entry", |args: &[Py]| entry(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "not_a_dose", py_func("not_a_dose", |args: &[Py]| not_a_dose(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_a_word_in", py_func("_a_word_in", |args: &[Py]| _a_word_in(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_a_child_in", py_func("_a_child_in", |args: &[Py]| _a_child_in(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_who_in_line", py_func("_who_in_line", |args: &[Py]| _who_in_line(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_who_this_is_about", py_func("_who_this_is_about", |args: &[Py]| _who_this_is_about(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_veg_reply", py_func("_veg_reply", |args: &[Py]| _veg_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_the_bowl_named", py_func("_the_bowl_named", |args: &[Py]| _the_bowl_named(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_cereal_verdict", py_func("_cereal_verdict", |args: &[Py]| _cereal_verdict(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_a_day", py_func("_a_day", |args: &[Py]| _a_day(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_pregnancy_vitamins", py_func("_pregnancy_vitamins", |args: &[Py]| _pregnancy_vitamins(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_more_of", py_func("_more_of", |args: &[Py]| _more_of(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_low_iron", py_func("_low_iron", |args: &[Py]| _low_iron(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_nutrient_role", py_func("_nutrient_role", |args: &[Py]| _nutrient_role(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_a_bare_word", py_func("_a_bare_word", |args: &[Py]| _a_bare_word(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_a_plant_milk", py_func("_a_plant_milk", |args: &[Py]| _a_plant_milk(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_coffee_or_tea", py_func("_coffee_or_tea", |args: &[Py]| _coffee_or_tea(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_not_a_keeping_question", py_func("_not_a_keeping_question", |args: &[Py]| _not_a_keeping_question(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_wholegrain", py_func("_wholegrain", |args: &[Py]| _wholegrain(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "not_a_cooking_question", py_func("not_a_cooking_question", |args: &[Py]| not_a_cooking_question(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_sugary_drink_reply", py_func("_sugary_drink_reply", |args: &[Py]| _sugary_drink_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_not_a_verdict_on_what_is_going_off", py_func("_not_a_verdict_on_what_is_going_off", |args: &[Py]| _not_a_verdict_on_what_is_going_off(args)));
    pyrt::xrefs::reg_fn("knowledge.nutrition", "_not_a_named_takeaway", py_func("_not_a_named_takeaway", |args: &[Py]| _not_a_named_takeaway(args)));
    pyrt::xrefs::reg_access("knowledge.nutrition", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "COOKING_VERBS", g_COOKING_VERBS, s_COOKING_VERBS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "DAY_WORDS", g_DAY_WORDS, s_DAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.nutrition", "FOOD_HEALTH", g_FOOD_HEALTH, s_FOOD_HEALTH);
    pyrt::xrefs::reg_access("knowledge.nutrition", "HEALTH_AVOID", g_HEALTH_AVOID, s_HEALTH_AVOID);
    pyrt::xrefs::reg_access("knowledge.nutrition", "HEALTH_WORDS", g_HEALTH_WORDS, s_HEALTH_WORDS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "MORE_OF", g_MORE_OF, s_MORE_OF);
    pyrt::xrefs::reg_access("knowledge.nutrition", "NUTRIENT_ROLE", g_NUTRIENT_ROLE, s_NUTRIENT_ROLE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "PACK_DATE", g_PACK_DATE, s_PACK_DATE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "PORRIDGE_VERDICT", g_PORRIDGE_VERDICT, s_PORRIDGE_VERDICT);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_AGE_WORDS", g__AGE_WORDS, s__AGE_WORDS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_BARE_OK_ASK", g__A_BARE_OK_ASK, s__A_BARE_OK_ASK);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_CHILD", g__A_CHILD, s__A_CHILD);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_DAY", g__A_DAY, s__A_DAY);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_DAY_TEEN", g__A_DAY_TEEN, s__A_DAY_TEEN);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_DAY_WOMAN", g__A_DAY_WOMAN, s__A_DAY_WOMAN);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_DAY_WORDS", g__A_DAY_WORDS, s__A_DAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_DOSE", g__A_DOSE, s__A_DOSE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_GROWN_UP", g__A_GROWN_UP, s__A_GROWN_UP);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_STATEMENT_ABOUT_ME", g__A_STATEMENT_ABOUT_ME, s__A_STATEMENT_ABOUT_ME);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_STATE_NOT_SPOILAGE", g__A_STATE_NOT_SPOILAGE, s__A_STATE_NOT_SPOILAGE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_THERMOMETER", g__A_THERMOMETER, s__A_THERMOMETER);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_A_VERDICT_ASK", g__A_VERDICT_ASK, s__A_VERDICT_ASK);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_CEREAL_BOWLS", g__CEREAL_BOWLS, s__CEREAL_BOWLS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_CEREAL_SHORT", g__CEREAL_SHORT, s__CEREAL_SHORT);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_CHILD_AGE", g__CHILD_AGE, s__CHILD_AGE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_COW_MILK_REPLY", g__COW_MILK_REPLY, s__COW_MILK_REPLY);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_DISLIKE", g__DISLIKE, s__DISLIKE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_EAT_BETTER", g__EAT_BETTER, s__EAT_BETTER);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_EDIBILITY_ASK", g__EDIBILITY_ASK, s__EDIBILITY_ASK);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_FUSSY", g__FUSSY, s__FUSSY);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_GOING_OFF", g__GOING_OFF, s__GOING_OFF);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_ME_ASKING", g__ME_ASKING, s__ME_ASKING);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_MILK_AND_THE_PLANET", g__MILK_AND_THE_PLANET, s__MILK_AND_THE_PLANET);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_MILK_FOR_A_CHILD", g__MILK_FOR_A_CHILD, s__MILK_FOR_A_CHILD);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_MORE_OF_ASK", g__MORE_OF_ASK, s__MORE_OF_ASK);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_NAMED_TAKEAWAY", g__NAMED_TAKEAWAY, s__NAMED_TAKEAWAY);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_NOT_A_PERSON_OR_PET", g__NOT_A_PERSON_OR_PET, s__NOT_A_PERSON_OR_PET);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_NOURISHING", g__NOURISHING, s__NOURISHING);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_PACK_DATE", g__PACK_DATE, s__PACK_DATE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_PLANT_MILKS", g__PLANT_MILKS, s__PLANT_MILKS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_PREGNANT", g__PREGNANT, s__PREGNANT);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_ROLE_ASK", g__ROLE_ASK, s__ROLE_ASK);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_SOMEONE_ELSE", g__SOMEONE_ELSE, s__SOMEONE_ELSE);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_SWEET_DRINKS", g__SWEET_DRINKS, s__SWEET_DRINKS);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_THE_FRIDGE_CHECK", g__THE_FRIDGE_CHECK, s__THE_FRIDGE_CHECK);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_THE_MILK_RANKING", g__THE_MILK_RANKING, s__THE_MILK_RANKING);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_THE_PLANET", g__THE_PLANET, s__THE_PLANET);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_VEG_A_CHILD", g__VEG_A_CHILD, s__VEG_A_CHILD);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_VEG_A_GROWN_UP", g__VEG_A_GROWN_UP, s__VEG_A_GROWN_UP);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_VEG_FIVE_A_DAY", g__VEG_FIVE_A_DAY, s__VEG_FIVE_A_DAY);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_VEG_WHO_IS_IT", g__VEG_WHO_IS_IT, s__VEG_WHO_IS_IT);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_WHICH_MILK", g__WHICH_MILK, s__WHICH_MILK);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_food_avoid", g__food_avoid, s__food_avoid);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_food_id", g__food_id, s__food_id);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_food_line", g__food_line, s__food_line);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_food_names", g__food_names, s__food_names);
    pyrt::xrefs::reg_access("knowledge.nutrition", "_raw_entry", g__raw_entry, s__raw_entry);
    pyrt::xrefs::reg_access("knowledge.nutrition", "a_food_is_being_kept", g_a_food_is_being_kept, s_a_food_is_being_kept);
    pyrt::xrefs::reg_access("knowledge.nutrition", "food_has_gone_off", g_food_has_gone_off, s_food_has_gone_off);
    pyrt::xrefs::reg_access("knowledge.nutrition", "not_a_count_for_the_box_or_the_dish", g_not_a_count_for_the_box_or_the_dish, s_not_a_count_for_the_box_or_the_dish);
    pyrt::xrefs::reg_access("knowledge.nutrition", "not_a_present", g_not_a_present, s_not_a_present);
}
