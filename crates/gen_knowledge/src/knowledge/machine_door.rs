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
thread_local! { static G__A_SMELLY_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MACHINE_IN_THE_CHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__A_SMELLY_MACHINE() -> Py { ensure_init(); G__A_SMELLY_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SMELLY_MACHINE(v: Py) { ensure_init(); G__A_SMELLY_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g__MACHINE_IN_THE_CHAT() -> Py { ensure_init(); G__MACHINE_IN_THE_CHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MACHINE_IN_THE_CHAT(v: Py) { ensure_init(); G__MACHINE_IN_THE_CHAT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The door of a machine: a damp washer, and a remark about leaving it shut.\n\nThis is one row, and it sits in its own file rather than in home.py because home.py is at the\npublisher's 1 MiB limit and every lane is appending to the end of it, so a row there collides on\nevery rebase. It answers a line that is a *cause*, not a fault.\n");
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
    s__MACHINE_IN_THE_CHAT(py_tuple(vec![py_str("washing machine"), py_str("washer"), py_str("washer dryer"), py_str("dishwasher"), py_str("washing")]));
    s__A_SMELLY_MACHINE(py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelly"), py_str("stink"), py_str("stinks"), py_str("damp"), py_str("musty"), py_str("mould"), py_str("mold"), py_str("mildew"), py_str("fusty"), py_str("whiff"), py_str("sour")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("damp-machine-door")); __args.push(py_tuple(vec![py_tuple(vec![py_str("door"), py_str("doors")]), py_tuple(vec![py_str("shut"), py_str("shuts"), py_str("closed"), py_str("closing"), py_str("left shut"), py_str("keep it shut")])])); __args.push(g__a_damp_machine_and_its_door()); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("stiff"), py_str("sticking"), py_str("sticks"), py_str("tight"), py_str("hard to shut"), py_str("hard to close"), py_str("rubbing"), py_str("rubs"), py_str("drags"), py_str("dragging"), py_str("squeak"), py_str("squeaks"), py_str("squeaking"), py_str("squeal"), py_str("creak"), py_str("creaks"), py_str("hinge"), py_str("hinges"), py_str("latch"), py_str("lock"), py_str("locked"), py_str("keyhole"), py_str("slam"), py_str("slams"), py_str("bang"), py_str("bangs"), py_str("banging"), py_str("swings"), py_str("swinging"), py_str("swung"), py_str("on its own"), py_str("itself"), py_str("hanging"), py_str("off"), py_str("dropped"), py_str("draught"), py_str("draft"), py_str("measure"), py_str("paint"), py_str("cant shut"), py_str("cant close"), py_str("cannot shut"), py_str("cannot close"), py_str("can not shut"), py_str("can not close"), py_str("wont shut"), py_str("wont close"), py_str("will not shut"), py_str("will not close"), py_str("does not shut"), py_str("does not close"), py_str("doesnt shut"), py_str("doesnt close"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("microwave"), py_str("cooker"), py_str("car door"), py_str("front door"), py_str("back door"), py_str("car"), py_str("van"), py_str("bedroom door"), py_str("shower"), py_str("curtain"), py_str("blinds"), py_str("window"), py_str("gate"), py_str("key"), py_str("lock"), py_str("house"), py_str("flat"), py_str("landlord"), py_str("neighbour"), py_str("neighbor")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _a_damp_machine_and_its_door(args: &[Py]) -> Py {
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
    let mut chat: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The door left shut, said after a smelly machine, is the cause of the smell.\n\n    Deck row `door-subject-01`: \"my washing machine smells of damp\" then \"i leave the door shut\".\n    On main that second turn gets \"Oil the hinges\" - the machine card has just said that leaving\n    the door and drawer open is what stops the smell coming back, and the person is telling the bot\n    why their machine smells.\n\n    The row needs the machine and a smell in what the *person* said (not in the bot's last answer,\n    which mentions a smell on the way past), and a door that is shut rather than a fault with it:\n    a door that will not shut, swings, rubs, squeaks, bangs, hangs off or needs a lock is the hinge\n    and latch rows' answer, and so is another door - a fridge, an oven, a car, a bedroom door.\n    ");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&stdlib_call("re", "search", &[py_str("cant|cannot|can not|won'?t|will not|does not|doesn'?t|hard to|stuck|swings?|swinging|on its own|itself|hanging|off|lock|keyhole|slam|bang|squeak|squeal|rub|paint|draught|draft|dragging|dropped|measure|door is not|door isnt|door isn'?t|door is broken|door is stiff|door is loose|door is stuck|door is jamming|door was broken|door was stiff|door was loose"), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    said = py_list(vec![call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let __aug1 = op_add(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() });
    said = __aug1;
    chat = mcall(&mcall(&py_str(" "), "join", &[said.clone()]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MACHINE_IN_THE_CHAT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&chat.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_SMELLY_MACHINE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&chat.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return py_str("Then that is very likely the cause: the inside of the machine stays wet between washes, and the smell grows in water that cannot dry. Leave the door and the detergent drawer open between washes, wipe the rubber seal round the door, and run one hot 60C or 90C wash with a machine cleaner or a cup of bicarbonate of soda to clear what has already built up.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.machine_door".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_SMELLY_MACHINE" => Some(g__A_SMELLY_MACHINE()),
            "_MACHINE_IN_THE_CHAT" => Some(g__MACHINE_IN_THE_CHAT()),
            "_a_damp_machine_and_its_door" => Some(py_func("_a_damp_machine_and_its_door", |args: &[Py]| _a_damp_machine_and_its_door(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_A_SMELLY_MACHINE" => s__A_SMELLY_MACHINE(v),
            "_MACHINE_IN_THE_CHAT" => s__MACHINE_IN_THE_CHAT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.machine_door", module_obj);
    pyrt::xrefs::reg_fn("knowledge.machine_door", "_a_damp_machine_and_its_door", py_func("_a_damp_machine_and_its_door", |args: &[Py]| _a_damp_machine_and_its_door(args)));
    pyrt::xrefs::reg_access("knowledge.machine_door", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.machine_door", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.machine_door", "_A_SMELLY_MACHINE", g__A_SMELLY_MACHINE, s__A_SMELLY_MACHINE);
    pyrt::xrefs::reg_access("knowledge.machine_door", "_MACHINE_IN_THE_CHAT", g__MACHINE_IN_THE_CHAT, s__MACHINE_IN_THE_CHAT);
}
