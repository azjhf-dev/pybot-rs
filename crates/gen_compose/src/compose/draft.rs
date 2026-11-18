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


thread_local! { static G_ABSENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_ILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGREEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOLOGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOLOGY_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOLOGY_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WISH_FOR_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAPITALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CREATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURTS_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT_TARGET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT_WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_INS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MESSAGE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MESSAGE_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MESSAGE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_YET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MESSAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTED_SPEECH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTS_ON_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHAPE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_EXAMPLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELL_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_CONTENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_GONE_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIDY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VENT_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_MORE_OF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_THEM_DEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANT_FROM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRITING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABSENT() -> Py { ensure_init(); G_ABSENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABSENT(v: Py) { ensure_init(); G_ABSENT.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_ILL() -> Py { ensure_init(); G_AFTER_ILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_ILL(v: Py) { ensure_init(); G_AFTER_ILL.with(|c| *c.borrow_mut() = v); }
pub fn g_AGREEMENT() -> Py { ensure_init(); G_AGREEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREEMENT(v: Py) { ensure_init(); G_AGREEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_APOLOGY() -> Py { ensure_init(); G_APOLOGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOLOGY(v: Py) { ensure_init(); G_APOLOGY.with(|c| *c.borrow_mut() = v); }
pub fn g_APOLOGY_ACT() -> Py { ensure_init(); G_APOLOGY_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOLOGY_ACT(v: Py) { ensure_init(); G_APOLOGY_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g_APOLOGY_SELF() -> Py { ensure_init(); G_APOLOGY_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOLOGY_SELF(v: Py) { ensure_init(); G_APOLOGY_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SPAN() -> Py { ensure_init(); G_A_SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SPAN(v: Py) { ensure_init(); G_A_SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WISH_FOR_SOMEONE() -> Py { ensure_init(); G_A_WISH_FOR_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WISH_FOR_SOMEONE(v: Py) { ensure_init(); G_A_WISH_FOR_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_LEAD() -> Py { ensure_init(); G_BARE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_LEAD(v: Py) { ensure_init(); G_BARE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_CAPITALS() -> Py { ensure_init(); G_CAPITALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAPITALS(v: Py) { ensure_init(); G_CAPITALS.with(|c| *c.borrow_mut() = v); }
pub fn g_COPULA() -> Py { ensure_init(); G_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); G_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_CREATIVE() -> Py { ensure_init(); G_CREATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CREATIVE(v: Py) { ensure_init(); G_CREATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING() -> Py { ensure_init(); G_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING(v: Py) { ensure_init(); G_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING_PHRASES() -> Py { ensure_init(); G_FEELING_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING_PHRASES(v: Py) { ensure_init(); G_FEELING_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_FORMAL() -> Py { ensure_init(); G_FORMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORMAL(v: Py) { ensure_init(); G_FORMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_HURTS_SOMEONE() -> Py { ensure_init(); G_HURTS_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURTS_SOMEONE(v: Py) { ensure_init(); G_HURTS_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT_TARGET() -> Py { ensure_init(); G_HURT_TARGET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT_TARGET(v: Py) { ensure_init(); G_HURT_TARGET.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT_WISH() -> Py { ensure_init(); G_HURT_WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT_WISH(v: Py) { ensure_init(); G_HURT_WISH.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT_WORD() -> Py { ensure_init(); G_HURT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT_WORD(v: Py) { ensure_init(); G_HURT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS() -> Py { ensure_init(); G_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); G_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_INS() -> Py { ensure_init(); G_LEAD_INS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_INS(v: Py) { ensure_init(); G_LEAD_INS.with(|c| *c.borrow_mut() = v); }
pub fn g_MESSAGE_NOUN() -> Py { ensure_init(); G_MESSAGE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MESSAGE_NOUN(v: Py) { ensure_init(); G_MESSAGE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_MESSAGE_SHAPES() -> Py { ensure_init(); G_MESSAGE_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MESSAGE_SHAPES(v: Py) { ensure_init(); G_MESSAGE_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_MESSAGE_WORD() -> Py { ensure_init(); G_MESSAGE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MESSAGE_WORD(v: Py) { ensure_init(); G_MESSAGE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS() -> Py { ensure_init(); G_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS(v: Py) { ensure_init(); G_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_PHRASES() -> Py { ensure_init(); G_NEWS_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_PHRASES(v: Py) { ensure_init(); G_NEWS_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_CHANGE() -> Py { ensure_init(); G_NOT_A_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CHANGE(v: Py) { ensure_init(); G_NOT_A_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_YET() -> Py { ensure_init(); G_NOT_YET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_YET(v: Py) { ensure_init(); G_NOT_YET.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MESSAGE() -> Py { ensure_init(); G_NO_MESSAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MESSAGE(v: Py) { ensure_init(); G_NO_MESSAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFER() -> Py { ensure_init(); G_OFFER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFER(v: Py) { ensure_init(); G_OFFER.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION() -> Py { ensure_init(); G_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); G_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTED() -> Py { ensure_init(); G_QUOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTED(v: Py) { ensure_init(); G_QUOTED.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTED_SPEECH() -> Py { ensure_init(); G_REPORTED_SPEECH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTED_SPEECH(v: Py) { ensure_init(); G_REPORTED_SPEECH.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTS_ON_SOMEONE() -> Py { ensure_init(); G_REPORTS_ON_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTS_ON_SOMEONE(v: Py) { ensure_init(); G_REPORTS_ON_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHAPE_CUES() -> Py { ensure_init(); G_SHAPE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHAPE_CUES(v: Py) { ensure_init(); G_SHAPE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_SORRY() -> Py { ensure_init(); G_SORRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORRY(v: Py) { ensure_init(); G_SORRY.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT() -> Py { ensure_init(); G_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT(v: Py) { ensure_init(); G_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_EXAMPLES() -> Py { ensure_init(); G_SUBJECT_EXAMPLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_EXAMPLES(v: Py) { ensure_init(); G_SUBJECT_EXAMPLES.with(|c| *c.borrow_mut() = v); }
pub fn g_TELL_FRAME() -> Py { ensure_init(); G_TELL_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELL_FRAME(v: Py) { ensure_init(); G_TELL_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_CONTENT() -> Py { ensure_init(); G_THE_CONTENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_CONTENT(v: Py) { ensure_init(); G_THE_CONTENT.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_GONE_WRONG() -> Py { ensure_init(); G_THING_GONE_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_GONE_WRONG(v: Py) { ensure_init(); G_THING_GONE_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PERSON() -> Py { ensure_init(); G_THIRD_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PERSON(v: Py) { ensure_init(); G_THIRD_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_TIDY() -> Py { ensure_init(); G_TIDY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIDY(v: Py) { ensure_init(); G_TIDY.with(|c| *c.borrow_mut() = v); }
pub fn g_VENT() -> Py { ensure_init(); G_VENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VENT(v: Py) { ensure_init(); G_VENT.with(|c| *c.borrow_mut() = v); }
pub fn g_VENT_PHRASES() -> Py { ensure_init(); G_VENT_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VENT_PHRASES(v: Py) { ensure_init(); G_VENT_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_MORE_OF_IT() -> Py { ensure_init(); G_WANTS_MORE_OF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_MORE_OF_IT(v: Py) { ensure_init(); G_WANTS_MORE_OF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_THEM_DEAD() -> Py { ensure_init(); G_WANTS_THEM_DEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_THEM_DEAD(v: Py) { ensure_init(); G_WANTS_THEM_DEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_WANT_FROM() -> Py { ensure_init(); G_WANT_FROM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANT_FROM(v: Py) { ensure_init(); G_WANT_FROM.with(|c| *c.borrow_mut() = v); }
pub fn g_WRITING_ASK() -> Py { ensure_init(); G_WRITING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRITING_ASK(v: Py) { ensure_init(); G_WRITING_ASK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The bot offered to draft a message; the next line gives it what it asked for.\n\nA line like \"can you help me write a text to my boss\" is answered, honestly, with an offer:\n\"give me who it's for and the one thing you need from them, and I'll write something you can\nsend.\" On main the next turn - \"saying im off sick tomorrow\" - was echoed back (\"Got it - you're\noff sick tomorrow\") or met with \"That sounds rough. If you want, I can help you think through\nwhat to do next\". The offer promised a draft and then went missing, which is the worst kind of\nreply: a person who was told to hand over the details and did, and got nothing.\n\nSo this answers exactly one case: the bot's own last reply offered to draft, and the line just\nsaid what to say. It hands back a short message with the person's own content in it.\n\nWhat it refuses, because the offer is not an invitation to write anything:\n\n* a line that is a question, not the content (\"what do you think?\"), or a hello, or a thanks;\n* a line with nothing to put in a message (\"ok\", \"yes\", \"that's it\"), which is two words or\n  fewer once the lead-in is gone;\n* a line so long that no short message could carry it (over 30 words), where the honest reply is\n  a question rather than a draft that throws half of it away;\n* a line whose content is creative work: a poem, a speech or a joke about a specific person is\n  not a message to send, and the bot cannot write one.\n\nThe register comes from who it is for: a boss, a landlord, a school or a surgery gets a formal\nmessage, anything else the friendly one. A draft that has to apologise (\"off sick\", \"can't make\nSaturday\") says so in the first line, because that is the part people get stuck on.\n");
    s_OFFER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi can draft that\\b|\\bi can write it\\b|\\bi'll draft it\\b|\\bi'll write something you can send\\b|\\bgive me the gist and i'll draft it\\b|\\bi(?:'ll|\\s+(?:can|could|will|would)) write (?:it|yours|one|that)\\b|\\bwrite (?:it|yours|one) for you\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FORMAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("boss"), py_str("manager"), py_str("employer"), py_str("company"), py_str("hr"), py_str("work"), py_str("workplace"), py_str("office"), py_str("landlord"), py_str("landlady"), py_str("letting"), py_str("agent"), py_str("lettings"), py_str("housing"), py_str("council"), py_str("school"), py_str("head"), py_str("headteacher"), py_str("teacher"), py_str("tutor"), py_str("nursery"), py_str("childminder"), py_str("university"), py_str("college"), py_str("lecturer"), py_str("professor"), py_str("surgery"), py_str("gp"), py_str("doctor"), py_str("dentist"), py_str("hospital"), py_str("clinic"), py_str("bank"), py_str("insurer"), py_str("insurance"), py_str("utility"), py_str("energy"), py_str("council"), py_str("department"), py_str("tenant"), py_str("customer"), py_str("client"), py_str("supplier"), py_str("landlord's")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEADS(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|well\\s+|right\\s+)?(?:it'?s\\s+|its\\s+)?(?:saying|say|to\\s+say|i\\s+was\\s+going\\s+to\\s+say)\\s+(?:that\\s+)?(?:i\\s+)?(?:am\\s+|was\\s+)?(?=\\w)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("said")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|right\\s+)?(?:it'?s\\s+|its\\s+)?(?:asking|ask|to\\s+ask)\\s+(?:for|about)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("asked")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|right\\s+)?(?:it'?s\\s+|its\\s+)?(?:to\\s+say|saying|the\\s+message\\s+is)\\s+(?:that\\s+)?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("said")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|well\\s+|right\\s+)?(?:telling|told)\\s+(?:them|him|her)\\s+(?:that\\s+)?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("told")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|well\\s+|right\\s+)?(?:i\\s+(?:just\\s+)?(?:want|need|would\\s+like)\\s+to\\s+say|i(?:'d|\\s+would)\\s+like\\s+to\\s+say|let\\s+(?:them|him|her)\\s+know|tell\\s+(?:them|him|her))\\s+(?:that\\s+)?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("said")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|right\\s+)?(?:that\\s+|it'?s\\s+)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("said")])]));
    s_TIDY(py_dict(vec![(py_str("im"), py_str("I'm")), (py_str("i"), py_str("I")), (py_str("ive"), py_str("I've")), (py_str("id"), py_str("I'd")), (py_str("cant"), py_str("can't")), (py_str("wont"), py_str("won't")), (py_str("dont"), py_str("don't")), (py_str("doesnt"), py_str("doesn't")), (py_str("didnt"), py_str("didn't")), (py_str("isnt"), py_str("isn't")), (py_str("wasnt"), py_str("wasn't")), (py_str("couldnt"), py_str("couldn't")), (py_str("wouldnt"), py_str("wouldn't")), (py_str("shouldnt"), py_str("shouldn't")), (py_str("havent"), py_str("haven't")), (py_str("thats"), py_str("that's")), (py_str("theres"), py_str("there's")), (py_str("whats"), py_str("what's")), (py_str("lets"), py_str("let's")), (py_str("pls"), py_str("please")), (py_str("plz"), py_str("please")), (py_str("thx"), py_str("thanks")), (py_str("tmrw"), py_str("tomorrow")), (py_str("tomoz"), py_str("tomorrow")), (py_str("tomorow"), py_str("tomorrow")), (py_str("tommorow"), py_str("tomorrow")), (py_str("u"), py_str("you")), (py_str("ur"), py_str("your")), (py_str("n"), py_str("and")), (py_str("cos"), py_str("because")), (py_str("coz"), py_str("because")), (py_str("appt"), py_str("appointment")), (py_str("appointment's"), py_str("appointment's")), (py_str("meds"), py_str("medication")), (py_str("i'll"), py_str("I'll"))]));
    s_ABSENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:off\\s+(?:sick|ill|work)|sick\\s+(?:leave|note|pay)|day\\s+off|time\\s+off|not\\s+coming\\s+in|won'?t\\s+be\\s+in|cant\\s+come\\s+in|can'?t\\s+come\\s+in)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THIRD_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:her|him|them|his|hers|they|she|he)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APOLOGY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:i'?m\\s+|i\\s+am\\s+)?sorry\\b|^apolog(?:y|ise|ize)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APOLOGY_ACT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bsorry\\s+(?:for|about)\\s+(?:being\\s+)?([a-z']+ing)\\b|\\bsorry\\s+(?:that\\s+)?(?:i|we)\\s+([a-z']+)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APOLOGY_SELF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bsorry\\s+(?:for|about)\\s+(?:being\\s+)?(?:such\\s+)?(?:a\\s+|an\\s+)?(?:mess|burden|nightmare|nuisance|pain|disaster|state|rubbish|useless|awful|horrible|annoying|needy|too much|like this|this way|me|myself|the way i am)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SORRY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sick|ill|unwell|off\\s+work|not\\s+(?:coming|in|be\\s+in)|won'?t\\s+be\\s+(?:in|there)|can'?t\\s+(?:come|make|be\\s+(?:in|there))|cant\\s+come|cannot\\s+come|late|miss(?:ing)?|day\\s+off|time\\s+off|cancel(?:ling)?|postpone|reschedule|sorry)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:won'?t|can'?t|cannot|should\\s+be|will\\s+be|might\\s+be|not\\s+be|going\\s+to|need|want|wonder|hope|must|would\\s+like|would|didn'?t|don'?t|am|was|were|have|had|keep|late\\b|stuck\\b|away\\b)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COPULA({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:off\\b|sick\\b|ill\\b|unwell\\b|back\\s+(?:in|at|on|from)\\b|still\\s+(?:off|ill|unwell|sick)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CREATIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("poem"), py_str("poems"), py_str("poetry"), py_str("haiku"), py_str("story"), py_str("stories"), py_str("song"), py_str("songs"), py_str("rhyme"), py_str("rhymes"), py_str("joke"), py_str("jokes"), py_str("speech"), py_str("speeches"), py_str("limerick")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGREEMENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ok"), py_str("okay"), py_str("okey"), py_str("k"), py_str("yes"), py_str("yeah"), py_str("yep"), py_str("yup"), py_str("aye"), py_str("sure"), py_str("please"), py_str("thanks"), py_str("thank"), py_str("you"), py_str("ta"), py_str("cheers"), py_str("go"), py_str("on"), py_str("then"), py_str("ahead"), py_str("that"), py_str("thats"), py_str("it"), py_str("is"), py_str("that's"), py_str("alright"), py_str("right"), py_str("cool"), py_str("great"), py_str("fine"), py_str("nice"), py_str("sound"), py_str("and"), py_str("so"), py_str("do"), py_str("thatll"), py_str("that'll"), py_str("let's"), py_str("lets"), py_str("have"), py_str("a"), py_str("shot"), py_str("give"), py_str("us"), py_str("for"), py_str("me"), py_str("my"), py_str("no"), py_str("nope"), py_str("nah"), py_str("not"), py_str("now"), py_str("later"), py_str("sounds"), py_str("good"), py_str("perfect"), py_str("lovely"), py_str("brilliant")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:what|why|how|who|which|when|where|should|could|would|can|do|does|did|is|are|was|were|will|have|has|any)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CAPITALS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_VENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("nightmare"), py_str("joke"), py_str("useless"), py_str("annoying"), py_str("awful"), py_str("horrible"), py_str("rubbish"), py_str("unbearable"), py_str("impossible"), py_str("toxic"), py_str("miserable"), py_str("unfair"), py_str("frustrating"), py_str("exhausting")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_VENT_PHRASES(py_tuple(vec![py_str("a pain in the"), py_str("fed up"), py_str("had enough"), py_str("cant stand"), py_str("driving me mad"), py_str("driving me up the wall"), py_str("driving me crazy")]));
    s_NEWS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("died"), py_str("death"), py_str("dead"), py_str("funeral"), py_str("buried"), py_str("cremation"), py_str("cancer"), py_str("tumour"), py_str("hospital"), py_str("hospice"), py_str("ward"), py_str("icu"), py_str("stolen"), py_str("burgled"), py_str("robbed"), py_str("mugged"), py_str("struggling"), py_str("panicking"), py_str("hyperventilating"), py_str("breakdown"), py_str("tumor"), py_str("diagnosed"), py_str("diagnosis"), py_str("miscarriage"), py_str("stillborn"), py_str("chemo"), py_str("redundant"), py_str("sacked"), py_str("fired"), py_str("bankrupt"), py_str("evicted"), py_str("divorce"), py_str("divorced"), py_str("breakup"), py_str("dumped"), py_str("debt"), py_str("debts"), py_str("arrears"), py_str("homeless"), py_str("bereaved"), py_str("bereavement"), py_str("dying")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEWS_PHRASES(py_tuple(vec![py_str("passed away"), py_str("lost my job"), py_str("lost his job"), py_str("lost her job"), py_str("lost the baby"), py_str("in debt"), py_str("behind on"), py_str("split up"), py_str("walked out"), py_str("left me"), py_str("laid off"), py_str("in hospital"), py_str("in the hospital"), py_str("my phone was"), py_str("got stolen"), py_str("was stolen"), py_str("panic attack"), py_str("anxiety attack"), py_str("im struggling"), py_str("really struggling"), py_str("not coping"), py_str("falling apart"), py_str("i feel awful"), py_str("feel terrible")]));
    s_FEELING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("tired"), py_str("exhausted"), py_str("knackered"), py_str("shattered"), py_str("drained"), py_str("stressed"), py_str("overwhelmed"), py_str("anxious"), py_str("depressed"), py_str("upset"), py_str("unhappy"), py_str("miserable"), py_str("angry"), py_str("furious"), py_str("worried"), py_str("scared"), py_str("awful"), py_str("terrible"), py_str("horrible"), py_str("panicky"), py_str("terrified"), py_str("devastated"), py_str("heartbroken"), py_str("gutted"), py_str("low")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FEELING_PHRASES(py_tuple(vec![py_str("fed up"), py_str("had enough"), py_str("cant cope"), py_str("can't cope"), py_str("burnt out"), py_str("worn out")]));
    s_MESSAGE_SHAPES(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi (?:need|want|would like|have) to\\b|\\bi (?:need|want)\\b|\\bcan i\\b|\\bcould i\\b|\\bplease\\b|\\basking (?:for|about)\\b|\\bi(?:'m| am|m) asking\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi(?:'ll| will|ll) be\\b|\\bi(?:'m| am|m) (?:off|out|away|late|not coming|stuck|leaving|moving|starting|finishing|going to|back)\\b|\\bi (?:cant|cannot|can not|wont|will not)\\b|\\bi(?:'ve| have|ve) (?:decided|quit|resigned|booked|cancelled)\\b|\\bi dont (?:think i can|want to come)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_THING_GONE_WRONG({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|the|our)\\b[^.,!?]{0,28}\\b(?:broke|broken|bust|busted|snapped|stopped working|wont start|packed in|died|got stolen|was stolen|flooded|leaking|blocked|failed|cancelled|fell through)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_CHANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi (?:cant|cannot|can not|wont|will not|dont|do not)\\s+(?:sleep|eat|stop|breathe|walk|see|hear|swallow|keep|remember|focus|concentrate|pee|cry|poo|sit|stand)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANT_FROM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi (?:need|want|would like)\\b[^.,!?]{0,30}\\bfrom\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_MESSAGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi (?:dont|do not) know\\b|\\bno idea\\b|\\bnot sure (?:what|how)\\b|\\bi cant (?:think|find the words)\\b|\\bwhat should i say\\b|\\bhelp me (?:word|say)\\b|\\bi dont know what to say\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WISH_FOR_SOMEONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:want|wants|wanted|wish|wishing)\\s+(?:her|him|them|it)\\s+back\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HURT_WORD(py_str("(?:kill|murder|hurt|punch|hit|smash|beat|stab|shoot|strangle|choke|throttle|slap|kick|wring|batter|poison|drown)"));
    s_HURT_TARGET(py_str("(?:me|you|him|her|them|us|myself|yourself|himself|herself|themselves|my|his|her|their|our|someone|somebody|everyone|everybody)"));
    s_HURT_WISH(py_str("\\b(?:i|we|ill|i'll|im|i'm|id|i'd)\\s+(?:really\\s+|just\\s+|so\\s+|seriously\\s+)?(?:want|wanna|wanted|would\\s+like|would\\s+love|need|have|am\\s+going|going|will|ll|might|should|could|must|love|like|planned?\\s+to|wish(?:ed)?)\\s+(?:to\\s+)?"));
    s_HURTS_SOMEONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&g_HURT_WISH(), &g_HURT_WORD()), &py_str("\\s+(?:\\w+\\s+){0,3}")), &g_HURT_TARGET()), &py_str("\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANTS_THEM_DEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:want|wants|wanted|wish|wishes|wished)\\s+(?:\\w+\\s+){0,2}dead\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REPORTS_ON_SOMEONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:he|she|they)\\b|\\b(?:keeps?|kept)\\s+(?:asking|saying|telling|ringing|texting|changing|forgetting|moaning)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_YET(py_tuple(vec![py_str("That's the mood rather than the words - give me the one line you want the message to say and I'll write it."), py_str("Nearly: tell me what the message should actually say, and I'll put it in order."), py_str("Got the feeling. Now the words: what do you want them to read?")]));
    s_LEAD_INS(py_tuple(vec![py_str("Here's something you could send:"), py_str("Try this:"), py_str("Something like this works:"), py_str("Here's a short version:")]));
    s_QUOTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\\"\\u201c]([^\\\"\\u201d]{15,})[\\\"\\u201d]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MESSAGE_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:text|message|email|e-mail|note|letter)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SHAPE_CUES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:say|says|keep|short|specific|ask\\s+for|ask\\s+them|mention|put|include|subject|sign\\s?-?off|signed|date[ds]?|address|tell\\s+them|follow\\s+it\\s+up|first\\s+sentence|first\\s+line|opening|closing|paragraphs?|\\d+\\s*words|bullet\\s+points?|three\\s+or\\s+four|few\\s+lines|lines)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MESSAGE_NOUN(py_str("(?:text|message|msg|email|note|letter)"));
    s_WRITING_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:can|could|would|will)\\s+(?:you\\s+)?(?:please\\s+)?(?:help\\s+(?:me\\s+)?)?(?:write|word|draft|phrase|put\\s+together)\\b[^?]{0,40}?"), &g_MESSAGE_NOUN()), &py_str("\\s+(?:to|for)\\b|\\bhelp\\s+me\\s+(?:write|word|draft|phrase)\\b[^?]{0,40}?")), &g_MESSAGE_NOUN()), &py_str("\\s+(?:to|for)\\b|\\b(?:write|draft|word)\\s+me\\s+(?:a|an|the)\\s+(?:short\\s+|quick\\s+|nice\\s+|polite\\s+|firm\\s+)?")), &g_MESSAGE_NOUN()), &py_str("\\s+(?:to|for)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_CONTENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:saying|says|tell(?:ing)?|about|asking|ask|because|cos|since|explaining|to\\s+say|to\\s+ask|that)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AFTER_ILL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("be"), py_str("have"), py_str("see"), py_str("try"), py_str("call"), py_str("let"), py_str("need"), py_str("get"), py_str("go"), py_str("ring"), py_str("text"), py_str("come"), py_str("bring"), py_str("pay"), py_str("ask"), py_str("tell"), py_str("do"), py_str("send"), py_str("sort")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANTS_MORE_OF_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+)?i\\s+(?:just\\s+)?(?:want|wants|need|needs|would\\s+like|wanna)\\s+(?:to\\s+)?(?:\\w+\\s+){0,4}?(?:more|less|again|every\\s+day|every\\s+week|every\\s+morning|more\\s+often)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_SPAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:since|for)\\s+(?:\\d+\\s+|a\\s+|an\\s+|two\\s+|three\\s+|four\\s+|five\\s+|six\\s+|seven\\s+|eight\\s+|nine\\s+|ten\\s+)?(?:day|days|week|weeks|fortnight|fortnights|month|months|year|years|january|february|march|april|may|june|july|august|september|october|november|december)\\b|\\b(?:has|had)\\s+been\\b|\\bkeeps?\\s+(?:coming|going|happening|flaring|leaking|breaking)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REPORTED_SPEECH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:says?|said|telling|told|reckons?|thinks?|asked|wants?\\s+me\\s+to)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_LEAD(geti(&geti(&g_LEADS(), &op_neg(&Py::Int(1))), &Py::Int(0)));
    s_TELL_FRAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+)?i\\s+(?:just\\s+)?(?:need|want|have)\\s+to\\s+tell\\s+(?:them|him|her)\\s+(?:that\\s+)?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBJECT_EXAMPLES(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:plumber|electrician|boiler|leak|radiator|roofer|builder|joiner|damp|drain|washing machine|fridge|oven|heating|handyman)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("Leak under the kitchen sink - when can you come?")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:teacher|school|headteacher|head|nursery|childminder|tutor|college|university|lecturer|professor|year \\d+|form|class)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("Year 6 trip - a question about the form")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gp|doctor|surgery|dentist|hospital|clinic|prescription|appointment|nurse|consultant)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("Repeat prescription - could you check this for me")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:landlord|landlady|letting|lettings|agent|housing|council|tenant)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("Boiler still not working - flat 3")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:boss|manager|employer|hr|work|workplace|office|team|colleague|shift|rota)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("A quick question about Thursday")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:invoice|payment|paid|pay|bill|overdue|refund|quote|estimate|customer|client|supplier)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("Invoice 2144 - payment due")])]));
}
pub fn _wants_to_hurt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let _ = py_str("True when the line is a wish to hurt somebody, which is never a message to send.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HURTS_SOMEONE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WANTS_THEM_DEAD(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _is_a_message_shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut shape: Py = Py::None;
    let _ = py_str("True when the line reads as the content of a message rather than a fragment or a state.");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NO_MESSAGE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_CHANGE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WANT_FROM(), "search", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MESSAGE_SHAPES()) { let mut shape: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&shape.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THING_GONE_WRONG(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| says_it_is_the_message(args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn names_a_draft(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply is the composer's drafted message rather than a line about it.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEAD_INS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lstrip", args))(&__args) }, "startswith", args))(&__args) };
    Py::None
}
pub fn offers_a_draft(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply is the bot's promise to write the message, rather than the message.\n\n    The offer is the other half of the draft: on the first turn of \"can you help me write a text\n    to my landlord\" there is no detail yet to turn into a message, so the honest reply is to ask\n    for it. Read by its own words (`OFFER`), because it is drawn from a pool: see\n    `fallback_data.UNKNOWN_WORD`.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OFFER(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn hands_over_words(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let _ = py_str("True when a reply already hands the person words to send, in quotation marks.\n\n    A card that answers \"can you help me write a note to my son's teacher\" with the note itself\n    (\"Sam (5B) has been anxious about maths homework - could we talk?\") has done the job the\n    offer would only promise to do, so the offer must not take that turn. Read by the shape of\n    the quotation and not by the words in it: four words or more between quotes is a message to\n    send, not a phrase the bot is quoting back.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUOTED(), "findall", args))(&__args) });
    for __item in __iter1 {
    body = __item.clone();
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn hands_over_the_shape(args: &[Py]) -> Py {
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
    let mut cues: Py = Py::None;
    let mut low: Py = Py::None;
    let mut m: Py = Py::None;
    let _ = py_str("True when a reply already gives the parts of the message, without quoting it.\n\n    The other half of `hands_over_words`: a card that tells them what to put in the message has\n    answered the ask just as well as one that quotes the whole thing, and the offer may not take\n    that turn. Read by the message's name beside two of the cues, so a card about the subject\n    itself (the tenancy entry answers \"text to my landlord\" only about damp and the council) is\n    not mistaken for one that writes the message.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    cues = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SHAPE_CUES(), "finditer", args))(&__args) }) { let mut m: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&m.clone(), "group", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let __b = op_ge(&cues.clone(), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = op_ge(&cues.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MESSAGE_WORD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
    Py::None
}
pub fn reads_as_a_writing_ask(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("True when the line asks the bot to write a message and has not said what it should say.");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&g_CREATIVE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:how|what|where|when|why)\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WRITING_ASK(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&clean.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_CONTENT(), "search", args))(&__args) }, &Py::None));
    Py::None
}
pub fn _tidy(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut line: Py = Py::None;
    let mut out: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The person's own sentence, with chat-speak put right and a capital at the front.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "strip", args))(&__args) }, "lower", args))(&__args) }, &py_str("ill"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFTER_ILL(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(1)), "strip", args))(&__args) }, "lower", args))(&__args) })) } } } }) {
    text = op_add(&py_str("i'll "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) });
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    tail = if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) } } }) { slice_or_raise(&word.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_CAPITALS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "lower", args))(&__args) }))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&bare.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&bare.clone(), &Py::Int(1), &Py::None, &Py::None)), &tail.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    continue;
    }
    fixed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TIDY(), "get", args))(&__args) };
    if py_truthy(&{ let __b = fixed.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "islower", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&fixed.clone(), &tail.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    } else {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&line.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&line.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _small(args: &[Py]) -> Py {
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
    let _ = py_str("\"A recycling bin\" -> \"a recycling bin\", but \"I need my stuff back\" keeps its capital.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("I "), py_str("I'"), py_str("I,")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "startswith", args))(&__args) } } } } }) {
    return text.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_CAPITALS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }, "lower", args))(&__args) }))) {
    return text.clone();
    }
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&text.clone(), &Py::Int(0)), "lower", args))(&__args) }, &slice_or_raise(&text.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut body: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(the sentence to put in the message, shape) or (None, None).");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&py_set(vec![py_str("ok"), py_str("okay"), py_str("yes"), py_str("yeah"), py_str("no"), py_str("thanks"), py_str("thank you"), py_str("hi"), py_str("hello"), py_str("hey")]), &low.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_AGREEMENT(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUESTION(), "match", args))(&__args) }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    shape = py_str("said");
    let __iter1 = py_iter_or_raise(&g_LEADS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "match", args))(&__args) };
    if py_truthy(&found.clone()) {
    shape = kind.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    break;
    }
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WANTS_MORE_OF_IT(), "match", args))(&__args) }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(30)) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "lower", args))(&__args) }, "strip", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_CREATIVE())) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    body = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _tidy(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&body.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    return py_tuple(vec![body.clone(), shape.clone()]);
    Py::None
}
pub fn _formal(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the message is going to someone a person writes to formally.");
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FORMAL())) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FORMAL())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut low: Py = Py::None;
    let _ = py_str("The line's words, when the bot's last reply offered to draft; otherwise None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::last_bot(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OFFER(), "search", args))(&__args) }))) {
    return Py::None;
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    return low.clone();
    Py::None
}
pub fn _a_fault_they_were_asked_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut phrase: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How long the thing has been wrong, on the turn after the bot asked what to say.\n\n    The offer is the difference. \"give me who it's for and what you want them to do\" is a\n    question with an answer, and the answer arrives as a report rather than as a message: \"the\n    boiler has been broken for 2 weeks\", \"the damp has been there since october\". Neither has a\n    lead-in, so the shape test refused them and a topic entry answered instead - the boiler-repair\n    steps, one line after the bot had asked for the words - and the offer it made went missing.\n\n    Only the span makes a line this: \"since october\", \"for two weeks\", \"has been\", \"keeps\n    coming back for a month\". A live fault keeps the answer it has, because that answer is worth\n    more than a message (\"the boiler is leaking\" wants the water off at the wall, not \"Hi, the\n    boiler is leaking. Thanks.\"), and so does anything about a third person, anything reported at\n    second hand (\"he says hes waiting on a part\") and news or a feeling.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SPAN(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THIRD_PERSON(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPORTED_SPEECH(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_bitand(&words.clone(), &op_bitor(&g_NEWS(), &g_FEELING()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_NEWS_PHRASES(), &g_FEELING_PHRASES())) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn is_theirs(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut lead: Py = Py::None;
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the offer is open and the line is news about the person, or a mood.\n\n    Those are not the content of a message, and they are not a complaint to be answered with\n    \"tell me what the message should say\" either: they keep the answer they would have had.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_before(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&low.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WISH_FOR_SOMEONE(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _wants_to_hurt(args))(&__args) }) {
    return Py::Bool(true);
    }
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_the_message(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&lead.clone(), &Py::None))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&lead.clone(), &g_BARE_LEAD())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_a_message_shape(args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    rest = slice_or_raise(&low.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "match", args))(&__args) }, "end", args))(&__args) }, &Py::None, &Py::None);
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_bitor(&g_NEWS(), &g_FEELING()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_NEWS_PHRASES(), &g_FEELING_PHRASES())) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&rest.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APOLOGY(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THIRD_PERSON(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABSENT(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("i feel"), py_str("im "), py_str("i'm "), py_str("i am "), py_str("ive been "), py_str("i've been ")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "startswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FEELING()); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FEELING_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_VENT())) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VENT_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPORTS_ON_SOMEONE(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_a_message_shape(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_fault_they_were_asked_for(args))(&__args) } } }))) {
    return Py::Bool(true);
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_bitor(&g_NEWS(), &g_FEELING()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_NEWS_PHRASES(), &g_FEELING_PHRASES())) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::badnews::reads_as_pregnancy_bleeding(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::badnews::reads_as_sudden_loss(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::badnews::reads_as_bad_news(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::badnews::reads_like_a_symptom(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::loss::names_a_feeling(args))(&__args) } } } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _names_the_message(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut _kind: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The lead that opens the line, or None: \"saying ...\", \"asking for ...\", \"tell them ...\".");
    let __iter1 = py_iter_or_raise(&g_LEADS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    _kind = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "match", args))(&__args) }) {
    return pattern.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn says_it_is_the_message(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let _ = py_str("True when the line opens by naming the message: \"saying ...\", \"asking for ...\", \"tell them\n    ...\". The bare \"that/it's\" lead is left out on purpose: \"its about the deposit\" is a fragment,\n    not someone saying that what follows is the message.\n\n    The composer uses this to let the draft past the topic entries on a line that names what the\n    message says - \"saying i cant make saturday cos im skint\" is the note, and the money reading\n    of it is the note's reason rather than a new subject. A line that does not name the message\n    waits its turn behind the news, the symptoms and the losses.\n    ");
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_the_message(args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&lead.clone(), &g_BARE_LEAD())) } };
    Py::None
}
pub fn not_the_message(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("True when the offer is open but this line is a complaint about somebody, not the message.");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_before(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&low.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| is_theirs(args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_VENT())) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VENT_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPORTS_ON_SOMEONE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn drafter(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut apology: Py = Py::None;
    let mut body: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut friendly: Py = Py::None;
    let mut hello: Py = Py::None;
    let mut keeps_cap: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut small: Py = Py::None;
    let _ = py_str("The draft for this line, or None when the bot did not offer one last turn.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::last_bot(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OFFER(), "search", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| is_theirs(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| not_the_message(args))(&__args) } } }) {
    return Py::None;
    }
    frame = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TELL_FRAME(), "match", args))(&__args) };
    if py_truthy(&frame.clone()) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THIRD_PERSON(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    line = rest.clone();
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _content(args))(&__args) }, 2, None);
    body = __unpk1[0].clone();
    shape = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&body.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("told")))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("asked")))) {
    body = op_add(&py_str("I wanted to ask about "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _small(args))(&__args) });
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("?"), py_str("!"), py_str(".")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "endswith", args))(&__args) }) {
    body = slice_or_raise(&body.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    body = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _small(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COPULA(), "match", args))(&__args) }) {
    body = op_add(&op_add(&py_str("I'm "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&body.clone(), &Py::Int(0)), "lower", args))(&__args) }), &slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None));
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SUBJECT(), "match", args))(&__args) }) {
    body = op_add(&op_add(&py_str("I "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&body.clone(), &Py::Int(0)), "lower", args))(&__args) }), &slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    apology = Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APOLOGY(), "match", args))(&__args) }, &Py::None));
    if py_truthy(&apology.clone()) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APOLOGY_ACT(), "search", args))(&__args) }, &Py::None)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APOLOGY_SELF(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    body = op_add(&body.clone(), &py_str(". I shouldn't have done that"));
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SORRY(), "search", args))(&__args) }) {
    small = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&body.clone(), &Py::Int(0)), "lower", args))(&__args) }, &slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None));
    keeps_cap = Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("i(?:'|\\b)")); __args.push(body.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None));
    body = op_add(&py_str("sorry for the short notice - "), &if py_truthy(&keeps_cap.clone()) { body.clone() } else { small.clone() });
    }
    }
    friendly = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = words.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _formal(args))(&__args) }));
    hello = if py_truthy(&friendly.clone()) { py_str("Hey") } else { py_str("Hi") };
    sign = if py_truthy(&friendly.clone()) { py_str("Cheers") } else { py_str("Thanks") };
    if py_truthy(&{ let __b = apology.clone(); if py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&hello.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&body.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&hello.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&body.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&sign.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn example_subject(args: &[Py]) -> Py {
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
    let mut example: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The sample subject line for the email-shape card, in the world the line names.");
    line = py_str("");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    } else {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&g_SUBJECT_EXAMPLES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    example = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return example.clone();
    }
    }
    return py_str("Invoice 2144 - payment due");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.draft".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABSENT" => Some(g_ABSENT()),
            "AFTER_ILL" => Some(g_AFTER_ILL()),
            "AGREEMENT" => Some(g_AGREEMENT()),
            "APOLOGY" => Some(g_APOLOGY()),
            "APOLOGY_ACT" => Some(g_APOLOGY_ACT()),
            "APOLOGY_SELF" => Some(g_APOLOGY_SELF()),
            "A_SPAN" => Some(g_A_SPAN()),
            "A_WISH_FOR_SOMEONE" => Some(g_A_WISH_FOR_SOMEONE()),
            "BARE_LEAD" => Some(g_BARE_LEAD()),
            "CAPITALS" => Some(g_CAPITALS()),
            "COPULA" => Some(g_COPULA()),
            "CREATIVE" => Some(g_CREATIVE()),
            "FEELING" => Some(g_FEELING()),
            "FEELING_PHRASES" => Some(g_FEELING_PHRASES()),
            "FORMAL" => Some(g_FORMAL()),
            "HURTS_SOMEONE" => Some(g_HURTS_SOMEONE()),
            "HURT_TARGET" => Some(g_HURT_TARGET()),
            "HURT_WISH" => Some(g_HURT_WISH()),
            "HURT_WORD" => Some(g_HURT_WORD()),
            "LEADS" => Some(g_LEADS()),
            "LEAD_INS" => Some(g_LEAD_INS()),
            "MESSAGE_NOUN" => Some(g_MESSAGE_NOUN()),
            "MESSAGE_SHAPES" => Some(g_MESSAGE_SHAPES()),
            "MESSAGE_WORD" => Some(g_MESSAGE_WORD()),
            "NEWS" => Some(g_NEWS()),
            "NEWS_PHRASES" => Some(g_NEWS_PHRASES()),
            "NOT_A_CHANGE" => Some(g_NOT_A_CHANGE()),
            "NOT_YET" => Some(g_NOT_YET()),
            "NO_MESSAGE" => Some(g_NO_MESSAGE()),
            "OFFER" => Some(g_OFFER()),
            "QUESTION" => Some(g_QUESTION()),
            "QUOTED" => Some(g_QUOTED()),
            "REPORTED_SPEECH" => Some(g_REPORTED_SPEECH()),
            "REPORTS_ON_SOMEONE" => Some(g_REPORTS_ON_SOMEONE()),
            "SHAPE_CUES" => Some(g_SHAPE_CUES()),
            "SORRY" => Some(g_SORRY()),
            "SUBJECT" => Some(g_SUBJECT()),
            "SUBJECT_EXAMPLES" => Some(g_SUBJECT_EXAMPLES()),
            "TELL_FRAME" => Some(g_TELL_FRAME()),
            "THE_CONTENT" => Some(g_THE_CONTENT()),
            "THING_GONE_WRONG" => Some(g_THING_GONE_WRONG()),
            "THIRD_PERSON" => Some(g_THIRD_PERSON()),
            "TIDY" => Some(g_TIDY()),
            "VENT" => Some(g_VENT()),
            "VENT_PHRASES" => Some(g_VENT_PHRASES()),
            "WANTS_MORE_OF_IT" => Some(g_WANTS_MORE_OF_IT()),
            "WANTS_THEM_DEAD" => Some(g_WANTS_THEM_DEAD()),
            "WANT_FROM" => Some(g_WANT_FROM()),
            "WRITING_ASK" => Some(g_WRITING_ASK()),
            "_a_fault_they_were_asked_for" => Some(py_func("_a_fault_they_were_asked_for", |args: &[Py]| _a_fault_they_were_asked_for(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_formal" => Some(py_func("_formal", |args: &[Py]| _formal(args))),
            "_is_a_message_shape" => Some(py_func("_is_a_message_shape", |args: &[Py]| _is_a_message_shape(args))),
            "_names_the_message" => Some(py_func("_names_the_message", |args: &[Py]| _names_the_message(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "_small" => Some(py_func("_small", |args: &[Py]| _small(args))),
            "_tidy" => Some(py_func("_tidy", |args: &[Py]| _tidy(args))),
            "_wants_to_hurt" => Some(py_func("_wants_to_hurt", |args: &[Py]| _wants_to_hurt(args))),
            "drafter" => Some(py_func("drafter", |args: &[Py]| drafter(args))),
            "example_subject" => Some(py_func("example_subject", |args: &[Py]| example_subject(args))),
            "hands_over_the_shape" => Some(py_func("hands_over_the_shape", |args: &[Py]| hands_over_the_shape(args))),
            "hands_over_words" => Some(py_func("hands_over_words", |args: &[Py]| hands_over_words(args))),
            "is_theirs" => Some(py_func("is_theirs", |args: &[Py]| is_theirs(args))),
            "names_a_draft" => Some(py_func("names_a_draft", |args: &[Py]| names_a_draft(args))),
            "not_the_message" => Some(py_func("not_the_message", |args: &[Py]| not_the_message(args))),
            "offers_a_draft" => Some(py_func("offers_a_draft", |args: &[Py]| offers_a_draft(args))),
            "reads_as_a_writing_ask" => Some(py_func("reads_as_a_writing_ask", |args: &[Py]| reads_as_a_writing_ask(args))),
            "says_it_is_the_message" => Some(py_func("says_it_is_the_message", |args: &[Py]| says_it_is_the_message(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABSENT" => s_ABSENT(v),
            "AFTER_ILL" => s_AFTER_ILL(v),
            "AGREEMENT" => s_AGREEMENT(v),
            "APOLOGY" => s_APOLOGY(v),
            "APOLOGY_ACT" => s_APOLOGY_ACT(v),
            "APOLOGY_SELF" => s_APOLOGY_SELF(v),
            "A_SPAN" => s_A_SPAN(v),
            "A_WISH_FOR_SOMEONE" => s_A_WISH_FOR_SOMEONE(v),
            "BARE_LEAD" => s_BARE_LEAD(v),
            "CAPITALS" => s_CAPITALS(v),
            "COPULA" => s_COPULA(v),
            "CREATIVE" => s_CREATIVE(v),
            "FEELING" => s_FEELING(v),
            "FEELING_PHRASES" => s_FEELING_PHRASES(v),
            "FORMAL" => s_FORMAL(v),
            "HURTS_SOMEONE" => s_HURTS_SOMEONE(v),
            "HURT_TARGET" => s_HURT_TARGET(v),
            "HURT_WISH" => s_HURT_WISH(v),
            "HURT_WORD" => s_HURT_WORD(v),
            "LEADS" => s_LEADS(v),
            "LEAD_INS" => s_LEAD_INS(v),
            "MESSAGE_NOUN" => s_MESSAGE_NOUN(v),
            "MESSAGE_SHAPES" => s_MESSAGE_SHAPES(v),
            "MESSAGE_WORD" => s_MESSAGE_WORD(v),
            "NEWS" => s_NEWS(v),
            "NEWS_PHRASES" => s_NEWS_PHRASES(v),
            "NOT_A_CHANGE" => s_NOT_A_CHANGE(v),
            "NOT_YET" => s_NOT_YET(v),
            "NO_MESSAGE" => s_NO_MESSAGE(v),
            "OFFER" => s_OFFER(v),
            "QUESTION" => s_QUESTION(v),
            "QUOTED" => s_QUOTED(v),
            "REPORTED_SPEECH" => s_REPORTED_SPEECH(v),
            "REPORTS_ON_SOMEONE" => s_REPORTS_ON_SOMEONE(v),
            "SHAPE_CUES" => s_SHAPE_CUES(v),
            "SORRY" => s_SORRY(v),
            "SUBJECT" => s_SUBJECT(v),
            "SUBJECT_EXAMPLES" => s_SUBJECT_EXAMPLES(v),
            "TELL_FRAME" => s_TELL_FRAME(v),
            "THE_CONTENT" => s_THE_CONTENT(v),
            "THING_GONE_WRONG" => s_THING_GONE_WRONG(v),
            "THIRD_PERSON" => s_THIRD_PERSON(v),
            "TIDY" => s_TIDY(v),
            "VENT" => s_VENT(v),
            "VENT_PHRASES" => s_VENT_PHRASES(v),
            "WANTS_MORE_OF_IT" => s_WANTS_MORE_OF_IT(v),
            "WANTS_THEM_DEAD" => s_WANTS_THEM_DEAD(v),
            "WANT_FROM" => s_WANT_FROM(v),
            "WRITING_ASK" => s_WRITING_ASK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.draft", module_obj);
    pyrt::xrefs::reg_fn("compose.draft", "_wants_to_hurt", py_func("_wants_to_hurt", |args: &[Py]| _wants_to_hurt(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_is_a_message_shape", py_func("_is_a_message_shape", |args: &[Py]| _is_a_message_shape(args)));
    pyrt::xrefs::reg_fn("compose.draft", "names_a_draft", py_func("names_a_draft", |args: &[Py]| names_a_draft(args)));
    pyrt::xrefs::reg_fn("compose.draft", "offers_a_draft", py_func("offers_a_draft", |args: &[Py]| offers_a_draft(args)));
    pyrt::xrefs::reg_fn("compose.draft", "hands_over_words", py_func("hands_over_words", |args: &[Py]| hands_over_words(args)));
    pyrt::xrefs::reg_fn("compose.draft", "hands_over_the_shape", py_func("hands_over_the_shape", |args: &[Py]| hands_over_the_shape(args)));
    pyrt::xrefs::reg_fn("compose.draft", "reads_as_a_writing_ask", py_func("reads_as_a_writing_ask", |args: &[Py]| reads_as_a_writing_ask(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_tidy", py_func("_tidy", |args: &[Py]| _tidy(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_small", py_func("_small", |args: &[Py]| _small(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_content", py_func("_content", |args: &[Py]| _content(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_formal", py_func("_formal", |args: &[Py]| _formal(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_said_before", py_func("_said_before", |args: &[Py]| _said_before(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_a_fault_they_were_asked_for", py_func("_a_fault_they_were_asked_for", |args: &[Py]| _a_fault_they_were_asked_for(args)));
    pyrt::xrefs::reg_fn("compose.draft", "is_theirs", py_func("is_theirs", |args: &[Py]| is_theirs(args)));
    pyrt::xrefs::reg_fn("compose.draft", "_names_the_message", py_func("_names_the_message", |args: &[Py]| _names_the_message(args)));
    pyrt::xrefs::reg_fn("compose.draft", "says_it_is_the_message", py_func("says_it_is_the_message", |args: &[Py]| says_it_is_the_message(args)));
    pyrt::xrefs::reg_fn("compose.draft", "not_the_message", py_func("not_the_message", |args: &[Py]| not_the_message(args)));
    pyrt::xrefs::reg_fn("compose.draft", "drafter", py_func("drafter", |args: &[Py]| drafter(args)));
    pyrt::xrefs::reg_fn("compose.draft", "example_subject", py_func("example_subject", |args: &[Py]| example_subject(args)));
    pyrt::xrefs::reg_access("compose.draft", "ABSENT", g_ABSENT, s_ABSENT);
    pyrt::xrefs::reg_access("compose.draft", "AFTER_ILL", g_AFTER_ILL, s_AFTER_ILL);
    pyrt::xrefs::reg_access("compose.draft", "AGREEMENT", g_AGREEMENT, s_AGREEMENT);
    pyrt::xrefs::reg_access("compose.draft", "APOLOGY", g_APOLOGY, s_APOLOGY);
    pyrt::xrefs::reg_access("compose.draft", "APOLOGY_ACT", g_APOLOGY_ACT, s_APOLOGY_ACT);
    pyrt::xrefs::reg_access("compose.draft", "APOLOGY_SELF", g_APOLOGY_SELF, s_APOLOGY_SELF);
    pyrt::xrefs::reg_access("compose.draft", "A_SPAN", g_A_SPAN, s_A_SPAN);
    pyrt::xrefs::reg_access("compose.draft", "A_WISH_FOR_SOMEONE", g_A_WISH_FOR_SOMEONE, s_A_WISH_FOR_SOMEONE);
    pyrt::xrefs::reg_access("compose.draft", "BARE_LEAD", g_BARE_LEAD, s_BARE_LEAD);
    pyrt::xrefs::reg_access("compose.draft", "CAPITALS", g_CAPITALS, s_CAPITALS);
    pyrt::xrefs::reg_access("compose.draft", "COPULA", g_COPULA, s_COPULA);
    pyrt::xrefs::reg_access("compose.draft", "CREATIVE", g_CREATIVE, s_CREATIVE);
    pyrt::xrefs::reg_access("compose.draft", "FEELING", g_FEELING, s_FEELING);
    pyrt::xrefs::reg_access("compose.draft", "FEELING_PHRASES", g_FEELING_PHRASES, s_FEELING_PHRASES);
    pyrt::xrefs::reg_access("compose.draft", "FORMAL", g_FORMAL, s_FORMAL);
    pyrt::xrefs::reg_access("compose.draft", "HURTS_SOMEONE", g_HURTS_SOMEONE, s_HURTS_SOMEONE);
    pyrt::xrefs::reg_access("compose.draft", "HURT_TARGET", g_HURT_TARGET, s_HURT_TARGET);
    pyrt::xrefs::reg_access("compose.draft", "HURT_WISH", g_HURT_WISH, s_HURT_WISH);
    pyrt::xrefs::reg_access("compose.draft", "HURT_WORD", g_HURT_WORD, s_HURT_WORD);
    pyrt::xrefs::reg_access("compose.draft", "LEADS", g_LEADS, s_LEADS);
    pyrt::xrefs::reg_access("compose.draft", "LEAD_INS", g_LEAD_INS, s_LEAD_INS);
    pyrt::xrefs::reg_access("compose.draft", "MESSAGE_NOUN", g_MESSAGE_NOUN, s_MESSAGE_NOUN);
    pyrt::xrefs::reg_access("compose.draft", "MESSAGE_SHAPES", g_MESSAGE_SHAPES, s_MESSAGE_SHAPES);
    pyrt::xrefs::reg_access("compose.draft", "MESSAGE_WORD", g_MESSAGE_WORD, s_MESSAGE_WORD);
    pyrt::xrefs::reg_access("compose.draft", "NEWS", g_NEWS, s_NEWS);
    pyrt::xrefs::reg_access("compose.draft", "NEWS_PHRASES", g_NEWS_PHRASES, s_NEWS_PHRASES);
    pyrt::xrefs::reg_access("compose.draft", "NOT_A_CHANGE", g_NOT_A_CHANGE, s_NOT_A_CHANGE);
    pyrt::xrefs::reg_access("compose.draft", "NOT_YET", g_NOT_YET, s_NOT_YET);
    pyrt::xrefs::reg_access("compose.draft", "NO_MESSAGE", g_NO_MESSAGE, s_NO_MESSAGE);
    pyrt::xrefs::reg_access("compose.draft", "OFFER", g_OFFER, s_OFFER);
    pyrt::xrefs::reg_access("compose.draft", "QUESTION", g_QUESTION, s_QUESTION);
    pyrt::xrefs::reg_access("compose.draft", "QUOTED", g_QUOTED, s_QUOTED);
    pyrt::xrefs::reg_access("compose.draft", "REPORTED_SPEECH", g_REPORTED_SPEECH, s_REPORTED_SPEECH);
    pyrt::xrefs::reg_access("compose.draft", "REPORTS_ON_SOMEONE", g_REPORTS_ON_SOMEONE, s_REPORTS_ON_SOMEONE);
    pyrt::xrefs::reg_access("compose.draft", "SHAPE_CUES", g_SHAPE_CUES, s_SHAPE_CUES);
    pyrt::xrefs::reg_access("compose.draft", "SORRY", g_SORRY, s_SORRY);
    pyrt::xrefs::reg_access("compose.draft", "SUBJECT", g_SUBJECT, s_SUBJECT);
    pyrt::xrefs::reg_access("compose.draft", "SUBJECT_EXAMPLES", g_SUBJECT_EXAMPLES, s_SUBJECT_EXAMPLES);
    pyrt::xrefs::reg_access("compose.draft", "TELL_FRAME", g_TELL_FRAME, s_TELL_FRAME);
    pyrt::xrefs::reg_access("compose.draft", "THE_CONTENT", g_THE_CONTENT, s_THE_CONTENT);
    pyrt::xrefs::reg_access("compose.draft", "THING_GONE_WRONG", g_THING_GONE_WRONG, s_THING_GONE_WRONG);
    pyrt::xrefs::reg_access("compose.draft", "THIRD_PERSON", g_THIRD_PERSON, s_THIRD_PERSON);
    pyrt::xrefs::reg_access("compose.draft", "TIDY", g_TIDY, s_TIDY);
    pyrt::xrefs::reg_access("compose.draft", "VENT", g_VENT, s_VENT);
    pyrt::xrefs::reg_access("compose.draft", "VENT_PHRASES", g_VENT_PHRASES, s_VENT_PHRASES);
    pyrt::xrefs::reg_access("compose.draft", "WANTS_MORE_OF_IT", g_WANTS_MORE_OF_IT, s_WANTS_MORE_OF_IT);
    pyrt::xrefs::reg_access("compose.draft", "WANTS_THEM_DEAD", g_WANTS_THEM_DEAD, s_WANTS_THEM_DEAD);
    pyrt::xrefs::reg_access("compose.draft", "WANT_FROM", g_WANT_FROM, s_WANT_FROM);
    pyrt::xrefs::reg_access("compose.draft", "WRITING_ASK", g_WRITING_ASK, s_WRITING_ASK);
}
