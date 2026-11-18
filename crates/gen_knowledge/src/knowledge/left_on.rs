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
thread_local! { static G_BOILER_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHARGE_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLD_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISHWASHER_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRE_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAUNDRY_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIGHTS_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FIRE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IN_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAVES_AN_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONLY_HEATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RISK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BOILER_TEXT() -> Py { ensure_init(); G_BOILER_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOILER_TEXT(v: Py) { ensure_init(); G_BOILER_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHARGE_TEXT() -> Py { ensure_init(); G_CHARGE_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHARGE_TEXT(v: Py) { ensure_init(); G_CHARGE_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_COLD_TEXT() -> Py { ensure_init(); G_COLD_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLD_TEXT(v: Py) { ensure_init(); G_COLD_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_DISHWASHER_TEXT() -> Py { ensure_init(); G_DISHWASHER_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISHWASHER_TEXT(v: Py) { ensure_init(); G_DISHWASHER_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRE_TEXT() -> Py { ensure_init(); G_FIRE_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRE_TEXT(v: Py) { ensure_init(); G_FIRE_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_LAUNDRY_TEXT() -> Py { ensure_init(); G_LAUNDRY_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAUNDRY_TEXT(v: Py) { ensure_init(); G_LAUNDRY_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_LIGHTS_TEXT() -> Py { ensure_init(); G_LIGHTS_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIGHTS_TEXT(v: Py) { ensure_init(); G_LIGHTS_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__AWAY() -> Py { ensure_init(); G__AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AWAY(v: Py) { ensure_init(); G__AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__FIRE_WORDS() -> Py { ensure_init(); G__FIRE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FIRE_WORDS(v: Py) { ensure_init(); G__FIRE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUPS() -> Py { ensure_init(); G__GROUPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUPS(v: Py) { ensure_init(); G__GROUPS.with(|c| *c.borrow_mut() = v); }
pub fn g__IN_THE_THING() -> Py { ensure_init(); G__IN_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IN_THE_THING(v: Py) { ensure_init(); G__IN_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAVES_AN_OBJECT() -> Py { ensure_init(); G__LEAVES_AN_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAVES_AN_OBJECT(v: Py) { ensure_init(); G__LEAVES_AN_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__ONLY_HEATING() -> Py { ensure_init(); G__ONLY_HEATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONLY_HEATING(v: Py) { ensure_init(); G__ONLY_HEATING.with(|c| *c.borrow_mut() = v); }
pub fn g__RISK() -> Py { ensure_init(); G__RISK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RISK(v: Py) { ensure_init(); G__RISK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What happens if it is left on: the boiler, the things that heat, and the ones that run.\n\n\"Is it safe to leave the boiler on all night\" was answered with \"Use the habits that work in any\ncity after dark: stay on the busy, lit streets...\". The word `safe` was doing the routing and the\nsubject underneath it was dropped, so a line about a boiler went to the personal-safety-at-night\ncard. The same thing happened the other way: \"is it safe to leave the washing machine on\novernight\" got the dark-marks-in-the-drum answer, because `overnight` and `washing machine` are\nthat row's own words too.\n\nTwo rows live here rather than in home.py or travel.py, for the reason house_odds.py gives: a new\nrow beside the competing ones is where the question is decided, and this file costs one discovery\npass and nothing on a line it does not know.\n\nThe answers are split by what actually goes wrong. Cooking and candles are where most house fires\nstart; a tumble dryer is the fluff in its own filter; a boiler is built to run overnight and what\nit wants is a flue that is checked and an alarm that works; a fridge left shut for a fortnight\ngrows mould.\n");
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("safety")]));
    s_FIRE_TEXT(py_str("Switch it off and stay with it while it is on: cooking and candles are where most house fires start, and a grill pan or a chip pan catching is the one that kills people. If it is already burning, get everyone out and call 999 rather than throwing water on fat. If you are out of the house and it is still on, do not go back in for it - call 999 and stay out."));
    s_LAUNDRY_TEXT(py_str("A tumble dryer is the one to think about and a washer is mostly a flood: the fluff that collects in the filter and the vent is what burns, so clean the filter after every load, keep the vent clear, and run it while you are up and about rather than overnight or out. A washer left running is fine while the hoses are sound; turn the water off at the valve if you are leaving the house for long."));
    s_DISHWASHER_TEXT(py_str("A dishwasher left running is mostly a water question, not a fire one: what goes wrong is the inlet hose, the door seal or the pump, and the damage is a flood, which matters most in a flat above someone else's ceiling. Look at the hose for bulges and the seal for grit, and if you want it to run overnight put it on the delay timer after checking those rather than starting it as you go to bed. An old machine with a hose you have never looked at is the one to run while you are still up."));
    s_CHARGE_TEXT(py_str("Unplug it: a charger with nothing on the end of it stays warm for no reason, and a damaged cable or a cheap plug is the part that fails. An electric blanket is only meant to warm the bed before you get in, so put it on for half an hour and switch it off rather than leaving it on all night, and anything with a frayed lead goes in the bin."));
    s_LIGHTS_TEXT(py_str("LED sets run cool and a set left on overnight is not the same risk as a candle, but the cheap ones are the ones that fail: unplug them if the plug is warm, the cable is frayed or the set is older than you can remember buying it, and a plug-in timer costs a few pounds and takes the decision away. Nothing that runs off a battery should be left on while you are out."));
    s_COLD_TEXT(py_str("Leave the freezer running: a full freezer holds its temperature for hours in a power cut and costs very little. An empty fridge is better emptied, switched off and left with the door propped open for a fortnight away, because a fridge left shut and empty grows mould; if you leave either running, turn the water off at the valve if it has an ice maker."));
    s_BOILER_TEXT(py_str("Leave it on: a modern boiler is built to run overnight, and its frost setting fires it only when the house gets cold. What matters is not the hours but the flue and the alarm - a carbon monoxide alarm on the same floor as the boiler, the flue checked at the yearly service, and nothing stacked against the vents or the air brick. An old back boiler or an open flue is the one to ask a Gas Safe engineer about before you sleep with it on. If you mean the heating rather than the hot water, it costs more than switching it off and back on, which is the money question and not this one."));
    s__GROUPS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("freezers"), py_str("fridge freezer")]), g_COLD_TEXT()]), py_tuple(vec![py_tuple(vec![py_str("washing machine"), py_str("tumble dryer"), py_str("tumble drier"), py_str("the dryer"), py_str("my dryer")]), g_LAUNDRY_TEXT()]), py_tuple(vec![py_tuple(vec![py_str("dishwasher"), py_str("dishwashers")]), g_DISHWASHER_TEXT()]), py_tuple(vec![py_tuple(vec![py_str("charger"), py_str("on charge"), py_str("extension lead"), py_str("electric blanket"), py_str("straighteners")]), g_CHARGE_TEXT()]), py_tuple(vec![py_tuple(vec![py_str("lights"), py_str("light"), py_str("fairy lights"), py_str("christmas lights"), py_str("tree lights")]), g_LIGHTS_TEXT()])]));
    s__FIRE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("oven"), py_str("ovens"), py_str("hob"), py_str("hobs"), py_str("cooker"), py_str("cookers"), py_str("grill"), py_str("grills"), py_str("stove"), py_str("stoves"), py_str("chip pan"), py_str("deep fat fryer"), py_str("deep fryer"), py_str("fryer"), py_str("air fryer"), py_str("candle"), py_str("candles"), py_str("iron"), py_str("ironing"), py_str("heater"), py_str("fan heater"), py_str("electric heater"), py_str("toaster"), py_str("microwave"), py_str("burner"), py_str("woodburner"), py_str("fire")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__IN_THE_THING(py_tuple(vec![py_str("in the oven"), py_str("in the fridge"), py_str("in the freezer"), py_str("in the microwave"), py_str("in the dishwasher"), py_str("in the washer"), py_str("in the tumble dryer"), py_str("in the air fryer")]));
    s__AWAY(py_tuple(vec![py_str("while im away"), py_str("while i am away"), py_str("while we are away"), py_str("while we're away"), py_str("on holiday"), py_str("fortnight"), py_str("two weeks"), py_str("a week away"), py_str("for a week"), py_str("for the weekend"), py_str("turn it off"), py_str("turning it off"), py_str("switch it off"), py_str("switched it off"), py_str("switching it off"), py_str("switched off"), py_str("switch off"), py_str("unplug"), py_str("empty it"), py_str("emptied"), py_str("left running"), py_str("left on"), py_str("leave it on"), py_str("leave the fridge on"), py_str("leave the freezer on"), py_str("leave both on"), py_str("running while"), py_str("keep it on"), py_str("keeping it on"), py_str("propped open")]));
    s__LEAVES_AN_OBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bleaves\\s+(?!it\\b|them\\b|on\\b|everything\\s+on\\b|the\\s+[\\w']+\\s+on\\b|a\\s+[\\w']+\\s+on\\b|all\\b)\\w+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__RISK(py_tuple(vec![py_str("safe"), py_str("unsafe"), py_str("dangerous"), py_str("risky"), py_str("risk"), py_str("carbon monoxide"), py_str("flue"), py_str("fumes"), py_str("fire"), py_str("leak"), py_str("leaking"), py_str("the alarm"), py_str("smell"), py_str("smells"), py_str("smoke"), py_str("worried"), py_str("worry")]));
    s__ONLY_HEATING(py_tuple(vec![py_str("heating"), py_str("hot water")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("boiler-left-on-overnight")); __args.push(py_tuple(vec![py_tuple(vec![py_str("boiler"), py_str("boilers"), py_str("combi"), py_str("central heating"), py_str("heating"), py_str("hot water"), py_str("radiator"), py_str("radiators")]), py_tuple(vec![py_str("leave"), py_str("leaves"), py_str("left"), py_str("leaving"), py_str("on all night"), py_str("on overnight"), py_str("overnight"), py_str("all night"), py_str("at night"), py_str("while im out"), py_str("while i am out"), py_str("while im at work"), py_str("unattended"), py_str("turn it off"), py_str("turn off"), py_str("switch it off"), py_str("switch off")])])); __args.push(g_BOILER_TEXT()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), g__a_heating_habit_is_not_a_risk()), ("avoid".to_string(), py_tuple(vec![py_str("cheaper"), py_str("cheapest"), py_str("cost"), py_str("costs"), py_str("costly"), py_str("expensive"), py_str("save money"), py_str("saving money"), py_str("bill"), py_str("bills"), py_str("on low"), py_str("heating on low"), py_str("economy 7"), py_str("tariff"), py_str("thermostat setting"), py_str("what temperature"), py_str("away"), py_str("go away"), py_str("going away"), py_str("went away"), py_str("while im away"), py_str("while i am away"), py_str("while we are away"), py_str("on holiday"), py_str("holidays"), py_str("next week"), py_str("a week off"), py_str("for a week"), py_str("for two weeks"), py_str("fortnight"), py_str("weekend away"), py_str("for the weekend"), py_str("out all day"), py_str("all day"), py_str("pressure"), py_str("bleed"), py_str("bleeding"), py_str("noise"), py_str("noisy"), py_str("kettling"), py_str("leak"), py_str("leaking"), py_str("broken"), py_str("not working"), py_str("fault"), py_str("error code"), py_str("service"), py_str("serviced"), py_str("towel rail"), py_str("radiator valve"), py_str("trv"), py_str("away"), py_str("holiday"), py_str("holidays"), py_str("empty house"), py_str("nobody in"), py_str("for a week"), py_str("for a fortnight"), py_str("a week away"), py_str("go away"), py_str("going away"), py_str("went away"), py_str("frost"), py_str("freeze"), py_str("freezing"), py_str("pipes"), py_str("burst pipe"), py_str("all day"), py_str("out at work"), py_str("im out at work"), py_str("when im out"), py_str("timer"), py_str("24 hours"), py_str("how many hours"), py_str("immersion"), py_str("hot water cylinder"), py_str("hot water tank"), py_str("hot cylinder"), py_str("where is the valve"), py_str("isolation valve"), py_str("drain"), py_str("draining"), py_str("drained"), py_str("spur"), py_str("fused spur")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("left-on-unattended")); __args.push(py_tuple(vec![py_tuple(vec![py_str("oven"), py_str("ovens"), py_str("hob"), py_str("hobs"), py_str("cooker"), py_str("cookers"), py_str("grill"), py_str("grills"), py_str("stove"), py_str("stoves"), py_str("chip pan"), py_str("deep fat fryer"), py_str("deep fryer"), py_str("fryer"), py_str("air fryer"), py_str("candle"), py_str("candles"), py_str("iron"), py_str("straighteners"), py_str("hair straighteners"), py_str("heater"), py_str("fan heater"), py_str("electric heater"), py_str("electric blanket"), py_str("toaster"), py_str("microwave"), py_str("burner"), py_str("woodburner"), py_str("gas fire"), py_str("open fire"), py_str("fireplace"), py_str("log fire"), py_str("coal fire"), py_str("fire"), py_str("tumble dryer"), py_str("tumble drier"), py_str("dryer"), py_str("washing machine"), py_str("washer"), py_str("dishwasher"), py_str("charger"), py_str("phone charger"), py_str("laptop charger"), py_str("extension lead"), py_str("fridge"), py_str("freezer"), py_str("fridge freezer"), py_str("fairy lights"), py_str("christmas lights"), py_str("tree lights")]), py_tuple(vec![py_str("leave"), py_str("leaves"), py_str("left"), py_str("leaving"), py_str("on all night"), py_str("on overnight"), py_str("overnight"), py_str("all night"), py_str("at night"), py_str("while im out"), py_str("while i go out"), py_str("while we are out"), py_str("while im away"), py_str("while we are away"), py_str("while im at work"), py_str("while i sleep"), py_str("asleep"), py_str("out of the house"), py_str("unattended"), py_str("forgot"), py_str("forgot to turn"), py_str("didnt turn"), py_str("did not turn"), py_str("still on"), py_str("should i turn"), py_str("turn it off"), py_str("switch it off")])])); __args.push(g__left_on_reply()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), g__left_on_answers_this()), ("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("how many minutes"), py_str("temperature"), py_str("gas mark"), py_str("preheat"), py_str("recipe"), py_str("leaves clothes"), py_str("leaves the clothes"), py_str("leaves my clothes"), py_str("clothes too wet"), py_str("clothes are too wet"), py_str("clothes still wet"), py_str("clothes come out wet"), py_str("washing too wet"), py_str("washing comes out wet"), py_str("comes out too wet"), py_str("too wet"), py_str("roast"), py_str("bake"), py_str("clean"), py_str("cleaning"), py_str("descale"), py_str("filter"), py_str("smell"), py_str("smells"), py_str("not heating"), py_str("broken"), py_str("repair"), py_str("fuse"), py_str("error code"), py_str("buy"), py_str("recommend"), py_str("which one"), py_str("drum"), py_str("seal"), py_str("dark marks"), py_str("wont spin"), py_str("not draining"), py_str("slow cooker"), py_str("crockpot"), py_str("crock pot"), py_str("air fryer recipe"), py_str("defrost"), py_str("ice"), py_str("frozen"), py_str("power cut"), py_str("not cold"), py_str("freezer not"), py_str("cheaper"), py_str("cheapest"), py_str("cheap"), py_str("cheaply"), py_str("cost"), py_str("costs"), py_str("costly"), py_str("expensive"), py_str("electricity"), py_str("tariff"), py_str("off peak"), py_str("off-peak"), py_str("economy 7"), py_str("economy7"), py_str("time of use"), py_str("best time"), py_str("what time"), py_str("low rate"), py_str("save money"), py_str("saving money"), py_str("bill"), py_str("bills"), py_str("how much does it cost")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _left_on_answers_this(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("False for the keeping questions that name an appliance but are not about it.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__IN_THE_THING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEAVES_AN_OBJECT(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&line.clone(), &py_str("fridge"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("freezer"))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__AWAY()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_heating_habit_is_not_a_risk(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False for \"should i leave the heating on overnight\", which is the timer's question.\n\n    The boiler card is the answer to a risk question about a boiler. `heating` and `hot water`\n    are in the subject list because \"is it safe to leave the heating on all night\" is a real\n    line, but they are also how people ask what the timer should do - and that question has the\n    cost answer, which says the timer wins. So when `heating` is the only subject in the line\n    and nothing in it is a risk word, this row stands down.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__RISK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ONLY_HEATING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn _left_on_reply(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut line: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The answer for the thing that is on, and the fire answer when it is anything that heats.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&g__GROUPS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    group = __unpk2[0].clone();
    text = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g__FIRE_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&group.clone(), &py_str("lights"))) } }) {
    return g_FIRE_TEXT();
    }
    return text.clone();
    }
    }
    return g_FIRE_TEXT();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.left_on".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "BOILER_TEXT" => Some(g_BOILER_TEXT()),
            "CHARGE_TEXT" => Some(g_CHARGE_TEXT()),
            "COLD_TEXT" => Some(g_COLD_TEXT()),
            "DISHWASHER_TEXT" => Some(g_DISHWASHER_TEXT()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FIRE_TEXT" => Some(g_FIRE_TEXT()),
            "LAUNDRY_TEXT" => Some(g_LAUNDRY_TEXT()),
            "LIGHTS_TEXT" => Some(g_LIGHTS_TEXT()),
            "_AWAY" => Some(g__AWAY()),
            "_FIRE_WORDS" => Some(g__FIRE_WORDS()),
            "_GROUPS" => Some(g__GROUPS()),
            "_IN_THE_THING" => Some(g__IN_THE_THING()),
            "_LEAVES_AN_OBJECT" => Some(g__LEAVES_AN_OBJECT()),
            "_ONLY_HEATING" => Some(g__ONLY_HEATING()),
            "_RISK" => Some(g__RISK()),
            "_a_heating_habit_is_not_a_risk" => Some(py_func("_a_heating_habit_is_not_a_risk", |args: &[Py]| _a_heating_habit_is_not_a_risk(args))),
            "_left_on_answers_this" => Some(py_func("_left_on_answers_this", |args: &[Py]| _left_on_answers_this(args))),
            "_left_on_reply" => Some(py_func("_left_on_reply", |args: &[Py]| _left_on_reply(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "BOILER_TEXT" => s_BOILER_TEXT(v),
            "CHARGE_TEXT" => s_CHARGE_TEXT(v),
            "COLD_TEXT" => s_COLD_TEXT(v),
            "DISHWASHER_TEXT" => s_DISHWASHER_TEXT(v),
            "ENTRIES" => s_ENTRIES(v),
            "FIRE_TEXT" => s_FIRE_TEXT(v),
            "LAUNDRY_TEXT" => s_LAUNDRY_TEXT(v),
            "LIGHTS_TEXT" => s_LIGHTS_TEXT(v),
            "_AWAY" => s__AWAY(v),
            "_FIRE_WORDS" => s__FIRE_WORDS(v),
            "_GROUPS" => s__GROUPS(v),
            "_IN_THE_THING" => s__IN_THE_THING(v),
            "_LEAVES_AN_OBJECT" => s__LEAVES_AN_OBJECT(v),
            "_ONLY_HEATING" => s__ONLY_HEATING(v),
            "_RISK" => s__RISK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.left_on", module_obj);
    pyrt::xrefs::reg_fn("knowledge.left_on", "_left_on_answers_this", py_func("_left_on_answers_this", |args: &[Py]| _left_on_answers_this(args)));
    pyrt::xrefs::reg_fn("knowledge.left_on", "_a_heating_habit_is_not_a_risk", py_func("_a_heating_habit_is_not_a_risk", |args: &[Py]| _a_heating_habit_is_not_a_risk(args)));
    pyrt::xrefs::reg_fn("knowledge.left_on", "_left_on_reply", py_func("_left_on_reply", |args: &[Py]| _left_on_reply(args)));
    pyrt::xrefs::reg_access("knowledge.left_on", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.left_on", "BOILER_TEXT", g_BOILER_TEXT, s_BOILER_TEXT);
    pyrt::xrefs::reg_access("knowledge.left_on", "CHARGE_TEXT", g_CHARGE_TEXT, s_CHARGE_TEXT);
    pyrt::xrefs::reg_access("knowledge.left_on", "COLD_TEXT", g_COLD_TEXT, s_COLD_TEXT);
    pyrt::xrefs::reg_access("knowledge.left_on", "DISHWASHER_TEXT", g_DISHWASHER_TEXT, s_DISHWASHER_TEXT);
    pyrt::xrefs::reg_access("knowledge.left_on", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.left_on", "FIRE_TEXT", g_FIRE_TEXT, s_FIRE_TEXT);
    pyrt::xrefs::reg_access("knowledge.left_on", "LAUNDRY_TEXT", g_LAUNDRY_TEXT, s_LAUNDRY_TEXT);
    pyrt::xrefs::reg_access("knowledge.left_on", "LIGHTS_TEXT", g_LIGHTS_TEXT, s_LIGHTS_TEXT);
    pyrt::xrefs::reg_access("knowledge.left_on", "_AWAY", g__AWAY, s__AWAY);
    pyrt::xrefs::reg_access("knowledge.left_on", "_FIRE_WORDS", g__FIRE_WORDS, s__FIRE_WORDS);
    pyrt::xrefs::reg_access("knowledge.left_on", "_GROUPS", g__GROUPS, s__GROUPS);
    pyrt::xrefs::reg_access("knowledge.left_on", "_IN_THE_THING", g__IN_THE_THING, s__IN_THE_THING);
    pyrt::xrefs::reg_access("knowledge.left_on", "_LEAVES_AN_OBJECT", g__LEAVES_AN_OBJECT, s__LEAVES_AN_OBJECT);
    pyrt::xrefs::reg_access("knowledge.left_on", "_ONLY_HEATING", g__ONLY_HEATING, s__ONLY_HEATING);
    pyrt::xrefs::reg_access("knowledge.left_on", "_RISK", g__RISK, s__RISK);
}
