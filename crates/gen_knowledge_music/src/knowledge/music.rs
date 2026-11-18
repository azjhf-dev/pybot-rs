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
thread_local! { static G_BROKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLEANING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINGERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINE_HEARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEW_STRINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVERHEARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSIC_AROUND_STAFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSIC_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STAFF_MEANS_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STAFF_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.music: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_BROKEN() -> Py { ensure_init(); G_BROKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROKEN(v: Py) { ensure_init(); G_BROKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CLEANING() -> Py { ensure_init(); G_CLEANING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLEANING(v: Py) { ensure_init(); G_CLEANING.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FINGERS() -> Py { ensure_init(); G_FINGERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINGERS(v: Py) { ensure_init(); G_FINGERS.with(|c| *c.borrow_mut() = v); }
pub fn g_KIT() -> Py { ensure_init(); G_KIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIT(v: Py) { ensure_init(); G_KIT.with(|c| *c.borrow_mut() = v); }
pub fn g_MINE_HEARD() -> Py { ensure_init(); G_MINE_HEARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINE_HEARD(v: Py) { ensure_init(); G_MINE_HEARD.with(|c| *c.borrow_mut() = v); }
pub fn g_NEW_STRINGS() -> Py { ensure_init(); G_NEW_STRINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEW_STRINGS(v: Py) { ensure_init(); G_NEW_STRINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_OVERHEARD() -> Py { ensure_init(); G_OVERHEARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVERHEARD(v: Py) { ensure_init(); G_OVERHEARD.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAYING() -> Py { ensure_init(); G_PLAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAYING(v: Py) { ensure_init(); G_PLAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIET() -> Py { ensure_init(); G_QUIET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIET(v: Py) { ensure_init(); G_QUIET.with(|c| *c.borrow_mut() = v); }
pub fn g_STRINGS() -> Py { ensure_init(); G_STRINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRINGS(v: Py) { ensure_init(); G_STRINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSIC_AROUND_STAFF() -> Py { ensure_init(); G__MUSIC_AROUND_STAFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSIC_AROUND_STAFF(v: Py) { ensure_init(); G__MUSIC_AROUND_STAFF.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSIC_PHRASES() -> Py { ensure_init(); G__MUSIC_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSIC_PHRASES(v: Py) { ensure_init(); G__MUSIC_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g__STAFF_MEANS_PEOPLE() -> Py { ensure_init(); G__STAFF_MEANS_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STAFF_MEANS_PEOPLE(v: Py) { ensure_init(); G__STAFF_MEANS_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__STAFF_TAILS() -> Py { ensure_init(); G__STAFF_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STAFF_TAILS(v: Py) { ensure_init(); G__STAFF_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The instrument in the corner: its strings, its upkeep and the practice on it.\n\nThe picks (what to listen to, what to play next) are `entertainment.py`, and the first\nsteps into a new hobby are its activity table. What was missing was the instrument itself\nand the work of keeping it playable: a string that broke, a guitar that needs cleaning, the\nneighbours hearing you, sore fingertips, a barre that will not ring, reading the dots. Those\nlines were a shrug or, worse, a neighbour's card - \"my guitar string broke\" was told \"Ugh,\nthat's a lot to carry\" and \"how do i restring a guitar\" was sold a beginner's acoustic.\n\nEach row keys on the instrument and the thing being asked about it, so one answer covers\nevery guitar, ukulele and violin people type, and a line that only names an instrument\n(\"i want to learn guitar\") stays with the learning rows it already had.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("entertainment"), py_str("creative")]));
    fn __data1() -> Py { py_tuple(vec![py_str("guitar"), py_str("guitars"), py_str("ukulele"), py_str("uke"), py_str("violin"), py_str("violins"), py_str("fiddle"), py_str("cello"), py_str("double bass"), py_str("bass guitar"), py_str("banjo"), py_str("mandolin"), py_str("harp"), py_str("lute"), py_str("sitar"), py_str("bass")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.STRINGS"); }
    s_STRINGS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("piano"), py_str("pianos"), py_str("digital piano"), py_str("drum kit"), py_str("drum kits"), py_str("flute"), py_str("saxophone"), py_str("sax"), py_str("trumpet"), py_str("clarinet"), py_str("accordion"), py_str("harmonica"), py_str("recorder"), py_str("trombone"), py_str("tuba")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.KIT"); }
    s_KIT(op_add(&g_STRINGS(), &__data2()));
    fn __data3() -> Py { py_tuple(vec![py_str("playing"), py_str("practising"), py_str("practicing"), py_str("practice"), py_str("rehearsing"), py_str("guitar"), py_str("guitars"), py_str("piano"), py_str("violin"), py_str("ukulele"), py_str("cello"), py_str("bass"), py_str("mandolin"), py_str("banjo"), py_str("chords"), py_str("strings"), py_str("fretboard"), py_str("fretting"), py_str("drum kit"), py_str("singing")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.PLAYING"); }
    s_PLAYING(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("neighbours"), py_str("neighbors"), py_str("neighbour"), py_str("neighbor"), py_str("next door"), py_str("flat above"), py_str("people above"), py_str("landlord"), py_str("my mum"), py_str("my dad"), py_str("my family"), py_str("my partner"), py_str("everyone in the house")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.OVERHEARD"); }
    s_OVERHEARD(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("hear me"), py_str("hears me"), py_str("can hear me"), py_str("hear us"), py_str("hears us"), py_str("can hear us"), py_str("hear my"), py_str("hears my"), py_str("hear it when i"), py_str("hear me when i"), py_str("while i practise"), py_str("while i practice"), py_str("when i practise"), py_str("when i practice"), py_str("when i play"), py_str("about my playing"), py_str("about my practising"), py_str("about my practice"), py_str("about my music"), py_str("complain about me"), py_str("complaining about me"), py_str("complains about me"), py_str("moan about me"), py_str("moaning about me"), py_str("moans about me")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.MINE_HEARD"); }
    s_MINE_HEARD(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("broke"), py_str("broken"), py_str("snapped"), py_str("snap"), py_str("snaps"), py_str("bust"), py_str("busted"), py_str("popped"), py_str("gone"), py_str("pinged"), py_str("off"), py_str("came off"), py_str("keeps breaking"), py_str("keeps snapping"), py_str("one string missing"), py_str("missing string"), py_str("missing a string"), py_str("a string short"), py_str("without a string")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.BROKEN"); }
    s_BROKEN(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("change"), py_str("changing"), py_str("changed"), py_str("replace"), py_str("replacing"), py_str("new"), py_str("put on"), py_str("putting on"), py_str("how often"), py_str("when should i"), py_str("when do i"), py_str("every"), py_str("restring"), py_str("resting"), py_str("fit"), py_str("fitting"), py_str("sounds dull"), py_str("sounds dead"), py_str("sounds flat"), py_str("sounds rubbish"), py_str("no sustain"), py_str("lost its tone"), py_str("lost tone"), py_str("dull sound")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.NEW_STRINGS"); }
    s_NEW_STRINGS(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("polish"), py_str("polishing"), py_str("dust"), py_str("dusty"), py_str("grime"), py_str("grimy"), py_str("sticky"), py_str("smelly"), py_str("smells"), py_str("sweat"), py_str("marks"), py_str("hygiene"), py_str("wipe"), py_str("disinfect")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.CLEANING"); }
    s_CLEANING(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("quieter"), py_str("quiet"), py_str("quietly"), py_str("loud"), py_str("headphones"), py_str("silent"), py_str("silence"), py_str("soundproof"), py_str("soundproofing"), py_str("hear me"), py_str("hear it"), py_str("hearing"), py_str("anyone hearing"), py_str("nobody hears"), py_str("no one hears"), py_str("complaining"), py_str("mute"), py_str("muted"), py_str("wake the house"), py_str("wake my family")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.QUIET"); }
    s_QUIET(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("fingers"), py_str("fingertips"), py_str("finger tips"), py_str("fingertip"), py_str("sore"), py_str("hurts"), py_str("hurt"), py_str("painful"), py_str("blisters"), py_str("calluses"), py_str("callouses"), py_str("aching"), py_str("ache")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.FINGERS"); }
    s_FINGERS(__data10());
    fn __data11() -> Py { py_tuple(vec![py_str("music"), py_str("musical"), py_str("stave"), py_str("clef"), py_str("clefs"), py_str("treble"), py_str("bass"), py_str("note"), py_str("notes"), py_str("dot"), py_str("dots"), py_str("line"), py_str("lines"), py_str("space"), py_str("spaces"), py_str("sheet"), py_str("score"), py_str("theory"), py_str("sight"), py_str("octave"), py_str("piano"), py_str("violin"), py_str("instrument"), py_str("sing"), py_str("song"), py_str("chord"), py_str("chords"), py_str("bar"), py_str("bars"), py_str("read"), py_str("reading"), py_str("learn"), py_str("learning")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music._MUSIC_AROUND_STAFF"); }
    s__MUSIC_AROUND_STAFF(call_builtin("frozenset", &[__data11()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music._MUSIC_PHRASES"); }
    s__MUSIC_PHRASES(py_tuple(vec![py_str("middle c"), py_str("note lengths"), py_str("sheet music"), py_str("sight read"), py_str("treble clef"), py_str("bass clef")]));
    fn __data12() -> Py { py_tuple(vec![py_str("my staff"), py_str("our staff"), py_str("two staff"), py_str("three staff"), py_str("four staff"), py_str("staff who"), py_str("staff member"), py_str("staff members"), py_str("members of staff"), py_str("staff are"), py_str("staff is"), py_str("staff not"), py_str("staff dont"), py_str("staff won't"), py_str("staff wont"), py_str("staff keep"), py_str("staff always"), py_str("staff never"), py_str("staff argue"), py_str("staff meeting"), py_str("staff rota"), py_str("staff turnover"), py_str("short staffed"), py_str("understaffed"), py_str("of staff")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music._STAFF_MEANS_PEOPLE"); }
    s__STAFF_MEANS_PEOPLE(__data12());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music._STAFF_TAILS"); }
    s__STAFF_TAILS(call_builtin("frozenset", &[py_tuple(vec![py_str(""), py_str("ed"), py_str("ing"), py_str("s"), py_str("er"), py_str("ers")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data14() -> Py { py_tuple(vec![py_str("car"), py_str("drawstring"), py_str("shoe"), py_str("shoes"), py_str("shoelace"), py_str("shoelaces"), py_str("bag"), py_str("hoodie"), py_str("tracksuit"), py_str("curtain"), py_str("curtains"), py_str("neck"), py_str("headstock"), py_str("fretboard"), py_str("purse"), py_str("scissors"), py_str("theatre"), py_str("gift"), py_str("present"), py_str("fish"), py_str("fishing"), py_str("sea"), py_str("lake"), py_str("river"), py_str("pond"), py_str("loch"), py_str("speaker"), py_str("speakers"), py_str("sub"), py_str("subwoofer")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("car"), py_str("drawstring"), py_str("shoe"), py_str("shoes"), py_str("shoelace"), py_str("shoelaces"), py_str("bag"), py_str("hoodie"), py_str("curtain"), py_str("curtains"), py_str("purse"), py_str("scissors"), py_str("theatre"), py_str("gift"), py_str("present"), py_str("bank"), py_str("insurance"), py_str("energy"), py_str("supplier"), py_str("fish"), py_str("fishing"), py_str("sea"), py_str("lake"), py_str("river"), py_str("pond"), py_str("speaker"), py_str("speakers"), py_str("subwoofer"), py_str("practice"), py_str("practise"), py_str("practising"), py_str("practicing"), py_str("rehearse"), py_str("rehearsing"), py_str("practice pad")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("car"), py_str("oven"), py_str("hob"), py_str("kitchen"), py_str("bathroom"), py_str("house"), py_str("room"), py_str("window"), py_str("windows"), py_str("floor"), py_str("carpet"), py_str("fridge"), py_str("washing machine"), py_str("sofa"), py_str("gutter"), py_str("damp"), py_str("mould"), py_str("mold"), py_str("tumble"), py_str("dryer"), py_str("wall"), py_str("ceiling"), py_str("smell"), py_str("smells"), py_str("smelly")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("television"), py_str("tv"), py_str("speaker"), py_str("speakers"), py_str("hi-fi"), py_str("hifi"), py_str("air conditioner"), py_str("boiler"), py_str("fan"), py_str("extractor"), py_str("car"), py_str("engine"), py_str("motor"), py_str("kids"), py_str("children")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("barking"), py_str("tv"), py_str("television"), py_str("stereo"), py_str("hi-fi"), py_str("party"), py_str("washing machine"), py_str("builders"), py_str("building work"), py_str("renovating"), py_str("children"), py_str("toddler"), py_str("football"), py_str("car"), py_str("engine"), py_str("alarm"), py_str("council"), py_str("environmental")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("wrist"), py_str("wrist pain"), py_str("carpal"), py_str("tingling"), py_str("numb"), py_str("pins and needles"), py_str("arm"), py_str("elbow"), py_str("shoulder"), py_str("back"), py_str("neck"), py_str("typing"), py_str("keyboard at work"), py_str("mouse"), py_str("tendonitis"), py_str("rsi")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("buzz"), py_str("buzzing"), py_str("buzzes"), py_str("hum"), py_str("humming"), py_str("hums"), py_str("hiss"), py_str("hissing"), py_str("crackle"), py_str("crackling"), py_str("noise"), py_str("noisy"), py_str("rattle"), py_str("rattling")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("car"), py_str("caravan"), py_str("fuse box"), py_str("boiler"), py_str("doorbell"), py_str("intercom"), py_str("buzzer"), py_str("bee"), py_str("bees"), py_str("wasp"), py_str("fridge"), py_str("washing machine"), py_str("hearing aid"), py_str("ear"), py_str("tinnitus"), py_str("speaker"), py_str("subwoofer"), py_str("hi-fi"), py_str("tv"), py_str("television"), py_str("phone")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("hold"), py_str("holding"), py_str("grip"), py_str("how do i hold"), py_str("thick"), py_str("thin"), py_str("thickness"), py_str("size"), py_str("material"), py_str("lose"), py_str("losing"), py_str("drops"), py_str("drop"), py_str("keep dropping"), py_str("my hand"), py_str("loose"), py_str("slipping"), py_str("slip"), py_str("smoothly"), py_str("smooth"), py_str("evenly"), py_str("rhythm"), py_str("strum"), py_str("strumming"), py_str("nylon"), py_str("tortex"), py_str("dunlop")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("nose"), py_str("spot"), py_str("spots"), py_str("pimple"), py_str("scab"), py_str("lock"), py_str("door"), py_str("car"), py_str("apple"), py_str("berries"), py_str("strawberry"), py_str("blackberry"), py_str("lottery"), py_str("numbers"), py_str("football"), py_str("fantasy"), py_str("bet"), py_str("betting"), py_str("choosy"), py_str("fussy"), py_str("pick up"), py_str("picking up")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("learn"), py_str("learning"), py_str("first"), py_str("strum"), py_str("strumming"), py_str("change chords"), py_str("change between"), py_str("switch chords"), py_str("songs"), py_str("start"), py_str("beginner"), py_str("how do i"), py_str("play")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("better"), py_str("improve"), py_str("in tune"), py_str("on pitch"), py_str("technique"), py_str("breath"), py_str("breathing"), py_str("warm up"), py_str("warmup"), py_str("how do i"), py_str("tips"), py_str("practice"), py_str("practise"), py_str("range"), py_str("high notes")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("choir"), py_str("choir practice"), py_str("join a choir"), py_str("lessons"), py_str("lesson"), py_str("teacher"), py_str("shaking"), py_str("shaky"), py_str("shake"), py_str("nervous"), py_str("nerves"), py_str("presentation"), py_str("speech"), py_str("public speaking"), py_str("interview"), py_str("audience"), py_str("stage fright"), py_str("cracking"), py_str("hoarse"), py_str("husky"), py_str("raspy"), py_str("voice going"), py_str("lost my voice"), py_str("out of breath"), py_str("breathless"), py_str("wheeze"), py_str("wheezing"), py_str("asthma"), py_str("chest"), py_str("dizzy"), py_str("faint"), py_str("cough"), py_str("gig"), py_str("karaoke"), py_str("carol"), py_str("carols"), py_str("national anthem"), py_str("kids"), py_str("children"), py_str("toddler"), py_str("baby"), py_str("nursery"), py_str("school"), py_str("happy birthday"), py_str("wedding"), py_str("join"), py_str("joining"), py_str("group"), py_str("club"), py_str("class"), py_str("classes"), py_str("choir")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("essay"), py_str("exam"), py_str("revision"), py_str("revise"), py_str("study"), py_str("school"), py_str("homework"), py_str("child"), py_str("reading to my"), py_str("bedtime story"), py_str("phonics"), py_str("dyslexia"), py_str("letter"), py_str("letters"), py_str("book"), py_str("books"), py_str("novel"), py_str("read a book")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("spill"), py_str("spilt"), py_str("spilled"), py_str("drink"), py_str("coffee"), py_str("tea"), py_str("beer"), py_str("water"), py_str("sticky"), py_str("sticking"), py_str("sticks"), py_str("jammed"), py_str("broken"), py_str("laptop"), py_str("computer"), py_str("typing"), py_str("mechanical"), py_str("phone"), py_str("tablet"), py_str("wont type"), py_str("not working")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("buy"), py_str("buying"), py_str("should i get"), py_str("should i buy"), py_str("worth it"), py_str("cheap"), py_str("cheapest"), py_str("afford"), py_str("budget"), py_str("which"), py_str("recommend"), py_str("what kind"), py_str("second hand"), py_str("secondhand"), py_str("rent"), py_str("renting"), py_str("for a beginner"), py_str("for my kid"), py_str("for my child")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("tune"), py_str("tuning"), py_str("broken"), py_str("not working"), py_str("repair"), py_str("stuck key"), py_str("car"), py_str("computer"), py_str("laptop"), py_str("pc"), py_str("mac"), py_str("phone"), py_str("ipad"), py_str("typing"), py_str("wireless"), py_str("bluetooth"), py_str("mechanical"), py_str("shortcut"), py_str("shortcuts"), py_str("work"), py_str("office"), py_str("meeting"), py_str("game"), py_str("games"), py_str("controller")]) }
    fn __data13() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-string-broke")); __args.push(py_tuple(vec![g_STRINGS(), g_BROKEN()])); __args.push(py_str("That is a few pounds, not a new instrument: take the broken string off so the loose end does not scratch the top, then fit a new one - a single string or a full set is under a tenner from any music shop, and most will restring it for you for about ten pounds if you would rather not. Loosen it right off before you cut, and if the same string keeps going it is usually a sharp edge in the nut or the bridge, which a shop files smooth.")); __kws.extend(vec![("avoid".to_string(), __data14()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-string-change")); __args.push(py_tuple(vec![op_add(&g_STRINGS(), &py_tuple(vec![py_str("my strings"), py_str("the strings"), py_str("these strings")])), g_NEW_STRINGS()])); __args.push(py_str("Dead strings are most of what makes a guitar sound dull. Change them every one to three months if you play often, or the moment they feel rough, look tarnished or stop holding tune. Fit them one at a time so the neck keeps its tension: thread the string, leave two or three windings round the post, tune up to pitch and pull each string gently away from the fretboard a few times, then tune again. Nylon strings take a few days to settle, so expect to retune the first three days.")); __kws.extend(vec![("avoid".to_string(), __data15()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-clean")); __args.push(py_tuple(vec![g_KIT(), g_CLEANING()])); __args.push(py_str("Wipe the strings and the back of the neck with a dry microfibre cloth every time you put it down - the sweat and skin oil is what makes a neck sticky and kills strings. Once or twice a year, take the strings off a rosewood or ebony board, scrape the grime off with a plastic card, and put a few drops of lemon oil on it; a lacquered maple board only needs the dry cloth, and never use furniture polish on the body.")); __kws.extend(vec![("avoid".to_string(), __data16()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-play-quietly")); __args.push(py_tuple(vec![op_add(&g_KIT(), &py_tuple(vec![py_str("practice"), py_str("practise"), py_str("practising"), py_str("practicing"), py_str("playing music"), py_str("play music"), py_str("my instrument"), py_str("learning an instrument")])), g_QUIET()])); __args.push(py_str("For an electric, the whole fix is a headphone amp or an audio interface into your phone - a twenty pound plug-in amp is silent to everyone else. On an acoustic, stuff a cloth or a soundhole cover in the hole, play with a thin pick and a light touch, and put the instrument on a cushion in a small room rather than against a shared wall; a digital piano with headphones does the same for keys.")); __kws.extend(vec![("avoid".to_string(), __data17()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-overheard")); __args.push(py_tuple(vec![g_OVERHEARD(), g_MINE_HEARD()])); __args.push(py_str("That is what headphones, a mute or a practice room are for, and most of it is cheaper than a row: a headphone amp for an electric guitar, a soundhole cover or a folded cloth in an acoustic, a practice mute for a violin or a brass instrument, and headphones on a digital piano. Pick the hours you would want quiet in yourself - not before nine and not after ten - and tell them once, cheerfully, that you play and when you stop.")); __kws.extend(vec![("guard".to_string(), py_func("_a_musical_line", |args: &[Py]| _a_musical_line(args))), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data18())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-sore-fingers")); __args.push(py_tuple(vec![g_PLAYING(), g_FINGERS()])); __args.push(py_str("That is the callus building, and it passes: keep sessions to fifteen or twenty minutes and stop when they start to sting, rather than pushing through to blisters. Press just behind the fret with the lightest touch that makes a clean note, play with lighter strings (10s on an electric, extra-light on an acoustic) and have the action lowered by a shop if it is high - and take the day off if one is torn, because a broken blister sets you back a fortnight.")); __kws.extend(vec![("avoid".to_string(), __data19()), ("guard".to_string(), py_func("_not_an_infection", |args: &[Py]| _not_an_infection(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-amp-noise")); __args.push(py_tuple(vec![py_tuple(vec![py_str("amp"), py_str("amplifier"), py_str("guitar amp"), py_str("combo"), py_str("valve amp")]), __data20()])); __args.push(py_str("Hiss with nothing plugged in is normal for a valve amp, and a buzz that stops when you touch the strings or turn down the guitar's volume is the room's wiring or a single-coil pickup, not a fault: stand further from the amp, turn your body, and keep a phone and a dimmer switch away from it. If it hums with a cable in and nothing else, swap the cable; if it crackles when you move the guitar lead, the jack is loose; if it buzzes only on one socket in the house, try another room before you blame the amp.")); __kws.extend(vec![("avoid".to_string(), __data21()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-pick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pick"), py_str("plectrum"), py_str("plectrums"), py_str("picks"), py_str("strumming"), py_str("strum"), py_str("guitar pick"), py_str("picking hand")]), __data22()])); __args.push(py_str("Hold it between the thumb and the side of the curled index finger, with just a few millimetres showing, and grip it as lightly as you can without it turning - most beginners squeeze and the pick fights back. A thin pick (0.5 to 0.6mm) is easier for strumming chords, a thicker one (1mm) for single notes, and if it keeps slipping, the ones with a textured or drilled grip are worth the extra pound. Strum from the wrist with the arm still, and let the pick pass across the strings at a slight angle rather than flat, which is what stops the catching and the dropping.")); __kws.extend(vec![("avoid".to_string(), __data23()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-get-faster")); __args.push(py_tuple(vec![g_PLAYING(), py_tuple(vec![py_str("faster"), py_str("speed"), py_str("quicker"), py_str("get fast"), py_str("speed up"), py_str("play fast"), py_str("shred"), py_str("alternate picking"), py_str("technique"), py_str("accuracy"), py_str("cleanly"), py_str("sloppy")])])); __args.push(py_str("Speed is accuracy first: set a metronome or a drum loop well below the tempo you can just about manage, play the passage clean five times in a row, then add five beats a minute, and drop back the moment it gets sloppy - two weeks of that beats months of playing it fast and wrong. Keep the fretting fingers close to the strings, pick with a small movement from the wrist rather than the whole arm, and practise the hard two bars on their own instead of the whole song. Ten focused minutes a day is plenty; speed comes from the slow repetitions, not from trying harder.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("chord"), py_str("chords"), py_str("barre"), py_str("bar chord"), py_str("bar chords")])), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-barre-chords")); __args.push(py_tuple(vec![py_tuple(vec![py_str("barre"), py_str("barre chord"), py_str("barre chords"), py_str("bar chord"), py_str("bar chords"), py_str("barring")]), py_tuple(vec![py_str("chord"), py_str("chords"), py_str("play"), py_str("ring"), py_str("buzz"), py_str("clear"), py_str("how do i"), py_str("learn"), py_str("hurt")])])); __args.push(py_str("Roll the first finger slightly onto its bony outer edge rather than flat on the pad, keep the thumb low behind the neck and pull the elbow in towards your body - the pressure comes from the arm, not from squeezing. Start at the fifth fret where the strings are easier, get one clean barre there, then walk the same shape down a fret at a time; a guitar set up with high action will not let you barre cleanly, so that is the next thing to fix.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("car"), py_str("bar"), py_str("pub"), py_str("restaurant"), py_str("drink"), py_str("cocktail"), py_str("salad")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-first-chords")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chord"), py_str("chords"), py_str("chord shapes"), py_str("open chords")]), __data24()])); __args.push(py_str("Learn four shapes first - G, C, D and E minor - because those four are most of the songs you already know, and practise moving between two of them rather than holding one well. Ten minutes a day: one minute of slow changes with a metronome or a steady count, then play a song with the same two chords in it, and add a third only when the change is clean.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("barre"), py_str("bar chord"), py_str("bar chords")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sing-better")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sing"), py_str("singing"), py_str("voice"), py_str("vocals"), py_str("vocal"), py_str("pitchy")]), __data25()])); __args.push(py_str("Sing along to something you can hear clearly and quietly - headphones at half volume - because most people cannot hear themselves and push too hard. Breathe low into the belly, keep the throat relaxed, warm up with lip trills or humming for two minutes, and record yourself on your phone once a week: the recording is the lesson. For range or a song that keeps going wrong, three or four lessons with a teacher do more than a year of videos.")); __kws.extend(vec![("avoid".to_string(), __data26()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("read-sheet-music")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sheet music"), py_str("music theory"), py_str("read music"), py_str("reading music"), py_str("sight read"), py_str("sight reading"), py_str("staff"), py_str("clef"), py_str("treble clef"), py_str("bass clef"), py_str("music notes")]), py_tuple(vec![py_str("learn"), py_str("reading"), py_str("read"), py_str("understand"), py_str("how do i"), py_str("start"), py_str("beginner"), py_str("teach myself"), py_str("what are")])])); __args.push(py_str("Start with the treble clef only: the five lines are E, G, B, D, F from the bottom up and the four spaces spell FACE, and ten minutes a day of naming notes off a page is how it sticks. Then the same for the bass clef (Good Boys Do Fine Always on the lines), then note lengths and rests; a beginner tutor book and a free app like Tenuto cover it, and playing each note on the instrument as you name it is the part that makes it real.")); __kws.extend(vec![("avoid".to_string(), __data27()), ("guard".to_string(), py_func("_staff_is_the_five_lines", |args: &[Py]| _staff_is_the_five_lines(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("piano-keys-layout")); __args.push(py_tuple(vec![py_tuple(vec![py_str("piano"), py_str("piano keyboard"), py_str("keys on a piano"), py_str("keys of the piano"), py_str("keys on the piano")]), py_tuple(vec![py_str("white keys"), py_str("black keys"), py_str("notes on"), py_str("which note"), py_str("layout"), py_str("the pattern"), py_str("middle c"), py_str("where is c"), py_str("find c"), py_str("which key")])])); __args.push(py_str("The keys repeat a run of twelve, seven white and five black, and the black keys come in a group of two and then a group of three: the white key to the left of the pair of black ones is C, and the rest run D E F G A B up to the next C. So find any pair of black keys and the white key just left of them is C, one octave apart all the way up. Middle C is the C nearest the middle of the keyboard, and the tune you can pick out by ear - three blind mice, happy birthday - is a faster way to learn it than naming keys.")); __kws.extend(vec![("avoid".to_string(), __data28()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("first-keyboard")); __args.push(py_tuple(vec![py_tuple(vec![py_str("piano"), py_str("keyboard"), py_str("digital piano"), py_str("electric piano")]), __data29()])); __args.push(py_str("For a first piano, the thing that matters is weighted, full-size keys: a second-hand 88-key digital piano from Yamaha, Roland or Casio is 200 to 350 pounds and is the cheapest thing that teaches real finger strength. A 61-key keyboard with plastic keys is fine for a month to see if you like it, but it will not teach you to play, so hire or borrow one first if the money is tight - shops rent them monthly.")); __kws.extend(vec![("avoid".to_string(), __data30()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.music.ENTRIES"); }
    s_ENTRIES(__data13());
}
pub fn _not_an_infection(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True unless the fingertip is red, hot, swollen or weeping: that is a whitlow's question.");
    text = if py_truthy(&call_builtin("isinstance", &[mcall(&understood.clone(), "get", &[py_str("norm")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { let __b = mcall(&understood.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_str("") } } } else { py_str("") };
    text = { let __b = text.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("red"), py_str("swollen"), py_str("swelling"), py_str("pus"), py_str("infection"), py_str("infected"), py_str("whitlow"), py_str("abscess"), py_str("throbbing"), py_str("hot"), py_str("fever"), py_str("septic"), py_str("nail fold")]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &word.clone()))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_musical_line(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about making music, so a neighbour noise row cannot take a dog.");
    text = if py_truthy(&call_builtin("isinstance", &[mcall(&understood.clone(), "get", &[py_str("norm")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { let __b = mcall(&understood.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_str("") } } } else { py_str("") };
    text = op_add(&op_add(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &py_str(" ")), &{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } });
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("music"), py_str("practice"), py_str("practise"), py_str("practising"), py_str("practicing"), py_str("playing"), py_str("play"), py_str("guitar"), py_str("piano"), py_str("violin"), py_str("drums"), py_str("singing"), py_str("sing"), py_str("band"), py_str("instrument"), py_str("chords"), py_str("rehearsal"), py_str("rehearsing"), py_str("trumpet"), py_str("flute"), py_str("saxophone"), py_str("cello"), py_str("ukulele"), py_str("bass"), py_str("clarinet")]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&mcall(&text.clone(), "split", &[]), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_add(&op_add(&py_str(" "), &text.clone()), &py_str(" ")), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _staff_is_the_five_lines(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether \"staff\" in this line is the music, rather than the people who work somewhere.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    words = call_builtin("set", &[mcall(&mcall(&clean.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = mcall(&word.clone(), "startswith", &[py_str("staff")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__STAFF_TAILS(), &slice_or_raise(&word.clone(), &Py::Int(5), &Py::None, &Py::None))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MUSIC_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STAFF_MEANS_PEOPLE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[op_bitand(&words.clone(), &g__MUSIC_AROUND_STAFF())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.music".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "BROKEN" => Some(g_BROKEN()),
            "CLEANING" => Some(g_CLEANING()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FINGERS" => Some(g_FINGERS()),
            "KIT" => Some(g_KIT()),
            "MINE_HEARD" => Some(g_MINE_HEARD()),
            "NEW_STRINGS" => Some(g_NEW_STRINGS()),
            "OVERHEARD" => Some(g_OVERHEARD()),
            "PLAYING" => Some(g_PLAYING()),
            "QUIET" => Some(g_QUIET()),
            "STRINGS" => Some(g_STRINGS()),
            "_MUSIC_AROUND_STAFF" => Some(g__MUSIC_AROUND_STAFF()),
            "_MUSIC_PHRASES" => Some(g__MUSIC_PHRASES()),
            "_STAFF_MEANS_PEOPLE" => Some(g__STAFF_MEANS_PEOPLE()),
            "_STAFF_TAILS" => Some(g__STAFF_TAILS()),
            "_a_musical_line" => Some(py_func("_a_musical_line", |args: &[Py]| _a_musical_line(args))),
            "_not_an_infection" => Some(py_func("_not_an_infection", |args: &[Py]| _not_an_infection(args))),
            "_staff_is_the_five_lines" => Some(py_func("_staff_is_the_five_lines", |args: &[Py]| _staff_is_the_five_lines(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "BROKEN" => s_BROKEN(v),
            "CLEANING" => s_CLEANING(v),
            "ENTRIES" => s_ENTRIES(v),
            "FINGERS" => s_FINGERS(v),
            "KIT" => s_KIT(v),
            "MINE_HEARD" => s_MINE_HEARD(v),
            "NEW_STRINGS" => s_NEW_STRINGS(v),
            "OVERHEARD" => s_OVERHEARD(v),
            "PLAYING" => s_PLAYING(v),
            "QUIET" => s_QUIET(v),
            "STRINGS" => s_STRINGS(v),
            "_MUSIC_AROUND_STAFF" => s__MUSIC_AROUND_STAFF(v),
            "_MUSIC_PHRASES" => s__MUSIC_PHRASES(v),
            "_STAFF_MEANS_PEOPLE" => s__STAFF_MEANS_PEOPLE(v),
            "_STAFF_TAILS" => s__STAFF_TAILS(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.music", module_obj);
    pyrt::xrefs::reg_fn("knowledge.music", "_not_an_infection", py_func("_not_an_infection", |args: &[Py]| _not_an_infection(args)));
    pyrt::xrefs::reg_fn("knowledge.music", "_a_musical_line", py_func("_a_musical_line", |args: &[Py]| _a_musical_line(args)));
    pyrt::xrefs::reg_fn("knowledge.music", "_staff_is_the_five_lines", py_func("_staff_is_the_five_lines", |args: &[Py]| _staff_is_the_five_lines(args)));
    pyrt::xrefs::reg_access("knowledge.music", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.music", "BROKEN", g_BROKEN, s_BROKEN);
    pyrt::xrefs::reg_access("knowledge.music", "CLEANING", g_CLEANING, s_CLEANING);
    pyrt::xrefs::reg_access("knowledge.music", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.music", "FINGERS", g_FINGERS, s_FINGERS);
    pyrt::xrefs::reg_access("knowledge.music", "KIT", g_KIT, s_KIT);
    pyrt::xrefs::reg_access("knowledge.music", "MINE_HEARD", g_MINE_HEARD, s_MINE_HEARD);
    pyrt::xrefs::reg_access("knowledge.music", "NEW_STRINGS", g_NEW_STRINGS, s_NEW_STRINGS);
    pyrt::xrefs::reg_access("knowledge.music", "OVERHEARD", g_OVERHEARD, s_OVERHEARD);
    pyrt::xrefs::reg_access("knowledge.music", "PLAYING", g_PLAYING, s_PLAYING);
    pyrt::xrefs::reg_access("knowledge.music", "QUIET", g_QUIET, s_QUIET);
    pyrt::xrefs::reg_access("knowledge.music", "STRINGS", g_STRINGS, s_STRINGS);
    pyrt::xrefs::reg_access("knowledge.music", "_MUSIC_AROUND_STAFF", g__MUSIC_AROUND_STAFF, s__MUSIC_AROUND_STAFF);
    pyrt::xrefs::reg_access("knowledge.music", "_MUSIC_PHRASES", g__MUSIC_PHRASES, s__MUSIC_PHRASES);
    pyrt::xrefs::reg_access("knowledge.music", "_STAFF_MEANS_PEOPLE", g__STAFF_MEANS_PEOPLE, s__STAFF_MEANS_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.music", "_STAFF_TAILS", g__STAFF_TAILS, s__STAFF_TAILS);
    pyrt::xrefs::reg_access("knowledge.music", "entry", g_entry, s_entry);
}
