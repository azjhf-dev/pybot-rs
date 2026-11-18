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


thread_local! { static G_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ASK_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_INTENTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONTRAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POINTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POSSESSION_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REAL_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REMARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERB_IN_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_OR_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SWAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWAPPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FACT_MOVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WORD_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__READINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_answer_in_hand: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_current: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_keywords: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.repair: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AN_ASK() -> Py { ensure_init(); G_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); G_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ASK_OPENS() -> Py { ensure_init(); G_AN_ASK_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK_OPENS(v: Py) { ensure_init(); G_AN_ASK_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_INTENTION() -> Py { ensure_init(); G_AN_INTENTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_INTENTION(v: Py) { ensure_init(); G_AN_INTENTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONTRAST() -> Py { ensure_init(); G_A_CONTRAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONTRAST(v: Py) { ensure_init(); G_A_CONTRAST.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DETERMINER() -> Py { ensure_init(); G_A_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DETERMINER(v: Py) { ensure_init(); G_A_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_REPAIR() -> Py { ensure_init(); G_A_FIGURE_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_REPAIR(v: Py) { ensure_init(); G_A_FIGURE_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POINTER() -> Py { ensure_init(); G_A_POINTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POINTER(v: Py) { ensure_init(); G_A_POINTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POSSESSION_REPAIR() -> Py { ensure_init(); G_A_POSSESSION_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POSSESSION_REPAIR(v: Py) { ensure_init(); G_A_POSSESSION_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REAL_ANSWER() -> Py { ensure_init(); G_A_REAL_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REAL_ANSWER(v: Py) { ensure_init(); G_A_REAL_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REMARK() -> Py { ensure_init(); G_A_REMARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REMARK(v: Py) { ensure_init(); G_A_REMARK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VALUE() -> Py { ensure_init(); G_A_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VALUE(v: Py) { ensure_init(); G_A_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERB_IN_THE_THING() -> Py { ensure_init(); G_A_VERB_IN_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERB_IN_THE_THING(v: Py) { ensure_init(); G_A_VERB_IN_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_REPAIR() -> Py { ensure_init(); G_BARE_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_REPAIR(v: Py) { ensure_init(); G_BARE_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_OR_WORD() -> Py { ensure_init(); G_FIGURE_OR_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_OR_WORD(v: Py) { ensure_init(); G_FIGURE_OR_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_UNITS() -> Py { ensure_init(); G_FIGURE_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_UNITS(v: Py) { ensure_init(); G_FIGURE_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEDGE() -> Py { ensure_init(); G_HEDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEDGE(v: Py) { ensure_init(); G_HEDGE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SWAP() -> Py { ensure_init(); G_NOT_A_SWAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SWAP(v: Py) { ensure_init(); G_NOT_A_SWAP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); G_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); G_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR() -> Py { ensure_init(); G_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR(v: Py) { ensure_init(); G_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_LEAD() -> Py { ensure_init(); G_REPEAT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_LEAD(v: Py) { ensure_init(); G_REPEAT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_ADVICE() -> Py { ensure_init(); G_SAME_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_ADVICE(v: Py) { ensure_init(); G_SAME_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_AGAIN() -> Py { ensure_init(); G_SAME_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_AGAIN(v: Py) { ensure_init(); G_SAME_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE_END() -> Py { ensure_init(); G_SENTENCE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE_END(v: Py) { ensure_init(); G_SENTENCE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_SWAPPED() -> Py { ensure_init(); G_SWAPPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWAPPED(v: Py) { ensure_init(); G_SWAPPED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FACT_MOVED() -> Py { ensure_init(); G_THE_FACT_MOVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FACT_MOVED(v: Py) { ensure_init(); G_THE_FACT_MOVED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WORD_FOR() -> Py { ensure_init(); G_THE_WORD_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WORD_FOR(v: Py) { ensure_init(); G_THE_WORD_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP() -> Py { ensure_init(); G__KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP(v: Py) { ensure_init(); G__KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g__READINGS() -> Py { ensure_init(); G__READINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__READINGS(v: Py) { ensure_init(); G__READINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_answer_in_hand() -> Py { ensure_init(); G_answer_in_hand.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_answer_in_hand(v: Py) { ensure_init(); G_answer_in_hand.with(|c| *c.borrow_mut() = v); }
pub fn g_current() -> Py { ensure_init(); G_current.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_current(v: Py) { ensure_init(); G_current.with(|c| *c.borrow_mut() = v); }
pub fn g_keywords() -> Py { ensure_init(); G_keywords.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_keywords(v: Py) { ensure_init(); G_keywords.with(|c| *c.borrow_mut() = v); }
pub fn g_last_user() -> Py { ensure_init(); G_last_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_user(v: Py) { ensure_init(); G_last_user.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("They swapped the thing they were asking about: \"no i meant the front room\".\n\nA person asks about one thing, gets an answer, and then says the thing they meant was a different\none. It is a repair, and it keeps the ask: the question is the same question about a different\nthing. Main reads only the value side of that - `no i said 700`, `sorry, i meant the 13th of june`\n- and there the store has a fact to move. When the thing swapped is the *subject* of the ask and\nno fact holds it, the line was left to the composer, and the person who corrected themselves got\nno answer at all:\n\n```\ni need to pick a paint colour for the hall\n  -> \"Pick the thing you cannot change - the floor, the tiles, the worktop...\"\nno i meant the front room\n  -> \"Thanks for telling me - I'm here if you want to dig into any of that.\"\nits quite dark in there\n  -> \"It's quite dark in there - noted.\"\n```\n\nTwo turns lost after a good answer. What they asked for is the same advice about the front room,\nand the bot has it: read as the ask they would have said if they had said it right the first time,\nit answers.\n\nSo this module builds that ask. The thing the last line named is its last content word - the plan\nis kept as the sentence it arrived in (\"Pick a paint colour for the hall\"), so the word that moves\nis the one at the end of it - and the new thing takes its place:\n\n    i need to pick a paint colour for the hall  ->  i need to pick a paint colour for the front room\n\nThree rules keep it to the lines it is good at:\n\n* the whole line has to be the repair and a thing: \"no i meant the front room\", \"sorry, i meant\n  the other one\" is refused because a pronoun names no thing, and \"no i meant 500\" is refused\n  because a value is the store's own correction to make, not a subject to re-ask.\n* the thing has to replace something: a word of its own already in the line (\"no i meant the\n  hall\" after the hall) leaves the ask exactly as it was, and answering it again is a repeat.\n* the answers have to have something to say about the ask they would have made. When they do not\n  (\"i live in bristol\" / \"no i meant leeds\" is a fact to change, not an ask to answer) this module\n  says nothing and the turn goes back to the rest of the bot, which knows how to move a fact.\n\nThe store also keeps the swap, so the turns after it are about the thing they meant: the topic is\nthe corrected line and the plan says the new thing, or \"its quite dark in there\" is answered about\nthe hall they have just taken back.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_current(pyrt::xrefs::xget("memory", "current"));
    s_last_user(py_func("last_user", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_user", args)));
    s_answer_in_hand(py_func("answer_in_hand", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "answer_in_hand", args)));
    s_keywords(py_func("keywords", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "keywords", args)));
    s_REPAIR(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:no|nah|nope|sorry|actually|oh|wait|hmm|well|ok|okay|right|yes)[,\\s]+)*(?:i\\s+(?:meant|mean)|i\\s+did\\s+mean|we\\s+meant|meant|rather)\\b[,\\s:]+(?P<thing>\\S.*?)\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_REPAIR(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?P<frame>no|nah|nope|sorry|actually|oh|wait)[,\\s]+)+(?P<thing>[^.!?]+?)\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SWAP(call_builtin("frozenset", &[mcall(&py_str("\n    thanks thank worries worry problem problems idea matter matters clue doubt chance way luck\n    bother hurry rush need point sense reason question questions bother drama fault apologies\n    apology offence offense harm damage wonder need boat idea lot bit little\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POSSESSION_REPAIR(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?P<frame>no|nah|nope|sorry|actually|oh|wait)[,\\s]+)+(?:i|we)(?:'ve|ve|\\s+have|\\s+have\\s+got|\\s+got)\\s+(?:got\\s+)?(?P<thing>[^.!?]+?)\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VERB_IN_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\n    is are was were am be been being do does did done doing have has had having\n    can could should would will shall may might must\n    want wants wanted need needs needed got get gets got make makes made take takes took\n    give gives gave go goes went gone like likes liked love loves loved hate hates hated\n    feel feels felt hurt hurts hurt think thinks thought know knows knew said says say\n    tell tells told see sees saw seen try tries tried use uses used work works worked\n    live lives lived play plays played eat eats ate drink drinks drank buy buys bought\n    sell sells sold pay pays paid find finds found lose loses lost break breaks broke\n    fix fixes fixed start starts started stop stops stopped keep keeps kept put puts\n    run runs ran walk walks walked cook cooks cooked clean cleans cleaned\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_INTENTION(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+meant\\s+to\\s+(?!say\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VALUE(stdlib_call("re", "compile", &[py_str("\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONTRAST(stdlib_call("re", "compile", &[py_str("\\b(?:not|instead|isn'?t|wasn'?t|isnt|wasnt)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ASK(call_builtin("frozenset", &[mcall(&py_str("\n    should shall can could would will may might must do does did is are was were am\n    have has had what whats how when where which who why whose\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\n    it that this these those them one other the a an something anything nothing whatever thing\n    things what which same more less first second last next another both all here there\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REMARK(call_builtin("frozenset", &[mcall(&py_str("\n    i im id ill ive we you he she they it its that there this just only also really very so\n    whatever anyway honestly\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\n    a an the my our your his her their its some any this that these those it them\n    is are was were am be been being do does did done doing have has had having\n    can could should would will shall may might must\n    and or but so then than too also just very really quite more most less least\n    for with without about at in on of to from by into onto over under again\n    what whats how when where which who why whose way much many long\n    good best great nice better bad worse ok okay please thanks thank\n    today tomorrow tonight yesterday now soon later weekend weekends weekday weekdays\n    this weekend next weekend last weekend\n    one ones thing things stuff lot lots bit little\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DETERMINER(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|my|our|your|his|her|their|this|that|these|those)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__READINGS(py_dict(vec![]));
    s_A_POINTER(call_builtin("frozenset", &[py_tuple(vec![py_str("one"), py_str("ones")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE_REPAIR(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:no|nah|nope|sorry|actually|oh|wait|hmm|well|ok|okay|right|yes)[,\\s]+)*(?:i\\s+(?:meant|mean)|i\\s+did\\s+mean|we\\s+meant|meant)\\b[\\s,:-]+(?P<figure>\\d+(?:\\.\\d+)?)\\s*(?P<unit>[a-z]{0,10})\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_UNITS(call_builtin("frozenset", &[mcall(&py_str("\n    kg kilo kilos g gram grams lb lbs oz litre litres liter liters ml pint pints\n    pound pounds quid hours hour mins minutes min days day weeks week months month years year\n    people person guests adults kids children degrees degree percent\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?P<figure>\\d+(?:\\.\\d+)?)\\s*(?P<unit>[a-z]{0,12})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_OR_WORD(stdlib_call("re", "compile", &[py_str("(?<![\\d.])\\d+(?:\\.\\d+)?(?![\\d])|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE_END(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WORD_FOR({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("memory.facts", "FIGURE_WORDS"), "items", &[])) { let __unpk9607 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk9607[0].clone(); let mut number: Py = __unpk9607[1].clone(); __out.push((number.clone(), word.clone())); } py_dict(__out) })() });
    s_SWAPPED(py_tuple(vec![py_str("{thing}, then."), py_str("Got it — {thing}."), py_str("{thing} it is.")]));
    s_AN_ASK_OPENS(stdlib_call("re", "compile", &[py_str("^(?:how|what|why|when|where|which|who|whose|whom)\\b|^(?:can|could|should|would|will|shall|do|does|did|is|are|was|were|have|has|had|am|may|might|must|need)\\s+(?:i|we|you|they|it|he|she|that|this|there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGE(stdlib_call("re", "compile", &[py_str("\\b(?:beyond what i know|don'?t know enough|a quick search|i'?d rather not guess|not sure about that|i don'?t know that one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REAL_ANSWER(Py::Int(80));
    s_REPEAT_LEAD(stdlib_call("re", "compile", &[py_str("^[^:]{0,90}\\b(?:same goes for|same advice (?:as|for|as for)|same as (?:for |the |as )?)\\b[^:]{0,90}:\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAME_AGAIN(py_tuple(vec![py_str("{said}, then — the same advice as for the {old}: {answer}"), py_str("Got it — {thing}. The same advice as for the {old} holds: {answer}"), py_str("{said}, then. Same advice as the {old}: {answer}")]));
    s_SAME_ADVICE(py_tuple(vec![py_str("{said}, then — the same advice as for the {old}."), py_str("Got it — {thing}. The same advice as for the {old} applies."), py_str("{said}, then. Same advice as the {old}.")]));
    s_THE_FACT_MOVED(py_tuple(vec![py_str("Got it — your {new}'s {fact}, not your {old}."), py_str("Noted — your {new} is the one who's {fact}."), py_str("Your {new}, then, not your {old} — your {new}'s {fact}.")]));
    s__KEEP(Py::Bool(true));
}
pub fn _the_thing(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the repair names, in the words it was said in, or None.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&mcall(&g_AN_INTENTION(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_CONTRAST(), "search", &[said.clone()])) {
    return Py::None;
    }
    match_ = mcall(&g_REPAIR(), "match", &[said.clone()]);
    bare = Py::Bool(false);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = mcall(&g_A_POSSESSION_REPAIR(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = mcall(&g_BARE_REPAIR(), "match", &[said.clone()]);
    if py_truthy(&match_.clone()) {
    bare = Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    }
    thing = stdlib_call("re", "sub", &[py_str("^to\\s+(?:say\\s+|mean\\s+)?"), py_str(""), mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    thing = mcall(&thing.clone(), "strip", &[py_str(" ,.!?;:\"'")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_VALUE(), "search", &[thing.clone()]) } }) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&thing.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_REMARK(), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DETERMINER(), "match", &[thing.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NOT_THE_THING(), &word.clone()))) {  __out.push(Py::Bool(py_contains(&g_NOT_A_SWAP(), &word.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_VERB_IN_THE_THING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_AN_ASK(), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &geti(&words.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&words.clone(), &op_neg(&Py::Int(1))), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return if py_truthy(&Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&op_bitor(&g_NOT_A_THING(), &g_NOT_THE_THING()), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) { thing.clone() } else { Py::None };
    }
    return thing.clone();
    Py::None
}
pub fn _a_different_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut old: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut held: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the thing they meant is a thing of its own, and not the one already in the ask.\n\n    A word of the old thing inside the new one is a refinement, not a repeat: \"i lost my keys\"\n    then \"no my car keys\" is the answer about car keys, and \"how long do i boil an egg\" then \"no a\n    duck egg\" is the egg answer for a duck egg. The line is unchanged only when the new thing says\n    nothing the old one did not - \"no i meant the hall\" after the hall is the same ask, and\n    answering it again is a repeat.\n    ");
    held = _content_words(&[old.clone()]);
    said = _content_words(&[thing.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[op_sub(&said.clone(), &held.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _content_words(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The words a thing is named with, determiner and all left out.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'")])); } py_list(__out) })() };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("its"), py_str("some"), py_str("any"), py_str("this"), py_str("that")]), &word.clone())) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _the_reading(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut understand: Py = Py::None;
    let _ = py_str("The understanding of the line the correction is about, read once and kept.");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g__READINGS(), &said.clone()))) {
    return geti(&g__READINGS(), &said.clone());
    }
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    let __try1 = exc::catch(|| {
    read = py_call(&understand, &[said.clone(), py_dict(vec![(py_str("turns"), py_list(vec![]))])]);
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
    read = py_dict(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    read = py_dict(vec![]);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[g__READINGS()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12))) {
    let _ = mcall(&g__READINGS(), "clear", &[]);
    }
    let _ = py_setitem(&g__READINGS(), &said.clone(), read.clone());
    return read.clone();
    Py::None
}
pub fn _the_thing_the_reading_named(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The thing the reading finds in the line, or \"\" when it finds no single one.\n\n    \"the washing machine is broken\" then \"no the dishwasher\" is a swap of the machine. The last\n    content word of that line is \"broken\" - the state the thing is in, not the thing - so the\n    corrected ask came out as \"the washing machine is dishwasher\" and the row on how to run a\n    wash answered it. The reading names one thing there (\"washing machine\"), and the swap made on\n    that gives \"the dishwasher is broken\", which is the dishwasher's own row. Only a thing, never\n    a person: a correction of a person is `_the_person_the_line_named`'s to move.\n    ");
    read = _the_reading(&[line.clone()]);
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&read.clone(), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("thing"), py_str("animal"), py_str("food"), py_str("place")]), &call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("type")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    continue;
    }
    at = mcall(&one.clone(), "get", &[py_str("at")]);
    at = if py_truthy(&call_builtin("isinstance", &[at.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { at.clone() } else { Py::None };
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![at.clone(), text.clone()])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let at = at.clone(); let before = before.clone(); let found = found.clone(); let index = index.clone(); let line = line.clone(); let one = one.clone(); let read = read.clone(); let said = said.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let at = at.clone(); let before = before.clone(); let found = found.clone(); let index = index.clone(); let line = line.clone(); let one = one.clone(); let read = read.clone(); let said = said.clone(); let text = text.clone(); let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); py_tuple(vec![Py::Bool(py_is(&geti(&pair.clone(), &Py::Int(0)), &Py::None)), geti(&pair.clone(), &Py::Int(0))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    one = geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(1));
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&found.clone(), &Py::Int(1), &Py::None, &Py::None)); __kws.extend(vec![("start".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    let __unpk4 = py_unpack(&__unpk3[1].clone(), 2, None);
    at = __unpk4[0].clone();
    text = __unpk4[1].clone();
    before = geti(&found.clone(), &op_sub(&index.clone(), &Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(!py_is(&at.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&geti(&before.clone(), &Py::Int(0)), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&at.clone(), &op_add(&geti(&before.clone(), &Py::Int(0)), &call_builtin("len", &[mcall(&geti(&before.clone(), &Py::Int(1)), "split", &[])]).unwrap_or_else(|e| exc::raise(e))))) } } } }) {
    one = op_add(&op_add(&one.clone(), &py_str(" ")), &text.clone());
    } else {
    return py_str("");
    }
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return if py_truthy(&Py::Bool(py_contains(&said.clone(), &one.clone()))) { one.clone() } else { py_str("") };
    Py::None
}
pub fn _the_thing_the_last_line_named(args: &[Py]) -> Py {
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
    let _ = py_str("The thing the ask before the repair was about: its last content word, or None.\n\n    A plan is kept as the sentence it arrived in, so the thing a line is about is at the end of\n    it: \"pick a paint colour for the hall\" is about the hall, \"cook a whole chicken\" is about the\n    chicken. A line that ends on a grammar word, a figure or a date named no thing this way, and\n    the swap is not made.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DAYS"), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "isdigit", &[]) } } } }) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.topics", "keywords", &[word.clone()])) {
    return word.clone();
    }
    return Py::None;
    }
    return Py::None;
    Py::None
}
pub fn _the_noun_the_line_named(args: &[Py]) -> Py {
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
    let _ = py_str("The thing a pointer counts: the last noun the ask named, with the verbs passed over.\n\n    \"my back hurts\" names the back, not the hurting, and the hurting is the word at the end of the\n    line. A word that says what the thing is doing is not the thing.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_VERB_IN_THE_THING(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DAYS"), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.topics", "keywords", &[word.clone()])) {
    return word.clone();
    }
    return Py::None;
    }
    return Py::None;
    Py::None
}
pub fn _the_pointer_refines(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut mods: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What the pointer refines, as the thing to swap in, or None.\n\n    Returns the pair (the word it counts, the thing to put in its place): \"the lower one\" about\n    \"my back hurts\" is (\"back\", \"lower back\").\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_POINTER(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    return Py::None;
    }
    mods = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_THE_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&mods.clone()))) {
    return Py::None;
    }
    noun = _the_noun_the_line_named(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&noun.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![noun.clone(), op_add(&op_add(&mcall(&py_str(" "), "join", &[mods.clone()]), &py_str(" ")), &noun.clone())]);
    Py::None
}
pub fn _the_relation(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("The relation a word names, allowing for the way it is typed: \"grans\" is \"gran\".\n\n    \"my grans 84\" is \"my gran's 84\" with the apostrophe dropped, which is how most people type\n    it, and the word that moves on \"sorry i meant my nan\" is that gran. Without this the line was\n    read as being about a \"grans\", which is nobody, so the correction had no person to move and\n    the age stayed on the gran.\n    ");
    plain = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "strip", &[py_str("'")]);
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &plain.clone()))) {
    return plain.clone();
    }
    if py_truthy(&mcall(&plain.clone(), "endswith", &[py_str("'s")])) {
    plain = slice_or_raise(&plain.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    } else {
    if py_truthy(&mcall(&plain.clone(), "endswith", &[py_str("s")])) {
    plain = slice_or_raise(&plain.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &plain.clone()))) {
    return plain.clone();
    }
    if py_truthy(&mcall(&plain.clone(), "endswith", &[py_str("'")])) {
    plain = slice_or_raise(&plain.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &plain.clone()))) { plain.clone() } else { Py::None };
    Py::None
}
pub fn _the_person_the_line_named(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut named: Py = Py::None;
    let mut person: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person a correction of who is about: \"my son has a cough\" / \"sorry, i meant my daughter\".\n\n    A correction of a person often names somebody who would not fit where the last content word\n    of the line goes - \"my son has a cough\" with \"my daughter\" is not \"my son has a my daughter\" -\n    and the word that moves is the person the line was about. Only a correction that names a\n    person of its own is read this way; anything else keeps to the last content word.\n    ");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_the_relation(&[word.clone()])); } py_list(__out) })() }) { let mut person: Py = __item.clone(); if py_truthy(&person.clone()) {  __out.push(person.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    person = _the_relation(&[word.clone()]);
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&named.clone(), &person.clone())) } }) {
    return person.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _swapped(args: &[Py]) -> Py {
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
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut new: Py = Py::None;
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The ask they would have said if they had said it right the first time.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    new = mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    tail = if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &old.clone()))) { py_str("(?:'s|s)?") } else { py_str("") };
    match_ = stdlib_call("re", "search", &[op_add(&op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), &tail.clone()), &py_str("\\b")), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    before = mcall(&slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[]);
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "match", &[py_str("^(?:a|an|the|my|our|your|his|her|their|this|that|these|those)\\b"), new.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    new = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the|my|our|your|his|her|their|this|that|these|those)\\s+")); __args.push(py_str("")); __args.push(new.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    out = op_add(&op_add(&slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &new.clone()), &slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None));
    out = _no_repeat(&[mcall(&py_str(" "), "join", &[mcall(&out.clone(), "split", &[])])]);
    return if py_truthy(&Py::Bool(py_eq(&mcall(&out.clone(), "lower", &[]), &mcall(&said.clone(), "lower", &[])))) { py_str("") } else { out.clone() };
    Py::None
}
pub fn _no_repeat(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line without a run of words the swap made twice: \"a new new shower\" -> \"a new shower\".\n\n    The new thing brings its own words and the old one had them in front of it, so \"i need a new\n    boiler\" with \"a new shower\" is \"i need a new new shower\" unless one run goes. The words before\n    the thing are read from the left, so a thing that repeats a word already in the sentence keeps\n    the sentence's own word and drops its copy.\n    ");
    words = mcall(&line.clone(), "split", &[]);
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&word.clone(), "lower", &[]), &mcall(&geti(&out.clone(), &op_neg(&Py::Int(1))), "lower", &[]))) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _the_answer(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answers: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut key: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut understand: Py = Py::None;
    let _ = py_str("What the answers say about the ask they would have made, or None.\n\n    The readers are given this turn's memory, so the line is read on its own the way the\n    corrected ask would have been read on the turn it was first made: a swap is not a follow-up\n    to the thing it takes back.\n    ");
    answers = pyrt::xrefs::xmodule("modules.answers");
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    fresh = py_dict(vec![(py_str("turns"), py_list(vec![])), (py_str("aside"), Py::Bool(true)), (py_str("understood"), py_call(&understand, &[line.clone(), py_dict(vec![(py_str("turns"), py_list(vec![]))])]))]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("user"), py_str("others"), py_str("facts_log")]));
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&memory.clone(), &key.clone()))) {
    let _ = py_setitem(&fresh.clone(), &key.clone(), geti(&memory.clone(), &key.clone()));
    }
    }
    let __try2 = exc::catch(|| {
    reply = mcall(&answers.clone(), "respond", &[line.clone(), fresh.clone()]);
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
    return Py::None;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&reply.clone(), "strip", &[]) } }) {
    let __iter7 = py_iter_or_raise(&py_tuple(vec![py_str("entry_answer"), py_str("entry_answers")]));
    for __item in __iter7 {
    key = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&fresh.clone(), &key.clone()))) {
    let _ = py_setitem(&memory.clone(), &key.clone(), geti(&fresh.clone(), &key.clone()));
    }
    }
    return mcall(&reply.clone(), "strip", &[]);
    }
    return Py::None;
    Py::None
}
pub fn _the_plain_answer(args: &[Py]) -> Py {
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
    let mut answers: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("What the answers say about the line as it stands, read with the chat it is in.");
    answers = pyrt::xrefs::xmodule("modules.answers");
    let __try1 = exc::catch(|| {
    reply = mcall(&answers.clone(), "respond", &[text.clone(), memory.clone()]);
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
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return { let __b = said.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _about_the_same_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut first: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "first") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: first"))) };
    let mut second: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "second") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: second"))) };
    let mut left: Py = Py::None;
    let mut right: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether two answers are about the same thing, by the content words they share.\n\n    A correction line is read for an answer of its own, and that answer only counts when it is\n    about the thing the person just named: \"sorry, i meant my dad\" read whole reached the row on\n    how to word an apology and never says \"dad\", so it is an answer to the word \"sorry\" and not to\n    what they meant, while \"sorry i meant a turkey\" read whole is the turkey. The words are the\n    content words of both, stemmed, so \"turkeys\" and \"turkey\" are one thing.\n    ");
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[call_builtin("str", &[{ let __b = first.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_set(__out) })() };
    right = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[call_builtin("str", &[{ let __b = second.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_set(__out) })() };
    return call_builtin("bool", &[op_bitand(&left.clone(), &right.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_shape_the_swap_had(args: &[Py]) -> Py {
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
    let mut old: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut last: Py = Py::None;
    let mut older: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The corrected line written the way the store has always written it: the word at the end of\n    the line swapped for the new thing.\n\n    The reply to a correction is about the thing the line named (\"the washing machine is broken\" /\n    \"no the dishwasher\" is answered about dishwashers). The record the follow-ups are read against\n    is a different matter: everything downstream - the rewritten line a bare \"what about a duck\"\n    is read as, the words a part or a measure ask is filled from - was written against the store's\n    own shape, which swaps the last word of the line. Handing that a reshaped sentence changed\n    turns that answered before: \"how long does a chicken take to roast\" / \"i meant a turkey\" /\n    \"what about a duck\" reached the duck on the old shape and got a search hedge on the new one.\n    So the reply keeps the better reading and the bookkeeping keeps the old shape, and no chat\n    downstream of a correction moves.\n    ");
    last = call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    tail = if py_truthy(&last.clone()) { _the_thing_the_last_line_named(&[last.clone()]) } else { py_str("") };
    if py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { _a_state_not_a_thing(&[last.clone(), tail.clone()]) } }) {
    return line.clone();
    }
    if py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&tail.clone(), &old.clone())); if !py_truthy(&__b) { __b } else { _a_different_thing(&[tail.clone(), thing.clone()]) } } } }) {
    older = _swapped(&[last.clone(), tail.clone(), thing.clone()]);
    if py_truthy(&older.clone()) {
    return older.clone();
    }
    }
    return line.clone();
    Py::None
}
pub fn _a_state_not_a_thing(args: &[Py]) -> Py {
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
    let mut tail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tail"))) };
    let mut pattern: Py = Py::None;
    let _ = py_str("True when the word the line ends on is what the thing is doing or is, not a thing itself.\n\n    \"the boiler is noisy\" ends on what it is like, and \"i need a new boiler\" ends on the thing. The\n    copula in front of it is what tells the two apart, and it is the whole of the test on purpose:\n    a state word chosen from a list would miss the next one somebody says.\n    ");
    pattern = op_add(&op_add(&py_str("\\b(?:is|are|was|were|isn'?t|aren'?t|wasn'?t|weren'?t|feels?|seems?|looks?|gets?|got|goes|went|has|have|had|keeps?|stays?|stops?)\\s+"), &stdlib_call("re", "escape", &[call_builtin("str", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b"));
    return call_builtin("bool", &[stdlib_call("re", "search", &[pattern.clone(), call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _keep_the_swap(args: &[Py]) -> Py {
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
    let mut old: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut one: Py = Py::None;
    let mut record: Py = Py::None;
    let mut topics: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The topic and the plan are the corrected line, so the turns after it are about the thing\n    they meant rather than the one they took back.");
    line = _the_shape_the_swap_had(&[memory.clone(), line.clone(), old.clone(), thing.clone()]);
    record = py_call(&pyrt::xrefs::xget("memory", "current"), &[memory.clone()]);
    if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&record.clone(), &py_str("text"), line.clone());
    let _ = py_setitem(&record.clone(), &py_str("words"), pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()]));
    let _ = py_setitem(&record.clone(), &py_str("corrected"), thing.clone());
    topics = mcall(&memory.clone(), "get", &[py_str("topics")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[topics.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = topics.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&geti(&topics.clone(), &op_neg(&Py::Int(1))), &record.clone())) } } } }) {
    let _ = py_setitem(&topics.clone(), &op_neg(&Py::Int(1)), record.clone());
    }
    }
    known = mcall(&memory.clone(), "get", &[py_str("user")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plan"), py_str("plans")]));
    for __item in __iter1 {
    key = __item.clone();
    held = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    moved = _swapped(&[held.clone(), old.clone(), thing.clone()]);
    if py_truthy(&moved.clone()) {
    let _ = py_setitem(&known.clone(), &key.clone(), moved.clone());
    }
    } else {
    if py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&known.clone(), &key.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut one: Py = __item.clone(); __out.push({ let __b = _swapped(&[call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), old.clone(), thing.clone()]); if py_truthy(&__b) { __b } else { call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)) } }); } py_list(__out) })() });
    }
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(6), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("plan"), py_str("plans")]), &mcall(&entry.clone(), "get", &[py_str("key")]))) } }) {
    continue;
    }
    value = mcall(&entry.clone(), "get", &[py_str("value")]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    moved = _swapped(&[value.clone(), old.clone(), thing.clone()]);
    if py_truthy(&moved.clone()) {
    let _ = py_setitem(&entry.clone(), &py_str("value"), moved.clone());
    }
    }
    }
    Py::None
}
pub fn _the_paragraph_already_given(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut old: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut earlier: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when this answer is the one the chat has just read, with the thing's word changed.\n\n    `the washing machine is broken` / `no the dishwasher` gets the dishwasher card, and the store's\n    two cards for a machine that stops dead are the same paragraph with the machine's own word in\n    it. Read out, it is the same answer twice in two turns, which is what the rest of this module\n    spends its time preventing; what the person asked for was the swap, not the paragraph again.\n    ");
    said = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25))) {
    return Py::Bool(false);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    earlier = call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&earlier.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20))) {
    continue;
    }
    other = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&earlier.clone(), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&other.clone()))) {
    continue;
    }
    shared = op_truediv(&call_builtin("len", &[op_bitand(&said.clone(), &other.clone())]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("float", &[call_builtin("len", &[op_bitor(&said.clone(), &other.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&op_lt(&shared.clone(), &Py::Float(0.8))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = pyrt::xrefs::xcall("memory.topics", "keywords", &[thing.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.topics", "keywords", &[old.clone()]) } }))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_names(&[said.clone(), pyrt::xrefs::xcall("memory.topics", "keywords", &[thing.clone()])]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[other.clone(), pyrt::xrefs::xcall("memory.topics", "keywords", &[old.clone()])]))) } }) {
    continue;
    }
    if py_truthy(&_names(&[said.clone(), pyrt::xrefs::xcall("memory.topics", "keywords", &[old.clone()])])) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
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
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a set of words of an answer holds one of the words, plural or possessive.\n\n    \"the dishwasher\" is in a card that says \"most dishwashers that will not start\", and the swap\n    is only worth standing down when the answer itself is about the thing they meant.\n    ");
    let __iter1 = py_iter_or_raise(&wanted.clone());
    for __item in __iter1 {
    one = __item.clone();
    one = mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&one.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &one.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&word.clone(), &op_add(&one.clone(), &py_str("s")))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&word.clone(), "rstrip", &[py_str("s")]), &one.clone())) } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _without_the_repeat_lead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The answer itself, without a lead-in that already names the thing they meant.");
    said = mcall(&g_REPEAT_LEAD(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    return if py_truthy(&op_ge(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40))) { said.clone() } else { answer.clone() };
    Py::None
}
pub fn _still_the_old_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut opening: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the store's own answer is still about the thing they took back.\n\n    Read with the chat, \"sorry i meant a turkey\" is widened back to the chicken, and the answer\n    that comes back is a chicken curry - the thing they just replaced. An answer that opens on the\n    thing they meant is not this: \"The same goes for the front room: before paint you need ...\"\n    names the room, and only then talks about the paint both rooms share.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&old.clone()))) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = pyrt::xrefs::xcall("memory.topics", "keywords", &[old.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    opening = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&said.clone(), "split", &[]), &Py::None, &Py::Int(12), &Py::None)])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = pyrt::xrefs::xcall("memory.topics", "keywords", &[thing.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&opening.clone(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _bare(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The thing without the word in front of it, for a sentence that has one of its own.");
    words = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .,")]), "split", &[]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their")]), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    return mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
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
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut opening: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when this answer is one the chat has already read, word for word.\n\n    The same ask about a thing of the same kind often reaches the same entry: \"i need to pick a\n    paint colour for the hall\" / \"no i meant the front room\" is the paint advice again, because\n    the advice is about paint and not about which room. Read out a second time with a nod in\n    front it is not a new answer, and the arbiter passes it over as an echo, which leaves the\n    turn to whatever else scored on the line. Said as the same advice for the thing they meant,\n    it is the answer, and it keeps the full paragraph for the follow-up that asks for it.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&answer.clone(), "lower", &[]), "split", &[])]), "split", &[]), &Py::None, &Py::Int(10), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &opening.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_same_words(args: &[Py]) -> Py {
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
    let mut stemmed: Py = Py::None;
    let _ = py_str("Whether two sentences are the same advice once their figures are taken out of them.\n\n    A figure takes its unit with it when it moves: \"a 2kg bird takes roughly 1 hour 40 minutes\"\n    and \"a 3kg bird takes roughly 2 hours 20 minutes\" are the same sentence with the arithmetic\n    done again, and the singular turning plural is part of the arithmetic, not a new sentence.\n    ");
pub fn __nested_stemmed_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut one: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut other: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut stemmed: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut sentence: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "sentence").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    words = mcall(&mcall(&g_FIGURE_OR_WORD(), "sub", &[py_str("#"), mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), "split", &[]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("s")]) } }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { word.clone() }); } py_list(__out) })() };
    Py::None
}
    stemmed = {{ let one = one.clone(); let other = other.clone(); let stemmed = stemmed.clone(); py_func("stemmed", move |args: &[Py]| { let mut full = vec![one.clone(), other.clone(), stemmed.clone()]; full.extend_from_slice(args); __nested_stemmed_0(&full) }) }};
    return Py::Bool(py_eq(&py_call(&stemmed, &[one.clone()]), &py_call(&stemmed, &[other.clone()])));
    Py::None
}
pub fn _the_figure_in(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The figure at the end of the line a correction is about, and the unit said with it.\n\n    \"im doing a roast for 6\" -> the 6, with no unit of its own; \"i need 2 kilos of potatoes\" ->\n    the 2, in kilos. The last figure of the line is the one a person corrects, because it is the\n    one they have just said.\n    ");
    found = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_A_FIGURE(), "finditer", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    match_ = __item.clone();
    unit = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    found = py_tuple(vec![match_.clone(), if py_truthy(&Py::Bool(py_contains(&g_FIGURE_UNITS(), &unit.clone()))) { unit.clone() } else { py_str("") }]);
    }
    return found.clone();
    Py::None
}
pub fn _the_figure_they_meant(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut held_unit: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut redone: Py = Py::None;
    let mut said: Py = Py::None;
    let mut scratch: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The answer worked out again for the figure they put right, or \"\".\n\n    \"im doing a roast for 6\" is answered with the joint to buy and the time in the oven for six,\n    and \"sorry i meant 8\" was met with \"Thanks for telling me - I'm here if you want to dig into\n    any of that.\": the number the whole answer was worked out from had changed, and nothing was\n    worked out again.\n\n    So the line is said again with the new figure in it and read as the ask it is, and what comes\n    back is the part of the answer that moved - the joint, the time - rather than the paragraph\n    the person has just read. That is only the right turn when the change really is only a\n    figure: both answers have to be word for word the same once the figures are taken out of\n    them. A correction that reaches a different answer altogether is a different question, and\n    the rest of the bot answers it.\n    ");
    match_ = mcall(&g_A_FIGURE_REPAIR(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    figure = mcall(&match_.clone(), "group", &[py_str("figure")]);
    unit = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&{ let __b = unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FIGURE_UNITS(), &unit.clone())) } }) {
    return py_str("");
    }
    before = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return py_str("");
    }
    found = _the_figure_in(&[before.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    held = __unpk1[0].clone();
    held_unit = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&held.clone(), "group", &[py_str("figure")]), &figure.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&unit.clone(), &held_unit.clone())) } }) {
    return py_str("");
    }
    line = op_add(&op_add(&slice_or_raise(&before.clone(), &Py::None, &mcall(&held.clone(), "start", &[py_str("figure")]), &Py::None), &figure.clone()), &slice_or_raise(&before.clone(), &mcall(&held.clone(), "end", &[py_str("figure")]), &Py::None, &Py::None));
    said = { let __b = pyrt::xrefs::xcall("memory.topics", "answer_in_hand", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    said = if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_answer_says(&[said.clone(), mcall(&held.clone(), "group", &[py_str("figure")])]))) } }) {
    return py_str("");
    }
    scratch = call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    redone = _the_answer(&[line.clone(), scratch.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&redone.clone()))) {
    return py_str("");
    }
    changed = _the_figures_that_moved(&[said.clone(), redone.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&changed.clone()))) {
    return py_str("");
    }
    let _ = _keep_the_corrected_answer(&[memory.clone(), scratch.clone(), line.clone()]);
    return changed.clone();
    Py::None
}
pub fn _the_answer_says(args: &[Py]) -> Py {
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
    let mut figure: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut spelling: Py = Py::None;
    let _ = py_str("Whether the answer on the screen was worked out from this figure, in figures or words.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![call_builtin("str", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e)), mcall(&g_THE_WORD_FOR(), "get", &[call_builtin("str", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e)), py_str("")])]));
    for __item in __iter1 {
    spelling = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&spelling.clone()))) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![\\d.])"), &stdlib_call("re", "escape", &[spelling.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![\\d])")), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_figures_that_moved(args: &[Py]) -> Py {
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
    let mut redone: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "redone") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: redone"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut new: Py = Py::None;
    let mut old: Py = Py::None;
    let mut one: Py = Py::None;
    let _ = py_str("The sentences of the answer worked out again that carry a figure the first one did not.\n\n    Nothing comes back unless every sentence of the two is word for word the same once the\n    figures are out of them: then the advice has not changed and only the arithmetic has, so the\n    sentences whose arithmetic moved are the whole of what the person needs.\n    ");
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE_END(), "split", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE_END(), "split", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = redone.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    changed = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("zip", &[before.clone(), after.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    old = __unpk2[0].clone();
    new = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_the_same_words(&[old.clone(), new.clone()])))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_eq(&old.clone(), &new.clone()))) {
    let _ = mcall(&changed.clone(), "append", &[new.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&changed.clone()))) {
    return py_str("");
    }
    said = mcall(&mcall(&py_str(" "), "join", &[changed.clone()]), "strip", &[]);
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _keep_the_corrected_answer(args: &[Py]) -> Py {
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
    let mut scratch: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "scratch") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: scratch"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut key: Py = Py::None;
    let mut record: Py = Py::None;
    let _ = py_str("The answers' note and the topic are the line with the right figure in it.\n\n    The reply below is part of a long answer, and the turn after it asks for another part of the\n    same one: \"and how long does that take\" after the joint and the time for eight has to reach\n    the eight, not the six the entry was first asked about.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("entry_answer"), py_str("entry_answers")]));
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&scratch.clone(), &key.clone()))) {
    let _ = py_setitem(&memory.clone(), &key.clone(), geti(&scratch.clone(), &key.clone()));
    }
    }
    record = py_call(&pyrt::xrefs::xget("memory", "current"), &[memory.clone()]);
    if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&record.clone(), &py_str("text"), line.clone());
    let _ = py_setitem(&record.clone(), &py_str("words"), pyrt::xrefs::xcall("memory.topics", "keywords", &[line.clone()]));
    }
    Py::None
}
pub fn _a_fact_of_the_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut age: Py = Py::None;
    let mut allergies: Py = Py::None;
    let _ = py_str("The first fact the store holds about somebody, as a person would say it: \"84\".");
    age = mcall(&entry.clone(), "get", &[py_str("age")]);
    if py_truthy(&age.clone()) {
    return call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    allergies = mcall(&entry.clone(), "get", &[py_str("allergies")]);
    if py_truthy(&allergies.clone()) {
    return op_add(&py_str("allergic to "), &_a_list_of(&[allergies.clone()]));
    }
    return py_str("");
    Py::None
}
pub fn _a_list_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut things: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "things") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: things"))) };
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("`nuts` -> \"nuts\", `nuts` and `dairy` -> \"nuts and dairy\".");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&said.clone(), &Py::Int(0));
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&said.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn _the_fact_moved_to_the_person_they_meant(args: &[Py]) -> Py {
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
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut entry: Py = Py::None;
    let mut fact: Py = Py::None;
    let mut key: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut others: Py = Py::None;
    let mut remember_other: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The facts filed under the person they took back, moved onto the person they meant.\n\n    A correction of who says the words were wrong and the person is one: there is no gran to keep\n    an 84 for. The entry is put back through the store's own merge, so a name on it and a person\n    already held under the new word are read the same way as any other fact.\n    ");
    others = py_func("others", |args: &[Py]| pyrt::xrefs::xcall("memory", "others", args));
    remember_other = py_func("remember_other", |args: &[Py]| pyrt::xrefs::xcall("memory", "remember_other", args));
    wanted = pyrt::xrefs::xcall("memory.facts", "a_person_word", &[_bare(&[thing.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wanted.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&wanted.clone(), &pyrt::xrefs::xcall("memory.facts", "a_person_word", &[old.clone()]))) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("list", &[py_call(&others, &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&pyrt::xrefs::xcall("memory.facts", "a_person_word", &[mcall(&entry.clone(), "get", &[py_str("who")])]), &pyrt::xrefs::xcall("memory.facts", "a_person_word", &[old.clone()])))) {
    continue;
    }
    fact = _a_fact_of_the_person(&[entry.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&fact.clone()))) {
    return py_str("");
    }
    let _ = mcall(&py_call(&others, &[memory.clone()]), "remove", &[entry.clone()]);
    moved = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&entry.clone(), "items", &[])) { let __unpk9015 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk9015[0].clone(); let mut value: Py = __unpk9015[1].clone(); if py_truthy(&Py::Bool(!py_eq(&key.clone(), &py_str("who")))) {  __out.push((key.clone(), value.clone())); } } py_dict(__out) })() };
    let _ = py_setitem(&moved.clone(), &py_str("who"), wanted.clone());
    let _ = py_call(&remember_other, &[memory.clone(), moved.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    return fact.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_fact_moved_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fact: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fact") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fact"))) };
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The reply to a correction of who, when the fact is the only thing worth saying.");
    if py_truthy(&Py::Bool(!py_truthy(&fact.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("new".to_string(), _bare(&[thing.clone()])), ("old".to_string(), _bare(&[old.clone()])), ("fact".to_string(), fact.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_THE_FACT_MOVED(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]), "format", args))(&__args) };
    return if py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { said.clone() } else { op_add(&said.clone(), &py_str(".")) };
    Py::None
}
pub fn _the_swap_in_the_line(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut old: Py = Py::None;
    let mut olds: Py = Py::None;
    let mut one: Py = Py::None;
    let mut refined: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("What the line swaps: (the last line, the words that could be the old thing, the new thing).\n\n    None when the line is no correction of a thing, or when there is no earlier line to correct.\n    The old thing is tried in order: the person the last line named, then the thing its reading\n    names - the thing and not the state it is in (\"the washing machine is broken\" names the\n    machine, not the broken) - then the word at the end of the line, which is what a row keyed on\n    the whole sentence was reached by.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    thing = _the_thing(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_AN_ASK_OPENS(), "match", &[thing.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&thing.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    return Py::None;
    }
    last = call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::None;
    }
    refined = _the_pointer_refines(&[thing.clone(), last.clone()]);
    if py_truthy(&refined.clone()) {
    let __unpk1 = py_unpack(&refined.clone(), 2, None);
    old = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    return py_tuple(vec![last.clone(), py_list(vec![old.clone()]), thing.clone(), refined.clone()]);
    }
    old = _the_person_the_line_named(&[last.clone(), thing.clone()]);
    olds = if py_truthy(&old.clone()) { py_list(vec![old.clone()]) } else { py_list(vec![]) };
    let __iter2 = py_iter_or_raise(&py_tuple(vec![_the_thing_the_reading_named(&[last.clone()]), _the_thing_the_last_line_named(&[last.clone()])]));
    for __item in __iter2 {
    one = __item.clone();
    if py_truthy(&{ let __b = one.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&olds.clone(), &one.clone())) } }) {
    let _ = mcall(&olds.clone(), "append", &[one.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&olds.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![last.clone(), olds.clone(), thing.clone(), Py::None]);
    Py::None
}
pub fn keep_the_correction(args: &[Py]) -> Py {
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
    let mut _refined: Py = Py::None;
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut olds: Py = Py::None;
    let mut one: Py = Py::None;
    let mut swapped: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("Keep the swap the line made, for a correction another reader answered.\n\n    \"how long does a chicken take to roast\" then \"i meant a turkey\" reads right either way - the\n    turkey row answers the correction itself - but the chat has still been corrected, and the line\n    after it has to be read against the turkey: asked \"what about a duck\" the chat was still on\n    the chicken, so the duck row was never reached and the person got a search hedge. Called by\n    the store whatever gave the reply, so the bookkeeping does not depend on which reader won the\n    turn.\n    ");
    found = _the_swap_in_the_line(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 4, None);
    last = __unpk1[0].clone();
    olds = __unpk1[1].clone();
    thing = __unpk1[2].clone();
    _refined = __unpk1[3].clone();
    let __iter2 = py_iter_or_raise(&olds.clone());
    for __item in __iter2 {
    one = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_a_different_thing(&[one.clone(), thing.clone()])))) {
    continue;
    }
    swapped = _swapped(&[last.clone(), one.clone(), thing.clone()]);
    if py_truthy(&swapped.clone()) {
    let _ = _keep_the_swap(&[memory.clone(), swapped.clone(), one.clone(), thing.clone()]);
    return Py::None;
    }
    }
    Py::None
}
pub fn the_thing_they_meant(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut fact: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut old: Py = Py::None;
    let mut olds: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut refined: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut swapped: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The answer about the thing they said they meant, or None.\n\n    Called when the rest of the store has nothing to say about the line, so a value correction\n    and a fact to move have already had their turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    figure = _the_figure_they_meant(&[text.clone(), memory.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    thing = _the_thing(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_AN_ASK_OPENS(), "match", &[thing.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&thing.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    answer = _the_answer(&[thing.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    answer = _without_the_repeat_lead(&[answer.clone()]);
    if py_truthy(&_said_before(&[answer.clone(), memory.clone()])) {
    return Py::None;
    }
    return if py_truthy(&mcall(&answer.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { answer.clone() } else { op_add(&answer.clone(), &py_str(".")) };
    }
    let __unpk1 = py_unpack(&_the_swap_in_the_line(&[text.clone(), memory.clone()]), 4, None);
    last = __unpk1[0].clone();
    olds = __unpk1[1].clone();
    thing = __unpk1[2].clone();
    refined = __unpk1[3].clone();
    held = geti(&olds.clone(), &Py::Int(0));
    fact = _the_fact_moved_to_the_person_they_meant(&[memory.clone(), held.clone(), thing.clone(), text.clone()]);
    let __unpk2 = py_unpack(&py_tuple(vec![py_str(""), py_str(""), py_str("")]), 3, None);
    answer = __unpk2[0].clone();
    line = __unpk2[1].clone();
    old = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&olds.clone());
    for __item in __iter3 {
    one = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_a_different_thing(&[one.clone(), thing.clone()])))) {
    continue;
    }
    swapped = _swapped(&[last.clone(), one.clone(), thing.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&swapped.clone()))) {
    continue;
    }
    found = _the_answer(&[swapped.clone(), memory.clone()]);
    if py_truthy(&found.clone()) {
    let __unpk4 = py_unpack(&py_tuple(vec![found.clone(), swapped.clone(), one.clone()]), 3, None);
    answer = __unpk4[0].clone();
    line = __unpk4[1].clone();
    old = __unpk4[2].clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return _the_fact_moved_line(&[fact.clone(), held.clone(), thing.clone(), text.clone(), memory.clone()]);
    }
    answer = _without_the_repeat_lead(&[answer.clone()]);
    plain = _the_plain_answer(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _about_the_same_thing(&[plain.clone(), thing.clone()]); if py_truthy(&__b) { __b } else { _about_the_same_thing(&[plain.clone(), answer.clone()]) } })) } }) {
    plain = Py::None;
    }
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_without_the_repeat_lead(&[plain.clone()]), &answer.clone())) } }) {
    let _ = _keep_the_swap(&[memory.clone(), line.clone(), old.clone(), thing.clone()]);
    return Py::None;
    }
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_REAL_ANSWER()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_HEDGE(), "search", &[plain.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_still_the_old_thing(&[plain.clone(), old.clone(), thing.clone()]))) } } } } } }) {
    if py_truthy(&_said_before(&[plain.clone(), memory.clone()])) {
    return Py::None;
    }
    let _ = _keep_the_swap(&[memory.clone(), line.clone(), old.clone(), thing.clone()]);
    return if py_truthy(&mcall(&plain.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { plain.clone() } else { op_add(&plain.clone(), &py_str(".")) };
    }
    if py_truthy(&Py::Bool(py_eq(&plain.clone(), &answer.clone()))) {
    let _ = _keep_the_swap(&[memory.clone(), line.clone(), old.clone(), thing.clone()]);
    return Py::None;
    }
    let _ = _keep_the_swap(&[memory.clone(), line.clone(), old.clone(), thing.clone()]);
    if py_truthy(&{ let __b = refined.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_before(&[answer.clone(), memory.clone()]))) } }) {
    return if py_truthy(&mcall(&answer.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { answer.clone() } else { op_add(&answer.clone(), &py_str(".")) };
    }
    shown = op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&_the_paragraph_already_given(&[answer.clone(), memory.clone(), old.clone(), thing.clone()])) {
    held = _bare(&[thing.clone()]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her")]), &mcall(&geti(&mcall(&shown.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    held = op_add(&mcall(&slice_or_raise(&shown.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&shown.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), shown.clone()), ("thing".to_string(), held.clone()), ("old".to_string(), _bare(&[old.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_SAME_ADVICE(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]), "format", args))(&__args) };
    return if py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { said.clone() } else { op_add(&said.clone(), &py_str(".")) };
    }
    if py_truthy(&_said_before(&[answer.clone(), memory.clone()])) {
    held = _bare(&[thing.clone()]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her")]), &mcall(&geti(&mcall(&shown.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    held = op_add(&mcall(&slice_or_raise(&shown.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&shown.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), shown.clone()), ("thing".to_string(), held.clone()), ("old".to_string(), _bare(&[old.clone()])), ("answer".to_string(), op_add(&mcall(&slice_or_raise(&answer.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&answer.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_SAME_AGAIN(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]), "format", args))(&__args) };
    return if py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { said.clone() } else { op_add(&said.clone(), &py_str(".")) };
    }
    shape = pyrt::xrefs::xcall("compose", "pick", &[g_SWAPPED(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&shape.clone(), "startswith", &[py_str("{thing}")])))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her")]), &mcall(&geti(&mcall(&shown.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    shown = op_add(&mcall(&slice_or_raise(&shown.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&shown.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), shown.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&shape.clone(), "format", args))(&__args) };
    return mcall(&op_add(&op_add(&said.clone(), &py_str(" ")), &answer.clone()), "strip", &[]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.repair".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ASK" => Some(g_AN_ASK()),
            "AN_ASK_OPENS" => Some(g_AN_ASK_OPENS()),
            "AN_INTENTION" => Some(g_AN_INTENTION()),
            "A_CONTRAST" => Some(g_A_CONTRAST()),
            "A_DETERMINER" => Some(g_A_DETERMINER()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_FIGURE_REPAIR" => Some(g_A_FIGURE_REPAIR()),
            "A_POINTER" => Some(g_A_POINTER()),
            "A_POSSESSION_REPAIR" => Some(g_A_POSSESSION_REPAIR()),
            "A_REAL_ANSWER" => Some(g_A_REAL_ANSWER()),
            "A_REMARK" => Some(g_A_REMARK()),
            "A_VALUE" => Some(g_A_VALUE()),
            "A_VERB_IN_THE_THING" => Some(g_A_VERB_IN_THE_THING()),
            "BARE_REPAIR" => Some(g_BARE_REPAIR()),
            "FIGURE_OR_WORD" => Some(g_FIGURE_OR_WORD()),
            "FIGURE_UNITS" => Some(g_FIGURE_UNITS()),
            "HEDGE" => Some(g_HEDGE()),
            "NOT_A_SWAP" => Some(g_NOT_A_SWAP()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_THE_THING" => Some(g_NOT_THE_THING()),
            "REPAIR" => Some(g_REPAIR()),
            "REPEAT_LEAD" => Some(g_REPEAT_LEAD()),
            "SAME_ADVICE" => Some(g_SAME_ADVICE()),
            "SAME_AGAIN" => Some(g_SAME_AGAIN()),
            "SENTENCE_END" => Some(g_SENTENCE_END()),
            "SWAPPED" => Some(g_SWAPPED()),
            "THE_FACT_MOVED" => Some(g_THE_FACT_MOVED()),
            "THE_WORD_FOR" => Some(g_THE_WORD_FOR()),
            "_KEEP" => Some(g__KEEP()),
            "_READINGS" => Some(g__READINGS()),
            "_a_different_thing" => Some(py_func("_a_different_thing", |args: &[Py]| _a_different_thing(args))),
            "_a_fact_of_the_person" => Some(py_func("_a_fact_of_the_person", |args: &[Py]| _a_fact_of_the_person(args))),
            "_a_list_of" => Some(py_func("_a_list_of", |args: &[Py]| _a_list_of(args))),
            "_a_state_not_a_thing" => Some(py_func("_a_state_not_a_thing", |args: &[Py]| _a_state_not_a_thing(args))),
            "_about_the_same_thing" => Some(py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args))),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_content_words" => Some(py_func("_content_words", |args: &[Py]| _content_words(args))),
            "_keep_the_corrected_answer" => Some(py_func("_keep_the_corrected_answer", |args: &[Py]| _keep_the_corrected_answer(args))),
            "_keep_the_swap" => Some(py_func("_keep_the_swap", |args: &[Py]| _keep_the_swap(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_no_repeat" => Some(py_func("_no_repeat", |args: &[Py]| _no_repeat(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "_still_the_old_thing" => Some(py_func("_still_the_old_thing", |args: &[Py]| _still_the_old_thing(args))),
            "_swapped" => Some(py_func("_swapped", |args: &[Py]| _swapped(args))),
            "_the_answer" => Some(py_func("_the_answer", |args: &[Py]| _the_answer(args))),
            "_the_answer_says" => Some(py_func("_the_answer_says", |args: &[Py]| _the_answer_says(args))),
            "_the_fact_moved_line" => Some(py_func("_the_fact_moved_line", |args: &[Py]| _the_fact_moved_line(args))),
            "_the_fact_moved_to_the_person_they_meant" => Some(py_func("_the_fact_moved_to_the_person_they_meant", |args: &[Py]| _the_fact_moved_to_the_person_they_meant(args))),
            "_the_figure_in" => Some(py_func("_the_figure_in", |args: &[Py]| _the_figure_in(args))),
            "_the_figure_they_meant" => Some(py_func("_the_figure_they_meant", |args: &[Py]| _the_figure_they_meant(args))),
            "_the_figures_that_moved" => Some(py_func("_the_figures_that_moved", |args: &[Py]| _the_figures_that_moved(args))),
            "_the_noun_the_line_named" => Some(py_func("_the_noun_the_line_named", |args: &[Py]| _the_noun_the_line_named(args))),
            "_the_paragraph_already_given" => Some(py_func("_the_paragraph_already_given", |args: &[Py]| _the_paragraph_already_given(args))),
            "_the_person_the_line_named" => Some(py_func("_the_person_the_line_named", |args: &[Py]| _the_person_the_line_named(args))),
            "_the_plain_answer" => Some(py_func("_the_plain_answer", |args: &[Py]| _the_plain_answer(args))),
            "_the_pointer_refines" => Some(py_func("_the_pointer_refines", |args: &[Py]| _the_pointer_refines(args))),
            "_the_reading" => Some(py_func("_the_reading", |args: &[Py]| _the_reading(args))),
            "_the_relation" => Some(py_func("_the_relation", |args: &[Py]| _the_relation(args))),
            "_the_same_words" => Some(py_func("_the_same_words", |args: &[Py]| _the_same_words(args))),
            "_the_shape_the_swap_had" => Some(py_func("_the_shape_the_swap_had", |args: &[Py]| _the_shape_the_swap_had(args))),
            "_the_swap_in_the_line" => Some(py_func("_the_swap_in_the_line", |args: &[Py]| _the_swap_in_the_line(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_the_thing_the_last_line_named" => Some(py_func("_the_thing_the_last_line_named", |args: &[Py]| _the_thing_the_last_line_named(args))),
            "_the_thing_the_reading_named" => Some(py_func("_the_thing_the_reading_named", |args: &[Py]| _the_thing_the_reading_named(args))),
            "_without_the_repeat_lead" => Some(py_func("_without_the_repeat_lead", |args: &[Py]| _without_the_repeat_lead(args))),
            "answer_in_hand" => Some(g_answer_in_hand()),
            "current" => Some(g_current()),
            "keep_the_correction" => Some(py_func("keep_the_correction", |args: &[Py]| keep_the_correction(args))),
            "keywords" => Some(g_keywords()),
            "last_user" => Some(g_last_user()),
            "pick" => Some(g_pick()),
            "the_thing_they_meant" => Some(py_func("the_thing_they_meant", |args: &[Py]| the_thing_they_meant(args))),
            "extract" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_ASK" => s_AN_ASK(v),
            "AN_ASK_OPENS" => s_AN_ASK_OPENS(v),
            "AN_INTENTION" => s_AN_INTENTION(v),
            "A_CONTRAST" => s_A_CONTRAST(v),
            "A_DETERMINER" => s_A_DETERMINER(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_FIGURE_REPAIR" => s_A_FIGURE_REPAIR(v),
            "A_POINTER" => s_A_POINTER(v),
            "A_POSSESSION_REPAIR" => s_A_POSSESSION_REPAIR(v),
            "A_REAL_ANSWER" => s_A_REAL_ANSWER(v),
            "A_REMARK" => s_A_REMARK(v),
            "A_VALUE" => s_A_VALUE(v),
            "A_VERB_IN_THE_THING" => s_A_VERB_IN_THE_THING(v),
            "BARE_REPAIR" => s_BARE_REPAIR(v),
            "FIGURE_OR_WORD" => s_FIGURE_OR_WORD(v),
            "FIGURE_UNITS" => s_FIGURE_UNITS(v),
            "HEDGE" => s_HEDGE(v),
            "NOT_A_SWAP" => s_NOT_A_SWAP(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_THE_THING" => s_NOT_THE_THING(v),
            "REPAIR" => s_REPAIR(v),
            "REPEAT_LEAD" => s_REPEAT_LEAD(v),
            "SAME_ADVICE" => s_SAME_ADVICE(v),
            "SAME_AGAIN" => s_SAME_AGAIN(v),
            "SENTENCE_END" => s_SENTENCE_END(v),
            "SWAPPED" => s_SWAPPED(v),
            "THE_FACT_MOVED" => s_THE_FACT_MOVED(v),
            "THE_WORD_FOR" => s_THE_WORD_FOR(v),
            "_KEEP" => s__KEEP(v),
            "_READINGS" => s__READINGS(v),
            "answer_in_hand" => s_answer_in_hand(v),
            "current" => s_current(v),
            "keywords" => s_keywords(v),
            "last_user" => s_last_user(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.repair", module_obj);
    pyrt::xrefs::reg_fn("memory.repair", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_a_different_thing", py_func("_a_different_thing", |args: &[Py]| _a_different_thing(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_content_words", py_func("_content_words", |args: &[Py]| _content_words(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_reading", py_func("_the_reading", |args: &[Py]| _the_reading(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_thing_the_reading_named", py_func("_the_thing_the_reading_named", |args: &[Py]| _the_thing_the_reading_named(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_thing_the_last_line_named", py_func("_the_thing_the_last_line_named", |args: &[Py]| _the_thing_the_last_line_named(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_noun_the_line_named", py_func("_the_noun_the_line_named", |args: &[Py]| _the_noun_the_line_named(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_pointer_refines", py_func("_the_pointer_refines", |args: &[Py]| _the_pointer_refines(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_relation", py_func("_the_relation", |args: &[Py]| _the_relation(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_person_the_line_named", py_func("_the_person_the_line_named", |args: &[Py]| _the_person_the_line_named(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_swapped", py_func("_swapped", |args: &[Py]| _swapped(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_no_repeat", py_func("_no_repeat", |args: &[Py]| _no_repeat(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_answer", py_func("_the_answer", |args: &[Py]| _the_answer(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_plain_answer", py_func("_the_plain_answer", |args: &[Py]| _the_plain_answer(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_about_the_same_thing", py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_shape_the_swap_had", py_func("_the_shape_the_swap_had", |args: &[Py]| _the_shape_the_swap_had(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_a_state_not_a_thing", py_func("_a_state_not_a_thing", |args: &[Py]| _a_state_not_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_keep_the_swap", py_func("_keep_the_swap", |args: &[Py]| _keep_the_swap(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_paragraph_already_given", py_func("_the_paragraph_already_given", |args: &[Py]| _the_paragraph_already_given(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_names", py_func("_names", |args: &[Py]| _names(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_without_the_repeat_lead", py_func("_without_the_repeat_lead", |args: &[Py]| _without_the_repeat_lead(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_still_the_old_thing", py_func("_still_the_old_thing", |args: &[Py]| _still_the_old_thing(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_said_before", py_func("_said_before", |args: &[Py]| _said_before(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_same_words", py_func("_the_same_words", |args: &[Py]| _the_same_words(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_figure_in", py_func("_the_figure_in", |args: &[Py]| _the_figure_in(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_figure_they_meant", py_func("_the_figure_they_meant", |args: &[Py]| _the_figure_they_meant(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_answer_says", py_func("_the_answer_says", |args: &[Py]| _the_answer_says(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_figures_that_moved", py_func("_the_figures_that_moved", |args: &[Py]| _the_figures_that_moved(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_keep_the_corrected_answer", py_func("_keep_the_corrected_answer", |args: &[Py]| _keep_the_corrected_answer(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_a_fact_of_the_person", py_func("_a_fact_of_the_person", |args: &[Py]| _a_fact_of_the_person(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_a_list_of", py_func("_a_list_of", |args: &[Py]| _a_list_of(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_fact_moved_to_the_person_they_meant", py_func("_the_fact_moved_to_the_person_they_meant", |args: &[Py]| _the_fact_moved_to_the_person_they_meant(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_fact_moved_line", py_func("_the_fact_moved_line", |args: &[Py]| _the_fact_moved_line(args)));
    pyrt::xrefs::reg_fn("memory.repair", "_the_swap_in_the_line", py_func("_the_swap_in_the_line", |args: &[Py]| _the_swap_in_the_line(args)));
    pyrt::xrefs::reg_fn("memory.repair", "keep_the_correction", py_func("keep_the_correction", |args: &[Py]| keep_the_correction(args)));
    pyrt::xrefs::reg_fn("memory.repair", "the_thing_they_meant", py_func("the_thing_they_meant", |args: &[Py]| the_thing_they_meant(args)));
    pyrt::xrefs::reg_access("memory.repair", "AN_ASK", g_AN_ASK, s_AN_ASK);
    pyrt::xrefs::reg_access("memory.repair", "AN_ASK_OPENS", g_AN_ASK_OPENS, s_AN_ASK_OPENS);
    pyrt::xrefs::reg_access("memory.repair", "AN_INTENTION", g_AN_INTENTION, s_AN_INTENTION);
    pyrt::xrefs::reg_access("memory.repair", "A_CONTRAST", g_A_CONTRAST, s_A_CONTRAST);
    pyrt::xrefs::reg_access("memory.repair", "A_DETERMINER", g_A_DETERMINER, s_A_DETERMINER);
    pyrt::xrefs::reg_access("memory.repair", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("memory.repair", "A_FIGURE_REPAIR", g_A_FIGURE_REPAIR, s_A_FIGURE_REPAIR);
    pyrt::xrefs::reg_access("memory.repair", "A_POINTER", g_A_POINTER, s_A_POINTER);
    pyrt::xrefs::reg_access("memory.repair", "A_POSSESSION_REPAIR", g_A_POSSESSION_REPAIR, s_A_POSSESSION_REPAIR);
    pyrt::xrefs::reg_access("memory.repair", "A_REAL_ANSWER", g_A_REAL_ANSWER, s_A_REAL_ANSWER);
    pyrt::xrefs::reg_access("memory.repair", "A_REMARK", g_A_REMARK, s_A_REMARK);
    pyrt::xrefs::reg_access("memory.repair", "A_VALUE", g_A_VALUE, s_A_VALUE);
    pyrt::xrefs::reg_access("memory.repair", "A_VERB_IN_THE_THING", g_A_VERB_IN_THE_THING, s_A_VERB_IN_THE_THING);
    pyrt::xrefs::reg_access("memory.repair", "BARE_REPAIR", g_BARE_REPAIR, s_BARE_REPAIR);
    pyrt::xrefs::reg_access("memory.repair", "FIGURE_OR_WORD", g_FIGURE_OR_WORD, s_FIGURE_OR_WORD);
    pyrt::xrefs::reg_access("memory.repair", "FIGURE_UNITS", g_FIGURE_UNITS, s_FIGURE_UNITS);
    pyrt::xrefs::reg_access("memory.repair", "HEDGE", g_HEDGE, s_HEDGE);
    pyrt::xrefs::reg_access("memory.repair", "NOT_A_SWAP", g_NOT_A_SWAP, s_NOT_A_SWAP);
    pyrt::xrefs::reg_access("memory.repair", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.repair", "NOT_THE_THING", g_NOT_THE_THING, s_NOT_THE_THING);
    pyrt::xrefs::reg_access("memory.repair", "REPAIR", g_REPAIR, s_REPAIR);
    pyrt::xrefs::reg_access("memory.repair", "REPEAT_LEAD", g_REPEAT_LEAD, s_REPEAT_LEAD);
    pyrt::xrefs::reg_access("memory.repair", "SAME_ADVICE", g_SAME_ADVICE, s_SAME_ADVICE);
    pyrt::xrefs::reg_access("memory.repair", "SAME_AGAIN", g_SAME_AGAIN, s_SAME_AGAIN);
    pyrt::xrefs::reg_access("memory.repair", "SENTENCE_END", g_SENTENCE_END, s_SENTENCE_END);
    pyrt::xrefs::reg_access("memory.repair", "SWAPPED", g_SWAPPED, s_SWAPPED);
    pyrt::xrefs::reg_access("memory.repair", "THE_FACT_MOVED", g_THE_FACT_MOVED, s_THE_FACT_MOVED);
    pyrt::xrefs::reg_access("memory.repair", "THE_WORD_FOR", g_THE_WORD_FOR, s_THE_WORD_FOR);
    pyrt::xrefs::reg_access("memory.repair", "_KEEP", g__KEEP, s__KEEP);
    pyrt::xrefs::reg_access("memory.repair", "_READINGS", g__READINGS, s__READINGS);
    pyrt::xrefs::reg_access("memory.repair", "answer_in_hand", g_answer_in_hand, s_answer_in_hand);
    pyrt::xrefs::reg_access("memory.repair", "current", g_current, s_current);
    pyrt::xrefs::reg_access("memory.repair", "keywords", g_keywords, s_keywords);
    pyrt::xrefs::reg_access("memory.repair", "last_user", g_last_user, s_last_user);
    pyrt::xrefs::reg_access("memory.repair", "pick", g_pick, s_pick);
}
