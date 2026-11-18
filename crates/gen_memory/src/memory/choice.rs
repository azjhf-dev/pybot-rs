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


thread_local! { static G_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BY_A_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEFINITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSISTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AT_THE_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_CONTINUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_THEN_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRONG_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERBISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEAK_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_HAND_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INSIST_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INSIST_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INSIST_WRAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PICK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PICK_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAIL_STRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_CHOICE() -> Py { ensure_init(); G_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHOICE(v: Py) { ensure_init(); G_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_CHANGE() -> Py { ensure_init(); G_BARE_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_CHANGE(v: Py) { ensure_init(); G_BARE_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_BY_A_RULE() -> Py { ensure_init(); G_BY_A_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BY_A_RULE(v: Py) { ensure_init(); G_BY_A_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_IT() -> Py { ensure_init(); G_CHANGED_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_IT(v: Py) { ensure_init(); G_CHANGED_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION() -> Py { ensure_init(); G_CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION(v: Py) { ensure_init(); G_CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_OPENER() -> Py { ensure_init(); G_CORRECTION_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_OPENER(v: Py) { ensure_init(); G_CORRECTION_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_CUE() -> Py { ensure_init(); G_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUE(v: Py) { ensure_init(); G_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDING_ASK() -> Py { ensure_init(); G_DECIDING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDING_ASK(v: Py) { ensure_init(); G_DECIDING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DEFINITE() -> Py { ensure_init(); G_DEFINITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFINITE(v: Py) { ensure_init(); G_DEFINITE.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_INSISTED() -> Py { ensure_init(); G_INSISTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSISTED(v: Py) { ensure_init(); G_INSISTED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AT_THE_START() -> Py { ensure_init(); G_NOT_AT_THE_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AT_THE_START(v: Py) { ensure_init(); G_NOT_AT_THE_START.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); G_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); G_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_CONTINUES() -> Py { ensure_init(); G_NO_CONTINUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_CONTINUES(v: Py) { ensure_init(); G_NO_CONTINUES.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_THEN_THING() -> Py { ensure_init(); G_NO_THEN_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_THEN_THING(v: Py) { ensure_init(); G_NO_THEN_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PICKS() -> Py { ensure_init(); G_PICKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICKS(v: Py) { ensure_init(); G_PICKS.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_FRAME() -> Py { ensure_init(); G_SECOND_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_FRAME(v: Py) { ensure_init(); G_SECOND_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLED() -> Py { ensure_init(); G_SETTLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLED(v: Py) { ensure_init(); G_SETTLED.with(|c| *c.borrow_mut() = v); }
pub fn g_STRONG_CHANGE() -> Py { ensure_init(); G_STRONG_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRONG_CHANGE(v: Py) { ensure_init(); G_STRONG_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAIL() -> Py { ensure_init(); G_TRAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAIL(v: Py) { ensure_init(); G_TRAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_VERBISH() -> Py { ensure_init(); G_VERBISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERBISH(v: Py) { ensure_init(); G_VERBISH.with(|c| *c.borrow_mut() = v); }
pub fn g_WEAK_CHANGE() -> Py { ensure_init(); G_WEAK_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEAK_CHANGE(v: Py) { ensure_init(); G_WEAK_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED() -> Py { ensure_init(); G__ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED(v: Py) { ensure_init(); G__ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEASURE() -> Py { ensure_init(); G__A_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEASURE(v: Py) { ensure_init(); G__A_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_HAND_BACK() -> Py { ensure_init(); G__BARE_HAND_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_HAND_BACK(v: Py) { ensure_init(); G__BARE_HAND_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__CLAUSE() -> Py { ensure_init(); G__CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLAUSE(v: Py) { ensure_init(); G__CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__INSIST_LEAD() -> Py { ensure_init(); G__INSIST_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INSIST_LEAD(v: Py) { ensure_init(); G__INSIST_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__INSIST_SAID() -> Py { ensure_init(); G__INSIST_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INSIST_SAID(v: Py) { ensure_init(); G__INSIST_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__INSIST_WRAP() -> Py { ensure_init(); G__INSIST_WRAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INSIST_WRAP(v: Py) { ensure_init(); G__INSIST_WRAP.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD() -> Py { ensure_init(); G__LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); G__LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THING() -> Py { ensure_init(); G__NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING(v: Py) { ensure_init(); G__NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__PICK_ASK() -> Py { ensure_init(); G__PICK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PICK_ASK(v: Py) { ensure_init(); G__PICK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PICK_SAID() -> Py { ensure_init(); G__PICK_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PICK_SAID(v: Py) { ensure_init(); G__PICK_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__STEP() -> Py { ensure_init(); G__STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEP(v: Py) { ensure_init(); G__STEP.with(|c| *c.borrow_mut() = v); }
pub fn g__TAIL_STRIP() -> Py { ensure_init(); G__TAIL_STRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAIL_STRIP(v: Py) { ensure_init(); G__TAIL_STRIP.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("When the person hands the decision back: \"you pick\", \"your call\", \"either is fine\".\n\nThe bot offered two or three things a turn ago (\"pasta, eggs on toast, or cold rice\"), and the\nperson answers \"you pick\". That is not a new question about the world: it asks the bot to\nchoose between options it has just proposed. Reading it back as a fresh question gives a search\nhedge, and reading it as nothing gives \"Noted!\".\n\nSo this reads the bot's own last reply, takes the options out of it and says one. It keeps the\nchoice general: a bulleted list, a \"X, Y or Z\" list, italics (*Groundhog Day*) and a \"would you\nrather X or Y?\" question are all read the same way, so every lane's answers get followed up on\nwithout each of them teaching the bot about picks.\n\nIt is deliberately narrow. The whole line has to be the ask (a real question like \"which one\nwould you pick for a wedding present?\" carries its own subject and belongs to the answers\nlane), and a reply with no readable list of two or more options returns None, so the rest of the\nbot answers as it did before.\n");
    s__LEAD(py_str("^(?:(?:ok|okay|fine|right|well|so|then|hmm|ha)[, ]+)*"));
    s__PICK_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&g__LEAD(), &py_str("(?:(?:you|u|yous)[ ]?(?:pick|choose|decide|select|choose one|pick one|decide for me)|(?:you|u) (?:can )?(?:pick|choose|decide)(?: (?:one|it|for me|the (?:one|rest|\\w+)))?|your (?:call|choice|pick|decision)|(?:i'?ll |i will |i'?d )?let you (?:pick|choose|decide)(?: (?:one|it|for me|this time))?|(?:can|could|will|would) you (?:pick|choose|decide)(?: (?:one|it|for me))?|up to you|over to you|you tell me|you know best|surprise me|(?:i )?(?:don'?t|do not) mind[, ]+you (?:pick|choose|decide)|(?:either|any|whatever)(?: one| of (?:them|those|these))?(?: is| are)? (?:fine|good|ok|okay)(?:[, ]+(?:you|u) (?:pick|choose|decide))?|(?:you|u) (?:pick|choose|decide)(?:[, ]+(?:i )?(?:don'?t|do not) mind)?)(?: (?:then|please|thanks|for me|between (?:them|those|these|the two)))?[.?!]*$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|i'll|i'd|you|you'll|we|they|tell me|let me|if|but|because|which|whichever|whatever|search|google|not sure|don'?t know)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TAIL_STRIP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s.;:,]+$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__STEP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:with|then|until|after|before|for \\d|at \\d|under|onto|over|into|using|while|once)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:so|are|is|was|were|works|tastes|means|are)\\b|^-")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PICKS(py_tuple(vec![py_str("My pick: {option}."), py_str("I'd say {option}."), py_str("If it were me: {option}."), py_str("{option} — that's the one I'd choose.")]));
    s__ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:\\W*(?:what|which|who|when|where|why|how|any|anyone|anything|can|could|should|would|do|does|did|is|are|shall|give me|tell me|suggest|recommend)\\b|.*\\?\\s*$)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BARE_HAND_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:ok|okay|fine|right|well|so|then|and|hmm|ha)[, ]+)*(?:(?:i\\s)?(?:don'?t|do not|dont) mind(?: (?:really|either|much|at all|tbh))?|(?:i'?m|im) (?:easy|not bothered|not fussed)(?: (?:either|really|tbh))?|(?:either|any|anything)(?: one| of (?:them|those|these))?(?: ?(?:is|are)? ?(?:fine|good|ok|okay|works))?|(?:any|either) of (?:them|those|these)|whatever (?:you|u) (?:think|reckon|say|like))[.?!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PICK_SAID(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:my pick:|if it were me:|i'?d say|i'?d go with|i would go with) *(.+?)[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(.+?)\\s+\\u2014\\s+that's the one i'?d choose[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s__INSIST_LEAD(py_str("then it is|my pick is still|same pick:|i'?ll decide then:|go with"));
    s__INSIST_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("^(?:%s) *(.+?)[.!](?:\\s|$)"), &g__INSIST_LEAD())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__INSIST_WRAP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("^(?:%s) *(.+?)[.!]*$"), &g__INSIST_LEAD())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_INSISTED(py_tuple(vec![py_str("Then it is {option}. You can blame me if it is wrong."), py_str("My pick is still {option}."), py_str("Same pick: {option}."), py_str("I'll decide then: {option}."), py_str("Go with {option}. Someone has to choose, so I have."), py_str("Then it is {option}. There is nothing else to weigh up.")]));
    s_BY_A_RULE(py_tuple(vec![py_str("Then take the cheapest one that fits the time you have, and sort the details as you go."), py_str("Go with the one you would be annoyed to lose. The rest is noise."), py_str("Pick the first one you liked the sound of. That instinct is usually right."), py_str("Then choose the one that leaves you the most time. The rest sorts itself out.")]));
    s__A_MEASURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+(?:\\.\\d+)?\\s*(?:cal|calories|kcal|g|kg|mg|ml|l|gram|grams|kilo|kilos|litre|litres|min|mins|minutes|hour|hours|degrees?|percent|%)|per\\s+\\d)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DECIDING_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:ok|okay|fine|right|well|so|then|and|but|hmm|ah)[, ]+)*(?:(?:what|which)(?:\\s+(?:one|thing|of them))?\\s+(?:did|do|have|had)\\s+(?:i|we)\\s+(?:decide|decided|settle|settled|pick|picked|choose|chose|go for|go with|end up with|end up getting|come down to|land on|get|got|buy|bought)|(?:so\\s+)?what\\s+(?:am|are)\\s+(?:i|we)\\s+(?:getting|having|going for|picking|ending up with|settling on))\\b[^?.!]{0,24}[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STRONG_CHANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:actually|instead|on second thoughts?|changed my mind|then again|make it|i(?:'| a)?ve decided|i decided|we decided|we(?:'| a)?ve decided|decided on|settled on|settling on|i(?:'| a)?ve settled|we(?:'| a)?ve settled)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WEAK_CHANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:go with|going with|go for|going for|went with|went for|end up with|ended up with|land on|landed on|opt for|opted for|pick|picked|picks|choose|chose|chosen)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHANGED_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&gattr(&g_STRONG_CHANGE(), "pattern"), &py_str("|")), &gattr(&g_WEAK_CHANGE(), "pattern"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("no nope ok okay so then well and but lets let fine right actually instead\n    hmm ah yes yeah"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SECOND_FRAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:make (?:it|that)|go (?:with|for)|going (?:with|for)|decided on|settled on|settling on|(?:i|we)(?:'ve|'a)?ve?\\s+(?:decided|settled)(?:\\s+(?:on|to))?|lets|let us|to (?:get|go|buy|have)|just|then|so|ok|okay|instead|actually)\\b[\\s,]*)+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_AT_THE_START({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("up out off on down back in into onto over around through about\n    at by from with without for to of after before since until away along across"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DEFINITE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("the this that these those my our your his her their"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TRAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("instead actually really probably maybe i think though anyway honestly then\n    now please thanks for with to on in at about of and but so also too both"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:its|it's|it is|theyre|they're|they are|theres|there's|thats|that's|hes|he's|shes|she's|im|i'm|i am|we're|we are)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_CHOICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:thinking (?:of|about)|consider(?:ing)?|deciding|decided between|chose between|choosing between|torn between|looking at|can'?t decide|undecided|options?|should i (?:get|go|buy|have|pick|choose)|which (?:one|would)|instead of|whether)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_CHANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("actually"), py_str("rather")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DETERMINERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the my our your his her their this that"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_THEN_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:no|nope)\\b[\\s,!]+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTION_OPENER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:actually|oh|sorry|ok|okay|well|right|wait)\\s+[,.]?\\s*)*(?:no|nope)\\b[\\s,.!-]+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_CONTINUES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("longer more way thanks thank such bother idea need chance matter doubt"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_VERBISH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nlike likes liked love loved hate hated want wanted need needed think thought know knew\nsaid say says tell told do did does done be been being is are was were will would can\ncould should might have has had get got go went feel felt seems seem sound sounds work works\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\na an the my our your his her their this that these those it its it's them they he she hes shes\none ones you your yours i im ive we us there here\nto get getting have having buy buying go going for with on about at in\nof and or but so then now instead actually really probably maybe i think we\nsomething anything nothing else more better different\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SETTLED(py_tuple(vec![py_str("{thing} — that's the one you settled on."), py_str("You settled on {thing}."), py_str("The one you landed on was {thing}."), py_str("{thing}, as far as this chat goes.")]));
}
pub fn _clean_option(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut words: Py = Py::None;
    let _ = py_str("One option, trimmed, or \"\" when the piece is a clause rather than a thing to choose.");
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" \t*_-—")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) };
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:or|and|either|then|else)\\s+")); __args.push(py_str("")); __args.push(piece.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TAIL_STRIP(), "sub", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&piece.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(11)); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(70)) } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CLAUSE(), "search", args))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("?!:"), &slice_or_raise(&piece.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("...")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "endswith", args))(&__args) } } }) {
    return py_str("");
    }
    return piece.clone();
    Py::None
}
pub fn _from_italics(args: &[Py]) -> Py {
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
    let mut options: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut title: Py = Py::None;
    let _ = py_str("Options marked out as titles: \"*Groundhog Day* for comedy, or *Arrival* ...\".");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\*([^*\\n]{2,40})\\*(?: ([^,;.\\n]{1,30}))?")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    options = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&found.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    title = __unpk2[0].clone();
    tail = __unpk2[1].clone();
    title = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(title.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean_option(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&title.clone()))) {
    continue;
    }
    tail = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = tail.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CLAUSE(), "search", args))(&__args) }) { py_str("") } else { { let __b = tail.clone(); if py_truthy(&__b) { __b } else { py_str("") } } };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&title.clone(), &py_str(" ")), &tail.clone()), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean_option(args))(&__args) }; if py_truthy(&__b) { __b } else { title.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&options.clone(), "append", args))(&__args) };
    }
    return options.clone();
    Py::None
}
pub fn _segment(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut sep: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The part of a line that could hold the list: after a colon or a dash, else all of it.");
    body = text.clone();
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str(":"), py_str(" — "), py_str(" - ")]));
    for __item in __iter1 {
    sep = __item.clone();
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sep.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "rpartition", args))(&__args) }, 3, None);
    head = __unpk2[0].clone();
    found = __unpk2[1].clone();
    tail = __unpk2[2].clone();
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tail.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {
    return tail.clone();
    }
    }
    return body.clone();
    Py::None
}
pub fn _from_list(args: &[Py]) -> Py {
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
    let mut comma: Py = Py::None;
    let mut o: Py = Py::None;
    let mut options: Py = Py::None;
    let mut p: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut things: Py = Py::None;
    let _ = py_str("A plain list: \"pasta with garlic and oil; eggs on toast; or cold rice fried with veg\".");
    body = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _segment(args))(&__args) };
    body = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\.\\s+[A-Z]")); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
pub fn __nested_things_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut body: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut comma: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut o: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut options: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut p: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut pieces: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut things: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut pieces: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "pieces").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut steps: Py = __pos.get(9).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "steps").map(|(_, v)| v.clone())).unwrap_or_else(|| Py::Bool(false));
    let mut option: Py = Py::None;
    let mut out: Py = Py::None;
    let mut piece: Py = Py::None;
    let _ = py_str("The pieces that are a thing to choose, not a step or the tail of a sentence.\n\n        A piece split off by a semicolon or an \"or\" is a list item whatever words it uses\n        (\"cold rice fried with frozen veg\"); a piece split off only by a comma can be a step in\n        a method (\"a quarter turn at a time\"), so only those are checked for step words.\n        ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pieces.clone());
    for __item in __iter1 {
    piece = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_A_THING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = steps.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STEP(), "search", args))(&__args) } } } } }) {
    continue;
    }
    option = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean_option(args))(&__args) };
    if py_truthy(&option.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    return out.clone();
    Py::None
}
    things = {{ let body = body.clone(); let comma = comma.clone(); let o = o.clone(); let options = options.clone(); let p = p.clone(); let pieces = pieces.clone(); let text = text.clone(); let things = things.clone(); py_func("things", move |args: &[Py]| { let mut full = vec![body.clone(), comma.clone(), o.clone(), options.clone(), p.clone(), pieces.clone(), text.clone(), things.clone()]; full.extend_from_slice(args); __nested_things_0(&full) }) }};
    if py_truthy(&{ let __b = Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "lower", args))(&__args) }, &py_str(" or "))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&body.clone(), &py_str(";"))) } }) {
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(";")); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut p: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "strip", args))(&__args) }) {  __out.push(p.clone()); } } py_list(__out) })() };
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pieces.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&things, args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",\\s+|\\s+or\\s+|\\s+and\\s+")); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut p: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "strip", args))(&__args) }) {  __out.push(p.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&things, args))(&__args) };
    }
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return options.clone();
    }
    }
    comma = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",\\s+")); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut p: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "strip", args))(&__args) }) {  __out.push(p.clone()); } } py_list(__out) })() }); __kws.extend(vec![("steps".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&things, args))(&__args) }) { let mut o: Py = __item.clone(); if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&o.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8))) {  __out.push(o.clone()); } } py_list(__out) })() };
    return if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(comma.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) { comma.clone() } else { py_list(vec![]) };
    Py::None
}
pub fn _from_labels(args: &[Py]) -> Py {
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
    let mut label: Py = Py::None;
    let mut labels: Py = Py::None;
    let mut option: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut unique: Py = Py::None;
    let _ = py_str("A reply that answers with named dishes or options and explains each: \"Scrambled eggs: ...\n    Aglio e olio: ...\" The names are the things to choose between, not the explanations.");
    labels = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:^|[.;]\\s+)([A-Za-z][^:.;\\n]{2,30}):")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    label = __item.clone();
    option = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(label.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean_option(args))(&__args) };
    if py_truthy(&option.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&labels.clone(), "append", args))(&__args) };
    }
    }
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_list(vec![])]), 2, None);
    seen = __unpk2[0].clone();
    unique = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&labels.clone());
    for __item in __iter3 {
    label = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&seen.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&label.clone(), "lower", args))(&__args) }))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&label.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(label.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unique.clone(), "append", args))(&__args) };
    }
    }
    return unique.clone();
    Py::None
}
pub fn _from_question(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut body: Py = Py::None;
    let mut p: Py = Py::None;
    let _ = py_str("The bot's own either/or question: \"Would you rather go Saturday or Sunday?\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "rstrip", args))(&__args) }, "endswith", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }, &py_str(" or "))) } }) {
    return py_list(vec![]);
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:so |then )?(?:would you rather|would you prefer|shall we|do you want|do you fancy|which would you rather|are you (?:thinking|leaning)|how about|which one)\\s+")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) }); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "subn", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 2, None);
    body = __unpk1[0].clone();
    again = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&again.clone()))) {
    return py_list(vec![]);
    }
    body = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&body.clone(), "rstrip", args))(&__args) };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+or\\s+")); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut p: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(p.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean_option(args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(p.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean_option(args))(&__args) }); } } py_list(__out) })() };
    Py::None
}
pub fn options_in(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut option: Py = Py::None;
    let mut options: Py = Py::None;
    let mut reader: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut text: Py = Py::None;
    let mut unique: Py = Py::None;
    let _ = py_str("The options the bot offered in one reply, or [] when it did not offer a choice.");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return py_list(vec![]);
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_func("_from_italics", |args: &[Py]| _from_italics(args)), py_func("_from_labels", |args: &[Py]| _from_labels(args)), py_func("_from_question", |args: &[Py]| _from_question(args)), py_func("_from_list", |args: &[Py]| _from_list(args))]));
    for __item in __iter1 {
    reader = __item.clone();
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&reader, args))(&__args) };
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_list(vec![])]), 2, None);
    seen = __unpk2[0].clone();
    unique = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&options.clone());
    for __item in __iter3 {
    option = __item.clone();
    key = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&option.clone(), "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&seen.clone(), &key.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unique.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(unique.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return unique.clone();
    }
    }
    return py_list(vec![]);
    Py::None
}
pub fn asks_for_a_pick(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the whole line hands the choice back to the bot.");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PICK_ASK(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _from_user_line(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let mut first: Py = Py::None;
    let mut line: Py = Py::None;
    let mut o: Py = Py::None;
    let mut option: Py = Py::None;
    let mut options: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut r: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The choice the person put on the table themselves: \"should i get a cat or a dog\".\n\n    When the bot's own answer offered nothing, the options the person named are still the\n    choices in front of them, and \"you pick\" still means one of those.\n    ");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::recent(args))(&__args) };
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = if py_truthy(&turns.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) } } else { py_str("") }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str(" or ")))) {
    return py_list(vec![]);
    }
    body = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:so|ok|okay|but)[, ]+)?(?:should|could|do|would|shall|can|will)\\s+(?:i|we)\\s+|^\\W*(?:is it better to|which is better,?|what about|is it worth)\\s+")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "rstrip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    options = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+or\\s+")); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    piece = __item.clone();
    option = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean_option(args))(&__args) };
    if py_truthy(&{ let __b = option.clone(); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&option.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&options.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return py_list(vec![]);
    }
    first = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); __out.push(geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&o.clone(), "split", args))(&__args) }, &Py::Int(0))); } py_set(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); __out.push(op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&o.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&o.clone(), "split", args))(&__args) }, &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut r: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&r.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    options = rest.clone();
    }
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&options.clone(), &Py::Int(0)), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:a|an|the)\\b")); __args.push(geti(&options.clone(), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); __out.push(if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&o.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&o.clone(), "split", args))(&__args) }, &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } } else { o.clone() }); } py_list(__out) })() };
    }
    return options.clone();
    Py::None
}
pub fn _asked_for_help(args: &[Py]) -> Py {
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
    let mut n: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(4) };
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the person has asked for suggestions in the last few turns.\n\n    The ask is usually the line just before the bot's offer (\"what should i cook tonight\"), but\n    it can be one turn further back (\"what should i cook\" / \"i only have eggs and pasta in\" /\n    \"you pick\"). A choice handed back long after a statement (\"i have a nut allergy\" /\n    \"either is fine\") is a comment, not a request to pick.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(n.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::recent(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKED(), "match", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _peeled(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut option: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "option") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: option"))) };
    let mut _u: Py = Py::None;
    let mut match_: Py = Py::None;
    option = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" *_—,.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&option.clone(), "strip", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    _u = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__INSIST_WRAP(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    break;
    }
    option = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" *_—,.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    }
    return option.clone();
    Py::None
}
pub fn _readable(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut keep: Py = Py::None;
    let mut option: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The options a person could actually be choosing between.\n\n    A list read off an answer that was not an offer is full of the answer's own figures, so\n    anything carrying a quantity is dropped, and so is a mouthful that runs to a whole clause. A\n    thing being chosen is a name (\"eggs on toast\", \"Croatia before the crowds\"), not a measurement.\n    ");
    keep = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&options.clone());
    for __item in __iter1 {
    option = __item.clone();
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MEASURE(), "search", args))(&__args) } } } } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&keep.clone(), "append", args))(&__args) };
    }
    return keep.clone();
    Py::None
}
pub fn _already_picked(args: &[Py]) -> Py {
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
    let mut back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { Py::Int(3) };
    let mut match_: Py = Py::None;
    let mut option: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The one thing the bot said it would choose, if it said one a turn or two ago.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(back.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::recent(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "endswith", args))(&__args) } } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&op_add(&py_tuple(vec![g__INSIST_SAID()]), &g__PICK_SAID()));
    for __item in __iter2 {
    pattern = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    option = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _peeled(args))(&__args) };
    if py_truthy(&{ let __b = option.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&option.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&option.clone(), &py_str("?"))) } } } }) {
    return option.clone();
    }
    }
    }
    return Py::None;
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut chosen: Py = Py::None;
    let mut line: Py = Py::None;
    let mut o: Py = Py::None;
    let mut options: Py = Py::None;
    let mut picked: Py = Py::None;
    let mut said: Py = Py::None;
    let mut short: Py = Py::None;
    let mut shrug: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One option from what the bot just offered, or None when there was no choice to make.");
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_for_a_pick(args))(&__args) };
    shrug = if py_truthy(&asked.clone()) { Py::Bool(false) } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BARE_HAND_BACK(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = asked.clone(); if py_truthy(&__b) { __b } else { shrug.clone() } }))) {
    return Py::None;
    }
    turns = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    picked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _already_picked(args))(&__args) };
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::last_bot(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| options_in(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _readable(args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _from_user_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _readable(args))(&__args) };
    }
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    short = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&o.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8))) {  __out.push(o.clone()); } } py_list(__out) })() };
    options = { let __b = short.clone(); if py_truthy(&__b) { __b } else { options.clone() } };
    chosen = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("random", "Random", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "choice", args))(&__args) };
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PICKS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(7))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("random", "Random", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "randrange", args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("option".to_string(), chosen.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_PICKS(), &line.clone()), "format", args))(&__args) };
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&picked.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("option".to_string(), picked.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_INSISTED()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_for_help(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_BY_A_RULE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _the_thing_in(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut opened: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut w: Py = Py::None;
    let mut weak: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words_after: Py = Py::None;
    let _ = py_str("The thing named after the word that says they changed their mind, or \"\".");
    weak = Py::Bool(false);
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STRONG_CHANGE(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WEAK_CHANGE(), "search", args))(&__args) };
    weak = Py::Bool(!py_is(&said.clone(), &Py::None));
    }
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NO_THEN_THING(), "match", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)) } }) {
    return py_str("");
    }
    } else {
    if py_truthy(&{ let __b = weak.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&line.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _hands_the_choice_over(args))(&__args) })) } }) {
    return py_str("");
    }
    }
    tail = slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "end", args))(&__args) }, &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    _u = __item.clone();
    opened = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTION_OPENER(), "match", args))(&__args) };
    rest = if py_truthy(&opened.clone()) { slice_or_raise(&tail.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&opened.clone(), "end", args))(&__args) }, &Py::None, &Py::None) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "strip", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NO_CONTINUES(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) }, "strip", args))(&__args) })) } }) {
    break;
    }
    tail = rest.clone();
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    _u = __item.clone();
    frame = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SECOND_FRAME(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&frame.clone()))) {
    break;
    }
    tail = slice_or_raise(&tail.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "end", args))(&__args) }, &Py::None, &Py::None);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTION(), "match", args))(&__args) }) {
    return py_str("");
    }
    words_after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tail.clone(), "split", args))(&__args) }) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.!?;:'\"()")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    kept = py_list(vec![]);
    lead = py_str("");
    let __iter3 = py_iter_or_raise(&words_after.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DETERMINERS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&kept.clone())) } }) {
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) };
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&kept.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TRAIL(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) })) } }) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)))) {
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str("instead"))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&line.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_in(args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VERBISH(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&kept.clone(), &Py::Int(0)), "lower", args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_AT_THE_START(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&kept.clone(), &Py::Int(0)), "lower", args))(&__args) }))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = weak.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DEFINITE(), &lead.clone())) } }) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&if py_truthy(&lead.clone()) { py_list(vec![lead.clone()]) } else { py_list(vec![]) }, &kept.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _hands_the_choice_over(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words_: Py = Py::None;
    let _ = py_str("True when the words before a weak change word are only the ones that hand a choice over.");
    words_ = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[A-Za-z']+")); __args.push({ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut w: Py = __item.clone(); __out.push(w.clone()); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words_.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CUE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn settled_on(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The thing the person last settled on, in their own words, or \"\".\n\n    Read from the last few things they said, newest first, and only from a line that carries a\n    change of mind: the thing has to be named in the line that moved the choice.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::recent(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_in(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _bare_change(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_choice_in_hand(args))(&__args) })) } }) {
    continue;
    }
    return thing.clone();
    }
    return py_str("");
    Py::None
}
pub fn _bare_change(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The change word when every one the line carries is an ordinary English one, or \"\".\n\n    \"actually make it 60\" carries both kinds, and the explicit one says the choice was made, so\n    the line is read without asking for a choice already on the table.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STRONG_CHANGE(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_BARE_CHANGE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }))) {
    return py_str("");
    }
    }
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STRONG_CHANGE(), "search", args))(&__args) };
    return if py_truthy(&first.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _a_choice_in_hand(args: &[Py]) -> Py {
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
    let mut this: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "this") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: this"))) };
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when an earlier line of the chat put a choice on the table.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(8)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::recent(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&line.clone(), &this.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CHOICE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }, &py_str(" or "))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_decision(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing they settled on, said back, on a deciding ask. None on every other line.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DECIDING_ASK(), "match", args))(&__args) }))) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| settled_on(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_way_of_saying_it(args))(&__args) };
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _a_way_of_saying_it(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut frame: Py = Py::None;
    let mut frames: Py = Py::None;
    let mut last: Py = Py::None;
    let _ = py_str("One of the SETTLED lines, not the one used on the last deciding ask.");
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::last_bot(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    frames = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SETTLED()) { let mut frame: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) }, "lower", args))(&__args) }, &last.clone()))) {  __out.push(frame.clone()); } } py_list(__out) })() };
    frames = { let __b = frames.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SETTLED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(frames.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("random", "Random", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "choice", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn offered(args: &[Py]) -> Py {
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
    let mut n: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(1) };
    let mut out: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The options the bot offered over its last n replies, oldest first.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(n.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::recent(args))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| options_in(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "extend", args))(&__args) };
    }
    return out.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.choice".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CHOICE" => Some(g_A_CHOICE()),
            "BARE_CHANGE" => Some(g_BARE_CHANGE()),
            "BY_A_RULE" => Some(g_BY_A_RULE()),
            "CHANGED_IT" => Some(g_CHANGED_IT()),
            "CORRECTION" => Some(g_CORRECTION()),
            "CORRECTION_OPENER" => Some(g_CORRECTION_OPENER()),
            "CUE" => Some(g_CUE()),
            "DECIDING_ASK" => Some(g_DECIDING_ASK()),
            "DEFINITE" => Some(g_DEFINITE()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "INSISTED" => Some(g_INSISTED()),
            "NOT_AT_THE_START" => Some(g_NOT_AT_THE_START()),
            "NOT_THE_THING" => Some(g_NOT_THE_THING()),
            "NO_CONTINUES" => Some(g_NO_CONTINUES()),
            "NO_THEN_THING" => Some(g_NO_THEN_THING()),
            "PICKS" => Some(g_PICKS()),
            "SECOND_FRAME" => Some(g_SECOND_FRAME()),
            "SETTLED" => Some(g_SETTLED()),
            "STRONG_CHANGE" => Some(g_STRONG_CHANGE()),
            "TRAIL" => Some(g_TRAIL()),
            "VERBISH" => Some(g_VERBISH()),
            "WEAK_CHANGE" => Some(g_WEAK_CHANGE()),
            "_ASKED" => Some(g__ASKED()),
            "_A_MEASURE" => Some(g__A_MEASURE()),
            "_BARE_HAND_BACK" => Some(g__BARE_HAND_BACK()),
            "_CLAUSE" => Some(g__CLAUSE()),
            "_INSIST_LEAD" => Some(g__INSIST_LEAD()),
            "_INSIST_SAID" => Some(g__INSIST_SAID()),
            "_INSIST_WRAP" => Some(g__INSIST_WRAP()),
            "_LEAD" => Some(g__LEAD()),
            "_NOT_A_THING" => Some(g__NOT_A_THING()),
            "_PICK_ASK" => Some(g__PICK_ASK()),
            "_PICK_SAID" => Some(g__PICK_SAID()),
            "_STEP" => Some(g__STEP()),
            "_TAIL_STRIP" => Some(g__TAIL_STRIP()),
            "_a_choice_in_hand" => Some(py_func("_a_choice_in_hand", |args: &[Py]| _a_choice_in_hand(args))),
            "_a_way_of_saying_it" => Some(py_func("_a_way_of_saying_it", |args: &[Py]| _a_way_of_saying_it(args))),
            "_already_picked" => Some(py_func("_already_picked", |args: &[Py]| _already_picked(args))),
            "_asked_for_help" => Some(py_func("_asked_for_help", |args: &[Py]| _asked_for_help(args))),
            "_bare_change" => Some(py_func("_bare_change", |args: &[Py]| _bare_change(args))),
            "_clean_option" => Some(py_func("_clean_option", |args: &[Py]| _clean_option(args))),
            "_from_italics" => Some(py_func("_from_italics", |args: &[Py]| _from_italics(args))),
            "_from_labels" => Some(py_func("_from_labels", |args: &[Py]| _from_labels(args))),
            "_from_list" => Some(py_func("_from_list", |args: &[Py]| _from_list(args))),
            "_from_question" => Some(py_func("_from_question", |args: &[Py]| _from_question(args))),
            "_from_user_line" => Some(py_func("_from_user_line", |args: &[Py]| _from_user_line(args))),
            "_hands_the_choice_over" => Some(py_func("_hands_the_choice_over", |args: &[Py]| _hands_the_choice_over(args))),
            "_peeled" => Some(py_func("_peeled", |args: &[Py]| _peeled(args))),
            "_readable" => Some(py_func("_readable", |args: &[Py]| _readable(args))),
            "_segment" => Some(py_func("_segment", |args: &[Py]| _segment(args))),
            "_the_thing_in" => Some(py_func("_the_thing_in", |args: &[Py]| _the_thing_in(args))),
            "asks_for_a_pick" => Some(py_func("asks_for_a_pick", |args: &[Py]| asks_for_a_pick(args))),
            "offered" => Some(py_func("offered", |args: &[Py]| offered(args))),
            "options_in" => Some(py_func("options_in", |args: &[Py]| options_in(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "settled_on" => Some(py_func("settled_on", |args: &[Py]| settled_on(args))),
            "the_decision" => Some(py_func("the_decision", |args: &[Py]| the_decision(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CHOICE" => s_A_CHOICE(v),
            "BARE_CHANGE" => s_BARE_CHANGE(v),
            "BY_A_RULE" => s_BY_A_RULE(v),
            "CHANGED_IT" => s_CHANGED_IT(v),
            "CORRECTION" => s_CORRECTION(v),
            "CORRECTION_OPENER" => s_CORRECTION_OPENER(v),
            "CUE" => s_CUE(v),
            "DECIDING_ASK" => s_DECIDING_ASK(v),
            "DEFINITE" => s_DEFINITE(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "INSISTED" => s_INSISTED(v),
            "NOT_AT_THE_START" => s_NOT_AT_THE_START(v),
            "NOT_THE_THING" => s_NOT_THE_THING(v),
            "NO_CONTINUES" => s_NO_CONTINUES(v),
            "NO_THEN_THING" => s_NO_THEN_THING(v),
            "PICKS" => s_PICKS(v),
            "SECOND_FRAME" => s_SECOND_FRAME(v),
            "SETTLED" => s_SETTLED(v),
            "STRONG_CHANGE" => s_STRONG_CHANGE(v),
            "TRAIL" => s_TRAIL(v),
            "VERBISH" => s_VERBISH(v),
            "WEAK_CHANGE" => s_WEAK_CHANGE(v),
            "_ASKED" => s__ASKED(v),
            "_A_MEASURE" => s__A_MEASURE(v),
            "_BARE_HAND_BACK" => s__BARE_HAND_BACK(v),
            "_CLAUSE" => s__CLAUSE(v),
            "_INSIST_LEAD" => s__INSIST_LEAD(v),
            "_INSIST_SAID" => s__INSIST_SAID(v),
            "_INSIST_WRAP" => s__INSIST_WRAP(v),
            "_LEAD" => s__LEAD(v),
            "_NOT_A_THING" => s__NOT_A_THING(v),
            "_PICK_ASK" => s__PICK_ASK(v),
            "_PICK_SAID" => s__PICK_SAID(v),
            "_STEP" => s__STEP(v),
            "_TAIL_STRIP" => s__TAIL_STRIP(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.choice", module_obj);
    pyrt::xrefs::reg_fn("memory.choice", "_clean_option", py_func("_clean_option", |args: &[Py]| _clean_option(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_from_italics", py_func("_from_italics", |args: &[Py]| _from_italics(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_segment", py_func("_segment", |args: &[Py]| _segment(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_from_list", py_func("_from_list", |args: &[Py]| _from_list(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_from_labels", py_func("_from_labels", |args: &[Py]| _from_labels(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_from_question", py_func("_from_question", |args: &[Py]| _from_question(args)));
    pyrt::xrefs::reg_fn("memory.choice", "options_in", py_func("options_in", |args: &[Py]| options_in(args)));
    pyrt::xrefs::reg_fn("memory.choice", "asks_for_a_pick", py_func("asks_for_a_pick", |args: &[Py]| asks_for_a_pick(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_from_user_line", py_func("_from_user_line", |args: &[Py]| _from_user_line(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_asked_for_help", py_func("_asked_for_help", |args: &[Py]| _asked_for_help(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_peeled", py_func("_peeled", |args: &[Py]| _peeled(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_readable", py_func("_readable", |args: &[Py]| _readable(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_already_picked", py_func("_already_picked", |args: &[Py]| _already_picked(args)));
    pyrt::xrefs::reg_fn("memory.choice", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_the_thing_in", py_func("_the_thing_in", |args: &[Py]| _the_thing_in(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_hands_the_choice_over", py_func("_hands_the_choice_over", |args: &[Py]| _hands_the_choice_over(args)));
    pyrt::xrefs::reg_fn("memory.choice", "settled_on", py_func("settled_on", |args: &[Py]| settled_on(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_bare_change", py_func("_bare_change", |args: &[Py]| _bare_change(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_a_choice_in_hand", py_func("_a_choice_in_hand", |args: &[Py]| _a_choice_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.choice", "the_decision", py_func("the_decision", |args: &[Py]| the_decision(args)));
    pyrt::xrefs::reg_fn("memory.choice", "_a_way_of_saying_it", py_func("_a_way_of_saying_it", |args: &[Py]| _a_way_of_saying_it(args)));
    pyrt::xrefs::reg_fn("memory.choice", "offered", py_func("offered", |args: &[Py]| offered(args)));
    pyrt::xrefs::reg_access("memory.choice", "A_CHOICE", g_A_CHOICE, s_A_CHOICE);
    pyrt::xrefs::reg_access("memory.choice", "BARE_CHANGE", g_BARE_CHANGE, s_BARE_CHANGE);
    pyrt::xrefs::reg_access("memory.choice", "BY_A_RULE", g_BY_A_RULE, s_BY_A_RULE);
    pyrt::xrefs::reg_access("memory.choice", "CHANGED_IT", g_CHANGED_IT, s_CHANGED_IT);
    pyrt::xrefs::reg_access("memory.choice", "CORRECTION", g_CORRECTION, s_CORRECTION);
    pyrt::xrefs::reg_access("memory.choice", "CORRECTION_OPENER", g_CORRECTION_OPENER, s_CORRECTION_OPENER);
    pyrt::xrefs::reg_access("memory.choice", "CUE", g_CUE, s_CUE);
    pyrt::xrefs::reg_access("memory.choice", "DECIDING_ASK", g_DECIDING_ASK, s_DECIDING_ASK);
    pyrt::xrefs::reg_access("memory.choice", "DEFINITE", g_DEFINITE, s_DEFINITE);
    pyrt::xrefs::reg_access("memory.choice", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("memory.choice", "INSISTED", g_INSISTED, s_INSISTED);
    pyrt::xrefs::reg_access("memory.choice", "NOT_AT_THE_START", g_NOT_AT_THE_START, s_NOT_AT_THE_START);
    pyrt::xrefs::reg_access("memory.choice", "NOT_THE_THING", g_NOT_THE_THING, s_NOT_THE_THING);
    pyrt::xrefs::reg_access("memory.choice", "NO_CONTINUES", g_NO_CONTINUES, s_NO_CONTINUES);
    pyrt::xrefs::reg_access("memory.choice", "NO_THEN_THING", g_NO_THEN_THING, s_NO_THEN_THING);
    pyrt::xrefs::reg_access("memory.choice", "PICKS", g_PICKS, s_PICKS);
    pyrt::xrefs::reg_access("memory.choice", "SECOND_FRAME", g_SECOND_FRAME, s_SECOND_FRAME);
    pyrt::xrefs::reg_access("memory.choice", "SETTLED", g_SETTLED, s_SETTLED);
    pyrt::xrefs::reg_access("memory.choice", "STRONG_CHANGE", g_STRONG_CHANGE, s_STRONG_CHANGE);
    pyrt::xrefs::reg_access("memory.choice", "TRAIL", g_TRAIL, s_TRAIL);
    pyrt::xrefs::reg_access("memory.choice", "VERBISH", g_VERBISH, s_VERBISH);
    pyrt::xrefs::reg_access("memory.choice", "WEAK_CHANGE", g_WEAK_CHANGE, s_WEAK_CHANGE);
    pyrt::xrefs::reg_access("memory.choice", "_ASKED", g__ASKED, s__ASKED);
    pyrt::xrefs::reg_access("memory.choice", "_A_MEASURE", g__A_MEASURE, s__A_MEASURE);
    pyrt::xrefs::reg_access("memory.choice", "_BARE_HAND_BACK", g__BARE_HAND_BACK, s__BARE_HAND_BACK);
    pyrt::xrefs::reg_access("memory.choice", "_CLAUSE", g__CLAUSE, s__CLAUSE);
    pyrt::xrefs::reg_access("memory.choice", "_INSIST_LEAD", g__INSIST_LEAD, s__INSIST_LEAD);
    pyrt::xrefs::reg_access("memory.choice", "_INSIST_SAID", g__INSIST_SAID, s__INSIST_SAID);
    pyrt::xrefs::reg_access("memory.choice", "_INSIST_WRAP", g__INSIST_WRAP, s__INSIST_WRAP);
    pyrt::xrefs::reg_access("memory.choice", "_LEAD", g__LEAD, s__LEAD);
    pyrt::xrefs::reg_access("memory.choice", "_NOT_A_THING", g__NOT_A_THING, s__NOT_A_THING);
    pyrt::xrefs::reg_access("memory.choice", "_PICK_ASK", g__PICK_ASK, s__PICK_ASK);
    pyrt::xrefs::reg_access("memory.choice", "_PICK_SAID", g__PICK_SAID, s__PICK_SAID);
    pyrt::xrefs::reg_access("memory.choice", "_STEP", g__STEP, s__STEP);
    pyrt::xrefs::reg_access("memory.choice", "_TAIL_STRIP", g__TAIL_STRIP, s__TAIL_STRIP);
}
