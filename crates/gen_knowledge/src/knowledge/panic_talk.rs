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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEHIND_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVING_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ADVICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANXIETY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ATTACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ATTACK_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEHIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEHIND_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DIAGNOSIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HABIT_MARKERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HABIT_ONCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HABIT_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PATTERN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEIR_OWN_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORRY_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_BEHIND_YOU() -> Py { ensure_init(); G_BEHIND_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEHIND_YOU(v: Py) { ensure_init(); G_BEHIND_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVING_WITH() -> Py { ensure_init(); G_LIVING_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVING_WITH(v: Py) { ensure_init(); G_LIVING_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NOSE() -> Py { ensure_init(); G_NOT_A_NOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NOSE(v: Py) { ensure_init(); G_NOT_A_NOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); G_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); G_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__ADVICE_ASK() -> Py { ensure_init(); G__ADVICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ADVICE_ASK(v: Py) { ensure_init(); G__ADVICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ANXIETY() -> Py { ensure_init(); G__ANXIETY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANXIETY(v: Py) { ensure_init(); G__ANXIETY.with(|c| *c.borrow_mut() = v); }
pub fn g__ATTACK() -> Py { ensure_init(); G__ATTACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ATTACK(v: Py) { ensure_init(); G__ATTACK.with(|c| *c.borrow_mut() = v); }
pub fn g__ATTACK_NAMES() -> Py { ensure_init(); G__ATTACK_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ATTACK_NAMES(v: Py) { ensure_init(); G__ATTACK_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__BEHIND() -> Py { ensure_init(); G__BEHIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEHIND(v: Py) { ensure_init(); G__BEHIND.with(|c| *c.borrow_mut() = v); }
pub fn g__BEHIND_THEM() -> Py { ensure_init(); G__BEHIND_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEHIND_THEM(v: Py) { ensure_init(); G__BEHIND_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g__DIAGNOSIS() -> Py { ensure_init(); G__DIAGNOSIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DIAGNOSIS(v: Py) { ensure_init(); G__DIAGNOSIS.with(|c| *c.borrow_mut() = v); }
pub fn g__HABIT() -> Py { ensure_init(); G__HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HABIT(v: Py) { ensure_init(); G__HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g__HABIT_MARKERS() -> Py { ensure_init(); G__HABIT_MARKERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HABIT_MARKERS(v: Py) { ensure_init(); G__HABIT_MARKERS.with(|c| *c.borrow_mut() = v); }
pub fn g__HABIT_ONCE() -> Py { ensure_init(); G__HABIT_ONCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HABIT_ONCE(v: Py) { ensure_init(); G__HABIT_ONCE.with(|c| *c.borrow_mut() = v); }
pub fn g__HABIT_VERBS() -> Py { ensure_init(); G__HABIT_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HABIT_VERBS(v: Py) { ensure_init(); G__HABIT_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__MINE() -> Py { ensure_init(); G__MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINE(v: Py) { ensure_init(); G__MINE.with(|c| *c.borrow_mut() = v); }
pub fn g__PATTERN() -> Py { ensure_init(); G__PATTERN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PATTERN(v: Py) { ensure_init(); G__PATTERN.with(|c| *c.borrow_mut() = v); }
pub fn g__THEIR_OWN_PAST() -> Py { ensure_init(); G__THEIR_OWN_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEIR_OWN_PAST(v: Py) { ensure_init(); G__THEIR_OWN_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g__THEM() -> Py { ensure_init(); G__THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEM(v: Py) { ensure_init(); G__THEM.with(|c| *c.borrow_mut() = v); }
pub fn g__WORRY_ABOUT() -> Py { ensure_init(); G__WORRY_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORRY_ABOUT(v: Py) { ensure_init(); G__WORRY_ABOUT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Panic: whose attack it is, and whether it is happening now.\n\nThe card for a panic attack is written for the person having one this minute - it says the attack\ncannot hurt *you* and to ride it out. Lines the card did not fit were answered with it anyway:\nsomebody else's attack (\"how do i help someone having a panic attack\") and one that is over\n(\"i used to have panic attacks every day\"). The person in the room and the tense are the whole of\nthose two questions, so they are two rows here, beside the card rather than inside it: a row that\nkeys what it answers is easier to keep from taking the others.\n\nA third row is one step further out again. \"my husband has panic attacks\" is a habit rather than a\nminute, so the minute-by-minute card is an instruction for something that is not happening - but\nthe ordinary answer it was left with (\"Got it. Is there anything you'd like a hand with?\") helps\nnobody who has just said what the trouble is either. A habit gets the week-to-week answer instead:\nask what helps, do not build a life around avoiding the places, and it is treatable.\n\nThe answers lane owns this domain.\n\nThe domain declares one topic. `feelings` is chat's job (`test_answers_contract` says a knowledge\nanswer must not be preferred on it), and both rows here are about a body in a bad minute rather\nthan about a feeling, so `health` is the whole declaration.\n");
    s_NOT_A_NOSE(crate::knowledge::lookup::g_NOT_A_NOSE());
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    s_SOMEONE_ELSE(py_str("Stay with them and say it will pass, because it does: an attack peaks in about ten minutes. Sit them down, loosen anything tight at the neck, and breathe out loud with them - out for longer than in, so they have something to follow rather than an instruction. Do not argue with what they are frightened of and do not leave them alone to fetch anything. If they have never had one before, or there is chest pain with it, get medical advice rather than assuming it is panic."));
    s_BEHIND_YOU(py_str("Then it is behind you, and getting through one is the thing to be pleased about rather than something to keep watch on. If one starts again, the out-breath is what helps - out for longer than in - and leaving the place it started rather than fighting it. If they come back often enough to shape your week, ask your GP about CBT: it is the treatment with the best evidence for panic."));
    s__ATTACK(py_tuple(vec![py_str("panic attack"), py_str("panic attacks"), py_str("panicking"), py_str("panicky"), py_str("panics"), py_str("panicked"), py_str("cant breathe"), py_str("heart racing"), py_str("hyperventilating")]));
    s__ANXIETY(py_tuple(vec![py_str("anxiety attack"), py_str("anxiety attacks")]));
    s__THEM(py_tuple(vec![py_str("my mum"), py_str("my mom"), py_str("my dad"), py_str("my sister"), py_str("my brother"), py_str("my son"), py_str("my daughter"), py_str("my friend"), py_str("my mate"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my boyfriend"), py_str("my girlfriend"), py_str("my colleague"), py_str("my boss"), py_str("my nan"), py_str("my nana"), py_str("my grandma"), py_str("my grandad"), py_str("my neighbour"), py_str("he is"), py_str("she is"), py_str("hes having"), py_str("shes having"), py_str("theyre having"), py_str("they are having"), py_str("someone"), py_str("somebody"), py_str("him"), py_str("her"), py_str("he had"), py_str("she had"), py_str("they had")]));
    s__BEHIND(py_tuple(vec![py_str("last year"), py_str("last month"), py_str("last week"), py_str("last summer"), py_str("yesterday"), py_str("last night"), py_str("years ago"), py_str("a year ago"), py_str("months ago"), py_str("weeks ago"), py_str("ages ago"), py_str("a while ago"), py_str("in the past"), py_str("when i was"), py_str("used to"), py_str("have had"), py_str("had one before"), py_str("had a few"), py_str("had a panic"), py_str("has had"), py_str("was having"), py_str("kept having"), py_str("got them"), py_str("get them")]));
    s__MINE(py_tuple(vec![py_str("im having a panic"), py_str("i am having a panic"), py_str("i think im having"), py_str("im panicking"), py_str("i am panicking"), py_str("ive been having panic"), py_str("im about to have a panic"), py_str("my panic attack")]));
    s__WORRY_ABOUT(py_tuple(vec![py_str("about the"), py_str("about this"), py_str("about that"), py_str("about her"), py_str("about his"), py_str("about their"), py_str("about work"), py_str("about money"), py_str("about exams"), py_str("about tomorrow")]));
    s__THEIR_OWN_PAST(py_tuple(vec![py_str("he had"), py_str("she had"), py_str("they had"), py_str("he has"), py_str("she has"), py_str("they have"), py_str("he gets"), py_str("she gets")]));
    s__BEHIND_THEM(py_tuple(vec![py_str("years ago"), py_str("last year"), py_str("last month"), py_str("last week"), py_str("last summer"), py_str("in the past"), py_str("when i was"), py_str("when he was"), py_str("when she was"), py_str("used to"), py_str("months ago"), py_str("weeks ago"), py_str("a while ago")]));
    s__DIAGNOSIS(py_tuple(vec![py_str("or is he just"), py_str("or is she just"), py_str("or is it just"), py_str("or is that just"), py_str("or am i just"), py_str("or just shy"), py_str("just shy"), py_str("or attention")]));
    s__PATTERN(py_tuple(vec![py_str("when he has to"), py_str("when she has to"), py_str("when they have to"), py_str("when i have to"), py_str("whenever"), py_str("every time"), py_str("each time")]));
    s__ATTACK_NAMES(py_tuple(vec![py_str("panic attacks"), py_str("anxiety attacks"), py_str("panics")]));
    s__HABIT_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ATTACK_NAMES()) { let mut name: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("has"), py_str("have"), py_str("gets"), py_str("get"), py_str("had"), py_str("having")])) { let mut verb: Py = __item.clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&verb.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); py_string(__s) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HABIT_MARKERS(py_tuple(vec![py_str("panics"), py_str("panicked"), py_str("every day"), py_str("every week"), py_str("every few days"), py_str("usually"), py_str("tends to"), py_str("at school"), py_str("before exams"), py_str("on the tube"), py_str("at work"), py_str("in meetings"), py_str("a lot")]));
    s__HABIT_ONCE(py_tuple(vec![py_str(" had "), py_str(" panicked"), py_str(" was having"), py_str(" kept having"), py_str(" last "), py_str(" on the way")]));
    s__HABIT(op_add(&g__HABIT_VERBS(), &g__HABIT_MARKERS()));
    s__ADVICE_ASK(py_tuple(vec![py_str("what can i do"), py_str("what do i do"), py_str("what should i do"), py_str("how do i help"), py_str("how can i help"), py_str("how do i calm"), py_str("how do i talk"), py_str("what can i say"), py_str("how do i support"), py_str("what helps"), py_str("any advice"), py_str("what can we do")]));
    s_LIVING_WITH(py_str("Ask them, on a calm day, what helps when one starts - most people know, and being asked is worth more than being managed. What helps most between attacks is not avoiding the places they happen: the list of avoided things grows, and the fear with it. Panic disorder is treatable, so it is worth a GP conversation: CBT has the best evidence, and an SSRI helps some people alongside it. If the attacks are getting more frequent, or they start avoiding work, shops or travel, say so to the GP rather than waiting. And if one ever comes with chest pain, or it is their first, get it checked rather than assuming panic."));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("panic-someone-else")); __args.push(py_tuple(vec![g__ATTACK(), g__THEM()])); __args.push(g_SOMEONE_ELSE()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&g__MINE(), &g__DIAGNOSIS()), &g__PATTERN()), &g__WORRY_ABOUT()), &g__BEHIND_THEM()), &crate::knowledge::lookup::g_NOT_A_NOSE())), ("guard".to_string(), g__a_habit_is_not_a_minute())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("panic-attack-in-the-past")); __args.push(py_tuple(vec![g__ATTACK(), g__BEHIND()])); __args.push(g_BEHIND_YOU()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("im having a panic"), py_str("i am having a panic"), py_str("right now")])), ("guard".to_string(), g__the_attack_is_the_speakers_own())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("panic-someone-else-habit")); __args.push(py_tuple(vec![op_add(&g__ATTACK(), &g__ANXIETY()), g__THEM()])); __args.push(g_LIVING_WITH()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&g__MINE(), &g__DIAGNOSIS()), &g__PATTERN()), &g__WORRY_ABOUT()), &g__BEHIND_THEM()), &crate::knowledge::lookup::g_NOT_A_NOSE())), ("guard".to_string(), g__a_flat_habit())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _a_flat_habit(args: &[Py]) -> Py {
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
    let _ = py_str("True on a habit stated flatly, which is the one line this row is written for.");
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_habit_is_not_a_minute(args))(&__args) }));
    Py::None
}
pub fn _the_attack_is_the_speakers_own(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut own: Py = Py::None;
    let mut them: Py = Py::None;
    let _ = py_str("True when the past card is the right card, which is the speaker's own attack.\n\n    \"my husband had a panic attack yesterday\" is the same evening seen from the outside. The past\n    card thanks *you* for getting through it, and it took those lines off the helping card when\n    the habit row arrived; telling the person what to do for somebody else is the helping card's\n    job, and it says \"them\" throughout. A past that is a long way back keeps the past card,\n    whatever the person, because there the question really is whether it is behind them.\n    ");
    low = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _text(args))(&__args) }), &py_str(" "));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MINE()) { let mut mine: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &mine.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("i had"), py_str("ive had"), py_str("i have had"), py_str("i get"), py_str("i used to"), py_str("i was having")])) { let mut own: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &own.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BEHIND_THEM()) { let mut back: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &back.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g__THEM(), &g__THEIR_OWN_PAST())) { let mut them: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &them.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn _the_line_asks_what_to_do(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("True when a habit line is asking for the advice, which is the one time this card fits.");
    low = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _text(args))(&__args) }), &py_str(" "));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ADVICE_ASK()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "strip", args))(&__args) }, "startswith", args))(&__args) } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_habit_is_not_a_minute(args: &[Py]) -> Py {
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
    let mut habit: Py = Py::None;
    let mut low: Py = Py::None;
    let mut once: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("False on a habit stated flatly, true when the line asks what to do about it.");
    low = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _text(args))(&__args) }), &py_str(" "));
    habit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HABIT_VERBS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&habit.clone()))) {
    once = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HABIT_ONCE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    habit = { let __b = Py::Bool(!py_truthy(&once.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HABIT_MARKERS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    }
    return { let __b = Py::Bool(!py_truthy(&habit.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_asks_what_to_do(args))(&__args) } } };
    Py::None
}
pub fn _text(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line as lower-case words, the way the other tables read one.");
    words = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { words.clone() } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.panic_talk".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "BEHIND_YOU" => Some(g_BEHIND_YOU()),
            "ENTRIES" => Some(g_ENTRIES()),
            "LIVING_WITH" => Some(g_LIVING_WITH()),
            "NOT_A_NOSE" => Some(g_NOT_A_NOSE()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "_ADVICE_ASK" => Some(g__ADVICE_ASK()),
            "_ANXIETY" => Some(g__ANXIETY()),
            "_ATTACK" => Some(g__ATTACK()),
            "_ATTACK_NAMES" => Some(g__ATTACK_NAMES()),
            "_BEHIND" => Some(g__BEHIND()),
            "_BEHIND_THEM" => Some(g__BEHIND_THEM()),
            "_DIAGNOSIS" => Some(g__DIAGNOSIS()),
            "_HABIT" => Some(g__HABIT()),
            "_HABIT_MARKERS" => Some(g__HABIT_MARKERS()),
            "_HABIT_ONCE" => Some(g__HABIT_ONCE()),
            "_HABIT_VERBS" => Some(g__HABIT_VERBS()),
            "_MINE" => Some(g__MINE()),
            "_PATTERN" => Some(g__PATTERN()),
            "_THEIR_OWN_PAST" => Some(g__THEIR_OWN_PAST()),
            "_THEM" => Some(g__THEM()),
            "_WORRY_ABOUT" => Some(g__WORRY_ABOUT()),
            "_a_flat_habit" => Some(py_func("_a_flat_habit", |args: &[Py]| _a_flat_habit(args))),
            "_a_habit_is_not_a_minute" => Some(py_func("_a_habit_is_not_a_minute", |args: &[Py]| _a_habit_is_not_a_minute(args))),
            "_text" => Some(py_func("_text", |args: &[Py]| _text(args))),
            "_the_attack_is_the_speakers_own" => Some(py_func("_the_attack_is_the_speakers_own", |args: &[Py]| _the_attack_is_the_speakers_own(args))),
            "_the_line_asks_what_to_do" => Some(py_func("_the_line_asks_what_to_do", |args: &[Py]| _the_line_asks_what_to_do(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "BEHIND_YOU" => s_BEHIND_YOU(v),
            "ENTRIES" => s_ENTRIES(v),
            "LIVING_WITH" => s_LIVING_WITH(v),
            "NOT_A_NOSE" => s_NOT_A_NOSE(v),
            "SOMEONE_ELSE" => s_SOMEONE_ELSE(v),
            "_ADVICE_ASK" => s__ADVICE_ASK(v),
            "_ANXIETY" => s__ANXIETY(v),
            "_ATTACK" => s__ATTACK(v),
            "_ATTACK_NAMES" => s__ATTACK_NAMES(v),
            "_BEHIND" => s__BEHIND(v),
            "_BEHIND_THEM" => s__BEHIND_THEM(v),
            "_DIAGNOSIS" => s__DIAGNOSIS(v),
            "_HABIT" => s__HABIT(v),
            "_HABIT_MARKERS" => s__HABIT_MARKERS(v),
            "_HABIT_ONCE" => s__HABIT_ONCE(v),
            "_HABIT_VERBS" => s__HABIT_VERBS(v),
            "_MINE" => s__MINE(v),
            "_PATTERN" => s__PATTERN(v),
            "_THEIR_OWN_PAST" => s__THEIR_OWN_PAST(v),
            "_THEM" => s__THEM(v),
            "_WORRY_ABOUT" => s__WORRY_ABOUT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.panic_talk", module_obj);
    pyrt::xrefs::reg_fn("knowledge.panic_talk", "_a_flat_habit", py_func("_a_flat_habit", |args: &[Py]| _a_flat_habit(args)));
    pyrt::xrefs::reg_fn("knowledge.panic_talk", "_the_attack_is_the_speakers_own", py_func("_the_attack_is_the_speakers_own", |args: &[Py]| _the_attack_is_the_speakers_own(args)));
    pyrt::xrefs::reg_fn("knowledge.panic_talk", "_the_line_asks_what_to_do", py_func("_the_line_asks_what_to_do", |args: &[Py]| _the_line_asks_what_to_do(args)));
    pyrt::xrefs::reg_fn("knowledge.panic_talk", "_a_habit_is_not_a_minute", py_func("_a_habit_is_not_a_minute", |args: &[Py]| _a_habit_is_not_a_minute(args)));
    pyrt::xrefs::reg_fn("knowledge.panic_talk", "_text", py_func("_text", |args: &[Py]| _text(args)));
    pyrt::xrefs::reg_access("knowledge.panic_talk", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "BEHIND_YOU", g_BEHIND_YOU, s_BEHIND_YOU);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "LIVING_WITH", g_LIVING_WITH, s_LIVING_WITH);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "NOT_A_NOSE", g_NOT_A_NOSE, s_NOT_A_NOSE);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "SOMEONE_ELSE", g_SOMEONE_ELSE, s_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_ADVICE_ASK", g__ADVICE_ASK, s__ADVICE_ASK);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_ANXIETY", g__ANXIETY, s__ANXIETY);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_ATTACK", g__ATTACK, s__ATTACK);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_ATTACK_NAMES", g__ATTACK_NAMES, s__ATTACK_NAMES);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_BEHIND", g__BEHIND, s__BEHIND);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_BEHIND_THEM", g__BEHIND_THEM, s__BEHIND_THEM);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_DIAGNOSIS", g__DIAGNOSIS, s__DIAGNOSIS);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_HABIT", g__HABIT, s__HABIT);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_HABIT_MARKERS", g__HABIT_MARKERS, s__HABIT_MARKERS);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_HABIT_ONCE", g__HABIT_ONCE, s__HABIT_ONCE);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_HABIT_VERBS", g__HABIT_VERBS, s__HABIT_VERBS);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_MINE", g__MINE, s__MINE);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_PATTERN", g__PATTERN, s__PATTERN);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_THEIR_OWN_PAST", g__THEIR_OWN_PAST, s__THEIR_OWN_PAST);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_THEM", g__THEM, s__THEM);
    pyrt::xrefs::reg_access("knowledge.panic_talk", "_WORRY_ABOUT", g__WORRY_ABOUT, s__WORRY_ABOUT);
}
