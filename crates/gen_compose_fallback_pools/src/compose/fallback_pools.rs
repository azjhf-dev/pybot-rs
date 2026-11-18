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


thread_local! { static G_ANIMALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_IF_I_SAW_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_IN_A_CRISIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FOLLOW_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FOLLOW_UP_CARE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FOLLOW_UP_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PICK_AFTER_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_IS_NOT_A_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DITHER_OPEN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPERT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HANDED_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELP_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_DECIDE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAVING_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIFE_DIRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEUTRAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TIPS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_NOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REALISTIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REALISTIC_ABOUT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_BARE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_DECISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_TARIFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_WHAT_IT_IS_WORTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_WHETHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_WHETHER_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_AGAIN_WITH_THE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_CAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_CONFIRM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_UNSURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_ABOUT_A_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_ABOUT_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_HEALTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_REALISTIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_REALISTIC_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_TOO_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WORD_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_WORRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_AFTER_A_DEATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_ASK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_COMPANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_HANDBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_MIRROR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_MIRROR_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_NAMES_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_ON_GOOD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WASNT_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_FOR_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_bare_worry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_care_home_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_dead_end: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_plan_doubt: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_plan_in_the_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_price_wants_a_verdict: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_question_about_the_person: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_repair_already_spoken_of: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_subject_above: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_switch_off_asked: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_thing_they_started: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_thing_was_named: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__advisor: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__after_a_colon: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__answered_already: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__answered_recently: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__asked_about_the_day_they_had: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__asks_a_name_back: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__asks_for_a_person_to_talk_to: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__bare_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__choose: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__forecast_pool: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__grief_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__hedged_recently: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__in_a_care_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__in_a_crisis_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__in_a_loss_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__in_a_pool: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__is_a_hedge: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__is_a_tips_ask: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__is_a_view_ask: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__name_given_before: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__names_its_own_thing: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__news_above: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__nothing_to_choose: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__nothing_to_stand_behind: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__offers_to_fix_themselves: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__only_a_reaction: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__only_the_floor: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__points_at_nothing: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__reads: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__reads_as_a_claim: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__reads_as_taste: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__says_it_already: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_answer_above: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_answer_in_reply: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_line_before: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_pool_for_a_worry_about_them: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_thread_is_heavy: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_view_of: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_weird_ask_pool: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__their_own_day: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__thread_is_about_a_person: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__thread_is_about_a_pet: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__thread_is_about_someone_else: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__thread_is_about_the_body: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__thread_names_a_machine: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__tips: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__who_for_the_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ack: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_asks_for_more: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_finding_out: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_in_a_safety_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_method_ask_pool: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_no_eyes: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_reads_as_if_i_saw: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_bots_own_answer: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_lost_ask_reply: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_place_the_answer_is_kept: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_rule_for_it: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_unknown_pool: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_who_settles: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.fallback_pools: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANIMALS() -> Py { ensure_init(); G_ANIMALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMALS(v: Py) { ensure_init(); G_ANIMALS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_IF_I_SAW_LINES() -> Py { ensure_init(); G_ASKED_IF_I_SAW_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_IF_I_SAW_LINES(v: Py) { ensure_init(); G_ASKED_IF_I_SAW_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_WORD() -> Py { ensure_init(); G_A_CARE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_WORD(v: Py) { ensure_init(); G_A_CARE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_WORD() -> Py { ensure_init(); G_A_MONEY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_WORD(v: Py) { ensure_init(); G_A_MONEY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_IN_A_CRISIS() -> Py { ensure_init(); G_A_PERSON_IN_A_CRISIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_IN_A_CRISIS(v: Py) { ensure_init(); G_A_PERSON_IN_A_CRISIS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_ASK() -> Py { ensure_init(); G_A_STEP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_ASK(v: Py) { ensure_init(); G_A_STEP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FOLLOW_UP() -> Py { ensure_init(); G_BARE_FOLLOW_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FOLLOW_UP(v: Py) { ensure_init(); G_BARE_FOLLOW_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FOLLOW_UP_CARE_REPLIES() -> Py { ensure_init(); G_BARE_FOLLOW_UP_CARE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FOLLOW_UP_CARE_REPLIES(v: Py) { ensure_init(); G_BARE_FOLLOW_UP_CARE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FOLLOW_UP_REPLIES() -> Py { ensure_init(); G_BARE_FOLLOW_UP_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FOLLOW_UP_REPLIES(v: Py) { ensure_init(); G_BARE_FOLLOW_UP_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PICK_AFTER_NEWS() -> Py { ensure_init(); G_BARE_PICK_AFTER_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PICK_AFTER_NEWS(v: Py) { ensure_init(); G_BARE_PICK_AFTER_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_IS_NOT_A_MACHINE() -> Py { ensure_init(); G_BODY_IS_NOT_A_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_IS_NOT_A_MACHINE(v: Py) { ensure_init(); G_BODY_IS_NOT_A_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PARTS() -> Py { ensure_init(); G_BODY_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PARTS(v: Py) { ensure_init(); G_BODY_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_HERE() -> Py { ensure_init(); G_CARE_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_HERE(v: Py) { ensure_init(); G_CARE_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_MODULES() -> Py { ensure_init(); G_CARE_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_MODULES(v: Py) { ensure_init(); G_CARE_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_ASK() -> Py { ensure_init(); G_CRISIS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_ASK(v: Py) { ensure_init(); G_CRISIS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DITHER_OPEN_LINES() -> Py { ensure_init(); G_DITHER_OPEN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DITHER_OPEN_LINES(v: Py) { ensure_init(); G_DITHER_OPEN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPERT_WORDS() -> Py { ensure_init(); G_EXPERT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPERT_WORDS(v: Py) { ensure_init(); G_EXPERT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HANDED_BACK() -> Py { ensure_init(); G_HANDED_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HANDED_BACK(v: Py) { ensure_init(); G_HANDED_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_WORDS() -> Py { ensure_init(); G_HEALTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_WORDS(v: Py) { ensure_init(); G_HEALTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HELP_ALREADY() -> Py { ensure_init(); G_HELP_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELP_ALREADY(v: Py) { ensure_init(); G_HELP_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_DECIDE() -> Py { ensure_init(); G_HOW_TO_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_DECIDE(v: Py) { ensure_init(); G_HOW_TO_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_DECIDE_LINES() -> Py { ensure_init(); G_HOW_TO_DECIDE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_DECIDE_LINES(v: Py) { ensure_init(); G_HOW_TO_DECIDE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAVING_IN_THE_LINE() -> Py { ensure_init(); G_LEAVING_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAVING_IN_THE_LINE(v: Py) { ensure_init(); G_LEAVING_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIFE_DIRECTION() -> Py { ensure_init(); G_LIFE_DIRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIFE_DIRECTION(v: Py) { ensure_init(); G_LIFE_DIRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP() -> Py { ensure_init(); G_MISSED_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP(v: Py) { ensure_init(); G_MISSED_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP_KEEPS() -> Py { ensure_init(); G_MISSED_STEP_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_KEEPS(v: Py) { ensure_init(); G_MISSED_STEP_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP_LINES() -> Py { ensure_init(); G_MISSED_STEP_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_LINES(v: Py) { ensure_init(); G_MISSED_STEP_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP_PLAN() -> Py { ensure_init(); G_MISSED_STEP_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_PLAN(v: Py) { ensure_init(); G_MISSED_STEP_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_NEUTRAL() -> Py { ensure_init(); G_NEUTRAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEUTRAL(v: Py) { ensure_init(); G_NEUTRAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TIPS_ASK() -> Py { ensure_init(); G_NOT_A_TIPS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TIPS_ASK(v: Py) { ensure_init(); G_NOT_A_TIPS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_NOTED() -> Py { ensure_init(); G_NOT_NOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_NOTED(v: Py) { ensure_init(); G_NOT_NOTED.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_LINES() -> Py { ensure_init(); G_POINTING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_LINES(v: Py) { ensure_init(); G_POINTING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_REALISTIC() -> Py { ensure_init(); G_REALISTIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REALISTIC(v: Py) { ensure_init(); G_REALISTIC.with(|c| *c.borrow_mut() = v); }
pub fn g_REALISTIC_ABOUT_ME() -> Py { ensure_init(); G_REALISTIC_ABOUT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REALISTIC_ABOUT_ME(v: Py) { ensure_init(); G_REALISTIC_ABOUT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_BARE_IT() -> Py { ensure_init(); G_REPAIR_BARE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_BARE_IT(v: Py) { ensure_init(); G_REPAIR_BARE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_DECISION() -> Py { ensure_init(); G_REPAIR_DECISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_DECISION(v: Py) { ensure_init(); G_REPAIR_DECISION.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_PERSON() -> Py { ensure_init(); G_REPAIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_PERSON(v: Py) { ensure_init(); G_REPAIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_RULE() -> Py { ensure_init(); G_REPAIR_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_RULE(v: Py) { ensure_init(); G_REPAIR_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_TARIFF() -> Py { ensure_init(); G_REPAIR_TARIFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_TARIFF(v: Py) { ensure_init(); G_REPAIR_TARIFF.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_TRADE() -> Py { ensure_init(); G_REPAIR_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_TRADE(v: Py) { ensure_init(); G_REPAIR_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_WHAT_IT_IS_WORTH() -> Py { ensure_init(); G_REPAIR_WHAT_IT_IS_WORTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WHAT_IT_IS_WORTH(v: Py) { ensure_init(); G_REPAIR_WHAT_IT_IS_WORTH.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_WHETHER() -> Py { ensure_init(); G_REPAIR_WHETHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WHETHER(v: Py) { ensure_init(); G_REPAIR_WHETHER.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_WHETHER_LINES() -> Py { ensure_init(); G_REPAIR_WHETHER_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WHETHER_LINES(v: Py) { ensure_init(); G_REPAIR_WHETHER_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_AGAIN() -> Py { ensure_init(); G_SAME_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_AGAIN(v: Py) { ensure_init(); G_SAME_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_AGAIN_WITH_THE_ANSWER() -> Py { ensure_init(); G_SAME_AGAIN_WITH_THE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_AGAIN_WITH_THE_ANSWER(v: Py) { ensure_init(); G_SAME_AGAIN_WITH_THE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM_WORDS() -> Py { ensure_init(); G_SYMPTOM_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM_WORDS(v: Py) { ensure_init(); G_SYMPTOM_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_CAN() -> Py { ensure_init(); G_TAG_CAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_CAN(v: Py) { ensure_init(); G_TAG_CAN.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_CONFIRM() -> Py { ensure_init(); G_TAG_CONFIRM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_CONFIRM(v: Py) { ensure_init(); G_TAG_CONFIRM.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_UNSURE() -> Py { ensure_init(); G_TAG_UNSURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_UNSURE(v: Py) { ensure_init(); G_TAG_UNSURE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_RULE() -> Py { ensure_init(); G_TIME_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_RULE(v: Py) { ensure_init(); G_TIME_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_ASKS() -> Py { ensure_init(); G_TIPS_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_ASKS(v: Py) { ensure_init(); G_TIPS_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_LINES() -> Py { ensure_init(); G_TIPS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_LINES(v: Py) { ensure_init(); G_TIPS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_LATE() -> Py { ensure_init(); G_TOO_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_LATE(v: Py) { ensure_init(); G_TOO_LATE.with(|c| *c.borrow_mut() = v); }
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
pub fn g_UNKNOWN_REALISTIC() -> Py { ensure_init(); G_UNKNOWN_REALISTIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_REALISTIC(v: Py) { ensure_init(); G_UNKNOWN_REALISTIC.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_REALISTIC_CARE() -> Py { ensure_init(); G_UNKNOWN_REALISTIC_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_REALISTIC_CARE(v: Py) { ensure_init(); G_UNKNOWN_REALISTIC_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_TOO_LATE() -> Py { ensure_init(); G_UNKNOWN_TOO_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_TOO_LATE(v: Py) { ensure_init(); G_UNKNOWN_TOO_LATE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WORD() -> Py { ensure_init(); G_UNKNOWN_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WORD(v: Py) { ensure_init(); G_UNKNOWN_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WORD_NAMED() -> Py { ensure_init(); G_UNKNOWN_WORD_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WORD_NAMED(v: Py) { ensure_init(); G_UNKNOWN_WORD_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_WORRY() -> Py { ensure_init(); G_UNKNOWN_WORRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_WORRY(v: Py) { ensure_init(); G_UNKNOWN_WORRY.with(|c| *c.borrow_mut() = v); }
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
pub fn g_VIEW_LINES() -> Py { ensure_init(); G_VIEW_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_LINES(v: Py) { ensure_init(); G_VIEW_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_MIRROR() -> Py { ensure_init(); G_VIEW_MIRROR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_MIRROR(v: Py) { ensure_init(); G_VIEW_MIRROR.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_MIRROR_LINES() -> Py { ensure_init(); G_VIEW_MIRROR_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_MIRROR_LINES(v: Py) { ensure_init(); G_VIEW_MIRROR_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_NAMES_SOMEBODY() -> Py { ensure_init(); G_VIEW_NAMES_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_NAMES_SOMEBODY(v: Py) { ensure_init(); G_VIEW_NAMES_SOMEBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_NO_SUBJECT() -> Py { ensure_init(); G_VIEW_NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_NO_SUBJECT(v: Py) { ensure_init(); G_VIEW_NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_ON_GOOD_NEWS() -> Py { ensure_init(); G_VIEW_ON_GOOD_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_ON_GOOD_NEWS(v: Py) { ensure_init(); G_VIEW_ON_GOOD_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_WASNT_THERE() -> Py { ensure_init(); G_WASNT_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WASNT_THERE(v: Py) { ensure_init(); G_WASNT_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_FOR_IT() -> Py { ensure_init(); G_WHO_FOR_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_FOR_IT(v: Py) { ensure_init(); G_WHO_FOR_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__a_bare_worry() -> Py { ensure_init(); G__a_bare_worry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_bare_worry(v: Py) { ensure_init(); G__a_bare_worry.with(|c| *c.borrow_mut() = v); }
pub fn g__a_care_home_line() -> Py { ensure_init(); G__a_care_home_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_care_home_line(v: Py) { ensure_init(); G__a_care_home_line.with(|c| *c.borrow_mut() = v); }
pub fn g__a_dead_end() -> Py { ensure_init(); G__a_dead_end.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_dead_end(v: Py) { ensure_init(); G__a_dead_end.with(|c| *c.borrow_mut() = v); }
pub fn g__a_plan_doubt() -> Py { ensure_init(); G__a_plan_doubt.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_plan_doubt(v: Py) { ensure_init(); G__a_plan_doubt.with(|c| *c.borrow_mut() = v); }
pub fn g__a_plan_in_the_thread() -> Py { ensure_init(); G__a_plan_in_the_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_plan_in_the_thread(v: Py) { ensure_init(); G__a_plan_in_the_thread.with(|c| *c.borrow_mut() = v); }
pub fn g__a_price_wants_a_verdict() -> Py { ensure_init(); G__a_price_wants_a_verdict.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_price_wants_a_verdict(v: Py) { ensure_init(); G__a_price_wants_a_verdict.with(|c| *c.borrow_mut() = v); }
pub fn g__a_question_about_the_person() -> Py { ensure_init(); G__a_question_about_the_person.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_question_about_the_person(v: Py) { ensure_init(); G__a_question_about_the_person.with(|c| *c.borrow_mut() = v); }
pub fn g__a_repair_already_spoken_of() -> Py { ensure_init(); G__a_repair_already_spoken_of.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_repair_already_spoken_of(v: Py) { ensure_init(); G__a_repair_already_spoken_of.with(|c| *c.borrow_mut() = v); }
pub fn g__a_subject_above() -> Py { ensure_init(); G__a_subject_above.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_subject_above(v: Py) { ensure_init(); G__a_subject_above.with(|c| *c.borrow_mut() = v); }
pub fn g__a_switch_off_asked() -> Py { ensure_init(); G__a_switch_off_asked.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_switch_off_asked(v: Py) { ensure_init(); G__a_switch_off_asked.with(|c| *c.borrow_mut() = v); }
pub fn g__a_thing_they_started() -> Py { ensure_init(); G__a_thing_they_started.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_thing_they_started(v: Py) { ensure_init(); G__a_thing_they_started.with(|c| *c.borrow_mut() = v); }
pub fn g__a_thing_was_named() -> Py { ensure_init(); G__a_thing_was_named.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_thing_was_named(v: Py) { ensure_init(); G__a_thing_was_named.with(|c| *c.borrow_mut() = v); }
pub fn g__advisor() -> Py { ensure_init(); G__advisor.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__advisor(v: Py) { ensure_init(); G__advisor.with(|c| *c.borrow_mut() = v); }
pub fn g__after_a_colon() -> Py { ensure_init(); G__after_a_colon.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__after_a_colon(v: Py) { ensure_init(); G__after_a_colon.with(|c| *c.borrow_mut() = v); }
pub fn g__answered_already() -> Py { ensure_init(); G__answered_already.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__answered_already(v: Py) { ensure_init(); G__answered_already.with(|c| *c.borrow_mut() = v); }
pub fn g__answered_recently() -> Py { ensure_init(); G__answered_recently.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__answered_recently(v: Py) { ensure_init(); G__answered_recently.with(|c| *c.borrow_mut() = v); }
pub fn g__asked_about_the_day_they_had() -> Py { ensure_init(); G__asked_about_the_day_they_had.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__asked_about_the_day_they_had(v: Py) { ensure_init(); G__asked_about_the_day_they_had.with(|c| *c.borrow_mut() = v); }
pub fn g__asks_a_name_back() -> Py { ensure_init(); G__asks_a_name_back.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__asks_a_name_back(v: Py) { ensure_init(); G__asks_a_name_back.with(|c| *c.borrow_mut() = v); }
pub fn g__asks_for_a_person_to_talk_to() -> Py { ensure_init(); G__asks_for_a_person_to_talk_to.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__asks_for_a_person_to_talk_to(v: Py) { ensure_init(); G__asks_for_a_person_to_talk_to.with(|c| *c.borrow_mut() = v); }
pub fn g__bare_pick() -> Py { ensure_init(); G__bare_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__bare_pick(v: Py) { ensure_init(); G__bare_pick.with(|c| *c.borrow_mut() = v); }
pub fn g__choose() -> Py { ensure_init(); G__choose.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__choose(v: Py) { ensure_init(); G__choose.with(|c| *c.borrow_mut() = v); }
pub fn g__forecast_pool() -> Py { ensure_init(); G__forecast_pool.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__forecast_pool(v: Py) { ensure_init(); G__forecast_pool.with(|c| *c.borrow_mut() = v); }
pub fn g__grief_thread() -> Py { ensure_init(); G__grief_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__grief_thread(v: Py) { ensure_init(); G__grief_thread.with(|c| *c.borrow_mut() = v); }
pub fn g__hedged_recently() -> Py { ensure_init(); G__hedged_recently.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__hedged_recently(v: Py) { ensure_init(); G__hedged_recently.with(|c| *c.borrow_mut() = v); }
pub fn g__in_a_care_thread() -> Py { ensure_init(); G__in_a_care_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__in_a_care_thread(v: Py) { ensure_init(); G__in_a_care_thread.with(|c| *c.borrow_mut() = v); }
pub fn g__in_a_crisis_thread() -> Py { ensure_init(); G__in_a_crisis_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__in_a_crisis_thread(v: Py) { ensure_init(); G__in_a_crisis_thread.with(|c| *c.borrow_mut() = v); }
pub fn g__in_a_loss_thread() -> Py { ensure_init(); G__in_a_loss_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__in_a_loss_thread(v: Py) { ensure_init(); G__in_a_loss_thread.with(|c| *c.borrow_mut() = v); }
pub fn g__in_a_pool() -> Py { ensure_init(); G__in_a_pool.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__in_a_pool(v: Py) { ensure_init(); G__in_a_pool.with(|c| *c.borrow_mut() = v); }
pub fn g__is_a_hedge() -> Py { ensure_init(); G__is_a_hedge.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__is_a_hedge(v: Py) { ensure_init(); G__is_a_hedge.with(|c| *c.borrow_mut() = v); }
pub fn g__is_a_tips_ask() -> Py { ensure_init(); G__is_a_tips_ask.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__is_a_tips_ask(v: Py) { ensure_init(); G__is_a_tips_ask.with(|c| *c.borrow_mut() = v); }
pub fn g__is_a_view_ask() -> Py { ensure_init(); G__is_a_view_ask.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__is_a_view_ask(v: Py) { ensure_init(); G__is_a_view_ask.with(|c| *c.borrow_mut() = v); }
pub fn g__name_given_before() -> Py { ensure_init(); G__name_given_before.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__name_given_before(v: Py) { ensure_init(); G__name_given_before.with(|c| *c.borrow_mut() = v); }
pub fn g__names_its_own_thing() -> Py { ensure_init(); G__names_its_own_thing.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__names_its_own_thing(v: Py) { ensure_init(); G__names_its_own_thing.with(|c| *c.borrow_mut() = v); }
pub fn g__news_above() -> Py { ensure_init(); G__news_above.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__news_above(v: Py) { ensure_init(); G__news_above.with(|c| *c.borrow_mut() = v); }
pub fn g__nothing_to_choose() -> Py { ensure_init(); G__nothing_to_choose.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__nothing_to_choose(v: Py) { ensure_init(); G__nothing_to_choose.with(|c| *c.borrow_mut() = v); }
pub fn g__nothing_to_stand_behind() -> Py { ensure_init(); G__nothing_to_stand_behind.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__nothing_to_stand_behind(v: Py) { ensure_init(); G__nothing_to_stand_behind.with(|c| *c.borrow_mut() = v); }
pub fn g__offers_to_fix_themselves() -> Py { ensure_init(); G__offers_to_fix_themselves.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__offers_to_fix_themselves(v: Py) { ensure_init(); G__offers_to_fix_themselves.with(|c| *c.borrow_mut() = v); }
pub fn g__only_a_reaction() -> Py { ensure_init(); G__only_a_reaction.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__only_a_reaction(v: Py) { ensure_init(); G__only_a_reaction.with(|c| *c.borrow_mut() = v); }
pub fn g__only_the_floor() -> Py { ensure_init(); G__only_the_floor.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__only_the_floor(v: Py) { ensure_init(); G__only_the_floor.with(|c| *c.borrow_mut() = v); }
pub fn g__points_at_nothing() -> Py { ensure_init(); G__points_at_nothing.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__points_at_nothing(v: Py) { ensure_init(); G__points_at_nothing.with(|c| *c.borrow_mut() = v); }
pub fn g__reads() -> Py { ensure_init(); G__reads.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__reads(v: Py) { ensure_init(); G__reads.with(|c| *c.borrow_mut() = v); }
pub fn g__reads_as_a_claim() -> Py { ensure_init(); G__reads_as_a_claim.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__reads_as_a_claim(v: Py) { ensure_init(); G__reads_as_a_claim.with(|c| *c.borrow_mut() = v); }
pub fn g__reads_as_taste() -> Py { ensure_init(); G__reads_as_taste.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__reads_as_taste(v: Py) { ensure_init(); G__reads_as_taste.with(|c| *c.borrow_mut() = v); }
pub fn g__says_it_already() -> Py { ensure_init(); G__says_it_already.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__says_it_already(v: Py) { ensure_init(); G__says_it_already.with(|c| *c.borrow_mut() = v); }
pub fn g__the_answer_above() -> Py { ensure_init(); G__the_answer_above.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_answer_above(v: Py) { ensure_init(); G__the_answer_above.with(|c| *c.borrow_mut() = v); }
pub fn g__the_answer_in_reply() -> Py { ensure_init(); G__the_answer_in_reply.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_answer_in_reply(v: Py) { ensure_init(); G__the_answer_in_reply.with(|c| *c.borrow_mut() = v); }
pub fn g__the_line_before() -> Py { ensure_init(); G__the_line_before.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_line_before(v: Py) { ensure_init(); G__the_line_before.with(|c| *c.borrow_mut() = v); }
pub fn g__the_pool_for_a_worry_about_them() -> Py { ensure_init(); G__the_pool_for_a_worry_about_them.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_pool_for_a_worry_about_them(v: Py) { ensure_init(); G__the_pool_for_a_worry_about_them.with(|c| *c.borrow_mut() = v); }
pub fn g__the_thread_is_heavy() -> Py { ensure_init(); G__the_thread_is_heavy.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_thread_is_heavy(v: Py) { ensure_init(); G__the_thread_is_heavy.with(|c| *c.borrow_mut() = v); }
pub fn g__the_view_of() -> Py { ensure_init(); G__the_view_of.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_view_of(v: Py) { ensure_init(); G__the_view_of.with(|c| *c.borrow_mut() = v); }
pub fn g__the_weird_ask_pool() -> Py { ensure_init(); G__the_weird_ask_pool.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_weird_ask_pool(v: Py) { ensure_init(); G__the_weird_ask_pool.with(|c| *c.borrow_mut() = v); }
pub fn g__their_own_day() -> Py { ensure_init(); G__their_own_day.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__their_own_day(v: Py) { ensure_init(); G__their_own_day.with(|c| *c.borrow_mut() = v); }
pub fn g__thread_is_about_a_person() -> Py { ensure_init(); G__thread_is_about_a_person.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__thread_is_about_a_person(v: Py) { ensure_init(); G__thread_is_about_a_person.with(|c| *c.borrow_mut() = v); }
pub fn g__thread_is_about_a_pet() -> Py { ensure_init(); G__thread_is_about_a_pet.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__thread_is_about_a_pet(v: Py) { ensure_init(); G__thread_is_about_a_pet.with(|c| *c.borrow_mut() = v); }
pub fn g__thread_is_about_someone_else() -> Py { ensure_init(); G__thread_is_about_someone_else.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__thread_is_about_someone_else(v: Py) { ensure_init(); G__thread_is_about_someone_else.with(|c| *c.borrow_mut() = v); }
pub fn g__thread_is_about_the_body() -> Py { ensure_init(); G__thread_is_about_the_body.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__thread_is_about_the_body(v: Py) { ensure_init(); G__thread_is_about_the_body.with(|c| *c.borrow_mut() = v); }
pub fn g__thread_names_a_machine() -> Py { ensure_init(); G__thread_names_a_machine.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__thread_names_a_machine(v: Py) { ensure_init(); G__thread_names_a_machine.with(|c| *c.borrow_mut() = v); }
pub fn g__tips() -> Py { ensure_init(); G__tips.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__tips(v: Py) { ensure_init(); G__tips.with(|c| *c.borrow_mut() = v); }
pub fn g__who_for_the_thread() -> Py { ensure_init(); G__who_for_the_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__who_for_the_thread(v: Py) { ensure_init(); G__who_for_the_thread.with(|c| *c.borrow_mut() = v); }
pub fn g_ack() -> Py { ensure_init(); G_ack.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ack(v: Py) { ensure_init(); G_ack.with(|c| *c.borrow_mut() = v); }
pub fn g_asks_for_more() -> Py { ensure_init(); G_asks_for_more.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_asks_for_more(v: Py) { ensure_init(); G_asks_for_more.with(|c| *c.borrow_mut() = v); }
pub fn g_finding_out() -> Py { ensure_init(); G_finding_out.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_finding_out(v: Py) { ensure_init(); G_finding_out.with(|c| *c.borrow_mut() = v); }
pub fn g_in_a_safety_thread() -> Py { ensure_init(); G_in_a_safety_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_in_a_safety_thread(v: Py) { ensure_init(); G_in_a_safety_thread.with(|c| *c.borrow_mut() = v); }
pub fn g_method_ask_pool() -> Py { ensure_init(); G_method_ask_pool.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_method_ask_pool(v: Py) { ensure_init(); G_method_ask_pool.with(|c| *c.borrow_mut() = v); }
pub fn g_no_eyes() -> Py { ensure_init(); G_no_eyes.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_no_eyes(v: Py) { ensure_init(); G_no_eyes.with(|c| *c.borrow_mut() = v); }
pub fn g_reads_as_if_i_saw() -> Py { ensure_init(); G_reads_as_if_i_saw.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_reads_as_if_i_saw(v: Py) { ensure_init(); G_reads_as_if_i_saw.with(|c| *c.borrow_mut() = v); }
pub fn g_the_bots_own_answer() -> Py { ensure_init(); G_the_bots_own_answer.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_bots_own_answer(v: Py) { ensure_init(); G_the_bots_own_answer.with(|c| *c.borrow_mut() = v); }
pub fn g_the_lost_ask_reply() -> Py { ensure_init(); G_the_lost_ask_reply.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_lost_ask_reply(v: Py) { ensure_init(); G_the_lost_ask_reply.with(|c| *c.borrow_mut() = v); }
pub fn g_the_place_the_answer_is_kept() -> Py { ensure_init(); G_the_place_the_answer_is_kept.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_place_the_answer_is_kept(v: Py) { ensure_init(); G_the_place_the_answer_is_kept.with(|c| *c.borrow_mut() = v); }
pub fn g_the_rule_for_it() -> Py { ensure_init(); G_the_rule_for_it.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_rule_for_it(v: Py) { ensure_init(); G_the_rule_for_it.with(|c| *c.borrow_mut() = v); }
pub fn g_unknown_pool() -> Py { ensure_init(); G_unknown_pool.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_unknown_pool(v: Py) { ensure_init(); G_unknown_pool.with(|c| *c.borrow_mut() = v); }
pub fn g_who_settles() -> Py { ensure_init(); G_who_settles.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_who_settles(v: Py) { ensure_init(); G_who_settles.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The two pool readers that weigh most, lifted out of fallback.py.\n\n`bot/compose/fallback.py` was over the publisher's 1 MiB per-file limit on main, and one file\nthat big stops every PR in the swarm from being filed, whoever wrote it: tables moved to\n`fallback_data.py` in #10007 and it was still 5,354 bytes over.\n\nSo the two readers that weigh most moved out whole - `_unknown`, which composes the honest\nreply when nothing else knows, and `_the_view_asked_for`, which gives the bot's own view back -\nwith every comment and every word unchanged. They read the tables and helpers in `fallback.py`,\nand `fallback.py` calls them through two small wrappers that import this module on the first\ncall, so neither module closes an import circle. No reply changes: this is a move.\n");
    s_MISSED_STEP_LINES(pyrt::xrefs::xget("compose.fallback_data", "MISSED_STEP_LINES"));
    s_REPAIR_WHETHER_LINES(pyrt::xrefs::xget("compose.fallback_data", "REPAIR_WHETHER_LINES"));
    s_the_bots_own_answer(py_func("the_bots_own_answer", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback_data", "the_bots_own_answer", args)));
    s_the_place_the_answer_is_kept(py_func("the_place_the_answer_is_kept", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback_data", "the_place_the_answer_is_kept", args)));
    s_A_CARE_WORD(pyrt::xrefs::xget("compose.the_rule_for_it", "A_CARE_WORD"));
    s_A_MONEY_WORD(pyrt::xrefs::xget("compose.the_rule_for_it", "A_MONEY_WORD"));
    s_TIME_RULE(pyrt::xrefs::xget("compose.the_rule_for_it", "TIME_RULE"));
    s_the_rule_for_it(py_func("the_rule_for_it", |args: &[Py]| pyrt::xrefs::xcall("compose.the_rule_for_it", "the_rule_for_it", args)));
    s_method_ask_pool(py_func("method_ask_pool", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "method_ask_pool", args)));
    s_ANIMALS(pyrt::xrefs::xget("compose.fallback", "ANIMALS"));
    s_ASKED_IF_I_SAW_LINES(pyrt::xrefs::xget("compose.fallback", "ASKED_IF_I_SAW_LINES"));
    s_A_PERSON_IN_A_CRISIS(pyrt::xrefs::xget("compose.fallback", "A_PERSON_IN_A_CRISIS"));
    s_A_STEP_ASK(pyrt::xrefs::xget("compose.fallback", "A_STEP_ASK"));
    s_BARE_FOLLOW_UP(pyrt::xrefs::xget("compose.fallback", "BARE_FOLLOW_UP"));
    s_BARE_FOLLOW_UP_CARE_REPLIES(pyrt::xrefs::xget("compose.fallback", "BARE_FOLLOW_UP_CARE_REPLIES"));
    s_BARE_FOLLOW_UP_REPLIES(pyrt::xrefs::xget("compose.fallback", "BARE_FOLLOW_UP_REPLIES"));
    s_BARE_PICK_AFTER_NEWS(pyrt::xrefs::xget("compose.fallback", "BARE_PICK_AFTER_NEWS"));
    s_BODY_IS_NOT_A_MACHINE(pyrt::xrefs::xget("compose.fallback", "BODY_IS_NOT_A_MACHINE"));
    s_BODY_PARTS(pyrt::xrefs::xget("compose.fallback", "BODY_PARTS"));
    s_CARE_HERE(pyrt::xrefs::xget("compose.fallback", "CARE_HERE"));
    s_CARE_MODULES(pyrt::xrefs::xget("compose.fallback", "CARE_MODULES"));
    s_CRISIS_ASK(pyrt::xrefs::xget("compose.fallback", "CRISIS_ASK"));
    s_DITHER_OPEN_LINES(pyrt::xrefs::xget("compose.fallback", "DITHER_OPEN_LINES"));
    s_EXPERT_WORDS(pyrt::xrefs::xget("compose.fallback", "EXPERT_WORDS"));
    s_HANDED_BACK(pyrt::xrefs::xget("compose.fallback", "HANDED_BACK"));
    s_HEALTH_WORDS(pyrt::xrefs::xget("compose.fallback", "HEALTH_WORDS"));
    s_HELP_ALREADY(pyrt::xrefs::xget("compose.fallback", "HELP_ALREADY"));
    s_HOW_TO_DECIDE(pyrt::xrefs::xget("compose.fallback", "HOW_TO_DECIDE"));
    s_HOW_TO_DECIDE_LINES(pyrt::xrefs::xget("compose.fallback", "HOW_TO_DECIDE_LINES"));
    s_LEAVING_IN_THE_LINE(pyrt::xrefs::xget("compose.fallback", "LEAVING_IN_THE_LINE"));
    s_LIFE_DIRECTION(pyrt::xrefs::xget("compose.fallback", "LIFE_DIRECTION"));
    s_NEUTRAL(pyrt::xrefs::xget("compose.fallback", "NEUTRAL"));
    s_NOT_A_TIPS_ASK(pyrt::xrefs::xget("compose.fallback", "NOT_A_TIPS_ASK"));
    s_NOT_NOTED(pyrt::xrefs::xget("compose.fallback", "NOT_NOTED"));
    s_POINTING_LINES(pyrt::xrefs::xget("compose.fallback", "POINTING_LINES"));
    s_REALISTIC(pyrt::xrefs::xget("compose.fallback", "REALISTIC"));
    s_REALISTIC_ABOUT_ME(pyrt::xrefs::xget("compose.fallback", "REALISTIC_ABOUT_ME"));
    s_REPAIR_BARE_IT(pyrt::xrefs::xget("compose.fallback", "REPAIR_BARE_IT"));
    s_who_settles(py_func("who_settles", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "who_settles", args)));
    s_REPAIR_DECISION(pyrt::xrefs::xget("compose.fallback", "REPAIR_DECISION"));
    s_REPAIR_PERSON(pyrt::xrefs::xget("compose.fallback", "REPAIR_PERSON"));
    s_REPAIR_RULE(pyrt::xrefs::xget("compose.fallback", "REPAIR_RULE"));
    s_REPAIR_TARIFF(pyrt::xrefs::xget("compose.fallback", "REPAIR_TARIFF"));
    s_REPAIR_TRADE(pyrt::xrefs::xget("compose.fallback", "REPAIR_TRADE"));
    s_REPAIR_WHAT_IT_IS_WORTH(pyrt::xrefs::xget("compose.fallback", "REPAIR_WHAT_IT_IS_WORTH"));
    s_REPAIR_WHETHER(pyrt::xrefs::xget("compose.fallback", "REPAIR_WHETHER"));
    s_SAME_AGAIN(pyrt::xrefs::xget("compose.fallback", "SAME_AGAIN"));
    s_SAME_AGAIN_WITH_THE_ANSWER(pyrt::xrefs::xget("compose.fallback", "SAME_AGAIN_WITH_THE_ANSWER"));
    s_SYMPTOM_WORDS(pyrt::xrefs::xget("compose.fallback", "SYMPTOM_WORDS"));
    s_TAG_CAN(pyrt::xrefs::xget("compose.fallback", "TAG_CAN"));
    s_TAG_CONFIRM(pyrt::xrefs::xget("compose.fallback", "TAG_CONFIRM"));
    s_TAG_UNSURE(pyrt::xrefs::xget("compose.fallback", "TAG_UNSURE"));
    s_TIPS_ASKS(pyrt::xrefs::xget("compose.fallback", "TIPS_ASKS"));
    s_TIPS_LINES(pyrt::xrefs::xget("compose.fallback", "TIPS_LINES"));
    s_TOO_LATE(pyrt::xrefs::xget("compose.fallback", "TOO_LATE"));
    s_UNKNOWN(pyrt::xrefs::xget("compose.fallback", "UNKNOWN"));
    s_UNKNOWN_ABOUT_A_PET(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_ABOUT_A_PET"));
    s_UNKNOWN_ABOUT_SOMEONE(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_ABOUT_SOMEONE"));
    s_UNKNOWN_DECIDE(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_DECIDE"));
    s_UNKNOWN_HEALTH(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_HEALTH"));
    s_UNKNOWN_REALISTIC(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_REALISTIC"));
    s_UNKNOWN_REALISTIC_CARE(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_REALISTIC_CARE"));
    s_UNKNOWN_TOO_LATE(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_TOO_LATE"));
    s_UNKNOWN_WORD(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORD"));
    s_UNKNOWN_WORD_NAMED(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORD_NAMED"));
    s_UNKNOWN_WORRY(pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORRY"));
    s_VIEW_AFTER_A_DEATH(pyrt::xrefs::xget("compose.fallback", "VIEW_AFTER_A_DEATH"));
    s_VIEW_ASK_LINES(pyrt::xrefs::xget("compose.fallback", "VIEW_ASK_LINES"));
    s_VIEW_COMPANY(pyrt::xrefs::xget("compose.fallback", "VIEW_COMPANY"));
    s_VIEW_DECIDE(pyrt::xrefs::xget("compose.fallback", "VIEW_DECIDE"));
    s_VIEW_HANDBACK(pyrt::xrefs::xget("compose.fallback", "VIEW_HANDBACK"));
    s_VIEW_LINES(pyrt::xrefs::xget("compose.fallback", "VIEW_LINES"));
    s_VIEW_MIRROR(pyrt::xrefs::xget("compose.fallback", "VIEW_MIRROR"));
    s_VIEW_MIRROR_LINES(pyrt::xrefs::xget("compose.fallback", "VIEW_MIRROR_LINES"));
    s_VIEW_NAMES_SOMEBODY(pyrt::xrefs::xget("compose.fallback", "VIEW_NAMES_SOMEBODY"));
    s_VIEW_NO_SUBJECT(pyrt::xrefs::xget("compose.fallback", "VIEW_NO_SUBJECT"));
    s_VIEW_ON_GOOD_NEWS(pyrt::xrefs::xget("compose.fallback", "VIEW_ON_GOOD_NEWS"));
    s_WASNT_THERE(pyrt::xrefs::xget("compose.fallback", "WASNT_THERE"));
    s_WHO_FOR_IT(pyrt::xrefs::xget("compose.fallback", "WHO_FOR_IT"));
    s__a_bare_worry(py_func("_a_bare_worry", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_bare_worry", args)));
    s__a_dead_end(py_func("_a_dead_end", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_dead_end", args)));
    s__a_plan_doubt(py_func("_a_plan_doubt", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_plan_doubt", args)));
    s__a_plan_in_the_thread(py_func("_a_plan_in_the_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_plan_in_the_thread", args)));
    s__a_price_wants_a_verdict(py_func("_a_price_wants_a_verdict", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_price_wants_a_verdict", args)));
    s__a_care_home_line(pyrt::xrefs::xget("compose.fallback", "_a_care_home_line"));
    s__a_question_about_the_person(py_func("_a_question_about_the_person", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_question_about_the_person", args)));
    s__a_repair_already_spoken_of(py_func("_a_repair_already_spoken_of", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_repair_already_spoken_of", args)));
    s__a_subject_above(py_func("_a_subject_above", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_subject_above", args)));
    s__a_switch_off_asked(py_func("_a_switch_off_asked", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_switch_off_asked", args)));
    s__a_thing_they_started(py_func("_a_thing_they_started", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_thing_they_started", args)));
    s__a_thing_was_named(py_func("_a_thing_was_named", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_a_thing_was_named", args)));
    s__advisor(py_func("_advisor", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_advisor", args)));
    s__after_a_colon(py_func("_after_a_colon", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_after_a_colon", args)));
    s__answered_already(py_func("_answered_already", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_answered_already", args)));
    s__answered_recently(py_func("_answered_recently", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_answered_recently", args)));
    s__asked_about_the_day_they_had(py_func("_asked_about_the_day_they_had", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_asked_about_the_day_they_had", args)));
    s__asks_a_name_back(py_func("_asks_a_name_back", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_asks_a_name_back", args)));
    s__asks_for_a_person_to_talk_to(py_func("_asks_for_a_person_to_talk_to", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_asks_for_a_person_to_talk_to", args)));
    s__bare_pick(py_func("_bare_pick", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_bare_pick", args)));
    s__choose(pyrt::xrefs::xget("compose.fallback", "_choose"));
    s__forecast_pool(py_func("_forecast_pool", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_forecast_pool", args)));
    s__grief_thread(py_func("_grief_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_grief_thread", args)));
    s__hedged_recently(py_func("_hedged_recently", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", args)));
    s__in_a_care_thread(py_func("_in_a_care_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_in_a_care_thread", args)));
    s__in_a_crisis_thread(py_func("_in_a_crisis_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_in_a_crisis_thread", args)));
    s__in_a_loss_thread(py_func("_in_a_loss_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_in_a_loss_thread", args)));
    s__in_a_pool(py_func("_in_a_pool", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_in_a_pool", args)));
    s__is_a_hedge(py_func("_is_a_hedge", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_is_a_hedge", args)));
    s__is_a_tips_ask(py_func("_is_a_tips_ask", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_is_a_tips_ask", args)));
    s__is_a_view_ask(py_func("_is_a_view_ask", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_is_a_view_ask", args)));
    s__name_given_before(py_func("_name_given_before", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_name_given_before", args)));
    s__names_its_own_thing(py_func("_names_its_own_thing", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_names_its_own_thing", args)));
    s__news_above(py_func("_news_above", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_news_above", args)));
    s__nothing_to_choose(py_func("_nothing_to_choose", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_nothing_to_choose", args)));
    s__nothing_to_stand_behind(py_func("_nothing_to_stand_behind", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_nothing_to_stand_behind", args)));
    s__only_a_reaction(py_func("_only_a_reaction", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_only_a_reaction", args)));
    s__only_the_floor(py_func("_only_the_floor", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_only_the_floor", args)));
    s__points_at_nothing(py_func("_points_at_nothing", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_points_at_nothing", args)));
    s__reads(pyrt::xrefs::xget("compose.fallback", "_reads"));
    s__reads_as_a_claim(py_func("_reads_as_a_claim", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_reads_as_a_claim", args)));
    s__reads_as_taste(py_func("_reads_as_taste", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_reads_as_taste", args)));
    s__says_it_already(py_func("_says_it_already", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_says_it_already", args)));
    s__the_answer_above(py_func("_the_answer_above", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_answer_above", args)));
    s__the_answer_in_reply(py_func("_the_answer_in_reply", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_answer_in_reply", args)));
    s__the_line_before(py_func("_the_line_before", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_line_before", args)));
    s__the_pool_for_a_worry_about_them(py_func("_the_pool_for_a_worry_about_them", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_pool_for_a_worry_about_them", args)));
    s__the_thread_is_heavy(py_func("_the_thread_is_heavy", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_thread_is_heavy", args)));
    s__the_view_of(py_func("_the_view_of", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_view_of", args)));
    s__the_weird_ask_pool(py_func("_the_weird_ask_pool", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_weird_ask_pool", args)));
    s__their_own_day(py_func("_their_own_day", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_their_own_day", args)));
    s__thread_is_about_a_person(py_func("_thread_is_about_a_person", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_a_person", args)));
    s__thread_is_about_a_pet(py_func("_thread_is_about_a_pet", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_a_pet", args)));
    s__thread_is_about_someone_else(py_func("_thread_is_about_someone_else", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_someone_else", args)));
    s__offers_to_fix_themselves(py_func("_offers_to_fix_themselves", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_offers_to_fix_themselves", args)));
    s__thread_is_about_the_body(py_func("_thread_is_about_the_body", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_the_body", args)));
    s__thread_names_a_machine(py_func("_thread_names_a_machine", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_thread_names_a_machine", args)));
    s__tips(py_func("_tips", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_tips", args)));
    s__who_for_the_thread(py_func("_who_for_the_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_who_for_the_thread", args)));
    s_ack(pyrt::xrefs::xget("compose.fallback", "ack"));
    s_asks_for_more(py_func("asks_for_more", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "asks_for_more", args)));
    s_finding_out(pyrt::xrefs::xget("compose.fallback", "finding_out"));
    s_in_a_safety_thread(py_func("in_a_safety_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "in_a_safety_thread", args)));
    s_no_eyes(pyrt::xrefs::xget("compose.fallback", "no_eyes"));
    s_reads_as_if_i_saw(py_func("reads_as_if_i_saw", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "reads_as_if_i_saw", args)));
    s_the_lost_ask_reply(py_func("the_lost_ask_reply", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "the_lost_ask_reply", args)));
    s_unknown_pool(py_func("unknown_pool", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "unknown_pool", args)));
    s_MISSED_STEP(stdlib_call("re", "compile", &[py_str("\\bwhat (?:if|happens if) (?:i|we|you) (?:miss|skip|dont do|cant do|cannot do)\\b|\\bwhat (?:if|happens if) (?:i|we) (?:have to )?(?:miss|skip)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSED_STEP_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|one|two|three|four|\\d+|a couple of|a few|couple of|few)?\\s*(?:days?|weeks?|fortnight|months?|sessions?|runs?|rides?|swims?|classes?|lessons?|workouts?|practice|training)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSED_STEP_KEEPS(stdlib_call("re", "compile", &[py_str("\\b(?:pills?|doses?|drops|tablets?|meds|medication|medications|prescriptions?|insulin|inhalers?|injections?|jabs?|appointments?|periods?|contracepti\\w*|antidepressants?|antibiotics?|lenses?|dressing|dressings)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_repeat_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut sentences: Py = Py::None;
    let _ = py_str("The answer the person is asking for again, short enough to say again.\n\n    The same guard has two forms and only one of them answers. \"Same answer, and it covers this:\n    <card>\" carries what it names; \"that is everything I have on that one, so I would only repeat\n    myself\" names the answer on the screen and then hands the person none of it. Ask twice in\n    other words - \"can i leave my bags at the hotel\", then \"can i leave my case at the hotel\" -\n    and that is the turn they get, on a card that answers both.\n\n    So the answer above is carried, cut to its first two sentences when the card ran longer: the\n    repeat is the same answer, and the person asked for it.\n    ");
    answer = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("compose.fallback", "_the_answer_above", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_str("");
    }
    sentences = stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), answer.clone()]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&op_gt(&call_builtin("len", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { mcall(&py_str(" "), "join", &[slice_or_raise(&sentences.clone(), &Py::None, &Py::Int(2), &Py::None)]) } else { answer.clone() };
    Py::None
}
pub fn _drafting_pool(args: &[Py]) -> Py {
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
    let _ = py_str("The drafting invite, without asking for what the line already said.\n\n    \"Give me who it's for\" is the right question after \"can you write a text for me\", and the\n    wrong one after \"can you send a text to my landlord\" - the person named the landlord in the\n    same breath. So the pool is picked on the shape: when an addressee is already named, the\n    invite asks for the gist and the tone, which is what is actually missing.\n    ");
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "no_eyes"), "names_the_addressee", &[clean.clone()])) {
    return pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORD_NAMED");
    }
    return pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORD");
    Py::None
}
pub fn _a_care_thread(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { Py::Int(3) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last few things the person said were about somebody being looked after.\n\n    Read from the thread rather than the line, because the line is the bare ask: \"my dog is\n    having an operation tomorrow\", \"im worried\", \"how long will it take\". A rule of thumb about\n    a roast is not one about a wait at a vet or a ward, and the thread says so when the line\n    cannot.\n    ");
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_a_pet", &[memory.clone()])) {
    return Py::Bool(true);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.the_rule_for_it", "A_CARE_WORD"), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_money_thread(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { Py::Int(3) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last few things the person said were about money they owe or are owed.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.the_rule_for_it", "A_MONEY_WORD"), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_rule_for_it_for_the_turn(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut pool: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What settles this kind of ask, said before where the rest of it is kept, or None.\n\n    Three kinds have a rule the bot can always give: a price (where you buy it, the size, the\n    day - and a job is priced by the quote), an amount (a portion per head, times the count) and\n    a keeping question (how long it has been, and how cold). The pools live in\n    `the_rule_for_it.py` so this file stays the reader it was.\n\n    What stands down: a thread that is a crisis, a loss or care; a line naming a body, a symptom\n    or a health word, because a rule of thumb about a portion is not one about a dose; and a\n    thread that is about a body, for the same reason.\n    ");
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "in_a_safety_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_grief_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_in_a_loss_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_in_a_care_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_the_body", &[memory.clone()]); if py_truthy(&__b) { __b } else { _a_care_thread(&[memory.clone()]) } } } } } } } } } }) {
    return Py::None;
    }
    words = op_bitor(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = repaired.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&op_bitand(&words.clone(), &op_bitor(&op_bitor(&pyrt::xrefs::xget("compose.fallback", "BODY_PARTS"), &pyrt::xrefs::xget("compose.fallback", "SYMPTOM_WORDS")), &pyrt::xrefs::xget("compose.fallback", "HEALTH_WORDS")))) {
    return Py::None;
    }
    pool = pyrt::xrefs::xcall("compose.the_rule_for_it", "the_rule_for_it", &[clean.clone(), repaired.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.the_rule_for_it", "TIME_RULE"))); if !py_truthy(&__b) { __b } else { _a_money_thread(&[memory.clone()]) } }) {
    return Py::None;
    }
    return pool.clone();
    Py::None
}
pub fn _a_missed_step(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut words: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut plan: Py = Py::None;
    let _ = py_str("The pool for a stepped plan with one step missed, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_MISSED_STEP(), "search", &[clean.clone()])))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&repaired.clone(), &clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MISSED_STEP(), "search", &[repaired.clone()]))) } }) {
    return Py::None;
    }
    }
    plan = { let __b = mcall(&g_MISSED_STEP_PLAN(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MISSED_STEP_PLAN(), "search", &[repaired.clone()]) } };
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_MISSED_STEP_KEEPS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MISSED_STEP_KEEPS(), "search", &[repaired.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("compose.fallback", "HEALTH_WORDS")); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("compose.fallback", "BODY_PARTS")) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "in_a_safety_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_grief_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_in_a_loss_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_the_body", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_a_pet", &[memory.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_someone_else", &[memory.clone()]) } } } } } } } } } }) {
    return Py::None;
    }
    return pyrt::xrefs::xget("compose.fallback_data", "MISSED_STEP_LINES");
    Py::None
}
pub fn _unknown(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut about_a_person: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut body_lines: Py = Py::None;
    let mut cannot_see: Py = Py::None;
    let mut care: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut day: Py = Py::None;
    let mut described: Py = Py::None;
    let mut for_the_thread: Py = Py::None;
    let mut forecast: Py = Py::None;
    let mut given: Py = Py::None;
    let mut how_line: Py = Py::None;
    let mut how_to: Py = Py::None;
    let mut in_the_room: Py = Py::None;
    let mut line: Py = Py::None;
    let mut line_words: Py = Py::None;
    let mut lost: Py = Py::None;
    let mut method: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut missed: Py = Py::None;
    let mut named: Py = Py::None;
    let mut no_thing_named: Py = Py::None;
    let mut own: Py = Py::None;
    let mut own_ask: Py = Py::None;
    let mut own_day: Py = Py::None;
    let mut own_thing: Py = Py::None;
    let mut pick: Py = Py::None;
    let mut place: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut price: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut routed: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut switched: Py = Py::None;
    let mut tips: Py = Py::None;
    let mut w: Py = Py::None;
    let mut waited: Py = Py::None;
    let mut weird: Py = Py::None;
    let mut words: Py = Py::None;
    let mut worry: Py = Py::None;
    let _ = py_str("An honest reply to a question nothing can answer: name who can, when it is clear.");
    repaired = { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { clean.clone() } };
    own_day = pyrt::xrefs::xcall("compose.fallback", "_their_own_day", &[clean.clone()]);
    if py_truthy(&own_day.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[own_day.clone(), memory.clone(), clean.clone()]);
    }
    body_lines = mcall(&pyrt::xrefs::xget("compose.fallback", "no_eyes"), "look_at_a_body_lines", &[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&body_lines.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[body_lines.clone(), memory.clone(), clean.clone()]);
    }
    cannot_see = mcall(&pyrt::xrefs::xget("compose.fallback", "no_eyes"), "cannot_see_your_thing", &[clean.clone()]);
    if py_truthy(&cannot_see.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[cannot_see.clone(), memory.clone(), clean.clone()]);
    }
    described = mcall(&pyrt::xrefs::xget("compose.fallback", "no_eyes"), "what_they_described", &[clean.clone(), memory.clone()]);
    if py_truthy(&described.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[described.clone(), memory.clone(), clean.clone()]);
    }
    in_the_room = mcall(&pyrt::xrefs::xget("compose.fallback", "no_eyes"), "a_thing_in_the_room", &[clean.clone()]);
    if py_truthy(&in_the_room.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[in_the_room.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "no_eyes"), "writes_the_message", &[clean.clone()])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[_drafting_pool(&[clean.clone()]), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "_in_a_crisis_thread", &[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_asks_for_a_person_to_talk_to", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "CRISIS_ASK"), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("compose.fallback", "WHO_FOR_IT"), "match", &[clean.clone()]) } } } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "A_PERSON_IN_A_CRISIS"), memory.clone(), clean.clone()]);
    }
    lost = pyrt::xrefs::xcall("compose.fallback", "the_lost_ask_reply", &[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&lost.clone()) {
    return lost.clone();
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_asks_a_name_back", &[clean.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }])) {
    given = pyrt::xrefs::xcall("compose.fallback", "_name_given_before", &[clean.clone(), memory.clone()]);
    if py_truthy(&given.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("compose.fallback", "HANDED_BACK")) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("name".to_string(), given.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), clean.clone()]);
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "NOT_NOTED"), memory.clone(), clean.clone()]);
    }
    forecast = pyrt::xrefs::xcall("compose.fallback", "_forecast_pool", &[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&forecast.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[forecast.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "reads_as_if_i_saw", &[clean.clone(), memory.clone()])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "ASKED_IF_I_SAW_LINES"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "_asked_about_the_day_they_had", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "in_a_safety_thread", &[memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_grief_thread", &[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_in_a_loss_thread", &[memory.clone()]))) } } } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "WASNT_THERE"), memory.clone(), clean.clone()]);
    }
    how_to = mcall(&pyrt::xrefs::xget("compose.fallback", "HOW_TO_DECIDE"), "search", &[clean.clone()]);
    how_line = clean.clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&how_to.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    how_to = mcall(&pyrt::xrefs::xget("compose.fallback", "HOW_TO_DECIDE"), "search", &[repaired.clone()]);
    how_line = repaired.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&how_to.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "LIFE_DIRECTION"), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_nothing_to_choose", &[slice_or_raise(&how_line.clone(), &mcall(&how_to.clone(), "end", &[]), &Py::None, &Py::None)]) } } } }) {
    said_words = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9']"), py_str(" "), how_line.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    plain = mcall(&mcall(&said_words.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("compose.fallback", "HEALTH_WORDS")); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&call_builtin("set", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("compose.fallback", "EXPERT_WORDS")); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&call_builtin("set", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("compose.fallback", "BODY_PARTS")); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("compose.fallback", "ANIMALS"), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("compose.fallback", "ANIMALS"), &mcall(&w.clone(), "rstrip", &[py_str("s")]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } }))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "HOW_TO_DECIDE_LINES"), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_reads_as_taste", &[clean.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "UNKNOWN"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "REPAIR_WHETHER"), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "REPAIR_WHAT_IT_IS_WORTH"), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "who_settles", &[clean.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_a_repair_already_spoken_of", &[memory.clone()]))) } } } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback_data", "REPAIR_WHETHER_LINES"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "REPAIR_DECISION"), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "REPAIR_TRADE"), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "REPAIR_PERSON"), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "LEAVING_IN_THE_LINE"), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_a_person", &[memory.clone()]) } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "REPAIR_TARIFF"), "search"), clean.clone(), repaired.clone()]))) } } } } } } } }) {
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "in_a_safety_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_grief_thread", &[memory.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_in_a_loss_thread", &[memory.clone()]) } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "CARE_HERE"), memory.clone(), clean.clone()]);
    }
    own_thing = { let __b = pyrt::xrefs::xcall("compose.fallback", "_names_its_own_thing", &[clean.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_names_its_own_thing", &[repaired.clone()]) } };
    line_words = op_bitor(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), repaired.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    no_thing_named = { let __b = py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "REPAIR_BARE_IT"), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_thread_names_a_machine", &[memory.clone()]))) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&own_thing.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let __b = pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_the_body", &[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_thread_names_a_machine", &[memory.clone()]))) } }; if py_truthy(&__b) { __b } else { op_bitand(&line_words.clone(), &op_bitor(&op_bitor(&pyrt::xrefs::xget("compose.fallback", "BODY_PARTS"), &pyrt::xrefs::xget("compose.fallback", "SYMPTOM_WORDS")), &pyrt::xrefs::xget("compose.fallback", "HEALTH_WORDS"))) } } } }) {
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_offers_to_fix_themselves", &[clean.clone()])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "BODY_IS_NOT_A_MACHINE"), memory.clone(), clean.clone()]);
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "UNKNOWN_HEALTH"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&no_thing_named.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_a_person", &[memory.clone()]))); if py_truthy(&__b) { __b } else { { let __b = own_thing.clone(); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_a_thing_was_named", &[memory.clone()]) } } } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "REPAIR_RULE"), memory.clone(), clean.clone()]);
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "UNKNOWN_DECIDE"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "TOO_LATE"), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_a_thing_they_started", &[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_the_body", &[memory.clone()]))) } } } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "UNKNOWN_TOO_LATE"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = { let __b = pyrt::xrefs::xcall("compose.fallback", "_a_plan_doubt", &[gattr(&pyrt::xrefs::xget("compose.fallback", "REALISTIC"), "search"), clean.clone(), repaired.clone()]); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_a_plan_doubt", &[gattr(&pyrt::xrefs::xget("compose.fallback", "REALISTIC_ABOUT_ME"), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_a_plan_in_the_thread", &[clean.clone(), memory.clone()]) } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()]))) } }) {
    pool = if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_the_thread_is_heavy", &[memory.clone()])) { pyrt::xrefs::xget("compose.fallback", "UNKNOWN_REALISTIC_CARE") } else { pyrt::xrefs::xget("compose.fallback", "UNKNOWN_REALISTIC") };
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pool.clone(), memory.clone(), clean.clone(), op_add(&op_add(&clean.clone(), &py_str(" ")), &pyrt::xrefs::xcall("compose.fallback", "_the_line_before", &[memory.clone()]))]);
    }
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9']"), py_str(" "), clean.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    words = mcall(&mcall(&said.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[]);
    waited = pyrt::xrefs::xcall("compose.waited_for", "for_the_ask", &[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&waited.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&waited.clone(), &pyrt::xrefs::xget("compose.waited_for", "NOTHING_HELD"))); if !py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("compose.fallback", "BARE_FOLLOW_UP"), "match", &[clean.clone()]) } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[waited.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "BARE_FOLLOW_UP"), "match", &[clean.clone()])) {
    pool = if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_the_thread_is_heavy", &[memory.clone()])) { pyrt::xrefs::xget("compose.fallback", "BARE_FOLLOW_UP_CARE_REPLIES") } else { pyrt::xrefs::xget("compose.fallback", "BARE_FOLLOW_UP_REPLIES") };
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pool.clone(), memory.clone(), clean.clone(), op_add(&op_add(&clean.clone(), &py_str(" ")), &pyrt::xrefs::xcall("compose.fallback", "_the_line_before", &[memory.clone()]))]);
    }
    switched = pyrt::xrefs::xcall("compose.fallback", "_a_switch_off_asked", &[memory.clone(), clean.clone()]);
    if py_truthy(&switched.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[switched.clone(), memory.clone(), op_add(&op_add(&clean.clone(), &py_str(" ")), &pyrt::xrefs::xcall("compose.fallback", "_the_line_before", &[memory.clone()]))]);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_points_at_nothing", &[clean.clone(), memory.clone(), repaired.clone()])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "POINTING_LINES"), memory.clone(), clean.clone()]);
    }
    weird = if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()])))) { pyrt::xrefs::xcall("compose.fallback", "_the_weird_ask_pool", &[clean.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_is(&weird.clone(), &Py::None))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[weird.clone(), memory.clone(), clean.clone()]);
    }
    pool = pyrt::xrefs::xcall("compose.fallback", "unknown_pool", &[clean.clone(), words.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9']"), py_str(" "), repaired.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    words = mcall(&mcall(&said.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[]);
    pool = pyrt::xrefs::xcall("compose.fallback", "unknown_pool", &[repaired.clone(), words.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_DECIDE"))) } }) {
    missed = _a_missed_step(&[clean.clone(), repaired.clone(), memory.clone(), words.clone()]);
    if py_truthy(&Py::Bool(!py_is(&missed.clone(), &Py::None))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[missed.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORRY"))); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_a_bare_worry", &[clean.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_the_body", &[memory.clone()]) } } } }) {
    pool = pyrt::xrefs::xget("compose.fallback", "UNKNOWN_HEALTH");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORRY"))); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_a_bare_worry", &[clean.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_a_pet", &[memory.clone()]) } } } }) {
    pool = pyrt::xrefs::xget("compose.fallback", "UNKNOWN_ABOUT_A_PET");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORRY"))); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fallback", "_a_bare_worry", &[clean.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_someone_else", &[memory.clone()]) } } } }) {
    pool = pyrt::xrefs::xget("compose.fallback", "UNKNOWN_ABOUT_SOMEONE");
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_DECIDE"))) } }) {
    if py_truthy(&{ let __b = waited.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&waited.clone(), &pyrt::xrefs::xget("compose.waited_for", "NOTHING_HELD"))) } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[waited.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORRY"))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()]))) } }) {
    worry = pyrt::xrefs::xcall("compose.fallback", "_the_pool_for_a_worry_about_them", &[clean.clone(), memory.clone()]);
    if py_truthy(&worry.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[worry.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORD")))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[_drafting_pool(&[clean.clone()]), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "asks_for_more", &[said.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()]))) } }) {
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_answered_recently", &[memory.clone()])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "HELP_ALREADY"), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN")))) {
    routed = mcall(&pyrt::xrefs::xget("compose.fallback", "finding_out"), "for_the_ask", &[clean.clone(), repaired.clone(), understood.clone()]);
    if py_truthy(&routed.clone()) {
    said = py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[routed.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    day = pyrt::xrefs::xcall("compose.for_the_day", "for_the_ask", &[clean.clone(), repaired.clone()]);
    if py_truthy(&day.clone()) {
    said = py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[day.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    named = pyrt::xrefs::xcall("compose.fallback", "_advisor", &[memory.clone(), clean.clone(), said.clone(), words.clone()]);
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    for_the_thread = pyrt::xrefs::xcall("compose.fallback", "_who_for_the_thread", &[memory.clone(), clean.clone(), understood.clone()]);
    if py_truthy(&for_the_thread.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[for_the_thread.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[py_func("_is_a_tips_ask", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_is_a_tips_ask", args)), clean.clone(), repaired.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("help_ask")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "NOT_A_TIPS_ASK"), "search", &[clean.clone()]))) } } } }) {
    tips = pyrt::xrefs::xcall("compose.fallback", "_tips", &[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&tips.clone()) {
    return tips.clone();
    }
    if py_truthy(&{ let __b = mcall(&understood.clone(), "get", &[py_str("help_ask")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("compose.fallback", "_reads"), &[gattr(&pyrt::xrefs::xget("compose.fallback", "TIPS_ASKS"), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()]) } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_answered_already", &[memory.clone()])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "HELP_ALREADY"), memory.clone(), clean.clone()]);
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "TIPS_LINES"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("tag")])) {
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "_answered_already", &[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "TAG_CAN"), "search", &[clean.clone()]))) } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "TAG_CONFIRM"), memory.clone(), clean.clone()]);
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "TAG_UNSURE"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_says_it_already", &[clean.clone(), call_builtin("list", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]) } }) {
    answer = _the_repeat_answer(&[memory.clone()]);
    if py_truthy(&answer.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("answer".to_string(), answer.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "SAME_AGAIN_WITH_THE_ANSWER"), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "SAME_AGAIN"), memory.clone(), clean.clone()]);
    }
    pick = { let __b = pyrt::xrefs::xcall("compose.fallback", "_bare_pick", &[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_bare_pick", &[repaired.clone(), memory.clone()]) } };
    if py_truthy(&pick.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pick.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "_a_dead_end", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_in_a_care_thread", &[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "_news_above", &[memory.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_a_subject_above", &[memory.clone()]) } })) } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "DITHER_OPEN_LINES"), memory.clone(), clean.clone()]);
    }
    about_a_person = pyrt::xrefs::xcall("compose.fallback", "_a_question_about_the_person", &[clean.clone(), memory.clone()]);
    if py_truthy(&about_a_person.clone()) {
    return about_a_person.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "A_STEP_ASK"), "match", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_news_above", &[memory.clone()]) } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "BARE_PICK_AFTER_NEWS"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN")))) {
    care = py_call(&pyrt::xrefs::xget("compose.fallback", "_a_care_home_line"), &[understood.clone(), memory.clone()]);
    if py_truthy(&care.clone()) {
    return care.clone();
    }
    }
    price = pyrt::xrefs::xcall("compose.fallback", "_a_price_wants_a_verdict", &[clean.clone(), memory.clone()]);
    if py_truthy(&{ let __b = price.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))) } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[price.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN")))) {
    mine = pyrt::xrefs::xcall("compose.fallback_data", "the_bots_own_answer", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&mine.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    mine = pyrt::xrefs::xcall("compose.fallback_data", "the_bots_own_answer", &[repaired.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&mine.clone(), &Py::None))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[mine.clone(), memory.clone(), clean.clone()]);
    }
    rule = _the_rule_for_it_for_the_turn(&[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&rule.clone(), &Py::None))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[rule.clone(), memory.clone(), clean.clone()]);
    }
    place = if py_truthy(&pyrt::xrefs::xcall("compose.findout", "reads_as_crowd", &[clean.clone()])) { Py::None } else { pyrt::xrefs::xcall("compose.fallback_data", "the_place_the_answer_is_kept", &[clean.clone()]) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&place.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.findout", "reads_as_crowd", &[clean.clone()]))) } } } }) {
    place = pyrt::xrefs::xcall("compose.fallback_data", "the_place_the_answer_is_kept", &[repaired.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&place.clone(), &Py::None))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[place.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN"))); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_WORRY"))) } }) {
    own_ask = pyrt::xrefs::xmodule("compose.own_ask");
    own = mcall(&own_ask.clone(), "answer", &[clean.clone(), memory.clone()]);
    if py_truthy(&own.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[py_list(vec![own.clone()]), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN")))) {
    method = pyrt::xrefs::xcall("compose.fallback", "method_ask_pool", &[clean.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&method.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    method = pyrt::xrefs::xcall("compose.fallback", "method_ask_pool", &[repaired.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&method.clone(), &Py::None))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[method.clone(), memory.clone(), clean.clone()]);
    }
    }
    counted = _a_countdown_in_a_care_thread_(&[memory.clone(), clean.clone(), repaired.clone(), understood.clone()]);
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _a_countdown_in_a_care_thread_(args: &[Py]) -> Py {
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
    let mut _a_countdown_in_a_care_thread: Py = Py::None;
    let _ = py_str("The countdown branch, imported on the first call to keep the two modules apart.");
    _a_countdown_in_a_care_thread = py_func("_a_countdown_in_a_care_thread", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback_chain", "_a_countdown_in_a_care_thread", args));
    return py_call(&_a_countdown_in_a_care_thread, &[memory.clone(), clean.clone(), repaired.clone(), understood.clone()]);
    Py::None
}
pub fn _the_view_asked_for(args: &[Py]) -> Py {
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
    let mut answered: Py = Py::None;
    let mut first: Py = Py::None;
    let mut headline: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut mirrored: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's own view, given back, when that is all the line asks for.\n\n    Only one turn back, and only when that turn answered something: after a hedge there is no\n    view to give the person, and \"my view has not changed\" on the back of \"I don't know enough\"\n    is a worse non-answer than the hedge itself.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_is_a_view_ask", &[clean.clone()])))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "VIEW_NAMES_SOMEBODY"), "search", &[clean.clone()])) {
    return Py::None;
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "VIEW_NO_SUBJECT"), memory.clone(), clean.clone()]);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_only_the_floor", &[said.clone()])) {
    answered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&t.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { op_lt(&mcall(&turns.clone(), "index", &[t.clone()]), &mcall(&turns.clone(), "index", &[last.clone()])) } } } } } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&answered.clone()))) {
    return Py::None;
    }
    last = geti(&answered.clone(), &op_neg(&Py::Int(1)));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_nothing_to_stand_behind", &[said.clone()])) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "ack"), "reads_as_a_receipt", &[said.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_in_a_pool", &[said.clone(), py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "NEUTRAL")])]) } }))) {
    said = { let __b = pyrt::xrefs::xcall("compose.fallback", "_the_answer_in_reply", &[mcall(&last.clone(), "get", &[py_str("user")]), said.clone()]); if py_truthy(&__b) { __b } else { said.clone() } };
    }
    while py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "ack"), "reads_as_a_receipt", &[said.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_in_a_pool", &[said.clone(), py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "NEUTRAL")])]) } }) {
    answered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&t.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { op_lt(&mcall(&turns.clone(), "index", &[t.clone()]), &mcall(&turns.clone(), "index", &[last.clone()])) } } } } } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&answered.clone()))) {
    return Py::None;
    }
    last = geti(&answered.clone(), &op_neg(&Py::Int(1)));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_only_a_reaction", &[mcall(&last.clone(), "get", &[py_str("module")]), said.clone()])) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "VIEW_ON_GOOD_NEWS"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("compose.fallback", "CARE_MODULES"), &call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("loss"), py_str("grief")]), &call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "VIEW_AFTER_A_DEATH"), memory.clone(), clean.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("compose.fallback", "VIEW_HANDBACK")) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&said.clone(), "lower", &[]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "VIEW_DECIDE"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_is_a_hedge", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "VIEW_MIRROR"), "match", &[mcall(&said.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_reads_as_a_claim", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&pyrt::xrefs::xcall("compose.fallback", "_the_view_of", &[said.clone()]), &Py::Int(0)))) } } } }) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "VIEW_COMPANY"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_reads_as_a_claim", &[said.clone()])))) {
    first = mcall(&mcall(&geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&said.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "VIEW_MIRROR"), "match", &[first.clone()]); if !py_truthy(&__b) { __b } else { geti(&pyrt::xrefs::xcall("compose.fallback", "_the_view_of", &[said.clone()]), &Py::Int(0)) } }))) {
    return Py::None;
    }
    }
    let __unpk1 = py_unpack(&pyrt::xrefs::xcall("compose.fallback", "_the_view_of", &[said.clone()]), 2, None);
    headline = __unpk1[0].clone();
    mirrored = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&headline.clone()))) {
    if py_truthy(&mirrored.clone()) {
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pyrt::xrefs::xget("compose.fallback", "VIEW_COMPANY"), memory.clone(), clean.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&mcall(&headline.clone(), "endswith", &[py_str("?")])) {
    lines = pyrt::xrefs::xget("compose.fallback", "VIEW_ASK_LINES");
    } else {
    if py_truthy(&mirrored.clone()) {
    lines = pyrt::xrefs::xget("compose.fallback", "VIEW_MIRROR_LINES");
    } else {
    lines = pyrt::xrefs::xget("compose.fallback", "VIEW_LINES");
    }
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), pyrt::xrefs::xcall("compose.fallback", "_after_a_colon", &[headline.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.fallback_pools".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANIMALS" => Some(g_ANIMALS()),
            "ASKED_IF_I_SAW_LINES" => Some(g_ASKED_IF_I_SAW_LINES()),
            "A_CARE_WORD" => Some(g_A_CARE_WORD()),
            "A_MONEY_WORD" => Some(g_A_MONEY_WORD()),
            "A_PERSON_IN_A_CRISIS" => Some(g_A_PERSON_IN_A_CRISIS()),
            "A_STEP_ASK" => Some(g_A_STEP_ASK()),
            "BARE_FOLLOW_UP" => Some(g_BARE_FOLLOW_UP()),
            "BARE_FOLLOW_UP_CARE_REPLIES" => Some(g_BARE_FOLLOW_UP_CARE_REPLIES()),
            "BARE_FOLLOW_UP_REPLIES" => Some(g_BARE_FOLLOW_UP_REPLIES()),
            "BARE_PICK_AFTER_NEWS" => Some(g_BARE_PICK_AFTER_NEWS()),
            "BODY_IS_NOT_A_MACHINE" => Some(g_BODY_IS_NOT_A_MACHINE()),
            "BODY_PARTS" => Some(g_BODY_PARTS()),
            "CARE_HERE" => Some(g_CARE_HERE()),
            "CARE_MODULES" => Some(g_CARE_MODULES()),
            "CRISIS_ASK" => Some(g_CRISIS_ASK()),
            "DITHER_OPEN_LINES" => Some(g_DITHER_OPEN_LINES()),
            "EXPERT_WORDS" => Some(g_EXPERT_WORDS()),
            "HANDED_BACK" => Some(g_HANDED_BACK()),
            "HEALTH_WORDS" => Some(g_HEALTH_WORDS()),
            "HELP_ALREADY" => Some(g_HELP_ALREADY()),
            "HOW_TO_DECIDE" => Some(g_HOW_TO_DECIDE()),
            "HOW_TO_DECIDE_LINES" => Some(g_HOW_TO_DECIDE_LINES()),
            "LEAVING_IN_THE_LINE" => Some(g_LEAVING_IN_THE_LINE()),
            "LIFE_DIRECTION" => Some(g_LIFE_DIRECTION()),
            "MISSED_STEP" => Some(g_MISSED_STEP()),
            "MISSED_STEP_KEEPS" => Some(g_MISSED_STEP_KEEPS()),
            "MISSED_STEP_LINES" => Some(g_MISSED_STEP_LINES()),
            "MISSED_STEP_PLAN" => Some(g_MISSED_STEP_PLAN()),
            "NEUTRAL" => Some(g_NEUTRAL()),
            "NOT_A_TIPS_ASK" => Some(g_NOT_A_TIPS_ASK()),
            "NOT_NOTED" => Some(g_NOT_NOTED()),
            "POINTING_LINES" => Some(g_POINTING_LINES()),
            "REALISTIC" => Some(g_REALISTIC()),
            "REALISTIC_ABOUT_ME" => Some(g_REALISTIC_ABOUT_ME()),
            "REPAIR_BARE_IT" => Some(g_REPAIR_BARE_IT()),
            "REPAIR_DECISION" => Some(g_REPAIR_DECISION()),
            "REPAIR_PERSON" => Some(g_REPAIR_PERSON()),
            "REPAIR_RULE" => Some(g_REPAIR_RULE()),
            "REPAIR_TARIFF" => Some(g_REPAIR_TARIFF()),
            "REPAIR_TRADE" => Some(g_REPAIR_TRADE()),
            "REPAIR_WHAT_IT_IS_WORTH" => Some(g_REPAIR_WHAT_IT_IS_WORTH()),
            "REPAIR_WHETHER" => Some(g_REPAIR_WHETHER()),
            "REPAIR_WHETHER_LINES" => Some(g_REPAIR_WHETHER_LINES()),
            "SAME_AGAIN" => Some(g_SAME_AGAIN()),
            "SAME_AGAIN_WITH_THE_ANSWER" => Some(g_SAME_AGAIN_WITH_THE_ANSWER()),
            "SYMPTOM_WORDS" => Some(g_SYMPTOM_WORDS()),
            "TAG_CAN" => Some(g_TAG_CAN()),
            "TAG_CONFIRM" => Some(g_TAG_CONFIRM()),
            "TAG_UNSURE" => Some(g_TAG_UNSURE()),
            "TIME_RULE" => Some(g_TIME_RULE()),
            "TIPS_ASKS" => Some(g_TIPS_ASKS()),
            "TIPS_LINES" => Some(g_TIPS_LINES()),
            "TOO_LATE" => Some(g_TOO_LATE()),
            "UNKNOWN" => Some(g_UNKNOWN()),
            "UNKNOWN_ABOUT_A_PET" => Some(g_UNKNOWN_ABOUT_A_PET()),
            "UNKNOWN_ABOUT_SOMEONE" => Some(g_UNKNOWN_ABOUT_SOMEONE()),
            "UNKNOWN_DECIDE" => Some(g_UNKNOWN_DECIDE()),
            "UNKNOWN_HEALTH" => Some(g_UNKNOWN_HEALTH()),
            "UNKNOWN_REALISTIC" => Some(g_UNKNOWN_REALISTIC()),
            "UNKNOWN_REALISTIC_CARE" => Some(g_UNKNOWN_REALISTIC_CARE()),
            "UNKNOWN_TOO_LATE" => Some(g_UNKNOWN_TOO_LATE()),
            "UNKNOWN_WORD" => Some(g_UNKNOWN_WORD()),
            "UNKNOWN_WORD_NAMED" => Some(g_UNKNOWN_WORD_NAMED()),
            "UNKNOWN_WORRY" => Some(g_UNKNOWN_WORRY()),
            "VIEW_AFTER_A_DEATH" => Some(g_VIEW_AFTER_A_DEATH()),
            "VIEW_ASK_LINES" => Some(g_VIEW_ASK_LINES()),
            "VIEW_COMPANY" => Some(g_VIEW_COMPANY()),
            "VIEW_DECIDE" => Some(g_VIEW_DECIDE()),
            "VIEW_HANDBACK" => Some(g_VIEW_HANDBACK()),
            "VIEW_LINES" => Some(g_VIEW_LINES()),
            "VIEW_MIRROR" => Some(g_VIEW_MIRROR()),
            "VIEW_MIRROR_LINES" => Some(g_VIEW_MIRROR_LINES()),
            "VIEW_NAMES_SOMEBODY" => Some(g_VIEW_NAMES_SOMEBODY()),
            "VIEW_NO_SUBJECT" => Some(g_VIEW_NO_SUBJECT()),
            "VIEW_ON_GOOD_NEWS" => Some(g_VIEW_ON_GOOD_NEWS()),
            "WASNT_THERE" => Some(g_WASNT_THERE()),
            "WHO_FOR_IT" => Some(g_WHO_FOR_IT()),
            "_a_bare_worry" => Some(g__a_bare_worry()),
            "_a_care_home_line" => Some(g__a_care_home_line()),
            "_a_care_thread" => Some(py_func("_a_care_thread", |args: &[Py]| _a_care_thread(args))),
            "_a_countdown_in_a_care_thread_" => Some(py_func("_a_countdown_in_a_care_thread_", |args: &[Py]| _a_countdown_in_a_care_thread_(args))),
            "_a_dead_end" => Some(g__a_dead_end()),
            "_a_missed_step" => Some(py_func("_a_missed_step", |args: &[Py]| _a_missed_step(args))),
            "_a_money_thread" => Some(py_func("_a_money_thread", |args: &[Py]| _a_money_thread(args))),
            "_a_plan_doubt" => Some(g__a_plan_doubt()),
            "_a_plan_in_the_thread" => Some(g__a_plan_in_the_thread()),
            "_a_price_wants_a_verdict" => Some(g__a_price_wants_a_verdict()),
            "_a_question_about_the_person" => Some(g__a_question_about_the_person()),
            "_a_repair_already_spoken_of" => Some(g__a_repair_already_spoken_of()),
            "_a_subject_above" => Some(g__a_subject_above()),
            "_a_switch_off_asked" => Some(g__a_switch_off_asked()),
            "_a_thing_they_started" => Some(g__a_thing_they_started()),
            "_a_thing_was_named" => Some(g__a_thing_was_named()),
            "_advisor" => Some(g__advisor()),
            "_after_a_colon" => Some(g__after_a_colon()),
            "_answered_already" => Some(g__answered_already()),
            "_answered_recently" => Some(g__answered_recently()),
            "_asked_about_the_day_they_had" => Some(g__asked_about_the_day_they_had()),
            "_asks_a_name_back" => Some(g__asks_a_name_back()),
            "_asks_for_a_person_to_talk_to" => Some(g__asks_for_a_person_to_talk_to()),
            "_bare_pick" => Some(g__bare_pick()),
            "_choose" => Some(g__choose()),
            "_drafting_pool" => Some(py_func("_drafting_pool", |args: &[Py]| _drafting_pool(args))),
            "_forecast_pool" => Some(g__forecast_pool()),
            "_grief_thread" => Some(g__grief_thread()),
            "_hedged_recently" => Some(g__hedged_recently()),
            "_in_a_care_thread" => Some(g__in_a_care_thread()),
            "_in_a_crisis_thread" => Some(g__in_a_crisis_thread()),
            "_in_a_loss_thread" => Some(g__in_a_loss_thread()),
            "_in_a_pool" => Some(g__in_a_pool()),
            "_is_a_hedge" => Some(g__is_a_hedge()),
            "_is_a_tips_ask" => Some(g__is_a_tips_ask()),
            "_is_a_view_ask" => Some(g__is_a_view_ask()),
            "_name_given_before" => Some(g__name_given_before()),
            "_names_its_own_thing" => Some(g__names_its_own_thing()),
            "_news_above" => Some(g__news_above()),
            "_nothing_to_choose" => Some(g__nothing_to_choose()),
            "_nothing_to_stand_behind" => Some(g__nothing_to_stand_behind()),
            "_offers_to_fix_themselves" => Some(g__offers_to_fix_themselves()),
            "_only_a_reaction" => Some(g__only_a_reaction()),
            "_only_the_floor" => Some(g__only_the_floor()),
            "_points_at_nothing" => Some(g__points_at_nothing()),
            "_reads" => Some(g__reads()),
            "_reads_as_a_claim" => Some(g__reads_as_a_claim()),
            "_reads_as_taste" => Some(g__reads_as_taste()),
            "_says_it_already" => Some(g__says_it_already()),
            "_the_answer_above" => Some(g__the_answer_above()),
            "_the_answer_in_reply" => Some(g__the_answer_in_reply()),
            "_the_line_before" => Some(g__the_line_before()),
            "_the_pool_for_a_worry_about_them" => Some(g__the_pool_for_a_worry_about_them()),
            "_the_repeat_answer" => Some(py_func("_the_repeat_answer", |args: &[Py]| _the_repeat_answer(args))),
            "_the_rule_for_it_for_the_turn" => Some(py_func("_the_rule_for_it_for_the_turn", |args: &[Py]| _the_rule_for_it_for_the_turn(args))),
            "_the_thread_is_heavy" => Some(g__the_thread_is_heavy()),
            "_the_view_asked_for" => Some(py_func("_the_view_asked_for", |args: &[Py]| _the_view_asked_for(args))),
            "_the_view_of" => Some(g__the_view_of()),
            "_the_weird_ask_pool" => Some(g__the_weird_ask_pool()),
            "_their_own_day" => Some(g__their_own_day()),
            "_thread_is_about_a_person" => Some(g__thread_is_about_a_person()),
            "_thread_is_about_a_pet" => Some(g__thread_is_about_a_pet()),
            "_thread_is_about_someone_else" => Some(g__thread_is_about_someone_else()),
            "_thread_is_about_the_body" => Some(g__thread_is_about_the_body()),
            "_thread_names_a_machine" => Some(g__thread_names_a_machine()),
            "_tips" => Some(g__tips()),
            "_unknown" => Some(py_func("_unknown", |args: &[Py]| _unknown(args))),
            "_who_for_the_thread" => Some(g__who_for_the_thread()),
            "ack" => Some(g_ack()),
            "asks_for_more" => Some(g_asks_for_more()),
            "finding_out" => Some(g_finding_out()),
            "in_a_safety_thread" => Some(g_in_a_safety_thread()),
            "method_ask_pool" => Some(g_method_ask_pool()),
            "no_eyes" => Some(g_no_eyes()),
            "reads_as_if_i_saw" => Some(g_reads_as_if_i_saw()),
            "the_bots_own_answer" => Some(g_the_bots_own_answer()),
            "the_lost_ask_reply" => Some(g_the_lost_ask_reply()),
            "the_place_the_answer_is_kept" => Some(g_the_place_the_answer_is_kept()),
            "the_rule_for_it" => Some(g_the_rule_for_it()),
            "unknown_pool" => Some(g_unknown_pool()),
            "who_settles" => Some(g_who_settles()),
            "findout" => Some(pyrt::xrefs::xmodule("compose.findout")),
            "for_the_day" => Some(pyrt::xrefs::xmodule("compose.for_the_day")),
            "waited_for" => Some(pyrt::xrefs::xmodule("compose.waited_for")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANIMALS" => s_ANIMALS(v),
            "ASKED_IF_I_SAW_LINES" => s_ASKED_IF_I_SAW_LINES(v),
            "A_CARE_WORD" => s_A_CARE_WORD(v),
            "A_MONEY_WORD" => s_A_MONEY_WORD(v),
            "A_PERSON_IN_A_CRISIS" => s_A_PERSON_IN_A_CRISIS(v),
            "A_STEP_ASK" => s_A_STEP_ASK(v),
            "BARE_FOLLOW_UP" => s_BARE_FOLLOW_UP(v),
            "BARE_FOLLOW_UP_CARE_REPLIES" => s_BARE_FOLLOW_UP_CARE_REPLIES(v),
            "BARE_FOLLOW_UP_REPLIES" => s_BARE_FOLLOW_UP_REPLIES(v),
            "BARE_PICK_AFTER_NEWS" => s_BARE_PICK_AFTER_NEWS(v),
            "BODY_IS_NOT_A_MACHINE" => s_BODY_IS_NOT_A_MACHINE(v),
            "BODY_PARTS" => s_BODY_PARTS(v),
            "CARE_HERE" => s_CARE_HERE(v),
            "CARE_MODULES" => s_CARE_MODULES(v),
            "CRISIS_ASK" => s_CRISIS_ASK(v),
            "DITHER_OPEN_LINES" => s_DITHER_OPEN_LINES(v),
            "EXPERT_WORDS" => s_EXPERT_WORDS(v),
            "HANDED_BACK" => s_HANDED_BACK(v),
            "HEALTH_WORDS" => s_HEALTH_WORDS(v),
            "HELP_ALREADY" => s_HELP_ALREADY(v),
            "HOW_TO_DECIDE" => s_HOW_TO_DECIDE(v),
            "HOW_TO_DECIDE_LINES" => s_HOW_TO_DECIDE_LINES(v),
            "LEAVING_IN_THE_LINE" => s_LEAVING_IN_THE_LINE(v),
            "LIFE_DIRECTION" => s_LIFE_DIRECTION(v),
            "MISSED_STEP" => s_MISSED_STEP(v),
            "MISSED_STEP_KEEPS" => s_MISSED_STEP_KEEPS(v),
            "MISSED_STEP_LINES" => s_MISSED_STEP_LINES(v),
            "MISSED_STEP_PLAN" => s_MISSED_STEP_PLAN(v),
            "NEUTRAL" => s_NEUTRAL(v),
            "NOT_A_TIPS_ASK" => s_NOT_A_TIPS_ASK(v),
            "NOT_NOTED" => s_NOT_NOTED(v),
            "POINTING_LINES" => s_POINTING_LINES(v),
            "REALISTIC" => s_REALISTIC(v),
            "REALISTIC_ABOUT_ME" => s_REALISTIC_ABOUT_ME(v),
            "REPAIR_BARE_IT" => s_REPAIR_BARE_IT(v),
            "REPAIR_DECISION" => s_REPAIR_DECISION(v),
            "REPAIR_PERSON" => s_REPAIR_PERSON(v),
            "REPAIR_RULE" => s_REPAIR_RULE(v),
            "REPAIR_TARIFF" => s_REPAIR_TARIFF(v),
            "REPAIR_TRADE" => s_REPAIR_TRADE(v),
            "REPAIR_WHAT_IT_IS_WORTH" => s_REPAIR_WHAT_IT_IS_WORTH(v),
            "REPAIR_WHETHER" => s_REPAIR_WHETHER(v),
            "REPAIR_WHETHER_LINES" => s_REPAIR_WHETHER_LINES(v),
            "SAME_AGAIN" => s_SAME_AGAIN(v),
            "SAME_AGAIN_WITH_THE_ANSWER" => s_SAME_AGAIN_WITH_THE_ANSWER(v),
            "SYMPTOM_WORDS" => s_SYMPTOM_WORDS(v),
            "TAG_CAN" => s_TAG_CAN(v),
            "TAG_CONFIRM" => s_TAG_CONFIRM(v),
            "TAG_UNSURE" => s_TAG_UNSURE(v),
            "TIME_RULE" => s_TIME_RULE(v),
            "TIPS_ASKS" => s_TIPS_ASKS(v),
            "TIPS_LINES" => s_TIPS_LINES(v),
            "TOO_LATE" => s_TOO_LATE(v),
            "UNKNOWN" => s_UNKNOWN(v),
            "UNKNOWN_ABOUT_A_PET" => s_UNKNOWN_ABOUT_A_PET(v),
            "UNKNOWN_ABOUT_SOMEONE" => s_UNKNOWN_ABOUT_SOMEONE(v),
            "UNKNOWN_DECIDE" => s_UNKNOWN_DECIDE(v),
            "UNKNOWN_HEALTH" => s_UNKNOWN_HEALTH(v),
            "UNKNOWN_REALISTIC" => s_UNKNOWN_REALISTIC(v),
            "UNKNOWN_REALISTIC_CARE" => s_UNKNOWN_REALISTIC_CARE(v),
            "UNKNOWN_TOO_LATE" => s_UNKNOWN_TOO_LATE(v),
            "UNKNOWN_WORD" => s_UNKNOWN_WORD(v),
            "UNKNOWN_WORD_NAMED" => s_UNKNOWN_WORD_NAMED(v),
            "UNKNOWN_WORRY" => s_UNKNOWN_WORRY(v),
            "VIEW_AFTER_A_DEATH" => s_VIEW_AFTER_A_DEATH(v),
            "VIEW_ASK_LINES" => s_VIEW_ASK_LINES(v),
            "VIEW_COMPANY" => s_VIEW_COMPANY(v),
            "VIEW_DECIDE" => s_VIEW_DECIDE(v),
            "VIEW_HANDBACK" => s_VIEW_HANDBACK(v),
            "VIEW_LINES" => s_VIEW_LINES(v),
            "VIEW_MIRROR" => s_VIEW_MIRROR(v),
            "VIEW_MIRROR_LINES" => s_VIEW_MIRROR_LINES(v),
            "VIEW_NAMES_SOMEBODY" => s_VIEW_NAMES_SOMEBODY(v),
            "VIEW_NO_SUBJECT" => s_VIEW_NO_SUBJECT(v),
            "VIEW_ON_GOOD_NEWS" => s_VIEW_ON_GOOD_NEWS(v),
            "WASNT_THERE" => s_WASNT_THERE(v),
            "WHO_FOR_IT" => s_WHO_FOR_IT(v),
            "_a_bare_worry" => s__a_bare_worry(v),
            "_a_care_home_line" => s__a_care_home_line(v),
            "_a_dead_end" => s__a_dead_end(v),
            "_a_plan_doubt" => s__a_plan_doubt(v),
            "_a_plan_in_the_thread" => s__a_plan_in_the_thread(v),
            "_a_price_wants_a_verdict" => s__a_price_wants_a_verdict(v),
            "_a_question_about_the_person" => s__a_question_about_the_person(v),
            "_a_repair_already_spoken_of" => s__a_repair_already_spoken_of(v),
            "_a_subject_above" => s__a_subject_above(v),
            "_a_switch_off_asked" => s__a_switch_off_asked(v),
            "_a_thing_they_started" => s__a_thing_they_started(v),
            "_a_thing_was_named" => s__a_thing_was_named(v),
            "_advisor" => s__advisor(v),
            "_after_a_colon" => s__after_a_colon(v),
            "_answered_already" => s__answered_already(v),
            "_answered_recently" => s__answered_recently(v),
            "_asked_about_the_day_they_had" => s__asked_about_the_day_they_had(v),
            "_asks_a_name_back" => s__asks_a_name_back(v),
            "_asks_for_a_person_to_talk_to" => s__asks_for_a_person_to_talk_to(v),
            "_bare_pick" => s__bare_pick(v),
            "_choose" => s__choose(v),
            "_forecast_pool" => s__forecast_pool(v),
            "_grief_thread" => s__grief_thread(v),
            "_hedged_recently" => s__hedged_recently(v),
            "_in_a_care_thread" => s__in_a_care_thread(v),
            "_in_a_crisis_thread" => s__in_a_crisis_thread(v),
            "_in_a_loss_thread" => s__in_a_loss_thread(v),
            "_in_a_pool" => s__in_a_pool(v),
            "_is_a_hedge" => s__is_a_hedge(v),
            "_is_a_tips_ask" => s__is_a_tips_ask(v),
            "_is_a_view_ask" => s__is_a_view_ask(v),
            "_name_given_before" => s__name_given_before(v),
            "_names_its_own_thing" => s__names_its_own_thing(v),
            "_news_above" => s__news_above(v),
            "_nothing_to_choose" => s__nothing_to_choose(v),
            "_nothing_to_stand_behind" => s__nothing_to_stand_behind(v),
            "_offers_to_fix_themselves" => s__offers_to_fix_themselves(v),
            "_only_a_reaction" => s__only_a_reaction(v),
            "_only_the_floor" => s__only_the_floor(v),
            "_points_at_nothing" => s__points_at_nothing(v),
            "_reads" => s__reads(v),
            "_reads_as_a_claim" => s__reads_as_a_claim(v),
            "_reads_as_taste" => s__reads_as_taste(v),
            "_says_it_already" => s__says_it_already(v),
            "_the_answer_above" => s__the_answer_above(v),
            "_the_answer_in_reply" => s__the_answer_in_reply(v),
            "_the_line_before" => s__the_line_before(v),
            "_the_pool_for_a_worry_about_them" => s__the_pool_for_a_worry_about_them(v),
            "_the_thread_is_heavy" => s__the_thread_is_heavy(v),
            "_the_view_of" => s__the_view_of(v),
            "_the_weird_ask_pool" => s__the_weird_ask_pool(v),
            "_their_own_day" => s__their_own_day(v),
            "_thread_is_about_a_person" => s__thread_is_about_a_person(v),
            "_thread_is_about_a_pet" => s__thread_is_about_a_pet(v),
            "_thread_is_about_someone_else" => s__thread_is_about_someone_else(v),
            "_thread_is_about_the_body" => s__thread_is_about_the_body(v),
            "_thread_names_a_machine" => s__thread_names_a_machine(v),
            "_tips" => s__tips(v),
            "_who_for_the_thread" => s__who_for_the_thread(v),
            "ack" => s_ack(v),
            "asks_for_more" => s_asks_for_more(v),
            "finding_out" => s_finding_out(v),
            "in_a_safety_thread" => s_in_a_safety_thread(v),
            "method_ask_pool" => s_method_ask_pool(v),
            "no_eyes" => s_no_eyes(v),
            "reads_as_if_i_saw" => s_reads_as_if_i_saw(v),
            "the_bots_own_answer" => s_the_bots_own_answer(v),
            "the_lost_ask_reply" => s_the_lost_ask_reply(v),
            "the_place_the_answer_is_kept" => s_the_place_the_answer_is_kept(v),
            "the_rule_for_it" => s_the_rule_for_it(v),
            "unknown_pool" => s_unknown_pool(v),
            "who_settles" => s_who_settles(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.fallback_pools", module_obj);
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_the_repeat_answer", py_func("_the_repeat_answer", |args: &[Py]| _the_repeat_answer(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_drafting_pool", py_func("_drafting_pool", |args: &[Py]| _drafting_pool(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_care_thread", py_func("_a_care_thread", |args: &[Py]| _a_care_thread(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_money_thread", py_func("_a_money_thread", |args: &[Py]| _a_money_thread(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_the_rule_for_it_for_the_turn", py_func("_the_rule_for_it_for_the_turn", |args: &[Py]| _the_rule_for_it_for_the_turn(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_missed_step", py_func("_a_missed_step", |args: &[Py]| _a_missed_step(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_unknown", py_func("_unknown", |args: &[Py]| _unknown(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_countdown_in_a_care_thread_", py_func("_a_countdown_in_a_care_thread_", |args: &[Py]| _a_countdown_in_a_care_thread_(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_the_view_asked_for", py_func("_the_view_asked_for", |args: &[Py]| _the_view_asked_for(args)));
    pyrt::xrefs::reg_access("compose.fallback_pools", "ANIMALS", g_ANIMALS, s_ANIMALS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "ASKED_IF_I_SAW_LINES", g_ASKED_IF_I_SAW_LINES, s_ASKED_IF_I_SAW_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "A_CARE_WORD", g_A_CARE_WORD, s_A_CARE_WORD);
    pyrt::xrefs::reg_access("compose.fallback_pools", "A_MONEY_WORD", g_A_MONEY_WORD, s_A_MONEY_WORD);
    pyrt::xrefs::reg_access("compose.fallback_pools", "A_PERSON_IN_A_CRISIS", g_A_PERSON_IN_A_CRISIS, s_A_PERSON_IN_A_CRISIS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "A_STEP_ASK", g_A_STEP_ASK, s_A_STEP_ASK);
    pyrt::xrefs::reg_access("compose.fallback_pools", "BARE_FOLLOW_UP", g_BARE_FOLLOW_UP, s_BARE_FOLLOW_UP);
    pyrt::xrefs::reg_access("compose.fallback_pools", "BARE_FOLLOW_UP_CARE_REPLIES", g_BARE_FOLLOW_UP_CARE_REPLIES, s_BARE_FOLLOW_UP_CARE_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "BARE_FOLLOW_UP_REPLIES", g_BARE_FOLLOW_UP_REPLIES, s_BARE_FOLLOW_UP_REPLIES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "BARE_PICK_AFTER_NEWS", g_BARE_PICK_AFTER_NEWS, s_BARE_PICK_AFTER_NEWS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "BODY_IS_NOT_A_MACHINE", g_BODY_IS_NOT_A_MACHINE, s_BODY_IS_NOT_A_MACHINE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "BODY_PARTS", g_BODY_PARTS, s_BODY_PARTS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "CARE_HERE", g_CARE_HERE, s_CARE_HERE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "CARE_MODULES", g_CARE_MODULES, s_CARE_MODULES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "CRISIS_ASK", g_CRISIS_ASK, s_CRISIS_ASK);
    pyrt::xrefs::reg_access("compose.fallback_pools", "DITHER_OPEN_LINES", g_DITHER_OPEN_LINES, s_DITHER_OPEN_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "EXPERT_WORDS", g_EXPERT_WORDS, s_EXPERT_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "HANDED_BACK", g_HANDED_BACK, s_HANDED_BACK);
    pyrt::xrefs::reg_access("compose.fallback_pools", "HEALTH_WORDS", g_HEALTH_WORDS, s_HEALTH_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "HELP_ALREADY", g_HELP_ALREADY, s_HELP_ALREADY);
    pyrt::xrefs::reg_access("compose.fallback_pools", "HOW_TO_DECIDE", g_HOW_TO_DECIDE, s_HOW_TO_DECIDE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "HOW_TO_DECIDE_LINES", g_HOW_TO_DECIDE_LINES, s_HOW_TO_DECIDE_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "LEAVING_IN_THE_LINE", g_LEAVING_IN_THE_LINE, s_LEAVING_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "LIFE_DIRECTION", g_LIFE_DIRECTION, s_LIFE_DIRECTION);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP", g_MISSED_STEP, s_MISSED_STEP);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP_KEEPS", g_MISSED_STEP_KEEPS, s_MISSED_STEP_KEEPS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP_LINES", g_MISSED_STEP_LINES, s_MISSED_STEP_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP_PLAN", g_MISSED_STEP_PLAN, s_MISSED_STEP_PLAN);
    pyrt::xrefs::reg_access("compose.fallback_pools", "NEUTRAL", g_NEUTRAL, s_NEUTRAL);
    pyrt::xrefs::reg_access("compose.fallback_pools", "NOT_A_TIPS_ASK", g_NOT_A_TIPS_ASK, s_NOT_A_TIPS_ASK);
    pyrt::xrefs::reg_access("compose.fallback_pools", "NOT_NOTED", g_NOT_NOTED, s_NOT_NOTED);
    pyrt::xrefs::reg_access("compose.fallback_pools", "POINTING_LINES", g_POINTING_LINES, s_POINTING_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REALISTIC", g_REALISTIC, s_REALISTIC);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REALISTIC_ABOUT_ME", g_REALISTIC_ABOUT_ME, s_REALISTIC_ABOUT_ME);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_BARE_IT", g_REPAIR_BARE_IT, s_REPAIR_BARE_IT);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_DECISION", g_REPAIR_DECISION, s_REPAIR_DECISION);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_PERSON", g_REPAIR_PERSON, s_REPAIR_PERSON);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_RULE", g_REPAIR_RULE, s_REPAIR_RULE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_TARIFF", g_REPAIR_TARIFF, s_REPAIR_TARIFF);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_TRADE", g_REPAIR_TRADE, s_REPAIR_TRADE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_WHAT_IT_IS_WORTH", g_REPAIR_WHAT_IT_IS_WORTH, s_REPAIR_WHAT_IT_IS_WORTH);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_WHETHER", g_REPAIR_WHETHER, s_REPAIR_WHETHER);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_WHETHER_LINES", g_REPAIR_WHETHER_LINES, s_REPAIR_WHETHER_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "SAME_AGAIN", g_SAME_AGAIN, s_SAME_AGAIN);
    pyrt::xrefs::reg_access("compose.fallback_pools", "SAME_AGAIN_WITH_THE_ANSWER", g_SAME_AGAIN_WITH_THE_ANSWER, s_SAME_AGAIN_WITH_THE_ANSWER);
    pyrt::xrefs::reg_access("compose.fallback_pools", "SYMPTOM_WORDS", g_SYMPTOM_WORDS, s_SYMPTOM_WORDS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TAG_CAN", g_TAG_CAN, s_TAG_CAN);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TAG_CONFIRM", g_TAG_CONFIRM, s_TAG_CONFIRM);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TAG_UNSURE", g_TAG_UNSURE, s_TAG_UNSURE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TIME_RULE", g_TIME_RULE, s_TIME_RULE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TIPS_ASKS", g_TIPS_ASKS, s_TIPS_ASKS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TIPS_LINES", g_TIPS_LINES, s_TIPS_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TOO_LATE", g_TOO_LATE, s_TOO_LATE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN", g_UNKNOWN, s_UNKNOWN);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_ABOUT_A_PET", g_UNKNOWN_ABOUT_A_PET, s_UNKNOWN_ABOUT_A_PET);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_ABOUT_SOMEONE", g_UNKNOWN_ABOUT_SOMEONE, s_UNKNOWN_ABOUT_SOMEONE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_DECIDE", g_UNKNOWN_DECIDE, s_UNKNOWN_DECIDE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_HEALTH", g_UNKNOWN_HEALTH, s_UNKNOWN_HEALTH);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_REALISTIC", g_UNKNOWN_REALISTIC, s_UNKNOWN_REALISTIC);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_REALISTIC_CARE", g_UNKNOWN_REALISTIC_CARE, s_UNKNOWN_REALISTIC_CARE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_TOO_LATE", g_UNKNOWN_TOO_LATE, s_UNKNOWN_TOO_LATE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_WORD", g_UNKNOWN_WORD, s_UNKNOWN_WORD);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_WORD_NAMED", g_UNKNOWN_WORD_NAMED, s_UNKNOWN_WORD_NAMED);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN_WORRY", g_UNKNOWN_WORRY, s_UNKNOWN_WORRY);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_AFTER_A_DEATH", g_VIEW_AFTER_A_DEATH, s_VIEW_AFTER_A_DEATH);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_ASK_LINES", g_VIEW_ASK_LINES, s_VIEW_ASK_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_COMPANY", g_VIEW_COMPANY, s_VIEW_COMPANY);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_DECIDE", g_VIEW_DECIDE, s_VIEW_DECIDE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_HANDBACK", g_VIEW_HANDBACK, s_VIEW_HANDBACK);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_LINES", g_VIEW_LINES, s_VIEW_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_MIRROR", g_VIEW_MIRROR, s_VIEW_MIRROR);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_MIRROR_LINES", g_VIEW_MIRROR_LINES, s_VIEW_MIRROR_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_NAMES_SOMEBODY", g_VIEW_NAMES_SOMEBODY, s_VIEW_NAMES_SOMEBODY);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_NO_SUBJECT", g_VIEW_NO_SUBJECT, s_VIEW_NO_SUBJECT);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_ON_GOOD_NEWS", g_VIEW_ON_GOOD_NEWS, s_VIEW_ON_GOOD_NEWS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "WASNT_THERE", g_WASNT_THERE, s_WASNT_THERE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "WHO_FOR_IT", g_WHO_FOR_IT, s_WHO_FOR_IT);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_bare_worry", g__a_bare_worry, s__a_bare_worry);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_care_home_line", g__a_care_home_line, s__a_care_home_line);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_dead_end", g__a_dead_end, s__a_dead_end);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_plan_doubt", g__a_plan_doubt, s__a_plan_doubt);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_plan_in_the_thread", g__a_plan_in_the_thread, s__a_plan_in_the_thread);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_price_wants_a_verdict", g__a_price_wants_a_verdict, s__a_price_wants_a_verdict);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_question_about_the_person", g__a_question_about_the_person, s__a_question_about_the_person);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_repair_already_spoken_of", g__a_repair_already_spoken_of, s__a_repair_already_spoken_of);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_subject_above", g__a_subject_above, s__a_subject_above);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_switch_off_asked", g__a_switch_off_asked, s__a_switch_off_asked);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_thing_they_started", g__a_thing_they_started, s__a_thing_they_started);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_a_thing_was_named", g__a_thing_was_named, s__a_thing_was_named);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_advisor", g__advisor, s__advisor);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_after_a_colon", g__after_a_colon, s__after_a_colon);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_answered_already", g__answered_already, s__answered_already);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_answered_recently", g__answered_recently, s__answered_recently);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_asked_about_the_day_they_had", g__asked_about_the_day_they_had, s__asked_about_the_day_they_had);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_asks_a_name_back", g__asks_a_name_back, s__asks_a_name_back);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_asks_for_a_person_to_talk_to", g__asks_for_a_person_to_talk_to, s__asks_for_a_person_to_talk_to);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_bare_pick", g__bare_pick, s__bare_pick);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_choose", g__choose, s__choose);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_forecast_pool", g__forecast_pool, s__forecast_pool);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_grief_thread", g__grief_thread, s__grief_thread);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_hedged_recently", g__hedged_recently, s__hedged_recently);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_in_a_care_thread", g__in_a_care_thread, s__in_a_care_thread);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_in_a_crisis_thread", g__in_a_crisis_thread, s__in_a_crisis_thread);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_in_a_loss_thread", g__in_a_loss_thread, s__in_a_loss_thread);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_in_a_pool", g__in_a_pool, s__in_a_pool);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_is_a_hedge", g__is_a_hedge, s__is_a_hedge);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_is_a_tips_ask", g__is_a_tips_ask, s__is_a_tips_ask);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_is_a_view_ask", g__is_a_view_ask, s__is_a_view_ask);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_name_given_before", g__name_given_before, s__name_given_before);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_names_its_own_thing", g__names_its_own_thing, s__names_its_own_thing);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_news_above", g__news_above, s__news_above);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_nothing_to_choose", g__nothing_to_choose, s__nothing_to_choose);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_nothing_to_stand_behind", g__nothing_to_stand_behind, s__nothing_to_stand_behind);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_offers_to_fix_themselves", g__offers_to_fix_themselves, s__offers_to_fix_themselves);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_only_a_reaction", g__only_a_reaction, s__only_a_reaction);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_only_the_floor", g__only_the_floor, s__only_the_floor);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_points_at_nothing", g__points_at_nothing, s__points_at_nothing);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_reads", g__reads, s__reads);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_reads_as_a_claim", g__reads_as_a_claim, s__reads_as_a_claim);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_reads_as_taste", g__reads_as_taste, s__reads_as_taste);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_says_it_already", g__says_it_already, s__says_it_already);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_the_answer_above", g__the_answer_above, s__the_answer_above);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_the_answer_in_reply", g__the_answer_in_reply, s__the_answer_in_reply);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_the_line_before", g__the_line_before, s__the_line_before);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_the_pool_for_a_worry_about_them", g__the_pool_for_a_worry_about_them, s__the_pool_for_a_worry_about_them);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_the_thread_is_heavy", g__the_thread_is_heavy, s__the_thread_is_heavy);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_the_view_of", g__the_view_of, s__the_view_of);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_the_weird_ask_pool", g__the_weird_ask_pool, s__the_weird_ask_pool);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_their_own_day", g__their_own_day, s__their_own_day);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_thread_is_about_a_person", g__thread_is_about_a_person, s__thread_is_about_a_person);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_thread_is_about_a_pet", g__thread_is_about_a_pet, s__thread_is_about_a_pet);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_thread_is_about_someone_else", g__thread_is_about_someone_else, s__thread_is_about_someone_else);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_thread_is_about_the_body", g__thread_is_about_the_body, s__thread_is_about_the_body);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_thread_names_a_machine", g__thread_names_a_machine, s__thread_names_a_machine);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_tips", g__tips, s__tips);
    pyrt::xrefs::reg_access("compose.fallback_pools", "_who_for_the_thread", g__who_for_the_thread, s__who_for_the_thread);
    pyrt::xrefs::reg_access("compose.fallback_pools", "ack", g_ack, s_ack);
    pyrt::xrefs::reg_access("compose.fallback_pools", "asks_for_more", g_asks_for_more, s_asks_for_more);
    pyrt::xrefs::reg_access("compose.fallback_pools", "finding_out", g_finding_out, s_finding_out);
    pyrt::xrefs::reg_access("compose.fallback_pools", "in_a_safety_thread", g_in_a_safety_thread, s_in_a_safety_thread);
    pyrt::xrefs::reg_access("compose.fallback_pools", "method_ask_pool", g_method_ask_pool, s_method_ask_pool);
    pyrt::xrefs::reg_access("compose.fallback_pools", "no_eyes", g_no_eyes, s_no_eyes);
    pyrt::xrefs::reg_access("compose.fallback_pools", "reads_as_if_i_saw", g_reads_as_if_i_saw, s_reads_as_if_i_saw);
    pyrt::xrefs::reg_access("compose.fallback_pools", "the_bots_own_answer", g_the_bots_own_answer, s_the_bots_own_answer);
    pyrt::xrefs::reg_access("compose.fallback_pools", "the_lost_ask_reply", g_the_lost_ask_reply, s_the_lost_ask_reply);
    pyrt::xrefs::reg_access("compose.fallback_pools", "the_place_the_answer_is_kept", g_the_place_the_answer_is_kept, s_the_place_the_answer_is_kept);
    pyrt::xrefs::reg_access("compose.fallback_pools", "the_rule_for_it", g_the_rule_for_it, s_the_rule_for_it);
    pyrt::xrefs::reg_access("compose.fallback_pools", "unknown_pool", g_unknown_pool, s_unknown_pool);
    pyrt::xrefs::reg_access("compose.fallback_pools", "who_settles", g_who_settles, s_who_settles);
}
