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
pub static _ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HAS_ALREADY_FROZEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IN_A_POT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_COLD_AND_THE_MOVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g__ASK() -> Py { ensure_init(); _ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK(v: Py) { ensure_init(); if let Ok(mut w) = _ASK.write() { *w = v; } }
pub fn g__HAS_ALREADY_FROZEN() -> Py { ensure_init(); _HAS_ALREADY_FROZEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HAS_ALREADY_FROZEN(v: Py) { ensure_init(); if let Ok(mut w) = _HAS_ALREADY_FROZEN.write() { *w = v; } }
pub fn g__IN_A_POT() -> Py { ensure_init(); _IN_A_POT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IN_A_POT(v: Py) { ensure_init(); if let Ok(mut w) = _IN_A_POT.write() { *w = v; } }
pub fn g__REPLY() -> Py { ensure_init(); _REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__REPLY(v: Py) { ensure_init(); if let Ok(mut w) = _REPLY.write() { *w = v; } }
pub fn g__THE_COLD_AND_THE_MOVE() -> Py { ensure_init(); _THE_COLD_AND_THE_MOVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_COLD_AND_THE_MOVE(v: Py) { ensure_init(); if let Ok(mut w) = _THE_COLD_AND_THE_MOVE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A pot is not a patch of ground, and the frost question about it is a different one.\n\n`do i need to bring my pots in tonight` was answered with the allotment-rules card: the word `pots`\ntook the line to the tenancy of a plot and nothing in it was about a plant. `should i bring my pots\ninside` was a shrug. The cards that do know about frost answer a *plant in the ground* - water the\nsoil, cover the tender plants, leave the blackened leaves alone - and the one thing that matters\nfor a container is the one thing none of them says: a pot freezes from every side, so its roots sit\nabove the frost line and a night that a border shrugs off can kill a plant in a tub.\n\nSo the question is read as the move, not the word: the line has to ask about bringing something in,\nmoving it, or leaving it out, and only then is a pot or a tender plant what it is about. A plant\nthat has already frozen is the frost-damage card's question, and a pot that will \"survive the\nfrost\" is the watering card's, so both are left alone.\n");
    s_ANSWERS(py_tuple(vec![py_str("garden-and-plants")]));
    s__IN_A_POT(py_tuple(vec![py_str("pots"), py_str("pot"), py_str("plant pots"), py_str("patio pots"), py_str("containers"), py_str("container"), py_str("tubs"), py_str("tub"), py_str("planters"), py_str("planter"), py_str("hanging baskets"), py_str("window boxes"), py_str("geraniums"), py_str("geranium"), py_str("pelargoniums"), py_str("fuchsias"), py_str("begonias"), py_str("bedding plants"), py_str("tender plants"), py_str("bay tree"), py_str("bay trees"), py_str("olive tree"), py_str("olive trees"), py_str("lemon tree"), py_str("citrus"), py_str("agapanthus"), py_str("canna"), py_str("cannas"), py_str("dahlias"), py_str("succulents"), py_str("houseplants"), py_str("house plants")]));
    s__THE_COLD_AND_THE_MOVE(py_tuple(vec![py_str("frost"), py_str("frosts"), py_str("frosty"), py_str("freezing"), py_str("freeze"), py_str("cold"), py_str("cold snap"), py_str("tonight"), py_str("overnight"), py_str("winter"), py_str("wintering"), py_str("snow"), py_str("minus"), py_str("bring in"), py_str("bring inside"), py_str("take in"), py_str("move"), py_str("moving"), py_str("moved"), py_str("leave out"), py_str("left out"), py_str("outside"), py_str("indoors"), py_str("inside"), py_str("shelter"), py_str("stay out"), py_str("over winter"), py_str("in"), py_str("in for the winter")]));
    s__ASK(py_tuple(vec![py_str("bring in"), py_str("bring them in"), py_str("bring it in"), py_str("bring inside"), py_str("bring my"), py_str("take in"), py_str("take them in"), py_str("move"), py_str("should i move"), py_str("leave out"), py_str("leave them out"), py_str("covered"), py_str("cover them"), py_str("cover my"), py_str("tonight"), py_str("overnight"), py_str("too late"), py_str("stay out"), py_str("over winter"), py_str("leave out"), py_str("left out"), py_str("bring my")]));
    s__HAS_ALREADY_FROZEN(stdlib_call("re", "compile", &[py_str("\\b(?:have|has|had|got|been|is|are|was|were)\\s+(?:been\\s+)?(?:frozen|caught|blackened|dead)\\b|\\bblackened\\b|\\bdead\\b|\\bdied\\b|\\bkilled\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__REPLY(py_str("Bring them in if you can, because a pot freezes from every side: the roots sit above the frost line, so a night that a border shrugs off can see off a plant in a tub. If they are too heavy to move, stand them together against a house wall on pot feet, wrap the pot rather than the leaves in bubble wrap or a sack, and push them under the eaves - grouping them and getting them out of the wind does most of the work. A forecast of 2C or below at ground level is the one to act on, and anything tender left out after that is worth bringing in for good."));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-pot-is-not-a-border")); __args.push(py_tuple(vec![g__IN_A_POT(), g__THE_COLD_AND_THE_MOVE()])); __args.push(g__REPLY()); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), g__ASK()), ("guard".to_string(), g__the_night_is_still_coming()), ("avoid".to_string(), py_tuple(vec![py_str("cracked"), py_str("broken"), py_str("chip"), py_str("chipped"), py_str("glaze"), py_str("terracotta pot broke"), py_str("repot"), py_str("repotting"), py_str("potting on"), py_str("compost"), py_str("soil"), py_str("saucepan"), py_str("casserole"), py_str("slow cooker"), py_str("paint"), py_str("plant pot saucer"), py_str("buy"), py_str("price"), py_str("cheap"), py_str("where can i"), py_str("allotment"), py_str("plot"), py_str("tenancy"), py_str("greenhouse heater"), py_str("heater"), py_str("paraffin")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _the_night_is_still_coming(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True while the frost is still ahead of the line, which is when moving a pot is the answer.");
    said = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    said = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    said = if py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]) } else { py_str("") };
    }
    }
    return Py::Bool(!py_truthy(&mcall(&g__HAS_ALREADY_FROZEN(), "search", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.containers_frost".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASK" => Some(g__ASK()),
            "_HAS_ALREADY_FROZEN" => Some(g__HAS_ALREADY_FROZEN()),
            "_IN_A_POT" => Some(g__IN_A_POT()),
            "_REPLY" => Some(g__REPLY()),
            "_THE_COLD_AND_THE_MOVE" => Some(g__THE_COLD_AND_THE_MOVE()),
            "_the_night_is_still_coming" => Some(py_func("_the_night_is_still_coming", |args: &[Py]| _the_night_is_still_coming(args))),
            _ => None,
        }),
        set: None,
    }))
}
