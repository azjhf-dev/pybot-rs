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
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIGHT_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOISE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCREEN_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_COMPLAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_DEPENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_LIGHTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_NOISES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_OUTSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_SCREENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_SLEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_SOUNDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_TELEVISIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_THROUGH_A_WALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_AID_WALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOUND_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELLY_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WALL_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COMPLAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DEPENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NOISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_OUTSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SCREEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SLEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SOMEONE_ELSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SOUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TELLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THROUGH_A_WALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_LIGHT_REPLY() -> Py { ensure_init(); G_LIGHT_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIGHT_REPLY(v: Py) { ensure_init(); G_LIGHT_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOISE_REPLY() -> Py { ensure_init(); G_NOISE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOISE_REPLY(v: Py) { ensure_init(); G_NOISE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_SCREEN_REPLY() -> Py { ensure_init(); G_SCREEN_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCREEN_REPLY(v: Py) { ensure_init(); G_SCREEN_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_COMPLAINT() -> Py { ensure_init(); G_SLEEP_AID_COMPLAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_COMPLAINT(v: Py) { ensure_init(); G_SLEEP_AID_COMPLAINT.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_DEPENDS() -> Py { ensure_init(); G_SLEEP_AID_DEPENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_DEPENDS(v: Py) { ensure_init(); G_SLEEP_AID_DEPENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_LIGHTS() -> Py { ensure_init(); G_SLEEP_AID_LIGHTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_LIGHTS(v: Py) { ensure_init(); G_SLEEP_AID_LIGHTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_NOISES() -> Py { ensure_init(); G_SLEEP_AID_NOISES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_NOISES(v: Py) { ensure_init(); G_SLEEP_AID_NOISES.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_OUTSIDE() -> Py { ensure_init(); G_SLEEP_AID_OUTSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_OUTSIDE(v: Py) { ensure_init(); G_SLEEP_AID_OUTSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_SCREENS() -> Py { ensure_init(); G_SLEEP_AID_SCREENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_SCREENS(v: Py) { ensure_init(); G_SLEEP_AID_SCREENS.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_SLEEP() -> Py { ensure_init(); G_SLEEP_AID_SLEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_SLEEP(v: Py) { ensure_init(); G_SLEEP_AID_SLEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_SOUNDS() -> Py { ensure_init(); G_SLEEP_AID_SOUNDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_SOUNDS(v: Py) { ensure_init(); G_SLEEP_AID_SOUNDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_TELEVISIONS() -> Py { ensure_init(); G_SLEEP_AID_TELEVISIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_TELEVISIONS(v: Py) { ensure_init(); G_SLEEP_AID_TELEVISIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_THROUGH_A_WALL() -> Py { ensure_init(); G_SLEEP_AID_THROUGH_A_WALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_THROUGH_A_WALL(v: Py) { ensure_init(); G_SLEEP_AID_THROUGH_A_WALL.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_AID_WALL() -> Py { ensure_init(); G_SLEEP_AID_WALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_AID_WALL(v: Py) { ensure_init(); G_SLEEP_AID_WALL.with(|c| *c.borrow_mut() = v); }
pub fn g_SOUND_REPLY() -> Py { ensure_init(); G_SOUND_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOUND_REPLY(v: Py) { ensure_init(); G_SOUND_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_TELLY_REPLY() -> Py { ensure_init(); G_TELLY_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELLY_REPLY(v: Py) { ensure_init(); G_TELLY_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_WALL_REPLY() -> Py { ensure_init(); G_WALL_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WALL_REPLY(v: Py) { ensure_init(); G_WALL_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COMPLAINT() -> Py { ensure_init(); G__A_COMPLAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COMPLAINT(v: Py) { ensure_init(); G__A_COMPLAINT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DEPENDS() -> Py { ensure_init(); G__A_DEPENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DEPENDS(v: Py) { ensure_init(); G__A_DEPENDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LIGHT() -> Py { ensure_init(); G__A_LIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LIGHT(v: Py) { ensure_init(); G__A_LIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NOISE() -> Py { ensure_init(); G__A_NOISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NOISE(v: Py) { ensure_init(); G__A_NOISE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_OUTSIDE() -> Py { ensure_init(); G__A_OUTSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_OUTSIDE(v: Py) { ensure_init(); G__A_OUTSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SCREEN() -> Py { ensure_init(); G__A_SCREEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SCREEN(v: Py) { ensure_init(); G__A_SCREEN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SLEEP() -> Py { ensure_init(); G__A_SLEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SLEEP(v: Py) { ensure_init(); G__A_SLEEP.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SOMEONE_ELSES() -> Py { ensure_init(); G__A_SOMEONE_ELSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SOMEONE_ELSES(v: Py) { ensure_init(); G__A_SOMEONE_ELSES.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SOUND() -> Py { ensure_init(); G__A_SOUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SOUND(v: Py) { ensure_init(); G__A_SOUND.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TELLY() -> Py { ensure_init(); G__A_TELLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TELLY(v: Py) { ensure_init(); G__A_TELLY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THROUGH_A_WALL() -> Py { ensure_init(); G__A_THROUGH_A_WALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THROUGH_A_WALL(v: Py) { ensure_init(); G__A_THROUGH_A_WALL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WALL() -> Py { ensure_init(); G__A_WALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WALL(v: Py) { ensure_init(); G__A_WALL.with(|c| *c.borrow_mut() = v); }
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
    s__A_SCREEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_SCREENS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_SOUND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_SOUNDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_LIGHT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_LIGHTS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_TELLY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_TELEVISIONS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_SLEEP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_SLEEP()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_DEPENDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_DEPENDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_COMPLAINT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_COMPLAINT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_NOISE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_NOISES()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_OUTSIDE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_OUTSIDE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_SOMEONE_ELSES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("neighbour"), py_str("neighbor"), py_str("neighbours"), py_str("neighbors"), py_str("next door"), py_str("next room"), py_str("other room"), py_str("another room"), py_str("outside"), py_str("upstairs"), py_str("downstairs"), py_str("people"), py_str("someone"), py_str("somebody")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_WALL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_WALL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s__A_THROUGH_A_WALL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLEEP_AID_THROUGH_A_WALL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-a-screen")); __args.push(py_tuple(vec![g_SLEEP_AID_SCREENS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_SCREEN_REPLY()); __kws.extend(vec![("ask".to_string(), g_SLEEP_AID_COMPLAINT()), ("guard".to_string(), g__the_line_itself())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-the-telly")); __args.push(py_tuple(vec![g_SLEEP_AID_TELEVISIONS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_TELLY_REPLY()); __kws.extend(vec![("guard".to_string(), g__the_telly_line())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-a-sound")); __args.push(py_tuple(vec![g_SLEEP_AID_SOUNDS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_SOUND_REPLY()); __kws.extend(vec![("ask".to_string(), g_SLEEP_AID_COMPLAINT()), ("guard".to_string(), g__the_line_itself())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-that-needs-a-light")); __args.push(py_tuple(vec![g_SLEEP_AID_LIGHTS(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_DEPENDS()])); __args.push(g_LIGHT_REPLY()); __kws.extend(vec![("ask".to_string(), g_SLEEP_AID_COMPLAINT()), ("guard".to_string(), g__the_line_itself())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-broken-by-a-thing-next-door")); __args.push(py_tuple(vec![g_SLEEP_AID_WALL(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_COMPLAINT()])); __args.push(g_WALL_REPLY()); __kws.extend(vec![("guard".to_string(), g__the_wall_line())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sleep-broken-by-a-noise")); __args.push(py_tuple(vec![g_SLEEP_AID_NOISES(), g_SLEEP_AID_SLEEP(), g_SLEEP_AID_COMPLAINT(), g_SLEEP_AID_OUTSIDE()])); __args.push(g_NOISE_REPLY()); __kws.extend(vec![("guard".to_string(), g__the_noise_line())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\b(?:%s)\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) })); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SOMEONE_ELSES(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SCREEN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SOUND(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_LIGHT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TELLY(), "search", args))(&__args) } } } } } } }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SLEEP(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DEPENDS(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_COMPLAINT(), "search", args))(&__args) } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SLEEP(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_COMPLAINT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_NOISE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_OUTSIDE(), "search", args))(&__args) } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TELLY(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SCREEN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SOUND(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_THROUGH_A_WALL(), "search", args))(&__args) } } } } } } }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SLEEP(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_COMPLAINT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WALL(), "search", args))(&__args) } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_sleep_that_needs_a_thing(args))(&__args) }))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TELLY(), "search", args))(&__args) }) {
    return py_str("telly");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SCREEN(), "search", args))(&__args) }) {
    return py_str("screen");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SOUND(), "search", args))(&__args) }) {
    return py_str("sound");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_LIGHT(), "search", args))(&__args) }) {
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
    reading = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) } } else { Py::None };
    said = py_str("");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    aid = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { text.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| which_aid(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&aid.clone(), &Py::None))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { text.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_sleep_broken_by_a_thing_next_door(args))(&__args) }) {
    return g_WALL_REPLY();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { text.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_sleep_broken_by_a_noise(args))(&__args) }) {
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_sleep_that_needs_a_thing(args))(&__args) };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_sleep_broken_by_a_noise(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| which_aid(args))(&__args) }, &Py::None)) } };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_sleep_broken_by_a_thing_next_door(args))(&__args) };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TELLY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SLEEP(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DEPENDS(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_COMPLAINT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SOMEONE_ELSES(), "search", args))(&__args) })) } } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "LIGHT_REPLY" => s_LIGHT_REPLY(v),
            "NOISE_REPLY" => s_NOISE_REPLY(v),
            "SCREEN_REPLY" => s_SCREEN_REPLY(v),
            "SLEEP_AID_COMPLAINT" => s_SLEEP_AID_COMPLAINT(v),
            "SLEEP_AID_DEPENDS" => s_SLEEP_AID_DEPENDS(v),
            "SLEEP_AID_LIGHTS" => s_SLEEP_AID_LIGHTS(v),
            "SLEEP_AID_NOISES" => s_SLEEP_AID_NOISES(v),
            "SLEEP_AID_OUTSIDE" => s_SLEEP_AID_OUTSIDE(v),
            "SLEEP_AID_SCREENS" => s_SLEEP_AID_SCREENS(v),
            "SLEEP_AID_SLEEP" => s_SLEEP_AID_SLEEP(v),
            "SLEEP_AID_SOUNDS" => s_SLEEP_AID_SOUNDS(v),
            "SLEEP_AID_TELEVISIONS" => s_SLEEP_AID_TELEVISIONS(v),
            "SLEEP_AID_THROUGH_A_WALL" => s_SLEEP_AID_THROUGH_A_WALL(v),
            "SLEEP_AID_WALL" => s_SLEEP_AID_WALL(v),
            "SOUND_REPLY" => s_SOUND_REPLY(v),
            "TELLY_REPLY" => s_TELLY_REPLY(v),
            "WALL_REPLY" => s_WALL_REPLY(v),
            "_A_COMPLAINT" => s__A_COMPLAINT(v),
            "_A_DEPENDS" => s__A_DEPENDS(v),
            "_A_LIGHT" => s__A_LIGHT(v),
            "_A_NOISE" => s__A_NOISE(v),
            "_A_OUTSIDE" => s__A_OUTSIDE(v),
            "_A_SCREEN" => s__A_SCREEN(v),
            "_A_SLEEP" => s__A_SLEEP(v),
            "_A_SOMEONE_ELSES" => s__A_SOMEONE_ELSES(v),
            "_A_SOUND" => s__A_SOUND(v),
            "_A_TELLY" => s__A_TELLY(v),
            "_A_THROUGH_A_WALL" => s__A_THROUGH_A_WALL(v),
            "_A_WALL" => s__A_WALL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.sleep_aids", module_obj);
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "a_sleep_that_needs_a_thing", py_func("a_sleep_that_needs_a_thing", |args: &[Py]| a_sleep_that_needs_a_thing(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "a_sleep_broken_by_a_noise", py_func("a_sleep_broken_by_a_noise", |args: &[Py]| a_sleep_broken_by_a_noise(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "a_sleep_broken_by_a_thing_next_door", py_func("a_sleep_broken_by_a_thing_next_door", |args: &[Py]| a_sleep_broken_by_a_thing_next_door(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "which_aid", py_func("which_aid", |args: &[Py]| which_aid(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "_the_line_itself", py_func("_the_line_itself", |args: &[Py]| _the_line_itself(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "_the_noise_line", py_func("_the_noise_line", |args: &[Py]| _the_noise_line(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "_the_wall_line", py_func("_the_wall_line", |args: &[Py]| _the_wall_line(args)));
    pyrt::xrefs::reg_fn("knowledge.sleep_aids", "_the_telly_line", py_func("_the_telly_line", |args: &[Py]| _the_telly_line(args)));
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "LIGHT_REPLY", g_LIGHT_REPLY, s_LIGHT_REPLY);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "NOISE_REPLY", g_NOISE_REPLY, s_NOISE_REPLY);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SCREEN_REPLY", g_SCREEN_REPLY, s_SCREEN_REPLY);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_COMPLAINT", g_SLEEP_AID_COMPLAINT, s_SLEEP_AID_COMPLAINT);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_DEPENDS", g_SLEEP_AID_DEPENDS, s_SLEEP_AID_DEPENDS);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_LIGHTS", g_SLEEP_AID_LIGHTS, s_SLEEP_AID_LIGHTS);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_NOISES", g_SLEEP_AID_NOISES, s_SLEEP_AID_NOISES);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_OUTSIDE", g_SLEEP_AID_OUTSIDE, s_SLEEP_AID_OUTSIDE);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_SCREENS", g_SLEEP_AID_SCREENS, s_SLEEP_AID_SCREENS);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_SLEEP", g_SLEEP_AID_SLEEP, s_SLEEP_AID_SLEEP);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_SOUNDS", g_SLEEP_AID_SOUNDS, s_SLEEP_AID_SOUNDS);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_TELEVISIONS", g_SLEEP_AID_TELEVISIONS, s_SLEEP_AID_TELEVISIONS);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_THROUGH_A_WALL", g_SLEEP_AID_THROUGH_A_WALL, s_SLEEP_AID_THROUGH_A_WALL);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SLEEP_AID_WALL", g_SLEEP_AID_WALL, s_SLEEP_AID_WALL);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "SOUND_REPLY", g_SOUND_REPLY, s_SOUND_REPLY);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "TELLY_REPLY", g_TELLY_REPLY, s_TELLY_REPLY);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "WALL_REPLY", g_WALL_REPLY, s_WALL_REPLY);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_COMPLAINT", g__A_COMPLAINT, s__A_COMPLAINT);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_DEPENDS", g__A_DEPENDS, s__A_DEPENDS);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_LIGHT", g__A_LIGHT, s__A_LIGHT);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_NOISE", g__A_NOISE, s__A_NOISE);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_OUTSIDE", g__A_OUTSIDE, s__A_OUTSIDE);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_SCREEN", g__A_SCREEN, s__A_SCREEN);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_SLEEP", g__A_SLEEP, s__A_SLEEP);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_SOMEONE_ELSES", g__A_SOMEONE_ELSES, s__A_SOMEONE_ELSES);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_SOUND", g__A_SOUND, s__A_SOUND);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_TELLY", g__A_TELLY, s__A_TELLY);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_THROUGH_A_WALL", g__A_THROUGH_A_WALL, s__A_THROUGH_A_WALL);
    pyrt::xrefs::reg_access("knowledge.sleep_aids", "_A_WALL", g__A_WALL, s__A_WALL);
}
