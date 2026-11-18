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


thread_local! { static G_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGAIN_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_ALREADY_HAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_FIRST_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_PUT_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_DATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRANGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRANGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_OF_THE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONDOLENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GROUP_OF_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HELD_NAME_AND_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOSS_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MILESTONE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NIGHT_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_LOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_CARRIED_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WEAPON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_WITH_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_SETBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARRIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIGGER_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURGLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURGLED_FOR_SOMEONE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURGLED_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURGLED_NOT_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURGLED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURGLED_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTERFACTUAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRACKED_A_JOKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEADLINE_LATER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEPENDANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEPENDANT_WANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEPENDANT_WANT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_TO_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBLE_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_STRETCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FUTURE_REGRET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_ON_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAD_IT_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTEREST_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTEREST_BACK_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTEREST_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTEREST_DOING_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTEREST_STARTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTEREST_STARTED_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOSS_FOLLOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOSS_FOLLOW_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MARKED_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MARRIAGE_IN_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEVER_CAME_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_INTEREST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_BREAK_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NAME_AFTER_CALLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_RELATION_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TASTE_AS_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_WEAPON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_REGRET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_ARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_ELECTRIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_ELECTRIC_CAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_FACTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_KIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_KIT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_KIT_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_KIT_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SLIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SPACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SPACES_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_TWO_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_CODES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLANNED_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_REPLIES_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POLITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRACTICAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT_SETBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIET_BETWEEN_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGRET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGRET_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGRET_LINES_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION_GONE_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROWS_BETWEEN_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETBACK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOULDNT_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLIP_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMASHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMASHED_OF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_AHEAD_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_BADLY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_FINE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_GONE_BADLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_WELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_START_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_START_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_START_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_START_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKEN_FROM_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TASTE_TOO_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_BABY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_CALLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_CHILD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_DISLIKE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_ELDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_ELDER_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_ELDER_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_GROWN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_HATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_HATE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_JOB_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_LIKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_LIKING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_MOOD_WITH_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_MOOD_WITH_YOU_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAMES_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME_AGE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME_GROWN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME_SO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PLACE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_SMALL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TEEN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PERSON_WITH_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_QUIET_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_RELATION_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ROWS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_THING_IS_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THOUGHT_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOOK_IT_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TURNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_TO_BE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEAPON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WENT_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WISH_I: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YEAR_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AHEAD_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HARM_AT_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_SHOULD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RELATION_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_without_leaders: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.remark: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AGAIN() -> Py { ensure_init(); G_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN(v: Py) { ensure_init(); G_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_AGAIN_REPLIES() -> Py { ensure_init(); G_AGAIN_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN_REPLIES(v: Py) { ensure_init(); G_AGAIN_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_ALREADY_HAD() -> Py { ensure_init(); G_AGE_ALREADY_HAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_ALREADY_HAD(v: Py) { ensure_init(); G_AGE_ALREADY_HAD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_FIRST_TOLD() -> Py { ensure_init(); G_AGE_FIRST_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_FIRST_TOLD(v: Py) { ensure_init(); G_AGE_FIRST_TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_PUT_RIGHT() -> Py { ensure_init(); G_AGE_PUT_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_PUT_RIGHT(v: Py) { ensure_init(); G_AGE_PUT_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_WORDS() -> Py { ensure_init(); G_AGE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_WORDS(v: Py) { ensure_init(); G_AGE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_DATED() -> Py { ensure_init(); G_ALREADY_DATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_DATED(v: Py) { ensure_init(); G_ALREADY_DATED.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRANGED() -> Py { ensure_init(); G_ARRANGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRANGED(v: Py) { ensure_init(); G_ARRANGED.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRANGEMENT() -> Py { ensure_init(); G_ARRANGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRANGEMENT(v: Py) { ensure_init(); G_ARRANGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_OF_THE_PERSON() -> Py { ensure_init(); G_ASKED_OF_THE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_OF_THE_PERSON(v: Py) { ensure_init(); G_ASKED_OF_THE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD_NAMED() -> Py { ensure_init(); G_A_CHILD_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_NAMED(v: Py) { ensure_init(); G_A_CHILD_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONDOLENCE() -> Py { ensure_init(); G_A_CONDOLENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONDOLENCE(v: Py) { ensure_init(); G_A_CONDOLENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GROUP_OF_PEOPLE() -> Py { ensure_init(); G_A_GROUP_OF_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GROUP_OF_PEOPLE(v: Py) { ensure_init(); G_A_GROUP_OF_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HELD_NAME_AND_AN_AGE() -> Py { ensure_init(); G_A_HELD_NAME_AND_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HELD_NAME_AND_AN_AGE(v: Py) { ensure_init(); G_A_HELD_NAME_AND_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOSS_IN_THE_LINE() -> Py { ensure_init(); G_A_LOSS_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOSS_IN_THE_LINE(v: Py) { ensure_init(); G_A_LOSS_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MILESTONE_TAIL() -> Py { ensure_init(); G_A_MILESTONE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MILESTONE_TAIL(v: Py) { ensure_init(); G_A_MILESTONE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NIGHT_OUT() -> Py { ensure_init(); G_A_NIGHT_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NIGHT_OUT(v: Py) { ensure_init(); G_A_NIGHT_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_LOST() -> Py { ensure_init(); G_A_PERSON_LOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_LOST(v: Py) { ensure_init(); G_A_PERSON_LOST.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_OF_THEIRS() -> Py { ensure_init(); G_A_PERSON_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_OF_THEIRS(v: Py) { ensure_init(); G_A_PERSON_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_CARRIED_OFF() -> Py { ensure_init(); G_A_THING_CARRIED_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_CARRIED_OFF(v: Py) { ensure_init(); G_A_THING_CARRIED_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_TAKEN() -> Py { ensure_init(); G_A_THING_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_TAKEN(v: Py) { ensure_init(); G_A_THING_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WEAPON() -> Py { ensure_init(); G_A_WEAPON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WEAPON(v: Py) { ensure_init(); G_A_WEAPON.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY_WORDS() -> Py { ensure_init(); G_BABY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY_WORDS(v: Py) { ensure_init(); G_BABY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_WITH_THEM() -> Py { ensure_init(); G_BAD_WITH_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_WITH_THEM(v: Py) { ensure_init(); G_BAD_WITH_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_SETBACK() -> Py { ensure_init(); G_BARE_SETBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_SETBACK(v: Py) { ensure_init(); G_BARE_SETBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARRIER() -> Py { ensure_init(); G_BARRIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARRIER(v: Py) { ensure_init(); G_BARRIER.with(|c| *c.borrow_mut() = v); }
pub fn g_BIGGER_PLAN() -> Py { ensure_init(); G_BIGGER_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIGGER_PLAN(v: Py) { ensure_init(); G_BIGGER_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED() -> Py { ensure_init(); G_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED(v: Py) { ensure_init(); G_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED_REPLIES() -> Py { ensure_init(); G_BOOKED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED_REPLIES(v: Py) { ensure_init(); G_BOOKED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGLED() -> Py { ensure_init(); G_BURGLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGLED(v: Py) { ensure_init(); G_BURGLED.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGLED_FOR_SOMEONE_REPLIES() -> Py { ensure_init(); G_BURGLED_FOR_SOMEONE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGLED_FOR_SOMEONE_REPLIES(v: Py) { ensure_init(); G_BURGLED_FOR_SOMEONE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGLED_HOME() -> Py { ensure_init(); G_BURGLED_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGLED_HOME(v: Py) { ensure_init(); G_BURGLED_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGLED_NOT_THEIRS() -> Py { ensure_init(); G_BURGLED_NOT_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGLED_NOT_THEIRS(v: Py) { ensure_init(); G_BURGLED_NOT_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGLED_REPLIES() -> Py { ensure_init(); G_BURGLED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGLED_REPLIES(v: Py) { ensure_init(); G_BURGLED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGLED_SOMEONE_ELSE() -> Py { ensure_init(); G_BURGLED_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGLED_SOMEONE_ELSE(v: Py) { ensure_init(); G_BURGLED_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTERFACTUAL() -> Py { ensure_init(); G_COUNTERFACTUAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTERFACTUAL(v: Py) { ensure_init(); G_COUNTERFACTUAL.with(|c| *c.borrow_mut() = v); }
pub fn g_CRACKED_A_JOKE() -> Py { ensure_init(); G_CRACKED_A_JOKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRACKED_A_JOKE(v: Py) { ensure_init(); G_CRACKED_A_JOKE.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS() -> Py { ensure_init(); G_CRISIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS(v: Py) { ensure_init(); G_CRISIS.with(|c| *c.borrow_mut() = v); }
pub fn g_DEADLINE_LATER() -> Py { ensure_init(); G_DEADLINE_LATER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEADLINE_LATER(v: Py) { ensure_init(); G_DEADLINE_LATER.with(|c| *c.borrow_mut() = v); }
pub fn g_DEPENDANT() -> Py { ensure_init(); G_DEPENDANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEPENDANT(v: Py) { ensure_init(); G_DEPENDANT.with(|c| *c.borrow_mut() = v); }
pub fn g_DEPENDANT_WANT() -> Py { ensure_init(); G_DEPENDANT_WANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEPENDANT_WANT(v: Py) { ensure_init(); G_DEPENDANT_WANT.with(|c| *c.borrow_mut() = v); }
pub fn g_DEPENDANT_WANT_REPLIES() -> Py { ensure_init(); G_DEPENDANT_WANT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEPENDANT_WANT_REPLIES(v: Py) { ensure_init(); G_DEPENDANT_WANT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_TO_ME() -> Py { ensure_init(); G_DONE_TO_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_TO_ME(v: Py) { ensure_init(); G_DONE_TO_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBLE_BOOKED() -> Py { ensure_init(); G_DOUBLE_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBLE_BOOKED(v: Py) { ensure_init(); G_DOUBLE_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT() -> Py { ensure_init(); G_DOUBT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT(v: Py) { ensure_init(); G_DOUBT.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON() -> Py { ensure_init(); G_FIRST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON(v: Py) { ensure_init(); G_FIRST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_STRETCH() -> Py { ensure_init(); G_FIRST_STRETCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_STRETCH(v: Py) { ensure_init(); G_FIRST_STRETCH.with(|c| *c.borrow_mut() = v); }
pub fn g_FUTURE_REGRET() -> Py { ensure_init(); G_FUTURE_REGRET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FUTURE_REGRET(v: Py) { ensure_init(); G_FUTURE_REGRET.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_ON() -> Py { ensure_init(); G_GOING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_ON(v: Py) { ensure_init(); G_GOING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_ON_LINES() -> Py { ensure_init(); G_GOING_ON_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_ON_LINES(v: Py) { ensure_init(); G_GOING_ON_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_TO() -> Py { ensure_init(); G_GOING_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_TO(v: Py) { ensure_init(); G_GOING_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_HAD_IT_BOOKED() -> Py { ensure_init(); G_HAD_IT_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAD_IT_BOOKED(v: Py) { ensure_init(); G_HAD_IT_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_HE_PEOPLE() -> Py { ensure_init(); G_HE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HE_PEOPLE(v: Py) { ensure_init(); G_HE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_HE_PERSON() -> Py { ensure_init(); G_HE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HE_PERSON(v: Py) { ensure_init(); G_HE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_INTEREST_BACK() -> Py { ensure_init(); G_INTEREST_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTEREST_BACK(v: Py) { ensure_init(); G_INTEREST_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_INTEREST_BACK_PLAIN() -> Py { ensure_init(); G_INTEREST_BACK_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTEREST_BACK_PLAIN(v: Py) { ensure_init(); G_INTEREST_BACK_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_INTEREST_DOING() -> Py { ensure_init(); G_INTEREST_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTEREST_DOING(v: Py) { ensure_init(); G_INTEREST_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_INTEREST_DOING_PLAIN() -> Py { ensure_init(); G_INTEREST_DOING_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTEREST_DOING_PLAIN(v: Py) { ensure_init(); G_INTEREST_DOING_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_INTEREST_STARTED() -> Py { ensure_init(); G_INTEREST_STARTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTEREST_STARTED(v: Py) { ensure_init(); G_INTEREST_STARTED.with(|c| *c.borrow_mut() = v); }
pub fn g_INTEREST_STARTED_PLAIN() -> Py { ensure_init(); G_INTEREST_STARTED_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTEREST_STARTED_PLAIN(v: Py) { ensure_init(); G_INTEREST_STARTED_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_LOSS_FOLLOW() -> Py { ensure_init(); G_LOSS_FOLLOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOSS_FOLLOW(v: Py) { ensure_init(); G_LOSS_FOLLOW.with(|c| *c.borrow_mut() = v); }
pub fn g_LOSS_FOLLOW_REPLIES() -> Py { ensure_init(); G_LOSS_FOLLOW_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOSS_FOLLOW_REPLIES(v: Py) { ensure_init(); G_LOSS_FOLLOW_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MARKED_DATE() -> Py { ensure_init(); G_MARKED_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MARKED_DATE(v: Py) { ensure_init(); G_MARKED_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_MARRIAGE_IN_TROUBLE() -> Py { ensure_init(); G_MARRIAGE_IN_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MARRIAGE_IN_TROUBLE(v: Py) { ensure_init(); G_MARRIAGE_IN_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED() -> Py { ensure_init(); G_MISSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED(v: Py) { ensure_init(); G_MISSED.with(|c| *c.borrow_mut() = v); }
pub fn g_NEVER_CAME_SUBJECT() -> Py { ensure_init(); G_NEVER_CAME_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEVER_CAME_SUBJECT(v: Py) { ensure_init(); G_NEVER_CAME_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_INTEREST() -> Py { ensure_init(); G_NOT_AN_INTEREST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_INTEREST(v: Py) { ensure_init(); G_NOT_AN_INTEREST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_BREAK_IN() -> Py { ensure_init(); G_NOT_A_BREAK_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_BREAK_IN(v: Py) { ensure_init(); G_NOT_A_BREAK_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_LOSS() -> Py { ensure_init(); G_NOT_A_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_LOSS(v: Py) { ensure_init(); G_NOT_A_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NAME_AFTER_CALLED() -> Py { ensure_init(); G_NOT_A_NAME_AFTER_CALLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME_AFTER_CALLED(v: Py) { ensure_init(); G_NOT_A_NAME_AFTER_CALLED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_RELATION_TROUBLE() -> Py { ensure_init(); G_NOT_A_RELATION_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RELATION_TROUBLE(v: Py) { ensure_init(); G_NOT_A_RELATION_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TASTE_AS_A_THING() -> Py { ensure_init(); G_NOT_A_TASTE_AS_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TASTE_AS_A_THING(v: Py) { ensure_init(); G_NOT_A_TASTE_AS_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_WEAPON() -> Py { ensure_init(); G_NOT_A_WEAPON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_WEAPON(v: Py) { ensure_init(); G_NOT_A_WEAPON.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_PLAIN() -> Py { ensure_init(); G_NOT_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_PLAIN(v: Py) { ensure_init(); G_NOT_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_REGRET() -> Py { ensure_init(); G_NO_REGRET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_REGRET(v: Py) { ensure_init(); G_NO_REGRET.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PEOPLE() -> Py { ensure_init(); G_OTHER_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PEOPLE(v: Py) { ensure_init(); G_OTHER_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_ARTICLE() -> Py { ensure_init(); G_OWN_ARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_ARTICLE(v: Py) { ensure_init(); G_OWN_ARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_ASKS() -> Py { ensure_init(); G_OWN_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_ASKS(v: Py) { ensure_init(); G_OWN_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_ELECTRIC() -> Py { ensure_init(); G_OWN_ELECTRIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_ELECTRIC(v: Py) { ensure_init(); G_OWN_ELECTRIC.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_ELECTRIC_CAR() -> Py { ensure_init(); G_OWN_ELECTRIC_CAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_ELECTRIC_CAR(v: Py) { ensure_init(); G_OWN_ELECTRIC_CAR.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_FACTS() -> Py { ensure_init(); G_OWN_FACTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_FACTS(v: Py) { ensure_init(); G_OWN_FACTS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_FAULT() -> Py { ensure_init(); G_OWN_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_FAULT(v: Py) { ensure_init(); G_OWN_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_HAVE() -> Py { ensure_init(); G_OWN_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_HAVE(v: Py) { ensure_init(); G_OWN_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_KIT() -> Py { ensure_init(); G_OWN_KIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_KIT(v: Py) { ensure_init(); G_OWN_KIT.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_KIT_ASKS() -> Py { ensure_init(); G_OWN_KIT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_KIT_ASKS(v: Py) { ensure_init(); G_OWN_KIT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_KIT_NAMES() -> Py { ensure_init(); G_OWN_KIT_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_KIT_NAMES(v: Py) { ensure_init(); G_OWN_KIT_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_KIT_PLAIN() -> Py { ensure_init(); G_OWN_KIT_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_KIT_PLAIN(v: Py) { ensure_init(); G_OWN_KIT_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_NOT_THE_THING() -> Py { ensure_init(); G_OWN_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_NOT_THE_THING(v: Py) { ensure_init(); G_OWN_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SLIP() -> Py { ensure_init(); G_OWN_SLIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SLIP(v: Py) { ensure_init(); G_OWN_SLIP.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SPACES() -> Py { ensure_init(); G_OWN_SPACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SPACES(v: Py) { ensure_init(); G_OWN_SPACES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SPACES_PLAIN() -> Py { ensure_init(); G_OWN_SPACES_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SPACES_PLAIN(v: Py) { ensure_init(); G_OWN_SPACES_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_TAIL() -> Py { ensure_init(); G_OWN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_TAIL(v: Py) { ensure_init(); G_OWN_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_TIME() -> Py { ensure_init(); G_OWN_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_TIME(v: Py) { ensure_init(); G_OWN_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_TWO_THINGS() -> Py { ensure_init(); G_OWN_TWO_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_TWO_THINGS(v: Py) { ensure_init(); G_OWN_TWO_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_CODES() -> Py { ensure_init(); G_PLACE_CODES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_CODES(v: Py) { ensure_init(); G_PLACE_CODES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN() -> Py { ensure_init(); G_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN(v: Py) { ensure_init(); G_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLANNED_VERBS() -> Py { ensure_init(); G_PLANNED_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLANNED_VERBS(v: Py) { ensure_init(); G_PLANNED_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_REPLIES() -> Py { ensure_init(); G_PLAN_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_REPLIES(v: Py) { ensure_init(); G_PLAN_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_REPLIES_PLAIN() -> Py { ensure_init(); G_PLAN_REPLIES_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_REPLIES_PLAIN(v: Py) { ensure_init(); G_PLAN_REPLIES_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_POLITE() -> Py { ensure_init(); G_POLITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POLITE(v: Py) { ensure_init(); G_POLITE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRACTICAL_ASK() -> Py { ensure_init(); G_PRACTICAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRACTICAL_ASK(v: Py) { ensure_init(); G_PRACTICAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT_SETBACK() -> Py { ensure_init(); G_PRESENT_SETBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT_SETBACK(v: Py) { ensure_init(); G_PRESENT_SETBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE() -> Py { ensure_init(); G_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE(v: Py) { ensure_init(); G_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_DOWN() -> Py { ensure_init(); G_PUT_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_DOWN(v: Py) { ensure_init(); G_PUT_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_FIRST() -> Py { ensure_init(); G_QUESTION_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_FIRST(v: Py) { ensure_init(); G_QUESTION_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIET_BETWEEN_THEM() -> Py { ensure_init(); G_QUIET_BETWEEN_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIET_BETWEEN_THEM(v: Py) { ensure_init(); G_QUIET_BETWEEN_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_REGRET() -> Py { ensure_init(); G_REGRET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGRET(v: Py) { ensure_init(); G_REGRET.with(|c| *c.borrow_mut() = v); }
pub fn g_REGRET_LINES() -> Py { ensure_init(); G_REGRET_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGRET_LINES(v: Py) { ensure_init(); G_REGRET_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_REGRET_LINES_PLAIN() -> Py { ensure_init(); G_REGRET_LINES_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGRET_LINES_PLAIN(v: Py) { ensure_init(); G_REGRET_LINES_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION_GONE_WRONG() -> Py { ensure_init(); G_RELATION_GONE_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION_GONE_WRONG(v: Py) { ensure_init(); G_RELATION_GONE_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_BODY() -> Py { ensure_init(); G_REPORT_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_BODY(v: Py) { ensure_init(); G_REPORT_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_FEELING() -> Py { ensure_init(); G_REPORT_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_FEELING(v: Py) { ensure_init(); G_REPORT_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_RISE() -> Py { ensure_init(); G_RISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RISE(v: Py) { ensure_init(); G_RISE.with(|c| *c.borrow_mut() = v); }
pub fn g_ROWS_BETWEEN_THEM() -> Py { ensure_init(); G_ROWS_BETWEEN_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROWS_BETWEEN_THEM(v: Py) { ensure_init(); G_ROWS_BETWEEN_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_SETBACK() -> Py { ensure_init(); G_SETBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETBACK(v: Py) { ensure_init(); G_SETBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_SETBACK_REPLIES() -> Py { ensure_init(); G_SETBACK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETBACK_REPLIES(v: Py) { ensure_init(); G_SETBACK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SHE_PEOPLE() -> Py { ensure_init(); G_SHE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHE_PEOPLE(v: Py) { ensure_init(); G_SHE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHE_PERSON() -> Py { ensure_init(); G_SHE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHE_PERSON(v: Py) { ensure_init(); G_SHE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOULDNT_HAVE() -> Py { ensure_init(); G_SHOULDNT_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOULDNT_HAVE(v: Py) { ensure_init(); G_SHOULDNT_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_SLIP_REPLIES() -> Py { ensure_init(); G_SLIP_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLIP_REPLIES(v: Py) { ensure_init(); G_SLIP_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SMASHED() -> Py { ensure_init(); G_SMASHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMASHED(v: Py) { ensure_init(); G_SMASHED.with(|c| *c.borrow_mut() = v); }
pub fn g_SMASHED_OF_A_THING() -> Py { ensure_init(); G_SMASHED_OF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMASHED_OF_A_THING(v: Py) { ensure_init(); G_SMASHED_OF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED() -> Py { ensure_init(); G_STARTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED(v: Py) { ensure_init(); G_STARTED.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_AHEAD_REPLIES() -> Py { ensure_init(); G_STARTED_AHEAD_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_AHEAD_REPLIES(v: Py) { ensure_init(); G_STARTED_AHEAD_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_BADLY_REPLIES() -> Py { ensure_init(); G_STARTED_BADLY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_BADLY_REPLIES(v: Py) { ensure_init(); G_STARTED_BADLY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_FINE_REPLIES() -> Py { ensure_init(); G_STARTED_FINE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_FINE_REPLIES(v: Py) { ensure_init(); G_STARTED_FINE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_GONE_BADLY() -> Py { ensure_init(); G_STARTED_GONE_BADLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_GONE_BADLY(v: Py) { ensure_init(); G_STARTED_GONE_BADLY.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_REPLIES() -> Py { ensure_init(); G_STARTED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_REPLIES(v: Py) { ensure_init(); G_STARTED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_VERDICT() -> Py { ensure_init(); G_STARTED_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_VERDICT(v: Py) { ensure_init(); G_STARTED_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_WELL() -> Py { ensure_init(); G_STARTED_WELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_WELL(v: Py) { ensure_init(); G_STARTED_WELL.with(|c| *c.borrow_mut() = v); }
pub fn g_START_AHEAD() -> Py { ensure_init(); G_START_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_START_AHEAD(v: Py) { ensure_init(); G_START_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_START_CLOCK() -> Py { ensure_init(); G_START_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_START_CLOCK(v: Py) { ensure_init(); G_START_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_START_PAST() -> Py { ensure_init(); G_START_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_START_PAST(v: Py) { ensure_init(); G_START_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_START_PRESENT() -> Py { ensure_init(); G_START_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_START_PRESENT(v: Py) { ensure_init(); G_START_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_NOUNS() -> Py { ensure_init(); G_STEP_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_NOUNS(v: Py) { ensure_init(); G_STEP_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKEN_FROM_THEM() -> Py { ensure_init(); G_TAKEN_FROM_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKEN_FROM_THEM(v: Py) { ensure_init(); G_TAKEN_FROM_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_TASTE_TOO_LONG() -> Py { ensure_init(); G_TASTE_TOO_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TASTE_TOO_LONG(v: Py) { ensure_init(); G_TASTE_TOO_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_AGE() -> Py { ensure_init(); G_THEIR_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_AGE(v: Py) { ensure_init(); G_THEIR_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_BABY_LINES() -> Py { ensure_init(); G_THEIR_BABY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_BABY_LINES(v: Py) { ensure_init(); G_THEIR_BABY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_CALLED() -> Py { ensure_init(); G_THEIR_CALLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_CALLED(v: Py) { ensure_init(); G_THEIR_CALLED.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_CHILD() -> Py { ensure_init(); G_THEIR_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_CHILD(v: Py) { ensure_init(); G_THEIR_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_CHILD_LINES() -> Py { ensure_init(); G_THEIR_CHILD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_CHILD_LINES(v: Py) { ensure_init(); G_THEIR_CHILD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_DISLIKE() -> Py { ensure_init(); G_THEIR_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_DISLIKE(v: Py) { ensure_init(); G_THEIR_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_DISLIKE_LINES() -> Py { ensure_init(); G_THEIR_DISLIKE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_DISLIKE_LINES(v: Py) { ensure_init(); G_THEIR_DISLIKE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_ELDER() -> Py { ensure_init(); G_THEIR_ELDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_ELDER(v: Py) { ensure_init(); G_THEIR_ELDER.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_ELDER_AGE() -> Py { ensure_init(); G_THEIR_ELDER_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_ELDER_AGE(v: Py) { ensure_init(); G_THEIR_ELDER_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_ELDER_LINES() -> Py { ensure_init(); G_THEIR_ELDER_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_ELDER_LINES(v: Py) { ensure_init(); G_THEIR_ELDER_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_GROWN_LINES() -> Py { ensure_init(); G_THEIR_GROWN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_GROWN_LINES(v: Py) { ensure_init(); G_THEIR_GROWN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_HATE() -> Py { ensure_init(); G_THEIR_HATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_HATE(v: Py) { ensure_init(); G_THEIR_HATE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_HATE_LINES() -> Py { ensure_init(); G_THEIR_HATE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_HATE_LINES(v: Py) { ensure_init(); G_THEIR_HATE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_JOB() -> Py { ensure_init(); G_THEIR_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_JOB(v: Py) { ensure_init(); G_THEIR_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_JOB_LINES() -> Py { ensure_init(); G_THEIR_JOB_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_JOB_LINES(v: Py) { ensure_init(); G_THEIR_JOB_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_LIKING() -> Py { ensure_init(); G_THEIR_LIKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_LIKING(v: Py) { ensure_init(); G_THEIR_LIKING.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_LIKING_LINES() -> Py { ensure_init(); G_THEIR_LIKING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_LIKING_LINES(v: Py) { ensure_init(); G_THEIR_LIKING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_MOOD_WITH_YOU() -> Py { ensure_init(); G_THEIR_MOOD_WITH_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_MOOD_WITH_YOU(v: Py) { ensure_init(); G_THEIR_MOOD_WITH_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_MOOD_WITH_YOU_LINES() -> Py { ensure_init(); G_THEIR_MOOD_WITH_YOU_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_MOOD_WITH_YOU_LINES(v: Py) { ensure_init(); G_THEIR_MOOD_WITH_YOU_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME() -> Py { ensure_init(); G_THEIR_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME(v: Py) { ensure_init(); G_THEIR_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAMES_LINES() -> Py { ensure_init(); G_THEIR_NAMES_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAMES_LINES(v: Py) { ensure_init(); G_THEIR_NAMES_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME_AGE() -> Py { ensure_init(); G_THEIR_NAME_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME_AGE(v: Py) { ensure_init(); G_THEIR_NAME_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME_AGE_LINES() -> Py { ensure_init(); G_THEIR_NAME_AGE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME_AGE_LINES(v: Py) { ensure_init(); G_THEIR_NAME_AGE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME_GROWN_LINES() -> Py { ensure_init(); G_THEIR_NAME_GROWN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME_GROWN_LINES(v: Py) { ensure_init(); G_THEIR_NAME_GROWN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME_LINES() -> Py { ensure_init(); G_THEIR_NAME_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME_LINES(v: Py) { ensure_init(); G_THEIR_NAME_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME_SO() -> Py { ensure_init(); G_THEIR_NAME_SO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME_SO(v: Py) { ensure_init(); G_THEIR_NAME_SO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON() -> Py { ensure_init(); G_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON(v: Py) { ensure_init(); G_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PLACE() -> Py { ensure_init(); G_THEIR_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PLACE(v: Py) { ensure_init(); G_THEIR_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PLACE_LINES() -> Py { ensure_init(); G_THEIR_PLACE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PLACE_LINES(v: Py) { ensure_init(); G_THEIR_PLACE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_QUALIFIER() -> Py { ensure_init(); G_THEIR_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_QUALIFIER(v: Py) { ensure_init(); G_THEIR_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_SMALL_LINES() -> Py { ensure_init(); G_THEIR_SMALL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_SMALL_LINES(v: Py) { ensure_init(); G_THEIR_SMALL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TEEN_LINES() -> Py { ensure_init(); G_THEIR_TEEN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TEEN_LINES(v: Py) { ensure_init(); G_THEIR_TEEN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_PERSON() -> Py { ensure_init(); G_THEY_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_PERSON(v: Py) { ensure_init(); G_THEY_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PERSON_WITH_IT() -> Py { ensure_init(); G_THE_PERSON_WITH_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PERSON_WITH_IT(v: Py) { ensure_init(); G_THE_PERSON_WITH_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_QUIET_LINES() -> Py { ensure_init(); G_THE_QUIET_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_QUIET_LINES(v: Py) { ensure_init(); G_THE_QUIET_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_RELATION_LINES() -> Py { ensure_init(); G_THE_RELATION_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_RELATION_LINES(v: Py) { ensure_init(); G_THE_RELATION_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ROWS_LINES() -> Py { ensure_init(); G_THE_ROWS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ROWS_LINES(v: Py) { ensure_init(); G_THE_ROWS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_THING_IS_A_CLAUSE() -> Py { ensure_init(); G_THE_THING_IS_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_THING_IS_A_CLAUSE(v: Py) { ensure_init(); G_THE_THING_IS_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_THOUGHT_OBJECT() -> Py { ensure_init(); G_THOUGHT_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THOUGHT_OBJECT(v: Py) { ensure_init(); G_THOUGHT_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_TOOK_IT_IN() -> Py { ensure_init(); G_TOOK_IT_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOOK_IT_IN(v: Py) { ensure_init(); G_TOOK_IT_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_TURNED() -> Py { ensure_init(); G_TURNED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TURNED(v: Py) { ensure_init(); G_TURNED.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_TO_BE() -> Py { ensure_init(); G_WANTS_TO_BE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_TO_BE(v: Py) { ensure_init(); G_WANTS_TO_BE.with(|c| *c.borrow_mut() = v); }
pub fn g_WEAPON_WORDS() -> Py { ensure_init(); G_WEAPON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEAPON_WORDS(v: Py) { ensure_init(); G_WEAPON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WENT_WRONG() -> Py { ensure_init(); G_WENT_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WENT_WRONG(v: Py) { ensure_init(); G_WENT_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_WISH_I() -> Py { ensure_init(); G_WISH_I.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WISH_I(v: Py) { ensure_init(); G_WISH_I.with(|c| *c.borrow_mut() = v); }
pub fn g_YEAR_UNIT() -> Py { ensure_init(); G_YEAR_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YEAR_UNIT(v: Py) { ensure_init(); G_YEAR_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__AHEAD_DAY() -> Py { ensure_init(); G__AHEAD_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AHEAD_DAY(v: Py) { ensure_init(); G__AHEAD_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__HARM_AT_PERSON() -> Py { ensure_init(); G__HARM_AT_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HARM_AT_PERSON(v: Py) { ensure_init(); G__HARM_AT_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_SHOULD() -> Py { ensure_init(); G__OWN_SHOULD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_SHOULD(v: Py) { ensure_init(); G__OWN_SHOULD.with(|c| *c.borrow_mut() = v); }
pub fn g__RELATION_PERSON() -> Py { ensure_init(); G__RELATION_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RELATION_PERSON(v: Py) { ensure_init(); G__RELATION_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_without_leaders() -> Py { ensure_init(); G_without_leaders.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_without_leaders(v: Py) { ensure_init(); G_without_leaders.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The reply to an ordinary statement that says what the person means to do.\n\nMost of the lines the bot cannot answer are not questions. Someone tells it they are thinking of\nmoving, that they mean to go back to college, that they want to start saving. The reply such a\nline gets today comes from one pool that has to fit every statement at once (\"Got it. Is there\nanything you'd like a hand with?\"), and a line that would fit any message is the failure\nSTYLE.md names. But a plan has a shape, and a friend answers the shape: not with a stock nod,\nand not with a question about what it is doing (a thing that has gone wrong is\n`bot/compose/trouble.py`'s), but with the part people forget - the first step, and what has been\nin the way.\n\nA plan is read from cues that say the person means to do something over time (\"planning to\",\n\"want to\", \"saving up\", \"applying for\"), and from a \"going to\" or a \"thinking of\" whose next\nword is a thing you plan (\"going to move house\", \"thinking of moving\") rather than a thing you\ndo now (\"going to have a bath\") or the object of a thought (\"thinking about my mum\"). A line\nthat asks something is left alone too, whatever else it says: \"got any ideas for somewhere\nwarmer next year\" is a request for ideas, and \"should i move\" is a doubt, not a plan.\n\nThe other half of the same family is a report: a statement about something that has already\nhappened (\"the shop was shut\", \"the bus was late again\", \"i started a new job\"). It is not a\nquestion, so no module answers it; it is not a plan, so the above passes it; and it is not a\nfault of theirs to troubleshoot (that is `bot/compose/trouble.py`). What is left for it today is\nthe same stocking pool (\"Got it. Is there anything you'd like a hand with?\"), and a friend\nanswers a report the way they answer a plan: by its shape, with the feeling it deserves - a small\nsetback is a nuisance, a thing that keeps happening is \"again?\", and something new that has\nstarted is worth a question. The line is still left to the caller's own pools when it carries a\nfeeling, a loss or good news: those read better with the mood than with the shape.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_A_THING_TAKEN(pyrt::xrefs::xget("compose.fallback_data", "A_THING_TAKEN"));
    s_OTHER_PEOPLE(pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE"));
    s_without_leaders(py_func("without_leaders", |args: &[Py]| pyrt::xrefs::xcall("understanding.normalize", "without_leaders", args)));
    s_PLAN(py_tuple(vec![py_str("planning"), py_str("plan to"), py_str("planning to"), py_str("book a"), py_str("save up"), py_str("saving up"), py_str("saving for"), py_str("applying"), py_str("apply for")]));
    s_BOOKED(stdlib_call("re", "compile", &[py_str("\\bbook(?:ed|ing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("venue"), py_str("venues"), py_str("hall"), py_str("reception"), py_str("caterer"), py_str("caterers"), py_str("band"), py_str("dj"), py_str("photographer"), py_str("tickets"), py_str("flights"), py_str("flight"), py_str("hotel"), py_str("room"), py_str("table"), py_str("minibus")]) }
    s_STEP_NOUNS(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("wedding"), py_str("weddings"), py_str("venue"), py_str("honeymoon"), py_str("holiday"), py_str("trip"), py_str("course"), py_str("reception"), py_str("mortgage"), py_str("house"), py_str("move"), py_str("moving"), py_str("exam"), py_str("exams")]) }
    s_BIGGER_PLAN(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    s_ARRANGEMENT(call_builtin("frozenset", &[py_tuple(vec![py_str("visit"), py_str("visiting"), py_str("visits")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_TO(py_tuple(vec![py_str("going to "), py_str("gonna "), py_str("want to "), py_str("wanting to "), py_str("would like to "), py_str("hoping to "), py_str("thinking of "), py_str("thinking about ")]));
    fn __data3() -> Py { py_tuple(vec![py_str("start"), py_str("move"), py_str("save"), py_str("buy"), py_str("learn"), py_str("book"), py_str("travel"), py_str("visit"), py_str("change"), py_str("quit"), py_str("sell"), py_str("apply"), py_str("build"), py_str("train"), py_str("study"), py_str("open"), py_str("join"), py_str("switch"), py_str("retrain"), py_str("volunteer"), py_str("emigrate"), py_str("adopt"), py_str("redecorate"), py_str("renovate"), py_str("downsize"), py_str("sort"), py_str("fix"), py_str("paint"), py_str("clear"), py_str("declutter"), py_str("budget"), py_str("insulate"), py_str("convert"), py_str("rent"), py_str("plant"), py_str("grow"), py_str("write")]) }
    s_PLANNED_VERBS(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("died"), py_str("dying"), py_str("dead"), py_str("death"), py_str("passed"), py_str("funeral"), py_str("buried"), py_str("miscarriage"), py_str("cancer"), py_str("tumour"), py_str("tumor"), py_str("diagnosis"), py_str("hospital"), py_str("icu"), py_str("hospice"), py_str("suicide"), py_str("overdose"), py_str("abuse"), py_str("beaten"), py_str("assaulted"), py_str("attacked"), py_str("emergency"), py_str("ambulance"), py_str("bleeding"), py_str("chest"), py_str("stroke"), py_str("seizure"), py_str("unconscious"), py_str("miscarried"), py_str("grieving"), py_str("miscarrying"), py_str("divorce"), py_str("custody"), py_str("abortion"), py_str("abortions"), py_str("termination"), py_str("terminations"), py_str("crematorium"), py_str("cremation"), py_str("cremations"), py_str("stillbirth"), py_str("ectopic"), py_str("coffin"), py_str("scan"), py_str("appointment"), py_str("consultant"), py_str("clinic"), py_str("surgery"), py_str("gp"), py_str("doctor"), py_str("midwife"), py_str("chemo"), py_str("treatment"), py_str("results"), py_str("biopsy"), py_str("operation"), py_str("ward"), py_str("petrol"), py_str("gas"), py_str("gas leak"), py_str("fumes"), py_str("smoke"), py_str("smoking"), py_str("burning"), py_str("carbon"), py_str("monoxide"), py_str("sparks"), py_str("wiring"), py_str("fire"), py_str("breath"), py_str("breathless"), py_str("breathing"), py_str("breathlessly"), py_str("wheeze"), py_str("wheezing"), py_str("cough"), py_str("coughing"), py_str("dizzy"), py_str("faint"), py_str("palpitations"), py_str("pain"), py_str("hurts"), py_str("aching"), py_str("sore")]) }
    s_NOT_PLAIN(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTERFACTUAL(py_tuple(vec![py_str("would have been"), py_str("would've been"), py_str("would have turned"), py_str("wouldve been"), py_str("should have been"), py_str("never got to"), py_str("if only")]));
    s_MARKED_DATE(call_builtin("frozenset", &[py_tuple(vec![py_str("anniversary"), py_str("birthday"), py_str("birthdays"), py_str("remembrance"), py_str("memorial")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("want to die"), py_str("wanna die"), py_str("kill myself"), py_str("end it all"), py_str("end my life"), py_str("no point"), py_str("hurt myself"), py_str("self harm"), py_str("not worth living"), py_str("better off dead"), py_str("be here anymore"), py_str("dont want to be here"), py_str("don't want to be here"), py_str("dont want to wake up"), py_str("better off without me"), py_str("no way out"), py_str("tired of living"), py_str("cant go on"), py_str("overdosing")]) }
    s_CRISIS(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("what"), py_str("whats"), py_str("which"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("whos"), py_str("ideas"), py_str("idea"), py_str("suggest"), py_str("suggestions"), py_str("recommend"), py_str("advice"), py_str("tips"), py_str("anyone"), py_str("anybody"), py_str("shout")]) }
    s_ASKS(call_builtin("frozenset", &[__data6()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_tuple(vec![py_str("dont know"), py_str("don't know"), py_str("not sure"), py_str("unsure"), py_str("whether"), py_str("if i"), py_str("should i"), py_str("no idea"), py_str("in two minds"), py_str("not certain"), py_str("dont want"), py_str("don't want"), py_str("do not want"), py_str("dont fancy"), py_str("don't fancy"), py_str("leaving my husband"), py_str("leaving my wife"), py_str("leaving my partner"), py_str("leave my husband"), py_str("leave my wife"), py_str("leave my partner"), py_str("keep thinking about leaving"), py_str("thinking of leaving"), py_str("leaving him"), py_str("leaving her"), py_str("walking out on"), py_str("keep planning"), py_str("always planning"), py_str("never do"), py_str("never follow through"), py_str("things i never")]) }
    s_DOUBT(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("but"), py_str("except"), py_str("only"), py_str("never"), py_str("dont have"), py_str("don't have"), py_str("havent got"), py_str("haven't got"), py_str("cant"), py_str("can't"), py_str("no money"), py_str("no time"), py_str("too late"), py_str("already tried")]) }
    s_BARRIER(__data8());
    s_PLAN_REPLIES_PLAIN(py_tuple(vec![py_str("Good. What's the first step you're unsure about?"), py_str("Sounds like a good plan. What's the bit you're least sure about?")]));
    s_PLAN_REPLIES(py_tuple(vec![py_str("Good. What's the first step you're unsure about?"), py_str("Nice — worth doing properly. What would you want sorted out first?"), py_str("Worth deciding what the first step is, and what you'd count as done."), py_str("Sounds like a good plan. What's the bit you're least sure about?")]));
    s_DEPENDANT(call_builtin("frozenset", &[mcall(&py_str("\nson daughter child children kid kids toddler teenager teen lad boy girl baby grandson\nstepson stepdaughter granddaughter mum mom mother father dad wife husband partner grandchild\nnephew niece\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DEPENDANT_WANT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our|his|her|their)\\s+(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_DEPENDANT()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b[^.!?]*?\\b(?:wants?|wanted|needs?|would\\s+like|would\\s+love|is\\s+desperate\\s+for|keeps\\s+asking\\s+for|is\\s+asking\\s+for)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED_OF_THE_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:wants?|needs?|would\\s+like)\\s+(?:me|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_TO_BE(stdlib_call("re", "compile", &[py_str("\\bwants?\\s+to\\s+be\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WEAPON(call_builtin("frozenset", &[mcall(&py_str("\ngun guns shotgun shotguns pistol pistols handgun handguns revolver revolvers firearm firearms\nairgun airguns rifle rifles crossbow crossbows machete machetes taser tasers knuckleduster\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WEAPON_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("knife"), py_str("knives"), py_str("blade"), py_str("blades"), py_str("sword"), py_str("swords")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data9() -> Py { py_tuple(vec![py_str("cooking"), py_str("cook"), py_str("chef"), py_str("kitchen"), py_str("carving"), py_str("whittling"), py_str("craft"), py_str("fishing"), py_str("camping"), py_str("garden"), py_str("bread"), py_str("sharpening"), py_str("cutlery")]) }
    s_NOT_A_WEAPON(__data9());
    s_A_LOSS_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|death|dead|passed\\s+away|funeral|hospice|terminal|bereave\\w*|widow\\w*|cancer)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONDOLENCE(stdlib_call("re", "compile", &[py_str("\\bi(?:'m|\\s+am)\\s+(?:really\\s+|so\\s+|very\\s+)?sorry\\b|\\bsorry\\s+for\\s+your\\s+loss\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEPENDANT_WANT_REPLIES(py_tuple(vec![py_str("Then start with what it would really take - the price, and what the rules allow - before anyone says yes or no. What has been looked at so far?"), py_str("The asking is the easy half. Price it and check the rules first, because those two decide it more often than the wanting does."), py_str("Ask what it is really for first - the thing itself and what it gets them are different problems. What has been said so far?")]));
    s_SETBACK(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|got|went|kept|had)\\s+(?:\\w+\\s+){0,2}?(?:late|shut|closed|cancell?ed|delayed|sold\\s+out|broken|bust|packed\\s+in|refused|rejected|declined|postponed|overbooked|queueing|queuing|back\\s+in|let\\s+(?:me|us)\\s+down)\\b|\\b(?:broke\\s+down|packed\\s+in|gave\\s+up\\s+the\\s+ghost|ran\\s+out|fell\\s+through|let\\s+(?:me|us)\\s+down|missed\\s+(?:the|my|a|our)\\s+\\w+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_RISE(stdlib_call("re", "compile", &[py_str("\\b(?:went|gone|goes|has\\s+gone|have\\s+gone|has\\s+went|have\\s+went)\\s+up\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE(call_builtin("frozenset", &[mcall(&py_str("\nrent price prices gas electric electricity bill bills rate rates cost costs fare fares interest\nmortgage insurance heating petrol fuel diesel food shopping council tax stamp fees fee nursery\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSED(stdlib_call("re", "compile", &[py_str("\\bmissed\\s+(?:the|my|a|our|his|her|their)?\\s*(\\w+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_SLIP(stdlib_call("re", "compile", &[py_str("\\bmissed\\s+(?:the|my|a|our)\\s+(?:deadline|deadlines|cut-?off|interview)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DEADLINE_LATER(stdlib_call("re", "compile", &[py_str("\\b(?:accounts|confirmation\\s+statement|companies\\s+house|annual\\s+return|self\\s+assessment|statutory\\s+accounts|filing|court|hearing|tribunal)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRESENT_SETBACK(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|keeps?|kept)\\s+(?:\\w+\\s+){0,2}?(?:broken|shut|closed|cancell?ed|delayed|off|down|out\\s+of\\s+order|full|sold\\s+out)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURGLED(stdlib_call("re", "compile", &[py_str("\\b(?:burgled|burglary)\\b|\\b(?:broken\\s+into|broke\\s+into|broken\\s+in\\s+and|break-?in\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BURGLED_HOME(stdlib_call("re", "compile", &[py_str("\\b(?:house|home|flat|apartment|place|maisonette|bedsit|caravan|bungalow|cottage|rented|landlady|landlord)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data10() -> Py { py_tuple(vec![py_str("shed"), py_str("garage"), py_str("gate"), py_str("cupboard"), py_str("door"), py_str("doors"), py_str("lock"), py_str("window"), py_str("windows"), py_str("boot"), py_str("car"), py_str("van"), py_str("bike"), py_str("motorbike"), py_str("bicycle"), py_str("allotment"), py_str("container"), py_str("bin"), py_str("suitcase"), py_str("locker"), py_str("box"), py_str("wardrobe"), py_str("safe"), py_str("parcel"), py_str("packet")]) }
    s_NOT_A_BREAK_IN(call_builtin("frozenset", &[__data10()]).unwrap_or_else(|e| exc::raise(e)));
    s_BURGLED_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our|the)\\s+(?:mum|mom|dad|sister|brother|son|daughter|friend|mate|nan|gran|grandma|grandad|granny|aunt|uncle|cousin|partner|husband|wife|boyfriend|girlfriend|parents?|neighbours?|in-laws?|family|in-laws?)\\b|\\b(?:a\\s+mate|a\\s+friend)\\b|\\bthey\\s+(?:were|got|have\\s+been|had\\s+been)\\s+(?:burgled|broken\\s+into)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BURGLED_NOT_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:on\\s+(?:our|my|the)\\s+(?:street|road|estate|avenue|close)|next\\s+door|round\\s+the\\s+corner|down\\s+the\\s+road|in\\s+the\\s+area|locally|nearby)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_SETBACK(stdlib_call("re", "compile", &[py_str("\\b(?:shut|closed|cancell?ed|delayed|sold\\s+out)\\s+(?:early|late|today|yesterday|again|on\\s+me|before\\s+(?:i|we)\\s+(?:got|arrived))\\b|\\bnever\\s+(?:turned\\s+up|showed\\s+up)\\b|\\b(?:(?:was|were|got|has|have|had)\\s+(?:been\\s+)?(?:nicked|stolen|pinched))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEVER_CAME_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:i|you|he|she|we|they|him|her|them|mum|mom|dad|my\\s+(?:mum|mom|dad|sister|brother|son|daughter|friend|mate|partner|husband|wife|boyfriend|girlfriend))\\s+never\\s+(?:turned\\s+up|showed\\s+up)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_TO_ME(stdlib_call("re", "compile", &[py_str("\\b(?:got|was|were|has been|have been|had been)\\s+(?:\\w+\\s+){0,2}?(?:pickpocketed|pick-?pocketed|keyed|scratch(?:ed)?|dented|chipped|cracked|smashed|slashed|vandalised|vandalized|damaged|towed|clamped)\\b|\\b(?:my|our)\\s+(?:\\w+\\s+){0,3}?(?:ran off|ran away|bolted|escaped|got out|wandered off|went missing|gone missing|disappeared|vanished|cracked|stopped working|packed in)\\b|\\bi\\s+(?:chipped|cracked|broke|knocked out|lost)\\s+(?:a|an|my|one of my|two)\\s+\\w+\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SMASHED(stdlib_call("re", "compile", &[py_str("\\bsmashed\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SMASHED_OF_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their|its)\\s+(?:\\w+\\s+){0,3}?(?:got|was|were|has\\s+been|had\\s+been|smashed|broken|shattered)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GROUP_OF_PEOPLE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their)\\s+(?:team|side|club|lot|lads|boys|girls|mates|friends|family|kids|children|crowd|opposition|squad|band)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_OF_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:boss|manager|mum|mom|mother|dad|father|son|daughter|boy|girl|kid|kids|child|children|husband|wife|partner|boyfriend|girlfriend|ex|brother|sister|mate|mates|friend|friends|colleague|colleagues|coworker|coworkers|nana|nan|gran|grandad|grandma|grandpa|aunt|uncle|cousin|neighbour|neighbor|landlord|landlady|flatmate|housemate|teenager|baby|toddler)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_LOST(stdlib_call("re", "compile", &[py_str("\\b(?:went\\s+missing|gone\\s+missing|disappeared|vanished)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CRACKED_A_JOKE(stdlib_call("re", "compile", &[py_str("\\bcracked\\s+(?:a|up|his|her|their|its)\\s*(?:joke|smile|grin|laugh|voice|wise)|\\bcracked\\s+(?:on|it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_CARRIED_OFF(stdlib_call("re", "compile", &[py_str("\\b(?:ran|ran\\s+off|ran\\s+away|got\\s+out|escaped|went\\s+off|cracked|smashed|broke)\\s+(?:with|out)\\s+(?:my|our|the)\\b|\\b(?:cracked|smashed|broke|dented|scratched|chipped)\\s+(?:open\\s+)?(?:my|our)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NIGHT_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:party|pub|night\\s+out|works\\s+do|wedding|stag|hen|club|bar|drinks|round|celebration)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data11() -> Py { py_tuple(vec![py_str("sick"), py_str("ill"), py_str("unwell"), py_str("poorly"), py_str("vomit"), py_str("vomiting"), py_str("sickness"), py_str("fever"), py_str("temperature"), py_str("flu"), py_str("cough"), py_str("cold"), py_str("bug"), py_str("hospital"), py_str("ambulance"), py_str("a&e"), py_str("999"), py_str("111"), py_str("doctor"), py_str("gp"), py_str("nurse"), py_str("midwife"), py_str("pain"), py_str("hurts"), py_str("hurting"), py_str("ache"), py_str("aching"), py_str("sore"), py_str("swollen"), py_str("bleeding"), py_str("blood"), py_str("dizzy"), py_str("faint"), py_str("fainted"), py_str("breathless"), py_str("seizure"), py_str("fit"), py_str("rash"), py_str("chest"), py_str("stroke"), py_str("allergic"), py_str("reaction"), py_str("concussion"), py_str("wound"), py_str("burn"), py_str("burnt")]) }
    s_REPORT_BODY(call_builtin("frozenset", &[__data11()]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTED_VERDICT(stdlib_call("re", "compile", &[py_str("\\b(?:was|is|went|were|been|going|getting|seems|looks)\\s+(?:\\w+\\s+){0,2}?(?:fine|good|great|ok|okay|well|alright|amazing|lovely|brilliant|easy|hard|tough|awful|terrible)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTED_GONE_BADLY(stdlib_call("re", "compile", &[py_str("\\b(?:problem|problems|trouble|troubles|teething|snag|snags|hiccup|hiccups|issue|issues)\\b|\\b(?:hate|hates|hating|dread|dreading)\\b|\\b(?:going|gone|went|is|was|been|getting|feels?)\\s+(?:\\w+\\s+){0,2}?(?:badly|bad|awful|terrible|horrible|rough|hard|tough|stressful|draining|exhausting|rubbish|pants)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:again|again\\s+today|again\\s+this\\s+(?:week|morning|time)|third\\s+time|second\\s+time|every\\s+\\w+\\s+now|once\\s+more)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTED(stdlib_call("re", "compile", &[py_str("\\bstart(?:ed|ing)?\\s+(?:a\\s+|my\\s+|his\\s+|her\\s+|their\\s+|the\\s+)?(?:new\\s+)?(?:job|work|course|college|uni|university|school|placement|apprenticeship|shift|contract|role|internship|training)\\b|\\bnew\\s+(?:job|role|position)\\b|\\bfirst\\s+(?:day|week|shift)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AHEAD_DAY(py_str("(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december)"));
    s_START_AHEAD(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:(?:on|next|this)\\s+"), &g__AHEAD_DAY()), &py_str("|(?:next|this)\\s+(?:week|weekend|month|year|term|afternoon|evening)|tomorrow|tonight|soon|in\\s+(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|\\d+)\\s+(?:day|days|week|weeks|month|months)|in\\s+")), &g__AHEAD_DAY()), &py_str("|(?:is|are)\\s+(?:on\\s+|this\\s+|next\\s+)?")), &g__AHEAD_DAY()), &py_str("|(?:is|are)\\s+(?:tomorrow|tonight))\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_START_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:started|began|begun)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_START_PRESENT(stdlib_call("re", "compile", &[py_str("\\b(?:start|starts|starting|begin|begins|beginning)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_START_CLOCK(stdlib_call("re", "compile", &[py_str("\\bat\\s+\\d{1,2}(?::\\d{2})?\\b|\\b\\d{1,2}\\s*o'?clock\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data12() -> Py { py_tuple(vec![py_str("happy"), py_str("glad"), py_str("sad"), py_str("gutted"), py_str("scared"), py_str("afraid"), py_str("worried"), py_str("angry"), py_str("furious"), py_str("annoyed"), py_str("frustrated"), py_str("upset"), py_str("excited"), py_str("proud"), py_str("relieved"), py_str("tired"), py_str("exhausted"), py_str("fed"), py_str("sick"), py_str("awful"), py_str("terrible"), py_str("worst"), py_str("great"), py_str("brilliant"), py_str("amazing"), py_str("love"), py_str("hate"), py_str("cried"), py_str("crying"), py_str("tears"), py_str("worst"), py_str("nightmare"), py_str("disaster"), py_str("stress"), py_str("stressed"), py_str("anxious"), py_str("panic"), py_str("worst"), py_str("sorry"), py_str("unfortunately"), py_str("gutted")]) }
    s_REPORT_FEELING(call_builtin("frozenset", &[__data12()]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKEN_FROM_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:(?:my|our|his|her|their|the)\\s+\\w+\\s+(?:was|were|got|has|have|had)\\s+(?:been\\s+)?(?:nicked|stolen|swiped|pinched|lifted|thieved|snaffled|taken))\\b|\\bi\\s+had\\s+(?:my|our|his|her|their)\\s+\\w+\\s+(?:nicked|stolen|swiped|pinched|lifted|thieved|snaffled|taken)\\b|\\b(?:someone|somebody|some\\s+(?:lad|bloke|kid|guy|person|scrote)|they)\\s+(?:nicked|stole|swiped|pinched|lifted|took|thieved|snaffled)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_LOSS(stdlib_call("re", "compile", &[py_str("\\b(?:table|seat|seats|place|places|order|orders|turn|call|name|names|number|photo|photos|picture|pictures|temperature|blood|bloods|details|booking|form|form\\s+in|payment|space|spaces|spot|spots|parking|slot|slots)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TOOK_IT_IN(stdlib_call("re", "compile", &[py_str("\\b(?:took|taken|takes|take)\\b[^.]{0,25}\\bin\\b(?!\\s+(?:the\\s+)?(?:night|morning|afternoon|evening|day|daylight|hours|town|time|future|error|haste|early|late|later)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_FOLLOW(stdlib_call("re", "compile", &[py_str("\\bcards?\\b|\\bcash\\b|\\bmoney\\b|\\bkey|\\bkeys\\b|\\bphone\\b|\\bwallet\\b|\\bpurse\\b|\\bbag\\b|\\bbike\\b|\\bpassport\\b|\\binsurance\\b|\\binsurer\\b|\\bclaim\\b|\\bpolice\\b|\\bblock|\\bfreez|\\bcancel|\\bbank\\b|\\blaptop\\b|\\bcar\\b|\\bvan\\b|\\btools?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_FOLLOW_REPLIES(py_tuple(vec![py_str("Then stop the cards before anything else - most banking apps freeze them in one tap, and it is quicker than the phone queue. Ring 101 for a crime reference number and keep it, because the bank and the insurer both ask for it by name."), py_str("Money and cards are the urgent part: freeze them in the app now, then ring the bank and say taken, not lost. A stolen card is handled differently from one left on a bus, and the difference is the crime reference."), py_str("Insurance next, while it is fresh. The insurer wants the crime reference, a list of what went, and whatever receipts or photos you can find - and anything else damaged wants photographing before it is tidied up.")]));
    s_SETBACK_REPLIES(py_tuple(vec![py_str("Oh, that's a nuisance."), py_str("That's a pain."), py_str("Rotten luck."), py_str("Ugh, that's annoying.")]));
    s_AGAIN_REPLIES(py_tuple(vec![py_str("Again? That's a pain."), py_str("Again - that's annoying."), py_str("Again. That's a nuisance."), py_str("That keeps happening, then. Do you know why?"), py_str("Again? Has it been like that a while?")]));
    s_STARTED_REPLIES(py_tuple(vec![py_str("Nice. How's it going so far?"), py_str("Good. How were the first days?"), py_str("Nice one. How's it treating you?"), py_str("Good luck with it - how has the first week been?")]));
    s_STARTED_AHEAD_REPLIES(py_tuple(vec![py_str("Good luck with it - how are you feeling about it?"), py_str("Nice. What are you looking forward to?"), py_str("Good luck on the day. Anything you want sorted before then?"), py_str("A fresh start, then. What is the first thing you will do?"), py_str("Nice one. Are you looking forward to it, or dreading it a bit?")]));
    s_BURGLED_REPLIES(py_tuple(vec![py_str("That's a horrible thing to come back to. Ring the police first and get a crime reference number - the insurer asks for it - and tell the insurer before you clear anything up."), py_str("I'm sorry. Report it and get a crime reference number, then ring the insurer: they want to know before anything is tidied away, and they ask for that number first."), py_str("Awful. The order that matters is the police for a crime reference number, then the insurer, before you touch anything.")]));
    s_BURGLED_FOR_SOMEONE_REPLIES(py_tuple(vec![py_str("That's a horrible thing to happen to them. Are they all right, and is someone with them?"), py_str("Awful for them. Do they have somebody there with them tonight?"), py_str("That's a nasty thing to go through. How are they holding up?")]));
    s_SLIP_REPLIES(py_tuple(vec![py_str("Deadlines do that. How much time is left?"), py_str("It happens. What can you still get in?"), py_str("Easy to lose track of one. What is left to do?")]));
    s_STARTED_FINE_REPLIES(py_tuple(vec![py_str("Good - glad it's going all right."), py_str("Nice one. Sounds like a good start."), py_str("Good to hear it.")]));
    s_STARTED_BADLY_REPLIES(py_tuple(vec![py_str("That's a rough start. What's going wrong with it?"), py_str("Sorry it's been like that. What's the worst of it?"), py_str("Ah, that's a shame this early on. What's not working?"), py_str("A rough first stretch. What would make it easier?")]));
    s_STARTED_WELL(stdlib_call("re", "compile", &[py_str("\\b(?:fine|good|great|ok|okay|well|alright|amazing|lovely|brilliant|easy)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_STRETCH(stdlib_call("re", "compile", &[py_str("\\bfirst\\s+(?:day|week|shift|month)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REGRET(stdlib_call("re", "compile", &[py_str("\n    \\bregrets?\\b | \\bregretted\\b | \\bregretting\\b\n  | \\bwish\\s+(?:i|we)\\s+(?:had|did)\\s*(?:n'?t|nt| not)\\b\n  | \\bwish\\s+(?:i|we)\\s+had\\s+(?!a\\b|an\\b|the\\b|more\\b|enough\\b|some\\b|any\\b|no\\b\n                                     |another\\b|my\\b|your\\b|his\\b|her\\b|their\\b|our\\b\n                                     |one\\b|two\\b|three\\b|\\d)\\w+\\b\n  | \\b(?:not|never)\\s+sure\\s+(?:i|we)\\s+(?:should|ought\\s+to)\\b\n"), stdlib_attr("re", "VERBOSE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data13() -> Py { py_tuple(vec![py_str("died"), py_str("dying"), py_str("dead"), py_str("death"), py_str("funeral"), py_str("passed"), py_str("buried"), py_str("hospital"), py_str("cancer"), py_str("diagnosis"), py_str("chemo"), py_str("miscarriage"), py_str("stillbirth")]) }
    s_NO_REGRET(call_builtin("frozenset", &[__data13()]).unwrap_or_else(|e| exc::raise(e)));
    s_REGRET_LINES(py_tuple(vec![py_str("That's the thing about hindsight - it always turns up after. What would you do differently?"), py_str("Regret is worth listening to. If you could undo it, what would you do instead?"), py_str("That sounds like it has been sitting with you. What happened?"), py_str("A decision can turn out wrong without you being wrong to make it. What would you change?"), py_str("That is a fair thing to feel. What is the part you keep going back to?"), py_str("Easy to see it now. Would you do it differently next time?")]));
    s_REGRET_LINES_PLAIN(py_tuple(vec![py_str("Hindsight is always clearer than the decision was."), py_str("That is a lot to carry without it being a mistake."), py_str("Regret usually means the thing mattered to you."), py_str("You can tell me the part that keeps coming back.")]));
    s_FIRST_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|ive|we|weve)\\b(?:\\s+(?:really|so|only|just|still|already|now|also|kind\\s+of|sort\\s+of|do|did|dont|absolutely))*\\s+(?:regret|regrets|regretted|regretting|wish)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRACTICAL_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|should|shall|may)\\s+(?:i|we)\\s+\\w+|\\bhow\\s+do\\s+(?:i|we)\\s+\\w+|\\bhow\\s+can\\s+(?:i|we)\\s+\\w+|\\bwhere\\s+do\\s+(?:i|we)\\s+\\w+|\\bwhat\\s+do\\s+(?:i|we)\\s+do\\b|\\bis\\s+it\\s+worth\\b|\\b(can|could)\\s+(?:i|we)\\s+(?:still\\s+)?(?:return|refund|exchange|cancel|sell|fix|claim|complain|change|get\\s+(?:out|rid))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_POLITE(stdlib_call("re", "compile", &[py_str("\\bregret\\s+to\\s+(?:say|tell|inform|report|announce|admit|hear|learn|let)\\b|\\bregret\\s+(?:that\\s+)?(?:i|we)\\s+(?:cant|can'?t|cannot|wont|won'?t|am\\s+unable|are\\s+unable|am\\s+not\\s+able|must\\s+miss)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FUTURE_REGRET(stdlib_call("re", "compile", &[py_str("\\b(?:going to|gonna|will|won'?t|wont|shall)\\s+(?:\\w+\\s+){0,2}?regret\\b|\\bregret\\s+(?:this|it)\\s+(?:tomorrow|in the morning|later|after)\\b|\\byou(?:'?ll|ll|\\s+will)\\s+regret\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SHOULDNT_HAVE(stdlib_call("re", "compile", &[py_str("\\bshould(?:n'?t|nt|\\s+not)\\s+have(?!\\s+to\\b)\\s+(?:\\w+ed|said|told|bought|gone|done|left|taken|kept|booked|paid|sent|agreed|started|moved|quit|sold|given|made|let|put|drunk|eaten|spent|worn|seen|heard|written|driven|known)\\b|\\bshould\\s+have\\s+(?:said|told|bought|gone|done|kept|booked|paid|sent|left|taken|stayed|waited|asked|listened|started|stopped|quit|held|walked|stood)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_DOWN(stdlib_call("re", "compile", &[py_str("\\b(?:put|putting)\\s+(?:\\w+\\s+){0,2}?(?:down|to\\s+sleep)\\b|\\b(?:euthanis|euthaniz)\\w*")]).unwrap_or_else(|e| exc::raise(e)));
    s__OWN_SHOULD(stdlib_call("re", "compile", &[py_str("^\\s*(?:[a-z]+,?\\s+){0,2}(?:i|we)(?:'?ve|ve)?\\s+should")]).unwrap_or_else(|e| exc::raise(e)));
    s__HARM_AT_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:shout(?:ed|ing)?|snapp(?:ed|ing)|yell(?:ed|ing)|swore|screamed|scream(?:ed|ing)|lied|lash(?:ed)?\\s+out|sulk(?:ed)?|was\\s+(?:horrible|awful|mean|cruel|nasty))\\s+(?:at|to)\\s+(?:my|his|her|our|your|the)\\s+\\w+")]).unwrap_or_else(|e| exc::raise(e)));
    s_WISH_I(stdlib_call("re", "compile", &[py_str("^\\s*wish\\s+(?:i|we)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_PERSON(py_str("(?:mum|mom|mam|mother|dad|father|parents|sister|brother|son|daughter|kid|kids|child|children|boy|girl|eldest|youngest|baby|baby boy|baby girl|little one|little boy|little girl|little man|little lad|little miss|oldest|firstborn|toddler|gran|granny|nan|grandma|grandmother|grandad|grandpa|grandfather|grandparents|nana|aunt|uncle|cousin|partner|husband|wife|niece|nephew|grandson|granddaughter|grandchildren|grandkids|grandsons|granddaughters|stepson|stepdaughter|folks|inlaws|in-laws|friend|friends|mate|mates|best friend|best mate|boss|colleague|workmate|neighbour|neighbor|neighbours|neighbors|flatmate|housemate|roommate)"));
    s_THEIR_QUALIFIER(py_str("(?:other|eldest|oldest|youngest|little|big|small|wee|first|second|third|middle|baby|grown up|grownup|step|adopted|foster) "));
    s_THEIR_CHILD(py_str("(?:son|daughter|kid|kids|child|children|boy|girl|eldest|youngest|niece|baby|baby boy|baby girl|little one|little boy|little girl|little man|little lad|little miss|oldest|firstborn|toddler|nephew|grandson|granddaughter|grandkids|grandchildren|stepson|stepdaughter|baby|babies|toddler|toddlers|newborn|newborns|infant|infants|little one|little lad|little man|little fella|little fellow|little girl|little boy|wee one|wee man|wee lad|small one)"));
    s_THEIR_NAME_AGE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^my (?:"), &g_THEIR_QUALIFIER()), &py_str(")?(?P<person>")), &g_THEIR_CHILD()), &py_str(")(?:'?s)? (?:is called|is named|name is|name's|called|named) (?P<name>[a-z][a-z'-]{1,14})(?: and |, )?(?:she|he|they|it)(?:'?s| is| are) (?:just |only |nearly )?(?P<age>\\d{1,2})$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_NAME(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_PERSON()), &py_str(")(?:'?s)? (?:is called|is named|name is|name's) (?P<name>[a-z][a-z'-]{1,14})$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_WORDS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12))]));
    s_THEIR_AGE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^(?:my|our|the) (?:"), &g_THEIR_QUALIFIER()), &py_str(")?(?P<person>")), &g_THEIR_CHILD()), &py_str(")(?:'?s)? (?:is|are|is now|just turned|turns?) (?:just |only |nearly |about )?(?P<n>\\d{1,2}|")), &mcall(&py_str("|"), "join", &[g_AGE_WORDS()])), &py_str(")(?:\\s?(?:years?|yrs?\\.?|y/o|yo|months?|mths?\\.?|mth)(?:[ -]?old)?| old| now)?$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_BABY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("newborns"), py_str("infant"), py_str("infants")])]).unwrap_or_else(|e| exc::raise(e)));
    s_YEAR_UNIT(stdlib_call("re", "compile", &[py_str("\\b(?:years?|yrs?)\\b|\\d\\s*(?:years?|yrs?|y/o|y\\.o|yo|yr)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_PLACE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_PERSON()), &py_str(")(?:'?s)? (?:live|lives|are living|is living|are based|lives away) (?:in|near|up in|down in) (?P<place>[a-z][a-z '\\-]{1,24}?)$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_JOB(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_PERSON()), &py_str(")(?:'?s)? (?:is|works as) (?:a |an )?(?P<phrase>[a-z][a-z '\\-]{2,28})$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_JOB_LINES(py_tuple(vec![py_str("{Job}. What does a normal day look like for {them}?"), py_str("{Job} — how long has {they} been at it?"), py_str("{Job}. Does {they} work nights, or is it days?"), py_str("{Job}. Is that what {they} always wanted to do?")]));
    s_THEIR_ELDER(py_str("(?:nan|nana|gran|granny|grandma|grandmother|grandad|granddad|grandpa|grandfather|great gran|great nan|great grandma|great grandad|great aunt|great uncle)"));
    s_THEIR_ELDER_AGE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ELDER()), &py_str(")(?:'?s)? (?:is|is now|just turned|turns?) (?:just |only |nearly |about )?(?P<n>\\d{2,3})(?: (?:years? old|year old|now))?$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_ELDER_LINES(py_tuple(vec![py_str("{n} — that is a good innings. Does {they} still live on {their} own?"), py_str("{n}. Do you get over to see {them} much?"), py_str("{n} — good going. What does {they} put it down to?")]));
    s_NOT_A_NAME_AFTER_CALLED(call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten\n    eleven twelve once twice thrice again times in out round over back by up off through"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_CALLED(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_PERSON()), &py_str(")s? (?:is |are )?(?:called|named) (?P<names>[a-z][a-z'\\-]{1,14}(?: and [a-z][a-z'\\-]{1,14})?)(?: and (?:she|he|they)(?:'?s| is| are) (?:just |only |nearly )?(?P<age>\\d{1,2}))?$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_NAME_SO(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^my (?:"), &g_THEIR_QUALIFIER()), &py_str(")?(?P<person>")), &g_THEIR_CHILD()), &py_str(") (?P<name>[a-z][a-z'\\-]{1,14}) (?:is|is now|just turned|turns?) (?:just |only |nearly )?(?P<age>\\d{1,2})$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_LIKING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_PERSON()), &py_str(")(?:'?s)? (?:really |absolutely |totally |just |properly |so )?(?:loves?|adores?|likes?|enjoys?|collects?|is into|is really into|is mad about|is big on|is obsessed with|is a fan of|is a big fan of|is addicted to) (?P<thing>[a-z][a-z '\\-]{1,30})$"))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data14() -> Py { py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("them"), py_str("these"), py_str("those"), py_str("me"), py_str("you"), py_str("us"), py_str("him"), py_str("her"), py_str("his"), py_str("hers"), py_str("mine"), py_str("yours"), py_str("life"), py_str("everything"), py_str("anything")]) }
    s_NOT_A_THING(call_builtin("frozenset", &[__data14()]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_THING_IS_A_CLAUSE(py_tuple(vec![py_str(" when "), py_str(" when"), py_str(" if "), py_str(" if"), py_str(" because"), py_str(" but "), py_str(" that "), py_str(" until "), py_str(" after "), py_str(" before ")]));
    s_THEIR_DISLIKE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_PERSON()), &py_str(")(?:'?s)?(?: really | absolutely | actually )? (?:doesnt|does not|don'?t|do not|didnt|did not|wont|will not|wouldnt|never|can'?t|cannot|refuses to|refused to) (?:like|eat|touch|drink|have|take|stand|finish|wear|try|watch|read|go to|get on with) (?P<thing>[a-z][a-z '\\-]{1,30})$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_HATE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_PERSON()), &py_str(")(?:'?s)?(?: really | absolutely | actually )? (?:hates?|loathes?|detests?|can'?t stand|cannot stand|is not keen on|isn'?t keen on|is scared of|is frightened of) (?P<thing>[a-z][a-z '\\-]{1,30})$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_TASTE_AS_A_THING(call_builtin("frozenset", &[mcall(&py_str("\n    baby boy brother child children cousin dad daughter father gran grandad grandma grandpa\n    granny husband kid kids mam mom mother mum nan nana nephew niece partner sister son wife\n    job school work boss teacher shift money house home bed bedtime bath mornings nights\n    mondays tuesdays wednesdays thursdays fridays saturdays sundays\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TASTE_TOO_LONG(Py::Int(4));
    s_THEIR_DISLIKE_LINES(py_tuple(vec![py_str("{Thing} — noted. Good to know for {them}."), py_str("Fair enough — {thing} is not for {them}."), py_str("{Thing} — noted, and {they} can steer round it.")]));
    s_THEIR_HATE_LINES(py_tuple(vec![py_str("{Thing} — fair enough, and {they} can steer round it."), py_str("Hating {thing} is allowed, and {they} {are} in good company."), py_str("{Thing} — noted, with sympathy for {them}.")]));
    fn __data15() -> Py { py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("sister"), py_str("daughter"), py_str("wife"), py_str("niece"), py_str("gran"), py_str("granny"), py_str("nan"), py_str("nana"), py_str("grandma"), py_str("grandmother"), py_str("granddaughter"), py_str("aunt"), py_str("stepdaughter"), py_str("girl"), py_str("little girl")]) }
    s_SHE_PEOPLE(call_builtin("frozenset", &[__data15()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data16() -> Py { py_tuple(vec![py_str("dad"), py_str("father"), py_str("brother"), py_str("son"), py_str("husband"), py_str("nephew"), py_str("grandad"), py_str("grandpa"), py_str("grandfather"), py_str("grandson"), py_str("uncle"), py_str("stepson"), py_str("boy"), py_str("little boy"), py_str("little lad"), py_str("little man"), py_str("little fella"), py_str("little fellow"), py_str("wee man"), py_str("wee lad")]) }
    s_HE_PEOPLE(call_builtin("frozenset", &[__data16()]).unwrap_or_else(|e| exc::raise(e)));
    s_THEY_PERSON(py_tuple(vec![py_str("they"), py_str("them"), py_str("their"), py_str("do"), py_str("are")]));
    s_SHE_PERSON(py_tuple(vec![py_str("she"), py_str("her"), py_str("her"), py_str("does"), py_str("is")]));
    s_HE_PERSON(py_tuple(vec![py_str("he"), py_str("him"), py_str("his"), py_str("does"), py_str("is")]));
    s_THEIR_MOOD_WITH_YOU(stdlib_call("re", "compile", &[py_str("\\b(?:(?:my|our)\\s+(?:\\w+\\s+){0,2}?)?(?:mum|mom|mam|mother|dad|father|husband|wife|partner|boyfriend|girlfriend|sister|brother|son|daughter|friend|mate|boss|manager|colleague|flatmate|housemate|neighbour|neighbor|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|he|she|they)\\s+(?:was|were|is|are|has been|had been|'s been|been)\\s+(?:being\\s+)?(?:funny|off|short|sharp|snappy|offhand|cold|cool|distant|offish|surly|cross|irritable|grumpy|moody|brusque|abrupt|weird|odd|strange)\\s+with\\s+(?:me|us)\\b|\\b(?:he|she|they|my\\s+\\w+)\\s+snapped\\s+at\\s+me\\b|\\b(?:he|she|they|my\\s+\\w+)\\s+blanked\\s+me\\b|\\bgave\\s+me\\s+the\\s+cold\\s+shoulder\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_MOOD_WITH_YOU_LINES(py_tuple(vec![py_str("Being funny with you and not saying why is unsettling. Do you know what set {them} off?"), py_str("That is an odd way to be with you. Is it a one-off, or has it been like this a while?"), py_str("You are left guessing when {they} {do} that. Did {they} say anything at all?"), py_str("Being got at without knowing why wears you down. What happened just before it?")]));
    s_GOING_ON(stdlib_call("re", "compile", &[py_str("^(?:(?:it|that|this)s?\\s+)?(?:all|most|every)\\s+(?:the\\s+)?(?:time|weeks?|weekends?|days?|nights?)\\b|\\bits\\s+been\\s+(?:going\\s+on|like\\s+that|happening)\\s+(?:for|since|all|most|every)\\b|\\b(?:it|that|this|she|he|they)s?\\s+(?:been|has\\s+been|'s\\s+been)\\s+like\\s+(?:that|this)\\s+(?:all|for|since)\\b|(?:it|that|this|she|he|they)\\s+(?:happens|happened|does\\s+(?:it|that)|is\\s+like\\s+that|has\\s+been\\s+like\\s+(?:that|this)|keeps?\\s+happening|goes?\\s+on)[^.!?]{0,26}?\\b(?:most|every|all)\\s+(?:single\\s+|other\\s+)?(?:weeks?|weekends?|days?|nights?|mornings?|time|the\\s+time)\\b|\\b(?:she|he|they|it)\\s+(?:has\\s+been|had\\s+been|'s\\s+been|was|is)\\s+like\\s+(?:that|this)\\s+(?:all|for|since)\\b|\\b(?:it|this|that)\\s+(?:has\\s+been|had\\s+been|'s\\s+been)\\s+going\\s+on\\s+(?:for|since)\\b|\\b(?:it|this|that)\\s+(?:has\\s+been|had\\s+been|'s\\s+been)\\s+for\\s+(?:about\\s+)?(?:\\d+|a|an|two|three|four|five|six|seven|eight|nine|ten|a\\s+few|a\\s+couple\\s+of|several)\\s+(?:weeks?|months?|years?|days?)\\b|\\b(?:for|since)\\s+(?:about\\s+)?(?:\\d+|a|an|two|three|four|five|six|seven|eight|nine|ten|a\\s+few|a\\s+couple\\s+of|several)\\s+(?:weeks?|months?|years?|days?)\\b|\\bsince\\s+(?:january|february|march|april|may|june|july|august|september|october|november|december|christmas|last\\s+(?:week|month|year|summer|autumn|winter|spring))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_ON_LINES(py_tuple(vec![py_str("That long is a lot of guessing. Have you asked {them} straight out?"), py_str("Being off with you for that long stops being a mood and starts being a habit. Have you said anything to {them}?"), py_str("If it has been going on that long, it is worth saying to {them}, not sitting on. What would you want to say?")]));
    s_THEIR_NAME_LINES(py_tuple(vec![py_str("{name} — that is a good one. Where did it come from?"), py_str("{name}. Do you call {them} {name} all the time, or does it get shortened?"), py_str("{name}. Who picked it?")]));
    s_THEIR_NAME_GROWN_LINES(py_tuple(vec![py_str("{name} — that's a lovely name. Is it a family one?"), py_str("{name}. {Do} {they} get called anything for short?"), py_str("{name} — nice. Is there a story behind it?")]));
    s_A_CHILD_NAMED(call_builtin("frozenset", &[mcall(&mcall(&g_THEIR_CHILD(), "strip", &[py_str("(?:)")]), "split", &[py_str("|")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_NAME_AGE_LINES(py_tuple(vec![py_str("{name} at {n} — that is a good age. What {are} {they} into at the moment?"), py_str("{name}, {n}. What {are} {they} like just now?")]));
    s_THEIR_NAMES_LINES(py_tuple(vec![py_str("{names} — two good names. How old are they?"), py_str("{names}. What are they like, the pair of them?"), py_str("{names} — a houseful, then. How old are they?")]));
    s_THEIR_BABY_LINES(py_tuple(vec![py_str("{m} — are you getting any sleep?"), py_str("{m} old. Is it the nights that are hardest, or the days?"), py_str("{m} — that is the age everything goes in the mouth. Is that where you are?")]));
    s_THEIR_SMALL_LINES(py_tuple(vec![py_str("{n} is a busy age. What {are} {they} into at the moment?"), py_str("{n} — that is the age of asking why about everything. Does that sound right?"), py_str("{y}. Is it easier than the baby stage, or harder?")]));
    s_THEIR_CHILD_LINES(py_tuple(vec![py_str("{n} is a good age. What {are} {they} into at the moment?"), py_str("{y}. Is school going all right for {them}?"), py_str("{n} — what are the days like with {them} just now?")]));
    s_THEIR_TEEN_LINES(py_tuple(vec![py_str("{n} — is that a good age or a hard one?"), py_str("{y}. What {are} {they} like just now?"), py_str("{n} — do you get much of a look in these days?"), py_str("{n} — {do} {they} still want you around, or are you embarrassing now?")]));
    s_THEIR_GROWN_LINES(py_tuple(vec![py_str("{n} — grown, then. Still at home?"), py_str("{n} is a proper landmark. What {are} {they} doing now?"), py_str("{y}. {Do} {they} still let you fuss over {them}?")]));
    s_THEIR_PLACE_LINES(py_tuple(vec![py_str("{place}. How often do you get to see {them}?"), py_str("{place} — {do} {they} ever come to you instead?"), py_str("{place}. Do you get up there much, or is it {them} coming to you?")]));
    s_THEIR_LIKING_LINES(py_tuple(vec![py_str("{thing}. What got {them} into it?"), py_str("{thing} — good one. Is it a long-standing thing with {them}?"), py_str("{thing}. Do you get roped into it with {them} at all?")]));
    s_PLACE_CODES(py_dict(vec![(py_str("us"), py_str("US")), (py_str("usa"), py_str("USA")), (py_str("uk"), py_str("UK")), (py_str("uae"), py_str("UAE")), (py_str("eu"), py_str("EU")), (py_str("nz"), py_str("NZ")), (py_str("nyc"), py_str("NYC")), (py_str("drc"), py_str("DRC"))]));
    s_A_HELD_NAME_AND_AN_AGE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?P<name>[a-z][a-z'\\-]{1,19}) (?:is|is now|will be|turns|just turned) (?:just |only |nearly |about )?(?P<n>\\d{1,2}|"), &mcall(&py_str("|"), "join", &[g_AGE_WORDS()])), &py_str(")(?:\\s?(?:years?|yrs?\\.?|y/o|yo|months?|mths?\\.?|mth)(?:[ -]?old)?| old| now)?$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MILESTONE_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:next|on|this|in)\\s+(?:week|month|year|monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_PUT_RIGHT(py_tuple(vec![py_str("{name} at {n} now — I had {old}."), py_str("{name}'s {n}, then, not {old} — got it."), py_str("{name} at {n}. I had {old}, so that is the one thing I will change.")]));
    s_AGE_FIRST_TOLD(py_tuple(vec![py_str("{name} at {n}, then."), py_str("{name}, {n} — good to know."), py_str("{name} at {n}. That is one to remember.")]));
    s_AGE_ALREADY_HAD(py_tuple(vec![py_str("{name} at {n} — that is what I had."), py_str("{name}, {n}. I had that already, so nothing changes.")]));
    s_OWN_HAVE(stdlib_call("re", "compile", &[py_str("^(?:i|we)(?:'ve|ve)\\s+got\\s+(?P<one>.+)$|^(?:i|we)\\s+(?:have|has)\\s+(?:got\\s+)?(?P<two>[^?]+)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_SPACES(call_builtin("frozenset", &[mcall(&py_str("\nroom bedroom kitchen garage greenhouse shed garden allotment plot patch balcony patio loft cellar\nbasement workshop study conservatory hut outbuilding yard driveway orchard pond\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_KIT(call_builtin("frozenset", &[mcall(&py_str("\npiano guitar keyboard drums telescope bike bikes motorbike car caravan boat canoe kayak\nnetflix projector printer turntable bench washing machine dishwasher\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_KIT_NAMES(call_builtin("frozenset", &[py_tuple(vec![py_str("drum kit"), py_str("record player"), py_str("sewing machine"), py_str("hot tub"), py_str("log burner"), py_str("solar panels"), py_str("summer house")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data17() -> Py { py_dict(vec![(py_str("room"), py_str("{A} {thing} earns its keep as a bed, a desk or storage, and it is cheapest left as two of those")), (py_str("bedroom"), py_str("a spare bedroom earns its keep as a bed, a desk or storage")), (py_str("kitchen"), py_str("{A} {thing} earns its space back: a good surface and one heavy pan do more than a cupboard of gadgets")), (py_str("garage"), py_str("{A} {thing} is a store that can also take a car, and most of them end up too full for one")), (py_str("greenhouse"), py_str("{A} {thing} buys weeks at both ends of the season - tomatoes ripen in it and seedlings stop going leggy on a windowsill")), (py_str("shed"), py_str("{A} {thing} is where the tools stop rusting and the mower stops being in the way")), (py_str("garden"), py_str("{A} {thing} is the cheapest ground you will ever get for potatoes, beans and salad")), (py_str("allotment"), py_str("{A} {thing} is hard work the first year while the ground is cleared, and much less after that")), (py_str("plot"), py_str("{A} {thing} is hard work the first year while the ground is cleared, and much less after")), (py_str("patch"), py_str("{A} {thing} that size grows salad, herbs and a few beans, which is most of what people actually eat")), (py_str("balcony"), py_str("{A} {thing} grows more than people expect: tomatoes, herbs and salad all do well in pots out there")), (py_str("patio"), py_str("{A} {thing} earns its keep in pots - tomatoes and herbs do better against a warm wall")), (py_str("loft"), py_str("{A} {thing} is the cheapest storage a house has, as long as the damp stays out of it")), (py_str("cellar"), py_str("{A} {thing} keeps a steady cool temperature, which is what potatoes, squash and wine want")), (py_str("basement"), py_str("{A} {thing} keeps a steady cool temperature, which is what potatoes and squash want")), (py_str("workshop"), py_str("{A} {thing} is worth having for the bench more than the tools")), (py_str("study"), py_str("{A} {thing} is mostly a door that shuts: a desk and a quiet hour do the rest")), (py_str("conservatory"), py_str("{A} {thing} gets hot in summer and cold in winter, so it works best with plants in it and a chair")), (py_str("piano"), py_str("{A} {thing} is the easiest instrument to start from nothing on, and an acoustic one wants tuning about once a year")), (py_str("guitar"), py_str("{A} {thing} is the one you can pick up for ten minutes and put down again, which is how people actually learn")), (py_str("keyboard"), py_str("{A} {thing} is the quiet way to practise - headphones and nobody in the house hears it")), (py_str("drums"), py_str("drums are the loudest hobby a house can hold, so an electric kit and headphones keep the neighbours onside")), (py_str("drum kit"), py_str("drums are the loudest hobby a house can hold, so an electric kit and headphones keep the neighbours onside")), (py_str("bike"), py_str("{A} {thing} beats the car for anything under a couple of miles once parking is counted")), (py_str("bikes"), py_str("bikes beat the car for short trips once parking is counted, and two of them fit where one car will not")), (py_str("motorbike"), py_str("{A} {thing} is the cheap way through traffic and the expensive way to stay warm and dry")), (py_str("caravan"), py_str("{A} {thing} saves the hotel bill and costs you the towing, which is why most of them sit for months at a time")), (py_str("boat"), py_str("{A} {thing} is a hobby in itself - the upkeep is the part nobody warns you about")), (py_str("canoe"), py_str("a canoe gets you on water a boat cannot reach, and it goes on the roof of a car")), (py_str("kayak"), py_str("a kayak gets you on water a boat cannot reach, and it goes on the roof of a car")), (py_str("netflix"), py_str("the good part of netflix is that a film night costs nothing past the subscription, and you can stop halfway and finish it tomorrow")), (py_str("projector"), py_str("{A} {thing} turns a blank wall into a big screen, and it wants the room properly dark")), (py_str("sewing machine"), py_str("{A} {thing} pays for itself on repairs - a hem or a zip costs more to have done than the machine did")), (py_str("printer"), py_str("{A} {thing} earns its space on the parts nobody sells any more, not on ornaments")), (py_str("record player"), py_str("records are the format that makes you listen to a whole side")), (py_str("turntable"), py_str("records are the format that makes you listen to a whole side")), (py_str("pond"), py_str("{A} {thing} brings frogs and birds in and then more or less runs itself once the plants are in")), (py_str("bench"), py_str("{A} {thing} is where a hobby gets done properly, and one good vice is worth more than the rest of it")), (py_str("car"), py_str("{A} {thing} costs more to keep than people plan for, and the short trips are what wear it hardest")), (py_str("washing machine"), py_str("{A} {thing} is worth mending once before it is replaced - the pump and the door seal are the two that go, and both are cheap parts")), (py_str("dishwasher"), py_str("{A} {thing} uses less water than washing up by hand, and the salt is the part people forget")), (py_str("hot tub"), py_str("{A} {thing} costs more to run than to buy - the heater and the chemicals are the bill nobody plans for")), (py_str("log burner"), py_str("{A} {thing} heats the one room it is in well, and it wants dry wood or the chimney soots up")), (py_str("solar panels"), py_str("solar panels pay back over something like ten years on a roof that faces the sun, and much less on one that does not")), (py_str("summer house"), py_str("{A} {thing} is a room the garden gets three months out of, so treat it like a shed and keep the damp out"))]) }
    s_OWN_FACTS(__data17());
    s_OWN_SPACES_PLAIN(py_str("{A} {thing} takes the mess and the work out of the middle of the house, which is what makes it worth having"));
    s_OWN_KIT_PLAIN(py_str("{A} {thing} is worth owning rather than borrowing, which is the point of it"));
    s_OWN_ELECTRIC(stdlib_call("re", "compile", &[py_str("\\b(?:electric|electrical|ev|hybrid)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_ELECTRIC_CAR(py_str("{A} {thing} is cheapest on the short trips that wear a petrol one out, and the cold and a motorway run are what use the charge up fastest"));
    s_OWN_ASKS(py_tuple(vec![py_str("What do you use it for?"), py_str("What have you done with it so far?"), py_str("Is it doing the job you wanted?"), py_str("What would you change about it?")]));
    s_OWN_KIT_ASKS(py_tuple(vec![py_str("Does it get much use?"), py_str("How long have you had it?"), py_str("Is it what you hoped for?"), py_str("Has it been worth what it cost?")]));
    s_OWN_NOT_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:problem|idea|lot|few|bit|chance|job|question|go|look|word|feeling|guess|break|rest|while|minute|moment|day|week|month|year|time|filo|hint|tip|note|deal|point|reason|choice|decision|say|think|try|best|crack|go)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_TWO_THINGS(stdlib_call("re", "compile", &[py_str("\\b(?:and|or|plus|also|with)\\b|&|/|,|\\b(?:but|no|not|never|without|except|apart from|instead of|rather than)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:leak\\w*|broken|broke\\w*|cracked|cracks|blocked|faulty|fault|stopped working|not working|dead|damp|mould|mold|smell\\w*|flood\\w*|noisy|rattl\\w*|jammed|stuck|cold|flu|bug|temperature|cough|sore|ache|pain|migraine|rash|infection|sick)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|tomorrow|morning|afternoon|evening|night|weekend|monday|tuesday|wednesday|thursday|friday|saturday|sunday|week|month|year|o'clock)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_ARTICLE(py_tuple(vec![py_str("a "), py_str("an "), py_str("the "), py_str("one "), py_str("two "), py_str("three "), py_str("four "), py_str("some "), py_str("another ")]));
    s_OWN_TAIL(stdlib_call("re", "compile", &[py_str("\\s+(?:by the .*|at the .*|in the .*|out the .*|out back|in back|upstairs|downstairs|out front|round the .*|down the .*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s__RELATION_PERSON(py_str("(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|best friend|bestie|friend|flatmate|housemate|mother in law|father in law|in laws)"));
    s_MARRIAGE_IN_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:marriage|relationship)\\s+(?:is|has|feels)\\s+(?:on the rocks|rocky|in trouble|in a bad way|falling apart|a mess|not in a good place|over|dead)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BAD_WITH_THEM(stdlib_call("re", "compile", &[op_modulo(&py_str("\\bthings (?:are|have been|arent|are not|aint)\\s+(?:bad|rough|hard|rubbish|terrible|awful|not good|not great|not the same)\\s+(?:with|between)\\s+(?:my|our)\\s+(%s)\\b"), &g__RELATION_PERSON())]).unwrap_or_else(|e| exc::raise(e)));
    s_QUIET_BETWEEN_THEM(stdlib_call("re", "compile", &[op_modulo(&py_str("\\b(?:me and|i and|myself and)\\s+(?:my|our)\\s+(%s)\\s+(?:barely|hardly|never)\\s+(?:talk|speak)\\b|\\b(?:my|our)\\s+(%s)\\s+and\\s+i\\s+(?:barely|hardly|never|dont|don't|do not)\\s+(?:talk|speak|get on)\\b|\\bwe\\s+(?:barely|hardly|never)\\s+(?:talk|speak)\\b|\\b(?:my|our)\\s+(%s)\\s+(?:is|has been|seems|feels|sounds)\\s+(?:distant|cold|withdrawn|off with me|cold with me|distant with me)\\b"), &py_tuple(vec![g__RELATION_PERSON(), g__RELATION_PERSON(), g__RELATION_PERSON()]))]).unwrap_or_else(|e| exc::raise(e)));
    s_ROWS_BETWEEN_THEM(stdlib_call("re", "compile", &[op_modulo(&py_str("\\b(?:me and|i and)\\s+(?:my|our)\\s+(%s)\\s+(?:keep\\s+)?(?:argue|arguing|row|rowing|fight|fighting|bicker|bickering)\\b|\\b(?:my|our)\\s+(%s)\\s+and\\s+i\\s+(?:keep\\s+)?(?:argue|arguing|row|rowing|fight|fighting|bicker|bickering)\\b|\\bwe\\s+(?:keep\\s+)?(?:argue|arguing|row|rowing)\\b"), &py_tuple(vec![g__RELATION_PERSON(), g__RELATION_PERSON()]))]).unwrap_or_else(|e| exc::raise(e)));
    s_RELATION_GONE_WRONG(stdlib_call("re", "compile", &[op_modulo(&py_str("\\b(?:my|our)\\s+(%s)\\s+and\\s+i\\s+(?:dont|don't|do not)\\s+get on\\b|\\b(?:dont|don't|do not)\\s+get on with\\s+(?:my|our)\\s+(%s)\\b|\\bwe (?:are|keep|have been)\\s+(?:growing|drifting) apart\\b"), &py_tuple(vec![g__RELATION_PERSON(), g__RELATION_PERSON()]))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_RELATION_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:hurt|hurts|hurting|injur\\w*|hospital|ambulance|a and e|stroke|seizure|fit|fits|collapsed|collapse|bleeding|blood|overdose|cancer|tumour|tumor|dementia|alzheimer\\w*|chemo\\w*|died|dying|death|funeral|drink|drinks|drinking|alcohol|drunk|hit|hits|hitting|punch\\w*|knife|gun|police|threat\\w*|abuse\\w*|affair|cheat\\w*|divorce|cheated)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_QUIET_LINES(py_tuple(vec![py_str("That is a lonely way to live. When did {pair} last have a proper talk?"), py_str("Sorry to hear it. What is an evening at home like for {pair} now?"), py_str("That is hard going. What happened the last time {pair} tried to talk?")]));
    s_THE_ROWS_LINES(py_tuple(vec![py_str("That wears a person down. What do the rows tend to be about?"), py_str("That is exhausting. Is there one thing that sets it off with {who}?"), py_str("Nothing I can fix from here, but I can listen. What was the last row about?")]));
    s_THE_RELATION_LINES(py_tuple(vec![py_str("That is a hard place to be. What has changed with {who}?"), py_str("Sorry to hear it. What has gone wrong with {who} lately?"), py_str("That is a heavy thing to carry. What does a bad week look like for {pair}?")]));
    fn __data18() -> Py { py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("them"), py_str("they"), py_str("everything"), py_str("anything"), py_str("nothing"), py_str("myself"), py_str("me"), py_str("him"), py_str("her"), py_str("past"), py_str("life"), py_str("death"), py_str("everything"), py_str("people"), py_str("someone"), py_str("somebody"), py_str("anyone"), py_str("anybody"), py_str("childhood"), py_str("then"), py_str("before")]) }
    s_THOUGHT_OBJECT(op_bitor(&call_builtin("frozenset", &[__data18()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE")));
    s_ARRANGED(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|i'?ve|we'?ve|ive|weve)\\s+(?:(?:have|had|just|finally|only|already)\\s+)*booked\\b|\\b(?:i'?m|im|we'?re|were)\\s+booking\\b|\\b(?:is|are|was|were|it's|its)\\s+(?:now\\s+|all\\s+|already\\s+|finally\\s+)?booked\\b|\\b[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,2}\\s+booked\\s+(?:for|on|at|in|next|this|tomorrow|today|tonight|yesterday|[0-9])\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBLE_BOOKED(stdlib_call("re", "compile", &[py_str("\\b(?:double|over|re)[- ]?book\\w*")]).unwrap_or_else(|e| exc::raise(e)));
    s_WENT_WRONG(stdlib_call("re", "compile", &[py_str("\\b(?:wrong|cancel(?:s|led|ling|lation)?|refund(?:s|ed|ing)?|charges?|charged|overcharged|expired|expires|can'?t|cant|cannot|unable|couldn'?t|couldnt|didn'?t|didnt|doesn'?t|doesnt|won'?t|wont|never|lost|missed|regret(?:s|ted|ting)?|dread(?:ing)?|fell\\s+through|mucked\\s+up)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TURNED(stdlib_call("re", "compile", &[py_str("\\b(?:but|though|although|except|only)\\b|\\band\\s+(?:now|then)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BOOKED_REPLIES(py_tuple(vec![py_str("Good - that's one off the list. When is it?"), py_str("Nice one. Is it soon, or have you got a while yet?"), py_str("Sorted. Is there anything left to sort before it?"), py_str("Good - that's one off the list.")]));
    s_ALREADY_DATED(py_dict(vec![(py_str("Good - that's one off the list. When is it?"), py_str("Good - that's one off the list.")), (py_str("Nice one. Is it soon, or have you got a while yet?"), py_str("Nice one. Anything left to sort before then?"))]));
    s_QUESTION_FIRST(stdlib_call("re", "compile", &[py_str("^\\W*(?:is|are|was|were|do|does|did|can|could|should|shall|will|would|have|has|am)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PERSON_WITH_IT(stdlib_call("re", "compile", &[py_str("^(?:i|we|i'?m|im|i'?ve|ive|we'?re|were|we'?ve|weve)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAD_IT_BOOKED(stdlib_call("re", "compile", &[py_str("\\bhad\\b(?:\\s+\\w+){0,5}\\s+booked\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INTEREST_DOING(py_tuple(vec![py_str("How's it going with the {thing}?"), py_str("{Thing} - how long have you been at it?"), py_str("What got you into {thing}?"), py_str("How are you finding the {thing}?"), py_str("How's {thing} treating you?")]));
    s_INTEREST_STARTED(py_tuple(vec![py_str("How are the first weeks of the {thing} going?"), py_str("Nice one - what have you got so far with the {thing}?"), py_str("Are you going to a class for the {thing}, or working it out yourself?"), py_str("What made you start {thing}?"), py_str("Good luck with the {thing}. How are you finding it?")]));
    s_INTEREST_DOING_PLAIN(py_tuple(vec![py_str("There's a lot in the {thing} worth talking about, so say more when you want to."), py_str("Good thing to be into. I'm happy to talk about the {thing} any time.")]));
    s_INTEREST_STARTED_PLAIN(py_tuple(vec![py_str("The first few weeks of the {thing} are the hard part. Say how it goes."), py_str("Good luck with the {thing} - it gets easier once it is a habit.")]));
    s_INTEREST_BACK(py_tuple(vec![py_str("What got you back into {thing}?"), py_str("Welcome back to it - what made you pick {thing} up again?"), py_str("How's the {thing} going the second time round?")]));
    s_INTEREST_BACK_PLAIN(py_tuple(vec![py_str("Good to hear you're back at it. Say how the {thing} goes."), py_str("Nice that you went back to the {thing} - it comes back faster the second time.")]));
    s_NOT_AN_INTEREST(call_builtin("frozenset", &[mcall(&py_str("work job jobs shift shifts nights overtime rota house home\n    cleaning ironing washing shopping money trouble debt him her them you me it this that\n    everything anything nothing something trouble bother fights arguments"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_good_night_out(args: &[Py]) -> Py {
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
    let _ = py_str("True when \"smashed\" means drunk, so the setback reading stands down.\n\n    \"we were smashed\" and \"got smashed last night\" have no thing of theirs in front of them, so\n    they are the night out. \"the lads got smashed at the party\" is people who drank. Every other\n    thing said to be smashed - \"my phone got smashed\", \"my glasses got smashed\" - is broken.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SMASHED(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SMASHED_OF_A_THING(), "search", &[clean.clone()])))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_A_GROUP_OF_PEOPLE(), "search", &[clean.clone()])) {
    return call_builtin("bool", &[mcall(&g_A_NIGHT_OUT(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn done_to_me(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line reports something done to a thing of theirs, for the pools that cheer.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DONE_TO_ME(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { { let __b = _a_good_night_out(&[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CRACKED_A_JOKE(), "search", &[clean.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_PERSON_OF_THEIRS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_PERSON_LOST(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_THING_CARRIED_OFF(), "search", &[clean.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_thing_taken(args: &[Py]) -> Py {
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
    let _ = py_str("True for a report of something of theirs taken by somebody else.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TAKEN_FROM_THEM(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_A_LOSS(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TOOK_IT_IN(), "search", &[text.clone()]))) } };
    Py::None
}
pub fn _the_loss_was_asked(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last reply was the loss pool asking its question.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&if py_truthy(&turns.clone()) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return Py::Bool(py_contains(&pyrt::xrefs::xget("compose.fallback_data", "A_THING_TAKEN"), &last.clone()));
    Py::None
}
pub fn _the_answers_take_it(args: &[Py]) -> Py {
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
    let mut answers: Py = Py::None;
    let _ = py_str("Whether the knowledge module answers this line itself.\n\n    Asked the same way `fallback.py` asks it for the identity card: the module that owns the\n    answer says whether it has one, and a hit that comes back empty is not an answer.\n    ");
    let __try1 = exc::catch(|| {
    answers = pyrt::xrefs::xmodule("modules.answers");
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&mcall(&answers.clone(), "respond", &[clean.clone(), py_dict(vec![])]), &Py::None))]);
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
pub fn _missed_a_person(args: &[Py]) -> Py {
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
    let _ = py_str("True when \"missed\" is about a person: \"i missed my dad\" is not a bus that went without me.");
    let __iter1 = py_iter_or_raise(&mcall(&g_MISSED(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE"), &mcall(&match_.clone(), "group", &[Py::Int(1)])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_report(args: &[Py]) -> Py {
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
    let _ = py_str("'setback', 'again', 'started' or None: the shape of the thing they told us.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_REPORT_FEELING()); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_REPORT_BODY()) } }) {
    return Py::None;
    }
    if py_truthy(&_missed_a_person(&[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&_reads_as_a_break_in(&[clean.clone(), words.clone()])) {
    return py_str("burgled");
    }
    if py_truthy(&mcall(&g_STARTED(), "search", &[clean.clone()])) {
    return py_str("started");
    }
    if py_truthy(&mcall(&g_OWN_SLIP(), "search", &[clean.clone()])) {
    return py_str("slip");
    }
    if py_truthy(&{ let __b = mcall(&g_MISSED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_DEADLINE_LATER(), "search", &[clean.clone()]) } }) {
    return py_str("slip");
    }
    if py_truthy(&{ let __b = mcall(&g_BARE_SETBACK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_NEVER_CAME_SUBJECT(), "search", &[clean.clone()]) } }) {
    return if py_truthy(&mcall(&g_AGAIN(), "search", &[clean.clone()])) { py_str("again") } else { Py::None };
    }
    if py_truthy(&{ let __b = mcall(&g_SETBACK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BARE_SETBACK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = done_to_me(&[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_RISE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PRICE()) } } } } } } } }) {
    return if py_truthy(&mcall(&g_AGAIN(), "search", &[clean.clone()])) { py_str("again") } else { py_str("setback") };
    }
    if py_truthy(&{ let __b = mcall(&g_AGAIN(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_PRESENT_SETBACK(), "search", &[clean.clone()]) } }) {
    return py_str("again");
    }
    return Py::None;
    Py::None
}
pub fn _still_to_come(args: &[Py]) -> Py {
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
    let _ = py_str("A start the person is telling the bot about before it has happened.\n\n    Two ways a start is still ahead: a day or a stretch of time that has not come (\"i start my\n    new job monday\", \"my first day is thursday\"), or a verb in the present with no day at all\n    (\"i start a new job\" - a start named and not yet had). A past verb makes it a report whatever\n    day is in the line, and a clock time (\"i start work at 9\") is a shift today, not a date.\n    ");
    if py_truthy(&{ let __b = mcall(&g_START_PAST(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_START_CLOCK(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_START_AHEAD(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_START_PRESENT(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _started_badly(args: &[Py]) -> Py {
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
    let mut bad: Py = Py::None;
    let _ = py_str("True when a start is said with something wrong in it and nothing to say it came right.\n\n    \"a few teething problems but going ok\" says both, and the good half is the one the person\n    means, so the order of the words decides it.\n    ");
    bad = mcall(&g_STARTED_GONE_BADLY(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bad.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_is(&mcall(&g_STARTED_WELL(), "search", &[clean.clone(), mcall(&bad.clone(), "end", &[])]), &Py::None));
    Py::None
}
pub fn _reads_as_a_break_in(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a home being entered, rather than a thing to repair.\n\n    \"burgled\" always is one. \"broken into\" is also a shed door, a gate and a cupboard, and those\n    keep the hinge answer, so with that verb the home has to be named and no repairable thing may\n    be in the line.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_BURGLED(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_BURGLED_NOT_THEIRS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_BURGLED_SOMEONE_ELSE(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:burgled|burglary)\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_BREAK_IN())) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_BURGLED_HOME(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:i|we|they)\\s+(?:was|were|got|have\\s+been|had\\s+been)\\s+(?:broken\\s+into|broken\\s+in)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn break_in_reply_for(args: &[Py]) -> Py {
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
    let mut pool: Py = Py::None;
    let _ = py_str("The reply to a home being broken into, or None when the line is not one.\n\n    Its own function so the pools that answer a sad day can stand aside for it: \"someone broke\n    into our house\" is read as bad news and answered \"Ugh, that's a lot to carry\", which is a\n    shrug at a break-in.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_reads_as_a_break_in(&[clean.clone(), words.clone()])))) {
    return Py::None;
    }
    pool = if py_truthy(&mcall(&g_BURGLED_SOMEONE_ELSE(), "search", &[clean.clone()])) { g_BURGLED_FOR_SOMEONE_REPLIES() } else { g_BURGLED_REPLIES() };
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn report_reply_for(args: &[Py]) -> Py {
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
    let mut o: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("A reply to a report of something that happened, or None to leave it to the caller.");
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_PLAIN()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_CRISIS()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _has_phrase(&[clean.clone(), g_COUNTERFACTUAL()]); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MARKED_DATE()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ASKS()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_DOUBT()]) } }) {
    return Py::None;
    }
    shape = reads_as_report(&[clean.clone(), words.clone()]);
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("burgled")))) {
    return break_in_reply_for(&[clean.clone(), words.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = _a_thing_taken(&[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_answers_take_it(&[clean.clone()]))) } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[pyrt::xrefs::xget("compose.fallback_data", "A_THING_TAKEN"), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = _the_loss_was_asked(&[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ASKS()))); if !py_truthy(&__b) { __b } else { mcall(&g_LOSS_FOLLOW(), "search", &[clean.clone()]) } } } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_LOSS_FOLLOW_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&shape.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&shape.clone(), &py_str("started"))); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.trouble", "reads_as_trouble", &[clean.clone(), words.clone(), clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shape.clone(), &py_str("started"))); if !py_truthy(&__b) { __b } else { _started_badly(&[clean.clone()]) } }) {
    if py_truthy(&mcall(&g_FIRST_STRETCH(), "search", &[clean.clone()])) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[g_STARTED_BADLY_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shape.clone(), &py_str("started"))); if !py_truthy(&__b) { __b } else { mcall(&g_STARTED_VERDICT(), "search", &[clean.clone()]) } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_STARTED_FINE_REPLIES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shape.clone(), &py_str("started"))); if !py_truthy(&__b) { __b } else { _still_to_come(&[clean.clone()]) } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_STARTED_AHEAD_REPLIES(), memory.clone(), clean.clone()]);
    }
    pool = geti(&py_dict(vec![(py_str("setback"), g_SETBACK_REPLIES()), (py_str("again"), g_AGAIN_REPLIES()), (py_str("started"), g_STARTED_REPLIES()), (py_str("slip"), g_SLIP_REPLIES())]), &shape.clone());
    if py_truthy(&{ let __b = _asked_last(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&shape.clone(), &py_str("started"))) } }) {
    pool = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&o.clone(), "endswith", &[py_str("?")])))) {  __out.push(o.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { pool.clone() } };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn reads_as_regret(args: &[Py]) -> Py {
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
    else { py_tuple(vec![]) };
    let _ = py_str("True when the line is a regret the person is telling the bot about.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("(?:do|does|did|would|have|are|is|was|were|can|could)\\s+(?:you|u)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NO_REGRET()); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_PLAIN()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _has_phrase(&[clean.clone(), g_CRISIS()]); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_COUNTERFACTUAL()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_FIRST_PERSON(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WISH_I(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__OWN_SHOULD(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_SHOULDNT_HAVE(), "search", &[clean.clone()]) } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_POLITE(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_PRACTICAL_ASK(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_FUTURE_REGRET(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_PUT_DOWN(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__HARM_AT_PERSON(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:no|dont|don't|never|not)\\s+regrets?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bregrets?\\s+(?:nothing|none)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("regret a thing"))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_REGRET(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SHOULDNT_HAVE(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn regret_reply_for(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { py_str("") };
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A reply to a regret, or None to leave the line to the caller.\n\n    `repaired` is the same line with its chat-speak put right, as the rest of the composer\n    reads one (bot/understanding's `plain`). The words above are written for the standard\n    spelling, so \"i shudnt have said anything\" was a regret when it was typed with its\n    apostrophe and a small account of the day when it was not - the same feeling, two turns\n    depending on how fast the person typed. The typed line is asked first, so a line that\n    reads with no repair is untouched.\n    ");
    said = clean.clone();
    words = { let __b = words.clone(); if py_truthy(&__b) { __b } else { mcall(&clean.clone(), "split", &[]) } };
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_regret(&[clean.clone(), words.clone()])))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { reads_as_regret(&[repaired.clone(), mcall(&repaired.clone(), "split", &[])]) } } } }))) {
    return Py::None;
    }
    said = repaired.clone();
    words = mcall(&repaired.clone(), "split", &[]);
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ASKS()); if py_truthy(&__b) { __b } else { _has_phrase(&[said.clone(), g_DOUBT()]) } }) {
    return Py::None;
    }
    pool = if py_truthy(&_asked_last(&[memory.clone()])) { g_REGRET_LINES_PLAIN() } else { g_REGRET_LINES() };
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), said.clone()]);
    Py::None
}
pub fn _a_taste_they_refuse(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut hang: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut person: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("A reply to a line that is only what one of their people will not have, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_THEIR_DISLIKE(), g_THEIR_DISLIKE_LINES()]), py_tuple(vec![g_THEIR_HATE(), g_THEIR_HATE_LINES()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    pool = __unpk2[1].clone();
    match_ = mcall(&pattern.clone(), "match", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    thing = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "lower", &[]), "split", &[])]);
    bare = stdlib_call("re", "sub", &[py_str("^(?:the|a|an|his|her|their|its) "), py_str(""), thing.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_THING_IS_A_CLAUSE()) { let mut hang: Py = __item.clone(); __out.push(Py::Bool(py_contains(&op_add(&op_add(&py_str(" "), &thing.clone()), &py_str(" ")), &hang.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&bare.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_TASTE_TOO_LONG()) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &bare.clone())); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[mcall(&bare.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_TASTE_AS_A_THING()) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_AGE_WORDS(), &geti(&mcall(&bare.clone(), "split", &[]), &Py::Int(0)))); if py_truthy(&__b) { __b } else { mcall(&bare.clone(), "isdigit", &[]) } }) {
    continue;
    }
    thing = stdlib_call("re", "sub", &[py_str("\\b(?:my|our)\\b"), py_str("your"), thing.clone()]).unwrap_or_else(|e| exc::raise(e));
    person = mcall(&mcall(&match_.clone(), "group", &[py_str("person")]), "strip", &[]);
    return _pick_filled(&[pool.clone(), memory.clone(), clean.clone(), { let bare = bare.clone(); let clean = clean.clone(); let hang = hang.clone(); let match_ = match_.clone(); let memory = memory.clone(); let pattern = pattern.clone(); let person = person.clone(); let pool = pool.clone(); let thing = thing.clone(); py_func("lambda", move |args: &[Py]| { let bare = bare.clone(); let clean = clean.clone(); let hang = hang.clone(); let match_ = match_.clone(); let memory = memory.clone(); let pattern = pattern.clone(); let person = person.clone(); let pool = pool.clone(); let thing = thing.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); let thing: Py = __kwp(args, 1, "thing").unwrap_or_else(|| thing.clone()); let person: Py = __kwp(args, 2, "person").unwrap_or_else(|| person.clone()); fill_person(&[mcall(&mcall(&line.clone(), "replace", &[py_str("{Thing}"), op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))]), "replace", &[py_str("{thing}"), thing.clone()]), person.clone()]) }) }]);
    }
    return Py::None;
    Py::None
}
pub fn person_pronouns(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut person_word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person_word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person_word"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The pronoun, the object and the verb forms for the person word in the line.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = person_word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_SHE_PEOPLE(), &word.clone()))) {
    return g_SHE_PERSON();
    }
    if py_truthy(&Py::Bool(py_contains(&g_HE_PEOPLE(), &word.clone()))) {
    return g_HE_PERSON();
    }
    return g_THEY_PERSON();
    Py::None
}
pub fn fill_person(args: &[Py]) -> Py {
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
    let mut person_word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person_word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person_word"))) };
    let mut are: Py = Py::None;
    let mut do_: Py = Py::None;
    let mut filled: Py = Py::None;
    let mut their: Py = Py::None;
    let mut them: Py = Py::None;
    let mut they: Py = Py::None;
    let _ = py_str("The pool line with the person's own pronouns and the verb that agrees with them.\n\n    The placeholders come in both cases, because a person's pronoun and the verb that agrees\n    with it can start a sentence as well as sit inside one: \"{y}. {Do} {they} still let you fuss\n    over {them}?\" was \"18 years old. Do he still let you fuss over him?\" while a hard \"Does\"\n    would have said \"Does they\" to a cousin. A line that starts on a placeholder is capitalised\n    for the same reason.\n    ");
    let __unpk1 = py_unpack(&person_pronouns(&[person_word.clone()]), 5, None);
    they = __unpk1[0].clone();
    them = __unpk1[1].clone();
    their = __unpk1[2].clone();
    do_ = __unpk1[3].clone();
    are = __unpk1[4].clone();
    filled = mcall(&mcall(&mcall(&mcall(&mcall(&mcall(&mcall(&mcall(&mcall(&mcall(&line.clone(), "replace", &[py_str("{they}"), they.clone()]), "replace", &[py_str("{them}"), them.clone()]), "replace", &[py_str("{their}"), their.clone()]), "replace", &[py_str("{do}"), do_.clone()]), "replace", &[py_str("{are}"), are.clone()]), "replace", &[py_str("{They}"), mcall(&they.clone(), "capitalize", &[])]), "replace", &[py_str("{Them}"), mcall(&them.clone(), "capitalize", &[])]), "replace", &[py_str("{Their}"), mcall(&their.clone(), "capitalize", &[])]), "replace", &[py_str("{Do}"), mcall(&do_.clone(), "capitalize", &[])]), "replace", &[py_str("{Are}"), mcall(&are.clone(), "capitalize", &[])]);
    return filled.clone();
    Py::None
}
pub fn their_mood_going_on_reply_for(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut person: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("\"it happens most weeks\" after a mood with them, or None to leave the line to the caller.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    kept = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("mood_with_you")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[{ let __b = mcall(&kept.clone(), "get", &[py_str("at")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&op_le(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(2)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_GOING_ON(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_has_phrase(&[clean.clone(), g_CRISIS()])) {
    return Py::None;
    }
    person = _pronoun_person(&[{ let __b = mcall(&kept.clone(), "get", &[py_str("person")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return _pick_filled(&[g_GOING_ON_LINES(), memory.clone(), clean.clone(), { let clean = clean.clone(); let kept = kept.clone(); let memory = memory.clone(); let person = person.clone(); let turns = turns.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let kept = kept.clone(); let memory = memory.clone(); let person = person.clone(); let turns = turns.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[line.clone(), person.clone()]) }) }]);
    Py::None
}
pub fn _pronoun_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut person: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let _ = py_str("The relation whose pronouns are the ones in the line: \"he snapped\" is a husband here.\n\n    The pronoun sets in this file are keyed by the relation (\"husband\", \"wife\"), so a bare \"he\"\n    filled every line with \"they\" - \"You are left guessing when they do that\" about a line that\n    said he. Anything else is passed through as it is (an empty word means \"they\").\n    ");
    return mcall(&py_dict(vec![(py_str("he"), py_str("husband")), (py_str("she"), py_str("wife")), (py_str("they"), py_str(""))]), "get", &[person.clone(), person.clone()]);
    Py::None
}
pub fn their_mood_with_you_reply_for(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut person: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A reply to \"somebody was off with me\", or None to leave the line to whoever owns it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    match_ = mcall(&g_THEIR_MOOD_WITH_YOU(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&clean.clone(), "split", &[])]);
    if py_truthy(&{ let __b = _has_phrase(&[said.clone(), g_CRISIS()]); if py_truthy(&__b) { __b } else { _has_phrase(&[said.clone(), g_NOT_PLAIN()]) } }) {
    return Py::None;
    }
    person = py_str("");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("dad"), py_str("father"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("sister"), py_str("brother"), py_str("son"), py_str("daughter"), py_str("friend"), py_str("mate"), py_str("boss"), py_str("manager"), py_str("colleague"), py_str("flatmate"), py_str("housemate"), py_str("neighbour"), py_str("neighbor"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("he"), py_str("she"), py_str("they")]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    person = word.clone();
    break;
    }
    }
    person = _pronoun_person(&[person.clone()]);
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&memory.clone(), &py_str("mood_with_you"), py_dict(vec![(py_str("person"), person.clone()), (py_str("at"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)))]));
    }
    return _pick_filled(&[g_THEIR_MOOD_WITH_YOU_LINES(), memory.clone(), clean.clone(), { let clean = clean.clone(); let match_ = match_.clone(); let memory = memory.clone(); let person = person.clone(); let said = said.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let match_ = match_.clone(); let memory = memory.clone(); let person = person.clone(); let said = said.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[line.clone(), person.clone()]) }) }]);
    Py::None
}
pub fn name_a_place(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut place: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place as a person would write it, for a pool line that opens with it.");
    place = mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    lead = py_str("");
    if py_truthy(&{ let __b = mcall(&mcall(&place.clone(), "lower", &[]), "startswith", &[py_str("the ")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[place.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![py_str("The "), slice_or_raise(&place.clone(), &Py::Int(4), &Py::None, &Py::None)]), 2, None);
    lead = __unpk1[0].clone();
    place = __unpk1[1].clone();
    }
    return op_add(&lead.clone(), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&place.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&g_PLACE_CODES(), "get", &[mcall(&word.clone(), "lower", &[]), mcall(&word.clone(), "title", &[])])); } py_list(__out) })() }]));
    Py::None
}
pub fn fill_age(args: &[Py]) -> Py {
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
    let mut n: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: n"))) };
    let mut months: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "months") { v.clone() }
    else { Py::Bool(false) };
    let mut count: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("The pool line with the age filled in: a one-year-old is \"1 year old\", not \"1 years\".");
    years = { let mut __s = String::new(); __s.push_str(&fstr(&n.clone(), -1, "")); __s.push_str(" year"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(!py_eq(&n.clone(), &Py::Int(1)))) { py_str("s") } else { py_str("") }, -1, "")); __s.push_str(" old"); py_string(__s) };
    count = if py_truthy(&months.clone()) { { let mut __s = String::new(); __s.push_str(&fstr(&n.clone(), -1, "")); __s.push_str(" month"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(!py_eq(&n.clone(), &Py::Int(1)))) { py_str("s") } else { py_str("") }, -1, "")); py_string(__s) } } else { years.clone() };
    return mcall(&mcall(&mcall(&line.clone(), "replace", &[py_str("{y}"), years.clone()]), "replace", &[py_str("{m}"), count.clone()]), "replace", &[py_str("{n}"), call_builtin("str", &[n.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _age_pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut n: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: n"))) };
    let mut months: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "months") { v.clone() }
    else { Py::Bool(false) };
    let _ = py_str("The pool for a child of this age: a baby is a different conversation from a grown one.");
    if py_truthy(&{ let __b = months.clone(); if !py_truthy(&__b) { __b } else { op_le(&n.clone(), &Py::Int(30)) } }) {
    return g_THEIR_BABY_LINES();
    }
    if py_truthy(&op_lt(&n.clone(), &Py::Int(5))) {
    return g_THEIR_SMALL_LINES();
    }
    if py_truthy(&op_lt(&n.clone(), &Py::Int(13))) {
    return g_THEIR_CHILD_LINES();
    }
    if py_truthy(&op_lt(&n.clone(), &Py::Int(18))) {
    return g_THEIR_TEEN_LINES();
    }
    return g_THEIR_GROWN_LINES();
    Py::None
}
pub fn _the_job_in(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The job the phrase names, written the way a person would, or None if it names no job.\n\n    \"nurse\" and \"builder\" are on the store's list. \"care assistant\" is the same trade with a word\n    in front, so a two-word phrase counts when one of its words does. \"of thirty years\" is not a\n    job however the line is read, and more than two words is a description, not a trade.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s'-]+"), mcall(&call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("memory.facts", "JOB_WORDS"))))) {
    return Py::None;
    }
    return mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "title", &[]);
    Py::None
}
pub fn _pick_filled(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut fill: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fill") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fill"))) };
    let mut line: Py = Py::None;
    let _ = py_str("One of the pool, chosen after the line is filled in.\n\n    The pools carry the person's word, the number and the pronouns as placeholders, and the picker\n    keeps the reply just given out of the choice. Choosing a template and filling it afterwards\n    hides the line from the picker, so \"my brother is a chef\" then \"he works in town\" asked \"what\n    does a normal day look like for him?\" twice running. Filling first, then choosing, is the same\n    reply with the repeat actually visible to it.\n    ");
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push(py_call(&fill, &[line.clone()])); } py_list(__out) })() }, memory.clone(), clean.clone()]);
    Py::None
}
pub fn _the_people_in(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("Every entry the store holds for that name, the people and the animals.");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&held.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &name.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[entry.clone()]);
    }
    }
    theirs = if py_truthy(&call_builtin("isinstance", &[mcall(&held.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&held.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&theirs.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    entry = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &name.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[entry.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_age_i_had(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fact: Py = Py::None;
    let mut held: Py = Py::None;
    let mut old: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The age an earlier turn gave that name, or None: the store overwrites it in place.");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    old = Py::None;
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&held.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    fact = __item.clone();
    value = if py_truthy(&call_builtin("isinstance", &[fact.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&fact.clone(), "get", &[py_str("value")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &name.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&value.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { geti(&value.clone(), &py_str("age")) } }) {
    old = geti(&value.clone(), &py_str("age"));
    }
    }
    return old.clone();
    Py::None
}
pub fn _an_age_put_right(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut n: Py = Py::None;
    let mut name: Py = Py::None;
    let mut old: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spelled: Py = Py::None;
    let _ = py_str("A known child or animal, named alone with a new age, said back with the age it had.");
    if py_truthy(&mcall(&g_A_MILESTONE_TAIL(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::None;
    }
    match_ = mcall(&g_A_HELD_NAME_AND_AN_AGE(), "match", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    name = mcall(&match_.clone(), "group", &[py_str("name")]);
    found = _the_people_in(&[memory.clone(), name.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    said = mcall(&match_.clone(), "group", &[py_str("n")]);
    n = if py_truthy(&mcall(&said.clone(), "isdigit", &[])) { call_builtin("int", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { geti(&g_AGE_WORDS(), &said.clone()) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&n.clone())); if py_truthy(&__b) { __b } else { op_gt(&n.clone(), &Py::Int(120)) } }) {
    return Py::None;
    }
    spelled = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&found.clone(), &Py::Int(0)), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { name.clone() } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "title", &[]);
    old = _the_age_i_had(&[name.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[old.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&old.clone())) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("name".to_string(), spelled.clone()), ("n".to_string(), n.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_AGE_FIRST_TOLD(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&old.clone(), &n.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("name".to_string(), spelled.clone()), ("n".to_string(), n.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_AGE_ALREADY_HAD(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("name".to_string(), spelled.clone()), ("n".to_string(), n.clone()), ("old".to_string(), old.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_AGE_PUT_RIGHT(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn their_people_reply_for(args: &[Py]) -> Py {
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
    let mut job: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut months: Py = Py::None;
    let mut n: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut person: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut refused: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A reply to a plain fact about one of their people, or None to leave it to the caller.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    clean = { let __b = pyrt::xrefs::xcall("understanding.normalize", "without_leaders", &[clean.clone()]); if py_truthy(&__b) { __b } else { clean.clone() } };
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_PLAIN()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_CRISIS()]) } }) {
    return Py::None;
    }
    refused = _a_taste_they_refuse(&[clean.clone(), memory.clone()]);
    if py_truthy(&refused.clone()) {
    return refused.clone();
    }
    put_right = _an_age_put_right(&[clean.clone(), memory.clone()]);
    if py_truthy(&put_right.clone()) {
    return put_right.clone();
    }
    match_ = mcall(&g_THEIR_ELDER_AGE(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    said = mcall(&match_.clone(), "group", &[py_str("n")]);
    n = if py_truthy(&mcall(&said.clone(), "isdigit", &[])) { call_builtin("int", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { geti(&g_AGE_WORDS(), &said.clone()) };
    return _pick_filled(&[g_THEIR_ELDER_LINES(), memory.clone(), clean.clone(), { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[fill_age(&[line.clone(), n.clone()]), mcall(&match_.clone(), "group", &[py_str("person")])]) }) }]);
    }
    match_ = mcall(&g_THEIR_JOB(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    job = _the_job_in(&[mcall(&match_.clone(), "group", &[py_str("phrase")])]);
    if py_truthy(&job.clone()) {
    return _pick_filled(&[g_THEIR_JOB_LINES(), memory.clone(), clean.clone(), { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[mcall(&line.clone(), "replace", &[py_str("{Job}"), job.clone()]), mcall(&match_.clone(), "group", &[py_str("person")])]) }) }]);
    }
    }
    match_ = mcall(&g_THEIR_PLACE(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    place = name_a_place(&[mcall(&match_.clone(), "group", &[py_str("place")])]);
    return _pick_filled(&[g_THEIR_PLACE_LINES(), memory.clone(), clean.clone(), { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[mcall(&line.clone(), "replace", &[py_str("{place}"), place.clone()]), mcall(&match_.clone(), "group", &[py_str("person")])]) }) }]);
    }
    match_ = mcall(&g_THEIR_NAME_AGE(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    name = mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("name")]), "strip", &[]), "title", &[]);
    n = call_builtin("str", &[call_builtin("int", &[mcall(&match_.clone(), "group", &[py_str("age")])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return _pick_filled(&[g_THEIR_NAME_AGE_LINES(), memory.clone(), clean.clone(), { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[mcall(&mcall(&line.clone(), "replace", &[py_str("{name}"), name.clone()]), "replace", &[py_str("{n}"), n.clone()]), mcall(&match_.clone(), "group", &[py_str("person")])]) }) }]);
    }
    match_ = mcall(&g_THEIR_AGE(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    months = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("months?|mths?|\\bmth\\b"), mcall(&match_.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&match_.clone(), "group", &[py_str("n")]);
    n = if py_truthy(&mcall(&said.clone(), "isdigit", &[])) { call_builtin("int", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { geti(&g_AGE_WORDS(), &said.clone()) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&months.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "isdigit", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BABY_WORDS(), &mcall(&match_.clone(), "group", &[py_str("person")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_YEAR_UNIT(), "search", &[mcall(&match_.clone(), "group", &[Py::Int(0)])]))) } } } } } }) {
    if py_truthy(&op_gt(&n.clone(), &Py::Int(24))) {
    return Py::None;
    }
    months = Py::Bool(true);
    }
    return _pick_filled(&[_age_pool(&[n.clone(), months.clone()]), memory.clone(), clean.clone(), { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[fill_age(&[line.clone(), n.clone(), months.clone()]), mcall(&match_.clone(), "group", &[py_str("person")])]) }) }]);
    }
    match_ = mcall(&g_THEIR_NAME(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    name = mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("name")]), "strip", &[]), "title", &[]);
    person = mcall(&mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("person")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    pool = if py_truthy(&Py::Bool(py_contains(&g_A_CHILD_NAMED(), &person.clone()))) { g_THEIR_NAME_LINES() } else { g_THEIR_NAME_GROWN_LINES() };
    return _pick_filled(&[pool.clone(), memory.clone(), clean.clone(), { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[mcall(&line.clone(), "replace", &[py_str("{name}"), name.clone()]), mcall(&match_.clone(), "group", &[py_str("person")])]) }) }]);
    line = mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_THEIR_NAME_LINES(), memory.clone(), clean.clone()]), "replace", &[py_str("{name}"), name.clone()]);
    return fill_person(&[line.clone(), mcall(&match_.clone(), "group", &[py_str("person")])]);
    }
    match_ = mcall(&g_THEIR_NAME_SO(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    name = mcall(&mcall(&match_.clone(), "group", &[py_str("name")]), "strip", &[]);
    if py_truthy(&pyrt::xrefs::xcall("memory.facts", "a_name_could_be", &[mcall(&name.clone(), "lower", &[])])) {
    line = pyrt::xrefs::xcall("compose", "pick", &[g_THEIR_NAME_AGE_LINES(), memory.clone(), clean.clone()]);
    line = mcall(&mcall(&line.clone(), "replace", &[py_str("{name}"), mcall(&name.clone(), "title", &[])]), "replace", &[py_str("{n}"), call_builtin("str", &[call_builtin("int", &[mcall(&match_.clone(), "group", &[py_str("age")])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    return fill_person(&[line.clone(), mcall(&match_.clone(), "group", &[py_str("person")])]);
    }
    }
    match_ = mcall(&g_THEIR_CALLED(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[py_str("names")]), "split", &[py_str(" and ")])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[])); } py_list(__out) })() };
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = pyrt::xrefs::xcall("memory.facts", "a_name_could_be", &[mcall(&word.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_NAME_AFTER_CALLED(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    said = mcall(&py_str(" and "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "title", &[])); } py_list(__out) })() }]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_THEIR_NAMES_LINES(), memory.clone(), clean.clone()]), "replace", &[py_str("{names}"), said.clone()]);
    }
    if py_truthy(&mcall(&match_.clone(), "group", &[py_str("age")])) {
    line = pyrt::xrefs::xcall("compose", "pick", &[g_THEIR_NAME_AGE_LINES(), memory.clone(), clean.clone()]);
    return mcall(&mcall(&fill_person(&[line.clone(), mcall(&match_.clone(), "group", &[py_str("person")])]), "replace", &[py_str("{name}"), said.clone()]), "replace", &[py_str("{n}"), call_builtin("str", &[call_builtin("int", &[mcall(&match_.clone(), "group", &[py_str("age")])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    line = mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_THEIR_NAME_LINES(), memory.clone(), clean.clone()]), "replace", &[py_str("{name}"), said.clone()]);
    return fill_person(&[line.clone(), mcall(&match_.clone(), "group", &[py_str("person")])]);
    }
    }
    match_ = mcall(&g_THEIR_LIKING(), "match", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    thing = mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "strip", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0))))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_THING_IS_A_CLAUSE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" "); py_string(__s) }, &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    said = op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None));
    return _pick_filled(&[g_THEIR_LIKING_LINES(), memory.clone(), clean.clone(), { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let job = job.clone(); let line = line.clone(); let match_ = match_.clone(); let memory = memory.clone(); let months = months.clone(); let n = n.clone(); let name = name.clone(); let names = names.clone(); let person = person.clone(); let place = place.clone(); let pool = pool.clone(); let put_right = put_right.clone(); let refused = refused.clone(); let said = said.clone(); let thing = thing.clone(); let word = word.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); fill_person(&[mcall(&line.clone(), "replace", &[py_str("{thing}"), said.clone()]), mcall(&match_.clone(), "group", &[py_str("person")])]) }) }]);
    }
    return Py::None;
    Py::None
}
pub fn _the_thing_they_have(args: &[Py]) -> Py {
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
    let mut article: Py = Py::None;
    let mut key: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the line says they have, and which class it is in, or None.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]), "rstrip", &[py_str(".!,")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    match_ = mcall(&g_OWN_HAVE(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    thing = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("one")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("two")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", &[]);
    let __iter1 = py_iter_or_raise(&g_OWN_ARTICLE());
    for __item in __iter1 {
    article = __item.clone();
    if py_truthy(&mcall(&thing.clone(), "startswith", &[article.clone()])) {
    thing = slice_or_raise(&thing.clone(), &call_builtin("len", &[article.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    break;
    }
    }
    thing = mcall(&mcall(&g_OWN_TAIL(), "sub", &[py_str(""), thing.clone()]), "strip", &[]);
    words = mcall(&thing.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_OWN_NOT_THE_THING(), "search", &[thing.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_OWN_FAULT(), "search", &[thing.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_OWN_TIME(), "search", &[thing.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_OWN_TWO_THINGS(), "search", &[thing.clone()])) {
    return Py::None;
    }
    key = _the_class_word(&[thing.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![thing.clone(), if py_truthy(&Py::Bool(py_contains(&g_OWN_SPACES(), &key.clone()))) { py_str("space") } else { py_str("kit") }]);
    Py::None
}
pub fn _the_class_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut size: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The longest ending of the phrase that names a space or a piece of kit, or \"\".\n\n    Longest first, so \"sewing machine\" is not read as a machine and \"record player\" is not read as\n    a player - the two-word names are things of their own.\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Int(3), Py::Int(2), Py::Int(1)]));
    for __item in __iter1 {
    size = __item.clone();
    if py_truthy(&op_ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone())) {
    tail = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &op_neg(&size.clone()), &Py::None, &Py::None)]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OWN_SPACES(), &tail.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OWN_KIT(), &tail.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OWN_KIT_NAMES(), &tail.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWN_FACTS(), &tail.clone())) } } } } } }) {
    return tail.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _article(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut word: Py = Py::None;
    let _ = py_str("A or An, by the sound the first word makes: \"an allotment\", \"a spare room\".");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("A");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("aeiou"), &geti(&word.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "startswith", &[py_tuple(vec![py_str("one"), py_str("uni"), py_str("use"), py_str("euro")])]))) } }) {
    return py_str("An");
    }
    return py_str("A");
    Py::None
}
pub fn _the_fact_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut article: Py = Py::None;
    let mut fact: Py = Py::None;
    let mut key: Py = Py::None;
    let _ = py_str("The one true thing about this kind of thing, filled in the way the person said it.\n\n    Matched on the head noun, so \"a small patch\" finds the patch, \"the spare room upstairs\" finds\n    the room and \"a big garden\" finds the garden. With no fact of its own the class line stands,\n    because a reply that only asks a question is not an answer.\n    ");
    article = _article(&[thing.clone()]);
    key = { let __b = _the_class_word(&[thing.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    fact = mcall(&g_OWN_FACTS(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("car"))); if !py_truthy(&__b) { __b } else { mcall(&g_OWN_ELECTRIC(), "search", &[thing.clone()]) } }) {
    fact = g_OWN_ELECTRIC_CAR();
    }
    if py_truthy(&Py::Bool(!py_truthy(&fact.clone()))) {
    fact = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("space")))) { g_OWN_SPACES_PLAIN() } else { g_OWN_KIT_PLAIN() };
    }
    fact = mcall(&mcall(&fact.clone(), "replace", &[py_str("{A}"), article.clone()]), "replace", &[py_str("{thing}"), thing.clone()]);
    return fact.clone();
    Py::None
}
pub fn _capped(args: &[Py]) -> Py {
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
    return if py_truthy(&line.clone()) { op_add(&mcall(&geti(&line.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&line.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { line.clone() };
    Py::None
}
pub fn a_thing_of_theirs_reply_for(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut asks: Py = Py::None;
    let mut fact: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("A reply to a plain fact about a thing they have, or None to leave it to the caller.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_PLAIN()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_CRISIS()]) } }) {
    return Py::None;
    }
    found = _the_thing_they_have(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    thing = __unpk1[0].clone();
    kind = __unpk1[1].clone();
    fact = _capped(&[_the_fact_about(&[thing.clone(), kind.clone()])]);
    asks = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("space")))) { g_OWN_ASKS() } else { g_OWN_KIT_ASKS() };
    return _pick_filled(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asks.clone()) { let mut ask: Py = __item.clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&fact.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&ask.clone(), -1, "")); py_string(__s) }); } py_list(__out) })() }, memory.clone(), clean.clone(), { let ask = ask.clone(); let asks = asks.clone(); let clean = clean.clone(); let fact = fact.clone(); let found = found.clone(); let kind = kind.clone(); let memory = memory.clone(); let thing = thing.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let ask = ask.clone(); let asks = asks.clone(); let clean = clean.clone(); let fact = fact.clone(); let found = found.clone(); let kind = kind.clone(); let memory = memory.clone(); let thing = thing.clone(); let words = words.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); line.clone() }) }]);
    Py::None
}
pub fn the_two_of_them(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("A reply to a relationship said to be in trouble, or None to leave it to the caller.\n\n    The line has no question in it, so nothing reads it as one: without this the store files it\n    as a fact and says it back, or the floor says it is listening, and both of those answer a\n    marriage the way they answer a bus timetable.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_A_RELATION_TROUBLE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    pool = Py::None;
    if py_truthy(&mcall(&g_QUIET_BETWEEN_THEM(), "search", &[clean.clone()])) {
    pool = g_THE_QUIET_LINES();
    } else {
    if py_truthy(&mcall(&g_ROWS_BETWEEN_THEM(), "search", &[clean.clone()])) {
    pool = g_THE_ROWS_LINES();
    } else {
    if py_truthy(&{ let __b = mcall(&g_MARRIAGE_IN_TROUBLE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BAD_WITH_THEM(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_RELATION_GONE_WRONG(), "search", &[clean.clone()]) } } } }) {
    pool = g_THE_RELATION_LINES();
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    found = stdlib_call("re", "search", &[op_modulo(&py_str("\\b(?:my|our)\\s+(%s)\\b"), &g__RELATION_PERSON()), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    who = if py_truthy(&found.clone()) { op_add(&py_str("your "), &mcall(&found.clone(), "group", &[Py::Int(1)])) } else { py_str("the two of you") };
    pair = if py_truthy(&found.clone()) { op_add(&py_str("you and your "), &mcall(&found.clone(), "group", &[Py::Int(1)])) } else { py_str("the two of you") };
    return _pick_filled(&[pool.clone(), memory.clone(), clean.clone(), { let clean = clean.clone(); let found = found.clone(); let memory = memory.clone(); let pair = pair.clone(); let pool = pool.clone(); let who = who.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let found = found.clone(); let memory = memory.clone(); let pair = pair.clone(); let pool = pool.clone(); let who = who.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); mcall(&mcall(&line.clone(), "replace", &[py_str("{who}"), who.clone()]), "replace", &[py_str("{pair}"), pair.clone()]) }) }]);
    Py::None
}
pub fn _has_phrase(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&clean.clone(), -1, "")); __s.push_str(" "); py_string(__s) }, &{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&phrase.clone(), -1, "")); __s.push_str(" "); py_string(__s) }))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _planned_word(args: &[Py]) -> Py {
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
    let mut verb: Py = Py::None;
    let _ = py_str("True when a word is one of PLANNED_VERBS or an ordinary inflection of one.\n\n    \"thinking of moving house\" has to count (\"moving\" is the plan), while \"thinking about my mum\"\n    and \"i keep thinking about it\" must not: the object of a thought is not a plan. So the word\n    after the opener has to be a verb of the work a plan is made of, in whatever form it is said.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_PLANNED_VERBS(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&g_PLANNED_VERBS());
    for __item in __iter1 {
    verb = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![op_add(&verb.clone(), &py_str("ing")), op_add(&slice_or_raise(&verb.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing")), op_add(&verb.clone(), &py_str("s")), op_add(&verb.clone(), &py_str("d")), op_add(&op_add(&verb.clone(), &geti(&verb.clone(), &op_neg(&Py::Int(1)))), &py_str("ing")), op_add(&slice_or_raise(&verb.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ed"))]), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _going_to_plan(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut rest: Py = Py::None;
    let __iter1 = py_iter_or_raise(&g_GOING_TO());
    for __item in __iter1 {
    opener = __item.clone();
    at = mcall(&clean.clone(), "find", &[opener.clone()]);
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    continue;
    }
    rest = mcall(&slice_or_raise(&clean.clone(), &op_add(&at.clone(), &call_builtin("len", &[opener.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None), "split", &[]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("work"), py_str("working"), py_str("works")]), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(py_eq(&geti(&rest.clone(), &Py::Int(1)), &py_str("on")));
    }
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { _planned_word(&[geti(&rest.clone(), &Py::Int(0))]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("going"), py_str("getting")]), &geti(&rest.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&rest.clone(), &Py::Int(1)), &py_str("back"))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("thinking of"), py_str("thinking about")]), &mcall(&opener.clone(), "strip", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[slice_or_raise(&rest.clone(), &Py::None, &Py::Int(3), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_THOUGHT_OBJECT()))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_plan(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says what the person means to do, over time.");
    if py_truthy(&op_bitand(&call_builtin("set", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_ARRANGEMENT())) {
    return _has_phrase(&[clean.clone(), g_PLAN()]);
    }
    if py_truthy(&{ let __b = _has_phrase(&[clean.clone(), g_PLAN()]); if py_truthy(&__b) { __b } else { _going_to_plan(&[clean.clone()]) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_BOOKED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&call_builtin("set", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_NOUNS()); if !py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_BIGGER_PLAN()) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asked_last(args: &[Py]) -> Py {
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
    let _ = py_str("True when the bot's last reply ended on a question.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&if py_truthy(&turns.clone()) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return mcall(&last.clone(), "endswith", &[py_str("?")]);
    Py::None
}
pub fn a_want_said_back(args: &[Py]) -> Py {
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
    let mut repeat: Py = Py::None;
    let _ = py_str("True when the store's answer to this line only filed the want, so the reader can take it.\n\n    The test is the shape, not a list of frames: a reply that only says the line back has filed\n    the want and answered nothing, whether it opens \"Noted:\", \"Got it -\" or, straight after a\n    feeling, ends \"That explains it - no wonder it sits on you.\" The store's own shaped answers\n    are not that shape - \"So you are thinking about a horse. Who would look after it in the day?\"\n    is a better reply than anything this pool can say, and it says something the line did not.\n    A death in the line keeps the store's turn whatever shape its answer has.\n    ");
    if py_truthy(&{ let __b = mcall(&g_A_LOSS_IN_THE_LINE(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { mcall(&g_A_CONDOLENCE(), "search", &[call_builtin("str", &[{ let __b = said_back.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return Py::Bool(false);
    }
    repeat = pyrt::xrefs::xmodule("memory.repeat");
    return mcall(&repeat.clone(), "only_said_it_back", &[clean.clone(), said_back.clone()]);
    Py::None
}
pub fn dependant_want_reply_for(args: &[Py]) -> Py {
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
    let mut o: Py = Py::None;
    let mut options: Py = Py::None;
    let _ = py_str("A reply to a want somebody the person looks after has, or None to leave it to the caller.\n\n    The line is a request for help with that want, so the reply is the first step of it - never\n    the echo, which files the want and answers nothing. Everything this reader does not own is\n    left to the caller: a want of the person's own (that is `reply_for` above), a want aimed at\n    the person rather than at a thing (\"my son wants me to buy him a car\"), a line about a loss\n    or an emergency, and any line that asks something.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DEPENDANT_WANT(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_PLAIN()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_CRISIS()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_ASKED_OF_THE_PERSON(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WANTS_TO_BE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_WEAPON())) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_WEAPON_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_has_phrase(&[clean.clone(), g_NOT_A_WEAPON()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ASKS()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_DOUBT()]) } }) {
    return Py::None;
    }
    if py_truthy(&_has_phrase(&[clean.clone(), g_COUNTERFACTUAL()])) {
    return Py::None;
    }
    options = g_DEPENDANT_WANT_REPLIES();
    if py_truthy(&_asked_last(&[memory.clone()])) {
    options = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&o.clone(), "endswith", &[py_str("?")])))) {  __out.push(o.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { options.clone() } };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone(), clean.clone()]);
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut topics: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topics") { v.clone() }
    else { py_tuple(vec![]) };
    let mut o: Py = Py::None;
    let mut options: Py = Py::None;
    let _ = py_str("A short reply that fits the plan in a statement, or None to leave it to the caller.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_PLAIN()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_CRISIS()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _has_phrase(&[clean.clone(), g_COUNTERFACTUAL()]); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MARKED_DATE()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ASKS()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_DOUBT()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_plan(&[clean.clone()])))) {
    return Py::None;
    }
    options = if py_truthy(&_has_phrase(&[clean.clone(), g_BARRIER()])) { g_PLAN_REPLIES_PLAIN() } else { g_PLAN_REPLIES() };
    if py_truthy(&_asked_last(&[memory.clone()])) {
    options = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&o.clone(), "endswith", &[py_str("?")])))) {  __out.push(o.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { options.clone() } };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn reads_as_arranged(args: &[Py]) -> Py {
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
    let mut booked: Py = Py::None;
    let _ = py_str("True when the line says the person has booked or is booking something of their own.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_DOUBLE_BOOKED(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_WENT_WRONG(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    booked = mcall(&g_ARRANGED(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = booked.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_TURNED(), "search", &[slice_or_raise(&clean.clone(), &mcall(&booked.clone(), "start", &[]), &Py::None, &Py::None)]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_HAD_IT_BOOKED(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = booked.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUESTION_FIRST(), "match", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THE_PERSON_WITH_IT(), "match", &[slice_or_raise(&clean.clone(), &mcall(&booked.clone(), "start", &[]), &Py::None, &Py::None)]))) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[booked.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_when_it_is(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line already says when the thing it names is.\n\n    The plan reader's own future-date reader (`facts.FUTURE`) is the whole of it, so every way of\n    saying a date the store understands counts here too - \"in june\", \"next friday\", \"on the 3rd\",\n    \"tomorrow\", \"in three weeks\", and a date after the verb that carries it (\"the term ends 12\n    july\").");
    return { let __b = call_builtin("bool", &[{ let __b = clean.clone(); if !py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("memory.facts", "FUTURE"), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[op_bitand(&call_builtin("set", &[mcall(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str(","), py_str(" ")]), "replace", &[py_str("."), py_str(" ")]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &op_bitor(&pyrt::xrefs::xget("memory.facts", "DAYS"), &pyrt::xrefs::xget("memory.facts", "MONTHS")))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn arranged_reply_for(args: &[Py]) -> Py {
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
    let mut o: Py = Py::None;
    let mut options: Py = Py::None;
    let _ = py_str("A reply to a thing the person has arranged, or None to leave it to the caller.");
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_PLAIN()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_CRISIS()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _has_phrase(&[clean.clone(), g_COUNTERFACTUAL()]); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MARKED_DATE()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ASKS()); if py_truthy(&__b) { __b } else { _has_phrase(&[clean.clone(), g_DOUBT()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_arranged(&[clean.clone()])))) {
    return Py::None;
    }
    options = g_BOOKED_REPLIES();
    if py_truthy(&_says_when_it_is(&[clean.clone()])) {
    options = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); __out.push(mcall(&g_ALREADY_DATED(), "get", &[o.clone(), o.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&_asked_last(&[memory.clone()])) {
    options = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&o.clone(), "endswith", &[py_str("?")])))) {  __out.push(o.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { options.clone() } };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut template: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "template") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: template"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The template with the activity in it, capitalised when it opens the sentence.");
    said = mcall(&template.clone(), "replace", &[py_str("{thing}"), thing.clone()]);
    return mcall(&said.clone(), "replace", &[py_str("{Thing}"), op_add(&mcall(&geti(&thing.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    Py::None
}
pub fn interest_reply_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut interest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "interest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: interest"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked_last: Py = Py::None;
    let mut back: Py = Py::None;
    let mut options: Py = Py::None;
    let mut started: Py = Py::None;
    let mut template: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A reply to an activity the person is into, or None to leave it to the caller.\n\n    `interest` is `understood[\"taking_up\"]`: the activity and whether it is new. Nothing else is\n    read here - the understanding engine has already said whose interest it is and fenced the\n    lines where \"into\" means a person, a fight or a debt - so this only decides the wording.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[interest.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    thing = mcall(&call_builtin("str", &[{ let __b = mcall(&interest.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&thing.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_AN_INTEREST(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    started = Py::Bool(py_eq(&mcall(&interest.clone(), "get", &[py_str("shape")]), &py_str("started")));
    back = call_builtin("bool", &[mcall(&interest.clone(), "get", &[py_str("back")])]).unwrap_or_else(|e| exc::raise(e));
    asked_last = _asked_last(&[memory.clone()]);
    if py_truthy(&back.clone()) {
    options = if py_truthy(&asked_last.clone()) { g_INTEREST_BACK_PLAIN() } else { g_INTEREST_BACK() };
    } else {
    options = if py_truthy(&started.clone()) { g_INTEREST_STARTED_PLAIN() } else { g_INTEREST_DOING_PLAIN() };
    if py_truthy(&Py::Bool(!py_truthy(&asked_last.clone()))) {
    options = if py_truthy(&started.clone()) { g_INTEREST_STARTED() } else { g_INTEREST_DOING() };
    }
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut template: Py = __item.clone(); __out.push(_named(&[template.clone(), thing.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), thing.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.remark".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGAIN" => Some(g_AGAIN()),
            "AGAIN_REPLIES" => Some(g_AGAIN_REPLIES()),
            "AGE_ALREADY_HAD" => Some(g_AGE_ALREADY_HAD()),
            "AGE_FIRST_TOLD" => Some(g_AGE_FIRST_TOLD()),
            "AGE_PUT_RIGHT" => Some(g_AGE_PUT_RIGHT()),
            "AGE_WORDS" => Some(g_AGE_WORDS()),
            "ALREADY_DATED" => Some(g_ALREADY_DATED()),
            "ARRANGED" => Some(g_ARRANGED()),
            "ARRANGEMENT" => Some(g_ARRANGEMENT()),
            "ASKED_OF_THE_PERSON" => Some(g_ASKED_OF_THE_PERSON()),
            "ASKS" => Some(g_ASKS()),
            "A_CHILD_NAMED" => Some(g_A_CHILD_NAMED()),
            "A_CONDOLENCE" => Some(g_A_CONDOLENCE()),
            "A_GROUP_OF_PEOPLE" => Some(g_A_GROUP_OF_PEOPLE()),
            "A_HELD_NAME_AND_AN_AGE" => Some(g_A_HELD_NAME_AND_AN_AGE()),
            "A_LOSS_IN_THE_LINE" => Some(g_A_LOSS_IN_THE_LINE()),
            "A_MILESTONE_TAIL" => Some(g_A_MILESTONE_TAIL()),
            "A_NIGHT_OUT" => Some(g_A_NIGHT_OUT()),
            "A_PERSON_LOST" => Some(g_A_PERSON_LOST()),
            "A_PERSON_OF_THEIRS" => Some(g_A_PERSON_OF_THEIRS()),
            "A_THING_CARRIED_OFF" => Some(g_A_THING_CARRIED_OFF()),
            "A_THING_TAKEN" => Some(g_A_THING_TAKEN()),
            "A_WEAPON" => Some(g_A_WEAPON()),
            "BABY_WORDS" => Some(g_BABY_WORDS()),
            "BAD_WITH_THEM" => Some(g_BAD_WITH_THEM()),
            "BARE_SETBACK" => Some(g_BARE_SETBACK()),
            "BARRIER" => Some(g_BARRIER()),
            "BIGGER_PLAN" => Some(g_BIGGER_PLAN()),
            "BOOKED" => Some(g_BOOKED()),
            "BOOKED_REPLIES" => Some(g_BOOKED_REPLIES()),
            "BURGLED" => Some(g_BURGLED()),
            "BURGLED_FOR_SOMEONE_REPLIES" => Some(g_BURGLED_FOR_SOMEONE_REPLIES()),
            "BURGLED_HOME" => Some(g_BURGLED_HOME()),
            "BURGLED_NOT_THEIRS" => Some(g_BURGLED_NOT_THEIRS()),
            "BURGLED_REPLIES" => Some(g_BURGLED_REPLIES()),
            "BURGLED_SOMEONE_ELSE" => Some(g_BURGLED_SOMEONE_ELSE()),
            "COUNTERFACTUAL" => Some(g_COUNTERFACTUAL()),
            "CRACKED_A_JOKE" => Some(g_CRACKED_A_JOKE()),
            "CRISIS" => Some(g_CRISIS()),
            "DEADLINE_LATER" => Some(g_DEADLINE_LATER()),
            "DEPENDANT" => Some(g_DEPENDANT()),
            "DEPENDANT_WANT" => Some(g_DEPENDANT_WANT()),
            "DEPENDANT_WANT_REPLIES" => Some(g_DEPENDANT_WANT_REPLIES()),
            "DONE_TO_ME" => Some(g_DONE_TO_ME()),
            "DOUBLE_BOOKED" => Some(g_DOUBLE_BOOKED()),
            "DOUBT" => Some(g_DOUBT()),
            "FIRST_PERSON" => Some(g_FIRST_PERSON()),
            "FIRST_STRETCH" => Some(g_FIRST_STRETCH()),
            "FUTURE_REGRET" => Some(g_FUTURE_REGRET()),
            "GOING_ON" => Some(g_GOING_ON()),
            "GOING_ON_LINES" => Some(g_GOING_ON_LINES()),
            "GOING_TO" => Some(g_GOING_TO()),
            "HAD_IT_BOOKED" => Some(g_HAD_IT_BOOKED()),
            "HE_PEOPLE" => Some(g_HE_PEOPLE()),
            "HE_PERSON" => Some(g_HE_PERSON()),
            "INTEREST_BACK" => Some(g_INTEREST_BACK()),
            "INTEREST_BACK_PLAIN" => Some(g_INTEREST_BACK_PLAIN()),
            "INTEREST_DOING" => Some(g_INTEREST_DOING()),
            "INTEREST_DOING_PLAIN" => Some(g_INTEREST_DOING_PLAIN()),
            "INTEREST_STARTED" => Some(g_INTEREST_STARTED()),
            "INTEREST_STARTED_PLAIN" => Some(g_INTEREST_STARTED_PLAIN()),
            "LOSS_FOLLOW" => Some(g_LOSS_FOLLOW()),
            "LOSS_FOLLOW_REPLIES" => Some(g_LOSS_FOLLOW_REPLIES()),
            "MARKED_DATE" => Some(g_MARKED_DATE()),
            "MARRIAGE_IN_TROUBLE" => Some(g_MARRIAGE_IN_TROUBLE()),
            "MISSED" => Some(g_MISSED()),
            "NEVER_CAME_SUBJECT" => Some(g_NEVER_CAME_SUBJECT()),
            "NOT_AN_INTEREST" => Some(g_NOT_AN_INTEREST()),
            "NOT_A_BREAK_IN" => Some(g_NOT_A_BREAK_IN()),
            "NOT_A_LOSS" => Some(g_NOT_A_LOSS()),
            "NOT_A_NAME_AFTER_CALLED" => Some(g_NOT_A_NAME_AFTER_CALLED()),
            "NOT_A_RELATION_TROUBLE" => Some(g_NOT_A_RELATION_TROUBLE()),
            "NOT_A_TASTE_AS_A_THING" => Some(g_NOT_A_TASTE_AS_A_THING()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_A_WEAPON" => Some(g_NOT_A_WEAPON()),
            "NOT_PLAIN" => Some(g_NOT_PLAIN()),
            "NO_REGRET" => Some(g_NO_REGRET()),
            "OTHER_PEOPLE" => Some(g_OTHER_PEOPLE()),
            "OWN_ARTICLE" => Some(g_OWN_ARTICLE()),
            "OWN_ASKS" => Some(g_OWN_ASKS()),
            "OWN_ELECTRIC" => Some(g_OWN_ELECTRIC()),
            "OWN_ELECTRIC_CAR" => Some(g_OWN_ELECTRIC_CAR()),
            "OWN_FACTS" => Some(g_OWN_FACTS()),
            "OWN_FAULT" => Some(g_OWN_FAULT()),
            "OWN_HAVE" => Some(g_OWN_HAVE()),
            "OWN_KIT" => Some(g_OWN_KIT()),
            "OWN_KIT_ASKS" => Some(g_OWN_KIT_ASKS()),
            "OWN_KIT_NAMES" => Some(g_OWN_KIT_NAMES()),
            "OWN_KIT_PLAIN" => Some(g_OWN_KIT_PLAIN()),
            "OWN_NOT_THE_THING" => Some(g_OWN_NOT_THE_THING()),
            "OWN_SLIP" => Some(g_OWN_SLIP()),
            "OWN_SPACES" => Some(g_OWN_SPACES()),
            "OWN_SPACES_PLAIN" => Some(g_OWN_SPACES_PLAIN()),
            "OWN_TAIL" => Some(g_OWN_TAIL()),
            "OWN_TIME" => Some(g_OWN_TIME()),
            "OWN_TWO_THINGS" => Some(g_OWN_TWO_THINGS()),
            "PLACE_CODES" => Some(g_PLACE_CODES()),
            "PLAN" => Some(g_PLAN()),
            "PLANNED_VERBS" => Some(g_PLANNED_VERBS()),
            "PLAN_REPLIES" => Some(g_PLAN_REPLIES()),
            "PLAN_REPLIES_PLAIN" => Some(g_PLAN_REPLIES_PLAIN()),
            "POLITE" => Some(g_POLITE()),
            "PRACTICAL_ASK" => Some(g_PRACTICAL_ASK()),
            "PRESENT_SETBACK" => Some(g_PRESENT_SETBACK()),
            "PRICE" => Some(g_PRICE()),
            "PUT_DOWN" => Some(g_PUT_DOWN()),
            "QUESTION_FIRST" => Some(g_QUESTION_FIRST()),
            "QUIET_BETWEEN_THEM" => Some(g_QUIET_BETWEEN_THEM()),
            "REGRET" => Some(g_REGRET()),
            "REGRET_LINES" => Some(g_REGRET_LINES()),
            "REGRET_LINES_PLAIN" => Some(g_REGRET_LINES_PLAIN()),
            "RELATION_GONE_WRONG" => Some(g_RELATION_GONE_WRONG()),
            "REPORT_BODY" => Some(g_REPORT_BODY()),
            "REPORT_FEELING" => Some(g_REPORT_FEELING()),
            "RISE" => Some(g_RISE()),
            "ROWS_BETWEEN_THEM" => Some(g_ROWS_BETWEEN_THEM()),
            "SETBACK" => Some(g_SETBACK()),
            "SETBACK_REPLIES" => Some(g_SETBACK_REPLIES()),
            "SHE_PEOPLE" => Some(g_SHE_PEOPLE()),
            "SHE_PERSON" => Some(g_SHE_PERSON()),
            "SHOULDNT_HAVE" => Some(g_SHOULDNT_HAVE()),
            "SLIP_REPLIES" => Some(g_SLIP_REPLIES()),
            "SMASHED" => Some(g_SMASHED()),
            "SMASHED_OF_A_THING" => Some(g_SMASHED_OF_A_THING()),
            "STARTED" => Some(g_STARTED()),
            "STARTED_AHEAD_REPLIES" => Some(g_STARTED_AHEAD_REPLIES()),
            "STARTED_BADLY_REPLIES" => Some(g_STARTED_BADLY_REPLIES()),
            "STARTED_FINE_REPLIES" => Some(g_STARTED_FINE_REPLIES()),
            "STARTED_GONE_BADLY" => Some(g_STARTED_GONE_BADLY()),
            "STARTED_REPLIES" => Some(g_STARTED_REPLIES()),
            "STARTED_VERDICT" => Some(g_STARTED_VERDICT()),
            "STARTED_WELL" => Some(g_STARTED_WELL()),
            "START_AHEAD" => Some(g_START_AHEAD()),
            "START_CLOCK" => Some(g_START_CLOCK()),
            "START_PAST" => Some(g_START_PAST()),
            "START_PRESENT" => Some(g_START_PRESENT()),
            "STEP_NOUNS" => Some(g_STEP_NOUNS()),
            "TAKEN_FROM_THEM" => Some(g_TAKEN_FROM_THEM()),
            "TASTE_TOO_LONG" => Some(g_TASTE_TOO_LONG()),
            "THEIR_AGE" => Some(g_THEIR_AGE()),
            "THEIR_BABY_LINES" => Some(g_THEIR_BABY_LINES()),
            "THEIR_CALLED" => Some(g_THEIR_CALLED()),
            "THEIR_CHILD" => Some(g_THEIR_CHILD()),
            "THEIR_CHILD_LINES" => Some(g_THEIR_CHILD_LINES()),
            "THEIR_DISLIKE" => Some(g_THEIR_DISLIKE()),
            "THEIR_DISLIKE_LINES" => Some(g_THEIR_DISLIKE_LINES()),
            "THEIR_ELDER" => Some(g_THEIR_ELDER()),
            "THEIR_ELDER_AGE" => Some(g_THEIR_ELDER_AGE()),
            "THEIR_ELDER_LINES" => Some(g_THEIR_ELDER_LINES()),
            "THEIR_GROWN_LINES" => Some(g_THEIR_GROWN_LINES()),
            "THEIR_HATE" => Some(g_THEIR_HATE()),
            "THEIR_HATE_LINES" => Some(g_THEIR_HATE_LINES()),
            "THEIR_JOB" => Some(g_THEIR_JOB()),
            "THEIR_JOB_LINES" => Some(g_THEIR_JOB_LINES()),
            "THEIR_LIKING" => Some(g_THEIR_LIKING()),
            "THEIR_LIKING_LINES" => Some(g_THEIR_LIKING_LINES()),
            "THEIR_MOOD_WITH_YOU" => Some(g_THEIR_MOOD_WITH_YOU()),
            "THEIR_MOOD_WITH_YOU_LINES" => Some(g_THEIR_MOOD_WITH_YOU_LINES()),
            "THEIR_NAME" => Some(g_THEIR_NAME()),
            "THEIR_NAMES_LINES" => Some(g_THEIR_NAMES_LINES()),
            "THEIR_NAME_AGE" => Some(g_THEIR_NAME_AGE()),
            "THEIR_NAME_AGE_LINES" => Some(g_THEIR_NAME_AGE_LINES()),
            "THEIR_NAME_GROWN_LINES" => Some(g_THEIR_NAME_GROWN_LINES()),
            "THEIR_NAME_LINES" => Some(g_THEIR_NAME_LINES()),
            "THEIR_NAME_SO" => Some(g_THEIR_NAME_SO()),
            "THEIR_PERSON" => Some(g_THEIR_PERSON()),
            "THEIR_PLACE" => Some(g_THEIR_PLACE()),
            "THEIR_PLACE_LINES" => Some(g_THEIR_PLACE_LINES()),
            "THEIR_QUALIFIER" => Some(g_THEIR_QUALIFIER()),
            "THEIR_SMALL_LINES" => Some(g_THEIR_SMALL_LINES()),
            "THEIR_TEEN_LINES" => Some(g_THEIR_TEEN_LINES()),
            "THEY_PERSON" => Some(g_THEY_PERSON()),
            "THE_PERSON_WITH_IT" => Some(g_THE_PERSON_WITH_IT()),
            "THE_QUIET_LINES" => Some(g_THE_QUIET_LINES()),
            "THE_RELATION_LINES" => Some(g_THE_RELATION_LINES()),
            "THE_ROWS_LINES" => Some(g_THE_ROWS_LINES()),
            "THE_THING_IS_A_CLAUSE" => Some(g_THE_THING_IS_A_CLAUSE()),
            "THOUGHT_OBJECT" => Some(g_THOUGHT_OBJECT()),
            "TOOK_IT_IN" => Some(g_TOOK_IT_IN()),
            "TURNED" => Some(g_TURNED()),
            "WANTS_TO_BE" => Some(g_WANTS_TO_BE()),
            "WEAPON_WORDS" => Some(g_WEAPON_WORDS()),
            "WENT_WRONG" => Some(g_WENT_WRONG()),
            "WISH_I" => Some(g_WISH_I()),
            "YEAR_UNIT" => Some(g_YEAR_UNIT()),
            "_AHEAD_DAY" => Some(g__AHEAD_DAY()),
            "_HARM_AT_PERSON" => Some(g__HARM_AT_PERSON()),
            "_OWN_SHOULD" => Some(g__OWN_SHOULD()),
            "_RELATION_PERSON" => Some(g__RELATION_PERSON()),
            "_a_good_night_out" => Some(py_func("_a_good_night_out", |args: &[Py]| _a_good_night_out(args))),
            "_a_taste_they_refuse" => Some(py_func("_a_taste_they_refuse", |args: &[Py]| _a_taste_they_refuse(args))),
            "_a_thing_taken" => Some(py_func("_a_thing_taken", |args: &[Py]| _a_thing_taken(args))),
            "_age_pool" => Some(py_func("_age_pool", |args: &[Py]| _age_pool(args))),
            "_an_age_put_right" => Some(py_func("_an_age_put_right", |args: &[Py]| _an_age_put_right(args))),
            "_article" => Some(py_func("_article", |args: &[Py]| _article(args))),
            "_asked_last" => Some(py_func("_asked_last", |args: &[Py]| _asked_last(args))),
            "_capped" => Some(py_func("_capped", |args: &[Py]| _capped(args))),
            "_going_to_plan" => Some(py_func("_going_to_plan", |args: &[Py]| _going_to_plan(args))),
            "_has_phrase" => Some(py_func("_has_phrase", |args: &[Py]| _has_phrase(args))),
            "_missed_a_person" => Some(py_func("_missed_a_person", |args: &[Py]| _missed_a_person(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_pick_filled" => Some(py_func("_pick_filled", |args: &[Py]| _pick_filled(args))),
            "_planned_word" => Some(py_func("_planned_word", |args: &[Py]| _planned_word(args))),
            "_pronoun_person" => Some(py_func("_pronoun_person", |args: &[Py]| _pronoun_person(args))),
            "_reads_as_a_break_in" => Some(py_func("_reads_as_a_break_in", |args: &[Py]| _reads_as_a_break_in(args))),
            "_says_when_it_is" => Some(py_func("_says_when_it_is", |args: &[Py]| _says_when_it_is(args))),
            "_started_badly" => Some(py_func("_started_badly", |args: &[Py]| _started_badly(args))),
            "_still_to_come" => Some(py_func("_still_to_come", |args: &[Py]| _still_to_come(args))),
            "_the_age_i_had" => Some(py_func("_the_age_i_had", |args: &[Py]| _the_age_i_had(args))),
            "_the_answers_take_it" => Some(py_func("_the_answers_take_it", |args: &[Py]| _the_answers_take_it(args))),
            "_the_class_word" => Some(py_func("_the_class_word", |args: &[Py]| _the_class_word(args))),
            "_the_fact_about" => Some(py_func("_the_fact_about", |args: &[Py]| _the_fact_about(args))),
            "_the_job_in" => Some(py_func("_the_job_in", |args: &[Py]| _the_job_in(args))),
            "_the_loss_was_asked" => Some(py_func("_the_loss_was_asked", |args: &[Py]| _the_loss_was_asked(args))),
            "_the_people_in" => Some(py_func("_the_people_in", |args: &[Py]| _the_people_in(args))),
            "_the_thing_they_have" => Some(py_func("_the_thing_they_have", |args: &[Py]| _the_thing_they_have(args))),
            "a_thing_of_theirs_reply_for" => Some(py_func("a_thing_of_theirs_reply_for", |args: &[Py]| a_thing_of_theirs_reply_for(args))),
            "a_want_said_back" => Some(py_func("a_want_said_back", |args: &[Py]| a_want_said_back(args))),
            "arranged_reply_for" => Some(py_func("arranged_reply_for", |args: &[Py]| arranged_reply_for(args))),
            "break_in_reply_for" => Some(py_func("break_in_reply_for", |args: &[Py]| break_in_reply_for(args))),
            "dependant_want_reply_for" => Some(py_func("dependant_want_reply_for", |args: &[Py]| dependant_want_reply_for(args))),
            "done_to_me" => Some(py_func("done_to_me", |args: &[Py]| done_to_me(args))),
            "fill_age" => Some(py_func("fill_age", |args: &[Py]| fill_age(args))),
            "fill_person" => Some(py_func("fill_person", |args: &[Py]| fill_person(args))),
            "interest_reply_for" => Some(py_func("interest_reply_for", |args: &[Py]| interest_reply_for(args))),
            "name_a_place" => Some(py_func("name_a_place", |args: &[Py]| name_a_place(args))),
            "person_pronouns" => Some(py_func("person_pronouns", |args: &[Py]| person_pronouns(args))),
            "pick" => Some(g_pick()),
            "reads_as_arranged" => Some(py_func("reads_as_arranged", |args: &[Py]| reads_as_arranged(args))),
            "reads_as_plan" => Some(py_func("reads_as_plan", |args: &[Py]| reads_as_plan(args))),
            "reads_as_regret" => Some(py_func("reads_as_regret", |args: &[Py]| reads_as_regret(args))),
            "reads_as_report" => Some(py_func("reads_as_report", |args: &[Py]| reads_as_report(args))),
            "regret_reply_for" => Some(py_func("regret_reply_for", |args: &[Py]| regret_reply_for(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "report_reply_for" => Some(py_func("report_reply_for", |args: &[Py]| report_reply_for(args))),
            "the_two_of_them" => Some(py_func("the_two_of_them", |args: &[Py]| the_two_of_them(args))),
            "their_mood_going_on_reply_for" => Some(py_func("their_mood_going_on_reply_for", |args: &[Py]| their_mood_going_on_reply_for(args))),
            "their_mood_with_you_reply_for" => Some(py_func("their_mood_with_you_reply_for", |args: &[Py]| their_mood_with_you_reply_for(args))),
            "their_people_reply_for" => Some(py_func("their_people_reply_for", |args: &[Py]| their_people_reply_for(args))),
            "without_leaders" => Some(g_without_leaders()),
            "trouble" => Some(pyrt::xrefs::xmodule("compose.trouble")),
            "facts" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGAIN" => s_AGAIN(v),
            "AGAIN_REPLIES" => s_AGAIN_REPLIES(v),
            "AGE_ALREADY_HAD" => s_AGE_ALREADY_HAD(v),
            "AGE_FIRST_TOLD" => s_AGE_FIRST_TOLD(v),
            "AGE_PUT_RIGHT" => s_AGE_PUT_RIGHT(v),
            "AGE_WORDS" => s_AGE_WORDS(v),
            "ALREADY_DATED" => s_ALREADY_DATED(v),
            "ARRANGED" => s_ARRANGED(v),
            "ARRANGEMENT" => s_ARRANGEMENT(v),
            "ASKED_OF_THE_PERSON" => s_ASKED_OF_THE_PERSON(v),
            "ASKS" => s_ASKS(v),
            "A_CHILD_NAMED" => s_A_CHILD_NAMED(v),
            "A_CONDOLENCE" => s_A_CONDOLENCE(v),
            "A_GROUP_OF_PEOPLE" => s_A_GROUP_OF_PEOPLE(v),
            "A_HELD_NAME_AND_AN_AGE" => s_A_HELD_NAME_AND_AN_AGE(v),
            "A_LOSS_IN_THE_LINE" => s_A_LOSS_IN_THE_LINE(v),
            "A_MILESTONE_TAIL" => s_A_MILESTONE_TAIL(v),
            "A_NIGHT_OUT" => s_A_NIGHT_OUT(v),
            "A_PERSON_LOST" => s_A_PERSON_LOST(v),
            "A_PERSON_OF_THEIRS" => s_A_PERSON_OF_THEIRS(v),
            "A_THING_CARRIED_OFF" => s_A_THING_CARRIED_OFF(v),
            "A_THING_TAKEN" => s_A_THING_TAKEN(v),
            "A_WEAPON" => s_A_WEAPON(v),
            "BABY_WORDS" => s_BABY_WORDS(v),
            "BAD_WITH_THEM" => s_BAD_WITH_THEM(v),
            "BARE_SETBACK" => s_BARE_SETBACK(v),
            "BARRIER" => s_BARRIER(v),
            "BIGGER_PLAN" => s_BIGGER_PLAN(v),
            "BOOKED" => s_BOOKED(v),
            "BOOKED_REPLIES" => s_BOOKED_REPLIES(v),
            "BURGLED" => s_BURGLED(v),
            "BURGLED_FOR_SOMEONE_REPLIES" => s_BURGLED_FOR_SOMEONE_REPLIES(v),
            "BURGLED_HOME" => s_BURGLED_HOME(v),
            "BURGLED_NOT_THEIRS" => s_BURGLED_NOT_THEIRS(v),
            "BURGLED_REPLIES" => s_BURGLED_REPLIES(v),
            "BURGLED_SOMEONE_ELSE" => s_BURGLED_SOMEONE_ELSE(v),
            "COUNTERFACTUAL" => s_COUNTERFACTUAL(v),
            "CRACKED_A_JOKE" => s_CRACKED_A_JOKE(v),
            "CRISIS" => s_CRISIS(v),
            "DEADLINE_LATER" => s_DEADLINE_LATER(v),
            "DEPENDANT" => s_DEPENDANT(v),
            "DEPENDANT_WANT" => s_DEPENDANT_WANT(v),
            "DEPENDANT_WANT_REPLIES" => s_DEPENDANT_WANT_REPLIES(v),
            "DONE_TO_ME" => s_DONE_TO_ME(v),
            "DOUBLE_BOOKED" => s_DOUBLE_BOOKED(v),
            "DOUBT" => s_DOUBT(v),
            "FIRST_PERSON" => s_FIRST_PERSON(v),
            "FIRST_STRETCH" => s_FIRST_STRETCH(v),
            "FUTURE_REGRET" => s_FUTURE_REGRET(v),
            "GOING_ON" => s_GOING_ON(v),
            "GOING_ON_LINES" => s_GOING_ON_LINES(v),
            "GOING_TO" => s_GOING_TO(v),
            "HAD_IT_BOOKED" => s_HAD_IT_BOOKED(v),
            "HE_PEOPLE" => s_HE_PEOPLE(v),
            "HE_PERSON" => s_HE_PERSON(v),
            "INTEREST_BACK" => s_INTEREST_BACK(v),
            "INTEREST_BACK_PLAIN" => s_INTEREST_BACK_PLAIN(v),
            "INTEREST_DOING" => s_INTEREST_DOING(v),
            "INTEREST_DOING_PLAIN" => s_INTEREST_DOING_PLAIN(v),
            "INTEREST_STARTED" => s_INTEREST_STARTED(v),
            "INTEREST_STARTED_PLAIN" => s_INTEREST_STARTED_PLAIN(v),
            "LOSS_FOLLOW" => s_LOSS_FOLLOW(v),
            "LOSS_FOLLOW_REPLIES" => s_LOSS_FOLLOW_REPLIES(v),
            "MARKED_DATE" => s_MARKED_DATE(v),
            "MARRIAGE_IN_TROUBLE" => s_MARRIAGE_IN_TROUBLE(v),
            "MISSED" => s_MISSED(v),
            "NEVER_CAME_SUBJECT" => s_NEVER_CAME_SUBJECT(v),
            "NOT_AN_INTEREST" => s_NOT_AN_INTEREST(v),
            "NOT_A_BREAK_IN" => s_NOT_A_BREAK_IN(v),
            "NOT_A_LOSS" => s_NOT_A_LOSS(v),
            "NOT_A_NAME_AFTER_CALLED" => s_NOT_A_NAME_AFTER_CALLED(v),
            "NOT_A_RELATION_TROUBLE" => s_NOT_A_RELATION_TROUBLE(v),
            "NOT_A_TASTE_AS_A_THING" => s_NOT_A_TASTE_AS_A_THING(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_A_WEAPON" => s_NOT_A_WEAPON(v),
            "NOT_PLAIN" => s_NOT_PLAIN(v),
            "NO_REGRET" => s_NO_REGRET(v),
            "OTHER_PEOPLE" => s_OTHER_PEOPLE(v),
            "OWN_ARTICLE" => s_OWN_ARTICLE(v),
            "OWN_ASKS" => s_OWN_ASKS(v),
            "OWN_ELECTRIC" => s_OWN_ELECTRIC(v),
            "OWN_ELECTRIC_CAR" => s_OWN_ELECTRIC_CAR(v),
            "OWN_FACTS" => s_OWN_FACTS(v),
            "OWN_FAULT" => s_OWN_FAULT(v),
            "OWN_HAVE" => s_OWN_HAVE(v),
            "OWN_KIT" => s_OWN_KIT(v),
            "OWN_KIT_ASKS" => s_OWN_KIT_ASKS(v),
            "OWN_KIT_NAMES" => s_OWN_KIT_NAMES(v),
            "OWN_KIT_PLAIN" => s_OWN_KIT_PLAIN(v),
            "OWN_NOT_THE_THING" => s_OWN_NOT_THE_THING(v),
            "OWN_SLIP" => s_OWN_SLIP(v),
            "OWN_SPACES" => s_OWN_SPACES(v),
            "OWN_SPACES_PLAIN" => s_OWN_SPACES_PLAIN(v),
            "OWN_TAIL" => s_OWN_TAIL(v),
            "OWN_TIME" => s_OWN_TIME(v),
            "OWN_TWO_THINGS" => s_OWN_TWO_THINGS(v),
            "PLACE_CODES" => s_PLACE_CODES(v),
            "PLAN" => s_PLAN(v),
            "PLANNED_VERBS" => s_PLANNED_VERBS(v),
            "PLAN_REPLIES" => s_PLAN_REPLIES(v),
            "PLAN_REPLIES_PLAIN" => s_PLAN_REPLIES_PLAIN(v),
            "POLITE" => s_POLITE(v),
            "PRACTICAL_ASK" => s_PRACTICAL_ASK(v),
            "PRESENT_SETBACK" => s_PRESENT_SETBACK(v),
            "PRICE" => s_PRICE(v),
            "PUT_DOWN" => s_PUT_DOWN(v),
            "QUESTION_FIRST" => s_QUESTION_FIRST(v),
            "QUIET_BETWEEN_THEM" => s_QUIET_BETWEEN_THEM(v),
            "REGRET" => s_REGRET(v),
            "REGRET_LINES" => s_REGRET_LINES(v),
            "REGRET_LINES_PLAIN" => s_REGRET_LINES_PLAIN(v),
            "RELATION_GONE_WRONG" => s_RELATION_GONE_WRONG(v),
            "REPORT_BODY" => s_REPORT_BODY(v),
            "REPORT_FEELING" => s_REPORT_FEELING(v),
            "RISE" => s_RISE(v),
            "ROWS_BETWEEN_THEM" => s_ROWS_BETWEEN_THEM(v),
            "SETBACK" => s_SETBACK(v),
            "SETBACK_REPLIES" => s_SETBACK_REPLIES(v),
            "SHE_PEOPLE" => s_SHE_PEOPLE(v),
            "SHE_PERSON" => s_SHE_PERSON(v),
            "SHOULDNT_HAVE" => s_SHOULDNT_HAVE(v),
            "SLIP_REPLIES" => s_SLIP_REPLIES(v),
            "SMASHED" => s_SMASHED(v),
            "SMASHED_OF_A_THING" => s_SMASHED_OF_A_THING(v),
            "STARTED" => s_STARTED(v),
            "STARTED_AHEAD_REPLIES" => s_STARTED_AHEAD_REPLIES(v),
            "STARTED_BADLY_REPLIES" => s_STARTED_BADLY_REPLIES(v),
            "STARTED_FINE_REPLIES" => s_STARTED_FINE_REPLIES(v),
            "STARTED_GONE_BADLY" => s_STARTED_GONE_BADLY(v),
            "STARTED_REPLIES" => s_STARTED_REPLIES(v),
            "STARTED_VERDICT" => s_STARTED_VERDICT(v),
            "STARTED_WELL" => s_STARTED_WELL(v),
            "START_AHEAD" => s_START_AHEAD(v),
            "START_CLOCK" => s_START_CLOCK(v),
            "START_PAST" => s_START_PAST(v),
            "START_PRESENT" => s_START_PRESENT(v),
            "STEP_NOUNS" => s_STEP_NOUNS(v),
            "TAKEN_FROM_THEM" => s_TAKEN_FROM_THEM(v),
            "TASTE_TOO_LONG" => s_TASTE_TOO_LONG(v),
            "THEIR_AGE" => s_THEIR_AGE(v),
            "THEIR_BABY_LINES" => s_THEIR_BABY_LINES(v),
            "THEIR_CALLED" => s_THEIR_CALLED(v),
            "THEIR_CHILD" => s_THEIR_CHILD(v),
            "THEIR_CHILD_LINES" => s_THEIR_CHILD_LINES(v),
            "THEIR_DISLIKE" => s_THEIR_DISLIKE(v),
            "THEIR_DISLIKE_LINES" => s_THEIR_DISLIKE_LINES(v),
            "THEIR_ELDER" => s_THEIR_ELDER(v),
            "THEIR_ELDER_AGE" => s_THEIR_ELDER_AGE(v),
            "THEIR_ELDER_LINES" => s_THEIR_ELDER_LINES(v),
            "THEIR_GROWN_LINES" => s_THEIR_GROWN_LINES(v),
            "THEIR_HATE" => s_THEIR_HATE(v),
            "THEIR_HATE_LINES" => s_THEIR_HATE_LINES(v),
            "THEIR_JOB" => s_THEIR_JOB(v),
            "THEIR_JOB_LINES" => s_THEIR_JOB_LINES(v),
            "THEIR_LIKING" => s_THEIR_LIKING(v),
            "THEIR_LIKING_LINES" => s_THEIR_LIKING_LINES(v),
            "THEIR_MOOD_WITH_YOU" => s_THEIR_MOOD_WITH_YOU(v),
            "THEIR_MOOD_WITH_YOU_LINES" => s_THEIR_MOOD_WITH_YOU_LINES(v),
            "THEIR_NAME" => s_THEIR_NAME(v),
            "THEIR_NAMES_LINES" => s_THEIR_NAMES_LINES(v),
            "THEIR_NAME_AGE" => s_THEIR_NAME_AGE(v),
            "THEIR_NAME_AGE_LINES" => s_THEIR_NAME_AGE_LINES(v),
            "THEIR_NAME_GROWN_LINES" => s_THEIR_NAME_GROWN_LINES(v),
            "THEIR_NAME_LINES" => s_THEIR_NAME_LINES(v),
            "THEIR_NAME_SO" => s_THEIR_NAME_SO(v),
            "THEIR_PERSON" => s_THEIR_PERSON(v),
            "THEIR_PLACE" => s_THEIR_PLACE(v),
            "THEIR_PLACE_LINES" => s_THEIR_PLACE_LINES(v),
            "THEIR_QUALIFIER" => s_THEIR_QUALIFIER(v),
            "THEIR_SMALL_LINES" => s_THEIR_SMALL_LINES(v),
            "THEIR_TEEN_LINES" => s_THEIR_TEEN_LINES(v),
            "THEY_PERSON" => s_THEY_PERSON(v),
            "THE_PERSON_WITH_IT" => s_THE_PERSON_WITH_IT(v),
            "THE_QUIET_LINES" => s_THE_QUIET_LINES(v),
            "THE_RELATION_LINES" => s_THE_RELATION_LINES(v),
            "THE_ROWS_LINES" => s_THE_ROWS_LINES(v),
            "THE_THING_IS_A_CLAUSE" => s_THE_THING_IS_A_CLAUSE(v),
            "THOUGHT_OBJECT" => s_THOUGHT_OBJECT(v),
            "TOOK_IT_IN" => s_TOOK_IT_IN(v),
            "TURNED" => s_TURNED(v),
            "WANTS_TO_BE" => s_WANTS_TO_BE(v),
            "WEAPON_WORDS" => s_WEAPON_WORDS(v),
            "WENT_WRONG" => s_WENT_WRONG(v),
            "WISH_I" => s_WISH_I(v),
            "YEAR_UNIT" => s_YEAR_UNIT(v),
            "_AHEAD_DAY" => s__AHEAD_DAY(v),
            "_HARM_AT_PERSON" => s__HARM_AT_PERSON(v),
            "_OWN_SHOULD" => s__OWN_SHOULD(v),
            "_RELATION_PERSON" => s__RELATION_PERSON(v),
            "pick" => s_pick(v),
            "without_leaders" => s_without_leaders(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.remark", module_obj);
    pyrt::xrefs::reg_fn("compose.remark", "_a_good_night_out", py_func("_a_good_night_out", |args: &[Py]| _a_good_night_out(args)));
    pyrt::xrefs::reg_fn("compose.remark", "done_to_me", py_func("done_to_me", |args: &[Py]| done_to_me(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_a_thing_taken", py_func("_a_thing_taken", |args: &[Py]| _a_thing_taken(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_loss_was_asked", py_func("_the_loss_was_asked", |args: &[Py]| _the_loss_was_asked(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_answers_take_it", py_func("_the_answers_take_it", |args: &[Py]| _the_answers_take_it(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_missed_a_person", py_func("_missed_a_person", |args: &[Py]| _missed_a_person(args)));
    pyrt::xrefs::reg_fn("compose.remark", "reads_as_report", py_func("reads_as_report", |args: &[Py]| reads_as_report(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_still_to_come", py_func("_still_to_come", |args: &[Py]| _still_to_come(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_started_badly", py_func("_started_badly", |args: &[Py]| _started_badly(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_reads_as_a_break_in", py_func("_reads_as_a_break_in", |args: &[Py]| _reads_as_a_break_in(args)));
    pyrt::xrefs::reg_fn("compose.remark", "break_in_reply_for", py_func("break_in_reply_for", |args: &[Py]| break_in_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "report_reply_for", py_func("report_reply_for", |args: &[Py]| report_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "reads_as_regret", py_func("reads_as_regret", |args: &[Py]| reads_as_regret(args)));
    pyrt::xrefs::reg_fn("compose.remark", "regret_reply_for", py_func("regret_reply_for", |args: &[Py]| regret_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_a_taste_they_refuse", py_func("_a_taste_they_refuse", |args: &[Py]| _a_taste_they_refuse(args)));
    pyrt::xrefs::reg_fn("compose.remark", "person_pronouns", py_func("person_pronouns", |args: &[Py]| person_pronouns(args)));
    pyrt::xrefs::reg_fn("compose.remark", "fill_person", py_func("fill_person", |args: &[Py]| fill_person(args)));
    pyrt::xrefs::reg_fn("compose.remark", "their_mood_going_on_reply_for", py_func("their_mood_going_on_reply_for", |args: &[Py]| their_mood_going_on_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_pronoun_person", py_func("_pronoun_person", |args: &[Py]| _pronoun_person(args)));
    pyrt::xrefs::reg_fn("compose.remark", "their_mood_with_you_reply_for", py_func("their_mood_with_you_reply_for", |args: &[Py]| their_mood_with_you_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "name_a_place", py_func("name_a_place", |args: &[Py]| name_a_place(args)));
    pyrt::xrefs::reg_fn("compose.remark", "fill_age", py_func("fill_age", |args: &[Py]| fill_age(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_age_pool", py_func("_age_pool", |args: &[Py]| _age_pool(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_job_in", py_func("_the_job_in", |args: &[Py]| _the_job_in(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_pick_filled", py_func("_pick_filled", |args: &[Py]| _pick_filled(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_people_in", py_func("_the_people_in", |args: &[Py]| _the_people_in(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_age_i_had", py_func("_the_age_i_had", |args: &[Py]| _the_age_i_had(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_an_age_put_right", py_func("_an_age_put_right", |args: &[Py]| _an_age_put_right(args)));
    pyrt::xrefs::reg_fn("compose.remark", "their_people_reply_for", py_func("their_people_reply_for", |args: &[Py]| their_people_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_thing_they_have", py_func("_the_thing_they_have", |args: &[Py]| _the_thing_they_have(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_class_word", py_func("_the_class_word", |args: &[Py]| _the_class_word(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_article", py_func("_article", |args: &[Py]| _article(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_the_fact_about", py_func("_the_fact_about", |args: &[Py]| _the_fact_about(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_capped", py_func("_capped", |args: &[Py]| _capped(args)));
    pyrt::xrefs::reg_fn("compose.remark", "a_thing_of_theirs_reply_for", py_func("a_thing_of_theirs_reply_for", |args: &[Py]| a_thing_of_theirs_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "the_two_of_them", py_func("the_two_of_them", |args: &[Py]| the_two_of_them(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_has_phrase", py_func("_has_phrase", |args: &[Py]| _has_phrase(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_planned_word", py_func("_planned_word", |args: &[Py]| _planned_word(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_going_to_plan", py_func("_going_to_plan", |args: &[Py]| _going_to_plan(args)));
    pyrt::xrefs::reg_fn("compose.remark", "reads_as_plan", py_func("reads_as_plan", |args: &[Py]| reads_as_plan(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_asked_last", py_func("_asked_last", |args: &[Py]| _asked_last(args)));
    pyrt::xrefs::reg_fn("compose.remark", "a_want_said_back", py_func("a_want_said_back", |args: &[Py]| a_want_said_back(args)));
    pyrt::xrefs::reg_fn("compose.remark", "dependant_want_reply_for", py_func("dependant_want_reply_for", |args: &[Py]| dependant_want_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "reads_as_arranged", py_func("reads_as_arranged", |args: &[Py]| reads_as_arranged(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_says_when_it_is", py_func("_says_when_it_is", |args: &[Py]| _says_when_it_is(args)));
    pyrt::xrefs::reg_fn("compose.remark", "arranged_reply_for", py_func("arranged_reply_for", |args: &[Py]| arranged_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.remark", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("compose.remark", "interest_reply_for", py_func("interest_reply_for", |args: &[Py]| interest_reply_for(args)));
    pyrt::xrefs::reg_access("compose.remark", "AGAIN", g_AGAIN, s_AGAIN);
    pyrt::xrefs::reg_access("compose.remark", "AGAIN_REPLIES", g_AGAIN_REPLIES, s_AGAIN_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "AGE_ALREADY_HAD", g_AGE_ALREADY_HAD, s_AGE_ALREADY_HAD);
    pyrt::xrefs::reg_access("compose.remark", "AGE_FIRST_TOLD", g_AGE_FIRST_TOLD, s_AGE_FIRST_TOLD);
    pyrt::xrefs::reg_access("compose.remark", "AGE_PUT_RIGHT", g_AGE_PUT_RIGHT, s_AGE_PUT_RIGHT);
    pyrt::xrefs::reg_access("compose.remark", "AGE_WORDS", g_AGE_WORDS, s_AGE_WORDS);
    pyrt::xrefs::reg_access("compose.remark", "ALREADY_DATED", g_ALREADY_DATED, s_ALREADY_DATED);
    pyrt::xrefs::reg_access("compose.remark", "ARRANGED", g_ARRANGED, s_ARRANGED);
    pyrt::xrefs::reg_access("compose.remark", "ARRANGEMENT", g_ARRANGEMENT, s_ARRANGEMENT);
    pyrt::xrefs::reg_access("compose.remark", "ASKED_OF_THE_PERSON", g_ASKED_OF_THE_PERSON, s_ASKED_OF_THE_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("compose.remark", "A_CHILD_NAMED", g_A_CHILD_NAMED, s_A_CHILD_NAMED);
    pyrt::xrefs::reg_access("compose.remark", "A_CONDOLENCE", g_A_CONDOLENCE, s_A_CONDOLENCE);
    pyrt::xrefs::reg_access("compose.remark", "A_GROUP_OF_PEOPLE", g_A_GROUP_OF_PEOPLE, s_A_GROUP_OF_PEOPLE);
    pyrt::xrefs::reg_access("compose.remark", "A_HELD_NAME_AND_AN_AGE", g_A_HELD_NAME_AND_AN_AGE, s_A_HELD_NAME_AND_AN_AGE);
    pyrt::xrefs::reg_access("compose.remark", "A_LOSS_IN_THE_LINE", g_A_LOSS_IN_THE_LINE, s_A_LOSS_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.remark", "A_MILESTONE_TAIL", g_A_MILESTONE_TAIL, s_A_MILESTONE_TAIL);
    pyrt::xrefs::reg_access("compose.remark", "A_NIGHT_OUT", g_A_NIGHT_OUT, s_A_NIGHT_OUT);
    pyrt::xrefs::reg_access("compose.remark", "A_PERSON_LOST", g_A_PERSON_LOST, s_A_PERSON_LOST);
    pyrt::xrefs::reg_access("compose.remark", "A_PERSON_OF_THEIRS", g_A_PERSON_OF_THEIRS, s_A_PERSON_OF_THEIRS);
    pyrt::xrefs::reg_access("compose.remark", "A_THING_CARRIED_OFF", g_A_THING_CARRIED_OFF, s_A_THING_CARRIED_OFF);
    pyrt::xrefs::reg_access("compose.remark", "A_THING_TAKEN", g_A_THING_TAKEN, s_A_THING_TAKEN);
    pyrt::xrefs::reg_access("compose.remark", "A_WEAPON", g_A_WEAPON, s_A_WEAPON);
    pyrt::xrefs::reg_access("compose.remark", "BABY_WORDS", g_BABY_WORDS, s_BABY_WORDS);
    pyrt::xrefs::reg_access("compose.remark", "BAD_WITH_THEM", g_BAD_WITH_THEM, s_BAD_WITH_THEM);
    pyrt::xrefs::reg_access("compose.remark", "BARE_SETBACK", g_BARE_SETBACK, s_BARE_SETBACK);
    pyrt::xrefs::reg_access("compose.remark", "BARRIER", g_BARRIER, s_BARRIER);
    pyrt::xrefs::reg_access("compose.remark", "BIGGER_PLAN", g_BIGGER_PLAN, s_BIGGER_PLAN);
    pyrt::xrefs::reg_access("compose.remark", "BOOKED", g_BOOKED, s_BOOKED);
    pyrt::xrefs::reg_access("compose.remark", "BOOKED_REPLIES", g_BOOKED_REPLIES, s_BOOKED_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "BURGLED", g_BURGLED, s_BURGLED);
    pyrt::xrefs::reg_access("compose.remark", "BURGLED_FOR_SOMEONE_REPLIES", g_BURGLED_FOR_SOMEONE_REPLIES, s_BURGLED_FOR_SOMEONE_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "BURGLED_HOME", g_BURGLED_HOME, s_BURGLED_HOME);
    pyrt::xrefs::reg_access("compose.remark", "BURGLED_NOT_THEIRS", g_BURGLED_NOT_THEIRS, s_BURGLED_NOT_THEIRS);
    pyrt::xrefs::reg_access("compose.remark", "BURGLED_REPLIES", g_BURGLED_REPLIES, s_BURGLED_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "BURGLED_SOMEONE_ELSE", g_BURGLED_SOMEONE_ELSE, s_BURGLED_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("compose.remark", "COUNTERFACTUAL", g_COUNTERFACTUAL, s_COUNTERFACTUAL);
    pyrt::xrefs::reg_access("compose.remark", "CRACKED_A_JOKE", g_CRACKED_A_JOKE, s_CRACKED_A_JOKE);
    pyrt::xrefs::reg_access("compose.remark", "CRISIS", g_CRISIS, s_CRISIS);
    pyrt::xrefs::reg_access("compose.remark", "DEADLINE_LATER", g_DEADLINE_LATER, s_DEADLINE_LATER);
    pyrt::xrefs::reg_access("compose.remark", "DEPENDANT", g_DEPENDANT, s_DEPENDANT);
    pyrt::xrefs::reg_access("compose.remark", "DEPENDANT_WANT", g_DEPENDANT_WANT, s_DEPENDANT_WANT);
    pyrt::xrefs::reg_access("compose.remark", "DEPENDANT_WANT_REPLIES", g_DEPENDANT_WANT_REPLIES, s_DEPENDANT_WANT_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "DONE_TO_ME", g_DONE_TO_ME, s_DONE_TO_ME);
    pyrt::xrefs::reg_access("compose.remark", "DOUBLE_BOOKED", g_DOUBLE_BOOKED, s_DOUBLE_BOOKED);
    pyrt::xrefs::reg_access("compose.remark", "DOUBT", g_DOUBT, s_DOUBT);
    pyrt::xrefs::reg_access("compose.remark", "FIRST_PERSON", g_FIRST_PERSON, s_FIRST_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "FIRST_STRETCH", g_FIRST_STRETCH, s_FIRST_STRETCH);
    pyrt::xrefs::reg_access("compose.remark", "FUTURE_REGRET", g_FUTURE_REGRET, s_FUTURE_REGRET);
    pyrt::xrefs::reg_access("compose.remark", "GOING_ON", g_GOING_ON, s_GOING_ON);
    pyrt::xrefs::reg_access("compose.remark", "GOING_ON_LINES", g_GOING_ON_LINES, s_GOING_ON_LINES);
    pyrt::xrefs::reg_access("compose.remark", "GOING_TO", g_GOING_TO, s_GOING_TO);
    pyrt::xrefs::reg_access("compose.remark", "HAD_IT_BOOKED", g_HAD_IT_BOOKED, s_HAD_IT_BOOKED);
    pyrt::xrefs::reg_access("compose.remark", "HE_PEOPLE", g_HE_PEOPLE, s_HE_PEOPLE);
    pyrt::xrefs::reg_access("compose.remark", "HE_PERSON", g_HE_PERSON, s_HE_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "INTEREST_BACK", g_INTEREST_BACK, s_INTEREST_BACK);
    pyrt::xrefs::reg_access("compose.remark", "INTEREST_BACK_PLAIN", g_INTEREST_BACK_PLAIN, s_INTEREST_BACK_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "INTEREST_DOING", g_INTEREST_DOING, s_INTEREST_DOING);
    pyrt::xrefs::reg_access("compose.remark", "INTEREST_DOING_PLAIN", g_INTEREST_DOING_PLAIN, s_INTEREST_DOING_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "INTEREST_STARTED", g_INTEREST_STARTED, s_INTEREST_STARTED);
    pyrt::xrefs::reg_access("compose.remark", "INTEREST_STARTED_PLAIN", g_INTEREST_STARTED_PLAIN, s_INTEREST_STARTED_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "LOSS_FOLLOW", g_LOSS_FOLLOW, s_LOSS_FOLLOW);
    pyrt::xrefs::reg_access("compose.remark", "LOSS_FOLLOW_REPLIES", g_LOSS_FOLLOW_REPLIES, s_LOSS_FOLLOW_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "MARKED_DATE", g_MARKED_DATE, s_MARKED_DATE);
    pyrt::xrefs::reg_access("compose.remark", "MARRIAGE_IN_TROUBLE", g_MARRIAGE_IN_TROUBLE, s_MARRIAGE_IN_TROUBLE);
    pyrt::xrefs::reg_access("compose.remark", "MISSED", g_MISSED, s_MISSED);
    pyrt::xrefs::reg_access("compose.remark", "NEVER_CAME_SUBJECT", g_NEVER_CAME_SUBJECT, s_NEVER_CAME_SUBJECT);
    pyrt::xrefs::reg_access("compose.remark", "NOT_AN_INTEREST", g_NOT_AN_INTEREST, s_NOT_AN_INTEREST);
    pyrt::xrefs::reg_access("compose.remark", "NOT_A_BREAK_IN", g_NOT_A_BREAK_IN, s_NOT_A_BREAK_IN);
    pyrt::xrefs::reg_access("compose.remark", "NOT_A_LOSS", g_NOT_A_LOSS, s_NOT_A_LOSS);
    pyrt::xrefs::reg_access("compose.remark", "NOT_A_NAME_AFTER_CALLED", g_NOT_A_NAME_AFTER_CALLED, s_NOT_A_NAME_AFTER_CALLED);
    pyrt::xrefs::reg_access("compose.remark", "NOT_A_RELATION_TROUBLE", g_NOT_A_RELATION_TROUBLE, s_NOT_A_RELATION_TROUBLE);
    pyrt::xrefs::reg_access("compose.remark", "NOT_A_TASTE_AS_A_THING", g_NOT_A_TASTE_AS_A_THING, s_NOT_A_TASTE_AS_A_THING);
    pyrt::xrefs::reg_access("compose.remark", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.remark", "NOT_A_WEAPON", g_NOT_A_WEAPON, s_NOT_A_WEAPON);
    pyrt::xrefs::reg_access("compose.remark", "NOT_PLAIN", g_NOT_PLAIN, s_NOT_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "NO_REGRET", g_NO_REGRET, s_NO_REGRET);
    pyrt::xrefs::reg_access("compose.remark", "OTHER_PEOPLE", g_OTHER_PEOPLE, s_OTHER_PEOPLE);
    pyrt::xrefs::reg_access("compose.remark", "OWN_ARTICLE", g_OWN_ARTICLE, s_OWN_ARTICLE);
    pyrt::xrefs::reg_access("compose.remark", "OWN_ASKS", g_OWN_ASKS, s_OWN_ASKS);
    pyrt::xrefs::reg_access("compose.remark", "OWN_ELECTRIC", g_OWN_ELECTRIC, s_OWN_ELECTRIC);
    pyrt::xrefs::reg_access("compose.remark", "OWN_ELECTRIC_CAR", g_OWN_ELECTRIC_CAR, s_OWN_ELECTRIC_CAR);
    pyrt::xrefs::reg_access("compose.remark", "OWN_FACTS", g_OWN_FACTS, s_OWN_FACTS);
    pyrt::xrefs::reg_access("compose.remark", "OWN_FAULT", g_OWN_FAULT, s_OWN_FAULT);
    pyrt::xrefs::reg_access("compose.remark", "OWN_HAVE", g_OWN_HAVE, s_OWN_HAVE);
    pyrt::xrefs::reg_access("compose.remark", "OWN_KIT", g_OWN_KIT, s_OWN_KIT);
    pyrt::xrefs::reg_access("compose.remark", "OWN_KIT_ASKS", g_OWN_KIT_ASKS, s_OWN_KIT_ASKS);
    pyrt::xrefs::reg_access("compose.remark", "OWN_KIT_NAMES", g_OWN_KIT_NAMES, s_OWN_KIT_NAMES);
    pyrt::xrefs::reg_access("compose.remark", "OWN_KIT_PLAIN", g_OWN_KIT_PLAIN, s_OWN_KIT_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "OWN_NOT_THE_THING", g_OWN_NOT_THE_THING, s_OWN_NOT_THE_THING);
    pyrt::xrefs::reg_access("compose.remark", "OWN_SLIP", g_OWN_SLIP, s_OWN_SLIP);
    pyrt::xrefs::reg_access("compose.remark", "OWN_SPACES", g_OWN_SPACES, s_OWN_SPACES);
    pyrt::xrefs::reg_access("compose.remark", "OWN_SPACES_PLAIN", g_OWN_SPACES_PLAIN, s_OWN_SPACES_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "OWN_TAIL", g_OWN_TAIL, s_OWN_TAIL);
    pyrt::xrefs::reg_access("compose.remark", "OWN_TIME", g_OWN_TIME, s_OWN_TIME);
    pyrt::xrefs::reg_access("compose.remark", "OWN_TWO_THINGS", g_OWN_TWO_THINGS, s_OWN_TWO_THINGS);
    pyrt::xrefs::reg_access("compose.remark", "PLACE_CODES", g_PLACE_CODES, s_PLACE_CODES);
    pyrt::xrefs::reg_access("compose.remark", "PLAN", g_PLAN, s_PLAN);
    pyrt::xrefs::reg_access("compose.remark", "PLANNED_VERBS", g_PLANNED_VERBS, s_PLANNED_VERBS);
    pyrt::xrefs::reg_access("compose.remark", "PLAN_REPLIES", g_PLAN_REPLIES, s_PLAN_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "PLAN_REPLIES_PLAIN", g_PLAN_REPLIES_PLAIN, s_PLAN_REPLIES_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "POLITE", g_POLITE, s_POLITE);
    pyrt::xrefs::reg_access("compose.remark", "PRACTICAL_ASK", g_PRACTICAL_ASK, s_PRACTICAL_ASK);
    pyrt::xrefs::reg_access("compose.remark", "PRESENT_SETBACK", g_PRESENT_SETBACK, s_PRESENT_SETBACK);
    pyrt::xrefs::reg_access("compose.remark", "PRICE", g_PRICE, s_PRICE);
    pyrt::xrefs::reg_access("compose.remark", "PUT_DOWN", g_PUT_DOWN, s_PUT_DOWN);
    pyrt::xrefs::reg_access("compose.remark", "QUESTION_FIRST", g_QUESTION_FIRST, s_QUESTION_FIRST);
    pyrt::xrefs::reg_access("compose.remark", "QUIET_BETWEEN_THEM", g_QUIET_BETWEEN_THEM, s_QUIET_BETWEEN_THEM);
    pyrt::xrefs::reg_access("compose.remark", "REGRET", g_REGRET, s_REGRET);
    pyrt::xrefs::reg_access("compose.remark", "REGRET_LINES", g_REGRET_LINES, s_REGRET_LINES);
    pyrt::xrefs::reg_access("compose.remark", "REGRET_LINES_PLAIN", g_REGRET_LINES_PLAIN, s_REGRET_LINES_PLAIN);
    pyrt::xrefs::reg_access("compose.remark", "RELATION_GONE_WRONG", g_RELATION_GONE_WRONG, s_RELATION_GONE_WRONG);
    pyrt::xrefs::reg_access("compose.remark", "REPORT_BODY", g_REPORT_BODY, s_REPORT_BODY);
    pyrt::xrefs::reg_access("compose.remark", "REPORT_FEELING", g_REPORT_FEELING, s_REPORT_FEELING);
    pyrt::xrefs::reg_access("compose.remark", "RISE", g_RISE, s_RISE);
    pyrt::xrefs::reg_access("compose.remark", "ROWS_BETWEEN_THEM", g_ROWS_BETWEEN_THEM, s_ROWS_BETWEEN_THEM);
    pyrt::xrefs::reg_access("compose.remark", "SETBACK", g_SETBACK, s_SETBACK);
    pyrt::xrefs::reg_access("compose.remark", "SETBACK_REPLIES", g_SETBACK_REPLIES, s_SETBACK_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "SHE_PEOPLE", g_SHE_PEOPLE, s_SHE_PEOPLE);
    pyrt::xrefs::reg_access("compose.remark", "SHE_PERSON", g_SHE_PERSON, s_SHE_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "SHOULDNT_HAVE", g_SHOULDNT_HAVE, s_SHOULDNT_HAVE);
    pyrt::xrefs::reg_access("compose.remark", "SLIP_REPLIES", g_SLIP_REPLIES, s_SLIP_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "SMASHED", g_SMASHED, s_SMASHED);
    pyrt::xrefs::reg_access("compose.remark", "SMASHED_OF_A_THING", g_SMASHED_OF_A_THING, s_SMASHED_OF_A_THING);
    pyrt::xrefs::reg_access("compose.remark", "STARTED", g_STARTED, s_STARTED);
    pyrt::xrefs::reg_access("compose.remark", "STARTED_AHEAD_REPLIES", g_STARTED_AHEAD_REPLIES, s_STARTED_AHEAD_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "STARTED_BADLY_REPLIES", g_STARTED_BADLY_REPLIES, s_STARTED_BADLY_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "STARTED_FINE_REPLIES", g_STARTED_FINE_REPLIES, s_STARTED_FINE_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "STARTED_GONE_BADLY", g_STARTED_GONE_BADLY, s_STARTED_GONE_BADLY);
    pyrt::xrefs::reg_access("compose.remark", "STARTED_REPLIES", g_STARTED_REPLIES, s_STARTED_REPLIES);
    pyrt::xrefs::reg_access("compose.remark", "STARTED_VERDICT", g_STARTED_VERDICT, s_STARTED_VERDICT);
    pyrt::xrefs::reg_access("compose.remark", "STARTED_WELL", g_STARTED_WELL, s_STARTED_WELL);
    pyrt::xrefs::reg_access("compose.remark", "START_AHEAD", g_START_AHEAD, s_START_AHEAD);
    pyrt::xrefs::reg_access("compose.remark", "START_CLOCK", g_START_CLOCK, s_START_CLOCK);
    pyrt::xrefs::reg_access("compose.remark", "START_PAST", g_START_PAST, s_START_PAST);
    pyrt::xrefs::reg_access("compose.remark", "START_PRESENT", g_START_PRESENT, s_START_PRESENT);
    pyrt::xrefs::reg_access("compose.remark", "STEP_NOUNS", g_STEP_NOUNS, s_STEP_NOUNS);
    pyrt::xrefs::reg_access("compose.remark", "TAKEN_FROM_THEM", g_TAKEN_FROM_THEM, s_TAKEN_FROM_THEM);
    pyrt::xrefs::reg_access("compose.remark", "TASTE_TOO_LONG", g_TASTE_TOO_LONG, s_TASTE_TOO_LONG);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_AGE", g_THEIR_AGE, s_THEIR_AGE);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_BABY_LINES", g_THEIR_BABY_LINES, s_THEIR_BABY_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_CALLED", g_THEIR_CALLED, s_THEIR_CALLED);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_CHILD", g_THEIR_CHILD, s_THEIR_CHILD);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_CHILD_LINES", g_THEIR_CHILD_LINES, s_THEIR_CHILD_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_DISLIKE", g_THEIR_DISLIKE, s_THEIR_DISLIKE);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_DISLIKE_LINES", g_THEIR_DISLIKE_LINES, s_THEIR_DISLIKE_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_ELDER", g_THEIR_ELDER, s_THEIR_ELDER);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_ELDER_AGE", g_THEIR_ELDER_AGE, s_THEIR_ELDER_AGE);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_ELDER_LINES", g_THEIR_ELDER_LINES, s_THEIR_ELDER_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_GROWN_LINES", g_THEIR_GROWN_LINES, s_THEIR_GROWN_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_HATE", g_THEIR_HATE, s_THEIR_HATE);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_HATE_LINES", g_THEIR_HATE_LINES, s_THEIR_HATE_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_JOB", g_THEIR_JOB, s_THEIR_JOB);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_JOB_LINES", g_THEIR_JOB_LINES, s_THEIR_JOB_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_LIKING", g_THEIR_LIKING, s_THEIR_LIKING);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_LIKING_LINES", g_THEIR_LIKING_LINES, s_THEIR_LIKING_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_MOOD_WITH_YOU", g_THEIR_MOOD_WITH_YOU, s_THEIR_MOOD_WITH_YOU);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_MOOD_WITH_YOU_LINES", g_THEIR_MOOD_WITH_YOU_LINES, s_THEIR_MOOD_WITH_YOU_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_NAME", g_THEIR_NAME, s_THEIR_NAME);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_NAMES_LINES", g_THEIR_NAMES_LINES, s_THEIR_NAMES_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_NAME_AGE", g_THEIR_NAME_AGE, s_THEIR_NAME_AGE);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_NAME_AGE_LINES", g_THEIR_NAME_AGE_LINES, s_THEIR_NAME_AGE_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_NAME_GROWN_LINES", g_THEIR_NAME_GROWN_LINES, s_THEIR_NAME_GROWN_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_NAME_LINES", g_THEIR_NAME_LINES, s_THEIR_NAME_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_NAME_SO", g_THEIR_NAME_SO, s_THEIR_NAME_SO);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_PERSON", g_THEIR_PERSON, s_THEIR_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_PLACE", g_THEIR_PLACE, s_THEIR_PLACE);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_PLACE_LINES", g_THEIR_PLACE_LINES, s_THEIR_PLACE_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_QUALIFIER", g_THEIR_QUALIFIER, s_THEIR_QUALIFIER);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_SMALL_LINES", g_THEIR_SMALL_LINES, s_THEIR_SMALL_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEIR_TEEN_LINES", g_THEIR_TEEN_LINES, s_THEIR_TEEN_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THEY_PERSON", g_THEY_PERSON, s_THEY_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "THE_PERSON_WITH_IT", g_THE_PERSON_WITH_IT, s_THE_PERSON_WITH_IT);
    pyrt::xrefs::reg_access("compose.remark", "THE_QUIET_LINES", g_THE_QUIET_LINES, s_THE_QUIET_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THE_RELATION_LINES", g_THE_RELATION_LINES, s_THE_RELATION_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THE_ROWS_LINES", g_THE_ROWS_LINES, s_THE_ROWS_LINES);
    pyrt::xrefs::reg_access("compose.remark", "THE_THING_IS_A_CLAUSE", g_THE_THING_IS_A_CLAUSE, s_THE_THING_IS_A_CLAUSE);
    pyrt::xrefs::reg_access("compose.remark", "THOUGHT_OBJECT", g_THOUGHT_OBJECT, s_THOUGHT_OBJECT);
    pyrt::xrefs::reg_access("compose.remark", "TOOK_IT_IN", g_TOOK_IT_IN, s_TOOK_IT_IN);
    pyrt::xrefs::reg_access("compose.remark", "TURNED", g_TURNED, s_TURNED);
    pyrt::xrefs::reg_access("compose.remark", "WANTS_TO_BE", g_WANTS_TO_BE, s_WANTS_TO_BE);
    pyrt::xrefs::reg_access("compose.remark", "WEAPON_WORDS", g_WEAPON_WORDS, s_WEAPON_WORDS);
    pyrt::xrefs::reg_access("compose.remark", "WENT_WRONG", g_WENT_WRONG, s_WENT_WRONG);
    pyrt::xrefs::reg_access("compose.remark", "WISH_I", g_WISH_I, s_WISH_I);
    pyrt::xrefs::reg_access("compose.remark", "YEAR_UNIT", g_YEAR_UNIT, s_YEAR_UNIT);
    pyrt::xrefs::reg_access("compose.remark", "_AHEAD_DAY", g__AHEAD_DAY, s__AHEAD_DAY);
    pyrt::xrefs::reg_access("compose.remark", "_HARM_AT_PERSON", g__HARM_AT_PERSON, s__HARM_AT_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "_OWN_SHOULD", g__OWN_SHOULD, s__OWN_SHOULD);
    pyrt::xrefs::reg_access("compose.remark", "_RELATION_PERSON", g__RELATION_PERSON, s__RELATION_PERSON);
    pyrt::xrefs::reg_access("compose.remark", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("compose.remark", "without_leaders", g_without_leaders, s_without_leaders);
}
