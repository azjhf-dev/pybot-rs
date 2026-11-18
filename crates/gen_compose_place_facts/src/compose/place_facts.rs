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


thread_local! { static G_A_BRING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLOCK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CROWD_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DESTINATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIXTURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FREE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOME_OF_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RUNS_TO_A_CALENDAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STANDING_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SWITCHED_ON_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PLACE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ANIMALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ASKS_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ASKS_WITH_NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_EVENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_FACILITIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_FREE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_IS_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_LETS_ANIMALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ON_A_DAY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_PERMISSION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_VENUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_WITH_A_VENUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.place_facts: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_BRING_ASK() -> Py { ensure_init(); G_A_BRING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BRING_ASK(v: Py) { ensure_init(); G_A_BRING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLOCK_ASK() -> Py { ensure_init(); G_A_CLOCK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLOCK_ASK(v: Py) { ensure_init(); G_A_CLOCK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CROWD_ASK() -> Py { ensure_init(); G_A_CROWD_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CROWD_ASK(v: Py) { ensure_init(); G_A_CROWD_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY() -> Py { ensure_init(); G_A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY(v: Py) { ensure_init(); G_A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DESTINATION() -> Py { ensure_init(); G_A_DESTINATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DESTINATION(v: Py) { ensure_init(); G_A_DESTINATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIXTURE() -> Py { ensure_init(); G_A_FIXTURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIXTURE(v: Py) { ensure_init(); G_A_FIXTURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FREE_ASK() -> Py { ensure_init(); G_A_FREE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FREE_ASK(v: Py) { ensure_init(); G_A_FREE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOME_OF_THEIR_OWN() -> Py { ensure_init(); G_A_HOME_OF_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOME_OF_THEIR_OWN(v: Py) { ensure_init(); G_A_HOME_OF_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RUNS_TO_A_CALENDAR() -> Py { ensure_init(); G_A_RUNS_TO_A_CALENDAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RUNS_TO_A_CALENDAR(v: Py) { ensure_init(); G_A_RUNS_TO_A_CALENDAR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STANDING_DAY() -> Py { ensure_init(); G_A_STANDING_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STANDING_DAY(v: Py) { ensure_init(); G_A_STANDING_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SWITCHED_ON_THING() -> Py { ensure_init(); G_A_SWITCHED_ON_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SWITCHED_ON_THING(v: Py) { ensure_init(); G_A_SWITCHED_ON_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PLACE_SUBJECT() -> Py { ensure_init(); G_BARE_PLACE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PLACE_SUBJECT(v: Py) { ensure_init(); G_BARE_PLACE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_A_DAY() -> Py { ensure_init(); G_ON_A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_A_DAY(v: Py) { ensure_init(); G_ON_A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ANIMALS() -> Py { ensure_init(); G_PLACE_ANIMALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ANIMALS(v: Py) { ensure_init(); G_PLACE_ANIMALS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ASKS() -> Py { ensure_init(); G_PLACE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ASKS(v: Py) { ensure_init(); G_PLACE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ASKS_OF_ITS_OWN() -> Py { ensure_init(); G_PLACE_ASKS_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ASKS_OF_ITS_OWN(v: Py) { ensure_init(); G_PLACE_ASKS_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ASKS_WITH_NO_SUBJECT() -> Py { ensure_init(); G_PLACE_ASKS_WITH_NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ASKS_WITH_NO_SUBJECT(v: Py) { ensure_init(); G_PLACE_ASKS_WITH_NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_EVENTS() -> Py { ensure_init(); G_PLACE_EVENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_EVENTS(v: Py) { ensure_init(); G_PLACE_EVENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_FACILITIES() -> Py { ensure_init(); G_PLACE_FACILITIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_FACILITIES(v: Py) { ensure_init(); G_PLACE_FACILITIES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_FREE_LINES() -> Py { ensure_init(); G_PLACE_FREE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_FREE_LINES(v: Py) { ensure_init(); G_PLACE_FREE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_IS_THERE() -> Py { ensure_init(); G_PLACE_IS_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_IS_THERE(v: Py) { ensure_init(); G_PLACE_IS_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_LETS_ANIMALS() -> Py { ensure_init(); G_PLACE_LETS_ANIMALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_LETS_ANIMALS(v: Py) { ensure_init(); G_PLACE_LETS_ANIMALS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_LINES() -> Py { ensure_init(); G_PLACE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_LINES(v: Py) { ensure_init(); G_PLACE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_OF_ITS_OWN() -> Py { ensure_init(); G_PLACE_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_OF_ITS_OWN(v: Py) { ensure_init(); G_PLACE_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ON_A_DAY_LINES() -> Py { ensure_init(); G_PLACE_ON_A_DAY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ON_A_DAY_LINES(v: Py) { ensure_init(); G_PLACE_ON_A_DAY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_PERMISSION() -> Py { ensure_init(); G_PLACE_PERMISSION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_PERMISSION(v: Py) { ensure_init(); G_PLACE_PERMISSION.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_VENUES() -> Py { ensure_init(); G_PLACE_VENUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_VENUES(v: Py) { ensure_init(); G_PLACE_VENUES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_WITH_A_VENUE() -> Py { ensure_init(); G_PLACE_WITH_A_VENUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_WITH_A_VENUE(v: Py) { ensure_init(); G_PLACE_WITH_A_VENUE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A fact that belongs to one particular place, said honestly.\n\nWhether a pub has a car park, whether a hotel takes dogs, whether a leisure centre has a pool: the\nbot cannot know any of it. The line it used to give was the generic search hedge (\"A quick search\nonline will get you a solid answer\"), which is the worst reply for this shape, because the answer\nexists and is one tap away on the place's own page. The bot already answers opening hours that way\n(\"I can't check today's opening hours, sorry. The venue's own page or its map listing shows them\"),\nand a place's own facilities are the same kind of fact: it belongs to that place, it changes, and\nnobody but them can confirm it.\n\n`bot/compose/fallback.py` asks this first thing in `_live`, so it only ever replaces a hedge or a\nshrug. The ask has to be put to a place, and it has to name a thing that belongs to one.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_ASKS"); }
    s_PLACE_ASKS(stdlib_call("re", "compile", &[py_str("(?:\\b(?:do|does|did)\\s+(?:they|it)\\s+(?:have|got|do|offer|allow|take|sell|serve|provide|accept|permit|charge)\\b|\\b(?:do|does|did)\\s+(?:the|this|that|these|those|a|an|my|our|your)\\s+(?:[a-z'-]+\\s+){0,2}(?:have|got|do|offer|allow|take|sell|serve|provide|accept|permit|charge)\\b|\\b(?:is|are)\\s+there\\s+(?:a\\s+|any\\s+)?|\\bcan\\s+i\\s+(?:bring|take|park|book|pay|use|get)\\b|\\b(?:are|is|were|was)\\s+(?:the\\s+|our\\s+|your\\s+)?(?:dogs?|kids?|children|child|pets?)\\s+(?:allowed|welcome|permitted)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_ASKS_OF_ITS_OWN"); }
    s_PLACE_ASKS_OF_ITS_OWN(stdlib_call("re", "compile", &[py_str("(?:^(?:oh |and |so |are |is |were |was |do |does |did )?(?:the\\s+|our\\s+|your\\s+|his\\s+|her\\s+)?(?:dogs?|kids?|children|child)\\s+(?:allowed|welcome|ok|okay)\\b|\\b(?:dog|kid|child|family)[\\s-]?friendly\\b|\\bdog[\\s-]?friendly\\b|\\b(?:is|are)\\s+(?:it|they)\\s+(?:good|ok|okay|suitable|fine)\\s+for\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("car park"), py_str("car parking"), py_str("parking"), py_str("toilets"), py_str("baby change"), py_str("takeaway"), py_str("take away"), py_str("beer garden"), py_str("play area"), py_str("soft play"), py_str("high chairs"), py_str("highchair"), py_str("dress code"), py_str("step free"), py_str("wheelchair access"), py_str("wheelchair"), py_str("accessible entrance"), py_str("table service"), py_str("cash only"), py_str("card only"), py_str("booking fee"), py_str("cover charge")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_OF_ITS_OWN"); }
    s_PLACE_OF_ITS_OWN(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("pool"), py_str("swimming pool"), py_str("dogs"), py_str("dog"), py_str("kids"), py_str("children"), py_str("child"), py_str("wifi"), py_str("wi-fi"), py_str("vegan"), py_str("gluten free"), py_str("vegetarian"), py_str("bookings"), py_str("booking"), py_str("tables"), py_str("music"), py_str("quiz"), py_str("screens"), py_str("garden"), py_str("food"), py_str("menu"), py_str("delivery"), py_str("parking"), py_str("rooms"), py_str("cafe"), py_str("café"), py_str("coffee"), py_str("toilets"), py_str("toilet"), py_str("loo"), py_str("car park"), py_str("gift shop"), py_str("shop"), py_str("restaurant"), py_str("bar"), py_str("cloakroom"), py_str("lockers"), py_str("changing rooms"), py_str("baby change"), py_str("lifts"), py_str("step free access"), py_str("hearing loop"), py_str("picnic"), py_str("seating"), py_str("water")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_WITH_A_VENUE"); }
    s_PLACE_WITH_A_VENUE(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("pool"), py_str("swimming pool"), py_str("wifi"), py_str("wi-fi"), py_str("vegan"), py_str("gluten free"), py_str("vegetarian"), py_str("bookings"), py_str("tables"), py_str("menu"), py_str("food"), py_str("delivery"), py_str("parking"), py_str("rooms")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_ASKS_WITH_NO_SUBJECT"); }
    s_PLACE_ASKS_WITH_NO_SUBJECT(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_ANIMALS"); }
    s_PLACE_ANIMALS(py_tuple(vec![py_str("dogs"), py_str("dog")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_PERMISSION"); }
    s_PLACE_PERMISSION(stdlib_call("re", "compile", &[py_str("\\b(?:allow|allows|allowed|take|takes|taking|taken|permit|permits|permitted|welcome|welcomes|welcomed|accept|accepts|accepted|let|lets)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_LETS_ANIMALS"); }
    s_PLACE_LETS_ANIMALS(stdlib_call("re", "compile", &[py_str("\\b(?:do|does|did)\\s+(?:they|it)\\s+(?:let|lets|welcome|welcomes)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.BARE_PLACE_SUBJECT"); }
    s_BARE_PLACE_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:do|does|did)\\s+(?:they|it)\\b|\\b(?:is|are)\\s+(?:it|they)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_BRING_ASK"); }
    s_A_BRING_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|may|am\\s+i\\s+allowed\\s+to)\\s+(?:i\\s+|we\\s+)?(?:bring|take)\\b[^.!?]{0,30}\\b(?:dogs?|pets?|kids?|children|child|pram|buggy)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_DESTINATION"); }
    s_A_DESTINATION(stdlib_call("re", "compile", &[py_str("\\bto\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_HOME_OF_THEIR_OWN"); }
    s_A_HOME_OF_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|your)\\s+(?:(?:new|own|old|little|small|big|rented)\\s+)?(?:flat|house|home|place|garden|car|van|caravan|room|office|desk)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_IS_THERE"); }
    s_PLACE_IS_THERE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are)\\s+there\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("pub"), py_str("restaurant"), py_str("cafe"), py_str("café"), py_str("coffee shop"), py_str("bar"), py_str("hotel"), py_str("b&b"), py_str("guest house"), py_str("campsite"), py_str("caravan site"), py_str("holiday park"), py_str("gym"), py_str("leisure centre"), py_str("swimming pool"), py_str("museum"), py_str("gallery"), py_str("cinema"), py_str("theatre"), py_str("venue"), py_str("shop"), py_str("store"), py_str("supermarket"), py_str("garden centre"), py_str("station"), py_str("salon"), py_str("barber"), py_str("hairdresser"), py_str("garage"), py_str("surgery"), py_str("dentist"), py_str("pharmacy"), py_str("church"), py_str("village hall"), py_str("zoo"), py_str("farm park"), py_str("library"), py_str("club"), py_str("spa"), py_str("soft play"), py_str("bowling"), py_str("snooker hall"), py_str("arcade")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_VENUES"); }
    s_PLACE_VENUES(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_LINES"); }
    s_PLACE_LINES(py_tuple(vec![py_str("That is the place's own call, and it is usually written down: their own page lists it under visiting or facilities, and somewhere you are staying puts it in the booking terms. One phone call settles it either way."), py_str("Whoever runs the place decides that, and it is one of the few things they all publish: their own page lists it under visiting or facilities, a phone call in opening hours gets a straight answer, and the booking terms cover a room."), py_str("I can't see inside one particular place, so the answer is theirs: it will be on their page under visiting, or in the booking terms, and the phone settles anything the page leaves out.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_DAY"); }
    s_A_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|tomorrow|this\\s+(?:week|weekend|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|next\\s+week|the\\s+weekend|weekends?|bank\\s+holiday(?:\\s+monday)?|(?:mon|tues|wednes|thurs|fri|satur|sun)days?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.ON_A_DAY"); }
    s_ON_A_DAY(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|so|but|also|oh|hi|hello|quick one)?\\s*,?\\s*(?:(?:is|are|will|was|were)\\s+[^.!?]{1,40}?|(?:does|do)\\s+(?:the|that|this|our|your|their|his|her)\\b[^.!?]{0,34}?)\\s+\\b(?:on|running|happening|taking\\s+place|going\\s+ahead)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_CLOCK_ASK"); }
    s_A_CLOCK_ASK(py_tuple(vec![py_str("open"), py_str("opens"), py_str("opening"), py_str("opened"), py_str("close"), py_str("closes"), py_str("closing"), py_str("closed"), py_str("shut"), py_str("shuts"), py_str("shutting")]));
    fn __data5() -> Py { py_tuple(vec![py_str("busy"), py_str("busier"), py_str("busiest"), py_str("crowded"), py_str("packed"), py_str("rammed"), py_str("heaving"), py_str("quiet"), py_str("quieter"), py_str("quietest"), py_str("dead"), py_str("queues"), py_str("queue"), py_str("waiting"), py_str("how many people")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_CROWD_ASK"); }
    s_A_CROWD_ASK(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("heating"), py_str("boiler"), py_str("oven"), py_str("kettle"), py_str("telly"), py_str("tv"), py_str("television"), py_str("radio"), py_str("lights"), py_str("light"), py_str("lamp"), py_str("fan"), py_str("alarm"), py_str("immersion"), py_str("water"), py_str("hot water"), py_str("washing machine"), py_str("dishwasher"), py_str("tumble dryer"), py_str("dryer"), py_str("fridge"), py_str("freezer"), py_str("phone"), py_str("laptop"), py_str("fire"), py_str("shower"), py_str("radiator"), py_str("hob"), py_str("grill"), py_str("microwave"), py_str("sprinkler"), py_str("washer"), py_str("damp")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_SWITCHED_ON_THING"); }
    s_A_SWITCHED_ON_THING(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("market"), py_str("boot sale"), py_str("car boot"), py_str("match"), py_str("game"), py_str("fixture"), py_str("class"), py_str("classes"), py_str("club night"), py_str("5 a side"), py_str("five a side"), py_str("concert"), py_str("gig"), py_str("show"), py_str("fair"), py_str("fete"), py_str("quiz"), py_str("sale"), py_str("service"), py_str("session"), py_str("sessions"), py_str("practice"), py_str("training"), py_str("meeting"), py_str("event"), py_str("festival"), py_str("parade"), py_str("funfair"), py_str("playgroup"), py_str("toddler group"), py_str("story time"), py_str("swim"), py_str("swimming"), py_str("lesson"), py_str("lessons"), py_str("course"), py_str("workshop"), py_str("jumble"), py_str("table top")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_EVENTS"); }
    s_PLACE_EVENTS(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_ON_A_DAY_LINES"); }
    s_PLACE_ON_A_DAY_LINES(py_tuple(vec![py_str("Whether it is on that day is the organiser's own call, and it is the last thing they change: a wet pitch, a stall that cannot come, a hall that falls through - and a bank holiday moves the day a market or a class runs. Their own page or a phone call on the morning gives the yes or no."), py_str("That is settled on the day itself, not from here: a market, a match or a class is called off late when the weather turns, and a bank holiday can move it to another day. Their own page or listing is where the change goes up, and the phone is answered on the morning."), py_str("Nobody could tell you that in advance from this end - whoever runs it decides late, for weather, a clash or a hall, and a holiday shifts the day. Their own page or a call on the morning is where the yes or no is.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_FREE_ASK"); }
    s_A_FREE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\s+(?:it|that|this|the|a|an|my|our|your)?[\\s\\w'-]{0,24}?\\bfree\\b|\\bfree\\s+(?:entry|admission|to\\s+get\\s+in|on\\s+(?:mon|tues|wednes|thurs|fri|satur|sun))\\b|\\b(?:do|does)\\s+(?:you|i|they|we)\\s+(?:have\\s+to\\s+|need\\s+to\\s+)?pay\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_FREE_LINES"); }
    s_PLACE_FREE_LINES(py_tuple(vec![py_str("Whether it is free is the place's own rule and it moves with the season: some of what is there is charged even when the rest is not, and the free days change through the year. Their own site puts the price and any free day under visiting or opening times."), py_str("That varies with what is on and with who is going: an event day, a member's rate and a child's rate each carry their own price, and the place sets them. Their own page lists the price for the day you are going."), py_str("I can't see a place's prices from here, and they are not fixed - what is on and the time of year both change them. Their own page lists the ticket price, the free days and any concession, and the desk confirms it on the phone.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.PLACE_FACILITIES"); }
    s_PLACE_FACILITIES(call_builtin("frozenset", &[mcall(&py_str("\n    wifi wi-fi toilets toilet loo loos baby changing lockers locker cafe caf restaurant\n    shop food drink cash machine atm cloakroom\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_FIXTURE"); }
    s_A_FIXTURE(stdlib_call("re", "compile", &[py_str("\\bmatch(?:es)?\\b|\\bfixtures?\\b|\\bkick[\\s-]?offs?\\b|\\bthe\\s+game\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_RUNS_TO_A_CALENDAR"); }
    s_A_RUNS_TO_A_CALENDAR(stdlib_call("re", "compile", &[py_str("\\b(?:market|boot sale|car boot|stall|fete|fair|jumble|table top|quiz|class|classes|session|sessions|group|club)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.place_facts.A_STANDING_DAY"); }
    s_A_STANDING_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:(?:mon|tues|wednes|thurs|fri|satur|sun)days|weekends|every\\s+[a-z]+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _bounded(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    return stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])"))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _has(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let _ = py_str("True when one of these phrases is in the line as whole words.");
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&mcall(&_bounded(&[phrase.clone()]), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_place_they_are_bringing_somebody_to(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks to bring an animal or a child to a place it names.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_BRING_ASK(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_DESTINATION(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_has(&[clean.clone(), g_PLACE_VENUES()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn the_line(args: &[Py]) -> Py {
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
    let mut bringing: Py = Py::None;
    let mut lets_animals: Py = Py::None;
    let mut of_its_own: Py = Py::None;
    let _ = py_str("The honest line for a fact that belongs to one place, or None.");
    if py_truthy(&{ let __b = mcall(&g_A_HOME_OF_THEIR_OWN(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_has(&[clean.clone(), g_PLACE_VENUES()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_PLACE_IS_THERE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { _has(&[clean.clone(), g_PLACE_FACILITIES()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_FIXTURE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_RUNS_TO_A_CALENDAR(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    of_its_own = call_builtin("bool", &[mcall(&g_PLACE_ASKS_OF_ITS_OWN(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    bringing = _a_place_they_are_bringing_somebody_to(&[clean.clone()]);
    lets_animals = call_builtin("bool", &[{ let __b = mcall(&g_PLACE_LETS_ANIMALS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { _has(&[clean.clone(), g_PLACE_ANIMALS()]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = of_its_own.clone(); if py_truthy(&__b) { __b } else { { let __b = bringing.clone(); if py_truthy(&__b) { __b } else { { let __b = lets_animals.clone(); if py_truthy(&__b) { __b } else { mcall(&g_PLACE_ASKS(), "search", &[clean.clone()]) } } } } } }))) {
    return _a_day_ask(&[clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = of_its_own.clone(); if py_truthy(&__b) { __b } else { { let __b = bringing.clone(); if py_truthy(&__b) { __b } else { { let __b = lets_animals.clone(); if py_truthy(&__b) { __b } else { { let __b = _has(&[clean.clone(), g_PLACE_OF_ITS_OWN()]); if py_truthy(&__b) { __b } else { { let __b = { let __b = _has(&[clean.clone(), g_PLACE_VENUES()]); if !py_truthy(&__b) { __b } else { _has(&[clean.clone(), g_PLACE_WITH_A_VENUE()]) } }; if py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_BARE_PLACE_SUBJECT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { _has(&[clean.clone(), g_PLACE_ASKS_WITH_NO_SUBJECT()]) } }; if py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_PLACE_IS_THERE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _has(&[clean.clone(), g_PLACE_ASKS_WITH_NO_SUBJECT()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_HOME_OF_THEIR_OWN(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_has(&[clean.clone(), g_PLACE_VENUES()]))) } })) } } } }; if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BARE_PLACE_SUBJECT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _has(&[clean.clone(), g_PLACE_ANIMALS()]); if !py_truthy(&__b) { __b } else { mcall(&g_PLACE_PERMISSION(), "search", &[clean.clone()]) } } } } } } } } } } } } } } } } } }))) {
    return _a_day_ask(&[clean.clone()]);
    }
    return g_PLACE_LINES();
    Py::None
}
pub fn _a_day_ask(args: &[Py]) -> Py {
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
    let _ = py_str("The lines for a place or an event asked about on a named day, or None.\n\n    Read after the asks above, because \"is the pub free on sunday\" is both a price and a place's\n    fact and the price is the part the person is asking.\n    ");
    if py_truthy(&_has(&[clean.clone(), g_A_CLOCK_ASK()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_FREE_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _has(&[clean.clone(), g_PLACE_VENUES()]); if py_truthy(&__b) { __b } else { { let __b = _has(&[clean.clone(), g_PLACE_EVENTS()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DAY(), "search", &[clean.clone()]) } } } } } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_has(&[clean.clone(), g_A_SWITCHED_ON_THING()])))) {
    return g_PLACE_FREE_LINES();
    }
    }
    if py_truthy(&_has(&[clean.clone(), g_A_CROWD_ASK()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_STANDING_DAY(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_ON_A_DAY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_DAY(), "search", &[clean.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&_has(&[clean.clone(), g_A_SWITCHED_ON_THING()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _has(&[clean.clone(), g_PLACE_VENUES()]); if py_truthy(&__b) { __b } else { _has(&[clean.clone(), g_PLACE_EVENTS()]) } }))) {
    return Py::None;
    }
    return g_PLACE_ON_A_DAY_LINES();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.place_facts".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BRING_ASK" => Some(g_A_BRING_ASK()),
            "A_CLOCK_ASK" => Some(g_A_CLOCK_ASK()),
            "A_CROWD_ASK" => Some(g_A_CROWD_ASK()),
            "A_DAY" => Some(g_A_DAY()),
            "A_DESTINATION" => Some(g_A_DESTINATION()),
            "A_FIXTURE" => Some(g_A_FIXTURE()),
            "A_FREE_ASK" => Some(g_A_FREE_ASK()),
            "A_HOME_OF_THEIR_OWN" => Some(g_A_HOME_OF_THEIR_OWN()),
            "A_RUNS_TO_A_CALENDAR" => Some(g_A_RUNS_TO_A_CALENDAR()),
            "A_STANDING_DAY" => Some(g_A_STANDING_DAY()),
            "A_SWITCHED_ON_THING" => Some(g_A_SWITCHED_ON_THING()),
            "BARE_PLACE_SUBJECT" => Some(g_BARE_PLACE_SUBJECT()),
            "ON_A_DAY" => Some(g_ON_A_DAY()),
            "PLACE_ANIMALS" => Some(g_PLACE_ANIMALS()),
            "PLACE_ASKS" => Some(g_PLACE_ASKS()),
            "PLACE_ASKS_OF_ITS_OWN" => Some(g_PLACE_ASKS_OF_ITS_OWN()),
            "PLACE_ASKS_WITH_NO_SUBJECT" => Some(g_PLACE_ASKS_WITH_NO_SUBJECT()),
            "PLACE_EVENTS" => Some(g_PLACE_EVENTS()),
            "PLACE_FACILITIES" => Some(g_PLACE_FACILITIES()),
            "PLACE_FREE_LINES" => Some(g_PLACE_FREE_LINES()),
            "PLACE_IS_THERE" => Some(g_PLACE_IS_THERE()),
            "PLACE_LETS_ANIMALS" => Some(g_PLACE_LETS_ANIMALS()),
            "PLACE_LINES" => Some(g_PLACE_LINES()),
            "PLACE_OF_ITS_OWN" => Some(g_PLACE_OF_ITS_OWN()),
            "PLACE_ON_A_DAY_LINES" => Some(g_PLACE_ON_A_DAY_LINES()),
            "PLACE_PERMISSION" => Some(g_PLACE_PERMISSION()),
            "PLACE_VENUES" => Some(g_PLACE_VENUES()),
            "PLACE_WITH_A_VENUE" => Some(g_PLACE_WITH_A_VENUE()),
            "_a_day_ask" => Some(py_func("_a_day_ask", |args: &[Py]| _a_day_ask(args))),
            "_a_place_they_are_bringing_somebody_to" => Some(py_func("_a_place_they_are_bringing_somebody_to", |args: &[Py]| _a_place_they_are_bringing_somebody_to(args))),
            "_bounded" => Some(py_func("_bounded", |args: &[Py]| _bounded(args))),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "the_line" => Some(py_func("the_line", |args: &[Py]| the_line(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BRING_ASK" => s_A_BRING_ASK(v),
            "A_CLOCK_ASK" => s_A_CLOCK_ASK(v),
            "A_CROWD_ASK" => s_A_CROWD_ASK(v),
            "A_DAY" => s_A_DAY(v),
            "A_DESTINATION" => s_A_DESTINATION(v),
            "A_FIXTURE" => s_A_FIXTURE(v),
            "A_FREE_ASK" => s_A_FREE_ASK(v),
            "A_HOME_OF_THEIR_OWN" => s_A_HOME_OF_THEIR_OWN(v),
            "A_RUNS_TO_A_CALENDAR" => s_A_RUNS_TO_A_CALENDAR(v),
            "A_STANDING_DAY" => s_A_STANDING_DAY(v),
            "A_SWITCHED_ON_THING" => s_A_SWITCHED_ON_THING(v),
            "BARE_PLACE_SUBJECT" => s_BARE_PLACE_SUBJECT(v),
            "ON_A_DAY" => s_ON_A_DAY(v),
            "PLACE_ANIMALS" => s_PLACE_ANIMALS(v),
            "PLACE_ASKS" => s_PLACE_ASKS(v),
            "PLACE_ASKS_OF_ITS_OWN" => s_PLACE_ASKS_OF_ITS_OWN(v),
            "PLACE_ASKS_WITH_NO_SUBJECT" => s_PLACE_ASKS_WITH_NO_SUBJECT(v),
            "PLACE_EVENTS" => s_PLACE_EVENTS(v),
            "PLACE_FACILITIES" => s_PLACE_FACILITIES(v),
            "PLACE_FREE_LINES" => s_PLACE_FREE_LINES(v),
            "PLACE_IS_THERE" => s_PLACE_IS_THERE(v),
            "PLACE_LETS_ANIMALS" => s_PLACE_LETS_ANIMALS(v),
            "PLACE_LINES" => s_PLACE_LINES(v),
            "PLACE_OF_ITS_OWN" => s_PLACE_OF_ITS_OWN(v),
            "PLACE_ON_A_DAY_LINES" => s_PLACE_ON_A_DAY_LINES(v),
            "PLACE_PERMISSION" => s_PLACE_PERMISSION(v),
            "PLACE_VENUES" => s_PLACE_VENUES(v),
            "PLACE_WITH_A_VENUE" => s_PLACE_WITH_A_VENUE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.place_facts", module_obj);
    pyrt::xrefs::reg_fn("compose.place_facts", "_bounded", py_func("_bounded", |args: &[Py]| _bounded(args)));
    pyrt::xrefs::reg_fn("compose.place_facts", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("compose.place_facts", "_a_place_they_are_bringing_somebody_to", py_func("_a_place_they_are_bringing_somebody_to", |args: &[Py]| _a_place_they_are_bringing_somebody_to(args)));
    pyrt::xrefs::reg_fn("compose.place_facts", "the_line", py_func("the_line", |args: &[Py]| the_line(args)));
    pyrt::xrefs::reg_fn("compose.place_facts", "_a_day_ask", py_func("_a_day_ask", |args: &[Py]| _a_day_ask(args)));
    pyrt::xrefs::reg_access("compose.place_facts", "A_BRING_ASK", g_A_BRING_ASK, s_A_BRING_ASK);
    pyrt::xrefs::reg_access("compose.place_facts", "A_CLOCK_ASK", g_A_CLOCK_ASK, s_A_CLOCK_ASK);
    pyrt::xrefs::reg_access("compose.place_facts", "A_CROWD_ASK", g_A_CROWD_ASK, s_A_CROWD_ASK);
    pyrt::xrefs::reg_access("compose.place_facts", "A_DAY", g_A_DAY, s_A_DAY);
    pyrt::xrefs::reg_access("compose.place_facts", "A_DESTINATION", g_A_DESTINATION, s_A_DESTINATION);
    pyrt::xrefs::reg_access("compose.place_facts", "A_FIXTURE", g_A_FIXTURE, s_A_FIXTURE);
    pyrt::xrefs::reg_access("compose.place_facts", "A_FREE_ASK", g_A_FREE_ASK, s_A_FREE_ASK);
    pyrt::xrefs::reg_access("compose.place_facts", "A_HOME_OF_THEIR_OWN", g_A_HOME_OF_THEIR_OWN, s_A_HOME_OF_THEIR_OWN);
    pyrt::xrefs::reg_access("compose.place_facts", "A_RUNS_TO_A_CALENDAR", g_A_RUNS_TO_A_CALENDAR, s_A_RUNS_TO_A_CALENDAR);
    pyrt::xrefs::reg_access("compose.place_facts", "A_STANDING_DAY", g_A_STANDING_DAY, s_A_STANDING_DAY);
    pyrt::xrefs::reg_access("compose.place_facts", "A_SWITCHED_ON_THING", g_A_SWITCHED_ON_THING, s_A_SWITCHED_ON_THING);
    pyrt::xrefs::reg_access("compose.place_facts", "BARE_PLACE_SUBJECT", g_BARE_PLACE_SUBJECT, s_BARE_PLACE_SUBJECT);
    pyrt::xrefs::reg_access("compose.place_facts", "ON_A_DAY", g_ON_A_DAY, s_ON_A_DAY);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_ANIMALS", g_PLACE_ANIMALS, s_PLACE_ANIMALS);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_ASKS", g_PLACE_ASKS, s_PLACE_ASKS);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_ASKS_OF_ITS_OWN", g_PLACE_ASKS_OF_ITS_OWN, s_PLACE_ASKS_OF_ITS_OWN);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_ASKS_WITH_NO_SUBJECT", g_PLACE_ASKS_WITH_NO_SUBJECT, s_PLACE_ASKS_WITH_NO_SUBJECT);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_EVENTS", g_PLACE_EVENTS, s_PLACE_EVENTS);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_FACILITIES", g_PLACE_FACILITIES, s_PLACE_FACILITIES);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_FREE_LINES", g_PLACE_FREE_LINES, s_PLACE_FREE_LINES);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_IS_THERE", g_PLACE_IS_THERE, s_PLACE_IS_THERE);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_LETS_ANIMALS", g_PLACE_LETS_ANIMALS, s_PLACE_LETS_ANIMALS);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_LINES", g_PLACE_LINES, s_PLACE_LINES);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_OF_ITS_OWN", g_PLACE_OF_ITS_OWN, s_PLACE_OF_ITS_OWN);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_ON_A_DAY_LINES", g_PLACE_ON_A_DAY_LINES, s_PLACE_ON_A_DAY_LINES);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_PERMISSION", g_PLACE_PERMISSION, s_PLACE_PERMISSION);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_VENUES", g_PLACE_VENUES, s_PLACE_VENUES);
    pyrt::xrefs::reg_access("compose.place_facts", "PLACE_WITH_A_VENUE", g_PLACE_WITH_A_VENUE, s_PLACE_WITH_A_VENUE);
}
