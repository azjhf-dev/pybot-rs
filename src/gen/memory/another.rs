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


pub static ABOUT_ANOTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANSWER_MODULES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_OPTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COMMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CONDITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MISSING_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PROFESSIONAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARRY_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTINUATION_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DO_THIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FILING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LENGTH_IDIOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOK_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFFERS_ANOTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPTION_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_OPTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUOTE_MARKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULED_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWAP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_MISSING_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static URGENT_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_ANOTHER() -> Py { ensure_init(); ABOUT_ANOTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_ANOTHER(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_ANOTHER.write() { *w = v; } }
pub fn g_ANSWER_MODULES() -> Py { ensure_init(); ANSWER_MODULES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWER_MODULES(v: Py) { ensure_init(); if let Ok(mut w) = ANSWER_MODULES.write() { *w = v; } }
pub fn g_AN_OPTION() -> Py { ensure_init(); AN_OPTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_OPTION(v: Py) { ensure_init(); if let Ok(mut w) = AN_OPTION.write() { *w = v; } }
pub fn g_ASK() -> Py { ensure_init(); ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ASK.write() { *w = v; } }
pub fn g_A_COMMENT() -> Py { ensure_init(); A_COMMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COMMENT(v: Py) { ensure_init(); if let Ok(mut w) = A_COMMENT.write() { *w = v; } }
pub fn g_A_CONDITION() -> Py { ensure_init(); A_CONDITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION(v: Py) { ensure_init(); if let Ok(mut w) = A_CONDITION.write() { *w = v; } }
pub fn g_A_MISSING_PART() -> Py { ensure_init(); A_MISSING_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MISSING_PART(v: Py) { ensure_init(); if let Ok(mut w) = A_MISSING_PART.write() { *w = v; } }
pub fn g_A_PERSON() -> Py { ensure_init(); A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON.write() { *w = v; } }
pub fn g_A_PROFESSIONAL() -> Py { ensure_init(); A_PROFESSIONAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PROFESSIONAL(v: Py) { ensure_init(); if let Ok(mut w) = A_PROFESSIONAL.write() { *w = v; } }
pub fn g_CARRY_ON() -> Py { ensure_init(); CARRY_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARRY_ON(v: Py) { ensure_init(); if let Ok(mut w) = CARRY_ON.write() { *w = v; } }
pub fn g_CONTINUATION_WORDS() -> Py { ensure_init(); CONTINUATION_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTINUATION_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = CONTINUATION_WORDS.write() { *w = v; } }
pub fn g_DO_THIS() -> Py { ensure_init(); DO_THIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DO_THIS(v: Py) { ensure_init(); if let Ok(mut w) = DO_THIS.write() { *w = v; } }
pub fn g_FILING() -> Py { ensure_init(); FILING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FILING(v: Py) { ensure_init(); if let Ok(mut w) = FILING.write() { *w = v; } }
pub fn g_FRAME() -> Py { ensure_init(); FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = FRAME.write() { *w = v; } }
pub fn g_FRAME_WORDS() -> Py { ensure_init(); FRAME_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_WORDS.write() { *w = v; } }
pub fn g_HEDGE() -> Py { ensure_init(); HEDGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGE(v: Py) { ensure_init(); if let Ok(mut w) = HEDGE.write() { *w = v; } }
pub fn g_LEADS() -> Py { ensure_init(); LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LEADS.write() { *w = v; } }
pub fn g_LEAD_IN() -> Py { ensure_init(); LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = LEAD_IN.write() { *w = v; } }
pub fn g_LEAST() -> Py { ensure_init(); LEAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAST(v: Py) { ensure_init(); if let Ok(mut w) = LEAST.write() { *w = v; } }
pub fn g_LENGTH_IDIOM() -> Py { ensure_init(); LENGTH_IDIOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LENGTH_IDIOM(v: Py) { ensure_init(); if let Ok(mut w) = LENGTH_IDIOM.write() { *w = v; } }
pub fn g_LOOK_BACK() -> Py { ensure_init(); LOOK_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); if let Ok(mut w) = LOOK_BACK.write() { *w = v; } }
pub fn g_MOST() -> Py { ensure_init(); MOST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOST(v: Py) { ensure_init(); if let Ok(mut w) = MOST.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_OFFERS_ANOTHER() -> Py { ensure_init(); OFFERS_ANOTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFFERS_ANOTHER(v: Py) { ensure_init(); if let Ok(mut w) = OFFERS_ANOTHER.write() { *w = v; } }
pub fn g_OPTION_WORDS() -> Py { ensure_init(); OPTION_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPTION_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = OPTION_WORDS.write() { *w = v; } }
pub fn g_OTHER_OPTION() -> Py { ensure_init(); OTHER_OPTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_OPTION(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_OPTION.write() { *w = v; } }
pub fn g_QUOTE_MARKS() -> Py { ensure_init(); QUOTE_MARKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUOTE_MARKS(v: Py) { ensure_init(); if let Ok(mut w) = QUOTE_MARKS.write() { *w = v; } }
pub fn g_RULED_OUT() -> Py { ensure_init(); RULED_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT(v: Py) { ensure_init(); if let Ok(mut w) = RULED_OUT.write() { *w = v; } }
pub fn g_SENTENCE() -> Py { ensure_init(); SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = SENTENCE.write() { *w = v; } }
pub fn g_SOFT_FILLER() -> Py { ensure_init(); SOFT_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = SOFT_FILLER.write() { *w = v; } }
pub fn g_STOCK() -> Py { ensure_init(); STOCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOCK(v: Py) { ensure_init(); if let Ok(mut w) = STOCK.write() { *w = v; } }
pub fn g_SWAP_WORDS() -> Py { ensure_init(); SWAP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWAP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SWAP_WORDS.write() { *w = v; } }
pub fn g_THEM() -> Py { ensure_init(); THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEM(v: Py) { ensure_init(); if let Ok(mut w) = THEM.write() { *w = v; } }
pub fn g_THE_MISSING_PART() -> Py { ensure_init(); THE_MISSING_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_MISSING_PART(v: Py) { ensure_init(); if let Ok(mut w) = THE_MISSING_PART.write() { *w = v; } }
pub fn g_URGENT_LINE() -> Py { ensure_init(); URGENT_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_URGENT_LINE(v: Py) { ensure_init(); if let Ok(mut w) = URGENT_LINE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("They ask for another option, and the answer they were given had more than one thing in it.\n\n\"my tomato plants have blight\" / \"what else can i do about it\" is not a new question: it is\nabout the answer just given, and it means \"something other than that, please\". Main answers it\nwith the same opening line again (\"From just now: Give them sun...\") or with a search hedge, and\nboth say the bot has nothing else - when the answer at the top of the screen usually has a second\nhalf in it that was read past.\n\nSo this module reads that turn: a line that asks for another option and names nothing of its own,\nthe answer the bot gave a turn ago, and the part of that answer the person has not been handed\nback yet. It replies with that part, said as what it is, rather than the whole paragraph again.\n\nThe rules, narrow on purpose:\n* the line has to name nothing (\"what else can i cook with mince\" is a question of its own and\n  never comes here);\n* the answer it leans on has to be a real answer - a hedge, a stock line, a question or an\n  emergency reply has nothing to hand over;\n* only the sentences after the one the bot leads with are offered, because the opening line is\n  what the person has already been given twice when a repeat is under way;\n* a sentence the bot has said in the last few turns is not offered again, so a second \"anything\n  else?\" moves on to the next one and then the module passes the turn back.\n\nThe subject is never invented: every word of the reply comes from the answer the person already\nhas.\n");
    s_OPTION_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    else other others another alternative alternatives different ideas idea options option\n    suggestion suggestions way ways thing things approach approaches trick tricks tip tips\n    works work working help helps helping useful\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTINUATION_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    go goes going on carry carries keep keeps continue continuing next more further then again\n    deeper say telling\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SWAP_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    swap swaps swapping swapped substitute substitutes substitution substituting\n    replacement replacements replace replacing stand-in\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK(bitor(&bitor(&bitor(&bitor(&crate::gen::memory::repeat::g_ASK(), &crate::gen::memory::repeat::g_FILLER()), &g_OPTION_WORDS()), &g_CONTINUATION_WORDS()), &g_SWAP_WORDS()));
    s_THE_MISSING_PART(call_builtin("frozenset", &[mcall(&py_str("\n    include includes including included bring bringing add adding say saying mention mentioning\n    cover covers covering know wear take expect prepare check forget forgetting forgot missed\n    miss left put\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MISSING_PART(stdlib_call("re", "compile", &[py_str("\\b(?:anything|what|which|is there|are there|something|all)\\b[^.?!]{0,40}?\\b(?:include|includes|including|bring|bringing|add|adding|say|saying|mention|mentioning|cover|covering|know|wear|take|expect|prepare|check|forget|forgetting|forgot|missed|miss|left|put)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOFT_FILLER(call_builtin("frozenset", &[mcall(&py_str("\n    defo deffo definitely probs prob probably tbh honestly basically totally literally\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK(bitor(&bitor(&g_ASK(), &g_THE_MISSING_PART()), &g_SOFT_FILLER()));
    s_OTHER_OPTION(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything|any|something|is there|are there|got any|have you got|else|other|others|another|different|alternative|alternatives)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_OPTION(stdlib_call("re", "compile", &[py_str("\\b(?:else|other|others|another|different|alternative|alternatives|ideas?|options?|suggestions?|ways?|things?|approaches?|swaps?|substitutes?|substitutions?|replacements?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARRY_ON(stdlib_call("re", "compile", &[py_str("^(?:(?:go|carry)\\s+on|keep\\s+going|continue|and(?:\\s+(?:then|after\\s+that|on))?|then\\s+what|after\\s+that|whats?(?:'s|\\s+is)?\\s+next|next|more|tell\\s+me\\s+more|say\\s+more|go\\s+deeper|and\\s+what\\s+else)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD_IN(stdlib_call("re", "compile", &[py_str("^(?:ok(?:ay)?|right|so|yeah|yes|alright|well|please|ta|thanks|cheers|and)[\\s,]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWER_MODULES(py_tuple(vec![py_str("answers"), py_str("chat"), py_str("knowledge")]));
    s_HEDGE(crate::gen::memory::repeat::g_HEDGE());
    s_STOCK(stdlib_call("re", "compile", &[py_str("\\b(?:thanks for telling me|tell me more|say more if it helps|i'?m listening|what would be most useful|anything you'?d like a hand with|noted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUOTE_MARKS(py_str("\"“”"));
    s_FRAME(stdlib_call("re", "compile", &[py_str("^(?:from just now|same as before|as i said|like i said|same answer|same answer i'?m afraid|it'?s what i said a moment ago|you'?ve got it|that'?s the one)\\s*[:\\-]\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADS(py_tuple(vec![py_str("Of that, the bit that matters most: {rest}"), py_str("If you only do one thing from that answer: {rest}"), py_str("The part of that I would act on first: {rest}"), py_str("Of all of that, this is the bit I would not skip: {rest}"), py_str("The bit of that answer worth doing first: {rest}")]));
    s_DO_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:if|so|then|try|use|keep|check|give|take|put|get|add|buy|swap|replace|clean|wash|rinse|leave|stand|wrap|store|feed|water|encourage|bleed|call|ask|book|avoid|stop|start|turn|run|set|hold|move|cover|cut|open|close|repeat|ring|email|text|worth|helps?|worthwhile)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COMMENT(stdlib_call("re", "compile", &[py_str("^\\s*(?:it'?s|it is|it was|that'?s|that is|that was|this is|this was|there'?s nothing|nothing|none of|both|all of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OFFERS_ANOTHER(stdlib_call("re", "compile", &[py_str("^\\s*(?:something else|anything else|another (?:option|one|idea|way|thing|idea for)|an? alternative|alternatively|otherwise|instead|or (?:else )?(?:try|make|use|have|just|simply|do|go with)|you could also|you can also|also try|you might also|there(?:'s| is| are)\\b(?![^.]{0,70}\\b(?:charge|fee|fees|cost|costs|price|deposit|catch|risk|downside|caveat|trick|trouble|problem)\\b)[^.]{0,70}\\b(?:as well|too)\\b)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_IDIOM(stdlib_call("re", "compile", &[py_str("\\b(?:from\\s+)?start\\s+to\\s+finish\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_ANOTHER(stdlib_call("re", "compile", &[py_str("^(?!\\s*(?:if|when|once|after|before|while|then|so|but|and|or|try|check|keep|give|take|put|use|get|add|buy|swap|replace|clean|wash|rinse|leave|stand|wrap|store|feed|call|ask|book|avoid|stop|start|turn|run|set|hold|move|cover|cut|open|close|repeat|ring|email|text|pick|pinch|encourage|bleed)\\b)\\w+(?: [\\w'\\u2019()-]+){0,3} (?:hates?|likes?|loves?|prefers?|needs?|wants?|is|are)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the)\\s+(?:son|daughter|kids?|child|children|toddler|baby|boy|girl|mum|mom|mother|dad|father|wife|husband|partner|boyfriend|girlfriend|brother|sister|friend|cat|dog|puppy|kitten|rabbit|hamster|horse|nan|nana|granny|grandma|grandad|grandpa)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEM(call_builtin("frozenset", &[mcall(&py_str("\n    he she him his her hers they them their themselves toddler baby boy girl son daughter kid\n    kids child children cat dog puppy kitten rabbit hamster horse\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FILING(stdlib_call("re", "compile", &[py_str("^\\s*(?:got it|noted|right|okay|fair enough|thanks|that'?s noted|i'?ll remember|understood|sure)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RULED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:don'?t|do not|doesn'?t|didn'?t|can'?t|couldn'?t|won'?t|wouldn'?t|never)\\s+(?:really\\s+|actually\\s+)?(?:like|love|want|need|use|eat|drink|fancy|trust|stand|have|bother with|go to)\\b|\\b(?:hates?|hated|avoid|avoids|allergic to|not a fan of|sick of|tired of|gave up|given up|stopped|quit|no time for|can'?t afford|too expensive|won'?t do)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(bitor(&bitor(&bitor(&crate::gen::memory::topics::g_STOPWORDS(), &crate::gen::memory::repeat::g_ASK()), &crate::gen::memory::repeat::g_FILLER()), &call_builtin("frozenset", &[mcall(&py_str("\n    dont do does doesnt didnt cant wont wouldnt never not anymore either too also any more\n    actually really just very much such thing things stuff one ones going again\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_A_CONDITION(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+)?if\\b|\\bunless\\b|\\bsuppose\\b|\\bin\\s+case\\b|\\bwhat\\s+happens\\s+(?:if|when)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOOK_BACK(Py::Int(3));
    s_MOST(Py::Int(220));
    s_LEAST(Py::Int(25));
    s_A_PROFESSIONAL(stdlib_call("re", "compile", &[py_str("\\b(?:see|speak to|talk to|ring|call|contact|ask|visit|book in with)\\s+(?:a|an|your|the)\\s+(?:gp|doctor|nurse|practice nurse|pharmacist|chemist|midwife|health visitor|physio(?:therapist)?|dentist|optician|consultant|specialist)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:if|when|whenever|unless|once|should|in case|and|so|then|but|or|also|really|actually|still|again|too|either|instead)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_URGENT_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:ring|call|phone)\\s+(?:111|999|911|112|nhs 24)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn names_nothing(args: &[Py]) -> Py {
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
    let mut asks_for_more: Py = Py::None;
    let mut left: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks for another option or the rest of the answer, naming nothing itself.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    asks_for_more = { let __b = call_builtin("bool", &[{ let __b = mcall(&g_CARRY_ON(), "match", &[mcall(&said.clone(), "strip", &[])]); if py_truthy(&__b) { __b } else { mcall(&g_CARRY_ON(), "match", &[mcall(&g_LEAD_IN(), "sub", &[py_str(""), mcall(&said.clone(), "strip", &[])])]) } }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = { let __b = call_builtin("bool", &[mcall(&g_OTHER_OPTION(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_AN_OPTION(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } }; if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_MISSING_PART(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&asks_for_more.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&crate::gen::memory::repeat::g_ABOUT_THE_BOT(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_CONDITION(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    words = crate::gen::memory::topics::words(&[said.clone()]);
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&crate::gen::memory::topics::g_STOPWORDS(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&w.clone(), "isdigit", &[]))) } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&left.clone())) } };
    Py::None
}
pub fn ruled_out_words(args: &[Py]) -> Py {
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
    let _ = py_str("The things a line says the person does not want, as stems, so \"gyms\" meets \"gym\".");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_RULED_OUT(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::words(&[text.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))) } } } }) {  __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } } py_set(__out) })() };
    Py::None
}
pub fn _names_one(args: &[Py]) -> Py {
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
    let mut ruled_out: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ruled_out") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ruled_out"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when a sentence of the old answer offers something the person has just ruled out.");
    if py_truthy(&Py::Bool(!py_truthy(&ruled_out.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[bitand(&ruled_out.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::words(&[part.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() })]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_turn_to_lean_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut back: Py = Py::None;
    let mut ruled_out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The last answer to lean on, and what the person ruled out between it and the ask.");
    memory = { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    ruled_out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), add(&g_LOOK_BACK(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    back = __item.clone();
    if py_truthy(&lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &back.clone())) {
    break;
    }
    turn = geti(&turns.clone(), &neg(&back.clone()));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_FILING(), "match", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::repeat::a_hand_back(&[said.clone(), memory.clone()]))) } } } }) {
    let __aug2 = bitor(&ruled_out.clone(), &ruled_out_words(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    ruled_out = __aug2;
    return py_tuple(vec![turn.clone(), ruled_out.clone(), sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &back.clone())]);
    }
    let __aug3 = bitor(&ruled_out.clone(), &ruled_out_words(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    ruled_out = __aug3;
    }
    return py_tuple(vec![Py::None, ruled_out.clone(), Py::None]);
    Py::None
}
pub fn split_sentences(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut char: Py = Py::None;
    let mut ended: Py = Py::None;
    let mut opened: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The sentences of a line, with a full stop inside a quotation not counted as an end.\n\n    An answer that hands the person a letter to send quotes it, and the letter has full stops in\n    it: cutting there leaves the rest of the answer beginning inside the quote, so the person is\n    handed `Thanks, [name].\" Follow it up in writing if nothing happens in 48 hours.` The quoted\n    passage is one sentence of the answer for this purpose; the full stop that ends it still ends\n    the sentence, at the closing quote.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0), Py::Bool(false), Py::Bool(false)]), 4, None);
    parts = __unpk1[0].clone();
    start = __unpk1[1].clone();
    opened = __unpk1[2].clone();
    ended = __unpk1[3].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    char = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_QUOTE_MARKS(), &char.clone()))) {
    if py_truthy(&opened.clone()) {
    opened = Py::Bool(false);
    if py_truthy(&{ let __b = ended.clone(); if !py_truthy(&__b) { __b } else { { let __b = lt(&add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { mcall(&geti(&said.clone(), &add(&at.clone(), &Py::Int(1))), "isspace", &[]) } } } }) {
    let _ = mcall(&parts.clone(), "append", &[slice_or_raise(&said.clone(), &start.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None)]);
    start = add(&at.clone(), &Py::Int(1));
    }
    ended = Py::Bool(false);
    } else {
    opened = Py::Bool(true);
    }
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_str(".!?"), &char.clone()))) {
    if py_truthy(&opened.clone()) {
    ended = Py::Bool(true);
    continue;
    }
    after = add(&at.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = lt(&after.clone(), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_QUOTE_MARKS(), &geti(&said.clone(), &after.clone()))) } }) {
    let __aug4 = add(&after.clone(), &Py::Int(1));
    after = __aug4;
    }
    if py_truthy(&{ let __b = ge(&after.clone(), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { mcall(&geti(&said.clone(), &after.clone()), "isspace", &[]) } }) {
    let _ = mcall(&parts.clone(), "append", &[slice_or_raise(&said.clone(), &start.clone(), &after.clone(), &Py::None)]);
    start = after.clone();
    }
    }
    }
    }
    let _ = mcall(&parts.clone(), "append", &[slice_or_raise(&said.clone(), &start.clone(), &Py::None, &Py::None)]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    Py::None
}
pub fn _sentences(args: &[Py]) -> Py {
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
    let _ = py_str("The answer as sentences, with the lead-in frame the bot puts on a repeat taken off.");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    said = mcall(&g_FRAME(), "sub", &[py_str(""), said.clone()]);
    return split_sentences(&[said.clone()]);
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::None };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("True when the bot has already said this sentence in a turn before the answer at hand.\n\n    The answer being handed back is the last turn, and every sentence of it is in that turn, so it\n    is left out here: what is looked for is the same sentence said *again* later on.\n    ");
    wanted = mcall(&slice_or_raise(&mcall(&said.clone(), "strip", &[]), &Py::None, &Py::Int(50), &Py::None), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_is(&before.clone(), &Py::None))) {
    before = sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &call_builtin("max", &[Py::Int(0), add(&sub(&before.clone(), &g_LOOK_BACK()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), &before.clone(), &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &wanted.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _cut(args: &[Py]) -> Py {
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
    let _ = py_str("A sentence short enough to say, cut at a comma or semicolon when it is not.\n\n    The cut is the shared one (`compose.whole.a_whole_cut`): a sentence cut at the last comma can\n    be cut between a step and the condition it is for, and the hand-back then ends on \"If it keeps\n    happening.\" - the half of the sentence that says nothing. Where the condition would be left\n    hanging, the helper keeps the doing or steps back in front of the condition instead.\n    ");
    said = mcall(&said.clone(), "strip", &[]);
    if py_truthy(&le(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST())) {
    return said.clone();
    }
    return add(&mcall(&crate::gen::compose::whole::a_whole_cut(&[said.clone(), g_MOST()]), "rstrip", &[py_str(" ,;:.")]), &py_str("."));
    Py::None
}
pub fn _a_caveat_about_getting_help(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let _ = py_str("True when the sentence is when to get help rather than something to try.\n\n    The test is what is left when the referral is taken out. \"Drink plenty of water, take\n    paracetamol for the sting, and see a GP or pharmacist\" still tells them to do two things and\n    is handed over as one; a sentence whose only doing is the referral tells them nothing they can\n    start on, so it is the warning at the end of the answer however it is worded.\n    ");
    if py_truthy(&mcall(&crate::gen::memory::topics::g_RED_FLAG(), "match", &[part.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_PROFESSIONAL(), "search", &[part.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_URGENT_LINE(), "search", &[part.clone()]) } }))) {
    return Py::Bool(false);
    }
    left = mcall(&g_A_PROFESSIONAL(), "sub", &[py_str(" "), mcall(&g_URGENT_LINE(), "sub", &[py_str(" "), part.clone()])]);
    left = mcall(&g_FRAME_WORDS(), "sub", &[py_str(" "), left.clone()]);
    return Py::Bool(!py_truthy(&is_a_step(&[left.clone()])));
    Py::None
}
pub fn is_a_step(args: &[Py]) -> Py {
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
    let _ = py_str("True when the sentence tells the person to do something, not just how long it takes.");
    return call_builtin("bool", &[mcall(&g_DO_THIS(), "search", &[mcall(&g_LENGTH_IDIOM(), "sub", &[py_str(" "), part.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_rest(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut ruled_out: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ruled_out") { v.clone() }
    else { py_tuple(vec![]) };
    let mut at: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { Py::None };
    let mut _is_the_rest: Py = Py::None;
    let mut after: Py = Py::None;
    let mut given: Py = Py::None;
    let mut here: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The part of the answer the person has not been handed back yet, or \"\" when there is none.");
    parts = _sentences(&[said.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    given = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    part = __unpk2[1].clone();
    if py_truthy(&{ let __b = is_a_step(&[part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_caveat_about_getting_help(&[part.clone()]))) } }) {
    given = at.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&given.clone(), &Py::None))) {
    return py_str("");
    }
pub fn __nested__is_the_rest_2(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _is_the_rest: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut after: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut at: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut given: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut here: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut part: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut parts: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut ruled_out: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut part: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "part").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut before: Py = __pos.get(11).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "before").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut offers: Py = Py::None;
    offers = call_builtin("bool", &[mcall(&g_OFFERS_ANOTHER(), "match", &[part.clone()])]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&{ let __b = lt(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LEAST()); if py_truthy(&__b) { __b } else { { let __b = _a_caveat_about_getting_help(&[part.clone()]); if py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(!py_truthy(&is_a_step(&[part.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&offers.clone())) } }; if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_COMMENT(), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_ABOUT_ANOTHER(), "match", &[part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&offers.clone())) } }; if py_truthy(&__b) { __b } else { { let __b = _names_one(&[part.clone(), ruled_out.clone()]); if py_truthy(&__b) { __b } else { _said_before(&[memory.clone(), part.clone(), before.clone()]) } } } } } } } } } } } }));
    Py::None
}
    _is_the_rest = py_func("_is_the_rest", move |args: &[Py]| { let mut full = vec![_is_the_rest.clone(), after.clone(), at.clone(), given.clone(), here.clone(), memory.clone(), part.clone(), parts.clone(), ruled_out.clone(), said.clone()]; full.extend_from_slice(args); __nested__is_the_rest_2(&full) });
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[add(&given.clone(), &Py::Int(1)), call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    here = __item.clone();
    part = geti(&parts.clone(), &here.clone());
    if py_truthy(&Py::Bool(!py_truthy(&py_call(&_is_the_rest, &[part.clone(), at.clone()])))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_QUOTE_MARKS(), &geti(&part.clone(), &neg(&Py::Int(1)))))) {
    let __iter5 = py_iter_or_raise(&call_builtin("range", &[add(&here.clone(), &Py::Int(1)), call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    after = __item.clone();
    if py_truthy(&py_call(&_is_the_rest, &[geti(&parts.clone(), &after.clone()), at.clone()])) {
    part = add(&add(&part.clone(), &py_str(" ")), &geti(&parts.clone(), &after.clone()));
    break;
    }
    }
    }
    if py_truthy(&{ let __b = mcall(&g_OFFERS_ANOTHER(), "match", &[part.clone()]); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST()) } }) {
    return add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return _cut(&[part.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn other_option(args: &[Py]) -> Py {
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
    let mut asked_about: Py = Py::None;
    let mut at: Py = Py::None;
    let mut module: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut ruled_out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The second half of the answer they just asked for more of, or None when it is not that.");
    if py_truthy(&Py::Bool(!py_truthy(&names_nothing(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&crate::gen::memory::named::asks_a_part_of_the_trip(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_turn_to_lean_on(&[turns.clone(), memory.clone()]), 3, None);
    turn = __unpk1[0].clone();
    ruled_out = __unpk1[1].clone();
    at = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_is(&turn.clone(), &Py::None))) {
    return Py::None;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    module = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ANSWER_MODULES(), &module.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HEDGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_STOCK(), "search", &[said.clone()]) } } } } } }) {
    return Py::None;
    }
    rest = _the_rest(&[said.clone(), memory.clone(), ruled_out.clone(), at.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    asked_about = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_A_PERSON(), "search", &[asked_about.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&g_THEM(), &call_builtin("set", &[crate::gen::memory::topics::words(&[rest.clone()])]).unwrap_or_else(|e| exc::raise(e))))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_OFFERS_ANOTHER(), "match", &[rest.clone()])) {
    return rest.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("rest".to_string(), rest.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_LEADS(), memory.clone(), text.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.another".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_ANOTHER" => Some(g_ABOUT_ANOTHER()),
            "ANSWER_MODULES" => Some(g_ANSWER_MODULES()),
            "AN_OPTION" => Some(g_AN_OPTION()),
            "ASK" => Some(g_ASK()),
            "A_COMMENT" => Some(g_A_COMMENT()),
            "A_CONDITION" => Some(g_A_CONDITION()),
            "A_MISSING_PART" => Some(g_A_MISSING_PART()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_PROFESSIONAL" => Some(g_A_PROFESSIONAL()),
            "CARRY_ON" => Some(g_CARRY_ON()),
            "CONTINUATION_WORDS" => Some(g_CONTINUATION_WORDS()),
            "DO_THIS" => Some(g_DO_THIS()),
            "FILING" => Some(g_FILING()),
            "FRAME" => Some(g_FRAME()),
            "FRAME_WORDS" => Some(g_FRAME_WORDS()),
            "HEDGE" => Some(g_HEDGE()),
            "LEADS" => Some(g_LEADS()),
            "LEAD_IN" => Some(g_LEAD_IN()),
            "LEAST" => Some(g_LEAST()),
            "LENGTH_IDIOM" => Some(g_LENGTH_IDIOM()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "MOST" => Some(g_MOST()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "OFFERS_ANOTHER" => Some(g_OFFERS_ANOTHER()),
            "OPTION_WORDS" => Some(g_OPTION_WORDS()),
            "OTHER_OPTION" => Some(g_OTHER_OPTION()),
            "QUOTE_MARKS" => Some(g_QUOTE_MARKS()),
            "RULED_OUT" => Some(g_RULED_OUT()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SOFT_FILLER" => Some(g_SOFT_FILLER()),
            "STOCK" => Some(g_STOCK()),
            "SWAP_WORDS" => Some(g_SWAP_WORDS()),
            "THEM" => Some(g_THEM()),
            "THE_MISSING_PART" => Some(g_THE_MISSING_PART()),
            "URGENT_LINE" => Some(g_URGENT_LINE()),
            "_a_caveat_about_getting_help" => Some(py_func("_a_caveat_about_getting_help", |args: &[Py]| _a_caveat_about_getting_help(args))),
            "_cut" => Some(py_func("_cut", |args: &[Py]| _cut(args))),
            "_names_one" => Some(py_func("_names_one", |args: &[Py]| _names_one(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "_sentences" => Some(py_func("_sentences", |args: &[Py]| _sentences(args))),
            "_the_rest" => Some(py_func("_the_rest", |args: &[Py]| _the_rest(args))),
            "_the_turn_to_lean_on" => Some(py_func("_the_turn_to_lean_on", |args: &[Py]| _the_turn_to_lean_on(args))),
            "is_a_step" => Some(py_func("is_a_step", |args: &[Py]| is_a_step(args))),
            "names_nothing" => Some(py_func("names_nothing", |args: &[Py]| names_nothing(args))),
            "other_option" => Some(py_func("other_option", |args: &[Py]| other_option(args))),
            "ruled_out_words" => Some(py_func("ruled_out_words", |args: &[Py]| ruled_out_words(args))),
            "split_sentences" => Some(py_func("split_sentences", |args: &[Py]| split_sentences(args))),
            _ => None,
        }),
        set: None,
    }))
}
