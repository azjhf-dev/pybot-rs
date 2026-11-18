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


thread_local! { static G_ABOUT_THE_BOT_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACT_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ACT_NOW_SOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_THE_MARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGAIN_SHORTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGAIN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AND_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AND_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWER_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GENERAL_CLAIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOD_IS_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOD_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACEHOLDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NAMED_BEFORE_A_COLON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_WHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSING_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_BEFORE_COLON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITIONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITIONAL_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DANGLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENDS_THE_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_END_OF_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXAMPLE_IN_QUOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPLAINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_SENTENCES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEDGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IMPERATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSTRUCTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_CAPITAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADING_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_CUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENS_ON_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESULT_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_WHAT_IT_DOES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_OF_A_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SO_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SO_FROM_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SO_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBSTANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUGGESTED_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_DECISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_SUGGESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BOT_DESCRIBED_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_REASON_POINTS_AT_THE_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THINKS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_IT_TAKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_ANSWERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_NAMED_BECAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_NAMED_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_NOT_ANSWERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_NOT_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY_YOU_SAID_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRONG_ANSWERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRONG_NOT_A_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANNOUNCES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CATEGORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CAUSE_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLAUSE_LEAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DOING_WINDOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LONG_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PARTICIPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_REASON_LEAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SAVE_LEAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STRIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXPLAINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEANS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEANS_BACK_WINDOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEEDS_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_ON_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_FORWARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_FORWARD_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_FORWARD_LEAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAYS_HOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERB_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_THE_BOT_LINE() -> Py { ensure_init(); G_ABOUT_THE_BOT_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_BOT_LINE(v: Py) { ensure_init(); G_ABOUT_THE_BOT_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ACT_NOW() -> Py { ensure_init(); G_ACT_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACT_NOW(v: Py) { ensure_init(); G_ACT_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_ACT_NOW_SOON() -> Py { ensure_init(); G_ACT_NOW_SOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACT_NOW_SOON(v: Py) { ensure_init(); G_ACT_NOW_SOON.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_THE_MARK() -> Py { ensure_init(); G_AFTER_THE_MARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_MARK(v: Py) { ensure_init(); G_AFTER_THE_MARK.with(|c| *c.borrow_mut() = v); }
pub fn g_AGAIN_SHORTER() -> Py { ensure_init(); G_AGAIN_SHORTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN_SHORTER(v: Py) { ensure_init(); G_AGAIN_SHORTER.with(|c| *c.borrow_mut() = v); }
pub fn g_AGAIN_WORDS() -> Py { ensure_init(); G_AGAIN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN_WORDS(v: Py) { ensure_init(); G_AGAIN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AND_ADVICE() -> Py { ensure_init(); G_AND_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AND_ADVICE(v: Py) { ensure_init(); G_AND_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_AND_STEP() -> Py { ensure_init(); G_AND_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AND_STEP(v: Py) { ensure_init(); G_AND_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERED() -> Py { ensure_init(); G_ANSWERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERED(v: Py) { ensure_init(); G_ANSWERED.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWER_LONG() -> Py { ensure_init(); G_ANSWER_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWER_LONG(v: Py) { ensure_init(); G_ANSWER_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GENERAL_CLAIM() -> Py { ensure_init(); G_A_GENERAL_CLAIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GENERAL_CLAIM(v: Py) { ensure_init(); G_A_GENERAL_CLAIM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LEAD_IN() -> Py { ensure_init(); G_A_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEAD_IN(v: Py) { ensure_init(); G_A_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOD_IS_SHORT() -> Py { ensure_init(); G_A_NOD_IS_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOD_IS_SHORT(v: Py) { ensure_init(); G_A_NOD_IS_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOD_OPENS() -> Py { ensure_init(); G_A_NOD_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOD_OPENS(v: Py) { ensure_init(); G_A_NOD_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACEHOLDER() -> Py { ensure_init(); G_A_PLACEHOLDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACEHOLDER(v: Py) { ensure_init(); G_A_PLACEHOLDER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NAMED_BEFORE_A_COLON() -> Py { ensure_init(); G_A_THING_NAMED_BEFORE_A_COLON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED_BEFORE_A_COLON(v: Py) { ensure_init(); G_A_THING_NAMED_BEFORE_A_COLON.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_WHY() -> Py { ensure_init(); G_BARE_WHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_WHY(v: Py) { ensure_init(); G_BARE_WHY.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_ANSWER() -> Py { ensure_init(); G_CARE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_ANSWER(v: Py) { ensure_init(); G_CARE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSE() -> Py { ensure_init(); G_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSE(v: Py) { ensure_init(); G_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSING_IT() -> Py { ensure_init(); G_CAUSING_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSING_IT(v: Py) { ensure_init(); G_CAUSING_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE() -> Py { ensure_init(); G_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE(v: Py) { ensure_init(); G_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_BEFORE_COLON() -> Py { ensure_init(); G_CLAIM_BEFORE_COLON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_BEFORE_COLON(v: Py) { ensure_init(); G_CLAIM_BEFORE_COLON.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_SUBJECT() -> Py { ensure_init(); G_CLAIM_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_SUBJECT(v: Py) { ensure_init(); G_CLAIM_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_END() -> Py { ensure_init(); G_CLAUSE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_END(v: Py) { ensure_init(); G_CLAUSE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITIONAL() -> Py { ensure_init(); G_CONDITIONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL(v: Py) { ensure_init(); G_CONDITIONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITIONAL_SENTENCE() -> Py { ensure_init(); G_CONDITIONAL_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL_SENTENCE(v: Py) { ensure_init(); G_CONDITIONAL_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_DANGLING() -> Py { ensure_init(); G_DANGLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DANGLING(v: Py) { ensure_init(); G_DANGLING.with(|c| *c.borrow_mut() = v); }
pub fn g_DOSE() -> Py { ensure_init(); G_DOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOSE(v: Py) { ensure_init(); G_DOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ENDS_THE_CLASS() -> Py { ensure_init(); G_ENDS_THE_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENDS_THE_CLASS(v: Py) { ensure_init(); G_ENDS_THE_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_END_OF_SENTENCE() -> Py { ensure_init(); G_END_OF_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_END_OF_SENTENCE(v: Py) { ensure_init(); G_END_OF_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_EXAMPLE_IN_QUOTES() -> Py { ensure_init(); G_EXAMPLE_IN_QUOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXAMPLE_IN_QUOTES(v: Py) { ensure_init(); G_EXAMPLE_IN_QUOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPLAINS() -> Py { ensure_init(); G_EXPLAINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPLAINS(v: Py) { ensure_init(); G_EXPLAINS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_SENTENCES() -> Py { ensure_init(); G_FIRST_SENTENCES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_SENTENCES(v: Py) { ensure_init(); G_FIRST_SENTENCES.with(|c| *c.borrow_mut() = v); }
pub fn g_HEDGED() -> Py { ensure_init(); G_HEDGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEDGED(v: Py) { ensure_init(); G_HEDGED.with(|c| *c.borrow_mut() = v); }
pub fn g_IMPERATIVE() -> Py { ensure_init(); G_IMPERATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IMPERATIVE(v: Py) { ensure_init(); G_IMPERATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_INSTRUCTION() -> Py { ensure_init(); G_INSTRUCTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSTRUCTION(v: Py) { ensure_init(); G_INSTRUCTION.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_CAPITAL() -> Py { ensure_init(); G_KEEP_CAPITAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_CAPITAL(v: Py) { ensure_init(); G_KEEP_CAPITAL.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADING_MARKER() -> Py { ensure_init(); G_LEADING_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADING_MARKER(v: Py) { ensure_init(); G_LEADING_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g_LIFE() -> Py { ensure_init(); G_LIFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIFE(v: Py) { ensure_init(); G_LIFE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_CAUSE() -> Py { ensure_init(); G_NAMED_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_CAUSE(v: Py) { ensure_init(); G_NAMED_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATIVE() -> Py { ensure_init(); G_NEGATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE(v: Py) { ensure_init(); G_NEGATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_CUT() -> Py { ensure_init(); G_NO_CUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_CUT(v: Py) { ensure_init(); G_NO_CUT.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENS_ON_CARE() -> Py { ensure_init(); G_OPENS_ON_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENS_ON_CARE(v: Py) { ensure_init(); G_OPENS_ON_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTS() -> Py { ensure_init(); G_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTS(v: Py) { ensure_init(); G_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_LEADS() -> Py { ensure_init(); G_REPEAT_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_LEADS(v: Py) { ensure_init(); G_REPEAT_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_RESULT_NUMBER() -> Py { ensure_init(); G_RESULT_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESULT_NUMBER(v: Py) { ensure_init(); G_RESULT_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AGAIN() -> Py { ensure_init(); G_SAID_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AGAIN(v: Py) { ensure_init(); G_SAID_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_NO() -> Py { ensure_init(); G_SAID_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_NO(v: Py) { ensure_init(); G_SAID_NO.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_WHAT_IT_DOES() -> Py { ensure_init(); G_SAYS_WHAT_IT_DOES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_WHAT_IT_DOES(v: Py) { ensure_init(); G_SAYS_WHAT_IT_DOES.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_OF_A_CAUSE() -> Py { ensure_init(); G_SHORT_OF_A_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_OF_A_CAUSE(v: Py) { ensure_init(); G_SHORT_OF_A_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SO_ADVICE() -> Py { ensure_init(); G_SO_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SO_ADVICE(v: Py) { ensure_init(); G_SO_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_SO_FROM_HERE() -> Py { ensure_init(); G_SO_FROM_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SO_FROM_HERE(v: Py) { ensure_init(); G_SO_FROM_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g_SO_REASON() -> Py { ensure_init(); G_SO_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SO_REASON(v: Py) { ensure_init(); G_SO_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_VERBS() -> Py { ensure_init(); G_STEP_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_VERBS(v: Py) { ensure_init(); G_STEP_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBSTANCE() -> Py { ensure_init(); G_SUBSTANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBSTANCE(v: Py) { ensure_init(); G_SUBSTANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SUGGESTED_VERBS() -> Py { ensure_init(); G_SUGGESTED_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUGGESTED_VERBS(v: Py) { ensure_init(); G_SUGGESTED_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_DECISION() -> Py { ensure_init(); G_THEIR_DECISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_DECISION(v: Py) { ensure_init(); G_THEIR_DECISION.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_SUGGESTION() -> Py { ensure_init(); G_THEIR_SUGGESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_SUGGESTION(v: Py) { ensure_init(); G_THEIR_SUGGESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BOT_DESCRIBED_ITSELF() -> Py { ensure_init(); G_THE_BOT_DESCRIBED_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BOT_DESCRIBED_ITSELF(v: Py) { ensure_init(); G_THE_BOT_DESCRIBED_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FIX() -> Py { ensure_init(); G_THE_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FIX(v: Py) { ensure_init(); G_THE_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_REASON_POINTS_AT_THE_ADVICE() -> Py { ensure_init(); G_THE_REASON_POINTS_AT_THE_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_REASON_POINTS_AT_THE_ADVICE(v: Py) { ensure_init(); G_THE_REASON_POINTS_AT_THE_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_THINKS_BACK() -> Py { ensure_init(); G_THINKS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THINKS_BACK(v: Py) { ensure_init(); G_THINKS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_IT_TAKES() -> Py { ensure_init(); G_WHAT_IT_TAKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_IT_TAKES(v: Py) { ensure_init(); G_WHAT_IT_TAKES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_CAUSE() -> Py { ensure_init(); G_WHEN_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_CAUSE(v: Py) { ensure_init(); G_WHEN_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_AGAIN() -> Py { ensure_init(); G_WHY_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_AGAIN(v: Py) { ensure_init(); G_WHY_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_ANSWERED() -> Py { ensure_init(); G_WHY_ANSWERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_ANSWERED(v: Py) { ensure_init(); G_WHY_ANSWERED.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_BODY() -> Py { ensure_init(); G_WHY_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_BODY(v: Py) { ensure_init(); G_WHY_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_CARE() -> Py { ensure_init(); G_WHY_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_CARE(v: Py) { ensure_init(); G_WHY_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_END() -> Py { ensure_init(); G_WHY_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_END(v: Py) { ensure_init(); G_WHY_END.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_FRONT() -> Py { ensure_init(); G_WHY_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_FRONT(v: Py) { ensure_init(); G_WHY_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_METHOD() -> Py { ensure_init(); G_WHY_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_METHOD(v: Py) { ensure_init(); G_WHY_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_NAMED() -> Py { ensure_init(); G_WHY_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_NAMED(v: Py) { ensure_init(); G_WHY_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_NAMED_BECAUSE() -> Py { ensure_init(); G_WHY_NAMED_BECAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_NAMED_BECAUSE(v: Py) { ensure_init(); G_WHY_NAMED_BECAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_NAMED_SELF() -> Py { ensure_init(); G_WHY_NAMED_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_NAMED_SELF(v: Py) { ensure_init(); G_WHY_NAMED_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_NOT() -> Py { ensure_init(); G_WHY_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_NOT(v: Py) { ensure_init(); G_WHY_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_NOT_ANSWERED() -> Py { ensure_init(); G_WHY_NOT_ANSWERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_NOT_ANSWERED(v: Py) { ensure_init(); G_WHY_NOT_ANSWERED.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_NOT_ITS_OWN() -> Py { ensure_init(); G_WHY_NOT_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_NOT_ITS_OWN(v: Py) { ensure_init(); G_WHY_NOT_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_THE_BOT() -> Py { ensure_init(); G_WHY_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_THE_BOT(v: Py) { ensure_init(); G_WHY_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY_YOU_SAID_IT() -> Py { ensure_init(); G_WHY_YOU_SAID_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY_YOU_SAID_IT(v: Py) { ensure_init(); G_WHY_YOU_SAID_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WRONG() -> Py { ensure_init(); G_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRONG(v: Py) { ensure_init(); G_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_WRONG_ANSWERED() -> Py { ensure_init(); G_WRONG_ANSWERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRONG_ANSWERED(v: Py) { ensure_init(); G_WRONG_ANSWERED.with(|c| *c.borrow_mut() = v); }
pub fn g_WRONG_NOT_A_FAULT() -> Py { ensure_init(); G_WRONG_NOT_A_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRONG_NOT_A_FAULT(v: Py) { ensure_init(); G_WRONG_NOT_A_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__ANNOUNCES() -> Py { ensure_init(); G__ANNOUNCES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANNOUNCES(v: Py) { ensure_init(); G__ANNOUNCES.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CATEGORY() -> Py { ensure_init(); G__A_CATEGORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CATEGORY(v: Py) { ensure_init(); G__A_CATEGORY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CAUSE_OPENS() -> Py { ensure_init(); G__A_CAUSE_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CAUSE_OPENS(v: Py) { ensure_init(); G__A_CAUSE_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLAUSE_LEAST() -> Py { ensure_init(); G__A_CLAUSE_LEAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLAUSE_LEAST(v: Py) { ensure_init(); G__A_CLAUSE_LEAST.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DOING() -> Py { ensure_init(); G__A_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DOING(v: Py) { ensure_init(); G__A_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DOING_WINDOW() -> Py { ensure_init(); G__A_DOING_WINDOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DOING_WINDOW(v: Py) { ensure_init(); G__A_DOING_WINDOW.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE() -> Py { ensure_init(); G__A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); G__A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LEAD_IN() -> Py { ensure_init(); G__A_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LEAD_IN(v: Py) { ensure_init(); G__A_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LONG_SENTENCE() -> Py { ensure_init(); G__A_LONG_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LONG_SENTENCE(v: Py) { ensure_init(); G__A_LONG_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PARTICIPLE() -> Py { ensure_init(); G__A_PARTICIPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PARTICIPLE(v: Py) { ensure_init(); G__A_PARTICIPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_REASON_LEAST() -> Py { ensure_init(); G__A_REASON_LEAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_REASON_LEAST(v: Py) { ensure_init(); G__A_REASON_LEAST.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SAVE_LEAST() -> Py { ensure_init(); G__A_SAVE_LEAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SAVE_LEAST(v: Py) { ensure_init(); G__A_SAVE_LEAST.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STRIDE() -> Py { ensure_init(); G__A_STRIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STRIDE(v: Py) { ensure_init(); G__A_STRIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERB() -> Py { ensure_init(); G__A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERB(v: Py) { ensure_init(); G__A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__EXPLAINS() -> Py { ensure_init(); G__EXPLAINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXPLAINS(v: Py) { ensure_init(); G__EXPLAINS.with(|c| *c.borrow_mut() = v); }
pub fn g__LEANS_BACK() -> Py { ensure_init(); G__LEANS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEANS_BACK(v: Py) { ensure_init(); G__LEANS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__LEANS_BACK_WINDOW() -> Py { ensure_init(); G__LEANS_BACK_WINDOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEANS_BACK_WINDOW(v: Py) { ensure_init(); G__LEANS_BACK_WINDOW.with(|c| *c.borrow_mut() = v); }
pub fn g__NEEDS_A_SUBJECT() -> Py { ensure_init(); G__NEEDS_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEEDS_A_SUBJECT(v: Py) { ensure_init(); G__NEEDS_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_ON_A_FIGURE() -> Py { ensure_init(); G__OPENS_ON_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_ON_A_FIGURE(v: Py) { ensure_init(); G__OPENS_ON_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_FORWARD() -> Py { ensure_init(); G__POINTS_FORWARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_FORWARD(v: Py) { ensure_init(); G__POINTS_FORWARD.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_FORWARD_ANY() -> Py { ensure_init(); G__POINTS_FORWARD_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_FORWARD_ANY(v: Py) { ensure_init(); G__POINTS_FORWARD_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_FORWARD_LEAST() -> Py { ensure_init(); G__POINTS_FORWARD_LEAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_FORWARD_LEAST(v: Py) { ensure_init(); G__POINTS_FORWARD_LEAST.with(|c| *c.borrow_mut() = v); }
pub fn g__SAYS_HOW() -> Py { ensure_init(); G__SAYS_HOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAYS_HOW(v: Py) { ensure_init(); G__SAYS_HOW.with(|c| *c.borrow_mut() = v); }
pub fn g__VERB_SHAPE() -> Py { ensure_init(); G__VERB_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERB_SHAPE(v: Py) { ensure_init(); G__VERB_SHAPE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The reason behind the answer just given: a bare \"why\" as a follow-up.\n\n\"why\", \"why not\" and \"how come\" carry no subject of their own, so nothing that matches words on\na line can take them: the knowledge base has no key to match, and the small-talk pools want a\nmood, not a question. Main answered them with the search hedge - \"I'd rather not guess, a quick\nsearch online will get you a solid answer\" - which throws away the turn just given and sends the\nperson off to look up something they already have half of.\n\nWhat a friend does with the nudge is say plainly that the reason is not something they can see\nfrom here, and ask the one question that would narrow it. That is all this module says. It never\ninvents a cause: a confident wrong reason is worse than an honest short one (STYLE.md,\n\"Honest\"), and the thing that would let the bot give a real reason is an entry\nin the knowledge base, not a phrase put together here.\n\nTwo things have to hold before it speaks. The line has to be the whole question, so a why with\nwords of its own (\"why is my car overheating\") goes to whatever can match those words. And the\nturn above has to be an answer that made a claim: a condolence, an emergency line, a recall or\nthe bot's own shrug is not something to ask the reason for, so those keep whatever they would\nhave said. That is what the module's ANSWERED list is for.\n");
    s_BARE_WHY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:but|and|so|ok|okay|yeah|no|well|right)\\s+)*(?:(?:why|y|how come)(?:\\s+(?:not|tho|though|so|then|is that|is it|was that|does it|do they|did it|did they|would it|wouldnt it|should i|shouldnt i)(?:\\s+(?:tho|though|then))?)?(?:\\s+(?:did|does|do|is|was|were|would|will|should)\\s+(?:that|it|this|they|those|these)(?:\\s+(?:happen|happened|happening|keep happening|keeps happening|work|works|not work|never work|be|matter|help|go|goes|go wrong|goes wrong|come|do that|do it))?)?|how come\\s+(?:that|it|this)(?:\\s+(?:happens?|happened|went wrong|goes wrong|keeps happening))?|what(?:'?s|\\s+is|\\s+was|\\s+(?:could|might|would|can|may)\\s+be)\\s+causing\\s+(?:that|it|this)|what(?:'?s|\\s+is|\\s+was)\\s+(?:going wrong|wrong|the (?:problem|issue|matter))|what\\s+(?:am|are|was|were)\\s+(?:i|we)\\s+(?:doing\\s+wrong|missing|getting\\s+wrong)|what\\s+did\\s+(?:i|we)\\s+do\\s+wrong)[\\s?!.,]*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CAUSING_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what(?:'?s|\\s+is|\\s+was|\\s+(?:could|might|would|can|may)\\s+be)\\s+causing\\s+(?:that|it|this)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WRONG({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat(?:'?s|\\s+is|\\s+was)?\\s*(?:am\\s+i\\s+)?(?:going\\s+wrong|wrong|doing\\s+wrong|getting\\s+wrong|missing|the\\s+(?:problem|issue|matter))\\b|\\bwhat\\s+did\\s+(?:i|we)\\s+do\\s+wrong\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LIFE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my (?:job|work|boss|career|business|kids?|children|son|daughter|mum|mom|dad|husband|wife|partner|boyfriend|girlfriend|friend|sister|brother|family|marriage|relationship|life|money|salary|wages|benefits|rent|interview|exam|degree|course)|job|jobs|jobless|unemployed|redundant|sacked|fired|divorce|divorced|breakup|broke up|shouting|shout at|arguing|argued|argument|falling out|fell out|not talking to me|grieving|funeral|depressed|anxious|lonely|ashamed|guilty)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_NOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bnot\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_DECISION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|they|him|her|them|his|their|my (?:boss|manager|landlord|landlady|wife|husband|partner|boyfriend|girlfriend|ex|mum|mom|mam|dad|father|mother|son|daughter|child|kid|friend|mate|neighbour|neighbor|colleague|council|company|employer|school|teacher|gp|doctor|nurse|worker|agent|insurer)\\b|the (?:council|company|school|landlord|manager|boss|insurer|bank|hospital|surgery)\\b)")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAID_NO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:said|says|saying|told|tells|refused|refuses|refusing|won'?t|wont|will not|can'?t|cant|cannot|denied|rejected|decided|decides|reckons|thinks|banned|stopped|turned (?:me|it|us) down|not allowed|no chance)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_SUGGESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\A(?:you (?:could|should|might|may|can) (?:try|just|also)|try|maybe try|have you tried|why don'?t you|what about|how about|consider)\\b|\\b(?:you (?:could|should|might) (?:try|also try)|have you tried)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUGGESTED_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("try"), py_str("cut"), py_str("stop"), py_str("quit"), py_str("feed"), py_str("turn"), py_str("bleed"), py_str("top"), py_str("add"), py_str("start"), py_str("use"), py_str("take"), py_str("buy"), py_str("get"), py_str("give"), py_str("keep"), py_str("put"), py_str("run"), py_str("walk"), py_str("drink"), py_str("eat"), py_str("avoid"), py_str("ask"), py_str("go"), py_str("see"), py_str("check"), py_str("swap"), py_str("replace"), py_str("wash"), py_str("clean"), py_str("sort"), py_str("leave"), py_str("make"), py_str("do")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANSWERED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("answers"), py_str("chat")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBSTANCE(Py::Int(8));
    s_HEDGED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:search|searching|beyond what i know|don'?t know enough|not something i can see|rather not guess|no way of knowing)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_ANSWERED(py_tuple(vec![py_str("I'd be guessing at the reason - the cause is not something I can see from here. Which part of that do you want the why for?"), py_str("I don't know why, and I'd rather say so than invent one. What happens just before it goes wrong?"), py_str("Honestly the why is the bit I can't tell from this side. Tell me which part you mean and I will help you work it out.")]));
    s_WHY_BODY(py_tuple(vec![py_str("Honestly, I can't tell you why from here - I'd be guessing. What sets it off?"), py_str("I don't know the reason, and I'd rather say so than invent one. When did you first notice it?"), py_str("The why is the part I can't see from what you've told me. What is it like at its worst?"), py_str("No idea why, honestly. Does it happen at a particular time, or after anything in particular?")]));
    s_CAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:because|since|that'?s why|which is why|that is why|is why|comes? from|caus(?:e|es|ed) by|means that|so that|the reason (?:is|was)|scatters?|keeps? \\w+ (?:from|off|out)|stops? [^.]{0,28}?\\b(?:forming|coming|happening|going|building|getting|spreading)|is down to|which (?:makes|means|is why)|that'?s what (?:makes|keeps|stops|drives|turns)|is what (?:drives|turns|causes|makes|keeps|stops)|(?:has|have|had|got) (?:too (?:little|much)|not enough)|(?:is|are) (?:almost |nearly |usually |normally |often |mostly )+(?:always )?\\w|live?s? in |breaks? (?:the |it |them )?\\w+ into |\\b(?:sinks?|go(?:es)?|turns?|comes?|happens?|fails?|breaks?|stops?|sticks?|cracks?|curdl(?:e|es)|collapses?|dries?|burns?|smells?|tastes?|looks?)\\b[^.;:]{0,40}?\\bwhen\\b|\\b(?:is|are|was|were)\\s+[^.;:]{0,40}?\\b\\w+(?:ed|en)\\b[^.;:]{0,20}?\\b(?:by|from)\\b)")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STEP_VERBS(py_str("(?:ventilate|open|keep|use|take|leave|put|run|wipe|wash|give|avoid|let|get|ask|check|try|swap|cover|move|hold|store|cook|drink|eat|sit|add|lower|raise|turn|clean|buy|restrict|limit|reduce|cut|change|switch|replace|bin|tip|throw|sort|fit|hire|book|pick|start|stop|do|make|go|wear|ring|call|fill|write|read|set|work|stay|carry|delete|update|download|install|uninstall|restart|reboot|reinstall|clear|free|close|move|rotate|treat|rinse|hoover|vacuum|dust|air|dry|iron|seal|label)"));
    s_SO_ADVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(",? ?so (?="), &g_STEP_VERBS()), &py_str("\\b)"))); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:once|twice|three times|four times|daily|weekly|nightly|every (?:day|week|night|morning)|a (?:day|week|night|month)|per (?:day|week|month)|\\d+ ?(?:times|minutes|hours|glasses|litres|grams|g|mg|mg|ml|degrees|sessions))\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AND_STEP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(",? ?and (?=(?!air\\b)"), &g_STEP_VERBS()), &py_str("\\b)"))); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AND_ADVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",? ?(?:and|but) (?:it|they|that|this|you|then|if)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEGATIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:never|don'?t|do not|avoid|no need to|there'?s no need)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDITIONAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:if|unless|once|when|before|after|until)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDITIONAL_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:if|unless|once|whenever|when|before|after|until|in case)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_INSTRUCTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:never|don'?t|do not|keep|call|ring|take|ask|check|make sure|you should|if you|try to|avoid|be careful|watch|use|put|put it|store|leave it|cover it|fit|swap|replace)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PARTS(py_tuple(vec![py_str(": "), py_str(" - "), py_str("; ")]));
    s_SO_REASON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bso\\s+(?:the|it|they|you|your|that|this|air|water|heat|warmth)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEADING_MARKER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:since|as|so)\\s+(?=\\w)")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAIM_BEFORE_COLON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^[^:]{4,}?\\b(?:is|are|was|were)\\b[^:]*:\\s*(?P<why>[^:]{12,})$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAIM_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:here|there|this|these|those|what|below|following)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IMPERATIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:soak|add|mix|stir|do|use|put|take|keep|leave|cover|store|check|try|avoid|don'?t|never|give|wipe|wash|open|close|turn|set|ring|call|ask|make|let|get|go|run|start|stop|fill|brush|floss|drink|eat|sit|stand|sleep|walk|stretch|reduce|cut|change|swap|replace|clean|remove|apply|spread|heat|cook|boil|bake|roast|freeze|thaw|rest|massage|wear|hang|bin|throw|unplug|label|write|book|hire|buy|sell|learn|practise|practice|repeat|look|watch|listen|say|tell|note|remember|think|pick|choose|plan|sort|fit|trim|prune|water|feed|move|top|blot|rinse|scrape|vacuum|spray|chop|slice|simmer|grill|season|serve|cool|chill|drain|pat|press|squeeze|peel|whisk|steam|blanch|defrost|shake|dust|sweep|mop|scour|polish|iron|sand|prime|seal|charge|reboot|restart|reset|update|empty|plug|tighten|loosen|screw|glue|nail|drill|measure|mark|taste|knead|fold|roll|save|delete)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_FIX({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was|are|'s) the (?:fix|answer|thing to do|way to do it)\\b|\\bfix next time\\b|\\bthe fix (?:is|for|next time)\\b|\\bis how you (?:do|fix|stop|get)\\b|\\bwhat (?:you do|to do) (?:next time|instead|now)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMED_CAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:the\\s+)?(?:cause|reason)\\s+(?:is|was)\\s+(.+)")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EXPLAINS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:because|since|so\\s+(?:air|the|it|they|you)|so\\s+\\w+(?:\\s+\\w+){0,4}\\s+(?:are|is|was|were)\\b|when\\b[^.!?]*\\b(?:are|is|was|were)\\b|which\\s+(?:makes|means|traps|tightens|widens)|making\\b|heats?\\s+(?:\\w+ )?(?:up\\b|the\\b|it\\b|them\\b|your\\b|a\\b)|expands?|traps?|tightens?|wide(?:ns?|ned|ning)?|carries|evaporat(?:es|ed|ing)|scatters?|tell(?:s|ing)?\\b[^.!?]*\\bto\\b|seems?\\s+to)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DANGLING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:but|and|so|though)\\s+)?(?:if\\s+)?(?:it|they|he|she|that|this|these|those)\\b|^(?:then|also|which|the same)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RESULT_NUMBER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",\\s*so\\b[^.!?]*\\d")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sinks?|sank|rises?|rose|drops?|dropped|collapses?|collapsed|sags?|sagged|cracks?|cracked|splits?|curdles?|curdled|separates?|separated|goes|went|gets|got|turns?|turned|becomes?|became|sticks?|stuck|burns?|burnt|dries?|dried|toughens?|toughened|shrinks?|shrank|smells?|smelt|tastes?|tasted|wrinkles?|wrinkled|tears?|tore|melts?|melted|sets?|yellows?|yellowed|browns?|browned|wilts?|wilted|droops?|drooped|leaks?|leaked|clogs?|clogged|overcooks?|overcooked|undercooks?|undercooked|overbakes?|overbaked|underbakes?|underbaked|overmixes?|overmixed)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHEN_CAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:when|whenever)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SO_FROM_HERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",?\\s+so\\s+|;\\s*")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_NAMED(py_tuple(vec![py_str("That's what is behind it: {c}"), py_str("It comes down to this: {c}"), py_str("{c} - that's the reason.")]));
    s_KEEP_CAPITAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("i"), py_str("i'm"), py_str("i'd"), py_str("i've"), py_str("i'll"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("britain"), py_str("british"), py_str("england"), py_str("english"), py_str("london"), py_str("scotland"), py_str("wales"), py_str("ireland"), py_str("america"), py_str("american"), py_str("europe"), py_str("christmas"), py_str("easter"), py_str("god")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_NAMED_BECAUSE(py_tuple(vec![py_str("Because {c}"), py_str("That's because {c}"), py_str("{c} - that's the reason.")]));
    s_WHY_NAMED_SELF(py_tuple(vec![py_str("The cause is {c}"), py_str("The reason is {c}")]));
    s_WHY_FRONT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&g_WHY_NAMED(), &g_WHY_NAMED_BECAUSE()), &g_WHY_NAMED_SELF())) { let mut frame: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }, "rstrip", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }) { let mut front: Py = __item.clone(); if py_truthy(&front.clone()) {  __out.push(front.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_END(py_str("- that's the reason"));
    s_WHY_AGAIN(py_tuple(vec![py_str("Because all I have is what you've told me, and the cause isn't in it - one more detail and I might see it."), py_str("I still can't see the cause from here: it isn't in what you've told me. What does it do just before it goes wrong?"), py_str("The reason is still the part I can't reach from this side. When it goes wrong, what has just happened?")]));
    s_WRONG_ANSWERED(py_tuple(vec![py_str("I can't see what is going wrong from here, and I'd rather say so than invent a cause. Does it go wrong the same way every time?"), py_str("Nothing you've told me points at the cause, so I'd be guessing. Which part goes wrong - the start, the middle, or the end?"), py_str("I can't see what is wrong from here, and guessing would not help you. Talk me through what happens and I'll tell you what I can see in it.")]));
    s_WRONG_NOT_A_FAULT(py_tuple(vec![py_str("I can't see a reason in what you've told me, and I'd rather say that than invent one. What happened just before it?"), py_str("Nothing you've told me points at a cause, and I won't guess at one. Talk me through what happened and I'll tell you what I can see in it."), py_str("I don't know why, honestly. Tell me a bit more about how it came about and I'll say what I can see.")]));
    s_WHY_NOT_ANSWERED(py_tuple(vec![py_str("I can't speak for anyone else's reasons - only for what you can do about it. What was said when you raised it?"), py_str("Fair question, and I have no way of knowing what is behind it. What is their side of it?"), py_str("I can't tell why not from here. What is stopping it, as far as you can see?")]));
    s_WHY_NOT_ITS_OWN(py_tuple(vec![py_str("No reason it has to be that way - it is the version that works for most people, not a rule. What is it about it that does not sit right?"), py_str("Only because it tends to be the one that works. If it does not suit you, say so and we will find another way."), py_str("There is nothing wrong with the other way if it suits you. What are you weighing up?"), py_str("Mostly because it works for most people, not because it is the only way. What would you rather do?")]));
    s_THINKS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:but|and|so|ok|okay|yeah|no|well|right)\\s+)*(?:why|y)\\s+(?:do|did|would)\\s+(?:you|u)\\s+(?:think|thinking)\\s+(?:that|so|this|it)(?:\\s+(?:then|though|tho))?[\\s?!.,]*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_NOD_OPENS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:alright|all\\s+right|ok|okay|sure|got\\s+it|noted|right|of\\s+course|no\\s+problem|lovely|great|good|glad|happy|sorry|thanks|thank\\s+you|fair\\s+enough|makes\\s+sense|understood|will\\s+do|cool|nice)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_NOD_IS_SHORT(Py::Int(12));
    s_THE_REASON_POINTS_AT_THE_ADVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:because\\s+|since\\s+)?(?:that|this)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FIRST_SENTENCES(Py::Int(2));
    s_WHY_METHOD(py_tuple(vec![py_str("Those steps are the how-to; the reason under them is the part I'd be guessing at. Tell me what happens and I'll say what I can see."), py_str("Everything I gave you there is what to do, not why it happens - that half isn't in what I have. What is it doing when it goes wrong?"), py_str("The method is what I can stand behind. The cause is the bit I can't see from here, so tell me what it does and I'll work it back from that.")]));
    s_EXAMPLE_IN_QUOTES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\"[^\"]{4,}\"|“[^”]{4,}”")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHAT_IT_TAKES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<thing>[^.;:]{4,}?\\b(?:is|are)\\s+what\\b[^,.;]{0,40}?(?:needs?|wants?|feeds?|thrives? on|likes?|goes? for)\\b[^,.;]*)[,.;]?\\s+so\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_NAMED_BEFORE_A_COLON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\A(?:it|that|this)\\s+(?:is|was|isn'?t|wasn'?t)\\s+(?:almost\\s+always\\s+|nearly\\s+always\\s+|usually\\s+|often\\s+|just\\s+)?(?:the|a|an|your|too\\s+much|not\\s+enough|too\\s+little)\\s+(?P<thing>[a-z][^:,.;!?]{1,40}?)(?:,\\s*(?:not|rather\\s+than)\\s+(?:the\\s+|a\\s+|your\\s+)?[a-z][^:,.;!?]{1,25})?\\s*:\\s*\\S")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LEAD_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\A(?:(?:nine|eight|seven|six|five|four|three|two)\\s+times\\s+(?:in|out\\s+of)\\s+(?:ten|twenty|a\\s+hundred|\\d+)|most\\s+of\\s+the\\s+time|more\\s+often\\s+than\\s+not|usually|nearly\\s+always|almost\\s+always|in\\s+my\\s+experience|as\\s+a\\s+rule|nine\\s+times\\s+out\\s+of\\s+ten)\\s+")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SHORT_OF_A_CAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\A(?!(?:it|that|this|there|which)\\b)[^.;:]{3,60}?\\s+(?:is|are|was|were)\\s+(?:usually\\s+|nearly\\s+always\\s+|almost\\s+always\\s+|often\\s+|probably\\s+|just\\s+)?(?P<cause>(?:short\\s+of|low\\s+on|lacking|too\\s+much|too\\s+little|not\\s+enough)\\b[^.;:]{1,60})")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AFTER_THE_MARK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?::| - |; )\\s*(?P<rest>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GENERAL_CLAIM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\A(?:most|almost\\s+all|nearly\\s+all|in\\s+most\\s+cases|the\\s+majority\\s+of|mostly)\\s+(?P<subject>[^.;:,]{2,60}?)\\s+(?:is|are|was|were)\\s+(?P<what>[^.;:]{2,140})")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ENDS_THE_CLASS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?::|,| - | and | so | which | but | worse )")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAYS_WHAT_IT_DOES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:want|wants|need|needs|live|lives|take|takes|last|lasts|go|goes|come|comes|get|gets|settle|settles|heal|heals|die|dies|cost|costs|help|helps|work|works|hold|holds|keep|keeps|put|puts|eat|eats|do|does|say|says|give|gives|pay|pays|rise|rises|fall|falls|start|starts|finish|finishes|run|runs|turn|turns|sit|sits|charge|charges|cover|covers|include|includes|pass|passes|count|counts|matter|matters|mean|means|stay|stays|carry|carries)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PLACEHOLDER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("them"), py_str("they"), py_str("one"), py_str("so"), py_str("true"), py_str("fine"), py_str("ok"), py_str("okay"), py_str("normal"), py_str("harmless"), py_str("nothing"), py_str("none"), py_str("air"), py_str("you"), py_str("me"), py_str("us")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANSWER_LONG(Py::Int(18));
    s_ACT_NOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:999|911|112|a&e|a and e|ambulance|paramedic|unconscious|not breathing|can'?t breathe|scald\\w*|never open|still hot|poison\\w*|gas leak|on fire|stop driving|pull over|don'?t wait|do not wait|stay on the line)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ACT_NOW_SOON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:111|same[- ]day|straight away|right away|immediately|today)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDITIONAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:if|unless|should|in case|any time)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^.?!;]+[.?!;]?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAID_AGAIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?!so\\s+what\\b)(?:(?:sorry|ok|okay|er|erm|um|eh|and|so|but|wait|no|yes)[\\s,]+)*(?:what|wat|wot|wut|huh|pardon|what\\s+did\\s+(?:you|u)\\s+say|what\\s+(?:was|is)\\s+that|say\\s+(?:that|it)\\s+again|say\\s+again|come\\s+again|run\\s+that\\s+by\\s+me\\s+again)(?:\\s+(?:sorry|please|mate|then|now))?[\\s?!.,]*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGAIN_SHORTER(py_tuple(vec![py_str("In short: {short}"), py_str("Shorter: {short}"), py_str("The short of it: {short}"), py_str("The gist of it: {short}")]));
    s_REPEAT_LEADS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AGAIN_SHORTER()) { let mut frame: Py = __item.clone(); __out.push(geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{short}")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(frame.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }, &Py::Int(0))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE_END({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":\\s+|;\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_SAVE_LEAST(Py::Int(5));
    s__A_CLAUSE_LEAST(Py::Int(6));
    s__A_LONG_SENTENCE(Py::Int(25));
    s__A_STRIDE(Py::Int(3));
    s_AGAIN_WORDS(Py::Int(40));
    s_NO_CUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gp|doctor|nurse|pharmacist|chemist|dentist|physio\\w*|midwife|health visitor|111|999|a&e|hospital|ambulance|paramedic)\\b|\\b(?:get|have)\\s+it\\s+(?:checked|looked at|seen)\\b|\\bworth\\s+(?:getting|having)\\s+.{0,30}?(?:checked|looked at)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_END_OF_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OPENS_ON_CARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok|okay|right|well|oh|ah|aw|ugh|sorry)?[\\s,\\u2014-]*(?:i'?m (?:really |so )?sorry|sorry|that (?:is|was) (?:a rough|rough|tough|hard|a lot)|no wonder|that sounds|lousy|horrible|awful|poor you|rough one)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DOING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:turn|switch|take|keep|put|use|try|check|give|add|buy|swap|replace|clean|wash|rinse|leave|stand|wrap|store|feed|water|encourage|bleed|call|ask|book|avoid|stop|start|run|set|hold|move|cover|cut|open|close|repeat|ring|email|text|pick|press|fill|mix|stir|boil|blot|pour|write|say|tell|offer|lower|heat|defrost|freeze|lock|shut|prick|scrub|soak|scrape|wipe|sweep|vacuum|iron|sand|seal|tape|glue|screw|drill|tie|hang|fold|chop|slice|drain|compare|measure|weigh|test|log|track|budget|plan|read|learn|practise|split|share|anchor|aim|ask|check|brush|clear|empty|refill|label|freeze|work|go|look|listen|watch|wait|sit|step|point|answer|name|note|mark|count|sort|tidy|eat|drink|cook|bake|grill|roast|serve|pack|carry|lift|bend|stretch|breathe|rest|get|give|help|do|redo|rub|polish|dry|spray|apply|dab|swab|wax|shampoo|polish)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DOING_WINDOW(Py::Int(3));
    s__A_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|thirty|forty|fifty|half|quarter|once|twice|per ?cent)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_LEAD_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("on for in with at to from after before if when once and then also"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SAYS_HOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|was|were|has|have|had|does|do|will|would|can|could|wants?|needs?|keeps?|stops?|strips?|holds?|works?|matters?|helps?|makes?|means?|costs?|takes?|gets?|gives?|comes?|stays?|lasts?)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ANNOUNCES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|them|these|those|this|that)\\b(?:\\s+[\\w'-]+){0,2}\\s+(?:order|list|steps?|way|shape)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NEEDS_A_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:he|she|it|they|them|this|that|these|those|his|her|their|its|itself|themselves)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PARTICIPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("laid launched built made given taken held written born grown founded\nopened designed painted published signed set sold bought won lost sent kept left put begun"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__VERB_SHAPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\w{4,}(?:ed|ing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("is are was were be been am has have had do does did can could will would\nshould must might may need needs want wants take takes give gives make makes keep keeps leave\nleaves put puts get gets add adds use uses turn turns go goes come comes cost costs last lasts\nhelp helps hold holds cut cuts run runs set sets let lets seem seems feel feels work works mean\nmeans change changes choose chooses find finds show shows matter matters depend depends ring\nrings wash washes clean cleans"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CAUSE_OPENS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:nearly|usually|mostly|often|almost|probably|generally|mainly|sometimes|always|never|rarely|hardly)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LEANS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:them|these|those|both|each|either|neither|the rest|all of (?:the|them)|the others?|any of (?:the|them)|that end|this end|this order|these steps|the ones|as follows)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LEANS_BACK_WINDOW(Py::Int(6));
    s__EXPLAINS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the|most|some|many|much|which|because|so|that|this)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_REASON_LEAST(Py::Int(12));
    s__OPENS_ON_A_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:[£$\\u20ac]\\s*\\d|\\d)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CATEGORY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|them|these|those|things|stuff|bits|ones|kinds|sorts|lots)\\b|\\b(?:need|needs|want|wants|have|has|how|what|where|when|who)\\s+to\\W*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__POINTS_FORWARD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|them|these|those)\\W*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__POINTS_FORWARD_ANY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bthe (?:most|best|worst|biggest|main|first|last|rest)\\W*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__POINTS_FORWARD_LEAST(Py::Int(10));
    s_WHY_YOU_SAID_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:but|and|so|ok|okay|yeah|no|well|right)\\s+)*(?:(?:why|y|how come)\\s+(?:(?:do|did|would|will|are|were)\\s+)?(?:you|u)\\s+(?:say|saying|said|tell me|told me|reckon|claim)|what\\s+makes\\s+(?:you|u)\\s+say|why\\s+(?:do\\s+you\\s+|would\\s+you\\s+)?think)\\s+(?:that|this|so|it|those|these)(?:\\s+is)?(?:\\s+(?:then|though|tho))?[\\s?!.,]*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_BOT_DESCRIBED_ITSELF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a program|a chatbot|a bot|not a person|no feelings|nothing in me|just rules|written rules|rules someone wrote|rules doing their job|no mind behind|behind the curtain|nothing to report on my side|no inner)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CARE_ANSWER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i'?m (?:really |so )?sorry|so sorry|sorry to hear|sorry about|a real loss|be gentle with yourself|no words needed|you did the right thing|i'?m here for you|here if you want|nothing you have to say|how are you (?:holding up|doing))\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY_CARE(py_tuple(vec![py_str("Nothing behind it: you told me something hard, and I meant it."), py_str("No reason beyond what you said - I'm sorry, and I still mean it."), py_str("There's nothing to work out in it: you said something hard, and that was enough for me to be sorry about it.")]));
    s_WHY_THE_BOT(py_tuple(vec![py_str("Because there is nothing in here to feel it with: I'm rules someone wrote for ordinary questions, and the words come from those rather than from anything I have lived."), py_str("Because that is all I am made of - written rules. Nothing in me feels something first and reports it after, so there is no inner reason behind the line."), py_str("Because there is no machinery for it in here, which is the whole of the answer: I pick the words from what is written into me and from what you have told me here.")]));
    s_ABOUT_THE_BOT_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:so|ok|okay|and|but|well|right|hey|hi)\\s+)*(?:(?:are|do|did|does|can|could|would|will|have|has|were|is)\\s+(?:you|u|ya)\\b|(?:you|u)\\s+(?:are|re)\\b|tell me about yourself\\b|what are you\\b|who are you\\b|what do you do\\b)")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _named_as_a_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut reason: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reason") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reason"))) };
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("True when the answer named this very clause as its cause, as \"the cause is ...\".");
    want = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reason.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_CAUSE(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_eq(&clause.clone(), &want.clone()))) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _reads_a_fault(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut said: Py = Py::None;
    let mut them: Py = Py::None;
    let _ = py_str("True when the turn above was about a thing that can go wrong, not about the person's life.\n\n    The fault wording asks which part of the process goes wrong, so it needs a process: a loaf,\n    a boiler, a laptop, a washing machine. A line about a job, a row with a friend or a funeral\n    is not one, and the honest reply there is the shrug that asks what happened.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    them = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(them.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LIFE(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _about_someone_elses_decision(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the person's own last line was about somebody else deciding or refusing.\n\n    Read on their line rather than the bot's, because the bot's last line is the advice the \"why\n    not\" is asking about. Only their line can say whether there is another party in the question.\n    ");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_DECISION(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAID_NO(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_suggestion_was_theirs(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the person's own line handed on the advice the \"why not\" is asking about.");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_SUGGESTION(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?:;")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "strip", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SUGGESTED_VERBS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?:;")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "strip", args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _in_frame(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frame: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame"))) };
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut keep: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The clause put into a frame, with its first letter following the frame's sentence.");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\"'(*[")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) };
    keep = { let __b = Py::Bool(py_contains(&g_KEEP_CAPITAL(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?:;")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, "strip", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&word.clone(), &Py::Int(1), &Py::Int(2), &Py::None), "isupper", args))(&__args) } } };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{c}")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "startswith", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{c}")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "replace", args))(&__args) };
    }
    if py_truthy(&{ let __b = keep.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clause.clone())) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{c}")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "replace", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{c}")); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", args))(&__args) }, &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "replace", args))(&__args) };
    Py::None
}
pub fn is_a_reason_line(args: &[Py]) -> Py {
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
    let mut front: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("True when this is a cause this module handed back, not an answer of its own.\n\n    \"my phone battery drains fast\" / \"why is that\" is answered \"It's the reason I gave: social and\n    video apps in the background are usually it\", and the person then asks \"is it dangerous\". That\n    question is about the phone, so whoever answers it has to read past this line to the answer the\n    reason came out of: without this the verdict composer read the reason line as the answer and\n    sent a worried person to a search engine instead of saying the boiler's bang is ordinary.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_END()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "rstrip", args))(&__args) }, "endswith", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHY_FRONT()) { let mut front: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(front.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "startswith", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _said_the_reason_cannot_be_seen(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last line was this module's own answer to the same question.\n\n    \"why\" asked twice in a row, or a \"why\" after \"what could be causing it\", is the same question\n    unchanged, and a search hedge the second time reads as if the first answer had not happened.\n    Only this module's own pools count, matched whole: a framed line carries the answer's own\n    words, so it is not a shrug and a second \"why\" on it is not this case.\n    ");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turns.clone(), &op_neg(&Py::Int(1)))); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } } } }) {
    return Py::Bool(false);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let __b = Py::Bool(py_contains(&g_WHY_ANSWERED(), &said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WHY_BODY(), &said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WRONG_ANSWERED(), &said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WHY_METHOD(), &said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WRONG_NOT_A_FAULT(), &said.clone())) } } } } } } } };
    Py::None
}
pub fn _is_bare_why(args: &[Py]) -> Py {
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
    let _ = py_str("True when the whole line asks for the reason and nothing else.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_WHY(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _last_turn(args: &[Py]) -> Py {
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
    let mut reason: Py = Py::None;
    let mut said: Py = Py::None;
    let mut skipped_a_nod: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The turn above this one, or {} when there is nothing to explain.\n\n    Turns memory keeps {\"user\", \"bot\", \"module\"} for each line. Only an answer that made a claim\n    is worth asking the reason for: one of the ANSWERED modules, and long enough to be an answer.\n    ");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_dict(vec![]);
    }
    skipped_a_nod = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_dict(vec![]);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _answered_nothing(args))(&__args) } } }) {
    skipped_a_nod = Py::Bool(true);
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_ANSWERED(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SUBSTANCE()) } }) {
    return py_dict(vec![]);
    }
    if py_truthy(&skipped_a_nod.clone()) {
    reason = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reason_in(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reason.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reason.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_REASON_POINTS_AT_THE_ADVICE(), "match", args))(&__args) } } }) {
    return py_dict(vec![]);
    }
    }
    return last.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }, &py_str("greeting"))); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HEDGED(), "search", args))(&__args) } } } } }) {
    continue;
    }
    return py_dict(vec![]);
    }
    return py_dict(vec![]);
    Py::None
}
pub fn _answered_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("True when a bot line is a nod rather than an answer: it gives no step and names no cause.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_A_NOD_IS_SHORT()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CAUSE(), "search", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EXPLAINS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_CAUSE(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_NOD_OPENS(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _gave_a_method(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut s: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut steps: Py = Py::None;
    let _ = py_str("True when the answer above was the how-to and said nothing about why it goes wrong.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CAUSE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EXPLAINS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_CAUSE(), "search", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut s: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&s.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&s.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    steps = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut s: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(s.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(s.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) } } }) {  __out.push(s.clone()); } } py_list(__out) })() };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(steps.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_ge(&op_mul(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(steps.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } };
    Py::None
}
pub fn _a_cause_named_outright(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut cause: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The cause the answer above names as a cause, or \"\" when it only has a \"because\" clause.\n\n    For \"why not\", where the answer's positive reason said back as a reason not to is the\n    opposite of what it wrote. The two shapes that name a cause on their face are the thing in\n    front of a colon (\"It is the caffeine: it peaks about half an hour after the cup ...\") and\n    \"the cause is ...\".\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &g_FIRST_SENTENCES(), &Py::None));
    for __item in __iter1 {
    sentence = __item.clone();
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) }) {
    continue;
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_CAUSE(), "search", args))(&__args) };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIX(), "search", args))(&__args) })) } }) {
    cause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return cause.clone();
    }
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_NAMED_BEFORE_A_COLON(), "match", args))(&__args) };
    if py_truthy(&thing.clone()) {
    return op_add(&py_str("the "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "group", args))(&__args) }, "strip", args))(&__args) });
    }
    }
    return py_str("");
    Py::None
}
pub fn _reason_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut strict: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "strict") { v.clone() }
    else { Py::Bool(false) };
    let mut reason: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The cause the answer above already named, as one clause, or \"\" when it named none.\n\n    An answer sometimes carries an example in its own quotation marks, and the \"because\" in it is\n    the example's reason, not the answer's: \"A thesis statement is one sentence that says the\n    claim your essay will prove, and why it matters: `Cheaper public transport would cut city\n    emissions more than electric cars, because most journeys are short.` Put it at the end of your\n    first paragraph\" answered \"and why does it matter\" with \"Because most journeys are short.\" Put\n    it at the end of your first paragraph - the reason out of somebody else's sample sentence, cut\n    where its own sentence ended. So a reason found inside a quotation is thrown away and the\n    answer is read again without its examples. What is left is the answer's own claim, and if it\n    names no reason the shrug is the honest reply.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    reason = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("strict".to_string(), strict.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_reason_in(args))(&__args) };
    if py_truthy(&{ let __b = reason.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(reason.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _inside_a_quotation(args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_examples(args))(&__args) }); __kws.extend(vec![("strict".to_string(), strict.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_reason_in(args))(&__args) };
    }
    return reason.clone();
    Py::None
}
pub fn _without_examples(args: &[Py]) -> Py {
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
    let _ = py_str("The answer with its quoted examples taken out, so their words cannot be handed back.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EXAMPLE_IN_QUOTES(), "sub", args))(&__args) };
    Py::None
}
pub fn _inside_a_quotation(args: &[Py]) -> Py {
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
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut marks: Py = Py::None;
    let _ = py_str("True when the clause sits inside a quotation in the answer, so it is an example's words.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "find", args))(&__args) };
    if py_truthy(&op_le(&at.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    before = slice_or_raise(&said.clone(), &Py::None, &at.clone(), &Py::None);
    marks = op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\"")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "count", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("“")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "count", args))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("”")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "count", args))(&__args) });
    return Py::Bool(py_eq(&op_modulo(&marks.clone(), &Py::Int(2)), &Py::Int(1)));
    Py::None
}
pub fn _a_reason_in(args: &[Py]) -> Py {
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
    let mut strict: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "strict") { v.clone() }
    else { Py::Bool(false) };
    let mut cause: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut instruction_ok: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_class: Py = Py::None;
    let mut number: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut passes: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut short: Py = Py::None;
    let mut text: Py = Py::None;
    let mut trigger: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The cause the answer above already named, as one clause, or \"\" when it named none.\n\n    Quoting the bot's own answer invents nothing: the words were said on the turn before. It is\n    cut to the clause that carries the cause, because the rest of the answer is the advice the\n    person already has.\n\n    The sentences are read twice. The first reading only takes a part that is not an instruction,\n    because a part that starts with one is what to do, not why it happened: \"take it off the heat,\n    because the smoky taste spreads\" and \"wipe the mould off, then stop it coming back\" carry\n    cause words and would answer \"why\" with a to-do list. Only when the answer names its cause\n    nowhere else does the second reading take those parts too, which is what it did before the\n    line shapes here were widened - so a line that used to get a reason still gets one.\n    ");
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    sentences = slice_or_raise(&whole.clone(), &Py::None, &g_FIRST_SENTENCES(), &Py::None);
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) }) {
    continue;
    }
    named = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_NAMED_BEFORE_A_COLON(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LEAD_IN(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_NAMED_BEFORE_A_COLON(), "match", args))(&__args) } } };
    if py_truthy(&named.clone()) {
    return op_add(&py_str("the "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "group", args))(&__args) }, "strip", args))(&__args) });
    }
    short = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SHORT_OF_A_CAUSE(), "match", args))(&__args) };
    if py_truthy(&short.clone()) {
    cause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cause")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&short.clone(), "group", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return cause.clone();
    }
    }
    }
    let __iter2 = py_iter_or_raise(&whole.clone());
    for __item in __iter2 {
    sentence = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_CAUSE(), "search", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RESULT_NUMBER(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIX(), "search", args))(&__args) } } } } } } }) {
    continue;
    }
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return clause.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&sentences.clone());
    for __item in __iter3 {
    sentence = __item.clone();
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHAT_IT_TAKES(), "search", args))(&__args) };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) })) } }) {
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {
    return clause.clone();
    }
    }
    }
    passes = if py_truthy(&strict.clone()) { py_tuple(vec![Py::Bool(false)]) } else { py_tuple(vec![Py::Bool(false), Py::Bool(true)]) };
    let __iter4 = py_iter_or_raise(&passes.clone());
    for __item in __iter4 {
    instruction_ok = __item.clone();
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    number = __unpk6[0].clone();
    sentence = __unpk6[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RESULT_NUMBER(), "search", args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DANGLING(), "match", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&instruction_ok.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) } } } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL_SENTENCE(), "match", args))(&__args) }) {
    continue;
    }
    trigger = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _trigger_in(args))(&__args) };
    if py_truthy(&{ let __b = trigger.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(trigger.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIX(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) } } })) } } } }) {
    return trigger.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CAUSE(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAIM_BEFORE_COLON(), "match", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EXPLAINS(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIX(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL_SENTENCE(), "match", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DOSE(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_the_answer_itself(args))(&__args) })) } } } } } } } } } } } } } }) {
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {
    return clause.clone();
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL_SENTENCE(), "match", args))(&__args) }) {
    continue;
    }
    parts = py_list(vec![sentence.clone()]);
    let __iter7 = py_iter_or_raise(&g_PARTS());
    for __item in __iter7 {
    mark = __item.clone();
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mark.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "split", args))(&__args) }) { let mut piece: Py = __item.clone(); __out.push(piece.clone()); } } py_list(__out) })() };
    }
    let __iter8 = py_iter_or_raise(&parts.clone());
    for __item in __iter8 {
    part = __item.clone();
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(number.clone()); __args.push(instruction_ok.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cause_clause(args))(&__args) };
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIX(), "search", args))(&__args) })) } }) {
    if py_truthy(&{ let __b = strict.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carries_the_fix(args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }, "rstrip", args))(&__args) };
    }
    return clause.clone();
    }
    trigger = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _trigger_in(args))(&__args) };
    if py_truthy(&{ let __b = trigger.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(trigger.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIX(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) } } })) } } } }) {
    return trigger.clone();
    }
    }
    }
    }
    named_class = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_by_definition(args))(&__args) };
    if py_truthy(&named_class.clone()) {
    return named_class.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_class_it_belongs_to(args))(&__args) };
    Py::None
}
pub fn _is_the_answer_itself(args: &[Py]) -> Py {
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
    let mut sentence: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut part: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("True when this sentence is the whole answer, so handing it back explains nothing.\n\n    A one-sentence answer has no paragraph for the sentence to be a part of. Handing it back to\n    \"why\" gives the person the line they have just read with a frame in front of it: \"That's what\n    is behind it: Six to eight glasses a day, about 1.5 to 2 litres, is plenty, and more in heat\n    or after exercise...\" is not one word of reason, and the advice in it reads as if it were the\n    reason. The reading goes on to the parts below, where a `so` clause inside the sentence can\n    still name the cause (\"spit rather than rinse so the fluoride stays on\"); when there is none\n    the honest shrug is the reply, which is more use than the answer again.\n\n    Only a sentence that carries nearly every word of the answer counts. A sentence of a longer\n    answer is a part of it, and one that is most of a two-sentence answer is still not the whole\n    thing.\n    ");
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) };
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&whole.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&part.clone())) } }) {
    return Py::Bool(false);
    }
    return op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(0.9))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    let mut word: Py = Py::None;
    let _ = py_str("The words of a line, folded, so punctuation and case cannot hide a repeat.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _carries_the_fix(args: &[Py]) -> Py {
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
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut found: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the sentence says the cause and then the fix after its colon or dash.");
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "startswith", args))(&__args) }))) {
    return Py::Bool(false);
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&rest.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFTER_THE_MARK(), "match", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _trigger_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut clause: Py = Py::None;
    let mut head: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The reason when the answer explains a fault by its trigger, else \"\".\n\n    \"A cake sinks when it's underbaked\", \"pastry cracks when it's too dry or too warm\": the\n    change comes first and the condition that makes it happen comes after `when`, so the whole\n    clause, cut where the advice starts, is the reason. It invents nothing - the words were said\n    on the turn before - and it is the shape most keeping and cooking entries use to explain a\n    fault. An instruction (\"replace them when the midsole feels flat\") and a definition (\"it's\n    done when a skewer comes out clean\") are not causes, so they keep the honest shrug.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEGATIVE(), "search", args))(&__args) } } } } } } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHEN_CAUSE(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) };
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CHANGE(), "search", args))(&__args) })) } }) {
    continue;
    }
    clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SO_FROM_HERE(), "split", args))(&__args) }, &Py::Int(0));
    clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(": ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }, &Py::Int(0));
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&head.clone(), &py_str(" ")), &clause.clone()), "strip", args))(&__args) }, "rstrip", args))(&__args) }, "strip", args))(&__args) };
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cut_long(args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))) {
    continue;
    }
    return clause.clone();
    }
    return py_str("");
    Py::None
}
pub fn _cause_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut number: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut instruction_ok: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "instruction_ok") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: instruction_ok"))) };
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut from_the_marker: Py = Py::None;
    let mut prefix: Py = Py::None;
    let _ = py_str("The cause in one part of a sentence, as a clause, or \"\" when it names none.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CAUSE(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SO_REASON(), "search", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&instruction_ok.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("because"), py_str("since")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }))) {
    prefix = slice_or_raise(&part.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "lower", args))(&__args) }, "find", args))(&__args) }, &Py::None);
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(prefix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEGATIVE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(prefix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL(), "search", args))(&__args) } } }) {
    return py_str("");
    }
    }
    from_the_marker = { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("because"), py_str("since")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&gattr(&found.clone(), "re"), &g_SO_REASON())) } };
    clause = if py_truthy(&from_the_marker.clone()) { slice_or_raise(&part.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "lower", args))(&__args) }, "find", args))(&__args) }, &Py::None, &Py::None) } else { part.clone() };
    clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SO_ADVICE(), "split", args))(&__args) }, &Py::Int(0));
    clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AND_STEP(), "split", args))(&__args) }, &Py::Int(0));
    clause = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AND_ADVICE(), "split", args))(&__args) }, &Py::Int(0));
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("because ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "strip", args))(&__args) }); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEADING_MARKER(), "sub", args))(&__args) };
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "strip", args))(&__args) }, "rstrip", args))(&__args) }, "strip", args))(&__args) };
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cut_long(args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("because "), py_str("since ")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "lower", args))(&__args) }, "startswith", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) } } }) {
    return py_str("");
    }
    return clause.clone();
    Py::None
}
pub fn _the_class_it_belongs_to(args: &[Py]) -> Py {
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
    let mut claim: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("The class the answer said the thing belongs to, as the reason for it, or \"\".\n\n    Read from the answer's own opening sentence, because that is where an entry puts what the thing\n    is: \"Most back pain is muscular and settles in two to six weeks, and staying active is what\n    helps most\". What comes back is the claim as it stands, not a rewrite of it, so the reply is the\n    bot's own words: quoting them invents nothing.\n    ");
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)), "strip", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL_SENTENCE(), "match", args))(&__args) } } }) {
    return py_str("");
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GENERAL_CLAIM(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    claim = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENDS_THE_CLASS(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&claim.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&claim.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(14)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(claim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAYS_WHAT_IT_DOES(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d")); __args.push(claim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PLACEHOLDER(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&claim.clone(), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(claim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIX(), "search", args))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(claim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(claim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DOSE(), "search", args))(&__args) } } }) {
    return py_str("");
    }
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&first.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = lead.clone(); if py_truthy(&__b) { __b } else { op_add(&py_str("Most "), &subject.clone()) } }); __args.push(claim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{} {}"), "format", args))(&__args) };
    Py::None
}
pub fn _named_by_definition(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut why: Py = Py::None;
    let _ = py_str("The reason when the answer gives it as a definition: \"Most headaches are tension: ...\".\n\n    Some answers say what the thing is and then say what that means, and the part after the colon\n    is the reason the person is asking for. It is only taken when the claim in front of the colon\n    is about the thing and not an announcement (\"here is what to do\"), and when what follows reads\n    as a clause rather than as instructions, so the reason handed back is never an order.\n    ");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::Int(1), &Py::None));
    for __item in __iter1 {
    sentence = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAIM_BEFORE_COLON(), "match", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAIM_SUBJECT(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) } } } } }) {
    continue;
    }
    why = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("why")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("\"'‘“"), &slice_or_raise(&why.clone(), &Py::None, &Py::Int(1), &Py::None))); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(why.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IMPERATIVE(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(why.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INSTRUCTION(), "match", args))(&__args) } } } } }) {
    continue;
    }
    why = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(why.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SO_ADVICE(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }, "rstrip", args))(&__args) };
    why = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(why.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cut_long(args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&why.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))) {
    continue;
    }
    return op_add(&op_add(&py_str("because "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&why.clone(), &Py::Int(0)), "lower", args))(&__args) }), &slice_or_raise(&why.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    Py::None
}
pub fn _cut_long(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut head: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The clause at its own comma, or \"\" when it only ends where the character limit is.\n\n    `\"...and if that fails make...\"` is a cut-off instruction handed back as a reason. A clause\n    that has to be chopped is not a reason this module can stand behind.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) };
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(24))) {
    return clause.clone();
    }
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &Py::None, &Py::Int(24), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(", ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "rfind", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" - ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "rfind", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_lt(&at.clone(), &op_floordiv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)))) {
    return py_str("");
    }
    return slice_or_raise(&head.clone(), &Py::None, &at.clone(), &Py::None);
    Py::None
}
pub fn _an_answer_to_act_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("True when the answer above says to act now, so the reason is not read out of it.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ACT_NOW(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENTENCE(), "findall", args))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ACT_NOW_SOON(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL(), "search", args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _first_sentence(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The answer's own opening sentence, or \"\" when it has only one.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_END_OF_SENTENCE(), "split", args))(&__args) }) { let mut part: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:- ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&parts.clone(), &Py::Int(0)), "rstrip", args))(&__args) };
    Py::None
}
pub fn _a_shorter_way(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut first: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("The answer above said shorter, or \"\" when there is nothing in it to cut down to.\n\n    In order: the answer's opening sentence, cut at a colon or semicolon when that sentence is a\n    long one and the cut is a real clause; the opening sentence itself when it is one; and a clause\n    of the whole answer for an answer that is a single sentence.\n\n    The cut comes first only for a long sentence, because \"In short:\" in front of a sentence the\n    length of a paragraph is not a short form of anything: \"Defrost it when the ice is more than\n    about 5mm thick: switch it off at the wall, move the food into a cool box, and leave bowls of\n    hot water inside with the door shut.\" was handed back whole behind the frame at 37 words, where\n    the answer two turns up says \"I said: Defrost it when the ice is more than about 5mm thick.\"\n    A sentence short enough to be read again is kept whole even when it has a colon in it, because\n    its own opening is what a person who did not follow it needs (\"A cold stays in the nose and\n    throat: a runny nose, sneezing, a sore throat\" says more than \"A cold stays in the nose and\n    throat.\"). A clause is only taken when it is a real one - \"The things that work: same wake-up\n    time every day...\" opens on a label, and handing back \"In short: The things that work.\" is the\n    label and none of the answer.\n\n    Only the answer's own words, in their own order, so a shortened answer cannot be wrong in a\n    way the long one was not.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _past_a_lead_in(args))(&__args) };
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _first_sentence(args))(&__args) };
    if py_truthy(&first.clone()) {
    doing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_doing_of(args))(&__args) };
    if py_truthy(&doing.clone()) {
    return doing.clone();
    }
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_clause_of(args))(&__args) };
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__A_LONG_SENTENCE())) {
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { op_le(&op_mul(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__A_STRIDE()), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    return clause.clone();
    }
    if py_truthy(&Py::Bool(!py_eq(&lead.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _end_a_sentence(args))(&__args) };
    }
    return py_str("");
    } else {
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { op_ge(&op_mul(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    return clause.clone();
    }
    }
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_AGAIN_WORDS())) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _end_a_sentence(args))(&__args) };
    }
    return clause.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _past_a_lead_in(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_clause_of(args))(&__args) };
    Py::None
}
pub fn _past_a_lead_in(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The answer without a lead-in sentence in front of it, which is a label and not the answer.\n\n    \"Take it in this order. First clear any credit card...\" hands back \"In short: Take it in this\n    order.\" when the lead-in is read as the opening: five words that say what the answer is about\n    to do and none of it. A short opening sentence with more after it is stepped over, so the\n    sentence under it is the one kept.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_END_OF_SENTENCE(), "split", args))(&__args) }) { let mut part: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return said.clone();
    }
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&parts.clone(), &Py::Int(0)), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    return said.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&parts.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    Py::None
}
pub fn _end_a_sentence(args: &[Py]) -> Py {
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
    let _ = py_str("The sentence with its full stop, so the frame in front of it reads as a sentence.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:- ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    return if py_truthy(&Py::Bool(py_contains(&py_str(".!?"), &geti(&text.clone(), &op_neg(&Py::Int(1)))))) { text.clone() } else { op_add(&text.clone(), &py_str(".")) };
    Py::None
}
pub fn _a_clause_of(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The first real clause of one long sentence, or \"\" when it has none worth handing over.");
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAUSE_END(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:-— ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&sentence.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _brackets_closed(args))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carries_a_verb(args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leans_back(args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _points_at_what_follows(args))(&__args) }) {
    continue;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _end_a_sentence(args))(&__args) };
    }
    return py_str("");
    Py::None
}
pub fn _the_doing_of(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The half after the sentence's colon when that half is what to do, or \"\" otherwise.\n\n    Read before the opening cut, because the stride rule below asks whether the opening half is\n    enough of the sentence to stand for it - and the half that says what to do is the answer to\n    \"what?\" at any length, which is the opposite of the opening half.\n    ");
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAUSE_END(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:-— ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&sentence.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _brackets_closed(args))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _points_at_what_follows(args))(&__args) }) {
    continue;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); __args.push(slice_or_raise(&sentence.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_half_that_does_it(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _end_a_sentence(args))(&__args) };
    }
    return py_str("");
    Py::None
}
pub fn _opens_a_doing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the half opens on a thing to do rather than on a thing to know.\n\n    The first word is what is read, not any word in the first three: \"warm tyres read a few psi\n    high\" is a statement with the verb \"read\" in it, and handing it back as the step for \"how do i\n    check my tyre pressure\" is the reason rather than the answer.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DOING(), "fullmatch", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g__A_LEAD_IN(), &geti(&words.clone(), &Py::Int(0))))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &op_add(&g__A_DOING_WINDOW(), &Py::Int(2)), &Py::None)) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DOING(), "fullmatch", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_how_it_works(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let mut first: Py = Py::None;
    let _ = py_str("True when a colon half is a clause about the thing rather than a list of the things.");
    first = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;]")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SAYS_HOW(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__VERB_SHAPE(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _announces_the_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let _ = py_str("True when a half is a heading for the list in the other half rather than the doing itself.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ANNOUNCES(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _needs_its_subject(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a half cannot stand alone as a short form: it opens on a pronoun or a participle.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NEEDS_A_SUBJECT(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z]")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let __b = Py::Bool(py_contains(&g__A_PARTICIPLE(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("set"), py_str("left"), py_str("put"), py_str("lost")]), &first.clone())) } };
    Py::None
}
pub fn _carries_a_verb(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a half is a sentence rather than a list of places or things.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DOING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__VERB_SHAPE(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_VERB(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CAUSE_OPENS(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _leans_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let _ = py_str("True when a half needs the half before it to mean anything.\n\n    Only in its first few words: \"work through the examples with the book open, then close it and do\n    them again\" carries its own \"them\" - the examples - while \"Read them, and if half of them bore\n    you\" opens on a word with nothing behind it.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }, &Py::None, &g__LEANS_BACK_WINDOW(), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEANS_BACK(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_reason(args: &[Py]) -> Py {
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
    let _ = py_str("True when the half after the colon justifies the half before it rather than completing it.");
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__A_REASON_LEAST()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&after.clone(), &py_str(","))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&after.clone(), &py_str(";"))) } } } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carries_a_verb(args))(&__args) };
    Py::None
}
pub fn _opens_on_a_figure(args: &[Py]) -> Py {
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
    let _ = py_str("True when the half after the colon opens on an amount or a count.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OPENS_ON_A_FIGURE(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _names_no_particular(args: &[Py]) -> Py {
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
    let _ = py_str("True when the opening is general enough that only the colon half says what to do.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CATEGORY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_half_that_does_it(args: &[Py]) -> Py {
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
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("The half of a colon sentence a person who did not follow it needs, or \"\" for the opening.\n\n    The opening half is kept when it is itself the doing (\"Take away the food and the easy way in\")\n    or when it carries the figure asked for (\"A pound or two, or round it up\"). The half after the\n    colon is handed over when the opening only says what is wrong or what a thing is and the doing\n    or the number is on the other side (\"A dripping tap is nearly always a worn washer: turn the\n    water off at the valve under the sink\").\n    ");
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:- ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_floordiv(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carries_a_verb(args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leans_back(args))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _needs_its_subject(args))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_a_doing(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _announces_the_list(args))(&__args) })) } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_a_doing(args))(&__args) }) {
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) {
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_reason(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_on_a_figure(args))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_on_a_figure(args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_no_particular(args))(&__args) }))) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says_how_it_works(args))(&__args) }) {
    return py_str("");
    }
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_a_doing(args))(&__args) }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_on_a_figure(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_no_particular(args))(&__args) })) } } } }) {
    return py_str("");
    }
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__EXPLAINS(), "match", args))(&__args) }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _opens_on_a_figure(args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_no_particular(args))(&__args) }))) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_reason(args))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carries_a_verb(args))(&__args) }) {
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE(), "search", args))(&__args) } } }) {
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    Py::None
}
pub fn _points_at_what_follows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cut: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cut") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cut"))) };
    let _ = py_str("True when a cut is too short and too vague to be anything but a heading.");
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = cut.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__POINTS_FORWARD_ANY(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__POINTS_FORWARD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__POINTS_FORWARD_LEAST()) } };
    Py::None
}
pub fn _brackets_closed(args: &[Py]) -> Py {
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
    let _ = py_str("True when every bracket in `text` is opened and closed in it.");
    text = { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    return { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "count", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(")")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "count", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "count", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "count", args))(&__args) })) } };
    Py::None
}
pub fn _the_route_is_in_it(args: &[Py]) -> Py {
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
    let _ = py_str("True when the answer names the way out of it, so no part of it can be handed back alone.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NO_CUT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn said_again(args: &[Py]) -> Py {
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
    let mut short: Py = Py::None;
    let _ = py_str("The reply to a bare \"say that again\", or None when the line is not one this reads.\n\n    Public because the composer's own repeat pool (`REACTION_REPEAT` in fallback.py) is reached\n    first for \"what?\" and \"huh\", and there is no reason for it to say a stock \"which bit?\" when\n    the answer above can simply be said shorter. When it cannot be said shorter the turn is left\n    to that pool: a stock \"which part?\" gives the person nothing where the answer's own opening\n    would, and the pool's \"I said: <the opening>\" is the answer's own words.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAID_AGAIN(), "match", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_turn(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_answer_to_act_on(args))(&__args) }) {
    return Py::None;
    }
    short = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_above(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_shortened(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&short.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{short}")); __args.push(short.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_AGAIN_SHORTER()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) }, "replace", args))(&__args) };
    Py::None
}
pub fn is_the_answer_shorter(args: &[Py]) -> Py {
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
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut lead: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut short: Py = Py::None;
    let mut size: Py = Py::None;
    let mut start: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when `reply` is the last answer's own words, shorter, in this module's frame.\n\n    The arbiter's never-repeat rule needs one thing this module knows and it cannot see: whether a\n    candidate that would otherwise be passed over is a second telling of the answer or a cut of\n    it. Every real candidate on a bare repeat is the answer again (\"Sure - <the paragraph>\"), and\n    the reply this module wrote for the same line is a cut of the same words - so it is the only\n    thing on the table that says anything new, and the arbiter may take it where the whole\n    paragraph is all that is beside it. Only the module's own frame is recognised and only where\n    the words behind it are the answer's own, in their own order, so a stock line that happens to\n    share a phrase is never read as a short form of anything.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    short = py_str("");
    let __iter1 = py_iter_or_raise(&g_REPEAT_LEADS());
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "startswith", args))(&__args) }) {
    short = slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&short.clone()))) {
    return Py::Bool(false);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&short.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    previous = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))) } }) {
    return Py::Bool(false);
    }
    size = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_add(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &size.clone()), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut start: Py = __item.clone(); __out.push(Py::Bool(py_eq(&slice_or_raise(&previous.clone(), &start.clone(), &op_add(&start.clone(), &size.clone()), &Py::None), &words.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _said_above(args: &[Py]) -> Py {
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
    let _ = py_str("The answer the person is asking about, or \"\" when there is not one.\n\n    An answer that is itself a repeat (\"From just now: Titanic was built in Belfast...\") is read\n    for what it says, not for the frame it came in: shortening the framed line stacked one frame\n    on another (\"The gist of it: From just now: Titanic was built in Belfast\").\n    ");
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_turn(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _unframed(args))(&__args) };
    Py::None
}
pub fn _unframed(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("The answer under a repeat's lead-in (\"From just now: ...\", \"You've got it: ...\").\n\n    Every module that hands an answer back puts a lead-in in front of it, and none of those leads\n    is part of the answer: shortening the framed line stacked one frame on another (\"The gist of\n    it: From just now: Titanic was built in Belfast\"). `bot/memory/another.FRAME` is the one place\n    that reads those leads, so this asks it rather than keeping a list of its own.\n    ");
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&g_REPEAT_LEADS());
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "startswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&reply.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&reply.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&gen_memory::memory::another::g_FRAME(), "match", args))(&__args) };
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&reply.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&reply.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::again::unframed(args))(&__args) }; if py_truthy(&__b) { __b } else { reply.clone() } };
    Py::None
}
pub fn _not_shortened(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut echo: Py = Py::None;
    let mut saved: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("The answer said shorter, or \"\" when the answer is not this module's to shorten.\n\n    The four kinds left alone are the answer that says to act now (its own words are what the\n    person needs), the answer that opens by taking bad news in (a condolence said shorter reads as\n    a form letter), the answer that names the way out (the opening is the harmless half and the\n    route is the sentence that matters), and the answer with nothing in it to cut.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANSWER_LONG())) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CARE_ANSWER(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _first_sentence(args))(&__args) }; if py_truthy(&__b) { __b } else { said.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPENS_ON_CARE(), "match", args))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_route_is_in_it(args))(&__args) }) {
    return py_str("");
    }
    short = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_shorter_way(args))(&__args) };
    echo = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _past_a_lead_in(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _first_sentence(args))(&__args) }; if py_truthy(&__b) { __b } else { said.clone() } };
    doing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(echo.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_doing_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _end_a_sentence(args))(&__args) };
    if py_truthy(&{ let __b = doing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&short.clone(), &doing.clone())) } }) {
    echo = said.clone();
    }
    saved = if py_truthy(&short.clone()) { op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&echo.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&short.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } else { Py::Int(0) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _past_a_lead_in(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) })); if !py_truthy(&__b) { __b } else { op_lt(&saved.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__A_SAVE_LEAST()); __args.push(op_floordiv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&echo.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    return py_str("");
    }
    return short.clone();
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
    let mut last: Py = Py::None;
    let _ = py_str("Read by the arbiter: on a bare \"why\" after an answer, the reason is this module's turn.\n\n    The line has no words of its own, so the module that answered first owns it by default and its\n    follow-up path hands the answer above back - \"Same as before: <the whole paragraph>\". That\n    paragraph is what the person just read, so nothing in it answers the line they typed. This\n    module reads that turn and gives the cause out of it, or says it cannot see one and asks, so\n    the arbiter asks for this declaration before the never-repeat fallback (README.md,\n    \"Interface\"). It says no to every line that is not a bare why after a substantive answer.\n\n    A card that warns of harm in the next few minutes is not claimed either, so the module that\n    wrote it hands it back whole (see `respond`). A card that says \"the same day\" is claimed: it is\n    the ordinary GP-this-week answer, and stepping aside for it handed the paragraph back.\n\n    Everything else is claimed even when no reason can be read out of the answer above, because\n    the alternative is not silence: the module that answered first hands its own answer back as\n    \"Same as before: <the whole paragraph>\", which is the paragraph the person just read. That\n    echo is what this claim exists to stop (it is the one reply the repeat fallback cannot see).\n    ");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAID_AGAIN(), "match", args))(&__args) }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_turn(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_answer_to_act_on(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_above(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_shortened(args))(&__args) }))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_bare_why(args))(&__args) }))) {
    return Py::Bool(false);
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_turn(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_answer_to_act_on(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_the_reason_cannot_be_seen(args))(&__args) }) {
    return Py::Bool(true);
    }
    return op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANSWER_LONG());
    Py::None
}
pub fn asks_about_the_last_answer(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks about the words the bot just used (\"why do you say that\").");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHY_YOU_SAID_IT(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn respond_to_a_saying_ask(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut body: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { Py::Bool(false) };
    let mut above: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The reply to \"why do you say that\", or None when the line is not one of those.\n\n    It is the bare ask in more words, so the answer is the bare ask's answer: the reason read out\n    of the turn above. The one turn that gets its own reason is a saying ask about an answer that\n    described the bot itself: the reason asked for is the bot's own, and it is said from here\n    rather than left to a pool that no longer reads the line (see WHY_THE_BOT).\n\n    Two answers do not come out of the reason reading, and both are named here rather than handed\n    back to whatever is left, because what is left is the search hedge (\"that's beyond what I know\n    well, sorry\"): an answer that described the bot (WHY_THE_BOT) and an answer that was care\n    (WHY_CARE). Everything else is the reason, and when the answer above names no cause at all the\n    turn is left exactly as it was rather than turned into \"which part of that do you want the why\n    for?\" - a shove at a person who has just been told a fact they did not ask about.\n\n    Half of the composer's work is done before this function is reached: a line that says what it\n    is asking about (\"why did you say that about my sister\") is not this ask at all, and the pool\n    for a question about the bot answers the rest.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THINKS_BACK(), "match", args))(&__args) }) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_above(args))(&__args) }, "get", args))(&__args) }))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); __args.push(understood.clone()); __kws.extend(vec![("body".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| respond(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_about_the_last_answer(args))(&__args) }))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_above(args))(&__args) }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABOUT_THE_BOT_LINE(), "match", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_THE_BOT()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    above = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_above(args))(&__args) }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(above.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CARE_ANSWER(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_CARE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(above.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_BOT_DESCRIBED_ITSELF(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_THE_BOT()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_turn(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reason_in(args))(&__args) }))) {
    let mut topics: Py = gen_memory::memory::topics::module_obj();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_above(args))(&__args) }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&topics.clone(), "said_again", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("why")); __args.push(memory.clone()); __args.push(understood.clone()); __kws.extend(vec![("body".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| respond(args))(&__args) };
    Py::None
}
pub fn _the_turn_above(args: &[Py]) -> Py {
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
    let _ = py_str("The last turn, whatever module answered it, or {} when there is none.");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_dict(vec![]);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { last.clone() } else { py_dict(vec![]) };
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut body: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { Py::Bool(false) };
    let mut act_now: Py = Py::None;
    let mut again: Py = Py::None;
    let mut handed_back: Py = Py::None;
    let mut last: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut reason: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thinks_back: Py = Py::None;
    let mut whole_thing: Py = Py::None;
    let _ = py_str("The reply to a bare \"why\" follow-up, or None when the line is not one.\n\n    `clean` is the tidied line, `memory` the conversation so far and `body` the caller's reading\n    of whether the last few lines were about their body. `understood` is unused here and kept so\n    every composer helper takes the same call shape.\n    ");
    again = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| said_again(args))(&__args) };
    if py_truthy(&again.clone()) {
    return again.clone();
    }
    thinks_back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THINKS_BACK(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_bare_why(args))(&__args) }; if py_truthy(&__b) { __b } else { thinks_back.clone() } }))) {
    return Py::None;
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_turn(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_above(args))(&__args) }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_BOT_DESCRIBED_ITSELF(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_THE_BOT()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_the_reason_cannot_be_seen(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_AGAIN()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    return Py::None;
    }
    act_now = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_answer_to_act_on(args))(&__args) };
    whole_thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WRONG(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CAUSING_IT(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHY_NOT(), "search", args))(&__args) }) {
    reason = if py_truthy(&act_now.clone()) { py_str("") } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_cause_named_outright(args))(&__args) } };
    if py_truthy(&Py::Bool(!py_truthy(&reason.clone()))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about_someone_elses_decision(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_NOT_ANSWERED()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_suggestion_was_theirs(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_NOT_ITS_OWN()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_NOT_ANSWERED()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    } else {
    reason = if py_truthy(&act_now.clone()) { py_str("") } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __kws.extend(vec![("strict".to_string(), whole_thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reason_in(args))(&__args) } };
    }
    if py_truthy(&reason.clone()) {
    said = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("because ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reason.clone(), "lower", args))(&__args) }, "startswith", args))(&__args) }) { slice_or_raise(&reason.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("because ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None) } else { reason.clone() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_as_a_sentence(args))(&__args) }) {
    pool = g_WHY_NAMED_SELF();
    } else {
    pool = if py_truthy(&Py::Bool(!py_is(&said.clone(), &reason.clone()))) { g_WHY_NAMED_BECAUSE() } else { g_WHY_NAMED() };
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _in_frame(args))(&__args) };
    reply = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("."), py_str("!"), py_str("?")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "endswith", args))(&__args) }) { reply.clone() } else { op_add(&reply.clone(), &py_str(".")) };
    reply = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&reply.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None));
    return reply.clone();
    }
    if py_truthy(&{ let __b = thinks_back.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&act_now.clone())) } }) {
    let mut topics: Py = gen_memory::memory::topics::module_obj();
    handed_back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_above(args))(&__args) }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&topics.clone(), "said_again", args))(&__args) };
    if py_truthy(&handed_back.clone()) {
    return handed_back.clone();
    }
    }
    if py_truthy(&body.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_BODY()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&act_now.clone())); if !py_truthy(&__b) { __b } else { { let __b = whole_thing.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gave_a_method(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_a_fault(args))(&__args) } } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_METHOD()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&whole_thing.clone()) {
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_a_fault(args))(&__args) }) { g_WRONG_ANSWERED() } else { g_WRONG_NOT_A_FAULT() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHY_ANSWERED()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.why".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_BOT_LINE" => Some(g_ABOUT_THE_BOT_LINE()),
            "ACT_NOW" => Some(g_ACT_NOW()),
            "ACT_NOW_SOON" => Some(g_ACT_NOW_SOON()),
            "AFTER_THE_MARK" => Some(g_AFTER_THE_MARK()),
            "AGAIN_SHORTER" => Some(g_AGAIN_SHORTER()),
            "AGAIN_WORDS" => Some(g_AGAIN_WORDS()),
            "AND_ADVICE" => Some(g_AND_ADVICE()),
            "AND_STEP" => Some(g_AND_STEP()),
            "ANSWERED" => Some(g_ANSWERED()),
            "ANSWER_LONG" => Some(g_ANSWER_LONG()),
            "A_GENERAL_CLAIM" => Some(g_A_GENERAL_CLAIM()),
            "A_LEAD_IN" => Some(g_A_LEAD_IN()),
            "A_NOD_IS_SHORT" => Some(g_A_NOD_IS_SHORT()),
            "A_NOD_OPENS" => Some(g_A_NOD_OPENS()),
            "A_PLACEHOLDER" => Some(g_A_PLACEHOLDER()),
            "A_THING_NAMED_BEFORE_A_COLON" => Some(g_A_THING_NAMED_BEFORE_A_COLON()),
            "BARE_WHY" => Some(g_BARE_WHY()),
            "CARE_ANSWER" => Some(g_CARE_ANSWER()),
            "CAUSE" => Some(g_CAUSE()),
            "CAUSING_IT" => Some(g_CAUSING_IT()),
            "CHANGE" => Some(g_CHANGE()),
            "CLAIM_BEFORE_COLON" => Some(g_CLAIM_BEFORE_COLON()),
            "CLAIM_SUBJECT" => Some(g_CLAIM_SUBJECT()),
            "CLAUSE_END" => Some(g_CLAUSE_END()),
            "CONDITIONAL" => Some(g_CONDITIONAL()),
            "CONDITIONAL_SENTENCE" => Some(g_CONDITIONAL_SENTENCE()),
            "DANGLING" => Some(g_DANGLING()),
            "DOSE" => Some(g_DOSE()),
            "ENDS_THE_CLASS" => Some(g_ENDS_THE_CLASS()),
            "END_OF_SENTENCE" => Some(g_END_OF_SENTENCE()),
            "EXAMPLE_IN_QUOTES" => Some(g_EXAMPLE_IN_QUOTES()),
            "EXPLAINS" => Some(g_EXPLAINS()),
            "FIRST_SENTENCES" => Some(g_FIRST_SENTENCES()),
            "HEDGED" => Some(g_HEDGED()),
            "IMPERATIVE" => Some(g_IMPERATIVE()),
            "INSTRUCTION" => Some(g_INSTRUCTION()),
            "KEEP_CAPITAL" => Some(g_KEEP_CAPITAL()),
            "LEADING_MARKER" => Some(g_LEADING_MARKER()),
            "LIFE" => Some(g_LIFE()),
            "NAMED_CAUSE" => Some(g_NAMED_CAUSE()),
            "NEGATIVE" => Some(g_NEGATIVE()),
            "NO_CUT" => Some(g_NO_CUT()),
            "OPENS_ON_CARE" => Some(g_OPENS_ON_CARE()),
            "PARTS" => Some(g_PARTS()),
            "REPEAT_LEADS" => Some(g_REPEAT_LEADS()),
            "RESULT_NUMBER" => Some(g_RESULT_NUMBER()),
            "SAID_AGAIN" => Some(g_SAID_AGAIN()),
            "SAID_NO" => Some(g_SAID_NO()),
            "SAYS_WHAT_IT_DOES" => Some(g_SAYS_WHAT_IT_DOES()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SHORT_OF_A_CAUSE" => Some(g_SHORT_OF_A_CAUSE()),
            "SO_ADVICE" => Some(g_SO_ADVICE()),
            "SO_FROM_HERE" => Some(g_SO_FROM_HERE()),
            "SO_REASON" => Some(g_SO_REASON()),
            "STEP_VERBS" => Some(g_STEP_VERBS()),
            "SUBSTANCE" => Some(g_SUBSTANCE()),
            "SUGGESTED_VERBS" => Some(g_SUGGESTED_VERBS()),
            "THEIR_DECISION" => Some(g_THEIR_DECISION()),
            "THEIR_SUGGESTION" => Some(g_THEIR_SUGGESTION()),
            "THE_BOT_DESCRIBED_ITSELF" => Some(g_THE_BOT_DESCRIBED_ITSELF()),
            "THE_FIX" => Some(g_THE_FIX()),
            "THE_REASON_POINTS_AT_THE_ADVICE" => Some(g_THE_REASON_POINTS_AT_THE_ADVICE()),
            "THINKS_BACK" => Some(g_THINKS_BACK()),
            "WHAT_IT_TAKES" => Some(g_WHAT_IT_TAKES()),
            "WHEN_CAUSE" => Some(g_WHEN_CAUSE()),
            "WHY_AGAIN" => Some(g_WHY_AGAIN()),
            "WHY_ANSWERED" => Some(g_WHY_ANSWERED()),
            "WHY_BODY" => Some(g_WHY_BODY()),
            "WHY_CARE" => Some(g_WHY_CARE()),
            "WHY_END" => Some(g_WHY_END()),
            "WHY_FRONT" => Some(g_WHY_FRONT()),
            "WHY_METHOD" => Some(g_WHY_METHOD()),
            "WHY_NAMED" => Some(g_WHY_NAMED()),
            "WHY_NAMED_BECAUSE" => Some(g_WHY_NAMED_BECAUSE()),
            "WHY_NAMED_SELF" => Some(g_WHY_NAMED_SELF()),
            "WHY_NOT" => Some(g_WHY_NOT()),
            "WHY_NOT_ANSWERED" => Some(g_WHY_NOT_ANSWERED()),
            "WHY_NOT_ITS_OWN" => Some(g_WHY_NOT_ITS_OWN()),
            "WHY_THE_BOT" => Some(g_WHY_THE_BOT()),
            "WHY_YOU_SAID_IT" => Some(g_WHY_YOU_SAID_IT()),
            "WRONG" => Some(g_WRONG()),
            "WRONG_ANSWERED" => Some(g_WRONG_ANSWERED()),
            "WRONG_NOT_A_FAULT" => Some(g_WRONG_NOT_A_FAULT()),
            "_ANNOUNCES" => Some(g__ANNOUNCES()),
            "_A_CATEGORY" => Some(g__A_CATEGORY()),
            "_A_CAUSE_OPENS" => Some(g__A_CAUSE_OPENS()),
            "_A_CLAUSE_LEAST" => Some(g__A_CLAUSE_LEAST()),
            "_A_DOING" => Some(g__A_DOING()),
            "_A_DOING_WINDOW" => Some(g__A_DOING_WINDOW()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_LEAD_IN" => Some(g__A_LEAD_IN()),
            "_A_LONG_SENTENCE" => Some(g__A_LONG_SENTENCE()),
            "_A_PARTICIPLE" => Some(g__A_PARTICIPLE()),
            "_A_REASON_LEAST" => Some(g__A_REASON_LEAST()),
            "_A_SAVE_LEAST" => Some(g__A_SAVE_LEAST()),
            "_A_STRIDE" => Some(g__A_STRIDE()),
            "_A_VERB" => Some(g__A_VERB()),
            "_EXPLAINS" => Some(g__EXPLAINS()),
            "_LEANS_BACK" => Some(g__LEANS_BACK()),
            "_LEANS_BACK_WINDOW" => Some(g__LEANS_BACK_WINDOW()),
            "_NEEDS_A_SUBJECT" => Some(g__NEEDS_A_SUBJECT()),
            "_OPENS_ON_A_FIGURE" => Some(g__OPENS_ON_A_FIGURE()),
            "_POINTS_FORWARD" => Some(g__POINTS_FORWARD()),
            "_POINTS_FORWARD_ANY" => Some(g__POINTS_FORWARD_ANY()),
            "_POINTS_FORWARD_LEAST" => Some(g__POINTS_FORWARD_LEAST()),
            "_SAYS_HOW" => Some(g__SAYS_HOW()),
            "_VERB_SHAPE" => Some(g__VERB_SHAPE()),
            "_a_cause_named_outright" => Some(py_func("_a_cause_named_outright", |args: &[Py]| _a_cause_named_outright(args))),
            "_a_clause_of" => Some(py_func("_a_clause_of", |args: &[Py]| _a_clause_of(args))),
            "_a_reason_in" => Some(py_func("_a_reason_in", |args: &[Py]| _a_reason_in(args))),
            "_a_shorter_way" => Some(py_func("_a_shorter_way", |args: &[Py]| _a_shorter_way(args))),
            "_about_someone_elses_decision" => Some(py_func("_about_someone_elses_decision", |args: &[Py]| _about_someone_elses_decision(args))),
            "_an_answer_to_act_on" => Some(py_func("_an_answer_to_act_on", |args: &[Py]| _an_answer_to_act_on(args))),
            "_announces_the_list" => Some(py_func("_announces_the_list", |args: &[Py]| _announces_the_list(args))),
            "_answered_nothing" => Some(py_func("_answered_nothing", |args: &[Py]| _answered_nothing(args))),
            "_brackets_closed" => Some(py_func("_brackets_closed", |args: &[Py]| _brackets_closed(args))),
            "_carries_a_verb" => Some(py_func("_carries_a_verb", |args: &[Py]| _carries_a_verb(args))),
            "_carries_the_fix" => Some(py_func("_carries_the_fix", |args: &[Py]| _carries_the_fix(args))),
            "_cause_clause" => Some(py_func("_cause_clause", |args: &[Py]| _cause_clause(args))),
            "_cut_long" => Some(py_func("_cut_long", |args: &[Py]| _cut_long(args))),
            "_end_a_sentence" => Some(py_func("_end_a_sentence", |args: &[Py]| _end_a_sentence(args))),
            "_first_sentence" => Some(py_func("_first_sentence", |args: &[Py]| _first_sentence(args))),
            "_gave_a_method" => Some(py_func("_gave_a_method", |args: &[Py]| _gave_a_method(args))),
            "_in_frame" => Some(py_func("_in_frame", |args: &[Py]| _in_frame(args))),
            "_inside_a_quotation" => Some(py_func("_inside_a_quotation", |args: &[Py]| _inside_a_quotation(args))),
            "_is_bare_why" => Some(py_func("_is_bare_why", |args: &[Py]| _is_bare_why(args))),
            "_is_the_answer_itself" => Some(py_func("_is_the_answer_itself", |args: &[Py]| _is_the_answer_itself(args))),
            "_last_turn" => Some(py_func("_last_turn", |args: &[Py]| _last_turn(args))),
            "_leans_back" => Some(py_func("_leans_back", |args: &[Py]| _leans_back(args))),
            "_named_as_a_sentence" => Some(py_func("_named_as_a_sentence", |args: &[Py]| _named_as_a_sentence(args))),
            "_named_by_definition" => Some(py_func("_named_by_definition", |args: &[Py]| _named_by_definition(args))),
            "_names_no_particular" => Some(py_func("_names_no_particular", |args: &[Py]| _names_no_particular(args))),
            "_needs_its_subject" => Some(py_func("_needs_its_subject", |args: &[Py]| _needs_its_subject(args))),
            "_not_shortened" => Some(py_func("_not_shortened", |args: &[Py]| _not_shortened(args))),
            "_opens_a_doing" => Some(py_func("_opens_a_doing", |args: &[Py]| _opens_a_doing(args))),
            "_opens_on_a_figure" => Some(py_func("_opens_on_a_figure", |args: &[Py]| _opens_on_a_figure(args))),
            "_past_a_lead_in" => Some(py_func("_past_a_lead_in", |args: &[Py]| _past_a_lead_in(args))),
            "_points_at_what_follows" => Some(py_func("_points_at_what_follows", |args: &[Py]| _points_at_what_follows(args))),
            "_reads_a_fault" => Some(py_func("_reads_a_fault", |args: &[Py]| _reads_a_fault(args))),
            "_reason_in" => Some(py_func("_reason_in", |args: &[Py]| _reason_in(args))),
            "_said_above" => Some(py_func("_said_above", |args: &[Py]| _said_above(args))),
            "_said_the_reason_cannot_be_seen" => Some(py_func("_said_the_reason_cannot_be_seen", |args: &[Py]| _said_the_reason_cannot_be_seen(args))),
            "_says_how_it_works" => Some(py_func("_says_how_it_works", |args: &[Py]| _says_how_it_works(args))),
            "_the_class_it_belongs_to" => Some(py_func("_the_class_it_belongs_to", |args: &[Py]| _the_class_it_belongs_to(args))),
            "_the_doing_of" => Some(py_func("_the_doing_of", |args: &[Py]| _the_doing_of(args))),
            "_the_half_that_does_it" => Some(py_func("_the_half_that_does_it", |args: &[Py]| _the_half_that_does_it(args))),
            "_the_reason" => Some(py_func("_the_reason", |args: &[Py]| _the_reason(args))),
            "_the_route_is_in_it" => Some(py_func("_the_route_is_in_it", |args: &[Py]| _the_route_is_in_it(args))),
            "_the_suggestion_was_theirs" => Some(py_func("_the_suggestion_was_theirs", |args: &[Py]| _the_suggestion_was_theirs(args))),
            "_the_turn_above" => Some(py_func("_the_turn_above", |args: &[Py]| _the_turn_above(args))),
            "_trigger_in" => Some(py_func("_trigger_in", |args: &[Py]| _trigger_in(args))),
            "_unframed" => Some(py_func("_unframed", |args: &[Py]| _unframed(args))),
            "_without_examples" => Some(py_func("_without_examples", |args: &[Py]| _without_examples(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "asks_about_the_last_answer" => Some(py_func("asks_about_the_last_answer", |args: &[Py]| asks_about_the_last_answer(args))),
            "is_a_reason_line" => Some(py_func("is_a_reason_line", |args: &[Py]| is_a_reason_line(args))),
            "is_the_answer_shorter" => Some(py_func("is_the_answer_shorter", |args: &[Py]| is_the_answer_shorter(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "respond_to_a_saying_ask" => Some(py_func("respond_to_a_saying_ask", |args: &[Py]| respond_to_a_saying_ask(args))),
            "said_again" => Some(py_func("said_again", |args: &[Py]| said_again(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THE_BOT_LINE" => s_ABOUT_THE_BOT_LINE(v),
            "ACT_NOW" => s_ACT_NOW(v),
            "ACT_NOW_SOON" => s_ACT_NOW_SOON(v),
            "AFTER_THE_MARK" => s_AFTER_THE_MARK(v),
            "AGAIN_SHORTER" => s_AGAIN_SHORTER(v),
            "AGAIN_WORDS" => s_AGAIN_WORDS(v),
            "AND_ADVICE" => s_AND_ADVICE(v),
            "AND_STEP" => s_AND_STEP(v),
            "ANSWERED" => s_ANSWERED(v),
            "ANSWER_LONG" => s_ANSWER_LONG(v),
            "A_GENERAL_CLAIM" => s_A_GENERAL_CLAIM(v),
            "A_LEAD_IN" => s_A_LEAD_IN(v),
            "A_NOD_IS_SHORT" => s_A_NOD_IS_SHORT(v),
            "A_NOD_OPENS" => s_A_NOD_OPENS(v),
            "A_PLACEHOLDER" => s_A_PLACEHOLDER(v),
            "A_THING_NAMED_BEFORE_A_COLON" => s_A_THING_NAMED_BEFORE_A_COLON(v),
            "BARE_WHY" => s_BARE_WHY(v),
            "CARE_ANSWER" => s_CARE_ANSWER(v),
            "CAUSE" => s_CAUSE(v),
            "CAUSING_IT" => s_CAUSING_IT(v),
            "CHANGE" => s_CHANGE(v),
            "CLAIM_BEFORE_COLON" => s_CLAIM_BEFORE_COLON(v),
            "CLAIM_SUBJECT" => s_CLAIM_SUBJECT(v),
            "CLAUSE_END" => s_CLAUSE_END(v),
            "CONDITIONAL" => s_CONDITIONAL(v),
            "CONDITIONAL_SENTENCE" => s_CONDITIONAL_SENTENCE(v),
            "DANGLING" => s_DANGLING(v),
            "DOSE" => s_DOSE(v),
            "ENDS_THE_CLASS" => s_ENDS_THE_CLASS(v),
            "END_OF_SENTENCE" => s_END_OF_SENTENCE(v),
            "EXAMPLE_IN_QUOTES" => s_EXAMPLE_IN_QUOTES(v),
            "EXPLAINS" => s_EXPLAINS(v),
            "FIRST_SENTENCES" => s_FIRST_SENTENCES(v),
            "HEDGED" => s_HEDGED(v),
            "IMPERATIVE" => s_IMPERATIVE(v),
            "INSTRUCTION" => s_INSTRUCTION(v),
            "KEEP_CAPITAL" => s_KEEP_CAPITAL(v),
            "LEADING_MARKER" => s_LEADING_MARKER(v),
            "LIFE" => s_LIFE(v),
            "NAMED_CAUSE" => s_NAMED_CAUSE(v),
            "NEGATIVE" => s_NEGATIVE(v),
            "NO_CUT" => s_NO_CUT(v),
            "OPENS_ON_CARE" => s_OPENS_ON_CARE(v),
            "PARTS" => s_PARTS(v),
            "REPEAT_LEADS" => s_REPEAT_LEADS(v),
            "RESULT_NUMBER" => s_RESULT_NUMBER(v),
            "SAID_AGAIN" => s_SAID_AGAIN(v),
            "SAID_NO" => s_SAID_NO(v),
            "SAYS_WHAT_IT_DOES" => s_SAYS_WHAT_IT_DOES(v),
            "SENTENCE" => s_SENTENCE(v),
            "SHORT_OF_A_CAUSE" => s_SHORT_OF_A_CAUSE(v),
            "SO_ADVICE" => s_SO_ADVICE(v),
            "SO_FROM_HERE" => s_SO_FROM_HERE(v),
            "SO_REASON" => s_SO_REASON(v),
            "STEP_VERBS" => s_STEP_VERBS(v),
            "SUBSTANCE" => s_SUBSTANCE(v),
            "SUGGESTED_VERBS" => s_SUGGESTED_VERBS(v),
            "THEIR_DECISION" => s_THEIR_DECISION(v),
            "THEIR_SUGGESTION" => s_THEIR_SUGGESTION(v),
            "THE_BOT_DESCRIBED_ITSELF" => s_THE_BOT_DESCRIBED_ITSELF(v),
            "THE_FIX" => s_THE_FIX(v),
            "THE_REASON_POINTS_AT_THE_ADVICE" => s_THE_REASON_POINTS_AT_THE_ADVICE(v),
            "THINKS_BACK" => s_THINKS_BACK(v),
            "WHAT_IT_TAKES" => s_WHAT_IT_TAKES(v),
            "WHEN_CAUSE" => s_WHEN_CAUSE(v),
            "WHY_AGAIN" => s_WHY_AGAIN(v),
            "WHY_ANSWERED" => s_WHY_ANSWERED(v),
            "WHY_BODY" => s_WHY_BODY(v),
            "WHY_CARE" => s_WHY_CARE(v),
            "WHY_END" => s_WHY_END(v),
            "WHY_FRONT" => s_WHY_FRONT(v),
            "WHY_METHOD" => s_WHY_METHOD(v),
            "WHY_NAMED" => s_WHY_NAMED(v),
            "WHY_NAMED_BECAUSE" => s_WHY_NAMED_BECAUSE(v),
            "WHY_NAMED_SELF" => s_WHY_NAMED_SELF(v),
            "WHY_NOT" => s_WHY_NOT(v),
            "WHY_NOT_ANSWERED" => s_WHY_NOT_ANSWERED(v),
            "WHY_NOT_ITS_OWN" => s_WHY_NOT_ITS_OWN(v),
            "WHY_THE_BOT" => s_WHY_THE_BOT(v),
            "WHY_YOU_SAID_IT" => s_WHY_YOU_SAID_IT(v),
            "WRONG" => s_WRONG(v),
            "WRONG_ANSWERED" => s_WRONG_ANSWERED(v),
            "WRONG_NOT_A_FAULT" => s_WRONG_NOT_A_FAULT(v),
            "_ANNOUNCES" => s__ANNOUNCES(v),
            "_A_CATEGORY" => s__A_CATEGORY(v),
            "_A_CAUSE_OPENS" => s__A_CAUSE_OPENS(v),
            "_A_CLAUSE_LEAST" => s__A_CLAUSE_LEAST(v),
            "_A_DOING" => s__A_DOING(v),
            "_A_DOING_WINDOW" => s__A_DOING_WINDOW(v),
            "_A_FIGURE" => s__A_FIGURE(v),
            "_A_LEAD_IN" => s__A_LEAD_IN(v),
            "_A_LONG_SENTENCE" => s__A_LONG_SENTENCE(v),
            "_A_PARTICIPLE" => s__A_PARTICIPLE(v),
            "_A_REASON_LEAST" => s__A_REASON_LEAST(v),
            "_A_SAVE_LEAST" => s__A_SAVE_LEAST(v),
            "_A_STRIDE" => s__A_STRIDE(v),
            "_A_VERB" => s__A_VERB(v),
            "_EXPLAINS" => s__EXPLAINS(v),
            "_LEANS_BACK" => s__LEANS_BACK(v),
            "_LEANS_BACK_WINDOW" => s__LEANS_BACK_WINDOW(v),
            "_NEEDS_A_SUBJECT" => s__NEEDS_A_SUBJECT(v),
            "_OPENS_ON_A_FIGURE" => s__OPENS_ON_A_FIGURE(v),
            "_POINTS_FORWARD" => s__POINTS_FORWARD(v),
            "_POINTS_FORWARD_ANY" => s__POINTS_FORWARD_ANY(v),
            "_POINTS_FORWARD_LEAST" => s__POINTS_FORWARD_LEAST(v),
            "_SAYS_HOW" => s__SAYS_HOW(v),
            "_VERB_SHAPE" => s__VERB_SHAPE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.why", module_obj);
    pyrt::xrefs::reg_fn("compose.why", "_named_as_a_sentence", py_func("_named_as_a_sentence", |args: &[Py]| _named_as_a_sentence(args)));
    pyrt::xrefs::reg_fn("compose.why", "_reads_a_fault", py_func("_reads_a_fault", |args: &[Py]| _reads_a_fault(args)));
    pyrt::xrefs::reg_fn("compose.why", "_about_someone_elses_decision", py_func("_about_someone_elses_decision", |args: &[Py]| _about_someone_elses_decision(args)));
    pyrt::xrefs::reg_fn("compose.why", "_the_suggestion_was_theirs", py_func("_the_suggestion_was_theirs", |args: &[Py]| _the_suggestion_was_theirs(args)));
    pyrt::xrefs::reg_fn("compose.why", "_in_frame", py_func("_in_frame", |args: &[Py]| _in_frame(args)));
    pyrt::xrefs::reg_fn("compose.why", "is_a_reason_line", py_func("is_a_reason_line", |args: &[Py]| is_a_reason_line(args)));
    pyrt::xrefs::reg_fn("compose.why", "_said_the_reason_cannot_be_seen", py_func("_said_the_reason_cannot_be_seen", |args: &[Py]| _said_the_reason_cannot_be_seen(args)));
    pyrt::xrefs::reg_fn("compose.why", "_is_bare_why", py_func("_is_bare_why", |args: &[Py]| _is_bare_why(args)));
    pyrt::xrefs::reg_fn("compose.why", "_last_turn", py_func("_last_turn", |args: &[Py]| _last_turn(args)));
    pyrt::xrefs::reg_fn("compose.why", "_answered_nothing", py_func("_answered_nothing", |args: &[Py]| _answered_nothing(args)));
    pyrt::xrefs::reg_fn("compose.why", "_gave_a_method", py_func("_gave_a_method", |args: &[Py]| _gave_a_method(args)));
    pyrt::xrefs::reg_fn("compose.why", "_a_cause_named_outright", py_func("_a_cause_named_outright", |args: &[Py]| _a_cause_named_outright(args)));
    pyrt::xrefs::reg_fn("compose.why", "_reason_in", py_func("_reason_in", |args: &[Py]| _reason_in(args)));
    pyrt::xrefs::reg_fn("compose.why", "_without_examples", py_func("_without_examples", |args: &[Py]| _without_examples(args)));
    pyrt::xrefs::reg_fn("compose.why", "_inside_a_quotation", py_func("_inside_a_quotation", |args: &[Py]| _inside_a_quotation(args)));
    pyrt::xrefs::reg_fn("compose.why", "_a_reason_in", py_func("_a_reason_in", |args: &[Py]| _a_reason_in(args)));
    pyrt::xrefs::reg_fn("compose.why", "_is_the_answer_itself", py_func("_is_the_answer_itself", |args: &[Py]| _is_the_answer_itself(args)));
    pyrt::xrefs::reg_fn("compose.why", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.why", "_carries_the_fix", py_func("_carries_the_fix", |args: &[Py]| _carries_the_fix(args)));
    pyrt::xrefs::reg_fn("compose.why", "_trigger_in", py_func("_trigger_in", |args: &[Py]| _trigger_in(args)));
    pyrt::xrefs::reg_fn("compose.why", "_cause_clause", py_func("_cause_clause", |args: &[Py]| _cause_clause(args)));
    pyrt::xrefs::reg_fn("compose.why", "_the_class_it_belongs_to", py_func("_the_class_it_belongs_to", |args: &[Py]| _the_class_it_belongs_to(args)));
    pyrt::xrefs::reg_fn("compose.why", "_named_by_definition", py_func("_named_by_definition", |args: &[Py]| _named_by_definition(args)));
    pyrt::xrefs::reg_fn("compose.why", "_cut_long", py_func("_cut_long", |args: &[Py]| _cut_long(args)));
    pyrt::xrefs::reg_fn("compose.why", "_an_answer_to_act_on", py_func("_an_answer_to_act_on", |args: &[Py]| _an_answer_to_act_on(args)));
    pyrt::xrefs::reg_fn("compose.why", "_first_sentence", py_func("_first_sentence", |args: &[Py]| _first_sentence(args)));
    pyrt::xrefs::reg_fn("compose.why", "_a_shorter_way", py_func("_a_shorter_way", |args: &[Py]| _a_shorter_way(args)));
    pyrt::xrefs::reg_fn("compose.why", "_past_a_lead_in", py_func("_past_a_lead_in", |args: &[Py]| _past_a_lead_in(args)));
    pyrt::xrefs::reg_fn("compose.why", "_end_a_sentence", py_func("_end_a_sentence", |args: &[Py]| _end_a_sentence(args)));
    pyrt::xrefs::reg_fn("compose.why", "_a_clause_of", py_func("_a_clause_of", |args: &[Py]| _a_clause_of(args)));
    pyrt::xrefs::reg_fn("compose.why", "_the_doing_of", py_func("_the_doing_of", |args: &[Py]| _the_doing_of(args)));
    pyrt::xrefs::reg_fn("compose.why", "_opens_a_doing", py_func("_opens_a_doing", |args: &[Py]| _opens_a_doing(args)));
    pyrt::xrefs::reg_fn("compose.why", "_says_how_it_works", py_func("_says_how_it_works", |args: &[Py]| _says_how_it_works(args)));
    pyrt::xrefs::reg_fn("compose.why", "_announces_the_list", py_func("_announces_the_list", |args: &[Py]| _announces_the_list(args)));
    pyrt::xrefs::reg_fn("compose.why", "_needs_its_subject", py_func("_needs_its_subject", |args: &[Py]| _needs_its_subject(args)));
    pyrt::xrefs::reg_fn("compose.why", "_carries_a_verb", py_func("_carries_a_verb", |args: &[Py]| _carries_a_verb(args)));
    pyrt::xrefs::reg_fn("compose.why", "_leans_back", py_func("_leans_back", |args: &[Py]| _leans_back(args)));
    pyrt::xrefs::reg_fn("compose.why", "_the_reason", py_func("_the_reason", |args: &[Py]| _the_reason(args)));
    pyrt::xrefs::reg_fn("compose.why", "_opens_on_a_figure", py_func("_opens_on_a_figure", |args: &[Py]| _opens_on_a_figure(args)));
    pyrt::xrefs::reg_fn("compose.why", "_names_no_particular", py_func("_names_no_particular", |args: &[Py]| _names_no_particular(args)));
    pyrt::xrefs::reg_fn("compose.why", "_the_half_that_does_it", py_func("_the_half_that_does_it", |args: &[Py]| _the_half_that_does_it(args)));
    pyrt::xrefs::reg_fn("compose.why", "_points_at_what_follows", py_func("_points_at_what_follows", |args: &[Py]| _points_at_what_follows(args)));
    pyrt::xrefs::reg_fn("compose.why", "_brackets_closed", py_func("_brackets_closed", |args: &[Py]| _brackets_closed(args)));
    pyrt::xrefs::reg_fn("compose.why", "_the_route_is_in_it", py_func("_the_route_is_in_it", |args: &[Py]| _the_route_is_in_it(args)));
    pyrt::xrefs::reg_fn("compose.why", "said_again", py_func("said_again", |args: &[Py]| said_again(args)));
    pyrt::xrefs::reg_fn("compose.why", "is_the_answer_shorter", py_func("is_the_answer_shorter", |args: &[Py]| is_the_answer_shorter(args)));
    pyrt::xrefs::reg_fn("compose.why", "_said_above", py_func("_said_above", |args: &[Py]| _said_above(args)));
    pyrt::xrefs::reg_fn("compose.why", "_unframed", py_func("_unframed", |args: &[Py]| _unframed(args)));
    pyrt::xrefs::reg_fn("compose.why", "_not_shortened", py_func("_not_shortened", |args: &[Py]| _not_shortened(args)));
    pyrt::xrefs::reg_fn("compose.why", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_fn("compose.why", "asks_about_the_last_answer", py_func("asks_about_the_last_answer", |args: &[Py]| asks_about_the_last_answer(args)));
    pyrt::xrefs::reg_fn("compose.why", "respond_to_a_saying_ask", py_func("respond_to_a_saying_ask", |args: &[Py]| respond_to_a_saying_ask(args)));
    pyrt::xrefs::reg_fn("compose.why", "_the_turn_above", py_func("_the_turn_above", |args: &[Py]| _the_turn_above(args)));
    pyrt::xrefs::reg_fn("compose.why", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("compose.why", "ABOUT_THE_BOT_LINE", g_ABOUT_THE_BOT_LINE, s_ABOUT_THE_BOT_LINE);
    pyrt::xrefs::reg_access("compose.why", "ACT_NOW", g_ACT_NOW, s_ACT_NOW);
    pyrt::xrefs::reg_access("compose.why", "ACT_NOW_SOON", g_ACT_NOW_SOON, s_ACT_NOW_SOON);
    pyrt::xrefs::reg_access("compose.why", "AFTER_THE_MARK", g_AFTER_THE_MARK, s_AFTER_THE_MARK);
    pyrt::xrefs::reg_access("compose.why", "AGAIN_SHORTER", g_AGAIN_SHORTER, s_AGAIN_SHORTER);
    pyrt::xrefs::reg_access("compose.why", "AGAIN_WORDS", g_AGAIN_WORDS, s_AGAIN_WORDS);
    pyrt::xrefs::reg_access("compose.why", "AND_ADVICE", g_AND_ADVICE, s_AND_ADVICE);
    pyrt::xrefs::reg_access("compose.why", "AND_STEP", g_AND_STEP, s_AND_STEP);
    pyrt::xrefs::reg_access("compose.why", "ANSWERED", g_ANSWERED, s_ANSWERED);
    pyrt::xrefs::reg_access("compose.why", "ANSWER_LONG", g_ANSWER_LONG, s_ANSWER_LONG);
    pyrt::xrefs::reg_access("compose.why", "A_GENERAL_CLAIM", g_A_GENERAL_CLAIM, s_A_GENERAL_CLAIM);
    pyrt::xrefs::reg_access("compose.why", "A_LEAD_IN", g_A_LEAD_IN, s_A_LEAD_IN);
    pyrt::xrefs::reg_access("compose.why", "A_NOD_IS_SHORT", g_A_NOD_IS_SHORT, s_A_NOD_IS_SHORT);
    pyrt::xrefs::reg_access("compose.why", "A_NOD_OPENS", g_A_NOD_OPENS, s_A_NOD_OPENS);
    pyrt::xrefs::reg_access("compose.why", "A_PLACEHOLDER", g_A_PLACEHOLDER, s_A_PLACEHOLDER);
    pyrt::xrefs::reg_access("compose.why", "A_THING_NAMED_BEFORE_A_COLON", g_A_THING_NAMED_BEFORE_A_COLON, s_A_THING_NAMED_BEFORE_A_COLON);
    pyrt::xrefs::reg_access("compose.why", "BARE_WHY", g_BARE_WHY, s_BARE_WHY);
    pyrt::xrefs::reg_access("compose.why", "CARE_ANSWER", g_CARE_ANSWER, s_CARE_ANSWER);
    pyrt::xrefs::reg_access("compose.why", "CAUSE", g_CAUSE, s_CAUSE);
    pyrt::xrefs::reg_access("compose.why", "CAUSING_IT", g_CAUSING_IT, s_CAUSING_IT);
    pyrt::xrefs::reg_access("compose.why", "CHANGE", g_CHANGE, s_CHANGE);
    pyrt::xrefs::reg_access("compose.why", "CLAIM_BEFORE_COLON", g_CLAIM_BEFORE_COLON, s_CLAIM_BEFORE_COLON);
    pyrt::xrefs::reg_access("compose.why", "CLAIM_SUBJECT", g_CLAIM_SUBJECT, s_CLAIM_SUBJECT);
    pyrt::xrefs::reg_access("compose.why", "CLAUSE_END", g_CLAUSE_END, s_CLAUSE_END);
    pyrt::xrefs::reg_access("compose.why", "CONDITIONAL", g_CONDITIONAL, s_CONDITIONAL);
    pyrt::xrefs::reg_access("compose.why", "CONDITIONAL_SENTENCE", g_CONDITIONAL_SENTENCE, s_CONDITIONAL_SENTENCE);
    pyrt::xrefs::reg_access("compose.why", "DANGLING", g_DANGLING, s_DANGLING);
    pyrt::xrefs::reg_access("compose.why", "DOSE", g_DOSE, s_DOSE);
    pyrt::xrefs::reg_access("compose.why", "ENDS_THE_CLASS", g_ENDS_THE_CLASS, s_ENDS_THE_CLASS);
    pyrt::xrefs::reg_access("compose.why", "END_OF_SENTENCE", g_END_OF_SENTENCE, s_END_OF_SENTENCE);
    pyrt::xrefs::reg_access("compose.why", "EXAMPLE_IN_QUOTES", g_EXAMPLE_IN_QUOTES, s_EXAMPLE_IN_QUOTES);
    pyrt::xrefs::reg_access("compose.why", "EXPLAINS", g_EXPLAINS, s_EXPLAINS);
    pyrt::xrefs::reg_access("compose.why", "FIRST_SENTENCES", g_FIRST_SENTENCES, s_FIRST_SENTENCES);
    pyrt::xrefs::reg_access("compose.why", "HEDGED", g_HEDGED, s_HEDGED);
    pyrt::xrefs::reg_access("compose.why", "IMPERATIVE", g_IMPERATIVE, s_IMPERATIVE);
    pyrt::xrefs::reg_access("compose.why", "INSTRUCTION", g_INSTRUCTION, s_INSTRUCTION);
    pyrt::xrefs::reg_access("compose.why", "KEEP_CAPITAL", g_KEEP_CAPITAL, s_KEEP_CAPITAL);
    pyrt::xrefs::reg_access("compose.why", "LEADING_MARKER", g_LEADING_MARKER, s_LEADING_MARKER);
    pyrt::xrefs::reg_access("compose.why", "LIFE", g_LIFE, s_LIFE);
    pyrt::xrefs::reg_access("compose.why", "NAMED_CAUSE", g_NAMED_CAUSE, s_NAMED_CAUSE);
    pyrt::xrefs::reg_access("compose.why", "NEGATIVE", g_NEGATIVE, s_NEGATIVE);
    pyrt::xrefs::reg_access("compose.why", "NO_CUT", g_NO_CUT, s_NO_CUT);
    pyrt::xrefs::reg_access("compose.why", "OPENS_ON_CARE", g_OPENS_ON_CARE, s_OPENS_ON_CARE);
    pyrt::xrefs::reg_access("compose.why", "PARTS", g_PARTS, s_PARTS);
    pyrt::xrefs::reg_access("compose.why", "REPEAT_LEADS", g_REPEAT_LEADS, s_REPEAT_LEADS);
    pyrt::xrefs::reg_access("compose.why", "RESULT_NUMBER", g_RESULT_NUMBER, s_RESULT_NUMBER);
    pyrt::xrefs::reg_access("compose.why", "SAID_AGAIN", g_SAID_AGAIN, s_SAID_AGAIN);
    pyrt::xrefs::reg_access("compose.why", "SAID_NO", g_SAID_NO, s_SAID_NO);
    pyrt::xrefs::reg_access("compose.why", "SAYS_WHAT_IT_DOES", g_SAYS_WHAT_IT_DOES, s_SAYS_WHAT_IT_DOES);
    pyrt::xrefs::reg_access("compose.why", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("compose.why", "SHORT_OF_A_CAUSE", g_SHORT_OF_A_CAUSE, s_SHORT_OF_A_CAUSE);
    pyrt::xrefs::reg_access("compose.why", "SO_ADVICE", g_SO_ADVICE, s_SO_ADVICE);
    pyrt::xrefs::reg_access("compose.why", "SO_FROM_HERE", g_SO_FROM_HERE, s_SO_FROM_HERE);
    pyrt::xrefs::reg_access("compose.why", "SO_REASON", g_SO_REASON, s_SO_REASON);
    pyrt::xrefs::reg_access("compose.why", "STEP_VERBS", g_STEP_VERBS, s_STEP_VERBS);
    pyrt::xrefs::reg_access("compose.why", "SUBSTANCE", g_SUBSTANCE, s_SUBSTANCE);
    pyrt::xrefs::reg_access("compose.why", "SUGGESTED_VERBS", g_SUGGESTED_VERBS, s_SUGGESTED_VERBS);
    pyrt::xrefs::reg_access("compose.why", "THEIR_DECISION", g_THEIR_DECISION, s_THEIR_DECISION);
    pyrt::xrefs::reg_access("compose.why", "THEIR_SUGGESTION", g_THEIR_SUGGESTION, s_THEIR_SUGGESTION);
    pyrt::xrefs::reg_access("compose.why", "THE_BOT_DESCRIBED_ITSELF", g_THE_BOT_DESCRIBED_ITSELF, s_THE_BOT_DESCRIBED_ITSELF);
    pyrt::xrefs::reg_access("compose.why", "THE_FIX", g_THE_FIX, s_THE_FIX);
    pyrt::xrefs::reg_access("compose.why", "THE_REASON_POINTS_AT_THE_ADVICE", g_THE_REASON_POINTS_AT_THE_ADVICE, s_THE_REASON_POINTS_AT_THE_ADVICE);
    pyrt::xrefs::reg_access("compose.why", "THINKS_BACK", g_THINKS_BACK, s_THINKS_BACK);
    pyrt::xrefs::reg_access("compose.why", "WHAT_IT_TAKES", g_WHAT_IT_TAKES, s_WHAT_IT_TAKES);
    pyrt::xrefs::reg_access("compose.why", "WHEN_CAUSE", g_WHEN_CAUSE, s_WHEN_CAUSE);
    pyrt::xrefs::reg_access("compose.why", "WHY_AGAIN", g_WHY_AGAIN, s_WHY_AGAIN);
    pyrt::xrefs::reg_access("compose.why", "WHY_ANSWERED", g_WHY_ANSWERED, s_WHY_ANSWERED);
    pyrt::xrefs::reg_access("compose.why", "WHY_BODY", g_WHY_BODY, s_WHY_BODY);
    pyrt::xrefs::reg_access("compose.why", "WHY_CARE", g_WHY_CARE, s_WHY_CARE);
    pyrt::xrefs::reg_access("compose.why", "WHY_END", g_WHY_END, s_WHY_END);
    pyrt::xrefs::reg_access("compose.why", "WHY_FRONT", g_WHY_FRONT, s_WHY_FRONT);
    pyrt::xrefs::reg_access("compose.why", "WHY_METHOD", g_WHY_METHOD, s_WHY_METHOD);
    pyrt::xrefs::reg_access("compose.why", "WHY_NAMED", g_WHY_NAMED, s_WHY_NAMED);
    pyrt::xrefs::reg_access("compose.why", "WHY_NAMED_BECAUSE", g_WHY_NAMED_BECAUSE, s_WHY_NAMED_BECAUSE);
    pyrt::xrefs::reg_access("compose.why", "WHY_NAMED_SELF", g_WHY_NAMED_SELF, s_WHY_NAMED_SELF);
    pyrt::xrefs::reg_access("compose.why", "WHY_NOT", g_WHY_NOT, s_WHY_NOT);
    pyrt::xrefs::reg_access("compose.why", "WHY_NOT_ANSWERED", g_WHY_NOT_ANSWERED, s_WHY_NOT_ANSWERED);
    pyrt::xrefs::reg_access("compose.why", "WHY_NOT_ITS_OWN", g_WHY_NOT_ITS_OWN, s_WHY_NOT_ITS_OWN);
    pyrt::xrefs::reg_access("compose.why", "WHY_THE_BOT", g_WHY_THE_BOT, s_WHY_THE_BOT);
    pyrt::xrefs::reg_access("compose.why", "WHY_YOU_SAID_IT", g_WHY_YOU_SAID_IT, s_WHY_YOU_SAID_IT);
    pyrt::xrefs::reg_access("compose.why", "WRONG", g_WRONG, s_WRONG);
    pyrt::xrefs::reg_access("compose.why", "WRONG_ANSWERED", g_WRONG_ANSWERED, s_WRONG_ANSWERED);
    pyrt::xrefs::reg_access("compose.why", "WRONG_NOT_A_FAULT", g_WRONG_NOT_A_FAULT, s_WRONG_NOT_A_FAULT);
    pyrt::xrefs::reg_access("compose.why", "_ANNOUNCES", g__ANNOUNCES, s__ANNOUNCES);
    pyrt::xrefs::reg_access("compose.why", "_A_CATEGORY", g__A_CATEGORY, s__A_CATEGORY);
    pyrt::xrefs::reg_access("compose.why", "_A_CAUSE_OPENS", g__A_CAUSE_OPENS, s__A_CAUSE_OPENS);
    pyrt::xrefs::reg_access("compose.why", "_A_CLAUSE_LEAST", g__A_CLAUSE_LEAST, s__A_CLAUSE_LEAST);
    pyrt::xrefs::reg_access("compose.why", "_A_DOING", g__A_DOING, s__A_DOING);
    pyrt::xrefs::reg_access("compose.why", "_A_DOING_WINDOW", g__A_DOING_WINDOW, s__A_DOING_WINDOW);
    pyrt::xrefs::reg_access("compose.why", "_A_FIGURE", g__A_FIGURE, s__A_FIGURE);
    pyrt::xrefs::reg_access("compose.why", "_A_LEAD_IN", g__A_LEAD_IN, s__A_LEAD_IN);
    pyrt::xrefs::reg_access("compose.why", "_A_LONG_SENTENCE", g__A_LONG_SENTENCE, s__A_LONG_SENTENCE);
    pyrt::xrefs::reg_access("compose.why", "_A_PARTICIPLE", g__A_PARTICIPLE, s__A_PARTICIPLE);
    pyrt::xrefs::reg_access("compose.why", "_A_REASON_LEAST", g__A_REASON_LEAST, s__A_REASON_LEAST);
    pyrt::xrefs::reg_access("compose.why", "_A_SAVE_LEAST", g__A_SAVE_LEAST, s__A_SAVE_LEAST);
    pyrt::xrefs::reg_access("compose.why", "_A_STRIDE", g__A_STRIDE, s__A_STRIDE);
    pyrt::xrefs::reg_access("compose.why", "_A_VERB", g__A_VERB, s__A_VERB);
    pyrt::xrefs::reg_access("compose.why", "_EXPLAINS", g__EXPLAINS, s__EXPLAINS);
    pyrt::xrefs::reg_access("compose.why", "_LEANS_BACK", g__LEANS_BACK, s__LEANS_BACK);
    pyrt::xrefs::reg_access("compose.why", "_LEANS_BACK_WINDOW", g__LEANS_BACK_WINDOW, s__LEANS_BACK_WINDOW);
    pyrt::xrefs::reg_access("compose.why", "_NEEDS_A_SUBJECT", g__NEEDS_A_SUBJECT, s__NEEDS_A_SUBJECT);
    pyrt::xrefs::reg_access("compose.why", "_OPENS_ON_A_FIGURE", g__OPENS_ON_A_FIGURE, s__OPENS_ON_A_FIGURE);
    pyrt::xrefs::reg_access("compose.why", "_POINTS_FORWARD", g__POINTS_FORWARD, s__POINTS_FORWARD);
    pyrt::xrefs::reg_access("compose.why", "_POINTS_FORWARD_ANY", g__POINTS_FORWARD_ANY, s__POINTS_FORWARD_ANY);
    pyrt::xrefs::reg_access("compose.why", "_POINTS_FORWARD_LEAST", g__POINTS_FORWARD_LEAST, s__POINTS_FORWARD_LEAST);
    pyrt::xrefs::reg_access("compose.why", "_SAYS_HOW", g__SAYS_HOW, s__SAYS_HOW);
    pyrt::xrefs::reg_access("compose.why", "_VERB_SHAPE", g__VERB_SHAPE, s__VERB_SHAPE);
}
