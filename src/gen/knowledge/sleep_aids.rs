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
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIGHT_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOISE_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SCREEN_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_COMPLAINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_DEPENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_LIGHTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_NOISES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_OUTSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_SCREENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_SLEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_SOUNDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_TELEVISIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_THROUGH_A_WALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLEEP_AID_WALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOUND_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TELLY_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WALL_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_COMPLAINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DEPENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_NOISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_OUTSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SCREEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SLEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SOMEONE_ELSES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SOUND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_TELLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_THROUGH_A_WALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_WALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g_LIGHT_REPLY() -> Py { ensure_init(); LIGHT_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIGHT_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = LIGHT_REPLY.write() { *w = v; } }
pub fn g_NOISE_REPLY() -> Py { ensure_init(); NOISE_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOISE_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = NOISE_REPLY.write() { *w = v; } }
pub fn g_SCREEN_REPLY() -> Py { ensure_init(); SCREEN_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SCREEN_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = SCREEN_REPLY.write() { *w = v; } }
pub fn g_SLEEP_AID_COMPLAINT() -> Py { ensure_init(); SLEEP_AID_COMPLAINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_COMPLAINT(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_COMPLAINT.write() { *w = v; } }
pub fn g_SLEEP_AID_DEPENDS() -> Py { ensure_init(); SLEEP_AID_DEPENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_DEPENDS(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_DEPENDS.write() { *w = v; } }
pub fn g_SLEEP_AID_LIGHTS() -> Py { ensure_init(); SLEEP_AID_LIGHTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_LIGHTS(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_LIGHTS.write() { *w = v; } }
pub fn g_SLEEP_AID_NOISES() -> Py { ensure_init(); SLEEP_AID_NOISES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_NOISES(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_NOISES.write() { *w = v; } }
pub fn g_SLEEP_AID_OUTSIDE() -> Py { ensure_init(); SLEEP_AID_OUTSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_OUTSIDE(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_OUTSIDE.write() { *w = v; } }
pub fn g_SLEEP_AID_SCREENS() -> Py { ensure_init(); SLEEP_AID_SCREENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_SCREENS(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_SCREENS.write() { *w = v; } }
pub fn g_SLEEP_AID_SLEEP() -> Py { ensure_init(); SLEEP_AID_SLEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_SLEEP(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_SLEEP.write() { *w = v; } }
pub fn g_SLEEP_AID_SOUNDS() -> Py { ensure_init(); SLEEP_AID_SOUNDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_SOUNDS(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_SOUNDS.write() { *w = v; } }
pub fn g_SLEEP_AID_TELEVISIONS() -> Py { ensure_init(); SLEEP_AID_TELEVISIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_TELEVISIONS(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_TELEVISIONS.write() { *w = v; } }
pub fn g_SLEEP_AID_THROUGH_A_WALL() -> Py { ensure_init(); SLEEP_AID_THROUGH_A_WALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_THROUGH_A_WALL(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_THROUGH_A_WALL.write() { *w = v; } }
pub fn g_SLEEP_AID_WALL() -> Py { ensure_init(); SLEEP_AID_WALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_WALL(v: Py) { ensure_init(); if let Ok(mut w) = SLEEP_AID_WALL.write() { *w = v; } }
pub fn g_SOUND_REPLY() -> Py { ensure_init(); SOUND_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOUND_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = SOUND_REPLY.write() { *w = v; } }
pub fn g_TELLY_REPLY() -> Py { ensure_init(); TELLY_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TELLY_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = TELLY_REPLY.write() { *w = v; } }
pub fn g_WALL_REPLY() -> Py { ensure_init(); WALL_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WALL_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = WALL_REPLY.write() { *w = v; } }
pub fn g__A_COMPLAINT() -> Py { ensure_init(); _A_COMPLAINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_COMPLAINT(v: Py) { ensure_init(); if let Ok(mut w) = _A_COMPLAINT.write() { *w = v; } }
pub fn g__A_DEPENDS() -> Py { ensure_init(); _A_DEPENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DEPENDS(v: Py) { ensure_init(); if let Ok(mut w) = _A_DEPENDS.write() { *w = v; } }
pub fn g__A_LIGHT() -> Py { ensure_init(); _A_LIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LIGHT(v: Py) { ensure_init(); if let Ok(mut w) = _A_LIGHT.write() { *w = v; } }
pub fn g__A_NOISE() -> Py { ensure_init(); _A_NOISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_NOISE(v: Py) { ensure_init(); if let Ok(mut w) = _A_NOISE.write() { *w = v; } }
pub fn g__A_OUTSIDE() -> Py { ensure_init(); _A_OUTSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_OUTSIDE(v: Py) { ensure_init(); if let Ok(mut w) = _A_OUTSIDE.write() { *w = v; } }
pub fn g__A_SCREEN() -> Py { ensure_init(); _A_SCREEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SCREEN(v: Py) { ensure_init(); if let Ok(mut w) = _A_SCREEN.write() { *w = v; } }
pub fn g__A_SLEEP() -> Py { ensure_init(); _A_SLEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SLEEP(v: Py) { ensure_init(); if let Ok(mut w) = _A_SLEEP.write() { *w = v; } }
pub fn g__A_SOMEONE_ELSES() -> Py { ensure_init(); _A_SOMEONE_ELSES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SOMEONE_ELSES(v: Py) { ensure_init(); if let Ok(mut w) = _A_SOMEONE_ELSES.write() { *w = v; } }
pub fn g__A_SOUND() -> Py { ensure_init(); _A_SOUND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SOUND(v: Py) { ensure_init(); if let Ok(mut w) = _A_SOUND.write() { *w = v; } }
pub fn g__A_TELLY() -> Py { ensure_init(); _A_TELLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_TELLY(v: Py) { ensure_init(); if let Ok(mut w) = _A_TELLY.write() { *w = v; } }
pub fn g__A_THROUGH_A_WALL() -> Py { ensure_init(); _A_THROUGH_A_WALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_THROUGH_A_WALL(v: Py) { ensure_init(); if let Ok(mut w) = _A_THROUGH_A_WALL.write() { *w = v; } }
pub fn g__A_WALL() -> Py { ensure_init(); _A_WALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_WALL(v: Py) { ensure_init(); if let Ok(mut w) = _A_WALL.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A sleep that depends on a thing in the room is a question about that thing.\n\nPeople say the aid and the sleep in one line: \"i cant fall asleep without the telly on\",\n\"i need the fan on to get to sleep\", \"i cant sleep, the light is too bright\". The chat\nmodule read the first of those as \"i can't sleep\" and offered the bad-week sympathy pool,\nand the answers module had no row that named the telly, the fan or the lamp. This domain\nholds the rows, keyed by the aid, the sleep, the dependence and the complaint.\n\nA line that asks about the habit is not the same question: \"is it bad to sleep with the tv\non\" keeps the older `tv-sleep` card, and \"how do i get to sleep without my phone\" keeps the\nsleep-hygiene advice, because it asks for a way to stop rather than reporting a dependence.\n");
    s_ANSWERS(py_tuple(vec![py_str("health"), py_str("sleep")]));
    s_SLEEP_AID_SCREENS(py_tuple(vec![py_str("telly"), py_str("phone"), py_str("screen"), py_str("laptop"), py_str("tablet"), py_str("ipad"), py_str("computer"), py_str("monitor")]));
    s_SLEEP_AID_SOUNDS(py_tuple(vec![py_str("radio"), py_str("podcast"), py_str("music"), py_str("white noise"), py_str("fan"), py_str("audiobook")]));
    s_SLEEP_AID_LIGHTS(py_tuple(vec![py_str("light"), py_str("lights"), py_str("lamp"), py_str("lamps"), py_str("streetlight"), py_str("street light"), py_str("daylight"), py_str("bright")]));
    s_SLEEP_AID_TELEVISIONS(py_tuple(vec![py_str("tv"), py_str("television"), py_str("telly")]));
    s_SLEEP_AID_SLEEP(py_tuple(vec![py_str("sleep"), py_str("sleeping"), py_str("asleep"), py_str("fall asleep"), py_str("get to sleep"), py_str("go to sleep"), py_str("bed"), py_str("bedtime"), py_str("awake"), py_str("wake")]));
    s_SLEEP_AID_DEPENDS(py_tuple(vec![py_str("without"), py_str("with"), py_str("unless"), py_str("only"), py_str("need"), py_str("needs"), py_str("need it"), py_str("have to have"), py_str("got to have"), py_str("because"), py_str("keeps me"), py_str("helps me"), py_str("is on"), py_str("was on"), py_str("left on"), py_str("loud"), py_str("too bright"), py_str("too loud")]));
    s_SLEEP_AID_NOISES(py_tuple(vec![py_str("noise"), py_str("noisy")]));
    s_SLEEP_AID_OUTSIDE(py_tuple(vec![py_str("outside"), py_str("street"), py_str("road"), py_str("main road"), py_str("outside my window"), py_str("outside the window")]));
    s_SLEEP_AID_WALL(py_tuple(vec![py_str("next door"), py_str("next room"), py_str("other room"), py_str("another room"), py_str("upstairs"), py_str("downstairs")]));
    s_SLEEP_AID_THROUGH_A_WALL(py_tuple(vec![py_str("washing machine"), py_str("dishwasher"), py_str("tumble dryer"), py_str("hoover"), py_str("vacuum"), py_str("music"), py_str("stereo"), py_str("party"), py_str("drilling"), py_str("diy")]));
    s_SLEEP_AID_COMPLAINT(py_tuple(vec![py_str("cant"), py_str("can't"), py_str("cannot"), py_str("can not"), py_str("struggle"), py_str("struggling"), py_str("hard to"), py_str("unable"), py_str("need"), py_str("needs"), py_str("needed"), py_str("only"), py_str("have to"), py_str("got to"), py_str("keeps me awake"), py_str("waking me"), py_str("wake me"), py_str("better")]));
    s_SCREEN_REPLY(py_str("It isn't dangerous, but the light and the story keep your brain alert, so you fall asleep later and wake more in the night. If you need something playing, a podcast or radio set to a sleep timer is easier on sleep than a screen, and charging the phone across the room helps too."));
    s_TELLY_REPLY(py_str("It isn't dangerous, but the light and the story keep your brain alert, so you fall asleep later and wake more in the night. If you need something playing, a podcast or radio set to a sleep timer is easier on sleep than a screen."));
    s_SOUND_REPLY(py_str("A steady sound is easier on sleep than a screen, because there's nothing to watch. Keep the radio or podcast low and on a sleep timer; if it is a fan or white noise, point it away from your face. If the noise is a neighbour's, earplugs and a heavy curtain take the edge off."));
    s_NOISE_REPLY(py_str("If the noise is coming from outside your room, earplugs and a heavy curtain take the edge off, and a fan or white noise can mask it. If it is a neighbour and it happens often, your council's noise team can act on it; a short log of the times is what they ask for."));
    s_LIGHT_REPLY(py_str("Light tells your brain it is still daytime, so keep the room as dark as you can: a blackout curtain or an eye mask, and turn the lamp off rather than dimming it. If a streetlight gets in, a curtain that reaches the floor helps."));
    s_WALL_REPLY(py_str("Sound goes through a wall more than people expect: earplugs and a heavy curtain or a bookcase against the shared wall take the edge off, and a fan or white noise masks the rest. If it is the room next door, ask them once at a calm time to turn it down after a set hour; if it keeps happening, keep a short log of the times and your council's noise team can act on it."));
    s__A_SCREEN(_words(&[g_SLEEP_AID_SCREENS()]));
    s__A_SOUND(_words(&[g_SLEEP_AID_SOUNDS()]));
    s__A_LIGHT(_words(&[g_SLEEP_AID_LIGHTS()]));
    s__A_TELLY(_words(&[g_SLEEP_AID_TELEVISIONS()]));
    s__A_SLEEP(_words(&[g_SLEEP_AID_SLEEP()]));
    s__A_DEPENDS(_words(&[g_SLEEP_AID_DEPENDS()]));
    s__A_COMPLAINT(_words(&[g_SLEEP_AID_COMPLAINT()]));
    s__A_NOISE(_words(&[g_SLEEP_AID_NOISES()]));
    s__A_OUTSIDE(_words(&[g_SLEEP_AID_OUTSIDE()]));
    s__A_SOMEONE_ELSES(_words(&[py_tuple(vec![py_str("neighbour"), py_str("neighbor"), py_str("neighbours"), py_str("neighbors"), py_str("next door"), py_str("next room"), py_str("other room"), py_str("another room"), py_str("outside"), py_str("upstairs"), py_str("downstairs"), py_str("people"), py_str("someone"), py_str("somebody")])]));
    s__A_WALL(_words(&[g_SLEEP_AID_WALL()]));
    s__A_THROUGH_A_WALL(_words(&[g_SLEEP_AID_THROUGH_A_WALL()]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-a-screen")); __args.push(py_tuple(vec![g_SLEEP_AID_SCREENS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_SCREEN_REPLY()); __kws.extend(vec![("ask".to_string(), g_SLEEP_AID_COMPLAINT()), ("guard".to_string(), g__the_line_itself())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-the-telly")); __args.push(py_tuple(vec![g_SLEEP_AID_TELEVISIONS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_TELLY_REPLY()); __kws.extend(vec![("guard".to_string(), g__the_telly_line())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-a-sound")); __args.push(py_tuple(vec![g_SLEEP_AID_SOUNDS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_SOUND_REPLY()); __kws.extend(vec![("ask".to_string(), g_SLEEP_AID_COMPLAINT()), ("guard".to_string(), g__the_line_itself())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-a-light")); __args.push(py_tuple(vec![g_SLEEP_AID_LIGHTS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_LIGHT_REPLY()); __kws.extend(vec![("ask".to_string(), g_SLEEP_AID_COMPLAINT()), ("guard".to_string(), g__the_line_itself())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-broken-by-a-thing-next-door")); __args.push(py_tuple(vec![g_SLEEP_AID_WALL(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_COMPLAINT()])); __args.push(g_WALL_REPLY()); __kws.extend(vec![("guard".to_string(), g__the_wall_line())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-broken-by-a-noise")); __args.push(py_tuple(vec![g_SLEEP_AID_NOISES(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_COMPLAINT(), g_SLEEP_AID_OUTSIDE()])); __args.push(g_NOISE_REPLY()); __kws.extend(vec![("guard".to_string(), g__the_noise_line())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
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
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut name: Py = Py::None;
    return stdlib_call("re", "compile", &[modulo(&py_str("\\b(?:%s)\\b"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_sleep_that_needs_a_thing(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line names an aid, the sleep, the dependence and the complaint.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__A_SOMEONE_ELSES(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = { let __b = mcall(&g__A_SCREEN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SOUND(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_LIGHT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_TELLY(), "search", &[said.clone()]) } } } } } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SLEEP(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_DEPENDS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_COMPLAINT(), "search", &[said.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_sleep_broken_by_a_noise(args: &[Py]) -> Py {
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
    let _ = py_str("True for a sleep broken by a noise from somewhere else, when no aid is in the room.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g__A_SLEEP(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_COMPLAINT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_NOISE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_OUTSIDE(), "search", &[said.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_sleep_broken_by_a_thing_next_door(args: &[Py]) -> Py {
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
    let _ = py_str("True when an aid named with the sleep is on the other side of a wall, not in the room.\n\n    \"i cant sleep because the tv is on in the next room\" names the telly, the sleep and the\n    complaint, but the telly is not theirs to turn off: the answer is about sound through a wall,\n    not about the light and the story keeping their own brain alert.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = { let __b = mcall(&g__A_TELLY(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SCREEN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SOUND(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_THROUGH_A_WALL(), "search", &[said.clone()]) } } } } } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SLEEP(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_COMPLAINT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_WALL(), "search", &[said.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn which_aid(args: &[Py]) -> Py {
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
    let _ = py_str("The aid the sleep depends on, or None when the line is not that complaint.");
    if py_truthy(&Py::Bool(!py_truthy(&a_sleep_that_needs_a_thing(&[text.clone()])))) {
    return Py::None;
    }
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__A_TELLY(), "search", &[said.clone()])) {
    return py_str("telly");
    }
    if py_truthy(&mcall(&g__A_SCREEN(), "search", &[said.clone()])) {
    return py_str("screen");
    }
    if py_truthy(&mcall(&g__A_SOUND(), "search", &[said.clone()])) {
    return py_str("sound");
    }
    if py_truthy(&mcall(&g__A_LIGHT(), "search", &[said.clone()])) {
    return py_str("light");
    }
    return Py::None;
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut aid: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The card for a sleep that depends on a thing, or None for every other line.");
    reading = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]) } else { Py::None };
    said = py_str("");
    if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    said = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    }
    aid = which_aid(&[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { text.clone() } }]);
    if py_truthy(&Py::Bool(py_is(&aid.clone(), &Py::None))) {
    if py_truthy(&a_sleep_broken_by_a_thing_next_door(&[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { text.clone() } }])) {
    return g_WALL_REPLY();
    }
    if py_truthy(&a_sleep_broken_by_a_noise(&[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { text.clone() } }])) {
    return g_NOISE_REPLY();
    }
    return Py::None;
    }
    return geti(&py_dict(vec![(py_str("telly"), g_TELLY_REPLY()), (py_str("screen"), g_SCREEN_REPLY()), (py_str("sound"), g_SOUND_REPLY()), (py_str("light"), g_LIGHT_REPLY())]), &aid.clone());
    Py::None
}
pub fn _the_line_itself(args: &[Py]) -> Py {
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
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return a_sleep_that_needs_a_thing(&[said.clone()]);
    Py::None
}
pub fn _the_noise_line(args: &[Py]) -> Py {
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
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = a_sleep_broken_by_a_noise(&[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&which_aid(&[said.clone()]), &Py::None)) } };
    Py::None
}
pub fn _the_wall_line(args: &[Py]) -> Py {
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
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return a_sleep_broken_by_a_thing_next_door(&[said.clone()]);
    Py::None
}
pub fn _the_telly_line(args: &[Py]) -> Py {
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
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g__A_TELLY(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SLEEP(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_DEPENDS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_COMPLAINT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_SOMEONE_ELSES(), "search", &[said.clone()]))) } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.sleep_aids".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "LIGHT_REPLY" => Some(g_LIGHT_REPLY()),
            "NOISE_REPLY" => Some(g_NOISE_REPLY()),
            "SCREEN_REPLY" => Some(g_SCREEN_REPLY()),
            "SLEEP_AID_COMPLAINT" => Some(g_SLEEP_AID_COMPLAINT()),
            "SLEEP_AID_DEPENDS" => Some(g_SLEEP_AID_DEPENDS()),
            "SLEEP_AID_LIGHTS" => Some(g_SLEEP_AID_LIGHTS()),
            "SLEEP_AID_NOISES" => Some(g_SLEEP_AID_NOISES()),
            "SLEEP_AID_OUTSIDE" => Some(g_SLEEP_AID_OUTSIDE()),
            "SLEEP_AID_SCREENS" => Some(g_SLEEP_AID_SCREENS()),
            "SLEEP_AID_SLEEP" => Some(g_SLEEP_AID_SLEEP()),
            "SLEEP_AID_SOUNDS" => Some(g_SLEEP_AID_SOUNDS()),
            "SLEEP_AID_TELEVISIONS" => Some(g_SLEEP_AID_TELEVISIONS()),
            "SLEEP_AID_THROUGH_A_WALL" => Some(g_SLEEP_AID_THROUGH_A_WALL()),
            "SLEEP_AID_WALL" => Some(g_SLEEP_AID_WALL()),
            "SOUND_REPLY" => Some(g_SOUND_REPLY()),
            "TELLY_REPLY" => Some(g_TELLY_REPLY()),
            "WALL_REPLY" => Some(g_WALL_REPLY()),
            "_A_COMPLAINT" => Some(g__A_COMPLAINT()),
            "_A_DEPENDS" => Some(g__A_DEPENDS()),
            "_A_LIGHT" => Some(g__A_LIGHT()),
            "_A_NOISE" => Some(g__A_NOISE()),
            "_A_OUTSIDE" => Some(g__A_OUTSIDE()),
            "_A_SCREEN" => Some(g__A_SCREEN()),
            "_A_SLEEP" => Some(g__A_SLEEP()),
            "_A_SOMEONE_ELSES" => Some(g__A_SOMEONE_ELSES()),
            "_A_SOUND" => Some(g__A_SOUND()),
            "_A_TELLY" => Some(g__A_TELLY()),
            "_A_THROUGH_A_WALL" => Some(g__A_THROUGH_A_WALL()),
            "_A_WALL" => Some(g__A_WALL()),
            "_the_line_itself" => Some(py_func("_the_line_itself", |args: &[Py]| _the_line_itself(args))),
            "_the_noise_line" => Some(py_func("_the_noise_line", |args: &[Py]| _the_noise_line(args))),
            "_the_telly_line" => Some(py_func("_the_telly_line", |args: &[Py]| _the_telly_line(args))),
            "_the_wall_line" => Some(py_func("_the_wall_line", |args: &[Py]| _the_wall_line(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "a_sleep_broken_by_a_noise" => Some(py_func("a_sleep_broken_by_a_noise", |args: &[Py]| a_sleep_broken_by_a_noise(args))),
            "a_sleep_broken_by_a_thing_next_door" => Some(py_func("a_sleep_broken_by_a_thing_next_door", |args: &[Py]| a_sleep_broken_by_a_thing_next_door(args))),
            "a_sleep_that_needs_a_thing" => Some(py_func("a_sleep_that_needs_a_thing", |args: &[Py]| a_sleep_that_needs_a_thing(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "which_aid" => Some(py_func("which_aid", |args: &[Py]| which_aid(args))),
            _ => None,
        }),
        set: None,
    }))
}
