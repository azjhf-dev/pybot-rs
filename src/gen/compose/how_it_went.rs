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


pub static AS_EXPECTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAREFUL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPARE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPECTED_BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FELT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAVY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD_STRIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MORE_THAN_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_WENT_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_AN_EVENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_AS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_SO_BAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_SURPRISES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RELIEF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SINGULAR_ENDS_IN_S: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STANDS_ALONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_PLURAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_SINGLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTED_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHICH_WAS_THE_AIM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AS_EXPECTED() -> Py { ensure_init(); AS_EXPECTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AS_EXPECTED(v: Py) { ensure_init(); if let Ok(mut w) = AS_EXPECTED.write() { *w = v; } }
pub fn g_BETTER() -> Py { ensure_init(); BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = BETTER.write() { *w = v; } }
pub fn g_CAREFUL() -> Py { ensure_init(); CAREFUL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAREFUL(v: Py) { ensure_init(); if let Ok(mut w) = CAREFUL.write() { *w = v; } }
pub fn g_COMPARE() -> Py { ensure_init(); COMPARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARE(v: Py) { ensure_init(); if let Ok(mut w) = COMPARE.write() { *w = v; } }
pub fn g_COMPARED() -> Py { ensure_init(); COMPARED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARED(v: Py) { ensure_init(); if let Ok(mut w) = COMPARED.write() { *w = v; } }
pub fn g_COMPARE_WORD() -> Py { ensure_init(); COMPARE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPARE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = COMPARE_WORD.write() { *w = v; } }
pub fn g_COPULA() -> Py { ensure_init(); COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); if let Ok(mut w) = COPULA.write() { *w = v; } }
pub fn g_EXPECTED_BARE() -> Py { ensure_init(); EXPECTED_BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPECTED_BARE(v: Py) { ensure_init(); if let Ok(mut w) = EXPECTED_BARE.write() { *w = v; } }
pub fn g_FELT() -> Py { ensure_init(); FELT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FELT(v: Py) { ensure_init(); if let Ok(mut w) = FELT.write() { *w = v; } }
pub fn g_HEAVY() -> Py { ensure_init(); HEAVY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAVY(v: Py) { ensure_init(); if let Ok(mut w) = HEAVY.write() { *w = v; } }
pub fn g_HOPED() -> Py { ensure_init(); HOPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOPED(v: Py) { ensure_init(); if let Ok(mut w) = HOPED.write() { *w = v; } }
pub fn g_HOW() -> Py { ensure_init(); HOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW(v: Py) { ensure_init(); if let Ok(mut w) = HOW.write() { *w = v; } }
pub fn g_LEAD() -> Py { ensure_init(); LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = LEAD.write() { *w = v; } }
pub fn g_LEAD_STRIP() -> Py { ensure_init(); LEAD_STRIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD_STRIP(v: Py) { ensure_init(); if let Ok(mut w) = LEAD_STRIP.write() { *w = v; } }
pub fn g_LOW() -> Py { ensure_init(); LOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOW(v: Py) { ensure_init(); if let Ok(mut w) = LOW.write() { *w = v; } }
pub fn g_MINE() -> Py { ensure_init(); MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MINE(v: Py) { ensure_init(); if let Ok(mut w) = MINE.write() { *w = v; } }
pub fn g_MORE_THAN_ONE() -> Py { ensure_init(); MORE_THAN_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MORE_THAN_ONE(v: Py) { ensure_init(); if let Ok(mut w) = MORE_THAN_ONE.write() { *w = v; } }
pub fn g_NOTHING_WENT_WRONG() -> Py { ensure_init(); NOTHING_WENT_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_WENT_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_WENT_WRONG.write() { *w = v; } }
pub fn g_NOT_AN_EVENT() -> Py { ensure_init(); NOT_AN_EVENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_EVENT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_AN_EVENT.write() { *w = v; } }
pub fn g_NOT_AS() -> Py { ensure_init(); NOT_AS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_AS(v: Py) { ensure_init(); if let Ok(mut w) = NOT_AS.write() { *w = v; } }
pub fn g_NOT_SO_BAD() -> Py { ensure_init(); NOT_SO_BAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_SO_BAD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_SO_BAD.write() { *w = v; } }
pub fn g_NO_SURPRISES() -> Py { ensure_init(); NO_SURPRISES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_SURPRISES(v: Py) { ensure_init(); if let Ok(mut w) = NO_SURPRISES.write() { *w = v; } }
pub fn g_PAST() -> Py { ensure_init(); PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST(v: Py) { ensure_init(); if let Ok(mut w) = PAST.write() { *w = v; } }
pub fn g_RELIEF() -> Py { ensure_init(); RELIEF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RELIEF(v: Py) { ensure_init(); if let Ok(mut w) = RELIEF.write() { *w = v; } }
pub fn g_SINGULAR_ENDS_IN_S() -> Py { ensure_init(); SINGULAR_ENDS_IN_S.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SINGULAR_ENDS_IN_S(v: Py) { ensure_init(); if let Ok(mut w) = SINGULAR_ENDS_IN_S.write() { *w = v; } }
pub fn g_SOFT() -> Py { ensure_init(); SOFT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT(v: Py) { ensure_init(); if let Ok(mut w) = SOFT.write() { *w = v; } }
pub fn g_STANDS_ALONE() -> Py { ensure_init(); STANDS_ALONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STANDS_ALONE(v: Py) { ensure_init(); if let Ok(mut w) = STANDS_ALONE.write() { *w = v; } }
pub fn g_STILL_PLURAL() -> Py { ensure_init(); STILL_PLURAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_PLURAL(v: Py) { ensure_init(); if let Ok(mut w) = STILL_PLURAL.write() { *w = v; } }
pub fn g_STILL_SINGLE() -> Py { ensure_init(); STILL_SINGLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_SINGLE(v: Py) { ensure_init(); if let Ok(mut w) = STILL_SINGLE.write() { *w = v; } }
pub fn g_THAN() -> Py { ensure_init(); THAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THAN(v: Py) { ensure_init(); if let Ok(mut w) = THAN.write() { *w = v; } }
pub fn g_THEIRS() -> Py { ensure_init(); THEIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIRS(v: Py) { ensure_init(); if let Ok(mut w) = THEIRS.write() { *w = v; } }
pub fn g_THE_BOT() -> Py { ensure_init(); THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = THE_BOT.write() { *w = v; } }
pub fn g_WANTED_IT() -> Py { ensure_init(); WANTED_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTED_IT(v: Py) { ensure_init(); if let Ok(mut w) = WANTED_IT.write() { *w = v; } }
pub fn g_WHICH_WAS_THE_AIM() -> Py { ensure_init(); WHICH_WAS_THE_AIM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHICH_WAS_THE_AIM(v: Py) { ensure_init(); if let Ok(mut w) = WHICH_WAS_THE_AIM.write() { *w = v; } }
pub fn g_WORSE() -> Py { ensure_init(); WORSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORSE(v: Py) { ensure_init(); if let Ok(mut w) = WORSE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A question about how a thing of theirs went is theirs to answer, not a search.\n\nOn main \"how was the wedding\", \"how did the scan go\", \"how are the kids\" and \"how is work\" all\nreach the composer's search hedge: \"A quick search online will get you a solid answer\". Nothing a\nsearch turns up about the wedding is the wedding the person went to. The bot was not there and\ncannot see the person's week; the one who knows is the person, and what a friend does with the\nquestion is ask it back and let them tell it.\n\nThe reading is whose the thing is, which is the possessive in the line:\n\n    how was the wedding        a thing of theirs     -> \"How did it go?\"\n    how did the scan go        a thing of theirs     -> \"I hope it went as well as it could\"\n    how are the kids           people of theirs     -> \"How are they doing?\"\n    how is work                theirs              -> \"How is it going?\"\n    how was your day           the bot's own day  -> not this file (the manners pools)\n    how was the roman empire   a thing in the world -> not this file\n    how long do i boil an egg  a how-to           -> not this file\n\n\"my/our/his/her/their <anything>\" is a thing of the person's that the bot has no way to know, so\nany noun after it is read as theirs. \"the <thing>\" is theirs only when the thing is the kind that\nsits in a person's own life - an event, a person, a pet, work, the house, a meal, a journey - so\nthat list is what keeps \"how was the war\" and \"how was the empire\" out. A line with the bot in it\n(\"how was your day\", \"how are you\") is the manners question and stands down here.\n\nEvery reply says only what is true - the bot was not there and only has the person's words - and\ngives the question back in the same words for any noun, so nothing is invented to fill the gap.\nThe medical ones are careful: a cheerful \"how was it?\" is the wrong tone for a scan, so those\nsay they hope it went as well as it could.\n\nThis is a last resort and it is written to be one: `bot/modules/greeting.py` asks it in the same\nplace as the other readers of a hedge, and the composer is asked first, so a line any entry, any\nmodule or the composer can answer never reaches this file. Only a hedge is replaced.\n");
    s_THEIRS(call_builtin("frozenset", &[mcall(&py_str("\nwedding funeral wake christening baptism party celebration anniversary birthday\nscan appointment interview exam test operation surgery treatment chemo chemotherapy\nradiotherapy biopsy results procedure checkup consultation diagnosis meeting\npresentation review appraisal shift holiday honeymoon trip flight journey drive\nmove date match game concert gig show play film meal dinner lunch breakfast roast\ntakeaway restaurant hotel kids children child family mum mom dad mother father parent wife\nhusband partner son daughter sister brother baby toddler grandparents gran grandma\ngrandad nana dog cat puppy kitten pet rabbit hamster work job business shop team\nhouse home flat garden\nallotment course term lesson class school nursery weekend week day night morning\nevening church ceremony service birth viewing rehearsal training practice football\nrugby\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_EVENT(call_builtin("frozenset", &[py_tuple(vec![py_str("fault"), py_str("idea"), py_str("point"), py_str("question"), py_str("turn"), py_str("problem"), py_str("favour")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEAVY(call_builtin("frozenset", &[mcall(&py_str("\nfuneral wake scan operation surgery treatment chemo chemotherapy radiotherapy biopsy\nresults procedure checkup diagnosis\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW(stdlib_call("re", "compile", &[py_str("^(?:(?:so|and|ok|okay|well|oh|but|right)[,\\s]+)?how(?:\\s+(?:was|were|is|are|did|does)|'s|s)\\s+(?:(?P<whose>my|our|his|her|their|the)\\s+)?(?P<thing>[a-z][a-z' -]{0,30}?)(?:\\s+(?:go|went|going|doing|then|today|yesterday|tonight|last night|this week))*\\s*\\??$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FELT(stdlib_call("re", "compile", &[py_str("^(?:(?:so|and|ok|okay|well|oh|but|right)[,\\s]+)?(?:was|were)\\s+(?:it|that|this)\\s+(?:(?:really|actually|very|too|that|so|pretty)\\s+)?(?P<word>bad|good|nice|hard|tough|rough|scary|easy|awful|ok|okay|alright|fine|fun|horrible|dreadful|painful|tiring|stressful|busy|long|slow|worrying|grim|lovely|great|sunny|warm|cold|wet|windy|hot)\\s*(?:for\\s+(?:you|her|him|them))?\\s*[?!.]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_BOT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:you|your|yours|yourself|u|ur)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST(py_tuple(vec![py_str("You'll have to tell me - I only get what you type. How did it go?"), py_str("I wasn't there, so you're the one who knows. How did it go?"), py_str("Nothing of it reached me here, so tell me: how did it go?"), py_str("You tell me - how did it go? I only get what you type.")]));
    s_STILL_PLURAL(py_tuple(vec![py_str("You'll have to tell me - I only get what you type. How are they doing?"), py_str("I'm not there for any of it, so you're the one who knows. How are they?"), py_str("Nothing of that reaches me here. So how are they?")]));
    s_STILL_SINGLE(py_tuple(vec![py_str("You'll have to tell me - I only get what you type. How is it going?"), py_str("I'm not there for any of it, so you're the one who knows. How is it going?"), py_str("Nothing of that reaches me here. So how is it going?")]));
    s_CAREFUL(py_tuple(vec![py_str("I only know what you tell me. I hope it went as well as it could - how are you doing now?"), py_str("Nothing of that reaches me, so I only have what you say. How did it go?"), py_str("I hope it went as well as it could. Tell me how you are doing with it.")]));
    s_MORE_THAN_ONE(call_builtin("frozenset", &[mcall(&py_str("\nchildren kids twins people parents grandparents mumanddad results tests scans appointments\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SINGULAR_ENDS_IN_S(call_builtin("frozenset", &[mcall(&py_str("\nbusiness class christmas maths news series gas address boss\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD(py_str("(?:(?:oh|ah|well|actually|honestly|yeah|yep|no|ha|hmm|so|and|but|ok|okay|thankfully|happily)[,\\s]+)*"));
    s_COMPARE(py_str("(?:better|worse|easier|harder|nicer|quicker|slower|cheaper|quieter|rougher|smoother|calmer|lighter|heavier|longer|shorter|funnier|duller)"));
    s_SOFT(py_str("(?:much|far|a lot|loads|a bit|bit|slightly|definitely|way|even|so much)?\\s*"));
    s_COPULA(py_str("(?:(?:it|that|this)(?:'s|s| is| was| went| turned out| feels)?\\s+)?"));
    s_MINE(py_str("(?:i|we)"));
    s_HOPED(py_str("(?:expected|thought|hoped|feared|imagined|dreaded|anticipated|planned|was expecting|were expecting|was hoping|were hoping)"));
    s_THAN(add(&add(&add(&add(&py_str("than\\s+(?:"), &g_MINE()), &py_str("\\s+(?:")), &g_HOPED()), &py_str(")|last\\s+time|yesterday|before|the\\s+last\\s+one|the\\s+other\\s+one|the\\s+first\\s+(?:one|time)|the\\s+usual|usual|(?:the\\s+)?(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekend|christmas|last\\s+year|last\\s+week|last\\s+month))\\b")));
    s_COMPARED(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&py_str("^"), &g_LEAD()), &g_COPULA()), &g_SOFT()), &py_str("(?P<word>")), &g_COMPARE()), &py_str(")\\s*")), &g_THAN()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AS(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&py_str("^"), &g_LEAD()), &g_COPULA()), &py_str("not\\s+(?:as|so)\\s+(?P<word>good|bad|nice|easy|hard|busy|expensive|cheap|scary|painful)\\s+as\\s+")), &g_MINE()), &py_str("\\s+(?:")), &g_HOPED()), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AS_EXPECTED(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("^"), &g_LEAD()), &py_str("(?:(?:about|roughly|pretty|more\\s+or\\s+less)\\s+)?(?:as|what)\\s+")), &g_MINE()), &py_str("\\s+expected\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPECTED_BARE(stdlib_call("re", "compile", &[add(&add(&py_str("^"), &g_LEAD()), &py_str("(?:(?:pretty|much|about|roughly|more\\s+or\\s+less)\\s+|pretty\\s+much\\s+)*as\\s+expected\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHICH_WAS_THE_AIM(stdlib_call("re", "compile", &[add(&add(&py_str("^(?P<what>[a-z][a-z' -]{1,24}?),?\\s+which\\s+(?:is|was)\\s+(?:just\\s+)?what\\s+"), &g_MINE()), &py_str("\\s+(?:wanted|needed|hoped\\s+for|(?:was|were)\\s+after)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_WENT_WRONG(stdlib_call("re", "compile", &[py_str("\\b(?:broke|broken|failed|died|stopped|leaked|flooded|crashed|caught\\s+fire|went\\s+wrong|lost|cancelled|ruined)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STANDS_ALONE(call_builtin("frozenset", &[mcall(&py_str("\nit that this not much far way so even still definitely bit slightly honestly actually\nwell oh ah yeah yep no about roughly pretty as what\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPARE_WORD(stdlib_call("re", "compile", &[add(&add(&py_str("^(?:"), &g_COMPARE()), &py_str(")$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD_STRIP(stdlib_call("re", "compile", &[add(&py_str("^"), &g_LEAD())]).unwrap_or_else(|e| exc::raise(e)));
    s_BETTER(py_tuple(vec![py_str("Good - better than you expected is the best way round for it to go."), py_str("That's a nice surprise. What made the difference?"), py_str("Pleased to hear it. What was better about it than you thought?")]));
    s_WORSE(py_tuple(vec![py_str("Sorry - that's a let-down when you had hoped for more."), py_str("Ah, I'm sorry. What was off about it?"), py_str("That's a shame. Do you know what changed?")]));
    s_RELIEF(py_tuple(vec![py_str("That's a relief, then. What was it actually like?"), py_str("Good - the dread is usually worse than the thing itself."), py_str("Pleased it was not the ordeal you had braced for.")]));
    s_NO_SURPRISES(py_tuple(vec![py_str("No surprises then. Is there anything you would do differently next time?"), py_str("Fair enough - it did what you expected of it."), py_str("So it went the way you thought it would. Nothing to fix.")]));
    s_WANTED_IT(py_tuple(vec![py_str("Good - {0} is exactly what you were after."), py_str("That's the good kind of {0}. Pleased it landed that way."), py_str("{0} was the point of it, so that counts as a good one.")]));
    s_LOW(call_builtin("frozenset", &[mcall(&py_str("worse harder rougher busier heavier longer slower duller"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_SO_BAD(call_builtin("frozenset", &[mcall(&py_str("bad hard busy expensive scary painful"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn a_kind_of_theirs(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut head: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let mut suffix: Py = Py::None;
    let _ = py_str("Is the bare \"the X\" a kind of thing that sits in a person's own life?\n\n    The plural of a word that is in THEIRS is the same kind of thing, so the list holds the word\n    once (\"the tests\", \"the appointments\", \"the scans\" all read through \"test\", \"appointment\",\n    \"scan\"). That is a rule rather than a second list, so a word added above comes with its\n    plural already.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_THEIRS(), &head.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("s"), py_str("es")]));
    for __item in __iter1 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = mcall(&head.clone(), "endswith", &[suffix.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THEIRS(), &slice_or_raise(&head.clone(), &Py::None, &neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn more_than_one(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut head: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let _ = py_str("Is the thing the person asks about more than one of something?");
    if py_truthy(&Py::Bool(py_contains(&g_MORE_THAN_ONE(), &head.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = mcall(&head.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SINGULAR_ENDS_IN_S(), &head.clone())) } };
    Py::None
}
pub fn _head(args: &[Py]) -> Py {
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
    let _ = py_str("The last word is what the thing is: \"school play\" is a play, \"first day\" is a day.");
    return if py_truthy(&mcall(&thing.clone(), "split", &[])) { mcall(&geti(&mcall(&thing.clone(), "split", &[]), &neg(&Py::Int(1))), "strip", &[py_str("'")]) } else { py_str("") };
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
    else { Py::None };
    let mut felt: Py = Py::None;
    let mut head: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut whose: Py = Py::None;
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]), "rstrip", &[py_str(" .!,;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_THE_BOT(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    match_ = mcall(&g_HOW(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    felt = mcall(&g_FELT(), "match", &[line.clone()]);
    if py_truthy(&felt.clone()) {
    return crate::gen::compose::pick(&[call_builtin("list", &[g_PAST()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    }
    return Py::None;
    }
    thing = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    whose = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("whose")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    head = _head(&[thing.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_AN_EVENT(), &head.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their")]), &whose.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_kind_of_theirs(&[head.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_HEAVY(), &head.clone()))) {
    pool = g_CAREFUL();
    } else {
    if py_truthy(&stdlib_call("re", "match", &[py_str("^(?:(?:so|and|ok|okay|well|oh|but|right)[,\\s]+)?how(?:'s|s| is| are)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    pool = if py_truthy(&more_than_one(&[head.clone()])) { g_STILL_PLURAL() } else { g_STILL_SINGLE() };
    } else {
    pool = g_PAST();
    }
    }
    return crate::gen::compose::pick(&[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    Py::None
}
pub fn _opens_on_the_comparison(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let _ = py_str("True when nothing but the comparison is in front of it.\n\n    \"better than i expected\" opens on the comparison; \"my back is worse than yesterday\" opens on\n    a body part, and the reading below is not about bodies.\n    ");
    rest = mcall(&g_LEAD_STRIP(), "sub", &[py_str(""), line.clone()]);
    first = if py_truthy(&mcall(&rest.clone(), "split", &[])) { geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0)) } else { py_str("") };
    return { let __b = Py::Bool(py_contains(&g_STANDS_ALONE(), &first.clone())); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_COMPARE_WORD(), "match", &[first.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn the_answer_they_gave(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut not_as: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("The reply to a person's verdict on how a thing of theirs went, or None.\n\n    Only a line that is nothing but the comparison is read. A line with a subject, a question\n    mark or the bot in it keeps whatever answer it already had.\n    ");
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]), "rstrip", &[py_str(" .!,;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_THE_BOT(), "search", &[line.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9))) {
    return Py::None;
    }
    aim = mcall(&g_WHICH_WAS_THE_AIM(), "match", &[line.clone()]);
    if py_truthy(&aim.clone()) {
    what = mcall(&mcall(&aim.clone(), "group", &[py_str("what")]), "strip", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("it was "), py_str("it is "), py_str("that was "), py_str("this was ")]));
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&mcall(&what.clone(), "startswith", &[lead.clone()])) {
    what = slice_or_raise(&what.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&what.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOTHING_WENT_WRONG(), "search", &[what.clone()]); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&what.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } } } }) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WANTED_IT()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "format", &[what.clone()])); } py_list(__out) })() }, { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_opens_on_the_comparison(&[line.clone()])))) {
    return Py::None;
    }
    match_ = mcall(&g_COMPARED(), "search", &[line.clone()]);
    not_as = mcall(&g_NOT_AS(), "search", &[line.clone()]);
    if py_truthy(&match_.clone()) {
    pool = if py_truthy(&Py::Bool(py_contains(&g_LOW(), &mcall(&mcall(&match_.clone(), "group", &[py_str("word")]), "lower", &[])))) { g_WORSE() } else { g_BETTER() };
    } else {
    if py_truthy(&not_as.clone()) {
    pool = if py_truthy(&Py::Bool(py_contains(&g_NOT_SO_BAD(), &mcall(&mcall(&not_as.clone(), "group", &[py_str("word")]), "lower", &[])))) { g_RELIEF() } else { g_WORSE() };
    } else {
    if py_truthy(&{ let __b = mcall(&g_AS_EXPECTED(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_EXPECTED_BARE(), "search", &[line.clone()]) } }) {
    pool = g_NO_SURPRISES();
    } else {
    return Py::None;
    }
    }
    }
    return crate::gen::compose::pick(&[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.how_it_went".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AS_EXPECTED" => Some(g_AS_EXPECTED()),
            "BETTER" => Some(g_BETTER()),
            "CAREFUL" => Some(g_CAREFUL()),
            "COMPARE" => Some(g_COMPARE()),
            "COMPARED" => Some(g_COMPARED()),
            "COMPARE_WORD" => Some(g_COMPARE_WORD()),
            "COPULA" => Some(g_COPULA()),
            "EXPECTED_BARE" => Some(g_EXPECTED_BARE()),
            "FELT" => Some(g_FELT()),
            "HEAVY" => Some(g_HEAVY()),
            "HOPED" => Some(g_HOPED()),
            "HOW" => Some(g_HOW()),
            "LEAD" => Some(g_LEAD()),
            "LEAD_STRIP" => Some(g_LEAD_STRIP()),
            "LOW" => Some(g_LOW()),
            "MINE" => Some(g_MINE()),
            "MORE_THAN_ONE" => Some(g_MORE_THAN_ONE()),
            "NOTHING_WENT_WRONG" => Some(g_NOTHING_WENT_WRONG()),
            "NOT_AN_EVENT" => Some(g_NOT_AN_EVENT()),
            "NOT_AS" => Some(g_NOT_AS()),
            "NOT_SO_BAD" => Some(g_NOT_SO_BAD()),
            "NO_SURPRISES" => Some(g_NO_SURPRISES()),
            "PAST" => Some(g_PAST()),
            "RELIEF" => Some(g_RELIEF()),
            "SINGULAR_ENDS_IN_S" => Some(g_SINGULAR_ENDS_IN_S()),
            "SOFT" => Some(g_SOFT()),
            "STANDS_ALONE" => Some(g_STANDS_ALONE()),
            "STILL_PLURAL" => Some(g_STILL_PLURAL()),
            "STILL_SINGLE" => Some(g_STILL_SINGLE()),
            "THAN" => Some(g_THAN()),
            "THEIRS" => Some(g_THEIRS()),
            "THE_BOT" => Some(g_THE_BOT()),
            "WANTED_IT" => Some(g_WANTED_IT()),
            "WHICH_WAS_THE_AIM" => Some(g_WHICH_WAS_THE_AIM()),
            "WORSE" => Some(g_WORSE()),
            "_head" => Some(py_func("_head", |args: &[Py]| _head(args))),
            "_opens_on_the_comparison" => Some(py_func("_opens_on_the_comparison", |args: &[Py]| _opens_on_the_comparison(args))),
            "a_kind_of_theirs" => Some(py_func("a_kind_of_theirs", |args: &[Py]| a_kind_of_theirs(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "more_than_one" => Some(py_func("more_than_one", |args: &[Py]| more_than_one(args))),
            "the_answer_they_gave" => Some(py_func("the_answer_they_gave", |args: &[Py]| the_answer_they_gave(args))),
            _ => None,
        }),
        set: None,
    }))
}
