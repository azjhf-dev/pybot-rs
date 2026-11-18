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


thread_local! { static G_ACTION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_BY_WHAT_THEY_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALL_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALL_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDED_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRIVE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMPLOYER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_KNOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_OLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_TELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPING_TRACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNOWING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNOWN_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOKS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENS_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_MODIFIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMEMBERING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETURNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAY_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCAFFOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPENDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPENT_ON_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_CONTRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELLS_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_THE_FACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORLD_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKING_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G___all__: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ACTION_ASK() -> Py { ensure_init(); G_ACTION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACTION_ASK(v: Py) { ensure_init(); G_ACTION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE() -> Py { ensure_init(); G_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE(v: Py) { ensure_init(); G_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_AVOID() -> Py { ensure_init(); G_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AVOID(v: Py) { ensure_init(); G_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_BY_WHAT_THEY_DO() -> Py { ensure_init(); G_A_PERSON_BY_WHAT_THEY_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_BY_WHAT_THEY_DO(v: Py) { ensure_init(); G_A_PERSON_BY_WHAT_THEY_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_CALL_A_THING() -> Py { ensure_init(); G_CALL_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALL_A_THING(v: Py) { ensure_init(); G_CALL_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_CALL_NAME() -> Py { ensure_init(); G_CALL_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALL_NAME(v: Py) { ensure_init(); G_CALL_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDED_ABOUT() -> Py { ensure_init(); G_DECIDED_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDED_ABOUT(v: Py) { ensure_init(); G_DECIDED_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDING() -> Py { ensure_init(); G_DECIDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDING(v: Py) { ensure_init(); G_DECIDING.with(|c| *c.borrow_mut() = v); }
pub fn g_DRIVE_ASK() -> Py { ensure_init(); G_DRIVE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRIVE_ASK(v: Py) { ensure_init(); G_DRIVE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_EMPLOYER_WORDS() -> Py { ensure_init(); G_EMPLOYER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMPLOYER_WORDS(v: Py) { ensure_init(); G_EMPLOYER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_KNOW() -> Py { ensure_init(); G_HOW_KNOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_KNOW(v: Py) { ensure_init(); G_HOW_KNOW.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_OLD() -> Py { ensure_init(); G_HOW_OLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_OLD(v: Py) { ensure_init(); G_HOW_OLD.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_TELL() -> Py { ensure_init(); G_HOW_TO_TELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_TELL(v: Py) { ensure_init(); G_HOW_TO_TELL.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPING_TRACK() -> Py { ensure_init(); G_KEEPING_TRACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPING_TRACK(v: Py) { ensure_init(); G_KEEPING_TRACK.with(|c| *c.borrow_mut() = v); }
pub fn g_KNOWING() -> Py { ensure_init(); G_KNOWING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNOWING(v: Py) { ensure_init(); G_KNOWING.with(|c| *c.borrow_mut() = v); }
pub fn g_KNOWN_OBJECT() -> Py { ensure_init(); G_KNOWN_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNOWN_OBJECT(v: Py) { ensure_init(); G_KNOWN_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMIT() -> Py { ensure_init(); G_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMIT(v: Py) { ensure_init(); G_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMIT_WORDS() -> Py { ensure_init(); G_LIMIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMIT_WORDS(v: Py) { ensure_init(); G_LIMIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVING() -> Py { ensure_init(); G_LIVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVING(v: Py) { ensure_init(); G_LIVING.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOKS_BACK() -> Py { ensure_init(); G_LOOKS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOKS_BACK(v: Py) { ensure_init(); G_LOOKS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ME() -> Py { ensure_init(); G_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ME(v: Py) { ensure_init(); G_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENS_WITH() -> Py { ensure_init(); G_OPENS_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENS_WITH(v: Py) { ensure_init(); G_OPENS_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE() -> Py { ensure_init(); G_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); G_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSONAL() -> Py { ensure_init(); G_PERSONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSONAL(v: Py) { ensure_init(); G_PERSONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_START() -> Py { ensure_init(); G_QUESTION_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_START(v: Py) { ensure_init(); G_QUESTION_START.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALLING() -> Py { ensure_init(); G_RECALLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALLING(v: Py) { ensure_init(); G_RECALLING.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_MODIFIERS() -> Py { ensure_init(); G_RECALL_MODIFIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_MODIFIERS(v: Py) { ensure_init(); G_RECALL_MODIFIERS.with(|c| *c.borrow_mut() = v); }
pub fn g_REMEMBERING() -> Py { ensure_init(); G_REMEMBERING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMEMBERING(v: Py) { ensure_init(); G_REMEMBERING.with(|c| *c.borrow_mut() = v); }
pub fn g_RETURNING() -> Py { ensure_init(); G_RETURNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETURNING(v: Py) { ensure_init(); G_RETURNING.with(|c| *c.borrow_mut() = v); }
pub fn g_SAY_NOW() -> Py { ensure_init(); G_SAY_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAY_NOW(v: Py) { ensure_init(); G_SAY_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_SCAFFOLD() -> Py { ensure_init(); G_SCAFFOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCAFFOLD(v: Py) { ensure_init(); G_SCAFFOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_SPENDING() -> Py { ensure_init(); G_SPENDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPENDING(v: Py) { ensure_init(); G_SPENDING.with(|c| *c.borrow_mut() = v); }
pub fn g_SPENT_ON_A_THING() -> Py { ensure_init(); G_SPENT_ON_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPENT_ON_A_THING(v: Py) { ensure_init(); G_SPENT_ON_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECTS() -> Py { ensure_init(); G_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECTS(v: Py) { ensure_init(); G_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_CONTRACTION() -> Py { ensure_init(); G_SUBJECT_CONTRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_CONTRACTION(v: Py) { ensure_init(); G_SUBJECT_CONTRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g_TELLS_APART() -> Py { ensure_init(); G_TELLS_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELLS_APART(v: Py) { ensure_init(); G_TELLS_APART.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORDS() -> Py { ensure_init(); G_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORDS(v: Py) { ensure_init(); G_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_USER() -> Py { ensure_init(); G_USER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USER(v: Py) { ensure_init(); G_USER.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_THE_FACT() -> Py { ensure_init(); G_WANTS_THE_FACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_THE_FACT(v: Py) { ensure_init(); G_WANTS_THE_FACT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHY() -> Py { ensure_init(); G_WHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHY(v: Py) { ensure_init(); G_WHY.with(|c| *c.borrow_mut() = v); }
pub fn g_WORLD_SUBJECT() -> Py { ensure_init(); G_WORLD_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORLD_SUBJECT(v: Py) { ensure_init(); G_WORLD_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__ACTION() -> Py { ensure_init(); G__ACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ACTION(v: Py) { ensure_init(); G__ACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING_FRAME() -> Py { ensure_init(); G__ASKING_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING_FRAME(v: Py) { ensure_init(); G__ASKING_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g___all__() -> Py { ensure_init(); G___all__.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s___all__(v: Py) { ensure_init(); G___all__.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Reading the ask in a question about what the user told you.\n\nThe memory store answered a fixed list of phrasings (\"what's my name\", \"what did I say my\nbudget was\"), so the same ask in other words fell through to the composer's \"I don't know\":\n\"what was my name\", \"do you remember my budget?\", \"where do I live\", \"you know my name?\".\n\nA recall question has one shape whatever its words: it asks about something of the user's and\nnames nothing else. So ask() strips the asking frame - the question word, the verb of saying or\nknowing, the politeness and the lead-in - and then reads what is left:\n\n* what remains is the user themselves (\"my\", \"I\") and one thing they told you (\"name\", \"budget\",\n  \"live\") -> that is the ask, and the store answers it.\n* anything else is left over (\"what should I cook with my dog?\") -> the line is a real request\n  and this passes.\n\nIt only reads; the store owns the answers (bot/modules/memory_store.py) and the facts\n(bot/memory/facts.py). Keys are the store's own fact keys.\n");
    s_SCAFFOLD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nwhat whats which who whom whose how when whens where why whether that this these those it its\nwas were is are am be been being do does did done doing have has had having\nyou u ya i im ive id ill me my mine our ours we us your yours dont doesnt didnt isnt wasnt\narent wont cant couldnt wouldnt\nagain also still yet now then just only even ever before already first last next other\nremember recall forget forgot know knew knews say said saying says tell told telling remind\nelse more again\nplease pls can could would will shall may might should must about of for to in at on\nwith by any anything something everything one ones thing things much many more most\nthe a an and or but so or right ok okay fine really actually honestly anyway well oh ah hmm\ncall called calls calling\nyou u ur your yours youve youre youd youll i me my mine we us our ours ya chat chats\nconversation talk\nup\ngot get gets getting gotten\ngive gimme hand\nfar\nmyself yourself yourselves ourselves themselves itself himself herself\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REMEMBERING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:remind|rememb|forget|forgot|know|knew|kno|say|said|say|tell|told|recall|summar|recap|remind|mention|learn|learnt|learned|studied|picked)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KNOWING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:know|knew|knows|knowing|remember|remembers|remembered|recall|recalls|summar(?:y|ise|ize)|recap|learn|learns|learned|learnt|learning|studied|pick|picks|picked|picking)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:ok|okay|k|kk|right|so|and|but|oh|ah|well|thanks|thank you|thx|ty|cheers|ta|hi|hello|hey|morning|yes|yeah|yep|no|nah|great|cool|nice|perfect|brilliant|lovely|good|sure|hmm|sorry|got it|i see|fair enough|makes sense)[\\s,.!?;:—-]+)+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RETURNING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:and\\s+)?(?:back|going\\s+back|coming\\s+back|returning)\\s+to\\s+(?:the|my|our|that|this|your)\\s+[a-z'’-]+|(?:about|on|re|regarding|for)\\s+(?:the|my|our|that|this|your)\\s+[a-z'’-]+|the\\s+[a-z'’-]+|(?:anyway|also|oh\\s+and|and\\s+also|separate\\s+thing)\\b)[\\s,.!?;:—-]*")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBJECTS(py_tuple(vec![py_tuple(vec![py_str("allergy"), py_tuple(vec![py_str("allergic"), py_str("allergy"), py_str("allergies"), py_str("intolerant"), py_str("intolerance"), py_str("avoid"), py_str("avoided"), py_str("avoiding")])]), py_tuple(vec![py_str("budget"), py_tuple(vec![py_str("budget"), py_str("spend"), py_str("spending"), py_str("spent"), py_str("afford"), py_str("cost")])]), py_tuple(vec![py_str("diet"), py_tuple(vec![py_str("diet"), py_str("diets"), py_str("dieting"), py_str("vegan"), py_str("vegetarian"), py_str("vegetarian"), py_str("pescatarian"), py_str("keto"), py_str("paleo")])]), py_tuple(vec![py_str("pet"), py_tuple(vec![py_str("pet"), py_str("pets"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("kitten"), py_str("rabbit"), py_str("hamster"), py_str("budgie"), py_str("parrot"), py_str("horse")])]), py_tuple(vec![py_str("birthday"), py_tuple(vec![py_str("birthday"), py_str("birthdays"), py_str("bday"), py_str("dob"), py_str("birthdate")])]), py_tuple(vec![py_str("weight"), py_tuple(vec![py_str("weigh"), py_str("weighs"), py_str("weight"), py_str("weights"), py_str("kilos"), py_str("kilo"), py_str("stone"), py_str("pounds"), py_str("heavy")])]), py_tuple(vec![py_str("height"), py_tuple(vec![py_str("height"), py_str("tall"), py_str("taller"), py_str("tallest"), py_str("centimetres"), py_str("centimetres"), py_str("cm")])]), py_tuple(vec![py_str("name"), py_tuple(vec![py_str("name"), py_str("named"), py_str("nickname")])]), py_tuple(vec![py_str("city"), py_tuple(vec![py_str("live"), py_str("lives"), py_str("living"), py_str("lived"), py_str("home"), py_str("based"), py_str("address"), py_str("resident")])]), py_tuple(vec![py_str("job"), py_tuple(vec![py_str("job"), py_str("jobs"), py_str("work"), py_str("works"), py_str("worked"), py_str("role"), py_str("career"), py_str("profession"), py_str("employer"), py_str("employed")])])]));
    s_LIMIT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\neat eats eating eaten have has having drink drinks drinking food foods meal meals\nbreakfast lunch dinner tea supper snack snacks pudding dessert anything else starchy sugary\navoid avoiding allowed ok okay safe should shouldnt shouldn not no though tho then\ncant cannot dont doesnt didn could couldnt mustnt shouldn would wouldnt anymore\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LIMIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:can't|cant|can not|cannot|don't|dont|do not|shouldn't|shouldnt|mustn't|mustnt|never|not|avoid|avoiding|no)\\b[^?.!]{0,24}?\\b(?:eat|eats|eating|have|has|having|drink|drinks|drinking|food|foods)\\b|\\b(?:eat|eats|eating|drink|drinks|drinking|food|foods)\\b[^?.!]{0,24}?\\b(?:can't|cant|cannot|not|shouldn't|shouldnt|avoid|avoiding)\\b|\\bwhat (?:should|shall|must|do) i (?:avoid|stay away from|keep away from)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\ndaughter daughters son sons kid kids child children baby toddler boy girl family wife husband\npartner mum mom mother dad father parent parents sister brother flatmate housemate friend\nfriends neighbour colleague boss\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RECALL_MODIFIERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("new"), py_str("old"), py_str("older"), py_str("current"), py_str("previous"), py_str("former")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_OLD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow old\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\babout me\\b|\\bof me\\b|\\bwhat have i (?:told|said|mentioned)\\b|\\bhave i (?:told|mentioned)\\b|\\bdid i (?:tell|say|mention)\\b|\\bsummar(?:y|ise|ize)\\b|\\brecap\\b|\\brun me through\\b|\\bwhat do you remember about (?:it|that|this)\\s*\\??$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_KNOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:do|does|did|would|will|can|could|should|might) (?:i|you|we) (?:\\w+ly )?(?:know|tell)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhy\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ADVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bshould\\b|\\bshall\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KEEPING_TRACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:do|can|could|should|would|to)\\s+(?:i|you|we|u)?\\s*(?:remember|keep\\s+track|track|stop\\s+forgetting|never\\s+forget)\\b|\\b(?:tips?|advice|ways?|tricks?|trick)\\b[^.?!]{0,30}?\\b(?:remember|remembering|keep\\s+track)\\b|\\bbest\\s+way\\s+to\\s+remember\\b|\\bis\\s+there\\s+a\\s+way\\s+to\\s+remember\\b|\\bhow\\s+to\\s+remember\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAY_NOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:do|can|could|would|will) (?:i|we) (?:say|tell)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ACTION(py_str("do|use|try|make|clean|cook|get|put|keep|fix|buy|say|start|stop|avoid|wear|take|give|move|store|wash|paint|grow|sort|plan|change|improve|prevent|protect|choose|pick|find|save|spend|eat|drink|add|remove|repair|replace|help"));
    s_ACTION_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:what|how|anything|something|where|is there)\\b[^?]{0,60}?\\b(?:i|we)\\s+(?:can|could|should|shall|would|will)\\s+(?:"), &g__ACTION()), &py_str(")\\b|\\b(?:what|how|anything|something)\\b[^?]{0,60}?\\b(?:can|could|should|shall|would|will)\\s+(?:i|we)\\s+(?:")), &g__ACTION()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RECALLING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:again|remind|reminds|remember|recall|was|were|did|said|told|knew)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORLD_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:will|would|can|could|may|might|should|shall|is|are|does|do)\\s+(?:a|an|the|my|our|your|this|that|these|those|any|one|two|both)\\s+\\S+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DECIDING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:can|could|should|shall|may|might|would)\\s+(?:we|i|you)\\s+(?:still\\s+)?(?:get|have|keep|adopt|buy|rescue|own|afford|take on)\\b(?=\\s+(?!(?:it|that|this|one|ones|them|those|these|more|anything)\\b)\\S)|\\bare\\s+(?:we|you|i)\\s+(?:allowed|able)\\s+to\\b|\\bis it (?:ok|okay|fine|wise|allowed)\\s+to\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DECIDED_ABOUT(py_tuple(vec![py_str("pet"), py_str("budget")]));
    s_PERSONAL(py_tuple(vec![py_str("name"), py_str("job"), py_str("city"), py_str("allergy"), py_str("budget"), py_str("diet"), py_str("birthday"), py_str("weight"), py_str("height")]));
    s_USER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|mine|i|i'm|im|i've|ive|i'd|id|i'll|ill|me|we|our|ours|us)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KNOWN_OBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:me|my|mine|myself)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CALL_NAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:am i|was i|do you|did you|would you) call(?:ed)?\\b|\\bcall me\\b|\\bi(?:'m| am| was)\\s+called\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CALL_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bcall(?:ed)?\\s+(?:a|an)\\s+[a-z]")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON_BY_WHAT_THEY_DO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bcall(?:ed)?\\s+(?:a|an)?\\s*(?:person|people|man|woman|boy|girl|child|kid|someone|somebody|thing|one|guy|bloke|lady)\\s+(?:who|that|which|whose)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bcall(?:ed)?\\s+(?:a|an)\\s+(?:person|people|man|woman|boy|girl|child|kid|someone|somebody|thing|one|guy|bloke|lady)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DRIVE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:do|did|am) i driv(?:e|ing)\\b|\\bwhich car\\b|\\bwhat car\\b|\\bwhat (?:kind|make|model) of car\\b|\\bwhat(?:'s|s| is| was) my (?:car|van)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AVOID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:have|has|had|need|needs|needed|got|ought) to avoid\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_TO_TELL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:do|can|should|would|could|will|might) (?:i|we|you) (?:know|tell|check|spot|see|notice|work out|find out|make sure)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TELLS_APART({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:when|if|whether|that)\\b|\\b(?:is|are|am)\\s+(?:\\w+\\s+){0,2}?(?:enough|ok|okay|happy|sad|ill|sick|unwell|in pain|ready|too|allergic|missing|done|safe|off)\\b|\\benough\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SPENT_ON_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:on|for)\\s+\\S.*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SPENDING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:spent|paid)\\b|\\b(?:did|have|has|had)\\s+(?:i|we)\\s+(?:spend|pay)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LIVING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfor a living\\b|\\bdo(?:es)? for a job\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EMPLOYER_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("employer"), py_str("employers"), py_str("employed"), py_str("employment")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANTS_THE_FACT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|which|who|whom|whose|where|when|how|why)\\b|\\bremind|\\bremember|\\bknow\\b|\\btell me\\b|\\bsay again\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TIME_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nweek weeks weekend weekends month months year years day days today tonight tomorrow yesterday\nmorning mornings afternoon afternoons evening evenings night nights midnight noon\nminute minutes hour hours second seconds ago fortnight fortnightly daily weekly monthly yearly\nmonday tuesday wednesday thursday friday saturday sunday\njanuary february march april may june july august september october november december\nearlier lately recently sometime once\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUESTION_START({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nwhat what's whats which who whom whose how when whens where why is are was were am do does did have\nhas can could would should will remind remember know tell say anything\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OPENS_WITH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*([A-Za-z]+['\\u2019][A-Za-z]+)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBJECT_CONTRACTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("we i they you he she it that there this these those"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s___all__(py_list(vec![py_str("ask"), py_str("SUBJECTS"), py_str("SCAFFOLD"), py_str("PEOPLE"), py_str("ME"), py_str("ADVICE")]));
    s_LOOKS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:again|remind|reminds|reminded|reminding|remember|remembers|remembered|recall|recalls|recalled|forget|forgets|forgot|forgotten)\\b|\\b(?:did|do|does|was|were|am|are|have|has|had)\\s+(?:i|we)\\b|\\bwhat\\s+(?:was|were)\\s+(?:my|our)\\b|\\bwhat(?:'s|s|\\s+is)\\s+(?:my|our)\\b|\\bhow\\s+(?:many|much)\\b|\\b(?:was|were)\\s+(?:that|it|they|those|these)\\b|\\b(?:you|u)\\s+(?:said|told)\\b|\\btold\\s+me\\b|\\b(?:earlier|before)\\b|\\bback\\s+to\\b|\\bsame\\s+(?:again|one)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKING_FRAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nagain remind reminds reminded reminding remember remembers remembered recall recalls recalled\nforget forgets forgot forgotten me my our us we i you u your yours the a an and or so but\nhow many much what which who when where why was were is are am do does did done have has had\nlong far old big tall heavy short wide deep high low often soon much\nsaid say says saying tell told telling earlier before back to same one ones that this it\nplease pls can could would will about of for in at on with by any anything something thing\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn asks_for_a_word(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this is the word for a thing rather than a name the chat was given.\n\n    The article is what tells the two apart: a recall points at what was said (\"call it\", \"call\n    him\", \"what am i called\", \"what did i call my cats\"), and a thing is named with \"a\" or \"an\".\n    A word for a person is not a fact the store or the entries hold, so it is neither.\n    ");
    line = { let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CALL_A_THING(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_BY_WHAT_THEY_DO(), "search", args))(&__args) } } })) } };
    Py::None
}
pub fn _key(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let _ = py_str("The fact key a content word asks about, or None.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::breeds::the_kind_of(args))(&__args) }) {
    return py_str("pet");
    }
    let __iter1 = py_iter_or_raise(&g_SUBJECTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    names = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&names.clone(), &word.clone()))) {
    return key.clone();
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&name.clone(), &Py::None, &Py::Int(4), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("alle"), py_str("budg")]), &slice_or_raise(&name.clone(), &Py::None, &Py::Int(4), &Py::None))) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return key.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_contraction_opens(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line opens with a subject folded into a verb: \"we're\", \"i've\", \"that's\".");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPENS_WITH(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_SUBJECT_CONTRACTION(), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }, &Py::Int(0))));
    Py::None
}
pub fn _asked(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let _ = py_str("Whether the line asks something at all: a statement never is a recall question.");
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("?")))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_QUESTION_START(), &geti(&low.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_contraction_opens(args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&low.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMEMBERING(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("do you"), py_str("did you"), py_str("you know"), py_str("did i"), py_str("do i"), py_str("can you")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&low.clone(), &Py::None, &Py::Int(2), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&g_QUESTION_START(), &geti(&low.clone(), &Py::Int(0))));
    Py::None
}
pub fn _asked_them(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("Whether the line asks the user a question about themselves, rather than about the bot.\n\n    \"what else have i said\" is the person; \"do you remember me\" is about the bot's memory of\n    them, and the store has a warmer answer for it than a list of facts.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("do"), py_str("did"), py_str("are"), py_str("can"), py_str("could"), py_str("would"), py_str("will")]), &geti(&low.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&low.clone(), &Py::None, &Py::Int(2), &Py::None), &py_str("you"))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_slot(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line names one of the store's own slots behind \"the\" rather than \"my\".\n\n    People say the thing back to the bot the way they would to someone who was listening: \"what\n    was the budget\", \"what was the name\", \"and the dog, whats he called\". The rule above wants\n    \"my\" or \"i\" in the line, so all of those were refused as questions about the world and the\n    answer was a hedge or a card about budgeting. Only the definite article in front of a slot\n    counts, so \"what is the budget for a wedding\" names a budget of nobody's and is still left\n    alone by the frame it is asked in (`ask` reads \"the\" only with a recalling word).\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("the"))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&i.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&low.clone(), &op_add(&i.clone(), &Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _key(args))(&__args) } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _without_the_lead(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The line with its pleasantries and its return-to-topic clause taken off the front, and\n    the slot that clause named, if it named one.\n\n    Both are in front of the question rather than part of it: \"thanks! what was my name\" and\n    \"back to the job, what was my job again\" ask the same thing, and reading the lead-in as if\n    it were the question made the second one ask nothing at all. The clause often names the\n    thing and then the question calls it \"it\" (\"about the budget, what was it again\"), so the\n    slot it named is handed back to be the answer when the question itself names none.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.!?;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD(), "sub", args))(&__args) }, "strip", args))(&__args) };
    named = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    _u = __item.clone();
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RETURNING(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&cut.clone()))) {
    break;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) };
    if py_truthy(&said.clone()) {
    named = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&said.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _key(args))(&__args) }; if py_truthy(&__b) { __b } else { named.clone() } };
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.!?;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    return py_tuple(vec![line.clone(), named.clone()]);
    Py::None
}
pub fn ask(args: &[Py]) -> Py {
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
    let mut why: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "why") { v.clone() }
    else { Py::Bool(false) };
    let mut _u: Py = Py::None;
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut content: Py = Py::None;
    let mut how_to: Py = Py::None;
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut modifierless: Py = Py::None;
    let mut returning: Py = Py::None;
    let mut spend_tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What the user wants to be reminded of: a fact key (\"name\", \"job\", \"city\", \"pet\",\n    \"allergy\", \"budget\"), or None when the line is not a question about their own facts.\n\n    A why-line is not one of those questions: the cause it wants is in the world, and the store\n    only keeps what they said. `why=True` is for the caller that has their facts in hand and\n    only wants the subject named (\"why do we have pets\" after \"my dog is called rex\"), which is\n    where the store does have something to say.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_the_lead(args))(&__args) }, 2, None);
    line = __unpk1[0].clone();
    returning = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(12)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&why.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_KEEPING_TRACK(), "search", args))(&__args) }) {
    return Py::None;
    }
    limit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LIMIT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ADVICE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAY_NOW(), "search", args))(&__args) } } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALLING(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&limit.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ACTION_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALLING(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&limit.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:much|many)\\b|\\bfor a living\\b")); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } } } } } }) {
    return Py::None;
    }
    how_to = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_TO_TELL(), "search", args))(&__args) };
    if py_truthy(&{ let __b = how_to.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&how_to.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TELLS_APART(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DRIVE_ASK(), "search", args))(&__args) }) {
    return py_str("car");
    }
    spend_tail = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPENDING(), "search", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPENT_ON_A_THING(), "search", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = spend_tail.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend_tail.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) } } }) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend_tail.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) };
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked(args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CALL_NAME(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_for_a_word(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_BY_WHAT_THEY_DO(), "search", args))(&__args) })) } } } }) {
    return py_str("name");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AVOID(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_USER(), "search", args))(&__args) } } }) {
    return py_str("allergy");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bdate of birth\\b|\\bbirth date\\b")); __args.push(line.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("birthday");
    }
    content = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_SCAFFOLD(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_TIME_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMEMBERING(), "match", args))(&__args) })) } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = content.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WANTS_THE_FACT(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_EMPLOYER_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&content.clone()))) {
    if py_truthy(&returning.clone()) {
    return returning.clone();
    }
    if py_truthy(&limit.clone()) {
    return py_str("limit");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_KNOW(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ME(), "search", args))(&__args) }) {
    return py_str("me");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_USER(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_them(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_KNOWING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_KNOWN_OBJECT(), "search", args))(&__args) } } } } } } }) {
    return py_str("me");
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_USER(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALLING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_slot(args))(&__args) } } })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LIVING(), "search", args))(&__args) }) {
    return py_str("job");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PEOPLE(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = limit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_LIMIT_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() })) } }) {
    return py_str("limit");
    }
    modifierless = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(content.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    word = __unpk3[1].clone();
    after = if py_truthy(&op_lt(&op_add(&at.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(content.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&content.clone(), &op_add(&at.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_RECALL_MODIFIERS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _key(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("old"), py_str("older")]), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_OLD(), "search", args))(&__args) } } })) } } } } } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&modifierless.clone(), "append", args))(&__args) };
    }
    content = modifierless.clone();
    keys = py_list(vec![]);
    let __iter4 = py_iter_or_raise(&content.clone());
    for __item in __iter4 {
    word = __item.clone();
    key = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _key(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return Py::None;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&keys.clone(), "append", args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DECIDING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALLING(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DECIDED_ABOUT()) { let mut key: Py = __item.clone(); __out.push(Py::Bool(py_contains(&keys.clone(), &key.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORLD_SUBJECT(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALLING(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_PERSONAL(), &key.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    return Py::None;
    }
    let __iter5 = py_iter_or_raise(&g_SUBJECTS());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    key = __unpk6[0].clone();
    _u = __unpk6[1].clone();
    if py_truthy(&Py::Bool(py_contains(&keys.clone(), &key.clone()))) {
    return key.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn looks_back(args: &[Py]) -> Py {
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
    let _ = py_str("The words that make this a look-back line, or \"\" when the line stands on its own.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_str("");
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LOOKS_BACK(), "search", args))(&__args) };
    return if py_truthy(&found.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _stems(args: &[Py]) -> Py {
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
    let mut stem: Py = py_func("stem", |args: &[Py]| crate::memory::topics::stem(args));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&stem, args))(&__args) }); } } py_set(__out) })() };
    Py::None
}
pub fn the_earlier_turn(args: &[Py]) -> Py {
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
    let mut cue: Py = Py::None;
    let mut everywhere: Py = Py::None;
    let mut have: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut unsaid: Py = Py::None;
    let mut want: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The earlier turn a look-back line points at: {\"cue\", \"thing\", \"said\"} or None.\n\n    This is the check every reply from memory goes through, and it answers the question the way it\n    should be asked: *is there an earlier turn carrying the thing this line names?* There is no\n    look-back when there is nothing behind it, so None is returned both for a line that stands on\n    its own and for one that points at something never said (\"remind me how many nights in\n    portugal\" when no night in portugal was ever mentioned).\n\n    The thing is the line's own content words with the asking frame stripped - \"remind me how many\n    nights\" points at the nights, \"what was my deposit again\" at the deposit - and a turn counts\n    when it carries every one of them, so a line that adds a word never said is not pointed at the\n    turn that lacks it. `said` is the person's own earlier lines that carry the thing, newest\n    first, at most three.\n    ");
    cue = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| looks_back(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&cue.clone()))) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__ASKING_FRAME(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    want = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stems(args))(&__args) };
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &Py::None, &Py::None, &op_neg(&Py::Int(1)))) { let mut turn: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) } } }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() };
    said = slice_or_raise(&said.clone(), &Py::None, &Py::Int(3), &Py::None);
    return if py_truthy(&said.clone()) { py_dict(vec![(py_str("cue"), cue.clone()), (py_str("thing"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| ask(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }), (py_str("said"), said.clone())]) } else { Py::None };
    }
    everywhere = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __aug2 = op_bitor(&everywhere.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stems(args))(&__args) });
    everywhere = __aug2;
    }
    }
    unsaid = op_sub(&want.clone(), &everywhere.clone());
    said = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    turn = __item.clone();
    line = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    have = if py_truthy(&line.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stems(args))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&want.clone(), &have.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { op_le(&op_sub(&want.clone(), &have.clone()), &unsaid.clone()) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    } else {
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { op_le(&want.clone(), &have.clone()) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return py_dict(vec![(py_str("cue"), cue.clone()), (py_str("thing"), thing.clone()), (py_str("said"), said.clone())]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.recall".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACTION_ASK" => Some(g_ACTION_ASK()),
            "ADVICE" => Some(g_ADVICE()),
            "AVOID" => Some(g_AVOID()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_PERSON_BY_WHAT_THEY_DO" => Some(g_A_PERSON_BY_WHAT_THEY_DO()),
            "CALL_A_THING" => Some(g_CALL_A_THING()),
            "CALL_NAME" => Some(g_CALL_NAME()),
            "DECIDED_ABOUT" => Some(g_DECIDED_ABOUT()),
            "DECIDING" => Some(g_DECIDING()),
            "DRIVE_ASK" => Some(g_DRIVE_ASK()),
            "EMPLOYER_WORDS" => Some(g_EMPLOYER_WORDS()),
            "HOW_KNOW" => Some(g_HOW_KNOW()),
            "HOW_OLD" => Some(g_HOW_OLD()),
            "HOW_TO_TELL" => Some(g_HOW_TO_TELL()),
            "KEEPING_TRACK" => Some(g_KEEPING_TRACK()),
            "KNOWING" => Some(g_KNOWING()),
            "KNOWN_OBJECT" => Some(g_KNOWN_OBJECT()),
            "LEAD" => Some(g_LEAD()),
            "LIMIT" => Some(g_LIMIT()),
            "LIMIT_WORDS" => Some(g_LIMIT_WORDS()),
            "LIVING" => Some(g_LIVING()),
            "LOOKS_BACK" => Some(g_LOOKS_BACK()),
            "ME" => Some(g_ME()),
            "OPENS_WITH" => Some(g_OPENS_WITH()),
            "PEOPLE" => Some(g_PEOPLE()),
            "PERSONAL" => Some(g_PERSONAL()),
            "QUESTION_START" => Some(g_QUESTION_START()),
            "RECALLING" => Some(g_RECALLING()),
            "RECALL_MODIFIERS" => Some(g_RECALL_MODIFIERS()),
            "REMEMBERING" => Some(g_REMEMBERING()),
            "RETURNING" => Some(g_RETURNING()),
            "SAY_NOW" => Some(g_SAY_NOW()),
            "SCAFFOLD" => Some(g_SCAFFOLD()),
            "SPENDING" => Some(g_SPENDING()),
            "SPENT_ON_A_THING" => Some(g_SPENT_ON_A_THING()),
            "SUBJECTS" => Some(g_SUBJECTS()),
            "SUBJECT_CONTRACTION" => Some(g_SUBJECT_CONTRACTION()),
            "TELLS_APART" => Some(g_TELLS_APART()),
            "TIME_WORDS" => Some(g_TIME_WORDS()),
            "USER" => Some(g_USER()),
            "WANTS_THE_FACT" => Some(g_WANTS_THE_FACT()),
            "WHY" => Some(g_WHY()),
            "WORLD_SUBJECT" => Some(g_WORLD_SUBJECT()),
            "_ACTION" => Some(g__ACTION()),
            "_ASKING_FRAME" => Some(g__ASKING_FRAME()),
            "__all__" => Some(g___all__()),
            "_a_contraction_opens" => Some(py_func("_a_contraction_opens", |args: &[Py]| _a_contraction_opens(args))),
            "_asked" => Some(py_func("_asked", |args: &[Py]| _asked(args))),
            "_asked_them" => Some(py_func("_asked_them", |args: &[Py]| _asked_them(args))),
            "_key" => Some(py_func("_key", |args: &[Py]| _key(args))),
            "_stems" => Some(py_func("_stems", |args: &[Py]| _stems(args))),
            "_the_slot" => Some(py_func("_the_slot", |args: &[Py]| _the_slot(args))),
            "_without_the_lead" => Some(py_func("_without_the_lead", |args: &[Py]| _without_the_lead(args))),
            "ask" => Some(py_func("ask", |args: &[Py]| ask(args))),
            "asks_for_a_word" => Some(py_func("asks_for_a_word", |args: &[Py]| asks_for_a_word(args))),
            "looks_back" => Some(py_func("looks_back", |args: &[Py]| looks_back(args))),
            "the_earlier_turn" => Some(py_func("the_earlier_turn", |args: &[Py]| the_earlier_turn(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACTION_ASK" => s_ACTION_ASK(v),
            "ADVICE" => s_ADVICE(v),
            "AVOID" => s_AVOID(v),
            "A_PERSON" => s_A_PERSON(v),
            "A_PERSON_BY_WHAT_THEY_DO" => s_A_PERSON_BY_WHAT_THEY_DO(v),
            "CALL_A_THING" => s_CALL_A_THING(v),
            "CALL_NAME" => s_CALL_NAME(v),
            "DECIDED_ABOUT" => s_DECIDED_ABOUT(v),
            "DECIDING" => s_DECIDING(v),
            "DRIVE_ASK" => s_DRIVE_ASK(v),
            "EMPLOYER_WORDS" => s_EMPLOYER_WORDS(v),
            "HOW_KNOW" => s_HOW_KNOW(v),
            "HOW_OLD" => s_HOW_OLD(v),
            "HOW_TO_TELL" => s_HOW_TO_TELL(v),
            "KEEPING_TRACK" => s_KEEPING_TRACK(v),
            "KNOWING" => s_KNOWING(v),
            "KNOWN_OBJECT" => s_KNOWN_OBJECT(v),
            "LEAD" => s_LEAD(v),
            "LIMIT" => s_LIMIT(v),
            "LIMIT_WORDS" => s_LIMIT_WORDS(v),
            "LIVING" => s_LIVING(v),
            "LOOKS_BACK" => s_LOOKS_BACK(v),
            "ME" => s_ME(v),
            "OPENS_WITH" => s_OPENS_WITH(v),
            "PEOPLE" => s_PEOPLE(v),
            "PERSONAL" => s_PERSONAL(v),
            "QUESTION_START" => s_QUESTION_START(v),
            "RECALLING" => s_RECALLING(v),
            "RECALL_MODIFIERS" => s_RECALL_MODIFIERS(v),
            "REMEMBERING" => s_REMEMBERING(v),
            "RETURNING" => s_RETURNING(v),
            "SAY_NOW" => s_SAY_NOW(v),
            "SCAFFOLD" => s_SCAFFOLD(v),
            "SPENDING" => s_SPENDING(v),
            "SPENT_ON_A_THING" => s_SPENT_ON_A_THING(v),
            "SUBJECTS" => s_SUBJECTS(v),
            "SUBJECT_CONTRACTION" => s_SUBJECT_CONTRACTION(v),
            "TELLS_APART" => s_TELLS_APART(v),
            "TIME_WORDS" => s_TIME_WORDS(v),
            "USER" => s_USER(v),
            "WANTS_THE_FACT" => s_WANTS_THE_FACT(v),
            "WHY" => s_WHY(v),
            "WORLD_SUBJECT" => s_WORLD_SUBJECT(v),
            "_ACTION" => s__ACTION(v),
            "_ASKING_FRAME" => s__ASKING_FRAME(v),
            "__all__" => s___all__(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.recall", module_obj);
    pyrt::xrefs::reg_fn("memory.recall", "asks_for_a_word", py_func("asks_for_a_word", |args: &[Py]| asks_for_a_word(args)));
    pyrt::xrefs::reg_fn("memory.recall", "_key", py_func("_key", |args: &[Py]| _key(args)));
    pyrt::xrefs::reg_fn("memory.recall", "_a_contraction_opens", py_func("_a_contraction_opens", |args: &[Py]| _a_contraction_opens(args)));
    pyrt::xrefs::reg_fn("memory.recall", "_asked", py_func("_asked", |args: &[Py]| _asked(args)));
    pyrt::xrefs::reg_fn("memory.recall", "_asked_them", py_func("_asked_them", |args: &[Py]| _asked_them(args)));
    pyrt::xrefs::reg_fn("memory.recall", "_the_slot", py_func("_the_slot", |args: &[Py]| _the_slot(args)));
    pyrt::xrefs::reg_fn("memory.recall", "_without_the_lead", py_func("_without_the_lead", |args: &[Py]| _without_the_lead(args)));
    pyrt::xrefs::reg_fn("memory.recall", "ask", py_func("ask", |args: &[Py]| ask(args)));
    pyrt::xrefs::reg_fn("memory.recall", "looks_back", py_func("looks_back", |args: &[Py]| looks_back(args)));
    pyrt::xrefs::reg_fn("memory.recall", "_stems", py_func("_stems", |args: &[Py]| _stems(args)));
    pyrt::xrefs::reg_fn("memory.recall", "the_earlier_turn", py_func("the_earlier_turn", |args: &[Py]| the_earlier_turn(args)));
    pyrt::xrefs::reg_access("memory.recall", "ACTION_ASK", g_ACTION_ASK, s_ACTION_ASK);
    pyrt::xrefs::reg_access("memory.recall", "ADVICE", g_ADVICE, s_ADVICE);
    pyrt::xrefs::reg_access("memory.recall", "AVOID", g_AVOID, s_AVOID);
    pyrt::xrefs::reg_access("memory.recall", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("memory.recall", "A_PERSON_BY_WHAT_THEY_DO", g_A_PERSON_BY_WHAT_THEY_DO, s_A_PERSON_BY_WHAT_THEY_DO);
    pyrt::xrefs::reg_access("memory.recall", "CALL_A_THING", g_CALL_A_THING, s_CALL_A_THING);
    pyrt::xrefs::reg_access("memory.recall", "CALL_NAME", g_CALL_NAME, s_CALL_NAME);
    pyrt::xrefs::reg_access("memory.recall", "DECIDED_ABOUT", g_DECIDED_ABOUT, s_DECIDED_ABOUT);
    pyrt::xrefs::reg_access("memory.recall", "DECIDING", g_DECIDING, s_DECIDING);
    pyrt::xrefs::reg_access("memory.recall", "DRIVE_ASK", g_DRIVE_ASK, s_DRIVE_ASK);
    pyrt::xrefs::reg_access("memory.recall", "EMPLOYER_WORDS", g_EMPLOYER_WORDS, s_EMPLOYER_WORDS);
    pyrt::xrefs::reg_access("memory.recall", "HOW_KNOW", g_HOW_KNOW, s_HOW_KNOW);
    pyrt::xrefs::reg_access("memory.recall", "HOW_OLD", g_HOW_OLD, s_HOW_OLD);
    pyrt::xrefs::reg_access("memory.recall", "HOW_TO_TELL", g_HOW_TO_TELL, s_HOW_TO_TELL);
    pyrt::xrefs::reg_access("memory.recall", "KEEPING_TRACK", g_KEEPING_TRACK, s_KEEPING_TRACK);
    pyrt::xrefs::reg_access("memory.recall", "KNOWING", g_KNOWING, s_KNOWING);
    pyrt::xrefs::reg_access("memory.recall", "KNOWN_OBJECT", g_KNOWN_OBJECT, s_KNOWN_OBJECT);
    pyrt::xrefs::reg_access("memory.recall", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("memory.recall", "LIMIT", g_LIMIT, s_LIMIT);
    pyrt::xrefs::reg_access("memory.recall", "LIMIT_WORDS", g_LIMIT_WORDS, s_LIMIT_WORDS);
    pyrt::xrefs::reg_access("memory.recall", "LIVING", g_LIVING, s_LIVING);
    pyrt::xrefs::reg_access("memory.recall", "LOOKS_BACK", g_LOOKS_BACK, s_LOOKS_BACK);
    pyrt::xrefs::reg_access("memory.recall", "ME", g_ME, s_ME);
    pyrt::xrefs::reg_access("memory.recall", "OPENS_WITH", g_OPENS_WITH, s_OPENS_WITH);
    pyrt::xrefs::reg_access("memory.recall", "PEOPLE", g_PEOPLE, s_PEOPLE);
    pyrt::xrefs::reg_access("memory.recall", "PERSONAL", g_PERSONAL, s_PERSONAL);
    pyrt::xrefs::reg_access("memory.recall", "QUESTION_START", g_QUESTION_START, s_QUESTION_START);
    pyrt::xrefs::reg_access("memory.recall", "RECALLING", g_RECALLING, s_RECALLING);
    pyrt::xrefs::reg_access("memory.recall", "RECALL_MODIFIERS", g_RECALL_MODIFIERS, s_RECALL_MODIFIERS);
    pyrt::xrefs::reg_access("memory.recall", "REMEMBERING", g_REMEMBERING, s_REMEMBERING);
    pyrt::xrefs::reg_access("memory.recall", "RETURNING", g_RETURNING, s_RETURNING);
    pyrt::xrefs::reg_access("memory.recall", "SAY_NOW", g_SAY_NOW, s_SAY_NOW);
    pyrt::xrefs::reg_access("memory.recall", "SCAFFOLD", g_SCAFFOLD, s_SCAFFOLD);
    pyrt::xrefs::reg_access("memory.recall", "SPENDING", g_SPENDING, s_SPENDING);
    pyrt::xrefs::reg_access("memory.recall", "SPENT_ON_A_THING", g_SPENT_ON_A_THING, s_SPENT_ON_A_THING);
    pyrt::xrefs::reg_access("memory.recall", "SUBJECTS", g_SUBJECTS, s_SUBJECTS);
    pyrt::xrefs::reg_access("memory.recall", "SUBJECT_CONTRACTION", g_SUBJECT_CONTRACTION, s_SUBJECT_CONTRACTION);
    pyrt::xrefs::reg_access("memory.recall", "TELLS_APART", g_TELLS_APART, s_TELLS_APART);
    pyrt::xrefs::reg_access("memory.recall", "TIME_WORDS", g_TIME_WORDS, s_TIME_WORDS);
    pyrt::xrefs::reg_access("memory.recall", "USER", g_USER, s_USER);
    pyrt::xrefs::reg_access("memory.recall", "WANTS_THE_FACT", g_WANTS_THE_FACT, s_WANTS_THE_FACT);
    pyrt::xrefs::reg_access("memory.recall", "WHY", g_WHY, s_WHY);
    pyrt::xrefs::reg_access("memory.recall", "WORLD_SUBJECT", g_WORLD_SUBJECT, s_WORLD_SUBJECT);
    pyrt::xrefs::reg_access("memory.recall", "_ACTION", g__ACTION, s__ACTION);
    pyrt::xrefs::reg_access("memory.recall", "_ASKING_FRAME", g__ASKING_FRAME, s__ASKING_FRAME);
    pyrt::xrefs::reg_access("memory.recall", "__all__", g___all__, s___all__);
}
