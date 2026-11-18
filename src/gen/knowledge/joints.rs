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
pub static AVOID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPLACING_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIP_KNEE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HURTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IMPLANT_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IMPLANT_LIFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KNEEL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KNEEL_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCKED_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCK_PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOSE_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEW_JOINT_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEW_JOINT_NOISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOISY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPLACED_JOINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROAD_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SCAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SCAR_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOULDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOULDER_IMPLANT_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOULDER_SLEEPING_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOULDER_SORE_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHOULDER_TIMELINE_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEPING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEPING_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SORE_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAIRS_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWOLLEN_LEG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWOLLEN_LEG_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_JOINT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIMELINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIMELINE_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIGGER_FINGER_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WASH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WASH_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AVOID() -> Py { ensure_init(); AVOID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AVOID(v: Py) { ensure_init(); if let Ok(mut w) = AVOID.write() { *w = v; } }
pub fn g_A_REPLACING_A_THING() -> Py { ensure_init(); A_REPLACING_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPLACING_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_REPLACING_A_THING.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g_HIP_KNEE() -> Py { ensure_init(); HIP_KNEE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIP_KNEE(v: Py) { ensure_init(); if let Ok(mut w) = HIP_KNEE.write() { *w = v; } }
pub fn g_HURTS() -> Py { ensure_init(); HURTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HURTS(v: Py) { ensure_init(); if let Ok(mut w) = HURTS.write() { *w = v; } }
pub fn g_IMPLANT_CARD() -> Py { ensure_init(); IMPLANT_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IMPLANT_CARD(v: Py) { ensure_init(); if let Ok(mut w) = IMPLANT_CARD.write() { *w = v; } }
pub fn g_IMPLANT_LIFE() -> Py { ensure_init(); IMPLANT_LIFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IMPLANT_LIFE(v: Py) { ensure_init(); if let Ok(mut w) = IMPLANT_LIFE.write() { *w = v; } }
pub fn g_KNEEL() -> Py { ensure_init(); KNEEL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KNEEL(v: Py) { ensure_init(); if let Ok(mut w) = KNEEL.write() { *w = v; } }
pub fn g_KNEEL_CARD() -> Py { ensure_init(); KNEEL_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KNEEL_CARD(v: Py) { ensure_init(); if let Ok(mut w) = KNEEL_CARD.write() { *w = v; } }
pub fn g_LOCKED() -> Py { ensure_init(); LOCKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCKED(v: Py) { ensure_init(); if let Ok(mut w) = LOCKED.write() { *w = v; } }
pub fn g_LOCKED_CARD() -> Py { ensure_init(); LOCKED_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCKED_CARD(v: Py) { ensure_init(); if let Ok(mut w) = LOCKED_CARD.write() { *w = v; } }
pub fn g_LOCK_PARTS() -> Py { ensure_init(); LOCK_PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCK_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = LOCK_PARTS.write() { *w = v; } }
pub fn g_LOOSE() -> Py { ensure_init(); LOOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOSE(v: Py) { ensure_init(); if let Ok(mut w) = LOOSE.write() { *w = v; } }
pub fn g_LOOSE_CARD() -> Py { ensure_init(); LOOSE_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOSE_CARD(v: Py) { ensure_init(); if let Ok(mut w) = LOOSE_CARD.write() { *w = v; } }
pub fn g_NEW_JOINT_CARD() -> Py { ensure_init(); NEW_JOINT_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEW_JOINT_CARD(v: Py) { ensure_init(); if let Ok(mut w) = NEW_JOINT_CARD.write() { *w = v; } }
pub fn g_NEW_JOINT_NOISE() -> Py { ensure_init(); NEW_JOINT_NOISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEW_JOINT_NOISE(v: Py) { ensure_init(); if let Ok(mut w) = NEW_JOINT_NOISE.write() { *w = v; } }
pub fn g_NOISY() -> Py { ensure_init(); NOISY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOISY(v: Py) { ensure_init(); if let Ok(mut w) = NOISY.write() { *w = v; } }
pub fn g_ONE() -> Py { ensure_init(); ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONE(v: Py) { ensure_init(); if let Ok(mut w) = ONE.write() { *w = v; } }
pub fn g_PARTS() -> Py { ensure_init(); PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = PARTS.write() { *w = v; } }
pub fn g_REPLACED_JOINT() -> Py { ensure_init(); REPLACED_JOINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPLACED_JOINT(v: Py) { ensure_init(); if let Ok(mut w) = REPLACED_JOINT.write() { *w = v; } }
pub fn g_ROAD_WORDS() -> Py { ensure_init(); ROAD_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROAD_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ROAD_WORDS.write() { *w = v; } }
pub fn g_SCAR() -> Py { ensure_init(); SCAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SCAR(v: Py) { ensure_init(); if let Ok(mut w) = SCAR.write() { *w = v; } }
pub fn g_SCAR_CARD() -> Py { ensure_init(); SCAR_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SCAR_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SCAR_CARD.write() { *w = v; } }
pub fn g_SHOULDER() -> Py { ensure_init(); SHOULDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOULDER(v: Py) { ensure_init(); if let Ok(mut w) = SHOULDER.write() { *w = v; } }
pub fn g_SHOULDER_IMPLANT_CARD() -> Py { ensure_init(); SHOULDER_IMPLANT_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOULDER_IMPLANT_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SHOULDER_IMPLANT_CARD.write() { *w = v; } }
pub fn g_SHOULDER_SLEEPING_CARD() -> Py { ensure_init(); SHOULDER_SLEEPING_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOULDER_SLEEPING_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SHOULDER_SLEEPING_CARD.write() { *w = v; } }
pub fn g_SHOULDER_SORE_CARD() -> Py { ensure_init(); SHOULDER_SORE_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOULDER_SORE_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SHOULDER_SORE_CARD.write() { *w = v; } }
pub fn g_SHOULDER_TIMELINE_CARD() -> Py { ensure_init(); SHOULDER_TIMELINE_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHOULDER_TIMELINE_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SHOULDER_TIMELINE_CARD.write() { *w = v; } }
pub fn g_SLEEPING() -> Py { ensure_init(); SLEEPING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEPING(v: Py) { ensure_init(); if let Ok(mut w) = SLEEPING.write() { *w = v; } }
pub fn g_SLEEPING_CARD() -> Py { ensure_init(); SLEEPING_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEPING_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SLEEPING_CARD.write() { *w = v; } }
pub fn g_SORE() -> Py { ensure_init(); SORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SORE(v: Py) { ensure_init(); if let Ok(mut w) = SORE.write() { *w = v; } }
pub fn g_SORE_CARD() -> Py { ensure_init(); SORE_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SORE_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SORE_CARD.write() { *w = v; } }
pub fn g_STAIRS() -> Py { ensure_init(); STAIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAIRS(v: Py) { ensure_init(); if let Ok(mut w) = STAIRS.write() { *w = v; } }
pub fn g_STAIRS_CARD() -> Py { ensure_init(); STAIRS_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAIRS_CARD(v: Py) { ensure_init(); if let Ok(mut w) = STAIRS_CARD.write() { *w = v; } }
pub fn g_SWOLLEN_LEG() -> Py { ensure_init(); SWOLLEN_LEG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWOLLEN_LEG(v: Py) { ensure_init(); if let Ok(mut w) = SWOLLEN_LEG.write() { *w = v; } }
pub fn g_SWOLLEN_LEG_CARD() -> Py { ensure_init(); SWOLLEN_LEG_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWOLLEN_LEG_CARD(v: Py) { ensure_init(); if let Ok(mut w) = SWOLLEN_LEG_CARD.write() { *w = v; } }
pub fn g_THE_JOINT_WORDS() -> Py { ensure_init(); THE_JOINT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_JOINT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = THE_JOINT_WORDS.write() { *w = v; } }
pub fn g_TIMELINE() -> Py { ensure_init(); TIMELINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIMELINE(v: Py) { ensure_init(); if let Ok(mut w) = TIMELINE.write() { *w = v; } }
pub fn g_TIMELINE_CARD() -> Py { ensure_init(); TIMELINE_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIMELINE_CARD(v: Py) { ensure_init(); if let Ok(mut w) = TIMELINE_CARD.write() { *w = v; } }
pub fn g_TRIGGER_FINGER_CARD() -> Py { ensure_init(); TRIGGER_FINGER_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIGGER_FINGER_CARD(v: Py) { ensure_init(); if let Ok(mut w) = TRIGGER_FINGER_CARD.write() { *w = v; } }
pub fn g_WASH() -> Py { ensure_init(); WASH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WASH(v: Py) { ensure_init(); if let Ok(mut w) = WASH.write() { *w = v; } }
pub fn g_WASH_CARD() -> Py { ensure_init(); WASH_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WASH_CARD(v: Py) { ensure_init(); if let Ok(mut w) = WASH_CARD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A joint that clicks, pops or grinds: the noise people notice and then ask about.\n\n\"my knuckle clicks when i bend it\", \"my jaw clicks when i eat\", \"my wrist clicks when i move it\"\nare everyday lines, typed as a symptom rather than a question, so the key is the part plus the\nword for the noise. The knee has had an answer of its own since the sport domain was written\n(bot/knowledge/sport.py, knee-noise), and every other joint had none: the line came back as an\necho, which is the worst answer for a person who is quietly wondering whether it is serious.\n\nThe reply is built from the part the line names, because a clicking jaw and a clicking knuckle\nare not the same worry, and \"a joint of yours clicks\" reads as if nobody heard which one.\n\nA line that also says it hurts is not this entry at all: the body-part entries already answer\n\"my wrist clicks when i move it and it hurts\" with what a wrist pain probably is and what to do\nabout it, and a general \"have it looked at\" would say less than the answer it replaced. So the\nwords for hurting are in AVOID, and this entry answers the noise on its own.\n\nThe words of a machine and of a sense are held out in AVOID: cars, boilers, phones and doors\nclick all day, and an ear that clicks is a hearing question, not a joint.\n");
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    s_NOISY(py_tuple(vec![py_str("click"), py_str("clicks"), py_str("clicked"), py_str("clicking"), py_str("pop"), py_str("pops"), py_str("popped"), py_str("popping"), py_str("creak"), py_str("creaks"), py_str("creaking"), py_str("grind"), py_str("grinds"), py_str("grinding"), py_str("snap"), py_str("snaps"), py_str("snapping"), py_str("crack"), py_str("cracks"), py_str("cracking"), py_str("cracked"), py_str("noisy"), py_str("noise"), py_str("clunk"), py_str("clunks"), py_str("clunking")]));
    s_PARTS(py_tuple(vec![py_str("knuckle"), py_str("knuckles"), py_str("jaw"), py_str("jaws"), py_str("wrist"), py_str("wrists"), py_str("elbow"), py_str("elbows"), py_str("shoulder"), py_str("shoulders"), py_str("hip"), py_str("hips"), py_str("ankle"), py_str("ankles"), py_str("thumb"), py_str("thumbs"), py_str("toe"), py_str("toes"), py_str("finger"), py_str("fingers"), py_str("joint"), py_str("joints"), py_str("neck"), py_str("spine")]));
    s_ONE(py_dict(vec![(py_str("knuckles"), py_str("knuckle")), (py_str("jaws"), py_str("jaw")), (py_str("wrists"), py_str("wrist")), (py_str("elbows"), py_str("elbow")), (py_str("shoulders"), py_str("shoulder")), (py_str("hips"), py_str("hip")), (py_str("ankles"), py_str("ankle")), (py_str("thumbs"), py_str("thumb")), (py_str("toes"), py_str("toe")), (py_str("fingers"), py_str("finger")), (py_str("joints"), py_str("joint"))]));
    s_HURTS(py_tuple(vec![py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("pains"), py_str("painful"), py_str("sore"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("achy"), py_str("swollen"), py_str("swelling"), py_str("swelled"), py_str("stiff"), py_str("locked"), py_str("locking"), py_str("locks"), py_str("stuck"), py_str("sharp")]));
    s_AVOID(py_tuple(vec![py_str("knee"), py_str("knees"), py_str("kneecap"), py_str("jaw"), py_str("jaws"), py_str("jawbone"), py_str("jaw joint"), py_str("tmj"), py_str("car"), py_str("cars"), py_str("brake"), py_str("brakes"), py_str("engine"), py_str("clutch"), py_str("gearbox"), py_str("exhaust"), py_str("bike"), py_str("bicycle"), py_str("boiler"), py_str("radiator"), py_str("pipe"), py_str("pipes"), py_str("tap"), py_str("taps"), py_str("washing machine"), py_str("dishwasher"), py_str("tumble"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("toaster"), py_str("kettle"), py_str("door"), py_str("doors"), py_str("gate"), py_str("window"), py_str("windows"), py_str("hinge"), py_str("floor"), py_str("floors"), py_str("floorboard"), py_str("floorboards"), py_str("decking"), py_str("stair"), py_str("stairs"), py_str("wall"), py_str("walls"), py_str("wood"), py_str("wooden"), py_str("tile"), py_str("tiles"), py_str("pavement"), py_str("road"), py_str("phone"), py_str("iphone"), py_str("laptop"), py_str("computer"), py_str("mouse"), py_str("keyboard"), py_str("headphones"), py_str("speaker"), py_str("speakers"), py_str("radio"), py_str("tv"), py_str("television"), py_str("printer"), py_str("camera"), py_str("app"), py_str("video"), py_str("audio"), py_str("volume"), py_str("hearing aid"), py_str("playstation"), py_str("xbox"), py_str("ear"), py_str("ears"), py_str("throat"), py_str("swallow"), py_str("swallowing"), py_str("hearing"), py_str("tinnitus"), py_str("chest"), py_str("heart"), py_str("breath"), py_str("breathing"), py_str("tooth"), py_str("teeth"), py_str("egg"), py_str("eggs"), py_str("shell"), py_str("boil"), py_str("boiling"), py_str("boiled"), py_str("mug"), py_str("cup"), py_str("glass"), py_str("screen"), py_str("voice"), py_str("whip"), py_str("meringue"), py_str("batter"), py_str("pastry"), py_str("dough"), py_str("bread"), py_str("cake"), py_str("sprain"), py_str("sprained"), py_str("sprains"), py_str("strain"), py_str("strained"), py_str("strains"), py_str("injury"), py_str("injuries"), py_str("injured"), py_str("hurt myself"), py_str("fall"), py_str("falls"), py_str("fell"), py_str("fallen"), py_str("accident"), py_str("surgery"), py_str("operation"), py_str("op"), py_str("operated"), py_str("replaced"), py_str("replacement"), py_str("physio"), py_str("physiotherapy"), py_str("cast"), py_str("crutches"), py_str("brace"), py_str("break"), py_str("broke"), py_str("broken"), py_str("fracture"), py_str("fractured"), py_str("dislocated"), py_str("dislocate"), py_str("torn"), py_str("tear"), py_str("tore"), py_str("arthroscopy"), py_str("keyhole"), py_str("gives way"), py_str("give way"), py_str("giving way"), py_str("gave way"), py_str("out of place"), py_str("popped out"), py_str("pops out"), py_str("numb"), py_str("numbness"), py_str("numbed"), py_str("tingling"), py_str("tingles"), py_str("tingle"), py_str("pins and needles"), py_str("weak"), py_str("weakness"), py_str("unstable"), py_str("catching"), py_str("catches"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my kids"), py_str("my baby"), py_str("my toddler"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my mom"), py_str("my mother"), py_str("my dad"), py_str("my father"), py_str("my dog"), py_str("my cat"), py_str("my puppy")]));
    s_SHOULDER_TIMELINE_CARD(py_str("A shoulder replacement is in a sling for the first 3 to 6 weeks, with gentle movement exercises started within days and the physio's programme once the sling comes off: driving again at about 6 to 8 weeks once you can turn the wheel and brake without pain, desk work at 6 to 12 weeks, and full recovery 6 to 12 months. Hospital is usually one or two nights. Ring the ward or 111 if the wound starts leaking, the shoulder turns hot or red, or the pain suddenly gets worse."));
    s_SHOULDER_IMPLANT_CARD(py_str("A shoulder replacement usually lasts 10 to 15 years, and a reverse shoulder about the same - it is not a countdown: the muscle you keep working and the load you put through it decide, so the exercises the physio gives you are what buy the time. If a shoulder that has been fine for years starts to ache, grind or give way, that is worth an X-ray rather than waiting, and a worn shoulder can be revised the way a hip can."));
    s_SHOULDER_SORE_CARD(py_str("Yes - soreness and stiffness for months after a shoulder replacement is normal, and it eases with the exercises rather than with rest: it is at its stiffest in the first six weeks and keeps loosening for up to a year, and a warm, puffy shoulder after a session is the usual reaction to doing more. Build up slowly, ice it for 15 minutes after the exercises, and keep the daily ones going even on a bad day. Ring the ward or 111 if the shoulder is hot and red, the wound leaks, or the pain gets worse each week rather than better."));
    s_SHOULDER_SLEEPING_CARD(py_str("Sleep is the awkward part for the first weeks after a shoulder replacement: sleep propped up on two or three pillows with the arm supported on one in front of you or on a pillow beside you, and not on the operated side. A cushion tucked under the elbow takes the pull off the shoulder, and most people find a recliner or the corner of a sofa easier than a bed at first. A bad night is normal rather than a sign of damage."));
    s_NEW_JOINT_NOISE(py_tuple(vec![py_str("click"), py_str("clicks"), py_str("clicked"), py_str("clicking"), py_str("clunk"), py_str("clunks"), py_str("clunking"), py_str("pop"), py_str("pops"), py_str("popped"), py_str("popping"), py_str("grind"), py_str("grinds"), py_str("grinding"), py_str("creak"), py_str("creaks"), py_str("creaking"), py_str("catch"), py_str("catches"), py_str("catching"), py_str("noisy"), py_str("noise")]));
    s_NEW_JOINT_CARD(py_str("Clicking, clunking or a catching feeling around a new joint is usually the soft tissue moving over the new parts, and it is common while the muscles are still settling - keep up the exercises the team gave you and mention it at your next check. Ring the ward that did the operation, or 111, the same day if the joint is hot, red or more painful than it was, if you have a fever or the wound is weeping, or if the joint gives way or you cannot put your weight on it."));
    s_ROAD_WORDS(py_tuple(vec![py_str("road"), py_str("roads"), py_str("street"), py_str("streets"), py_str("kerb"), py_str("kerbs"), py_str("pavement"), py_str("pavements"), py_str("driveway"), py_str("driveways"), py_str("tarmac"), py_str("junction"), py_str("junctions"), py_str("carriageway"), py_str("footpath"), py_str("footpaths"), py_str("pothole"), py_str("potholes")]));
    s_A_REPLACING_A_THING(stdlib_call("re", "compile", &[py_str("\\breplac(?:e|ing|ed)\\s+(?:(?:a|an|the|my|our|your|his|her|their)\\s+)?(?P<thing>[a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_JOINT_WORDS(py_tuple(vec![py_str("hip"), py_str("knee"), py_str("shoulder"), py_str("joint"), py_str("joints"), py_str("resurfacing"), py_str("implant"), py_str("implanted"), py_str("replacement"), py_str("dressing"), py_str("dressings"), py_str("bandage"), py_str("bandages"), py_str("cast"), py_str("plaster")]));
    s_REPLACED_JOINT(py_tuple(vec![py_str("replacement"), py_str("replaced"), py_str("new hip"), py_str("new knee"), py_str("new joint"), py_str("new shoulder"), py_str("resurfacing"), py_str("resurfaced"), py_str("hip replacement"), py_str("knee replacement"), py_str("shoulder replacement"), py_str("partial knee"), py_str("total knee"), py_str("total hip"), py_str("had my hip done"), py_str("had my knee done"), py_str("reverse shoulder")]));
    s_SHOULDER(py_tuple(vec![py_str("shoulder"), py_str("shoulders"), py_str("new shoulder"), py_str("shoulder replacement"), py_str("reverse shoulder"), py_str("reverse shoulder replacement"), py_str("resurfaced shoulder")]));
    s_HIP_KNEE(py_tuple(vec![py_str("replacement"), py_str("replaced"), py_str("new hip"), py_str("new knee"), py_str("new joint"), py_str("resurfacing"), py_str("resurfaced"), py_str("hip replacement"), py_str("knee replacement"), py_str("partial knee"), py_str("total knee"), py_str("total hip"), py_str("had my hip done"), py_str("had my knee done")]));
    s_TIMELINE(py_tuple(vec![py_str("how long"), py_str("heal"), py_str("heals"), py_str("healing"), py_str("recover"), py_str("recovery"), py_str("recovering"), py_str("drive"), py_str("driving"), py_str("driven"), py_str("back to work"), py_str("go back to work"), py_str("return to work"), py_str("swim"), py_str("swimming"), py_str("hospital"), py_str("nights in"), py_str("how many days"), py_str("how many weeks"), py_str("when can i"), py_str("how soon"), py_str("how long before"), py_str("what can i do"), py_str("weeks"), py_str("months")]));
    s_TIMELINE_CARD(py_str("Walking on it the day after the operation, off the sticks at about 6 weeks, driving again at 6 weeks once you can do an emergency stop without thinking, and back to desk work at 6 to 12 weeks: hospital is usually 2 to 4 nights for a hip and 1 to 3 for a knee, and full recovery is 6 to 12 months for a hip and up to a year for a knee. Kneeling after a knee replacement is uncomfortable for most people and may never come back fully. Ring the ward or 111 if the wound starts leaking, the joint turns hot or red, or the pain suddenly gets worse."));
    s_SORE(py_tuple(vec![py_str("stiff"), py_str("stiffness"), py_str("stiffen"), py_str("sore"), py_str("soreness"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful"), py_str("swollen"), py_str("swelling"), py_str("normal"), py_str("expected"), py_str("should it"), py_str("is it")]));
    s_SORE_CARD(py_str("Yes - soreness and stiffness for months after a joint replacement is normal, and it eases with the exercises rather than with rest: the joint is at its stiffest in the first six weeks and keeps loosening for up to a year, and a warm, puffy knee or hip after exercise is the usual reaction to doing more. Build up slowly, ice it for 15 minutes after the exercises, and keep the daily ones going even on a bad day. Ordinary swelling is around the joint; a calf or a whole leg that swells, goes hot or is tender to touch is not ordinary swelling - ring the ward or 111 the same day, because a clot is the thing to rule out. Ring about the joint too if it is hot and red, the wound leaks, or the pain gets worse each week rather than better."));
    s_KNEEL(py_tuple(vec![py_str("kneel"), py_str("kneeling"), py_str("kneel down"), py_str("on my knees"), py_str("get down on the floor"), py_str("get up off the floor")]));
    s_KNEEL_CARD(py_str("Kneeling is the one thing many people never get back after a knee replacement: the new knee can take it, but the skin over it is numb and sore against a hard floor, so most people kneel on a cushion for a few minutes at a time rather than rule it out - try it on something soft once the wound is fully healed and the swelling has gone down. After a hip replacement you can kneel, but get down on the operated knee first and keep the new hip from bending past 90 degrees for the first few months. If kneeling is for gardening or a job, ask the physio to practise it with you rather than finding out on the day."));
    s_IMPLANT_LIFE(py_tuple(vec![py_str("how long does it last"), py_str("how long will it last"), py_str("how long do they last"), py_str("last"), py_str("lasts"), py_str("lifetime"), py_str("how many years"), py_str("wear out"), py_str("worn out"), py_str("wears out"), py_str("need another"), py_str("second replacement"), py_str("revision"), py_str("20 years"), py_str("second hip replacement"), py_str("second knee replacement"), py_str("another hip replacement"), py_str("another knee replacement"), py_str("needs doing again"), py_str("needs redone"), py_str("has worn out"), py_str("gone wrong after years")]));
    s_IMPLANT_CARD(py_str("A hip or knee replacement usually lasts 15 to 25 years, and about 8 in 10 are still going at 20 years; when one does wear out, a second operation to replace the worn parts is a routine job - smaller than the first, with a similar recovery. It is not a countdown: weight, activity and the wear on it decide, so the muscles kept strong and a healthy weight are what buy the time. If a joint that has been fine for years starts to ache, grind or give way, that is worth an X-ray rather than waiting."));
    s_WASH(py_tuple(vec![py_str("shower"), py_str("showers"), py_str("bath"), py_str("baths"), py_str("washing"), py_str("wash"), py_str("get it wet"), py_str("get it wet"), py_str("wound"), py_str("wounds"), py_str("dressing"), py_str("dressings"), py_str("stitches"), py_str("staples"), py_str("scar"), py_str("scars"), py_str("waterproof"), py_str("sealed"), py_str("healed over")]));
    s_WASH_CARD(py_str("A shower is usually fine once the dressing is off and the wound is dry and sealed, and most wards send people home with a waterproof dressing so it can happen sooner - but a bath, a hot tub or a swim waits until the wound is fully closed, around two to six weeks, because standing water is where infection gets in. Pat the wound dry rather than rubbing it, leave it open to the air when you can, and put nothing on it - no creams, oils or powders. Ring the ward or 111 if the wound turns hot or red, leaks or smells, or the edges start to open."));
    s_STAIRS(py_tuple(vec![py_str("stairs"), py_str("stair"), py_str("upstairs"), py_str("downstairs"), py_str("steps"), py_str("flight of stairs")]));
    s_STAIRS_CARD(py_str("Stairs are fine from the day you get home, one stair at a time with a rail and the crutches in the other hand: the good leg goes up first and the operated leg comes down first - up with the good, down with the bad - and both feet land on each step rather than one leg per stair. Sit down on the bottom step to put the sticks and slippers on rather than bending. If you live alone and the stairs are steep, ask the ward to watch you do a flight before you go home; it is a normal part of the discharge check."));
    s_SLEEPING(py_tuple(vec![py_str("sleep"), py_str("sleeping"), py_str("slept"), py_str("lie on"), py_str("lying on"), py_str("lie down"), py_str("on my side"), py_str("side to sleep"), py_str("in bed")]));
    s_SLEEPING_CARD(py_str("Sleep is the awkward part for the first few weeks, and the answer depends on the joint: after a hip, sleep on your back with a pillow between the knees, or on the unoperated side with the pillow still between the knees so the new hip cannot cross the middle - keep that up for the six weeks the ward tells you. After a knee, sleep on your back with the leg straight on a pillow, or on the good side with a pillow under the new knee. A pillow under the heels stops the sheet rubbing, and a bad night is normal rather than a sign of damage."));
    s_SCAR(py_tuple(vec![py_str("scar"), py_str("scars"), py_str("numb"), py_str("numbness"), py_str("no feeling"), py_str("lost feeling"), py_str("pins and needles"), py_str("tingling"), py_str("tingles"), py_str("itchy"), py_str("itches"), py_str("itching"), py_str("dead patch")]));
    s_SCAR_CARD(py_str("A numb patch of skin around the scar is normal after a joint replacement and it often stays that way: the small nerves under the incision are cut on the way in, so the skin there feels wooden, tingly or dead to touch, and it usually softens over a year or two rather than meaning anything about the joint. An itchy scar is the same story - healing nerves - and so is a scar that pulls or feels tight. A scar that turns hot, red, weepy or spreads, or numbness that comes on suddenly or spreads up the leg, is the one to ring the ward or 111 about, the same day."));
    s_LOOSE(py_tuple(vec![py_str("feels loose"), py_str("feel loose"), py_str("loose"), py_str("wobble"), py_str("wobbles"), py_str("wobbly"), py_str("gives way"), py_str("give way"), py_str("giving way"), py_str("gave way"), py_str("gives out"), py_str("unstable"), py_str("not stable"), py_str("pops out")]));
    s_SWOLLEN_LEG(py_tuple(vec![py_str("leg is swollen"), py_str("leg swollen"), py_str("leg has swollen"), py_str("swollen leg"), py_str("leg is puffy"), py_str("puffy leg"), py_str("leg has puffed"), py_str("calf is swollen"), py_str("swollen calf"), py_str("calf hurts"), py_str("calf is sore"), py_str("calf is tender"), py_str("my calf"), py_str("leg went big"), py_str("leg swelled up"), py_str("leg is tight")]));
    s_SWOLLEN_LEG_CARD(py_str("A leg that swells after a hip or knee replacement is not ordinary swelling and it is worth a phone call rather than a wait: a calf that is hot, tight, red or tender, or a whole leg that swells up, can be a clot, and clots are more common in the weeks after this operation than people expect - ring the ward that did the operation or 111 and say when the swelling started. If you are also short of breath, coughing or feeling faint, that is 999. Swelling around the joint itself is the ordinary sort, and that one goes down with ice and the leg raised."));
    s_LOOSE_CARD(py_str("A new joint that feels loose, gives way or clunks hard when you put weight on it is one to have looked at rather than wait for the next appointment: ring the ward or your surgeon's team and tell them when it started and what it does. Sometimes it is weak muscle around a joint that is fine - the physio can tell you that in one visit - and sometimes the parts need checking, which is why it is a phone call rather than a wait. If it gives way and you fall, or the joint is hot and red with it, treat that as urgent: 111 the same day."));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("noisy-joint")); __args.push(py_tuple(vec![g_NOISY(), g_PARTS()])); __args.push(g__noisy_joint()); __kws.extend(vec![("avoid".to_string(), add(&g_AVOID(), &g_HURTS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("replaced-joint-noise")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_NEW_JOINT_NOISE()])); __args.push(g_NEW_JOINT_CARD()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("hot"), py_str("red"), py_str("fever"), py_str("temperature"), py_str("wound"), py_str("weeping"), py_str("oozing"), py_str("pus"), py_str("infection"), py_str("infected"), py_str("dislocated"), py_str("dislocate"), py_str("popped out"), py_str("out of place"), py_str("fell"), py_str("fallen"), py_str("fall"), py_str("broke"), py_str("broken"), py_str("fracture"), py_str("torn"), py_str("tore"), py_str("numb"), py_str("numbness"), py_str("tingling"), py_str("pins and needles"), py_str("dog"), py_str("cat"), py_str("puppy")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-implant-life")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_IMPLANT_LIFE()])); __args.push(g__implant_life()); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-kneeling")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_KNEEL()])); __args.push(g_KNEEL_CARD()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("pray"), py_str("praying"), py_str("church"), py_str("garden"), py_str("weed"), py_str("weeds"), py_str("weeding")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-timeline")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_TIMELINE()])); __args.push(g__timeline()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("cost"), py_str("costs"), py_str("private"), py_str("how much does"), py_str("how much is"), py_str("nhs waiting"), py_str("waiting list"), py_str("dislocated"), py_str("dislocate"), py_str("popped out"), py_str("out of place"), py_str("broke"), py_str("broken"), py_str("fracture"), py_str("fractured"), py_str("infected"), py_str("infection"), py_str("hot and red"), py_str("red and hot"), py_str("pus"), py_str("oozing"), py_str("kneel"), py_str("kneeling"), py_str("shower"), py_str("bath"), py_str("wound"), py_str("dressing"), py_str("stairs"), py_str("upstairs"), py_str("downstairs"), py_str("sleep"), py_str("sleeping"), py_str("side"), py_str("licence"), py_str("license"), py_str("passport"), py_str("photocard"), py_str("document"), py_str("documents"), py_str("battery"), py_str("batteries"), py_str("phone"), py_str("laptop"), py_str("screen"), py_str("remote"), py_str("bulb"), py_str("bulbs"), py_str("fuse"), py_str("fuses"), py_str("filter"), py_str("filters"), py_str("part"), py_str("parts"), py_str("spare"), py_str("tyre"), py_str("tyres"), py_str("tire"), py_str("tires"), py_str("wheel"), py_str("wheels"), py_str("boiler"), py_str("radiator"), py_str("washer"), py_str("washing machine"), py_str("dishwasher"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("kettle"), py_str("toaster"), py_str("door"), py_str("lock"), py_str("handle"), py_str("zip"), py_str("zipper"), py_str("button"), py_str("buttons"), py_str("strap"), py_str("strap on"), py_str("tile"), py_str("tiles"), py_str("roof"), py_str("fence"), py_str("key"), py_str("keys"), py_str("card"), py_str("cards")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-washing")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_WASH()])); __args.push(g_WASH_CARD()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("hot and red"), py_str("red and hot"), py_str("oozing"), py_str("pus"), py_str("smelly"), py_str("infected"), py_str("infection"), py_str("fever"), py_str("temperature"), py_str("numb"), py_str("numbness"), py_str("no feeling"), py_str("pins and needles"), py_str("tingling"), py_str("itches"), py_str("itching"), py_str("itchy"), py_str("sore"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-scar")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_SCAR()])); __args.push(g_SCAR_CARD()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("hot and red"), py_str("red and hot"), py_str("oozing"), py_str("pus"), py_str("smelly"), py_str("infected"), py_str("infection"), py_str("fever"), py_str("temperature"), py_str("weeping")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-stairs")); __args.push(py_tuple(vec![g_HIP_KNEE(), g_STAIRS()])); __args.push(g_STAIRS_CARD()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("click"), py_str("clicks"), py_str("clicking"), py_str("clunk"), py_str("clunks"), py_str("clunking"), py_str("grind"), py_str("grinds"), py_str("grinding"), py_str("creak"), py_str("creaks"), py_str("catch"), py_str("catches"), py_str("catch"), py_str("pop"), py_str("pops"), py_str("loose"), py_str("wobble"), py_str("wobbly"), py_str("gives way"), py_str("giving way"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful"), py_str("sore"), py_str("swollen"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("shoulder"), py_str("shoulders"), py_str("arm"), py_str("elbow"), py_str("wrist"), py_str("hand")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-sleeping")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_SLEEPING()])); __args.push(g__sleeping()); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-loose")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_LOOSE()])); __args.push(g_LOOSE_CARD()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("hot and red"), py_str("red and hot"), py_str("dislocated"), py_str("popped out"), py_str("broke"), py_str("broken"), py_str("fracture")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-swollen-leg")); __args.push(py_tuple(vec![g_HIP_KNEE(), g_SWOLLEN_LEG()])); __args.push(g_SWOLLEN_LEG_CARD()); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-replacement-soreness")); __args.push(py_tuple(vec![g_REPLACED_JOINT(), g_SORE()])); __args.push(g__soreness()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("dislocated"), py_str("dislocate"), py_str("popped out"), py_str("out of place"), py_str("broke"), py_str("broken"), py_str("fracture"), py_str("infected"), py_str("infection"), py_str("hot and red"), py_str("red and hot"), py_str("pus"), py_str("oozing"), py_str("fever"), py_str("temperature"), py_str("worth replacing"), py_str("worth repairing"), py_str("worth fixing"), py_str("repair or replace"), py_str("fix or replace"), py_str("replace it"), py_str("replace the"), py_str("replacing it")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_replacement(args))(&__args) }]));
    s_LOCKED(py_tuple(vec![py_str("locks"), py_str("locked"), py_str("locking"), py_str("lock up"), py_str("locks up"), py_str("locked up"), py_str("locking up"), py_str("catches"), py_str("catching"), py_str("caught on"), py_str("gets stuck"), py_str("get stuck"), py_str("got stuck"), py_str("will not straighten"), py_str("wont straighten"), py_str("cannot straighten"), py_str("cant straighten"), py_str("will not bend"), py_str("wont bend"), py_str("cannot bend"), py_str("cant bend"), py_str("jams"), py_str("jammed"), py_str("jamming"), py_str("in a locked")]));
    s_LOCK_PARTS(add(&g_PARTS(), &py_tuple(vec![py_str("knee"), py_str("knees"), py_str("kneecap"), py_str("leg"), py_str("legs"), py_str("thumb"), py_str("hand"), py_str("hands")])));
    s_TRIGGER_FINGER_CARD(py_str("A finger that locks or clicks and then catches as it straightens is trigger finger: the tendon that bends the finger runs through a sheath, the sheath swells, and the tendon has to jump through the tight spot - which is why it locks bent and then lets go with a click. Rest it from hard gripping for a few weeks, keep it straight at night with a finger splint from a chemist, and take ibuprofen if it is sore. See your GP if it stays locked, keeps locking during the day, or a second finger starts: a steroid injection into the sheath usually sorts it, and a small operation does if it does not."));
    s_LOCKED_CARD(py_str("A {part} that locks, catches or will not straighten has something physically in the way, so it is worth being examined this week rather than waiting for it to settle: ring your GP and say which movement brings it on, whether it lets go on its own, and whether there was a twist or a knock before it started. A knee that locks is usually a torn cartilage or a loose piece of it catching in the joint, which a physio or doctor can tell apart by hand in one appointment, so keep off twisting sports and deep squats until then rather than testing it. Go to 111 the same day if the joint swells up, is hot, or you cannot put weight on it at all."));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("locked-joint")); __args.push(py_tuple(vec![g_LOCKED(), g_LOCK_PARTS()])); __args.push(g__locked_joint()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how do i lock"), py_str("lock out"), py_str("lock my knee"), py_str("lock your knee"), py_str("locking out"), py_str("lock the knee"), py_str("jaw"), py_str("jaws"), py_str("jaw joint"), py_str("tmj"), py_str("dog"), py_str("cat"), py_str("door"), py_str("doors"), py_str("gate"), py_str("car"), py_str("cars"), py_str("bike"), py_str("bicycle"), py_str("phone"), py_str("laptop"), py_str("computer"), py_str("screen"), py_str("window"), py_str("windows"), py_str("cupboard"), py_str("cupboards"), py_str("drawer"), py_str("drawers")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
}
pub fn _timeline(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("The healing schedule, spoken about the joint the line named.");
    if py_truthy(&Py::Bool(py_eq(&_joint(&[understood.clone()]), &py_str("shoulder")))) {
    return g_SHOULDER_TIMELINE_CARD();
    }
    return g_TIMELINE_CARD();
    Py::None
}
pub fn _implant_life(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    if py_truthy(&Py::Bool(py_eq(&_joint(&[understood.clone()]), &py_str("shoulder")))) {
    return g_SHOULDER_IMPLANT_CARD();
    }
    return g_IMPLANT_CARD();
    Py::None
}
pub fn _soreness(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    if py_truthy(&Py::Bool(py_eq(&_joint(&[understood.clone()]), &py_str("shoulder")))) {
    return g_SHOULDER_SORE_CARD();
    }
    return g_SORE_CARD();
    Py::None
}
pub fn _sleeping(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    if py_truthy(&Py::Bool(py_eq(&_joint(&[understood.clone()]), &py_str("shoulder")))) {
    return g_SHOULDER_SLEEPING_CARD();
    }
    return g_SLEEPING_CARD();
    Py::None
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every word of the line, and of the turn it leans on, as a set.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("words"), py_str("background")]));
    for __item in __iter1 {
    key = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "add", &[mcall(&word.clone(), "lower", &[])]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _a(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("The article for the word: \"an ankle\", \"a knuckle\". The sentence is spoken about the\n    part the person named, so it has to read like a sentence for every one of them.");
    return add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an ") } else { py_str("a ") }, &word.clone());
    Py::None
}
pub fn _joint(args: &[Py]) -> Py {
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
    let _ = py_str("\"shoulder\" if the line named one, otherwise \"replacement\" for a hip or a knee.\n\n    The numbers in the cards below are a hip's and a knee's - sticks, driving an emergency stop,\n    kneeling, 15 to 25 years - and a shoulder shares none of them. Rather than fence the shoulder\n    out of the row entirely (which would send the person back to a shrug), the card speaks about\n    the joint they had.\n    ");
    words = _words(&[understood.clone()]);
    return if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SHOULDER()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { py_str("shoulder") } else { py_str("replacement") };
    Py::None
}
pub fn _not_a_road(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line is about a road that names no joint - then a roadworks line answers.");
    words = _words(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ROAD_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PARTS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("hip"), py_str("knee"), py_str("shoulder"), py_str("surgeon"), py_str("operation"), py_str("physio"), py_str("hospital"), py_str("nurse"), py_str("ward"), py_str("recovery"), py_str("physiotherapy")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _not_something_else_replaced(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut things: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True unless the line replaces a thing that is not a joint - a bath, a boiler, a road.");
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    things = mcall(&g_A_REPLACING_A_THING(), "findall", &[mcall(&said.clone(), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&things.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THE_JOINT_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_replacement(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut args: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut kwargs: Py = py_dict(__kw.iter().filter(|(n, _)| ![].contains(&n.as_str())).map(|(n, v)| (py_str(n), v.clone())).collect());
    let mut args: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut kwargs: Py = Py::None;
    let _ = py_str("One of the cards about a joint that was replaced, which a road line must not reach.");
    asked = mcall(&kwargs.clone(), "pop", &[py_str("guard"), Py::None]);
    if py_truthy(&Py::Bool(py_is(&asked.clone(), &Py::None))) {
    let _ = py_setitem(&kwargs.clone(), &py_str("guard"), py_func("lambda", move |args: &[Py]| { let args = args.clone(); let asked = asked.clone(); let kwargs = kwargs.clone(); let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = _not_a_road(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _not_something_else_replaced(&[understood.clone()]) } } }));
    } else {
    let _ = py_setitem(&kwargs.clone(), &py_str("guard"), py_func("lambda", move |args: &[Py]| { let args = args.clone(); let asked = asked.clone(); let kwargs = kwargs.clone(); let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = _not_a_road(&[understood.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _not_something_else_replaced(&[understood.clone()]); if !py_truthy(&__b) { __b } else { py_call(&asked, &[understood.clone()]) } } } } }));
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&args.clone())); for (k, v) in kw_pairs(&kwargs.clone()) { __kws.push((k, v)); } if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) };
    Py::None
}
pub fn _noisy_joint(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The noise answer, about the part the line named: \"A knuckle that clicks...\".\n\n    A line with a word for hurting never reaches here: it is in AVOID, so the body-part entry\n    answers it with what the pain probably is.\n    ");
    words = _words(&[understood.clone()]);
    part = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PARTS()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&words.clone(), &word.clone()))) {  __out.push(mcall(&g_ONE(), "get", &[word.clone(), word.clone()])); } } py_list(__out) })() }, py_str("joint")]).unwrap_or_else(|e| exc::raise(e));
    return add(&mcall(&_a(&[part.clone()]), "capitalize", &[]), &py_str(" that clicks or pops and does not hurt is almost always harmless: a tendon flicking over a bump, or gas coming out of the joint fluid, and it needs nothing doing. Keep the muscles around it working, because clicking that turns into pain is usually a strength problem rather than the noise. If it starts to hurt, swell or lock, have a physio or your GP look at it."));
    Py::None
}
pub fn _locked_joint(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The lock answer, about the part the line named, with the finger's own self-care.");
    words = _words(&[understood.clone()]);
    part = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LOCK_PARTS()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&words.clone(), &word.clone()))) {  __out.push(mcall(&g_ONE(), "get", &[word.clone(), word.clone()])); } } py_list(__out) })() }, py_str("joint")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("thumbs")]), &part.clone()))) {
    return g_TRIGGER_FINGER_CARD();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("part".to_string(), part.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LOCKED_CARD(), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.joints".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "AVOID" => Some(g_AVOID()),
            "A_REPLACING_A_THING" => Some(g_A_REPLACING_A_THING()),
            "ENTRIES" => Some(g_ENTRIES()),
            "HIP_KNEE" => Some(g_HIP_KNEE()),
            "HURTS" => Some(g_HURTS()),
            "IMPLANT_CARD" => Some(g_IMPLANT_CARD()),
            "IMPLANT_LIFE" => Some(g_IMPLANT_LIFE()),
            "KNEEL" => Some(g_KNEEL()),
            "KNEEL_CARD" => Some(g_KNEEL_CARD()),
            "LOCKED" => Some(g_LOCKED()),
            "LOCKED_CARD" => Some(g_LOCKED_CARD()),
            "LOCK_PARTS" => Some(g_LOCK_PARTS()),
            "LOOSE" => Some(g_LOOSE()),
            "LOOSE_CARD" => Some(g_LOOSE_CARD()),
            "NEW_JOINT_CARD" => Some(g_NEW_JOINT_CARD()),
            "NEW_JOINT_NOISE" => Some(g_NEW_JOINT_NOISE()),
            "NOISY" => Some(g_NOISY()),
            "ONE" => Some(g_ONE()),
            "PARTS" => Some(g_PARTS()),
            "REPLACED_JOINT" => Some(g_REPLACED_JOINT()),
            "ROAD_WORDS" => Some(g_ROAD_WORDS()),
            "SCAR" => Some(g_SCAR()),
            "SCAR_CARD" => Some(g_SCAR_CARD()),
            "SHOULDER" => Some(g_SHOULDER()),
            "SHOULDER_IMPLANT_CARD" => Some(g_SHOULDER_IMPLANT_CARD()),
            "SHOULDER_SLEEPING_CARD" => Some(g_SHOULDER_SLEEPING_CARD()),
            "SHOULDER_SORE_CARD" => Some(g_SHOULDER_SORE_CARD()),
            "SHOULDER_TIMELINE_CARD" => Some(g_SHOULDER_TIMELINE_CARD()),
            "SLEEPING" => Some(g_SLEEPING()),
            "SLEEPING_CARD" => Some(g_SLEEPING_CARD()),
            "SORE" => Some(g_SORE()),
            "SORE_CARD" => Some(g_SORE_CARD()),
            "STAIRS" => Some(g_STAIRS()),
            "STAIRS_CARD" => Some(g_STAIRS_CARD()),
            "SWOLLEN_LEG" => Some(g_SWOLLEN_LEG()),
            "SWOLLEN_LEG_CARD" => Some(g_SWOLLEN_LEG_CARD()),
            "THE_JOINT_WORDS" => Some(g_THE_JOINT_WORDS()),
            "TIMELINE" => Some(g_TIMELINE()),
            "TIMELINE_CARD" => Some(g_TIMELINE_CARD()),
            "TRIGGER_FINGER_CARD" => Some(g_TRIGGER_FINGER_CARD()),
            "WASH" => Some(g_WASH()),
            "WASH_CARD" => Some(g_WASH_CARD()),
            "_a" => Some(py_func("_a", |args: &[Py]| _a(args))),
            "_a_replacement" => Some(py_func("_a_replacement", |args: &[Py]| _a_replacement(args))),
            "_implant_life" => Some(py_func("_implant_life", |args: &[Py]| _implant_life(args))),
            "_joint" => Some(py_func("_joint", |args: &[Py]| _joint(args))),
            "_locked_joint" => Some(py_func("_locked_joint", |args: &[Py]| _locked_joint(args))),
            "_noisy_joint" => Some(py_func("_noisy_joint", |args: &[Py]| _noisy_joint(args))),
            "_not_a_road" => Some(py_func("_not_a_road", |args: &[Py]| _not_a_road(args))),
            "_not_something_else_replaced" => Some(py_func("_not_something_else_replaced", |args: &[Py]| _not_something_else_replaced(args))),
            "_sleeping" => Some(py_func("_sleeping", |args: &[Py]| _sleeping(args))),
            "_soreness" => Some(py_func("_soreness", |args: &[Py]| _soreness(args))),
            "_timeline" => Some(py_func("_timeline", |args: &[Py]| _timeline(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            _ => None,
        }),
        set: None,
    }))
}
