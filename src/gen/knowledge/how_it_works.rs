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


pub static ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_INDUCTION_HOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HEARING_AID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PACEMAKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SEPTIC_TANK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SPIN_CLASS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CRUISE_CONTROL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DECAF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MECHANISM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TROUBLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS() -> Py { ensure_init(); ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS.write() { *w = v; } }
pub fn g_AN_INDUCTION_HOB() -> Py { ensure_init(); AN_INDUCTION_HOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_INDUCTION_HOB(v: Py) { ensure_init(); if let Ok(mut w) = AN_INDUCTION_HOB.write() { *w = v; } }
pub fn g_A_HEARING_AID() -> Py { ensure_init(); A_HEARING_AID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HEARING_AID(v: Py) { ensure_init(); if let Ok(mut w) = A_HEARING_AID.write() { *w = v; } }
pub fn g_A_PACEMAKER() -> Py { ensure_init(); A_PACEMAKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PACEMAKER(v: Py) { ensure_init(); if let Ok(mut w) = A_PACEMAKER.write() { *w = v; } }
pub fn g_A_SAIL() -> Py { ensure_init(); A_SAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SAIL(v: Py) { ensure_init(); if let Ok(mut w) = A_SAIL.write() { *w = v; } }
pub fn g_A_SEPTIC_TANK() -> Py { ensure_init(); A_SEPTIC_TANK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SEPTIC_TANK(v: Py) { ensure_init(); if let Ok(mut w) = A_SEPTIC_TANK.write() { *w = v; } }
pub fn g_A_SPIN_CLASS() -> Py { ensure_init(); A_SPIN_CLASS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SPIN_CLASS(v: Py) { ensure_init(); if let Ok(mut w) = A_SPIN_CLASS.write() { *w = v; } }
pub fn g_CRUISE_CONTROL() -> Py { ensure_init(); CRUISE_CONTROL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CRUISE_CONTROL(v: Py) { ensure_init(); if let Ok(mut w) = CRUISE_CONTROL.write() { *w = v; } }
pub fn g_DECAF() -> Py { ensure_init(); DECAF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DECAF(v: Py) { ensure_init(); if let Ok(mut w) = DECAF.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g_MECHANISM() -> Py { ensure_init(); MECHANISM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MECHANISM(v: Py) { ensure_init(); if let Ok(mut w) = MECHANISM.write() { *w = v; } }
pub fn g_TROUBLE() -> Py { ensure_init(); TROUBLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TROUBLE(v: Py) { ensure_init(); if let Ok(mut w) = TROUBLE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("How an everyday thing works: the mechanism, said in one short card.\n\n`explains.py` owns this question and its asking words, and these eight subjects had no row in\nit, so the line fell through to the composer and came back as something else entirely:\n\n    how does a hearing aid work        \"Nothing of it reached me here, so tell me: how did it go?\"\n    how does a pacemaker work          the same card again\n    how does a septic tank work        the same card again\n    how does a sail work               \"I wasn't there, so you're the one who knows\"\n    what is a spinning class           \"I'm not sure about that one, sorry\"\n    what does cruise control do        \"I don't know enough to answer that honestly\"\n    what is decaf coffee               \"That's beyond what I know well, sorry\"\n    how does an induction hob work     the instructions for cleaning a hob\n\nThat last one is the worst of them: the hob's cleaning row keys the word \"hob\", so a person\nasking how induction heats a pan was told to wipe it with hot soapy water. Each row here needs\nthe thing *and* the ask for its mechanism, so \"how much does a hearing aid cost\", \"my septic\ntank smells\" and \"how do i clean an induction hob\" keep the answers they had.\n\nA separate file rather than rows inside `explains.py` because that file is over 2,000 lines and\na new domain registers itself.\n");
    s_MECHANISM(py_tuple(vec![py_str("work"), py_str("works"), py_str("working"), py_str("explain"), py_str("explains"), py_str("explained"), py_str("what is"), py_str("what are"), py_str("what's"), py_str("whats"), py_str("what does"), py_str("what do"), py_str("inside"), py_str("mean"), py_str("means"), py_str("actually"), py_str("how does"), py_str("how do they"), py_str("how do these"), py_str("how do those")]));
    s_TROUBLE(py_tuple(vec![py_str("not"), py_str("isnt"), py_str("isn't"), py_str("doesnt"), py_str("doesn't"), py_str("wont"), py_str("won't"), py_str("broken"), py_str("broke"), py_str("fault"), py_str("faulty"), py_str("error"), py_str("fix"), py_str("repair"), py_str("replacement"), py_str("replace"), py_str("leaking"), py_str("leak"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("noise"), py_str("noisy"), py_str("beep"), py_str("beeping"), py_str("stopped"), py_str("stopping"), py_str("help"), py_str("problem"), py_str("problem with"), py_str("wrong"), py_str("dead"), py_str("charge"), py_str("charging"), py_str("battery"), py_str("cost"), py_str("costs"), py_str("price"), py_str("much"), py_str("cheap"), py_str("cheapest"), py_str("buy"), py_str("buying"), py_str("best"), py_str("better"), py_str("worth"), py_str("review"), py_str("recommend"), py_str("deals")]));
    s_ANSWERS(py_tuple(vec![py_str("technology"), py_str("home-and-diy"), py_str("health"), py_str("food-and-drink"), py_str("sport")]));
    s_A_HEARING_AID(py_str("A hearing aid is a microphone, an amplifier and a tiny speaker in your ear: the microphone picks up the room, the electronics boost the frequencies speech lives in and the speaker sends it down the canal louder. Digital ones split the sound into bands and turn down the background to suit the room. It does not give you normal hearing back, so it takes a few weeks of wearing it all day to stop noticing it. NHS aids are free."));
    s_A_PACEMAKER(py_str("A pacemaker is a matchbox-sized unit under the collarbone with one or two thin leads into the heart: it watches the beat and sends a small electrical pulse when it goes too slow or skips, so the heart keeps a steady 60 or so. Fitting one takes about an hour under a local anaesthetic, the battery lasts 8 to 12 years and is swapped with the whole unit, and a yearly check reads it through the skin."));
    s_A_SEPTIC_TANK(py_str("A septic tank is an underground tank for a house with no mains sewer: waste flows in, the solids sink and settle, bacteria break them down, and the clearer liquid runs out through pipes into a soakaway in the ground. Nothing goes in it but pee, poo and paper - wipes, fat and bleach are what kill the bacteria - and a tanker has to empty it every year or two. A blocked or smelly one is usually a soakaway that has had its chips."));
    s_A_SPIN_CLASS(py_str("A spinning class is indoor cycling on a fixed bike for about 45 minutes to music: the instructor calls out when to stand, sprint or add resistance, and you turn the resistance knob yourself, so the effort is as hard as you make it. Set the saddle to hip height, take a towel and water, and start with the resistance light enough that you could still talk."));
    s_A_SAIL(py_str("A sail works like a wing stood on end: the wind crossing the curved side travels further and faster than the wind on the other side, the pressure drops, and the sail is pulled forward. That is why a boat can head towards the wind but not straight into it - you work about 45 degrees off it and zigzag, turning through the wind to gain ground, which is tacking."));
    s_CRUISE_CONTROL(py_str("Cruise control holds the speed you set by working the throttle for you: get to the speed, press set, and the car holds it until you touch the brake or clutch, which cancels it. Use it on a motorway in dry weather, never in rain, traffic or on bends, because it will not slow for anything ahead - only the adaptive kind follows the car in front."));
    s_DECAF(py_str("Decaf is ordinary coffee with the caffeine taken out of the beans before roasting, usually by soaking them in water, a solvent or carbon dioxide: a mug has 2 to 15mg against about 95mg in regular coffee, and the taste and antioxidants are much the same. It is the swap to make if caffeine is costing you sleep, without losing the ritual or the afternoon mug."));
    s_AN_INDUCTION_HOB(py_str("An induction hob makes the pan the heater: a coil under the glass sends a fast-changing magnetic field into the base, which sets up currents in the metal and heats it directly, so the glass around it stays cool enough to touch. It is quicker and more controllable than gas, and the pan must be magnetic - if a magnet sticks to its base, it will work."));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-hearing-aid-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hearing aid"), py_str("hearing aids"), py_str("a hearing aid"), py_str("hearing aids work")]), g_MECHANISM()])); __args.push(g_A_HEARING_AID()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-pacemaker-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pacemaker"), py_str("pacemakers"), py_str("a pacemaker"), py_str("heart pacemaker")]), g_MECHANISM()])); __args.push(g_A_PACEMAKER()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-septic-tank-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("septic tank"), py_str("septic tanks"), py_str("a septic tank"), py_str("cesspit"), py_str("cess pool")]), g_MECHANISM()])); __args.push(g_A_SEPTIC_TANK()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-a-spinning-class-is")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spinning class"), py_str("spinning classes"), py_str("spin class"), py_str("spin classes"), py_str("spinning session"), py_str("indoor cycling class")]), g_MECHANISM()])); __args.push(g_A_SPIN_CLASS()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-a-sail-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("how a sail works"), py_str("how sails work"), py_str("how sailing works"), py_str("a sail works"), py_str("how does a sail work"), py_str("how do sails work"), py_str("how does sailing work"), py_str("sail work"), py_str("sails work")]), py_tuple(vec![py_str("sail"), py_str("sails"), py_str("sailing"), py_str("sailboat"), py_str("yacht"), py_str("boat")])])); __args.push(g_A_SAIL()); __kws.extend(vec![("avoid".to_string(), add(&g_TROUBLE(), &py_tuple(vec![py_str("holiday"), py_str("holidays"), py_str("cruise"), py_str("ferry"), py_str("set sail"), py_str("sail away"), py_str("sail to"), py_str("sailing holiday")]))), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-cruise-control-does")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cruise control"), py_str("adaptive cruise control"), py_str("speed limiter")]), g_MECHANISM()])); __args.push(g_CRUISE_CONTROL()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-decaf-coffee-is")); __args.push(py_tuple(vec![py_tuple(vec![py_str("decaf"), py_str("decaffeinated"), py_str("decaf coffee"), py_str("decaf coffee work")]), g_MECHANISM()])); __args.push(g_DECAF()); __kws.extend(vec![("avoid".to_string(), g_TROUBLE()), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-an-induction-hob-works")); __args.push(py_tuple(vec![py_tuple(vec![py_str("induction hob"), py_str("induction hobs"), py_str("an induction hob"), py_str("induction cooker"), py_str("induction ring"), py_str("induction rings"), py_str("induction work")]), g_MECHANISM()])); __args.push(g_AN_INDUCTION_HOB()); __kws.extend(vec![("avoid".to_string(), add(&g_TROUBLE(), &py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("cleaned"), py_str("wipe"), py_str("wiping"), py_str("burnt"), py_str("burned"), py_str("scratch"), py_str("scratched"), py_str("scratches"), py_str("mark"), py_str("marks"), py_str("stain"), py_str("stains"), py_str("pan"), py_str("pans"), py_str("saucepan"), py_str("wok"), py_str("fit"), py_str("install"), py_str("installation"), py_str("socket"), py_str("wiring"), py_str("noise"), py_str("clicking")]))), ("ask".to_string(), g_MECHANISM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
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
            _ => None,
        }),
        set: None,
    }))
}
