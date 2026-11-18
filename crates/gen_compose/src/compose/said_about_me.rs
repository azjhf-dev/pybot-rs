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


thread_local! { static G_CLOSE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTED_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTED_LABEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ACCUSED_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLOSE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEFORE_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DANGER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GENERIC_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRANDPARENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRANDPARENT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HABIT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARENT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARTNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPORTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SELF_ADDRESSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIBLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIBLING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_CLOSE_PERSON() -> Py { ensure_init(); G_CLOSE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSE_PERSON(v: Py) { ensure_init(); G_CLOSE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTED() -> Py { ensure_init(); G_REPORTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTED(v: Py) { ensure_init(); G_REPORTED.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTED_ACT() -> Py { ensure_init(); G_REPORTED_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTED_ACT(v: Py) { ensure_init(); G_REPORTED_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTED_LABEL() -> Py { ensure_init(); G_REPORTED_LABEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTED_LABEL(v: Py) { ensure_init(); G_REPORTED_LABEL.with(|c| *c.borrow_mut() = v); }
pub fn g__ACCUSED_LINES() -> Py { ensure_init(); G__ACCUSED_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ACCUSED_LINES(v: Py) { ensure_init(); G__ACCUSED_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__ACT() -> Py { ensure_init(); G__ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ACT(v: Py) { ensure_init(); G__ACT.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_SAYING() -> Py { ensure_init(); G__AFTER_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_SAYING(v: Py) { ensure_init(); G__AFTER_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLOSE_PERSON() -> Py { ensure_init(); G__A_CLOSE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLOSE_PERSON(v: Py) { ensure_init(); G__A_CLOSE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HABIT() -> Py { ensure_init(); G__A_HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HABIT(v: Py) { ensure_init(); G__A_HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATE() -> Py { ensure_init(); G__A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATE(v: Py) { ensure_init(); G__A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g__BEFORE_SAYING() -> Py { ensure_init(); G__BEFORE_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEFORE_SAYING(v: Py) { ensure_init(); G__BEFORE_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g__DANGER() -> Py { ensure_init(); G__DANGER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DANGER(v: Py) { ensure_init(); G__DANGER.with(|c| *c.borrow_mut() = v); }
pub fn g__GENERIC_LINES() -> Py { ensure_init(); G__GENERIC_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GENERIC_LINES(v: Py) { ensure_init(); G__GENERIC_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__GETTING() -> Py { ensure_init(); G__GETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GETTING(v: Py) { ensure_init(); G__GETTING.with(|c| *c.borrow_mut() = v); }
pub fn g__GRANDPARENT() -> Py { ensure_init(); G__GRANDPARENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRANDPARENT(v: Py) { ensure_init(); G__GRANDPARENT.with(|c| *c.borrow_mut() = v); }
pub fn g__GRANDPARENT_LINES() -> Py { ensure_init(); G__GRANDPARENT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRANDPARENT_LINES(v: Py) { ensure_init(); G__GRANDPARENT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__HABIT_LINES() -> Py { ensure_init(); G__HABIT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HABIT_LINES(v: Py) { ensure_init(); G__HABIT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__PARENT() -> Py { ensure_init(); G__PARENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARENT(v: Py) { ensure_init(); G__PARENT.with(|c| *c.borrow_mut() = v); }
pub fn g__PARENT_LINES() -> Py { ensure_init(); G__PARENT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARENT_LINES(v: Py) { ensure_init(); G__PARENT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__PARTNER() -> Py { ensure_init(); G__PARTNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARTNER(v: Py) { ensure_init(); G__PARTNER.with(|c| *c.borrow_mut() = v); }
pub fn g__REPORTING() -> Py { ensure_init(); G__REPORTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPORTING(v: Py) { ensure_init(); G__REPORTING.with(|c| *c.borrow_mut() = v); }
pub fn g__SAYING() -> Py { ensure_init(); G__SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAYING(v: Py) { ensure_init(); G__SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL() -> Py { ensure_init(); G__SCHOOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL(v: Py) { ensure_init(); G__SCHOOL.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL_LINES() -> Py { ensure_init(); G__SCHOOL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL_LINES(v: Py) { ensure_init(); G__SCHOOL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__SELF_ADDRESSED() -> Py { ensure_init(); G__SELF_ADDRESSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SELF_ADDRESSED(v: Py) { ensure_init(); G__SELF_ADDRESSED.with(|c| *c.borrow_mut() = v); }
pub fn g__SIBLING() -> Py { ensure_init(); G__SIBLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIBLING(v: Py) { ensure_init(); G__SIBLING.with(|c| *c.borrow_mut() = v); }
pub fn g__SIBLING_LINES() -> Py { ensure_init(); G__SIBLING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIBLING_LINES(v: Py) { ensure_init(); G__SIBLING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT() -> Py { ensure_init(); G__SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT(v: Py) { ensure_init(); G__SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__VERDICT() -> Py { ensure_init(); G__VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERDICT(v: Py) { ensure_init(); G__VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g__WORK() -> Py { ensure_init(); G__WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORK(v: Py) { ensure_init(); G__WORK.with(|c| *c.borrow_mut() = v); }
pub fn g__WORK_LINES() -> Py { ensure_init(); G__WORK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORK_LINES(v: Py) { ensure_init(); G__WORK_LINES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Someone's judgement of the person, reported: \"my mum says im not trying hard enough\".\n\nThe store files the line as a fact and hands it back - \"Noted: your mum says you're not trying\nhard enough\" - which is the one reply a person who has just been criticised at home does not\nwant. The other end of the same line is worse: \"my sister says im a cow\" came back as\n\"Cow - good to know. What do you like most about it?\", a question about a farm animal.\n\nWhat they are telling you is not a fact about their mother. It is what it is like to be spoken to\nthat way, so the reply has to land on that: say the sting is real, and ask the one thing that\nopens it up - what it was about, whether it is new, or what they make of it.\n\nRead here by shape, not by wording: a person who is close (a parent, a sibling, a teacher, a\nboss - and the bare \"he says\"/\"she says\" that people type when the person is obvious from the\nlast turn), a verb of saying, and then a verdict on the person in the first person. Every way of\nwriting that is the same line, so every one of them gets the same kind of answer.\n\nTwo things stand it down, both deliberate. A line with violence, fear or control in it keeps\nwhatever it had: \"my husband says im worthless\" is the domestic abuse card in `bot/modules/\nsafety.py`, and a composer reader taking it away would be a real loss. And the partner-ish\nsubject is left alone for the same reason - a partner who calls you useless is the case that card\nis for, and whether a mother's \"you're lazy\" belongs there too is #safety's call, not this\nmodule's.\n");
    s__A_CLOSE_PERSON(py_str("(?:mum|mom|mam|mammy|mother|dad|father|daddy|stepmum|stepmom|stepdad|stepfather|stepmother|parent|nan|nana|gran|granny|grandma|grandad|grandpa|grandmother|grandfather|sister|brother|sis|bro|son|daughter|child|kid|teenager|teacher|tutor|head\\s+teacher|boss|manager|colleague|friend|mate|neighbour|neighbor|uncle|aunt|auntie|mother[\\s-]in[\\s-]law|father[\\s-]in[\\s-]law|in[\\s-]laws|cousin)"));
    s_CLOSE_PERSON(op_add(&g__A_CLOSE_PERSON(), &py_str("(?:s|es)?")));
    s__SUBJECT(op_add(&op_add(&py_str("(?:(?:my|our)\\s+"), &g_CLOSE_PERSON()), &py_str("|(?:he|she|they))")));
    s__SAYING(py_str("(?:say|says|said|saying|tell|tells|told|telling|think|thinks|thought|reckon|reckons|moan|moans|moaned|complain|complains|complained|goes|went)(?:\\s+(?:me|that|how))?"));
    s__A_STATE(py_str("(?!fine\\b|ok\\b|okay\\b|alright\\b|all\\s+right\\b|good\\b|great\\b|grand\\b|tired\\b|busy\\b|hungry\\b|sleepy\\b|bored\\b|sad\\b|happy\\b|upset\\b|angry\\b|cross\\b|late\\b|early\\b|right\\b|wrong\\b)"));
    s__GETTING(py_str("(?:getting|putting\\s+on|gone)\\s+(?:a\\s+bit\\s+|so\\s+|really\\s+|too\\s+)?(?:fat|big|heavy|chubby|large|weight|old|older|slow|slower|forgetful|lazy|picky|soft|naughty|moody|grumpy|difficult)"));
    s__VERDICT(op_add(&op_add(&op_add(&op_add(&py_str("(?:i(?:'?m|\\s+am|\\s+aint|m)\\s+"), &g__GETTING()), &py_str("|i(?:'?m|\\s+am|\\s+aint|m)\\s+(?:not\\s+|so\\s+|too\\s+|just\\s+|always\\s+|really\\s+)?(?:a\\s+|an\\s+|the\\s+)?")), &g__A_STATE()), &py_str("[a-z']+)")));
    s__BEFORE_SAYING(py_str("(?:\\s+(?:keeps?|kept|always|constantly|regularly|just|still|also|now|honestly|literally|basically|straight|even))*(?:\\s+is|\\s+was)?\\s*"));
    s__AFTER_SAYING(py_str("\\s+(?:(?:me|that|to\\s+me|straight|honestly|basically|again|either)\\s+){0,2}"));
    s__ACT(py_str("(?:i\\s+(?:dont|don't|do\\s+not|never|always|keep|can'?t|cannot|wont|won'?t|made|make|mess|screw))"));
    s_REPORTED_LABEL(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g__SUBJECT()), &py_str("\\b")), &g__BEFORE_SAYING()), &py_str("\\b")), &g__SAYING()), &g__AFTER_SAYING()), &g__VERDICT()), &py_str("[a-z']*")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORTED_ACT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g__SUBJECT()), &py_str("\\b")), &g__BEFORE_SAYING()), &py_str("\\b")), &g__SAYING()), &g__AFTER_SAYING()), &g__ACT()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORTED(g_REPORTED_LABEL());
    s__DANGER(stdlib_call("re", "compile", &[py_str("\\b(?:hit|hits|hitting|hurt|hurts|hurting|punch|punched|kick|kicked|slap|slapped|push|pushed|pushing|throw|threw|thrown|strangle|strangled|choke|choked|choking|threat|threatens|threatened|threatening|scared|scary|frightened|afraid|terrified|fear|unsafe|not\\s+safe|safety|lock|locked|locks|police|arrest|bruise|bruises|bleeding|kill|kills|killing|dead|die|suicide|self[\\s-]harm|starve|starving|withhold|control|controlling|controls|track|tracks|tracking|monitor|monitors|isolate|money|phone|passport|keys|drink|drunk|drugs|scream|screams|screaming|shout|shouts|shouting|yell|yells|yelling|swear|swears|swearing|silent\\s+treatment|ignores\\s+me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PARTNER(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|ex|ex[\\s-]?(?:husband|wife|partner|boyfriend|girlfriend)|hubby|carer|step[\\s-]?dad|step[\\s-]?mum|step[\\s-]?father|step[\\s-]?mother)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__GRANDPARENT(stdlib_call("re", "compile", &[py_str("\\b(?:nan|nana|gran|granny|grandma|grandad|grandpa|grandmother|grandfather)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PARENT(stdlib_call("re", "compile", &[py_str("\\b(?:mum|mom|mam|mammy|mother|dad|father|daddy|stepmum|stepmom|stepdad|stepfather|stepmother|parent)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SIBLING(stdlib_call("re", "compile", &[py_str("\\b(?:sister|brother|sis|bro)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WORK(stdlib_call("re", "compile", &[py_str("\\b(?:boss|manager|colleague)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SCHOOL(stdlib_call("re", "compile", &[py_str("\\b(?:teacher|tutor|head\\s+teacher|lecturer|form\\s+tutor)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PARENT_LINES(py_tuple(vec![py_str("That is a hard thing to hear from your mum or dad, and it says more about the day they are having than about you. Is this how they usually talk, or is it new?"), py_str("It lands hardest from a parent, whoever else says the same thing. What did they want you to be doing differently?"), py_str("Parents say the thing that will sting without meaning to. Has it been like this a while, or did something happen today?")]));
    s__GRANDPARENT_LINES(py_tuple(vec![py_str("That is the kind of thing a nan says and then worries about. How did it land - did it come out of the blue, or is it a theme?"), py_str("Grandparents talk about weight in a way their own generation did, and it still stings coming from someone who is supposed to be on your side. What did you say back?")]));
    s__SIBLING_LINES(py_tuple(vec![py_str("Brothers and sisters know exactly where to aim. Is this how they always talk to you, or has something changed?"), py_str("That is a sibling's line if ever there was one. Do they mean it, or is it the way they talk when they are in a mood?")]));
    s__WORK_LINES(py_tuple(vec![py_str("That is a rough thing to hear where you are trying to do a job. Was it a fair point about the work, or was it a dig?"), py_str("Being told you are not up to it at work is a heavy thing to carry home. Has it come up more than once?"), py_str("A comment like that is easier to carry when you know what it was about. What was the job they were talking about?")]));
    s__SCHOOL_LINES(py_tuple(vec![py_str("Being marked down by a teacher lands differently from being told it at home. Is it this subject, or all of it?"), py_str("A teacher saying that is usually about one piece of work, not about the whole person. What was it about?"), py_str("That is the kind of comment that sticks, and it says as much about their marking as about you. Has it been said more than once?")]));
    s__ACCUSED_LINES(py_tuple(vec![py_str("That sounds like an old argument wearing a new coat. What makes her say it?"), py_str("Being told you do not care, when you clearly do, is a maddening thing to hear. What set it off?"), py_str("It is worth asking her straight what she means by it. What would she want you to do differently?"), py_str("That is a charge, not a small thing. How long has it been going on?")]));
    s__GENERIC_LINES(py_tuple(vec![py_str("That is a rough thing to be told. What did they want you to do differently?"), py_str("Somebody saying that to you says more about them than about you. How did it land - did you believe it?"), py_str("That is a hard thing to hear, whichever way round it was meant. What do you think is behind it?")]));
    s__REPORTING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:he|she|they|him|her|them|"), &mcall(&g_CLOSE_PERSON(), "replace", &[py_str("(?:"), py_str("(?:"), Py::Int(1)])), &py_str(")\\b[^.?!]{0,25}?\\b(?:say|says|said|saying|tell|tells|told|telling|think|thinks|thought|reckon|reckons|reckoned|goes|went|call|calls|called|calling|moan|moans|complain|complains)\\b[^.?!]{0,20}?\\b(?:i(?:'?m|\\s+am)|im)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HABIT(stdlib_call("re", "compile", &[py_str("\\b(?:every\\s+time|every\\s+(?:day|visit|week|night|time)|all\\s+the\\s+time|always|constantly|whenever|each\\s+time|again\\s+and\\s+again|keeps?\\s+(?:doing|saying|going)|again\\s+today|it\\s+happens|she\\s+does\\s+it|he\\s+does\\s+it|they\\s+do\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HABIT_LINES(py_tuple(vec![py_str("Then it is a pattern, not a comment, and a fresh answer each time is what keeps it going. Pick one sentence and use the same one every time, said calmly and early: \"I have heard you, and I am not going to argue about it today.\""), py_str("If it comes up again and again, say once at the start that you are not going over it this time, and then do not take the bait. Naming it before it lands takes the teeth out of it."), py_str("A remark that comes every time needs a conversation at a calm moment rather than in the moment it happens. What would you want to say to her when nothing is going on?")]));
    s__SELF_ADDRESSED(stdlib_call("re", "compile", &[py_str("\\b(?:yourself|how\\s+you\\s+feel|feel\\s+about\\s+yourself|you\\s+feel\\s+like|youd\\s+be\\s+on\\s+a\\s+friend|harder\\s+on\\s+yourself)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn reads_as_somebody_elses_words(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the person is quoting what somebody close calls them, in their own words.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__REPORTING(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _kind(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Which of the two the line is: \"label\" for a name they were called, \"act\" for a charge\n    about what they do, and \"\" for neither.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g__DANGER(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__PARTNER(), "search", &[said.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_REPORTED_LABEL(), "search", &[said.clone()])) {
    return py_str("label");
    }
    if py_truthy(&mcall(&g_REPORTED_ACT(), "search", &[said.clone()])) {
    return py_str("act");
    }
    return py_str("");
    Py::None
}
pub fn reads_as_reported_judgement(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is somebody close handing the person a verdict about themselves.");
    return call_builtin("bool", &[_kind(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _user_lines(args: &[Py]) -> Py {
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
    let mut how_many: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how_many") { v.clone() }
    else { Py::Int(3) };
    let mut line: Py = Py::None;
    let mut out: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_list(vec![]);
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_mul(&op_neg(&how_many.clone()), &Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&out.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&out.clone(), &op_neg(&Py::Int(1))), &line.clone())) } }) {
    let _ = mcall(&out.clone(), "append", &[line.clone()]);
    }
    }
    return slice_or_raise(&out.clone(), &op_neg(&how_many.clone()), &Py::None, &Py::None);
    Py::None
}
pub fn _on_the_thread(args: &[Py]) -> Py {
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
    let mut lines: Py = Py::None;
    let _ = py_str("Whether the person has just been talking about being spoken to that way.\n\n    The judgement itself, or the remark coming round again - \"she says it every time i visit\",\n    \"and he does it all the time\" - is the same thread, so a third turn stays on it too.\n    ");
    lines = _user_lines(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&reads_as_reported_judgement(&[geti(&lines.clone(), &op_neg(&Py::Int(1)))])) {
    return Py::Bool(true);
    }
    return { let __b = op_gt(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g__A_HABIT(), "search", &[geti(&lines.clone(), &op_neg(&Py::Int(1)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { reads_as_reported_judgement(&[geti(&lines.clone(), &op_neg(&Py::Int(2)))]) } } } };
    Py::None
}
pub fn answer_followup(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("A reply when the judgement just described is said to happen again and again, or \"\".");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g__DANGER(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_HABIT(), "search", &[said.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_on_the_thread(&[memory.clone()])))) {
    return py_str("");
    }
    return crate::compose::pick(&[g__HABIT_LINES(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, said.clone()]);
    Py::None
}
pub fn speaks_to_a_self_judgement(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the reply answers the person as if they had judged themselves.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__SELF_ADDRESSED(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
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
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A reply to a reported judgement, or \"\" when the line is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_reported_judgement(&[text.clone()])))) {
    return py_str("");
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(py_eq(&_kind(&[said.clone()]), &py_str("act")))) {
    return crate::compose::pick(&[g__ACCUSED_LINES(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, said.clone()]);
    }
    if py_truthy(&mcall(&g__SCHOOL(), "search", &[said.clone()])) {
    pool = g__SCHOOL_LINES();
    } else {
    if py_truthy(&mcall(&g__WORK(), "search", &[said.clone()])) {
    pool = g__WORK_LINES();
    } else {
    if py_truthy(&mcall(&g__SIBLING(), "search", &[said.clone()])) {
    pool = g__SIBLING_LINES();
    } else {
    if py_truthy(&mcall(&g__GRANDPARENT(), "search", &[said.clone()])) {
    pool = g__GRANDPARENT_LINES();
    } else {
    if py_truthy(&mcall(&g__PARENT(), "search", &[said.clone()])) {
    pool = g__PARENT_LINES();
    } else {
    pool = g__GENERIC_LINES();
    }
    }
    }
    }
    }
    return crate::compose::pick(&[pool.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, said.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.said_about_me".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CLOSE_PERSON" => Some(g_CLOSE_PERSON()),
            "REPORTED" => Some(g_REPORTED()),
            "REPORTED_ACT" => Some(g_REPORTED_ACT()),
            "REPORTED_LABEL" => Some(g_REPORTED_LABEL()),
            "_ACCUSED_LINES" => Some(g__ACCUSED_LINES()),
            "_ACT" => Some(g__ACT()),
            "_AFTER_SAYING" => Some(g__AFTER_SAYING()),
            "_A_CLOSE_PERSON" => Some(g__A_CLOSE_PERSON()),
            "_A_HABIT" => Some(g__A_HABIT()),
            "_A_STATE" => Some(g__A_STATE()),
            "_BEFORE_SAYING" => Some(g__BEFORE_SAYING()),
            "_DANGER" => Some(g__DANGER()),
            "_GENERIC_LINES" => Some(g__GENERIC_LINES()),
            "_GETTING" => Some(g__GETTING()),
            "_GRANDPARENT" => Some(g__GRANDPARENT()),
            "_GRANDPARENT_LINES" => Some(g__GRANDPARENT_LINES()),
            "_HABIT_LINES" => Some(g__HABIT_LINES()),
            "_PARENT" => Some(g__PARENT()),
            "_PARENT_LINES" => Some(g__PARENT_LINES()),
            "_PARTNER" => Some(g__PARTNER()),
            "_REPORTING" => Some(g__REPORTING()),
            "_SAYING" => Some(g__SAYING()),
            "_SCHOOL" => Some(g__SCHOOL()),
            "_SCHOOL_LINES" => Some(g__SCHOOL_LINES()),
            "_SELF_ADDRESSED" => Some(g__SELF_ADDRESSED()),
            "_SIBLING" => Some(g__SIBLING()),
            "_SIBLING_LINES" => Some(g__SIBLING_LINES()),
            "_SUBJECT" => Some(g__SUBJECT()),
            "_VERDICT" => Some(g__VERDICT()),
            "_WORK" => Some(g__WORK()),
            "_WORK_LINES" => Some(g__WORK_LINES()),
            "_kind" => Some(py_func("_kind", |args: &[Py]| _kind(args))),
            "_on_the_thread" => Some(py_func("_on_the_thread", |args: &[Py]| _on_the_thread(args))),
            "_user_lines" => Some(py_func("_user_lines", |args: &[Py]| _user_lines(args))),
            "answer_followup" => Some(py_func("answer_followup", |args: &[Py]| answer_followup(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "reads_as_reported_judgement" => Some(py_func("reads_as_reported_judgement", |args: &[Py]| reads_as_reported_judgement(args))),
            "reads_as_somebody_elses_words" => Some(py_func("reads_as_somebody_elses_words", |args: &[Py]| reads_as_somebody_elses_words(args))),
            "speaks_to_a_self_judgement" => Some(py_func("speaks_to_a_self_judgement", |args: &[Py]| speaks_to_a_self_judgement(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CLOSE_PERSON" => s_CLOSE_PERSON(v),
            "REPORTED" => s_REPORTED(v),
            "REPORTED_ACT" => s_REPORTED_ACT(v),
            "REPORTED_LABEL" => s_REPORTED_LABEL(v),
            "_ACCUSED_LINES" => s__ACCUSED_LINES(v),
            "_ACT" => s__ACT(v),
            "_AFTER_SAYING" => s__AFTER_SAYING(v),
            "_A_CLOSE_PERSON" => s__A_CLOSE_PERSON(v),
            "_A_HABIT" => s__A_HABIT(v),
            "_A_STATE" => s__A_STATE(v),
            "_BEFORE_SAYING" => s__BEFORE_SAYING(v),
            "_DANGER" => s__DANGER(v),
            "_GENERIC_LINES" => s__GENERIC_LINES(v),
            "_GETTING" => s__GETTING(v),
            "_GRANDPARENT" => s__GRANDPARENT(v),
            "_GRANDPARENT_LINES" => s__GRANDPARENT_LINES(v),
            "_HABIT_LINES" => s__HABIT_LINES(v),
            "_PARENT" => s__PARENT(v),
            "_PARENT_LINES" => s__PARENT_LINES(v),
            "_PARTNER" => s__PARTNER(v),
            "_REPORTING" => s__REPORTING(v),
            "_SAYING" => s__SAYING(v),
            "_SCHOOL" => s__SCHOOL(v),
            "_SCHOOL_LINES" => s__SCHOOL_LINES(v),
            "_SELF_ADDRESSED" => s__SELF_ADDRESSED(v),
            "_SIBLING" => s__SIBLING(v),
            "_SIBLING_LINES" => s__SIBLING_LINES(v),
            "_SUBJECT" => s__SUBJECT(v),
            "_VERDICT" => s__VERDICT(v),
            "_WORK" => s__WORK(v),
            "_WORK_LINES" => s__WORK_LINES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.said_about_me", module_obj);
    pyrt::xrefs::reg_fn("compose.said_about_me", "reads_as_somebody_elses_words", py_func("reads_as_somebody_elses_words", |args: &[Py]| reads_as_somebody_elses_words(args)));
    pyrt::xrefs::reg_fn("compose.said_about_me", "_kind", py_func("_kind", |args: &[Py]| _kind(args)));
    pyrt::xrefs::reg_fn("compose.said_about_me", "reads_as_reported_judgement", py_func("reads_as_reported_judgement", |args: &[Py]| reads_as_reported_judgement(args)));
    pyrt::xrefs::reg_fn("compose.said_about_me", "_user_lines", py_func("_user_lines", |args: &[Py]| _user_lines(args)));
    pyrt::xrefs::reg_fn("compose.said_about_me", "_on_the_thread", py_func("_on_the_thread", |args: &[Py]| _on_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.said_about_me", "answer_followup", py_func("answer_followup", |args: &[Py]| answer_followup(args)));
    pyrt::xrefs::reg_fn("compose.said_about_me", "speaks_to_a_self_judgement", py_func("speaks_to_a_self_judgement", |args: &[Py]| speaks_to_a_self_judgement(args)));
    pyrt::xrefs::reg_fn("compose.said_about_me", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.said_about_me", "CLOSE_PERSON", g_CLOSE_PERSON, s_CLOSE_PERSON);
    pyrt::xrefs::reg_access("compose.said_about_me", "REPORTED", g_REPORTED, s_REPORTED);
    pyrt::xrefs::reg_access("compose.said_about_me", "REPORTED_ACT", g_REPORTED_ACT, s_REPORTED_ACT);
    pyrt::xrefs::reg_access("compose.said_about_me", "REPORTED_LABEL", g_REPORTED_LABEL, s_REPORTED_LABEL);
    pyrt::xrefs::reg_access("compose.said_about_me", "_ACCUSED_LINES", g__ACCUSED_LINES, s__ACCUSED_LINES);
    pyrt::xrefs::reg_access("compose.said_about_me", "_ACT", g__ACT, s__ACT);
    pyrt::xrefs::reg_access("compose.said_about_me", "_AFTER_SAYING", g__AFTER_SAYING, s__AFTER_SAYING);
    pyrt::xrefs::reg_access("compose.said_about_me", "_A_CLOSE_PERSON", g__A_CLOSE_PERSON, s__A_CLOSE_PERSON);
    pyrt::xrefs::reg_access("compose.said_about_me", "_A_HABIT", g__A_HABIT, s__A_HABIT);
    pyrt::xrefs::reg_access("compose.said_about_me", "_A_STATE", g__A_STATE, s__A_STATE);
    pyrt::xrefs::reg_access("compose.said_about_me", "_BEFORE_SAYING", g__BEFORE_SAYING, s__BEFORE_SAYING);
    pyrt::xrefs::reg_access("compose.said_about_me", "_DANGER", g__DANGER, s__DANGER);
    pyrt::xrefs::reg_access("compose.said_about_me", "_GENERIC_LINES", g__GENERIC_LINES, s__GENERIC_LINES);
    pyrt::xrefs::reg_access("compose.said_about_me", "_GETTING", g__GETTING, s__GETTING);
    pyrt::xrefs::reg_access("compose.said_about_me", "_GRANDPARENT", g__GRANDPARENT, s__GRANDPARENT);
    pyrt::xrefs::reg_access("compose.said_about_me", "_GRANDPARENT_LINES", g__GRANDPARENT_LINES, s__GRANDPARENT_LINES);
    pyrt::xrefs::reg_access("compose.said_about_me", "_HABIT_LINES", g__HABIT_LINES, s__HABIT_LINES);
    pyrt::xrefs::reg_access("compose.said_about_me", "_PARENT", g__PARENT, s__PARENT);
    pyrt::xrefs::reg_access("compose.said_about_me", "_PARENT_LINES", g__PARENT_LINES, s__PARENT_LINES);
    pyrt::xrefs::reg_access("compose.said_about_me", "_PARTNER", g__PARTNER, s__PARTNER);
    pyrt::xrefs::reg_access("compose.said_about_me", "_REPORTING", g__REPORTING, s__REPORTING);
    pyrt::xrefs::reg_access("compose.said_about_me", "_SAYING", g__SAYING, s__SAYING);
    pyrt::xrefs::reg_access("compose.said_about_me", "_SCHOOL", g__SCHOOL, s__SCHOOL);
    pyrt::xrefs::reg_access("compose.said_about_me", "_SCHOOL_LINES", g__SCHOOL_LINES, s__SCHOOL_LINES);
    pyrt::xrefs::reg_access("compose.said_about_me", "_SELF_ADDRESSED", g__SELF_ADDRESSED, s__SELF_ADDRESSED);
    pyrt::xrefs::reg_access("compose.said_about_me", "_SIBLING", g__SIBLING, s__SIBLING);
    pyrt::xrefs::reg_access("compose.said_about_me", "_SIBLING_LINES", g__SIBLING_LINES, s__SIBLING_LINES);
    pyrt::xrefs::reg_access("compose.said_about_me", "_SUBJECT", g__SUBJECT, s__SUBJECT);
    pyrt::xrefs::reg_access("compose.said_about_me", "_VERDICT", g__VERDICT, s__VERDICT);
    pyrt::xrefs::reg_access("compose.said_about_me", "_WORK", g__WORK, s__WORK);
    pyrt::xrefs::reg_access("compose.said_about_me", "_WORK_LINES", g__WORK_LINES, s__WORK_LINES);
}
