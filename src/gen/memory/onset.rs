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


pub static BABY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BABY_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BY_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAUSES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DIED_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOB_LOST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOB_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAVE_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LENGTH_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LONG_ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVED_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_DAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONSET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPLIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPLIT_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNIT_DAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_BABY() -> Py { ensure_init(); BABY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BABY(v: Py) { ensure_init(); if let Ok(mut w) = BABY.write() { *w = v; } }
pub fn g_BABY_REPLIES() -> Py { ensure_init(); BABY_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BABY_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = BABY_REPLIES.write() { *w = v; } }
pub fn g_BARE_UNIT() -> Py { ensure_init(); BARE_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = BARE_UNIT.write() { *w = v; } }
pub fn g_BODY() -> Py { ensure_init(); BODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); if let Ok(mut w) = BODY.write() { *w = v; } }
pub fn g_BODY_REPLIES() -> Py { ensure_init(); BODY_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = BODY_REPLIES.write() { *w = v; } }
pub fn g_BY_CAUSE() -> Py { ensure_init(); BY_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BY_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = BY_CAUSE.write() { *w = v; } }
pub fn g_CAUSES() -> Py { ensure_init(); CAUSES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAUSES(v: Py) { ensure_init(); if let Ok(mut w) = CAUSES.write() { *w = v; } }
pub fn g_DIED() -> Py { ensure_init(); DIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DIED(v: Py) { ensure_init(); if let Ok(mut w) = DIED.write() { *w = v; } }
pub fn g_DIED_REPLIES() -> Py { ensure_init(); DIED_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DIED_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = DIED_REPLIES.write() { *w = v; } }
pub fn g_JOB_LOST() -> Py { ensure_init(); JOB_LOST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOB_LOST(v: Py) { ensure_init(); if let Ok(mut w) = JOB_LOST.write() { *w = v; } }
pub fn g_JOB_REPLIES() -> Py { ensure_init(); JOB_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOB_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = JOB_REPLIES.write() { *w = v; } }
pub fn g_LEAVE_IT() -> Py { ensure_init(); LEAVE_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAVE_IT(v: Py) { ensure_init(); if let Ok(mut w) = LEAVE_IT.write() { *w = v; } }
pub fn g_LENGTH() -> Py { ensure_init(); LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = LENGTH.write() { *w = v; } }
pub fn g_LENGTH_REPLIES() -> Py { ensure_init(); LENGTH_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LENGTH_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = LENGTH_REPLIES.write() { *w = v; } }
pub fn g_LONG_ENOUGH() -> Py { ensure_init(); LONG_ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LONG_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = LONG_ENOUGH.write() { *w = v; } }
pub fn g_MONEY() -> Py { ensure_init(); MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = MONEY.write() { *w = v; } }
pub fn g_MONEY_REPLIES() -> Py { ensure_init(); MONEY_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_REPLIES.write() { *w = v; } }
pub fn g_MOVED() -> Py { ensure_init(); MOVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVED(v: Py) { ensure_init(); if let Ok(mut w) = MOVED.write() { *w = v; } }
pub fn g_MOVED_REPLIES() -> Py { ensure_init(); MOVED_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVED_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = MOVED_REPLIES.write() { *w = v; } }
pub fn g_NAMED() -> Py { ensure_init(); NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = NAMED.write() { *w = v; } }
pub fn g_NAMED_DAYS() -> Py { ensure_init(); NAMED_DAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_DAYS(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_DAYS.write() { *w = v; } }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); NUMBER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER_WORDS.write() { *w = v; } }
pub fn g_ONSET() -> Py { ensure_init(); ONSET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONSET(v: Py) { ensure_init(); if let Ok(mut w) = ONSET.write() { *w = v; } }
pub fn g_OTHER() -> Py { ensure_init(); OTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER(v: Py) { ensure_init(); if let Ok(mut w) = OTHER.write() { *w = v; } }
pub fn g_OTHER_REPLIES() -> Py { ensure_init(); OTHER_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_REPLIES.write() { *w = v; } }
pub fn g_SPLIT() -> Py { ensure_init(); SPLIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPLIT(v: Py) { ensure_init(); if let Ok(mut w) = SPLIT.write() { *w = v; } }
pub fn g_SPLIT_REPLIES() -> Py { ensure_init(); SPLIT_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPLIT_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = SPLIT_REPLIES.write() { *w = v; } }
pub fn g_THE_PERSON() -> Py { ensure_init(); THE_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = THE_PERSON.write() { *w = v; } }
pub fn g_UNIT_DAYS() -> Py { ensure_init(); UNIT_DAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNIT_DAYS(v: Py) { ensure_init(); if let Ok(mut w) = UNIT_DAYS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("When the thing we were just on began: \"it started when i moved house\".\n\nA person who has just described how they are often adds *when* it began, and the clause holds the\ncause rather than a new subject:\n\n    i cant sleep\n    it started when i moved house\n\nRead as a line of its own, `house` is a kind of music and `moved in` is a removals checklist, so\nthe bot answered a question about the wrong thing - the same failure `_filling_in_an_emergency`\nprevents after a safety turn, in a shape every ordinary topic has.\n\nWhat is in here:\n\n- `reads_as_onset(line)` says whether the line is an onset statement, and what it says began it:\n  a life event (`moved house`, `lost my job`, `had the baby`, `my dad died`, `we split up`,\n  `money`) or only a length of time (`about a month ago`, `since christmas`).\n- `reply(line, memory)` turns those into the short answer a friend would give: name the cause,\n  say what it usually does to a person, and give the one thing worth doing about it.\n\nIt stays out of the way of everything else. No onset frame, no reply. A cause it does not know\n(\"it started when i lifted a wardrobe\") is left to the module that answers the pain itself, and a\nbody complaint in the line before it stands the whole reading down, because a move that hurt\nsomeone's back is a lifting injury, not a life event that is keeping them awake.\n");
    s_ONSET(stdlib_call("re", "compile", &[py_str("(?:\\b(?:it|this|that|they|these|those|everything|the\\s+\\w+)\\s+)?\\b(?:all\\s+)?(?:started|started\\s+off|began|begun|kicked\\s+off|came\\s+on|set\\s+off)\\b\\s*(?:off\\s+)?(?:(?:when|after|around|about|in|since|because|following|with|from|last|this)\\b|(?=\\d+|a\\b|an\\b|one\\b|two\\b|three\\b|four\\b|five\\b|six\\b|seven\\b|eight\\b|nine\\b|ten\\b|eleven\\b|twelve\\b|a\\s+couple\\b|a\\s+few\\b|several\\b))")]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY(py_tuple(vec![py_str("\\b(?:back|knee|knees|shoulder|hip|leg|arm|neck|wrist|ankle|foot|feet|hand|hands|head|stomach|chest|tooth|teeth|elbow|thigh|calf|joint|muscle)\\b(?:'s)?\\b|\\b(?:hurts?|hurting|ache|aches|aching|pain|painful|sore|swollen|injured|sprain\\w*)\\b")]));
    s_THE_PERSON(py_tuple(vec![py_str("\\b(?:sleep|sleeps|sleeping|slept|insomnia|wake|wakes|waking|awake|asleep|nightmare|nightmares|nap|naps|napping|tired|exhausted|knackered|drained|weary)\\b|\\b(?:anxious|anxiety|worried|worry|worrying|panic|panicking|stressed|stress|tense|down|low|depressed|depression|miserable|tearful|weepy|crying|hopeless|flat|numb|overwhelmed|burnt\\s+out|burned\\s+out)\\b|\\b(?:cant\\s+sleep|can'?t\\s+sleep|not\\s+sleeping|feel\\s+(?:\\w+\\s+)?(?:low|down|awful))\\b")]));
    s_MOVED(py_tuple(vec![py_str("\\b(?:moved|moving|move)\\b.{0,24}\\b(?:house|home|flat|place|out|in|abroad|country|city|town)\\b|\\b(?:moved\\s+in|house\\s+move|new\\s+(?:house|flat|place|home))\\b")]));
    s_JOB_LOST(py_tuple(vec![py_str("\\b(?:lost|loss|losing|lose)\\b.{0,16}\\b(?:job|work|position|role)\\b|\\b(?:made\\s+redundant|redundancy|redundant|got\\s+the\\s+sack|sacked|laid\\s+off|fired|out\\s+of\\s+work|unemployed)\\b")]));
    s_BABY(py_tuple(vec![py_str("\\b(?:had|having|born|arrived|came\\s+home\\s+with)\\b.{0,20}\\b(?:baby|babies|twins|child|newborn|son|daughter)\\b|\\b(?:new\\s+baby|newborn|pregnan\\w+)\\b")]));
    s_DIED(py_tuple(vec![py_str("\\b(?:died|dying|passed\\s+away|death|funeral|bereave\\w*)\\b|\\blost\\s+(?:my\\s+)?(?:mum|mom|mother|dad|father|husband|wife|partner|son|daughter|brother|sister|friend|gran|nan|grandad|grandma|child)\\b")]));
    s_SPLIT(py_tuple(vec![py_str("\\b(?:split\\s+up|splitting\\s+up|broke\\s+up|broken\\s+up|break\\s+up|breaking\\s+up|divorce\\w*|separated|separation|left\\s+me|walked\\s+out|kicked\\s+me\\s+out)\\b")]));
    s_MONEY(py_tuple(vec![py_str("\\b(?:debts?|arrears|overdraft|rent|mortgage|bills?|money|cost\\s+of\\s+living|can'?t\\s+pay|cant\\s+pay|council\\s+tax|evict\\w*|repossess\\w*)\\b")]));
    s_OTHER(py_tuple(vec![py_str("\\b(?:accident|fell|fall|surgery|operation|operated|hospital|diagnos\\w*|chemo|treat\\w*|illness|ill\\b|unwell|pregnan\\w*|menopause|hormones?|birth|new\\s+shift|night\\s+shift|shift\\s+work|kids?|children|caring|carer|made\\s+homeless|eviction)\\b")]));
    s_CAUSES(py_tuple(vec![py_tuple(vec![py_str("move"), g_MOVED()]), py_tuple(vec![py_str("job"), g_JOB_LOST()]), py_tuple(vec![py_str("baby"), g_BABY()]), py_tuple(vec![py_str("died"), g_DIED()]), py_tuple(vec![py_str("split"), g_SPLIT()]), py_tuple(vec![py_str("money"), g_MONEY()]), py_tuple(vec![py_str("other"), g_OTHER()])]));
    s_NAMED(py_tuple(vec![py_str("\\b(?:christmas|new\\s+year|easter|summer|winter|spring|autumn|last\\s+(?:year|summer|winter)|the\\s+summer|the\\s+winter)\\b")]));
    s_NAMED_DAYS(Py::Int(30));
    s_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:about|around|nearly|almost|over|just\\s+over|a\\s+good|at\\s+least)?\\s*(\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\\s+few|a\\s+couple|several|many)\\s+(?:of\\s+)?(day|days|week|weeks|month|months|year|years)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_UNIT(stdlib_call("re", "compile", &[py_str("^(day|days|week|weeks|month|months|year|years)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORDS(py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("a few"), Py::Int(3)), (py_str("a couple"), Py::Int(2)), (py_str("several"), Py::Int(4)), (py_str("many"), Py::Int(30))]));
    s_MOVED_REPLIES(py_tuple(vec![py_str("Moving is one of the biggest upheavals there is - new room, new noises, a head full of boxes - so this is the usual bill for it. Hold on to your normal routine as far as you can and give it a few weeks."), py_str("A house move unsettles people for a while: everything that used to be familiar is not. That is worth knowing, because it means this is a reaction to a change rather than something wrong with you."), py_str("Nobody sleeps properly in the first weeks of a move - the room, the noises and the unpacking all keep the night shallow. It settles as the place becomes yours.")]));
    s_JOB_REPLIES(py_tuple(vec![py_str("Losing a job is exactly the kind of worry that keeps people awake and on edge, and that is the situation rather than anything wrong with you. Give the routine a few weeks; if it is still bad after a month, take it to a GP."), py_str("A job going is a shock to the routine as much as the money, and it takes weeks for the days to settle again. Keep the wake-up time where it was - it is the part that holds the rest."), py_str("That is a blow, and it lands on sleep and mood first. Being kind about the basics - food, daylight, someone to talk to - does more in the first weeks than trying to fix it all.")]));
    s_BABY_REPLIES(py_tuple(vec![py_str("A baby rewrites your nights, and that is the baby rather than a fault in you. Sleep when you can rather than when it is bedtime, and take any offer of help with the nights while it is offered."), py_str("The first months with a baby are broken nights by design, so this is the shape of it rather than something to fix. If it is still this bad once they sleep through, then it is worth looking at yours.")]));
    s_DIED_REPLIES(py_tuple(vec![py_str("Losing someone close changes sleep and mood for months, and that is grief doing its work rather than a fault in you. Be gentle with the basics, and tell your GP if it is not shifting - they can help without turning it into a diagnosis."), py_str("That kind of loss sits in the body as well as the head, so broken sleep and a low week are the usual shape of it. Company and daylight help more than anything clever, and a GP is there if it stays bad.")]));
    s_SPLIT_REPLIES(py_tuple(vec![py_str("A break-up turns everything over, and sleep and mood are the first things to go with it. It eases as the new shape of the days settles - which it does, though not in a straight line."), py_str("That is the sort of upheaval that keeps people awake and swinging between angry and low. Keep the routine steady where you can; the rest catches up.")]));
    s_MONEY_REPLIES(py_tuple(vec![py_str("Money worry is the kind that keeps people awake at 3am, and it is the worry rather than the sum that does it. Getting the numbers written down in one place usually makes the night quieter, and the free debt advice services are there for exactly this."), py_str("That is a real weight, and it sits on sleep before it sits on anything else. One thing at a time - the smallest bill or the first call - is worth more than a plan you cannot start.")]));
    s_OTHER_REPLIES(py_tuple(vec![py_str("That is the sort of thing that would do it - a change like that unsettles sleep and mood for a few weeks before it settles."), py_str("That would unsettle anyone for a while, and it is the change rather than you. Give it a few weeks and keep the routine as steady as you can.")]));
    s_BODY_REPLIES(py_tuple(vec![py_str("That is the usual way to do it - a day of lifting, twisting and carrying is harder on a joint than it looks. Keep moving gently rather than resting it, and it usually eases over a couple of weeks."), py_str("Strains like that come from exactly this kind of day, and the muscle settles faster if you keep gently active than if you sit still. Warmth and a painkiller are fine while it does.")]));
    s_LENGTH_REPLIES(py_tuple(vec![py_str("This long is long enough that waiting for it to pass is not a plan: if it has not eased by now, that is worth taking to a GP, and saying when it started is the useful bit."), py_str("Since it has been going on this long, it is worth asking someone rather than sitting with it - a GP can check the ordinary causes, and being able to say how long it has been going on helps.")]));
    s_BY_CAUSE(py_dict(vec![(py_str("move"), g_MOVED_REPLIES()), (py_str("split"), g_SPLIT_REPLIES()), (py_str("money"), g_MONEY_REPLIES())]));
    s_LEAVE_IT(py_tuple(vec![py_str("job"), py_str("baby"), py_str("died")]));
    s_LONG_ENOUGH(Py::Int(14));
    s_UNIT_DAYS(py_dict(vec![(py_str("day"), Py::Int(1)), (py_str("days"), Py::Int(1)), (py_str("week"), Py::Int(7)), (py_str("weeks"), Py::Int(7)), (py_str("month"), Py::Int(30)), (py_str("months"), Py::Int(30)), (py_str("year"), Py::Int(365)), (py_str("years"), Py::Int(365))]));
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    return mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]);
    Py::None
}
pub fn _pattern(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let _ = py_str("The regex itself: the tables above are written as one-item tuples, like the other readers.");
    return if py_truthy(&call_builtin("isinstance", &[found.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { geti(&found.clone(), &Py::Int(0)) } else { found.clone() };
    Py::None
}
pub fn _days(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut count: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How long, in days, the length in the line is worth.");
    word = stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    count = mcall(&g_NUMBER_WORDS(), "get", &[word.clone()]);
    if py_truthy(&Py::Bool(py_is(&count.clone(), &Py::None))) {
    let __try1 = exc::catch(|| {
    count = call_builtin("int", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    return mul(&count.clone(), &mcall(&g_UNIT_DAYS(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(2)]), Py::Int(1)]));
    Py::None
}
pub fn _clause(args: &[Py]) -> Py {
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
    let _ = py_str("The words after the verb of beginning, or \"\" when the line has no onset frame.");
    found = mcall(&g_ONSET(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    return mcall(&slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .,!;:-")]);
    Py::None
}
pub fn reads_as_onset(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let mut length: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("(\"move\"|\"job\"|\"baby\"|\"died\"|\"split\"|\"money\"|\"time\"|\"other\", days or None) or None.\n\n    None is the answer for every line that is not an onset statement.\n    ");
    clean = _clean(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    clause = _clause(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return Py::None;
    }
    found = mcall(&g_ONSET(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&found.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&found.clone(), "end", &[]), &mcall(&found.clone(), "start", &[]))) } }) {
    unit = mcall(&g_BARE_UNIT(), "match", &[clause.clone()]);
    if py_truthy(&unit.clone()) {
    return py_tuple(vec![py_str("time"), mcall(&g_UNIT_DAYS(), "get", &[mcall(&unit.clone(), "group", &[Py::Int(1)]), Py::Int(1)])]);
    }
    length = mcall(&g_LENGTH(), "search", &[clause.clone()]);
    if py_truthy(&{ let __b = length.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_days(&[length.clone()]), &Py::None)) } }) {
    return py_tuple(vec![py_str("time"), _days(&[length.clone()])]);
    }
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_CAUSES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[_pattern(&[pattern.clone()]), clause.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![name.clone(), Py::None]);
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[_pattern(&[g_NAMED()]), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![py_str("time"), g_NAMED_DAYS()]);
    }
    unit = mcall(&g_BARE_UNIT(), "match", &[clause.clone()]);
    if py_truthy(&unit.clone()) {
    return py_tuple(vec![py_str("time"), mcall(&g_UNIT_DAYS(), "get", &[mcall(&unit.clone(), "group", &[Py::Int(1)]), Py::Int(1)])]);
    }
    length = mcall(&g_LENGTH(), "search", &[clause.clone()]);
    if py_truthy(&{ let __b = length.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_days(&[length.clone()]), &Py::None)) } }) {
    return py_tuple(vec![py_str("time"), _days(&[length.clone()])]);
    }
    return Py::None;
    Py::None
}
pub fn _body_before(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the line before this one was about a body, not a life.");
    let __try1 = exc::catch(|| {
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    said = _clean(&[mcall(&{ let __b = geti(&turns.clone(), &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")])]);
    return call_builtin("bool", &[stdlib_call("re", "search", &[_pattern(&[g_BODY()]), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_person_before(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the line before this one was about the person themselves, not about a thing.");
    let __try1 = exc::catch(|| {
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    said = _clean(&[mcall(&{ let __b = geti(&turns.clone(), &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = stdlib_call("re", "search", &[_pattern(&[g_BODY()]), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[_pattern(&[g_THE_PERSON()]), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _answered_before(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot has just answered something: a card, not a nod.");
    let __try1 = exc::catch(|| {
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return ge(&call_builtin("len", &[mcall(&said.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(60));
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    let mut cause: Py = Py::None;
    let mut days: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut reading: Py = Py::None;
    let _ = py_str("The answer to an onset statement about the thing just discussed, or None to pass.");
    reading = reads_as_onset(&[line.clone()]);
    if py_truthy(&Py::Bool(py_is(&reading.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_answered_before(&[memory.clone()])))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&reading.clone(), 2, None);
    cause = __unpk1[0].clone();
    days = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_LEAVE_IT(), &cause.clone()))) {
    return Py::None;
    }
    if py_truthy(&_body_before(&[memory.clone()])) {
    return if py_truthy(&Py::Bool(!py_eq(&cause.clone(), &py_str("time")))) { crate::gen::compose::pick(&[g_BODY_REPLIES(), memory.clone(), line.clone()]) } else { Py::None };
    }
    if py_truthy(&Py::Bool(py_eq(&cause.clone(), &py_str("time")))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&days.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&days.clone(), &g_LONG_ENOUGH()) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _body_before(&[memory.clone()]); if py_truthy(&__b) { __b } else { _the_person_before(&[memory.clone()]) } }))) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[g_LENGTH_REPLIES(), memory.clone(), line.clone()]);
    }
    pool = { let __b = mcall(&g_BY_CAUSE(), "get", &[cause.clone()]); if py_truthy(&__b) { __b } else { g_OTHER_REPLIES() } };
    return crate::gen::compose::pick(&[pool.clone(), memory.clone(), line.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.onset".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BABY" => Some(g_BABY()),
            "BABY_REPLIES" => Some(g_BABY_REPLIES()),
            "BARE_UNIT" => Some(g_BARE_UNIT()),
            "BODY" => Some(g_BODY()),
            "BODY_REPLIES" => Some(g_BODY_REPLIES()),
            "BY_CAUSE" => Some(g_BY_CAUSE()),
            "CAUSES" => Some(g_CAUSES()),
            "DIED" => Some(g_DIED()),
            "DIED_REPLIES" => Some(g_DIED_REPLIES()),
            "JOB_LOST" => Some(g_JOB_LOST()),
            "JOB_REPLIES" => Some(g_JOB_REPLIES()),
            "LEAVE_IT" => Some(g_LEAVE_IT()),
            "LENGTH" => Some(g_LENGTH()),
            "LENGTH_REPLIES" => Some(g_LENGTH_REPLIES()),
            "LONG_ENOUGH" => Some(g_LONG_ENOUGH()),
            "MONEY" => Some(g_MONEY()),
            "MONEY_REPLIES" => Some(g_MONEY_REPLIES()),
            "MOVED" => Some(g_MOVED()),
            "MOVED_REPLIES" => Some(g_MOVED_REPLIES()),
            "NAMED" => Some(g_NAMED()),
            "NAMED_DAYS" => Some(g_NAMED_DAYS()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "ONSET" => Some(g_ONSET()),
            "OTHER" => Some(g_OTHER()),
            "OTHER_REPLIES" => Some(g_OTHER_REPLIES()),
            "SPLIT" => Some(g_SPLIT()),
            "SPLIT_REPLIES" => Some(g_SPLIT_REPLIES()),
            "THE_PERSON" => Some(g_THE_PERSON()),
            "UNIT_DAYS" => Some(g_UNIT_DAYS()),
            "_answered_before" => Some(py_func("_answered_before", |args: &[Py]| _answered_before(args))),
            "_body_before" => Some(py_func("_body_before", |args: &[Py]| _body_before(args))),
            "_clause" => Some(py_func("_clause", |args: &[Py]| _clause(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_days" => Some(py_func("_days", |args: &[Py]| _days(args))),
            "_pattern" => Some(py_func("_pattern", |args: &[Py]| _pattern(args))),
            "_the_person_before" => Some(py_func("_the_person_before", |args: &[Py]| _the_person_before(args))),
            "reads_as_onset" => Some(py_func("reads_as_onset", |args: &[Py]| reads_as_onset(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            _ => None,
        }),
        set: None,
    }))
}
