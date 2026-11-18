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
    let _ = py_str("Somebody they are close to is in a rough patch: answer the person, not the receipt.\n\nThe reading and the wording are in `bot/compose/theirperson.py`, which says why the shape is\nworth one question back. This is the module that offers it, and the reason it is a module of its\nown rather than four lines in the seed fallback: the shape is a *shape*, and a hook in the\nfallback would have to grow a branch for the next one too.\n\nPRIORITY 250 puts it above the seed fallback (-1000) and below every module that can answer the\nline for itself: `chat` (300) keeps \"my mate has been really down lately\", `answers` (1100) keeps\n\"my best friend is struggling with his reading\", and `loss` (2000) keeps \"my mum is having a hard\ntime\" when the line also says somebody died. What it can lose is a receipt; what it cannot do is\ntake a turn from a better answer.\n\nIt also asks the composer what it would have said, and stands down unless that line was a\nplaceholder (`bot/compose/theirperson.py`, `may_replace`): a pool line with a feeling in it -\n\"That sounds rough. If you want, I can help you think through what to do next.\" - is a reply, and\nthis module has nothing to add to it. The composer is asked here and again by the fallback, which\nis the price of not touching a file every lane edits.\n");
    s_PRIORITY(Py::Int(250));
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reply: Py = Py::None;
    reply = gen_compose::compose::theirperson::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&gen_compose::compose::theirperson::may_replace(&[py_call(&gen_compose::compose::fallback::g_reply_for(), &[text.clone(), memory.clone()]), text.clone()])))) {
    return Py::None;
    }
    return reply.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.theirperson".to_string(),
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
    pyrt::xrefs::reg_module("modules.theirperson", module_obj);
    pyrt::xrefs::reg_fn("modules.theirperson", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.theirperson", "PRIORITY", g_PRIORITY, s_PRIORITY);
}
