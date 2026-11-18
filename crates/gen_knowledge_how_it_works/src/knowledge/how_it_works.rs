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
thread_local! { static G_AN_INDUCTION_HOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HEARING_AID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PACEMAKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SEPTIC_TANK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SPIN_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRUISE_CONTROL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECAF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MECHANISM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.how_it_works: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_INDUCTION_HOB() -> Py { ensure_init(); G_AN_INDUCTION_HOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_INDUCTION_HOB(v: Py) { ensure_init(); G_AN_INDUCTION_HOB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HEARING_AID() -> Py { ensure_init(); G_A_HEARING_AID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HEARING_AID(v: Py) { ensure_init(); G_A_HEARING_AID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PACEMAKER() -> Py { ensure_init(); G_A_PACEMAKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PACEMAKER(v: Py) { ensure_init(); G_A_PACEMAKER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SAIL() -> Py { ensure_init(); G_A_SAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SAIL(v: Py) { ensure_init(); G_A_SAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SEPTIC_TANK() -> Py { ensure_init(); G_A_SEPTIC_TANK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SEPTIC_TANK(v: Py) { ensure_init(); G_A_SEPTIC_TANK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SPIN_CLASS() -> Py { ensure_init(); G_A_SPIN_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SPIN_CLASS(v: Py) { ensure_init(); G_A_SPIN_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_CRUISE_CONTROL() -> Py { ensure_init(); G_CRUISE_CONTROL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRUISE_CONTROL(v: Py) { ensure_init(); G_CRUISE_CONTROL.with(|c| *c.borrow_mut() = v); }
pub fn g_DECAF() -> Py { ensure_init(); G_DECAF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECAF(v: Py) { ensure_init(); G_DECAF.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MECHANISM() -> Py { ensure_init(); G_MECHANISM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MECHANISM(v: Py) { ensure_init(); G_MECHANISM.with(|c| *c.borrow_mut() = v); }
pub fn g_TROUBLE() -> Py { ensure_init(); G_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TROUBLE(v: Py) { ensure_init(); G_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How an everyday thing works: the mechanism, said in one short card.\n\n`explains.py` owns this question and its asking words, and these eight subjects had no row in\nit, so the line fell through to the composer and came back as something else entirely:\n\n    how does a hearing aid work        \"Nothing of it reached me here, so tell me: how did it go?\"\n    how does a pacemaker work          the same card again\n    how does a septic tank work        the same card again\n    how does a sail work               \"I wasn't there, so you're the one who knows\"\n    what is a spinning class           \"I'm not sure about that one, sorry\"\n    what does cruise control do        \"I don't know enough to answer that honestly\"\n    what is decaf coffee               \"That's beyond what I know well, sorry\"\n    how does an induction hob work     the instructions for cleaning a hob\n\nThat last one is the worst of them: the hob's cleaning row keys the word \"hob\", so a person\nasking how induction heats a pan was told to wipe it with hot soapy water. Each row here needs\nthe thing *and* the ask for its mechanism, so \"how much does a hearing aid cost\", \"my septic\ntank smells\" and \"how do i clean an induction hob\" keep the answers they had.\n\nA separate file rather than rows inside `explains.py` because that file is over 2,000 lines and\na new domain registers itself.\n");
    fn __data1() -> Py { py_tuple(vec![py_str("work"), py_str("works"), py_str("working"), py_str("explain"), py_str("explains"), py_str("explained"), py_str("what is"), py_str("what are"), py_str("what's"), py_str("whats"), py_str("what does"), py_str("what do"), py_str("inside"), py_str("mean"), py_str("means"), py_str("actually"), py_str("how does"), py_str("how do they"), py_str("how do these"), py_str("how do those")]) }
    s_MECHANISM(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("not"), py_str("isnt"), py_str("isn't"), py_str("doesnt"), py_str("doesn't"), py_str("wont"), py_str("won't"), py_str("broken"), py_str("broke"), py_str("fault"), py_str("faulty"), py_str("error"), py_str("fix"), py_str("repair"), py_str("replacement"), py_str("replace"), py_str("leaking"), py_str("leak"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("noise"), py_str("noisy"), py_str("beep"), py_str("beeping"), py_str("stopped"), py_str("stopping"), py_str("help"), py_str("problem"), py_str("problem with"), py_str("wrong"), py_str("dead"), py_str("charge"), py_str("charging"), py_str("battery"), py_str("cost"), py_str("costs"), py_str("price"), py_str("much"), py_str("cheap"), py_str("cheapest"), py_str("buy"), py_str("buying"), py_str("best"), py_str("better"), py_str("worth"), py_str("review"), py_str("recommend"), py_str("deals")]) }
    s_TROUBLE(__data2());
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("technology"), py_str("home-and-diy"), py_str("health"), py_str("food-and-drink"), py_str("sport")]));
    s_A_HEARING_AID(py_str("A hearing aid is a microphone, an amplifier and a tiny speaker in your ear: the microphone picks up the room, the electronics boost the frequencies speech lives in and the speaker sends it down the canal louder. Digital ones split the sound into bands and turn down the background to suit the room. It does not give you normal hearing back, so it takes a few weeks of wearing it all day to stop noticing it. NHS aids are free."));
    s_A_PACEMAKER(py_str("A pacemaker is a matchbox-sized unit under the collarbone with one or two thin leads into the heart: it watches the beat and sends a small electrical pulse when it goes too slow or skips, so the heart keeps a steady 60 or so. Fitting one takes about an hour under a local anaesthetic, the battery lasts 8 to 12 years and is swapped with the whole unit, and a yearly check reads it through the skin."));
    s_A_SEPTIC_TANK(py_str("A septic tank is an underground tank for a house with no mains sewer: waste flows in, the solids sink and settle, bacteria break them down, and the clearer liquid runs out through pipes into a soakaway in the ground. Nothing goes in it but pee, poo and paper - wipes, fat and bleach are what kill the bacteria - and a tanker has to empty it every year or two. A blocked or smelly one is usually a soakaway that has had its chips."));
    s_A_SPIN_CLASS(py_str("A spinning class is indoor cycling on a fixed bike for about 45 minutes to music: the instructor calls out when to stand, sprint or add resistance, and you turn the resistance knob yourself, so the effort is as hard as you make it. Set the saddle to hip height, take a towel and water, and start with the resistance light enough that you could still talk."));
    s_A_SAIL(py_str("A sail works like a wing stood on end: the wind crossing the curved side travels further and faster than the wind on the other side, the pressure drops, and the sail is pulled forward. That is why a boat can head towards the wind but not straight into it - you work about 45 degrees off it and zigzag, turning through the wind to gain ground, which is tacking."));
    s_CRUISE_CONTROL(py_str("Cruise control holds the speed you set by working the throttle for you: get to the speed, press set, and the car holds it until you touch the brake or clutch, which cancels it. Use it on a motorway in dry weather, never in rain, traffic or on bends, because it will not slow for anything ahead - only the adaptive kind follows the car in front."));
    s_DECAF(py_str("Decaf is ordinary coffee with the caffeine taken out of the beans before roasting, usually by soaking them in water, a solvent or carbon dioxide: a mug has 2 to 15mg against about 95mg in regular coffee, and the taste and antioxidants are much the same. It is the swap to make if caffeine is costing you sleep, without losing the ritual or the afternoon mug."));
    s_AN_INDUCTION_HOB(py_str("An induction hob makes the pan the heater: a coil under the glass sends a fast-changing magnetic field into the base, which sets up currents in the metal and heats it directly, so the glass around it stays cool enough to touch. It is quicker and more controllable than gas, and the pan must be magnetic - if a magnet sticks to its base, it will work."));
    fn __data3() -> Py { py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("cleaned"), py_str("wipe"), py_str("wiping"), py_str("burnt"), py_str("burned"), py_str("scratch"), py_str("scratched"), py_str("scratches"), py_str("mark"), py_str("marks"), py_str("stain"), py_str("stains"), py_str("pan"), py_str("pans"), py_str("saucepan"), py_str("wok"), py_str("fit"), py_str("install"), py_str("installation"), py_str("socket"), py_str("wiring"), py_str("noise"), py_str("clicking")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-hearing-aid-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hearing aid"), py_str("hearing aids"), py_str("a hearing aid"), py_str("hearing aids work")]), g_MECHANISM()])); __args.push(g_A_HEARING_AID()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-pacemaker-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pacemaker"), py_str("pacemakers"), py_str("a pacemaker"), py_str("heart pacemaker")]), g_MECHANISM()])); __args.push(g_A_PACEMAKER()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-septic-tank-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("septic tank"), py_str("septic tanks"), py_str("a septic tank"), py_str("cesspit"), py_str("cess pool")]), g_MECHANISM()])); __args.push(g_A_SEPTIC_TANK()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-a-spinning-class-is")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spinning class"), py_str("spinning classes"), py_str("spin class"), py_str("spin classes"), py_str("spinning session"), py_str("indoor cycling class")]), g_MECHANISM()])); __args.push(g_A_SPIN_CLASS()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-sail-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("how a sail works"), py_str("how sails work"), py_str("how sailing works"), py_str("a sail works"), py_str("how does a sail work"), py_str("how do sails work"), py_str("how does sailing work"), py_str("sail work"), py_str("sails work")]), py_tuple(vec![py_str("sail"), py_str("sails"), py_str("sailing"), py_str("sailboat"), py_str("yacht"), py_str("boat")])])); __args.push(g_A_SAIL()); __kws.extend(vec![("avoid".to_string(), op_add(&g_TROUBLE(), &py_tuple(vec![py_str("holiday"), py_str("holidays"), py_str("cruise"), py_str("ferry"), py_str("set sail"), py_str("sail away"), py_str("sail to"), py_str("sailing holiday")]))), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-cruise-control-does")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cruise control"), py_str("adaptive cruise control"), py_str("speed limiter")]), g_MECHANISM()])); __args.push(g_CRUISE_CONTROL()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-decaf-coffee-is")); __args.push(py_tuple(vec![py_tuple(vec![py_str("decaf"), py_str("decaffeinated"), py_str("decaf coffee"), py_str("decaf coffee work")]), g_MECHANISM()])); __args.push(g_DECAF()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-an-induction-hob-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("induction hob"), py_str("induction hobs"), py_str("an induction hob"), py_str("induction cooker"), py_str("induction ring"), py_str("induction rings"), py_str("induction work")]), g_MECHANISM()])); __args.push(g_AN_INDUCTION_HOB()); __kws.extend(vec![("avoid".to_string(), op_add(&g_TROUBLE(), &__data3())), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.how_it_works".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "AN_INDUCTION_HOB" => Some(g_AN_INDUCTION_HOB()),
            "A_HEARING_AID" => Some(g_A_HEARING_AID()),
            "A_PACEMAKER" => Some(g_A_PACEMAKER()),
            "A_SAIL" => Some(g_A_SAIL()),
            "A_SEPTIC_TANK" => Some(g_A_SEPTIC_TANK()),
            "A_SPIN_CLASS" => Some(g_A_SPIN_CLASS()),
            "CRUISE_CONTROL" => Some(g_CRUISE_CONTROL()),
            "DECAF" => Some(g_DECAF()),
            "ENTRIES" => Some(g_ENTRIES()),
            "MECHANISM" => Some(g_MECHANISM()),
            "TROUBLE" => Some(g_TROUBLE()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "AN_INDUCTION_HOB" => s_AN_INDUCTION_HOB(v),
            "A_HEARING_AID" => s_A_HEARING_AID(v),
            "A_PACEMAKER" => s_A_PACEMAKER(v),
            "A_SAIL" => s_A_SAIL(v),
            "A_SEPTIC_TANK" => s_A_SEPTIC_TANK(v),
            "A_SPIN_CLASS" => s_A_SPIN_CLASS(v),
            "CRUISE_CONTROL" => s_CRUISE_CONTROL(v),
            "DECAF" => s_DECAF(v),
            "ENTRIES" => s_ENTRIES(v),
            "MECHANISM" => s_MECHANISM(v),
            "TROUBLE" => s_TROUBLE(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.how_it_works", module_obj);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "AN_INDUCTION_HOB", g_AN_INDUCTION_HOB, s_AN_INDUCTION_HOB);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "A_HEARING_AID", g_A_HEARING_AID, s_A_HEARING_AID);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "A_PACEMAKER", g_A_PACEMAKER, s_A_PACEMAKER);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "A_SAIL", g_A_SAIL, s_A_SAIL);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "A_SEPTIC_TANK", g_A_SEPTIC_TANK, s_A_SEPTIC_TANK);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "A_SPIN_CLASS", g_A_SPIN_CLASS, s_A_SPIN_CLASS);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "CRUISE_CONTROL", g_CRUISE_CONTROL, s_CRUISE_CONTROL);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "DECAF", g_DECAF, s_DECAF);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "MECHANISM", g_MECHANISM, s_MECHANISM);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "TROUBLE", g_TROUBLE, s_TROUBLE);
    pyrt::xrefs::reg_access("knowledge.how_it_works", "entry", g_entry, s_entry);
}
