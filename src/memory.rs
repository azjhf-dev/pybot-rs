//! Conversation memory: a plain Python dict in the transpiled world.
use pyrt::value::*;

pub type Memory = Py;

pub fn new_memory() -> Memory { py_dict(vec![]) }

pub fn push_turn(memory: &Memory, text: &str, reply: &str, module: Option<&str>) {
    if let Py::Dict(_) = memory {
        let turns = py_getitem(memory, &py_str("turns")).unwrap_or(Py::None);
        let turns = if matches!(turns, Py::List(_)) { turns } else {
            let t = py_list(vec![]);
            let _ = py_setitem(memory, &py_str("turns"), t.clone());
            t
        };
        if let Py::List(l) = turns {
            l.borrow_mut().push(py_dict(vec![
                (py_str("user"), py_str(text)),
                (py_str("bot"), py_str(reply)),
                (py_str("module"), module.map(py_str).unwrap_or(Py::None)),
            ]));
        }
    }
}
