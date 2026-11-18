//! Python exceptions as Rust panics carrying a PyExc payload.
use std::any::Any;
use std::cell::Cell;
use std::rc::Rc;

use crate::value::Py;

#[derive(Clone, Debug)]
pub struct PyExc {
    pub class_name: String,
    pub message: String,
    pub value: Option<Py>,
}

impl PyExc {
    pub fn new(class_name: &str, message: &str) -> PyExc {
        PyExc { class_name: class_name.to_string(), message: message.to_string(), value: None }
    }
    pub fn with_value(class_name: &str, message: &str, value: Py) -> PyExc {
        PyExc { class_name: class_name.to_string(), message: message.to_string(), value: Some(value) }
    }
    /// True when this exception is caught by a handler naming `handler`.
    pub fn matches(&self, handler: &str) -> bool {
        exc_matches(&self.class_name, handler)
    }
}

pub fn py_err(class_name: &str, message: &str) -> PyExc { PyExc::new(class_name, message) }

thread_local! {
    static SUPPRESS: Cell<bool> = const { Cell::new(false) };
}

/// Panic with a Python exception payload.
pub fn raise(exc: PyExc) -> ! {
    SUPPRESS.with(|s| s.set(true));
    std::panic::panic_any(exc);
}

/// Run f, converting a PyExc panic into Err. Other panics propagate.
pub fn catch<F: FnOnce() -> Py>(f: F) -> Result<Py, PyExc> {
    SUPPRESS.with(|s| s.set(false));
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(v) => Ok(v),
        Err(payload) => {
            SUPPRESS.with(|s| s.set(false));
            match downcast(payload) {
                Some(exc) => Err(exc),
                None => std::panic::panic_any(FatalPanic),
            }
        }
    }
}

struct FatalPanic;

pub fn downcast(payload: Box<dyn Any + Send>) -> Option<PyExc> {
    if let Ok(exc) = payload.downcast::<PyExc>() { Some(*exc) } else { None }
}

/// Install a panic hook that swallows PyExc panics (they are control flow).
pub fn install_hook() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if SUPPRESS.with(|s| s.get()) {
            SUPPRESS.with(|s| s.set(false));
            return;
        }
        default(info);
    }));
}

fn exc_matches(exc: &str, handler: &str) -> bool {
    if exc == handler { return true; }
    match handler {
        "Exception" | "BaseException" => true,
        "ArithmeticError" => matches!(exc, "ZeroDivisionError" | "OverflowError" | "FloatingPointError"),
        "LookupError" => matches!(exc, "KeyError" | "IndexError"),
        "OSError" => matches!(exc, "FileNotFoundError" | "IOError"),
        "ValueError" => matches!(exc, "UnicodeDecodeError" | "UnicodeError"),
        _ => false,
    }
}

/// Helper for except matching with a tuple of names.
pub fn matches_any(exc: &PyExc, handlers: &[&str]) -> bool {
    handlers.iter().any(|h| exc.matches(h))
}

pub fn exc_to_py(exc: &PyExc) -> Py {
    Py::Exc(Rc::new(exc.clone()))
}

/// Extract the exception payload from a panic during `try` codegen.
/// Returns (exc, handler args as Py).
pub fn bind_args(exc: &PyExc) -> Py {
    exc_to_py(exc)
}
