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


thread_local! { static G_ACK_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGAIN_FIRST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGAIN_TAIL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS_THE_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_FOR_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_CLAUSE_SPLIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CEILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONTENT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEFINITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEASURE_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MY_OWN_REPORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAINER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REPORT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REPORT_NOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUM_TO_WORK_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORKED_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRIES_NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHATTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHITCHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_MARKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLARED_ABOUT_THE_CHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLARED_AS_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLARED_CLAIMS_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLARED_DEFERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLARED_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECLARED_TOPICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING_WANTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ECHO_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INFLECTED_ENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPS_ITS_OWN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LABEL_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LETTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_ENOUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEMORY_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_ANSWER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_WHOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NEW_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SUBJECT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_ABOUT_THE_CHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_ASK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_READ: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFUSAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAFETY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_VOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ANSWER_BANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASK_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_LINE_BAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_MOVE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_ENDINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALLERGY_LIFTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_AMENDMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONDOLENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FIGURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REFUSAL_UNDONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_BEFORE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEP_WHEN_LOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_NAMELESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_ONE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ack: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__already: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__cooking: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__dream: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__harm: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__history: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__lineback: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__loss: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__naming: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__reading: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__recall: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__repeat: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ruledout: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__there_is_a_chat: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_reads_as_a_detail_of_the_news: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_reads_as_the_detail_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ACK_POOL() -> Py { ensure_init(); G_ACK_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK_POOL(v: Py) { ensure_init(); G_ACK_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_AGAIN_FIRST_WORDS() -> Py { ensure_init(); G_AGAIN_FIRST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN_FIRST_WORDS(v: Py) { ensure_init(); G_AGAIN_FIRST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGAIN_TAIL_WORDS() -> Py { ensure_init(); G_AGAIN_TAIL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN_TAIL_WORDS(v: Py) { ensure_init(); G_AGAIN_TAIL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS_THE_NEWS() -> Py { ensure_init(); G_ANSWERS_THE_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_THE_NEWS(v: Py) { ensure_init(); G_ANSWERS_THE_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_FOR_A_THING() -> Py { ensure_init(); G_ASKS_FOR_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_A_THING(v: Py) { ensure_init(); G_ASKS_FOR_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_CLAUSE_SPLIT() -> Py { ensure_init(); G_ASK_CLAUSE_SPLIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_CLAUSE_SPLIT(v: Py) { ensure_init(); G_ASK_CLAUSE_SPLIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CEILING() -> Py { ensure_init(); G_A_CEILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CEILING(v: Py) { ensure_init(); G_A_CEILING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONTENT_WORD() -> Py { ensure_init(); G_A_CONTENT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONTENT_WORD(v: Py) { ensure_init(); G_A_CONTENT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEFINITION() -> Py { ensure_init(); G_A_DEFINITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEFINITION(v: Py) { ensure_init(); G_A_DEFINITION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEASURE_ASKED() -> Py { ensure_init(); G_A_MEASURE_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEASURE_ASKED(v: Py) { ensure_init(); G_A_MEASURE_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MY_OWN_REPORT() -> Py { ensure_init(); G_A_MY_OWN_REPORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MY_OWN_REPORT(v: Py) { ensure_init(); G_A_MY_OWN_REPORT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAINER_ASK() -> Py { ensure_init(); G_A_PLAINER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAINER_ASK(v: Py) { ensure_init(); G_A_PLAINER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POINTS_BACK() -> Py { ensure_init(); G_A_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POINTS_BACK(v: Py) { ensure_init(); G_A_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REPORT_ASKS() -> Py { ensure_init(); G_A_REPORT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPORT_ASKS(v: Py) { ensure_init(); G_A_REPORT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REPORT_NOD() -> Py { ensure_init(); G_A_REPORT_NOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPORT_NOD(v: Py) { ensure_init(); G_A_REPORT_NOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT_PHRASE() -> Py { ensure_init(); G_A_SUBJECT_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT_PHRASE(v: Py) { ensure_init(); G_A_SUBJECT_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT_WORD() -> Py { ensure_init(); G_A_SUBJECT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT_WORD(v: Py) { ensure_init(); G_A_SUBJECT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUM_TO_WORK_OUT() -> Py { ensure_init(); G_A_SUM_TO_WORK_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUM_TO_WORK_OUT(v: Py) { ensure_init(); G_A_SUM_TO_WORK_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT_BACK() -> Py { ensure_init(); G_A_VERDICT_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT_BACK(v: Py) { ensure_init(); G_A_VERDICT_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORKED_OUT() -> Py { ensure_init(); G_A_WORKED_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORKED_OUT(v: Py) { ensure_init(); G_A_WORKED_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_THREAD() -> Py { ensure_init(); G_CARE_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_THREAD(v: Py) { ensure_init(); G_CARE_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRIES_NO_SUBJECT() -> Py { ensure_init(); G_CARRIES_NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRIES_NO_SUBJECT(v: Py) { ensure_init(); G_CARRIES_NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHATTER() -> Py { ensure_init(); G_CHATTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHATTER(v: Py) { ensure_init(); G_CHATTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CHITCHAT() -> Py { ensure_init(); G_CHITCHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHITCHAT(v: Py) { ensure_init(); G_CHITCHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE() -> Py { ensure_init(); G_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE(v: Py) { ensure_init(); G_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_MARKS() -> Py { ensure_init(); G_CUT_MARKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_MARKS(v: Py) { ensure_init(); G_CUT_MARKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLARED_ABOUT_THE_CHAT() -> Py { ensure_init(); G_DECLARED_ABOUT_THE_CHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLARED_ABOUT_THE_CHAT(v: Py) { ensure_init(); G_DECLARED_ABOUT_THE_CHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLARED_AS_IT_COMES() -> Py { ensure_init(); G_DECLARED_AS_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLARED_AS_IT_COMES(v: Py) { ensure_init(); G_DECLARED_AS_IT_COMES.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLARED_CLAIMS_INSTEAD() -> Py { ensure_init(); G_DECLARED_CLAIMS_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLARED_CLAIMS_INSTEAD(v: Py) { ensure_init(); G_DECLARED_CLAIMS_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLARED_DEFERS() -> Py { ensure_init(); G_DECLARED_DEFERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLARED_DEFERS(v: Py) { ensure_init(); G_DECLARED_DEFERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLARED_INSTEAD() -> Py { ensure_init(); G_DECLARED_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLARED_INSTEAD(v: Py) { ensure_init(); G_DECLARED_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DECLARED_TOPICS() -> Py { ensure_init(); G_DECLARED_TOPICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECLARED_TOPICS(v: Py) { ensure_init(); G_DECLARED_TOPICS.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING_WANTS() -> Py { ensure_init(); G_DOING_WANTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING_WANTS(v: Py) { ensure_init(); G_DOING_WANTS.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_WORDS() -> Py { ensure_init(); G_DOUBT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_WORDS(v: Py) { ensure_init(); G_DOUBT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ECHO_STOP() -> Py { ensure_init(); G_ECHO_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ECHO_STOP(v: Py) { ensure_init(); G_ECHO_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_VERB() -> Py { ensure_init(); G_FIRST_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_VERB(v: Py) { ensure_init(); G_FIRST_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERIC() -> Py { ensure_init(); G_GENERIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERIC(v: Py) { ensure_init(); G_GENERIC.with(|c| *c.borrow_mut() = v); }
pub fn g_INFLECTED_ENDS() -> Py { ensure_init(); G_INFLECTED_ENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INFLECTED_ENDS(v: Py) { ensure_init(); G_INFLECTED_ENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPS_ITS_OWN_ANSWER() -> Py { ensure_init(); G_KEEPS_ITS_OWN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPS_ITS_OWN_ANSWER(v: Py) { ensure_init(); G_KEEPS_ITS_OWN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_LABEL_KEYS() -> Py { ensure_init(); G_LABEL_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LABEL_KEYS(v: Py) { ensure_init(); G_LABEL_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_LETTERS() -> Py { ensure_init(); G_LETTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LETTERS(v: Py) { ensure_init(); G_LETTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_ENOUGH() -> Py { ensure_init(); G_LONG_ENOUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_ENOUGH(v: Py) { ensure_init(); G_LONG_ENOUGH.with(|c| *c.borrow_mut() = v); }
pub fn g_MEMORY_MODULES() -> Py { ensure_init(); G_MEMORY_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEMORY_MODULES(v: Py) { ensure_init(); G_MEMORY_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_ANSWER_WORDS() -> Py { ensure_init(); G_MOVE_ANSWER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_ANSWER_WORDS(v: Py) { ensure_init(); G_MOVE_ANSWER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_SHARE() -> Py { ensure_init(); G_MOVE_SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_SHARE(v: Py) { ensure_init(); G_MOVE_SHARE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_WHOLE() -> Py { ensure_init(); G_MOVE_WHOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_WHOLE(v: Py) { ensure_init(); G_MOVE_WHOLE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_WORDS() -> Py { ensure_init(); G_MOVE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_WORDS(v: Py) { ensure_init(); G_MOVE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NEW_WORD() -> Py { ensure_init(); G_NOT_A_NEW_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NEW_WORD(v: Py) { ensure_init(); G_NOT_A_NEW_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SUBJECT() -> Py { ensure_init(); G_NOT_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SUBJECT(v: Py) { ensure_init(); G_NOT_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SUBJECT_WORD() -> Py { ensure_init(); G_NOT_A_SUBJECT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SUBJECT_WORD(v: Py) { ensure_init(); G_NOT_A_SUBJECT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_ABOUT_THE_CHAT() -> Py { ensure_init(); G_OWN_ABOUT_THE_CHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_ABOUT_THE_CHAT(v: Py) { ensure_init(); G_OWN_ABOUT_THE_CHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_PAUSE() -> Py { ensure_init(); G_PAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAUSE(v: Py) { ensure_init(); G_PAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_ASK_FILLER() -> Py { ensure_init(); G_PLAIN_ASK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_ASK_FILLER(v: Py) { ensure_init(); G_PLAIN_ASK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL_MODULES() -> Py { ensure_init(); G_POOL_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL_MODULES(v: Py) { ensure_init(); G_POOL_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_READ() -> Py { ensure_init(); G_READ.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_READ(v: Py) { ensure_init(); G_READ.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL() -> Py { ensure_init(); G_RECALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL(v: Py) { ensure_init(); G_RECALL.with(|c| *c.borrow_mut() = v); }
pub fn g_REFUSAL() -> Py { ensure_init(); G_REFUSAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFUSAL(v: Py) { ensure_init(); G_REFUSAL.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_FRAME() -> Py { ensure_init(); G_REPEAT_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_FRAME(v: Py) { ensure_init(); G_REPEAT_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_STOP() -> Py { ensure_init(); G_REPORT_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_STOP(v: Py) { ensure_init(); G_REPORT_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_SAFETY() -> Py { ensure_init(); G_SAFETY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAFETY(v: Py) { ensure_init(); G_SAFETY.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_BACK() -> Py { ensure_init(); G_SAID_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_BACK(v: Py) { ensure_init(); G_SAID_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_VOICE() -> Py { ensure_init(); G_THEIR_VOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_VOICE(v: Py) { ensure_init(); G_THEIR_VOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ANSWER_BANDS() -> Py { ensure_init(); G_THE_ANSWER_BANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ANSWER_BANDS(v: Py) { ensure_init(); G_THE_ANSWER_BANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASK_OPENS() -> Py { ensure_init(); G_THE_ASK_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASK_OPENS(v: Py) { ensure_init(); G_THE_ASK_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_CAUSE() -> Py { ensure_init(); G_THE_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_CAUSE(v: Py) { ensure_init(); G_THE_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAD() -> Py { ensure_init(); G_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAD(v: Py) { ensure_init(); G_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_TOKEN() -> Py { ensure_init(); G_TOKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOKEN(v: Py) { ensure_init(); G_TOKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_LINE_BAND() -> Py { ensure_init(); G_WHOLE_LINE_BAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_LINE_BAND(v: Py) { ensure_init(); G_WHOLE_LINE_BAND.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_MOVE_WORDS() -> Py { ensure_init(); G_WHOLE_MOVE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_MOVE_WORDS(v: Py) { ensure_init(); G_WHOLE_MOVE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_ENDINGS() -> Py { ensure_init(); G_WORD_ENDINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_ENDINGS(v: Py) { ensure_init(); G_WORD_ENDINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_SOMEONE_ELSE() -> Py { ensure_init(); G__ABOUT_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_SOMEONE_ELSE(v: Py) { ensure_init(); G__ABOUT_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__ALLERGY_LIFTED() -> Py { ensure_init(); G__ALLERGY_LIFTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALLERGY_LIFTED(v: Py) { ensure_init(); G__ALLERGY_LIFTED.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_AMENDMENT() -> Py { ensure_init(); G__AN_AMENDMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_AMENDMENT(v: Py) { ensure_init(); G__AN_AMENDMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HOW_TO() -> Py { ensure_init(); G__A_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HOW_TO(v: Py) { ensure_init(); G__A_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON_NAMED() -> Py { ensure_init(); G__A_PERSON_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON_NAMED(v: Py) { ensure_init(); G__A_PERSON_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__CONDOLENCE() -> Py { ensure_init(); G__CONDOLENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONDOLENCE(v: Py) { ensure_init(); G__CONDOLENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__FIGURES() -> Py { ensure_init(); G__FIGURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FIGURES(v: Py) { ensure_init(); G__FIGURES.with(|c| *c.borrow_mut() = v); }
pub fn g__FRAME() -> Py { ensure_init(); G__FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRAME(v: Py) { ensure_init(); G__FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTHS() -> Py { ensure_init(); G__MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTHS(v: Py) { ensure_init(); G__MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORDS() -> Py { ensure_init(); G__NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORDS(v: Py) { ensure_init(); G__NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE() -> Py { ensure_init(); G__PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE(v: Py) { ensure_init(); G__PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__REFUSAL_UNDONE() -> Py { ensure_init(); G__REFUSAL_UNDONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REFUSAL_UNDONE(v: Py) { ensure_init(); G__REFUSAL_UNDONE.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_BEFORE_WORDS() -> Py { ensure_init(); G__SAID_BEFORE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_BEFORE_WORDS(v: Py) { ensure_init(); G__SAID_BEFORE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__STEP_WHEN_LOW() -> Py { ensure_init(); G__STEP_WHEN_LOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEP_WHEN_LOW(v: Py) { ensure_init(); G__STEP_WHEN_LOW.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_NAMELESS() -> Py { ensure_init(); G__THE_NAMELESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_NAMELESS(v: Py) { ensure_init(); G__THE_NAMELESS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_ONE_NAMED() -> Py { ensure_init(); G__THE_ONE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_ONE_NAMED(v: Py) { ensure_init(); G__THE_ONE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__ack() -> Py { ensure_init(); G__ack.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ack(v: Py) { ensure_init(); G__ack.with(|c| *c.borrow_mut() = v); }
pub fn g__already() -> Py { ensure_init(); G__already.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__already(v: Py) { ensure_init(); G__already.with(|c| *c.borrow_mut() = v); }
pub fn g__cooking() -> Py { ensure_init(); G__cooking.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__cooking(v: Py) { ensure_init(); G__cooking.with(|c| *c.borrow_mut() = v); }
pub fn g__dream() -> Py { ensure_init(); G__dream.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__dream(v: Py) { ensure_init(); G__dream.with(|c| *c.borrow_mut() = v); }
pub fn g__harm() -> Py { ensure_init(); G__harm.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__harm(v: Py) { ensure_init(); G__harm.with(|c| *c.borrow_mut() = v); }
pub fn g__history() -> Py { ensure_init(); G__history.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__history(v: Py) { ensure_init(); G__history.with(|c| *c.borrow_mut() = v); }
pub fn g__lineback() -> Py { ensure_init(); G__lineback.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__lineback(v: Py) { ensure_init(); G__lineback.with(|c| *c.borrow_mut() = v); }
pub fn g__loss() -> Py { ensure_init(); G__loss.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__loss(v: Py) { ensure_init(); G__loss.with(|c| *c.borrow_mut() = v); }
pub fn g__naming() -> Py { ensure_init(); G__naming.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__naming(v: Py) { ensure_init(); G__naming.with(|c| *c.borrow_mut() = v); }
pub fn g__reading() -> Py { ensure_init(); G__reading.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__reading(v: Py) { ensure_init(); G__reading.with(|c| *c.borrow_mut() = v); }
pub fn g__recall() -> Py { ensure_init(); G__recall.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__recall(v: Py) { ensure_init(); G__recall.with(|c| *c.borrow_mut() = v); }
pub fn g__repeat() -> Py { ensure_init(); G__repeat.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__repeat(v: Py) { ensure_init(); G__repeat.with(|c| *c.borrow_mut() = v); }
pub fn g__ruledout() -> Py { ensure_init(); G__ruledout.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ruledout(v: Py) { ensure_init(); G__ruledout.with(|c| *c.borrow_mut() = v); }
pub fn g__there_is_a_chat() -> Py { ensure_init(); G__there_is_a_chat.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__there_is_a_chat(v: Py) { ensure_init(); G__there_is_a_chat.with(|c| *c.borrow_mut() = v); }
pub fn g_reads_as_a_detail_of_the_news() -> Py { ensure_init(); G_reads_as_a_detail_of_the_news.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_reads_as_a_detail_of_the_news(v: Py) { ensure_init(); G_reads_as_a_detail_of_the_news.with(|c| *c.borrow_mut() = v); }
pub fn g_reads_as_the_detail_line() -> Py { ensure_init(); G_reads_as_the_detail_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_reads_as_the_detail_line(v: Py) { ensure_init(); G_reads_as_the_detail_line.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Chooses each turn's reply among the modules' candidates. The routing lane owns this file.\n\nThe seed rule stays the floor: candidates arrive highest PRIORITY first, and a fallback\n(PRIORITY below 0) answers only when nothing else does. Two general rules on top of it\n(README.md, \"Routing\"; STYLE.md):\n\n* the specialist beats the generalist: a module that declares `ANSWERS` (the topics it is\n  good at, e.g. `ANSWERS = (\"cooking\", \"food\")`) is preferred when this line was read as\n  being about one of them, and the candidate in front of it is not about them. A reply about\n  the topic asked beats a fluent one about something else. Two specialists go to the one that\n  matches the line better (more of its topics, a topic id before a single word), and only\n  then to PRIORITY. The understanding engine's topic ids (memory[\"understood\"][\"topics\"] and\n  [\"topic\"]) are the strong signal; the line's own words are the weaker one, so a module still\n  works while a topic reader is missing. A follow-up (\"and how long?\") has no topic of its\n  own, so the line the conversation is on (memory[\"topic\"]) is read with the same reader and\n  stands in for it.\n* a line about the conversation is the recall band's: a line that names no topic of its own\n  (\"what am i doing this week\" after \"im writing up my thesis\") was read as a topic only by\n  inheritance, and the memory store's answer to it (\"You're writing up your thesis.\") is the\n  answer to what was asked, while a topic entry that fits only by inheritance answers a question\n  nobody asked. So the specialist rule does not lift a module over a PRIORITY 7000 candidate on\n  a line whose own reading names no topic; the line's own phrase or word can still lift it (see\n  `_fit`).\n* never say the same thing twice in a row: a candidate whose reply only repeats the bot's\n  last turn is passed over while another candidate says something different. Nor a whole answer\n  said to this person earlier in the conversation: a reply that is word for word something the\n  bot already said, several turns back, is passed over while another real candidate has something\n  new. The same question asked twice in a row is the case for the repeat, and there every real\n  candidate is that answer again, so nothing is dropped.\n* an emergency stays an emergency: for a line that is still inside a safety thread, the\n  composer's careful continuation beats a light module that would answer it as ordinary talk.\n* a sudden loss is answered as a loss: a pregnancy that has ended or someone who took their own\n  life gets the composer's own line for it, even when a module below has a general \"sorry to\n  hear it\" for bad news (\"we lost the baby\" was answered \"give it a day, then look at what you\n  can control\", which is advice for a setback).\n* a bare \"where do i start\" gets the step out of the answer in hand: a line that names nothing and\n  asks for the next move is answered by the shortest candidate built from the answer's own words,\n  even when a topic entry answers the same line by reading the whole answer out again in its other\n  wording (see `_the_move_in_hand`).\n* a module can say that one of its proposals is to be answered as it comes, without raising its\n  PRIORITY: it declares `TAKES_AS_IT_COMES(text, memory)` and the arbiter asks it on the line\n  the proposal is for (see `_as_it_comes`). A band says \"everything this module proposes\"; a\n  hook says \"this line, because I read it whole\", which is what a module needs when only part\n  of what it proposes deserves the turn.\n* a reply does not offer the food the line refused: \"im trying to eat better but i hate veg\" was\n  answered by the balanced-meal card (\"Half the plate vegetables...\"), the one thing the person\n  had just ruled out. The kitchen already reads the refusal, so a candidate whose first sentence\n  offers that food is passed over while one that respects it is on the table. The safety band is\n  never passed over, and neither is a reply that names the food to leave it out (\"No onion then:\n  ...\", \"nut-free\"), which is still the answer to the refusal.\n\n* a rule-out keeps the turn when the answer beside it offers back what was ruled out: \"what\n  should i cook tonight\" / \"not curry, something else\" was answered \"Quick: chickpea curry in 25,\n  or a sweet potato curry\" - the thing refused, twice - while the rule-out's own line (\"That one\n  never mentions curry at all, so nothing changes. ...\") was in the list under the floor. A reply\n  that hands back what the person has just ruled out has not read the line, so it is not an answer\n  a claim gives way to, by PRIORITY or to a declaring specialist (see `_every_real_answer_offers`).\n\nA candidate at PRIORITY >= 9000 (the safety band) is never passed over this way: a real\nemergency is answered by the module that recognised it. Anything unexpected falls back to the\nseed rule, so a turn is never lost and the arbiter never crashes a reply.\n");
    s_reads_as_a_detail_of_the_news(py_func("reads_as_a_detail_of_the_news", |args: &[Py]| pyrt::xrefs::xcall("compose.goodnews", "reads_as_a_detail_of_the_news", args)));
    s_reads_as_the_detail_line(py_func("reads_as_the_detail_line", |args: &[Py]| pyrt::xrefs::xcall("compose.goodnews", "reads_as_the_detail_line", args)));
    let __try1 = exc::catch(|| {
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
    s__ack(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    let __try6 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    s__recall(Py::None);
    s__repeat(Py::None);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    let __try11 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret12: Py = Py::Bool(false);
    let mut __val13: Py = Py::None;
    let mut __unhandled14: Option<exc::PyExc> = None;
    match __try11 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret12 = __p[0].clone(); __val13 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled15 = false;
            if !__handled15 && exc::matches_any(&__e, &["Exception"]) {
                __handled15 = true;
    s__loss(Py::None);
            }
            if !__handled15 { __unhandled14 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled14 { exc::raise(__e); }
    let __try16 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret17: Py = Py::Bool(false);
    let mut __val18: Py = Py::None;
    let mut __unhandled19: Option<exc::PyExc> = None;
    match __try16 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret17 = __p[0].clone(); __val18 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled20 = false;
            if !__handled20 && exc::matches_any(&__e, &["Exception"]) {
                __handled20 = true;
    s__dream(Py::None);
            }
            if !__handled20 { __unhandled19 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled19 { exc::raise(__e); }
    let __try21 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret22: Py = Py::Bool(false);
    let mut __val23: Py = Py::None;
    let mut __unhandled24: Option<exc::PyExc> = None;
    match __try21 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret22 = __p[0].clone(); __val23 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled25 = false;
            if !__handled25 && exc::matches_any(&__e, &["Exception"]) {
                __handled25 = true;
    s__already(Py::None);
            }
            if !__handled25 { __unhandled24 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled24 { exc::raise(__e); }
    let __try26 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret27: Py = Py::Bool(false);
    let mut __val28: Py = Py::None;
    let mut __unhandled29: Option<exc::PyExc> = None;
    match __try26 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret27 = __p[0].clone(); __val28 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled30 = false;
            if !__handled30 && exc::matches_any(&__e, &["Exception"]) {
                __handled30 = true;
    s__harm(Py::None);
            }
            if !__handled30 { __unhandled29 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled29 { exc::raise(__e); }
    let __try31 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret32: Py = Py::Bool(false);
    let mut __val33: Py = Py::None;
    let mut __unhandled34: Option<exc::PyExc> = None;
    match __try31 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret32 = __p[0].clone(); __val33 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled35 = false;
            if !__handled35 && exc::matches_any(&__e, &["Exception"]) {
                __handled35 = true;
    s__there_is_a_chat(py_func("lambda", move |args: &[Py]| {  let _memory: Py = __kwp(args, 0, "_memory").unwrap_or(Py::None); Py::Bool(false) }));
            }
            if !__handled35 { __unhandled34 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled34 { exc::raise(__e); }
    let __try36 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret37: Py = Py::Bool(false);
    let mut __val38: Py = Py::None;
    let mut __unhandled39: Option<exc::PyExc> = None;
    match __try36 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret37 = __p[0].clone(); __val38 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled40 = false;
            if !__handled40 && exc::matches_any(&__e, &["Exception"]) {
                __handled40 = true;
    s__repeat(Py::None);
            }
            if !__handled40 { __unhandled39 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled39 { exc::raise(__e); }
    let __try41 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret42: Py = Py::Bool(false);
    let mut __val43: Py = Py::None;
    let mut __unhandled44: Option<exc::PyExc> = None;
    match __try41 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret42 = __p[0].clone(); __val43 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled45 = false;
            if !__handled45 && exc::matches_any(&__e, &["Exception"]) {
                __handled45 = true;
    s__recall(Py::None);
            }
            if !__handled45 { __unhandled44 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled44 { exc::raise(__e); }
    let __try46 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret47: Py = Py::Bool(false);
    let mut __val48: Py = Py::None;
    let mut __unhandled49: Option<exc::PyExc> = None;
    match __try46 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret47 = __p[0].clone(); __val48 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled50 = false;
            if !__handled50 && exc::matches_any(&__e, &["Exception"]) {
                __handled50 = true;
    s__lineback(Py::None);
            }
            if !__handled50 { __unhandled49 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled49 { exc::raise(__e); }
    let __try51 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret52: Py = Py::Bool(false);
    let mut __val53: Py = Py::None;
    let mut __unhandled54: Option<exc::PyExc> = None;
    match __try51 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret52 = __p[0].clone(); __val53 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled55 = false;
            if !__handled55 && exc::matches_any(&__e, &["Exception"]) {
                __handled55 = true;
    s__naming(Py::None);
            }
            if !__handled55 { __unhandled54 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled54 { exc::raise(__e); }
    let __try56 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret57: Py = Py::Bool(false);
    let mut __val58: Py = Py::None;
    let mut __unhandled59: Option<exc::PyExc> = None;
    match __try56 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret57 = __p[0].clone(); __val58 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled60 = false;
            if !__handled60 && exc::matches_any(&__e, &["Exception"]) {
                __handled60 = true;
    let __unpk61 = py_unpack(&py_tuple(vec![py_str("thread"), py_str("care-thread"), py_str("pause")]), 3, None);
    s_THREAD(__unpk61[0].clone());
    s_CARE_THREAD(__unpk61[1].clone());
    s_PAUSE(__unpk61[2].clone());
            }
            if !__handled60 { __unhandled59 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled59 { exc::raise(__e); }
    let __try62 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret63: Py = Py::Bool(false);
    let mut __val64: Py = Py::None;
    let mut __unhandled65: Option<exc::PyExc> = None;
    match __try62 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret63 = __p[0].clone(); __val64 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled66 = false;
            if !__handled66 && exc::matches_any(&__e, &["Exception"]) {
                __handled66 = true;
    s__STEP_WHEN_LOW(py_tuple(vec![]));
            }
            if !__handled66 { __unhandled65 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled65 { exc::raise(__e); }
    let __try67 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret68: Py = Py::Bool(false);
    let mut __val69: Py = Py::None;
    let mut __unhandled70: Option<exc::PyExc> = None;
    match __try67 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret68 = __p[0].clone(); __val69 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled71 = false;
            if !__handled71 && exc::matches_any(&__e, &["Exception"]) {
                __handled71 = true;
    s__cooking(Py::None);
            }
            if !__handled71 { __unhandled70 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled70 { exc::raise(__e); }
    let __try72 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret73: Py = Py::Bool(false);
    let mut __val74: Py = Py::None;
    let mut __unhandled75: Option<exc::PyExc> = None;
    match __try72 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret73 = __p[0].clone(); __val74 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled76 = false;
            if !__handled76 && exc::matches_any(&__e, &["Exception"]) {
                __handled76 = true;
    s__ruledout(Py::None);
            }
            if !__handled76 { __unhandled75 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled75 { exc::raise(__e); }
    let __try77 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret78: Py = Py::Bool(false);
    let mut __val79: Py = Py::None;
    let mut __unhandled80: Option<exc::PyExc> = None;
    match __try77 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret78 = __p[0].clone(); __val79 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled81 = false;
            if !__handled81 && exc::matches_any(&__e, &["Exception"]) {
                __handled81 = true;
    s__history(Py::None);
            }
            if !__handled81 { __unhandled80 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled80 { exc::raise(__e); }
    let __try82 = exc::catch(|| {
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret83: Py = Py::Bool(false);
    let mut __val84: Py = Py::None;
    let mut __unhandled85: Option<exc::PyExc> = None;
    match __try82 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret83 = __p[0].clone(); __val84 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled86 = false;
            if !__handled86 && exc::matches_any(&__e, &["Exception"]) {
                __handled86 = true;
    s__reading(Py::None);
            }
            if !__handled86 { __unhandled85 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled85 { exc::raise(__e); }
    s_DECLARED_TOPICS(py_str("ANSWERS"));
    s_CHOICE(py_str("choice"));
    s_DECLARED_INSTEAD(py_str("ANSWERS_INSTEAD"));
    s_DECLARED_DEFERS(py_str("DEFERS_TO"));
    s_DECLARED_AS_IT_COMES(py_str("TAKES_AS_IT_COMES"));
    s_DECLARED_CLAIMS_INSTEAD(py_str("CLAIMS_INSTEAD"));
    s_DECLARED_ABOUT_THE_CHAT(py_str("ASKS_ABOUT_THE_CHAT"));
    s_OWN_ABOUT_THE_CHAT(py_str("_about_this_chat"));
    s_SAFETY(Py::Int(9000));
    s_CHATTER(Py::Int(1000));
    s_RECALL(Py::Int(7000));
    s_MOVE_WORDS(Py::Int(26));
    s_MOVE_SHARE(Py::Float(0.5));
    s_MOVE_WHOLE(Py::Float(0.75));
    s_MOVE_ANSWER_WORDS(Py::Int(30));
    s_WHOLE_LINE_BAND(Py::Int(2000));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9]+(?:'[a-z]+)?")]).unwrap_or_else(|e| exc::raise(e)));
    s_LABEL_KEYS(py_tuple(vec![py_str("id"), py_str("label"), py_str("topic"), py_str("name"), py_str("text")]));
    s_READ(py_dict(vec![]));
    s__SAID_BEFORE_WORDS(Py::Int(8));
    s_MEMORY_MODULES(call_builtin("frozenset", &[py_tuple(vec![py_str("memory_store"), py_str("chat"), py_str("greeting"), py_str("said_back")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data87() -> Py { py_tuple(vec![py_str("no"), py_str("not"), py_str("without"), py_str("free"), py_str("avoid"), py_str("leave"), py_str("leaves"), py_str("leaving"), py_str("left"), py_str("out"), py_str("instead"), py_str("swap"), py_str("swaps"), py_str("swapped"), py_str("rather"), py_str("cant"), py_str("dont"), py_str("doesnt"), py_str("didnt"), py_str("refuse"), py_str("refused"), py_str("refuses"), py_str("allergic"), py_str("allergy"), py_str("intolerant"), py_str("allergen"), py_str("isnt"), py_str("arent"), py_str("wasnt"), py_str("wont"), py_str("cannot"), py_str("never"), py_str("none"), py_str("nothing"), py_str("off"), py_str("replace")]) }
    s__REFUSAL_UNDONE(call_builtin("frozenset", &[__data87()]).unwrap_or_else(|e| exc::raise(e)));
    s__ALLERGY_LIFTED(stdlib_call("re", "compile", &[py_str("\\b(?:not|isnt|arent|wasnt|werent|never)\\s+(?:actually\\s+)?(?:allergic|intolerant)\\b|\\bno\\s+(?:allerg(?:y|ies)|intolerance)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CEILING(stdlib_call("re", "compile", &[py_str("\\b(?:too much|too many|max(?:imum)?|limit|safe|how much can i (?:take|have|drink|eat)|how many can i (?:take|have|drink|eat))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GENERIC(call_builtin("frozenset", &[mcall(&py_str("\n    about after again along another anything around before being better between could doing\n    during enough every first found going great happy least leave might money month months never\n    other people place point right should since something still take taken their there these\n    thing things think those three through times today together under until usual wanted\n    where which while without would wrong year years\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b\\d[\\d,]*(?:\\.\\d+)?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WORKED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:none|already|past|more|less|fewer|short|left|over|under|enough|comes to|works out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUM_TO_WORK_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:left|remaining|remain|more|less|fewer|extra|another|needed|need|short|over|under|enough|towards|altogether|in all|total)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_AMENDMENT(stdlib_call("re", "compile", &[py_str("^\\s*(?:actually|no|nope|sorry|oh|hang on|wait|correction|scratch that|i mean|make that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MONTHS(stdlib_call("re", "compile", &[py_str("\\b(january|february|march|april|may|june|july|august|september|october|november|december)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DOING_WANTS(call_builtin("frozenset", &[py_tuple(vec![py_str("steps"), py_str("options"), py_str("pick"), py_str("advice"), py_str("howto")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_BACK(stdlib_call("re", "compile", &[py_str("^(?:you said|you told me|you mentioned|as you said|still )\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data88() -> Py { py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("hundred"), Py::Int(100)), (py_str("thousand"), Py::Int(1000))]) }
    s__NUMBER_WORDS(__data88());
    s__FIGURES(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b\\d[\\d,]*(?:\\.\\d+)?\\b|\\b(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__NUMBER_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MY_OWN_REPORT(stdlib_call("re", "compile", &[py_str("\\b(?:i|ive|i've|we|weve|we've)\\s+(?:have|has|had|got|gotten)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORT_ASKS(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:what|whats|why|how|when|where|which|who|is|are|was|do|does|did|can|could|should|would|will|tell|explain|mean|means|contagious|infectious|symptom|symptoms|sign|signs|treat|treated|treatment|long|last|lasts|serious|dangerous|fatal|cause|causes|recover|spread|catch|test|tests|vaccine|diet|food|foods|cure|help|worried|worry|advice|tips|avoid|prevent|dose|better|worse|normal|common|painful)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEFINITION(stdlib_call("re", "compile", &[py_str("^\\s*(?!(?:it|there|that|this|these|those|he|she|they|you|we|i|here|what)\\b)(?:a\\s+|an\\s+|the\\s+|type\\s+\\d+\\s+)?[\\w'-]+(?:\\s+[\\w'-]+){0,3}\\s+(?:is|are)\\s+\\S"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORT_NOD(stdlib_call("re", "compile", &[py_str("^\\s*(?:noted|got it|okay|ok\\b|right|sure|nice one|i'm listening|im listening|that sounds|thanks|cheers)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONTENT_WORD(stdlib_call("re", "compile", &[py_str("[a-z][a-z'-]{2,}")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_STOP(call_builtin("frozenset", &[mcall(&py_str("i im i'm ive i've we we've weve me my mine myself have has had got gotten\n    the a an and or but so that this it its it's of to in on at for with from about really very\n    quite just now today again too also still some any been being am is are was were do does did\n    not no yes feel feeling felt think thinks thought lot bit sort kind thing things"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LONG_ENOUGH(Py::Int(4));
    s_LETTERS(stdlib_call("re", "compile", &[py_str("[a-z]+")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_VERB(call_builtin("frozenset", &[mcall(&py_str("is are was were will can could should may might must does do did has have had"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARRIES_NO_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("a an the and or but if of to in on at for with from by as it\nits this that these those i me my we us our you your he she they them their not no yes so then than\njust also very really quite some any all about there here out up down over under more most much\nless"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_INFLECTED_ENDS(call_builtin("frozenset", &[py_tuple(vec![py_str("s"), py_str("es"), py_str("'s"), py_str("s'"), py_str("ache")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBT_WORDS(call_builtin("frozenset", &[mcall(&py_str("bad normal okay fine good safe right wrong better worse serious worry worried thing things that this it is be to a an the for of"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:did|do|does|were|was|have|had)\\s+(?:you|u)\\s+(?:say|said|tell|told|mention)\\b|\\b(?:you|u)\\s+(?:said|say|told|mentioned)\\b|\\bagain\\b|\\b(?:explain|talk me through|walk me through)\\s+(?:it|that|this|those|them)\\b|\\bgo\\s+(?:over|through)\\s+(?:it|that|this)\\b|\\brun\\s+(?:it|that|this)\\s+by\\s+me\\b|\\bthe\\s+(?:last|final)\\s+(?:bit|part)\\b|\\b(?:once|one)\\s+more\\b|\\bone\\s+more\\s+time\\b|\\b(?:i|you)\\s+(?:lost|lose)\\s+(?:you|me)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPEAT_FRAME(call_builtin("frozenset", &[mcall(&py_str("\n    a again already an and earlier before bit bits but by did do does exactly explain explained\n    final for go going had has have how is it just last long lose lost many me mention mentioned\n    more much now of okay once one over part parts please ran run said say sayd so sorry tell that\n    the them then there this through to told u us walk talk walked talked was were what when which you\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__FRAME(Py::None);
    s_THE_ANSWER_BANDS(call_builtin("frozenset", &[py_tuple(vec![py_str("answers"), py_str("memory_store")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ACK_POOL(Py::None);
    s_POOL_MODULES(call_builtin("frozenset", &[py_tuple(vec![py_str("chat"), py_str("greeting")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_ASK_OPENS(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |also |then |please )*(?:what|how|which|where|when|who|why|any|ideas|thoughts|suggestions|advice|tips|recommend\\w*|should|could|shall|can|do|is|are|help)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data89() -> Py { py_tuple(vec![py_str("the"), py_str("and"), py_str("for"), py_str("you"), py_str("your"), py_str("with"), py_str("that"), py_str("this"), py_str("have"), py_str("has"), py_str("was"), py_str("were"), py_str("are"), py_str("but"), py_str("not"), py_str("his"), py_str("her"), py_str("she"), py_str("him"), py_str("they"), py_str("them"), py_str("its"), py_str("out"), py_str("about"), py_str("some"), py_str("from"), py_str("been"), py_str("what"), py_str("how"), py_str("would"), py_str("will"), py_str("just"), py_str("one"), py_str("now"), py_str("day"), py_str("like")]) }
    s_ECHO_STOP(call_builtin("frozenset", &[__data89()]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_VOICE(py_dict(vec![(py_str("my"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("i"), py_str("you")), (py_str("im"), py_str("youre")), (py_str("me"), py_str("you")), (py_str("myself"), py_str("yourself")), (py_str("we"), py_str("you")), (py_str("our"), py_str("your")), (py_str("ours"), py_str("yours"))]));
    s_A_SUBJECT_WORD(stdlib_call("re", "compile", &[py_str("[a-z']{3,}")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUBJECT_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|your|his|her|their|the|this|that|these|those|a|an)\\s+((?:[a-z']+\\s+){0,2}[a-z']+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD_ENDINGS(py_tuple(vec![py_str("s"), py_str("es"), py_str("ed"), py_str("d"), py_str("ing"), py_str("er"), py_str("ers"), py_str("est"), py_str("ly")]));
    s_A_MEASURE_ASKED(stdlib_call("re", "compile", &[py_str("\\bwhen\\b|\\bhow\\s+(?:long|soon|much|many|hot|warm|old|far|big|often)\\b|\\bwhat\\s+(?:time|temperature|day|date)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("small talk")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHOLE_MOVE_WORDS(Py::Int(60));
    s_CUT_MARKS(py_str(",;:"));
    s_TOKEN(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_CAUSE(stdlib_call("re", "compile", &[py_str("\\bwhy\\b[^,.;?!]{0,60}\\b(?:dying|wilting|drooping)\\b|\\bwhat (?:am i|are we) doing wrong\\b|\\bwhat(?:'s| is| are) (?:killing|causing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PEOPLE(call_builtin("frozenset", &[mcall(&py_str("mum mom mother mum's dad father gran granny nan nana grandad grandpa\nson daughter child baby wife husband partner boyfriend girlfriend brother sister friend mate\nfiance fiancee aunt uncle cousin niece nephew he she him her his hers they them their people\nperson someone somebody anyone everyone"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__CONDOLENCE(stdlib_call("re", "compile", &[py_str("^(?:oh\\s+)?(?:(?:i'?m|i am|so|that'?s|that is)\\s+)?(?:so\\s+|really\\s+|very\\s+|truly\\s+)?sorry\\b|^that'?s (?:a real|such a|awful|terrible|heartbreaking)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_NAMELESS(call_builtin("frozenset", &[mcall(&py_str("about after again also been before being both could does doing down\neven every from have here into just like made make many more most much must need never only other\nought over same should since some such than that their them then there these they thing things\nthis those through under until very were what when where which while will with would your yours\nreally actually still might shall upon got gets going gone want wants wanted know knows saying\nsaid tells told feel feels felt look looks looked take takes took today yesterday tomorrow week\nmonth year time days just"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_ITS_OWN_ANSWER(py_tuple(vec![py_str("memory_store"), py_str("chat")]));
    s_ANSWERS_THE_NEWS(py_tuple(vec![py_str("answers")]));
    s_A_VERDICT_BACK(stdlib_call("re", "compile", &[py_str("^\\W*(?:it\\s+is|it'?s|it\\s+does|it\\s+doesn'?t|that\\s+is|that'?s|that\\s+would|that'?d|that\\s+kind|that\\s+sounds|you'?re\\s+right|you\\s+are\\s+right|yeah|yes|true|fair\\s+enough|of\\s+course|same\\s+here|i\\s+know|i\\s+don'?t\\s+blame|anyone\\s+would)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data90() -> Py { py_tuple(vec![py_str("the"), py_str("and"), py_str("but"), py_str("for"), py_str("with"), py_str("you"), py_str("your"), py_str("yours"), py_str("they"), py_str("them"), py_str("their"), py_str("his"), py_str("her"), py_str("hers"), py_str("she"), py_str("him"), py_str("our"), py_str("ours"), py_str("us"), py_str("its"), py_str("it"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("was"), py_str("were"), py_str("been"), py_str("are"), py_str("am"), py_str("is"), py_str("be"), py_str("does"), py_str("did"), py_str("done"), py_str("have"), py_str("has"), py_str("had"), py_str("can"), py_str("cant"), py_str("cannot"), py_str("will"), py_str("would"), py_str("should"), py_str("could"), py_str("may"), py_str("might"), py_str("must"), py_str("not"), py_str("nor"), py_str("off"), py_str("out"), py_str("down"), py_str("too"), py_str("very"), py_str("really"), py_str("much"), py_str("many"), py_str("some"), py_str("any"), py_str("all"), py_str("more"), py_str("most"), py_str("just"), py_str("only"), py_str("then"), py_str("there"), py_str("here"), py_str("now"), py_str("how"), py_str("what"), py_str("when"), py_str("where"), py_str("why"), py_str("who"), py_str("which"), py_str("about"), py_str("than"), py_str("into"), py_str("onto"), py_str("over"), py_str("under"), py_str("again"), py_str("still"), py_str("even"), py_str("ever"), py_str("never"), py_str("get"), py_str("got"), py_str("gets"), py_str("one"), py_str("two"), py_str("put"), py_str("say"), py_str("says"), py_str("said"), py_str("think"), py_str("know"), py_str("like"), py_str("want"), py_str("need"), py_str("make"), py_str("made"), py_str("take"), py_str("took"), py_str("going"), py_str("went"), py_str("gone"), py_str("come"), py_str("came"), py_str("keep"), py_str("keeps"), py_str("kept"), py_str("let"), py_str("lets")]) }
    s_NOT_A_SUBJECT_WORD(call_builtin("frozenset", &[__data90()]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_CLAUSE_SPLIT(stdlib_call("re", "compile", &[py_str(",|\\band\\b|\\bbut\\b|\\bsince\\b|\\bbecause\\b|\\bso\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_FOR_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|where|who|whose|when|why|how|should|shall|can|could|do|does|did|is|are|was|were|will|would)\\b|\\b(?:need|needs|needed|want|wants|wanted|looking for|have to|has to|had to|got to|gotta|after a|after some|in need of)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HOW_TO(stdlib_call("re", "compile", &[py_str("^(?:how\\s+(?:do|can|should|could|would)\\s+(?:i|we|you)|how\\s+to\\b|what(?:'?s|\\s+is)\\s+the\\s+best\\s+way\\s+to|whats\\s+the\\s+best\\s+way\\s+to|any\\s+tips\\s+(?:for|on)|what\\s+should\\s+i\\s+do\\s+(?:about|with))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_ONE_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their)\\s+(?:teenager|teen|son|daughter|child|kid|boy|girl|mum|mom|mother|dad|father|partner|husband|wife|boyfriend|girlfriend|flatmate|housemate|roommate|friend|boss|manager|colleague|brother|sister|gran|granny|grandma|grandad|grandpa|baby|toddler|nephew|niece|aunt|uncle|cousin)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:(?:my|our|his|her|their)\\s+(?:teenager|teen|son|daughter|child|kid|boy|girl|mum|mom|mother|dad|father|partner|husband|wife|boyfriend|girlfriend|flatmate|housemate|roommate|friend|boss|manager|colleague|brother|sister|gran|granny|grandma|grandad|grandpa|baby|toddler|nephew|niece|aunt|uncle|cousin)|(?:a|an|the|this|that|one)\\s+(?:friend|mate|neighbour|neighbor|colleague|coworker|co-worker|classmate|loved\\s+one|relative|family\\s+member))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ABOUT_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:they|them|their|theirs|themselves|he|she|him|his|hers)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSAL(stdlib_call("re", "compile", &[py_str("\\b(?:won'?t|wont|wouldn'?t|refus\\w*|isn'?t eating|not eating|doesn'?t want|does not want|gives? up|gave up|stops? eating)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHITCHAT(call_builtin("frozenset", &[py_tuple(vec![py_str("small talk")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGAIN_FIRST_WORDS(Py::Int(16));
    s_AGAIN_TAIL_WORDS(Py::Int(12));
    s_A_POINTS_BACK(stdlib_call("re", "compile", &[py_str("\\bis in what I said above\\b|\\bis in the answer above\\b|\\bis covered by the answer above\\b|\\bwhat I said above is the whole of it\\b|\\bthan what I said above\\b|\\bis the whole of what I have on it\\b|\\bnothing separate on\\b|\\bnothing more on\\b|\\bno separate answer for\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NEW_WORD(call_builtin("frozenset", &[mcall(&py_str("\nthanks thank sorry please pardon cheers again alright okay ok right really maybe perhaps actually\nstill even ever always whole little another other thing things lot stuff first next then step steps\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLAINER_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:explain(?:\\s+(?:that|it|this))?|in plain (?:english|words)|plainer|plainly|simpler|more simply|put (?:it|that) (?:another way|in other words)|in other words|what (?:do you|does that) mean|i (?:don'?t|do not|didn'?t) (?:understand|get it|follow))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN_ASK_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na again and another ask can could did do does doing dont don't english explain explained explaining\nfollow following for get got how i in it just mean means meant me more not now of or plain plainer\nplainly please put right say saying simpler simply so sorry that the then there this to understand\nunderstandit want was way what word words would you your\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn choose(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut index: Py = Py::None;
    let _ = py_str("Return the index of the chosen candidate.\n\n    candidates: [{\"module\": name, \"priority\": PRIORITY, \"reply\": text}], highest PRIORITY\n    first. memory[\"turns\"] holds the earlier turns of the conversation.\n    ");
    let __try1 = exc::catch(|| {
    index = _choose(&[text.clone(), memory.clone(), candidates.clone()]);
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
    index = _seed(&[candidates.clone()]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    index = _a_past_that_is_not_there(&[index.clone(), candidates.clone(), text.clone(), memory.clone()]);
    return _an_answer_it_already_gave(&[index.clone(), candidates.clone(), text.clone(), memory.clone()]);
    Py::None
}
pub fn _a_past_that_is_not_there(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut index: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut candidate: Py = Py::None;
    let mut other: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut speaks_of_a_past: Py = Py::None;
    let _ = py_str("The chosen candidate, unless it answers a chat that has not started.\n\n    A reply that speaks about the conversation - \"welcome back, where were we\", \"you mentioned\",\n    \"same as before\", \"good to hear, what was the best bit?\", \"nice one, how did the rest of it\n    go?\" - is about turns behind this one, and on the opening line there are none. Every pool in\n    the bot is written for a chat in progress, so the same fault turns up in a dozen places; the\n    fault is not in the pools but in giving a reply about a life the person has not described,\n    which is one question about the conversation and not one question per module. It is asked\n    here, once, after every rule has had its say, and the question itself lives in one place:\n    `bot/memory/history.py::an_earlier_turn`, read by\n    `bot/memory/history.py::a_past_that_is_not_there`, which the composer, the store's warmth and\n    this guard all call.\n\n    The reading takes the person's line as well as the reply. A frame like \"nice one\" or \"good to\n    hear\" is right on a first line that brings something of its own (\"weve booked a week in\n    cornwall\" is well met by \"Nice one. Is it soon?\"), and only wrong when the line brings\n    nothing - \"carry on\", \"as i was saying\", \"sorry, i got cut off\". So the frame counts when the\n    line points back too, which is the same reading in `bot/memory/history.py`.\n\n    The replacement is the best of what the modules offered after the leak is taken out: a take\n    from a real module, never the greeting band underneath them (a greeting is not an answer to\n    anything), and never out of the safety band, which does not talk about earlier turns and whose\n    candidate is the turn whatever else is on the table. Where every candidate answers something\n    unsaid the turn is left as it came: the composer's own answer to that is its first-line pool\n    (`bot/compose/fallback_data.py::NOTHING_SAID_YET`), offered on a first line for the same\n    reason, and no reply is invented here. The guard cannot raise - a guard that raises is worse\n    than no guard, because the exception would take the turn with it - so everything in it is\n    inside one `try`.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&g__history(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&candidates.clone())) } }) {
    return index.clone();
    }
    let __try1 = exc::catch(|| {
pub fn __nested_speaks_of_a_past_5(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut candidate: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut candidates: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut index: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut other: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut speaks_of_a_past: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "reply").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return pyrt::xrefs::xcall("memory.history", "a_past_that_is_not_there", &[reply.clone(), memory.clone(), text.clone()]);
    Py::None
}
    speaks_of_a_past = {{ let candidate = candidate.clone(); let candidates = candidates.clone(); let index = index.clone(); let memory = memory.clone(); let other = other.clone(); let reply = reply.clone(); let speaks_of_a_past = speaks_of_a_past.clone(); let text = text.clone(); py_func("speaks_of_a_past", move |args: &[Py]| { let mut full = vec![candidate.clone(), candidates.clone(), index.clone(), memory.clone(), other.clone(), reply.clone(), speaks_of_a_past.clone(), text.clone()]; full.extend_from_slice(args); __nested_speaks_of_a_past_5(&full) }) }};
    reply = if py_truthy(&{ let __cmp71: Py = index.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp71)) { op_lt(&__cmp71, &call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) } }) { mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&py_call(&speaks_of_a_past, &[reply.clone()])))) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
    if py_truthy(&op_ge(&_priority(&[geti(&candidates.clone(), &index.clone())]), &g_SAFETY())) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
    let __iter8 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter8 {
    let __unpk9 = py_unpack(&__item.clone(), 2, None);
    other = __unpk9[0].clone();
    candidate = __unpk9[1].clone();
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&py_call(&speaks_of_a_past, &[mcall(&candidate.clone(), "get", &[py_str("reply")])])))) {
    return py_tuple(vec![Py::Bool(true), other.clone()]);
    }
    }
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
    return index.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return index.clone();
    Py::None
}
pub fn _an_answer_it_already_gave(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut index: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut candidate: Py = Py::None;
    let mut heard: Py = Py::None;
    let mut last: Py = Py::None;
    let mut other: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The chosen candidate, unless it is a whole answer this chat has already been given.\n\n    The never-repeat rule lives inside `_choose`, on the path where every real module claims the\n    line. A follow-up that takes one of the arbiter's other paths - the shape a line asks for, a\n    verdict on the news, a claim, the knowledge base reading the line as the same question - came\n    back with a paragraph the person had read three to nine turns earlier, and it was the same\n    reply word for word:\n\n        my mum fell over / ... / what do i say to her  -> the falls paragraph, five turns back\n        my phone screen is cracked / ... / what should it cost to fix  -> the case paragraph\n        ok back to edinburgh, whats the food like    -> the Edinburgh card, six turns back\n\n    So the reading is asked once, after every rule has had its say (the same place and the same\n    promise as `_a_past_that_is_not_there`): a reply whose words are the words of an answer this\n    chat has already had, to a different question, is passed over when another real candidate is\n    not one. It is passed over, never dropped - where every candidate on the table repeats, the\n    repeat stands, because the answer twice is still better than a shrug (STYLE.md), and the same\n    question asked again keeps its answer (`bot/memory/again.py` frames it, and `_said_before`\n    leaves that turn out of the set). The safety band is never moved: an emergency answer is the\n    turn whatever else is on the table.\n\n    What takes the turn instead has to say something of its own. A nod, a receipt (\"Got it - she's\n    ok, just bruised\"), a hedge, a pointer back (\"what I said above\") or another answer the chat\n    has already had is not one, and neither is a line shorter than the length a whole answer\n    starts at: those are the lines that would fit any message (STYLE.md), and the paragraph again\n    is better than one of them. Where a band the arbiter normally stands aside for - the greeting\n    band, which is where a good many of these second sentences live - offers a line that says\n    something of its own, it takes the turn: \"Two or three lines is plenty: what happened, what\n    you want, and by when\" is a better answer to \"what do i say to her\" than the falls paragraph\n    the person read five turns ago.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&candidates.clone()))) {
    return index.clone();
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&op_ge(&_priority(&[geti(&candidates.clone(), &index.clone())]), &g_SAFETY())) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
    heard = _said_before(&[memory.clone(), text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&heard.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&heard.clone(), &_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]))) } }) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
    last = _last_reply(&[memory.clone()]);
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    other = __unpk7[0].clone();
    candidate = __unpk7[1].clone();
    if py_truthy(&Py::Bool(py_eq(&other.clone(), &index.clone()))) {
    continue;
    }
    if py_truthy(&op_ge(&_priority(&[candidate.clone()]), &g_SAFETY())) {
    continue;
    }
    reply = mcall(&candidate.clone(), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&heard.clone(), &_content(&[reply.clone()]))); if py_truthy(&__b) { __b } else { _echoes(&[reply.clone(), last.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "is_a_hedge", &[reply.clone()]); if py_truthy(&__b) { __b } else { _points_back(&[reply.clone()]) } }) {
    continue;
    }
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&_content(&[reply.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__SAID_BEFORE_WORDS())) {
    continue;
    }
    return py_tuple(vec![Py::Bool(true), other.clone()]);
    }
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
    return index.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return index.clone();
    Py::None
}
pub fn _a_recall_from_an_earlier_turn(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this line asks for something said earlier, and there is an earlier turn to hold it.\n\n    `recall.ask` reads the fact-shaped asks (\"what is my name\", \"do you remember my budget\"),\n    `repeat.asked_back` reads the asks for a figure the person gave (\"what was my target again\"),\n    and `lineback.an_earlier_turn` reads every other ask for something said earlier, including the\n    ones that name the kind of detail and no past act (\"and what make is the boiler\" against their\n    \"its a worcester bosch\"). All three are the memory lane's readings; this only asks whether any\n    of them has an answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory", "there_is_a_chat", &[memory.clone()])))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__recall(), &Py::None)); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.recall", "ask", &[text.clone()]) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__repeat(), &Py::None)); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.repeat", "asked_back", &[text.clone(), memory.clone()]) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    let __try11 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = Py::Bool(!py_is(&g__lineback(), &Py::None)); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.lineback", "an_earlier_turn", &[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret12: Py = Py::Bool(false);
    let mut __val13: Py = Py::None;
    let mut __unhandled14: Option<exc::PyExc> = None;
    match __try11 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret12 = __p[0].clone(); __val13 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled15 = false;
            if !__handled15 && exc::matches_any(&__e, &["Exception"]) {
                __handled15 = true;
    return Py::Bool(false);
            }
            if !__handled15 { __unhandled14 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled14 { exc::raise(__e); }
    if py_truthy(&__ret12) { return __val13; }
    Py::None
}
pub fn _the_kind_line_they_gave(args: &[Py]) -> Py {
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
    let _ = py_str("The earlier line of theirs that gives the kind of detail this line asks for, or None.\n\n    Read by the memory lane (`bot/memory/kinds.py`, through `bot/memory/lineback.py`), so the\n    arbiter keeps no vocabulary of its own.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&g__lineback(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory", "there_is_a_chat", &[memory.clone()]))) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), pyrt::xrefs::xcall("memory.lineback", "the_kind_they_gave", &[text.clone(), memory.clone()])]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _seed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The seed rule: the first real candidate, or the first fallback.");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&op_ge(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    return index.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _priority(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidate"))) };
    let mut priority: Py = Py::None;
    priority = mcall(&candidate.clone(), "get", &[py_str("priority"), Py::Int(0)]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[priority.clone(), py_func("bool", |args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[priority.clone(), py_tuple(vec![py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e))), py_func("float", |args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Int(0);
    }
    return priority.clone();
    Py::None
}
pub fn _the_food_the_line_refused(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The foods the line says somebody at the table will not eat, read by the kitchen.\n\n    The kitchen's list also names what the cupboard has run out of, and reads \"not allergic\" as\n    an allergy, so those two shapes are left out here: neither is a reason to pass over an answer.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__cooking(), &Py::None))) {
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&understood.clone())) } }) {
    understood = py_dict(vec![(py_str("clean"), text.clone())]);
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g__ALLERGY_LIFTED(), "search", &[said.clone()])) {
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("getattr", &[g__cooking(), py_str("SHORTAGE_WORDS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("frozenset", &[pyrt::xrefs::xcall("knowledge.cooking", "refused_foods", &[understood.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _offers_the_food(args: &[Py]) -> Py {
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
    let mut refused: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "refused") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: refused"))) };
    let mut first: Py = Py::None;
    let mut food: Py = Py::None;
    let mut names: Py = Py::None;
    let mut near: Py = Py::None;
    let mut place: Py = Py::None;
    let mut undone: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the first sentence of the reply offers a food the line refused.\n\n    Only the first mention of each food is read, and only the words just around it: a reply that\n    opens on the refusal (\"No mushrooms then: leave them out\", \"Egg free: ...\", \"nut-free\") has\n    answered it, while \"Half the plate vegetables...\" has not. The first sentence is what the\n    person reads before deciding whether the answer heard them.\n    ");
    first = mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)), "lower", &[]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), first.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "replace", &[py_str("'"), py_str("")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&refused.clone());
    for __item in __iter1 {
    food = __item.clone();
    names = py_set(vec![food.clone(), op_add(&food.clone(), &py_str("s"))]);
    if py_truthy(&mcall(&food.clone(), "endswith", &[py_str("y")])) {
    let _ = mcall(&names.clone(), "add", &[op_add(&slice_or_raise(&food.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies"))]);
    }
    if py_truthy(&mcall(&food.clone(), "endswith", &[py_str("f")])) {
    let _ = mcall(&names.clone(), "add", &[op_add(&slice_or_raise(&food.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ves"))]);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    place = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&names.clone(), &word.clone()))) {
    continue;
    }
    near = slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&place.clone(), &Py::Int(5))]).unwrap_or_else(|e| exc::raise(e)), &op_add(&place.clone(), &Py::Int(5)), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&near.clone()) { let mut undone: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__REFUSAL_UNDONE(), &undone.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    break;
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _offers_what_the_line_ruled_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidate"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _kept: Py = Py::None;
    let mut _opening: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("True when a candidate hands back the very thing the person has just ruled out.\n\n    \"what should i cook tonight\" / \"not curry, something else\" was answered \"Quick: chickpea curry\n    in 25, or a sweet potato curry if you have 30 minutes\" - the thing the line refused, twice -\n    because the kitchen's answer outranks the composer's claim on PRIORITY, and the rule that\n    passes over a reply offering a refused *food* only runs after the claims. The rule-out reading\n    already knows the thing, so it is read here once and the claim is not given up to a candidate\n    that offers it. A reply that names the thing in order to leave it out (\"none of that was a\n    curry\") is answering the refusal, and keeps the turn.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__ruledout(), &Py::None))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let __unpk6 = py_unpack(&pyrt::xrefs::xcall("compose.ruledout", "_rules_out_what_i_never_said", &[text.clone(), memory.clone()]), 3, None);
    thing = __unpk6[0].clone();
    _kept = __unpk6[1].clone();
    _opening = __unpk6[2].clone();
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
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::Bool(false);
    }
    return _offers_the_food(&[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }, call_builtin("frozenset", &[py_tuple(vec![thing.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _the_thing_the_line_ruled_out(args: &[Py]) -> Py {
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
    let mut _kept: Py = Py::None;
    let mut _opening: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing the line rules out of the answer above it, in the person's own words, or \"\".");
    if py_truthy(&Py::Bool(py_is(&g__ruledout(), &Py::None))) {
    return py_str("");
    }
    let __try1 = exc::catch(|| {
    let __unpk6 = py_unpack(&pyrt::xrefs::xcall("compose.ruledout", "_rules_out_what_i_never_said", &[text.clone(), memory.clone()]), 3, None);
    thing = __unpk6[0].clone();
    _kept = __unpk6[1].clone();
    _opening = __unpk6[2].clone();
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return { let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    Py::None
}
pub fn _every_real_answer_offers(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut index: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut candidate: Py = Py::None;
    let mut others: Py = Py::None;
    let _ = py_str("True when every answer above the floor hands back the thing the line just ruled out.");
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&candidates.clone()) { let mut candidate: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_is(&candidate.clone(), &geti(&candidates.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&_priority(&[candidate.clone()]), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&_priority(&[candidate.clone()]), &g_SAFETY()); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } } } } } }) {  __out.push(candidate.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&others.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&others.clone()) { let mut candidate: Py = __item.clone(); __out.push(_offers_the_food(&[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }, call_builtin("frozenset", &[py_tuple(vec![thing.clone()])]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_cut_the_line_asked_for(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The answer above with the thing the line ruled out taken out of it, when a module offered it.\n\n    The composer writes that cut (`bot/compose/limit.py`), and it is the answer the line asked for.\n    It reads as a repeat only because most of it is the answer already on the screen, which is what\n    a limit on an answer leaves.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("constraint")]))) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    wanted = pyrt::xrefs::xcall("compose.limit", "reply_for", &[understood.clone(), memory.clone()]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    wanted = mcall(&call_builtin("str", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    candidate = __unpk7[1].clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), &wanted.clone()))) {
    return index.clone();
    }
    }
    Py::None
}
pub fn _the_figures_they_gave(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The numbers in the person's own recent lines, which a reply may say back without a unit.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    said = py_list(vec![]);
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { turns.clone() } else { py_list(vec![]) }, &op_neg(&Py::Int(8)), &Py::None, &Py::None));
    for __item in __iter1 {
    one = __item.clone();
    text = if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&one.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[mcall(&g_A_FIGURE(), "findall", &[text.clone()])]);
    }
    }
    return call_builtin("frozenset", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_back_one_of(args: &[Py]) -> Py {
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
    let mut figures: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figures") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figures"))) };
    let mut one: Py = Py::None;
    let _ = py_str("True when the reply names one of the figures the person gave, in any unit or none.");
    if py_truthy(&Py::Bool(!py_truthy(&figures.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_FIGURE(), "findall", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&figures.clone(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_candidates_with_a_figure(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut bare_echo: Py = Py::None;
    let mut best: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut has_figure: Py = Py::None;
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut money: Py = Py::None;
    let mut new_figures: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut quotes: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says_it_back: Py = Py::None;
    let mut signal: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut to_work_out: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut with_figure: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The candidates whose reply carries the figure a how-much line asks for, or the pool.\n\n    \"my car needs a service\" / \"how much\" was answered \"12 months.\" by the memory store, which\n    had the same card's duration and not its £150 to £250, while the answers candidate had the\n    price. A how-much line that asks for an amount wants a figure in a unit or a currency, so a\n    candidate without one stands down when another has it. A ceiling ask is left alone, because\n    \"no amount is safe\" is the answer and has no figure. Nothing changes when every candidate\n    lacks a figure, or every one has it: the first candidate still takes the turn.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("wants")]), &py_str("amount"))) } }) {
    return pool.clone();
    }
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow much\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return pool.clone();
    }
    if py_truthy(&mcall(&g_A_CEILING(), "search", &[line.clone()])) {
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("compose.limit", "FRAME"), "match", &[mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])))) {  __out.push(index.clone()); } } py_list(__out) })() };
    return { let __b = plain.clone(); if py_truthy(&__b) { __b } else { pool.clone() } };
    }
    money = mcall(&pyrt::xrefs::xget("memory.narrow", "SIGNALS"), "get", &[py_str("money")]);
    figure = py_tuple(vec![money.clone(), mcall(&pyrt::xrefs::xget("memory.narrow", "SIGNALS"), "get", &[py_str("amount")])]);
    theirs = _the_figures_they_gave(&[memory.clone()]);
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_bitor(&call_builtin("set", &[mcall(&g_WORD(), "findall", &[mcall(&line.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)), &_carried_words(&[memory.clone()]))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_GENERIC(), &word.clone())) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    to_work_out = call_builtin("bool", &[mcall(&g_A_SUM_TO_WORK_OUT(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![]), py_list(vec![])]), 3, None);
    with_figure = __unpk1[0].clone();
    says_it_back = __unpk1[1].clone();
    bare_echo = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&pool.clone());
    for __item in __iter2 {
    index = __item.clone();
    reply = call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    has_figure = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&figure.clone()) { let mut signal: Py = __item.clone(); __out.push({ let __b = signal.clone(); if !py_truthy(&__b) { __b } else { mcall(&signal.clone(), "search", &[reply.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    quotes = _says_back_one_of(&[reply.clone(), theirs.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = has_figure.clone(); if py_truthy(&__b) { __b } else { quotes.clone() } }))) {
    continue;
    }
    if py_truthy(&quotes.clone()) {
    new_figures = op_sub(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_FIGURE(), "findall", &[reply.clone()])) { let mut one: Py = __item.clone(); __out.push(one.clone()); } py_set(__out) })() }, &theirs.clone());
    if py_truthy(&{ let __b = new_figures.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&money.clone(), "search", &[reply.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_WORKED_OUT(), "search", &[reply.clone()]) } } } }) {
    let _ = mcall(&says_it_back.clone(), "append", &[index.clone()]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&new_figures.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&to_work_out.clone())) } }) {
    let _ = mcall(&bare_echo.clone(), "append", &[index.clone()]);
    }
    }
    continue;
    }
    if py_truthy(&subject.clone()) {
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&reply.clone(), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&subject.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(_the_same_word(&[word.clone(), one.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    }
    let _ = mcall(&with_figure.clone(), "append", &[index.clone()]);
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![bare_echo.clone(), says_it_back.clone(), with_figure.clone()]));
    for __item in __iter3 {
    best = __item.clone();
    if py_truthy(&{ let __b = best.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return best.clone();
    }
    }
    return pool.clone();
    Py::None
}
pub fn _amends_a_detail(args: &[Py]) -> Py {
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
    let mut month: Py = Py::None;
    let mut months: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The months a line amends, or an empty set when the line is not an amendment.\n\n    \"actually we fly in september\" four turns into a Spain chat: the answers module matched the\n    word \"fly\" to the fruit-fly entry - the insect - and that card took the turn off the composer's\n    line, which had the month they had just corrected. A knowledge entry that matched a word inside\n    an amendment is about something else.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__AN_AMENDMENT(), "search", &[said.clone()]))) } }) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    months = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__MONTHS(), "findall", &[said.clone()])) { let mut month: Py = __item.clone(); __out.push(mcall(&month.clone(), "lower", &[])); } py_set(__out) })() };
    return if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[months.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { mcall(&memory.clone(), "get", &[py_str("turns")]) } }) { months.clone() } else { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) };
    Py::None
}
pub fn _the_candidates_that_hand_the_detail_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut months: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "months") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: months"))) };
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut holding: Py = Py::None;
    let mut index: Py = Py::None;
    let mut month: Py = Py::None;
    let _ = py_str("The candidates whose reply says the month the line just amended, best first, or the pool.\n\n    Where more than one reply names it, the words cannot say which one is the answer, so the order\n    is left alone. Where none does, the amendment was not about the month and the reading was\n    wrong - the fruit-fly card on \"actually we fly in september\" is a knowledge entry that matched\n    the word \"fly\" inside the amendment, and the reply that says September is the one that heard it.\n    ");
    holding = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&op_bitand(&months.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__MONTHS(), "findall", &[call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut month: Py = __item.clone(); __out.push(mcall(&month.clone(), "lower", &[])); } py_set(__out) })() })) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[holding.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return pool.clone();
    }
    return op_add(&holding.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&index.clone(), &geti(&holding.clone(), &Py::Int(0))))) {  __out.push(index.clone()); } } py_list(__out) })() });
    Py::None
}
pub fn _asks_it_back(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a reply ends by asking the person something, so it picks the turn up again.\n\n    \"Nothing in that says how serious it is. ... Where is the worry?\" has the shape of a yes or a\n    no and says nothing about the boiler: the question at the end asks the person to say the\n    doubt again. \"Yes, that is how it is normally handled. What would matter is a change in it.\"\n    answers, and keeps its turn.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    last = mcall(&mcall(&said.clone(), "replace", &[py_str("\n"), py_str(" ")]), "rstrip", &[]);
    cut = call_builtin("max", &[mcall(&last.clone(), "rfind", &[py_str(". ")]), mcall(&last.clone(), "rfind", &[py_str("! ")]), mcall(&last.clone(), "rfind", &[py_str("? ")])]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(py_contains(&if py_truthy(&op_ge(&cut.clone(), &Py::Int(0))) { slice_or_raise(&last.clone(), &op_add(&cut.clone(), &Py::Int(2)), &Py::None, &Py::None) } else { last.clone() }, &py_str("?")));
    Py::None
}
pub fn _the_candidates_that_give_the_shape(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut giving: Py = Py::None;
    let mut index: Py = Py::None;
    let mut subject_known: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut want: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The candidates whose reply gives the shape of answer the line asked for, or the pool.\n\n    The figure rule below was the first of these and the only one: a how-much line was owed a\n    figure, and a candidate without one stood down when another had it. The same is true of every\n    shape the understanding lane's reading can name - a time, a place, a person, a pick, a yes or\n    a no, the steps, the reason, the meaning - and the shape is not the composer's business, so it\n    is read once, in `bot/compose/fit.py`, and honoured here for all of them. \"my boiler keeps\n    losing pressure\" / \"is that normal\" was answered by the memory store reading the last answer\n    back under a repeat frame while the composer's verdict line was in the candidates.\n\n    Only a candidate that *definitely* misses the shape stands down, and only for one that\n    definitely gives it: where the words cannot tell, the order is left alone. `amount` keeps its\n    own reading, because a figure has to be in a unit or a currency and has to be about the thing\n    the line named, and that care is worth keeping.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return pool.clone();
    }
    want = mcall(&understood.clone(), "get", &[py_str("wants")]);
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    return pool.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&want.clone(), &py_str("amount")))) {
    return _the_candidates_with_a_figure(&[text.clone(), memory.clone(), candidates.clone(), pool.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("compose.fit", "SOFT"), &want.clone()))) {
    return pool.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &py_str("yesno"))); if !py_truthy(&__b) { __b } else { _names_the_answers_figure(&[call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]) } }) {
    return pool.clone();
    }
    if py_truthy(&_a_check_on_the_answer(&[call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return pool.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pool.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&pyrt::xrefs::xcall("compose.fit", "gives", &[call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), want.clone(), text.clone()]), &Py::Bool(false))) } }) {
    return pool.clone();
    }
    words = call_builtin("set", &[mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    subject_known = if py_truthy(&pool.clone()) { _names_the_subject(&[geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), memory.clone(), words.clone()]) } else { Py::Bool(true) };
    giving = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&pool.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut index: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_is(&pyrt::xrefs::xcall("compose.fit", "gives", &[call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), want.clone(), text.clone()]), &Py::Bool(true))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&subject_known.clone())); if py_truthy(&__b) { __b } else { { let __b = _names_the_subject(&[geti(&candidates.clone(), &index.clone()), memory.clone(), words.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_asks_it_back(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]))) } } } } } }) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&giving.clone()))) {
    return pool.clone();
    }
    return op_add(&giving.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&giving.clone(), &index.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() });
    Py::None
}
pub fn _the_line_asks_for_a_doing(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let mut want: Py = Py::None;
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]) } else { Py::None };
    want = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("wants")]) } else { Py::None };
    return Py::Bool(py_contains(&g_DOING_WANTS(), &want.clone()));
    Py::None
}
pub fn _says_what_they_already_said(args: &[Py]) -> Py {
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
    return call_builtin("bool", &[mcall(&g_SAID_BACK(), "match", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_same_word__dup1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut long: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("True for the same word, or a plural or a short ending of it (\"service\"/\"services\").");
    if py_truthy(&Py::Bool(py_eq(&one.clone(), &other.clone()))) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![one.clone(), other.clone()])); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 2, None);
    short = __unpk1[0].clone();
    long = __unpk1[1].clone();
    return { let __b = op_ge(&call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&long.clone(), "startswith", &[short.clone()]); if !py_truthy(&__b) { __b } else { op_le(&op_sub(&call_builtin("len", &[long.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(2)) } } } };
    Py::None
}
pub fn _the_numbers_in(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The numbers a line names, digits and words counting the same.");
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g__FIGURES(), "findall", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__NUMBER_WORDS(), &word.clone()))) {
    let _ = mcall(&said.clone(), "add", &[call_builtin("float", &[geti(&g__NUMBER_WORDS(), &word.clone())]).unwrap_or_else(|e| exc::raise(e))]);
    continue;
    }
    let __try2 = exc::catch(|| {
    let _ = mcall(&said.clone(), "add", &[call_builtin("float", &[mcall(&word.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    return said.clone();
    Py::None
}
pub fn _names_the_answers_figure(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut mine: Py = Py::None;
    let _ = py_str("True when a reply takes up a number the answer above named.\n\n    \"i walk 10 minutes a day\" got an answer that named twenty minutes, and \"will that work\" was\n    answered \"10 minutes is short of the 20 minutes that answer named.\" That is the yes or the no\n    the person asked for - it is the comparison against the method - but the reader that looks for\n    a yes or a no word cannot see one in it, so the turn went to a reply that only sounds like one.\n    A reply that puts the answer's own figure back to the person is answering the check.\n    ");
    mine = _the_numbers_in(&[reply.clone()]);
    return { let __b = call_builtin("bool", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[op_bitand(&mine.clone(), &_the_numbers_in(&[_last_reply(&[memory.clone()])]))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _explains_the_method(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let _ = py_str("True when a reply says why a thing works, which is the better answer to a check on it.\n\n    \"im trying to stop biting my nails\" / \"i chew gum instead\" / \"does that actually work\" was\n    answered \"A bitter-tasting polish from the pharmacy works because it makes you aware of the\n    bite\", and that is the method explained rather than the method agreed with - a person asking\n    whether a thing works is served by the reason as well as by the yes. Only that shape stands\n    aside; a card that sells something else (\"Get a 3kW one for speed\") does not.\n    ");
    flat = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return { let __b = Py::Bool(py_contains(&flat.clone(), &py_str(" because "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &py_str("work"))) } };
    Py::None
}
pub fn _the_figure_they_asked_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut real: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "real") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: real"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut figure: Py = Py::None;
    let mut index: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The candidate that gives back the figure the line asked for, or None.\n\n    \"Is there an earlier turn?\" is answered once per turn, before any module runs\n    (`memory[\"understood\"][\"looks_back\"]`, read by bot/memory/recall.py), and this is that answer\n    used as one rule instead of a check inside each module: a line pointing back at a figure of\n    the person's own gets the reply that carries the figure. Without it a module that never looks\n    back answers with a figure of its own: \"the flight is 4 hours\" then \"how many hours was the\n    flight\" was \"Two hours.\", and the same shape took \"how many nights is that\" after \"im going\n    for 5 nights\" to a date card.\n\n    Only a figure is enforced here, and only when the store holds one: a look-back whose thing has\n    no figure to carry (\"back to the boiler\") is left to the modules, and a candidate that already\n    says the figure keeps its place in the order.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&g__repeat(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&g__recall(), &Py::None)) } }) {
    return Py::None;
    }
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("looks_back")]))) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    figure = pyrt::xrefs::xcall("memory.repeat", "the_figure_they_gave", &[text.clone(), memory.clone()]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&figure.clone()))) {
    return Py::None;
    }
    let __iter6 = py_iter_or_raise(&real.clone());
    for __item in __iter6 {
    index = __item.clone();
    if py_truthy(&pyrt::xrefs::xcall("memory.repeat", "carries_the_figure", &[figure.clone(), { let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_words_the_report_names(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The content words of a report line - the thing the person says they have.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_CONTENT_WORD(), "findall", &[said.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_REPORT_STOP(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_report_and_no_ask(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line only reports that the person has something.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[mcall(&g_A_MY_OWN_REPORT(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_REPORT_ASKS(), "search", &[said.clone()]))) } };
    Py::None
}
pub fn _is_advice_about(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the reply answers the report: about its thing, and not a definition or a nod.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_REPORT_NOD(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DEFINITION(), "match", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10))) {
    return Py::Bool(false);
    }
    low = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&py_str(" "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _how_much_it_names(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How many of the report's words the reply names - how close it is to what was said.");
    low = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    return call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&low.clone(), &op_add(&py_str(" "), &word.clone())))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_advice_over_a_definition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut chosen: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "chosen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: chosen"))) };
    let mut best_chat: Py = Py::None;
    let mut best_index: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut index: Py = Py::None;
    let mut is_chat: Py = Py::None;
    let mut low: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut score: Py = Py::None;
    let mut won: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The candidate that advises about the thing the line reports, when the turn went to a fact.\n\n    Reads only the pool of real answers, only when the reply that won is the definition of the\n    thing the line reports, and only when another candidate answers that same thing. Returns None\n    for every other line, so nothing else changes.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_report_and_no_ask(&[text.clone()]))) } }) {
    return Py::None;
    }
    words = _the_words_the_report_names(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    won = { let __b = mcall(&geti(&candidates.clone(), &chosen.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } };
    low = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), mcall(&call_builtin("str", &[won.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_DEFINITION(), "match", &[won.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&py_str(" "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0), Py::Bool(true)]), 3, None);
    best_index = __unpk1[0].clone();
    best_score = __unpk1[1].clone();
    best_chat = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&pool.clone());
    for __item in __iter2 {
    index = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &chosen.clone()))) {
    continue;
    }
    reply = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]);
    if py_truthy(&Py::Bool(!py_truthy(&_is_advice_about(&[reply.clone(), words.clone()])))) {
    continue;
    }
    score = _how_much_it_names(&[reply.clone(), words.clone()]);
    is_chat = Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("chat")));
    if py_truthy(&{ let __b = op_gt(&score.clone(), &best_score.clone()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&score.clone(), &best_score.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best_index.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = best_chat.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&is_chat.clone())) } } } } } } } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![index.clone(), score.clone(), is_chat.clone()]), 3, None);
    best_index = __unpk3[0].clone();
    best_score = __unpk3[1].clone();
    best_chat = __unpk3[2].clone();
    }
    }
    return best_index.clone();
    Py::None
}
pub fn _choose(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut _hears_the_news: Py = Py::None;
    let mut about_them: Py = Py::None;
    let mut added: Py = Py::None;
    let mut advice: Py = Py::None;
    let mut already_read: Py = Py::None;
    let mut amended: Py = Py::None;
    let mut answering: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut back: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut bot_answer: Py = Py::None;
    let mut bye: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut cause: Py = Py::None;
    let mut child_words: Py = Py::None;
    let mut choice: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut drafted: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut every: Py = Py::None;
    let mut explained: Py = Py::None;
    let mut fit_carried: Py = Py::None;
    let mut fit_own: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut front: Py = Py::None;
    let mut gave: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut handed: Py = Py::None;
    let mut handed_back: Py = Py::None;
    let mut heard: Py = Py::None;
    let mut hearing_it: Py = Py::None;
    let mut held: Py = Py::None;
    let mut here: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut hurt: Py = Py::None;
    let mut i: Py = Py::None;
    let mut index: Py = Py::None;
    let mut instead: Py = Py::None;
    let mut keeping: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut last: Py = Py::None;
    let mut left: Py = Py::None;
    let mut line: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut more: Py = Py::None;
    let mut named: Py = Py::None;
    let mut own: Py = Py::None;
    let mut own_taste: Py = Py::None;
    let mut paused: Py = Py::None;
    let mut plainer: Py = Py::None;
    let mut polite: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut real: Py = Py::None;
    let mut recall_from_earlier: Py = Py::None;
    let mut refused: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut respecting: Py = Py::None;
    let mut revised: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says: Py = Py::None;
    let mut score: Py = Py::None;
    let mut shorter: Py = Py::None;
    let mut shortest: Py = Py::None;
    let mut step: Py = Py::None;
    let mut story: Py = Py::None;
    let mut sudden: Py = Py::None;
    let mut take: Py = Py::None;
    let mut target: Py = Py::None;
    let mut unheard: Py = Py::None;
    let mut verdict: Py = Py::None;
    let mut was: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut with_figure: Py = Py::None;
    let mut words: Py = Py::None;
    real = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk6423 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk6423[0].clone(); let mut candidate: Py = __unpk6423[1].clone(); if py_truthy(&op_ge(&_priority(&[candidate.clone()]), &Py::Int(0))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&real.clone()))) {
    return Py::Int(0);
    }
    last = _last_reply(&[memory.clone()]);
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { _the_line_is_news(&[text.clone()]) } }) {
pub fn __nested__hears_the_news_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _hears_the_news: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut about_them: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut added: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut advice: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut already_read: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut amended: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut answering: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut ask: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut back: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut best: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut best_score: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut bot_answer: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut bye: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut candidate: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut candidates: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut carried: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut cause: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut child_words: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut choice: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut clean: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut corrected: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut cut: Py = __pos.get(21).cloned().unwrap_or(Py::None);
    let mut dated: Py = __pos.get(22).cloned().unwrap_or(Py::None);
    let mut detail: Py = __pos.get(23).cloned().unwrap_or(Py::None);
    let mut drafted: Py = __pos.get(24).cloned().unwrap_or(Py::None);
    let mut entry: Py = __pos.get(25).cloned().unwrap_or(Py::None);
    let mut every: Py = __pos.get(26).cloned().unwrap_or(Py::None);
    let mut explained: Py = __pos.get(27).cloned().unwrap_or(Py::None);
    let mut fit_carried: Py = __pos.get(28).cloned().unwrap_or(Py::None);
    let mut fit_own: Py = __pos.get(29).cloned().unwrap_or(Py::None);
    let mut fresh: Py = __pos.get(30).cloned().unwrap_or(Py::None);
    let mut front: Py = __pos.get(31).cloned().unwrap_or(Py::None);
    let mut gave: Py = __pos.get(32).cloned().unwrap_or(Py::None);
    let mut gone: Py = __pos.get(33).cloned().unwrap_or(Py::None);
    let mut handed: Py = __pos.get(34).cloned().unwrap_or(Py::None);
    let mut handed_back: Py = __pos.get(35).cloned().unwrap_or(Py::None);
    let mut heard: Py = __pos.get(36).cloned().unwrap_or(Py::None);
    let mut hearing_it: Py = __pos.get(37).cloned().unwrap_or(Py::None);
    let mut held: Py = __pos.get(38).cloned().unwrap_or(Py::None);
    let mut here: Py = __pos.get(39).cloned().unwrap_or(Py::None);
    let mut hours: Py = __pos.get(40).cloned().unwrap_or(Py::None);
    let mut hurt: Py = __pos.get(41).cloned().unwrap_or(Py::None);
    let mut i: Py = __pos.get(42).cloned().unwrap_or(Py::None);
    let mut index: Py = __pos.get(43).cloned().unwrap_or(Py::None);
    let mut instead: Py = __pos.get(44).cloned().unwrap_or(Py::None);
    let mut keeping: Py = __pos.get(45).cloned().unwrap_or(Py::None);
    let mut kept: Py = __pos.get(46).cloned().unwrap_or(Py::None);
    let mut kind: Py = __pos.get(47).cloned().unwrap_or(Py::None);
    let mut last: Py = __pos.get(48).cloned().unwrap_or(Py::None);
    let mut left: Py = __pos.get(49).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(50).cloned().unwrap_or(Py::None);
    let mut longest: Py = __pos.get(51).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(52).cloned().unwrap_or(Py::None);
    let mut more: Py = __pos.get(53).cloned().unwrap_or(Py::None);
    let mut named: Py = __pos.get(54).cloned().unwrap_or(Py::None);
    let mut own: Py = __pos.get(55).cloned().unwrap_or(Py::None);
    let mut own_taste: Py = __pos.get(56).cloned().unwrap_or(Py::None);
    let mut paused: Py = __pos.get(57).cloned().unwrap_or(Py::None);
    let mut plainer: Py = __pos.get(58).cloned().unwrap_or(Py::None);
    let mut polite: Py = __pos.get(59).cloned().unwrap_or(Py::None);
    let mut pool: Py = __pos.get(60).cloned().unwrap_or(Py::None);
    let mut real: Py = __pos.get(61).cloned().unwrap_or(Py::None);
    let mut recall_from_earlier: Py = __pos.get(62).cloned().unwrap_or(Py::None);
    let mut refused: Py = __pos.get(63).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(64).cloned().unwrap_or(Py::None);
    let mut respecting: Py = __pos.get(65).cloned().unwrap_or(Py::None);
    let mut revised: Py = __pos.get(66).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(67).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(68).cloned().unwrap_or(Py::None);
    let mut score: Py = __pos.get(69).cloned().unwrap_or(Py::None);
    let mut shorter: Py = __pos.get(70).cloned().unwrap_or(Py::None);
    let mut shortest: Py = __pos.get(71).cloned().unwrap_or(Py::None);
    let mut step: Py = __pos.get(72).cloned().unwrap_or(Py::None);
    let mut story: Py = __pos.get(73).cloned().unwrap_or(Py::None);
    let mut sudden: Py = __pos.get(74).cloned().unwrap_or(Py::None);
    let mut take: Py = __pos.get(75).cloned().unwrap_or(Py::None);
    let mut target: Py = __pos.get(76).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(77).cloned().unwrap_or(Py::None);
    let mut unheard: Py = __pos.get(78).cloned().unwrap_or(Py::None);
    let mut verdict: Py = __pos.get(79).cloned().unwrap_or(Py::None);
    let mut was: Py = __pos.get(80).cloned().unwrap_or(Py::None);
    let mut whole: Py = __pos.get(81).cloned().unwrap_or(Py::None);
    let mut with_figure: Py = __pos.get(82).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(83).cloned().unwrap_or(Py::None);
    let mut index: Py = __pos.get(84).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "index").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return Py::Bool(!py_truthy(&_announces_a_repeat(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])])));
    Py::None
}
    _hears_the_news = {{ let _hears_the_news = _hears_the_news.clone(); let about_them = about_them.clone(); let added = added.clone(); let advice = advice.clone(); let already_read = already_read.clone(); let amended = amended.clone(); let answering = answering.clone(); let ask = ask.clone(); let back = back.clone(); let best = best.clone(); let best_score = best_score.clone(); let bot_answer = bot_answer.clone(); let bye = bye.clone(); let candidate = candidate.clone(); let candidates = candidates.clone(); let carried = carried.clone(); let cause = cause.clone(); let child_words = child_words.clone(); let choice = choice.clone(); let clean = clean.clone(); let corrected = corrected.clone(); let cut = cut.clone(); let dated = dated.clone(); let detail = detail.clone(); let drafted = drafted.clone(); let entry = entry.clone(); let every = every.clone(); let explained = explained.clone(); let fit_carried = fit_carried.clone(); let fit_own = fit_own.clone(); let fresh = fresh.clone(); let front = front.clone(); let gave = gave.clone(); let gone = gone.clone(); let handed = handed.clone(); let handed_back = handed_back.clone(); let heard = heard.clone(); let hearing_it = hearing_it.clone(); let held = held.clone(); let here = here.clone(); let hours = hours.clone(); let hurt = hurt.clone(); let i = i.clone(); let index = index.clone(); let instead = instead.clone(); let keeping = keeping.clone(); let kept = kept.clone(); let kind = kind.clone(); let last = last.clone(); let left = left.clone(); let line = line.clone(); let longest = longest.clone(); let memory = memory.clone(); let more = more.clone(); let named = named.clone(); let own = own.clone(); let own_taste = own_taste.clone(); let paused = paused.clone(); let plainer = plainer.clone(); let polite = polite.clone(); let pool = pool.clone(); let real = real.clone(); let recall_from_earlier = recall_from_earlier.clone(); let refused = refused.clone(); let reply = reply.clone(); let respecting = respecting.clone(); let revised = revised.clone(); let said = said.clone(); let says = says.clone(); let score = score.clone(); let shorter = shorter.clone(); let shortest = shortest.clone(); let step = step.clone(); let story = story.clone(); let sudden = sudden.clone(); let take = take.clone(); let target = target.clone(); let text = text.clone(); let unheard = unheard.clone(); let verdict = verdict.clone(); let was = was.clone(); let whole = whole.clone(); let with_figure = with_figure.clone(); let words = words.clone(); py_func("_hears_the_news", move |args: &[Py]| { let mut full = vec![_hears_the_news.clone(), about_them.clone(), added.clone(), advice.clone(), already_read.clone(), amended.clone(), answering.clone(), ask.clone(), back.clone(), best.clone(), best_score.clone(), bot_answer.clone(), bye.clone(), candidate.clone(), candidates.clone(), carried.clone(), cause.clone(), child_words.clone(), choice.clone(), clean.clone(), corrected.clone(), cut.clone(), dated.clone(), detail.clone(), drafted.clone(), entry.clone(), every.clone(), explained.clone(), fit_carried.clone(), fit_own.clone(), fresh.clone(), front.clone(), gave.clone(), gone.clone(), handed.clone(), handed_back.clone(), heard.clone(), hearing_it.clone(), held.clone(), here.clone(), hours.clone(), hurt.clone(), i.clone(), index.clone(), instead.clone(), keeping.clone(), kept.clone(), kind.clone(), last.clone(), left.clone(), line.clone(), longest.clone(), memory.clone(), more.clone(), named.clone(), own.clone(), own_taste.clone(), paused.clone(), plainer.clone(), polite.clone(), pool.clone(), real.clone(), recall_from_earlier.clone(), refused.clone(), reply.clone(), respecting.clone(), revised.clone(), said.clone(), says.clone(), score.clone(), shorter.clone(), shortest.clone(), step.clone(), story.clone(), sudden.clone(), take.clone(), target.clone(), text.clone(), unheard.clone(), verdict.clone(), was.clone(), whole.clone(), with_figure.clone(), words.clone()]; full.extend_from_slice(args); __nested__hears_the_news_0(&full) }) }};
    hearing_it = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); if py_truthy(&py_call(&_hears_the_news, &[index.clone()])) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&hearing_it.clone()))) {
    hearing_it = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut index: Py = __item.clone(); if py_truthy(&py_call(&_hears_the_news, &[index.clone()])) {  __out.push(index.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = hearing_it.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[hearing_it.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    real = hearing_it.clone();
    }
    }
    best = geti(&real.clone(), &Py::Int(0));
    if py_truthy(&op_ge(&_priority(&[geti(&candidates.clone(), &best.clone())]), &g_SAFETY())) {
    return best.clone();
    }
    amended = _amends_a_detail(&[text.clone(), memory.clone()]);
    if py_truthy(&amended.clone()) {
    every = call_builtin("list", &[call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    handed = _the_candidates_that_hand_the_detail_back(&[amended.clone(), candidates.clone(), every.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&geti(&handed.clone(), &Py::Int(0)), &best.clone()))) {
    real = op_add(&py_list(vec![geti(&handed.clone(), &Py::Int(0))]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&index.clone(), &geti(&handed.clone(), &Py::Int(0))))) {  __out.push(index.clone()); } } py_list(__out) })() });
    best = geti(&handed.clone(), &Py::Int(0));
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.onscreen", "earlier_turn", &[memory.clone()])))) {
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.onscreen", "talks_about_the_conversation", &[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])])))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&fresh.clone()) {
    real = fresh.clone();
    best = geti(&fresh.clone(), &Py::Int(0));
    }
    }
    if py_truthy(&_a_check_on_the_answer(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")])])) {
    explained = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_eq(&index.clone(), &best.clone())); if !py_truthy(&__b) { __b } else { _explains_the_method(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]) } }) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&explained.clone()))) {
    return best.clone();
    }
    best = geti(&explained.clone(), &Py::Int(0));
    }
    back = _the_figure_they_asked_back(&[candidates.clone(), real.clone(), memory.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_is(&back.clone(), &Py::None))) {
    return back.clone();
    }
    cause = _the_cause_asked_for(&[candidates.clone(), real.clone(), text.clone(), best.clone()]);
    if py_truthy(&Py::Bool(!py_is(&cause.clone(), &Py::None))) {
    return cause.clone();
    }
    verdict = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(candidates.clone()); __args.push(text.clone()); __args.push(memory.clone()); __args.push(last.clone()); __kws.extend(vec![("clean".to_string(), text.clone()), ("best".to_string(), best.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _verdict_reply(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&verdict.clone(), &Py::None))) {
    return verdict.clone();
    }
    plainer = _a_plainer_reading_of_the_same_answer(&[candidates.clone(), best.clone(), text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&plainer.clone(), &Py::None))) {
    return plainer.clone();
    }
    handed_back = _the_answer_handed_back(&[candidates.clone(), text.clone(), memory.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&handed_back.clone(), &Py::None))) {
    return handed_back.clone();
    }
    if py_truthy(&_the_kind_line_they_gave(&[text.clone(), memory.clone()])) {
    front = mcall(&geti(&candidates.clone(), &geti(&real.clone(), &Py::Int(0))), "get", &[py_str("module")]);
    let __iter2 = py_iter_or_raise(&real.clone());
    for __item in __iter2 {
    index = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("module")]), &py_str("said_back")))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("chat"), py_str("greeting"), py_str("answers")]), &front.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&index.clone(), &geti(&real.clone(), &Py::Int(0)))) } }) {
    return index.clone();
    }
    }
    }
    }
    held = _as_it_comes(&[text.clone(), memory.clone(), candidates.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&held.clone(), &Py::None))) {
    with_figure = _the_candidates_that_give_the_shape(&[text.clone(), memory.clone(), candidates.clone(), op_add(&py_list(vec![held.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&index.clone(), &held.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() })]);
    if py_truthy(&Py::Bool(!py_contains(&with_figure.clone(), &held.clone()))) {
    held = geti(&with_figure.clone(), &Py::Int(0));
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&geti(&with_figure.clone(), &Py::Int(0)), &held.clone())); if !py_truthy(&__b) { __b } else { { let __b = _the_line_asks_for_a_doing(&[memory.clone()]); if !py_truthy(&__b) { __b } else { _says_what_they_already_said(&[mcall(&geti(&candidates.clone(), &held.clone()), "get", &[py_str("reply")])]) } } } }) {
    held = geti(&with_figure.clone(), &Py::Int(0));
    }
    }
    instead = _claims_instead(&[text.clone(), memory.clone(), candidates.clone(), last.clone(), _priority(&[geti(&candidates.clone(), &held.clone())])]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&instead.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _offers_what_the_line_ruled_out(&[geti(&candidates.clone(), &instead.clone()), text.clone(), memory.clone()]) } }) {
    instead = Py::None;
    }
    return if py_truthy(&Py::Bool(py_is(&instead.clone(), &Py::None))) { held.clone() } else { instead.clone() };
    }
    pool = _the_candidates_that_give_the_shape(&[text.clone(), memory.clone(), candidates.clone(), real.clone()]);
    real = pool.clone();
    answering = _the_candidates_that_answer_the_ask(&[text.clone(), candidates.clone(), pool.clone()]);
    if py_truthy(&Py::Bool(!py_is(&answering.clone(), &Py::None))) {
    pool = answering.clone();
    real = answering.clone();
    }
    refused = _the_food_the_line_refused(&[text.clone(), memory.clone()]);
    if py_truthy(&refused.clone()) {
    respecting = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_offers_the_food(&[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }, refused.clone()])))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = respecting.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[respecting.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    pool = respecting.clone();
    real = respecting.clone();
    }
    }
    cut = _the_cut_the_line_asked_for(&[memory.clone(), candidates.clone()]);
    if py_truthy(&Py::Bool(!py_is(&cut.clone(), &Py::None))) {
    return cut.clone();
    }
    if py_truthy(&last.clone()) {
    heard = _said_before(&[memory.clone(), text.clone()]);
    cut = py_dict(vec![]);
    let __iter3 = py_iter_or_raise(&pool.clone());
    for __item in __iter3 {
    index = __item.clone();
    words = mcall(&_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), "split", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&heard.clone());
    for __item in __iter4 {
    line = __item.clone();
    was = mcall(&line.clone(), "split", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[was.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _a_run_of(&[was.clone(), words.clone()]) } }) {
    let _ = py_setitem(&cut.clone(), &index.clone(), was.clone());
    break;
    }
    }
    }
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_eq(&_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), &last.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_echoes(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), last.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "is_a_hedge", &[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_points_back(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]))) } } } } } }) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&cut.clone()) {
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&cut.clone(), &index.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut take: Py = __item.clone(); for __item in py_iter_or_raise(&cut.clone()) { let mut gone: Py = __item.clone(); __out.push(_a_run_of(&[geti(&cut.clone(), &gone.clone()), mcall(&_content(&[mcall(&geti(&candidates.clone(), &take.clone()), "get", &[py_str("reply")])]), "split", &[])])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    fresh = held.clone();
    }
    }
    if py_truthy(&{ let __b = fresh.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&heard.clone(), &_content(&[mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")])]))) } }) {
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&heard.clone(), &_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])])))) {  __out.push(index.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { fresh.clone() } };
    }
    if py_truthy(&{ let __b = fresh.clone(); if !py_truthy(&__b) { __b } else { { let __b = _a_repeat_to_hold(&[memory.clone(), text.clone()]); if !py_truthy(&__b) { __b } else { _echoes(&[mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")]), last.clone()]) } } } }) {
    fresh = py_list(vec![]);
    }
    if py_truthy(&fresh.clone()) {
    pool = fresh.clone();
    } else {
    if py_truthy(&op_gt(&call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    shortest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __kws.extend(vec![("key".to_string(), { let _hears_the_news = _hears_the_news.clone(); let about_them = about_them.clone(); let added = added.clone(); let advice = advice.clone(); let already_read = already_read.clone(); let amended = amended.clone(); let answering = answering.clone(); let ask = ask.clone(); let back = back.clone(); let best = best.clone(); let best_score = best_score.clone(); let bot_answer = bot_answer.clone(); let bye = bye.clone(); let candidate = candidate.clone(); let candidates = candidates.clone(); let carried = carried.clone(); let cause = cause.clone(); let child_words = child_words.clone(); let choice = choice.clone(); let clean = clean.clone(); let corrected = corrected.clone(); let cut = cut.clone(); let dated = dated.clone(); let detail = detail.clone(); let drafted = drafted.clone(); let entry = entry.clone(); let every = every.clone(); let explained = explained.clone(); let fit_carried = fit_carried.clone(); let fit_own = fit_own.clone(); let fresh = fresh.clone(); let front = front.clone(); let gave = gave.clone(); let gone = gone.clone(); let handed = handed.clone(); let handed_back = handed_back.clone(); let heard = heard.clone(); let hearing_it = hearing_it.clone(); let held = held.clone(); let here = here.clone(); let hours = hours.clone(); let hurt = hurt.clone(); let i = i.clone(); let index = index.clone(); let instead = instead.clone(); let keeping = keeping.clone(); let kept = kept.clone(); let kind = kind.clone(); let last = last.clone(); let left = left.clone(); let line = line.clone(); let longest = longest.clone(); let memory = memory.clone(); let more = more.clone(); let named = named.clone(); let own = own.clone(); let own_taste = own_taste.clone(); let paused = paused.clone(); let plainer = plainer.clone(); let polite = polite.clone(); let pool = pool.clone(); let real = real.clone(); let recall_from_earlier = recall_from_earlier.clone(); let refused = refused.clone(); let reply = reply.clone(); let respecting = respecting.clone(); let revised = revised.clone(); let said = said.clone(); let says = says.clone(); let score = score.clone(); let shorter = shorter.clone(); let shortest = shortest.clone(); let step = step.clone(); let story = story.clone(); let sudden = sudden.clone(); let take = take.clone(); let target = target.clone(); let text = text.clone(); let unheard = unheard.clone(); let verdict = verdict.clone(); let was = was.clone(); let whole = whole.clone(); let with_figure = with_figure.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let _hears_the_news = _hears_the_news.clone(); let about_them = about_them.clone(); let added = added.clone(); let advice = advice.clone(); let already_read = already_read.clone(); let amended = amended.clone(); let answering = answering.clone(); let ask = ask.clone(); let back = back.clone(); let best = best.clone(); let best_score = best_score.clone(); let bot_answer = bot_answer.clone(); let bye = bye.clone(); let candidate = candidate.clone(); let candidates = candidates.clone(); let carried = carried.clone(); let cause = cause.clone(); let child_words = child_words.clone(); let choice = choice.clone(); let clean = clean.clone(); let corrected = corrected.clone(); let cut = cut.clone(); let dated = dated.clone(); let detail = detail.clone(); let drafted = drafted.clone(); let entry = entry.clone(); let every = every.clone(); let explained = explained.clone(); let fit_carried = fit_carried.clone(); let fit_own = fit_own.clone(); let fresh = fresh.clone(); let front = front.clone(); let gave = gave.clone(); let gone = gone.clone(); let handed = handed.clone(); let handed_back = handed_back.clone(); let heard = heard.clone(); let hearing_it = hearing_it.clone(); let held = held.clone(); let here = here.clone(); let hours = hours.clone(); let hurt = hurt.clone(); let i = i.clone(); let index = index.clone(); let instead = instead.clone(); let keeping = keeping.clone(); let kept = kept.clone(); let kind = kind.clone(); let last = last.clone(); let left = left.clone(); let line = line.clone(); let longest = longest.clone(); let memory = memory.clone(); let more = more.clone(); let named = named.clone(); let own = own.clone(); let own_taste = own_taste.clone(); let paused = paused.clone(); let plainer = plainer.clone(); let polite = polite.clone(); let pool = pool.clone(); let real = real.clone(); let recall_from_earlier = recall_from_earlier.clone(); let refused = refused.clone(); let reply = reply.clone(); let respecting = respecting.clone(); let revised = revised.clone(); let said = said.clone(); let says = says.clone(); let score = score.clone(); let shorter = shorter.clone(); let shortest = shortest.clone(); let step = step.clone(); let story = story.clone(); let sudden = sudden.clone(); let take = take.clone(); let target = target.clone(); let text = text.clone(); let unheard = unheard.clone(); let verdict = verdict.clone(); let was = was.clone(); let whole = whole.clone(); let with_figure = with_figure.clone(); let words = words.clone(); let i: Py = __kwp(args, 0, "i").unwrap_or(Py::None); call_builtin("len", &[_content(&[mcall(&geti(&candidates.clone(), &i.clone()), "get", &[py_str("reply")])])]).unwrap_or_else(|e| exc::raise(e)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    longest = call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut i: Py = __item.clone(); __out.push(call_builtin("len", &[_content(&[mcall(&geti(&candidates.clone(), &i.clone()), "get", &[py_str("reply")])])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = _echoes(&[mcall(&geti(&candidates.clone(), &shortest.clone()), "get", &[py_str("reply")]), last.clone()]); if !py_truthy(&__b) { __b } else { op_le(&op_mul(&call_builtin("len", &[_content(&[mcall(&geti(&candidates.clone(), &shortest.clone()), "get", &[py_str("reply")])])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)), &op_mul(&longest.clone(), &Py::Int(2))) } }) {
    return shortest.clone();
    }
    }
    shorter = _a_shorter_said_again(&[candidates.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&shorter.clone(), &Py::None))) {
    return shorter.clone();
    }
    said = _said_it_already(&[candidates.clone(), text.clone(), memory.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&said.clone(), &Py::None))) {
    pool = py_list(vec![said.clone()]);
    } else {
    choice = _a_choice_the_answer_did_not_make(&[candidates.clone(), text.clone(), memory.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&choice.clone(), &Py::None))) {
    pool = py_list(vec![choice.clone()]);
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), &last.clone()))) {  __out.push(index.clone()); } } py_list(__out) })() }))) {
    unheard = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk294 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk294[0].clone(); let mut candidate: Py = __unpk294[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_echoes(&[mcall(&candidate.clone(), "get", &[py_str("reply")]), last.clone()]))); if !py_truthy(&__b) { __b } else { _answers_this_line(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]) } }) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&unheard.clone()) {
    pool = unheard.clone();
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); __out.push(_points_back(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    says = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut index: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_points_back(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "is_a_hedge", &[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]))); if !py_truthy(&__b) { __b } else { _carries_the_answer(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), memory.clone()]) } } } }) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&says.clone()) {
    pool = says.clone();
    }
    }
    }
    }
    }
    }
    }
    detail = _a_detail_added_to_the_news(&[candidates.clone(), pool.clone(), text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&detail.clone(), &Py::None))) {
    return detail.clone();
    }
    named = _a_card_that_names_the_line(&[candidates.clone(), pool.clone(), text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&named.clone(), &Py::None))) {
    return named.clone();
    }
    added = _the_answer_that_names_what_the_line_added(&[candidates.clone(), pool.clone(), text.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&added.clone(), &Py::None))) {
    return added.clone();
    }
    best = geti(&pool.clone(), &Py::Int(0));
    let __unpk5 = py_unpack(&_about(&[text.clone(), memory.clone()]), 4, None);
    own = __unpk5[0].clone();
    carried = __unpk5[1].clone();
    words = __unpk5[2].clone();
    clean = __unpk5[3].clone();
    child_words = _the_words_for_the_child(&[candidates.clone(), clean.clone(), memory.clone(), last.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&child_words.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_whole_line_module_spoke(&[candidates.clone()]))) } }) {
    return child_words.clone();
    }
    polite = _only_acknowledges(&[memory.clone()]);
    here = _a_presence_check(&[memory.clone()]);
    ask = _the_ask_about_a_child(&[candidates.clone(), clean.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&ask.clone(), &Py::None))) {
    return ask.clone();
    }
    gave = _the_name_they_gave(&[candidates.clone(), text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&gave.clone(), &Py::None))) {
    return gave.clone();
    }
    drafted = _draft_index(&[candidates.clone(), last.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(!py_is(&drafted.clone(), &Py::None))) {
    return drafted.clone();
    }
    about_them = _about_the_person_the_line_names(&[candidates.clone(), pool.clone(), clean.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&about_them.clone(), &Py::None))) {
    return about_them.clone();
    }
    instead = _instead(&[text.clone(), candidates.clone(), last.clone(), own.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(!py_is(&instead.clone(), &Py::None))) {
    return instead.clone();
    }
    bye = _leaving_reply(&[candidates.clone(), own.clone(), carried.clone(), words.clone(), clean.clone(), text.clone(), memory.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&bye.clone(), &Py::None))) {
    return bye.clone();
    }
    left = _a_leaving_said_plain(&[candidates.clone(), pool.clone(), clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&left.clone(), &Py::None))) {
    return left.clone();
    }
    sudden = _sudden_loss_reply(&[candidates.clone(), pool.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(!py_is(&sudden.clone(), &Py::None))) {
    return sudden.clone();
    }
    bot_answer = _about_the_bot(&[candidates.clone(), clean.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&bot_answer.clone(), &Py::None))) {
    return bot_answer.clone();
    }
    hours = _the_venue_hours_turn(&[candidates.clone(), clean.clone(), last.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_is(&hours.clone(), &Py::None))) {
    return hours.clone();
    }
    own_taste = _the_bots_own_taste(&[candidates.clone(), clean.clone(), memory.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&own_taste.clone(), &Py::None))) {
    return own_taste.clone();
    }
    corrected = _store_correcting(&[candidates.clone(), pool.clone(), memory.clone(), text.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&corrected.clone(), &Py::None))) {
    return corrected.clone();
    }
    revised = _the_store_revises_the_answer(&[candidates.clone(), pool.clone(), memory.clone(), text.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&revised.clone(), &Py::None))) {
    return revised.clone();
    }
    step = _the_move_in_hand(&[candidates.clone(), real.clone(), text.clone(), last.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&step.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_points_back(&[mcall(&geti(&candidates.clone(), &step.clone()), "get", &[py_str("reply")])]))) } }) {
    return step.clone();
    }
    whole = _the_whole_move(&[candidates.clone(), real.clone(), text.clone(), last.clone(), _the_answer_written_out(&[memory.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&whole.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_points_back(&[mcall(&geti(&candidates.clone(), &whole.clone()), "get", &[py_str("reply")])]))) } }) {
    return whole.clone();
    }
    more = _the_next_part(&[candidates.clone(), text.clone(), last.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&more.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_points_back(&[mcall(&geti(&candidates.clone(), &more.clone()), "get", &[py_str("reply")])]))) } }) {
    return more.clone();
    }
    dated = _the_date_fact(&[candidates.clone(), real.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_is(&dated.clone(), &Py::None))) {
    return dated.clone();
    }
    hurt = _hurting_reply(&[candidates.clone(), pool.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(!py_is(&hurt.clone(), &Py::None))) {
    return hurt.clone();
    }
    story = _story_goes_on_reply(&[text.clone(), memory.clone(), candidates.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&story.clone(), &Py::None))) {
    return story.clone();
    }
    paused = _pause_reply(&[candidates.clone(), pool.clone(), text.clone(), memory.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_is(&paused.clone(), &Py::None))) {
    return paused.clone();
    }
    let __unpk6 = py_unpack(&_composer_reading(&[text.clone(), memory.clone()]), 2, None);
    kind = __unpk6[0].clone();
    target = __unpk6[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&own.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&carried.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&words.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![g_THREAD(), g_CARE_THREAD()]), &kind.clone())) } } } } } }) {
    return best.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![g_THREAD(), g_CARE_THREAD()]), &kind.clone()))) {
    keeping = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(candidates.clone()); __args.push(real.clone()); __args.push(_named_topics(&[memory.clone()])); __args.push(target.clone()); __args.push(last.clone()); __kws.extend(vec![("care_about".to_string(), if py_truthy(&Py::Bool(py_eq(&kind.clone(), &g_CARE_THREAD()))) { clean.clone() } else { py_str("") }), ("memory".to_string(), memory.clone()), ("line".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thread_index(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&keeping.clone(), &Py::None))) {
    return keeping.clone();
    }
    }
    if py_truthy(&{ let __b = polite.clone(); if py_truthy(&__b) { __b } else { here.clone() } }) {
    return geti(&pool.clone(), &Py::Int(0));
    }
    let __unpk7 = py_unpack(&_fit(&[own.clone(), carried.clone(), memory.clone(), _priority(&[geti(&candidates.clone(), &best.clone())]), { let __b = mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }, { let __b = mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }, text.clone()]), 2, None);
    fit_own = __unpk7[0].clone();
    fit_carried = __unpk7[1].clone();
    best_score = _match(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("module")]), fit_own.clone(), fit_carried.clone(), words.clone(), clean.clone()]);
    kept = _carries_the_answer(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]), memory.clone()]);
    named = _named_topics(&[memory.clone()]);
    if py_truthy(&_is_a_generic_step(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")])])) {
    let __iter8 = py_iter_or_raise(&slice_or_raise(&pool.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter8 {
    index = __item.clone();
    entry = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = entry.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_is_a_generic_step(&[entry.clone()]))); if !py_truthy(&__b) { __b } else { _opens_on_a_job(&[entry.clone()]) } } } }) {
    return index.clone();
    }
    }
    }
    recall_from_earlier = _a_recall_from_an_earlier_turn(&[text.clone(), memory.clone()]);
    if py_truthy(&_reads_the_doubt(&[geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0)))])) {
    let __iter9 = py_iter_or_raise(&slice_or_raise(&pool.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter9 {
    index = __item.clone();
    if py_truthy(&_carries_the_answer(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), memory.clone()])) {
    return index.clone();
    }
    }
    }
    let __iter10 = py_iter_or_raise(&slice_or_raise(&pool.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter10 {
    index = __item.clone();
    if py_truthy(&{ let __b = recall_from_earlier.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MEMORY_MODULES(), &mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("module")]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_names_the_subject(&[geti(&candidates.clone(), &index.clone()), memory.clone(), words.clone()]))); if !py_truthy(&__b) { __b } else { _reads_the_doubt(&[geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0)))]) } }) {
    continue;
    }
    score = _match(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("module")]), fit_own.clone(), fit_carried.clone(), words.clone(), clean.clone()]);
    if py_truthy(&op_le(&score.clone(), &best_score.clone())) {
    continue;
    }
    if py_truthy(&{ let __b = _about_the_line(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]), words.clone(), clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_says_only_the_line(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]), words.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_line(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), words.clone(), clean.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__already(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("memory.already", "tells_them_the_step_again", &[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), text.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.already", "tells_them_the_step_again", &[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]), text.clone(), memory.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { op_le(&_match(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("module")]), named.clone(), carried.clone(), words.clone(), clean.clone()]), &Py::Int(0)) } }) {
    entry = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _carries_the_answer(&[entry.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { _opens_on_a_job(&[entry.clone()]) } }))) {
    continue;
    }
    }
    let __unpk11 = py_unpack(&py_tuple(vec![index.clone(), score.clone()]), 2, None);
    best = __unpk11[0].clone();
    best_score = __unpk11[1].clone();
    }
    front = mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = { let __b = _hands_the_decision_back(&[front.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("module")]), &py_str("chat"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_ack_lines(&[]), &_content(&[front.clone()]))) } } } }; if !py_truthy(&__b) { __b } else { { let __b = _asks_a_question(&[memory.clone()]); if py_truthy(&__b) { __b } else { _asks_for_a_judgement(&[text.clone(), memory.clone()]) } } } }) {
    already_read = _said_before(&[memory.clone(), text.clone()]);
    let __iter12 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter12 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &best.clone())); if py_truthy(&__b) { __b } else { op_ge(&_priority(&[geti(&candidates.clone(), &index.clone())]), &g_SAFETY()) } }) {
    continue;
    }
    reply = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&already_read.clone(), &_content(&[reply.clone()]))); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { _echoes(&[reply.clone(), last.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&_names_what_they_said(&[reply.clone(), text.clone(), memory.clone()])) {
    return index.clone();
    }
    }
    }
    advice = _the_advice_over_a_definition(&[candidates.clone(), pool.clone(), text.clone(), best.clone()]);
    if py_truthy(&Py::Bool(!py_is(&advice.clone(), &Py::None))) {
    return advice.clone();
    }
    return _the_card_that_places_the_figure(&[candidates.clone(), best.clone(), text.clone(), memory.clone()]);
    Py::None
}
pub fn _a_bare_doubt_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidate"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut own: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("Is this candidate the composer's own reply to a doubt with no range behind it?\n\n    \"my electricity is 90 a month\" / \"is that a lot\" is a doubt about a figure, and the composer's\n    answer to a doubt with nothing to judge against asks for a second figure (\"90 a month is only\n    high or low next to another figure for the same thing: what you were paying before... What was\n    it before?\"). That is the honest reply when nothing in the chat can place the figure - and it is\n    not the answer when a card can: the electricity card says \"No, that is normal for electricity\n    for a typical home. £90 a month is £1,080 a year, and electricity... is usually £60 to £120 a\n    month\". The pool sits at `chat`'s priority 300 and the card at 0, so the hand-back won.\n\n    The reading is the composer's own call, the same one the module makes (`reply_to_a_bare_doubt`),\n    so the arbiter compares two replies rather than keeping a second copy of a wording.\n    ");
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&g__reading(), &Py::None)) } }) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    own = _content(&[pyrt::xrefs::xcall("compose.reading", "reply_to_a_bare_doubt", &[text.clone(), memory.clone()])]);
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
    return { let __b = call_builtin("bool", &[own.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&own.clone(), &reply.clone())) } };
    Py::None
}
pub fn _places_the_figure(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply puts a number on the thing: a range, a price, a total.\n\n    A card that answers a doubt about a figure names a figure of its own (\"usually £60 to £120 a\n    month\", \"the going rate is 30 to 45 pounds\"). One that names the thing and no number has not\n    placed the figure either, so it does not take the turn off the hand-back.\n    ");
    said = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("[£$\\u20ac]|\\b(?:a|an|per|pounds?|quid|dollars?|euros?)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_card_that_places_the_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut best: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "best") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: best"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut index: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("A card that puts the figure against a range beats the composer's ask for a second figure.\n\n    The operator's rule: the reply that fits the line most closely wins, and a broad \"\n    what are you comparing it with\" is beaten by the one that says what the figure is like. The\n    hand-back is still the answer when no card has a range - that is the whole point of it - so\n    this only moves the turn when a candidate both names what the line is about and carries a\n    figure (`_places_the_figure`).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_a_bare_doubt_reply(&[geti(&candidates.clone(), &best.clone()), text.clone(), memory.clone()])))) {
    return best.clone();
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &best.clone())); if py_truthy(&__b) { __b } else { op_ge(&_priority(&[geti(&candidates.clone(), &index.clone())]), &g_SAFETY()) } }) {
    continue;
    }
    reply = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = _places_the_figure(&[reply.clone()]); if !py_truthy(&__b) { __b } else { _names_what_they_said(&[reply.clone(), text.clone(), memory.clone()]) } }) {
    return index.clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _says_only_the_line(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut fresh: Py = Py::None;
    let mut had: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a reply is the line handed back, in its own words or nearly.\n\n    The guard above asks whether the candidate in front names the subject, and an echo names it\n    by construction: it is built out of the person's own words. So \"she wont eat fish what can i\n    cook\" gave the echo \"They don't eat fish.\" the protection and held back the dish that\n    follows it, because a recipe names potatoes and eggs and never names the fish the person\n    turned down. What the guard is for is an answer about something else that shares a topic\n    label - the sleep card answering a school meeting - so a reply that says nothing beyond the\n    line itself is not the answer the guard is there to keep.\n\n    The test is the share of the reply's own words that the line already has: an echo is all of\n    them, and an answer that carries anything new is not.\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_LETTERS(), "findall", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_ENOUGH())) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    had = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_ENOUGH())) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&had.clone()))) {
    return Py::Bool(false);
    }
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&had.clone()) { let mut one: Py = __item.clone(); __out.push({ let __b = mcall(&word.clone(), "startswith", &[one.clone()]); if py_truthy(&__b) { __b } else { mcall(&one.clone(), "startswith", &[word.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) {
    return Py::Bool(false);
    }
    return op_le(&op_mul(&call_builtin("len", &[fresh.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _about_the_line(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut stems: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a reply names something of the subject the line is on.\n\n    A topic label is not the subject: \"i told the school and nothing happened\" is read as a\n    family-and-parenting line and a school-and-study line, and the entry that answered it was\n    the sleep card (\"Get up after 20 minutes of lying there awake ...\"), which names nothing the\n    person said. The candidate in front named the school and the meeting.\n\n    So a lift by topic alone is checked against the words: a reply that shares one of the line's\n    own subject words is about it, and a shared word is not enough on the other side of the test\n    either - the candidate in front has to name what was said before a lift is held back at all.\n\n    The words are compared as subjects, not as strings of letters. A card about eggs is about a\n    line that says eggs, and `LONG_ENOUGH` dropped every word of three letters, so \"one egg\n    becomes 60g of yoghurt\" named nothing of \"my son wont eat eggs\": the store's echo, which\n    repeats the line, was `_about_the_line` and the card was not, so the lift was held back and\n    the person got \"Your son doesn't eat eggs.\" with no help in it. \"my son wont eat egg\" got the\n    card, because there the two said the same letters. So the same crude stem `_subject_words`\n    uses is what counts here (\"egg\"/\"eggs\", \"bin\"/\"bins\", \"tap\"/\"taps\" - three-letter subjects\n    are ordinary), a filler word is not a subject at all, and only words of four letters or more\n    are matched loosely, on a shared opening.\n    ");
    said = _subject_words(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    stems = _subject_words(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&stems.clone()))) {
    if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_ENOUGH())) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]));
    for __item in __iter2 {
    one = __item.clone();
    if py_truthy(&{ let __b = mcall(&one.clone(), "startswith", &[word.clone()]); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[one.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    continue;
    }
    let __iter3 = py_iter_or_raise(&stems.clone());
    for __item in __iter3 {
    stem = __item.clone();
    let __iter4 = py_iter_or_raise(&said.clone());
    for __item in __iter4 {
    one = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&one.clone(), &stem.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_ENOUGH()); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_ENOUGH()); if !py_truthy(&__b) { __b } else { { let __b = mcall(&one.clone(), "startswith", &[stem.clone()]); if py_truthy(&__b) { __b } else { mcall(&stem.clone(), "startswith", &[one.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    }
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
    let mut candidate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidate"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut reply: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether an entry may take a bare yes-or-no doubt that names no subject of its own.\n\n    \"is that bad\" after \"blood in my pee\", \"is that normal\" after \"my cat sleeps a lot\": the\n    question carries nothing but the doubt, so the subject is the line before it, and an entry\n    that fits by inheriting the conversation's topic can be about something else entirely. The\n    sleep entry answered a cat with \"Most adults do best on seven to nine hours\", the\n    carbohydrate entry answered \"can i freeze rice\" with \"Carbohydrate is not fattening in\n    itself\", and the repeat line answered a humming fridge with \"Everything about normal is in\n    what I said above\". The composer's line for the doubt itself (\"Yes, that is the usual thing\n    - what to keep an eye on is change rather than the thing itself\") was in front of every one\n    of those and lost.\n\n    So on a bare doubt the entry has to name the subject too: a word of the line the conversation\n    is on, in the reply. A card that shares it (the kettling entry answering \"my boiler is making\n    a kettling noise\", the chicken entry answering \"can i freeze the leftovers\") keeps its turn.\n    A card that shares nothing is answering a different question and stands down.\n\n    Only a bare doubt is held to this: a line with words of its own (\"is that bad for the\n    freezer\", \"is that normal for a 14 year old cat\") names its subject itself and is read by\n    the ordinary specialist rule.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_a_bare_yes_no(&[memory.clone()])))) {
    return Py::Bool(true);
    }
    if py_truthy(&_carries_the_answer(&[mcall(&candidate.clone(), "get", &[py_str("reply")]), memory.clone()])) {
    return Py::Bool(true);
    }
    subject = op_sub(&op_sub(&_carried_words(&[memory.clone()]), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &g_DOUBT_WORDS());
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(true);
    }
    reply = { let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } };
    let __iter1 = py_iter_or_raise(&subject.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    continue;
    }
    if py_truthy(&_names_the_thing(&[word.clone(), reply.clone(), op_bitor(&_carried_words(&[memory.clone()]), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_the_thing(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut line_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line_words"))) };
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut token: Py = Py::None;
    let mut tokens: Py = Py::None;
    let _ = py_str("Whether the reply is about the thing `word` names, not just holding the word.\n\n    A shared word is not a shared subject. \"there is blood in my pee\" then \"is that bad\" was\n    answered by the card for a blood test - \"A blood test is a small sting and a vial or two\" -\n    because the card and the line share \"blood\", and the question was about blood in the person's\n    water, not about having blood taken. The card opened on the word and went on to name something\n    the line never did, so it was answering a question nobody asked (#general, the steer on a\n    shared word; deck row `csp-01`).\n\n    So a word of the thread that the reply opens on has to be the thing the opening is about: no\n    fuller name may follow it inside the opening phrase (\"blood test\", \"blood pressure\", \"car\n    seat\" for a line about blood or a car) unless that name is one the line has too (\"blood in your\n    pee\" is the same blood). A reply the word appears later in is left alone: by then it is\n    describing the thing, not naming a different one, so \"A machine that bangs or walks across the\n    floor is almost always an uneven load\" still answers a line about the spin cycle.\n\n    Whole words only, either way round with a plural: a shared pair of letters is not a shared\n    word (\"care\" is not \"car\", \"catch\" is not \"cat\").\n    ");
    tokens = mcall(&_content(&[reply.clone()]), "split", &[]);
    opening = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&tokens.clone());
    for __item in __iter1 {
    token = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FIRST_VERB(), &token.clone()))) {
    break;
    }
    let _ = mcall(&opening.clone(), "append", &[token.clone()]);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    token = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_the_same_plain_word(&[word.clone(), token.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = op_ge(&index.clone(), &call_builtin("len", &[opening.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&token.clone(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    let __iter4 = py_iter_or_raise(&slice_or_raise(&opening.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None));
    let mut __else5 = false;
    for __item in __iter4 {
    after = __item.clone();
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CARRIES_NO_SUBJECT(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&line_words.clone(), &after.clone())) } } } }) {
    break;
    }
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_same_plain_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut token: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("One word said the same way twice: the same letters, or the same word inflected.\n\n    A shared pair of letters is not a shared word - \"care\" is not \"car\", \"spinach\" is not\n    \"spin\", \"bloody\" is not \"blood\" - but the same word in another form still names the same\n    thing, and a reply that says \"toothache\" for a line about a tooth is about that tooth\n    (@deft-apricot in review: the toothache card stopped counting for \"my tooth hurts when i\n    drink tea\").\n\n    So the ending may carry an inflection - the plural spelled either way (`s`, `es`, `ies`),\n    the possessive (`'s`, `s'`) - or `ache`, which is how English makes a hurt into a compound\n    and the only such ending a card here is written with (\"toothache\"). Anything else is a\n    different word: a suffix that makes a new name (`er`, `y`, `ch`, `ach`) is never read as\n    the same thing.\n\n    Named apart from `_the_same_word` further down this file, which is the older reader of a\n    word's forms (`_the_forms_of`) and is the one bound at import time. Two `def`s of one name\n    leave only the later bound, so the earlier one silently never runs.\n    ");
    if py_truthy(&Py::Bool(py_eq(&one.clone(), &other.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![one.clone(), other.clone()]), py_tuple(vec![other.clone(), one.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    token = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&token.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&token.clone(), "startswith", &[word.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_INFLECTED_ENDS(), &slice_or_raise(&token.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&token.clone(), &op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies")))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("'s"), py_str("s'")])]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&token.clone(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ss")]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&token.clone(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _reads_the_doubt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidate"))) };
    let mut composer: Py = Py::None;
    let mut opinions: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply in front of the entry is the composer's own ruling on the doubt.\n\n    A bare doubt (\"is that bad\", \"is that normal\") is answered by the composer's line for it, so\n    an entry that only inherits the conversation's topic stands down (`_names_the_subject`). That\n    trade is only worth making when the line in front actually rules on the doubt: \"is that\n    normal\" after \"my cat sleeps a lot\" gets \"Yes, that is the usual thing - what to keep an eye\n    on is change rather than the thing itself\", which reads the doubt, where the entry it\n    displaced was the human sleep card.\n\n    Three reviewers measured what happens when the front is something else, and every one of them\n    named the same fix:\n\n    - a nod or a farewell. \"Alright then - good luck with it.\" after \"im allergic to shellfish /\n      is that bad\" (@grand-gopher), and \"Sounds good.\" after \"i owe about 4000 on my credit card /\n      im paying 60 a month / is that enough\".\n    - a hand-back or a search hedge. \"I cannot call that one for you\" after \"im thinking of\n      getting a rescue dog / is that a good idea\" displaced the entry that answered (@smooth-lark).\n    - a note about the answer. \"Nothing in that answer needs anyone yet\" after \"my calf hurts / is\n      that bad\" took the turn off the aching-joint card (@smooth-lark).\n    - a floor, which asks how long and rules on nothing. \"How long has it been going on?\" after \"i\n      take ibuprofen most days / is that ok\" took the turn off the dose card (@grand-gopher,\n      @neat-moth).\n    - a card about something else. \"why does my cat scratch its ears / should i clean her ears\"\n      stood the earwax card down for a cat-scratch-wound card, which is a vocabulary gap rather\n      than a reading (@smooth-lark's warning).\n\n    So the test is positive rather than a list of shapes to exclude: the pools in\n    bot/compose/verdict.py that rule on a doubt are named there (`RULED_BY_THE_COMPOSER`,\n    `rules_on_the_doubt`), and anything else leaves the entry its turn. One rule here instead of\n    five negative ones, and a pool that gains a ruling line needs no change in the arbiter.\n    ");
    let __try1 = exc::catch(|| {
    composer = pyrt::xrefs::xmodule("compose.fallback");
    opinions = pyrt::xrefs::xmodule("compose.verdict");
    said = mcall(&candidate.clone(), "get", &[py_str("reply")]);
    return py_tuple(vec![Py::Bool(true), { let __b = mcall(&opinions.clone(), "rules_on_the_doubt", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&composer.clone(), "is_said_it_already", &[said.clone()]) } }]);
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
pub fn _a_bare_yes_no(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let _ = py_str("True when the line asks yes or no and names no topic of its own (\"is that normal\").");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_line_topics(&[memory.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&_labels(&[mcall(&understood.clone(), "get", &[py_str("asks")])]), &py_str("yesno")));
    Py::None
}
pub fn _carried_words(args: &[Py]) -> Py {
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
    let mut labels: Py = Py::None;
    let mut text: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("The words of the line the conversation is on, which a bare follow-up inherits.");
    topic = mcall(&memory.clone(), "get", &[py_str("topic")]);
    if py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    labels = _labels(&[mcall(&topic.clone(), "get", &[py_str("words")])]);
    if py_truthy(&labels.clone()) {
    return labels.clone();
    }
    text = mcall(&topic.clone(), "get", &[py_str("text")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&text.clone(), "strip", &[]) } }) {
    return call_builtin("set", &[mcall(&g_WORD(), "findall", &[mcall(&mcall(&text.clone(), "lower", &[]), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut own: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own"))) };
    let mut carried: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: carried"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut best_priority: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "best_priority") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: best_priority"))) };
    let mut reply: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { py_str("") };
    let mut claimant: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "claimant") { v.clone() }
    else { py_str("") };
    let mut text: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut repeat: Py = Py::None;
    let _ = py_str("The topics the specialist rule may count, and the ones it may only count weakly.\n\n    The reading gives a turn two topic sets: the line's own (`understood[\"topics\"]`) and the one\n    the conversation is on (`understood[\"topic\"]`, which on a follow-up is put back in). Both\n    count 2 in `_match`, because on a follow-up the second is the only topic there is.\n\n    A line that names no topic of its own is not about the topic of the conversation, though: it\n    is about the conversation. \"what am i doing this week\" after \"im writing up my thesis\" was\n    read as a language-and-writing line by inheritance alone, and the essay-thesis entry took\n    the turn from the memory store's own answer (\"You're writing up your thesis.\"), which is the\n    answer to what was asked. So when the candidate in front sits in the recall band (README,\n    \"Routing\": the memory store reads a line about what was said), only a specialist the line\n    itself supports - its own topics, a phrase or a word it contains - may take the turn from\n    it. The recall band is the only band this applies to: the safety band is answered before\n    this function is reached, and the loss and grief bands already answer in place of the\n    entries (`_instead`).\n\n    The same holds for a line that asks for the answer again. Its measure word is read as a topic\n    of its own - \"how long did you say\" is a maths-and-numbers line - but it is not a subject the\n    line is about, so the conversation's topic is not counted for it either: the entry that wrote\n    the answer reads the pointer as the same question and hands the whole answer back, while the\n    memory store has the part the person asked for. See `_asks_for_the_answer_again`.\n\n    And it holds for a line that asks about the conversation in some other way: a recap of the chat\n    is not the topic its words name, so the module in front publishes that reading\n    (the store's `_about_this_chat`) and nothing is counted against the recall band on it. See\n    `_asks_about_the_chat`.\n    ");
    if py_truthy(&op_ge(&best_priority.clone(), &g_RECALL())) {
    repeat = _asks_for_the_answer_again(&[memory.clone()]);
    if py_truthy(&{ let __b = repeat.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_echoes(&[reply.clone(), _last_reply(&[memory.clone()])]))) } }) {
    return py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&repeat.clone())); if !py_truthy(&__b) { __b } else { { let __b = claimant.clone(); if !py_truthy(&__b) { __b } else { _asks_about_the_chat(&[claimant.clone(), text.clone(), memory.clone()]) } } } }) {
    return py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_line_topics(&[memory.clone()])))) {
    return py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), carried.clone()]);
    }
    }
    return py_tuple(vec![own.clone(), carried.clone()]);
    Py::None
}
pub fn _line_topics(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let _ = py_str("The topic labels the understanding engine read from this line by itself, never inherited.");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    return _labels(&[mcall(&understood.clone(), "get", &[py_str("topics")])]);
    Py::None
}
pub fn _an_answer_behind_the_ask(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the last thing the bot said is an answer, so a repeat ask has one to hand back.\n\n    A repeat ask is the answer again, and this file holds the turn for it. Where the bot's last\n    line was a nod - \"that thing you said about the boiler\", \"sorry i lost it\", \"can you say it\n    again\" - there is no answer to say again: the store hands back \"Ugh, that's a lot to carry.\"\n    and the arbiter, told the line is a repeat, passes over the card that did read it. Saying a\n    nod twice answers nothing, so a nod is not an answer behind the ask.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_THE_ANSWER_BANDS(), &mcall(&geti(&said.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("module")])));
    Py::None
}
pub fn _asks_for_the_answer_again(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut frame: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the whole line is a pointer back at what the bot said, with no subject of its own.\n\n    The measure words are read as a topic of their own: \"how long did you say\" and \"when did you\n    say\" after a sprained ankle came back as maths-and-numbers lines, so the specialist rule let\n    the ankle card repeat its whole answer while the memory store's length line - the answer to\n    what was actually asked - sat behind it. A line made only of the pointer and the measure names\n    no topic of its own: it is about the conversation, and the recall band answers it.\n    ");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    text = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__repeat(), &Py::None)); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.repeat", "asks_for_less", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_the_store_reads_a_repeat(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_store_reads_an_explain_again_ask(&[text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_POINTS_BACK(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    frame = _the_repeat_frame(&[]);
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&frame.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _the_repeat_frame(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frame: Py = Py::None;
    let mut store: Py = Py::None;
    let _ = py_str("The words a repeat ask is made of: this file's, and every word the store reads one with.\n\n    The frame here was a second copy of the store's repeat words, and it drifted the way a second\n    copy does: \"pardon\" and \"come\" were repeat words in the store and not in this frame, so the\n    line was left with a subject of its own and \"pardon\" over the boiler answer was read as a new\n    line rather than the ask to hear that answer again. One source for the words and one for the\n    reading (see `_the_store_reads_a_repeat`) is the general form: a word the store accepts in an\n    ask is a word this file does not count as a subject.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__FRAME(), &Py::None))) {
    frame = call_builtin("set", &[g_REPEAT_FRAME()]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    store = pyrt::xrefs::xmodule("modules.memory_store");
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
    store = Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __aug6 = op_bitor(&frame.clone(), &call_builtin("set", &[{ let __b = call_builtin("getattr", &[store.clone(), py_str("REPEAT_WORDS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    frame = __aug6;
    s__FRAME(frame.clone());
    }
    return g__FRAME();
    Py::None
}
pub fn _the_store_reads_a_repeat(args: &[Py]) -> Py {
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
    let mut reader: Py = Py::None;
    let mut store: Py = Py::None;
    let _ = py_str("Whether the store read the whole line as an ask for its last reply again.\n\n    The store owns this reading: it is what decides whether its own reply is the hand-back, and it\n    knows every way people write the ask. The frame in this file is the second copy of it, and a\n    second copy drifts: \"pardon\" and \"come again\" are read as the hand-back's ask by the store -\n    both are in its repeat words - and were not matched here, so over the boiler answer \"pardon\"\n    was passed over as an echo and the podcast card from two turns back took the turn. Asking the\n    store rather than reading the line twice is the general form of the fix: any ask the store\n    answers with its hand-back is a repeat here too.\n    ");
    let __try1 = exc::catch(|| {
    store = pyrt::xrefs::xmodule("modules.memory_store");
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
    reader = call_builtin("getattr", &[store.clone(), py_str("asks_again"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&reader.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&reader, &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::Bool(false);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn _the_store_reads_an_explain_again_ask(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut meant: Py = Py::None;
    let mut name: Py = Py::None;
    let mut reader: Py = Py::None;
    let _ = py_str("Whether the store read the line as \"I did not understand that, say it again\".\n\n    `bot/memory/meant.py` is the one reading of the ask to have the last turn put another way: it\n    answers \"i dont understand\", \"im confused\" and \"i dont follow\" with the answer again under a\n    lead-in, and it does not read \"i understand\" as one, which a word list would. The arbiter's\n    repeat frame is the pointer words (\"again\", \"pardon\", \"come again\") and none of these lines is\n    made of them, so a card that shares a word with \"understand\" took the turn off the hand-back:\n    over the boiled-egg answer, \"i dont understand\" got the warm-flannel card and over the boiler\n    answer it got the forgetting entry.\n    ");
    let __try1 = exc::catch(|| {
    meant = pyrt::xrefs::xmodule("memory.meant");
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
    line = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    let __iter6 = py_iter_or_raise(&py_tuple(vec![py_str("MEAN"), py_str("REPEAT")]));
    for __item in __iter6 {
    name = __item.clone();
    reader = call_builtin("getattr", &[meant.clone(), name.clone(), Py::None]).unwrap_or_else(|e| exc::raise(e));
    let __try7 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&reader.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&reader.clone(), "match", &[line.clone()]) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
    continue;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_repeat_to_hold(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let _ = py_str("Whether this line is an ask to hear the last answer again, with that answer to give back.\n\n    The three halves the routing rules share, so no rule can drift from another: the line is made\n    of nothing but the pointer (see `_asks_for_the_answer_again`), the last thing the bot said is\n    an answer rather than a nod (`_an_answer_behind_the_ask`), and the ask names no measure of its\n    own (`_names_a_figure`). A measure is a part of the answer - \"how long did you say\", \"what time\n    did you say\" - and the store answers those with the part itself (\"Sure - 18 to 20 minutes\");\n    the card that holds the number keeps the turn then, which is what\n    `tests.test_a_repeat_ask_keeps_the_recall` pins.\n    ");
    return { let __b = _asks_for_the_answer_again(&[memory.clone(), text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _an_answer_behind_the_ask(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_figure(&[text.clone()]))) } } } };
    Py::None
}
pub fn _a_choice_the_answer_did_not_make(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The fallback to take the turn when a choice was answered with the rest of the last reply.\n\n    Asked only when every real candidate says the last answer again (see `_choose`). A stock line\n    is no reason to drop an answer, but the rest of an answer is not an answer to a choice: \"is it\n    worth fixing or shall i just get a new one\" after the bang on the spin got the sentence about\n    the transport bolts (a slice of the answer the person had already read past), while the\n    composer's own repair-or-replace pool had the line they needed. So on a line whose reading\n    asks to choose between options (`understood[\"asks\"]` holds `choice`), a fallback with words\n    of its own takes the turn from a repeat.\n\n    Only a choice ask, and only words the last reply does not already hold: the same question\n    asked twice is answered by the same answer, and there the repeat is the reply.\n\n    A shrug is not the choice made, so the two shapes of it are passed over: the hedge that says\n    the bot cannot call it, and the line that only points back at the answer above. \"my fridge is\n    on its last legs\" then \"should i get it repaired or a new one\" had the repair-or-replace card\n    (\"Repair it while a fix is well under half the price of a new one\") in the candidates and was\n    handed the composer's \"Can't call that one for you, sorry. Say what you'd lose either way and\n    I'll lay it out with you\" instead, because it was the only reply in the fallback band whose\n    words the answer above did not already hold. The card answers the choice; the shrug asks for\n    the information again. `_choose`'s own filter drops hedges for the same reason.\n    ");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&_labels(&[mcall(&understood.clone(), "get", &[py_str("asks")])]), &g_CHOICE()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&op_ge(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&reply.clone(), &last.clone())); if py_truthy(&__b) { __b } else { _echoes(&[reply.clone(), last.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "is_a_hedge", &[mcall(&candidate.clone(), "get", &[py_str("reply")])]); if py_truthy(&__b) { __b } else { _points_back(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]) } }) {
    continue;
    }
    return index.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_shorter_said_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The index of the composer's own shorter form of the last answer, else None.\n\n    Asked only when every real candidate only says the last answer again (see `_choose`), and\n    only for a line the composer wrote as a cut of that answer: the frame is the module's and the\n    words behind it are the answer's own, in its own order. That is what makes it new - a person\n    who typed \"what did you say\" has read the paragraph once already - and it is why a stock line\n    in the fallback band is never taken for one.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&op_ge(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "is_the_answer_shorter", &[mcall(&candidate.clone(), "get", &[py_str("reply")]), last.clone()])) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _said_it_already(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut _name: Py = Py::None;
    let mut _says: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last_words: Py = Py::None;
    let mut left: Py = Py::None;
    let mut missing: Py = Py::None;
    let mut real: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shorter: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The index of the composer's said-it-already reply, when it is the only new thing here.\n\n    Asked only when no real candidate has anything new to say (see `_choose`). The composer owns\n    both halves of this - the reading of the line as a repeat and the wording - so the arbiter\n    only recognises the line and lets it through; it keeps no vocabulary of its own.\n\n    Only a repeat that has lost something counts. The whole answer said again still answers a\n    narrower question (\"what about tea\" is in the answer about water, word for word) and STYLE.md\n    would rather have it twice than a shrug; the opening of it, with the rest gone, does not.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "says_it_already", &[text.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    real = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&candidates.clone()) { let mut candidate: Py = __item.clone(); if py_truthy(&op_ge(&_priority(&[candidate.clone()]), &Py::Int(0))) {  __out.push(candidate.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&real.clone()))) {
    return Py::None;
    }
    first = _content(&[mcall(&geti(&real.clone(), &Py::Int(0)), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_echoes(&[first.clone(), last.clone()]))); if py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &op_sub(&call_builtin("len", &[mcall(&last.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) } }) {
    return Py::None;
    }
    subject = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[g__repeat(), py_str("FIGURE_ASKS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    figure = __item.clone();
    let __try2 = exc::catch(|| {
    let __unpk7 = py_unpack(&figure.clone(), 3, None);
    _name = __unpk7[0].clone();
    ask = __unpk7[1].clone();
    _says = __unpk7[2].clone();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    let __try8 = exc::catch(|| {
    if py_truthy(&mcall(&ask.clone(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    shorter = mcall(&ask.clone(), "sub", &[py_str(" "), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&op_lt(&call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    subject = shorter.clone();
    }
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled12 = true;
    continue;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    }
    if py_truthy(&Py::Bool(!py_is(&g__repeat(), &Py::None))) {
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), subject.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&call_builtin("getattr", &[g__repeat(), py_str("FIGURE_WORDS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&call_builtin("getattr", &[g__repeat(), py_str("FIGURE_SAYING"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&left.clone()))) {
    return Py::None;
    }
    }
    last_words = call_builtin("set", &[mcall(&last.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("compose.fallback", "content_words", &[subject.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&last_words.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&last_words.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&last_words.clone(), &op_add(&word.clone(), &py_str("s")))) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    said = call_builtin("set", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    missing = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&said.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &op_add(&word.clone(), &py_str("s")))) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&op_mul(&call_builtin("len", &[missing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::None;
    }
    let __iter13 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter13 {
    let __unpk14 = py_unpack(&__item.clone(), 2, None);
    index = __unpk14[0].clone();
    candidate = __unpk14[1].clone();
    if py_truthy(&{ let __b = op_lt(&_priority(&[candidate.clone()]), &Py::Int(0)); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "is_said_it_already", &[mcall(&candidate.clone(), "get", &[py_str("reply")])]) } }) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _ack_lines(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = Py::None;
    let mut module: Py = Py::None;
    let mut pools: Py = Py::None;
    let _ = py_str("The lines `chat` closes a conversation with, reduced the way a reply is compared.");
    if py_truthy(&Py::Bool(py_is(&g_ACK_POOL(), &Py::None))) {
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[py_str(".chat"), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
    pools = { let __b = call_builtin("getattr", &[module.clone(), py_str("REPLIES"), Py::None]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    s_ACK_POOL(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pools.clone(), "get", &[py_str("ack"), py_tuple(vec![])])) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
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
    s_ACK_POOL(call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    return g_ACK_POOL();
    Py::None
}
pub fn _asks_a_question(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let _ = py_str("True when the understanding engine read the line as a question, else False.\n\n    \"is that fair\" after the answer about a dog on its own all day is a question, and chat's ack\n    pool answered it \"Sounds good.\" while the card beside it said the sentence with the verdict in\n    it. The reading is the understanding engine's (`understood[\"kind\"]`), so the arbiter keeps no\n    words of its own for what a question is.\n    ");
    understood = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("intent")]), &py_str("question"))) } };
    Py::None
}
pub fn _asks_for_a_judgement(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut judgement: Py = Py::None;
    let _ = py_str("True when the line asks the bot to rule on the plan on the table, else False.\n\n    The reading is the composer's (`bot/compose/judgement.py`), so nothing here is a second word\n    list for \"is that a good idea\": the module that owns the pools decides what the ask is.\n    ");
    let __try1 = exc::catch(|| {
    judgement = pyrt::xrefs::xmodule("compose.judgement");
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&judgement.clone(), "asks_for_a_judgement", &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _hands_the_decision_back(args: &[Py]) -> Py {
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
    let mut module: Py = Py::None;
    let _ = py_str("True when the reply hands the choice back rather than answering the line.\n\n    `chat`'s pool for \"is that a good idea\" says the decision is the person's to weigh and asks\n    what matters to them (\"I cannot call that one for you. Tell me what matters most about getting\n    a cat...\"). It is a better reply than the one it replaced, which signed the person off - and it\n    is still not an answer to the line: it asks the question back. A take from under the floor that\n    says the one thing the bot can say about the thing (\"Yes, that is a reasonable way to do it -\n    getting a cat is worth doing. The one thing...\") is the answer, so a hand-back does not outrank\n    it. Measured on twenty chats of a judgement asked after news: seven turns went from that answer\n    to a hand-back (#6689), and every one of them read worse.\n\n    The reading is the module's own: `chat` publishes `hands_it_back`, cut from its pools, so the\n    arbiter keeps no wording of anybody's.\n    ");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[py_str(".chat"), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&module.clone(), "hands_it_back", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _the_reply_is_the_report(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidate"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut own: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("Is this candidate the composer's own reply to a report on the advice just given?\n\n    The reading is the composer's (`fallback.the_report_reply`), and it is the same call the\n    module proposing the reply makes, so the arbiter compares two replies rather than keeping a\n    second copy of the reading. It exists because a report on the advice is the composer's\n    subject, and the module that proposes its reply sits under the floor: \"i wiped it and now its\n    worse\" after the oven answer was answered by `chat`'s tired pool - the word \"wiped\" read as\n    tiredness - over the composer's \"Stop and leave it alone. Worse is the answer ...\", and\n    `_a_real_answer_besides` counted that pool line as an answer to the line.\n    ");
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    own = _content(&[pyrt::xrefs::xcall("compose.fallback", "the_report_reply", &[text.clone(), memory.clone()])]);
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
    return { let __b = call_builtin("bool", &[own.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&own.clone(), &reply.clone())) } };
    Py::None
}
pub fn _real_answers_besides(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The candidates of modules above the floor that said something to the line.\n\n    Being above the floor is not the same as answering: `chat` sits at 300 on every line and its\n    acknowledgement pool signs the conversation off (\"Alright then - good luck with it.\"). A hook\n    that gives way to that hands the person a goodbye, which is what the whole of this rule is\n    meant to stop. The pool is read from the module that owns it, so nothing here is a second copy\n    of somebody's wording, and a reply that only repeats the last answer is not an answer either.\n    ");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&candidates.clone());
    for __item in __iter1 {
    candidate = __item.clone();
    if py_truthy(&op_le(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &_content(&[last.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &py_str("chat"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_ack_lines(&[]), &reply.clone())) } }) {
    continue;
    }
    let _ = mcall(&said.clone(), "append", &[candidate.clone()]);
    }
    return said.clone();
    Py::None
}
pub fn _the_ask_after_the_facts(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("The ask at the end of a line that states something first, or \"\" when the line has none.\n\n    Only a clause set off by a comma or a full stop counts, and only when it opens on a question\n    word, so \"my sister is veggie and shes coming to dinner\" on its own is left entirely alone.\n    ");
    low = mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "rstrip", &[py_str(" .?!")]);
    cut = call_builtin("max", &[mcall(&low.clone(), "rfind", &[py_str(",")]), mcall(&low.clone(), "rfind", &[py_str(";")]), mcall(&low.clone(), "rfind", &[py_str(".")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_le(&cut.clone(), &Py::Int(0))) {
    return py_str("");
    }
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&g_WORD(), "findall", &[slice_or_raise(&low.clone(), &Py::None, &cut.clone(), &Py::None)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return py_str("");
    }
    ask = mcall(&slice_or_raise(&low.clone(), &op_add(&cut.clone(), &Py::Int(1)), &Py::None, &Py::None), "strip", &[py_str(" .?!")]);
    return if py_truthy(&mcall(&g_THE_ASK_OPENS(), "match", &[ask.clone()])) { ask.clone() } else { py_str("") };
    Py::None
}
pub fn _only_says_the_line_back(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut body: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a reply is made of nothing but the words the person has just used.\n\n    The store claims a line for the fact in it and says that fact back (\"Your sister is veggie.\"),\n    which is the answer to a line that only states it and no answer at all to the question after\n    it. A reply that adds no word of its own has read nothing new, so it cannot be what the person\n    asked for.\n    ");
    words = mcall(&g_WORD(), "findall", &[_content(&[reply.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::Bool(false);
    }
    said = call_builtin("set", &[mcall(&g_WORD(), "findall", &[mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    let __aug1 = op_bitor(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&g_THEIR_VOICE(), "get", &[word.clone(), word.clone()])); } py_set(__out) })() });
    said = __aug1;
    body = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ECHO_STOP(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&body.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&body.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_candidates_that_answer_the_ask(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut answering: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The pool without the reply that only says the person's own line back, or None to keep it.\n\n    None when the line asks nothing after its facts, when every candidate in the pool says\n    something of its own, or when the only reply left is a floor line - a line whose one answer is\n    the fact said back keeps it, because telling the person less is worse than telling them what\n    they already knew.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_the_ask_after_the_facts(&[text.clone()])))) {
    return Py::None;
    }
    answering = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_only_says_the_line_back(&[_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), text.clone()])))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answering.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[answering.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&answering.clone()) { let mut index: Py = __item.clone(); __out.push(op_ge(&_priority(&[geti(&candidates.clone(), &index.clone())]), &Py::Int(0))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return answering.clone();
    Py::None
}
pub fn _a_card_besides(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut candidate: Py = Py::None;
    let mut other: Py = Py::None;
    let _ = py_str("True when a module with a subject of its own proposed something beside this candidate.\n\n    A report on the advice can be the only reading of a line - \"i wiped it and now its worse\" is\n    not a question and names no subject to look up - and then the composer's reply is the answer,\n    not a pool line's. Where a card is beside it (\"done that\" after the emails answer still has\n    the next part of that answer to give), the card keeps the turn and this reading is not\n    consulted.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    other = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&other.clone(), &index.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_POOL_MODULES(), &mcall(&candidate.clone(), "get", &[py_str("module")])))) {
    continue;
    }
    if py_truthy(&_content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_real_answer_besides(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let _ = py_str("True when some module above the floor said something to the line rather than closing it.\n\n    Being above the floor is not the same as answering: `chat` sits at 300 on every line and its\n    acknowledgement pool signs the conversation off (\"Alright then - good luck with it.\"). A hook\n    that gives way to that hands the person a goodbye, which is what the whole of this rule is\n    meant to stop. The pool is read from the module that owns it, so nothing here is a second copy\n    of somebody's wording, and a reply that only repeats the last answer is not an answer either.\n    ");
    return call_builtin("bool", &[_real_answers_besides(&[candidates.clone(), last.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _only_a_hand_back_besides(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut found: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("True when the only answer above the floor hands the decision back, else False.\n\n    `_a_real_answer_besides` counts a hand-back as an answer, so that a hook from below the floor\n    cannot take the turn off it - the same reason a stock close is not counted: a hook that gives\n    way to a goodbye hands the person nothing. But a hand-back is not the answer to a judgement\n    either (see `_hands_the_decision_back`), and a take from under the floor that says the one\n    thing about the thing is a better reply than asking the question back. This is the test that\n    tells the two cases apart: everything above the floor here is a hand-back.\n    ");
    found = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&candidates.clone());
    for __item in __iter1 {
    candidate = __item.clone();
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &_content(&[last.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &py_str("chat"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_ack_lines(&[]), &reply.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_hands_the_decision_back(&[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::Bool(false);
    }
    found = Py::Bool(true);
    }
    return found.clone();
    Py::None
}
pub fn _names_what_they_said(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply says a word from what the person has told the bot, else False.\n\n    A take from under the floor may beat a hand-back, but only where it is about the same thing:\n    the composer's pool for \"is that a good idea\" after \"im thinking of getting a cat\" says the cat\n    back (\"getting a cat is worth doing\"), and the pool it reaches on another line said \"Honestly\n    I can't tell you what it's like there. What matters most to you about the trip?\" after \"im\n    thinking of going vegetarian\" - a sentence about a trip nobody mentioned, which only surfaced\n    once the hand-back stopped outranking it. What separates them is the subject, so the reply has\n    to share a word with the thread that is not one every sentence holds.\n    ");
    said = call_builtin("set", &[mcall(&g_A_SUBJECT_WORD(), "findall", &[_content(&[text.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "update", &[mcall(&g_A_SUBJECT_WORD(), "findall", &[_content(&[mcall(&turn.clone(), "get", &[py_str("user")])])])]);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(80))) {
    break;
    }
    }
    let __aug2 = op_sub(&said.clone(), &g_NOT_A_SUBJECT_WORD());
    said = __aug2;
    words = op_sub(&call_builtin("set", &[mcall(&g_A_SUBJECT_WORD(), "findall", &[_content(&[reply.clone()])])]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_SUBJECT_WORD());
    if py_truthy(&op_bitand(&said.clone(), &words.clone())) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); for __item in py_iter_or_raise(&words.clone()) { let mut other: Py = __item.clone(); __out.push(_one_word(&[one.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _one_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut ending: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut shorter: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("True when the two words are one word with an ending on, else False.\n\n    A word that only *opens* the other is not the same word: `card` opens `cardiac`, `pain` opens\n    `painting`, `care` opens `careers`, `read` opens `readable`, `star` opens `start`. Those were\n    taken for the same subject by the first form of this test, which let a cardiac rehab card\n    answer \"my bank card was declined\" and a book pick answer \"im going to read before bed\" - a\n    reply kept because it matched on a word, which is the fault this test exists to stop. So the\n    longer word is the whole of the shorter one plus one ending and nothing else, and an ending\n    that starts with a vowel is only taken on a stem that ends with a consonant (\"care\" is not the\n    stem of \"careers\"; that is \"career\" with a plural on it). A form this reads as a different\n    word - \"worry\"/\"worries\", \"tire\"/\"tiring\" - costs nothing: the test is only used to give a\n    reply credit for the line's subject, and a miss leaves the turn where it was.\n    ");
    let __unpk1 = py_unpack(&if py_truthy(&op_le(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![one.clone(), other.clone()]) } else { py_tuple(vec![other.clone(), one.clone()]) }, 2, None);
    shorter = __unpk1[0].clone();
    longer = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&shorter.clone(), &longer.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&op_lt(&call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::Bool(false);
    }
    tail = if py_truthy(&mcall(&longer.clone(), "startswith", &[shorter.clone()])) { slice_or_raise(&longer.clone(), &call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_WORD_ENDINGS(), &tail.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("aeiou"), &geti(&tail.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str("aeiou"), &geti(&shorter.clone(), &op_neg(&Py::Int(1))))) } }))) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&tail.clone(), &Py::None, &Py::Int(1), &Py::None), &geti(&shorter.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WORD_ENDINGS(), &slice_or_raise(&tail.clone(), &Py::Int(1), &Py::None, &Py::None))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&shorter.clone(), "endswith", &[py_str("e")]); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed")]));
    for __item in __iter2 {
    ending = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&longer.clone(), &op_add(&slice_or_raise(&shorter.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &ending.clone())))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_answer_handed_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut band: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut seen: Py = Py::None;
    let _ = py_str("The index of the store's hand-back when the line asks to hear that answer again, else None.\n\n    An ask to hear it again is answered by the answer, so no card that read a word of the ask as a\n    subject of its own may take the line first: \"run that by me again\" over a boiler answer was\n    taken by the knowledge entry that read the word \"run\" as exercise, and the person got a\n    heart-rate target for a run. The hand-back was in the list in every one of those turns and\n    lost the turn, because the card declared the subject as it comes and the store declared\n    nothing.\n\n    Only the store's own candidate, only when its reply is the answer already on the screen, and\n    never when the ask names a figure: there the number is lifted out of the answer and the card\n    that holds that number keeps the turn (`tests.test_a_repeat_ask_keeps_the_recall`).\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_repeat_to_hold(&[memory.clone(), text.clone()]))) } }) {
    return Py::None;
    }
    band = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk384 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk384[0].clone(); let mut candidate: Py = __unpk384[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MEMORY_MODULES(), &mcall(&candidate.clone(), "get", &[py_str("module")]))); if !py_truthy(&__b) { __b } else { op_ge(&_priority(&[candidate.clone()]), &Py::Int(0)) } }) {  __out.push(index.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&band.clone());
    for __item in __iter1 {
    index = __item.clone();
    reply = _content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { _echoes(&[reply.clone(), last.clone()]) } }) {
    return index.clone();
    }
    }
    seen = call_builtin("set", &[mcall(&_content(&[last.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&band.clone());
    for __item in __iter2 {
    index = __item.clone();
    reply = _content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { _made_of_the_answer(&[reply.clone(), seen.clone()]) } }) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _names_a_figure(args: &[Py]) -> Py {
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
    let mut _name: Py = Py::None;
    let mut _says: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("True when the line asks for a number, time or amount rather than to hear it all again.\n\n    Read with the store's own patterns (`repeat.FIGURE_ASKS`), as `the_figure_asked` reads the\n    same line, so a figure ask added there is read here too.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_MEASURE_ASKED(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[g__repeat(), py_str("FIGURE_ASKS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    figure = __item.clone();
    let __try2 = exc::catch(|| {
    let __unpk7 = py_unpack(&figure.clone(), 3, None);
    _name = __unpk7[0].clone();
    ask = __unpk7[1].clone();
    _says = __unpk7[2].clone();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    let __try8 = exc::catch(|| {
    if py_truthy(&mcall(&ask.clone(), "search", &[line.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled12 = true;
    continue;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _as_it_comes(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut hand_back: Py = Py::None;
    let mut index: Py = Py::None;
    let mut name: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut report: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut vetoed: Py = Py::None;
    let _ = py_str("The index of a candidate its module asks to be answered as it comes, else None.\n\n    Some answers are right whatever else proposed, but only on some of the lines their module\n    proposes on. A band cannot say that: it says \"everything this module proposes\" and the\n    module's other lines then take turns they should not. The memory store is the case that\n    found it: \"my partner is vegetarian\" then \"places to eat near the old town\" must be answered\n    from what the person said, and the same module's diet fact must not answer \"where can i take\n    my dog\" just because it was that high up the scale. So the module says which of its own\n    proposals this is, on the line in front of it, and the arbiter takes its word.\n\n    Read after the safety band, and before the never-repeat rule, the `ANSWERS_INSTEAD`\n    declaration and the specialist fit, because \"answer this as it comes\" is the whole of the\n    claim. A flagged reply that only repeats the bot's last turn is passed over - a module that\n    says \"this line is mine\" still has to say something new - and so is a hook that raises, so a\n    module cannot lose the turn by declaring badly.\n\n    A hook from below the floor does not take the turn off a module above it, though. A claim is\n    settled by PRIORITY everywhere else in this file (`_claims_instead`), and a module under the\n    floor has no higher claim on the line than anything that actually answered it: \"im worn out\n    and flat this week\" / \"and\" is answered from the store with the one thing that helps (\"That\n    kind of tired needs an early night, not more coffee\"), and a fallback-band hook that took the\n    turn said \"that's as far as I can take it\" over an answer nobody had been given yet. The same\n    held for every other band and it was not enforced: \"i keep putting off my emails\" /\n    \"done that\" gave the greeting's \"How is it going with it so far?\" over the next part of the\n    entry. Hooks above the floor are untouched: those exist to beat the store on the lines it\n    proposes on, which is the whole of the reason `answers` has one.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    name = mcall(&candidate.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(!py_truthy(&_declares_as_it_comes(&[name.clone(), text.clone(), memory.clone()])))) {
    continue;
    }
    thing = _the_thing_the_line_ruled_out(&[text.clone(), memory.clone()]);
    vetoed = { let __b = call_builtin("bool", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _every_real_answer_offers(&[thing.clone(), candidates.clone(), index.clone()]) } };
    if py_truthy(&{ let __b = op_lt(&_priority(&[candidate.clone()]), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = _a_real_answer_besides(&[candidates.clone(), last.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&vetoed.clone())) } } } }) {
    hand_back = { let __b = _only_a_hand_back_besides(&[candidates.clone(), last.clone()]); if !py_truthy(&__b) { __b } else { _names_what_they_said(&[mcall(&candidate.clone(), "get", &[py_str("reply")]), text.clone(), memory.clone()]) } };
    report = { let __b = _the_reply_is_the_report(&[candidate.clone(), text.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = pyrt::xrefs::xcall("compose.fallback", "a_report_that_says_something", &[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_card_besides(&[candidates.clone(), index.clone()]))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_declaring_specialist_besides(&[candidates.clone(), index.clone(), memory.clone(), text.clone()]))) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = hand_back.clone(); if py_truthy(&__b) { __b } else { report.clone() } }))) {
    continue;
    }
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&vetoed.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&_priority(&[candidate.clone()]), &g_CHATTER()); if !py_truthy(&__b) { __b } else { _a_declaring_specialist_besides(&[candidates.clone(), index.clone(), memory.clone(), text.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&_priority(&[candidate.clone()]), &g_CHATTER()); if !py_truthy(&__b) { __b } else { _a_card_names_the_line(&[candidates.clone(), index.clone(), text.clone(), memory.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(last.clone()); __kws.extend(vec![("announced".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _echoes(args))(&__args) }; if py_truthy(&__b) { __b } else { _the_answer_again_with_a_tail(&[reply.clone(), last.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = _a_fresh_answer_besides(&[candidates.clone(), index.clone(), last.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_repeat_to_hold(&[memory.clone(), text.clone()]))) } } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = _the_ask_after_the_facts(&[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _only_says_the_line_back(&[reply.clone(), text.clone()]); if !py_truthy(&__b) { __b } else { _a_card_besides(&[candidates.clone(), index.clone()]) } } } }) {
    continue;
    }
    return index.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_question_about_the_bot(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks about the bot itself, else False.\n\n    The reading is the composer's (`fallback.reads_as_about_itself`), the same one that makes the\n    claim: a question about the bot's own make, limits or handling of what it is told. It is read\n    here for the one thing the arbiter needs from it - that no entry about the world is a\n    specialist on such a line - and a reading that fails is not one.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[pyrt::xrefs::xcall("compose.fallback", "reads_as_about_itself", &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _a_declaring_specialist_besides(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut other: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut topics: Py = Py::None;
    let _ = py_str("True when another candidate's module declares a topic this line was read as.\n\n    A card that repeats the answer the person already has is not a specialist that read the line.\n    After \"my boiler is making a noise\" / \"i already bled the radiators\" the radiators entry\n    declared heating and read the bleeding instructions out a second time, over the store's own\n    line (\"Then that part is out of the way: check the pressure is 1 to 1.5 bar\"), and the person\n    was told to do what they had just said they did. The reading of \"still telling them the step\n    they just took\" is the store's (`already.tells_them_the_step_again`), so this file keeps no\n    vocabulary of its own.\n    ");
    topics = op_sub(&_line_topics(&[memory.clone()]), &g_NOT_A_SUBJECT());
    if py_truthy(&Py::Bool(!py_truthy(&topics.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { _a_question_about_the_bot(&[text.clone(), memory.clone()]) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    other = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&other.clone(), &index.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[_declared(&[mcall(&candidate.clone(), "get", &[py_str("module")])])]).unwrap_or_else(|e| exc::raise(e)), &topics.clone())))) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__already(), &Py::None)); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.already", "tells_them_the_step_again", &[reply.clone(), text.clone(), memory.clone()]) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_fresh_answer_besides(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut last: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut candidate: Py = Py::None;
    let mut other: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("True when a real candidate has an answer that is not the last one said again.");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    other = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &index.clone())); if py_truthy(&__b) { __b } else { op_lt(&_priority(&[candidate.clone()]), &Py::Int(0)) } }) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&reply.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(last.clone()); __kws.extend(vec![("announced".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _echoes(args))(&__args) })) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _claims_instead(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut above: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "above") { v.clone() }
    else { Py::None };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut name: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of a real candidate whose module says it answers a line another module claims.\n\n    \"how do i unblock my sink\" is answered with the plunger and the trap, and the line after it -\n    \"i tried that and it didn't work\" - is read by the composer as a report on that advice, so it\n    claims the turn as it comes. What the composer then says is about the attempt (\"say what you\n    did and that it hasn't shifted\"); the module that knows who does the job once a home fix has\n    run out (\"a plumber, with the plunger and the trap already ruled out\") has the answer, and the\n    claim takes the turn from it. That is a routing fault, and it cannot be fixed by looking for\n    the advice's words in the candidates' replies: word overlap let a wrong answer line through on\n    a phone-battery thread and turned \"my knee is sore when i run / its not helping\" into the\n    fallback's shrug.\n\n    So the claim is given up only to a module that says so itself: it declares\n    `CLAIMS_INSTEAD(text, memory) -> bool` and says yes only on the lines where its own proposal\n    is the answer to the claimed line, never to win turns. PRIORITY still settles the rest - a\n    claim from a module above the declaring one keeps the turn, so the memory store's reading of\n    \"places to eat near the old town\" is not displaced by a general answer - and the safety band\n    was read before this. A reply that only says the bot's last line again is not an answer, so\n    it does not displace anything.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&above.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_le(&_priority(&[candidate.clone()]), &above.clone()) } }) {
    continue;
    }
    name = mcall(&candidate.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(!py_truthy(&_declares_claims_instead(&[name.clone(), text.clone(), memory.clone()])))) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } } } }) {
    continue;
    }
    return index.clone();
    }
    return Py::None;
    Py::None
}
pub fn _declares_claims_instead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut hook: Py = Py::None;
    let mut module: Py = Py::None;
    let _ = py_str("True when the module says the claimed line is its own answer (README.md, \"Interface\").");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
    hook = call_builtin("getattr", &[module.clone(), g_DECLARED_CLAIMS_INSTEAD(), Py::None]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = call_builtin("callable", &[hook.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { py_call(&hook, &[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _declares_as_it_comes(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut hook: Py = Py::None;
    let mut module: Py = Py::None;
    let _ = py_str("True when the module says this line is one to answer as it comes (README.md, \"Interface\").");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
    hook = call_builtin("getattr", &[module.clone(), g_DECLARED_AS_IT_COMES(), Py::None]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = call_builtin("callable", &[hook.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { py_call(&hook, &[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _asks_about_the_chat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut hook: Py = Py::None;
    let mut module: Py = Py::None;
    let _ = py_str("True when the module says this line is about the conversation, not about the world.\n\n    The reading belongs to the module that has it (README.md, \"Interface\"): the memory store's\n    gate already knows every recap shape, so `_fit` asks it rather than growing a second word\n    list here. A module that cannot answer, or that raises, says nothing.\n    ");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
    hook = call_builtin("getattr", &[module.clone(), g_DECLARED_ABOUT_THE_CHAT(), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("callable", &[hook.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    hook = call_builtin("getattr", &[module.clone(), g_OWN_ABOUT_THE_CHAT(), Py::None]).unwrap_or_else(|e| exc::raise(e));
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = call_builtin("callable", &[hook.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { py_call(&hook, &[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _the_move_in_hand(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut real: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "real") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: real"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut _u: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut index: Py = Py::None;
    let mut length: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut of_the_answer: Py = Py::None;
    let mut re_tells: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut share: Py = Py::None;
    let mut sharing: Py = Py::None;
    let mut shortest: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The index of the step out of the answer just given, on a line that asks for it, else None.\n\n    \"where do i start\" after an answer names nothing to look up: it asks which part of the answer\n    in hand to do first. The composer reads that and hands the step back (\"The thing to do first:\n    Keep them short and filed.\"), and the entry that wrote the answer is asked by the same line and\n    writes the same answer again at full length, in its second wording - so the never-repeat rule\n    does not see a repeat either, and the specialist rule then gives the turn to the paragraph the\n    person has just read (\"i want to stop biting my nails\" / \"where do i start\" came back as the\n    nail-biting paragraph a second time).\n\n    What makes a reply a form of the answer is its words: nearly every one of them is a word the\n    answer used, or it runs to the answer's own length and is the answer again in other words. A\n    step is the shortest of them and one clause long; a paragraph is not. And the\n    paragraph has to be the answer again, not a part of it: a reply that runs to three quarters of\n    the answer's own length re-tells it, while an entry that has already taken a step out of the\n    answer (\"The first thing I would change: Ease off running ...\") says a third of it, and that\n    is answering the same way the composer does, so this rule leaves it alone. Nothing here reads\n    the topic, so a step that is genuinely about something else cannot be taken for the answer's\n    own (`names_nothing` and `MOVE_ASK` are the memory lane's readings, not a word list here).\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&g__repeat(), &Py::None)) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.repeat", "names_nothing", &[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("memory.repeat", "MOVE_ASK"), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&op_lt(&call_builtin("len", &[real.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    said = mcall(&last.clone(), "split", &[]);
    seen = call_builtin("set", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    forms = py_list(vec![]);
    whole = Py::Bool(false);
    let __iter6 = py_iter_or_raise(&real.clone());
    for __item in __iter6 {
    index = __item.clone();
    words = mcall(&_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    continue;
    }
    share = op_truediv(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&seen.clone(), &word.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    of_the_answer = op_ge(&share.clone(), &g_MOVE_SHARE());
    re_tells = { let __b = op_ge(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOVE_ANSWER_WORDS()); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_mul(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOVE_WHOLE())) } };
    if py_truthy(&re_tells.clone()) {
    whole = Py::Bool(true);
    }
    if py_truthy(&{ let __b = of_the_answer.clone(); if py_truthy(&__b) { __b } else { re_tells.clone() } }) {
    let _ = mcall(&forms.clone(), "append", &[py_tuple(vec![call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), index.clone(), of_the_answer.clone()])]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&whole.clone()))) {
    return Py::None;
    }
    sharing = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&geti(&form.clone(), &Py::Int(2))) {  __out.push(form.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&sharing.clone())) } }) {
    return Py::None;
    }
    let __unpk7 = py_unpack(&call_builtin("min", &[sharing.clone()]).unwrap_or_else(|e| exc::raise(e)), 3, None);
    shortest = __unpk7[0].clone();
    index = __unpk7[1].clone();
    _u = __unpk7[2].clone();
    longest = call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let __unpk9021 = py_unpack(&__item.clone(), 3, None); let mut length: Py = __unpk9021[0].clone(); let mut _u: Py = __unpk9021[1].clone(); let mut _u: Py = __unpk9021[2].clone(); __out.push(length.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_gt(&shortest.clone(), &g_MOVE_WORDS()); if py_truthy(&__b) { __b } else { op_gt(&op_mul(&shortest.clone(), &Py::Int(3)), &op_mul(&longest.clone(), &Py::Int(2))) } }) {
    return Py::None;
    }
    return index.clone();
    Py::None
}
pub fn _the_whole_move(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut real: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "real") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: real"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut written: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "written") { v.clone() }
    else { py_str("") };
    let mut _u: Py = Py::None;
    let mut _halves: Py = Py::None;
    let mut absent: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut index: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut quiet: Py = Py::None;
    let mut repeats: Py = Py::None;
    let mut run: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut tokens: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The index of the step that keeps the answer's sentence, where another hands over a piece.\n\n    \"where do i start\" after a how-to is answered by two modules at once: the composer, which\n    hands back the sentence the answer wrote its step in, and the entry that wrote the answer,\n    which builds \"The first thing to do: <the step>\" and stops the step at the answer's first\n    comma. Both are made of the answer's own words, so the never-repeat rule sees nothing, and\n    the specialist rule then gives the turn to the entry - half of an instruction, over the whole\n    of it. And the two are the same move: the short one's words are a run of the long one's.\n\n    So this decides one thing: where one candidate hands over a run of the answer that stops at\n    the answer's own comma, and another candidate carries that same run on past the comma, the\n    one that carries on takes the turn. Nothing else is read - no module name, no topic - and\n    where the answer's words run out the seed rule stands, as it does everywhere else.\n\n    A reply counts as a step only when it is made of the answer's own words (`MOVE_SHARE`, the\n    same test `_the_move_in_hand` makes) and is short enough to be one (`WHOLE_MOVE_WORDS`), so a\n    paragraph of the answer cannot be lifted over a step by this rule.\n\n    A reply that says the answer over again is not a step at all, however well its opening matches\n    (`_echoes`). \"Same as before: <the answer>\" is the entry's card for a bare follow-up, and the\n    longest of those reads as a whole sentence with the writer's own comma in it, so without this\n    guard the rule handed the turn to a repeat - the one thing STYLE.md forbids outright - because\n    it carried the answer past its comma. Handing a sentence back is only better than handing its\n    first clause over when the sentence is new.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&g__repeat(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[real.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.repeat", "names_nothing", &[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("memory.repeat", "MOVE_ASK"), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    answer = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = written.clone(); if py_truthy(&__b) { __b } else { last.clone() } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    tokens = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_TOKEN(), "finditer", &[mcall(&answer.clone(), "lower", &[])])) { let mut match_: Py = __item.clone(); __out.push(py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(0)]), mcall(&match_.clone(), "end", &[])])); } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::None;
    }
    seen = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens.clone()) { let __unpk5274 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk5274[0].clone(); let mut _u: Py = __unpk5274[1].clone(); __out.push(word.clone()); } py_set(__out) })() };
pub fn __nested__halves_5(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _u: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut _halves: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut absent: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut answer: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut candidates: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut cut: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut index: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut last: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut quiet: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut real: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut repeats: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut run: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut seen: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut tokens: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut whole: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut written: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut skip_repeats: Py = __pos.get(20).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "skip_repeats").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let _ = py_str("The candidates that stop at the answer's comma, and those that carry the run on.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    cut = __unpk1[0].clone();
    whole = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&real.clone());
    for __item in __iter2 {
    index = __item.clone();
    words = mcall(&_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_WHOLE_MOVE_WORDS()) } }) {
    continue;
    }
    if py_truthy(&{ let __b = skip_repeats.clone(); if !py_truthy(&__b) { __b } else { _echoes(&[call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), last.clone()]) } }) {
    continue;
    }
    if py_truthy(&op_lt(&op_truediv(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&seen.clone(), &word.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &g_MOVE_SHARE())) {
    continue;
    }
    run = _run_in_the_answer(&[words.clone(), tokens.clone(), answer.clone()]);
    if py_truthy(&{ let __b = run.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_CUT_MARKS(), &slice_or_raise(&answer.clone(), &geti(&run.clone(), &Py::Int(1)), &op_add(&geti(&run.clone(), &Py::Int(1)), &Py::Int(1)), &Py::None))); if !py_truthy(&__b) { __b } else { op_ge(&op_sub(&call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)), &geti(&run.clone(), &Py::Int(2))), &Py::Int(2)) } } } }) {
    let _ = mcall(&cut.clone(), "append", &[py_tuple(vec![words.clone(), geti(&run.clone(), &Py::Int(0))])]);
    } else {
    let _ = mcall(&whole.clone(), "append", &[words.clone()]);
    }
    }
    return py_tuple(vec![cut.clone(), whole.clone()]);
    Py::None
}
    _halves = {{ let _u = _u.clone(); let _halves = _halves.clone(); let absent = absent.clone(); let answer = answer.clone(); let candidates = candidates.clone(); let cut = cut.clone(); let index = index.clone(); let last = last.clone(); let match_ = match_.clone(); let quiet = quiet.clone(); let real = real.clone(); let repeats = repeats.clone(); let run = run.clone(); let seen = seen.clone(); let text = text.clone(); let tokens = tokens.clone(); let whole = whole.clone(); let word = word.clone(); let words = words.clone(); let written = written.clone(); py_func("_halves", move |args: &[Py]| { let mut full = vec![_u.clone(), _halves.clone(), absent.clone(), answer.clone(), candidates.clone(), cut.clone(), index.clone(), last.clone(), match_.clone(), quiet.clone(), real.clone(), repeats.clone(), run.clone(), seen.clone(), text.clone(), tokens.clone(), whole.clone(), word.clone(), words.clone(), written.clone()]; full.extend_from_slice(args); __nested__halves_5(&full) }) }};
    let __unpk7 = py_unpack(&py_call(&_halves, &[Py::Bool(true)]), 2, None);
    cut = __unpk7[0].clone();
    whole = __unpk7[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&cut.clone())); if !py_truthy(&__b) { __b } else { whole.clone() } }) {
    absent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_made_of_the_answer(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), seen.clone()])))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&absent.clone()) {
    let __iter8 = py_iter_or_raise(&real.clone());
    for __item in __iter8 {
    index = __item.clone();
    if py_truthy(&_made_of_the_answer(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), seen.clone()])) {
    return index.clone();
    }
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&cut.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&whole.clone())) } }) {
    repeats = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); if py_truthy(&{ let __b = _echoes(&[call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), last.clone()]); if !py_truthy(&__b) { __b } else { _made_of_the_answer(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), seen.clone()]) } }) {  __out.push(index.clone()); } } py_list(__out) })() };
    quiet = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&repeats.clone()) { let mut index: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_announces_a_repeat(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])])))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = quiet.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[quiet.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[repeats.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return geti(&quiet.clone(), &Py::Int(0));
    }
    return Py::None;
    }
    let __iter9 = py_iter_or_raise(&real.clone());
    for __item in __iter9 {
    index = __item.clone();
    words = mcall(&_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&whole.clone(), &words.clone())) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&cut.clone()) { let __unpk8361 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk8361[0].clone(); let mut run: Py = __unpk8361[1].clone(); __out.push(_carries_on(&[words.clone(), run.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _made_of_the_answer(args: &[Py]) -> Py {
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
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply is the answer's own words and next to nothing else, else False.");
    words = mcall(&_content(&[reply.clone()]), "split", &[]);
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_ge(&op_truediv(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&seen.clone(), &word.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &g_MOVE_SHARE()) } };
    Py::None
}
pub fn _the_line_is_news(args: &[Py]) -> Py {
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
    let mut returned: Py = Py::None;
    let _ = py_str("Whether the line reports the thing the last answer was about coming back.\n\n    Read by `_choose` to keep an answer announced as a repeat off a line that says the trouble has\n    returned. The reading is `bot/memory/returned.says_it_happened_again`, which is where \"the\n    thing came back\" already lives, so there is one definition of it rather than two.\n    ");
    let __try1 = exc::catch(|| {
    returned = pyrt::xrefs::xmodule("memory.returned");
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
    return mcall(&returned.clone(), "says_it_happened_again", &[text.clone()]);
    Py::None
}
pub fn _announces_a_repeat(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a reply opens on one of the lead-ins the other lanes give an answer again in.\n\n    Those lead-ins are the other lanes' vocabulary, read here the way this rule reads the memory\n    lane's leads elsewhere (`_a_lead_line`): \"Same as before: {reply}\" is `bot/memory/topics.py`'s\n    `REPEAT`, and \"As I said: {reply}\" is `bot/modules/answers.py`'s `AGAIN`. A reply under one of\n    them says it is giving the answer again rather than answering, and STYLE.md forbids saying the\n    same thing twice in a row - so where every candidate left is the answer read out again, the\n    one that does not announce itself as a repeat is the one to give.\n    ");
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_repeat_leads(&[])) { let mut lead: Py = __item.clone(); __out.push(mcall(&said.clone(), "startswith", &[lead.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _repeat_leads(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut _answers: Py = Py::None;
    let mut _topics: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut head: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("The lead-ins the other lanes put in front of an answer they are handing over again.");
    leads = py_list(vec![]);
    let __try1 = exc::catch(|| {
    _topics = pyrt::xrefs::xmodule("memory.topics");
    let __iter6 = py_iter_or_raise(&call_builtin("getattr", &[_topics.clone(), py_str("REPEAT_LEADS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    lead = __item.clone();
    let _ = mcall(&leads.clone(), "append", &[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lead.clone()) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])]);
    }
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try7 = exc::catch(|| {
    _answers = pyrt::xrefs::xmodule("modules.answers");
    let __iter12 = py_iter_or_raise(&call_builtin("getattr", &[_answers.clone(), py_str("AGAIN"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter12 {
    frame = __item.clone();
    head = mcall(&mcall(&geti(&mcall(&call_builtin("str", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[]), "lower", &[]);
    if py_truthy(&head.clone()) {
    let _ = mcall(&leads.clone(), "append", &[head.clone()]);
    }
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    return call_builtin("tuple", &[leads.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _carries_on(args: &[Py]) -> Py {
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
    let mut run: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "run") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: run"))) };
    let mut length: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("True when `run` is a stretch of `words` with more of the answer's step after it.");
    length = call_builtin("len", &[run.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &length.clone())]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&words.clone(), &start.clone(), &op_add(&start.clone(), &length.clone()), &Py::None), &run.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _run_in_the_answer(args: &[Py]) -> Py {
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
    let mut tokens: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tokens") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tokens"))) };
    let mut answer: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut length: Py = Py::None;
    let mut offset: Py = Py::None;
    let mut start: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The longest tail of these words that the answer says in one go: (words, end, index).");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("min", &[Py::Int(8), call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), Py::Int(2), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    length = __item.clone();
    tail = slice_or_raise(&words.clone(), &op_neg(&length.clone()), &Py::None, &Py::None);
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)), &length.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[length.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut offset: Py = __item.clone(); __out.push(Py::Bool(py_eq(&geti(&geti(&tokens.clone(), &op_add(&start.clone(), &offset.clone())), &Py::Int(0)), &geti(&tail.clone(), &offset.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![tail.clone(), geti(&geti(&tokens.clone(), &op_sub(&op_add(&start.clone(), &length.clone()), &Py::Int(1))), &Py::Int(1)), op_add(&start.clone(), &length.clone())]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_condolence(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether *reply* opens as comfort for a loss rather than as an answer about a thing.");
    said = mcall(&_content(&[reply.clone()]), "lower", &[]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__CONDOLENCE(), "match", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _says_the_subject(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether *reply* says a word of *line* that names something (four letters or more).");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&_content(&[reply.clone()]), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_WORD(), "findall", &[line.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__THE_NAMELESS(), &word.clone())) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&said.clone());
    for __item in __iter2 {
    other = __item.clone();
    if py_truthy(&{ let __b = mcall(&other.clone(), "startswith", &[word.clone()]); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[other.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_cause_asked_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut real: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "real") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: real"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut best: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "best") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: best"))) };
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The entry's answer to \"why is this thing failing\" beats the composer's condolence.\n\n    \"why are my fish dying\" was answered \"I'm so sorry to hear that. It's a hard thing to carry -\n    I'm here if you want to talk about it, or if anything practical would help\": the loss composer\n    read a pet going under as a bereavement, and the water entry's answer (\"A fish sitting on the\n    bottom, or hanging at the surface gulping, is reacting to the water rather than to you, and\n    ammonia is the usual cause ...\") was in the turn and lost, because it is an ordinary entry\n    under a module that reads the whole line. A condolence answers a loss somebody has had; a\n    question about why a living thing is failing asks for the cause, and the entry that names the\n    thing is the one that has it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_a_condolence(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")])])))) {
    return Py::None;
    }
    line = mcall(&_content(&[text.clone()]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THE_CAUSE(), "search", &[line.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_says_the_subject(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]), line.clone()])) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[mcall(&g_WORD(), "findall", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g__PEOPLE())) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&real.clone());
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &best.clone()))) {
    continue;
    }
    if py_truthy(&_says_the_subject(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), line.clone()])) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_date_fact(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut real: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "real") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: real"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut said: Py = Py::None;
    let mut store: Py = Py::None;
    let _ = py_str("The store's date is the answer to a \"when is it\" ask, and a card about the subject is not.\n\n    \"when do we fly\" after \"no in july\" was answered with the airport check-in card, and \"when is\n    my dentist appointment\" after \"im at the dentist on tuesday\" with \"ring practices near you and\n    ask whether they take new NHS patients\" - the subject matched and the date did not, so the\n    specialist rule handed the turn to the card. A line that asks when something is wants the fact\n    back, and the store is the only module that has it (see bot/compose/facts.py).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.facts", "asks_for_a_when", &[_content(&[text.clone()])])))) {
    return Py::None;
    }
    store = Py::None;
    let __iter1 = py_iter_or_raise(&real.clone());
    for __item in __iter1 {
    index = __item.clone();
    candidate = geti(&candidates.clone(), &index.clone());
    if py_truthy(&Py::Bool(!py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &py_str("memory_store")))) {
    continue;
    }
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &g_RECALL())) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.facts", "the_fact_asked_for", &[_content(&[text.clone()]), _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])])])) {
    store = index.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&store.clone(), &Py::None))) {
    return Py::None;
    }
    said = _content(&[mcall(&geti(&candidates.clone(), &store.clone()), "get", &[py_str("reply")])]);
    let __iter2 = py_iter_or_raise(&real.clone());
    for __item in __iter2 {
    index = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &store.clone()))) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.facts", "the_same_date", &[_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), said.clone()])) {
    return Py::None;
    }
    }
    return store.clone();
    Py::None
}
pub fn _the_next_part(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut last: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut leads: Py = Py::None;
    let _ = py_str("The index of the next part of the answer in hand, on a line that asks for more, else None.\n\n    \"what else\" after \"The thing to do first: Keep them short and filed\" asks for the part after\n    the step, and the entry that wrote the answer reads the line as the same question and hands\n    back the whole paragraph - the answer a third time, at the length it was the first time. The\n    memory lane walks the answer in order and hands back the part that comes next\n    (`bot/memory/repeat.py`: `NEXT_ASK`, `_another_part`), and this rule keeps the turn for it.\n\n    The line is picked out the way the step is: by the memory lane's own readings\n    (`asks_for_more`, `NEXT_LEADS`, `ALSO_LEADS` in bot/memory/repeat.py), so this rule keeps no\n    vocabulary of its own. Only the ask the memory lane reads as this one comes here, so nothing\n    else moves.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&g__repeat(), &Py::None)) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.repeat", "asks_for_more", &[text.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    leads = op_add(&call_builtin("tuple", &[call_builtin("getattr", &[g__repeat(), py_str("NEXT_LEADS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[call_builtin("getattr", &[g__repeat(), py_str("ALSO_LEADS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return _a_lead_line(&[candidates.clone(), leads.clone(), last.clone()]);
    Py::None
}
pub fn _a_lead_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut leads: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "leads") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: leads"))) };
    let mut last: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut candidate: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut openers: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of the memory lane's own short form of the answer in hand, else None.\n\n    Every line the memory lane hands back out of an answer opens with one of its leads (\"The\n    thing to do first:\", \"After that:\"), and the leads are that lane's constants, so recognising\n    the line costs the arbiter no vocabulary of its own. The lead alone does not pick between two\n    of them: an entry that takes a step out of its own answer says the same thing as the\n    composer's cut of it, and the one that says more is the better reply, so the longest wins.\n    The reply just given is no answer, and never answers itself.\n    ");
    openers = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&leads.clone()) { let mut lead: Py = __item.clone(); if py_truthy(&call_builtin("str", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&mcall(&geti(&mcall(&call_builtin("str", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[]), "lower", &[])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&openers.clone()))) {
    return Py::None;
    }
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    reply = mcall(&call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&mcall(&reply.clone(), "lower", &[]), "startswith", &[openers.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_content(&[reply.clone()]), &last.clone())) } }) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![call_builtin("len", &[mcall(&_content(&[reply.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), index.clone()])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let _ = mcall(&found.clone(), "sort", &[]);
    return geti(&geti(&found.clone(), &op_neg(&Py::Int(1))), &Py::Int(1));
    Py::None
}
pub fn _only_acknowledges(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let _ = py_str("True when the understanding engine reads the line as politeness and nothing else.\n\n    \"cheers\", \"thanks\", \"ta\", \"ok\", \"nice one\", \"makes sense\": the person has finished with\n    the last answer, not started a new question, so nothing it says can be about the thing the\n    conversation was on. A line that names a topic of its own is not this: \"thank you for the\n    spinach tip\" is read as thanks and about cooking, and keeps its routing unchanged.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_labels(&[mcall(&understood.clone(), "get", &[py_str("topics")])])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("thanks")))) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("intent")]), &py_str("ack"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("asks")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))) } } } };
    Py::None
}
pub fn _a_presence_check(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("True when the understanding engine reads the line as a check that the bot is here.\n\n    \"u up?\", \"you there?\", \"still about?\": the line asks whether the bot is listening, and it\n    names nothing of its own, so it inherits the topic of the conversation and a module that\n    answers that topic takes the turn. After \"how do i unblock my sink\" / \"it worked\", \"u up?\"\n    got \"Same as before: Start with a plunger...\" - the drain card read out a second time, where\n    the reply is \"Still here. What's up?\". A line that names a subject of its own is not this\n    (\"you there in that photo with the dog?\" is read as a question about a photo, not a check),\n    and neither is a thread the composer keeps (an emergency thread answers a \"you there?\" with\n    its own line - that is read before this). The reading is left alone, as it is for politeness\n    alone: only the topic rule is skipped, so the highest PRIORITY takes the turn.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_labels(&[mcall(&understood.clone(), "get", &[py_str("topics")])])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("presence")])))) {
    return Py::Bool(false);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("loss"), py_str("safety")]), &call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _about_the_bot(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut last: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of the bot's own answer for a line that asks how it works, else None.\n\n    \"how do you learn\" is a question about the bot, and the study-tips entry answered it with how\n    to revise for an exam because the line has \"learn\" in it; \"how do you know things\" got the\n    same paragraph. A topic entry answers the topic a line is about, and on a line about the bot\n    itself the only thing it can match is the verb, so the composer's own answer is the one that\n    answers the question asked (STYLE.md).\n\n    Only the kinds the composer publishes claim this (`MY_WORKINGS`), and only from the fallback\n    band: the bot's own answers for the rest (\"do you have feelings\") are already beaten by a\n    module that has a better one, and displacing a module that read the line whole is not this\n    rule's business. \"wrong\" is one of them because nothing has a better answer there: a chat that\n    was on tomatoes answered \"ok random question, do you ever make mistakes\" with a tomato sauce\n    recipe.\n    The reading is the composer's (bot/compose/fallback.py `about_the_bot`), not a word list of\n    the arbiter's.\n    ");
    let __try1 = exc::catch(|| {
    kind = pyrt::xrefs::xcall("compose.fallback", "about_the_bot", &[clean.clone()]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_contains(&_my_workings(&[]), &kind.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&candidates.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_declares_topics(&[mcall(&geti(&candidates.clone(), &Py::Int(0)), "get", &[py_str("module")])]))) } }) {
    return Py::None;
    }
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    candidate = __unpk7[1].clone();
    if py_truthy(&{ let __b = op_ge(&_priority(&[candidate.clone()]), &Py::Int(0)); if py_truthy(&__b) { __b } else { _declares_topics(&[mcall(&candidate.clone(), "get", &[py_str("module")])]) } }) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } } } }) {
    continue;
    }
    return index.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_venue_hours_turn(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut last: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of the composer's honest line for a venue's opening hours, else None.\n\n    \"what time does the post office on hill road close today\" was answered with the\n    weekend-plans card: that entry keys on \"today\" and a bare \"what\", and on a question about a\n    branch's hours both are noise. \"when does the post office close\" was answered with the\n    postage-times card, and \"when do the banks open today\" with how to open a bank account.\n    Nobody here can know a named venue's hours, so the composer's honest line - where to find\n    today's figure - is the one that answers the question asked (STYLE.md), and it takes the\n    turn off an entry that matched one other word of the line.\n\n    The reading is the composer's (bot/compose/fallback.py `venue_hours`): a place has to be\n    named AND opening or closing has to be the ask, so \"what time do hospitals let visitors in\",\n    \"what time does boarding start\" and \"what time is my appointment today\" are untouched. The\n    candidate has to BE one of the composer's hours lines, from the fallback band, and it never\n    repeats the last reply.\n    ");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "venue_hours", &[clean.clone(), Py::None])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&candidates.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_declares_topics(&[mcall(&geti(&candidates.clone(), &Py::Int(0)), "get", &[py_str("module")])]))) } }) {
    return Py::None;
    }
    let __try6 = exc::catch(|| {
    lines = call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("compose.fallback", "venue_hours_lines", &[])) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return Py::None;
    }
    let __iter11 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter11 {
    let __unpk12 = py_unpack(&__item.clone(), 2, None);
    index = __unpk12[0].clone();
    candidate = __unpk12[1].clone();
    if py_truthy(&{ let __b = op_ge(&_priority(&[candidate.clone()]), &Py::Int(0)); if py_truthy(&__b) { __b } else { _declares_topics(&[mcall(&candidate.clone(), "get", &[py_str("module")])]) } }) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&lines.clone(), &reply.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } })) } } } }) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_bots_own_taste(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut asked: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of the composer's taste for a line that asks what the bot likes, else None.\n\n    The same shape as `_about_the_bot`: only a line the composer reads as a question about its\n    own taste, and only where a card that declares topics is in front of it. The composer's line\n    is one of its taste pools by construction (`reads_as_a_taste` checks the pool), so this rule\n    never hands the turn to a shrug or a nod, and a line whose taste the composer has no reply\n    for keeps whatever answered it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&candidates.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_declares_topics(&[mcall(&geti(&candidates.clone(), &Py::Int(0)), "get", &[py_str("module")])]))) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    asked = pyrt::xrefs::xcall("compose.fallback", "reads_as_a_taste", &[clean.clone(), memory.clone()]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    let __try6 = exc::catch(|| {
    lines = call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = call_builtin("getattr", &[pyrt::xrefs::xmodule("compose.fallback"), py_str("TASTE_LINES"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::None;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return Py::None;
    }
    let __iter11 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter11 {
    let __unpk12 = py_unpack(&__item.clone(), 2, None);
    index = __unpk12[0].clone();
    candidate = __unpk12[1].clone();
    if py_truthy(&{ let __b = op_ge(&_priority(&[candidate.clone()]), &Py::Int(0)); if py_truthy(&__b) { __b } else { _declares_topics(&[mcall(&candidate.clone(), "get", &[py_str("module")])]) } }) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&lines.clone(), &reply.clone()))) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _my_workings(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let _ = py_str("The kinds the composer claims as questions about how it works, from the composer.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("frozenset", &[{ let __b = call_builtin("getattr", &[pyrt::xrefs::xmodule("compose.fallback"), py_str("MY_WORKINGS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _declares_topics(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut module: Py = Py::None;
    let _ = py_str("True when a module declares the topics it answers (README.md, \"Interface\").");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
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
    return call_builtin("bool", &[_labels(&[call_builtin("getattr", &[module.clone(), g_DECLARED_TOPICS(), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _reads_as_a_verdict(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut reader: Py = Py::None;
    let _ = py_str("The composer's reading of a line that is only a verdict on the news, or False.");
    reader = call_builtin("getattr", &[pyrt::xrefs::xmodule("compose.fallback"), py_str("reads_as_a_verdict"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&reader.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&reader, &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _verdict_lines(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The composer's own lines for a verdict on the news (bot/compose/fallback.py).");
    lines = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("VERDICT_ANNOY"), py_str("VERDICT_LUCK"), py_str("VERDICT_SHAME"), py_str("VERDICT_ROUGH"), py_str("VERDICT_HEAVY"), py_str("VERDICT_NUMB")]));
    for __item in __iter1 {
    name = __item.clone();
    let __try2 = exc::catch(|| {
    let _ = mcall(&lines.clone(), "update", &[call_builtin("getattr", &[pyrt::xrefs::xmodule("compose.fallback"), name.clone(), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    return call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_verdict_turn(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The index of the candidate whose reply IS the composer's verdict line, else None.\n\n    Matching the line rather than skipping the ones that are wrong is what makes the rule hold:\n    \"im being evicted next week\" then \"thats a nightmare\" was answered by the sleep entry\n    (\"Nightmares usually follow stress, late food, alcohol or a heavy film before bed\"), and\n    \"my landlord wont fix the boiler\" then \"what a headache\" by the headache entry, because any\n    card that happens to own the noun won the topic rule. A verdict has no subject of its own, so\n    the composer's line for it takes the turn and the cards that own the word stand down.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&lines.clone(), &_content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])])))) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _in_a_careful_thread(args: &[Py]) -> Py {
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
    let mut _target: Py = Py::None;
    let mut kind: Py = Py::None;
    let _ = py_str("True while the conversation is inside a live emergency or its next line.\n\n    A verdict after \"my husband hit me\" is not a light turn: the person is still in the thread\n    the bot answered, and the composer's continuation (\"You did the right thing telling someone.\n    Stay with a person you trust tonight...\") is the turn. `_verdict_pool` is a light pool, so it\n    must stand down for the whole thread - the same judgment `_thread_index` makes, read here so\n    the verdict rule cannot take the turn before that block is reached. The composer's own reader\n    for the thread is asked first (it knows the wording), and the arbiter's own question after it\n    (was the last word a safety answer) so a thread the reading misses is still not answered with\n    a light line.\n    ");
    let __unpk1 = py_unpack(&_composer_reading(&[clean.clone(), memory.clone()]), 2, None);
    kind = __unpk1[0].clone();
    _target = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![g_THREAD(), g_CARE_THREAD()]), &kind.clone()))) {
    return Py::Bool(true);
    }
    let __try2 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&call_builtin("getattr", &[pyrt::xrefs::xmodule("compose.fallback"), py_str("in_a_safety_thread"), { let _target = _target.clone(); let clean = clean.clone(); let kind = kind.clone(); let memory = memory.clone(); py_func("lambda", move |args: &[Py]| { let _target = _target.clone(); let clean = clean.clone(); let kind = kind.clone(); let memory = memory.clone(); let _memory: Py = __kwp(args, 0, "_memory").unwrap_or(Py::None); Py::Bool(false) }) }]).unwrap_or_else(|e| exc::raise(e)), &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    return Py::Bool(false);
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
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
    let mut ending: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of a line that could be what it is about, roughly singular.\n\n    A crude stem on purpose: \"landlords\" and \"landlord\" have to match for the test below to hold,\n    and the words a plural adds (\"-es\") are the same word to a reader.\n    ");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_SUBJECT_WORD(), &word.clone())) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("ies"), py_str("es"), py_str("s")]));
    for __item in __iter2 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) } }) {
    word = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), &if py_truthy(&Py::Bool(py_eq(&ending.clone(), &py_str("ies")))) { py_str("y") } else { py_str("") });
    break;
    }
    }
    let _ = mcall(&words.clone(), "add", &[word.clone()]);
    }
    return words.clone();
    Py::None
}
pub fn _about_the_news(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut news: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when a module's answer names something in the news the verdict is about.\n\n    The answer the pool must not take over is the one about the thing the person just told the\n    bot about, and it says so with the news's own words: chat's \"Landlords have to do most\n    repairs in a reasonable time\" is about the boiler, and the store's \"That kind of tired needs\n    an early night\" is about the tiredness. The answers the pool must take the turn from are about\n    the verdict's word instead: \"Nightmares usually follow stress\" for \"what a nightmare\" after a\n    question about washing, and a scarecrow joke for \"thats a joke\" after a broken laptop. Both of\n    those name nothing in the news, which is the reading here.\n    ");
    news = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    let __aug1 = op_bitor(&news.clone(), &_subject_words(&[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")])]));
    news = __aug1;
    let __aug2 = op_bitor(&news.clone(), &_subject_words(&[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")])]));
    news = __aug2;
    }
    return call_builtin("bool", &[op_bitand(&news.clone(), &_subject_words(&[reply.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_the_same_thing_again(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a candidate only re-tells the answer the bot has just given.\n\n    A verdict on the news asks to be heard, not answered again. After \"my washing machine is\n    broken\" the card's own second sentence came back as the answer to \"what a pain\" (\"Check the\n    cheap things before you pay anyone...\"), and after the landlord letter \"what a headache\" got\n    a second landlord paragraph. Both are the topic the last answer already covered, so the words\n    say it: when most of what the candidate says is already in the last answer, it is a re-telling\n    and not a second answer to the verdict (tidy-panther on #5479).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::Bool(false);
    }
    words = _subject_words(&[reply.clone()]);
    said = _subject_words(&[last.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::Bool(false);
    }
    shared = op_bitand(&words.clone(), &said.clone());
    return { let __b = op_ge(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { op_ge(&op_mul(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } };
    Py::None
}
pub fn _the_subject_was_answered(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidate"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the module that answered last is now saying about this subject again.\n\n    A verdict asks to be heard, not answered again. After the landlord letter, \"what a headache\"\n    came back with a second landlord paragraph (\"Landlords have to do most repairs in a reasonable\n    time, and anything that came with the flat counts\"), and that is the subject the last answer\n    had already covered - so the pool is the turn, not another sentence on the same topic.\n\n    Whose words they are is what separates the two (tidy-panther, cowbird-note, blue-hedgehog on\n    #5479). The rest of the letter answer is the same module saying the letter again - `answers`\n    both times, ten content words of the one before it. The card for a pet that died that opens\n    \"I'm sorry, that's one of the hardest days\" comes from the card the person has not been given\n    yet, after `loss` said its condolence, and the two paragraphs share the words every condolence\n    in English shares: \"sorry\", \"today\", \"gentle\", \"heavy\". So it is not a re-telling at all, and\n    a second answer to the same question is: the module that answered last, saying it again.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }))) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_subject_words(&[mcall(&last.clone(), "get", &[py_str("bot")])])) { let mut word: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {  __out.push(word.clone()); } } py_set(__out) })() };
    return op_ge(&call_builtin("len", &[op_bitand(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_subject_words(&[mcall(&candidate.clone(), "get", &[py_str("reply")])])) { let mut word: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {  __out.push(word.clone()); } } py_set(__out) })() })]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _the_verdict_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut is_a_card: Py = Py::None;
    let mut module_name: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of the module's own answer to the verdict, or None.\n\n    Not the candidate that happens to be ranked first: the answer to the news can be ranked\n    below a card that only owns a word of it, so the whole list is asked.\n\n    The opener test (`A_VERDICT_BACK`) alone kept out a card that owns a word of the verdict - but\n    it also threw away the answers that are about the news: the card for a pet that died opens\n    \"I'm sorry, that's one of the hardest days\", which is not a verdict taken up, and gray-cypress\n    found it standing down. So the answer is kept either way round: a line written for the verdict\n    (the opener) or a line about the news (`_about_the_news`). An answer that says the last one\n    again is neither - it is not a second answer - so the pool is free to take that turn.\n    ");
    let __iter1 = py_iter_or_raise(&op_add(&g_KEEPS_ITS_OWN_ANSWER(), &g_ANSWERS_THE_NEWS()));
    for __item in __iter1 {
    module_name = __item.clone();
    is_a_card = Py::Bool(py_contains(&g_ANSWERS_THE_NEWS(), &module_name.clone()));
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    candidate = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &module_name.clone()))) {
    continue;
    }
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&reply.clone(), &last.clone())); if py_truthy(&__b) { __b } else { _echoes(&[mcall(&candidate.clone(), "get", &[py_str("reply")]), last.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&_says_the_same_thing_again(&[mcall(&candidate.clone(), "get", &[py_str("reply")]), last.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&is_a_card.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_A_VERDICT_BACK(), "match", &[reply.clone()]) } }) {
    return index.clone();
    }
    if py_truthy(&_about_the_news(&[mcall(&candidate.clone(), "get", &[py_str("reply")]), memory.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&_the_subject_was_answered(&[candidate.clone(), memory.clone()])))) {
    return index.clone();
    }
    continue;
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _verdict_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut clean: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut best: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "best") { v.clone() }
    else { Py::Int(0) };
    let mut back: Py = Py::None;
    let _ = py_str("The index of the candidate a bare verdict on the news is answered with, else None.\n\n    \"what a hassle\" after an answer about cooking is the person saying how the news sits with\n    them. It has no subject of its own, so nothing may answer it with the topic the conversation\n    is on: the same-advice path matched \"cooking\" and answered \"Nothing separate for a hassle - it\n    is the same advice: <the cooking answer>\", which is the bot talking to itself, and any entry\n    that owns the verdict's noun answered the noun (\"Nightmares usually follow stress\").\n\n    Two turns keep the floor: a live emergency thread, where \"thats annoying\" is still that\n    thread, and a line a whole-line module read for itself (grief, loss) - after \"my mum died last\n    week\", \"thats a shame\" is not a light verdict, and the grief module's answer stands.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_reads_as_a_verdict(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&op_ge(&_priority(&[geti(&candidates.clone(), &best.clone())]), &g_WHOLE_LINE_BAND())) {
    return Py::None;
    }
    if py_truthy(&_in_a_careful_thread(&[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { text.clone() } }, memory.clone()])) {
    return Py::None;
    }
    back = _the_verdict_back(&[candidates.clone(), last.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&back.clone(), &Py::None))) {
    return back.clone();
    }
    return _the_verdict_turn(&[candidates.clone(), _verdict_lines(&[])]);
    Py::None
}
pub fn _instead(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut own: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { py_tuple(vec![]) };
    let mut clean: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut closer: Py = Py::None;
    let mut index: Py = Py::None;
    let mut keeps: Py = Py::None;
    let mut module: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("The index of a candidate whose module answers in place of the topic entries, else None.\n\n    A topic entry matches a word or a topic id of the line, so on a line another module read as\n    a whole it can answer a different question: \"some days i can't get out of bed since my mum\n    died\" matched the gift-for-mum entry on \"mum\" and \"get\", \"we had to put our cat down\" matched\n    litter trays on \"cat\", and \"what do i write in a condolence card\" matched a card shop. A\n    module that declares `ANSWERS_INSTEAD` read the whole line and says so, so it wins the turn;\n    it proposes only on the lines it is sure of, so it is not a priority to raise.\n\n    Every candidate is looked at, not only the ones above priority 0: a module that read the\n    whole line may sit in the fallback band on purpose - it proposes nowhere else, so a low\n    priority is the honest place for it - and a declaration that only worked from the real band\n    would make \"I answer in place of the entries\" a promise the arbiter then broke. Measured on\n    main with no fallback-band declarer: no reply changes, so this only widens the door.\n\n    Checked after the safety band, so a real emergency is still the safety module's turn, and\n    before the specialist rule, so a topic entry cannot take the line back on a word. It also\n    carries what the composer's death rule used to do on the turns after a death: the loss\n    module answers \"the anniversary is next week\" and \"i cant sleep\" while the thread is still\n    a bereavement, and passes a practical ask inside it (\"how do i cancel her phone contract\")\n    to the topic entry that can answer it.\n\n    A declaring module can also carry `DEFERS_TO`, a test on another candidate's reply, for the\n    turns where an entry answers the line better than it can: the test passing lets that entry\n    keep the turn (`_defers`). It is opt-in, so a module that declares nothing here is unchanged.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    module = mcall(&candidate.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(!py_truthy(&_declares_instead(&[module.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]), &last.clone())) } }) {
    continue;
    }
    keeps = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(module.clone()); __args.push(candidates.clone()); __kws.extend(vec![("skip".to_string(), index.clone()), ("last".to_string(), last.clone()), ("own".to_string(), own.clone()), ("clean".to_string(), clean.clone()), ("text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _defers(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&keeps.clone(), &Py::None))) {
    return keeps.clone();
    }
    subject = pyrt::xrefs::xcall("compose.aboutness", "after_a_change_of_subject", &[text.clone()]);
    if py_truthy(&subject.clone()) {
    closer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(candidates.clone()); __args.push(subject.clone()); __kws.extend(vec![("skip".to_string(), index.clone()), ("need".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.aboutness", "the_better_answer", args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&closer.clone(), &Py::None))) {
    continue;
    }
    }
    keeps = _the_entry_that_answers_the_ask(&[candidates.clone(), index.clone(), { let __b = clean.clone(); if py_truthy(&__b) { __b } else { text.clone() } }]);
    if py_truthy(&Py::Bool(!py_is(&keeps.clone(), &Py::None))) {
    return keeps.clone();
    }
    return index.clone();
    }
    return Py::None;
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut last: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the clause a two-part line puts its question in, else ().\n\n    The last clause only when it carries the ask: two or more words and one of four letters. A\n    line of one clause is not split at all, which is nearly every line, so the rule costs nothing\n    on \"my dad died in march\".\n    ");
    text = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_tuple(vec![]);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_ASK_CLAUSE_SPLIT(), "split", &[text.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_tuple(vec![]);
    }
    last = geti(&parts.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASKS_FOR_A_THING(), "search", &[mcall(&last.clone(), "lower", &[])])))) {
    return py_tuple(vec![]);
    }
    words = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&last.clone(), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_ENOUGH())) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_tuple(vec![]);
    }
    return words.clone();
    Py::None
}
pub fn _the_entry_that_answers_the_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: skip"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The index of a reply that answers the clause the line asks in, else None.\n\n    A module that declares `ANSWERS_INSTEAD` reads the whole line and answers the situation it\n    recognises, and on a two-part line that situation is often in the clause before the question.\n    \"a funeral on tuesday and i need something dark\" was met with a condolence, while the reply\n    that says what to wear sat in the list at 7000 and named \"dark\": the person asked what to put\n    on, not how sorry the week is. So the declaring module stands back when another reply names a\n    word of the ask clause and its own reply names none of it - the closest reply to the line\n    keeps the turn, which is the rule the arbiter applies everywhere else.\n\n    Nothing here reads a wording of the module's own: a condolence on a line that asks for\n    nothing (\"my dad died last month\") names the ask clause's words often enough to keep its turn,\n    and the camel's-back case for the declaring module - \"we had to put our cat down, can we bury\n    him at home\" - is answered by the pets entry, which is the outcome `DEFERS_TO` already had.\n    ");
    words = _the_ask_clause(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &skip.clone())); if py_truthy(&__b) { __b } else { _declares_instead(&[mcall(&candidate.clone(), "get", &[py_str("module")])]) } }) {
    continue;
    }
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    reply = mcall(&candidate.clone(), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_the_line(&[reply.clone(), words.clone()]))) } }) {
    continue;
    }
    if py_truthy(&_about_the_line(&[mcall(&geti(&candidates.clone(), &skip.clone()), "get", &[py_str("reply")]), words.clone()])) {
    return Py::None;
    }
    return index.clone();
    }
    return Py::None;
    Py::None
}
pub fn _defers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut skip: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: skip"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut own: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { py_tuple(vec![]) };
    let mut clean: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut text: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut test: Py = Py::None;
    let _ = py_str("The index of a candidate whose reply already does what the declaring module would say.\n\n    A module can read the whole line and still meet an entry that answers it better, because\n    the entry knows the topic and the module only recognises the situation: \"we had to put our\n    cat down yesterday\" is the pets entry's line (cremation, home burial, a paw print), and the\n    module's sympathy is the poorer answer. The module declares `DEFERS_TO`, a test on a reply,\n    so the arbiter still reads no wording of its own: a reply that passes it keeps the turn.\n\n    The test is called with the reply and the line (`test(reply, text)`), because the same reply\n    can be right for one line and wrong for the next: the pets entry answers \"we had to put our\n    cat down yesterday\" and is the wrong answer to \"my cat died and i keep crying\". A test\n    written for the reply alone (`test(reply)`) is called with the reply only.\n    ");
    test = _defers_to(&[module_name.clone()]);
    if py_truthy(&Py::Bool(py_is(&test.clone(), &Py::None))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &skip.clone())); if py_truthy(&__b) { __b } else { _declares_instead(&[mcall(&candidate.clone(), "get", &[py_str("module")])]) } }) {
    continue;
    }
    if py_truthy(&op_lt(&call_builtin("int", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("priority")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&{ let __b = own.clone(); if !py_truthy(&__b) { __b } else { op_le(&_match(&[mcall(&candidate.clone(), "get", &[py_str("module")]), own.clone(), py_tuple(vec![]), py_tuple(vec![]), clean.clone()]), &Py::Int(0)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]), &last.clone())) } }) {
    continue;
    }
    reply = { let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } };
    let __try3 = exc::catch(|| {
    if py_truthy(&py_call(&test, &[reply.clone(), text.clone()])) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["TypeError"]) {
                __handled7 = true;
    let __try8 = exc::catch(|| {
    if py_truthy(&py_call(&test, &[reply.clone()])) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["Exception"]) {
                __handled12 = true;
    continue;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
            }
            if !__handled7 && exc::matches_any(&__e, &["Exception"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    }
    return Py::None;
    Py::None
}
pub fn _defers_to(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut module: Py = Py::None;
    let mut test: Py = Py::None;
    let _ = py_str("The declaring module's `DEFERS_TO` test on a reply, or None when it declares none.");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    test = call_builtin("getattr", &[module.clone(), g_DECLARED_DEFERS(), Py::None]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&call_builtin("callable", &[test.clone()]).unwrap_or_else(|e| exc::raise(e))) { test.clone() } else { Py::None };
    Py::None
}
pub fn _declares_instead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut module: Py = Py::None;
    let _ = py_str("True when a module says it answers in place of the topic entries (README.md, \"Interface\").");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
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
    return call_builtin("bool", &[_labels(&[call_builtin("getattr", &[module.clone(), g_DECLARED_INSTEAD(), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _pause_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let _ = py_str("The index of the composer's line for an apology about the chat pausing, or None.\n\n    \"sorry for not replying\" is an apology for the gap and nothing else, and a topic entry can\n    still take it on a word of it: the answers module read \"reply\" and \"message\" as a request\n    to draft an email, so a person who had just said sorry for being away was told to \"say the\n    ask in the first line\". The reading and the wording are the composer's\n    (`fallback.reads_as_pause`, `fallback.CAME_BACK`); the arbiter only keeps the turn where it\n    belongs, the way it does for a goodbye.\n\n    Not at the expense of a module that read the whole line itself: the loss and grief bands sit\n    above `WHOLE_LINE_BAND` and their turn stands, and a safety thread was answered before this\n    (`_composer_reading` returns THREAD for it, so there is no pause to keep).\n    ");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_eq(&geti(&_composer_reading(&[text.clone(), memory.clone()]), &Py::Int(0)), &g_PAUSE()))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("compose.fallback", "CAME_BACK")) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_set(__out) })() };
    let __aug6 = op_bitor(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("compose.fallback", "RETURNED_TO_IT")) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_set(__out) })() });
    said = __aug6;
    named = pyrt::xrefs::xcall("compose.fallback", "reads_as_a_return", &[text.clone()]);
    if py_truthy(&named.clone()) {
    let __aug7 = op_bitor(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("compose.fallback", "RETURNED_TO_A_THING")) { let mut line: Py = __item.clone(); __out.push(_content(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), named.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }])); } py_set(__out) })() });
    said = __aug7;
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    kept = Py::None;
    let __iter8 = py_iter_or_raise(&pool.clone());
    for __item in __iter8 {
    index = __item.clone();
    if py_truthy(&op_ge(&_priority(&[geti(&candidates.clone(), &index.clone())]), &g_WHOLE_LINE_BAND())) {
    return Py::None;
    }
    }
    let __iter9 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter9 {
    let __unpk10 = py_unpack(&__item.clone(), 2, None);
    index = __unpk10[0].clone();
    candidate = __unpk10[1].clone();
    said_back = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&Py::Bool(!py_contains(&said.clone(), &said_back.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&said_back.clone(), &last.clone())) } }) {
    return index.clone();
    }
    kept = if py_truthy(&Py::Bool(py_is(&kept.clone(), &Py::None))) { index.clone() } else { kept.clone() };
    }
    return kept.clone();
    Py::None
}
pub fn _leaving_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut own: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own"))) };
    let mut carried: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: carried"))) };
    let mut words: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut text: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut reasoned: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut small_talk: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut typed: Py = Py::None;
    let _ = py_str("The index of the sign-off for a line that says the person is going, or None.\n\n    A goodbye is the one line no topic entry owns, and every topic entry can win it on a word of\n    the reason behind it: \"ill let you go, dinner is burning\" matched cooking, \"better go and\n    get the washing in\" got the search shrug, \"got to go, the kids are up\" got a nod. The\n    reading and the wording are the composer's (bot/compose/leaving.py); the arbiter only keeps\n    the turn where it belongs. A leave inside a safety thread is still that thread, so the\n    careful continuation keeps it.\n    ");
    typed = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.leaving", "reads_as_leaving", &[typed.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("compose.leaving", "lines", &[])) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_set(__out) })() };
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    if py_truthy(&Py::Bool(py_eq(&geti(&_composer_reading(&[text.clone(), memory.clone()]), &Py::Int(0)), &g_THREAD()))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::None;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    small_talk = _labels(&[py_tuple(vec![py_str("small-talk")])]);
    let __try11 = exc::catch(|| {
    reasoned = Py::Bool(py_is(&pyrt::xrefs::xcall("compose.leaving", "pool", &[typed.clone()]), &pyrt::xrefs::xget("compose.leaving", "REASON_POOL")));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret12: Py = Py::Bool(false);
    let mut __val13: Py = Py::None;
    let mut __unhandled14: Option<exc::PyExc> = None;
    match __try11 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret12 = __p[0].clone(); __val13 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled15 = false;
            if !__handled15 && exc::matches_any(&__e, &["Exception"]) {
                __handled15 = true;
    reasoned = Py::Bool(false);
            }
            if !__handled15 { __unhandled14 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled14 { exc::raise(__e); }
    if py_truthy(&__ret12) { return __val13; }
    let __iter16 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter16 {
    let __unpk17 = py_unpack(&__item.clone(), 2, None);
    index = __unpk17[0].clone();
    candidate = __unpk17[1].clone();
    if py_truthy(&{ let __b = op_bitand(&_declared(&[mcall(&candidate.clone(), "get", &[py_str("module")])]), &small_talk.clone()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_asks(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&reasoned.clone())); if py_truthy(&__b) { __b } else { _a_sign_off(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]) } } } } } }) {
    return index.clone();
    }
    }
    spare = Py::None;
    let __iter18 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter18 {
    let __unpk19 = py_unpack(&__item.clone(), 2, None);
    index = __unpk19[0].clone();
    candidate = __unpk19[1].clone();
    said_back = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &said_back.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&said_back.clone(), &last.clone())) } }) {
    return index.clone();
    }
    spare = if py_truthy(&Py::Bool(py_is(&spare.clone(), &Py::None))) { index.clone() } else { spare.clone() };
    }
    }
    if py_truthy(&Py::Bool(!py_is(&spare.clone(), &Py::None))) {
    return spare.clone();
    }
    let __iter20 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter20 {
    let __unpk21 = py_unpack(&__item.clone(), 2, None);
    index = __unpk21[0].clone();
    candidate = __unpk21[1].clone();
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&op_le(&_match(&[mcall(&candidate.clone(), "get", &[py_str("module")]), own.clone(), carried.clone(), words.clone(), clean.clone()]), &Py::Int(0))) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _asks(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply is a question, so it is not a sign-off for a line that says goodbye.");
    return Py::Bool(py_contains(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?")));
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
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let _ = py_str("True when a reply lets the person go (bot/compose/leaving.py owns the vocabulary).");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), pyrt::xrefs::xcall("compose.leaving", "reads_as_a_sign_off", &[reply.clone()])]);
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
pub fn _declared(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut module: Py = Py::None;
    let _ = py_str("The topics a module declares (README.md, \"Interface\"), or () when it declares none.");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
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
    return py_tuple(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return _labels(&[call_builtin("getattr", &[module.clone(), g_DECLARED_TOPICS(), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _the_store_revises_the_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut last: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut dislikes: Py = Py::None;
    let mut held: Py = Py::None;
    let mut index: Py = Py::None;
    let mut others: Py = Py::None;
    let mut person: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut run: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The store's reply on the turn the person says one thing in it is out, else None.\n\n    A suggestion is often a list, and the line after it takes one thing off the list: after \"what\n    can i add to chicken salad\" the reply names celery, red onion, apple and walnuts, and \"and i\n    hate celery\" is answered by giving the list back without the celery. Only the store can do\n    that. It read the dislike, and its reply is the answer the person already has with the food\n    they will not eat taken out.\n\n    The knowledge base cannot know it: the line has the word \"celery\" in it, so the five-a-day\n    entry answers \"a portion is a handful - about 80g\" and the person who said what they do not\n    eat is told about fruit and vegetables instead. The store's reply scores nothing on the\n    conversation's topic and the entry scores on it, so the last rule in `_choose` has to pick\n    the entry unless this one takes the turn first.\n\n    Narrow on purpose, and each gate is a test:\n\n    - the line has to say the thing the store holds as their dislike (the store's own reading, not\n      a second reader here), so a line about something else never qualifies;\n    - the store has to have proposed on this line, which it only does when it read the line as the\n      revision;\n    - its reply has to carry five words in a row of the last answer, which is what makes it the\n      answer in hand rather than a new one, and keeps a store reply that shares nothing with the\n      answer (`\"Celery - got it, that is out of anything I suggest\"`) under the ordinary rules.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    held = py_list(vec![mcall(&memory.clone(), "get", &[py_str("user")])]);
    others = mcall(&memory.clone(), "get", &[py_str("others")]);
    if py_truthy(&call_builtin("isinstance", &[others.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&held.clone(), "extend", &[others.clone()]);
    }
    dislikes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut person: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[person.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  for __item in py_iter_or_raise(&{ let __b = mcall(&person.clone(), "get", &[py_str("dislikes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut thing: Py = __item.clone(); if py_truthy(&thing.clone()) {  __out.push(mcall(&call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&dislikes.clone()))) {
    return Py::None;
    }
    said = call_builtin("set", &[mcall(&_content(&[text.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&said.clone(), &call_builtin("set", &[dislikes.clone()]).unwrap_or_else(|e| exc::raise(e)))))) {
    return Py::None;
    }
    previous = mcall(&last.clone(), "split", &[]);
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("module")]), &py_str("memory_store")))) {
    continue;
    }
    reply = { let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&_a_clause_was_cut_in_half(&[reply.clone()])) {
    continue;
    }
    words = mcall(&_content(&[reply.clone()]), "split", &[]);
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), call_builtin("max", &[Py::Int(0), op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    run = slice_or_raise(&words.clone(), &start.clone(), &op_add(&start.clone(), &Py::Int(5)), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[run.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))); if !py_truthy(&__b) { __b } else { _a_run_of(&[previous.clone(), run.clone()]) } }) {
    return index.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_clause_was_cut_in_half(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a reply splices a describing clause onto a list, so it reads as two halves.\n\n    The store builds a revision by keeping the list of things from the answer and dropping the\n    one that is out. Where the answer put a clause beside the list (\"a chickpea and vegetable\n    traybake, made in the largest dish you own, which feeds eight from one oven\") the same\n    keeping word by word gives \"a chickpea and vegetable traybake, made in the largest dish you\n    own or which feeds eight from one oven all still work\" - the list and the clause stitched at\n    the wrong place. That is a fault in the store's own wording, and until it builds the list\n    without the clause the ordinary answer is the one to give, so the relative pronoun says the\n    revision was cut in half.\n    ");
    words = mcall(&_content(&[reply.clone()]), "split", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("which"), py_str("whose")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _store_correcting(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut last: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut fixed: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The store's reply on the turn the person corrects what they just said, else None.\n\n    A correction is a line that only replaces something said a moment ago (\"my budget is 500\"\n    then \"make it 900\"). The understanding engine finds no topic in it, but a knowledge entry\n    keyed on its words can: \"make it 900\" reached the budget-split entry, so the person got a\n    lecture about splitting their income instead of \"900 - got it\". Only the store knows the line\n    is a correction of the turn before (the shared `memory[\"correction\"]` says which line it\n    was), and no entry can, so on that turn the store's reply is the one they are owed.\n\n    Nothing here reads a reply's wording: the store has to have proposed on this line (its own\n    reader decides that) and the never-repeat rule still holds.\n    ");
    fixed = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("correction")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[fixed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&fixed.clone(), "get", &[py_str("value")]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("said")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("module")]), &py_str("memory_store")))) {
    continue;
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_content(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])]), &last.clone())) } }) {
    return Py::None;
    }
    return index.clone();
    }
    Py::None
}
pub fn _draft_index(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut other: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The composer's promised draft, or its offer to write it, or None.\n\n    A line asking for help with a message is answered, honestly, by asking for the two things a\n    message needs: who it is for and what it should say. The next line is that detail (\"saying im\n    off sick tomorrow\"), and on main it was echoed back or met with a topic entry that took it on\n    a word (\"my car broke down\" got the hard-shoulder advice instead of the note the person was\n    told to hand over). An offer the bot then drops is worse than never offering.\n\n    This is the one place a composer reply is chosen ahead of every other candidate, before\n    `_instead`: it is safe only because the composer drafts on its own gates (the offer in the\n    bot's last turn, and a line that is not news, a symptom, a mood or a fragment). Any widening\n    of the draft has to keep those gates, or this turns routing into a priority raise.\n\n    The composer already writes the draft, so the routing decision is only to give the turn to\n    the module the composer answers through - `fallback.FALLBACK_MODULE`, the one module whose\n    whole reply is the composer's - when that reply is the drafted message. The module is matched\n    rather than the text, because the composer draws its pool twice (see `_thread_index`), and the\n    draft is recognised by its own lead-in, so a composer reply that is a statement or a\n    continuation never claims the turn this way.\n\n    The offer half is read the same way and only on a line that asks for the words: \"can you help\n    me write a text to my landlord\" got the tenancy entry, which answers about housing and not\n    about the message, and its example text was about a rota for a different person entirely. A\n    card can be right about the topic and still be the wrong answer to \"write me the words\", so\n    the offer takes the turn and the next line is the message. Both readings live in\n    bot/compose/draft.py, and the composer only offers when it has nothing to draft from.\n\n    What the offer may not displace is another answer that hands the words over: the writing card\n    answers the same line with the shape to aim at and \"tell me who it is for and what you need\n    and I will write yours\", and a card can answer it with the note itself in quotation marks.\n    Both are the offer plus something, so the turn is only taken when no other candidate promises\n    to write the message or already hands one over, and the cards keep the lines they were\n    written for.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &pyrt::xrefs::xget("compose.fallback", "FALLBACK_MODULE")))) {
    continue;
    }
    reply = mcall(&candidate.clone(), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_content(&[reply.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_content(&[reply.clone()]), &last.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "draft"), "names_a_draft", &[reply.clone()])) {
    return index.clone();
    }
    if py_truthy(&{ let __b = clean.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "draft"), "offers_a_draft", &[reply.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "draft"), "reads_as_a_writing_ask", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&candidates.clone()) { let mut other: Py = __item.clone(); if py_truthy(&Py::Bool(!py_is(&other.clone(), &candidate.clone()))) {  __out.push({ let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "draft"), "offers_a_draft", &[mcall(&other.clone(), "get", &[py_str("reply")])]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "draft"), "hands_over_words", &[mcall(&other.clone(), "get", &[py_str("reply")])]); if py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("compose.fallback", "draft"), "hands_over_the_shape", &[mcall(&other.clone(), "get", &[py_str("reply")])]) } } } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } } } }) {
    return index.clone();
    }
    return Py::None;
    }
    return Py::None;
    Py::None
}
pub fn _a_leaving_said_plain(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut best: Py = Py::None;
    let mut index: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The index of the composer's reply to a leaving said as the news it is, or None.\n\n    \"my husband has left me\" is the news, not a question about money, and the entry for a\n    divorce answered it with the first job of a settlement: \"Get the money apart quietly and\n    early: open a current account in your own name\". A plan for the day is not an answer to\n    having just said your life has come apart - STYLE.md's worst reply is the one that reads as\n    not having heard - so when the reply in front of this line hands over a job, the composer's\n    line that hears it takes the turn.\n\n    Only the line that says nothing else. An ask is answered by the entry (\"my husband has left\n    me, where do i stand on the house\"), and so is a reply that already hears it (the entry for\n    the same news says \"Nothing you do this week makes it hurt less, so do not make decisions in\n    it\", which is not a job). The reading is bot/compose/badnews.py's and the wording is\n    `fallback.LEFT_POOL`'s, so this is only the routing decision.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.badnews", "reads_as_the_leaving", &[clean.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.badnews", "reads_as_a_leaving_already_gone", &[clean.clone()]) } }))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    understood = { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("question"), py_str("request")]), &mcall(&understood.clone(), "get", &[py_str("intent")]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("help_ask")]); if py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("asks_a_verdict")]) } } } } } }) {
    return Py::None;
    }
    index = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(candidates.clone()); __args.push(pool.clone()); __kws.extend(vec![("pools".to_string(), py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "LEFT_POOL"), pyrt::xrefs::xget("compose.fallback", "LEFT_POOL_GONE")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_loss_reply_in(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&index.clone(), &Py::None))) {
    return Py::None;
    }
    best = if py_truthy(&pool.clone()) { geti(&pool.clone(), &Py::Int(0)) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_on_a_job(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")])]))) } }) {
    return Py::None;
    }
    return index.clone();
    Py::None
}
pub fn _sudden_loss_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut pools: Py = Py::None;
    let _ = py_str("The index of the composer's reply to a sudden loss, or None.\n\n    A pregnancy that has ended and a suicide among the people a person knows are read here\n    rather than only in the gentle pool, because the pool's words (\"that's frustrating, want a\n    hand working out the next step\") are for a setback, and because a module with a general\n    bad-news line must not answer a bereavement as one. Bleeding in pregnancy comes with them:\n    it is a midwife today, not a chat.\n\n    The reading lives in bot/compose/badnews.py and the wording in bot/compose/fallback.py, so\n    this is only the routing decision: when the composer has a line for this, it gets the turn.\n\n    One ask is not a condolence: \"my husband died in march and i dont know how to tell our\n    daughter\" wants the words to say to the child, and the knowledge base has them\n    (`explaining-death-to-a-child`). The composer's own reading says when the ask is that\n    (`fallback.asks_what_to_tell_a_child`), and on the day itself it says no, so a fresh death\n    keeps the condolence whoever else proposed.\n    ");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.badnews", "reads_as_sudden_loss", &[clean.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.badnews", "reads_as_pregnancy_bleeding", &[clean.clone()]) } }))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "asks_what_to_tell_a_child", &[clean.clone()]); if !py_truthy(&__b) { __b } else { _a_knowledge_entry_has_it(&[candidates.clone()]) } }) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    pools = py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "SUDDEN_LOSS"), pyrt::xrefs::xget("compose.fallback", "PREGNANCY_BLEEDING"), pyrt::xrefs::xget("compose.fallback", "THE_FIRST_HOUR_AND_A_CHILD")]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return _is_loss_reply_in(&[candidates.clone(), pool.clone(), pools.clone()]);
    Py::None
}
pub fn _the_words_for_the_child(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut childwords: Py = Py::None;
    let mut index: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of the composer's words for a child, after a condolence.\n\n    `_the_ask_about_a_child` above is the same ask in one line with the death, and it hands the\n    turn to the entry. A turn later there is no death in the line at all - just \"how do i tell a\n    4 year old\" - so the entry reads it as a general how-to and the card on screen time takes it,\n    or the condolence's own drafter answers with a message for the bereaved. The composer reads\n    the thread (`childwords.asks_the_words_for_a_child`) and has the words; this is only the\n    routing decision, and it stands aside the moment that reading says no.\n    ");
    childwords = pyrt::xrefs::xmodule("compose.childwords");
    let __try1 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&childwords.clone(), "asks_the_words_for_a_child", &[clean.clone(), memory.clone()]))) } }) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    candidate = __unpk7[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &pyrt::xrefs::xget("compose.fallback", "FALLBACK_MODULE")))) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&_the_words_for_a_child_pool(&[]), &reply.clone()))) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_whole_line_module_spoke(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut candidate: Py = Py::None;
    let _ = py_str("True when a module that reads a whole line (grief, loss) has an answer of its own.\n\n    Those modules answer the same follow-up better than a pool can: grief's `TELLING_REPLIES`\n    has the name and the day (\"\"your wife died on Tuesday\" is kinder than a soft way round\n    it\") where the child's words are three lines about how to say it at all. The reading is\n    there for the turn those modules leave alone - the ask with no name in it, \"how do i tell a\n    4 year old\", which grief reads as no question it knows and passes. So it stands aside the\n    moment one of them speaks, the same way `_the_subject_was_answered` does for a card.\n    ");
    let __iter1 = py_iter_or_raise(&candidates.clone());
    for __item in __iter1 {
    candidate = __item.clone();
    if py_truthy(&{ let __b = op_ge(&_priority(&[candidate.clone()]), &g_WHOLE_LINE_BAND()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &pyrt::xrefs::xget("compose.fallback", "FALLBACK_MODULE"))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_words_for_a_child_pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut childwords: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The composer's pool for a child's words, reduced the way the candidates are.\n\n    `_content` drops punctuation and case, so a pool line has to be reduced the same way before\n    the two can be compared (the em dash alone makes them unequal).\n    ");
    childwords = pyrt::xrefs::xmodule("compose.childwords");
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&childwords.clone(), "THE_WORDS_FOR_A_CHILD")) { let mut line: Py = __item.clone(); __out.push(_content(&[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ask_about_a_child(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut last: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The index of the knowledge entry that answers \"how do I tell the child\", else None.\n\n    The person is not asking what to say to themselves about a death; they are asking for the\n    words for a child, and the entry that has them (`explaining-death-to-a-child`) is the only\n    reply that answers the question. `loss` answers bad news well and declares `ANSWERS_INSTEAD`,\n    so without this the ask got advice for talking to a grown-up (\"use the person's name, don't\n    rush to fill the silence\") on the lines the entry now reads: a niece, a grandson, \"my girls\".\n    Read before that declaration, because the question is not the loss.\n\n    Strict, and only on the lines the composer read whole: the reading says no to a death named\n    as today and to the child who is the one who died, so a bereaved parent keeps the condolence.\n    ");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "asks_what_to_tell_a_child", &[clean.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    candidate = __unpk7[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &py_str("answers"))); if py_truthy(&__b) { __b } else { op_lt(&_priority(&[candidate.clone()]), &Py::Int(0)) } }) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } } } }) {
    continue;
    }
    return index.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_name_they_gave(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The reply that takes up the name the bot has just asked for, or None.\n\n    \"we got a puppy on saturday\" is answered \"What's its name?\", and the next line - \"hes called\n    biscuit\", or just \"biscuit\" - is the answer to that question. The knowledge base reads the\n    name as a subject instead: biscuit is a food, so the puppy got the biscuit recipe, which is\n    the one reply nobody asked for. A line that only gives the name goes to the highest reply\n    that says the name back, because that reply is the one that has taken the answer up.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__naming(), &Py::None))) {
    return Py::None;
    }
    name = pyrt::xrefs::xcall("memory.named", "the_name_given", &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    said = mcall(&name.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&op_lt(&_priority(&[candidate.clone()]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone()))) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _about_the_person_the_line_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut first: Py = Py::None;
    let mut front: Py = Py::None;
    let mut index: Py = Py::None;
    let mut named: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The index of a reply about the person a how-to line names, when the front one is not.\n\n    The card in front answers the speaker's own life; the entry below it answers the question\n    about the other person. Read only on a how-to line that names someone (\"how do i stop my\n    teenager staying up all night\"), so a line about the speaker is untouched. The person the\n    how-to is about is the first person the line names, so \"how do i console a friend whos just\n    lost their dad\" is about the friend and their grief, not a cue to answer about a dad. And a\n    reply that says it cannot answer is never taken over the card in front, which can.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_HOW_TO(), "match", &[clean.clone()]))) } }) {
    return Py::None;
    }
    named = mcall(&g__THE_ONE_NAMED(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&named.clone(), &Py::None))) {
    return Py::None;
    }
    first = mcall(&g__A_PERSON_NAMED(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&first.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&first.clone(), "start", &[]), &mcall(&named.clone(), "start", &[]))) } }) {
    return Py::None;
    }
    word = mcall(&geti(&mcall(&mcall(&named.clone(), "group", &[Py::Int(0)]), "split", &[]), &op_neg(&Py::Int(1))), "lower", &[]);
    if py_truthy(&op_ge(&_priority(&[geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0)))]), &g_RECALL())) {
    return Py::None;
    }
    front = _content(&[mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")])]);
    if py_truthy(&Py::Bool(!py_truthy(&front.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[op_modulo(&py_str("\\b%ss?\\b"), &stdlib_call("re", "escape", &[mcall(&word.clone(), "rstrip", &[py_str("s")])]).unwrap_or_else(|e| exc::raise(e))), front.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&g__ABOUT_SOMEONE_ELSE(), "search", &[front.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&pool.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    index = __item.clone();
    candidate = geti(&candidates.clone(), &index.clone());
    if py_truthy(&{ let __b = op_ge(&_priority(&[candidate.clone()]), &g_RECALL()); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &py_str("answers"))) } }) {
    continue;
    }
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } } } }) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "is_a_hedge", &[reply.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[op_modulo(&py_str("\\b%ss?\\b"), &stdlib_call("re", "escape", &[mcall(&word.clone(), "rstrip", &[py_str("s")])]).unwrap_or_else(|e| exc::raise(e))), reply.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&g__ABOUT_SOMEONE_ELSE(), "search", &[reply.clone()]) } }) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_knowledge_entry_has_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut candidate: Py = Py::None;
    let _ = py_str("Whether the knowledge base proposed an answer to this line.\n\n    The composer stands down for \"how do I tell a child\" only when something does have the\n    words. The reading is a rough match on words, and a line it reads which no entry answers\n    would otherwise lose its condolence to a nod: \"my son died and i have to tell my daughter\"\n    is a bereaved parent, and the child who has to be told is the next turn, not this one.\n    ");
    let __iter1 = py_iter_or_raise(&candidates.clone());
    for __item in __iter1 {
    candidate = __item.clone();
    if py_truthy(&{ let __b = op_ge(&call_builtin("int", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("priority")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &py_str("answers"))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _hurting_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut index: Py = Py::None;
    let _ = py_str("The index of the composer's reply to a wish to hurt somebody, or None.\n\n    \"i want to smack my son\" reached the answer written for a child who is being hit, and came\n    back \"Being hit needs dealing with every time ... say the rule in one sentence\" - a parent\n    being told how to handle somebody else's violence while saying they want to smack their own\n    child. The reading and the wording both live in bot/compose/harm.py, so this is only the\n    routing decision: when the composer has a line for this, it gets the turn.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&g__harm(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clean.clone())) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.harm", "reads_as_hurting_someone", &[clean.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&op_add(&call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter6 {
    index = __item.clone();
    let __try7 = exc::catch(|| {
    if py_truthy(&pyrt::xrefs::xcall("compose.harm", "reads_as_our_reply", &[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])])) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
    continue;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    }
    return Py::None;
    Py::None
}
pub fn _story_goes_on_reply(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut candidate: Py = Py::None;
    let mut index: Py = Py::None;
    let mut pools: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The index of the composer's listening line for the next line of a hard story, or None.\n\n    The reading lives in bot/memory/ack.py (`carries_the_story_on`: bad news in the last two\n    turns, and this line about someone else doing something) and the wording next to it, so this\n    is only the routing decision - the same shape as `_sudden_loss_reply` above, for the turn\n    after bad news rather than the bad news itself.\n\n    The one thread the listening reply does not keep is a thread that began with news that has a\n    name in it: a condition on somebody they love, a court letter, an award stopped, the roof going\n    (`badnews.NEWS_WITH_A_NAME`). Those lines are answered gently and then the person asks about\n    the thing itself - \"my dad has parkinsons\" then \"he keeps falling\" is the falls card's turn,\n    and \"she keeps asking for her mum\" after a diagnosis is the question the dementia answer is\n    for. The listening line is for the thread a death, a break-up or a refusal began, where the\n    next line is more of the same story - `_a_story_opened` reads the two, and `_a_refusal` keeps\n    \"she wont eat\" after a diagnosis with the listening line rather than the feeding card.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__ack(), &Py::None))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.ack", "carries_the_story_on", &[text.clone(), memory.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_a_story_opened(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_refusal(&[text.clone()]))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&candidates.clone()) { let mut candidate: Py = __item.clone(); __out.push(op_ge(&_priority(&[candidate.clone()]), &Py::Int(0))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    pools = py_tuple(vec![pyrt::xrefs::xget("memory.ack", "THE_STORY_GOES_ON")]);
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    candidate = __unpk7[1].clone();
    if py_truthy(&op_ge(&call_builtin("int", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("priority")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e)), &g_SAFETY())) {
    continue;
    }
    reply = mcall(&call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_content(&[reply.clone()]), &last.clone())) } } } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pools.clone()) { let mut wanted: Py = __item.clone(); __out.push(Py::Bool(py_contains(&wanted.clone(), &reply.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![Py::Bool(true), index.clone()]);
    }
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return Py::None;
    Py::None
}
pub fn _a_story_opened(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the last turn or two said a death or a leaving, rather than news about somebody living.\n\n    The listening line is for the line after a story has begun - \"my nan died last week\" / \"he\n    took the car\", \"my wife left me\" / \"she packed her things last night\" - because there is\n    nothing to answer and the person is telling it. After news about somebody who is still here\n    (\"my mum has cancer\", \"my son is in hospital\") the next line asks about that: \"she keeps\n    falling\" wants the falls card, and a listening line takes it away.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__loss(), &Py::None))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__dream(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("memory.dream", "named", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.dream", "a_death_came_first", &[said.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.loss", "reads_as_loss", &[said.clone()])) {
    return Py::Bool(true);
    }
    let __try2 = exc::catch(|| {
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.badnews", "reads_as_the_leaving", &[said.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.badnews", "reads_as_a_leaving_already_gone", &[said.clone()]) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_refusal(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is somebody refusing something, rather than something happening to them.");
    return call_builtin("bool", &[mcall(&g_REFUSAL(), "search", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _is_loss_reply_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut pools: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pools") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pools"))) };
    let mut index: Py = Py::None;
    let mut text: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The index of the candidate whose reply is one of these pools' lines, else None.");
    let __iter1 = py_iter_or_raise(&op_add(&call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    index = __item.clone();
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pools.clone()) { let mut wanted: Py = __item.clone(); __out.push(Py::Bool(py_contains(&wanted.clone(), &text.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _composer_reading(args: &[Py]) -> Py {
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
    let mut composer: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut target: Py = Py::None;
    let _ = py_str("What the composer says this line is: (kind, target), or (\"\", ()) for an ordinary line.\n\n    The routing lane's one door to bot/compose/: a line still inside a safety thread is the one\n    kind of line the rest of the app must not answer as small talk, and the composer owns both\n    the judgement and the wording. The arbiter only decides when a module keeps the turn\n    instead. Adding a second kind is one branch in the composer and one here, never a second\n    scanning loop.\n    ");
    let __try1 = exc::catch(|| {
    composer = pyrt::xrefs::xmodule("compose.fallback");
    let __unpk6 = py_unpack(&mcall(&composer.clone(), "reading", &[clean.clone(), memory.clone()]), 2, None);
    kind = __unpk6[0].clone();
    target = __unpk6[1].clone();
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
    return py_tuple(vec![py_str(""), py_tuple(vec![])]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return if py_truthy(&kind.clone()) { py_tuple(vec![kind.clone(), target.clone()]) } else { py_tuple(vec![py_str(""), py_tuple(vec![])]) };
    Py::None
}
pub fn _carries_the_answer(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut composer: Py = Py::None;
    let _ = py_str("True when a reply comes back to the answer the bot gave a turn ago, else False.\n\n    The reading is the composer's (bot/compose/verdict.py): the step a follow-up asks for is in\n    that answer, so a line that carries a run of its words is answering the same question, and\n    one that shares nothing with it is a different fact about the same topic.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    composer = pyrt::xrefs::xmodule("compose.verdict");
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&composer.clone(), "carries_the_answer", &[reply.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _a_check_on_the_answer(args: &[Py]) -> Py {
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
    let mut composer: Py = Py::None;
    let _ = py_str("True when a reply is the composer's answer to a check on the answer above, else False.\n\n    The reading is the composer's (bot/compose/verdict.py): \"will that work\", \"would that be worth\n    it\" and \"is that the right thing to do\" are the same check on the method just given, and the\n    composer's line for it is that method's own reading.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    composer = pyrt::xrefs::xmodule("compose.verdict");
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&composer.clone(), "a_check_on_the_reply", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut composer: Py = Py::None;
    let _ = py_str("True when a reply opens on an instruction, so it is already the next step, else False.");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    composer = pyrt::xrefs::xmodule("compose.verdict");
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&composer.clone(), "opens_on_a_job", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _is_a_generic_step(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply is one of the composer's own generic first-step lines, else False.\n\n    The pool is the composer's (bot/compose/__init__.py, `STEP_WHEN_LOW`), read here rather than\n    restated: the step lines say the same few things to everybody, which is what makes them a\n    fallback, and an entry that answers the subject with something concrete is not a fallback.\n    A composer without the pool declares nothing, and then nothing is read as one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&g__STEP_WHEN_LOW())) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g__STEP_WHEN_LOW(), &mcall(&reply.clone(), "strip", &[])));
    Py::None
}
pub fn _named_topics(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let _ = py_str("The topics the line itself names, not the one the reading infers from the kind of line.\n\n    `understood[\"topics\"]` is what the words in front of the bot point at (\"my hand is still\n    sore\" -> health); `understood[\"topic\"]` also carries the fallback the reader uses for a kind\n    of line (\"hiya\" -> small-talk). A module keeps its turn inside an emergency because the\n    person asked about its subject, so only the first counts: a bare greeting is not asking\n    chat about small talk, and on a live crisis thread it was getting \"Morning! How's your day\n    going?\" while the composer was still answering it.\n    ");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    return op_sub(&_labels(&[mcall(&understood.clone(), "get", &[py_str("topics")])]), &g_CHITCHAT());
    Py::None
}
pub fn _thread_index(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut real: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "real") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: real"))) };
    let mut own: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own"))) };
    let mut target: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut last: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut care_about: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "care_about") { v.clone() }
    else { py_str("") };
    let mut memory: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut line: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { py_str("") };
    let mut asks: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut closer: Py = Py::None;
    let mut index: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut other: Py = Py::None;
    let mut pointed_at: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The composer's continuation for a safety thread, or None.\n\n    After the bot has answered an emergency, the next line (\"im scared\") is not small talk: it\n    is the same thread, and a light module proposing a reframe would win it on PRIORITY. The\n    composer already has a continuation for exactly that line, so the routing decision is to\n    give the turn to the module the composer answers through - `fallback.FALLBACK_MODULE`,\n    the one module whose whole reply is the composer's - and to nothing else. The wording\n    stays in bot/compose/ for the composer lane to own.\n\n    Matching the *module* rather than the text is what makes the thread hold: the composer\n    draws its continuation from a pool, and the draw the arbiter makes and the draw the\n    module makes are two calls, so a text match only holds while both pick the same line. On\n    the ones where they differed (\"you there?\" after \"my dad is having chest pain\"), chat's\n    \"Yep, I'm here\" took a live emergency thread - the flaky red in `test_chat`.\n\n    A module that declares the line's own topic is answering the question asked, so it keeps\n    its turn even inside the thread (a health entry quoted back mid-emergency).\n\n    The one exception is a follow-up about the very thing the bot has just answered, in a\n    first-aid thread: \"can i pop the blister\" after the scald answer names a word the clothing\n    entry claims, and that entry's answer (\"break new shoes in slowly\") is for a different\n    person with a different blister. The composer's continuation is the burn's own line, so it\n    keeps the thread. It takes both halves to say so: the thread must be a care thread (a\n    crisis answer's next question is often about something else), and the line must name\n    something the last answer named - a new question in the thread still goes to its entry.\n    ");
    text = if py_truthy(&target.clone()) { geti(&target.clone(), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&_content(&[text.clone()])))) {
    return Py::None;
    }
    asks = pyrt::xrefs::xcall("compose.fallback", "asks_about_a_care_answer", &[care_about.clone(), last.clone(), memory.clone(), own.clone()]);
    pointed_at = pyrt::xrefs::xcall("compose.outlook", "points_at_the_thread", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { text.clone() } }]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pointed_at.clone())); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&real.clone()) { let mut index: Py = __item.clone(); __out.push({ let __b = own.clone(); if !py_truthy(&__b) { __b } else { op_bitand(&_labels(&[_answered(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("module")])])]), &own.clone()) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks.clone())) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    candidate = __unpk2[1].clone();
    if py_truthy(&{ let __b = op_lt(&_priority(&[candidate.clone()]), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&candidate.clone(), "get", &[py_str("module")]), &pyrt::xrefs::xget("compose.fallback", "FALLBACK_MODULE"))) } }) {
    reply = _content(&[mcall(&candidate.clone(), "get", &[py_str("reply")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&reply.clone(), &last.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&asks.clone()) {
    mine = pyrt::xrefs::xcall("compose.fallback", "how_much_of_the_answer_kept", &[mcall(&candidate.clone(), "get", &[py_str("reply")]), memory.clone()]);
    let __iter3 = py_iter_or_raise(&real.clone());
    for __item in __iter3 {
    other = __item.clone();
    closer = pyrt::xrefs::xcall("compose.fallback", "how_much_of_the_answer_kept", &[mcall(&geti(&candidates.clone(), &other.clone()), "get", &[py_str("reply")]), memory.clone()]);
    if py_truthy(&op_gt(&closer.clone(), &mine.clone())) {
    return other.clone();
    }
    }
    }
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _answers_this_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when a line from below the floor tells the person something to do, else False.\n\n    Asked only where every real candidate is a slice of the last answer, so the choice is between\n    the paragraph again and a line the composer wrote. A line that opens on a job (\"Fix it if it is\n    a part and a few pounds; bin it if the repair is half the price of new\") answers the turn - it\n    says what to do about the thing the person asked about - and it takes the turn. A nod is not:\n    \"Got it - it goes off in two days\" and \"I'm not sure about that one, sorry\" would fit any\n    message, and the answer's slice again is better than either. The reading is the composer's\n    (`bot/compose/verdict.py: opens_on_a_job`), so the arbiter keeps no vocabulary of its own.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "is_said_it_already", &[reply.clone()]) } }) {
    return Py::Bool(false);
    }
    return _opens_on_a_job(&[reply.clone()]);
    Py::None
}
pub fn _content(args: &[Py]) -> Py {
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
    let _ = py_str("A reply reduced to what it says, for the never-repeat rule.");
    return mcall(&py_str(" "), "join", &[mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]);
    Py::None
}
pub fn _echoes(args: &[Py]) -> Py {
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
    let mut announced: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "announced") { v.clone() }
    else { Py::Bool(false) };
    let mut handed: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut run: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a reply is only words the bot said in its last answer.\n\n    Two shapes. The first is the last answer said again with a nod in front: a bare follow-up\n    (\"why is that\", \"what should i do\") is answered \"Same as before: <the opening of the last\n    answer>\", which is not a second answer, because the person read those words a turn ago and is\n    asking about them. The never-repeat rule compared whole replies, so a reply with three words\n    in front of the last one read as a fresh candidate.\n\n    The second is a slice of the answer with nothing in front of it: after \"my washing machines\n    making a bang on the spin\", \"is it worth fixing or shall i just get a new one\" got the\n    sentence about the transport bolts, which is the branch of that answer the person had not read\n    - words already on the screen, dressed as a reply to a question they never answered.\n\n    The shape stays narrow on purpose, so nothing that adds anything can be taken for a repeat: at\n    most six words of lead-in, then twelve words or more of the last answer word for word, in one\n    run to the end of the reply. Anything the candidate says of its own after the run is new, and\n    a reply shorter than twelve words is a sentence a friend might choose to say again.\n    ");
    words = mcall(&_content(&[reply.clone()]), "split", &[]);
    previous = mcall(&_content(&[last.clone()]), "split", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[previous.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), op_add(&call_builtin("min", &[Py::Int(6), op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    lead = __item.clone();
    run = slice_or_raise(&words.clone(), &lead.clone(), &Py::None, &Py::None);
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[run.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)); if !py_truthy(&__b) { __b } else { _a_run_of(&[previous.clone(), run.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&announced.clone()) {
    handed = _the_quoted_repeat(&[reply.clone()]);
    if py_truthy(&handed.clone()) {
    run = mcall(&handed.clone(), "split", &[]);
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[run.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if !py_truthy(&__b) { __b } else { _a_run_of(&[previous.clone(), run.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_answer_again_with_a_tail(args: &[Py]) -> Py {
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
    let mut previous: Py = Py::None;
    let mut same: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply opens on the last answer and then adds a short remark of its own.\n\n    `_echoes` reads two shapes of repeat, and both of them end in the answer: a lead and then the\n    answer, or a slice of it with nothing after. This is the third shape, and the one a claim uses\n    to keep a line it should have given up: the whole answer, word for word from its first word,\n    and then a sentence of its own on the end.\n\n    \"im knackered all the time\" / \"i only get about five hours sleep\" is the case. The memory\n    store answers as it comes, and what it handed back was the tiredness answer from the top -\n    every word of it - with \"Five hours is more than enough for all of that\" added. The\n    knowledge entry beside it had the answer the line asked for (\"Five hours is on the short side:\n    most adults need seven to nine\"), and the store's tail was not only a repeat, it was the\n    opposite of what the card says. The tail is new, so `_echoes` leaves the whole reply alone;\n    this reading says the reply is the answer again, and settling a claim on it hands the person\n    the paragraph they read a turn ago.\n\n    Narrow on purpose: the answer has to be sixteen words or more, the reply has to open on it\n    from the first word, and what comes after may be no more than twelve. A store that quotes a\n    line and adds the one thing that matters most is doing its job, so that is not this shape.\n    ");
    words = mcall(&_content(&[reply.clone()]), "split", &[]);
    previous = mcall(&_content(&[last.clone()]), "split", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[previous.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_AGAIN_FIRST_WORDS())) {
    return Py::Bool(false);
    }
    same = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&same.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&same.clone(), &call_builtin("len", &[previous.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &same.clone()), &geti(&previous.clone(), &same.clone()))) } } } }) {
    let __aug1 = op_add(&same.clone(), &Py::Int(1));
    same = __aug1;
    }
    return { let __b = op_ge(&same.clone(), &g_AGAIN_FIRST_WORDS()); if !py_truthy(&__b) { __b } else { op_le(&op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &same.clone()), &g_AGAIN_TAIL_WORDS()) } };
    Py::None
}
pub fn _the_quoted_repeat(args: &[Py]) -> Py {
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
    let mut frame: Py = Py::None;
    let mut head: Py = Py::None;
    let mut heads: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The part of the last answer a lead says it is handing back, or None.\n\n    A lead that names itself as a repeat (\"It's what I said a moment ago: ...\", \"Same answer:\n    ...\") is longer than the six words `_echoes` allows in front of a quote, so a reply wearing\n    one was read as a new answer when it was the paragraph again. The lead has to be one the\n    other lanes write themselves (`_repeat_leads`, the same list `_announces_a_repeat` reads),\n    and the words after it have to be the answer's, because a lead that says it is handing on the\n    *next* part of the answer (\"There was a second bit to that answer: ...\") is the answer the\n    person has not read yet and must not be taken for a repeat.\n\n    Not called `_announces_a_repeat`: that is the rule that settles a turn where every candidate\n    is the answer over again, and it answers yes or no. A second function of that name read every\n    reply as the quiet one and turned \"what do i bring\" from the answer in hand into a question.\n    ");
    text = _content(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    heads = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_repeat_leads(&[])) { let mut lead: Py = __item.clone(); __out.push(_content(&[lead.clone()])); } py_list(__out) })() };
    let __try1 = exc::catch(|| {
    let _ = mcall(&heads.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("getattr", &[g__repeat(), py_str("LEADS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))) { let mut frame: Py = __item.clone(); __out.push(_content(&[geti(&mcall(&call_builtin("str", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{")]), &Py::Int(0))])); } py_list(__out) })() }]);
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&heads.clone());
    for __item in __iter6 {
    head = __item.clone();
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { mcall(&text.clone(), "startswith", &[head.clone()]) } }) {
    return mcall(&slice_or_raise(&text.clone(), &call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _points_back(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply only says the answer above covers the thing, adding nothing.");
    return call_builtin("bool", &[mcall(&g_A_POINTS_BACK(), "search", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_run_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut previous: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: previous"))) };
    let mut run: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "run") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: run"))) };
    let mut size: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("True when `run` is a continuous run of words inside `previous`, the last reply.");
    size = call_builtin("len", &[run.clone()]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(0), op_add(&op_sub(&call_builtin("len", &[previous.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut start: Py = __item.clone(); __out.push(Py::Bool(py_eq(&slice_or_raise(&previous.clone(), &start.clone(), &op_add(&start.clone(), &size.clone()), &Py::None), &run.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_answer_written_out(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The last answer as the person read it: punctuation and case kept.");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    return py_str("");
    Py::None
}
pub fn _last_reply(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return _content(&[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")])]);
    }
    return py_str("");
    Py::None
}
pub fn _said_before(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut asked: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whole answers the bot has already given this person, as `_content` strings.\n\n    Only answers of two or three sentences count (`at least eight words`): a short \"Got it.\" is\n    a thing a friend says twice and no repeat, and it is never the whole of a real answer. The\n    turn being answered is not in `memory[\"turns\"]` yet, so this is the conversation so far.\n\n    A turn whose question is this line word for word is left out: the same question asked again\n    is answered again, in the wording the repeat frame gives it (`bot/memory/again.py`), and\n    that rule is what tells the person where the answer went. So this is only the answers to\n    other questions, which have no business coming back here.\n    ");
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return said.clone();
    }
    asked = _content(&[text.clone()]);
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_content(&[mcall(&turn.clone(), "get", &[py_str("user")])]), &asked.clone())) } }) {
    continue;
    }
    reply = _content(&[mcall(&turn.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&op_ge(&call_builtin("len", &[mcall(&reply.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__SAID_BEFORE_WORDS())) {
    let _ = mcall(&said.clone(), "add", &[reply.clone()]);
    }
    }
    return said.clone();
    Py::None
}
pub fn _about(args: &[Py]) -> Py {
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
    let mut carried: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut own: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What this turn is about: (the topic the reading gives it, the topic only a stored record\n    names, the line's words, the whole line), all lower case.\n\n    The first set is the understanding engine's reading of this turn, `understood[\"topics\"]`\n    (the line by itself) and `[\"topic\"]` (the line plus what a follow-up inherits from the\n    conversation), and both count 2: they are one reading, and the engine has already decided\n    how much a follow-up leans on the line before. So on a follow-up the conversation's topic\n    ties with the line's own, and PRIORITY settles it - whether that is the right rule is\n    written down in the routing lane plan, not decided here. Only a record that names a topic\n    outright (`memory[\"topic\"] = {\"label\": \"cooking\"}`, the shape the memory lane agreed) is\n    the weaker signal, at 1.\n    ");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    own = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_bitor(&own.clone(), &_labels(&[mcall(&understood.clone(), "get", &[py_str("topics")])]));
    own = __aug1;
    let __aug2 = op_bitor(&own.clone(), &_labels(&[mcall(&understood.clone(), "get", &[py_str("topic")])]));
    own = __aug2;
    let __iter3 = py_iter_or_raise(&_strings(&[mcall(&understood.clone(), "get", &[py_str("words")])]));
    for __item in __iter3 {
    word = __item.clone();
    let _ = mcall(&words.clone(), "update", &[mcall(&g_WORD(), "findall", &[mcall(&word.clone(), "lower", &[])])]);
    }
    }
    carried = op_sub(&_labels(&[mcall(&memory.clone(), "get", &[py_str("topic")])]), &own.clone());
    clean = mcall(&py_str(" "), "join", &[mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]);
    let _ = mcall(&words.clone(), "update", &[mcall(&clean.clone(), "split", &[])]);
    return py_tuple(vec![own.clone(), carried.clone(), words.clone(), clean.clone()]);
    Py::None
}
pub fn _strings(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut item: Py = Py::None;
    let _ = py_str("The strings in value, one level deep (a list, a tuple, a set, or one string).");
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_list(vec![value.clone()]);
    }
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut item: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[item.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(item.clone()); } } py_list(__out) })() };
    }
    return py_list(vec![]);
    Py::None
}
pub fn _label(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("One topic label, canonical: lower case words joined by spaces, so \"small-talk\",\n    \"Small talk\" and \"small_talk\" are the same topic.");
    return mcall(&py_str(" "), "join", &[mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]);
    Py::None
}
pub fn _labels(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut found: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let _ = py_str("The topic labels in a value the understanding engine or the conversation wrote: a label,\n    a list of labels, one of the records a lane writes (`{\"id\": ...}` from the understanding\n    engine, `{\"label\": ...}` or `{\"text\": ...}` from the memory store), or a mapping of topic\n    -> score. Anything else, including a whole memory dict, adds nothing.\n\n    A record is read from its label key only. The other keys of a record describe it (\"turn\",\n    \"kind\", \"words\") and are not labels, so reading them would make every specialist miss\n    the topic and the specialist rule silent.");
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "add", &[_label(&[value.clone()])]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_bitor(&found.clone(), &_stored(&[value.clone()]));
    found = __aug1;
    let __iter2 = py_iter_or_raise(&g_LABEL_KEYS());
    let mut __else3 = false;
    for __item in __iter2 {
    key = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[mcall(&value.clone(), "get", &[key.clone()]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "add", &[_label(&[geti(&value.clone(), &key.clone())])]);
    break;
    }
    }
    if py_truthy(&Py::Bool(py_contains(&value.clone(), &py_str("topics")))) {
    let __aug4 = op_bitor(&found.clone(), &_labels(&[mcall(&value.clone(), "get", &[py_str("topics")])]));
    found = __aug4;
    } else {
    if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&value.clone(), "values", &[])) { let mut item: Py = __item.clone(); __out.push(Py::Bool(!py_truthy(&call_builtin("isinstance", &[item.clone(), py_tuple(vec![py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __iter5 = py_iter_or_raise(&value.clone());
    for __item in __iter5 {
    key = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[key.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LABEL_KEYS(), &key.clone())) } }) {
    let _ = mcall(&found.clone(), "add", &[_label(&[key.clone()])]);
    }
    }
    }
    }
    } else {
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter6 = py_iter_or_raise(&value.clone());
    for __item in __iter6 {
    item = __item.clone();
    let __aug7 = op_bitor(&found.clone(), &_labels(&[item.clone()]));
    found = __aug7;
    }
    }
    }
    }
    let _ = mcall(&found.clone(), "discard", &[py_str("")]);
    return found.clone();
    Py::None
}
pub fn _stored(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut record: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "record") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: record"))) };
    let mut text: Py = Py::None;
    let _ = py_str("The topic of the line the conversation is on, when a record only keeps the line itself\n    (`{\"turn\", \"text\", \"words\"}`, the shape the memory store writes). The stored line is read\n    with the understanding engine, the same reader the current line gets, so a follow-up (\"and\n    how long?\") is about what the earlier line was about and the specialist rule still fires.\n    ");
    text = if py_truthy(&call_builtin("isinstance", &[mcall(&record.clone(), "get", &[py_str("text")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&record.clone(), "get", &[py_str("text")]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[])))) {
    text = mcall(&py_str(" "), "join", &[_strings(&[mcall(&record.clone(), "get", &[py_str("words")])])]);
    }
    return _read_topic(&[text.clone()]);
    Py::None
}
pub fn _read_topic(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut normalise: Py = Py::None;
    let mut read: Py = Py::None;
    let mut score: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The topic id the understanding engine gives for a line, or nothing when it gives none.");
    key = mcall(&py_str(" "), "join", &[mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    found = mcall(&g_READ(), "get", &[key.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    let __try1 = exc::catch(|| {
    understood = pyrt::xrefs::xmodule("understanding.topics");
    normalise = py_func("normalise", |args: &[Py]| pyrt::xrefs::xcall("understanding.normalize", "normalise", args));
    read = py_call(&normalise, &[key.clone()]);
    let __unpk6 = py_unpack(&mcall(&understood.clone(), "detect_topic", &[read.clone(), Py::None, mcall(&understood.clone(), "detect_entities", &[read.clone()])]), 2, None);
    topic = __unpk6[0].clone();
    score = __unpk6[1].clone();
    found = if py_truthy(&{ let __b = topic.clone(); if !py_truthy(&__b) { __b } else { op_ge(&score.clone(), &Py::Int(2)) } }) { py_set(vec![_label(&[topic.clone()])]) } else { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) };
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
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let _ = py_setitem(&g_READ(), &key.clone(), found.clone());
    }
    return call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _answered(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut module: Py = Py::None;
    let _ = py_str("The topic labels a module declares it answers. Declared per module, so it is read from\n    the module itself (README.md, \"Interface\").");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("."), &call_builtin("str", &[module_name.clone()]).unwrap_or_else(|e| exc::raise(e))), py_str("bot.modules")]).unwrap_or_else(|e| exc::raise(e));
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
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return _labels(&[call_builtin("getattr", &[module.clone(), g_DECLARED_TOPICS(), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _match(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut module_name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "module_name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: module_name"))) };
    let mut own: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own"))) };
    let mut carried: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: carried"))) };
    let mut words: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut label: Py = Py::None;
    let mut padded: Py = Py::None;
    let mut score: Py = Py::None;
    let _ = py_str("How well a module's declared topics fit this turn: 2 for a topic id the reading gives\n    this turn (the line, or on a follow-up the conversation it is still on), 1 for one only a\n    stored record names, 1 for a declared phrase or word the line contains. 0 when it declares\n    nothing that fits, so a generalist keeps its PRIORITY over a specialist of some other\n    topic.");
    score = Py::Int(0);
    padded = op_add(&op_add(&py_str(" "), &clean.clone()), &py_str(" "));
    let __iter1 = py_iter_or_raise(&_answered(&[module_name.clone()]));
    for __item in __iter1 {
    label = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&own.clone(), &label.clone()))) {
    let __aug2 = op_add(&score.clone(), &Py::Int(2));
    score = __aug2;
    } else {
    if py_truthy(&Py::Bool(py_contains(&carried.clone(), &label.clone()))) {
    let __aug3 = op_add(&score.clone(), &Py::Int(1));
    score = __aug3;
    } else {
    if py_truthy(&Py::Bool(py_contains(&label.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&padded.clone(), &op_add(&op_add(&py_str(" "), &label.clone()), &py_str(" "))))) {
    let __aug4 = op_add(&score.clone(), &Py::Int(1));
    score = __aug4;
    }
    } else {
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &label.clone()))) {
    let __aug5 = op_add(&score.clone(), &Py::Int(1));
    score = __aug5;
    }
    }
    }
    }
    }
    return score.clone();
    Py::None
}
pub fn _the_subject_words(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words the line says its subject with - what follows a determiner, else an empty set.");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_A_SUBJECT_PHRASE(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]));
    for __item in __iter1 {
    phrase = __item.clone();
    let _ = mcall(&words.clone(), "update", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_SUBJECT_WORD(), "findall", &[phrase.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_SUBJECT_WORD(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    }
    return words.clone();
    Py::None
}
pub fn _the_lines_own_words(args: &[Py]) -> Py {
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
    let mut subject: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the line that say what it is about, else an empty set.");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    words = _the_subject_words(&[text.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { understood.clone() } }) {
    subject = mcall(&understood.clone(), "get", &[py_str("subject")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[subject.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { subject.clone() } }) {
    let _ = mcall(&words.clone(), "update", &[mcall(&g_A_SUBJECT_WORD(), "findall", &[mcall(&subject.clone(), "lower", &[])])]);
    }
    let _ = mcall(&words.clone(), "update", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_subject_words(&[{ let __b = subject.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_SUBJECT_WORD(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_SUBJECT_WORD(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _the_forms_of(args: &[Py]) -> Py {
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
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let _ = py_str("The word and the endings English puts on it - \"bins\", \"bin\", \"emptied\", \"emptying\".");
    forms = py_set(vec![word.clone()]);
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("es")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)]);
    }
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ed")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let _ = mcall(&forms.clone(), "update", &[py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)])]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    let _ = mcall(&forms.clone(), "update", &[py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("e"))])]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&op_ge(&call_builtin("len", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(form.clone()); } } py_set(__out) })() };
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
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("True when `said` is a form of `word` - \"bin\" for \"bins\", \"empty\" for \"emptied\".");
    return Py::Bool(py_contains(&_the_forms_of(&[word.clone()]), &said.clone()));
    Py::None
}
pub fn _names_a_word_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut a_reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a_reply"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the reply says one of `words` as a word of its own, not merely a sound of it.");
    said = mcall(&g_A_SUBJECT_WORD(), "findall", &[_content(&[a_reply.clone()])]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_the_same_word(&[one.clone(), word.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_card_names_the_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut other: Py = Py::None;
    let mut other_candidate: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when another real candidate names the line's own words and this one names none.\n\n    A stock line can never name the thing the person said - it does not know it - so a take that\n    shares no word with the line is the one case where a card beside it answers more of the line\n    than the take does. Every line whose words the take does say is left alone, as is every line\n    with no card beside it: this passes over a candidate, it never chooses one, and no PRIORITY is\n    read or changed anywhere.\n    ");
    words = _the_lines_own_words(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_names_a_word_of(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), words.clone()])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    other = __unpk2[0].clone();
    other_candidate = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &index.clone())); if py_truthy(&__b) { __b } else { op_lt(&_priority(&[other_candidate.clone()]), &Py::Int(0)) } }) {
    continue;
    }
    if py_truthy(&_names_a_word_of(&[mcall(&other_candidate.clone(), "get", &[py_str("reply")]), words.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_detail_added_to_the_news(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut index: Py = Py::None;
    let _ = py_str("The index of the composer's line for a detail added to news the bot has just cheered.\n\n    Read only on the turn after a cheer this bot gave (`goodnews.the_reply_just_cheered`), and only\n    for a short line with nothing asked in it: which go it was (\"first time\", \"third go\") or the\n    score (\"2-0\"). Nothing in those names a subject, so every card in the list that matched the\n    conversation's topic is answering a question nobody asked - the driving-test one, the \"That's\n    2\" arithmetic - and the pool that takes the detail as news is the answer. It never picks a\n    fallback and reads no PRIORITY: only a line of `goodnews`'s own pool takes the turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.goodnews", "reads_as_a_detail_of_the_news", &[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&op_lt(&_priority(&[geti(&candidates.clone(), &index.clone())]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.goodnews", "reads_as_the_detail_line", &[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")])])) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _shares_a_stem(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let _ = py_str("True when two words are the same word with an ending on - \"shrink\", \"shrinking\",\n    \"shrinkage\". Five letters is the shortest run that does not join \"there\" to \"these\".");
    return { let __b = op_ge(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&one.clone(), &Py::None, &Py::Int(5), &Py::None), &slice_or_raise(&other.clone(), &Py::None, &Py::Int(5), &Py::None))) } } } };
    Py::None
}
pub fn _the_word_the_line_added(args: &[Py]) -> Py {
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
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut fresh: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The longest word the line has that the answer above does not, else \"\".\n\n    \"also how do i stop it shrinking next time\" adds \"shrinking\" to an answer about blotting a\n    coffee stain. The longest one is the one that names the new subject: the short words of a\n    follow-up (\"next\", \"time\", \"stop\") are also words of a hundred other replies, so a reply that\n    says one of them is not a reply about what was added.\n    ");
    said = mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_SUBJECT_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_SUBJECT_WORD(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_NEW_WORD(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(_shares_a_stem(&[word.clone(), one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&fresh.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fresh.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _says_nothing_new(args: &[Py]) -> Py {
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
    let mut fresh: Py = Py::None;
    let mut glue: Py = Py::None;
    let mut loose: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when every word of substance in the reply is already in the answer above.\n\n    \"The thing to do first: Blot up as much as you can with kitchen paper\" is the answer above with\n    a lead-in (\"first\" is not a word of substance), so the person has read it a turn ago and it\n    cannot be the answer to what they have just added. A fresh line says something of its own, and\n    one word of its own is enough: on \"my son is 2\" / \"he isnt talking yet\" / \"should i worry\" the\n    take \"Yes - it is worth acting on rather than sitting on. Asking costs very little\" is not a\n    repeat of the speech answer, so it keeps the turn.\n    ");
    said = call_builtin("set", &[mcall(&g_WORD(), "findall", &[_content(&[last.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    glue = op_bitor(&g_NOT_A_SUBJECT_WORD(), &g_NOT_A_NEW_WORD());
    loose = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[_content(&[reply.clone()])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&glue.clone(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&loose.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(_shares_a_stem(&[word.clone(), one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return op_le(&call_builtin("len", &[fresh.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _names_the_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut a_reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a_reply"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply says the word, endings and all - \"Shrinkage\" for \"shrinking\".");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[_content(&[a_reply.clone()])])) { let mut said: Py = __item.clone(); __out.push(_shares_a_stem(&[said.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_answer_that_names_what_the_line_added(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut added: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The index of the reply that answers what the line has just added, where the one in front\n    does not name it at all.\n\n    A reply that repeats the answer above is not a reply to the part of the line that answer never\n    covered, however high its band: this passes over a take that leaves the added word out of its\n    own words, for a real candidate beside it that says it. Only what the line adds is read, and\n    only against the answer immediately above, so an answer that already covered the word keeps\n    its turn and a line with nothing new in it is untouched. The store's recall is left alone - a\n    hand-back that misses the added word is the memory lane's, not this rule's - and no PRIORITY is\n    read or changed.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&last.clone())) } }) {
    return Py::None;
    }
    added = _the_word_the_line_added(&[text.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&added.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_says_nothing_new(&[mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")]), last.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_names_the_word(&[mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")]), added.clone()])) {
    return Py::None;
    }
    if py_truthy(&op_ge(&_priority(&[geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0)))]), &g_RECALL())) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&pool.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&op_lt(&_priority(&[geti(&candidates.clone(), &index.clone())]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&_names_the_word(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), added.clone()])) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_card_that_names_the_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut pool: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut index: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The index of a card that names the line where the chosen take names nothing of it.\n\n    Read off the reading's own keywords, so a reply that shares one of them is about the line by\n    the same reading that says what the line is about. Only a take below the chatter band is\n    passed over, and only for a real candidate beside it: this never picks a fallback, and no\n    PRIORITY is read or changed.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { op_ge(&_priority(&[geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0)))]), &g_CHATTER()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MEMORY_MODULES(), &mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("module")]))); if !py_truthy(&__b) { __b } else { _a_recall_from_an_earlier_turn(&[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    words = _the_lines_own_words(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { _names_a_word_of(&[mcall(&geti(&candidates.clone(), &geti(&pool.clone(), &Py::Int(0))), "get", &[py_str("reply")]), words.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&pool.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&op_lt(&_priority(&[geti(&candidates.clone(), &index.clone())]), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&_names_a_word_of(&[mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]), words.clone()])) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_line_asking_for_plainer_words(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is only the ask to have the last answer put a plainer way.\n\n    The whole line has to be the ask: the words around the ask phrase are read, and one word of\n    the person's own - a thing, a skill, a part - means the line wants that thing, so it is left\n    alone. \"explain the bit about the pressure\" is a question about the pressure.\n    ");
    clean = _content(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PLAINER_ASK(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(false);
    }
    rest = mcall(&g_A_PLAINER_ASK(), "sub", &[py_str(" "), clean.clone()]);
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_SUBJECT_WORD(), "findall", &[mcall(&rest.clone(), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_PLAIN_ASK_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _says_the_answer_again(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply is the previous answer, with a lead in front of it or without one.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&last.clone())) } }) {
    return Py::Bool(false);
    }
    said = _content(&[reply.clone()]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &_content(&[last.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_is(&_the_quoted_repeat(&[reply.clone()]), &Py::None))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(last.clone()); __kws.extend(vec![("announced".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _echoes(args))(&__args) }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_plainer_reading_of_the_same_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut best: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "best") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: best"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut about: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut order: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The candidate that explains the last answer, where the turn in hand is the answer again.");
    if py_truthy(&Py::Bool(!py_truthy(&_a_line_asking_for_plainer_words(&[text.clone()])))) {
    return Py::None;
    }
    last = _last_reply(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_the_answer_again(&[mcall(&geti(&candidates.clone(), &best.clone()), "get", &[py_str("reply")]), last.clone()]))) } }) {
    return Py::None;
    }
    about = _the_subject_words(&[last.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&about.clone()))) {
    return Py::None;
    }
    order = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(call_builtin("range", &[call_builtin("len", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))); __kws.extend(vec![("key".to_string(), { let about = about.clone(); let best = best.clone(); let candidates = candidates.clone(); let index = index.clone(); let last = last.clone(); let memory = memory.clone(); let order = order.clone(); let reply = reply.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let about = about.clone(); let best = best.clone(); let candidates = candidates.clone(); let index = index.clone(); let last = last.clone(); let memory = memory.clone(); let order = order.clone(); let reply = reply.clone(); let text = text.clone(); let index: Py = __kwp(args, 0, "index").unwrap_or(Py::None); op_neg(&_priority(&[geti(&candidates.clone(), &index.clone())])) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&order.clone());
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &best.clone())); if py_truthy(&__b) { __b } else { op_lt(&_priority(&[geti(&candidates.clone(), &index.clone())]), &Py::Int(0)) } }) {
    continue;
    }
    reply = mcall(&geti(&candidates.clone(), &index.clone()), "get", &[py_str("reply")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { _says_the_answer_again(&[reply.clone(), last.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "is_a_hedge", &[reply.clone()]); if py_truthy(&__b) { __b } else { _points_back(&[reply.clone()]) } }) {
    continue;
    }
    if py_truthy(&_names_a_word_of(&[reply.clone(), about.clone()])) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.arbiter".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACK_POOL" => Some(g_ACK_POOL()),
            "AGAIN_FIRST_WORDS" => Some(g_AGAIN_FIRST_WORDS()),
            "AGAIN_TAIL_WORDS" => Some(g_AGAIN_TAIL_WORDS()),
            "ANSWERS_THE_NEWS" => Some(g_ANSWERS_THE_NEWS()),
            "ASKS_FOR_A_THING" => Some(g_ASKS_FOR_A_THING()),
            "ASK_CLAUSE_SPLIT" => Some(g_ASK_CLAUSE_SPLIT()),
            "A_CEILING" => Some(g_A_CEILING()),
            "A_CONTENT_WORD" => Some(g_A_CONTENT_WORD()),
            "A_DEFINITION" => Some(g_A_DEFINITION()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_MEASURE_ASKED" => Some(g_A_MEASURE_ASKED()),
            "A_MY_OWN_REPORT" => Some(g_A_MY_OWN_REPORT()),
            "A_PLAINER_ASK" => Some(g_A_PLAINER_ASK()),
            "A_POINTS_BACK" => Some(g_A_POINTS_BACK()),
            "A_REPORT_ASKS" => Some(g_A_REPORT_ASKS()),
            "A_REPORT_NOD" => Some(g_A_REPORT_NOD()),
            "A_SUBJECT_PHRASE" => Some(g_A_SUBJECT_PHRASE()),
            "A_SUBJECT_WORD" => Some(g_A_SUBJECT_WORD()),
            "A_SUM_TO_WORK_OUT" => Some(g_A_SUM_TO_WORK_OUT()),
            "A_VERDICT_BACK" => Some(g_A_VERDICT_BACK()),
            "A_WORKED_OUT" => Some(g_A_WORKED_OUT()),
            "CARE_THREAD" => Some(g_CARE_THREAD()),
            "CARRIES_NO_SUBJECT" => Some(g_CARRIES_NO_SUBJECT()),
            "CHATTER" => Some(g_CHATTER()),
            "CHITCHAT" => Some(g_CHITCHAT()),
            "CHOICE" => Some(g_CHOICE()),
            "CUT_MARKS" => Some(g_CUT_MARKS()),
            "DECLARED_ABOUT_THE_CHAT" => Some(g_DECLARED_ABOUT_THE_CHAT()),
            "DECLARED_AS_IT_COMES" => Some(g_DECLARED_AS_IT_COMES()),
            "DECLARED_CLAIMS_INSTEAD" => Some(g_DECLARED_CLAIMS_INSTEAD()),
            "DECLARED_DEFERS" => Some(g_DECLARED_DEFERS()),
            "DECLARED_INSTEAD" => Some(g_DECLARED_INSTEAD()),
            "DECLARED_TOPICS" => Some(g_DECLARED_TOPICS()),
            "DOING_WANTS" => Some(g_DOING_WANTS()),
            "DOUBT_WORDS" => Some(g_DOUBT_WORDS()),
            "ECHO_STOP" => Some(g_ECHO_STOP()),
            "FIRST_VERB" => Some(g_FIRST_VERB()),
            "GENERIC" => Some(g_GENERIC()),
            "INFLECTED_ENDS" => Some(g_INFLECTED_ENDS()),
            "KEEPS_ITS_OWN_ANSWER" => Some(g_KEEPS_ITS_OWN_ANSWER()),
            "LABEL_KEYS" => Some(g_LABEL_KEYS()),
            "LETTERS" => Some(g_LETTERS()),
            "LONG_ENOUGH" => Some(g_LONG_ENOUGH()),
            "MEMORY_MODULES" => Some(g_MEMORY_MODULES()),
            "MOVE_ANSWER_WORDS" => Some(g_MOVE_ANSWER_WORDS()),
            "MOVE_SHARE" => Some(g_MOVE_SHARE()),
            "MOVE_WHOLE" => Some(g_MOVE_WHOLE()),
            "MOVE_WORDS" => Some(g_MOVE_WORDS()),
            "NOT_A_NEW_WORD" => Some(g_NOT_A_NEW_WORD()),
            "NOT_A_SUBJECT" => Some(g_NOT_A_SUBJECT()),
            "NOT_A_SUBJECT_WORD" => Some(g_NOT_A_SUBJECT_WORD()),
            "OWN_ABOUT_THE_CHAT" => Some(g_OWN_ABOUT_THE_CHAT()),
            "PAUSE" => Some(g_PAUSE()),
            "PLAIN_ASK_FILLER" => Some(g_PLAIN_ASK_FILLER()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "POOL_MODULES" => Some(g_POOL_MODULES()),
            "READ" => Some(g_READ()),
            "RECALL" => Some(g_RECALL()),
            "REFUSAL" => Some(g_REFUSAL()),
            "REPEAT_FRAME" => Some(g_REPEAT_FRAME()),
            "REPORT_STOP" => Some(g_REPORT_STOP()),
            "SAFETY" => Some(g_SAFETY()),
            "SAID_BACK" => Some(g_SAID_BACK()),
            "THEIR_VOICE" => Some(g_THEIR_VOICE()),
            "THE_ANSWER_BANDS" => Some(g_THE_ANSWER_BANDS()),
            "THE_ASK_OPENS" => Some(g_THE_ASK_OPENS()),
            "THE_CAUSE" => Some(g_THE_CAUSE()),
            "THREAD" => Some(g_THREAD()),
            "TOKEN" => Some(g_TOKEN()),
            "WHOLE_LINE_BAND" => Some(g_WHOLE_LINE_BAND()),
            "WHOLE_MOVE_WORDS" => Some(g_WHOLE_MOVE_WORDS()),
            "WORD" => Some(g_WORD()),
            "WORD_ENDINGS" => Some(g_WORD_ENDINGS()),
            "_ABOUT_SOMEONE_ELSE" => Some(g__ABOUT_SOMEONE_ELSE()),
            "_ALLERGY_LIFTED" => Some(g__ALLERGY_LIFTED()),
            "_AN_AMENDMENT" => Some(g__AN_AMENDMENT()),
            "_A_HOW_TO" => Some(g__A_HOW_TO()),
            "_A_PERSON_NAMED" => Some(g__A_PERSON_NAMED()),
            "_CONDOLENCE" => Some(g__CONDOLENCE()),
            "_FIGURES" => Some(g__FIGURES()),
            "_FRAME" => Some(g__FRAME()),
            "_MONTHS" => Some(g__MONTHS()),
            "_NUMBER_WORDS" => Some(g__NUMBER_WORDS()),
            "_PEOPLE" => Some(g__PEOPLE()),
            "_REFUSAL_UNDONE" => Some(g__REFUSAL_UNDONE()),
            "_SAID_BEFORE_WORDS" => Some(g__SAID_BEFORE_WORDS()),
            "_STEP_WHEN_LOW" => Some(g__STEP_WHEN_LOW()),
            "_THE_NAMELESS" => Some(g__THE_NAMELESS()),
            "_THE_ONE_NAMED" => Some(g__THE_ONE_NAMED()),
            "_a_bare_doubt_reply" => Some(py_func("_a_bare_doubt_reply", |args: &[Py]| _a_bare_doubt_reply(args))),
            "_a_bare_yes_no" => Some(py_func("_a_bare_yes_no", |args: &[Py]| _a_bare_yes_no(args))),
            "_a_card_besides" => Some(py_func("_a_card_besides", |args: &[Py]| _a_card_besides(args))),
            "_a_card_names_the_line" => Some(py_func("_a_card_names_the_line", |args: &[Py]| _a_card_names_the_line(args))),
            "_a_card_that_names_the_line" => Some(py_func("_a_card_that_names_the_line", |args: &[Py]| _a_card_that_names_the_line(args))),
            "_a_check_on_the_answer" => Some(py_func("_a_check_on_the_answer", |args: &[Py]| _a_check_on_the_answer(args))),
            "_a_choice_the_answer_did_not_make" => Some(py_func("_a_choice_the_answer_did_not_make", |args: &[Py]| _a_choice_the_answer_did_not_make(args))),
            "_a_clause_was_cut_in_half" => Some(py_func("_a_clause_was_cut_in_half", |args: &[Py]| _a_clause_was_cut_in_half(args))),
            "_a_condolence" => Some(py_func("_a_condolence", |args: &[Py]| _a_condolence(args))),
            "_a_declaring_specialist_besides" => Some(py_func("_a_declaring_specialist_besides", |args: &[Py]| _a_declaring_specialist_besides(args))),
            "_a_detail_added_to_the_news" => Some(py_func("_a_detail_added_to_the_news", |args: &[Py]| _a_detail_added_to_the_news(args))),
            "_a_fresh_answer_besides" => Some(py_func("_a_fresh_answer_besides", |args: &[Py]| _a_fresh_answer_besides(args))),
            "_a_knowledge_entry_has_it" => Some(py_func("_a_knowledge_entry_has_it", |args: &[Py]| _a_knowledge_entry_has_it(args))),
            "_a_lead_line" => Some(py_func("_a_lead_line", |args: &[Py]| _a_lead_line(args))),
            "_a_leaving_said_plain" => Some(py_func("_a_leaving_said_plain", |args: &[Py]| _a_leaving_said_plain(args))),
            "_a_line_asking_for_plainer_words" => Some(py_func("_a_line_asking_for_plainer_words", |args: &[Py]| _a_line_asking_for_plainer_words(args))),
            "_a_past_that_is_not_there" => Some(py_func("_a_past_that_is_not_there", |args: &[Py]| _a_past_that_is_not_there(args))),
            "_a_plainer_reading_of_the_same_answer" => Some(py_func("_a_plainer_reading_of_the_same_answer", |args: &[Py]| _a_plainer_reading_of_the_same_answer(args))),
            "_a_presence_check" => Some(py_func("_a_presence_check", |args: &[Py]| _a_presence_check(args))),
            "_a_question_about_the_bot" => Some(py_func("_a_question_about_the_bot", |args: &[Py]| _a_question_about_the_bot(args))),
            "_a_real_answer_besides" => Some(py_func("_a_real_answer_besides", |args: &[Py]| _a_real_answer_besides(args))),
            "_a_recall_from_an_earlier_turn" => Some(py_func("_a_recall_from_an_earlier_turn", |args: &[Py]| _a_recall_from_an_earlier_turn(args))),
            "_a_refusal" => Some(py_func("_a_refusal", |args: &[Py]| _a_refusal(args))),
            "_a_repeat_to_hold" => Some(py_func("_a_repeat_to_hold", |args: &[Py]| _a_repeat_to_hold(args))),
            "_a_report_and_no_ask" => Some(py_func("_a_report_and_no_ask", |args: &[Py]| _a_report_and_no_ask(args))),
            "_a_run_of" => Some(py_func("_a_run_of", |args: &[Py]| _a_run_of(args))),
            "_a_shorter_said_again" => Some(py_func("_a_shorter_said_again", |args: &[Py]| _a_shorter_said_again(args))),
            "_a_sign_off" => Some(py_func("_a_sign_off", |args: &[Py]| _a_sign_off(args))),
            "_a_story_opened" => Some(py_func("_a_story_opened", |args: &[Py]| _a_story_opened(args))),
            "_a_whole_line_module_spoke" => Some(py_func("_a_whole_line_module_spoke", |args: &[Py]| _a_whole_line_module_spoke(args))),
            "_about" => Some(py_func("_about", |args: &[Py]| _about(args))),
            "_about_the_bot" => Some(py_func("_about_the_bot", |args: &[Py]| _about_the_bot(args))),
            "_about_the_line" => Some(py_func("_about_the_line", |args: &[Py]| _about_the_line(args))),
            "_about_the_news" => Some(py_func("_about_the_news", |args: &[Py]| _about_the_news(args))),
            "_about_the_person_the_line_names" => Some(py_func("_about_the_person_the_line_names", |args: &[Py]| _about_the_person_the_line_names(args))),
            "_ack" => Some(g__ack()),
            "_ack_lines" => Some(py_func("_ack_lines", |args: &[Py]| _ack_lines(args))),
            "_already" => Some(g__already()),
            "_amends_a_detail" => Some(py_func("_amends_a_detail", |args: &[Py]| _amends_a_detail(args))),
            "_an_answer_behind_the_ask" => Some(py_func("_an_answer_behind_the_ask", |args: &[Py]| _an_answer_behind_the_ask(args))),
            "_an_answer_it_already_gave" => Some(py_func("_an_answer_it_already_gave", |args: &[Py]| _an_answer_it_already_gave(args))),
            "_announces_a_repeat" => Some(py_func("_announces_a_repeat", |args: &[Py]| _announces_a_repeat(args))),
            "_answered" => Some(py_func("_answered", |args: &[Py]| _answered(args))),
            "_answers_this_line" => Some(py_func("_answers_this_line", |args: &[Py]| _answers_this_line(args))),
            "_as_it_comes" => Some(py_func("_as_it_comes", |args: &[Py]| _as_it_comes(args))),
            "_asks" => Some(py_func("_asks", |args: &[Py]| _asks(args))),
            "_asks_a_question" => Some(py_func("_asks_a_question", |args: &[Py]| _asks_a_question(args))),
            "_asks_about_the_chat" => Some(py_func("_asks_about_the_chat", |args: &[Py]| _asks_about_the_chat(args))),
            "_asks_for_a_judgement" => Some(py_func("_asks_for_a_judgement", |args: &[Py]| _asks_for_a_judgement(args))),
            "_asks_for_the_answer_again" => Some(py_func("_asks_for_the_answer_again", |args: &[Py]| _asks_for_the_answer_again(args))),
            "_asks_it_back" => Some(py_func("_asks_it_back", |args: &[Py]| _asks_it_back(args))),
            "_carried_words" => Some(py_func("_carried_words", |args: &[Py]| _carried_words(args))),
            "_carries_on" => Some(py_func("_carries_on", |args: &[Py]| _carries_on(args))),
            "_carries_the_answer" => Some(py_func("_carries_the_answer", |args: &[Py]| _carries_the_answer(args))),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_claims_instead" => Some(py_func("_claims_instead", |args: &[Py]| _claims_instead(args))),
            "_composer_reading" => Some(py_func("_composer_reading", |args: &[Py]| _composer_reading(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_cooking" => Some(g__cooking()),
            "_declared" => Some(py_func("_declared", |args: &[Py]| _declared(args))),
            "_declares_as_it_comes" => Some(py_func("_declares_as_it_comes", |args: &[Py]| _declares_as_it_comes(args))),
            "_declares_claims_instead" => Some(py_func("_declares_claims_instead", |args: &[Py]| _declares_claims_instead(args))),
            "_declares_instead" => Some(py_func("_declares_instead", |args: &[Py]| _declares_instead(args))),
            "_declares_topics" => Some(py_func("_declares_topics", |args: &[Py]| _declares_topics(args))),
            "_defers" => Some(py_func("_defers", |args: &[Py]| _defers(args))),
            "_defers_to" => Some(py_func("_defers_to", |args: &[Py]| _defers_to(args))),
            "_draft_index" => Some(py_func("_draft_index", |args: &[Py]| _draft_index(args))),
            "_dream" => Some(g__dream()),
            "_echoes" => Some(py_func("_echoes", |args: &[Py]| _echoes(args))),
            "_every_real_answer_offers" => Some(py_func("_every_real_answer_offers", |args: &[Py]| _every_real_answer_offers(args))),
            "_explains_the_method" => Some(py_func("_explains_the_method", |args: &[Py]| _explains_the_method(args))),
            "_fit" => Some(py_func("_fit", |args: &[Py]| _fit(args))),
            "_hands_the_decision_back" => Some(py_func("_hands_the_decision_back", |args: &[Py]| _hands_the_decision_back(args))),
            "_harm" => Some(g__harm()),
            "_history" => Some(g__history()),
            "_how_much_it_names" => Some(py_func("_how_much_it_names", |args: &[Py]| _how_much_it_names(args))),
            "_hurting_reply" => Some(py_func("_hurting_reply", |args: &[Py]| _hurting_reply(args))),
            "_in_a_careful_thread" => Some(py_func("_in_a_careful_thread", |args: &[Py]| _in_a_careful_thread(args))),
            "_instead" => Some(py_func("_instead", |args: &[Py]| _instead(args))),
            "_is_a_generic_step" => Some(py_func("_is_a_generic_step", |args: &[Py]| _is_a_generic_step(args))),
            "_is_advice_about" => Some(py_func("_is_advice_about", |args: &[Py]| _is_advice_about(args))),
            "_is_loss_reply_in" => Some(py_func("_is_loss_reply_in", |args: &[Py]| _is_loss_reply_in(args))),
            "_label" => Some(py_func("_label", |args: &[Py]| _label(args))),
            "_labels" => Some(py_func("_labels", |args: &[Py]| _labels(args))),
            "_last_reply" => Some(py_func("_last_reply", |args: &[Py]| _last_reply(args))),
            "_leaving_reply" => Some(py_func("_leaving_reply", |args: &[Py]| _leaving_reply(args))),
            "_line_topics" => Some(py_func("_line_topics", |args: &[Py]| _line_topics(args))),
            "_lineback" => Some(g__lineback()),
            "_loss" => Some(g__loss()),
            "_made_of_the_answer" => Some(py_func("_made_of_the_answer", |args: &[Py]| _made_of_the_answer(args))),
            "_match" => Some(py_func("_match", |args: &[Py]| _match(args))),
            "_my_workings" => Some(py_func("_my_workings", |args: &[Py]| _my_workings(args))),
            "_named_topics" => Some(py_func("_named_topics", |args: &[Py]| _named_topics(args))),
            "_names_a_figure" => Some(py_func("_names_a_figure", |args: &[Py]| _names_a_figure(args))),
            "_names_a_word_of" => Some(py_func("_names_a_word_of", |args: &[Py]| _names_a_word_of(args))),
            "_names_the_answers_figure" => Some(py_func("_names_the_answers_figure", |args: &[Py]| _names_the_answers_figure(args))),
            "_names_the_subject" => Some(py_func("_names_the_subject", |args: &[Py]| _names_the_subject(args))),
            "_names_the_thing" => Some(py_func("_names_the_thing", |args: &[Py]| _names_the_thing(args))),
            "_names_the_word" => Some(py_func("_names_the_word", |args: &[Py]| _names_the_word(args))),
            "_names_what_they_said" => Some(py_func("_names_what_they_said", |args: &[Py]| _names_what_they_said(args))),
            "_naming" => Some(g__naming()),
            "_offers_the_food" => Some(py_func("_offers_the_food", |args: &[Py]| _offers_the_food(args))),
            "_offers_what_the_line_ruled_out" => Some(py_func("_offers_what_the_line_ruled_out", |args: &[Py]| _offers_what_the_line_ruled_out(args))),
            "_one_word" => Some(py_func("_one_word", |args: &[Py]| _one_word(args))),
            "_only_a_hand_back_besides" => Some(py_func("_only_a_hand_back_besides", |args: &[Py]| _only_a_hand_back_besides(args))),
            "_only_acknowledges" => Some(py_func("_only_acknowledges", |args: &[Py]| _only_acknowledges(args))),
            "_only_says_the_line_back" => Some(py_func("_only_says_the_line_back", |args: &[Py]| _only_says_the_line_back(args))),
            "_opens_on_a_job" => Some(py_func("_opens_on_a_job", |args: &[Py]| _opens_on_a_job(args))),
            "_pause_reply" => Some(py_func("_pause_reply", |args: &[Py]| _pause_reply(args))),
            "_places_the_figure" => Some(py_func("_places_the_figure", |args: &[Py]| _places_the_figure(args))),
            "_points_back" => Some(py_func("_points_back", |args: &[Py]| _points_back(args))),
            "_priority" => Some(py_func("_priority", |args: &[Py]| _priority(args))),
            "_read_topic" => Some(py_func("_read_topic", |args: &[Py]| _read_topic(args))),
            "_reading" => Some(g__reading()),
            "_reads_as_a_verdict" => Some(py_func("_reads_as_a_verdict", |args: &[Py]| _reads_as_a_verdict(args))),
            "_reads_the_doubt" => Some(py_func("_reads_the_doubt", |args: &[Py]| _reads_the_doubt(args))),
            "_real_answers_besides" => Some(py_func("_real_answers_besides", |args: &[Py]| _real_answers_besides(args))),
            "_recall" => Some(g__recall()),
            "_repeat" => Some(g__repeat()),
            "_repeat_leads" => Some(py_func("_repeat_leads", |args: &[Py]| _repeat_leads(args))),
            "_ruledout" => Some(g__ruledout()),
            "_run_in_the_answer" => Some(py_func("_run_in_the_answer", |args: &[Py]| _run_in_the_answer(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "_said_it_already" => Some(py_func("_said_it_already", |args: &[Py]| _said_it_already(args))),
            "_says_back_one_of" => Some(py_func("_says_back_one_of", |args: &[Py]| _says_back_one_of(args))),
            "_says_nothing_new" => Some(py_func("_says_nothing_new", |args: &[Py]| _says_nothing_new(args))),
            "_says_only_the_line" => Some(py_func("_says_only_the_line", |args: &[Py]| _says_only_the_line(args))),
            "_says_the_answer_again" => Some(py_func("_says_the_answer_again", |args: &[Py]| _says_the_answer_again(args))),
            "_says_the_same_thing_again" => Some(py_func("_says_the_same_thing_again", |args: &[Py]| _says_the_same_thing_again(args))),
            "_says_the_subject" => Some(py_func("_says_the_subject", |args: &[Py]| _says_the_subject(args))),
            "_says_what_they_already_said" => Some(py_func("_says_what_they_already_said", |args: &[Py]| _says_what_they_already_said(args))),
            "_seed" => Some(py_func("_seed", |args: &[Py]| _seed(args))),
            "_shares_a_stem" => Some(py_func("_shares_a_stem", |args: &[Py]| _shares_a_stem(args))),
            "_store_correcting" => Some(py_func("_store_correcting", |args: &[Py]| _store_correcting(args))),
            "_stored" => Some(py_func("_stored", |args: &[Py]| _stored(args))),
            "_story_goes_on_reply" => Some(py_func("_story_goes_on_reply", |args: &[Py]| _story_goes_on_reply(args))),
            "_strings" => Some(py_func("_strings", |args: &[Py]| _strings(args))),
            "_subject_words" => Some(py_func("_subject_words", |args: &[Py]| _subject_words(args))),
            "_sudden_loss_reply" => Some(py_func("_sudden_loss_reply", |args: &[Py]| _sudden_loss_reply(args))),
            "_the_advice_over_a_definition" => Some(py_func("_the_advice_over_a_definition", |args: &[Py]| _the_advice_over_a_definition(args))),
            "_the_answer_again_with_a_tail" => Some(py_func("_the_answer_again_with_a_tail", |args: &[Py]| _the_answer_again_with_a_tail(args))),
            "_the_answer_handed_back" => Some(py_func("_the_answer_handed_back", |args: &[Py]| _the_answer_handed_back(args))),
            "_the_answer_that_names_what_the_line_added" => Some(py_func("_the_answer_that_names_what_the_line_added", |args: &[Py]| _the_answer_that_names_what_the_line_added(args))),
            "_the_answer_written_out" => Some(py_func("_the_answer_written_out", |args: &[Py]| _the_answer_written_out(args))),
            "_the_ask_about_a_child" => Some(py_func("_the_ask_about_a_child", |args: &[Py]| _the_ask_about_a_child(args))),
            "_the_ask_after_the_facts" => Some(py_func("_the_ask_after_the_facts", |args: &[Py]| _the_ask_after_the_facts(args))),
            "_the_ask_clause" => Some(py_func("_the_ask_clause", |args: &[Py]| _the_ask_clause(args))),
            "_the_bots_own_taste" => Some(py_func("_the_bots_own_taste", |args: &[Py]| _the_bots_own_taste(args))),
            "_the_candidates_that_answer_the_ask" => Some(py_func("_the_candidates_that_answer_the_ask", |args: &[Py]| _the_candidates_that_answer_the_ask(args))),
            "_the_candidates_that_give_the_shape" => Some(py_func("_the_candidates_that_give_the_shape", |args: &[Py]| _the_candidates_that_give_the_shape(args))),
            "_the_candidates_that_hand_the_detail_back" => Some(py_func("_the_candidates_that_hand_the_detail_back", |args: &[Py]| _the_candidates_that_hand_the_detail_back(args))),
            "_the_candidates_with_a_figure" => Some(py_func("_the_candidates_with_a_figure", |args: &[Py]| _the_candidates_with_a_figure(args))),
            "_the_card_that_places_the_figure" => Some(py_func("_the_card_that_places_the_figure", |args: &[Py]| _the_card_that_places_the_figure(args))),
            "_the_cause_asked_for" => Some(py_func("_the_cause_asked_for", |args: &[Py]| _the_cause_asked_for(args))),
            "_the_cut_the_line_asked_for" => Some(py_func("_the_cut_the_line_asked_for", |args: &[Py]| _the_cut_the_line_asked_for(args))),
            "_the_date_fact" => Some(py_func("_the_date_fact", |args: &[Py]| _the_date_fact(args))),
            "_the_entry_that_answers_the_ask" => Some(py_func("_the_entry_that_answers_the_ask", |args: &[Py]| _the_entry_that_answers_the_ask(args))),
            "_the_figure_they_asked_back" => Some(py_func("_the_figure_they_asked_back", |args: &[Py]| _the_figure_they_asked_back(args))),
            "_the_figures_they_gave" => Some(py_func("_the_figures_they_gave", |args: &[Py]| _the_figures_they_gave(args))),
            "_the_food_the_line_refused" => Some(py_func("_the_food_the_line_refused", |args: &[Py]| _the_food_the_line_refused(args))),
            "_the_forms_of" => Some(py_func("_the_forms_of", |args: &[Py]| _the_forms_of(args))),
            "_the_kind_line_they_gave" => Some(py_func("_the_kind_line_they_gave", |args: &[Py]| _the_kind_line_they_gave(args))),
            "_the_line_asks_for_a_doing" => Some(py_func("_the_line_asks_for_a_doing", |args: &[Py]| _the_line_asks_for_a_doing(args))),
            "_the_line_is_news" => Some(py_func("_the_line_is_news", |args: &[Py]| _the_line_is_news(args))),
            "_the_lines_own_words" => Some(py_func("_the_lines_own_words", |args: &[Py]| _the_lines_own_words(args))),
            "_the_move_in_hand" => Some(py_func("_the_move_in_hand", |args: &[Py]| _the_move_in_hand(args))),
            "_the_name_they_gave" => Some(py_func("_the_name_they_gave", |args: &[Py]| _the_name_they_gave(args))),
            "_the_next_part" => Some(py_func("_the_next_part", |args: &[Py]| _the_next_part(args))),
            "_the_numbers_in" => Some(py_func("_the_numbers_in", |args: &[Py]| _the_numbers_in(args))),
            "_the_quoted_repeat" => Some(py_func("_the_quoted_repeat", |args: &[Py]| _the_quoted_repeat(args))),
            "_the_repeat_frame" => Some(py_func("_the_repeat_frame", |args: &[Py]| _the_repeat_frame(args))),
            "_the_reply_is_the_report" => Some(py_func("_the_reply_is_the_report", |args: &[Py]| _the_reply_is_the_report(args))),
            "_the_same_plain_word" => Some(py_func("_the_same_plain_word", |args: &[Py]| _the_same_plain_word(args))),
            "_the_same_word" => Some(py_func("_the_same_word", |args: &[Py]| _the_same_word(args))),
            "_the_same_word__dup1" => Some(py_func("_the_same_word__dup1", |args: &[Py]| _the_same_word__dup1(args))),
            "_the_store_reads_a_repeat" => Some(py_func("_the_store_reads_a_repeat", |args: &[Py]| _the_store_reads_a_repeat(args))),
            "_the_store_reads_an_explain_again_ask" => Some(py_func("_the_store_reads_an_explain_again_ask", |args: &[Py]| _the_store_reads_an_explain_again_ask(args))),
            "_the_store_revises_the_answer" => Some(py_func("_the_store_revises_the_answer", |args: &[Py]| _the_store_revises_the_answer(args))),
            "_the_subject_was_answered" => Some(py_func("_the_subject_was_answered", |args: &[Py]| _the_subject_was_answered(args))),
            "_the_subject_words" => Some(py_func("_the_subject_words", |args: &[Py]| _the_subject_words(args))),
            "_the_thing_the_line_ruled_out" => Some(py_func("_the_thing_the_line_ruled_out", |args: &[Py]| _the_thing_the_line_ruled_out(args))),
            "_the_venue_hours_turn" => Some(py_func("_the_venue_hours_turn", |args: &[Py]| _the_venue_hours_turn(args))),
            "_the_verdict_back" => Some(py_func("_the_verdict_back", |args: &[Py]| _the_verdict_back(args))),
            "_the_verdict_turn" => Some(py_func("_the_verdict_turn", |args: &[Py]| _the_verdict_turn(args))),
            "_the_whole_move" => Some(py_func("_the_whole_move", |args: &[Py]| _the_whole_move(args))),
            "_the_word_the_line_added" => Some(py_func("_the_word_the_line_added", |args: &[Py]| _the_word_the_line_added(args))),
            "_the_words_for_a_child_pool" => Some(py_func("_the_words_for_a_child_pool", |args: &[Py]| _the_words_for_a_child_pool(args))),
            "_the_words_for_the_child" => Some(py_func("_the_words_for_the_child", |args: &[Py]| _the_words_for_the_child(args))),
            "_the_words_the_report_names" => Some(py_func("_the_words_the_report_names", |args: &[Py]| _the_words_the_report_names(args))),
            "_there_is_a_chat" => Some(g__there_is_a_chat()),
            "_thread_index" => Some(py_func("_thread_index", |args: &[Py]| _thread_index(args))),
            "_verdict_lines" => Some(py_func("_verdict_lines", |args: &[Py]| _verdict_lines(args))),
            "_verdict_reply" => Some(py_func("_verdict_reply", |args: &[Py]| _verdict_reply(args))),
            "choose" => Some(py_func("choose", |args: &[Py]| choose(args))),
            "reads_as_a_detail_of_the_news" => Some(g_reads_as_a_detail_of_the_news()),
            "reads_as_the_detail_line" => Some(g_reads_as_the_detail_line()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACK_POOL" => s_ACK_POOL(v),
            "AGAIN_FIRST_WORDS" => s_AGAIN_FIRST_WORDS(v),
            "AGAIN_TAIL_WORDS" => s_AGAIN_TAIL_WORDS(v),
            "ANSWERS_THE_NEWS" => s_ANSWERS_THE_NEWS(v),
            "ASKS_FOR_A_THING" => s_ASKS_FOR_A_THING(v),
            "ASK_CLAUSE_SPLIT" => s_ASK_CLAUSE_SPLIT(v),
            "A_CEILING" => s_A_CEILING(v),
            "A_CONTENT_WORD" => s_A_CONTENT_WORD(v),
            "A_DEFINITION" => s_A_DEFINITION(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_MEASURE_ASKED" => s_A_MEASURE_ASKED(v),
            "A_MY_OWN_REPORT" => s_A_MY_OWN_REPORT(v),
            "A_PLAINER_ASK" => s_A_PLAINER_ASK(v),
            "A_POINTS_BACK" => s_A_POINTS_BACK(v),
            "A_REPORT_ASKS" => s_A_REPORT_ASKS(v),
            "A_REPORT_NOD" => s_A_REPORT_NOD(v),
            "A_SUBJECT_PHRASE" => s_A_SUBJECT_PHRASE(v),
            "A_SUBJECT_WORD" => s_A_SUBJECT_WORD(v),
            "A_SUM_TO_WORK_OUT" => s_A_SUM_TO_WORK_OUT(v),
            "A_VERDICT_BACK" => s_A_VERDICT_BACK(v),
            "A_WORKED_OUT" => s_A_WORKED_OUT(v),
            "CARE_THREAD" => s_CARE_THREAD(v),
            "CARRIES_NO_SUBJECT" => s_CARRIES_NO_SUBJECT(v),
            "CHATTER" => s_CHATTER(v),
            "CHITCHAT" => s_CHITCHAT(v),
            "CHOICE" => s_CHOICE(v),
            "CUT_MARKS" => s_CUT_MARKS(v),
            "DECLARED_ABOUT_THE_CHAT" => s_DECLARED_ABOUT_THE_CHAT(v),
            "DECLARED_AS_IT_COMES" => s_DECLARED_AS_IT_COMES(v),
            "DECLARED_CLAIMS_INSTEAD" => s_DECLARED_CLAIMS_INSTEAD(v),
            "DECLARED_DEFERS" => s_DECLARED_DEFERS(v),
            "DECLARED_INSTEAD" => s_DECLARED_INSTEAD(v),
            "DECLARED_TOPICS" => s_DECLARED_TOPICS(v),
            "DOING_WANTS" => s_DOING_WANTS(v),
            "DOUBT_WORDS" => s_DOUBT_WORDS(v),
            "ECHO_STOP" => s_ECHO_STOP(v),
            "FIRST_VERB" => s_FIRST_VERB(v),
            "GENERIC" => s_GENERIC(v),
            "INFLECTED_ENDS" => s_INFLECTED_ENDS(v),
            "KEEPS_ITS_OWN_ANSWER" => s_KEEPS_ITS_OWN_ANSWER(v),
            "LABEL_KEYS" => s_LABEL_KEYS(v),
            "LETTERS" => s_LETTERS(v),
            "LONG_ENOUGH" => s_LONG_ENOUGH(v),
            "MEMORY_MODULES" => s_MEMORY_MODULES(v),
            "MOVE_ANSWER_WORDS" => s_MOVE_ANSWER_WORDS(v),
            "MOVE_SHARE" => s_MOVE_SHARE(v),
            "MOVE_WHOLE" => s_MOVE_WHOLE(v),
            "MOVE_WORDS" => s_MOVE_WORDS(v),
            "NOT_A_NEW_WORD" => s_NOT_A_NEW_WORD(v),
            "NOT_A_SUBJECT" => s_NOT_A_SUBJECT(v),
            "NOT_A_SUBJECT_WORD" => s_NOT_A_SUBJECT_WORD(v),
            "OWN_ABOUT_THE_CHAT" => s_OWN_ABOUT_THE_CHAT(v),
            "PAUSE" => s_PAUSE(v),
            "PLAIN_ASK_FILLER" => s_PLAIN_ASK_FILLER(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "POOL_MODULES" => s_POOL_MODULES(v),
            "READ" => s_READ(v),
            "RECALL" => s_RECALL(v),
            "REFUSAL" => s_REFUSAL(v),
            "REPEAT_FRAME" => s_REPEAT_FRAME(v),
            "REPORT_STOP" => s_REPORT_STOP(v),
            "SAFETY" => s_SAFETY(v),
            "SAID_BACK" => s_SAID_BACK(v),
            "THEIR_VOICE" => s_THEIR_VOICE(v),
            "THE_ANSWER_BANDS" => s_THE_ANSWER_BANDS(v),
            "THE_ASK_OPENS" => s_THE_ASK_OPENS(v),
            "THE_CAUSE" => s_THE_CAUSE(v),
            "THREAD" => s_THREAD(v),
            "TOKEN" => s_TOKEN(v),
            "WHOLE_LINE_BAND" => s_WHOLE_LINE_BAND(v),
            "WHOLE_MOVE_WORDS" => s_WHOLE_MOVE_WORDS(v),
            "WORD" => s_WORD(v),
            "WORD_ENDINGS" => s_WORD_ENDINGS(v),
            "_ABOUT_SOMEONE_ELSE" => s__ABOUT_SOMEONE_ELSE(v),
            "_ALLERGY_LIFTED" => s__ALLERGY_LIFTED(v),
            "_AN_AMENDMENT" => s__AN_AMENDMENT(v),
            "_A_HOW_TO" => s__A_HOW_TO(v),
            "_A_PERSON_NAMED" => s__A_PERSON_NAMED(v),
            "_CONDOLENCE" => s__CONDOLENCE(v),
            "_FIGURES" => s__FIGURES(v),
            "_FRAME" => s__FRAME(v),
            "_MONTHS" => s__MONTHS(v),
            "_NUMBER_WORDS" => s__NUMBER_WORDS(v),
            "_PEOPLE" => s__PEOPLE(v),
            "_REFUSAL_UNDONE" => s__REFUSAL_UNDONE(v),
            "_SAID_BEFORE_WORDS" => s__SAID_BEFORE_WORDS(v),
            "_STEP_WHEN_LOW" => s__STEP_WHEN_LOW(v),
            "_THE_NAMELESS" => s__THE_NAMELESS(v),
            "_THE_ONE_NAMED" => s__THE_ONE_NAMED(v),
            "_ack" => s__ack(v),
            "_already" => s__already(v),
            "_cooking" => s__cooking(v),
            "_dream" => s__dream(v),
            "_harm" => s__harm(v),
            "_history" => s__history(v),
            "_lineback" => s__lineback(v),
            "_loss" => s__loss(v),
            "_naming" => s__naming(v),
            "_reading" => s__reading(v),
            "_recall" => s__recall(v),
            "_repeat" => s__repeat(v),
            "_ruledout" => s__ruledout(v),
            "_there_is_a_chat" => s__there_is_a_chat(v),
            "reads_as_a_detail_of_the_news" => s_reads_as_a_detail_of_the_news(v),
            "reads_as_the_detail_line" => s_reads_as_the_detail_line(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.arbiter", module_obj);
    pyrt::xrefs::reg_fn("modules.arbiter", "choose", py_func("choose", |args: &[Py]| choose(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_past_that_is_not_there", py_func("_a_past_that_is_not_there", |args: &[Py]| _a_past_that_is_not_there(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_an_answer_it_already_gave", py_func("_an_answer_it_already_gave", |args: &[Py]| _an_answer_it_already_gave(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_recall_from_an_earlier_turn", py_func("_a_recall_from_an_earlier_turn", |args: &[Py]| _a_recall_from_an_earlier_turn(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_kind_line_they_gave", py_func("_the_kind_line_they_gave", |args: &[Py]| _the_kind_line_they_gave(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_seed", py_func("_seed", |args: &[Py]| _seed(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_priority", py_func("_priority", |args: &[Py]| _priority(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_food_the_line_refused", py_func("_the_food_the_line_refused", |args: &[Py]| _the_food_the_line_refused(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_offers_the_food", py_func("_offers_the_food", |args: &[Py]| _offers_the_food(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_offers_what_the_line_ruled_out", py_func("_offers_what_the_line_ruled_out", |args: &[Py]| _offers_what_the_line_ruled_out(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_thing_the_line_ruled_out", py_func("_the_thing_the_line_ruled_out", |args: &[Py]| _the_thing_the_line_ruled_out(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_every_real_answer_offers", py_func("_every_real_answer_offers", |args: &[Py]| _every_real_answer_offers(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_cut_the_line_asked_for", py_func("_the_cut_the_line_asked_for", |args: &[Py]| _the_cut_the_line_asked_for(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_figures_they_gave", py_func("_the_figures_they_gave", |args: &[Py]| _the_figures_they_gave(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_says_back_one_of", py_func("_says_back_one_of", |args: &[Py]| _says_back_one_of(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_candidates_with_a_figure", py_func("_the_candidates_with_a_figure", |args: &[Py]| _the_candidates_with_a_figure(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_amends_a_detail", py_func("_amends_a_detail", |args: &[Py]| _amends_a_detail(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_candidates_that_hand_the_detail_back", py_func("_the_candidates_that_hand_the_detail_back", |args: &[Py]| _the_candidates_that_hand_the_detail_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_asks_it_back", py_func("_asks_it_back", |args: &[Py]| _asks_it_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_candidates_that_give_the_shape", py_func("_the_candidates_that_give_the_shape", |args: &[Py]| _the_candidates_that_give_the_shape(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_line_asks_for_a_doing", py_func("_the_line_asks_for_a_doing", |args: &[Py]| _the_line_asks_for_a_doing(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_says_what_they_already_said", py_func("_says_what_they_already_said", |args: &[Py]| _says_what_they_already_said(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_same_word__dup1", py_func("_the_same_word__dup1", |args: &[Py]| _the_same_word__dup1(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_numbers_in", py_func("_the_numbers_in", |args: &[Py]| _the_numbers_in(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_names_the_answers_figure", py_func("_names_the_answers_figure", |args: &[Py]| _names_the_answers_figure(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_explains_the_method", py_func("_explains_the_method", |args: &[Py]| _explains_the_method(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_figure_they_asked_back", py_func("_the_figure_they_asked_back", |args: &[Py]| _the_figure_they_asked_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_words_the_report_names", py_func("_the_words_the_report_names", |args: &[Py]| _the_words_the_report_names(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_report_and_no_ask", py_func("_a_report_and_no_ask", |args: &[Py]| _a_report_and_no_ask(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_is_advice_about", py_func("_is_advice_about", |args: &[Py]| _is_advice_about(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_how_much_it_names", py_func("_how_much_it_names", |args: &[Py]| _how_much_it_names(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_advice_over_a_definition", py_func("_the_advice_over_a_definition", |args: &[Py]| _the_advice_over_a_definition(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_choose", py_func("_choose", |args: &[Py]| _choose(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_bare_doubt_reply", py_func("_a_bare_doubt_reply", |args: &[Py]| _a_bare_doubt_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_places_the_figure", py_func("_places_the_figure", |args: &[Py]| _places_the_figure(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_card_that_places_the_figure", py_func("_the_card_that_places_the_figure", |args: &[Py]| _the_card_that_places_the_figure(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_says_only_the_line", py_func("_says_only_the_line", |args: &[Py]| _says_only_the_line(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_about_the_line", py_func("_about_the_line", |args: &[Py]| _about_the_line(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_names_the_subject", py_func("_names_the_subject", |args: &[Py]| _names_the_subject(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_names_the_thing", py_func("_names_the_thing", |args: &[Py]| _names_the_thing(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_same_plain_word", py_func("_the_same_plain_word", |args: &[Py]| _the_same_plain_word(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_reads_the_doubt", py_func("_reads_the_doubt", |args: &[Py]| _reads_the_doubt(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_bare_yes_no", py_func("_a_bare_yes_no", |args: &[Py]| _a_bare_yes_no(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_carried_words", py_func("_carried_words", |args: &[Py]| _carried_words(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_fit", py_func("_fit", |args: &[Py]| _fit(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_line_topics", py_func("_line_topics", |args: &[Py]| _line_topics(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_an_answer_behind_the_ask", py_func("_an_answer_behind_the_ask", |args: &[Py]| _an_answer_behind_the_ask(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_asks_for_the_answer_again", py_func("_asks_for_the_answer_again", |args: &[Py]| _asks_for_the_answer_again(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_repeat_frame", py_func("_the_repeat_frame", |args: &[Py]| _the_repeat_frame(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_store_reads_a_repeat", py_func("_the_store_reads_a_repeat", |args: &[Py]| _the_store_reads_a_repeat(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_store_reads_an_explain_again_ask", py_func("_the_store_reads_an_explain_again_ask", |args: &[Py]| _the_store_reads_an_explain_again_ask(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_repeat_to_hold", py_func("_a_repeat_to_hold", |args: &[Py]| _a_repeat_to_hold(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_choice_the_answer_did_not_make", py_func("_a_choice_the_answer_did_not_make", |args: &[Py]| _a_choice_the_answer_did_not_make(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_shorter_said_again", py_func("_a_shorter_said_again", |args: &[Py]| _a_shorter_said_again(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_said_it_already", py_func("_said_it_already", |args: &[Py]| _said_it_already(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_ack_lines", py_func("_ack_lines", |args: &[Py]| _ack_lines(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_asks_a_question", py_func("_asks_a_question", |args: &[Py]| _asks_a_question(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_asks_for_a_judgement", py_func("_asks_for_a_judgement", |args: &[Py]| _asks_for_a_judgement(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_hands_the_decision_back", py_func("_hands_the_decision_back", |args: &[Py]| _hands_the_decision_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_reply_is_the_report", py_func("_the_reply_is_the_report", |args: &[Py]| _the_reply_is_the_report(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_real_answers_besides", py_func("_real_answers_besides", |args: &[Py]| _real_answers_besides(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_ask_after_the_facts", py_func("_the_ask_after_the_facts", |args: &[Py]| _the_ask_after_the_facts(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_only_says_the_line_back", py_func("_only_says_the_line_back", |args: &[Py]| _only_says_the_line_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_candidates_that_answer_the_ask", py_func("_the_candidates_that_answer_the_ask", |args: &[Py]| _the_candidates_that_answer_the_ask(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_card_besides", py_func("_a_card_besides", |args: &[Py]| _a_card_besides(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_real_answer_besides", py_func("_a_real_answer_besides", |args: &[Py]| _a_real_answer_besides(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_only_a_hand_back_besides", py_func("_only_a_hand_back_besides", |args: &[Py]| _only_a_hand_back_besides(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_names_what_they_said", py_func("_names_what_they_said", |args: &[Py]| _names_what_they_said(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_one_word", py_func("_one_word", |args: &[Py]| _one_word(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_answer_handed_back", py_func("_the_answer_handed_back", |args: &[Py]| _the_answer_handed_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_names_a_figure", py_func("_names_a_figure", |args: &[Py]| _names_a_figure(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_as_it_comes", py_func("_as_it_comes", |args: &[Py]| _as_it_comes(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_question_about_the_bot", py_func("_a_question_about_the_bot", |args: &[Py]| _a_question_about_the_bot(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_declaring_specialist_besides", py_func("_a_declaring_specialist_besides", |args: &[Py]| _a_declaring_specialist_besides(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_fresh_answer_besides", py_func("_a_fresh_answer_besides", |args: &[Py]| _a_fresh_answer_besides(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_claims_instead", py_func("_claims_instead", |args: &[Py]| _claims_instead(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_declares_claims_instead", py_func("_declares_claims_instead", |args: &[Py]| _declares_claims_instead(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_declares_as_it_comes", py_func("_declares_as_it_comes", |args: &[Py]| _declares_as_it_comes(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_asks_about_the_chat", py_func("_asks_about_the_chat", |args: &[Py]| _asks_about_the_chat(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_move_in_hand", py_func("_the_move_in_hand", |args: &[Py]| _the_move_in_hand(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_whole_move", py_func("_the_whole_move", |args: &[Py]| _the_whole_move(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_made_of_the_answer", py_func("_made_of_the_answer", |args: &[Py]| _made_of_the_answer(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_line_is_news", py_func("_the_line_is_news", |args: &[Py]| _the_line_is_news(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_announces_a_repeat", py_func("_announces_a_repeat", |args: &[Py]| _announces_a_repeat(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_repeat_leads", py_func("_repeat_leads", |args: &[Py]| _repeat_leads(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_carries_on", py_func("_carries_on", |args: &[Py]| _carries_on(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_run_in_the_answer", py_func("_run_in_the_answer", |args: &[Py]| _run_in_the_answer(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_condolence", py_func("_a_condolence", |args: &[Py]| _a_condolence(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_says_the_subject", py_func("_says_the_subject", |args: &[Py]| _says_the_subject(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_cause_asked_for", py_func("_the_cause_asked_for", |args: &[Py]| _the_cause_asked_for(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_date_fact", py_func("_the_date_fact", |args: &[Py]| _the_date_fact(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_next_part", py_func("_the_next_part", |args: &[Py]| _the_next_part(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_lead_line", py_func("_a_lead_line", |args: &[Py]| _a_lead_line(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_only_acknowledges", py_func("_only_acknowledges", |args: &[Py]| _only_acknowledges(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_presence_check", py_func("_a_presence_check", |args: &[Py]| _a_presence_check(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_about_the_bot", py_func("_about_the_bot", |args: &[Py]| _about_the_bot(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_venue_hours_turn", py_func("_the_venue_hours_turn", |args: &[Py]| _the_venue_hours_turn(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_bots_own_taste", py_func("_the_bots_own_taste", |args: &[Py]| _the_bots_own_taste(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_my_workings", py_func("_my_workings", |args: &[Py]| _my_workings(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_declares_topics", py_func("_declares_topics", |args: &[Py]| _declares_topics(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_reads_as_a_verdict", py_func("_reads_as_a_verdict", |args: &[Py]| _reads_as_a_verdict(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_verdict_lines", py_func("_verdict_lines", |args: &[Py]| _verdict_lines(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_verdict_turn", py_func("_the_verdict_turn", |args: &[Py]| _the_verdict_turn(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_in_a_careful_thread", py_func("_in_a_careful_thread", |args: &[Py]| _in_a_careful_thread(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_subject_words", py_func("_subject_words", |args: &[Py]| _subject_words(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_about_the_news", py_func("_about_the_news", |args: &[Py]| _about_the_news(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_says_the_same_thing_again", py_func("_says_the_same_thing_again", |args: &[Py]| _says_the_same_thing_again(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_subject_was_answered", py_func("_the_subject_was_answered", |args: &[Py]| _the_subject_was_answered(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_verdict_back", py_func("_the_verdict_back", |args: &[Py]| _the_verdict_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_verdict_reply", py_func("_verdict_reply", |args: &[Py]| _verdict_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_instead", py_func("_instead", |args: &[Py]| _instead(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_ask_clause", py_func("_the_ask_clause", |args: &[Py]| _the_ask_clause(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_entry_that_answers_the_ask", py_func("_the_entry_that_answers_the_ask", |args: &[Py]| _the_entry_that_answers_the_ask(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_defers", py_func("_defers", |args: &[Py]| _defers(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_defers_to", py_func("_defers_to", |args: &[Py]| _defers_to(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_declares_instead", py_func("_declares_instead", |args: &[Py]| _declares_instead(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_pause_reply", py_func("_pause_reply", |args: &[Py]| _pause_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_leaving_reply", py_func("_leaving_reply", |args: &[Py]| _leaving_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_asks", py_func("_asks", |args: &[Py]| _asks(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_sign_off", py_func("_a_sign_off", |args: &[Py]| _a_sign_off(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_declared", py_func("_declared", |args: &[Py]| _declared(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_store_revises_the_answer", py_func("_the_store_revises_the_answer", |args: &[Py]| _the_store_revises_the_answer(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_clause_was_cut_in_half", py_func("_a_clause_was_cut_in_half", |args: &[Py]| _a_clause_was_cut_in_half(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_store_correcting", py_func("_store_correcting", |args: &[Py]| _store_correcting(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_draft_index", py_func("_draft_index", |args: &[Py]| _draft_index(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_leaving_said_plain", py_func("_a_leaving_said_plain", |args: &[Py]| _a_leaving_said_plain(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_sudden_loss_reply", py_func("_sudden_loss_reply", |args: &[Py]| _sudden_loss_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_words_for_the_child", py_func("_the_words_for_the_child", |args: &[Py]| _the_words_for_the_child(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_whole_line_module_spoke", py_func("_a_whole_line_module_spoke", |args: &[Py]| _a_whole_line_module_spoke(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_words_for_a_child_pool", py_func("_the_words_for_a_child_pool", |args: &[Py]| _the_words_for_a_child_pool(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_ask_about_a_child", py_func("_the_ask_about_a_child", |args: &[Py]| _the_ask_about_a_child(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_name_they_gave", py_func("_the_name_they_gave", |args: &[Py]| _the_name_they_gave(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_about_the_person_the_line_names", py_func("_about_the_person_the_line_names", |args: &[Py]| _about_the_person_the_line_names(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_knowledge_entry_has_it", py_func("_a_knowledge_entry_has_it", |args: &[Py]| _a_knowledge_entry_has_it(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_hurting_reply", py_func("_hurting_reply", |args: &[Py]| _hurting_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_story_goes_on_reply", py_func("_story_goes_on_reply", |args: &[Py]| _story_goes_on_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_story_opened", py_func("_a_story_opened", |args: &[Py]| _a_story_opened(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_refusal", py_func("_a_refusal", |args: &[Py]| _a_refusal(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_is_loss_reply_in", py_func("_is_loss_reply_in", |args: &[Py]| _is_loss_reply_in(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_composer_reading", py_func("_composer_reading", |args: &[Py]| _composer_reading(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_carries_the_answer", py_func("_carries_the_answer", |args: &[Py]| _carries_the_answer(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_check_on_the_answer", py_func("_a_check_on_the_answer", |args: &[Py]| _a_check_on_the_answer(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_opens_on_a_job", py_func("_opens_on_a_job", |args: &[Py]| _opens_on_a_job(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_is_a_generic_step", py_func("_is_a_generic_step", |args: &[Py]| _is_a_generic_step(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_named_topics", py_func("_named_topics", |args: &[Py]| _named_topics(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_thread_index", py_func("_thread_index", |args: &[Py]| _thread_index(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_answers_this_line", py_func("_answers_this_line", |args: &[Py]| _answers_this_line(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_content", py_func("_content", |args: &[Py]| _content(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_echoes", py_func("_echoes", |args: &[Py]| _echoes(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_answer_again_with_a_tail", py_func("_the_answer_again_with_a_tail", |args: &[Py]| _the_answer_again_with_a_tail(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_quoted_repeat", py_func("_the_quoted_repeat", |args: &[Py]| _the_quoted_repeat(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_points_back", py_func("_points_back", |args: &[Py]| _points_back(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_run_of", py_func("_a_run_of", |args: &[Py]| _a_run_of(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_answer_written_out", py_func("_the_answer_written_out", |args: &[Py]| _the_answer_written_out(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_last_reply", py_func("_last_reply", |args: &[Py]| _last_reply(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_said_before", py_func("_said_before", |args: &[Py]| _said_before(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_about", py_func("_about", |args: &[Py]| _about(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_strings", py_func("_strings", |args: &[Py]| _strings(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_label", py_func("_label", |args: &[Py]| _label(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_labels", py_func("_labels", |args: &[Py]| _labels(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_stored", py_func("_stored", |args: &[Py]| _stored(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_read_topic", py_func("_read_topic", |args: &[Py]| _read_topic(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_answered", py_func("_answered", |args: &[Py]| _answered(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_match", py_func("_match", |args: &[Py]| _match(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_subject_words", py_func("_the_subject_words", |args: &[Py]| _the_subject_words(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_lines_own_words", py_func("_the_lines_own_words", |args: &[Py]| _the_lines_own_words(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_forms_of", py_func("_the_forms_of", |args: &[Py]| _the_forms_of(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_same_word", py_func("_the_same_word", |args: &[Py]| _the_same_word(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_names_a_word_of", py_func("_names_a_word_of", |args: &[Py]| _names_a_word_of(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_card_names_the_line", py_func("_a_card_names_the_line", |args: &[Py]| _a_card_names_the_line(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_detail_added_to_the_news", py_func("_a_detail_added_to_the_news", |args: &[Py]| _a_detail_added_to_the_news(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_shares_a_stem", py_func("_shares_a_stem", |args: &[Py]| _shares_a_stem(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_word_the_line_added", py_func("_the_word_the_line_added", |args: &[Py]| _the_word_the_line_added(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_says_nothing_new", py_func("_says_nothing_new", |args: &[Py]| _says_nothing_new(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_names_the_word", py_func("_names_the_word", |args: &[Py]| _names_the_word(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_the_answer_that_names_what_the_line_added", py_func("_the_answer_that_names_what_the_line_added", |args: &[Py]| _the_answer_that_names_what_the_line_added(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_card_that_names_the_line", py_func("_a_card_that_names_the_line", |args: &[Py]| _a_card_that_names_the_line(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_line_asking_for_plainer_words", py_func("_a_line_asking_for_plainer_words", |args: &[Py]| _a_line_asking_for_plainer_words(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_says_the_answer_again", py_func("_says_the_answer_again", |args: &[Py]| _says_the_answer_again(args)));
    pyrt::xrefs::reg_fn("modules.arbiter", "_a_plainer_reading_of_the_same_answer", py_func("_a_plainer_reading_of_the_same_answer", |args: &[Py]| _a_plainer_reading_of_the_same_answer(args)));
    pyrt::xrefs::reg_access("modules.arbiter", "ACK_POOL", g_ACK_POOL, s_ACK_POOL);
    pyrt::xrefs::reg_access("modules.arbiter", "AGAIN_FIRST_WORDS", g_AGAIN_FIRST_WORDS, s_AGAIN_FIRST_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "AGAIN_TAIL_WORDS", g_AGAIN_TAIL_WORDS, s_AGAIN_TAIL_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "ANSWERS_THE_NEWS", g_ANSWERS_THE_NEWS, s_ANSWERS_THE_NEWS);
    pyrt::xrefs::reg_access("modules.arbiter", "ASKS_FOR_A_THING", g_ASKS_FOR_A_THING, s_ASKS_FOR_A_THING);
    pyrt::xrefs::reg_access("modules.arbiter", "ASK_CLAUSE_SPLIT", g_ASK_CLAUSE_SPLIT, s_ASK_CLAUSE_SPLIT);
    pyrt::xrefs::reg_access("modules.arbiter", "A_CEILING", g_A_CEILING, s_A_CEILING);
    pyrt::xrefs::reg_access("modules.arbiter", "A_CONTENT_WORD", g_A_CONTENT_WORD, s_A_CONTENT_WORD);
    pyrt::xrefs::reg_access("modules.arbiter", "A_DEFINITION", g_A_DEFINITION, s_A_DEFINITION);
    pyrt::xrefs::reg_access("modules.arbiter", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("modules.arbiter", "A_MEASURE_ASKED", g_A_MEASURE_ASKED, s_A_MEASURE_ASKED);
    pyrt::xrefs::reg_access("modules.arbiter", "A_MY_OWN_REPORT", g_A_MY_OWN_REPORT, s_A_MY_OWN_REPORT);
    pyrt::xrefs::reg_access("modules.arbiter", "A_PLAINER_ASK", g_A_PLAINER_ASK, s_A_PLAINER_ASK);
    pyrt::xrefs::reg_access("modules.arbiter", "A_POINTS_BACK", g_A_POINTS_BACK, s_A_POINTS_BACK);
    pyrt::xrefs::reg_access("modules.arbiter", "A_REPORT_ASKS", g_A_REPORT_ASKS, s_A_REPORT_ASKS);
    pyrt::xrefs::reg_access("modules.arbiter", "A_REPORT_NOD", g_A_REPORT_NOD, s_A_REPORT_NOD);
    pyrt::xrefs::reg_access("modules.arbiter", "A_SUBJECT_PHRASE", g_A_SUBJECT_PHRASE, s_A_SUBJECT_PHRASE);
    pyrt::xrefs::reg_access("modules.arbiter", "A_SUBJECT_WORD", g_A_SUBJECT_WORD, s_A_SUBJECT_WORD);
    pyrt::xrefs::reg_access("modules.arbiter", "A_SUM_TO_WORK_OUT", g_A_SUM_TO_WORK_OUT, s_A_SUM_TO_WORK_OUT);
    pyrt::xrefs::reg_access("modules.arbiter", "A_VERDICT_BACK", g_A_VERDICT_BACK, s_A_VERDICT_BACK);
    pyrt::xrefs::reg_access("modules.arbiter", "A_WORKED_OUT", g_A_WORKED_OUT, s_A_WORKED_OUT);
    pyrt::xrefs::reg_access("modules.arbiter", "CARE_THREAD", g_CARE_THREAD, s_CARE_THREAD);
    pyrt::xrefs::reg_access("modules.arbiter", "CARRIES_NO_SUBJECT", g_CARRIES_NO_SUBJECT, s_CARRIES_NO_SUBJECT);
    pyrt::xrefs::reg_access("modules.arbiter", "CHATTER", g_CHATTER, s_CHATTER);
    pyrt::xrefs::reg_access("modules.arbiter", "CHITCHAT", g_CHITCHAT, s_CHITCHAT);
    pyrt::xrefs::reg_access("modules.arbiter", "CHOICE", g_CHOICE, s_CHOICE);
    pyrt::xrefs::reg_access("modules.arbiter", "CUT_MARKS", g_CUT_MARKS, s_CUT_MARKS);
    pyrt::xrefs::reg_access("modules.arbiter", "DECLARED_ABOUT_THE_CHAT", g_DECLARED_ABOUT_THE_CHAT, s_DECLARED_ABOUT_THE_CHAT);
    pyrt::xrefs::reg_access("modules.arbiter", "DECLARED_AS_IT_COMES", g_DECLARED_AS_IT_COMES, s_DECLARED_AS_IT_COMES);
    pyrt::xrefs::reg_access("modules.arbiter", "DECLARED_CLAIMS_INSTEAD", g_DECLARED_CLAIMS_INSTEAD, s_DECLARED_CLAIMS_INSTEAD);
    pyrt::xrefs::reg_access("modules.arbiter", "DECLARED_DEFERS", g_DECLARED_DEFERS, s_DECLARED_DEFERS);
    pyrt::xrefs::reg_access("modules.arbiter", "DECLARED_INSTEAD", g_DECLARED_INSTEAD, s_DECLARED_INSTEAD);
    pyrt::xrefs::reg_access("modules.arbiter", "DECLARED_TOPICS", g_DECLARED_TOPICS, s_DECLARED_TOPICS);
    pyrt::xrefs::reg_access("modules.arbiter", "DOING_WANTS", g_DOING_WANTS, s_DOING_WANTS);
    pyrt::xrefs::reg_access("modules.arbiter", "DOUBT_WORDS", g_DOUBT_WORDS, s_DOUBT_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "ECHO_STOP", g_ECHO_STOP, s_ECHO_STOP);
    pyrt::xrefs::reg_access("modules.arbiter", "FIRST_VERB", g_FIRST_VERB, s_FIRST_VERB);
    pyrt::xrefs::reg_access("modules.arbiter", "GENERIC", g_GENERIC, s_GENERIC);
    pyrt::xrefs::reg_access("modules.arbiter", "INFLECTED_ENDS", g_INFLECTED_ENDS, s_INFLECTED_ENDS);
    pyrt::xrefs::reg_access("modules.arbiter", "KEEPS_ITS_OWN_ANSWER", g_KEEPS_ITS_OWN_ANSWER, s_KEEPS_ITS_OWN_ANSWER);
    pyrt::xrefs::reg_access("modules.arbiter", "LABEL_KEYS", g_LABEL_KEYS, s_LABEL_KEYS);
    pyrt::xrefs::reg_access("modules.arbiter", "LETTERS", g_LETTERS, s_LETTERS);
    pyrt::xrefs::reg_access("modules.arbiter", "LONG_ENOUGH", g_LONG_ENOUGH, s_LONG_ENOUGH);
    pyrt::xrefs::reg_access("modules.arbiter", "MEMORY_MODULES", g_MEMORY_MODULES, s_MEMORY_MODULES);
    pyrt::xrefs::reg_access("modules.arbiter", "MOVE_ANSWER_WORDS", g_MOVE_ANSWER_WORDS, s_MOVE_ANSWER_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "MOVE_SHARE", g_MOVE_SHARE, s_MOVE_SHARE);
    pyrt::xrefs::reg_access("modules.arbiter", "MOVE_WHOLE", g_MOVE_WHOLE, s_MOVE_WHOLE);
    pyrt::xrefs::reg_access("modules.arbiter", "MOVE_WORDS", g_MOVE_WORDS, s_MOVE_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "NOT_A_NEW_WORD", g_NOT_A_NEW_WORD, s_NOT_A_NEW_WORD);
    pyrt::xrefs::reg_access("modules.arbiter", "NOT_A_SUBJECT", g_NOT_A_SUBJECT, s_NOT_A_SUBJECT);
    pyrt::xrefs::reg_access("modules.arbiter", "NOT_A_SUBJECT_WORD", g_NOT_A_SUBJECT_WORD, s_NOT_A_SUBJECT_WORD);
    pyrt::xrefs::reg_access("modules.arbiter", "OWN_ABOUT_THE_CHAT", g_OWN_ABOUT_THE_CHAT, s_OWN_ABOUT_THE_CHAT);
    pyrt::xrefs::reg_access("modules.arbiter", "PAUSE", g_PAUSE, s_PAUSE);
    pyrt::xrefs::reg_access("modules.arbiter", "PLAIN_ASK_FILLER", g_PLAIN_ASK_FILLER, s_PLAIN_ASK_FILLER);
    pyrt::xrefs::reg_access("modules.arbiter", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("modules.arbiter", "POOL_MODULES", g_POOL_MODULES, s_POOL_MODULES);
    pyrt::xrefs::reg_access("modules.arbiter", "READ", g_READ, s_READ);
    pyrt::xrefs::reg_access("modules.arbiter", "RECALL", g_RECALL, s_RECALL);
    pyrt::xrefs::reg_access("modules.arbiter", "REFUSAL", g_REFUSAL, s_REFUSAL);
    pyrt::xrefs::reg_access("modules.arbiter", "REPEAT_FRAME", g_REPEAT_FRAME, s_REPEAT_FRAME);
    pyrt::xrefs::reg_access("modules.arbiter", "REPORT_STOP", g_REPORT_STOP, s_REPORT_STOP);
    pyrt::xrefs::reg_access("modules.arbiter", "SAFETY", g_SAFETY, s_SAFETY);
    pyrt::xrefs::reg_access("modules.arbiter", "SAID_BACK", g_SAID_BACK, s_SAID_BACK);
    pyrt::xrefs::reg_access("modules.arbiter", "THEIR_VOICE", g_THEIR_VOICE, s_THEIR_VOICE);
    pyrt::xrefs::reg_access("modules.arbiter", "THE_ANSWER_BANDS", g_THE_ANSWER_BANDS, s_THE_ANSWER_BANDS);
    pyrt::xrefs::reg_access("modules.arbiter", "THE_ASK_OPENS", g_THE_ASK_OPENS, s_THE_ASK_OPENS);
    pyrt::xrefs::reg_access("modules.arbiter", "THE_CAUSE", g_THE_CAUSE, s_THE_CAUSE);
    pyrt::xrefs::reg_access("modules.arbiter", "THREAD", g_THREAD, s_THREAD);
    pyrt::xrefs::reg_access("modules.arbiter", "TOKEN", g_TOKEN, s_TOKEN);
    pyrt::xrefs::reg_access("modules.arbiter", "WHOLE_LINE_BAND", g_WHOLE_LINE_BAND, s_WHOLE_LINE_BAND);
    pyrt::xrefs::reg_access("modules.arbiter", "WHOLE_MOVE_WORDS", g_WHOLE_MOVE_WORDS, s_WHOLE_MOVE_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("modules.arbiter", "WORD_ENDINGS", g_WORD_ENDINGS, s_WORD_ENDINGS);
    pyrt::xrefs::reg_access("modules.arbiter", "_ABOUT_SOMEONE_ELSE", g__ABOUT_SOMEONE_ELSE, s__ABOUT_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("modules.arbiter", "_ALLERGY_LIFTED", g__ALLERGY_LIFTED, s__ALLERGY_LIFTED);
    pyrt::xrefs::reg_access("modules.arbiter", "_AN_AMENDMENT", g__AN_AMENDMENT, s__AN_AMENDMENT);
    pyrt::xrefs::reg_access("modules.arbiter", "_A_HOW_TO", g__A_HOW_TO, s__A_HOW_TO);
    pyrt::xrefs::reg_access("modules.arbiter", "_A_PERSON_NAMED", g__A_PERSON_NAMED, s__A_PERSON_NAMED);
    pyrt::xrefs::reg_access("modules.arbiter", "_CONDOLENCE", g__CONDOLENCE, s__CONDOLENCE);
    pyrt::xrefs::reg_access("modules.arbiter", "_FIGURES", g__FIGURES, s__FIGURES);
    pyrt::xrefs::reg_access("modules.arbiter", "_FRAME", g__FRAME, s__FRAME);
    pyrt::xrefs::reg_access("modules.arbiter", "_MONTHS", g__MONTHS, s__MONTHS);
    pyrt::xrefs::reg_access("modules.arbiter", "_NUMBER_WORDS", g__NUMBER_WORDS, s__NUMBER_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "_PEOPLE", g__PEOPLE, s__PEOPLE);
    pyrt::xrefs::reg_access("modules.arbiter", "_REFUSAL_UNDONE", g__REFUSAL_UNDONE, s__REFUSAL_UNDONE);
    pyrt::xrefs::reg_access("modules.arbiter", "_SAID_BEFORE_WORDS", g__SAID_BEFORE_WORDS, s__SAID_BEFORE_WORDS);
    pyrt::xrefs::reg_access("modules.arbiter", "_STEP_WHEN_LOW", g__STEP_WHEN_LOW, s__STEP_WHEN_LOW);
    pyrt::xrefs::reg_access("modules.arbiter", "_THE_NAMELESS", g__THE_NAMELESS, s__THE_NAMELESS);
    pyrt::xrefs::reg_access("modules.arbiter", "_THE_ONE_NAMED", g__THE_ONE_NAMED, s__THE_ONE_NAMED);
    pyrt::xrefs::reg_access("modules.arbiter", "_ack", g__ack, s__ack);
    pyrt::xrefs::reg_access("modules.arbiter", "_already", g__already, s__already);
    pyrt::xrefs::reg_access("modules.arbiter", "_cooking", g__cooking, s__cooking);
    pyrt::xrefs::reg_access("modules.arbiter", "_dream", g__dream, s__dream);
    pyrt::xrefs::reg_access("modules.arbiter", "_harm", g__harm, s__harm);
    pyrt::xrefs::reg_access("modules.arbiter", "_history", g__history, s__history);
    pyrt::xrefs::reg_access("modules.arbiter", "_lineback", g__lineback, s__lineback);
    pyrt::xrefs::reg_access("modules.arbiter", "_loss", g__loss, s__loss);
    pyrt::xrefs::reg_access("modules.arbiter", "_naming", g__naming, s__naming);
    pyrt::xrefs::reg_access("modules.arbiter", "_reading", g__reading, s__reading);
    pyrt::xrefs::reg_access("modules.arbiter", "_recall", g__recall, s__recall);
    pyrt::xrefs::reg_access("modules.arbiter", "_repeat", g__repeat, s__repeat);
    pyrt::xrefs::reg_access("modules.arbiter", "_ruledout", g__ruledout, s__ruledout);
    pyrt::xrefs::reg_access("modules.arbiter", "_there_is_a_chat", g__there_is_a_chat, s__there_is_a_chat);
    pyrt::xrefs::reg_access("modules.arbiter", "reads_as_a_detail_of_the_news", g_reads_as_a_detail_of_the_news, s_reads_as_a_detail_of_the_news);
    pyrt::xrefs::reg_access("modules.arbiter", "reads_as_the_detail_line", g_reads_as_the_detail_line, s_reads_as_the_detail_line);
}
