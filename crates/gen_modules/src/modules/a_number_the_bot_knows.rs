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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A person asking for a phone number gets the number this bot already knows, not a search.\n\n\"whats the number for the samaritans\", \"whats the number for 111\", \"childline number\", \"how do i\nring acas\" - on main every one of those is answered by the same sentence:\n\n    \"No directory here, sorry. A map app search for the name shows the address and the number,\n     and the place's own page has the hours.\"\n\nThat is the right route for a shop, a surgery or a takeaway, whose number only a directory has.\nIt is the wrong route for a national line, because the bot already states those numbers in its own\ncards (116 123, 0808 2000 247, 0300 123 1100, 111, 988, 999), and somebody in trouble should not\nbe sent to a map app to find the Samaritans.\n\nNothing here works out a number or fills one in: the reading in `bot/compose/a_number_the_bot_knows.py`\nholds only the lines whose numbers the tree already states, word-bounded so \"refuge\" does not fire\non \"refugees\" and \"111\" not on \"1111\", and a helpline whose number the tree does not hold gets its\nown page named rather than a directory route.\n\nThis sits above the greeting band (PRIORITY -1000) and below every knowledge module, the same\nrung as `a_day_somebody_else_keeps`, so a card, an entry, `answers` or the safety band keeps the\nturn: only a line nothing else answered is answered here. `how do i ring acas` still gets the\nanswers module's card for that reason, which is the ordering working as it should.\n");
    s_PRIORITY(op_neg(&Py::Int(999)));
}
pub fn respond(args: &[Py]) -> Py {
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
    else { Py::None };
    let _ = py_str("The number the bot already knows, or None.");
    return gen_compose::compose::a_number_the_bot_knows::answer_for(&[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_number_the_bot_knows".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_number_the_bot_knows", module_obj);
    pyrt::xrefs::reg_fn("modules.a_number_the_bot_knows", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_number_the_bot_knows", "PRIORITY", g_PRIORITY, s_PRIORITY);
}
