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


thread_local! { static G_ABOUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANGER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANGER_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPRAISAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEAR_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUILT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUILT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOW_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHAME_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_IS_WRONG_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRONG_WITH_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_WORDS() -> Py { ensure_init(); G_ABOUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_WORDS(v: Py) { ensure_init(); G_ABOUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANGER() -> Py { ensure_init(); G_ANGER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANGER(v: Py) { ensure_init(); G_ANGER.with(|c| *c.borrow_mut() = v); }
pub fn g_ANGER_LINES() -> Py { ensure_init(); G_ANGER_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANGER_LINES(v: Py) { ensure_init(); G_ANGER_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_APPRAISAL() -> Py { ensure_init(); G_APPRAISAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPRAISAL(v: Py) { ensure_init(); G_APPRAISAL.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FEAR() -> Py { ensure_init(); G_FEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEAR(v: Py) { ensure_init(); G_FEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_FEAR_LINES() -> Py { ensure_init(); G_FEAR_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEAR_LINES(v: Py) { ensure_init(); G_FEAR_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_GUILT() -> Py { ensure_init(); G_GUILT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUILT(v: Py) { ensure_init(); G_GUILT.with(|c| *c.borrow_mut() = v); }
pub fn g_GUILT_LINES() -> Py { ensure_init(); G_GUILT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUILT_LINES(v: Py) { ensure_init(); G_GUILT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LOW() -> Py { ensure_init(); G_LOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOW(v: Py) { ensure_init(); G_LOW.with(|c| *c.borrow_mut() = v); }
pub fn g_LOW_LINES() -> Py { ensure_init(); G_LOW_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOW_LINES(v: Py) { ensure_init(); G_LOW_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_POOLS() -> Py { ensure_init(); G_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOLS(v: Py) { ensure_init(); G_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF() -> Py { ensure_init(); G_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF(v: Py) { ensure_init(); G_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_SHAME() -> Py { ensure_init(); G_SHAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHAME(v: Py) { ensure_init(); G_SHAME.with(|c| *c.borrow_mut() = v); }
pub fn g_SHAME_LINES() -> Py { ensure_init(); G_SHAME_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHAME_LINES(v: Py) { ensure_init(); G_SHAME_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASK() -> Py { ensure_init(); G_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASK(v: Py) { ensure_init(); G_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_IS_WRONG_LINES() -> Py { ensure_init(); G_WHAT_IS_WRONG_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_IS_WRONG_LINES(v: Py) { ensure_init(); G_WHAT_IS_WRONG_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WRONG_WITH_ME() -> Py { ensure_init(); G_WRONG_WITH_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRONG_WITH_ME(v: Py) { ensure_init(); G_WRONG_WITH_ME.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("An ask about the asker's own feeling or conduct is not a look-up.\n\nThe search hedge is written for the one gap it fits: a fact that lives somewhere - tonight's\ntrading hours, the bus time, a branch's phone number. Put in front of \"what should i do about the\nguilt\" it answers a question nobody asked: there is nothing to search for, and sending somebody to\nread about themselves is the one reply that cannot help. Main does this for seven of the eight\nshapes below:\n\n    what should i do about the guilt     -> \"That's beyond what I know well... Searching for it\"\n    why do i feel like this              -> the same\n    what can i do about the shame        -> the same\n    how do i forgive myself              -> the same\n    how do i stop the panic              -> the same\n    what is wrong with me                -> the same\n\nThe rule, and the reason this file is written as one reader rather than six, is about the *shape*\nof the line and not its wording: **when the person's own feeling is the whole subject of the ask,\nno fact is being asked for**. So the reading is two halves, and a line has to have both:\n\n- an ask frame - \"how do i <verb>\", \"what can i do about\", \"why do i <feel>\", \"what is wrong with\n  me\";\n- and, after the ask is taken out, nothing left in the line that names a thing. \"how do i stop the\n  panic\" is the feeling; \"how do i stop the smoke alarm\", \"how do i get rid of the moths\", \"how do\n  i stop the fan humming\" are things, and every one of them keeps the answer it had. This is the\n  same fence `bot/compose/loss.py` uses for the grief ask (`_a_question_about_the_feeling`), and\n  it is what makes the reader safe against new wording: a word it does not know makes the reader\n  stand down rather than guess.\n\nWhat it says, when it does answer: the same three things STYLE.md's \"Not knowing, honestly\" asks\nfor, in the order a friend says them - the honest limit in a few words (nobody can tell you why\nfrom the outside; that is not a fact anyone can check), then the part that is true of this kind of\nthing and is worth having (what guilt is for, what shame is heavier than, what helps while panic is\nhappening, the question underneath anger), then one question that opens the next turn. Nothing\nhere is a fact about the person: the pools only say what these feelings are like in general.\n\nA feeling that has a knowledge card keeps the card: this reader is asked at the very end, after\nevery pool, every advisor and both the bot's-own-answer and where-is-it-kept readers. Everything it\ncan see has already been given the turn.\n");
    s_GUILT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("guilt"), py_str("guilty"), py_str("regret"), py_str("remorse"), py_str("bad")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SHAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ashamed"), py_str("shame"), py_str("shamed"), py_str("embarrassed"), py_str("humiliated")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FEAR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("panic"), py_str("panicking"), py_str("panicky"), py_str("anxious"), py_str("anxiety"), py_str("scared"), py_str("afraid"), py_str("fear"), py_str("frightened"), py_str("terrified"), py_str("dread"), py_str("worried"), py_str("worry"), py_str("nerves"), py_str("nervous"), py_str("stress"), py_str("stressed")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANGER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("angry"), py_str("anger"), py_str("rage"), py_str("furious"), py_str("resentful"), py_str("resentment"), py_str("bitter"), py_str("irritated"), py_str("annoyed"), py_str("cross")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("sad"), py_str("sadness"), py_str("low"), py_str("miserable"), py_str("down"), py_str("empty"), py_str("hopeless"), py_str("lonely"), py_str("loneliness"), py_str("numb"), py_str("flat"), py_str("tearful"), py_str("upset")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SELF(op_bitor(&op_bitor(&op_bitor(&op_bitor(&g_GUILT(), &g_SHAME()), &g_FEAR()), &g_ANGER()), &g_LOW()));
    s_THE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:do|can|could|should|would)\\s+i\\s+(?:stop|stops|deal\\s+with|cope\\s+with|get\\s+rid\\s+of|handle|manage|forgive|blame|get\\s+over|move\\s+on\\s+from|let\\s+go\\s+of|live\\s+with|make\\s+(?:it|this|that|things)\\s+up|fix|change|shake|beat|calm|settle)\\b|\\bhow\\s+(?:do|can|could|should|would)\\s+i\\s+(?:stop|keep|start)\\s+(?:feeling|being|getting)\\b|\\bwhat\\s+(?:can|could|should|do)\\s+i\\s+do\\s+(?:about|with|for)\\b|\\bwhat\\s+(?:can|could|should)\\s+i\\s+do\\b(?!\\s+(?:about\\s+)?(?:a|an|the|my)\\b)|\\bwhat\\s+do\\s+i\\s+do\\s+(?:now|next|about)\\b|\\bwhy\\s+(?:do|am|did|does|is)\\s+(?:i|it)\\b|\\bwhat\\s+(?:is|was)\\s+(?:wrong|up)\\s+with\\s+me\\b|\\bi\\s+(?:just\\s+|really\\s+|still\\s+|keep\\s+)?(?:feel|feeling)\\b(?!\\s+(?:like|as\\s+if)\\b)|\\bi\\s+(?:[a-z]+\\s+){0,2}feel\\s+like\\s+(?:a\\s+|an\\s+|such\\s+a\\s+|a\\s+total\\s+|a\\s+complete\\s+|an\\s+absolute\\s+){1,2}(?:failure|mess|fraud|idiot|burden|loser|wreck|joke|nothing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WRONG_WITH_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat(?:'s|\\s+is|\\s+was)?\\s+(?:wrong|up|the\\s+matter|the\\s+problem)\\s+with\\s+me\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_WORDS(op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nhow do does did can could should would i me my mine myself im i'm am is was are be been being\nwhat why when now next about with for of to in on at this that these those it its it's\nfeel feels feeling felt like as if so very really just still always never ever much more most\nsome any the a an and but or then there here wrong up matter problem\nstop stops stopped deal cope get rid handle manage forgive get over move on from let go live\nmake made fix change shake beat calm settle keep start being getting bad worse better again\nway ways blame blaming forgive forgiving less more\nanymore today lately always sometimes ok okay well please help tell know thing things something\nanything nothing lot bit\nfailure mess fraud idiot burden loser wreck joke useless worthless stupid pathetic hopeless\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SELF()));
    s_ABOUT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("about"), py_str("over"), py_str("regarding"), py_str("concerning"), py_str("around"), py_str("because")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPRAISAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("failure"), py_str("mess"), py_str("fraud"), py_str("idiot"), py_str("burden"), py_str("loser"), py_str("wreck"), py_str("joke"), py_str("useless"), py_str("worthless"), py_str("stupid"), py_str("pathetic"), py_str("hopeless")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GUILT_LINES(py_tuple(vec![py_str("There is no fact I can look up here - nobody can tell you whether you ought to feel guilty, and the feeling itself is a poor guide. What helps is smaller and more concrete: the one thing you would have done differently, and whether any part of it can still be said or put right. What was it?"), py_str("Guilt is about something you did, and it usually lifts when the thing is dealt with rather than when you argue with it. I can't tell you whether you should still feel it - nobody can check that - but I can help you find the part that is still open. What happened?"), py_str("I can't settle whether you deserve to feel bad about it, and a search would only find somebody else's opinion. The useful question is what you would want to have done instead, and whether that is still possible in some smaller form. What would it be?")]));
    s_SHAME_LINES(py_tuple(vec![py_str("Shame is heavier than guilt - guilt is about a thing you did, shame is about who you are - and there is nothing to look up that settles it. What shrinks it is being said out loud to one person who won't flinch, which is the step most people are given. Is there anyone you could say it to?"), py_str("Nothing on a screen will make this better, and I would be making it up if I said otherwise. The thing that helps with shame is specific and hard: telling one person, in your own words, and letting them still be there afterwards. Who could that be?")]));
    s_FEAR_LINES(py_tuple(vec![py_str("I can't tell you why it comes for you - that needs somebody who can ask you questions. What does help while it is happening is the oldest trick there is: breathe out longer than you breathe in, and stay where you are until it passes, because it does pass. Do you know what sets it off?"), py_str("There's nothing to search that will tell you why this is happening to you, and anyone who says they can do it from a webpage is guessing. Two things worth having: while it is happening, slow the out-breath and don't fight it; if it keeps coming back, tell your GP, because that is a conversation worth having. How often is it happening?"), py_str("I can't check what is going on inside you, so I won't pretend to. What I can say is the usual first step for this: keep it small while it is happening - long breaths out, feet on the floor, one thing at a time - and then write down what came just before it, because the pattern is usually the useful part. What came before the last one?")]));
    s_ANGER_LINES(py_tuple(vec![py_str("Anger usually has something underneath it - hurt, or a line that someone crossed - and there is no fact I can look up that gets to it. The question worth asking is what it is protecting. What happened just before you felt it?"), py_str("I can't tell you whether you are right to be this angry; that depends on things only you know. What I can say is that anger is usually the second feeling, and the first one is what to look for. What was it that got to you?")]));
    s_LOW_LINES(py_tuple(vec![py_str("I can't tell from out here why you feel like this, and nothing I could look up would know either. What usually helps is naming the part that is heaviest and when it started - a GP or a friend would ask the same first question. When did it start?"), py_str("There's no fact to find on this one, and I'd be guessing if I gave you a reason. The part worth having is small and concrete: what has changed lately - sleep, work, somebody, your health - because a low patch usually has one of those behind it. Which of them is nearest?"), py_str("Why you feel like this isn't something anyone can check from outside, me least of all. But it is worth saying out loud: if it has been like this for more than a couple of weeks, that is the point where a GP is the right person rather than a website. How long has it been?")]));
    s_WHAT_IS_WRONG_LINES(py_tuple(vec![py_str("Nothing about you is a fact I can look up, and 'what's wrong with me' is usually a question about one part of your life rather than all of you. Which part is it?"), py_str("I can't tell you that, and I'd be inventing something if I tried. What I can do is take it apart with you - there is usually one thing that is actually wrong, and it is smaller and more fixable than the question makes it sound. Where would you start?")]));
    s_POOLS(py_tuple(vec![py_tuple(vec![g_GUILT(), g_GUILT_LINES()]), py_tuple(vec![g_SHAME(), g_SHAME_LINES()]), py_tuple(vec![g_FEAR(), g_FEAR_LINES()]), py_tuple(vec![g_ANGER(), g_ANGER_LINES()]), py_tuple(vec![g_LOW(), g_LOW_LINES()])]));
}
pub fn _words(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _named_thing(args: &[Py]) -> Py {
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
    let mut feeling_seen: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A content word in the line that is not the ask and not the feeling.\n\n    One is enough: \"how do i stop the panic\" has none and is the feeling; \"how do i stop the panic\n    alarm\" has \"alarm\" and is a thing. A word nobody put in ASK_WORDS is a subject until proven\n    otherwise, so a new shape stands down rather than answer as if it were a feeling.\n    ");
    feeling_seen = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SELF(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_APPRAISAL(), &word.clone())) } }) {
    feeling_seen = Py::Bool(true);
    continue;
    }
    if py_truthy(&{ let __b = feeling_seen.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ABOUT_WORDS(), &word.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_ASK_WORDS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("me"), py_str("my"), py_str("im")]), &word.clone())) } }) {
    continue;
    }
    return word.clone();
    }
    return Py::None;
    Py::None
}
pub fn _family(args: &[Py]) -> Py {
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
    let mut feelings: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The pool for the feeling in the line, or the what's-wrong-with-me one.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WRONG_WITH_ME(), "search", args))(&__args) }) {
    return g_WHAT_IS_WRONG_LINES();
    }
    let __iter1 = py_iter_or_raise(&g_POOLS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    feelings = __unpk2[0].clone();
    lines = __unpk2[1].clone();
    if py_truthy(&op_bitand(&words.clone(), &feelings.clone())) {
    return lines.clone();
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:forgive|blame)\\s+(?:myself|me)\\b")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g_GUILT_LINES();
    }
    if py_truthy(&op_bitand(&words.clone(), &g_APPRAISAL())) {
    return g_SHAME_LINES();
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("feel"), py_str("feels"), py_str("feeling"), py_str("felt")]))) {
    return g_LOW_LINES();
    }
    return Py::None;
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut lines: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A reply for an ask about the asker's own feeling, or None.\n\n    Asked at the end of `_unknown`, so a line with a card, an entry, an advisor or a place keeps\n    all of them: this only ever stands where the search hedge would have been.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WRONG_WITH_ME(), "search", args))(&__args) } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_thing(args))(&__args) }) {
    return Py::None;
    }
    lines = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _family(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return Py::None;
    }
    let mut random: Py = stdlib_module("random");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&random.clone(), "choice", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.own_ask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_WORDS" => Some(g_ABOUT_WORDS()),
            "ANGER" => Some(g_ANGER()),
            "ANGER_LINES" => Some(g_ANGER_LINES()),
            "APPRAISAL" => Some(g_APPRAISAL()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "FEAR" => Some(g_FEAR()),
            "FEAR_LINES" => Some(g_FEAR_LINES()),
            "GUILT" => Some(g_GUILT()),
            "GUILT_LINES" => Some(g_GUILT_LINES()),
            "LOW" => Some(g_LOW()),
            "LOW_LINES" => Some(g_LOW_LINES()),
            "POOLS" => Some(g_POOLS()),
            "SELF" => Some(g_SELF()),
            "SHAME" => Some(g_SHAME()),
            "SHAME_LINES" => Some(g_SHAME_LINES()),
            "THE_ASK" => Some(g_THE_ASK()),
            "WHAT_IS_WRONG_LINES" => Some(g_WHAT_IS_WRONG_LINES()),
            "WRONG_WITH_ME" => Some(g_WRONG_WITH_ME()),
            "_family" => Some(py_func("_family", |args: &[Py]| _family(args))),
            "_named_thing" => Some(py_func("_named_thing", |args: &[Py]| _named_thing(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_WORDS" => s_ABOUT_WORDS(v),
            "ANGER" => s_ANGER(v),
            "ANGER_LINES" => s_ANGER_LINES(v),
            "APPRAISAL" => s_APPRAISAL(v),
            "ASK_WORDS" => s_ASK_WORDS(v),
            "FEAR" => s_FEAR(v),
            "FEAR_LINES" => s_FEAR_LINES(v),
            "GUILT" => s_GUILT(v),
            "GUILT_LINES" => s_GUILT_LINES(v),
            "LOW" => s_LOW(v),
            "LOW_LINES" => s_LOW_LINES(v),
            "POOLS" => s_POOLS(v),
            "SELF" => s_SELF(v),
            "SHAME" => s_SHAME(v),
            "SHAME_LINES" => s_SHAME_LINES(v),
            "THE_ASK" => s_THE_ASK(v),
            "WHAT_IS_WRONG_LINES" => s_WHAT_IS_WRONG_LINES(v),
            "WRONG_WITH_ME" => s_WRONG_WITH_ME(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.own_ask", module_obj);
    pyrt::xrefs::reg_fn("compose.own_ask", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.own_ask", "_named_thing", py_func("_named_thing", |args: &[Py]| _named_thing(args)));
    pyrt::xrefs::reg_fn("compose.own_ask", "_family", py_func("_family", |args: &[Py]| _family(args)));
    pyrt::xrefs::reg_fn("compose.own_ask", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_access("compose.own_ask", "ABOUT_WORDS", g_ABOUT_WORDS, s_ABOUT_WORDS);
    pyrt::xrefs::reg_access("compose.own_ask", "ANGER", g_ANGER, s_ANGER);
    pyrt::xrefs::reg_access("compose.own_ask", "ANGER_LINES", g_ANGER_LINES, s_ANGER_LINES);
    pyrt::xrefs::reg_access("compose.own_ask", "APPRAISAL", g_APPRAISAL, s_APPRAISAL);
    pyrt::xrefs::reg_access("compose.own_ask", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("compose.own_ask", "FEAR", g_FEAR, s_FEAR);
    pyrt::xrefs::reg_access("compose.own_ask", "FEAR_LINES", g_FEAR_LINES, s_FEAR_LINES);
    pyrt::xrefs::reg_access("compose.own_ask", "GUILT", g_GUILT, s_GUILT);
    pyrt::xrefs::reg_access("compose.own_ask", "GUILT_LINES", g_GUILT_LINES, s_GUILT_LINES);
    pyrt::xrefs::reg_access("compose.own_ask", "LOW", g_LOW, s_LOW);
    pyrt::xrefs::reg_access("compose.own_ask", "LOW_LINES", g_LOW_LINES, s_LOW_LINES);
    pyrt::xrefs::reg_access("compose.own_ask", "POOLS", g_POOLS, s_POOLS);
    pyrt::xrefs::reg_access("compose.own_ask", "SELF", g_SELF, s_SELF);
    pyrt::xrefs::reg_access("compose.own_ask", "SHAME", g_SHAME, s_SHAME);
    pyrt::xrefs::reg_access("compose.own_ask", "SHAME_LINES", g_SHAME_LINES, s_SHAME_LINES);
    pyrt::xrefs::reg_access("compose.own_ask", "THE_ASK", g_THE_ASK, s_THE_ASK);
    pyrt::xrefs::reg_access("compose.own_ask", "WHAT_IS_WRONG_LINES", g_WHAT_IS_WRONG_LINES, s_WHAT_IS_WRONG_LINES);
    pyrt::xrefs::reg_access("compose.own_ask", "WRONG_WITH_ME", g_WRONG_WITH_ME, s_WRONG_WITH_ME);
}
