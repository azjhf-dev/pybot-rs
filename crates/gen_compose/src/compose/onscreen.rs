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


thread_local! { static G_ASKS_RATHER_THAN_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHAT_DECIDES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHAT_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BUY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DECISION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GO_ON_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HELP_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIMIT_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LONG_SENTENCE_SPLIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PURCHASE_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NAMED_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORD_THAT_ONLY_JOINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_AFTER_A_COLON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EFFORT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMES_NOTHING_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_TO_HAND_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_SCREEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_SCREEN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STANDS_BY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_THINGS_IT_COULD_BE_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_TO_TALK_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_THE_CONVERSATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DOUBLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKS_RATHER_THAN_NAMES() -> Py { ensure_init(); G_ASKS_RATHER_THAN_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_RATHER_THAN_NAMES(v: Py) { ensure_init(); G_ASKS_RATHER_THAN_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHAT_DECIDES() -> Py { ensure_init(); G_ASKS_WHAT_DECIDES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHAT_DECIDES(v: Py) { ensure_init(); G_ASKS_WHAT_DECIDES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHAT_TO_DO() -> Py { ensure_init(); G_ASKS_WHAT_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHAT_TO_DO(v: Py) { ensure_init(); G_ASKS_WHAT_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BUY_ASK() -> Py { ensure_init(); G_A_BUY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BUY_ASK(v: Py) { ensure_init(); G_A_BUY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DECISION_ASK() -> Py { ensure_init(); G_A_DECISION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DECISION_ASK(v: Py) { ensure_init(); G_A_DECISION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GO_ON_ASK() -> Py { ensure_init(); G_A_GO_ON_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GO_ON_ASK(v: Py) { ensure_init(); G_A_GO_ON_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HELP_IN_IT() -> Py { ensure_init(); G_A_HELP_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HELP_IN_IT(v: Py) { ensure_init(); G_A_HELP_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LEAD_IN() -> Py { ensure_init(); G_A_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEAD_IN(v: Py) { ensure_init(); G_A_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIMIT_IN_IT() -> Py { ensure_init(); G_A_LIMIT_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIMIT_IN_IT(v: Py) { ensure_init(); G_A_LIMIT_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LONG_SENTENCE_SPLIT() -> Py { ensure_init(); G_A_LONG_SENTENCE_SPLIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LONG_SENTENCE_SPLIT(v: Py) { ensure_init(); G_A_LONG_SENTENCE_SPLIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PURCHASE_IN_IT() -> Py { ensure_init(); G_A_PURCHASE_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PURCHASE_IN_IT(v: Py) { ensure_init(); G_A_PURCHASE_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_IN_IT() -> Py { ensure_init(); G_A_STEP_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_IN_IT(v: Py) { ensure_init(); G_A_STEP_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NAMED_IN_FRONT() -> Py { ensure_init(); G_A_THING_NAMED_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED_IN_FRONT(v: Py) { ensure_init(); G_A_THING_NAMED_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORD_THAT_ONLY_JOINS() -> Py { ensure_init(); G_A_WORD_THAT_ONLY_JOINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORD_THAT_ONLY_JOINS(v: Py) { ensure_init(); G_A_WORD_THAT_ONLY_JOINS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_AFTER_A_COLON() -> Py { ensure_init(); G_CLAUSE_AFTER_A_COLON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_AFTER_A_COLON(v: Py) { ensure_init(); G_CLAUSE_AFTER_A_COLON.with(|c| *c.borrow_mut() = v); }
pub fn g_EFFORT_WORDS() -> Py { ensure_init(); G_EFFORT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EFFORT_WORDS(v: Py) { ensure_init(); G_EFFORT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_SENTENCE() -> Py { ensure_init(); G_LONG_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_SENTENCE(v: Py) { ensure_init(); G_LONG_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMES_NOTHING_OF_ITS_OWN() -> Py { ensure_init(); G_NAMES_NOTHING_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMES_NOTHING_OF_ITS_OWN(v: Py) { ensure_init(); G_NAMES_NOTHING_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_TO_HAND_BACK() -> Py { ensure_init(); G_NOT_TO_HAND_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_TO_HAND_BACK(v: Py) { ensure_init(); G_NOT_TO_HAND_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_SCREEN_ASK() -> Py { ensure_init(); G_ON_SCREEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_SCREEN_ASK(v: Py) { ensure_init(); G_ON_SCREEN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_SCREEN_LINES() -> Py { ensure_init(); G_ON_SCREEN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_SCREEN_LINES(v: Py) { ensure_init(); G_ON_SCREEN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTED() -> Py { ensure_init(); G_QUOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTED(v: Py) { ensure_init(); G_QUOTED.with(|c| *c.borrow_mut() = v); }
pub fn g_STANDS_BY_LINES() -> Py { ensure_init(); G_STANDS_BY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STANDS_BY_LINES(v: Py) { ensure_init(); G_STANDS_BY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_THINGS_IT_COULD_BE_ABOUT() -> Py { ensure_init(); G_THE_THINGS_IT_COULD_BE_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_THINGS_IT_COULD_BE_ABOUT(v: Py) { ensure_init(); G_THE_THINGS_IT_COULD_BE_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_TO_TALK_TO() -> Py { ensure_init(); G_WHO_TO_TALK_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_TO_TALK_TO(v: Py) { ensure_init(); G_WHO_TO_TALK_TO.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_THE_CONVERSATION() -> Py { ensure_init(); G__ABOUT_THE_CONVERSATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_THE_CONVERSATION(v: Py) { ensure_init(); G__ABOUT_THE_CONVERSATION.with(|c| *c.borrow_mut() = v); }
pub fn g__DOUBLED() -> Py { ensure_init(); G__DOUBLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DOUBLED(v: Py) { ensure_init(); G__DOUBLED.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up about the answer the bot has just given is answered out of that answer.\n\n    how do i get rid of the smell in the washing machine\n      \"The black lives in the fold of the door seal: pull it open, wipe the fold with a cloth and\n       a little white vinegar or a mould spray, and dry it. Leaving the door and the drawer open\n       between washes is what stops the smell coming back.\"\n    what should i do about the smell\n      \"That's beyond what I know well, sorry. Searching for it will turn up better sources than I\n       can offer; tell me what you find and I'll help you make sense of it.\"\n\nNo entry claims the second line and no module does either, so it reached the search hedge - and\nthe hedge is not true there, because the answer is on the person's screen already. The same on\n\"should i cut the leaves off\" after the powdery mildew answer (\"Not mine to decide, sorry\") and on\n\"does that mean i should stop watering\" after the white mould one. What the person wants is the\npart of that answer their line is about, so the reply hands back the sentence of it that carries\nthe words they asked with - the sentence, not the whole answer, because the answer is a paragraph\nand they have read it.\n\nThree things have to hold, and each is a way of not taking a turn that has a better answer: the\nlast turn was a knowledge answer (`_the_answer_to_hand_back` - not a hedge, and not one of the\ncomposer's own notes, which add no fact), the line names no thing of its own (`what about a\nhelmet` wants a helmet, and no sentence of the bike answer gives one), and there is a sentence of\nthe answer worth handing back.\n\nIt is asked from `bot/modules/greeting.py`, last, and only in place of a hedge: the composer's\nrepeat card (\"that is all of what I know on it\") is a real answer to the same question asked\ntwice, and the arbiter has rules for it (`bot/modules/arbiter.py`).\n");
    s_ON_SCREEN_LINES(py_tuple(vec![py_str("That is in the answer above: {said}"), py_str("Same answer: {said}"), py_str("It is in what I said above: {said}")]));
    s_STANDS_BY_LINES(py_tuple(vec![py_str("I'd go by what I said: {said}"), py_str("I still stand by this: {said}"), py_str("Same answer, and it holds: {said}")]));
    s_ON_SCREEN_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|and|but|ok|okay|then|also)?\\s*,?\\s*(?:what|how|which|where|when|who|why|should|shall|can|could|do|does|did|is|are|will|would|any)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_NAMED_IN_FRONT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|and|but|ok|okay|then|also)?\\s*,?\\s*what about\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_STEP_IN_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:check|clean|clear|call|ring|ask|take|put|add|set|switch|turn|leave|keep|try|use|replace|restart|update|move|fix|buy|get|book|look|make|run|wash|start|plug|test|wipe|pull|open|close|soak|water|cut|pick|apply|scrub|hoover|vacuum|cover|store|wrap|rinse|push|carry|plough|plow|soldier)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_TO_HAND_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:hope\\b|good luck|let me know|take care|all the best|sorry\\b|i'm sorry|im sorry|i don't know|i dont know|i'm not sure|im not sure|i can't|i cant|i would|i'd)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_WHAT_TO_DO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what (?:should|do|can) i do|what do i do|how do i|how can i|should i|do i need to|does that mean|will that|is that enough|is it enough|how much|how long|how often)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POINTS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|its|it's|that|this|they|them)\\b|\\b(?:about|with) (?:the|my|our|his|her|their)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMES_NOTHING_OF_ITS_OWN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("look"), py_str("expect"), py_str("watch"), py_str("take"), py_str("bring"), py_str("wear"), py_str("say"), py_str("ask"), py_str("try"), py_str("buy"), py_str("get"), py_str("need"), py_str("want"), py_str("now"), py_str("next"), py_str("first"), py_str("else"), py_str("then"), py_str("there"), py_str("anything"), py_str("something"), py_str("work"), py_str("works"), py_str("actually"), py_str("really"), py_str("truly"), py_str("genuinely"), py_str("seriously"), py_str("even"), py_str("ever"), py_str("honestly"), py_str("help"), py_str("helps"), py_str("helping"), py_str("true"), py_str("right"), py_str("correct"), py_str("wrong"), py_str("case"), py_str("sure")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_RATHER_THAN_NAMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nmean means meant stop stops started start keep keeps keeping give gives need needs\nhelp helps try tries use uses used work works working do does doing done happen happening\nhappens enough better worse best long much many often soon cost costs price way time point\nanyway else instead still just really actually suppose supposed about around\noff out up down over back away on in through along ahead\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_THINGS_IT_COULD_BE_ABOUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nsmell smells smelly noise noisy sound sounds leak leaks leaking damp mould mold mildew stain\nstains colour color taste tastes itch itchy ache aches pain pains mess dirty dirt rust rusty\nwater wet burning squeak squeaks rattle rattles bang banging drip dripping dust\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHO_TO_TALK_TO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:who can i (?:talk|speak|ring|call)|who should i (?:talk|speak|ring|call)|who do i (?:talk|speak|ring|call)|where can i get help|who can help me)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_HELP_IN_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:crisis line|helpline|hotline|samaritans|116 ?123|988|999|111|911|emergency number|a and e|a&e|gp|doctor|nurse|midwife|health visitor|pharmacist|social worker|ring|call|phone)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BUY_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:do i need to buy|need to buy|anything to buy|what do i need to buy|what to buy|will it cost|how much (?:is|does|will)|is it (?:cheap|expensive)|worth buying)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PURCHASE_IN_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:buy|buys|buying|bought|cost|costs|price|prices|pound|pounds|dollar|dollars|cheap|cheaper|expensive|order|shop|store|supermarket|pharmacy|chemist|can of|a tube of|a bottle of|a packet of|a box of)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LEAD_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|then|and|but|also|now|first|next),?\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUOTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\\"\\u201c\\u201d][^\\\"\\u201c\\u201d]{2,}[\\\"\\u201c\\u201d]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DECISION_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should|shall|ought)\\s+i\\b|\\bis it worth\\b|\\bdo i need\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LONG_SENTENCE(Py::Int(52));
    s_A_WORD_THAT_ONLY_JOINS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nbecause cause cos since whether though although while during before after until unless\nnot dont doesnt didnt cant cannot wont isnt arent wasnt werent havent hasnt couldnt wouldnt\nshouldnt am are was were been being have has had having do does did doing can could would\nshould will shall may might must\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DOUBLED(py_str("bdgmnprtl"));
    s_EFFORT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("push pushes pushing carry carrying keep keeping continue continuing\non through harder ahead along"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE_AFTER_A_COLON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":\\s+(.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LONG_SENTENCE_SPLIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":\\s+|;\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GO_ON_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:push(?:\\s+(?:on|through|harder))?|carry\\s+on|keep\\s+(?:going|on|at\\s+it|it\\s+up)|continuing|continue|go\\s+on|going\\s+on|crack\\s+on)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LIMIT_IN_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bas\\s+far\\s+as\\b|\\bfor\\s+as\\s+long\\s+as\\b|\\bno\\s+more\\s+than\\b|\\bnot\\s+(?:much\\s+)?past\\b|\\b(?:un)?til\\s+(?:it|you|they|things|the\\s+(?:pain|soreness|swelling|bleeding))\\s*(?:eases|eased|is\\s+(?:better|easier|still|comfortable)|stops|settles|calms|feels|can|are\\s+able|hurts|gets\\s+worse)?\\b|\\bwhile\\s+(?:it|you|the|they)\\b|\\bshort\\s+of\\b|\\bup\\s+to\\b|\\bstops?\\s+(?:if|when|as\\s+soon|before|at)\\b|\\bease[sd]?\\s+off\\b|\\bback(?:s|ed)?\\s+off\\b|\\bfor\\s+(?:a|the\\s+first|one|two|three)\\s+(?:week|fortnight|day|while|few)\\b|\\bfor\\s+a\\s+while\\b|\\bin\\s+short\\s+bursts\\b|\\blittle\\s+and\\s+often\\b|\\bno\\s+more\\s+of\\s+than\\b|\\bwhen\\s+it\\s+(?:starts|hurts|gets\\s+hard|feels)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_WHAT_DECIDES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tell|say|let)\\s+me\\s+(?:what|how|which|whether|if|more)\\b|\\bwhat\\s+(?:it|they|you)\\s+(?:did|does|looks?\\s+like|smells?\\s+like|tastes?\\s+like)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ABOUT_THE_CONVERSATION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:hello again|hi again|you again|here again)\\b|\\bwhere (?:were we|we (?:were|left off|got to)|did we (?:get to|get))\\b|\\bwhat were we (?:saying|doing|on about)\\b|\\bcarry on where\\b|\\bpick (?:it |things |up )?where\\b|\\bas (?:i|we) (?:said|mentioned)\\b|\\blike i (?:said|mentioned|told you)\\b|\\b(?:what|that) i (?:said|told you)\\b|\\byou (?:said|mentioned|told me) (?:earlier|before|last time|a moment ago|just now)\\b|\\byou said\\s*:|\\bsince we last spoke\\b|\\bsame as before\\b|(?<!not )\\bthe last time\\b|\\blast time you\\b|\\bearlier you\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _stem(args: &[Py]) -> Py {
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
    let mut ending: Py = Py::None;
    let _ = py_str("The word without the ending that makes it a plural, a past tense or a participle.\n\n    The doubled last letter of \"running\", \"stopped\" and \"getting\" comes off too, so a question in\n    the plain form (\"can i still run\") meets the answer's \"-ing\" word (\"drop the running mileage\")\n    instead of looking like a thing the answer has never heard of.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = op_ge(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None);
    break;
    }
    }
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&word.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__DOUBLED(), &geti(&word.clone(), &op_neg(&Py::Int(1))))) } } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
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
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut other: Py = Py::None;
    let _ = py_str("True when `word` is in `said`, plurals and the -ing form counting as the same word.");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "rstrip", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_add(&word.clone(), &py_str("s")))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stems(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stem(args))(&__args) }))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut other: Py = __item.clone(); __out.push({ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(other.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(5), &Py::None), &slice_or_raise(&other.clone(), &Py::None, &Py::Int(5), &Py::None))) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stem(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _names_something_new(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut in_the_chat: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "in_the_chat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: in_the_chat"))) };
    let mut last: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line carries a word the answer has not got and that names a thing.\n\n    A word the answer shares is the same subject, a word that asks (`is that enough`, `how long`)\n    adds no subject at all, and a word that describes the thing the answer is about (`the smell`,\n    `the leak`) adds none either - but only when the chat has already named it, which is what\n    `in_the_chat` is. A verb in front of the thing is an action rather than a subject (`should i\n    cut the leaves off`); a verb at the end of the line is where a thing would be (`the school\n    run`) and counts like any other word. Anything else is something the answer does not have, and\n    it stands the reading down.\n    ");
    last = if py_truthy(&mine.clone()) { geti(&mine.clone(), &op_neg(&Py::Int(1))) } else { py_str("") };
    let __iter1 = py_iter_or_raise(&mine.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_word(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ASKS_RATHER_THAN_NAMES(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_EFFORT_WORDS(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THE_THINGS_IT_COULD_BE_ABOUT(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(in_the_chat.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_word(args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&word.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STEP_IN_IT(), "search", args))(&__args) } } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_words_in_the_chat(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every word said in the exchange the answer belongs to: the line before it, and the answer.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    let __aug2 = op_bitor(&words.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    words = __aug2;
    }
    return words.clone();
    Py::None
}
pub fn _the_answer_to_hand_back(args: &[Py]) -> Py {
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
    let _ = py_str("The bot's last knowledge answer, skipping notes and repeats of it.\n\n    \"what should i do about the smell\" comes two turns after the answer, because the line\n    between them (\"how do i clean it\") was itself answered with the composer's \"that was the\n    whole of my answer on it\": the answer is still the one on the person's screen, so it is the\n    one to hand back. A turn of the composer's own is not an answer to come back with - the note\n    it wrote adds no fact - and a hedge is not either.\n\n    A safety card counts as a knowledge answer: \"i was abused growing up\" is answered with the\n    card that says flinching is what an old injury does, and \"is that why i flinch\" one line later\n    got \"Searching for it will turn up better sources than I can offer\". The card's own sentence is\n    the answer to that, and handing it back cannot take the card away - it was the turn before.\n\n    The reach itself is one check, in `bot/memory/history.py`, so that every module that reads a\n    follow-up out of an earlier turn steps back over the same turns (`the_answer_on_screen`).\n    ");
    let mut history: Py = gen_memory::memory::history::module_obj();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&history.clone(), "the_answer_on_screen", args))(&__args) };
    Py::None
}
pub fn _the_answer_is_a_card(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the answer above is a safety card rather than a knowledge entry.");
    let mut history: Py = gen_memory::memory::history::module_obj();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&history.clone(), "the_answer_is_a_card", args))(&__args) };
    Py::None
}
pub fn _the_parts(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The sentence, or its parts at a colon or a semicolon when it is too long to say on its own.");
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_LONG_SENTENCE())) {
    return py_list(vec![sentence.clone()]);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LONG_SENTENCE_SPLIT(), "split", args))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return py_list(vec![sentence.clone()]);
    }
    return op_add(&py_list(vec![geti(&parts.clone(), &Py::Int(0))]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&one.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&one.clone(), &Py::Int(1), &Py::None, &Py::None))); } py_list(__out) })() });
    Py::None
}
pub fn _the_sentence_to_hand_back(args: &[Py]) -> Py {
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
    let mut mine: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut clause: Py = Py::None;
    let mut in_the_clause: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("`sentence`, or its clause after a colon when that is the part the line asks about.");
    tail = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAUSE_AFTER_A_COLON(), "search", args))(&__args) };
    if py_truthy(&tail.clone()) {
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tail.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    in_the_clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)); if !py_truthy(&__b) { __b } else { { let __b = in_the_clause.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_word(args))(&__args) }) {  __out.push(Py::Int(1)); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sum", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0))) } } } }) {
    sentence = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&clause.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    return sentence.clone();
    Py::None
}
pub fn the_answer_on_screen(args: &[Py]) -> Py {
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
    let mut already: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut asks_who_to_talk_to: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut first_said: Py = Py::None;
    let mut first_step: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut frames: Py = Py::None;
    let mut given: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut one: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut place: Py = Py::None;
    let mut punt: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says_it_again: Py = Py::None;
    let mut score: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut source: Py = Py::None;
    let mut step: Py = Py::None;
    let mut steps: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The sentence of the last answer that answers this line, framed, or \"\" for none.\n\n    Only a knowledge answer or a safety card counts (`_the_answer_to_hand_back`), because a card's\n    answer is the thing worth handing back: the composer's own line has no entry behind it to come\n    back with, and a reply that is already an acknowledgement hands back nothing.\n    ");
    let __try1 = exc::catch(|| {
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ON_SCREEN_ASK(), "match", args))(&__args) })) } }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_NAMED_IN_FRONT(), "match", args))(&__args) }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8))) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_hedged_recently(args))(&__args) }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_answer_to_hand_back(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::content_words(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_A_WORD_THAT_ONLY_JOINS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    asks_who_to_talk_to = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHO_TO_TALK_TO(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_answer_is_a_card(args))(&__args) } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asks_who_to_talk_to.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NAMES_NOTHING_OF_ITS_OWN(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words_in_the_chat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_something_new(args))(&__args) } } } } }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    let __unpk6 = py_unpack(&py_tuple(vec![py_str(""), Py::Int(0), py_str(""), py_str("")]), 4, None);
    best = __unpk6[0].clone();
    best_score = __unpk6[1].clone();
    first_step = __unpk6[2].clone();
    opening = __unpk6[3].clone();
    steps = py_list(vec![]);
    let __iter7 = py_iter_or_raise(&sentences.clone());
    for __item in __iter7 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "endswith", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_TO_HAND_BACK(), "match", args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUOTED(), "search", args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::compose::fallback::g_PUNT()) { let mut punt: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "lower", args))(&__args) }, &punt.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    step = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STEP_IN_IT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&step.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&steps.clone(), "append", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&first_step.clone()))) {
    first_step = sentence.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    opening = sentence.clone();
    }
    let __iter8 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_parts(args))(&__args) });
    for __item in __iter8 {
    piece = __item.clone();
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_LONG_SENTENCE())) {
    continue;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    score = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_word(args))(&__args) }) {  __out.push(Py::Int(1)); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sum", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = score.clone(); if !py_truthy(&__b) { __b } else { op_gt(&score.clone(), &best_score.clone()) } }) {
    let __unpk9 = py_unpack(&py_tuple(vec![piece.clone(), score.clone()]), 2, None);
    best = __unpk9[0].clone();
    best_score = __unpk9[1].clone();
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if !py_truthy(&__b) { __b } else { { let __b = first_step.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_WHAT_TO_DO(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_POINTS_BACK(), "search", args))(&__args) } } } } } } }) {
    best = first_step.clone();
    }
    says_it_again = Py::Bool(false);
    if py_truthy(&{ let __b = asks_who_to_talk_to.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&best.clone())) } }) {
    let __iter10 = py_iter_or_raise(&sentences.clone());
    for __item in __iter10 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_HELP_IN_IT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_TO_HAND_BACK(), "match", args))(&__args) })) } }) {
    best = sentence.clone();
    break;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if !py_truthy(&__b) { __b } else { { let __b = opening.clone(); if !py_truthy(&__b) { __b } else { asks_who_to_talk_to.clone() } } } }) {
    let __unpk11 = py_unpack(&py_tuple(vec![opening.clone(), Py::Bool(true)]), 2, None);
    best = __unpk11[0].clone();
    says_it_again = __unpk11[1].clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if !py_truthy(&__b) { __b } else { { let __b = mine.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NAMES_NOTHING_OF_ITS_OWN(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    let __unpk12 = py_unpack(&py_tuple(vec![opening.clone(), Py::Bool(true)]), 2, None);
    best = __unpk12[0].clone();
    says_it_again = __unpk12[1].clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GO_ON_ASK(), "search", args))(&__args) } } }) {
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&steps.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LIMIT_IN_IT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_WHAT_DECIDES(), "search", args))(&__args) } } }) {  __out.push(one.clone()); } } py_list(__out) })() }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DECISION_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STEP_IN_IT(), "search", args))(&__args) })) } }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BUY_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PURCHASE_IN_IT(), "search", args))(&__args) })) } }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    already = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    source = op_neg(&Py::Int(1));
    let __iter13 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter13 {
    let __unpk14 = py_unpack(&__item.clone(), 2, None);
    place = __unpk14[0].clone();
    turn = __unpk14[1].clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &answer.clone()))) {
    source = place.clone();
    }
    }
    let __iter15 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_add(&source.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter15 {
    turn = __item.clone();
    if py_truthy(&{ let __b = already.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &already.clone())) } }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&says_it_again.clone()))) {
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); __args.push(mine.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_sentence_to_hand_back(args))(&__args) };
    }
    first_said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&says_it_again.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &Py::None, &Py::Int(60), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first_said.clone(), "startswith", args))(&__args) } } }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    given = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LEAD_IN(), "sub", args))(&__args) };
    if py_truthy(&{ let __b = given.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&given.clone(), &best.clone())) } }) {
    best = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&given.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&given.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::kinds::a_stop_ask(args))(&__args) }) {
    frames = g_STANDS_BY_LINES();
    } else {
    frames = if py_truthy(&says_it_again.clone()) { slice_or_raise(&g_ON_SCREEN_LINES(), &Py::None, &Py::Int(1), &Py::None) } else { g_ON_SCREEN_LINES() };
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{said}")); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(frames.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) }, "replace", args))(&__args) };
    flat = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    recent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&t.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_flat_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&recent.clone(), &flat.clone()))) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    return py_tuple(vec![Py::Bool(true), reply.clone()]);
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
    Py::None
}
pub fn earlier_turn(args: &[Py]) -> Py {
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
    let _ = py_str("Whether anything has been said before this line.\n\n    The runtime appends the turn after the modules have answered, so a memory with no turns in it\n    is the opening line of a chat and any turn at all means there is a conversation.\n    ");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { turns.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn talks_about_the_conversation(args: &[Py]) -> Py {
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
    let mut unquoted: Py = Py::None;
    let _ = py_str("Whether a reply refers to something said before it in this conversation.\n\n    Only the reply's own words are read: a line that says \"as I said\" or \"you said earlier\" is\n    standing on an earlier turn whether or not one happened, and that is the whole test. What the\n    person's own life refers to is not this: \"welcome back from Leeds\" answers someone who has\n    just come back from Leeds on the first line, so the pools whose lines are about a real return\n    (`trip`, `back_from`) are left alone, and the pools that are only about returning to a chat\n    are answered as a greeting instead (`FIRST_LINE_POOL` in `bot/modules/chat.py`).\n    ");
    unquoted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\\"“][^\\\"”]*[\\\"”]")); __args.push(py_str(" ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(unquoted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ABOUT_THE_CONVERSATION(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.onscreen".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_RATHER_THAN_NAMES" => Some(g_ASKS_RATHER_THAN_NAMES()),
            "ASKS_WHAT_DECIDES" => Some(g_ASKS_WHAT_DECIDES()),
            "ASKS_WHAT_TO_DO" => Some(g_ASKS_WHAT_TO_DO()),
            "A_BUY_ASK" => Some(g_A_BUY_ASK()),
            "A_DECISION_ASK" => Some(g_A_DECISION_ASK()),
            "A_GO_ON_ASK" => Some(g_A_GO_ON_ASK()),
            "A_HELP_IN_IT" => Some(g_A_HELP_IN_IT()),
            "A_LEAD_IN" => Some(g_A_LEAD_IN()),
            "A_LIMIT_IN_IT" => Some(g_A_LIMIT_IN_IT()),
            "A_LONG_SENTENCE_SPLIT" => Some(g_A_LONG_SENTENCE_SPLIT()),
            "A_PURCHASE_IN_IT" => Some(g_A_PURCHASE_IN_IT()),
            "A_STEP_IN_IT" => Some(g_A_STEP_IN_IT()),
            "A_THING_NAMED_IN_FRONT" => Some(g_A_THING_NAMED_IN_FRONT()),
            "A_WORD_THAT_ONLY_JOINS" => Some(g_A_WORD_THAT_ONLY_JOINS()),
            "CLAUSE_AFTER_A_COLON" => Some(g_CLAUSE_AFTER_A_COLON()),
            "EFFORT_WORDS" => Some(g_EFFORT_WORDS()),
            "LONG_SENTENCE" => Some(g_LONG_SENTENCE()),
            "NAMES_NOTHING_OF_ITS_OWN" => Some(g_NAMES_NOTHING_OF_ITS_OWN()),
            "NOT_TO_HAND_BACK" => Some(g_NOT_TO_HAND_BACK()),
            "ON_SCREEN_ASK" => Some(g_ON_SCREEN_ASK()),
            "ON_SCREEN_LINES" => Some(g_ON_SCREEN_LINES()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "QUOTED" => Some(g_QUOTED()),
            "STANDS_BY_LINES" => Some(g_STANDS_BY_LINES()),
            "THE_THINGS_IT_COULD_BE_ABOUT" => Some(g_THE_THINGS_IT_COULD_BE_ABOUT()),
            "WHO_TO_TALK_TO" => Some(g_WHO_TO_TALK_TO()),
            "_ABOUT_THE_CONVERSATION" => Some(g__ABOUT_THE_CONVERSATION()),
            "_DOUBLED" => Some(g__DOUBLED()),
            "_names_something_new" => Some(py_func("_names_something_new", |args: &[Py]| _names_something_new(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_stems" => Some(py_func("_stems", |args: &[Py]| _stems(args))),
            "_the_answer_is_a_card" => Some(py_func("_the_answer_is_a_card", |args: &[Py]| _the_answer_is_a_card(args))),
            "_the_answer_to_hand_back" => Some(py_func("_the_answer_to_hand_back", |args: &[Py]| _the_answer_to_hand_back(args))),
            "_the_parts" => Some(py_func("_the_parts", |args: &[Py]| _the_parts(args))),
            "_the_same_word" => Some(py_func("_the_same_word", |args: &[Py]| _the_same_word(args))),
            "_the_sentence_to_hand_back" => Some(py_func("_the_sentence_to_hand_back", |args: &[Py]| _the_sentence_to_hand_back(args))),
            "_the_words_in_the_chat" => Some(py_func("_the_words_in_the_chat", |args: &[Py]| _the_words_in_the_chat(args))),
            "earlier_turn" => Some(py_func("earlier_turn", |args: &[Py]| earlier_turn(args))),
            "talks_about_the_conversation" => Some(py_func("talks_about_the_conversation", |args: &[Py]| talks_about_the_conversation(args))),
            "the_answer_on_screen" => Some(py_func("the_answer_on_screen", |args: &[Py]| the_answer_on_screen(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS_RATHER_THAN_NAMES" => s_ASKS_RATHER_THAN_NAMES(v),
            "ASKS_WHAT_DECIDES" => s_ASKS_WHAT_DECIDES(v),
            "ASKS_WHAT_TO_DO" => s_ASKS_WHAT_TO_DO(v),
            "A_BUY_ASK" => s_A_BUY_ASK(v),
            "A_DECISION_ASK" => s_A_DECISION_ASK(v),
            "A_GO_ON_ASK" => s_A_GO_ON_ASK(v),
            "A_HELP_IN_IT" => s_A_HELP_IN_IT(v),
            "A_LEAD_IN" => s_A_LEAD_IN(v),
            "A_LIMIT_IN_IT" => s_A_LIMIT_IN_IT(v),
            "A_LONG_SENTENCE_SPLIT" => s_A_LONG_SENTENCE_SPLIT(v),
            "A_PURCHASE_IN_IT" => s_A_PURCHASE_IN_IT(v),
            "A_STEP_IN_IT" => s_A_STEP_IN_IT(v),
            "A_THING_NAMED_IN_FRONT" => s_A_THING_NAMED_IN_FRONT(v),
            "A_WORD_THAT_ONLY_JOINS" => s_A_WORD_THAT_ONLY_JOINS(v),
            "CLAUSE_AFTER_A_COLON" => s_CLAUSE_AFTER_A_COLON(v),
            "EFFORT_WORDS" => s_EFFORT_WORDS(v),
            "LONG_SENTENCE" => s_LONG_SENTENCE(v),
            "NAMES_NOTHING_OF_ITS_OWN" => s_NAMES_NOTHING_OF_ITS_OWN(v),
            "NOT_TO_HAND_BACK" => s_NOT_TO_HAND_BACK(v),
            "ON_SCREEN_ASK" => s_ON_SCREEN_ASK(v),
            "ON_SCREEN_LINES" => s_ON_SCREEN_LINES(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "QUOTED" => s_QUOTED(v),
            "STANDS_BY_LINES" => s_STANDS_BY_LINES(v),
            "THE_THINGS_IT_COULD_BE_ABOUT" => s_THE_THINGS_IT_COULD_BE_ABOUT(v),
            "WHO_TO_TALK_TO" => s_WHO_TO_TALK_TO(v),
            "_ABOUT_THE_CONVERSATION" => s__ABOUT_THE_CONVERSATION(v),
            "_DOUBLED" => s__DOUBLED(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.onscreen", module_obj);
    pyrt::xrefs::reg_fn("compose.onscreen", "_stem", py_func("_stem", |args: &[Py]| _stem(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_the_same_word", py_func("_the_same_word", |args: &[Py]| _the_same_word(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_stems", py_func("_stems", |args: &[Py]| _stems(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_names_something_new", py_func("_names_something_new", |args: &[Py]| _names_something_new(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_the_words_in_the_chat", py_func("_the_words_in_the_chat", |args: &[Py]| _the_words_in_the_chat(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_the_answer_to_hand_back", py_func("_the_answer_to_hand_back", |args: &[Py]| _the_answer_to_hand_back(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_the_answer_is_a_card", py_func("_the_answer_is_a_card", |args: &[Py]| _the_answer_is_a_card(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_the_parts", py_func("_the_parts", |args: &[Py]| _the_parts(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "_the_sentence_to_hand_back", py_func("_the_sentence_to_hand_back", |args: &[Py]| _the_sentence_to_hand_back(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "the_answer_on_screen", py_func("the_answer_on_screen", |args: &[Py]| the_answer_on_screen(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "earlier_turn", py_func("earlier_turn", |args: &[Py]| earlier_turn(args)));
    pyrt::xrefs::reg_fn("compose.onscreen", "talks_about_the_conversation", py_func("talks_about_the_conversation", |args: &[Py]| talks_about_the_conversation(args)));
    pyrt::xrefs::reg_access("compose.onscreen", "ASKS_RATHER_THAN_NAMES", g_ASKS_RATHER_THAN_NAMES, s_ASKS_RATHER_THAN_NAMES);
    pyrt::xrefs::reg_access("compose.onscreen", "ASKS_WHAT_DECIDES", g_ASKS_WHAT_DECIDES, s_ASKS_WHAT_DECIDES);
    pyrt::xrefs::reg_access("compose.onscreen", "ASKS_WHAT_TO_DO", g_ASKS_WHAT_TO_DO, s_ASKS_WHAT_TO_DO);
    pyrt::xrefs::reg_access("compose.onscreen", "A_BUY_ASK", g_A_BUY_ASK, s_A_BUY_ASK);
    pyrt::xrefs::reg_access("compose.onscreen", "A_DECISION_ASK", g_A_DECISION_ASK, s_A_DECISION_ASK);
    pyrt::xrefs::reg_access("compose.onscreen", "A_GO_ON_ASK", g_A_GO_ON_ASK, s_A_GO_ON_ASK);
    pyrt::xrefs::reg_access("compose.onscreen", "A_HELP_IN_IT", g_A_HELP_IN_IT, s_A_HELP_IN_IT);
    pyrt::xrefs::reg_access("compose.onscreen", "A_LEAD_IN", g_A_LEAD_IN, s_A_LEAD_IN);
    pyrt::xrefs::reg_access("compose.onscreen", "A_LIMIT_IN_IT", g_A_LIMIT_IN_IT, s_A_LIMIT_IN_IT);
    pyrt::xrefs::reg_access("compose.onscreen", "A_LONG_SENTENCE_SPLIT", g_A_LONG_SENTENCE_SPLIT, s_A_LONG_SENTENCE_SPLIT);
    pyrt::xrefs::reg_access("compose.onscreen", "A_PURCHASE_IN_IT", g_A_PURCHASE_IN_IT, s_A_PURCHASE_IN_IT);
    pyrt::xrefs::reg_access("compose.onscreen", "A_STEP_IN_IT", g_A_STEP_IN_IT, s_A_STEP_IN_IT);
    pyrt::xrefs::reg_access("compose.onscreen", "A_THING_NAMED_IN_FRONT", g_A_THING_NAMED_IN_FRONT, s_A_THING_NAMED_IN_FRONT);
    pyrt::xrefs::reg_access("compose.onscreen", "A_WORD_THAT_ONLY_JOINS", g_A_WORD_THAT_ONLY_JOINS, s_A_WORD_THAT_ONLY_JOINS);
    pyrt::xrefs::reg_access("compose.onscreen", "CLAUSE_AFTER_A_COLON", g_CLAUSE_AFTER_A_COLON, s_CLAUSE_AFTER_A_COLON);
    pyrt::xrefs::reg_access("compose.onscreen", "EFFORT_WORDS", g_EFFORT_WORDS, s_EFFORT_WORDS);
    pyrt::xrefs::reg_access("compose.onscreen", "LONG_SENTENCE", g_LONG_SENTENCE, s_LONG_SENTENCE);
    pyrt::xrefs::reg_access("compose.onscreen", "NAMES_NOTHING_OF_ITS_OWN", g_NAMES_NOTHING_OF_ITS_OWN, s_NAMES_NOTHING_OF_ITS_OWN);
    pyrt::xrefs::reg_access("compose.onscreen", "NOT_TO_HAND_BACK", g_NOT_TO_HAND_BACK, s_NOT_TO_HAND_BACK);
    pyrt::xrefs::reg_access("compose.onscreen", "ON_SCREEN_ASK", g_ON_SCREEN_ASK, s_ON_SCREEN_ASK);
    pyrt::xrefs::reg_access("compose.onscreen", "ON_SCREEN_LINES", g_ON_SCREEN_LINES, s_ON_SCREEN_LINES);
    pyrt::xrefs::reg_access("compose.onscreen", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("compose.onscreen", "QUOTED", g_QUOTED, s_QUOTED);
    pyrt::xrefs::reg_access("compose.onscreen", "STANDS_BY_LINES", g_STANDS_BY_LINES, s_STANDS_BY_LINES);
    pyrt::xrefs::reg_access("compose.onscreen", "THE_THINGS_IT_COULD_BE_ABOUT", g_THE_THINGS_IT_COULD_BE_ABOUT, s_THE_THINGS_IT_COULD_BE_ABOUT);
    pyrt::xrefs::reg_access("compose.onscreen", "WHO_TO_TALK_TO", g_WHO_TO_TALK_TO, s_WHO_TO_TALK_TO);
    pyrt::xrefs::reg_access("compose.onscreen", "_ABOUT_THE_CONVERSATION", g__ABOUT_THE_CONVERSATION, s__ABOUT_THE_CONVERSATION);
    pyrt::xrefs::reg_access("compose.onscreen", "_DOUBLED", g__DOUBLED, s__DOUBLED);
}
