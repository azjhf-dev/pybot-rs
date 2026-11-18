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
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A day cut out of school: the office, the note, the lesson that will be missed.\n\nOne question whoever types it, so it is keyed on the day and the appointment rather than on\nwhether a pupil or a parent is asking. It lives in its own file rather than in `study.py`\nbecause that file is edited by several lanes at once and a row added in the middle of it is the\nfirst thing a rebase takes away; a domain file that exists only for this question costs nothing\nand cannot collide.\n\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("school-and-study")]));
    s_ENTRIES(py_list(vec![]));
    fn __data1() -> Py { py_tuple(vec![py_str("leave school early"), py_str("leaving school early"), py_str("left school early"), py_str("leave early from school"), py_str("leaving early from school"), py_str("out of school early"), py_str("out of school for"), py_str("leave class early"), py_str("leaving class early"), py_str("leave a lesson early"), py_str("leave school before the end"), py_str("finish school early"), py_str("school finish early"), py_str("early from school"), py_str("sign out of school"), py_str("sign them out"), py_str("sign him out"), py_str("sign her out"), py_str("collect my child early"), py_str("collect her early"), py_str("collect him early"), py_str("collect them early"), py_str("pick my child up early"), py_str("pick my son up early"), py_str("pick my daughter up early"), py_str("leave college early"), py_str("leaving college early"), py_str("leave uni early"), py_str("leaving uni early"), py_str("take my son out of school"), py_str("take my daughter out of school"), py_str("take my child out of school"), py_str("take him out of school"), py_str("take her out of school"), py_str("appointment in school time"), py_str("appointment during school"), py_str("dentist in school time"), py_str("dentist appointment school"), py_str("during school hours")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("school"), py_str("class"), py_str("lesson"), py_str("period"), py_str("pupil"), py_str("teacher"), py_str("form tutor"), py_str("office"), py_str("reception"), py_str("head of year"), py_str("child"), py_str("son"), py_str("daughter"), py_str("kid"), py_str("kids"), py_str("early"), py_str("earlier"), py_str("today"), py_str("tomorrow"), py_str("afternoon"), py_str("morning"), py_str("appointment"), py_str("dentist"), py_str("doctor"), py_str("hospital"), py_str("clinic"), py_str("note"), py_str("email"), py_str("permission"), py_str("authorised"), py_str("authorized"), py_str("attendance")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("school"), py_str("class"), py_str("lesson"), py_str("pupil"), py_str("teacher"), py_str("form tutor"), py_str("college"), py_str("uni"), py_str("sixth form"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("off school"), py_str("stay off school"), py_str("keep him home"), py_str("keep her home"), py_str("keep them home"), py_str("keep my child home"), py_str("well enough"), py_str("temperature"), py_str("fever"), py_str("sick"), py_str("vomiting"), py_str("diarrhoea"), py_str("refuses to go"), py_str("refusing to go"), py_str("wont go"), py_str("won't go"), py_str("hates"), py_str("hate"), py_str("bully"), py_str("bullied"), py_str("bullying"), py_str("breaks up"), py_str("break up"), py_str("term dates"), py_str("term starts"), py_str("go back"), py_str("holidays"), py_str("half term"), py_str("inset day"), py_str("school uniform"), py_str("nursery"), py_str("childminder"), py_str("homework"), py_str("exam"), py_str("revision"), py_str("next year"), py_str("next summer"), py_str("next term"), py_str("for good"), py_str("at the end of"), py_str("school leaver"), py_str("sixth form"), py_str("leaving school soon"), py_str("when i leave"), py_str("get a job"), py_str("got a job"), py_str("and get a job"), py_str("a full time job"), py_str("career"), py_str("careers"), py_str("apprenticeship"), py_str("apprenticeships"), py_str("a levels"), py_str("a level"), py_str("qualifications"), py_str("at 16"), py_str("sixteen"), py_str("drop out"), py_str("dropping out"), py_str("quit school"), py_str("quitting school"), py_str("leave at 16"), py_str("leaving at 16"), py_str("finish school at 16"), py_str("not going back"), py_str("wont be going back"), py_str("won't be going back")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-leaving-early")); __args.push(py_tuple(vec![__data1(), __data2()])); __args.push(py_str("Ask the school office in advance and put it in writing: a note or an email naming the day, the time and who is collecting is enough for an appointment, and the office logs it as authorised absence. A pupil who leaves on their own should tell their form tutor and the teacher whose lesson they will miss, so nobody marks them absent, and take the work with them. Ring the office the same morning only for something you could not plan, and read the attendance policy before anything that is not medical - an appointment, a funeral and a religious day are authorised, a family trip in term time usually is not, and repeated unauthorised absence is what leads to a fine.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), __data3()), ("avoid".to_string(), __data4())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.school_days".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.school_days", module_obj);
    pyrt::xrefs::reg_access("knowledge.school_days", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.school_days", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.school_days", "entry", g_entry, s_entry);
}
