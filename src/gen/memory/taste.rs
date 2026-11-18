#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ASKS_ABOUT_A_TASTE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_FOR_TASTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DISLIKE_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIKING_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MY_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_KEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNED_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TASTE_ASKED_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNSURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_FOR_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_SOMETHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_ADVERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_CLAUSE_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CLAUSE_SPLIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HATED_CHORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OTHER_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OTHER_OWNER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OTHER_SAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OTHER_TASTE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OTHER_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _QUANTITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SHORTFALL_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SHORTFALL_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TASTE_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASKS_ABOUT_A_TASTE() -> Py { ensure_init(); ASKS_ABOUT_A_TASTE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_A_TASTE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_ABOUT_A_TASTE.write() { *w = v; } }
pub fn g_ASKS_FOR_TASTES() -> Py { ensure_init(); ASKS_FOR_TASTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_TASTES(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_FOR_TASTES.write() { *w = v; } }
pub fn g_DISLIKE_IN() -> Py { ensure_init(); DISLIKE_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DISLIKE_IN(v: Py) { ensure_init(); if let Ok(mut w) = DISLIKE_IN.write() { *w = v; } }
pub fn g_HATE() -> Py { ensure_init(); HATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HATE(v: Py) { ensure_init(); if let Ok(mut w) = HATE.write() { *w = v; } }
pub fn g_LIKING_BEFORE() -> Py { ensure_init(); LIKING_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIKING_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = LIKING_BEFORE.write() { *w = v; } }
pub fn g_MY_THING() -> Py { ensure_init(); MY_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MY_THING(v: Py) { ensure_init(); if let Ok(mut w) = MY_THING.write() { *w = v; } }
pub fn g_NO() -> Py { ensure_init(); NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO(v: Py) { ensure_init(); if let Ok(mut w) = NO.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOT_KEEN() -> Py { ensure_init(); NOT_KEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_KEEN(v: Py) { ensure_init(); if let Ok(mut w) = NOT_KEEN.write() { *w = v; } }
pub fn g_OWNED_BEFORE() -> Py { ensure_init(); OWNED_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNED_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = OWNED_BEFORE.write() { *w = v; } }
pub fn g_TAIL() -> Py { ensure_init(); TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = TAIL.write() { *w = v; } }
pub fn g_TASTE_ASKED_ABOUT() -> Py { ensure_init(); TASTE_ASKED_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TASTE_ASKED_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = TASTE_ASKED_ABOUT.write() { *w = v; } }
pub fn g_THING() -> Py { ensure_init(); THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING(v: Py) { ensure_init(); if let Ok(mut w) = THING.write() { *w = v; } }
pub fn g_THING_STOP() -> Py { ensure_init(); THING_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_STOP(v: Py) { ensure_init(); if let Ok(mut w) = THING_STOP.write() { *w = v; } }
pub fn g_UNSURE() -> Py { ensure_init(); UNSURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNSURE(v: Py) { ensure_init(); if let Ok(mut w) = UNSURE.write() { *w = v; } }
pub fn g_YES() -> Py { ensure_init(); YES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YES(v: Py) { ensure_init(); if let Ok(mut w) = YES.write() { *w = v; } }
pub fn g__ASKS_FOR_A_THING() -> Py { ensure_init(); _ASKS_FOR_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_FOR_A_THING.write() { *w = v; } }
pub fn g__ASKS_SOMETHING() -> Py { ensure_init(); _ASKS_SOMETHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_SOMETHING(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_SOMETHING.write() { *w = v; } }
pub fn g__ASK_ADVERB() -> Py { ensure_init(); _ASK_ADVERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_ADVERB(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_ADVERB.write() { *w = v; } }
pub fn g__ASK_FORM() -> Py { ensure_init(); _ASK_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_FORM(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_FORM.write() { *w = v; } }
pub fn g__ASK_START() -> Py { ensure_init(); _ASK_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_START(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_START.write() { *w = v; } }
pub fn g__A_CLAUSE_AFTER() -> Py { ensure_init(); _A_CLAUSE_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_CLAUSE_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = _A_CLAUSE_AFTER.write() { *w = v; } }
pub fn g__CLAUSE_SPLIT() -> Py { ensure_init(); _CLAUSE_SPLIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CLAUSE_SPLIT(v: Py) { ensure_init(); if let Ok(mut w) = _CLAUSE_SPLIT.write() { *w = v; } }
pub fn g__HATED_CHORE() -> Py { ensure_init(); _HATED_CHORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HATED_CHORE(v: Py) { ensure_init(); if let Ok(mut w) = _HATED_CHORE.write() { *w = v; } }
pub fn g__OTHER_CLAUSE() -> Py { ensure_init(); _OTHER_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OTHER_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = _OTHER_CLAUSE.write() { *w = v; } }
pub fn g__OTHER_OWNER() -> Py { ensure_init(); _OTHER_OWNER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OTHER_OWNER(v: Py) { ensure_init(); if let Ok(mut w) = _OTHER_OWNER.write() { *w = v; } }
pub fn g__OTHER_SAYS() -> Py { ensure_init(); _OTHER_SAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OTHER_SAYS(v: Py) { ensure_init(); if let Ok(mut w) = _OTHER_SAYS.write() { *w = v; } }
pub fn g__OTHER_TASTE() -> Py { ensure_init(); _OTHER_TASTE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OTHER_TASTE(v: Py) { ensure_init(); if let Ok(mut w) = _OTHER_TASTE.write() { *w = v; } }
pub fn g__OTHER_VERB() -> Py { ensure_init(); _OTHER_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OTHER_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _OTHER_VERB.write() { *w = v; } }
pub fn g__QUANTITY() -> Py { ensure_init(); _QUANTITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__QUANTITY(v: Py) { ensure_init(); if let Ok(mut w) = _QUANTITY.write() { *w = v; } }
pub fn g__SHORTFALL_TAIL() -> Py { ensure_init(); _SHORTFALL_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SHORTFALL_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _SHORTFALL_TAIL.write() { *w = v; } }
pub fn g__SHORTFALL_VERB() -> Py { ensure_init(); _SHORTFALL_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SHORTFALL_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _SHORTFALL_VERB.write() { *w = v; } }
pub fn g__TASTE_VERB() -> Py { ensure_init(); _TASTE_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TASTE_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _TASTE_VERB.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A taste they have told the bot about: \"i love dogs\", \"not keen on fish\", \"not sure if i like curry\".\n\nA taste is a fact about the person, so it is said back like any other (\"You love dogs - noted\"),\nbut it is not a pet. Before this, \"i love dogs\", \"i hate cats\" and \"not keen on rabbit\" each put a\nnameless animal in the store, and the recall answered \"what pets do i have?\" with \"You've got a\ncat\" to someone who had just said they hated them. An animal is only theirs when a possessive says\nso (\"we love our dog\", \"i adore my cat\"), which is what `LIKING_BEFORE` and `OWNED_BEFORE` are for:\nthe first reads the taste, the second keeps the pet in the line that really has one.\n\n`read()` is the taste itself, for the echo in `bot/memory/ack.py`: the thing, and whether they\nlike it, do not, or are not sure. \"not sure if i like curry\" is not the same fact as \"i don't like\ncurry\", and a bot that stores the first as the second will leave curry out of every suggestion\nafter one hedge.\n");
    s_TAIL(call_builtin("frozenset", &[mcall(&py_str("\ntho though lol lmao haha hehe tbh ngl imo idk innit tbf anyhow anyway really actually\nright now today tonight tomorrow weekend week weekday evening morning afternoon night\neither at all any more anymore yet still too also as well\nand or but with so much very loads lots lot quite pretty\na an the of to for from in on at by about than then\nmonday tuesday wednesday thursday friday saturday sunday christmas birthday\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_STOP(call_builtin("frozenset", &[mcall(&py_str("\ni we me us he she they you it them that this these those there\nwhat how why when where which who whose whatever\nis are was were am be been being will would should can could shall may might\ndo does did dont doesn't doesn't isn't aren't have has had got\nso if because since though although while until\nlike loves loved love adore adores prefer prefers hate hates hated dislike dislikes\nloathe loathes detest detests enjoy enjoys\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING(py_str("([A-Za-z][A-Za-z'\\u2019-]*(?:\\s+(?:[A-Za-z][A-Za-z'\\u2019-]*|and|or|of|to|the|a|my|our|your|their|his|her)){0,4})"));
    s_UNSURE(stdlib_call("re", "compile", &[add(&py_str("\\b(?:not sure|not certain|unsure|don'?t know|do not know|dunno|no idea)\\s+(?:if|whether|that)\\s+(?:i|we)(?:'d|\\s+would)?\\s+(?:actually\\s+|really\\s+|even\\s+)?(?:like|love|enjoy|be into|get on with)\\s+"), &g_THING()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO(stdlib_call("re", "compile", &[add(&py_str("\\b(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+)?(?:don'?t|do not|dont|never)\\s+(?:think\\s+(?:i|we)\\s+|really\\s+)?(?:like|love|eat|drink|want)\\s+"), &g_THING()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_KEEN(stdlib_call("re", "compile", &[add(&py_str("(?:\\b(?:i'?m|i am|we'?re|we are)|^)\\s*not\\s+(?:(?:that|really|very|so|all that|quite)\\s+)?(?:much\\s+of\\s+)?(?:a\\s+|the\\s+)?(?:(?:big|huge|massive|great)\\s+)?(?:keen on|fan of|into|mad about|fussed on|bothered about)\\s+"), &g_THING()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HATE(stdlib_call("re", "compile", &[add(&py_str("\\b(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+|absolutely\\s+|properly\\s+)?(?:hate|hates|dislike|dislikes|loathe|loathes|detest|detests|can'?t stand|cannot stand|really can'?t stand)\\s+"), &g_THING()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MY_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b"), &g_THING()), &py_str("\\s+(?:isn'?t|is not|aren'?t|are not)\\s+(?:(?:really|that|exactly|quite)\\s+)?(?:my|our)\\s+(?:(?:kind|sort)\\s+of\\s+|cup\\s+of\\s+tea|thing)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_YES(stdlib_call("re", "compile", &[add(&py_str("\\b(?:i|we)\\s+(?:do\\s+|really\\s+|absolutely\\s+|just\\s+|actually\\s+)?(?:like|love|adore|prefer)\\s+"), &g_THING()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nit that this these those them him her you me us one ones a an the to when if how why what\nwhich who there here so such being doing having\nmyself yourself himself herself itself ourselves yourselves themselves oneself\nway\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DISLIKE_IN(stdlib_call("re", "compile", &[py_str("\\b(?:(?:really|just|actually|absolutely|properly|simply)\\s+)*(?:hate|hates|hated|dislike|dislikes|disliked|loathe|loathes|loathed|detest|detests|detested|can'?t stand|cannot stand)\\b.*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIKING_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:like|likes|liked|love|loves|loved|adore|adores|adored|hate|hates|hated|prefer|prefers|preferred|keen on|a fan of|fan of|fond of|into|can'?t stand|cant stand|crazy about|big on|dislike|dislikes|disliked|loathe|loathes|loathed|detest|detests|detested|not\\s+(?:\\w+\\s+)?(?:a fan of|keen on|into|fond of|big on))\\s+(?:\\w+\\s+){0,2}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OWNED_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|the|this|that)\\s+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__QUANTITY(stdlib_call("re", "compile", &[py_str("^(?:enough|too much|too little|much|any|more|less|loads of|lots of|plenty of|a lot of|as much)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SHORTFALL_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:at\\s+all|any\\s?more)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SHORTFALL_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:eat|drink)\\s+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TASTE_VERB(py_str("(?:(?:i|we|he|she|they)\\s+)?(?:do\\s+|really\\s+|absolutely\\s+|just\\s+|actually\\s+|properly\\s+)?(?:like|love|adore|prefer|hate|hates|dislike|dislikes|loathe|detest|enjoy|eat|drink|want|can'?t stand|cannot stand)\\b"));
    s__CLAUSE_SPLIT(stdlib_call("re", "compile", &[add(&add(&py_str("\\s*;\\s*|\\s+(?:but|and|though|whereas)\\s+(?="), &g__TASTE_VERB()), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_FORM(stdlib_call("re", "compile", &[py_str("\\b(?:how (?:do|can|should|could) i|how to|what should i|what can i|what could i|any (?:tips|advice|ideas|suggestions)?|where (?:do|should) i start|is there a way)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_FOR_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:recommend|recommendations?|suggest|suggestions?)\\b|\\b(?:any|got any|have you got any)\\s+good\\b|\\bgive me (?:a|an|some|a few|your)\\b|\\bwhat should i (?:watch|read|listen to|see|get|buy|cook|make for|eat|play|do with)\\b|\\bhelp me (?:pick|choose|find|decide)\\b|\\bi(?:'m| am) looking for\\b|\\bi need (?:a|an|some|a few|new|ideas?|advice|help)\\b|\\bi(?:'d| would)? want (?:a|an|some) (?:new |good |different )?(?:film|movie|series|show|boxset|book|novel|podcast|audiobook|album|playlist|song)\\b|\\bwhere can i (?:get|buy|find)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HATED_CHORE(stdlib_call("re", "compile", &[py_str("\\b(?:hate|hates|dislike|dislikes|loathe|detest|can'?t stand|cannot stand|don'?t like|do not like)\\s+(?:to\\s+)?(?:cook|cooking|bake|baking|prep|prepping|meal prep|exercise|exercising|the gym|working out|run|running|clean|cleaning|iron|ironing|tidy|tidying|shop|shopping|wash|washing|diet|dieting)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_START(py_str("^\\W*(?:(?:so|and|but|ok|okay|hmm|oh|well|wait)\\b[,.]?\\s+)*"));
    s__ASK_ADVERB(py_str("(?:(?:still|really|actually|even|now|just|ever)\\s+)*"));
    s_ASKS_ABOUT_A_TASTE(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&g__ASK_START(), &py_str("(?:(?:do|did|does)\\s+(?:i|we)\\s+")), &g__ASK_ADVERB()), &py_str("(?:like|love|enjoy|eat|drink|want)\\b|am\\s+i\\s+")), &g__ASK_ADVERB()), &py_str("(?:a fan of|into|keen on|mad about|fussed on)\\b|did\\s+i\\s+say\\s+i\\s+(?:like|liked|love|loved|hate|hated)\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TASTE_ASKED_ABOUT(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&g__ASK_START(), &py_str("(?:(?:do|did|does)\\s+(?:i|we)\\s+")), &g__ASK_ADVERB()), &py_str("(?:like|love|enjoy|eat|drink|want)\\s+|am\\s+i\\s+")), &g__ASK_ADVERB()), &py_str("(?:a fan of|into|keen on|mad about|fussed on)\\s+|did\\s+i\\s+say\\s+i\\s+(?:like|liked|love|loved|hate|hated)\\s+)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_FOR_TASTES(stdlib_call("re", "compile", &[add(&add(&add(&g__ASK_START(), &py_str("(?:(?:what|which)\\b[^?.!]*?\\bdo\\s+(?:i|we)\\s+")), &g__ASK_ADVERB()), &py_str("(?:like|love|enjoy|eat|drink)\\b|(?:what|which)\\s+(?:are|were)\\s+my\\s+(?:favou?rite|best)\\s+\\w+|do\\s+you\\s+(?:know|remember)\\s+what\\s+(?:i|we)\\s+(?:like|love)\\b|what\\s+do\\s+you\\s+know\\s+(?:that\\s+)?i\\s+(?:like|love)\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OTHER_OWNER(stdlib_call("re", "compile", &[py_str("^(?:(?:my|our)\\s+(?P<who>[a-z][a-z'\\u2019-]{1,20})|(?P<pronoun>she|he|they))(?:'?s|'?re)?\\s+(?P<rest>.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OTHER_SAYS(py_tuple(vec![py_tuple(vec![py_str("(?:isn'?t|is\\s+not|wasn'?t|was\\s+not)\\s+(?:that\\s+|really\\s+|very\\s+|all\\s+that\\s+)?(?:into|keen\\s+on|a\\s+fan\\s+of|mad\\s+about|fussed\\s+on|big\\s+on)"), py_str("don't like")]), py_tuple(vec![py_str("(?:doesn'?t|does\\s+not|doesnt|won'?t|will\\s+not|never|can'?t|cannot|can\\s+not)\\s+(?:really\\s+|actually\\s+|ever\\s+)?(?:like|love|eat|drink|stand|touch|want|have|be\\s+keen\\s+on)"), py_str("don't like")]), py_tuple(vec![py_str("(?:hates?|loathes?|detests?|dislikes?|can'?t\\s+stand|cannot\\s+stand)"), py_str("hate")]), py_tuple(vec![py_str("is\\s+(?:really\\s+|absolutely\\s+|just\\s+|very\\s+)?(?:into|keen\\s+on|mad\\s+about|a\\s+fan\\s+of|big\\s+on|fussed\\s+on|fond\\s+of)"), py_str("like")]), py_tuple(vec![py_str("(?:loves?|likes?|adores?|prefers?|enjoys?)"), py_str("love")])]));
    s__OTHER_VERB(mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__OTHER_SAYS()) { let __unpk6383 = py_unpack(&__item.clone(), 2, None); let mut pattern: Py = __unpk6383[0].clone(); let mut _u: Py = __unpk6383[1].clone(); __out.push(modulo(&py_str("(?:%s)"), &pattern.clone())); } py_list(__out) })() }]));
    s__OTHER_CLAUSE(stdlib_call("re", "compile", &[add(&add(&py_str("\\s*;\\s*|\\s*,\\s*|\\s+(?:but|and|so|though|whereas|while)\\s+(?=(?:[a-z][a-z'\\u2019-]{1,20}\\s+)?(?:"), &g__OTHER_VERB()), &py_str(")\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CLAUSE_AFTER(stdlib_call("re", "compile", &[py_str("^\\s*[,;:]?\\s*(?:so|but|and|because|cos|cause|though|although|whereas|yet|if|then|when|while|as)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_SOMETHING(stdlib_call("re", "compile", &[py_str("\\b(?:what|how|why|when|where|which|who|whose|whats|what's|should|shall|could|would|any ideas|any tips|recommend)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OTHER_TASTE(stdlib_call("re", "compile", &[add(&add(&py_str("^(?:really |absolutely |just |actually |properly |simply |still )?(?:"), &g__OTHER_VERB()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _kept(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the thing that are kept, with the index each had in `thing`.");
    words = mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    raw = __unpk2[1].clone();
    word = mcall(&raw.clone(), "strip", &[py_str(".,;:!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THING_STOP(), &mcall(&word.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("your")]), &mcall(&word.clone(), "lower", &[]))) } } } }) {
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAIL(), &mcall(&word.clone(), "lower", &[]))) } }) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[py_tuple(vec![index.clone(), word.clone()])]);
    }
    while py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAIL(), &mcall(&geti(&geti(&kept.clone(), &neg(&Py::Int(1))), &Py::Int(1)), "lower", &[]))) } }) {
    let _ = mcall(&kept.clone(), "pop", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &mcall(&geti(&geti(&kept.clone(), &Py::Int(0)), &Py::Int(1)), "lower", &[]))) } }) {
    return py_list(vec![]);
    }
    return kept.clone();
    Py::None
}
pub fn _clean(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing with its tail off (\"curry tho\" -> \"curry\"), or None when it names nothing.");
    kept = _kept(&[thing.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let __unpk659 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk659[0].clone(); let mut word: Py = __unpk659[1].clone(); __out.push(word.clone()); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[])))) {
    let _ = py_setitem(&words.clone(), &Py::Int(0), py_str("your"));
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _thing_end(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut kept: Py = Py::None;
    let mut last: Py = Py::None;
    let mut took: Py = Py::None;
    let _ = py_str("Where the thing ends in the clause, so the words after it can be looked at.");
    took = slice_or_raise(&text.clone(), &mcall(&match_.clone(), "start", &[Py::Int(1)]), &mcall(&match_.clone(), "end", &[Py::Int(1)]), &Py::None);
    kept = _kept(&[took.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return mcall(&match_.clone(), "start", &[Py::Int(1)]);
    }
    last = geti(&mcall(&took.clone(), "split", &[]), &geti(&geti(&kept.clone(), &neg(&Py::Int(1))), &Py::Int(0)));
    return add(&add(&mcall(&match_.clone(), "start", &[Py::Int(1)]), &mcall(&took.clone(), "rfind", &[last.clone()])), &call_builtin("len", &[last.clone()]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _read_clause(args: &[Py]) -> Py {
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
    let mut how: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut shortfall: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("`(the taste in the clause, where in the clause it ends)`, or `(None, None)`.\n\n    The end is what lets a caller tell a line that is nothing but a taste from one that only\n    starts with one (\"i hate cats so what should i cook\").\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { mcall(&text.clone(), "endswith", &[py_str("?")]) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_UNSURE(), py_str("unsure")]), py_tuple(vec![g_NO(), py_str("no")]), py_tuple(vec![g_NOT_KEEN(), py_str("no")]), py_tuple(vec![g_MY_THING(), py_str("no")]), py_tuple(vec![g_YES(), py_str("yes")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    how = __unpk2[1].clone();
    match_ = mcall(&pattern.clone(), "search", &[text.clone()]);
    if py_truthy(&match_.clone()) {
    raw = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\s*to\\b"), raw.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    thing = _clean(&[raw.clone()]);
    shortfall = mcall(&g__SHORTFALL_VERB(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[Py::Int(1)]), &Py::None)]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__QUANTITY(), "match", &[thing.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__QUANTITY(), "match", &[mcall(&raw.clone(), "strip", &[])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = shortfall.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__SHORTFALL_TAIL(), "search", &[raw.clone()]) } })) } } } } } }) {
    return py_tuple(vec![py_dict(vec![(py_str("thing"), thing.clone()), (py_str("how"), how.clone())]), _thing_end(&[text.clone(), match_.clone()])]);
    }
    }
    }
    match_ = mcall(&g_HATE(), "search", &[text.clone()]);
    if py_truthy(&match_.clone()) {
    raw = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    thing = _clean(&[raw.clone()]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("\\s*(?:my|our)\\b"), raw.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_tuple(vec![py_dict(vec![(py_str("thing"), thing.clone()), (py_str("how"), py_str("no"))]), _thing_end(&[text.clone(), match_.clone()])]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn read(args: &[Py]) -> Py {
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
    let _ = py_str("The taste in the line as `{\"thing\", \"how\"}`, or None.\n\n    `how` is \"unsure\" (\"not sure if i like curry\"), \"no\" (\"i don't think i like fish\") or \"yes\"\n    (\"i love dogs\"). Unsure is kept apart on purpose: it is a hedge, not a dislike.\n    ");
    return geti(&_read_clause(&[line.clone()]), &Py::Int(0));
    Py::None
}
pub fn read_all(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Every taste in the line, in the order they said it.\n\n    `read()` answers with one taste, which is wrong for a line that holds two with different\n    polarity: \"i like dogs but i hate cats\" was kept as the single thing \"dogs but i hate cats\",\n    so the store called a dislike a like and said the person's own \"i\" back to them.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g__CLAUSE_SPLIT(), "split", &[text.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    clause = mcall(&clause.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "match", &[g__TASTE_VERB(), clause.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("(?:i|we|he|she|they)\\b"), clause.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    clause = add(&py_str("i "), &clause.clone());
    }
    taste = read(&[clause.clone()]);
    if py_truthy(&{ let __b = taste.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&found.clone(), &taste.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[taste.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn only_a_taste(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut end: Py = Py::None;
    let mut left: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut taste: Py = Py::None;
    let _ = py_str("Whether the line says nothing but tastes of theirs, one per clause.\n\n    \"i like dogs but i hate cats\" is a preference, not a household: an entry about introducing a\n    dog to a cat used to answer it (\"keep them apart behind a baby gate\"), because the line has\n    the two animals and the word \"hate\" in it. The store is what answers a taste, so the\n    knowledge base keeps out of a line that is only tastes. A line that carries anything else\n    (\"i hate cats so what should i cook\") is not this one.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__ASK_FORM(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__HATED_CHORE(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__ASKS_FOR_A_THING(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    seen = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&mcall(&g__CLAUSE_SPLIT(), "split", &[text.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    clause = mcall(&clause.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    continue;
    }
    said = clause.clone();
    bare = { let __b = stdlib_call("re", "match", &[g__TASTE_VERB(), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("(?:i|we|he|she|they)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } };
    if py_truthy(&bare.clone()) {
    said = add(&py_str("i "), &said.clone());
    }
    let __unpk2 = py_unpack(&_read_clause(&[said.clone()]), 2, None);
    taste = __unpk2[0].clone();
    end = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&taste.clone()))) {
    return Py::Bool(false);
    }
    left = if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) { slice_or_raise(&said.clone(), &end.clone(), &Py::None, &Py::None) } else { slice_or_raise(&said.clone(), &end.clone(), &Py::None, &Py::None) };
    if py_truthy(&sub(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&left.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_TAIL())) {
    return Py::Bool(false);
    }
    seen = Py::Bool(true);
    }
    return seen.clone();
    Py::None
}
pub fn said_about(args: &[Py]) -> Py {
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
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut before: Py = Py::None;
    let _ = py_str("Whether the animal at `position` is what a taste in the line is about, not their own pet.");
    before = slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), sub(&position.clone(), &Py::Int(30))]).unwrap_or_else(|e| exc::raise(e)), &position.clone(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_LIKING_BEFORE(), "search", &[before.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_OWNED_BEFORE(), "search", &[slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), sub(&position.clone(), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e)), &position.clone(), &Py::None)])));
    Py::None
}
pub fn asks_about_a_taste(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a question about a taste of theirs (\"do i like fish?\").");
    return call_builtin("bool", &[mcall(&g_ASKS_ABOUT_A_TASTE(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn read_other(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut person: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tasted: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Tastes somebody else has, as `[{\"who\"/\"pronoun\", \"thing\", \"how\"}]`.\n\n    \"my sister loves cooking\", \"she doesnt like olives\", \"he is mad about football\": the person\n    is the relation in front of the taste, or the pronoun when the line names nobody (the store\n    fills that in from the person the chat is on). The taste itself is read by the reader above,\n    on the clause rewritten as the person's own (\"i love cooking\"), so a quantity, a tail or a\n    pronoun for a thing is refused here in exactly the same way, and a line that carries more\n    than the taste (\"she loves cooking so what do i cook\") is not one.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { mcall(&text.clone(), "endswith", &[py_str("?")]) } }) {
    return py_list(vec![]);
    }
    found = py_list(vec![]);
    person = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g__OTHER_CLAUSE(), "split", &[text.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    clause = mcall(&clause.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    continue;
    }
    match_ = mcall(&g__OTHER_OWNER(), "match", &[clause.clone()]);
    if py_truthy(&match_.clone()) {
    person = if py_truthy(&mcall(&match_.clone(), "group", &[py_str("who")])) { py_dict(vec![(py_str("who"), mcall(&mcall(&match_.clone(), "group", &[py_str("who")]), "lower", &[]))]) } else { py_dict(vec![(py_str("pronoun"), mcall(&mcall(&match_.clone(), "group", &[py_str("pronoun")]), "lower", &[]))]) };
    clause = mcall(&match_.clone(), "group", &[py_str("rest")]);
    }
    if py_truthy(&Py::Bool(py_is(&person.clone(), &Py::None))) {
    continue;
    }
    said = mcall(&g__OTHER_TASTE(), "match", &[clause.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    tasted = _their_taste_in(&[clause.clone(), said.clone()]);
    if py_truthy(&tasted.clone()) {
    let _ = mcall(&found.clone(), "append", &[dict_spread(&[tasted.clone(), person.clone()])]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _their_taste_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut as_own: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut end: Py = Py::None;
    let mut left: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rewritten: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut tasted: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The taste one clause gives, read as the person's own (\"i love cooking\"), or None.");
    verb = mcall(&mcall(&said.clone(), "group", &[Py::Int(0)]), "strip", &[]);
    thing = mcall(&slice_or_raise(&rest.clone(), &mcall(&said.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    as_own = py_str("love");
    let __iter1 = py_iter_or_raise(&g__OTHER_SAYS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    rewritten = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "fullmatch", &[pattern.clone(), verb.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    as_own = rewritten.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    clause = add(&add(&add(&py_str("i "), &as_own.clone()), &py_str(" ")), &thing.clone());
    let __unpk3 = py_unpack(&_read_clause(&[clause.clone()]), 2, None);
    taste = __unpk3[0].clone();
    end = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&taste.clone()))) {
    return Py::None;
    }
    left = slice_or_raise(&clause.clone(), &end.clone(), &Py::None, &Py::None);
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&left.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&sub(&words.clone(), &g_TAIL())) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__A_CLAUSE_AFTER(), "match", &[left.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g__ASKS_SOMETHING(), "search", &[left.clone()]) } }) {
    return Py::None;
    }
    }
    tasted = py_dict(vec![(py_str("thing"), geti(&taste.clone(), &py_str("thing"))), (py_str("how"), geti(&taste.clone(), &py_str("how")))]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:eat|eats|ate|drink|drinks|drank)\\b"), verb.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&tasted.clone(), &py_str("eaten"), Py::Bool(true));
    }
    return tasted.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.taste".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_ABOUT_A_TASTE" => Some(g_ASKS_ABOUT_A_TASTE()),
            "ASKS_FOR_TASTES" => Some(g_ASKS_FOR_TASTES()),
            "DISLIKE_IN" => Some(g_DISLIKE_IN()),
            "HATE" => Some(g_HATE()),
            "LIKING_BEFORE" => Some(g_LIKING_BEFORE()),
            "MY_THING" => Some(g_MY_THING()),
            "NO" => Some(g_NO()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_KEEN" => Some(g_NOT_KEEN()),
            "OWNED_BEFORE" => Some(g_OWNED_BEFORE()),
            "TAIL" => Some(g_TAIL()),
            "TASTE_ASKED_ABOUT" => Some(g_TASTE_ASKED_ABOUT()),
            "THING" => Some(g_THING()),
            "THING_STOP" => Some(g_THING_STOP()),
            "UNSURE" => Some(g_UNSURE()),
            "YES" => Some(g_YES()),
            "_ASKS_FOR_A_THING" => Some(g__ASKS_FOR_A_THING()),
            "_ASKS_SOMETHING" => Some(g__ASKS_SOMETHING()),
            "_ASK_ADVERB" => Some(g__ASK_ADVERB()),
            "_ASK_FORM" => Some(g__ASK_FORM()),
            "_ASK_START" => Some(g__ASK_START()),
            "_A_CLAUSE_AFTER" => Some(g__A_CLAUSE_AFTER()),
            "_CLAUSE_SPLIT" => Some(g__CLAUSE_SPLIT()),
            "_HATED_CHORE" => Some(g__HATED_CHORE()),
            "_OTHER_CLAUSE" => Some(g__OTHER_CLAUSE()),
            "_OTHER_OWNER" => Some(g__OTHER_OWNER()),
            "_OTHER_SAYS" => Some(g__OTHER_SAYS()),
            "_OTHER_TASTE" => Some(g__OTHER_TASTE()),
            "_OTHER_VERB" => Some(g__OTHER_VERB()),
            "_QUANTITY" => Some(g__QUANTITY()),
            "_SHORTFALL_TAIL" => Some(g__SHORTFALL_TAIL()),
            "_SHORTFALL_VERB" => Some(g__SHORTFALL_VERB()),
            "_TASTE_VERB" => Some(g__TASTE_VERB()),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_kept" => Some(py_func("_kept", |args: &[Py]| _kept(args))),
            "_read_clause" => Some(py_func("_read_clause", |args: &[Py]| _read_clause(args))),
            "_their_taste_in" => Some(py_func("_their_taste_in", |args: &[Py]| _their_taste_in(args))),
            "_thing_end" => Some(py_func("_thing_end", |args: &[Py]| _thing_end(args))),
            "asks_about_a_taste" => Some(py_func("asks_about_a_taste", |args: &[Py]| asks_about_a_taste(args))),
            "only_a_taste" => Some(py_func("only_a_taste", |args: &[Py]| only_a_taste(args))),
            "read" => Some(py_func("read", |args: &[Py]| read(args))),
            "read_all" => Some(py_func("read_all", |args: &[Py]| read_all(args))),
            "read_other" => Some(py_func("read_other", |args: &[Py]| read_other(args))),
            "said_about" => Some(py_func("said_about", |args: &[Py]| said_about(args))),
            _ => None,
        }),
        set: None,
    }))
}
