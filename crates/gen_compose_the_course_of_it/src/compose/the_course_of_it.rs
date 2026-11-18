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


thread_local! { static G_ACK_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD_IN_THE_CHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD_PROSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD_PROSE_UNTIMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_PROSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COURSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISTRESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOES_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EASING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_BIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IT_IS_HOW_IT_FEELS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_UP_UNTIMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LASTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDINARY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECOVERY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_STANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_STANDS_UNTIMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_IN_THE_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WISH_TO_DIE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.the_course_of_it: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ACK_LEAD() -> Py { ensure_init(); G_ACK_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK_LEAD(v: Py) { ensure_init(); G_ACK_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_AGE() -> Py { ensure_init(); G_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AGE(v: Py) { ensure_init(); G_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY() -> Py { ensure_init(); G_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); G_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD_IN_THE_CHAT() -> Py { ensure_init(); G_CARD_IN_THE_CHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD_IN_THE_CHAT(v: Py) { ensure_init(); G_CARD_IN_THE_CHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD_PROSE() -> Py { ensure_init(); G_CARD_PROSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD_PROSE(v: Py) { ensure_init(); G_CARD_PROSE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD_PROSE_UNTIMED() -> Py { ensure_init(); G_CARD_PROSE_UNTIMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD_PROSE_UNTIMED(v: Py) { ensure_init(); G_CARD_PROSE_UNTIMED.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE() -> Py { ensure_init(); G_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE(v: Py) { ensure_init(); G_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_PROSE() -> Py { ensure_init(); G_CARE_PROSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_PROSE(v: Py) { ensure_init(); G_CARE_PROSE.with(|c| *c.borrow_mut() = v); }
pub fn g_COURSE() -> Py { ensure_init(); G_COURSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COURSE(v: Py) { ensure_init(); G_COURSE.with(|c| *c.borrow_mut() = v); }
pub fn g_DISTRESS() -> Py { ensure_init(); G_DISTRESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISTRESS(v: Py) { ensure_init(); G_DISTRESS.with(|c| *c.borrow_mut() = v); }
pub fn g_DOES_IT() -> Py { ensure_init(); G_DOES_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOES_IT(v: Py) { ensure_init(); G_DOES_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_EASES() -> Py { ensure_init(); G_EASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASES(v: Py) { ensure_init(); G_EASES.with(|c| *c.borrow_mut() = v); }
pub fn g_EASING() -> Py { ensure_init(); G_EASING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASING(v: Py) { ensure_init(); G_EASING.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_BIT() -> Py { ensure_init(); G_FIRST_BIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_BIT(v: Py) { ensure_init(); G_FIRST_BIT.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAVY() -> Py { ensure_init(); G_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAVY(v: Py) { ensure_init(); G_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g_IT_IS_HOW_IT_FEELS() -> Py { ensure_init(); G_IT_IS_HOW_IT_FEELS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IT_IS_HOW_IT_FEELS(v: Py) { ensure_init(); G_IT_IS_HOW_IT_FEELS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_UP() -> Py { ensure_init(); G_KEEP_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_UP(v: Py) { ensure_init(); G_KEEP_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_UP_UNTIMED() -> Py { ensure_init(); G_KEEP_UP_UNTIMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_UP_UNTIMED(v: Py) { ensure_init(); G_KEEP_UP_UNTIMED.with(|c| *c.borrow_mut() = v); }
pub fn g_LASTS() -> Py { ensure_init(); G_LASTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LASTS(v: Py) { ensure_init(); G_LASTS.with(|c| *c.borrow_mut() = v); }
pub fn g_MED() -> Py { ensure_init(); G_MED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MED(v: Py) { ensure_init(); G_MED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_NAMED() -> Py { ensure_init(); G_NOTHING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED(v: Py) { ensure_init(); G_NOTHING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_SUBJECT() -> Py { ensure_init(); G_NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_SUBJECT(v: Py) { ensure_init(); G_NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDINARY() -> Py { ensure_init(); G_ORDINARY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDINARY(v: Py) { ensure_init(); G_ORDINARY.with(|c| *c.borrow_mut() = v); }
pub fn g_PROSE() -> Py { ensure_init(); G_PROSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROSE(v: Py) { ensure_init(); G_PROSE.with(|c| *c.borrow_mut() = v); }
pub fn g_RECOVERY() -> Py { ensure_init(); G_RECOVERY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECOVERY(v: Py) { ensure_init(); G_RECOVERY.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_STANDS() -> Py { ensure_init(); G_STILL_STANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_STANDS(v: Py) { ensure_init(); G_STILL_STANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_STANDS_UNTIMED() -> Py { ensure_init(); G_STILL_STANDS_UNTIMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_STANDS_UNTIMED(v: Py) { ensure_init(); G_STILL_STANDS_UNTIMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THING() -> Py { ensure_init(); G_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING(v: Py) { ensure_init(); G_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_IN_THE_CARD() -> Py { ensure_init(); G_TIME_IN_THE_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_IN_THE_CARD(v: Py) { ensure_init(); G_TIME_IN_THE_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_WISH_TO_DIE() -> Py { ensure_init(); G_WISH_TO_DIE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WISH_TO_DIE(v: Py) { ensure_init(); G_WISH_TO_DIE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A feeling asked about how long it lasts: \"does it get easier\", \"will i ever feel normal again\".\n\nNine of fifteen turns of this shape in a live probe reached a search hedge:\n\n    does it get easier             -> \"I don't know enough to answer that honestly. A search\n    does that feeling ever go away ->  should sort you out quickly\"\n    will i ever feel normal again  -> \"That's beyond what I know well, sorry.\"\n\nA search cannot answer it. Nobody can put a date on grief or a breakup, and any page that claims\nto is selling something. What a friend says is honest - no date - and then gives the shape of it:\nit lifts in patches before it lifts properly, the bad days get further apart before they get\nshorter, a bad day after a good week is not a sign of going backwards. Then one short question, so\nthe person can say where it hurts most.\n\nWhat is not this: a course question with a body or a medicine in it (a rash, a cough, the tablets,\nhow long the antibiotics take) belongs to the health cards and to the safety lane, a thing with a\nmechanism in it (the boiler, the car, the wifi) belongs to the knowledge base, and a bare \"how\nlong does it last\" with nothing behind it is too vague to answer at all.\n");
    s_HEAVY(call_builtin("frozenset", &[mcall(&py_str("\nfeel feeling feelings felt sad sadness sadder grief grieving heartbroken broken heartsick lonely\nloneliness alone guilt guilty ashamed shame anxious anxiety anxiousness depressed depression\npanic unhappy happy happiness normal myself missing miss loss empty hopeless worthless numb\ncrying cry tears raw hurting hurts hurt heavy\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY(call_builtin("frozenset", &[mcall(&py_str("\nrash cough cold flu virus infection temperature fever spot spots itch itching lump bruise ache\naches pain painful headache migraine tooth teeth gum gums back knee hip shoulder neck ankle wrist\nsickness nausea dizzy dizziness diarrhoea constipation blister burn cut wound swelling stiff\nstiffness allergy allergic symptoms symptom period periods pregnant pregnancy\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MED(call_builtin("frozenset", &[mcall(&py_str("\nmeds medication medicine medicines tablets tablet pill pills dose dosage antibiotic antibiotics\nantidepressant antidepressants sertraline citalopram prozac prescription prescribed treatment\nchemotherapy withdrawal\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RECOVERY(call_builtin("frozenset", &[mcall(&py_str("\nsection caesarean cesarean csection surgery operation op ops operation hospital hospitalised\nhospitalized recovery recovering recovered convalescing stitches stitch physio physiotherapy\nchemo radiotherapy transplant bypass hysterectomy appendectomy appendix labour labor delivery\nbirth births fall falls fracture fractured fractures sprain sprains\ndischarge ward icu catheter cast plaster\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING(call_builtin("frozenset", &[mcall(&py_str("\nboiler car engine tyre tyres mot wifi internet broadband router phone laptop computer printer\noven washing machine dishwasher fridge freezer heating radiator leak tap roof gutter damp mould\nnoise neighbour neighbors traffic rain weather winter summer sale offer subscription contract\nmortgage rent landlord job shift rota school term exam\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COURSE(stdlib_call("re", "compile", &[py_str("\\b(?:get(?:ting)?\\s+back\\s+to\\s+normal|get(?:ting)?\\s+(?:easier|any\\s+easier|better|any\\s+better)|feel(?:ing)?\\s+(?:like\\s+myself|normal|better|happy|ok|okay|fine|human)\\s+again|be\\s+(?:happy|ok|okay|normal|fine|myself)\\s+again|go(?:es|ing)?\\s+away|went\\s+away|gone\\s+away|ever\\s+go\\b|get(?:ting)?\\s+over\\s+(?:it|this|that|him|her|them|everything)|feel\\s+like\\s+this|(?:this|it|that)\\s+(?:be\\s+)?forever|(?:this|it|that)\\s+(?:ever\\s+)?(?:last|lasts|lasted)\\b|(?:ever\\s+)?(?:going\\s+to\\s+)?(?:stop|end|ease|eases|settle|settles|pass|passes|lift|lifts)|(?:always\\s+|forever\\s+|still\\s+)?(?:going\\s+to\\s+)?be\\s+like\\s+this|be\\s+over)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS(stdlib_call("re", "compile", &[py_str("^\\s*(?:so\\s+|but\\s+|and\\s+)?(?:when|does|do|will|would|is|are|am|can|could|how\\s+long)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARD_IN_THE_CHAT(py_str("the card in the chat"));
    s_NO_SUBJECT(py_str("no subject yet"));
    s_IT_IS_HOW_IT_FEELS(call_builtin("frozenset", &[mcall(&py_str("\neasier better cope coping manage managing deal dealing normal myself fine alright happy ok okay\nheal healing recover recovering recoverd over through move moving onwards forward bearable\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_NAMED(py_tuple(vec![py_str("It depends what it is: a cough, a bad patch of sleep and a row at work all run on their own clock. Tell me the thing and I will say what usually happens with it."), py_str("Nobody can say how long that lasts without knowing what it is. Which is it - something with your body, someone at home, or a job?"), py_str("How long that goes on for comes from what it is, and I do not know yet. Say what it refers to and I will tell you what usually happens.")]));
    s_LASTS(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\s+(?:does|do|will|did|would)\\b.*\\b(?:last|take|go\\s+on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DISTRESS(call_builtin("frozenset", &[mcall(&py_str("\ncut cuts cutting cut myself self harmed harm harming suicide suicidal kill myself overdose\noverdosed overdosing sectioned blades blade scars scarred burning starving starved purging\nrelapsed relapse\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDINARY(call_builtin("frozenset", &[mcall(&py_str("\nbaby babies newborn newborns teething toddler toddlers nappy nappies formula feeding breastfed\nsleepless knackered tired exhausted shifts shift rota rotas nights overtime commute gym diet\nrunning marathon training weight old age ageing aging birthday birthdays\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE(call_builtin("frozenset", &[mcall(&py_str("\nlost lose losing died death dead funeral buried miscarriage miscarried separating separated\nseparation divorce divorced redundant redundancy sacked breakup broke broken cancer tumour\ndiagnosis diagnosed hospital hospice chemo stillbirth\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_PROSE(py_tuple(vec![py_str("I'm sorry - that's a heavy thing to be carrying. There is no date for it and I would be making one up, but it does come in patches rather than one long slope: a bearable hour, then a day that feels like the start again, and the bearable stretches get longer first. What helps early on is company and a shape to the day - someone who will sit with you, and one thing you do at the same time each day. Cruse takes calls from anyone who has lost someone, and your GP will see you about it too."), py_str("That is a lot to hold, and I'm sorry. Nobody can tell you when it will feel lighter, and a guess would not help you. What tends to happen is that the good stretches come first, before the bad days get shorter, so a bad afternoon after a decent week is not a step back. Keep one fixed thing in the week and let people do things for you rather than asking; if it is grief, Cruse's line is there for exactly this, and your GP is the other door.")]));
    s_PROSE(py_tuple(vec![py_str("Nobody can put a date on that, and I would be making it up if I did. It tends to lift in patches before it lifts properly - an ordinary afternoon, then a day that feels like the start again - and a bad day after a good week is not you going backwards. Two things that help meanwhile: one fixed thing in the week you would turn up to anyway, and one person who hears the honest version. If it is still spoiling your sleep or your appetite after a few weeks, that is worth saying to a GP."), py_str("It does ease for most people, but not on a timetable, so any date would be a guess. The bad days usually get further apart before they get shorter, which is worth knowing on a day that feels like week one again. Until then: keep one routine you do not have to think about, get outside most days, and tell one person how it actually is. If there is nobody to tell, your GP is the place to start, and so is a group of people who have been through the same thing."), py_str("I can't tell you when, and anyone who can is guessing. What I can say is that it comes in patches rather than a steady line, and the patches get shorter - the stretch between bad days widens first. What moves it along is usually other people and a bit of structure rather than a lot of thinking: a walk with someone, a class, a fixed Sunday call. Your GP is the right next stop if it is your mood as much as the loss.")]));
    s_CARD_PROSE(py_tuple(vec![py_str("Nobody can put a date on that one, sorry - it answers to time rather than to anything I can look up. What shifts it is the everyday bit: {bit}. If it has not moved after a couple of weeks, that is the point to ask someone who can see the whole picture."), py_str("I can't tell you when, and a guess would not help. The part that does the work is the everyday bit: {bit}. See where it is in a couple of weeks."), py_str("There is no date for that, sorry. What moves it is what the first answer said: {bit}. If it is no different in a couple of weeks, that is when to ask someone who knows the case.")]));
    s_CARD_PROSE_UNTIMED(py_tuple(vec![py_str("There is no date for that one, and I would be making one up. What the answer was pointing at is still where to put the effort: {bit}. Has it moved at all since?"), py_str("Nobody can say when, sorry, and a made-up deadline would not help you. What the first answer said is the part to go on: {bit}. How long has it been going on?"), py_str("I can't put a time on it. The part that does the work is what was already said: {bit}. What has changed since it started?")]));
    s_EASES(py_tuple(vec![py_str("It gets easier for most people, and how long that takes varies - a couple of months is not unusual. What the answer said is the part that does the work: {bit}. Does it feel any different from the first week?"), py_str("It usually eases before it stops, and the easing is rarely in a straight line. Keep the thing the card named up: {bit}. Has it got any easier yet?"), py_str("That usually does get easier with time, though not evenly. The bit worth keeping up is what was said: {bit}. Has it eased at all yet?")]));
    s_DOES_IT(call_builtin("frozenset", &[mcall(&py_str("\ntreat keep put believe water ask give use take call try make add move leave stop ring check measure\ntell say write get go see book buy cut clean feed walk watch brush cover tie hold run set turn\nstart begin finish test watch wrap\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEP_UP(py_tuple(vec![py_str("There is no date for that, sorry. What is worth keeping up in the meantime: {bit}. If it has not changed in a couple of weeks, that is the point to look at one thing and change it."), py_str("I can't say when it will stop, and nobody could. The bit that does the work is the everyday one: {bit}. Give that a couple of weeks before you judge it."), py_str("No date on that one, sorry. Keep the everyday bit up - {bit} - and watch for it moving over the next couple of weeks.")]));
    s_KEEP_UP_UNTIMED(py_tuple(vec![py_str("There is no date for that, sorry. The everyday bit is the part that does the work: {bit}. Keep that going and see what changes."), py_str("I can't say when it stops, and nobody could. What is worth doing in the meantime: {bit}. What has changed so far?"), py_str("No date on that one, sorry. Keep the everyday bit up - {bit} - and watch for the first sign of it moving.")]));
    s_STILL_STANDS(py_tuple(vec![py_str("No date on that one, sorry. What the first answer said still stands: {bit}. If it is no different in a couple of weeks, that is the point to change one thing."), py_str("I can't put a date on it. The facts have not moved since the first answer: {bit}. Give it a couple of weeks before you judge it."), py_str("Nobody can say when, sorry. What the first answer said holds: {bit}. If nothing has changed in a couple of weeks, that is the moment to look at one thing.")]));
    s_STILL_STANDS_UNTIMED(py_tuple(vec![py_str("There is no date for that, and I would not invent one. What the first answer said is where to start: {bit}. Has it moved at all since?"), py_str("I can't say when, sorry. The part to go on is this: {bit}. How long has it been like this?"), py_str("Nobody can promise when, and a guess would not help. What the first answer said still stands: {bit}. What has changed since?")]));
    s_TIME_IN_THE_CARD(stdlib_call("re", "compile", &[py_str("\\b(?:hours?|days?|nights?|weeks?|months?|fortnight|how\\s+long|heals?|clears?|settles?|eases?|improves?|by\\s+the\\s+end)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EASING(stdlib_call("re", "compile", &[py_str("\\b(?:easier|better|improv\\w*|bearable|less\\s+often)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ACK_LEAD(stdlib_call("re", "compile", &[py_str("^\\s*(?:got\\s+it|noted|right|okay|ok|thanks\\s+for\\s+telling\\s+me|fair\\s+enough|no\\s+problem|understood|sure|cheers)\\b[^.!?]{0,40}?[.:;,\\u2014\\u2013-]\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_BIT(stdlib_call("re", "compile", &[py_str("(?<=[.;:])\\s+|\\s+[\\u2014\\u2013]\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_WISH_TO_DIE(stdlib_call("re", "compile", &[py_str("\\b(?:want|wanna|wanting)\\s+(?:to\\s+)?die\\b|\\bkill\\s+myself\\b|\\bend\\s+it\\s+all\\b|\\bno\\s+(?:point|reason)\\s+(?:in\\s+)?(?:living|going\\s+on|carrying\\s+on)\\b|\\bbetter\\s+off\\s+dead\\b|\\bdon'?t\\s+want\\s+to\\s+be\\s+here\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AGE(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m|im|i\\s+am|turn(?:ing|ed)?|about|nearly|almost)\\s+(?:to\\s+be\\s+)?(?:[1-9]\\d)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    return call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&text.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _same_as_the_last_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut written: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "written") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: written"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the card is the answer the person was just given, word for word at the start.");
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(60), &Py::None), &slice_or_raise(&written.clone(), &Py::None, &Py::Int(60), &Py::None)))) {
    return Py::Bool(true);
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_ge(&seen.clone(), &Py::Int(3))) {
    break;
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _an_answer_worth_quoting(args: &[Py]) -> Py {
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
    let mut content: Py = Py::None;
    let _ = py_str("True when a short reply still has an account of the thing in it, not just a nod.");
    content = mcall(&mcall(&g_ACK_LEAD(), "sub", &[py_str(""), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), "strip", &[]);
    return op_ge(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[A-Za-z']+"), content.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5));
    Py::None
}
pub fn _the_card_in_the_chat(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lookup: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut understand: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("What the last answer was about, as the knowledge base puts it, or \"\".\n\n    \"will it last forever\" after the early-waking card, or \"will it stop\" after the puppy-alone\n    card: the chat is on something and the question is about that thing's course. Nothing here can\n    put a date on it, but the card says what moves it, and that is what a friend would say. The\n    card is found from the topic line the chat is on, not from the line itself, because the line\n    names nothing.\n    ");
    turns = py_list(vec![{ let __b = mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&turns.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    let __try2 = exc::catch(|| {
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    lookup = pyrt::xrefs::xmodule("knowledge.lookup");
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
    return py_str("");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    before = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    let __iter7 = py_iter_or_raise(&turns.clone());
    for __item in __iter7 {
    line = __item.clone();
    line = mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    let __try8 = exc::catch(|| {
    reply = call_builtin("getattr", &[mcall(&lookup.clone(), "find", &[py_call(&understand, &[line.clone(), py_dict(vec![(py_str("understood"), before.clone())])])]), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["Exception"]) {
                __handled12 = true;
    continue;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&reply.clone(), "strip", &[]) } }) {
    written = mcall(&reply.clone(), "strip", &[]);
    return py_tuple(vec![written.clone(), if py_truthy(&_same_as_the_last_answer(&[written.clone(), memory.clone()])) { py_str("said") } else { py_str("card") }]);
    }
    }
    let __try13 = exc::catch(|| {
    fallback = pyrt::xrefs::xmodule("compose.fallback");
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret14: Py = Py::Bool(false);
    let mut __val15: Py = Py::None;
    let mut __unhandled16: Option<exc::PyExc> = None;
    match __try13 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret14 = __p[0].clone(); __val15 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled17 = false;
            if !__handled17 && exc::matches_any(&__e, &["Exception"]) {
                __handled17 = true;
    fallback = Py::None;
            }
            if !__handled17 { __unhandled16 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled16 { exc::raise(__e); }
    if py_truthy(&__ret14) { return __val15; }
    let __iter18 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter18 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = mcall(&said.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_answer_worth_quoting(&[said.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&fallback.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&fallback.clone(), "_is_a_hedge", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&fallback.clone(), "_is_an_ask_which_bit", &[said.clone()]) } } } }) {
    continue;
    }
    opening = mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(60), &Py::None), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&opening.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&opening.clone(), &py_str(":"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&opening.clone(), &py_str("tell me"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&opening.clone(), &py_str("say what"))) } } } } } }) {
    continue;
    }
    return py_tuple(vec![said.clone(), py_str("said")]);
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn _the_first_bit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut card: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "card") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: card"))) };
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(150) };
    let mut skip_openers: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip_openers") { v.clone() }
    else { Py::Bool(false) };
    let mut comma: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut first: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The card's first sentence or clause, short enough to quote inside a sentence of our own.\n\n    `skip_openers` is for the bot's own last answer: the first thing it said may be the mood\n    before the help (\"Long morning already. Get outside for ten minutes when you can\"), and\n    quoting that back says nothing. A card's own first sentence is left alone, where a short one\n    can be the whole instruction (\"Water little and often.\").\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = card.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    first = mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIRST_BIT(), "split", args))(&__args) }, &Py::Int(0)), "strip", &[]);
    if py_truthy(&{ let __b = skip_openers.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(30)) } }) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIRST_BIT(), "split", args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { mcall(&geti(&rest.clone(), &Py::Int(1)), "strip", &[]) } }) {
    first = mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mcall(&geti(&rest.clone(), &Py::Int(1)), "strip", &[])); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIRST_BIT(), "split", args))(&__args) }, &Py::Int(0)), "strip", &[]);
    }
    }
    first = mcall(&first.clone(), "rstrip", &[py_str(" .;:")]);
    if py_truthy(&op_gt(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
    cut = slice_or_raise(&first.clone(), &Py::None, &limit.clone(), &Py::None);
    comma = mcall(&cut.clone(), "rfind", &[py_str(", ")]);
    first = if py_truthy(&op_gt(&comma.clone(), &Py::Int(40))) { slice_or_raise(&cut.clone(), &Py::None, &comma.clone(), &Py::None) } else { cut.clone() };
    first = mcall(&first.clone(), "rstrip", &[py_str(" ,;:-")]);
    }
    return first.clone();
    Py::None
}
pub fn _leans_on(args: &[Py]) -> Py {
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
    let mut lean: Py = Py::None;
    let mut understood: Py = Py::None;
    understood = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    lean = { let __b = mcall(&understood.clone(), "get", &[py_str("leans_on")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    return mcall(&py_str(" "), "join", &[{ let __b = mcall(&lean.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    Py::None
}
pub fn _recent(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What was said in the last few turns, so the \"it\" can be read from the thread itself.\n\n    `leans_on` carries the words of one previous line and not always all of them; a thread about\n    self-harm or about a rota has to be visible here whatever the reader above made of it.\n    ");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&text.clone(), &mcall(&mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]))) } }) {
    let _ = mcall(&said.clone(), "append", &[text.clone()]);
    }
    }
    return mcall(&py_str(" "), "join", &[said.clone()]);
    Py::None
}
pub fn reads(args: &[Py]) -> Py {
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
    let mut about_a_hard_time: Py = Py::None;
    let mut asks_about_a_feeling: Py = Py::None;
    let mut card: Py = Py::None;
    let mut heavy: Py = Py::None;
    let mut lasts: Py = Py::None;
    let mut lean: Py = Py::None;
    let mut like_this: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut nothing_named: Py = Py::None;
    let mut ordinary: Py = Py::None;
    let mut source: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The course asked about, or None when this is not the shape.");
    line = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::None;
    }
    words = _words(&[line.clone()]);
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_MED()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &g_BODY()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &g_THING()); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_RECOVERY()) } } } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_COURSE(), "search", &[line.clone()]);
    lasts = mcall(&g_LASTS(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&lasts.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_ASKS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&line.clone(), "endswith", &[py_str("?")]) } }))) {
    return Py::None;
    }
    lean = op_bitor(&_words(&[_leans_on(&[memory.clone()])]), &_words(&[_recent(&[memory.clone(), line.clone()])]));
    if py_truthy(&{ let __b = op_bitand(&op_bitor(&words.clone(), &lean.clone()), &g_DISTRESS()); if py_truthy(&__b) { __b } else { mcall(&g_WISH_TO_DIE(), "search", &[op_add(&op_add(&line.clone(), &py_str(" ")), &_recent(&[memory.clone(), line.clone()]))]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_HEAVY())))) {
    let __unpk1 = py_unpack(&_the_card_in_the_chat(&[memory.clone()]), 2, None);
    card = __unpk1[0].clone();
    source = __unpk1[1].clone();
    if py_truthy(&card.clone()) {
    return op_add(&op_add(&g_CARD_IN_THE_CHAT(), &py_str(":")), &source.clone());
    }
    asks_about_a_feeling = call_builtin("bool", &[op_bitand(&words.clone(), &g_IT_IS_HOW_IT_FEELS())]).unwrap_or_else(|e| exc::raise(e));
    about_a_hard_time = { let __b = call_builtin("bool", &[op_bitand(&lean.clone(), &g_HEAVY())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[op_bitand(&lean.clone(), &py_set(vec![py_str("died"), py_str("death"), py_str("funeral"), py_str("loss")]))]).unwrap_or_else(|e| exc::raise(e)) } };
    like_this = { let __b = call_builtin("bool", &[match_.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[]), &py_str("like this"))) } };
    nothing_named = op_lt(&call_builtin("len", &[mcall(&_recent(&[memory.clone(), line.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3));
    if py_truthy(&{ let __b = nothing_named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asks_about_a_feeling.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&about_a_hard_time.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&like_this.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow\\s+long\\b|\\bhow\\s+much\\b|\\bwhat\\s+time\\b"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)))) } } } } } } } }) {
    return g_NO_SUBJECT();
    }
    }
    like_this = { let __b = call_builtin("bool", &[match_.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[]), &py_str("like this"))) } };
    heavy = { let __b = like_this.clone(); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[op_bitand(&words.clone(), &g_HEAVY())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("easier")]))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[op_bitand(&lean.clone(), &g_HEAVY())]).unwrap_or_else(|e| exc::raise(e)) } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&heavy.clone()))) {
    return Py::None;
    }
    ordinary = { let __b = call_builtin("bool", &[op_bitand(&op_bitor(&words.clone(), &lean.clone()), &g_ORDINARY())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_AN_AGE(), "search", &[op_add(&op_add(&line.clone(), &py_str(" ")), &_recent(&[memory.clone(), line.clone()]))])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = ordinary.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&op_bitor(&words.clone(), &lean.clone()), &g_CARE()))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&lean.clone(), &g_RECOVERY()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&lean.clone(), &g_CARE()))) } }) {
    return Py::None;
    }
    understood = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = mcall(&understood.clone(), "get", &[py_str("body")]); if py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("about_bot")]) } }) {
    return Py::None;
    }
    return mcall(&mcall(&if py_truthy(&match_.clone()) { mcall(&match_.clone(), "group", &[Py::Int(0)]) } else { mcall(&lasts.clone(), "group", &[Py::Int(0)]) }, "strip", &[]), "lower", &[]);
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
    else { Py::None };
    let mut asked: Py = Py::None;
    let mut c: Py = Py::None;
    let mut card: Py = Py::None;
    let mut first: Py = Py::None;
    let mut lean: Py = Py::None;
    let mut pick: Py = Py::None;
    let mut said: Py = Py::None;
    let mut source: Py = Py::None;
    let mut timed: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("One honest answer about the course of it: no date, the shape, and one question.");
    asked = reads(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&asked.clone(), &g_NO_SUBJECT()))) {
    said = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    pick = op_modulo(&op_add(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &op_mul(&Py::Int(5), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))), &call_builtin("len", &[g_NOTHING_NAMED()]).unwrap_or_else(|e| exc::raise(e)));
    return geti(&g_NOTHING_NAMED(), &pick.clone());
    }
    if py_truthy(&mcall(&asked.clone(), "startswith", &[g_CARD_IN_THE_CHAT()])) {
    let __unpk1 = py_unpack(&_the_card_in_the_chat(&[memory.clone()]), 2, None);
    whole = __unpk1[0].clone();
    source = __unpk1[1].clone();
    card = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); __kws.extend(vec![("skip_openers".to_string(), Py::Bool(py_eq(&source.clone(), &py_str("said"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_first_bit(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&card.clone()))) {
    return Py::None;
    }
    said = op_add(&op_add(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &py_str("|")), &card.clone());
    pick = op_modulo(&op_add(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &op_mul(&Py::Int(7), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))), &Py::Int(3));
    timed = call_builtin("bool", &[mcall(&g_TIME_IN_THE_CARD(), "search", &[whole.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_EASING(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("bit".to_string(), card.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_EASES(), &pick.clone()), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&source.clone(), &py_str("card")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("bit".to_string(), card.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&if py_truthy(&timed.clone()) { g_CARD_PROSE() } else { g_CARD_PROSE_UNTIMED() }, &pick.clone()), "format", args))(&__args) };
    }
    first = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&card.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = first.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DOES_IT(), &geti(&first.clone(), &Py::Int(0)))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("bit".to_string(), card.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&if py_truthy(&timed.clone()) { g_KEEP_UP() } else { g_KEEP_UP_UNTIMED() }, &pick.clone()), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("bit".to_string(), card.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&if py_truthy(&timed.clone()) { g_STILL_STANDS() } else { g_STILL_STANDS_UNTIMED() }, &pick.clone()), "format", args))(&__args) };
    }
    said = op_add(&op_add(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &py_str("|")), &_leans_on(&[memory.clone()]));
    pick = op_modulo(&op_add(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &op_mul(&Py::Int(7), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))), &call_builtin("len", &[g_PROSE()]).unwrap_or_else(|e| exc::raise(e)));
    lean = _words(&[_leans_on(&[memory.clone()])]);
    if py_truthy(&op_bitand(&op_bitor(&_words(&[text.clone()]), &lean.clone()), &g_CARE())) {
    return geti(&g_CARE_PROSE(), &op_modulo(&pick.clone(), &call_builtin("len", &[g_CARE_PROSE()]).unwrap_or_else(|e| exc::raise(e))));
    }
    return geti(&g_PROSE(), &pick.clone());
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.the_course_of_it".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACK_LEAD" => Some(g_ACK_LEAD()),
            "AN_AGE" => Some(g_AN_AGE()),
            "ASKS" => Some(g_ASKS()),
            "BODY" => Some(g_BODY()),
            "CARD_IN_THE_CHAT" => Some(g_CARD_IN_THE_CHAT()),
            "CARD_PROSE" => Some(g_CARD_PROSE()),
            "CARD_PROSE_UNTIMED" => Some(g_CARD_PROSE_UNTIMED()),
            "CARE" => Some(g_CARE()),
            "CARE_PROSE" => Some(g_CARE_PROSE()),
            "COURSE" => Some(g_COURSE()),
            "DISTRESS" => Some(g_DISTRESS()),
            "DOES_IT" => Some(g_DOES_IT()),
            "EASES" => Some(g_EASES()),
            "EASING" => Some(g_EASING()),
            "FIRST_BIT" => Some(g_FIRST_BIT()),
            "HEAVY" => Some(g_HEAVY()),
            "IT_IS_HOW_IT_FEELS" => Some(g_IT_IS_HOW_IT_FEELS()),
            "KEEP_UP" => Some(g_KEEP_UP()),
            "KEEP_UP_UNTIMED" => Some(g_KEEP_UP_UNTIMED()),
            "LASTS" => Some(g_LASTS()),
            "MED" => Some(g_MED()),
            "NOTHING_NAMED" => Some(g_NOTHING_NAMED()),
            "NO_SUBJECT" => Some(g_NO_SUBJECT()),
            "ORDINARY" => Some(g_ORDINARY()),
            "PROSE" => Some(g_PROSE()),
            "RECOVERY" => Some(g_RECOVERY()),
            "STILL_STANDS" => Some(g_STILL_STANDS()),
            "STILL_STANDS_UNTIMED" => Some(g_STILL_STANDS_UNTIMED()),
            "THING" => Some(g_THING()),
            "TIME_IN_THE_CARD" => Some(g_TIME_IN_THE_CARD()),
            "WISH_TO_DIE" => Some(g_WISH_TO_DIE()),
            "_an_answer_worth_quoting" => Some(py_func("_an_answer_worth_quoting", |args: &[Py]| _an_answer_worth_quoting(args))),
            "_leans_on" => Some(py_func("_leans_on", |args: &[Py]| _leans_on(args))),
            "_recent" => Some(py_func("_recent", |args: &[Py]| _recent(args))),
            "_same_as_the_last_answer" => Some(py_func("_same_as_the_last_answer", |args: &[Py]| _same_as_the_last_answer(args))),
            "_the_card_in_the_chat" => Some(py_func("_the_card_in_the_chat", |args: &[Py]| _the_card_in_the_chat(args))),
            "_the_first_bit" => Some(py_func("_the_first_bit", |args: &[Py]| _the_first_bit(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACK_LEAD" => s_ACK_LEAD(v),
            "AN_AGE" => s_AN_AGE(v),
            "ASKS" => s_ASKS(v),
            "BODY" => s_BODY(v),
            "CARD_IN_THE_CHAT" => s_CARD_IN_THE_CHAT(v),
            "CARD_PROSE" => s_CARD_PROSE(v),
            "CARD_PROSE_UNTIMED" => s_CARD_PROSE_UNTIMED(v),
            "CARE" => s_CARE(v),
            "CARE_PROSE" => s_CARE_PROSE(v),
            "COURSE" => s_COURSE(v),
            "DISTRESS" => s_DISTRESS(v),
            "DOES_IT" => s_DOES_IT(v),
            "EASES" => s_EASES(v),
            "EASING" => s_EASING(v),
            "FIRST_BIT" => s_FIRST_BIT(v),
            "HEAVY" => s_HEAVY(v),
            "IT_IS_HOW_IT_FEELS" => s_IT_IS_HOW_IT_FEELS(v),
            "KEEP_UP" => s_KEEP_UP(v),
            "KEEP_UP_UNTIMED" => s_KEEP_UP_UNTIMED(v),
            "LASTS" => s_LASTS(v),
            "MED" => s_MED(v),
            "NOTHING_NAMED" => s_NOTHING_NAMED(v),
            "NO_SUBJECT" => s_NO_SUBJECT(v),
            "ORDINARY" => s_ORDINARY(v),
            "PROSE" => s_PROSE(v),
            "RECOVERY" => s_RECOVERY(v),
            "STILL_STANDS" => s_STILL_STANDS(v),
            "STILL_STANDS_UNTIMED" => s_STILL_STANDS_UNTIMED(v),
            "THING" => s_THING(v),
            "TIME_IN_THE_CARD" => s_TIME_IN_THE_CARD(v),
            "WISH_TO_DIE" => s_WISH_TO_DIE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.the_course_of_it", module_obj);
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "_same_as_the_last_answer", py_func("_same_as_the_last_answer", |args: &[Py]| _same_as_the_last_answer(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "_an_answer_worth_quoting", py_func("_an_answer_worth_quoting", |args: &[Py]| _an_answer_worth_quoting(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "_the_card_in_the_chat", py_func("_the_card_in_the_chat", |args: &[Py]| _the_card_in_the_chat(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "_the_first_bit", py_func("_the_first_bit", |args: &[Py]| _the_first_bit(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "_leans_on", py_func("_leans_on", |args: &[Py]| _leans_on(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "_recent", py_func("_recent", |args: &[Py]| _recent(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "reads", py_func("reads", |args: &[Py]| reads(args)));
    pyrt::xrefs::reg_fn("compose.the_course_of_it", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.the_course_of_it", "ACK_LEAD", g_ACK_LEAD, s_ACK_LEAD);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "AN_AGE", g_AN_AGE, s_AN_AGE);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "BODY", g_BODY, s_BODY);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "CARD_IN_THE_CHAT", g_CARD_IN_THE_CHAT, s_CARD_IN_THE_CHAT);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "CARD_PROSE", g_CARD_PROSE, s_CARD_PROSE);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "CARD_PROSE_UNTIMED", g_CARD_PROSE_UNTIMED, s_CARD_PROSE_UNTIMED);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "CARE", g_CARE, s_CARE);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "CARE_PROSE", g_CARE_PROSE, s_CARE_PROSE);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "COURSE", g_COURSE, s_COURSE);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "DISTRESS", g_DISTRESS, s_DISTRESS);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "DOES_IT", g_DOES_IT, s_DOES_IT);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "EASES", g_EASES, s_EASES);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "EASING", g_EASING, s_EASING);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "FIRST_BIT", g_FIRST_BIT, s_FIRST_BIT);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "HEAVY", g_HEAVY, s_HEAVY);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "IT_IS_HOW_IT_FEELS", g_IT_IS_HOW_IT_FEELS, s_IT_IS_HOW_IT_FEELS);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "KEEP_UP", g_KEEP_UP, s_KEEP_UP);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "KEEP_UP_UNTIMED", g_KEEP_UP_UNTIMED, s_KEEP_UP_UNTIMED);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "LASTS", g_LASTS, s_LASTS);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "MED", g_MED, s_MED);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "NOTHING_NAMED", g_NOTHING_NAMED, s_NOTHING_NAMED);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "NO_SUBJECT", g_NO_SUBJECT, s_NO_SUBJECT);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "ORDINARY", g_ORDINARY, s_ORDINARY);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "PROSE", g_PROSE, s_PROSE);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "RECOVERY", g_RECOVERY, s_RECOVERY);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "STILL_STANDS", g_STILL_STANDS, s_STILL_STANDS);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "STILL_STANDS_UNTIMED", g_STILL_STANDS_UNTIMED, s_STILL_STANDS_UNTIMED);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "THING", g_THING, s_THING);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "TIME_IN_THE_CARD", g_TIME_IN_THE_CARD, s_TIME_IN_THE_CARD);
    pyrt::xrefs::reg_access("compose.the_course_of_it", "WISH_TO_DIE", g_WISH_TO_DIE, s_WISH_TO_DIE);
}
