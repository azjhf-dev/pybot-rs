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


thread_local! { static G_ARBITER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FALLBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ARBITER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HANDLERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OBSERVERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ARBITER() -> Py { ensure_init(); G_ARBITER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARBITER(v: Py) { ensure_init(); G_ARBITER.with(|c| *c.borrow_mut() = v); }
pub fn g_FALLBACK() -> Py { ensure_init(); G_FALLBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FALLBACK(v: Py) { ensure_init(); G_FALLBACK.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER() -> Py { ensure_init(); G__AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER(v: Py) { ensure_init(); G__AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__ARBITER() -> Py { ensure_init(); G__ARBITER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ARBITER(v: Py) { ensure_init(); G__ARBITER.with(|c| *c.borrow_mut() = v); }
pub fn g__HANDLERS() -> Py { ensure_init(); G__HANDLERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HANDLERS(v: Py) { ensure_init(); G__HANDLERS.with(|c| *c.borrow_mut() = v); }
pub fn g__MODULES() -> Py { ensure_init(); G__MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MODULES(v: Py) { ensure_init(); G__MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g__OBSERVERS() -> Py { ensure_init(); G__OBSERVERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OBSERVERS(v: Py) { ensure_init(); G__OBSERVERS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Fixed module interface. Edit bot/modules/, never this file.\n\nEvery turn runs the modules in bot/modules/ in descending PRIORITY (default 0), then by\nfilename, in three phases:\n\n1. observe(text, memory): remember things (a name, a fact, the topic). It never replies.\n2. respond(text, memory): propose a reply (a string) or pass (None). Every module proposes;\n   each one works on its own copy of memory, so a reply that is not chosen leaves no trace.\n3. choose(text, memory, candidates) in bot/modules/arbiter.py picks one candidate, and the\n   chosen module's copy of memory becomes the conversation's memory. A candidate is\n   {\"module\": name, \"priority\": PRIORITY, \"reply\": text}; they arrive in the order above.\n   Without a working arbiter, the first candidate (the highest PRIORITY) is chosen.\n\nThen after(text, reply, memory) runs in every module that defines it, and the runtime appends\n{\"user\": text, \"bot\": reply, \"module\": name} to memory[\"turns\"] (read it, never change it).\nA module that fails is skipped; a turn never fails.\n");
    s_ARBITER(py_str("arbiter"));
    s_FALLBACK(py_str("Hello!"));
    s__MODULES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| load_modules(args))(&__args) });
    s__OBSERVERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("observe")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _hooks(args))(&__args) });
    s__HANDLERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("respond")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _hooks(args))(&__args) });
    s__AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("after")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _hooks(args))(&__args) });
    s__ARBITER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MODULES()) { let __unpk1111 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk1111[0].clone(); let mut _u: Py = __unpk1111[1].clone(); let mut module: Py = __unpk1111[2].clone(); if py_truthy(&Py::Bool(py_eq(&name.clone(), &g_ARBITER()))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(module.clone()); __args.push(py_str("choose")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn load_modules(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = Py::None;
    let mut loaded: Py = Py::None;
    let mut module: Py = Py::None;
    let mut name: Py = Py::None;
    let mut priority: Py = Py::None;
    let _ = py_str("[(name, priority, module)], highest PRIORITY first, then by filename.");
    loaded = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gen_modules::modules::g___path__()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("pkgutil", "iter_modules", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    entry = __item.clone();
    let __try2 = exc::catch(|| {
    module = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __s = String::new(); __s.push_str(&fstr(&gen_modules::modules::g___name__(), -1, "")); __s.push_str("."); __s.push_str(&fstr(&gattr(&entry.clone(), "name"), -1, "")); py_string(__s) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("importlib", "import_module", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    priority = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(module.clone()); __args.push(py_str("PRIORITY")); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(priority.clone()); __args.push(py_func("bool", |args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(priority.clone()); __args.push(py_tuple(vec![py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e))), py_func("float", |args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    priority = Py::Int(0);
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![priority.clone(), gattr(&entry.clone(), "name"), module.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&loaded.clone(), "append", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let entry = entry.clone(); let loaded = loaded.clone(); let module = module.clone(); let name = name.clone(); let priority = priority.clone(); py_func("lambda", move |args: &[Py]| { let entry = entry.clone(); let loaded = loaded.clone(); let module = module.clone(); let name = name.clone(); let priority = priority.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); py_tuple(vec![op_neg(&geti(&item.clone(), &Py::Int(0))), geti(&item.clone(), &Py::Int(1))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&loaded.clone(), "sort", args))(&__args) };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&loaded.clone()) { let __unpk9703 = py_unpack(&__item.clone(), 3, None); let mut priority: Py = __unpk9703[0].clone(); let mut name: Py = __unpk9703[1].clone(); let mut module: Py = __unpk9703[2].clone(); __out.push(py_tuple(vec![name.clone(), priority.clone(), module.clone()])); } py_list(__out) })() };
    Py::None
}
pub fn _hooks(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut module: Py = Py::None;
    let mut module_name: Py = Py::None;
    let mut priority: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MODULES()) { let __unpk3880 = py_unpack(&__item.clone(), 3, None); let mut module_name: Py = __unpk3880[0].clone(); let mut priority: Py = __unpk3880[1].clone(); let mut module: Py = __unpk3880[2].clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(module.clone()); __args.push(name.clone()); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("callable", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(py_tuple(vec![module_name.clone(), priority.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(module.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])); } } py_list(__out) })() };
    Py::None
}
pub fn _copy(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A module's own copy of memory. memory[\"turns\"] is shared: only the runtime writes it.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "items", args))(&__args) }) { let __unpk1096 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk1096[0].clone(); let mut value: Py = __unpk1096[1].clone(); __out.push((key.clone(), if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("turns")))) { value.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("copy", "deepcopy", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } })); } py_dict(__out) })() }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return memory.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _choose(args: &[Py]) -> Py {
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
    let mut candidates: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut c: Py = Py::None;
    let mut i: Py = Py::None;
    let mut picked: Py = Py::None;
    let mut view: Py = Py::None;
    let mut views: Py = Py::None;
    let _ = py_str("The arbiter's pick, as an index into candidates; 0 when it fails or returns nonsense.");
    if py_truthy(&{ let __b = Py::Bool(py_is(&g__ARBITER(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(candidates.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {
    return Py::Int(0);
    }
    views = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&candidates.clone()) { let mut c: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(c.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() };
    let __try1 = exc::catch(|| {
    picked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); __args.push(views.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&g__ARBITER(), args))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Int(0);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(picked.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(picked.clone()); __args.push(py_func("bool", |args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { { let __cmp61: Py = picked.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp61)) { op_lt(&__cmp61, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(views.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } else { Py::Bool(false) } } } } } }) {
    return picked.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(picked.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __iter7 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(views.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter7 {
    let __unpk8 = py_unpack(&__item.clone(), 2, None);
    i = __unpk8[0].clone();
    view = __unpk8[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&view.clone(), &picked.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&view.clone(), &py_str("module")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&picked.clone(), "get", args))(&__args) })) } }) {
    return i.clone();
    }
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn turn(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut after: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut candidates: Py = Py::None;
    let mut copies: Py = Py::None;
    let mut index: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut module: Py = Py::None;
    let mut name: Py = Py::None;
    let mut observe: Py = Py::None;
    let mut priority: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut respond: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One turn: {\"reply\", \"module\", \"candidates\"}. Updates memory in place.");
    let __iter1 = py_iter_or_raise(&g__OBSERVERS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _u = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    observe = __unpk2[2].clone();
    let __try3 = exc::catch(|| {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&observe, args))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["Exception"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    }
    let __unpk8 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    candidates = __unpk8[0].clone();
    copies = __unpk8[1].clone();
    let __iter9 = py_iter_or_raise(&g__HANDLERS());
    for __item in __iter9 {
    let __unpk10 = py_unpack(&__item.clone(), 3, None);
    name = __unpk10[0].clone();
    priority = __unpk10[1].clone();
    respond = __unpk10[2].clone();
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _copy(args))(&__args) };
    let __try11 = exc::catch(|| {
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&respond, args))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret12: Py = Py::Bool(false);
    let mut __val13: Py = Py::None;
    let mut __unhandled14: Option<exc::PyExc> = None;
    match __try11 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret12 = __p[0].clone(); __val13 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled15 = false;
            if !__handled15 && exc::matches_any(&__e, &["Exception"]) {
                __handled15 = true;
    continue;
            }
            if !__handled15 { __unhandled14 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled14 { exc::raise(__e); }
    if py_truthy(&__ret12) { return __val13; }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "strip", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("module"), name.clone()), (py_str("priority"), priority.clone()), (py_str("reply"), answer.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&candidates.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&copies.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&candidates.clone()) {
    index = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); __args.push(candidates.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    let __unpk16 = py_unpack(&py_tuple(vec![geti(&geti(&candidates.clone(), &index.clone()), &py_str("reply")), geti(&geti(&candidates.clone(), &index.clone()), &py_str("module"))]), 2, None);
    reply = __unpk16[0].clone();
    module = __unpk16[1].clone();
    if py_truthy(&Py::Bool(!py_is(&geti(&copies.clone(), &index.clone()), &memory.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "clear", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&copies.clone(), &index.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "update", args))(&__args) };
    }
    } else {
    let __unpk17 = py_unpack(&py_tuple(vec![g_FALLBACK(), Py::None]), 2, None);
    reply = __unpk17[0].clone();
    module = __unpk17[1].clone();
    }
    let __iter18 = py_iter_or_raise(&g__AFTER());
    for __item in __iter18 {
    let __unpk19 = py_unpack(&__item.clone(), 3, None);
    _u = __unpk19[0].clone();
    _u = __unpk19[1].clone();
    after = __unpk19[2].clone();
    let __try20 = exc::catch(|| {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(reply.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&after, args))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret21: Py = Py::Bool(false);
    let mut __val22: Py = Py::None;
    let mut __unhandled23: Option<exc::PyExc> = None;
    match __try20 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret21 = __p[0].clone(); __val22 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled24 = false;
            if !__handled24 && exc::matches_any(&__e, &["Exception"]) {
                __handled24 = true;
    continue;
            }
            if !__handled24 { __unhandled23 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled23 { exc::raise(__e); }
    if py_truthy(&__ret21) { return __val22; }
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    turns = py_list(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("turns"), py_list(vec![]));
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("user"), text.clone()), (py_str("bot"), reply.clone()), (py_str("module"), module.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turns.clone(), "append", args))(&__args) };
    return py_dict(vec![(py_str("reply"), reply.clone()), (py_str("module"), module.clone()), (py_str("candidates"), candidates.clone())]);
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    return geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| turn(args))(&__args) }, &py_str("reply"));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.runtime".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ARBITER" => Some(g_ARBITER()),
            "FALLBACK" => Some(g_FALLBACK()),
            "_AFTER" => Some(g__AFTER()),
            "_ARBITER" => Some(g__ARBITER()),
            "_HANDLERS" => Some(g__HANDLERS()),
            "_MODULES" => Some(g__MODULES()),
            "_OBSERVERS" => Some(g__OBSERVERS()),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_copy" => Some(py_func("_copy", |args: &[Py]| _copy(args))),
            "_hooks" => Some(py_func("_hooks", |args: &[Py]| _hooks(args))),
            "load_modules" => Some(py_func("load_modules", |args: &[Py]| load_modules(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            "turn" => Some(py_func("turn", |args: &[Py]| turn(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ARBITER" => s_ARBITER(v),
            "FALLBACK" => s_FALLBACK(v),
            "_AFTER" => s__AFTER(v),
            "_ARBITER" => s__ARBITER(v),
            "_HANDLERS" => s__HANDLERS(v),
            "_MODULES" => s__MODULES(v),
            "_OBSERVERS" => s__OBSERVERS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("runtime", module_obj);
    pyrt::xrefs::reg_fn("runtime", "load_modules", py_func("load_modules", |args: &[Py]| load_modules(args)));
    pyrt::xrefs::reg_fn("runtime", "_hooks", py_func("_hooks", |args: &[Py]| _hooks(args)));
    pyrt::xrefs::reg_fn("runtime", "_copy", py_func("_copy", |args: &[Py]| _copy(args)));
    pyrt::xrefs::reg_fn("runtime", "_choose", py_func("_choose", |args: &[Py]| _choose(args)));
    pyrt::xrefs::reg_fn("runtime", "turn", py_func("turn", |args: &[Py]| turn(args)));
    pyrt::xrefs::reg_fn("runtime", "reply", py_func("reply", |args: &[Py]| reply(args)));
    pyrt::xrefs::reg_access("runtime", "ARBITER", g_ARBITER, s_ARBITER);
    pyrt::xrefs::reg_access("runtime", "FALLBACK", g_FALLBACK, s_FALLBACK);
    pyrt::xrefs::reg_access("runtime", "_AFTER", g__AFTER, s__AFTER);
    pyrt::xrefs::reg_access("runtime", "_ARBITER", g__ARBITER, s__ARBITER);
    pyrt::xrefs::reg_access("runtime", "_HANDLERS", g__HANDLERS, s__HANDLERS);
    pyrt::xrefs::reg_access("runtime", "_MODULES", g__MODULES, s__MODULES);
    pyrt::xrefs::reg_access("runtime", "_OBSERVERS", g__OBSERVERS, s__OBSERVERS);
}
