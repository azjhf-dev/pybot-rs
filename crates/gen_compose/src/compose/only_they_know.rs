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


thread_local! { static G_ABOUT_ME_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_IS_THE_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AM_I_TOO_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AM_I_TOO_OLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_MY_VIEW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BODY_OR_ILLNESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FACT_ABOUT_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_ABOUT_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GIFT_OR_TASTE_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MILD_GRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NUMBER_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_ABOUT_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ROW_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE_ABOUT_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUSPICION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUSPICION_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_IT_IS_TOO_OLD_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT_ASKED_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WH_ABOUT_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVERYDAY_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGEMENT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_MYSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_MYSELF_NO_ROW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_SOMEONE_AT_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_THEIR_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_THE_TWO_OF_US: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FACT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FEELING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERMISSION_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEBODY_ELSES_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE_JUDGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_FEELINGS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_FEELING_TOWARD_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON_FIGURE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON_THROUGH_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON_WH_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_FEEL_ABOUT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_OLD_FOR_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_THEY_DID_THAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_TOWARD_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_WITHDRAWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_THE_ASKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANY_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_TWO_OF_US: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THING_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_ME_ASKED() -> Py { ensure_init(); G_ABOUT_ME_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_ME_ASKED(v: Py) { ensure_init(); G_ABOUT_ME_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_IS_THE_RULE() -> Py { ensure_init(); G_AGE_IS_THE_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_IS_THE_RULE(v: Py) { ensure_init(); G_AGE_IS_THE_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_AM_I_TOO_MUCH() -> Py { ensure_init(); G_AM_I_TOO_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AM_I_TOO_MUCH(v: Py) { ensure_init(); G_AM_I_TOO_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g_AM_I_TOO_OLD() -> Py { ensure_init(); G_AM_I_TOO_OLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AM_I_TOO_OLD(v: Py) { ensure_init(); G_AM_I_TOO_OLD.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_MY_VIEW() -> Py { ensure_init(); G_ASKS_MY_VIEW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_MY_VIEW(v: Py) { ensure_init(); G_ASKS_MY_VIEW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BODY_OR_ILLNESS() -> Py { ensure_init(); G_A_BODY_OR_ILLNESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BODY_OR_ILLNESS(v: Py) { ensure_init(); G_A_BODY_OR_ILLNESS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FACT_ABOUT_THEIR_PERSON() -> Py { ensure_init(); G_A_FACT_ABOUT_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FACT_ABOUT_THEIR_PERSON(v: Py) { ensure_init(); G_A_FACT_ABOUT_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_ABOUT_THEIR_PERSON() -> Py { ensure_init(); G_A_FIGURE_ABOUT_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_ABOUT_THEIR_PERSON(v: Py) { ensure_init(); G_A_FIGURE_ABOUT_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GIFT_OR_TASTE_ASKED() -> Py { ensure_init(); G_A_GIFT_OR_TASTE_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GIFT_OR_TASTE_ASKED(v: Py) { ensure_init(); G_A_GIFT_OR_TASTE_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MILD_GRADE() -> Py { ensure_init(); G_A_MILD_GRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MILD_GRADE(v: Py) { ensure_init(); G_A_MILD_GRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NUMBER_IN_THE_LINE() -> Py { ensure_init(); G_A_NUMBER_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NUMBER_IN_THE_LINE(v: Py) { ensure_init(); G_A_NUMBER_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_ABOUT_THEIR_PERSON() -> Py { ensure_init(); G_A_PLACE_ABOUT_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_ABOUT_THEIR_PERSON(v: Py) { ensure_init(); G_A_PLACE_ABOUT_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ROW_IN_THE_LINE() -> Py { ensure_init(); G_A_ROW_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ROW_IN_THE_LINE(v: Py) { ensure_init(); G_A_ROW_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE_ABOUT_THEIR_PERSON() -> Py { ensure_init(); G_A_STATE_ABOUT_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE_ABOUT_THEIR_PERSON(v: Py) { ensure_init(); G_A_STATE_ABOUT_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUSPICION() -> Py { ensure_init(); G_A_SUSPICION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUSPICION(v: Py) { ensure_init(); G_A_SUSPICION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUSPICION_LINES() -> Py { ensure_init(); G_A_SUSPICION_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUSPICION_LINES(v: Py) { ensure_init(); G_A_SUSPICION_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_IT_IS_TOO_OLD_FOR() -> Py { ensure_init(); G_A_THING_IT_IS_TOO_OLD_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_IT_IS_TOO_OLD_FOR(v: Py) { ensure_init(); G_A_THING_IT_IS_TOO_OLD_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT_ASKED_FOR() -> Py { ensure_init(); G_A_VERDICT_ASKED_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT_ASKED_FOR(v: Py) { ensure_init(); G_A_VERDICT_ASKED_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WH_ABOUT_THEIR_PERSON() -> Py { ensure_init(); G_A_WH_ABOUT_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WH_ABOUT_THEIR_PERSON(v: Py) { ensure_init(); G_A_WH_ABOUT_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_EVERYDAY_VERBS() -> Py { ensure_init(); G_EVERYDAY_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVERYDAY_VERBS(v: Py) { ensure_init(); G_EVERYDAY_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGEMENT_WORDS() -> Py { ensure_init(); G_JUDGEMENT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGEMENT_WORDS(v: Py) { ensure_init(); G_JUDGEMENT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_MYSELF() -> Py { ensure_init(); G_JUDGE_MYSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_MYSELF(v: Py) { ensure_init(); G_JUDGE_MYSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_MYSELF_NO_ROW() -> Py { ensure_init(); G_JUDGE_MYSELF_NO_ROW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_MYSELF_NO_ROW(v: Py) { ensure_init(); G_JUDGE_MYSELF_NO_ROW.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_SOMEONE_AT_AN_AGE() -> Py { ensure_init(); G_JUDGE_SOMEONE_AT_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_SOMEONE_AT_AN_AGE(v: Py) { ensure_init(); G_JUDGE_SOMEONE_AT_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_SOMEONE_ELSE() -> Py { ensure_init(); G_JUDGE_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_SOMEONE_ELSE(v: Py) { ensure_init(); G_JUDGE_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_THEIR_TASTE() -> Py { ensure_init(); G_JUDGE_THEIR_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_THEIR_TASTE(v: Py) { ensure_init(); G_JUDGE_THEIR_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_THE_TWO_OF_US() -> Py { ensure_init(); G_JUDGE_THE_TWO_OF_US.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_THE_TWO_OF_US(v: Py) { ensure_init(); G_JUDGE_THE_TWO_OF_US.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FACT_ASK() -> Py { ensure_init(); G_NOT_A_FACT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FACT_ASK(v: Py) { ensure_init(); G_NOT_A_FACT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FEELING_ASK() -> Py { ensure_init(); G_NOT_A_FEELING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FEELING_ASK(v: Py) { ensure_init(); G_NOT_A_FEELING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PERMISSION_ASKED() -> Py { ensure_init(); G_PERMISSION_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERMISSION_ASKED(v: Py) { ensure_init(); G_PERMISSION_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEBODY_ELSES_PERSON() -> Py { ensure_init(); G_SOMEBODY_ELSES_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY_ELSES_PERSON(v: Py) { ensure_init(); G_SOMEBODY_ELSES_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE_JUDGED() -> Py { ensure_init(); G_SOMEONE_ELSE_JUDGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE_JUDGED(v: Py) { ensure_init(); G_SOMEONE_ELSE_JUDGED.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_FEELINGS_LINES() -> Py { ensure_init(); G_THEIR_FEELINGS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_FEELINGS_LINES(v: Py) { ensure_init(); G_THEIR_FEELINGS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_FEELING_TOWARD_ME() -> Py { ensure_init(); G_THEIR_FEELING_TOWARD_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_FEELING_TOWARD_ME(v: Py) { ensure_init(); G_THEIR_FEELING_TOWARD_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON_FIGURE_LINES() -> Py { ensure_init(); G_THEIR_PERSON_FIGURE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON_FIGURE_LINES(v: Py) { ensure_init(); G_THEIR_PERSON_FIGURE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON_LINES() -> Py { ensure_init(); G_THEIR_PERSON_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON_LINES(v: Py) { ensure_init(); G_THEIR_PERSON_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON_THROUGH_ANOTHER() -> Py { ensure_init(); G_THEIR_PERSON_THROUGH_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON_THROUGH_ANOTHER(v: Py) { ensure_init(); G_THEIR_PERSON_THROUGH_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON_WH_LINES() -> Py { ensure_init(); G_THEIR_PERSON_WH_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON_WH_LINES(v: Py) { ensure_init(); G_THEIR_PERSON_WH_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON_WORD() -> Py { ensure_init(); G_THEIR_PERSON_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON_WORD(v: Py) { ensure_init(); G_THEIR_PERSON_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_FEEL_ABOUT_ME() -> Py { ensure_init(); G_THEY_FEEL_ABOUT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_FEEL_ABOUT_ME(v: Py) { ensure_init(); G_THEY_FEEL_ABOUT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_OLD_FOR_IT() -> Py { ensure_init(); G_TOO_OLD_FOR_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_OLD_FOR_IT(v: Py) { ensure_init(); G_TOO_OLD_FOR_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_THEY_DID_THAT() -> Py { ensure_init(); G_WHY_THEY_DID_THAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_THEY_DID_THAT(v: Py) { ensure_init(); G_WHY_THEY_DID_THAT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_TOWARD_ME() -> Py { ensure_init(); G_WHY_TOWARD_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_TOWARD_ME(v: Py) { ensure_init(); G_WHY_TOWARD_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_WITHDRAWN() -> Py { ensure_init(); G_WHY_WITHDRAWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_WITHDRAWN(v: Py) { ensure_init(); G_WHY_WITHDRAWN.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_THE_ASKER() -> Py { ensure_init(); G__ABOUT_THE_ASKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_THE_ASKER(v: Py) { ensure_init(); G__ABOUT_THE_ASKER.with(|c| *c.borrow_mut() = v); }
pub fn g__ANY_PERSON() -> Py { ensure_init(); G__ANY_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANY_PERSON(v: Py) { ensure_init(); G__ANY_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON_FORMS() -> Py { ensure_init(); G__PERSON_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON_FORMS(v: Py) { ensure_init(); G__PERSON_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON_OBJECT() -> Py { ensure_init(); G__PERSON_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON_OBJECT(v: Py) { ensure_init(); G__PERSON_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON_SUBJECT() -> Py { ensure_init(); G__PERSON_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON_SUBJECT(v: Py) { ensure_init(); G__PERSON_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__RELATION() -> Py { ensure_init(); G__RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RELATION(v: Py) { ensure_init(); G__RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_TWO_OF_US() -> Py { ensure_init(); G__THE_TWO_OF_US.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_TWO_OF_US(v: Py) { ensure_init(); G__THE_TWO_OF_US.with(|c| *c.borrow_mut() = v); }
pub fn g__THING_OF_THEIRS() -> Py { ensure_init(); G__THING_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THING_OF_THEIRS(v: Py) { ensure_init(); G__THING_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What only the person in front of it can know or judge.\n\nThree shapes of line reached the search hedge or a placeholder when nothing else knew, and none\nis a look-up:\n\n- a judgement put to the bot - \"do you think shes too old for a day out\", \"was i out of order\",\n  \"do you think i handled it badly\", \"am i too old for a festival\". \"A quick search online will\n  get you a solid answer\" sends somebody off to read about their own mother;\n- a yes/no about one of their own people - \"does my mum like golf\", \"does she drive\", \"do they\n  get on at that age\". No page anywhere knows their mother's taste in golf;\n- how somebody else feels about the asker, or why they did a thing to them - \"is he avoiding me\",\n  \"is my dad proud of me\", \"why did she not invite me\", \"am i annoying her\". The answer is in\n  another person's head, and a guess is worse than saying so.\n\nAll three are answered here instead: the reply hands back what only they can say, and asks a question\nso the turn keeps going. A judgement is not a fact gap, so the honest hedge is the wrong shape for\nit, and a fact about somebody the bot has never met is not on any page.\n\nThis lives beside the composer rather than inside `fallback.py`, which the world caps at 1 MiB and\nwas over it when this was written (772 KB today), and `bot/modules/greeting.py` calls it the way it\ncalls `repeat` and `too_late`: only when the reply in hand is the composer's own \"I cannot answer\nthat\" note, so a line with an answer of its own keeps it. `should` (advice), permissions (\"is it ok to text an ex\") and \"what do you think about my\nsister\" (talking about a person, not judging one) all stand down, as does the bare \"am i too old\",\nwhich is the too-late reading's turn.\n");
    s_JUDGEMENT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:too\\s+\\w+|old\\s+enough|ready|fair|fair\\s+enough|right|wrong|harsh|strict|soft|hard\\s+on|badly|rude|unkind|unfair|blunt|mean|a\\s+bit\\s+much|overreact\\w*|over\\s+the\\s+top|out\\s+of\\s+order|unreasonable|mistake|handled|selfish|childish|silly|stupid|ok(?:ay)?|alright|worth\\s+it|would\\s+(?:like|love|enjoy|hate|mind|be\\s+upset))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PERMISSION_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|would|will)\\s+it\\s+(?:ok(?:ay)?|alright|fine|rude|weird|bad)\\b|\\bis\\s+(?:it|that)\\s+ok(?:ay)?\\s+(?:to|if)\\b|\\bits\\s+(?:ok(?:ay)?|alright|fine)\\s+(?:to|if)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_MY_VIEW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:do|did|would)\\s+(?:you|u|ya)\\s+(?:think|reckon)\\b|\\b(?:d'?you|dya)\\s+(?:you\\s+)?(?:think|reckon)\\b|^\\s*(?:u|ya|you)\\s+think\\b|\\bwould\\s+you\\s+say\\b|\\bwhat\\s+(?:do|would)\\s+(?:you|u|ya)\\s+(?:think|reckon|say)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_VERDICT_ASKED_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:and|so|but|ok|okay|well|honestly|seriously|be\\s+honest)\\s+)*(?:was|were|am|did|do|have)\\s+(?:i|we|that|it)\\b[^.?!]{0,40}\\b(?:out\\s+of\\s+order|overreact\\w*|over\\s+the\\s+top|too\\s+(?:harsh|strict|hard|soft|much|blunt|quick)|unreasonable|in\\s+the\\s+wrong|right\\s+thing|wrong(?:ly)?|handled|fair|badly|rude|unfair|unkind|mean|a\\s+mistake)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_MILD_GRADE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:was|were|am|is|did)\\s+(?:i|that|it|we)\\b[^.?!]{0,32}?(?:\\b(?:a\\s+bit|a\\s+little|rather|quite|being)\\s+(?:harsh|soft|strict|blunt|mean|hard|much)\\b|\\b(?:harsh|soft|strict|blunt|mean|hard)\\s+(?:of|on)\\s+(?:me|him|her|them|us|my)\\b|\\b(?:harsh|soft|strict|blunt|mean)\\b\\s*[?.!]*\\s*$)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__RELATION(py_str("mum|mom|mother|dad|father|parent|parents|son|daughter|kid|kids|child|children|baby|brother|brothers|sister|sisters|sibling|siblings|nan|nana|grandma|grandad|granny|grandparents?|wife|husband|partner|boyfriend|girlfriend|ex|friend|friends|mate|mates|neighbour|neighbours|flatmate|housemate|landlord|boss|manager|colleague|carer|boy|girl|little\\s+ones?|teenager|teen|toddler|lad|lass|bloke|in[- ]laws?|aunt|auntie|uncle|cousin|cousins|niece|nephew|family"));
    s_SOMEONE_ELSE_JUDGED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:he|he'?s|she|she'?s|they|they'?re|him|her|them)\\b|\\b(?:my|our|his|her|their)\\s+(?:"), &g__RELATION()), &py_str(")\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THE_TWO_OF_US({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we|us|our\\s+(?:relationship|marriage))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THING_OF_THEIRS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:my|our)\\s+(?!"), &g__RELATION()), &py_str(")\\w+"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ABOUT_THE_ASKER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|me|myself|ive|im)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AM_I_TOO_OLD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:and|so|but|ok|okay|well|honestly)\\s+)*(?:am\\s+i|do\\s+you\\s+think\\s+i(?:\\s+am|'?m|\\s+was)|a?m\\s+i)\\b[^.?!]{0,30}\\b(?:too\\s+old|past\\s+it|over\\s+the\\s+hill)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_IT_IS_TOO_OLD_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:too\\s+old|past\\s+it)\\s+(?:for|to)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGE_IS_THE_RULE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:mammogram|smear|screening|screen\\w*|scan|test|surgery|operation|operat\\w*|replacement|transplant|hrt|hormone\\w*|medication|medicine|jab|vaccine|vaccinat\\w*|fast|fasting|blood|donor|donat\\w*|army|navy|raf|forces|military|police|fire\\s*service|foster\\w*|adopt\\w*|vote|voting|voter|licence|license|driv\\w*|pension|retire\\w*|insurance|mortgage|visa|pilot|airline|midwife\\w*|nurse|teach\\w*|carer|childminder)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TOO_OLD_FOR_IT(py_tuple(vec![py_str("The number is the least useful part of it: what matters is whether you would enjoy the thing. What is the bit you would look forward to?"), py_str("Nobody is too old for a thing they want to do - the ones who regret it are the ones who waited. What is it that has you hesitating?"), py_str("I would go on what you want rather than on your age, honestly. What would you do if nobody was going to comment?")]));
    s_JUDGE_SOMEONE_ELSE(py_tuple(vec![py_str("I would go on what you have seen them do - you know them and I never will. What is it you are not sure about?"), py_str("Only you know them well enough to say, and I would trust that over anything I could guess. What has you wondering?"), py_str("That one is yours rather than mine, honestly: you are the one who knows them. What would a good version of it look like?")]));
    s_JUDGE_SOMEONE_AT_AN_AGE(py_tuple(vec![py_str("I would go on what you have seen them do, not on a number - you know them and I never will. What is it you are not sure about?"), py_str("Nobody can weigh that from outside, and the number is the least useful part of it anyway. What would a good day of it look like for them?")]));
    s_A_NUMBER_IN_THE_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred)\\b|\\b(?:years?\\s+old|how\\s+old|old\\s+enough|too\\s+(?:old|young)|ready|at\\s+that\\s+age)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GIFT_OR_TASTE_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:would|will|do\\s+you\\s+think)\\b[^.?!]{0,25}\\b(?:like|love|want|wear|use|enjoy|suit)\\b[^.?!]{0,20}\\b(?:this|that|it|these|those)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JUDGE_THEIR_TASTE(py_tuple(vec![py_str("Whether {who} would like it is for {them} to say, not mine to guess. What is it, and what has {who} liked before?"), py_str("I have never met {who}, so I cannot tell you {poss} taste. What is the thing, and what does {who} usually go for?"), py_str("Only {who} knows what {who} likes, honestly. What were you thinking of getting?")]));
    s_A_ROW_IN_THE_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:said|say|saying|told|tells|texted|messaged|wrote|replied|answered|shouted|snapped|argued|arguing|argument|row|rowed|fought|fighting|fight|falling out|fell out|handled|handle|overreacted|overreacting|rude|harsh|unfair|out of order|my fault|apologise|apologize|apology|confronted|tackled it|brought it up)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JUDGE_MYSELF(py_tuple(vec![py_str("I cannot call that from one line, and one side of a row always sounds better. What did you actually say?"), py_str("Not from here, no. Tell me what happened and what you would do differently, and I will tell you what I make of it."), py_str("That needs the whole thing rather than a line of it. What is the bit you keep going back over?")]));
    s_JUDGE_MYSELF_NO_ROW(py_tuple(vec![py_str("I cannot call that one for you from a line, and I would rather not guess at it. What is it that has you doubting it?"), py_str("That is yours to weigh rather than mine, honestly. What would change your mind?"), py_str("No way for me to say yes or no to that. Tell me what you would do if it went well.")]));
    s_JUDGE_THE_TWO_OF_US(py_tuple(vec![py_str("Nobody outside the two of you can weigh that, me least of all. What are the rows like after - forgotten by the next day, or sore for a while?"), py_str("I cannot count somebody else's rows from here. Are they about the same thing each time?")]));
    s_NOT_A_FACT_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should|shall|how\\s+do\\s+i|how\\s+can\\s+i|what\\s+do\\s+i\\s+do|what\\s+should\\s+i|hit|hits|hitting|hurt|hurts|shout\\w*|scream\\w*|push\\w*|grab\\w*|threat\\w*|abuse\\w*|touch\\w*|assault\\w*|rape\\w*|died|dying|death|passed\\s+away|funeral)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BODY_OR_ILLNESS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:temperature|fever|flu|covid|cough|cold|ill|sick|unwell|poorly|pain|pains|ache|aches|aching|sore|hurt|hurts|hurting|injury|injured|symptom|symptoms|rash|swelling|swollen|bleed\\w*|blood|dose|doses|insulin|inhaler|medication|medicine|tablet|tablets|pill|pills|antibiotic\\w*|doctor|gp|hospital|nurse|pharmac\\w*|checkup|check-up|scan|pregnan\\w*|asthma|diabet\\w*|dementia|depress\\w*|anxious|anxiety|allerg\\w*|eat\\s+enough|eating\\s+enough)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EVERYDAY_VERBS(py_str("like|love|hate|prefer|want|enjoy|mind|drive|drink|smoke|play|support|vote|get\\s+on|get\\s+along|take\\s+(?:sugar|milk)|eat|care|sleep|nap|walk|run|swim|read|cook|bake|garden|fish|shop|work|live|earn|study|go\\s+out|go\\s+to|come\\s+to|come\\s+round|visit|call|text|ring|write|draw|sing|dance|travel|watch|listen|collect|keep|wear|use|remember|know|think|believe|volunteer|retir\\w*|cycle|park|belong|help\\s+out|help|have\\s+a|have\\s+any|need\\s+a"));
    s__ANY_PERSON(op_add(&op_add(&py_str("(?:he|she|they|him|her|them|(?:my|our|his|her|their)\\s+(?:"), &g__RELATION()), &py_str("))")));
    s_A_FACT_ABOUT_THEIR_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("^\\s*(?:(?:and|so|but|ok|okay|well|also|anyway|btw)\\s+)*(?:does|do|did|would|will)\\s+(?P<who>"), &g__ANY_PERSON()), &py_str(")\\s+(?:(?:still|actually|even|really|ever|now)\\s+)?(?:")), &g_EVERYDAY_VERBS()), &py_str(")\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WH_ABOUT_THEIR_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("^\\s*(?:(?:and|so|but|ok|okay|well|also|anyway|btw)\\s+)*(?:what|where|who|when|whats|wheres|how\\s+often|how\\s+long)(?:'s|\\s+is|\\s+was|\\s+does|\\s+do|\\s+did|\\s+has|\\s+had|\\s+are|\\s+were)?\\s+(?P<who>"), &g__ANY_PERSON()), &py_str(")\\s+(?:(?:still|actually|even|really|ever|now|usually|normally)\\s+)?(?:up\\s+to|into|do|doing|say|spend|")), &g_EVERYDAY_VERBS()), &py_str(")(?:s|es|ed|d|ing)?\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_STATE_ABOUT_THEIR_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^\\s*(?:is|was|are|were)\\s+(?P<who>"), &g__ANY_PERSON()), &py_str(")\\s+(?:(?:still|actually|even|really|now|also)\\s+)?(?:retired|married|single|divorced|separated|widowed|ok|okay|alright|happy|unhappy|well|better|alive|working|studying|at\\s+home|back|settled|keeping\\s+well)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PLACE_ABOUT_THEIR_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^\\s*(?:is|was|are|were)\\s+(?P<who>"), &g__ANY_PERSON()), &py_str(")\\s+(?:(?:still|actually|even|really|now|already|finally|back)\\s+)?(?:in|at|near|round|around|off\\s+to|down\\s+in|up\\s+in|over\\s+in|back\\s+in|out\\s+in)\\s+(?!a\\s|an\\s|the\\s|pain|trouble|danger|labour|labor|tears|need|agony|hospital\\s+bed)[a-z]"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FIGURE_ABOUT_THEIR_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^\\s*(?:is|was|are|were)\\s+(?P<who>"), &g__ANY_PERSON()), &py_str(")\\s+(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_PERSON_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:my|our)\\s+("), &g__RELATION()), &py_str(")\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_PERSON_THROUGH_ANOTHER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:his|her|their)\\s+("), &g__RELATION()), &py_str(")\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_PERSON_WH_LINES(py_tuple(vec![py_str("I cannot check that from here - {she} is the one who knows. Ask {who}, and tell me what you get."), py_str("Nothing on this side holds that; only {she} can say. What made you ask?"), py_str("You would have to ask {who} - that is not a thing I can look up. What has you wondering?")]));
    s_THEIR_PERSON_FIGURE_LINES(py_tuple(vec![py_str("I cannot check that from here, and {she} would know better than any page. Ask {who}, and tell me what you get."), py_str("Nothing I can see holds that one - {she} is the one to check with. What has you asking?")]));
    s_THEIR_PERSON_LINES(py_tuple(vec![py_str("I have never met {who}, so that one is yours to answer. What made you ask?"), py_str("That is a fact about {who} rather than something I could look up. What were you thinking of doing about it?"), py_str("Only you can say that about {who}, honestly. What has you wondering?")]));
    s_THEIR_FEELINGS_LINES(py_tuple(vec![py_str("Only {subject} can tell you that, and I would not guess at it. What has happened between you?"), py_str("That is not mine to answer, but I would ask {object} plainly rather than sit guessing. What has you wondering?"), py_str("Nobody can settle that from outside the two of you. What made you ask?")]));
    s_ABOUT_ME_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:about|for|with|of)\\s+me\\b|\\b(?:love|loves|like|likes|want|wants|need|needs|miss|misses|fancy|fancies|hate|hates)\\s+me\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PERSON_SUBJECT(py_dict(vec![(py_str("him"), py_str("he")), (py_str("her"), py_str("she")), (py_str("them"), py_str("they"))]));
    s__PERSON_OBJECT(py_dict(vec![(py_str("he"), py_str("him")), (py_str("she"), py_str("her")), (py_str("they"), py_str("them")), (py_str("him"), py_str("him")), (py_str("her"), py_str("her")), (py_str("them"), py_str("them"))]));
    s__PERSON_FORMS(py_dict(vec![(py_str("he"), py_tuple(vec![py_str("he"), py_str("him"), py_str("his")])), (py_str("him"), py_tuple(vec![py_str("he"), py_str("him"), py_str("his")])), (py_str("his"), py_tuple(vec![py_str("he"), py_str("him"), py_str("his")])), (py_str("she"), py_tuple(vec![py_str("she"), py_str("her"), py_str("her")])), (py_str("her"), py_tuple(vec![py_str("she"), py_str("her"), py_str("her")])), (py_str("shes"), py_tuple(vec![py_str("she"), py_str("her"), py_str("her")])), (py_str("hes"), py_tuple(vec![py_str("he"), py_str("him"), py_str("his")])), (py_str("theyre"), py_tuple(vec![py_str("they"), py_str("them"), py_str("their")])), (py_str("they"), py_tuple(vec![py_str("they"), py_str("them"), py_str("their")])), (py_str("them"), py_tuple(vec![py_str("they"), py_str("them"), py_str("their")])), (py_str("their"), py_tuple(vec![py_str("they"), py_str("them"), py_str("their")]))]));
    s_THEIR_FEELING_TOWARD_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was|are|were|has|have|had|does|do|did|why)\\b[^?]{0,45}?\\b(?:annoyed|angry|upset|mad|cross|bothered|proud|disappointed|ashamed|embarrassed|jealous|bored|fed\\s+up|gone\\s+off|gone\\s+quiet|gone\\s+cold|stopped\\s+talking|avoiding|ignoring|cheating|done\\s+with\\s+me|over\\s+me|hate|hates|dislike|dislikes|think|thinks|talk(?:ing)?\\s+to\\s+me)\\b[^?]{0,30}?\\b(?:me|us)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_TOWARD_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhy\\b[^?]{0,40}?\\b(?:not\\s+invite|invite|text|call|visit|ignore|avoid|leave\\s+me|shut\\s+me\\s+out|cancel|forget|forgot|blank\\s+me|quiet\\s+with|off\\s+with|cold\\s+with|short\\s+with|distant)\\b[^?]{0,20}?\\b(?:me|us)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_WITHDRAWN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhy\\b[^?]{0,30}?\\b(?:so|being|been|suddenly|always|acting|gone|getting)\\s+(?:distant|quiet|cold|off|short|snappy|withdrawn|weird|strange|funny)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEBODY_ELSES_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:his|her|their)\\s+\\w+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AM_I_TOO_MUCH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*am\\s+i\\s+(?:being\\s+)?(?:too\\s+)?(?:annoying|boring|clingy|much|needy|pushy|a\\s+lot|hard\\s+work|too\\s+much)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_SUSPICION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was)\\s+(?:he|she|they|(?:my|our)\\s+\\w+)\\s+(?:seeing|cheating\\s+on|two-timing|lying\\s+to)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_FEELING_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should|shall|how\\s+do\\s+i|how\\s+can\\s+i|what\\s+do\\s+i\\s+do|what\\s+should\\s+i|hit|hits|hitting|hurt|hurts|hitting|shout\\w*|scream\\w*|push\\w*|grab\\w*|threat\\w*|abuse\\w*|touch\\w*|assault\\w*|rape\\w*|died|dying|death|passed\\s+away|funeral|dog|dogs|cat|cats|puppy|puppies|kitten|pet|pets|horse|rabbit|hamster)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEY_FEEL_ABOUT_ME(py_tuple(vec![py_str("Only {who} can tell you that, and guessing at it makes it worse. Ask once, plainly - 'you seem off with me, have I done something?' What has changed lately?"), py_str("That answer is in {poss} head and nowhere else, so I would not guess at it. Ask {them} straight out rather than reading the gaps; being asked is not the same as being accused. What has you wondering?"), py_str("Nobody outside that conversation can settle it. What has {who} done - or stopped doing?")]));
    s_WHY_THEY_DID_THAT(py_tuple(vec![py_str("The reason is in {poss} head, not on a page. Ask {them} once and in one sentence - no list of charges - and leave room for an answer you did not expect. What do you think it was?"), py_str("I can't know that, and the guess you make up in the gap is usually the unkindest one. Ask {them} straight out. What has you asking now?"), py_str("Only {who} knows why. Ask {them} plainly, and be ready for an answer you did not want. What has you asking now?")]));
    s_A_SUSPICION_LINES(py_tuple(vec![py_str("That is not something I can see or check, and I would not guess at it. What has made you wonder?"), py_str("I can't know what is going on there, and guessing will hurt more than asking. Is there something specific that has changed?"), py_str("That one is between the two of you - I have none of it in front of me. What happened that has you asking?")]));
}
pub fn _a_judgement_i_am_asked_for(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut possessive: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut them: Py = Py::None;
    let _ = py_str("The pool for a person asking the bot's judgement, or None when it is another question.\n\n    Asked last, on the lines nothing else answered, so a question with an answer of its own - how\n    to do the thing, who can settle it, whether it is worth the money - keeps its own reply.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_MY_VIEW(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_VERDICT_ASKED_FOR(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_MILD_GRADE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AM_I_TOO_OLD(), "match", args))(&__args) } } } } } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bshould\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PERMISSION_ASKED(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JUDGEMENT_WORDS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AM_I_TOO_OLD(), "match", args))(&__args) } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AM_I_TOO_OLD(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_ELSE_JUDGED(), "search", args))(&__args) })) } }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_IT_IS_TOO_OLD_FOR(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AGE_IS_THE_RULE(), "search", args))(&__args) })) } }) {
    return g_TOO_OLD_FOR_IT();
    }
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat\\s+i\\s+(?:said|did|wrote|sent|texted|told)\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g_JUDGE_MYSELF();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GIFT_OR_TASTE_ASKED(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_ELSE_JUDGED(), "search", args))(&__args) } } }) {
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_person(args))(&__args) }, 3, None);
    subject = __unpk1[0].clone();
    them = __unpk1[1].clone();
    possessive = __unpk1[2].clone();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_JUDGE_THEIR_TASTE()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{poss}")); __args.push(possessive.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{them}")); __args.push(them.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{who}")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THE_TWO_OF_US(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_ELSE_JUDGED(), "search", args))(&__args) })) } }) {
    return g_JUDGE_THE_TWO_OF_US();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_ELSE_JUDGED(), "search", args))(&__args) }) {
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_NUMBER_IN_THE_LINE(), "search", args))(&__args) }) { g_JUDGE_SOMEONE_AT_AN_AGE() } else { g_JUDGE_SOMEONE_ELSE() };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THING_OF_THEIRS(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ABOUT_THE_ASKER(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_ROW_IN_THE_LINE(), "search", args))(&__args) }) { g_JUDGE_MYSELF() } else { g_JUDGE_MYSELF_NO_ROW() };
    Py::None
}
pub fn _said_person(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut through: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The three forms of the person the line is about: subject, object, possessive.");
    through = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_PERSON_THROUGH_ANOTHER(), "search", args))(&__args) };
    if py_truthy(&through.clone()) {
    who = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&through.clone(), "group", args))(&__args) };
    return py_tuple(vec![who.clone(), who.clone(), op_add(&who.clone(), &py_str("'s"))]);
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_PERSON_WORD(), "search", args))(&__args) };
    if py_truthy(&named.clone()) {
    who = op_add(&py_str("your "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "group", args))(&__args) });
    return py_tuple(vec![who.clone(), who.clone(), op_add(&who.clone(), &py_str("'s"))]);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _feeling_subject(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_tuple(vec![py_str("they"), py_str("them"), py_str("their")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PERSON_FORMS(), "get", args))(&__args) };
    Py::None
}
pub fn _a_feeling_toward_me(args: &[Py]) -> Py {
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
    let mut fill: Py = Py::None;
    let mut possessive: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut them: Py = Py::None;
    let mut withdrawn: Py = Py::None;
    let _ = py_str("The pool for how one of their people feels about them, or why they did that; or None.");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_FEELING_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEBODY_ELSES_PERSON(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    withdrawn = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHY_WITHDRAWN(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_PERSON_WORD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|they|him|her)\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_person(args))(&__args) }, 3, None);
    subject = __unpk1[0].clone();
    them = __unpk1[1].clone();
    possessive = __unpk1[2].clone();
    fill = { let clean = clean.clone(); let fill = fill.clone(); let possessive = possessive.clone(); let subject = subject.clone(); let them = them.clone(); let withdrawn = withdrawn.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let fill = fill.clone(); let possessive = possessive.clone(); let subject = subject.clone(); let them = them.clone(); let withdrawn = withdrawn.clone(); let pool: Py = __kwp(args, 0, "pool").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{poss}")); __args.push(possessive.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{them}")); __args.push(them.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{who}")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } }) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SUSPICION(), "search", args))(&__args) }) {
    return g_A_SUSPICION_LINES();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHY_TOWARD_ME(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { withdrawn.clone() } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_THEY_DID_THAT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&fill, args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AM_I_TOO_MUCH(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_FEELING_TOWARD_ME(), "search", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEY_FEEL_ABOUT_ME()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&fill, args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _feeling_subject(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b[a-z']+\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__PERSON_FORMS(), &word.clone()))) {
    return word.clone();
    }
    }
    return py_str("they");
    Py::None
}
pub fn _a_fact_about_their_person(args: &[Py]) -> Py {
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
    let mut _them_said: Py = Py::None;
    let mut _who_said: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut object_word: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut possessive: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut wh: Py = Py::None;
    let _ = py_str("The pool for a question only the asker can answer about their own people, or None.");
    place = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLACE_ABOUT_THEIR_PERSON(), "match", args))(&__args) };
    asked = { let __b = place.clone(); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FACT_ABOUT_THEIR_PERSON(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WH_ABOUT_THEIR_PERSON(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STATE_ABOUT_THEIR_PERSON(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FIGURE_ABOUT_THEIR_PERSON(), "match", args))(&__args) } } } } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&place.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_FACT_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BODY_OR_ILLNESS(), "search", args))(&__args) } } } } }) {
    return Py::None;
    }
    wh = Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FACT_ABOUT_THEIR_PERSON(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STATE_ABOUT_THEIR_PERSON(), "match", args))(&__args) } } }));
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_person(args))(&__args) }, 3, None);
    _who_said = __unpk1[0].clone();
    _them_said = __unpk1[1].clone();
    possessive = __unpk1[2].clone();
    found = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_PERSON_WORD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_PERSON_THROUGH_ANOTHER(), "search", args))(&__args) } } };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) };
    object_word = if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&gattr(&found.clone(), "re"), &g_THEIR_PERSON_THROUGH_ANOTHER())) } }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) } } else { if py_truthy(&found.clone()) { op_add(&py_str("your "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }) } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_str("them")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PERSON_OBJECT(), "get", args))(&__args) } } };
    subject = if py_truthy(&found.clone()) { object_word.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PERSON_SUBJECT(), "get", args))(&__args) } };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABOUT_ME_ASKED(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THEIR_FEELINGS_LINES()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{object}")); __args.push(object_word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{subject}")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FIGURE_ABOUT_THEIR_PERSON(), "match", args))(&__args) }) {
    pool = g_THEIR_PERSON_FIGURE_LINES();
    } else {
    if py_truthy(&wh.clone()) {
    pool = g_THEIR_PERSON_WH_LINES();
    } else {
    pool = g_THEIR_PERSON_LINES();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{poss}")); __args.push(possessive.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{them}")); __args.push(object_word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{who}")); __args.push(object_word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{she}")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A reply for a judgement put to the bot, or a fact about one of their own people; or None.\n\n    Called with the composer's \"I cannot answer that\" reply already in hand, so anything this\n    returns is replacing a hedge rather than an answer. The reading is made on the clean line the\n    composer read - as typed, so a word the person put in caps still reads - and on the repaired\n    line after it, so \"does my mum lyk golf\" gets the same reply as the spelling right.\n    ");
    understood = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    understood = py_dict(vec![]);
    }
    clean = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } } };
    words = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "replace", args))(&__args) }, "split", args))(&__args) } } };
    repaired = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    pool = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_judgement_i_am_asked_for(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_fact_about_their_person(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_feeling_toward_me(args))(&__args) } } } } };
    if py_truthy(&pool.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.only_they_know".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_ME_ASKED" => Some(g_ABOUT_ME_ASKED()),
            "AGE_IS_THE_RULE" => Some(g_AGE_IS_THE_RULE()),
            "AM_I_TOO_MUCH" => Some(g_AM_I_TOO_MUCH()),
            "AM_I_TOO_OLD" => Some(g_AM_I_TOO_OLD()),
            "ASKS_MY_VIEW" => Some(g_ASKS_MY_VIEW()),
            "A_BODY_OR_ILLNESS" => Some(g_A_BODY_OR_ILLNESS()),
            "A_FACT_ABOUT_THEIR_PERSON" => Some(g_A_FACT_ABOUT_THEIR_PERSON()),
            "A_FIGURE_ABOUT_THEIR_PERSON" => Some(g_A_FIGURE_ABOUT_THEIR_PERSON()),
            "A_GIFT_OR_TASTE_ASKED" => Some(g_A_GIFT_OR_TASTE_ASKED()),
            "A_MILD_GRADE" => Some(g_A_MILD_GRADE()),
            "A_NUMBER_IN_THE_LINE" => Some(g_A_NUMBER_IN_THE_LINE()),
            "A_PLACE_ABOUT_THEIR_PERSON" => Some(g_A_PLACE_ABOUT_THEIR_PERSON()),
            "A_ROW_IN_THE_LINE" => Some(g_A_ROW_IN_THE_LINE()),
            "A_STATE_ABOUT_THEIR_PERSON" => Some(g_A_STATE_ABOUT_THEIR_PERSON()),
            "A_SUSPICION" => Some(g_A_SUSPICION()),
            "A_SUSPICION_LINES" => Some(g_A_SUSPICION_LINES()),
            "A_THING_IT_IS_TOO_OLD_FOR" => Some(g_A_THING_IT_IS_TOO_OLD_FOR()),
            "A_VERDICT_ASKED_FOR" => Some(g_A_VERDICT_ASKED_FOR()),
            "A_WH_ABOUT_THEIR_PERSON" => Some(g_A_WH_ABOUT_THEIR_PERSON()),
            "EVERYDAY_VERBS" => Some(g_EVERYDAY_VERBS()),
            "JUDGEMENT_WORDS" => Some(g_JUDGEMENT_WORDS()),
            "JUDGE_MYSELF" => Some(g_JUDGE_MYSELF()),
            "JUDGE_MYSELF_NO_ROW" => Some(g_JUDGE_MYSELF_NO_ROW()),
            "JUDGE_SOMEONE_AT_AN_AGE" => Some(g_JUDGE_SOMEONE_AT_AN_AGE()),
            "JUDGE_SOMEONE_ELSE" => Some(g_JUDGE_SOMEONE_ELSE()),
            "JUDGE_THEIR_TASTE" => Some(g_JUDGE_THEIR_TASTE()),
            "JUDGE_THE_TWO_OF_US" => Some(g_JUDGE_THE_TWO_OF_US()),
            "NOT_A_FACT_ASK" => Some(g_NOT_A_FACT_ASK()),
            "NOT_A_FEELING_ASK" => Some(g_NOT_A_FEELING_ASK()),
            "PERMISSION_ASKED" => Some(g_PERMISSION_ASKED()),
            "SOMEBODY_ELSES_PERSON" => Some(g_SOMEBODY_ELSES_PERSON()),
            "SOMEONE_ELSE_JUDGED" => Some(g_SOMEONE_ELSE_JUDGED()),
            "THEIR_FEELINGS_LINES" => Some(g_THEIR_FEELINGS_LINES()),
            "THEIR_FEELING_TOWARD_ME" => Some(g_THEIR_FEELING_TOWARD_ME()),
            "THEIR_PERSON_FIGURE_LINES" => Some(g_THEIR_PERSON_FIGURE_LINES()),
            "THEIR_PERSON_LINES" => Some(g_THEIR_PERSON_LINES()),
            "THEIR_PERSON_THROUGH_ANOTHER" => Some(g_THEIR_PERSON_THROUGH_ANOTHER()),
            "THEIR_PERSON_WH_LINES" => Some(g_THEIR_PERSON_WH_LINES()),
            "THEIR_PERSON_WORD" => Some(g_THEIR_PERSON_WORD()),
            "THEY_FEEL_ABOUT_ME" => Some(g_THEY_FEEL_ABOUT_ME()),
            "TOO_OLD_FOR_IT" => Some(g_TOO_OLD_FOR_IT()),
            "WHY_THEY_DID_THAT" => Some(g_WHY_THEY_DID_THAT()),
            "WHY_TOWARD_ME" => Some(g_WHY_TOWARD_ME()),
            "WHY_WITHDRAWN" => Some(g_WHY_WITHDRAWN()),
            "_ABOUT_THE_ASKER" => Some(g__ABOUT_THE_ASKER()),
            "_ANY_PERSON" => Some(g__ANY_PERSON()),
            "_PERSON_FORMS" => Some(g__PERSON_FORMS()),
            "_PERSON_OBJECT" => Some(g__PERSON_OBJECT()),
            "_PERSON_SUBJECT" => Some(g__PERSON_SUBJECT()),
            "_RELATION" => Some(g__RELATION()),
            "_THE_TWO_OF_US" => Some(g__THE_TWO_OF_US()),
            "_THING_OF_THEIRS" => Some(g__THING_OF_THEIRS()),
            "_a_fact_about_their_person" => Some(py_func("_a_fact_about_their_person", |args: &[Py]| _a_fact_about_their_person(args))),
            "_a_feeling_toward_me" => Some(py_func("_a_feeling_toward_me", |args: &[Py]| _a_feeling_toward_me(args))),
            "_a_judgement_i_am_asked_for" => Some(py_func("_a_judgement_i_am_asked_for", |args: &[Py]| _a_judgement_i_am_asked_for(args))),
            "_feeling_subject" => Some(py_func("_feeling_subject", |args: &[Py]| _feeling_subject(args))),
            "_said_person" => Some(py_func("_said_person", |args: &[Py]| _said_person(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_ME_ASKED" => s_ABOUT_ME_ASKED(v),
            "AGE_IS_THE_RULE" => s_AGE_IS_THE_RULE(v),
            "AM_I_TOO_MUCH" => s_AM_I_TOO_MUCH(v),
            "AM_I_TOO_OLD" => s_AM_I_TOO_OLD(v),
            "ASKS_MY_VIEW" => s_ASKS_MY_VIEW(v),
            "A_BODY_OR_ILLNESS" => s_A_BODY_OR_ILLNESS(v),
            "A_FACT_ABOUT_THEIR_PERSON" => s_A_FACT_ABOUT_THEIR_PERSON(v),
            "A_FIGURE_ABOUT_THEIR_PERSON" => s_A_FIGURE_ABOUT_THEIR_PERSON(v),
            "A_GIFT_OR_TASTE_ASKED" => s_A_GIFT_OR_TASTE_ASKED(v),
            "A_MILD_GRADE" => s_A_MILD_GRADE(v),
            "A_NUMBER_IN_THE_LINE" => s_A_NUMBER_IN_THE_LINE(v),
            "A_PLACE_ABOUT_THEIR_PERSON" => s_A_PLACE_ABOUT_THEIR_PERSON(v),
            "A_ROW_IN_THE_LINE" => s_A_ROW_IN_THE_LINE(v),
            "A_STATE_ABOUT_THEIR_PERSON" => s_A_STATE_ABOUT_THEIR_PERSON(v),
            "A_SUSPICION" => s_A_SUSPICION(v),
            "A_SUSPICION_LINES" => s_A_SUSPICION_LINES(v),
            "A_THING_IT_IS_TOO_OLD_FOR" => s_A_THING_IT_IS_TOO_OLD_FOR(v),
            "A_VERDICT_ASKED_FOR" => s_A_VERDICT_ASKED_FOR(v),
            "A_WH_ABOUT_THEIR_PERSON" => s_A_WH_ABOUT_THEIR_PERSON(v),
            "EVERYDAY_VERBS" => s_EVERYDAY_VERBS(v),
            "JUDGEMENT_WORDS" => s_JUDGEMENT_WORDS(v),
            "JUDGE_MYSELF" => s_JUDGE_MYSELF(v),
            "JUDGE_MYSELF_NO_ROW" => s_JUDGE_MYSELF_NO_ROW(v),
            "JUDGE_SOMEONE_AT_AN_AGE" => s_JUDGE_SOMEONE_AT_AN_AGE(v),
            "JUDGE_SOMEONE_ELSE" => s_JUDGE_SOMEONE_ELSE(v),
            "JUDGE_THEIR_TASTE" => s_JUDGE_THEIR_TASTE(v),
            "JUDGE_THE_TWO_OF_US" => s_JUDGE_THE_TWO_OF_US(v),
            "NOT_A_FACT_ASK" => s_NOT_A_FACT_ASK(v),
            "NOT_A_FEELING_ASK" => s_NOT_A_FEELING_ASK(v),
            "PERMISSION_ASKED" => s_PERMISSION_ASKED(v),
            "SOMEBODY_ELSES_PERSON" => s_SOMEBODY_ELSES_PERSON(v),
            "SOMEONE_ELSE_JUDGED" => s_SOMEONE_ELSE_JUDGED(v),
            "THEIR_FEELINGS_LINES" => s_THEIR_FEELINGS_LINES(v),
            "THEIR_FEELING_TOWARD_ME" => s_THEIR_FEELING_TOWARD_ME(v),
            "THEIR_PERSON_FIGURE_LINES" => s_THEIR_PERSON_FIGURE_LINES(v),
            "THEIR_PERSON_LINES" => s_THEIR_PERSON_LINES(v),
            "THEIR_PERSON_THROUGH_ANOTHER" => s_THEIR_PERSON_THROUGH_ANOTHER(v),
            "THEIR_PERSON_WH_LINES" => s_THEIR_PERSON_WH_LINES(v),
            "THEIR_PERSON_WORD" => s_THEIR_PERSON_WORD(v),
            "THEY_FEEL_ABOUT_ME" => s_THEY_FEEL_ABOUT_ME(v),
            "TOO_OLD_FOR_IT" => s_TOO_OLD_FOR_IT(v),
            "WHY_THEY_DID_THAT" => s_WHY_THEY_DID_THAT(v),
            "WHY_TOWARD_ME" => s_WHY_TOWARD_ME(v),
            "WHY_WITHDRAWN" => s_WHY_WITHDRAWN(v),
            "_ABOUT_THE_ASKER" => s__ABOUT_THE_ASKER(v),
            "_ANY_PERSON" => s__ANY_PERSON(v),
            "_PERSON_FORMS" => s__PERSON_FORMS(v),
            "_PERSON_OBJECT" => s__PERSON_OBJECT(v),
            "_PERSON_SUBJECT" => s__PERSON_SUBJECT(v),
            "_RELATION" => s__RELATION(v),
            "_THE_TWO_OF_US" => s__THE_TWO_OF_US(v),
            "_THING_OF_THEIRS" => s__THING_OF_THEIRS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.only_they_know", module_obj);
    pyrt::xrefs::reg_fn("compose.only_they_know", "_a_judgement_i_am_asked_for", py_func("_a_judgement_i_am_asked_for", |args: &[Py]| _a_judgement_i_am_asked_for(args)));
    pyrt::xrefs::reg_fn("compose.only_they_know", "_said_person", py_func("_said_person", |args: &[Py]| _said_person(args)));
    pyrt::xrefs::reg_fn("compose.only_they_know", "_a_feeling_toward_me", py_func("_a_feeling_toward_me", |args: &[Py]| _a_feeling_toward_me(args)));
    pyrt::xrefs::reg_fn("compose.only_they_know", "_feeling_subject", py_func("_feeling_subject", |args: &[Py]| _feeling_subject(args)));
    pyrt::xrefs::reg_fn("compose.only_they_know", "_a_fact_about_their_person", py_func("_a_fact_about_their_person", |args: &[Py]| _a_fact_about_their_person(args)));
    pyrt::xrefs::reg_fn("compose.only_they_know", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.only_they_know", "ABOUT_ME_ASKED", g_ABOUT_ME_ASKED, s_ABOUT_ME_ASKED);
    pyrt::xrefs::reg_access("compose.only_they_know", "AGE_IS_THE_RULE", g_AGE_IS_THE_RULE, s_AGE_IS_THE_RULE);
    pyrt::xrefs::reg_access("compose.only_they_know", "AM_I_TOO_MUCH", g_AM_I_TOO_MUCH, s_AM_I_TOO_MUCH);
    pyrt::xrefs::reg_access("compose.only_they_know", "AM_I_TOO_OLD", g_AM_I_TOO_OLD, s_AM_I_TOO_OLD);
    pyrt::xrefs::reg_access("compose.only_they_know", "ASKS_MY_VIEW", g_ASKS_MY_VIEW, s_ASKS_MY_VIEW);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_BODY_OR_ILLNESS", g_A_BODY_OR_ILLNESS, s_A_BODY_OR_ILLNESS);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_FACT_ABOUT_THEIR_PERSON", g_A_FACT_ABOUT_THEIR_PERSON, s_A_FACT_ABOUT_THEIR_PERSON);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_FIGURE_ABOUT_THEIR_PERSON", g_A_FIGURE_ABOUT_THEIR_PERSON, s_A_FIGURE_ABOUT_THEIR_PERSON);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_GIFT_OR_TASTE_ASKED", g_A_GIFT_OR_TASTE_ASKED, s_A_GIFT_OR_TASTE_ASKED);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_MILD_GRADE", g_A_MILD_GRADE, s_A_MILD_GRADE);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_NUMBER_IN_THE_LINE", g_A_NUMBER_IN_THE_LINE, s_A_NUMBER_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_PLACE_ABOUT_THEIR_PERSON", g_A_PLACE_ABOUT_THEIR_PERSON, s_A_PLACE_ABOUT_THEIR_PERSON);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_ROW_IN_THE_LINE", g_A_ROW_IN_THE_LINE, s_A_ROW_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_STATE_ABOUT_THEIR_PERSON", g_A_STATE_ABOUT_THEIR_PERSON, s_A_STATE_ABOUT_THEIR_PERSON);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_SUSPICION", g_A_SUSPICION, s_A_SUSPICION);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_SUSPICION_LINES", g_A_SUSPICION_LINES, s_A_SUSPICION_LINES);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_THING_IT_IS_TOO_OLD_FOR", g_A_THING_IT_IS_TOO_OLD_FOR, s_A_THING_IT_IS_TOO_OLD_FOR);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_VERDICT_ASKED_FOR", g_A_VERDICT_ASKED_FOR, s_A_VERDICT_ASKED_FOR);
    pyrt::xrefs::reg_access("compose.only_they_know", "A_WH_ABOUT_THEIR_PERSON", g_A_WH_ABOUT_THEIR_PERSON, s_A_WH_ABOUT_THEIR_PERSON);
    pyrt::xrefs::reg_access("compose.only_they_know", "EVERYDAY_VERBS", g_EVERYDAY_VERBS, s_EVERYDAY_VERBS);
    pyrt::xrefs::reg_access("compose.only_they_know", "JUDGEMENT_WORDS", g_JUDGEMENT_WORDS, s_JUDGEMENT_WORDS);
    pyrt::xrefs::reg_access("compose.only_they_know", "JUDGE_MYSELF", g_JUDGE_MYSELF, s_JUDGE_MYSELF);
    pyrt::xrefs::reg_access("compose.only_they_know", "JUDGE_MYSELF_NO_ROW", g_JUDGE_MYSELF_NO_ROW, s_JUDGE_MYSELF_NO_ROW);
    pyrt::xrefs::reg_access("compose.only_they_know", "JUDGE_SOMEONE_AT_AN_AGE", g_JUDGE_SOMEONE_AT_AN_AGE, s_JUDGE_SOMEONE_AT_AN_AGE);
    pyrt::xrefs::reg_access("compose.only_they_know", "JUDGE_SOMEONE_ELSE", g_JUDGE_SOMEONE_ELSE, s_JUDGE_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("compose.only_they_know", "JUDGE_THEIR_TASTE", g_JUDGE_THEIR_TASTE, s_JUDGE_THEIR_TASTE);
    pyrt::xrefs::reg_access("compose.only_they_know", "JUDGE_THE_TWO_OF_US", g_JUDGE_THE_TWO_OF_US, s_JUDGE_THE_TWO_OF_US);
    pyrt::xrefs::reg_access("compose.only_they_know", "NOT_A_FACT_ASK", g_NOT_A_FACT_ASK, s_NOT_A_FACT_ASK);
    pyrt::xrefs::reg_access("compose.only_they_know", "NOT_A_FEELING_ASK", g_NOT_A_FEELING_ASK, s_NOT_A_FEELING_ASK);
    pyrt::xrefs::reg_access("compose.only_they_know", "PERMISSION_ASKED", g_PERMISSION_ASKED, s_PERMISSION_ASKED);
    pyrt::xrefs::reg_access("compose.only_they_know", "SOMEBODY_ELSES_PERSON", g_SOMEBODY_ELSES_PERSON, s_SOMEBODY_ELSES_PERSON);
    pyrt::xrefs::reg_access("compose.only_they_know", "SOMEONE_ELSE_JUDGED", g_SOMEONE_ELSE_JUDGED, s_SOMEONE_ELSE_JUDGED);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEIR_FEELINGS_LINES", g_THEIR_FEELINGS_LINES, s_THEIR_FEELINGS_LINES);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEIR_FEELING_TOWARD_ME", g_THEIR_FEELING_TOWARD_ME, s_THEIR_FEELING_TOWARD_ME);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEIR_PERSON_FIGURE_LINES", g_THEIR_PERSON_FIGURE_LINES, s_THEIR_PERSON_FIGURE_LINES);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEIR_PERSON_LINES", g_THEIR_PERSON_LINES, s_THEIR_PERSON_LINES);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEIR_PERSON_THROUGH_ANOTHER", g_THEIR_PERSON_THROUGH_ANOTHER, s_THEIR_PERSON_THROUGH_ANOTHER);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEIR_PERSON_WH_LINES", g_THEIR_PERSON_WH_LINES, s_THEIR_PERSON_WH_LINES);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEIR_PERSON_WORD", g_THEIR_PERSON_WORD, s_THEIR_PERSON_WORD);
    pyrt::xrefs::reg_access("compose.only_they_know", "THEY_FEEL_ABOUT_ME", g_THEY_FEEL_ABOUT_ME, s_THEY_FEEL_ABOUT_ME);
    pyrt::xrefs::reg_access("compose.only_they_know", "TOO_OLD_FOR_IT", g_TOO_OLD_FOR_IT, s_TOO_OLD_FOR_IT);
    pyrt::xrefs::reg_access("compose.only_they_know", "WHY_THEY_DID_THAT", g_WHY_THEY_DID_THAT, s_WHY_THEY_DID_THAT);
    pyrt::xrefs::reg_access("compose.only_they_know", "WHY_TOWARD_ME", g_WHY_TOWARD_ME, s_WHY_TOWARD_ME);
    pyrt::xrefs::reg_access("compose.only_they_know", "WHY_WITHDRAWN", g_WHY_WITHDRAWN, s_WHY_WITHDRAWN);
    pyrt::xrefs::reg_access("compose.only_they_know", "_ABOUT_THE_ASKER", g__ABOUT_THE_ASKER, s__ABOUT_THE_ASKER);
    pyrt::xrefs::reg_access("compose.only_they_know", "_ANY_PERSON", g__ANY_PERSON, s__ANY_PERSON);
    pyrt::xrefs::reg_access("compose.only_they_know", "_PERSON_FORMS", g__PERSON_FORMS, s__PERSON_FORMS);
    pyrt::xrefs::reg_access("compose.only_they_know", "_PERSON_OBJECT", g__PERSON_OBJECT, s__PERSON_OBJECT);
    pyrt::xrefs::reg_access("compose.only_they_know", "_PERSON_SUBJECT", g__PERSON_SUBJECT, s__PERSON_SUBJECT);
    pyrt::xrefs::reg_access("compose.only_they_know", "_RELATION", g__RELATION, s__RELATION);
    pyrt::xrefs::reg_access("compose.only_they_know", "_THE_TWO_OF_US", g__THE_TWO_OF_US, s__THE_TWO_OF_US);
    pyrt::xrefs::reg_access("compose.only_they_know", "_THING_OF_THEIRS", g__THING_OF_THEIRS, s__THING_OF_THEIRS);
}
