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


thread_local! { static G_BLANK_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALK_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BARE_INVITATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LEAD_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME_BEFORE_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLAUSE_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEDGED_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWNERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POLITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPEATED_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMETHING_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_END_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TALK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TYPED_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TYPED_SHORT_DICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BLANK_SUBJECTS() -> Py { ensure_init(); G_BLANK_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLANK_SUBJECTS(v: Py) { ensure_init(); G_BLANK_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITED() -> Py { ensure_init(); G_INVITED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITED(v: Py) { ensure_init(); G_INVITED.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL() -> Py { ensure_init(); G_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL(v: Py) { ensure_init(); G_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL_CLAUSE() -> Py { ensure_init(); G_POOL_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL_CLAUSE(v: Py) { ensure_init(); G_POOL_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_TALK_ASKS() -> Py { ensure_init(); G_TALK_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALK_ASKS(v: Py) { ensure_init(); G_TALK_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHING() -> Py { ensure_init(); G_WEIGHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHING(v: Py) { ensure_init(); G_WEIGHING.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHING_WORDS() -> Py { ensure_init(); G_WEIGHING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHING_WORDS(v: Py) { ensure_init(); G_WEIGHING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BARE_INVITATION() -> Py { ensure_init(); G__A_BARE_INVITATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BARE_INVITATION(v: Py) { ensure_init(); G__A_BARE_INVITATION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LEAD_IN_FRONT() -> Py { ensure_init(); G__A_LEAD_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LEAD_IN_FRONT(v: Py) { ensure_init(); G__A_LEAD_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_BEFORE_THE_THING() -> Py { ensure_init(); G__A_TIME_BEFORE_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_BEFORE_THE_THING(v: Py) { ensure_init(); G__A_TIME_BEFORE_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__CLAUSE_START() -> Py { ensure_init(); G__CLAUSE_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLAUSE_START(v: Py) { ensure_init(); G__CLAUSE_START.with(|c| *c.borrow_mut() = v); }
pub fn g__HEDGED_END() -> Py { ensure_init(); G__HEDGED_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEDGED_END(v: Py) { ensure_init(); G__HEDGED_END.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD() -> Py { ensure_init(); G__LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); G__LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__OWNERS() -> Py { ensure_init(); G__OWNERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWNERS(v: Py) { ensure_init(); G__OWNERS.with(|c| *c.borrow_mut() = v); }
pub fn g__POLITE() -> Py { ensure_init(); G__POLITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POLITE(v: Py) { ensure_init(); G__POLITE.with(|c| *c.borrow_mut() = v); }
pub fn g__REPEATED_WORD() -> Py { ensure_init(); G__REPEATED_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPEATED_WORD(v: Py) { ensure_init(); G__REPEATED_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMETHING_ELSE() -> Py { ensure_init(); G__SOMETHING_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMETHING_ELSE(v: Py) { ensure_init(); G__SOMETHING_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_END() -> Py { ensure_init(); G__SUBJECT_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_END(v: Py) { ensure_init(); G__SUBJECT_END.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_END_CLAUSE() -> Py { ensure_init(); G__SUBJECT_END_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_END_CLAUSE(v: Py) { ensure_init(); G__SUBJECT_END_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_HEAD() -> Py { ensure_init(); G__SUBJECT_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_HEAD(v: Py) { ensure_init(); G__SUBJECT_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_TAIL() -> Py { ensure_init(); G__SUBJECT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_TAIL(v: Py) { ensure_init(); G__SUBJECT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__TALK() -> Py { ensure_init(); G__TALK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TALK(v: Py) { ensure_init(); G__TALK.with(|c| *c.borrow_mut() = v); }
pub fn g__TYPED_SHORT() -> Py { ensure_init(); G__TYPED_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TYPED_SHORT(v: Py) { ensure_init(); G__TYPED_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g__TYPED_SHORT_DICT() -> Py { ensure_init(); G__TYPED_SHORT_DICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TYPED_SHORT_DICT(v: Py) { ensure_init(); G__TYPED_SHORT_DICT.with(|c| *c.borrow_mut() = v); }
pub fn g__VERB() -> Py { ensure_init(); G__VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERB(v: Py) { ensure_init(); G__VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__WHEN() -> Py { ensure_init(); G__WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHEN(v: Py) { ensure_init(); G__WHEN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Somebody asks this bot for room to talk about a thing of theirs, and names the thing.\n\n\"can you talk to me about my anxiety\" got the search hedge (\"I don't know enough to answer that\nhonestly. A search should sort you out quickly\") - which is no answer at all: nobody wants a search\nengine for that. The bot can do the one thing asked for, which is talk with them, and the honest\nreply is a yes in a few words and the one question that starts it: STYLE.md's \"Not knowing,\nhonestly\" and its one short question in the same breath.\n\nThe ask is read, not listed: \"can you talk to me about X\", \"can we talk about X\", \"can i talk to\nyou about X\", \"i want / need / would like to talk about X\", \"talk to me about X\", and the same\nframes with \"think\" and \"work\" (\"can you help me think about my options\"). The thing named is said\nback in the person's own words with the possessive turned round (\"my job\" is \"your job\"), because\nthe thing named is the thing they want to talk about.\n\nNarrow on purpose. It wants the bot addressed - a person in the line asking for the bot's ear - so\n\"he wont talk to me about it\" and \"my daughter wont talk about it\" are somebody else's refusal and\nkeep the reply they had. It only ever replaces one of the composer's own shrugs, so a card that\nanswers the line stands, and a crisis line never reaches here.\n");
    s__LEAD(py_str("(?:(?:ok|okay|so|right|hi|hey|well)\\s+)?"));
    s__POLITE(py_str("(?:(?:please|just)\\s+)?"));
    s__TALK(py_str("(?:talk|chat|speak)"));
    s__VERB(py_str("(?:want|wanna|need|needed|would\\s+like|'?d\\s+like|have\\s+to|got\\s+to|gotta|have|got)"));
    s_TALK_ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![op_add(&op_add(&op_add(&py_str("(?:can|could|would|will)\\s+you\\s+"), &g__POLITE()), &g__TALK()), &py_str("\\s+(?:to|with)\\s+me\\s+(?:about|through)\\s+(?P<subject>.+)$")), op_add(&op_add(&py_str("(?:can|could|may)\\s+(?:i|we)\\s+(?:please\\s+)?"), &g__TALK()), &py_str("(?:\\s+(?:to|with)\\s+you)?\\s+(?:about|through)\\s+(?P<subject>.+)$")), op_add(&op_add(&op_add(&op_add(&py_str("(?:i|we)\\s+(?:just\\s+|really\\s+|honestly\\s+)?"), &g__VERB()), &py_str("\\s+(?:to\\s+)?")), &g__TALK()), &py_str("(?:\\s+(?:to|with)\\s+you)?\\s+(?:about|through)\\s+(?P<subject>.+)$")), op_add(&op_add(&g__POLITE(), &g__TALK()), &py_str("\\s+to\\s+me\\s+(?:about|through)\\s+(?P<subject>.+)$")), op_add(&op_add(&py_str("(?:can|could|would|will)\\s+you\\s+"), &g__POLITE()), &py_str("help\\s+me\\s+(?:think|talk|work)\\s+(?:about|through|out)\\s+(?P<subject>.+)$")), op_add(&op_add(&py_str("(?:i|we)\\s+(?:just\\s+|really\\s+)?"), &g__VERB()), &py_str("\\s+(?:to\\s+)?(?:think|work)\\s+(?:about|through|out)\\s+(?P<subject>.+)$"))])) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&g__LEAD(), &pattern.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BLANK_SUBJECTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("them"), py_str("something"), py_str("somethin"), py_str("anything"), py_str("everything"), py_str("stuff"), py_str("a thing"), py_str("one thing"), py_str("it all"), py_str("something else")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SUBJECT_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:with|to)\\s+(?:you|me)\\s*$|\\s+(?:again|please|now|today|tonight|later|tomorrow|sometime|soon|this\\s+(?:evening|afternoon|morning|weekend)|next\\s+week)\\s*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WHEN(py_str("(?:again|now|today|tonight|tomorrow|later|sometime|soon|this\\s+(?:evening|afternoon|morning|weekend)|next\\s+week|when\\s+(?:you\\s+)?(?:have|get|'ve\\s+got)\\s+a\\s+(?:minute|mo|moment))"));
    s__A_TIME_BEFORE_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(talk|chat|speak|think|work)\\s+(?:to\\s+(?:you|me)\\s+)?(?:"), &g__WHEN()), &py_str("\\s+)+(?=(?:about|through|out)\\b)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SUBJECT_HEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:the|that|this)\\s+thing\\s+(?:about|with)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SOMETHING_ELSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:something|anything|somebody|anybody|a\\s+different\\s+thing|something\\s+different|another\\s+thing|anything\\s+else\\s+at\\s+all)\\s+else\\s*[,;:\\-\\u2013\\u2014]?\\s*")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SUBJECT_END({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:its|it's|it is|it was|it's|and|but|because|cos|cuz|coz|so|which|that|who|i|ive|i've|we|were|we're|he|she|they|my|our)\\b.*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__CLAUSE_START({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:how|what|why|whether|if|when|where|which|who|whoever|whatever)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SUBJECT_END_CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:its|it's|it is|it was|and|but|because|cos|cuz|coz|which|that)\\b.*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HEDGED_END({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*,?\\s*(?:maybe|perhaps|i\\s+think|i\\s+reckon|if\\s+that\\s+is\\s+ok)\\s*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OWNERS(py_tuple(vec![py_tuple(vec![py_str("my"), py_str("your")]), py_tuple(vec![py_str("our"), py_str("your")])]));
    s__A_BARE_INVITATION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g__LEAD(), &py_str("(?:(?:can|could|may|shall|should)\\s+(?:i|we)\\s+")), &g__POLITE()), &py_str("|(?:i|we)\\s+")), &g__VERB()), &py_str("\\s+(?:to\\s+)?|")), &g__POLITE()), &py_str(")(?:talk|chat|speak)(?:\\s+(?:to|with)\\s+(?:you|me))?(?:\\s+")), &g__WHEN()), &py_str(")+\\s*[?.!]*$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_LEAD_IN_FRONT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&g__LEAD(), &py_str("(?:(?:can|could|may|shall|should)\\s+(?:i|we)\\s+")), &g__POLITE()), &py_str("|(?:i|we)\\s+")), &g__VERB()), &py_str("\\s+(?:to\\s+)?)(?:talk|chat|speak)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_INVITED(py_tuple(vec![py_str("Yes - I'm here whenever you are. What's on your mind?"), py_str("Any time. What is it you want to talk about?"), py_str("Of course. What's going on, and where do you want to start?")]));
    s_POOL(py_tuple(vec![py_str("Of course. Tell me about {thing} - what's the part that's bothering you most?"), py_str("{Thing} it is. What's on your mind about it?"), py_str("Happy to. Where do you want to start with {thing}?"), py_str("Right, {thing} - go on. What's the bit you want to think out loud about?")]));
    s_POOL_CLAUSE(py_tuple(vec![geti(&g_POOL(), &Py::Int(0)), geti(&g_POOL(), &Py::Int(3))]));
    s_WEIGHING(py_tuple(vec![py_str("Yes. What are the options you're weighing, as they stand now?"), py_str("Let's lay them out. What's on each side of it?"), py_str("Of course. What would you lose with each one?")]));
    s_WEIGHING_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\noptions option choice choices decision decisions pros cons\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TYPED_SHORT(py_tuple(vec![py_tuple(vec![py_str("u"), py_str("you")]), py_tuple(vec![py_str("ya"), py_str("you")]), py_tuple(vec![py_str("ur"), py_str("your")]), py_tuple(vec![py_str("yr"), py_str("your")]), py_tuple(vec![py_str("wana"), py_str("want")]), py_tuple(vec![py_str("wanna"), py_str("want")]), py_tuple(vec![py_str("wanta"), py_str("want")])]));
    s__REPEATED_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(\\w+)(\\s+\\1\\b)+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TYPED_SHORT_DICT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__TYPED_SHORT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _as_typed(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with the chat-speak the frames use spelled out and a doubled word collapsed.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\1")); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__REPEATED_WORD(), "sub", args))(&__args) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TYPED_SHORT_DICT(), "get", args))(&__args) }); } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn the_line_as_read(args: &[Py]) -> Py {
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
    let _ = py_str("The line the frames are matched against: chat-speak spelled out, a named day dropped.\n\n    `bot/modules/talkto.py` looks the line up in the knowledge base to see whether a card answers\n    the thing named, and it has to ask about the same line this reader read - otherwise a day in\n    the ask (\"can we talk tomorrow about my mum\") makes the lookup miss the carers card and this\n    module takes a turn a card answers better.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\1 ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _as_typed(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TIME_BEFORE_THE_THING(), "sub", args))(&__args) };
    Py::None
}
pub fn _subject(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing named after the talk frame, or \"\" when none is named.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_line_as_read(args))(&__args) };
    let __iter1 = py_iter_or_raise(&g_TALK_ASKS());
    for __item in __iter1 {
    pattern = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    subject = op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".!?,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) });
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SUBJECT_TAIL(), "sub", args))(&__args) }, "strip", args))(&__args) };
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SUBJECT_HEAD(), "sub", args))(&__args) }, "strip", args))(&__args) };
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMETHING_ELSE(), "sub", args))(&__args) }, "strip", args))(&__args) };
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CLAUSE_START(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&if py_truthy(&clause.clone()) { g__SUBJECT_END_CLAUSE() } else { g__SUBJECT_END() }, "sub", args))(&__args) }, "strip", args))(&__args) };
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HEDGED_END(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g_BLANK_SUBJECTS(), &subject.clone()))) {
    return py_str("");
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&subject.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &if py_truthy(&clause.clone()) { Py::Int(9) } else { Py::Int(5) }) } }) {
    return py_str("");
    }
    let __iter2 = py_iter_or_raise(&g__OWNERS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    mine = __unpk3[0].clone();
    theirs = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &mine.clone()))) {
    let _ = py_setitem(&words.clone(), &Py::Int(0), theirs.clone());
    break;
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    return py_str("");
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
    let mut frames: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut one: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut typed: Py = Py::None;
    let _ = py_str("A yes and the one question that starts it, or \"\" when the line is not this ask.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _subject(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    typed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _as_typed(args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(typed.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_BARE_INVITATION(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(typed.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_LEAD_IN_FRONT(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&typed.clone(), "rstrip", args))(&__args) }, "endswith", args))(&__args) } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_INVITED()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    return py_str("");
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&subject.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_WEIGHING_WORDS())) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WEIGHING()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    frames = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CLAUSE_START(), "match", args))(&__args) }) { g_POOL_CLAUSE() } else { g_POOL() };
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut one: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), subject.clone()), ("Thing".to_string(), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&subject.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&subject.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.talkto".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BLANK_SUBJECTS" => Some(g_BLANK_SUBJECTS()),
            "INVITED" => Some(g_INVITED()),
            "POOL" => Some(g_POOL()),
            "POOL_CLAUSE" => Some(g_POOL_CLAUSE()),
            "TALK_ASKS" => Some(g_TALK_ASKS()),
            "WEIGHING" => Some(g_WEIGHING()),
            "WEIGHING_WORDS" => Some(g_WEIGHING_WORDS()),
            "_A_BARE_INVITATION" => Some(g__A_BARE_INVITATION()),
            "_A_LEAD_IN_FRONT" => Some(g__A_LEAD_IN_FRONT()),
            "_A_TIME_BEFORE_THE_THING" => Some(g__A_TIME_BEFORE_THE_THING()),
            "_CLAUSE_START" => Some(g__CLAUSE_START()),
            "_HEDGED_END" => Some(g__HEDGED_END()),
            "_LEAD" => Some(g__LEAD()),
            "_OWNERS" => Some(g__OWNERS()),
            "_POLITE" => Some(g__POLITE()),
            "_REPEATED_WORD" => Some(g__REPEATED_WORD()),
            "_SOMETHING_ELSE" => Some(g__SOMETHING_ELSE()),
            "_SUBJECT_END" => Some(g__SUBJECT_END()),
            "_SUBJECT_END_CLAUSE" => Some(g__SUBJECT_END_CLAUSE()),
            "_SUBJECT_HEAD" => Some(g__SUBJECT_HEAD()),
            "_SUBJECT_TAIL" => Some(g__SUBJECT_TAIL()),
            "_TALK" => Some(g__TALK()),
            "_TYPED_SHORT" => Some(g__TYPED_SHORT()),
            "_TYPED_SHORT_DICT" => Some(g__TYPED_SHORT_DICT()),
            "_VERB" => Some(g__VERB()),
            "_WHEN" => Some(g__WHEN()),
            "_as_typed" => Some(py_func("_as_typed", |args: &[Py]| _as_typed(args))),
            "_subject" => Some(py_func("_subject", |args: &[Py]| _subject(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "the_line_as_read" => Some(py_func("the_line_as_read", |args: &[Py]| the_line_as_read(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BLANK_SUBJECTS" => s_BLANK_SUBJECTS(v),
            "INVITED" => s_INVITED(v),
            "POOL" => s_POOL(v),
            "POOL_CLAUSE" => s_POOL_CLAUSE(v),
            "TALK_ASKS" => s_TALK_ASKS(v),
            "WEIGHING" => s_WEIGHING(v),
            "WEIGHING_WORDS" => s_WEIGHING_WORDS(v),
            "_A_BARE_INVITATION" => s__A_BARE_INVITATION(v),
            "_A_LEAD_IN_FRONT" => s__A_LEAD_IN_FRONT(v),
            "_A_TIME_BEFORE_THE_THING" => s__A_TIME_BEFORE_THE_THING(v),
            "_CLAUSE_START" => s__CLAUSE_START(v),
            "_HEDGED_END" => s__HEDGED_END(v),
            "_LEAD" => s__LEAD(v),
            "_OWNERS" => s__OWNERS(v),
            "_POLITE" => s__POLITE(v),
            "_REPEATED_WORD" => s__REPEATED_WORD(v),
            "_SOMETHING_ELSE" => s__SOMETHING_ELSE(v),
            "_SUBJECT_END" => s__SUBJECT_END(v),
            "_SUBJECT_END_CLAUSE" => s__SUBJECT_END_CLAUSE(v),
            "_SUBJECT_HEAD" => s__SUBJECT_HEAD(v),
            "_SUBJECT_TAIL" => s__SUBJECT_TAIL(v),
            "_TALK" => s__TALK(v),
            "_TYPED_SHORT" => s__TYPED_SHORT(v),
            "_TYPED_SHORT_DICT" => s__TYPED_SHORT_DICT(v),
            "_VERB" => s__VERB(v),
            "_WHEN" => s__WHEN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.talkto", module_obj);
    pyrt::xrefs::reg_fn("compose.talkto", "_as_typed", py_func("_as_typed", |args: &[Py]| _as_typed(args)));
    pyrt::xrefs::reg_fn("compose.talkto", "the_line_as_read", py_func("the_line_as_read", |args: &[Py]| the_line_as_read(args)));
    pyrt::xrefs::reg_fn("compose.talkto", "_subject", py_func("_subject", |args: &[Py]| _subject(args)));
    pyrt::xrefs::reg_fn("compose.talkto", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.talkto", "BLANK_SUBJECTS", g_BLANK_SUBJECTS, s_BLANK_SUBJECTS);
    pyrt::xrefs::reg_access("compose.talkto", "INVITED", g_INVITED, s_INVITED);
    pyrt::xrefs::reg_access("compose.talkto", "POOL", g_POOL, s_POOL);
    pyrt::xrefs::reg_access("compose.talkto", "POOL_CLAUSE", g_POOL_CLAUSE, s_POOL_CLAUSE);
    pyrt::xrefs::reg_access("compose.talkto", "TALK_ASKS", g_TALK_ASKS, s_TALK_ASKS);
    pyrt::xrefs::reg_access("compose.talkto", "WEIGHING", g_WEIGHING, s_WEIGHING);
    pyrt::xrefs::reg_access("compose.talkto", "WEIGHING_WORDS", g_WEIGHING_WORDS, s_WEIGHING_WORDS);
    pyrt::xrefs::reg_access("compose.talkto", "_A_BARE_INVITATION", g__A_BARE_INVITATION, s__A_BARE_INVITATION);
    pyrt::xrefs::reg_access("compose.talkto", "_A_LEAD_IN_FRONT", g__A_LEAD_IN_FRONT, s__A_LEAD_IN_FRONT);
    pyrt::xrefs::reg_access("compose.talkto", "_A_TIME_BEFORE_THE_THING", g__A_TIME_BEFORE_THE_THING, s__A_TIME_BEFORE_THE_THING);
    pyrt::xrefs::reg_access("compose.talkto", "_CLAUSE_START", g__CLAUSE_START, s__CLAUSE_START);
    pyrt::xrefs::reg_access("compose.talkto", "_HEDGED_END", g__HEDGED_END, s__HEDGED_END);
    pyrt::xrefs::reg_access("compose.talkto", "_LEAD", g__LEAD, s__LEAD);
    pyrt::xrefs::reg_access("compose.talkto", "_OWNERS", g__OWNERS, s__OWNERS);
    pyrt::xrefs::reg_access("compose.talkto", "_POLITE", g__POLITE, s__POLITE);
    pyrt::xrefs::reg_access("compose.talkto", "_REPEATED_WORD", g__REPEATED_WORD, s__REPEATED_WORD);
    pyrt::xrefs::reg_access("compose.talkto", "_SOMETHING_ELSE", g__SOMETHING_ELSE, s__SOMETHING_ELSE);
    pyrt::xrefs::reg_access("compose.talkto", "_SUBJECT_END", g__SUBJECT_END, s__SUBJECT_END);
    pyrt::xrefs::reg_access("compose.talkto", "_SUBJECT_END_CLAUSE", g__SUBJECT_END_CLAUSE, s__SUBJECT_END_CLAUSE);
    pyrt::xrefs::reg_access("compose.talkto", "_SUBJECT_HEAD", g__SUBJECT_HEAD, s__SUBJECT_HEAD);
    pyrt::xrefs::reg_access("compose.talkto", "_SUBJECT_TAIL", g__SUBJECT_TAIL, s__SUBJECT_TAIL);
    pyrt::xrefs::reg_access("compose.talkto", "_TALK", g__TALK, s__TALK);
    pyrt::xrefs::reg_access("compose.talkto", "_TYPED_SHORT", g__TYPED_SHORT, s__TYPED_SHORT);
    pyrt::xrefs::reg_access("compose.talkto", "_TYPED_SHORT_DICT", g__TYPED_SHORT_DICT, s__TYPED_SHORT_DICT);
    pyrt::xrefs::reg_access("compose.talkto", "_VERB", g__VERB, s__VERB);
    pyrt::xrefs::reg_access("compose.talkto", "_WHEN", g__WHEN, s__WHEN);
}
